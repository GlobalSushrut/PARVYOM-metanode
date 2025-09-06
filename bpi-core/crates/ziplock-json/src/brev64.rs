//! BREV-64 forensics encoding for ZIPLOCK-JSON
//! Black-box forensics encoding for attack reasons, evidence, and snapshots

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use blake3::Hasher;
use crate::{ZjlResult, ZjlError};
use crate::blocks::{BlockType, Block};

/// BREV-64 attack reason codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum AttackReason {
    // Memory safety violations
    BufferOverflow = 0x01,
    UseAfterFree = 0x02,
    DoubleFree = 0x03,
    NullPointerDeref = 0x04,
    StackOverflow = 0x05,
    HeapCorruption = 0x06,

    // Injection attacks
    SqlInjection = 0x10,
    CommandInjection = 0x11,
    CodeInjection = 0x12,
    ScriptInjection = 0x13,
    LdapInjection = 0x14,
    XmlInjection = 0x15,

    // Cryptographic attacks
    WeakCrypto = 0x20,
    KeyLeakage = 0x21,
    RandomnessFailure = 0x22,
    HashCollision = 0x23,
    TimingAttack = 0x24,
    SideChannel = 0x25,

    // Network attacks
    ManInTheMiddle = 0x30,
    DnsPoison = 0x31,
    ArpSpoof = 0x32,
    PacketInjection = 0x33,
    SessionHijack = 0x34,
    ReplayAttack = 0x35,

    // Access control violations
    PrivilegeEscalation = 0x40,
    AuthBypass = 0x41,
    SessionFixation = 0x42,
    TokenTheft = 0x43,
    PermissionBypass = 0x44,
    RoleConfusion = 0x45,

    // Resource exhaustion
    DosAttack = 0x50,
    ResourceExhaustion = 0x51,
    MemoryLeak = 0x52,
    CpuExhaustion = 0x53,
    DiskFull = 0x54,
    ConnectionFlood = 0x55,

    // Data integrity
    DataCorruption = 0x60,
    ChecksumMismatch = 0x61,
    TamperDetected = 0x62,
    UnauthorizedModify = 0x63,
    IntegrityViolation = 0x64,
    AuditLogTamper = 0x65,

    // VM-specific attacks
    VmEscape = 0x70,
    ContainerBreakout = 0x71,
    SandboxEscape = 0x72,
    HypervisorAttack = 0x73,
    GuestToHost = 0x74,
    VmwareExploit = 0x75,

    // Unknown/Custom
    Unknown = 0xFE,
    Custom = 0xFF,
}

