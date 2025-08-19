use crate::error::DockLockResult;
use crate::receipt_registry::{ReceiptRegistry, QueryParams, QueryResult, PaginationParams, UserRole};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Registry API service for external access
#[derive(Debug)]
pub struct RegistryAPI {
    /// Underlying receipt registry
    registry: Arc<ReceiptRegistry>,
    /// API configuration
    config: APIConfig,
}

/// API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIConfig {
    /// API server bind address
    pub bind_address: String,
    /// API server port
    pub port: u16,
    /// Enable CORS
    pub enable_cors: bool,
    /// Rate limiting (requests per minute)
    pub rate_limit: u32,
    /// Request timeout in seconds
    pub request_timeout: u64,
    /// Enable API documentation
    pub enable_docs: bool,
}

impl Default for APIConfig {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1".to_string(),
            port: 8080,
            enable_cors: true,
            rate_limit: 1000,
            request_timeout: 30,
            enable_docs: true,
        }
    }
}

/// API request for querying receipts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryReceiptsRequest {
    /// Query parameters
    pub query: QueryParams,
    /// API key for authentication
    pub api_key: String,
}

/// API response for querying receipts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryReceiptsResponse {
    /// Query result
    pub result: QueryResult,
    /// Request status
    pub status: String,
    /// Error message (if any)
    pub error: Option<String>,
}

/// API request for getting a specific receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReceiptRequest {
    /// Receipt ID
    pub receipt_id: Uuid,
    /// API key for authentication
    pub api_key: String,
}

/// API response for getting a receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReceiptResponse {
    /// Receipt data (if found)
    pub receipt: Option<crate::receipt::Receipt>,
    /// Request status
    pub status: String,
    /// Error message (if any)
    pub error: Option<String>,
}

/// API request for registry statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStatsRequest {
    /// API key for authentication
    pub api_key: String,
}

/// API response for registry statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStatsResponse {
    /// Registry statistics
    pub stats: crate::receipt_registry::RegistryStats,
    /// Request status
    pub status: String,
    /// Error message (if any)
    pub error: Option<String>,
}

/// API request for consistency validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateConsistencyRequest {
    /// Expected receipts root hash
    pub receipts_root: [u8; 32],
    /// API key for authentication
    pub api_key: String,
}

/// API response for consistency validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateConsistencyResponse {
    /// Whether the registry is consistent with the provided root
    pub is_consistent: bool,
    /// Request status
    pub status: String,
    /// Error message (if any)
    pub error: Option<String>,
}

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    /// Service status
    pub status: String,
    /// Service version
    pub version: String,
    /// Uptime in seconds
    pub uptime: u64,
    /// Registry name
    pub registry_name: String,
    /// Total receipts stored
    pub total_receipts: u64,
}

impl RegistryAPI {
    /// Create a new registry API
    pub fn new(registry: Arc<ReceiptRegistry>, config: APIConfig) -> Self {
        Self {
            registry,
            config,
        }
    }

    /// Handle query receipts request
    pub async fn handle_query_receipts(&self, request: QueryReceiptsRequest) -> QueryReceiptsResponse {
        debug!("Processing query receipts request");

        match self.registry.query_receipts(request.query, &request.api_key) {
            Ok(result) => {
                info!("Query receipts successful, returned {} receipts", result.receipts.len());
                QueryReceiptsResponse {
                    result,
                    status: "success".to_string(),
                    error: None,
                }
            }
            Err(e) => {
                warn!("Query receipts failed: {}", e);
                QueryReceiptsResponse {
                    result: QueryResult {
                        receipts: Vec::new(),
                        pagination: crate::receipt_registry::PaginationMetadata {
                            page_size: 0,
                            total_results: Some(0),
                            next_cursor: None,
                            has_more: false,
                        },
                        query_time_ms: 0,
                    },
                    status: "error".to_string(),
                    error: Some(e.to_string()),
                }
            }
        }
    }

