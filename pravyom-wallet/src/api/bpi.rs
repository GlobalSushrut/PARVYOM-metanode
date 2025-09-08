use axum::{extract::{State, Path}, response::Json};
use serde_json::json;

use crate::{AppState, api::{ApiResult, ApiError}};

pub async fn get_components_status(State(state): State<AppState>) -> ApiResult<Json<serde_json::Value>> {
    let components = state.wallet_core.component_manager.get_all_components().await;
    
    let mut components_by_category = std::collections::HashMap::new();
    let mut total_running = 0;
    let mut total_stopped = 0;
    let mut total_error = 0;
    
    for component in components.values() {
        let category_name = match component.category {
            crate::core::ComponentCategory::Core => "core",
            crate::core::ComponentCategory::Security => "security",
            crate::core::ComponentCategory::Storage => "storage",
            crate::core::ComponentCategory::Network => "network",
            crate::core::ComponentCategory::Consensus => "consensus",
            crate::core::ComponentCategory::VM => "vm",
            crate::core::ComponentCategory::API => "api",
            crate::core::ComponentCategory::Monitoring => "monitoring",
            crate::core::ComponentCategory::Integration => "integration",
        };
        
        components_by_category
            .entry(category_name)
            .or_insert_with(Vec::new)
            .push(json!({
                "name": component.name,
                "display_name": component.display_name,
                "status": component.status,
                "port": component.port,
                "dependencies": component.dependencies,
                "metrics": component.metrics,
                "last_check": component.last_status_check
            }));
            
        match component.status {
            crate::core::ComponentStatus::Running => total_running += 1,
            crate::core::ComponentStatus::Stopped => total_stopped += 1,
            crate::core::ComponentStatus::Error(_) => total_error += 1,
            _ => {}
        }
    }
    
    let response = json!({
        "summary": {
            "total": components.len(),
            "running": total_running,
            "stopped": total_stopped,
            "error": total_error
        },
        "components_by_category": components_by_category,
        "last_updated": chrono::Utc::now()
    });
    
    Ok(Json(response))
}

pub async fn get_component_details(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    match state.wallet_core.component_manager.get_component(&name).await {
        Some(component) => {
            let response = json!({
                "name": component.name,
                "display_name": component.display_name,
                "description": component.description,
                "category": component.category,
                "status": component.status,
                "port": component.port,
                "health_check_url": component.health_check_url,
                "dependencies": component.dependencies,
                "auto_restart": component.auto_restart,
                "last_status_check": component.last_status_check,
                "uptime": component.uptime,
                "metrics": component.metrics
            });
            Ok(Json(response))
        }
        None => Err(ApiError::NotFound(format!("Component '{}' not found", name)))
    }
}

pub async fn start_component(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    match state.wallet_core.component_manager.start_component(&name).await {
        Ok(_) => {
            let response = json!({
                "status": "success",
                "message": format!("Component '{}' started successfully", name),
                "component": name,
                "timestamp": chrono::Utc::now()
            });
            Ok(Json(response))
        }
        Err(e) => Err(ApiError::Internal(format!("Failed to start component '{}': {}", name, e)))
    }
}

pub async fn stop_component(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    match state.wallet_core.component_manager.stop_component(&name).await {
        Ok(_) => {
            let response = json!({
                "status": "success",
                "message": format!("Component '{}' stopped successfully", name),
                "component": name,
                "timestamp": chrono::Utc::now()
            });
            Ok(Json(response))
        }
        Err(e) => Err(ApiError::Internal(format!("Failed to stop component '{}': {}", name, e)))
    }
}

pub async fn get_metrics(State(state): State<AppState>) -> ApiResult<Json<serde_json::Value>> {
    let metrics = state.wallet_core.metrics.read().await.clone();
    
    let response = json!({
        "system": metrics.system,
        "wallet": metrics.wallet,
        "bpi_core": metrics.bpi_core,
        "performance": metrics.performance,
        "health_score": metrics.get_health_score(),
        "last_updated": metrics.last_updated
    });
    
    Ok(Json(response))
}

pub async fn get_component_logs(
    State(state): State<AppState>,
    Path(component): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    match state.wallet_core.bpi_integration.get_component_logs(&component, Some(100)).await {
        Ok(logs) => {
            let response = json!({
                "component": component,
                "logs": logs,
                "count": logs.len(),
                "timestamp": chrono::Utc::now()
            });
            Ok(Json(response))
        }
        Err(e) => Err(ApiError::Internal(format!("Failed to get logs for component '{}': {}", component, e)))
    }
}
