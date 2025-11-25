//! W4-FT Fluid Transport Layer
//! 
//! Implements the Web-4 Fluid Transport (W4-FT) layer for PRAVYOM DOOR (Aegis).
//! 
//! # Overview
//! 
//! W4-FT provides adaptive, self-healing network transport using fluid dynamics:
//! - **w (weight)**: Routing metric that evolves via Ricci flow
//! - **ν (viscosity)**: Resistance to flow, adapts to congestion
//! - **Θ (temperature)**: Disorder/jitter, triggers healing flows
//! - **κ (curvature)**: Forman/Ollivier-Ricci curvature
//! - **ρ (density)**: Traffic load
//! 
//! # Core Concepts
//! 
//! ## Fluid Properties
//! 
//! Each network edge maintains fluid properties that evolve over time:
//! 
//! - **Weight Evolution**: `w' = w - 2κ + κ_f·φ(ρ) + ν·Δw - η·w`
//! - **Viscosity**: `ν = ν_base + a·queue + b·jitter`
//! - **Temperature**: `Θ = jitter` (disorder metric)
//! - **Density**: `ρ = load` (traffic load)
//! 
//! ## EMA Telemetry
//! 
//! All metrics use Exponential Moving Average (EMA) for smoothing:
//! - Latency (milliseconds)
//! - Loss rate (0.0 - 1.0)
//! - Jitter (milliseconds)
//! - Queue depth (packets)
//! 
//! ## Healing Flows
//! 
//! When temperature exceeds threshold, healing flows are triggered:
//! - Spin = ○ (healing)
//! - Reduces congestion
//! - Redistributes load
//! 
//! # Usage
//! 
//! ```rust
//! use bpci_enterprise::w4_fluid::{FluidState, FluidConfig, EdgeId};
//! 
//! // Create fluid state
//! let config = FluidConfig::default();
//! let mut state = FluidState::new(config);
//! 
//! // Add edges
//! let edge = EdgeId::new(1, 2);
//! state.add_edge(edge, 1000.0); // 1 Gbps capacity
//! 
//! // Update telemetry
//! state.update_edge_telemetry(
//!     edge,
//!     10.0,   // latency_ms
//!     0.001,  // loss_rate
//!     2.0,    // jitter_ms
//!     5.0,    // queue_depth
//! );
//! 
//! // Perform fluid step
//! state.fluid_step();
//! 
//! // Check for healing
//! let healing_edges = state.edges_needing_healing();
//! if !healing_edges.is_empty() {
//!     println!("Edges need healing: {:?}", healing_edges);
//! }
//! ```
//! 
//! # Integration with HERMES
//! 
//! The fluid state integrates with HERMES P2P mesh:
//! - Each HERMES edge has corresponding fluid state
//! - Telemetry collected from actual network measurements
//! - Fluid scores used for routing decisions
//! - Healing flows triggered automatically

pub mod ema;
pub mod edge;
pub mod state;
pub mod curvature;
pub mod ricci_flow;

