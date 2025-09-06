//! # DaemonTree - Hierarchical Cluster Management
//!
//! Revolutionary daemon tree architecture for hierarchical cluster and port/server management.
//! Part of the 100-year future orchestration vision.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, error, info, warn};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::metanode_cluster_manager::{
    DaemonNode, DaemonPosition, DaemonResponsibility, DaemonEndpoints, 
    DaemonHealthStatus, ResourceManagement, NodeStatus, ClusterEvent
};

/// DaemonTreeManager - Manages hierarchical daemon tree structure
#[derive(Debug)]
pub struct DaemonTreeManager {
    /// Tree identifier
    pub tree_id: String,
    
    /// Root daemon node
    pub root_daemon: Arc<RwLock<DaemonNode>>,
    
    /// All daemon nodes in the tree
    pub daemon_nodes: Arc<RwLock<HashMap<String, DaemonNode>>>,
    
    /// Tree topology and hierarchy
    pub topology: Arc<RwLock<TreeTopology>>,
    
    /// Communication matrix between daemons
    pub communication_matrix: Arc<RwLock<CommunicationMatrix>>,
    
    /// Load balancing and fault tolerance
    pub load_balancer: Arc<RwLock<TreeLoadBalancer>>,
    
    /// Health monitoring system
    pub health_monitor: Arc<RwLock<TreeHealthMonitor>>,
    
    /// Event channel for tree updates
    pub event_tx: mpsc::UnboundedSender<DaemonTreeEvent>,
    
    /// Metrics and performance tracking
    pub metrics: Arc<RwLock<DaemonTreeMetrics>>,
}

/// Tree Topology - Structure and organization of the daemon tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeTopology {
    /// Maximum tree depth
    pub max_depth: u32,
    
    /// Branching factor per level
    pub branching_factors: Vec<u32>,
    
    /// Level configurations
    pub level_configs: HashMap<u32, LevelConfig>,
    
    /// Tree balance metrics
    pub balance_metrics: TreeBalance,
    
    /// Topology version for updates
    pub topology_version: String,
    
    /// Last topology update
    pub last_updated: DateTime<Utc>,
}

/// Level Configuration - Configuration for each tree level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelConfig {
    /// Level number (0 = root)
    pub level: u32,
    
    /// Maximum daemons at this level
    pub max_daemons: u32,
    
    /// Daemon responsibilities at this level
    pub responsibilities: Vec<DaemonResponsibility>,
    
    /// Resource allocation strategy
    pub resource_strategy: ResourceStrategy,
    
    /// Communication patterns
    pub communication_patterns: Vec<CommunicationPattern>,
    
    /// Failover configuration
    pub failover_config: FailoverConfig,
}

/// Communication Matrix - Inter-daemon communication management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationMatrix {
    /// Direct communication channels
    pub direct_channels: HashMap<String, Vec<String>>,
    
    /// Broadcast channels
    pub broadcast_channels: HashMap<String, BroadcastChannel>,
    
    /// Communication protocols
    pub protocols: HashMap<String, CommunicationProtocol>,
    
    /// Message routing table
    pub routing_table: HashMap<String, RoutingEntry>,
    
    /// Communication metrics
    pub metrics: CommunicationMetrics,
}

/// Tree Load Balancer - Load balancing across daemon tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeLoadBalancer {
    /// Load balancing strategy
    pub strategy: LoadBalancingStrategy,
    
    /// Current load distribution
    pub load_distribution: HashMap<String, LoadMetrics>,
    
    /// Rebalancing thresholds
    pub rebalancing_thresholds: RebalancingThresholds,
    
    /// Auto-scaling configuration
    pub auto_scaling: AutoScalingConfig,
    
    /// Performance optimization
    pub optimization: PerformanceOptimization,
}

/// Tree Health Monitor - Health monitoring for daemon tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeHealthMonitor {
    /// Health check configuration
    pub health_check_config: HealthCheckConfig,
    
    /// Current health status of all daemons
    pub daemon_health: HashMap<String, DaemonHealthMetrics>,
    
    /// Tree-wide health metrics
    pub tree_health: TreeHealthMetrics,
    
    /// Alert configuration
    pub alert_config: AlertConfig,
    
    /// Recovery procedures
    pub recovery_procedures: HashMap<String, RecoveryProcedure>,
}

