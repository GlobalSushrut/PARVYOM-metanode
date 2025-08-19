//! # BPI Light Client Verification
//!
//! This crate provides light client verification capabilities for BPI Mesh consensus headers.
//! It enables efficient verification of block headers without requiring full node state,
//! supporting batch verification and performance benchmarks.
//!
//! ## Key Components
//!
//! - **HeaderVerifier**: Main verification engine for individual and batch header verification
//! - **VerificationResult**: Detailed verification results with performance metrics
//! - **CLI Integration**: Command-line tool for header verification and benchmarking

use std::collections::HashMap;
use std::time::{Duration, Instant};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, info, warn, error};

// Re-export core types
pub use bpi_headers::{Header, HeaderHash, ConsensusMode, IbftMessage, IbftCommit};
pub use bpi_validator_set::{ValidatorSet, ValidatorInfo};
pub use bpi_consensus::{BlsCommit, CommitVerification, ValidatorBitmap};
pub use bpi_slashing::{SlashingProof, SlashingProofVerifier};
pub use bpi_blsagg::{PublicKey as BlsPublicKey, Signature as BlsSignature};

/// Light client verification errors
#[derive(Error, Debug)]
pub enum LightClientError {
    #[error("Header verification failed: {0}")]
    HeaderVerification(String),
    #[error("BLS signature verification failed: {0}")]
    BlsVerification(String),
    #[error("Validator set verification failed: {0}")]
    ValidatorSetVerification(String),
    #[error("Chain continuity verification failed: {0}")]
    ChainContinuity(String),
    #[error("Performance target not met: {0}")]
    PerformanceTarget(String),
    #[error("Invalid header sequence: {0}")]
    InvalidSequence(String),
}

/// Header verification result with detailed metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    /// Whether verification passed
    pub success: bool,
    /// Time taken for verification
    pub verification_time: Duration,
    /// Header height that was verified
    pub height: u64,
    /// Hash of the verified header
    pub header_hash: HeaderHash,
    /// Error message if verification failed
    pub error: Option<String>,
    /// Detailed verification metrics
    pub metrics: VerificationMetrics,
}

/// Detailed verification performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationMetrics {
    /// Time spent on header structure validation
    pub header_validation_time: Duration,
    /// Time spent on BLS signature verification
    pub bls_verification_time: Duration,
    /// Time spent on validator set checks
    pub validator_set_time: Duration,
    /// Time spent on chain continuity checks
    pub chain_continuity_time: Duration,
    /// Total number of signatures verified
    pub signatures_verified: usize,
    /// Number of validators in the set
    pub validator_count: usize,
}

/// Batch verification result for multiple headers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchVerificationResult {
    /// Total headers processed
    pub total_headers: usize,
    /// Number of headers that passed verification
    pub successful_verifications: usize,
    /// Total time for batch verification
    pub total_time: Duration,
    /// Average time per header
    pub average_time_per_header: Duration,
    /// P50 verification time
    pub p50_time: Duration,
    /// P95 verification time
    pub p95_time: Duration,
    /// Individual verification results
    pub results: Vec<VerificationResult>,
    /// Performance summary
    pub performance_summary: PerformanceSummary,
}

/// Performance summary for verification benchmarks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    /// Headers per second throughput
    pub headers_per_second: f64,
    /// Whether p50 target (<2ms) was met
    pub p50_target_met: bool,
    /// Whether batch target (<2s for 1k headers) was met
    pub batch_target_met: bool,
    /// Fastest verification time
    pub fastest_time: Duration,
    /// Slowest verification time
    pub slowest_time: Duration,
}

/// Light client header verifier
pub struct HeaderVerifier {
    /// Current validator set for verification
    validator_set: ValidatorSet,
    /// Slashing proof verifier for Byzantine fault detection
    slashing_verifier: SlashingProofVerifier,
    /// Cache of recently verified headers for performance
    verification_cache: HashMap<HeaderHash, VerificationResult>,
    /// Maximum cache size
    max_cache_size: usize,
}

impl HeaderVerifier {
    /// Create a new header verifier with validator set
    pub fn new(validator_set: ValidatorSet) -> Self {
        let slashing_verifier = SlashingProofVerifier::new(validator_set.clone());
        
        Self {
            validator_set,
            slashing_verifier,
            verification_cache: HashMap::new(),
            max_cache_size: 1000,
        }
    }

