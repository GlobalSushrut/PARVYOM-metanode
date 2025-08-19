use crate::error::{DockLockError, DockLockResult};




use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Domain separator for receipt hashing
pub const RECEIPT_HASH: u8 = 0x15;

/// Domain separator for receipt root computation
pub const RECEIPT_ROOT_HASH: u8 = 0x16;

/// Receipt structure containing execution metadata and witness data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    /// Unique receipt identifier
    pub receipt_id: String,
    /// Execution run header with context information
    pub run_header: RunHeader,
    /// Trace roots from witness logs and event streams
    pub trace_roots: TraceRoots,
    /// Policy compliance information
    pub policy_info: PolicyInfo,
    /// Execution statistics and metadata
    pub execution_stats: ExecutionStats,
    /// Timestamp when receipt was created
    pub timestamp: u64,
    /// Receipt signature (Ed25519)
    #[serde(skip)]
    pub signature: Option<Signature>,
    /// Signer public key
    pub signer_pubkey: Option<Vec<u8>>,
}

/// Run header containing execution context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunHeader {
    /// Execution session identifier
    pub session_id: String,
    /// Container image hash or identifier
    pub image_hash: String,
    /// Execution command and arguments
    pub command: Vec<String>,
    /// Environment variables
    pub environment: HashMap<String, String>,
    /// Working directory
    pub working_dir: String,
    /// Resource limits and constraints
    pub resource_limits: ResourceLimits,
    /// Determinism cage configuration
    pub cage_config: CageConfig,
}

/// Resource limits for execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum memory usage in bytes
    pub max_memory: u64,
    /// Maximum CPU time in milliseconds
    pub max_cpu_time: u64,
    /// Maximum file system operations
    pub max_fs_ops: u64,
    /// Maximum network operations
    pub max_net_ops: u64,
}

/// Determinism cage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CageConfig {
    /// RNG seed for deterministic execution
    pub rng_seed: Vec<u8>,
    /// Syscall filter configuration
    pub syscall_filter_enabled: bool,
    /// Witness recording enabled
    pub witness_recording: bool,
    /// Event stream correlation enabled
    pub event_correlation: bool,
}

/// Trace roots from witness logs and event streams
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceRoots {
    /// Merkle root of witness log entries
    pub witness_root: [u8; 32],
    /// Merkle root of canonical event stream
    pub event_stream_root: [u8; 32],
    /// Merkle root of wallet operations
    pub wallet_root: [u8; 32],
    /// Combined trace root (Merkle of above roots)
    pub combined_root: [u8; 32],
}

/// Policy compliance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyInfo {
    /// Policy validation results
    pub validation_results: Vec<PolicyValidationResult>,
    /// Compliance status
    pub compliance_status: ComplianceStatus,
    /// Policy violations (if any)
    pub violations: Vec<PolicyViolation>,
    /// Regulatory compliance metadata
    pub regulatory_metadata: HashMap<String, String>,
}

/// Policy validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyValidationResult {
    /// Policy identifier
    pub policy_id: String,
    /// Validation status
    pub status: ValidationStatus,
    /// Validation message
    pub message: String,
    /// Evidence or proof data
    pub evidence: Vec<u8>,
}

/// Validation status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValidationStatus {
    /// Policy validation passed
    Passed,
    /// Policy validation failed
    Failed,
    /// Policy validation skipped
    Skipped,
    /// Policy validation pending
    Pending,
}

/// Compliance status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComplianceStatus {
    /// Fully compliant
    Compliant,
    /// Non-compliant with violations
    NonCompliant,
    /// Compliance pending review
    Pending,
    /// Compliance not applicable
    NotApplicable,
}

/// Policy violation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyViolation {
    /// Violation identifier
    pub violation_id: String,
    /// Violation type
    pub violation_type: String,
    /// Violation description
    pub description: String,
    /// Severity level
    pub severity: ViolationSeverity,
    /// Remediation suggestions
    pub remediation: Vec<String>,
}

/// Violation severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ViolationSeverity {
    /// Low severity violation
    Low,
    /// Medium severity violation
    Medium,
    /// High severity violation
    High,
    /// Critical severity violation
    Critical,
}

/// Execution statistics and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStats {
    /// Execution start time
    pub start_time: u64,
    /// Execution end time
    pub end_time: u64,
    /// Total execution duration in milliseconds
    pub duration_ms: u64,
    /// Exit code of execution
    pub exit_code: i32,
    /// Memory usage statistics
    pub memory_stats: MemoryStats,
    /// I/O operation statistics
    pub io_stats: IoStats,
    /// Witness recording statistics
    pub witness_stats: WitnessStats,
    /// Event stream statistics
    pub event_stats: EventStats,
}

/// Memory usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    /// Peak memory usage in bytes
    pub peak_memory: u64,
    /// Average memory usage in bytes
    pub avg_memory: u64,
    /// Memory allocations count
    pub allocations: u64,
    /// Memory deallocations count
    pub deallocations: u64,
}

