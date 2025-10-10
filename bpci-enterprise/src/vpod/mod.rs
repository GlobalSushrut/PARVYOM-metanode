//! # vPod (Virtual Processing on Demand) Module
//! 
//! Revolutionary lightweight actor-based runtime system that replaces traditional
//! container orchestration with deterministic, high-performance execution.
//!
//! ## Core Components:
//! - **VPodRuntime**: Main runtime engine with dual-core scheduling
//! - **VPodActor**: Lightweight actors (≤1.5KB state) with ring buffer communication
//! - **VPodScheduler**: Epoch-based scheduler with edge coloring and quanta selection
//! - **VPodNode**: Universal node type that replaces all traditional node implementations
//!
//! ## Performance Targets:
//! - **Throughput**: ≥2.5M messages/second per vPod
//! - **Latency**: P50 ≤20μs, P99 ≤1ms
//! - **Memory**: ≤50MB per application (10× improvement over containers)
//! - **CPU**: ≤0.1 core per application (10× improvement over containers)

pub mod actor;
pub mod runtime;
pub mod scheduler;
pub mod ring_buffer;
pub mod vpod_node;
pub mod actor_types;
pub mod legacy_node_migration;
pub mod blockchain_bridge;
pub mod audit_system;
pub mod migration_tools;
pub mod performance_monitor;

// Re-export core types
pub use actor::{VPodActor, ActorId, ActorState, ActorBudget, Message, ActorStatus, ActorSpecialization, MessagePayload, ControlMessage};
pub use runtime::{VPodRuntime, VPodConfig, RuntimeMetrics};
pub use scheduler::{VPodScheduler, SchedulerConfig, SchedulerMetrics, ArenaAllocator};
pub use ring_buffer::{SPSCRingBuffer, RingBufferError};
pub use vpod_node::{VPodNode, NodeSpecialization, VPodCapabilities};
pub use actor_types::{SpecializedActor};
pub use blockchain_bridge::{BlockchainBridge, ProofBundle, EpochProof};
pub use legacy_node_migration::*;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// vPod system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPodSystemConfig {
    /// Maximum number of actors per vPod
    pub max_actors: usize,
    
    /// Epoch duration in microseconds (5-20μs recommended)
    pub epoch_duration_micros: u64,
    
    /// Ring buffer size for actor communication
    pub ring_buffer_size: usize,
    
    /// Maximum actor state size in bytes (1536 bytes = 1.5KB)
    pub max_actor_state_bytes: usize,
    
    /// Dual-core scheduling enabled
    pub dual_core_enabled: bool,
    
    /// Edge coloring algorithm configuration
    pub edge_coloring_config: EdgeColoringConfig,
    
    /// Quanta selection parameters
    pub quanta_config: QuantaConfig,
    
    /// Blockchain integration settings
    pub blockchain_config: BlockchainConfig,
}

/// Edge coloring algorithm configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeColoringConfig {
    /// Maximum colors to use (Δ+1 where Δ is max degree)
    pub max_colors: u32,
    
    /// Coloring algorithm variant
    pub algorithm: ColoringAlgorithm,
    
    /// Recoloring threshold for dynamic graphs
    pub recolor_threshold: f64,
}

/// Quanta selection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantaConfig {
    /// Maximum quanta per edge per epoch
    pub max_quanta: u32,
    
    /// PI controller parameters for adaptive selection
    pub pi_controller: PIControllerConfig,
    
    /// Queue depth target for optimal performance
    pub target_queue_depth: u32,
    
    /// Adaptive mixing parameter (theta)
    pub adaptive_theta: f64,
}

/// PI Controller configuration for quanta selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PIControllerConfig {
    /// Proportional gain
    pub kp: f64,
    
    /// Integral gain
    pub ki: f64,
    
    /// Integral windup limit
    pub integral_limit: f64,
}

/// Blockchain integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    /// Enable proof-of-execution bundle generation
    pub proof_generation_enabled: bool,
    
    /// Merkle tree depth for audit trails
    pub merkle_tree_depth: u32,
    
    /// BPCI auction integration enabled
    pub bpci_integration_enabled: bool,
    
    /// Audit compression algorithm
    pub compression_algorithm: CompressionAlgorithm,
}

/// Edge coloring algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColoringAlgorithm {
    /// Vizing's theorem implementation (optimal Δ+1 coloring)
    Vizing,
    
    /// Greedy coloring (faster but may use more colors)
    Greedy,
    
    /// Dynamic coloring for changing graphs
    Dynamic,
}

/// Compression algorithms for audit trails
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    /// ZIPLOCK compression (custom algorithm)
    ZipLock,
    
    /// Standard LZ4 compression
    Lz4,
    
    /// No compression (for debugging)
    None,
}

impl Default for VPodSystemConfig {
    fn default() -> Self {
        Self {
            max_actors: 1000,
            epoch_duration_micros: 10, // 10 microseconds
            ring_buffer_size: 1024,
            max_actor_state_bytes: 1536, // 1.5KB
            dual_core_enabled: true,
            edge_coloring_config: EdgeColoringConfig::default(),
            quanta_config: QuantaConfig::default(),
            blockchain_config: BlockchainConfig::default(),
        }
    }
}

impl Default for EdgeColoringConfig {
    fn default() -> Self {
        Self {
            max_colors: 16, // Reasonable default for most graphs
            algorithm: ColoringAlgorithm::Vizing,
            recolor_threshold: 0.1, // Recolor when 10% of edges change
        }
    }
}

impl Default for QuantaConfig {
    fn default() -> Self {
        Self {
            max_quanta: 100,
            pi_controller: PIControllerConfig::default(),
            target_queue_depth: 10,
            adaptive_theta: 0.7, // 70% model-based, 30% PI controller
        }
    }
}

impl Default for PIControllerConfig {
    fn default() -> Self {
        Self {
            kp: 1.0,
            ki: 0.1,
            integral_limit: 100.0,
        }
    }
}

impl Default for BlockchainConfig {
    fn default() -> Self {
        Self {
            proof_generation_enabled: true,
            merkle_tree_depth: 16, // 65536 leaf capacity
            bpci_integration_enabled: true,
            compression_algorithm: CompressionAlgorithm::ZipLock,
        }
    }
}

/// Initialize vPod system with configuration
pub async fn initialize_vpod_system(config: VPodSystemConfig) -> Result<VPodRuntime> {
    let runtime = VPodRuntime::new(VPodConfig::from(config)).await?;
    Ok(runtime)
}

/// Convert system config to runtime config
impl From<VPodSystemConfig> for VPodConfig {
    fn from(system_config: VPodSystemConfig) -> Self {
        VPodConfig {
            max_actors: system_config.max_actors,
            epoch_duration: Duration::from_micros(system_config.epoch_duration_micros),
            ring_buffer_size: system_config.ring_buffer_size,
            max_actor_state_bytes: system_config.max_actor_state_bytes,
            dual_core_enabled: system_config.dual_core_enabled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = VPodSystemConfig::default();
        assert_eq!(config.max_actors, 1000);
        assert_eq!(config.epoch_duration_micros, 10);
        assert_eq!(config.max_actor_state_bytes, 1536);
        assert!(config.dual_core_enabled);
    }

    #[tokio::test]
    async fn test_system_initialization() {
        let config = VPodSystemConfig::default();
        let result = initialize_vpod_system(config).await;
        assert!(result.is_ok());
    }
}
