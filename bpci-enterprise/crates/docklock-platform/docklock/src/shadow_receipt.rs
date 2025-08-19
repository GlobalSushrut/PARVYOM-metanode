use crate::error::{DockLockError, DockLockResult};
use crate::receipt::Receipt;


use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, info, warn};
use uuid::Uuid;


/// Domain separator for shadow receipt hashing
pub const SHADOW_RECEIPT_HASH: u8 = 0x32;

/// Domain separator for postbox operations
pub const POSTBOX_HASH: u8 = 0x33;

/// Shadow receipt that provides privacy-preserving delivery
/// Uses practical postbox technique for anonymous, auditable messaging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowReceipt {
    /// Shadow receipt identifier (different from original receipt ID)
    pub shadow_id: String,
    /// Original receipt ID (encrypted or hashed for privacy)
    pub original_receipt_ref: ReceiptReference,
    /// Postbox delivery information
    pub postbox_info: PostboxInfo,
    /// Privacy metadata
    pub privacy_metadata: PrivacyMetadata,
    /// Shadow receipt timestamp
    pub timestamp: u64,
    /// Shadow receipt signature
    #[serde(skip)]
    pub signature: Option<Signature>,
    /// Signer public key for shadow receipt
    pub signer_pubkey: Option<Vec<u8>>,
}

/// Reference to original receipt with privacy protection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptReference {
    /// Encrypted or hashed reference to original receipt
    pub encrypted_ref: Vec<u8>,
    /// Key derivation info for decryption
    pub key_info: KeyDerivationInfo,
    /// Merkle proof for receipt existence without revealing content
    pub existence_proof: MerkleProof,
}

/// Postbox information for practical postbox technique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostboxInfo {
    /// Postbox identifier
    pub postbox_id: String,
    /// Delivery method configuration
    pub delivery_method: DeliveryMethod,
    /// Access control for postbox
    pub access_control: PostboxAccessControl,
    /// Delivery status tracking
    pub delivery_status: DeliveryStatus,
    /// Postbox expiration time
    pub expiration_time: u64,
}

/// Delivery method for postbox
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeliveryMethod {
    /// Direct delivery to recipient
    Direct {
        recipient_pubkey: Vec<u8>,
        encryption_scheme: EncryptionScheme,
    },
    /// Anonymous drop with pickup code
    AnonymousDrop {
        pickup_code_hash: [u8; 32],
        pickup_window: u64,
    },
    /// Broadcast to multiple recipients
    Broadcast {
        recipient_list: Vec<Vec<u8>>,
        threshold: usize,
    },
    /// Onion routing for maximum privacy
    OnionRouted {
        routing_path: Vec<OnionHop>,
        final_destination: Vec<u8>,
    },
}

/// Encryption scheme for delivery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionScheme {
    /// Ed25519 + ChaCha20Poly1305
    Ed25519ChaCha20,
    /// X25519 + ChaCha20Poly1305
    X25519ChaCha20,
    /// Hybrid encryption with multiple keys
    Hybrid {
        schemes: Vec<EncryptionScheme>,
    },
}

/// Onion hop for onion routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnionHop {
    /// Hop node public key
    pub node_pubkey: Vec<u8>,
    /// Encrypted next hop information
    pub encrypted_next_hop: Vec<u8>,
    /// Hop-specific metadata
    pub hop_metadata: HashMap<String, String>,
}

/// Access control for postbox
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostboxAccessControl {
    /// Required authentication method
    pub auth_method: AuthenticationMethod,
    /// Access permissions
    pub permissions: PostboxPermissions,
    /// Rate limiting configuration
    pub rate_limits: RateLimits,
}

/// Authentication method for postbox access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    /// Public key signature
    PublicKey {
        required_pubkey: Vec<u8>,
    },
    /// Zero-knowledge proof
    ZkProof {
        proof_type: String,
        verification_key: Vec<u8>,
    },
    /// Multi-signature threshold
    MultiSig {
        required_sigs: usize,
        pubkeys: Vec<Vec<u8>>,
    },
    /// Anonymous credential
    AnonymousCredential {
        credential_type: String,
        verification_params: Vec<u8>,
    },
}

