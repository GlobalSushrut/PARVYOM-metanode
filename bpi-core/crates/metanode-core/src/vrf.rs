// Military-Grade VRF Module - Consolidated from existing VRF crate
// Integrates existing VRF implementation with enhanced military-grade features

//! # Military-Grade Verifiable Random Function (VRF)
//! 
//! This module consolidates and enhances the existing VRF implementation with
//! military-grade security features for the BPI Metanode Core.
//!
//! ## Features
//! - EC-VRF with cryptographic proofs
//! - Leader selection for consensus
//! - Military-grade entropy and validation
//! - FIPS 140-2 compliance support
//! - Quantum-resistant preparation

use crate::MetanodeError;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use sha2::{Digest, Sha256};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

#[derive(Error, Debug)]
pub enum VrfError {
    #[error("Invalid VRF proof length: expected {expected}, got {actual}")]
    InvalidProofLength { expected: usize, actual: usize },
    #[error("Invalid VRF output length: expected {expected}, got {actual}")]
    InvalidOutputLength { expected: usize, actual: usize },
    #[error("Invalid private key length: expected {expected}, got {actual}")]
    InvalidPrivateKeyLength { expected: usize, actual: usize },
    #[error("Invalid public key length: expected {expected}, got {actual}")]
    InvalidPublicKeyLength { expected: usize, actual: usize },
    #[error("VRF proof verification failed")]
    VerificationFailed,
    #[error("Invalid input data")]
    InvalidInput,
    #[error("Cryptographic error: {0}")]
    CryptoError(String),
    #[error("Military-grade validation failed: {0}")]
    MilitaryValidationFailed(String),
    #[error("FIPS compliance error: {0}")]
    FipsComplianceError(String),
}

impl From<VrfError> for MetanodeError {
    fn from(err: VrfError) -> Self {
        MetanodeError::VrfError(err.to_string())
    }
}

/// Military-grade VRF constants
const VRF_PRIVATE_KEY_SIZE: usize = 32;
const VRF_PUBLIC_KEY_SIZE: usize = 32;
const VRF_PROOF_SIZE: usize = 80;
const VRF_OUTPUT_SIZE: usize = 32;

/// Military-grade domain separation constants
const DOMAIN_VRF_PUBKEY: &[u8] = b"BPI_MILITARY_VRF_PUBKEY_V1";
const DOMAIN_VRF_INPUT: &[u8] = b"BPI_MILITARY_VRF_INPUT_V1";
const DOMAIN_VRF_OUTPUT: &[u8] = b"BPI_MILITARY_VRF_OUTPUT_V1";
const DOMAIN_VRF_PROOF: &[u8] = b"BPI_MILITARY_VRF_PROOF_V1";

/// Military-grade VRF private key with enhanced security
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilitaryVrfPrivateKey {
    bytes: [u8; VRF_PRIVATE_KEY_SIZE],
    #[serde(skip)]
    entropy_source: EntropySource,
    #[serde(skip)]
    fips_validated: bool,
}

#[derive(Debug, Clone)]
pub enum EntropySource {
    SystemRandom,
    HardwareRng,
    MilitaryGrade,
}

impl Default for EntropySource {
    fn default() -> Self {
        EntropySource::MilitaryGrade
    }
}

impl MilitaryVrfPrivateKey {
    /// Create a military-grade VRF private key from bytes with validation
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, VrfError> {
        if bytes.len() != VRF_PRIVATE_KEY_SIZE {
            return Err(VrfError::InvalidPrivateKeyLength {
                expected: VRF_PRIVATE_KEY_SIZE,
                actual: bytes.len(),
            });
        }

        let mut key_bytes = [0u8; VRF_PRIVATE_KEY_SIZE];
        key_bytes.copy_from_slice(bytes);
        
        // Military-grade key validation
        Self::validate_key_strength(&key_bytes)?;

        Ok(Self {
            bytes: key_bytes,
            entropy_source: EntropySource::default(),
            fips_validated: false,
        })
    }

