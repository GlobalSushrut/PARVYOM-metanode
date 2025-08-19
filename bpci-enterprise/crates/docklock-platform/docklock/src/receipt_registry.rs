use crate::error::{DockLockError, DockLockResult};
use crate::receipt::Receipt;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, RwLock};
use tracing::info;
use uuid::Uuid;

/// Domain separation constant for registry hashing
const REGISTRY_HASH: u8 = 0x08;

/// Receipt hash wrapper for indexing
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReceiptHash(pub [u8; 32]);

/// Receipt Registry - main service for receipt storage and querying
#[derive(Debug)]
pub struct ReceiptRegistry {
    /// Registry identifier
    pub id: Uuid,
    /// Registry name
    pub name: String,
    /// Primary receipt storage indexed by receipt ID
    receipts: Arc<RwLock<HashMap<Uuid, Receipt>>>,
    /// Receipt indexing system
    index: Arc<RwLock<ReceiptIndex>>,
    /// Authentication service
    auth_service: Arc<RwLock<AuthService>>,
    /// Registry configuration
    config: RegistryConfig,
    /// Registry statistics
    stats: Arc<RwLock<RegistryStats>>,
}

/// Receipt indexing system for fast queries
#[derive(Debug, Default)]
pub struct ReceiptIndex {
    /// Index by block height
    by_block_height: BTreeMap<u64, Vec<Uuid>>,
    /// Index by timestamp
    by_timestamp: BTreeMap<u64, Vec<Uuid>>,
    /// Index by execution context ID
    by_execution_id: HashMap<Uuid, Vec<Uuid>>,
    /// Index by policy compliance status
    by_compliance_status: HashMap<String, Vec<Uuid>>,
    /// Index by receipt hash
    by_receipt_hash: HashMap<ReceiptHash, Uuid>,
}

/// Authentication service for registry access
#[derive(Debug, Default)]
pub struct AuthService {
    /// API keys and their associated roles
    api_keys: HashMap<String, UserRole>,
    /// User sessions
    sessions: HashMap<String, UserSession>,
    /// Access audit log
    audit_log: Vec<AccessLogEntry>,
}

/// User roles for access control
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    /// Administrator with full access
    Admin,
    /// Validator with read/write access to own receipts
    Validator,
    /// Public user with read-only access to public receipts
    Public,
}

/// User session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    /// Session ID
    pub session_id: String,
    /// User role
    pub role: UserRole,
    /// Session creation timestamp
    pub created_at: u64,
    /// Session expiration timestamp
    pub expires_at: u64,
    /// Associated API key
    pub api_key: String,
}

/// Access log entry for audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessLogEntry {
    /// Log entry ID
    pub id: Uuid,
    /// Timestamp of access
    pub timestamp: u64,
    /// API key used
    pub api_key: String,
    /// User role
    pub role: UserRole,
    /// Operation performed
    pub operation: String,
    /// Receipt ID accessed (if applicable)
    pub receipt_id: Option<Uuid>,
    /// Success status
    pub success: bool,
    /// Error message (if any)
    pub error: Option<String>,
}

/// Registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    /// Maximum number of receipts to store
    pub max_receipts: usize,
    /// Default page size for queries
    pub default_page_size: usize,
    /// Maximum page size allowed
    pub max_page_size: usize,
    /// Session timeout in seconds
    pub session_timeout: u64,
    /// Whether to enable public access
    pub enable_public_access: bool,
    /// Maximum audit log entries
    pub max_audit_entries: usize,
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            max_receipts: 1_000_000,
            default_page_size: 50,
            max_page_size: 1000,
            session_timeout: 3600, // 1 hour
            enable_public_access: true,
            max_audit_entries: 10_000,
        }
    }
}

/// Registry statistics
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegistryStats {
    /// Total number of receipts stored
    pub total_receipts: u64,
    /// Total number of queries processed
    pub total_queries: u64,
    /// Total number of successful authentications
    pub successful_auths: u64,
    /// Total number of failed authentications
    pub failed_auths: u64,
    /// Average query response time in milliseconds
    pub avg_query_time_ms: f64,
    /// Index size statistics
    pub index_stats: IndexStats,
}

