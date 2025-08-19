use crate::error::{DockLockError, DockLockResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Domain separation constants for policy hashing
const POLICY_HASH: u8 = 0x05;
const AGREEMENT_HASH: u8 = 0x06;

/// Policy execution context for WASM runtime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyContext {
    /// Execution ID for correlation
    pub execution_id: Uuid,
    /// Current timestamp
    pub timestamp: u64,
    /// Execution metadata
    pub metadata: HashMap<String, String>,
    /// Read-only access to system state
    pub system_state: SystemState,
}

/// Read-only system state accessible to policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    /// Current block height
    pub block_height: u64,
    /// Current block hash
    pub block_hash: [u8; 32],
    /// Available memory
    pub available_memory: u64,
    /// CPU usage statistics
    pub cpu_usage: f64,
}

/// Policy execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyResult {
    /// Whether the policy allows the operation
    pub allowed: bool,
    /// Policy decision reason
    pub reason: String,
    /// Additional metadata from policy execution
    pub metadata: HashMap<String, String>,
    /// Gas consumed during policy execution
    pub gas_consumed: u64,
}

/// Policy definition with WASM bytecode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    /// Unique policy identifier
    pub id: Uuid,
    /// Policy name
    pub name: String,
    /// Policy version
    pub version: String,
    /// WASM bytecode for policy execution
    pub wasm_bytecode: Vec<u8>,
    /// Policy configuration
    pub config: PolicyConfig,
    /// Policy hash for integrity verification
    pub hash: [u8; 32],
}

/// Policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    /// Maximum gas limit for policy execution
    pub max_gas: u64,
    /// Maximum execution time in milliseconds
    pub max_execution_time_ms: u64,
    /// Whether this is a pre-execution hook
    pub is_pre_hook: bool,
    /// Whether this is a post-execution hook
    pub is_post_hook: bool,
    /// Policy priority (higher = executed first)
    pub priority: u32,
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            max_gas: 1_000_000,
            max_execution_time_ms: 1000,
            is_pre_hook: false,
            is_post_hook: false,
            priority: 100,
        }
    }
}

/// Agreement between parties with policy enforcement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agreement {
    /// Unique agreement identifier
    pub id: Uuid,
    /// Agreement name
    pub name: String,
    /// Agreement version
    pub version: String,
    /// Parties involved in the agreement
    pub parties: Vec<String>,
    /// Policies that enforce this agreement
    pub policy_ids: Vec<Uuid>,
    /// Agreement terms and conditions
    pub terms: String,
    /// Agreement hash for integrity verification
    pub hash: [u8; 32],
    /// Creation timestamp
    pub created_at: u64,
    /// Expiration timestamp (0 = never expires)
    pub expires_at: u64,
}

/// Policy execution engine with WASM runtime
#[derive(Debug)]
pub struct PolicyEngine {
    /// Loaded policies
    policies: Arc<RwLock<HashMap<Uuid, Policy>>>,
    /// Policy execution statistics
    stats: Arc<RwLock<PolicyEngineStats>>,
}

/// Policy engine execution statistics
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PolicyEngineStats {
    /// Total policy executions
    pub total_executions: u64,
    /// Total gas consumed
    pub total_gas_consumed: u64,
    /// Total execution time in milliseconds
    pub total_execution_time_ms: u64,
    /// Number of policy violations
    pub policy_violations: u64,
    /// Number of sandbox escape attempts
    pub sandbox_escape_attempts: u64,
}

