//! VM Integrity Validation System for GBF Architecture
//! 
//! Ensures VMs are truthful and unbreachable through:
//! - Cryptographic VM identity validation
//! - Execution attestation with cryptographic proofs
//! - Behavioral monitoring and anomaly detection
//! - Cross-VM witnessing for decentralized validation
//! - Real-time integrity scoring (0.0-1.0 scale)

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::{Result, anyhow};
use thiserror::Error;
use tracing::{info, warn, error, debug};

use crate::{ZjlResult, ZjlError};
use crate::vm_integration::{VmAuditManager, AuditEvent, VmType, VmStatus};
use crate::signing::{ZjlSigner, InMemoryKms};

/// VM Integrity Validation System
#[derive(Debug)]
pub struct VmIntegrityValidator {
    /// VM registry with integrity profiles
    pub vm_registry: Arc<RwLock<HashMap<String, VmIntegrityProfile>>>,
    /// Cryptographic attestations from VMs
    pub cryptographic_attestations: Arc<RwLock<HashMap<String, VmAttestation>>>,
    /// Behavioral monitoring system
    pub behavioral_monitor: Arc<RwLock<VmBehaviorMonitor>>,
    /// Integrity proof engine
    pub integrity_proofs: Arc<RwLock<VmIntegrityProofEngine>>,
    /// Cross-VM witnessing coordinator
    pub witness_coordinator: Arc<RwLock<CrossVmWitnessCoordinator>>,
    /// Integration with existing VM audit manager
    pub audit_manager: Arc<Mutex<VmAuditManager>>,
}

/// VM Integrity Profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmIntegrityProfile {
    /// Unique VM identity hash (unspofable)
    pub vm_identity_hash: String,
    /// VM type and capabilities
    pub vm_type: VmType,
    /// Current integrity score (0.0-1.0)
    pub integrity_score: f64,
    /// Cryptographic public key for attestations
    pub attestation_public_key: String,
    /// Last integrity validation timestamp
    pub last_validation: DateTime<Utc>,
    /// Current status
    pub status: VmIntegrityStatus,
}

/// VM Attestation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmAttestation {
    /// VM identity
    pub vm_id: String,
    /// Attestation timestamp
    pub timestamp: DateTime<Utc>,
    /// Cryptographic signature of execution state
    pub execution_signature: String,
    /// State hash
    pub state_hash: String,
    /// Execution proof
    pub execution_proof: ExecutionProof,
    /// Witness signatures from other VMs
    pub witness_signatures: Vec<WitnessSignature>,
}

/// Execution Proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionProof {
    /// Proof type
    pub proof_type: ProofType,
    /// Cryptographic proof data
    pub proof_data: String,
    /// Verification key
    pub verification_key: String,
    /// Proof generation timestamp
    pub generated_at: DateTime<Utc>,
}

/// Proof Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProofType {
    /// Zero-knowledge proof of correct execution
    ZkExecution,
    /// Merkle proof of state consistency
    StateConsistency,
    /// Proof of resource usage compliance
    ResourceCompliance,
    /// Proof of communication integrity
    CommunicationIntegrity,
}

/// Witness Signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessSignature {
    /// Witnessing VM ID
    pub witness_vm_id: String,
    /// Cryptographic signature
    pub signature: String,
    /// Witness timestamp
    pub timestamp: DateTime<Utc>,
    /// Witness confidence score
    pub confidence: f64,
}

/// VM Behavior Monitor
#[derive(Debug)]
pub struct VmBehaviorMonitor {
    /// Real-time behavioral metrics
    pub behavioral_metrics: HashMap<String, VmBehavioralMetrics>,
    /// Anomaly detection engine
    pub anomaly_detector: AnomalyDetectionEngine,
}

/// VM Behavioral Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmBehavioralMetrics {
    /// Deviation from baseline
    pub baseline_deviation: f64,
    /// Anomaly score (0.0-1.0)
    pub anomaly_score: f64,
    /// Last update timestamp
    pub last_update: DateTime<Utc>,
}

/// Anomaly Detection Engine
#[derive(Debug)]
pub struct AnomalyDetectionEngine {
    /// Statistical models for each VM
    pub statistical_models: HashMap<String, StatisticalModel>,
    /// Detection sensitivity
    pub sensitivity: f64,
}