// Re-export main types
pub use ema::{ExponentialMovingAverage, EmaPresets};
pub use edge::{EdgeId, EdgeState, EdgeTelemetry};
pub use state::{FluidState, FluidConfig};
pub use curvature::{CurvatureGraph, FormanCurvature, OllivierRicciCurvature, CurvatureResult};
pub use ricci_flow::{RicciFlowEngine, RicciFlowConfig, EvolutionStats};

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_end_to_end_fluid_dynamics() {
        // Scenario: Network with 3 edges, one becomes congested
        
        let config = FluidConfig::default();
        let mut state = FluidState::new(config);
        
        // Add edges
        let edge1 = EdgeId::new(1, 2);
        let edge2 = EdgeId::new(2, 3);
        let edge3 = EdgeId::new(3, 4);
        
        state.add_edge(edge1, 1000.0);
        state.add_edge(edge2, 1000.0);
        state.add_edge(edge3, 1000.0);
        
        // Normal traffic on edge1 and edge3
        state.update_edge_telemetry(edge1, 10.0, 0.001, 2.0, 5.0);
        state.update_edge_telemetry(edge3, 12.0, 0.001, 2.5, 6.0);
        
        // Congested traffic on edge2
        state.update_edge_telemetry(edge2, 50.0, 0.05, 20.0, 50.0);
        
        // Check healing detection
        let healing = state.edges_needing_healing();
        assert_eq!(healing.len(), 1);
        assert_eq!(healing[0], edge2);
        
        // Check viscosity increased on congested edge
        let edge2_state = state.get_edge(&edge2).unwrap();
        let edge1_state = state.get_edge(&edge1).unwrap();
        assert!(edge2_state.viscosity > edge1_state.viscosity);
    }
    
    #[test]
    fn test_fluid_step_evolution() {
        let config = FluidConfig::default();
        let mut state = FluidState::new(config);
        
        let edge = EdgeId::new(1, 2);
        state.add_edge(edge, 1000.0);
        
        // Initial state
        assert_eq!(state.epoch, 0);
        
        // Update telemetry
        state.update_edge_telemetry(edge, 10.0, 0.001, 2.0, 5.0);
        
        // Perform multiple fluid steps
        for i in 1..=10 {
            state.fluid_step();
            assert_eq!(state.epoch, i);
        }
        
        // Verify edge state is maintained
        assert!(state.get_edge(&edge).is_some());
    }
    
    #[test]
    fn test_adaptive_viscosity() {
        let config = FluidConfig::default();
        let mut state = FluidState::new(config);
        
        let edge = EdgeId::new(1, 2);
        state.add_edge(edge, 1000.0);
        
        // Low congestion
        state.update_edge_telemetry(edge, 10.0, 0.001, 2.0, 5.0);
        let low_viscosity = state.get_edge(&edge).unwrap().viscosity;
        
        // High congestion
        state.update_edge_telemetry(edge, 50.0, 0.05, 20.0, 50.0);
        let high_viscosity = state.get_edge(&edge).unwrap().viscosity;
        
        // Viscosity should increase with congestion
        assert!(high_viscosity > low_viscosity);
    }
    
    #[test]
    fn test_temperature_tracking() {
        let config = FluidConfig::default();
        let mut state = FluidState::new(config);
        
        let edge = EdgeId::new(1, 2);
        state.add_edge(edge, 1000.0);
        
        // Low jitter = low temperature
        state.update_edge_telemetry(edge, 10.0, 0.001, 2.0, 5.0);
        assert_eq!(state.get_edge(&edge).unwrap().temperature, 2.0);
        
        // High jitter = high temperature
        state.update_edge_telemetry(edge, 10.0, 0.001, 20.0, 5.0);
        assert_eq!(state.get_edge(&edge).unwrap().temperature, 20.0);
    }
    
    #[test]
    fn test_load_tracking() {
        let config = FluidConfig::default();
        let mut state = FluidState::new(config);
        
        let edge = EdgeId::new(1, 2);
        state.add_edge(edge, 1000.0);
        
        // 30% load
        state.update_edge_telemetry(edge, 10.0, 0.001, 2.0, 30.0);
        assert_eq!(state.get_edge(&edge).unwrap().load, 0.3);
        
        // 80% load
        state.update_edge_telemetry(edge, 10.0, 0.001, 2.0, 80.0);
        assert_eq!(state.get_edge(&edge).unwrap().load, 0.8);
    }
    
    #[test]
    fn test_config_presets() {
        let conservative = FluidConfig::conservative();
        let aggressive = FluidConfig::aggressive();
        
        // Conservative should have higher base viscosity
        assert!(conservative.nu_base > aggressive.nu_base);
        
        // Aggressive should have lower healing threshold
        assert!(aggressive.temp_healing_threshold < conservative.temp_healing_threshold);
        
        // Test with both configs
        let mut state1 = FluidState::new(conservative);
        let mut state2 = FluidState::new(aggressive);
        
        let edge = EdgeId::new(1, 2);
        state1.add_edge(edge, 1000.0);
        state2.add_edge(edge, 1000.0);
        
        // Same telemetry
        state1.update_edge_telemetry(edge, 10.0, 0.001, 8.0, 10.0);
        state2.update_edge_telemetry(edge, 10.0, 0.001, 8.0, 10.0);
        
        // Aggressive should trigger healing, conservative shouldn't
        assert!(state1.edges_needing_healing().is_empty());
        assert!(!state2.edges_needing_healing().is_empty());
    }
}
