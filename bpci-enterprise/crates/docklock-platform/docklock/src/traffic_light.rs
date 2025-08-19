//! Stage 33: Traffic Light Data Flow Pipeline Foundation
//! 
//! This module implements the core Traffic Light system for real-time data flow control
//! with allow/review/block decisions based on BISO policies and compliance rules.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use tracing::{info, debug};

use crate::error::{DockLockError, DockLockResult};
use crate::receipt::ComplianceStatus;

/// Domain separator for traffic light hashing
pub const TRAFFIC_LIGHT_HASH: u8 = 0x17;

/// Traffic Light state indicating data flow decision
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrafficLightState {
    /// Green: Fully compliant, allow data flow
    Green,
    /// Yellow: Requires review, quarantine for inspection
    Yellow,
    /// Red: Violation detected, block data flow
    Red,
}

impl TrafficLightState {
    /// Get human-readable description of the state
    pub fn description(&self) -> &'static str {
        match self {
            TrafficLightState::Green => "Fully compliant - Pass",
            TrafficLightState::Yellow => "Requires review - Quarantine",
            TrafficLightState::Red => "Violation - Block",
        }
    }

    /// Get security behavior for the state
    pub fn security_behavior(&self) -> &'static str {
        match self {
            TrafficLightState::Green => "Encrypt in transit (XChaCha20-Poly1305) + log to Blockbook",
            TrafficLightState::Yellow => "Route to secure inspection buffer, apply enhanced scanning",
            TrafficLightState::Red => "Stop packet, generate violation receipt, trigger SOC alert",
        }
    }
}

/// Data classification levels for compliance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataClassification {
    /// Personally Identifiable Information
    PII,
    /// Protected Health Information
    PHI,
    /// Payment Card Industry data
    PCI,
    /// General business data
    General,
    /// Public data
    Public,
}

/// Traffic Light decision with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficLightDecision {
    /// Unique decision ID
    pub decision_id: Uuid,
    /// Packet ID being evaluated
    pub packet_id: String,
    /// Final traffic light state
    pub state: TrafficLightState,
    /// Data classification
    pub classification: DataClassification,
    /// Policy ID that was evaluated
    pub policy_id: String,
    /// Reason for the decision
    pub reason: String,
    /// Timestamp of decision
    pub timestamp: u64,
    /// Source service/node making the decision
    pub source: String,
    /// Destination service/node
    pub destination: Option<String>,
    /// Compliance status
    pub compliance_status: ComplianceStatus,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// Cryptographic signature (stored as bytes for serialization)
    pub signature: Option<Vec<u8>>,
    /// Signer public key
    pub signer_pubkey: Option<Vec<u8>>,
}

