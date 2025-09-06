// Daemon Tree Manager - Hierarchical cluster, port, and server management
// Revolutionary orchestration system for 100-year future-proofing

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn, error};
use uuid::Uuid;

use crate::{NodeType, SecurityLevel, AuditEvent};

/// Daemon Tree Manager - Manages hierarchical daemon structure
#[derive(Debug)]
pub struct DaemonTreeManager {
    manager_id: String,
    root_daemon: Arc<RwLock<DaemonNode>>,
    daemon_registry: Arc<DashMap<String, Arc<RwLock<DaemonNode>>>>,
    port_registry: Arc<DashMap<u16, PortAllocation>>,
    server_registry: Arc<DashMap<String, ServerInstance>>,
    cluster_topology: Arc<RwLock<ClusterTopology>>,
    audit_trail: Arc<RwLock<Vec<AuditEvent>>>,
}

/// Daemon Node - Individual daemon in the tree hierarchy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonNode {
    pub daemon_id: String,
    pub daemon_type: DaemonType,
    pub parent_id: Option<String>,
    pub children: HashSet<String>,
    pub status: DaemonStatus,
    pub configuration: DaemonConfiguration,
    pub resource_allocation: DaemonResourceAllocation,
    pub port_assignments: Vec<u16>,
    pub server_instances: Vec<String>,
    pub health_metrics: HealthMetrics,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

/// Daemon Type - Different types of daemons in the hierarchy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DaemonType {
    /// Root daemon managing entire cluster
    ClusterRoot,
    /// Node daemon managing a single node
    NodeManager,
    /// Service daemon managing specific services
    ServiceManager,
    /// Port daemon managing port allocations
    PortManager,
    /// Server daemon managing server instances
    ServerManager,
    /// Load balancer daemon
    LoadBalancer,
    /// Monitor daemon for health checks
    Monitor,
    /// Security daemon for access control
    Security,
    /// Audit daemon for compliance
    Audit,
}

/// Daemon Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DaemonStatus {
    Initializing,
    Running,
    Paused,
    Stopping,
    Stopped,
    Failed,
    Maintenance,
}

/// Daemon Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfiguration {
    pub name: String,
    pub description: String,
    pub auto_restart: bool,
    pub restart_policy: RestartPolicy,
    pub environment_variables: HashMap<String, String>,
    pub command_line_args: Vec<String>,
    pub working_directory: Option<String>,
    pub log_level: String,
    pub security_context: SecurityContext,
}

/// Restart Policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RestartPolicy {
    Never,
    Always,
    OnFailure,
    UnlessStoppedManually,
}

/// Security Context for daemon execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    pub user_id: Option<u32>,
    pub group_id: Option<u32>,
    pub capabilities: Vec<String>,
    pub read_only_root_filesystem: bool,
    pub allow_privilege_escalation: bool,
    pub security_level: SecurityLevel,
}

/// Daemon Resource Allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonResourceAllocation {
    pub cpu_limit: f64,
    pub memory_limit_mb: u64,
    pub disk_limit_mb: u64,
    pub network_limit_mbps: f64,
    pub file_descriptor_limit: u32,
    pub process_limit: u32,
}

/// Health Metrics for daemon monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMetrics {
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: u64,
    pub disk_usage_mb: u64,
    pub network_io_mbps: f64,
    pub uptime_seconds: u64,
    pub restart_count: u32,
    pub last_health_check: DateTime<Utc>,
    pub health_status: HealthStatus,
}

/// Health Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

/// Port Allocation tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortAllocation {
    pub port: u16,
    pub daemon_id: String,
    pub service_name: String,
    pub protocol: PortProtocol,
    pub access_level: PortAccessLevel,
    pub allocated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Port Protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PortProtocol {
    TCP,
    UDP,
    HTTP,
    HTTPS,
    WebSocket,
    gRPC,
    Custom(String),
}

