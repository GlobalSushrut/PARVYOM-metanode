// BPI Core Integration Bridge
// Bridges BPCI Enterprise system with existing BPI Core blockchain OS kernel
// Provides seamless integration between enterprise services and core OS infrastructure

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use anyhow::{Result, anyhow};
use tracing::{info, warn, debug, error};

// Import from central orchestration
use crate::central_orchestration::BPCICentralOrchestrator;
use crate::registry::{BpciRegistry, NodeRegistration};

pub mod kernel_bridge;
pub mod service_mapper;
pub mod resource_coordinator;

pub use kernel_bridge::BlockchainOSKernelBridge;
pub use service_mapper::EnterpriseServiceMapper;
pub use resource_coordinator::ResourceCoordinator;

/// Main BPI Core Integration System
/// Bridges BPCI Enterprise with existing BPI Core blockchain OS kernel
#[derive(Debug)]
pub struct BpiCoreIntegration {
    /// Integration identifier
    pub integration_id: String,
    
    /// Bridge to BPI Core blockchain OS kernel
    pub kernel_bridge: Arc<BlockchainOSKernelBridge>,
    
    /// Service mapper for enterprise services
    pub service_mapper: Arc<EnterpriseServiceMapper>,
    
    /// Resource coordinator between systems
    pub resource_coordinator: Arc<ResourceCoordinator>,
    
    /// Integration state
    pub integration_state: Arc<RwLock<IntegrationState>>,
    
    /// Active service mappings
    pub active_mappings: Arc<Mutex<HashMap<String, ServiceMapping>>>,
}

/// Integration state tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationState {
    /// Integration status
    pub status: IntegrationStatus,
    /// Initialization timestamp
    pub initialized_at: DateTime<Utc>,
    /// Last sync with BPI Core
    pub last_sync: DateTime<Utc>,
    /// Active enterprise services count
    pub active_services: u32,
    /// Resource utilization from BPI Core
    pub core_resource_utilization: f64,
    /// Integration health metrics
    pub health_metrics: IntegrationHealthMetrics,
}

/// Integration status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationStatus {
    Initializing,
    Connected,
    Syncing,
    Active,
    Degraded,
    Disconnected,
}

/// Integration health metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationHealthMetrics {
    /// Connection latency to BPI Core (ms)
    pub connection_latency: f64,
    /// Sync success rate (percentage)
    pub sync_success_rate: f64,
    /// Resource coordination efficiency
    pub coordination_efficiency: f64,
    /// Service mapping accuracy
    pub mapping_accuracy: f64,
}

/// Service mapping between enterprise and core services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMapping {
    /// Enterprise service identifier
    pub enterprise_service_id: String,
    /// BPI Core process identifier
    pub core_process_id: String,
    /// Service type
    pub service_type: EnterpriseServiceType,
    /// Resource requirements
    pub resource_requirements: ResourceRequirements,
    /// Security context
    pub security_context: SecurityContext,
    /// Mapping status
    pub status: MappingStatus,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
}

/// Enterprise service types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EnterpriseServiceType {
    GovernanceAPI,
    OrchestrationService,
    CompanyManagement,
    SAPIMeshService,
    AuditService,
    MultiSigWorkflow,
    AutomationRule,
}

/// Resource requirements for service mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// CPU percentage required
    pub cpu_percent: f64,
    /// Memory in bytes
    pub memory_bytes: u64,
    /// Network bandwidth in bytes/sec
    pub network_bandwidth: u64,
    /// Storage in bytes
    pub storage_bytes: u64,
}

/// Security context for enterprise services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    /// Security level required
    pub security_level: SecurityLevel,
    /// Quantum encryption required
    pub quantum_encryption: bool,
    /// Audit logging enabled
    pub audit_logging: bool,
    /// Access permissions
    pub permissions: Vec<String>,
}

/// Security levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Public,
    Restricted,
    Confidential,
    Enterprise,
}

/// Service mapping status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MappingStatus {
    Pending,
    Active,
    Suspended,
    Failed,
    Terminated,
}

impl BpiCoreIntegration {
    /// Create new BPI Core integration
    pub async fn new(orchestrator: Arc<BPCICentralOrchestrator>) -> Result<Self> {
        let integration_id = format!("bpi_integration_{}", Uuid::new_v4());
        let now = Utc::now();
        
        // Initialize integration components
        let kernel_bridge = Arc::new(BlockchainOSKernelBridge::new().await?);
        let service_mapper = Arc::new(EnterpriseServiceMapper::new().await?);
        let resource_coordinator = Arc::new(ResourceCoordinator::new(orchestrator).await?);
        
        // Initialize integration state
        let integration_state = Arc::new(RwLock::new(IntegrationState {
            status: IntegrationStatus::Initializing,
            initialized_at: now,
            last_sync: now,
            active_services: 0,
            core_resource_utilization: 0.0,
            health_metrics: IntegrationHealthMetrics {
                connection_latency: 0.0,
                sync_success_rate: 100.0,
                coordination_efficiency: 100.0,
                mapping_accuracy: 100.0,
            },
        }));
        
        let active_mappings = Arc::new(Mutex::new(HashMap::new()));
        
        Ok(BpiCoreIntegration {
            integration_id,
            kernel_bridge,
            service_mapper,
            resource_coordinator,
            integration_state,
            active_mappings,
        })
    }
    
