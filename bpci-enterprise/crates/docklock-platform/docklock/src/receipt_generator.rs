use crate::error::DockLockResult;
use crate::receipt::{
    Receipt, RunHeader, TraceRoots, PolicyInfo, ExecutionStats, 
    MemoryStats, IoStats, WitnessStats, EventStats, ResourceLimits, CageConfig,
    PolicyValidationResult, ValidationStatus, ComplianceStatus
};
use crate::event_stream::CanonicalEventStream;
use crate::witness_enhanced::EnhancedWitnessRecorder;
use crate::witness_cage_integration::WitnessEnabledCage;

use ed25519_dalek::SigningKey;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, warn};


/// Receipt generator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptGeneratorConfig {
    /// Automatically sign receipts
    pub auto_sign: bool,
    /// Include detailed execution statistics
    pub include_detailed_stats: bool,
    /// Include witness data validation
    pub validate_witness_data: bool,
    /// Include policy compliance checking
    pub check_policy_compliance: bool,
    /// Maximum receipt size in bytes
    pub max_receipt_size: usize,
}

/// Receipt generator for creating receipts from execution results
pub struct ReceiptGenerator {
    /// Generator configuration
    config: ReceiptGeneratorConfig,
    /// Signing key for receipt authentication
    signing_key: Option<SigningKey>,
    /// Event stream for correlation
    event_stream: Arc<RwLock<CanonicalEventStream>>,
    /// Receipt generation statistics
    stats: ReceiptGeneratorStats,
}

/// Receipt generator statistics
#[derive(Debug, Clone, Default)]
pub struct ReceiptGeneratorStats {
    /// Total receipts generated
    pub total_generated: u64,
    /// Total receipts signed
    pub total_signed: u64,
    /// Total receipts with policy violations
    pub total_violations: u64,
    /// Average receipt generation time in milliseconds
    pub avg_generation_time_ms: f64,
    /// Total generation time
    pub total_generation_time_ms: u64,
}

/// Execution context for receipt generation
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Session identifier
    pub session_id: String,
    /// Container image information
    pub image_hash: String,
    /// Command and arguments
    pub command: Vec<String>,
    /// Environment variables
    pub environment: HashMap<String, String>,
    /// Working directory
    pub working_dir: String,
    /// Resource limits
    pub resource_limits: ResourceLimits,
    /// Cage configuration
    pub cage_config: CageConfig,
    /// Execution start time
    pub start_time: SystemTime,
}

/// Execution result for receipt generation
#[derive(Debug)]
pub struct ExecutionResult {
    /// Execution context
    pub context: ExecutionContext,
    /// Execution end time
    pub end_time: SystemTime,
    /// Exit code
    pub exit_code: i32,
    /// Memory usage statistics
    pub memory_stats: MemoryStats,
    /// I/O operation statistics
    pub io_stats: IoStats,
    /// Witness recorder with captured data
    pub witness_recorder: Option<EnhancedWitnessRecorder>,
    /// Event stream state
    pub event_stream_snapshot: Option<Vec<u8>>, // Serialized event stream state
}

impl ReceiptGenerator {
    /// Create a new receipt generator
    pub fn new(
        config: ReceiptGeneratorConfig,
        event_stream: Arc<RwLock<CanonicalEventStream>>,
    ) -> Self {
        Self {
            config,
            signing_key: None,
            event_stream,
            stats: ReceiptGeneratorStats::default(),
        }
    }

    /// Set the signing key for automatic receipt signing
    pub fn set_signing_key(&mut self, signing_key: SigningKey) {
        self.signing_key = Some(signing_key);
        info!("Receipt generator configured with signing key");
    }

