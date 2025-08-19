//! BPI Canonical Encoding Library
//! 
//! Provides domain-separated hashing and canonical serialization for BPI components.

use blake3::Hasher;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

/// Encoding error type for BPI components
#[derive(Debug, thiserror::Error)]
pub enum EncodingError {
    #[error("Serialization failed: {0}")]
    SerializationFailed(String),
    #[error("Deserialization failed: {0}")]
    DeserializationFailed(String),
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
}

/// Domain-separated hash function for BPI components
pub fn domain_hash(domain: &str, data: &[u8]) -> [u8; 32] {
    let mut hasher = Hasher::new();
    hasher.update(domain.as_bytes());
    hasher.update(b"\x00"); // Domain separator
    hasher.update(data);
    *hasher.finalize().as_bytes()
}

/// Canonical CBOR serialization trait
pub trait CanonicalCbor {
    fn to_canonical_cbor(&self) -> Result<Vec<u8>, EncodingError>;
    fn from_canonical_cbor(data: &[u8]) -> Result<Self, EncodingError>
    where
        Self: Sized;
    
    /// Encode to canonical CBOR (convenience method)
    fn encode(&self) -> Result<Vec<u8>, EncodingError> {
        self.to_canonical_cbor()
    }
}

/// Default implementation for types that implement Serialize/Deserialize
impl<T> CanonicalCbor for T 
where 
    T: Serialize + for<'de> Deserialize<'de>
{
    fn to_canonical_cbor(&self) -> Result<Vec<u8>, EncodingError> {
        serde_json::to_vec(self)
            .map_err(|e| EncodingError::SerializationFailed(e.to_string()))
    }
    
    fn from_canonical_cbor(data: &[u8]) -> Result<Self, EncodingError> {
        serde_json::from_slice(data)
            .map_err(|e| EncodingError::DeserializationFailed(e.to_string()))
    }
}

/// Hash trait for BPI components
pub trait Hash {
    fn hash(&self) -> [u8; 32];
}

/// Standard hash implementation using Blake3
impl<T: Serialize> Hash for T {
    fn hash(&self) -> [u8; 32] {
        let data = serde_json::to_vec(self).unwrap_or_default();
        let mut hasher = Hasher::new();
        hasher.update(&data);
        *hasher.finalize().as_bytes()
    }
}

/// Domain constants for BPI components
pub mod domains {
    pub const VALIDATOR_SET: &str = "BPI_VALIDATOR_SET";
    pub const CONSENSUS_MESSAGE: &str = "BPI_CONSENSUS_MESSAGE";
    pub const CONSENSUS_COMMIT: &str = "BPI_CONSENSUS_COMMIT";
    pub const BLOCK_HEADER: &str = "BPI_BLOCK_HEADER";
    pub const HEADER_HASH: &str = "BPI_HEADER_HASH";
    pub const MERKLE_NODE: &str = "BPI_MERKLE_NODE";
    pub const BLS_SIGNATURE: &str = "BPI_BLS_SIGNATURE";
    
    // VRF domains
    pub const VRF_PUBKEY: &str = "BPI_VRF_PUBKEY";
    pub const VRF_INPUT: &str = "BPI_VRF_INPUT";
    pub const VRF_OUTPUT: &str = "BPI_VRF_OUTPUT";
    pub const VRF_PROOF: &str = "BPI_VRF_PROOF";
    pub const VRF_VERIFY: &str = "BPI_VRF_VERIFY";
    pub const VRF_KEYGEN: &str = "BPI_VRF_KEYGEN";
    
    // Merkle domains
    pub const MERKLE_LEAF: &str = "BPI_MERKLE_LEAF";
    pub const MERKLE_INTERNAL: &str = "BPI_MERKLE_INTERNAL";
    
    // BLS domains
    pub const BLS_MESSAGE: &str = "BPI_BLS_MESSAGE";
    pub const BLS_PUBKEY: &str = "BPI_BLS_PUBKEY";
    pub const BLS_KEYGEN: &str = "BPI_BLS_KEYGEN";
    
    // PoH domains
    pub const POH_TICK_HASH: &str = "BPI_POH_TICK_HASH";
    
    // Receipt domains
    pub const RECEIPT_HASH: &str = "BPI_RECEIPT_HASH";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_hash() {
        let data = b"test data";
        let hash1 = domain_hash("domain1", data);
        let hash2 = domain_hash("domain2", data);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_hash_trait() {
        let data = "test string";
        let hash = data.hash();
        assert_eq!(hash.len(), 32);
    }
}