impl AttackReason {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(Self::BufferOverflow),
            0x02 => Some(Self::UseAfterFree),
            0x03 => Some(Self::DoubleFree),
            0x04 => Some(Self::NullPointerDeref),
            0x05 => Some(Self::StackOverflow),
            0x06 => Some(Self::HeapCorruption),
            0x10 => Some(Self::SqlInjection),
            0x11 => Some(Self::CommandInjection),
            0x12 => Some(Self::CodeInjection),
            0x13 => Some(Self::ScriptInjection),
            0x14 => Some(Self::LdapInjection),
            0x15 => Some(Self::XmlInjection),
            0x20 => Some(Self::WeakCrypto),
            0x21 => Some(Self::KeyLeakage),
            0x22 => Some(Self::RandomnessFailure),
            0x23 => Some(Self::HashCollision),
            0x24 => Some(Self::TimingAttack),
            0x25 => Some(Self::SideChannel),
            0x30 => Some(Self::ManInTheMiddle),
            0x31 => Some(Self::DnsPoison),
            0x32 => Some(Self::ArpSpoof),
            0x33 => Some(Self::PacketInjection),
            0x34 => Some(Self::SessionHijack),
            0x35 => Some(Self::ReplayAttack),
            0x40 => Some(Self::PrivilegeEscalation),
            0x41 => Some(Self::AuthBypass),
            0x42 => Some(Self::SessionFixation),
            0x43 => Some(Self::TokenTheft),
            0x44 => Some(Self::PermissionBypass),
            0x45 => Some(Self::RoleConfusion),
            0x50 => Some(Self::DosAttack),
            0x51 => Some(Self::ResourceExhaustion),
            0x52 => Some(Self::MemoryLeak),
            0x53 => Some(Self::CpuExhaustion),
            0x54 => Some(Self::DiskFull),
            0x55 => Some(Self::ConnectionFlood),
            0x60 => Some(Self::DataCorruption),
            0x61 => Some(Self::ChecksumMismatch),
            0x62 => Some(Self::TamperDetected),
            0x63 => Some(Self::UnauthorizedModify),
            0x64 => Some(Self::IntegrityViolation),
            0x65 => Some(Self::AuditLogTamper),
            0x70 => Some(Self::VmEscape),
            0x71 => Some(Self::ContainerBreakout),
            0x72 => Some(Self::SandboxEscape),
            0x73 => Some(Self::HypervisorAttack),
            0x74 => Some(Self::GuestToHost),
            0x75 => Some(Self::VmwareExploit),
            0xFE => Some(Self::Unknown),
            0xFF => Some(Self::Custom),
            _ => None,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::BufferOverflow => "Buffer overflow detected",
            Self::UseAfterFree => "Use-after-free vulnerability",
            Self::DoubleFree => "Double-free detected",
            Self::NullPointerDeref => "Null pointer dereference",
            Self::StackOverflow => "Stack overflow",
            Self::HeapCorruption => "Heap corruption detected",
            Self::SqlInjection => "SQL injection attempt",
            Self::CommandInjection => "Command injection",
            Self::CodeInjection => "Code injection attack",
            Self::ScriptInjection => "Script injection",
            Self::LdapInjection => "LDAP injection",
            Self::XmlInjection => "XML injection",
            Self::WeakCrypto => "Weak cryptographic algorithm",
            Self::KeyLeakage => "Cryptographic key leakage",
            Self::RandomnessFailure => "Insufficient randomness",
            Self::HashCollision => "Hash collision attack",
            Self::TimingAttack => "Timing attack detected",
            Self::SideChannel => "Side-channel attack",
            Self::ManInTheMiddle => "Man-in-the-middle attack",
            Self::DnsPoison => "DNS poisoning",
            Self::ArpSpoof => "ARP spoofing",
            Self::PacketInjection => "Packet injection",
            Self::SessionHijack => "Session hijacking",
            Self::ReplayAttack => "Replay attack",
            Self::PrivilegeEscalation => "Privilege escalation",
            Self::AuthBypass => "Authentication bypass",
            Self::SessionFixation => "Session fixation",
            Self::TokenTheft => "Token theft",
            Self::PermissionBypass => "Permission bypass",
            Self::RoleConfusion => "Role confusion attack",
            Self::DosAttack => "Denial of service",
            Self::ResourceExhaustion => "Resource exhaustion",
            Self::MemoryLeak => "Memory leak detected",
            Self::CpuExhaustion => "CPU exhaustion",
            Self::DiskFull => "Disk space exhaustion",
            Self::ConnectionFlood => "Connection flooding",
            Self::DataCorruption => "Data corruption",
            Self::ChecksumMismatch => "Checksum mismatch",
            Self::TamperDetected => "Tampering detected",
            Self::UnauthorizedModify => "Unauthorized modification",
            Self::IntegrityViolation => "Integrity violation",
            Self::AuditLogTamper => "Audit log tampering",
            Self::VmEscape => "Virtual machine escape",
            Self::ContainerBreakout => "Container breakout",
            Self::SandboxEscape => "Sandbox escape",
            Self::HypervisorAttack => "Hypervisor attack",
            Self::GuestToHost => "Guest-to-host attack",
            Self::VmwareExploit => "VMware exploit",
            Self::Unknown => "Unknown attack type",
            Self::Custom => "Custom attack type",
        }
    }
}

