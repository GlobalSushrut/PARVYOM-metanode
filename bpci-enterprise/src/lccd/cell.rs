//! LCCD Cell Formation
//! 
//! Implements Living Cellular Consensus Division (LCCD) cell formation
//! based on network topology and curvature.
//! 
//! # Cell Formation Algorithm
//! 
//! 1. Calculate curvature for all edges
//! 2. Identify negative curvature edges (boundaries)
//! 3. Group nodes by connectivity (within boundaries)
//! 4. Form cells from connected components
//! 5. Validate cell properties (size, diversity, health)
//! 
//! # Curvature-Based Boundaries
//! 
//! - **Positive curvature**: Well-connected region (inside cell)
//! - **Negative curvature**: Bottleneck (cell boundary)
//! - **Zero curvature**: Neutral (edge case)

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use crate::w4_fluid::{EdgeId, EdgeState, CurvatureGraph, FormanCurvature};

/// Cell identifier
pub type CellId = u64;

/// Node identifier
pub type NodeId = u64;

/// LCCD Cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LccdCell {
    /// Unique cell identifier
    pub cell_id: CellId,
    
    /// Member nodes in this cell
    pub members: Vec<NodeId>,
    
    /// Boundary edges (negative curvature)
    pub boundary_edges: Vec<EdgeId>,
    
    /// Curvature profile of the cell
    pub curvature_profile: CurvatureProfile,
    
    /// Cell health metrics
    pub health: CellHealth,
    
    /// Current cell state
    pub state: CellState,
}

/// Curvature profile for a cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurvatureProfile {
    /// Average curvature inside cell
    pub avg_internal_curvature: f64,
    
    /// Average curvature on boundary
    pub avg_boundary_curvature: f64,
    
    /// Minimum curvature (most negative)
    pub min_curvature: f64,
    
    /// Maximum curvature (most positive)
    pub max_curvature: f64,
}

/// Cell health metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellHealth {
    /// Overall health score (0.0 - 1.0)
    pub score: f64,
    
    /// Size health (optimal size = 1.0)
    pub size_health: f64,
    
    /// Connectivity health (well-connected = 1.0)
    pub connectivity_health: f64,
    
    /// Boundary health (clear boundaries = 1.0)
    pub boundary_health: f64,
}

/// Cell state in lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellState {
    /// Cell is forming
    Forming,
    
    /// Cell is active and healthy
    Active,
    
    /// Cell is growing (absorbing nodes)
    Growing,
    
    /// Cell is dividing (too large)
    Dividing,
    
    /// Cell is merging with another
    Merging,
    
    /// Cell is unhealthy
    Unhealthy,
    
    /// Cell is dissolving
    Dissolving,
}

/// Cell formation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellFormationConfig {
    /// Curvature threshold for boundaries (edges below this are boundaries)
    pub boundary_threshold: f64,
    
    /// Minimum cell size (nodes)
    pub min_cell_size: usize,
    
    /// Maximum cell size (nodes)
    pub max_cell_size: usize,
    
    /// Optimal cell size (nodes)
    pub optimal_cell_size: usize,
    
    /// Minimum boundary clarity (difference between internal and boundary curvature)
    pub min_boundary_clarity: f64,
}

impl Default for CellFormationConfig {
    fn default() -> Self {
        Self {
            boundary_threshold: -0.1,
            min_cell_size: 3,
            max_cell_size: 10,
            optimal_cell_size: 5,
            min_boundary_clarity: 0.2,
        }
    }
}

/// Cell formation engine
pub struct CellFormationEngine {
    config: CellFormationConfig,
    next_cell_id: CellId,
}

impl CellFormationEngine {
    /// Create a new cell formation engine
    pub fn new(config: CellFormationConfig) -> Self {
        Self {
            config,
            next_cell_id: 1,
        }
    }
    
    /// Form cells from edges based on curvature
    pub fn form_cells(&mut self, edges: &HashMap<EdgeId, EdgeState>) -> Vec<LccdCell> {
        // Build curvature graph
        let mut graph = CurvatureGraph::new();
        for edge in edges.values() {
            graph.add_edge(edge.clone());
        }
        
        // Calculate curvatures
        let forman = FormanCurvature::new(graph.clone());
        let mut curvatures = HashMap::new();
        for edge_id in edges.keys() {
            let curvature = forman.calculate(edge_id);
            curvatures.insert(*edge_id, curvature);
        }
        
        // Identify boundary edges (negative curvature)
        let boundary_edges: HashSet<EdgeId> = curvatures
            .iter()
            .filter(|(_, &curv)| curv < self.config.boundary_threshold)
            .map(|(id, _)| *id)
            .collect();
        
        // Build adjacency without boundary edges
        let adjacency = self.build_internal_adjacency(edges, &boundary_edges);
        
        // Find connected components (cells)
        let components = self.find_connected_components(&adjacency);
        
        // Create cells from components
        let mut cells = Vec::new();
        for component in components {
            if component.len() >= self.config.min_cell_size {
                let cell = self.create_cell(component, edges, &curvatures, &boundary_edges);
                cells.push(cell);
            }
        }
        
        cells
    }
    
