//! ZIPLOCK-JSON (ZJL) - Immutable audit artifact system for Metanode VM
//! 
//! This crate implements the ZIPLOCK-JSON file format for unified, immutable,
//! forensically-analyzable audit trails across all BPI VMs.
//!
//! ## Features
//! - Write-once, non-executable, override-delete only
//! - Compressed with Zstd, optionally encrypted with ChaCha20-Poly1305
//! - 10-year/1-TB retention via Merkle rollups
//! - Black-box forensics with BREV-64 encoding
//! - BPI ledger anchoring for tamper-proof audit trails
//! - I-JSON enforcement for security

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write, Seek};
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub mod header;
pub mod blocks;
pub mod json_encoder;
pub mod merkle;
pub mod brev64;
pub mod signing;
pub mod central_dir;
pub mod writer;
pub mod reader;
pub mod vm_integration;
pub mod vm_integrity;
pub mod bundle_transaction;
pub mod nakamoto_monitor;
pub mod geographic_distribution_enforcer;
pub mod anti_manipulation_engine;
pub mod validator_rotation_coordinator;
pub mod bpci_bundle_auction;
pub mod system_audit_coordinator;
pub mod bpi_master_audit;
pub mod minute_root_anchoring;
pub mod zk3_attestation_circuits;
pub mod gov_index_aggregation;

pub use header::*;
pub use blocks::*;
pub use writer::*;
pub use json_encoder::*;
pub use merkle::*;
pub use brev64::*;
pub use signing::*;

/// ZIPLOCK-JSON file magic signature
pub const ZJL_MAGIC: [u8; 4] = *b"ZJLK";

/// Current ZJL format version
pub const ZJL_VERSION: u16 = 0x0001;

/// Configuration options for ZJL file creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZjlOptions {
    pub compression_level: u32,
    pub enable_encryption: bool,
    pub max_file_size: u64,
    pub chunk_size: usize,
    pub enable_signatures: bool,
    pub kms_endpoint: Option<String>,
    pub enable_forensic_mode: bool,
    pub enable_merkle_proofs: bool,
    pub retention_days: u32,
    pub enforce_i_json: bool,
    pub enable_rollups: bool,
    pub enable_brev64: bool,
}

impl Default for ZjlOptions {
    fn default() -> Self {
        Self {
            compression_level: 6,
            enable_encryption: false,
            max_file_size: 1024 * 1024 * 1024, // 1GB
            chunk_size: 64 * 1024, // 64KB chunks
            enable_signatures: true,
            kms_endpoint: None,
            enable_forensic_mode: true,
            enable_merkle_proofs: true,
            retention_days: 3650, // 10 years
            enforce_i_json: true,
            enable_rollups: true,
            enable_brev64: true,
        }
    }
}

/// Default compression level for Zstd
pub const DEFAULT_ZSTD_LEVEL: i32 = 6;

/// Default shard size (128 MB)
pub const DEFAULT_SHARD_SIZE: u64 = 128 * 1024 * 1024;

/// Default hot window (90 days)
pub const DEFAULT_HOT_WINDOW_DAYS: u64 = 90;

/// MIME type for ZJL files
pub const ZJL_MIME_TYPE: &str = "application/vnd.metanode.zjlock";

/// File extension for ZJL files
pub const ZJL_EXTENSION: &str = ".zjlock";

/// ZJL file creation options
#[derive(Debug, Clone)]
pub struct CreateOpts {
    /// Enable AEAD encryption
    pub encrypt: bool,
    /// Enforce I-JSON compliance
    pub enforce_i_json: bool,
    /// Zstd compression level (1-22, default 6)
    pub zstd_level: i32,
    /// Optional compression dictionary (4-8 KB trained per quarter)
    pub dict: Option<Vec<u8>>,
    /// Maximum shard size before sealing
    pub max_shard_size: u64,
    /// Hot window duration in days
    pub hot_window_days: u64,
}