/// BREV-64 evidence types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum EvidenceType {
    MemoryDump = 0x01,
    StackTrace = 0x02,
    NetworkCapture = 0x03,
    SystemCall = 0x04,
    FileSystem = 0x05,
    Registry = 0x06,
    ProcessList = 0x07,
    LogEntry = 0x08,
    CrashDump = 0x09,
    CoreDump = 0x0A,
    Disassembly = 0x0B,
    Shellcode = 0x0C,
    Payload = 0x0D,
    Certificate = 0x0E,
    Signature = 0x0F,
}

impl EvidenceType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(Self::MemoryDump),
            0x02 => Some(Self::StackTrace),
            0x03 => Some(Self::NetworkCapture),
            0x04 => Some(Self::SystemCall),
            0x05 => Some(Self::FileSystem),
            0x06 => Some(Self::Registry),
            0x07 => Some(Self::ProcessList),
            0x08 => Some(Self::LogEntry),
            0x09 => Some(Self::CrashDump),
            0x0A => Some(Self::CoreDump),
            0x0B => Some(Self::Disassembly),
            0x0C => Some(Self::Shellcode),
            0x0D => Some(Self::Payload),
            0x0E => Some(Self::Certificate),
            0x0F => Some(Self::Signature),
            _ => None,
        }
    }
}

/// BREV-64 forensic record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForensicRecord {
    /// Unique record ID
    pub record_id: [u8; 16],
    /// Attack reason code
    pub reason: AttackReason,
    /// Timestamp (nanoseconds since Unix epoch)
    pub timestamp_ns: u64,
    /// VM identifier where attack occurred
    pub vm_id: String,
    /// Process ID (if applicable)
    pub process_id: Option<u32>,
    /// Thread ID (if applicable)
    pub thread_id: Option<u32>,
    /// Memory address (if applicable)
    pub memory_address: Option<u64>,
    /// Severity level (0-255)
    pub severity: u8,
    /// Evidence entries
    pub evidence: Vec<EvidenceEntry>,
    /// Attack vector description
    pub vector: String,
    /// Mitigation applied
    pub mitigation: String,
    /// Custom attributes
    pub attributes: HashMap<String, String>,
}

/// Evidence entry in forensic record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceEntry {
    /// Evidence type
    pub evidence_type: EvidenceType,
    /// Evidence data (base64 encoded)
    pub data: String,
    /// Data hash for integrity
    pub hash: [u8; 32],
    /// Size of original data
    pub size: u64,
    /// Description
    pub description: String,
}

impl EvidenceEntry {
    pub fn new(evidence_type: EvidenceType, data: Vec<u8>, description: String) -> Self {
        // Calculate hash
        let mut hasher = Hasher::new();
        hasher.update(&data);
        let hash = *hasher.finalize().as_bytes();

        // Encode data as base64
        let encoded_data = base64::encode(&data);

        Self {
            evidence_type,
            data: encoded_data,
            hash,
            size: data.len() as u64,
            description,
        }
    }

    pub fn decode_data(&self) -> ZjlResult<Vec<u8>> {
        base64::decode(&self.data)
            .map_err(|e| ZjlError::DecodingError(format!("Base64 decode error: {}", e)))
    }

    pub fn verify_integrity(&self) -> ZjlResult<bool> {
        let decoded = self.decode_data()?;
        let mut hasher = Hasher::new();
        hasher.update(&decoded);
        let calculated_hash = *hasher.finalize().as_bytes();
        Ok(calculated_hash == self.hash)
    }
}

