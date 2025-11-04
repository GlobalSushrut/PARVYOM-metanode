//! mDNS Proxy Manager for BPI Addresses
//! 
//! Separate module for managing mDNS (Multicast DNS) proxies for BPI addresses
//! Enables network discovery and real address-like behavior for BPI connections

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::{Result, anyhow};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use chrono::{DateTime, Utc};

/// mDNS service record for BPI address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdnsServiceRecord {
    /// Service name (e.g., "bpi-node-production")
    pub service_name: String,
    
    /// Service type (e.g., "_bpi._tcp")
    pub service_type: String,
    
    /// Domain (e.g., "local")
    pub domain: String,
    
    /// Full service name (service_name.service_type.domain)
    pub full_name: String,
    
    /// Target hostname
    pub target: String,
    
    /// Port number
    pub port: u16,
    
    /// Priority (for SRV records)
    pub priority: u16,
    
    /// Weight (for SRV records)
    pub weight: u16,
    
    /// TTL (Time To Live) in seconds
    pub ttl: u32,
    
    /// TXT records for additional metadata
    pub txt_records: HashMap<String, String>,
    
    /// IP addresses associated with this service
    pub ip_addresses: Vec<IpAddr>,
    
    /// BPI address this mDNS record represents
    pub bpi_address: String,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    
    /// Status of the mDNS record
    pub status: MdnsRecordStatus,
}

/// mDNS record status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MdnsRecordStatus {
    Active,
    Inactive,
    Resolving,
    Error,
}

/// mDNS proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdnsProxyConfig {
    /// Enable mDNS proxy functionality
    pub enabled: bool,
    
    /// Default service type for BPI nodes
    pub default_service_type: String,
    
    /// Default domain
    pub default_domain: String,
    
    /// Default TTL for records
    pub default_ttl: u32,
    
    /// Network interface to bind to
    pub bind_interface: Option<String>,
    
    /// Multicast address for mDNS
    pub multicast_addr: IpAddr,
    
    /// Multicast port for mDNS
    pub multicast_port: u16,
    
    /// Enable IPv6 support
    pub ipv6_enabled: bool,
    
    /// Cache timeout for resolved records
    pub cache_timeout: u64,
}

/// mDNS query for service discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdnsQuery {
    /// Query type (A, AAAA, SRV, TXT, PTR)
    pub query_type: String,
    
    /// Query name
    pub name: String,
    
    /// Query class (usually IN for Internet)
    pub class: String,
    
    /// Timestamp when query was made
    pub timestamp: DateTime<Utc>,
}

/// mDNS response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdnsResponse {
    /// Response records
    pub records: Vec<MdnsServiceRecord>,
    
    /// Query that generated this response
    pub query: MdnsQuery,
    
    /// Response timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Response source IP
    pub source_ip: IpAddr,
}

/// mDNS Proxy Manager - Network discovery for BPI addresses
#[derive(Debug)]
pub struct MdnsProxyManager {
    /// Active mDNS service records
    service_records: Arc<RwLock<HashMap<String, MdnsServiceRecord>>>,
    
    /// BPI address to mDNS mapping
    address_mapping: Arc<RwLock<HashMap<String, String>>>, // bpi_address -> service_name
    
    /// mDNS proxy configuration
    config: Arc<RwLock<MdnsProxyConfig>>,
    
    /// Query cache for performance
    query_cache: Arc<RwLock<HashMap<String, MdnsResponse>>>,
    
    /// Statistics
    stats: Arc<RwLock<MdnsProxyStats>>,
}

/// Statistics for mDNS proxy operations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MdnsProxyStats {
    pub total_records: u64,
    pub active_records: u64,
    pub total_queries: u64,
    pub successful_resolutions: u64,
    pub failed_resolutions: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub last_operation: Option<DateTime<Utc>>,
}

impl Default for MdnsProxyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_service_type: "_bpi._tcp".to_string(),
            default_domain: "local".to_string(),
            default_ttl: 300, // 5 minutes
            bind_interface: None,
            multicast_addr: IpAddr::V4(Ipv4Addr::new(224, 0, 0, 251)), // Standard mDNS multicast
            multicast_port: 5353, // Standard mDNS port
            ipv6_enabled: true,
            cache_timeout: 300, // 5 minutes
        }
    }
}

