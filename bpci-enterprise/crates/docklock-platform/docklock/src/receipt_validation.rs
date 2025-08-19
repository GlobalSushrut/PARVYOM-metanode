use crate::error::{DockLockError, DockLockResult};
use crate::receipt::{Receipt, ComplianceStatus};
// Domain separation implemented manually
use bpi_merkle::MerkleTree;
use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use tracing::{debug, info, warn};

/// Domain separator for receipts root computation
pub const RECEIPTS_ROOT_HASH: u8 = 0x16;

/// Receipt validator for signature verification and integrity checking
pub struct ReceiptValidator {
    /// Validation configuration
    config: ReceiptValidatorConfig,
    /// Trusted public keys for signature verification
    trusted_keys: HashMap<String, VerifyingKey>,
    /// Validation statistics
    stats: ReceiptValidatorStats,
}

/// Receipt validator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptValidatorConfig {
    /// Require signatures for all receipts
    pub require_signatures: bool,
    /// Validate policy compliance
    pub validate_policy_compliance: bool,
    /// Validate witness data integrity
    pub validate_witness_integrity: bool,
    /// Validate event stream correlation
    pub validate_event_correlation: bool,
    /// Maximum allowed receipt age in seconds
    pub max_receipt_age_seconds: u64,
    /// Strict validation mode
    pub strict_mode: bool,
}

/// Receipt validation statistics
#[derive(Debug, Clone, Default)]
pub struct ReceiptValidatorStats {
    /// Total receipts validated
    pub total_validated: u64,
    /// Total valid receipts
    pub total_valid: u64,
    /// Total invalid receipts
    pub total_invalid: u64,
    /// Total signature verification failures
    pub signature_failures: u64,
    /// Total policy compliance failures
    pub policy_failures: u64,
    /// Total witness integrity failures
    pub witness_failures: u64,
    /// Average validation time in milliseconds
    pub avg_validation_time_ms: f64,
}

/// Receipt validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptValidationResult {
    /// Overall validation status
    pub is_valid: bool,
    /// Receipt identifier
    pub receipt_id: String,
    /// Validation errors
    pub errors: Vec<ValidationError>,
    /// Validation warnings
    pub warnings: Vec<ValidationWarning>,
    /// Signature verification result
    pub signature_valid: bool,
    /// Policy compliance result
    pub policy_compliant: bool,
    /// Witness integrity result
    pub witness_integrity_valid: bool,
    /// Event correlation result
    pub event_correlation_valid: bool,
    /// Validation timestamp
    pub validation_timestamp: u64,
}

/// Validation error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Error severity
    pub severity: ErrorSeverity,
    /// Additional context
    pub context: HashMap<String, String>,
}

/// Validation warning information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    /// Warning code
    pub code: String,
    /// Warning message
    pub message: String,
    /// Additional context
    pub context: HashMap<String, String>,
}

/// Error severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Low severity error
    Low,
    /// Medium severity error
    Medium,
    /// High severity error
    High,
    /// Critical severity error
    Critical,
}

/// Receipt Merkle tree for computing receipts_root
pub struct ReceiptMerkleTree {
    /// Merkle tree instance
    tree: Option<MerkleTree>,
    /// Receipt hashes
    receipt_hashes: Vec<[u8; 32]>,
    /// Configuration
    config: ReceiptMerkleConfig,
}

/// Receipt Merkle tree configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptMerkleConfig {
    /// Maximum number of receipts per tree
    pub max_receipts: usize,
    /// Enable incremental updates
    pub incremental_updates: bool,
    /// Validate tree integrity on updates
    pub validate_on_update: bool,
}

impl ReceiptValidator {
    /// Create a new receipt validator
    pub fn new(config: ReceiptValidatorConfig) -> Self {
        Self {
            config,
            trusted_keys: HashMap::new(),
            stats: ReceiptValidatorStats::default(),
        }
    }

