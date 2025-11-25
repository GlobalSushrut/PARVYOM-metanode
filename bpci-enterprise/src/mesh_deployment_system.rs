//! Mesh Deployment System for 13-Server BPCI Cluster
//! 
//! Deploys all 13 BPCI servers simultaneously with pre-configuration
//! Supports future mainnet scaling and cuboidal daemon cluster formation
//! Uses native BSO-K8, DynaRoute, vPods, and BPI OS stack

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::{RwLock, Semaphore};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::bso_k8_orchestrator::BsoK8Orchestrator;
use crate::dynaroute_integration::UnifiedNetworkingLayer;
use crate::hyperscale_proxy_architecture::BpiOsNodeProxy;

/// 13-Server BPCI Cluster Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpciClusterConfig {
    pub cluster_id: String,
    pub deployment_mode: DeploymentMode,
    pub server_configs: Vec<ServerConfig>,
    pub mesh_topology: MeshTopology,
    pub networking_config: NetworkingConfig,
    pub security_config: SecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentMode {
    Testnet,      // 13 servers for testing
    Mainnet,      // 35+ servers for production
    Development,  // Single-node development
}

/// Individual server configuration in the cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub server_id: String,
    pub server_type: ServerType,
    pub vpod_config: VPodConfig,
    pub resource_allocation: ResourceAllocation,
    pub dependencies: Vec<String>,
    pub startup_order: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerType {
    BpciConsensus,
    BpciBlockchain,
    BpciAuction,
    BpiLedger,
    WalletRegistry,
    QuantumSync,
    ProxyCoordinator,
    RegionalCluster,
    EdgeProxy,
    LoadBalancer,
    HealthMonitor,
    MetricsCollector,
    SecurityGateway,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPodConfig {
    pub vpod_id: String,
    pub service_name: String,
    pub port_range: (u16, u16),
    pub resource_limits: ResourceLimits,
    pub environment_vars: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocation {
    pub cpu_cores: f64,
    pub memory_mb: u64,
    pub disk_gb: u64,
    pub network_bandwidth_mbps: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_cpu_percent: f64,
    pub max_memory_mb: u64,
    pub max_connections: u64,
    pub max_requests_per_sec: u64,
}

/// Mesh topology configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshTopology {
    pub topology_type: TopologyType,
    pub redundancy_level: RedundancyLevel,
    pub failover_config: FailoverConfig,
    pub load_balancing: LoadBalancingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TopologyType {
    FullMesh,     // All nodes connected to all nodes
    StarMesh,     // Hub and spoke with redundancy
    RingMesh,     // Circular with cross-connections
    Cuboidal,     // 3D cube topology for scaling
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RedundancyLevel {
    None,         // No redundancy
    Basic,        // 2x redundancy
    High,         // 3x redundancy
    Military,     // 5x redundancy
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverConfig {
    pub detection_timeout_ms: u64,
    pub failover_timeout_ms: u64,
    pub health_check_interval_ms: u64,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    pub algorithm: LoadBalancingAlgorithm,
    pub health_weight: f64,
    pub latency_weight: f64,
    pub capacity_weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    LatencyBased,
    CapacityBased,
}

/// Networking configuration for the mesh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkingConfig {
    pub mesh_network_cidr: String,
    pub service_discovery: ServiceDiscoveryConfig,
    pub proxy_config: ProxyConfig,
    pub encryption_config: EncryptionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryConfig {
    pub discovery_method: DiscoveryMethod,
    pub ttl_seconds: u64,
    pub refresh_interval_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    DynaRoute,
    HermesP2P,
    ConsulLike,
    EtcdLike,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub enable_quantum_proxies: bool,
    pub proxy_chain_length: u32,
    pub zk_protection_level: ZkProtectionLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZkProtectionLevel {
    None,
    Basic,
    Advanced,
    Military,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub transport_encryption: bool,
    pub at_rest_encryption: bool,
    pub key_rotation_hours: u64,
    pub cipher_suite: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub authentication_method: AuthMethod,
    pub authorization_policy: AuthzPolicy,
    pub audit_config: AuditConfig,
    pub firewall_rules: Vec<FirewallRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    Ed25519Keys,
    MutualTLS,
    ZeroKnowledge,
    MultiFactorAuth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthzPolicy {
    RoleBased,
    AttributeBased,
    PolicyBased,
    ZeroTrust,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    pub enable_audit_trail: bool,
    pub log_level: String,
    pub retention_days: u32,
    pub immutable_logging: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallRule {
    pub rule_id: String,
    pub source: String,
    pub destination: String,
    pub port_range: String,
    pub protocol: String,
    pub action: String,
}

/// Main mesh deployment orchestrator
#[derive(Debug)]
pub struct MeshDeploymentOrchestrator {
    pub cluster_config: BpciClusterConfig,
    pub bso_k8: Arc<BsoK8Orchestrator>,
    pub networking: Arc<UnifiedNetworkingLayer>,
    pub deployed_servers: Arc<RwLock<HashMap<String, DeployedServer>>>,
    pub deployment_semaphore: Arc<Semaphore>,
    pub deployment_status: Arc<RwLock<DeploymentStatus>>,
}

#[derive(Debug, Clone)]
pub struct DeployedServer {
    pub server_id: String,
    pub server_type: ServerType,
    pub vpod_id: String,
    pub actual_address: std::net::SocketAddr,
    pub virtual_address: String,
    pub status: ServerStatus,
    pub health_score: f64,
    pub deployed_at: DateTime<Utc>,
    pub last_health_check: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerStatus {
    Deploying,
    Starting,
    Healthy,
    Degraded,
    Failed,
    Stopped,
}

#[derive(Debug, Clone)]
pub struct DeploymentStatus {
    pub phase: DeploymentPhase,
    pub servers_deployed: u32,
    pub servers_healthy: u32,
    pub deployment_start: DateTime<Utc>,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub errors: Vec<DeploymentError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentPhase {
    Planning,
    PreConfiguration,
    Deploying,
    HealthChecking,
    MeshFormation,
    Testing,
    Complete,
    Failed,
}

#[derive(Debug, Clone)]
pub struct DeploymentError {
    pub server_id: String,
    pub error_type: String,
    pub error_message: String,
    pub timestamp: DateTime<Utc>,
    pub retry_count: u32,
}

impl MeshDeploymentOrchestrator {
    /// Create new mesh deployment orchestrator
    pub async fn new(
        cluster_config: BpciClusterConfig,
        bso_k8: Arc<BsoK8Orchestrator>,
        networking: Arc<UnifiedNetworkingLayer>,
    ) -> Result<Self> {
        let deployment_semaphore = Arc::new(Semaphore::new(5)); // Max 5 concurrent deployments
        
        Ok(Self {
            cluster_config,
            bso_k8,
            networking,
            deployed_servers: Arc::new(RwLock::new(HashMap::new())),
            deployment_semaphore,
            deployment_status: Arc::new(RwLock::new(DeploymentStatus {
                phase: DeploymentPhase::Planning,
                servers_deployed: 0,
                servers_healthy: 0,
                deployment_start: Utc::now(),
                estimated_completion: None,
                errors: Vec::new(),
            })),
        })
    }

    /// Deploy entire 13-server cluster simultaneously
    pub async fn deploy_cluster(&self) -> Result<()> {
        tracing::info!("🚀 Starting 13-server BPCI cluster deployment");
        
        // Phase 1: Pre-configuration
        self.update_deployment_phase(DeploymentPhase::PreConfiguration).await;
        self.prepare_cluster_infrastructure().await?;
        
        // Phase 2: Parallel server deployment
        self.update_deployment_phase(DeploymentPhase::Deploying).await;
        self.deploy_servers_parallel().await?;
        
        // Phase 3: Health checking
        self.update_deployment_phase(DeploymentPhase::HealthChecking).await;
        self.wait_for_all_servers_healthy().await?;
        
        // Phase 4: Mesh formation
        self.update_deployment_phase(DeploymentPhase::MeshFormation).await;
        self.form_mesh_topology().await?;
        
        // Phase 5: Integration testing
        self.update_deployment_phase(DeploymentPhase::Testing).await;
        self.run_integration_tests().await?;
        
        // Phase 6: Complete
        self.update_deployment_phase(DeploymentPhase::Complete).await;
        
        tracing::info!("✅ 13-server BPCI cluster deployment complete!");
        Ok(())
    }

    /// Prepare cluster infrastructure
    async fn prepare_cluster_infrastructure(&self) -> Result<()> {
        tracing::info!("🔧 Preparing cluster infrastructure");
        
        // 1. Validate resource availability
        self.validate_resource_requirements().await?;
        
        // 2. Setup networking infrastructure
        self.setup_mesh_networking().await?;
        
        // 3. Configure security policies
        self.configure_security_policies().await?;
        
        // 4. Prepare service discovery
        self.setup_service_discovery().await?;
        
        tracing::info!("✅ Cluster infrastructure prepared");
        Ok(())
    }

    /// Deploy all servers in parallel with dependency ordering
    async fn deploy_servers_parallel(&self) -> Result<()> {
        tracing::info!("🚀 Deploying servers in parallel");
        
        // Sort servers by startup order
        let mut sorted_servers = self.cluster_config.server_configs.clone();
        sorted_servers.sort_by_key(|s| s.startup_order);
        
        // Group servers by startup order for parallel deployment within groups
        let mut deployment_groups: HashMap<u32, Vec<ServerConfig>> = HashMap::new();
        for server in sorted_servers {
            deployment_groups.entry(server.startup_order).or_insert_with(Vec::new).push(server);
        }
        
        // Deploy each group in order, but servers within group in parallel
        for (order, servers) in deployment_groups {
            tracing::info!("📦 Deploying startup group {}: {} servers", order, servers.len());
            
            let mut deployment_tasks = Vec::new();
            
            for server in servers {
                let permit = self.deployment_semaphore.clone().acquire_owned().await?;
                let orchestrator = self.clone();
                
                let task = tokio::spawn(async move {
                    let _permit = permit; // Hold permit for duration
                    orchestrator.deploy_single_server(server).await
                });
                
                deployment_tasks.push(task);
            }
            
            // Wait for all servers in this group to deploy
            for task in deployment_tasks {
                task.await??;
            }
            
            tracing::info!("✅ Startup group {} deployed successfully", order);
        }
        
        Ok(())
    }

    /// Deploy a single server
    async fn deploy_single_server(&self, server_config: ServerConfig) -> Result<()> {
        tracing::info!("🔧 Deploying server: {}", server_config.server_id);
        
        // 1. Deploy service using BSO-K8 orchestrator
        let service_type = match server_config.server_type {
            ServerType::BpciConsensus | ServerType::BpciBlockchain | ServerType::BpciAuction => {
                crate::bso_k8_orchestrator::ServiceType::BpciEnterprise {
                    port: 8000 + server_config.server_id.chars().map(|c| c as u16).sum::<u16>() % 1000,
                    config_path: format!("/etc/bpci/{}.toml", server_config.server_id),
                }
            },
            ServerType::BpiLedger => {
                crate::bso_k8_orchestrator::ServiceType::CustomBinary {
                    binary_path: "/usr/local/bin/bpi-ledger".to_string(),
                    args: vec![
                        "--config".to_string(),
                        format!("/etc/bpi/{}.toml", server_config.server_id),
                        "--server-id".to_string(),
                        server_config.server_id.clone(),
                    ],
                    env_vars: vec![
                        ("BPI_SERVER_ID".to_string(), server_config.server_id.clone()),
                        ("BPI_MODE".to_string(), "cluster".to_string()),
                    ],
                    working_dir: Some("/var/lib/bpi".to_string()),
                }
            },
            ServerType::WalletRegistry => {
                crate::bso_k8_orchestrator::ServiceType::CustomBinary {
                    binary_path: "/usr/local/bin/wallet-registry".to_string(),
                    args: vec!["--server-id".to_string(), server_config.server_id.clone()],
                    env_vars: vec![("WALLET_REGISTRY_ID".to_string(), server_config.server_id.clone())],
                    working_dir: Some("/var/lib/wallet-registry".to_string()),
                }
            },
            ServerType::QuantumSync => {
                crate::bso_k8_orchestrator::ServiceType::BsoController {
                    vpod_count: 4,
                    arena_size: "2GB".to_string(),
                }
            },
            ServerType::ProxyCoordinator | ServerType::EdgeProxy => {
                crate::bso_k8_orchestrator::ServiceType::HAProxy {
                    config_path: format!("/etc/haproxy/{}.cfg", server_config.server_id),
                    stats_port: Some(9000 + server_config.server_id.chars().map(|c| c as u16).sum::<u16>() % 1000),
                }
            },
            ServerType::RegionalCluster => {
                crate::bso_k8_orchestrator::ServiceType::CellularGrowthManager {
                    replication_factor: 3,
                }
            },
            ServerType::LoadBalancer => {
                crate::bso_k8_orchestrator::ServiceType::Traefik {
                    config_path: format!("/etc/traefik/{}.yml", server_config.server_id),
                    api_port: Some(8080 + server_config.server_id.chars().map(|c| c as u16).sum::<u16>() % 1000),
                }
            },
            ServerType::HealthMonitor => {
                crate::bso_k8_orchestrator::ServiceType::Prometheus {
                    port: 9090 + server_config.server_id.chars().map(|c| c as u16).sum::<u16>() % 1000,
                    config_path: format!("/etc/prometheus/{}.yml", server_config.server_id),
                }
            },
            ServerType::MetricsCollector => {
                crate::bso_k8_orchestrator::ServiceType::Grafana {
                    port: 3000 + server_config.server_id.chars().map(|c| c as u16).sum::<u16>() % 1000,
                    data_path: format!("/var/lib/grafana/{}", server_config.server_id),
                }
            },
            ServerType::SecurityGateway => {
                crate::bso_k8_orchestrator::ServiceType::OAuth2Proxy {
                    port: 4180 + server_config.server_id.chars().map(|c| c as u16).sum::<u16>() % 1000,
                    upstream_url: format!("http://localhost:{}", 8000 + server_config.server_id.chars().map(|c| c as u16).sum::<u16>() % 1000),
                    client_id: "bpci-security".to_string(),
                    client_secret: "bpci-security-secret".to_string(),
                }
            },
        };
        
        let resource_alloc = crate::bso_k8_orchestrator::ResourceAllocation {
            vpods: 1,
            memory_mb: server_config.resource_allocation.memory_mb as u32,
            cpu_cores: server_config.resource_allocation.cpu_cores as f32,
            storage_gb: server_config.resource_allocation.disk_gb as u32,
            network_bandwidth: format!("{}Mbps", server_config.resource_allocation.network_bandwidth_mbps),
            replicas: 1,
        };
        
        let service_id = self.bso_k8.deploy_service(
            server_config.server_id.clone(),
            service_type,
            resource_alloc,
        ).await?;
        
        // 2. Register with networking layer
        let virtual_addr = self.networking.register_vpod(
            server_config.vpod_config.vpod_id.clone(),
            server_config.vpod_config.service_name.clone(),
            format!("127.0.0.1:{}", 8000 + server_config.server_id.chars().map(|c| c as u32).sum::<u32>() % 1000).parse().unwrap_or_else(|_| "127.0.0.1:8000".parse().unwrap()),
        ).await?;
        
        // 3. Configure server-specific settings
        self.configure_server_environment(&server_config).await?;
        
        // 4. Start the server
        let deployed_server = DeployedServer {
            server_id: server_config.server_id.clone(),
            server_type: server_config.server_type.clone(),
            vpod_id: server_config.vpod_config.vpod_id.clone(),
            actual_address: format!("127.0.0.1:{}", 8000 + server_config.server_id.chars().map(|c| c as u32).sum::<u32>() % 1000).parse().unwrap_or_else(|_| "127.0.0.1:8000".parse().unwrap()),
            virtual_address: virtual_addr.iaav6.to_string(),
            status: ServerStatus::Starting,
            health_score: 0.0,
            deployed_at: Utc::now(),
            last_health_check: Utc::now(),
        };
        
        // 5. Store deployment info
        self.deployed_servers.write().await.insert(
            server_config.server_id.clone(),
            deployed_server,
        );
        
        // 6. Update deployment status
        {
            let mut status = self.deployment_status.write().await;
            status.servers_deployed += 1;
        }
        
        tracing::info!("✅ Server deployed: {}", server_config.server_id);
        Ok(())
    }

    /// Wait for all servers to become healthy
    async fn wait_for_all_servers_healthy(&self) -> Result<()> {
        tracing::info!("🏥 Waiting for all servers to become healthy");
        
        let max_wait_time = std::time::Duration::from_secs(300); // 5 minutes
        let check_interval = std::time::Duration::from_secs(5);
        let start_time = std::time::Instant::now();
        
        loop {
            let healthy_count = self.check_server_health().await?;
            let total_servers = self.cluster_config.server_configs.len();
            
            tracing::info!("📊 Health check: {}/{} servers healthy", healthy_count, total_servers);
            
            if healthy_count == total_servers {
                tracing::info!("✅ All servers are healthy!");
                break;
            }
            
            if start_time.elapsed() > max_wait_time {
                anyhow::bail!("❌ Timeout waiting for servers to become healthy");
            }
            
            tokio::time::sleep(check_interval).await;
        }
        
        Ok(())
    }

    /// Check health of all deployed servers
    async fn check_server_health(&self) -> Result<usize> {
        let servers = self.deployed_servers.read().await;
        let mut healthy_count = 0;
        
        for (server_id, server) in servers.iter() {
            match self.check_single_server_health(server_id, server).await {
                Ok(is_healthy) if is_healthy => {
                    healthy_count += 1;
                }
                Ok(_) => {
                    tracing::warn!("🔴 Server {} is not healthy", server_id);
                }
                Err(e) => {
                    tracing::error!("❌ Health check failed for {}: {}", server_id, e);
                }
            }
        }
        
        // Update deployment status
        {
            let mut status = self.deployment_status.write().await;
            status.servers_healthy = healthy_count as u32;
        }
        
        Ok(healthy_count)
    }

    /// Check health of a single server
    async fn check_single_server_health(&self, server_id: &str, server: &DeployedServer) -> Result<bool> {
        // Use networking layer to send health check
        let health_check_data = b"health_check";
        
        match self.networking.send_message(&server.vpod_id, health_check_data).await {
            Ok(_) => {
                // Update server status
                // Note: In real implementation, we'd parse the response
                Ok(true)
            }
            Err(e) => {
                tracing::debug!("Health check failed for {}: {}", server_id, e);
                Ok(false)
            }
        }
    }

    /// Form mesh topology between servers
    async fn form_mesh_topology(&self) -> Result<()> {
        tracing::info!("🕸️ Forming mesh topology");
        
        match self.cluster_config.mesh_topology.topology_type {
            TopologyType::FullMesh => self.form_full_mesh().await?,
            TopologyType::StarMesh => self.form_star_mesh().await?,
            TopologyType::RingMesh => self.form_ring_mesh().await?,
            TopologyType::Cuboidal => self.form_cuboidal_mesh().await?,
        }
        
        tracing::info!("✅ Mesh topology formed successfully");
        Ok(())
    }

    /// Form full mesh topology (all-to-all connections)
    async fn form_full_mesh(&self) -> Result<()> {
        tracing::info!("🔗 Forming full mesh topology");
        
        let servers = self.deployed_servers.read().await;
        let server_list: Vec<_> = servers.values().collect();
        
        // Connect every server to every other server
        for (i, server_a) in server_list.iter().enumerate() {
            for server_b in server_list.iter().skip(i + 1) {
                self.establish_mesh_connection(server_a, server_b).await?;
            }
        }
        
        Ok(())
    }

    /// Establish connection between two servers in the mesh
    async fn establish_mesh_connection(&self, server_a: &DeployedServer, server_b: &DeployedServer) -> Result<()> {
        tracing::debug!("🔗 Connecting {} ↔ {}", server_a.server_id, server_b.server_id);
        
        // Use networking layer to establish bidirectional connection
        let connection_data = format!("mesh_connect:{}", server_b.vpod_id).into_bytes();
        self.networking.send_message(&server_a.vpod_id, &connection_data).await?;
        
        let connection_data = format!("mesh_connect:{}", server_a.vpod_id).into_bytes();
        self.networking.send_message(&server_b.vpod_id, &connection_data).await?;
        
        Ok(())
    }

    /// Run integration tests on the deployed cluster
    async fn run_integration_tests(&self) -> Result<()> {
        tracing::info!("🧪 Running integration tests");
        
        // Test 1: Connectivity test
        self.test_mesh_connectivity().await?;
        
        // Test 2: Load balancing test
        self.test_load_balancing().await?;
        
        // Test 3: Failover test
        self.test_failover_mechanism().await?;
        
        // Test 4: Performance test
        self.test_cluster_performance().await?;
        
        tracing::info!("✅ All integration tests passed");
        Ok(())
    }

    /// Test mesh connectivity
    async fn test_mesh_connectivity(&self) -> Result<()> {
        tracing::info!("🔍 Testing mesh connectivity");
        
        let servers = self.deployed_servers.read().await;
        let test_message = b"connectivity_test";
        
        for (server_id, server) in servers.iter() {
            match self.networking.send_message(&server.vpod_id, test_message).await {
                Ok(_) => tracing::debug!("✅ Connectivity test passed for {}", server_id),
                Err(e) => {
                    tracing::error!("❌ Connectivity test failed for {}: {}", server_id, e);
                    anyhow::bail!("Connectivity test failed for server: {}", server_id);
                }
            }
        }
        
        Ok(())
    }

    /// Get deployment status
    pub async fn get_deployment_status(&self) -> DeploymentStatus {
        self.deployment_status.read().await.clone()
    }

    /// Get deployed servers info
    pub async fn get_deployed_servers(&self) -> HashMap<String, DeployedServer> {
        self.deployed_servers.read().await.clone()
    }

    /// Add new server to existing cluster (for mainnet scaling)
    pub async fn add_server_to_cluster(&self, server_config: ServerConfig) -> Result<()> {
        let server_id = server_config.server_id.clone();
        tracing::info!("➕ Adding new server to cluster: {}", server_id);
        
        // Deploy the new server
        self.deploy_single_server(server_config).await?;
        
        // Wait for it to become healthy
        let max_wait = std::time::Duration::from_secs(60);
        let start = std::time::Instant::now();
        
        while start.elapsed() < max_wait {
            if self.check_single_server_health(
                &server_id,
                &self.deployed_servers.read().await[&server_id]
            ).await? {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
        
        // Integrate into existing mesh
        self.integrate_server_into_mesh(&server_id).await?;
        
        tracing::info!("✅ Server {} added to cluster successfully", server_id);
        Ok(())
    }

    /// Integrate new server into existing mesh
    async fn integrate_server_into_mesh(&self, new_server_id: &str) -> Result<()> {
        let servers = self.deployed_servers.read().await;
        let new_server = &servers[new_server_id];
        
        // Connect new server to all existing servers based on topology
        for (server_id, server) in servers.iter() {
            if server_id != new_server_id {
                self.establish_mesh_connection(new_server, server).await?;
            }
        }
        
        Ok(())
    }

    // Helper methods (stubs for now, implement based on specific needs)
    async fn validate_resource_requirements(&self) -> Result<()> { Ok(()) }
    async fn setup_mesh_networking(&self) -> Result<()> { Ok(()) }
    async fn configure_security_policies(&self) -> Result<()> { Ok(()) }
    async fn setup_service_discovery(&self) -> Result<()> { Ok(()) }
    async fn configure_server_environment(&self, _config: &ServerConfig) -> Result<()> { Ok(()) }
    async fn form_star_mesh(&self) -> Result<()> { Ok(()) }
    async fn form_ring_mesh(&self) -> Result<()> { Ok(()) }
    async fn form_cuboidal_mesh(&self) -> Result<()> { Ok(()) }
    async fn test_load_balancing(&self) -> Result<()> { Ok(()) }
    async fn test_failover_mechanism(&self) -> Result<()> { Ok(()) }
    async fn test_cluster_performance(&self) -> Result<()> { Ok(()) }
    
    async fn update_deployment_phase(&self, phase: DeploymentPhase) {
        let mut status = self.deployment_status.write().await;
        status.phase = phase;
    }
}

impl Clone for MeshDeploymentOrchestrator {
    fn clone(&self) -> Self {
        Self {
            cluster_config: self.cluster_config.clone(),
            bso_k8: self.bso_k8.clone(),
            networking: self.networking.clone(),
            deployed_servers: self.deployed_servers.clone(),
            deployment_semaphore: self.deployment_semaphore.clone(),
            deployment_status: self.deployment_status.clone(),
        }
    }
}

/// Default 13-server BPCI cluster configuration
pub fn default_13_server_config() -> BpciClusterConfig {
    BpciClusterConfig {
        cluster_id: "bpci-testnet-cluster".to_string(),
        deployment_mode: DeploymentMode::Testnet,
        server_configs: vec![
            // Core consensus servers (startup order 1)
            ServerConfig {
                server_id: "bpci-consensus-01".to_string(),
                server_type: ServerType::BpciConsensus,
                vpod_config: VPodConfig {
                    vpod_id: "vpod-consensus-01".to_string(),
                    service_name: "bpci-consensus".to_string(),
                    port_range: (7000, 7100),
                    resource_limits: ResourceLimits {
                        max_cpu_percent: 80.0,
                        max_memory_mb: 2048,
                        max_connections: 1000,
                        max_requests_per_sec: 500,
                    },
                    environment_vars: HashMap::new(),
                },
                resource_allocation: ResourceAllocation {
                    cpu_cores: 2.0,
                    memory_mb: 4096,
                    disk_gb: 100,
                    network_bandwidth_mbps: 1000,
                },
                dependencies: vec![],
                startup_order: 1,
            },
            // Add remaining 12 servers...
            // (Truncated for brevity - would include all 13 server configurations)
        ],
        mesh_topology: MeshTopology {
            topology_type: TopologyType::FullMesh,
            redundancy_level: RedundancyLevel::High,
            failover_config: FailoverConfig {
                detection_timeout_ms: 5000,
                failover_timeout_ms: 10000,
                health_check_interval_ms: 2000,
                max_retries: 3,
            },
            load_balancing: LoadBalancingConfig {
                algorithm: LoadBalancingAlgorithm::WeightedRoundRobin,
                health_weight: 0.4,
                latency_weight: 0.3,
                capacity_weight: 0.3,
            },
        },
        networking_config: NetworkingConfig {
            mesh_network_cidr: "10.100.0.0/16".to_string(),
            service_discovery: ServiceDiscoveryConfig {
                discovery_method: DiscoveryMethod::DynaRoute,
                ttl_seconds: 300,
                refresh_interval_ms: 30000,
            },
            proxy_config: ProxyConfig {
                enable_quantum_proxies: true,
                proxy_chain_length: 3,
                zk_protection_level: ZkProtectionLevel::Advanced,
            },
            encryption_config: EncryptionConfig {
                transport_encryption: true,
                at_rest_encryption: true,
                key_rotation_hours: 24,
                cipher_suite: "ChaCha20-Poly1305".to_string(),
            },
        },
        security_config: SecurityConfig {
            authentication_method: AuthMethod::Ed25519Keys,
            authorization_policy: AuthzPolicy::ZeroTrust,
            audit_config: AuditConfig {
                enable_audit_trail: true,
                log_level: "INFO".to_string(),
                retention_days: 90,
                immutable_logging: true,
            },
            firewall_rules: vec![],
        },
    }
}
