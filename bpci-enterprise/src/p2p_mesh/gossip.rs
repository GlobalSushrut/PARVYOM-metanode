//! Gossip Protocol
//! 
//! Implements epidemic-style information propagation for P2P mesh.
//! 
//! # Purpose
//! 
//! Propagate peer lists, service registry updates, and network state:
//! - Peer discovery: Share peer lists for O(n log n) connectivity
//! - Service updates: Propagate service registrations across mesh
//! - Health status: Share node health and availability
//! - Wave synchronization: Coordinate epoch transitions
//! 
//! # Algorithm
//! 
//! ```text
//! Every gossip_interval:
//! 1. Select random subset of peers (fanout)
//! 2. Send digest of local state
//! 3. Receive digest from peers
//! 4. Exchange differences (anti-entropy)
//! 5. Update local state
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use super::discovery::ServiceEndpoint;

/// Simple peer info for gossip (serializable)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GossipPeerInfo {
    pub node_id: String,
    pub address: String,
    pub stability_score: f64,
}

/// Gossip message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GossipMessage {
    /// Peer list update
    PeerUpdate {
        peers: Vec<GossipPeerInfo>,
        timestamp: u64,
    },
    
    /// Service registry update
    ServiceUpdate {
        services: Vec<ServiceEndpoint>,
        timestamp: u64,
    },
    
    /// Health status update
    HealthUpdate {
        node_id: String,
        healthy: bool,
        timestamp: u64,
    },
    
    /// Epoch synchronization
    EpochSync {
        epoch: u64,
        seed: [u8; 32],
        timestamp: u64,
    },
    
    /// State digest (for anti-entropy)
    Digest {
        peer_count: usize,
        service_count: usize,
        epoch: u64,
        timestamp: u64,
    },
}

/// Gossip configuration
#[derive(Debug, Clone)]
pub struct GossipConfig {
    /// Gossip interval
    pub interval: Duration,
    
    /// Fanout (number of peers to gossip with)
    pub fanout: usize,
    
    /// Maximum message age (for filtering stale messages)
    pub max_age: Duration,
}

impl Default for GossipConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(10),
            fanout: 3,
            max_age: Duration::from_secs(300), // 5 minutes
        }
    }
}

/// Gossip protocol state
pub struct GossipProtocol {
    /// Local node ID
    node_id: String,
    
    /// Configuration
    config: GossipConfig,
    
    /// Received message cache (for deduplication)
    message_cache: Arc<RwLock<HashMap<String, Instant>>>,
    
    /// Current epoch
    current_epoch: Arc<RwLock<u64>>,
    
    /// Epoch seed
    epoch_seed: Arc<RwLock<[u8; 32]>>,
}

impl GossipProtocol {
    /// Create a new gossip protocol
    pub fn new(node_id: String, config: GossipConfig) -> Self {
        Self {
            node_id,
            config,
            message_cache: Arc::new(RwLock::new(HashMap::new())),
            current_epoch: Arc::new(RwLock::new(0)),
            epoch_seed: Arc::new(RwLock::new([0u8; 32])),
        }
    }
    
    /// Create a peer update message
    pub fn create_peer_update(&self, peers: Vec<GossipPeerInfo>) -> GossipMessage {
        GossipMessage::PeerUpdate {
            peers,
            timestamp: Self::current_timestamp(),
        }
    }
    
    /// Create a service update message
    pub fn create_service_update(&self, services: Vec<ServiceEndpoint>) -> GossipMessage {
        GossipMessage::ServiceUpdate {
            services,
            timestamp: Self::current_timestamp(),
        }
    }
    
    /// Create a health update message
    pub fn create_health_update(&self, healthy: bool) -> GossipMessage {
        GossipMessage::HealthUpdate {
            node_id: self.node_id.clone(),
            healthy,
            timestamp: Self::current_timestamp(),
        }
    }
    
    /// Create an epoch sync message
    pub async fn create_epoch_sync(&self) -> GossipMessage {
        let epoch = *self.current_epoch.read().await;
        let seed = *self.epoch_seed.read().await;
        
        GossipMessage::EpochSync {
            epoch,
            seed,
            timestamp: Self::current_timestamp(),
        }
    }
    
    /// Create a state digest
    pub fn create_digest(
        &self,
        peer_count: usize,
        service_count: usize,
        epoch: u64,
    ) -> GossipMessage {
        GossipMessage::Digest {
            peer_count,
            service_count,
            epoch,
            timestamp: Self::current_timestamp(),
        }
    }
    
    /// Process incoming gossip message
    pub async fn process_message(
        &self,
        message: GossipMessage,
    ) -> Result<Vec<GossipMessage>, String> {
        // Check if message is too old
        if !self.is_message_fresh(&message) {
            return Ok(Vec::new());
        }
        
        // Check if we've seen this message before (deduplication)
        let message_id = self.message_id(&message);
        {
            let mut cache = self.message_cache.write().await;
            if cache.contains_key(&message_id) {
                return Ok(Vec::new());
            }
            cache.insert(message_id, Instant::now());
        }
        
        // Process based on message type
        match message {
            GossipMessage::EpochSync { epoch, seed, .. } => {
                self.handle_epoch_sync(epoch, seed).await
            }
            _ => Ok(Vec::new()),
        }
    }
    
