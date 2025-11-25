//! LCCD Cell Metrics & Health Monitoring
//! 
//! Implements comprehensive cell health scoring and performance metrics.
//! 
//! # Health Scoring
//! 
//! Cell health is calculated from multiple factors:
//! - Size health (optimal size = best)
//! - Connectivity health (positive curvature = good)
//! - Boundary health (clear boundaries = good)
//! - Performance health (low latency, high throughput)
//! - Stability health (consistent metrics over time)

use super::cell::{LccdCell, CellHealth, CurvatureProfile};
use crate::w4_fluid::{EdgeId, EdgeState};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Cell performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellPerformanceMetrics {
    /// Average latency across cell edges (ms)
    pub avg_latency: f64,
    
    /// Average loss rate (0.0 - 1.0)
    pub avg_loss_rate: f64,
    
    /// Average jitter (ms)
    pub avg_jitter: f64,
    
    /// Average load (0.0 - 1.0)
    pub avg_load: f64,
    
    /// Total throughput (Mbps)
    pub total_throughput: f64,
    
    /// Number of edges in cell
    pub edge_count: usize,
}

/// Cell stability metrics (over time)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellStabilityMetrics {
    /// Health variance (lower = more stable)
    pub health_variance: f64,
    
    /// Member churn rate (changes per epoch)
    pub member_churn_rate: f64,
    
    /// Epochs since last state change
    pub epochs_in_current_state: u64,
    
    /// Total epochs cell has existed
    pub total_epochs: u64,
}

/// Enhanced cell health calculator
pub struct CellHealthCalculator {
    /// Optimal cell size
    optimal_size: usize,
    
    /// Min cell size
    min_size: usize,
    
    /// Max cell size
    max_size: usize,
    
    /// Minimum boundary clarity
    min_boundary_clarity: f64,
    
    /// Health weights
    weights: HealthWeights,
}

/// Weights for health calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthWeights {
    pub size: f64,
    pub connectivity: f64,
    pub boundary: f64,
    pub performance: f64,
    pub stability: f64,
}

impl Default for HealthWeights {
    fn default() -> Self {
        Self {
            size: 0.25,
            connectivity: 0.25,
            boundary: 0.20,
            performance: 0.20,
            stability: 0.10,
        }
    }
}

impl CellHealthCalculator {
    /// Create a new health calculator
    pub fn new(
        optimal_size: usize,
        min_size: usize,
        max_size: usize,
        min_boundary_clarity: f64,
    ) -> Self {
        Self {
            optimal_size,
            min_size,
            max_size,
            min_boundary_clarity,
            weights: HealthWeights::default(),
        }
    }
    
    /// Create with custom weights
    pub fn with_weights(mut self, weights: HealthWeights) -> Self {
        self.weights = weights;
        self
    }
    
    /// Calculate comprehensive cell health
    pub fn calculate_health(
        &self,
        cell: &LccdCell,
        performance: Option<&CellPerformanceMetrics>,
        stability: Option<&CellStabilityMetrics>,
    ) -> CellHealth {
        let size_health = self.calculate_size_health(cell.members.len());
        let connectivity_health = self.calculate_connectivity_health(&cell.curvature_profile);
        let boundary_health = self.calculate_boundary_health(&cell.curvature_profile);
        let performance_health = performance
            .map(|p| self.calculate_performance_health(p))
            .unwrap_or(0.7); // Default if no performance data
        let stability_health = stability
            .map(|s| self.calculate_stability_health(s))
            .unwrap_or(0.7); // Default if no stability data
        
        // Weighted average
        let score = self.weights.size * size_health
            + self.weights.connectivity * connectivity_health
            + self.weights.boundary * boundary_health
            + self.weights.performance * performance_health
            + self.weights.stability * stability_health;
        
        CellHealth {
            score,
            size_health,
            connectivity_health,
            boundary_health,
        }
    }
    
    /// Calculate size health
    fn calculate_size_health(&self, size: usize) -> f64 {
        if size == self.optimal_size {
            1.0
        } else if size < self.min_size {
            0.3
        } else if size > self.max_size {
            0.5
        } else {
            let distance = (size as f64 - self.optimal_size as f64).abs();
            let max_distance = (self.max_size - self.optimal_size) as f64;
            1.0 - (distance / max_distance) * 0.5
        }
    }
    
    /// Calculate connectivity health
    fn calculate_connectivity_health(&self, profile: &CurvatureProfile) -> f64 {
        // Positive internal curvature = good connectivity
        let health = (profile.avg_internal_curvature + 1.0) / 2.0;
        health.max(0.0).min(1.0)
    }
    
