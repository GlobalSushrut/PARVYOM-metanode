use crate::error::{DockLockError, DockLockResult};
use crate::policy_engine::{Policy, Agreement, PolicyEngine, PolicyContext, PolicyResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{debug, info};
use uuid::Uuid;

/// Domain separation constant for court hashing
const COURT_HASH: u8 = 0x07;

/// Court - a container for hosting policies and agreements
#[derive(Debug)]
pub struct Court {
    /// Court identifier
    pub id: Uuid,
    /// Court name
    pub name: String,
    /// Court description
    pub description: String,
    /// Hosted policies
    policies: Arc<RwLock<HashMap<Uuid, Policy>>>,
    /// Hosted agreements
    agreements: Arc<RwLock<HashMap<Uuid, Agreement>>>,
    /// Policy engine for execution
    policy_engine: PolicyEngine,
    /// Court configuration
    config: CourtConfig,
    /// Court statistics
    stats: Arc<RwLock<CourtStats>>,
}

/// Court configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtConfig {
    /// Maximum number of policies that can be hosted
    pub max_policies: usize,
    /// Maximum number of agreements that can be hosted
    pub max_agreements: usize,
    /// Whether the court allows public policy registration
    pub allow_public_registration: bool,
    /// Court administrator identities
    pub administrators: Vec<String>,
    /// Court jurisdiction or domain
    pub jurisdiction: String,
}

impl Default for CourtConfig {
    fn default() -> Self {
        Self {
            max_policies: 1000,
            max_agreements: 10000,
            allow_public_registration: false,
            administrators: Vec::new(),
            jurisdiction: "default".to_string(),
        }
    }
}

/// Court statistics
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CourtStats {
    /// Total policies hosted
    pub total_policies: usize,
    /// Total agreements hosted
    pub total_agreements: usize,
    /// Total policy executions
    pub total_executions: u64,
    /// Total policy violations
    pub total_violations: u64,
    /// Active agreements (not expired)
    pub active_agreements: usize,
    /// Court uptime in seconds
    pub uptime_seconds: u64,
}

/// Court registry for managing multiple courts
#[derive(Debug)]
pub struct CourtRegistry {
    /// Registered courts
    courts: Arc<RwLock<HashMap<Uuid, Court>>>,
    /// Registry statistics
    stats: Arc<RwLock<CourtRegistryStats>>,
}

/// Court registry statistics
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CourtRegistryStats {
    /// Total courts registered
    pub total_courts: usize,
    /// Total policies across all courts
    pub total_policies: usize,
    /// Total agreements across all courts
    pub total_agreements: usize,
}

/// Court operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtOperationResult {
    /// Whether the operation was successful
    pub success: bool,
    /// Operation result message
    pub message: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Agreement enforcement result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementEnforcementResult {
    /// Agreement ID
    pub agreement_id: Uuid,
    /// Whether all policies passed
    pub enforced: bool,
    /// Policy execution results
    pub policy_results: Vec<PolicyResult>,
    /// Enforcement message
    pub message: String,
}

impl Court {
    /// Create a new court
    pub fn new(name: String, description: String, config: CourtConfig) -> Self {
        let id = Uuid::new_v4();
        info!("Creating new court: {} ({})", name, id);
        
        Self {
            id,
            name,
            description,
            policies: Arc::new(RwLock::new(HashMap::new())),
            agreements: Arc::new(RwLock::new(HashMap::new())),
            policy_engine: PolicyEngine::new(),
            config,
            stats: Arc::new(RwLock::new(CourtStats::default())),
        }
    }

    /// Host a policy in this court
    pub fn host_policy(&self, policy: Policy) -> DockLockResult<CourtOperationResult> {
        debug!("Hosting policy: {} in court: {}", policy.name, self.name);
        
        // Check capacity
        {
            let policies = self.policies.read().unwrap();
            if policies.len() >= self.config.max_policies {
                return Ok(CourtOperationResult {
                    success: false,
                    message: "Court has reached maximum policy capacity".to_string(),
                    metadata: HashMap::new(),
                });
            }
        }

        // Load policy into the engine
        self.policy_engine.load_policy(policy.clone())?;
        
        // Store policy in court
        {
            let mut policies = self.policies.write().unwrap();
            policies.insert(policy.id, policy);
        }

        // Update statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_policies += 1;
        }

