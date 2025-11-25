//! Service Migration Framework
//! 
//! Migrates services from hardcoded URLs to dynamic mesh discovery.
//! 
//! # Purpose
//! 
//! Enable serverless service discovery and routing:
//! - Replace hardcoded service URLs with mesh discovery
//! - Dynamic service endpoint resolution
//! - Load balancing across service instances
//! - Health-aware routing
//! - Automatic failover
//! 
//! # Migration Strategy
//! 
//! ```text
//! Before: http://hardcoded-consensus:8080/api/consensus
//! After:  mesh://consensus/api/consensus
//! 
//! Resolution:
//! 1. Query ServiceRegistry for "consensus"
//! 2. Get healthy endpoints with load balancing
//! 3. Route to selected endpoint
//! 4. Automatic retry on failure
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

use super::discovery::ServiceRegistry;
use super::handshake::P2PMesh;

/// Service URL types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceUrl {
    /// Hardcoded URL (legacy)
    Hardcoded(String),
    
    /// Mesh-based URL (new)
    Mesh {
        service_name: String,
        path: String,
    },
}

impl ServiceUrl {
    /// Parse a service URL
    pub fn parse(url: &str) -> Result<Self, String> {
        if url.starts_with("mesh://") {
            // Parse mesh URL: mesh://service_name/path
            let without_prefix = url.strip_prefix("mesh://").unwrap();
            let parts: Vec<&str> = without_prefix.splitn(2, '/').collect();
            
            if parts.is_empty() {
                return Err("Invalid mesh URL: missing service name".to_string());
            }
            
            let service_name = parts[0].to_string();
            let path = if parts.len() > 1 {
                format!("/{}", parts[1])
            } else {
                "/".to_string()
            };
            
            Ok(ServiceUrl::Mesh { service_name, path })
        } else {
            // Legacy hardcoded URL
            Ok(ServiceUrl::Hardcoded(url.to_string()))
        }
    }
    
    /// Convert to string
    pub fn to_string(&self) -> String {
        match self {
            ServiceUrl::Hardcoded(url) => url.clone(),
            ServiceUrl::Mesh { service_name, path } => {
                format!("mesh://{}{}", service_name, path)
            }
        }
    }
}

/// Service migration configuration
#[derive(Debug, Clone)]
pub struct MigrationConfig {
    /// Enable mesh discovery
    pub enable_mesh: bool,
    
    /// Fallback to hardcoded URLs on mesh failure
    pub fallback_to_hardcoded: bool,
    
    /// Retry attempts
    pub retry_attempts: usize,
    
    /// Retry delay
    pub retry_delay: Duration,
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self {
            enable_mesh: true,
            fallback_to_hardcoded: true,
            retry_attempts: 3,
            retry_delay: Duration::from_millis(100),
        }
    }
}

/// Service router with mesh discovery
pub struct ServiceRouter {
    /// Configuration
    config: MigrationConfig,
    
    /// Service registry
    registry: Arc<ServiceRegistry>,
    
    /// P2P mesh
    mesh: Arc<RwLock<P2PMesh>>,
    
    /// Hardcoded URL mappings (fallback)
    hardcoded_urls: Arc<RwLock<HashMap<String, String>>>,
}

