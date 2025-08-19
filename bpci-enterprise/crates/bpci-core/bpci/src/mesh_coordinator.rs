//! BPCI Mesh Coordinator & Service Registry
//! 
//! Provides automatic service discovery, health monitoring, and mesh coordination
//! for the Metanode/BPI enterprise infrastructure.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, info, warn, error};
use uuid::Uuid;

use crate::{BpciTransport, BpciConfig, BpciError};

/// Mesh Coordinator Errors
#[derive(Error, Debug)]
pub enum MeshError {
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
    #[error("Service already registered: {0}")]
    ServiceAlreadyExists(String),
    #[error("Health check failed: {0}")]
    HealthCheckFailed(String),
    #[error("Discovery protocol error: {0}")]
    DiscoveryError(String),
    #[error("Load balancing error: {0}")]
    LoadBalancingError(String),
    #[error("Configuration distribution failed: {0}")]
    ConfigDistributionFailed(String),
    #[error("Transport error: {0}")]
    Transport(#[from] BpciError),
}

/// Service identifier type
pub type ServiceId = String;

/// Service capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceCapability {
    /// BPCI messaging capability
    BpciMessaging,
    /// BCI blockchain transactions
    BciTransactions,
    /// DockLock container deployment
    DockLockDeployment,
    /// ENC cluster execution
    EncExecution,
    /// Traffic Light policy enforcement
    TrafficLightPolicy,
    /// BISO compliance monitoring
    BisoCompliance,
    /// Storage and database services
    StorageServices,
    /// Wallet registry services
    WalletRegistry,
    /// Custom capability
    Custom(String),
}

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    /// Service is healthy and operational
    Healthy,
    /// Service is degraded but functional
    Degraded,
    /// Service is unhealthy
    Unhealthy,
    /// Service is unreachable
    Unreachable,
    /// Unknown status
    Unknown,
}

/// Service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Unique service identifier
    pub service_id: ServiceId,
    /// Service endpoint address
    pub endpoint: SocketAddr,
    /// Service capabilities
    pub capabilities: Vec<ServiceCapability>,
    /// Current health status
    pub health_status: HealthStatus,
    /// Last successful heartbeat
    pub last_heartbeat: SystemTime,
    /// Service metadata
    pub metadata: HashMap<String, String>,
    /// Service version
    pub version: String,
    /// Load balancing weight
    pub weight: u32,
    /// Registration timestamp
    pub registered_at: SystemTime,
}

/// Health monitor configuration
#[derive(Debug, Clone)]
pub struct HealthMonitorConfig {
    /// Heartbeat interval
    pub heartbeat_interval: Duration,
    /// Health check timeout
    pub health_check_timeout: Duration,
    /// Maximum missed heartbeats before marking unhealthy
    pub max_missed_heartbeats: u32,
    /// Health check retry attempts
    pub retry_attempts: u32,
}

impl Default for HealthMonitorConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval: Duration::from_secs(30),
            health_check_timeout: Duration::from_secs(10),
            max_missed_heartbeats: 3,
            retry_attempts: 3,
        }
    }
}

/// Discovery protocol configuration
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// Discovery announcement interval
    pub announcement_interval: Duration,
    /// Service discovery timeout
    pub discovery_timeout: Duration,
    /// Maximum services to track
    pub max_services: usize,
    /// Enable automatic service discovery
    pub auto_discovery: bool,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            announcement_interval: Duration::from_secs(60),
            discovery_timeout: Duration::from_secs(30),
            max_services: 1000,
            auto_discovery: true,
        }
    }
}

/// Load balancing strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    /// Round-robin load balancing
    RoundRobin,
    /// Weighted round-robin
    WeightedRoundRobin,
    /// Least connections
    LeastConnections,
    /// Random selection
    Random,
    /// Health-based selection
    HealthBased,
}

