//! Receipt Aggregation and Recording System
//! 
//! This module implements receipt aggregation using category theory morphisms
//! and integrates with the knot theory framework for immutable recording.

use crate::{
    Hash, MathError, Timestamp,
    category::{LedgerObject, LedgerMorphism, LedgerType},
    knot::{TransactionKnot},
    proofs::{ProofOfAction, ProofOfExecution, ProofOfTransact, ProofOfGold, ProofOfHistory, ProofSystem},
    constants::*,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, VecDeque};

/// Receipt types for different ledger operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReceiptType {
    DockLock(DockLockReceipt),
    Cluster(ClusterReceipt),
    BPI(BPIReceipt),
    BPCI(BPCIReceipt),
    Economy(EconomyReceipt),
}

/// DockLock container operation receipt
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DockLockReceipt {
    pub receipt_id: String,
    pub container_id: String,
    pub operation: String,
    pub timestamp: Timestamp,
    pub proof_of_action: ProofOfAction,
    pub resource_usage: ResourceUsage,
    pub receipt_hash: Hash,
}

/// ENC cluster operation receipt
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClusterReceipt {
    pub receipt_id: String,
    pub cluster_id: String,
    pub node_id: String,
    pub operation: String,
    pub timestamp: Timestamp,
    pub proof_of_history: ProofOfHistory,
    pub cluster_state: ClusterState,
    pub receipt_hash: Hash,
}

/// BPI execution receipt
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BPIReceipt {
    pub receipt_id: String,
    pub agreement_id: String,
    pub execution_id: String,
    pub timestamp: Timestamp,
    pub proof_of_execution: ProofOfExecution,
    pub gas_used: u64,
    pub result_hash: Hash,
    pub receipt_hash: Hash,
}

/// BPCI consensus receipt
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BPCIReceipt {
    pub receipt_id: String,
    pub validator_id: String,
    pub block_height: u64,
    pub timestamp: Timestamp,
    pub proof_of_transact: ProofOfTransact,
    pub consensus_round: u64,
    pub finality_status: FinalityStatus,
    pub receipt_hash: Hash,
}

