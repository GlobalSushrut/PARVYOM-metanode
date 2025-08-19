//! # BPI Block Headers & Validation
//!
//! This crate provides block header structures, canonical serialization, hash computation,
//! and validation logic for the BPI Mesh Web3 architecture.
//!
//! ## Key Components
//!
//! - **Header**: Block header structure per logic.md specification
//! - **HeaderHash**: Canonical hash computation with domain separation
//! - **Validation**: Header validation and consistency checks
//! - **IBFT Integration**: Support for IBFT consensus mode

use std::fmt;

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Re-export core types
pub use bpi_enc::{domain_hash, CanonicalCbor, domains::HEADER_HASH};
pub use bpi_blsagg::{Signature as BlsSignature, PublicKey as BlsPublicKey};
pub use bpi_merkle::{MerkleTree, Hash as MerkleHash};
pub use bpi_vrf::{VrfProof, VrfOutput};

mod validation;
mod ibft;

pub use validation::*;
pub use ibft::*;

/// Block header consensus mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsensusMode {
    /// IBFT consensus mode
    Ibft = 2,
}

/// Block header structure per logic.md specification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Header {
    /// Protocol version
    pub version: u8,
    /// Block height/number
    pub height: u64,
    /// Hash of previous block header
    pub prev_hash: [u8; 32],
    /// Merkle root of PoH ticks
    pub poh_root: [u8; 32],
    /// Merkle root of receipts (DockLock record hashes) or ZERO
    pub receipts_root: [u8; 32],
    /// Merkle root of DA shard headers
    pub da_root: [u8; 32],
    /// Outbound cross-chain message queue root
    pub xcmp_root: [u8; 32],
    /// Validator set hash (Merkle-map root: index → PK)
    pub validator_set_hash: [u8; 32],
    /// Consensus mode (2 = IBFT)
    pub mode: ConsensusMode,
    /// IBFT round number
    pub round: u64,
    /// Header creation timestamp
    pub timestamp: DateTime<Utc>,
}

/// Header hash with domain separation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HeaderHash(pub [u8; 32]);

/// Genesis header configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisConfig {
    /// Chain ID for genesis block
    pub chain_id: u64,
    /// Genesis timestamp
    pub timestamp: DateTime<Utc>,
    /// Initial validator set hash
    pub validator_set_hash: [u8; 32],
}

/// Header configuration for construction
#[derive(Debug, Clone)]
pub struct HeaderConfig {
    pub version: u8,
    pub height: u64,
    pub prev_hash: [u8; 32],
    pub poh_root: [u8; 32],
    pub receipts_root: [u8; 32],
    pub da_root: [u8; 32],
    pub xcmp_root: [u8; 32],
    pub validator_set_hash: [u8; 32],
    pub mode: ConsensusMode,
    pub round: u64,
}

impl Header {
    /// Create a new block header from configuration
    pub fn new(config: HeaderConfig) -> Self {
        Self {
            version: config.version,
            height: config.height,
            prev_hash: config.prev_hash,
            poh_root: config.poh_root,
            receipts_root: config.receipts_root,
            da_root: config.da_root,
            xcmp_root: config.xcmp_root,
            validator_set_hash: config.validator_set_hash,
            mode: config.mode,
            round: config.round,
            timestamp: Utc::now(),
        }
    }
    
    /// Create genesis header
    pub fn genesis(config: &GenesisConfig) -> Self {
        Self {
            version: 1,
            height: 0,
            prev_hash: [0u8; 32],
            poh_root: [0u8; 32],
            receipts_root: [0u8; 32],
            da_root: [0u8; 32],
            xcmp_root: [0u8; 32],
            validator_set_hash: config.validator_set_hash,
            mode: ConsensusMode::Ibft,
            round: 0,
            timestamp: config.timestamp,
        }
    }
    
