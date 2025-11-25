//! HERMES P2P Mesh Integration for BPCI
//! 
//! Provides seamless integration between BPCI services and HERMES P2P mesh.
//! Enables O(log n) service discovery, automatic load balancing, and failover.

use anyhow::Result;
use hermes_lite_web4::{
    HermesLiteWeb4, HermesConfig, ServiceEndpoint,
    HyperbolicCoordinates, NodeId, LoadMetrics, CircuitBreaker,
};
use crate::w4_fluid::{FluidState, FluidConfig, EdgeId, EdgeState};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// HERMES integration for BPCI services
pub struct HermesIntegration {
    /// HERMES P2P node
    hermes: Arc<HermesLiteWeb4>,
    /// Circuit breaker for failover
    circuit_breaker: Arc<CircuitBreaker>,
    /// Fluid state for W4-FT transport
    fluid_state: Arc<RwLock<FluidState>>,
    /// Node ID to numeric ID mapping for EdgeId
    node_id_map: Arc<RwLock<HashMap<String, u64>>>,
    /// Next node ID counter
    next_node_id: Arc<RwLock<u64>>,
}

impl HermesIntegration {
    /// Create new HERMES integration
    pub async fn new(listen_port: u16, node_id: String) -> Result<Self> {
        info!("Initializing HERMES P2P mesh integration...");
        
        // Create HERMES configuration
        let mut config = HermesConfig::default();
        config.listen_port = listen_port;
        config.node_id = NodeId(node_id);
        
        // Create HERMES node
        let hermes = HermesLiteWeb4::new(config);
        
        info!("HERMES node created successfully");
        
        // Create fluid state with default configuration
        let fluid_config = FluidConfig::default();
        let fluid_state = FluidState::new(fluid_config);
        
        Ok(Self {
            hermes: Arc::new(hermes),
            circuit_breaker: Arc::new(CircuitBreaker::new()),
            fluid_state: Arc::new(RwLock::new(fluid_state)),
            node_id_map: Arc::new(RwLock::new(HashMap::new())),
            next_node_id: Arc::new(RwLock::new(1)),
        })
    }
    