impl MdnsProxyManager {
    /// Create new mDNS proxy manager
    pub fn new(config: MdnsProxyConfig) -> Self {
        Self {
            service_records: Arc::new(RwLock::new(HashMap::new())),
            address_mapping: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(RwLock::new(config)),
            query_cache: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(MdnsProxyStats::default())),
        }
    }
    
    /// Register BPI address with mDNS proxy
    pub async fn register_bpi_address(
        &self,
        bpi_address: &str,
        service_name: &str,
        port: u16,
        txt_records: HashMap<String, String>,
    ) -> Result<String> {
        let config = self.config.read().await;
        
        // Create full service name
        let full_name = format!("{}.{}.{}", 
            service_name, 
            config.default_service_type, 
            config.default_domain
        );
        
        // Generate target hostname from BPI address
        let target = format!("{}.{}", 
            bpi_address.replace("bpi_", ""), 
            config.default_domain
        );
        
        // Create service record
        let record = MdnsServiceRecord {
            service_name: service_name.to_string(),
            service_type: config.default_service_type.clone(),
            domain: config.default_domain.clone(),
            full_name: full_name.clone(),
            target,
            port,
            priority: 0,
            weight: 5,
            ttl: config.default_ttl,
            txt_records,
            ip_addresses: vec![], // Will be resolved dynamically
            bpi_address: bpi_address.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: MdnsRecordStatus::Active,
        };
        
        // Store record
        self.service_records.write().await.insert(full_name.clone(), record);
        self.address_mapping.write().await.insert(bpi_address.to_string(), service_name.to_string());
        
        // Update statistics
        self.update_stats_register().await;
        
        Ok(full_name)
    }
    
    /// Resolve BPI address to network information
    pub async fn resolve_bpi_address(&self, bpi_address: &str) -> Result<Option<MdnsServiceRecord>> {
        // Check if we have a mapping for this BPI address
        let service_name = {
            let mapping = self.address_mapping.read().await;
            mapping.get(bpi_address).cloned()
        };
        
        if let Some(service_name) = service_name {
            let config = self.config.read().await;
            let full_name = format!("{}.{}.{}", 
                service_name, 
                config.default_service_type, 
                config.default_domain
            );
            
            let records = self.service_records.read().await;
            if let Some(record) = records.get(&full_name) {
                self.update_stats_resolve_success().await;
                return Ok(Some(record.clone()));
            }
        }
        
        self.update_stats_resolve_fail().await;
        Ok(None)
    }
    
    /// Discover BPI services on the network
    pub async fn discover_bpi_services(&self) -> Result<Vec<MdnsServiceRecord>> {
        let records = self.service_records.read().await;
        let active_records: Vec<MdnsServiceRecord> = records
            .values()
            .filter(|record| record.status == MdnsRecordStatus::Active)
            .cloned()
            .collect();
        
        self.update_stats_query().await;
        Ok(active_records)
    }
    
    /// Query for specific service type
    pub async fn query_service_type(&self, service_type: &str) -> Result<Vec<MdnsServiceRecord>> {
        let records = self.service_records.read().await;
        let matching_records: Vec<MdnsServiceRecord> = records
            .values()
            .filter(|record| {
                record.service_type == service_type && 
                record.status == MdnsRecordStatus::Active
            })
            .cloned()
            .collect();
        
        self.update_stats_query().await;
        Ok(matching_records)
    }
    
    /// Update service record
    pub async fn update_service_record(
        &self,
        bpi_address: &str,
        port: Option<u16>,
        txt_records: Option<HashMap<String, String>>,
        ip_addresses: Option<Vec<IpAddr>>,
    ) -> Result<bool> {
        let service_name = {
            let mapping = self.address_mapping.read().await;
            mapping.get(bpi_address).cloned()
        };
        
        if let Some(service_name) = service_name {
            let config = self.config.read().await;
            let full_name = format!("{}.{}.{}", 
                service_name, 
                config.default_service_type, 
                config.default_domain
            );
            
            let mut records = self.service_records.write().await;
            if let Some(record) = records.get_mut(&full_name) {
                if let Some(port) = port {
                    record.port = port;
                }
                if let Some(txt_records) = txt_records {
                    record.txt_records = txt_records;
                }
                if let Some(ip_addresses) = ip_addresses {
                    record.ip_addresses = ip_addresses;
                }
                record.updated_at = Utc::now();
                
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    /// Remove BPI address from mDNS proxy
    pub async fn unregister_bpi_address(&self, bpi_address: &str) -> Result<bool> {
        let service_name = {
            let mut mapping = self.address_mapping.write().await;
            mapping.remove(bpi_address)
        };
        
        if let Some(service_name) = service_name {
            let config = self.config.read().await;
            let full_name = format!("{}.{}.{}", 
                service_name, 
                config.default_service_type, 
                config.default_domain
            );
            
            let mut records = self.service_records.write().await;
            records.remove(&full_name);
            
            return Ok(true);
        }
        
        Ok(false)
    }
    
    /// Get all registered BPI addresses
    pub async fn list_registered_addresses(&self) -> Vec<String> {
        let mapping = self.address_mapping.read().await;
        mapping.keys().cloned().collect()
    }
    
    /// Get mDNS proxy statistics
    pub async fn get_stats(&self) -> MdnsProxyStats {
        self.stats.read().await.clone()
    }
    
    /// Health check for mDNS proxy
    pub async fn health_check(&self) -> Result<bool> {
        let config = self.config.read().await;
        Ok(config.enabled)
    }
    
    /// Start mDNS proxy service (placeholder for actual network implementation)
    pub async fn start_service(&self) -> Result<()> {
        // In a real implementation, this would:
        // 1. Bind to multicast socket
        // 2. Start listening for mDNS queries
        // 3. Respond to queries for registered services
        // 4. Handle service announcements
        
        println!("🌐 mDNS Proxy Service started for BPI addresses");
        Ok(())
    }
    
    /// Stop mDNS proxy service
    pub async fn stop_service(&self) -> Result<()> {
        println!("🛑 mDNS Proxy Service stopped");
        Ok(())
    }
    
    // Private helper methods
    
    async fn update_stats_register(&self) {
        let mut stats = self.stats.write().await;
        stats.total_records += 1;
        stats.active_records += 1;
        stats.last_operation = Some(Utc::now());
    }
    
    async fn update_stats_query(&self) {
        let mut stats = self.stats.write().await;
        stats.total_queries += 1;
        stats.last_operation = Some(Utc::now());
    }
    
    async fn update_stats_resolve_success(&self) {
        let mut stats = self.stats.write().await;
        stats.successful_resolutions += 1;
        stats.cache_hits += 1;
        stats.last_operation = Some(Utc::now());
    }
    
    async fn update_stats_resolve_fail(&self) {
        let mut stats = self.stats.write().await;
        stats.failed_resolutions += 1;
        stats.cache_misses += 1;
        stats.last_operation = Some(Utc::now());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_mdns_proxy_creation() {
        let config = MdnsProxyConfig::default();
        let manager = MdnsProxyManager::new(config);
        assert!(manager.health_check().await.unwrap());
    }
    
    #[tokio::test]
    async fn test_register_bpi_address() {
        let config = MdnsProxyConfig::default();
        let manager = MdnsProxyManager::new(config);
        
        let mut txt_records = HashMap::new();
        txt_records.insert("version".to_string(), "1.0".to_string());
        txt_records.insert("type".to_string(), "bpi-node".to_string());
        
        let full_name = manager.register_bpi_address(
            "bpi_test_address_123",
            "test-node",
            8080,
            txt_records,
        ).await.unwrap();
        
        assert!(full_name.contains("test-node"));
        assert!(full_name.contains("_bpi._tcp"));
        assert!(full_name.contains("local"));
    }
    
    #[tokio::test]
    async fn test_resolve_bpi_address() {
        let config = MdnsProxyConfig::default();
        let manager = MdnsProxyManager::new(config);
        
        // Register first
        let mut txt_records = HashMap::new();
        txt_records.insert("version".to_string(), "1.0".to_string());
        
        manager.register_bpi_address(
            "bpi_test_address_456",
            "resolve-test",
            9090,
            txt_records,
        ).await.unwrap();
        
        // Then resolve
        let resolved = manager.resolve_bpi_address("bpi_test_address_456").await.unwrap();
        assert!(resolved.is_some());
        
        let record = resolved.unwrap();
        assert_eq!(record.bpi_address, "bpi_test_address_456");
        assert_eq!(record.port, 9090);
    }
}
