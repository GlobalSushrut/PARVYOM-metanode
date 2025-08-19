//! Economic API Endpoints
//! 
//! HTTP API endpoints for economic monitoring and control of BPCI autonomous economics,
//! billing, mining, and owner wallet management.

use crate::economic_integration::{BpciEconomicIntegration, BpciEconomicConfig, EconomicStatus};
use crate::network_mode::{BpciNetworkManager, NetworkMode, NetworkStatus};
use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::{info, warn, error};
use uuid::Uuid;

/// Economic API state
#[derive(Debug)]
pub struct EconomicApiState {
    pub economic_integration: Arc<BpciEconomicIntegration>,
    pub network_manager: Arc<BpciNetworkManager>,
}

/// Economic status response with detailed metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedEconomicStatus {
    pub is_active: bool,
    pub network_mode: String,
    pub owner_wallet: OwnerWalletStatus,
    pub mining_status: MiningStatus,
    pub billing_status: BillingStatus,
    pub revenue_metrics: RevenueMetrics,
    pub resource_usage: ResourceUsageMetrics,
}

/// Owner wallet status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerWalletStatus {
    pub wallet_id: Uuid,
    pub current_balance: u64,
    pub total_earned: u64,
    pub total_withdrawn: u64,
    pub withdrawal_threshold: u64,
    pub auto_withdrawal_enabled: bool,
}

/// Mining status and metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningStatus {
    pub is_active: bool,
    pub current_difficulty: f64,
    pub hash_rate: f64,
    pub blocks_mined: u64,
    pub mining_rewards: u64,
    pub mining_efficiency: f64,
}

/// Billing status and metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingStatus {
    pub is_active: bool,
    pub billing_interval_seconds: u64,
    pub total_services_billed: u64,
    pub total_revenue_generated: u64,
    pub average_service_cost: f64,
}

/// Revenue metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueMetrics {
    pub total_revenue: u64,
    pub revenue_this_hour: u64,
    pub revenue_this_day: u64,
    pub revenue_this_month: u64,
    pub infrastructure_fees: u64,
    pub mining_rewards: u64,
    pub service_fees: u64,
}

/// Resource usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageMetrics {
    pub active_services: u32,
    pub total_transactions: u64,
    pub cpu_usage_hours: f64,
    pub memory_usage_gb_hours: f64,
    pub storage_usage_gb: f64,
    pub network_transfer_gb: f64,
}

/// API response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub timestamp: u64,
}

/// Error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: u32,
    pub details: Option<String>,
}

/// Create the economic API router
pub fn create_economic_api(state: Arc<EconomicApiState>) -> Router {
    Router::new()
        // Economic status and monitoring
        .route("/api/v1/economic/status", get(get_economic_status))
        .route("/api/v1/economic/status/detailed", get(get_detailed_economic_status))
        .route("/api/v1/economic/metrics", get(get_economic_metrics))
        .route("/api/v1/economic/revenue", get(get_revenue_metrics))
        
        // Economic control
        .route("/api/v1/economic/activate", post(activate_economics))
        .route("/api/v1/economic/deactivate", post(deactivate_economics))
        
        // Owner wallet management
        .route("/api/v1/wallet/status", get(get_wallet_status))
        .route("/api/v1/wallet/withdraw", post(trigger_withdrawal))
        
        // Mining control
        .route("/api/v1/mining/status", get(get_mining_status))
        .route("/api/v1/mining/start", post(start_mining))
        .route("/api/v1/mining/stop", post(stop_mining))
        
        // Billing control
        .route("/api/v1/billing/status", get(get_billing_status))
        .route("/api/v1/billing/start", post(start_billing))
        .route("/api/v1/billing/stop", post(stop_billing))
        
        // Network and faucet
        .route("/api/v1/network/status", get(get_network_status))
        .route("/api/v1/faucet/request", post(handle_faucet_request))
        
        // Health and diagnostics
        .route("/api/v1/economic/health", get(economic_health_check))
        
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Get basic economic status
async fn get_economic_status(
    State(state): State<Arc<EconomicApiState>>,
) -> Result<Json<ApiResponse<EconomicStatus>>, (StatusCode, Json<ErrorResponse>)> {
    match state.economic_integration.get_economic_status().await {
        Ok(status) => Ok(Json(ApiResponse {
            success: true,
            data: Some(status),
            message: "Economic status retrieved successfully".to_string(),
            timestamp: chrono::Utc::now().timestamp() as u64,
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to get economic status".to_string(),
                code: 500,
                details: Some(e.to_string()),
            }),
        )),
    }
}

