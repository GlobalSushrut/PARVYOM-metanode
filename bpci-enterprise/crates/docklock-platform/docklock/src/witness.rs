//! I/O witness recording and Merkle-ization for deterministic execution

use crate::error::{DockLockError, DockLockResult};
use bpi_enc::{domain_hash, CanonicalCbor};
use bpi_merkle::{MerkleTree, MerkleProof};
use serde::{Deserialize, Serialize};

use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;

/// Domain tag for witness entry hashing
pub const WITNESS_ENTRY_HASH: &str = "WITNESS_ENTRY";

/// Types of operations that can be witnessed
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WitnessOperationType {
    /// File read operation
    FileRead,
    /// File write operation
    FileWrite,
    /// Network I/O operation
    NetworkIo,
    /// System call result
    SyscallResult,
    /// Environment variable access
    EnvAccess,
    /// Random number generation
    RandomGeneration,
}

/// A single witnessed operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessEntry {
    /// Sequence number for ordering
    pub sequence: u64,
    /// Logical timestamp
    pub timestamp_ns: u64,
    /// Type of operation
    pub operation_type: WitnessOperationType,
    /// Process ID that performed the operation
    pub pid: u32,
    /// Thread ID that performed the operation
    pub tid: u32,
    /// Operation-specific data
    pub data: WitnessData,
    /// Hash of the entry for integrity
    pub entry_hash: [u8; 32],
}

/// Data associated with a witnessed operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WitnessData {
    /// File operation data
    FileOperation {
        path: String,
        offset: u64,
        data: Vec<u8>,
        result: i64,
    },
    /// System call result data
    SyscallResult {
        syscall_name: String,
        args: Vec<u64>,
        result: i64,
        errno: i32,
    },
    /// Environment variable access
    EnvAccess {
        var_name: String,
        value: Option<String>,
    },
    /// Random data generation
    RandomData {
        data: Vec<u8>,
    },
    /// Random number generation
    RandomGeneration {
        seed: [u8; 32],
        output: Vec<u8>,
    },
}

impl WitnessEntry {
    /// Create a new witness entry
    pub fn new(
        sequence: u64,
        operation_type: WitnessOperationType,
        pid: u32,
        tid: u32,
        data: WitnessData,
    ) -> DockLockResult<Self> {
        let timestamp_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;

        let mut entry = Self {
            sequence,
            timestamp_ns,
            operation_type,
            pid,
            tid,
            data,
            entry_hash: [0u8; 32],
        };

        // Compute hash of the entry
        entry.entry_hash = entry.compute_hash()?;
        Ok(entry)
    }

    /// Compute hash of the witness entry
    pub fn compute_hash(&self) -> DockLockResult<[u8; 32]> {
        // Create a copy without the hash field for hashing
        let mut entry_for_hash = self.clone();
        entry_for_hash.entry_hash = [0u8; 32];

        let encoded = CanonicalCbor::encode(&entry_for_hash)?;
        Ok(domain_hash(WITNESS_ENTRY_HASH, &encoded))
    }

    /// Verify the integrity of the witness entry
    pub fn verify_integrity(&self) -> DockLockResult<bool> {
        let computed_hash = self.compute_hash()?;
        Ok(computed_hash == self.entry_hash)
    }
}

/// Log of witnessed operations with Merkle tree support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessLog {
    /// List of witness entries in sequence order
    pub entries: Vec<WitnessEntry>,
    /// Merkle root of all entries
    pub merkle_root: Option<[u8; 32]>,
    /// Maximum size limit in bytes
    pub max_size: usize,
    /// Current size in bytes
    pub current_size: usize,
    /// Next sequence number
    pub next_sequence: u64,
}