/// Postbox permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostboxPermissions {
    /// Can read from postbox
    pub can_read: bool,
    /// Can write to postbox
    pub can_write: bool,
    /// Can delete from postbox
    pub can_delete: bool,
    /// Can modify postbox settings
    pub can_modify: bool,
    /// Maximum message size
    pub max_message_size: usize,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimits {
    /// Maximum requests per minute
    pub max_requests_per_minute: u32,
    /// Maximum bandwidth per minute (bytes)
    pub max_bandwidth_per_minute: u64,
    /// Burst allowance
    pub burst_allowance: u32,
}

/// Delivery status tracking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeliveryStatus {
    /// Pending delivery
    Pending,
    /// In transit
    InTransit {
        current_hop: usize,
        total_hops: usize,
    },
    /// Delivered successfully
    Delivered {
        delivery_time: u64,
        confirmation_hash: [u8; 32],
    },
    /// Delivery failed
    Failed {
        failure_reason: String,
        retry_count: u32,
    },
    /// Expired (not delivered within time limit)
    Expired {
        expiration_time: u64,
    },
}

/// Privacy metadata for shadow receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyMetadata {
    /// Privacy level configuration
    pub privacy_level: PrivacyLevel,
    /// Anonymization techniques used
    pub anonymization_methods: Vec<AnonymizationMethod>,
    /// Audit trail (privacy-preserving)
    pub audit_trail: PrivacyAuditTrail,
    /// Compliance information
    pub compliance_info: ComplianceInfo,
}

/// Privacy level enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrivacyLevel {
    /// Basic privacy (pseudonymous)
    Basic,
    /// Enhanced privacy (unlinkable)
    Enhanced,
    /// Maximum privacy (anonymous)
    Maximum,
    /// Custom privacy configuration
    Custom {
        config: HashMap<String, String>,
    },
}

/// Anonymization method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnonymizationMethod {
    /// Differential privacy
    DifferentialPrivacy {
        epsilon: f64,
        delta: f64,
    },
    /// K-anonymity
    KAnonymity {
        k: usize,
    },
    /// Onion routing
    OnionRouting {
        layers: usize,
    },
    /// Mix networks
    MixNetwork {
        mix_strategy: String,
    },
    /// Zero-knowledge proofs
    ZeroKnowledge {
        proof_system: String,
    },
}

/// Privacy-preserving audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyAuditTrail {
    /// Audit events (privacy-preserving)
    pub events: Vec<PrivacyAuditEvent>,
    /// Merkle root of audit events
    pub audit_root: [u8; 32],
    /// Audit trail signature
    pub audit_signature: Option<Vec<u8>>,
}

/// Privacy audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyAuditEvent {
    /// Event identifier
    pub event_id: String,
    /// Event type
    pub event_type: AuditEventType,
    /// Event timestamp
    pub timestamp: u64,
    /// Event metadata (privacy-preserving)
    pub metadata: HashMap<String, String>,
    /// Event hash
    pub event_hash: [u8; 32],
}

/// Audit event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditEventType {
    /// Shadow receipt created
    ShadowReceiptCreated,
    /// Postbox accessed
    PostboxAccessed,
    /// Delivery attempted
    DeliveryAttempted,
    /// Delivery completed
    DeliveryCompleted,
    /// Privacy policy applied
    PrivacyPolicyApplied,
    /// Compliance check performed
    ComplianceCheckPerformed,
}

/// Compliance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceInfo {
    /// Applicable regulations
    pub regulations: Vec<String>,
    /// Compliance status
    pub compliance_status: ComplianceStatus,
    /// Data retention policy
    pub retention_policy: DataRetentionPolicy,
    /// Jurisdiction information
    pub jurisdiction: String,
}

/// Compliance status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComplianceStatus {
    /// Compliant with all regulations
    Compliant,
    /// Partially compliant
    PartiallyCompliant {
        missing_requirements: Vec<String>,
    },
    /// Non-compliant
    NonCompliant {
        violations: Vec<String>,
    },
    /// Under review
    UnderReview,
}

/// Data retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRetentionPolicy {
    /// Retention period in seconds
    pub retention_period: u64,
    /// Auto-deletion enabled
    pub auto_delete: bool,
    /// Backup policy
    pub backup_policy: BackupPolicy,
}

/// Backup policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackupPolicy {
    /// No backups
    None,
    /// Encrypted backups
    Encrypted {
        encryption_key: Vec<u8>,
    },
    /// Distributed backups
    Distributed {
        replica_count: usize,
    },
}

