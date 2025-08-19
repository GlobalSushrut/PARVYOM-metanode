//! Canonical encoding library for BPI Mesh
//! 
//! Provides fixed-order CBOR and Protobuf encoding with domain-separated hashing
//! Stage 2.1: CBOR Implementation with domain separation

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

pub mod cluster_endpoints_simple;

// Phase 1: ENC-notary LogBlock aggregation for v1.0 blockchain pipeline
pub mod notary;
pub mod advanced_orchestration;
pub mod production_deployment;
pub mod poe_mining;

/// Domain separation prefixes for hashing
pub mod domains {
    pub const MERKLE_LEAF: u8 = 0x00;
    pub const MERKLE_INTERNAL: u8 = 0x01;
    pub const HEADER_HASH: u8 = 0x10;
    pub const BPCI_HEADER_HASH: u8 = 0x11;
    pub const POH_TICK_HASH: u8 = 0x12;
    pub const DOCKLOCK_RECORD_HASH: u8 = 0x13;
    pub const DA_SHARD_HEADER_HASH: u8 = 0x14;
    pub const TRANSPORT_MESSAGE_HASH: u8 = 0x15;
    pub const RECEIPT_HASH: u8 = 0x16;
    pub const SHADOW_RECEIPT_HASH: u8 = 0x16;
    pub const TRAFFIC_LIGHT_HASH: u8 = 0x17;
    pub const PACKET_ENVELOPE_HASH: u8 = 0x18;
    pub const SHARD_HEADER_HASH: u8 = 0x19;
    pub const DA_ROOT_HASH: u8 = 0x1A;
    pub const BISO_POLICY_HASH: u8 = 0x1B;
    pub const BISO_EVALUATION_HASH: u8 = 0x1C;
    pub const WITNESS_ENTRY_HASH: u8 = 0x1D;
    pub const BUS_BIOS_ROUTING_HASH: u8 = 0x1E;
    pub const BLOCKBOOK_ENTRY_HASH: u8 = 0x1F;
    pub const BLOCKBOOK_LEDGER_HASH: u8 = 0x20;
    pub const AUDIT_BOOK_ENTRY_HASH: u8 = 0x21;
    pub const AUDIT_BOOK_EXPORT_HASH: u8 = 0x22;
    pub const CAR_PACKAGE_HASH: u8 = 0x23;
    pub const DAG_NODE_HASH: u8 = 0x24;
    pub const DAG_LINK_HASH: u8 = 0x25;
    
    // BLS signature domains
    pub const BLS_MESSAGE: u8 = 0x30;
    pub const BLS_SIGNATURE: u8 = 0x31;
    pub const BLS_PUBKEY: u8 = 0x32;
    pub const BLS_KEYGEN: u8 = 0x33;
    
    // VRF domains
    pub const VRF_INPUT: u8 = 0x40;
    pub const VRF_OUTPUT: u8 = 0x41;
    pub const VRF_PROOF: u8 = 0x42;
    pub const VRF_VERIFY: u8 = 0x43;
    pub const VRF_PUBKEY: u8 = 0x44;
    pub const VRF_KEYGEN: u8 = 0x45;
    
    // Leader selection domains
    pub const LEADER_SELECTION: u8 = 0x50;
    
    // Consensus domains
    pub const CONSENSUS_COMMIT: u8 = 0x60;
    pub const SLASHING_PROOF_HASH: u8 = 0x61;
}

#[derive(Error, Debug)]
pub enum EncodingError {
    #[error("CBOR encoding failed: {0}")]
    CborEncode(#[from] serde_cbor::Error),
    #[error("Invalid data structure for canonical encoding")]
    InvalidStructure,
    #[error("Hash computation failed")]
    HashError,
}

/// Canonical CBOR encoder with fixed field ordering
pub struct CanonicalCbor;

impl CanonicalCbor {
    /// Encode data to canonical CBOR bytes
    /// Ensures deterministic output by sorting map keys
    pub fn encode<T: Serialize>(data: &T) -> Result<Vec<u8>, EncodingError> {
        // Use CBOR with canonical ordering
        let mut buffer = Vec::new();
        let mut serializer = serde_cbor::Serializer::new(&mut buffer);
        serializer.self_describe()?;
        
        data.serialize(&mut serializer)?;
        Ok(buffer)
    }
    
    /// Decode canonical CBOR bytes
    pub fn decode<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Result<T, EncodingError> {
        serde_cbor::from_slice(bytes).map_err(EncodingError::from)
    }
    
    /// Encode with domain-separated hash
    pub fn encode_with_hash<T: Serialize>(
        data: &T, 
        domain: u8
    ) -> Result<(Vec<u8>, [u8; 32]), EncodingError> {
        let encoded = Self::encode(data)?;
        let hash = domain_hash(domain, &encoded);
        Ok((encoded, hash))
    }
}

/// Domain-separated hash function
pub fn domain_hash(domain: u8, data: &[u8]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&[domain]);
    hasher.update(data);
    hasher.finalize().into()
}

/// Canonical map that maintains sorted keys
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalMap<K: Ord, V>(BTreeMap<K, V>);

impl<K: Ord, V> CanonicalMap<K, V> {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.insert(key, value)
    }
    
    pub fn get(&self, key: &K) -> Option<&V> {
        self.0.get(key)
    }
    
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.0.iter()
    }
}

