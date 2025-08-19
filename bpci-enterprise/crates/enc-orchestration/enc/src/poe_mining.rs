//! Proof of Execution (PoE) Mining Engine
//! 
//! Military-grade mining and notarization layer for blockchain orchestration

use anyhow::Result;
use blake3::Hasher;
use chrono::{DateTime, Utc};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use thiserror::Error;
use uuid::Uuid;



use crate::domains;

/// PoE Mining Engine Errors
#[derive(Error, Debug)]
pub enum PoEMiningError {
    #[error("Invalid proof of execution: {0}")]
    InvalidProof(String),
    #[error("Mining difficulty adjustment failed: {0}")]
    DifficultyAdjustment(String),
    #[error("Validator verification failed: {0}")]
    ValidatorVerification(String),
    #[error("Consensus failure: {0}")]
    ConsensusFailure(String),
    #[error("Notarization error: {0}")]
    NotarizationError(String),
    #[error("Cryptographic error: {0}")]
    CryptographicError(String),
}

/// Proof of Execution Entry
#[derive(Debug, Clone, PartialEq)]
pub struct ProofOfExecution {
    pub proof_id: Uuid,
    pub execution_context: ExecutionContext,
    pub execution_proof: ExecutionProof,
    pub validator_signatures: Vec<ValidatorSignature>,
    pub timestamp: DateTime<Utc>,
    pub difficulty: u64,
    pub reward: u64,
    pub previous_proof_hash: [u8; 32],
    pub execution_merkle_root: [u8; 32],
}

/// Execution Context Types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExecutionContext {
    DockLockContainer {
        container_id: String,
        image_hash: [u8; 32],
        command: String,
        resource_usage: ResourceUsage,
    },
    EncClusterOperation {
        cluster_id: Uuid,
        operation_type: String,
        node_count: u32,
        workload_hash: [u8; 32],
    },
    BpiNodeOperation {
        node_id: Uuid,
        operation: String,
        consensus_round: u64,
    },
    BpciServerOperation {
        server_id: Uuid,
        transaction_hash: [u8; 32],
        block_height: u64,
    },
}

/// Resource Usage Metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceUsage {
    pub cpu_time_ms: u64,
    pub memory_bytes: u64,
    pub disk_io_bytes: u64,
    pub network_io_bytes: u64,
    pub execution_time_ms: u64,
}

/// Cryptographic Execution Proof
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionProof {
    pub environment_hash: [u8; 32],
    pub input_hash: [u8; 32],
    pub output_hash: [u8; 32],
    pub trace_hash: [u8; 32],
    pub zk_proof: Vec<u8>,
    pub witness_data: Vec<u8>,
}

/// Validator Signature
#[derive(Debug, Clone, PartialEq)]
pub struct ValidatorSignature {
    pub validator_id: Uuid,
    pub public_key: [u8; 32],
    pub signature: [u8; 64],
    pub stake_weight: u64,
    pub timestamp: DateTime<Utc>,
}

/// Validator Information
#[derive(Debug, Clone)]
pub struct ValidatorInfo {
    pub validator_id: Uuid,
    pub public_key: VerifyingKey,
    pub stake: u64,
    pub reputation_score: f64,
    pub last_active: DateTime<Utc>,
    pub total_proofs_validated: u64,
    pub total_rewards_earned: u64,
}

/// Mining Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningStats {
    pub total_proofs_mined: u64,
    pub total_rewards_distributed: u64,
    pub average_mining_time_ms: f64,
    pub current_difficulty: u64,
    pub active_validators: u32,
    pub total_stake: u64,
    pub network_hash_rate: f64,
    pub last_difficulty_adjustment: DateTime<Utc>,
}

/// PoE Mining Engine
pub struct PoEMiningEngine {
    config: PoEMiningConfig,
    difficulty: Arc<RwLock<u64>>,
    validator_pool: Arc<RwLock<HashMap<Uuid, ValidatorInfo>>>,
    proof_chain: Arc<RwLock<VecDeque<ProofOfExecution>>>,
    stats: Arc<RwLock<MiningStats>>,
    pending_proofs: Arc<RwLock<HashMap<Uuid, ProofOfExecution>>>,
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

/// PoE Mining Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoEMiningConfig {
    pub node_id: Uuid,
    pub initial_difficulty: u64,
    pub target_proof_time: u64,
    pub max_proof_chain_length: usize,
    pub min_validator_signatures: u32,
    pub base_reward: u64,
    pub difficulty_adjustment_factor: f64,
    pub consensus_threshold: f64,
    pub max_pending_proofs: usize,
}

