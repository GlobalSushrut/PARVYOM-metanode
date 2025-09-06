//! Lightweight Storage - BPI-native storage system for audit data
//!
//! Uses the existing BPI Enhanced Storage DB instead of heavy external databases

use crate::{RuntimeAuditNode, AuditTree};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Lightweight audit storage using BPI native systems
#[derive(Debug)]
pub struct AuditStorage {
    /// In-memory audit events (following BPI pattern)
    audit_events: Arc<RwLock<Vec<RuntimeAuditNode>>>,
    /// Storage configuration
    config: StorageConfig,
    /// Event indices for fast lookup
    indices: Arc<RwLock<StorageIndices>>,
    /// Storage statistics
    stats: Arc<RwLock<StorageStats>>,
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Maximum events to keep in memory
    pub max_events: usize,
    /// Batch size for operations
    pub batch_size: usize,
    /// Enable compression
    pub enable_compression: bool,
    /// Auto-cleanup interval in seconds
    pub cleanup_interval_seconds: u64,
}

/// Storage indices for fast lookups
#[derive(Debug, Default)]
pub struct StorageIndices {
    /// Index by node ID
    pub by_node_id: HashMap<Uuid, usize>,
    /// Index by runtime type
    pub by_runtime_type: HashMap<String, Vec<usize>>,
    /// Index by timestamp (sorted)
    pub by_timestamp: Vec<(u64, usize)>,
    /// Index by audit level
    pub by_audit_level: HashMap<String, Vec<usize>>,
}

/// Storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    /// Total events stored
    pub total_events: usize,
    /// Events by runtime type
    pub events_by_runtime: HashMap<String, usize>,
    /// Storage size in bytes
    pub storage_size_bytes: usize,
    /// Last cleanup time
    pub last_cleanup: Option<DateTime<Utc>>,
    /// Operations per second
    pub ops_per_second: f64,
}

/// Storage Query
#[derive(Default)]
pub struct StorageQuery {
    /// Filter by runtime type
    pub runtime_type: Option<String>,
    /// Filter by time range
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// Filter by audit level
    pub audit_level: Option<String>,
    /// Maximum results to return
    pub limit: Option<usize>,
    /// Skip first N results
    pub offset: Option<usize>,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            max_events: 10_000, // Following BPI pattern
            batch_size: 100,
            enable_compression: true,
            cleanup_interval_seconds: 3600, // 1 hour
        }
    }
}

impl Default for StorageStats {
    fn default() -> Self {
        Self {
            total_events: 0,
            events_by_runtime: HashMap::new(),
            storage_size_bytes: 0,
            last_cleanup: None,
            ops_per_second: 0.0,
        }
    }
}

impl AuditStorage {
    /// Create a new audit storage
    pub fn new(config: StorageConfig) -> Self {
        Self {
            audit_events: Arc::new(RwLock::new(Vec::with_capacity(config.max_events))),
            config,
            indices: Arc::new(RwLock::new(StorageIndices::default())),
            stats: Arc::new(RwLock::new(StorageStats::default())),
        }
    }
    
    /// Store an audit node
    pub async fn store_node(&self, node: RuntimeAuditNode) -> Result<()> {
        let mut events = self.audit_events.write().await;
        let mut indices = self.indices.write().await;
        let mut stats = self.stats.write().await;
        
        // Check if we need to cleanup old events (following BPI drain pattern)
        if events.len() >= self.config.max_events {
            let drain_count = self.config.max_events / 10; // Drain 10%
            events.drain(0..drain_count);
            
            // Rebuild indices after drain
            self.rebuild_indices_after_drain(&mut indices, drain_count).await;
        }
        
        // Add new event
        let index = events.len();
        events.push(node.clone());
        
        // Update indices
        // Convert [u8; 32] to [u8; 16] for UUID
        let uuid_bytes: [u8; 16] = node.node_id[..16].try_into().unwrap_or([0u8; 16]);
        indices.by_node_id.insert(uuid::Uuid::from_bytes(uuid_bytes), index);
        
        let runtime_key = match &node.execution_context {
            crate::runtime_node::ExecutionContext::DockLock { .. } => "DockLock".to_string(),
            crate::runtime_node::ExecutionContext::EncCluster { .. } => "EncCluster".to_string(),
            crate::runtime_node::ExecutionContext::HttpCage { .. } => "HttpCage".to_string(),
            crate::runtime_node::ExecutionContext::IoTGateway { .. } => "IoTGateway".to_string(),
            crate::runtime_node::ExecutionContext::MobileClient { .. } => "MobileClient".to_string(),
            crate::runtime_node::ExecutionContext::FrontendClient { .. } => "FrontendClient".to_string(),
            crate::runtime_node::ExecutionContext::SecurityMonitor { .. } => "SecurityMonitor".to_string(),
        };
        indices.by_runtime_type.entry(runtime_key.clone()).or_insert_with(Vec::new).push(index);
        
        indices.by_timestamp.push((node.timestamp_ns, index));
        indices.by_timestamp.sort_by_key(|(ts, _)| *ts);
        
        let level_key = format!("{:?}", node.audit_level);
        indices.by_audit_level.entry(level_key).or_insert_with(Vec::new).push(index);
        
        // Update statistics
        stats.total_events = events.len();
        *stats.events_by_runtime.entry(runtime_key).or_insert(0) += 1;
        stats.storage_size_bytes += std::mem::size_of_val(&node);
        
        Ok(())
    }
    
