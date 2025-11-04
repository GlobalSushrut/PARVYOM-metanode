//! Message structures for commute.lock
//! 
//! Defines message format for inter-component communication

use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// Message type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    /// Data message
    Data,
    /// Control message
    Control,
    /// Event notification
    Event,
    /// Request message
    Request,
    /// Response message
    Response,
    /// Broadcast message
    Broadcast,
}

/// Message header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHeader {
    /// Message type
    pub msg_type: MessageType,
    /// Source component
    pub source: String,
    /// Target component
    pub target: String,
    /// Message ID
    pub message_id: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Data length
    pub data_len: usize,
}

/// Message for inter-component communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message header
    pub header: MessageHeader,
    /// Message data
    pub data: Vec<u8>,
}

impl Message {
    /// Create new message
    pub fn new(
        msg_type: MessageType,
        source: &str,
        target: &str,
        data: &[u8],
    ) -> Self {
        let message_id = uuid::Uuid::new_v4().to_string();
        
        Self {
            header: MessageHeader {
                msg_type,
                source: source.to_string(),
                target: target.to_string(),
                message_id,
                timestamp: Utc::now(),
                data_len: data.len(),
            },
            data: data.to_vec(),
        }
    }
    
    /// Create request message
    pub fn request(source: &str, target: &str, data: &[u8]) -> Self {
        Self::new(MessageType::Request, source, target, data)
    }
    
    /// Create response message
    pub fn response(source: &str, target: &str, data: &[u8]) -> Self {
        Self::new(MessageType::Response, source, target, data)
    }
    
    /// Create event message
    pub fn event(source: &str, target: &str, data: &[u8]) -> Self {
        Self::new(MessageType::Event, source, target, data)
    }
    
    /// Create broadcast message
    pub fn broadcast(source: &str, data: &[u8]) -> Self {
        Self::new(MessageType::Broadcast, source, "all", data)
    }
    
    /// Serialize message to bytes
    pub fn serialize(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self)
            .map_err(|e| anyhow!("Failed to serialize message: {}", e))
    }
    
    /// Deserialize message from bytes
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        serde_json::from_slice(data)
            .map_err(|e| anyhow!("Failed to deserialize message: {}", e))
    }
    
    /// Get message size
    pub fn size(&self) -> usize {
        self.serialize().map(|v| v.len()).unwrap_or(0)
    }
    
    /// Get message type
    pub fn msg_type(&self) -> &MessageType {
        &self.header.msg_type
    }
    
    /// Get source component
    pub fn source(&self) -> &str {
        &self.header.source
    }
    
    /// Get target component
    pub fn target(&self) -> &str {
        &self.header.target
    }
    
    /// Get message ID
    pub fn message_id(&self) -> &str {
        &self.header.message_id
    }
    
    /// Get timestamp
    pub fn timestamp(&self) -> DateTime<Utc> {
        self.header.timestamp
    }
    
    /// Get data
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_message_creation() {
        let data = b"Hello, commute.lock!";
        let msg = Message::new(
            MessageType::Data,
            "blockchain",
            "cluster_ledger",
            data,
        );
        
        assert_eq!(msg.source(), "blockchain");
        assert_eq!(msg.target(), "cluster_ledger");
        assert_eq!(msg.data(), data);
        assert_eq!(msg.msg_type(), &MessageType::Data);
    }
    
    #[test]
    fn test_message_serialize_deserialize() {
        let data = b"Test message data";
        let msg = Message::new(
            MessageType::Request,
            "consensus",
            "blockchain",
            data,
        );
        
        let serialized = msg.serialize().unwrap();
        let deserialized = Message::deserialize(&serialized).unwrap();
        
        assert_eq!(msg.source(), deserialized.source());
        assert_eq!(msg.target(), deserialized.target());
        assert_eq!(msg.data(), deserialized.data());
        assert_eq!(msg.msg_type(), deserialized.msg_type());
    }
    
    #[test]
    fn test_broadcast_message() {
        let data = b"Broadcast event";
        let msg = Message::broadcast("cluster_ledger", data);
        
        assert_eq!(msg.msg_type(), &MessageType::Broadcast);
        assert_eq!(msg.target(), "all");
    }
}