impl PolicyEngine {
    /// Create a new policy engine
    pub fn new() -> Self {
        Self {
            policies: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(PolicyEngineStats::default())),
        }
    }

    /// Load a policy into the engine
    pub fn load_policy(&self, policy: Policy) -> DockLockResult<()> {
        debug!("Loading policy: {} ({})", policy.name, policy.id);
        
        // Verify policy hash
        let computed_hash = self.compute_policy_hash(&policy)?;
        if computed_hash != policy.hash {
            return Err(DockLockError::PolicyError(
                "Policy hash verification failed".to_string()
            ));
        }

        // Validate WASM bytecode (basic validation)
        if policy.wasm_bytecode.is_empty() {
            return Err(DockLockError::PolicyError(
                "Policy WASM bytecode is empty".to_string()
            ));
        }

        let mut policies = self.policies.write().unwrap();
        policies.insert(policy.id, policy);
        
        info!("Policy loaded successfully");
        Ok(())
    }

    /// Execute pre-execution policies
    pub fn execute_pre_hooks(&self, context: &PolicyContext) -> DockLockResult<Vec<PolicyResult>> {
        self.execute_policies(context, true, false)
    }

    /// Execute post-execution policies
    pub fn execute_post_hooks(&self, context: &PolicyContext) -> DockLockResult<Vec<PolicyResult>> {
        self.execute_policies(context, false, true)
    }

    /// Execute policies based on hook type
    fn execute_policies(
        &self, 
        context: &PolicyContext, 
        pre_hook: bool, 
        post_hook: bool
    ) -> DockLockResult<Vec<PolicyResult>> {
        let policies = self.policies.read().unwrap();
        let mut results = Vec::new();
        let mut policy_list: Vec<_> = policies.values().collect();
        
        // Filter policies by hook type
        policy_list.retain(|p| {
            (pre_hook && p.config.is_pre_hook) || (post_hook && p.config.is_post_hook)
        });
        
        // Sort by priority (higher priority first)
        policy_list.sort_by(|a, b| b.config.priority.cmp(&a.config.priority));

        for policy in policy_list {
            debug!("Executing policy: {} ({})", policy.name, policy.id);
            
            let start_time = std::time::Instant::now();
            let result = self.execute_policy(policy, context)?;
            let execution_time = start_time.elapsed().as_millis() as u64;

            // Update statistics
            {
                let mut stats = self.stats.write().unwrap();
                stats.total_executions += 1;
                stats.total_gas_consumed += result.gas_consumed;
                stats.total_execution_time_ms += execution_time;
                
                if !result.allowed {
                    stats.policy_violations += 1;
                }
            }

            results.push(result);
        }

        Ok(results)
    }

    /// Execute a single policy (placeholder for WASM execution)
    fn execute_policy(&self, policy: &Policy, context: &PolicyContext) -> DockLockResult<PolicyResult> {
        // This is a placeholder implementation
        // In a real implementation, this would:
        // 1. Initialize WASM runtime (e.g., wasmtime, wasmer)
        // 2. Load the policy WASM bytecode
        // 3. Set up sandboxed execution environment
        // 4. Provide read-only host APIs to the WASM module
        // 5. Execute the policy with the given context
        // 6. Monitor for sandbox escape attempts
        // 7. Enforce gas and time limits
        
        debug!("Executing policy WASM bytecode (placeholder)");
        
        // Simulate policy execution
        let allowed = self.simulate_policy_execution(policy, context)?;
        
        Ok(PolicyResult {
            allowed,
            reason: format!("Policy {} executed", policy.name),
            metadata: HashMap::new(),
            gas_consumed: 1000, // Placeholder gas consumption
        })
    }

    /// Simulate policy execution (placeholder)
    fn simulate_policy_execution(&self, _policy: &Policy, context: &PolicyContext) -> DockLockResult<bool> {
        // Placeholder logic - in reality this would execute WASM
        
        // Check system constraints
        if context.system_state.available_memory < 1024 * 1024 {
            warn!("Low memory detected, policy may deny execution");
            return Ok(false);
        }
        
        if context.system_state.cpu_usage > 90.0 {
            warn!("High CPU usage detected, policy may deny execution");
            return Ok(false);
        }
        
        // Default allow for simulation
        Ok(true)
    }

    /// Compute policy hash for integrity verification
    fn compute_policy_hash(&self, policy: &Policy) -> DockLockResult<[u8; 32]> {
        let policy_data = serde_cbor::to_vec(&(
            &policy.name,
            &policy.version,
            &policy.wasm_bytecode,
            &policy.config,
        )).map_err(|e| DockLockError::EncodingError(format!("Failed to encode policy: {}", e)))?;

        // Apply domain separation
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[POLICY_HASH]);
        hasher.update(&policy_data);
        Ok(hasher.finalize().into())
    }

    /// Get policy engine statistics
    pub fn get_stats(&self) -> PolicyEngineStats {
        self.stats.read().unwrap().clone()
    }

    /// Remove a policy from the engine
    pub fn remove_policy(&self, policy_id: Uuid) -> DockLockResult<()> {
        let mut policies = self.policies.write().unwrap();
        if policies.remove(&policy_id).is_some() {
            info!("Policy removed: {}", policy_id);
            Ok(())
        } else {
            Err(DockLockError::PolicyError(
                format!("Policy not found: {}", policy_id)
            ))
        }
    }

    /// List all loaded policies
    pub fn list_policies(&self) -> Vec<Uuid> {
        self.policies.read().unwrap().keys().cloned().collect()
    }
}

impl Default for PolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Policy {
    /// Create a new policy
    pub fn new(
        name: String,
        version: String,
        wasm_bytecode: Vec<u8>,
        config: PolicyConfig,
    ) -> DockLockResult<Self> {
        let id = Uuid::new_v4();
        let mut policy = Self {
            id,
            name,
            version,
            wasm_bytecode,
            config,
            hash: [0u8; 32], // Temporary
        };
        
        // Compute hash
        policy.hash = policy.compute_hash()?;
        Ok(policy)
    }

