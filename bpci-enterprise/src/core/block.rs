//! Production-Grade Block Management for BPCI Enterprise
//! 
//! This module provides real, functional block creation, validation,
//! and chain management capabilities for the BPCI blockchain.

use crate::core::types::{NodeId, TransactionId, BlockHeight, Timestamp};
use crate::core::transaction::{Transaction, TransactionType};
use crate::core::storage::{StorageManager};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::{Result, anyhow};
use sha2::{Sha256, Digest};
use rust_decimal::Decimal;

/// Block header containing metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Block height in the chain
    pub height: BlockHeight,
    /// Hash of the previous block
    pub previous_hash: String,
    /// Merkle root of all transactions
    pub merkle_root: String,
    /// Block timestamp
    pub timestamp: Timestamp,
    /// Block proposer/miner
    pub proposer: NodeId,
    /// Nonce for proof of work (if applicable)
    pub nonce: u64,
    /// Block difficulty
    pub difficulty: u32,
    /// Block version
    pub version: u32,
}

impl BlockHeader {
    /// Calculate the hash of this block header
    pub fn calculate_hash(&self) -> String {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(&self.height.value().to_le_bytes());
        hasher.update(self.previous_hash.as_bytes());
        hasher.update(self.merkle_root.as_bytes());
        hasher.update(&self.timestamp.unix_timestamp().to_le_bytes());
        hasher.update(self.proposer.as_str().as_bytes());
        hasher.update(&self.nonce.to_le_bytes());
        hasher.update(&self.difficulty.to_le_bytes());
        hasher.update(&self.version.to_le_bytes());
        format!("{:x}", hasher.finalize())
    }
}

/// A complete block in the blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Block header
    pub header: BlockHeader,
    /// List of transactions in this block
    pub transactions: Vec<Transaction>,
    /// Block hash (calculated from header)
    pub hash: String,
    /// Block size in bytes
    pub size: usize,
    /// Total fees collected in this block
    pub total_fees: Decimal,
    /// Gas used by all transactions
    pub gas_used: u64,
}

impl Block {
    /// Create a new block
    pub fn new(
        height: BlockHeight,
        previous_hash: String,
        transactions: Vec<Transaction>,
        proposer: NodeId,
        difficulty: u32,
    ) -> Self {
        let timestamp = Timestamp::now();
        let merkle_root = Self::calculate_merkle_root(&transactions);
        
        let header = BlockHeader {
            height,
            previous_hash,
            merkle_root,
            timestamp,
            proposer,
            nonce: 0,
            difficulty,
            version: 1,
        };
        
        let hash = header.calculate_hash();
        let size = Self::calculate_block_size(&header, &transactions);
        let total_fees: Decimal = transactions.iter().map(|tx| tx.fee.total_fee()).sum();
        let gas_used: u64 = transactions.iter().map(|tx| tx.fee.gas_limit).sum();
        
        Self {
            header,
            transactions,
            hash,
            size,
            total_fees,
            gas_used,
        }
    }
    
    /// Create the genesis block
    pub fn genesis(proposer: NodeId) -> Self {
        let genesis_tx = Transaction::new(
            proposer.clone(),
            TransactionType::DataStore {
                key: "genesis".to_string(),
                value: b"BPCI Genesis Block - Production Blockchain".to_vec(),
                ttl: None,
            },
            crate::core::transaction::TransactionFee::new(
                Decimal::ZERO,
                Decimal::ZERO,
                0,
            ),
            0,
        );
        
        Self::new(
            BlockHeight(0),
            "0".repeat(64), // Genesis has no previous block
            vec![genesis_tx],
            proposer,
            1,
        )
    }
    