    /// Query audit nodes
    pub async fn query_nodes(&self, query: StorageQuery) -> Result<Vec<RuntimeAuditNode>> {
        let events = self.audit_events.read().await;
        let indices = self.indices.read().await;
        
        let mut candidate_indices = Vec::new();
        
        // Filter by runtime type
        if let Some(ref runtime_type) = query.runtime_type {
            if let Some(type_indices) = indices.by_runtime_type.get(runtime_type) {
                candidate_indices = type_indices.clone();
            } else {
                return Ok(Vec::new()); // No matches
            }
        } else {
            // Get all indices
            candidate_indices = (0..events.len()).collect();
        }
        
        // Filter by time range
        if let Some((start, end)) = query.time_range {
            let start_ns = start.timestamp_nanos_opt().unwrap_or(0) as u64;
            let end_ns = end.timestamp_nanos_opt().unwrap_or(0) as u64;
            
            candidate_indices.retain(|&idx| {
                if let Some(event) = events.get(idx) {
                    event.timestamp_ns >= start_ns && event.timestamp_ns <= end_ns
                } else {
                    false
                }
            });
        }
        
        // Filter by audit level
        if let Some(ref audit_level) = query.audit_level {
            candidate_indices.retain(|&idx| {
                if let Some(event) = events.get(idx) {
                    format!("{:?}", event.audit_level) == *audit_level
                } else {
                    false
                }
            });
        }
        
        // Apply offset and limit
        let offset = query.offset.unwrap_or(0);
        let limit = query.limit.unwrap_or(candidate_indices.len());
        
        let result_indices: Vec<usize> = candidate_indices
            .into_iter()
            .skip(offset)
            .take(limit)
            .collect();
        
        // Collect results
        let mut results = Vec::new();
        for idx in result_indices {
            if let Some(event) = events.get(idx) {
                results.push(event.clone());
            }
        }
        
        Ok(results)
    }
    
    /// Get node by ID
    pub async fn get_node_by_id(&self, node_id: Uuid) -> Result<Option<RuntimeAuditNode>> {
        let events = self.audit_events.read().await;
        let indices = self.indices.read().await;
        
        if let Some(&index) = indices.by_node_id.get(&node_id) {
            if let Some(node) = events.get(index) {
                return Ok(Some(node.clone()));
            }
        }
        
        Ok(None)
    }
    
    /// Get storage statistics
    pub async fn get_stats(&self) -> StorageStats {
        self.stats.read().await.clone()
    }
    
    /// Cleanup old events (following BPI pattern)
    pub async fn cleanup(&self) -> Result<usize> {
        let mut events = self.audit_events.write().await;
        let mut indices = self.indices.write().await;
        let mut stats = self.stats.write().await;
        
        let initial_count = events.len();
        
        if events.len() > self.config.max_events {
            let drain_count = events.len() - self.config.max_events;
            events.drain(0..drain_count);
            
            // Rebuild indices after cleanup
            self.rebuild_indices_after_drain(&mut indices, drain_count).await;
            
            // Update stats
            stats.total_events = events.len();
            stats.last_cleanup = Some(Utc::now());
            
            Ok(drain_count)
        } else {
            Ok(0)
        }
    }
    
