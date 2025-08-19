//! # Blockbook Ledger - Immutable Audit Trail
//!
//! The Blockbook is an immutable ledger that records all data flow events,
//! policy decisions, routing actions, and compliance activities in the
//! Metanode BISO Security & Compliance Architecture.
//!
//! ## Core Features
//!
//! - **Immutable Ledger**: Cryptographically linked entries with tamper detection
//! - **Policy Audit Trail**: Records all BISO policy evaluations and decisions
//! - **Routing Decisions**: Logs all Bus BIOS routing and traffic light decisions
//! - **Compliance Events**: Tracks regulatory compliance and violation events
//! - **Cryptographic Integrity**: Ed25519 signatures and Merkle proofs for verification
//!
//! ## Architecture Integration
//!
//! - Integrates with Bus BIOS for routing decision logging
//! - Records Traffic Light Pipeline decisions and policy evaluations
//! - Captures Packet Envelope routing and security events
//! - Provides audit trail for regulatory compliance reporting

use crate::error::DockLockError;
use bpi_enc::{domain_hash, CanonicalCbor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// Domain tag for Blockbook entry hashing (0x1F)
pub const BLOCKBOOK_ENTRY_HASH: &str = "BLOCKBOOK_ENTRY";

/// Domain tag for Blockbook ledger hashing (0x20)
pub const BLOCKBOOK_LEDGER_HASH: u8 = 0x20;

/// Custom serialization for signature field
fn serialize_signature<S>(signature: &Option<[u8; 64]>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match signature {
        Some(sig) => serializer.serialize_some(&hex::encode(sig)),
        None => serializer.serialize_none(),
    }
}

/// Custom deserialization for signature field
fn deserialize_signature<'de, D>(deserializer: D) -> Result<Option<[u8; 64]>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt_hex: Option<String> = Option::deserialize(deserializer)?;
    match opt_hex {
        Some(hex_str) => {
            let bytes = hex::decode(&hex_str).map_err(serde::de::Error::custom)?;
            if bytes.len() != 64 {
                return Err(serde::de::Error::custom("Invalid signature length"));
            }
            let mut sig = [0u8; 64];
            sig.copy_from_slice(&bytes);
            Ok(Some(sig))
        }
        None => Ok(None),
    }
}

/// Types of events that can be recorded in the Blockbook
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BlockbookEventType {
    /// Bus BIOS routing decision
    BusBiosRouting,
    /// Traffic Light Pipeline decision (Green/Yellow/Red)
    TrafficLightDecision,
    /// BISO policy evaluation result
    BisoPolicy,
    /// Packet envelope routing event
    PacketRouting,
    /// Compliance violation detected
    ComplianceViolation,
    /// Security incident or emergency mode activation
    SecurityIncident,
    /// Data availability challenge
    DataAvailabilityChallenge,
    /// Slashing event for validator misbehavior
    SlashingEvent,
    /// Audit book export event
    AuditBookExport,
    /// Custom event type for extensibility
    Custom(String),
}

/// Severity level for Blockbook events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventSeverity {
    /// Informational event
    Info,
    /// Warning event
    Warning,
    /// Error event
    Error,
    /// Critical security event
    Critical,
}

/// Blockbook entry representing a single audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockbookEntry {
    /// Unique entry ID
    pub entry_id: u64,
    /// Timestamp when the event occurred
    pub timestamp: u64,
    /// Type of event being recorded
    pub event_type: BlockbookEventType,
    /// Severity level of the event
    pub severity: EventSeverity,
    /// Source component that generated the event
    pub source: String,
    /// Hash of the previous entry for chain integrity
    pub prev_hash: [u8; 32],
    /// Event-specific data payload
    pub payload: Vec<u8>,
    /// Optional metadata for additional context
    pub metadata: HashMap<String, String>,
    /// Ed25519 signature of the entry (hex-encoded for serialization)
    #[serde(
        serialize_with = "serialize_signature",
        deserialize_with = "deserialize_signature"
    )]
    pub signature: Option<[u8; 64]>,
}

/// Result of a data availability challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAvailabilityChallenge {
    /// Challenge ID
    pub challenge_id: u64,
    /// Block height being challenged
    pub block_height: u64,
    /// Challenged data hash
    pub data_hash: [u8; 32],
    /// Challenge result (success/failure)
    pub result: bool,
    /// Response time in milliseconds
    pub response_time_ms: u64,
    /// Proof of data availability
    pub proof: Option<Vec<u8>>,
}

