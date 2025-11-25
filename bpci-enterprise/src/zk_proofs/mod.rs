//! Zero-Knowledge Proofs
//! 
//! Implements cryptographic proofs for LCCD operations and state transitions.
//! 
//! # Proof Types
//! 
//! - **CPR (Cell Proof of Residency)**: Proves cell existence and properties
//! - **MPR (Migration Proof of Residency)**: Proves migration correctness
//! - **ER (Execution Receipt)**: Proves operation execution
//! 
//! # Cryptographic Primitives
//! 
//! - **Commitments**: SHA-256 hash-based commitments
//! - **Signatures**: Ed25519 digital signatures (simulated)
//! - **Timestamps**: Unix timestamps for temporal ordering
//! 
//! # Security Properties
//! 
//! - **Completeness**: Valid proofs always verify
//! - **Soundness**: Invalid proofs never verify
//! - **Zero-Knowledge**: Proofs reveal minimal information

pub mod cpr;
pub mod mpr;
pub mod er;

// Re-export main types
pub use cpr::{
    CellProofOfResidency,
    CprGenerator,
    CprVerifier,
    Signature,
    Commitment,
};
pub use mpr::{
    MigrationProofOfResidency,
    MprGenerator,
    MprVerifier,
};
pub use er::{
    ExecutionReceipt,
    ErGenerator,
    ErVerifier,
    OperationType,
};

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::lccd::cell::{LccdCell, CellId, CurvatureProfile, CellHealth, CellState};
    
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
    fn test_end_to_end_cpr() {
        // Setup
        let signing_key = [42u8; 32];
        let generator = CprGenerator::new(signing_key);
        let verifier = CprVerifier::new(signing_key);
        
        // Create cell
        let cell = create_test_cell(1);
        
        // Generate proof
        let proof = generator.generate(&cell);
        
        // Verify proof
        assert!(verifier.verify(&proof));
        
        // Verify commitments
        assert!(verifier.verify_curvature_commitment(&proof, &cell.curvature_profile));
        assert!(verifier.verify_health_commitment(&proof, &cell.health));
    }
}
