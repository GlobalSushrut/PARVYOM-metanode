use axum::extract::ws::{Message, WebSocket};
use futures::{sink::SinkExt, stream::StreamExt};
use serde_json::json;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{info, warn};

use crate::AppState;

pub async fn handle_websocket(socket: WebSocket, state: AppState) {
    let (sender, mut receiver) = socket.split();
    let sender = Arc::new(tokio::sync::Mutex::new(sender));
    
    info!("🔌 WebSocket connection established");
    
    // Send initial status
    let initial_status = get_wallet_status(&state).await;
    if let Ok(status_msg) = serde_json::to_string(&initial_status) {
        let mut sender_guard = sender.lock().await;
        if sender_guard.send(Message::Text(status_msg)).await.is_err() {
            return;
        }
    }
    
    // Spawn task for periodic updates
    let state_clone = state.clone();
    let sender_clone = sender.clone();
    let update_task = tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(2));
        
        loop {
            interval.tick().await;
            
            let status = get_wallet_status(&state_clone).await;
            let message = json!({
                "type": "status_update",
                "data": status,
                "timestamp": chrono::Utc::now()
            });
            
            if let Ok(msg_str) = serde_json::to_string(&message) {
                let mut sender_guard = sender_clone.lock().await;
                if sender_guard.send(Message::Text(msg_str)).await.is_err() {
                    break;
                }
            }
        }
    });
    
    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(request) = serde_json::from_str::<serde_json::Value>(&text) {
                    let response = handle_websocket_request(request, &state).await;
                    
                    if let Ok(response_str) = serde_json::to_string(&response) {
                        let mut sender_guard = sender.lock().await;
                        if sender_guard.send(Message::Text(response_str)).await.is_err() {
                            break;
                        }
                    }
                }
            }
            Ok(Message::Close(_)) => {
                info!("🔌 WebSocket connection closed by client");
                break;
            }
            Ok(Message::Ping(data)) => {
                let mut sender_guard = sender.lock().await;
                if sender_guard.send(Message::Pong(data)).await.is_err() {
                    break;
                }
            }
            Err(e) => {
                warn!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
    
    update_task.abort();
    info!("🔌 WebSocket connection terminated");
}

async fn get_wallet_status(state: &AppState) -> serde_json::Value {
    let wallet_info = state.wallet_core.get_wallet_info().await;
    let bpi_status = state.wallet_core.bpi_integration.get_connection_status().await;
    let metrics = state.wallet_core.metrics.read().await;
    let components = state.wallet_core.component_manager.get_all_components().await;
    
    let running_components = components.values()
        .filter(|c| matches!(c.status, crate::core::ComponentStatus::Running))
        .count();
    
    json!({
        "wallet": {
            "address": wallet_info.address,
            "balance": wallet_info.balance,
            "status": wallet_info.status,
            "network": wallet_info.network,
            "last_activity": wallet_info.last_activity
        },
        "bpi_core": {
            "connected": bpi_status.connected,
            "last_ping": bpi_status.last_ping,
            "version": bpi_status.version
        },
        "system": {
            "health_score": metrics.get_health_score(),
            "cpu_usage": metrics.system.cpu_usage,
            "memory_usage": metrics.system.memory_usage,
            "uptime": metrics.system.uptime_seconds
        },
        "components": {
            "total": components.len(),
            "running": running_components,
            "health_score": if components.len() > 0 {
                (running_components as f64 / components.len() as f64) * 100.0
            } else {
                0.0
            }
        }
    })
}

async fn handle_websocket_request(request: serde_json::Value, state: &AppState) -> serde_json::Value {
    let request_type = request.get("type").and_then(|v| v.as_str()).unwrap_or("");
    
    match request_type {
        "unlock_wallet" => {
            if let Some(password) = request.get("password").and_then(|v| v.as_str()) {
                match state.wallet_core.unlock(password).await {
                    Ok(_) => json!({
                        "type": "unlock_response",
                        "success": true,
                        "message": "Wallet unlocked successfully"
                    }),
                    Err(e) => json!({
                        "type": "unlock_response",
                        "success": false,
                        "error": e.to_string()
                    })
                }
            } else {
                json!({
                    "type": "unlock_response",
                    "success": false,
                    "error": "Password required"
                })
            }
        }
        "lock_wallet" => {
            match state.wallet_core.lock().await {
                Ok(_) => json!({
                    "type": "lock_response",
                    "success": true,
                    "message": "Wallet locked successfully"
                }),
                Err(e) => json!({
                    "type": "lock_response",
                    "success": false,
                    "error": e.to_string()
                })
            }
        }
        "start_component" => {
            if let Some(component_name) = request.get("component").and_then(|v| v.as_str()) {
                match state.wallet_core.component_manager.start_component(component_name).await {
                    Ok(_) => json!({
                        "type": "component_response",
                        "action": "start",
                        "component": component_name,
                        "success": true,
                        "message": format!("Component '{}' started successfully", component_name)
                    }),
                    Err(e) => json!({
                        "type": "component_response",
                        "action": "start",
                        "component": component_name,
                        "success": false,
                        "error": e.to_string()
                    })
                }
            } else {
                json!({
                    "type": "component_response",
                    "success": false,
                    "error": "Component name required"
                })
            }
        }
        "stop_component" => {
            if let Some(component_name) = request.get("component").and_then(|v| v.as_str()) {
                match state.wallet_core.component_manager.stop_component(component_name).await {
                    Ok(_) => json!({
                        "type": "component_response",
                        "action": "stop",
                        "component": component_name,
                        "success": true,
                        "message": format!("Component '{}' stopped successfully", component_name)
                    }),
                    Err(e) => json!({
                        "type": "component_response",
                        "action": "stop",
                        "component": component_name,
                        "success": false,
                        "error": e.to_string()
                    })
                }
            } else {
                json!({
                    "type": "component_response",
                    "success": false,
                    "error": "Component name required"
                })
            }
        }
        "refresh_components" => {
            match state.wallet_core.component_manager.refresh_all_components().await {
                Ok(_) => json!({
                    "type": "refresh_response",
                    "success": true,
                    "message": "Components refreshed successfully"
                }),
                Err(e) => json!({
                    "type": "refresh_response",
                    "success": false,
                    "error": e.to_string()
                })
            }
        }
        "get_component_logs" => {
            if let Some(component_name) = request.get("component").and_then(|v| v.as_str()) {
                match state.wallet_core.bpi_integration.get_component_logs(component_name, Some(50)).await {
                    Ok(logs) => json!({
                        "type": "logs_response",
                        "component": component_name,
                        "logs": logs,
                        "success": true
                    }),
                    Err(e) => json!({
                        "type": "logs_response",
                        "component": component_name,
                        "success": false,
                        "error": e.to_string()
                    })
                }
            } else {
                json!({
                    "type": "logs_response",
                    "success": false,
                    "error": "Component name required"
                })
            }
        }
        _ => json!({
            "type": "error",
            "message": format!("Unknown request type: {}", request_type)
        })
    }
}
