//! Stage 42: Force-Inclusion Inbox
//! 
//! Provides a dedicated inbox system for forcing transaction inclusion at the consensus layer.
//! Integrates with Stage 41 Inclusion Lists to ensure mandatory transaction processing and
//! prevent censorship through a priority-based inbox mechanism.

use crate::error::{DockLockError, DockLockResult};
use crate::packet_envelope::PacketEnvelope;
use crate::inclusion_lists::{InclusionListManager, InclusionPriority, InclusionRuleType};
use crate::blockbook::Blockbook;
use ed25519_dalek::{SigningKey, Signer};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Domain separation constants for cryptographic hashing
const FORCE_INCLUSION_REQUEST_HASH: u8 = 0x33;
const FORCE_INCLUSION_RECEIPT_HASH: u8 = 0x34;

/// Force-inclusion request submitted to the inbox
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceInclusionRequest {
    /// Unique request identifier
    pub request_id: Uuid,
    /// Transaction envelope to force include
    pub envelope: PacketEnvelope,
    /// Priority level for inclusion
    pub priority: InclusionPriority,
    /// Reason for force inclusion
    pub reason: ForceInclusionReason,
    /// Submitter identity
    pub submitter: String,
    /// Timestamp when request was created
    pub timestamp: u64,
    /// Maximum time to wait for inclusion (seconds)
    pub timeout: u64,
    /// Cryptographic signature over request
    pub signature: Option<Vec<u8>>,
    /// Signer public key
    pub signer_pubkey: Option<Vec<u8>>,
}

/// Reasons for requesting force inclusion
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ForceInclusionReason {
    /// Emergency transaction requiring immediate inclusion
    Emergency,
    /// High-priority transaction from authorized entity
    HighPriority,
    /// Censorship resistance - transaction being unfairly excluded
    CensorshipResistance,
    /// Regulatory compliance requirement
    RegulatoryCompliance,
    /// Network maintenance or upgrade
    NetworkMaintenance,
    /// Custom reason with description
    Custom(String),
}

/// Status of a force inclusion request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ForceInclusionStatus {
    /// Request is pending processing
    Pending,
    /// Request has been accepted and queued for inclusion
    Accepted,
    /// Request has been included in a block
    Included,
    /// Request was rejected
    Rejected(String),
    /// Request timed out
    TimedOut,
    /// Request was cancelled
    Cancelled,
}

/// Receipt for force inclusion request processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceInclusionReceipt {
    /// Request ID this receipt corresponds to
    pub request_id: Uuid,
    /// Current status of the request
    pub status: ForceInclusionStatus,
    /// Timestamp when status was updated
    pub timestamp: u64,
    /// Block height where transaction was included (if applicable)
    pub block_height: Option<u64>,
    /// Transaction hash (if included)
    pub transaction_hash: Option<String>,
    /// Processing details or error message
    pub details: Option<String>,
    /// Cryptographic signature over receipt
    pub signature: Option<Vec<u8>>,
    /// Signer public key
    pub signer_pubkey: Option<Vec<u8>>,
}

/// Configuration for the force inclusion inbox
#[derive(Debug, Clone)]
pub struct ForceInclusionInboxConfig {
    /// Maximum number of pending requests
    pub max_pending_requests: usize,
    /// Maximum timeout for requests (seconds)
    pub max_timeout: u64,
    /// Cleanup interval for expired requests (seconds)
    pub cleanup_interval: u64,
    /// Maximum requests per submitter per hour
    pub rate_limit_per_hour: usize,
    /// Enable cryptographic signing of requests and receipts
    pub enable_signing: bool,
}

impl Default for ForceInclusionInboxConfig {
    fn default() -> Self {
        Self {
            max_pending_requests: 1000,
            max_timeout: 3600, // 1 hour
            cleanup_interval: 300, // 5 minutes
            rate_limit_per_hour: 10,
            enable_signing: true,
        }
    }
}

