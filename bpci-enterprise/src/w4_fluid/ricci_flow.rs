//! Ricci-Flow Evolution for W4-FT
//! 
//! Implements Ricci-flow evolution equations for adaptive weight updates.
//! 
//! # Ricci-Flow Equation
//! 
//! The basic Ricci-flow equation evolves edge weights based on curvature:
//! 
//! ```text
//! dw/dt = -2κ(e)
//! ```
//! 
//! # Enhanced W4-FT Equation
//! 
//! The full W4-FT evolution includes additional terms:
//! 
//! ```text
//! w' = w - 2κ + κ_f·φ(ρ) + ν·Δw - η·w
//! ```
//! 
//! Where:
//! - κ: Ricci curvature
//! - κ_f: Curvature gain
//! - φ(ρ): Density-dependent term
//! - ν: Viscosity (diffusion)
//! - Δw: Laplacian (smoothing)
//! - η: Damping coefficient

use super::curvature::{CurvatureGraph, FormanCurvature, OllivierRicciCurvature};
use super::edge::{EdgeId, EdgeState};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Ricci-flow configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RicciFlowConfig {
    /// Time step size (dt)
    pub dt: f64,
    
    /// Curvature gain (κ_f)
    pub kappa_gain: f64,
    
    /// Damping coefficient (η)
    pub eta_damp: f64,
    
    /// Minimum weight
    pub w_min: f64,
    
    /// Maximum weight
    pub w_max: f64,
    
    /// Use Forman curvature (vs Ollivier-Ricci)
    pub use_forman: bool,
    
    /// Ollivier-Ricci alpha parameter
    pub or_alpha: f64,
    
    /// Enable density-dependent term
    pub enable_density: bool,
    
    /// Enable viscosity diffusion
    pub enable_diffusion: bool,
}

impl Default for RicciFlowConfig {
    fn default() -> Self {
        Self {
            dt: 0.01,
            kappa_gain: 0.6,
            eta_damp: 0.05,
            w_min: 0.1,
            w_max: 10.0,
            use_forman: true,
            or_alpha: 0.5,
            enable_density: true,
            enable_diffusion: true,
        }
    }
}

impl RicciFlowConfig {
    /// Conservative configuration (slow evolution)
    pub fn conservative() -> Self {
        Self {
            dt: 0.005,
            kappa_gain: 0.3,
            eta_damp: 0.1,
            w_min: 0.5,
            w_max: 5.0,
            use_forman: true,
            or_alpha: 0.5,
            enable_density: true,
            enable_diffusion: true,
        }
    }
    
    /// Aggressive configuration (fast evolution)
    pub fn aggressive() -> Self {
        Self {
            dt: 0.02,
            kappa_gain: 0.8,
            eta_damp: 0.02,
            w_min: 0.1,
            w_max: 20.0,
            use_forman: true,
            or_alpha: 0.5,
            enable_density: true,
            enable_diffusion: true,
        }
    }
}

/// Ricci-flow evolution engine
#[derive(Debug, Clone)]
pub struct RicciFlowEngine {
    config: RicciFlowConfig,
    epoch: u64,
}

impl RicciFlowEngine {
    /// Create a new Ricci-flow engine
    pub fn new(config: RicciFlowConfig) -> Self {
        Self { config, epoch: 0 }
    }
    
    /// Perform one Ricci-flow step
    /// 
    /// Updates edge weights based on curvature and returns new weights
    pub fn step(&mut self, edges: &HashMap<EdgeId, EdgeState>) -> HashMap<EdgeId, f64> {
        // Build curvature graph
        let mut graph = CurvatureGraph::new();
        for edge in edges.values() {
            graph.add_edge(edge.clone());
        }
        
        // Calculate curvatures
        let curvatures = if self.config.use_forman {
            self.calculate_forman_curvatures(&graph, edges)
        } else {
            self.calculate_or_curvatures(&graph, edges)
        };
        
        // Evolve weights
        let mut new_weights = HashMap::new();
        
        for (edge_id, edge) in edges {
            let kappa = curvatures.get(edge_id).copied().unwrap_or(0.0);
            
            // Basic Ricci-flow term: -2κ
            let mut dw = -2.0 * kappa;
            
            // Curvature gain term: κ_f·φ(ρ)
            if self.config.enable_density {
                let density_term = self.config.kappa_gain * self.density_function(edge.density);
                dw += density_term;
            }
            
            // Viscosity diffusion term: ν·Δw
            if self.config.enable_diffusion {
                let laplacian = self.calculate_laplacian(&graph, edge_id, edges);
                dw += edge.viscosity * laplacian;
            }
            
            // Damping term: -η·w
            dw -= self.config.eta_damp * edge.weight;
            
            // Update weight: w' = w + dt·dw/dt
            let new_weight = edge.weight + self.config.dt * dw;
            
            // Clamp to bounds
            let clamped_weight = new_weight.max(self.config.w_min).min(self.config.w_max);
            
            new_weights.insert(*edge_id, clamped_weight);
        }
        
        self.epoch += 1;
        new_weights
    }
    