impl ServiceRouter {
    /// Create a new service router
    pub fn new(
        config: MigrationConfig,
        registry: Arc<ServiceRegistry>,
        mesh: Arc<RwLock<P2PMesh>>,
    ) -> Self {
        Self {
            config,
            registry,
            mesh,
            hardcoded_urls: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Register hardcoded URL (for fallback)
    pub async fn register_hardcoded(&self, service_name: String, url: String) {
        self.hardcoded_urls.write().await.insert(service_name, url);
    }
    
    /// Resolve service URL to actual endpoint
    pub async fn resolve(&self, url: &str) -> Result<String, String> {
        let service_url = ServiceUrl::parse(url)?;
        
        match service_url {
            ServiceUrl::Hardcoded(url) => Ok(url),
            ServiceUrl::Mesh { service_name, path } => {
                self.resolve_mesh(&service_name, &path).await
            }
        }
    }
    
    /// Resolve mesh URL to actual endpoint
    async fn resolve_mesh(&self, service_name: &str, path: &str) -> Result<String, String> {
        if !self.config.enable_mesh {
            return self.fallback_to_hardcoded(service_name, path).await;
        }
        
        // Try to discover service from registry
        match self.registry.discover(service_name).await {
            Ok(endpoint) => {
                let url = format!("http://{}{}", endpoint.address, path);
                Ok(url)
            }
            Err(e) => {
                println!("Mesh discovery failed for {}: {}", service_name, e);
                
                if self.config.fallback_to_hardcoded {
                    self.fallback_to_hardcoded(service_name, path).await
                } else {
                    Err(format!("Service {} not found in mesh", service_name))
                }
            }
        }
    }
    
    /// Fallback to hardcoded URL
    async fn fallback_to_hardcoded(&self, service_name: &str, path: &str) -> Result<String, String> {
        let hardcoded = self.hardcoded_urls.read().await;
        
        if let Some(base_url) = hardcoded.get(service_name) {
            Ok(format!("{}{}", base_url, path))
        } else {
            Err(format!("No fallback URL for service {}", service_name))
        }
    }
    
    /// Route request with automatic retry
    pub async fn route_with_retry(
        &self,
        url: &str,
        request_fn: impl Fn(String) -> Result<String, String>,
    ) -> Result<String, String> {
        let mut last_error = String::new();
        
        for attempt in 0..self.config.retry_attempts {
            match self.resolve(url).await {
                Ok(endpoint) => {
                    match request_fn(endpoint) {
                        Ok(response) => return Ok(response),
                        Err(e) => {
                            last_error = e;
                            if attempt < self.config.retry_attempts - 1 {
                                tokio::time::sleep(self.config.retry_delay).await;
                            }
                        }
                    }
                }
                Err(e) => {
                    last_error = e;
                    if attempt < self.config.retry_attempts - 1 {
                        tokio::time::sleep(self.config.retry_delay).await;
                    }
                }
            }
        }
        
        Err(format!("All retry attempts failed: {}", last_error))
    }
}

/// Service migration helper
pub struct ServiceMigration {
    /// Service router
    router: Arc<ServiceRouter>,
    
    /// Migration status
    migrated_services: Arc<RwLock<HashMap<String, bool>>>,
}

impl ServiceMigration {
    /// Create a new service migration helper
    pub fn new(router: Arc<ServiceRouter>) -> Self {
        Self {
            router,
            migrated_services: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Migrate a service from hardcoded URL to mesh
    pub async fn migrate_service(
        &self,
        service_name: String,
        hardcoded_url: String,
    ) -> Result<(), String> {
        // Register hardcoded URL as fallback
        self.router.register_hardcoded(service_name.clone(), hardcoded_url).await;
        
        // Mark as migrated
        self.migrated_services.write().await.insert(service_name.clone(), true);
        
        println!("Migrated service: {} to mesh discovery", service_name);
        Ok(())
    }
    
    /// Check if service is migrated
    pub async fn is_migrated(&self, service_name: &str) -> bool {
        self.migrated_services
            .read()
            .await
            .get(service_name)
            .copied()
            .unwrap_or(false)
    }
    
    /// Get migration status
    pub async fn get_status(&self) -> HashMap<String, bool> {
        self.migrated_services.read().await.clone()
    }
}

/// Common service names
pub mod services {
    pub const CONSENSUS: &str = "consensus";
    pub const STORAGE: &str = "storage";
    pub const NETWORK: &str = "network";
    pub const REGISTRY: &str = "registry";
    pub const WALLET: &str = "wallet";
    pub const ECONOMY: &str = "economy";
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_service_url_parse() {
        // Test mesh URL
        let url = ServiceUrl::parse("mesh://consensus/api/status").unwrap();
        match url {
            ServiceUrl::Mesh { service_name, path } => {
                assert_eq!(service_name, "consensus");
                assert_eq!(path, "/api/status");
            }
            _ => panic!("Expected Mesh URL"),
        }
        
        // Test hardcoded URL
        let url = ServiceUrl::parse("http://localhost:8080/api").unwrap();
        match url {
            ServiceUrl::Hardcoded(u) => {
                assert_eq!(u, "http://localhost:8080/api");
            }
            _ => panic!("Expected Hardcoded URL"),
        }
    }
    
    #[test]
    fn test_service_url_to_string() {
        let url = ServiceUrl::Mesh {
            service_name: "consensus".to_string(),
            path: "/api/status".to_string(),
        };
        assert_eq!(url.to_string(), "mesh://consensus/api/status");
        
        let url = ServiceUrl::Hardcoded("http://localhost:8080".to_string());
        assert_eq!(url.to_string(), "http://localhost:8080");
    }
    
    #[tokio::test]
    async fn test_service_router_creation() {
        use super::super::discovery::RegistryConfig;
        
        let config = MigrationConfig::default();
        let registry_config = RegistryConfig::default();
        let registry = Arc::new(ServiceRegistry::new("test".to_string(), registry_config));
        
        let mesh_config = super::super::handshake::MeshConfig::default();
        let mesh = Arc::new(RwLock::new(P2PMesh::new(
            "test".to_string(),
            mesh_config,
            registry.clone(),
        )));
        
        let router = ServiceRouter::new(config, registry, mesh);
        
        // Register hardcoded URL
        router.register_hardcoded("consensus".to_string(), "http://localhost:8080".to_string()).await;
        
        // Resolve should fallback to hardcoded
        let resolved = router.resolve("mesh://consensus/api/status").await.unwrap();
        assert_eq!(resolved, "http://localhost:8080/api/status");
    }
    
    #[tokio::test]
    async fn test_service_migration() {
        use super::super::discovery::RegistryConfig;
        
        let config = MigrationConfig::default();
        let registry_config = RegistryConfig::default();
        let registry = Arc::new(ServiceRegistry::new("test".to_string(), registry_config));
        
        let mesh_config = super::super::handshake::MeshConfig::default();
        let mesh = Arc::new(RwLock::new(P2PMesh::new(
            "test".to_string(),
            mesh_config,
            registry.clone(),
        )));
        
        let router = Arc::new(ServiceRouter::new(config, registry, mesh));
        let migration = ServiceMigration::new(router);
        
        // Migrate service
        migration.migrate_service(
            "consensus".to_string(),
            "http://localhost:8080".to_string(),
        ).await.unwrap();
        
        // Check migration status
        assert!(migration.is_migrated("consensus").await);
        assert!(!migration.is_migrated("storage").await);
    }
}
