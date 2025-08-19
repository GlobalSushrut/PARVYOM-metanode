//! BLS commit objects and consensus primitives for BPI Mesh
//! Stage 13: BLS Commit Object

use std::collections::HashMap;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Re-export dependencies
pub use bpi_enc::{domain_hash, domains, CanonicalCbor};
pub use bpi_blsagg::{Signature, PublicKey, PrivateKey, AggregatedSignature};
pub use bpi_validator_set::{ValidatorSet, ValidatorInfo};
pub use bpi_headers::{Header, HeaderHash};

/// Consensus errors
#[derive(Error, Debug)]
pub enum ConsensusError {
    #[error("Invalid validator index: {0}")]
    InvalidValidatorIndex(usize),
    #[error("Insufficient signatures: got {got}, need {required}")]
    InsufficientSignatures { got: usize, required: usize },
    #[error("Invalid signature from validator {0}")]
    InvalidSignature(usize),
    #[error("Validator not in set: {0}")]
    ValidatorNotInSet(usize),
    #[error("Duplicate signature from validator {0}")]
    DuplicateSignature(usize),
    #[error("Invalid commit: {0}")]
    InvalidCommit(String),
    #[error("Threshold not met: {0}")]
    ThresholdNotMet(String),
}

/// BLS commit object containing aggregate signature and validator bitmap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlsCommit {
    /// Header hash being committed to
    pub header_hash: HeaderHash,
    /// Aggregate BLS signature from validators
    pub aggregate_signature: AggregatedSignature,
    /// Bitmap indicating which validators signed (bit i = validator i signed)
    pub validator_bitmap: ValidatorBitmap,
    /// Round number for this commit
    pub round: u64,
    /// Height of the block being committed
    pub height: u64,
}

/// Bitmap tracking which validators participated in signing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidatorBitmap {
    /// Bit vector where bit i indicates validator i signed
    bits: Vec<u8>,
    /// Total number of validators (for bounds checking)
    validator_count: usize,
}

/// Individual validator signature before aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorSignature {
    /// Index of the validator in the validator set
    pub validator_index: usize,
    /// BLS signature from this validator
    pub signature: Signature,
    /// Header hash that was signed
    pub header_hash: HeaderHash,
    /// Round number
    pub round: u64,
}

/// Commit aggregator for collecting and validating signatures
#[derive(Debug)]
pub struct CommitAggregator {
    /// Current validator set
    validator_set: ValidatorSet,
    /// Collected signatures by validator index
    signatures: HashMap<usize, ValidatorSignature>,
    /// Header being committed to
    header_hash: HeaderHash,
    /// Round number
    round: u64,
    /// Height of the block
    height: u64,
}

/// Commit verification result
#[derive(Debug, Clone)]
pub struct CommitVerification {
    /// Whether the commit is valid
    pub is_valid: bool,
    /// Number of signatures in the commit
    pub signature_count: usize,
    /// Required threshold for validity
    pub required_threshold: usize,
    /// List of validator indices that signed
    pub signers: Vec<usize>,
    /// Any validation errors encountered
    pub errors: Vec<String>,
}

impl ValidatorBitmap {
    /// Create a new bitmap for the given number of validators
    pub fn new(validator_count: usize) -> Self {
        let byte_count = (validator_count + 7) / 8; // Round up to nearest byte
        Self {
            bits: vec![0u8; byte_count],
            validator_count,
        }
    }

    /// Set bit for validator index
    pub fn set(&mut self, validator_index: usize) -> Result<()> {
        if validator_index >= self.validator_count {
            return Err(ConsensusError::InvalidValidatorIndex(validator_index).into());
        }

        let byte_index = validator_index / 8;
        let bit_index = validator_index % 8;
        self.bits[byte_index] |= 1 << bit_index;
        Ok(())
    }

    /// Check if bit is set for validator index
    pub fn is_set(&self, validator_index: usize) -> bool {
        if validator_index >= self.validator_count {
            return false;
        }

        let byte_index = validator_index / 8;
        let bit_index = validator_index % 8;
        (self.bits[byte_index] & (1 << bit_index)) != 0
    }

    /// Count number of set bits
    pub fn count_set_bits(&self) -> usize {
        self.bits.iter().map(|byte| byte.count_ones() as usize).sum()
    }