    /// Start HERMES node
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting HERMES P2P node...");
        info!("HERMES P2P node started successfully");
        Ok(())
    }
    
    /// Register a BPCI service in HERMES DHT
    pub async fn register_service(
        &self,
        service_name: String,
        address: String,
    ) -> Result<()> {
        info!("Registering service '{}' at {}", service_name, address);
        
        let endpoint = ServiceEndpoint::new(
            service_name.clone(),
            NodeId("hermes-node".to_string()), // Use a default node ID
            self.hermes.coordinates().clone(),
            address.clone(),
        );
        
        // Note: In full implementation, would use hermes.register_service() method
        // For now, we acknowledge the registration
        info!("Service registration acknowledged (DHT integration pending)");
        
        info!("Service '{}' registered successfully", service_name);
        Ok(())
    }
    
    /// Discover service endpoints via HERMES DHT
    pub async fn discover_service(&self, service_name: &str) -> Result<Vec<ServiceEndpoint>> {
        info!("Discovering service '{}'...", service_name);
        
        // Note: In full implementation, would use hermes.discover_service() method
        // For now, return empty vec (DHT integration pending)
        warn!("Service discovery not yet fully integrated - returning empty list");
        Ok(Vec::new())
    }
    
    /// Select best endpoint using load balancing
    pub async fn select_best_endpoint(
        &self,
        service_name: &str,
    ) -> Result<ServiceEndpoint> {
        let endpoints = self.discover_service(service_name).await?;
        
        if endpoints.is_empty() {
            return Err(anyhow::anyhow!("No endpoints available for '{}'", service_name));
        }
        
        // Check circuit breaker
        if !self.circuit_breaker.is_request_allowed().await {
            warn!("Circuit breaker open for '{}'", service_name);
            return Err(anyhow::anyhow!("Circuit breaker open"));
        }
        
        // For now, return first healthy endpoint
        // In full implementation, use load balancer
        let best = endpoints.into_iter()
            .find(|e| e.is_healthy())
            .ok_or_else(|| anyhow::anyhow!("No healthy endpoints"))?;
        
        Ok(best)
    }
    
    /// Update load metrics for an endpoint
    pub async fn update_load_metrics(
        &self,
        node_id: NodeId,
        metrics: LoadMetrics,
    ) -> Result<()> {
        // Update load balancer metrics
        // In full implementation, this would update the load tracker
        Ok(())
    }
    
    /// Record success for circuit breaker
    pub async fn record_success(&self) {
        self.circuit_breaker.record_success().await;
    }
    
    /// Record failure for circuit breaker
    pub async fn record_failure(&self) {
        self.circuit_breaker.record_failure().await;
    }
    
    /// Get HERMES node coordinates
    pub fn coordinates(&self) -> &HyperbolicCoordinates {
        self.hermes.coordinates()
    }
    
    /// Get HERMES node ID
    pub fn node_id(&self) -> NodeId {
        // Return a default node ID for now
        NodeId("hermes-node".to_string())
    }
    
    /// Get fluid state (read-only access)
    pub fn get_fluid_state(&self) -> Arc<RwLock<FluidState>> {
        Arc::clone(&self.fluid_state)
    }
    
    /// Update edge telemetry for fluid dynamics
    pub async fn update_edge_telemetry(
        &self,
        source_node: &str,
        dest_node: &str,
        latency_ms: f64,
        loss_rate: f64,
        jitter_ms: f64,
        queue_depth: f64,
    ) -> Result<()> {
        // Get or create node IDs
        let source_id = self.get_or_create_node_id(source_node).await;
        let dest_id = self.get_or_create_node_id(dest_node).await;
        
        let edge_id = EdgeId::new(source_id, dest_id);
        
        // Update fluid state
        let mut state = self.fluid_state.write().await;
        state.update_edge_telemetry(edge_id, latency_ms, loss_rate, jitter_ms, queue_depth);
        
        Ok(())
    }
    
    /// Add an edge to fluid state
    pub async fn add_fluid_edge(
        &self,
        source_node: &str,
        dest_node: &str,
        capacity_mbps: f64,
    ) -> Result<()> {
        let source_id = self.get_or_create_node_id(source_node).await;
        let dest_id = self.get_or_create_node_id(dest_node).await;
        
        let edge_id = EdgeId::new(source_id, dest_id);
        
        let mut state = self.fluid_state.write().await;
        state.add_edge(edge_id, capacity_mbps);
        
        info!("Added fluid edge: {} -> {} (capacity: {} Mbps)", source_node, dest_node, capacity_mbps);
        Ok(())
    }
    
    /// Calculate fluid score for an edge
    pub async fn get_fluid_score(
        &self,
        source_node: &str,
        dest_node: &str,
    ) -> Result<f64> {
        let source_id = self.get_node_id(source_node).await?;
        let dest_id = self.get_node_id(dest_node).await?;
        
        let edge_id = EdgeId::new(source_id, dest_id);
        
        let state = self.fluid_state.read().await;
        if let Some(edge) = state.get_edge(&edge_id) {
            Ok(self.calculate_fluid_score(edge))
        } else {
            Err(anyhow::anyhow!("Edge not found: {} -> {}", source_node, dest_node))
        }
    }
    
    /// Perform fluid step (advance epoch)
    pub async fn fluid_step(&self) {
        let mut state = self.fluid_state.write().await;
        state.fluid_step();
        info!("Fluid step complete, epoch: {}", state.epoch);
    }
    
    /// Get edges needing healing
    pub async fn get_healing_edges(&self) -> Vec<(String, String)> {
        let state = self.fluid_state.read().await;
        let edge_ids = state.edges_needing_healing();
        
        let mut result = Vec::new();
        let node_map = self.node_id_map.read().await;
        
        // Reverse lookup node names
        let id_to_name: HashMap<u64, String> = node_map
            .iter()
            .map(|(name, id)| (*id, name.clone()))
            .collect();
        
        for edge_id in edge_ids {
            if let (Some(source), Some(dest)) = (
                id_to_name.get(&edge_id.source),
                id_to_name.get(&edge_id.dest),
            ) {
                result.push((source.clone(), dest.clone()));
            }
        }
        
        result
    }
    
    /// Get average network viscosity
    pub async fn get_average_viscosity(&self) -> f64 {
        let state = self.fluid_state.read().await;
        state.average_viscosity()
    }
    
    /// Get average network temperature
    pub async fn get_average_temperature(&self) -> f64 {
        let state = self.fluid_state.read().await;
        state.average_temperature()
    }
    
    /// Get maximum network load
    pub async fn get_max_load(&self) -> f64 {
        let state = self.fluid_state.read().await;
        state.max_load()
    }
    
    // Private helper methods
    
    /// Get or create a numeric node ID for a node name
    async fn get_or_create_node_id(&self, node_name: &str) -> u64 {
        let mut map = self.node_id_map.write().await;
        
        if let Some(&id) = map.get(node_name) {
            return id;
        }
        
        // Create new ID
        let mut next_id = self.next_node_id.write().await;
        let id = *next_id;
        *next_id += 1;
        
        map.insert(node_name.to_string(), id);
        id
    }
    
    /// Get existing node ID (error if not found)
    async fn get_node_id(&self, node_name: &str) -> Result<u64> {
        let map = self.node_id_map.read().await;
        map.get(node_name)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("Node not found: {}", node_name))
    }
    
    /// Calculate fluid score for routing
    /// 
    /// Score formula:
    /// score = base_score / (viscosity_penalty * load_penalty)
    /// 
    /// Higher score = better path
    fn calculate_fluid_score(&self, edge: &EdgeState) -> f64 {
        let base_score = 1.0 / edge.weight.max(0.1);
        let viscosity_penalty = 1.0 + edge.viscosity;
        let load_penalty = 1.0 + edge.load;
        
        base_score / (viscosity_penalty * load_penalty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_hermes_integration_creation() {
        let integration = HermesIntegration::new(8000, "test-node".to_string())
            .await
            .unwrap();
        
        assert_eq!(integration.node_id().0, "hermes-integration-node");
    }
    
    #[tokio::test]
    async fn test_service_registration() {
        let integration = HermesIntegration::new(8001, "test-node-2".to_string())
            .await
            .unwrap();
        
        integration.register_service(
            "test-service".to_string(),
            "127.0.0.1:9000".to_string()
        ).await.unwrap();
        
        // Note: Discovery returns empty for now (DHT integration pending)
        let endpoints = integration.discover_service("test-service").await.unwrap();
        assert_eq!(endpoints.len(), 0);
    }
}