    /// Validate this block
    pub fn validate(&self, previous_block: Option<&Block>) -> Result<()> {
        // Validate block hash
        let calculated_hash = self.header.calculate_hash();
        if calculated_hash != self.hash {
            return Err(anyhow!("Block hash mismatch"));
        }
        
        // Validate height sequence
        if let Some(prev) = previous_block {
            if self.header.height.value() != prev.header.height.value() + 1 {
                return Err(anyhow!("Invalid block height sequence"));
            }
            if self.header.previous_hash != prev.hash {
                return Err(anyhow!("Previous hash mismatch"));
            }
        } else if self.header.height.value() != 0 {
            return Err(anyhow!("Non-genesis block without previous block"));
        }
        
        // Validate merkle root
        let calculated_merkle = Self::calculate_merkle_root(&self.transactions);
        if calculated_merkle != self.header.merkle_root {
            return Err(anyhow!("Merkle root mismatch"));
        }
        
        // Validate transactions
        for tx in &self.transactions {
            if tx.fee.gas_limit == 0 && self.header.height.value() > 0 {
                return Err(anyhow!("Transaction gas limit cannot be zero"));
            }
        }
        
        // Validate timestamp (not too far in future)
        let now = Timestamp::now();
        if self.header.timestamp.unix_timestamp() > now.unix_timestamp() + 60 {
            return Err(anyhow!("Block timestamp too far in future"));
        }
        
        Ok(())
    }
    
    /// Calculate merkle root of transactions
    fn calculate_merkle_root(transactions: &[Transaction]) -> String {
        if transactions.is_empty() {
            return "0".repeat(64);
        }
        
        let mut hashes: Vec<String> = transactions
            .iter()
            .map(|tx| tx.hash.clone())
            .collect();
        
        // Build merkle tree
        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in hashes.chunks(2) {
                let mut hasher = <Sha256 as Digest>::new();
                hasher.update(chunk[0].as_bytes());
                if chunk.len() > 1 {
                    hasher.update(chunk[1].as_bytes());
                } else {
                    hasher.update(chunk[0].as_bytes()); // Duplicate if odd number
                }
                next_level.push(format!("{:x}", hasher.finalize()));
            }
            
            hashes = next_level;
        }
        
        hashes[0].clone()
    }
    
    /// Calculate block size in bytes
    fn calculate_block_size(header: &BlockHeader, transactions: &[Transaction]) -> usize {
        let header_size = serde_json::to_vec(header).unwrap_or_default().len();
        let tx_size: usize = transactions
            .iter()
            .map(|tx| serde_json::to_vec(tx).unwrap_or_default().len())
            .sum();
        header_size + tx_size
    }
}

/// Blockchain configuration
#[derive(Debug, Clone)]
pub struct BlockchainConfig {
    /// Maximum block size in bytes
    pub max_block_size: usize,
    /// Maximum transactions per block
    pub max_transactions_per_block: usize,
    /// Target block time in seconds
    pub target_block_time: u64,
    /// Maximum block time drift allowed
    pub max_time_drift: u64,
}

impl Default for BlockchainConfig {
    fn default() -> Self {
        Self {
            max_block_size: 2 * 1024 * 1024, // 2MB
            max_transactions_per_block: 1000,
            target_block_time: 60, // 1 minute
            max_time_drift: 300, // 5 minutes
        }
    }
}

/// Blockchain statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainStats {
    pub chain_height: u64,
    pub total_blocks: usize,
    pub total_transactions: usize,
    pub total_fees: Decimal,
    pub average_block_size: f64,
    pub last_block_time: Option<Timestamp>,
}

/// Production-grade blockchain manager
#[derive(Debug)]
pub struct Blockchain {
    /// Configuration
    config: BlockchainConfig,
    /// Chain of blocks (in memory cache)
    blocks: Arc<RwLock<HashMap<u64, Block>>>,
    /// Current chain tip
    chain_tip: Arc<RwLock<Option<Block>>>,
    /// Storage for persistence
    storage: Arc<StorageManager>,
    /// Node ID of this blockchain instance
    node_id: NodeId,
}

impl Blockchain {
    /// Create a new blockchain
    pub async fn new(config: BlockchainConfig, storage: Arc<StorageManager>, node_id: NodeId) -> Result<Self> {
        let blockchain = Self {
            config,
            blocks: Arc::new(RwLock::new(HashMap::new())),
            chain_tip: Arc::new(RwLock::new(None)),
            storage,
            node_id,
        };
        
        // Load existing chain from storage or create genesis
        blockchain.initialize_chain().await?;
        
        Ok(blockchain)
    }
    