impl TrafficLightDecision {
    /// Create a new traffic light decision
    pub fn new(
        packet_id: String,
        state: TrafficLightState,
        classification: DataClassification,
        policy_id: String,
        reason: String,
        source: String,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            decision_id: Uuid::new_v4(),
            packet_id,
            state,
            classification,
            policy_id,
            reason,
            timestamp,
            source,
            destination: None,
            compliance_status: match state {
                TrafficLightState::Green => ComplianceStatus::Compliant,
                TrafficLightState::Yellow => ComplianceStatus::Pending,
                TrafficLightState::Red => ComplianceStatus::NonCompliant,
            },
            metadata: HashMap::new(),
            signature: None,
            signer_pubkey: None,
        }
    }

    /// Set destination for the decision
    pub fn with_destination(mut self, destination: String) -> Self {
        self.destination = Some(destination);
        self
    }

    /// Add metadata to the decision
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Compute hash for signing (excludes signature fields)
    pub fn compute_signing_hash(&self) -> DockLockResult<[u8; 32]> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[TRAFFIC_LIGHT_HASH]);
        
        // Hash core decision data
        hasher.update(self.decision_id.as_bytes());
        hasher.update(self.packet_id.as_bytes());
        hasher.update(&[self.state as u8]);
        hasher.update(&bincode::serialize(&self.classification)
            .map_err(|e| DockLockError::EncodingError(format!("Failed to serialize classification: {}", e)))?);
        hasher.update(self.policy_id.as_bytes());
        hasher.update(self.reason.as_bytes());
        hasher.update(&self.timestamp.to_le_bytes());
        hasher.update(self.source.as_bytes());
        
        if let Some(dest) = &self.destination {
            hasher.update(dest.as_bytes());
        }
        
        hasher.update(&bincode::serialize(&self.compliance_status)
            .map_err(|e| DockLockError::EncodingError(format!("Failed to serialize compliance status: {}", e)))?);
        hasher.update(&bincode::serialize(&self.metadata)
            .map_err(|e| DockLockError::EncodingError(format!("Failed to serialize metadata: {}", e)))?);
        
        Ok(*hasher.finalize().as_bytes())
    }

    /// Sign the decision with Ed25519
    pub fn sign(&mut self, signing_key: &SigningKey) -> DockLockResult<()> {
        let hash = self.compute_signing_hash()?;
        let signature = signing_key.sign(&hash);
        
        self.signature = Some(signature.to_bytes().to_vec());
        self.signer_pubkey = Some(signing_key.verifying_key().to_bytes().to_vec());
        
        debug!("Signed traffic light decision {} with Ed25519", self.decision_id);
        Ok(())
    }

    /// Verify the decision signature
    pub fn verify_signature(&self) -> DockLockResult<bool> {
        let signature_bytes = self.signature.as_ref()
            .ok_or_else(|| DockLockError::InvalidOperation("No signature found".to_string()))?;
        let pubkey_bytes = self.signer_pubkey.as_ref()
            .ok_or_else(|| DockLockError::InvalidOperation("No public key found".to_string()))?;
        
        let mut pubkey_array = [0u8; 32];
        pubkey_array.copy_from_slice(pubkey_bytes);
        let verifying_key = VerifyingKey::from_bytes(&pubkey_array)
            .map_err(|e| DockLockError::CryptoError(format!("Invalid public key: {}", e)))?;
        
        let mut signature_array = [0u8; 64];
        signature_array.copy_from_slice(signature_bytes);
        let signature = Signature::from_bytes(&signature_array);
        
        let hash = self.compute_signing_hash()?;
        
        match verifying_key.verify(&hash, &signature) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

/// Traffic Light Pipeline for real-time data flow control
#[derive(Debug)]
pub struct TrafficLightPipeline {
    /// Pipeline ID
    pub id: Uuid,
    /// Pipeline name
    pub name: String,
    /// Recent decisions cache
    decisions: Arc<RwLock<HashMap<Uuid, TrafficLightDecision>>>,
    /// Pipeline statistics
    stats: Arc<RwLock<TrafficLightStats>>,
    /// Configuration
    config: PipelineConfig,
}

/// Pipeline configuration
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Maximum decisions to cache
    pub max_cache_size: usize,
    /// Enable signature verification
    pub verify_signatures: bool,
    /// Default classification for unclassified data
    pub default_classification: DataClassification,
    /// Enable enhanced logging
    pub enhanced_logging: bool,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            max_cache_size: 10000,
            verify_signatures: true,
            default_classification: DataClassification::General,
            enhanced_logging: true,
        }
    }
}

/// Pipeline statistics
#[derive(Debug, Clone, Default)]
pub struct TrafficLightStats {
    /// Total decisions made
    pub total_decisions: u64,
    /// Green (pass) decisions
    pub green_decisions: u64,
    /// Yellow (quarantine) decisions
    pub yellow_decisions: u64,
    /// Red (block) decisions
    pub red_decisions: u64,
    /// Signature verification successes
    pub signature_verifications: u64,
    /// Signature verification failures
    pub signature_failures: u64,
    /// Policy violations detected
    pub policy_violations: u64,
    /// Compliance checks performed
    pub compliance_checks: u64,
}

impl TrafficLightStats {
    /// Get pass rate (green decisions / total decisions)
    pub fn pass_rate(&self) -> f64 {
        if self.total_decisions == 0 {
            0.0
        } else {
            self.green_decisions as f64 / self.total_decisions as f64
        }
    }