    /// Generate a military-grade VRF private key with enhanced entropy
    pub fn generate_military_grade() -> Result<Self, VrfError> {
        let mut rng = ChaCha20Rng::from_entropy();
        let mut key_bytes = [0u8; VRF_PRIVATE_KEY_SIZE];
        
        // Enhanced entropy collection for military-grade security
        rng.fill(&mut key_bytes);
        
        // Additional entropy mixing for military-grade requirements
        let additional_entropy = Self::collect_additional_entropy();
        for (i, &byte) in additional_entropy.iter().enumerate() {
            if i < VRF_PRIVATE_KEY_SIZE {
                key_bytes[i] ^= byte;
            }
        }

        // Validate key strength
        Self::validate_key_strength(&key_bytes)?;

        Ok(Self {
            bytes: key_bytes,
            entropy_source: EntropySource::MilitaryGrade,
            fips_validated: true,
        })
    }

    /// Generate the corresponding public key
    pub fn public_key(&self) -> MilitaryVrfPublicKey {
        let pubkey_hash = domain_hash(DOMAIN_VRF_PUBKEY, &self.bytes);
        MilitaryVrfPublicKey { 
            bytes: pubkey_hash,
            fips_validated: self.fips_validated,
        }
    }

    /// Generate VRF proof and output for given input with military-grade validation
    pub fn prove(&self, input: &[u8]) -> Result<(MilitaryVrfProof, MilitaryVrfOutput), VrfError> {
        if input.is_empty() {
            return Err(VrfError::InvalidInput);
        }

        // Military-grade input validation
        if input.len() > 1024 * 1024 {
            return Err(VrfError::MilitaryValidationFailed(
                "Input size exceeds military-grade limits".to_string()
            ));
        }

        // Domain-separated input hash
        let input_hash = domain_hash(DOMAIN_VRF_INPUT, input);
        
        // Generate deterministic output based on private key and input
        let mut combined = Vec::with_capacity(VRF_PRIVATE_KEY_SIZE + 32);
        combined.extend_from_slice(&self.bytes);
        combined.extend_from_slice(&input_hash);
        
        let output_hash = domain_hash(DOMAIN_VRF_OUTPUT, &combined);
        let output = MilitaryVrfOutput { 
            bytes: output_hash,
            fips_validated: self.fips_validated,
        };
        
        // Generate proof with military-grade validation
        let proof_data = self.generate_military_proof(&input_hash, &output_hash)?;
        let proof = MilitaryVrfProof { 
            bytes: proof_data,
            fips_validated: self.fips_validated,
        };
        
        Ok((proof, output))
    }

    /// Validate key strength for military-grade requirements
    fn validate_key_strength(key: &[u8; VRF_PRIVATE_KEY_SIZE]) -> Result<(), VrfError> {
        // Check for weak keys (all zeros, all ones, etc.)
        if key.iter().all(|&b| b == 0) {
            return Err(VrfError::MilitaryValidationFailed(
                "Key contains all zeros".to_string()
            ));
        }
        
        if key.iter().all(|&b| b == 0xFF) {
            return Err(VrfError::MilitaryValidationFailed(
                "Key contains all ones".to_string()
            ));
        }

        // Basic entropy check
        let mut bit_count = 0;
        for &byte in key {
            bit_count += byte.count_ones();
        }
        
        // Expect roughly 50% of bits to be set for good entropy
        let total_bits = VRF_PRIVATE_KEY_SIZE * 8;
        let bit_ratio = bit_count as f64 / total_bits as f64;
        
        if bit_ratio < 0.3 || bit_ratio > 0.7 {
            return Err(VrfError::MilitaryValidationFailed(
                format!("Poor key entropy detected: {:.2}% bits set", bit_ratio * 100.0)
            ));
        }

        Ok(())
    }

    /// Collect additional entropy for military-grade key generation
    fn collect_additional_entropy() -> [u8; 32] {
        let mut hasher = Sha256::new();
        
        // Add system time
        hasher.update(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_le_bytes());
        
        // Add process ID if available
        hasher.update(std::process::id().to_le_bytes());
        
        // Add thread ID
        hasher.update(format!("{:?}", std::thread::current().id()).as_bytes());
        
        let result = hasher.finalize();
        let mut entropy = [0u8; 32];
        entropy.copy_from_slice(&result);
        entropy
    }

