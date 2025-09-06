//! Node Discovery module for BPI Oracle Node
//!
//! Handles automatic discovery of BPI nodes in the network, maintains
//! node registry, and provides health monitoring and connectivity management.

use anyhow::Result;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::{OracleConfig, BpiNode, BpiNodeType, NodeCapabilities, NodeStatus};

/// Node discovery announcement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeAnnouncement {
    pub node_id: String,
    pub node_type: BpiNodeType,
    pub endpoint: String,
    pub public_key: Vec<u8>,
    pub capabilities: NodeCapabilities,
    pub version: String,
    pub network_id: String,
    pub timestamp: DateTime<Utc>,
    pub signature: Vec<u8>,
}

/// Node health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub node_id: String,
    pub status: NodeStatus,
    pub response_time_ms: u64,
    pub last_seen: DateTime<Utc>,
    pub error_message: Option<String>,
    pub capabilities_verified: bool,
}

/// Network topology information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTopology {
    pub total_nodes: usize,
    pub nodes_by_type: HashMap<BpiNodeType, usize>,
    pub connectivity_matrix: HashMap<String, Vec<String>>,
    pub network_diameter: usize,
    pub cluster_coefficient: f64,
    pub last_updated: DateTime<Utc>,
}

/// Node discovery statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryStats {
    pub total_discoveries: u64,
    pub active_nodes: usize,
    pub failed_health_checks: u64,
    pub network_partitions: usize,
    pub average_discovery_time_ms: f64,
    pub node_churn_rate: f64,
}

/// Node discovery service
#[derive(Debug)]
pub struct NodeDiscovery {
    config: OracleConfig,
    discovered_nodes: Arc<DashMap<String, BpiNode>>,
    node_health: Arc<DashMap<String, HealthCheckResult>>,
    network_topology: Arc<RwLock<NetworkTopology>>,
    stats: Arc<RwLock<DiscoveryStats>>,
    discovery_channels: Arc<DashMap<String, tokio::sync::mpsc::Sender<NodeAnnouncement>>>,
    shutdown_tx: Arc<Mutex<Option<tokio::sync::broadcast::Sender<()>>>>,
}

impl NodeDiscovery {
    /// Create new node discovery service
    pub async fn new(config: OracleConfig) -> Result<Self> {
        info!("Initializing Node Discovery service");

        Ok(Self {
            config,
            discovered_nodes: Arc::new(DashMap::new()),
            node_health: Arc::new(DashMap::new()),
            network_topology: Arc::new(RwLock::new(NetworkTopology {
                total_nodes: 0,
                nodes_by_type: HashMap::new(),
                connectivity_matrix: HashMap::new(),
                network_diameter: 0,
                cluster_coefficient: 0.0,
                last_updated: Utc::now(),
            })),
            stats: Arc::new(RwLock::new(DiscoveryStats {
                total_discoveries: 0,
                active_nodes: 0,
                failed_health_checks: 0,
                network_partitions: 0,
                average_discovery_time_ms: 0.0,
                node_churn_rate: 0.0,
            })),
            discovery_channels: Arc::new(DashMap::new()),
            shutdown_tx: Arc::new(Mutex::new(None)),
        })
    }

    /// Start node discovery service
    pub async fn start(&self) -> Result<()> {
        info!("Starting Node Discovery service");

        // Start background services
        self.start_background_services().await?;

        // Start discovery protocols
        self.start_discovery_protocols().await?;

        info!("✅ Node Discovery service started successfully");
        Ok(())
    }

