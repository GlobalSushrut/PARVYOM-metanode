//! BPI Immutable OS Integration Bridge
//! 
//! Bridges the Blockchain OS Kernel with the existing BPI Immutable OS infrastructure,
//! enabling seamless integration between blockchain-controlled operations and the
//! immutable filesystem, vPod networking, and NXOS DRX services.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};
use uuid::Uuid;
use tracing::{info, warn, debug, error};

use super::{BlockchainOSKernel, ProcessInfo, ProcessType, SecurityContext, IsolationLevel};

/// BPI Immutable OS Integration Bridge
/// Provides seamless integration between Blockchain OS Kernel and BPI Immutable OS
#[derive(Debug)]
pub struct BpiImmutableOSIntegration {
    /// Integration configuration
    config: Arc<RwLock<IntegrationConfig>>,
    
    /// Active service mappings
    service_mappings: Arc<Mutex<HashMap<String, ServiceMapping>>>,
    
    /// Filesystem integration state
    filesystem_state: Arc<RwLock<FilesystemIntegrationState>>,
    
    /// Network integration state
    network_state: Arc<RwLock<NetworkIntegrationState>>,
    
    /// Integration statistics
    stats: Arc<RwLock<IntegrationStats>>,
}

/// Integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub bpi_namespace_root: String,
    pub enable_filesystem_integration: bool,
    pub enable_network_integration: bool,
    pub enable_service_mesh: bool,
    pub enable_quantum_security: bool,
    pub core_services_ports: HashMap<String, u16>,
    pub vm_cluster_ports: HashMap<String, u16>,
    pub security_services_ports: HashMap<String, u16>,
}

/// Service mapping between kernel processes and immutable OS services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMapping {
    pub process_id: String,
    pub service_name: String,
    pub service_port: u16,
    pub service_type: ImmutableOSServiceType,
    pub integration_status: IntegrationStatus,
    pub health_status: HealthStatus,
    pub last_health_check: u64,
}

/// Types of BPI Immutable OS services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImmutableOSServiceType {
    VMServer,           // BPI VM Server (port 7777)
    HttpCage,           // HTTP Cage (port 8888)
    ShadowRegistry,     // Shadow Registry (port 8080)
    ZKLockMobile,       // ZKLock Mobile (port 8081)
    FilesystemManager,  // Advanced Filesystem Manager
    NetworkConfigurator, // vPod Network Configurator
    SecurityHardening,  // Security Hardening Engine
    AtomicUpdates,      // Atomic Update System
    HardwareDetection,  // Hardware Detection Engine
}

/// Integration status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationStatus {
    Initializing,
    Connected,
    Synchronized,
    Degraded,
    Failed,
}

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

/// Filesystem integration state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemIntegrationState {
    pub namespace_mounted: bool,
    pub core_paths_available: bool,
    pub data_layer_accessible: bool,
    pub config_management_active: bool,
    pub runtime_state_synchronized: bool,
    pub immutable_overlays_count: u32,
    pub last_filesystem_sync: u64,
}

/// Network integration state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkIntegrationState {
    pub vpod_network_active: bool,
    pub trust_weighted_routing_enabled: bool,
    pub qlock_session_steering_active: bool,
    pub proof_of_forward_enabled: bool,
    pub service_mesh_configured: bool,
    pub active_connections: u32,
    pub last_network_sync: u64,
}

/// Integration statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationStats {
    pub total_services_integrated: u32,
    pub healthy_services: u32,
    pub degraded_services: u32,
    pub failed_services: u32,
    pub filesystem_operations: u64,
    pub network_operations: u64,
    pub security_validations: u64,
    pub uptime_seconds: u64,
    pub last_stats_update: u64,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        let mut core_services_ports = HashMap::new();
        core_services_ports.insert("vm_server".to_string(), 7777);
        core_services_ports.insert("http_cage".to_string(), 8888);
        core_services_ports.insert("shadow_registry".to_string(), 8080);
        core_services_ports.insert("zklock_mobile".to_string(), 8081);

        let mut vm_cluster_ports = HashMap::new();
        vm_cluster_ports.insert("cluster_coordinator".to_string(), 9000);
        vm_cluster_ports.insert("vm_orchestrator".to_string(), 9001);
        vm_cluster_ports.insert("resource_manager".to_string(), 9002);

        let mut security_services_ports = HashMap::new();
        security_services_ports.insert("quantum_enforcer".to_string(), 9100);
        security_services_ports.insert("security_monitor".to_string(), 9101);
        security_services_ports.insert("audit_collector".to_string(), 9102);

        Self {
            bpi_namespace_root: "/bpi".to_string(),
            enable_filesystem_integration: true,
            enable_network_integration: true,
            enable_service_mesh: true,
            enable_quantum_security: true,
            core_services_ports,
            vm_cluster_ports,
            security_services_ports,
        }
    }
}

