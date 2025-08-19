use std::collections::{HashMap, BTreeMap, VecDeque};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use uuid::Uuid;
use ed25519_dalek::{SigningKey, Signer};

use bpi_enc::{CanonicalCbor, domain_hash};
use crate::error::{DockLockError, DockLockResult};
use crate::bpi_wallet_registry::{BpiWalletRegistry, RegisteredWallet};
use crate::receipt_registry::ReceiptRegistry;
use crate::wallet::{WalletAddress, ServiceId};

/// Domain separation constants for enhanced storage
const ENHANCED_STORAGE_HASH: &str = "ENHANCED_STORAGE";
const STORAGE_TRANSACTION_HASH: &str = "STORAGE_TRANSACTION";

/// Enhanced Storage Database - Enterprise-grade military storage with wallet integration
#[derive(Debug)]
pub struct EnhancedStorageDb {
    pub id: Uuid,
    pub name: String,
    wallet_registry: Arc<BpiWalletRegistry>,
    receipt_registry: Arc<ReceiptRegistry>,
    storage_engines: Arc<RwLock<HashMap<StorageType, StorageEngine>>>,
    transaction_log: Arc<RwLock<VecDeque<StorageTransaction>>>,
    storage_indexes: Arc<RwLock<StorageIndexes>>,
    config: EnhancedStorageConfig,
    stats: Arc<RwLock<StorageStats>>,
    signing_key: SigningKey,
}

/// Storage engine for specific data types
#[derive(Debug)]
pub struct StorageEngine {
    pub id: Uuid,
    pub engine_type: StorageType,
    data: Arc<RwLock<HashMap<String, StorageRecord>>>,
    indexes: Arc<RwLock<HashMap<String, BTreeMap<String, Vec<String>>>>>,
    config: StorageEngineConfig,
    stats: Arc<RwLock<EngineStats>>,
}

/// Storage record with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageRecord {
    pub id: String,
    pub data: Vec<u8>,
    pub metadata: StorageMetadata,
    pub owner_wallet_id: Option<Uuid>,
    pub acl: AccessControlList,
    pub version: u64,
    pub created_at: u64,
    pub modified_at: u64,
    pub status: RecordStatus,
    pub signature: String,
}

/// Storage metadata for records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetadata {
    pub content_type: String,
    pub encoding: Option<String>,
    pub size: usize,
    pub content_hash: String,
    pub encryption_scheme: Option<String>,
    pub compression_scheme: Option<String>,
    pub tags: HashMap<String, String>,
    pub classification: DataClassification,
    pub retention_policy: RetentionPolicy,
}

/// Access control list for storage records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlList {
    pub owner_permissions: Permissions,
    pub wallet_permissions: HashMap<Uuid, Permissions>,
    pub service_permissions: HashMap<ServiceId, Permissions>,
    pub public_permissions: Option<Permissions>,
    pub expires_at: Option<u64>,
}

/// Permission levels for storage access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permissions {
    pub read: bool,
    pub write: bool,
    pub delete: bool,
    pub share: bool,
    pub admin: bool,
}

/// Storage types supported
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StorageType {
    WalletData,
    BpciMessages,
    BciTransactions,
    Receipts,
    Policies,
    EventStreams,
    BlobStorage,
    Documents,
    Configurations,
    Logs,
}

/// Record status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecordStatus {
    Active,
    Archived,
    Deleted,
    Corrupted,
    Migrating,
}

/// Data classification levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
    TopSecret,
}

/// Retention policy for data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub retention_seconds: Option<u64>,
    pub archive_after_seconds: Option<u64>,
    pub delete_after_seconds: Option<u64>,
    pub compliance_requirements: Vec<String>,
}

/// Storage transaction for audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageTransaction {
    pub id: Uuid,
    pub transaction_type: TransactionType,
    pub storage_type: StorageType,
    pub record_id: String,
    pub wallet_id: Option<Uuid>,
    pub service_id: Option<ServiceId>,
    pub data: Vec<u8>,
    pub metadata: HashMap<String, String>,
    pub timestamp: u64,
    pub status: TransactionStatus,
    pub signature: String,
}

/// Storage transaction types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionType {
    Create,
    Read,
    Update,
    Delete,
    Archive,
    Restore,
    Replicate,
    Migrate,
    Backup,
    Verify,
}

/// Transaction status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionStatus {
    Pending,
    Committed,
    Aborted,
    Failed,
}

