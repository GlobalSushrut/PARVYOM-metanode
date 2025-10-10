//! Ziplock Human Bundle v2 Implementation
//! 
//! This module implements the comprehensive Ziplock Human Bundle v2 format that preserves
//! end-to-end client↔server causality, deep security traces, and complete VM activity
//! reconstruction for audit purposes.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Main Ziplock Human Bundle v2 structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZiplockHumanBundleV2 {
    pub ziplock_bundle_v2: BundleContent,
}

/// Bundle content structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleContent {
    pub version: String,
    pub window: TimeWindow,
    pub date: String,
    pub super_root: String,
    pub previous_super_root: String,
    pub session_threads: Vec<SessionThread>,
    pub anomalies: AnomalyInventory,
    pub per_vm_segments: Vec<VMSegmentPreview>,
    pub cids_index: CIDsIndex,
    pub signatures: BundleSignatures,
}

/// Time window for the bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

/// Session thread representing end-to-end causality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionThread {
    pub thread_id: String,
    pub client: ClientInfo,
    pub server: ServerInfo,
    pub spans: Vec<Span>,
    pub end_to_end: EndToEndMetrics,
    pub security_trace: SecurityTrace,
}

/// Client information with geolocation and identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub wallet: String,
    pub geo_did: String,
    pub ipv6: String,
    pub ua_hash: String,
    pub qlock: QLockInfo,
}

/// Server information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub svc: String,
    pub geo_did: String,
    pub ipv6: String,
    pub pod: String,
    pub image: String,
}

/// Quantum lock information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QLockInfo {
    pub session: String,
    pub policy: String,
    pub mfa: bool,
}

/// Span representing a single VM operation in the thread
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    pub span_id: String,
    pub vm: String,
    pub name: String,
    pub inputs: serde_json::Value,
    pub outputs: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sec: Option<SecurityInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec: Option<ExecutionInfo>,
    pub links: SpanLinks,
    pub hash: String,
}

/// Links between spans for causality tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanLinks {
    pub prev: Option<String>,
    pub next: Option<String>,
}

/// Security information for spans
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rbac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ips_action: Option<String>,
}

/// Execution information for spans
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionInfo {
    pub model: String,
    pub verdict: String,
    pub seed: String,
}

/// End-to-end metrics for the session thread
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndToEndMetrics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p50_latency_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p95_latency_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_in: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_out: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny_reason: Option<String>,
}

/// Security trace aggregation for the session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityTrace {
    pub ids: Vec<IDSEvent>,
    pub ips: Vec<IPSEvent>,
    pub rbac: Vec<RBACEvent>,
    pub qlock: Vec<QLockEvent>,
    pub leak_signals: Vec<LeakEvent>,
    pub port_scan: Vec<PortScanEvent>,
}

/// Intrusion Detection System event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDSEvent {
    pub sig: String,
    pub sev: String,
    pub evidence: String,
}

/// Intrusion Prevention System event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPSEvent {
    pub rule: String,
    pub packets: u64,
}

/// Role-Based Access Control event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RBACEvent {
    pub role: String,
    pub perm: String,
    pub result: String,
}

/// Quantum Lock event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QLockEvent {
    pub session: String,
    pub events: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Data leak detection event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeakEvent {
    // TODO: Define leak event structure
}

/// Port scan detection event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortScanEvent {
    // TODO: Define port scan event structure
}

/// Global anomaly inventory for the window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyInventory {
    pub spikes: Vec<AnomalySpike>,
    pub clock: Vec<ClockAnomaly>,
    pub replay: Vec<ReplayAnomaly>,
    pub leak: Vec<LeakAnomaly>,
    pub port_scans: Vec<PortScanSummary>,
}

/// Performance anomaly spike
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalySpike {
    pub vmid: String,
    pub factor: f64,
    pub records: u64,
}

/// Clock anomaly detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClockAnomaly {
    // TODO: Define clock anomaly structure
}

/// Replay attack anomaly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayAnomaly {
    // TODO: Define replay anomaly structure
}

/// Data leak anomaly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeakAnomaly {
    pub thread_id: String,
    pub heuristic: String,
    pub details: serde_json::Value,
    pub vm_path: Vec<String>,
    pub status: String,
}

/// Port scan summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortScanSummary {
    pub src: String,
    pub hits: u64,
}

/// VM segment preview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMSegmentPreview {
    pub vm: VMInfo,
    pub segment: SegmentInfo,
    pub records_preview: RecordsPreview,
    pub totals: ResourceTotals,
    pub roots: SegmentRoots,
    pub cids: SegmentCIDs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signatures: Option<SegmentSignatures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sealed: Option<bool>,
}

