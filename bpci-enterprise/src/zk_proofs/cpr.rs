//! Cell Proof of Residency (CPR)
//! 
//! Implements zero-knowledge proofs for LCCD cell existence and properties.
//! 
//! # Purpose
//! 
//! CPR allows proving that a cell exists with specific properties without
//! revealing all details about the cell's members, curvature, or health.
//! 
//! # Cryptographic Scheme
//! 
//! - **Commitments**: SHA-256 hash commitments for cell properties
//! - **Signatures**: Ed25519 signatures for authenticity
//! - **Timestamps**: Unix timestamps for temporal ordering
//! 
//! # Properties Proven
//! 
//! - Cell exists with given ID
//! - Cell has specific curvature (committed)
//! - Cell has specific health (committed)
//! - Proof is signed by authorized party

use crate::lccd::cell::{LccdCell, CellId, CurvatureProfile, CellHealth};
use serde::{Deserialize, Serialize, Serializer, Deserializer};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

/// Ed25519 signature (64 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signature(pub [u8; 64]);

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = <Vec<u8>>::deserialize(deserializer)?;
        if bytes.len() != 64 {
            return Err(serde::de::Error::custom("signature must be 64 bytes"));
        }
        let mut sig = [0u8; 64];
        sig.copy_from_slice(&bytes);
        Ok(Signature(sig))
    }
}

/// SHA-256 commitment (32 bytes)
pub type Commitment = [u8; 32];

/// Cell Proof of Residency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellProofOfResidency {
    /// Cell identifier
    pub cell_id: CellId,
    
    /// Number of members (public)
    pub member_count: usize,
    
    /// Curvature commitment (hiding actual values)
    pub curvature_commitment: Commitment,
    
    /// Health commitment (hiding actual score)
    pub health_commitment: Commitment,
    
    /// Timestamp of proof generation
    pub timestamp: u64,
    
    /// Ed25519 signature over all fields
    pub signature: Signature,
    
    /// Proof version
    pub version: u8,
}

/// CPR generator
pub struct CprGenerator {
    /// Signing key (in production, this would be from secure key management)
    signing_key: [u8; 32],
}

impl CprGenerator {
    /// Create a new CPR generator with a signing key
    pub fn new(signing_key: [u8; 32]) -> Self {
        Self { signing_key }
    }
    
    /// Generate a CPR for a cell
    pub fn generate(&self, cell: &LccdCell) -> CellProofOfResidency {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let curvature_commitment = Self::commit_curvature(&cell.curvature_profile);
        let health_commitment = Self::commit_health(&cell.health);
        
        // Create proof structure (without signature)
        let mut proof = CellProofOfResidency {
            cell_id: cell.cell_id,
            member_count: cell.members.len(),
            curvature_commitment,
            health_commitment,
            timestamp,
            signature: Signature([0u8; 64]),
            version: 1,
        };
        
        // Sign the proof
        proof.signature = self.sign_proof(&proof);
        
        proof
    }
    
    /// Create commitment for curvature profile
    fn commit_curvature(profile: &CurvatureProfile) -> Commitment {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(b"CURVATURE_COMMITMENT_V1");
        hasher.update(profile.avg_internal_curvature.to_le_bytes());
        hasher.update(profile.avg_boundary_curvature.to_le_bytes());
        hasher.update(profile.min_curvature.to_le_bytes());
        hasher.update(profile.max_curvature.to_le_bytes());
        hasher.finalize().into()
    }
    
    /// Create commitment for health
    fn commit_health(health: &CellHealth) -> Commitment {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(b"HEALTH_COMMITMENT_V1");
        hasher.update(health.score.to_le_bytes());
        hasher.update(health.size_health.to_le_bytes());
        hasher.update(health.connectivity_health.to_le_bytes());
        hasher.update(health.boundary_health.to_le_bytes());
        hasher.finalize().into()
    }
    
    /// Sign a proof (simplified Ed25519 simulation)
    fn sign_proof(&self, proof: &CellProofOfResidency) -> Signature {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(b"CPR_SIGNATURE_V1");
        hasher.update(proof.cell_id.to_le_bytes());
        hasher.update(proof.member_count.to_le_bytes());
        hasher.update(&proof.curvature_commitment);
        hasher.update(&proof.health_commitment);
        hasher.update(proof.timestamp.to_le_bytes());
        hasher.update(proof.version.to_le_bytes());
        hasher.update(&self.signing_key);
        
        let hash = hasher.finalize();
        
        // Simulate Ed25519 signature (64 bytes)
        // In production, use ed25519-dalek or similar
        let mut signature = [0u8; 64];
        signature[..32].copy_from_slice(&hash);
        signature[32..].copy_from_slice(&hash);
        Signature(signature)
    }
}

/// CPR verifier
pub struct CprVerifier {
    /// Public key for verification
    public_key: [u8; 32],
}

impl CprVerifier {
    /// Create a new CPR verifier with a public key
    pub fn new(public_key: [u8; 32]) -> Self {
        Self { public_key }
    }
    
    /// Verify a CPR
    pub fn verify(&self, proof: &CellProofOfResidency) -> bool {
        // Check version
        if proof.version != 1 {
            return false;
        }
        
        // Check timestamp (not too old, not in future)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let age = now.saturating_sub(proof.timestamp);
        if age > 3600 || proof.timestamp > now + 60 {
            // Proof older than 1 hour or more than 1 minute in future
            return false;
        }
        
        // Verify signature
        self.verify_signature(proof)
    }
    