    /// Handle epoch synchronization
    async fn handle_epoch_sync(
        &self,
        epoch: u64,
        seed: [u8; 32],
    ) -> Result<Vec<GossipMessage>, String> {
        let mut current_epoch = self.current_epoch.write().await;
        
        if epoch > *current_epoch {
            *current_epoch = epoch;
            *self.epoch_seed.write().await = seed;
            
            // Propagate to other peers
            Ok(vec![self.create_epoch_sync().await])
        } else {
            Ok(Vec::new())
        }
    }
    
    /// Check if message is fresh
    fn is_message_fresh(&self, message: &GossipMessage) -> bool {
        let timestamp = match message {
            GossipMessage::PeerUpdate { timestamp, .. } => *timestamp,
            GossipMessage::ServiceUpdate { timestamp, .. } => *timestamp,
            GossipMessage::HealthUpdate { timestamp, .. } => *timestamp,
            GossipMessage::EpochSync { timestamp, .. } => *timestamp,
            GossipMessage::Digest { timestamp, .. } => *timestamp,
        };
        
        let now = Self::current_timestamp();
        let age = now.saturating_sub(timestamp);
        
        age <= self.config.max_age.as_secs()
    }
    
    /// Generate message ID for deduplication
    fn message_id(&self, message: &GossipMessage) -> String {
        match message {
            GossipMessage::PeerUpdate { timestamp, .. } => {
                format!("peer_{}", timestamp)
            }
            GossipMessage::ServiceUpdate { timestamp, .. } => {
                format!("service_{}", timestamp)
            }
            GossipMessage::HealthUpdate { node_id, timestamp, .. } => {
                format!("health_{}_{}", node_id, timestamp)
            }
            GossipMessage::EpochSync { epoch, .. } => {
                format!("epoch_{}", epoch)
            }
            GossipMessage::Digest { timestamp, .. } => {
                format!("digest_{}", timestamp)
            }
        }
    }
    
    /// Get current timestamp
    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    
    /// Cleanup old messages from cache
    pub async fn cleanup_cache(&self) {
        let mut cache = self.message_cache.write().await;
        let now = Instant::now();
        
        cache.retain(|_, timestamp| {
            now.duration_since(*timestamp) <= self.config.max_age
        });
    }
    
    /// Get current epoch
    pub async fn current_epoch(&self) -> u64 {
        *self.current_epoch.read().await
    }
    
    /// Get epoch seed
    pub async fn epoch_seed(&self) -> [u8; 32] {
        *self.epoch_seed.read().await
    }
    
    /// Set epoch (for testing or initialization)
    pub async fn set_epoch(&self, epoch: u64, seed: [u8; 32]) {
        *self.current_epoch.write().await = epoch;
        *self.epoch_seed.write().await = seed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_gossip_creation() {
        let config = GossipConfig::default();
        let gossip = GossipProtocol::new("node1".to_string(), config);
        
        assert_eq!(gossip.current_epoch().await, 0);
    }
    
    #[tokio::test]
    async fn test_peer_update_message() {
        let config = GossipConfig::default();
        let gossip = GossipProtocol::new("node1".to_string(), config);
        
        let peers = vec![];
        let msg = gossip.create_peer_update(peers);
        
        match msg {
            GossipMessage::PeerUpdate { .. } => (),
            _ => panic!("Wrong message type"),
        }
    }
    
    #[tokio::test]
    async fn test_epoch_sync() {
        let config = GossipConfig::default();
        let gossip = GossipProtocol::new("node1".to_string(), config);
        
        let seed = [42u8; 32];
        gossip.set_epoch(5, seed).await;
        
        assert_eq!(gossip.current_epoch().await, 5);
        assert_eq!(gossip.epoch_seed().await, seed);
    }
    
    #[tokio::test]
    async fn test_message_deduplication() {
        let config = GossipConfig::default();
        let gossip = GossipProtocol::new("node1".to_string(), config);
        
        let msg = gossip.create_health_update(true);
        
        // First processing should succeed
        let result1 = gossip.process_message(msg.clone()).await;
        assert!(result1.is_ok());
        
        // Second processing should be deduplicated
        let result2 = gossip.process_message(msg).await;
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap().len(), 0);
    }
    
    #[tokio::test]
    async fn test_epoch_propagation() {
        let config = GossipConfig::default();
        let gossip = GossipProtocol::new("node1".to_string(), config);
        
        let seed = [99u8; 32];
        let msg = GossipMessage::EpochSync {
            epoch: 10,
            seed,
            timestamp: GossipProtocol::current_timestamp(),
        };
        
        let result = gossip.process_message(msg).await.unwrap();
        
        // Should propagate to other peers
        assert_eq!(result.len(), 1);
        assert_eq!(gossip.current_epoch().await, 10);
    }
}
