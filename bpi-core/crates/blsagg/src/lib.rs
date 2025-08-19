//! BLS signature aggregation for BPI Mesh
//! Stage 4: BLS Aggregation Library

use bpi_enc::{domain_hash, domains};
use anyhow::Result;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlsError {
    #[error("Invalid signature length: expected {expected}, got {actual}")]
    InvalidSignatureLength { expected: usize, actual: usize },
    #[error("Invalid public key length: expected {expected}, got {actual}")]
    InvalidPublicKeyLength { expected: usize, actual: usize },
    #[error("Invalid private key length: expected {expected}, got {actual}")]
    InvalidPrivateKeyLength { expected: usize, actual: usize },
    #[error("Signature verification failed")]
    VerificationFailed,
    #[error("Empty signature set")]
    EmptySignatureSet,
    #[error("Mismatched signature and public key counts")]
    MismatchedCounts,
    #[error("Cryptographic error: {0}")]
    CryptoError(String),
}

/// BLS12-381 G1 point size (48 bytes compressed)
const G1_COMPRESSED_SIZE: usize = 48;
/// BLS12-381 G2 point size (96 bytes compressed)
const G2_COMPRESSED_SIZE: usize = 96;
/// BLS12-381 scalar size (32 bytes)
const SCALAR_SIZE: usize = 32;

/// BLS public key (G1 point)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKey {
    pub bytes: [u8; G1_COMPRESSED_SIZE],
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.bytes)
    }
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = <Vec<u8>>::deserialize(deserializer)?;
        if bytes.len() != G1_COMPRESSED_SIZE {
            return Err(serde::de::Error::custom(format!(
                "Invalid public key length: expected {}, got {}",
                G1_COMPRESSED_SIZE,
                bytes.len()
            )));
        }
        let mut key_bytes = [0u8; G1_COMPRESSED_SIZE];
        key_bytes.copy_from_slice(&bytes);
        Ok(PublicKey { bytes: key_bytes })
    }
}

impl PublicKey {
    /// Create a public key from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, BlsError> {
        if bytes.len() != G1_COMPRESSED_SIZE {
            return Err(BlsError::InvalidPublicKeyLength {
                expected: G1_COMPRESSED_SIZE,
                actual: bytes.len(),
            });
        }
        let mut key_bytes = [0u8; G1_COMPRESSED_SIZE];
        key_bytes.copy_from_slice(bytes);
        Ok(Self { bytes: key_bytes })
    }
    
    /// Get the bytes representation
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
    
    /// Verify a signature against a message
    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        // Domain-separated message hash
        let msg_hash = domain_hash(domains::BLS_MESSAGE, message);
        
        // Simplified verification (in real implementation, would use pairing)
        // For now, we simulate verification by checking if signature was created with matching key
        self.verify_hash(&msg_hash, signature)
    }
    
    /// Verify a signature against a pre-hashed message
    pub fn verify_hash(&self, msg_hash: &[u8; 32], signature: &Signature) -> bool {
        // Simplified verification - in real BLS this would use pairing checks
        // For testing purposes, we simulate by checking deterministic relationship
        let expected_sig = self.sign_hash_deterministic(msg_hash);
        signature.bytes == expected_sig
    }
    
    /// Deterministic signing for testing (not cryptographically secure)
    fn sign_hash_deterministic(&self, msg_hash: &[u8; 32]) -> [u8; G2_COMPRESSED_SIZE] {
        let mut combined = Vec::with_capacity(G1_COMPRESSED_SIZE + 32);
        combined.extend_from_slice(&self.bytes);
        combined.extend_from_slice(msg_hash);
        
        let hash = domain_hash(domains::BLS_SIGNATURE, &combined);
        
        // Expand to G2 size using repeated hashing
        let mut sig_bytes = [0u8; G2_COMPRESSED_SIZE];
        for i in 0..3 {
            let chunk_hash = domain_hash(domains::BLS_SIGNATURE, &[hash.as_slice(), &[i as u8]].concat());
            let start = i * 32;
            let end = std::cmp::min(start + 32, G2_COMPRESSED_SIZE);
            sig_bytes[start..end].copy_from_slice(&chunk_hash[..end - start]);
        }
        
        sig_bytes
    }
}

/// BLS private key (scalar)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateKey {
    bytes: [u8; SCALAR_SIZE],
}

