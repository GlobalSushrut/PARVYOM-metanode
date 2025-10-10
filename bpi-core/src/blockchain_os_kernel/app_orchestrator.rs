// VM Application Orchestrator
// Manages VM-based application deployment, scaling, and lifecycle through blockchain consensus

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use uuid::Uuid;

use super::OrchestrationMode;

/// Application deployment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppDeployment {
    pub deployment_id: String,
    pub app_name: String,
    pub app_version: String,
    pub vm_type: VMType,
    pub deployment_config: DeploymentConfig,
    pub resource_allocation: AppResourceAllocation,
    pub security_policy: AppSecurityPolicy,
    pub status: DeploymentStatus,
    pub created_at: u64,
    pub last_updated: u64,
}

/// VM types for application hosting
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VMType {
    DockLock,       // Container-based VM
    ENC,            // Encrypted VM
    HTTP,           // HTTP service VM
    CG,             // Client Gateway VM
    SAPI,           // Secure API VM
    QLOCK,          // Quantum-locked VM
    TSLS,           // Transport Security Layer VM
    Custom(String), // Custom VM type
}

/// Deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    pub replicas: u32,
    pub auto_scaling: AutoScalingConfig,
    pub health_check: HealthCheckConfig,
    pub networking: NetworkingConfig,
    pub storage: StorageConfig,
    pub environment_variables: HashMap<String, String>,
}

/// Auto-scaling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoScalingConfig {
    pub enabled: bool,
    pub min_replicas: u32,
    pub max_replicas: u32,
    pub cpu_threshold: f64,
    pub memory_threshold: f64,
    pub scale_up_cooldown: u64,
    pub scale_down_cooldown: u64,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub interval_seconds: u64,
    pub timeout_seconds: u64,
    pub failure_threshold: u32,
    pub success_threshold: u32,
}

/// Networking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkingConfig {
    pub port_mappings: Vec<PortMapping>,
    pub load_balancer: LoadBalancerConfig,
    pub service_mesh: bool,
    pub ingress_rules: Vec<IngressRule>,
}

/// Port mapping configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMapping {
    pub container_port: u16,
    pub host_port: u16,
    pub protocol: NetworkProtocol,
}

/// Network protocols
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkProtocol {
    TCP,
    UDP,
    HTTP,
    HTTPS,
    QUIC,
}

/// Load balancer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerConfig {
    pub algorithm: LoadBalancingAlgorithm,
    pub session_affinity: bool,
    pub health_check_path: String,
}

/// Load balancing algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    LeastConnections,
    WeightedRoundRobin,
    IPHash,
    ConsistentHash,
}

/// Ingress rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngressRule {
    pub host: String,
    pub path: String,
    pub backend_service: String,
    pub tls_enabled: bool,
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub volumes: Vec<VolumeMount>,
    pub persistent_storage: bool,
    pub backup_policy: BackupPolicy,
}

/// Volume mount configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeMount {
    pub name: String,
    pub mount_path: String,
    pub volume_type: VolumeType,
    pub size_gb: u64,
    pub read_only: bool,
}

/// Volume types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VolumeType {
    EmptyDir,
    HostPath,
    PersistentVolume,
    ConfigMap,
    Secret,
}

/// Backup policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupPolicy {
    pub enabled: bool,
    pub schedule: String, // Cron expression
    pub retention_days: u32,
    pub compression: bool,
    pub encryption: bool,
}

/// Application resource allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppResourceAllocation {
    pub cpu_cores: f64,
    pub memory_mb: u64,
    pub storage_gb: u64,
    pub network_bandwidth_mbps: u64,
    pub gpu_units: u32,
    pub priority_class: PriorityClass,
}

/// Priority classes for applications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriorityClass {
    System,
    High,
    Normal,
    Low,
    BestEffort,
}

/// Application security policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSecurityPolicy {
    pub security_context: AppSecurityContext,
    pub network_policies: Vec<NetworkPolicy>,
    pub rbac_rules: Vec<RBACRule>,
    pub pod_security_standards: PodSecurityStandards,
}

/// Application security context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSecurityContext {
    pub run_as_user: Option<u32>,
    pub run_as_group: Option<u32>,
    pub fs_group: Option<u32>,
    pub privileged: bool,
    pub read_only_root_filesystem: bool,
    pub allow_privilege_escalation: bool,
    pub capabilities: SecurityCapabilities,
}

/// Security capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCapabilities {
    pub add: Vec<String>,
    pub drop: Vec<String>,
}

