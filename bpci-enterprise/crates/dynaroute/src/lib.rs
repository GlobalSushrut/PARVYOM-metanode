//! # DynaRoute v2 - Identity-Anycast Dynamic Routing
//! 
//! Shared library for vPod-worthy dynamic routing across BPCI and BPI infrastructure.
//! 
//! ## Core Concepts
//! 
//! - **Identity Anycast Addressing (IAAv6)**: No ports, no service IPs - only identity
//! - **Segment Routing v6 (SRv6)**: Programmable paths encoded in packet headers
//! - **Rendezvous Hashing (HRW)**: Minimal-churn vPod selection
//! - **QUIC Flow Mobility**: Zero-break connection migration
//! - **Merkle Verification**: Cryptographic address validation
//! 
//! ## Architecture
//! 
//! ```text
//! Client → IAAv6 → Edge (SRv6) → vPod (HRW) → Direct
//!          ✅ No ports
//!          ✅ Identity-based
//!          ✅ Flow mobility
//!          ✅ Infinite scale
//! ```

pub mod iaav6;
pub mod srv6;
pub mod hrw;
pub mod quic_cid;
pub mod agent;
pub mod merkle;
pub mod transport;  // Cloud-ready transport layer

// Re-export commonly used types
pub use iaav6::{IAAv6Address, compute_iaav6};
pub use srv6::{SRv6SegmentList, SRv6Segment};
pub use hrw::{RendezvousHasher, VPodWeight};
pub use quic_cid::{BpciConnectionId, ConnectionIdCodec};
pub use agent::AddressSyncAgent;
pub use merkle::{MerkleProof, MerkleTree};
pub use transport::{CloudTransport, CloudServiceDiscovery};

use std::net::Ipv6Addr;

/// DynaRoute configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DynaRouteConfig {
    /// Base IAAv6 prefix (e.g., "2001:db8:03ba::/64")
    pub iaav6_base_prefix: String,
    
    /// SRv6 policy table ID
    pub srv6_policy_table: u32,
    
    /// Maximum vPods per service
    pub hrw_max_vpods_per_service: usize,
    
    /// Epoch rotation interval (seconds)
    pub epoch_rotation_seconds: u64,
    
    /// Realm (e.g., "production", "staging")
    pub realm: String,
    
    /// BlakePage URL for Merkle sync
    pub blakepage_url: String,
    
    /// Merkle sync interval (milliseconds)
    pub merkle_sync_interval_ms: u64,
    
    /// Enable eBPF data plane
    pub ebpf_enabled: bool,
}

impl Default for DynaRouteConfig {
    fn default() -> Self {
        Self {
            iaav6_base_prefix: "2001:db8:03ba::/64".to_string(),
            srv6_policy_table: 200,
            hrw_max_vpods_per_service: 256,
            epoch_rotation_seconds: 3600,
            realm: "production".to_string(),
            blakepage_url: "http://localhost:8090".to_string(),
            merkle_sync_interval_ms: 1000,
            ebpf_enabled: false,
        }
    }
}

/// Virtual address for vPod
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VirtualAddress {
    /// Identity-anycast IPv6 address
    pub iaav6: Ipv6Addr,
    
    /// vPod identifier
    pub vpod_id: String,
    
    /// Service identifier
    pub service_id: String,
    
    /// Holder address (e.g., "consensus.bpci.local")
    pub holder_address: String,
    
    /// Blake3 holder hash
    pub holder_hash: [u8; 32],
    
    /// Merkle proof of assignment
    pub merkle_proof: MerkleProof,
    
    /// QUIC connection ID
    pub quic_conn_id: u64,
    
    /// Epoch
    pub epoch: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = DynaRouteConfig::default();
        assert_eq!(config.realm, "production");
        assert_eq!(config.srv6_policy_table, 200);
    }
}
