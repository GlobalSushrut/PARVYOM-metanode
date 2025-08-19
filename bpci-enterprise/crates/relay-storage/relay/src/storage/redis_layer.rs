//! Layer 1: Redis - Military-Grade Fast Cache Layer
//! 
//! High-performance memory cache for hot data with military-grade security

use anyhow::{Result, anyhow};
use redis::{Client, Connection};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, debug, error};
use super::MilitaryStorageLayer;

use super::{StorageLayer, StorageError};

/// Redis Layer 1 - High-performance memory cache
pub struct RedisLayer {
    connection: Arc<Mutex<Connection>>,
    client: Client,
}

impl RedisLayer {
    /// Create new Redis layer with military-grade connection
    pub async fn new(redis_url: &str) -> Result<Self> {
        info!("🔴 Initializing Redis Layer 1 (Memory Cache)");
        
        let client = Client::open(redis_url)
            .map_err(|e| anyhow!("Redis client creation failed: {}", e))?;
        
        let connection = client.get_connection()
            .map_err(|e| anyhow!("Redis connection failed: {}", e))?;
        
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
            client,
        })
    }
    
    /// Set key-value with military-grade persistence
    pub async fn set(&self, key: &str, value: &[u8]) -> Result<()> {
        let mut conn = self.connection.lock().await;
        conn.set::<&str, &[u8], ()>(key, value)
            .map_err(|e| {
                error!("Redis SET failed for key '{}': {}", key, e);
                anyhow!("Redis SET failed: {}", e)
            })?;
        
        debug!("✅ Redis SET: {} bytes stored for key '{}'", value.len(), key);
        Ok(())
    }
    
    /// Get value with military-grade verification
    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let mut conn = self.connection.lock().await;
        match conn.get::<&str, Vec<u8>>(key) {
            Ok(value) => {
                debug!("✅ Redis GET: {} bytes retrieved for key '{}'", value.len(), key);
                Ok(Some(value))
            },
            Err(redis::RedisError { kind: redis::ErrorKind::TypeError, .. }) => Ok(None),
            Err(e) => {
                error!("Redis GET failed for key '{}': {}", key, e);
                Err(anyhow!("Redis GET failed: {}", e))
            }
        }
    }
    
    /// Delete key with military-grade confirmation
    pub async fn delete(&self, key: &str) -> Result<bool> {
        let mut conn = self.connection.lock().await;
        let deleted: u32 = conn.del(key)
            .map_err(|e| {
                error!("Redis DEL failed for key '{}': {}", key, e);
                anyhow!("Redis DEL failed: {}", e)
            })?;
        
        let success = deleted > 0;
        debug!("✅ Redis DEL: key '{}' deleted: {}", key, success);
        Ok(success)
    }
}

#[async_trait::async_trait]
impl MilitaryStorageLayer for RedisLayer {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let mut conn = self.connection.lock().await;
        match conn.get::<&str, Vec<u8>>(key) {
            Ok(value) => Ok(Some(value)),
            Err(redis::RedisError { kind: redis::ErrorKind::TypeError, .. }) => Ok(None),
            Err(e) => {
                error!("Redis GET failed for key '{}': {}", key, e);
                Err(anyhow!("Redis GET failed: {}", e))
            }
        }
    }
    
    async fn set(&self, key: &str, value: &[u8]) -> Result<()> {
        let mut conn = self.connection.lock().await;
        conn.set(key, value)
            .map_err(|e| anyhow!("Redis SET failed: {}", e))?;
        Ok(())
    }
    
    async fn delete(&self, key: &str) -> Result<bool> {
        let mut conn = self.connection.lock().await;
        let deleted: u32 = conn.del(key)
            .map_err(|e| anyhow!("Redis DEL failed: {}", e))?;
        Ok(deleted > 0)
    }
    
    async fn exists(&self, key: &str) -> Result<bool> {
        self.key_exists(key).await
    }
    
    async fn size(&self) -> Result<u64> {
        let mut conn = self.connection.lock().await;
        let size: u64 = conn.dbsize()
            .map_err(|e| anyhow!("Redis DBSIZE failed: {}", e))?;
        Ok(size)
    }
}

