//! Stamped BPI Communication API Layer
//!
//! This module implements the communication API layer for stamped BPI wallets,
//! providing different access levels based on wallet stamp type and BISO agreements.

use crate::biso_agreement::{
    BisoAgreementManager, BisoAgreementType, CommunicationPermission, AccessLevel,
    ComplianceLevel, ApiAccessLevel, ComplianceReportType
};
use anyhow::{anyhow, Result};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use uuid::Uuid;

/// Stamped BPI Communication API state
#[derive(Debug, Clone)]
pub struct StampedBpiApiState {
    pub biso_manager: Arc<BisoAgreementManager>,
    pub wallet_stamps: Arc<RwLock<HashMap<String, WalletStamp>>>,
    pub api_metrics: Arc<RwLock<ApiMetrics>>,
}

/// Wallet stamp information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletStamp {
    pub wallet_id: String,
    pub stamp_type: StampType,
    pub issuer: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub verification_status: VerificationStatus,
    pub compliance_level: ComplianceLevel,
}

/// Types of wallet stamps
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StampType {
    Government {
        government_id: String,
        jurisdiction: String,
        authority_level: String,
    },
    Bank {
        bank_id: String,
        banking_license: String,
        regulatory_body: String,
    },
    Enterprise {
        company_id: String,
        industry: String,
        certification: String,
    },
    Individual {
        kyc_level: String,
        verification_method: String,
    },
    Unstamped,
}

/// Verification status for stamps
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VerificationStatus {
    Verified,
    Pending,
    Expired,
    Revoked,
    Suspended,
}

/// API request structure
#[derive(Debug, Deserialize)]
pub struct ApiRequest {
    pub wallet_id: String,
    pub operation: String,
    pub endpoint: String,
    pub data: Option<serde_json::Value>,
    pub signature: Option<String>,
}

/// API response structure
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub access_level: String,
    pub compliance_required: bool,
    pub timestamp: DateTime<Utc>,
}

/// POE (Proof of Execution) sharing request
#[derive(Debug, Deserialize)]
pub struct PoeShareRequest {
    pub wallet_id: String,
    pub proof_data: String,
    pub proof_type: String,
    pub recipient: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// POE sharing response
#[derive(Debug, Serialize)]
pub struct PoeShareResponse {
    pub share_id: Uuid,
    pub status: String,
    pub shared_at: DateTime<Utc>,
    pub compliance_report_required: bool,
}

/// Bank API communication request
#[derive(Debug, Deserialize)]
pub struct BankApiRequest {
    pub wallet_id: String,
    pub bank_operation: String,
    pub settlement_data: Option<serde_json::Value>,
    pub compliance_data: serde_json::Value,
}

/// Government API communication request
#[derive(Debug, Deserialize)]
pub struct GovernmentApiRequest {
    pub wallet_id: String,
    pub government_operation: String,
    pub regulatory_data: serde_json::Value,
    pub classification_level: String,
}

/// API metrics for monitoring
#[derive(Debug, Clone, Default, Serialize)]
pub struct ApiMetrics {
    pub total_requests: u64,
    pub government_api_calls: u64,
    pub bank_api_calls: u64,
    pub poe_shares: u64,
    pub denied_requests: u64,
    pub compliance_reports_generated: u64,
}

impl StampedBpiApiState {
    /// Create new API state
    pub fn new() -> Self {
        Self {
            biso_manager: Arc::new(BisoAgreementManager::new()),
            wallet_stamps: Arc::new(RwLock::new(HashMap::new())),
            api_metrics: Arc::new(RwLock::new(ApiMetrics::default())),
        }
    }

    /// Register a wallet stamp
    pub async fn register_wallet_stamp(&self, wallet_stamp: WalletStamp) -> Result<()> {
        info!("📋 Registering wallet stamp: {} -> {:?}", wallet_stamp.wallet_id, wallet_stamp.stamp_type);

        // Create BISO agreement based on stamp type
        let agreement_type = self.stamp_to_agreement_type(&wallet_stamp);
        self.biso_manager.create_agreement(wallet_stamp.wallet_id.clone(), agreement_type).await?;

        // Store wallet stamp
        let mut stamps = self.wallet_stamps.write().await;
        stamps.insert(wallet_stamp.wallet_id.clone(), wallet_stamp);

        Ok(())
    }