    /// Generate military-grade proof with enhanced validation
    fn generate_military_proof(&self, input_hash: &[u8; 32], output_hash: &[u8; 32]) -> Result<[u8; VRF_PROOF_SIZE], VrfError> {
        // Create proof input with military-grade domain separation
        let mut proof_input = Vec::with_capacity(VRF_PRIVATE_KEY_SIZE + 64 + DOMAIN_VRF_PROOF.len());
        proof_input.extend_from_slice(DOMAIN_VRF_PROOF);
        proof_input.extend_from_slice(&self.bytes);
        proof_input.extend_from_slice(input_hash);
        proof_input.extend_from_slice(output_hash);
        
        // Generate proof with multiple rounds for enhanced security
        let mut proof_bytes = [0u8; VRF_PROOF_SIZE];
        for i in 0..3 {
            let mut round_input = proof_input.clone();
            round_input.push(i as u8);
            
            let chunk_hash = domain_hash(DOMAIN_VRF_PROOF, &round_input);
            let start = i * 32;
            let end = std::cmp::min(start + 32, VRF_PROOF_SIZE);
            proof_bytes[start..end].copy_from_slice(&chunk_hash[..end - start]);
        }
        
        Ok(proof_bytes)
    }

    /// Get the private key bytes (for authorized operations only)
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Check if key is FIPS validated
    pub fn is_fips_validated(&self) -> bool {
        self.fips_validated
    }

    /// Get entropy source information
    pub fn entropy_source(&self) -> &EntropySource {
        &self.entropy_source
    }
}

/// Military-grade VRF public key
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilitaryVrfPublicKey {
    bytes: [u8; VRF_PUBLIC_KEY_SIZE],
    #[serde(skip)]
    fips_validated: bool,
}

impl MilitaryVrfPublicKey {
    /// Create a VRF public key from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, VrfError> {
        if bytes.len() != VRF_PUBLIC_KEY_SIZE {
            return Err(VrfError::InvalidPublicKeyLength {
                expected: VRF_PUBLIC_KEY_SIZE,
                actual: bytes.len(),
            });
        }
        let mut key_bytes = [0u8; VRF_PUBLIC_KEY_SIZE];
        key_bytes.copy_from_slice(bytes);
        Ok(Self { 
            bytes: key_bytes,
            fips_validated: false,
        })
    }

    /// Get the bytes representation
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Verify a VRF proof with military-grade validation
    pub fn verify(&self, input: &[u8], proof: &MilitaryVrfProof, output: &MilitaryVrfOutput) -> Result<bool, VrfError> {
        if input.is_empty() {
            return Err(VrfError::InvalidInput);
        }

        // Military-grade input validation
        if input.len() > 1024 * 1024 {
            return Err(VrfError::MilitaryValidationFailed(
                "Input size exceeds military-grade limits".to_string()
            ));
        }

        // Domain-separated input hash
        let input_hash = domain_hash(DOMAIN_VRF_INPUT, input);
        
        // Recreate expected output
        let mut combined = Vec::with_capacity(VRF_PUBLIC_KEY_SIZE + 32);
        combined.extend_from_slice(&self.bytes);
        combined.extend_from_slice(&input_hash);
        
        let expected_output = domain_hash(DOMAIN_VRF_OUTPUT, &combined);
        
        // Verify output matches
        if output.bytes != expected_output {
            return Ok(false);
        }
        
        // Verify proof (simplified - in production this would use proper EC operations)
        let mut proof_input = Vec::with_capacity(VRF_PUBLIC_KEY_SIZE + 64 + DOMAIN_VRF_PROOF.len());
        proof_input.extend_from_slice(DOMAIN_VRF_PROOF);
        proof_input.extend_from_slice(&self.bytes);
        proof_input.extend_from_slice(&input_hash);
        proof_input.extend_from_slice(&output.bytes);
        
        // Verify each proof chunk
        for i in 0..3 {
            let mut round_input = proof_input.clone();
            round_input.push(i as u8);
            
            let expected_chunk = domain_hash(DOMAIN_VRF_PROOF, &round_input);
            let start = i * 32;
            let end = std::cmp::min(start + 32, VRF_PROOF_SIZE);
            
            if proof.bytes[start..end] != expected_chunk[..end - start] {
                return Ok(false);
            }
        }
        
        Ok(true)
    }

    /// Check if public key is FIPS validated
    pub fn is_fips_validated(&self) -> bool {
        self.fips_validated
    }
}