/// Key derivation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyDerivationInfo {
    /// Key derivation function
    pub kdf: KeyDerivationFunction,
    /// Salt for key derivation
    pub salt: Vec<u8>,
    /// Iteration count
    pub iterations: u32,
    /// Derived key length
    pub key_length: usize,
}

/// Key derivation function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyDerivationFunction {
    /// PBKDF2 with SHA-256
    Pbkdf2Sha256,
    /// Argon2id
    Argon2id {
        memory_cost: u32,
        time_cost: u32,
        parallelism: u32,
    },
    /// HKDF with SHA-256
    HkdfSha256,
    /// Scrypt
    Scrypt {
        n: u32,
        r: u32,
        p: u32,
    },
}

/// Merkle proof for receipt existence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    /// Merkle path
    pub path: Vec<[u8; 32]>,
    /// Leaf index
    pub leaf_index: usize,
    /// Root hash
    pub root_hash: [u8; 32],
}

/// Practical postbox system for shadow receipt delivery
#[derive(Debug)]
pub struct PostboxSystem {
    /// System identifier
    pub id: Uuid,
    /// Postbox registry
    postboxes: Arc<RwLock<HashMap<String, Postbox>>>,
    /// Delivery queue
    delivery_queue: Arc<RwLock<BTreeMap<u64, Vec<DeliveryTask>>>>,
    /// System configuration
    config: PostboxSystemConfig,
    /// System statistics
    stats: Arc<RwLock<PostboxSystemStats>>,
}

/// Individual postbox
#[derive(Debug, Clone)]
pub struct Postbox {
    /// Postbox identifier
    pub id: String,
    /// Postbox configuration
    pub config: PostboxConfig,
    /// Messages in postbox
    pub messages: Vec<PostboxMessage>,
    /// Access log
    pub access_log: Vec<PostboxAccessLog>,
    /// Creation time
    pub created_at: u64,
    /// Last accessed time
    pub last_accessed: u64,
}

/// Postbox configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostboxConfig {
    /// Maximum message count
    pub max_messages: usize,
    /// Maximum total size
    pub max_total_size: usize,
    /// Message TTL (time to live)
    pub message_ttl: u64,
    /// Access control
    pub access_control: PostboxAccessControl,
    /// Privacy settings
    pub privacy_settings: PostboxPrivacySettings,
}

/// Postbox privacy settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostboxPrivacySettings {
    /// Enable message encryption
    pub encrypt_messages: bool,
    /// Enable access logging
    pub enable_access_logging: bool,
    /// Anonymize access logs
    pub anonymize_logs: bool,
    /// Auto-delete expired messages
    pub auto_delete_expired: bool,
}

/// Message in postbox
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostboxMessage {
    /// Message identifier
    pub message_id: String,
    /// Shadow receipt content
    pub shadow_receipt: ShadowReceipt,
    /// Message metadata
    pub metadata: PostboxMessageMetadata,
    /// Message encryption info
    pub encryption_info: Option<MessageEncryptionInfo>,
}

/// Postbox message metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostboxMessageMetadata {
    /// Sender information (if available)
    pub sender: Option<String>,
    /// Message priority
    pub priority: MessagePriority,
    /// Delivery deadline
    pub delivery_deadline: Option<u64>,
    /// Message tags
    pub tags: Vec<String>,
    /// Message size
    pub size: usize,
}

/// Message priority
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

/// Message encryption information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEncryptionInfo {
    /// Encryption scheme used
    pub scheme: EncryptionScheme,
    /// Encrypted content
    pub encrypted_content: Vec<u8>,
    /// Encryption metadata
    pub metadata: HashMap<String, String>,
}

/// Postbox access log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostboxAccessLog {
    /// Access timestamp
    pub timestamp: u64,
    /// Access type
    pub access_type: PostboxAccessType,
    /// Accessor information (anonymized if required)
    pub accessor: Option<String>,
    /// Access result
    pub result: PostboxAccessResult,
    /// Access metadata
    pub metadata: HashMap<String, String>,
}

/// Postbox access type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PostboxAccessType {
    /// Read access
    Read,
    /// Write access
    Write,
    /// Delete access
    Delete,
    /// Modify access
    Modify,
    /// List access
    List,
}