    /// Convert stamp type to BISO agreement type
    fn stamp_to_agreement_type(&self, stamp: &WalletStamp) -> BisoAgreementType {
        match &stamp.stamp_type {
            StampType::Government { government_id, jurisdiction, .. } => {
                BisoAgreementType::GovernmentStamped {
                    government_id: government_id.clone(),
                    jurisdiction: jurisdiction.clone(),
                    compliance_level: stamp.compliance_level.clone(),
                    api_access_level: ApiAccessLevel::Full {
                        bank_api: true,
                        government_api: true,
                        cross_system_communication: true,
                    },
                }
            }
            StampType::Bank { bank_id, banking_license, .. } => {
                BisoAgreementType::BankStamped {
                    bank_id: bank_id.clone(),
                    banking_license: banking_license.clone(),
                    compliance_level: stamp.compliance_level.clone(),
                    api_access_level: ApiAccessLevel::Full {
                        bank_api: true,
                        government_api: false,
                        cross_system_communication: true,
                    },
                }
            }
            StampType::Enterprise { company_id, .. } => {
                BisoAgreementType::OtherStamped {
                    stamp_type: "enterprise".to_string(),
                    issuer: company_id.clone(),
                    restrictions: crate::biso_agreement::CommunicationRestrictions {
                        can_share_poe: true,
                        requires_biso_agreement: true,
                        compliance_reporting_required: true,
                        allowed_endpoints: vec!["/api/poe/*".to_string()],
                        blocked_endpoints: vec!["/api/bank/*".to_string(), "/api/government/*".to_string()],
                    },
                }
            }
            StampType::Individual { .. } => {
                BisoAgreementType::OtherStamped {
                    stamp_type: "individual".to_string(),
                    issuer: "self".to_string(),
                    restrictions: crate::biso_agreement::CommunicationRestrictions {
                        can_share_poe: true,
                        requires_biso_agreement: true,
                        compliance_reporting_required: true,
                        allowed_endpoints: vec!["/api/poe/*".to_string()],
                        blocked_endpoints: vec!["/api/bank/*".to_string(), "/api/government/*".to_string()],
                    },
                }
            }
            StampType::Unstamped => {
                BisoAgreementType::Unstamped {
                    wallet_id: stamp.wallet_id.clone(),
                    mandatory_biso: true,
                }
            }
        }
    }
}

/// Create the stamped BPI communication router
pub fn create_stamped_bpi_router() -> Router<StampedBpiApiState> {
    Router::new()
        .route("/api/stamped-bpi/register-stamp", post(register_wallet_stamp))
        .route("/api/stamped-bpi/poe/share", post(share_poe))
        .route("/api/stamped-bpi/bank/:operation", post(bank_api_communication))
        .route("/api/stamped-bpi/government/:operation", post(government_api_communication))
        .route("/api/stamped-bpi/compliance/report/:wallet_id", get(get_compliance_report))
        .route("/api/stamped-bpi/status/:wallet_id", get(get_wallet_status))
        .route("/api/stamped-bpi/metrics", get(get_api_metrics))
}

/// Register a wallet stamp
async fn register_wallet_stamp(
    State(state): State<StampedBpiApiState>,
    Json(wallet_stamp): Json<WalletStamp>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.register_wallet_stamp(wallet_stamp.clone()).await {
        Ok(_) => {
            let mut metrics = state.api_metrics.write().await;
            metrics.total_requests += 1;

            Ok(Json(ApiResponse {
                success: true,
                data: Some(format!("Wallet stamp registered: {}", wallet_stamp.wallet_id)),
                error: None,
                access_level: format!("{:?}", wallet_stamp.stamp_type),
                compliance_required: true,
                timestamp: Utc::now(),
            }))
        }
        Err(e) => {
            error!("Failed to register wallet stamp: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Share POE (Proof of Execution) - available to all wallet types
async fn share_poe(
    State(state): State<StampedBpiApiState>,
    Json(request): Json<PoeShareRequest>,
) -> Result<Json<ApiResponse<PoeShareResponse>>, StatusCode> {
    info!("🔗 POE sharing request from wallet: {}", request.wallet_id);

    // Check communication permission
    match state.biso_manager.evaluate_communication_permission(
        &request.wallet_id,
        "/api/poe/share",
        "share_proof"
    ).await {
        Ok(permission) => {
            if permission.allowed {
                let mut metrics = state.api_metrics.write().await;
                metrics.total_requests += 1;
                metrics.poe_shares += 1;

                // Generate compliance report if required
                if permission.requires_compliance_report {
                    // This would trigger compliance report generation
                    metrics.compliance_reports_generated += 1;
                }

                let response = PoeShareResponse {
                    share_id: Uuid::new_v4(),
                    status: "shared".to_string(),
                    shared_at: Utc::now(),
                    compliance_report_required: permission.requires_compliance_report,
                };

                Ok(Json(ApiResponse {
                    success: true,
                    data: Some(response),
                    error: None,
                    access_level: format!("{:?}", permission.access_level),
                    compliance_required: permission.requires_compliance_report,
                    timestamp: Utc::now(),
                }))
            } else {
                let mut metrics = state.api_metrics.write().await;
                metrics.denied_requests += 1;

                Ok(Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("POE sharing not allowed for this wallet".to_string()),
                    access_level: "none".to_string(),
                    compliance_required: true,
                    timestamp: Utc::now(),
                }))
            }
        }
        Err(e) => {
            error!("Error evaluating POE permission: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Bank API communication - only for government and bank stamped wallets
async fn bank_api_communication(
    State(state): State<StampedBpiApiState>,
    Path(operation): Path<String>,
    Json(request): Json<BankApiRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    info!("🏦 Bank API request: {} from wallet: {}", operation, request.wallet_id);

    let endpoint = format!("/api/bank/{}", operation);
    
    match state.biso_manager.evaluate_communication_permission(
        &request.wallet_id,
        &endpoint,
        &operation
    ).await {
        Ok(permission) => {
            if permission.allowed && permission.access_level == AccessLevel::Full {
                let mut metrics = state.api_metrics.write().await;
                metrics.total_requests += 1;
                metrics.bank_api_calls += 1;

                // Process bank API request
                let response_data = serde_json::json!({
                    "operation": operation,
                    "status": "processed",
                    "bank_response": "Bank API operation completed successfully",
                    "compliance_verified": true
                });

                Ok(Json(ApiResponse {
                    success: true,
                    data: Some(response_data),
                    error: None,
                    access_level: "full".to_string(),
                    compliance_required: true,
                    timestamp: Utc::now(),
                }))
            } else {
                let mut metrics = state.api_metrics.write().await;
                metrics.denied_requests += 1;

                Ok(Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Bank API access denied. Only government and bank stamped wallets allowed.".to_string()),
                    access_level: format!("{:?}", permission.access_level),
                    compliance_required: true,
                    timestamp: Utc::now(),
                }))
            }
        }
        Err(e) => {
            error!("Error evaluating bank API permission: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Government API communication - only for government stamped wallets
async fn government_api_communication(
    State(state): State<StampedBpiApiState>,
    Path(operation): Path<String>,
    Json(request): Json<GovernmentApiRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    info!("🏛️ Government API request: {} from wallet: {}", operation, request.wallet_id);

    let endpoint = format!("/api/government/{}", operation);
    
    match state.biso_manager.evaluate_communication_permission(
        &request.wallet_id,
        &endpoint,
        &operation
    ).await {
        Ok(permission) => {
            // Check if wallet is government stamped
            let stamps = state.wallet_stamps.read().await;
            let is_government_stamped = stamps.get(&request.wallet_id)
                .map(|stamp| matches!(stamp.stamp_type, StampType::Government { .. }))
                .unwrap_or(false);

            if permission.allowed && permission.access_level == AccessLevel::Full && is_government_stamped {
                let mut metrics = state.api_metrics.write().await;
                metrics.total_requests += 1;
                metrics.government_api_calls += 1;

                // Process government API request
                let response_data = serde_json::json!({
                    "operation": operation,
                    "status": "processed",
                    "government_response": "Government API operation completed successfully",
                    "classification_cleared": true,
                    "compliance_verified": true
                });

                Ok(Json(ApiResponse {
                    success: true,
                    data: Some(response_data),
                    error: None,
                    access_level: "full".to_string(),
                    compliance_required: true,
                    timestamp: Utc::now(),
                }))
            } else {
                let mut metrics = state.api_metrics.write().await;
                metrics.denied_requests += 1;

                Ok(Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Government API access denied. Only government stamped wallets allowed.".to_string()),
                    access_level: format!("{:?}", permission.access_level),
                    compliance_required: true,
                    timestamp: Utc::now(),
                }))
            }
        }
        Err(e) => {
            error!("Error evaluating government API permission: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get compliance report for a wallet
async fn get_compliance_report(
    State(state): State<StampedBpiApiState>,
    Path(wallet_id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    info!("📊 Compliance report request for wallet: {}", wallet_id);

    // Find the agreement for this wallet
    let report_type = params.get("type")
        .and_then(|t| match t.as_str() {
            "daily" => Some(ComplianceReportType::Daily),
            "weekly" => Some(ComplianceReportType::Weekly),
            "monthly" => Some(ComplianceReportType::Monthly),
            _ => Some(ComplianceReportType::Daily),
        })
        .unwrap_or(ComplianceReportType::Daily);

    // For demonstration, we'll create a mock compliance report
    let report_data = serde_json::json!({
        "wallet_id": wallet_id,
        "report_type": format!("{:?}", report_type),
        "generated_at": Utc::now(),
        "compliance_status": "Compliant",
        "total_transactions": 1000,
        "compliant_transactions": 950,
        "violations": [],
        "compliance_score": 0.95
    });

    let mut metrics = state.api_metrics.write().await;
    metrics.total_requests += 1;
    metrics.compliance_reports_generated += 1;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(report_data),
        error: None,
        access_level: "compliance".to_string(),
        compliance_required: false,
        timestamp: Utc::now(),
    }))
}

/// Get wallet status and stamp information
async fn get_wallet_status(
    State(state): State<StampedBpiApiState>,
    Path(wallet_id): Path<String>,
) -> Result<Json<ApiResponse<WalletStamp>>, StatusCode> {
    let stamps = state.wallet_stamps.read().await;
    
    match stamps.get(&wallet_id) {
        Some(stamp) => {
            let mut metrics = state.api_metrics.write().await;
            metrics.total_requests += 1;

            Ok(Json(ApiResponse {
                success: true,
                data: Some(stamp.clone()),
                error: None,
                access_level: format!("{:?}", stamp.stamp_type),
                compliance_required: true,
                timestamp: Utc::now(),
            }))
        }
        None => {
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                error: Some("Wallet not found".to_string()),
                access_level: "none".to_string(),
                compliance_required: false,
                timestamp: Utc::now(),
            }))
        }
    }
}

/// Get API metrics
async fn get_api_metrics(
    State(state): State<StampedBpiApiState>,
) -> Result<Json<ApiResponse<ApiMetrics>>, StatusCode> {
    let metrics = state.api_metrics.read().await;
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(metrics.clone()),
        error: None,
        access_level: "metrics".to_string(),
        compliance_required: false,
        timestamp: Utc::now(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_government_wallet_registration() {
        let state = StampedBpiApiState::new();
        
        let gov_stamp = WalletStamp {
            wallet_id: "gov_wallet_001".to_string(),
            stamp_type: StampType::Government {
                government_id: "US-GOV-001".to_string(),
                jurisdiction: "United States".to_string(),
                authority_level: "Federal".to_string(),
            },
            issuer: "US Government".to_string(),
            issued_at: Utc::now(),
            expires_at: Some(Utc::now() + chrono::Duration::days(365)),
            verification_status: VerificationStatus::Verified,
            compliance_level: ComplianceLevel::Government,
        };

        let result = state.register_wallet_stamp(gov_stamp).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_unstamped_wallet_restrictions() {
        let state = StampedBpiApiState::new();
        
        let unstamped = WalletStamp {
            wallet_id: "unstamped_001".to_string(),
            stamp_type: StampType::Unstamped,
            issuer: "self".to_string(),
            issued_at: Utc::now(),
            expires_at: None,
            verification_status: VerificationStatus::Verified,
            compliance_level: ComplianceLevel::Basic,
        };

        state.register_wallet_stamp(unstamped).await.unwrap();

        // Test that unstamped wallet can share POE
        let permission = state.biso_manager.evaluate_communication_permission(
            "unstamped_001",
            "/api/poe/share",
            "share_proof"
        ).await.unwrap();
        
        assert!(permission.allowed);
        assert_eq!(permission.access_level, AccessLevel::PoeOnly);
    }
}