/// Port Access Level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PortAccessLevel {
    Internal,      // Only accessible within cluster
    ClusterWide,   // Accessible across cluster nodes
    External,      // Accessible from outside cluster
    Public,        // Publicly accessible
}

/// Server Instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInstance {
    pub instance_id: String,
    pub daemon_id: String,
    pub server_type: ServerType,
    pub configuration: ServerConfiguration,
    pub status: ServerStatus,
    pub endpoints: Vec<ServerEndpoint>,
    pub health_metrics: ServerHealthMetrics,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

/// Server Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerType {
    WebServer,
    ApiServer,
    DatabaseServer,
    CacheServer,
    MessageQueue,
    LoadBalancer,
    ProxyServer,
    FileServer,
    StreamingServer,
    Custom(String),
}

/// Server Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfiguration {
    pub name: String,
    pub version: String,
    pub bind_address: String,
    pub bind_port: u16,
    pub max_connections: u32,
    pub timeout_seconds: u32,
    pub ssl_enabled: bool,
    pub compression_enabled: bool,
    pub custom_config: HashMap<String, String>,
}

/// Server Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServerStatus {
    Starting,
    Running,
    Paused,
    Stopping,
    Stopped,
    Failed,
    Maintenance,
}

/// Server Endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerEndpoint {
    pub path: String,
    pub method: String,
    pub handler: String,
    pub middleware: Vec<String>,
    pub rate_limit: Option<RateLimit>,
    pub authentication_required: bool,
}

/// Rate Limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub window_size_seconds: u32,
}

/// Server Health Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerHealthMetrics {
    pub requests_per_second: f64,
    pub response_time_ms: f64,
    pub error_rate_percent: f64,
    pub active_connections: u32,
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: u64,
    pub last_health_check: DateTime<Utc>,
    pub health_status: HealthStatus,
}

/// Cluster Topology - Overall cluster structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterTopology {
    pub cluster_id: String,
    pub total_nodes: u32,
    pub active_nodes: u32,
    pub total_daemons: u32,
    pub active_daemons: u32,
    pub port_usage: PortUsageStats,
    pub server_usage: ServerUsageStats,
    pub resource_utilization: ClusterResourceUtilization,
    pub last_updated: DateTime<Utc>,
}

/// Port Usage Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortUsageStats {
    pub total_ports_allocated: u32,
    pub available_ports: u32,
    pub port_range_start: u16,
    pub port_range_end: u16,
    pub protocol_distribution: HashMap<String, u32>,
}

/// Server Usage Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerUsageStats {
    pub total_servers: u32,
    pub running_servers: u32,
    pub failed_servers: u32,
    pub server_type_distribution: HashMap<String, u32>,
    pub average_response_time_ms: f64,
    pub total_requests_per_second: f64,
}

/// Cluster Resource Utilization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterResourceUtilization {
    pub cpu_utilization_percent: f64,
    pub memory_utilization_percent: f64,
    pub disk_utilization_percent: f64,
    pub network_utilization_percent: f64,
    pub load_average: f64,
}