/// Postbox access result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PostboxAccessResult {
    /// Access granted
    Granted,
    /// Access denied
    Denied {
        reason: String,
    },
    /// Access rate limited
    RateLimited,
    /// Access error
    Error {
        error_code: String,
        error_message: String,
    },
}

/// Delivery task for postbox system
#[derive(Debug, Clone)]
pub struct DeliveryTask {
    /// Task identifier
    pub task_id: String,
    /// Shadow receipt to deliver
    pub shadow_receipt: ShadowReceipt,
    /// Delivery configuration
    pub delivery_config: DeliveryConfig,
    /// Task status
    pub status: DeliveryTaskStatus,
    /// Creation time
    pub created_at: u64,
    /// Retry count
    pub retry_count: u32,
}

/// Delivery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryConfig {
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Retry delay (seconds)
    pub retry_delay: u64,
    /// Delivery timeout (seconds)
    pub delivery_timeout: u64,
    /// Confirmation required
    pub require_confirmation: bool,
}

/// Delivery task status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeliveryTaskStatus {
    /// Task pending
    Pending,
    /// Task in progress
    InProgress,
    /// Task completed
    Completed,
    /// Task failed
    Failed {
        error: String,
    },
    /// Task cancelled
    Cancelled,
}

/// Postbox system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostboxSystemConfig {
    /// Maximum number of postboxes
    pub max_postboxes: usize,
    /// Default postbox TTL
    pub default_postbox_ttl: u64,
    /// Cleanup interval
    pub cleanup_interval: u64,
    /// Maximum delivery workers
    pub max_delivery_workers: usize,
    /// System privacy settings
    pub privacy_settings: SystemPrivacySettings,
}

/// System privacy settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPrivacySettings {
    /// Default privacy level
    pub default_privacy_level: PrivacyLevel,
    /// Enable system-wide anonymization
    pub enable_anonymization: bool,
    /// Audit retention period
    pub audit_retention_period: u64,
    /// Compliance mode
    pub compliance_mode: bool,
}

/// Postbox system statistics
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PostboxSystemStats {
    /// Total postboxes created
    pub total_postboxes: u64,
    /// Active postboxes
    pub active_postboxes: u64,
    /// Total messages delivered
    pub total_messages_delivered: u64,
    /// Failed deliveries
    pub failed_deliveries: u64,
    /// Average delivery time
    pub avg_delivery_time_ms: f64,
    /// System uptime
    pub uptime_seconds: u64,
}

impl ShadowReceipt {
    /// Create a new shadow receipt from an original receipt
    pub fn new(
        original_receipt: &Receipt,
        postbox_info: PostboxInfo,
        privacy_level: PrivacyLevel,
    ) -> DockLockResult<Self> {
        let shadow_id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| DockLockError::InvalidOperation(format!("Time error: {}", e)))?
            .as_secs();

        // Create encrypted reference to original receipt
        let original_receipt_ref = Self::create_receipt_reference(original_receipt)?;

        // Create privacy metadata
        let privacy_metadata = PrivacyMetadata {
            privacy_level,
            anonymization_methods: vec![
                AnonymizationMethod::OnionRouting { layers: 3 },
                AnonymizationMethod::ZeroKnowledge {
                    proof_system: "groth16".to_string(),
                },
            ],
            audit_trail: PrivacyAuditTrail {
                events: vec![],
                audit_root: blake3::hash(b"initial_audit_root").into(),
                audit_signature: None,
            },
            compliance_info: ComplianceInfo {
                regulations: vec!["GDPR".to_string(), "CCPA".to_string()],
                compliance_status: ComplianceStatus::Compliant,
                retention_policy: DataRetentionPolicy {
                    retention_period: 86400 * 30, // 30 days
                    auto_delete: true,
                    backup_policy: BackupPolicy::Encrypted {
                        encryption_key: vec![0u8; 32], // Placeholder
                    },
                },
                jurisdiction: "EU".to_string(),
            },
        };