/// Storage indexes for fast lookups
#[derive(Debug, Default)]
pub struct StorageIndexes {
    by_owner_wallet: HashMap<Uuid, Vec<String>>,
    by_classification: HashMap<DataClassification, Vec<String>>,
    by_content_type: HashMap<String, Vec<String>>,
    by_tags: HashMap<String, HashMap<String, Vec<String>>>,
    by_creation_date: BTreeMap<u64, Vec<String>>,
    by_modification_date: BTreeMap<u64, Vec<String>>,
    by_size: BTreeMap<usize, Vec<String>>,
}

/// Storage engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEngineConfig {
    pub max_records: usize,
    pub max_storage_bytes: u64,
    pub enable_compression: bool,
    pub compression_algorithm: CompressionAlgorithm,
    pub default_encryption: bool,
    pub index_config: IndexConfig,
}

/// Compression algorithms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CompressionAlgorithm {
    None,
    Gzip,
    Lz4,
    Zstd,
}

/// Index configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexConfig {
    pub auto_index: bool,
    pub update_interval_seconds: u64,
    pub max_index_size: usize,
    pub compress_indexes: bool,
}

/// Engine statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EngineStats {
    pub total_records: u64,
    pub active_records: u64,
    pub archived_records: u64,
    pub deleted_records: u64,
    pub total_storage_bytes: u64,
    pub compressed_storage_bytes: u64,
    pub total_reads: u64,
    pub total_writes: u64,
    pub avg_read_time_ms: f64,
    pub avg_write_time_ms: f64,
}

/// Enhanced storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedStorageConfig {
    pub max_storage_engines: usize,
    pub default_storage_type: StorageType,
    pub transaction_log_retention_seconds: u64,
    pub enable_auto_cleanup: bool,
    pub cleanup_interval_seconds: u64,
    pub backup_config: BackupConfig,
}

/// Backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    pub enable_auto_backup: bool,
    pub backup_interval_seconds: u64,
    pub backup_retention_seconds: u64,
    pub backup_compression: CompressionAlgorithm,
    pub backup_encryption: bool,
    pub backup_location: String,
}

/// Storage statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageStats {
    pub total_engines: u64,
    pub total_records: u64,
    pub total_storage_bytes: u64,
    pub total_transactions: u64,
    pub encryption_coverage_percent: f64,
    pub compression_ratio: f64,
    pub uptime_seconds: u64,
    pub last_backup_at: Option<u64>,
}

impl Default for EnhancedStorageConfig {
    fn default() -> Self {
        Self {
            max_storage_engines: 50,
            default_storage_type: StorageType::Documents,
            transaction_log_retention_seconds: 90 * 24 * 60 * 60, // 90 days
            enable_auto_cleanup: true,
            cleanup_interval_seconds: 24 * 60 * 60, // Daily
            backup_config: BackupConfig {
                enable_auto_backup: true,
                backup_interval_seconds: 6 * 60 * 60, // 6 hours
                backup_retention_seconds: 30 * 24 * 60 * 60, // 30 days
                backup_compression: CompressionAlgorithm::Zstd,
                backup_encryption: true,
                backup_location: "/var/lib/metanode/backups".to_string(),
            },
        }
    }
}

impl Default for StorageEngineConfig {
    fn default() -> Self {
        Self {
            max_records: 1_000_000,
            max_storage_bytes: 10 * 1024 * 1024 * 1024, // 10GB
            enable_compression: true,
            compression_algorithm: CompressionAlgorithm::Lz4,
            default_encryption: true,
            index_config: IndexConfig {
                auto_index: true,
                update_interval_seconds: 300, // 5 minutes
                max_index_size: 100 * 1024 * 1024, // 100MB
                compress_indexes: true,
            },
        }
    }
}

impl Default for Permissions {
    fn default() -> Self {
        Self {
            read: true,
            write: false,
            delete: false,
            share: false,
            admin: false,
        }
    }
}

impl EnhancedStorageDb {
    /// Create a new enhanced storage database
    pub fn new(name: String, wallet_registry: Arc<BpiWalletRegistry>, 
               receipt_registry: Arc<ReceiptRegistry>, config: EnhancedStorageConfig) -> Self {
        let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
        let id = Uuid::new_v4();
        
        info!("Creating enhanced storage database: {} ({})", name, id);
        
        Self {
            id,
            name,
            wallet_registry,
            receipt_registry,
            storage_engines: Arc::new(RwLock::new(HashMap::new())),
            transaction_log: Arc::new(RwLock::new(VecDeque::new())),
            storage_indexes: Arc::new(RwLock::new(StorageIndexes::default())),
            config,
            stats: Arc::new(RwLock::new(StorageStats::default())),
            signing_key,
        }
    }