    /// Add a trusted public key for signature verification
    pub fn add_trusted_key(&mut self, key_id: String, public_key: VerifyingKey) {
        info!("Added trusted key: {}", key_id);
        self.trusted_keys.insert(key_id, public_key);
    }

    /// Remove a trusted public key
    pub fn remove_trusted_key(&mut self, key_id: &str) -> bool {
        self.trusted_keys.remove(key_id).is_some()
    }

    /// Validate a receipt
    pub fn validate_receipt(&mut self, receipt: &Receipt) -> DockLockResult<ReceiptValidationResult> {
        let start_time = std::time::SystemTime::now();
        
        debug!("Validating receipt: {}", receipt.receipt_id);

        let mut result = ReceiptValidationResult {
            is_valid: true,
            receipt_id: receipt.receipt_id.clone(),
            errors: Vec::new(),
            warnings: Vec::new(),
            signature_valid: false,
            policy_compliant: false,
            witness_integrity_valid: false,
            event_correlation_valid: false,
            validation_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };

        // Validate signature
        if self.config.require_signatures || receipt.signature.is_some() {
            result.signature_valid = self.validate_signature(receipt, &mut result)?;
        } else {
            result.signature_valid = true; // Not required
        }

        // Validate policy compliance
        if self.config.validate_policy_compliance {
            result.policy_compliant = self.validate_policy_compliance(receipt, &mut result)?;
        } else {
            result.policy_compliant = true; // Not required
        }

        // Validate witness integrity
        if self.config.validate_witness_integrity {
            result.witness_integrity_valid = self.validate_witness_integrity(receipt, &mut result)?;
        } else {
            result.witness_integrity_valid = true; // Not required
        }

        // Validate event correlation
        if self.config.validate_event_correlation {
            result.event_correlation_valid = self.validate_event_correlation(receipt, &mut result)?;
        } else {
            result.event_correlation_valid = true; // Not required
        }

        // Validate receipt age
        self.validate_receipt_age(receipt, &mut result)?;

        // Determine overall validity
        result.is_valid = result.signature_valid 
            && result.policy_compliant 
            && result.witness_integrity_valid 
            && result.event_correlation_valid
            && result.errors.iter().all(|e| e.severity != ErrorSeverity::Critical);

        // Update statistics
        self.update_validation_stats(&result, start_time);

        if result.is_valid {
            info!("Receipt {} validation passed", receipt.receipt_id);
        } else {
            warn!("Receipt {} validation failed with {} errors", 
                  receipt.receipt_id, result.errors.len());
        }

        Ok(result)
    }

    /// Validate receipt signature
    fn validate_signature(&mut self, receipt: &Receipt, result: &mut ReceiptValidationResult) -> DockLockResult<bool> {
        if receipt.signature.is_none() {
            if self.config.require_signatures {
                result.errors.push(ValidationError {
                    code: "MISSING_SIGNATURE".to_string(),
                    message: "Receipt signature is required but missing".to_string(),
                    severity: ErrorSeverity::Critical,
                    context: HashMap::new(),
                });
                self.stats.signature_failures += 1;
                return Ok(false);
            } else {
                return Ok(true);
            }
        }

        match receipt.verify_signature() {
            Ok(true) => {
                debug!("Receipt {} signature verification passed", receipt.receipt_id);
                Ok(true)
            }
            Ok(false) => {
                result.errors.push(ValidationError {
                    code: "INVALID_SIGNATURE".to_string(),
                    message: "Receipt signature verification failed".to_string(),
                    severity: ErrorSeverity::Critical,
                    context: HashMap::new(),
                });
                self.stats.signature_failures += 1;
                Ok(false)
            }
            Err(e) => {
                result.errors.push(ValidationError {
                    code: "SIGNATURE_ERROR".to_string(),
                    message: format!("Signature verification error: {}", e),
                    severity: ErrorSeverity::High,
                    context: HashMap::new(),
                });
                self.stats.signature_failures += 1;
                Ok(false)
            }
        }
    }