/// Statistics for the force inclusion inbox
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceInclusionStats {
    /// Total requests submitted
    pub total_requests: u64,
    /// Requests currently pending
    pub pending_requests: u64,
    /// Requests accepted and queued
    pub accepted_requests: u64,
    /// Requests successfully included
    pub included_requests: u64,
    /// Requests rejected
    pub rejected_requests: u64,
    /// Requests that timed out
    pub timed_out_requests: u64,
    /// Average processing time (seconds)
    pub avg_processing_time: f64,
    /// Requests by priority level
    pub requests_by_priority: HashMap<String, u64>,
    /// Requests by reason
    pub requests_by_reason: HashMap<String, u64>,
}

impl Default for ForceInclusionStats {
    fn default() -> Self {
        Self {
            total_requests: 0,
            pending_requests: 0,
            accepted_requests: 0,
            included_requests: 0,
            rejected_requests: 0,
            timed_out_requests: 0,
            avg_processing_time: 0.0,
            requests_by_priority: HashMap::new(),
            requests_by_reason: HashMap::new(),
        }
    }
}

/// Rate limiting tracker for submitters
#[derive(Debug, Clone)]
struct RateLimitEntry {
    /// Number of requests in current hour
    requests_count: usize,
    /// Start of current hour window
    window_start: u64,
}

/// Force inclusion inbox for managing mandatory transaction inclusion
#[derive(Debug)]
pub struct ForceInclusionInbox {
    /// Inbox configuration
    config: ForceInclusionInboxConfig,
    /// Pending requests queue (priority-ordered)
    pending_requests: Arc<RwLock<VecDeque<ForceInclusionRequest>>>,
    /// Request status tracking
    request_status: Arc<RwLock<HashMap<Uuid, ForceInclusionReceipt>>>,
    /// Rate limiting per submitter
    rate_limits: Arc<RwLock<HashMap<String, RateLimitEntry>>>,
    /// Inclusion list manager for rule enforcement
    inclusion_manager: Arc<InclusionListManager>,
    /// Blockbook for audit logging
    blockbook: Arc<RwLock<Blockbook>>,
    /// Inbox statistics
    stats: Arc<RwLock<ForceInclusionStats>>,
    /// Cryptographic signing key
    signing_key: Option<SigningKey>,
    /// Last cleanup timestamp
    last_cleanup: Arc<RwLock<u64>>,
}

impl ForceInclusionInbox {
    /// Create new force inclusion inbox
    pub fn new(
        config: ForceInclusionInboxConfig,
        inclusion_manager: Arc<InclusionListManager>,
        blockbook: Arc<RwLock<Blockbook>>,
    ) -> DockLockResult<Self> {
        let signing_key = if config.enable_signing {
            Some(SigningKey::generate(&mut rand::rngs::OsRng))
        } else {
            None
        };

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(Self {
            config,
            pending_requests: Arc::new(RwLock::new(VecDeque::new())),
            request_status: Arc::new(RwLock::new(HashMap::new())),
            rate_limits: Arc::new(RwLock::new(HashMap::new())),
            inclusion_manager,
            blockbook,
            stats: Arc::new(RwLock::new(ForceInclusionStats::default())),
            signing_key,
            last_cleanup: Arc::new(RwLock::new(current_time)),
        })
    }

    /// Submit a force inclusion request
    pub fn submit_request(&self, mut request: ForceInclusionRequest) -> DockLockResult<Uuid> {
        // Check rate limiting
        self.check_rate_limit(&request.submitter)?;

        // Validate request
        self.validate_request(&request)?;

        // Sign request if signing is enabled
        if self.config.enable_signing && self.signing_key.is_some() {
            self.sign_request(&mut request)?;
        }

        let request_id = request.request_id;

        // Add to pending queue (priority-ordered)
        {
            let mut pending = self.pending_requests.write()
                .map_err(|_| DockLockError::ConcurrencyError("Failed to acquire pending requests lock".to_string()))?;

            // Check capacity
            if pending.len() >= self.config.max_pending_requests {
                return Err(DockLockError::ValidationError("Inbox capacity exceeded".to_string()));
            }

            // Insert in priority order (Emergency first, then High, Normal, Low)
            let insert_pos = pending.iter().position(|r| {
                self.priority_value(&r.priority) < self.priority_value(&request.priority)
            }).unwrap_or(pending.len());

            pending.insert(insert_pos, request.clone());
        }

        // Create initial receipt
        let receipt = ForceInclusionReceipt {
            request_id,
            status: ForceInclusionStatus::Pending,
            timestamp: request.timestamp,
            block_height: None,
            transaction_hash: None,
            details: Some("Request submitted and queued".to_string()),
            signature: None,
            signer_pubkey: None,
        };

        // Store receipt
        {
            let mut status_map = self.request_status.write()
                .map_err(|_| DockLockError::ConcurrencyError("Failed to acquire status lock".to_string()))?;
            status_map.insert(request_id, receipt);
        }

        // Update statistics
        self.update_stats_for_submission(&request)?;

        // Log to blockbook
        self.log_request_to_blockbook(&request)?;

        Ok(request_id)
    }

