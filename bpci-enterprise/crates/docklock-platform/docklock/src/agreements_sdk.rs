use crate::error::{DockLockError, DockLockResult};
use crate::policy_engine::{PolicyContext, Policy, PolicyConfig, Agreement};

use crate::court::{Court, CourtConfig, CourtRegistry, AgreementEnforcementResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};
use uuid::Uuid;

/// Agreements SDK - High-level API for policy and agreement management
#[derive(Debug)]
pub struct AgreementsSDK {
    /// Court registry for managing courts
    registry: CourtRegistry,
    /// Default court for hosting policies and agreements
    default_court_id: Option<Uuid>,
}

/// SDK configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDKConfig {
    /// Default court configuration
    pub default_court_config: CourtConfig,
    /// SDK-wide settings
    pub enable_logging: bool,
    /// Maximum execution time for policy operations
    pub max_operation_time_ms: u64,
}

impl Default for SDKConfig {
    fn default() -> Self {
        Self {
            default_court_config: CourtConfig::default(),
            enable_logging: true,
            max_operation_time_ms: 5000,
        }
    }
}

/// Policy template for common use cases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyTemplate {
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Template category
    pub category: String,
    /// WASM bytecode template
    pub wasm_template: Vec<u8>,
    /// Default configuration
    pub default_config: PolicyConfig,
    /// Template parameters that can be customized
    pub parameters: HashMap<String, String>,
}

/// Agreement template for common use cases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementTemplate {
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Template category
    pub category: String,
    /// Template terms (with placeholders)
    pub terms_template: String,
    /// Required policy categories
    pub required_policy_categories: Vec<String>,
    /// Template parameters
    pub parameters: HashMap<String, String>,
}

/// SDK operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDKOperationResult<T> {
    /// Whether the operation was successful
    pub success: bool,
    /// Result data (if successful)
    pub data: Option<T>,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Operation metadata
    pub metadata: HashMap<String, String>,
}

impl<T> SDKOperationResult<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            metadata: HashMap::new(),
        }
    }

    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            metadata: HashMap::new(),
        }
    }
}

impl AgreementsSDK {
    /// Create a new Agreements SDK instance
    pub fn new(config: SDKConfig) -> DockLockResult<Self> {
        info!("Initializing Agreements SDK");
        
        let registry = CourtRegistry::new();
        
        // Create default court
        let default_court = Court::new(
            "Default Court".to_string(),
            "Default court for policies and agreements".to_string(),
            config.default_court_config,
        );
        
        let default_court_id = default_court.id;
        registry.register_court(default_court)?;
        
        Ok(Self {
            registry,
            default_court_id: Some(default_court_id),
        })
    }

    /// Create a new court
    pub fn create_court(
        &self,
        name: String,
        description: String,
        config: CourtConfig,
    ) -> DockLockResult<SDKOperationResult<Uuid>> {
        debug!("Creating new court: {}", name);
        
        let court = Court::new(name, description, config);
        let court_id = court.id;
        
        match self.registry.register_court(court) {
            Ok(_) => {
                info!("Court created successfully: {}", court_id);
                Ok(SDKOperationResult::success(court_id))
            }
            Err(e) => Ok(SDKOperationResult::failure(format!("Failed to create court: {}", e))),
        }
    }

    /// Deploy a policy to a court
    pub fn deploy_policy(
        &self,
        court_id: Option<Uuid>,
        name: String,
        version: String,
        wasm_bytecode: Vec<u8>,
        config: PolicyConfig,
    ) -> DockLockResult<SDKOperationResult<Uuid>> {
        let target_court_id = court_id.or(self.default_court_id)
            .ok_or_else(|| DockLockError::PolicyError("No court specified and no default court available".to_string()))?;
        
        debug!("Deploying policy: {} to court: {}", name, target_court_id);
        
        // Create policy
        let policy = Policy::new(name, version, wasm_bytecode, config)?;
        let policy_id = policy.id;
        
        // Get court and host policy
        if let Some(result) = self.registry.with_court(target_court_id, |court| {
            match court.host_policy(policy) {
                Ok(result) => {
                    if result.success {
                        info!("Policy deployed successfully: {}", policy_id);
                        SDKOperationResult::success(policy_id)
                    } else {
                        SDKOperationResult::failure(result.message)
                    }
                }
                Err(e) => SDKOperationResult::failure(format!("Failed to deploy policy: {}", e)),
            }
        }) {
            Ok(result)
        } else {
            Ok(SDKOperationResult::failure("Court not found".to_string()))
        }
    }