/// Network security policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicy {
    pub policy_name: String,
    pub ingress_rules: Vec<NetworkPolicyRule>,
    pub egress_rules: Vec<NetworkPolicyRule>,
}

/// Network policy rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicyRule {
    pub ports: Vec<u16>,
    pub protocols: Vec<NetworkProtocol>,
    pub from_selectors: Vec<String>,
    pub to_selectors: Vec<String>,
}

/// Role-Based Access Control rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RBACRule {
    pub role_name: String,
    pub resources: Vec<String>,
    pub verbs: Vec<String>,
    pub api_groups: Vec<String>,
}

/// Pod security standards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodSecurityStandards {
    pub enforce: SecurityStandard,
    pub audit: SecurityStandard,
    pub warn: SecurityStandard,
}

/// Security standards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityStandard {
    Privileged,
    Baseline,
    Restricted,
}

/// Deployment status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentStatus {
    Pending,
    Deploying,
    Running,
    Scaling,
    Updating,
    Failed(String),
    Terminated,
}

/// Orchestration policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationPolicy {
    pub policy_name: String,
    pub auto_deployment: bool,
    pub auto_scaling: bool,
    pub auto_healing: bool,
    pub resource_optimization: bool,
    pub security_enforcement: bool,
    pub compliance_checking: bool,
}

/// VM Application Orchestrator
#[derive(Debug)]
pub struct VMApplicationOrchestrator {
    /// Active deployments
    deployments: Arc<Mutex<HashMap<String, AppDeployment>>>,
    
    /// Orchestration policies
    policies: Arc<RwLock<HashMap<String, OrchestrationPolicy>>>,
    
    /// VM type registry
    vm_registry: Arc<RwLock<HashMap<VMType, VMTypeInfo>>>,
    
    /// Orchestration mode
    orchestration_mode: Arc<RwLock<OrchestrationMode>>,
    
    /// Orchestrator configuration
    config: Arc<RwLock<OrchestratorConfig>>,
    
    /// Deployment statistics
    stats: Arc<RwLock<OrchestrationStats>>,
}

/// VM type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMTypeInfo {
    pub vm_type: VMType,
    pub description: String,
    pub capabilities: Vec<String>,
    pub resource_requirements: AppResourceAllocation,
    pub security_features: Vec<String>,
    pub supported_protocols: Vec<NetworkProtocol>,
}

/// Orchestrator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorConfig {
    pub max_deployments: u32,
    pub default_replicas: u32,
    pub auto_scaling_enabled: bool,
    pub health_check_interval: u64,
    pub deployment_timeout: u64,
    pub resource_optimization: bool,
}

/// Orchestration statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationStats {
    pub total_deployments: u64,
    pub active_deployments: u32,
    pub failed_deployments: u64,
    pub total_scaling_events: u64,
    pub average_deployment_time: f64,
    pub resource_utilization: f64,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            max_deployments: 1000,
            default_replicas: 1,
            auto_scaling_enabled: true,
            health_check_interval: 30,
            deployment_timeout: 600, // 10 minutes
            resource_optimization: true,
        }
    }
}

impl Default for OrchestrationStats {
    fn default() -> Self {
        Self {
            total_deployments: 0,
            active_deployments: 0,
            failed_deployments: 0,
            total_scaling_events: 0,
            average_deployment_time: 0.0,
            resource_utilization: 0.0,
        }
    }
}

impl VMApplicationOrchestrator {
    /// Create a new VM application orchestrator
    pub async fn new() -> Result<Self> {
        Ok(Self {
            deployments: Arc::new(Mutex::new(HashMap::new())),
            policies: Arc::new(RwLock::new(HashMap::new())),
            vm_registry: Arc::new(RwLock::new(HashMap::new())),
            orchestration_mode: Arc::new(RwLock::new(OrchestrationMode::Autonomous)),
            config: Arc::new(RwLock::new(OrchestratorConfig::default())),
            stats: Arc::new(RwLock::new(OrchestrationStats::default())),
        })
    }

    /// Initialize the orchestrator
    pub async fn initialize(&self) -> Result<()> {
        println!("🔄 Initializing VM Application Orchestrator...");
        
        // Register default VM types
        self.register_default_vm_types().await?;
        
        // Initialize default policies
        self.initialize_default_policies().await?;
        
        // Start orchestration services
        self.start_orchestration_services().await?;
        
        println!("✅ VM Application Orchestrator initialized");
        Ok(())
    }