/// I/O operation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoStats {
    /// File read operations count
    pub file_reads: u64,
    /// File write operations count
    pub file_writes: u64,
    /// Network operations count
    pub network_ops: u64,
    /// Total bytes read
    pub bytes_read: u64,
    /// Total bytes written
    pub bytes_written: u64,
}

/// Witness recording statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessStats {
    /// Total witness entries recorded
    pub total_entries: u64,
    /// Total witness data size in bytes
    pub total_data_size: u64,
    /// Compression ratio achieved
    pub compression_ratio: f64,
    /// Witness validation status
    pub validation_passed: bool,
}

/// Event stream statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStats {
    /// Total events recorded
    pub total_events: u64,
    /// Event types distribution
    pub event_types: HashMap<String, u64>,
    /// Event stream integrity verified
    pub integrity_verified: bool,
}

impl Receipt {
    /// Create a new receipt with the given parameters
    pub fn new(
        run_header: RunHeader,
        trace_roots: TraceRoots,
        policy_info: PolicyInfo,
        execution_stats: ExecutionStats,
    ) -> Self {
        let receipt_id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            receipt_id,
            run_header,
            trace_roots,
            policy_info,
            execution_stats,
            timestamp,
            signature: None,
            signer_pubkey: None,
        }
    }

    /// Compute the hash of this receipt for signing
    pub fn compute_hash(&self) -> DockLockResult<[u8; 32]> {
        // Create a copy without signature for hashing
        let mut receipt_for_hash = self.clone();
        receipt_for_hash.signature = None;
        receipt_for_hash.signer_pubkey = None;

        let cbor_data = serde_cbor::to_vec(&receipt_for_hash)
            .map_err(|e| DockLockError::EncodingError(format!("Failed to encode receipt: {}", e)))?;

        // Apply domain separation by prefixing with hash type
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[RECEIPT_HASH]);
        hasher.update(&cbor_data);
        Ok(hasher.finalize().into())
    }

    /// Sign this receipt with the given signing key
    pub fn sign(&mut self, signing_key: &SigningKey) -> DockLockResult<()> {
        let hash = self.compute_hash()?;
        let signature = signing_key.sign(&hash);
        
        self.signature = Some(signature);
        self.signer_pubkey = Some(signing_key.verifying_key().to_bytes().to_vec());

        info!(
            "Signed receipt {} with Ed25519 signature",
            self.receipt_id
        );

        Ok(())
    }

    /// Verify the signature of this receipt
    pub fn verify_signature(&self) -> DockLockResult<bool> {
        let signature = self.signature.as_ref()
            .ok_or_else(|| DockLockError::CryptoError("Receipt not signed".to_string()))?;

        let pubkey_bytes = self.signer_pubkey.as_ref()
            .ok_or_else(|| DockLockError::CryptoError("No signer public key".to_string()))?;

        if pubkey_bytes.len() != 32 {
            return Err(DockLockError::CryptoError("Invalid public key length".to_string()));
        }

        let mut pubkey_array = [0u8; 32];
        pubkey_array.copy_from_slice(pubkey_bytes);

        let verifying_key = VerifyingKey::from_bytes(&pubkey_array)
            .map_err(|e| DockLockError::CryptoError(format!("Invalid public key: {}", e)))?;

        let hash = self.compute_hash()?;

        match verifying_key.verify(&hash, signature) {
            Ok(()) => {
                debug!("Receipt {} signature verification passed", self.receipt_id);
                Ok(true)
            }
            Err(e) => {
                warn!("Receipt {} signature verification failed: {}", self.receipt_id, e);
                Ok(false)
            }
        }
    }

    /// Check if this receipt has been tampered with
    pub fn detect_tampering(&self) -> DockLockResult<bool> {
        if self.signature.is_none() {
            return Ok(false); // Unsigned receipts cannot be tampered
        }

        // Verify signature to detect tampering
        let signature_valid = self.verify_signature()?;
        Ok(!signature_valid)
    }

    /// Get receipt metadata for indexing and search
    pub fn metadata(&self) -> ReceiptMetadata {
        ReceiptMetadata {
            receipt_id: self.receipt_id.clone(),
            session_id: self.run_header.session_id.clone(),
            image_hash: self.run_header.image_hash.clone(),
            timestamp: self.timestamp,
            exit_code: self.execution_stats.exit_code,
            duration_ms: self.execution_stats.duration_ms,
            compliance_status: self.policy_info.compliance_status.clone(),
            signed: self.signature.is_some(),
        }
    }
}