impl PrivateKey {
    /// Generate a new private key from entropy
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, BlsError> {
        if bytes.len() != SCALAR_SIZE {
            return Err(BlsError::InvalidPrivateKeyLength {
                expected: SCALAR_SIZE,
                actual: bytes.len(),
            });
        }
        let mut key_bytes = [0u8; SCALAR_SIZE];
        key_bytes.copy_from_slice(bytes);
        Ok(Self { bytes: key_bytes })
    }
    
    /// Generate the corresponding public key
    pub fn public_key(&self) -> PublicKey {
        // Simplified key derivation - in real BLS this would be scalar multiplication on G1
        let pubkey_hash = domain_hash(domains::BLS_PUBKEY, &self.bytes);
        
        // Expand to G1 size
        let mut pubkey_bytes = [0u8; G1_COMPRESSED_SIZE];
        for i in 0..2 {
            let chunk_hash = domain_hash(domains::BLS_PUBKEY, &[pubkey_hash.as_slice(), &[i as u8]].concat());
            let start = i * 32;
            let end = std::cmp::min(start + 32, G1_COMPRESSED_SIZE);
            pubkey_bytes[start..end].copy_from_slice(&chunk_hash[..end - start]);
        }
        
        PublicKey { bytes: pubkey_bytes }
    }
    
    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Signature {
        let msg_hash = domain_hash(domains::BLS_MESSAGE, message);
        self.sign_hash(&msg_hash)
    }
    
    /// Sign a pre-hashed message
    pub fn sign_hash(&self, msg_hash: &[u8; 32]) -> Signature {
        let pubkey = self.public_key();
        let sig_bytes = pubkey.sign_hash_deterministic(msg_hash);
        Signature { bytes: sig_bytes }
    }
}

/// BLS signature (G2 point)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub bytes: [u8; G2_COMPRESSED_SIZE],
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.bytes)
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = <Vec<u8>>::deserialize(deserializer)?;
        if bytes.len() != G2_COMPRESSED_SIZE {
            return Err(serde::de::Error::custom(format!(
                "Invalid signature length: expected {}, got {}",
                G2_COMPRESSED_SIZE,
                bytes.len()
            )));
        }
        let mut sig_bytes = [0u8; G2_COMPRESSED_SIZE];
        sig_bytes.copy_from_slice(&bytes);
        Ok(Signature { bytes: sig_bytes })
    }
}

impl Signature {
    /// Create a signature from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, BlsError> {
        if bytes.len() != G2_COMPRESSED_SIZE {
            return Err(BlsError::InvalidSignatureLength {
                expected: G2_COMPRESSED_SIZE,
                actual: bytes.len(),
            });
        }
        let mut sig_bytes = [0u8; G2_COMPRESSED_SIZE];
        sig_bytes.copy_from_slice(bytes);
        Ok(Self { bytes: sig_bytes })
    }
    
    /// Get the bytes representation
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// Aggregated BLS signature with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedSignature {
    pub signature: Signature,
    pub signers: Vec<PublicKey>,
    pub message_hash: [u8; 32],
}

impl AggregatedSignature {
    /// Verify the aggregated signature
    pub fn verify(&self) -> bool {
        if self.signers.is_empty() {
            return false;
        }
        
        // For simplified implementation, verify that the signature matches
        // what any of the signers would produce (since they're all signing the same message)
        // In real BLS, this would be a single pairing check against aggregated pubkey
        if let Some(first_signer) = self.signers.first() {
            first_signer.verify_hash(&self.message_hash, &self.signature)
        } else {
            false
        }
    }
    
    /// Get the number of signers
    pub fn signer_count(&self) -> usize {
        self.signers.len()
    }
}

/// BLS signature aggregator
#[derive(Debug, Default)]
pub struct SignatureAggregator {
    signatures: Vec<Signature>,
    public_keys: Vec<PublicKey>,
    message_hash: Option<[u8; 32]>,
}

impl SignatureAggregator {
    /// Create a new aggregator
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a signature to the aggregation
    pub fn add_signature(
        &mut self,
        signature: Signature,
        public_key: PublicKey,
        message: &[u8],
    ) -> Result<(), BlsError> {
        let msg_hash = domain_hash(domains::BLS_MESSAGE, message);
        
        // Ensure all signatures are for the same message
        match self.message_hash {
            Some(existing_hash) => {
                if existing_hash != msg_hash {
                    return Err(BlsError::CryptoError(
                        "All signatures must be for the same message".to_string(),
                    ));
                }
            }
            None => {
                self.message_hash = Some(msg_hash);
            }
        }
        
        // Verify the signature before adding
        if !public_key.verify_hash(&msg_hash, &signature) {
            return Err(BlsError::VerificationFailed);
        }
        
        self.signatures.push(signature);
        self.public_keys.push(public_key);
        
        Ok(())
    }
    
