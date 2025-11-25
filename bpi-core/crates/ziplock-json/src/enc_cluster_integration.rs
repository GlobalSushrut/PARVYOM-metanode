//! ENC Cluster Integration for ZipLock JSON
//! 
//! Provides enterprise cluster orchestration and consensus integration with ENC Cluster
//! Features: Cluster lifecycle management, consensus monitoring, node validation, distributed audit

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use uuid::Uuid;

/// ENC Cluster configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncClusterConfig {
    /// Cluster name
    pub cluster_name: String,
    /// Cluster endpoint
    pub cluster_endpoint: String,
    /// Cluster version
    pub cluster_version: String,
    /// Consensus algorithm
    pub consensus_algorithm: ConsensusAlgorithm,
    /// Node configuration
    pub node_config: NodeConfig,
    /// Validation configuration
    pub validation_config: ValidationConfig,
    /// Audit configuration
    pub audit_config: ClusterAuditConfig,
}

/// Consensus algorithms for BPI ENC Cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusAlgorithm {
    /// BPI native consensus mechanism
    BpiConsensus,
    /// ENC Cluster consensus for BPI nodes
    EncBpiConsensus,
}

/// Node configuration for ENC Cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Node ID
    pub node_id: String,
    /// Node role
    pub node_role: NodeRole,
    /// Maximum nodes in cluster
    pub max_nodes: u32,
    /// Minimum nodes for consensus
    pub min_consensus_nodes: u32,
    /// Node health check interval (seconds)
    pub health_check_interval: u32,
    /// Node timeout (seconds)
    pub node_timeout: u32,
}

/// Node roles in ENC Cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeRole {
    /// Leader node
    Leader,
    /// Follower node
    Follower,
    /// Candidate node
    Candidate,
    /// Observer node
    Observer,
    /// Validator node
    Validator,
}

/// Validation configuration for ENC Cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Enable transaction validation
    pub enable_tx_validation: bool,
    /// Enable state validation
    pub enable_state_validation: bool,
    /// Validation timeout (seconds)
    pub validation_timeout: u32,
    /// Maximum validation retries
    pub max_retries: u32,
    /// Validation batch size
    pub batch_size: u32,
}

/// Cluster audit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterAuditConfig {
    /// Enable consensus auditing
    pub enable_consensus_audit: bool,
    /// Enable node performance auditing
    pub enable_performance_audit: bool,
    /// Audit retention period (days)
    pub retention_days: u32,
    /// Real-time audit streaming
    pub enable_realtime_streaming: bool,
}

/// ENC Cluster operation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterOperation {
    /// Operation ID
    pub operation_id: String,
    /// Operation type
    pub operation_type: ClusterOperationType,
    /// Node ID that initiated operation
    pub initiator_node_id: String,
    /// Target nodes
    pub target_nodes: Vec<String>,
    /// Operation timestamp
    pub timestamp: DateTime<Utc>,
    /// Operation status
    pub status: ClusterOperationStatus,
    /// Consensus round (if applicable)
    pub consensus_round: Option<u64>,
    /// Operation payload
    pub payload: ClusterOperationPayload,
    /// Performance metrics
    pub performance_metrics: ClusterPerformanceMetrics,
}

/// Types of cluster operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClusterOperationType {
    /// Node join cluster
    NodeJoin,
    /// Node leave cluster
    NodeLeave,
    /// Leader election
    LeaderElection,
    /// Consensus proposal
    ConsensusProposal,
    /// Consensus vote
    ConsensusVote,
    /// State synchronization
    StateSync,
    /// Transaction validation
    TransactionValidation,
    /// Cluster reconfiguration
    ClusterReconfig,
    /// Health check
    HealthCheck,
}

/// Cluster operation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClusterOperationStatus {
    /// Operation initiated
    Initiated,
    /// Operation in progress
    InProgress,
    /// Waiting for consensus
    WaitingConsensus,
    /// Operation committed
    Committed,
    /// Operation failed
    Failed(String),
    /// Operation timeout
    Timeout,
    /// Operation rejected
    Rejected(String),
}