    /// Generate a receipt from execution results
    pub fn generate_receipt(&mut self, execution_result: ExecutionResult) -> DockLockResult<Receipt> {
        let start_time = SystemTime::now();
        
        info!(
            "Generating receipt for session: {}",
            execution_result.context.session_id
        );

        // Create run header
        let run_header = self.create_run_header(&execution_result.context)?;

        // Compute trace roots
        let trace_roots = self.compute_trace_roots(&execution_result)?;

        // Generate policy information
        let policy_info = self.generate_policy_info(&execution_result)?;

        // Create execution statistics
        let execution_stats = self.create_execution_stats(&execution_result)?;

        // Create the receipt
        let mut receipt = Receipt::new(run_header, trace_roots, policy_info, execution_stats);

        // Sign the receipt if configured
        if self.config.auto_sign {
            if let Some(ref signing_key) = self.signing_key {
                receipt.sign(signing_key)?;
                self.stats.total_signed += 1;
            } else {
                warn!("Auto-sign enabled but no signing key configured");
            }
        }

        // Update statistics
        let generation_time = start_time.elapsed()
            .unwrap_or_default()
            .as_millis() as u64;
        
        self.stats.total_generated += 1;
        self.stats.total_generation_time_ms += generation_time;
        self.stats.avg_generation_time_ms = 
            self.stats.total_generation_time_ms as f64 / self.stats.total_generated as f64;

        info!(
            "Generated receipt {} in {}ms",
            receipt.receipt_id,
            generation_time
        );

        Ok(receipt)
    }

    /// Generate a receipt from witness-enabled cage execution
    pub fn generate_from_cage_execution(
        &mut self,
        cage: &WitnessEnabledCage,
        session_id: String,
        image_hash: String,
        command: Vec<String>,
        exit_code: i32,
    ) -> DockLockResult<Receipt> {
        // Create execution context
        let context = ExecutionContext {
            session_id,
            image_hash,
            command,
            environment: HashMap::new(),
            working_dir: "/".to_string(),
            resource_limits: ResourceLimits::default(),
            cage_config: CageConfig::default(),
            start_time: SystemTime::now(),
        };

        // Get execution statistics from cage
        let cage_stats = cage.stats();
        
        let memory_stats = MemoryStats {
            peak_memory: 1024 * 1024, // 1MB placeholder
            avg_memory: 512 * 1024,   // 512KB placeholder
            allocations: cage_stats.syscalls_recorded as u64,
            deallocations: 0,
        };

        let io_stats = IoStats {
            file_reads: cage_stats.file_operations_recorded as u64,
            file_writes: cage_stats.file_operations_recorded as u64 / 2,
            network_ops: cage_stats.network_operations_recorded as u64,
            bytes_read: cage_stats.total_witness_size as u64,
            bytes_written: cage_stats.total_witness_size as u64 / 2,
        };

        // Create execution result
        let execution_result = ExecutionResult {
            context,
            end_time: SystemTime::now(),
            exit_code,
            memory_stats,
            io_stats,
            witness_recorder: None, // Would be extracted from cage in real implementation
            event_stream_snapshot: None,
        };

        self.generate_receipt(execution_result)
    }

    /// Create run header from execution context
    fn create_run_header(&self, context: &ExecutionContext) -> DockLockResult<RunHeader> {
        Ok(RunHeader {
            session_id: context.session_id.clone(),
            image_hash: context.image_hash.clone(),
            command: context.command.clone(),
            environment: context.environment.clone(),
            working_dir: context.working_dir.clone(),
            resource_limits: context.resource_limits.clone(),
            cage_config: context.cage_config.clone(),
        })
    }

    /// Compute trace roots from execution result
    fn compute_trace_roots(&self, execution_result: &ExecutionResult) -> DockLockResult<TraceRoots> {
        // Compute witness root
        let witness_root = if let Some(ref _recorder) = execution_result.witness_recorder {
            // In real implementation, we'd compute the actual Merkle root
            // For now, use a placeholder
            [1u8; 32]
        } else {
            [0u8; 32] // No witness data
        };

        // Compute event stream root
        let event_stream_root = if let Ok(stream) = self.event_stream.read() {
            if let Some(root_hash) = stream.get_merkle_root() {
                root_hash.into()
            } else {
                warn!("No event stream root available");
                [0u8; 32]
            }
        } else {
            [0u8; 32]
        };

        // Compute wallet root (placeholder for now)
        let wallet_root = [2u8; 32];

        // Compute combined root
        let combined_data = [witness_root, event_stream_root, wallet_root].concat();
        let combined_root = blake3::hash(&combined_data).into();

        Ok(TraceRoots {
            witness_root,
            event_stream_root,
            wallet_root,
            combined_root,
        })
    }

