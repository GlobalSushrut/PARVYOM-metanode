//! P2P Mesh Handshake Protocol with Fibonacci-Stability
//! 
//! Implements O(n log n) peer discovery with Fibonacci-Stability scoring.
//! 
//! # Purpose
//! 
//! Enable serverless peer discovery with stability-based admission:
//! - Bootstrap from seed nodes
//! - Fibonacci-Stability handshake (HELLO₁, HELLO₂, ACK₃)
//! - Rolling stability scoring with golden ratio thresholds
//! - Witness quorum endorsements
//! - Fibonacci lease duration and resource credits
//! - Establish O(log n) connections per node
//! - Propagate service information via gossip
//! 
//! # Algorithm
//! 
//! ```text
//! 1. Bootstrap: Connect to seed nodes
//! 2. Handshake: 3-way Fibonacci-Stability protocol
//! 3. Score: Calculate stability with witness quorum
//! 4. Admit: Accept (≥φ⁻¹), Probation (≥φ⁻²), or Reject
//! 5. Lease: Issue Fibonacci-graded lease and credits
//! 6. Connect: Maintain O(log n) connections
//! 7. Gossip: Propagate peer lists and service updates
//! ```

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use super::discovery::ServiceRegistry;
use super::fibonacci_stability::*;
use super::handshake_protocol::*;
use super::gossip::{GossipProtocol, GossipConfig};
use super::wave::WaveScheduler;

/// Peer connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// Disconnected
    Disconnected,
    
    /// Handshaking (HELLO₁ sent)
    Handshaking,
    
    /// Connected with full accept
    Connected,
    
    /// Connected on probation
    Probation,
    
    /// Failed (retry with backoff)
    Failed,
}

/// Peer information with Fibonacci-Stability metrics
#[derive(Debug, Clone)]
pub struct PeerInfo {
    /// Peer node ID
    pub node_id: String,
    
    /// Peer address
    pub address: SocketAddr,
    
    /// Ephemeral key
    pub ephemeral_key: Option<[u8; 32]>,
    
    /// Connection state
    pub state: ConnectionState,
    
    /// Stability tracker
    pub stability: StabilityTracker,
    
    /// Current lease
    pub lease: Option<Lease>,
    
    /// Backoff calculator
    pub backoff: FibonacciBackoff,
    
    /// Last seen timestamp
    pub last_seen: Instant,
    
    /// Last handshake attempt
    pub last_handshake: Option<Instant>,
    
    /// Peer metadata
    pub metadata: HashMap<String, String>,
}

/// P2P Mesh configuration
#[derive(Debug, Clone)]
pub struct MeshConfig {
    /// Local node address
    pub local_address: SocketAddr,
    
    /// Seed nodes for bootstrap
    pub seed_nodes: Vec<SocketAddr>,
    
    /// Target connections per node (O(log n))
    pub target_connections: usize,
    
    /// Gossip interval
    pub gossip_interval: Duration,
    
    /// Peer timeout
    pub peer_timeout: Duration,
}

impl Default for MeshConfig {
    fn default() -> Self {
        Self {
            local_address: "127.0.0.1:0".parse().unwrap(),
            seed_nodes: Vec::new(),
            target_connections: 8, // log2(256) = 8 connections
            gossip_interval: Duration::from_secs(10),
            peer_timeout: Duration::from_secs(60),
        }
    }
}

/// P2P Mesh with Fibonacci-Stability
pub struct P2PMesh {
    /// Local node ID
    node_id: String,
    
    /// Local ephemeral key (rotated every F_k seconds)
    ephemeral_key: Arc<RwLock<[u8; 32]>>,
    
    /// Configuration
    config: MeshConfig,
    
    /// Connected peers
    peers: Arc<RwLock<HashMap<String, PeerInfo>>>,
    
    /// Service registry
    registry: Arc<ServiceRegistry>,
    
    /// Gossip protocol
    gossip: Arc<GossipProtocol>,
    
    /// Wave scheduler
    wave_scheduler: Arc<WaveScheduler>,
    
    /// Stability parameters
    stability_params: StabilityParams,
    
    /// Retry token secret
    retry_secret: [u8; 32],
}

impl P2PMesh {
    /// Create a new P2P mesh with Fibonacci-Stability
    pub fn new(
        node_id: String,
        config: MeshConfig,
        registry: Arc<ServiceRegistry>,
    ) -> Self {
        // Generate initial ephemeral key
        let mut ephemeral_key = [0u8; 32];
        use rand::RngCore;
        rand::thread_rng().fill_bytes(&mut ephemeral_key);
        
        // Generate retry token secret
        let mut retry_secret = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut retry_secret);
        
        // Create gossip protocol
        let gossip_config = GossipConfig::default();
        let gossip = Arc::new(GossipProtocol::new(node_id.clone(), gossip_config));
        
        // Create wave scheduler (10 waves per epoch)
        let wave_scheduler = Arc::new(WaveScheduler::new(10));
        