impl DaemonTreeManager {
    /// Create new daemon tree manager
    pub async fn new(cluster_id: String) -> Result<Self> {
        let manager_id = Uuid::new_v4().to_string();
        
        // Create root daemon
        let root_daemon = DaemonNode {
            daemon_id: format!("root-{}", Uuid::new_v4()),
            daemon_type: DaemonType::ClusterRoot,
            parent_id: None,
            children: HashSet::new(),
            status: DaemonStatus::Initializing,
            configuration: DaemonConfiguration {
                name: "cluster-root".to_string(),
                description: "Root daemon managing entire cluster".to_string(),
                auto_restart: true,
                restart_policy: RestartPolicy::Always,
                environment_variables: HashMap::new(),
                command_line_args: vec![],
                working_directory: None,
                log_level: "info".to_string(),
                security_context: SecurityContext {
                    user_id: None,
                    group_id: None,
                    capabilities: vec!["CAP_NET_ADMIN".to_string()],
                    read_only_root_filesystem: false,
                    allow_privilege_escalation: false,
                    security_level: SecurityLevel::High,
                },
            },
            resource_allocation: DaemonResourceAllocation {
                cpu_limit: 2.0,
                memory_limit_mb: 1024,
                disk_limit_mb: 10240,
                network_limit_mbps: 1000.0,
                file_descriptor_limit: 10000,
                process_limit: 1000,
            },
            port_assignments: vec![],
            server_instances: vec![],
            health_metrics: HealthMetrics {
                cpu_usage_percent: 0.0,
                memory_usage_mb: 0,
                disk_usage_mb: 0,
                network_io_mbps: 0.0,
                uptime_seconds: 0,
                restart_count: 0,
                last_health_check: Utc::now(),
                health_status: HealthStatus::Unknown,
            },
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };
        
        let root_daemon_arc = Arc::new(RwLock::new(root_daemon.clone()));
        let daemon_registry = Arc::new(DashMap::new());
        daemon_registry.insert(root_daemon.daemon_id.clone(), root_daemon_arc.clone());
        
        let cluster_topology = ClusterTopology {
            cluster_id,
            total_nodes: 0,
            active_nodes: 0,
            total_daemons: 1,
            active_daemons: 0,
            port_usage: PortUsageStats {
                total_ports_allocated: 0,
                available_ports: 65535 - 1024, // Exclude system ports
                port_range_start: 1024,
                port_range_end: 65535,
                protocol_distribution: HashMap::new(),
            },
            server_usage: ServerUsageStats {
                total_servers: 0,
                running_servers: 0,
                failed_servers: 0,
                server_type_distribution: HashMap::new(),
                average_response_time_ms: 0.0,
                total_requests_per_second: 0.0,
            },
            resource_utilization: ClusterResourceUtilization {
                cpu_utilization_percent: 0.0,
                memory_utilization_percent: 0.0,
                disk_utilization_percent: 0.0,
                network_utilization_percent: 0.0,
                load_average: 0.0,
            },
            last_updated: Utc::now(),
        };
        
        Ok(Self {
            manager_id,
            root_daemon: root_daemon_arc,
            daemon_registry,
            port_registry: Arc::new(DashMap::new()),
            server_registry: Arc::new(DashMap::new()),
            cluster_topology: Arc::new(RwLock::new(cluster_topology)),
            audit_trail: Arc::new(RwLock::new(Vec::new())),
        })
    }
    
    /// Create a new daemon node
    pub async fn create_daemon(
        &self,
        daemon_type: DaemonType,
        parent_id: Option<String>,
        configuration: DaemonConfiguration,
    ) -> Result<String> {
        let daemon_id = format!("{:?}-{}", daemon_type, Uuid::new_v4());
        
        let daemon_node = DaemonNode {
            daemon_id: daemon_id.clone(),
            daemon_type: daemon_type.clone(),
            parent_id: parent_id.clone(),
            children: HashSet::new(),
            status: DaemonStatus::Initializing,
            configuration,
            resource_allocation: self.calculate_resource_allocation(&daemon_type),
            port_assignments: vec![],
            server_instances: vec![],
            health_metrics: HealthMetrics {
                cpu_usage_percent: 0.0,
                memory_usage_mb: 0,
                disk_usage_mb: 0,
                network_io_mbps: 0.0,
                uptime_seconds: 0,
                restart_count: 0,
                last_health_check: Utc::now(),
                health_status: HealthStatus::Unknown,
            },
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };
        
        // Add to registry
        let daemon_arc = Arc::new(RwLock::new(daemon_node));
        self.daemon_registry.insert(daemon_id.clone(), daemon_arc);
        
        // Update parent's children if parent exists
        if let Some(parent_id) = parent_id {
            if let Some(parent_daemon_arc) = self.daemon_registry.get(&parent_id) {
                let mut parent_daemon = parent_daemon_arc.write().await;
                parent_daemon.children.insert(daemon_id.clone());
                parent_daemon.last_updated = Utc::now();
            }
        }
        
        // Update cluster topology
        let mut topology = self.cluster_topology.write().await;
        topology.total_daemons += 1;
        topology.last_updated = Utc::now();
        
        // Log audit event
        self.log_audit_event(AuditEvent {
            event_id: Uuid::new_v4().to_string(),
            event_type: "daemon_created".to_string(),
            timestamp: Utc::now(),
            details: serde_json::json!({
                "daemon_id": daemon_id,
                "daemon_type": daemon_type,
                "parent_id": parent_id
            }),
        }).await;
        
        info!("✅ Created daemon: {} (type: {:?})", daemon_id, daemon_type);
        Ok(daemon_id)
    }
    