    /// Generate policy information from execution result
    fn generate_policy_info(&mut self, execution_result: &ExecutionResult) -> DockLockResult<PolicyInfo> {
        let mut validation_results = Vec::new();
        let violations = Vec::new();
        let mut compliance_status = ComplianceStatus::Compliant;

        if self.config.check_policy_compliance {
            // Example policy validations
            
            // Check resource usage policy
            if execution_result.memory_stats.peak_memory > execution_result.context.resource_limits.max_memory {
                validation_results.push(PolicyValidationResult {
                    policy_id: "memory-limit".to_string(),
                    status: ValidationStatus::Failed,
                    message: "Memory usage exceeded limit".to_string(),
                    evidence: execution_result.memory_stats.peak_memory.to_le_bytes().to_vec(),
                });
                compliance_status = ComplianceStatus::NonCompliant;
                self.stats.total_violations += 1;
            } else {
                validation_results.push(PolicyValidationResult {
                    policy_id: "memory-limit".to_string(),
                    status: ValidationStatus::Passed,
                    message: "Memory usage within limits".to_string(),
                    evidence: Vec::new(),
                });
            }

            // Check execution time policy
            let duration = execution_result.end_time
                .duration_since(execution_result.context.start_time)
                .unwrap_or_default()
                .as_millis() as u64;

            if duration > execution_result.context.resource_limits.max_cpu_time {
                validation_results.push(PolicyValidationResult {
                    policy_id: "cpu-time-limit".to_string(),
                    status: ValidationStatus::Failed,
                    message: "Execution time exceeded limit".to_string(),
                    evidence: duration.to_le_bytes().to_vec(),
                });
                compliance_status = ComplianceStatus::NonCompliant;
                self.stats.total_violations += 1;
            } else {
                validation_results.push(PolicyValidationResult {
                    policy_id: "cpu-time-limit".to_string(),
                    status: ValidationStatus::Passed,
                    message: "Execution time within limits".to_string(),
                    evidence: Vec::new(),
                });
            }

            // Check witness data completeness
            if self.config.validate_witness_data {
                let witness_complete = execution_result.witness_recorder.is_some();
                validation_results.push(PolicyValidationResult {
                    policy_id: "witness-completeness".to_string(),
                    status: if witness_complete { ValidationStatus::Passed } else { ValidationStatus::Failed },
                    message: if witness_complete { 
                        "Witness data complete".to_string() 
                    } else { 
                        "Missing witness data".to_string() 
                    },
                    evidence: Vec::new(),
                });

                if !witness_complete {
                    compliance_status = ComplianceStatus::NonCompliant;
                    self.stats.total_violations += 1;
                }
            }
        }

        Ok(PolicyInfo {
            validation_results,
            compliance_status,
            violations,
            regulatory_metadata: HashMap::new(),
        })
    }

    /// Create execution statistics from execution result
    fn create_execution_stats(&self, execution_result: &ExecutionResult) -> DockLockResult<ExecutionStats> {
        let start_time = execution_result.context.start_time
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let end_time = execution_result.end_time
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let duration_ms = execution_result.end_time
            .duration_since(execution_result.context.start_time)
            .unwrap_or_default()
            .as_millis() as u64;

        // Create witness statistics
        let witness_stats = if let Some(ref recorder) = execution_result.witness_recorder {
            let log = recorder.log();
            let stats = log.compression_stats();
            WitnessStats {
                total_entries: stats.total_entries as u64,
                total_data_size: stats.total_original_size as u64,
                compression_ratio: stats.average_compression_ratio,
                validation_passed: true, // Would validate in real implementation
            }
        } else {
            WitnessStats::default()
        };

        // Create event statistics
        let event_stats = if let Ok(stream) = self.event_stream.read() {
            let mut event_types = HashMap::new();
            for event in stream.get_recent_events(1000) {
                let kind_str = format!("{:?}", event.kind);
                *event_types.entry(kind_str).or_insert(0) += 1;
            }

            EventStats {
                total_events: stream.current_sequence(),
                event_types,
                integrity_verified: true, // Would verify in real implementation
            }
        } else {
            EventStats::default()
        };

        Ok(ExecutionStats {
            start_time,
            end_time,
            duration_ms,
            exit_code: execution_result.exit_code,
            memory_stats: execution_result.memory_stats.clone(),
            io_stats: execution_result.io_stats.clone(),
            witness_stats,
            event_stats,
        })
    }

