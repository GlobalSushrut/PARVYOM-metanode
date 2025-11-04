//! # QUIC Connection ID Encoding
//! 
//! Custom ConnectionId format for DynaRoute: {trace_id, realm, qos}
//! Enables connection migration and flow tracking without ports.

use serde::{Serialize, Deserialize};

/// Connection ID type (16 bytes)
pub type ConnectionId = [u8; 16];

/// BPCI Connection ID structure
/// 
/// Layout (16 bytes):
/// - Bytes 0-7: trace_id (64 bits) - unique flow identifier
/// - Bytes 8-9: realm (16 bits) - production/staging/canary
/// - Byte 10: qos (8 bits) - quality of service class
/// - Byte 11: reserved (8 bits) - future use
/// - Bytes 12-15: checksum/version (32 bits)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BpciConnectionId {
    /// Unique flow identifier
    pub trace_id: u64,
    
    /// Realm identifier
    pub realm: u16,
    
    /// Quality of Service class
    pub qos: u8,
    
    /// Reserved for future use
    pub reserved: u8,
}

impl BpciConnectionId {
    /// Create new connection ID
    pub fn new(trace_id: u64, realm: u16, qos: u8) -> Self {
        Self {
            trace_id,
            realm,
            qos,
            reserved: 0,
        }
    }
    
    /// Encode to QUIC ConnectionId
    pub fn encode(&self) -> ConnectionId {
        let mut bytes = [0u8; 16];
        
        // Trace ID (8 bytes)
        bytes[0..8].copy_from_slice(&self.trace_id.to_be_bytes());
        
        // Realm (2 bytes)
        bytes[8..10].copy_from_slice(&self.realm.to_be_bytes());
        
        // QoS (1 byte)
        bytes[10] = self.qos;
        
        // Reserved (1 byte)
        bytes[11] = self.reserved;
        
        // Checksum (4 bytes) - simple CRC
        let checksum = self.compute_checksum(&bytes[0..12]);
        bytes[12..16].copy_from_slice(&checksum.to_be_bytes());
        
        bytes
    }
    
    /// Decode from QUIC ConnectionId
    pub fn decode(cid: &ConnectionId) -> anyhow::Result<Self> {
        let bytes = cid;
        
        if bytes.len() != 16 {
            return Err(anyhow::anyhow!("Invalid ConnectionId length: {}", bytes.len()));
        }
        
        let trace_id = u64::from_be_bytes(bytes[0..8].try_into().unwrap());
        let realm = u16::from_be_bytes(bytes[8..10].try_into().unwrap());
        let qos = bytes[10];
        let reserved = bytes[11];
        
        // Verify checksum
        let stored_checksum = u32::from_be_bytes(bytes[12..16].try_into().unwrap());
        let cid_obj = Self { trace_id, realm, qos, reserved };
        let computed_checksum = cid_obj.compute_checksum(&bytes[0..12]);
        
        if stored_checksum != computed_checksum {
            return Err(anyhow::anyhow!("ConnectionId checksum mismatch"));
        }
        
        Ok(cid_obj)
    }
    
    /// Compute simple checksum
    fn compute_checksum(&self, data: &[u8]) -> u32 {
        let hash = blake3::hash(data);
        u32::from_le_bytes(hash.as_bytes()[0..4].try_into().unwrap())
    }
}

/// Realm identifiers
pub mod realms {
    pub const PRODUCTION: u16 = 0x0001;
    pub const STAGING: u16 = 0x0002;
    pub const CANARY: u16 = 0x0003;
    pub const DEVELOPMENT: u16 = 0x0004;
}

/// QoS classes
pub mod qos_classes {
    pub const BEST_EFFORT: u8 = 0;
    pub const LOW_LATENCY: u8 = 1;
    pub const HIGH_THROUGHPUT: u8 = 2;
    pub const GUARANTEED: u8 = 3;
}

/// Connection ID codec for QUIC endpoint
pub struct ConnectionIdCodec;

impl ConnectionIdCodec {
    /// Generate new connection ID
    pub fn generate(realm: u16, qos: u8) -> ConnectionId {
        let trace_id = Self::generate_trace_id();
        let cid = BpciConnectionId::new(trace_id, realm, qos);
        cid.encode()
    }
    
    /// Generate unique trace ID
    fn generate_trace_id() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        
        // Combine timestamp with random bits
        let timestamp = now.as_nanos() as u64;
        let random = rand::random::<u32>() as u64;
        
        timestamp ^ (random << 32)
    }
}

/// Use rand crate for random number generation
mod rand {
    pub fn random<T>() -> T 
    where
        T: From<u32>
    {
        // Simple pseudo-random using system time
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        T::from(nanos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let cid = BpciConnectionId::new(12345678, realms::PRODUCTION, qos_classes::LOW_LATENCY);
        let encoded = cid.encode();
        let decoded = BpciConnectionId::decode(&encoded).unwrap();
        
        assert_eq!(cid, decoded);
    }
    
    #[test]
    fn test_checksum_validation() {
        let cid = BpciConnectionId::new(12345678, realms::PRODUCTION, qos_classes::LOW_LATENCY);
        let mut encoded = cid.encode();
        
        // Corrupt a byte
        let bytes = encoded.as_ref();
        let mut corrupted = bytes.to_vec();
        corrupted[0] ^= 0xFF;
        let corrupted_cid = ConnectionId::new(&corrupted);
        
        // Should fail checksum
        assert!(BpciConnectionId::decode(&corrupted_cid).is_err());
    }
    
    #[test]
    fn test_codec_generate() {
        let cid1 = ConnectionIdCodec::generate(realms::PRODUCTION, qos_classes::BEST_EFFORT);
        let cid2 = ConnectionIdCodec::generate(realms::PRODUCTION, qos_classes::BEST_EFFORT);
        
        // Should generate different trace IDs
        assert_ne!(cid1, cid2);
    }
}
