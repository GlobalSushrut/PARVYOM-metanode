//! Hyperbolic Geometry for HERMES P2P
//! 
//! Implements Poincaré disk model for optimal P2P routing.
//! Based on Kleinberg's theorem: Greedy routing in hyperbolic space achieves O(log n) hops.
//! 
//! Mathematical Foundation:
//! - Poincaré disk: D = {z ∈ ℂ : |z| < 1}
//! - Hyperbolic distance: d(z₁, z₂) = arcosh(1 + 2·|z₁ - z₂|²/((1-|z₁|²)(1-|z₂|²)))
//! - Greedy routing: Forward to neighbor closest to target in hyperbolic space

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Hyperbolic coordinates in Poincaré disk model
/// 
/// All coordinates must satisfy: x² + y² < 1 (inside unit disk)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HyperbolicCoordinates {
    /// Real part (x-coordinate in Poincaré disk)
    pub x: f64,
    /// Imaginary part (y-coordinate in Poincaré disk)
    pub y: f64,
}

impl HyperbolicCoordinates {
    /// Create new hyperbolic coordinates
    /// 
    /// # Panics
    /// Panics if coordinates are outside unit disk (x² + y² >= 1)
    pub fn new(x: f64, y: f64) -> Self {
        let norm_sq = x * x + y * y;
        assert!(norm_sq < 1.0, "Coordinates must be inside unit disk: x²+y² < 1");
        Self { x, y }
    }
    
    /// Create coordinates at origin (center of Poincaré disk)
    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    
    /// Generate random coordinates inside unit disk
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // Generate random angle and radius
        let angle = rng.gen::<f64>() * 2.0 * PI;
        let radius = rng.gen::<f64>().sqrt() * 0.95; // Keep away from boundary
        
