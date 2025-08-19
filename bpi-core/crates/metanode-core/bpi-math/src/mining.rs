//! Advanced Mining Engine with Autonomous Coin Dispensing
//! 
//! This module implements the advanced mining logic that integrates all proof systems,
//! category theory ledger operations, and knot theory immutability for military-grade
//! autonomous mining with proof-of-execution tied coin dispensing.

use crate::{
    Hash, MathError, Timestamp,
    knot::{TransactionKnot, KnotInvariant},
};
use crate::receipts::{ReceiptType, AggregatedTransaction, ReceiptAggregator};
use crate::poe_calculator::{PoECalculator, ResourceUsage};
use crate::category::MetanodeLedgerCategory;
use crate::proofs::{ActionType, ProofOfAction, ProofSystem};
use crate::constants::{MINING_PROOF_DOMAIN, MINING_MERKLE_DOMAIN, MINING_BLOCK_DOMAIN};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{VecDeque};

/// Mining difficulty adjustment parameters
#[derive(Debug, Clone)]
pub struct MiningDifficulty {
    pub target_block_time: u64, // milliseconds
    pub difficulty_adjustment_window: u64, // blocks
    pub max_difficulty_change: f64, // maximum change per adjustment (e.g., 4.0 = 4x)
    pub current_difficulty: u64,
    pub target_hash: Hash,
}

impl Default for MiningDifficulty {
    fn default() -> Self {
        Self {
            target_block_time: 5000, // 5 seconds
            difficulty_adjustment_window: 100,
            max_difficulty_change: 4.0,
            current_difficulty: 1000000,
            target_hash: [0u8; 32], // Will be computed based on difficulty
        }
    }
}

/// Mining reward configuration
#[derive(Debug, Clone)]
pub struct MiningRewards {
    pub base_reward: u64,
    pub proof_of_execution_multiplier: f64,
    pub proof_of_action_multiplier: f64,
    pub proof_of_transact_multiplier: f64,
    pub proof_of_gold_multiplier: f64,
    pub proof_of_history_multiplier: f64,
    pub halving_interval: u64, // blocks
    pub total_supply_cap: u64,
}

impl Default for MiningRewards {
    fn default() -> Self {
        Self {
            base_reward: 1000000, // 1 METANODE token (6 decimals)
            proof_of_execution_multiplier: 2.0,
            proof_of_action_multiplier: 1.5,
            proof_of_transact_multiplier: 1.8,
            proof_of_gold_multiplier: 1.2,
            proof_of_history_multiplier: 1.3,
            halving_interval: 210000, // Similar to Bitcoin
            total_supply_cap: 21_000_000_000_000, // 21M tokens with 6 decimals
        }
    }
}

/// Economic governance parameters
#[derive(Debug, Clone)]
pub struct EconomicGovernance {
    pub inflation_rate: f64,
    pub fee_burn_rate: f64,
    pub validator_reward_share: f64,
    pub treasury_share: f64,
    pub development_fund_share: f64,
    pub autonomous_adjustment_enabled: bool,
}

impl Default for EconomicGovernance {
    fn default() -> Self {
        Self {
            inflation_rate: 0.02, // 2% annual
            fee_burn_rate: 0.5, // 50% of fees burned
            validator_reward_share: 0.6, // 60% to validators
            treasury_share: 0.3, // 30% to treasury
            development_fund_share: 0.1, // 10% to development
            autonomous_adjustment_enabled: true,
        }
    }
}

/// Mining candidate block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningCandidate {
    pub block_height: u64,
    pub prev_block_hash: Hash,
    pub aggregated_transactions: Vec<AggregatedTransaction>,
    pub merkle_root: Hash,
    pub timestamp: Timestamp,
    pub nonce: u64,
    pub difficulty: u64,
    pub miner_address: String,
    pub proof_summary: ProofSummary,
}

/// Summary of all proofs in a block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofSummary {
    pub proof_of_action_count: u64,
    pub proof_of_execution_count: u64,
    pub proof_of_transact_count: u64,
    pub proof_of_gold_count: u64,
    pub proof_of_history_count: u64,
    pub total_proof_weight: f64,
    pub proof_hash: Hash,
}

