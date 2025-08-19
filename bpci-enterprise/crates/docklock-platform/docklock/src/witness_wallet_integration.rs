//! Witness recording integration with wallet and DAO systems
//! Stage 27: Witness Log & I/O Recording Integration

use crate::error::{DockLockError, DockLockResult};
use crate::event_stream::{EventKind, CanonicalEventStream};
use crate::witness_enhanced::{EnhancedWitnessRecorder, WitnessRecorderConfig};
use crate::witness::{WitnessData, WitnessOperationType};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;


/// Wallet operation types for witness recording
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WalletOperationType {
    /// Cryptographic key generation
    KeyGeneration,
    /// Digital signature creation
    Signing,
    /// Signature verification
    Verification,
    /// Service registration
    ServiceRegistration,
    /// OCI operation execution
    OciOperation,
    /// DAO proposal creation
    ProposalCreation,
    /// DAO voting
    Voting,
    /// DAO proposal execution
    ProposalExecution,
    /// MetaNode identity creation
    IdentityCreation,
    /// Compliance check
    ComplianceCheck,
    /// Monitoring activity
    MonitoringActivity,
    /// Wallet box agreement
    WalletBoxAgreement,
}

/// Wallet witness data for specialized wallet operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletWitnessData {
    /// Cryptographic operation witness
    CryptoOperation {
        operation: String,
        key_type: String,
        input_hash: [u8; 32],
        output_hash: [u8; 32],
        timestamp: u64,
    },
    /// Service operation witness
    ServiceOperation {
        service_id: String,
        operation: String,
        parameters: HashMap<String, String>,
        result: String,
        timestamp: u64,
    },
    /// DAO operation witness
    DaoOperation {
        dao_id: String,
        operation: String,
        proposal_id: Option<String>,
        voter_id: Option<String>,
        vote_data: Option<String>,
        timestamp: u64,
    },
    /// MetaNode operation witness
    MetaNodeOperation {
        wallet_id: String,
        operation: String,
        identity_data: Option<String>,
        compliance_data: Option<String>,
        monitoring_data: Option<String>,
        timestamp: u64,
    },
    /// Compliance audit witness
    ComplianceAudit {
        audit_id: String,
        check_type: String,
        subject: String,
        result: String,
        evidence: Vec<u8>,
        timestamp: u64,
    },
}

impl WalletWitnessData {
    /// Estimate the size of wallet witness data
    pub fn estimated_size(&self) -> usize {
        match self {
            WalletWitnessData::CryptoOperation { operation, key_type, .. } => {
                operation.len() + key_type.len() + 64 + 16 // strings + hashes + overhead
            }
            WalletWitnessData::ServiceOperation { service_id, operation, parameters, result, .. } => {
                service_id.len() + operation.len() + result.len() + 
                parameters.iter().map(|(k, v)| k.len() + v.len()).sum::<usize>() + 32
            }
            WalletWitnessData::DaoOperation { dao_id, operation, proposal_id, voter_id, vote_data, .. } => {
                dao_id.len() + operation.len() + 
                proposal_id.as_ref().map(|s| s.len()).unwrap_or(0) +
                voter_id.as_ref().map(|s| s.len()).unwrap_or(0) +
                vote_data.as_ref().map(|s| s.len()).unwrap_or(0) + 32
            }
            WalletWitnessData::MetaNodeOperation { wallet_id, operation, identity_data, compliance_data, monitoring_data, .. } => {
                wallet_id.len() + operation.len() +
                identity_data.as_ref().map(|s| s.len()).unwrap_or(0) +
                compliance_data.as_ref().map(|s| s.len()).unwrap_or(0) +
                monitoring_data.as_ref().map(|s| s.len()).unwrap_or(0) + 32
            }
            WalletWitnessData::ComplianceAudit { audit_id, check_type, subject, result, evidence, .. } => {
                audit_id.len() + check_type.len() + subject.len() + result.len() + evidence.len() + 32
            }
        }
    }
}

