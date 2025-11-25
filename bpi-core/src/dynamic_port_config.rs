//! Dynamic Port Configuration System
//! 
//! Replaces hardcoded ports with DynaRoute-based service discovery
//! and dynamic port allocation for BPI Core services.

use std::collections::HashMap;
use std::sync::Arc;
use std::path::Path;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use tracing::{info, warn, debug};
use crate::dynaroute_client::DynaRouteClient;

/// Dynamic port configuration manager
#[derive(Debug)]
pub struct DynamicPortConfig {
    /// DynaRoute client for service discovery
    dynaroute_client: Arc<DynaRouteClient>,
    
    /// Local port cache
    port_cache: Arc<RwLock<HashMap<String, u16>>>,
    
    /// Fallback port mappings
    fallback_ports: HashMap<String, u16>,
    
    /// Registry endpoint
    registry_endpoint: String,
}

/// Service port configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePortConfig {
    pub service_name: String,
    pub port: u16,
    pub address: String,
    pub is_dynamic: bool,
}

impl DynamicPortConfig {
    /// Create new dynamic port configuration
    pub fn new(registry_endpoint: &str) -> Self {
        let dynaroute_client = Arc::new(DynaRouteClient::new(registry_endpoint));
        
        // Define fallback ports for development
        let mut fallback_ports = HashMap::new();
        fallback_ports.insert("dynaroute-registry".to_string(), 8087);
        fallback_ports.insert("bso-k8-kernel".to_string(), 9000);
        fallback_ports.insert("zk-kernel".to_string(), 9001);
        fallback_ports.insert("process-scheduler".to_string(), 9002);
        fallback_ports.insert("resource-manager".to_string(), 9003);
        fallback_ports.insert("vm-server".to_string(), 9090);
        fallback_ports.insert("6d-blockchain".to_string(), 9004);
        fallback_ports.insert("logbook-service".to_string(), 9005);
        fallback_ports.insert("ziplock-coordinator".to_string(), 9006);
        fallback_ports.insert("audit-http-server".to_string(), 9007);
        fallback_ports.insert("bpci-server".to_string(), 8080);
        fallback_ports.insert("consensus-server".to_string(), 8081);
        fallback_ports.insert("auction-mempool".to_string(), 7002);
        fallback_ports.insert("bpi-bpci-bridge".to_string(), 6001);
        
        Self {
            dynaroute_client,
            port_cache: Arc::new(RwLock::new(HashMap::new())),
            fallback_ports,
            registry_endpoint: registry_endpoint.to_string(),
        }
    }
    
    /// Get port for a service (with caching and fallback)
    pub async fn get_service_port(&self, service_name: &str) -> Result<u16> {
        // Check cache first
        {
            let cache = self.port_cache.read().await;
            if let Some(&port) = cache.get(service_name) {
                debug!("📋 Cache hit for service {}: port {}", service_name, port);
                return Ok(port);
            }
        }
        
        // Try DynaRoute discovery
        match self.dynaroute_client.discover_service(service_name).await {
            Ok(address) => {
                if let Some(port) = Self::extract_port_from_address(&address.to_string()) {
                    // Cache the result
                    let mut cache = self.port_cache.write().await;
                    cache.insert(service_name.to_string(), port);
                    
                    info!("🔍 Discovered service {} at port {}", service_name, port);
                    return Ok(port);
                }
            }
            Err(e) => {
                debug!("❌ DynaRoute discovery failed for {}: {}", service_name, e);
            }
        }
        
        // Fallback to predefined ports
        if let Some(&port) = self.fallback_ports.get(service_name) {
            warn!("⚠️ Using fallback port {} for service {}", port, service_name);
            
            // Cache fallback
            let mut cache = self.port_cache.write().await;
            cache.insert(service_name.to_string(), port);
            
            return Ok(port);
        }
        
        Err(anyhow::anyhow!("No port found for service: {}", service_name))
    }
    
    /// Get full address for a service
    pub async fn get_service_address(&self, service_name: &str) -> Result<String> {
        // Try DynaRoute first
        match self.dynaroute_client.discover_service(service_name).await {
            Ok(address) => {
                info!("🔍 Discovered service {} at {}", service_name, address);
                return Ok(address.to_string());
            }
            Err(e) => {
                debug!("❌ DynaRoute discovery failed for {}: {}", service_name, e);
            }
        }
        
        // Fallback to localhost + fallback port
        let port = self.get_service_port(service_name).await?;
        let address = format!("localhost:{}", port);
        
        warn!("⚠️ Using fallback address {} for service {}", address, service_name);
        Ok(address)
    }
    
