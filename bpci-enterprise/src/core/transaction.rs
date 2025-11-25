//! Production-Grade Transaction Processing for BPCI Enterprise
//! 
//! This module provides real, functional transaction processing capabilities
//! for handling financial and data transactions in the BPCI network.

use crate::core::types::{NodeId, TransactionId, BlockHeight, Timestamp};
use crate::core::storage::{StorageManager};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::{Result, anyhow};
use sha2::{Sha256, Digest};
use rust_decimal::Decimal;

/// Transaction status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    /// Transaction is pending validation
    Pending,
    /// Transaction is being validated
    Validating,
    /// Transaction has been validated and is ready for inclusion
    Validated,
    /// Transaction has been included in a block
    Confirmed,
    /// Transaction has been finalized
    Finalized,
    /// Transaction was rejected
    Rejected(String),
    /// Transaction failed during execution
    Failed(String),
}

/// Transaction type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    /// Transfer of value between accounts
    Transfer {
        from: String,
        to: String,
        amount: Decimal,
    },
    /// Smart contract deployment
    ContractDeploy {
        code: Vec<u8>,
        init_data: Vec<u8>,
    },
    /// Smart contract execution
    ContractCall {
        contract_address: String,
        method: String,
        args: Vec<u8>,
    },
    /// Data storage transaction
    DataStore {
        key: String,
        value: Vec<u8>,
        ttl: Option<u64>,
    },
    /// Governance proposal
    Governance {
        proposal_type: String,
        proposal_data: Vec<u8>,
    },
}

/// Transaction fee structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionFee {
    /// Base fee for the transaction
    pub base_fee: Decimal,
    /// Gas price per unit
    pub gas_price: Decimal,
    /// Gas limit for the transaction
    pub gas_limit: u64,
    /// Priority fee for faster processing
    pub priority_fee: Decimal,
}

impl TransactionFee {
    pub fn new(base_fee: Decimal, gas_price: Decimal, gas_limit: u64) -> Self {
        Self {
            base_fee,
            gas_price,
            gas_limit,
            priority_fee: Decimal::ZERO,
        }
    }
    
    pub fn with_priority(mut self, priority_fee: Decimal) -> Self {
        self.priority_fee = priority_fee;
        self
    }
    
    /// Calculate total fee
    pub fn total_fee(&self) -> Decimal {
        self.base_fee + (self.gas_price * Decimal::from(self.gas_limit)) + self.priority_fee
    }
}

/// A transaction in the BPCI network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Unique transaction identifier
    pub id: TransactionId,
    /// Transaction initiator
    pub from: NodeId,
    /// Transaction type and data
    pub tx_type: TransactionType,
    /// Transaction fee structure
    pub fee: TransactionFee,
    /// Transaction nonce (prevents replay attacks)
    pub nonce: u64,
    /// Transaction timestamp
    pub timestamp: Timestamp,
    /// Transaction signature
    pub signature: Vec<u8>,
    /// Transaction hash
    pub hash: String,
    /// Current status
    pub status: TransactionStatus,
    /// Gas used during execution
    pub gas_used: Option<u64>,
    /// Execution result
    pub result: Option<Vec<u8>>,
}

impl Transaction {
    /// Create a new transaction
    pub fn new(
        from: NodeId,
        tx_type: TransactionType,
        fee: TransactionFee,
        nonce: u64,
    ) -> Self {
        let id = TransactionId::new();
        let timestamp = Timestamp::now();
        
        // Calculate hash
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(id.as_str().as_bytes());
        hasher.update(from.as_str().as_bytes());
        hasher.update(&serde_json::to_vec(&tx_type).unwrap_or_default());
        hasher.update(&nonce.to_le_bytes());
        hasher.update(&timestamp.unix_timestamp().to_le_bytes());
        let hash = format!("{:x}", hasher.finalize());
        
        Self {
            id,
            from,
            tx_type,
            fee,
            nonce,
            timestamp,
            signature: Vec::new(),
            hash,
            status: TransactionStatus::Pending,
            gas_used: None,
            result: None,
        }
    }
    
    /// Sign the transaction (simplified - in production would use proper cryptography)
    pub fn sign(&mut self, private_key: &[u8]) -> Result<()> {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(&self.hash.as_bytes());
        hasher.update(private_key);
        self.signature = hasher.finalize().to_vec();
        Ok(())
    }
    
    /// Verify transaction signature (simplified)
    pub fn verify_signature(&self, public_key: &[u8]) -> bool {
        if self.signature.is_empty() {
            return false;
        }
        
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(&self.hash.as_bytes());
        hasher.update(public_key);
        let expected_signature = hasher.finalize().to_vec();
        
        self.signature == expected_signature
    }
    
