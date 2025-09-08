use axum::{extract::State, response::Json, Json as JsonExtractor};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{AppState, api::{ApiResult, ApiError}};

#[derive(Debug, Deserialize)]
pub struct SendTransactionRequest {
    pub to: String,
    pub amount: f64,
    pub password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub transaction_id: String,
    pub status: String,
    pub message: String,
}

pub async fn get_wallet_info(State(state): State<AppState>) -> ApiResult<Json<serde_json::Value>> {
    let wallet_info = state.wallet_core.get_wallet_info().await;
    let bpi_status = state.wallet_core.bpi_integration.get_connection_status().await;
    
    let response = json!({
        "address": wallet_info.address,
        "network": wallet_info.network,
        "status": wallet_info.status,
        "balance": wallet_info.balance,
        "created_at": wallet_info.created_at,
        "last_activity": wallet_info.last_activity,
        "bpi_connected": bpi_status.connected,
        "bpi_version": bpi_status.version
    });
    
    Ok(Json(response))
}

pub async fn get_balance(State(state): State<AppState>) -> ApiResult<Json<serde_json::Value>> {
    match state.wallet_core.get_balance().await {
        Ok(balance) => {
            let response = json!({
                "balance": balance,
                "currency": "BPI",
                "network": state.wallet_core.config.network,
                "last_updated": chrono::Utc::now()
            });
            Ok(Json(response))
        }
        Err(e) => Err(ApiError::ServiceUnavailable(format!("Failed to get balance: {}", e)))
    }
}

pub async fn get_transactions(State(state): State<AppState>) -> ApiResult<Json<serde_json::Value>> {
    let transactions = state.wallet_core.get_transactions().await;
    
    let response = json!({
        "transactions": transactions,
        "count": transactions.len(),
        "last_updated": chrono::Utc::now()
    });
    
    Ok(Json(response))
}

pub async fn send_transaction(
    State(state): State<AppState>,
    JsonExtractor(request): JsonExtractor<SendTransactionRequest>,
) -> ApiResult<Json<TransactionResponse>> {
    // Validate request
    if request.to.is_empty() {
        return Err(ApiError::BadRequest("Recipient address cannot be empty".to_string()));
    }
    
    if request.amount <= 0.0 {
        return Err(ApiError::BadRequest("Amount must be greater than 0".to_string()));
    }
    
    // Check if wallet is unlocked
    let wallet_info = state.wallet_core.get_wallet_info().await;
    match wallet_info.status {
        crate::core::WalletStatus::Connected => {},
        crate::core::WalletStatus::Locked => {
            return Err(ApiError::Unauthorized("Wallet is locked. Please unlock first.".to_string()));
        },
        crate::core::WalletStatus::Connecting => {
            return Err(ApiError::ServiceUnavailable("Wallet is connecting. Please wait.".to_string()));
        },
        crate::core::WalletStatus::Error(msg) => {
            return Err(ApiError::ServiceUnavailable(format!("Wallet error: {}", msg)));
        },
        _ => {
            return Err(ApiError::ServiceUnavailable("Wallet is not ready".to_string()));
        }
    }
    
    // Send transaction
    match state.wallet_core.send_transaction(&request.to, request.amount).await {
        Ok(tx_id) => {
            let response = TransactionResponse {
                transaction_id: tx_id,
                status: "pending".to_string(),
                message: "Transaction submitted successfully".to_string(),
            };
            Ok(Json(response))
        }
        Err(e) => Err(ApiError::Internal(format!("Failed to send transaction: {}", e)))
    }
}