/// System snapshot for forensic analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSnapshot {
    /// Snapshot ID
    pub snapshot_id: [u8; 16],
    /// Timestamp
    pub timestamp_ns: u64,
    /// VM state
    pub vm_state: VmState,
    /// Process list
    pub processes: Vec<ProcessInfo>,
    /// Network connections
    pub network_connections: Vec<NetworkConnection>,
    /// File system changes
    pub fs_changes: Vec<FileSystemChange>,
    /// Memory regions
    pub memory_regions: Vec<MemoryRegion>,
    /// System metrics
    pub metrics: SystemMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmState {
    pub vm_id: String,
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub disk_usage: u64,
    pub network_rx: u64,
    pub network_tx: u64,
    pub uptime_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub command_line: String,
    pub cpu_percent: f64,
    pub memory_bytes: u64,
    pub start_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConnection {
    pub local_addr: String,
    pub local_port: u16,
    pub remote_addr: String,
    pub remote_port: u16,
    pub protocol: String,
    pub state: String,
    pub pid: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemChange {
    pub path: String,
    pub change_type: String, // "created", "modified", "deleted"
    pub timestamp: u64,
    pub size: Option<u64>,
    pub permissions: Option<String>,
    pub hash: Option<[u8; 32]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRegion {
    pub start_addr: u64,
    pub end_addr: u64,
    pub permissions: String,
    pub mapping: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_cores: u32,
    pub total_memory: u64,
    pub available_memory: u64,
    pub total_disk: u64,
    pub available_disk: u64,
    pub load_average: [f64; 3],
    pub open_files: u32,
    pub network_interfaces: Vec<String>,
}

/// Attack graph node for forensic analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackGraphNode {
    /// Node ID
    pub node_id: String,
    /// Attack step description
    pub step: String,
    /// Timestamp
    pub timestamp_ns: u64,
    /// Attack technique (MITRE ATT&CK)
    pub technique: String,
    /// Tactic
    pub tactic: String,
    /// Evidence supporting this step
    pub evidence_refs: Vec<String>,
    /// Confidence level (0.0-1.0)
    pub confidence: f64,
    /// Connected nodes
    pub connections: Vec<String>,
}

/// Attack graph for forensic reconstruction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackGraph {
    /// Graph ID
    pub graph_id: [u8; 16],
    /// Attack campaign ID
    pub campaign_id: String,
    /// Nodes in the graph
    pub nodes: Vec<AttackGraphNode>,
    /// Attack timeline
    pub timeline: Vec<u64>,
    /// Root cause analysis
    pub root_cause: String,
    /// Attack attribution
    pub attribution: Option<String>,
    /// Indicators of compromise
    pub iocs: Vec<String>,
}

/// BREV-64 encoder for forensic data
pub struct Brev64Encoder;

impl Brev64Encoder {
    /// Encode forensic record to BREV-64 block
    pub fn encode_forensic_record(record: &ForensicRecord) -> ZjlResult<Block> {
        let serialized = serde_json::to_vec(record)
            .map_err(|e| ZjlError::SerializationErrorString(e.to_string()))?;

        let mut hasher = Hasher::new();
        hasher.update(&serialized);
        let hash = *hasher.finalize().as_bytes();

        Ok(Block::new(BlockType::Reason, 0, serialized, hash))
    }

    /// Encode system snapshot to BREV-64 block
    pub fn encode_snapshot(snapshot: &SystemSnapshot) -> ZjlResult<Block> {
        let serialized = serde_json::to_vec(snapshot)
            .map_err(|e| ZjlError::SerializationErrorString(e.to_string()))?;

        let mut hasher = Hasher::new();
        hasher.update(&serialized);
        let hash = *hasher.finalize().as_bytes();

        Ok(Block::new(BlockType::Snapshot, 0, serialized, hash))
    }

    /// Encode attack graph to BREV-64 block
    pub fn encode_attack_graph(graph: &AttackGraph) -> ZjlResult<Block> {
        let serialized = serde_json::to_vec(graph)
            .map_err(|e| ZjlError::SerializationErrorString(e.to_string()))?;

        let mut hasher = Hasher::new();
        hasher.update(&serialized);
        let hash = *hasher.finalize().as_bytes();

        Ok(Block::new(BlockType::AttackGraph, 0, serialized, hash))
    }

    /// Decode forensic record from block
    pub fn decode_forensic_record(block: &Block) -> ZjlResult<ForensicRecord> {
        if block.block_type() != Some(BlockType::Reason) {
            return Err(ZjlError::InvalidBlockType(block.header.block_type));
        }

        serde_json::from_slice(&block.payload)
            .map_err(|e| ZjlError::DecodingError(e.to_string()))
    }

    /// Decode system snapshot from block
    pub fn decode_snapshot(block: &Block) -> ZjlResult<SystemSnapshot> {
        if block.block_type() != Some(BlockType::Snapshot) {
            return Err(ZjlError::InvalidBlockType(block.header.block_type));
        }

        serde_json::from_slice(&block.payload)
            .map_err(|e| ZjlError::DecodingError(e.to_string()))
    }

    /// Decode attack graph from block
    pub fn decode_attack_graph(block: &Block) -> ZjlResult<AttackGraph> {
        if block.block_type() != Some(BlockType::AttackGraph) {
            return Err(ZjlError::InvalidBlockType(block.header.block_type));
        }

        serde_json::from_slice(&block.payload)
            .map_err(|e| ZjlError::DecodingError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_attack_reason_conversion() {
        assert_eq!(AttackReason::from_u8(0x01), Some(AttackReason::BufferOverflow));
        assert_eq!(AttackReason::from_u8(0xFF), Some(AttackReason::Custom));
        assert_eq!(AttackReason::from_u8(0x99), None);
    }

    #[test]
    fn test_evidence_entry() {
        let data = b"test evidence data".to_vec();
        let entry = EvidenceEntry::new(
            EvidenceType::MemoryDump,
            data,
            "Test memory dump".to_string(),
        );

        assert_eq!(entry.evidence_type, EvidenceType::MemoryDump);
        assert!(entry.verify_integrity().unwrap());
        assert_eq!(entry.decode_data().unwrap(), b"test evidence data");
    }

    #[test]
    fn test_forensic_record_encoding() {
        let mut record = ForensicRecord {
            record_id: *Uuid::new_v4().as_bytes(),
            reason: AttackReason::BufferOverflow,
            timestamp_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
            vm_id: "test_vm".to_string(),
            process_id: Some(1234),
            thread_id: Some(5678),
            memory_address: Some(0xDEADBEEF),
            severity: 200,
            evidence: vec![],
            vector: "Stack-based buffer overflow".to_string(),
            mitigation: "Process terminated".to_string(),
            attributes: HashMap::new(),
        };

        let evidence = EvidenceEntry::new(
            EvidenceType::StackTrace,
            b"stack trace data".to_vec(),
            "Stack trace at crash".to_string(),
        );
        record.evidence.push(evidence);

        let block = Brev64Encoder::encode_forensic_record(&record).unwrap();
        assert_eq!(block.block_type(), Some(BlockType::Reason));

        let decoded = Brev64Encoder::decode_forensic_record(&block).unwrap();
        assert_eq!(decoded.reason, AttackReason::BufferOverflow);
        assert_eq!(decoded.vm_id, "test_vm");
        assert_eq!(decoded.evidence.len(), 1);
    }

    #[test]
    fn test_system_snapshot() {
        let snapshot = SystemSnapshot {
            snapshot_id: *Uuid::new_v4().as_bytes(),
            timestamp_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
            vm_state: VmState {
                vm_id: "test_vm".to_string(),
                cpu_usage: 45.2,
                memory_usage: 1024 * 1024 * 512, // 512MB
                disk_usage: 1024 * 1024 * 1024, // 1GB
                network_rx: 1000,
                network_tx: 2000,
                uptime_seconds: 3600,
            },
            processes: vec![],
            network_connections: vec![],
            fs_changes: vec![],
            memory_regions: vec![],
            metrics: SystemMetrics {
                cpu_cores: 4,
                total_memory: 1024 * 1024 * 1024 * 8, // 8GB
                available_memory: 1024 * 1024 * 1024 * 4, // 4GB
                total_disk: 1024 * 1024 * 1024 * 100, // 100GB
                available_disk: 1024 * 1024 * 1024 * 50, // 50GB
                load_average: [1.0, 1.5, 2.0],
                open_files: 1024,
                network_interfaces: vec!["eth0".to_string(), "lo".to_string()],
            },
        };

        let block = Brev64Encoder::encode_snapshot(&snapshot).unwrap();
        assert_eq!(block.block_type(), Some(BlockType::Snapshot));

        let decoded = Brev64Encoder::decode_snapshot(&block).unwrap();
        assert_eq!(decoded.vm_state.vm_id, "test_vm");
        assert_eq!(decoded.metrics.cpu_cores, 4);
    }
}
