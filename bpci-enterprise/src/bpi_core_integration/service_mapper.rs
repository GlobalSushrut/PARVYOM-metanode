// Enterprise Service Mapper
// Maps BPCI Enterprise services to BPI Core blockchain OS processes
// Provides intelligent service-to-process mapping and lifecycle management

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use anyhow::{Result, anyhow};
use tracing::{info, warn, debug, error};

use super::{EnterpriseServiceType, ResourceRequirements, SecurityContext};
use super::kernel_bridge::{BlockchainOSKernelBridge, ProcessType, ResourceAllocation, ProcessPriority};

/// Enterprise Service Mapper
/// Intelligently maps enterprise services to appropriate BPI Core processes
#[derive(Debug)]
pub struct EnterpriseServiceMapper {
    /// Mapper identifier
    pub mapper_id: String,
    
    /// Service mapping rules
    pub mapping_rules: Arc<RwLock<HashMap<EnterpriseServiceType, MappingRule>>>,
    
    /// Active service mappings
    pub active_mappings: Arc<Mutex<HashMap<String, ServiceProcessMapping>>>,
    
    /// Mapping statistics
    pub mapping_stats: Arc<RwLock<MappingStatistics>>,
    
    /// Reference to kernel bridge
    pub kernel_bridge: Option<Arc<BlockchainOSKernelBridge>>,
}

/// Mapping rule for enterprise services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingRule {
    /// Target process type in BPI Core
    pub target_process_type: ProcessType,
    /// Default resource allocation
    pub default_resources: ResourceAllocation,
    /// Required security level
    pub required_security: SecurityLevel,
    /// Scaling policy
    pub scaling_policy: ScalingPolicy,
    /// Priority level
    pub priority: ProcessPriority,
}

/// Security level requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Basic,
    Standard,
    Enhanced,
    Maximum,
}

/// Scaling policy for services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingPolicy {
    /// Minimum instances
    pub min_instances: u32,
    /// Maximum instances
    pub max_instances: u32,
    /// Auto-scaling enabled
    pub auto_scaling: bool,
    /// CPU threshold for scaling
    pub cpu_threshold: f64,
    /// Memory threshold for scaling
    pub memory_threshold: f64,
}

/// Service to process mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceProcessMapping {
    /// Service identifier
    pub service_id: String,
    /// Process identifier in BPI Core
    pub process_id: String,
    /// Service type
    pub service_type: EnterpriseServiceType,
    /// Mapping status
    pub status: MappingStatus,
    /// Resource allocation
    pub allocated_resources: ResourceAllocation,
    /// Performance metrics
    pub performance_metrics: ServicePerformanceMetrics,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated
    pub updated_at: DateTime<Utc>,
}

/// Mapping status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MappingStatus {
    Initializing,
    Active,
    Scaling,
    Degraded,
    Failed,
    Terminated,
}

/// Service performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePerformanceMetrics {
    /// CPU utilization percentage
    pub cpu_utilization: f64,
    /// Memory utilization percentage
    pub memory_utilization: f64,
    /// Request rate per second
    pub request_rate: f64,
    /// Average response time (ms)
    pub avg_response_time: f64,
    /// Error rate percentage
    pub error_rate: f64,
    /// Uptime percentage
    pub uptime: f64,
}

/// Mapping statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingStatistics {
    /// Total services mapped
    pub total_services_mapped: u64,
    /// Active services
    pub active_services: u64,
    /// Failed mappings
    pub failed_mappings: u64,
    /// Average mapping time (ms)
    pub avg_mapping_time: f64,
    /// Success rate percentage
    pub success_rate: f64,
    /// Last updated
    pub last_updated: DateTime<Utc>,
}

