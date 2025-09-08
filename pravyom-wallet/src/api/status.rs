use axum::{extract::State, response::Json};
use serde_json::json;

use crate::{AppState, api::ApiResult};

pub async fn get_status(State(state): State<AppState>) -> ApiResult<Json<serde_json::Value>> {
    let wallet_info = state.wallet_core.get_wallet_info().await;
    let bpi_status = state.wallet_core.bpi_integration.get_connection_status().await;
    let metrics = state.wallet_core.metrics.read().await;
    
    let response = json!({
        "status": "ok",
        "version": "0.1.0",
        "wallet": {
            "address": wallet_info.address,
            "network": wallet_info.network,
            "status": wallet_info.status,
            "balance": wallet_info.balance,
            "last_activity": wallet_info.last_activity
        },
        "bpi_core": {
            "connected": bpi_status.connected,
            "last_ping": bpi_status.last_ping,
            "version": bpi_status.version,
            "network": bpi_status.network
        },
        "system": {
            "health_score": metrics.get_health_score(),
            "cpu_usage": metrics.system.cpu_usage,
            "memory_usage": metrics.system.memory_usage,
            "uptime": metrics.system.uptime_seconds
        },
        "components": {
            "total": metrics.bpi_core.total_components,
            "connected": metrics.bpi_core.connected_components,
            "healthy": metrics.bpi_core.healthy_components
        },
        "timestamp": chrono::Utc::now()
    });
    
    Ok(Json(response))
}
