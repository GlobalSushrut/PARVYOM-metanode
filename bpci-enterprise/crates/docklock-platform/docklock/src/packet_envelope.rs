//! Stage 34: Packet Envelope Structure (Shard Headers & da_root)
//! 
//! This module implements the cryptographically sealed wrapper for every piece of data
//! in motion as part of the BISO Security & Compliance Architecture.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use tracing::{debug};

use crate::error::{DockLockError, DockLockResult};
use crate::traffic_light::{DataClassification, TrafficLightState};

/// Domain separator for packet envelope hashing
pub const PACKET_ENVELOPE_HASH: u8 = 0x18;

/// Domain separator for shard headers
pub const SHARD_HEADER_HASH: u8 = 0x19;

/// Domain separator for data availability root
pub const DA_ROOT_HASH: u8 = 0x1A;

/// Encryption schemes supported by packet envelopes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncryptionScheme {
    /// Ed25519 + ChaCha20-Poly1305
    Ed25519ChaCha20,
    /// AES-256-GCM
    AesGcm256,
    /// XChaCha20-Poly1305 (recommended)
    XChaCha20Poly1305,
    /// No encryption (for public data)
    None,
}

impl EncryptionScheme {
    /// Get human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            EncryptionScheme::Ed25519ChaCha20 => "Ed25519 + ChaCha20-Poly1305",
            EncryptionScheme::AesGcm256 => "AES-256-GCM",
            EncryptionScheme::XChaCha20Poly1305 => "XChaCha20-Poly1305",
            EncryptionScheme::None => "No encryption",
        }
    }

    /// Check if scheme provides encryption
    pub fn is_encrypted(&self) -> bool {
        !matches!(self, EncryptionScheme::None)
    }
}

/// Packet envelope metadata for routing and compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvelopeMetadata {
    /// Source service/node identifier
    pub origin: String,
    /// Destination service/node identifier
    pub destination: Option<String>,
    /// Data classification level
    pub classification: DataClassification,
    /// Traffic light state for this packet
    pub traffic_light_state: TrafficLightState,
    /// BISO policy hash that was applied
    pub biso_policy_hash: [u8; 32],
    /// Encryption scheme used for payload
    pub encryption_scheme: EncryptionScheme,
    /// Additional routing metadata
    pub routing_metadata: HashMap<String, String>,
    /// Compliance flags and status
    pub compliance_flags: Vec<String>,
    /// Priority level (0 = lowest, 255 = highest)
    pub priority: u8,
    /// Time-to-live in seconds
    pub ttl: u64,
}

impl EnvelopeMetadata {
    /// Create new envelope metadata
    pub fn new(
        origin: String,
        classification: DataClassification,
        traffic_light_state: TrafficLightState,
        biso_policy_hash: [u8; 32],
        encryption_scheme: EncryptionScheme,
    ) -> Self {
        Self {
            origin,
            destination: None,
            classification,
            traffic_light_state,
            biso_policy_hash,
            encryption_scheme,
            routing_metadata: HashMap::new(),
            compliance_flags: Vec::new(),
            priority: 128, // Default medium priority
            ttl: 3600, // Default 1 hour TTL
        }
    }

    /// Set destination
    pub fn with_destination(mut self, destination: String) -> Self {
        self.destination = Some(destination);
        self
    }

    /// Set priority
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Set TTL
    pub fn with_ttl(mut self, ttl: u64) -> Self {
        self.ttl = ttl;
        self
    }

    /// Add routing metadata
    pub fn with_routing_metadata(mut self, key: String, value: String) -> Self {
        self.routing_metadata.insert(key, value);
        self
    }

    /// Add compliance flag
    pub fn with_compliance_flag(mut self, flag: String) -> Self {
        self.compliance_flags.push(flag);
        self
    }
}

