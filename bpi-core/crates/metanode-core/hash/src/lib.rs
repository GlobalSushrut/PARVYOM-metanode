use blake3;
use sha2::{Sha256, Digest};
use std::collections::HashMap;

/// Real cryptographic hash engine for blockchain operations
pub struct HashEngine {
    blake3_hasher: blake3::Hasher,
}

impl HashEngine {
    /// Create a new hash engine
    pub fn new() -> Self {
        Self {
            blake3_hasher: blake3::Hasher::new(),
        }
    }

    /// Hash data using BLAKE3 (preferred for blockchain operations)
    pub fn hash_blake3(data: &[u8]) -> [u8; 32] {
        blake3::hash(data).into()
    }

    /// Hash data using SHA256
    pub fn hash_sha256(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    /// Domain-separated hash for blockchain security
    pub fn domain_separated_hash(domain: &str, data: &[u8]) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(domain.as_bytes());
        hasher.update(b"\x00"); // Domain separator
        hasher.update(data);
        hasher.finalize().into()
    }

    /// Hash multiple inputs together
    pub fn hash_multiple(inputs: &[&[u8]]) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        for input in inputs {
            hasher.update(input);
        }
        hasher.finalize().into()
    }

    /// Incremental hashing for large data
    pub fn incremental_hash(&mut self, data: &[u8]) {
        self.blake3_hasher.update(data);
    }

    /// Finalize incremental hash
    pub fn finalize_hash(&mut self) -> [u8; 32] {
        let result = self.blake3_hasher.finalize();
        self.blake3_hasher = blake3::Hasher::new(); // Reset for next use
        result.into()
    }
}

impl Default for HashEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Hash types for different blockchain operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashType {
    Block,
    Transaction,
    Merkle,
    State,
    Validator,
}

impl HashType {
    pub fn domain_string(&self) -> &'static str {
        match self {
            HashType::Block => "BPI_BLOCK_HASH",
            HashType::Transaction => "BPI_TRANSACTION_HASH",
            HashType::Merkle => "BPI_MERKLE_HASH",
            HashType::State => "BPI_STATE_HASH",
            HashType::Validator => "BPI_VALIDATOR_HASH",
        }
    }
}

/// Typed hash for blockchain operations
pub fn typed_hash(hash_type: HashType, data: &[u8]) -> [u8; 32] {
    HashEngine::domain_separated_hash(hash_type.domain_string(), data)
}

/// Convenience functions for common blockchain hashing operations
pub fn hash_block(block_data: &[u8]) -> [u8; 32] {
    typed_hash(HashType::Block, block_data)
}

pub fn hash_transaction(tx_data: &[u8]) -> [u8; 32] {
    typed_hash(HashType::Transaction, tx_data)
}

pub fn hash_merkle_node(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut combined = Vec::with_capacity(64);
    combined.extend_from_slice(left);
    combined.extend_from_slice(right);
    typed_hash(HashType::Merkle, &combined)
}

pub fn hash_state(state_data: &[u8]) -> [u8; 32] {
    typed_hash(HashType::State, state_data)
}

pub fn hash_validator_info(validator_data: &[u8]) -> [u8; 32] {
    typed_hash(HashType::Validator, validator_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_hash() {
        let data = b"test data";
        let hash1 = HashEngine::hash_blake3(data);
        let hash2 = HashEngine::hash_blake3(data);
        assert_eq!(hash1, hash2); // Deterministic
        assert_ne!(hash1, [0u8; 32]); // Not empty
    }

    #[test]
    fn test_sha256_hash() {
        let data = b"test data";
        let hash1 = HashEngine::hash_sha256(data);
        let hash2 = HashEngine::hash_sha256(data);
        assert_eq!(hash1, hash2); // Deterministic
        assert_ne!(hash1, [0u8; 32]); // Not empty
    }

    #[test]
    fn test_domain_separated_hash() {
        let data = b"test data";
        let hash1 = HashEngine::domain_separated_hash("domain1", data);
        let hash2 = HashEngine::domain_separated_hash("domain2", data);
        assert_ne!(hash1, hash2); // Different domains produce different hashes
    }

    #[test]
    fn test_typed_hash() {
        let data = b"test data";
        let block_hash = typed_hash(HashType::Block, data);
        let tx_hash = typed_hash(HashType::Transaction, data);
        assert_ne!(block_hash, tx_hash); // Different types produce different hashes
    }

    #[test]
    fn test_incremental_hash() {
        let mut engine = HashEngine::new();
        engine.incremental_hash(b"part1");
        engine.incremental_hash(b"part2");
        let hash1 = engine.finalize_hash();

        let hash2 = HashEngine::hash_blake3(b"part1part2");
        assert_eq!(hash1, hash2); // Incremental should equal direct hash
    }
}
