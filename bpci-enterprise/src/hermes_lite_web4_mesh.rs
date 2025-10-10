//! HERMES-Lite Web-4 Mesh Integration for BPCI LCCD
//! 
//! Phase 2 of Living Cellular Consensus Division (LCCD) implementation.
//! Integrates the mathematical foundation with HERMES-Lite Web-4 mesh networking
//! for real-time consensus propagation across the BPCI network.
//! 
//! Features:
//! - Living mesh nodes that integrate with Category-Chain nervous system
//! - κ-aware mesh routing based on circulatory health
//! - NxTri immune system integration for attack-resistant mesh
//! - Cellular division propagation across mesh network
//! - WAN-scale mesh coordination with minimal hardware efficiency

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use anyhow::Result;
use uuid::Uuid;

use crate::lccd_mathematical_foundation::{
    LccdMathematicalFoundation, LivingStateObject, ObjectId, Hash32, TriCoeff
};

/// HERMES-Lite Web-4 mesh node identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MeshNodeId(pub String);

impl MeshNodeId {
    pub fn generate() -> Self {
        Self(format!("hermes-{}", Uuid::new_v4()))
    }
    
    pub fn from_living_state(state_id: &ObjectId) -> Self {
        Self(format!("hermes-{}", state_id.0))
    }
}

/// Web-4 mesh network address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Web4Address {
    pub node_id: MeshNodeId,
    pub ip_address: String,
    pub port: u16,
    pub quantum_channel: Option<String>,
    pub mesh_layer: u8, // 0-7 for Web-4 hierarchical mesh
}

/// Living mesh node that integrates with LCCD mathematical foundation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivingMeshNode {
    pub node_id: MeshNodeId,
    pub address: Web4Address,
    pub living_state: LivingStateObject,
    pub mesh_health: f64, // 0.0 to 1.0
    pub kappa_routing_weight: f64,
    pub consensus_participation: bool,
    pub cellular_division_ready: bool,
    pub connected_peers: Vec<MeshNodeId>,
    pub last_heartbeat: DateTime<Utc>,
}

impl LivingMeshNode {
    pub fn new(address: Web4Address, living_state: LivingStateObject) -> Self {
        let node_id = MeshNodeId::from_living_state(&living_state.state_id);
        
        Self {
            node_id,
            address,
            living_state,
            mesh_health: 1.0,
            kappa_routing_weight: 1.0,
            consensus_participation: true,
            cellular_division_ready: false,
            connected_peers: Vec::new(),
            last_heartbeat: Utc::now(),
        }
    }
    
    /// Update mesh node health based on κ-circulatory system
    pub fn update_mesh_health(&mut self, kappa: f64, confidence: &TriCoeff) {
        // Mesh health is influenced by κ stability and NxTri confidence
        let kappa_health = if kappa > 0.001 && kappa < 100.0 { 1.0 } else { 0.5 };
        let confidence_health = confidence.overall_confidence();
        
        self.mesh_health = (kappa_health + confidence_health) / 2.0;
        self.kappa_routing_weight = 1.0 / (1.0 + kappa.abs());
        
        // Check if ready for cellular division
        self.cellular_division_ready = self.living_state.can_divide() && self.mesh_health > 0.8;
        
        self.last_heartbeat = Utc::now();
    }
    
    /// Check if node is healthy and responsive
    pub fn is_healthy(&self) -> bool {
        let age_seconds = (Utc::now() - self.last_heartbeat).num_seconds();
        self.mesh_health > 0.5 && age_seconds < 30 // 30 second heartbeat timeout
    }
}

/// HERMES-Lite Web-4 mesh message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Web4MessageType {
    ConsensusProposal,
    ConsensusVote,
    KappaHealthUpdate,
    NxTriConfidenceSync,
    CellularDivisionNotice,
    MeshTopologyUpdate,
    QuantumChannelEstablish,
    ByzantineAttackAlert,
}

/// Web-4 mesh message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Web4MeshMessage {
    pub message_id: String,
    pub message_type: Web4MessageType,
    pub source_node: MeshNodeId,
    pub target_nodes: Vec<MeshNodeId>, // Empty for broadcast
    pub payload: Vec<u8>,
    pub kappa_priority: f64,
    pub confidence_signature: TriCoeff,
    pub timestamp: DateTime<Utc>,
    pub ttl: u32, // Time to live for mesh propagation
}