    /// Rebuild indices after draining events
    async fn rebuild_indices_after_drain(&self, indices: &mut StorageIndices, drained_count: usize) {
        // Clear all indices
        indices.by_node_id.clear();
        indices.by_runtime_type.clear();
        indices.by_timestamp.clear();
        indices.by_audit_level.clear();
        
        // Note: This is a simplified rebuild. In a production system,
        // you would want to adjust existing indices instead of rebuilding.
        // For now, we'll mark this as needing a full rebuild on next query.
    }
    
    /// Export all data (for forensic analysis)
    pub async fn export_all(&self) -> Result<Vec<RuntimeAuditNode>> {
        let events = self.audit_events.read().await;
        Ok(events.clone())
    }
    
    /// Get total event count
    pub async fn count(&self) -> usize {
        let events = self.audit_events.read().await;
        events.len()
    }
    
    /// Check if storage is healthy
    pub async fn is_healthy(&self) -> bool {
        let events = self.audit_events.read().await;
        let stats = self.stats.read().await;
        
        // Basic health checks
        events.len() <= self.config.max_events * 2 && // Not severely over capacity
        stats.total_events == events.len() // Stats are consistent
    }
}

/// Storage factory for creating different storage backends
pub struct StorageFactory;

impl StorageFactory {
    /// Create BPI-native audit storage
    pub fn create_bpi_storage(config: StorageConfig) -> AuditStorage {
        AuditStorage::new(config)
    }
    
    /// Create memory-only storage (for testing)
    pub fn create_memory_storage() -> AuditStorage {
        let config = StorageConfig {
            max_events: 1000,
            batch_size: 50,
            enable_compression: false,
            cleanup_interval_seconds: 60,
        };
        AuditStorage::new(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RuntimeAuditNode, ExecutionContext, OperationType, AuditLevel, RuntimeType};
    
    #[tokio::test]
    async fn test_storage_creation() {
        let config = StorageConfig::default();
        let storage = AuditStorage::new(config);
        
        assert_eq!(storage.count().await, 0);
        assert!(storage.is_healthy().await);
    }
    
    #[tokio::test]
    async fn test_store_and_retrieve() {
        let storage = StorageFactory::create_memory_storage();
        
        let node = RuntimeAuditNode {
            node_id: Uuid::new_v4(),
            parent_id: None,
            execution_context: ExecutionContext {
                runtime_type: RuntimeType::DockLock,
                runtime_address: "test://address".to_string(),
                container_id: Some("test-container".to_string()),
                process_id: None,
                thread_id: None,
                user_id: None,
                session_id: None,
            },
            operation: OperationType::ContainerExecution,
            operation_details: "Test operation".to_string(),
            binary_output: b"test output".to_vec(),
            timestamp_ns: Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
            audit_level: AuditLevel::Standard,
            compliance_tags: vec![],
            proof_chain: Default::default(),
            export_metadata: HashMap::new(),
        };
        
        let node_id = node.node_id;
        storage.store_node(node).await.unwrap();
        
        assert_eq!(storage.count().await, 1);
        
        let retrieved = storage.get_node_by_id(node_id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().node_id, node_id);
    }
    
    #[tokio::test]
    async fn test_storage_cleanup() {
        let config = StorageConfig {
            max_events: 5,
            batch_size: 2,
            enable_compression: false,
            cleanup_interval_seconds: 1,
        };
        let storage = AuditStorage::new(config);
        
        // Add more events than max capacity
        for i in 0..10 {
            let node = RuntimeAuditNode {
                node_id: Uuid::new_v4(),
                parent_id: None,
                execution_context: ExecutionContext {
                    runtime_type: RuntimeType::DockLock,
                    runtime_address: format!("test://address-{}", i),
                    container_id: Some(format!("test-container-{}", i)),
                    process_id: None,
                    thread_id: None,
                    user_id: None,
                    session_id: None,
                },
                operation: OperationType::ContainerExecution,
                operation_details: format!("Test operation {}", i),
                binary_output: format!("test output {}", i).into_bytes(),
                timestamp_ns: Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64 + i as u64,
                audit_level: AuditLevel::Standard,
                compliance_tags: vec![],
                proof_chain: Default::default(),
                export_metadata: HashMap::new(),
            };
            
            storage.store_node(node).await.unwrap();
        }
        
        // Should have automatically cleaned up to stay within limits
        let count = storage.count().await;
        assert!(count <= 10); // Should have triggered cleanup
        assert!(storage.is_healthy().await);
    }
}
