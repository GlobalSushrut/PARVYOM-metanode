use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Deployment information for applications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentInfo {
    pub deployment_id: String,
    pub app_name: String,
    pub image: Option<String>,
    pub replicas: u32,
    pub status: String,
    pub created_at: SystemTime,
    pub access_url: String,
    pub receipt_id: String,
}

/// Enterprise deployment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseDeploymentInfo {
    pub company: String,
    pub deployment_id: String,
    pub access_url: String,
    pub security_level: String,
    pub compliance_active: Vec<String>,
}

/// Test results structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResults {
    pub security: Option<String>,
    pub performance: Option<String>,
    pub compliance: Option<String>,
}

impl TestResults {
    pub fn new() -> Self {
        Self {
            security: None,
            performance: None,
            compliance: None,
        }
    }
}
