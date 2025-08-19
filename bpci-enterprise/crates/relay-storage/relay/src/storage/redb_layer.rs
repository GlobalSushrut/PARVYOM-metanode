//! Layer 3: Redb - Military-Grade Warm Data Storage
//! 
//! Zero-copy reads for warm data with military-grade ACID compliance

use anyhow::{Result, anyhow};
use redb::{Database, ReadableTable, TableDefinition};
use std::path::Path;
use tracing::{info, error};

use super::MilitaryStorageLayer;

/// Table definition for military-grade key-value storage
const MILITARY_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("military_kv");

/// Redb layer for military-grade warm data storage
pub struct RedbLayer {
    db: Database,
}

impl RedbLayer {
    /// Create new Redb layer
    pub async fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        info!("🟠 Initializing Redb Layer 3 (Warm Data Storage)");
        
        let db = Database::create(path)
            .map_err(|e| anyhow!("Failed to create Redb database: {}", e))?;
        
        // Initialize table
        let write_txn = db.begin_write()
            .map_err(|e| anyhow!("Failed to begin write transaction: {}", e))?;
        {
            let _table = write_txn.open_table(MILITARY_TABLE)
                .map_err(|e| anyhow!("Failed to open table: {}", e))?;
        }
        write_txn.commit()
            .map_err(|e| anyhow!("Failed to commit table creation: {}", e))?;
        
        Ok(Self { db })
    }
    
    /// Insert key-value pair with military-grade zero-copy efficiency
    pub async fn insert(&self, key: &str, value: &[u8]) -> Result<()> {
        let write_txn = self.db.begin_write()
            .map_err(|e| anyhow!("Redb write transaction failed: {}", e))?;
        
        {
            let mut table = write_txn.open_table(MILITARY_TABLE)
                .map_err(|e| anyhow!("Redb table open failed: {}", e))?;
            
            table.insert(key, value)
                .map_err(|e| anyhow!("Redb INSERT failed: {}", e))?;
        }
        
        write_txn.commit()
            .map_err(|e| anyhow!("Redb commit failed: {}", e))?;
        
        Ok(())
    }
    
    /// Remove key with military-grade confirmation
    pub async fn remove(&self, key: &str) -> Result<bool> {
        let write_txn = self.db.begin_write()
            .map_err(|e| anyhow!("Redb write transaction failed: {}", e))?;
        
        let removed = {
            let mut table = write_txn.open_table(MILITARY_TABLE)
                .map_err(|e| anyhow!("Redb table open failed: {}", e))?;
            
            let result = table.remove(key)
                .map_err(|e| anyhow!("Redb REMOVE failed: {}", e))?;
            result.is_some()
        };
        
        write_txn.commit()
            .map_err(|e| anyhow!("Redb commit failed: {}", e))?;
        
        Ok(removed)
    }
    
    /// Get all keys (for military-grade auditing)
    pub async fn get_all_keys(&self) -> Result<Vec<String>> {
        let read_txn = self.db.begin_read()
            .map_err(|e| anyhow!("Redb read transaction failed: {}", e))?;
        
        let table = read_txn.open_table(MILITARY_TABLE)
            .map_err(|e| anyhow!("Redb table open failed: {}", e))?;
        
        let mut keys = Vec::new();
        let iter = table.iter()
            .map_err(|e| anyhow!("Redb iteration failed: {}", e))?;
        
        for result in iter {
            match result {
                Ok((key, _)) => {
                    keys.push(key.value().to_string());
                },
                Err(e) => {
                    error!("Error iterating Redb keys: {}", e);
                    return Err(anyhow!("Redb iteration failed: {}", e));
                }
            }
        }
        
        Ok(keys)
    }
    
    /// Compact database for military-grade efficiency
    pub async fn compact(&mut self) -> Result<()> {
        self.db.compact()
            .map_err(|e| anyhow!("Redb compact failed: {}", e))?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl MilitaryStorageLayer for RedbLayer {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let read_txn = self.db.begin_read()
            .map_err(|e| anyhow!("Redb read transaction failed: {}", e))?;
        
        let table = read_txn.open_table(MILITARY_TABLE)
            .map_err(|e| anyhow!("Redb table open failed: {}", e))?;
        
        let result = match table.get(key) {
            Ok(Some(value)) => {
                let data = value.value().to_vec();
                Ok(Some(data))
            },
            Ok(None) => Ok(None),
            Err(e) => {
                error!("Redb GET failed for key '{}': {}", key, e);
                Err(anyhow!("Redb GET failed: {}", e))
            }
        };
        result
    }
    
    async fn put(&self, key: &str, value: &[u8]) -> Result<()> {
        self.insert(key, value).await
    }
    
    async fn delete(&self, key: &str) -> Result<()> {
        self.remove(key).await.map(|_| ())
    }
    
    async fn exists(&self, key: &str) -> Result<bool> {
        let read_txn = self.db.begin_read()
            .map_err(|e| anyhow!("Redb read transaction failed: {}", e))?;
        
        let table = read_txn.open_table(MILITARY_TABLE)
            .map_err(|e| anyhow!("Redb table open failed: {}", e))?;
        
        let result = match table.get(key) {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => {
                error!("Redb EXISTS failed for key '{}': {}", key, e);
                Err(anyhow!("Redb EXISTS failed: {}", e))
            }
        };
        result
    }
    
    async fn health_check(&self) -> Result<()> {
        // Simple health check - try to begin a read transaction
        let _read_txn = self.db.begin_read()
            .map_err(|e| anyhow!("Redb health check failed: {}", e))?;
        Ok(())
    }
}