    /// Validate policy compliance
    fn validate_policy_compliance(&mut self, receipt: &Receipt, result: &mut ReceiptValidationResult) -> DockLockResult<bool> {
        let policy_info = &receipt.policy_info;

        // Check overall compliance status
        match policy_info.compliance_status {
            ComplianceStatus::Compliant => {
                debug!("Receipt {} policy compliance: compliant", receipt.receipt_id);
                Ok(true)
            }
            ComplianceStatus::NonCompliant => {
                result.errors.push(ValidationError {
                    code: "POLICY_NON_COMPLIANT".to_string(),
                    message: "Receipt indicates non-compliant execution".to_string(),
                    severity: ErrorSeverity::High,
                    context: HashMap::new(),
                });
                self.stats.policy_failures += 1;
                Ok(false)
            }
            ComplianceStatus::Pending => {
                result.warnings.push(ValidationWarning {
                    code: "POLICY_PENDING".to_string(),
                    message: "Policy compliance is pending review".to_string(),
                    context: HashMap::new(),
                });
                Ok(true) // Allow pending in non-strict mode
            }
            ComplianceStatus::NotApplicable => {
                Ok(true) // No policy requirements
            }
        }
    }

    /// Validate witness integrity
    fn validate_witness_integrity(&mut self, receipt: &Receipt, result: &mut ReceiptValidationResult) -> DockLockResult<bool> {
        let witness_stats = &receipt.execution_stats.witness_stats;

        // Check if witness validation passed
        if !witness_stats.validation_passed {
            result.errors.push(ValidationError {
                code: "WITNESS_VALIDATION_FAILED".to_string(),
                message: "Witness data validation failed".to_string(),
                severity: ErrorSeverity::High,
                context: HashMap::new(),
            });
            self.stats.witness_failures += 1;
            return Ok(false);
        }

        // Check witness data completeness
        if witness_stats.total_entries == 0 && self.config.strict_mode {
            result.warnings.push(ValidationWarning {
                code: "NO_WITNESS_DATA".to_string(),
                message: "No witness data recorded".to_string(),
                context: HashMap::new(),
            });
        }

        // Validate trace roots (simplified check)
        let trace_roots = &receipt.trace_roots;
        if trace_roots.witness_root == [0u8; 32] && witness_stats.total_entries > 0 {
            result.errors.push(ValidationError {
                code: "INVALID_WITNESS_ROOT".to_string(),
                message: "Witness root is zero but witness data exists".to_string(),
                severity: ErrorSeverity::Medium,
                context: HashMap::new(),
            });
            return Ok(false);
        }

        Ok(true)
    }

    /// Validate event correlation
    fn validate_event_correlation(&mut self, receipt: &Receipt, result: &mut ReceiptValidationResult) -> DockLockResult<bool> {
        let event_stats = &receipt.execution_stats.event_stats;

        // Check event stream integrity
        if !event_stats.integrity_verified {
            result.errors.push(ValidationError {
                code: "EVENT_INTEGRITY_FAILED".to_string(),
                message: "Event stream integrity verification failed".to_string(),
                severity: ErrorSeverity::High,
                context: HashMap::new(),
            });
            return Ok(false);
        }

        // Check event correlation (simplified)
        if event_stats.total_events == 0 && self.config.strict_mode {
            result.warnings.push(ValidationWarning {
                code: "NO_EVENT_DATA".to_string(),
                message: "No event data recorded".to_string(),
                context: HashMap::new(),
            });
        }

        Ok(true)
    }

    /// Validate receipt age
    fn validate_receipt_age(&self, receipt: &Receipt, result: &mut ReceiptValidationResult) -> DockLockResult<()> {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let receipt_age = current_time.saturating_sub(receipt.timestamp);

        if receipt_age > self.config.max_receipt_age_seconds {
            result.errors.push(ValidationError {
                code: "RECEIPT_TOO_OLD".to_string(),
                message: format!("Receipt is {} seconds old, maximum allowed is {}", 
                                receipt_age, self.config.max_receipt_age_seconds),
                severity: ErrorSeverity::Medium,
                context: {
                    let mut ctx = HashMap::new();
                    ctx.insert("receipt_age".to_string(), receipt_age.to_string());
                    ctx.insert("max_age".to_string(), self.config.max_receipt_age_seconds.to_string());
                    ctx
                },
            });
        }

        Ok(())
    }