/// Shard header for data availability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardHeader {
    /// Shard identifier
    pub shard_id: Uuid,
    /// Shard index in the data set
    pub shard_index: u32,
    /// Total number of shards
    pub total_shards: u32,
    /// Shard data hash
    pub shard_hash: [u8; 32],
    /// Reed-Solomon encoding parameters
    pub rs_params: ReedSolomonParams,
    /// Timestamp when shard was created
    pub timestamp: u64,
    /// Shard size in bytes
    pub shard_size: u64,
    /// Cryptographic signature
    pub signature: Option<Vec<u8>>,
    /// Signer public key
    pub signer_pubkey: Option<Vec<u8>>,
}

/// Reed-Solomon encoding parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReedSolomonParams {
    /// Number of data shards
    pub data_shards: u32,
    /// Number of parity shards
    pub parity_shards: u32,
    /// Shard size in bytes
    pub shard_size: u32,
}

impl ReedSolomonParams {
    /// Create new Reed-Solomon parameters
    pub fn new(data_shards: u32, parity_shards: u32, shard_size: u32) -> Self {
        Self {
            data_shards,
            parity_shards,
            shard_size,
        }
    }

    /// Get total number of shards
    pub fn total_shards(&self) -> u32 {
        self.data_shards + self.parity_shards
    }

    /// Get redundancy ratio
    pub fn redundancy_ratio(&self) -> f64 {
        self.parity_shards as f64 / self.data_shards as f64
    }
}

impl ShardHeader {
    /// Create new shard header
    pub fn new(
        shard_index: u32,
        total_shards: u32,
        shard_hash: [u8; 32],
        rs_params: ReedSolomonParams,
        shard_size: u64,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            shard_id: Uuid::new_v4(),
            shard_index,
            total_shards,
            shard_hash,
            rs_params,
            timestamp,
            shard_size,
            signature: None,
            signer_pubkey: None,
        }
    }

    /// Compute hash for signing (excludes signature fields)
    pub fn compute_signing_hash(&self) -> DockLockResult<[u8; 32]> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[SHARD_HEADER_HASH]);
        
        hasher.update(self.shard_id.as_bytes());
        hasher.update(&self.shard_index.to_le_bytes());
        hasher.update(&self.total_shards.to_le_bytes());
        hasher.update(&self.shard_hash);
        hasher.update(&bincode::serialize(&self.rs_params)
            .map_err(|e| DockLockError::EncodingError(format!("Failed to serialize RS params: {}", e)))?);
        hasher.update(&self.timestamp.to_le_bytes());
        hasher.update(&self.shard_size.to_le_bytes());
        
        Ok(*hasher.finalize().as_bytes())
    }

    /// Sign the shard header
    pub fn sign(&mut self, signing_key: &SigningKey) -> DockLockResult<()> {
        let hash = self.compute_signing_hash()?;
        let signature = signing_key.sign(&hash);
        
        self.signature = Some(signature.to_bytes().to_vec());
        self.signer_pubkey = Some(signing_key.verifying_key().to_bytes().to_vec());
        
        debug!("Signed shard header {} with Ed25519", self.shard_id);
        Ok(())
    }

    /// Verify the shard header signature
    pub fn verify_signature(&self) -> DockLockResult<bool> {
        let signature_bytes = self.signature.as_ref()
            .ok_or_else(|| DockLockError::InvalidOperation("No signature found".to_string()))?;
        let pubkey_bytes = self.signer_pubkey.as_ref()
            .ok_or_else(|| DockLockError::InvalidOperation("No public key found".to_string()))?;
        
        let mut pubkey_array = [0u8; 32];
        pubkey_array.copy_from_slice(pubkey_bytes);
        let verifying_key = VerifyingKey::from_bytes(&pubkey_array)
            .map_err(|e| DockLockError::CryptoError(format!("Invalid public key: {}", e)))?;
        
        let mut signature_array = [0u8; 64];
        signature_array.copy_from_slice(signature_bytes);
        let signature = Signature::from_bytes(&signature_array);
        
        let hash = self.compute_signing_hash()?;
        
        match verifying_key.verify(&hash, &signature) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

/// Data availability root for Merkle tree of shards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAvailabilityRoot {
    /// DA root identifier
    pub da_root_id: Uuid,
    /// Merkle root of all shard headers
    pub merkle_root: [u8; 32],
    /// List of shard IDs included in this DA root
    pub shard_ids: Vec<Uuid>,
    /// Total data size across all shards
    pub total_data_size: u64,
    /// Reed-Solomon parameters for the data set
    pub rs_params: ReedSolomonParams,
    /// Timestamp when DA root was computed
    pub timestamp: u64,
    /// Cryptographic signature
    pub signature: Option<Vec<u8>>,
    /// Signer public key
    pub signer_pubkey: Option<Vec<u8>>,
}