    /// Create a new storage engine for a specific type
    pub async fn create_storage_engine(&self, storage_type: StorageType, 
                                       config: StorageEngineConfig) -> DockLockResult<Uuid> {
        let mut engines = self.storage_engines.write().await;
        let mut stats = self.stats.write().await;

        if engines.len() >= self.config.max_storage_engines {
            return Err(DockLockError::CapacityExceeded("Maximum storage engines reached".to_string()));
        }

        let engine = StorageEngine {
            id: Uuid::new_v4(),
            engine_type: storage_type.clone(),
            data: Arc::new(RwLock::new(HashMap::new())),
            indexes: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(RwLock::new(EngineStats::default())),
        };

        let engine_id = engine.id;
        engines.insert(storage_type.clone(), engine);
        stats.total_engines += 1;

        info!("Created storage engine {} for type {:?}", engine_id, storage_type);
        Ok(engine_id)
    }

    /// Store a record in the database
    pub async fn store_record(&self, storage_type: StorageType, record_id: String, 
                              data: Vec<u8>, metadata: StorageMetadata, 
                              owner_wallet_id: Option<Uuid>, acl: AccessControlList) -> DockLockResult<()> {
        let engines = self.storage_engines.read().await;
        let engine = engines.get(&storage_type)
            .ok_or_else(|| DockLockError::NotFound(format!("Storage engine for {:?} not found", storage_type)))?;

        let mut engine_data = engine.data.write().await;
        let mut engine_stats = engine.stats.write().await;
        let mut transaction_log = self.transaction_log.write().await;

        if engine_data.contains_key(&record_id) {
            return Err(DockLockError::AlreadyExists(format!("Record {} already exists", record_id)));
        }

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let record = StorageRecord {
            id: record_id.clone(),
            data: data.clone(),
            metadata,
            owner_wallet_id,
            acl,
            version: 1,
            created_at: now,
            modified_at: now,
            status: RecordStatus::Active,
            signature: self.sign_record_data(&record_id, &data)?,
        };

        engine_data.insert(record_id.clone(), record.clone());
        engine_stats.total_records += 1;
        engine_stats.active_records += 1;
        engine_stats.total_storage_bytes += record.data.len() as u64;
        engine_stats.total_writes += 1;

        let transaction = StorageTransaction {
            id: Uuid::new_v4(),
            transaction_type: TransactionType::Create,
            storage_type: storage_type.clone(),
            record_id: record_id.clone(),
            wallet_id: owner_wallet_id,
            service_id: None,
            data: vec![],
            metadata: HashMap::new(),
            timestamp: now,
            status: TransactionStatus::Committed,
            signature: self.sign_transaction_data(&record_id, &TransactionType::Create)?,
        };

        transaction_log.push_back(transaction);

        // Update database-level statistics
        let mut db_stats = self.stats.write().await;
        db_stats.total_records += 1;
        db_stats.total_storage_bytes += record.data.len() as u64;
        db_stats.total_transactions += 1;

        info!("Stored record {} in {:?} storage engine", record_id, storage_type);
        Ok(())
    }

    /// Retrieve a record from the database
    pub async fn get_record(&self, storage_type: StorageType, record_id: &str, 
                            requesting_wallet_id: Option<Uuid>) -> DockLockResult<StorageRecord> {
        let engines = self.storage_engines.read().await;
        let engine = engines.get(&storage_type)
            .ok_or_else(|| DockLockError::NotFound(format!("Storage engine for {:?} not found", storage_type)))?;

        let engine_data = engine.data.read().await;
        let mut engine_stats = engine.stats.write().await;

        let record = engine_data.get(record_id)
            .ok_or_else(|| DockLockError::NotFound(format!("Record {} not found", record_id)))?;

        if let Some(wallet_id) = requesting_wallet_id {
            if !self.check_read_permission(record, wallet_id).await? {
                return Err(DockLockError::AccessDenied("Insufficient permissions to read record".to_string()));
            }
        }

        engine_stats.total_reads += 1;
        Ok(record.clone())
    }

    /// Get database statistics
    pub async fn get_stats(&self) -> StorageStats {
        self.stats.read().await.clone()
    }

    /// Check read permission for a record
    async fn check_read_permission(&self, record: &StorageRecord, wallet_id: Uuid) -> DockLockResult<bool> {
        if record.owner_wallet_id == Some(wallet_id) {
            return Ok(true);
        }

        if let Some(permissions) = record.acl.wallet_permissions.get(&wallet_id) {
            return Ok(permissions.read);
        }

        if let Some(public_permissions) = &record.acl.public_permissions {
            return Ok(public_permissions.read);
        }

        Ok(false)
    }