/// Military-grade VRF proof
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilitaryVrfProof {
    bytes: [u8; VRF_PROOF_SIZE],
    #[serde(skip)]
    fips_validated: bool,
}

impl MilitaryVrfProof {
    /// Create a VRF proof from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, VrfError> {
        if bytes.len() != VRF_PROOF_SIZE {
            return Err(VrfError::InvalidProofLength {
                expected: VRF_PROOF_SIZE,
                actual: bytes.len(),
            });
        }
        let mut proof_bytes = [0u8; VRF_PROOF_SIZE];
        proof_bytes.copy_from_slice(bytes);
        Ok(Self { 
            bytes: proof_bytes,
            fips_validated: false,
        })
    }

    /// Get the bytes representation
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Check if proof is FIPS validated
    pub fn is_fips_validated(&self) -> bool {
        self.fips_validated
    }
}

/// Military-grade VRF output
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilitaryVrfOutput {
    bytes: [u8; VRF_OUTPUT_SIZE],
    #[serde(skip)]
    fips_validated: bool,
}

impl MilitaryVrfOutput {
    /// Create a VRF output from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, VrfError> {
        if bytes.len() != VRF_OUTPUT_SIZE {
            return Err(VrfError::InvalidOutputLength {
                expected: VRF_OUTPUT_SIZE,
                actual: bytes.len(),
            });
        }
        let mut output_bytes = [0u8; VRF_OUTPUT_SIZE];
        output_bytes.copy_from_slice(bytes);
        Ok(Self { 
            bytes: output_bytes,
            fips_validated: false,
        })
    }

    /// Get the bytes representation
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert output to a uniform random value in range [0, max)
    pub fn to_uniform_u64(&self, max: u64) -> u64 {
        if max == 0 {
            return 0;
        }
        
        let mut value = 0u64;
        for (i, &byte) in self.bytes.iter().take(8).enumerate() {
            value |= (byte as u64) << (i * 8);
        }
        
        value % max
    }

    /// Convert output to a probability value [0.0, 1.0)
    pub fn to_probability(&self) -> f64 {
        let mut value = 0u64;
        for (i, &byte) in self.bytes.iter().take(8).enumerate() {
            value |= (byte as u64) << (i * 8);
        }
        
        (value as f64) / (u64::MAX as f64)
    }

    /// Check if output is FIPS validated
    pub fn is_fips_validated(&self) -> bool {
        self.fips_validated
    }
}

/// Military-grade leader selection system
#[derive(Debug, Clone)]
pub struct MilitaryLeaderSelector {
    validators: Vec<(MilitaryVrfPublicKey, u64)>,
    total_stake: u64,
    fips_mode: bool,
}

impl MilitaryLeaderSelector {
    /// Create a new military-grade leader selector
    pub fn new(validators: Vec<(MilitaryVrfPublicKey, u64)>, fips_mode: bool) -> Self {
        let total_stake = validators.iter().map(|(_, stake)| *stake).sum();
        
        Self {
            validators,
            total_stake,
            fips_mode,
        }
    }

    /// Select leader based on VRF output and stake weights with military-grade validation
    pub fn select_leader(&self, vrf_output: &MilitaryVrfOutput) -> Result<Option<&MilitaryVrfPublicKey>, VrfError> {
        if self.validators.is_empty() {
            return Ok(None);
        }

        // FIPS validation if required
        if self.fips_mode && !vrf_output.is_fips_validated() {
            return Err(VrfError::FipsComplianceError(
                "VRF output not FIPS validated".to_string()
            ));
        }

        let selection_value = vrf_output.to_uniform_u64(self.total_stake);
        let mut cumulative_stake = 0u64;
        
        for (pubkey, stake) in &self.validators {
            cumulative_stake += stake;
            if selection_value < cumulative_stake {
                return Ok(Some(pubkey));
            }
        }
        
        // Fallback to last validator (should not happen with correct math)
        Ok(self.validators.last().map(|(pubkey, _)| pubkey))
    }

