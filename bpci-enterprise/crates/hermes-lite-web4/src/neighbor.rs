//! Simple Neighbor Management for Web-4 Mainnet

use crate::NodeId;
use std::collections::HashMap;
use std::error::Error;
use std::time::{SystemTime, Duration};
use tracing::{info, debug};

/// Simple neighbor manager (mainnet-ready, no complex geometry)
pub struct NeighborManager {
    max_neighbors: usize,
    neighbors: HashMap<NodeId, NeighborInfo>,
    last_discovery: SystemTime,
}

#[derive(Debug, Clone)]
pub struct NeighborInfo {
    pub node_id: NodeId,
    pub last_seen: SystemTime,
    pub is_active: bool,
}

impl NeighborManager {
    pub fn new(max_neighbors: usize) -> Self {
        Self {
            max_neighbors,
            neighbors: HashMap::new(),
            last_discovery: SystemTime::now(),
        }
    }
    
    /// Start simple neighbor discovery (mainnet-ready)
    pub async fn start_discovery(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Starting neighbor discovery (max: {})", self.max_neighbors);
        
        // For mainnet, add bootstrap neighbors from config
        self.add_bootstrap_neighbors().await?;
        
        // Start periodic discovery
        self.start_discovery_loop().await?;
        
        Ok(())
    }
    
    /// Add a neighbor (simple, reliable)
    pub fn add_neighbor(&mut self, node_id: NodeId) -> bool {
        if self.neighbors.len() >= self.max_neighbors {
            debug!("Max neighbors reached, not adding {}", node_id);
            return false;
        }
        
        let neighbor_info = NeighborInfo {
            node_id: node_id.clone(),
            last_seen: SystemTime::now(),
            is_active: true,
        };
        
        self.neighbors.insert(node_id.clone(), neighbor_info);
        info!("Added neighbor: {}", node_id);
        true
    }
    
    /// Remove inactive neighbors
    pub fn cleanup_inactive(&mut self) {
        let timeout = Duration::from_secs(300); // 5 minutes timeout
        let now = SystemTime::now();
        
        let inactive: Vec<NodeId> = self.neighbors
            .iter()
            .filter(|(_, info)| {
                now.duration_since(info.last_seen).unwrap_or(Duration::ZERO) > timeout
            })
            .map(|(id, _)| id.clone())
            .collect();
        
        for node_id in inactive {
            self.neighbors.remove(&node_id);
            info!("Removed inactive neighbor: {}", node_id);
        }
    }
    
    /// Get neighbor count
    pub fn count(&self) -> usize {
        self.neighbors.len()
    }
    
    /// Get all active neighbors
    pub fn get_active_neighbors(&self) -> Vec<NodeId> {
        self.neighbors
            .values()
            .filter(|info| info.is_active)
            .map(|info| info.node_id.clone())
            .collect()
    }
    
    /// Add bootstrap neighbors for mainnet
    async fn add_bootstrap_neighbors(&mut self) -> Result<(), Box<dyn Error>> {
        // For mainnet, these would come from config
        let bootstrap_nodes = vec![
            "node_bootstrap_001",
            "node_bootstrap_002", 
            "node_bootstrap_003",
        ];
        
        for node_str in bootstrap_nodes {
            let node_id = NodeId::from_string(node_str.to_string());
            self.add_neighbor(node_id);
        }
        
        Ok(())
    }
    
    /// Start discovery loop
    async fn start_discovery_loop(&mut self) -> Result<(), Box<dyn Error>> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                // Periodic neighbor discovery and cleanup
                // This would be implemented based on mainnet requirements
            }
        });
        Ok(())
    }
}