    /// Process pending requests and create inclusion rules
    pub fn process_pending_requests(&self) -> DockLockResult<Vec<Uuid>> {
        let mut processed_requests = Vec::new();

        // Get next batch of requests to process
        let requests_to_process = {
            let mut pending = self.pending_requests.write()
                .map_err(|_| DockLockError::ConcurrencyError("Failed to acquire pending requests lock".to_string()))?;

            let batch_size = std::cmp::min(10, pending.len()); // Process up to 10 at a time
            let mut batch = Vec::new();

            for _ in 0..batch_size {
                if let Some(request) = pending.pop_front() {
                    batch.push(request);
                }
            }

            batch
        };

        // Process each request
        for request in requests_to_process {
            match self.process_single_request(&request) {
                Ok(()) => {
                    processed_requests.push(request.request_id);
                    self.update_request_status(request.request_id, ForceInclusionStatus::Accepted, None)?;
                }
                Err(e) => {
                    let error_msg = format!("Processing failed: {}", e);
                    self.update_request_status(
                        request.request_id,
                        ForceInclusionStatus::Rejected(error_msg.clone()),
                        Some(error_msg),
                    )?;
                }
            }
        }

        // Cleanup expired requests
        self.cleanup_expired_requests()?;

        Ok(processed_requests)
    }

    /// Get status of a force inclusion request
    pub fn get_request_status(&self, request_id: Uuid) -> DockLockResult<Option<ForceInclusionReceipt>> {
        let status_map = self.request_status.read()
            .map_err(|_| DockLockError::ConcurrencyError("Failed to acquire status lock".to_string()))?;

        Ok(status_map.get(&request_id).cloned())
    }

    /// Get current inbox statistics
    pub fn get_statistics(&self) -> DockLockResult<ForceInclusionStats> {
        let stats = self.stats.read()
            .map_err(|_| DockLockError::ConcurrencyError("Failed to acquire stats lock".to_string()))?;

        Ok(stats.clone())
    }

    /// Cancel a pending request
    pub fn cancel_request(&self, request_id: Uuid, submitter: &str) -> DockLockResult<bool> {
        // Remove from pending queue if present
        let mut removed = false;
        {
            let mut pending = self.pending_requests.write()
                .map_err(|_| DockLockError::ConcurrencyError("Failed to acquire pending requests lock".to_string()))?;

            if let Some(pos) = pending.iter().position(|r| r.request_id == request_id && r.submitter == submitter) {
                pending.remove(pos);
                removed = true;
            }
        }

        if removed {
            self.update_request_status(
                request_id,
                ForceInclusionStatus::Cancelled,
                Some("Request cancelled by submitter".to_string()),
            )?;
        }

        Ok(removed)
    }

    // Private helper methods

    fn check_rate_limit(&self, submitter: &str) -> DockLockResult<()> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let current_hour = current_time / 3600;

        let mut rate_limits = self.rate_limits.write()
            .map_err(|_| DockLockError::ConcurrencyError("Failed to acquire rate limits lock".to_string()))?;

        let entry = rate_limits.entry(submitter.to_string()).or_insert(RateLimitEntry {
            requests_count: 0,
            window_start: current_hour,
        });