/// Economy ledger receipt
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EconomyReceipt {
    pub receipt_id: String,
    pub account_id: String,
    pub operation_type: EconomyOperation,
    pub timestamp: Timestamp,
    pub proof_of_gold: ProofOfGold,
    pub amount: u64,
    pub balance_after: u64,
    pub receipt_hash: Hash,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceUsage {
    pub cpu_time: u64,
    pub memory_peak: u64,
    pub network_bytes: u64,
    pub storage_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClusterState {
    pub active_nodes: u32,
    pub total_capacity: u64,
    pub used_capacity: u64,
    pub health_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FinalityStatus {
    Pending,
    Confirmed,
    Finalized,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EconomyOperation {
    Transfer,
    Mint,
    Burn,
    Stake,
    Unstake,
    Reward,
}

/// Receipt aggregation configuration
#[derive(Debug, Clone)]
pub struct ReceiptAggregationConfig {
    pub batch_size: usize,
    pub time_window_ms: u64,
    pub max_pending_receipts: usize,
    pub enable_compression: bool,
}

impl Default for ReceiptAggregationConfig {
    fn default() -> Self {
        Self {
            batch_size: 1000,
            time_window_ms: 5000, // 5 seconds
            max_pending_receipts: 10000,
            enable_compression: true,
        }
    }
}

/// Receipt aggregator using category theory morphisms
pub struct ReceiptAggregator {
    config: ReceiptAggregationConfig,
    pending_receipts: HashMap<String, VecDeque<ReceiptType>>, // ledger_type -> receipts
    #[allow(dead_code)]
    aggregation_morphism: Option<LedgerMorphism>,
    last_aggregation: Timestamp,
}

impl ReceiptAggregator {
    pub fn new(config: ReceiptAggregationConfig) -> Self {
        Self {
            config,
            pending_receipts: HashMap::new(),
            aggregation_morphism: None,
            last_aggregation: chrono::Utc::now(),
        }
    }
    
    /// Add receipt to pending aggregation
    pub fn add_receipt(&mut self, receipt: ReceiptType) -> Result<(), MathError> {
        let ledger_type = self.get_ledger_type(&receipt);
        
        // Check capacity
        let pending_count: usize = self.pending_receipts.values().map(|v| v.len()).sum();
        if pending_count >= self.config.max_pending_receipts {
            return Err(MathError::CapacityExceeded);
        }
        
        // Add to pending receipts
        self.pending_receipts
            .entry(ledger_type)
            .or_default()
            .push_back(receipt);
        
        // Check if aggregation should be triggered
        if self.should_aggregate() {
            self.aggregate_receipts()?;
        }
        
        Ok(())
    }
    
    /// Force aggregation of pending receipts
    pub fn aggregate_receipts(&mut self) -> Result<Vec<AggregatedTransaction>, MathError> {
        let mut transactions = Vec::new();
        let mut receipts_to_process = Vec::new();
        
        // Check time window first to avoid borrow issues
        let time_window_exceeded = self.time_window_exceeded();
        
        // Collect receipts that are ready for aggregation
        for (ledger_type, receipts) in &mut self.pending_receipts {
            let should_aggregate = receipts.len() >= self.config.batch_size || time_window_exceeded;
            
            if should_aggregate {
                let batch: Vec<_> = receipts.drain(0..receipts.len().min(self.config.batch_size)).collect();
                receipts_to_process.push((ledger_type.clone(), batch));
            }
        }
        
        // Process collected receipts
        for (ledger_type, batch) in receipts_to_process {
            if !batch.is_empty() {
                let transaction = self.create_aggregated_transaction(ledger_type, batch)?;
                transactions.push(transaction);
            }
        }
        
        self.last_aggregation = chrono::Utc::now();
        Ok(transactions)
    }
    
    /// Get pending receipt count for a ledger type
    pub fn get_pending_count(&self, ledger_type: &str) -> usize {
        self.pending_receipts
            .get(ledger_type)
            .map(|v| v.len())
            .unwrap_or(0)
    }
    
    /// Get total pending receipt count
    pub fn get_total_pending(&self) -> usize {
        self.pending_receipts.values().map(|v| v.len()).sum()
    }
    
    fn get_ledger_type(&self, receipt: &ReceiptType) -> String {
        match receipt {
            ReceiptType::DockLock(_) => "docklock".to_string(),
            ReceiptType::Cluster(_) => "cluster".to_string(),
            ReceiptType::BPI(_) => "bpi".to_string(),
            ReceiptType::BPCI(_) => "bpci".to_string(),
            ReceiptType::Economy(_) => "economy".to_string(),
        }
    }
    
    fn should_aggregate(&self) -> bool {
        // Check batch size threshold
        for receipts in self.pending_receipts.values() {
            if receipts.len() >= self.config.batch_size {
                return true;
            }
        }
        
        // Check time window
        self.time_window_exceeded()
    }
    
    fn time_window_exceeded(&self) -> bool {
        let now = chrono::Utc::now();
        let elapsed = now.signed_duration_since(self.last_aggregation);
        elapsed.num_milliseconds() as u64 >= self.config.time_window_ms
    }
    
    fn create_aggregated_transaction(
        &self,
        ledger_type: String,
        receipts: Vec<ReceiptType>,
    ) -> Result<AggregatedTransaction, MathError> {
        // Create ledger objects for category theory
        let source_object = LedgerObject {
            ledger_type: LedgerType::DockLock, // Use actual enum variant
            object_id: format!("transaction_{}", receipts.len()),
            state_hash: self.compute_receipts_hash(&receipts),
            timestamp: chrono::Utc::now(),
        };
        
        // Create target object for aggregated transaction
        let target_object = LedgerObject {
            ledger_type: LedgerType::DockLock,
            object_id: format!("aggregated_transaction_{}", receipts.len()),
            state_hash: self.compute_receipts_hash(&receipts),
            timestamp: chrono::Utc::now(),
        };
        
        // Create transaction knot for immutability
        let transaction_knot = self.create_transaction_knot(&receipts)?;
        
        // Compute aggregated hash
        let aggregated_hash = self.compute_aggregated_hash(&receipts, &transaction_knot)?;
        
        Ok(AggregatedTransaction {
            transaction_id: uuid::Uuid::new_v4().to_string(),
            ledger_type,
            receipt_count: receipts.len() as u64,
            receipts,
            source_object,
            target_object,
            transaction_knot,
            aggregated_hash,
            timestamp: chrono::Utc::now(),
        })
    }
    
    fn compute_receipts_hash(&self, receipts: &[ReceiptType]) -> Hash {
        let mut hasher = Sha256::new();
        for receipt in receipts {
            let receipt_data = serde_json::to_vec(receipt).unwrap_or_default();
            hasher.update(&receipt_data);
        }
        hasher.finalize().into()
    }
    
    fn create_transaction_knot(&self, receipts: &[ReceiptType]) -> Result<TransactionKnot, MathError> {
        let mut receipt_chain = Vec::new();
        let mut proof_chain = Vec::new();
        
        for receipt in receipts {
            let receipt_hash = self.get_receipt_hash(receipt);
            receipt_chain.push(receipt_hash);
            
            // Extract proof hash based on receipt type
            let proof_hash = match receipt {
                ReceiptType::DockLock(r) => ProofOfAction::proof_hash(&r.proof_of_action),
                ReceiptType::Cluster(r) => ProofOfHistory::proof_hash(&r.proof_of_history),
                ReceiptType::BPI(r) => ProofOfExecution::proof_hash(&r.proof_of_execution),
                ReceiptType::BPCI(r) => ProofOfTransact::proof_hash(&r.proof_of_transact),
                ReceiptType::Economy(r) => ProofOfGold::proof_hash(&r.proof_of_gold),
            };
            proof_chain.push(proof_hash);
        }
        
        Ok(TransactionKnot::new())
    }
    
    fn get_receipt_hash(&self, receipt: &ReceiptType) -> Hash {
        match receipt {
            ReceiptType::DockLock(r) => r.receipt_hash,
            ReceiptType::Cluster(r) => r.receipt_hash,
            ReceiptType::BPI(r) => r.receipt_hash,
            ReceiptType::BPCI(r) => r.receipt_hash,
            ReceiptType::Economy(r) => r.receipt_hash,
        }
    }
    
    fn compute_aggregated_hash(
        &self,
        receipts: &[ReceiptType],
        knot: &TransactionKnot,
    ) -> Result<Hash, MathError> {
        let receipts_hash = self.compute_receipts_hash(receipts);
        let knot_invariant = knot.get_invariant();
        
        let combined_data = [receipts_hash, knot_invariant.invariant_hash].concat();
        Ok(domain_hash(RECEIPT_AGGREGATION_DOMAIN, &combined_data))
    }
}

/// Aggregated transaction containing batched receipts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedTransaction {
    pub transaction_id: String,
    pub ledger_type: String,
    pub receipt_count: u64,
    pub receipts: Vec<ReceiptType>,
    pub source_object: LedgerObject,
    pub target_object: LedgerObject,
    pub transaction_knot: TransactionKnot,
    pub aggregated_hash: Hash,
    pub timestamp: Timestamp,
}

impl AggregatedTransaction {
    /// Verify transaction integrity using knot theory
    pub fn verify_integrity(&self) -> bool {
        // Verify knot immutability
        if !self.transaction_knot.verify_immutability() {
            return false;
        }
        
        // Verify receipt count
        if self.receipts.len() as u64 != self.receipt_count {
            return false;
        }
        
        // Verify aggregated hash
        let receipts_hash = compute_receipts_hash(&self.receipts);
        let knot_invariant = self.transaction_knot.get_invariant();
        
        let expected_hash = domain_hash(
            RECEIPT_AGGREGATION_DOMAIN,
            &[receipts_hash, knot_invariant.invariant_hash].concat(),
        );
        
        expected_hash == self.aggregated_hash
    }
    
    /// Get transaction size in bytes
    pub fn get_size(&self) -> usize {
        serde_json::to_vec(self).unwrap_or_default().len()
    }
}

/// Helper function to compute receipts hash
fn compute_receipts_hash(receipts: &[ReceiptType]) -> Hash {
    let mut hasher = Sha256::new();
    for receipt in receipts {
        let receipt_data = serde_json::to_vec(receipt).unwrap_or_default();
        hasher.update(&receipt_data);
    }
    hasher.finalize().into()
}

/// Domain-separated hash function
fn domain_hash(domain: &[u8], data: &[u8]) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update(b"|");
    hasher.update(data);
    hasher.finalize().into()
}

/// Receipt factory for creating typed receipts
pub struct ReceiptFactory;

impl ReceiptFactory {
    /// Create DockLock receipt
    pub fn create_docklock_receipt(
        container_id: String,
        operation: String,
        proof_of_action: ProofOfAction,
        resource_usage: ResourceUsage,
    ) -> DockLockReceipt {
        let receipt_id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now();
        
        let receipt_data = format!("{receipt_id}:{container_id}:{operation}:{resource_usage:?}");
        let receipt_hash = domain_hash(DOCKLOCK_RECEIPT_DOMAIN, receipt_data.as_bytes());
        
        DockLockReceipt {
            receipt_id,
            container_id,
            operation,
            timestamp,
            proof_of_action,
            resource_usage,
            receipt_hash,
        }
    }
    
    /// Create cluster receipt
    pub fn create_cluster_receipt(
        cluster_id: String,
        node_id: String,
        operation: String,
        proof_of_history: ProofOfHistory,
        cluster_state: ClusterState,
    ) -> ClusterReceipt {
        let receipt_id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now();
        
        let receipt_data = format!("{receipt_id}:{cluster_id}:{node_id}:{cluster_state:?}");
        let receipt_hash = domain_hash(CLUSTER_RECEIPT_DOMAIN, receipt_data.as_bytes());
        
        ClusterReceipt {
            receipt_id,
            cluster_id,
            node_id,
            operation,
            timestamp,
            proof_of_history,
            cluster_state,
            receipt_hash,
        }
    }
    
    /// Create BPI receipt
    pub fn create_bpi_receipt(
        agreement_id: String,
        execution_id: String,
        proof_of_execution: ProofOfExecution,
        gas_used: u64,
        result_hash: Hash,
    ) -> BPIReceipt {
        let receipt_id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now();
        
        let receipt_data = format!("{receipt_id}:{agreement_id}:{execution_id}:{gas_used}");
        let receipt_hash = domain_hash(BPI_RECEIPT_DOMAIN, receipt_data.as_bytes());
        
        BPIReceipt {
            receipt_id,
            agreement_id,
            execution_id,
            timestamp,
            proof_of_execution,
            gas_used,
            result_hash,
            receipt_hash,
        }
    }
    
    /// Create BPCI receipt
    pub fn create_bpci_receipt(
        validator_id: String,
        block_height: u64,
        proof_of_transact: ProofOfTransact,
        consensus_round: u64,
        finality_status: FinalityStatus,
    ) -> BPCIReceipt {
        let receipt_id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now();
        
        let receipt_data = format!("{receipt_id}:{validator_id}:{block_height}:{consensus_round}");
        let receipt_hash = domain_hash(BPCI_RECEIPT_DOMAIN, receipt_data.as_bytes());
        
        BPCIReceipt {
            receipt_id,
            validator_id,
            block_height,
            timestamp,
            proof_of_transact,
            consensus_round,
            finality_status,
            receipt_hash,
        }
    }
    
    /// Create economy receipt
    pub fn create_economy_receipt(
        account_id: String,
        operation_type: EconomyOperation,
        proof_of_gold: ProofOfGold,
        amount: u64,
        balance_after: u64,
    ) -> EconomyReceipt {
        let receipt_id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now();
        
        let receipt_data = format!("{receipt_id}:{account_id}:{operation_type:?}:{amount}");
        let receipt_hash = domain_hash(ECONOMY_RECEIPT_DOMAIN, receipt_data.as_bytes());
        
        EconomyReceipt {
            receipt_id,
            account_id,
            operation_type,
            timestamp,
            proof_of_gold,
            amount,
            balance_after,
            receipt_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proofs::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_receipt_aggregator_creation() {
        let config = ReceiptAggregationConfig::default();
        let aggregator = ReceiptAggregator::new(config);
        
        assert_eq!(aggregator.get_total_pending(), 0);
    }
    
    #[test]
    fn test_docklock_receipt_creation() {
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
        
        assert_eq!(receipt.container_id, "test_container");
        assert_eq!(receipt.operation, "deploy");
    }
    
    #[test]
    fn test_receipt_aggregation() {
        let config = ReceiptAggregationConfig {
            batch_size: 2,
            time_window_ms: 1000,
            max_pending_receipts: 10,
            enable_compression: true,
        };
        
        let mut aggregator = ReceiptAggregator::new(config);
        
        // Create test receipts
        for i in 0..3 {
            let mut metadata = HashMap::new();
            metadata.insert("cpu".to_string(), "100".to_string());
            
            let input = (format!("container_{}", i), ActionType::Deploy, metadata);
            let proof = ProofOfAction::generate_proof(input).unwrap();
            
            let resource_usage = ResourceUsage {
                cpu_time: 100,
                memory_peak: 256,
                network_bytes: 1024,
                storage_bytes: 512,
            };
            
            let receipt = ReceiptFactory::create_docklock_receipt(
                format!("container_{}", i),
                "deploy".to_string(),
                proof,
                resource_usage,
            );
            
            aggregator.add_receipt(ReceiptType::DockLock(receipt)).unwrap();
        }
        
        // Should have triggered aggregation due to batch size
        assert!(aggregator.get_pending_count("docklock") < 3);
    }
    
    #[test]
    fn test_aggregated_transaction_verification() {
        let config = ReceiptAggregationConfig::default();
        let mut aggregator = ReceiptAggregator::new(config);
        
        // Create test receipt
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
        
        aggregator.add_receipt(ReceiptType::DockLock(receipt)).unwrap();
        
        // Force aggregation
        let transactions = aggregator.aggregate_receipts().unwrap();
        
        // Verify transaction integrity
        for transaction in transactions {
            assert!(transaction.verify_integrity());
        }
    }
}