impl Default for FilesystemIntegrationState {
    fn default() -> Self {
        Self {
            namespace_mounted: false,
            core_paths_available: false,
            data_layer_accessible: false,
            config_management_active: false,
            runtime_state_synchronized: false,
            immutable_overlays_count: 0,
            last_filesystem_sync: 0,
        }
    }
}

impl Default for NetworkIntegrationState {
    fn default() -> Self {
        Self {
            vpod_network_active: false,
            trust_weighted_routing_enabled: false,
            qlock_session_steering_active: false,
            proof_of_forward_enabled: false,
            service_mesh_configured: false,
            active_connections: 0,
            last_network_sync: 0,
        }
    }
}

impl Default for IntegrationStats {
    fn default() -> Self {
        Self {
            total_services_integrated: 0,
            healthy_services: 0,
            degraded_services: 0,
            failed_services: 0,
            filesystem_operations: 0,
            network_operations: 0,
            security_validations: 0,
            uptime_seconds: 0,
            last_stats_update: 0,
        }
    }
}

impl BpiImmutableOSIntegration {
    /// Create a new BPI Immutable OS integration bridge
    pub fn new() -> Result<Self> {
        info!("Creating BPI Immutable OS integration bridge");

        let config = Arc::new(RwLock::new(IntegrationConfig::default()));
        let service_mappings = Arc::new(Mutex::new(HashMap::new()));
        let filesystem_state = Arc::new(RwLock::new(FilesystemIntegrationState::default()));
        let network_state = Arc::new(RwLock::new(NetworkIntegrationState::default()));
        let stats = Arc::new(RwLock::new(IntegrationStats::default()));

        Ok(Self {
            config,
            service_mappings,
            filesystem_state,
            network_state,
            stats,
        })
    }

    /// Initialize the integration bridge
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing BPI Immutable OS integration bridge");

        // Initialize filesystem integration
        self.initialize_filesystem_integration().await?;

        // Initialize network integration
        self.initialize_network_integration().await?;

        // Initialize service mappings
        self.initialize_service_mappings().await?;

        // Start health monitoring
        self.start_health_monitoring().await?;