        info!("Policy hosted successfully in court: {}", self.name);
        Ok(CourtOperationResult {
            success: true,
            message: "Policy hosted successfully".to_string(),
            metadata: HashMap::new(),
        })
    }

    /// Host an agreement in this court
    pub fn host_agreement(&self, agreement: Agreement) -> DockLockResult<CourtOperationResult> {
        debug!("Hosting agreement: {} in court: {}", agreement.name, self.name);
        
        // Check capacity
        {
            let agreements = self.agreements.read().unwrap();
            if agreements.len() >= self.config.max_agreements {
                return Ok(CourtOperationResult {
                    success: false,
                    message: "Court has reached maximum agreement capacity".to_string(),
                    metadata: HashMap::new(),
                });
            }
        }

        // Validate agreement is not expired
        if agreement.is_expired() {
            return Ok(CourtOperationResult {
                success: false,
                message: "Cannot host expired agreement".to_string(),
                metadata: HashMap::new(),
            });
        }

        // Verify all referenced policies exist
        {
            let policies = self.policies.read().unwrap();
            for policy_id in &agreement.policy_ids {
                if !policies.contains_key(policy_id) {
                    return Ok(CourtOperationResult {
                        success: false,
                        message: format!("Referenced policy not found: {}", policy_id),
                        metadata: HashMap::new(),
                    });
                }
            }
        }

        // Store agreement in court
        {
            let mut agreements = self.agreements.write().unwrap();
            agreements.insert(agreement.id, agreement);
        }

        // Update statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_agreements += 1;
            stats.active_agreements += 1;
        }

        info!("Agreement hosted successfully in court: {}", self.name);
        Ok(CourtOperationResult {
            success: true,
            message: "Agreement hosted successfully".to_string(),
            metadata: HashMap::new(),
        })
    }

    /// Enforce an agreement by executing its policies
    pub fn enforce_agreement(
        &self, 
        agreement_id: Uuid, 
        context: &PolicyContext
    ) -> DockLockResult<AgreementEnforcementResult> {
        debug!("Enforcing agreement: {} in court: {}", agreement_id, self.name);
        
        // Get agreement
        let agreement = {
            let agreements = self.agreements.read().unwrap();
            match agreements.get(&agreement_id) {
                Some(agreement) => agreement.clone(),
                None => {
                    return Ok(AgreementEnforcementResult {
                        agreement_id,
                        enforced: false,
                        policy_results: Vec::new(),
                        message: "Agreement not found".to_string(),
                    });
                }
            }
        };

        // Check if agreement is expired
        if agreement.is_expired() {
            return Ok(AgreementEnforcementResult {
                agreement_id,
                enforced: false,
                policy_results: Vec::new(),
                message: "Agreement has expired".to_string(),
            });
        }

        // Execute all policies associated with the agreement
        let mut policy_results = Vec::new();
        let mut all_enforced = true;

        // Execute pre-hooks
        let pre_results = self.policy_engine.execute_pre_hooks(context)?;
        for result in &pre_results {
            if !result.allowed {
                all_enforced = false;
            }
        }
        policy_results.extend(pre_results);

        // Execute post-hooks
        let post_results = self.policy_engine.execute_post_hooks(context)?;
        for result in &post_results {
            if !result.allowed {
                all_enforced = false;
            }
        }
        policy_results.extend(post_results);

        // Update statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_executions += policy_results.len() as u64;
            if !all_enforced {
                stats.total_violations += 1;
            }
        }

        let message = if all_enforced {
            "Agreement enforced successfully".to_string()
        } else {
            "Agreement enforcement failed - policy violations detected".to_string()
        };

        info!("Agreement enforcement completed: {} - {}", agreement_id, message);
        
        Ok(AgreementEnforcementResult {
            agreement_id,
            enforced: all_enforced,
            policy_results,
            message,
        })
    }

    /// Get court statistics
    pub fn get_stats(&self) -> CourtStats {
        let mut stats = self.stats.read().unwrap().clone();
        
        // Update active agreements count
        let agreements = self.agreements.read().unwrap();
        stats.active_agreements = agreements.values()
            .filter(|agreement| !agreement.is_expired())
            .count();
            
        stats
    }

    /// List all hosted policies
    pub fn list_policies(&self) -> Vec<Uuid> {
        self.policies.read().unwrap().keys().cloned().collect()
    }

    /// List all hosted agreements
    pub fn list_agreements(&self) -> Vec<Uuid> {
        self.agreements.read().unwrap().keys().cloned().collect()
    }

    /// Get a specific policy
    pub fn get_policy(&self, policy_id: Uuid) -> Option<Policy> {
        self.policies.read().unwrap().get(&policy_id).cloned()
    }

    /// Get a specific agreement
    pub fn get_agreement(&self, agreement_id: Uuid) -> Option<Agreement> {
        self.agreements.read().unwrap().get(&agreement_id).cloned()
    }

    /// Remove a policy from the court
    pub fn remove_policy(&self, policy_id: Uuid) -> DockLockResult<CourtOperationResult> {
        debug!("Removing policy: {} from court: {}", policy_id, self.name);
        
        // Check if policy is referenced by any agreements
        {
            let agreements = self.agreements.read().unwrap();
            for agreement in agreements.values() {
                if agreement.policy_ids.contains(&policy_id) {
                    return Ok(CourtOperationResult {
                        success: false,
                        message: format!("Policy is referenced by agreement: {}", agreement.id),
                        metadata: HashMap::new(),
                    });
                }
            }
        }

        // Remove from policy engine
        self.policy_engine.remove_policy(policy_id)?;
        
        // Remove from court
        let removed = {
            let mut policies = self.policies.write().unwrap();
            policies.remove(&policy_id).is_some()
        };

        if removed {
            // Update statistics
            let mut stats = self.stats.write().unwrap();
            stats.total_policies -= 1;
            
            info!("Policy removed successfully from court: {}", self.name);
            Ok(CourtOperationResult {
                success: true,
                message: "Policy removed successfully".to_string(),
                metadata: HashMap::new(),
            })
        } else {
            Ok(CourtOperationResult {
                success: false,
                message: "Policy not found".to_string(),
                metadata: HashMap::new(),
            })
        }
    }

    /// Remove an agreement from the court
    pub fn remove_agreement(&self, agreement_id: Uuid) -> DockLockResult<CourtOperationResult> {
        debug!("Removing agreement: {} from court: {}", agreement_id, self.name);
        
        let removed = {
            let mut agreements = self.agreements.write().unwrap();
            agreements.remove(&agreement_id).is_some()
        };

        if removed {
            // Update statistics
            let mut stats = self.stats.write().unwrap();
            stats.total_agreements -= 1;
            
            info!("Agreement removed successfully from court: {}", self.name);
            Ok(CourtOperationResult {
                success: true,
                message: "Agreement removed successfully".to_string(),
                metadata: HashMap::new(),
            })
        } else {
            Ok(CourtOperationResult {
                success: false,
                message: "Agreement not found".to_string(),
                metadata: HashMap::new(),
            })
        }
    }

    /// Compute court hash for integrity verification
    pub fn compute_hash(&self) -> DockLockResult<[u8; 32]> {
        let policies = self.policies.read().unwrap();
        let agreements = self.agreements.read().unwrap();
        
        let court_data = serde_cbor::to_vec(&(
            &self.id,
            &self.name,
            &self.description,
            &self.config,
            policies.keys().collect::<Vec<_>>(),
            agreements.keys().collect::<Vec<_>>(),
        )).map_err(|e| DockLockError::EncodingError(format!("Failed to encode court: {}", e)))?;

        // Apply domain separation
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[COURT_HASH]);
        hasher.update(&court_data);
        Ok(hasher.finalize().into())
    }
}

