// QGC Wire - Fixed, audit-friendly wire protocol for QGC-C² consensus
// Zero-copy I/O, fixed opcodes, single-version frames

use crate::logbook_6d_bridge::qgc_core::*;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write, Result as IoResult};
use std::net::{TcpStream, UdpSocket, SocketAddr};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::{TcpListener, UdpSocket as TokioUdpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Wire protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireConfig {
    pub max_frame_size: usize,           // Maximum frame size (e.g., 4KB)
    pub buffer_size: usize,              // Fixed buffer size (256KB)
    pub max_connections: usize,          // Maximum concurrent connections
    pub timeout_ms: u64,                 // Network timeout
    pub enable_compression: bool,        // Enable frame compression
    pub enable_encryption: bool,         // Enable frame encryption
}

impl Default for WireConfig {
    fn default() -> Self {
        Self {
            max_frame_size: 4096,        // 4KB max frame
            buffer_size: 262144,         // 256KB buffer
            max_connections: 100,        // 100 concurrent connections
            timeout_ms: 5000,            // 5 second timeout
            enable_compression: false,   // Disabled for simplicity
            enable_encryption: false,    // Disabled for simplicity
        }
    }
}

/// Wire protocol opcodes (frozen, never mutate)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum WireOpcode {
    V1CA = 0x01,                         // V1 Confidence Attestation
    V1CC = 0x02,                         // V1 Confidence Certificate
    V1HDR = 0x03,                        // V1 Header
    V1KNT = 0x04,                        // V1 Knot (telemetry)
    V1BATCH = 0x05,                      // V1 Batch
    V1PING = 0x06,                       // V1 Ping
    V1PONG = 0x07,                       // V1 Pong
    V1ERROR = 0xFF,                      // V1 Error
}

impl WireOpcode {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(WireOpcode::V1CA),
            0x02 => Some(WireOpcode::V1CC),
            0x03 => Some(WireOpcode::V1HDR),
            0x04 => Some(WireOpcode::V1KNT),
            0x05 => Some(WireOpcode::V1BATCH),
            0x06 => Some(WireOpcode::V1PING),
            0x07 => Some(WireOpcode::V1PONG),
            0xFF => Some(WireOpcode::V1ERROR),
            _ => None,
        }
    }
}

/// Wire frame header (8 bytes, fixed)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireFrameHeader {
    pub opcode: WireOpcode,              // 1B - frame type
    pub version: u8,                     // 1B - protocol version (always 1)
    pub flags: u16,                      // 2B - frame flags
    pub payload_len: u32,                // 4B - payload length
}

impl WireFrameHeader {
    pub const SIZE: usize = 8;
    
    pub fn new(opcode: WireOpcode, payload_len: u32) -> Self {
        Self {
            opcode,
            version: 1,
            flags: 0,
            payload_len,
        }
    }
    
    pub fn to_bytes(&self) -> [u8; 8] {
        let mut bytes = [0u8; 8];
        bytes[0] = self.opcode as u8;
        bytes[1] = self.version;
        bytes[2..4].copy_from_slice(&self.flags.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.payload_len.to_le_bytes());
        bytes
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 8 {
            return Err("Insufficient bytes for frame header".to_string());
        }
        
        let opcode = WireOpcode::from_u8(bytes[0])
            .ok_or("Invalid opcode")?;
        let version = bytes[1];
        let flags = u16::from_le_bytes([bytes[2], bytes[3]]);
        let payload_len = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        
        if version != 1 {
            return Err(format!("Unsupported version: {}", version));
        }
        
        Ok(Self {
            opcode,
            version,
            flags,
            payload_len,
        })
    }
}

/// Wire frame (header + payload)
#[derive(Debug, Clone)]
pub struct WireFrame {
    pub header: WireFrameHeader,
    pub payload: Vec<u8>,
}

impl WireFrame {
    pub fn new(opcode: WireOpcode, payload: Vec<u8>) -> Result<Self, String> {
        if payload.len() > u32::MAX as usize {
            return Err("Payload too large".to_string());
        }
        
        let header = WireFrameHeader::new(opcode, payload.len() as u32);
        Ok(Self { header, payload })
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(WireFrameHeader::SIZE + self.payload.len());
        bytes.extend_from_slice(&self.header.to_bytes());
        bytes.extend_from_slice(&self.payload);
        bytes
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < WireFrameHeader::SIZE {
            return Err("Insufficient bytes for frame".to_string());
        }
        
        let header = WireFrameHeader::from_bytes(&bytes[..WireFrameHeader::SIZE])?;
        
        if bytes.len() < WireFrameHeader::SIZE + header.payload_len as usize {
            return Err("Insufficient bytes for payload".to_string());
        }
        
        let payload = bytes[WireFrameHeader::SIZE..WireFrameHeader::SIZE + header.payload_len as usize].to_vec();
        
        Ok(Self { header, payload })
    }
    
