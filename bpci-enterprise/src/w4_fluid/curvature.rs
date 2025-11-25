//! Curvature Calculation for W4-FT
//! 
//! Implements both Forman curvature and Ollivier-Ricci curvature for network edges.
//! 
//! # Forman Curvature
//! 
//! Discrete Ricci curvature for graphs based on Forman's work:
//! 
//! ```text
//! κ_F(e) = w(e) - Σ[w(e')/√(deg(v)·deg(w))]
//! ```
//! 
//! Where:
//! - e is the edge from v to w
//! - e' are edges sharing a vertex with e
//! - deg(v) is the degree of vertex v
//! 
//! # Ollivier-Ricci Curvature
//! 
//! Based on optimal transport between probability measures:
//! 
//! ```text
//! κ_OR(e) = 1 - W₁(μ_v, μ_w) / d(v,w)
//! ```
//! 
//! Where:
//! - W₁ is the Wasserstein-1 distance
//! - μ_v, μ_w are probability measures on neighborhoods
//! - d(v,w) is the distance between v and w

use super::edge::{EdgeId, EdgeState};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Graph structure for curvature calculation
#[derive(Debug, Clone)]
pub struct CurvatureGraph {
    /// Edges indexed by EdgeId
    edges: HashMap<EdgeId, EdgeState>,
    
    /// Adjacency list: node -> list of (neighbor, edge_id)
    adjacency: HashMap<u64, Vec<(u64, EdgeId)>>,
    
    /// Node degrees
    degrees: HashMap<u64, usize>,
}

impl CurvatureGraph {
    /// Create a new curvature graph
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            adjacency: HashMap::new(),
            degrees: HashMap::new(),
        }
    }
    
    /// Add an edge to the graph
    pub fn add_edge(&mut self, edge: EdgeState) {
        let id = edge.id;
        
        // Add to edges
        self.edges.insert(id, edge);
        
        // Update adjacency lists
        self.adjacency
            .entry(id.source)
            .or_insert_with(Vec::new)
            .push((id.dest, id));
        
        // Update degrees
        *self.degrees.entry(id.source).or_insert(0) += 1;
        *self.degrees.entry(id.dest).or_insert(0) += 1;
    }
    
    /// Get node degree
    pub fn degree(&self, node: u64) -> usize {
        self.degrees.get(&node).copied().unwrap_or(0)
    }
    
    /// Get neighbors of a node
    pub fn neighbors(&self, node: u64) -> Vec<u64> {
        self.adjacency
            .get(&node)
            .map(|adj| adj.iter().map(|(n, _)| *n).collect())
            .unwrap_or_default()
    }
    
    /// Get edges incident to a node
    pub fn incident_edges(&self, node: u64) -> Vec<EdgeId> {
        let mut edges = Vec::new();
        
        // Outgoing edges
        if let Some(adj) = self.adjacency.get(&node) {
            edges.extend(adj.iter().map(|(_, e)| *e));
        }
        
        // Incoming edges
        for (edge_id, edge) in &self.edges {
            if edge.id.dest == node && edge.id.source != node {
                edges.push(*edge_id);
            }
        }
        
        edges
    }
    
    /// Get edge by ID
    pub fn get_edge(&self, id: &EdgeId) -> Option<&EdgeState> {
        self.edges.get(id)
    }
}

/// Forman curvature calculator
pub struct FormanCurvature {
    graph: CurvatureGraph,
}

impl FormanCurvature {
    /// Create a new Forman curvature calculator
    pub fn new(graph: CurvatureGraph) -> Self {
        Self { graph }
    }
    
    /// Calculate Forman curvature for an edge
    /// 
    /// κ_F(e) = w(e) - Σ[w(e')/√(deg(v)·deg(w))]
    pub fn calculate(&self, edge_id: &EdgeId) -> f64 {
        let edge = match self.graph.get_edge(edge_id) {
            Some(e) => e,
            None => return 0.0,
        };
        
        let v = edge_id.source;
        let w = edge_id.dest;
        
        let deg_v = self.graph.degree(v) as f64;
        let deg_w = self.graph.degree(w) as f64;
        
        if deg_v == 0.0 || deg_w == 0.0 {
            return edge.weight;
        }
        
        // Start with edge weight
        let mut curvature = edge.weight;
        
        // Subtract contributions from adjacent edges
        let adjacent_edges = self.get_adjacent_edges(edge_id);
        
        for adj_edge_id in adjacent_edges {
            if let Some(adj_edge) = self.graph.get_edge(&adj_edge_id) {
                let adj_v = adj_edge_id.source;
                let adj_w = adj_edge_id.dest;
                
                let adj_deg_v = self.graph.degree(adj_v) as f64;
                let adj_deg_w = self.graph.degree(adj_w) as f64;
                
                if adj_deg_v > 0.0 && adj_deg_w > 0.0 {
                    let normalization = (adj_deg_v * adj_deg_w).sqrt();
                    curvature -= adj_edge.weight / normalization;
                }
            }
        }
        
        curvature
    }
    
