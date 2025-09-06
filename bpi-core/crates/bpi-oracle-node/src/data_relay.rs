//! Data Relay module for BPI Oracle Node
//!
//! Handles efficient data synchronization and relay between BPI nodes,
//! including batch processing, compression, and intelligent routing
//! for optimal network performance.

use anyhow::Result;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::{PerformanceConfig, OracleMessage, MessageType, MessagePriority};

/// Data synchronization request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSyncRequest {
    pub request_id: String,
    pub requesting_node: String,
    pub data_type: DataType,
    pub sync_parameters: SyncParameters,
    pub timestamp: DateTime<Utc>,
    pub priority: MessagePriority,
}

/// Types of data that can be synchronized
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DataType {
    /// Blockchain state data
    BlockchainState,
    /// Transaction pool data
    TransactionPool,
    /// Node registry information
    NodeRegistry,
    /// Configuration updates
    Configuration,
    /// Audit logs
    AuditLogs,
    /// Performance metrics
    Metrics,
    /// Custom data type
    Custom(String),
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::BlockchainState => write!(f, "BlockchainState"),
            DataType::TransactionPool => write!(f, "TransactionPool"),
            DataType::NodeRegistry => write!(f, "NodeRegistry"),
            DataType::Configuration => write!(f, "Configuration"),
            DataType::AuditLogs => write!(f, "AuditLogs"),
            DataType::Metrics => write!(f, "Metrics"),
            DataType::Custom(name) => write!(f, "Custom({})", name),
        }
    }
}

/// Parameters for data synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncParameters {
    /// Start timestamp for incremental sync
    pub from_timestamp: Option<DateTime<Utc>>,
    /// End timestamp for bounded sync
    pub to_timestamp: Option<DateTime<Utc>>,
    /// Maximum number of records to sync
    pub max_records: Option<usize>,
    /// Compression preference
    pub compression: CompressionType,
    /// Batch size for chunked transfer
    pub batch_size: usize,
    /// Include metadata in sync
    pub include_metadata: bool,
}

/// Compression types for data transfer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompressionType {
    None,
    Gzip,
    Lz4,
    Zstd,
}

/// Data relay batch for efficient transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataBatch {
    pub batch_id: String,
    pub data_type: DataType,
    pub sequence_number: usize,
    pub total_batches: usize,
    pub compression: CompressionType,
    pub data: Vec<u8>,
    pub checksum: String,
    pub timestamp: DateTime<Utc>,
}

/// Data relay statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRelayStats {
    pub total_sync_requests: u64,
    pub completed_syncs: u64,
    pub failed_syncs: u64,
    pub bytes_transferred: u64,
    pub average_sync_time_ms: f64,
    pub compression_ratio: f64,
    pub active_syncs: usize,
    pub batch_success_rate: f64,
}

/// Active synchronization session
#[derive(Debug, Clone)]
struct SyncSession {
    session_id: String,
    request: DataSyncRequest,
    target_node: String,
    batches_sent: usize,
    total_batches: usize,
    bytes_transferred: u64,
    started_at: DateTime<Utc>,
    last_activity: DateTime<Utc>,
    status: SyncStatus,
}

/// Status of synchronization session
#[derive(Debug, Clone, PartialEq)]
enum SyncStatus {
    Preparing,
    Transferring,
    Completed,
    Failed,
    Cancelled,
}

/// Data relay system for efficient cross-node data transfer
#[derive(Debug)]
pub struct DataRelay {
    config: PerformanceConfig,
    active_sessions: Arc<DashMap<String, SyncSession>>,
    pending_batches: Arc<DashMap<String, VecDeque<DataBatch>>>,
    data_cache: Arc<DashMap<String, Vec<u8>>>,
    stats: Arc<RwLock<DataRelayStats>>,
    batch_handlers: Arc<DashMap<String, tokio::sync::mpsc::Sender<DataBatch>>>,
    shutdown_tx: Arc<Mutex<Option<tokio::sync::broadcast::Sender<()>>>>,
}

impl DataRelay {
    /// Create new data relay system
    pub async fn new(config: PerformanceConfig) -> Result<Self> {
        info!("Initializing Data Relay system");

        Ok(Self {
            config,
            active_sessions: Arc::new(DashMap::new()),
            pending_batches: Arc::new(DashMap::new()),
            data_cache: Arc::new(DashMap::new()),
            stats: Arc::new(RwLock::new(DataRelayStats {
                total_sync_requests: 0,
                completed_syncs: 0,
                failed_syncs: 0,
                bytes_transferred: 0,
                average_sync_time_ms: 0.0,
                compression_ratio: 0.0,
                active_syncs: 0,
                batch_success_rate: 0.0,
            })),
            batch_handlers: Arc::new(DashMap::new()),
            shutdown_tx: Arc::new(Mutex::new(None)),
        })
    }

