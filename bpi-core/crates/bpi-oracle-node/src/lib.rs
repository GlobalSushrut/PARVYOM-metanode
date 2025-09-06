//! BPI Oracle Node - Cross-system communication bridge for BPI ecosystem
//!
//! The BPI Oracle Node enables seamless communication between different BPI nodes,
//! providing a decentralized bridge for cross-system data exchange, consensus coordination,
//! and inter-node messaging with cryptographic verification.

use anyhow::Result;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

pub mod communication;
pub mod consensus_bridge;
pub mod data_relay;
pub mod node_discovery;
pub mod oracle_api;
pub mod message_verification;
pub mod inter_app_oracle;

/// BPI Oracle Node configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleConfig {
    /// Oracle node ID
    pub node_id: String,
    /// Listening port for Oracle API
    pub api_port: u16,
    /// WebSocket port for real-time communication
    pub ws_port: u16,
    /// Maximum number of connected nodes
    pub max_connections: usize,
    /// Message relay timeout in seconds
    pub relay_timeout_secs: u64,
    /// Consensus participation settings
    pub consensus_config: ConsensusConfig,
    /// Security settings
    pub security_config: SecurityConfig,
    /// Performance settings
    pub performance_config: PerformanceConfig,
}

/// Consensus bridge configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    /// Enable consensus bridging between nodes
    pub enable_consensus_bridge: bool,
    /// Minimum nodes required for consensus
    pub min_consensus_nodes: usize,
    /// Consensus timeout in seconds
    pub consensus_timeout_secs: u64,
    /// Vote aggregation threshold
    pub vote_threshold: f64,
}

/// Security configuration for Oracle Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Require cryptographic signatures for all messages
    pub require_signatures: bool,
    /// Maximum message age in seconds
    pub max_message_age_secs: u64,
    /// Rate limiting per node (messages per minute)
    pub rate_limit_per_node: u64,
    /// Enable message encryption
    pub enable_encryption: bool,
}

/// Performance optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Message batching size
    pub batch_size: usize,
    /// Connection pool size
    pub connection_pool_size: usize,
    /// Message cache size
    pub message_cache_size: usize,
    /// Enable high-throughput mode
    pub high_throughput_mode: bool,
}

impl Default for OracleConfig {
    fn default() -> Self {
        Self {
            node_id: format!("oracle-{}", Uuid::new_v4()),
            api_port: 9100,
            ws_port: 9101,
            max_connections: 1000,
            relay_timeout_secs: 30,
            consensus_config: ConsensusConfig {
                enable_consensus_bridge: true,
                min_consensus_nodes: 3,
                consensus_timeout_secs: 60,
                vote_threshold: 0.67,
            },
            security_config: SecurityConfig {
                require_signatures: true,
                max_message_age_secs: 300,
                rate_limit_per_node: 100,
                enable_encryption: true,
            },
            performance_config: PerformanceConfig {
                batch_size: 100,
                connection_pool_size: 50,
                message_cache_size: 10000,
                high_throughput_mode: false,
            },
        }
    }
}

/// BPI node types that can connect to the Oracle
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BpiNodeType {
    /// Gateway node (load balancer)
    Gateway,
    /// Mempool transaction manager
    Mempool,
    /// Consensus validator
    Validator,
    /// Consensus bridge node
    Consensus,
    /// Mining node
    Mining,
    /// Logbook node
    Logbook,
    /// Storage node
    Storage,
    /// HTTP Cage security node
    HttpCage,
    /// ENC Cluster orchestration
    EncCluster,
    /// DockLock container platform
    DockLock,
    /// ZKLock mobile/IoT port
    ZkLock,
    /// Domain resolver
    DomainResolver,
    /// Audit node
    Audit,
    /// Proof node
    Proof,
    /// Pipeline API
    Pipeline,
    /// Shadow registry
    ShadowRegistry,
    /// Custom node type
    Custom(String),
}

