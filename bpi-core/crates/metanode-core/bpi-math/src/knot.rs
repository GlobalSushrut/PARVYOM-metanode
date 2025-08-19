//! Knot Theory Framework for Transaction Immutability
//! 
//! This module implements knot theory structures to ensure transaction
//! immutability through topological invariants

use crate::{Hash, MathError, Timestamp};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Alexander polynomial coefficients for knot invariants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlexanderPolynomial {
    coefficients: Vec<i64>,
    degree: i32,
}

impl AlexanderPolynomial {
    pub fn new(coefficients: Vec<i64>) -> Self {
        let degree = coefficients.len() as i32 - 1;
        Self { coefficients, degree }
    }
    
    /// Evaluate polynomial at a given point
    pub fn evaluate(&self, x: f64) -> f64 {
        self.coefficients.iter().enumerate()
            .map(|(i, &coeff)| coeff as f64 * x.powi(i as i32))
            .sum()
    }
    
    /// Check if polynomial is invariant (non-zero)
    pub fn is_invariant(&self) -> bool {
        !self.coefficients.iter().all(|&c| c == 0)
    }
}

/// Knot invariant for maintaining transaction immutability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnotInvariant {
    alexander_poly: AlexanderPolynomial,
    linking_number: i32,
    writhe: i32,
    pub invariant_hash: Hash,
}

impl KnotInvariant {
    pub fn new(receipt_chain: &[Hash], proof_chain: &[Hash]) -> Self {
        let alexander_poly = Self::compute_alexander_polynomial(receipt_chain, proof_chain);
        let linking_number = Self::compute_linking_number(receipt_chain, proof_chain);
        let writhe = Self::compute_writhe(receipt_chain, proof_chain);
        
        // Compute invariant hash from all components
        let mut hasher = blake3::Hasher::new();
        hasher.update(&bincode::serialize(&alexander_poly.coefficients).unwrap_or_default());
        hasher.update(&linking_number.to_le_bytes());
        hasher.update(&writhe.to_le_bytes());
        let invariant_hash = hasher.finalize().into();
        
        Self {
            alexander_poly,
            linking_number,
            writhe,
            invariant_hash,
        }
    }
    
    /// Verify that the knot invariant is preserved
    pub fn verify_invariant(&self, other: &KnotInvariant) -> bool {
        self.alexander_poly.coefficients == other.alexander_poly.coefficients &&
        self.linking_number == other.linking_number &&
        self.writhe == other.writhe
    }
    
    /// Compute next block height using knot invariant
    pub fn next_height(&self, current_height: u64) -> u64 {
        // Use knot invariant to ensure monotonic height increase
        let invariant_sum = self.alexander_poly.coefficients.iter().sum::<i64>().unsigned_abs();
        current_height + 1 + (invariant_sum % 3) // Add 1-3 based on invariant
    }
    
    fn compute_alexander_polynomial(receipt_chain: &[Hash], proof_chain: &[Hash]) -> AlexanderPolynomial {
        // Simplified Alexander polynomial computation
        // In practice, this would involve complex knot theory calculations
        let mut coefficients = Vec::new();
        
        for (receipt, proof) in receipt_chain.iter().zip(proof_chain.iter()) {
            let combined = [receipt.as_slice(), proof.as_slice()].concat();
            let hash_sum = combined.iter().map(|&b| b as i64).sum::<i64>();
            coefficients.push(hash_sum % 1000); // Normalize coefficient
        }
        
        if coefficients.is_empty() {
            coefficients.push(1); // Ensure non-trivial polynomial
        }
        
        AlexanderPolynomial::new(coefficients)
    }
    
    fn compute_linking_number(receipt_chain: &[Hash], proof_chain: &[Hash]) -> i32 {
        // Compute linking number between receipt and proof chains
        let mut linking = 0i32;
        
        for (receipt, proof) in receipt_chain.iter().zip(proof_chain.iter()) {
            let receipt_sum = receipt.iter().map(|&b| b as i32).sum::<i32>();
            let proof_sum = proof.iter().map(|&b| b as i32).sum::<i32>();
            linking += (receipt_sum ^ proof_sum) % 7; // Topological linking
        }
        
        linking
    }
    
    fn compute_writhe(receipt_chain: &[Hash], proof_chain: &[Hash]) -> i32 {
        // Compute writhe (self-linking) of the combined chain
        let mut writhe = 0i32;
        
        let combined_chain: Vec<_> = receipt_chain.iter()
            .zip(proof_chain.iter())
            .map(|(r, p)| [r.as_slice(), p.as_slice()].concat())
            .collect();
        
        for i in 0..combined_chain.len() {
            for j in (i + 1)..combined_chain.len() {
                let crossing = Self::compute_crossing(&combined_chain[i], &combined_chain[j]);
                writhe += crossing;
            }
        }
        
        writhe
    }
    
    fn compute_crossing(chain1: &[u8], chain2: &[u8]) -> i32 {
        // Simplified crossing number computation
        let sum1 = chain1.iter().map(|&b| b as i32).sum::<i32>();
        let sum2 = chain2.iter().map(|&b| b as i32).sum::<i32>();
        ((sum1 ^ sum2) % 3) - 1 // Returns -1, 0, or 1
    }
}

/// Transaction knot linking receipts and proofs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionKnot {
    receipt_chain: VecDeque<Hash>,
    proof_chain: VecDeque<Hash>,
    temporal_ordering: Vec<Timestamp>,
    knot_invariant: KnotInvariant,
}

