//! NX Network Plane abstraction
//!
//! This module provides a thin, honest wrapper around the core network
//! components used by the universal BPI OS kernel. It does **not** introduce
//! new behaviour yet; it simply centralizes configuration and wiring so that
//! higher layers can reason about "NX Network" as a first-class OS concept.

use std::sync::Arc;
use tracing::info;

use crate::blockchain_os_kernel::commute_link::CommuteLink;
use crate::config::{KernelConfig, is_mesh_internal_enabled};
use crate::vm_server::VmServerConfig;

/// Types of NX lanes exposed by the OS-level network plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NxLaneKind {
    /// HTTP Cage / httpcg edge gateway
    HttpCage,
    /// XTMP BPCI high-throughput transport
    XtmpBpci,
    /// Internal mesh-native communication via CommuteLink/CommuteLock
    MeshInternal,
    /// Shadow Registry Web2↔Web3 bridge
    ShadowRegistry,
}

/// Configuration for a single NX lane.
#[derive(Debug, Clone)]
pub struct NxLane {
    pub kind: NxLaneKind,
    /// Human-readable description of the lane
    pub description: String,
    /// Bind address or endpoint (e.g. "0.0.0.0:8888" or URL)
    pub endpoint: String,
}

/// OS-level view of the networking plane for a single kernel instance.
#[derive(Debug, Clone)]
pub struct NxNetworkPlane {
    /// Kernel profile (pilot/devnet/mainnet/etc.)
    pub profile: String,
    /// Node identifier used across mesh and services
    pub node_id: String,
    /// Core mesh-native communication fabric
    pub commute_link: Arc<CommuteLink>,
    /// VM server network configuration for this node/profile
    pub vm_config: VmServerConfig,
    /// Declared NX lanes for this node/profile
    pub lanes: Vec<NxLane>,
    /// Whether internal mesh-native paths are enabled for this node/profile
    /// (BPI_MESH_INTERNAL_ENABLED=true/1/yes/on).
    pub mesh_internal_enabled: bool,
}

impl NxNetworkPlane {
    /// Construct an NX Network Plane from a loaded KernelConfig and an
    /// already-initialized CommuteLink instance.
    ///
    /// Behaviour:
    /// - Keeps `VmServerConfig::default()` intact so existing runtime
    ///   behaviour is unchanged.
    /// - Derives lane endpoints from existing config (bind address, ports,
    ///   VM server defaults) and only **reports** them via logs/structs.
    pub fn new_from_kernel_config(
        kernel_config: &KernelConfig,
        commute_link: Arc<CommuteLink>,
    ) -> Self {
        // Preserve existing VM server behaviour by using its default config.
        let vm_config = VmServerConfig::default();

        let bind_host = &kernel_config.bpi.network.bind_address;

        // Derive representative endpoints for the main NX lanes. These are
        // informational only for now; actual servers are still started where
        // they were before.
        let vm_addr = format!("{}:{}", bind_host, vm_config.vm_port);
        let http_cage_addr = format!("{}:{}", bind_host, vm_config.http_cage_port);
        let xtmp_addr = format!(
            "{}:{}",
            bind_host,
            kernel_config.bpi.network.bpci_port,
        );
        let shadow_endpoint = vm_config.shadow_registry_endpoint.clone();

        let lanes = vec![
            NxLane {
                kind: NxLaneKind::HttpCage,
                description: "HTTP Cage / httpcg edge".to_string(),
                endpoint: http_cage_addr.clone(),
            },
            NxLane {
                kind: NxLaneKind::XtmpBpci,
                description: "XTMP BPCI high-throughput lane".to_string(),
                endpoint: xtmp_addr.clone(),
            },
            NxLane {
                kind: NxLaneKind::MeshInternal,
                description: "Internal mesh-native communication (CommuteLink)".to_string(),
                endpoint: vm_addr.clone(),
            },
            NxLane {
                kind: NxLaneKind::ShadowRegistry,
                description: "Shadow Registry Web2↔Web3 bridge".to_string(),
                endpoint: shadow_endpoint.clone(),
            },
        ];

        let mesh_enabled = is_mesh_internal_enabled();

        info!(
            profile = %kernel_config.profile,
            node_id = %kernel_config.node_id,
            vm_addr = %vm_addr,
            http_cage = %http_cage_addr,
            xtmp = %xtmp_addr,
            shadow_registry = %shadow_endpoint,
            mesh_internal_enabled = mesh_enabled,
            "NX Network Plane initialized for kernel profile",
        );

        Self {
            profile: kernel_config.profile.clone(),
            node_id: kernel_config.node_id.clone(),
            commute_link,
            vm_config,
            lanes,
            mesh_internal_enabled: mesh_enabled,
        }
    }

    /// Get the configuration for a specific NX lane kind.
    pub fn get_lane(&self, kind: NxLaneKind) -> Option<&NxLane> {
        self.lanes.iter().find(|lane| lane.kind == kind)
    }

    /// Returns true if a lane of the given kind is declared for this
    /// profile/node.
    pub fn has_lane(&self, kind: NxLaneKind) -> bool {
        self.get_lane(kind).is_some()
    }
}