impl EnterpriseServiceMapper {
    /// Create new service mapper
    pub async fn new() -> Result<Self> {
        let mapper_id = format!("service_mapper_{}", Uuid::new_v4());
        let now = Utc::now();
        
        // Initialize default mapping rules
        let mut mapping_rules = HashMap::new();
        
        // Governance API mapping rule
        mapping_rules.insert(
            EnterpriseServiceType::GovernanceAPI,
            MappingRule {
                target_process_type: ProcessType::APIEndpoint,
                default_resources: ResourceAllocation {
                    cpu_percent: 15.0,
                    memory_bytes: 512 * 1024 * 1024, // 512MB
                    network_bandwidth: 10 * 1024 * 1024, // 10MB/s
                    storage_bytes: 100 * 1024 * 1024, // 100MB
                    priority: ProcessPriority::High,
                },
                required_security: SecurityLevel::Enhanced,
                scaling_policy: ScalingPolicy {
                    min_instances: 2,
                    max_instances: 10,
                    auto_scaling: true,
                    cpu_threshold: 80.0,
                    memory_threshold: 85.0,
                },
                priority: ProcessPriority::High,
            }
        );
        
        // Orchestration Service mapping rule
        mapping_rules.insert(
            EnterpriseServiceType::OrchestrationService,
            MappingRule {
                target_process_type: ProcessType::BackgroundWorker,
                default_resources: ResourceAllocation {
                    cpu_percent: 25.0,
                    memory_bytes: 1024 * 1024 * 1024, // 1GB
                    network_bandwidth: 50 * 1024 * 1024, // 50MB/s
                    storage_bytes: 500 * 1024 * 1024, // 500MB
                    priority: ProcessPriority::Critical,
                },
                required_security: SecurityLevel::Maximum,
                scaling_policy: ScalingPolicy {
                    min_instances: 3,
                    max_instances: 15,
                    auto_scaling: true,
                    cpu_threshold: 70.0,
                    memory_threshold: 80.0,
                },
                priority: ProcessPriority::Critical,
            }
        );
        
        // Add more mapping rules for other service types
        mapping_rules.insert(
            EnterpriseServiceType::CompanyManagement,
            MappingRule {
                target_process_type: ProcessType::APIEndpoint,
                default_resources: ResourceAllocation {
                    cpu_percent: 10.0,
                    memory_bytes: 256 * 1024 * 1024, // 256MB
                    network_bandwidth: 5 * 1024 * 1024, // 5MB/s
                    storage_bytes: 200 * 1024 * 1024, // 200MB
                    priority: ProcessPriority::Normal,
                },
                required_security: SecurityLevel::Standard,
                scaling_policy: ScalingPolicy {
                    min_instances: 1,
                    max_instances: 5,
                    auto_scaling: true,
                    cpu_threshold: 75.0,
                    memory_threshold: 80.0,
                },
                priority: ProcessPriority::Normal,
            }
        );
        
        let mapping_rules = Arc::new(RwLock::new(mapping_rules));
        let active_mappings = Arc::new(Mutex::new(HashMap::new()));
        
        let mapping_stats = Arc::new(RwLock::new(MappingStatistics {
            total_services_mapped: 0,
            active_services: 0,
            failed_mappings: 0,
            avg_mapping_time: 0.0,
            success_rate: 100.0,
            last_updated: now,
        }));
        
        Ok(EnterpriseServiceMapper {
            mapper_id,
            mapping_rules,
            active_mappings,
            mapping_stats,
            kernel_bridge: None,
        })
    }
    
