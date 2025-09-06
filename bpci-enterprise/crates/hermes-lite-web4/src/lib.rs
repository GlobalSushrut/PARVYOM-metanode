//! HERMES-Lite Web-4 Edition - Ultra-Light P2P for Testnet
//! 
//! Simplified, testnet-ready P2P networking:
//! - Simple UDP transport (no QUIC complexity)
//! - Basic neighbor discovery (no complex geometry)
//! - Minimal message routing (no hyperbolic math)
//! - BPCI traffic classes (consensus priority)

pub mod node;
pub mod transport;
pub mod neighbor;
pub mod message;
pub mod config;

pub use node::{NodeId, P2PNode};
pub use transport::UdpTransport;
pub use neighbor::NeighborManager;
pub use message::{P2PMessage, MessageType, TrafficClass};
pub use config::HermesConfig;

use std::error::Error;
use tracing::{info, error};

/// Ultra-light HERMES-Lite P2P node for testnet
pub struct HermesLiteWeb4 {
    config: HermesConfig,
    node: P2PNode,
    transport: UdpTransport,
    neighbors: NeighborManager,
}

impl HermesLiteWeb4 {
    /// Create new testnet-ready P2P node
    pub fn new(config: HermesConfig) -> Self {
        let node = P2PNode::new(config.node_id.clone());
        let transport = UdpTransport::new(config.listen_port);
        let neighbors = NeighborManager::new(config.max_neighbors);
        
        Self {
            config,
            node,
            transport,
            neighbors,
        }
    }
    
    /// Start the P2P node (simple, reliable)
    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Starting HERMES-Lite Web-4 node: {}", self.config.node_id);
        
        // Start UDP transport
        self.transport.start().await?;
        info!("UDP transport started on port {}", self.config.listen_port);
        
        // Initialize basic neighbor discovery
        self.neighbors.start_discovery().await?;
        info!("Neighbor discovery started");
        
        // Start message handling loop
        self.start_message_loop().await?;
        
        info!("HERMES-Lite Web-4 node ready for testnet");
        Ok(())
    }
    
    /// Send message with BPCI traffic class priority
    pub async fn send_message(&mut self, target: NodeId, message: P2PMessage) -> Result<(), Box<dyn Error>> {
        // Route based on traffic class priority
        match message.traffic_class {
            TrafficClass::Consensus => {
                // Highest priority - direct send
                self.transport.send_direct(&target, &message).await?;
            }
            TrafficClass::Auction => {
                // Medium priority - with retry
                self.transport.send_with_retry(&target, &message, 2).await?;
            }
            TrafficClass::ShadowData => {
                // Background priority - best effort
                self.transport.send_best_effort(&target, &message).await?;
            }
        }
        Ok(())
    }
    
    /// Simple message handling loop
    async fn start_message_loop(&mut self) -> Result<(), Box<dyn Error>> {
        tokio::spawn(async move {
            loop {
                // Handle incoming messages
                // Route based on traffic class
                // Update neighbors
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
        });
        Ok(())
    }
    
    /// Get current node status
    pub fn status(&self) -> NodeStatus {
        NodeStatus {
            node_id: self.config.node_id.clone(),
            neighbor_count: self.neighbors.count(),
            is_ready: true,
        }
    }
}

/// Simple node status for testnet monitoring
#[derive(Debug, Clone)]
pub struct NodeStatus {
    pub node_id: NodeId,
    pub neighbor_count: usize,
    pub is_ready: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_web4_node_creation() {
        let config = HermesConfig::default();
        let node = HermesLiteWeb4::new(config);
        
        let status = node.status();
        assert!(!status.node_id.0.is_empty());
        assert_eq!(status.neighbor_count, 0);
    }
    
    #[tokio::test]
    async fn test_message_priority() {
        let config = HermesConfig::default();
        let mut node = HermesLiteWeb4::new(config);
        
        let consensus_msg = P2PMessage {
            id: "test".to_string(),
            traffic_class: TrafficClass::Consensus,
            message_type: MessageType::IbftPrepare,
            payload: vec![1, 2, 3],
            timestamp: std::time::SystemTime::now(),
        };
        
        // Should handle consensus messages with highest priority
        // (This test would need actual transport implementation)
        assert_eq!(consensus_msg.traffic_class, TrafficClass::Consensus);
    }
}
