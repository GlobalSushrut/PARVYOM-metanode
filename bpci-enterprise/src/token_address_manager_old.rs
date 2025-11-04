//! Token and Address Management System
//! 
//! Uses the revolutionary 4D Hash-Graph Database for token/address storage
//! with separate Merkle hashing and mDNS proxy modules for security and networking

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::{Result, anyhow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use blake3::Hash;

// Import 4D Database components
use crate::storage::{
    FourDHashGraphKernel, FourDConfig, FourDCoordinate,
    SecurityLevel, QueryResult
};

/// Token/Address entry in the 4D database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenAddressEntry {
    /// Unique identifier
    pub id: Uuid,
    
    /// BPI connection token
    pub token: String,
    
    /// BPI address
    pub address: String,
    
    /// Connection name/description
    pub name: String,
    
    /// Optional description
    pub description: Option<String>,
    
    /// User ID who owns this token/address
    pub user_id: String,
    
    /// Status of the connection
    pub status: ConnectionStatus,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last used timestamp
    pub last_used: Option<DateTime<Utc>>,
    
    /// mDNS proxy configuration
    pub mdns_config: Option<MdnsProxyConfig>,
    
    /// Security metadata
    pub security_metadata: SecurityMetadata,
}

/// Connection status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionStatus {
    Active,
    Inactive,
    Suspended,
    Revoked,
}

/// mDNS proxy configuration for BPI addresses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdnsProxyConfig {
    /// mDNS service name
    pub service_name: String,
    
    /// mDNS domain
    pub domain: String,
    
    /// Port for mDNS service
    pub port: u16,
    
    /// TXT records for additional metadata
    pub txt_records: HashMap<String, String>,
    
    /// Proxy enabled status
    pub enabled: bool,
}

/// Security metadata for token/address entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetadata {
    /// Security level classification
    pub security_level: SecurityLevel,
    
    /// Merkle tree hash reference (stored separately)
    pub merkle_hash_ref: Option<String>,
    
    /// Access control list
    pub access_control: Vec<String>,
    
    /// Audit trail references
    pub audit_refs: Vec<String>,
    
    /// Encryption status
    pub encrypted: bool,
}

/// Token/Address Manager using 4D Database
#[derive(Debug)]
pub struct TokenAddressManager {
    /// 4D Hash-Graph Database kernel
    four_d_db: Arc<FourDHashGraphKernel>,
    
    /// Collection name for token/address entries
    collection_name: String,
    
    /// Statistics
    stats: Arc<RwLock<TokenAddressStats>>,
}

/// Statistics for token/address management
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenAddressStats {
    pub total_entries: u64,
    pub active_connections: u64,
    pub inactive_connections: u64,
    pub total_mdns_proxies: u64,
    pub total_operations: u64,
    pub last_operation: Option<DateTime<Utc>>,
}

impl TokenAddressManager {
    /// Create new token/address manager with 4D database
    pub async fn new(four_d_config: FourDConfig) -> Result<Self> {
        let four_d_db = Arc::new(FourDHashGraphKernel::new(four_d_config).await?);
        
        Ok(Self {
            four_d_db,
            collection_name: "bpi_token_addresses".to_string(),
            stats: Arc::new(RwLock::new(TokenAddressStats::default())),
        })
    }
    
    /// Store token/address entry in 4D database
    pub async fn store_token_address(&self, entry: TokenAddressEntry) -> Result<String> {
        // Convert entry to JSON for 4D database storage
        let document = serde_json::to_value(&entry)?;
        
        // Store in 4D database
        let doc_id = self.four_d_db.insert_document(&self.collection_name, document).await?;
        
        // Update statistics
        self.update_stats_after_insert().await;
        
        Ok(doc_id)
    }
    
    /// Retrieve token/address entry by ID
    pub async fn get_token_address(&self, id: &Uuid) -> Result<Option<TokenAddressEntry>> {
        let query = serde_json::json!({
            "id": id.to_string()
        });
        
        let result = self.four_d_db.find_documents(&self.collection_name, query, Some(1)).await?;
        
        if result.documents.is_empty() {
            return Ok(None);
        }
        
        let entry: TokenAddressEntry = serde_json::from_value(
            serde_json::Value::Object(result.documents[0].clone())
        )?;
        
        Ok(Some(entry))
    }
    
    /// List all token/address entries for a user
    pub async fn list_user_tokens(&self, user_id: &str) -> Result<Vec<TokenAddressEntry>> {
        let query = serde_json::json!({
            "user_id": user_id
        });
        
        let result = self.four_d_db.find_documents(&self.collection_name, query, None).await?;
        
        let mut entries = Vec::new();
        for doc in result.documents {
            let entry: TokenAddressEntry = serde_json::from_value(
                serde_json::Value::Object(doc)
            )?;
            entries.push(entry);
        }
        
        Ok(entries)
    }
    