impl Web4MeshMessage {
    pub fn new(
        message_type: Web4MessageType,
        source_node: MeshNodeId,
        payload: Vec<u8>,
        kappa_priority: f64,
        confidence: TriCoeff,
    ) -> Self {
        Self {
            message_id: Uuid::new_v4().to_string(),
            message_type,
            source_node,
            target_nodes: Vec::new(), // Broadcast by default
            payload,
            kappa_priority,
            confidence_signature: confidence,
            timestamp: Utc::now(),
            ttl: 10, // 10 hops maximum
        }
    }
    
    /// Create consensus proposal message
    pub fn consensus_proposal(
        source_node: MeshNodeId,
        proposal_data: &[u8],
        kappa: f64,
        confidence: TriCoeff,
    ) -> Self {
        Self::new(
            Web4MessageType::ConsensusProposal,
            source_node,
            proposal_data.to_vec(),
            kappa,
            confidence,
        )
    }
    
    /// Create cellular division notice
    pub fn cellular_division_notice(
        source_node: MeshNodeId,
        division_data: &[u8],
        kappa: f64,
        confidence: TriCoeff,
    ) -> Self {
        Self::new(
            Web4MessageType::CellularDivisionNotice,
            source_node,
            division_data.to_vec(),
            kappa,
            confidence,
        )
    }
}

/// κ-aware mesh routing table
#[derive(Debug)]
pub struct KappaAwareMeshRouter {
    pub routing_table: Arc<RwLock<HashMap<MeshNodeId, Vec<MeshNodeId>>>>, // node -> next hops
    pub kappa_weights: Arc<RwLock<HashMap<MeshNodeId, f64>>>, // routing weights based on κ
    pub mesh_topology: Arc<RwLock<HashMap<MeshNodeId, LivingMeshNode>>>,
}