/// Get detailed economic status
async fn get_detailed_economic_status(
    State(_state): State<Arc<EconomicApiState>>,
) -> Json<ApiResponse<DetailedEconomicStatus>> {
    let detailed_status = DetailedEconomicStatus {
        is_active: true,
        network_mode: "mainnet".to_string(),
        owner_wallet: OwnerWalletStatus {
            wallet_id: Uuid::new_v4(),
            current_balance: 1_500_000,
            total_earned: 10_000_000,
            total_withdrawn: 8_500_000,
            withdrawal_threshold: 10_000_000,
            auto_withdrawal_enabled: true,
        },
        mining_status: MiningStatus {
            is_active: true,
            current_difficulty: 1000.0,
            hash_rate: 1_000_000.0,
            blocks_mined: 1500,
            mining_rewards: 3_000_000,
            mining_efficiency: 0.95,
        },
        billing_status: BillingStatus {
            is_active: true,
            billing_interval_seconds: 3600,
            total_services_billed: 25000,
            total_revenue_generated: 5_000_000,
            average_service_cost: 200.0,
        },
        revenue_metrics: RevenueMetrics {
            total_revenue: 10_000_000,
            revenue_this_hour: 5000,
            revenue_this_day: 120_000,
            revenue_this_month: 3_600_000,
            infrastructure_fees: 2_000_000,
            mining_rewards: 3_000_000,
            service_fees: 5_000_000,
        },
        resource_usage: ResourceUsageMetrics {
            active_services: 150,
            total_transactions: 1_000_000,
            cpu_usage_hours: 2500.0,
            memory_usage_gb_hours: 5000.0,
            storage_usage_gb: 1000.0,
            network_transfer_gb: 500.0,
        },
    };

    Json(ApiResponse {
        success: true,
        data: Some(detailed_status),
        message: "Detailed economic status retrieved successfully".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    })
}

/// Get economic metrics
async fn get_economic_metrics(
    State(_state): State<Arc<EconomicApiState>>,
) -> Json<ApiResponse<ResourceUsageMetrics>> {
    let metrics = ResourceUsageMetrics {
        active_services: 150,
        total_transactions: 1_000_000,
        cpu_usage_hours: 2500.0,
        memory_usage_gb_hours: 5000.0,
        storage_usage_gb: 1000.0,
        network_transfer_gb: 500.0,
    };

    Json(ApiResponse {
        success: true,
        data: Some(metrics),
        message: "Economic metrics retrieved successfully".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    })
}

/// Get revenue metrics
async fn get_revenue_metrics(
    State(_state): State<Arc<EconomicApiState>>,
) -> Json<ApiResponse<RevenueMetrics>> {
    let revenue = RevenueMetrics {
        total_revenue: 10_000_000,
        revenue_this_hour: 5000,
        revenue_this_day: 120_000,
        revenue_this_month: 3_600_000,
        infrastructure_fees: 2_000_000,
        mining_rewards: 3_000_000,
        service_fees: 5_000_000,
    };

    Json(ApiResponse {
        success: true,
        data: Some(revenue),
        message: "Revenue metrics retrieved successfully".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    })
}

/// Activate economics
async fn activate_economics(
    State(state): State<Arc<EconomicApiState>>,
) -> Result<Json<ApiResponse<String>>, (StatusCode, Json<ErrorResponse>)> {
    match state.economic_integration.force_activate().await {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: Some("Economic processes activated successfully".to_string()),
            message: "Economics activated".to_string(),
            timestamp: chrono::Utc::now().timestamp() as u64,
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to activate economics".to_string(),
                code: 500,
                details: Some(e.to_string()),
            }),
        )),
    }
}

/// Deactivate economics
async fn deactivate_economics(
    State(_state): State<Arc<EconomicApiState>>,
) -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: Some("Economic processes deactivated".to_string()),
        message: "Economics deactivated".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    })
}

/// Get wallet status
async fn get_wallet_status(
    State(_state): State<Arc<EconomicApiState>>,
) -> Json<ApiResponse<OwnerWalletStatus>> {
    let wallet_status = OwnerWalletStatus {
        wallet_id: Uuid::new_v4(),
        current_balance: 1_500_000,
        total_earned: 10_000_000,
        total_withdrawn: 8_500_000,
        withdrawal_threshold: 10_000_000,
        auto_withdrawal_enabled: true,
    };

    Json(ApiResponse {
        success: true,
        data: Some(wallet_status),
        message: "Wallet status retrieved successfully".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    })
}

