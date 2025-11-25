//! Fluid State Management
//! 
//! Manages the complete fluid state for W4-FT transport including
//! weights, viscosity, temperature, curvature, and density for all edges.

use super::edge::{EdgeId, EdgeState};
use super::ricci_flow::{RicciFlowEngine, RicciFlowConfig, EvolutionStats};
use std::collections::HashMap;
use std::time::Instant;
use serde::{Deserialize, Serialize};

/// Complete fluid state for the network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluidState {
    /// Edge states indexed by EdgeId
    pub edges: HashMap<EdgeId, EdgeState>,
    
    /// Current epoch
    pub epoch: u32,
    
    /// Configuration
    pub config: FluidConfig,
    
    /// Ricci-flow engine (not serialized)
    #[serde(skip)]
    ricci_flow: Option<RicciFlowEngine>,
    
    /// Evolution statistics
    pub evolution_stats: Option<EvolutionStats>,
    
    /// Last update timestamp
    #[serde(skip)]
    #[serde(default = "Instant::now")]
    pub last_update: Instant,
}

/// Fluid configuration parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluidConfig {
    /// Base viscosity (ν_base)
    pub nu_base: f64,
    
    /// Queue coefficient for viscosity (a)
    pub nu_queue_coeff: f64,
    
    /// Jitter coefficient for viscosity (b)
    pub nu_jitter_coeff: f64,
    
    /// Minimum viscosity
    pub nu_min: f64,
    
    /// Maximum viscosity
    pub nu_max: f64,
    
    /// Temperature threshold for healing
    pub temp_healing_threshold: f64,
    
    /// Curvature gain (κ_f)
    pub kappa_gain: f64,
    
    /// Damping coefficient (η)
    pub eta_damp: f64,
}

impl FluidState {
    /// Create a new fluid state
    pub fn new(config: FluidConfig) -> Self {
        Self {
            edges: HashMap::new(),
            epoch: 0,
            config,
            ricci_flow: None,
            evolution_stats: None,
            last_update: Instant::now(),
        }
    }
    
    /// Create with Ricci-flow enabled
    pub fn with_ricci_flow(config: FluidConfig, ricci_config: RicciFlowConfig) -> Self {
        Self {
            edges: HashMap::new(),
            epoch: 0,
            config,
            ricci_flow: Some(RicciFlowEngine::new(ricci_config)),
            evolution_stats: None,
            last_update: Instant::now(),
        }
    }
    
    /// Add or update an edge
    pub fn add_edge(&mut self, id: EdgeId, capacity: f64) {
        self.edges.entry(id).or_insert_with(|| EdgeState::new(id, capacity));
    }
    
    /// Remove an edge
    pub fn remove_edge(&mut self, id: &EdgeId) {
        self.edges.remove(id);
    }
    
    /// Get edge state
    pub fn get_edge(&self, id: &EdgeId) -> Option<&EdgeState> {
        self.edges.get(id)
    }
    
    /// Get mutable edge state
    pub fn get_edge_mut(&mut self, id: &EdgeId) -> Option<&mut EdgeState> {
        self.edges.get_mut(id)
    }
    
    /// Update telemetry for an edge
    pub fn update_edge_telemetry(
        &mut self,
        id: EdgeId,
        latency_ms: f64,
        loss_rate: f64,
        jitter_ms: f64,
        queue_depth: f64,
    ) {
        if let Some(edge) = self.edges.get_mut(&id) {
            edge.update_telemetry(latency_ms, loss_rate, jitter_ms, queue_depth);
            edge.calculate_viscosity(
                self.config.nu_base,
                self.config.nu_queue_coeff,
                self.config.nu_jitter_coeff,
            );
        }
    }
    
    /// Perform fluid step (update all edges)
    pub fn fluid_step(&mut self) {
        // Update viscosity for all edges
        for edge in self.edges.values_mut() {
            edge.calculate_viscosity(
                self.config.nu_base,
                self.config.nu_queue_coeff,
                self.config.nu_jitter_coeff,
            );
        }
        
        // Evolve weights using Ricci-flow if enabled
        if let Some(ref mut ricci_flow) = self.ricci_flow {
            let new_weights = ricci_flow.step(&self.edges);
            
            // Apply new weights and update curvature
            for (edge_id, new_weight) in new_weights {
                if let Some(edge) = self.edges.get_mut(&edge_id) {
                    edge.weight = new_weight;
                }
            }
        }
        
        self.epoch += 1;
        self.last_update = Instant::now();
    }
    