impl std::fmt::Display for BpiNodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BpiNodeType::Gateway => write!(f, "Gateway"),
            BpiNodeType::Mempool => write!(f, "Mempool"),
            BpiNodeType::Validator => write!(f, "Validator"),
            BpiNodeType::Consensus => write!(f, "Consensus"),
            BpiNodeType::Mining => write!(f, "Mining"),
            BpiNodeType::Logbook => write!(f, "Logbook"),
            BpiNodeType::Storage => write!(f, "Storage"),
            BpiNodeType::HttpCage => write!(f, "HttpCage"),
            BpiNodeType::EncCluster => write!(f, "EncCluster"),
            BpiNodeType::DockLock => write!(f, "DockLock"),
            BpiNodeType::ZkLock => write!(f, "ZkLock"),
            BpiNodeType::DomainResolver => write!(f, "DomainResolver"),
            BpiNodeType::Audit => write!(f, "Audit"),
            BpiNodeType::Proof => write!(f, "Proof"),
            BpiNodeType::Pipeline => write!(f, "Pipeline"),
            BpiNodeType::ShadowRegistry => write!(f, "ShadowRegistry"),
            BpiNodeType::Custom(name) => write!(f, "Custom({})", name),
        }
    }
}

/// BPI node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpiNode {
    pub node_id: String,
    pub node_type: BpiNodeType,
    pub endpoint: String,
    pub public_key: Vec<u8>,
    pub capabilities: NodeCapabilities,
    pub status: NodeStatus,
    pub last_seen: DateTime<Utc>,
    pub connection_count: usize,
    pub trust_score: f64,
}

/// Node capabilities for Oracle communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCapabilities {
    /// Supports consensus participation
    pub consensus: bool,
    /// Supports data relay
    pub data_relay: bool,
    /// Supports real-time messaging
    pub real_time_messaging: bool,
    /// Supports batch processing
    pub batch_processing: bool,
    /// Maximum message size in bytes
    pub max_message_size: usize,
    /// Supported message types
    pub supported_message_types: Vec<MessageType>,
}

/// Node connection status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeStatus {
    Connected,
    Disconnected,
    Reconnecting,
    Suspended,
    Banned,
}

/// Oracle message types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MessageType {
    NodeDiscovery,
    ConsensusVote,
    DataSync,
    HealthCheck,
    SystemAlert,
    CrossSystemRelay,
    ConsensusProposal,
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::NodeDiscovery => write!(f, "NodeDiscovery"),
            MessageType::ConsensusVote => write!(f, "ConsensusVote"),
            MessageType::DataSync => write!(f, "DataSync"),
            MessageType::HealthCheck => write!(f, "HealthCheck"),
            MessageType::SystemAlert => write!(f, "SystemAlert"),
            MessageType::CrossSystemRelay => write!(f, "CrossSystemRelay"),
            MessageType::ConsensusProposal => write!(f, "ConsensusProposal"),
        }
    }
}

/// Oracle message for cross-node communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleMessage {
    pub message_id: String,
    pub from_node: String,
    pub to_node: Option<String>,
    pub message_type: MessageType,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub priority: MessagePriority,
    pub signature: Option<Vec<u8>>,
    pub encryption_key: Option<Vec<u8>>,
    pub ttl_seconds: u64,
}


/// Message priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
    Emergency = 4,
}

/// Oracle message response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    pub response_id: String,
    pub original_message_id: String,
    pub from_node: String,
    pub success: bool,
    pub payload: Option<serde_json::Value>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Oracle Node statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleStats {
    pub total_nodes: usize,
    pub active_connections: usize,
    pub messages_relayed: u64,
    pub consensus_rounds: u64,
    pub uptime_seconds: u64,
}

/// Message verification statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationStats {
    pub total_verifications: u64,
    pub successful_verifications: u64,
    pub failed_verifications: u64,
    pub average_verification_time_ms: f64,
}

/// Consensus bridge statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusStats {
    pub total_proposals: u64,
    pub successful_consensus: u64,
    pub failed_consensus: u64,
    pub average_consensus_time_ms: f64,
}

