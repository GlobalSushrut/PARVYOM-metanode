//! DynaRoute Service Discovery
//! 
//! Implements distributed service registry for serverless P2P mesh.
//! 
//! # Purpose
//! 
//! Replaces hardcoded URLs with dynamic service discovery:
//! - Services register themselves on startup
//! - Services discover each other via distributed hash table (DHT)
//! - Health monitoring with automatic failover
//! - Load balancing across service instances
//! 
//! # Architecture
//! 
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    ServiceRegistry (DHT)                     │
//! ├─────────────────────────────────────────────────────────────┤
//! │  "consensus"  → [node1:8080, node2:8080, node3:8080]       │
//! │  "blockchain" → [node1:9000, node2:9000, node3:9000]       │
//! │  "auction"    → [node1:7002, node2:7002, node3:7002]       │
//! └─────────────────────────────────────────────────────────────┘
//! ```

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

/// Service endpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Service name (e.g., "consensus", "blockchain")
    pub service_name: String,
    
    /// Node identifier
    pub node_id: String,
    
    /// Network address
    pub address: SocketAddr,
    
    /// Service metadata
    pub metadata: HashMap<String, String>,
    
    /// Last heartbeat timestamp
    #[serde(skip)]
    #[serde(default = "Instant::now")]
    pub last_heartbeat: Instant,
    
    /// Service version
    pub version: String,
}

/// Service health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Service is healthy
    Healthy,
    
    /// Service is degraded
    Degraded,
    
    /// Service is unhealthy
    Unhealthy,
    
    /// Service is unknown (no recent heartbeat)
    Unknown,
}

/// Service registry configuration
#[derive(Debug, Clone)]
pub struct RegistryConfig {
    /// Heartbeat interval
    pub heartbeat_interval: Duration,
    
    /// Heartbeat timeout (mark as unhealthy)
    pub heartbeat_timeout: Duration,
    
    /// Service cleanup interval (remove dead services)
    pub cleanup_interval: Duration,
    
    /// Maximum services per name
    pub max_services_per_name: usize,
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval: Duration::from_secs(5),
            heartbeat_timeout: Duration::from_secs(30),
            cleanup_interval: Duration::from_secs(60),
            max_services_per_name: 100,
        }
    }
}

/// Distributed service registry
pub struct ServiceRegistry {
    /// Local node ID
    node_id: String,
    
    /// Configuration
    config: RegistryConfig,
    
    /// Service endpoints (service_name -> endpoints)
    services: Arc<RwLock<HashMap<String, Vec<ServiceEndpoint>>>>,
    
    /// Health status (service_name:node_id -> status)
    health: Arc<RwLock<HashMap<String, HealthStatus>>>,
    
    /// Round-robin counters for load balancing
    round_robin: Arc<RwLock<HashMap<String, usize>>>,
}