    /// Calculate Forman curvatures for all edges
    fn calculate_forman_curvatures(
        &self,
        graph: &CurvatureGraph,
        edges: &HashMap<EdgeId, EdgeState>,
    ) -> HashMap<EdgeId, f64> {
        let forman = FormanCurvature::new(graph.clone());
        
        edges
            .keys()
            .map(|edge_id| (*edge_id, forman.calculate(edge_id)))
            .collect()
    }
    
    /// Calculate Ollivier-Ricci curvatures for all edges
    fn calculate_or_curvatures(
        &self,
        graph: &CurvatureGraph,
        edges: &HashMap<EdgeId, EdgeState>,
    ) -> HashMap<EdgeId, f64> {
        let or_curv = OllivierRicciCurvature::new(graph.clone(), self.config.or_alpha);
        
        edges
            .keys()
            .map(|edge_id| (*edge_id, or_curv.calculate(edge_id)))
            .collect()
    }
    
    /// Density-dependent function φ(ρ)
    /// 
    /// Returns a value that increases with density (congestion)
    fn density_function(&self, density: f64) -> f64 {
        // Sigmoid-like function: φ(ρ) = 2/(1 + e^(-5(ρ-0.5))) - 1
        // Maps [0,1] to approximately [-1,1]
        2.0 / (1.0 + (-5.0 * (density - 0.5)).exp()) - 1.0
    }
    
    /// Calculate Laplacian (discrete) for an edge
    /// 
    /// Δw = Σ(w_neighbor - w) / degree
    fn calculate_laplacian(
        &self,
        graph: &CurvatureGraph,
        edge_id: &EdgeId,
        edges: &HashMap<EdgeId, EdgeState>,
    ) -> f64 {
        let edge = match edges.get(edge_id) {
            Some(e) => e,
            None => return 0.0,
        };
        
        // Get incident edges
        let source_edges = graph.incident_edges(edge_id.source);
        let dest_edges = graph.incident_edges(edge_id.dest);
        
        let mut neighbor_weights = Vec::new();
        
        for neighbor_id in source_edges.iter().chain(dest_edges.iter()) {
            if neighbor_id != edge_id {
                if let Some(neighbor) = edges.get(neighbor_id) {
                    neighbor_weights.push(neighbor.weight);
                }
            }
        }
        
        if neighbor_weights.is_empty() {
            return 0.0;
        }
        
        // Laplacian: average of (neighbor - self)
        let avg_neighbor: f64 = neighbor_weights.iter().sum::<f64>() / neighbor_weights.len() as f64;
        avg_neighbor - edge.weight
    }
    
    /// Get current epoch
    pub fn epoch(&self) -> u64 {
        self.epoch
    }
    
    /// Get configuration
    pub fn config(&self) -> &RicciFlowConfig {
        &self.config
    }
}

/// Evolution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionStats {
    pub epoch: u64,
    pub avg_weight: f64,
    pub avg_curvature: f64,
    pub weight_change: f64,
    pub converged: bool,
}