/// Mesh coordinator configuration
#[derive(Debug, Clone)]
pub struct MeshCoordinatorConfig {
    /// Coordinator identifier
    pub coordinator_id: String,
    /// Bind address for coordinator
    pub bind_address: SocketAddr,
    /// Health monitor configuration
    pub health_config: HealthMonitorConfig,
    /// Discovery protocol configuration
    pub discovery_config: DiscoveryConfig,
    /// Load balancing strategy
    pub load_balancing: LoadBalancingStrategy,
    /// Enable configuration distribution
    pub enable_config_distribution: bool,
    /// Maximum configuration size
    pub max_config_size: usize,
}

impl Default for MeshCoordinatorConfig {
    fn default() -> Self {
        Self {
            coordinator_id: format!("mesh-coordinator-{}", Uuid::new_v4()),
            bind_address: "127.0.0.1:21001".parse().unwrap(),
            health_config: HealthMonitorConfig::default(),
            discovery_config: DiscoveryConfig::default(),
            load_balancing: LoadBalancingStrategy::WeightedRoundRobin,
            enable_config_distribution: true,
            max_config_size: 1024 * 1024, // 1MB
        }
    }
}

/// Health monitor for tracking service health
#[derive(Debug)]
pub struct HealthMonitor {
    config: HealthMonitorConfig,
    health_checks: Arc<RwLock<HashMap<ServiceId, SystemTime>>>,
    missed_heartbeats: Arc<RwLock<HashMap<ServiceId, u32>>>,
}

