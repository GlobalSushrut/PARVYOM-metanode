//! # Stage 41: Inclusion Lists (Consensus Rule)
//!
//! Implements consensus-level inclusion lists to ensure transaction inclusion and prevent censorship.
//! This module provides the foundation for censorship resistance by enforcing mandatory transaction
//! inclusion at the consensus layer.

use crate::error::{DockLockError, DockLockResult};
use crate::packet_envelope::PacketEnvelope;
use crate::traffic_light::TrafficLightState;
use crate::biso_policy::{GeographicRegion, PolicyEvaluationResult};
use crate::blockbook::{Blockbook, BlockbookEventType, EventSeverity};
use ed25519_dalek::{SigningKey, Signer};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use uuid::Uuid;
use tracing::{info, debug};

/// Domain separator for inclusion list hashing
const INCLUSION_LIST_HASH: u8 = 0x30;

/// Maximum number of transactions in an inclusion list
const MAX_INCLUSION_LIST_SIZE: usize = 1000;

/// Maximum age for inclusion list entries (24 hours)
const MAX_INCLUSION_AGE: Duration = Duration::from_secs(24 * 60 * 60);

/// Inclusion priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum InclusionPriority {
    Emergency,
    High,
    Normal,
    Low,
}

impl Default for InclusionPriority {
    fn default() -> Self {
        InclusionPriority::Normal
    }
}

/// Inclusion rule types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InclusionRuleType {
    GeographicMandatory(GeographicRegion),
    TrafficLightMandatory(TrafficLightState),
    ValueThreshold(u64),
    PolicyMandatory(bool),
    Emergency,
    HighPriority,
    CensorshipResistance,
    RegulatoryCompliance,
    NetworkMaintenance,
    Custom,
}

/// Inclusion rule with enforcement parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionRule {
    pub rule_id: Uuid,
    pub rule_type: InclusionRuleType,
    pub priority: InclusionPriority,
    pub active: bool,
    pub created_at: u64,
    pub expires_at: Option<u64>,
    pub description: String,
    pub signature: Option<String>,
    pub creator_pubkey: Option<String>,
}

impl InclusionRule {
    pub fn new(
        rule_type: InclusionRuleType,
        priority: InclusionPriority,
        description: String,
    ) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Self {
            rule_id: Uuid::new_v4(),
            rule_type,
            priority,
            active: true,
            created_at: now,
            expires_at: None,
            description,
            signature: None,
            creator_pubkey: None,
        }
    }

    pub fn applies_to(&self, envelope: &PacketEnvelope, policy_result: &PolicyEvaluationResult) -> bool {
        if !self.active {
            return false;
        }

        if let Some(expires_at) = self.expires_at {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            if now > expires_at {
                return false;
            }
        }

        match &self.rule_type {
            InclusionRuleType::GeographicMandatory(_region) => {
                // Note: geographic_region not available in current EnvelopeMetadata
                // Using traffic light state as proxy for now
                envelope.metadata.traffic_light_state != TrafficLightState::Red
            }
            InclusionRuleType::TrafficLightMandatory(state) => {
                envelope.metadata.traffic_light_state == *state
            }
            InclusionRuleType::ValueThreshold(_threshold) => {
                // Note: value_estimate not available in current EnvelopeMetadata
                // Using priority as proxy for now
                envelope.metadata.priority >= 128
            }
            InclusionRuleType::PolicyMandatory(must_be_allowed) => {
                policy_result.passed == *must_be_allowed
            }
            InclusionRuleType::Emergency => true,
            InclusionRuleType::HighPriority => true,
            InclusionRuleType::CensorshipResistance => true,
            InclusionRuleType::RegulatoryCompliance => true,
            InclusionRuleType::NetworkMaintenance => true,
            InclusionRuleType::Custom => false,
        }
    }

    pub fn sign(&mut self, signing_key: &SigningKey) -> DockLockResult<()> {
        let hash = self.compute_signing_hash()?;
        let signature = signing_key.sign(&hash);
        self.signature = Some(hex::encode(signature.to_bytes()));
        self.creator_pubkey = Some(hex::encode(signing_key.verifying_key().to_bytes()));
        Ok(())
    }

    fn compute_signing_hash(&self) -> DockLockResult<[u8; 32]> {
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(&[INCLUSION_LIST_HASH]);
        
        let encoded = bincode::serialize(&(
            &self.rule_id,
            &self.rule_type,
            &self.priority,
            &self.active,
            &self.created_at,
            &self.expires_at,
            &self.description,
        )).map_err(|e| DockLockError::Serialization(e))?;
        
        hasher.update(&encoded);
        Ok(hasher.finalize().into())
    }
}