    /// Register a service with DynaRoute
    pub async fn register_service(&self, service_name: &str, port: u16) -> Result<()> {
        let address = format!("localhost:{}", port);
        
        match self.dynaroute_client.register_service(service_name, &address).await {
            Ok(_) => {
                // Update cache
                let mut cache = self.port_cache.write().await;
                cache.insert(service_name.to_string(), port);
                
                info!("✅ Registered service {} at {}", service_name, address);

                // If running on immutable OS, also persist an OS-level dynaroute config
                let era_dynaroutes = Path::new("/era/mutable/var/bpi/network/dynaroutes");
                if era_dynaroutes.exists() {
                    let cfg = serde_json::json!({
                        "service_name": service_name,
                        "local_bind": address,
                        "target": service_name,
                        "enc_cluster": null,
                        "enabled": true,
                    });

                    let file_path = era_dynaroutes.join(format!("{}.json", service_name));
                    if let Err(e) = std::fs::write(&file_path, serde_json::to_vec_pretty(&cfg)?) {
                        warn!("❌ Failed to write DynarouteConfig {}: {}", file_path.display(), e);
                    }
                }

                Ok(())
            }
            Err(e) => {
                warn!("❌ Failed to register service {}: {}", service_name, e);
                // Don't fail startup for registration errors
                Ok(())
            }
        }
    }
    
    /// Get all configured services
    pub async fn list_services(&self) -> Vec<ServicePortConfig> {
        let cache = self.port_cache.read().await;
        let mut services = Vec::new();
        
        // Add cached services
        for (service_name, &port) in cache.iter() {
            services.push(ServicePortConfig {
                service_name: service_name.clone(),
                port,
                address: format!("localhost:{}", port),
                is_dynamic: true,
            });
        }
        
        // Add fallback services not in cache
        for (service_name, &port) in &self.fallback_ports {
            if !cache.contains_key(service_name) {
                services.push(ServicePortConfig {
                    service_name: service_name.clone(),
                    port,
                    address: format!("localhost:{}", port),
                    is_dynamic: false,
                });
            }
        }
        
        services
    }
    
    /// Clear cache (force refresh)
    pub async fn clear_cache(&self) {
        let mut cache = self.port_cache.write().await;
        cache.clear();
        info!("🗑️ Cleared service port cache");
    }
    
    /// Extract port number from address string
    fn extract_port_from_address(address: &str) -> Option<u16> {
        if let Some(colon_pos) = address.rfind(':') {
            let port_str = &address[colon_pos + 1..];
            port_str.parse().ok()
        } else {
            None
        }
    }
    
    /// Check if DynaRoute registry is available
    pub async fn is_registry_available(&self) -> bool {
        // Try to connect to registry health endpoint
        let health_url = format!("http://{}/health", self.registry_endpoint);
        
        match reqwest::get(&health_url).await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }
    
    /// Get registry status
    pub async fn get_registry_status(&self) -> String {
        if self.is_registry_available().await {
            "✅ Available".to_string()
        } else {
            "❌ Unavailable (using fallbacks)".to_string()
        }
    }
}

/// Global port configuration instance
static mut GLOBAL_PORT_CONFIG: Option<Arc<DynamicPortConfig>> = None;
static INIT: std::sync::Once = std::sync::Once::new();

/// Initialize global port configuration
pub fn init_global_port_config(registry_endpoint: &str) {
    INIT.call_once(|| {
        let config = Arc::new(DynamicPortConfig::new(registry_endpoint));
        unsafe {
            GLOBAL_PORT_CONFIG = Some(config);
        }
    });
}

/// Get global port configuration
pub fn get_global_port_config() -> Option<Arc<DynamicPortConfig>> {
    unsafe { GLOBAL_PORT_CONFIG.clone() }
}

/// Convenience function to get service port
pub async fn get_service_port(service_name: &str) -> Result<u16> {
    match get_global_port_config() {
        Some(config) => config.get_service_port(service_name).await,
        None => Err(anyhow::anyhow!("Port configuration not initialized")),
    }
}

/// Convenience function to get service address
pub async fn get_service_address(service_name: &str) -> Result<String> {
    match get_global_port_config() {
        Some(config) => config.get_service_address(service_name).await,
        None => Err(anyhow::anyhow!("Port configuration not initialized")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_port_config_creation() {
        let config = DynamicPortConfig::new("localhost:8087");
        assert!(!config.fallback_ports.is_empty());
    }
    
    #[tokio::test]
    async fn test_fallback_port_lookup() {
        let config = DynamicPortConfig::new("localhost:8087");
        let port = config.get_service_port("bso-k8-kernel").await.unwrap();
        assert_eq!(port, 9000);
    }
    
    #[tokio::test]
    async fn test_service_listing() {
        let config = DynamicPortConfig::new("localhost:8087");
        let services = config.list_services().await;
        assert!(!services.is_empty());
    }
}