impl HealthMonitor {
    /// Create new health monitor
    pub fn new(config: HealthMonitorConfig) -> Self {
        Self {
            config,
            health_checks: Arc::new(RwLock::new(HashMap::new())),
            missed_heartbeats: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Record heartbeat for service
    pub async fn record_heartbeat(&self, service_id: &ServiceId) {
        let mut health_checks = self.health_checks.write().await;
        health_checks.insert(service_id.clone(), SystemTime::now());
        
        // Reset missed heartbeats counter
        let mut missed = self.missed_heartbeats.write().await;
        missed.remove(service_id);
        
        debug!("Recorded heartbeat for service: {}", service_id);
    }

    /// Check service health status
    pub async fn check_health(&self, service_id: &ServiceId) -> HealthStatus {
        let health_checks = self.health_checks.read().await;
        let missed = self.missed_heartbeats.read().await;
        
        if let Some(last_heartbeat) = health_checks.get(service_id) {
            let elapsed = SystemTime::now()
                .duration_since(*last_heartbeat)
                .unwrap_or(Duration::from_secs(u64::MAX));
            
            if elapsed > self.config.heartbeat_interval * 2 {
                let missed_count = missed.get(service_id).unwrap_or(&0);
                if *missed_count >= self.config.max_missed_heartbeats {
                    HealthStatus::Unhealthy
                } else {
                    HealthStatus::Degraded
                }
            } else {
                HealthStatus::Healthy
            }
        } else {
            HealthStatus::Unknown
        }
    }

    /// Update missed heartbeat count
    pub async fn increment_missed_heartbeat(&self, service_id: &ServiceId) {
        let mut missed = self.missed_heartbeats.write().await;
        let count = missed.entry(service_id.clone()).or_insert(0);
        *count += 1;
        
        warn!("Missed heartbeat for service: {} (count: {})", service_id, count);
    }

    /// Get health statistics
    pub async fn get_health_stats(&self) -> HashMap<ServiceId, (SystemTime, u32)> {
        let health_checks = self.health_checks.read().await;
        let missed = self.missed_heartbeats.read().await;
        
        let mut stats = HashMap::new();
        for (service_id, last_heartbeat) in health_checks.iter() {
            let missed_count = missed.get(service_id).unwrap_or(&0);
            stats.insert(service_id.clone(), (*last_heartbeat, *missed_count));
        }
        stats
    }
}

/// Discovery protocol for automatic service discovery
#[derive(Debug)]
pub struct DiscoveryProtocol {
    config: DiscoveryConfig,
    discovered_services: Arc<RwLock<HashMap<ServiceId, ServiceInfo>>>,
    announcement_history: Arc<RwLock<BTreeMap<SystemTime, ServiceId>>>,
}

impl DiscoveryProtocol {
    /// Create new discovery protocol
    pub fn new(config: DiscoveryConfig) -> Self {
        Self {
            config,
            discovered_services: Arc::new(RwLock::new(HashMap::new())),
            announcement_history: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    /// Announce service availability
    pub async fn announce_service(&self, service_info: ServiceInfo) -> Result<(), MeshError> {
        let mut services = self.discovered_services.write().await;
        let mut history = self.announcement_history.write().await;
        
        // Check capacity
        if services.len() >= self.config.max_services {
            return Err(MeshError::DiscoveryError("Maximum services reached".to_string()));
        }
        
        let service_id = service_info.service_id.clone();
        services.insert(service_id.clone(), service_info);
        history.insert(SystemTime::now(), service_id.clone());
        
        info!("Announced service: {}", service_id);
        Ok(())
    }

    /// Discover services by capability
    pub async fn discover_services(&self, capability: &ServiceCapability) -> Vec<ServiceInfo> {
        let services = self.discovered_services.read().await;
        services
            .values()
            .filter(|service| service.capabilities.contains(capability))
            .cloned()
            .collect()
    }

    /// Get all discovered services
    pub async fn get_all_services(&self) -> Vec<ServiceInfo> {
        let services = self.discovered_services.read().await;
        services.values().cloned().collect()
    }

    /// Remove service from discovery
    pub async fn remove_service(&self, service_id: &ServiceId) -> Result<(), MeshError> {
        let mut services = self.discovered_services.write().await;
        if services.remove(service_id).is_some() {
            info!("Removed service from discovery: {}", service_id);
            Ok(())
        } else {
            Err(MeshError::ServiceNotFound(service_id.clone()))
        }
    }

    /// Cleanup old announcements
    pub async fn cleanup_old_announcements(&self, max_age: Duration) {
        let mut history = self.announcement_history.write().await;
        let cutoff = SystemTime::now() - max_age;
        
        let old_entries: Vec<_> = history
            .range(..cutoff)
            .map(|(time, _)| *time)
            .collect();
        
        for time in old_entries {
            history.remove(&time);
        }
    }
}

/// Main BPCI Mesh Coordinator
#[derive(Debug)]
pub struct BpciMeshCoordinator {
    config: MeshCoordinatorConfig,
    service_registry: Arc<RwLock<HashMap<ServiceId, ServiceInfo>>>,
    health_monitor: HealthMonitor,
    discovery_protocol: DiscoveryProtocol,
    transport: Arc<BpciTransport>,
    load_balancer: Arc<RwLock<HashMap<ServiceCapability, Vec<ServiceId>>>>,
    running: Arc<RwLock<bool>>,
}

impl BpciMeshCoordinator {
    /// Create new mesh coordinator
    pub fn new(config: MeshCoordinatorConfig) -> Result<Self, MeshError> {
        let bpci_config = BpciConfig {
            bind_address: config.bind_address,
            ..Default::default()
        };
        
        let transport = Arc::new(BpciTransport::new(bpci_config)?);
        let health_monitor = HealthMonitor::new(config.health_config.clone());
        let discovery_protocol = DiscoveryProtocol::new(config.discovery_config.clone());
        
        Ok(Self {
            config,
            service_registry: Arc::new(RwLock::new(HashMap::new())),
            health_monitor,
            discovery_protocol,
            transport,
            load_balancer: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(false)),
        })
    }

    /// Start the mesh coordinator
    pub async fn start(&self) -> Result<(), MeshError> {
        let mut running = self.running.write().await;
        if *running {
            return Ok(());
        }

        // Start transport layer
        self.transport.start()?;
        
        *running = true;
        info!("BPCI Mesh Coordinator started on {}", self.config.bind_address);
        
        // Start background tasks
        self.start_health_monitoring().await;
        self.start_discovery_announcements().await;
        
        Ok(())
    }

    /// Register a service in the mesh
    pub async fn register_service(&self, mut service_info: ServiceInfo) -> Result<(), MeshError> {
        let mut registry = self.service_registry.write().await;
        
        if registry.contains_key(&service_info.service_id) {
            return Err(MeshError::ServiceAlreadyExists(service_info.service_id));
        }
        
        // Set registration timestamp
        service_info.registered_at = SystemTime::now();
        service_info.last_heartbeat = SystemTime::now();
        
        let service_id = service_info.service_id.clone();
        let capabilities = service_info.capabilities.clone();
        
        // Register service
        registry.insert(service_id.clone(), service_info.clone());
        
        // Update load balancer
        let mut load_balancer = self.load_balancer.write().await;
        for capability in capabilities {
            load_balancer
                .entry(capability)
                .or_insert_with(Vec::new)
                .push(service_id.clone());
        }
        
        // Announce to discovery protocol
        self.discovery_protocol.announce_service(service_info).await?;
        
        // Record initial heartbeat
        self.health_monitor.record_heartbeat(&service_id).await;
        
        info!("Registered service: {}", service_id);
        Ok(())
    }

    /// Unregister a service from the mesh
    pub async fn unregister_service(&self, service_id: &ServiceId) -> Result<(), MeshError> {
        let mut registry = self.service_registry.write().await;
        
        if let Some(service_info) = registry.remove(service_id) {
            // Remove from load balancer
            let mut load_balancer = self.load_balancer.write().await;
            for capability in &service_info.capabilities {
                if let Some(services) = load_balancer.get_mut(capability) {
                    services.retain(|id| id != service_id);
                }
            }
            
            // Remove from discovery
            self.discovery_protocol.remove_service(service_id).await?;
            
            info!("Unregistered service: {}", service_id);
            Ok(())
        } else {
            Err(MeshError::ServiceNotFound(service_id.clone()))
        }
    }

    /// Get service by ID
    pub async fn get_service(&self, service_id: &ServiceId) -> Option<ServiceInfo> {
        let registry = self.service_registry.read().await;
        registry.get(service_id).cloned()
    }

    /// List all registered services
    pub async fn list_services(&self) -> Vec<ServiceInfo> {
        let registry = self.service_registry.read().await;
        registry.values().cloned().collect()
    }

    /// Find services by capability
    pub async fn find_services_by_capability(&self, capability: &ServiceCapability) -> Vec<ServiceInfo> {
        let registry = self.service_registry.read().await;
        registry
            .values()
            .filter(|service| service.capabilities.contains(capability))
            .cloned()
            .collect()
    }

    /// Get service for load balancing
    pub async fn get_service_for_capability(&self, capability: &ServiceCapability) -> Option<ServiceInfo> {
        let load_balancer = self.load_balancer.read().await;
        let registry = self.service_registry.read().await;
        
        if let Some(service_ids) = load_balancer.get(capability) {
            if service_ids.is_empty() {
                return None;
            }
            
            // Simple round-robin for now
            let index = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as usize % service_ids.len();
            
            let service_id = &service_ids[index];
            registry.get(service_id).cloned()
        } else {
            None
        }
    }

    /// Record service heartbeat
    pub async fn record_heartbeat(&self, service_id: &ServiceId) -> Result<(), MeshError> {
        // Check if service is registered
        let registry = self.service_registry.read().await;
        if !registry.contains_key(service_id) {
            return Err(MeshError::ServiceNotFound(service_id.clone()));
        }
        drop(registry);
        
        // Record heartbeat
        self.health_monitor.record_heartbeat(service_id).await;
        
        // Update service last_heartbeat
        let mut registry = self.service_registry.write().await;
        if let Some(service) = registry.get_mut(service_id) {
            service.last_heartbeat = SystemTime::now();
            service.health_status = HealthStatus::Healthy;
        }
        
        Ok(())
    }

    /// Get mesh statistics
    pub async fn get_mesh_stats(&self) -> MeshStats {
        let registry = self.service_registry.read().await;
        let health_stats = self.health_monitor.get_health_stats().await;
        
        let total_services = registry.len();
        let healthy_services = registry
            .values()
            .filter(|s| s.health_status == HealthStatus::Healthy)
            .count();
        
        let services_by_capability = {
            let mut map = HashMap::new();
            for service in registry.values() {
                for capability in &service.capabilities {
                    *map.entry(capability.clone()).or_insert(0) += 1;
                }
            }
            map
        };
        
        MeshStats {
            total_services,
            healthy_services,
            services_by_capability,
            coordinator_uptime: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default(),
        }
    }

    /// Start health monitoring background task
    async fn start_health_monitoring(&self) {
        let health_monitor = self.health_monitor.clone();
        let registry = self.service_registry.clone();
        let interval = self.config.health_config.heartbeat_interval;
        
        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            
            loop {
                interval_timer.tick().await;
                
                let services: Vec<_> = {
                    let registry = registry.read().await;
                    registry.keys().cloned().collect()
                };
                
                for service_id in services {
                    let health_status = health_monitor.check_health(&service_id).await;
                    
                    if health_status != HealthStatus::Healthy {
                        health_monitor.increment_missed_heartbeat(&service_id).await;
                        
                        // Update service health status
                        let mut registry = registry.write().await;
                        if let Some(service) = registry.get_mut(&service_id) {
                            service.health_status = health_status;
                        }
                    }
                }
            }
        });
    }

    /// Start discovery announcements background task
    async fn start_discovery_announcements(&self) {
        let discovery = self.discovery_protocol.clone();
        let interval = self.config.discovery_config.announcement_interval;
        
        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            
            loop {
                interval_timer.tick().await;
                
                // Cleanup old announcements
                discovery.cleanup_old_announcements(interval * 10).await;
            }
        });
    }

    /// Stop the mesh coordinator
    pub async fn stop(&self) -> Result<(), MeshError> {
        let mut running = self.running.write().await;
        if !*running {
            return Ok(());
        }
        
        *running = false;
        self.transport.shutdown()?;
        
        info!("BPCI Mesh Coordinator stopped");
        Ok(())
    }

    /// Check if coordinator is running
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }
}

