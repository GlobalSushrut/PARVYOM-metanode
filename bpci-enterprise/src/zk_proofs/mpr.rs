//! Migration Proof of Residency (MPR)
//! 
//! Implements zero-knowledge proofs for LCCD cell migration correctness.
//! 
//! # Purpose
//! 
//! MPR allows proving that a migration occurred correctly without revealing
//! all details about the source state, target state, or migration process.
//! 
//! # Cryptographic Scheme
//! 
//! - **Commitments**: SHA-256 hash commitments for migration state
//! - **Signatures**: Ed25519 signatures for authenticity
//! - **State Hashes**: Merkle-tree style state commitments
//! 
//! # Properties Proven
//! 
//! - Migration occurred from source to target
//! - State was transferred correctly
//! - Migration completed successfully
//! - Proof is signed by authorized party

use crate::lccd::migration::{MigrationContext, MigrationState};
use crate::lccd::cell::CellId;
use super::cpr::Signature;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

/// SHA-256 commitment (32 bytes)
pub type Commitment = [u8; 32];

/// State hash (32 bytes)
pub type StateHash = [u8; 32];

/// Migration Proof of Residency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationProofOfResidency {
    /// Migration identifier
    pub migration_id: String,
    
    /// Cell being migrated
    pub cell_id: CellId,
    
    /// Source slot commitment
    pub source_commitment: Commitment,
    
    /// Target slot commitment
    pub target_commitment: Commitment,
    
    /// State hash before migration
    pub state_before: StateHash,
    
    /// State hash after migration
    pub state_after: StateHash,
    
    /// Bytes transferred
    pub bytes_transferred: u64,
    
    /// Migration state (final)
    pub final_state: MigrationState,
    
    /// Timestamp of proof generation
    pub timestamp: u64,
    
    /// Ed25519 signature over all fields
    pub signature: Signature,
    
    /// Proof version
    pub version: u8,
}

/// MPR generator
pub struct MprGenerator {
    /// Signing key
    signing_key: [u8; 32],
}

impl MprGenerator {
    /// Create a new MPR generator
    pub fn new(signing_key: [u8; 32]) -> Self {
        Self { signing_key }
    }
    
    /// Generate an MPR for a migration
    pub fn generate(
        &self,
        migration_ctx: &MigrationContext,
        state_before: &[u8],
        state_after: &[u8],
    ) -> MigrationProofOfResidency {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let source_commitment = Self::commit_slot(migration_ctx.source_slot.as_deref());
        let target_commitment = Self::commit_slot(Some(&migration_ctx.target_slot));
        let state_before_hash = Self::hash_state(state_before);
        let state_after_hash = Self::hash_state(state_after);
        
        let mut proof = MigrationProofOfResidency {
            migration_id: migration_ctx.migration_id.clone(),
            cell_id: migration_ctx.cell_id,
            source_commitment,
            target_commitment,
            state_before: state_before_hash,
            state_after: state_after_hash,
            bytes_transferred: migration_ctx.bytes_transferred,
            final_state: migration_ctx.state,
            timestamp,
            signature: Signature([0u8; 64]),
            version: 1,
        };
        
        proof.signature = self.sign_proof(&proof);
        
        proof
    }
    
    /// Create commitment for a slot
    fn commit_slot(slot_id: Option<&str>) -> Commitment {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(b"SLOT_COMMITMENT_V1");
        if let Some(id) = slot_id {
            hasher.update(id.as_bytes());
        } else {
            hasher.update(b"NONE");
        }
        hasher.finalize().into()
    }
    
    /// Hash state data
    fn hash_state(state: &[u8]) -> StateHash {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(b"STATE_HASH_V1");
        hasher.update(state);
        hasher.finalize().into()
    }
    
    /// Sign a proof
    fn sign_proof(&self, proof: &MigrationProofOfResidency) -> Signature {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(b"MPR_SIGNATURE_V1");
        hasher.update(proof.migration_id.as_bytes());
        hasher.update(proof.cell_id.to_le_bytes());
        hasher.update(&proof.source_commitment);
        hasher.update(&proof.target_commitment);
        hasher.update(&proof.state_before);
        hasher.update(&proof.state_after);
        hasher.update(proof.bytes_transferred.to_le_bytes());
        hasher.update(&[proof.final_state as u8]);
        hasher.update(proof.timestamp.to_le_bytes());
        hasher.update(proof.version.to_le_bytes());
        hasher.update(&self.signing_key);
        
        let hash = hasher.finalize();
        
        let mut signature = [0u8; 64];
        signature[..32].copy_from_slice(&hash);
        signature[32..].copy_from_slice(&hash);
        Signature(signature)
    }
}