    /// Create an agreement
    pub fn create_agreement(
        &self,
        court_id: Option<Uuid>,
        name: String,
        version: String,
        parties: Vec<String>,
        policy_ids: Vec<Uuid>,
        terms: String,
        expires_at: u64,
    ) -> DockLockResult<SDKOperationResult<Uuid>> {
        let target_court_id = court_id.or(self.default_court_id)
            .ok_or_else(|| DockLockError::PolicyError("No court specified and no default court available".to_string()))?;
        
        debug!("Creating agreement: {} in court: {}", name, target_court_id);
        
        // Create agreement
        let agreement = Agreement::new(name, version, parties, policy_ids, terms, expires_at)?;
        let agreement_id = agreement.id;
        
        // Get court and host agreement
        if let Some(result) = self.registry.with_court(target_court_id, |court| {
            match court.host_agreement(agreement) {
                Ok(result) => {
                    if result.success {
                        info!("Agreement created successfully: {}", agreement_id);
                        SDKOperationResult::success(agreement_id)
                    } else {
                        SDKOperationResult::failure(result.message)
                    }
                }
                Err(e) => SDKOperationResult::failure(format!("Failed to create agreement: {}", e)),
            }
        }) {
            Ok(result)
        } else {
            Ok(SDKOperationResult::failure("Court not found".to_string()))
        }
    }

    /// Enforce an agreement
    pub fn enforce_agreement(
        &self,
        court_id: Option<Uuid>,
        agreement_id: Uuid,
        context: &PolicyContext,
    ) -> DockLockResult<SDKOperationResult<AgreementEnforcementResult>> {
        let target_court_id = court_id.or(self.default_court_id)
            .ok_or_else(|| DockLockError::PolicyError("No court specified and no default court available".to_string()))?;
        
        debug!("Enforcing agreement: {} in court: {}", agreement_id, target_court_id);
        
        if let Some(result) = self.registry.with_court(target_court_id, |court| {
            match court.enforce_agreement(agreement_id, context) {
                Ok(result) => {
                    info!("Agreement enforcement completed: {} - enforced: {}", agreement_id, result.enforced);
                    SDKOperationResult::success(result)
                }
                Err(e) => SDKOperationResult::failure(format!("Failed to enforce agreement: {}", e)),
            }
        }) {
            Ok(result)
        } else {
            Ok(SDKOperationResult::failure("Court not found".to_string()))
        }
    }

