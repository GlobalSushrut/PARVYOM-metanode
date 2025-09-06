//! Runtime Audit Node - Core data structure for audit tree
//!
//! Each node represents a single auditable action/operation performed by hosted code

use crate::{AUDIT_NODE_HASH, proof_chain::ProofChain};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use bpi_enc::domain_hash;

/// Universal Runtime Audit Tree Node
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeAuditNode {
    /// Unique node identifier in the audit tree
    pub node_id: [u8; 32],
    /// Parent node (None for root nodes)
    pub parent_id: Option<[u8; 32]>,
    /// Child nodes for hierarchical structure
    pub children: Vec<[u8; 32]>,
    
    /// Temporal Information
    pub timestamp_ns: u64,
    pub duration_ns: Option<u64>,
    pub sequence_number: u64,
    
    /// Location/Address Information
    pub execution_context: ExecutionContext,
    pub runtime_address: RuntimeAddress,
    
    /// Action/Operation Details
    pub operation_type: OperationType,
    pub operation_data: OperationData,
    pub binary_outputs: Vec<BinaryOutput>,
    
    /// Cryptographic Proof Chain
    pub proof_chain: ProofChain,
    pub integrity_hash: [u8; 32],
    pub signature: Vec<u8>, // Ed25519 (64 bytes)
    
    /// Audit Metadata
    pub audit_level: AuditLevel,
    pub compliance_tags: Vec<ComplianceTag>,
    pub export_metadata: ExportMetadata,
}

/// Execution Context - Where the code is running
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecutionContext {
    DockLock {
        container_id: [u8; 32],
        workload_id: [u8; 32],
        cage_config: CageConfig,
    },
    EncCluster {
        cluster_id: [u8; 32],
        node_id: [u8; 32],
        workload_spec: WorkloadSpec,
    },
    HttpCage {
        cage_id: [u8; 32],
        client_session: [u8; 32],
        request_context: RequestContext,
    },
    IoTGateway {
        gateway_id: [u8; 32],
        device_id: [u8; 32],
        device_class: IoTClass,
    },
    MobileClient {
        device_fingerprint: [u8; 32],
        app_instance: [u8; 32],
        zklock_session: [u8; 32],
    },
    FrontendClient {
        browser_session: [u8; 32],
        page_context: [u8; 32],
        user_agent_hash: [u8; 32],
    },
    SecurityMonitor {
        detector_id: [u8; 32],
        rule_id: [u8; 32],
    },
}