impl Default for CreateOpts {
    fn default() -> Self {
        Self {
            encrypt: true,
            enforce_i_json: true,
            zstd_level: DEFAULT_ZSTD_LEVEL,
            dict: None,
            max_shard_size: DEFAULT_SHARD_SIZE,
            hot_window_days: DEFAULT_HOT_WINDOW_DAYS,
        }
    }
}

/// Micro-receipt for efficient event tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptMicro {
    /// Time delta from previous event (varint encoded)
    pub ts_delta: u32,
    /// Actor ID (interned, varint encoded)
    pub actor: u32,
    /// Event type code
    pub event_type: u16,
    /// Truncated event hash (8-16 bytes)
    pub hash16: [u8; 16],
}

/// BREV-64 forensic reason with evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reason {
    /// 64-bit Binary Reason Vector
    pub brev: u64,
    /// Timestamp in nanoseconds
    pub ts_ns: u64,
    /// Process information
    pub proc: ProcessInfo,
    /// Source hash (5-tuple or syscall ID)
    pub src_hash16: [u8; 16],
    /// Evidence hashes (BLAKE3 of snapshots/pcap/files)
    pub evidence_hashes: Vec<[u8; 32]>,
    /// Reference to minute root (filled on seal)
    pub minute_root_ref: Option<[u8; 32]>,
}

/// Process information for forensics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub uid: u32,
    pub gid: u32,
    pub cgroup_id: u64,
    pub image_sha256: [u8; 32],
}

/// Forensic snapshot of system state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// Timestamp
    pub ts_ns: u64,
    /// Process tree snapshot
    pub proc_tree: Vec<ProcessInfo>,
    /// Memory map segments (executable only)
    pub mmap_exec: Vec<MemMapSegment>,
    /// Container digest
    pub container_digest: Option<[u8; 32]>,
    /// SELinux/LSM context
    pub security_context: String,
    /// Last 3 network flows metadata
    pub network_flows: Vec<NetworkFlow>,
    /// File integrity monitoring deltas
    pub fim_deltas: Vec<FimDelta>,
    /// Kernel oops/panic ID
    pub kernel_oops_id: Option<u64>,
}

/// Memory map segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemMapSegment {
    pub start_addr: u64,
    pub end_addr: u64,
    pub permissions: String,
    pub path: String,
    pub hash: [u8; 32],
}

/// Network flow metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkFlow {
    pub src_ip: std::net::IpAddr,
    pub dst_ip: std::net::IpAddr,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: u8,
    pub bytes_tx: u64,
    pub bytes_rx: u64,
    pub ts_start: u64,
    pub ts_end: u64,
}

/// File integrity monitoring delta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FimDelta {
    pub path: String,
    pub old_hash: Option<[u8; 32]>,
    pub new_hash: [u8; 32],
    pub operation: String, // "create", "modify", "delete"
    pub timestamp: u64,
}

/// Merkle proof for inclusion verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    /// Leaf hash
    pub leaf: [u8; 32],
    /// Merkle path (sibling hashes)
    pub path: Vec<[u8; 32]>,
    /// Root hash
    pub root: [u8; 32],
    /// Leaf index
    pub index: u64,
}

/// Block index entry for scanning
#[derive(Debug, Clone)]
pub struct BlockIndex {
    /// Block offset in file
    pub offset: u64,
    /// Block type
    pub block_type: u8,
    /// Path ID (if applicable)
    pub path_id: u64,
    /// Block hash
    pub hash: [u8; 32],
    /// Compressed length
    pub compressed_len: u32,
    /// Uncompressed length
    pub uncompressed_len: u32,
}