    /// Start a daemon
    pub async fn start_daemon(&self, daemon_id: &str) -> Result<()> {
        if let Some(daemon_arc) = self.daemon_registry.get(daemon_id) {
            let mut daemon = daemon_arc.write().await;
            daemon.status = DaemonStatus::Running;
            daemon.last_updated = Utc::now();
            daemon.health_metrics.last_health_check = Utc::now();
            daemon.health_metrics.health_status = HealthStatus::Healthy;
            
            // Update cluster topology
            let mut topology = self.cluster_topology.write().await;
            topology.active_daemons += 1;
            topology.last_updated = Utc::now();
            
            info!("🚀 Started daemon: {}", daemon_id);
            Ok(())
        } else {
            Err(anyhow!("Daemon not found: {}", daemon_id))
        }
    }
    
    /// Stop a daemon
    pub async fn stop_daemon(&self, daemon_id: &str) -> Result<()> {
        if let Some(daemon_arc) = self.daemon_registry.get(daemon_id) {
            let mut daemon = daemon_arc.write().await;
            daemon.status = DaemonStatus::Stopped;
            daemon.last_updated = Utc::now();
            
            // Update cluster topology
            let mut topology = self.cluster_topology.write().await;
            if topology.active_daemons > 0 {
                topology.active_daemons -= 1;
            }
            topology.last_updated = Utc::now();
            
            info!("🛑 Stopped daemon: {}", daemon_id);
            Ok(())
        } else {
            Err(anyhow!("Daemon not found: {}", daemon_id))
        }
    }
    
    /// Allocate a port to a daemon
    pub async fn allocate_port(
        &self,
        daemon_id: &str,
        service_name: String,
        protocol: PortProtocol,
        access_level: PortAccessLevel,
        preferred_port: Option<u16>,
    ) -> Result<u16> {
        // Find available port
        let port = if let Some(preferred) = preferred_port {
            if self.is_port_available(preferred) {
                preferred
            } else {
                self.find_available_port(1024, 65535)?
            }
        } else {
            self.find_available_port(1024, 65535)?
        };
        
        // Create port allocation
        let allocation = PortAllocation {
            port,
            daemon_id: daemon_id.to_string(),
            service_name,
            protocol,
            access_level,
            allocated_at: Utc::now(),
            expires_at: None,
        };
        
        // Add to port registry
        self.port_registry.insert(port, allocation);
        
        // Update daemon's port assignments
        if let Some(daemon_arc) = self.daemon_registry.get(daemon_id) {
            let mut daemon = daemon_arc.write().await;
            daemon.port_assignments.push(port);
            daemon.last_updated = Utc::now();
        }
        
        // Update cluster topology
        let mut topology = self.cluster_topology.write().await;
        topology.port_usage.total_ports_allocated += 1;
        topology.port_usage.available_ports = topology.port_usage.available_ports.saturating_sub(1);
        topology.last_updated = Utc::now();
        
        info!("🔌 Allocated port {} to daemon: {}", port, daemon_id);
        Ok(port)
    }
    
