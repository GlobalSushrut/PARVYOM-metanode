//! Integrated Token/Address Management System
//! 
//! Combines the 4D Database, Merkle Secret Hashing, and mDNS Proxy systems
//! for comprehensive BPI token/address management with security and networking

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Import our separate modules
use crate::token_address_manager::{
    TokenAddressManager, TokenAddressEntry, ConnectionStatus, 
    MdnsProxyConfig as TokenMdnsConfig, SecurityMetadata
};
use crate::merkle_secret_hasher::{
    MerkleSecretHasher, MerkleProof, MerkleHasherStats
};
use crate::mdns_proxy_manager::{
    MdnsProxyManager, MdnsProxyConfig, MdnsServiceRecord, MdnsProxyStats
};
use crate::storage::FourDConfig;

/// Integrated system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegratedTokenSystemConfig {
    /// 4D Database configuration
    pub four_d_config: FourDConfig,
    
    /// Merkle hashing configuration
    pub merkle_master_salt: String,
    
    /// mDNS proxy configuration
    pub mdns_config: MdnsProxyConfig,
    
    /// Enable automatic Merkle tree generation
    pub auto_merkle_trees: bool,
    
    /// Enable automatic mDNS registration
    pub auto_mdns_registration: bool,
    
    /// Security level requirements
    pub min_security_level: String,
}

/// Complete token/address information with all security and networking data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteTokenInfo {
    /// Basic token/address entry
    pub entry: TokenAddressEntry,
    
    /// Merkle hash for security verification
    pub merkle_hash: String,
    
    /// Merkle proof for verification
    pub merkle_proof: Option<MerkleProof>,
    
    /// mDNS service record for networking
    pub mdns_record: Option<MdnsServiceRecord>,
    
    /// Network discovery status
    pub network_discoverable: bool,
    
    /// Last security verification
    pub last_verified: Option<DateTime<Utc>>,
}

/// System-wide statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegratedSystemStats {
    /// Token/address database stats
    pub database_stats: crate::token_address_manager::TokenAddressStats,
    
    /// Merkle hashing stats
    pub merkle_stats: MerkleHasherStats,
    
    /// mDNS proxy stats
    pub mdns_stats: MdnsProxyStats,
    
    /// Integration stats
    pub total_integrated_tokens: u64,
    pub successful_verifications: u64,
    pub network_discoveries: u64,
    pub last_operation: Option<DateTime<Utc>>,
}

/// Integrated Token/Address Management System
#[derive(Debug)]
pub struct IntegratedTokenSystem {
    /// Token/address database manager
    token_manager: Arc<TokenAddressManager>,
    
    /// Merkle secret hasher
    merkle_hasher: Arc<MerkleSecretHasher>,
    
    /// mDNS proxy manager
    mdns_manager: Arc<MdnsProxyManager>,
    
    /// System configuration
    config: Arc<RwLock<IntegratedTokenSystemConfig>>,
    
    /// Integration statistics
    stats: Arc<RwLock<IntegratedSystemStats>>,
}

impl Default for IntegratedTokenSystemConfig {
    fn default() -> Self {
        Self {
            four_d_config: FourDConfig::default(),
            merkle_master_salt: "bpci_enterprise_master_salt_2024".to_string(),
            mdns_config: MdnsProxyConfig::default(),
            auto_merkle_trees: true,
            auto_mdns_registration: true,
            min_security_level: "Internal".to_string(),
        }
    }
}

impl IntegratedTokenSystem {
    /// Create new integrated token/address management system
    pub async fn new(config: IntegratedTokenSystemConfig) -> Result<Self> {
        // Initialize token/address manager with 4D database
        let token_manager = Arc::new(
            TokenAddressManager::new(config.four_d_config.clone()).await?
        );
        
        // Initialize Merkle secret hasher
        let merkle_hasher = Arc::new(
            MerkleSecretHasher::new(config.merkle_master_salt.clone())
        );
        
        // Initialize mDNS proxy manager
        let mdns_manager = Arc::new(
            MdnsProxyManager::new(config.mdns_config.clone())
        );
        
        // Start mDNS service if enabled
        if config.mdns_config.enabled {
            mdns_manager.start_service().await?;
        }
        
        let stats = IntegratedSystemStats {
            database_stats: crate::token_address_manager::TokenAddressStats::default(),
            merkle_stats: MerkleHasherStats::default(),
            mdns_stats: MdnsProxyStats::default(),
            total_integrated_tokens: 0,
            successful_verifications: 0,
            network_discoveries: 0,
            last_operation: None,
        };
        
        Ok(Self {
            token_manager,
            merkle_hasher,
            mdns_manager,
            config: Arc::new(RwLock::new(config)),
            stats: Arc::new(RwLock::new(stats)),
        })
    }
    