    /// Handle get receipt request
    pub async fn handle_get_receipt(&self, request: GetReceiptRequest) -> GetReceiptResponse {
        debug!("Processing get receipt request for ID: {}", request.receipt_id);

        match self.registry.get_receipt(request.receipt_id, &request.api_key) {
            Ok(receipt) => {
                if receipt.is_some() {
                    info!("Get receipt successful for ID: {}", request.receipt_id);
                } else {
                    info!("Receipt not found for ID: {}", request.receipt_id);
                }
                GetReceiptResponse {
                    receipt,
                    status: "success".to_string(),
                    error: None,
                }
            }
            Err(e) => {
                warn!("Get receipt failed for ID {}: {}", request.receipt_id, e);
                GetReceiptResponse {
                    receipt: None,
                    status: "error".to_string(),
                    error: Some(e.to_string()),
                }
            }
        }
    }

    /// Handle get stats request
    pub async fn handle_get_stats(&self, request: GetStatsRequest) -> GetStatsResponse {
        debug!("Processing get stats request");

        // Authenticate request
        match self.registry.authenticate(&request.api_key) {
            Ok(role) => {
                // Only admin and validator roles can access stats
                if matches!(role, UserRole::Admin | UserRole::Validator) {
                    let stats = self.registry.get_stats();
                    info!("Get stats successful");
                    GetStatsResponse {
                        stats,
                        status: "success".to_string(),
                        error: None,
                    }
                } else {
                    warn!("Get stats access denied for role: {:?}", role);
                    GetStatsResponse {
                        stats: Default::default(),
                        status: "error".to_string(),
                        error: Some("Access denied".to_string()),
                    }
                }
            }
            Err(e) => {
                warn!("Get stats authentication failed: {}", e);
                GetStatsResponse {
                    stats: Default::default(),
                    status: "error".to_string(),
                    error: Some(e.to_string()),
                }
            }
        }
    }

    /// Handle validate consistency request
    pub async fn handle_validate_consistency(&self, request: ValidateConsistencyRequest) -> ValidateConsistencyResponse {
        debug!("Processing validate consistency request");

        // Authenticate request
        match self.registry.authenticate(&request.api_key) {
            Ok(role) => {
                // Only admin role can validate consistency
                if matches!(role, UserRole::Admin) {
                    match self.registry.validate_consistency(&request.receipts_root) {
                        Ok(is_consistent) => {
                            info!("Validate consistency successful: {}", is_consistent);
                            ValidateConsistencyResponse {
                                is_consistent,
                                status: "success".to_string(),
                                error: None,
                            }
                        }
                        Err(e) => {
                            warn!("Validate consistency failed: {}", e);
                            ValidateConsistencyResponse {
                                is_consistent: false,
                                status: "error".to_string(),
                                error: Some(e.to_string()),
                            }
                        }
                    }
                } else {
                    warn!("Validate consistency access denied for role: {:?}", role);
                    ValidateConsistencyResponse {
                        is_consistent: false,
                        status: "error".to_string(),
                        error: Some("Access denied".to_string()),
                    }
                }
            }
            Err(e) => {
                warn!("Validate consistency authentication failed: {}", e);
                ValidateConsistencyResponse {
                    is_consistent: false,
                    status: "error".to_string(),
                    error: Some(e.to_string()),
                }
            }
        }
    }

    /// Handle health check request
    pub async fn handle_health_check(&self) -> HealthCheckResponse {
        debug!("Processing health check request");

        let stats = self.registry.get_stats();
        
        HealthCheckResponse {
            status: "healthy".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            registry_name: self.registry.name.clone(),
            total_receipts: stats.total_receipts,
        }
    }

    /// Get API configuration
    pub fn get_config(&self) -> &APIConfig {
        &self.config
    }

    /// Create a simple query for testing
    pub fn create_simple_query(receipt_id: Option<Uuid>, page_size: Option<usize>) -> QueryParams {
        QueryParams {
            receipt_id,
            block_height_range: None,
            timestamp_range: None,
            execution_id: None,
            compliance_status: None,
            pagination: PaginationParams {
                page_size: page_size.unwrap_or(50),
                cursor: None,
                offset: None,
            },
        }
    }