    /// Initialize integration with BPI Core
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing BPI Core integration: {}", self.integration_id);
        
        // Update status to connecting
        {
            let mut state = self.integration_state.write().map_err(|_| anyhow!("State lock error"))?;
            state.status = IntegrationStatus::Connected;
        }
        
        // Initialize kernel bridge
        self.kernel_bridge.connect().await?;
        
        // Initialize service mapper
        self.service_mapper.initialize().await?;
        
        // Initialize resource coordinator
        self.resource_coordinator.initialize().await?;
        
        // Update status to active
        {
            let mut state = self.integration_state.write().map_err(|_| anyhow!("State lock error"))?;
            state.status = IntegrationStatus::Active;
            state.last_sync = Utc::now();
        }
        
        info!("BPI Core integration initialized successfully");
        Ok(())
    }
    
    /// Register enterprise service with BPI Core
    pub async fn register_enterprise_service(
        &self,
        service_type: EnterpriseServiceType,
        resource_requirements: ResourceRequirements,
        security_context: SecurityContext,
    ) -> Result<String> {
        let service_id = format!("enterprise_service_{}", Uuid::new_v4());
        
        // Map service to BPI Core process
        let core_process_id = self.service_mapper.map_service(
            &service_id,
            &service_type,
            &resource_requirements,
            &security_context,
        ).await?;
        
        // Create service mapping
        let mapping = ServiceMapping {
            enterprise_service_id: service_id.clone(),
            core_process_id: core_process_id.clone(),
            service_type,
            resource_requirements,
            security_context,
            status: MappingStatus::Active,
            created_at: Utc::now(),
        };
        
        // Store mapping
        {
            let mut mappings = self.active_mappings.lock().await;
            mappings.insert(service_id.clone(), mapping);
        }
        
        // Update service count
        {
            let mut state = self.integration_state.write().map_err(|_| anyhow!("State lock error"))?;
            state.active_services += 1;
        }
        
        info!("Registered enterprise service: {} -> {}", service_id, core_process_id);
        Ok(service_id)
    }
    
    /// Sync with BPI Core kernel
    pub async fn sync_with_core(&self) -> Result<()> {
        let start_time = std::time::Instant::now();
        
        // Sync kernel state
        let kernel_status = self.kernel_bridge.get_kernel_status().await?;
        
        // Sync resource utilization
        let resource_metrics = self.resource_coordinator.sync_resources().await?;
        
        // Update integration state
        {
            let mut state = self.integration_state.write().map_err(|_| anyhow!("State lock error"))?;
            state.last_sync = Utc::now();
            state.core_resource_utilization = resource_metrics.total_utilization;
            
            // Update health metrics
            let sync_duration = start_time.elapsed().as_millis() as f64;
            state.health_metrics.connection_latency = sync_duration;
            state.health_metrics.sync_success_rate = 100.0; // Success if we reach here
        }
        
        debug!("Synced with BPI Core successfully");
        Ok(())
    }
    
    /// Get integration status
    pub async fn get_integration_status(&self) -> Result<IntegrationState> {
        let state = self.integration_state.read().map_err(|_| anyhow!("State lock error"))?;
        Ok(state.clone())
    }
    
    /// Get active service mappings
    pub async fn get_active_mappings(&self) -> Result<Vec<ServiceMapping>> {
        let mappings = self.active_mappings.lock().await;
        Ok(mappings.values().cloned().collect())
    }
    
    /// Shutdown integration
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down BPI Core integration");
        
        // Update status
        {
            let mut state = self.integration_state.write().map_err(|_| anyhow!("State lock error"))?;
            state.status = IntegrationStatus::Disconnected;
        }
        
        // Shutdown components
        self.resource_coordinator.shutdown().await?;
        self.service_mapper.shutdown().await?;
        self.kernel_bridge.disconnect().await?;
        
        info!("BPI Core integration shutdown complete");
        Ok(())
    }
}

/// Integration error types
#[derive(Debug, thiserror::Error)]
pub enum IntegrationError {
    #[error("Kernel bridge error: {0}")]
    KernelBridgeError(String),
    #[error("Service mapping error: {0}")]
    ServiceMappingError(String),
    #[error("Resource coordination error: {0}")]
    ResourceCoordinationError(String),
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Sync error: {0}")]
    SyncError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::central_orchestration::BPCICentralOrchestrator;
    
    #[tokio::test]
    async fn test_integration_initialization() {
        // This test would require a mock orchestrator
        // Implementation depends on actual orchestrator structure
    }
    
    #[tokio::test]
    async fn test_service_registration() {
        // Test service registration flow
        // Implementation depends on actual service mapper
    }
}
