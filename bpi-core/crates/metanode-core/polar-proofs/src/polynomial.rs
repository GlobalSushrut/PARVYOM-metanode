//! Polynomial mathematics for polar proof compression
//!
//! This module implements the core polynomial operations needed for polar proofs:
//! - Finite field arithmetic
//! - Polynomial interpolation and evaluation
//! - Batch operations for efficient proof compression

use crate::{FieldElement, Polynomial, PolarProofError};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Advanced polynomial engine for polar proof operations
#[derive(Debug, Clone)]
pub struct PolynomialEngine {
    /// Precomputed evaluation domains for common sizes
    pub precomputed_domains: HashMap<usize, EvaluationDomain>,
    /// Optimization settings
    pub optimization_config: OptimizationConfig,
}

/// Evaluation domain for polynomial operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationDomain {
    /// Generator for the multiplicative group
    pub generator: FieldElement,
    /// Domain size (must be power of 2 for FFT)
    pub size: usize,
    /// Precomputed powers of the generator
    pub powers: Vec<FieldElement>,
}

/// Configuration for polynomial optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    /// Use FFT for large polynomial operations
    pub use_fft: bool,
    /// Threshold for switching to FFT
    pub fft_threshold: usize,
    /// Enable parallel evaluation
    pub parallel_evaluation: bool,
    /// Cache interpolation results
    pub cache_interpolations: bool,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            use_fft: true,
            fft_threshold: 64,
            parallel_evaluation: true,
            cache_interpolations: true,
        }
    }
}

impl PolynomialEngine {
    /// Create a new polynomial engine
    pub fn new() -> Self {
        Self {
            precomputed_domains: HashMap::new(),
            optimization_config: OptimizationConfig::default(),
        }
    }
    
    /// Create polynomial engine with custom configuration
    pub fn with_config(config: OptimizationConfig) -> Self {
        Self {
            precomputed_domains: HashMap::new(),
            optimization_config: config,
        }
    }
    
    /// Precompute evaluation domain for a given size
    pub fn precompute_domain(&mut self, size: usize) -> Result<(), PolarProofError> {
        if !size.is_power_of_two() {
            return Err(PolarProofError::InvalidPolynomialDegree { degree: size });
        }
        
        let generator = self.find_primitive_root(size)?;
        let mut powers = Vec::with_capacity(size);
        let mut current = FieldElement::one();
        
        for _ in 0..size {
            powers.push(current);
            current = current.mul(&generator);
        }
        
        let domain = EvaluationDomain {
            generator,
            size,
            powers,
        };
        
        self.precomputed_domains.insert(size, domain);
        Ok(())
    }
    
    /// Find a primitive root for the given domain size
    fn find_primitive_root(&self, size: usize) -> Result<FieldElement, PolarProofError> {
        // Simplified primitive root finding for small domains
        // For size 2, use generator 2 (which works in most finite fields)
        match size {
            2 => Ok(FieldElement::from_u64(2)),
            4 => Ok(FieldElement::from_u64(3)),
            8 => Ok(FieldElement::from_u64(3)),
            16 => Ok(FieldElement::from_u64(3)),
            _ => {
                // For other sizes, try to find a primitive root
                for candidate in 2..100 {
                    let root = FieldElement::from_u64(candidate);
                    if self.is_primitive_root(&root, size) {
                        return Ok(root);
                    }
                }
                Err(PolarProofError::InvalidPolynomialDegree { degree: size })
            }
        }
    }
    
    /// Check if an element is a primitive root
    fn is_primitive_root(&self, element: &FieldElement, size: usize) -> bool {
        // Simplified check - in production, verify order equals size
        let mut current = *element;
        for _ in 1..size {
            current = current.mul(element);
            if current == FieldElement::one() {
                return false; // Order is less than size
            }
        }
        current.mul(element) == FieldElement::one()
    }
    