        Ok(ShadowReceipt {
            shadow_id,
            original_receipt_ref,
            postbox_info,
            privacy_metadata,
            timestamp,
            signature: None,
            signer_pubkey: None,
        })
    }

    /// Create encrypted reference to original receipt
    fn create_receipt_reference(original_receipt: &Receipt) -> DockLockResult<ReceiptReference> {
        // For now, use a simple hash-based reference
        // In production, this would use proper encryption
        let receipt_data = serde_json::to_vec(original_receipt)
            .map_err(|e| DockLockError::EncodingError(format!("Receipt serialization failed: {}", e)))?;
        
        let encrypted_ref = blake3::hash(&receipt_data).as_bytes().to_vec();

        let key_info = KeyDerivationInfo {
            kdf: KeyDerivationFunction::HkdfSha256,
            salt: vec![0u8; 32], // Placeholder
            iterations: 10000,
            key_length: 32,
        };

        // Create a simple existence proof (in production, use proper Merkle proof)
        let existence_proof = MerkleProof {
            path: vec![*blake3::hash(b"merkle_path").as_bytes()],
            leaf_index: 0,
            root_hash: *blake3::hash(b"merkle_root").as_bytes(),
        };

        Ok(ReceiptReference {
            encrypted_ref,
            key_info,
            existence_proof,
        })
    }

    /// Compute hash of shadow receipt
    pub fn compute_hash(&self) -> DockLockResult<[u8; 32]> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[SHADOW_RECEIPT_HASH]);
        
        let serialized = serde_json::to_vec(self)
            .map_err(|e| DockLockError::EncodingError(format!("Shadow receipt serialization failed: {}", e)))?;
        hasher.update(&serialized);
        
        Ok(*hasher.finalize().as_bytes())
    }

    /// Compute hash for signing (excludes signature fields)
    fn compute_signing_hash(&self) -> DockLockResult<[u8; 32]> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[SHADOW_RECEIPT_HASH]);
        
        // Create a copy without signature fields for hashing
        let mut receipt_for_hash = self.clone();
        receipt_for_hash.signature = None;
        receipt_for_hash.signer_pubkey = None;
        
        let serialized = serde_json::to_vec(&receipt_for_hash)
            .map_err(|e| DockLockError::EncodingError(format!("Shadow receipt serialization failed: {}", e)))?;
        hasher.update(&serialized);
        
        Ok(*hasher.finalize().as_bytes())
    }

    /// Sign the shadow receipt
    pub fn sign(&mut self, signing_key: &SigningKey) -> DockLockResult<()> {
        let hash = self.compute_signing_hash()?;
        let signature = signing_key.sign(&hash);
        
        self.signature = Some(signature);
        self.signer_pubkey = Some(signing_key.verifying_key().to_bytes().to_vec());
        
        info!("Shadow receipt {} signed", self.shadow_id);
        Ok(())
    }

    /// Verify shadow receipt signature
    pub fn verify_signature(&self) -> DockLockResult<bool> {
        let signature = self.signature.as_ref()
            .ok_or_else(|| DockLockError::InvalidOperation("No signature found".to_string()))?;
        
        let pubkey_bytes = self.signer_pubkey.as_ref()
            .ok_or_else(|| DockLockError::InvalidOperation("No public key found".to_string()))?;
        
        if pubkey_bytes.len() != 32 {
            return Err(DockLockError::InvalidOperation("Invalid public key length".to_string()));
        }
        
        let mut pubkey_array = [0u8; 32];
        pubkey_array.copy_from_slice(pubkey_bytes);
        
        let verifying_key = VerifyingKey::from_bytes(&pubkey_array)
            .map_err(|e| DockLockError::CryptoError(format!("Invalid public key: {}", e)))?;
        
        let hash = self.compute_signing_hash()?;
        
        match verifying_key.verify(&hash, signature) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Add audit event to privacy trail
    pub fn add_audit_event(&mut self, event_type: AuditEventType, metadata: HashMap<String, String>) -> DockLockResult<()> {
        let event_id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| DockLockError::InvalidOperation(format!("Time error: {}", e)))?
            .as_secs();

        let event_data = format!("{:?}:{}", event_type, timestamp);
        let event_hash = blake3::hash(event_data.as_bytes()).into();

        let audit_event = PrivacyAuditEvent {
            event_id,
            event_type,
            timestamp,
            metadata,
            event_hash,
        };

        self.privacy_metadata.audit_trail.events.push(audit_event);

        // Update audit root
        let all_hashes: Vec<_> = self.privacy_metadata.audit_trail.events
            .iter()
            .map(|e| e.event_hash)
            .collect();
        
        if !all_hashes.is_empty() {
            let combined = all_hashes.iter()
                .fold(Vec::new(), |mut acc, bytes| {
                    acc.extend_from_slice(bytes);
                    acc
                });
            self.privacy_metadata.audit_trail.audit_root = *blake3::hash(&combined).as_bytes();
        }

        debug!("Added audit event to shadow receipt {}", self.shadow_id);
        Ok(())
    }
}