    /// Start the data relay system
    pub async fn start(&self) -> Result<()> {
        info!("Starting Data Relay system");

        // Start background services
        self.start_background_services().await?;

        info!("✅ Data Relay system started successfully");
        Ok(())
    }

    /// Request data synchronization from another node
    pub async fn request_sync(&self, request: DataSyncRequest, target_node: &str) -> Result<String> {
        info!("Requesting data sync: {} -> {} ({})", 
              request.requesting_node, target_node, request.data_type);

        // Create sync session
        let session_id = Uuid::new_v4().to_string();
        let session = SyncSession {
            session_id: session_id.clone(),
            request: request.clone(),
            target_node: target_node.to_string(),
            batches_sent: 0,
            total_batches: 0,
            bytes_transferred: 0,
            started_at: Utc::now(),
            last_activity: Utc::now(),
            status: SyncStatus::Preparing,
        };

        self.active_sessions.insert(session_id.clone(), session);

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_sync_requests += 1;
        stats.active_syncs = self.active_sessions.len();

        // Send sync request to target node
        self.send_sync_request(&request, target_node).await?;

        info!("✅ Data sync request sent: {}", session_id);
        Ok(session_id)
    }

    /// Process incoming data batch
    pub async fn process_data_batch(&self, batch: DataBatch, from_node: &str) -> Result<()> {
        debug!("Processing data batch: {} from {}", batch.batch_id, from_node);

        // Verify batch integrity
        self.verify_batch_integrity(&batch).await?;

        // Decompress data if needed
        let decompressed_data = self.decompress_data(&batch.data, &batch.compression).await?;

        // Store batch in pending queue
        let batch_key = format!("{}:{}", from_node, batch.data_type);
        self.pending_batches.entry(batch_key.clone())
            .or_insert_with(VecDeque::new)
            .push_back(batch.clone());

        // Check if all batches received
        if let Some(batches) = self.pending_batches.get(&batch_key) {
            if batches.len() == batch.total_batches {
                self.assemble_complete_data(&batch_key, from_node).await?;
            }
        }

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.bytes_transferred += batch.data.len() as u64;

        debug!("✅ Data batch processed: {}", batch.batch_id);
        Ok(())
    }

    /// Relay message to target node
    pub async fn relay_message(&self, target_node: &str, message: OracleMessage) -> Result<()> {
        debug!("Relaying message to node: {}", target_node);

        // Check if high-throughput mode is enabled
        if self.config.high_throughput_mode {
            self.relay_high_throughput(target_node, message).await
        } else {
            self.relay_standard(target_node, message).await
        }
    }

    /// Get data relay statistics
    pub async fn get_stats(&self) -> DataRelayStats {
        self.stats.read().await.clone()
    }

    /// Get active synchronization sessions
    pub async fn get_active_sessions(&self) -> Vec<String> {
        self.active_sessions.iter().map(|entry| entry.key().clone()).collect()
    }

    /// Cancel synchronization session
    pub async fn cancel_sync(&self, session_id: &str) -> Result<()> {
        if let Some(mut session) = self.active_sessions.get_mut(session_id) {
            session.status = SyncStatus::Cancelled;
            info!("Cancelled sync session: {}", session_id);
            
            let mut stats = self.stats.write().await;
            stats.active_syncs = self.active_sessions.len();
        }
        
        Ok(())
    }

    /// Shutdown data relay system
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down Data Relay system");

        // Send shutdown signal
        if let Some(tx) = self.shutdown_tx.lock().await.take() {
            let _ = tx.send(());
        }

        // Cancel all active sessions
        for session in self.active_sessions.iter() {
            let mut session_mut = session.value().clone();
            session_mut.status = SyncStatus::Cancelled;
        }

        self.active_sessions.clear();
        self.pending_batches.clear();
        self.data_cache.clear();