impl CourtRegistry {
    /// Create a new court registry
    pub fn new() -> Self {
        info!("Creating new court registry");
        Self {
            courts: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(CourtRegistryStats::default())),
        }
    }

    /// Register a court in the registry
    pub fn register_court(&self, court: Court) -> DockLockResult<CourtOperationResult> {
        info!("Registering court: {} ({})", court.name, court.id);
        
        let court_id = court.id;
        {
            let mut courts = self.courts.write().unwrap();
            courts.insert(court_id, court);
        }

        // Update statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_courts += 1;
        }

        Ok(CourtOperationResult {
            success: true,
            message: "Court registered successfully".to_string(),
            metadata: HashMap::new(),
        })
    }

    /// Get a court by ID (returns a reference with proper lifetime management)
    pub fn with_court<F, R>(&self, court_id: Uuid, f: F) -> Option<R>
    where
        F: FnOnce(&Court) -> R,
    {
        let courts = self.courts.read().unwrap();
        courts.get(&court_id).map(f)
    }

    /// List all registered courts
    pub fn list_courts(&self) -> Vec<Uuid> {
        self.courts.read().unwrap().keys().cloned().collect()
    }

    /// Get registry statistics
    pub fn get_stats(&self) -> CourtRegistryStats {
        let mut stats = self.stats.read().unwrap().clone();
        
        // Update aggregated statistics
        let courts = self.courts.read().unwrap();
        stats.total_policies = courts.values()
            .map(|court| court.get_stats().total_policies)
            .sum();
        stats.total_agreements = courts.values()
            .map(|court| court.get_stats().total_agreements)
            .sum();
            
        stats
    }

    /// Find courts by jurisdiction
    pub fn find_courts_by_jurisdiction(&self, jurisdiction: &str) -> Vec<Uuid> {
        let courts = self.courts.read().unwrap();
        courts.values()
            .filter(|court| court.config.jurisdiction == jurisdiction)
            .map(|court| court.id)
            .collect()
    }
}