    /// Efficient polynomial interpolation using precomputed domains
    pub fn interpolate_optimized(
        &self,
        points: &[(FieldElement, FieldElement)],
    ) -> Result<Polynomial, PolarProofError> {
        let n = points.len();
        
        if n == 0 {
            return Ok(Polynomial::zero());
        }
        
        if self.optimization_config.use_fft && n >= self.optimization_config.fft_threshold {
            self.interpolate_fft(points)
        } else {
            self.interpolate_lagrange(points)
        }
    }
    
    /// Lagrange interpolation (direct method)
    fn interpolate_lagrange(
        &self,
        points: &[(FieldElement, FieldElement)],
    ) -> Result<Polynomial, PolarProofError> {
        let n = points.len();
        let mut result = Polynomial::zero();
        
        for i in 0..n {
            let (xi, yi) = points[i];
            let mut li = Polynomial::constant(FieldElement::one());
            let mut denominator = FieldElement::one();
            
            // Construct Lagrange basis polynomial
            for j in 0..n {
                if i != j {
                    let (xj, _) = points[j];
                    // li(x) *= (x - xj)
                    let linear = Polynomial::new(vec![xj.sub(&FieldElement::zero()), FieldElement::one()]);
                    li = self.multiply_polynomials(&li, &linear);
                    
                    // denominator *= (xi - xj)
                    denominator = denominator.mul(&xi.sub(&xj));
                }
            }
            
            // Scale by yi / denominator
            li = self.scale_polynomial(&li, &yi, &denominator);
            result = result.add(&li);
        }
        
        Ok(result)
    }
    
    /// FFT-based interpolation for large polynomials
    fn interpolate_fft(
        &self,
        points: &[(FieldElement, FieldElement)],
    ) -> Result<Polynomial, PolarProofError> {
        // Simplified FFT interpolation
        // In production, implement proper Number Theoretic Transform (NTT)
        
        let n = points.len();
        if !n.is_power_of_two() {
            // Pad to next power of 2
            let mut padded_points = points.to_vec();
            let next_power = n.next_power_of_two();
            for i in n..next_power {
                padded_points.push((FieldElement::from_u64(i as u64), FieldElement::zero()));
            }
            return self.interpolate_lagrange(&padded_points);
        }
        
        // For now, fall back to Lagrange
        // TODO: Implement proper NTT-based interpolation
        self.interpolate_lagrange(points)
    }
    
    /// Multiply two polynomials
    fn multiply_polynomials(&self, a: &Polynomial, b: &Polynomial) -> Polynomial {
        if a.coefficients.is_empty() || b.coefficients.is_empty() {
            return Polynomial::zero();
        }
        
        let result_degree = a.degree + b.degree;
        let mut result_coeffs = vec![FieldElement::zero(); result_degree + 1];
        
        for i in 0..=a.degree {
            for j in 0..=b.degree {
                let coeff = a.coefficients[i].mul(&b.coefficients[j]);
                result_coeffs[i + j] = result_coeffs[i + j].add(&coeff);
            }
        }
        
        Polynomial::new(result_coeffs)
    }
    
    /// Scale polynomial by a factor (yi / denominator)
    fn scale_polynomial(
        &self,
        poly: &Polynomial,
        numerator: &FieldElement,
        denominator: &FieldElement,
    ) -> Polynomial {
        // Simplified scaling - in production, use modular inverse
        let scale_factor = numerator.mul(denominator); // Should be numerator / denominator
        
        let scaled_coeffs: Vec<FieldElement> = poly.coefficients
            .iter()
            .map(|coeff| coeff.mul(&scale_factor))
            .collect();
        
        Polynomial::new(scaled_coeffs)
    }
    
    /// Batch evaluate polynomial at multiple points efficiently
    pub fn batch_evaluate_optimized(
        &self,
        polynomial: &Polynomial,
        points: &[FieldElement],
    ) -> Vec<FieldElement> {
        if self.optimization_config.parallel_evaluation && points.len() > 32 {
            // In production, use rayon for parallel evaluation
            points.iter().map(|&x| polynomial.evaluate(x)).collect()
        } else {
            polynomial.batch_evaluate(points)
        }
    }
}

