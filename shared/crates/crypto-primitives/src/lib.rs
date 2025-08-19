// Shared Cryptographic Primitives
// Basic cryptographic functions shared between BPI Core and BPCI Enterprise

//! # Crypto Primitives
//! 
//! Shared cryptographic functions for both community and enterprise products.
//! Provides consistent, secure implementations across the entire ecosystem.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use sha2::{Digest, Sha256, Sha512};
use blake3::Hasher as Blake3Hasher;
use ring::{hmac, rand as ring_rand};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;

// Type alias for clarity
type PublicKey = VerifyingKey;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Invalid key length: expected {expected}, got {actual}")]
    InvalidKeyLength { expected: usize, actual: usize },
    #[error("Signature verification failed")]
    SignatureVerificationFailed,
    #[error("Key generation failed: {0}")]
    KeyGenerationFailed(String),
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Hash computation failed: {0}")]
    HashFailed(String),
}

/// Hash algorithms supported by the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HashAlgorithm {
    Sha256,
    Sha512,
    Blake3,
}

/// Cryptographic hash result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hash {
    pub algorithm: HashAlgorithm,
    pub bytes: Vec<u8>,
}

impl Hash {
    /// Create a new hash
    pub fn new(algorithm: HashAlgorithm, bytes: Vec<u8>) -> Self {
        Hash { algorithm, bytes }
    }

    /// Get hash as hex string
    pub fn to_hex(&self) -> String {
        hex::encode(&self.bytes)
    }

    /// Create hash from hex string
    pub fn from_hex(algorithm: HashAlgorithm, hex_str: &str) -> Result<Self, CryptoError> {
        let bytes = hex::decode(hex_str)
            .map_err(|e| CryptoError::HashFailed(format!("Invalid hex: {}", e)))?;
        Ok(Hash::new(algorithm, bytes))
    }
}

/// Hash data using specified algorithm
pub fn hash_data(data: &[u8], algorithm: HashAlgorithm) -> Result<Hash, CryptoError> {
    let bytes = match algorithm {
        HashAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        }
        HashAlgorithm::Sha512 => {
            let mut hasher = Sha512::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        }
        HashAlgorithm::Blake3 => {
            let mut hasher = Blake3Hasher::new();
            hasher.update(data);
            hasher.finalize().as_bytes().to_vec()
        }
    };
    
    Ok(Hash::new(algorithm, bytes))
}

/// HMAC key for message authentication
#[derive(Debug, Clone)]
pub struct HmacKey {
    key: hmac::Key,
}

impl HmacKey {
    /// Generate a new HMAC key
    pub fn generate() -> Result<Self, CryptoError> {
        let rng = ring_rand::SystemRandom::new();
        let key = hmac::Key::generate(hmac::HMAC_SHA256, &rng)
            .map_err(|e| CryptoError::KeyGenerationFailed(format!("HMAC key generation failed: {:?}", e)))?;
        Ok(HmacKey { key })
    }

    /// Create HMAC key from bytes
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let key = hmac::Key::new(hmac::HMAC_SHA256, bytes);
        HmacKey { key }
    }

    /// Sign data with HMAC
    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        hmac::sign(&self.key, data).as_ref().to_vec()
    }

    /// Verify HMAC signature
    pub fn verify(&self, data: &[u8], signature: &[u8]) -> bool {
        hmac::verify(&self.key, data, signature).is_ok()
    }
}

/// Ed25519 key pair for digital signatures
#[derive(Debug, Clone)]
pub struct Ed25519KeyPair {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl Ed25519KeyPair {
    /// Generate a new Ed25519 key pair
    pub fn generate() -> Self {
        let mut csprng = rand::thread_rng();
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        
        Ed25519KeyPair { signing_key, verifying_key }
    }

    /// Get public key
    pub fn public_key(&self) -> &VerifyingKey {
        &self.verifying_key
    }

    /// Get public key bytes
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.verifying_key.to_bytes()
    }