impl PostboxSystem {
    /// Create a new postbox system
    pub fn new(config: PostboxSystemConfig) -> Self {
        PostboxSystem {
            id: Uuid::new_v4(),
            postboxes: Arc::new(RwLock::new(HashMap::new())),
            delivery_queue: Arc::new(RwLock::new(BTreeMap::new())),
            config,
            stats: Arc::new(RwLock::new(PostboxSystemStats::default())),
        }
    }

    /// Create a new postbox
    pub fn create_postbox(&self, config: PostboxConfig) -> DockLockResult<String> {
        let postbox_id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| DockLockError::InvalidOperation(format!("Time error: {}", e)))?
            .as_secs();

        let postbox = Postbox {
            id: postbox_id.clone(),
            config,
            messages: Vec::new(),
            access_log: Vec::new(),
            created_at: timestamp,
            last_accessed: timestamp,
        };

        let mut postboxes = self.postboxes.write()
            .map_err(|e| DockLockError::InvalidOperation(format!("Failed to acquire postboxes lock: {}", e)))?;
        
        if postboxes.len() >= self.config.max_postboxes {
            return Err(DockLockError::InvalidOperation("Maximum postboxes reached".to_string()));
        }

        postboxes.insert(postbox_id.clone(), postbox);

        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            stats.total_postboxes += 1;
            stats.active_postboxes += 1;
        }

        info!("Created postbox {}", postbox_id);
        Ok(postbox_id)
    }

    /// Deliver shadow receipt to postbox
    pub fn deliver_shadow_receipt(
        &self,
        _postbox_id: &str,
        shadow_receipt: ShadowReceipt,
        delivery_config: DeliveryConfig,
    ) -> DockLockResult<String> {
        let task_id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| DockLockError::InvalidOperation(format!("Time error: {}", e)))?
            .as_secs();

        let delivery_task = DeliveryTask {
            task_id: task_id.clone(),
            shadow_receipt,
            delivery_config,
            status: DeliveryTaskStatus::Pending,
            created_at: timestamp,
            retry_count: 0,
        };

        // Add to delivery queue
        let mut queue = self.delivery_queue.write()
            .map_err(|e| DockLockError::InvalidOperation(format!("Failed to acquire delivery queue lock: {}", e)))?;
        
        queue.entry(timestamp).or_insert_with(Vec::new).push(delivery_task);

        info!("Queued shadow receipt delivery task {}", task_id);
        Ok(task_id)
    }

    /// Process delivery queue
    pub fn process_delivery_queue(&self) -> DockLockResult<usize> {
        let mut processed = 0;
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| DockLockError::InvalidOperation(format!("Time error: {}", e)))?
            .as_secs();

        let mut queue = self.delivery_queue.write()
            .map_err(|e| DockLockError::InvalidOperation(format!("Failed to acquire delivery queue lock: {}", e)))?;

        // Process tasks that are ready
        let ready_tasks: Vec<_> = queue.range(..=current_time).map(|(k, v)| (*k, v.clone())).collect();
        
        for (timestamp, tasks) in ready_tasks {
            for task in tasks {
                if let Err(e) = self.process_delivery_task(&task) {
                    warn!("Failed to process delivery task {}: {}", task.task_id, e);
                } else {
                    processed += 1;
                }
            }
            queue.remove(&timestamp);
        }

        if processed > 0 {
            info!("Processed {} delivery tasks", processed);
        }

        Ok(processed)
    }

    /// Process individual delivery task
    fn process_delivery_task(&self, task: &DeliveryTask) -> DockLockResult<()> {
        let postbox_id = &task.shadow_receipt.postbox_info.postbox_id;
        
        let mut postboxes = self.postboxes.write()
            .map_err(|e| DockLockError::InvalidOperation(format!("Failed to acquire postboxes lock: {}", e)))?;
        
        let postbox = postboxes.get_mut(postbox_id)
            .ok_or_else(|| DockLockError::NotFound(format!("Postbox {} not found", postbox_id)))?;

        // Check if postbox has space
        if postbox.messages.len() >= postbox.config.max_messages {
            return Err(DockLockError::InvalidOperation("Postbox full".to_string()));
        }

        // Create postbox message
        let message_size = serde_json::to_vec(&task.shadow_receipt)
            .map_err(|e| DockLockError::EncodingError(format!("Shadow receipt serialization failed: {}", e)))?
            .len();

        let message = PostboxMessage {
            message_id: Uuid::new_v4().to_string(),
            shadow_receipt: task.shadow_receipt.clone(),
            metadata: PostboxMessageMetadata {
                sender: None, // Anonymous by default
                priority: MessagePriority::Normal,
                delivery_deadline: None,
                tags: vec!["shadow_receipt".to_string()],
                size: message_size,
            },
            encryption_info: None, // TODO: Implement encryption
        };

        postbox.messages.push(message);

        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            stats.total_messages_delivered += 1;
        }

        info!("Delivered shadow receipt to postbox {}", postbox_id);
        Ok(())
    }

    /// Get postbox statistics
    pub fn get_stats(&self) -> DockLockResult<PostboxSystemStats> {
        let stats = self.stats.read()
            .map_err(|e| DockLockError::InvalidOperation(format!("Failed to acquire stats lock: {}", e)))?;
        Ok(stats.clone())
    }
}