    /// Create a server instance
    pub async fn create_server(
        &self,
        daemon_id: &str,
        server_type: ServerType,
        configuration: ServerConfiguration,
    ) -> Result<String> {
        let instance_id = format!("server-{}", Uuid::new_v4());
        
        let server_instance = ServerInstance {
            instance_id: instance_id.clone(),
            daemon_id: daemon_id.to_string(),
            server_type: server_type.clone(),
            configuration,
            status: ServerStatus::Starting,
            endpoints: vec![],
            health_metrics: ServerHealthMetrics {
                requests_per_second: 0.0,
                response_time_ms: 0.0,
                error_rate_percent: 0.0,
                active_connections: 0,
                cpu_usage_percent: 0.0,
                memory_usage_mb: 0,
                last_health_check: Utc::now(),
                health_status: HealthStatus::Unknown,
            },
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };
        
        // Add to server registry
        self.server_registry.insert(instance_id.clone(), server_instance);
        
        // Update daemon's server instances
        if let Some(daemon_arc) = self.daemon_registry.get(daemon_id) {
            let mut daemon = daemon_arc.write().await;
            daemon.server_instances.push(instance_id.clone());
            daemon.last_updated = Utc::now();
        }
        
        // Update cluster topology
        let mut topology = self.cluster_topology.write().await;
        topology.server_usage.total_servers += 1;
        topology.last_updated = Utc::now();
        
        info!("🖥️ Created server instance: {} (type: {:?})", instance_id, server_type);
        Ok(instance_id)
    }
    
    /// Get daemon tree structure
    pub async fn get_daemon_tree(&self) -> Result<serde_json::Value> {
        let root_daemon = self.root_daemon.read().await;
        let tree = self.build_daemon_tree_recursive(&root_daemon.daemon_id).await?;
        Ok(serde_json::to_value(tree)?)
    }
    
    /// Get cluster topology
    pub async fn get_cluster_topology(&self) -> ClusterTopology {
        self.cluster_topology.read().await.clone()
    }
    
    /// Get daemon status
    pub async fn get_daemon_status(&self, daemon_id: &str) -> Result<DaemonNode> {
        if let Some(daemon_arc) = self.daemon_registry.get(daemon_id) {
            Ok(daemon_arc.read().await.clone())
        } else {
            Err(anyhow!("Daemon not found: {}", daemon_id))
        }
    }
    
    /// Update daemon health metrics
    pub async fn update_daemon_health(&self, daemon_id: &str, metrics: HealthMetrics) -> Result<()> {
        if let Some(daemon_arc) = self.daemon_registry.get(daemon_id) {
            let mut daemon = daemon_arc.write().await;
            daemon.health_metrics = metrics;
            daemon.last_updated = Utc::now();
            Ok(())
        } else {
            Err(anyhow!("Daemon not found: {}", daemon_id))
        }
    }
    
    // Private helper methods
    
    fn calculate_resource_allocation(&self, daemon_type: &DaemonType) -> DaemonResourceAllocation {
        match daemon_type {
            DaemonType::ClusterRoot => DaemonResourceAllocation {
                cpu_limit: 2.0,
                memory_limit_mb: 1024,
                disk_limit_mb: 10240,
                network_limit_mbps: 1000.0,
                file_descriptor_limit: 10000,
                process_limit: 1000,
            },
            DaemonType::NodeManager => DaemonResourceAllocation {
                cpu_limit: 1.0,
                memory_limit_mb: 512,
                disk_limit_mb: 5120,
                network_limit_mbps: 500.0,
                file_descriptor_limit: 5000,
                process_limit: 500,
            },
            DaemonType::ServiceManager => DaemonResourceAllocation {
                cpu_limit: 0.5,
                memory_limit_mb: 256,
                disk_limit_mb: 2048,
                network_limit_mbps: 200.0,
                file_descriptor_limit: 2000,
                process_limit: 200,
            },
            _ => DaemonResourceAllocation {
                cpu_limit: 0.25,
                memory_limit_mb: 128,
                disk_limit_mb: 1024,
                network_limit_mbps: 100.0,
                file_descriptor_limit: 1000,
                process_limit: 100,
            },
        }
    }
    