    /// Get policy templates
    pub fn get_policy_templates(&self) -> Vec<PolicyTemplate> {
        vec![
            PolicyTemplate {
                name: "Memory Limit Policy".to_string(),
                description: "Enforces memory usage limits for executions".to_string(),
                category: "resource_management".to_string(),
                wasm_template: self.get_memory_limit_wasm_template(),
                default_config: PolicyConfig {
                    is_pre_hook: true,
                    max_gas: 100_000,
                    priority: 100,
                    ..Default::default()
                },
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("max_memory_mb".to_string(), "1024".to_string());
                    params
                },
            },
            PolicyTemplate {
                name: "CPU Usage Policy".to_string(),
                description: "Enforces CPU usage limits for executions".to_string(),
                category: "resource_management".to_string(),
                wasm_template: self.get_cpu_usage_wasm_template(),
                default_config: PolicyConfig {
                    is_pre_hook: true,
                    max_gas: 100_000,
                    priority: 90,
                    ..Default::default()
                },
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("max_cpu_percent".to_string(), "80.0".to_string());
                    params
                },
            },
            PolicyTemplate {
                name: "Time Limit Policy".to_string(),
                description: "Enforces execution time limits".to_string(),
                category: "resource_management".to_string(),
                wasm_template: self.get_time_limit_wasm_template(),
                default_config: PolicyConfig {
                    is_post_hook: true,
                    max_gas: 50_000,
                    priority: 80,
                    ..Default::default()
                },
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("max_execution_seconds".to_string(), "300".to_string());
                    params
                },
            },
        ]
    }

    /// Get agreement templates
    pub fn get_agreement_templates(&self) -> Vec<AgreementTemplate> {
        vec![
            AgreementTemplate {
                name: "Service Level Agreement".to_string(),
                description: "Standard SLA for microservice execution".to_string(),
                category: "service_management".to_string(),
                terms_template: "Service provider agrees to maintain {availability_percent}% uptime with maximum response time of {max_response_time_ms}ms. Resource usage shall not exceed {max_memory_mb}MB memory and {max_cpu_percent}% CPU.".to_string(),
                required_policy_categories: vec!["resource_management".to_string()],
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("availability_percent".to_string(), "99.9".to_string());
                    params.insert("max_response_time_ms".to_string(), "1000".to_string());
                    params.insert("max_memory_mb".to_string(), "1024".to_string());
                    params.insert("max_cpu_percent".to_string(), "80.0".to_string());
                    params
                },
            },
            AgreementTemplate {
                name: "Data Processing Agreement".to_string(),
                description: "Agreement for data processing and privacy compliance".to_string(),
                category: "data_management".to_string(),
                terms_template: "Data processor agrees to handle data according to {privacy_standard} standards. Data retention period is {retention_days} days. All processing must comply with {jurisdiction} regulations.".to_string(),
                required_policy_categories: vec!["data_privacy".to_string(), "compliance".to_string()],
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("privacy_standard".to_string(), "GDPR".to_string());
                    params.insert("retention_days".to_string(), "90".to_string());
                    params.insert("jurisdiction".to_string(), "EU".to_string());
                    params
                },
            },
        ]
    }

    /// Create policy from template
    pub fn create_policy_from_template(
        &self,
        template_name: &str,
        parameters: HashMap<String, String>,
        version: String,
    ) -> DockLockResult<SDKOperationResult<Policy>> {
        debug!("Creating policy from template: {}", template_name);
        
        let templates = self.get_policy_templates();
        if let Some(template) = templates.iter().find(|t| t.name == template_name) {
            // Customize WASM bytecode with parameters (placeholder implementation)
            let customized_wasm = self.customize_wasm_template(&template.wasm_template, &parameters);
            
            let policy = Policy::new(
                template.name.clone(),
                version,
                customized_wasm,
                template.default_config.clone(),
            )?;
            
            Ok(SDKOperationResult::success(policy))
        } else {
            Ok(SDKOperationResult::failure(format!("Template not found: {}", template_name)))
        }
    }

    /// Create agreement from template
    pub fn create_agreement_from_template(
        &self,
        template_name: &str,
        parties: Vec<String>,
        policy_ids: Vec<Uuid>,
        parameters: HashMap<String, String>,
        version: String,
        expires_at: u64,
    ) -> DockLockResult<SDKOperationResult<Agreement>> {
        debug!("Creating agreement from template: {}", template_name);
        
        let templates = self.get_agreement_templates();
        if let Some(template) = templates.iter().find(|t| t.name == template_name) {
            // Customize terms with parameters
            let customized_terms = self.customize_terms_template(&template.terms_template, &parameters);
            
            let agreement = Agreement::new(
                template.name.clone(),
                version,
                parties,
                policy_ids,
                customized_terms,
                expires_at,
            )?;
            
            Ok(SDKOperationResult::success(agreement))
        } else {
            Ok(SDKOperationResult::failure(format!("Template not found: {}", template_name)))
        }
    }

    /// List all courts
    pub fn list_courts(&self) -> Vec<Uuid> {
        self.registry.list_courts()
    }

    /// Get court statistics
    pub fn get_court_stats(&self, court_id: Uuid) -> Option<crate::court::CourtStats> {
        self.registry.with_court(court_id, |court| court.get_stats())
    }

    // Helper methods for WASM template generation (placeholder implementations)
    
    fn get_memory_limit_wasm_template(&self) -> Vec<u8> {
        // Placeholder WASM bytecode for memory limit policy
        vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00] // WASM magic + version
    }
    
    fn get_cpu_usage_wasm_template(&self) -> Vec<u8> {
        // Placeholder WASM bytecode for CPU usage policy
        vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00] // WASM magic + version
    }
    
    fn get_time_limit_wasm_template(&self) -> Vec<u8> {
        // Placeholder WASM bytecode for time limit policy
        vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00] // WASM magic + version
    }
    
    fn customize_wasm_template(&self, template: &[u8], _parameters: &HashMap<String, String>) -> Vec<u8> {
        // Placeholder implementation - in reality, this would modify the WASM bytecode
        // to incorporate the provided parameters
        template.to_vec()
    }
    
    fn customize_terms_template(&self, template: &str, parameters: &HashMap<String, String>) -> String {
        let mut customized = template.to_string();
        
        // Replace placeholders with actual values
        for (key, value) in parameters {
            let placeholder = format!("{{{}}}", key);
            customized = customized.replace(&placeholder, value);
        }
        
        customized
    }
}