impl ServiceRegistry {
    /// Create a new service registry
    pub fn new(node_id: String, config: RegistryConfig) -> Self {
        Self {
            node_id,
            config,
            services: Arc::new(RwLock::new(HashMap::new())),
            health: Arc::new(RwLock::new(HashMap::new())),
            round_robin: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Register a service
    pub async fn register(&self, endpoint: ServiceEndpoint) -> Result<(), String> {
        let mut services = self.services.write().await;
        let mut health = self.health.write().await;
        
        let service_list = services
            .entry(endpoint.service_name.clone())
            .or_insert_with(Vec::new);
        
        // Check if already registered
        if let Some(existing) = service_list
            .iter_mut()
            .find(|e| e.node_id == endpoint.node_id)
        {
            // Update existing
            *existing = endpoint.clone();
        } else {
            // Add new
            if service_list.len() >= self.config.max_services_per_name {
                return Err(format!(
                    "Maximum services reached for {}",
                    endpoint.service_name
                ));
            }
            service_list.push(endpoint.clone());
        }
        
        // Mark as healthy
        let key = format!("{}:{}", endpoint.service_name, endpoint.node_id);
        health.insert(key, HealthStatus::Healthy);
        
        Ok(())
    }
    
    /// Deregister a service
    pub async fn deregister(&self, service_name: &str, node_id: &str) -> Result<(), String> {
        let mut services = self.services.write().await;
        let mut health = self.health.write().await;
        
        if let Some(service_list) = services.get_mut(service_name) {
            service_list.retain(|e| e.node_id != node_id);
            if service_list.is_empty() {
                services.remove(service_name);
            }
        }
        
        let key = format!("{}:{}", service_name, node_id);
        health.remove(&key);
        
        Ok(())
    }
    
    /// Discover a service (returns one endpoint via load balancing)
    pub async fn discover(&self, service_name: &str) -> Result<ServiceEndpoint, String> {
        let services = self.services.read().await;
        let health = self.health.read().await;
        
        let service_list = services
            .get(service_name)
            .ok_or_else(|| format!("Service {} not found", service_name))?;
        
        if service_list.is_empty() {
            return Err(format!("No instances of service {}", service_name));
        }
        
        // Filter healthy services
        let healthy_services: Vec<&ServiceEndpoint> = service_list
            .iter()
            .filter(|e| {
                let key = format!("{}:{}", e.service_name, e.node_id);
                matches!(health.get(&key), Some(HealthStatus::Healthy))
            })
            .collect();
        
        if healthy_services.is_empty() {
            return Err(format!("No healthy instances of service {}", service_name));
        }
        
        // Round-robin load balancing
        let mut rr = self.round_robin.write().await;
        let counter = rr.entry(service_name.to_string()).or_insert(0);
        let index = *counter % healthy_services.len();
        *counter = (*counter + 1) % healthy_services.len();
        
        Ok(healthy_services[index].clone())
    }
    
    /// Discover all instances of a service
    pub async fn discover_all(&self, service_name: &str) -> Result<Vec<ServiceEndpoint>, String> {
        let services = self.services.read().await;
        
        let service_list = services
            .get(service_name)
            .ok_or_else(|| format!("Service {} not found", service_name))?;
        
        Ok(service_list.clone())
    }
    
    /// Update heartbeat for a service
    pub async fn heartbeat(&self, service_name: &str, node_id: &str) -> Result<(), String> {
        let mut services = self.services.write().await;
        let mut health = self.health.write().await;
        
        if let Some(service_list) = services.get_mut(service_name) {
            if let Some(endpoint) = service_list.iter_mut().find(|e| e.node_id == node_id) {
                endpoint.last_heartbeat = Instant::now();
                
                let key = format!("{}:{}", service_name, node_id);
                health.insert(key, HealthStatus::Healthy);
                
                return Ok(());
            }
        }
        
        Err(format!("Service {}:{} not found", service_name, node_id))
    }
    
    /// Check and update health status
    pub async fn check_health(&self) {
        let services = self.services.read().await;
        let mut health = self.health.write().await;
        
        let now = Instant::now();
        
        for (service_name, service_list) in services.iter() {
            for endpoint in service_list {
                let key = format!("{}:{}", service_name, endpoint.node_id);
                let age = now.duration_since(endpoint.last_heartbeat);
                
                let status = if age > self.config.heartbeat_timeout {
                    HealthStatus::Unhealthy
                } else if age > self.config.heartbeat_timeout / 2 {
                    HealthStatus::Degraded
                } else {
                    HealthStatus::Healthy
                };
                
                health.insert(key, status);
            }
        }
    }
    
    /// Cleanup dead services
    pub async fn cleanup(&self) {
        let mut services = self.services.write().await;
        let mut health = self.health.write().await;
        
        let now = Instant::now();
        
        for service_list in services.values_mut() {
            service_list.retain(|e| {
                let age = now.duration_since(e.last_heartbeat);
                age <= self.config.heartbeat_timeout * 2
            });
        }
        
        // Remove empty service lists
        services.retain(|_, v| !v.is_empty());
        
        // Cleanup health status
        let valid_keys: std::collections::HashSet<String> = services
            .iter()
            .flat_map(|(name, list)| {
                list.iter()
                    .map(move |e| format!("{}:{}", name, e.node_id))
            })
            .collect();
        
        health.retain(|k, _| valid_keys.contains(k));
    }
    
    /// Get service count
    pub async fn service_count(&self) -> usize {
        let services = self.services.read().await;
        services.len()
    }
    
    /// Get total endpoint count
    pub async fn endpoint_count(&self) -> usize {
        let services = self.services.read().await;
        services.values().map(|v| v.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_endpoint(service_name: &str, node_id: &str, port: u16) -> ServiceEndpoint {
        ServiceEndpoint {
            service_name: service_name.to_string(),
            node_id: node_id.to_string(),
            address: format!("127.0.0.1:{}", port).parse().unwrap(),
            metadata: HashMap::new(),
            last_heartbeat: Instant::now(),
            version: "1.0.0".to_string(),
        }
    }
    
    #[tokio::test]
    async fn test_registry_creation() {
        let config = RegistryConfig::default();
        let registry = ServiceRegistry::new("node1".to_string(), config);
        
        assert_eq!(registry.service_count().await, 0);
        assert_eq!(registry.endpoint_count().await, 0);
    }
    
    #[tokio::test]
    async fn test_service_registration() {
        let config = RegistryConfig::default();
        let registry = ServiceRegistry::new("node1".to_string(), config);
        
        let endpoint = create_test_endpoint("consensus", "node1", 8080);
        let result = registry.register(endpoint).await;
        
        assert!(result.is_ok());
        assert_eq!(registry.service_count().await, 1);
        assert_eq!(registry.endpoint_count().await, 1);
    }
    
    #[tokio::test]
    async fn test_service_discovery() {
        let config = RegistryConfig::default();
        let registry = ServiceRegistry::new("node1".to_string(), config);
        
        let endpoint = create_test_endpoint("consensus", "node1", 8080);
        registry.register(endpoint.clone()).await.unwrap();
        
        let discovered = registry.discover("consensus").await.unwrap();
        assert_eq!(discovered.service_name, "consensus");
        assert_eq!(discovered.node_id, "node1");
    }
    
    #[tokio::test]
    async fn test_service_deregistration() {
        let config = RegistryConfig::default();
        let registry = ServiceRegistry::new("node1".to_string(), config);
        
        let endpoint = create_test_endpoint("consensus", "node1", 8080);
        registry.register(endpoint).await.unwrap();
        
        registry.deregister("consensus", "node1").await.unwrap();
        
        assert_eq!(registry.service_count().await, 0);
    }
    
    #[tokio::test]
    async fn test_round_robin_load_balancing() {
        let config = RegistryConfig::default();
        let registry = ServiceRegistry::new("node1".to_string(), config);
        
        // Register 3 instances
        registry
            .register(create_test_endpoint("consensus", "node1", 8080))
            .await
            .unwrap();
        registry
            .register(create_test_endpoint("consensus", "node2", 8080))
            .await
            .unwrap();
        registry
            .register(create_test_endpoint("consensus", "node3", 8080))
            .await
            .unwrap();
        
        // Discover should round-robin
        let e1 = registry.discover("consensus").await.unwrap();
        let e2 = registry.discover("consensus").await.unwrap();
        let e3 = registry.discover("consensus").await.unwrap();
        let e4 = registry.discover("consensus").await.unwrap();
        
        assert_ne!(e1.node_id, e2.node_id);
        assert_ne!(e2.node_id, e3.node_id);
        assert_eq!(e1.node_id, e4.node_id); // Should wrap around
    }
    
    #[tokio::test]
    async fn test_discover_all() {
        let config = RegistryConfig::default();
        let registry = ServiceRegistry::new("node1".to_string(), config);
        
        registry
            .register(create_test_endpoint("consensus", "node1", 8080))
            .await
            .unwrap();
        registry
            .register(create_test_endpoint("consensus", "node2", 8080))
            .await
            .unwrap();
        
        let all = registry.discover_all("consensus").await.unwrap();
        assert_eq!(all.len(), 2);
    }
    
    #[tokio::test]
    async fn test_heartbeat() {
        let config = RegistryConfig::default();
        let registry = ServiceRegistry::new("node1".to_string(), config);
        
        let endpoint = create_test_endpoint("consensus", "node1", 8080);
        registry.register(endpoint).await.unwrap();
        
        let result = registry.heartbeat("consensus", "node1").await;
        assert!(result.is_ok());
    }
}