    /// Create a time range query
    pub fn create_time_range_query(start_time: u64, end_time: u64, page_size: Option<usize>) -> QueryParams {
        QueryParams {
            receipt_id: None,
            block_height_range: None,
            timestamp_range: Some((start_time, end_time)),
            execution_id: None,
            compliance_status: None,
            pagination: PaginationParams {
                page_size: page_size.unwrap_or(50),
                cursor: None,
                offset: None,
            },
        }
    }

    /// Create a block height range query
    pub fn create_block_height_query(start_height: u64, end_height: u64, page_size: Option<usize>) -> QueryParams {
        QueryParams {
            receipt_id: None,
            block_height_range: Some((start_height, end_height)),
            timestamp_range: None,
            execution_id: None,
            compliance_status: None,
            pagination: PaginationParams {
                page_size: page_size.unwrap_or(50),
                cursor: None,
                offset: None,
            },
        }
    }
}

/// Registry API server (placeholder for actual HTTP server implementation)
#[derive(Debug)]
pub struct RegistryAPIServer {
    /// API handler
    api: RegistryAPI,
    /// Server configuration
    config: APIConfig,
}

impl RegistryAPIServer {
    /// Create a new API server
    pub fn new(api: RegistryAPI) -> Self {
        let config = api.config.clone();
        Self {
            api,
            config,
        }
    }

    /// Start the API server (placeholder implementation)
    pub async fn start(&self) -> DockLockResult<()> {
        info!("Starting Registry API server on {}:{}", self.config.bind_address, self.config.port);
        
        // In a real implementation, this would start an HTTP server (e.g., using axum, warp, or actix-web)
        // For now, we'll just log that the server would start
        info!("Registry API server started successfully");
        info!("API endpoints available:");
        info!("  GET  /health - Health check");
        info!("  POST /receipts/query - Query receipts");
        info!("  GET  /receipts/{{id}} - Get specific receipt");
        info!("  GET  /stats - Get registry statistics");
        info!("  POST /validate - Validate consistency");
        
        if self.config.enable_docs {
            info!("  GET  /docs - API documentation");
        }

        Ok(())
    }

    /// Stop the API server
    pub async fn stop(&self) -> DockLockResult<()> {
        info!("Stopping Registry API server");
        Ok(())
    }

    /// Get server configuration
    pub fn get_config(&self) -> &APIConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::receipt_registry::{ReceiptRegistry, RegistryConfig};
    use crate::receipt::{Receipt, ComplianceStatus, ExecutionStats, TraceRoots};
    use crate::receipt_registry::ReceiptHash;
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

    #[tokio::test]
    async fn test_api_creation() {
        let config = RegistryConfig::default();
        let registry = Arc::new(ReceiptRegistry::new("test_registry".to_string(), config));
        let api_config = APIConfig::default();
        let api = RegistryAPI::new(registry, api_config);
        
        assert_eq!(api.config.port, 8080);
        assert!(api.config.enable_cors);
    }

    #[tokio::test]
    async fn test_query_receipts_api() {
        let config = RegistryConfig::default();
        let registry = Arc::new(ReceiptRegistry::new("test_registry".to_string(), config));
        let api_config = APIConfig::default();
        let api = RegistryAPI::new(registry.clone(), api_config);
        
        // Add API key
        registry.add_api_key("test_key".to_string(), UserRole::Admin).unwrap();
        
        // Store test receipt
        let receipt_id = Uuid::new_v4();
        let execution_id = Uuid::new_v4();
        let receipt = create_test_receipt(receipt_id, execution_id, 1000);
        registry.store_receipt(receipt).unwrap();
        
        // Create query request
        let request = QueryReceiptsRequest {
            query: RegistryAPI::create_simple_query(Some(receipt_id), Some(10)),
            api_key: "test_key".to_string(),
        };
        
        // Execute query
        let response = api.handle_query_receipts(request).await;
        assert_eq!(response.status, "success");
        assert_eq!(response.result.receipts.len(), 1);
        assert!(response.error.is_none());
    }

