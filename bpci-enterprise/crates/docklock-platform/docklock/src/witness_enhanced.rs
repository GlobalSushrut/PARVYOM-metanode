//! Enhanced witness recording with compression and event stream integration
//! Stage 27: Witness Log & I/O Recording Integration

use crate::error::{DockLockError, DockLockResult};
use crate::event_stream::CanonicalEventStream;
use crate::witness::{WitnessEntry, WitnessData, WitnessOperationType};
use bpi_enc::domain_hash;
use bpi_merkle::MerkleTree;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use tracing::{debug, info};


/// Domain tag for enhanced witness entry hashing
pub const ENHANCED_WITNESS_HASH: &str = "ENHANCED_WITNESS";

/// Compression algorithms supported for witness data
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CompressionAlgorithm {
    None,
    Lz4,
    Zstd,
}

/// Enhanced witness entry with compression and event correlation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedWitnessEntry {
    /// Base witness entry
    pub base_entry: WitnessEntry,
    /// Correlated event ID from canonical event stream
    pub event_id: Option<u128>,
    /// Compression algorithm used for data
    pub compression: CompressionAlgorithm,
    /// Compressed data size
    pub compressed_size: usize,
    /// Original data size before compression
    pub original_size: usize,
    /// Additional metadata for enhanced tracking
    pub metadata: HashMap<String, String>,
}

impl EnhancedWitnessEntry {
    /// Create a new enhanced witness entry
    pub fn new(
        base_entry: WitnessEntry,
        event_id: Option<u128>,
        compression: CompressionAlgorithm,
    ) -> DockLockResult<Self> {
        let original_size = base_entry.data.estimated_size();
        let compressed_size = match compression {
            CompressionAlgorithm::None => original_size,
            CompressionAlgorithm::Lz4 => {
                // Estimate LZ4 compression ratio (typically 2-4x)
                original_size / 3
            }
            CompressionAlgorithm::Zstd => {
                // Estimate Zstd compression ratio (typically 3-5x)
                original_size / 4
            }
        };

        Ok(Self {
            base_entry,
            event_id,
            compression,
            compressed_size,
            original_size,
            metadata: HashMap::new(),
        })
    }

    /// Add metadata to the enhanced witness entry
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Get compression ratio
    pub fn compression_ratio(&self) -> f64 {
        if self.original_size == 0 {
            1.0
        } else {
            self.original_size as f64 / self.compressed_size as f64
        }
    }

    /// Compute hash of the enhanced witness entry
    pub fn compute_hash(&self) -> DockLockResult<[u8; 32]> {
        let encoded = self.encode()?;
        Ok(domain_hash(ENHANCED_WITNESS_HASH, &encoded))
    }

    /// Encode the enhanced witness entry
    pub fn encode(&self) -> DockLockResult<Vec<u8>> {
        bincode::serialize(self)
            .map_err(|e| DockLockError::EncodingError(format!("Failed to encode enhanced witness entry: {}", e)))
    }
}

impl WitnessData {
    /// Estimate the size of witness data
    pub fn estimated_size(&self) -> usize {
        match self {
            WitnessData::FileOperation { path, data, .. } => {
                path.len() + data.len() + 32 // path + data + overhead
            }
            WitnessData::SyscallResult { syscall_name, args, .. } => {
                syscall_name.len() + args.len() * 8 + 16 // name + args + overhead
            }
            WitnessData::EnvAccess { var_name, value, .. } => {
                var_name.len() + value.as_ref().map(|v| v.len()).unwrap_or(0) + 16
            }
            WitnessData::RandomData { data } => data.len() + 8,
            WitnessData::RandomGeneration { seed, output } => seed.len() + output.len() + 16,
        }
    }
}

/// Compressed witness log with enhanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedWitnessLog {
    /// Enhanced witness entries
    entries: Vec<EnhancedWitnessEntry>,
    /// Maximum size before compression
    max_size: usize,
    /// Current sequence number
    sequence: u64,
    /// Merkle tree for integrity verification (not serialized)
    #[serde(skip)]
    merkle_tree: Option<MerkleTree>,
    /// Compression statistics
    compression_stats: CompressionStats,
    /// Event correlation mapping
    event_correlations: HashMap<u128, Vec<usize>>, // event_id -> witness_indices
}

/// Statistics about compression performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionStats {
    pub total_entries: usize,
    pub total_original_size: usize,
    pub total_compressed_size: usize,
    pub average_compression_ratio: f64,
    pub lz4_entries: usize,
    pub zstd_entries: usize,
    pub uncompressed_entries: usize,
}

