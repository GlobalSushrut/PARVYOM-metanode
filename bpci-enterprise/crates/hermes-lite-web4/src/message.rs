//! BPCI-First Message Types for Web-4 Testnet

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// BPCI traffic classes for priority routing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrafficClass {
    /// C1: Consensus (IBFT/HotStuff votes, headers) - HIGHEST PRIORITY
    Consensus,
    /// C2: Auction/Roundtable (BPCI auctions, coordination) - MEDIUM PRIORITY  
    Auction,
    /// C3: Shadow Data (swarm/torrent-like) - BACKGROUND PRIORITY
    ShadowData,
}

/// Message types for BPCI integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    // Consensus messages
    IbftPrepare,
    IbftCommit,
    HotStuffVote,
    
    // Auction messages
    AuctionBid,
    AuctionResult,
    RoundtableCoordination,
    
    // Data messages
    BlockData,
    TransactionPool,
    ShadowSync,
    
    // Control messages
    Heartbeat,
    NeighborDiscovery,
}

/// Simple P2P message for testnet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PMessage {
    pub id: String,
    pub traffic_class: TrafficClass,
    pub message_type: MessageType,
    pub payload: Vec<u8>,
    pub timestamp: SystemTime,
}

impl P2PMessage {
    /// Create consensus message (highest priority)
    pub fn consensus(message_type: MessageType, payload: Vec<u8>) -> Self {
        Self {
            id: format!("msg_{}", rand::random::<u64>()),
            traffic_class: TrafficClass::Consensus,
            message_type,
            payload,
            timestamp: SystemTime::now(),
        }
    }
    
    /// Create auction message (medium priority)
    pub fn auction(message_type: MessageType, payload: Vec<u8>) -> Self {
        Self {
            id: format!("msg_{}", rand::random::<u64>()),
            traffic_class: TrafficClass::Auction,
            message_type,
            payload,
            timestamp: SystemTime::now(),
        }
    }
    
    /// Create shadow data message (background priority)
    pub fn shadow_data(message_type: MessageType, payload: Vec<u8>) -> Self {
        Self {
            id: format!("msg_{}", rand::random::<u64>()),
            traffic_class: TrafficClass::ShadowData,
            message_type,
            payload,
            timestamp: SystemTime::now(),
        }
    }
    
    /// Get priority level (lower number = higher priority)
    pub fn priority(&self) -> u8 {
        match self.traffic_class {
            TrafficClass::Consensus => 0,    // Highest priority
            TrafficClass::Auction => 1,     // Medium priority
            TrafficClass::ShadowData => 2,  // Lowest priority
        }
    }
}