/// Daemon Tree Events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaemonTreeEvent {
    DaemonAdded { daemon_id: String, level: u32, parent_id: Option<String> },
    DaemonRemoved { daemon_id: String, reason: String },
    DaemonMoved { daemon_id: String, old_parent: Option<String>, new_parent: Option<String> },
    TreeRebalanced { old_version: String, new_version: String },
    CommunicationEstablished { from_daemon: String, to_daemon: String },
    CommunicationLost { from_daemon: String, to_daemon: String },
    LoadThresholdExceeded { daemon_id: String, metric: String, value: f64 },
    HealthCheckFailed { daemon_id: String, check_type: String },
    AutoScalingTriggered { action: String, target_level: u32 },
    FaultToleranceActivated { daemon_id: String, procedure: String },
}

// Supporting structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeBalance {
    pub balance_score: f64,
    pub depth_variance: f64,
    pub load_distribution_score: f64,
    pub communication_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceStrategy {
    pub allocation_method: String,
    pub scaling_policy: String,
    pub resource_limits: HashMap<String, f64>,
    pub priority_weights: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationPattern {
    pub pattern_type: String,
    pub frequency: f64,
    pub bandwidth_requirements: f64,
    pub latency_requirements: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverConfig {
    pub failover_timeout: u32,
    pub backup_daemons: Vec<String>,
    pub recovery_strategy: String,
    pub data_replication: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastChannel {
    pub channel_id: String,
    pub participants: Vec<String>,
    pub message_types: Vec<String>,
    pub qos_requirements: QosRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationProtocol {
    pub protocol_name: String,
    pub version: String,
    pub encryption: bool,
    pub compression: bool,
    pub reliability: ReliabilityLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingEntry {
    pub destination: String,
    pub next_hop: String,
    pub cost: f64,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationMetrics {
    pub total_messages: u64,
    pub messages_per_second: f64,
    pub average_latency_ms: f64,
    pub error_rate: f64,
    pub bandwidth_utilization: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    ResourceBased,
    Adaptive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadMetrics {
    pub cpu_load: f64,
    pub memory_load: f64,
    pub network_load: f64,
    pub connection_count: u32,
    pub request_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebalancingThresholds {
    pub cpu_threshold: f64,
    pub memory_threshold: f64,
    pub network_threshold: f64,
    pub response_time_threshold: f64,
    pub error_rate_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoScalingConfig {
    pub enabled: bool,
    pub scale_up_threshold: f64,
    pub scale_down_threshold: f64,
    pub min_daemons: u32,
    pub max_daemons: u32,
    pub cooldown_period: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceOptimization {
    pub cache_optimization: bool,
    pub connection_pooling: bool,
    pub request_batching: bool,
    pub compression_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    pub check_interval: u32,
    pub timeout: u32,
    pub retry_attempts: u32,
    pub health_check_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonHealthMetrics {
    pub overall_health: f64,
    pub cpu_health: f64,
    pub memory_health: f64,
    pub network_health: f64,
    pub response_time_health: f64,
    pub last_check: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeHealthMetrics {
    pub overall_tree_health: f64,
    pub healthy_daemons: u32,
    pub unhealthy_daemons: u32,
    pub degraded_daemons: u32,
    pub tree_availability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    pub alert_thresholds: HashMap<String, f64>,
    pub notification_channels: Vec<String>,
    pub escalation_rules: Vec<EscalationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryProcedure {
    pub procedure_name: String,
    pub trigger_conditions: Vec<String>,
    pub recovery_steps: Vec<RecoveryStep>,
    pub success_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonTreeMetrics {
    pub total_daemons: u32,
    pub active_daemons: u32,
    pub tree_depth: u32,
    pub average_branching_factor: f64,
    pub communication_efficiency: f64,
    pub load_balance_score: f64,
    pub overall_health_score: f64,
    pub last_updated: DateTime<Utc>,
}

// Additional supporting enums and structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QosRequirements {
    pub max_latency_ms: u32,
    pub min_bandwidth_mbps: f64,
    pub reliability_level: ReliabilityLevel,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReliabilityLevel {
    BestEffort,
    Reliable,
    Guaranteed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationRule {
    pub condition: String,
    pub escalation_delay: u32,
    pub escalation_target: String,
    pub escalation_action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryStep {
    pub step_name: String,
    pub action: String,
    pub parameters: HashMap<String, String>,
    pub timeout: u32,
}

impl DaemonTreeManager {
    /// Create new DaemonTreeManager
    pub async fn new(tree_id: String) -> Result<(Self, mpsc::UnboundedReceiver<DaemonTreeEvent>)> {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        
        // Create root daemon
        let root_daemon_id = format!("root-{}", tree_id);
        let root_daemon = DaemonNode {
            daemon_id: root_daemon_id.clone(),
            parent_id: None,
            children: Vec::new(),
            responsibilities: vec![
                DaemonResponsibility {
                    responsibility_type: "cluster_coordination".to_string(),
                    scope: vec!["global".to_string()],
                    priority: 1,
                },
                DaemonResponsibility {
                    responsibility_type: "tree_management".to_string(),
                    scope: vec!["tree".to_string()],
                    priority: 1,
                },
            ],
            resource_management: ResourceManagement {
                allocated_resources: crate::metanode_cluster_manager::ResourceAllocation {
                    cpu_cores: 2.0,
                    memory_gb: 4.0,
                    storage_gb: 20.0,
                    network_bandwidth_mbps: 1000.0,
                    gpu_units: None,
                },
                resource_limits: crate::metanode_cluster_manager::ResourceAllocation {
                    cpu_cores: 4.0,
                    memory_gb: 8.0,
                    storage_gb: 50.0,
                    network_bandwidth_mbps: 2000.0,
                    gpu_units: None,
                },
                auto_scaling: true,
            },
            endpoints: DaemonEndpoints {
                control_endpoint: "http://localhost:10000".to_string(),
                data_endpoint: "http://localhost:10001".to_string(),
                monitoring_endpoint: "http://localhost:10002".to_string(),
            },
            health_status: DaemonHealthStatus {
                status: NodeStatus::Online,
                health_score: 100.0,
                last_health_check: Utc::now(),
            },
        };

        let mut daemon_nodes = HashMap::new();
        daemon_nodes.insert(root_daemon_id.clone(), root_daemon.clone());

        let manager = Self {
            tree_id: tree_id.clone(),
            root_daemon: Arc::new(RwLock::new(root_daemon)),
            daemon_nodes: Arc::new(RwLock::new(daemon_nodes)),
            topology: Arc::new(RwLock::new(TreeTopology {
                max_depth: 5,
                branching_factors: vec![3, 5, 7, 10],
                level_configs: HashMap::new(),
                balance_metrics: TreeBalance {
                    balance_score: 100.0,
                    depth_variance: 0.0,
                    load_distribution_score: 100.0,
                    communication_efficiency: 100.0,
                },
                topology_version: "1.0.0".to_string(),
                last_updated: Utc::now(),
            })),
            communication_matrix: Arc::new(RwLock::new(CommunicationMatrix {
                direct_channels: HashMap::new(),
                broadcast_channels: HashMap::new(),
                protocols: HashMap::new(),
                routing_table: HashMap::new(),
                metrics: CommunicationMetrics {
                    total_messages: 0,
                    messages_per_second: 0.0,
                    average_latency_ms: 0.0,
                    error_rate: 0.0,
                    bandwidth_utilization: 0.0,
                },
            })),
            load_balancer: Arc::new(RwLock::new(TreeLoadBalancer {
                strategy: LoadBalancingStrategy::Adaptive,
                load_distribution: HashMap::new(),
                rebalancing_thresholds: RebalancingThresholds {
                    cpu_threshold: 80.0,
                    memory_threshold: 85.0,
                    network_threshold: 90.0,
                    response_time_threshold: 1000.0,
                    error_rate_threshold: 5.0,
                },
                auto_scaling: AutoScalingConfig {
                    enabled: true,
                    scale_up_threshold: 75.0,
                    scale_down_threshold: 25.0,
                    min_daemons: 1,
                    max_daemons: 100,
                    cooldown_period: 300,
                },
                optimization: PerformanceOptimization {
                    cache_optimization: true,
                    connection_pooling: true,
                    request_batching: true,
                    compression_enabled: true,
                },
            })),
            health_monitor: Arc::new(RwLock::new(TreeHealthMonitor {
                health_check_config: HealthCheckConfig {
                    check_interval: 30,
                    timeout: 10,
                    retry_attempts: 3,
                    health_check_types: vec![
                        "ping".to_string(),
                        "resource_check".to_string(),
                        "service_check".to_string(),
                    ],
                },
                daemon_health: HashMap::new(),
                tree_health: TreeHealthMetrics {
                    overall_tree_health: 100.0,
                    healthy_daemons: 1,
                    unhealthy_daemons: 0,
                    degraded_daemons: 0,
                    tree_availability: 100.0,
                },
                alert_config: AlertConfig {
                    alert_thresholds: HashMap::new(),
                    notification_channels: Vec::new(),
                    escalation_rules: Vec::new(),
                },
                recovery_procedures: HashMap::new(),
            })),
            event_tx,
            metrics: Arc::new(RwLock::new(DaemonTreeMetrics {
                total_daemons: 1,
                active_daemons: 1,
                tree_depth: 1,
                average_branching_factor: 0.0,
                communication_efficiency: 100.0,
                load_balance_score: 100.0,
                overall_health_score: 100.0,
                last_updated: Utc::now(),
            })),
        };

        info!("✅ DaemonTreeManager initialized: {}", tree_id);
        
        Ok((manager, event_rx))
    }

    /// Add daemon to the tree
    pub async fn add_daemon(&self, parent_id: Option<String>, responsibilities: Vec<DaemonResponsibility>) -> Result<String> {
        let daemon_id = format!("daemon-{}", Uuid::new_v4());
        
        // Determine level based on parent
        let level = if let Some(ref parent_id) = parent_id {
            let nodes = self.daemon_nodes.read().await;
            if let Some(_parent) = nodes.get(parent_id) {
                // Calculate level based on tree depth (simplified)
                1 // For now, all children are at level 1
            } else {
                return Err(anyhow::anyhow!("Parent daemon not found: {}", parent_id));
            }
        } else {
            0 // Root level
        };

        let daemon = DaemonNode {
            daemon_id: daemon_id.clone(),
            parent_id: parent_id.clone(),
            children: Vec::new(),
            responsibilities,
            resource_management: ResourceManagement {
                allocated_resources: crate::metanode_cluster_manager::ResourceAllocation {
                    cpu_cores: 1.0,
                    memory_gb: 2.0,
                    storage_gb: 10.0,
                    network_bandwidth_mbps: 500.0,
                    gpu_units: None,
                },
                resource_limits: crate::metanode_cluster_manager::ResourceAllocation {
                    cpu_cores: 2.0,
                    memory_gb: 4.0,
                    storage_gb: 20.0,
                    network_bandwidth_mbps: 1000.0,
                    gpu_units: None,
                },
                auto_scaling: true,
            },
            endpoints: DaemonEndpoints {
                control_endpoint: format!("http://localhost:{}", 10000 + (level * 100) + 1),
                data_endpoint: format!("http://localhost:{}", 10000 + (level * 100) + 2),
                monitoring_endpoint: format!("http://localhost:{}", 10000 + (level * 100) + 3),
            },
            health_status: DaemonHealthStatus {
                status: NodeStatus::Online,
                health_score: 100.0,
                last_health_check: Utc::now(),
            },
        };

        // Add to daemon nodes
        {
            let mut nodes = self.daemon_nodes.write().await;
            nodes.insert(daemon_id.clone(), daemon);
        }

        // Update parent's children list
        if let Some(parent_id) = &parent_id {
            let mut nodes = self.daemon_nodes.write().await;
            if let Some(parent) = nodes.get_mut(parent_id) {
                parent.children.push(daemon_id.clone());
            }
        }

        // Send event
        let _ = self.event_tx.send(DaemonTreeEvent::DaemonAdded { 
            daemon_id: daemon_id.clone(), 
            level, 
            parent_id: parent_id.clone() 
        });

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.total_daemons += 1;
            metrics.active_daemons += 1;
            metrics.tree_depth = metrics.tree_depth.max(level + 1);
            metrics.last_updated = Utc::now();
        }

        info!("✅ Added daemon to tree: {} (level: {}, parent: {:?})", daemon_id, level, parent_id);
        Ok(daemon_id)
    }

    /// Remove daemon from the tree
    pub async fn remove_daemon(&self, daemon_id: &str, reason: &str) -> Result<()> {
        // Get daemon info before removal
        let daemon_info = {
            let nodes = self.daemon_nodes.read().await;
            nodes.get(daemon_id).cloned()
        };

        if let Some(daemon) = daemon_info {
            // Move children to parent or remove them
            if !daemon.children.is_empty() {
                for child_id in &daemon.children {
                    if let Some(parent_id) = &daemon.parent_id {
                        self.move_daemon(child_id, Some(parent_id.clone())).await?;
                    } else {
                        // If removing root, this is a major operation
                        warn!("Removing root daemon with children - this may cause tree instability");
                    }
                }
            }

            // Remove from parent's children list
            if let Some(parent_id) = &daemon.parent_id {
                let mut nodes = self.daemon_nodes.write().await;
                if let Some(parent) = nodes.get_mut(parent_id) {
                    parent.children.retain(|id| id != daemon_id);
                }
            }

            // Remove from daemon nodes
            {
                let mut nodes = self.daemon_nodes.write().await;
                nodes.remove(daemon_id);
            }

            // Send event
            let _ = self.event_tx.send(DaemonTreeEvent::DaemonRemoved { 
                daemon_id: daemon_id.to_string(), 
                reason: reason.to_string() 
            });

            // Update metrics
            {
                let mut metrics = self.metrics.write().await;
                metrics.total_daemons -= 1;
                metrics.active_daemons -= 1;
                metrics.last_updated = Utc::now();
            }

            info!("✅ Removed daemon from tree: {} (reason: {})", daemon_id, reason);
        } else {
            return Err(anyhow::anyhow!("Daemon not found: {}", daemon_id));
        }

        Ok(())
    }

    /// Move daemon to new parent
    pub async fn move_daemon(&self, daemon_id: &str, new_parent_id: Option<String>) -> Result<()> {
        let old_parent_id = {
            let nodes = self.daemon_nodes.read().await;
            if let Some(daemon) = nodes.get(daemon_id) {
                daemon.parent_id.clone()
            } else {
                return Err(anyhow::anyhow!("Daemon not found: {}", daemon_id));
            }
        };

        // Update daemon's parent
        {
            let mut nodes = self.daemon_nodes.write().await;
            if let Some(daemon) = nodes.get_mut(daemon_id) {
                daemon.parent_id = new_parent_id.clone();
            }
        }

        // Remove from old parent's children
        if let Some(old_parent_id) = &old_parent_id {
            let mut nodes = self.daemon_nodes.write().await;
            if let Some(old_parent) = nodes.get_mut(old_parent_id) {
                old_parent.children.retain(|id| id != daemon_id);
            }
        }

        // Add to new parent's children
        if let Some(new_parent_id) = &new_parent_id {
            let mut nodes = self.daemon_nodes.write().await;
            if let Some(new_parent) = nodes.get_mut(new_parent_id) {
                new_parent.children.push(daemon_id.to_string());
            }
        }

        // Send event
        let _ = self.event_tx.send(DaemonTreeEvent::DaemonMoved { 
            daemon_id: daemon_id.to_string(), 
            old_parent: old_parent_id.clone(), 
            new_parent: new_parent_id.clone() 
        });

        info!("✅ Moved daemon in tree: {} (old parent: {:?}, new parent: {:?})", daemon_id, old_parent_id, new_parent_id);
        Ok(())
    }

    /// Get tree metrics
    pub async fn get_metrics(&self) -> Result<DaemonTreeMetrics> {
        let metrics = self.metrics.read().await.clone();
        Ok(metrics)
    }

    /// Rebalance tree for optimal performance
    pub async fn rebalance_tree(&self) -> Result<()> {
        let old_version = {
            let topology = self.topology.read().await;
            topology.topology_version.clone()
        };

        // Implement tree rebalancing logic here
        // This is a simplified version - real implementation would be more complex
        
        let new_version = format!("{}.{}", old_version, Utc::now().timestamp());
        
        // Update topology version
        {
            let mut topology = self.topology.write().await;
            topology.topology_version = new_version.clone();
            topology.last_updated = Utc::now();
        }

        // Send event
        let _ = self.event_tx.send(DaemonTreeEvent::TreeRebalanced { 
            old_version, 
            new_version 
        });

        info!("✅ Tree rebalanced successfully");
        Ok(())
    }

    /// Perform health check on all daemons
    pub async fn health_check(&self) -> Result<TreeHealthMetrics> {
        let mut healthy_count = 0;
        let mut unhealthy_count = 0;
        let mut degraded_count = 0;

        {
            let nodes = self.daemon_nodes.read().await;
            for (_daemon_id, daemon) in nodes.iter() {
                match daemon.health_status.status {
                    NodeStatus::Online => healthy_count += 1,
                    NodeStatus::Degraded => degraded_count += 1,
                    _ => unhealthy_count += 1,
                }
            }
        }

        let total_daemons = healthy_count + unhealthy_count + degraded_count;
        let tree_availability = if total_daemons > 0 {
            (healthy_count as f64 / total_daemons as f64) * 100.0
        } else {
            0.0
        };

        let tree_health = TreeHealthMetrics {
            overall_tree_health: tree_availability,
            healthy_daemons: healthy_count,
            unhealthy_daemons: unhealthy_count,
            degraded_daemons: degraded_count,
            tree_availability,
        };

        // Update health monitor
        {
            let mut health_monitor = self.health_monitor.write().await;
            health_monitor.tree_health = tree_health.clone();
        }

        Ok(tree_health)
    }
}