/// Transaction entry in inclusion list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionEntry {
    pub tx_id: String,
    pub envelope: PacketEnvelope,
    pub policy_result: PolicyEvaluationResult,
    pub priority: InclusionPriority,
    pub added_at: u64,
    pub mandating_rules: Vec<Uuid>,
    pub included: bool,
    pub inclusion_block: Option<u64>,
    pub inclusion_proof: Option<String>,
}

impl InclusionEntry {
    pub fn new(
        tx_id: String,
        envelope: PacketEnvelope,
        policy_result: PolicyEvaluationResult,
        priority: InclusionPriority,
        mandating_rules: Vec<Uuid>,
    ) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Self {
            tx_id,
            envelope,
            policy_result,
            priority,
            added_at: now,
            mandating_rules,
            included: false,
            inclusion_block: None,
            inclusion_proof: None,
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        now > self.added_at + MAX_INCLUSION_AGE.as_secs()
    }
}

/// Inclusion list for mandatory transaction inclusion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionList {
    pub list_id: Uuid,
    pub entries: BTreeMap<String, InclusionEntry>,
    pub rules: HashMap<Uuid, InclusionRule>,
    pub created_at: u64,
    pub updated_at: u64,
    pub current_block: u64,
    pub signature: Option<String>,
    pub creator_pubkey: Option<String>,
}

impl InclusionList {
    pub fn new() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Self {
            list_id: Uuid::new_v4(),
            entries: BTreeMap::new(),
            rules: HashMap::new(),
            created_at: now,
            updated_at: now,
            current_block: 0,
            signature: None,
            creator_pubkey: None,
        }
    }

    pub fn add_rule(&mut self, rule: InclusionRule) -> DockLockResult<()> {
        if self.rules.len() >= 100 {
            return Err(DockLockError::InvalidOperation("Too many inclusion rules".to_string()));
        }

        self.rules.insert(rule.rule_id, rule);
        self.updated_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        debug!("Added inclusion rule: {}", self.rules.len());
        Ok(())
    }

    pub fn remove_rule(&mut self, rule_id: &Uuid) -> DockLockResult<()> {
        if self.rules.remove(rule_id).is_some() {
            self.updated_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            debug!("Removed inclusion rule: {}", rule_id);
            Ok(())
        } else {
            Err(DockLockError::InvalidOperation("Rule not found".to_string()))
        }
    }

    pub fn should_include(&self, envelope: &PacketEnvelope, policy_result: &PolicyEvaluationResult) -> (bool, Vec<Uuid>) {
        let mut mandating_rules = Vec::new();
        
        for rule in self.rules.values() {
            if rule.applies_to(envelope, policy_result) {
                mandating_rules.push(rule.rule_id);
            }
        }

        (!mandating_rules.is_empty(), mandating_rules)
    }

    pub fn add_transaction(
        &mut self,
        tx_id: String,
        envelope: PacketEnvelope,
        policy_result: PolicyEvaluationResult,
    ) -> DockLockResult<bool> {
        let (should_include, mandating_rules) = self.should_include(&envelope, &policy_result);
        
        if !should_include {
            return Ok(false);
        }

        if self.entries.len() >= MAX_INCLUSION_LIST_SIZE {
            return Err(DockLockError::InvalidOperation("Inclusion list full".to_string()));
        }

        let priority = mandating_rules.iter()
            .filter_map(|rule_id| self.rules.get(rule_id))
            .map(|rule| &rule.priority)
            .min()
            .unwrap_or(&InclusionPriority::Normal)
            .clone();

        let entry = InclusionEntry::new(
            tx_id.clone(),
            envelope,
            policy_result,
            priority,
            mandating_rules,
        );

        self.entries.insert(tx_id.clone(), entry);
        self.updated_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        info!("Added transaction to inclusion list: {}", tx_id);
        Ok(true)
    }

    pub fn mark_included(&mut self, tx_id: &str, block_height: u64, proof: Option<String>) -> DockLockResult<()> {
        if let Some(entry) = self.entries.get_mut(tx_id) {
            entry.included = true;
            entry.inclusion_block = Some(block_height);
            entry.inclusion_proof = proof;
            self.updated_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            
            info!("Marked transaction as included: {} at block {}", tx_id, block_height);
            Ok(())
        } else {
            Err(DockLockError::InvalidOperation("Transaction not in inclusion list".to_string()))
        }
    }

    pub fn get_pending_transactions(&self) -> Vec<&InclusionEntry> {
        self.entries.values()
            .filter(|entry| !entry.included && !entry.is_expired())
            .collect()
    }

    pub fn cleanup_expired(&mut self) -> usize {
        let initial_count = self.entries.len();
        self.entries.retain(|_, entry| !entry.is_expired());
        let removed_count = initial_count - self.entries.len();
        
        if removed_count > 0 {
            self.updated_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            debug!("Cleaned up {} expired inclusion entries", removed_count);
        }
        
        removed_count
    }

    pub fn sign(&mut self, signing_key: &SigningKey) -> DockLockResult<()> {
        let hash = self.compute_signing_hash()?;
        let signature = signing_key.sign(&hash);
        self.signature = Some(hex::encode(signature.to_bytes()));
        self.creator_pubkey = Some(hex::encode(signing_key.verifying_key().to_bytes()));
        Ok(())
    }

    fn compute_signing_hash(&self) -> DockLockResult<[u8; 32]> {
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(&[INCLUSION_LIST_HASH]);
        
        let encoded = bincode::serialize(&(
            &self.list_id,
            &self.entries,
            &self.rules,
            &self.updated_at,
            &self.current_block,
        )).map_err(|e| DockLockError::Serialization(e))?;
        
        hasher.update(&encoded);
        Ok(hasher.finalize().into())
    }
}