    /// Create complete token/address with full integration
    pub async fn create_integrated_token(
        &self,
        token: String,
        address: String,
        name: String,
        description: Option<String>,
        user_id: String,
        enable_mdns: bool,
        mdns_port: Option<u16>,
    ) -> Result<CompleteTokenInfo> {
        // 1. Generate Merkle hash for security
        let merkle_hash = self.merkle_hasher
            .hash_token_address(&token, &address, &user_id)
            .await?;
        
        // 2. Create token/address entry
        let mut entry = TokenAddressEntry {
            id: Uuid::new_v4(),
            token: token.clone(),
            address: address.clone(),
            name: name.clone(),
            description: description.clone(),
            user_id: user_id.clone(),
            status: ConnectionStatus::Active,
            created_at: Utc::now(),
            last_used: None,
            mdns_config: None,
            security_metadata: SecurityMetadata {
                security_level: crate::storage::SecurityLevel::Internal,
                merkle_hash_ref: Some(merkle_hash.clone()),
                access_control: vec![user_id.clone()],
                audit_refs: vec![],
                encrypted: true,
            },
        };
        
        // 3. Register mDNS proxy if enabled
        let mut mdns_record = None;
        if enable_mdns {
            let config = self.config.read().await;
            if config.auto_mdns_registration {
                let port = mdns_port.unwrap_or(8080);
                let service_name = format!("bpi-{}", entry.id.to_string().split('-').next().unwrap_or("node"));
                
                let mut txt_records = HashMap::new();
                txt_records.insert("bpi_address".to_string(), address.clone());
                txt_records.insert("user_id".to_string(), user_id.clone());
                txt_records.insert("created".to_string(), Utc::now().to_rfc3339());
                txt_records.insert("version".to_string(), "1.0".to_string());
                
                let full_name = self.mdns_manager
                    .register_bpi_address(&address, &service_name, port, txt_records)
                    .await?;
                
                // Get the registered record
                mdns_record = self.mdns_manager.resolve_bpi_address(&address).await?;
                
                // Update entry with mDNS config
                entry.mdns_config = Some(TokenMdnsConfig {
                    service_name: service_name.clone(),
                    domain: "local".to_string(),
                    port,
                    txt_records: HashMap::new(),
                    enabled: true,
                });
            }
        }
        
        // 4. Store in 4D database
        println!("🔍 DEBUG: Storing token with entry.id: {}", entry.id);
        let _doc_id = self.token_manager.store_token_address(entry.clone()).await?;
        println!("🔍 DEBUG: Token stored successfully, will use entry.id for retrieval: {}", entry.id);
        
        // 5. Generate Merkle proof for verification
        // First, ensure the tree exists by creating it with the current token
        let tree_id = format!("user_tokens_{}", user_id);
        let tree_data = vec![merkle_hash.clone()];
        let _root_hash = self.merkle_hasher
            .create_merkle_tree(&tree_id, tree_data)
            .await?;
        
        // Now generate the proof
        let merkle_proof = self.merkle_hasher
            .generate_proof(&tree_id, &merkle_hash)
            .await
            .ok();
        
        // 6. Create complete token info
        let complete_info = CompleteTokenInfo {
            entry,
            merkle_hash,
            merkle_proof,
            mdns_record,
            network_discoverable: enable_mdns,
            last_verified: Some(Utc::now()),
        };
        
        // 7. Update statistics
        self.update_stats_create().await;
        
        Ok(complete_info)
    }
    
    /// Retrieve complete token information with verification
    pub async fn get_complete_token_info(&self, token_id: &Uuid) -> Result<Option<CompleteTokenInfo>> {
        println!("🔍 DEBUG: get_complete_token_info called with ID: {}", token_id);
        // Get basic token/address entry
        let entry = match self.token_manager.get_token_address(token_id).await? {
            Some(entry) => {
                println!("🔍 DEBUG: Token entry found in database");
                entry
            },
            None => {
                println!("🔍 DEBUG: Token entry NOT found in database");
                return Ok(None);
            },
        };
        
        // Get Merkle hash from security metadata
        let merkle_hash = entry.security_metadata.merkle_hash_ref
            .clone()
            .unwrap_or_default();
        
        // Generate fresh Merkle proof for verification
        let tree_id = format!("user_tokens_{}", entry.user_id);
        let merkle_proof = self.merkle_hasher
            .generate_proof(&tree_id, &merkle_hash)
            .await
            .ok();
        
        // Get mDNS record if available
        let mdns_record = self.mdns_manager
            .resolve_bpi_address(&entry.address)
            .await?;
        
        let complete_info = CompleteTokenInfo {
            entry,
            merkle_hash,
            merkle_proof,
            mdns_record: mdns_record.clone(),
            network_discoverable: mdns_record.is_some(),
            last_verified: Some(Utc::now()),
        };
        
        Ok(Some(complete_info))
    }
    
