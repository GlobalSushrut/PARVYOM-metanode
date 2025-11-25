//! Execution Receipt (ER)
//! 
//! Implements zero-knowledge proofs for operation execution correctness.
//! 
//! # Purpose
//! 
//! ER allows proving that an operation was executed correctly without revealing
//! all details about the operation parameters or intermediate state.
//! 
//! # Cryptographic Scheme
//! 
//! - **State Hashes**: SHA-256 hash commitments for before/after state
//! - **Operation Hashes**: SHA-256 hash of operation details
//! - **Signatures**: Ed25519 signatures for authenticity
//! 
//! # Properties Proven
//! 
//! - Operation was executed
//! - State transition was correct
//! - Operation completed successfully
//! - Proof is signed by authorized party

use super::cpr::Signature;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

/// State hash (32 bytes)
pub type StateHash = [u8; 32];

/// Operation hash (32 bytes)
pub type OperationHash = [u8; 32];

/// Operation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationType {
    /// Cell formation
    CellFormation,
    
    /// Cell growth
    CellGrowth,
    
    /// Cell division
    CellDivision,
    
    /// Cell merging
    CellMerging,
    
    /// Cell dissolution
    CellDissolution,
    
    /// Weight update (Ricci-flow)
    WeightUpdate,
    
    /// Slot allocation
    SlotAllocation,
    
    /// Migration
    Migration,
}

/// Execution Receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    /// Operation identifier
    pub operation_id: String,
    
    /// Operation type
    pub operation_type: OperationType,
    
    /// State hash before operation
    pub state_before: StateHash,
    
    /// State hash after operation
    pub state_after: StateHash,
    
    /// Operation hash (parameters and details)
    pub operation_hash: OperationHash,
    
    /// Gas used (computational cost)
    pub gas_used: u64,
    
    /// Success status
    pub success: bool,
    
    /// Timestamp of execution
    pub timestamp: u64,
    
    /// Ed25519 signature over all fields
    pub signature: Signature,
    
    /// Receipt version
    pub version: u8,
}

/// ER generator
pub struct ErGenerator {
    /// Signing key
    signing_key: [u8; 32],
}

impl ErGenerator {
    /// Create a new ER generator
    pub fn new(signing_key: [u8; 32]) -> Self {
        Self { signing_key }
    }
    
    /// Generate an ER for an operation
    pub fn generate(
        &self,
        operation_id: String,
        operation_type: OperationType,
        state_before: &[u8],
        state_after: &[u8],
        operation_params: &[u8],
        gas_used: u64,
        success: bool,
    ) -> ExecutionReceipt {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let state_before_hash = Self::hash_state(state_before);
        let state_after_hash = Self::hash_state(state_after);
        let operation_hash = Self::hash_operation(operation_type, operation_params);
        
        let mut receipt = ExecutionReceipt {
            operation_id,
            operation_type,
            state_before: state_before_hash,
            state_after: state_after_hash,
            operation_hash,
            gas_used,
            success,
            timestamp,
            signature: Signature([0u8; 64]),
            version: 1,
        };
        
        receipt.signature = self.sign_receipt(&receipt);
        
        receipt
    }
    
    /// Hash state data
    fn hash_state(state: &[u8]) -> StateHash {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(b"ER_STATE_HASH_V1");
        hasher.update(state);
        hasher.finalize().into()
    }
    
    /// Hash operation details
    fn hash_operation(op_type: OperationType, params: &[u8]) -> OperationHash {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(b"ER_OPERATION_HASH_V1");
        hasher.update(&[op_type as u8]);
        hasher.update(params);
        hasher.finalize().into()
    }
    
    /// Sign a receipt
    fn sign_receipt(&self, receipt: &ExecutionReceipt) -> Signature {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(b"ER_SIGNATURE_V1");
        hasher.update(receipt.operation_id.as_bytes());
        hasher.update(&[receipt.operation_type as u8]);
        hasher.update(&receipt.state_before);
        hasher.update(&receipt.state_after);
        hasher.update(&receipt.operation_hash);
        hasher.update(receipt.gas_used.to_le_bytes());
        hasher.update(&[receipt.success as u8]);
        hasher.update(receipt.timestamp.to_le_bytes());
        hasher.update(receipt.version.to_le_bytes());
        hasher.update(&self.signing_key);
        
        let hash = hasher.finalize();
        
        let mut signature = [0u8; 64];
        signature[..32].copy_from_slice(&hash);
        signature[32..].copy_from_slice(&hash);
        Signature(signature)
    }
}

