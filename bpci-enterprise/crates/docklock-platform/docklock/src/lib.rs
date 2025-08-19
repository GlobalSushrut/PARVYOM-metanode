//! # BPI DockLock - Determinism Cage
//!
//! Provides deterministic execution environment with syscall filtering,
//! RNG seed injection, and I/O witness recording for reproducible computation.
//!
//! ## Core Components
//!
//! - `DeterminismCage`: Main execution environment with seccomp filtering
//! - `SyscallFilter`: seccomp-based syscall policy enforcement
//! - `WitnessRecorder`: I/O and non-deterministic syscall result recording
//! - `RngSeeder`: Deterministic RNG seed injection and management
//!
//! ## Security Features
//!
//! - Blocks non-deterministic syscalls (gettimeofday, rdtsc, getrandom)
//! - Records all I/O operations for replay verification
//! - Injects deterministic RNG seeds for reproducible randomness
//! - Merkle-izes witness logs for cryptographic verification

pub mod cage;
pub mod error;
pub mod filter;
pub mod seeder;
pub mod witness;
pub mod witness_enhanced;
pub mod witness_wallet_integration;
pub mod witness_cage_integration;

// Stage 28: Receipt Structure & Signing
pub mod receipt;

// Phase 1: StepReceipt for v1.0 blockchain pipeline
pub mod step_receipt;
pub mod operation_monitor;

// Stage 11.2: Container Deployment API
pub mod container_api;
pub mod native_execution;
pub mod receipt_generator;
pub mod receipt_validation;

// Stage 35: DA Pinner Service (BISO Policy Engine)
pub mod biso_policy;

// Stage 36: DA Sampler (Bus BIOS Implementation)
pub mod bus_bios;

// Stage 29: Policy Engine (WASM) and Court container
pub mod policy_engine;
pub mod court;
pub mod agreements_sdk;

// Stage 30: ZK Claim Hooks (SNARK verification, ZK module)
pub mod zk_proofs;

// PoE Notarization Layer: StepReceipt Integration
pub mod step_receipt_integration;
pub mod zk_policy_integration;

// Stage 31: Receipt Registry Facade
pub mod receipt_registry;
pub mod registry_api;

// Stage 31.5: ENC Cluster - Revolutionary Blockchain Orchestration
pub mod enc_cluster;

// Stage 32: Shadow Receipt & Practical Postbox
pub mod shadow_receipt;

// Stage 33: RS Encoding at Edge (Traffic Light Pipeline Foundation)
pub mod traffic_light;

// Stage 7: Traffic Light & BISO Integration
pub mod traffic_light_integration;

pub mod bpi_wallet_registry;
pub use bpi_wallet_registry::*;

pub mod enhanced_storage_db;

#[cfg(test)]
pub mod bpi_integration_tests;

// Stage 34: Packet Envelope Structure (Shard Headers & da_root)
pub mod packet_envelope;

// Stage 40: DA Observability (Traffic Light Dashboard)
pub mod traffic_light_dashboard;

// Stage 41: Inclusion Lists (Consensus Rule)
pub mod inclusion_lists;
pub mod force_inclusion_inbox;

// Stage 37: DA Challenge & Slashing (Blockbook Ledger)
pub mod blockbook;

// Stage 38: Multi-Cloud Storage Policy (Audit Book Export)
pub mod audit_book;

// Stage 39: CAR/DAG Packaging (Envelope Optimization)
pub mod car_dag_packaging;

// Stage 26: Enhanced Canonical Event Stream + MetaNode Wallet System
pub mod event_stream;
pub mod wallet;
pub mod dao_wallet;
pub mod metanode_wallet;

pub use cage::*;
pub use filter::*;
pub use witness::*;
pub use seeder::*;
pub use error::*;

// Stage 26 exports
pub use event_stream::*;
pub use wallet::*;
pub use dao_wallet::*;

// Stage 7 exports
pub use traffic_light_integration::*;
// Export metanode_wallet types except Jurisdiction to avoid conflict
pub use metanode_wallet::{
    MetaNodeWallet, BoxedIdentity, VerificationLevel, WalletBoxAgreement,
    MetaNodeWalletStats, MetaMaskIntegration, MetaNodeWalletConfig
};

// Stage 37 exports
pub use blockbook::*;

// Stage 38 exports
// Export audit_book types with explicit naming to avoid conflicts
pub use audit_book::{
    AuditBook, AuditBookEntry, RegulatoryFramework, 
    Jurisdiction as AuditJurisdiction, CloudProvider, AccessLevel,
    ExportFormat, EncryptionScheme as AuditEncryptionScheme,
    MultiCloudStorageConfig, ExportConfig, AuditBookStats
};

// Stage 39 exports
pub use car_dag_packaging::*;

use bpi_enc::{domain_hash, CanonicalCbor};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;


/// Domain tag for DockLock record hashing (0x13)
pub const DOCKLOCK_RECORD_HASH: &str = "BPI_DOCKLOCK_RECORD_HASH";

/// Configuration for determinism cage execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CageConfig {
    /// Whether to enable seccomp syscall filtering
    pub enable_seccomp: bool,
    /// Whether to record I/O operations for witness generation
    pub enable_witness_recording: bool,
    /// Maximum size of witness log in bytes
    pub max_witness_size: usize,
    /// RNG seed for deterministic randomness
    pub rng_seed: [u8; 32],
    /// Allowed syscalls that bypass the filter
    pub allowed_syscalls: Vec<String>,
    /// Environment variables to inject
    pub env_vars: HashMap<String, String>,
}

impl Default for CageConfig {
    fn default() -> Self {
        Self {
            enable_seccomp: true,
            enable_witness_recording: true,
            max_witness_size: 1024 * 1024, // 1MB
            rng_seed: [0u8; 32],
            allowed_syscalls: vec![
                "read".to_string(),
                "write".to_string(),
                "mmap".to_string(),
                "munmap".to_string(),
                "brk".to_string(),
                "exit_group".to_string(),
            ],
            env_vars: HashMap::new(),
        }
    }
}

/// Execution result from determinism cage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Exit code of the executed process
    pub exit_code: i32,
    /// Standard output captured
    pub stdout: Vec<u8>,
    /// Standard error captured
    pub stderr: Vec<u8>,
    /// Witness log of I/O operations
    pub witness_log: WitnessLog,
    /// Execution duration in nanoseconds
    pub duration_ns: u64,
    /// Whether execution was deterministic
    pub is_deterministic: bool,
}

impl ExecutionResult {
    /// Compute hash of execution result for verification
    pub fn compute_hash(&self) -> anyhow::Result<[u8; 32]> {
        let encoded = CanonicalCbor::encode(self)?;
        Ok(domain_hash(DOCKLOCK_RECORD_HASH, &encoded))
    }
    
    /// Check if execution result matches expected hash
    pub fn verify_hash(&self, expected_hash: &[u8; 32]) -> anyhow::Result<bool> {
        let computed_hash = self.compute_hash()?;
        Ok(computed_hash == *expected_hash)
    }
}