impl WitnessLog {
    /// Create a new empty witness log
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: Vec::new(),
            merkle_root: None,
            max_size,
            current_size: 0,
            next_sequence: 0,
        }
    }

    /// Add a new witness entry to the log
    pub fn add_entry(
        &mut self,
        operation_type: WitnessOperationType,
        pid: u32,
        tid: u32,
        data: WitnessData,
    ) -> DockLockResult<()> {
        let entry = WitnessEntry::new(self.next_sequence, operation_type.clone(), pid, tid, data)?;

        // Check size limit
        let entry_size = bincode::serialize(&entry)?.len();
        if self.current_size + entry_size > self.max_size {
            return Err(DockLockError::WitnessLogTooLarge {
                size: self.current_size + entry_size,
            });
        }

        self.entries.push(entry);
        self.current_size += entry_size;
        self.next_sequence += 1;

        debug!(
            "Added witness entry {} (type: {:?}, size: {})",
            self.next_sequence - 1,
            operation_type,
            entry_size
        );

        Ok(())
    }

    /// Compute Merkle root of all witness entries
    pub fn compute_merkle_root(&mut self) -> DockLockResult<[u8; 32]> {
        if self.entries.is_empty() {
            let empty_root = [0u8; 32];
            self.merkle_root = Some(empty_root);
            return Ok(empty_root);
        }

        // Collect entry hashes as Vec<u8>
        let hashes: Vec<Vec<u8>> = self
            .entries
            .iter()
            .map(|entry| entry.entry_hash.to_vec())
            .collect();

        // Build Merkle tree
        let tree = MerkleTree::new(hashes)?;
        let root_bytes = tree.root()?;
        let mut root = [0u8; 32];
        if root_bytes.len() >= 32 {
            root.copy_from_slice(&root_bytes[..32]);
        } else {
            root[..root_bytes.len()].copy_from_slice(&root_bytes);
        }

        self.merkle_root = Some(root);
        Ok(root)
    }

    /// Get Merkle proof for a specific entry
    pub fn get_merkle_proof(&self, index: usize) -> DockLockResult<MerkleProof> {
        if index >= self.entries.len() {
            return Err(DockLockError::WitnessError(format!(
                "Entry index {} out of bounds (max: {})",
                index,
                self.entries.len()
            )));
        }

        let hashes: Vec<Vec<u8>> = self
            .entries
            .iter()
            .map(|entry| entry.entry_hash.to_vec())
            .collect();

        let tree = MerkleTree::new(hashes)?;
        let proof = tree.proof(index)?;
        Ok(proof)
    }

    /// Verify a Merkle proof for an entry
    pub fn verify_merkle_proof(
        &self,
        index: usize,
        proof: &MerkleProof,
    ) -> DockLockResult<bool> {
        if index >= self.entries.len() {
            return Ok(false);
        }

        let _entry_hash = self.entries[index].entry_hash;
        let _root = self.merkle_root.ok_or_else(|| {
            DockLockError::WitnessError("Merkle root not computed".to_string())
        })?;

        let root_bytes = self.merkle_root.ok_or_else(|| {
            DockLockError::WitnessError("Merkle root not computed".to_string())
        })?;
        Ok(proof.verify(root_bytes))
    }

    /// Get the number of entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the log is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get current size in bytes
    pub fn size(&self) -> usize {
        self.current_size
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.merkle_root = None;
        self.current_size = 0;
        self.next_sequence = 0;
    }
}

/// Witness recorder for capturing I/O operations
#[derive(Debug)]
pub struct WitnessRecorder {
    /// The witness log
    log: WitnessLog,
    /// Whether recording is enabled
    enabled: bool,
    /// Process ID being monitored
    target_pid: Option<u32>,
}

impl WitnessRecorder {
    /// Create a new witness recorder
    pub fn new(max_size: usize) -> Self {
        Self {
            log: WitnessLog::new(max_size),
            enabled: true,
            target_pid: None,
        }
    }

    /// Enable or disable recording
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        debug!("Witness recording {}", if enabled { "enabled" } else { "disabled" });
    }

    /// Set target process ID to monitor
    pub fn set_target_pid(&mut self, pid: u32) {
        self.target_pid = Some(pid);
        debug!("Set target PID for witness recording: {}", pid);
    }

    /// Record a file operation
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

        self.log.add_entry(operation_type, pid, tid, witness_data)
    }

    /// Record a syscall result
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

        self.log.add_entry(WitnessOperationType::SyscallResult, pid, tid, witness_data)
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
            value: value.map(|s| s.to_string()),
        };

        self.log.add_entry(WitnessOperationType::EnvAccess, pid, tid, witness_data)
    }

    /// Get the witness log
    pub fn log(&self) -> &WitnessLog {
        &self.log
    }

    /// Get mutable access to the witness log
    pub fn log_mut(&mut self) -> &mut WitnessLog {
        &mut self.log
    }

    /// Check if we should record for this PID
    fn should_record(&self, pid: u32) -> bool {
        self.enabled && (self.target_pid.is_none() || self.target_pid == Some(pid))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_witness_entry_creation() {
        let data = WitnessData::FileOperation {
            path: "/test/file".to_string(),
            offset: 0,
            data: b"test data".to_vec(),
            result: 9,
        };

        let entry = WitnessEntry::new(0, WitnessOperationType::FileRead, 1234, 5678, data);
        assert!(entry.is_ok());

        let entry = entry.unwrap();
        assert_eq!(entry.sequence, 0);
        assert_eq!(entry.pid, 1234);
        assert_eq!(entry.tid, 5678);
        assert!(entry.verify_integrity().unwrap());
    }

    #[test]
    fn test_witness_log_operations() {
        let mut log = WitnessLog::new(1024 * 1024);
        assert!(log.is_empty());

        let data = WitnessData::FileOperation {
            path: "/test/file".to_string(),
            offset: 0,
            data: b"test data".to_vec(),
            result: 9,
        };

        let result = log.add_entry(WitnessOperationType::FileRead, 1234, 5678, data);
        assert!(result.is_ok());
        assert_eq!(log.len(), 1);
        assert!(!log.is_empty());

        let root = log.compute_merkle_root();
        assert!(root.is_ok());
        assert!(log.merkle_root.is_some());
    }

    #[test]
    fn test_witness_recorder() {
        let mut recorder = WitnessRecorder::new(1024 * 1024);
        recorder.set_target_pid(1234);

        let result = recorder.record_file_operation(
            WitnessOperationType::FileRead,
            1234,
            5678,
            "/test/file",
            0,
            b"test data",
            9,
        );
        assert!(result.is_ok());
        assert_eq!(recorder.log().len(), 1);

        // Should not record for different PID
        let result = recorder.record_file_operation(
            WitnessOperationType::FileRead,
            9999,
            5678,
            "/test/file",
            0,
            b"test data",
            9,
        );
        assert!(result.is_ok());
        assert_eq!(recorder.log().len(), 1); // Still 1, not recorded
    }
}
