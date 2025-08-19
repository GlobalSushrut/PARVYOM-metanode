// Shared Storage Layer
// Database abstractions shared between BPI Core and BPCI Enterprise

//! # Storage
//! 
//! Database abstractions providing consistent data storage
//! across both community and enterprise products.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
}

/// Storage backend trait
#[async_trait::async_trait]
pub trait StorageBackend: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError>;
    async fn put(&self, key: &str, value: Vec<u8>) -> Result<(), StorageError>;
    async fn delete(&self, key: &str) -> Result<(), StorageError>;
    async fn exists(&self, key: &str) -> Result<bool, StorageError>;
    async fn list_keys(&self, prefix: &str) -> Result<Vec<String>, StorageError>;
}

/// In-memory storage implementation
#[derive(Debug, Clone)]
pub struct MemoryStorage {
    data: std::sync::Arc<tokio::sync::RwLock<HashMap<String, Vec<u8>>>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        MemoryStorage {
            data: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl StorageBackend for MemoryStorage {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        let data = self.data.read().await;
        Ok(data.get(key).cloned())
    }

    async fn put(&self, key: &str, value: Vec<u8>) -> Result<(), StorageError> {
        let mut data = self.data.write().await;
        data.insert(key.to_string(), value);
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), StorageError> {
        let mut data = self.data.write().await;
        data.remove(key);
        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool, StorageError> {
        let data = self.data.read().await;
        Ok(data.contains_key(key))
    }

    async fn list_keys(&self, prefix: &str) -> Result<Vec<String>, StorageError> {
        let data = self.data.read().await;
        let keys: Vec<String> = data.keys()
            .filter(|k| k.starts_with(prefix))
            .cloned()
            .collect();
        Ok(keys)
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// Sled-based persistent storage
pub struct SledStorage {
    db: sled::Db,
}

impl SledStorage {
    pub fn new(path: &str) -> Result<Self, StorageError> {
        let db = sled::open(path)
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        Ok(SledStorage { db })
    }
}

#[async_trait::async_trait]
impl StorageBackend for SledStorage {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        let result = self.db.get(key.as_bytes())
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        Ok(result.map(|v| v.to_vec()))
    }

    async fn put(&self, key: &str, value: Vec<u8>) -> Result<(), StorageError> {
        self.db.insert(key.as_bytes(), value)
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), StorageError> {
        self.db.remove(key.as_bytes())
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool, StorageError> {
        let exists = self.db.contains_key(key.as_bytes())
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        Ok(exists)
    }

    async fn list_keys(&self, prefix: &str) -> Result<Vec<String>, StorageError> {
        let mut keys = Vec::new();
        for result in self.db.scan_prefix(prefix.as_bytes()) {
            let (key, _) = result.map_err(|e| StorageError::DatabaseError(e.to_string()))?;
            let key_str = String::from_utf8_lossy(&key).to_string();
            keys.push(key_str);
        }
        Ok(keys)
    }
}

/// Generic storage manager
pub struct StorageManager<T: StorageBackend> {
    backend: T,
}

impl<T: StorageBackend> StorageManager<T> {
    pub fn new(backend: T) -> Self {
        StorageManager { backend }
    }

    pub async fn store_json<V: Serialize>(&self, key: &str, value: &V) -> Result<(), StorageError> {
        let json = serde_json::to_vec(value)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;
        self.backend.put(key, json).await
    }

    pub async fn load_json<V: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<V>, StorageError> {
        match self.backend.get(key).await? {
            Some(data) => {
                let value = serde_json::from_slice(&data)
                    .map_err(|e| StorageError::SerializationError(e.to_string()))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    pub async fn delete(&self, key: &str) -> Result<(), StorageError> {
        self.backend.delete(key).await
    }

    pub async fn exists(&self, key: &str) -> Result<bool, StorageError> {
        self.backend.exists(key).await
    }

    pub async fn list_keys(&self, prefix: &str) -> Result<Vec<String>, StorageError> {
        self.backend.list_keys(prefix).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_storage() {
        let storage = MemoryStorage::new();
        
        // Test put and get
        storage.put("test_key", b"test_value".to_vec()).await.unwrap();
        let value = storage.get("test_key").await.unwrap().unwrap();
        assert_eq!(value, b"test_value");
        
        // Test exists
        assert!(storage.exists("test_key").await.unwrap());
        assert!(!storage.exists("nonexistent").await.unwrap());
        
        // Test delete
        storage.delete("test_key").await.unwrap();
        assert!(!storage.exists("test_key").await.unwrap());
    }

    #[tokio::test]
    async fn test_storage_manager() {
        let storage = MemoryStorage::new();
        let manager = StorageManager::new(storage);
        
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestData {
            id: Uuid,
            name: String,
        }
        
        let test_data = TestData {
            id: Uuid::new_v4(),
            name: "test".to_string(),
        };
        
        // Store JSON
        manager.store_json("test_json", &test_data).await.unwrap();
        
        // Load JSON
        let loaded: TestData = manager.load_json("test_json").await.unwrap().unwrap();
        assert_eq!(loaded, test_data);
    }
}
