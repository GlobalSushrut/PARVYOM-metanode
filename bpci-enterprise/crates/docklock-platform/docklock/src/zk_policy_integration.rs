use crate::error::{DockLockError, DockLockResult};
use crate::policy_engine::{PolicyContext, PolicyConfig, PolicyResult};
use crate::court::Court;
use crate::zk_proofs::{ZkClaim, ZkVerifier, ZkVerificationResult, ZkClaimType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Extended policy context with ZK proof support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkPolicyContext {
    /// Base policy context
    pub base_context: PolicyContext,
    /// ZK claims to be verified
    pub zk_claims: Vec<ZkClaim>,
    /// ZK verification results (populated after verification)
    pub zk_verification_results: Vec<ZkVerificationResult>,
    /// Whether ZK path is active
    pub zk_enabled: bool,
    /// ZK-specific metadata
    pub zk_metadata: HashMap<String, String>,
}

/// ZK-enabled policy result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkPolicyResult {
    /// Base policy result
    pub base_result: PolicyResult,
    /// ZK verification results
    pub zk_results: Vec<ZkVerificationResult>,
    /// Whether ZK verification passed
    pub zk_verified: bool,
    /// ZK-specific metadata
    pub zk_metadata: HashMap<String, String>,
}

/// ZK policy engine that extends the base policy engine
#[derive(Debug)]
pub struct ZkPolicyEngine {
    /// ZK verifier for proof verification
    zk_verifier: Arc<RwLock<ZkVerifier>>,
    /// ZK policy engine statistics
    stats: Arc<RwLock<ZkPolicyEngineStats>>,
}

/// ZK policy engine statistics
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ZkPolicyEngineStats {
    /// Total ZK policy executions
    pub total_zk_executions: u64,
    /// ZK verifications performed
    pub total_zk_verifications: u64,
    /// Successful ZK verifications
    pub successful_zk_verifications: u64,
    /// Failed ZK verifications
    pub failed_zk_verifications: u64,
    /// Total ZK gas consumed
    pub total_zk_gas_consumed: u64,
}

/// ZK-enabled court for hosting ZK policies and agreements
#[derive(Debug)]
pub struct ZkCourt {
    /// Base court functionality
    base_court: Court,
    /// ZK policy engine
    zk_engine: ZkPolicyEngine,
    /// ZK court statistics
    stats: Arc<RwLock<ZkCourtStats>>,
}

/// ZK court statistics
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ZkCourtStats {
    /// Total ZK agreements enforced
    pub total_zk_agreements: u64,
    /// ZK policies executed
    pub total_zk_policies: u64,
    /// ZK claims verified
    pub total_zk_claims: u64,
}

/// ZK policy template for common ZK use cases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkPolicyTemplate {
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Required ZK claim types
    pub required_claim_types: Vec<ZkClaimType>,
    /// Base policy configuration
    pub base_config: PolicyConfig,
    /// ZK-specific parameters
    pub zk_parameters: HashMap<String, String>,
}

impl ZkPolicyContext {
    /// Create a new ZK policy context
    pub fn new(base_context: PolicyContext, zk_claims: Vec<ZkClaim>) -> Self {
        Self {
            base_context,
            zk_claims,
            zk_verification_results: Vec::new(),
            zk_enabled: true,
            zk_metadata: HashMap::new(),
        }
    }

    /// Create from base context without ZK claims
    pub fn from_base_context(base_context: PolicyContext) -> Self {
        Self {
            base_context,
            zk_claims: Vec::new(),
            zk_verification_results: Vec::new(),
            zk_enabled: false,
            zk_metadata: HashMap::new(),
        }
    }

    /// Add a ZK claim to the context
    pub fn add_zk_claim(&mut self, claim: ZkClaim) {
        self.zk_claims.push(claim);
        self.zk_enabled = true;
    }

    /// Check if ZK verification is required
    pub fn requires_zk_verification(&self) -> bool {
        self.zk_enabled && !self.zk_claims.is_empty()
    }

    /// Get ZK claims by type
    pub fn get_claims_by_type(&self, claim_type: &ZkClaimType) -> Vec<&ZkClaim> {
        self.zk_claims.iter()
            .filter(|claim| &claim.claim_type == claim_type)
            .collect()
    }
}

impl ZkPolicyEngine {
    /// Create a new ZK policy engine
    pub fn new() -> Self {
        Self {
            zk_verifier: Arc::new(RwLock::new(ZkVerifier::new())),
            stats: Arc::new(RwLock::new(ZkPolicyEngineStats::default())),
        }
    }

