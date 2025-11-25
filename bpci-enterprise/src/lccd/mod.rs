//! LCCD (Living Cellular Consensus Division)
//! 
//! Implements cell formation and resource-resident BPCI migration.
//! 
//! # Overview
//! 
//! LCCD enables BPCI to "walk off" the central server and live inside BPI slots
//! through cellular division and resource-resident architecture.
//! 
//! # Core Concepts
//! 
//! ## Cellular Division
//! 
//! - **Cell Formation**: Nodes group based on curvature boundaries
//! - **Cell Growth**: Cells absorb nearby nodes
//! - **Cell Division**: Large cells split into smaller ones
//! - **Cell Merging**: Small cells combine for efficiency
//! - **Cell Death**: Unhealthy cells dissolve
//! 
//! ## Curvature-Based Boundaries
//! 
//! - **Positive curvature**: Well-connected (inside cell)
//! - **Negative curvature**: Bottleneck (cell boundary)
//! - Cells form naturally at topology boundaries
//! 
//! ## Resource-Resident Architecture
//! 
//! - BPCI shards migrate to BPI slots
//! - Each cell runs in allocated BPI resources
//! - Slot selection via marketplace + curvature
//! - Policy compliance via Σ-majorization

pub mod cell;
pub mod lifecycle;
pub mod metrics;
pub mod slot_selection;
pub mod migration;

// Re-export main types
pub use cell::{
    LccdCell,
    CellId,
    NodeId,
    CurvatureProfile,
    CellHealth,
    CellState,
    CellFormationConfig,
    CellFormationEngine,
};
pub use lifecycle::{
    LifecycleConfig,
    CellLifecycleManager,
};
pub use metrics::{
    CellPerformanceMetrics,
    CellStabilityMetrics,
    CellHealthCalculator,
    CellMetricsCollector,
    HealthWeights,
};
pub use slot_selection::{
    SlotSelectionConfig,
    SelectionWeights,
    SlotSelectionResult,
    SlotSelector,
    DiversityAnalyzer,
};
pub use migration::{
    MigrationState,
    MigrationConfig,
    MigrationContext,
    MigrationResult,
    MigrationOrchestrator,
    MigrationHealthMonitor,
};

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::w4_fluid::{EdgeId, EdgeState, EdgeTelemetry};
    use std::collections::HashMap;
    
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
    fn test_end_to_end_cell_formation() {
        // Create a network with two clusters connected by a bottleneck
        let mut edges = HashMap::new();
        
        // Cluster 1 (nodes 1-3, well-connected triangle)
        edges.insert(EdgeId::new(1, 2), create_test_edge(1, 2, 1.0));
        edges.insert(EdgeId::new(2, 3), create_test_edge(2, 3, 1.0));
        edges.insert(EdgeId::new(3, 1), create_test_edge(3, 1, 1.0));
        
        // Bottleneck (node 3 to 4)
        edges.insert(EdgeId::new(3, 4), create_test_edge(3, 4, 1.0));
        
        // Cluster 2 (nodes 4-6, well-connected triangle)
        edges.insert(EdgeId::new(4, 5), create_test_edge(4, 5, 1.0));
        edges.insert(EdgeId::new(5, 6), create_test_edge(5, 6, 1.0));
        edges.insert(EdgeId::new(6, 4), create_test_edge(6, 4, 1.0));
        
        // Form cells
        let config = CellFormationConfig::default();
        let mut engine = CellFormationEngine::new(config);
        let cells = engine.form_cells(&edges);
        
        // Should form cells (may be 1 or 2 depending on curvature)
        assert!(!cells.is_empty());
        
        // Verify cell properties
        for cell in &cells {
            assert!(cell.members.len() >= 3);
            assert!(cell.health.score > 0.0);
            assert!(cell.health.score <= 1.0);
        }
    }
}