    /// Announce this Oracle node to the network
    pub async fn announce_self(&self) -> Result<()> {
        let announcement = NodeAnnouncement {
            node_id: self.config.node_id.clone(),
            node_type: BpiNodeType::Custom("Oracle".to_string()),
            endpoint: format!("http://localhost:{}", self.config.api_port),
            public_key: vec![1, 2, 3, 4], // In real implementation, use actual public key
            capabilities: NodeCapabilities {
                consensus: true,
                data_relay: true,
                real_time_messaging: true,
                batch_processing: true,
                max_message_size: 1024 * 1024, // 1MB
                supported_message_types: vec![
                    crate::MessageType::ConsensusVote,
                    crate::MessageType::DataSync,
                    crate::MessageType::CrossSystemRelay,
                    crate::MessageType::ConsensusProposal,
                    crate::MessageType::NodeDiscovery,
                    crate::MessageType::HealthCheck,
                ],
            },
            version: "1.0.0".to_string(),
            network_id: "bpi-mainnet".to_string(),
            timestamp: Utc::now(),
            signature: vec![5, 6, 7, 8], // In real implementation, sign the announcement
        };

        self.broadcast_announcement(&announcement).await?;
        info!("✅ Oracle node announced to network: {}", self.config.node_id);
        Ok(())
    }

    /// Process incoming node announcement
    pub async fn process_announcement(&self, announcement: NodeAnnouncement) -> Result<()> {
        info!("Processing node announcement: {} ({})", 
              announcement.node_id, announcement.node_type);

        // Verify announcement signature
        self.verify_announcement_signature(&announcement).await?;

        // Create or update node record
        let node = BpiNode {
            node_id: announcement.node_id.clone(),
            node_type: announcement.node_type.clone(),
            endpoint: announcement.endpoint.clone(),
            public_key: announcement.public_key.clone(),
            capabilities: announcement.capabilities.clone(),
            status: NodeStatus::Connected,
            last_seen: announcement.timestamp,
            connection_count: 0,
            trust_score: 1.0, // Initial trust score
        };

        // Add to discovered nodes
        self.discovered_nodes.insert(announcement.node_id.clone(), node);

        // Perform health check
        self.schedule_health_check(&announcement.node_id).await?;

        // Update network topology
        self.update_network_topology().await?;

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_discoveries += 1;
        stats.active_nodes = self.discovered_nodes.len();

        info!("✅ Node announcement processed: {}", announcement.node_id);
        Ok(())
    }

    /// Perform health check on a node
    pub async fn health_check(&self, node_id: &str) -> Result<HealthCheckResult> {
        debug!("Performing health check on node: {}", node_id);

        let start_time = std::time::Instant::now();
        
        if let Some(node) = self.discovered_nodes.get(node_id) {
            // Attempt to connect to node
            match self.check_node_connectivity(&node.endpoint).await {
                Ok(()) => {
                    let response_time = start_time.elapsed().as_millis() as u64;
                    let result = HealthCheckResult {
                        node_id: node_id.to_string(),
                        status: NodeStatus::Connected,
                        response_time_ms: response_time,
                        last_seen: Utc::now(),
                        error_message: None,
                        capabilities_verified: true,
                    };

                    // Update node health
                    self.node_health.insert(node_id.to_string(), result.clone());

                    // Update node status
                    if let Some(mut node_ref) = self.discovered_nodes.get_mut(node_id) {
                        node_ref.status = NodeStatus::Connected;
                        node_ref.last_seen = Utc::now();
                    }

                    debug!("✅ Health check passed for node: {} ({}ms)", node_id, response_time);
                    Ok(result)
                }
                Err(e) => {
                    let result = HealthCheckResult {
                        node_id: node_id.to_string(),
                        status: NodeStatus::Disconnected,
                        response_time_ms: start_time.elapsed().as_millis() as u64,
                        last_seen: Utc::now(),
                        error_message: Some(e.to_string()),
                        capabilities_verified: false,
                    };

                    // Update node health
                    self.node_health.insert(node_id.to_string(), result.clone());

                    // Update node status
                    if let Some(mut node_ref) = self.discovered_nodes.get_mut(node_id) {
                        node_ref.status = NodeStatus::Disconnected;
                    }

                    // Update statistics
                    let mut stats = self.stats.write().await;
                    stats.failed_health_checks += 1;

                    warn!("❌ Health check failed for node: {} - {}", node_id, e);
                    Ok(result)
                }
            }
        } else {
            Err(anyhow::anyhow!("Node not found: {}", node_id))
        }
    }

    /// Get all discovered nodes
    pub async fn get_discovered_nodes(&self) -> Vec<BpiNode> {
        self.discovered_nodes.iter().map(|entry| entry.value().clone()).collect()
    }