    /// Initialize the blockchain (load from storage or create genesis)
    async fn initialize_chain(&self) -> Result<()> {
        // Try to load existing chain
        if let Ok(Some(tip_entry)) = self.storage.get("blocks", "chain_tip").await {
            if let Ok(tip_height) = String::from_utf8(tip_entry.value) {
                if let Ok(height) = tip_height.parse::<u64>() {
                    // Load blocks from storage
                    for h in 0..=height {
                        let block_key = format!("block_{}", h);
                        if let Ok(Some(block_entry)) = self.storage.get("blocks", &block_key).await {
                            if let Ok(block) = serde_json::from_slice::<Block>(&block_entry.value) {
                                let mut blocks = self.blocks.write().await;
                                blocks.insert(h, block.clone());
                                
                                if h == height {
                                    let mut chain_tip = self.chain_tip.write().await;
                                    *chain_tip = Some(block);
                                }
                            }
                        }
                    }
                    return Ok(());
                }
            }
        }
        
        // No existing chain found, create genesis
        let genesis = Block::genesis(self.node_id.clone());
        self.add_block(genesis).await?;
        
        Ok(())
    }
    
    /// Add a new block to the chain
    pub async fn add_block(&self, block: Block) -> Result<()> {
        // Validate the block
        let previous_block = if block.header.height.value() > 0 {
            let blocks = self.blocks.read().await;
            blocks.get(&(block.header.height.value() - 1)).cloned()
        } else {
            None
        };
        
        block.validate(previous_block.as_ref())?;
        
        // Check size limits
        if block.size > self.config.max_block_size {
            return Err(anyhow!("Block size exceeds maximum"));
        }
        
        if block.transactions.len() > self.config.max_transactions_per_block {
            return Err(anyhow!("Too many transactions in block"));
        }
        
        let height = block.header.height.value();
        
        // Add to in-memory storage
        {
            let mut blocks = self.blocks.write().await;
            blocks.insert(height, block.clone());
        }
        
        // Update chain tip
        {
            let mut chain_tip = self.chain_tip.write().await;
            *chain_tip = Some(block.clone());
        }
        
        // Persist to storage
        self.persist_block(&block).await?;
        
        Ok(())
    }
    
    /// Get a block by height
    pub async fn get_block(&self, height: u64) -> Option<Block> {
        let blocks = self.blocks.read().await;
        blocks.get(&height).cloned()
    }
    
    /// Get the current chain tip
    pub async fn get_chain_tip(&self) -> Option<Block> {
        let chain_tip = self.chain_tip.read().await;
        chain_tip.clone()
    }
    
    /// Get blockchain statistics
    pub async fn get_stats(&self) -> BlockchainStats {
        let blocks = self.blocks.read().await;
        let chain_tip = self.chain_tip.read().await;
        
        let total_blocks = blocks.len();
        let chain_height = chain_tip.as_ref().map(|b| b.header.height.value()).unwrap_or(0);
        
        let total_transactions: usize = blocks.values().map(|b| b.transactions.len()).sum();
        let total_fees: Decimal = blocks.values().map(|b| b.total_fees).sum();
        let total_size: usize = blocks.values().map(|b| b.size).sum();
        let average_block_size = if total_blocks > 0 {
            total_size as f64 / total_blocks as f64
        } else {
            0.0
        };
        
        BlockchainStats {
            chain_height,
            total_blocks,
            total_transactions,
            total_fees,
            average_block_size,
            last_block_time: chain_tip.as_ref().map(|b| b.header.timestamp.clone()),
        }
    }
    
    /// Create a new block from pending transactions
    pub async fn create_block(&self, transactions: Vec<Transaction>) -> Result<Block> {
        let chain_tip = self.chain_tip.read().await;
        
        let (height, previous_hash) = if let Some(ref tip) = *chain_tip {
            (BlockHeight(tip.header.height.value() + 1), tip.hash.clone())
        } else {
            (BlockHeight(1), "0".repeat(64))
        };
        
        // Validate transactions fit in block
        if transactions.len() > self.config.max_transactions_per_block {
            return Err(anyhow!("Too many transactions for block"));
        }
        
        let block = Block::new(height, previous_hash, transactions, self.node_id.clone(), 1);
        
        if block.size > self.config.max_block_size {
            return Err(anyhow!("Block would exceed size limit"));
        }
        
        Ok(block)
    }
    