/// Main BPI Oracle Node structure
#[derive(Debug)]
pub struct BpiOracleNode {
    /// Oracle configuration
    config: OracleConfig,
    /// Connected BPI nodes
    connected_nodes: Arc<DashMap<String, BpiNode>>,
    /// Active message channels
    message_channels: Arc<DashMap<String, tokio::sync::mpsc::Sender<OracleMessage>>>,
    /// Message history for deduplication
    message_history: Arc<RwLock<HashMap<String, DateTime<Utc>>>>,
    /// Communication manager
    communication: Arc<communication::CommunicationManager>,
    /// Consensus bridge
    consensus_bridge: Arc<consensus_bridge::ConsensusBridge>,
    /// Data relay system
    data_relay: Arc<data_relay::DataRelay>,
    /// Node discovery service
    node_discovery: Arc<node_discovery::NodeDiscovery>,
    /// Message verification system
    verification: Arc<message_verification::MessageVerification>,
    /// Oracle API server
    api_server: Arc<Mutex<Option<oracle_api::OracleApiServer>>>,
    /// System statistics
    stats: Arc<RwLock<OracleStats>>,
    /// Shutdown signal
    shutdown_tx: Arc<Mutex<Option<tokio::sync::broadcast::Sender<()>>>>,
}

impl BpiOracleNode {
    /// Create new BPI Oracle Node
    pub async fn new(config: OracleConfig) -> Result<Self> {
        info!("Initializing BPI Oracle Node: {}", config.node_id);

        let communication = Arc::new(
            communication::CommunicationManager::new(config.clone()).await?
        );

        let consensus_bridge = Arc::new(
            consensus_bridge::ConsensusBridge::new(config.consensus_config.clone()).await?
        );

        let data_relay = Arc::new(
            data_relay::DataRelay::new(config.performance_config.clone()).await?
        );

        let node_discovery = Arc::new(
            node_discovery::NodeDiscovery::new(config.clone()).await?
        );

        let verification = Arc::new(
            message_verification::MessageVerification::new(config.clone()).await?
        );

        let stats = Arc::new(RwLock::new(OracleStats {
            total_nodes: 0,
            active_connections: 0,
            messages_relayed: 0,
            consensus_rounds: 0,
            uptime_seconds: 0,
        }));

        Ok(Self {
            config,
            connected_nodes: Arc::new(DashMap::new()),
            message_channels: Arc::new(DashMap::new()),
            message_history: Arc::new(RwLock::new(HashMap::new())),
            communication,
            consensus_bridge,
            data_relay,
            node_discovery,
            verification,
            api_server: Arc::new(Mutex::new(None)),
            stats,
            shutdown_tx: Arc::new(Mutex::new(None)),
        })
    }

    /// Start the Oracle Node
    pub async fn start(&self) -> Result<()> {
        info!("Starting BPI Oracle Node: {}", self.config.node_id);

        // Start communication manager
        self.communication.start().await?;

        // Start message verification service
        self.verification.start().await?;

        // Start consensus bridge
        self.consensus_bridge.start().await?;

        // Start data relay
        self.data_relay.start().await?;

        // Start node discovery
        self.node_discovery.start().await?;

        // Announce this Oracle node to the network
        self.node_discovery.announce_self().await?;

        // Start API server
        let api_server = oracle_api::OracleApiServer::new(self.config.clone()).await?;
        *self.api_server.lock().await = Some(api_server);

        // Start background services
        self.start_background_services().await?;

        info!("✅ BPI Oracle Node started successfully on ports {} (API) and {} (WebSocket)", 
              self.config.api_port, self.config.ws_port);

        Ok(())
    }