/// Runtime Address - Precise location in the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct RuntimeAddress {
    /// Physical/logical location
    pub location_type: LocationType,
    /// Network address (IP, port, etc.)
    pub network_address: NetworkAddress,
    /// Process/thread identifier
    pub process_context: ProcessContext,
    /// Code location (file, function, line)
    pub code_location: Option<CodeLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocationType {
    Container,
    VirtualMachine,
    BareMetal,
    Cloud,
    Edge,
    Mobile,
    Browser,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct NetworkAddress {
    pub ip_address: String,
    pub port: Option<u16>,
    pub protocol: String,
    pub hostname: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessContext {
    pub process_id: u32,
    pub thread_id: Option<u32>,
    pub parent_process_id: Option<u32>,
    pub user_id: Option<u32>,
    pub group_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CodeLocation {
    pub file_path: String,
    pub function_name: Option<String>,
    pub line_number: Option<u32>,
    pub column_number: Option<u32>,
    pub source_hash: Option<[u8; 32]>,
}

/// Operation Type - What action is being performed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperationType {
    // System Operations
    ProcessStart { command: String, args: Vec<String> },
    ProcessExit { exit_code: i32, signal: Option<i32> },
    FileAccess { path: String, mode: FileAccessMode },
    NetworkRequest { method: String, url: String, headers: HashMap<String, String> },
    
    // Container Operations
    ContainerDeploy { image: String, config: ContainerConfig },
    ContainerExec { command: String, working_dir: String },
    ContainerStop { reason: StopReason },
    
    // Cluster Operations
    WorkloadSchedule { workload_id: [u8; 32], node_assignment: [u8; 32] },
    ServiceMeshCommunication { from_service: String, to_service: String, protocol: String },
    
    // Client Operations
    HttpRequest { method: String, path: String, body_hash: [u8; 32] },
    HttpResponse { status: u16, body_hash: [u8; 32], headers: HashMap<String, String> },
    
    // IoT Operations
    SensorReading { sensor_type: String, value: Vec<u8> },
    ActuatorCommand { actuator_type: String, command: Vec<u8> },
    
    // Mobile/Frontend Operations
    UserInteraction { event_type: String, target: String, data: Vec<u8> },
    StateChange { from_state: String, to_state: String, trigger: String },
    
    // Binary Operations
    BinaryExecution { binary_hash: [u8; 32], args: Vec<String> },
    BinaryOutput { stdout: Vec<u8>, stderr: Vec<u8>, return_code: i32 },
    
    // Security Operations
    SecurityEvent { 
        event_type: String, 
        severity: SecuritySeverity, 
        description: String,
        indicators: Vec<String>,
    },
    
    // Custom Operations
    Custom { operation_name: String, data: Vec<u8> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileAccessMode {
    Read,
    Write,
    Execute,
    Create,
    Delete,
    Modify,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Operation Data - Additional context for the operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperationData {
    /// Raw operation data
    pub data: Vec<u8>,
    /// Data hash for integrity
    pub data_hash: [u8; 32],
    /// Data size in bytes
    pub size_bytes: usize,
    /// Compression used (if any)
    pub compression: Option<CompressionType>,
    /// Encryption used (if any)
    pub encryption: Option<EncryptionType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompressionType {
    None,
    Gzip,
    Lz4,
    Zstd,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EncryptionType {
    None,
    ChaCha20Poly1305,
    AES256GCM,
}

/// Binary Output Capture - Every binary output is recorded
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct BinaryOutput {
    /// Output type (stdout, stderr, file write, network send, etc.)
    pub output_type: OutputType,
    /// Raw binary data
    pub data: Vec<u8>,
    /// Hash of the data for integrity
    pub data_hash: [u8; 32],
    /// Timestamp when output was generated
    pub timestamp_ns: u64,
    /// Size in bytes
    pub size_bytes: usize,
    /// Encoding information (if applicable)
    pub encoding: Option<String>,
    /// Destination (file path, network endpoint, etc.)
    pub destination: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum OutputType {
    Stdout,
    Stderr,
    FileWrite,
    NetworkSend,
    DatabaseWrite,
    LogEntry,
    ApiResponse,
    Custom(String),
}

/// Audit Level - Importance/criticality of the audit event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuditLevel {
    Debug,
    Info,
    Standard,
    Warning,
    Error,
    Critical,
}

/// Compliance Tag - Regulatory compliance markers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComplianceTag {
    GDPR,
    HIPAA,
    SOX,
    PCI_DSS,
    ISO27001,
    ContainerSecurity,
    ProcessMonitoring,
    ClusterMonitoring,
    NetworkSecurity,
    HttpSecurity,
    NetworkMonitoring,
    IoTSecurity,
    DeviceMonitoring,
    ErrorReporting,
    Custom(String),
}

/// Export Metadata - Information for forensic export
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExportMetadata {
    /// Export priority
    pub priority: ExportPriority,
    /// Retention period in days
    pub retention_days: u32,
    /// Export formats supported
    pub supported_formats: Vec<String>,
    /// Legal hold status
    pub legal_hold: bool,
    /// Classification level
    pub classification: ClassificationLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExportPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClassificationLevel {
    Public,
    Internal,
    Confidential,
    Restricted,
    TopSecret,
}

// Configuration structs for different contexts
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct CageConfig {
    pub deterministic: bool,
    pub witness_recording: bool,
    pub syscall_filtering: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct WorkloadSpec {
    pub workload_type: String,
    pub resource_requirements: HashMap<String, String>,
    pub scheduling_constraints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct RequestContext {
    pub request_id: String,
    pub user_agent: Option<String>,
    pub client_ip: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum IoTClass {
    Sensor,
    Actuator,
    Gateway,
    Controller,
    Monitor,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ContainerConfig {
    pub image: String,
    pub environment: HashMap<String, String>,
    pub volumes: Vec<String>,
    pub network_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum StopReason {
    Normal,
    Error,
    Timeout,
    Killed,
    OutOfMemory,
    SecurityViolation,
}

impl RuntimeAuditNode {
    /// Create a new runtime audit node
    pub fn new(
        parent_id: Option<[u8; 32]>,
        execution_context: ExecutionContext,
        operation_type: OperationType,
        binary_outputs: Vec<BinaryOutput>,
    ) -> Result<Self> {
        let timestamp_ns = Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64;
        let sequence_number = Self::generate_sequence_number();
        
        // Create operation data
        let operation_data = OperationData {
            data: bincode::serialize(&operation_type)?,
            data_hash: [0u8; 32], // Will be computed below
            size_bytes: 0, // Will be computed below
            compression: Some(CompressionType::Lz4),
            encryption: None,
        };
        
        let mut node = Self {
            node_id: [0u8; 32], // Will be computed below
            parent_id,
            children: Vec::new(),
            timestamp_ns,
            duration_ns: None,
            sequence_number,
            execution_context,
            runtime_address: RuntimeAddress::default(),
            operation_type,
            operation_data,
            binary_outputs,
            proof_chain: ProofChain::default(),
            integrity_hash: [0u8; 32], // Will be computed below
            signature: vec![0u8; 64], // Will be signed later
            audit_level: AuditLevel::Info,
            compliance_tags: Vec::new(),
            export_metadata: ExportMetadata::default(),
        };
        
        // Compute hashes
        node.compute_hashes()?;
        
        Ok(node)
    }
    
    /// Compute all hashes for the node
    pub fn compute_hashes(&mut self) -> Result<()> {
        // Compute operation data hash
        self.operation_data.data_hash = blake3::hash(&self.operation_data.data).into();
        self.operation_data.size_bytes = self.operation_data.data.len();
        
        // Compute binary output hashes
        for output in &mut self.binary_outputs {
            output.data_hash = blake3::hash(&output.data).into();
            output.size_bytes = output.data.len();
        }
        
        // Compute node integrity hash
        let node_data = bincode::serialize(self)?;
        let integrity_hash = domain_hash("AUDIT_NODE_HASH", &node_data);
        
        // Compute node ID from integrity hash
        self.node_id = integrity_hash;
        self.integrity_hash = integrity_hash;
        
        Ok(())
    }
    
    /// Add a child node
    pub fn add_child(&mut self, child_id: [u8; 32]) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
        }
    }
    
    /// Set completion time and duration
    pub fn set_completion(&mut self, completion_time_ns: u64) {
        if completion_time_ns > self.timestamp_ns {
            self.duration_ns = Some(completion_time_ns - self.timestamp_ns);
        }
    }
    
    /// Generate a sequence number (simplified implementation)
    fn generate_sequence_number() -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};
        static SEQUENCE_COUNTER: AtomicU64 = AtomicU64::new(0);
        SEQUENCE_COUNTER.fetch_add(1, Ordering::SeqCst)
    }
    
    /// Verify node integrity
    pub fn verify_integrity(&self) -> Result<bool> {
        let mut temp_node = self.clone();
        temp_node.compute_hashes()?;
        Ok(temp_node.integrity_hash == self.integrity_hash)
    }
}

impl Default for RuntimeAddress {
    fn default() -> Self {
        Self {
            location_type: LocationType::Container,
            network_address: NetworkAddress {
                ip_address: "127.0.0.1".to_string(),
                port: None,
                protocol: "tcp".to_string(),
                hostname: None,
            },
            process_context: ProcessContext {
                process_id: std::process::id(),
                thread_id: None,
                parent_process_id: None,
                user_id: None,
                group_id: None,
            },
            code_location: None,
        }
    }
}

impl Default for ExportMetadata {
    fn default() -> Self {
        Self {
            priority: ExportPriority::Medium,
            retention_days: 2555, // 7 years
            supported_formats: vec!["json".to_string(), "cbor".to_string()],
            legal_hold: false,
            classification: ClassificationLevel::Internal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_runtime_audit_node_creation() {
        let context = ExecutionContext::DockLock {
            container_id: [1u8; 32],
            workload_id: [2u8; 32],
            cage_config: CageConfig::default(),
        };
        
        let operation = OperationType::ProcessStart {
            command: "test".to_string(),
            args: vec!["arg1".to_string()],
        };
        
        let node = RuntimeAuditNode::new(None, context, operation, vec![]);
        assert!(node.is_ok());
        
        let node = node.unwrap();
        assert_ne!(node.node_id, [0u8; 32]);
        assert_ne!(node.integrity_hash, [0u8; 32]);
    }
    
    #[test]
    fn test_node_integrity_verification() {
        let context = ExecutionContext::DockLock {
            container_id: [1u8; 32],
            workload_id: [2u8; 32],
            cage_config: CageConfig::default(),
        };
        
        let operation = OperationType::ProcessStart {
            command: "test".to_string(),
            args: vec!["arg1".to_string()],
        };
        
        let node = RuntimeAuditNode::new(None, context, operation, vec![]).unwrap();
        assert!(node.verify_integrity().unwrap());
    }
}