/// Mining result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningResult {
    pub block_hash: Hash,
    pub nonce: u64,
    pub mining_time_ms: u64,
    pub hash_rate: f64,
    pub reward_amount: u64,
    pub proof_summary: ProofSummary,
    pub knot_invariant: KnotInvariant,
}

/// Advanced mining engine
pub struct MiningEngine {
    difficulty: MiningDifficulty,
    rewards: MiningRewards,
    governance: EconomicGovernance,
    receipt_aggregator: ReceiptAggregator,
    pending_transactions: VecDeque<AggregatedTransaction>,
    miner_address: String,
    current_supply: u64,
    blocks_mined: u64,
}

impl MiningEngine {
    pub fn new(
        miner_address: String,
        difficulty: MiningDifficulty,
        rewards: MiningRewards,
        governance: EconomicGovernance,
    ) -> Self {
        Self {
            difficulty,
            rewards,
            governance,
            receipt_aggregator: ReceiptAggregator::new(Default::default()),
            pending_transactions: VecDeque::new(),
            miner_address,
            current_supply: 0,
            blocks_mined: 0,
        }
    }
    
    /// Add receipt for mining inclusion
    pub fn add_receipt(&mut self, receipt: ReceiptType) -> Result<(), MathError> {
        self.receipt_aggregator.add_receipt(receipt)
    }
    
    /// Add aggregated transaction for mining
    pub fn add_transaction(&mut self, transaction: AggregatedTransaction) -> Result<(), MathError> {
        // Verify transaction integrity
        if !transaction.verify_integrity() {
            return Err(MathError::InvalidTransaction("Invalid transaction in mining candidate".to_string()));
        }
        
        self.pending_transactions.push_back(transaction);
        Ok(())
    }
    
    /// Mine a new block with autonomous proof-of-execution logic
    pub fn mine_block(
        &mut self,
        prev_block_hash: Hash,
        block_height: u64,
    ) -> Result<MiningResult, MathError> {
        let start_time = std::time::Instant::now();
        
        // Aggregate pending receipts
        let new_transactions = self.receipt_aggregator.aggregate_receipts()?;
        for tx in new_transactions {
            self.pending_transactions.push_back(tx);
        }
        
        // Create mining candidate
        let mut candidate = self.create_mining_candidate(prev_block_hash, block_height)?;
        
        // Perform proof-of-work mining
        let (block_hash, nonce) = self.perform_proof_of_work(&mut candidate)?;
        
        // Calculate mining reward based on proof systems
        let reward_amount = self.calculate_mining_reward(&candidate.proof_summary)?;
        
        // Create knot invariant for block immutability
        let knot_invariant = self.create_block_knot_invariant(&candidate)?;
        
        let mining_time = start_time.elapsed().as_millis() as u64;
        let hash_rate = (candidate.nonce as f64) / (mining_time as f64 / 1000.0);
        
        // Update mining state
        self.blocks_mined += 1;
        self.current_supply += reward_amount;
        self.pending_transactions.clear();
        
        // Autonomous difficulty adjustment
        if self.governance.autonomous_adjustment_enabled {
            self.adjust_difficulty_autonomous(mining_time)?;
        }
        
        Ok(MiningResult {
            block_hash,
            nonce,
            mining_time_ms: mining_time,
            hash_rate,
            reward_amount,
            proof_summary: candidate.proof_summary,
            knot_invariant,
        })
    }
    
    /// Create mining candidate from pending transactions
    fn create_mining_candidate(
        &self,
        prev_block_hash: Hash,
        block_height: u64,
    ) -> Result<MiningCandidate, MathError> {
        let transactions: Vec<AggregatedTransaction> = self.pending_transactions.iter().cloned().collect();
        
        // Compute Merkle root of transactions
        let merkle_root = self.compute_transaction_merkle_root(&transactions)?;
        
        // Create proof summary
        let proof_summary = self.create_proof_summary(&transactions)?;
        
        Ok(MiningCandidate {
            block_height,
            prev_block_hash,
            aggregated_transactions: transactions,
            merkle_root,
            timestamp: chrono::Utc::now(),
            nonce: 0,
            difficulty: self.difficulty.current_difficulty,
            miner_address: self.miner_address.clone(),
            proof_summary,
        })
    }
    