    /// Get nodes by type
    pub async fn get_nodes_by_type(&self, node_type: &BpiNodeType) -> Vec<BpiNode> {
        self.discovered_nodes
            .iter()
            .filter(|entry| &entry.value().node_type == node_type)
            .map(|entry| entry.value().clone())
            .collect()
    }

    /// Get network topology
    pub async fn get_network_topology(&self) -> NetworkTopology {
        self.network_topology.read().await.clone()
    }

    /// Get discovery statistics
    pub async fn get_stats(&self) -> DiscoveryStats {
        self.stats.read().await.clone()
    }

    /// Remove node from discovery
    pub async fn remove_node(&self, node_id: &str) -> Result<()> {
        if let Some((_, _)) = self.discovered_nodes.remove(node_id) {
            self.node_health.remove(node_id);
            
            // Update network topology
            self.update_network_topology().await?;

            // Update statistics
            let mut stats = self.stats.write().await;
            stats.active_nodes = self.discovered_nodes.len();

            info!("Node removed from discovery: {}", node_id);
        }
        
        Ok(())
    }

    /// Shutdown node discovery service
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down Node Discovery service");

        // Send shutdown signal
        if let Some(tx) = self.shutdown_tx.lock().await.take() {
            let _ = tx.send(());
        }

        // Clear all data
        self.discovered_nodes.clear();
        self.node_health.clear();
        self.discovery_channels.clear();