    /// Add a verification key to the ZK verifier
    pub fn add_verification_key(&self, key_id: String, key_data: Vec<u8>) {
        let mut verifier = self.zk_verifier.write().unwrap();
        verifier.add_verification_key(key_id, key_data);
    }

    /// Execute ZK-enabled policy with pre-hooks
    pub fn execute_zk_pre_hooks(&self, context: &mut ZkPolicyContext) -> DockLockResult<ZkPolicyResult> {
        debug!("Executing ZK-enabled pre-hooks");
        
        // Verify ZK claims first if present
        if context.requires_zk_verification() {
            self.verify_zk_claims(context)?;
        }

        // Execute base policy logic (placeholder)
        let zk_base_result = self.simulate_base_policy_execution(context)?;
        
        // Create ZK policy result
        let zk_result = ZkPolicyResult {
            base_result: zk_base_result.base_result,
            zk_results: context.zk_verification_results.clone(),
            zk_verified: context.zk_verification_results.iter().all(|r| r.valid),
            zk_metadata: context.zk_metadata.clone(),
        };

        // Update statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_zk_executions += 1;
            stats.total_zk_verifications += context.zk_verification_results.len() as u64;
            stats.successful_zk_verifications += context.zk_verification_results.iter()
                .filter(|r| r.valid).count() as u64;
            stats.failed_zk_verifications += context.zk_verification_results.iter()
                .filter(|r| !r.valid).count() as u64;
            stats.total_zk_gas_consumed += context.zk_verification_results.iter()
                .map(|r| r.gas_consumed).sum::<u64>();
        }

