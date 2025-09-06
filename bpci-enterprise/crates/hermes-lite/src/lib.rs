//! HERMES-Lite (Web-5 Edition) - Stage 1: Minimal P2P Foundation
//! 
//! Starting with the absolute essentials:
//! - S³/H³ coordinate system
//! - Basic node identity
//! - Simple neighbor discovery
//! - QUIC transport foundation

pub mod coordinates;
pub mod node;
pub mod transport;
pub mod neighbor;

pub use coordinates::{S3, H3};
pub use node::{NodeId, HermesNode};
pub use transport::QuicTransport;
pub use neighbor::NeighborSet;

/// Stage 1 configuration - kept minimal
#[derive(Debug, Clone)]
pub struct HermesConfig {
    pub node_id: NodeId,
    pub listen_port: u16,
    pub epoch_duration_secs: u64,
    pub max_neighbors: usize,
}

impl Default for HermesConfig {
    fn default() -> Self {
        Self {
            node_id: NodeId::random(),
            listen_port: 9000,
            epoch_duration_secs: 30,
            max_neighbors: 8, // Start with degree-8 as specified
        }
    }
}

/// Minimal HERMES-Lite P2P node for Stage 1 testing
pub struct HermesLite {
    config: HermesConfig,
    node: HermesNode,
    transport: QuicTransport,
    neighbors: NeighborSet,
}

impl HermesLite {
    pub fn new(config: HermesConfig) -> Self {
        let node = HermesNode::new(config.node_id.clone());
        let transport = QuicTransport::new(config.listen_port);
        let neighbors = NeighborSet::new(config.max_neighbors);
        
        Self {
            config,
            node,
            transport,
            neighbors,
        }
    }
    
    /// Start the minimal P2P node
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Starting HERMES-Lite node: {:?}", self.config.node_id);
        
        // Start QUIC transport
        self.transport.start().await?;
        
        // Initialize S³/H³ coordinates
        self.node.initialize_coordinates(0)?; // epoch 0 for now
        
        tracing::info!("HERMES-Lite node started successfully");
        tracing::info!("S³ coordinates: {:?}", self.node.s3_coords());
        tracing::info!("H³ coordinates: {:?}", self.node.h3_coords());
        
        Ok(())
    }
    
    /// Get current node information
    pub fn node_info(&self) -> &HermesNode {
        &self.node
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_minimal_hermes_node() {
        let config = HermesConfig::default();
        let mut node = HermesLite::new(config);
        
        // Should be able to start without errors
        assert!(node.start().await.is_ok());
        
        // Should have valid coordinates
        let info = node.node_info();
        assert!(info.s3_coords().is_some());
        assert!(info.h3_coords().is_some());
    }
}