impl Default for AgreementsSDK {
    fn default() -> Self {
        Self::new(SDKConfig::default()).expect("Failed to create default AgreementsSDK")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy_engine::SystemState;

    #[test]
    fn test_sdk_creation() {
        let config = SDKConfig::default();
        let sdk = AgreementsSDK::new(config).unwrap();
        
        let courts = sdk.list_courts();
        assert_eq!(courts.len(), 1); // Default court
    }

    #[test]
    fn test_court_creation() {
        let config = SDKConfig::default();
        let sdk = AgreementsSDK::new(config).unwrap();
        
        let result = sdk.create_court(
            "Test Court".to_string(),
            "A test court".to_string(),
            CourtConfig::default(),
        ).unwrap();
        
        assert!(result.success);
        assert!(result.data.is_some());
        
        let courts = sdk.list_courts();
        assert_eq!(courts.len(), 2); // Default + new court
    }

    #[test]
    fn test_policy_deployment() {
        let config = SDKConfig::default();
        let sdk = AgreementsSDK::new(config).unwrap();
        
        let result = sdk.deploy_policy(
            None, // Use default court
            "test_policy".to_string(),
            "1.0.0".to_string(),
            vec![0x00, 0x61, 0x73, 0x6d],
            PolicyConfig::default(),
        ).unwrap();
        
        assert!(result.success);
        assert!(result.data.is_some());
    }

    #[test]
    fn test_agreement_creation() {
        let config = SDKConfig::default();
        let sdk = AgreementsSDK::new(config).unwrap();
        
        // First deploy a policy
        let policy_result = sdk.deploy_policy(
            None,
            "test_policy".to_string(),
            "1.0.0".to_string(),
            vec![0x00, 0x61, 0x73, 0x6d],
            PolicyConfig::default(),
        ).unwrap();
        
        let policy_id = policy_result.data.unwrap();
        
        // Now create an agreement
        let result = sdk.create_agreement(
            None,
            "test_agreement".to_string(),
            "1.0.0".to_string(),
            vec!["party1".to_string(), "party2".to_string()],
            vec![policy_id],
            "Terms and conditions".to_string(),
            0,
        ).unwrap();
        
        assert!(result.success);
        assert!(result.data.is_some());
    }

    #[test]
    fn test_policy_templates() {
        let config = SDKConfig::default();
        let sdk = AgreementsSDK::new(config).unwrap();
        
        let templates = sdk.get_policy_templates();
        assert!(!templates.is_empty());
        
        let memory_template = templates.iter()
            .find(|t| t.name == "Memory Limit Policy")
            .expect("Memory Limit Policy template should exist");
        
        assert_eq!(memory_template.category, "resource_management");
        assert!(!memory_template.parameters.is_empty());
    }

    #[test]
    fn test_agreement_templates() {
        let config = SDKConfig::default();
        let sdk = AgreementsSDK::new(config).unwrap();
        
        let templates = sdk.get_agreement_templates();
        assert!(!templates.is_empty());
        
        let sla_template = templates.iter()
            .find(|t| t.name == "Service Level Agreement")
            .expect("SLA template should exist");
        
        assert_eq!(sla_template.category, "service_management");
        assert!(!sla_template.parameters.is_empty());
    }

    #[test]
    fn test_policy_from_template() {
        let config = SDKConfig::default();
        let sdk = AgreementsSDK::new(config).unwrap();
        
        let mut parameters = HashMap::new();
        parameters.insert("max_memory_mb".to_string(), "2048".to_string());
        
        let result = sdk.create_policy_from_template(
            "Memory Limit Policy",
            parameters,
            "1.0.0".to_string(),
        ).unwrap();
        
        assert!(result.success);
        assert!(result.data.is_some());
        
        let policy = result.data.unwrap();
        assert_eq!(policy.name, "Memory Limit Policy");
    }

    #[test]
    fn test_agreement_enforcement() {
        let config = SDKConfig::default();
        let sdk = AgreementsSDK::new(config).unwrap();
        
        // Deploy a policy
        let policy_result = sdk.deploy_policy(
            None,
            "test_policy".to_string(),
            "1.0.0".to_string(),
            vec![0x00, 0x61, 0x73, 0x6d],
            PolicyConfig {
                is_pre_hook: true,
                ..Default::default()
            },
        ).unwrap();
        
        let policy_id = policy_result.data.unwrap();
        
        // Create an agreement
        let agreement_result = sdk.create_agreement(
            None,
            "test_agreement".to_string(),
            "1.0.0".to_string(),
            vec!["party1".to_string(), "party2".to_string()],
            vec![policy_id],
            "Terms and conditions".to_string(),
            0,
        ).unwrap();
        
        let agreement_id = agreement_result.data.unwrap();
        
        // Enforce the agreement
        let context = PolicyContext {
            execution_id: Uuid::new_v4(),
            timestamp: 1234567890,
            metadata: HashMap::new(),
            system_state: SystemState {
                block_height: 100,
                block_hash: [0u8; 32],
                available_memory: 1024 * 1024 * 1024,
                cpu_usage: 50.0,
            },
        };
        
        let result = sdk.enforce_agreement(None, agreement_id, &context).unwrap();
        assert!(result.success);
        
        let enforcement_result = result.data.unwrap();
        assert!(enforcement_result.enforced);
    }
}