    pub fn size(&self) -> usize {
        WireFrameHeader::SIZE + self.payload.len()
    }
}

/// Wire message types
#[derive(Debug, Clone)]
pub enum WireMessage {
    ConfidenceAttestation(ConfidenceAttestation),
    ConfidenceCertificate(ConfidenceCertificate),
    Header(QgcHeader),
    Knot(KnotMetric),
    Batch(Batch),
    Ping { timestamp: u64 },
    Pong { timestamp: u64 },
    Error { code: u16, message: String },
}

impl WireMessage {
    pub fn to_frame(&self) -> Result<WireFrame, String> {
        match self {
            WireMessage::ConfidenceAttestation(ca) => {
                let payload = bincode::serialize(ca)
                    .map_err(|e| format!("CA serialization error: {}", e))?;
                WireFrame::new(WireOpcode::V1CA, payload)
            },
            WireMessage::ConfidenceCertificate(cc) => {
                let payload = bincode::serialize(cc)
                    .map_err(|e| format!("CC serialization error: {}", e))?;
                WireFrame::new(WireOpcode::V1CC, payload)
            },
            WireMessage::Header(header) => {
                let payload = bincode::serialize(header)
                    .map_err(|e| format!("Header serialization error: {}", e))?;
                WireFrame::new(WireOpcode::V1HDR, payload)
            },
            WireMessage::Knot(knot) => {
                let payload = bincode::serialize(knot)
                    .map_err(|e| format!("Knot serialization error: {}", e))?;
                WireFrame::new(WireOpcode::V1KNT, payload)
            },
            WireMessage::Batch(batch) => {
                let payload = bincode::serialize(batch)
                    .map_err(|e| format!("Batch serialization error: {}", e))?;
                WireFrame::new(WireOpcode::V1BATCH, payload)
            },
            WireMessage::Ping { timestamp } => {
                let payload = timestamp.to_le_bytes().to_vec();
                WireFrame::new(WireOpcode::V1PING, payload)
            },
            WireMessage::Pong { timestamp } => {
                let payload = timestamp.to_le_bytes().to_vec();
                WireFrame::new(WireOpcode::V1PONG, payload)
            },
            WireMessage::Error { code, message } => {
                let mut payload = Vec::new();
                payload.extend_from_slice(&code.to_le_bytes());
                payload.extend_from_slice(message.as_bytes());
                WireFrame::new(WireOpcode::V1ERROR, payload)
            },
        }
    }
    
    pub fn from_frame(frame: &WireFrame) -> Result<Self, String> {
        match frame.header.opcode {
            WireOpcode::V1CA => {
                let ca = bincode::deserialize(&frame.payload)
                    .map_err(|e| format!("CA deserialization error: {}", e))?;
                Ok(WireMessage::ConfidenceAttestation(ca))
            },
            WireOpcode::V1CC => {
                let cc = bincode::deserialize(&frame.payload)
                    .map_err(|e| format!("CC deserialization error: {}", e))?;
                Ok(WireMessage::ConfidenceCertificate(cc))
            },
            WireOpcode::V1HDR => {
                let header = bincode::deserialize(&frame.payload)
                    .map_err(|e| format!("Header deserialization error: {}", e))?;
                Ok(WireMessage::Header(header))
            },
            WireOpcode::V1KNT => {
                let knot = bincode::deserialize(&frame.payload)
                    .map_err(|e| format!("Knot deserialization error: {}", e))?;
                Ok(WireMessage::Knot(knot))
            },
            WireOpcode::V1BATCH => {
                let batch = bincode::deserialize(&frame.payload)
                    .map_err(|e| format!("Batch deserialization error: {}", e))?;
                Ok(WireMessage::Batch(batch))
            },
            WireOpcode::V1PING => {
                if frame.payload.len() != 8 {
                    return Err("Invalid ping payload size".to_string());
                }
                let timestamp = u64::from_le_bytes(frame.payload[..8].try_into().unwrap());
                Ok(WireMessage::Ping { timestamp })
            },
            WireOpcode::V1PONG => {
                if frame.payload.len() != 8 {
                    return Err("Invalid pong payload size".to_string());
                }
                let timestamp = u64::from_le_bytes(frame.payload[..8].try_into().unwrap());
                Ok(WireMessage::Pong { timestamp })
            },
            WireOpcode::V1ERROR => {
                if frame.payload.len() < 2 {
                    return Err("Invalid error payload size".to_string());
                }
                let code = u16::from_le_bytes([frame.payload[0], frame.payload[1]]);
                let message = String::from_utf8_lossy(&frame.payload[2..]).to_string();
                Ok(WireMessage::Error { code, message })
            },
        }
    }
}