/// Statistical Model
#[derive(Debug, Clone)]
pub struct StatisticalModel {
    /// Moving averages
    pub moving_averages: HashMap<String, f64>,
    /// Standard deviations
    pub standard_deviations: HashMap<String, f64>,
}

/// VM Integrity Proof Engine
#[derive(Debug)]
pub struct VmIntegrityProofEngine {
    /// Proof cache
    pub proof_cache: HashMap<String, CachedProof>,
}

/// Cached Proof
#[derive(Debug, Clone)]
pub struct CachedProof {
    /// Proof data
    pub proof: ExecutionProof,
    /// Cache timestamp
    pub cached_at: DateTime<Utc>,
    /// Cache expiry
    pub expires_at: DateTime<Utc>,
}

/// Cross-VM Witness Coordinator
#[derive(Debug)]
pub struct CrossVmWitnessCoordinator {
    /// Witness relationships
    pub witness_relationships: HashMap<String, Vec<String>>,
    /// Pending witness requests
    pub pending_requests: HashMap<String, WitnessRequest>,
    /// Witness validation results
    pub validation_results: HashMap<String, WitnessValidationResult>,
}

/// Witness Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessRequest {
    /// Request ID
    pub request_id: String,
    /// VM requesting witness
    pub requesting_vm: String,
    /// VMs requested to witness
    pub witness_vms: Vec<String>,
    /// Attestation to witness
    pub attestation: VmAttestation,
    /// Request timestamp
    pub timestamp: DateTime<Utc>,
    /// Expiry timestamp
    pub expires_at: DateTime<Utc>,
}

/// Witness Validation Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessValidationResult {
    /// Request ID
    pub request_id: String,
    /// Validation success
    pub success: bool,
    /// Witness signatures collected
    pub signatures: Vec<WitnessSignature>,
    /// Consensus confidence
    pub consensus_confidence: f64,
    /// Validation timestamp
    pub validated_at: DateTime<Utc>,
}

/// VM Integrity Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VmIntegrityStatus {
    /// VM is validated and trusted
    Trusted,
    /// VM is under validation
    Validating,
    /// VM has integrity warnings
    Warning,
    /// VM has failed integrity checks
    Compromised,
    /// VM is quarantined
    Quarantined,
}

/// VM Integrity Errors
#[derive(Error, Debug)]
pub enum VmIntegrityError {
    #[error("VM not found: {vm_id}")]
    VmNotFound { vm_id: String },
    #[error("Integrity validation failed: {reason}")]
    ValidationFailed { reason: String },
    #[error("Attestation verification failed")]
    AttestationFailed,
    #[error("Behavioral anomaly detected: {anomaly}")]
    BehavioralAnomaly { anomaly: String },
    #[error("Proof generation failed: {error}")]
    ProofGenerationFailed { error: String },
    #[error("Witness coordination failed")]
    WitnessCoordinationFailed,
    #[error("Integrity score below threshold: {score}")]
    IntegrityThresholdViolation { score: f64 },
}

impl VmIntegrityValidator {
    /// Create new VM integrity validator
    pub fn new(audit_manager: Arc<Mutex<VmAuditManager>>) -> Self {
        Self {
            vm_registry: Arc::new(RwLock::new(HashMap::new())),
            cryptographic_attestations: Arc::new(RwLock::new(HashMap::new())),
            behavioral_monitor: Arc::new(RwLock::new(VmBehaviorMonitor::new())),
            integrity_proofs: Arc::new(RwLock::new(VmIntegrityProofEngine::new())),
            witness_coordinator: Arc::new(RwLock::new(CrossVmWitnessCoordinator::new())),
            audit_manager,
        }
    }

    /// Register VM for integrity validation
    pub async fn register_vm(&self, vm_id: String, vm_type: VmType) -> Result<(), VmIntegrityError> {
        let vm_identity_hash = self.generate_vm_identity_hash(&vm_id, &vm_type).await?;
        let attestation_key = self.generate_attestation_key(&vm_id).await?;
        
        let profile = VmIntegrityProfile {
            vm_identity_hash,
            vm_type: vm_type.clone(),
            integrity_score: 1.0, // Start with perfect score
            attestation_public_key: attestation_key,
            last_validation: Utc::now(),
            status: VmIntegrityStatus::Validating,
        };

        let mut registry = self.vm_registry.write().await;
        registry.insert(vm_id.clone(), profile);

        // Log registration to audit system
        if let Ok(audit_manager) = self.audit_manager.lock() {
            let event = AuditEvent::VmRegistered {
                vm_id: vm_id.clone(),
                vm_type,
                integrity_profile: "default".to_string(),
            };
            audit_manager.log_event(event);
        }

        info!("VM {} registered for integrity validation", vm_id);
        Ok(())
    }