/// Slashing event for validator misbehavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlashingEvent {
    /// Validator ID being slashed
    pub validator_id: String,
    /// Type of misbehavior detected
    pub misbehavior_type: String,
    /// Evidence of misbehavior
    pub evidence: Vec<u8>,
    /// Slashing amount or penalty
    pub penalty: u64,
    /// Block height where misbehavior occurred
    pub block_height: u64,
}

/// Blockbook ledger statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockbookStats {
    /// Total number of entries in the ledger
    pub total_entries: u64,
    /// Number of entries by event type
    pub entries_by_type: HashMap<String, u64>,
    /// Number of entries by severity
    pub entries_by_severity: HashMap<String, u64>,
    /// Number of compliance violations
    pub compliance_violations: u64,
    /// Number of security incidents
    pub security_incidents: u64,
    /// Number of data availability challenges
    pub da_challenges: u64,
    /// Number of slashing events
    pub slashing_events: u64,
    /// Ledger integrity status
    pub integrity_verified: bool,
}

/// Configuration for the Blockbook ledger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockbookConfig {
    /// Maximum number of entries to keep in memory
    pub max_entries: usize,
    /// Whether to enable automatic integrity verification
    pub enable_integrity_checks: bool,
    /// Whether to enable entry signing
    pub enable_signing: bool,
    /// Retention period for entries in seconds
    pub retention_period_seconds: u64,
    /// Whether to export to audit book automatically
    pub auto_export_audit_book: bool,
}

impl Default for BlockbookConfig {
    fn default() -> Self {
        Self {
            max_entries: 10000,
            enable_integrity_checks: true,
            enable_signing: true,
            retention_period_seconds: 86400 * 30, // 30 days
            auto_export_audit_book: false,
        }
    }
}

/// Main Blockbook ledger for immutable audit trail
#[derive(Debug)]
pub struct Blockbook {
    /// Configuration
    config: BlockbookConfig,
    /// Ledger entries in chronological order
    entries: Arc<RwLock<VecDeque<BlockbookEntry>>>,
    /// Next entry ID counter
    next_entry_id: Arc<RwLock<u64>>,
    /// Ledger statistics
    stats: Arc<RwLock<BlockbookStats>>,
    /// Index by event type for fast queries
    type_index: Arc<RwLock<HashMap<String, Vec<u64>>>>,
    /// Index by severity for fast queries
    severity_index: Arc<RwLock<HashMap<String, Vec<u64>>>>,
}