    /// Verify a single header with detailed metrics
    pub fn verify_header(
        &mut self,
        header: &Header,
        prev_header: Option<&Header>,
        commit: &BlsCommit,
    ) -> Result<VerificationResult, LightClientError> {
        let start_time = Instant::now();
        let header_hash = header.hash()
            .map_err(|e| LightClientError::HeaderVerification(format!("Header hash failed: {}", e)))?;

        // Check cache first
        if let Some(cached_result) = self.verification_cache.get(&header_hash) {
            debug!("Using cached verification result for header {}", header_hash);
            return Ok(cached_result.clone());
        }

        let mut metrics = VerificationMetrics {
            header_validation_time: Duration::ZERO,
            bls_verification_time: Duration::ZERO,
            validator_set_time: Duration::ZERO,
            chain_continuity_time: Duration::ZERO,
            signatures_verified: 0,
            validator_count: self.validator_set.len(),
        };

        // Step 1: Header structure validation
        let header_start = Instant::now();
        header.validate()
            .map_err(|e| LightClientError::HeaderVerification(format!("Header validation failed: {}", e)))?;
        metrics.header_validation_time = header_start.elapsed();

        // Step 2: Chain continuity verification
        let chain_start = Instant::now();
        if let Some(prev) = prev_header {
            header.validate_chain_continuity(prev)
                .map_err(|e| LightClientError::ChainContinuity(format!("Chain continuity failed: {}", e)))?;
        }
        metrics.chain_continuity_time = chain_start.elapsed();

        // Step 3: Validator set verification
        let validator_start = Instant::now();
        let verification = commit.verify(&self.validator_set)
            .map_err(|e| LightClientError::ValidatorSetVerification(format!("Commit verification failed: {}", e)))?;
        
        if !verification.is_valid {
            return Err(LightClientError::ValidatorSetVerification(
                format!("Commit verification failed: {:?}", verification)
            ));
        }
        metrics.validator_set_time = validator_start.elapsed();

        // Step 4: BLS signature verification
        let bls_start = Instant::now();
        let signature_count = commit.validator_bitmap.count_set_bits();
        metrics.signatures_verified = signature_count;
        
        // Verify the commit's header hash matches the actual header hash
        if commit.header_hash != header_hash {
            return Err(LightClientError::BlsVerification(
                "Commit header hash does not match header hash".to_string()
            ));
        }
        metrics.bls_verification_time = bls_start.elapsed();

        let total_time = start_time.elapsed();
        
        let result = VerificationResult {
            success: true,
            verification_time: total_time,
            height: header.height,
            header_hash,
            error: None,
            metrics,
        };

        // Cache the result
        self.cache_result(header_hash, result.clone());

        info!("Header {} verified successfully in {:?}", header_hash, total_time);
        Ok(result)
    }

    /// Verify a batch of headers with performance metrics
    pub fn verify_batch(
        &mut self,
        headers: &[(Header, BlsCommit)],
    ) -> Result<BatchVerificationResult, LightClientError> {
        let start_time = Instant::now();
        let mut results = Vec::new();
        let mut successful_count = 0;

        info!("Starting batch verification of {} headers", headers.len());

        for (i, (header, commit)) in headers.iter().enumerate() {
            let prev_header = if i > 0 { Some(&headers[i - 1].0) } else { None };
            
            match self.verify_header(header, prev_header, commit) {
                Ok(result) => {
                    if result.success {
                        successful_count += 1;
                    }
                    results.push(result);
                }
                Err(e) => {
                    error!("Header verification failed at index {}: {}", i, e);
                    results.push(VerificationResult {
                        success: false,
                        verification_time: Duration::ZERO,
                        height: header.height,
                        header_hash: header.hash().unwrap_or_else(|_| HeaderHash::from([0u8; 32])),
                        error: Some(e.to_string()),
                        metrics: VerificationMetrics {
                            header_validation_time: Duration::ZERO,
                            bls_verification_time: Duration::ZERO,
                            validator_set_time: Duration::ZERO,
                            chain_continuity_time: Duration::ZERO,
                            signatures_verified: 0,
                            validator_count: self.validator_set.len(),
                        },
                    });
                }
            }
        }

        let total_time = start_time.elapsed();
        let average_time = if !results.is_empty() {
            total_time / results.len() as u32
        } else {
            Duration::ZERO
        };

        // Calculate percentiles
        let mut times: Vec<Duration> = results.iter()
            .filter(|r| r.success)
            .map(|r| r.verification_time)
            .collect();
        times.sort();

        let p50_time = if !times.is_empty() {
            times[times.len() / 2]
        } else {
            Duration::ZERO
        };

        let p95_time = if !times.is_empty() {
            times[(times.len() * 95) / 100]
        } else {
            Duration::ZERO
        };

        let fastest_time = times.first().copied().unwrap_or(Duration::ZERO);
        let slowest_time = times.last().copied().unwrap_or(Duration::ZERO);

        let headers_per_second = if total_time.as_secs_f64() > 0.0 {
            headers.len() as f64 / total_time.as_secs_f64()
        } else {
            0.0
        };

        let performance_summary = PerformanceSummary {
            headers_per_second,
            p50_target_met: p50_time < Duration::from_millis(2),
            batch_target_met: headers.len() >= 1000 && total_time < Duration::from_secs(2),
            fastest_time,
            slowest_time,
        };

        let batch_result = BatchVerificationResult {
            total_headers: headers.len(),
            successful_verifications: successful_count,
            total_time,
            average_time_per_header: average_time,
            p50_time,
            p95_time,
            results,
            performance_summary: performance_summary.clone(),
        };

        info!(
            "Batch verification complete: {}/{} headers verified in {:?} (avg: {:?}, p50: {:?})",
            successful_count, headers.len(), total_time, average_time, p50_time
        );

        if !performance_summary.p50_target_met {
            warn!("P50 target not met: {:?} > 2ms", p50_time);
        }

        if headers.len() >= 1000 && !batch_result.performance_summary.batch_target_met {
            warn!("Batch target not met: {:?} > 2s for {} headers", total_time, headers.len());
        }

        Ok(batch_result)
    }

