//! Pravyom Standard Pipeline v1.0 - Core Traits and Types
//! 
//! This crate provides the canonical trait interfaces and type definitions
//! for the Pravyom Standard Pipeline: VM → Ziplock → BPI → BPCI (PoE Economy)

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub mod types;
pub mod traits;
pub mod helpers;

pub use types::*;
pub use traits::*;
pub use helpers::*;

/// Core pipeline result type
pub type PipelineResult<T> = Result<T>;

/// Transaction ID type
pub type TxId = String;

/// Content ID type for off-chain storage
pub type Cid = String;

/// Canonical VM types as defined in Pravyom Standard Pipeline v1.0
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VmType {
    #[serde(rename = "VM-APP")]
    App,
    #[serde(rename = "VM-ORCH")]
    Orch,
    #[serde(rename = "VM-CLUSTER")]
    Cluster,
    #[serde(rename = "VM-STORAGE")]
    Storage,
    #[serde(rename = "VM-FIREWALL")]
    Firewall,
    #[serde(rename = "VM-COURT")]
    Court,
    #[serde(rename = "VM-BISO")]
    Biso,
    #[serde(rename = "VM-TRAFFICLIGHT")]
    TrafficLight,
}

/// Action Record - Canonical format for VM actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRecord {
    /// Record ID: R-{YYYYMMDD}-{vmid}-{nonce16}
    pub rid: String,
    /// VM information
    pub vm: VmInfo,
    /// Actor information
    pub actor: ActorInfo,
    /// Action details
    pub action: ActionInfo,
    /// Result of the action
    pub result: ActionResult,
    /// Resource usage
    pub resource: ResourceUsage,
    /// Network information (optional)
    pub net: Option<NetworkInfo>,
    /// Geographic information (optional)
    pub geo: Option<GeoInfo>,
    /// Time information
    pub time: TimeInfo,
    /// Time anchor (Roughtime proof)
    pub time_anchor: TimeAnchor,
    /// Hash chain
    pub hash: HashChain,
    /// Signatures
    pub sig: RecordSignature,
    /// Execution determinism (for PoE-eligible actions)
    pub exec: Option<ExecInfo>,
}

/// VM Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub vm_type: VmType,
    pub image: String,
}

/// Actor Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorInfo {
    pub wallet: String,
    pub role: String, // "client|service|admin"
}

/// Action Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionInfo {
    #[serde(rename = "type")]
    pub action_type: String, // "WRITE|READ|EXEC|NET|POLICY"
    pub name: String,
    pub args: serde_json::Value,
}

/// Action Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub code: i32,
    pub latency_ms: f64,
    pub bytes_out: u64,
}

/// Resource Usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_ms: f64,
    pub ram_kb: u64,
    pub io: IoUsage,
}

/// I/O Usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoUsage {
    pub r: u64, // bytes read
    pub w: u64, // bytes written
}

/// Network Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub src_ip: String,
    pub dst_ip: String,
    pub port: u16,
}

/// Geographic Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoInfo {
    pub client_did: String,
    pub coarse: String,
}

/// Time Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeInfo {
    pub ts_wall: DateTime<Utc>,
    pub ts_mono: u64, // monotonic nanoseconds from boot
}

/// Time Anchor (Roughtime proof)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeAnchor {
    pub rt: String, // "draft-roughtime@v1"
    pub server: String,
    pub proof: String, // base64 encoded proof
}

/// Hash Chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashChain {
    pub prev: String,
    #[serde(rename = "self")]
    pub self_hash: String,
}

/// Record Signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordSignature {
    pub ed25519: String,
    pub pqc: String, // dilithium2
}

/// Execution Information (for PoE)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecInfo {
    pub model: String, // "WASM|SGX|DL"
    pub verdict: String, // "DET|QUASI|NONDET"
    pub seed: String, // blake3 hash
}

/// Segment Metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentMeta {
    pub vmid: String,
    pub segment_seq: u64,
    pub start_ts: DateTime<Utc>,
    pub prev_segment_root: String,
    pub seg_merkle_root: String,
    pub seg_resource_totals: ResourceTotals,
    pub receipt_self: String,
    pub time_anchor: TimeAnchor,
    pub sig: AggregateSignature,
}

/// Resource Totals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceTotals {
    pub cpu_ms: f64,
    pub ram_kb: u64,
    pub io: IoUsage,
}