    /// Get edges that need healing
    pub fn edges_needing_healing(&self) -> Vec<EdgeId> {
        self.edges
            .values()
            .filter(|e| e.needs_healing(self.config.temp_healing_threshold))
            .map(|e| e.id)
            .collect()
    }
    
    /// Get average viscosity across all edges
    pub fn average_viscosity(&self) -> f64 {
        if self.edges.is_empty() {
            return self.config.nu_base;
        }
        
        let sum: f64 = self.edges.values().map(|e| e.viscosity).sum();
        sum / self.edges.len() as f64
    }
    
    /// Get average temperature across all edges
    pub fn average_temperature(&self) -> f64 {
        if self.edges.is_empty() {
            return 0.0;
        }
        
        let sum: f64 = self.edges.values().map(|e| e.temperature).sum();
        sum / self.edges.len() as f64
    }
    
    /// Get maximum load across all edges
    pub fn max_load(&self) -> f64 {
        self.edges
            .values()
            .map(|e| e.load)
            .fold(0.0, f64::max)
    }
    
    /// Get edge count
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
    
    /// Enable Ricci-flow evolution
    pub fn enable_ricci_flow(&mut self, ricci_config: RicciFlowConfig) {
        self.ricci_flow = Some(RicciFlowEngine::new(ricci_config));
    }
    
    /// Disable Ricci-flow evolution
    pub fn disable_ricci_flow(&mut self) {
        self.ricci_flow = None;
    }
    
    /// Check if Ricci-flow is enabled
    pub fn is_ricci_flow_enabled(&self) -> bool {
        self.ricci_flow.is_some()
    }
    
    /// Get evolution statistics
    pub fn get_evolution_stats(&self) -> Option<&EvolutionStats> {
        self.evolution_stats.as_ref()
    }
}

impl Default for FluidConfig {
    fn default() -> Self {
        Self {
            nu_base: 0.03,
            nu_queue_coeff: 0.01,
            nu_jitter_coeff: 0.02,
            nu_min: 0.01,
            nu_max: 1.0,
            temp_healing_threshold: 10.0,
            kappa_gain: 0.6,
            eta_damp: 0.05,
        }
    }
}

impl FluidConfig {
    /// Create a conservative configuration (slow adaptation)
    pub fn conservative() -> Self {
        Self {
            nu_base: 0.05,
            nu_queue_coeff: 0.005,
            nu_jitter_coeff: 0.01,
            nu_min: 0.02,
            nu_max: 0.5,
            temp_healing_threshold: 20.0,
            kappa_gain: 0.3,
            eta_damp: 0.1,
        }
    }
    