impl KappaAwareMeshRouter {
    pub fn new() -> Self {
        Self {
            routing_table: Arc::new(RwLock::new(HashMap::new())),
            kappa_weights: Arc::new(RwLock::new(HashMap::new())),
            mesh_topology: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Add mesh node to topology
    pub async fn add_mesh_node(&self, node: LivingMeshNode) -> Result<()> {
        let mut topology = self.mesh_topology.write().await;
        let mut weights = self.kappa_weights.write().await;
        
        weights.insert(node.node_id.clone(), node.kappa_routing_weight);
        topology.insert(node.node_id.clone(), node);
        
        Ok(())
    }
    
    /// Update κ-based routing weights
    pub async fn update_kappa_weights(&self, node_id: &MeshNodeId, kappa: f64) -> Result<()> {
        let mut weights = self.kappa_weights.write().await;
        let weight = 1.0 / (1.0 + kappa.abs()); // Lower κ = higher routing priority
        weights.insert(node_id.clone(), weight);
        
        // Update mesh node
        let mut topology = self.mesh_topology.write().await;
        if let Some(node) = topology.get_mut(node_id) {
            node.kappa_routing_weight = weight;
        }
        
        Ok(())
    }
    
    /// Find optimal routing path based on κ-weights
    pub async fn find_optimal_path(
        &self,
        source: &MeshNodeId,
        target: &MeshNodeId,
    ) -> Result<Vec<MeshNodeId>> {
        let topology = self.mesh_topology.read().await;
        let weights = self.kappa_weights.read().await;
        
        // Simplified κ-aware pathfinding (in production, use Dijkstra with κ-weights)
        let mut path = Vec::new();
        path.push(source.clone());
        
        // Find intermediate nodes with best κ-weights
        let mut best_intermediate: Option<MeshNodeId> = None;
        let mut best_weight = 0.0;
        
        for (node_id, node) in topology.iter() {
            if node_id != source && node_id != target && node.is_healthy() {
                if let Some(weight) = weights.get(node_id) {
                    if *weight > best_weight {
                        best_weight = *weight;
                        best_intermediate = Some(node_id.clone());
                    }
                }
            }
        }
        
        if let Some(intermediate) = best_intermediate {
            path.push(intermediate);
        }
        
        path.push(target.clone());
        Ok(path)
    }
}

/// HERMES-Lite Web-4 Mesh Network Manager
#[derive(Debug)]
pub struct HermesLiteWeb4Mesh {
    pub mesh_id: String,
    pub local_node: Arc<RwLock<LivingMeshNode>>,
    pub router: KappaAwareMeshRouter,
    pub lccd_foundation: Arc<LccdMathematicalFoundation>,
    pub message_queue: Arc<RwLock<Vec<Web4MeshMessage>>>,
    pub consensus_channel: mpsc::UnboundedSender<Web4MeshMessage>,
    pub mesh_stats: Arc<RwLock<MeshNetworkStats>>,
}

/// Mesh network statistics
#[derive(Debug, Default)]
pub struct MeshNetworkStats {
    pub total_nodes: usize,
    pub healthy_nodes: usize,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub consensus_rounds: u64,
    pub cellular_divisions: u64,
    pub average_kappa: f64,
    pub average_confidence: f64,
}

impl HermesLiteWeb4Mesh {
    /// Create new HERMES-Lite Web-4 mesh
    pub fn new(
        local_address: Web4Address,
        lccd_foundation: Arc<LccdMathematicalFoundation>,
    ) -> Result<Self> {
        // Create local living state for mesh node
        let state_hash = Hash32::from_data(local_address.node_id.0.as_bytes());
        let living_state = LivingStateObject::new(state_hash);
        let local_node = LivingMeshNode::new(local_address, living_state);
        
        let (consensus_tx, _consensus_rx) = mpsc::unbounded_channel();
        
        Ok(Self {
            mesh_id: format!("hermes-web4-{}", Uuid::new_v4()),
            local_node: Arc::new(RwLock::new(local_node)),
            router: KappaAwareMeshRouter::new(),
            lccd_foundation,
            message_queue: Arc::new(RwLock::new(Vec::new())),
            consensus_channel: consensus_tx,
            mesh_stats: Arc::new(RwLock::new(MeshNetworkStats::default())),
        })
    }
    
    /// Join the HERMES-Lite Web-4 mesh network
    pub async fn join_mesh(&self, bootstrap_nodes: Vec<Web4Address>) -> Result<()> {
        // Add local node to router
        let local_node = self.local_node.read().await.clone();
        self.router.add_mesh_node(local_node).await?;
        
        // Connect to bootstrap nodes
        for bootstrap_addr in bootstrap_nodes {
            let bootstrap_hash = Hash32::from_data(bootstrap_addr.node_id.0.as_bytes());
            let bootstrap_state = LivingStateObject::new(bootstrap_hash);
            let bootstrap_node = LivingMeshNode::new(bootstrap_addr, bootstrap_state);
            
            self.router.add_mesh_node(bootstrap_node).await?;
        }
        
        // Update mesh stats
        let mut stats = self.mesh_stats.write().await;
        stats.total_nodes += 1;
        stats.healthy_nodes += 1;
        
        Ok(())
    }
    
    /// Process consensus round through mesh
    pub async fn process_mesh_consensus_round(&self, network_health: f64) -> Result<TriCoeff> {
        // Get current κ and confidence from LCCD foundation
        let kappa = self.lccd_foundation.kappa_circulatory.get_current_kappa().await;
        let confidence = self.lccd_foundation.process_consensus_round(network_health).await?;
        
        // Update local mesh node health
        let mut local_node = self.local_node.write().await;
        local_node.update_mesh_health(kappa, &confidence);
        
        // Update router κ-weights
        self.router.update_kappa_weights(&local_node.node_id, kappa).await?;
        
        // Broadcast consensus proposal to mesh
        let proposal_data = format!("consensus_round_kappa_{:.6}_confidence_{:.3}_{:.3}_{:.3}", 
                                   kappa, confidence.alpha, confidence.beta, confidence.gamma);
        
        let consensus_message = Web4MeshMessage::consensus_proposal(
            local_node.node_id.clone(),
            proposal_data.as_bytes(),
            kappa,
            confidence.clone(),
        );
        
        self.broadcast_message(consensus_message).await?;
        
        // Update mesh stats
        let mut stats = self.mesh_stats.write().await;
        stats.consensus_rounds += 1;
        stats.average_kappa = (stats.average_kappa + kappa) / 2.0;
        stats.average_confidence = (stats.average_confidence + confidence.overall_confidence()) / 2.0;
        
        Ok(confidence)
    }
    
    /// Broadcast message to mesh network
    pub async fn broadcast_message(&self, message: Web4MeshMessage) -> Result<()> {
        // Add to local message queue
        let mut queue = self.message_queue.write().await;
        queue.push(message.clone());
        
        // Send through consensus channel
        self.consensus_channel.send(message)?;
        
        // Update stats
        let mut stats = self.mesh_stats.write().await;
        stats.messages_sent += 1;
        
        Ok(())
    }
    
    /// Handle cellular division across mesh
    pub async fn handle_cellular_division(&self) -> Result<()> {
        let local_node = self.local_node.read().await;
        
        if local_node.cellular_division_ready {
            // Perform cellular division in LCCD foundation
            if let Ok((cell_a, cell_b)) = local_node.living_state.divide() {
                // Create division notice message
                let division_data = format!("cellular_division_{}_{}", cell_a.state_id.0, cell_b.state_id.0);
                let kappa = self.lccd_foundation.kappa_circulatory.get_current_kappa().await;
                let confidence = self.lccd_foundation.nxtri_immune.get_current_confidence().await;
                
                let division_message = Web4MeshMessage::cellular_division_notice(
                    local_node.node_id.clone(),
                    division_data.as_bytes(),
                    kappa,
                    confidence,
                );
                
                self.broadcast_message(division_message).await?;
                
                // Update stats
                let mut stats = self.mesh_stats.write().await;
                stats.cellular_divisions += 1;
            }
        }
        
        Ok(())
    }
    
    /// Get mesh network health status
    pub async fn get_mesh_health(&self) -> Result<MeshHealthStatus> {
        let stats = self.mesh_stats.read().await;
        let local_node = self.local_node.read().await;
        let topology = self.router.mesh_topology.read().await;
        
        // Count healthy nodes
        let healthy_count = topology.values().filter(|node| node.is_healthy()).count();
        let total_count = topology.len();
        
        let health_ratio = if total_count > 0 { healthy_count as f64 / total_count as f64 } else { 0.0 };
        
        Ok(MeshHealthStatus {
            mesh_id: self.mesh_id.clone(),
            total_nodes: total_count,
            healthy_nodes: healthy_count,
            health_ratio,
            local_node_health: local_node.mesh_health,
            average_kappa: stats.average_kappa,
            average_confidence: stats.average_confidence,
            consensus_rounds: stats.consensus_rounds,
            cellular_divisions: stats.cellular_divisions,
            messages_throughput: stats.messages_sent + stats.messages_received,
        })
    }
}

/// Mesh network health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshHealthStatus {
    pub mesh_id: String,
    pub total_nodes: usize,
    pub healthy_nodes: usize,
    pub health_ratio: f64,
    pub local_node_health: f64,
    pub average_kappa: f64,
    pub average_confidence: f64,
    pub consensus_rounds: u64,
    pub cellular_divisions: u64,
    pub messages_throughput: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_living_mesh_node_creation() {
        let address = Web4Address {
            node_id: MeshNodeId::generate(),
            ip_address: "192.168.1.100".to_string(),
            port: 8080,
            quantum_channel: Some("quantum-channel-1".to_string()),
            mesh_layer: 2,
        };
        
        let state_hash = Hash32::from_data(b"test_mesh_node");
        let living_state = LivingStateObject::new(state_hash);
        let mesh_node = LivingMeshNode::new(address, living_state);
        
        assert!(mesh_node.is_healthy());
        assert_eq!(mesh_node.mesh_health, 1.0);
    }
    
    #[tokio::test]
    async fn test_kappa_aware_routing() {
        let router = KappaAwareMeshRouter::new();
        
        // Add test nodes
        let address1 = Web4Address {
            node_id: MeshNodeId("node1".to_string()),
            ip_address: "192.168.1.1".to_string(),
            port: 8080,
            quantum_channel: None,
            mesh_layer: 1,
        };
        
        let state_hash = Hash32::from_data(b"node1");
        let living_state = LivingStateObject::new(state_hash);
        let node1 = LivingMeshNode::new(address1, living_state);
        
        router.add_mesh_node(node1).await.unwrap();
        router.update_kappa_weights(&MeshNodeId("node1".to_string()), 1.5).await.unwrap();
        
        let weights = router.kappa_weights.read().await;
        assert!(weights.contains_key(&MeshNodeId("node1".to_string())));
    }
    
    #[tokio::test]
    async fn test_hermes_lite_web4_mesh() {
        let lccd_foundation = Arc::new(LccdMathematicalFoundation::new());
        
        let local_address = Web4Address {
            node_id: MeshNodeId::generate(),
            ip_address: "127.0.0.1".to_string(),
            port: 9090,
            quantum_channel: Some("local-quantum".to_string()),
            mesh_layer: 0,
        };
        
        let mesh = HermesLiteWeb4Mesh::new(local_address, lccd_foundation).unwrap();
        
        // Test mesh consensus round
        let confidence = mesh.process_mesh_consensus_round(0.9).await.unwrap();
        assert!(confidence.overall_confidence() >= 0.0);
        
        // Test mesh health
        let health = mesh.get_mesh_health().await.unwrap();
        assert_eq!(health.total_nodes, 1);
    }
}