        info!("✅ Data Relay system shutdown complete");
        Ok(())
    }

    /// Send sync request to target node
    async fn send_sync_request(&self, request: &DataSyncRequest, target_node: &str) -> Result<()> {
        // In a real implementation, this would send the request through the communication layer
        debug!("Sending sync request to node: {}", target_node);
        Ok(())
    }

    /// Verify batch integrity
    async fn verify_batch_integrity(&self, batch: &DataBatch) -> Result<()> {
        // Calculate checksum and verify
        let calculated_checksum = self.calculate_checksum(&batch.data).await?;
        if calculated_checksum != batch.checksum {
            return Err(anyhow::anyhow!("Batch checksum mismatch"));
        }
        Ok(())
    }

    /// Calculate data checksum
    async fn calculate_checksum(&self, data: &[u8]) -> Result<String> {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Decompress data based on compression type
    async fn decompress_data(&self, data: &[u8], compression: &CompressionType) -> Result<Vec<u8>> {
        match compression {
            CompressionType::None => Ok(data.to_vec()),
            CompressionType::Gzip => {
                // In a real implementation, would use actual gzip decompression
                debug!("Decompressing gzip data");
                Ok(data.to_vec())
            }
            CompressionType::Lz4 => {
                debug!("Decompressing lz4 data");
                Ok(data.to_vec())
            }
            CompressionType::Zstd => {
                debug!("Decompressing zstd data");
                Ok(data.to_vec())
            }
        }
    }

    /// Assemble complete data from all batches
    async fn assemble_complete_data(&self, batch_key: &str, from_node: &str) -> Result<()> {
        if let Some((_, batches)) = self.pending_batches.remove(batch_key) {
            info!("Assembling complete data from {} batches", batches.len());

            // Sort batches by sequence number
            let mut sorted_batches: Vec<_> = batches.into_iter().collect();
            sorted_batches.sort_by_key(|b| b.sequence_number);

            // Combine all batch data
            let mut complete_data = Vec::new();
            for batch in sorted_batches {
                let decompressed = self.decompress_data(&batch.data, &batch.compression).await?;
                complete_data.extend(decompressed);
            }

            // Store complete data in cache
            let cache_key = format!("{}:{}", from_node, batch_key);
            self.data_cache.insert(cache_key, complete_data);

            // Update statistics
            let mut stats = self.stats.write().await;
            stats.completed_syncs += 1;

            info!("✅ Complete data assembled for: {}", batch_key);
        }

        Ok(())
    }

    /// Relay message in standard mode
    async fn relay_standard(&self, target_node: &str, message: OracleMessage) -> Result<()> {
        // Standard message relay with basic queuing
        debug!("Standard relay to {}: {}", target_node, message.message_id);
        Ok(())
    }

    /// Relay message in high-throughput mode
    async fn relay_high_throughput(&self, target_node: &str, message: OracleMessage) -> Result<()> {
        // High-throughput relay with batching and optimization
        debug!("High-throughput relay to {}: {}", target_node, message.message_id);
        Ok(())
    }

    /// Start background services
    async fn start_background_services(&self) -> Result<()> {
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel(1);
        *self.shutdown_tx.lock().await = Some(shutdown_tx);

        // Session cleanup service
        let active_sessions = Arc::clone(&self.active_sessions);
        let stats = Arc::clone(&self.stats);
        let mut shutdown_rx_cleanup = shutdown_rx.resubscribe();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
            
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        let now = Utc::now();
                        let mut expired_sessions = Vec::new();

                        // Find expired sessions (inactive for > 5 minutes)
                        for session in active_sessions.iter() {
                            let inactive_duration = now - session.last_activity;
                            if inactive_duration.num_seconds() > 300 {
                                expired_sessions.push(session.session_id.clone());
                            }
                        }

                        // Remove expired sessions
                        for session_id in expired_sessions {
                            if let Some((_, mut session)) = active_sessions.remove(&session_id) {
                                session.status = SyncStatus::Failed;
                                warn!("Expired sync session: {}", session_id);
                                
                                let mut stats_guard = stats.write().await;
                                stats_guard.failed_syncs += 1;
                                stats_guard.active_syncs = active_sessions.len();
                            }
                        }
                    }
                    _ = shutdown_rx_cleanup.recv() => break,
                }
            }
        });

        // Cache cleanup service
        let data_cache = Arc::clone(&self.data_cache);
        let mut shutdown_rx_cache = shutdown_rx.resubscribe();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // 5 minutes
            
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        // Clean up old cached data (keep only recent data)
                        if data_cache.len() > 1000 {
                            // In a real implementation, would implement LRU eviction
                            debug!("Cache size: {}, cleanup needed", data_cache.len());
                        }
                    }
                    _ = shutdown_rx_cache.recv() => break,
                }
            }
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_data_relay_creation() {
        let config = PerformanceConfig {
            batch_size: 100,
            connection_pool_size: 50,
            message_cache_size: 10000,
            high_throughput_mode: false,
        };
        
        let relay = DataRelay::new(config).await.unwrap();
        let stats = relay.get_stats().await;
        
        assert_eq!(stats.total_sync_requests, 0);
        assert_eq!(stats.active_syncs, 0);
    }

    #[tokio::test]
    async fn test_sync_request() {
        let config = PerformanceConfig {
            batch_size: 100,
            connection_pool_size: 50,
            message_cache_size: 10000,
            high_throughput_mode: false,
        };
        
        let relay = DataRelay::new(config).await.unwrap();
        
        let request = DataSyncRequest {
            request_id: "test-sync-1".to_string(),
            requesting_node: "node-1".to_string(),
            data_type: DataType::BlockchainState,
            sync_parameters: SyncParameters {
                from_timestamp: None,
                to_timestamp: None,
                max_records: Some(1000),
                compression: CompressionType::Gzip,
                batch_size: 100,
                include_metadata: true,
            },
            timestamp: Utc::now(),
            priority: MessagePriority::Normal,
        };

        let session_id = relay.request_sync(request, "target-node").await.unwrap();
        assert!(!session_id.is_empty());
        
        let active_sessions = relay.get_active_sessions().await;
        assert_eq!(active_sessions.len(), 1);
    }
}