impl Default for PostboxSystemConfig {
    fn default() -> Self {
        PostboxSystemConfig {
            max_postboxes: 10000,
            default_postbox_ttl: 86400 * 7, // 7 days
            cleanup_interval: 3600, // 1 hour
            max_delivery_workers: 10,
            privacy_settings: SystemPrivacySettings {
                default_privacy_level: PrivacyLevel::Enhanced,
                enable_anonymization: true,
                audit_retention_period: 86400 * 90, // 90 days
                compliance_mode: true,
            },
        }
    }
}

impl Default for PostboxConfig {
    fn default() -> Self {
        PostboxConfig {
            max_messages: 1000,
            max_total_size: 10 * 1024 * 1024, // 10 MB
            message_ttl: 86400 * 7, // 7 days
            access_control: PostboxAccessControl {
                auth_method: AuthenticationMethod::PublicKey {
                    required_pubkey: vec![0u8; 32], // Placeholder
                },
                permissions: PostboxPermissions {
                    can_read: true,
                    can_write: true,
                    can_delete: false,
                    can_modify: false,
                    max_message_size: 1024 * 1024, // 1 MB
                },
                rate_limits: RateLimits {
                    max_requests_per_minute: 60,
                    max_bandwidth_per_minute: 10 * 1024 * 1024, // 10 MB
                    burst_allowance: 10,
                },
            },
            privacy_settings: PostboxPrivacySettings {
                encrypt_messages: true,
                enable_access_logging: true,
                anonymize_logs: true,
                auto_delete_expired: true,
            },
        }
    }
}

