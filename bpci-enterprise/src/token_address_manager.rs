//! Token/Address Management System with 4D Database Integration
//! 
//! Manages BPI OS connection tokens and addresses using the revolutionary 4D Hash-Graph Database

use serde::{Serialize, Deserialize};
use serde_json;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::{Result, anyhow};
use crate::storage::{FourDHashGraphKernel, FourDConfig, SecurityLevel};

/// Connection status for token/address pairs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionStatus {
    Active,
    Inactive,
    Suspended,
    Expired,
}

/// mDNS proxy configuration for BPI addresses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdnsProxyConfig {
    pub service_name: String,
    pub domain: String,
    pub port: u16,
    pub txt_records: HashMap<String, String>,
    pub enabled: bool,
}

/// Security metadata for token/address entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetadata {
    pub security_level: SecurityLevel,
    pub merkle_hash_ref: Option<String>,
    pub access_control: Vec<String>,
    pub audit_refs: Vec<String>,
    pub encrypted: bool,
}

/// Token/Address entry in the 4D database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenAddressEntry {
    pub id: Uuid,
    pub token: String,
    pub address: String,
    pub name: String,
    pub description: Option<String>,
    pub user_id: String,
    pub status: ConnectionStatus,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub mdns_config: Option<MdnsProxyConfig>,
    pub security_metadata: SecurityMetadata,
}

/// Statistics for token/address management
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenAddressStats {
    pub total_entries: u64,
    pub active_connections: u64,
    pub inactive_connections: u64,
    pub suspended_connections: u64,
    pub expired_connections: u64,
    pub total_users: u64,
    pub mdns_enabled_count: u64,
    pub last_activity: Option<DateTime<Utc>>,
}

/// Token/Address Manager using 4D Database with in-memory cache
#[derive(Debug)]
pub struct TokenAddressManager {
    four_d_db: FourDHashGraphKernel,
    collection_name: String,
    /// In-memory token cache to work around 4D database query limitations
    token_cache: Arc<RwLock<HashMap<String, TokenAddressEntry>>>,
}

impl TokenAddressManager {
    /// Create new token/address manager with 4D database
    pub async fn new(config: FourDConfig) -> Result<Self> {
        let four_d_db = FourDHashGraphKernel::new(config).await?;
        let collection_name = "bpi_token_addresses".to_string();
        let token_cache = Arc::new(RwLock::new(HashMap::new()));
        
        Ok(Self {
            four_d_db,
            collection_name,
            token_cache,
        })
    }
    
    /// Store token/address entry in 4D database and in-memory cache
    pub async fn store_token_address(&self, entry: TokenAddressEntry) -> Result<String> {
        // Store in in-memory cache first for reliable retrieval
        let token_id = entry.id.to_string();
        {
            let mut cache = self.token_cache.write().await;
            cache.insert(token_id.clone(), entry.clone());
            println!("🔍 DEBUG: Token stored in cache with ID: {}", token_id);
        }
        
        // Also store in 4D database for persistence
        let mut document = serde_json::to_value(&entry)?;
        
        // Ensure the UUID is stored as both 'id' and '_id' for reliable retrieval
        if let Some(obj) = document.as_object_mut() {
            obj.insert("_id".to_string(), serde_json::Value::String(entry.id.to_string()));
            obj.insert("token_uuid".to_string(), serde_json::Value::String(entry.id.to_string()));
        }
        
        println!("🔍 DEBUG: Storing document with enhanced indexing: {}", document);
        let doc_id = self.four_d_db.insert_document(&self.collection_name, document).await?;
        println!("🔍 DEBUG: Document stored in 4D DB with ID: {}", doc_id);
        Ok(doc_id)
    }
    
    /// Retrieve token/address entry by ID (cache first, then 4D database)
    pub async fn get_token_address(&self, id: &Uuid) -> Result<Option<TokenAddressEntry>> {
        println!("🔍 DEBUG: get_token_address searching for ID: {}", id);
        let id_str = id.to_string();
        
        // First, try the in-memory cache for fast retrieval
        {
            let cache = self.token_cache.read().await;
            if let Some(entry) = cache.get(&id_str) {
                println!("🔍 DEBUG: Token found in cache! ID: {}", id_str);
                return Ok(Some(entry.clone()));
            }
            println!("🔍 DEBUG: Token not found in cache, trying 4D database...");
        }
        
        // Fallback to 4D database with multiple query strategies
        let queries = vec![
            serde_json::json!({"id": id_str}),
            serde_json::json!({"_id": id_str}),
            serde_json::json!({"token_uuid": id_str}),
        ];
        
        for (i, query) in queries.iter().enumerate() {
            println!("🔍 DEBUG: Trying 4D DB query strategy {}: {}", i + 1, query);
            
            let result = self.four_d_db.find_documents(&self.collection_name, query.clone(), Some(1)).await?;
            println!("🔍 DEBUG: 4D DB Query {} found {} documents", i + 1, result.documents.len());
            
            if !result.documents.is_empty() {
                println!("🔍 DEBUG: Successfully found token in 4D DB with query strategy {}", i + 1);
                // Convert HashMap to serde_json::Value properly
                let doc_value = serde_json::to_value(&result.documents[0])?;
                let entry: TokenAddressEntry = serde_json::from_value(doc_value)?;
                
                // Store in cache for future fast retrieval
                {
                    let mut cache = self.token_cache.write().await;
                    cache.insert(id_str.clone(), entry.clone());
                    println!("🔍 DEBUG: Token cached for future retrieval: {}", id_str);
                }
                
                return Ok(Some(entry));
            }
        }
        
        println!("🔍 DEBUG: No documents found for ID: {} in cache or 4D database", id);
        Ok(None)
    }
    