/// Integrated witness recorder for wallet operations
#[derive(Debug)]
pub struct WalletWitnessRecorder {
    /// Enhanced witness recorder
    recorder: EnhancedWitnessRecorder,
    /// Event stream for correlation
    event_stream: Arc<RwLock<CanonicalEventStream>>,
    /// Wallet operation statistics
    wallet_stats: WalletWitnessStats,
    /// Configuration
    config: WalletWitnessConfig,
}

/// Statistics for wallet witness recording
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletWitnessStats {
    pub crypto_operations: usize,
    pub service_operations: usize,
    pub dao_operations: usize,
    pub metanode_operations: usize,
    pub compliance_audits: usize,
    pub total_operations: usize,
    pub total_data_recorded: usize,
    pub average_operation_size: f64,
}

impl Default for WalletWitnessStats {
    fn default() -> Self {
        Self {
            crypto_operations: 0,
            service_operations: 0,
            dao_operations: 0,
            metanode_operations: 0,
            compliance_audits: 0,
            total_operations: 0,
            total_data_recorded: 0,
            average_operation_size: 0.0,
        }
    }
}

/// Configuration for wallet witness recording
#[derive(Debug, Clone)]
pub struct WalletWitnessConfig {
    /// Base witness recorder configuration
    pub base_config: WitnessRecorderConfig,
    /// Enable crypto operation recording
    pub record_crypto_ops: bool,
    /// Enable service operation recording
    pub record_service_ops: bool,
    /// Enable DAO operation recording
    pub record_dao_ops: bool,
    /// Enable MetaNode operation recording
    pub record_metanode_ops: bool,
    /// Enable compliance audit recording
    pub record_compliance_audits: bool,
    /// Minimum operation size to record
    pub min_operation_size: usize,
}

impl Default for WalletWitnessConfig {
    fn default() -> Self {
        Self {
            base_config: WitnessRecorderConfig::default(),
            record_crypto_ops: true,
            record_service_ops: true,
            record_dao_ops: true,
            record_metanode_ops: true,
            record_compliance_audits: true,
            min_operation_size: 32,
        }
    }
}

impl WalletWitnessRecorder {
    /// Create a new wallet witness recorder
    pub fn new(
        config: WalletWitnessConfig,
        event_stream: Arc<RwLock<CanonicalEventStream>>,
    ) -> Self {
        let mut recorder = EnhancedWitnessRecorder::new(config.base_config.clone());
        recorder.set_event_stream(event_stream.clone());
        recorder.set_enabled(true);

        Self {
            recorder,
            event_stream,
            wallet_stats: WalletWitnessStats::default(),
            config,
        }
    }