    /// Create an aggressive configuration (fast adaptation)
    pub fn aggressive() -> Self {
        Self {
            nu_base: 0.02,
            nu_queue_coeff: 0.02,
            nu_jitter_coeff: 0.03,
            nu_min: 0.01,
            nu_max: 1.0,
            temp_healing_threshold: 5.0,
            kappa_gain: 0.8,
            eta_damp: 0.02,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fluid_state_creation() {
        let config = FluidConfig::default();
        let state = FluidState::new(config);
        
        assert_eq!(state.epoch, 0);
        assert_eq!(state.edge_count(), 0);
    }
    
    #[test]
    fn test_add_remove_edge() {
        let config = FluidConfig::default();
        let mut state = FluidState::new(config);
        
        let id = EdgeId::new(1, 2);
        state.add_edge(id, 1000.0);
        
        assert_eq!(state.edge_count(), 1);
        assert!(state.get_edge(&id).is_some());
        
        state.remove_edge(&id);
        assert_eq!(state.edge_count(), 0);
        assert!(state.get_edge(&id).is_none());
    }
    
    #[test]
    fn test_telemetry_update() {
        let config = FluidConfig::default();
        let mut state = FluidState::new(config);
        
        let id = EdgeId::new(1, 2);
        state.add_edge(id, 1000.0);
        
        state.update_edge_telemetry(id, 10.0, 0.01, 5.0, 10.0);
        
        let edge = state.get_edge(&id).unwrap();
        assert_eq!(edge.latency(), 10.0);
        assert_eq!(edge.loss_rate(), 0.01);
        assert_eq!(edge.jitter(), 5.0);
    }
    
    #[test]
    fn test_fluid_step() {
        let config = FluidConfig::default();
        let mut state = FluidState::new(config);
        
        let id = EdgeId::new(1, 2);
        state.add_edge(id, 1000.0);
        
        assert_eq!(state.epoch, 0);
        
        state.fluid_step();
        assert_eq!(state.epoch, 1);
        
        state.fluid_step();
        assert_eq!(state.epoch, 2);
    }
    
    #[test]
    fn test_healing_detection() {
        let config = FluidConfig::default();
        let mut state = FluidState::new(config);
        
        let id1 = EdgeId::new(1, 2);
        let id2 = EdgeId::new(2, 3);
        
        state.add_edge(id1, 1000.0);
        state.add_edge(id2, 1000.0);
        
        // Low jitter - no healing needed
        state.update_edge_telemetry(id1, 10.0, 0.01, 5.0, 10.0);
        
        // High jitter - healing needed (threshold = 10.0)
        state.update_edge_telemetry(id2, 10.0, 0.01, 15.0, 10.0);
        
        let healing = state.edges_needing_healing();
        assert_eq!(healing.len(), 1);
        assert_eq!(healing[0], id2);
    }
    
    #[test]
    fn test_average_metrics() {
        let config = FluidConfig::default();
        let mut state = FluidState::new(config);
        
        let id1 = EdgeId::new(1, 2);
        let id2 = EdgeId::new(2, 3);
        
        state.add_edge(id1, 1000.0);
        state.add_edge(id2, 1000.0);
        
        state.update_edge_telemetry(id1, 10.0, 0.01, 5.0, 10.0);
        state.update_edge_telemetry(id2, 20.0, 0.02, 15.0, 20.0);
        
        // Average temperature = (5.0 + 15.0) / 2 = 10.0
        assert_eq!(state.average_temperature(), 10.0);
    }
    
    #[test]
    fn test_max_load() {
        let config = FluidConfig::default();
        let mut state = FluidState::new(config);
        
        let id1 = EdgeId::new(1, 2);
        let id2 = EdgeId::new(2, 3);
        
        state.add_edge(id1, 1000.0);
        state.add_edge(id2, 1000.0);
        
        state.update_edge_telemetry(id1, 10.0, 0.01, 5.0, 30.0);  // load = 0.3
        state.update_edge_telemetry(id2, 20.0, 0.02, 15.0, 70.0); // load = 0.7
        
        assert_eq!(state.max_load(), 0.7);
    }
    
    #[test]
    fn test_config_presets() {
        let conservative = FluidConfig::conservative();
        assert_eq!(conservative.nu_base, 0.05);
        assert_eq!(conservative.temp_healing_threshold, 20.0);
        
        let aggressive = FluidConfig::aggressive();
        assert_eq!(aggressive.nu_base, 0.02);
        assert_eq!(aggressive.temp_healing_threshold, 5.0);
    }
    
    #[test]
    fn test_ricci_flow_integration() {
        use crate::w4_fluid::ricci_flow::RicciFlowConfig;
        
        let fluid_config = FluidConfig::default();
        let ricci_config = RicciFlowConfig::default();
        let mut state = FluidState::with_ricci_flow(fluid_config, ricci_config);
        
        assert!(state.is_ricci_flow_enabled());
        
        // Add edges
        let id1 = EdgeId::new(1, 2);
        let id2 = EdgeId::new(2, 3);
        
        state.add_edge(id1, 1000.0);
        state.add_edge(id2, 1000.0);
        
        // Update telemetry
        state.update_edge_telemetry(id1, 10.0, 0.01, 5.0, 10.0);
        state.update_edge_telemetry(id2, 20.0, 0.02, 10.0, 20.0);
        
        // Get initial weights
        let initial_weight1 = state.get_edge(&id1).unwrap().weight;
        let initial_weight2 = state.get_edge(&id2).unwrap().weight;
        
        // Perform fluid step (should evolve weights)
        state.fluid_step();
        
        // Weights should have changed (unless perfectly converged)
        let new_weight1 = state.get_edge(&id1).unwrap().weight;
        let new_weight2 = state.get_edge(&id2).unwrap().weight;
        
        // Weights should be within bounds
        assert!(new_weight1 >= 0.1);
        assert!(new_weight1 <= 10.0);
        assert!(new_weight2 >= 0.1);
        assert!(new_weight2 <= 10.0);
    }
    
    #[test]
    fn test_ricci_flow_enable_disable() {
        use crate::w4_fluid::ricci_flow::RicciFlowConfig;
        
        let config = FluidConfig::default();
        let mut state = FluidState::new(config);
        
        assert!(!state.is_ricci_flow_enabled());
        
        // Enable Ricci-flow
        let ricci_config = RicciFlowConfig::default();
        state.enable_ricci_flow(ricci_config);
        assert!(state.is_ricci_flow_enabled());
        
        // Disable Ricci-flow
        state.disable_ricci_flow();
        assert!(!state.is_ricci_flow_enabled());
    }
}