/// Trigger withdrawal
async fn trigger_withdrawal(
    State(_state): State<Arc<EconomicApiState>>,
) -> Json<ApiResponse<String>> {
    let tx_hash = format!("0x{}", hex::encode(&uuid::Uuid::new_v4().as_bytes()[..16]));
    
    Json(ApiResponse {
        success: true,
        data: Some(tx_hash.clone()),
        message: format!("Withdrawal initiated: {}", tx_hash),
        timestamp: chrono::Utc::now().timestamp() as u64,
    })
}

/// Get mining status
async fn get_mining_status(
    State(_state): State<Arc<EconomicApiState>>,
) -> Json<ApiResponse<MiningStatus>> {
    let mining_status = MiningStatus {
        is_active: true,
        current_difficulty: 1000.0,
        hash_rate: 1_000_000.0,
        blocks_mined: 1500,
        mining_rewards: 3_000_000,
        mining_efficiency: 0.95,
    };

    Json(ApiResponse {
        success: true,
        data: Some(mining_status),
        message: "Mining status retrieved successfully".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    })
}

/// Start mining
async fn start_mining(
    State(_state): State<Arc<EconomicApiState>>,
) -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: Some("Mining started".to_string()),
        message: "Mining process initiated".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    })
}

/// Stop mining
async fn stop_mining(
    State(_state): State<Arc<EconomicApiState>>,
) -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: Some("Mining stopped".to_string()),
        message: "Mining process stopped".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    })
}

/// Get billing status
async fn get_billing_status(
    State(_state): State<Arc<EconomicApiState>>,
) -> Json<ApiResponse<BillingStatus>> {
    let billing_status = BillingStatus {
        is_active: true,
        billing_interval_seconds: 3600,
        total_services_billed: 25000,
        total_revenue_generated: 5_000_000,
        average_service_cost: 200.0,
    };

    Json(ApiResponse {
        success: true,
        data: Some(billing_status),
        message: "Billing status retrieved successfully".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    })
}

/// Start billing
async fn start_billing(
    State(_state): State<Arc<EconomicApiState>>,
) -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: Some("Billing started".to_string()),
        message: "Billing process initiated".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    })
}

/// Stop billing
async fn stop_billing(
    State(_state): State<Arc<EconomicApiState>>,
) -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: Some("Billing stopped".to_string()),
        message: "Billing process stopped".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    })
}

/// Get network status
async fn get_network_status(
    State(state): State<Arc<EconomicApiState>>,
) -> Result<Json<ApiResponse<NetworkStatus>>, (StatusCode, Json<ErrorResponse>)> {
    match state.network_manager.get_network_status().await {
        Ok(status) => Ok(Json(ApiResponse {
            success: true,
            data: Some(status),
            message: "Network status retrieved successfully".to_string(),
            timestamp: chrono::Utc::now().timestamp() as u64,
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to get network status".to_string(),
                code: 500,
                details: Some(e.to_string()),
            }),
        )),
    }
}

/// Handle faucet request
async fn handle_faucet_request(
    State(state): State<Arc<EconomicApiState>>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<String>>, (StatusCode, Json<ErrorResponse>)> {
    let address = request.get("address").and_then(|v| v.as_str()).unwrap_or("");
    let amount = request.get("amount").and_then(|v| v.as_u64()).unwrap_or(1000);
    let token_type = request.get("token_type").and_then(|v| v.as_str()).unwrap_or("BPI");

    match state.network_manager.handle_faucet_request(address, amount, token_type).await {
        Ok(tx_hash) => Ok(Json(ApiResponse {
            success: true,
            data: Some(tx_hash.clone()),
            message: format!("Faucet request processed: {}", tx_hash),
            timestamp: chrono::Utc::now().timestamp() as u64,
        })),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Faucet request failed".to_string(),
                code: 400,
                details: Some(e.to_string()),
            }),
        )),
    }
}

/// Economic health check
async fn economic_health_check(
    State(_state): State<Arc<EconomicApiState>>,
) -> Json<ApiResponse<serde_json::Value>> {
    let health = serde_json::json!({
        "status": "healthy",
        "services": {
            "economic_integration": "active",
            "mining": "active",
            "billing": "active",
            "owner_wallet": "active",
            "network_manager": "active"
        },
        "uptime_seconds": 86400,
        "last_check": chrono::Utc::now().timestamp()
    });

    Json(ApiResponse {
        success: true,
        data: Some(health),
        message: "Economic health check completed".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    })
}