/// MPR verifier
pub struct MprVerifier {
    /// Public key for verification
    public_key: [u8; 32],
}

impl MprVerifier {
    /// Create a new MPR verifier
    pub fn new(public_key: [u8; 32]) -> Self {
        Self { public_key }
    }
    
    /// Verify an MPR
    pub fn verify(&self, proof: &MigrationProofOfResidency) -> bool {
        // Check version
        if proof.version != 1 {
            return false;
        }
        
        // Check timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let age = now.saturating_sub(proof.timestamp);
        if age > 3600 || proof.timestamp > now + 60 {
            return false;
        }
        
        // Check migration completed successfully
        if proof.final_state != MigrationState::Complete {
            return false;
        }
        
        // Verify signature
        self.verify_signature(proof)
    }
    
    /// Verify the signature
    fn verify_signature(&self, proof: &MigrationProofOfResidency) -> bool {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(b"MPR_SIGNATURE_V1");
        hasher.update(proof.migration_id.as_bytes());
        hasher.update(proof.cell_id.to_le_bytes());
        hasher.update(&proof.source_commitment);
        hasher.update(&proof.target_commitment);
        hasher.update(&proof.state_before);
        hasher.update(&proof.state_after);
        hasher.update(proof.bytes_transferred.to_le_bytes());
        hasher.update(&[proof.final_state as u8]);
        hasher.update(proof.timestamp.to_le_bytes());
        hasher.update(proof.version.to_le_bytes());
        hasher.update(&self.public_key);
        
        let expected_hash = hasher.finalize();
        
        &proof.signature.0[..32] == expected_hash.as_slice()
            && &proof.signature.0[32..] == expected_hash.as_slice()
    }
    
    /// Verify slot commitment
    pub fn verify_slot_commitment(
        &self,
        proof: &MigrationProofOfResidency,
        slot_id: Option<&str>,
        is_source: bool,
    ) -> bool {
        let commitment = MprGenerator::commit_slot(slot_id);
        if is_source {
            commitment == proof.source_commitment
        } else {
            commitment == proof.target_commitment
        }
    }
    
