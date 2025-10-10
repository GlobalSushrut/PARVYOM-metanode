//! Pravyom Standard Pipeline v1.0 - Core Trait Interfaces
//! 
//! These traits define the canonical interfaces for each stage of the pipeline:
//! VM → Ziplock → BPI → BPCI (PoE Economy)

use crate::*;
use serde::{Deserialize, Serialize};

/// Record Emitter - VM layer interface for emitting action records
pub trait RecordEmitter {
    /// Emit an action record to the pipeline
    fn emit(&self, rec: ActionRecord) -> PipelineResult<()>;
    
    /// Get the VM ID for this emitter
    fn vm_id(&self) -> &str;
    
    /// Get the VM type for this emitter
    fn vm_type(&self) -> VmType;
}

/// Ziplock Writer - Interface for writing ziplock segments
pub trait ZiplockWriter {
    /// Append CBOR-encoded record to current segment
    fn append_cbor(&mut self, cbor: &[u8]) -> PipelineResult<()>;
    
    /// Append JSON record (optional mirror)
    fn append_json(&mut self, json: &str) -> PipelineResult<()>;
    
    /// Seal current segment and return metadata
    fn seal_segment(&mut self) -> PipelineResult<SegmentMeta>;
    
    /// Check if segment should be sealed (1000 records or 60s)
    fn should_seal(&self) -> bool;
    
    /// Get current segment statistics
    fn segment_stats(&self) -> SegmentStats;
}

/// Segment Statistics
#[derive(Debug, Clone)]
pub struct SegmentStats {
    pub record_count: u32,
    pub duration_secs: u64,
    pub size_bytes: u64,
}

/// Ticket Summarizer - Interface for creating summary tickets
pub trait TicketSummarizer {
    /// Summarize VM activity over a time window
    fn summarize(&self, win: TimeWindow) -> PipelineResult<SummaryTicket>;
    
    /// Check if summarization should be triggered
    fn should_summarize(&self) -> bool;
    
    /// Get rollup data for specific VM
    fn vm_rollup(&self, vmid: &str, win: TimeWindow) -> PipelineResult<VmRollup>;
    
    /// Get system-wide rollup data
    fn system_rollup(&self, win: TimeWindow) -> PipelineResult<SystemRollup>;
}

/// BPI Client - Interface for interacting with BPI ledger
pub trait BpiClient {
    /// Submit a ziplock ticket to BPI ledger
    fn submit_ticket(&self, ticket: &SummaryTicket) -> PipelineResult<TxId>;
    
    /// Announce a BPI bundle on the ledger
    fn announce_bpi_bundle(&self, bundle: &BpiBundle) -> PipelineResult<TxId>;
    
    /// Get block logbook for a specific height
    fn get_block_logbook(&self, height: u64) -> PipelineResult<BlockLogbook>;
    
    /// Check transaction status
    fn tx_status(&self, tx_id: &TxId) -> PipelineResult<TxStatus>;
}

/// Block Logbook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockLogbook {
    pub block_id: String,
    pub height: u64,
    pub time: DateTime<Utc>,
    pub ticket_count: u32,
    pub tickets_root: String,
    pub ziplock_map: Vec<ZiplockMapEntry>,
    pub poe_exec_in_block: u32,
    pub state_root: String,
    pub court_hooks: String,
}

/// Ziplock Map Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZiplockMapEntry {
    pub vmid: String,
    pub seg: String,
    pub status: String, // "ok|pending"
}

/// Transaction Status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TxStatus {
    Pending,
    Confirmed,
    Failed(String),
}

/// PoE Bundler - Interface for bundling Proof-of-Execution units
pub trait PoeBundler {
    /// Add a PoE unit to the bundle
    fn add(&mut self, poe: PoeUnit);
    
    /// Try to seal bundle (100 PoE or 10min timeout)
    fn maybe_seal(&mut self, now: Instant) -> Option<BpiBundle>;
    
    /// Force seal bundle (for partial bundles)
    fn force_seal(&mut self) -> Option<BpiBundle>;
    
    /// Get current bundle statistics
    fn bundle_stats(&self) -> BundleStats;
}

/// Bundle Statistics
#[derive(Debug, Clone)]
pub struct BundleStats {
    pub poe_count: u32,
    pub age_secs: u64,
    pub ready_to_seal: bool,
}

/// BPCI Auctioneer - Interface for creating auction bundles
pub trait BpciAuctioneer {
    /// Add a BPI bundle to the auction lot
    fn add_bundle(&mut self, bundle: BpiBundle);
    
    /// Try to open auction (100 bundles or 60min timeout)
    fn maybe_open(&mut self, now: Instant) -> Option<BpciAuctionLot>;
    
    /// Force open auction (for partial lots)
    fn force_open(&mut self) -> Option<BpciAuctionLot>;
    
    /// Get current auction statistics
    fn auction_stats(&self) -> AuctionStats;
}

/// Auction Statistics
#[derive(Debug, Clone)]
pub struct AuctionStats {
    pub bundle_count: u32,
    pub age_secs: u64,
    pub ready_to_open: bool,
    pub total_poe: u32,
}