impl DataAvailabilityRoot {
    /// Create new data availability root
    pub fn new(
        merkle_root: [u8; 32],
        shard_ids: Vec<Uuid>,
        total_data_size: u64,
        rs_params: ReedSolomonParams,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            da_root_id: Uuid::new_v4(),
            merkle_root,
            shard_ids,
            total_data_size,
            rs_params,
            timestamp,
            signature: None,
            signer_pubkey: None,
        }
    }

    /// Compute hash for signing (excludes signature fields)
    pub fn compute_signing_hash(&self) -> DockLockResult<[u8; 32]> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[DA_ROOT_HASH]);
        
        hasher.update(self.da_root_id.as_bytes());
        hasher.update(&self.merkle_root);
        
        // Hash shard IDs
        for shard_id in &self.shard_ids {
            hasher.update(shard_id.as_bytes());
        }
        
        hasher.update(&self.total_data_size.to_le_bytes());
        hasher.update(&bincode::serialize(&self.rs_params)
            .map_err(|e| DockLockError::EncodingError(format!("Failed to serialize RS params: {}", e)))?);
        hasher.update(&self.timestamp.to_le_bytes());
        
        Ok(*hasher.finalize().as_bytes())
    }

    /// Sign the DA root
    pub fn sign(&mut self, signing_key: &SigningKey) -> DockLockResult<()> {
        let hash = self.compute_signing_hash()?;
        let signature = signing_key.sign(&hash);
        
        self.signature = Some(signature.to_bytes().to_vec());
        self.signer_pubkey = Some(signing_key.verifying_key().to_bytes().to_vec());
        
        debug!("Signed DA root {} with Ed25519", self.da_root_id);
        Ok(())
    }

    /// Verify the DA root signature
    pub fn verify_signature(&self) -> DockLockResult<bool> {
        let signature_bytes = self.signature.as_ref()
            .ok_or_else(|| DockLockError::InvalidOperation("No signature found".to_string()))?;
        let pubkey_bytes = self.signer_pubkey.as_ref()
            .ok_or_else(|| DockLockError::InvalidOperation("No public key found".to_string()))?;
        
        let mut pubkey_array = [0u8; 32];
        pubkey_array.copy_from_slice(pubkey_bytes);
        let verifying_key = VerifyingKey::from_bytes(&pubkey_array)
            .map_err(|e| DockLockError::CryptoError(format!("Invalid public key: {}", e)))?;
        
        let mut signature_array = [0u8; 64];
        signature_array.copy_from_slice(signature_bytes);
        let signature = Signature::from_bytes(&signature_array);
        
        let hash = self.compute_signing_hash()?;
        
        match verifying_key.verify(&hash, &signature) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Get number of shards
    pub fn shard_count(&self) -> usize {
        self.shard_ids.len()
    }

    /// Check if DA root contains a specific shard
    pub fn contains_shard(&self, shard_id: &Uuid) -> bool {
        self.shard_ids.contains(shard_id)
    }
}