/// Cluster operation payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterOperationPayload {
    /// Operation data
    pub data: serde_json::Value,
    /// Data size in bytes
    pub data_size: u64,
    /// Checksum for integrity
    pub checksum: String,
    /// Encryption status
    pub encrypted: bool,
}

/// Cluster performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterPerformanceMetrics {
    /// Operation latency (milliseconds)
    pub latency_ms: u64,
    /// Throughput (operations per second)
    pub throughput_ops: f64,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Network I/O bytes
    pub network_io_bytes: u64,
    /// Consensus rounds required
    pub consensus_rounds: u32,
}

/// ENC Cluster consensus event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusEvent {
    /// Event ID
    pub event_id: String,
    /// Event type
    pub event_type: ConsensusEventType,
    /// Consensus round
    pub round: u64,
    /// Term (for Raft-like algorithms)
    pub term: u64,
    /// Proposer node ID
    pub proposer_node_id: String,
    /// Participating nodes
    pub participating_nodes: Vec<String>,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event data
    pub event_data: ConsensusEventData,
    /// Event outcome
    pub outcome: ConsensusOutcome,
}

/// Types of consensus events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusEventType {
    /// Proposal submitted
    Proposal,
    /// Vote cast
    Vote,
    /// Commit decision
    Commit,
    /// Abort decision
    Abort,
    /// View change
    ViewChange,
    /// Leader election
    LeaderElection,
}

/// Consensus event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusEventData {
    /// Proposal hash
    pub proposal_hash: String,
    /// Vote count
    pub vote_count: u32,
    /// Required votes for consensus
    pub required_votes: u32,
    /// Voting nodes
    pub voting_nodes: HashMap<String, bool>, // node_id -> vote (true/false)
    /// Event payload
    pub payload: serde_json::Value,
}

/// Consensus outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusOutcome {
    /// Consensus reached
    Consensus,
    /// No consensus reached
    NoConsensus,
    /// Consensus pending
    Pending,
    /// Consensus failed
    Failed(String),
}

/// ENC Cluster node status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStatus {
    /// Node ID
    pub node_id: String,
    /// Node role
    pub role: NodeRole,
    /// Node health status
    pub health_status: NodeHealthStatus,
    /// Last heartbeat
    pub last_heartbeat: DateTime<Utc>,
    /// Node performance metrics
    pub performance_metrics: NodePerformanceMetrics,
    /// Active connections
    pub active_connections: u32,
    /// Node version
    pub node_version: String,
}

/// Node health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeHealthStatus {
    /// Node is healthy
    Healthy,
    /// Node is degraded
    Degraded,
    /// Node is unhealthy
    Unhealthy,
    /// Node is unreachable
    Unreachable,
    /// Node status unknown
    Unknown,
}

/// Node performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePerformanceMetrics {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage percentage
    pub memory_usage: f64,
    /// Disk usage percentage
    pub disk_usage: f64,
    /// Network latency (milliseconds)
    pub network_latency_ms: u64,
    /// Operations processed per second
    pub ops_per_second: f64,
    /// Error rate percentage
    pub error_rate: f64,
}

/// ENC Cluster validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Validation ID
    pub validation_id: String,
    /// Validation type
    pub validation_type: ValidationType,
    /// Validator node ID
    pub validator_node_id: String,
    /// Validation timestamp
    pub timestamp: DateTime<Utc>,
    /// Validation status
    pub status: ValidationStatus,
    /// Validation details
    pub details: ValidationDetails,
    /// Validation metrics
    pub metrics: ValidationMetrics,
}

/// Types of validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationType {
    /// Transaction validation
    Transaction,
    /// State validation
    State,
    /// Block validation
    Block,
    /// Consensus validation
    Consensus,
    /// Node validation
    Node,
}

/// Validation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationStatus {
    /// Validation passed
    Valid,
    /// Validation failed
    Invalid(String),
    /// Validation in progress
    InProgress,
    /// Validation timeout
    Timeout,
    /// Validation error
    Error(String),
}