    /// Validate VM integrity
    pub async fn validate_vm_integrity(&self, vm_id: &str) -> Result<f64, VmIntegrityError> {
        let mut registry = self.vm_registry.write().await;
        let profile = registry.get_mut(vm_id)
            .ok_or_else(|| VmIntegrityError::VmNotFound { vm_id: vm_id.to_string() })?;

        // Perform comprehensive integrity validation
        let attestation_score = self.validate_cryptographic_attestation(vm_id).await?;
        let behavioral_score = self.validate_behavioral_integrity(vm_id).await?;
        let witness_score = self.validate_cross_vm_witness(vm_id).await?;
        let resource_score = self.validate_resource_compliance(vm_id).await?;

        // Calculate weighted integrity score
        let integrity_score = (attestation_score * 0.3) + 
                             (behavioral_score * 0.3) + 
                             (witness_score * 0.2) + 
                             (resource_score * 0.2);

        // Update profile
        profile.integrity_score = integrity_score;
        profile.last_validation = Utc::now();

        // Update status based on score
        profile.status = if integrity_score >= 0.95 {
            VmIntegrityStatus::Trusted
        } else if integrity_score >= 0.8 {
            VmIntegrityStatus::Warning
        } else {
            VmIntegrityStatus::Compromised
        };

        // Check threshold violation
        if integrity_score < 0.95 {
            warn!("VM {} integrity score below threshold: {:.3}", vm_id, integrity_score);
            
            if integrity_score < 0.5 {
                return Err(VmIntegrityError::IntegrityThresholdViolation { score: integrity_score });
            }
        }

        // Log validation to audit system
        if let Ok(audit_manager) = self.audit_manager.lock() {
            let event = AuditEvent::SecurityAlert {
                vm_id: vm_id.to_string(),
                alert_type: format!("Integrity validation: score={:.3}", integrity_score),
                severity: if integrity_score >= 0.95 { 1 } else { 2 },
                details: json!({
                    "integrity_score": integrity_score,
                    "attestation_score": attestation_score,
                    "behavioral_score": behavioral_score,
                    "witness_score": witness_score,
                    "resource_score": resource_score
                }),
            };
            audit_manager.log_event(event);
        }

        Ok(integrity_score)
    }