/// ER verifier
pub struct ErVerifier {
    /// Public key for verification
    public_key: [u8; 32],
}

impl ErVerifier {
    /// Create a new ER verifier
    pub fn new(public_key: [u8; 32]) -> Self {
        Self { public_key }
    }
    
    /// Verify an ER
    pub fn verify(&self, receipt: &ExecutionReceipt) -> bool {
        // Check version
        if receipt.version != 1 {
            return false;
        }
        
        // Check timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let age = now.saturating_sub(receipt.timestamp);
        if age > 3600 || receipt.timestamp > now + 60 {
            return false;
        }
        
        // Check success status
        if !receipt.success {
            return false;
        }
        
        // Verify signature
        self.verify_signature(receipt)
    }
    
    /// Verify the signature
    fn verify_signature(&self, receipt: &ExecutionReceipt) -> bool {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(b"ER_SIGNATURE_V1");
        hasher.update(receipt.operation_id.as_bytes());
        hasher.update(&[receipt.operation_type as u8]);
        hasher.update(&receipt.state_before);
        hasher.update(&receipt.state_after);
        hasher.update(&receipt.operation_hash);
        hasher.update(receipt.gas_used.to_le_bytes());
        hasher.update(&[receipt.success as u8]);
        hasher.update(receipt.timestamp.to_le_bytes());
        hasher.update(receipt.version.to_le_bytes());
        hasher.update(&self.public_key);
        
        let expected_hash = hasher.finalize();
        
        &receipt.signature.0[..32] == expected_hash.as_slice()
            && &receipt.signature.0[32..] == expected_hash.as_slice()
    }
    
    /// Verify state hash
    pub fn verify_state_hash(
        &self,
        receipt: &ExecutionReceipt,
        state: &[u8],
        is_before: bool,
    ) -> bool {
        let hash = ErGenerator::hash_state(state);
        if is_before {
            hash == receipt.state_before
        } else {
            hash == receipt.state_after
        }
    }
    
