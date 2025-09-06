//! BPI Mathematical Foundation
//! 
//! This crate provides the Mathematical Foundations for Metanode Blockchain
//! This crate provides category theory, knot theory, and proof systems
//! for military-grade blockchain ledger and mining operations.

pub mod constants;
pub mod category;
pub mod consensus_integration;
pub mod bpci_registry_guard;
pub mod production_bpci_client;
pub mod proofs;
pub mod receipts;
pub mod poe_calculator;
pub mod mining;
// Integration modules for full mathematical foundation
// Temporarily commenting out complex integration modules to focus on core functionality
// pub mod ledger_6d;
// pub mod network_6d;  // Keep commented until needed
// pub mod metanode_integration;
// pub mod integration_test;
pub mod phase1_integration_test;
pub mod phase_tests;
pub mod minimal_test;

// use chrono::{DateTime, Utc};  // Remove unused chrono imports
// use serde::{Deserialize, Serialize};  // Remove unused serde imports
use sha2::{Digest, Sha256};
use thiserror::Error;

/// Mathematical errors in the blockchain system
#[derive(Error, Debug)]
pub enum MathError {
    #[error("Category composition failed: {0}")]
    CategoryComposition(String),
    #[error("Knot invariant violation: {0}")]
    KnotInvariant(String),
    #[error("Proof verification failed: {0}")]
    ProofVerification(String),
    #[error("Receipt aggregation error: {0}")]
    ReceiptAggregation(String),
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    #[error("Mining timeout")]
    MiningTimeout,
    #[error("Not implemented")]
    NotImplemented,
    #[error("Capacity exceeded")]
    CapacityExceeded,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Security violation: {0}")]
    SecurityViolation(String),
}

/// Core mathematical types
pub type Hash = [u8; 32];
pub type Timestamp = chrono::DateTime<chrono::Utc>;

/// Generate cryptographic hash
pub fn hash_data(data: &[u8]) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}



/// Re-export main types
pub use category::{LedgerCategory, LedgerMorphism, LedgerObject};
// TODO: Add knot module when available
// pub use knot::{TransactionKnot, AlexanderPolynomial, KnotInvariant};
pub use proofs::{ProofSystem, ProofOfAction, ProofOfExecution, ProofOfTransact, ProofOfGold, ProofOfHistory};
pub use receipts::{ReceiptAggregator, ReceiptType, AggregatedTransaction};
pub use mining::{MiningEngine, MiningCandidate, MiningDifficulty};