    /// List all token/address entries for a user
    pub async fn list_user_tokens(&self, user_id: &str) -> Result<Vec<TokenAddressEntry>> {
        let query = serde_json::json!({
            "user_id": user_id
        });
        
        let result = self.four_d_db.find_documents(&self.collection_name, query, None).await?;
        
        let mut entries = Vec::new();
        for doc in result.documents {
            // Convert HashMap to serde_json::Value properly
            let doc_value = serde_json::to_value(&doc)?;
            let entry: TokenAddressEntry = serde_json::from_value(doc_value)?;
            entries.push(entry);
        }
        
        Ok(entries)
    }
    
    /// Update connection status
    pub async fn update_status(&self, id: &Uuid, status: ConnectionStatus) -> Result<bool> {
        let query = serde_json::json!({
            "id": id.to_string()
        });
        
        let update = serde_json::json!({
            "$set": {
                "status": status,
                "last_used": Utc::now()
            }
        });
        
        let result = self.four_d_db.update_document(&self.collection_name, query, update).await?;
        Ok(result > 0)
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
        
        // Convert HashMap to serde_json::Value properly
        let doc_value = serde_json::to_value(&result.documents[0])?;
        let entry: TokenAddressEntry = serde_json::from_value(doc_value)?;
        
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
        
        // Convert HashMap to serde_json::Value properly
        let doc_value = serde_json::to_value(&result.documents[0])?;
        let entry: TokenAddressEntry = serde_json::from_value(doc_value)?;
        
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
            // Convert HashMap to serde_json::Value properly
            let doc_value = serde_json::to_value(&doc)?;
            let entry: TokenAddressEntry = serde_json::from_value(doc_value)?;
            entries.push(entry);
        }
        
        Ok(entries)
    }
    
    /// Delete token/address entry
    pub async fn delete_token_address(&self, id: &Uuid) -> Result<bool> {
        let query = serde_json::json!({
            "id": id.to_string()
        });
        
        // Note: FourDHashGraphKernel doesn't have delete_document, using update with deletion flag
        let update = serde_json::json!({
            "$set": {
                "deleted": true,
                "deleted_at": chrono::Utc::now()
            }
        });
        let result = self.four_d_db.update_document(&self.collection_name, query, update).await?;
        Ok(result > 0)
    }
    
    /// Get statistics for token/address management
    pub async fn get_stats(&self) -> TokenAddressStats {
        // Get all documents to calculate stats
        let all_query = serde_json::json!({});
        let all_result = self.four_d_db.find_documents(&self.collection_name, all_query, None).await
            .unwrap_or_else(|_| crate::storage::QueryResult {
                documents: Vec::new(),
                tiles_accessed: Vec::new(),
                query_time_ms: 0,
                total_results: 0,
            });
        
        let mut stats = TokenAddressStats::default();
        stats.total_entries = all_result.documents.len() as u64;
        
        let mut users = std::collections::HashSet::new();
        let mut last_activity: Option<DateTime<Utc>> = None;
        
        for doc in all_result.documents {
            if let Ok(doc_value) = serde_json::to_value(&doc) {
                if let Ok(entry) = serde_json::from_value::<TokenAddressEntry>(doc_value) {
                    users.insert(entry.user_id);
                    
                    match entry.status {
                        ConnectionStatus::Active => stats.active_connections += 1,
                        ConnectionStatus::Inactive => stats.inactive_connections += 1,
                        ConnectionStatus::Suspended => stats.suspended_connections += 1,
                        ConnectionStatus::Expired => stats.expired_connections += 1,
                    }
                    
                    if entry.mdns_config.is_some() {
                        stats.mdns_enabled_count += 1;
                    }
                    
                    if let Some(used) = entry.last_used {
                        if last_activity.is_none() || used > last_activity.unwrap() {
                            last_activity = Some(used);
                        }
                    }
                }
            }
        }
        
        stats.total_users = users.len() as u64;
        stats.last_activity = last_activity;
        
        stats
    }
    
    /// Health check for token/address manager
    pub async fn health_check(&self) -> Result<bool> {
        // Test basic database connectivity
        let test_query = serde_json::json!({});
        let _result = self.four_d_db.find_documents(&self.collection_name, test_query, Some(1)).await?;
        Ok(true)
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
            name: "Test Entry".to_string(),
            description: Some("Test description".to_string()),
            user_id: "test_user".to_string(),
            status: ConnectionStatus::Active,
            created_at: Utc::now(),
            last_used: None,
            mdns_config: None,
            security_metadata: SecurityMetadata {
                security_level: SecurityLevel::Internal,
                merkle_hash_ref: None,
                access_control: vec!["test_user".to_string()],
                audit_refs: vec![],
                encrypted: false,
            },
        };
        
        let doc_id = manager.store_token_address(entry.clone()).await.unwrap();
        assert!(!doc_id.is_empty());
        
        let retrieved = manager.get_token_address(&entry.id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().token, "test_token_123");
    }
}