        Self {
            node_id,
            ephemeral_key: Arc::new(RwLock::new(ephemeral_key)),
            config,
            peers: Arc::new(RwLock::new(HashMap::new())),
            registry,
            gossip,
            wave_scheduler,
            stability_params: StabilityParams::default(),
            retry_secret,
        }
    }
    
    /// Bootstrap from seed nodes
    pub async fn bootstrap(&mut self) -> Result<(), String> {
        let seed_nodes = self.config.seed_nodes.clone();
        
        for seed_addr in seed_nodes {
            match self.handshake(seed_addr).await {
                Ok(peer_info) => {
                    println!("Connected to seed node: {}", peer_info.node_id);
                }
                Err(e) => {
                    println!("Failed to connect to seed {}: {}", seed_addr, e);
                }
            }
        }
        
        Ok(())
    }
    
    /// Perform Fibonacci-Stability handshake with a peer
    pub async fn handshake(&mut self, peer_addr: SocketAddr) -> Result<PeerInfo, String> {
        // 1. Create HELLO₁ message
        let ephemeral_key = *self.ephemeral_key.read().await;
        let hello1 = Hello1::new(self.node_id.clone(), ephemeral_key, 12); // 12-bit PoW
        
        // 2. Send HELLO₁ (in real impl, use QUIC/TCP)
        // For now, simulate network exchange
        println!("Sending HELLO₁ to {}", peer_addr);
        
        // 3. Receive HELLO₂ (simulated)
        // In real implementation, this would be received over network
        let responder_key = [2u8; 32]; // Simulated responder key
        let shared_key = self.derive_shared_key(&ephemeral_key, &responder_key);
        
        let retry_token = RetryToken::new(peer_addr, &self.retry_secret);
        
        // Get witness endorsements from existing peers
        let witnesses = self.get_witness_endorsements(&self.node_id, 3).await;
        
        let hello2 = Hello2::new(
            format!("peer_{}", peer_addr.port()),
            responder_key,
            retry_token.clone(),
            witnesses.clone(),
            &shared_key,
            &hello1,
        );
        
        // 4. Verify HELLO₂
        if !hello2.verify_hmac(&shared_key, &hello1) {
            return Err("HMAC verification failed".to_string());
        }
        
        if !retry_token.verify(&self.retry_secret) {
            return Err("Retry token verification failed".to_string());
        }
        
        // 5. Calculate blended stability score
        let link_score = 0.85; // In real impl, calculate from metrics
        let blended = blended_score(link_score, &witnesses, self.stability_params.lambda);
        
        // 6. Check admission decision
        let decision = admission_decision(blended);
        
        match decision {
            AdmissionDecision::Reject => {
                return Err(format!("Peer rejected (score: {:.3})", blended));
            }
            _ => {}
        }
        
        // 7. Send ACK₃
        let ack3 = Ack3::new(&shared_key, &hello1, &hello2);
        println!("Sending ACK₃ to {}", peer_addr);
        
        // 8. Create lease
        let duration = lease_duration(blended);
        let credits = resource_credits(blended, 1000.0, 1.5);
        let lease = Lease::new(hello2.node_id.clone(), duration.as_secs(), credits);
        
        // 9. Create peer info
        let state = match decision {
            AdmissionDecision::Accept => ConnectionState::Connected,
            AdmissionDecision::Probation => ConnectionState::Probation,
            AdmissionDecision::Reject => ConnectionState::Failed,
        };
        
        let peer_info = PeerInfo {
            node_id: hello2.node_id.clone(),
            address: peer_addr,
            ephemeral_key: Some(responder_key),
            state,
            stability: StabilityTracker::new(self.stability_params.clone()),
            lease: Some(lease),
            backoff: FibonacciBackoff::new(),
            last_seen: Instant::now(),
            last_handshake: Some(Instant::now()),
            metadata: HashMap::new(),
        };
        
        // 10. Store peer
        self.peers.write().await.insert(hello2.node_id.clone(), peer_info.clone());
        
        println!(
            "Handshake complete with {} (score: {:.3}, state: {:?})",
            hello2.node_id,
            blended,
            state
        );
        
        Ok(peer_info)
    }
    
    /// Derive shared key from ECDH (simplified for now)
    fn derive_shared_key(&self, local_key: &[u8; 32], remote_key: &[u8; 32]) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(b"SHARED_KEY_V1");
        hasher.update(local_key);
        hasher.update(remote_key);
        hasher.finalize().into()
    }
    
    /// Get witness endorsements from existing peers
    async fn get_witness_endorsements(&self, target_id: &str, count: usize) -> Vec<WitnessEndorsement> {
        let peers = self.peers.read().await;
        
        peers
            .values()
            .filter(|p| p.state == ConnectionState::Connected)
            .take(count)
            .map(|p| {
                let score = p.stability.calculate_score();
                WitnessEndorsement {
                    witness_id: p.node_id.clone(),
                    target_id: target_id.to_string(),
                    stability_score: score,
                    timestamp: 0,
                    signature: [0u8; 32],
                }
            })
            .collect()
    }
    
    /// Get connected peer count
    pub async fn peer_count(&self) -> usize {
        let peers = self.peers.read().await;
        peers.values().filter(|p| p.state == ConnectionState::Connected).count()
    }
    
    /// Get all peers
    pub async fn get_peers(&self) -> Vec<PeerInfo> {
        let peers = self.peers.read().await;
        peers.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::p2p_mesh::discovery::RegistryConfig;
    
    #[tokio::test]
    async fn test_mesh_creation() {
        let config = MeshConfig::default();
        let registry_config = RegistryConfig::default();
        let registry = Arc::new(ServiceRegistry::new("node1".to_string(), registry_config));
        
        let mesh = P2PMesh::new("node1".to_string(), config, registry);
        
        assert_eq!(mesh.peer_count().await, 0);
    }
    
    #[test]
    fn test_mesh_config() {
        let config = MeshConfig::default();
        assert_eq!(config.target_connections, 8);
    }
}
