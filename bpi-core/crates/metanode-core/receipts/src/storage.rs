//! Receipt storage and database management
//!
//! This module provides efficient storage, indexing, and retrieval of transaction receipts
//! using SQLite with optimized queries and bloom filter indexing.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sqlx::{SqlitePool, Row};
use tracing::{info, warn, error};

use crate::{TransactionReceipt, FinalityProof, FinalizedReceipt, ReceiptConfig, TransactionStatus, EventLog, GasUsage, BlsSignature};

/// Receipt storage engine with SQLite backend
#[derive(Debug, Clone)]
pub struct ReceiptStore {
    pub(crate) pool: SqlitePool,
    config: ReceiptConfig,
    cache: Arc<RwLock<HashMap<[u8; 32], TransactionReceipt>>>,
}

/// Storage statistics
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub total_receipts: u64,
    pub total_blocks: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub storage_size_bytes: u64,
    pub last_updated: DateTime<Utc>,
}

impl ReceiptStore {
    /// Create a new receipt store
    pub async fn new(config: ReceiptConfig) -> Result<Self> {
        let pool = SqlitePool::connect(&config.database_url)
            .await
            .context("Failed to connect to database")?;
            
        let store = Self {
            pool,
            config,
            cache: Arc::new(RwLock::new(HashMap::new())),
        };
        
        store.initialize_schema().await?;
        
        info!("Receipt store initialized with database: {}", store.config.database_url);
        
        Ok(store)
    }
    
    /// Initialize database schema
    async fn initialize_schema(&self) -> Result<()> {
        // Create receipts table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS receipts (
                tx_hash BLOB PRIMARY KEY,
                block_hash BLOB NOT NULL,
                block_height INTEGER NOT NULL,
                tx_index INTEGER NOT NULL,
                from_address BLOB NOT NULL,
                to_address BLOB,
                contract_address BLOB,
                status TEXT NOT NULL,
                gas_limit INTEGER NOT NULL,
                gas_used INTEGER NOT NULL,
                gas_price INTEGER NOT NULL,
                gas_fee INTEGER NOT NULL,
                logs_bloom BLOB NOT NULL,
                timestamp TEXT NOT NULL,
                created_at TEXT NOT NULL,
                receipt_data BLOB NOT NULL
            )
        "#)
        .execute(&self.pool)
        .await
        .context("Failed to create receipts table")?;
        