impl Default for CourtRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy_engine::{PolicyConfig, SystemState};

    #[test]
    fn test_court_creation() {
        let config = CourtConfig::default();
        let court = Court::new(
            "Test Court".to_string(),
            "A test court for policies and agreements".to_string(),
            config,
        );
        
        assert_eq!(court.name, "Test Court");
        assert_eq!(court.list_policies().len(), 0);
        assert_eq!(court.list_agreements().len(), 0);
    }

    #[test]
    fn test_court_registry() {
        let registry = CourtRegistry::new();
        let config = CourtConfig::default();
        let court = Court::new(
            "Test Court".to_string(),
            "A test court".to_string(),
            config,
        );
        
        let court_id = court.id;
        let result = registry.register_court(court).unwrap();
        assert!(result.success);
        
        let courts = registry.list_courts();
        assert_eq!(courts.len(), 1);
        assert!(courts.contains(&court_id));
    }

    #[test]
    fn test_policy_hosting() {
        let config = CourtConfig::default();
        let court = Court::new(
            "Test Court".to_string(),
            "A test court".to_string(),
            config,
        );
        
        let policy_config = PolicyConfig::default();
        let policy = Policy::new(
            "test_policy".to_string(),
            "1.0.0".to_string(),
            vec![0x00, 0x61, 0x73, 0x6d],
            policy_config,
        ).unwrap();
        
        let result = court.host_policy(policy).unwrap();
        assert!(result.success);
        
        let policies = court.list_policies();
        assert_eq!(policies.len(), 1);
    }

    #[test]
    fn test_agreement_hosting() {
        let config = CourtConfig::default();
        let court = Court::new(
            "Test Court".to_string(),
            "A test court".to_string(),
            config,
        );
        
        // First host a policy
        let policy_config = PolicyConfig::default();
        let policy = Policy::new(
            "test_policy".to_string(),
            "1.0.0".to_string(),
            vec![0x00, 0x61, 0x73, 0x6d],
            policy_config,
        ).unwrap();
        
        let policy_id = policy.id;
        court.host_policy(policy).unwrap();
        
        // Now host an agreement that references the policy
        let agreement = Agreement::new(
            "test_agreement".to_string(),
            "1.0.0".to_string(),
            vec!["party1".to_string(), "party2".to_string()],
            vec![policy_id],
            "Terms and conditions".to_string(),
            0, // Never expires
        ).unwrap();
        
        let result = court.host_agreement(agreement).unwrap();
        assert!(result.success);
        
        let agreements = court.list_agreements();
        assert_eq!(agreements.len(), 1);
    }

    #[test]
    fn test_agreement_enforcement() {
        let config = CourtConfig::default();
        let court = Court::new(
            "Test Court".to_string(),
            "A test court".to_string(),
            config,
        );
        
        // Host a policy
        let policy_config = PolicyConfig {
            is_pre_hook: true,
            ..Default::default()
        };
        let policy = Policy::new(
            "test_policy".to_string(),
            "1.0.0".to_string(),
            vec![0x00, 0x61, 0x73, 0x6d],
            policy_config,
        ).unwrap();
        
        let policy_id = policy.id;
        court.host_policy(policy).unwrap();
        
        // Host an agreement
        let agreement = Agreement::new(
            "test_agreement".to_string(),
            "1.0.0".to_string(),
            vec!["party1".to_string(), "party2".to_string()],
            vec![policy_id],
            "Terms and conditions".to_string(),
            0,
        ).unwrap();
        
        let agreement_id = agreement.id;
        court.host_agreement(agreement).unwrap();
        
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
        
        let result = court.enforce_agreement(agreement_id, &context).unwrap();
        assert!(result.enforced);
        assert!(!result.policy_results.is_empty());
    }

    #[test]
    fn test_court_statistics() {
        let config = CourtConfig::default();
        let court = Court::new(
            "Test Court".to_string(),
            "A test court".to_string(),
            config,
        );
        
        let stats = court.get_stats();
        assert_eq!(stats.total_policies, 0);
        assert_eq!(stats.total_agreements, 0);
        assert_eq!(stats.total_executions, 0);
    }
}
