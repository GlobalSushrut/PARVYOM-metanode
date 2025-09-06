//! Missing Types for Government Layer
//! 
//! This module defines all the missing types that are causing compilation errors
//! across the government layer modules.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// API Request/Response Types
#[derive(Debug, Deserialize)]
pub struct SmartContractDeploymentRequest {
    pub contract_name: String,
    pub jurisdiction_id: String,
    pub government_entity: String,
    pub contract_yaml: String,
    pub authority_level: String,
}

#[derive(Debug, Serialize)]
pub struct SmartContractDeploymentResponse {
    pub success: bool,
    pub contract_id: String,
    pub deployment_hash: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SmartContractExamplesResponse {
    pub examples: Vec<String>,
    pub total_count: usize,
    pub categories: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct JurisdictionContractsResponse {
    pub contracts: Vec<String>,
    pub jurisdiction_id: String,
    pub total_count: usize,
}

#[derive(Debug, Deserialize)]
pub struct ContractExecutionRequest {
    pub contract_id: String,
    pub execution_parameters: HashMap<String, String>,
    pub government_signature: String,
}

#[derive(Debug, Serialize)]
pub struct ContractExecutionResponse {
    pub success: bool,
    pub execution_id: String,
    pub result: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ApiAccessSetupRequest {
    pub government_id: String,
    pub jurisdiction: String,
    pub authority_level: String,
    pub requested_permissions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiAccessSetupResponse {
    pub success: bool,
    pub access_token: String,
    pub permissions: Vec<String>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ApiAccessValidationRequest {
    pub access_token: String,
    pub requested_endpoint: String,
    pub government_id: String,
}

#[derive(Debug, Serialize)]
pub struct ApiAccessValidationResponse {
    pub valid: bool,
    pub permissions: Vec<String>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ApiAccessStatusResponse {
    pub government_id: String,
    pub status: String,
    pub permissions: Vec<String>,
    pub last_accessed: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
}

// Test and Metrics Types
#[derive(Debug, Serialize)]
pub struct InternetGovernanceTestResults {
    pub test_suite: String,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub overall_score: f64,
    pub freedom_preservation_score: f64,
    pub autonomy_maintenance_score: f64,
    pub governance_effectiveness_score: f64,
    pub test_results: Vec<String>,
    pub test_timestamp: DateTime<Utc>,
    // Additional fields that are being accessed in the code
    pub test_name: String,
    pub success: bool,
    pub score: f64,
    pub details: String,
    pub freedom_impact: f64,
    pub autonomy_impact: f64,
    pub rights_protection_score: f64,
    pub multi_jurisdiction_coordination_score: f64,
    pub innovation_support_score: f64,
    pub emergency_response_score: f64,
    pub isp_governance_score: f64,
    pub datacenter_governance_score: f64,
    pub cdn_governance_score: f64,
    pub cable_governance_score: f64,
}

#[derive(Debug, Serialize)]
pub struct ContractExecutionMetrics {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub average_execution_time_ms: u64,
    pub last_execution: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct BpciIntegrationConfig {
    pub wallet_stamp_required: bool,
    pub audit_trail_enabled: bool,
    pub cross_jurisdiction_coordination: bool,
    pub api_access_level: String,
}

// Enum Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiAccessStatus {
    Active,
    Suspended,
    Expired,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernmentAuthorityLevel {
    Federal,
    National { country_code: String, iso_code: String },
    State { country_code: String, state_code: String },
    Regional { region_code: String, member_states: Vec<String> },
    Local,
    Municipal,
    Tribal,
    International,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernmentOperationStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernmentOperationType {
    Regulatory,
    Compliance,
    Audit,
    Investigation,
    Emergency,
    Routine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernmentSessionStatus {
    Active,
    Inactive,
    Suspended,
    Expired,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceStatus {
    Online,
    Offline,
    Maintenance,
    Degraded,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationLevel {
    Basic,
    Standard,
    Enhanced,
    Maximum,
    Government,
    Banking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenewalRequirements {
    pub renewal_period_days: u32,
    pub advance_notice_days: u32,
    pub verification_required: bool,
    pub documentation_required: Vec<String>,
    pub approval_authority: String,
}

// GeographicBoundaries is now defined as a struct in multi_jurisdiction_smartcontract_deployment.rs
// Removing duplicate enum definition to resolve ambiguity

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionFrequency {
    OnDemand,
    Scheduled,
    Periodic,
    Continuous,
    EventTriggered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnforcementMechanism {
    Regulatory,
    Legal,
    Administrative,
    Judicial,
    Economic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplicableLaw {
    LegalCode(String),
    Constitutional(String),
    Statutory(String),
    Regulatory(String),
    Common(String),
    International(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractDeploymentStatus {
    Pending,
    Active,
    Suspended,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernmentApiAccessLevel {
    Basic,
    Standard,
    Enhanced,
    Full,
    Emergency,
}

// Authority and Verification Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorityVerification {
    pub verification_method: String,
    pub verification_status: bool,
    pub verification_timestamp: DateTime<Utc>,
    pub verification_authority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRateLimits {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub emergency_bypass: bool,
}

// Permission Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiPermission {
    DeployContracts,
    ExecuteContracts,
    AuditAccess,
    EmergencyPowers,
    CrossJurisdiction,
    DataAccess,
    Custom(String),
}

impl From<String> for ApiPermission {
    fn from(s: String) -> Self {
        match s.as_str() {
            "deploy_contracts" => ApiPermission::DeployContracts,
            "execute_contracts" => ApiPermission::ExecuteContracts,
            "audit_access" => ApiPermission::AuditAccess,
            "emergency_powers" => ApiPermission::EmergencyPowers,
            "cross_jurisdiction" => ApiPermission::CrossJurisdiction,
            "data_access" => ApiPermission::DataAccess,
            _ => ApiPermission::Custom(s),
        }
    }
}

impl ToString for ApiPermission {
    fn to_string(&self) -> String {
        match self {
            ApiPermission::DeployContracts => "deploy_contracts".to_string(),
            ApiPermission::ExecuteContracts => "execute_contracts".to_string(),
            ApiPermission::AuditAccess => "audit_access".to_string(),
            ApiPermission::EmergencyPowers => "emergency_powers".to_string(),
            ApiPermission::CrossJurisdiction => "cross_jurisdiction".to_string(),
            ApiPermission::DataAccess => "data_access".to_string(),
            ApiPermission::Custom(s) => s.clone(),
        }
    }
}