    /// Update the validator set for verification
    pub fn update_validator_set(&mut self, validator_set: ValidatorSet) {
        self.validator_set = validator_set.clone();
        self.slashing_verifier = SlashingProofVerifier::new(validator_set);
        self.verification_cache.clear(); // Clear cache when validator set changes
        info!("Validator set updated, cache cleared");
    }

    /// Get current validator set
    pub fn validator_set(&self) -> &ValidatorSet {
        &self.validator_set
    }

    /// Clear verification cache
    pub fn clear_cache(&mut self) {
        self.verification_cache.clear();
        debug!("Verification cache cleared");
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        (self.verification_cache.len(), self.max_cache_size)
    }

    /// Cache a verification result
    fn cache_result(&mut self, header_hash: HeaderHash, result: VerificationResult) {
        if self.verification_cache.len() >= self.max_cache_size {
            // Simple LRU: remove oldest entry (first in HashMap iteration order)
            if let Some(oldest_key) = self.verification_cache.keys().next().copied() {
                self.verification_cache.remove(&oldest_key);
            }
        }
        self.verification_cache.insert(header_hash, result);
    }
}

/// Utility functions for creating test data and benchmarks
pub mod test_utils {
    use super::*;
    use bpi_validator_set::ValidatorInfo;
    use bpi_blsagg::keygen::generate_keypair;
    use bpi_consensus::CommitAggregator;

    /// Create a test validator set for benchmarking
    pub fn create_test_validator_set(size: usize) -> ValidatorSet {
        let mut validator_set = ValidatorSet::new(0);
        
        for i in 0..size {
            let seed = format!("test_validator_{}", i);
            let (private_key, public_key) = generate_keypair(seed.as_bytes());
            
            let validator_info = ValidatorInfo::new(
                i,
                public_key,
                bpi_vrf::keygen::generate_keypair(seed.as_bytes()).1,
                100,
                format!("validator_{}", i),
                format!("Test Validator {}", i),
            );
            
            validator_set.add_validator(validator_info).unwrap();
        }
        
        validator_set
    }