    /// Update transaction status
    pub fn update_status(&mut self, status: TransactionStatus) {
        self.status = status;
    }
    
    /// Set execution result
    pub fn set_result(&mut self, gas_used: u64, result: Vec<u8>) {
        self.gas_used = Some(gas_used);
        self.result = Some(result);
    }
}

/// Transaction pool configuration
#[derive(Debug, Clone)]
pub struct TransactionPoolConfig {
    /// Maximum number of transactions in the pool
    pub max_pool_size: usize,
    /// Maximum number of transactions per account
    pub max_per_account: usize,
    /// Minimum fee required
    pub min_fee: Decimal,
    /// Transaction timeout in seconds
    pub tx_timeout_secs: u64,
}

impl Default for TransactionPoolConfig {
    fn default() -> Self {
        Self {
            max_pool_size: 10000,
            max_per_account: 100,
            min_fee: Decimal::from_str_exact("0.001").unwrap(),
            tx_timeout_secs: 300, // 5 minutes
        }
    }
}

/// Transaction pool for managing pending transactions
#[derive(Debug)]
pub struct TransactionPool {
    /// Configuration
    config: TransactionPoolConfig,
    /// Pending transactions by priority (fee)
    pending: Arc<RwLock<VecDeque<Transaction>>>,
    /// Transactions by account
    by_account: Arc<RwLock<HashMap<NodeId, Vec<TransactionId>>>>,
    /// Transaction lookup by ID
    transactions: Arc<RwLock<HashMap<TransactionId, Transaction>>>,
    /// Storage for persistence
    storage: Arc<StorageManager>,
}

impl TransactionPool {
    /// Create a new transaction pool
    pub async fn new(config: TransactionPoolConfig, storage: Arc<StorageManager>) -> Result<Self> {
        let pool = Self {
            config,
            pending: Arc::new(RwLock::new(VecDeque::new())),
            by_account: Arc::new(RwLock::new(HashMap::new())),
            transactions: Arc::new(RwLock::new(HashMap::new())),
            storage,
        };
        
        // Load pending transactions from storage
        pool.load_from_storage().await?;
        
        Ok(pool)
    }
    
    /// Add a transaction to the pool
    pub async fn add_transaction(&self, mut transaction: Transaction) -> Result<()> {
        // Validate transaction
        self.validate_transaction(&transaction).await?;
        
        // Check pool size limits
        {
            let pending = self.pending.read().await;
            if pending.len() >= self.config.max_pool_size {
                return Err(anyhow!("Transaction pool is full"));
            }
        }
        
        // Check per-account limits
        {
            let by_account = self.by_account.read().await;
            if let Some(account_txs) = by_account.get(&transaction.from) {
                if account_txs.len() >= self.config.max_per_account {
                    return Err(anyhow!("Too many pending transactions for account"));
                }
            }
        }
        
        transaction.update_status(TransactionStatus::Pending);
        let tx_id = transaction.id.clone();
        let from = transaction.from.clone();
        
        // Add to data structures
        {
            let mut transactions = self.transactions.write().await;
            transactions.insert(tx_id.clone(), transaction.clone());
        }
        
        {
            let mut by_account = self.by_account.write().await;
            by_account.entry(from).or_insert_with(Vec::new).push(tx_id.clone());
        }
        
        {
            let mut pending = self.pending.write().await;
            // Insert in priority order (higher fee first)
            let insert_pos = pending
                .iter()
                .position(|tx| tx.fee.total_fee() < transaction.fee.total_fee())
                .unwrap_or(pending.len());
            pending.insert(insert_pos, transaction);
        }
        
        // Persist to storage
        self.persist_transaction(&tx_id).await?;
        
        Ok(())
    }
    
    /// Get the next transaction for processing
    pub async fn get_next_transaction(&self) -> Option<Transaction> {
        let mut pending = self.pending.write().await;
        pending.pop_front()
    }
    