        Ok(zk_result)
    }

    /// Execute ZK-enabled policy with post-hooks
    pub fn execute_zk_post_hooks(&self, context: &mut ZkPolicyContext) -> DockLockResult<ZkPolicyResult> {
        debug!("Executing ZK-enabled post-hooks");
        
        // Similar to pre-hooks but for post-execution validation
        if context.requires_zk_verification() {
            self.verify_zk_claims(context)?;
        }

        let zk_base_result = self.simulate_base_policy_execution(context)?;
        
        let zk_result = ZkPolicyResult {
            base_result: zk_base_result.base_result,
            zk_results: context.zk_verification_results.clone(),
            zk_verified: context.zk_verification_results.iter().all(|r| r.valid),
            zk_metadata: context.zk_metadata.clone(),
        };

        // Update statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_zk_executions += 1;
            stats.total_zk_verifications += context.zk_verification_results.len() as u64;
            stats.successful_zk_verifications += context.zk_verification_results.iter()
                .filter(|r| r.valid).count() as u64;
            stats.failed_zk_verifications += context.zk_verification_results.iter()
                .filter(|r| !r.valid).count() as u64;
            stats.total_zk_gas_consumed += context.zk_verification_results.iter()
                .map(|r| r.gas_consumed).sum::<u64>();
        }

        Ok(zk_result)
    }

    /// Verify ZK claims in the context
    fn verify_zk_claims(&self, context: &mut ZkPolicyContext) -> DockLockResult<()> {
        debug!("Verifying {} ZK claims", context.zk_claims.len());
        
        let mut verifier = self.zk_verifier.write().unwrap();
        let mut verification_results = Vec::new();

        for claim in &context.zk_claims {
            debug!("Verifying ZK claim: {} (type: {:?})", claim.id, claim.claim_type);
            
            // Verify claim integrity first
            if !claim.verify_integrity()? {
                warn!("ZK claim integrity verification failed: {}", claim.id);
                verification_results.push(ZkVerificationResult {
                    valid: false,
                    message: "Claim integrity verification failed".to_string(),
                    metadata: HashMap::new(),
                    gas_consumed: 1000,
                    verification_time_ms: 0,
                });
                continue;
            }

            // Verify the associated proof
            let result = verifier.verify_proof(&claim.proof)?;
            verification_results.push(result);
        }

        context.zk_verification_results = verification_results;
        
        // Update ZK metadata
        context.zk_metadata.insert(
            "zk_claims_verified".to_string(),
            context.zk_claims.len().to_string()
        );
        context.zk_metadata.insert(
            "zk_verification_success".to_string(),
            context.zk_verification_results.iter().all(|r| r.valid).to_string()
        );

        Ok(())
    }

    /// Simulate base policy execution (placeholder)
    fn simulate_base_policy_execution(&self, context: &ZkPolicyContext) -> DockLockResult<ZkPolicyResult> {
        // Placeholder implementation - in reality this would integrate with the actual policy engine
        
        let allowed = if context.requires_zk_verification() {
            // If ZK verification is required, check if all proofs are valid
            context.zk_verification_results.iter().all(|r| r.valid)
        } else {
            // Default policy logic
            context.base_context.system_state.available_memory > 1024 * 1024 &&
            context.base_context.system_state.cpu_usage < 90.0
        };

        let reason = if context.requires_zk_verification() {
            if allowed {
                "ZK verification successful, policy allows execution".to_string()
            } else {
                "ZK verification failed, policy denies execution".to_string()
            }
        } else {
            "Standard policy evaluation completed".to_string()
        };

        let base_result = PolicyResult {
            allowed,
            reason,
            metadata: context.zk_metadata.clone(),
            gas_consumed: 5000, // Base gas consumption
        };

        Ok(ZkPolicyResult {
            base_result,
            zk_results: Vec::new(),
            zk_verified: true,
            zk_metadata: context.zk_metadata.clone(),
        })
    }

    /// Get ZK policy engine statistics
    pub fn get_stats(&self) -> ZkPolicyEngineStats {
        self.stats.read().unwrap().clone()
    }

    /// Get ZK policy templates
    pub fn get_zk_policy_templates(&self) -> Vec<ZkPolicyTemplate> {
        vec![
            ZkPolicyTemplate {
                name: "Age Verification Policy".to_string(),
                description: "Verify user age without revealing exact age".to_string(),
                required_claim_types: vec![ZkClaimType::AgeVerification],
                base_config: PolicyConfig {
                    is_pre_hook: true,
                    max_gas: 100_000,
                    priority: 150,
                    ..Default::default()
                },
                zk_parameters: {
                    let mut params = HashMap::new();
                    params.insert("min_age".to_string(), "18".to_string());
                    params.insert("verification_key_id".to_string(), "age_verification_key".to_string());
                    params
                },
            },
            ZkPolicyTemplate {
                name: "Balance Verification Policy".to_string(),
                description: "Verify sufficient balance without revealing exact amount".to_string(),
                required_claim_types: vec![ZkClaimType::BalanceVerification],
                base_config: PolicyConfig {
                    is_pre_hook: true,
                    max_gas: 150_000,
                    priority: 140,
                    ..Default::default()
                },
                zk_parameters: {
                    let mut params = HashMap::new();
                    params.insert("min_balance".to_string(), "1000".to_string());
                    params.insert("verification_key_id".to_string(), "balance_verification_key".to_string());
                    params
                },
            },
            ZkPolicyTemplate {
                name: "Membership Verification Policy".to_string(),
                description: "Verify group membership without revealing identity".to_string(),
                required_claim_types: vec![ZkClaimType::MembershipVerification],
                base_config: PolicyConfig {
                    is_pre_hook: true,
                    max_gas: 200_000,
                    priority: 130,
                    ..Default::default()
                },
                zk_parameters: {
                    let mut params = HashMap::new();
                    params.insert("group_id".to_string(), "authorized_users".to_string());
                    params.insert("verification_key_id".to_string(), "membership_verification_key".to_string());
                    params
                },
            },
        ]
    }
}

impl Default for ZkPolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ZkCourt {
    /// Create a new ZK-enabled court
    pub fn new(name: String, description: String, config: crate::court::CourtConfig) -> Self {
        Self {
            base_court: Court::new(name, description, config),
            zk_engine: ZkPolicyEngine::new(),
            stats: Arc::new(RwLock::new(ZkCourtStats::default())),
        }
    }

    /// Add verification key to the court's ZK engine
    pub fn add_verification_key(&self, key_id: String, key_data: Vec<u8>) {
        self.zk_engine.add_verification_key(key_id, key_data);
    }

    /// Enforce ZK-enabled agreement
    pub fn enforce_zk_agreement(
        &self,
        agreement_id: Uuid,
        zk_context: &mut ZkPolicyContext,
    ) -> DockLockResult<crate::court::AgreementEnforcementResult> {
        debug!("Enforcing ZK-enabled agreement: {}", agreement_id);

        // Get the agreement from base court
        let agreement = self.base_court.get_agreement(agreement_id)
            .ok_or_else(|| DockLockError::PolicyError("Agreement not found".to_string()))?;

        // Check if agreement is expired
        if agreement.is_expired() {
            return Ok(crate::court::AgreementEnforcementResult {
                agreement_id,
                enforced: false,
                policy_results: Vec::new(),
                message: "Agreement has expired".to_string(),
            });
        }

        // Execute ZK-enabled policies
        let zk_pre_result = self.zk_engine.execute_zk_pre_hooks(zk_context)?;
        let zk_post_result = self.zk_engine.execute_zk_post_hooks(zk_context)?;

        let all_enforced = zk_pre_result.base_result.allowed && 
                          zk_post_result.base_result.allowed &&
                          zk_pre_result.zk_verified && 
                          zk_post_result.zk_verified;

        // Convert ZK policy results to base policy results for compatibility
        let policy_results = vec![zk_pre_result.base_result, zk_post_result.base_result];

        // Update statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_zk_agreements += 1;
            stats.total_zk_policies += 2; // pre + post hooks
            stats.total_zk_claims += zk_context.zk_claims.len() as u64;
        }