    /// Get block rate (red decisions / total decisions)
    pub fn block_rate(&self) -> f64 {
        if self.total_decisions == 0 {
            0.0
        } else {
            self.red_decisions as f64 / self.total_decisions as f64
        }
    }

    /// Get quarantine rate (yellow decisions / total decisions)
    pub fn quarantine_rate(&self) -> f64 {
        if self.total_decisions == 0 {
            0.0
        } else {
            self.yellow_decisions as f64 / self.total_decisions as f64
        }
    }
}

impl TrafficLightPipeline {
    /// Create a new traffic light pipeline
    pub fn new(name: String, config: PipelineConfig) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            decisions: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(TrafficLightStats::default())),
            config,
        }
    }

    /// Create pipeline with default configuration
    pub fn with_defaults(name: String) -> Self {
        Self::new(name, PipelineConfig::default())
    }

    /// Process a packet and make traffic light decision
    pub fn process_packet(
        &self,
        packet_id: String,
        classification: DataClassification,
        policy_id: String,
        source: String,
        destination: Option<String>,
        metadata: HashMap<String, String>,
    ) -> DockLockResult<TrafficLightDecision> {
        // Simulate policy evaluation (will be replaced with actual BISO policy engine)
        let (state, reason) = self.evaluate_policy(&classification, &policy_id, &metadata)?;
        
        let mut decision = TrafficLightDecision::new(
            packet_id,
            state,
            classification,
            policy_id,
            reason,
            source,
        );
        
        if let Some(dest) = destination {
            decision = decision.with_destination(dest);
        }
        
        for (key, value) in metadata {
            decision = decision.with_metadata(key, value);
        }
        
        // Update statistics
        self.update_stats(&decision)?;
        
        // Cache decision
        self.cache_decision(decision.clone())?;
        
        if self.config.enhanced_logging {
            info!(
                "Traffic light decision: {} -> {} ({})",
                decision.packet_id,
                decision.state.description(),
                decision.reason
            );
        }
        
        Ok(decision)
    }

    /// Evaluate policy for traffic light decision (placeholder implementation)
    fn evaluate_policy(
        &self,
        classification: &DataClassification,
        policy_id: &str,
        metadata: &HashMap<String, String>,
    ) -> DockLockResult<(TrafficLightState, String)> {
        // Placeholder policy evaluation logic
        // In real implementation, this would integrate with BISO policy engine
        
        match classification {
            DataClassification::PII => {
                if policy_id.contains("gdpr") {
                    if metadata.get("consent").map(|s| s.as_str()) == Some("true") {
                        Ok((TrafficLightState::Green, "GDPR compliant with consent".to_string()))
                    } else {
                        Ok((TrafficLightState::Red, "GDPR violation: missing consent".to_string()))
                    }
                } else {
                    Ok((TrafficLightState::Yellow, "PII requires review".to_string()))
                }
            },
            DataClassification::PHI => {
                if policy_id.contains("hipaa") {
                    Ok((TrafficLightState::Green, "HIPAA compliant".to_string()))
                } else {
                    Ok((TrafficLightState::Red, "PHI requires HIPAA compliance".to_string()))
                }
            },
            DataClassification::PCI => {
                if metadata.get("encryption").map(|s| s.as_str()) == Some("aes256") {
                    Ok((TrafficLightState::Green, "PCI compliant with encryption".to_string()))
                } else {
                    Ok((TrafficLightState::Red, "PCI violation: insufficient encryption".to_string()))
                }
            },
            DataClassification::General => {
                Ok((TrafficLightState::Green, "General data allowed".to_string()))
            },
            DataClassification::Public => {
                Ok((TrafficLightState::Green, "Public data allowed".to_string()))
            },
        }
    }

    /// Update pipeline statistics
    fn update_stats(&self, decision: &TrafficLightDecision) -> DockLockResult<()> {
        let mut stats = self.stats.write()
            .map_err(|e| DockLockError::InvalidOperation(format!("Failed to acquire stats lock: {}", e)))?;
        
        stats.total_decisions += 1;
        stats.compliance_checks += 1;
        
        match decision.state {
            TrafficLightState::Green => stats.green_decisions += 1,
            TrafficLightState::Yellow => stats.yellow_decisions += 1,
            TrafficLightState::Red => {
                stats.red_decisions += 1;
                stats.policy_violations += 1;
            },
        }
        
        Ok(())
    }

    /// Cache decision for future reference
    fn cache_decision(&self, decision: TrafficLightDecision) -> DockLockResult<()> {
        let mut decisions = self.decisions.write()
            .map_err(|e| DockLockError::InvalidOperation(format!("Failed to acquire decisions lock: {}", e)))?;
        
        // Enforce cache size limit
        if decisions.len() >= self.config.max_cache_size {
            // Remove oldest decision (simple FIFO, could be improved with LRU)
            if let Some(oldest_id) = decisions.keys().next().copied() {
                decisions.remove(&oldest_id);
            }
        }
        
        decisions.insert(decision.decision_id, decision);
        Ok(())
    }

    /// Get decision by ID
    pub fn get_decision(&self, decision_id: &Uuid) -> DockLockResult<Option<TrafficLightDecision>> {
        let decisions = self.decisions.read()
            .map_err(|e| DockLockError::InvalidOperation(format!("Failed to acquire decisions lock: {}", e)))?;
        
        Ok(decisions.get(decision_id).cloned())
    }

    /// Get pipeline statistics
    pub fn get_stats(&self) -> DockLockResult<TrafficLightStats> {
        let stats = self.stats.read()
            .map_err(|e| DockLockError::InvalidOperation(format!("Failed to acquire stats lock: {}", e)))?;
        
        Ok(stats.clone())
    }

    /// Get recent decisions (up to limit)
    pub fn get_recent_decisions(&self, limit: usize) -> DockLockResult<Vec<TrafficLightDecision>> {
        let decisions = self.decisions.read()
            .map_err(|e| DockLockError::InvalidOperation(format!("Failed to acquire decisions lock: {}", e)))?;
        
        let mut recent: Vec<_> = decisions.values().cloned().collect();
        recent.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        recent.truncate(limit);
        
        Ok(recent)
    }

    /// Clear all cached decisions
    pub fn clear_cache(&self) -> DockLockResult<()> {
        let mut decisions = self.decisions.write()
            .map_err(|e| DockLockError::InvalidOperation(format!("Failed to acquire decisions lock: {}", e)))?;
        
        decisions.clear();
        info!("Cleared traffic light pipeline cache for {}", self.name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    #[test]
    fn test_traffic_light_state_descriptions() {
        assert_eq!(TrafficLightState::Green.description(), "Fully compliant - Pass");
        assert_eq!(TrafficLightState::Yellow.description(), "Requires review - Quarantine");
        assert_eq!(TrafficLightState::Red.description(), "Violation - Block");
    }

    #[test]
    fn test_traffic_light_decision_creation() {
        let decision = TrafficLightDecision::new(
            "pkt_123".to_string(),
            TrafficLightState::Green,
            DataClassification::PII,
            "gdpr_policy".to_string(),
            "Compliant with consent".to_string(),
            "analytics_service".to_string(),
        );

        assert_eq!(decision.packet_id, "pkt_123");
        assert_eq!(decision.state, TrafficLightState::Green);
        assert_eq!(decision.classification, DataClassification::PII);
        assert_eq!(decision.policy_id, "gdpr_policy");
        assert_eq!(decision.compliance_status, ComplianceStatus::Compliant);
    }

    #[test]
    fn test_traffic_light_decision_signing_and_verification() {
        let mut rng = OsRng;
        let signing_key = SigningKey::generate(&mut rng);
        
        let mut decision = TrafficLightDecision::new(
            "pkt_456".to_string(),
            TrafficLightState::Yellow,
            DataClassification::PHI,
            "hipaa_policy".to_string(),
            "Requires review".to_string(),
            "medical_service".to_string(),
        );

        // Sign the decision
        decision.sign(&signing_key).unwrap();
        assert!(decision.signature.is_some());
        assert!(decision.signer_pubkey.is_some());

        // Verify the signature
        assert!(decision.verify_signature().unwrap());
    }

    #[test]
    fn test_traffic_light_pipeline_creation() {
        let config = PipelineConfig::default();
        let pipeline = TrafficLightPipeline::new("test_pipeline".to_string(), config);

        assert_eq!(pipeline.name, "test_pipeline");
        assert!(pipeline.config.verify_signatures);
    }

    #[test]
    fn test_traffic_light_pipeline_packet_processing() {
        let pipeline = TrafficLightPipeline::with_defaults("test_pipeline".to_string());
        
        let mut metadata = HashMap::new();
        metadata.insert("consent".to_string(), "true".to_string());
        
        let decision = pipeline.process_packet(
            "pkt_789".to_string(),
            DataClassification::PII,
            "gdpr_policy".to_string(),
            "analytics_service".to_string(),
            Some("storage_service".to_string()),
            metadata,
        ).unwrap();

        assert_eq!(decision.packet_id, "pkt_789");
        assert_eq!(decision.state, TrafficLightState::Green);
        assert_eq!(decision.classification, DataClassification::PII);
        assert_eq!(decision.destination, Some("storage_service".to_string()));
    }

    #[test]
    fn test_traffic_light_pipeline_statistics() {
        let pipeline = TrafficLightPipeline::with_defaults("test_pipeline".to_string());
        
        // Process some packets
        let metadata = HashMap::new();
        
        // Green decision
        pipeline.process_packet(
            "pkt_1".to_string(),
            DataClassification::Public,
            "public_policy".to_string(),
            "service_1".to_string(),
            None,
            metadata.clone(),
        ).unwrap();
        
        // Red decision
        pipeline.process_packet(
            "pkt_2".to_string(),
            DataClassification::PCI,
            "pci_policy".to_string(),
            "service_2".to_string(),
            None,
            metadata.clone(),
        ).unwrap();

        let stats = pipeline.get_stats().unwrap();
        assert_eq!(stats.total_decisions, 2);
        assert_eq!(stats.green_decisions, 1);
        assert_eq!(stats.red_decisions, 1);
        assert_eq!(stats.policy_violations, 1);
        assert_eq!(stats.pass_rate(), 0.5);
        assert_eq!(stats.block_rate(), 0.5);
    }

    #[test]
    fn test_traffic_light_pipeline_decision_caching() {
        let pipeline = TrafficLightPipeline::with_defaults("test_pipeline".to_string());
        
        let metadata = HashMap::new();
        let decision = pipeline.process_packet(
            "pkt_cache_test".to_string(),
            DataClassification::General,
            "general_policy".to_string(),
            "test_service".to_string(),
            None,
            metadata,
        ).unwrap();

        // Retrieve cached decision
        let cached_decision = pipeline.get_decision(&decision.decision_id).unwrap();
        assert!(cached_decision.is_some());
        assert_eq!(cached_decision.unwrap().packet_id, "pkt_cache_test");
    }

    #[test]
    fn test_traffic_light_pipeline_recent_decisions() {
        let pipeline = TrafficLightPipeline::with_defaults("test_pipeline".to_string());
        
        let metadata = HashMap::new();
        
        // Process multiple packets
        for i in 0..5 {
            pipeline.process_packet(
                format!("pkt_{}", i),
                DataClassification::General,
                "general_policy".to_string(),
                "test_service".to_string(),
                None,
                metadata.clone(),
            ).unwrap();
        }

        let recent = pipeline.get_recent_decisions(3).unwrap();
        assert_eq!(recent.len(), 3);
        
        // Should be sorted by timestamp (most recent first)
        for i in 0..recent.len()-1 {
            assert!(recent[i].timestamp >= recent[i+1].timestamp);
        }
    }
}