        // Create event logs table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS event_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                tx_hash BLOB NOT NULL,
                log_index INTEGER NOT NULL,
                address BLOB NOT NULL,
                topics BLOB NOT NULL,
                data BLOB NOT NULL,
                FOREIGN KEY (tx_hash) REFERENCES receipts (tx_hash)
            )
        "#)
        .execute(&self.pool)
        .await
        .context("Failed to create event_logs table")?;
        
        // Create finality proofs table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS finality_proofs (
                block_hash BLOB PRIMARY KEY,
                block_height INTEGER NOT NULL,
                commit_round INTEGER NOT NULL,
                aggregate_signature BLOB NOT NULL,
                validator_bitmap BLOB NOT NULL,
                validator_set BLOB NOT NULL,
                inclusion_proof BLOB NOT NULL,
                created_at TEXT NOT NULL
            )
        "#)
        .execute(&self.pool)
        .await
        .context("Failed to create finality_proofs table")?;
        
        // Create indexes for efficient queries
        self.create_indexes().await?;
        
        Ok(())
    }
    
    /// Create database indexes
    async fn create_indexes(&self) -> Result<()> {
        let indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_receipts_block_height ON receipts (block_height)",
            "CREATE INDEX IF NOT EXISTS idx_receipts_block_hash ON receipts (block_hash)",
            "CREATE INDEX IF NOT EXISTS idx_receipts_from_address ON receipts (from_address)",
            "CREATE INDEX IF NOT EXISTS idx_receipts_to_address ON receipts (to_address)",
            "CREATE INDEX IF NOT EXISTS idx_receipts_contract_address ON receipts (contract_address)",
            "CREATE INDEX IF NOT EXISTS idx_receipts_timestamp ON receipts (timestamp)",
            "CREATE INDEX IF NOT EXISTS idx_event_logs_address ON event_logs (address)",
            "CREATE INDEX IF NOT EXISTS idx_event_logs_tx_hash ON event_logs (tx_hash)",
            "CREATE INDEX IF NOT EXISTS idx_finality_proofs_height ON finality_proofs (block_height)",
        ];
        
        for index_sql in indexes {
            sqlx::query(index_sql)
                .execute(&self.pool)
                .await
                .context("Failed to create index")?;
        }
        
        Ok(())
    }
    
    /// Store a transaction receipt
    pub async fn store_receipt(&self, receipt: &TransactionReceipt) -> Result<()> {
        // Serialize receipt data
        let receipt_data = bincode::serialize(receipt)
            .map_err(|e| anyhow::anyhow!("Failed to serialize receipt: {}", e))?;
        
        // Store in database
        let mut tx = self.pool.begin().await?;
        
        // Insert receipt
        sqlx::query(r#"
            INSERT OR REPLACE INTO receipts (
                tx_hash, block_hash, block_height, tx_index,
                from_address, to_address, contract_address,
                status, gas_limit, gas_used, gas_price, gas_fee,
                logs_bloom, timestamp, created_at, receipt_data
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(&receipt.tx_hash[..])
        .bind(&receipt.block_hash[..])
        .bind(receipt.block_height as i64)
        .bind(receipt.tx_index as i64)
        .bind(&receipt.from[..])
        .bind(receipt.to.as_ref().map(|t| &t[..]))
        .bind(receipt.contract_address.as_ref().map(|a| &a[..]))
        .bind(self.status_to_string(&receipt.status))
        .bind(receipt.gas.gas_limit as i64)
        .bind(receipt.gas.gas_used as i64)
        .bind(receipt.gas.gas_price as i64)
        .bind(receipt.gas.gas_fee as i64)
        .bind(&receipt.logs_bloom[..])
        .bind(receipt.timestamp.to_rfc3339())
        .bind(receipt.created_at.to_rfc3339())
        .bind(&receipt_data)
        .execute(&mut *tx)
        .await
        .context("Failed to insert receipt")?;
        
        // Insert event logs
        for log in &receipt.logs {
            let topics_data = bincode::serialize(&log.topics)
                .context("Failed to serialize topics")?;
                
            sqlx::query(r#"
                INSERT INTO event_logs (tx_hash, log_index, address, topics, data)
                VALUES (?, ?, ?, ?, ?)
            "#)
            .bind(&receipt.tx_hash[..])
            .bind(log.log_index as i64)
            .bind(&log.address[..])
            .bind(&topics_data)
            .bind(&log.data)
            .execute(&mut *tx)
            .await
            .context("Failed to insert event log")?;
        }
        
        tx.commit().await?;
        
        // Update cache
        let mut cache = self.cache.write().await;
        cache.insert(receipt.tx_hash, receipt.clone());
        
        // Evict old entries if cache is full
        if cache.len() > self.config.cache_size {
            let keys_to_remove: Vec<_> = cache.keys().take(cache.len() - self.config.cache_size).cloned().collect();
            for key in keys_to_remove {
                cache.remove(&key);
            }
        }
        
        Ok(())
    }
    
    /// Store a finality proof
    pub async fn store_finality_proof(&self, proof: &FinalityProof) -> Result<()> {
        let validator_set_data = bincode::serialize(&proof.validator_set)
            .context("Failed to serialize validator set")?;
        let inclusion_proof_data = bincode::serialize(&proof.inclusion_proof)
            .context("Failed to serialize inclusion proof")?;
        
        sqlx::query(r#"
            INSERT OR REPLACE INTO finality_proofs (
                block_hash, block_height, commit_round,
                aggregate_signature, validator_bitmap, validator_set,
                inclusion_proof, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(&proof.block_hash[..])
        .bind(proof.block_height as i64)
        .bind(proof.commit_round as i64)
        .bind(&proof.aggregate_signature.as_bytes())
        .bind(&proof.validator_bitmap)
        .bind(&validator_set_data)
        .bind(&inclusion_proof_data)
        .bind(proof.created_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .context("Failed to store finality proof")?;
        
        Ok(())
    }
    
    /// Get receipt by transaction hash
    pub async fn get_receipt(&self, tx_hash: &[u8; 32]) -> Result<Option<TransactionReceipt>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(receipt) = cache.get(tx_hash) {
                return Ok(Some(receipt.clone()));
            }
        }
        
        // Query database
        let row = sqlx::query(r#"
            SELECT receipt_data FROM receipts WHERE tx_hash = ?
        "#)
        .bind(&tx_hash[..])
        .fetch_optional(&self.pool)
        .await
        .context("Failed to query receipt")?;
        
        if let Some(row) = row {
            let receipt_data: Vec<u8> = row.get("receipt_data");
            let receipt: TransactionReceipt = bincode::deserialize(&receipt_data)
                .context("Failed to deserialize receipt")?;
            
            // Update cache
            let mut cache = self.cache.write().await;
            cache.insert(*tx_hash, receipt.clone());
            
            Ok(Some(receipt))
        } else {
            Ok(None)
        }
    }
    
    /// Get receipts for a block
    pub async fn get_receipts_by_block(&self, block_hash: &[u8; 32]) -> Result<Vec<TransactionReceipt>> {
        let rows = sqlx::query(r#"
            SELECT receipt_data FROM receipts 
            WHERE block_hash = ? 
            ORDER BY tx_index
        "#)
        .bind(&block_hash[..])
        .fetch_all(&self.pool)
        .await
        .context("Failed to query receipts by block")?;
        
        let mut receipts = Vec::new();
        for row in rows {
            let receipt_data: Vec<u8> = row.get("receipt_data");
            let receipt: TransactionReceipt = bincode::deserialize(&receipt_data)
                .context("Failed to deserialize receipt")?;
            receipts.push(receipt);
        }
        
        Ok(receipts)
    }
    
    /// Get receipts by address (from or to)
    pub async fn get_receipts_by_address(&self, address: &[u8; 20], limit: Option<u32>) -> Result<Vec<TransactionReceipt>> {
        let limit_clause = if let Some(l) = limit {
            format!("LIMIT {}", l)
        } else {
            String::new()
        };
        
        let query = format!(r#"
            SELECT receipt_data FROM receipts 
            WHERE from_address = ? OR to_address = ? OR contract_address = ?
            ORDER BY block_height DESC, tx_index DESC
            {}
        "#, limit_clause);
        
        let rows = sqlx::query(&query)
            .bind(&address[..])
            .bind(&address[..])
            .bind(&address[..])
            .fetch_all(&self.pool)
            .await
            .context("Failed to query receipts by address")?;
        
        let mut receipts = Vec::new();
        for row in rows {
            let receipt_data: Vec<u8> = row.get("receipt_data");
            let receipt: TransactionReceipt = bincode::deserialize(&receipt_data)
                .context("Failed to deserialize receipt")?;
            receipts.push(receipt);
        }
        
        Ok(receipts)
    }
    
    /// Get finality proof for a block
    pub async fn get_finality_proof(&self, block_hash: &[u8; 32]) -> Result<Option<FinalityProof>> {
        let row = sqlx::query(r#"
            SELECT * FROM finality_proofs WHERE block_hash = ?
        "#)
        .bind(&block_hash[..])
        .fetch_optional(&self.pool)
        .await
        .context("Failed to query finality proof")?;
        
        if let Some(row) = row {
            let validator_set_data: Vec<u8> = row.get("validator_set");
            let inclusion_proof_data: Vec<u8> = row.get("inclusion_proof");
            let signature_bytes: Vec<u8> = row.get("aggregate_signature");
            
            let validator_set = bincode::deserialize(&validator_set_data)
                .context("Failed to deserialize validator set")?;
            let inclusion_proof = bincode::deserialize(&inclusion_proof_data)
                .context("Failed to deserialize inclusion proof")?;
            let aggregate_signature = BlsSignature::from_bytes(&signature_bytes)
                .map_err(|e| anyhow::anyhow!("Failed to deserialize BLS signature: {}", e))?;
            
            let created_at: String = row.get("created_at");
            let created_at = DateTime::parse_from_rfc3339(&created_at)
                .context("Failed to parse timestamp")?
                .with_timezone(&Utc);
            
            let proof = FinalityProof {
                block_height: row.get::<i64, _>("block_height") as u64,
                block_hash: {
                    let hash_bytes: Vec<u8> = row.get("block_hash");
                    let mut hash = [0u8; 32];
                    hash.copy_from_slice(&hash_bytes);
                    hash
                },
                commit_round: row.get::<i64, _>("commit_round") as u32,
                aggregate_signature,
                validator_bitmap: row.get("validator_bitmap"),
                validator_set,
                inclusion_proof,
                created_at,
            };
            
            Ok(Some(proof))
        } else {
            Ok(None)
        }
    }
    
    /// Get finalized receipt (receipt + finality proof)
    pub async fn get_finalized_receipt(&self, tx_hash: &[u8; 32]) -> Result<Option<FinalizedReceipt>> {
        let receipt = match self.get_receipt(tx_hash).await? {
            Some(r) => r,
            None => return Ok(None),
        };
        
        let finality_proof = match self.get_finality_proof(&receipt.block_hash).await? {
            Some(p) => p,
            None => return Ok(None),
        };
        
        Ok(Some(FinalizedReceipt::new(receipt, finality_proof)))
    }
    
    /// Get storage statistics
    pub async fn get_stats(&self) -> Result<StorageStats> {
        let total_receipts: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM receipts")
            .fetch_one(&self.pool)
            .await
            .context("Failed to count receipts")?;
            
        let total_blocks: i64 = sqlx::query_scalar("SELECT COUNT(DISTINCT block_hash) FROM receipts")
            .fetch_one(&self.pool)
            .await
            .context("Failed to count blocks")?;
        
        let cache = self.cache.read().await;
        
        Ok(StorageStats {
            total_receipts: total_receipts as u64,
            total_blocks: total_blocks as u64,
            cache_hits: 0, // TODO: Track cache hits/misses
            cache_misses: 0,
            storage_size_bytes: 0, // TODO: Calculate actual storage size
            last_updated: Utc::now(),
        })
    }
    
    /// Convert transaction status to string
    pub(crate) fn status_to_string(&self, status: &TransactionStatus) -> String {
        match status {
            TransactionStatus::Success => "success".to_string(),
            TransactionStatus::Failed(msg) => format!("failed:{}", msg),
            TransactionStatus::Reverted(reason) => format!("reverted:{}", reason),
            TransactionStatus::OutOfGas => "out_of_gas".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use crate::{EventLog, GasUsage};
    
    async fn create_test_store() -> Result<ReceiptStore> {
        let config = ReceiptConfig {
            database_url: "sqlite::memory:".to_string(),
            cache_size: 100,
            batch_size: 10,
            enable_bloom_indexing: true,
            finality_timeout_ms: 1000,
        };
        
        ReceiptStore::new(config).await
    }
    
    fn create_test_receipt() -> TransactionReceipt {
        let tx_hash = [1u8; 32];
        let block_hash = [2u8; 32];
        let from = [3u8; 20];
        let to = Some([4u8; 20]);
        
        let gas = GasUsage {
            gas_limit: 21000,
            gas_used: 21000,
            gas_price: 20_000_000_000,
            gas_fee: 21000 * 20_000_000_000,
        };
        
        let logs = vec![
            EventLog {
                address: [5u8; 20],
                topics: vec![[6u8; 32], [7u8; 32]],
                data: vec![1, 2, 3, 4],
                log_index: 0,
            }
        ];
        
        TransactionReceipt::new(
            tx_hash,
            block_hash,
            100,
            0,
            from,
            to,
            TransactionStatus::Success,
            gas,
            logs,
        )
    }
    
    #[tokio::test]
    async fn test_store_and_retrieve_receipt() {
        let store = create_test_store().await.unwrap();
        let receipt = create_test_receipt();
        
        // Store receipt
        store.store_receipt(&receipt).await.unwrap();
        
        // Retrieve receipt
        let retrieved = store.get_receipt(&receipt.tx_hash).await.unwrap();
        assert!(retrieved.is_some());
        
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.tx_hash, receipt.tx_hash);
        assert_eq!(retrieved.block_hash, receipt.block_hash);
        assert_eq!(retrieved.status, receipt.status);
    }
    
    #[tokio::test]
    async fn test_get_receipts_by_block() {
        let store = create_test_store().await.unwrap();
        let mut receipt1 = create_test_receipt();
        let mut receipt2 = create_test_receipt();
        
        receipt1.tx_hash = [1u8; 32];
        receipt1.tx_index = 0;
        receipt2.tx_hash = [2u8; 32];
        receipt2.tx_index = 1;
        
        store.store_receipt(&receipt1).await.unwrap();
        store.store_receipt(&receipt2).await.unwrap();
        
        let receipts = store.get_receipts_by_block(&receipt1.block_hash).await.unwrap();
        assert_eq!(receipts.len(), 2);
        assert_eq!(receipts[0].tx_index, 0);
        assert_eq!(receipts[1].tx_index, 1);
    }
    
    #[tokio::test]
    async fn test_get_receipts_by_address() {
        let store = create_test_store().await.unwrap();
        let receipt = create_test_receipt();
        
        store.store_receipt(&receipt).await.unwrap();
        
        let receipts = store.get_receipts_by_address(&receipt.from, Some(10)).await.unwrap();
        assert_eq!(receipts.len(), 1);
        assert_eq!(receipts[0].tx_hash, receipt.tx_hash);
    }
    
    #[tokio::test]
    async fn test_storage_stats() {
        let store = create_test_store().await.unwrap();
        let receipt = create_test_receipt();
        
        store.store_receipt(&receipt).await.unwrap();
        
        let stats = store.get_stats().await.unwrap();
        assert_eq!(stats.total_receipts, 1);
        assert_eq!(stats.total_blocks, 1);
    }
}