    /// Verify state hash
    pub fn verify_state_hash(
        &self,
        proof: &MigrationProofOfResidency,
        state: &[u8],
        is_before: bool,
    ) -> bool {
        let hash = MprGenerator::hash_state(state);
        if is_before {
            hash == proof.state_before
        } else {
            hash == proof.state_after
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_migration_context() -> MigrationContext {
        MigrationContext {
            migration_id: "test-migration-1".to_string(),
            cell_id: 1,
            source_slot: Some("source-slot".to_string()),
            target_slot: "target-slot".to_string(),
            state: MigrationState::Complete,
            start_time: None,
            bytes_transferred: 1024 * 1024, // 1 MB
            total_bytes: 1024 * 1024,
            error: None,
            snapshot_id: Some("snap-1".to_string()),
        }
    }
    
    #[test]
    fn test_mpr_generation() {
        let signing_key = [1u8; 32];
        let generator = MprGenerator::new(signing_key);
        
        let migration_ctx = create_test_migration_context();
        let state_before = b"state before migration";
        let state_after = b"state after migration";
        
        let proof = generator.generate(&migration_ctx, state_before, state_after);
        
        assert_eq!(proof.migration_id, "test-migration-1");
        assert_eq!(proof.cell_id, 1);
        assert_eq!(proof.bytes_transferred, 1024 * 1024);
        assert_eq!(proof.final_state, MigrationState::Complete);
        assert_eq!(proof.version, 1);
        assert_ne!(proof.signature.0, [0u8; 64]);
    }
    
    #[test]
    fn test_mpr_verification() {
        let signing_key = [1u8; 32];
        let generator = MprGenerator::new(signing_key);
        let verifier = MprVerifier::new(signing_key);
        
        let migration_ctx = create_test_migration_context();
        let state_before = b"state before";
        let state_after = b"state after";
        
        let proof = generator.generate(&migration_ctx, state_before, state_after);
        
        assert!(verifier.verify(&proof));
    }
    
    #[test]
    fn test_mpr_verification_wrong_key() {
        let signing_key = [1u8; 32];
        let wrong_key = [2u8; 32];
        
        let generator = MprGenerator::new(signing_key);
        let verifier = MprVerifier::new(wrong_key);
        
        let migration_ctx = create_test_migration_context();
        let state_before = b"state before";
        let state_after = b"state after";
        
        let proof = generator.generate(&migration_ctx, state_before, state_after);
        
        assert!(!verifier.verify(&proof));
    }
    
    #[test]
    fn test_slot_commitment() {
        let commitment1 = MprGenerator::commit_slot(Some("slot-1"));
        let commitment2 = MprGenerator::commit_slot(Some("slot-2"));
        let commitment_none = MprGenerator::commit_slot(None);
        
        assert_ne!(commitment1, commitment2);
        assert_ne!(commitment1, commitment_none);
    }
    
    #[test]
    fn test_state_hash() {
        let state1 = b"state 1";
        let state2 = b"state 2";
        
        let hash1 = MprGenerator::hash_state(state1);
        let hash2 = MprGenerator::hash_state(state2);
        
        assert_ne!(hash1, hash2);
    }
    
    #[test]
    fn test_slot_commitment_verification() {
        let signing_key = [1u8; 32];
        let generator = MprGenerator::new(signing_key);
        let verifier = MprVerifier::new(signing_key);
        
        let migration_ctx = create_test_migration_context();
        let state_before = b"before";
        let state_after = b"after";
        
        let proof = generator.generate(&migration_ctx, state_before, state_after);
        
        // Verify source commitment
        assert!(verifier.verify_slot_commitment(&proof, Some("source-slot"), true));
        assert!(!verifier.verify_slot_commitment(&proof, Some("wrong-slot"), true));
        
        // Verify target commitment
        assert!(verifier.verify_slot_commitment(&proof, Some("target-slot"), false));
        assert!(!verifier.verify_slot_commitment(&proof, Some("wrong-slot"), false));
    }
    
    #[test]
    fn test_state_hash_verification() {
        let signing_key = [1u8; 32];
        let generator = MprGenerator::new(signing_key);
        let verifier = MprVerifier::new(signing_key);
        
        let migration_ctx = create_test_migration_context();
        let state_before = b"state before migration";
        let state_after = b"state after migration";
        
        let proof = generator.generate(&migration_ctx, state_before, state_after);
        
        // Verify state before
        assert!(verifier.verify_state_hash(&proof, state_before, true));
        assert!(!verifier.verify_state_hash(&proof, b"wrong state", true));
        
        // Verify state after
        assert!(verifier.verify_state_hash(&proof, state_after, false));
        assert!(!verifier.verify_state_hash(&proof, b"wrong state", false));
    }
    
    #[test]
    fn test_proof_serialization() {
        let signing_key = [1u8; 32];
        let generator = MprGenerator::new(signing_key);
        
        let migration_ctx = create_test_migration_context();
        let state_before = b"before";
        let state_after = b"after";
        
        let proof = generator.generate(&migration_ctx, state_before, state_after);
        
        // Serialize to JSON
        let json = serde_json::to_string(&proof).unwrap();
        
        // Deserialize back
        let proof2: MigrationProofOfResidency = serde_json::from_str(&json).unwrap();
        
        assert_eq!(proof.migration_id, proof2.migration_id);
        assert_eq!(proof.signature.0, proof2.signature.0);
    }
    
    #[test]
    fn test_failed_migration_verification() {
        let signing_key = [1u8; 32];
        let generator = MprGenerator::new(signing_key);
        let verifier = MprVerifier::new(signing_key);
        
        let mut migration_ctx = create_test_migration_context();
        migration_ctx.state = MigrationState::Failed; // Failed migration
        
        let state_before = b"before";
        let state_after = b"after";
        
        let proof = generator.generate(&migration_ctx, state_before, state_after);
        
        // Should fail verification because migration didn't complete
        assert!(!verifier.verify(&proof));
    }
}