    /// Compute canonical header hash with domain separation
    /// header_hash = H(0x10 || enc(header))
    pub fn hash(&self) -> Result<HeaderHash> {
        let encoded = CanonicalCbor::encode(self)
            .map_err(|e| anyhow::anyhow!("Failed to encode header: {}", e))?;
        let hash = domain_hash(HEADER_HASH, &encoded);
        Ok(HeaderHash(hash))
    }
    
    /// Check if this is the genesis block
    pub fn is_genesis(&self) -> bool {
        self.height == 0
    }
    
    /// Check if header has empty receipts root (no transactions)
    pub fn is_empty_receipts(&self) -> bool {
        self.receipts_root == [0u8; 32]
    }
    
    /// Validate header structure and constraints
    pub fn validate(&self) -> Result<()> {
        // Version must be 1
        if self.version != 1 {
            return Err(anyhow::anyhow!("Invalid header version: {}", self.version));
        }
        
        // Mode must be IBFT
        if self.mode != ConsensusMode::Ibft {
            return Err(anyhow::anyhow!("Unsupported consensus mode: {:?}", self.mode));
        }
        
        // Genesis block constraints
        if self.is_genesis() {
            if self.prev_hash != [0u8; 32] {
                return Err(anyhow::anyhow!("Genesis block must have zero prev_hash"));
            }
            if self.round != 0 {
                return Err(anyhow::anyhow!("Genesis block must have round 0"));
            }
        } else {
            // Non-genesis constraints
            if self.prev_hash == [0u8; 32] {
                return Err(anyhow::anyhow!("Non-genesis block cannot have zero prev_hash"));
            }
        }
        
        Ok(())
    }
    
    /// Validate header chain continuity
    pub fn validate_chain_continuity(&self, prev_header: &Header) -> Result<()> {
        // Height must increment by 1
        if self.height != prev_header.height + 1 {
            return Err(anyhow::anyhow!(
                "Invalid height progression: {} -> {}", 
                prev_header.height, 
                self.height
            ));
        }
        
        // Previous hash must match
        let expected_prev_hash = prev_header.hash()?.0;
        if self.prev_hash != expected_prev_hash {
            return Err(anyhow::anyhow!(
                "Previous hash mismatch: expected {:?}, got {:?}",
                hex::encode(expected_prev_hash),
                hex::encode(self.prev_hash)
            ));
        }
        
        // Timestamp must be monotonic
        if self.timestamp <= prev_header.timestamp {
            return Err(anyhow::anyhow!(
                "Timestamp not monotonic: {} <= {}",
                self.timestamp,
                prev_header.timestamp
            ));
        }
        
        Ok(())
    }
}

// Header implements Serialize, so it can be encoded with CanonicalCbor::encode