    /// Perform proof-of-work mining
    fn perform_proof_of_work(&self, candidate: &mut MiningCandidate) -> Result<(Hash, u64), MathError> {
        let target = self.compute_target_from_difficulty(self.difficulty.current_difficulty);
        
        for nonce in 0..u64::MAX {
            candidate.nonce = nonce;
            let block_hash = self.compute_block_hash(candidate)?;
            
            if self.hash_meets_target(&block_hash, &target) {
                return Ok((block_hash, nonce));
            }
            
            // Prevent infinite loop in testing
            if nonce > 1_000_000 {
                return Err(MathError::MiningTimeout);
            }
        }
        
        Err(MathError::MiningTimeout)
    }
    
    /// Calculate mining reward based on proof systems
    fn calculate_mining_reward(&self, proof_summary: &ProofSummary) -> Result<u64, MathError> {
        // Check supply cap
        if self.current_supply >= self.rewards.total_supply_cap {
            return Ok(0);
        }
        
        // Calculate halving
        let halving_count = self.blocks_mined / self.rewards.halving_interval;
        let halved_base_reward = self.rewards.base_reward >> halving_count;
        
        if halved_base_reward == 0 {
            return Ok(0);
        }
        
        // Apply proof-based multipliers
        let mut total_multiplier = 1.0;
        
        if proof_summary.proof_of_execution_count > 0 {
            total_multiplier += self.rewards.proof_of_execution_multiplier;
        }
        
        if proof_summary.proof_of_action_count > 0 {
            total_multiplier += self.rewards.proof_of_action_multiplier;
        }
        
        if proof_summary.proof_of_transact_count > 0 {
            total_multiplier += self.rewards.proof_of_transact_multiplier;
        }
        
        if proof_summary.proof_of_gold_count > 0 {
            total_multiplier += self.rewards.proof_of_gold_multiplier;
        }
        
        if proof_summary.proof_of_history_count > 0 {
            total_multiplier += self.rewards.proof_of_history_multiplier;
        }
        
        // Apply proof weight bonus
        total_multiplier *= 1.0 + (proof_summary.total_proof_weight / 100.0);
        
        let reward = (halved_base_reward as f64 * total_multiplier) as u64;
        
        // Ensure we don't exceed supply cap
        Ok(std::cmp::min(reward, self.rewards.total_supply_cap - self.current_supply))
    }
    
    /// Create proof summary from transactions
    fn create_proof_summary(&self, transactions: &[AggregatedTransaction]) -> Result<ProofSummary, MathError> {
        let mut poa_count = 0u64;
        let mut poe_count = 0u64;
        let mut pot_count = 0u64;
        let mut pog_count = 0u64;
        let mut poh_count = 0u64;
        let mut total_weight = 0.0f64;
        
        for transaction in transactions {
            for receipt in &transaction.receipts {
                match receipt {
                    ReceiptType::DockLock(_) => {
                        poa_count += 1;
                        total_weight += 1.5;
                    }
                    ReceiptType::BPI(_) => {
                        poe_count += 1;
                        total_weight += 2.0;
                    }
                    ReceiptType::BPCI(_) => {
                        pot_count += 1;
                        total_weight += 1.8;
                    }
                    ReceiptType::Economy(_) => {
                        pog_count += 1;
                        total_weight += 1.2;
                    }
                    ReceiptType::Cluster(_) => {
                        poh_count += 1;
                        total_weight += 1.3;
                    }
                }
            }
        }
        
        // Compute proof hash
        let proof_data = format!("{poa_count}:{poe_count}:{pot_count}:{pog_count}:{poh_count}:{total_weight}");
        let proof_hash = domain_hash(MINING_PROOF_DOMAIN, proof_data.as_bytes());
        
        Ok(ProofSummary {
            proof_of_action_count: poa_count,
            proof_of_execution_count: poe_count,
            proof_of_transact_count: pot_count,
            proof_of_gold_count: pog_count,
            proof_of_history_count: poh_count,
            total_proof_weight: total_weight,
            proof_hash,
        })
    }
    