        Self {
            x: radius * angle.cos(),
            y: radius * angle.sin(),
        }
    }
    
    /// Calculate hyperbolic distance to another point
    /// 
    /// Uses Poincaré disk distance formula:
    /// d(z₁, z₂) = arcosh(1 + 2·|z₁ - z₂|²/((1-|z₁|²)(1-|z₂|²)))
    /// 
    /// # Returns
    /// Hyperbolic distance (always non-negative)
    pub fn distance(&self, other: &Self) -> f64 {
        let z1_norm_sq = self.x * self.x + self.y * self.y;
        let z2_norm_sq = other.x * other.x + other.y * other.y;
        let diff_x = self.x - other.x;
        let diff_y = self.y - other.y;
        let diff_norm_sq = diff_x * diff_x + diff_y * diff_y;
        
        // Handle edge cases
        if diff_norm_sq < 1e-10 {
            return 0.0; // Same point
        }
        
        let denominator = (1.0 - z1_norm_sq) * (1.0 - z2_norm_sq);
        if denominator < 1e-10 {
            return f64::INFINITY; // Near boundary
        }
        
        let numerator = 1.0 + 2.0 * diff_norm_sq / denominator;
        
        // arcosh(x) = ln(x + sqrt(x² - 1))
        if numerator < 1.0 {
            return 0.0; // Numerical error, treat as same point
        }
        
        numerator.acosh()
    }
    
    /// Greedy routing: Select neighbor closest to target
    /// 
    /// This is the core of HERMES routing algorithm.
    /// Kleinberg's theorem guarantees O(log n) hops with high probability.
    /// 
    /// # Arguments
    /// * `target` - Destination coordinates
    /// * `neighbors` - List of neighbor coordinates
    /// 
    /// # Returns
    /// Index of neighbor closest to target, or None if no neighbors
    pub fn greedy_route(&self, target: &Self, neighbors: &[(usize, Self)]) -> Option<usize> {
        if neighbors.is_empty() {
            return None;
        }
        
        // Find neighbor that minimizes distance to target
        neighbors.iter()
            .min_by(|(_, a), (_, b)| {
                let dist_a = a.distance(target);
                let dist_b = b.distance(target);
                dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(idx, _)| *idx)
    }
    
    /// Check if this point is closer to target than current position
    /// 
    /// Used to determine if greedy routing is making progress
    pub fn is_closer_to(&self, target: &Self, current: &Self) -> bool {
        self.distance(target) < current.distance(target)
    }
    
    /// Calculate norm squared (x² + y²)
    pub fn norm_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }
    
    /// Calculate Euclidean distance (for comparison, not routing)
    pub fn euclidean_distance(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Hyperbolic embedding for network nodes
/// 
/// Maintains mapping between node IDs and hyperbolic coordinates
#[derive(Debug, Clone)]
pub struct HyperbolicEmbedding {
    /// Coordinates for this node
    pub coordinates: HyperbolicCoordinates,
    /// Coordinates of neighbors (node_idx, coordinates)
    pub neighbors: Vec<(usize, HyperbolicCoordinates)>,
}

impl HyperbolicEmbedding {
    /// Create new embedding with given coordinates
    pub fn new(coordinates: HyperbolicCoordinates) -> Self {
        Self {
            coordinates,
            neighbors: Vec::new(),
        }
    }
    
    /// Create embedding at random position
    pub fn random() -> Self {
        Self::new(HyperbolicCoordinates::random())
    }
    
    /// Add a neighbor with their coordinates
    pub fn add_neighbor(&mut self, node_idx: usize, coords: HyperbolicCoordinates) {
        self.neighbors.push((node_idx, coords));
    }
    
    /// Remove a neighbor
    pub fn remove_neighbor(&mut self, node_idx: usize) {
        self.neighbors.retain(|(idx, _)| *idx != node_idx);
    }
    
    /// Get neighbor count
    pub fn neighbor_count(&self) -> usize {
        self.neighbors.len()
    }
    
    /// Greedy route to target
    /// 
    /// Returns index of next hop neighbor, or None if no neighbors
    pub fn route_to(&self, target: &HyperbolicCoordinates) -> Option<usize> {
        self.coordinates.greedy_route(target, &self.neighbors)
    }
    
    /// Calculate distance to target
    pub fn distance_to(&self, target: &HyperbolicCoordinates) -> f64 {
        self.coordinates.distance(target)
    }
    
    /// Get closest neighbor to target
    pub fn closest_neighbor_to(&self, target: &HyperbolicCoordinates) -> Option<(usize, f64)> {
        self.neighbors.iter()
            .map(|(idx, coords)| (*idx, coords.distance(target)))
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }
}

/// Hyperbolic space metrics and statistics
#[derive(Debug, Clone)]
pub struct HyperbolicMetrics {
    /// Average path length (in hops)
    pub avg_path_length: f64,
    /// Maximum path length observed
    pub max_path_length: usize,
    /// Greedy routing success rate
    pub success_rate: f64,
    /// Average stretch (ratio of greedy path to optimal path)
    pub avg_stretch: f64,
}

impl HyperbolicMetrics {
    /// Create empty metrics
    pub fn new() -> Self {
        Self {
            avg_path_length: 0.0,
            max_path_length: 0,
            success_rate: 0.0,
            avg_stretch: 1.0,
        }
    }
}

impl Default for HyperbolicMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_coordinates_creation() {
        let coords = HyperbolicCoordinates::new(0.5, 0.3);
        assert_eq!(coords.x, 0.5);
        assert_eq!(coords.y, 0.3);
        assert!(coords.norm_squared() < 1.0);
    }
    
    #[test]
    #[should_panic]
    fn test_coordinates_outside_disk() {
        // Should panic: 0.8² + 0.8² = 1.28 > 1
        HyperbolicCoordinates::new(0.8, 0.8);
    }
    
    #[test]
    fn test_origin() {
        let origin = HyperbolicCoordinates::origin();
        assert_eq!(origin.x, 0.0);
        assert_eq!(origin.y, 0.0);
    }
    
    #[test]
    fn test_distance_to_self() {
        let coords = HyperbolicCoordinates::new(0.5, 0.3);
        let dist = coords.distance(&coords);
        assert!(dist < 1e-6); // Should be ~0
    }
    
    #[test]
    fn test_distance_symmetry() {
        let a = HyperbolicCoordinates::new(0.2, 0.3);
        let b = HyperbolicCoordinates::new(0.5, 0.1);
        
        let dist_ab = a.distance(&b);
        let dist_ba = b.distance(&a);
        
        assert!((dist_ab - dist_ba).abs() < 1e-10); // Should be equal
    }
    
    #[test]
    fn test_distance_to_origin() {
        let origin = HyperbolicCoordinates::origin();
        let point = HyperbolicCoordinates::new(0.5, 0.0);
        
        let dist = origin.distance(&point);
        assert!(dist > 0.0);
        assert!(dist < f64::INFINITY);
    }
    
    #[test]
    fn test_greedy_routing() {
        let current = HyperbolicCoordinates::new(0.0, 0.0);
        let target = HyperbolicCoordinates::new(0.7, 0.0);
        
        let neighbors = vec![
            (0, HyperbolicCoordinates::new(0.3, 0.0)),  // Closer to target
            (1, HyperbolicCoordinates::new(-0.3, 0.0)), // Farther from target
            (2, HyperbolicCoordinates::new(0.0, 0.3)),  // Perpendicular
        ];
        
        let next_hop = current.greedy_route(&target, &neighbors);
        assert_eq!(next_hop, Some(0)); // Should select neighbor 0
    }
    
    #[test]
    fn test_greedy_routing_no_neighbors() {
        let current = HyperbolicCoordinates::new(0.0, 0.0);
        let target = HyperbolicCoordinates::new(0.7, 0.0);
        
        let next_hop = current.greedy_route(&target, &[]);
        assert_eq!(next_hop, None);
    }
    
    #[test]
    fn test_embedding_creation() {
        let coords = HyperbolicCoordinates::new(0.5, 0.3);
        let embedding = HyperbolicEmbedding::new(coords);
        
        assert_eq!(embedding.coordinates.x, 0.5);
        assert_eq!(embedding.coordinates.y, 0.3);
        assert_eq!(embedding.neighbor_count(), 0);
    }
    
    #[test]
    fn test_embedding_add_neighbor() {
        let mut embedding = HyperbolicEmbedding::new(HyperbolicCoordinates::origin());
        
        embedding.add_neighbor(1, HyperbolicCoordinates::new(0.3, 0.0));
        embedding.add_neighbor(2, HyperbolicCoordinates::new(0.0, 0.3));
        
        assert_eq!(embedding.neighbor_count(), 2);
    }
    
    #[test]
    fn test_embedding_remove_neighbor() {
        let mut embedding = HyperbolicEmbedding::new(HyperbolicCoordinates::origin());
        
        embedding.add_neighbor(1, HyperbolicCoordinates::new(0.3, 0.0));
        embedding.add_neighbor(2, HyperbolicCoordinates::new(0.0, 0.3));
        embedding.remove_neighbor(1);
        
        assert_eq!(embedding.neighbor_count(), 1);
    }
    
    #[test]
    fn test_embedding_routing() {
        let mut embedding = HyperbolicEmbedding::new(HyperbolicCoordinates::origin());
        
        embedding.add_neighbor(1, HyperbolicCoordinates::new(0.3, 0.0));
        embedding.add_neighbor(2, HyperbolicCoordinates::new(-0.3, 0.0));
        
        let target = HyperbolicCoordinates::new(0.7, 0.0);
        let next_hop = embedding.route_to(&target);
        
        assert_eq!(next_hop, Some(1)); // Should route to neighbor 1
    }
    
    #[test]
    fn test_random_coordinates() {
        for _ in 0..100 {
            let coords = HyperbolicCoordinates::random();
            assert!(coords.norm_squared() < 1.0); // Must be inside unit disk
        }
    }
    
    #[test]
    fn test_triangle_inequality() {
        // For any three points a, b, c: d(a,c) <= d(a,b) + d(b,c)
        let a = HyperbolicCoordinates::new(0.1, 0.1);
        let b = HyperbolicCoordinates::new(0.3, 0.2);
        let c = HyperbolicCoordinates::new(0.5, 0.4);
        
        let d_ac = a.distance(&c);
        let d_ab = a.distance(&b);
        let d_bc = b.distance(&c);
        
        assert!(d_ac <= d_ab + d_bc + 1e-10); // Allow small numerical error
    }
}