    /// Build adjacency list excluding boundary edges
    fn build_internal_adjacency(
        &self,
        edges: &HashMap<EdgeId, EdgeState>,
        boundary_edges: &HashSet<EdgeId>,
    ) -> HashMap<NodeId, Vec<NodeId>> {
        let mut adjacency: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        
        for (edge_id, _) in edges {
            // Skip boundary edges
            if boundary_edges.contains(edge_id) {
                continue;
            }
            
            // Add bidirectional connection
            adjacency
                .entry(edge_id.source)
                .or_insert_with(Vec::new)
                .push(edge_id.dest);
            
            adjacency
                .entry(edge_id.dest)
                .or_insert_with(Vec::new)
                .push(edge_id.source);
        }
        
        adjacency
    }
    
    /// Find connected components using DFS
    fn find_connected_components(
        &self,
        adjacency: &HashMap<NodeId, Vec<NodeId>>,
    ) -> Vec<Vec<NodeId>> {
        let mut visited = HashSet::new();
        let mut components = Vec::new();
        
        // Get all nodes
        let mut all_nodes: Vec<NodeId> = adjacency.keys().copied().collect();
        all_nodes.sort();
        
        for &node in &all_nodes {
            if !visited.contains(&node) {
                let mut component = Vec::new();
                self.dfs(node, adjacency, &mut visited, &mut component);
                components.push(component);
            }
        }
        
        components
    }
    