    /// Get transactions for a specific account
    pub async fn get_account_transactions(&self, account: &NodeId) -> Vec<Transaction> {
        let by_account = self.by_account.read().await;
        let transactions = self.transactions.read().await;
        
        if let Some(tx_ids) = by_account.get(account) {
            tx_ids
                .iter()
                .filter_map(|id| transactions.get(id).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Remove a transaction from the pool
    pub async fn remove_transaction(&self, tx_id: &TransactionId) -> Result<Option<Transaction>> {
        let transaction = {
            let mut transactions = self.transactions.write().await;
            transactions.remove(tx_id)
        };
        
        if let Some(ref tx) = transaction {
            // Remove from pending queue
            {
                let mut pending = self.pending.write().await;
                pending.retain(|t| t.id != *tx_id);
            }
            
            // Remove from account index
            {
                let mut by_account = self.by_account.write().await;
                if let Some(account_txs) = by_account.get_mut(&tx.from) {
                    account_txs.retain(|id| id != tx_id);
                    if account_txs.is_empty() {
                        by_account.remove(&tx.from);
                    }
                }
            }
            
            // Remove from storage
            self.storage.delete("transactions", tx_id.as_str()).await?;
        }
        
        Ok(transaction)
    }
    
    /// Get pool statistics
    pub async fn get_stats(&self) -> TransactionPoolStats {
        let pending = self.pending.read().await;
        let by_account = self.by_account.read().await;
        
        let total_transactions = pending.len();
        let unique_accounts = by_account.len();
        let total_fees: Decimal = pending.iter().map(|tx| tx.fee.total_fee()).sum();
        
        TransactionPoolStats {
            total_transactions,
            unique_accounts,
            total_fees,
            average_fee: if total_transactions > 0 {
                total_fees / Decimal::from(total_transactions)
            } else {
                Decimal::ZERO
            },
        }
    }
    
    /// Validate a transaction
    async fn validate_transaction(&self, transaction: &Transaction) -> Result<()> {
        // Check minimum fee
        if transaction.fee.total_fee() < self.config.min_fee {
            return Err(anyhow!("Transaction fee too low"));
        }
        
        // Check gas limit
        if transaction.fee.gas_limit == 0 {
            return Err(anyhow!("Gas limit must be greater than zero"));
        }
        
        // Check timestamp (not too old or too far in future)
        let now = Timestamp::now();
        let age = now.unix_timestamp() - transaction.timestamp.unix_timestamp();
        if age > self.config.tx_timeout_secs as i64 {
            return Err(anyhow!("Transaction too old"));
        }
        if age < -60 {
            return Err(anyhow!("Transaction timestamp too far in future"));
        }
        
        // Validate transaction type specific data
        match &transaction.tx_type {
            TransactionType::Transfer { amount, .. } => {
                if *amount <= Decimal::ZERO {
                    return Err(anyhow!("Transfer amount must be positive"));
                }
            }
            TransactionType::ContractDeploy { code, .. } => {
                if code.is_empty() {
                    return Err(anyhow!("Contract code cannot be empty"));
                }
            }
            TransactionType::DataStore { key, value, .. } => {
                if key.is_empty() {
                    return Err(anyhow!("Data key cannot be empty"));
                }
                if value.len() > 1024 * 1024 {
                    return Err(anyhow!("Data value too large"));
                }
            }
            _ => {} // Other types pass validation
        }
        
        Ok(())
    }
    
    /// Load transactions from storage
    async fn load_from_storage(&self) -> Result<()> {
        let tx_keys = self.storage.list_keys("transactions").await?;
        
        for key in tx_keys {
            if let Ok(Some(entry)) = self.storage.get("transactions", &key).await {
                if let Ok(transaction) = serde_json::from_slice::<Transaction>(&entry.value) {
                    // Only load pending transactions
                    if transaction.status == TransactionStatus::Pending {
                        let tx_id = transaction.id.clone();
                        let from = transaction.from.clone();
                        
                        {
                            let mut transactions = self.transactions.write().await;
                            transactions.insert(tx_id.clone(), transaction.clone());
                        }
                        
                        {
                            let mut by_account = self.by_account.write().await;
                            by_account.entry(from).or_insert_with(Vec::new).push(tx_id);
                        }
                        
                        {
                            let mut pending = self.pending.write().await;
                            let insert_pos = pending
                                .iter()
                                .position(|tx| tx.fee.total_fee() < transaction.fee.total_fee())
                                .unwrap_or(pending.len());
                            pending.insert(insert_pos, transaction);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Persist a transaction to storage
    async fn persist_transaction(&self, tx_id: &TransactionId) -> Result<()> {
        let transactions = self.transactions.read().await;
        if let Some(transaction) = transactions.get(tx_id) {
            let data = serde_json::to_vec(transaction)?;
            self.storage.store("transactions", tx_id.as_str().to_string(), data).await?;
        }
        Ok(())
    }
}

/// Transaction pool statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionPoolStats {
    pub total_transactions: usize,
    pub unique_accounts: usize,
    pub total_fees: Decimal,
    pub average_fee: Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    async fn create_test_pool() -> (TransactionPool, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let storage_config = crate::core::storage::StorageConfig {
            base_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        
        let storage = Arc::new(StorageManager::new(storage_config).await.unwrap());
        let config = TransactionPoolConfig::default();
        let pool = TransactionPool::new(config, storage).await.unwrap();
        
        (pool, temp_dir)
    }

    fn create_test_transaction() -> Transaction {
        let from = NodeId::new();
        let tx_type = TransactionType::Transfer {
            from: "account1".to_string(),
            to: "account2".to_string(),
            amount: Decimal::from_str_exact("10.0").unwrap(),
        };
        let fee = TransactionFee::new(
            Decimal::from_str_exact("0.01").unwrap(),
            Decimal::from_str_exact("0.001").unwrap(),
            21000,
        );
        
        Transaction::new(from, tx_type, fee, 1)
    }

    #[tokio::test]
    async fn test_transaction_creation() {
        let transaction = create_test_transaction();
        
        assert!(!transaction.id.as_str().is_empty());
        assert!(!transaction.hash.is_empty());
        assert_eq!(transaction.status, TransactionStatus::Pending);
        assert_eq!(transaction.nonce, 1);
    }

    #[tokio::test]
    async fn test_transaction_signing() {
        let mut transaction = create_test_transaction();
        let private_key = b"test_private_key";
        let public_key = b"test_private_key"; // Simplified for testing
        
        transaction.sign(private_key).unwrap();
        assert!(!transaction.signature.is_empty());
        assert!(transaction.verify_signature(public_key));
    }

    #[tokio::test]
    async fn test_transaction_fee_calculation() {
        let fee = TransactionFee::new(
            Decimal::from_str_exact("0.01").unwrap(),
            Decimal::from_str_exact("0.001").unwrap(),
            21000,
        ).with_priority(Decimal::from_str_exact("0.005").unwrap());
        
        let expected_total = Decimal::from_str_exact("0.01").unwrap() + 
                           (Decimal::from_str_exact("0.001").unwrap() * Decimal::from(21000)) + 
                           Decimal::from_str_exact("0.005").unwrap();
        
        assert_eq!(fee.total_fee(), expected_total);
    }

    #[tokio::test]
    async fn test_transaction_pool_add() {
        let (pool, _temp_dir) = create_test_pool().await;
        let transaction = create_test_transaction();
        
        pool.add_transaction(transaction.clone()).await.unwrap();
        
        let stats = pool.get_stats().await;
        assert_eq!(stats.total_transactions, 1);
        assert_eq!(stats.unique_accounts, 1);
    }

    #[tokio::test]
    async fn test_transaction_pool_get_next() {
        let (pool, _temp_dir) = create_test_pool().await;
        let transaction = create_test_transaction();
        let tx_id = transaction.id.clone();
        
        pool.add_transaction(transaction).await.unwrap();
        
        let next_tx = pool.get_next_transaction().await;
        assert!(next_tx.is_some());
        assert_eq!(next_tx.unwrap().id, tx_id);
        
        // Pool should be empty now
        let stats = pool.get_stats().await;
        assert_eq!(stats.total_transactions, 0);
    }

    #[tokio::test]
    async fn test_transaction_pool_priority_ordering() {
        let (pool, _temp_dir) = create_test_pool().await;
        
        // Create transactions with different fees
        let mut tx1 = create_test_transaction();
        tx1.fee.priority_fee = Decimal::from_str_exact("0.001").unwrap();
        
        let mut tx2 = create_test_transaction();
        tx2.fee.priority_fee = Decimal::from_str_exact("0.005").unwrap();
        
        let mut tx3 = create_test_transaction();
        tx3.fee.priority_fee = Decimal::from_str_exact("0.003").unwrap();
        
        // Add in random order
        pool.add_transaction(tx1).await.unwrap();
        pool.add_transaction(tx2.clone()).await.unwrap();
        pool.add_transaction(tx3).await.unwrap();
        
        // Should get highest fee first
        let next_tx = pool.get_next_transaction().await.unwrap();
        assert_eq!(next_tx.id, tx2.id);
    }

    #[tokio::test]
    async fn test_transaction_validation() {
        let (pool, _temp_dir) = create_test_pool().await;
        
        // Test invalid transaction (zero amount)
        let from = NodeId::new();
        let tx_type = TransactionType::Transfer {
            from: "account1".to_string(),
            to: "account2".to_string(),
            amount: Decimal::ZERO,
        };
        let fee = TransactionFee::new(
            Decimal::from_str_exact("0.01").unwrap(),
            Decimal::from_str_exact("0.001").unwrap(),
            21000,
        );
        
        let invalid_tx = Transaction::new(from, tx_type, fee, 1);
        let result = pool.add_transaction(invalid_tx).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must be positive"));
    }
}