    /// Sign record data for integrity
    fn sign_record_data(&self, record_id: &str, data: &[u8]) -> DockLockResult<String> {
        let hash_input = format!("{}{}", record_id, hex::encode(blake3::hash(data).as_bytes()));
        let hash = domain_hash(ENHANCED_STORAGE_HASH, hash_input.as_bytes());
        let signature = self.signing_key.sign(&hash);
        Ok(hex::encode(signature.to_bytes()))
    }

    /// Sign transaction data for integrity
    fn sign_transaction_data(&self, record_id: &str, transaction_type: &TransactionType) -> DockLockResult<String> {
        let data = format!("{}{:?}{}", record_id, transaction_type, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos());
        let hash = domain_hash(STORAGE_TRANSACTION_HASH, data.as_bytes());
        let signature = self.signing_key.sign(&hash);
        Ok(hex::encode(signature.to_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bpi_wallet_registry::{BpiWalletRegistryConfig, WalletCapabilities};

    #[tokio::test]
    async fn test_enhanced_storage_creation() {
        let wallet_config = BpiWalletRegistryConfig::default();
        let wallet_registry = Arc::new(BpiWalletRegistry::new("test_wallet_registry".to_string(), wallet_config));
        let receipt_registry = Arc::new(ReceiptRegistry::new("test_receipt_registry".to_string(), Default::default()));
        let storage_config = EnhancedStorageConfig::default();
        
        let storage_db = EnhancedStorageDb::new("test_storage".to_string(), wallet_registry, receipt_registry, storage_config);
        
        assert_eq!(storage_db.name, "test_storage");
        let stats = storage_db.get_stats().await;
        assert_eq!(stats.total_engines, 0);
    }

    #[tokio::test]
    async fn test_storage_engine_creation() {
        let wallet_config = BpiWalletRegistryConfig::default();
        let wallet_registry = Arc::new(BpiWalletRegistry::new("test_wallet_registry".to_string(), wallet_config));
        let receipt_registry = Arc::new(ReceiptRegistry::new("test_receipt_registry".to_string(), Default::default()));
        let storage_config = EnhancedStorageConfig::default();
        
        let storage_db = EnhancedStorageDb::new("test_storage".to_string(), wallet_registry, receipt_registry, storage_config);
        
        let engine_config = StorageEngineConfig::default();
        let engine_id = storage_db.create_storage_engine(StorageType::Documents, engine_config).await.unwrap();
        
        assert!(engine_id != Uuid::nil());
        let stats = storage_db.get_stats().await;
        assert_eq!(stats.total_engines, 1);
    }

    #[tokio::test]
    async fn test_record_storage_and_retrieval() {
        let wallet_config = BpiWalletRegistryConfig::default();
        let wallet_registry = Arc::new(BpiWalletRegistry::new("test_wallet_registry".to_string(), wallet_config));
        let receipt_registry = Arc::new(ReceiptRegistry::new("test_receipt_registry".to_string(), Default::default()));
        let storage_config = EnhancedStorageConfig::default();
        
        let storage_db = EnhancedStorageDb::new("test_storage".to_string(), wallet_registry, receipt_registry, storage_config);
        
        let engine_config = StorageEngineConfig::default();
        storage_db.create_storage_engine(StorageType::Documents, engine_config).await.unwrap();
        
        let metadata = StorageMetadata {
            content_type: "text/plain".to_string(),
            encoding: None,
            size: 13,
            content_hash: "test_hash".to_string(),
            encryption_scheme: None,
            compression_scheme: None,
            tags: HashMap::new(),
            classification: DataClassification::Internal,
            retention_policy: RetentionPolicy {
                retention_seconds: Some(86400),
                archive_after_seconds: None,
                delete_after_seconds: None,
                compliance_requirements: vec![],
            },
        };
        
        let acl = AccessControlList {
            owner_permissions: Permissions::default(),
            wallet_permissions: HashMap::new(),
            service_permissions: HashMap::new(),
            public_permissions: Some(Permissions::default()),
            expires_at: None,
        };
        
        storage_db.store_record(
            StorageType::Documents,
            "test_record".to_string(),
            b"Hello, world!".to_vec(),
            metadata,
            None,
            acl
        ).await.unwrap();
        
        let retrieved_record = storage_db.get_record(StorageType::Documents, "test_record", None).await.unwrap();
        assert_eq!(retrieved_record.id, "test_record");
        assert_eq!(retrieved_record.data, b"Hello, world!");
    }
}