    /// Depth-first search to find component
    fn dfs(
        &self,
        node: NodeId,
        adjacency: &HashMap<NodeId, Vec<NodeId>>,
        visited: &mut HashSet<NodeId>,
        component: &mut Vec<NodeId>,
    ) {
        visited.insert(node);
        component.push(node);
        
        if let Some(neighbors) = adjacency.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    self.dfs(neighbor, adjacency, visited, component);
                }
            }
        }
    }
    
    /// Create a cell from a component
    fn create_cell(
        &mut self,
        members: Vec<NodeId>,
        edges: &HashMap<EdgeId, EdgeState>,
        curvatures: &HashMap<EdgeId, f64>,
        boundary_edges: &HashSet<EdgeId>,
    ) -> LccdCell {
        let cell_id = self.next_cell_id;
        self.next_cell_id += 1;
        
        let member_set: HashSet<NodeId> = members.iter().copied().collect();
        
        // Find boundary edges for this cell
        let cell_boundary_edges: Vec<EdgeId> = boundary_edges
            .iter()
            .filter(|edge_id| {
                member_set.contains(&edge_id.source) || member_set.contains(&edge_id.dest)
            })
            .copied()
            .collect();
        
        // Calculate curvature profile
        let curvature_profile = self.calculate_curvature_profile(
            &members,
            &cell_boundary_edges,
            edges,
            curvatures,
        );
        
        // Calculate health
        let health = self.calculate_health(&members, &curvature_profile);
        
        // Determine initial state
        let state = if health.score > 0.8 {
            CellState::Active
        } else if health.score > 0.5 {
            CellState::Forming
        } else {
            CellState::Unhealthy
        };
        
        LccdCell {
            cell_id,
            members,
            boundary_edges: cell_boundary_edges,
            curvature_profile,
            health,
            state,
        }
    }
    
    /// Calculate curvature profile for a cell
    fn calculate_curvature_profile(
        &self,
        members: &[NodeId],
        boundary_edges: &[EdgeId],
        edges: &HashMap<EdgeId, EdgeState>,
        curvatures: &HashMap<EdgeId, f64>,
    ) -> CurvatureProfile {
        let member_set: HashSet<NodeId> = members.iter().copied().collect();
        
        // Internal edges (both endpoints in cell)
        let internal_curvatures: Vec<f64> = edges
            .keys()
            .filter(|edge_id| {
                member_set.contains(&edge_id.source) && member_set.contains(&edge_id.dest)
            })
            .filter_map(|edge_id| curvatures.get(edge_id).copied())
            .collect();
        
        // Boundary curvatures
        let boundary_curvatures: Vec<f64> = boundary_edges
            .iter()
            .filter_map(|edge_id| curvatures.get(edge_id).copied())
            .collect();
        
        let avg_internal = if internal_curvatures.is_empty() {
            0.0
        } else {
            internal_curvatures.iter().sum::<f64>() / internal_curvatures.len() as f64
        };
        
        let avg_boundary = if boundary_curvatures.is_empty() {
            0.0
        } else {
            boundary_curvatures.iter().sum::<f64>() / boundary_curvatures.len() as f64
        };
        
        let all_curvatures: Vec<f64> = internal_curvatures
            .iter()
            .chain(boundary_curvatures.iter())
            .copied()
            .collect();
        
        let min_curv = all_curvatures
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min);
        
        let max_curv = all_curvatures
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        
        CurvatureProfile {
            avg_internal_curvature: avg_internal,
            avg_boundary_curvature: avg_boundary,
            min_curvature: if min_curv.is_finite() { min_curv } else { 0.0 },
            max_curvature: if max_curv.is_finite() { max_curv } else { 0.0 },
        }
    }
    
    /// Calculate cell health
    fn calculate_health(
        &self,
        members: &[NodeId],
        profile: &CurvatureProfile,
    ) -> CellHealth {
        // Size health (optimal = 1.0, too small or large = lower)
        let size = members.len();
        let size_health = if size == self.config.optimal_cell_size {
            1.0
        } else if size < self.config.min_cell_size {
            0.3
        } else if size > self.config.max_cell_size {
            0.5
        } else {
            let distance = (size as f64 - self.config.optimal_cell_size as f64).abs();
            let max_distance = self.config.max_cell_size as f64 - self.config.optimal_cell_size as f64;
            1.0 - (distance / max_distance) * 0.5
        };
        
        // Connectivity health (positive internal curvature = good)
        let connectivity_health = (profile.avg_internal_curvature + 1.0) / 2.0;
        let connectivity_health = connectivity_health.max(0.0).min(1.0);
        
        // Boundary health (clear boundary = good)
        let boundary_clarity = profile.avg_internal_curvature - profile.avg_boundary_curvature;
        let boundary_health = if boundary_clarity >= self.config.min_boundary_clarity {
            1.0
        } else {
            (boundary_clarity / self.config.min_boundary_clarity).max(0.0)
        };
        
        // Overall score (weighted average)
        let score = 0.4 * size_health + 0.3 * connectivity_health + 0.3 * boundary_health;
        
        CellHealth {
            score,
            size_health,
            connectivity_health,
            boundary_health,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::w4_fluid::EdgeTelemetry;
    
    fn create_test_edge(source: u64, dest: u64, weight: f64) -> EdgeState {
        let id = EdgeId::new(source, dest);
        EdgeState {
            id,
            weight,
            viscosity: 0.03,
            temperature: 0.0,
            curvature: 0.0,
            density: 0.0,
            load: 0.0,
            telemetry: EdgeTelemetry::new(1000.0),
            last_update: None,
        }
    }
    
    #[test]
    fn test_cell_formation_config() {
        let config = CellFormationConfig::default();
        assert_eq!(config.boundary_threshold, -0.1);
        assert_eq!(config.min_cell_size, 3);
        assert_eq!(config.max_cell_size, 10);
        assert_eq!(config.optimal_cell_size, 5);
    }
    
    #[test]
    fn test_cell_formation_engine_creation() {
        let config = CellFormationConfig::default();
        let engine = CellFormationEngine::new(config);
        assert_eq!(engine.next_cell_id, 1);
    }
    
    #[test]
    fn test_simple_cell_formation() {
        let config = CellFormationConfig::default();
        let mut engine = CellFormationEngine::new(config);
        
        // Create a simple triangle (should form one cell)
        let mut edges = HashMap::new();
        edges.insert(EdgeId::new(1, 2), create_test_edge(1, 2, 1.0));
        edges.insert(EdgeId::new(2, 3), create_test_edge(2, 3, 1.0));
        edges.insert(EdgeId::new(3, 1), create_test_edge(3, 1, 1.0));
        
        let cells = engine.form_cells(&edges);
        
        // Should form at least one cell
        assert!(!cells.is_empty());
    }
    
    #[test]
    fn test_cell_state() {
        let state = CellState::Active;
        assert_eq!(state, CellState::Active);
        assert_ne!(state, CellState::Forming);
    }
    
    #[test]
    fn test_cell_health_calculation() {
        let config = CellFormationConfig::default();
        let engine = CellFormationEngine::new(config);
        
        let members = vec![1, 2, 3, 4, 5]; // Optimal size
        let profile = CurvatureProfile {
            avg_internal_curvature: 0.5,
            avg_boundary_curvature: -0.3,
            min_curvature: -0.5,
            max_curvature: 0.8,
        };
        
        let health = engine.calculate_health(&members, &profile);
        
        assert!(health.score > 0.0);
        assert!(health.score <= 1.0);
        assert!(health.size_health > 0.0);
        assert!(health.connectivity_health >= 0.0);
        assert!(health.boundary_health >= 0.0);
    }
}