/// Index statistics
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    /// Number of entries in block height index
    pub block_height_entries: usize,
    /// Number of entries in timestamp index
    pub timestamp_entries: usize,
    /// Number of entries in execution ID index
    pub execution_id_entries: usize,
    /// Number of entries in compliance status index
    pub compliance_status_entries: usize,
}

/// Query parameters for receipt searches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParams {
    /// Receipt ID to search for
    pub receipt_id: Option<Uuid>,
    /// Block height range
    pub block_height_range: Option<(u64, u64)>,
    /// Timestamp range
    pub timestamp_range: Option<(u64, u64)>,
    /// Execution context ID
    pub execution_id: Option<Uuid>,
    /// Policy compliance status filter
    pub compliance_status: Option<String>,
    /// Pagination parameters
    pub pagination: PaginationParams,
}

/// Pagination parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    /// Page size (number of results per page)
    pub page_size: usize,
    /// Cursor for pagination (receipt ID to start from)
    pub cursor: Option<Uuid>,
    /// Offset for pagination (alternative to cursor)
    pub offset: Option<usize>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page_size: 50,
            cursor: None,
            offset: None,
        }
    }
}

/// Query result with pagination metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    /// Retrieved receipts
    pub receipts: Vec<Receipt>,
    /// Pagination metadata
    pub pagination: PaginationMetadata,
    /// Query execution time in milliseconds
    pub query_time_ms: u64,
}

/// Pagination metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMetadata {
    /// Current page size
    pub page_size: usize,
    /// Total number of results (estimate)
    pub total_results: Option<usize>,
    /// Next cursor for pagination
    pub next_cursor: Option<Uuid>,
    /// Whether there are more results
    pub has_more: bool,
}