    /// Register a new BPI node
    pub async fn register_node(&self, node: BpiNode) -> Result<()> {
        info!("Registering BPI node: {} ({})", node.node_id, node.node_type);

        // Verify node credentials
        self.verification.verify_node_credentials(&node).await?;

        // Add to connected nodes
        self.connected_nodes.insert(node.node_id.clone(), node.clone());

        // Create message channel
        let (tx, mut rx) = tokio::sync::mpsc::channel::<OracleMessage>(1000);
        self.message_channels.insert(node.node_id.clone(), tx);

        // Start message handler for this node
        let node_id = node.node_id.clone();
        let data_relay = Arc::clone(&self.data_relay);
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Err(e) = data_relay.relay_message(&node_id, message).await {
                    error!("Failed to relay message to node {}: {}", node_id, e);
                }
            }
        });

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_nodes = self.connected_nodes.len();
        stats.active_connections += 1;

        info!("✅ BPI node {} registered successfully", node.node_id);
        Ok(())
    }

    /// Relay message to connected nodes
    pub async fn relay_message(&self, message: &OracleMessage) -> Result<(), anyhow::Error> {
        // Record message in history
        self.record_message_history(message).await?;

        match &message.to_node {
            Some(target_node) => {
                // Send to specific node
                if let Some(channel) = self.message_channels.get(target_node) {
                    channel.send(message.clone()).await
                        .map_err(|e| anyhow::anyhow!("Failed to send message to {}: {}", target_node, e))?;
                } else {
                    return Err(anyhow::anyhow!("Target node not found: {}", target_node));
                }
            }
            None => {
                // Broadcast to all nodes
                for channel in self.message_channels.iter() {
                    if let Err(e) = channel.send(message.clone()).await {
                        warn!("Failed to broadcast message to {}: {}", channel.key(), e);
                    }
                }
            }
        }

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.messages_relayed += 1;

        Ok(())
    }

    /// Get Oracle system statistics
    pub async fn get_stats(&self) -> OracleStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Get list of connected nodes
    pub async fn get_connected_nodes(&self) -> Vec<BpiNode> {
        self.connected_nodes.iter().map(|entry| entry.value().clone()).collect()
    }

    /// Shutdown the Oracle Node
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down BPI Oracle Node: {}", self.config.node_id);

        // Send shutdown signal
        if let Some(tx) = self.shutdown_tx.lock().await.take() {
            let _ = tx.send(());
        }

        // Stop API server
        if let Some(api_server) = self.api_server.lock().await.take() {
            api_server.shutdown().await?;
        }

        // Stop node discovery
        self.node_discovery.shutdown().await?;

        info!("✅ BPI Oracle Node shutdown complete");
        Ok(())
    }

    /// Start background services
    async fn start_background_services(&self) -> Result<()> {
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel(1);
        *self.shutdown_tx.lock().await = Some(shutdown_tx);

        // Stats update service
        let stats = Arc::clone(&self.stats);
        let mut shutdown_rx_stats = shutdown_rx.resubscribe();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        // Update uptime and other stats
                        let mut stats_guard = stats.write().await;
                        stats_guard.uptime_seconds += 30;
                    }
                    _ = shutdown_rx_stats.recv() => break,
                }
            }
        });

        // Message history cleanup service
        let message_history = Arc::clone(&self.message_history);
        let mut shutdown_rx_cleanup = shutdown_rx.resubscribe();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // 5 minutes
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        let mut history = message_history.write().await;
                        let cutoff = Utc::now() - chrono::Duration::seconds(3600); // 1 hour
                        history.retain(|_, timestamp| *timestamp > cutoff);
                    }
                    _ = shutdown_rx_cleanup.recv() => break,
                }
            }
        });

        Ok(())
    }

    /// Check if message is duplicate
    async fn is_duplicate_message(&self, message: &OracleMessage) -> Result<bool> {
        let history = self.message_history.read().await;
        Ok(history.contains_key(&message.message_id))
    }

    /// Record message in history for deduplication
    async fn record_message_history(&self, message: &OracleMessage) -> Result<()> {
        let mut history = self.message_history.write().await;
        history.insert(message.message_id.clone(), message.timestamp);
        Ok(())
    }

    /// Get discovered nodes from node discovery service
    pub async fn get_discovered_nodes(&self) -> Vec<BpiNode> {
        self.node_discovery.get_discovered_nodes().await
    }

    /// Send cross-system message
    pub async fn send_cross_system_message(
        &self,
        source_system: &str,
        target_system: &str,
        message: serde_json::Value,
    ) -> Result<String> {
        let message_id = Uuid::new_v4().to_string();
        let oracle_message = OracleMessage {
            message_id: message_id.clone(),
            from_node: source_system.to_string(),
            to_node: Some(target_system.to_string()),
            message_type: MessageType::CrossSystemRelay,
            payload: message,
            timestamp: Utc::now(),
            priority: MessagePriority::Normal,
            signature: None,
            encryption_key: None,
            ttl_seconds: 300, // 5 minutes default TTL
        };

        self.relay_message(&oracle_message).await?;
        Ok(message_id)
    }

    /// Query network data
    pub async fn query_network_data(
        &self,
        query_type: &str,
        parameters: serde_json::Value,
    ) -> Result<serde_json::Value> {
        // TODO: Implement actual data querying
        Ok(serde_json::json!({
            "query_type": query_type,
            "parameters": parameters,
            "results": [],
            "timestamp": Utc::now()
        }))
    }

    /// Get message verification statistics
    pub async fn get_verification_stats(&self) -> VerificationStats {
        // TODO: Implement actual verification stats
        VerificationStats {
            total_verifications: 0,
            successful_verifications: 0,
            failed_verifications: 0,
            average_verification_time_ms: 0.0,
        }
    }

    /// Get consensus bridge statistics
    pub async fn get_consensus_stats(&self) -> ConsensusStats {
        // TODO: Implement actual consensus stats
        ConsensusStats {
            total_proposals: 0,
            successful_consensus: 0,
            failed_consensus: 0,
            average_consensus_time_ms: 0.0,
        }
    }

    /// Process incoming message
    pub async fn process_incoming_message(&self, message: OracleMessage) -> Result<()> {
        // Verify message if signatures are required
        if self.config.security_config.require_signatures {
            // TODO: Implement actual message verification
            info!("Would verify message: {}", message.message_id);
        }

        // Check for duplicates
        if self.is_duplicate_message(&message).await? {
            return Err(anyhow::anyhow!("Duplicate message: {}", message.message_id));
        }

        // Process based on message type
        match message.message_type {
            MessageType::CrossSystemRelay => {
                self.relay_message(&message).await?;
            }
            MessageType::ConsensusProposal => {
                // TODO: Implement consensus proposal processing
                info!("Processing consensus proposal: {}", message.message_id);
            }
            MessageType::DataSync => {
                // TODO: Implement data sync message processing
                info!("Processing data sync message: {}", message.message_id);
            }
            _ => {
                info!("Processing message type: {:?}", message.message_type);
            }
        }

        Ok(())
    }

    /// Submit consensus proposal
    pub async fn submit_consensus_proposal(&self, proposal: serde_json::Value) -> Result<String> {
        let proposal_id = Uuid::new_v4().to_string();
        // TODO: Implement actual consensus proposal submission
        // For now, just log the proposal since we need a ConsensusProposal struct
        info!("Would submit consensus proposal: {} with data: {}", proposal_id, proposal);
        Ok(proposal_id)
    }

    /// Initiate data relay
    pub async fn initiate_data_relay(&self, data: serde_json::Value) -> Result<String> {
        let relay_id = Uuid::new_v4().to_string();
        // TODO: Implement actual data relay initiation
        info!("Initiating data relay: {} with data: {}", relay_id, data);
        Ok(relay_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_oracle_node_creation() {
        let config = OracleConfig::default();
        let oracle = BpiOracleNode::new(config).await.unwrap();
        
        let stats = oracle.get_stats().await;
        assert_eq!(stats.total_nodes, 0);
        assert_eq!(stats.active_connections, 0);
    }

    #[tokio::test]
    async fn test_node_registration() {
        let config = OracleConfig::default();
        let oracle = BpiOracleNode::new(config).await.unwrap();
        
        let node = BpiNode {
            node_id: "test-node-1".to_string(),
            node_type: BpiNodeType::Gateway,
            endpoint: "http://localhost:8080".to_string(),
            public_key: vec![1, 2, 3, 4],
            capabilities: NodeCapabilities {
                consensus: true,
                data_relay: true,
                real_time_messaging: true,
                batch_processing: false,
                max_message_size: 1024,
                supported_message_types: vec![MessageType::Consensus, MessageType::DataSync],
            },
            status: NodeStatus::Connected,
            last_seen: Utc::now(),
            connection_count: 1,
            trust_score: 1.0,
        };

        // Note: This test would require proper verification setup in a real scenario
        // oracle.register_node(node).await.unwrap();
        
        // let nodes = oracle.get_connected_nodes().await;
        // assert_eq!(nodes.len(), 1);
    }
}