    /// Calculate boundary health
    fn calculate_boundary_health(&self, profile: &CurvatureProfile) -> f64 {
        let clarity = profile.avg_internal_curvature - profile.avg_boundary_curvature;
        if clarity >= self.min_boundary_clarity {
            1.0
        } else {
            (clarity / self.min_boundary_clarity).max(0.0)
        }
    }
    
    /// Calculate performance health
    fn calculate_performance_health(&self, metrics: &CellPerformanceMetrics) -> f64 {
        // Lower latency = better (normalize to 0-1)
        let latency_score = (100.0 - metrics.avg_latency.min(100.0)) / 100.0;
        
        // Lower loss = better
        let loss_score = 1.0 - metrics.avg_loss_rate;
        
        // Lower jitter = better (normalize to 0-1)
        let jitter_score = (50.0 - metrics.avg_jitter.min(50.0)) / 50.0;
        
        // Lower load = better (but not too low)
        let load_score = if metrics.avg_load < 0.3 {
            metrics.avg_load / 0.3 // Penalize very low load
        } else if metrics.avg_load > 0.8 {
            (1.0 - metrics.avg_load) / 0.2 // Penalize high load
        } else {
            1.0
        };
        
        // Weighted average
        (0.3 * latency_score + 0.3 * loss_score + 0.2 * jitter_score + 0.2 * load_score)
            .max(0.0)
            .min(1.0)
    }
    
    /// Calculate stability health
    fn calculate_stability_health(&self, metrics: &CellStabilityMetrics) -> f64 {
        // Lower variance = better
        let variance_score = (1.0 - metrics.health_variance.min(1.0)).max(0.0);
        
        // Lower churn = better
        let churn_score = (1.0 - metrics.member_churn_rate.min(1.0)).max(0.0);
        
        // More epochs in current state = better (up to a point)
        let stability_score = (metrics.epochs_in_current_state as f64 / 10.0).min(1.0);
        
        // Weighted average
        (0.4 * variance_score + 0.3 * churn_score + 0.3 * stability_score)
            .max(0.0)
            .min(1.0)
    }
}

/// Cell metrics collector
pub struct CellMetricsCollector;

impl CellMetricsCollector {
    /// Collect performance metrics for a cell
    pub fn collect_performance(
        cell: &LccdCell,
        edges: &HashMap<EdgeId, EdgeState>,
    ) -> CellPerformanceMetrics {
        let member_set: std::collections::HashSet<u64> =
            cell.members.iter().copied().collect();
        
        // Find all edges within the cell
        let cell_edges: Vec<&EdgeState> = edges
            .values()
            .filter(|edge| {
                member_set.contains(&edge.id.source) && member_set.contains(&edge.id.dest)
            })
            .collect();
        
        if cell_edges.is_empty() {
            return CellPerformanceMetrics {
                avg_latency: 0.0,
                avg_loss_rate: 0.0,
                avg_jitter: 0.0,
                avg_load: 0.0,
                total_throughput: 0.0,
                edge_count: 0,
            };
        }
        
        let count = cell_edges.len() as f64;
        
        let avg_latency: f64 = cell_edges
            .iter()
            .map(|e| e.telemetry.latency_ema.value())
            .sum::<f64>()
            / count;
        
        let avg_loss_rate: f64 = cell_edges
            .iter()
            .map(|e| e.telemetry.loss_ema.value())
            .sum::<f64>()
            / count;
        
        let avg_jitter: f64 = cell_edges
            .iter()
            .map(|e| e.telemetry.jitter_ema.value())
            .sum::<f64>()
            / count;
        
        let avg_load: f64 = cell_edges.iter().map(|e| e.load).sum::<f64>() / count;
        
        let total_throughput: f64 = cell_edges
            .iter()
            .map(|e| e.telemetry.capacity * (1.0 - e.load))
            .sum();
        
        CellPerformanceMetrics {
            avg_latency,
            avg_loss_rate,
            avg_jitter,
            avg_load,
            total_throughput,
            edge_count: cell_edges.len(),
        }
    }
    