    /// Verify token/address integrity using Merkle proof
    pub async fn verify_token_integrity(&self, complete_info: &CompleteTokenInfo) -> Result<bool> {
        println!("🔍 DEBUG: verify_token_integrity called");
        println!("   - Has merkle_proof: {}", complete_info.merkle_proof.is_some());
        
        // Verify Merkle proof if available
        if let Some(proof) = &complete_info.merkle_proof {
            println!("   - Using merkle proof verification");
            let is_valid = self.merkle_hasher.verify_proof(proof).await?;
            
            if is_valid {
                self.update_stats_verify_success().await;
            }
            
            return Ok(is_valid);
        }
        
        println!("   - Using fallback hash comparison");
        // Fallback: regenerate hash and compare
        let regenerated_hash = self.merkle_hasher
            .hash_token_address(
                &complete_info.entry.token,
                &complete_info.entry.address,
                &complete_info.entry.user_id,
            )
            .await?;
        
        println!("   - Original hash: {}", complete_info.merkle_hash);
        println!("   - Regenerated hash: {}", regenerated_hash);
        
        let is_valid = regenerated_hash == complete_info.merkle_hash;
        
        println!("   - Hash comparison result: {}", is_valid);
        
        if is_valid {
            self.update_stats_verify_success().await;
        }
        
        Ok(is_valid)
    }
    
    /// Discover BPI services on the network
    pub async fn discover_network_services(&self) -> Result<Vec<MdnsServiceRecord>> {
        let services = self.mdns_manager.discover_bpi_services().await?;
        self.update_stats_discovery().await;
        Ok(services)
    }
    
    /// List all user tokens with complete information
    pub async fn list_user_complete_tokens(&self, user_id: &str) -> Result<Vec<CompleteTokenInfo>> {
        let entries = self.token_manager.list_user_tokens(user_id).await?;
        let mut complete_tokens = Vec::new();
        
        for entry in entries {
            if let Some(complete_info) = self.get_complete_token_info(&entry.id).await? {
                complete_tokens.push(complete_info);
            }
        }
        
        Ok(complete_tokens)
    }
    
    /// Update token status across all systems
    pub async fn update_token_status(&self, token_id: &Uuid, status: ConnectionStatus) -> Result<bool> {
        let updated = self.token_manager.update_status(token_id, status).await?;
        
        if updated {
            self.update_stats_update().await;
        }
        
        Ok(updated)
    }
    
    /// Get comprehensive system statistics
    pub async fn get_system_stats(&self) -> Result<IntegratedSystemStats> {
        let mut stats = self.stats.read().await.clone();
        
        // Update with current stats from each subsystem
        stats.database_stats = self.token_manager.get_stats().await;
        stats.merkle_stats = self.merkle_hasher.get_stats().await;
        stats.mdns_stats = self.mdns_manager.get_stats().await;
        
        Ok(stats)
    }
    
    /// Health check for entire integrated system
    pub async fn health_check(&self) -> Result<HashMap<String, bool>> {
        let mut health = HashMap::new();
        
        health.insert("token_manager".to_string(), 
            self.token_manager.health_check().await?);
        health.insert("merkle_hasher".to_string(), 
            self.merkle_hasher.health_check().await?);
        health.insert("mdns_manager".to_string(), 
            self.mdns_manager.health_check().await?);
        
        Ok(health)
    }
    
    // Private helper methods
    
    async fn update_stats_create(&self) {
        let mut stats = self.stats.write().await;
        stats.total_integrated_tokens += 1;
        stats.last_operation = Some(Utc::now());
    }
    
    async fn update_stats_verify_success(&self) {
        let mut stats = self.stats.write().await;
        stats.successful_verifications += 1;
        stats.last_operation = Some(Utc::now());
    }
    
    async fn update_stats_discovery(&self) {
        let mut stats = self.stats.write().await;
        stats.network_discoveries += 1;
        stats.last_operation = Some(Utc::now());
    }
    
    async fn update_stats_update(&self) {
        let mut stats = self.stats.write().await;
        stats.last_operation = Some(Utc::now());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_integrated_system_creation() {
        let config = IntegratedTokenSystemConfig::default();
        let system = IntegratedTokenSystem::new(config).await;
        assert!(system.is_ok());
    }
    
    #[tokio::test]
    async fn test_create_integrated_token() {
        let config = IntegratedTokenSystemConfig::default();
        let system = IntegratedTokenSystem::new(config).await.unwrap();
        
        let complete_info = system.create_integrated_token(
            "test_token_123".to_string(),
            "bpi_test_address_456".to_string(),
            "Test Integration".to_string(),
            Some("Test description".to_string()),
            "test_user".to_string(),
            true, // enable mDNS
            Some(8080),
        ).await.unwrap();
        
        assert_eq!(complete_info.entry.token, "test_token_123");
        assert_eq!(complete_info.entry.address, "bpi_test_address_456");
        assert!(!complete_info.merkle_hash.is_empty());
        assert!(complete_info.network_discoverable);
    }
    
    #[tokio::test]
    async fn test_verify_token_integrity() {
        let config = IntegratedTokenSystemConfig::default();
        let system = IntegratedTokenSystem::new(config).await.unwrap();
        
        let complete_info = system.create_integrated_token(
            "verify_token_789".to_string(),
            "bpi_verify_address_012".to_string(),
            "Verify Test".to_string(),
            None,
            "verify_user".to_string(),
            false, // disable mDNS for this test
            None,
        ).await.unwrap();
        
        let is_valid = system.verify_token_integrity(&complete_info).await.unwrap();
        assert!(is_valid);
    }
}