impl Default for CompressionStats {
    fn default() -> Self {
        Self {
            total_entries: 0,
            total_original_size: 0,
            total_compressed_size: 0,
            average_compression_ratio: 1.0,
            lz4_entries: 0,
            zstd_entries: 0,
            uncompressed_entries: 0,
        }
    }
}

impl CompressedWitnessLog {
    /// Create a new compressed witness log
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_size,
            sequence: 0,
            merkle_tree: None,
            compression_stats: CompressionStats::default(),
            event_correlations: HashMap::new(),
        }
    }

    /// Add an enhanced witness entry
    pub fn add_enhanced_entry(&mut self, entry: EnhancedWitnessEntry) -> DockLockResult<()> {
        // Update compression statistics
        self.compression_stats.total_entries += 1;
        self.compression_stats.total_original_size += entry.original_size;
        self.compression_stats.total_compressed_size += entry.compressed_size;

        match entry.compression {
            CompressionAlgorithm::None => self.compression_stats.uncompressed_entries += 1,
            CompressionAlgorithm::Lz4 => self.compression_stats.lz4_entries += 1,
            CompressionAlgorithm::Zstd => self.compression_stats.zstd_entries += 1,
        }

        // Update average compression ratio
        if self.compression_stats.total_original_size > 0 {
            self.compression_stats.average_compression_ratio = 
                self.compression_stats.total_original_size as f64 / 
                self.compression_stats.total_compressed_size as f64;
        }

        // Add event correlation if present
        if let Some(event_id) = entry.event_id {
            let entry_index = self.entries.len();
            self.event_correlations
                .entry(event_id)
                .or_insert_with(Vec::new)
                .push(entry_index);
        }

        self.entries.push(entry);
        self.merkle_tree = None; // Invalidate Merkle tree
        
        debug!(
            "Added enhanced witness entry {}, compression ratio: {:.2}x",
            self.entries.len() - 1,
            self.entries.last().unwrap().compression_ratio()
        );

        Ok(())
    }

    /// Add a witness entry from the base system with automatic compression
    pub fn add_witness_entry(
        &mut self,
        operation_type: WitnessOperationType,
        pid: u32,
        tid: u32,
        data: WitnessData,
        event_id: Option<u128>,
    ) -> DockLockResult<()> {
        self.sequence += 1;
        
        let base_entry = WitnessEntry::new(
            self.sequence,
            operation_type,
            pid,
            tid,
            data,
        )?;

        // Choose compression algorithm based on data size
        let compression = if base_entry.data.estimated_size() > 1024 {
            CompressionAlgorithm::Zstd // Better compression for larger data
        } else if base_entry.data.estimated_size() > 256 {
            CompressionAlgorithm::Lz4 // Faster compression for medium data
        } else {
            CompressionAlgorithm::None // No compression for small data
        };

        let enhanced_entry = EnhancedWitnessEntry::new(base_entry, event_id, compression)?;
        self.add_enhanced_entry(enhanced_entry)
    }

    /// Compute Merkle root of all enhanced witness entries
    pub fn compute_merkle_root(&mut self) -> DockLockResult<[u8; 32]> {
        if self.entries.is_empty() {
            return Ok([0u8; 32]);
        }

        let hashes: Result<Vec<[u8; 32]>, DockLockError> = self.entries
            .iter()
            .map(|entry| entry.compute_hash())
            .collect();

        let hashes = hashes?;
        let hash_vecs: Vec<Vec<u8>> = hashes.iter().map(|h| h.to_vec()).collect();
        let merkle_tree = MerkleTree::new(hash_vecs)
            .map_err(|e| DockLockError::MerkleError(format!("Failed to create Merkle tree: {}", e)))?;

        let root = merkle_tree.root()
            .map_err(|e| DockLockError::MerkleError(format!("Failed to get Merkle root: {}", e)))?;
        
        let root_bytes: [u8; 32] = root.try_into()
            .map_err(|_| DockLockError::MerkleError("Invalid root size".to_string()))?;
        
        self.merkle_tree = Some(merkle_tree);
        
        info!(
            "Computed Merkle root for {} enhanced witness entries: {}",
            self.entries.len(),
            hex::encode(&root_bytes)
        );

        Ok(root_bytes)
    }

    /// Get witness entries correlated with a specific event
    pub fn get_entries_for_event(&self, event_id: u128) -> Vec<&EnhancedWitnessEntry> {
        if let Some(indices) = self.event_correlations.get(&event_id) {
            indices.iter()
                .filter_map(|&index| self.entries.get(index))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get compression statistics
    pub fn compression_stats(&self) -> &CompressionStats {
        &self.compression_stats
    }

    /// Get the number of entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the log is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get estimated total size with compression
    pub fn compressed_size(&self) -> usize {
        self.compression_stats.total_compressed_size
    }

    /// Get original size without compression
    pub fn original_size(&self) -> usize {
        self.compression_stats.total_original_size
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.sequence = 0;
        self.merkle_tree = None;
        self.compression_stats = CompressionStats::default();
        self.event_correlations.clear();
    }
}

/// Enhanced witness recorder with event stream integration
#[derive(Debug)]
pub struct EnhancedWitnessRecorder {
    /// Compressed witness log
    log: CompressedWitnessLog,
    /// Whether recording is enabled
    enabled: bool,
    /// Target process ID to monitor (None = all processes)
    target_pid: Option<u32>,
    /// Event stream for correlation
    event_stream: Option<Arc<RwLock<CanonicalEventStream>>>,
    /// Recording configuration
    config: WitnessRecorderConfig,
}

/// Configuration for enhanced witness recording
#[derive(Debug, Clone)]
pub struct WitnessRecorderConfig {
    /// Maximum log size before rotation
    pub max_log_size: usize,
    /// Default compression algorithm
    pub default_compression: CompressionAlgorithm,
    /// Enable automatic event correlation
    pub enable_event_correlation: bool,
    /// Minimum data size for compression
    pub compression_threshold: usize,
}

impl Default for WitnessRecorderConfig {
    fn default() -> Self {
        Self {
            max_log_size: 100 * 1024 * 1024, // 100MB
            default_compression: CompressionAlgorithm::Lz4,
            enable_event_correlation: true,
            compression_threshold: 256,
        }
    }
}

impl EnhancedWitnessRecorder {
    /// Create a new enhanced witness recorder
    pub fn new(config: WitnessRecorderConfig) -> Self {
        Self {
            log: CompressedWitnessLog::new(config.max_log_size),
            enabled: false,
            target_pid: None,
            event_stream: None,
            config,
        }
    }

    /// Set the event stream for correlation
    pub fn set_event_stream(&mut self, event_stream: Arc<RwLock<CanonicalEventStream>>) {
        self.event_stream = Some(event_stream);
    }

    /// Enable or disable recording
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        info!("Enhanced witness recording {}", if enabled { "enabled" } else { "disabled" });
    }

    /// Set target process ID to monitor
    pub fn set_target_pid(&mut self, pid: u32) {
        self.target_pid = Some(pid);
        info!("Enhanced witness recording targeting PID: {}", pid);
    }

    /// Record a file operation with automatic event correlation
    pub fn record_file_operation(
        &mut self,
        operation_type: WitnessOperationType,
        pid: u32,
        tid: u32,
        path: &str,
        offset: u64,
        data: &[u8],
        result: i64,
    ) -> DockLockResult<()> {
        if !self.should_record(pid) {
            return Ok(());
        }

        let witness_data = WitnessData::FileOperation {
            path: path.to_string(),
            offset,
            data: data.to_vec(),
            result,
        };

        // Try to correlate with recent events if event stream is available
        let event_id = if self.config.enable_event_correlation {
            self.find_correlated_event(&format!("file_op:{}", path))
        } else {
            None
        };

        self.log.add_witness_entry(operation_type, pid, tid, witness_data, event_id)?;

        debug!(
            "Recorded file operation: {} at {} (PID: {}, TID: {}, Event: {:?})",
            path, offset, pid, tid, event_id
        );

        Ok(())
    }

    /// Record a syscall result with automatic event correlation
    pub fn record_syscall_result(
        &mut self,
        pid: u32,
        tid: u32,
        syscall_name: &str,
        args: &[u64],
        result: i64,
        errno: i32,
    ) -> DockLockResult<()> {
        if !self.should_record(pid) {
            return Ok(());
        }

        let witness_data = WitnessData::SyscallResult {
            syscall_name: syscall_name.to_string(),
            args: args.to_vec(),
            result,
            errno,
        };

        // Try to correlate with recent events
        let event_id = if self.config.enable_event_correlation {
            self.find_correlated_event(&format!("syscall:{}", syscall_name))
        } else {
            None
        };

        self.log.add_witness_entry(
            WitnessOperationType::SyscallResult,
            pid,
            tid,
            witness_data,
            event_id,
        )?;

        debug!(
            "Recorded syscall: {} -> {} (PID: {}, TID: {}, Event: {:?})",
            syscall_name, result, pid, tid, event_id
        );

        Ok(())
    }

    /// Record environment variable access
    pub fn record_env_access(
        &mut self,
        pid: u32,
        tid: u32,
        var_name: &str,
        value: Option<&str>,
    ) -> DockLockResult<()> {
        if !self.should_record(pid) {
            return Ok(());
        }

        let witness_data = WitnessData::EnvAccess {
            var_name: var_name.to_string(),
            value: value.map(|v| v.to_string()),
        };

        let event_id = if self.config.enable_event_correlation {
            self.find_correlated_event(&format!("env:{}", var_name))
        } else {
            None
        };

        self.log.add_witness_entry(
            WitnessOperationType::EnvAccess,
            pid,
            tid,
            witness_data,
            event_id,
        )?;

        Ok(())
    }

    /// Find a correlated event in the event stream
    fn find_correlated_event(&self, _operation_hint: &str) -> Option<u128> {
        if let Some(ref event_stream) = self.event_stream {
            if let Ok(stream) = event_stream.read() {
                // Look for recent events that might correlate with this operation
                // This is a simplified correlation - in practice, this would be more sophisticated
                if let Some(recent_event) = stream.get_recent_events(1).first() {
                    return Some(recent_event.eid);
                }
            }
        }
        None
    }

    /// Check if we should record for this PID
    fn should_record(&self, pid: u32) -> bool {
        self.enabled && (self.target_pid.is_none() || self.target_pid == Some(pid))
    }

    /// Get the compressed witness log
    pub fn log(&self) -> &CompressedWitnessLog {
        &self.log
    }

    /// Get mutable access to the compressed witness log
    pub fn log_mut(&mut self) -> &mut CompressedWitnessLog {
        &mut self.log
    }

    /// Get recording configuration
    pub fn config(&self) -> &WitnessRecorderConfig {
        &self.config
    }

    /// Compute Merkle root of all witness entries
    pub fn compute_merkle_root(&mut self) -> DockLockResult<[u8; 32]> {
        self.log.compute_merkle_root()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::witness::WitnessData;

    #[test]
    fn test_enhanced_witness_entry_creation() {
        let base_entry = WitnessEntry::new(
            1,
            WitnessOperationType::FileRead,
            1000,
            2000,
            WitnessData::FileOperation {
                path: "/test/file".to_string(),
                offset: 0,
                data: b"test data".to_vec(),
                result: 9,
            },
        ).unwrap();

        let enhanced_entry = EnhancedWitnessEntry::new(
            base_entry,
            Some(12345),
            CompressionAlgorithm::Lz4,
        ).unwrap();

        assert_eq!(enhanced_entry.event_id, Some(12345));
        assert_eq!(enhanced_entry.compression, CompressionAlgorithm::Lz4);
        assert!(enhanced_entry.compression_ratio() > 1.0);
    }

    #[test]
    fn test_compressed_witness_log() {
        let mut log = CompressedWitnessLog::new(1024 * 1024);

        // Add some witness entries
        for i in 0..5 {
            log.add_witness_entry(
                WitnessOperationType::FileRead,
                1000,
                2000,
                WitnessData::FileOperation {
                    path: format!("/test/file{}", i),
                    offset: i * 100,
                    data: vec![i as u8; 100],
                    result: 100,
                },
                Some(i as u128),
            ).unwrap();
        }

        assert_eq!(log.len(), 5);
        assert!(!log.is_empty());

        // Test event correlation
        let entries = log.get_entries_for_event(2);
        assert_eq!(entries.len(), 1);

        // Test Merkle root computation
        let root = log.compute_merkle_root().unwrap();
        assert_ne!(root, [0u8; 32]);

        // Test compression stats
        let stats = log.compression_stats();
        assert_eq!(stats.total_entries, 5);
        assert!(stats.average_compression_ratio >= 1.0);
    }

    #[test]
    fn test_enhanced_witness_recorder() {
        let config = WitnessRecorderConfig::default();
        let mut recorder = EnhancedWitnessRecorder::new(config);

        recorder.set_enabled(true);
        recorder.set_target_pid(1000);

        // Record some operations
        recorder.record_file_operation(
            WitnessOperationType::FileRead,
            1000,
            2000,
            "/test/file",
            0,
            b"test data",
            9,
        ).unwrap();

        recorder.record_syscall_result(
            1000,
            2000,
            "read",
            &[3, 0x1000, 100],
            9,
            0,
        ).unwrap();

        assert_eq!(recorder.log().len(), 2);

        // Test Merkle root computation
        let root = recorder.compute_merkle_root().unwrap();
        assert_ne!(root, [0u8; 32]);
    }
}