        // Reset counter if we're in a new hour
        if entry.window_start < current_hour {
            entry.requests_count = 0;
            entry.window_start = current_hour;
        }

        // Check rate limit
        if entry.requests_count >= self.config.rate_limit_per_hour {
            return Err(DockLockError::ValidationError("Rate limit exceeded".to_string()));
        }

        entry.requests_count += 1;
        Ok(())
    }

    fn validate_request(&self, request: &ForceInclusionRequest) -> DockLockResult<()> {
        // Check timeout
        if request.timeout > self.config.max_timeout {
            return Err(DockLockError::ValidationError("Timeout exceeds maximum allowed".to_string()));
        }

        // Check if request is already expired
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if request.timestamp + request.timeout < current_time {
            return Err(DockLockError::ValidationError("Request is already expired".to_string()));
        }

        // Validate submitter
        if request.submitter.is_empty() {
            return Err(DockLockError::ValidationError("Submitter cannot be empty".to_string()));
        }

        // Validate packet ID
        if request.envelope.packet_id.is_empty() {
            return Err(DockLockError::ValidationError("Packet ID cannot be empty".to_string()));
        }

        Ok(())
    }

    fn sign_request(&self, request: &mut ForceInclusionRequest) -> DockLockResult<()> {
        if let Some(signing_key) = &self.signing_key {
            let hash = self.compute_request_hash(request)?;
            let signature = signing_key.sign(&hash);

            request.signature = Some(signature.to_bytes().to_vec());
            request.signer_pubkey = Some(signing_key.verifying_key().to_bytes().to_vec());
        }

        Ok(())
    }

    fn compute_request_hash(&self, request: &ForceInclusionRequest) -> DockLockResult<[u8; 32]> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[FORCE_INCLUSION_REQUEST_HASH]);

        hasher.update(request.request_id.as_bytes());
        hasher.update(&bincode::serialize(&request.envelope)
            .map_err(|e| DockLockError::EncodingError(format!("Failed to serialize envelope: {}", e)))?);
        hasher.update(&bincode::serialize(&request.priority)
            .map_err(|e| DockLockError::EncodingError(format!("Failed to serialize priority: {}", e)))?);
        hasher.update(&bincode::serialize(&request.reason)
            .map_err(|e| DockLockError::EncodingError(format!("Failed to serialize reason: {}", e)))?);
        hasher.update(request.submitter.as_bytes());
        hasher.update(&request.timestamp.to_le_bytes());
        hasher.update(&request.timeout.to_le_bytes());

        Ok(*hasher.finalize().as_bytes())
    }

    fn priority_value(&self, priority: &InclusionPriority) -> u8 {
        match priority {
            InclusionPriority::Emergency => 4,
            InclusionPriority::High => 3,
            InclusionPriority::Normal => 2,
            InclusionPriority::Low => 1,
        }
    }

    fn process_single_request(&self, request: &ForceInclusionRequest) -> DockLockResult<()> {
        // Create inclusion rule based on the request
        let rule_type = match &request.reason {
            ForceInclusionReason::Emergency => InclusionRuleType::Emergency,
            ForceInclusionReason::HighPriority => InclusionRuleType::HighPriority,
            ForceInclusionReason::CensorshipResistance => InclusionRuleType::CensorshipResistance,
            ForceInclusionReason::RegulatoryCompliance => InclusionRuleType::RegulatoryCompliance,
            ForceInclusionReason::NetworkMaintenance => InclusionRuleType::NetworkMaintenance,
            ForceInclusionReason::Custom(_) => InclusionRuleType::Custom,
        };

        // Add to inclusion list manager
        self.inclusion_manager.add_transaction_for_inclusion(
            &request.envelope,
            request.priority.clone(),
            rule_type,
        )?;

        Ok(())
    }

    fn update_request_status(
        &self,
        request_id: Uuid,
        status: ForceInclusionStatus,
        details: Option<String>,
    ) -> DockLockResult<()> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut receipt = ForceInclusionReceipt {
            request_id,
            status: status.clone(),
            timestamp: current_time,
            block_height: None,
            transaction_hash: None,
            details,
            signature: None,
            signer_pubkey: None,
        };

        // Sign receipt if signing is enabled
        if self.config.enable_signing && self.signing_key.is_some() {
            self.sign_receipt(&mut receipt)?;
        }

        // Update status map
        {
            let mut status_map = self.request_status.write()
                .map_err(|_| DockLockError::ConcurrencyError("Failed to acquire status lock".to_string()))?;
            status_map.insert(request_id, receipt);
        }

        // Update statistics
        self.update_stats_for_status_change(&status)?;

        Ok(())
    }

    fn sign_receipt(&self, receipt: &mut ForceInclusionReceipt) -> DockLockResult<()> {
        if let Some(signing_key) = &self.signing_key {
            let hash = self.compute_receipt_hash(receipt)?;
            let signature = signing_key.sign(&hash);

            receipt.signature = Some(signature.to_bytes().to_vec());
            receipt.signer_pubkey = Some(signing_key.verifying_key().to_bytes().to_vec());
        }

        Ok(())
    }

    fn compute_receipt_hash(&self, receipt: &ForceInclusionReceipt) -> DockLockResult<[u8; 32]> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[FORCE_INCLUSION_RECEIPT_HASH]);

        hasher.update(receipt.request_id.as_bytes());
        hasher.update(&bincode::serialize(&receipt.status)
            .map_err(|e| DockLockError::EncodingError(format!("Failed to serialize status: {}", e)))?);
        hasher.update(&receipt.timestamp.to_le_bytes());

        if let Some(block_height) = receipt.block_height {
            hasher.update(&block_height.to_le_bytes());
        }

        if let Some(tx_hash) = &receipt.transaction_hash {
            hasher.update(tx_hash.as_bytes());
        }

        if let Some(details) = &receipt.details {
            hasher.update(details.as_bytes());
        }

        Ok(*hasher.finalize().as_bytes())
    }

    fn update_stats_for_submission(&self, request: &ForceInclusionRequest) -> DockLockResult<()> {
        let mut stats = self.stats.write()
            .map_err(|_| DockLockError::ConcurrencyError("Failed to acquire stats lock".to_string()))?;

        stats.total_requests += 1;
        stats.pending_requests += 1;

        let priority_key = format!("{:?}", request.priority);
        *stats.requests_by_priority.entry(priority_key).or_insert(0) += 1;

        let reason_key = match &request.reason {
            ForceInclusionReason::Custom(desc) => format!("Custom({})", desc),
            other => format!("{:?}", other),
        };
        *stats.requests_by_reason.entry(reason_key).or_insert(0) += 1;

        Ok(())
    }

    fn update_stats_for_status_change(&self, status: &ForceInclusionStatus) -> DockLockResult<()> {
        let mut stats = self.stats.write()
            .map_err(|_| DockLockError::ConcurrencyError("Failed to acquire stats lock".to_string()))?;

        match status {
            ForceInclusionStatus::Accepted => {
                if stats.pending_requests > 0 {
                    stats.pending_requests -= 1;
                }
                stats.accepted_requests += 1;
            }
            ForceInclusionStatus::Included => {
                if stats.accepted_requests > 0 {
                    stats.accepted_requests -= 1;
                }
                stats.included_requests += 1;
            }
            ForceInclusionStatus::Rejected(_) => {
                if stats.pending_requests > 0 {
                    stats.pending_requests -= 1;
                }
                stats.rejected_requests += 1;
            }
            ForceInclusionStatus::TimedOut => {
                if stats.pending_requests > 0 {
                    stats.pending_requests -= 1;
                }
                stats.timed_out_requests += 1;
            }
            ForceInclusionStatus::Cancelled => {
                if stats.pending_requests > 0 {
                    stats.pending_requests -= 1;
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn log_request_to_blockbook(&self, request: &ForceInclusionRequest) -> DockLockResult<()> {
        if let Ok(blockbook) = self.blockbook.write() {
            let event_data = format!(
                "Force inclusion request submitted: {} (priority: {:?}, reason: {:?})",
                request.request_id, request.priority, request.reason
            );
            
            let _ = blockbook.record_event(
                crate::blockbook::BlockbookEventType::Custom("force_inclusion_request".to_string()),
                crate::blockbook::EventSeverity::Info,
                event_data,
                Vec::new(),
                None,
            );
        }
        Ok(())
    }

    fn cleanup_expired_requests(&self) -> DockLockResult<()> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let last_cleanup = *self.last_cleanup.read()
            .map_err(|_| DockLockError::ConcurrencyError("Failed to acquire cleanup lock".to_string()))?;

        if current_time - last_cleanup < self.config.cleanup_interval {
            return Ok(());
        }

        let expired_requests = {
            let mut pending = self.pending_requests.write()
                .map_err(|_| DockLockError::ConcurrencyError("Failed to acquire pending requests lock".to_string()))?;

            let mut expired = Vec::new();
            pending.retain(|request| {
                let is_expired = request.timestamp + request.timeout < current_time;
                if is_expired {
                    expired.push(request.request_id);
                }
                !is_expired
            });

            expired
        };

        for request_id in expired_requests {
            self.update_request_status(
                request_id,
                ForceInclusionStatus::TimedOut,
                Some("Request expired".to_string()),
            )?;
        }

        let mut last_cleanup = self.last_cleanup.write()
            .map_err(|_| DockLockError::ConcurrencyError("Failed to acquire cleanup lock".to_string()))?;
        *last_cleanup = current_time;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockbook::BlockbookConfig;
    use crate::packet_envelope::{EnvelopeMetadata, EncryptionScheme};
    use crate::traffic_light::{DataClassification, TrafficLightState};

    fn create_test_components() -> (Arc<InclusionListManager>, Arc<RwLock<Blockbook>>) {
        let blockbook_config = BlockbookConfig::default();
        let blockbook = Arc::new(RwLock::new(Blockbook::new(blockbook_config)));
        let inclusion_config = crate::inclusion_lists::InclusionListConfig::default();
        let inclusion_manager = Arc::new(InclusionListManager::new(inclusion_config, blockbook.clone()));
        (inclusion_manager, blockbook)
    }

    fn create_test_request() -> ForceInclusionRequest {
        let metadata = EnvelopeMetadata::new(
            "test_source".to_string(),
            DataClassification::Public,
            TrafficLightState::Green,
            [0u8; 32],
            EncryptionScheme::None,
        );

        let envelope = PacketEnvelope {
            packet_id: "test_packet".to_string(),
            metadata,
            payload_hash: [1u8; 32],
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            shard_header: None,
            da_root_id: None,
            signature: None,
            signer_pubkey: None,
        };

        ForceInclusionRequest {
            request_id: Uuid::new_v4(),
            envelope,
            priority: InclusionPriority::Normal,
            reason: ForceInclusionReason::CensorshipResistance,
            submitter: "test_submitter".to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            timeout: 3600,
            signature: None,
            signer_pubkey: None,
        }
    }

    #[test]
    fn test_force_inclusion_inbox_creation() {
        let config = ForceInclusionInboxConfig::default();
        let (inclusion_manager, blockbook) = create_test_components();

        let inbox = ForceInclusionInbox::new(config, inclusion_manager, blockbook);
        assert!(inbox.is_ok());
    }

    #[test]
    fn test_submit_request() {
        let config = ForceInclusionInboxConfig::default();
        let (inclusion_manager, blockbook) = create_test_components();
        let inbox = ForceInclusionInbox::new(config, inclusion_manager, blockbook).unwrap();

        let request = create_test_request();
        let request_id = request.request_id;

        let result = inbox.submit_request(request);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), request_id);

        let status = inbox.get_request_status(request_id).unwrap();
        assert!(status.is_some());
        assert_eq!(status.unwrap().status, ForceInclusionStatus::Pending);
    }

    #[test]
    fn test_process_pending_requests() {
        let config = ForceInclusionInboxConfig::default();
        let (inclusion_manager, blockbook) = create_test_components();
        let inbox = ForceInclusionInbox::new(config, inclusion_manager, blockbook).unwrap();

        let request = create_test_request();
        let request_id = request.request_id;

        inbox.submit_request(request).unwrap();

        let processed = inbox.process_pending_requests().unwrap();
        assert_eq!(processed.len(), 1);
        assert_eq!(processed[0], request_id);

        let status = inbox.get_request_status(request_id).unwrap();
        assert!(status.is_some());
        assert_eq!(status.unwrap().status, ForceInclusionStatus::Accepted);
    }

    #[test]
    fn test_request_priority_ordering() {
        let config = ForceInclusionInboxConfig::default();
        let (inclusion_manager, blockbook) = create_test_components();
        let inbox = ForceInclusionInbox::new(config, inclusion_manager, blockbook).unwrap();

        let mut low_request = create_test_request();
        low_request.priority = InclusionPriority::Low;
        let low_id = low_request.request_id;

        let mut emergency_request = create_test_request();
        emergency_request.priority = InclusionPriority::Emergency;
        let emergency_id = emergency_request.request_id;

        inbox.submit_request(low_request).unwrap();
        inbox.submit_request(emergency_request).unwrap();

        let processed = inbox.process_pending_requests().unwrap();
        assert_eq!(processed.len(), 2);
        assert_eq!(processed[0], emergency_id);
        assert_eq!(processed[1], low_id);
    }

    #[test]
    fn test_cancel_request() {
        let config = ForceInclusionInboxConfig::default();
        let (inclusion_manager, blockbook) = create_test_components();
        let inbox = ForceInclusionInbox::new(config, inclusion_manager, blockbook).unwrap();

        let request = create_test_request();
        let request_id = request.request_id;
        let submitter = request.submitter.clone();

        inbox.submit_request(request).unwrap();

        let cancelled = inbox.cancel_request(request_id, &submitter).unwrap();
        assert!(cancelled);

        let status = inbox.get_request_status(request_id).unwrap();
        assert!(status.is_some());
        assert_eq!(status.unwrap().status, ForceInclusionStatus::Cancelled);
    }

    #[test]
    fn test_statistics_tracking() {
        let config = ForceInclusionInboxConfig::default();
        let (inclusion_manager, blockbook) = create_test_components();
        let inbox = ForceInclusionInbox::new(config, inclusion_manager, blockbook).unwrap();

        let request = create_test_request();
        inbox.submit_request(request).unwrap();

        let stats = inbox.get_statistics().unwrap();
        assert_eq!(stats.total_requests, 1);
        assert_eq!(stats.pending_requests, 1);
        assert!(stats.requests_by_priority.contains_key("Normal"));
        assert!(stats.requests_by_reason.contains_key("CensorshipResistance"));
    }

    #[test]
    fn test_rate_limiting() {
        let mut config = ForceInclusionInboxConfig::default();
        config.rate_limit_per_hour = 2;
        
        let (inclusion_manager, blockbook) = create_test_components();
        let inbox = ForceInclusionInbox::new(config, inclusion_manager, blockbook).unwrap();

        // First two requests should succeed
        let mut request1 = create_test_request();
        request1.submitter = "rate_limited_user".to_string();
        assert!(inbox.submit_request(request1).is_ok());

        let mut request2 = create_test_request();
        request2.submitter = "rate_limited_user".to_string();
        assert!(inbox.submit_request(request2).is_ok());

        // Third request should fail due to rate limiting
        let mut request3 = create_test_request();
        request3.submitter = "rate_limited_user".to_string();
        assert!(inbox.submit_request(request3).is_err());
    }

    #[test]
    fn test_request_validation() {
        let config = ForceInclusionInboxConfig::default();
        let (inclusion_manager, blockbook) = create_test_components();
        let inbox = ForceInclusionInbox::new(config, inclusion_manager, blockbook).unwrap();

        // Test with empty packet ID
        let mut invalid_request = create_test_request();
        invalid_request.envelope.packet_id = "".to_string();
        assert!(inbox.submit_request(invalid_request).is_err());

        // Test with empty submitter
        let mut invalid_request2 = create_test_request();
        invalid_request2.submitter = "".to_string();
        assert!(inbox.submit_request(invalid_request2).is_err());
    }
}