    fn is_port_available(&self, port: u16) -> bool {
        !self.port_registry.contains_key(&port)
    }
    
    fn find_available_port(&self, start: u16, end: u16) -> Result<u16> {
        for port in start..=end {
            if self.is_port_available(port) {
                return Ok(port);
            }
        }
        Err(anyhow!("No available ports in range {}-{}", start, end))
    }
    
    async fn build_daemon_tree_recursive(&self, daemon_id: &str) -> Result<serde_json::Value> {
        if let Some(daemon_arc) = self.daemon_registry.get(daemon_id) {
            let daemon = daemon_arc.read().await;
            let mut tree = serde_json::json!({
                "daemon_id": daemon.daemon_id,
                "daemon_type": daemon.daemon_type,
                "status": daemon.status,
                "children": []
            });
            
            let mut children = Vec::new();
            for child_id in &daemon.children {
                let child_tree = self.build_daemon_tree_recursive(child_id).await?;
                children.push(child_tree);
            }
            tree["children"] = serde_json::Value::Array(children);
            
            Ok(tree)
        } else {
            Err(anyhow!("Daemon not found: {}", daemon_id))
        }
    }
    
    async fn log_audit_event(&self, event: AuditEvent) {
        let mut audit_trail = self.audit_trail.write().await;
        audit_trail.push(event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_daemon_tree_creation() {
        let manager = DaemonTreeManager::new("test-cluster".to_string()).await.unwrap();
        
        // Create a node manager daemon
        let node_daemon_id = manager.create_daemon(
            DaemonType::NodeManager,
            None,
            DaemonConfiguration {
                name: "test-node-manager".to_string(),
                description: "Test node manager".to_string(),
                auto_restart: true,
                restart_policy: RestartPolicy::OnFailure,
                environment_variables: HashMap::new(),
                command_line_args: vec![],
                working_directory: None,
                log_level: "debug".to_string(),
                security_context: SecurityContext {
                    user_id: Some(1000),
                    group_id: Some(1000),
                    capabilities: vec![],
                    read_only_root_filesystem: true,
                    allow_privilege_escalation: false,
                    security_level: SecurityLevel::Medium,
                },
            },
        ).await.unwrap();
        
        // Start the daemon
        manager.start_daemon(&node_daemon_id).await.unwrap();
        
        // Check daemon status
        let status = manager.get_daemon_status(&node_daemon_id).await.unwrap();
        assert_eq!(status.status, DaemonStatus::Running);
        
        // Allocate a port
        let port = manager.allocate_port(
            &node_daemon_id,
            "test-service".to_string(),
            PortProtocol::HTTP,
            PortAccessLevel::Internal,
            Some(8080),
        ).await.unwrap();
        assert_eq!(port, 8080);
        
        // Create a server instance
        let server_id = manager.create_server(
            &node_daemon_id,
            ServerType::WebServer,
            ServerConfiguration {
                name: "test-web-server".to_string(),
                version: "1.0.0".to_string(),
                bind_address: "0.0.0.0".to_string(),
                bind_port: port,
                max_connections: 1000,
                timeout_seconds: 30,
                ssl_enabled: false,
                compression_enabled: true,
                custom_config: HashMap::new(),
            },
        ).await.unwrap();
        
        assert!(server_id.starts_with("server-"));
        
        // Get cluster topology
        let topology = manager.get_cluster_topology().await;
        assert_eq!(topology.total_daemons, 2); // Root + node manager
        assert_eq!(topology.active_daemons, 1); // Only node manager is started
        assert_eq!(topology.port_usage.total_ports_allocated, 1);
        assert_eq!(topology.server_usage.total_servers, 1);
    }
}