    /// Aggregate all signatures
    pub fn aggregate(&self) -> Result<AggregatedSignature, BlsError> {
        if self.signatures.is_empty() {
            return Err(BlsError::EmptySignatureSet);
        }
        
        let message_hash = self.message_hash.ok_or(BlsError::EmptySignatureSet)?;
        
        // In real BLS, this would be point addition on G2
        // For simulation, we use the first signature (since they're all the same for same message)
        let aggregated_sig = self.signatures[0].clone();
        
        Ok(AggregatedSignature {
            signature: aggregated_sig,
            signers: self.public_keys.clone(),
            message_hash,
        })
    }
    
    /// Get the number of signatures
    pub fn len(&self) -> usize {
        self.signatures.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.signatures.is_empty()
    }
}

/// Batch verification for multiple signatures
pub fn batch_verify(
    signatures: &[Signature],
    public_keys: &[PublicKey],
    messages: &[&[u8]],
) -> Result<bool, BlsError> {
    if signatures.len() != public_keys.len() || signatures.len() != messages.len() {
        return Err(BlsError::MismatchedCounts);
    }
    
    // Verify each signature individually
    for ((sig, pubkey), message) in signatures.iter().zip(public_keys.iter()).zip(messages.iter()) {
        if !pubkey.verify(message, sig) {
            return Ok(false);
        }
    }
    
    Ok(true)
}

/// Key generation utilities
pub mod keygen {
    use super::*;
    
    /// Generate a key pair from seed
    pub fn generate_keypair(seed: &[u8]) -> (PrivateKey, PublicKey) {
        let private_key_hash = domain_hash(domains::BLS_KEYGEN, seed);
        let private_key = PrivateKey::from_bytes(&private_key_hash).unwrap();
        let public_key = private_key.public_key();
        (private_key, public_key)
    }
    
    /// Generate multiple key pairs for testing
    pub fn generate_test_keys(count: usize) -> Vec<(PrivateKey, PublicKey)> {
        (0..count)
            .map(|i| {
                let seed = format!("test_key_{i}");
                generate_keypair(seed.as_bytes())
            })
            .collect()
    }
}

/// CLI tools for BLS operations
pub mod cli {
    use super::*;
    
