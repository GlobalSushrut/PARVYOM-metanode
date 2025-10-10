//! Core Production Types for BPCI Enterprise
//! 
//! This module contains the fundamental, production-grade data types
//! that form the foundation of the BPCI system.

use serde::{Serialize, Deserialize};
use std::fmt;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Unique identifier for nodes in the BPCI network
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(String);

impl NodeId {
    /// Create a new random NodeId
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
    
    /// Create NodeId from existing string
    pub fn from_string(id: String) -> Self {
        Self(id)
    }
    
    /// Get the inner string value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Transaction identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransactionId(String);

impl TransactionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
    
    pub fn from_string(id: String) -> Self {
        Self(id)
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TransactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Block height in the blockchain
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BlockHeight(pub u64);

impl BlockHeight {
    pub fn genesis() -> Self {
        Self(0)
    }
    
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
    
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for BlockHeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Timestamp wrapper for consistent time handling
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    pub fn now() -> Self {
        Self(Utc::now())
    }
    
    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }
    
    pub fn inner(&self) -> &DateTime<Utc> {
        &self.0
    }
    
    pub fn unix_timestamp(&self) -> i64 {
        self.0.timestamp()
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d %H:%M:%S UTC"))
    }
}

/// Network address for peer communication
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NetworkAddress {
    pub host: String,
    pub port: u16,
}

impl NetworkAddress {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }
    
    pub fn localhost(port: u16) -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port,
        }
    }
}

impl fmt::Display for NetworkAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

/// Status of a node in the network
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    Online,
    Offline,
    Syncing,
    Error(String),
}

impl fmt::Display for NodeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeStatus::Online => write!(f, "Online"),
            NodeStatus::Offline => write!(f, "Offline"),
            NodeStatus::Syncing => write!(f, "Syncing"),
            NodeStatus::Error(msg) => write!(f, "Error: {}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_id_creation() {
        let id1 = NodeId::new();
        let id2 = NodeId::new();
        assert_ne!(id1, id2);
        assert!(!id1.as_str().is_empty());
    }

    #[test]
    fn test_block_height_ordering() {
        let h1 = BlockHeight(10);
        let h2 = BlockHeight(20);
        assert!(h1 < h2);
        assert_eq!(h1.next(), BlockHeight(11));
    }

    #[test]
    fn test_timestamp_ordering() {
        let t1 = Timestamp::now();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let t2 = Timestamp::now();
        assert!(t1 < t2);
    }

    #[test]
    fn test_network_address() {
        let addr = NetworkAddress::localhost(8080);
        assert_eq!(addr.to_string(), "127.0.0.1:8080");
    }
}