impl EvolutionStats {
    /// Calculate statistics from edges and curvatures
    pub fn calculate(
        epoch: u64,
        edges: &HashMap<EdgeId, EdgeState>,
        new_weights: &HashMap<EdgeId, f64>,
        curvatures: &HashMap<EdgeId, f64>,
    ) -> Self {
        let n = edges.len() as f64;
        
        if n == 0.0 {
            return Self {
                epoch,
                avg_weight: 0.0,
                avg_curvature: 0.0,
                weight_change: 0.0,
                converged: true,
            };
        }
        
        // Average weight
        let avg_weight: f64 = new_weights.values().sum::<f64>() / n;
        
        // Average curvature
        let avg_curvature: f64 = curvatures.values().sum::<f64>() / n;
        
        // Weight change
        let mut total_change = 0.0;
        for (edge_id, new_weight) in new_weights {
            if let Some(edge) = edges.get(edge_id) {
                total_change += (new_weight - edge.weight).abs();
            }
        }
        let weight_change = total_change / n;
        
        // Convergence check (weight change < 0.001)
        let converged = weight_change < 0.001;
        
        Self {
            epoch,
            avg_weight,
            avg_curvature,
            weight_change,
            converged,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::edge::EdgeTelemetry;
    
    fn create_test_edge(source: u64, dest: u64, weight: f64, density: f64) -> EdgeState {
        let id = EdgeId::new(source, dest);
        EdgeState {
            id,
            weight,
            viscosity: 0.03,
            temperature: 0.0,
            curvature: 0.0,
            density,
            load: 0.0,
            telemetry: EdgeTelemetry::new(1000.0),
            last_update: None,
        }
    }
    
    #[test]
    fn test_ricci_flow_creation() {
        let config = RicciFlowConfig::default();
        let engine = RicciFlowEngine::new(config);
        
        assert_eq!(engine.epoch(), 0);
    }
    
    #[test]
    fn test_ricci_flow_step() {
        let config = RicciFlowConfig::default();
        let mut engine = RicciFlowEngine::new(config);
        
        let mut edges = HashMap::new();
        edges.insert(EdgeId::new(1, 2), create_test_edge(1, 2, 1.0, 0.5));
        edges.insert(EdgeId::new(2, 3), create_test_edge(2, 3, 1.0, 0.5));
        
        let new_weights = engine.step(&edges);
        
        assert_eq!(new_weights.len(), 2);
        assert_eq!(engine.epoch(), 1);
    }
    
    #[test]
    fn test_weight_bounds() {
        let config = RicciFlowConfig {
            w_min: 0.5,
            w_max: 2.0,
            ..Default::default()
        };
        let mut engine = RicciFlowEngine::new(config);
        
        let mut edges = HashMap::new();
        edges.insert(EdgeId::new(1, 2), create_test_edge(1, 2, 0.1, 0.5));
        
        let new_weights = engine.step(&edges);
        
        // Weight should be clamped to bounds
        for weight in new_weights.values() {
            assert!(*weight >= 0.5);
            assert!(*weight <= 2.0);
        }
    }
    
    #[test]
    fn test_density_function() {
        let config = RicciFlowConfig::default();
        let engine = RicciFlowEngine::new(config);
        
        // Low density should give negative value
        let low = engine.density_function(0.0);
        assert!(low < 0.0);
        
        // Medium density should be near zero
        let med = engine.density_function(0.5);
        assert!(med.abs() < 0.1);
        
        // High density should give positive value
        let high = engine.density_function(1.0);
        assert!(high > 0.0);
    }
    
    #[test]
    fn test_config_presets() {
        let conservative = RicciFlowConfig::conservative();
        assert_eq!(conservative.dt, 0.005);
        assert_eq!(conservative.kappa_gain, 0.3);
        
        let aggressive = RicciFlowConfig::aggressive();
        assert_eq!(aggressive.dt, 0.02);
        assert_eq!(aggressive.kappa_gain, 0.8);
    }
    
    #[test]
    fn test_evolution_stats() {
        let mut edges = HashMap::new();
        edges.insert(EdgeId::new(1, 2), create_test_edge(1, 2, 1.0, 0.5));
        edges.insert(EdgeId::new(2, 3), create_test_edge(2, 3, 1.5, 0.5));
        
        let mut new_weights = HashMap::new();
        new_weights.insert(EdgeId::new(1, 2), 1.1);
        new_weights.insert(EdgeId::new(2, 3), 1.6);
        
        let mut curvatures = HashMap::new();
        curvatures.insert(EdgeId::new(1, 2), 0.5);
        curvatures.insert(EdgeId::new(2, 3), -0.3);
        
        let stats = EvolutionStats::calculate(1, &edges, &new_weights, &curvatures);
        
        assert_eq!(stats.epoch, 1);
        assert!((stats.avg_weight - 1.35).abs() < 0.01);
        assert!((stats.avg_curvature - 0.1).abs() < 0.01);
        assert!((stats.weight_change - 0.1).abs() < 0.01);
    }
    
    #[test]
    fn test_convergence_detection() {
        let mut edges = HashMap::new();
        edges.insert(EdgeId::new(1, 2), create_test_edge(1, 2, 1.0, 0.5));
        
        let mut new_weights = HashMap::new();
        
        // Large change - not converged
        new_weights.insert(EdgeId::new(1, 2), 1.5);
        let stats1 = EvolutionStats::calculate(1, &edges, &new_weights, &HashMap::new());
        assert!(!stats1.converged);
        
        // Small change - converged
        new_weights.insert(EdgeId::new(1, 2), 1.0005);
        let stats2 = EvolutionStats::calculate(2, &edges, &new_weights, &HashMap::new());
        assert!(stats2.converged);
    }
}