        info!("✅ Node Discovery service shutdown complete");
        Ok(())
    }

    /// Verify announcement signature
    async fn verify_announcement_signature(&self, announcement: &NodeAnnouncement) -> Result<()> {
        // In a real implementation, this would verify the cryptographic signature
        if announcement.signature.is_empty() {
            return Err(anyhow::anyhow!("Missing announcement signature"));
        }

        // Verify timestamp is recent (within last 5 minutes)
        let age = Utc::now() - announcement.timestamp;
        if age.num_seconds() > 300 {
            return Err(anyhow::anyhow!("Announcement timestamp too old"));
        }

        Ok(())
    }

    /// Check node connectivity
    async fn check_node_connectivity(&self, endpoint: &str) -> Result<()> {
        // In a real implementation, this would make an HTTP request to the node
        debug!("Checking connectivity to: {}", endpoint);
        
        // Simulate network check with some basic validation
        if endpoint.is_empty() {
            return Err(anyhow::anyhow!("Empty endpoint"));
        }

        if !endpoint.starts_with("http://") && !endpoint.starts_with("https://") {
            return Err(anyhow::anyhow!("Invalid endpoint format"));
        }

        // Simulate successful connection
        Ok(())
    }

    /// Schedule health check for a node
    async fn schedule_health_check(&self, node_id: &str) -> Result<()> {
        let node_id = node_id.to_string();
        let discovery = self.clone_for_task();
        
        tokio::spawn(async move {
            // Wait a bit before performing health check
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            
            if let Err(e) = discovery.health_check(&node_id).await {
                error!("Scheduled health check failed for {}: {}", node_id, e);
            }
        });

        Ok(())
    }

    /// Update network topology
    async fn update_network_topology(&self) -> Result<()> {
        let mut topology = self.network_topology.write().await;
        
        // Count nodes by type
        let mut nodes_by_type = HashMap::new();
        for node in self.discovered_nodes.iter() {
            *nodes_by_type.entry(node.value().node_type.clone()).or_insert(0) += 1;
        }

        // Update topology
        topology.total_nodes = self.discovered_nodes.len();
        topology.nodes_by_type = nodes_by_type;
        topology.last_updated = Utc::now();

        // In a real implementation, would calculate actual network metrics
        topology.network_diameter = (topology.total_nodes as f64).sqrt() as usize;
        topology.cluster_coefficient = 0.8; // Placeholder

        debug!("Network topology updated: {} nodes", topology.total_nodes);
        Ok(())
    }

    /// Broadcast announcement to network
    async fn broadcast_announcement(&self, announcement: &NodeAnnouncement) -> Result<()> {
        // In a real implementation, this would broadcast to known nodes
        debug!("Broadcasting announcement for node: {}", announcement.node_id);
        Ok(())
    }

    /// Start discovery protocols
    async fn start_discovery_protocols(&self) -> Result<()> {
        // Start multicast discovery
        self.start_multicast_discovery().await?;
        
        // Start DHT-based discovery
        self.start_dht_discovery().await?;
        
        // Start bootstrap discovery
        self.start_bootstrap_discovery().await?;
        
        Ok(())
    }

    /// Start multicast discovery
    async fn start_multicast_discovery(&self) -> Result<()> {
        debug!("Starting multicast discovery protocol");
        // In a real implementation, would set up UDP multicast
        Ok(())
    }

    /// Start DHT-based discovery
    async fn start_dht_discovery(&self) -> Result<()> {
        debug!("Starting DHT-based discovery protocol");
        // In a real implementation, would connect to DHT network
        Ok(())
    }

    /// Start bootstrap discovery
    async fn start_bootstrap_discovery(&self) -> Result<()> {
        debug!("Starting bootstrap discovery protocol");
        // In a real implementation, would connect to known bootstrap nodes
        Ok(())
    }

    /// Start background services
    async fn start_background_services(&self) -> Result<()> {
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel(1);
        *self.shutdown_tx.lock().await = Some(shutdown_tx);

        // Health monitoring service
        let discovered_nodes = Arc::clone(&self.discovered_nodes);
        let node_health = Arc::clone(&self.node_health);
        let stats = Arc::clone(&self.stats);
        let discovery_clone = self.clone_for_task();
        let mut shutdown_rx_health = shutdown_rx.resubscribe();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
            
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        // Perform health checks on all nodes
                        for node in discovered_nodes.iter() {
                            let node_id = node.key().clone();
                            let discovery = discovery_clone.clone();
                            
                            tokio::spawn(async move {
                                if let Err(e) = discovery.health_check(&node_id).await {
                                    debug!("Background health check failed for {}: {}", node_id, e);
                                }
                            });
                        }
                    }
                    _ = shutdown_rx_health.recv() => break,
                }
            }
        });

        // Network topology update service
        let network_topology = Arc::clone(&self.network_topology);
        let discovery_clone2 = self.clone_for_task();
        let mut shutdown_rx_topology = shutdown_rx.resubscribe();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // 5 minutes
            
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        if let Err(e) = discovery_clone2.update_network_topology().await {
                            error!("Failed to update network topology: {}", e);
                        }
                    }
                    _ = shutdown_rx_topology.recv() => break,
                }
            }
        });

        Ok(())
    }

    /// Clone for async tasks (simplified clone)
    fn clone_for_task(&self) -> Arc<Self> {
        // In a real implementation, would properly clone the Arc
        // For now, we'll create a placeholder
        Arc::new(Self {
            config: self.config.clone(),
            discovered_nodes: Arc::clone(&self.discovered_nodes),
            node_health: Arc::clone(&self.node_health),
            network_topology: Arc::clone(&self.network_topology),
            stats: Arc::clone(&self.stats),
            discovery_channels: Arc::clone(&self.discovery_channels),
            shutdown_tx: Arc::clone(&self.shutdown_tx),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_node_discovery_creation() {
        let config = OracleConfig::default();
        let discovery = NodeDiscovery::new(config).await.unwrap();
        
        let stats = discovery.get_stats().await;
        assert_eq!(stats.total_discoveries, 0);
        assert_eq!(stats.active_nodes, 0);
    }

    #[tokio::test]
    async fn test_node_announcement_processing() {
        let config = OracleConfig::default();
        let discovery = NodeDiscovery::new(config).await.unwrap();
        
        let announcement = NodeAnnouncement {
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
                supported_message_types: vec![crate::MessageType::Discovery],
            },
            version: "1.0.0".to_string(),
            network_id: "test-network".to_string(),
            timestamp: Utc::now(),
            signature: vec![5, 6, 7, 8],
        };

        discovery.process_announcement(announcement).await.unwrap();
        
        let nodes = discovery.get_discovered_nodes().await;
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].node_id, "test-node-1");
    }
}