    /// Deploy an application
    pub async fn deploy_application(
        &self,
        app_name: &str,
        app_version: &str,
        vm_type: VMType,
        config: DeploymentConfig,
    ) -> Result<String> {
        let deployment_id = uuid::Uuid::new_v4().to_string();
        
        let deployment = AppDeployment {
            deployment_id: deployment_id.clone(),
            app_name: app_name.to_string(),
            app_version: app_version.to_string(),
            vm_type: vm_type.clone(),
            deployment_config: config,
            resource_allocation: self.calculate_resource_allocation(&vm_type).await?,
            security_policy: self.create_security_policy(&vm_type).await?,
            status: DeploymentStatus::Pending,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        };

        // Store deployment
        {
            let mut deployments = self.deployments.lock().await;
            deployments.insert(deployment_id.clone(), deployment);
        }

        // Start deployment process
        self.start_deployment_process(&deployment_id).await?;

        // Update statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_deployments += 1;
            stats.active_deployments += 1;
        }

        println!("🚀 Started deployment of {} v{} with ID: {}", app_name, app_version, deployment_id);
        Ok(deployment_id)
    }

    /// Scale an application
    pub async fn scale_application(&self, deployment_id: &str, replicas: u32) -> Result<()> {
        let mut deployments = self.deployments.lock().await;
        
        if let Some(deployment) = deployments.get_mut(deployment_id) {
            deployment.deployment_config.replicas = replicas;
            deployment.status = DeploymentStatus::Scaling;
            deployment.last_updated = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs();

            // Update statistics
            {
                let mut stats = self.stats.write().unwrap();
                stats.total_scaling_events += 1;
            }

            println!("📈 Scaling deployment {} to {} replicas", deployment_id, replicas);
        } else {
            return Err(anyhow::anyhow!("Deployment not found: {}", deployment_id));
        }

        Ok(())
    }

    /// Terminate an application deployment
    pub async fn terminate_deployment(&self, deployment_id: &str) -> Result<()> {
        let mut deployments = self.deployments.lock().await;
        
        if let Some(mut deployment) = deployments.remove(deployment_id) {
            deployment.status = DeploymentStatus::Terminated;
            
            // Update statistics
            {
                let mut stats = self.stats.write().unwrap();
                stats.active_deployments = stats.active_deployments.saturating_sub(1);
            }

            println!("⏹️ Terminated deployment: {}", deployment_id);
        } else {
            return Err(anyhow::anyhow!("Deployment not found: {}", deployment_id));
        }

        Ok(())
    }

    /// Get deployment status
    pub async fn get_deployment_status(&self, deployment_id: &str) -> Result<Option<AppDeployment>> {
        let deployments = self.deployments.lock().await;
        Ok(deployments.get(deployment_id).cloned())
    }

    /// List all deployments
    pub async fn list_deployments(&self) -> Result<Vec<AppDeployment>> {
        let deployments = self.deployments.lock().await;
        Ok(deployments.values().cloned().collect())
    }

    /// Update orchestration mode
    pub async fn update_orchestration_mode(&self, mode: &OrchestrationMode) -> Result<()> {
        {
            let mut current_mode = self.orchestration_mode.write().unwrap();
            *current_mode = mode.clone();
        }

        // Adjust orchestration behavior based on mode
        match mode {
            OrchestrationMode::Autonomous => {
                let mut config = self.config.write().unwrap();
                config.auto_scaling_enabled = true;
                config.resource_optimization = true;
            },
            OrchestrationMode::Supervised => {
                let mut config = self.config.write().unwrap();
                config.auto_scaling_enabled = true;
                config.resource_optimization = false;
            },
            OrchestrationMode::Manual => {
                let mut config = self.config.write().unwrap();
                config.auto_scaling_enabled = false;
                config.resource_optimization = false;
            },
            OrchestrationMode::Emergency => {
                let mut config = self.config.write().unwrap();
                config.max_deployments = 100; // Limit deployments in emergency
                config.auto_scaling_enabled = false;
            },
        }

        println!("🔄 Application orchestrator updated to {:?} mode", mode);
        Ok(())
    }

    /// Perform health check
    pub async fn health_check(&self) -> Result<bool> {
        let deployments = self.deployments.lock().await;
        let stats = self.stats.read().unwrap();
        
        // Check for reasonable deployment counts and failure rates
        let healthy = deployments.len() < 1000 && 
                     (stats.total_deployments == 0 || stats.failed_deployments < stats.total_deployments / 10);
        
        if healthy {
            println!("✅ Application orchestrator health check: HEALTHY");
        } else {
            println!("⚠️ Application orchestrator health check: DEGRADED (deployments: {}, failure rate: {:.1}%)", 
                deployments.len(),
                (stats.failed_deployments as f64 / stats.total_deployments.max(1) as f64) * 100.0
            );
        }
        
        Ok(healthy)
    }

    /// Shutdown the orchestrator
    pub async fn shutdown(&self) -> Result<()> {
        println!("🔄 Shutting down VM Application Orchestrator...");
        
        // Terminate all deployments gracefully
        let deployment_ids: Vec<String> = {
            let deployments = self.deployments.lock().await;
            deployments.keys().cloned().collect()
        };

        for deployment_id in deployment_ids {
            if let Err(e) = self.terminate_deployment(&deployment_id).await {
                println!("⚠️ Error terminating deployment {}: {}", deployment_id, e);
            }
        }

        println!("✅ VM Application Orchestrator shutdown complete");
        Ok(())
    }

    // Private helper methods

    async fn register_default_vm_types(&self) -> Result<()> {
        let mut registry = self.vm_registry.write().unwrap();
        
        // Register DockLock VM
        registry.insert(VMType::DockLock, VMTypeInfo {
            vm_type: VMType::DockLock,
            description: "Container-based VM for application isolation".to_string(),
            capabilities: vec!["containerization".to_string(), "resource_isolation".to_string()],
            resource_requirements: AppResourceAllocation {
                cpu_cores: 1.0,
                memory_mb: 512,
                storage_gb: 5,
                network_bandwidth_mbps: 100,
                gpu_units: 0,
                priority_class: PriorityClass::Normal,
            },
            security_features: vec!["namespace_isolation".to_string(), "cgroup_limits".to_string()],
            supported_protocols: vec![NetworkProtocol::TCP, NetworkProtocol::HTTP],
        });

        // Register ENC VM
        registry.insert(VMType::ENC, VMTypeInfo {
            vm_type: VMType::ENC,
            description: "Encrypted VM for secure computation".to_string(),
            capabilities: vec!["encryption".to_string(), "secure_computation".to_string()],
            resource_requirements: AppResourceAllocation {
                cpu_cores: 2.0,
                memory_mb: 1024,
                storage_gb: 10,
                network_bandwidth_mbps: 200,
                gpu_units: 0,
                priority_class: PriorityClass::High,
            },
            security_features: vec!["full_encryption".to_string(), "secure_boot".to_string()],
            supported_protocols: vec![NetworkProtocol::HTTPS, NetworkProtocol::QUIC],
        });

        println!("📋 Registered default VM types");
        Ok(())
    }

    async fn initialize_default_policies(&self) -> Result<()> {
        let mut policies = self.policies.write().unwrap();
        
        policies.insert("default".to_string(), OrchestrationPolicy {
            policy_name: "default".to_string(),
            auto_deployment: true,
            auto_scaling: true,
            auto_healing: true,
            resource_optimization: true,
            security_enforcement: true,
            compliance_checking: true,
        });

        println!("📋 Initialized default orchestration policies");
        Ok(())
    }

    async fn start_orchestration_services(&self) -> Result<()> {
        println!("🔄 Starting orchestration services...");
        // This would start background services for monitoring, scaling, etc.
        Ok(())
    }

    async fn calculate_resource_allocation(&self, vm_type: &VMType) -> Result<AppResourceAllocation> {
        let registry = self.vm_registry.read().unwrap();
        
        if let Some(vm_info) = registry.get(vm_type) {
            Ok(vm_info.resource_requirements.clone())
        } else {
            // Default allocation for unknown VM types
            Ok(AppResourceAllocation {
                cpu_cores: 1.0,
                memory_mb: 512,
                storage_gb: 5,
                network_bandwidth_mbps: 100,
                gpu_units: 0,
                priority_class: PriorityClass::Normal,
            })
        }
    }

    async fn create_security_policy(&self, _vm_type: &VMType) -> Result<AppSecurityPolicy> {
        Ok(AppSecurityPolicy {
            security_context: AppSecurityContext {
                run_as_user: Some(1000),
                run_as_group: Some(1000),
                fs_group: Some(2000),
                privileged: false,
                read_only_root_filesystem: true,
                allow_privilege_escalation: false,
                capabilities: SecurityCapabilities {
                    add: vec![],
                    drop: vec!["ALL".to_string()],
                },
            },
            network_policies: vec![],
            rbac_rules: vec![],
            pod_security_standards: PodSecurityStandards {
                enforce: SecurityStandard::Restricted,
                audit: SecurityStandard::Restricted,
                warn: SecurityStandard::Baseline,
            },
        })
    }

    async fn start_deployment_process(&self, deployment_id: &str) -> Result<()> {
        // Update deployment status to deploying
        {
            let mut deployments = self.deployments.lock().await;
            if let Some(deployment) = deployments.get_mut(deployment_id) {
                deployment.status = DeploymentStatus::Deploying;
                deployment.last_updated = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs();
            }
        }

        // Simulate deployment process
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Update deployment status to running
        {
            let mut deployments = self.deployments.lock().await;
            if let Some(deployment) = deployments.get_mut(deployment_id) {
                deployment.status = DeploymentStatus::Running;
                deployment.last_updated = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs();
            }
        }

        println!("✅ Deployment {} is now running", deployment_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_orchestrator_creation() {
        let orchestrator = VMApplicationOrchestrator::new().await.unwrap();
        assert!(orchestrator.initialize().await.is_ok());
        assert!(orchestrator.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_application_deployment() {
        let orchestrator = VMApplicationOrchestrator::new().await.unwrap();
        orchestrator.initialize().await.unwrap();

        let config = DeploymentConfig {
            replicas: 1,
            auto_scaling: AutoScalingConfig {
                enabled: false,
                min_replicas: 1,
                max_replicas: 1,
                cpu_threshold: 0.8,
                memory_threshold: 0.8,
                scale_up_cooldown: 300,
                scale_down_cooldown: 300,
            },
            health_check: HealthCheckConfig {
                enabled: true,
                endpoint: "/health".to_string(),
                interval_seconds: 30,
                timeout_seconds: 5,
                failure_threshold: 3,
                success_threshold: 1,
            },
            networking: NetworkingConfig {
                port_mappings: vec![],
                load_balancer: LoadBalancerConfig {
                    algorithm: LoadBalancingAlgorithm::RoundRobin,
                    session_affinity: false,
                    health_check_path: "/health".to_string(),
                },
                service_mesh: false,
                ingress_rules: vec![],
            },
            storage: StorageConfig {
                volumes: vec![],
                persistent_storage: false,
                backup_policy: BackupPolicy {
                    enabled: false,
                    schedule: "0 2 * * *".to_string(),
                    retention_days: 7,
                    compression: true,
                    encryption: true,
                },
            },
            environment_variables: HashMap::new(),
        };

        let deployment_id = orchestrator.deploy_application(
            "test_app",
            "1.0.0",
            VMType::DockLock,
            config,
        ).await.unwrap();

        assert!(!deployment_id.is_empty());

        let status = orchestrator.get_deployment_status(&deployment_id).await.unwrap();
        assert!(status.is_some());

        orchestrator.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_application_scaling() {
        let orchestrator = VMApplicationOrchestrator::new().await.unwrap();
        orchestrator.initialize().await.unwrap();

        let config = DeploymentConfig {
            replicas: 1,
            auto_scaling: AutoScalingConfig {
                enabled: true,
                min_replicas: 1,
                max_replicas: 5,
                cpu_threshold: 0.8,
                memory_threshold: 0.8,
                scale_up_cooldown: 300,
                scale_down_cooldown: 300,
            },
            health_check: HealthCheckConfig {
                enabled: true,
                endpoint: "/health".to_string(),
                interval_seconds: 30,
                timeout_seconds: 5,
                failure_threshold: 3,
                success_threshold: 1,
            },
            networking: NetworkingConfig {
                port_mappings: vec![],
                load_balancer: LoadBalancerConfig {
                    algorithm: LoadBalancingAlgorithm::RoundRobin,
                    session_affinity: false,
                    health_check_path: "/health".to_string(),
                },
                service_mesh: false,
                ingress_rules: vec![],
            },
            storage: StorageConfig {
                volumes: vec![],
                persistent_storage: false,
                backup_policy: BackupPolicy {
                    enabled: false,
                    schedule: "0 2 * * *".to_string(),
                    retention_days: 7,
                    compression: true,
                    encryption: true,
                },
            },
            environment_variables: HashMap::new(),
        };

        let deployment_id = orchestrator.deploy_application(
            "scalable_app",
            "1.0.0",
            VMType::DockLock,
            config,
        ).await.unwrap();

        // Scale the application
        assert!(orchestrator.scale_application(&deployment_id, 3).await.is_ok());

        let status = orchestrator.get_deployment_status(&deployment_id).await.unwrap();
        assert!(status.is_some());
        assert_eq!(status.unwrap().deployment_config.replicas, 3);

        orchestrator.shutdown().await.unwrap();
    }
}