/// Fixed buffer pool for zero-copy I/O
#[derive(Debug)]
pub struct BufferPool {
    buffers: Vec<Vec<u8>>,
    config: WireConfig,
}

impl BufferPool {
    pub fn new(config: WireConfig) -> Self {
        let mut buffers = Vec::new();
        
        // Pre-allocate 4 fixed buffers as per spec
        for _ in 0..4 {
            buffers.push(vec![0u8; config.buffer_size]);
        }
        
        Self { buffers, config }
    }
    
    pub fn get_buffer(&mut self) -> Option<Vec<u8>> {
        self.buffers.pop()
    }
    
    pub fn return_buffer(&mut self, mut buffer: Vec<u8>) {
        if buffer.len() == self.config.buffer_size {
            buffer.clear();
            buffer.resize(self.config.buffer_size, 0);
            self.buffers.push(buffer);
        }
        // Drop buffer if wrong size
    }
    
    pub fn get_memory_usage(&self) -> usize {
        self.buffers.len() * self.config.buffer_size
    }
}

/// Network connection handler
#[derive(Debug)]
pub struct NetworkConnection {
    peer_addr: SocketAddr,
    last_seen: u64,
    bytes_sent: u64,
    bytes_received: u64,
    frames_sent: u64,
    frames_received: u64,
}

impl NetworkConnection {
    pub fn new(peer_addr: SocketAddr) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        Self {
            peer_addr,
            last_seen: now,
            bytes_sent: 0,
            bytes_received: 0,
            frames_sent: 0,
            frames_received: 0,
        }
    }
    
    pub fn update_activity(&mut self) {
        self.last_seen = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    pub fn record_sent(&mut self, bytes: usize) {
        self.bytes_sent += bytes as u64;
        self.frames_sent += 1;
        self.update_activity();
    }
    
    pub fn record_received(&mut self, bytes: usize) {
        self.bytes_received += bytes as u64;
        self.frames_received += 1;
        self.update_activity();
    }
}

/// QGC Wire Protocol Engine
pub struct QgcWireEngine {
    config: WireConfig,
    buffer_pool: Arc<RwLock<BufferPool>>,
    connections: Arc<RwLock<HashMap<SocketAddr, NetworkConnection>>>,
    message_handlers: HashMap<WireOpcode, Box<dyn Fn(&WireMessage) -> Result<(), String> + Send + Sync>>,
}

impl std::fmt::Debug for QgcWireEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QgcWireEngine")
            .field("config", &self.config)
            .field("buffer_pool", &self.buffer_pool)
            .field("connections", &self.connections)
            .field("message_handlers", &format!("{} handlers", self.message_handlers.len()))
            .finish()
    }
}

impl QgcWireEngine {
    pub fn new(config: WireConfig) -> Self {
        let buffer_pool = Arc::new(RwLock::new(BufferPool::new(config.clone())));
        
        Self {
            config,
            buffer_pool,
            connections: Arc::new(RwLock::new(HashMap::new())),
            message_handlers: HashMap::new(),
        }
    }
    
    /// Send message to peer
    pub async fn send_message(&self, peer: SocketAddr, message: WireMessage) -> Result<(), String> {
        let frame = message.to_frame()?;
        let frame_bytes = frame.to_bytes();
        
        if frame_bytes.len() > self.config.max_frame_size {
            return Err(format!("Frame too large: {} > {}", frame_bytes.len(), self.config.max_frame_size));
        }
        
        // Send via UDP (simplified - would use proper networking in production)
        let socket = UdpSocket::bind("0.0.0.0:0")
            .map_err(|e| format!("Socket bind error: {}", e))?;
        
        socket.send_to(&frame_bytes, peer)
            .map_err(|e| format!("Send error: {}", e))?;
        
        // Update connection stats
        if let Ok(mut connections) = self.connections.write() {
            let conn = connections.entry(peer).or_insert_with(|| NetworkConnection::new(peer));
            conn.record_sent(frame_bytes.len());
        }
        
        Ok(())
    }
    