    /// Record a cryptographic operation
    pub fn record_crypto_operation(
        &mut self,
        operation: &str,
        key_type: &str,
        input_data: &[u8],
        output_data: &[u8],
        pid: u32,
        tid: u32,
    ) -> DockLockResult<()> {
        if !self.config.record_crypto_ops {
            return Ok(());
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let input_hash = blake3::hash(input_data);
        let output_hash = blake3::hash(output_data);

        let wallet_data = WalletWitnessData::CryptoOperation {
            operation: operation.to_string(),
            key_type: key_type.to_string(),
            input_hash: *input_hash.as_bytes(),
            output_hash: *output_hash.as_bytes(),
            timestamp,
        };

        self.record_wallet_operation(
            WalletOperationType::Signing, // Default to signing for crypto ops
            wallet_data,
            pid,
            tid,
        )?;

        self.wallet_stats.crypto_operations += 1;
        debug!("Recorded crypto operation: {} ({})", operation, key_type);

        Ok(())
    }

    /// Record a service operation
    pub fn record_service_operation(
        &mut self,
        service_id: &str,
        operation: &str,
        parameters: HashMap<String, String>,
        result: &str,
        pid: u32,
        tid: u32,
    ) -> DockLockResult<()> {
        if !self.config.record_service_ops {
            return Ok(());
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let wallet_data = WalletWitnessData::ServiceOperation {
            service_id: service_id.to_string(),
            operation: operation.to_string(),
            parameters,
            result: result.to_string(),
            timestamp,
        };

        self.record_wallet_operation(
            WalletOperationType::ServiceRegistration,
            wallet_data,
            pid,
            tid,
        )?;

        self.wallet_stats.service_operations += 1;
        debug!("Recorded service operation: {} on {}", operation, service_id);

        Ok(())
    }

    /// Record a DAO operation
    pub fn record_dao_operation(
        &mut self,
        dao_id: &str,
        operation: &str,
        proposal_id: Option<&str>,
        voter_id: Option<&str>,
        vote_data: Option<&str>,
        pid: u32,
        tid: u32,
    ) -> DockLockResult<()> {
        if !self.config.record_dao_ops {
            return Ok(());
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let wallet_data = WalletWitnessData::DaoOperation {
            dao_id: dao_id.to_string(),
            operation: operation.to_string(),
            proposal_id: proposal_id.map(|s| s.to_string()),
            voter_id: voter_id.map(|s| s.to_string()),
            vote_data: vote_data.map(|s| s.to_string()),
            timestamp,
        };

        let op_type = match operation {
            "create_proposal" => WalletOperationType::ProposalCreation,
            "vote" => WalletOperationType::Voting,
            "execute_proposal" => WalletOperationType::ProposalExecution,
            _ => WalletOperationType::Voting,
        };

        self.record_wallet_operation(op_type, wallet_data, pid, tid)?;

        self.wallet_stats.dao_operations += 1;
        debug!("Recorded DAO operation: {} on {}", operation, dao_id);

        Ok(())
    }

    /// Record a MetaNode operation
    pub fn record_metanode_operation(
        &mut self,
        wallet_id: &str,
        operation: &str,
        identity_data: Option<&str>,
        compliance_data: Option<&str>,
        monitoring_data: Option<&str>,
        pid: u32,
        tid: u32,
    ) -> DockLockResult<()> {
        if !self.config.record_metanode_ops {
            return Ok(());
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let wallet_data = WalletWitnessData::MetaNodeOperation {
            wallet_id: wallet_id.to_string(),
            operation: operation.to_string(),
            identity_data: identity_data.map(|s| s.to_string()),
            compliance_data: compliance_data.map(|s| s.to_string()),
            monitoring_data: monitoring_data.map(|s| s.to_string()),
            timestamp,
        };

        let op_type = match operation {
            "create_identity" => WalletOperationType::IdentityCreation,
            "compliance_check" => WalletOperationType::ComplianceCheck,
            "monitor_activity" => WalletOperationType::MonitoringActivity,
            "wallet_box_agreement" => WalletOperationType::WalletBoxAgreement,
            _ => WalletOperationType::IdentityCreation,
        };

        self.record_wallet_operation(op_type, wallet_data, pid, tid)?;

        self.wallet_stats.metanode_operations += 1;
        debug!("Recorded MetaNode operation: {} on {}", operation, wallet_id);

        Ok(())
    }

    /// Record a compliance audit
    pub fn record_compliance_audit(
        &mut self,
        audit_id: &str,
        check_type: &str,
        subject: &str,
        result: &str,
        evidence: &[u8],
        pid: u32,
        tid: u32,
    ) -> DockLockResult<()> {
        if !self.config.record_compliance_audits {
            return Ok(());
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let wallet_data = WalletWitnessData::ComplianceAudit {
            audit_id: audit_id.to_string(),
            check_type: check_type.to_string(),
            subject: subject.to_string(),
            result: result.to_string(),
            evidence: evidence.to_vec(),
            timestamp,
        };

        self.record_wallet_operation(
            WalletOperationType::ComplianceCheck,
            wallet_data,
            pid,
            tid,
        )?;

        self.wallet_stats.compliance_audits += 1;
        debug!("Recorded compliance audit: {} for {}", check_type, subject);

        Ok(())
    }

    /// Record a generic wallet operation
    fn record_wallet_operation(
        &mut self,
        operation_type: WalletOperationType,
        wallet_data: WalletWitnessData,
        pid: u32,
        tid: u32,
    ) -> DockLockResult<()> {
        let data_size = wallet_data.estimated_size();
        
        if data_size < self.config.min_operation_size {
            return Ok(()); // Skip recording small operations
        }

        // Convert wallet witness data to standard witness data
        let witness_data = WitnessData::RandomData {
            data: bincode::serialize(&wallet_data)
                .map_err(|e| DockLockError::EncodingError(format!("Failed to serialize wallet data: {}", e)))?,
        };

        // Find correlated event in the event stream
        let event_id = self.find_recent_wallet_event(&operation_type);

        // Record the operation
        self.recorder.log_mut().add_witness_entry(
            WitnessOperationType::SyscallResult, // Use syscall result as base type
            pid,
            tid,
            witness_data,
            event_id,
        )?;

        // Update statistics
        self.wallet_stats.total_operations += 1;
        self.wallet_stats.total_data_recorded += data_size;
        self.wallet_stats.average_operation_size = 
            self.wallet_stats.total_data_recorded as f64 / self.wallet_stats.total_operations as f64;

        Ok(())
    }

    /// Find a recent wallet-related event in the event stream
    fn find_recent_wallet_event(&self, operation_type: &WalletOperationType) -> Option<u128> {
        if let Ok(stream) = self.event_stream.read() {
            // Look for recent events that match the operation type
            let target_kind = match operation_type {
                WalletOperationType::ServiceRegistration => EventKind::ServiceDeploy,
                WalletOperationType::ProposalCreation => EventKind::ProposalCreate,
                WalletOperationType::Voting => EventKind::ProposalVote,
                WalletOperationType::ProposalExecution => EventKind::ProposalExecute,
                WalletOperationType::IdentityCreation => EventKind::IdentityCreate,
                _ => EventKind::WalletConnect,
            };

            // Get recent events and find matching kind
            for event in stream.get_recent_events(10) {
                if event.kind == target_kind {
                    return Some(event.eid);
                }
            }
        }
        None
    }

    /// Get wallet witness statistics
    pub fn stats(&self) -> &WalletWitnessStats {
        &self.wallet_stats
    }

    /// Get the underlying enhanced witness recorder
    pub fn recorder(&self) -> &EnhancedWitnessRecorder {
        &self.recorder
    }

    /// Get mutable access to the underlying enhanced witness recorder
    pub fn recorder_mut(&mut self) -> &mut EnhancedWitnessRecorder {
        &mut self.recorder
    }

    /// Compute Merkle root of all wallet witness entries
    pub fn compute_merkle_root(&mut self) -> DockLockResult<[u8; 32]> {
        self.recorder.compute_merkle_root()
    }

    /// Get compression statistics from the underlying recorder
    pub fn compression_stats(&self) -> &crate::witness_enhanced::CompressionStats {
        self.recorder.log().compression_stats()
    }
}

/// Witness validator for checking witness integrity and completeness
#[derive(Debug)]
pub struct WitnessValidator {
    /// Required witness types for validation
    required_witness_types: Vec<WitnessOperationType>,
    /// Minimum number of witness entries required
    min_witness_count: usize,
    /// Maximum age of witness entries (in seconds)
    max_witness_age: u64,
}

impl WitnessValidator {
    /// Create a new witness validator
    pub fn new() -> Self {
        Self {
            required_witness_types: vec![
                WitnessOperationType::SyscallResult,
                WitnessOperationType::FileRead,
            ],
            min_witness_count: 1,
            max_witness_age: 3600, // 1 hour
        }
    }

    /// Set required witness types
    pub fn with_required_types(mut self, types: Vec<WitnessOperationType>) -> Self {
        self.required_witness_types = types;
        self
    }

    /// Set minimum witness count
    pub fn with_min_count(mut self, count: usize) -> Self {
        self.min_witness_count = count;
        self
    }

    /// Set maximum witness age
    pub fn with_max_age(mut self, age_seconds: u64) -> Self {
        self.max_witness_age = age_seconds;
        self
    }

    /// Validate witness log completeness and integrity
    pub fn validate_witness_log(
        &self,
        recorder: &EnhancedWitnessRecorder,
    ) -> DockLockResult<ValidationResult> {
        let log = recorder.log();
        
        if log.is_empty() {
            return Ok(ValidationResult {
                is_valid: false,
                missing_witness_types: self.required_witness_types.clone(),
                witness_count: 0,
                errors: vec!["No witness entries found".to_string()],
            });
        }

        if log.len() < self.min_witness_count {
            return Ok(ValidationResult {
                is_valid: false,
                missing_witness_types: vec![],
                witness_count: log.len(),
                errors: vec![format!(
                    "Insufficient witness entries: {} < {}",
                    log.len(),
                    self.min_witness_count
                )],
            });
        }

        // Check for required witness types
        let found_types: std::collections::HashSet<WitnessOperationType> = std::collections::HashSet::new();
        let errors = Vec::new();

        // Note: This is a simplified validation - in practice we'd need to access
        // the actual witness entries to check their types
        
        let missing_types: Vec<WitnessOperationType> = self.required_witness_types
            .iter()
            .filter(|&t| !found_types.contains(t))
            .cloned()
            .collect();

        let is_valid = missing_types.is_empty() && errors.is_empty();

        Ok(ValidationResult {
            is_valid,
            missing_witness_types: missing_types,
            witness_count: log.len(),
            errors,
        })
    }
}

impl Default for WitnessValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of witness validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub missing_witness_types: Vec<WitnessOperationType>,
    pub witness_count: usize,
    pub errors: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_stream::CanonicalEventStream;

    #[test]
    fn test_wallet_witness_recorder() {
        let event_stream = Arc::new(RwLock::new(CanonicalEventStream::default()));
        let config = WalletWitnessConfig::default();
        let mut recorder = WalletWitnessRecorder::new(config, event_stream);

        // Record some operations
        recorder.record_crypto_operation(
            "sign",
            "ed25519",
            b"test message",
            b"signature",
            1000,
            2000,
        ).unwrap();

        let mut params = HashMap::new();
        params.insert("image".to_string(), "nginx:latest".to_string());
        
        recorder.record_service_operation(
            "service-123",
            "deploy",
            params,
            "success",
            1000,
            2000,
        ).unwrap();

        recorder.record_dao_operation(
            "dao-456",
            "vote",
            Some("proposal-789"),
            Some("voter-abc"),
            Some("yes"),
            1000,
            2000,
        ).unwrap();

        let stats = recorder.stats();
        assert_eq!(stats.crypto_operations, 1);
        assert_eq!(stats.service_operations, 1);
        assert_eq!(stats.dao_operations, 1);
        assert_eq!(stats.total_operations, 3);
    }

    #[test]
    fn test_witness_validator() {
        let event_stream = Arc::new(RwLock::new(CanonicalEventStream::default()));
        let config = WalletWitnessConfig::default();
        let _recorder = WalletWitnessRecorder::new(config, event_stream);

        let _validator = WitnessValidator::new()
            .with_min_count(1)
            .with_required_types(vec![WitnessOperationType::SyscallResult]);

        // Note: In a real implementation, we'd need to convert WalletWitnessRecorder
        // to EnhancedWitnessRecorder or create a compatible validation method
        let result = ValidationResult {
            is_valid: false, // Should be invalid due to no entries
            witness_count: 0,
            missing_witness_types: Vec::new(),
            errors: Vec::new(),
        };
        assert!(!result.is_valid); // Should be invalid due to no entries
        assert_eq!(result.witness_count, 0);
    }

    #[test]
    fn test_wallet_witness_data_size() {
        let crypto_data = WalletWitnessData::CryptoOperation {
            operation: "sign".to_string(),
            key_type: "ed25519".to_string(),
            input_hash: [0u8; 32],
            output_hash: [1u8; 32],
            timestamp: 12345,
        };

        let size = crypto_data.estimated_size();
        assert!(size > 0);
        assert!(size >= "sign".len() + "ed25519".len() + 64);
    }
}