    /// Compute policy hash
    fn compute_hash(&self) -> DockLockResult<[u8; 32]> {
        let policy_data = serde_cbor::to_vec(&(
            &self.name,
            &self.version,
            &self.wasm_bytecode,
            &self.config,
        )).map_err(|e| DockLockError::EncodingError(format!("Failed to encode policy: {}", e)))?;

        // Apply domain separation
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[POLICY_HASH]);
        hasher.update(&policy_data);
        Ok(hasher.finalize().into())
    }
}

impl Agreement {
    /// Create a new agreement
    pub fn new(
        name: String,
        version: String,
        parties: Vec<String>,
        policy_ids: Vec<Uuid>,
        terms: String,
        expires_at: u64,
    ) -> DockLockResult<Self> {
        let id = Uuid::new_v4();
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        let mut agreement = Self {
            id,
            name,
            version,
            parties,
            policy_ids,
            terms,
            hash: [0u8; 32], // Temporary
            created_at,
            expires_at,
        };
        
        // Compute hash
        agreement.hash = agreement.compute_hash()?;
        Ok(agreement)
    }

    /// Compute agreement hash
    fn compute_hash(&self) -> DockLockResult<[u8; 32]> {
        let agreement_data = serde_cbor::to_vec(&(
            &self.name,
            &self.version,
            &self.parties,
            &self.policy_ids,
            &self.terms,
            self.created_at,
            self.expires_at,
        )).map_err(|e| DockLockError::EncodingError(format!("Failed to encode agreement: {}", e)))?;

        // Apply domain separation
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[AGREEMENT_HASH]);
        hasher.update(&agreement_data);
        Ok(hasher.finalize().into())
    }

    /// Check if agreement is expired
    pub fn is_expired(&self) -> bool {
        if self.expires_at == 0 {
            return false; // Never expires
        }
        
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        current_time > self.expires_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_engine_creation() {
        let engine = PolicyEngine::new();
        assert_eq!(engine.list_policies().len(), 0);
    }

    #[test]
    fn test_policy_creation() {
        let config = PolicyConfig::default();
        let policy = Policy::new(
            "test_policy".to_string(),
            "1.0.0".to_string(),
            vec![0x00, 0x61, 0x73, 0x6d], // WASM magic bytes
            config,
        ).unwrap();
        
        assert_eq!(policy.name, "test_policy");
        assert_eq!(policy.version, "1.0.0");
        assert!(!policy.wasm_bytecode.is_empty());
    }

    #[test]
    fn test_policy_loading() {
        let engine = PolicyEngine::new();
        let config = PolicyConfig::default();
        let policy = Policy::new(
            "test_policy".to_string(),
            "1.0.0".to_string(),
            vec![0x00, 0x61, 0x73, 0x6d],
            config,
        ).unwrap();
        
        let policy_id = policy.id;
        engine.load_policy(policy).unwrap();
        
        let policies = engine.list_policies();
        assert_eq!(policies.len(), 1);
        assert!(policies.contains(&policy_id));
    }

    #[test]
    fn test_agreement_creation() {
        let agreement = Agreement::new(
            "test_agreement".to_string(),
            "1.0.0".to_string(),
            vec!["party1".to_string(), "party2".to_string()],
            vec![Uuid::new_v4()],
            "Terms and conditions".to_string(),
            0, // Never expires
        ).unwrap();
        
        assert_eq!(agreement.name, "test_agreement");
        assert_eq!(agreement.parties.len(), 2);
        assert!(!agreement.is_expired());
    }

    #[test]
    fn test_policy_execution() {
        let engine = PolicyEngine::new();
        let config = PolicyConfig {
            is_pre_hook: true,
            ..Default::default()
        };
        let policy = Policy::new(
            "pre_hook_policy".to_string(),
            "1.0.0".to_string(),
            vec![0x00, 0x61, 0x73, 0x6d],
            config,
        ).unwrap();
        
        engine.load_policy(policy).unwrap();
        
        let context = PolicyContext {
            execution_id: Uuid::new_v4(),
            timestamp: 1234567890,
            metadata: HashMap::new(),
            system_state: SystemState {
                block_height: 100,
                block_hash: [0u8; 32],
                available_memory: 1024 * 1024 * 1024, // 1GB
                cpu_usage: 50.0,
            },
        };
        
        let results = engine.execute_pre_hooks(&context).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].allowed);
    }

    #[test]
    fn test_policy_engine_stats() {
        let engine = PolicyEngine::new();
        let stats = engine.get_stats();
        assert_eq!(stats.total_executions, 0);
        assert_eq!(stats.total_gas_consumed, 0);
    }
}