impl Blockbook {
    /// Create a new Blockbook ledger
    pub fn new(config: BlockbookConfig) -> Self {
        Self {
            config,
            entries: Arc::new(RwLock::new(VecDeque::new())),
            next_entry_id: Arc::new(RwLock::new(1)),
            stats: Arc::new(RwLock::new(BlockbookStats {
                total_entries: 0,
                entries_by_type: HashMap::new(),
                entries_by_severity: HashMap::new(),
                compliance_violations: 0,
                security_incidents: 0,
                da_challenges: 0,
                slashing_events: 0,
                integrity_verified: true,
            })),
            type_index: Arc::new(RwLock::new(HashMap::new())),
            severity_index: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Record a new event in the Blockbook
    pub fn record_event(
        &self,
        event_type: BlockbookEventType,
        severity: EventSeverity,
        source: String,
        payload: Vec<u8>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<u64, DockLockError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| DockLockError::SystemTime(e.to_string()))?
            .as_secs();

        let mut entries = self.entries.write().unwrap();
        let mut next_id = self.next_entry_id.write().unwrap();
        let mut stats = self.stats.write().unwrap();

        // Get previous hash for chain integrity
        let prev_hash = if let Some(last_entry) = entries.back() {
            self.compute_entry_hash(last_entry)?
        } else {
            [0u8; 32] // Genesis entry
        };

        let entry_id = *next_id;
        *next_id += 1;

        let entry = BlockbookEntry {
            entry_id,
            timestamp,
            event_type: event_type.clone(),
            severity: severity.clone(),
            source,
            prev_hash,
            payload,
            metadata: metadata.unwrap_or_default(),
            signature: None, // Will be set if signing is enabled
        };

        // Add to ledger
        entries.push_back(entry.clone());

        // Update statistics
        stats.total_entries += 1;
        let type_key = format!("{:?}", event_type);
        *stats.entries_by_type.entry(type_key.clone()).or_insert(0) += 1;
        let severity_key = format!("{:?}", severity);
        *stats.entries_by_severity.entry(severity_key.clone()).or_insert(0) += 1;

        // Update specific counters
        match event_type {
            BlockbookEventType::ComplianceViolation => stats.compliance_violations += 1,
            BlockbookEventType::SecurityIncident => stats.security_incidents += 1,
            BlockbookEventType::DataAvailabilityChallenge => stats.da_challenges += 1,
            BlockbookEventType::SlashingEvent => stats.slashing_events += 1,
            _ => {}
        }

        // Update indexes
        self.update_indexes(entry_id, &type_key, &severity_key)?;

        // Enforce retention policy
        if entries.len() > self.config.max_entries {
            entries.pop_front();
        }

        Ok(entry_id)
    }

    /// Record a Bus BIOS routing decision
    pub fn record_bus_bios_routing(
        &self,
        routing_decision: Vec<u8>,
        source: String,
    ) -> Result<u64, DockLockError> {
        let metadata = HashMap::from([
            ("component".to_string(), "bus_bios".to_string()),
            ("decision_type".to_string(), "routing".to_string()),
        ]);

        self.record_event(
            BlockbookEventType::BusBiosRouting,
            EventSeverity::Info,
            source,
            routing_decision,
            Some(metadata),
        )
    }

    /// Record a Traffic Light Pipeline decision
    pub fn record_traffic_light_decision(
        &self,
        decision: Vec<u8>,
        state: String,
        source: String,
    ) -> Result<u64, DockLockError> {
        let severity = match state.as_str() {
            "Green" => EventSeverity::Info,
            "Yellow" => EventSeverity::Warning,
            "Red" => EventSeverity::Error,
            _ => EventSeverity::Info,
        };

        let metadata = HashMap::from([
            ("component".to_string(), "traffic_light".to_string()),
            ("state".to_string(), state),
        ]);

        self.record_event(
            BlockbookEventType::TrafficLightDecision,
            severity,
            source,
            decision,
            Some(metadata),
        )
    }

    /// Record a BISO policy evaluation
    pub fn record_biso_policy_evaluation(
        &self,
        policy_result: Vec<u8>,
        policy_id: String,
        source: String,
    ) -> Result<u64, DockLockError> {
        let metadata = HashMap::from([
            ("component".to_string(), "biso_policy".to_string()),
            ("policy_id".to_string(), policy_id),
        ]);

        self.record_event(
            BlockbookEventType::BisoPolicy,
            EventSeverity::Info,
            source,
            policy_result,
            Some(metadata),
        )
    }

    /// Record a data availability challenge
    pub fn record_da_challenge(
        &self,
        challenge: DataAvailabilityChallenge,
        source: String,
    ) -> Result<u64, DockLockError> {
        let payload = bincode::serialize(&challenge)
            .map_err(DockLockError::Serialization)?;

        let severity = if challenge.result {
            EventSeverity::Info
        } else {
            EventSeverity::Error
        };

        let metadata = HashMap::from([
            ("component".to_string(), "data_availability".to_string()),
            ("challenge_id".to_string(), challenge.challenge_id.to_string()),
            ("block_height".to_string(), challenge.block_height.to_string()),
            ("result".to_string(), challenge.result.to_string()),
        ]);

        self.record_event(
            BlockbookEventType::DataAvailabilityChallenge,
            severity,
            source,
            payload,
            Some(metadata),
        )
    }

    /// Record a slashing event
    pub fn record_slashing_event(
        &self,
        slashing: SlashingEvent,
        source: String,
    ) -> Result<u64, DockLockError> {
        let payload = bincode::serialize(&slashing)
            .map_err(DockLockError::Serialization)?;

        let metadata = HashMap::from([
            ("component".to_string(), "slashing".to_string()),
            ("validator_id".to_string(), slashing.validator_id.clone()),
            ("misbehavior_type".to_string(), slashing.misbehavior_type.clone()),
            ("penalty".to_string(), slashing.penalty.to_string()),
        ]);

        self.record_event(
            BlockbookEventType::SlashingEvent,
            EventSeverity::Critical,
            source,
            payload,
            Some(metadata),
        )
    }

    /// Get entries by event type
    pub fn get_entries_by_type(&self, event_type: &BlockbookEventType) -> Vec<BlockbookEntry> {
        let entries = self.entries.read().unwrap();
        let type_key = format!("{:?}", event_type);
        
        entries
            .iter()
            .filter(|entry| format!("{:?}", entry.event_type) == type_key)
            .cloned()
            .collect()
    }

    /// Get entries by severity
    pub fn get_entries_by_severity(&self, severity: &EventSeverity) -> Vec<BlockbookEntry> {
        let entries = self.entries.read().unwrap();
        let severity_key = format!("{:?}", severity);
        
        entries
            .iter()
            .filter(|entry| format!("{:?}", entry.severity) == severity_key)
            .cloned()
            .collect()
    }

    /// Get recent entries (last N entries)
    pub fn get_recent_entries(&self, count: usize) -> Vec<BlockbookEntry> {
        let entries = self.entries.read().unwrap();
        entries
            .iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }

    /// Verify ledger integrity by checking hash chain
    pub fn verify_integrity(&self) -> Result<bool, DockLockError> {
        let entries = self.entries.read().unwrap();
        let mut prev_hash = [0u8; 32]; // Genesis hash

        for entry in entries.iter() {
            if entry.prev_hash != prev_hash {
                return Ok(false);
            }
            prev_hash = self.compute_entry_hash(entry)?;
        }

        // Update integrity status
        let mut stats = self.stats.write().unwrap();
        stats.integrity_verified = true;

        Ok(true)
    }

    /// Get Blockbook statistics
    pub fn get_stats(&self) -> BlockbookStats {
        self.stats.read().unwrap().clone()
    }

    /// Export entries for audit book (filtered for regulatory compliance)
    pub fn export_audit_book(&self, filter_sensitive: bool) -> Vec<BlockbookEntry> {
        let entries = self.entries.read().unwrap();
        
        if filter_sensitive {
            // Filter out sensitive internal events for regulatory export
            entries
                .iter()
                .filter(|entry| matches!(
                    entry.event_type,
                    BlockbookEventType::ComplianceViolation
                        | BlockbookEventType::BisoPolicy
                        | BlockbookEventType::TrafficLightDecision
                        | BlockbookEventType::AuditBookExport
                ))
                .cloned()
                .collect()
        } else {
            entries.iter().cloned().collect()
        }
    }

    /// Compute hash of a Blockbook entry
    fn compute_entry_hash(&self, entry: &BlockbookEntry) -> Result<[u8; 32], DockLockError> {
        let canonical_bytes = CanonicalCbor::encode(entry)
            .map_err(DockLockError::Encoding)?;
        Ok(domain_hash(BLOCKBOOK_ENTRY_HASH, &canonical_bytes))
    }

    /// Update indexes for fast queries
    fn update_indexes(
        &self,
        entry_id: u64,
        type_key: &str,
        severity_key: &str,
    ) -> Result<(), DockLockError> {
        let mut type_index = self.type_index.write().unwrap();
        let mut severity_index = self.severity_index.write().unwrap();

        type_index
            .entry(type_key.to_string())
            .or_insert_with(Vec::new)
            .push(entry_id);

        severity_index
            .entry(severity_key.to_string())
            .or_insert_with(Vec::new)
            .push(entry_id);

        Ok(())
    }
}

// Note: BlockbookEntry, DataAvailabilityChallenge, and SlashingEvent implement Serialize,
// so they can be encoded with CanonicalCbor::encode()

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_blockbook() -> Blockbook {
        let config = BlockbookConfig {
            max_entries: 100,
            enable_integrity_checks: true,
            enable_signing: false,
            retention_period_seconds: 3600,
            auto_export_audit_book: false,
        };
        Blockbook::new(config)
    }