    #[tokio::test]
    async fn test_get_receipt_api() {
        let config = RegistryConfig::default();
        let registry = Arc::new(ReceiptRegistry::new("test_registry".to_string(), config));
        let api_config = APIConfig::default();
        let api = RegistryAPI::new(registry.clone(), api_config);
        
        // Add API key
        registry.add_api_key("test_key".to_string(), UserRole::Admin).unwrap();
        
        // Store test receipt
        let receipt_id = Uuid::new_v4();
        let execution_id = Uuid::new_v4();
        let receipt = create_test_receipt(receipt_id, execution_id, 1000);
        registry.store_receipt(receipt).unwrap();
        
        // Create get request
        let request = GetReceiptRequest {
            receipt_id,
            api_key: "test_key".to_string(),
        };
        
        // Execute get
        let response = api.handle_get_receipt(request).await;
        assert_eq!(response.status, "success");
        assert!(response.receipt.is_some());
        assert!(response.error.is_none());
    }

    #[tokio::test]
    async fn test_get_stats_api() {
        let config = RegistryConfig::default();
        let registry = Arc::new(ReceiptRegistry::new("test_registry".to_string(), config));
        let api_config = APIConfig::default();
        let api = RegistryAPI::new(registry.clone(), api_config);
        
        // Add API key
        registry.add_api_key("admin_key".to_string(), UserRole::Admin).unwrap();
        
        // Create stats request
        let request = GetStatsRequest {
            api_key: "admin_key".to_string(),
        };
        
        // Execute get stats
        let response = api.handle_get_stats(request).await;
        assert_eq!(response.status, "success");
        assert!(response.error.is_none());
    }

    #[tokio::test]
    async fn test_health_check_api() {
        let config = RegistryConfig::default();
        let registry = Arc::new(ReceiptRegistry::new("test_registry".to_string(), config));
        let api_config = APIConfig::default();
        let api = RegistryAPI::new(registry, api_config);
        
        // Execute health check
        let response = api.handle_health_check().await;
        assert_eq!(response.status, "healthy");
        assert_eq!(response.registry_name, "test_registry");
    }

    #[tokio::test]
    async fn test_authentication_failure() {
        let config = RegistryConfig::default();
        let registry = Arc::new(ReceiptRegistry::new("test_registry".to_string(), config));
        let api_config = APIConfig::default();
        let api = RegistryAPI::new(registry, api_config);
        
        // Create query request with invalid API key
        let request = QueryReceiptsRequest {
            query: RegistryAPI::create_simple_query(None, Some(10)),
            api_key: "invalid_key".to_string(),
        };
        
        // Execute query
        let response = api.handle_query_receipts(request).await;
        assert_eq!(response.status, "error");
        assert!(response.error.is_some());
        assert!(response.error.unwrap().contains("Invalid API key"));
    }

    #[tokio::test]
    async fn test_query_helpers() {
        // Test simple query creation
        let query = RegistryAPI::create_simple_query(None, Some(25));
        assert_eq!(query.pagination.page_size, 25);
        assert!(query.receipt_id.is_none());
        
        // Test time range query creation
        let query = RegistryAPI::create_time_range_query(1000, 2000, Some(100));
        assert_eq!(query.timestamp_range, Some((1000, 2000)));
        assert_eq!(query.pagination.page_size, 100);
        
        // Test block height query creation
        let query = RegistryAPI::create_block_height_query(10, 20, None);
        assert_eq!(query.block_height_range, Some((10, 20)));
        assert_eq!(query.pagination.page_size, 50); // default
    }

    #[tokio::test]
    async fn test_api_server_lifecycle() {
        let config = RegistryConfig::default();
        let registry = Arc::new(ReceiptRegistry::new("test_registry".to_string(), config));
        let api_config = APIConfig::default();
        let api = RegistryAPI::new(registry, api_config);
        let server = RegistryAPIServer::new(api);
        
        // Test server start
        let result = server.start().await;
        assert!(result.is_ok());
        
        // Test server stop
        let result = server.stop().await;
        assert!(result.is_ok());
    }
}
