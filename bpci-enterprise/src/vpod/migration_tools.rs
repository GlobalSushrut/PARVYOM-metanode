//! # vPod Migration Tools
//! 
//! Tools for migrating legacy nodes to vPod-based implementations.

pub use crate::vpod::vpod_node::VPodNode;

/// Migration utilities re-exported from vpod_node
pub use crate::vpod::vpod_node::VPodNode as MigrationTarget;