impl HeaderHash {
    /// Create from raw bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    
    /// Get raw bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
    
    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl fmt::Display for HeaderHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl From<[u8; 32]> for HeaderHash {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl AsRef<[u8]> for HeaderHash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    
    fn create_test_header() -> Header {
        Header::new(HeaderConfig {
            version: 1,
            height: 100,
            prev_hash: [1u8; 32],
            poh_root: [2u8; 32],
            receipts_root: [3u8; 32],
            da_root: [4u8; 32],
            xcmp_root: [5u8; 32],
            validator_set_hash: [6u8; 32],
            mode: ConsensusMode::Ibft,
            round: 1,
        })
    }
    
    #[test]
    fn test_header_creation() {
        let header = create_test_header();
        assert_eq!(header.version, 1);
        assert_eq!(header.height, 100);
        assert_eq!(header.mode, ConsensusMode::Ibft);
        assert!(!header.is_genesis());
    }
    
    #[test]
    fn test_genesis_header() {
        let config = GenesisConfig {
            timestamp: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
            validator_set_hash: [7u8; 32],
            chain_id: 1,
        };
        
        let genesis = Header::genesis(&config);
        assert!(genesis.is_genesis());
        assert_eq!(genesis.height, 0);
        assert_eq!(genesis.prev_hash, [0u8; 32]);
        assert_eq!(genesis.round, 0);
        assert_eq!(genesis.validator_set_hash, [7u8; 32]);
    }
    
    #[test]
    fn test_header_hash() {
        let header = create_test_header();
        let hash1 = header.hash().unwrap();
        let hash2 = header.hash().unwrap();
        
        // Hash should be deterministic
        assert_eq!(hash1, hash2);
        
        // Hash should be 32 bytes
        assert_eq!(hash1.as_bytes().len(), 32);
    }
    
    #[test]
    fn test_header_validation() {
        let mut header = create_test_header();
        
        // Valid header should pass
        assert!(header.validate().is_ok());
        
        // Invalid version should fail
        header.version = 2;
        assert!(header.validate().is_err());
        header.version = 1;
        
        // Invalid mode should fail
        // Note: We can't easily test this since ConsensusMode only has Ibft variant
    }
    
    #[test]
    fn test_genesis_validation() {
        let config = GenesisConfig {
            timestamp: Utc::now(),
            validator_set_hash: [7u8; 32],
            chain_id: 1,
        };
        
        let mut genesis = Header::genesis(&config);
        
        // Valid genesis should pass
        assert!(genesis.validate().is_ok());
        
        // Genesis with non-zero prev_hash should fail
        genesis.prev_hash = [1u8; 32];
        assert!(genesis.validate().is_err());
        genesis.prev_hash = [0u8; 32];
        
        // Genesis with non-zero round should fail
        genesis.round = 1;
        assert!(genesis.validate().is_err());
    }
    
    #[test]
    fn test_chain_continuity() {
        let config = GenesisConfig {
            timestamp: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
            validator_set_hash: [7u8; 32],
            chain_id: 1,
        };
        
        let genesis = Header::genesis(&config);
        let genesis_hash = genesis.hash().unwrap().0;
        
        let config = HeaderConfig {
            version: 1,
            height: 1,
            prev_hash: genesis_hash,
            poh_root: [2u8; 32],
            receipts_root: [3u8; 32],
            da_root: [4u8; 32],
            xcmp_root: [5u8; 32],
            validator_set_hash: [6u8; 32],
            mode: ConsensusMode::Ibft,
            round: 0,
        };
        let mut next_header = Header::new(config);
        next_header.timestamp = Utc.with_ymd_and_hms(2024, 1, 1, 1, 0, 0).unwrap();
        
        // Valid chain continuity should pass
        assert!(next_header.validate_chain_continuity(&genesis).is_ok());
        
        // Wrong height should fail
        next_header.height = 2;
        assert!(next_header.validate_chain_continuity(&genesis).is_err());
        next_header.height = 1;
        
        // Wrong prev_hash should fail
        next_header.prev_hash = [1u8; 32];
        assert!(next_header.validate_chain_continuity(&genesis).is_err());
        next_header.prev_hash = genesis_hash;
        
        // Non-monotonic timestamp should fail
        next_header.timestamp = genesis.timestamp;
        assert!(next_header.validate_chain_continuity(&genesis).is_err());
    }
    
    #[test]
    fn test_header_hash_display() {
        let header = create_test_header();
        let hash = header.hash().unwrap();
        
        let hex_str = hash.to_hex();
        let display_str = format!("{}", hash);
        
        assert_eq!(hex_str, display_str);
        assert_eq!(hex_str.len(), 64); // 32 bytes * 2 hex chars
    }
    
    #[test]
    fn test_canonical_encoding() {
        let header = create_test_header();
        
        // Should be able to encode to CBOR
        let encoded = CanonicalCbor::encode(&header).unwrap();
        assert!(!encoded.is_empty());
        
        // Same header should produce same encoding
        let encoded2 = CanonicalCbor::encode(&header).unwrap();
        assert_eq!(encoded, encoded2);
    }
}