/// CID Storage - Interface for off-chain content storage
pub trait CidStorage {
    /// Store content and return CID
    fn store(&self, content: &[u8]) -> PipelineResult<Cid>;
    
    /// Retrieve content by CID
    fn retrieve(&self, cid: &Cid) -> PipelineResult<Vec<u8>>;
    
    /// Check if CID is available
    fn is_available(&self, cid: &Cid) -> bool;
    
    /// Pin content for long-term storage
    fn pin(&self, cid: &Cid) -> PipelineResult<()>;
}

/// Merkle Tree Builder - Interface for building Merkle trees
pub trait MerkleBuilder {
    /// Add leaf to the tree
    fn add_leaf(&mut self, data: &[u8]);
    
    /// Build tree and return root hash
    fn build(&self) -> PipelineResult<String>;
    
    /// Generate proof for specific leaf
    fn proof(&self, leaf_index: usize) -> PipelineResult<MerkleProof>;
    
    /// Verify proof against root
    fn verify_proof(&self, proof: &MerkleProof, root: &str) -> bool;
}

/// Merkle Proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    pub leaf_index: usize,
    pub leaf_hash: String,
    pub siblings: Vec<String>,
    pub root: String,
}

/// Clock Provider - Interface for time and clock proofs
pub trait ClockProvider {
    /// Get current monotonic time
    fn monotonic_time(&self) -> u64;
    
    /// Get current wall clock time
    fn wall_time(&self) -> DateTime<Utc>;
    
    /// Generate clock proof
    fn clock_proof(&self, vmid: &str, prev_hash: &str) -> PipelineResult<String>;
    
    /// Get Roughtime anchor
    fn roughtime_anchor(&self) -> PipelineResult<TimeAnchor>;
}

/// Signature Provider - Interface for cryptographic signatures
pub trait SignatureProvider {
    /// Sign data with Ed25519
    fn sign_ed25519(&self, data: &[u8]) -> PipelineResult<String>;
    
    /// Sign data with post-quantum crypto
    fn sign_pqc(&self, data: &[u8]) -> PipelineResult<String>;
    
    /// Create BLS aggregate signature
    fn sign_bls_aggregate(&self, data: &[u8], signers: &[String]) -> PipelineResult<String>;
    
    /// Verify signature
    fn verify(&self, data: &[u8], signature: &str, public_key: &str) -> bool;
}

/// Pipeline Coordinator - Main orchestrator interface
pub trait PipelineCoordinator {
    /// Start the pipeline
    fn start(&mut self) -> PipelineResult<()>;
    
    /// Stop the pipeline gracefully
    fn stop(&mut self) -> PipelineResult<()>;
    
    /// Get pipeline status
    fn status(&self) -> PipelineStatus;
    
    /// Get pipeline metrics
    fn metrics(&self) -> PipelineMetrics;
    
    /// Handle VM event
    fn handle_vm_event(&mut self, event: VmEvent) -> PipelineResult<()>;
}

/// Pipeline Status
#[derive(Debug, Clone, PartialEq)]
pub enum PipelineStatus {
    Stopped,
    Starting,
    Running,
    Stopping,
    Error(String),
}

/// Pipeline Metrics
#[derive(Debug, Clone)]
pub struct PipelineMetrics {
    pub records_processed: u64,
    pub segments_sealed: u64,
    pub tickets_submitted: u64,
    pub bundles_created: u64,
    pub auctions_opened: u64,
    pub uptime_secs: u64,
}

/// VM Event
#[derive(Debug, Clone)]
pub enum VmEvent {
    ActionRecord(ActionRecord),
    SegmentSealed(SegmentMeta),
    Error { vmid: String, error: String },
}

/// Configuration Provider - Interface for pipeline configuration
pub trait ConfigProvider {
    /// Get threshold configuration
    fn thresholds(&self) -> ThresholdConfig;
    
    /// Get VM configuration
    fn vm_config(&self, vmid: &str) -> Option<VmConfig>;
    
    /// Get signing configuration
    fn signing_config(&self) -> SigningConfig;
    
    /// Get storage configuration
    fn storage_config(&self) -> StorageConfig;
}

/// Threshold Configuration
#[derive(Debug, Clone)]
pub struct ThresholdConfig {
    pub records_per_segment: u32,
    pub segment_max_duration_secs: u64,
    pub poe_per_bpi_bundle: u32,
    pub bpi_bundles_per_bpci: u32,
    pub poe_bundle_max_age_mins: u64,
    pub bpci_auction_max_age_mins: u64,
    pub anomaly_spike_factor: f64,
}

/// VM Configuration
#[derive(Debug, Clone)]
pub struct VmConfig {
    pub vm_type: VmType,
    pub image: String,
    pub description: String,
}

/// Signing Configuration
#[derive(Debug, Clone)]
pub struct SigningConfig {
    pub ed25519_enabled: bool,
    pub pqc_algorithm: String,
    pub bls_enabled: bool,
    pub pqc_multi_enabled: bool,
}

/// Storage Configuration
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub ziplock_path: String,
    pub cid_backends: Vec<String>,
    pub redundancy: u8,
    pub preimage_voucher: bool,
}