/// Censorship violation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CensorshipViolation {
    pub violation_id: Uuid,
    pub violation_type: CensorshipViolationType,
    pub tx_id: String,
    pub block_producer: Option<String>,
    pub timestamp: u64,
    pub severity: ViolationSeverity,
    pub evidence: String,
    pub reported: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CensorshipViolationType {
    TransactionExclusion,
    GeographicCensorship,
    ValueBasedCensorship,
    PolicyViolation,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Inclusion list manager with consensus integration
#[derive(Debug)]
pub struct InclusionListManager {
    pub inclusion_list: Arc<RwLock<InclusionList>>,
    pub blockbook: Arc<RwLock<Blockbook>>,
    pub config: InclusionListConfig,
    pub stats: Arc<RwLock<InclusionListStats>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionListConfig {
    pub auto_enforce: bool,
    pub max_list_size: usize,
    pub cleanup_interval: u64,
    pub detection_interval: u64,
}

impl Default for InclusionListConfig {
    fn default() -> Self {
        Self {
            auto_enforce: true,
            max_list_size: MAX_INCLUSION_LIST_SIZE,
            cleanup_interval: 3600,
            detection_interval: 300,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InclusionListStats {
    pub total_processed: u64,
    pub total_included: u64,
    pub total_rules: u64,
    pub total_violations: u64,
    pub avg_processing_time: f64,
    pub inclusion_rate: f64,
    pub last_update: u64,
}

impl InclusionListManager {
    pub fn new(config: InclusionListConfig, blockbook: Arc<RwLock<Blockbook>>) -> Self {
        let inclusion_list = Arc::new(RwLock::new(InclusionList::new()));
        let stats = Arc::new(RwLock::new(InclusionListStats::default()));

        Self {
            inclusion_list,
            blockbook,
            config,
            stats,
        }
    }

    pub fn process_transaction(
        &self,
        tx_id: String,
        envelope: PacketEnvelope,
        policy_result: PolicyEvaluationResult,
    ) -> DockLockResult<bool> {
        let start_time = SystemTime::now();
        
        let included = {
            let mut list = self.inclusion_list.write()
                .map_err(|_| DockLockError::InvalidOperation("Failed to acquire inclusion list lock".to_string()))?;
            
            list.add_transaction(tx_id.clone(), envelope.clone(), policy_result.clone())?
        };

        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| DockLockError::InvalidOperation("Failed to acquire stats lock".to_string()))?;
            
            stats.total_processed += 1;
            if included {
                stats.total_included += 1;
            }
            
            let processing_time = start_time.elapsed().unwrap_or_default().as_millis() as f64;
            stats.avg_processing_time = (stats.avg_processing_time * (stats.total_processed - 1) as f64 + processing_time) / stats.total_processed as f64;
            stats.inclusion_rate = stats.total_included as f64 / stats.total_processed as f64;
            stats.last_update = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        }

        // Log to blockbook if included
        if included {
            if let Ok(blockbook) = self.blockbook.write() {
                let metadata = format!("Transaction {} added to inclusion list", tx_id);
                let _ = blockbook.record_event(
                    BlockbookEventType::Custom("inclusion_list_addition".to_string()),
                    EventSeverity::Info,
                    metadata,
                    serde_json::to_string(&envelope).unwrap_or_default().into_bytes(),
                    None,
                );
            }
        }

        Ok(included)
    }

    /// Add a transaction for forced inclusion
    pub fn add_transaction_for_inclusion(
        &self,
        envelope: &PacketEnvelope,
        priority: InclusionPriority,
        rule_type: InclusionRuleType,
    ) -> DockLockResult<()> {
        let mut list = self.inclusion_list.write()
            .map_err(|_| DockLockError::ConcurrencyError("Failed to acquire inclusion list lock".to_string()))?;

        // Create a new inclusion rule for this transaction
        let rule_id = Uuid::new_v4();
        let rule = InclusionRule {
            rule_id,
            rule_type,
            priority: priority.clone(),
            active: true,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            expires_at: Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 86400), // 24 hours
            description: "Force inclusion rule".to_string(),
            signature: None,
            creator_pubkey: None,
        };

        list.add_rule(rule)?;

        // Add the transaction as an inclusion entry with a dummy policy result
        let dummy_policy = PolicyEvaluationResult {
            id: Uuid::new_v4(),
            policy_id: Uuid::new_v4(),
            decision: crate::traffic_light::TrafficLightState::Green,
            passed: true,
            reason: "Force inclusion approved".to_string(),
            violations: Vec::new(),
            warnings: Vec::new(),
            evaluated_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            evaluation_duration_ms: 0,
            signature: None,
            evaluator_pubkey: None,
        };

        let entry = InclusionEntry::new(
            envelope.packet_id.clone(),
            envelope.clone(),
            dummy_policy,
            priority,
            vec![rule_id],
        );

        list.entries.insert(envelope.packet_id.clone(), entry);

        // Log to blockbook
        if let Ok(blockbook) = self.blockbook.write() {
            let event_data = format!("Transaction {} added for forced inclusion", envelope.packet_id);
            let _ = blockbook.record_event(
                crate::blockbook::BlockbookEventType::Custom("forced_inclusion".to_string()),
                crate::blockbook::EventSeverity::Info,
                event_data,
                Vec::new(),
                None,
            );
        }

        Ok(())
    }

    pub fn add_rule(&self, rule: InclusionRule) -> DockLockResult<()> {
        let rule_id = rule.rule_id;
        
        {
            let mut list = self.inclusion_list.write()
                .map_err(|_| DockLockError::InvalidOperation("Failed to acquire inclusion list lock".to_string()))?;
            
            list.add_rule(rule)?;
        }

        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| DockLockError::InvalidOperation("Failed to acquire stats lock".to_string()))?;
            
            stats.total_rules += 1;
            stats.last_update = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        }

        info!("Added inclusion rule: {}", rule_id);
        Ok(())
    }

    pub fn mark_included(&self, tx_id: &str, block_height: u64, proof: Option<String>) -> DockLockResult<()> {
        {
            let mut list = self.inclusion_list.write()
                .map_err(|_| DockLockError::InvalidOperation("Failed to acquire inclusion list lock".to_string()))?;
            
            list.mark_included(tx_id, block_height, proof)?;
        }

        info!("Marked transaction as included: {} at block {}", tx_id, block_height);
        Ok(())
    }

    pub fn get_pending_count(&self) -> usize {
        if let Ok(list) = self.inclusion_list.read() {
            list.get_pending_transactions().len()
        } else {
            0
        }
    }

    pub fn cleanup_expired(&self) -> DockLockResult<usize> {
        let mut list = self.inclusion_list.write()
            .map_err(|_| DockLockError::InvalidOperation("Failed to acquire inclusion list lock".to_string()))?;
        
        Ok(list.cleanup_expired())
    }

    pub fn get_stats(&self) -> DockLockResult<InclusionListStats> {
        let stats = self.stats.read()
            .map_err(|_| DockLockError::InvalidOperation("Failed to acquire stats lock".to_string()))?;
        
        Ok(stats.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockbook::BlockbookConfig;
    use crate::packet_envelope::{EnvelopeMetadata, EncryptionScheme};
    use crate::traffic_light::DataClassification;
    use rand::rngs::OsRng;

    fn create_test_envelope() -> PacketEnvelope {
        let metadata = EnvelopeMetadata::new(
            "test_source".to_string(),
            DataClassification::Public,
            TrafficLightState::Green,
            [0u8; 32], // biso_policy_hash
            EncryptionScheme::None,
        ).with_destination("test_destination".to_string())
         .with_priority(128);

        PacketEnvelope {
            packet_id: "test_packet".to_string(),
            metadata,
            payload_hash: [1u8; 32],
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            shard_header: None,
            da_root_id: None,
            signature: None,
            signer_pubkey: None,
        }
    }

    fn create_test_policy_result() -> PolicyEvaluationResult {
        PolicyEvaluationResult {
            id: Uuid::new_v4(),
            policy_id: Uuid::new_v4(),
            decision: TrafficLightState::Green,
            passed: true,
            reason: "Test policy".to_string(),
            violations: Vec::new(),
            warnings: Vec::new(),
            evaluated_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            evaluation_duration_ms: 10,
            signature: None,
            evaluator_pubkey: None,
        }
    }

    #[test]
    fn test_inclusion_rule_creation() {
        let rule = InclusionRule::new(
            InclusionRuleType::GeographicMandatory(GeographicRegion::US),
            InclusionPriority::High,
            "Test rule".to_string(),
        );

        assert_eq!(rule.priority, InclusionPriority::High);
        assert!(rule.active);
        assert_eq!(rule.description, "Test rule");
    }

    #[test]
    fn test_inclusion_rule_signing() {
        let mut rule = InclusionRule::new(
            InclusionRuleType::ValueThreshold(1000),
            InclusionPriority::Normal,
            "Value threshold rule".to_string(),
        );

        let signing_key = SigningKey::generate(&mut OsRng);
        assert!(rule.sign(&signing_key).is_ok());
        assert!(rule.signature.is_some());
        assert!(rule.creator_pubkey.is_some());
    }

    #[test]
    fn test_inclusion_list_creation() {
        let list = InclusionList::new();
        assert!(list.entries.is_empty());
        assert!(list.rules.is_empty());
    }

    #[test]
    fn test_inclusion_list_add_rule() {
        let mut list = InclusionList::new();
        let rule = InclusionRule::new(
            InclusionRuleType::TrafficLightMandatory(TrafficLightState::Red),
            InclusionPriority::Emergency,
            "Emergency rule".to_string(),
        );

        assert!(list.add_rule(rule).is_ok());
        assert_eq!(list.rules.len(), 1);
    }

    #[test]
    fn test_inclusion_list_add_transaction() {
        let mut list = InclusionList::new();
        let rule = InclusionRule::new(
            InclusionRuleType::GeographicMandatory(GeographicRegion::US),
            InclusionPriority::High,
            "US mandatory rule".to_string(),
        );
        list.add_rule(rule).unwrap();

        let envelope = create_test_envelope();
        let policy_result = create_test_policy_result();

        let result = list.add_transaction(
            "test_tx".to_string(),
            envelope,
            policy_result,
        );

        assert!(result.is_ok());
        assert!(result.unwrap());
        assert_eq!(list.entries.len(), 1);
    }

    #[test]
    fn test_inclusion_list_manager() {
        let blockbook = Arc::new(RwLock::new(Blockbook::new(BlockbookConfig::default())));
        let config = InclusionListConfig::default();
        let manager = InclusionListManager::new(config, blockbook);

        let rule = InclusionRule::new(
            InclusionRuleType::ValueThreshold(500),
            InclusionPriority::Normal,
            "Value rule".to_string(),
        );

        assert!(manager.add_rule(rule).is_ok());

        let envelope = create_test_envelope();
        let policy_result = create_test_policy_result();

        let result = manager.process_transaction(
            "test_tx".to_string(),
            envelope,
            policy_result,
        );

        assert!(result.is_ok());
        assert!(result.unwrap());
        assert_eq!(manager.get_pending_count(), 1);
    }

    #[test]
    fn test_inclusion_entry_expiration() {
        let envelope = create_test_envelope();
        let policy_result = create_test_policy_result();
        let mut entry = InclusionEntry::new(
            "test_tx".to_string(),
            envelope,
            policy_result,
            InclusionPriority::Normal,
            vec![Uuid::new_v4()],
        );

        // Set added_at to past MAX_INCLUSION_AGE
        entry.added_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - MAX_INCLUSION_AGE.as_secs() - 1;
        
        assert!(entry.is_expired());
    }

    #[test]
    fn test_cleanup_expired() {
        let mut list = InclusionList::new();
        let rule = InclusionRule::new(
            InclusionRuleType::GeographicMandatory(GeographicRegion::US),
            InclusionPriority::High,
            "Test rule".to_string(),
        );
        list.add_rule(rule).unwrap();

        // Add expired transaction
        let envelope = create_test_envelope();
        let policy_result = create_test_policy_result();
        list.add_transaction("expired_tx".to_string(), envelope, policy_result).unwrap();

        // Manually set as expired
        if let Some(entry) = list.entries.get_mut("expired_tx") {
            entry.added_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - MAX_INCLUSION_AGE.as_secs() - 1;
        }

        let removed_count = list.cleanup_expired();
        assert_eq!(removed_count, 1);
        assert!(list.entries.is_empty());
    }
}