/// Packet envelope - cryptographically sealed wrapper for data in motion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketEnvelope {
    /// Unique packet identifier
    pub packet_id: String,
    /// Envelope metadata
    pub metadata: EnvelopeMetadata,
    /// Hash of the encrypted payload
    pub payload_hash: [u8; 32],
    /// Timestamp when envelope was created
    pub timestamp: u64,
    /// Optional shard header (for sharded data)
    pub shard_header: Option<ShardHeader>,
    /// Optional DA root reference
    pub da_root_id: Option<Uuid>,
    /// Cryptographic signature over envelope
    pub signature: Option<Vec<u8>>,
    /// Signer public key
    pub signer_pubkey: Option<Vec<u8>>,
}

impl PacketEnvelope {
    /// Create new packet envelope
    pub fn new(
        packet_id: String,
        metadata: EnvelopeMetadata,
        payload_hash: [u8; 32],
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            packet_id,
            metadata,
            payload_hash,
            timestamp,
            shard_header: None,
            da_root_id: None,
            signature: None,
            signer_pubkey: None,
        }
    }

    /// Add shard header to envelope
    pub fn with_shard_header(mut self, shard_header: ShardHeader) -> Self {
        self.shard_header = Some(shard_header);
        self
    }

    /// Add DA root reference to envelope
    pub fn with_da_root(mut self, da_root_id: Uuid) -> Self {
        self.da_root_id = Some(da_root_id);
        self
    }

    /// Compute hash for signing (excludes signature fields)
    pub fn compute_signing_hash(&self) -> DockLockResult<[u8; 32]> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[PACKET_ENVELOPE_HASH]);
        
        hasher.update(self.packet_id.as_bytes());
        hasher.update(&bincode::serialize(&self.metadata)
            .map_err(|e| DockLockError::EncodingError(format!("Failed to serialize metadata: {}", e)))?);
        hasher.update(&self.payload_hash);
        hasher.update(&self.timestamp.to_le_bytes());
        
        if let Some(shard_header) = &self.shard_header {
            hasher.update(&bincode::serialize(shard_header)
                .map_err(|e| DockLockError::EncodingError(format!("Failed to serialize shard header: {}", e)))?);
        }
        
        if let Some(da_root_id) = &self.da_root_id {
            hasher.update(da_root_id.as_bytes());
        }
        
        Ok(*hasher.finalize().as_bytes())
    }

    /// Sign the packet envelope
    pub fn sign(&mut self, signing_key: &SigningKey) -> DockLockResult<()> {
        let hash = self.compute_signing_hash()?;
        let signature = signing_key.sign(&hash);
        
        self.signature = Some(signature.to_bytes().to_vec());
        self.signer_pubkey = Some(signing_key.verifying_key().to_bytes().to_vec());
        
        debug!("Signed packet envelope {} with Ed25519", self.packet_id);
        Ok(())
    }

    /// Verify the packet envelope signature
    pub fn verify_signature(&self) -> DockLockResult<bool> {
        let signature_bytes = self.signature.as_ref()
            .ok_or_else(|| DockLockError::InvalidOperation("No signature found".to_string()))?;
        let pubkey_bytes = self.signer_pubkey.as_ref()
            .ok_or_else(|| DockLockError::InvalidOperation("No public key found".to_string()))?;
        
        let mut pubkey_array = [0u8; 32];
        pubkey_array.copy_from_slice(pubkey_bytes);
        let verifying_key = VerifyingKey::from_bytes(&pubkey_array)
            .map_err(|e| DockLockError::CryptoError(format!("Invalid public key: {}", e)))?;
        
        let mut signature_array = [0u8; 64];
        signature_array.copy_from_slice(signature_bytes);
        let signature = Signature::from_bytes(&signature_array);
        
        let hash = self.compute_signing_hash()?;
        
        match verifying_key.verify(&hash, &signature) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Check if envelope has expired based on TTL
    pub fn is_expired(&self) -> bool {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        current_time > self.timestamp + self.metadata.ttl
    }

    /// Get age of envelope in seconds
    pub fn age_seconds(&self) -> u64 {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        current_time.saturating_sub(self.timestamp)
    }

    /// Check if envelope is sharded
    pub fn is_sharded(&self) -> bool {
        self.shard_header.is_some()
    }

    /// Check if envelope has DA root reference
    pub fn has_da_root(&self) -> bool {
        self.da_root_id.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;
    use crate::traffic_light::TrafficLightState;

    #[test]
    fn test_encryption_scheme_descriptions() {
        assert_eq!(EncryptionScheme::Ed25519ChaCha20.description(), "Ed25519 + ChaCha20-Poly1305");
        assert_eq!(EncryptionScheme::XChaCha20Poly1305.description(), "XChaCha20-Poly1305");
        assert!(EncryptionScheme::XChaCha20Poly1305.is_encrypted());
        assert!(!EncryptionScheme::None.is_encrypted());
    }

    #[test]
    fn test_reed_solomon_params() {
        let rs_params = ReedSolomonParams::new(10, 5, 1024);
        assert_eq!(rs_params.total_shards(), 15);
        assert_eq!(rs_params.redundancy_ratio(), 0.5);
    }

    #[test]
    fn test_envelope_metadata_creation() {
        let policy_hash = [0u8; 32];
        let metadata = EnvelopeMetadata::new(
            "analytics_service".to_string(),
            DataClassification::PII,
            TrafficLightState::Green,
            policy_hash,
            EncryptionScheme::XChaCha20Poly1305,
        );

        assert_eq!(metadata.origin, "analytics_service");
        assert_eq!(metadata.classification, DataClassification::PII);
        assert_eq!(metadata.traffic_light_state, TrafficLightState::Green);
        assert_eq!(metadata.encryption_scheme, EncryptionScheme::XChaCha20Poly1305);
        assert_eq!(metadata.priority, 128);
        assert_eq!(metadata.ttl, 3600);
    }

    #[test]
    fn test_shard_header_creation_and_signing() {
        let mut rng = OsRng;
        let signing_key = SigningKey::generate(&mut rng);
        
        let shard_hash = [1u8; 32];
        let rs_params = ReedSolomonParams::new(10, 5, 1024);
        
        let mut shard_header = ShardHeader::new(
            0,
            15,
            shard_hash,
            rs_params,
            1024,
        );

        // Sign the shard header
        shard_header.sign(&signing_key).unwrap();
        assert!(shard_header.signature.is_some());
        assert!(shard_header.signer_pubkey.is_some());

        // Verify the signature
        assert!(shard_header.verify_signature().unwrap());
    }

    #[test]
    fn test_data_availability_root_creation() {
        let merkle_root = [2u8; 32];
        let shard_ids = vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
        let rs_params = ReedSolomonParams::new(10, 5, 1024);
        
        let da_root = DataAvailabilityRoot::new(
            merkle_root,
            shard_ids.clone(),
            30720, // 30KB total
            rs_params,
        );

        assert_eq!(da_root.merkle_root, merkle_root);
        assert_eq!(da_root.shard_count(), 3);
        assert!(da_root.contains_shard(&shard_ids[0]));
        assert!(!da_root.contains_shard(&Uuid::new_v4()));
    }

    #[test]
    fn test_data_availability_root_signing_and_verification() {
        let mut rng = OsRng;
        let signing_key = SigningKey::generate(&mut rng);
        
        let merkle_root = [3u8; 32];
        let shard_ids = vec![Uuid::new_v4()];
        let rs_params = ReedSolomonParams::new(5, 3, 512);
        
        let mut da_root = DataAvailabilityRoot::new(
            merkle_root,
            shard_ids,
            2560, // 2.5KB total
            rs_params,
        );

        // Sign the DA root
        da_root.sign(&signing_key).unwrap();
        assert!(da_root.signature.is_some());
        assert!(da_root.signer_pubkey.is_some());

        // Verify the signature
        assert!(da_root.verify_signature().unwrap());
    }

    #[test]
    fn test_packet_envelope_creation() {
        let policy_hash = [4u8; 32];
        let metadata = EnvelopeMetadata::new(
            "data_service".to_string(),
            DataClassification::General,
            TrafficLightState::Green,
            policy_hash,
            EncryptionScheme::AesGcm256,
        ).with_destination("storage_service".to_string())
         .with_priority(200);

        let payload_hash = [5u8; 32];
        let envelope = PacketEnvelope::new(
            "pkt_envelope_test".to_string(),
            metadata,
            payload_hash,
        );

        assert_eq!(envelope.packet_id, "pkt_envelope_test");
        assert_eq!(envelope.payload_hash, payload_hash);
        assert_eq!(envelope.metadata.priority, 200);
        assert!(!envelope.is_sharded());
        assert!(!envelope.has_da_root());
        assert!(!envelope.is_expired());
    }

    #[test]
    fn test_packet_envelope_with_shard_header() {
        let policy_hash = [6u8; 32];
        let metadata = EnvelopeMetadata::new(
            "sharded_service".to_string(),
            DataClassification::PCI,
            TrafficLightState::Yellow,
            policy_hash,
            EncryptionScheme::Ed25519ChaCha20,
        );

        let payload_hash = [7u8; 32];
        let shard_hash = [8u8; 32];
        let rs_params = ReedSolomonParams::new(8, 4, 2048);
        let shard_header = ShardHeader::new(2, 12, shard_hash, rs_params, 2048);

        let envelope = PacketEnvelope::new(
            "pkt_sharded_test".to_string(),
            metadata,
            payload_hash,
        ).with_shard_header(shard_header)
         .with_da_root(Uuid::new_v4());

        assert!(envelope.is_sharded());
        assert!(envelope.has_da_root());
        assert_eq!(envelope.shard_header.as_ref().unwrap().shard_index, 2);
        assert_eq!(envelope.shard_header.as_ref().unwrap().total_shards, 12);
    }

    #[test]
    fn test_packet_envelope_signing_and_verification() {
        let mut rng = OsRng;
        let signing_key = SigningKey::generate(&mut rng);
        
        let policy_hash = [9u8; 32];
        let metadata = EnvelopeMetadata::new(
            "secure_service".to_string(),
            DataClassification::PHI,
            TrafficLightState::Green,
            policy_hash,
            EncryptionScheme::XChaCha20Poly1305,
        );

        let payload_hash = [10u8; 32];
        let mut envelope = PacketEnvelope::new(
            "pkt_secure_test".to_string(),
            metadata,
            payload_hash,
        );

        // Sign the envelope
        envelope.sign(&signing_key).unwrap();
        assert!(envelope.signature.is_some());
        assert!(envelope.signer_pubkey.is_some());

        // Verify the signature
        assert!(envelope.verify_signature().unwrap());
    }

    #[test]
    fn test_packet_envelope_expiration() {
        let policy_hash = [11u8; 32];
        let metadata = EnvelopeMetadata::new(
            "test_service".to_string(),
            DataClassification::Public,
            TrafficLightState::Green,
            policy_hash,
            EncryptionScheme::None,
        ).with_ttl(1); // 1 second TTL

        let payload_hash = [12u8; 32];
        let envelope = PacketEnvelope::new(
            "pkt_expiry_test".to_string(),
            metadata,
            payload_hash,
        );

        // Should not be expired immediately
        assert!(!envelope.is_expired());
        assert!(envelope.age_seconds() < 2);

        // Sleep would be needed to test actual expiration, but we'll test the logic
        // by manually adjusting the timestamp
        let mut expired_envelope = envelope.clone();
        expired_envelope.timestamp = expired_envelope.timestamp - 10; // 10 seconds ago
        assert!(expired_envelope.is_expired());
    }
}