/// Error types for ZJL operations
#[derive(Debug, thiserror::Error)]
pub enum ZjlError {
    #[error("Invalid magic signature")]
    InvalidMagic,
    #[error("Unsupported version: {0}")]
    UnsupportedVersion(u16),
    #[error("File is sealed and cannot be modified")]
    FileSealed,
    #[error("Key has been revoked: {0}")]
    KeyRevoked(String),
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    #[error("Invalid I-JSON: {0}")]
    InvalidIJson(String),
    #[error("Invalid JSON: {0}")]
    InvalidJson(String),
    #[error("Block verification failed")]
    BlockVerificationFailed,
    #[error("Merkle proof verification failed")]
    MerkleProofFailed,
    #[error("Signature verification failed")]
    SignatureVerificationFailed,
    #[error("Encryption/decryption failed")]
    CryptoError,
    #[error("IO error: {0}")]
    IoError(String),
    #[error("IO error: {0}")]
    IoErrorStd(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Serialization error: {0}")]
    SerializationErrorString(String),
    #[error("Compression error: {0}")]
    CompressionError(String),
    #[error("Not implemented: {0}")]
    NotImplemented(String),
    #[error("File is already sealed")]
    AlreadySealed,
    #[error("Not sealed")]
    NotSealed,
    #[error("Invalid Merkle tree: {0}")]
    InvalidMerkleTree(String),
    #[error("Decoding error: {0}")]
    DecodingError(String),
    #[error("Invalid block type: {0}")]
    InvalidBlockType(u8),
    #[error("Signing error: {0}")]
    SigningError(String),
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Missing key ID")]
    MissingKeyId,
    #[error("Invalid key ID")]
    InvalidKeyId,
    #[error("Missing metadata")]
    MissingMetadata,
    #[error("Invalid offset: {0}")]
    InvalidOffset(u64),
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

/// Result type for ZJL operations
pub type ZjlResult<T> = Result<T, ZjlError>;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_create_opts_default() {
        let opts = CreateOpts::default();
        assert!(opts.encrypt);
        assert!(opts.enforce_i_json);
        assert_eq!(opts.zstd_level, DEFAULT_ZSTD_LEVEL);
        assert_eq!(opts.max_shard_size, DEFAULT_SHARD_SIZE);
        assert_eq!(opts.hot_window_days, DEFAULT_HOT_WINDOW_DAYS);
    }

    #[test]
    fn test_receipt_micro_serialization() {
        let receipt = ReceiptMicro {
            ts_delta: 1000,
            actor: 42,
            event_type: 1,
            hash16: [0u8; 16],
        };

        let serialized = serde_json::to_string(&receipt).unwrap();
        let deserialized: ReceiptMicro = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(receipt.ts_delta, deserialized.ts_delta);
        assert_eq!(receipt.actor, deserialized.actor);
        assert_eq!(receipt.event_type, deserialized.event_type);
        assert_eq!(receipt.hash16, deserialized.hash16);
    }

    #[test]
    #[ignore] // TODO: Implement brev module
    fn test_brev64_encoding() {
        // Test BREV-64 encoding: INFO class, HTTP vector, ABUSE primitive, USER priv
        // let brev = brev::encode_brev64(
        //     brev::BrevClass::Info,
        //     brev::BrevVector::Http,
        //     brev::BrevPrimitive::Abuse,
        //     brev::BrevPrivLevel::User,
        //     42, // scope
        //     brev::BrevImpact::Medium,
        //     0x1234, // subcode
        //     0x5678, // vendor
        // );

        // let decoded = brev::decode_brev64(brev);
        // assert_eq!(decoded.class, brev::BrevClass::Info);
        // assert_eq!(decoded.vector, brev::BrevVector::Http);
        // assert_eq!(decoded.primitive, brev::BrevPrimitive::Abuse);
        // assert_eq!(decoded.priv_level, brev::BrevPrivLevel::User);
        // assert_eq!(decoded.scope, 42);
        // assert_eq!(decoded.impact, brev::BrevImpact::Medium);
        // assert_eq!(decoded.subcode, 0x1234);
        // assert_eq!(decoded.vendor, 0x5678);
    }
}