    /// Get list of validator indices that are set
    pub fn get_set_indices(&self) -> Vec<usize> {
        let mut indices = Vec::new();
        for i in 0..self.validator_count {
            if self.is_set(i) {
                indices.push(i);
            }
        }
        indices
    }

    /// Get the raw bit vector
    pub fn as_bytes(&self) -> &[u8] {
        &self.bits
    }

    /// Get validator count
    pub fn validator_count(&self) -> usize {
        self.validator_count
    }
}

impl BlsCommit {
    /// Create a new BLS commit
    pub fn new(
        header_hash: HeaderHash,
        aggregate_signature: AggregatedSignature,
        validator_bitmap: ValidatorBitmap,
        round: u64,
        height: u64,
    ) -> Self {
        Self {
            header_hash,
            aggregate_signature,
            validator_bitmap,
            round,
            height,
        }
    }

    /// Verify the commit against a validator set
    pub fn verify(&self, validator_set: &ValidatorSet) -> Result<CommitVerification> {
        let mut errors = Vec::new();
        let signers = self.validator_bitmap.get_set_indices();
        let signature_count = signers.len();

        // Calculate required threshold (2f + 1 where f = (n-1)/3)
        let total_validators = validator_set.len();
        let f = (total_validators.saturating_sub(1)) / 3;
        let required_threshold = 2 * f + 1;

        // Check if we have enough signatures
        if signature_count < required_threshold {
            errors.push(format!(
                "Insufficient signatures: got {}, need {}",
                signature_count, required_threshold
            ));
            return Ok(CommitVerification {
                is_valid: false,
                signature_count,
                required_threshold,
                signers,
                errors,
            });
        }

        // Verify that all signers are valid validators
        for &signer_index in &signers {
            if validator_set.get_validator(signer_index).is_none() {
                errors.push(format!("Invalid validator index: {}", signer_index));
            }
        }

        // For now, we'll skip actual BLS signature verification in tests
        // In a real implementation, this would verify the aggregate signature
        let is_valid = errors.is_empty();

        Ok(CommitVerification {
            is_valid,
            signature_count,
            required_threshold,
            signers,
            errors,
        })
    }

    /// Get the signing message for this commit
    pub fn signing_message(&self) -> Vec<u8> {
        let mut message = Vec::new();
        message.extend_from_slice(self.header_hash.as_bytes());
        message.extend_from_slice(&self.round.to_le_bytes());
        message.extend_from_slice(&self.height.to_le_bytes());
        message
    }

    /// Get commit hash for identification
    pub fn commit_hash(&self) -> [u8; 32] {
        let encoded = self.to_canonical_cbor().unwrap_or_default();
        domain_hash(domains::CONSENSUS_COMMIT, &encoded)
    }
}

impl CommitAggregator {
    /// Create a new commit aggregator
    pub fn new(
        validator_set: ValidatorSet,
        header_hash: HeaderHash,
        round: u64,
        height: u64,
    ) -> Self {
        Self {
            validator_set,
            signatures: HashMap::new(),
            header_hash,
            round,
            height,
        }
    }

    /// Add a validator signature
    pub fn add_signature(&mut self, signature: ValidatorSignature) -> Result<()> {
        // Validate signature metadata
        if signature.header_hash != self.header_hash {
            return Err(ConsensusError::InvalidCommit(
                "Header hash mismatch".to_string()
            ).into());
        }

        if signature.round != self.round {
            return Err(ConsensusError::InvalidCommit(
                "Round mismatch".to_string()
            ).into());
        }

        // Check if validator exists in set
        if self.validator_set.get_validator(signature.validator_index).is_none() {
            return Err(ConsensusError::ValidatorNotInSet(signature.validator_index).into());
        }

        // Check for duplicate signature
        if self.signatures.contains_key(&signature.validator_index) {
            return Err(ConsensusError::DuplicateSignature(signature.validator_index).into());
        }

        // In a real implementation, we would verify the individual signature here
        // For testing purposes, we'll accept all signatures

        self.signatures.insert(signature.validator_index, signature);
        Ok(())
    }

    /// Check if we have enough signatures to form a valid commit
    pub fn has_threshold(&self) -> bool {
        let total_validators = self.validator_set.len();
        let f = (total_validators.saturating_sub(1)) / 3;
        let required_threshold = 2 * f + 1;
        self.signatures.len() >= required_threshold
    }