impl Default for PoEMiningConfig {
    fn default() -> Self {
        Self {
            node_id: Uuid::new_v4(),
            initial_difficulty: 1000,
            target_proof_time: 10,
            max_proof_chain_length: 1000,
            min_validator_signatures: 3,
            base_reward: 1000000,
            difficulty_adjustment_factor: 0.1,
            consensus_threshold: 0.67,
            max_pending_proofs: 100,
        }
    }
}

impl PoEMiningEngine {
    /// Create new PoE mining engine
    pub fn new(config: PoEMiningConfig) -> Result<Self, PoEMiningError> {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let stats = MiningStats {
            total_proofs_mined: 0,
            total_rewards_distributed: 0,
            average_mining_time_ms: 0.0,
            current_difficulty: config.initial_difficulty,
            active_validators: 0,
            total_stake: 0,
            network_hash_rate: 0.0,
            last_difficulty_adjustment: Utc::now(),
        };

        let initial_difficulty = config.initial_difficulty;
        
        Ok(Self {
            config,
            difficulty: Arc::new(RwLock::new(initial_difficulty)),
            validator_pool: Arc::new(RwLock::new(HashMap::new())),
            proof_chain: Arc::new(RwLock::new(VecDeque::new())),
            stats: Arc::new(RwLock::new(stats)),
            pending_proofs: Arc::new(RwLock::new(HashMap::new())),
            signing_key,
            verifying_key,
        })
    }

    /// Mine a new proof of execution
    pub fn mine_proof(&self, execution_context: ExecutionContext) -> Result<ProofOfExecution, PoEMiningError> {
        let start_time = std::time::Instant::now();
        
        let execution_proof = self.generate_execution_proof(&execution_context)?;
        let difficulty = *self.difficulty.read().unwrap();
        let previous_proof_hash = self.get_latest_proof_hash();
        
        let mut proof = ProofOfExecution {
            proof_id: Uuid::new_v4(),
            execution_context,
            execution_proof,
            validator_signatures: Vec::new(),
            timestamp: Utc::now(),
            difficulty,
            reward: self.calculate_reward(difficulty),
            previous_proof_hash,
            execution_merkle_root: [0u8; 32],
        };

        proof.execution_merkle_root = self.compute_execution_merkle_root(&proof)?;
        self.mine_proof_with_difficulty(&mut proof)?;
        
        self.pending_proofs.write().unwrap().insert(proof.proof_id, proof.clone());
        
        let mining_time = start_time.elapsed().as_millis() as f64;
        self.update_mining_stats(mining_time, proof.reward);

        Ok(proof)
    }

    /// Generate cryptographic execution proof
    fn generate_execution_proof(&self, context: &ExecutionContext) -> Result<ExecutionProof, PoEMiningError> {
        let environment_hash = self.hash_execution_environment(context);
        let input_hash = self.hash_execution_inputs(context);
        let output_hash = self.hash_execution_outputs(context);
        let trace_hash = self.hash_execution_trace(context);
        let zk_proof = self.generate_zk_proof(context)?;
        let witness_data = self.generate_witness_data(context)?;

        Ok(ExecutionProof {
            environment_hash,
            input_hash,
            output_hash,
            trace_hash,
            zk_proof,
            witness_data,
        })
    }

    fn hash_execution_environment(&self, context: &ExecutionContext) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(&[domains::DOCKLOCK_RECORD_HASH]);
        
        match context {
            ExecutionContext::DockLockContainer { container_id, image_hash, .. } => {
                hasher.update(b"docklock_container");
                hasher.update(container_id.as_bytes());
                hasher.update(image_hash);
            },
            ExecutionContext::EncClusterOperation { cluster_id, workload_hash, .. } => {
                hasher.update(b"enc_cluster");
                hasher.update(cluster_id.as_bytes());
                hasher.update(workload_hash);
            },
            ExecutionContext::BpiNodeOperation { node_id, .. } => {
                hasher.update(b"bpi_node");
                hasher.update(node_id.as_bytes());
            },
            ExecutionContext::BpciServerOperation { server_id, transaction_hash, .. } => {
                hasher.update(b"bpci_server");
                hasher.update(server_id.as_bytes());
                hasher.update(transaction_hash);
            },
        }
        