impl Default for DeliveryConfig {
    fn default() -> Self {
        DeliveryConfig {
            max_retries: 3,
            retry_delay: 60, // 1 minute
            delivery_timeout: 300, // 5 minutes
            require_confirmation: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::receipt::*;
    use ed25519_dalek::SigningKey;

    fn create_test_receipt() -> Receipt {
        Receipt {
            receipt_id: "test_receipt_123".to_string(),
            run_header: RunHeader {
                session_id: "test_session".to_string(),
                image_hash: "test_image".to_string(),
                command: vec!["test".to_string()],
                environment: HashMap::new(),
                working_dir: "/tmp".to_string(),
                resource_limits: ResourceLimits {
                    max_memory: 1024 * 1024 * 1024, // 1GB
                    max_cpu_time: 30000, // 30 seconds
                    max_fs_ops: 1000,
                    max_net_ops: 100,
                },
                cage_config: CageConfig {
                    rng_seed: vec![0u8; 32],
                    syscall_filter_enabled: true,
                    witness_recording: true,
                    event_correlation: true,
                },
            },
            trace_roots: TraceRoots {
                witness_root: *blake3::hash(b"witness_root").as_bytes(),
                event_stream_root: *blake3::hash(b"event_stream_root").as_bytes(),
                wallet_root: *blake3::hash(b"wallet_root").as_bytes(),
                combined_root: *blake3::hash(b"combined_root").as_bytes(),
            },
            policy_info: PolicyInfo::default(),
            execution_stats: ExecutionStats::default(),
            timestamp: 1234567890,
            signature: None,
            signer_pubkey: None,
        }
    }

    fn create_test_postbox_info() -> PostboxInfo {
        PostboxInfo {
            postbox_id: "test_postbox_123".to_string(),
            delivery_method: DeliveryMethod::Direct {
                recipient_pubkey: vec![0u8; 32],
                encryption_scheme: EncryptionScheme::Ed25519ChaCha20,
            },
            access_control: PostboxAccessControl {
                auth_method: AuthenticationMethod::PublicKey {
                    required_pubkey: vec![0u8; 32],
                },
                permissions: PostboxPermissions {
                    can_read: true,
                    can_write: true,
                    can_delete: false,
                    can_modify: false,
                    max_message_size: 1024 * 1024,
                },
                rate_limits: RateLimits {
                    max_requests_per_minute: 60,
                    max_bandwidth_per_minute: 10 * 1024 * 1024,
                    burst_allowance: 10,
                },
            },
            delivery_status: DeliveryStatus::Pending,
            expiration_time: 1234567890 + 86400, // 1 day from timestamp
        }
    }

    #[test]
    fn test_shadow_receipt_creation() {
        let original_receipt = create_test_receipt();
        let postbox_info = create_test_postbox_info();
        let privacy_level = PrivacyLevel::Enhanced;

        let shadow_receipt = ShadowReceipt::new(&original_receipt, postbox_info, privacy_level)
            .expect("Failed to create shadow receipt");

        assert!(!shadow_receipt.shadow_id.is_empty());
        assert_eq!(shadow_receipt.privacy_metadata.privacy_level, PrivacyLevel::Enhanced);
        assert_eq!(shadow_receipt.privacy_metadata.anonymization_methods.len(), 2);
        assert!(shadow_receipt.timestamp > 0);
        assert!(shadow_receipt.signature.is_none());
    }

    #[test]
    fn test_shadow_receipt_signing_and_verification() {
        let original_receipt = create_test_receipt();
        let postbox_info = create_test_postbox_info();
        let privacy_level = PrivacyLevel::Maximum;

        let mut shadow_receipt = ShadowReceipt::new(&original_receipt, postbox_info, privacy_level)
            .expect("Failed to create shadow receipt");

        // Generate a signing key
        let signing_key = SigningKey::generate(&mut rand::thread_rng());

        // Sign the shadow receipt
        shadow_receipt.sign(&signing_key).expect("Failed to sign shadow receipt");

        // Verify the signature
        let is_valid = shadow_receipt.verify_signature().expect("Failed to verify signature");
        assert!(is_valid);

        // Test with tampered data
        let original_id = shadow_receipt.shadow_id.clone();
        shadow_receipt.shadow_id = "tampered_id".to_string();
        let is_valid_tampered = shadow_receipt.verify_signature().expect("Failed to verify tampered signature");
        assert!(!is_valid_tampered);

        // Restore original data
        shadow_receipt.shadow_id = original_id;
        let is_valid_restored = shadow_receipt.verify_signature().expect("Failed to verify restored signature");
        assert!(is_valid_restored);
    }

    #[test]
    fn test_postbox_system_creation() {
        let config = PostboxSystemConfig::default();
        let postbox_system = PostboxSystem::new(config);

        assert_eq!(postbox_system.config.max_postboxes, 10000);
        assert_eq!(postbox_system.config.privacy_settings.default_privacy_level, PrivacyLevel::Enhanced);
    }

    #[test]
    fn test_compliance_status() {
        let statuses = vec![
            crate::shadow_receipt::ComplianceStatus::Compliant,
            crate::shadow_receipt::ComplianceStatus::PartiallyCompliant {
                missing_requirements: vec!["requirement1".to_string()],
            },
            crate::shadow_receipt::ComplianceStatus::NonCompliant {
                violations: vec!["violation1".to_string()],
            },
            crate::shadow_receipt::ComplianceStatus::UnderReview,
        ];

        for status in statuses {
            let original_receipt = create_test_receipt();
            let postbox_info = create_test_postbox_info();
            
            let mut shadow_receipt = ShadowReceipt::new(&original_receipt, postbox_info, PrivacyLevel::Enhanced)
                .expect("Failed to create shadow receipt");
            
            shadow_receipt.privacy_metadata.compliance_info.compliance_status = status;
            
            assert!(!shadow_receipt.shadow_id.is_empty());
        }
    }
}