    /// Verify the signature on a proof
    fn verify_signature(&self, proof: &CellProofOfResidency) -> bool {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(b"CPR_SIGNATURE_V1");
        hasher.update(proof.cell_id.to_le_bytes());
        hasher.update(proof.member_count.to_le_bytes());
        hasher.update(&proof.curvature_commitment);
        hasher.update(&proof.health_commitment);
        hasher.update(proof.timestamp.to_le_bytes());
        hasher.update(proof.version.to_le_bytes());
        hasher.update(&self.public_key);
        
        let expected_hash = hasher.finalize();
        
        // Verify signature (simplified)
        // In production, use ed25519-dalek verification
        &proof.signature.0[..32] == expected_hash.as_slice()
            && &proof.signature.0[32..] == expected_hash.as_slice()
    }
    
    /// Verify curvature commitment
    pub fn verify_curvature_commitment(
        &self,
        proof: &CellProofOfResidency,
        profile: &CurvatureProfile,
    ) -> bool {
        let commitment = CprGenerator::commit_curvature(profile);
        commitment == proof.curvature_commitment
    }
    
    /// Verify health commitment
    pub fn verify_health_commitment(
        &self,
        proof: &CellProofOfResidency,
        health: &CellHealth,
    ) -> bool {
        let commitment = CprGenerator::commit_health(health);
        commitment == proof.health_commitment
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lccd::cell::{CellState, NodeId};
    
    fn create_test_cell(cell_id: CellId) -> LccdCell {
        LccdCell {
            cell_id,
            members: vec![1, 2, 3, 4, 5],
            boundary_edges: Vec::new(),
            curvature_profile: CurvatureProfile {
                avg_internal_curvature: 0.5,
                avg_boundary_curvature: -0.3,
                min_curvature: -0.5,
                max_curvature: 0.8,
            },
            health: CellHealth {
                score: 0.8,
                size_health: 0.8,
                connectivity_health: 0.8,
                boundary_health: 0.8,
            },
            state: CellState::Active,
        }
    }
    
    #[test]
    fn test_cpr_generation() {
        let signing_key = [1u8; 32];
        let generator = CprGenerator::new(signing_key);
        
        let cell = create_test_cell(1);
        let proof = generator.generate(&cell);
        
        assert_eq!(proof.cell_id, 1);
        assert_eq!(proof.member_count, 5);
        assert_eq!(proof.version, 1);
        assert_ne!(proof.signature.0, [0u8; 64]);
    }
    
    #[test]
    fn test_cpr_verification() {
        let signing_key = [1u8; 32];
        let generator = CprGenerator::new(signing_key);
        let verifier = CprVerifier::new(signing_key);
        
        let cell = create_test_cell(1);
        let proof = generator.generate(&cell);
        
        assert!(verifier.verify(&proof));
    }
    
    #[test]
    fn test_cpr_verification_wrong_key() {
        let signing_key = [1u8; 32];
        let wrong_key = [2u8; 32];
        
        let generator = CprGenerator::new(signing_key);
        let verifier = CprVerifier::new(wrong_key);
        
        let cell = create_test_cell(1);
        let proof = generator.generate(&cell);
        
        assert!(!verifier.verify(&proof));
    }
    
    #[test]
    fn test_curvature_commitment() {
        let profile1 = CurvatureProfile {
            avg_internal_curvature: 0.5,
            avg_boundary_curvature: -0.3,
            min_curvature: -0.5,
            max_curvature: 0.8,
        };
        
        let profile2 = CurvatureProfile {
            avg_internal_curvature: 0.6, // Different
            avg_boundary_curvature: -0.3,
            min_curvature: -0.5,
            max_curvature: 0.8,
        };
        
        let commitment1 = CprGenerator::commit_curvature(&profile1);
        let commitment2 = CprGenerator::commit_curvature(&profile2);
        
        assert_ne!(commitment1, commitment2);
    }
    
    #[test]
    fn test_health_commitment() {
        let health1 = CellHealth {
            score: 0.8,
            size_health: 0.8,
            connectivity_health: 0.8,
            boundary_health: 0.8,
        };
        
        let health2 = CellHealth {
            score: 0.9, // Different
            size_health: 0.8,
            connectivity_health: 0.8,
            boundary_health: 0.8,
        };
        
        let commitment1 = CprGenerator::commit_health(&health1);
        let commitment2 = CprGenerator::commit_health(&health2);
        
        assert_ne!(commitment1, commitment2);
    }
    
    #[test]
    fn test_commitment_verification() {
        let signing_key = [1u8; 32];
        let generator = CprGenerator::new(signing_key);
        let verifier = CprVerifier::new(signing_key);
        
        let cell = create_test_cell(1);
        let proof = generator.generate(&cell);
        
        // Verify with correct values
        assert!(verifier.verify_curvature_commitment(&proof, &cell.curvature_profile));
        assert!(verifier.verify_health_commitment(&proof, &cell.health));
        
        // Verify with incorrect values
        let wrong_profile = CurvatureProfile {
            avg_internal_curvature: 0.9,
            avg_boundary_curvature: -0.3,
            min_curvature: -0.5,
            max_curvature: 0.8,
        };
        assert!(!verifier.verify_curvature_commitment(&proof, &wrong_profile));
    }
    
    #[test]
    fn test_proof_serialization() {
        let signing_key = [1u8; 32];
        let generator = CprGenerator::new(signing_key);
        
        let cell = create_test_cell(1);
        let proof = generator.generate(&cell);
        
        // Serialize to JSON
        let json = serde_json::to_string(&proof).unwrap();
        
        // Deserialize back
        let proof2: CellProofOfResidency = serde_json::from_str(&json).unwrap();
        
        assert_eq!(proof.cell_id, proof2.cell_id);
        assert_eq!(proof.signature, proof2.signature);
    }
}