    /// Initialize mapper with kernel bridge
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing enterprise service mapper: {}", self.mapper_id);
        // Initialization logic here
        Ok(())
    }
    
    /// Set kernel bridge reference
    pub async fn set_kernel_bridge(&mut self, bridge: Arc<BlockchainOSKernelBridge>) {
        self.kernel_bridge = Some(bridge);
    }
    
    /// Map enterprise service to BPI Core process
    pub async fn map_service(
        &self,
        service_id: &str,
        service_type: &EnterpriseServiceType,
        resource_requirements: &ResourceRequirements,
        security_context: &SecurityContext,
    ) -> Result<String> {
        let start_time = std::time::Instant::now();
        
        info!("Mapping enterprise service: {} (type: {:?})", service_id, service_type);
        
        // Get mapping rule for service type
        let mapping_rule = {
            let rules = self.mapping_rules.read().map_err(|_| anyhow!("Rules lock error"))?;
            rules.get(service_type).cloned()
                .ok_or_else(|| anyhow!("No mapping rule found for service type: {:?}", service_type))?
        };
        
        // Convert enterprise requirements to kernel allocation
        let resource_allocation = self.convert_resource_requirements(
            resource_requirements,
            &mapping_rule.default_resources,
        );
        
        // Create kernel process (simulated for now)
        let process_id = format!("kernel_proc_{}", Uuid::new_v4());
        
        // Create service mapping
        let mapping = ServiceProcessMapping {
            service_id: service_id.to_string(),
            process_id: process_id.clone(),
            service_type: service_type.clone(),
            status: MappingStatus::Active,
            allocated_resources: resource_allocation,
            performance_metrics: ServicePerformanceMetrics {
                cpu_utilization: 0.0,
                memory_utilization: 0.0,
                request_rate: 0.0,
                avg_response_time: 0.0,
                error_rate: 0.0,
                uptime: 100.0,
            },
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Store mapping
        {
            let mut mappings = self.active_mappings.lock().await;
            mappings.insert(service_id.to_string(), mapping);
        }
        
        // Update statistics
        let mapping_duration = start_time.elapsed().as_millis() as f64;
        {
            let mut stats = self.mapping_stats.write().map_err(|_| anyhow!("Stats lock error"))?;
            stats.total_services_mapped += 1;
            stats.active_services += 1;
            stats.avg_mapping_time = (stats.avg_mapping_time + mapping_duration) / 2.0;
            stats.success_rate = (stats.total_services_mapped - stats.failed_mappings) as f64 
                / stats.total_services_mapped as f64 * 100.0;
            stats.last_updated = Utc::now();
        }
        
        info!("Successfully mapped service {} to process {}", service_id, process_id);
        Ok(process_id)
    }
    
    /// Convert enterprise resource requirements to kernel allocation
    fn convert_resource_requirements(
        &self,
        requirements: &ResourceRequirements,
        defaults: &ResourceAllocation,
    ) -> ResourceAllocation {
        ResourceAllocation {
            cpu_percent: if requirements.cpu_percent > 0.0 {
                requirements.cpu_percent
            } else {
                defaults.cpu_percent
            },
            memory_bytes: if requirements.memory_bytes > 0 {
                requirements.memory_bytes
            } else {
                defaults.memory_bytes
            },
            network_bandwidth: if requirements.network_bandwidth > 0 {
                requirements.network_bandwidth
            } else {
                defaults.network_bandwidth
            },
            storage_bytes: if requirements.storage_bytes > 0 {
                requirements.storage_bytes
            } else {
                defaults.storage_bytes
            },
            priority: defaults.priority.clone(),
        }
    }
    
    /// Get service mapping
    pub async fn get_service_mapping(&self, service_id: &str) -> Result<Option<ServiceProcessMapping>> {
        let mappings = self.active_mappings.lock().await;
        Ok(mappings.get(service_id).cloned())
    }
    
    /// Get all active mappings
    pub async fn get_all_mappings(&self) -> Result<Vec<ServiceProcessMapping>> {
        let mappings = self.active_mappings.lock().await;
        Ok(mappings.values().cloned().collect())
    }
    
    /// Update service performance metrics
    pub async fn update_service_metrics(
        &self,
        service_id: &str,
        metrics: ServicePerformanceMetrics,
    ) -> Result<()> {
        let mut mappings = self.active_mappings.lock().await;
        if let Some(mapping) = mappings.get_mut(service_id) {
            mapping.performance_metrics = metrics;
            mapping.updated_at = Utc::now();
        }
        Ok(())
    }
    
    /// Get mapping statistics
    pub async fn get_mapping_statistics(&self) -> Result<MappingStatistics> {
        let stats = self.mapping_stats.read().map_err(|_| anyhow!("Stats lock error"))?;
        Ok(stats.clone())
    }
    
    /// Shutdown service mapper
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down enterprise service mapper");
        
        // Terminate all active mappings
        {
            let mut mappings = self.active_mappings.lock().await;
            for (_, mapping) in mappings.iter_mut() {
                mapping.status = MappingStatus::Terminated;
            }
        }
        
        info!("Enterprise service mapper shutdown complete");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_service_mapper_creation() {
        let mapper = EnterpriseServiceMapper::new().await.unwrap();
        assert!(!mapper.mapper_id.is_empty());
    }
    
    #[tokio::test]
    async fn test_service_mapping() {
        let mapper = EnterpriseServiceMapper::new().await.unwrap();
        
        let resource_requirements = ResourceRequirements {
            cpu_percent: 20.0,
            memory_bytes: 512 * 1024 * 1024,
            network_bandwidth: 10 * 1024 * 1024,
            storage_bytes: 100 * 1024 * 1024,
        };
        
        let security_context = SecurityContext {
            security_level: super::super::SecurityLevel::Enterprise,
            quantum_encryption: true,
            audit_logging: true,
            permissions: vec!["read".to_string(), "write".to_string()],
        };
        
        let process_id = mapper.map_service(
            "test_service_001",
            &EnterpriseServiceType::GovernanceAPI,
            &resource_requirements,
            &security_context,
        ).await.unwrap();
        
        assert!(!process_id.is_empty());
        
        let mapping = mapper.get_service_mapping("test_service_001").await.unwrap();
        assert!(mapping.is_some());
    }
}