        let message = if all_enforced {
            "ZK-enabled agreement enforced successfully".to_string()
        } else {
            "ZK-enabled agreement enforcement failed".to_string()
        };

        info!("ZK agreement enforcement completed: {} - enforced: {}", agreement_id, all_enforced);

        Ok(crate::court::AgreementEnforcementResult {
            agreement_id,
            enforced: all_enforced,
            policy_results,
            message,
        })
    }

    /// Get ZK court statistics
    pub fn get_zk_stats(&self) -> ZkCourtStats {
        self.stats.read().unwrap().clone()
    }

    /// Get base court reference
    pub fn get_base_court(&self) -> &Court {
        &self.base_court
    }

    /// Get ZK policy engine reference
    pub fn get_zk_engine(&self) -> &ZkPolicyEngine {
        &self.zk_engine
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy_engine::SystemState;
    use crate::zk_proofs::{ZkProofType, ZkProof};

    #[test]
    fn test_zk_policy_context_creation() {
        let base_context = PolicyContext {
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

        let zk_context = ZkPolicyContext::from_base_context(base_context);
        assert!(!zk_context.requires_zk_verification());
        assert!(!zk_context.zk_enabled);
    }

    #[test]
    fn test_zk_policy_context_with_claims() {
        let base_context = PolicyContext {
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

        // Create a ZK proof
        let mut metadata = HashMap::new();
        metadata.insert("test".to_string(), "value".to_string());

        let proof = ZkProof::new(
            ZkProofType::RangeProof,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32],
            vec![5, 6, 7, 8],
            "test_key".to_string(),
            metadata,
        ).unwrap();

        // Create a ZK claim
        let mut parameters = HashMap::new();
        parameters.insert("min_age".to_string(), "18".to_string());

        let claim = ZkClaim::new(
            "User is over 18 years old".to_string(),
            ZkClaimType::AgeVerification,
            proof,
            parameters,
        ).unwrap();

        let zk_context = ZkPolicyContext::new(base_context, vec![claim]);
        assert!(zk_context.requires_zk_verification());
        assert!(zk_context.zk_enabled);
        assert_eq!(zk_context.zk_claims.len(), 1);
    }

    #[test]
    fn test_zk_policy_engine() {
        let engine = ZkPolicyEngine::new();
        engine.add_verification_key("test_key".to_string(), vec![1, 2, 3, 4]);

        let stats = engine.get_stats();
        assert_eq!(stats.total_zk_executions, 0);
    }

    #[test]
    fn test_zk_policy_templates() {
        let engine = ZkPolicyEngine::new();
        let templates = engine.get_zk_policy_templates();
        
        assert!(!templates.is_empty());
        
        let age_template = templates.iter()
            .find(|t| t.name == "Age Verification Policy")
            .expect("Age verification template should exist");
        
        assert_eq!(age_template.required_claim_types, vec![ZkClaimType::AgeVerification]);
        assert!(age_template.zk_parameters.contains_key("min_age"));
    }

    #[test]
    fn test_zk_court_creation() {
        let config = crate::court::CourtConfig::default();
        let zk_court = ZkCourt::new(
            "ZK Test Court".to_string(),
            "A test court with ZK capabilities".to_string(),
            config,
        );

        let stats = zk_court.get_zk_stats();
        assert_eq!(stats.total_zk_agreements, 0);
        assert_eq!(stats.total_zk_policies, 0);
    }

    #[test]
    fn test_zk_policy_execution() {
        let engine = ZkPolicyEngine::new();
        engine.add_verification_key("test_key".to_string(), vec![1, 2, 3, 4]);

        let base_context = PolicyContext {
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

        let mut zk_context = ZkPolicyContext::from_base_context(base_context);
        let result = engine.execute_zk_pre_hooks(&mut zk_context).unwrap();
        
        assert!(result.base_result.allowed);
        assert!(result.zk_verified); // No ZK claims, so verification passes
    }
}