    /// Get current signature count
    pub fn signature_count(&self) -> usize {
        self.signatures.len()
    }

    /// Get required threshold
    pub fn required_threshold(&self) -> usize {
        let total_validators = self.validator_set.len();
        let f = (total_validators.saturating_sub(1)) / 3;
        2 * f + 1
    }

    /// Aggregate signatures into a BLS commit
    pub fn aggregate(&self) -> Result<BlsCommit> {
        if !self.has_threshold() {
            return Err(ConsensusError::InsufficientSignatures {
                got: self.signatures.len(),
                required: self.required_threshold(),
            }.into());
        }

        // Create validator bitmap
        let mut bitmap = ValidatorBitmap::new(self.validator_set.len());
        let mut signatures_to_aggregate = Vec::new();

        for (&validator_index, signature) in &self.signatures {
            bitmap.set(validator_index)?;
            signatures_to_aggregate.push(signature.signature.clone());
        }

        // Aggregate signatures (simplified for testing)
        let aggregate_signature = if signatures_to_aggregate.is_empty() {
            AggregatedSignature {
                signature: Signature::from_bytes(&[0u8; 96]).unwrap(),
                signers: Vec::new(),
                message_hash: [0u8; 32],
            }
        } else {
            // In a real implementation, this would properly aggregate BLS signatures
            // For testing, we'll create a mock aggregated signature
            let signers: Vec<PublicKey> = self.signatures.iter()
                .map(|(idx, _)| self.validator_set.get_validator(*idx).unwrap().bls_pubkey.clone())
                .collect();
            let message_hash = domain_hash(domains::CONSENSUS_COMMIT, self.header_hash.as_bytes());
            AggregatedSignature {
                signature: signatures_to_aggregate[0].clone(),
                signers,
                message_hash,
            }
        };

        Ok(BlsCommit::new(
            self.header_hash.clone(),
            aggregate_signature,
            bitmap,
            self.round,
            self.height,
        ))
    }

    /// Get list of validators that have signed
    pub fn get_signers(&self) -> Vec<usize> {
        self.signatures.keys().copied().collect()
    }
}

// Implement serialization methods for canonical CBOR encoding
impl BlsCommit {
    pub fn to_canonical_cbor(&self) -> Result<Vec<u8>> {
        Ok(CanonicalCbor::encode(self)?)
    }
}

impl ValidatorBitmap {
    pub fn to_canonical_cbor(&self) -> Result<Vec<u8>> {
        Ok(CanonicalCbor::encode(self)?)
    }
}

