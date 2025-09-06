use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// Missing type definitions for government layer

// GeographicBoundaries is now defined as a struct in multi_jurisdiction_smartcontract_deployment.rs
// Removing duplicate enum definition to resolve ambiguity

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnforcementMechanism {
    Legal,
    Regulatory,
    Administrative,
    Judicial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplicableLaw {
    LegalCode(String),
    Regulation(String),
    Statute(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetGovernanceTestResults {
    pub total_tests: u32,
    pub passed_tests: u32,
    pub failed_tests: u32,
    pub test_duration_ms: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedGovernmentApi {
    pub api_version: String,
    pub capabilities: Vec<String>,
    pub endpoints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossBorderMonitoringSystem {
    pub monitoring_id: String,
    pub active_jurisdictions: Vec<String>,
    pub compliance_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernmentAuthorityLevel {
    National {
        member_states: Vec<String>,
    },
    State {
        member_states: Vec<String>,
    },
    Regional {
        member_states: Vec<String>,
    },
    Local {
        member_states: Vec<String>,
    },
}
