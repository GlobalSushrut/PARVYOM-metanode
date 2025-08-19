// Shared Networking Layer
// P2P networking layer shared between BPI Core and BPCI Enterprise

//! # Networking
//! 
//! P2P networking layer providing consistent network communication
//! across both community and enterprise products.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::net::{TcpListener, TcpStream};
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Bind failed: {0}")]
    BindFailed(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Invalid peer address: {0}")]
    InvalidPeerAddress(String),
    #[error("Network timeout")]
    Timeout,
}

/// Network message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Ping,
    Pong,
    Data(Vec<u8>),
    Handshake,
    Disconnect,
}

/// Network message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
    pub id: Uuid,
    pub message_type: MessageType,
    pub timestamp: u64,
    pub sender: String,
}

/// Peer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub id: Uuid,
    pub address: SocketAddr,
    pub connected_at: u64,
    pub last_seen: u64,
}

/// Network node trait
#[async_trait::async_trait]
pub trait NetworkNode {
    async fn start(&mut self, bind_addr: SocketAddr) -> Result<(), NetworkError>;
    async fn connect_peer(&mut self, addr: SocketAddr) -> Result<(), NetworkError>;
    async fn send_message(&self, peer_id: Uuid, message: NetworkMessage) -> Result<(), NetworkError>;
    async fn broadcast_message(&self, message: NetworkMessage) -> Result<(), NetworkError>;
    fn get_peers(&self) -> Vec<PeerInfo>;
}

/// Simple P2P network implementation
#[derive(Debug)]
pub struct P2PNetwork {
    pub node_id: Uuid,
    pub peers: std::collections::HashMap<Uuid, PeerInfo>,
    pub listener: Option<TcpListener>,
}

impl P2PNetwork {
    /// Create new P2P network
    pub fn new() -> Self {
        P2PNetwork {
            node_id: Uuid::new_v4(),
            peers: std::collections::HashMap::new(),
            listener: None,
        }
    }

    /// Get node ID
    pub fn node_id(&self) -> Uuid {
        self.node_id
    }

    /// Get peer count
    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }
}

#[async_trait::async_trait]
impl NetworkNode for P2PNetwork {
    async fn start(&mut self, bind_addr: SocketAddr) -> Result<(), NetworkError> {
        let listener = TcpListener::bind(bind_addr).await
            .map_err(|e| NetworkError::BindFailed(e.to_string()))?;
        
        self.listener = Some(listener);
        Ok(())
    }

    async fn connect_peer(&mut self, addr: SocketAddr) -> Result<(), NetworkError> {
        let _stream = TcpStream::connect(addr).await
            .map_err(|e| NetworkError::ConnectionFailed(e.to_string()))?;
        
        let peer_info = PeerInfo {
            id: Uuid::new_v4(),
            address: addr,
            connected_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_seen: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        self.peers.insert(peer_info.id, peer_info);
        Ok(())
    }

    async fn send_message(&self, _peer_id: Uuid, _message: NetworkMessage) -> Result<(), NetworkError> {
        // Implementation would send message to specific peer
        Ok(())
    }

    async fn broadcast_message(&self, _message: NetworkMessage) -> Result<(), NetworkError> {
        // Implementation would broadcast to all peers
        Ok(())
    }

    fn get_peers(&self) -> Vec<PeerInfo> {
        self.peers.values().cloned().collect()
    }
}

impl Default for P2PNetwork {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2p_network_creation() {
        let network = P2PNetwork::new();
        assert_eq!(network.peer_count(), 0);
    }

    #[test]
    fn test_network_message() {
        let message = NetworkMessage {
            id: Uuid::new_v4(),
            message_type: MessageType::Ping,
            timestamp: 12345,
            sender: "test".to_string(),
        };
        
        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: NetworkMessage = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(message.id, deserialized.id);
        assert_eq!(message.timestamp, deserialized.timestamp);
    }
}