    /// Calculate stability metrics (requires historical data)
    pub fn calculate_stability(
        previous_health: f64,
        current_health: f64,
        previous_member_count: usize,
        current_member_count: usize,
        epochs_in_state: u64,
        total_epochs: u64,
    ) -> CellStabilityMetrics {
        let health_variance = (current_health - previous_health).abs();
        
        let member_change = (current_member_count as i64 - previous_member_count as i64).abs();
        let member_churn_rate = member_change as f64 / previous_member_count.max(1) as f64;
        
        CellStabilityMetrics {
            health_variance,
            member_churn_rate,
            epochs_in_current_state: epochs_in_state,
            total_epochs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lccd::cell::{CellId, NodeId};
    
    fn create_test_profile(internal: f64, boundary: f64) -> CurvatureProfile {
        CurvatureProfile {
            avg_internal_curvature: internal,
            avg_boundary_curvature: boundary,
            min_curvature: boundary,
            max_curvature: internal,
        }
    }
    
    fn create_test_cell(members: Vec<NodeId>, profile: CurvatureProfile) -> LccdCell {
        LccdCell {
            cell_id: 1,
            members,
            boundary_edges: Vec::new(),
            curvature_profile: profile,
            health: CellHealth {
                score: 0.0,
                size_health: 0.0,
                connectivity_health: 0.0,
                boundary_health: 0.0,
            },
            state: super::super::cell::CellState::Active,
        }
    }
    
    #[test]
    fn test_health_weights_default() {
        let weights = HealthWeights::default();
        assert_eq!(weights.size, 0.25);
        assert_eq!(weights.connectivity, 0.25);
        assert_eq!(weights.boundary, 0.20);
    }
    
    #[test]
    fn test_health_calculator_creation() {
        let calculator = CellHealthCalculator::new(5, 3, 10, 0.2);
        assert_eq!(calculator.optimal_size, 5);
        assert_eq!(calculator.min_size, 3);
        assert_eq!(calculator.max_size, 10);
    }
    
    #[test]
    fn test_size_health_optimal() {
        let calculator = CellHealthCalculator::new(5, 3, 10, 0.2);
        let health = calculator.calculate_size_health(5);
        assert_eq!(health, 1.0);
    }
    
    #[test]
    fn test_size_health_too_small() {
        let calculator = CellHealthCalculator::new(5, 3, 10, 0.2);
        let health = calculator.calculate_size_health(2);
        assert_eq!(health, 0.3);
    }
    
    #[test]
    fn test_size_health_too_large() {
        let calculator = CellHealthCalculator::new(5, 3, 10, 0.2);
        let health = calculator.calculate_size_health(11);
        assert_eq!(health, 0.5);
    }
    
    #[test]
    fn test_connectivity_health() {
        let calculator = CellHealthCalculator::new(5, 3, 10, 0.2);
        let profile = create_test_profile(0.5, -0.3);
        let health = calculator.calculate_connectivity_health(&profile);
        assert!(health > 0.5);
        assert!(health <= 1.0);
    }
    
    #[test]
    fn test_boundary_health_clear() {
        let calculator = CellHealthCalculator::new(5, 3, 10, 0.2);
        let profile = create_test_profile(0.5, -0.3); // Clarity = 0.8
        let health = calculator.calculate_boundary_health(&profile);
        assert_eq!(health, 1.0);
    }
    
    #[test]
    fn test_boundary_health_unclear() {
        let calculator = CellHealthCalculator::new(5, 3, 10, 0.2);
        let profile = create_test_profile(0.1, 0.0); // Clarity = 0.1
        let health = calculator.calculate_boundary_health(&profile);
        assert!(health < 1.0);
    }
    
    #[test]
    fn test_performance_health() {
        let calculator = CellHealthCalculator::new(5, 3, 10, 0.2);
        let metrics = CellPerformanceMetrics {
            avg_latency: 10.0,
            avg_loss_rate: 0.01,
            avg_jitter: 2.0,
            avg_load: 0.5,
            total_throughput: 1000.0,
            edge_count: 5,
        };
        let health = calculator.calculate_performance_health(&metrics);
        assert!(health > 0.5);
        assert!(health <= 1.0);
    }
    
    #[test]
    fn test_stability_health() {
        let calculator = CellHealthCalculator::new(5, 3, 10, 0.2);
        let metrics = CellStabilityMetrics {
            health_variance: 0.1,
            member_churn_rate: 0.2,
            epochs_in_current_state: 5,
            total_epochs: 20,
        };
        let health = calculator.calculate_stability_health(&metrics);
        assert!(health > 0.0);
        assert!(health <= 1.0);
    }
    
    #[test]
    fn test_comprehensive_health_calculation() {
        let calculator = CellHealthCalculator::new(5, 3, 10, 0.2);
        let profile = create_test_profile(0.5, -0.3);
        let cell = create_test_cell(vec![1, 2, 3, 4, 5], profile);
        
        let health = calculator.calculate_health(&cell, None, None);
        
        assert!(health.score > 0.0);
        assert!(health.score <= 1.0);
        assert!(health.size_health > 0.0);
        assert!(health.connectivity_health > 0.0);
        assert!(health.boundary_health > 0.0);
    }
    
    #[test]
    fn test_stability_calculation() {
        let stability = CellMetricsCollector::calculate_stability(
            0.8, // previous health
            0.85, // current health
            5,   // previous members
            6,   // current members
            10,  // epochs in state
            50,  // total epochs
        );
        
        assert_eq!(stability.health_variance, 0.05);
        assert_eq!(stability.member_churn_rate, 0.2);
        assert_eq!(stability.epochs_in_current_state, 10);
        assert_eq!(stability.total_epochs, 50);
    }
}
