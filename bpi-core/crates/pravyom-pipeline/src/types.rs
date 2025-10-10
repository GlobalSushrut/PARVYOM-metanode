//! Pravyom Standard Pipeline v1.0 - Additional Type Definitions
//! 
//! Extended type definitions and enums for the pipeline

use crate::VmType;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Pipeline Error Types
#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Cryptographic error: {0}")]
    Crypto(String),
    
    #[error("Timeout error: {0}")]
    Timeout(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
}

/// Action Types - Canonical set for VM actions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionType {
    #[serde(rename = "WRITE")]
    Write,
    #[serde(rename = "READ")]
    Read,
    #[serde(rename = "EXEC")]
    Exec,
    #[serde(rename = "NET")]
    Net,
    #[serde(rename = "POLICY")]
    Policy,
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionType::Write => write!(f, "WRITE"),
            ActionType::Read => write!(f, "READ"),
            ActionType::Exec => write!(f, "EXEC"),
            ActionType::Net => write!(f, "NET"),
            ActionType::Policy => write!(f, "POLICY"),
        }
    }
}

/// Actor Role Types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorRole {
    #[serde(rename = "client")]
    Client,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "admin")]
    Admin,
}

impl fmt::Display for ActorRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActorRole::Client => write!(f, "client"),
            ActorRole::Service => write!(f, "service"),
            ActorRole::Admin => write!(f, "admin"),
        }
    }
}

/// Execution Model Types for PoE
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExecModel {
    #[serde(rename = "WASM")]
    Wasm,
    #[serde(rename = "SGX")]
    Sgx,
    #[serde(rename = "DL")]
    Dl, // Deep Learning
}

impl fmt::Display for ExecModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecModel::Wasm => write!(f, "WASM"),
            ExecModel::Sgx => write!(f, "SGX"),
            ExecModel::Dl => write!(f, "DL"),
        }
    }
}

/// Execution Verdict Types for PoE
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExecVerdict {
    #[serde(rename = "DET")]
    Deterministic,
    #[serde(rename = "QUASI")]
    QuasiDeterministic,
    #[serde(rename = "NONDET")]
    NonDeterministic,
}

impl fmt::Display for ExecVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecVerdict::Deterministic => write!(f, "DET"),
            ExecVerdict::QuasiDeterministic => write!(f, "QUASI"),
            ExecVerdict::NonDeterministic => write!(f, "NONDET"),
        }
    }
}

/// Segment Status Types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SegmentStatus {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "failed")]
    Failed,
}

impl fmt::Display for SegmentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SegmentStatus::Ok => write!(f, "ok"),
            SegmentStatus::Pending => write!(f, "pending"),
            SegmentStatus::Failed => write!(f, "failed"),
        }
    }
}

/// Market Class Types for BPCI Auctions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarketClass {
    #[serde(rename = "PoE_EXECUTION")]
    PoeExecution,
    #[serde(rename = "STORAGE")]
    Storage,
    #[serde(rename = "COMPUTE")]
    Compute,
    #[serde(rename = "BANDWIDTH")]
    Bandwidth,
}

impl fmt::Display for MarketClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarketClass::PoeExecution => write!(f, "PoE_EXECUTION"),
            MarketClass::Storage => write!(f, "STORAGE"),
            MarketClass::Compute => write!(f, "COMPUTE"),
            MarketClass::Bandwidth => write!(f, "BANDWIDTH"),
        }
    }
}

/// Privacy Tier Types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrivacyTier {
    #[serde(rename = "PUBLIC")]
    Public,
    #[serde(rename = "PSEUDONYMOUS")]
    Pseudonymous,
    #[serde(rename = "COURT_SEALED")]
    CourtSealed,
}

impl fmt::Display for PrivacyTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrivacyTier::Public => write!(f, "PUBLIC"),
            PrivacyTier::Pseudonymous => write!(f, "PSEUDONYMOUS"),
            PrivacyTier::CourtSealed => write!(f, "COURT_SEALED"),
        }
    }
}

/// Warrant Token for Court-Sealed Data Access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarrantToken {
    pub token_id: String,
    pub court_id: String,
    pub target_wallet: String,
    pub scope: WarrantScope,
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub signature: String,
}

/// Warrant Scope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarrantScope {
    pub vm_types: Vec<VmType>,
    pub time_range: Option<crate::TimeWindow>,
    pub action_types: Vec<ActionType>,
}

/// Court Validator Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtValidator {
    pub validator_id: String,
    pub public_key: String,
    pub jurisdiction: String,
    pub active: bool,
}

/// Court Quorum Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtQuorum {
    pub required_validators: u8,
    pub validators: Vec<CourtValidator>,
    pub consensus_threshold: f64, // 0.0 to 1.0
}

/// Extended VM Information with Privacy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedVmInfo {
    pub base: crate::VmInfo,
    pub privacy_tier: PrivacyTier,
    pub court_hooks: Option<String>,
    pub isolation_level: IsolationLevel,
}