    /// Update token/address entry status
    pub async fn update_status(&self, id: &Uuid, status: ConnectionStatus) -> Result<bool> {
        let filter = serde_json::json!({
            "id": id.to_string()
        });
        
        let update = serde_json::json!({
            "$set": {
                "status": status,
                "last_used": Utc::now()
            }
        });
        
        let updated_count = self.four_d_db.update_document(&self.collection_name, filter, update).await?;
        
        if updated_count > 0 {
            self.update_stats_after_update().await;
        }
        
        Ok(updated_count > 0)
    }
    
    /// Find token/address by BPI address
    pub async fn find_by_address(&self, address: &str) -> Result<Option<TokenAddressEntry>> {
        let query = serde_json::json!({
            "address": address
        });
        
        let result = self.four_d_db.find_documents(&self.collection_name, query, Some(1)).await?;
        
        if result.documents.is_empty() {
            return Ok(None);
        }
        
        let entry: TokenAddressEntry = serde_json::from_value(
            serde_json::Value::Object(result.documents[0].clone())
        )?;
        
        Ok(Some(entry))
    }
    
    /// Find token/address by connection token
    pub async fn find_by_token(&self, token: &str) -> Result<Option<TokenAddressEntry>> {
        let query = serde_json::json!({
            "token": token
        });
        
        let result = self.four_d_db.find_documents(&self.collection_name, query, Some(1)).await?;
        
        if result.documents.is_empty() {
            return Ok(None);
        }
        
        let entry: TokenAddressEntry = serde_json::from_value(
            serde_json::Value::Object(result.documents[0].clone())
        )?;
        
        Ok(Some(entry))
    }
    
    /// Get all active connections
    pub async fn get_active_connections(&self) -> Result<Vec<TokenAddressEntry>> {
        let query = serde_json::json!({
            "status": "Active"
        });
        
        let result = self.four_d_db.find_documents(&self.collection_name, query, None).await?;
        
        let mut entries = Vec::new();
        for doc in result.documents {
            let entry: TokenAddressEntry = serde_json::from_value(
                serde_json::Value::Object(doc)
            )?;
            entries.push(entry);
        }
        
        Ok(entries)
    }
    
    /// Get statistics
    pub async fn get_stats(&self) -> TokenAddressStats {
        self.stats.read().await.clone()
    }
    
    /// Health check for the token/address manager
    pub async fn health_check(&self) -> Result<bool> {
        // Check 4D database health
        self.four_d_db.health_check().await
    }
    
    // Private helper methods
    
    async fn update_stats_after_insert(&self) {
        let mut stats = self.stats.write().await;
        stats.total_entries += 1;
        stats.total_operations += 1;
        stats.last_operation = Some(Utc::now());
    }
    
    async fn update_stats_after_update(&self) {
        let mut stats = self.stats.write().await;
        stats.total_operations += 1;
        stats.last_operation = Some(Utc::now());
    }
}

impl Default for MdnsProxyConfig {
    fn default() -> Self {
        Self {
            service_name: "bpi-node".to_string(),
            domain: "local".to_string(),
            port: 8080,
            txt_records: HashMap::new(),
            enabled: false,
        }
    }
}

impl Default for SecurityMetadata {
    fn default() -> Self {
        Self {
            security_level: SecurityLevel::Internal,
            merkle_hash_ref: None,
            access_control: Vec::new(),
            audit_refs: Vec::new(),
            encrypted: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_token_address_manager_creation() {
        let config = FourDConfig::default();
        let manager = TokenAddressManager::new(config).await;
        assert!(manager.is_ok());
    }
    
    #[tokio::test]
    async fn test_store_and_retrieve_token_address() {
        let config = FourDConfig::default();
        let manager = TokenAddressManager::new(config).await.unwrap();
        
        let entry = TokenAddressEntry {
            id: Uuid::new_v4(),
            token: "test_token_123".to_string(),
            address: "bpi_test_address_456".to_string(),
            name: "Test Connection".to_string(),
            description: Some("Test description".to_string()),
            user_id: "test_user".to_string(),
            status: ConnectionStatus::Active,
            created_at: Utc::now(),
            last_used: None,
            mdns_config: None,
            security_metadata: SecurityMetadata::default(),
        };
        
        let doc_id = manager.store_token_address(entry.clone()).await.unwrap();
        assert!(!doc_id.is_empty());
        
        let retrieved = manager.get_token_address(&entry.id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().token, entry.token);
    }
}