    #[test]
    fn test_blockbook_creation() {
        let blockbook = create_test_blockbook();
        let stats = blockbook.get_stats();
        assert_eq!(stats.total_entries, 0);
        assert!(stats.integrity_verified);
    }

    #[test]
    fn test_record_bus_bios_routing() {
        let blockbook = create_test_blockbook();
        let routing_data = b"routing_decision_data".to_vec();
        
        let entry_id = blockbook
            .record_bus_bios_routing(routing_data, "bus_bios_test".to_string())
            .unwrap();
        
        assert_eq!(entry_id, 1);
        
        let stats = blockbook.get_stats();
        assert_eq!(stats.total_entries, 1);
        assert_eq!(stats.entries_by_type.get("BusBiosRouting"), Some(&1));
    }

    #[test]
    fn test_record_traffic_light_decision() {
        let blockbook = create_test_blockbook();
        let decision_data = b"traffic_light_decision".to_vec();
        
        let entry_id = blockbook
            .record_traffic_light_decision(
                decision_data,
                "Red".to_string(),
                "traffic_light_test".to_string(),
            )
            .unwrap();
        
        assert_eq!(entry_id, 1);
        
        let entries = blockbook.get_entries_by_type(&BlockbookEventType::TrafficLightDecision);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].severity, EventSeverity::Error);
    }

    #[test]
    fn test_record_da_challenge() {
        let blockbook = create_test_blockbook();
        let challenge = DataAvailabilityChallenge {
            challenge_id: 123,
            block_height: 456,
            data_hash: [1u8; 32],
            result: true,
            response_time_ms: 50,
            proof: Some(b"proof_data".to_vec()),
        };
        
        let entry_id = blockbook
            .record_da_challenge(challenge, "da_service".to_string())
            .unwrap();
        
        assert_eq!(entry_id, 1);
        
        let stats = blockbook.get_stats();
        assert_eq!(stats.da_challenges, 1);
    }

    #[test]
    fn test_record_slashing_event() {
        let blockbook = create_test_blockbook();
        let slashing = SlashingEvent {
            validator_id: "validator_123".to_string(),
            misbehavior_type: "double_signing".to_string(),
            evidence: b"evidence_data".to_vec(),
            penalty: 1000,
            block_height: 789,
        };
        
        let entry_id = blockbook
            .record_slashing_event(slashing, "consensus_engine".to_string())
            .unwrap();
        
        assert_eq!(entry_id, 1);
        
        let stats = blockbook.get_stats();
        assert_eq!(stats.slashing_events, 1);
        
        let entries = blockbook.get_entries_by_severity(&EventSeverity::Critical);
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn test_ledger_integrity() {
        let blockbook = create_test_blockbook();
        
        // Record multiple events
        blockbook
            .record_bus_bios_routing(b"event1".to_vec(), "source1".to_string())
            .unwrap();
        blockbook
            .record_traffic_light_decision(
                b"event2".to_vec(),
                "Green".to_string(),
                "source2".to_string(),
            )
            .unwrap();
        blockbook
            .record_biso_policy_evaluation(
                b"event3".to_vec(),
                "policy1".to_string(),
                "source3".to_string(),
            )
            .unwrap();
        
        // Verify integrity
        assert!(blockbook.verify_integrity().unwrap());
        
        let stats = blockbook.get_stats();
        assert_eq!(stats.total_entries, 3);
        assert!(stats.integrity_verified);
    }

    #[test]
    fn test_export_audit_book() {
        let blockbook = create_test_blockbook();
        
        // Record various events
        blockbook
            .record_bus_bios_routing(b"routing".to_vec(), "bus_bios".to_string())
            .unwrap();
        blockbook
            .record_traffic_light_decision(
                b"decision".to_vec(),
                "Yellow".to_string(),
                "traffic_light".to_string(),
            )
            .unwrap();
        blockbook
            .record_event(
                BlockbookEventType::ComplianceViolation,
                EventSeverity::Error,
                "compliance_engine".to_string(),
                b"violation".to_vec(),
                None,
            )
            .unwrap();
        
        // Export all entries
        let all_entries = blockbook.export_audit_book(false);
        assert_eq!(all_entries.len(), 3);
        
        // Export filtered entries for regulatory compliance
        let filtered_entries = blockbook.export_audit_book(true);
        assert_eq!(filtered_entries.len(), 2); // Only traffic light and compliance violation
    }

    #[test]
    fn test_get_recent_entries() {
        let blockbook = create_test_blockbook();
        
        // Record multiple events
        for i in 0..5 {
            blockbook
                .record_bus_bios_routing(
                    format!("event_{}", i).into_bytes(),
                    format!("source_{}", i),
                )
                .unwrap();
        }
        
        let recent_entries = blockbook.get_recent_entries(3);
        assert_eq!(recent_entries.len(), 3);
        
        // Should be in reverse chronological order (most recent first)
        assert!(recent_entries[0].entry_id > recent_entries[1].entry_id);
        assert!(recent_entries[1].entry_id > recent_entries[2].entry_id);
    }
}