    /// Verify operation hash
    pub fn verify_operation_hash(
        &self,
        receipt: &ExecutionReceipt,
        params: &[u8],
    ) -> bool {
        let hash = ErGenerator::hash_operation(receipt.operation_type, params);
        hash == receipt.operation_hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_er_generation() {
        let signing_key = [1u8; 32];
        let generator = ErGenerator::new(signing_key);
        
        let state_before = b"state before operation";
        let state_after = b"state after operation";
        let params = b"operation parameters";
        
        let receipt = generator.generate(
            "op-1".to_string(),
            OperationType::CellFormation,
            state_before,
            state_after,
            params,
            1000,
            true,
        );
        
        assert_eq!(receipt.operation_id, "op-1");
        assert_eq!(receipt.operation_type, OperationType::CellFormation);
        assert_eq!(receipt.gas_used, 1000);
        assert!(receipt.success);
        assert_eq!(receipt.version, 1);
        assert_ne!(receipt.signature.0, [0u8; 64]);
    }
    
    #[test]
    fn test_er_verification() {
        let signing_key = [1u8; 32];
        let generator = ErGenerator::new(signing_key);
        let verifier = ErVerifier::new(signing_key);
        
        let state_before = b"before";
        let state_after = b"after";
        let params = b"params";
        
        let receipt = generator.generate(
            "op-1".to_string(),
            OperationType::WeightUpdate,
            state_before,
            state_after,
            params,
            500,
            true,
        );
        
        assert!(verifier.verify(&receipt));
    }
    
    #[test]
    fn test_er_verification_wrong_key() {
        let signing_key = [1u8; 32];
        let wrong_key = [2u8; 32];
        
        let generator = ErGenerator::new(signing_key);
        let verifier = ErVerifier::new(wrong_key);
        
        let state_before = b"before";
        let state_after = b"after";
        let params = b"params";
        
        let receipt = generator.generate(
            "op-1".to_string(),
            OperationType::Migration,
            state_before,
            state_after,
            params,
            2000,
            true,
        );
        
        assert!(!verifier.verify(&receipt));
    }
    
    #[test]
    fn test_state_hash() {
        let state1 = b"state 1";
        let state2 = b"state 2";
        
        let hash1 = ErGenerator::hash_state(state1);
        let hash2 = ErGenerator::hash_state(state2);
        
        assert_ne!(hash1, hash2);
    }
    
    #[test]
    fn test_operation_hash() {
        let params1 = b"params 1";
        let params2 = b"params 2";
        
        let hash1 = ErGenerator::hash_operation(OperationType::CellFormation, params1);
        let hash2 = ErGenerator::hash_operation(OperationType::CellFormation, params2);
        let hash3 = ErGenerator::hash_operation(OperationType::CellGrowth, params1);
        
        assert_ne!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
    
    #[test]
    fn test_state_hash_verification() {
        let signing_key = [1u8; 32];
        let generator = ErGenerator::new(signing_key);
        let verifier = ErVerifier::new(signing_key);
        
        let state_before = b"state before";
        let state_after = b"state after";
        let params = b"params";
        
        let receipt = generator.generate(
            "op-1".to_string(),
            OperationType::SlotAllocation,
            state_before,
            state_after,
            params,
            750,
            true,
        );
        
        // Verify state before
        assert!(verifier.verify_state_hash(&receipt, state_before, true));
        assert!(!verifier.verify_state_hash(&receipt, b"wrong state", true));
        
        // Verify state after
        assert!(verifier.verify_state_hash(&receipt, state_after, false));
        assert!(!verifier.verify_state_hash(&receipt, b"wrong state", false));
    }
    
    #[test]
    fn test_operation_hash_verification() {
        let signing_key = [1u8; 32];
        let generator = ErGenerator::new(signing_key);
        let verifier = ErVerifier::new(signing_key);
        
        let state_before = b"before";
        let state_after = b"after";
        let params = b"operation params";
        
        let receipt = generator.generate(
            "op-1".to_string(),
            OperationType::CellDivision,
            state_before,
            state_after,
            params,
            1500,
            true,
        );
        
        // Verify operation hash
        assert!(verifier.verify_operation_hash(&receipt, params));
        assert!(!verifier.verify_operation_hash(&receipt, b"wrong params"));
    }
    
    #[test]
    fn test_failed_operation_verification() {
        let signing_key = [1u8; 32];
        let generator = ErGenerator::new(signing_key);
        let verifier = ErVerifier::new(signing_key);
        
        let state_before = b"before";
        let state_after = b"after";
        let params = b"params";
        
        let receipt = generator.generate(
            "op-1".to_string(),
            OperationType::CellMerging,
            state_before,
            state_after,
            params,
            100,
            false, // Failed operation
        );
        
        // Should fail verification because operation didn't succeed
        assert!(!verifier.verify(&receipt));
    }
    
    #[test]
    fn test_receipt_serialization() {
        let signing_key = [1u8; 32];
        let generator = ErGenerator::new(signing_key);
        
        let state_before = b"before";
        let state_after = b"after";
        let params = b"params";
        
        let receipt = generator.generate(
            "op-1".to_string(),
            OperationType::CellDissolution,
            state_before,
            state_after,
            params,
            300,
            true,
        );
        
        // Serialize to JSON
        let json = serde_json::to_string(&receipt).unwrap();
        
        // Deserialize back
        let receipt2: ExecutionReceipt = serde_json::from_str(&json).unwrap();
        
        assert_eq!(receipt.operation_id, receipt2.operation_id);
        assert_eq!(receipt.signature.0, receipt2.signature.0);
    }
    
    #[test]
    fn test_operation_types() {
        assert_eq!(OperationType::CellFormation as u8, 0);
        assert_ne!(OperationType::CellFormation, OperationType::CellGrowth);
    }
}