    /// Sign data
    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        self.signing_key.sign(data).to_bytes().to_vec()
    }

    /// Verify signature
    pub fn verify(&self, data: &[u8], signature: &Signature) -> Result<(), CryptoError> {
        self.verifying_key.verify(data, signature)
            .map_err(|_| CryptoError::SignatureVerificationFailed)
    }
}

/// Ed25519 public key for signature verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ed25519PublicKey {
    public_key_bytes: [u8; 32],
}

impl Ed25519PublicKey {
    /// Create from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        if bytes.len() != 32 {
            return Err(CryptoError::InvalidKeyLength {
                expected: 32,
                actual: bytes.len(),
            });
        }

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(bytes);
        
        // Validate that the bytes form a valid public key
        VerifyingKey::from_bytes(&key_bytes)
            .map_err(|_| CryptoError::InvalidKeyLength {
                expected: 32,
                actual: bytes.len(),
            })?;

        Ok(Self { public_key_bytes: key_bytes })
    }

    /// Get bytes
    pub fn to_bytes(&self) -> [u8; 32] {
        self.public_key_bytes
    }
    
    /// Get the VerifyingKey for cryptographic operations
    pub fn verifying_key(&self) -> Result<VerifyingKey, CryptoError> {
        VerifyingKey::from_bytes(&self.public_key_bytes)
            .map_err(|_| CryptoError::InvalidKeyLength {
                expected: 32,
                actual: 32,
            })
    }
}

/// Secure random number generation
pub struct SecureRandom {
    rng: ring_rand::SystemRandom,
}

impl SecureRandom {
    /// Create new secure random generator
    pub fn new() -> Self {
        SecureRandom {
            rng: ring_rand::SystemRandom::new(),
        }
    }

    /// Fill buffer with random bytes
    pub fn fill(&self, dest: &mut [u8]) -> Result<(), CryptoError> {
        ring_rand::SecureRandom::fill(&self.rng, dest)
            .map_err(|e| CryptoError::KeyGenerationFailed(format!("Random generation failed: {:?}", e)))
    }

    /// Generate random bytes
    pub fn generate_bytes(&self, len: usize) -> Result<Vec<u8>, CryptoError> {
        let mut bytes = vec![0u8; len];
        self.fill(&mut bytes)?;
        Ok(bytes)
    }
}

impl Default for SecureRandom {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_algorithms() {
        let data = b"test data";
        
        let sha256_hash = hash_data(data, HashAlgorithm::Sha256).unwrap();
        assert_eq!(sha256_hash.bytes.len(), 32);
        
        let sha512_hash = hash_data(data, HashAlgorithm::Sha512).unwrap();
        assert_eq!(sha512_hash.bytes.len(), 64);
        
        let blake3_hash = hash_data(data, HashAlgorithm::Blake3).unwrap();
        assert_eq!(blake3_hash.bytes.len(), 32);
    }

    #[test]
    fn test_hmac() {
        let key = HmacKey::generate().unwrap();
        let data = b"test message";
        
        let signature = key.sign(data);
        assert!(key.verify(data, &signature));
        assert!(!key.verify(b"different data", &signature));
    }

    #[test]
    fn test_ed25519() {
        let keypair = Ed25519KeyPair::generate();
        let data = b"test message";
        
        let signature = keypair.sign(data);
        keypair.verify(data, &Signature::from_slice(&signature).unwrap()).unwrap();
        
        let public_key = Ed25519PublicKey::from_bytes(&keypair.public_key_bytes()).unwrap();
        let verifying_key = public_key.verifying_key().unwrap();
        verifying_key.verify(data, &Signature::from_slice(&signature).unwrap()).unwrap();
    }

    #[test]
    fn test_secure_random() {
        let rng = SecureRandom::new();
        let bytes1 = rng.generate_bytes(32).unwrap();
        let bytes2 = rng.generate_bytes(32).unwrap();
        
        assert_eq!(bytes1.len(), 32);
        assert_eq!(bytes2.len(), 32);
        assert_ne!(bytes1, bytes2);
    }
}