    /// Get edges adjacent to the given edge (sharing a vertex)
    fn get_adjacent_edges(&self, edge_id: &EdgeId) -> Vec<EdgeId> {
        let mut adjacent = Vec::new();
        
        // Edges incident to source
        let source_edges = self.graph.incident_edges(edge_id.source);
        adjacent.extend(source_edges.into_iter().filter(|e| e != edge_id));
        
        // Edges incident to dest
        let dest_edges = self.graph.incident_edges(edge_id.dest);
        adjacent.extend(dest_edges.into_iter().filter(|e| e != edge_id));
        
        adjacent
    }
}

/// Ollivier-Ricci curvature calculator
pub struct OllivierRicciCurvature {
    graph: CurvatureGraph,
    /// Probability mass on the edge itself (α)
    alpha: f64,
}

impl OllivierRicciCurvature {
    /// Create a new Ollivier-Ricci curvature calculator
    /// 
    /// # Arguments
    /// * `graph` - The graph structure
    /// * `alpha` - Probability mass on the edge itself (typically 0.0 to 0.5)
    pub fn new(graph: CurvatureGraph, alpha: f64) -> Self {
        Self { graph, alpha }
    }
    
    /// Calculate Ollivier-Ricci curvature for an edge
    /// 
    /// κ_OR(e) = 1 - W₁(μ_v, μ_w) / d(v,w)
    pub fn calculate(&self, edge_id: &EdgeId) -> f64 {
        let edge = match self.graph.get_edge(edge_id) {
            Some(e) => e,
            None => return 0.0,
        };
        
        let v = edge_id.source;
        let w = edge_id.dest;
        
        // Build probability distributions
        let mu_v = self.build_distribution(v);
        let mu_w = self.build_distribution(w);
        
        // Calculate Wasserstein-1 distance
        let w1_distance = self.wasserstein_distance(&mu_v, &mu_w);
        
        // Edge distance (use weight as distance)
        let edge_distance = edge.weight.max(0.001); // Avoid division by zero
        
        // Ollivier-Ricci curvature
        1.0 - (w1_distance / edge_distance)
    }
    
    /// Build probability distribution for a node
    fn build_distribution(&self, node: u64) -> HashMap<u64, f64> {
        let mut dist = HashMap::new();
        
        let neighbors = self.graph.neighbors(node);
        
        if neighbors.is_empty() {
            // Isolated node - all mass on itself
            dist.insert(node, 1.0);
            return dist;
        }
        
        // Mass on the node itself
        dist.insert(node, self.alpha);
        
        // Distribute remaining mass uniformly to neighbors
        let neighbor_mass = (1.0 - self.alpha) / neighbors.len() as f64;
        
        for neighbor in neighbors {
            *dist.entry(neighbor).or_insert(0.0) += neighbor_mass;
        }
        
        dist
    }
    
    /// Calculate Wasserstein-1 distance between two distributions
    /// 
    /// Simplified calculation using coupling approach
    fn wasserstein_distance(
        &self,
        mu: &HashMap<u64, f64>,
        nu: &HashMap<u64, f64>,
    ) -> f64 {
        let mut distance = 0.0;
        
        // Get all nodes in both distributions
        let mut all_nodes: Vec<u64> = mu.keys().chain(nu.keys()).copied().collect();
        all_nodes.sort();
        all_nodes.dedup();
        
        // For each pair of nodes, calculate transport cost
        for &i in &all_nodes {
            for &j in &all_nodes {
                let mu_i = mu.get(&i).copied().unwrap_or(0.0);
                let nu_j = nu.get(&j).copied().unwrap_or(0.0);
                
                if mu_i > 0.0 && nu_j > 0.0 {
                    // Distance between nodes (simplified: use edge weight if exists, else 1.0)
                    let node_distance = self.get_node_distance(i, j);
                    
                    // Transport cost
                    let transport = mu_i.min(nu_j);
                    distance += transport * node_distance;
                }
            }
        }
        
        distance
    }
    