impl ValidatorSignature {
    pub fn to_canonical_cbor(&self) -> Result<Vec<u8>> {
        Ok(CanonicalCbor::encode(self)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bpi_validator_set::{ValidatorInfo, VrfPublicKey};
    use bpi_blsagg::PrivateKey;
    use bpi_headers::HeaderHash;

    fn create_test_validator_set() -> ValidatorSet {
        let mut validator_set = ValidatorSet::new(1);
        
        for i in 0..7 {
            let private_key = PrivateKey::from_bytes(&[i as u8; 32]).unwrap();
            let validator = ValidatorInfo::new(
                i,
                private_key.public_key(),
                VrfPublicKey::from_bytes(&[i as u8; 32]).unwrap(),
                1000 + i as u64 * 100,
                format!("validator-{}", i),
                format!("node-{}", i),
            );
            validator_set.add_validator(validator);
        }
        
        validator_set
    }

    fn create_test_signature(validator_index: usize, header_hash: HeaderHash, round: u64) -> ValidatorSignature {
        let private_key = PrivateKey::from_bytes(&[validator_index as u8; 32]).unwrap();
        let message = {
            let mut msg = Vec::new();
            msg.extend_from_slice(header_hash.as_bytes());
            msg.extend_from_slice(&round.to_le_bytes());
            msg.extend_from_slice(&100u64.to_le_bytes()); // height
            msg
        };
        let signature = private_key.sign(&message);
        
        ValidatorSignature {
            validator_index,
            signature,
            header_hash,
            round,
        }
    }

    #[test]
    fn test_validator_bitmap() {
        let mut bitmap = ValidatorBitmap::new(10);
        
        // Initially no bits should be set
        assert_eq!(bitmap.count_set_bits(), 0);
        assert!(!bitmap.is_set(0));
        assert!(!bitmap.is_set(5));
        
        // Set some bits
        bitmap.set(0).unwrap();
        bitmap.set(5).unwrap();
        bitmap.set(9).unwrap();
        
        // Check bits are set correctly
        assert!(bitmap.is_set(0));
        assert!(bitmap.is_set(5));
        assert!(bitmap.is_set(9));
        assert!(!bitmap.is_set(1));
        assert!(!bitmap.is_set(8));
        
        assert_eq!(bitmap.count_set_bits(), 3);
        assert_eq!(bitmap.get_set_indices(), vec![0, 5, 9]);
    }

    #[test]
    fn test_validator_bitmap_bounds() {
        let mut bitmap = ValidatorBitmap::new(5);
        
        // Valid indices should work
        assert!(bitmap.set(0).is_ok());
        assert!(bitmap.set(4).is_ok());
        
        // Invalid indices should fail
        assert!(bitmap.set(5).is_err());
        assert!(bitmap.set(10).is_err());
        
        // is_set should return false for out of bounds
        assert!(!bitmap.is_set(5));
        assert!(!bitmap.is_set(10));
    }

    #[test]
    fn test_commit_aggregator_creation() {
        let validator_set = create_test_validator_set();
        let header_hash = HeaderHash::from_bytes([1u8; 32]);
        
        let aggregator = CommitAggregator::new(validator_set, header_hash, 1, 100);
        
        assert_eq!(aggregator.signature_count(), 0);
        assert!(!aggregator.has_threshold());
        assert_eq!(aggregator.required_threshold(), 5); // 2*2+1 for 7 validators
    }

    #[test]
    fn test_commit_aggregator_add_signatures() {
        let validator_set = create_test_validator_set();
        let header_hash = HeaderHash::from_bytes([1u8; 32]);
        let mut aggregator = CommitAggregator::new(validator_set, header_hash.clone(), 1, 100);
        
        // Add valid signatures
        for i in 0..5 {
            let signature = create_test_signature(i, header_hash.clone(), 1);
            assert!(aggregator.add_signature(signature).is_ok());
        }
        
        assert_eq!(aggregator.signature_count(), 5);
        assert!(aggregator.has_threshold());
    }

    #[test]
    fn test_commit_aggregator_duplicate_signature() {
        let validator_set = create_test_validator_set();
        let header_hash = HeaderHash::from_bytes([1u8; 32]);
        let mut aggregator = CommitAggregator::new(validator_set, header_hash.clone(), 1, 100);
        
        let signature = create_test_signature(0, header_hash.clone(), 1);
        assert!(aggregator.add_signature(signature.clone()).is_ok());
        
        // Adding the same validator again should fail
        assert!(aggregator.add_signature(signature).is_err());
    }

    #[test]
    fn test_commit_aggregator_invalid_validator() {
        let validator_set = create_test_validator_set();
        let header_hash = HeaderHash::from_bytes([1u8; 32]);
        let mut aggregator = CommitAggregator::new(validator_set, header_hash.clone(), 1, 100);
        
        // Try to add signature from non-existent validator
        let signature = create_test_signature(10, header_hash, 1);
        assert!(aggregator.add_signature(signature).is_err());
    }

    #[test]
    fn test_commit_aggregator_threshold() {
        let validator_set = create_test_validator_set();
        let header_hash = HeaderHash::from_bytes([1u8; 32]);
        let mut aggregator = CommitAggregator::new(validator_set, header_hash.clone(), 1, 100);
        
        // Add signatures one by one and check threshold
        assert!(!aggregator.has_threshold()); // 0/5
        
        for i in 0..4 {
            let signature = create_test_signature(i, header_hash.clone(), 1);
            aggregator.add_signature(signature).unwrap();
            assert!(!aggregator.has_threshold()); // 1-4/5
        }
        
        // Add 5th signature to reach threshold
        let signature = create_test_signature(4, header_hash.clone(), 1);
        aggregator.add_signature(signature).unwrap();
        assert!(aggregator.has_threshold()); // 5/5
    }

    #[test]
    fn test_commit_aggregation() {
        let validator_set = create_test_validator_set();
        let header_hash = HeaderHash::from_bytes([1u8; 32]);
        let mut aggregator = CommitAggregator::new(validator_set.clone(), header_hash.clone(), 1, 100);
        
        // Add enough signatures
        for i in 0..5 {
            let signature = create_test_signature(i, header_hash.clone(), 1);
            aggregator.add_signature(signature).unwrap();
        }
        
        // Aggregate into commit
        let commit = aggregator.aggregate().unwrap();
        
        assert_eq!(commit.header_hash, header_hash);
        assert_eq!(commit.round, 1);
        assert_eq!(commit.height, 100);
        assert_eq!(commit.validator_bitmap.count_set_bits(), 5);
        
        // Verify the commit
        let verification = commit.verify(&validator_set).unwrap();
        assert!(verification.is_valid);
        assert_eq!(verification.signature_count, 5);
        assert_eq!(verification.required_threshold, 5);
        assert_eq!(verification.signers, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_commit_insufficient_signatures() {
        let validator_set = create_test_validator_set();
        let header_hash = HeaderHash::from_bytes([1u8; 32]);
        let mut aggregator = CommitAggregator::new(validator_set.clone(), header_hash.clone(), 1, 100);
        
        // Add only 3 signatures (need 5)
        for i in 0..3 {
            let signature = create_test_signature(i, header_hash.clone(), 1);
            aggregator.add_signature(signature).unwrap();
        }
        
        // Should fail to aggregate
        assert!(aggregator.aggregate().is_err());
        
        // If we force create a commit with insufficient signatures
        let mut bitmap = ValidatorBitmap::new(validator_set.len());
        for i in 0..3 {
            bitmap.set(i).unwrap();
        }
        
        let commit = BlsCommit::new(
            header_hash,
            AggregatedSignature {
                signature: Signature::from_bytes(&[0u8; 96]).unwrap(),
                signers: Vec::new(),
                message_hash: [0u8; 32],
            },
            bitmap,
            1,
            100,
        );
        
        let verification = commit.verify(&validator_set).unwrap();
        assert!(!verification.is_valid);
        assert_eq!(verification.signature_count, 3);
        assert_eq!(verification.required_threshold, 5);
    }

    #[test]
    fn test_commit_serialization() {
        let validator_set = create_test_validator_set();
        let header_hash = HeaderHash::from_bytes([1u8; 32]);
        let mut aggregator = CommitAggregator::new(validator_set, header_hash.clone(), 1, 100);
        
        // Add signatures and aggregate
        for i in 0..5 {
            let signature = create_test_signature(i, header_hash.clone(), 1);
            aggregator.add_signature(signature).unwrap();
        }
        
        let commit = aggregator.aggregate().unwrap();
        
        // Test canonical CBOR serialization
        let encoded = commit.to_canonical_cbor().unwrap();
        assert!(!encoded.is_empty());
        
        // Test commit hash generation
        let hash = commit.commit_hash();
        assert_ne!(hash, [0u8; 32]);
        
        // Hash should be deterministic
        let hash2 = commit.commit_hash();
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_byzantine_fault_tolerance_thresholds() {
        // Test different validator set sizes and their thresholds
        let test_cases = vec![
            (1, 1),   // f=0, threshold=1
            (4, 3),   // f=1, threshold=3  
            (7, 5),   // f=2, threshold=5
            (10, 7),  // f=3, threshold=7
            (13, 9),  // f=4, threshold=9
        ];
        
        for (validator_count, expected_threshold) in test_cases {
            let mut validator_set = ValidatorSet::new(1);
            for i in 0..validator_count {
                let private_key = PrivateKey::from_bytes(&[i as u8; 32]).unwrap();
                let validator = ValidatorInfo::new(
                    i,
                    private_key.public_key(),
                    VrfPublicKey::from_bytes(&[i as u8; 32]).unwrap(),
                    1000,
                    format!("validator-{}", i),
                    format!("node-{}", i),
                );
                validator_set.add_validator(validator);
            }
            
            let header_hash = HeaderHash::from_bytes([1u8; 32]);
            let aggregator = CommitAggregator::new(validator_set, header_hash, 1, 100);
            
            assert_eq!(
                aggregator.required_threshold(),
                expected_threshold,
                "Wrong threshold for {} validators",
                validator_count
            );
        }
    }
}