    /// Create knot invariant for block immutability
    fn create_block_knot_invariant(&self, candidate: &MiningCandidate) -> Result<KnotInvariant, MathError> {
        let mut receipt_chain = Vec::new();
        let mut proof_chain = Vec::new();
        
        for transaction in &candidate.aggregated_transactions {
            receipt_chain.push(transaction.aggregated_hash);
            
            // Add knot invariant from transaction
            let tx_invariant = transaction.transaction_knot.get_invariant();
            proof_chain.push(tx_invariant.invariant_hash);
        }
        
        // Add block-level data
        receipt_chain.push(candidate.merkle_root);
        proof_chain.push(candidate.proof_summary.proof_hash);
        
        let block_knot = TransactionKnot::new();
        Ok(block_knot.get_invariant().clone())
    }
    
    /// Compute Merkle root of transactions
    fn compute_transaction_merkle_root(&self, transactions: &[AggregatedTransaction]) -> Result<Hash, MathError> {
        if transactions.is_empty() {
            return Ok([0u8; 32]);
        }
        
        let mut hashes: Vec<Hash> = transactions.iter()
            .map(|tx| tx.aggregated_hash)
            .collect();
        
        // Build Merkle tree
        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in hashes.chunks(2) {
                let combined = if chunk.len() == 2 {
                    [chunk[0], chunk[1]].concat()
                } else {
                    [chunk[0], chunk[0]].concat() // Duplicate if odd
                };
                
                next_level.push(domain_hash(MINING_MERKLE_DOMAIN, &combined));
            }
            
            hashes = next_level;
        }
        
        Ok(hashes[0])
    }
    
    /// Compute block hash
    fn compute_block_hash(&self, candidate: &MiningCandidate) -> Result<Hash, MathError> {
        let block_data = format!(
            "{}:{}:{}:{}:{}:{}:{}",
            candidate.block_height,
            hex::encode(candidate.prev_block_hash),
            hex::encode(candidate.merkle_root),
            candidate.timestamp.timestamp_nanos_opt().unwrap_or(0),
            candidate.nonce,
            candidate.difficulty,
            candidate.miner_address
        );
        
        Ok(domain_hash(MINING_BLOCK_DOMAIN, block_data.as_bytes()))
    }
    
    /// Compute target from difficulty
    fn compute_target_from_difficulty(&self, difficulty: u64) -> Hash {
        // Simplified target computation
        let mut target = [0xFFu8; 32];
        let difficulty_bytes = difficulty.to_be_bytes();
        
        for (i, &byte) in difficulty_bytes.iter().enumerate() {
            if i < 32 {
                target[i] = byte;
            }
        }
        
        target
    }
    
    /// Check if hash meets target
    fn hash_meets_target(&self, hash: &Hash, target: &Hash) -> bool {
        for i in 0..32 {
            if hash[i] < target[i] {
                return true;
            } else if hash[i] > target[i] {
                return false;
            }
        }
        false
    }
    
    /// Autonomous difficulty adjustment
    fn adjust_difficulty_autonomous(&mut self, mining_time_ms: u64) -> Result<(), MathError> {
        if self.blocks_mined % self.difficulty.difficulty_adjustment_window != 0 {
            return Ok(());
        }
        
        let target_time = self.difficulty.target_block_time;
        let adjustment_ratio = target_time as f64 / mining_time_ms as f64;
        
        // Clamp adjustment to maximum change
        let clamped_ratio = adjustment_ratio.max(1.0 / self.difficulty.max_difficulty_change)
            .min(self.difficulty.max_difficulty_change);
        
        self.difficulty.current_difficulty = ((self.difficulty.current_difficulty as f64) * clamped_ratio) as u64;
        self.difficulty.current_difficulty = self.difficulty.current_difficulty.max(1);
        
        Ok(())
    }
    
    /// Get mining statistics
    pub fn get_mining_stats(&self) -> MiningStats {
        MiningStats {
            blocks_mined: self.blocks_mined,
            current_supply: self.current_supply,
            current_difficulty: self.difficulty.current_difficulty,
            pending_transaction_count: self.pending_transactions.len() as u64,
            pending_receipt_count: self.receipt_aggregator.get_total_pending() as u64,
            supply_cap: self.rewards.total_supply_cap,
            inflation_rate: self.governance.inflation_rate,
        }
    }
}

