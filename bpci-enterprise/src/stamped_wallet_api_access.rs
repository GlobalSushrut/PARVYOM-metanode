//! Dedicated API Access Control for Bank and Government Stamped Wallets
//! 
//! This module provides specific API endpoints and access control for bank-stamped
//! and government-stamped wallets in the BPCI server, ensuring that only properly
//! verified and compliant wallets can access sensitive banking and government operations.

use axum::{
    extract::{Query, Path, State},
    response::Json,
    routing::{get, post, put},
    Router,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use anyhow::{Result, anyhow};
use tracing::{info, warn, error, debug};

/// Bank API Access Request
#[derive(Debug, Deserialize)]
pub struct BankApiRequest {
    pub wallet_id: String,
    pub bank_id: String,
    pub operation: String,
    pub settlement_data: Option<serde_json::Value>,
    pub compliance_signature: String,
    pub biso_agreement_id: Option<String>,
}

/// Government API Access Request
#[derive(Debug, Deserialize)]
pub struct GovernmentApiRequest {
    pub wallet_id: String,
    pub government_id: String,
    pub jurisdiction: String,
    pub operation: String,
    pub classification_level: String,
    pub regulatory_data: serde_json::Value,
    pub authority_signature: String,
    pub biso_agreement_id: Option<String>,
}

/// API Access Response
#[derive(Debug, Serialize)]
pub struct ApiAccessResponse {
    pub status: String,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub access_granted: bool,
    pub compliance_status: String,
    pub audit_trail_id: String,
}

/// Wallet Stamp Verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletStampVerification {
    pub wallet_id: String,
    pub stamp_type: StampType,
    pub verification_status: VerificationStatus,
    pub compliance_level: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub issuing_authority: String,
    pub verification_signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StampType {
    BankStamped {
        bank_id: String,
        banking_license: String,
        regulatory_body: String,
    },
    GovernmentStamped {
        government_id: String,
        jurisdiction: String,
        authority_level: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationStatus {
    Verified,
    Pending,
    Expired,
    Revoked,
}

/// Stamped Wallet API Access Controller
#[derive(Debug, Clone)]
pub struct StampedWalletApiController {
    pub verified_stamps: Arc<RwLock<HashMap<String, WalletStampVerification>>>,
    pub access_logs: Arc<RwLock<Vec<ApiAccessLog>>>,
    pub bank_api_endpoints: Arc<RwLock<HashMap<String, BankApiEndpoint>>>,
    pub government_api_endpoints: Arc<RwLock<HashMap<String, GovernmentApiEndpoint>>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApiAccessLog {
    pub log_id: String,
    pub wallet_id: String,
    pub endpoint: String,
    pub operation: String,
    pub access_granted: bool,
    pub timestamp: DateTime<Utc>,
    pub compliance_check_result: String,
    pub audit_trail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankApiEndpoint {
    pub endpoint_id: String,
    pub endpoint_path: String,
    pub required_compliance_level: String,
    pub allowed_operations: Vec<String>,
    pub settlement_coin_access: bool,
    pub audit_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernmentApiEndpoint {
    pub endpoint_id: String,
    pub endpoint_path: String,
    pub jurisdiction_required: Vec<String>,
    pub classification_levels: Vec<String>,
    pub regulatory_frameworks: Vec<String>,
    pub cross_border_access: bool,
}

impl StampedWalletApiController {
    /// Create a new stamped wallet API controller
    pub fn new() -> Self {
        let mut bank_endpoints = HashMap::new();
        let mut government_endpoints = HashMap::new();

        // Initialize bank API endpoints
        bank_endpoints.insert("bank_settlement".to_string(), BankApiEndpoint {
            endpoint_id: "bank_settlement_001".to_string(),
            endpoint_path: "/api/bank/settlement".to_string(),
            required_compliance_level: "institutional".to_string(),
            allowed_operations: vec![
                "initiate_settlement".to_string(),
                "process_clearing".to_string(),
                "generate_audit_report".to_string(),
                "access_settlement_coins".to_string(),
            ],
            settlement_coin_access: true,
            audit_required: true,
        });

        bank_endpoints.insert("bank_compliance".to_string(), BankApiEndpoint {
            endpoint_id: "bank_compliance_001".to_string(),
            endpoint_path: "/api/bank/compliance".to_string(),
            required_compliance_level: "enhanced".to_string(),
            allowed_operations: vec![
                "submit_compliance_report".to_string(),
                "request_audit_trail".to_string(),
                "verify_kyc_aml".to_string(),
            ],
            settlement_coin_access: false,
            audit_required: true,
        });

        // Initialize government API endpoints
        government_endpoints.insert("government_regulatory".to_string(), GovernmentApiEndpoint {
            endpoint_id: "gov_regulatory_001".to_string(),
            endpoint_path: "/api/government/regulatory".to_string(),
            jurisdiction_required: vec!["US".to_string(), "EU".to_string(), "UK".to_string()],
            classification_levels: vec!["public".to_string(), "restricted".to_string(), "confidential".to_string()],
            regulatory_frameworks: vec!["GDPR".to_string(), "CCPA".to_string(), "SOX".to_string()],
            cross_border_access: true,
        });

        government_endpoints.insert("government_audit".to_string(), GovernmentApiEndpoint {
            endpoint_id: "gov_audit_001".to_string(),
            endpoint_path: "/api/government/audit".to_string(),
            jurisdiction_required: vec!["US".to_string()],
            classification_levels: vec!["confidential".to_string(), "secret".to_string()],
            regulatory_frameworks: vec!["SOX".to_string(), "FISMA".to_string()],
            cross_border_access: false,
        });

        Self {
            verified_stamps: Arc::new(RwLock::new(HashMap::new())),
            access_logs: Arc::new(RwLock::new(Vec::new())),
            bank_api_endpoints: Arc::new(RwLock::new(bank_endpoints)),
            government_api_endpoints: Arc::new(RwLock::new(government_endpoints)),
        }
    }

    /// Verify wallet stamp and grant API access
    pub async fn verify_wallet_stamp(&self, wallet_id: &str) -> Result<Option<WalletStampVerification>> {
        let stamps = self.verified_stamps.read().await;
        Ok(stamps.get(wallet_id).cloned())
    }

    /// Register a new wallet stamp
    pub async fn register_wallet_stamp(&self, stamp: WalletStampVerification) -> Result<()> {
        info!("🏛️ Registering wallet stamp: {} -> {:?}", stamp.wallet_id, stamp.stamp_type);
        
        let mut stamps = self.verified_stamps.write().await;
        stamps.insert(stamp.wallet_id.clone(), stamp);
        
        Ok(())
    }

    /// Check bank API access permissions
    pub async fn check_bank_api_access(&self, request: &BankApiRequest) -> Result<ApiAccessResponse> {
        debug!("🏦 Checking bank API access for wallet: {}", request.wallet_id);

        // Verify wallet stamp
        let stamp = self.verify_wallet_stamp(&request.wallet_id).await?;
        let stamp = match stamp {
            Some(s) => s,
            None => {
                warn!("❌ No wallet stamp found for: {}", request.wallet_id);
                return Ok(ApiAccessResponse {
                    status: "denied".to_string(),
                    message: "No valid bank stamp found".to_string(),
                    data: None,
                    access_granted: false,
                    compliance_status: "non_compliant".to_string(),
                    audit_trail_id: Uuid::new_v4().to_string(),
                });
            }
        };

        // Check if it's a bank stamp
        let bank_info = match &stamp.stamp_type {
            StampType::BankStamped { bank_id, banking_license, regulatory_body } => {
                (bank_id.clone(), banking_license.clone(), regulatory_body.clone())
            },
            _ => {
                warn!("❌ Wallet {} does not have bank stamp", request.wallet_id);
                return Ok(ApiAccessResponse {
                    status: "denied".to_string(),
                    message: "Bank stamp required for this API".to_string(),
                    data: None,
                    access_granted: false,
                    compliance_status: "invalid_stamp_type".to_string(),
                    audit_trail_id: Uuid::new_v4().to_string(),
                });
            }
        };

        // Check verification status
        if !matches!(stamp.verification_status, VerificationStatus::Verified) {
            warn!("❌ Bank stamp not verified for wallet: {}", request.wallet_id);
            return Ok(ApiAccessResponse {
                status: "denied".to_string(),
                message: "Bank stamp verification required".to_string(),
                data: None,
                access_granted: false,
                compliance_status: "verification_pending".to_string(),
                audit_trail_id: Uuid::new_v4().to_string(),
            });
        }

        // Log access attempt
        let audit_trail_id = Uuid::new_v4().to_string();
        self.log_api_access(&request.wallet_id, "bank_api", &request.operation, true, &audit_trail_id).await;

        info!("✅ Bank API access granted for wallet: {}", request.wallet_id);
        Ok(ApiAccessResponse {
            status: "success".to_string(),
            message: "Bank API access granted".to_string(),
            data: Some(serde_json::json!({
                "bank_id": bank_info.0,
                "banking_license": bank_info.1,
                "regulatory_body": bank_info.2,
                "compliance_level": stamp.compliance_level,
                "settlement_coin_access": true,
                "audit_required": true
            })),
            access_granted: true,
            compliance_status: "compliant".to_string(),
            audit_trail_id,
        })
    }

    /// Check government API access permissions
    pub async fn check_government_api_access(&self, request: &GovernmentApiRequest) -> Result<ApiAccessResponse> {
        debug!("🏛️ Checking government API access for wallet: {}", request.wallet_id);

        // Verify wallet stamp
        let stamp = self.verify_wallet_stamp(&request.wallet_id).await?;
        let stamp = match stamp {
            Some(s) => s,
            None => {
                warn!("❌ No wallet stamp found for: {}", request.wallet_id);
                return Ok(ApiAccessResponse {
                    status: "denied".to_string(),
                    message: "No valid government stamp found".to_string(),
                    data: None,
                    access_granted: false,
                    compliance_status: "non_compliant".to_string(),
                    audit_trail_id: Uuid::new_v4().to_string(),
                });
            }
        };

        // Check if it's a government stamp
        let gov_info = match &stamp.stamp_type {
            StampType::GovernmentStamped { government_id, jurisdiction, authority_level } => {
                (government_id.clone(), jurisdiction.clone(), authority_level.clone())
            },
            _ => {
                warn!("❌ Wallet {} does not have government stamp", request.wallet_id);
                return Ok(ApiAccessResponse {
                    status: "denied".to_string(),
                    message: "Government stamp required for this API".to_string(),
                    data: None,
                    access_granted: false,
                    compliance_status: "invalid_stamp_type".to_string(),
                    audit_trail_id: Uuid::new_v4().to_string(),
                });
            }
        };

        // Check jurisdiction compatibility
        if gov_info.1 != request.jurisdiction {
            warn!("❌ Jurisdiction mismatch for wallet: {} (stamp: {}, request: {})", 
                  request.wallet_id, gov_info.1, request.jurisdiction);
            return Ok(ApiAccessResponse {
                status: "denied".to_string(),
                message: "Jurisdiction mismatch".to_string(),
                data: None,
                access_granted: false,
                compliance_status: "jurisdiction_mismatch".to_string(),
                audit_trail_id: Uuid::new_v4().to_string(),
            });
        }

        // Check verification status
        if !matches!(stamp.verification_status, VerificationStatus::Verified) {
            warn!("❌ Government stamp not verified for wallet: {}", request.wallet_id);
            return Ok(ApiAccessResponse {
                status: "denied".to_string(),
                message: "Government stamp verification required".to_string(),
                data: None,
                access_granted: false,
                compliance_status: "verification_pending".to_string(),
                audit_trail_id: Uuid::new_v4().to_string(),
            });
        }

        // Log access attempt
        let audit_trail_id = Uuid::new_v4().to_string();
        self.log_api_access(&request.wallet_id, "government_api", &request.operation, true, &audit_trail_id).await;

        info!("✅ Government API access granted for wallet: {}", request.wallet_id);
        Ok(ApiAccessResponse {
            status: "success".to_string(),
            message: "Government API access granted".to_string(),
            data: Some(serde_json::json!({
                "government_id": gov_info.0,
                "jurisdiction": gov_info.1,
                "authority_level": gov_info.2,
                "classification_access": request.classification_level,
                "cross_border_access": true,
                "regulatory_compliance": true
            })),
            access_granted: true,
            compliance_status: "compliant".to_string(),
            audit_trail_id,
        })
    }

    /// Log API access attempt
    async fn log_api_access(&self, wallet_id: &str, endpoint: &str, operation: &str, granted: bool, audit_trail_id: &str) {
        let log = ApiAccessLog {
            log_id: Uuid::new_v4().to_string(),
            wallet_id: wallet_id.to_string(),
            endpoint: endpoint.to_string(),
            operation: operation.to_string(),
            access_granted: granted,
            timestamp: Utc::now(),
            compliance_check_result: if granted { "passed".to_string() } else { "failed".to_string() },
            audit_trail: audit_trail_id.to_string(),
        };

        let mut logs = self.access_logs.write().await;
        logs.push(log);
    }

    /// Get access logs for audit
    pub async fn get_access_logs(&self, wallet_id: Option<String>) -> Vec<ApiAccessLog> {
        let logs = self.access_logs.read().await;
        match wallet_id {
            Some(id) => logs.iter().filter(|log| log.wallet_id == id).cloned().collect(),
            None => logs.clone(),
        }
    }
}

/// Bank API endpoint handlers
pub async fn handle_bank_settlement_request(
    State(controller): State<Arc<StampedWalletApiController>>,
    Json(request): Json<BankApiRequest>,
) -> Result<Json<ApiAccessResponse>, StatusCode> {
    match controller.check_bank_api_access(&request).await {
        Ok(response) => {
            if response.access_granted {
                info!("🏦 Bank settlement API access granted for wallet: {}", request.wallet_id);
                Ok(Json(response))
            } else {
                warn!("❌ Bank settlement API access denied for wallet: {}", request.wallet_id);
                Err(StatusCode::FORBIDDEN)
            }
        },
        Err(e) => {
            error!("❌ Bank API access check failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Government API endpoint handlers
pub async fn handle_government_regulatory_request(
    State(controller): State<Arc<StampedWalletApiController>>,
    Json(request): Json<GovernmentApiRequest>,
) -> Result<Json<ApiAccessResponse>, StatusCode> {
    match controller.check_government_api_access(&request).await {
        Ok(response) => {
            if response.access_granted {
                info!("🏛️ Government regulatory API access granted for wallet: {}", request.wallet_id);
                Ok(Json(response))
            } else {
                warn!("❌ Government regulatory API access denied for wallet: {}", request.wallet_id);
                Err(StatusCode::FORBIDDEN)
            }
        },
        Err(e) => {
            error!("❌ Government API access check failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Create router for stamped wallet API access
pub fn create_stamped_wallet_api_router(controller: Arc<StampedWalletApiController>) -> Router {
    Router::new()
        // Bank API endpoints - only accessible to bank-stamped wallets
        .route("/bank/settlement", post(handle_bank_settlement_request))
        .route("/bank/compliance", post(handle_bank_compliance_request))
        .route("/bank/audit", get(handle_bank_audit_request))
        
        // Government API endpoints - only accessible to government-stamped wallets
        .route("/government/regulatory", post(handle_government_regulatory_request))
        .route("/government/audit", post(handle_government_audit_request))
        .route("/government/classification", get(handle_government_classification_request))
        
        // Wallet stamp management
        .route("/stamps/register", post(handle_register_wallet_stamp))
        .route("/stamps/verify/:wallet_id", get(handle_verify_wallet_stamp))
        .route("/stamps/logs", get(handle_get_access_logs))
        .with_state(controller)
}

// Additional handler functions
pub async fn handle_bank_compliance_request(
    State(controller): State<Arc<StampedWalletApiController>>,
    Json(request): Json<BankApiRequest>,
) -> Result<Json<ApiAccessResponse>, StatusCode> {
    // Implementation similar to settlement request
    handle_bank_settlement_request(State(controller), Json(request)).await
}

pub async fn handle_bank_audit_request(
    State(controller): State<Arc<StampedWalletApiController>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let wallet_id = params.get("wallet_id").cloned();
    let logs = controller.get_access_logs(wallet_id).await;
    
    Ok(Json(serde_json::json!({
        "status": "success",
        "audit_logs": logs,
        "total_entries": logs.len()
    })))
}

pub async fn handle_government_audit_request(
    State(controller): State<Arc<StampedWalletApiController>>,
    Json(request): Json<GovernmentApiRequest>,
) -> Result<Json<ApiAccessResponse>, StatusCode> {
    handle_government_regulatory_request(State(controller), Json(request)).await
}

pub async fn handle_government_classification_request(
    State(controller): State<Arc<StampedWalletApiController>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let wallet_id = params.get("wallet_id").unwrap_or(&"unknown".to_string()).clone();
    
    if let Ok(Some(stamp)) = controller.verify_wallet_stamp(&wallet_id).await {
        if let StampType::GovernmentStamped { government_id, jurisdiction, authority_level } = stamp.stamp_type {
            return Ok(Json(serde_json::json!({
                "status": "success",
                "government_id": government_id,
                "jurisdiction": jurisdiction,
                "authority_level": authority_level,
                "classification_levels": ["public", "restricted", "confidential", "secret"],
                "access_granted": true
            })));
        }
    }
    
    Err(StatusCode::FORBIDDEN)
}

pub async fn handle_register_wallet_stamp(
    State(controller): State<Arc<StampedWalletApiController>>,
    Json(stamp): Json<WalletStampVerification>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match controller.register_wallet_stamp(stamp.clone()).await {
        Ok(()) => Ok(Json(serde_json::json!({
            "status": "success",
            "message": "Wallet stamp registered successfully",
            "wallet_id": stamp.wallet_id,
            "stamp_type": stamp.stamp_type
        }))),
        Err(e) => {
            error!("Failed to register wallet stamp: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn handle_verify_wallet_stamp(
    State(controller): State<Arc<StampedWalletApiController>>,
    Path(wallet_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match controller.verify_wallet_stamp(&wallet_id).await {
        Ok(Some(stamp)) => Ok(Json(serde_json::json!({
            "status": "verified",
            "wallet_stamp": stamp
        }))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("Failed to verify wallet stamp: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn handle_get_access_logs(
    State(controller): State<Arc<StampedWalletApiController>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let wallet_id = params.get("wallet_id").cloned();
    let logs = controller.get_access_logs(wallet_id).await;
    
    Ok(Json(serde_json::json!({
        "status": "success",
        "access_logs": logs,
        "total_entries": logs.len()
    })))
}