impl TransactionKnot {
    pub fn new() -> Self {
        Self {
            receipt_chain: VecDeque::new(),
            proof_chain: VecDeque::new(),
            temporal_ordering: Vec::new(),
            knot_invariant: KnotInvariant {
                alexander_poly: AlexanderPolynomial::new(vec![1]),
                linking_number: 0,
                writhe: 0,
                invariant_hash: [0u8; 32], // Default hash for empty knot
            },
        }
    }
    
    /// Add a receipt-proof pair to the knot
    pub fn add_link(&mut self, receipt_hash: Hash, proof_hash: Hash, timestamp: Timestamp) -> Result<(), MathError> {
        // Verify temporal ordering
        if let Some(last_time) = self.temporal_ordering.last() {
            if timestamp <= *last_time {
                return Err(MathError::KnotInvariant(
                    "Temporal ordering violation".to_string()
                ));
            }
        }
        
        self.receipt_chain.push_back(receipt_hash);
        self.proof_chain.push_back(proof_hash);
        self.temporal_ordering.push(timestamp);
        
        // Recompute knot invariant
        let receipt_vec: Vec<Hash> = self.receipt_chain.iter().cloned().collect();
        let proof_vec: Vec<Hash> = self.proof_chain.iter().cloned().collect();
        self.knot_invariant = KnotInvariant::new(&receipt_vec, &proof_vec);
        
        Ok(())
    }
    
    /// Verify knot immutability
    pub fn verify_immutability(&self) -> bool {
        // Check that Alexander polynomial is non-trivial
        if !self.knot_invariant.alexander_poly.is_invariant() {
            return false;
        }
        
        // Verify temporal ordering
        for window in self.temporal_ordering.windows(2) {
            if window[0] >= window[1] {
                return false;
            }
        }
        
        // Verify chain lengths match
        self.receipt_chain.len() == self.proof_chain.len() &&
        self.receipt_chain.len() == self.temporal_ordering.len()
    }
    
    /// Get knot invariant for external verification
    pub fn get_invariant(&self) -> &KnotInvariant {
        &self.knot_invariant
    }
    
    /// Compute next block height using knot mathematics
    pub fn compute_next_height(&self, current_height: u64) -> u64 {
        self.knot_invariant.next_height(current_height)
    }
    
    /// Create transaction hash from knot structure
    pub fn transaction_hash(&self) -> Hash {
        let mut data = Vec::new();
        
        // Add all receipt hashes
        for receipt in &self.receipt_chain {
            data.extend_from_slice(receipt);
        }
        
        // Add all proof hashes
        for proof in &self.proof_chain {
            data.extend_from_slice(proof);
        }
        
        // Add temporal ordering
        for timestamp in &self.temporal_ordering {
            data.extend_from_slice(&timestamp.timestamp().to_be_bytes());
        }
        
        // Add knot invariant
        let invariant_data = serde_json::to_vec(&self.knot_invariant).unwrap_or_default();
        data.extend_from_slice(&invariant_data);
        
        crate::hash_data(&data)
    }
}

impl Default for TransactionKnot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    
    #[test]
    fn test_alexander_polynomial() {
        let poly = AlexanderPolynomial::new(vec![1, -1, 1]);
        assert_eq!(poly.degree, 2);
        assert!(poly.is_invariant());
        
        let value = poly.evaluate(1.0);
        assert_eq!(value, 1.0);
    }
    
    #[test]
    fn test_knot_invariant() {
        let receipts = vec![
            crate::hash_data(b"receipt1"),
            crate::hash_data(b"receipt2"),
        ];
        let proofs = vec![
            crate::hash_data(b"proof1"),
            crate::hash_data(b"proof2"),
        ];
        
        let invariant1 = KnotInvariant::new(&receipts, &proofs);
        let invariant2 = KnotInvariant::new(&receipts, &proofs);
        
        assert!(invariant1.verify_invariant(&invariant2));
    }
    
    #[test]
    fn test_transaction_knot() {
        let mut knot = TransactionKnot::new();
        
        let receipt1 = crate::hash_data(b"receipt1");
        let proof1 = crate::hash_data(b"proof1");
        let time1 = Utc::now();
        
        let receipt2 = crate::hash_data(b"receipt2");
        let proof2 = crate::hash_data(b"proof2");
        let time2 = time1 + chrono::Duration::seconds(1);
        
        assert!(knot.add_link(receipt1, proof1, time1).is_ok());
        assert!(knot.add_link(receipt2, proof2, time2).is_ok());
        
        assert!(knot.verify_immutability());
        
        let next_height = knot.compute_next_height(1000);
        assert!(next_height > 1000);
    }
    
    #[test]
    fn test_temporal_ordering_violation() {
        let mut knot = TransactionKnot::new();
        
        let receipt1 = crate::hash_data(b"receipt1");
        let proof1 = crate::hash_data(b"proof1");
        let time1 = Utc::now();
        
        let receipt2 = crate::hash_data(b"receipt2");
        let proof2 = crate::hash_data(b"proof2");
        let time2 = time1 - chrono::Duration::seconds(1); // Earlier time
        
        assert!(knot.add_link(receipt1, proof1, time1).is_ok());
        assert!(knot.add_link(receipt2, proof2, time2).is_err());
    }
}