/// Mesh statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshStats {
    pub total_services: usize,
    pub healthy_services: usize,
    pub services_by_capability: HashMap<ServiceCapability, usize>,
    pub coordinator_uptime: Duration,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[tokio::test]
    async fn test_mesh_coordinator_creation() {
        let config = MeshCoordinatorConfig::default();
        let coordinator = BpciMeshCoordinator::new(config).unwrap();
        assert!(!coordinator.is_running().await);
    }

    #[tokio::test]
    async fn test_service_registration() {
        let config = MeshCoordinatorConfig::default();
        let coordinator = BpciMeshCoordinator::new(config).unwrap();
        
        let service_info = ServiceInfo {
            service_id: "test-service".to_string(),
            endpoint: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            capabilities: vec![ServiceCapability::BpciMessaging],
            health_status: HealthStatus::Healthy,
            last_heartbeat: SystemTime::now(),
            metadata: HashMap::new(),
            version: "1.0.0".to_string(),
            weight: 100,
            registered_at: SystemTime::now(),
        };
        
        coordinator.register_service(service_info.clone()).await.unwrap();
        
        let retrieved = coordinator.get_service(&service_info.service_id).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().service_id, service_info.service_id);
    }

    #[tokio::test]
    async fn test_service_discovery_by_capability() {
        let config = MeshCoordinatorConfig::default();
        let coordinator = BpciMeshCoordinator::new(config).unwrap();
        
        // Register services with different capabilities
        let service1 = ServiceInfo {
            service_id: "bpci-service".to_string(),
            endpoint: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            capabilities: vec![ServiceCapability::BpciMessaging],
            health_status: HealthStatus::Healthy,
            last_heartbeat: SystemTime::now(),
            metadata: HashMap::new(),
            version: "1.0.0".to_string(),
            weight: 100,
            registered_at: SystemTime::now(),
        };
        
        let service2 = ServiceInfo {
            service_id: "docklock-service".to_string(),
            endpoint: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081),
            capabilities: vec![ServiceCapability::DockLockDeployment],
            health_status: HealthStatus::Healthy,
            last_heartbeat: SystemTime::now(),
            metadata: HashMap::new(),
            version: "1.0.0".to_string(),
            weight: 100,
            registered_at: SystemTime::now(),
        };
        
        coordinator.register_service(service1).await.unwrap();
        coordinator.register_service(service2).await.unwrap();
        
        let bpci_services = coordinator
            .find_services_by_capability(&ServiceCapability::BpciMessaging)
            .await;
        assert_eq!(bpci_services.len(), 1);
        assert_eq!(bpci_services[0].service_id, "bpci-service");
        
        let docklock_services = coordinator
            .find_services_by_capability(&ServiceCapability::DockLockDeployment)
            .await;
        assert_eq!(docklock_services.len(), 1);
        assert_eq!(docklock_services[0].service_id, "docklock-service");
    }

    #[tokio::test]
    async fn test_health_monitoring() {
        let config = HealthMonitorConfig::default();
        let health_monitor = HealthMonitor::new(config);
        
        let service_id = "test-service".to_string();
        
        // Initially unknown
        assert_eq!(health_monitor.check_health(&service_id).await, HealthStatus::Unknown);
        
        // Record heartbeat
        health_monitor.record_heartbeat(&service_id).await;
        assert_eq!(health_monitor.check_health(&service_id).await, HealthStatus::Healthy);
        
        // Increment missed heartbeat
        health_monitor.increment_missed_heartbeat(&service_id).await;
        
        let stats = health_monitor.get_health_stats().await;
        assert!(stats.contains_key(&service_id));
        assert_eq!(stats[&service_id].1, 1); // 1 missed heartbeat
    }

    #[tokio::test]
    async fn test_discovery_protocol() {
        let config = DiscoveryConfig::default();
        let discovery = DiscoveryProtocol::new(config);
        
        let service_info = ServiceInfo {
            service_id: "discovered-service".to_string(),
            endpoint: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            capabilities: vec![ServiceCapability::BpciMessaging],
            health_status: HealthStatus::Healthy,
            last_heartbeat: SystemTime::now(),
            metadata: HashMap::new(),
            version: "1.0.0".to_string(),
            weight: 100,
            registered_at: SystemTime::now(),
        };
        
        discovery.announce_service(service_info.clone()).await.unwrap();
        
        let discovered = discovery
            .discover_services(&ServiceCapability::BpciMessaging)
            .await;
        assert_eq!(discovered.len(), 1);
        assert_eq!(discovered[0].service_id, service_info.service_id);
        
        let all_services = discovery.get_all_services().await;
        assert_eq!(all_services.len(), 1);
    }

    #[tokio::test]
    async fn test_load_balancing() {
        let config = MeshCoordinatorConfig::default();
        let coordinator = BpciMeshCoordinator::new(config).unwrap();
        
        // Register multiple services with same capability
        for i in 0..3 {
            let service_info = ServiceInfo {
                service_id: format!("service-{}", i),
                endpoint: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080 + i),
                capabilities: vec![ServiceCapability::BpciMessaging],
                health_status: HealthStatus::Healthy,
                last_heartbeat: SystemTime::now(),
                metadata: HashMap::new(),
                version: "1.0.0".to_string(),
                weight: 100,
                registered_at: SystemTime::now(),
            };
            coordinator.register_service(service_info).await.unwrap();
        }
        
        // Test load balancing
        let service = coordinator
            .get_service_for_capability(&ServiceCapability::BpciMessaging)
            .await;
        assert!(service.is_some());
        
        let stats = coordinator.get_mesh_stats().await;
        assert_eq!(stats.total_services, 3);
        assert_eq!(stats.healthy_services, 3);
        assert_eq!(stats.services_by_capability[&ServiceCapability::BpciMessaging], 3);
    }
}