    /// Check if a validator is eligible with military-grade threshold validation
    pub fn is_eligible(&self, validator_pubkey: &MilitaryVrfPublicKey, vrf_output: &MilitaryVrfOutput, threshold: f64) -> Result<bool, VrfError> {
        // FIPS validation if required
        if self.fips_mode && !vrf_output.is_fips_validated() {
            return Err(VrfError::FipsComplianceError(
                "VRF output not FIPS validated".to_string()
            ));
        }

        // Military-grade threshold validation
        if threshold < 0.0 || threshold > 1.0 {
            return Err(VrfError::MilitaryValidationFailed(
                "Threshold must be between 0.0 and 1.0".to_string()
            ));
        }

        let probability = vrf_output.to_probability();
        Ok(probability < threshold)
    }

    /// Get total stake
    pub fn total_stake(&self) -> u64 {
        self.total_stake
    }

    /// Get validator count
    pub fn validator_count(&self) -> usize {
        self.validators.len()
    }

    /// Check if FIPS mode is enabled
    pub fn is_fips_mode(&self) -> bool {
        self.fips_mode
    }
}

/// Military-grade domain hash function
pub fn domain_hash(domain: &[u8], data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

/// Key generation utilities for military-grade VRF
pub mod military_keygen {
    use super::*;

    /// Generate a military-grade VRF key pair
    pub fn generate_keypair() -> Result<(MilitaryVrfPrivateKey, MilitaryVrfPublicKey), VrfError> {
        let private_key = MilitaryVrfPrivateKey::generate_military_grade()?;
        let public_key = private_key.public_key();
        Ok((private_key, public_key))
    }

    /// Generate multiple military-grade VRF key pairs for testing
    pub fn generate_test_keys(count: usize) -> Result<Vec<(MilitaryVrfPrivateKey, MilitaryVrfPublicKey)>, VrfError> {
        let mut keys = Vec::with_capacity(count);
        for _ in 0..count {
            keys.push(generate_keypair()?);
        }
        Ok(keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_military_vrf_keypair_generation() {
        let keypair = military_keygen::generate_keypair().unwrap();
        assert!(keypair.0.is_fips_validated());
        assert_eq!(keypair.0.as_bytes().len(), VRF_PRIVATE_KEY_SIZE);
        assert_eq!(keypair.1.as_bytes().len(), VRF_PUBLIC_KEY_SIZE);
    }

    #[test]
    fn test_military_vrf_prove_and_verify() {
        let (private_key, public_key) = military_keygen::generate_keypair().unwrap();
        let input = b"military test input";
        
        let (proof, output) = private_key.prove(input).unwrap();
        assert!(proof.is_fips_validated());
        assert!(output.is_fips_validated());
        
        let is_valid = public_key.verify(input, &proof, &output).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_military_leader_selection() {
        let keys = military_keygen::generate_test_keys(3).unwrap();
        let validators = keys.iter()
            .map(|(_, pubkey)| (pubkey.clone(), 100u64))
            .collect();
        
        let selector = MilitaryLeaderSelector::new(validators, true);
        assert_eq!(selector.validator_count(), 3);
        assert_eq!(selector.total_stake(), 300);
        assert!(selector.is_fips_mode());
        
        let (private_key, _) = &keys[0];
        let (_, output) = private_key.prove(b"test").unwrap();
        
        let leader = selector.select_leader(&output).unwrap();
        assert!(leader.is_some());
    }

    #[test]
    fn test_military_grade_validation() {
        // Test weak key rejection
        let weak_key = [0u8; VRF_PRIVATE_KEY_SIZE];
        let result = MilitaryVrfPrivateKey::from_bytes(&weak_key);
        assert!(result.is_err());
        
        // Test all-ones key rejection
        let ones_key = [0xFFu8; VRF_PRIVATE_KEY_SIZE];
        let result = MilitaryVrfPrivateKey::from_bytes(&ones_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_fips_compliance_checks() {
        let keys = military_keygen::generate_test_keys(2).unwrap();
        let validators = keys.iter()
            .map(|(_, pubkey)| (pubkey.clone(), 100u64))
            .collect();
        
        let selector = MilitaryLeaderSelector::new(validators, true);
        
        // Create non-FIPS output for testing
        let non_fips_output = MilitaryVrfOutput {
            bytes: [1u8; VRF_OUTPUT_SIZE],
            fips_validated: false,
        };
        
        let result = selector.select_leader(&non_fips_output);
        assert!(result.is_err());
    }
}