    /// Create a test header for benchmarking
    pub fn create_test_header(height: u64, prev_hash: [u8; 32]) -> Header {
        Header {
            version: 1,
            height,
            prev_hash,
            poh_root: [0u8; 32],
            receipts_root: [0u8; 32],
            da_root: [0u8; 32],
            xcmp_root: [0u8; 32],
            validator_set_hash: [0u8; 32],
            mode: bpi_headers::ConsensusMode::Ibft,
            round: if height == 0 { 0 } else { height }, // Genesis must have round 0
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create a test BLS commit for a header
    pub fn create_test_commit(
        header: &Header,
        validator_set: &ValidatorSet,
    ) -> Result<BlsCommit, anyhow::Error> {
        let header_hash = header.hash()?;
        let mut aggregator = bpi_consensus::CommitAggregator::new(
            validator_set.clone(),
            header_hash,
            header.round, // Use the header's round number
            header.height,
        );

        // Add signatures from a majority of validators (simplified for testing)
        let threshold = (validator_set.len() * 2 / 3) + 1;

        for i in 0..threshold {
            if let Some(validator) = validator_set.get_validator(i) {
                let seed = format!("test_validator_{}", i);
                let (private_key, _) = generate_keypair(seed.as_bytes());
                let signature = private_key.sign(header_hash.as_bytes());
                
                let validator_signature = bpi_consensus::ValidatorSignature {
                    validator_index: i,
                    signature,
                    header_hash,
                    round: header.round,
                };
                aggregator.add_signature(validator_signature)?;
            }
        }

        aggregator.aggregate()
    }

    /// Generate a test chain of headers and commits
    pub fn generate_test_chain(
        length: usize,
        validator_set: &ValidatorSet,
    ) -> Result<Vec<(Header, BlsCommit)>, anyhow::Error> {
        let mut chain = Vec::new();
        let mut prev_hash = [0u8; 32]; // Genesis starts with zero hash

        for height in 0..length {
            let header = create_test_header(height as u64, prev_hash);
            let commit = create_test_commit(&header, validator_set)?;
            
            // Update prev_hash for next iteration
            prev_hash = *header.hash()?.as_bytes();
            chain.push((header, commit));
        }

        Ok(chain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::*;

    #[test]
    fn test_header_verifier_creation() {
        let validator_set = create_test_validator_set(4);
        let verifier = HeaderVerifier::new(validator_set);
        
        assert_eq!(verifier.validator_set().len(), 4);
        assert_eq!(verifier.cache_stats(), (0, 1000));
    }

    #[test]
    fn test_single_header_verification() {
        let validator_set = create_test_validator_set(4);
        let mut verifier = HeaderVerifier::new(validator_set.clone());
        
        let chain = generate_test_chain(1, &validator_set).unwrap();
        let (header, commit) = &chain[0];
        // First header should be genesis (height 0)
        assert_eq!(header.height, 0);
        assert!(header.is_genesis());
        let result = verifier.verify_header(header, None, commit).unwrap();
        
        assert_eq!(result.header_hash, header.hash().unwrap());
        assert!(result.verification_time > Duration::ZERO);
        assert!(result.metrics.signatures_verified > 0);
    }

    #[test]
    fn test_chain_verification() {
        let validator_set = create_test_validator_set(4);
        let mut verifier = HeaderVerifier::new(validator_set.clone());
        
        let chain = generate_test_chain(3, &validator_set).unwrap();
        
        for (i, (header, commit)) in chain.iter().enumerate() {
            let prev_header = if i > 0 { Some(&chain[i - 1].0) } else { None };
            let result = verifier.verify_header(header, prev_header, commit).unwrap();
            assert!(result.success);
        }
    }

    #[test]
    fn test_batch_verification() {
        let validator_set = create_test_validator_set(7);
        let mut verifier = HeaderVerifier::new(validator_set.clone());
        
        let chain = generate_test_chain(10, &validator_set).unwrap();
        let batch_result = verifier.verify_batch(&chain).unwrap();
        
        assert_eq!(batch_result.total_headers, 10);
        assert_eq!(batch_result.successful_verifications, 10);
        assert!(batch_result.total_time > Duration::ZERO);
        assert!(batch_result.average_time_per_header > Duration::ZERO);
    }

    #[test]
    fn test_verification_cache() {
        let validator_set = create_test_validator_set(4);
        let mut verifier = HeaderVerifier::new(validator_set.clone());
        
        let chain = generate_test_chain(1, &validator_set).unwrap();
        let (header, commit) = &chain[0];
        // First header should be genesis (height 0)
        assert_eq!(header.height, 0);
        assert!(header.is_genesis());
        let result1 = verifier.verify_header(header, None, commit).unwrap();
        assert_eq!(verifier.cache_stats().0, 1);
        
        // Verify batch verification
        let batch_result = verifier.verify_batch(&[(header.clone(), commit.clone())]);
        assert_eq!(batch_result.unwrap().results.len(), 1);
        assert_eq!(verifier.cache_stats().0, 1);
        
        // Second verification should use cache
        let result2 = verifier.verify_header(&header, None, &commit).unwrap();
        assert_eq!(result1.header_hash, result2.header_hash);
    }

    #[test]
    fn test_performance_targets() {
        let validator_set = create_test_validator_set(10);
        let mut verifier = HeaderVerifier::new(validator_set.clone());
        
        let chain = generate_test_chain(100, &validator_set).unwrap();
        let result = verifier.verify_batch(&chain).unwrap();
        
        // Check that verification is reasonably fast
        assert!(result.average_time_per_header < Duration::from_millis(10));
        assert!(result.p50_time < Duration::from_millis(10));
    }

    #[test]
    fn test_validator_set_update() {
        let validator_set1 = create_test_validator_set(4);
        let validator_set2 = create_test_validator_set(7);
        let mut verifier = HeaderVerifier::new(validator_set1);
        
        assert_eq!(verifier.validator_set().len(), 4);
        
        verifier.update_validator_set(validator_set2);
        assert_eq!(verifier.validator_set().len(), 7);
        assert_eq!(verifier.cache_stats().0, 0); // Cache should be cleared
    }
}