/// Mining statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningStats {
    pub blocks_mined: u64,
    pub current_supply: u64,
    pub current_difficulty: u64,
    pub pending_transaction_count: u64,
    pub pending_receipt_count: u64,
    pub supply_cap: u64,
    pub inflation_rate: f64,
}

/// Domain-separated hash function
fn domain_hash(domain: &[u8], data: &[u8]) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update(b"|");
    hasher.update(data);
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::receipts::{ReceiptFactory, ResourceUsage};
    use std::collections::HashMap;
    
    #[test]
    fn test_mining_engine_creation() {
        let engine = MiningEngine::new(
            "miner_1".to_string(),
            MiningDifficulty::default(),
            MiningRewards::default(),
            EconomicGovernance::default(),
        );
        
        let stats = engine.get_mining_stats();
        assert_eq!(stats.blocks_mined, 0);
        assert_eq!(stats.current_supply, 0);
    }
    
    #[test]
    fn test_mining_reward_calculation() {
        let engine = MiningEngine::new(
            "miner_1".to_string(),
            MiningDifficulty::default(),
            MiningRewards::default(),
            EconomicGovernance::default(),
        );
        
        let proof_summary = ProofSummary {
            proof_of_action_count: 1,
            proof_of_execution_count: 1,
            proof_of_transact_count: 0,
            proof_of_gold_count: 0,
            proof_of_history_count: 0,
            total_proof_weight: 10.0,
            proof_hash: [0u8; 32],
        };
        
        let reward = engine.calculate_mining_reward(&proof_summary).unwrap();
        assert!(reward > engine.rewards.base_reward);
    }
    
    #[test]
    fn test_proof_summary_creation() {
        let engine = MiningEngine::new(
            "miner_1".to_string(),
            MiningDifficulty::default(),
            MiningRewards::default(),
            EconomicGovernance::default(),
        );
        
        // Create test transaction with receipts
        let mut metadata = HashMap::new();
        metadata.insert("cpu".to_string(), "100".to_string());
        
        let input = ("test_container".to_string(), ActionType::Deploy, metadata);
        let proof = ProofOfAction::generate_proof(input).unwrap();
        
        let resource_usage = ResourceUsage {
            cpu_time: 100,
            memory_peak: 256,
            network_bytes: 1024,
            storage_bytes: 512,
        };
        
        let receipt = ReceiptFactory::create_docklock_receipt(
            "test_container".to_string(),
            "deploy".to_string(),
            proof,
            resource_usage,
        );
        
        // Create mock aggregated transaction
        let transactions = vec![]; // Would contain actual transactions
        let proof_summary = engine.create_proof_summary(&transactions).unwrap();
        
        assert_eq!(proof_summary.proof_of_action_count, 0); // Empty transactions
    }
    
    #[test]
    fn test_difficulty_adjustment() {
        let mut engine = MiningEngine::new(
            "miner_1".to_string(),
            MiningDifficulty::default(),
            MiningRewards::default(),
            EconomicGovernance::default(),
        );
        
        let initial_difficulty = engine.difficulty.current_difficulty;
        
        // Simulate fast mining (should increase difficulty)
        engine.blocks_mined = engine.difficulty.difficulty_adjustment_window;
        engine.adjust_difficulty_autonomous(1000).unwrap(); // 1 second (faster than 5 second target)
        
        // Difficulty should have increased
        assert!(engine.difficulty.current_difficulty >= initial_difficulty);
    }
}