/// VM Isolation Level
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IsolationLevel {
    #[serde(rename = "STANDARD")]
    Standard,
    #[serde(rename = "ENHANCED")]
    Enhanced,
    #[serde(rename = "COURT_SEALED")]
    CourtSealed,
}

impl fmt::Display for IsolationLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IsolationLevel::Standard => write!(f, "STANDARD"),
            IsolationLevel::Enhanced => write!(f, "ENHANCED"),
            IsolationLevel::CourtSealed => write!(f, "COURT_SEALED"),
        }
    }
}

/// BPI Transaction Types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BpiTxType {
    #[serde(rename = "ZIPLOCK_TICKET")]
    ZiplockTicket,
    #[serde(rename = "POE_BUNDLE")]
    PoeBundle,
    #[serde(rename = "GOVERNANCE")]
    Governance,
    #[serde(rename = "TREASURY")]
    Treasury,
}

impl fmt::Display for BpiTxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BpiTxType::ZiplockTicket => write!(f, "ZIPLOCK_TICKET"),
            BpiTxType::PoeBundle => write!(f, "POE_BUNDLE"),
            BpiTxType::Governance => write!(f, "GOVERNANCE"),
            BpiTxType::Treasury => write!(f, "TREASURY"),
        }
    }
}

/// BPI Transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpiTransaction {
    pub tx_id: String,
    pub tx_type: BpiTxType,
    pub payload: serde_json::Value,
    pub block_height: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub gas_used: u64,
    pub status: crate::TxStatus,
}

/// Performance Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub ziplock_write_rps: f64,
    pub ticket_commit_p95_ms: f64,
    pub bundle_seal_p95_ms: f64,
    pub cid_retrieval_p999_local_ms: f64,
    pub cid_retrieval_p999_remote_ms: f64,
}

/// Storage Backend Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageBackend {
    pub name: String,
    pub endpoint: String,
    pub credentials: Option<String>,
    pub redundancy_level: u8,
    pub availability_sla: f64, // 0.0 to 1.0
}

/// Network Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub listen_addr: String,
    pub external_addr: String,
    pub tls_enabled: bool,
    pub qlock_enabled: bool,
    pub firewall_rules: Vec<FirewallRule>,
}

/// Firewall Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallRule {
    pub rule_id: String,
    pub action: FirewallAction,
    pub src_pattern: String,
    pub dst_pattern: String,
    pub port_range: Option<(u16, u16)>,
    pub protocol: NetworkProtocol,
}

/// Firewall Action
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FirewallAction {
    #[serde(rename = "ALLOW")]
    Allow,
    #[serde(rename = "DENY")]
    Deny,
    #[serde(rename = "LOG")]
    Log,
}

/// Network Protocol
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NetworkProtocol {
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
    #[serde(rename = "ICMP")]
    Icmp,
    #[serde(rename = "ANY")]
    Any,
}

/// Resource Governor Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceGovernor {
    pub cpu_limit_percent: f64,
    pub ram_limit_mb: u64,
    pub io_limit_mbps: f64,
    pub network_limit_mbps: f64,
    pub enforcement_mode: EnforcementMode,
}

/// Enforcement Mode for Resource Governor
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnforcementMode {
    #[serde(rename = "SOFT")]
    Soft, // Log violations
    #[serde(rename = "HARD")]
    Hard, // Throttle/kill processes
    #[serde(rename = "ADAPTIVE")]
    Adaptive, // Dynamic adjustment
}

/// Audit Trail Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTrailEntry {
    pub entry_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_type: AuditEventType,
    pub source: AuditSource,
    pub details: serde_json::Value,
    pub integrity_hash: String,
}

/// Audit Event Type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuditEventType {
    #[serde(rename = "RECORD_CREATED")]
    RecordCreated,
    #[serde(rename = "SEGMENT_SEALED")]
    SegmentSealed,
    #[serde(rename = "TICKET_SUBMITTED")]
    TicketSubmitted,
    #[serde(rename = "BUNDLE_CREATED")]
    BundleCreated,
    #[serde(rename = "AUCTION_OPENED")]
    AuctionOpened,
    #[serde(rename = "ERROR_OCCURRED")]
    ErrorOccurred,
}

/// Audit Source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSource {
    pub component: String,
    pub vm_id: Option<String>,
    pub process_id: Option<u32>,
}

/// Configuration Validation Result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    
    pub fn add_error(&mut self, error: String) {
        self.valid = false;
        self.errors.push(error);
    }
    
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
    
    pub fn is_valid(&self) -> bool {
        self.valid && self.errors.is_empty()
    }
}

/// Default implementations for common types
impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            ziplock_write_rps: 100000.0,
            ticket_commit_p95_ms: 400.0,
            bundle_seal_p95_ms: 50.0,
            cid_retrieval_p999_local_ms: 150.0,
            cid_retrieval_p999_remote_ms: 900.0,
        }
    }
}

impl Default for ResourceGovernor {
    fn default() -> Self {
        Self {
            cpu_limit_percent: 80.0,
            ram_limit_mb: 4096,
            io_limit_mbps: 100.0,
            network_limit_mbps: 100.0,
            enforcement_mode: EnforcementMode::Soft,
        }
    }
}