/// VM information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub vm_type: String,
    pub image: String,
}

/// Segment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentInfo {
    pub id: String,
    pub start_ts: DateTime<Utc>,
    pub end_ts: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_segment_root: Option<String>,
    pub record_count: u64,
}

/// Preview of first and last records in segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordsPreview {
    pub first_1: RecordPreview,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_1: Option<RecordPreview>,
}

/// Individual record preview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordPreview {
    pub rid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<serde_json::Value>,
    pub action: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<serde_json::Value>,
    pub hash: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_anchor: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sig: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sec: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub court_anchor: Option<String>,
}

/// Resource usage totals for the segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceTotals {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_kb_avg: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io: Option<IOTotals>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net: Option<NetworkTotals>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sec: Option<SecurityTotals>,
}

/// I/O totals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOTotals {
    pub r: u64,
    pub w: u64,
}

/// Network totals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTotals {
    pub flows: u64,
}

/// Security totals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityTotals {
    pub allow: u64,
    pub deny: u64,
    pub qlock_events: u64,
}

/// Segment cryptographic roots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentRoots {
    pub seg_merkle_root: String,
    pub receipt_self: String,
}

/// Content-addressed identifiers for segment data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentCIDs {
    pub jsonl: String,
    pub cbor: String,
}

/// Segment signatures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentSignatures {
    pub aggregate: AggregateSignature,
}

/// Aggregate signature structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateSignature {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bls: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pqc_multi: Option<Vec<String>>,
}

/// CIDs index for tickets and PoE candidates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CIDsIndex {
    pub tickets: Vec<String>,
    pub poe_candidates: Vec<String>,
}

/// Bundle-level signatures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleSignatures {
    pub bundle_bls: String,
    pub bundle_pqc_multi: Vec<String>,
}

/// Builder for constructing session threads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadBuilder {
    thread_id: String,
    client: ClientInfo,
    server: ServerInfo,
    spans: Vec<Span>,
}

impl ThreadBuilder {
    pub fn new(thread_id: String, client: ClientInfo, server: ServerInfo) -> Self {
        Self {
            thread_id,
            client,
            server,
            spans: Vec::new(),
        }
    }

    pub fn add_span(&mut self, span: Span) -> &mut Self {
        self.spans.push(span);
        self
    }

    pub fn build(self, end_to_end: EndToEndMetrics, security_trace: SecurityTrace) -> SessionThread {
        SessionThread {
            thread_id: self.thread_id,
            client: self.client,
            server: self.server,
            spans: self.spans,
            end_to_end,
            security_trace,
        }
    }
}

/// Builder for constructing security traces
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecTraceBuilder {
    ids: Vec<IDSEvent>,
    ips: Vec<IPSEvent>,
    rbac: Vec<RBACEvent>,
    qlock: Vec<QLockEvent>,
    leak_signals: Vec<LeakEvent>,
    port_scan: Vec<PortScanEvent>,
}

impl SecTraceBuilder {
    pub fn new() -> Self {
        Self {
            ids: Vec::new(),
            ips: Vec::new(),
            rbac: Vec::new(),
            qlock: Vec::new(),
            leak_signals: Vec::new(),
            port_scan: Vec::new(),
        }
    }

    pub fn add_ids_event(&mut self, event: IDSEvent) -> &mut Self {
        self.ids.push(event);
        self
    }

    pub fn add_ips_event(&mut self, event: IPSEvent) -> &mut Self {
        self.ips.push(event);
        self
    }

    pub fn add_rbac_event(&mut self, event: RBACEvent) -> &mut Self {
        self.rbac.push(event);
        self
    }

    pub fn add_qlock_event(&mut self, event: QLockEvent) -> &mut Self {
        self.qlock.push(event);
        self
    }

    pub fn build(self) -> SecurityTrace {
        SecurityTrace {
            ids: self.ids,
            ips: self.ips,
            rbac: self.rbac,
            qlock: self.qlock,
            leak_signals: self.leak_signals,
            port_scan: self.port_scan,
        }
    }
}

/// Helper for linking spans across VMs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanLink {
    pub from_span: String,
    pub to_span: String,
    pub vm_transition: (String, String),
}

impl SpanLink {
    pub fn new(from_span: String, to_span: String, from_vm: String, to_vm: String) -> Self {
        Self {
            from_span,
            to_span,
            vm_transition: (from_vm, to_vm),
        }
    }
}