        info!("BPI Immutable OS integration bridge initialized successfully");
        Ok(())
    }

    /// Initialize filesystem integration with BPI Immutable OS
    async fn initialize_filesystem_integration(&self) -> Result<()> {
        info!("Initializing filesystem integration");

        let config = self.config.read().map_err(|e| anyhow!("Failed to read config: {}", e))?;
        
        if !config.enable_filesystem_integration {
            info!("Filesystem integration disabled in configuration");
            return Ok(());
        }

        // Check if BPI namespace is mounted
        let namespace_exists = tokio::fs::metadata(&config.bpi_namespace_root).await.is_ok();
        
        if namespace_exists {
            info!("BPI namespace found at: {}", config.bpi_namespace_root);
            
            // Update filesystem state
            let mut fs_state = self.filesystem_state.write()
                .map_err(|e| anyhow!("Failed to write filesystem state: {}", e))?;
            
            fs_state.namespace_mounted = true;
            fs_state.core_paths_available = self.verify_core_paths(&config.bpi_namespace_root).await?;
            fs_state.data_layer_accessible = self.verify_data_layer(&config.bpi_namespace_root).await?;
            fs_state.config_management_active = self.verify_config_management(&config.bpi_namespace_root).await?;
            fs_state.last_filesystem_sync = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        } else {
            warn!("BPI namespace not found at: {}", config.bpi_namespace_root);
        }

        Ok(())
    }

    /// Initialize network integration with vPod networking
    async fn initialize_network_integration(&self) -> Result<()> {
        info!("Initializing network integration");

        let config = self.config.read().map_err(|e| anyhow!("Failed to read config: {}", e))?;
        
        if !config.enable_network_integration {
            info!("Network integration disabled in configuration");
            return Ok(());
        }

        // Check core services availability
        let vm_server_available = self.check_service_availability("127.0.0.1", 7777).await;
        let http_cage_available = self.check_service_availability("127.0.0.1", 8888).await;
        let shadow_registry_available = self.check_service_availability("127.0.0.1", 8080).await;
        let zklock_mobile_available = self.check_service_availability("127.0.0.1", 8081).await;

        // Update network state
        let mut net_state = self.network_state.write()
            .map_err(|e| anyhow!("Failed to write network state: {}", e))?;
        
        net_state.vpod_network_active = vm_server_available || http_cage_available;
        net_state.service_mesh_configured = vm_server_available && http_cage_available && shadow_registry_available;
        net_state.last_network_sync = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if net_state.vpod_network_active {
            info!("vPod network integration active");
        } else {
            warn!("vPod network services not available");
        }

        Ok(())
    }

    /// Initialize service mappings between kernel processes and immutable OS services
    async fn initialize_service_mappings(&self) -> Result<()> {
        info!("Initializing service mappings");

        let config = self.config.read().map_err(|e| anyhow!("Failed to read config: {}", e))?;
        let mut mappings = self.service_mappings.lock().await;

        // Create mappings for core services
        for (service_name, port) in &config.core_services_ports {
            let service_type = match service_name.as_str() {
                "vm_server" => ImmutableOSServiceType::VMServer,
                "http_cage" => ImmutableOSServiceType::HttpCage,
                "shadow_registry" => ImmutableOSServiceType::ShadowRegistry,
                "zklock_mobile" => ImmutableOSServiceType::ZKLockMobile,
                _ => continue,
            };

            let mapping = ServiceMapping {
                process_id: Uuid::new_v4().to_string(),
                service_name: service_name.clone(),
                service_port: *port,
                service_type,
                integration_status: IntegrationStatus::Initializing,
                health_status: HealthStatus::Unknown,
                last_health_check: 0,
            };

            mappings.insert(service_name.clone(), mapping);
        }

        info!("Created {} service mappings", mappings.len());
        Ok(())
    }

    /// Start health monitoring for integrated services
    async fn start_health_monitoring(&self) -> Result<()> {
        info!("Starting health monitoring");

        // Clone necessary data for the monitoring task
        let service_mappings = Arc::clone(&self.service_mappings);
        let stats = Arc::clone(&self.stats);

        // Spawn health monitoring task
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                if let Err(e) = Self::perform_health_checks(&service_mappings, &stats).await {
                    error!("Health check failed: {}", e);
                }
            }
        });

        Ok(())
    }

    /// Perform health checks on all integrated services
    async fn perform_health_checks(
        service_mappings: &Arc<Mutex<HashMap<String, ServiceMapping>>>,
        stats: &Arc<RwLock<IntegrationStats>>,
    ) -> Result<()> {
        let mut mappings = service_mappings.lock().await;
        let mut healthy_count = 0;
        let mut degraded_count = 0;
        let mut failed_count = 0;

        for (_, mapping) in mappings.iter_mut() {
            let is_healthy = Self::check_service_health(&mapping.service_name, mapping.service_port).await;
            
            mapping.health_status = if is_healthy {
                healthy_count += 1;
                HealthStatus::Healthy
            } else {
                failed_count += 1;
                HealthStatus::Critical
            };
            
            mapping.last_health_check = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }

        // Update statistics
        if let Ok(mut stats_guard) = stats.write() {
            stats_guard.healthy_services = healthy_count;
            stats_guard.degraded_services = degraded_count;
            stats_guard.failed_services = failed_count;
            stats_guard.total_services_integrated = mappings.len() as u32;
            stats_guard.last_stats_update = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }

        debug!("Health check completed: {} healthy, {} degraded, {} failed", 
               healthy_count, degraded_count, failed_count);

        Ok(())
    }

    /// Check if a service is available on the given host and port
    async fn check_service_availability(&self, host: &str, port: u16) -> bool {
        match tokio::net::TcpStream::connect(format!("{}:{}", host, port)).await {
            Ok(_) => {
                debug!("Service available at {}:{}", host, port);
                true
            }
            Err(_) => {
                debug!("Service not available at {}:{}", host, port);
                false
            }
        }
    }

    /// Check service health
    async fn check_service_health(service_name: &str, port: u16) -> bool {
        // For now, just check if the port is listening
        // In a real implementation, this would make HTTP health check requests
        match tokio::net::TcpStream::connect(format!("127.0.0.1:{}", port)).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Verify core paths in BPI namespace
    async fn verify_core_paths(&self, namespace_root: &str) -> Result<bool> {
        let core_paths = vec![
            format!("{}/core", namespace_root),
            format!("{}/nxos", namespace_root),
            format!("{}/data", namespace_root),
            format!("{}/config", namespace_root),
            format!("{}/runtime", namespace_root),
        ];

        for path in core_paths {
            if tokio::fs::metadata(&path).await.is_err() {
                debug!("Core path not found: {}", path);
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Verify data layer accessibility
    async fn verify_data_layer(&self, namespace_root: &str) -> Result<bool> {
        let data_path = format!("{}/data", namespace_root);
        match tokio::fs::read_dir(&data_path).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Verify config management
    async fn verify_config_management(&self, namespace_root: &str) -> Result<bool> {
        let config_path = format!("{}/config", namespace_root);
        match tokio::fs::read_dir(&config_path).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Get integration status
    pub async fn get_integration_status(&self) -> Result<IntegrationStatus> {
        let mappings = self.service_mappings.lock().await;
        let healthy_services = mappings.values()
            .filter(|m| matches!(m.health_status, HealthStatus::Healthy))
            .count();

        let total_services = mappings.len();

        if total_services == 0 {
            return Ok(IntegrationStatus::Initializing);
        }

        let health_ratio = healthy_services as f64 / total_services as f64;

        Ok(match health_ratio {
            r if r >= 0.9 => IntegrationStatus::Synchronized,
            r if r >= 0.7 => IntegrationStatus::Connected,
            r if r >= 0.3 => IntegrationStatus::Degraded,
            _ => IntegrationStatus::Failed,
        })
    }

    /// Get integration statistics
    pub fn get_integration_stats(&self) -> Result<IntegrationStats> {
        let stats = self.stats.read()
            .map_err(|e| anyhow!("Failed to read stats: {}", e))?;
        Ok(stats.clone())
    }

    /// Get filesystem integration state
    pub fn get_filesystem_state(&self) -> Result<FilesystemIntegrationState> {
        let state = self.filesystem_state.read()
            .map_err(|e| anyhow!("Failed to read filesystem state: {}", e))?;
        Ok(state.clone())
    }

    /// Get network integration state
    pub fn get_network_state(&self) -> Result<NetworkIntegrationState> {
        let state = self.network_state.read()
            .map_err(|e| anyhow!("Failed to read network state: {}", e))?;
        Ok(state.clone())
    }

    /// Get service mappings
    pub async fn get_service_mappings(&self) -> Result<HashMap<String, ServiceMapping>> {
        let mappings = self.service_mappings.lock().await;
        Ok(mappings.clone())
    }

    /// Shutdown the integration bridge
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down BPI Immutable OS integration bridge");

        // Update integration status to indicate shutdown
        let mut mappings = self.service_mappings.lock().await;
        for (_, mapping) in mappings.iter_mut() {
            mapping.integration_status = IntegrationStatus::Failed;
        }

        info!("BPI Immutable OS integration bridge shutdown complete");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain_os_kernel::BlockchainOSKernel;

    #[tokio::test]
    async fn test_integration_bridge_creation() {
        let kernel = Arc::new(BlockchainOSKernel::new().await.unwrap());
        let bridge = BpiImmutableOSIntegration::new();
        assert!(bridge.is_ok());
    }

    #[tokio::test]
    async fn test_service_availability_check() {
        let kernel = Arc::new(BlockchainOSKernel::new().await.unwrap());
        let bridge = BpiImmutableOSIntegration::new().unwrap();
        
        // This should fail since no service is running on port 65534
        let available = bridge.check_service_availability("127.0.0.1", 65534).await;
        assert!(!available);
    }

    #[tokio::test]
    async fn test_integration_status() {
        let kernel = Arc::new(BlockchainOSKernel::new().await.unwrap());
        let bridge = BpiImmutableOSIntegration::new().unwrap();
        
        let status = bridge.get_integration_status().await.unwrap();
        assert!(matches!(status, IntegrationStatus::Initializing));
    }

    #[tokio::test]
    async fn test_stats_retrieval() {
        let kernel = Arc::new(BlockchainOSKernel::new().await.unwrap());
        let bridge = BpiImmutableOSIntegration::new().unwrap();
        
        let stats = bridge.get_integration_stats().unwrap();
        assert_eq!(stats.total_services_integrated, 0);
    }
}