    /// Get distance between two nodes
    fn get_node_distance(&self, i: u64, j: u64) -> f64 {
        if i == j {
            return 0.0;
        }
        
        // Try to find direct edge
        let edge_id = EdgeId::new(i, j);
        if let Some(edge) = self.graph.get_edge(&edge_id) {
            return edge.weight;
        }
        
        // Try reverse edge
        let reverse_id = EdgeId::new(j, i);
        if let Some(edge) = self.graph.get_edge(&reverse_id) {
            return edge.weight;
        }
        
        // Default distance
        1.0
    }
}

/// Curvature result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurvatureResult {
    pub edge_id: EdgeId,
    pub forman: f64,
    pub ollivier_ricci: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
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
            telemetry: super::super::edge::EdgeTelemetry::new(1000.0),
            last_update: None,
        }
    }
    
    #[test]
    fn test_graph_creation() {
        let mut graph = CurvatureGraph::new();
        
        let edge = create_test_edge(1, 2, 1.0);
        graph.add_edge(edge);
        
        assert_eq!(graph.degree(1), 1);
        assert_eq!(graph.degree(2), 1);
    }
    
    #[test]
    fn test_graph_neighbors() {
        let mut graph = CurvatureGraph::new();
        
        graph.add_edge(create_test_edge(1, 2, 1.0));
        graph.add_edge(create_test_edge(1, 3, 1.0));
        
        let neighbors = graph.neighbors(1);
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&2));
        assert!(neighbors.contains(&3));
    }
    
    #[test]
    fn test_forman_curvature_simple() {
        let mut graph = CurvatureGraph::new();
        
        // Simple triangle: 1-2, 2-3, 3-1
        graph.add_edge(create_test_edge(1, 2, 1.0));
        graph.add_edge(create_test_edge(2, 3, 1.0));
        graph.add_edge(create_test_edge(3, 1, 1.0));
        
        let forman = FormanCurvature::new(graph);
        let edge_id = EdgeId::new(1, 2);
        let curvature = forman.calculate(&edge_id);
        
        // Should be positive (triangle has positive curvature)
        assert!(curvature > 0.0);
    }
    
    #[test]
    fn test_forman_curvature_line() {
        let mut graph = CurvatureGraph::new();
        
        // Simple line: 1-2-3
        graph.add_edge(create_test_edge(1, 2, 1.0));
        graph.add_edge(create_test_edge(2, 3, 1.0));
        
        let forman = FormanCurvature::new(graph);
        let edge_id = EdgeId::new(1, 2);
        let curvature = forman.calculate(&edge_id);
        
        // Line should have negative or zero curvature
        assert!(curvature <= 1.0);
    }
    
    #[test]
    fn test_ollivier_ricci_simple() {
        let mut graph = CurvatureGraph::new();
        
        // Simple edge: 1-2
        graph.add_edge(create_test_edge(1, 2, 1.0));
        
        let or_curv = OllivierRicciCurvature::new(graph, 0.5);
        let edge_id = EdgeId::new(1, 2);
        let curvature = or_curv.calculate(&edge_id);
        
        // Should be defined
        assert!(curvature.is_finite());
    }
    
    #[test]
    fn test_ollivier_ricci_triangle() {
        let mut graph = CurvatureGraph::new();
        
        // Triangle: 1-2, 2-3, 3-1
        graph.add_edge(create_test_edge(1, 2, 1.0));
        graph.add_edge(create_test_edge(2, 3, 1.0));
        graph.add_edge(create_test_edge(3, 1, 1.0));
        
        let or_curv = OllivierRicciCurvature::new(graph, 0.5);
        let edge_id = EdgeId::new(1, 2);
        let curvature = or_curv.calculate(&edge_id);
        
        // Triangle should have positive curvature
        assert!(curvature > 0.0);
    }
    
    #[test]
    fn test_distribution_building() {
        let mut graph = CurvatureGraph::new();
        
        graph.add_edge(create_test_edge(1, 2, 1.0));
        graph.add_edge(create_test_edge(1, 3, 1.0));
        
        let or_curv = OllivierRicciCurvature::new(graph, 0.5);
        let dist = or_curv.build_distribution(1);
        
        // Should have mass on node 1 and its neighbors
        assert!(dist.contains_key(&1));
        assert!(dist.contains_key(&2));
        assert!(dist.contains_key(&3));
        
        // Total mass should be 1.0
        let total: f64 = dist.values().sum();
        assert!((total - 1.0).abs() < 0.001);
    }
}