/// Aggregate Signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateSignature {
    pub bls: String,
    pub pqc_multi: Vec<String>,
}

/// Time Window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

/// Summary Ticket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryTicket {
    pub ticket_id: String,
    pub window: TimeWindow,
    pub policy: TicketPolicy,
    pub vm_rollup: Vec<VmRollup>,
    pub system: SystemRollup,
    pub roots: TicketRoots,
    pub anchors: TicketAnchors,
    pub sig: AggregateSignature,
}

/// Ticket Policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketPolicy {
    pub threshold: String, // "1min_or_1000rec"
    pub vm_count: u8,
}

/// VM Rollup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmRollup {
    pub vmid: String,
    pub records: u64,
    pub cpu_ms: f64,
    pub ram_kb: u64,
    pub io: IoUsage,
    pub net: NetworkRollup,
    pub seg: SegmentRef,
}

/// Network Rollup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRollup {
    pub flows: u32,
}

/// Segment Reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentRef {
    pub id: String,
    pub root: String,
}

/// System Rollup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemRollup {
    pub totals: SystemTotals,
    pub sec: SecurityRollup,
    pub poe: PoeRollup,
    pub anomaly: Option<AnomalyRollup>,
}

/// System Totals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemTotals {
    pub records: u64,
    pub cpu_ms: f64,
    pub ram_kb_avg: u64,
}

/// Security Rollup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRollup {
    pub allow: u32,
    pub deny: u32,
    pub qlock_events: u32,
}

/// PoE Rollup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoeRollup {
    pub exec_count: u32,
    pub ready_for_poe_bundle: bool,
}

/// Anomaly Rollup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyRollup {
    pub spikes: Vec<AnomalySpike>,
}

/// Anomaly Spike
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalySpike {
    pub vmid: String,
    pub factor: f64,
}

/// Ticket Roots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketRoots {
    pub vm_merkle: String,
    pub ziplock_super_root: String,
}

/// Ticket Anchors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketAnchors {
    pub previous_ticket: String,
    pub bpi_tip_hint: String,
}

/// PoE Unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoeUnit {
    pub poe_id: String,
    pub origin: PoeOrigin,
    pub exec: ExecInfo,
    pub witness: PoeWitness,
    pub charge: ResourceUsage,
    pub sig: RecordSignature,
}

/// PoE Origin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoeOrigin {
    pub vmid: String,
    pub rid: String,
}

/// PoE Witness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoeWitness {
    pub inputs: Cid,
    pub outputs: Cid,
}

/// BPI Bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpiBundle {
    pub bpi_bundle_id: String,
    pub count: u32,
    pub poe_root: String,
    pub ticket_refs: Vec<String>,
    pub bpi_block_ref: String,
    pub treasury_split: TreasurySplit,
    pub sig: AggregateSignature,
}

/// Treasury Split
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasurySplit {
    pub miner: String,
    pub community: String,
}

/// BPCI Auction Lot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpciAuctionLot {
    pub bpci_auction_id: String,
    pub bpi_bundles: u32,
    pub bpi_bundle_root: String,
    pub market_meta: MarketMeta,
    pub accounting: AuctionAccounting,
    pub ziplock_anchor: String,
    pub sig: AggregateSignature,
}

/// Market Metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketMeta {
    pub class: String, // "PoE_EXECUTION"
    pub min_partner_stake: String,
    pub reserve_price: String,
    pub sla: ServiceLevelAgreement,
}

/// Service Level Agreement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceLevelAgreement {
    pub retrievability: String, // ">=99.99%"
    pub latency_ms_p95: u32,
}

/// Auction Accounting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionAccounting {
    pub poe_total: u32,
    pub window_from: DateTime<Utc>,
    pub window_to: DateTime<Utc>,
}

/// Pipeline configuration constants
pub mod constants {
    pub const RECORDS_PER_SEGMENT: u32 = 1000;
    pub const SEGMENT_MAX_DURATION_SECS: u64 = 60;
    pub const POE_PER_BPI_BUNDLE: u32 = 100;
    pub const BPI_BUNDLES_PER_BPCI: u32 = 100;
    pub const POE_BUNDLE_MAX_AGE_MINS: u64 = 10;
    pub const BPCI_AUCTION_MAX_AGE_MINS: u64 = 60;
    pub const CLOCK_SKEW_TOLERANCE_SECS: u64 = 3;
    pub const ANOMALY_SPIKE_FACTOR: f64 = 10.0;
}