    /// Update validation statistics
    fn update_validation_stats(&mut self, result: &ReceiptValidationResult, start_time: std::time::SystemTime) {
        self.stats.total_validated += 1;
        
        if result.is_valid {
            self.stats.total_valid += 1;
        } else {
            self.stats.total_invalid += 1;
        }

        let validation_time = start_time.elapsed()
            .unwrap_or_default()
            .as_millis() as f64;

        self.stats.avg_validation_time_ms = 
            (self.stats.avg_validation_time_ms * (self.stats.total_validated - 1) as f64 + validation_time) 
            / self.stats.total_validated as f64;
    }

    /// Get validation statistics
    pub fn stats(&self) -> &ReceiptValidatorStats {
        &self.stats
    }

    /// Reset validation statistics
    pub fn reset_stats(&mut self) {
        self.stats = ReceiptValidatorStats::default();
    }
}

impl ReceiptMerkleTree {
    /// Create a new receipt Merkle tree
    pub fn new(config: ReceiptMerkleConfig) -> Self {
        Self {
            tree: None,
            receipt_hashes: Vec::new(),
            config,
        }
    }

    /// Add a receipt to the Merkle tree
    pub fn add_receipt(&mut self, receipt: &Receipt) -> DockLockResult<()> {
        if self.receipt_hashes.len() >= self.config.max_receipts {
            return Err(DockLockError::InvalidOperation(
                "Maximum number of receipts reached".to_string()
            ));
        }

        let receipt_hash = receipt.compute_hash()?;
        self.receipt_hashes.push(receipt_hash);

        if self.config.incremental_updates {
            self.rebuild_tree()?;
        }

        debug!("Added receipt {} to Merkle tree", receipt.receipt_id);
        Ok(())
    }

    /// Compute the receipts root hash
    pub fn compute_receipts_root(&mut self) -> DockLockResult<[u8; 32]> {
        if self.receipt_hashes.is_empty() {
            return Ok([0u8; 32]); // Empty root
        }

        if self.tree.is_none() || !self.config.incremental_updates {
            self.rebuild_tree()?;
        }

        let tree = self.tree.as_ref()
            .ok_or_else(|| DockLockError::MerkleError("No Merkle tree available".to_string()))?;

        let root = tree.root()
            .map_err(|e| DockLockError::MerkleError(format!("Failed to get Merkle root: {}", e)))?;

        // Convert to fixed-size array
        let root_bytes: [u8; 32] = root.try_into()
            .map_err(|_| DockLockError::MerkleError("Invalid root size".to_string()))?;

        // Apply domain separation by prefixing with hash type
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[RECEIPTS_ROOT_HASH]);
        hasher.update(&root_bytes);
        Ok(hasher.finalize().into())
    }

    /// Rebuild the Merkle tree
    fn rebuild_tree(&mut self) -> DockLockResult<()> {
        if self.receipt_hashes.is_empty() {
            self.tree = None;
            return Ok(());
        }

        let hash_vecs: Vec<Vec<u8>> = self.receipt_hashes
            .iter()
            .map(|h| h.to_vec())
            .collect();

        let tree = MerkleTree::new(hash_vecs)
            .map_err(|e| DockLockError::MerkleError(format!("Failed to create Merkle tree: {}", e)))?;

        self.tree = Some(tree);

        if self.config.validate_on_update {
            // Validate tree integrity
            let _root = self.tree.as_ref().unwrap().root()
                .map_err(|e| DockLockError::MerkleError(format!("Tree validation failed: {}", e)))?;
        }

        debug!("Rebuilt Merkle tree with {} receipts", self.receipt_hashes.len());
        Ok(())
    }

    /// Get the number of receipts in the tree
    pub fn receipt_count(&self) -> usize {
        self.receipt_hashes.len()
    }

    /// Clear all receipts from the tree
    pub fn clear(&mut self) {
        self.receipt_hashes.clear();
        self.tree = None;
        debug!("Cleared receipt Merkle tree");
    }
}

