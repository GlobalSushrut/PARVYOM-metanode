//! Simple Node Implementation for Web-4 Testnet

use serde::{Deserialize, Serialize};
use std::fmt;

/// Simple node identifier (no complex crypto for testnet)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub String);

impl NodeId {
    /// Generate random node ID for testnet
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let id: u64 = rng.gen();
        Self(format!("node_{:016x}", id))
    }
    
    /// Create from string
    pub fn from_string(s: String) -> Self {
        Self(s)
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Simple P2P node for testnet
#[derive(Debug)]
pub struct P2PNode {
    pub id: NodeId,
    pub created_at: std::time::SystemTime,
}

impl P2PNode {
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            created_at: std::time::SystemTime::now(),
        }
    }
    
    pub fn id(&self) -> &NodeId {
        &self.id
    }
}