    /// Get receipt generator statistics
    pub fn stats(&self) -> &ReceiptGeneratorStats {
        &self.stats
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = ReceiptGeneratorStats::default();
    }
}

impl Default for ReceiptGeneratorConfig {
    fn default() -> Self {
        Self {
            auto_sign: true,
            include_detailed_stats: true,
            validate_witness_data: true,
            check_policy_compliance: true,
            max_receipt_size: 1024 * 1024, // 1MB
        }
    }
}

impl ExecutionContext {
    /// Create a new execution context
    pub fn new(session_id: String, image_hash: String, command: Vec<String>) -> Self {
        Self {
            session_id,
            image_hash,
            command,
            environment: HashMap::new(),
            working_dir: "/".to_string(),
            resource_limits: ResourceLimits::default(),
            cage_config: CageConfig::default(),
            start_time: SystemTime::now(),
        }
    }

    /// Set environment variables
    pub fn with_environment(mut self, environment: HashMap<String, String>) -> Self {
        self.environment = environment;
        self
    }

    /// Set working directory
    pub fn with_working_dir(mut self, working_dir: String) -> Self {
        self.working_dir = working_dir;
        self
    }

    /// Set resource limits
    pub fn with_resource_limits(mut self, resource_limits: ResourceLimits) -> Self {
        self.resource_limits = resource_limits;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_stream::CanonicalEventStream;

    #[test]
    fn test_receipt_generator_creation() {
        let event_stream = Arc::new(RwLock::new(CanonicalEventStream::default()));
        let config = ReceiptGeneratorConfig::default();
        let generator = ReceiptGenerator::new(config, event_stream);

        assert_eq!(generator.stats.total_generated, 0);
        assert!(generator.signing_key.is_none());
    }

    #[test]
    fn test_execution_context_creation() {
        let context = ExecutionContext::new(
            "test-session".to_string(),
            "sha256:abc123".to_string(),
            vec!["echo".to_string(), "hello".to_string()],
        );

        assert_eq!(context.session_id, "test-session");
        assert_eq!(context.image_hash, "sha256:abc123");
        assert_eq!(context.command.len(), 2);
    }

    #[test]
    fn test_receipt_generation() {
        let event_stream = Arc::new(RwLock::new(CanonicalEventStream::default()));
        let config = ReceiptGeneratorConfig::default();
        let mut generator = ReceiptGenerator::new(config, event_stream);

        let context = ExecutionContext::new(
            "test-session".to_string(),
            "sha256:test123".to_string(),
            vec!["test".to_string()],
        );

        let execution_result = ExecutionResult {
            context,
            end_time: SystemTime::now(),
            exit_code: 0,
            memory_stats: MemoryStats::default(),
            io_stats: IoStats::default(),
            witness_recorder: None,
            event_stream_snapshot: None,
        };

        let receipt = generator.generate_receipt(execution_result).unwrap();

        assert!(!receipt.receipt_id.is_empty());
        assert_eq!(receipt.run_header.session_id, "test-session");
        assert_eq!(receipt.execution_stats.exit_code, 0);
        assert_eq!(generator.stats.total_generated, 1);
    }

    #[test]
    fn test_policy_validation() {
        let event_stream = Arc::new(RwLock::new(CanonicalEventStream::default()));
        let config = ReceiptGeneratorConfig {
            check_policy_compliance: true,
            ..Default::default()
        };
        let mut generator = ReceiptGenerator::new(config, event_stream);

        let context = ExecutionContext::new(
            "test-session".to_string(),
            "sha256:test123".to_string(),
            vec!["test".to_string()],
        );

        // Create execution result that exceeds memory limit
        let execution_result = ExecutionResult {
            context,
            end_time: SystemTime::now(),
            exit_code: 0,
            memory_stats: MemoryStats {
                peak_memory: 2 * 1024 * 1024 * 1024, // 2GB (exceeds default 1GB limit)
                ..Default::default()
            },
            io_stats: IoStats::default(),
            witness_recorder: None,
            event_stream_snapshot: None,
        };

        let receipt = generator.generate_receipt(execution_result).unwrap();

        // Should have policy violations
        assert_eq!(receipt.policy_info.compliance_status, ComplianceStatus::NonCompliant);
        assert!(!receipt.policy_info.validation_results.is_empty());
        assert_eq!(generator.stats.total_violations, 2); // Memory and CPU time violations
    }
}