/// Validation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationDetails {
    /// Items validated
    pub items_validated: u32,
    /// Items passed
    pub items_passed: u32,
    /// Items failed
    pub items_failed: u32,
    /// Validation rules applied
    pub rules_applied: Vec<String>,
    /// Error messages
    pub error_messages: Vec<String>,
}

/// Validation performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationMetrics {
    /// Validation duration (milliseconds)
    pub duration_ms: u64,
    /// Throughput (validations per second)
    pub throughput: f64,
    /// Resource usage during validation
    pub resource_usage: NodePerformanceMetrics,
}

/// ENC Cluster integrator
pub struct EncClusterIntegrator {
    /// Configuration
    config: EncClusterConfig,
    /// Active cluster operations
    active_operations: Arc<RwLock<HashMap<String, ClusterOperation>>>,
    /// Consensus monitor
    consensus_monitor: ConsensusMonitor,
    /// Node manager
    node_manager: NodeManager,
    /// Validation engine
    validation_engine: ValidationEngine,
    /// Cluster audit manager
    audit_manager: ClusterAuditManager,
}

/// Consensus monitoring system
pub struct ConsensusMonitor {
    /// Configuration
    config: EncClusterConfig,
    /// Active consensus events
    active_events: Arc<RwLock<HashMap<u64, ConsensusEvent>>>,
    /// Consensus history
    consensus_history: Arc<RwLock<Vec<ConsensusEvent>>>,
}

/// Node management system
pub struct NodeManager {
    /// Configuration
    config: NodeConfig,
    /// Active nodes
    active_nodes: Arc<RwLock<HashMap<String, NodeStatus>>>,
    /// Node health monitor
    health_monitor: Arc<RwLock<HashMap<String, DateTime<Utc>>>>,
}

/// Validation engine for ENC Cluster
pub struct ValidationEngine {
    /// Configuration
    config: ValidationConfig,
    /// Active validations
    active_validations: Arc<RwLock<HashMap<String, ValidationResult>>>,
    /// Validation history
    validation_history: Arc<RwLock<Vec<ValidationResult>>>,
}

/// Cluster audit manager
pub struct ClusterAuditManager {
    /// Configuration
    config: ClusterAuditConfig,
    /// Audit events
    audit_events: Arc<RwLock<Vec<ClusterOperation>>>,
    /// Consensus audit events
    consensus_events: Arc<RwLock<Vec<ConsensusEvent>>>,
}