impl ReceiptRegistry {
    /// Create a new receipt registry
    pub fn new(name: String, config: RegistryConfig) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            receipts: Arc::new(RwLock::new(HashMap::new())),
            index: Arc::new(RwLock::new(ReceiptIndex::default())),
            auth_service: Arc::new(RwLock::new(AuthService::default())),
            config,
            stats: Arc::new(RwLock::new(RegistryStats::default())),
        }
    }

    /// Store a receipt in the registry
    pub fn store_receipt(&self, receipt: Receipt) -> DockLockResult<()> {
        let receipt_id = Uuid::parse_str(&receipt.receipt_id).map_err(|e| 
            DockLockError::RegistryError(format!("Invalid receipt ID: {}", e)))?;
        
        // Store receipt
        {
            let mut receipts = self.receipts.write().unwrap();
            if receipts.len() >= self.config.max_receipts {
                return Err(DockLockError::RegistryError(
                    "Registry is at maximum capacity".to_string()
                ));
            }
            receipts.insert(receipt_id, receipt.clone());
        }

        // Update indexes
        {
            let mut index = self.index.write().unwrap();
            
            // Index by block height (extract from environment or use default)
            if let Some(block_height_str) = receipt.run_header.environment.get("BLOCK_HEIGHT") {
                if let Ok(block_height) = block_height_str.parse::<u64>() {
                    index.by_block_height.entry(block_height).or_default().push(receipt_id);
                }
            }

            // Index by timestamp
            index.by_timestamp.entry(receipt.timestamp).or_default().push(receipt_id);

            // Index by session ID (closest equivalent to execution ID)
            if let Ok(session_uuid) = Uuid::parse_str(&receipt.run_header.session_id) {
                index.by_execution_id.entry(session_uuid).or_default().push(receipt_id);
            }

            // Index by compliance status
            let status = format!("{:?}", receipt.policy_info.compliance_status);
            index.by_compliance_status.entry(status).or_default().push(receipt_id);

            // Index by receipt hash (compute hash from receipt ID for now)
            let receipt_hash = ReceiptHash(blake3::hash(receipt.receipt_id.as_bytes()).into());
            index.by_receipt_hash.insert(receipt_hash, receipt_id);
        }

        // Update stats
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_receipts += 1;
        }

        info!("Stored receipt {} in registry {}", receipt_id, self.name);
        Ok(())
    }

    /// Query receipts with authentication
    pub fn query_receipts(&self, params: QueryParams, api_key: &str) -> DockLockResult<QueryResult> {
        let start_time = std::time::Instant::now();

        // Authenticate request
        let user_role = self.authenticate(api_key)?;
        
        // Log access attempt
        self.log_access(api_key, &user_role, "query_receipts", None, true, None);

        // Execute query
        let result = self.execute_query(params, &user_role)?;

        // Update stats
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_queries += 1;
            let query_time = start_time.elapsed().as_millis() as f64;
            stats.avg_query_time_ms = (stats.avg_query_time_ms * (stats.total_queries - 1) as f64 + query_time) / stats.total_queries as f64;
        }

        Ok(result)
    }

    /// Get a specific receipt by ID
    pub fn get_receipt(&self, receipt_id: Uuid, api_key: &str) -> DockLockResult<Option<Receipt>> {
        // Authenticate request
        let user_role = self.authenticate(api_key)?;
        
        // Check access permissions
        let receipt = {
            let receipts = self.receipts.read().unwrap();
            receipts.get(&receipt_id).cloned()
        };

        if let Some(ref receipt) = receipt {
            if !self.can_access_receipt(&user_role, receipt) {
                self.log_access(api_key, &user_role, "get_receipt", Some(receipt_id), false, 
                    Some("Access denied".to_string()));
                return Err(DockLockError::AuthenticationError("Access denied".to_string()));
            }
        }

        // Log successful access
        self.log_access(api_key, &user_role, "get_receipt", Some(receipt_id), true, None);

        Ok(receipt)
    }

    /// Execute query based on parameters
    fn execute_query(&self, params: QueryParams, user_role: &UserRole) -> DockLockResult<QueryResult> {
        let start_time = std::time::Instant::now();
        let mut candidate_ids = Vec::new();

        // Get candidate receipt IDs based on query parameters
        {
            let index = self.index.read().unwrap();
            
            if let Some(receipt_id) = params.receipt_id {
                candidate_ids.push(receipt_id);
            } else {
                // Start with all receipt IDs and filter
                let receipts = self.receipts.read().unwrap();
                candidate_ids = receipts.keys().cloned().collect();

                // Filter by block height range
                if let Some((min_height, max_height)) = params.block_height_range {
                    let mut filtered_ids = Vec::new();
                    for height in min_height..=max_height {
                        if let Some(ids) = index.by_block_height.get(&height) {
                            filtered_ids.extend(ids.iter().cloned());
                        }
                    }
                    candidate_ids.retain(|id| filtered_ids.contains(id));
                }

                // Filter by timestamp range
                if let Some((min_time, max_time)) = params.timestamp_range {
                    let mut filtered_ids = Vec::new();
                    for (_timestamp, ids) in index.by_timestamp.range(min_time..=max_time) {
                        filtered_ids.extend(ids.iter().cloned());
                    }
                    candidate_ids.retain(|id| filtered_ids.contains(id));
                }

                // Filter by execution ID
                if let Some(execution_id) = params.execution_id {
                    if let Some(ids) = index.by_execution_id.get(&execution_id) {
                        candidate_ids.retain(|id| ids.contains(id));
                    } else {
                        candidate_ids.clear();
                    }
                }

                // Filter by compliance status
                if let Some(ref status) = params.compliance_status {
                    if let Some(ids) = index.by_compliance_status.get(status) {
                        candidate_ids.retain(|id| ids.contains(id));
                    } else {
                        candidate_ids.clear();
                    }
                }
            }
        }

        // Sort candidate IDs for consistent pagination
        candidate_ids.sort();

        // Apply pagination
        let page_size = std::cmp::min(params.pagination.page_size, self.config.max_page_size);
        let start_index = if let Some(cursor) = params.pagination.cursor {
            candidate_ids.iter().position(|&id| id == cursor).unwrap_or(0)
        } else {
            params.pagination.offset.unwrap_or(0)
        };

        let end_index = std::cmp::min(start_index + page_size, candidate_ids.len());
        let page_ids = &candidate_ids[start_index..end_index];

        // Retrieve receipts and filter by access permissions
        let mut receipts = Vec::new();
        {
            let receipt_storage = self.receipts.read().unwrap();
            for &receipt_id in page_ids {
                if let Some(receipt) = receipt_storage.get(&receipt_id) {
                    if self.can_access_receipt(user_role, receipt) {
                        receipts.push(receipt.clone());
                    }
                }
            }
        }

        // Create pagination metadata
        let next_cursor = if end_index < candidate_ids.len() {
            candidate_ids.get(end_index).cloned()
        } else {
            None
        };

        let pagination = PaginationMetadata {
            page_size,
            total_results: Some(candidate_ids.len()),
            next_cursor,
            has_more: end_index < candidate_ids.len(),
        };

        let query_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(QueryResult {
            receipts,
            pagination,
            query_time_ms,
        })
    }

    /// Authenticate API key and return user role
    pub fn authenticate(&self, api_key: &str) -> DockLockResult<UserRole> {
        let auth_service = self.auth_service.read().unwrap();
        
        if let Some(role) = auth_service.api_keys.get(api_key) {
            // Update stats
            {
                let mut stats = self.stats.write().unwrap();
                stats.successful_auths += 1;
            }
            Ok(role.clone())
        } else {
            // Update stats
            {
                let mut stats = self.stats.write().unwrap();
                stats.failed_auths += 1;
            }
            Err(DockLockError::AuthenticationError("Invalid API key".to_string()))
        }
    }

    /// Check if user role can access a specific receipt
    fn can_access_receipt(&self, user_role: &UserRole, _receipt: &Receipt) -> bool {
        match user_role {
            UserRole::Admin => true,
            UserRole::Validator => true, // In real implementation, check if validator owns the receipt
            UserRole::Public => self.config.enable_public_access,
        }
    }

    /// Log access attempt for audit trail
    fn log_access(&self, api_key: &str, role: &UserRole, operation: &str, 
                  receipt_id: Option<Uuid>, success: bool, error: Option<String>) {
        let entry = AccessLogEntry {
            id: Uuid::new_v4(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            api_key: api_key.to_string(),
            role: role.clone(),
            operation: operation.to_string(),
            receipt_id,
            success,
            error,
        };

        let mut auth_service = self.auth_service.write().unwrap();
        auth_service.audit_log.push(entry);

        // Trim audit log if it exceeds maximum size
        if auth_service.audit_log.len() > self.config.max_audit_entries {
            let excess = auth_service.audit_log.len() - self.config.max_audit_entries;
        auth_service.audit_log.drain(0..excess);
        }
    }

    /// Add API key with role
    pub fn add_api_key(&self, api_key: String, role: UserRole) -> DockLockResult<()> {
        let mut auth_service = self.auth_service.write().unwrap();
        auth_service.api_keys.insert(api_key, role);
        Ok(())
    }

    /// Get registry statistics
    pub fn get_stats(&self) -> RegistryStats {
        let stats = self.stats.read().unwrap();
        let mut result = stats.clone();
        
        // Update index stats
        let index = self.index.read().unwrap();
        result.index_stats = IndexStats {
            block_height_entries: index.by_block_height.len(),
            timestamp_entries: index.by_timestamp.len(),
            execution_id_entries: index.by_execution_id.len(),
            compliance_status_entries: index.by_compliance_status.len(),
        };

        result
    }

    /// Validate consistency with receipts root
    pub fn validate_consistency(&self, receipts_root: &[u8; 32]) -> DockLockResult<bool> {
        // Compute Merkle root of all stored receipts
        let receipts = self.receipts.read().unwrap();
        let mut receipt_hashes = Vec::<[u8; 32]>::new();
        
        for receipt in receipts.values() {
            let receipt_hash = blake3::hash(receipt.receipt_id.as_bytes());
            let mut hash_array = [0u8; 32];
            hash_array.copy_from_slice(receipt_hash.as_bytes());
            receipt_hashes.push(hash_array);
        }

        // Sort hashes for deterministic ordering
        receipt_hashes.sort();

        // Compute Merkle root
        let computed_root = if receipt_hashes.is_empty() {
            [0u8; 32]
        } else {
            // Use domain-separated hashing for receipt root
            let mut hasher = blake3::Hasher::new();
            hasher.update(&[REGISTRY_HASH]);
            for hash in &receipt_hashes {
                hasher.update(hash);
            }
            let hash = hasher.finalize();
            let mut root = [0u8; 32];
            root.copy_from_slice(hash.as_bytes());
            root
        };

        Ok(computed_root == *receipts_root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::receipt::{ComplianceStatus, ExecutionStats, TraceRoots};
    use std::collections::HashMap;

    fn create_test_receipt(receipt_id: Uuid, session_id: Uuid, timestamp: u64) -> Receipt {
        use crate::receipt::{RunHeader, ResourceLimits, CageConfig, TraceRoots, PolicyInfo, ExecutionStats, 
                            MemoryStats, IoStats, WitnessStats, EventStats};
        use std::collections::HashMap;
        
        let mut environment = HashMap::new();
        environment.insert("BLOCK_HEIGHT".to_string(), "100".to_string());
        
        Receipt {
            receipt_id: receipt_id.to_string(),
            run_header: RunHeader {
                session_id: session_id.to_string(),
                image_hash: "test_image_hash".to_string(),
                command: vec!["test".to_string()],
                environment,
                working_dir: "/tmp".to_string(),
                resource_limits: ResourceLimits {
                    max_memory: 1024 * 1024,
                    max_cpu_time: 5000,
                    max_fs_ops: 1000,
                    max_net_ops: 100,
                },
                cage_config: CageConfig {
                    rng_seed: vec![1, 2, 3, 4],
                    syscall_filter_enabled: true,
                    witness_recording: true,
                    event_correlation: true,
                },
            },
            trace_roots: TraceRoots {
                witness_root: [1u8; 32],
                event_stream_root: [2u8; 32],
                combined_root: [4u8; 32],
                wallet_root: [3u8; 32],
            },
            policy_info: PolicyInfo {
                compliance_status: crate::receipt::ComplianceStatus::Compliant,
                validation_results: Vec::new(),
                violations: Vec::new(),
                regulatory_metadata: HashMap::new(),
            },
            execution_stats: ExecutionStats {
                start_time: timestamp,
                end_time: timestamp + 1000,
                duration_ms: 1000,
                exit_code: 0,
                memory_stats: MemoryStats {
                    peak_memory: 1024,
                    avg_memory: 512,
                    allocations: 10,
                    deallocations: 8,
                },
                io_stats: IoStats {
                    bytes_read: 256,
                    bytes_written: 512,
                    file_reads: 5,
                    file_writes: 3,
                    network_ops: 2,
                },
                witness_stats: WitnessStats {
                    total_entries: 50,
                    total_data_size: 2048,
                    compression_ratio: 0.5,
                    validation_passed: true,
                },
                event_stats: EventStats {
                    total_events: 25,
                    event_types: HashMap::new(),
                    integrity_verified: true,
                },
            },
            timestamp,
            signature: None,
            signer_pubkey: None,
        }
    }

    #[test]
    fn test_registry_creation() {
        let config = RegistryConfig::default();
        let registry = ReceiptRegistry::new("test_registry".to_string(), config);
        
        assert_eq!(registry.name, "test_registry");
        assert_eq!(registry.receipts.read().unwrap().len(), 0);
    }

    #[test]
    fn test_store_and_retrieve_receipt() {
        let config = RegistryConfig::default();
        let registry = ReceiptRegistry::new("test_registry".to_string(), config);
        
        // Add API key
        registry.add_api_key("test_key".to_string(), UserRole::Admin).unwrap();
        
        // Create and store receipt
        let receipt_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();
        let receipt = create_test_receipt(receipt_id, session_id, 1000);
        
        registry.store_receipt(receipt.clone()).unwrap();
        
        // Retrieve receipt
        let retrieved = registry.get_receipt(receipt_id, "test_key").unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().receipt_id, receipt_id.to_string());
    }

    #[test]
    fn test_query_with_pagination() {
        let config = RegistryConfig::default();
        let registry = ReceiptRegistry::new("test_registry".to_string(), config);
        
        // Add API key
        registry.add_api_key("test_key".to_string(), UserRole::Admin).unwrap();
        
        // Store multiple receipts
        for i in 0..10 {
            let receipt_id = Uuid::new_v4();
            let session_id = Uuid::new_v4();
            let receipt = create_test_receipt(receipt_id, session_id, 1000 + i);
            registry.store_receipt(receipt).unwrap();
        }
        
        // Query with pagination
        let params = QueryParams {
            receipt_id: None,
            block_height_range: None,
            timestamp_range: None,
            execution_id: None,
            compliance_status: None,
            pagination: PaginationParams {
                page_size: 5,
                cursor: None,
                offset: None,
            },
        };
        
        let result = registry.query_receipts(params, "test_key").unwrap();
        assert_eq!(result.receipts.len(), 5);
        assert!(result.pagination.has_more);
    }

    #[test]
    fn test_authentication() {
        let config = RegistryConfig::default();
        let registry = ReceiptRegistry::new("test_registry".to_string(), config);
        
        // Add API key
        registry.add_api_key("valid_key".to_string(), UserRole::Admin).unwrap();
        
        // Test valid authentication
        let role = registry.authenticate("valid_key").unwrap();
        assert_eq!(role, UserRole::Admin);
        
        // Test invalid authentication
        let result = registry.authenticate("invalid_key");
        assert!(result.is_err());
    }

    #[test]
    fn test_indexing() {
        let config = RegistryConfig::default();
        let registry = ReceiptRegistry::new("test_registry".to_string(), config);
        
        // Add API key
        registry.add_api_key("test_key".to_string(), UserRole::Admin).unwrap();
        
        // Store receipt
        let receipt_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();
        let receipt = create_test_receipt(receipt_id, session_id, 1000);
        registry.store_receipt(receipt).unwrap();
        
        // Query by session ID (execution ID equivalent)
        let params = QueryParams {
            receipt_id: None,
            block_height_range: None,
            timestamp_range: None,
            execution_id: Some(session_id),
            compliance_status: None,
            pagination: PaginationParams::default(),
        };
        
        let result = registry.query_receipts(params, "test_key").unwrap();
        assert_eq!(result.receipts.len(), 1);
        assert_eq!(result.receipts[0].run_header.session_id, session_id.to_string());
    }

    #[test]
    fn test_consistency_validation() {
        let config = RegistryConfig::default();
        let registry = ReceiptRegistry::new("test_registry".to_string(), config);
        
        // Empty registry should validate against zero root
        let zero_root = [0u8; 32];
        assert!(registry.validate_consistency(&zero_root).unwrap());
        
        // Store receipt and test consistency
        let receipt_id = Uuid::new_v4();
        let execution_id = Uuid::new_v4();
        let receipt = create_test_receipt(receipt_id, execution_id, 1000);
        registry.store_receipt(receipt).unwrap();
        
        // Should not validate against zero root anymore
        assert!(!registry.validate_consistency(&zero_root).unwrap());
    }

    #[test]
    fn test_registry_stats() {
        let config = RegistryConfig::default();
        let registry = ReceiptRegistry::new("test_registry".to_string(), config);
        
        // Add API key
        registry.add_api_key("test_key".to_string(), UserRole::Admin).unwrap();
        
        // Initial stats
        let stats = registry.get_stats();
        assert_eq!(stats.total_receipts, 0);
        assert_eq!(stats.total_queries, 0);
        
        // Store receipt
        let receipt_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();
        let receipt = create_test_receipt(receipt_id, session_id, 1000);
        registry.store_receipt(receipt).unwrap();
        
        // Query receipt
        let params = QueryParams {
            receipt_id: Some(receipt_id),
            block_height_range: None,
            timestamp_range: None,
            execution_id: None,
            compliance_status: None,
            pagination: PaginationParams::default(),
        };
        registry.query_receipts(params, "test_key").unwrap();
        
        // Check updated stats
        let stats = registry.get_stats();
        assert_eq!(stats.total_receipts, 1);
        assert_eq!(stats.total_queries, 1);
        assert!(stats.avg_query_time_ms >= 0.0); // Allow zero for very fast queries
    }
}
