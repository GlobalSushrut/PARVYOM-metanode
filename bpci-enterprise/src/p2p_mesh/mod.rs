//! P2P Mesh Infrastructure
//! 
//! Implements serverless P2P mesh for dynamic service discovery.
//! 
//! # Overview
//! 
//! Replaces hardcoded URLs with dynamic service discovery:
//! ```rust
//! // ❌ OLD: Hardcoded
//! let url = "http://localhost:8080";
//! 
//! // ✅ NEW: Dynamic discovery
//! let endpoint = registry.discover("consensus").await?;
//! let url = format!("http://{}", endpoint.address);
//! ```
//! 
//! # Components
//! 
//! - **DynaRoute Service Discovery**: Distributed service registry
//! - **P2P Mesh Handshake**: O(n log n) peer discovery (coming soon)
//! - **Gossip Protocol**: Information propagation (coming soon)
//! - **CommuteLock Integration**: Zero-copy communication (coming soon)

pub mod discovery;
pub mod handshake;
pub mod factoradic;
pub mod gossip;
pub mod wave;
pub mod fibonacci_stability;
pub mod handshake_protocol;
pub mod commutelock;
pub mod service_migration;

// Re-export main types
pub use discovery::{
    ServiceRegistry,
    ServiceEndpoint,
    HealthStatus,
    RegistryConfig,
};
pub use handshake::{
    P2PMesh,
    PeerInfo,
    MeshConfig,
};
pub use factoradic::{
    Permutation,
    Factoradic,
    permutation_to_factoradic,
    factoradic_to_permutation,
    prf_to_permutation,
    log2_factorial,
};
pub use gossip::{
    GossipProtocol,
    GossipMessage,
    GossipConfig,
    GossipPeerInfo,
};
pub use wave::{
    WaveToken,
    PortalWave,
    WaveScheduler,
};
pub use fibonacci_stability::{
    StabilityTracker,
    StabilityParams,
    WitnessEndorsement,
    AdmissionDecision,
    FibonacciBackoff,
    admission_decision,
    blended_score,
    lease_duration,
    resource_credits,
    fibonacci_fanout,
    FIBONACCI,
    PHI,
    PHI_INV,
    PHI_INV2,
};
pub use handshake_protocol::{
    HandshakeMessage,
    Hello1,
    Hello2,
    Ack3,
    RetryToken,
    Lease,
    ProofOfWork,
    NodeCapabilities,
};
pub use commutelock::{
    CommuteLockChannel,
    CommuteLockManager,
    CommuteLockMessage,
    ChannelType,
};
pub use service_migration::{
    ServiceRouter,
    ServiceMigration,
    ServiceUrl,
    MigrationConfig,
    services,
};

#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::collections::HashMap;
    
    #[tokio::test]
    async fn test_multi_service_discovery() {
        let config = RegistryConfig::default();
        let registry = ServiceRegistry::new("test-node".to_string(), config);
        
        // Register multiple services
        let consensus = ServiceEndpoint {
            service_name: "consensus".to_string(),
            node_id: "node1".to_string(),
            address: "127.0.0.1:8080".parse().unwrap(),
            metadata: HashMap::new(),
            last_heartbeat: std::time::Instant::now(),
            version: "1.0.0".to_string(),
        };
        
        let blockchain = ServiceEndpoint {
            service_name: "blockchain".to_string(),
            node_id: "node1".to_string(),
            address: "127.0.0.1:9000".parse().unwrap(),
            metadata: HashMap::new(),
            last_heartbeat: std::time::Instant::now(),
            version: "1.0.0".to_string(),
        };
        
        registry.register(consensus).await.unwrap();
        registry.register(blockchain).await.unwrap();
        
        // Discover both services
        let c = registry.discover("consensus").await.unwrap();
        let b = registry.discover("blockchain").await.unwrap();
        
        assert_eq!(c.service_name, "consensus");
        assert_eq!(b.service_name, "blockchain");
    }
}