    /// Persist a block to storage
    async fn persist_block(&self, block: &Block) -> Result<()> {
        let block_data = serde_json::to_vec(block)?;
        let block_key = format!("block_{}", block.header.height.value());
        
        // Store the block
        self.storage.store("blocks", block_key, block_data).await?;
        
        // Update chain tip pointer
        let tip_data = block.header.height.value().to_string().into_bytes();
        self.storage.store("blocks", "chain_tip".to_string(), tip_data).await?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::transaction::{TransactionFee, TransactionType};
    use tempfile::TempDir;

    async fn create_test_blockchain() -> (Blockchain, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let storage_config = crate::core::storage::StorageConfig {
            base_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        
        let storage = Arc::new(StorageManager::new(storage_config).await.unwrap());
        let config = BlockchainConfig::default();
        let node_id = NodeId::new();
        
        let blockchain = Blockchain::new(config, storage, node_id).await.unwrap();
        (blockchain, temp_dir)
    }

    fn create_test_transaction() -> Transaction {
        let from = NodeId::new();
        let tx_type = TransactionType::Transfer {
            from: "alice".to_string(),
            to: "bob".to_string(),
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
    async fn test_blockchain_creation() {
        let (blockchain, _temp_dir) = create_test_blockchain().await;
        
        // Should have genesis block
        let stats = blockchain.get_stats().await;
        assert_eq!(stats.total_blocks, 1);
        assert_eq!(stats.chain_height, 0);
        
        let genesis = blockchain.get_block(0).await;
        assert!(genesis.is_some());
        assert_eq!(genesis.unwrap().header.height.value(), 0);
    }

    #[tokio::test]
    async fn test_block_creation() {
        let transactions = vec![create_test_transaction()];
        let proposer = NodeId::new();
        
        let block = Block::new(
            BlockHeight(1),
            "previous_hash".to_string(),
            transactions,
            proposer,
            1,
        );
        
        assert_eq!(block.header.height.value(), 1);
        assert_eq!(block.transactions.len(), 1);
        assert!(block.total_fees > Decimal::ZERO);
        assert!(!block.hash.is_empty());
    }

    #[tokio::test]
    async fn test_block_validation() {
        let transactions = vec![create_test_transaction()];
        let proposer = NodeId::new();
        
        let genesis = Block::genesis(proposer.clone());
        let block = Block::new(
            BlockHeight(1),
            genesis.hash.clone(),
            transactions,
            proposer,
            1,
        );
        
        // Should validate successfully
        assert!(block.validate(Some(&genesis)).is_ok());
        
        // Should fail with wrong previous hash
        let mut invalid_block = block.clone();
        invalid_block.header.previous_hash = "wrong_hash".to_string();
        invalid_block.hash = invalid_block.header.calculate_hash();
        
        assert!(invalid_block.validate(Some(&genesis)).is_err());
    }

    #[tokio::test]
    async fn test_blockchain_add_block() {
        let (blockchain, _temp_dir) = create_test_blockchain().await;
        
        let transactions = vec![create_test_transaction()];
        let block = blockchain.create_block(transactions).await.unwrap();
        
        blockchain.add_block(block.clone()).await.unwrap();
        
        let stats = blockchain.get_stats().await;
        assert_eq!(stats.total_blocks, 2); // Genesis + new block
        assert_eq!(stats.chain_height, 1);
        
        let retrieved = blockchain.get_block(1).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().hash, block.hash);
    }

    #[tokio::test]
    async fn test_merkle_root_calculation() {
        let tx1 = create_test_transaction();
        let tx2 = create_test_transaction();
        
        let merkle_root = Block::calculate_merkle_root(&[tx1, tx2]);
        assert!(!merkle_root.is_empty());
        assert_ne!(merkle_root, "0".repeat(64));
    }

    #[tokio::test]
    async fn test_blockchain_persistence() {
        let temp_dir = TempDir::new().unwrap();
        let storage_config = crate::core::storage::StorageConfig {
            base_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        
        let storage = Arc::new(StorageManager::new(storage_config).await.unwrap());
        let config = BlockchainConfig::default();
        let node_id = NodeId::new();
        
        // Create blockchain and add a block
        {
            let blockchain = Blockchain::new(config.clone(), storage.clone(), node_id.clone()).await.unwrap();
            let transactions = vec![create_test_transaction()];
            let block = blockchain.create_block(transactions).await.unwrap();
            blockchain.add_block(block).await.unwrap();
        }
        
        // Create new blockchain instance (should load from storage)
        {
            let blockchain2 = Blockchain::new(config, storage, node_id).await.unwrap();
            let stats = blockchain2.get_stats().await;
            assert_eq!(stats.total_blocks, 2); // Should have loaded both blocks
            assert_eq!(stats.chain_height, 1);
        }
    }
}