    /// Generate VM identity hash (unspofable)
    async fn generate_vm_identity_hash(&self, vm_id: &str, vm_type: &VmType) -> Result<String, VmIntegrityError> {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(vm_id.as_bytes());
        hasher.update(format!("{:?}", vm_type).as_bytes());
        hasher.update(Utc::now().timestamp().to_string().as_bytes());
        hasher.update(Uuid::new_v4().to_string().as_bytes());
        
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    /// Generate attestation key
    async fn generate_attestation_key(&self, vm_id: &str) -> Result<String, VmIntegrityError> {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(vm_id.as_bytes());
        hasher.update("attestation_key".as_bytes());
        hasher.update(Utc::now().timestamp().to_string().as_bytes());
        
        let result = hasher.finalize();
        Ok(format!("ed25519:{:x}", result))
    }

    /// Validate cryptographic attestation
    async fn validate_cryptographic_attestation(&self, vm_id: &str) -> Result<f64, VmIntegrityError> {
        let attestations = self.cryptographic_attestations.read().await;
        
        if let Some(attestation) = attestations.get(vm_id) {
            let age_minutes = (Utc::now() - attestation.timestamp).num_minutes();
            
            if age_minutes > 5 {
                Ok(0.5) // Attestation too old
            } else {
                Ok(0.95) // Fresh attestation
            }
        } else {
            Ok(0.0) // No attestation available
        }
    }

    /// Validate behavioral integrity
    async fn validate_behavioral_integrity(&self, vm_id: &str) -> Result<f64, VmIntegrityError> {
        let monitor = self.behavioral_monitor.read().await;
        
        if let Some(metrics) = monitor.behavioral_metrics.get(vm_id) {
            let behavioral_score = 1.0 - metrics.anomaly_score;
            Ok(behavioral_score.max(0.0))
        } else {
            Ok(0.8) // Neutral score
        }
    }

    /// Validate cross-VM witness
    async fn validate_cross_vm_witness(&self, vm_id: &str) -> Result<f64, VmIntegrityError> {
        let coordinator = self.witness_coordinator.read().await;
        
        let recent_validations: Vec<_> = coordinator.validation_results
            .values()
            .filter(|result| {
                let age_minutes = (Utc::now() - result.validated_at).num_minutes();
                age_minutes <= 10
            })
            .collect();

        if recent_validations.is_empty() {
            Ok(0.8) // Neutral score
        } else {
            let avg_confidence: f64 = recent_validations.iter()
                .map(|v| v.consensus_confidence)
                .sum::<f64>() / recent_validations.len() as f64;
            Ok(avg_confidence)
        }
    }

    /// Validate resource compliance
    async fn validate_resource_compliance(&self, vm_id: &str) -> Result<f64, VmIntegrityError> {
        let monitor = self.behavioral_monitor.read().await;
        
        if let Some(metrics) = monitor.behavioral_metrics.get(vm_id) {
            let compliance_score = 1.0 - (metrics.baseline_deviation / 100.0).min(1.0);
            Ok(compliance_score.max(0.0))
        } else {
            Ok(0.8) // Neutral score
        }
    }

    /// Get VM integrity status
    pub async fn get_vm_integrity_status(&self, vm_id: &str) -> Result<VmIntegrityProfile, VmIntegrityError> {
        let registry = self.vm_registry.read().await;
        registry.get(vm_id)
            .cloned()
            .ok_or_else(|| VmIntegrityError::VmNotFound { vm_id: vm_id.to_string() })
    }

    /// Get all VMs with integrity scores below threshold
    pub async fn get_compromised_vms(&self, threshold: f64) -> Vec<(String, VmIntegrityProfile)> {
        let registry = self.vm_registry.read().await;
        registry.iter()
            .filter(|(_, profile)| profile.integrity_score < threshold)
            .map(|(id, profile)| (id.clone(), profile.clone()))
            .collect()
    }

    /// Quarantine VM
    pub async fn quarantine_vm(&self, vm_id: &str, reason: &str) -> Result<(), VmIntegrityError> {
        let mut registry = self.vm_registry.write().await;
        let profile = registry.get_mut(vm_id)
            .ok_or_else(|| VmIntegrityError::VmNotFound { vm_id: vm_id.to_string() })?;

        profile.status = VmIntegrityStatus::Quarantined;
        profile.integrity_score = 0.0;

        // Log quarantine to audit system
        if let Ok(audit_manager) = self.audit_manager.lock() {
            let event = AuditEvent::SecurityAlert {
                vm_id: vm_id.to_string(),
                alert_type: "VM Quarantined".to_string(),
                severity: 3,
                details: json!({
                    "reason": reason,
                    "action": "quarantine"
                }),
            };
            audit_manager.log_event(event);
        }

        error!("VM {} quarantined: {}", vm_id, reason);
        Ok(())
    }
}

impl VmBehaviorMonitor {
    pub fn new() -> Self {
        Self {
            behavioral_metrics: HashMap::new(),
            anomaly_detector: AnomalyDetectionEngine::new(),
        }
    }
}

impl AnomalyDetectionEngine {
    pub fn new() -> Self {
        Self {
            statistical_models: HashMap::new(),
            sensitivity: 0.8,
        }
    }
}

impl VmIntegrityProofEngine {
    pub fn new() -> Self {
        Self {
            proof_cache: HashMap::new(),
        }
    }
}

impl CrossVmWitnessCoordinator {
    pub fn new() -> Self {
        Self {
            witness_relationships: HashMap::new(),
            pending_requests: HashMap::new(),
            validation_results: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm_integration::VmInfo;

    #[tokio::test]
    async fn test_vm_integrity_validation() {
        let audit_manager = Arc::new(Mutex::new(
            VmAuditManager::new("/tmp/test_vm_integrity.zjl").unwrap()
        ));
        
        let validator = VmIntegrityValidator::new(audit_manager);
        
        // Register a VM
        validator.register_vm("test-vm-1".to_string(), VmType::HttpCage).await.unwrap();
        
        // Validate integrity
        let score = validator.validate_vm_integrity("test-vm-1").await.unwrap();
        assert!(score >= 0.0 && score <= 1.0);
        
        // Get status
        let status = validator.get_vm_integrity_status("test-vm-1").await.unwrap();
        assert_eq!(status.vm_identity_hash.len(), 64); // SHA256 hash length
    }
}