impl EncClusterIntegrator {
    /// Create new ENC Cluster integrator
    pub fn new(config: EncClusterConfig) -> Self {
        Self {
            consensus_monitor: ConsensusMonitor::new(config.clone()),
            node_manager: NodeManager::new(config.node_config.clone()),
            validation_engine: ValidationEngine::new(config.validation_config.clone()),
            audit_manager: ClusterAuditManager::new(config.audit_config.clone()),
            config,
            active_operations: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Record cluster operation
    pub async fn record_cluster_operation(&self, operation: ClusterOperation) -> Result<String> {
        let operation_id = operation.operation_id.clone();
        
        // Add to active operations
        {
            let mut ops = self.active_operations.write().await;
            ops.insert(operation_id.clone(), operation.clone());
        }

        // Record in audit manager
        self.audit_manager.record_operation(operation.clone()).await?;

        // Start consensus monitoring if needed
        if matches!(operation.operation_type, ClusterOperationType::ConsensusProposal) {
            self.consensus_monitor.start_monitoring_consensus(operation.consensus_round.unwrap_or(0)).await?;
        }

        Ok(operation_id)
    }

    /// Record consensus event
    pub async fn record_consensus_event(&self, event: ConsensusEvent) -> Result<()> {
        self.consensus_monitor.record_event(event.clone()).await?;
        self.audit_manager.record_consensus_event(event).await?;
        Ok(())
    }

    /// Update node status
    pub async fn update_node_status(&self, node_status: NodeStatus) -> Result<()> {
        self.node_manager.update_node_status(node_status).await
    }

    /// Record validation result
    pub async fn record_validation(&self, validation: ValidationResult) -> Result<()> {
        self.validation_engine.record_validation(validation).await
    }

    /// Get cluster health summary
    pub async fn get_cluster_health(&self) -> Result<ClusterHealthSummary> {
        let nodes = self.node_manager.get_all_nodes().await?;
        let active_ops = self.active_operations.read().await;
        let consensus_events = self.consensus_monitor.get_recent_events(10).await?;

        let healthy_nodes = nodes.iter()
            .filter(|node| matches!(node.health_status, NodeHealthStatus::Healthy))
            .count();

        let total_nodes = nodes.len();
        let active_operations = active_ops.len();
        let recent_consensus_events = consensus_events.len();

        Ok(ClusterHealthSummary {
            total_nodes: total_nodes as u32,
            healthy_nodes: healthy_nodes as u32,
            active_operations: active_operations as u32,
            recent_consensus_events: recent_consensus_events as u32,
            cluster_status: if healthy_nodes >= self.config.node_config.min_consensus_nodes as usize {
                ClusterStatus::Healthy
            } else {
                ClusterStatus::Degraded
            },
            last_updated: Utc::now(),
        })
    }

    /// Export cluster data for ZipLock JSON integration
    pub async fn export_for_ziplock(&self) -> Result<serde_json::Value> {
        let operations = self.audit_manager.get_all_operations().await?;
        let consensus_events = self.consensus_monitor.get_all_events().await?;
        let nodes = self.node_manager.get_all_nodes().await?;
        let validations = self.validation_engine.get_all_validations().await?;
        let health_summary = self.get_cluster_health().await?;

        Ok(serde_json::json!({
            "enc_cluster_integration": {
                "cluster_name": self.config.cluster_name,
                "cluster_version": self.config.cluster_version,
                "consensus_algorithm": self.config.consensus_algorithm,
                "cluster_operations": operations,
                "consensus_events": consensus_events,
                "node_statuses": nodes,
                "validation_results": validations,
                "cluster_health": health_summary,
                "export_timestamp": Utc::now(),
                "total_operations": operations.len(),
                "total_consensus_events": consensus_events.len(),
                "total_nodes": nodes.len(),
                "total_validations": validations.len()
            }
        }))
    }

    /// Get cluster performance metrics
    pub async fn get_performance_metrics(&self) -> Result<ClusterPerformanceReport> {
        let operations = self.audit_manager.get_recent_operations(100).await?;
        let nodes = self.node_manager.get_all_nodes().await?;

        let avg_latency = operations.iter()
            .map(|op| op.performance_metrics.latency_ms)
            .sum::<u64>() as f64 / operations.len().max(1) as f64;

        let total_throughput = operations.iter()
            .map(|op| op.performance_metrics.throughput_ops)
            .sum::<f64>();

        let avg_cpu_usage = nodes.iter()
            .map(|node| node.performance_metrics.cpu_usage)
            .sum::<f64>() / nodes.len().max(1) as f64;

        Ok(ClusterPerformanceReport {
            average_latency_ms: avg_latency,
            total_throughput_ops: total_throughput,
            average_cpu_usage: avg_cpu_usage,
            total_operations: operations.len() as u32,
            active_nodes: nodes.len() as u32,
            report_timestamp: Utc::now(),
        })
    }
}

/// Cluster health summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterHealthSummary {
    /// Total nodes in cluster
    pub total_nodes: u32,
    /// Healthy nodes count
    pub healthy_nodes: u32,
    /// Active operations count
    pub active_operations: u32,
    /// Recent consensus events count
    pub recent_consensus_events: u32,
    /// Overall cluster status
    pub cluster_status: ClusterStatus,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

/// Cluster status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClusterStatus {
    /// Cluster is healthy
    Healthy,
    /// Cluster is degraded
    Degraded,
    /// Cluster is unhealthy
    Unhealthy,
    /// Cluster is offline
    Offline,
}

/// Cluster performance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterPerformanceReport {
    /// Average operation latency
    pub average_latency_ms: f64,
    /// Total throughput
    pub total_throughput_ops: f64,
    /// Average CPU usage across nodes
    pub average_cpu_usage: f64,
    /// Total operations processed
    pub total_operations: u32,
    /// Active nodes count
    pub active_nodes: u32,
    /// Report timestamp
    pub report_timestamp: DateTime<Utc>,
}