    /// Receive and process messages
    pub async fn receive_messages(&self, bind_addr: SocketAddr) -> Result<(), String> {
        let socket = TokioUdpSocket::bind(bind_addr).await
            .map_err(|e| format!("Bind error: {}", e))?;
        
        let mut buffer = vec![0u8; self.config.max_frame_size];
        
        loop {
            match socket.recv_from(&mut buffer).await {
                Ok((size, peer)) => {
                    if let Err(e) = self.process_received_data(&buffer[..size], peer).await {
                        eprintln!("Error processing message from {}: {}", peer, e);
                    }
                },
                Err(e) => {
                    eprintln!("Receive error: {}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// Process received data
    async fn process_received_data(&self, data: &[u8], peer: SocketAddr) -> Result<(), String> {
        let frame = WireFrame::from_bytes(data)?;
        let message = WireMessage::from_frame(&frame)?;
        
        // Update connection stats
        if let Ok(mut connections) = self.connections.write() {
            let conn = connections.entry(peer).or_insert_with(|| NetworkConnection::new(peer));
            conn.record_received(data.len());
        }
        
        // Handle message based on type
        match &message {
            WireMessage::Ping { timestamp } => {
                // Respond with pong
                let pong = WireMessage::Pong { timestamp: *timestamp };
                let _ = self.send_message(peer, pong).await;
            },
            WireMessage::Pong { .. } => {
                // Update latency metrics (simplified)
            },
            _ => {
                // Handle other message types
                if let Some(handler) = self.message_handlers.get(&frame.header.opcode) {
                    handler(&message)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Register message handler
    pub fn register_handler<F>(&mut self, opcode: WireOpcode, handler: F)
    where
        F: Fn(&WireMessage) -> Result<(), String> + Send + Sync + 'static,
    {
        self.message_handlers.insert(opcode, Box::new(handler));
    }
    
    /// Get network statistics
    pub fn get_network_stats(&self) -> NetworkStats {
        let connections = self.connections.read().unwrap();
        
        let total_connections = connections.len();
        let total_bytes_sent: u64 = connections.values().map(|c| c.bytes_sent).sum();
        let total_bytes_received: u64 = connections.values().map(|c| c.bytes_received).sum();
        let total_frames_sent: u64 = connections.values().map(|c| c.frames_sent).sum();
        let total_frames_received: u64 = connections.values().map(|c| c.frames_received).sum();
        
        let buffer_pool_usage = self.buffer_pool.read().unwrap().get_memory_usage();
        
        NetworkStats {
            total_connections,
            total_bytes_sent,
            total_bytes_received,
            total_frames_sent,
            total_frames_received,
            buffer_pool_usage,
        }
    }
    
    /// Cleanup old connections
    pub fn cleanup_connections(&self, max_age_secs: u64) {
        let mut connections = self.connections.write().unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        connections.retain(|_, conn| now - conn.last_seen < max_age_secs);
    }
}

/// Network statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub total_connections: usize,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
    pub total_frames_sent: u64,
    pub total_frames_received: u64,
    pub buffer_pool_usage: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wire_frame_header() {
        let header = WireFrameHeader::new(WireOpcode::V1CA, 236);
        let bytes = header.to_bytes();
        let decoded = WireFrameHeader::from_bytes(&bytes).unwrap();
        
        assert_eq!(decoded.opcode, WireOpcode::V1CA);
        assert_eq!(decoded.version, 1);
        assert_eq!(decoded.payload_len, 236);
    }
    
    #[test]
    fn test_wire_frame() {
        let payload = vec![1, 2, 3, 4, 5];
        let frame = WireFrame::new(WireOpcode::V1PING, payload.clone()).unwrap();
        let bytes = frame.to_bytes();
        let decoded = WireFrame::from_bytes(&bytes).unwrap();
        
        assert_eq!(decoded.header.opcode, WireOpcode::V1PING);
        assert_eq!(decoded.payload, payload);
    }
    
    #[test]
    fn test_wire_message_serialization() {
        let ping = WireMessage::Ping { timestamp: 12345 };
        let frame = ping.to_frame().unwrap();
        let decoded = WireMessage::from_frame(&frame).unwrap();
        
        match decoded {
            WireMessage::Ping { timestamp } => assert_eq!(timestamp, 12345),
            _ => panic!("Wrong message type"),
        }
    }
    
    #[test]
    fn test_buffer_pool() {
        let config = WireConfig::default();
        let mut pool = BufferPool::new(config.clone());
        
        assert_eq!(pool.buffers.len(), 4);
        
        let buffer = pool.get_buffer().unwrap();
        assert_eq!(buffer.len(), config.buffer_size);
        assert_eq!(pool.buffers.len(), 3);
        
        pool.return_buffer(buffer);
        assert_eq!(pool.buffers.len(), 4);
    }
    
    #[test]
    fn test_network_connection() {
        let addr = "127.0.0.1:8080".parse().unwrap();
        let mut conn = NetworkConnection::new(addr);
        
        assert_eq!(conn.peer_addr, addr);
        assert_eq!(conn.bytes_sent, 0);
        
        conn.record_sent(100);
        assert_eq!(conn.bytes_sent, 100);
        assert_eq!(conn.frames_sent, 1);
    }
    
    #[test]
    fn test_wire_engine_creation() {
        let config = WireConfig::default();
        let engine = QgcWireEngine::new(config);
        
        let stats = engine.get_network_stats();
        assert_eq!(stats.total_connections, 0);
        assert!(stats.buffer_pool_usage > 0);
    }
}