/// Receipt metadata for indexing and search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptMetadata {
    /// Receipt identifier
    pub receipt_id: String,
    /// Execution session identifier
    pub session_id: String,
    /// Container image hash
    pub image_hash: String,
    /// Receipt timestamp
    pub timestamp: u64,
    /// Execution exit code
    pub exit_code: i32,
    /// Execution duration
    pub duration_ms: u64,
    /// Compliance status
    pub compliance_status: ComplianceStatus,
    /// Whether receipt is signed
    pub signed: bool,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory: 1024 * 1024 * 1024, // 1GB
            max_cpu_time: 300_000, // 5 minutes
            max_fs_ops: 10_000,
            max_net_ops: 1_000,
        }
    }
}

impl Default for CageConfig {
    fn default() -> Self {
        Self {
            rng_seed: vec![0u8; 32],
            syscall_filter_enabled: true,
            witness_recording: true,
            event_correlation: true,
        }
    }
}

impl Default for PolicyInfo {
    fn default() -> Self {
        Self {
            validation_results: Vec::new(),
            compliance_status: ComplianceStatus::NotApplicable,
            violations: Vec::new(),
            regulatory_metadata: HashMap::new(),
        }
    }
}

impl Default for ExecutionStats {
    fn default() -> Self {
        Self {
            start_time: 0,
            end_time: 0,
            duration_ms: 0,
            exit_code: 0,
            memory_stats: MemoryStats::default(),
            io_stats: IoStats::default(),
            witness_stats: WitnessStats::default(),
            event_stats: EventStats::default(),
        }
    }
}

impl Default for MemoryStats {
    fn default() -> Self {
        Self {
            peak_memory: 0,
            avg_memory: 0,
            allocations: 0,
            deallocations: 0,
        }
    }
}

impl Default for IoStats {
    fn default() -> Self {
        Self {
            file_reads: 0,
            file_writes: 0,
            network_ops: 0,
            bytes_read: 0,
            bytes_written: 0,
        }
    }
}

impl Default for WitnessStats {
    fn default() -> Self {
        Self {
            total_entries: 0,
            total_data_size: 0,
            compression_ratio: 1.0,
            validation_passed: false,
        }
    }
}

impl Default for EventStats {
    fn default() -> Self {
        Self {
            total_events: 0,
            event_types: HashMap::new(),
            integrity_verified: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receipt_creation() {
        let run_header = RunHeader {
            session_id: "test-session".to_string(),
            image_hash: "sha256:abc123".to_string(),
            command: vec!["echo".to_string(), "hello".to_string()],
            environment: HashMap::new(),
            working_dir: "/tmp".to_string(),
            resource_limits: ResourceLimits::default(),
            cage_config: CageConfig::default(),
        };

        let trace_roots = TraceRoots {
            witness_root: [1u8; 32],
            event_stream_root: [2u8; 32],
            wallet_root: [3u8; 32],
            combined_root: [4u8; 32],
        };

        let policy_info = PolicyInfo::default();
        let execution_stats = ExecutionStats::default();

        let receipt = Receipt::new(run_header, trace_roots, policy_info, execution_stats);

        assert!(!receipt.receipt_id.is_empty());
        assert_eq!(receipt.run_header.session_id, "test-session");
        assert_eq!(receipt.run_header.image_hash, "sha256:abc123");
        assert!(receipt.signature.is_none());
    }

    #[test]
    fn test_receipt_signing_and_verification() {
        let mut receipt = create_test_receipt();
        let signing_key = SigningKey::generate(&mut rand::thread_rng());

        // Sign the receipt
        receipt.sign(&signing_key).unwrap();
        assert!(receipt.signature.is_some());
        assert!(receipt.signer_pubkey.is_some());

        // Verify the signature
        let is_valid = receipt.verify_signature().unwrap();
        assert!(is_valid);

        // Test tampering detection
        let is_tampered = receipt.detect_tampering().unwrap();
        assert!(!is_tampered);
    }

    #[test]
    fn test_receipt_hash_computation() {
        let receipt = create_test_receipt();
        let hash1 = receipt.compute_hash().unwrap();
        let hash2 = receipt.compute_hash().unwrap();

        // Hash should be deterministic
        assert_eq!(hash1, hash2);

        // Hash should be 32 bytes
        assert_eq!(hash1.len(), 32);
    }

    #[test]
    fn test_receipt_metadata() {
        let receipt = create_test_receipt();
        let metadata = receipt.metadata();

        assert_eq!(metadata.receipt_id, receipt.receipt_id);
        assert_eq!(metadata.session_id, receipt.run_header.session_id);
        assert_eq!(metadata.image_hash, receipt.run_header.image_hash);
        assert!(!metadata.signed);
    }

    fn create_test_receipt() -> Receipt {
        let run_header = RunHeader {
            session_id: "test-session".to_string(),
            image_hash: "sha256:test123".to_string(),
            command: vec!["test".to_string()],
            environment: HashMap::new(),
            working_dir: "/test".to_string(),
            resource_limits: ResourceLimits::default(),
            cage_config: CageConfig::default(),
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