// Implementation of sub-components
impl ConsensusMonitor {
    pub fn new(config: EncClusterConfig) -> Self {
        Self {
            config,
            active_events: Arc::new(RwLock::new(HashMap::new())),
            consensus_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn start_monitoring_consensus(&self, round: u64) -> Result<()> {
        // Start monitoring consensus for the given round
        Ok(())
    }

    pub async fn record_event(&self, event: ConsensusEvent) -> Result<()> {
        let mut active = self.active_events.write().await;
        let mut history = self.consensus_history.write().await;
        
        active.insert(event.round, event.clone());
        history.push(event);
        
        Ok(())
    }

    pub async fn get_recent_events(&self, count: usize) -> Result<Vec<ConsensusEvent>> {
        let history = self.consensus_history.read().await;
        Ok(history.iter().rev().take(count).cloned().collect())
    }

    pub async fn get_all_events(&self) -> Result<Vec<ConsensusEvent>> {
        let history = self.consensus_history.read().await;
        Ok(history.clone())
    }
}

impl NodeManager {
    pub fn new(config: NodeConfig) -> Self {
        Self {
            config,
            active_nodes: Arc::new(RwLock::new(HashMap::new())),
            health_monitor: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn update_node_status(&self, status: NodeStatus) -> Result<()> {
        let mut nodes = self.active_nodes.write().await;
        let mut health = self.health_monitor.write().await;
        
        nodes.insert(status.node_id.clone(), status.clone());
        health.insert(status.node_id, status.last_heartbeat);
        
        Ok(())
    }

    pub async fn get_all_nodes(&self) -> Result<Vec<NodeStatus>> {
        let nodes = self.active_nodes.read().await;
        Ok(nodes.values().cloned().collect())
    }
}

impl ValidationEngine {
    pub fn new(config: ValidationConfig) -> Self {
        Self {
            config,
            active_validations: Arc::new(RwLock::new(HashMap::new())),
            validation_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn record_validation(&self, validation: ValidationResult) -> Result<()> {
        let mut active = self.active_validations.write().await;
        let mut history = self.validation_history.write().await;
        
        active.insert(validation.validation_id.clone(), validation.clone());
        history.push(validation);
        
        Ok(())
    }

    pub async fn get_all_validations(&self) -> Result<Vec<ValidationResult>> {
        let history = self.validation_history.read().await;
        Ok(history.clone())
    }
}

impl ClusterAuditManager {
    pub fn new(config: ClusterAuditConfig) -> Self {
        Self {
            config,
            audit_events: Arc::new(RwLock::new(Vec::new())),
            consensus_events: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn record_operation(&self, operation: ClusterOperation) -> Result<()> {
        let mut events = self.audit_events.write().await;
        events.push(operation);
        Ok(())
    }

    pub async fn record_consensus_event(&self, event: ConsensusEvent) -> Result<()> {
        let mut events = self.consensus_events.write().await;
        events.push(event);
        Ok(())
    }

    pub async fn get_all_operations(&self) -> Result<Vec<ClusterOperation>> {
        let events = self.audit_events.read().await;
        Ok(events.clone())
    }

    pub async fn get_recent_operations(&self, count: usize) -> Result<Vec<ClusterOperation>> {
        let events = self.audit_events.read().await;
        Ok(events.iter().rev().take(count).cloned().collect())
    }
}

impl Default for EncClusterConfig {
    fn default() -> Self {
        Self {
            cluster_name: "enc-cluster-1".to_string(),
            cluster_endpoint: "http://localhost:9090".to_string(),
            cluster_version: "1.0.0".to_string(),
            consensus_algorithm: ConsensusAlgorithm::BpiConsensus,
            node_config: NodeConfig {
                node_id: Uuid::new_v4().to_string(),
                node_role: NodeRole::Follower,
                max_nodes: 10,
                min_consensus_nodes: 3,
                health_check_interval: 30,
                node_timeout: 60,
            },
            validation_config: ValidationConfig {
                enable_tx_validation: true,
                enable_state_validation: true,
                validation_timeout: 30,
                max_retries: 3,
                batch_size: 100,
            },
            audit_config: ClusterAuditConfig {
                enable_consensus_audit: true,
                enable_performance_audit: true,
                retention_days: 90,
                enable_realtime_streaming: true,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_enc_cluster_integration() {
        let config = EncClusterConfig::default();
        let integrator = EncClusterIntegrator::new(config);
        
        let operation = ClusterOperation {
            operation_id: "cluster_op_1".to_string(),
            operation_type: ClusterOperationType::NodeJoin,
            initiator_node_id: "node_1".to_string(),
            target_nodes: vec!["node_2".to_string(), "node_3".to_string()],
            timestamp: Utc::now(),
            status: ClusterOperationStatus::Initiated,
            consensus_round: Some(1),
            payload: ClusterOperationPayload {
                data: serde_json::json!({"action": "join_cluster"}),
                data_size: 100,
                checksum: "abc123".to_string(),
                encrypted: false,
            },
            performance_metrics: ClusterPerformanceMetrics {
                latency_ms: 50,
                throughput_ops: 100.0,
                cpu_usage_percent: 25.0,
                memory_usage_bytes: 1024 * 1024,
                network_io_bytes: 2048,
                consensus_rounds: 1,
            },
        };

        let op_id = integrator.record_cluster_operation(operation).await.unwrap();
        assert_eq!(op_id, "cluster_op_1");
        
        let health = integrator.get_cluster_health().await.unwrap();
        assert_eq!(health.active_operations, 1);
    }

    #[tokio::test]
    async fn test_consensus_monitoring() {
        let config = EncClusterConfig::default();
        let monitor = ConsensusMonitor::new(config);
        
        let event = ConsensusEvent {
            event_id: "consensus_1".to_string(),
            event_type: ConsensusEventType::Proposal,
            round: 1,
            term: 1,
            proposer_node_id: "node_1".to_string(),
            participating_nodes: vec!["node_1".to_string(), "node_2".to_string()],
            timestamp: Utc::now(),
            event_data: ConsensusEventData {
                proposal_hash: "hash123".to_string(),
                vote_count: 2,
                required_votes: 2,
                voting_nodes: HashMap::new(),
                payload: serde_json::json!({}),
            },
            outcome: ConsensusOutcome::Consensus,
        };

        monitor.record_event(event).await.unwrap();
        
        let recent_events = monitor.get_recent_events(5).await.unwrap();
        assert_eq!(recent_events.len(), 1);
    }

    #[tokio::test]
    async fn test_node_management() {
        let config = NodeConfig {
            node_id: "test_node".to_string(),
            node_role: NodeRole::Leader,
            max_nodes: 5,
            min_consensus_nodes: 3,
            health_check_interval: 30,
            node_timeout: 60,
        };
        
        let manager = NodeManager::new(config);
        
        let status = NodeStatus {
            node_id: "node_1".to_string(),
            role: NodeRole::Follower,
            health_status: NodeHealthStatus::Healthy,
            last_heartbeat: Utc::now(),
            performance_metrics: NodePerformanceMetrics {
                cpu_usage: 25.0,
                memory_usage: 50.0,
                disk_usage: 30.0,
                network_latency_ms: 10,
                ops_per_second: 100.0,
                error_rate: 0.1,
            },
            active_connections: 5,
            node_version: "1.0.0".to_string(),
        };

        manager.update_node_status(status).await.unwrap();
        
        let nodes = manager.get_all_nodes().await.unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].node_id, "node_1");
    }
}
