//! Government API Request/Response Types for SmartContract++ Deployment

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::government_layer::{GovernmentSmartContract, GovernmentApiAccess, ExecutionContext, ExecutionResult};

/// SmartContract++ Deployment Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartContractDeploymentRequest {
    pub jurisdiction_id: String,
    pub contract: GovernmentSmartContract,
    pub government_signature: String,
    pub deployment_authorization: String,
}

/// SmartContract++ Deployment Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartContractDeploymentResponse {
    pub success: bool,
    pub deployment_id: Option<String>,
    pub message: String,
    pub contract_hash: Option<String>,
    pub deployment_timestamp: DateTime<Utc>,
}

/// SmartContract++ Examples Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartContractExamplesResponse {
    pub success: bool,
    pub examples: HashMap<String, String>,
    pub description: String,
}

/// Jurisdiction Contracts Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JurisdictionContractsResponse {
    pub success: bool,
    pub jurisdiction_id: String,
    pub contracts: Vec<GovernmentSmartContract>,
    pub total_contracts: usize,
}

/// Contract Execution Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractExecutionRequest {
    pub jurisdiction_id: String,
    pub contract_id: String,
    pub execution_context: ExecutionContext,
    pub execution_authorization: String,
}

/// Contract Execution Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractExecutionResponse {
    pub success: bool,
    pub execution_id: String,
    pub result_data: HashMap<String, serde_json::Value>,
    pub gas_used: u64,
    pub execution_time_ms: u64,
}

/// API Access Setup Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAccessSetupRequest {
    pub government_id: String,
    pub jurisdiction_id: String,
    pub api_config: GovernmentApiAccess,
    pub setup_authorization: String,
}

/// API Access Setup Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAccessSetupResponse {
    pub success: bool,
    pub access_id: Option<String>,
    pub message: String,
    pub stamped_wallet_verified: bool,
    pub bpci_connections_established: bool,
}

/// API Access Validation Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAccessValidationRequest {
    pub government_id: String,
    pub stamped_wallet_id: String,
    pub validation_signature: String,
}

/// API Access Validation Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAccessValidationResponse {
    pub success: bool,
    pub is_valid: bool,
    pub government_id: String,
    pub validation_timestamp: DateTime<Utc>,
    pub access_permissions: Vec<String>,
}

/// API Access Status Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAccessStatusResponse {
    pub success: bool,
    pub government_id: String,
    pub status: String,
    pub last_access: Option<DateTime<Utc>>,
    pub active_sessions: u32,
    pub rate_limit_status: String,
}

/// Helper functions for validation
pub async fn validate_government_signature(signature: &str, government_entity: &str) -> bool {
    // Implementation for government signature validation
    // This would verify cryptographic signatures from government authorities
    true // Placeholder
}

pub async fn validate_government_api_access(request: &ApiAccessValidationRequest) -> bool {
    // Implementation for government API access validation
    // This would check stamped wallet validity and government authority
    true // Placeholder
}