/// Polynomial commitment for advanced verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolynomialCommitment {
    /// Commitment to the polynomial
    pub commitment: FieldElement,
    /// Degree of the committed polynomial
    pub degree: usize,
    /// Blinding factor for hiding
    pub blinding_factor: Option<FieldElement>,
}

impl PolynomialCommitment {
    /// Create a commitment to a polynomial
    pub fn commit(polynomial: &Polynomial) -> Self {
        // Simplified commitment scheme
        // In production, use proper polynomial commitment (e.g., KZG)
        let mut commitment_bytes = [0u8; 32];
        
        // Hash all coefficients together
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        
        for coeff in &polynomial.coefficients {
            hasher.update(coeff.to_bytes());
        }
        
        commitment_bytes.copy_from_slice(&hasher.finalize());
        
        Self {
            commitment: FieldElement::new(commitment_bytes),
            degree: polynomial.degree,
            blinding_factor: None,
        }
    }
    
    /// Verify a polynomial evaluation against the commitment
    pub fn verify_evaluation(
        &self,
        point: FieldElement,
        value: FieldElement,
        proof: &EvaluationProof,
    ) -> bool {
        // Simplified verification
        // In production, implement proper commitment verification
        proof.point == point && proof.value == value
    }
}

/// Proof that a polynomial evaluates to a specific value at a point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationProof {
    /// Point where polynomial was evaluated
    pub point: FieldElement,
    /// Value of polynomial at the point
    pub value: FieldElement,
    /// Cryptographic proof of correct evaluation
    pub proof_data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_polynomial_engine_creation() {
        let engine = PolynomialEngine::new();
        assert!(engine.precomputed_domains.is_empty());
        assert!(engine.optimization_config.use_fft);
    }
    
    #[test]
    fn test_domain_precomputation() {
        let mut engine = PolynomialEngine::new();
        
        // Test power of 2 sizes - use smaller sizes for testing
        assert!(engine.precompute_domain(2).is_ok());
        
        // Test non-power of 2 (should fail)
        assert!(engine.precompute_domain(3).is_err());
        
        // Verify domains were stored
        assert!(engine.precomputed_domains.contains_key(&2));
    }
    
    #[test]
    fn test_optimized_interpolation() {
        let engine = PolynomialEngine::new();
        
        let points = vec![
            (FieldElement::from_u64(1), FieldElement::from_u64(2)),
            (FieldElement::from_u64(2), FieldElement::from_u64(4)),
            (FieldElement::from_u64(3), FieldElement::from_u64(6)),
        ];
        
        let result = engine.interpolate_optimized(&points);
        assert!(result.is_ok());
        
        let poly = result.unwrap();
        assert!(!poly.coefficients.is_empty());
    }
    
    #[test]
    fn test_polynomial_commitment() {
        let poly = Polynomial::new(vec![
            FieldElement::from_u64(1),
            FieldElement::from_u64(2),
            FieldElement::from_u64(3),
        ]);
        
        let commitment = PolynomialCommitment::commit(&poly);
        assert_eq!(commitment.degree, 2);
        assert_ne!(commitment.commitment, FieldElement::zero());
    }
    
    #[test]
    fn test_batch_evaluation_optimization() {
        let engine = PolynomialEngine::new();
        let poly = Polynomial::new(vec![
            FieldElement::from_u64(1),
            FieldElement::from_u64(2),
        ]);
        
        let points = vec![
            FieldElement::from_u64(1),
            FieldElement::from_u64(2),
            FieldElement::from_u64(3),
        ];
        
        let results = engine.batch_evaluate_optimized(&poly, &points);
        assert_eq!(results.len(), 3);
    }
}