impl Default for ReceiptValidatorConfig {
    fn default() -> Self {
        Self {
            require_signatures: true,
            validate_policy_compliance: true,
            validate_witness_integrity: true,
            validate_event_correlation: true,
            max_receipt_age_seconds: 24 * 60 * 60, // 24 hours
            strict_mode: false,
        }
    }
}

impl Default for ReceiptMerkleConfig {
    fn default() -> Self {
        Self {
            max_receipts: 10_000,
            incremental_updates: true,
            validate_on_update: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::receipt::{Receipt, RunHeader, TraceRoots, PolicyInfo, ExecutionStats};
    use ed25519_dalek::SigningKey;
    use std::collections::HashMap;

    #[test]
    fn test_receipt_validator_creation() {
        let config = ReceiptValidatorConfig::default();
        let validator = ReceiptValidator::new(config);

        assert_eq!(validator.stats.total_validated, 0);
        assert!(validator.trusted_keys.is_empty());
    }

    #[test]
    fn test_receipt_validation_unsigned() {
        let config = ReceiptValidatorConfig {
            require_signatures: false,
            validate_policy_compliance: false,
            validate_witness_integrity: false,
            validate_event_correlation: false,
            max_receipt_age_seconds: 86400, // 24 hours
            ..Default::default()
        };
        let mut validator = ReceiptValidator::new(config);
        let receipt = create_test_receipt();

        let result = validator.validate_receipt(&receipt).unwrap();
        assert!(result.is_valid);
        assert!(result.signature_valid);
    }

    #[test]
    fn test_receipt_validation_signed() {
        let config = ReceiptValidatorConfig {
            require_signatures: true,
            validate_policy_compliance: false,
            validate_witness_integrity: false,
            validate_event_correlation: false,
            max_receipt_age_seconds: 86400, // 24 hours
            ..Default::default()
        };
        let mut validator = ReceiptValidator::new(config);
        let mut receipt = create_test_receipt();
        let signing_key = SigningKey::generate(&mut rand::thread_rng());

        receipt.sign(&signing_key).unwrap();

        let result = validator.validate_receipt(&receipt).unwrap();
        assert!(result.is_valid);
        assert!(result.signature_valid);
    }

    #[test]
    fn test_receipt_merkle_tree() {
        let config = ReceiptMerkleConfig::default();
        let mut tree = ReceiptMerkleTree::new(config);

        assert_eq!(tree.receipt_count(), 0);

        let receipt1 = create_test_receipt();
        let receipt2 = create_test_receipt();

        tree.add_receipt(&receipt1).unwrap();
        tree.add_receipt(&receipt2).unwrap();

        assert_eq!(tree.receipt_count(), 2);

        let root = tree.compute_receipts_root().unwrap();
        assert_ne!(root, [0u8; 32]);
    }

    fn create_test_receipt() -> Receipt {
        let run_header = RunHeader {
            session_id: "test-session".to_string(),
            image_hash: "sha256:test123".to_string(),
            command: vec!["test".to_string()],
            environment: HashMap::new(),
            working_dir: "/test".to_string(),
            resource_limits: crate::receipt::ResourceLimits::default(),
            cage_config: crate::receipt::CageConfig::default(),
        };

        let trace_roots = TraceRoots {
            witness_root: [1u8; 32],
            event_stream_root: [2u8; 32],
            wallet_root: [3u8; 32],
            combined_root: [4u8; 32],
        };

        Receipt::new(run_header, trace_roots, PolicyInfo::default(), ExecutionStats::default())
    }
}