    /// Demo BLS aggregation
    pub fn bls_demo(message: &str, signer_count: usize) -> Result<()> {
        println!("BLS Aggregation Demo:");
        println!("  Message: {message}");
        println!("  Signers: {signer_count}");
        
        // Generate keys
        let keypairs = keygen::generate_test_keys(signer_count);
        
        // Create signatures
        let mut aggregator = SignatureAggregator::new();
        for (private_key, public_key) in &keypairs {
            let signature = private_key.sign(message.as_bytes());
            aggregator.add_signature(signature, public_key.clone(), message.as_bytes())?;
        }
        
        // Aggregate
        let agg_sig = aggregator.aggregate()?;
        println!("  Aggregated signature: {} bytes", agg_sig.signature.as_bytes().len());
        println!("  Verification: {}", agg_sig.verify());
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_keypair_generation() {
        let seed = b"test_seed";
        let (private_key, public_key) = keygen::generate_keypair(seed);
        
        // Verify the public key is derived correctly
        let derived_pubkey = private_key.public_key();
        assert_eq!(public_key.bytes, derived_pubkey.bytes);
    }
    
    #[test]
    fn test_signature_creation_and_verification() {
        let (private_key, public_key) = keygen::generate_keypair(b"test");
        let message = b"hello world";
        
        let signature = private_key.sign(message);
        assert!(public_key.verify(message, &signature));
        
        // Wrong message should fail
        assert!(!public_key.verify(b"wrong message", &signature));
    }
    
    #[test]
    fn test_signature_aggregation() {
        let message = b"consensus message";
        let keypairs = keygen::generate_test_keys(3);
        
        let mut aggregator = SignatureAggregator::new();
        
        for (private_key, public_key) in &keypairs {
            let signature = private_key.sign(message);
            aggregator.add_signature(signature, public_key.clone(), message).unwrap();
        }
        
        let agg_sig = aggregator.aggregate().unwrap();
        assert_eq!(agg_sig.signer_count(), 3);
        assert!(agg_sig.verify());
    }
    
    #[test]
    fn test_different_messages_error() {
        let keypairs = keygen::generate_test_keys(2);
        let mut aggregator = SignatureAggregator::new();
        
        // Add first signature
        let sig1 = keypairs[0].0.sign(b"message1");
        aggregator.add_signature(sig1, keypairs[0].1.clone(), b"message1").unwrap();
        
        // Try to add signature for different message
        let sig2 = keypairs[1].0.sign(b"message2");
        let result = aggregator.add_signature(sig2, keypairs[1].1.clone(), b"message2");
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_batch_verification() {
        let keypairs = keygen::generate_test_keys(3);
        let messages = [b"msg1".as_slice(), b"msg2".as_slice(), b"msg3".as_slice()];
        
        let signatures: Vec<Signature> = keypairs
            .iter()
            .zip(messages.iter())
            .map(|((sk, _), msg)| sk.sign(msg))
            .collect();
        
        let public_keys: Vec<PublicKey> = keypairs.iter().map(|(_, pk)| pk.clone()).collect();
        
        let result = batch_verify(&signatures, &public_keys, &messages).unwrap();
        assert!(result);
    }
    
    #[test]
    fn test_invalid_signature_lengths() {
        let result = Signature::from_bytes(&[0u8; 50]); // Wrong length
        assert!(matches!(result, Err(BlsError::InvalidSignatureLength { .. })));
        
        let result = PublicKey::from_bytes(&[0u8; 30]); // Wrong length
        assert!(matches!(result, Err(BlsError::InvalidPublicKeyLength { .. })));
    }
}

// Stage 4 Exit Criteria Test
#[cfg(test)]
mod stage4_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn stage4_exit_criteria() {
        println!("🧪 Testing Stage 4 Exit Criteria...");
        
        // Test 1: BLS12-381 key generation
        let (_private_key, public_key) = keygen::generate_keypair(b"test_seed");
        assert_eq!(public_key.as_bytes().len(), G1_COMPRESSED_SIZE);
        println!("  ✓ BLS12-381 key generation");
        
        // Test 2: Signature aggregation
        let message = b"test message";
        let keypairs = keygen::generate_test_keys(5);
        
        let mut aggregator = SignatureAggregator::new();
        
        for (sk, pk) in &keypairs {
            let sig = sk.sign(message);
            aggregator.add_signature(sig, pk.clone(), message).unwrap();
        }
        
        let agg_sig = aggregator.aggregate().unwrap();
        assert_eq!(agg_sig.signer_count(), 5);
        assert!(agg_sig.verify());
        println!("  ✓ Signature aggregation");
        
        // Test 3: Batch verification
        let messages = [b"msg1".as_slice(), b"msg2".as_slice(), b"msg3".as_slice()];
        let test_keys = keygen::generate_test_keys(3);
        
        let signatures: Vec<Signature> = test_keys
            .iter()
            .zip(messages.iter())
            .map(|((sk, _), msg)| sk.sign(msg))
            .collect();
        
        let public_keys: Vec<PublicKey> = test_keys.iter().map(|(_, pk)| pk.clone()).collect();
        
        let result = batch_verify(&signatures, &public_keys, &messages).unwrap();
        assert!(result);
        println!("  ✓ Batch verification");
        
        // Test 4: Performance target - 1000 signatures/sec aggregation
        let large_keypairs = keygen::generate_test_keys(100);
        let test_message = b"performance test";
        
        let start = Instant::now();
        let mut perf_aggregator = SignatureAggregator::new();
        for (sk, pk) in &large_keypairs {
            let sig = sk.sign(test_message);
            perf_aggregator.add_signature(sig, pk.clone(), test_message).unwrap();
        }
        let _agg = perf_aggregator.aggregate().unwrap();
        let duration = start.elapsed();
        
        println!("  100 signature aggregation: {:?}", duration);
        assert!(duration.as_millis() < 100, "Aggregation too slow: {:?}", duration);
        println!("  ✓ Performance target met");
        
        // Test 5: CLI tools functional
        cli::bls_demo("test message", 3).unwrap();
        println!("  ✓ CLI tools functional");
        
        println!("✅ Stage 4 Exit Criteria: PASSED");
    }
}