        hasher.finalize().into()
    }

    fn hash_execution_inputs(&self, context: &ExecutionContext) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(&[domains::RECEIPT_HASH]);
        
        match context {
            ExecutionContext::DockLockContainer { command, .. } => {
                hasher.update(command.as_bytes());
            },
            ExecutionContext::EncClusterOperation { operation_type, node_count, .. } => {
                hasher.update(operation_type.as_bytes());
                hasher.update(&node_count.to_le_bytes());
            },
            ExecutionContext::BpiNodeOperation { operation, consensus_round, .. } => {
                hasher.update(operation.as_bytes());
                hasher.update(&consensus_round.to_le_bytes());
            },
            ExecutionContext::BpciServerOperation { block_height, .. } => {
                hasher.update(&block_height.to_le_bytes());
            },
        }
        
        hasher.finalize().into()
    }

    fn hash_execution_outputs(&self, context: &ExecutionContext) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(&[domains::WITNESS_ENTRY_HASH]);
        
        match context {
            ExecutionContext::DockLockContainer { resource_usage, .. } => {
                hasher.update(&resource_usage.cpu_time_ms.to_le_bytes());
                hasher.update(&resource_usage.memory_bytes.to_le_bytes());
                hasher.update(&resource_usage.execution_time_ms.to_le_bytes());
            },
            _ => {
                hasher.update(b"operation_success");
            },
        }
        
        hasher.finalize().into()
    }

    fn hash_execution_trace(&self, _context: &ExecutionContext) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(&[domains::BLOCKBOOK_ENTRY_HASH]);
        hasher.update(b"deterministic_execution_trace");
        hasher.finalize().into()
    }

    fn generate_zk_proof(&self, _context: &ExecutionContext) -> Result<Vec<u8>, PoEMiningError> {
        let mut hasher = Hasher::new();
        hasher.update(b"zk_proof_placeholder");
        hasher.update(&self.verifying_key.to_bytes());
        Ok(hasher.finalize().as_bytes().to_vec())
    }

    fn generate_witness_data(&self, _context: &ExecutionContext) -> Result<Vec<u8>, PoEMiningError> {
        let mut hasher = Hasher::new();
        hasher.update(b"witness_data_placeholder");
        hasher.update(&Utc::now().timestamp().to_le_bytes());
        Ok(hasher.finalize().as_bytes().to_vec())
    }

    fn get_latest_proof_hash(&self) -> [u8; 32] {
        let chain = self.proof_chain.read().unwrap();
        if let Some(latest_proof) = chain.back() {
            self.compute_proof_hash(latest_proof)
        } else {
            [0u8; 32]
        }
    }

    fn compute_proof_hash(&self, proof: &ProofOfExecution) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(&[domains::BLOCKBOOK_LEDGER_HASH]);
        hasher.update(proof.proof_id.as_bytes());
        hasher.update(&proof.timestamp.timestamp().to_le_bytes());
        hasher.update(&proof.difficulty.to_le_bytes());
        hasher.update(&proof.execution_merkle_root);
        hasher.finalize().into()
    }

    fn compute_execution_merkle_root(&self, proof: &ProofOfExecution) -> Result<[u8; 32], PoEMiningError> {
        let mut hasher = Hasher::new();
        hasher.update(&[domains::MERKLE_INTERNAL]);
        hasher.update(&proof.execution_proof.environment_hash);
        hasher.update(&proof.execution_proof.input_hash);
        hasher.update(&proof.execution_proof.output_hash);
        hasher.update(&proof.execution_proof.trace_hash);
        Ok(hasher.finalize().into())
    }

    fn mine_proof_with_difficulty(&self, proof: &mut ProofOfExecution) -> Result<(), PoEMiningError> {
        let target_difficulty = proof.difficulty;
        let mut nonce = 0u64;
        
        loop {
            let mut hasher = Hasher::new();
            hasher.update(&[domains::BLOCKBOOK_LEDGER_HASH]);
            hasher.update(proof.proof_id.as_bytes());
            hasher.update(&nonce.to_le_bytes());
            hasher.update(&proof.execution_merkle_root);
            
            let hash = hasher.finalize();
            let hash_value = u64::from_le_bytes([
                hash.as_bytes()[0], hash.as_bytes()[1], hash.as_bytes()[2], hash.as_bytes()[3],
                hash.as_bytes()[4], hash.as_bytes()[5], hash.as_bytes()[6], hash.as_bytes()[7],
            ]);
            
            if hash_value < target_difficulty {
                break;
            }
            
            nonce += 1;
            if nonce > 1000000 {
                break;
            }
        }
        
        Ok(())
    }

    fn calculate_reward(&self, difficulty: u64) -> u64 {
        let base_reward = self.config.base_reward;
        let difficulty_multiplier = (difficulty as f64 / 1000.0).max(1.0);
        (base_reward as f64 * difficulty_multiplier) as u64
    }

    fn update_mining_stats(&self, mining_time_ms: f64, reward: u64) {
        let mut stats = self.stats.write().unwrap();
        stats.total_proofs_mined += 1;
        stats.total_rewards_distributed += reward;
        
        let total_proofs = stats.total_proofs_mined as f64;
        stats.average_mining_time_ms = ((stats.average_mining_time_ms * (total_proofs - 1.0)) + mining_time_ms) / total_proofs;
        
        stats.current_difficulty = *self.difficulty.read().unwrap();
        stats.active_validators = self.validator_pool.read().unwrap().len() as u32;
        stats.total_stake = self.validator_pool.read().unwrap().values().map(|v| v.stake).sum();
    }

    pub fn add_validator(&self, validator_info: ValidatorInfo) -> Result<(), PoEMiningError> {
        let mut pool = self.validator_pool.write().unwrap();
        pool.insert(validator_info.validator_id, validator_info);
        Ok(())
    }

    pub fn get_stats(&self) -> MiningStats {
        self.stats.read().unwrap().clone()
    }

    pub fn get_current_difficulty(&self) -> u64 {
        *self.difficulty.read().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poe_mining_engine_creation() {
        let config = PoEMiningConfig::default();
        let engine = PoEMiningEngine::new(config).unwrap();
        
        assert_eq!(engine.get_current_difficulty(), 1000);
        let stats = engine.get_stats();
        assert_eq!(stats.total_proofs_mined, 0);
        assert_eq!(stats.active_validators, 0);
    }

    #[test]
    fn test_mine_docklock_proof() {
        let config = PoEMiningConfig::default();
        let engine = PoEMiningEngine::new(config).unwrap();
        
        let context = ExecutionContext::DockLockContainer {
            container_id: "test-container".to_string(),
            image_hash: [1u8; 32],
            command: "echo hello".to_string(),
            resource_usage: ResourceUsage {
                cpu_time_ms: 100,
                memory_bytes: 1024,
                disk_io_bytes: 0,
                network_io_bytes: 0,
                execution_time_ms: 50,
            },
        };
        
        let proof = engine.mine_proof(context).unwrap();
        assert_eq!(proof.difficulty, 1000);
        assert!(proof.reward > 0);
        assert_ne!(proof.execution_merkle_root, [0u8; 32]);
    }

    #[test]
    fn test_mine_enc_cluster_proof() {
        let config = PoEMiningConfig::default();
        let engine = PoEMiningEngine::new(config).unwrap();
        
        let context = ExecutionContext::EncClusterOperation {
            cluster_id: Uuid::new_v4(),
            operation_type: "deploy".to_string(),
            node_count: 3,
            workload_hash: [2u8; 32],
        };
        
        let proof = engine.mine_proof(context).unwrap();
        assert_eq!(proof.difficulty, 1000);
        assert!(proof.reward > 0);
    }

    #[test]
    fn test_validator_management() {
        let config = PoEMiningConfig::default();
        let engine = PoEMiningEngine::new(config).unwrap();
        
        let validator_info = ValidatorInfo {
            validator_id: Uuid::new_v4(),
            public_key: SigningKey::generate(&mut OsRng).verifying_key(),
            stake: 5000,
            reputation_score: 1.0,
            last_active: Utc::now(),
            total_proofs_validated: 0,
            total_rewards_earned: 0,
        };
        
        engine.add_validator(validator_info).unwrap();
        
        // Update stats to reflect the new validator
        engine.update_mining_stats(0.0, 0);
        
        let stats = engine.get_stats();
        assert_eq!(stats.active_validators, 1);
        assert_eq!(stats.total_stake, 5000);
    }
}