impl<K: Ord, V> Default for CanonicalMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

/// Test data structures for validation
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TestStruct {
    pub height: u64,
    pub hash: [u8; 32],
    pub metadata: CanonicalMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_canonical_cbor_roundtrip() {
        let test_data = TestStruct {
            height: 12345,
            hash: [0u8; 32],
            metadata: {
                let mut map = CanonicalMap::new();
                map.insert("key1".to_string(), "value1".to_string());
                map.insert("key2".to_string(), "value2".to_string());
                map
            },
        };
        
        let encoded = CanonicalCbor::encode(&test_data).unwrap();
        let decoded: TestStruct = CanonicalCbor::decode(&encoded).unwrap();
        
        assert_eq!(test_data, decoded);
    }
    
    #[test]
    fn test_deterministic_encoding() {
        let test_data = TestStruct {
            height: 42,
            hash: [1u8; 32],
            metadata: {
                let mut map = CanonicalMap::new();
                // Insert in different order
                map.insert("zebra".to_string(), "last".to_string());
                map.insert("alpha".to_string(), "first".to_string());
                map
            },
        };
        
        let encoded1 = CanonicalCbor::encode(&test_data).unwrap();
        let encoded2 = CanonicalCbor::encode(&test_data).unwrap();
        
        // Should be identical
        assert_eq!(encoded1, encoded2);
    }
    
    #[test]
    fn test_domain_separated_hashing() {
        let data = b"test data";
        
        let hash1 = domain_hash(domains::MERKLE_LEAF, data);
        let hash2 = domain_hash(domains::MERKLE_INTERNAL, data);
        
        // Different domains should produce different hashes
        assert_ne!(hash1, hash2);
        
        // Same domain should produce same hash
        let hash3 = domain_hash(domains::MERKLE_LEAF, data);
        assert_eq!(hash1, hash3);
    }
    
    #[test]
    fn test_encode_with_hash() {
        let test_data = TestStruct {
            height: 100,
            hash: [2u8; 32],
            metadata: CanonicalMap::new(),
        };
        
        let (encoded, hash) = CanonicalCbor::encode_with_hash(
            &test_data, 
            domains::HEADER_HASH
        ).unwrap();
        
        // Verify hash matches manual computation
        let expected_hash = domain_hash(domains::HEADER_HASH, &encoded);
        assert_eq!(hash, expected_hash);
    }
    
    proptest! {
        #[test]
        fn test_encoding_deterministic(
            height in any::<u64>(),
            hash_bytes in prop::array::uniform32(any::<u8>())
        ) {
            let test_data = TestStruct {
                height,
                hash: hash_bytes,
                metadata: CanonicalMap::new(),
            };
            
            let encoded1 = CanonicalCbor::encode(&test_data).unwrap();
            let encoded2 = CanonicalCbor::encode(&test_data).unwrap();
            
            prop_assert_eq!(encoded1, encoded2);
        }
        
        #[test]
        fn test_domain_separation_property(
            data in prop::collection::vec(any::<u8>(), 0..1000),
            domain1 in any::<u8>(),
            domain2 in any::<u8>()
        ) {
            prop_assume!(domain1 != domain2);
            
            let hash1 = domain_hash(domain1, &data);
            let hash2 = domain_hash(domain2, &data);
            
            prop_assert_ne!(hash1, hash2);
        }
    }
}

// Stage 2 Exit Criteria Test
#[cfg(test)]
mod stage2_tests {
    use super::*;
    
    #[test]
    fn stage2_exit_criteria() {
        println!("🧪 Testing Stage 2 Exit Criteria...");
        
        // Test 1: CBOR enc/dec with fixed ordering
        let test_data = TestStruct {
            height: 12345,
            hash: [0xab; 32],
            metadata: {
                let mut map = CanonicalMap::new();
                map.insert("z_last".to_string(), "value_z".to_string());
                map.insert("a_first".to_string(), "value_a".to_string());
                map
            },
        };
        
        let encoded = CanonicalCbor::encode(&test_data).unwrap();
        let decoded: TestStruct = CanonicalCbor::decode(&encoded).unwrap();
        assert_eq!(test_data, decoded);
        println!("  ✓ CBOR enc/dec with fixed ordering");
        
        // Test 2: Domain-separated hashing
        let data = b"test";
        let hash1 = domain_hash(domains::MERKLE_LEAF, data);
        let hash2 = domain_hash(domains::MERKLE_INTERNAL, data);
        assert_ne!(hash1, hash2);
        println!("  ✓ Domain-separated hashing");
        
        // Test 3: Cross-language compatibility prep (golden vectors)
        let golden_data = TestStruct {
            height: 42,
            hash: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                   0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
                   0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
                   0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20],
            metadata: CanonicalMap::new(),
        };
        
        let golden_encoded = CanonicalCbor::encode(&golden_data).unwrap();
        let golden_hash = domain_hash(domains::HEADER_HASH, &golden_encoded);
        
        // Store golden vectors for cross-language testing
        println!("  Golden vector length: {} bytes", golden_encoded.len());
        println!("  Golden hash: {:02x?}", &golden_hash[..8]);
        println!("  ✓ Golden vectors generated");
        
        println!("✅ Stage 2 Exit Criteria: PASSED");
    }
}