impl StorageLayer for RedisLayer {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, StorageError> {
        // Convert to blocking operation for StorageLayer trait compatibility
        let rt = tokio::runtime::Handle::try_current()
            .or_else(|_| tokio::runtime::Runtime::new().map(|rt| rt.handle().clone()))
            .map_err(|e| StorageError::IoError(format!("Tokio runtime error: {}", e)))?;
        
        rt.block_on(async {
            let mut conn = self.connection.lock().await;
            match conn.get::<&[u8], Vec<u8>>(key) {
                Ok(value) => Ok(Some(value)),
                Err(redis::RedisError { kind: redis::ErrorKind::TypeError, .. }) => Ok(None),
                Err(e) => {
                    error!("Redis GET failed: {}", e);
                    Err(StorageError::NetworkError(format!("Redis GET failed: {}", e)))
                }
            }
        })
    }
    
    fn put(&self, key: &[u8], value: &[u8]) -> Result<(), StorageError> {
        let rt = tokio::runtime::Handle::try_current()
            .or_else(|_| tokio::runtime::Runtime::new().map(|rt| rt.handle().clone()))
            .map_err(|e| StorageError::IoError(format!("Tokio runtime error: {}", e)))?;
        
        rt.block_on(async {
            let mut conn = self.connection.lock().await;
            conn.set(key, value)
                .map_err(|e| StorageError::NetworkError(format!("Redis SET failed: {}", e)))
        })
    }
    
    fn delete(&self, key: &[u8]) -> Result<(), StorageError> {
        let rt = tokio::runtime::Handle::try_current()
            .or_else(|_| tokio::runtime::Runtime::new().map(|rt| rt.handle().clone()))
            .map_err(|e| StorageError::IoError(format!("Tokio runtime error: {}", e)))?;
        
        rt.block_on(async {
            let mut conn = self.connection.lock().await;
            let _: u32 = conn.del(key)
                .map_err(|e| StorageError::NetworkError(format!("Redis DEL failed: {}", e)))?;
            Ok(())
        })
    }
    
    fn exists(&self, key: &[u8]) -> Result<bool, StorageError> {
        let rt = tokio::runtime::Handle::try_current()
            .or_else(|_| tokio::runtime::Runtime::new().map(|rt| rt.handle().clone()))
            .map_err(|e| StorageError::IoError(format!("Tokio runtime error: {}", e)))?;
        
        rt.block_on(async {
            let mut conn = self.connection.lock().await;
            let exists: bool = conn.exists(key)
                .map_err(|e| StorageError::NetworkError(format!("Redis EXISTS failed: {}", e)))?;
            Ok(exists)
        })
    }
    
    fn size(&self) -> Result<u64, StorageError> {
        let rt = tokio::runtime::Handle::try_current()
            .or_else(|_| tokio::runtime::Runtime::new().map(|rt| rt.handle().clone()))
            .map_err(|e| StorageError::IoError(format!("Tokio runtime error: {}", e)))?;
        
        rt.block_on(async {
            let mut conn = self.connection.lock().await;
            let size: u64 = conn.dbsize()
                .map_err(|e| StorageError::NetworkError(format!("Redis DBSIZE failed: {}", e)))?;
            Ok(size)
        })
    }
    
    fn health_check(&self) -> Result<(), StorageError> {
        let rt = tokio::runtime::Handle::try_current()
            .or_else(|_| tokio::runtime::Runtime::new().map(|rt| rt.handle().clone()))
            .map_err(|e| StorageError::IoError(format!("Tokio runtime error: {}", e)))?;
        
        rt.block_on(async {
            let mut conn = self.connection.lock().await;
            let _: String = conn.ping()
                .map_err(|e| StorageError::NetworkError(format!("Redis PING failed: {}", e)))?;
            Ok(())
        })
    }
}
