//! VM Integration for comprehensive ZIPLOCK-JSON audit coverage
//! Ensures every action, response, and operation from all VMs is recorded

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use tokio::sync::mpsc;
use uuid::Uuid;
use std::fs::OpenOptions;
use std::io::Write;

use crate::{ZjlResult, ZjlError};
use crate::writer::ZjlWriter;
use crate::reader::ZjlReader;
use crate::signing::{ZjlSigner, InMemoryKms, KmsProvider};

use crate::merkle::MicroReceipt;
use crate::brev64::{ForensicRecord, SystemSnapshot, AttackReason, EvidenceType, EvidenceEntry};

/// Comprehensive VM audit manager
#[derive(Debug)]
pub struct VmAuditManager {
    /// ZJL writer for audit file
    writer: Arc<Mutex<ZjlWriter<std::fs::File, InMemoryKms>>>,
    /// Event channel for async audit logging
    event_sender: mpsc::UnboundedSender<AuditEvent>,
    /// VM registry
    vm_registry: HashMap<String, VmInfo>,
    /// Audit statistics
    stats: Arc<Mutex<AuditStats>>,
}

/// VM information
#[derive(Debug, Clone)]
pub struct VmInfo {
    pub vm_id: String,
    pub vm_type: VmType,
    pub status: VmStatus,
    pub start_time: u64,
    pub audit_enabled: bool,
}

/// VM types in the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VmType {
    HttpCage,
    BpiAction,
    Forensic,
    UniversalAudit,
    Orchestration,
    CueOrchestration,
    DockLock,
    EncCluster,
    TrafficLight,
    Firewall,
    SystemComponent,
    Pipeline,
    Terraform,
    Nginx,
}

/// VM status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VmStatus {
    Starting,
    Running,
    Paused,
    Stopping,
    Stopped,
    Error,
}

/// Comprehensive audit event types
#[derive(Debug, Clone)]
pub enum AuditEvent {
    // VM Lifecycle Events
    VmStart { vm_id: String, vm_type: VmType, config: Value },
    VmStop { vm_id: String, reason: String },
    VmError { vm_id: String, error: String, stack_trace: Option<String> },
    VmRegistered { vm_id: String, vm_type: VmType, integrity_profile: String },
    
    // HTTP Cage Events
    HttpRequest { vm_id: String, method: String, url: String, headers: Value, body: Option<String> },
    HttpResponse { vm_id: String, status: u16, headers: Value, body: Option<String>, duration_ms: u64 },
    HttpError { vm_id: String, error: String, request_id: String },
    
    // BPI Action VM Events
    ContractDeploy { vm_id: String, contract_type: String, contract_id: String, config: Value },
    ContractExecution { vm_id: String, contract_id: String, action: String, params: Value, result: Value },
    ContractError { vm_id: String, contract_id: String, error: String },
    
    // CUE Orchestration Events
    CueValidation { vm_id: String, cue_file: String, validation_result: bool, errors: Vec<String> },
    CueExecution { vm_id: String, cue_config: Value, execution_result: Value },
    
    // DockLock Events
    ContainerCreate { vm_id: String, container_id: String, image: String, config: Value },
    ContainerStart { vm_id: String, container_id: String },
    ContainerStop { vm_id: String, container_id: String, exit_code: Option<i32> },
    ContainerExec { vm_id: String, container_id: String, command: String, result: Value },
    
    // ENC Cluster Events
    EncOperation { vm_id: String, operation: String, data_hash: String, result: Value },
    EncValidation { vm_id: String, validation_type: String, result: bool, details: Value },
    
    // Traffic Light Events
    TrafficAllow { vm_id: String, source: String, destination: String, protocol: String, port: u16 },
    TrafficBlock { vm_id: String, source: String, destination: String, reason: String },
    TrafficShape { vm_id: String, rule: String, bandwidth_limit: u64 },
    
    // Firewall Events
    FirewallRule { vm_id: String, rule_id: String, action: String, source: String, destination: String },
    FirewallBlock { vm_id: String, source_ip: String, reason: String, threat_level: u8 },
    FirewallAlert { vm_id: String, alert_type: String, details: Value },
    
    // Pipeline Events
    PipelineStart { vm_id: String, pipeline_id: String, trigger: String, config: Value },
    PipelineStep { vm_id: String, pipeline_id: String, step: String, status: String, output: Value },
    PipelineComplete { vm_id: String, pipeline_id: String, status: String, duration_ms: u64 },
    
    // Terraform Events
    TerraformPlan { vm_id: String, plan_id: String, resources: Value, changes: Value },
    TerraformApply { vm_id: String, plan_id: String, result: Value, resources_created: u32 },
    TerraformDestroy { vm_id: String, resources_destroyed: u32 },
    
    // Nginx Events
    NginxConfig { vm_id: String, config_hash: String, config: Value },
    NginxReload { vm_id: String, success: bool, error: Option<String> },
    NginxRequest { vm_id: String, client_ip: String, method: String, uri: String, status: u16 },
    
    // Security Events
    SecurityEvent { vm_id: String, event_type: String, severity: u8, details: Value },
    SecurityViolation { vm_id: String, violation_type: String, severity: u8, details: Value },
    IntrusionAttempt { vm_id: String, source_ip: String, attack_type: String, blocked: bool },
    
    // Network Events
    NetworkConnection { vm_id: String, local_addr: String, remote_addr: String, protocol: String },
    NetworkDisconnection { vm_id: String, connection_id: String, reason: String },
    NetworkTraffic { vm_id: String, bytes_in: u64, bytes_out: u64, connections: u32 },
    
    // System Events
    SystemMetrics { vm_id: String, cpu_percent: f64, memory_bytes: u64, disk_bytes: u64 },
    SystemAlert { vm_id: String, alert_type: String, threshold: f64, current_value: f64 },
    
    // Forensic Events
    ForensicCapture { vm_id: String, evidence_type: String, evidence_hash: String, size_bytes: u64 },
    AttackDetected { vm_id: String, attack_type: AttackReason, confidence: f64, evidence: Vec<String> },
    
    // Bundle and Transaction Events
    BundleCommitted { bundle_id: String, transaction_count: u32, size_bytes: u64, integrity_hash: String },
    BundleValidated { bundle_id: String, validation_result: bool, validator_id: String },
    
    // Additional Security Events
    SecurityAlert { vm_id: String, alert_type: String, severity: u8, details: Value },
}

/// Audit statistics
#[derive(Debug, Clone, Default)]
pub struct AuditStats {
    pub total_events: u64,
    pub events_by_vm: HashMap<String, u64>,
    pub events_by_type: HashMap<String, u64>,
    pub last_event_time: Option<u64>,
    pub audit_file_size: u64,
    pub compression_ratio: f64,
}

impl VmAuditManager {
    /// Create new VM audit manager
    pub fn new(audit_file_path: &str) -> ZjlResult<Self> {
        // Create ZJL writer with signing
        let mut kms = InMemoryKms::new();
        let public_key = kms.generate_key("vm_audit_key")?;
        let signer = ZjlSigner::new(kms, "vm_audit_key".to_string());
        
        let options = crate::ZjlOptions {
            compression_level: 6, // Higher compression for audit logs
            enable_encryption: false, // Can be enabled for sensitive environments
            max_file_size: 1024 * 1024 * 1024, // 1GB
            chunk_size: 64 * 1024, // 64KB chunks for VM events
            enable_signatures: true,
            kms_endpoint: None,
            enable_forensic_mode: true,
            enable_merkle_proofs: true,
            retention_days: 3650, // 10 years
            enforce_i_json: true,
            enable_rollups: true,
            enable_brev64: true,
        };

        let writer = crate::writer::create_signed_zjl_file(audit_file_path, options, signer)?;
        let writer = Arc::new(Mutex::new(writer));

        // Create event channel
        let (event_sender, mut event_receiver) = mpsc::unbounded_channel::<AuditEvent>();

        let writer_clone = writer.clone();
        let stats = Arc::new(Mutex::new(AuditStats::default()));
        let stats_clone = stats.clone();

        // Spawn audit processing task
        tokio::spawn(async move {
            while let Some(event) = event_receiver.recv().await {
                if let Err(e) = Self::process_audit_event(&writer_clone, &stats_clone, event).await {
                    eprintln!("Audit processing error: {}", e);
                }
            }
        });

        Ok(Self {
            writer,
            event_sender,
            vm_registry: HashMap::new(),
            stats,
        })
    }

    /// Register a VM for audit tracking
    pub fn register_vm(&mut self, vm_info: VmInfo) {
        let vm_id = vm_info.vm_id.clone();
        self.vm_registry.insert(vm_id.clone(), vm_info.clone());
        
        // Record VM registration
        let event = AuditEvent::VmStart {
            vm_id,
            vm_type: vm_info.vm_type,
            config: json!({
                "status": format!("{:?}", vm_info.status),
                "start_time": vm_info.start_time,
                "audit_enabled": vm_info.audit_enabled
            }),
        };
        
        let _ = self.event_sender.send(event);
    }

    /// Log audit event (non-blocking)
    pub fn log_event(&self, event: AuditEvent) {
        if let Err(_) = self.event_sender.send(event) {
            eprintln!("Failed to send audit event - channel closed");
        }
    }

    /// Process audit event and write to ZJL file
    async fn process_audit_event(
        writer: &Arc<Mutex<ZjlWriter<std::fs::File, InMemoryKms>>>,
        stats: &Arc<Mutex<AuditStats>>,
        event: AuditEvent,
    ) -> ZjlResult<()> {
        let event_json = Self::event_to_json(&event);
        let vm_id = Self::extract_vm_id(&event);
        let event_type = Self::event_type_name(&event);

        // Write to ZJL file
        {
            let mut writer_guard = writer.lock().unwrap();
            
            // Write as JSON audit event
            writer_guard.write_json_with_path(&event_json, &format!("vm_events/{}", vm_id))?;
            
            // Create micro-receipt for audit trail
            let payload = serde_json::to_vec(&event_json).unwrap();
            writer_guard.write_audit_event(event_type.clone(), vm_id.clone(), &payload)?;
            
            // Check for security events that need forensic records
            if let Some(forensic_record) = Self::create_forensic_record(&event) {
                writer_guard.write_forensic_record(&forensic_record)?;
            }
        }

        // Update statistics
        {
            let mut stats_guard = stats.lock().unwrap();
            stats_guard.total_events += 1;
            *stats_guard.events_by_vm.entry(vm_id).or_insert(0) += 1;
            *stats_guard.events_by_type.entry(event_type).or_insert(0) += 1;
            stats_guard.last_event_time = Some(chrono::Utc::now().timestamp() as u64);
        }

        Ok(())
    }

    /// Convert audit event to JSON
    fn event_to_json(event: &AuditEvent) -> Value {
        let timestamp = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64;
        let event_id = Uuid::new_v4().to_string();

        match event {
            AuditEvent::VmStart { vm_id, vm_type, config } => json!({
                "event_id": event_id,
                "timestamp_ns": timestamp,
                "event_type": "vm_start",
                "vm_id": vm_id,
                "vm_type": format!("{:?}", vm_type),
                "config": config
            }),
            
            AuditEvent::HttpRequest { vm_id, method, url, headers, body } => json!({
                "event_id": event_id,
                "timestamp_ns": timestamp,
                "event_type": "http_request",
                "vm_id": vm_id,
                "method": method,
                "url": url,
                "headers": headers,
                "body": body,
                "body_hash": body.as_ref().map(|b| blake3::hash(b.as_bytes()).to_hex().to_string())
            }),
            
            AuditEvent::ContractDeploy { vm_id, contract_type, contract_id, config } => json!({
                "event_id": event_id,
                "timestamp_ns": timestamp,
                "event_type": "contract_deploy",
                "vm_id": vm_id,
                "contract_type": contract_type,
                "contract_id": contract_id,
                "config": config,
                "config_hash": blake3::hash(&serde_json::to_vec(config).unwrap()).to_hex().to_string()
            }),
            
            AuditEvent::TrafficBlock { vm_id, source, destination, reason } => json!({
                "event_id": event_id,
                "timestamp_ns": timestamp,
                "event_type": "traffic_block",
                "vm_id": vm_id,
                "source": source,
                "destination": destination,
                "reason": reason,
                "security_impact": "medium"
            }),
            
            AuditEvent::FirewallBlock { vm_id, source_ip, reason, threat_level } => json!({
                "event_id": event_id,
                "timestamp_ns": timestamp,
                "event_type": "firewall_block",
                "vm_id": vm_id,
                "source_ip": source_ip,
                "reason": reason,
                "threat_level": threat_level,
                "security_impact": if *threat_level > 7 { "critical" } else if *threat_level > 4 { "high" } else { "medium" }
            }),
            
            AuditEvent::AttackDetected { vm_id, attack_type, confidence, evidence } => json!({
                "event_id": event_id,
                "timestamp_ns": timestamp,
                "event_type": "attack_detected",
                "vm_id": vm_id,
                "attack_type": format!("{:?}", attack_type),
                "confidence": confidence,
                "evidence_count": evidence.len(),
                "evidence_hashes": evidence,
                "security_impact": "critical"
            }),
            
            // Add more event types as needed...
            _ => json!({
                "event_id": event_id,
                "timestamp_ns": timestamp,
                "event_type": "generic_event",
                "vm_id": Self::extract_vm_id(event),
                "raw_event": format!("{:?}", event)
            })
        }
    }

    /// Extract VM ID from event
    fn extract_vm_id(event: &AuditEvent) -> String {
        match event {
            AuditEvent::VmStart { vm_id, .. } |
            AuditEvent::VmStop { vm_id, .. } |
            AuditEvent::VmError { vm_id, .. } |
            AuditEvent::HttpRequest { vm_id, .. } |
            AuditEvent::HttpResponse { vm_id, .. } |
            AuditEvent::ContractDeploy { vm_id, .. } |
            AuditEvent::TrafficBlock { vm_id, .. } |
            AuditEvent::FirewallBlock { vm_id, .. } |
            AuditEvent::AttackDetected { vm_id, .. } => vm_id.clone(),
            // Handle all other event types with default VM ID
            _ => "unknown_vm".to_string(),
        }
    }

    /// Get event type name
    fn event_type_name(event: &AuditEvent) -> String {
        match event {
            AuditEvent::VmStart { .. } => "vm_start".to_string(),
            AuditEvent::VmStop { .. } => "vm_stop".to_string(),
            AuditEvent::HttpRequest { .. } => "http_request".to_string(),
            AuditEvent::HttpResponse { .. } => "http_response".to_string(),
            AuditEvent::ContractDeploy { .. } => "contract_deploy".to_string(),
            AuditEvent::TrafficBlock { .. } => "traffic_block".to_string(),
            AuditEvent::FirewallBlock { .. } => "firewall_block".to_string(),
            AuditEvent::AttackDetected { .. } => "attack_detected".to_string(),
            _ => "generic_event".to_string(),
        }
    }

    /// Create forensic record for security events
    fn create_forensic_record(event: &AuditEvent) -> Option<ForensicRecord> {
        match event {
            AuditEvent::AttackDetected { vm_id, attack_type, confidence, evidence } => {
                let mut forensic_record = ForensicRecord {
                    record_id: *Uuid::new_v4().as_bytes(),
                    reason: *attack_type,
                    timestamp_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
                    vm_id: vm_id.clone(),
                    process_id: None,
                    thread_id: None,
                    memory_address: None,
                    severity: (confidence * 255.0) as u8,
                    evidence: Vec::new(),
                    vector: format!("Attack detected by VM: {}", vm_id),
                    mitigation: "Automated blocking applied".to_string(),
                    attributes: HashMap::new(),
                };

                // Add evidence entries
                for evidence_hash in evidence {
                    let evidence_entry = EvidenceEntry::new(
                        EvidenceType::LogEntry,
                        evidence_hash.as_bytes().to_vec(),
                        format!("Attack evidence: {}", evidence_hash),
                    );
                    forensic_record.evidence.push(evidence_entry);
                }

                Some(forensic_record)
            }
            
            AuditEvent::FirewallBlock { vm_id, source_ip, reason, threat_level } => {
                if *threat_level > 5 { // Only create forensic records for significant threats
                    Some(ForensicRecord {
                        record_id: *Uuid::new_v4().as_bytes(),
                        reason: AttackReason::Unknown, // Map to appropriate attack reason
                        timestamp_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
                        vm_id: vm_id.clone(),
                        process_id: None,
                        thread_id: None,
                        memory_address: None,
                        severity: *threat_level * 32, // Scale threat level
                        evidence: vec![
                            EvidenceEntry::new(
                                EvidenceType::NetworkCapture,
                                format!("Source IP: {}, Reason: {}", source_ip, reason).as_bytes().to_vec(),
                                "Firewall block evidence".to_string(),
                            )
                        ],
                        vector: format!("Network attack from {}", source_ip),
                        mitigation: "Traffic blocked by firewall".to_string(),
                        attributes: HashMap::new(),
                    })
                } else {
                    None
                }
            }
            
            _ => None
        }
    }

    /// Get audit statistics
    pub fn get_stats(&self) -> AuditStats {
        self.stats.lock().unwrap().clone()
    }

    /// Seal the audit file
    pub fn seal_audit_file(&self) -> ZjlResult<()> {
        let mut writer_guard = self.writer.lock().unwrap();
        writer_guard.seal()
    }

    /// Get VM registry
    pub fn get_vm_registry(&self) -> HashMap<String, VmInfo> {
        self.vm_registry.clone()
    }

    /// Generate human-readable audit report from ZJL file
    pub fn generate_readable_report(&self, output_path: &str) -> ZjlResult<()> {
        // Get the ZJL file path from the writer
        let zjl_path = {
            let writer_guard = self.writer.lock().unwrap();
            writer_guard.get_file_path().unwrap_or_else(|| "audit.zjl".to_string())
        };

        // Create ZJL reader and generate report
        let mut reader = ZjlReader::open(&zjl_path)?;
        reader.export_text(&zjl_path, &output_path.to_string())?;
        
        println!("✅ Human-readable audit report generated: {}", output_path);
        Ok(())
    }

    /// Generate JSON audit report from ZJL file
    pub fn generate_json_report(&self, output_path: &str) -> ZjlResult<()> {
        let zjl_path = {
            let writer_guard = self.writer.lock().unwrap();
            writer_guard.get_file_path().unwrap_or_else(|| "audit.zjl".to_string())
        };

        let mut reader = ZjlReader::open(&zjl_path)?;
        reader.export_json(&zjl_path, &output_path.to_string())?;
        
        println!("✅ JSON audit report generated: {}", output_path);
        Ok(())
    }

    /// Enable parallel human-readable logging alongside binary ZJL
    pub fn enable_readable_logging(&self, log_file_path: &str) -> ZjlResult<()> {
        // Create or append to human-readable log file
        let mut log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)
            .map_err(|e| ZjlError::IoError(format!("Failed to open readable log: {}", e)))?;

        // Write header if file is new
        if log_file.metadata().unwrap().len() == 0 {
            writeln!(log_file, "=====================================")
                .map_err(|e| ZjlError::IoError(format!("Write error: {}", e)))?;
            writeln!(log_file, "BPI VM AUDIT LOG - HUMAN READABLE")
                .map_err(|e| ZjlError::IoError(format!("Write error: {}", e)))?;
            writeln!(log_file, "=====================================")
                .map_err(|e| ZjlError::IoError(format!("Write error: {}", e)))?;
            writeln!(log_file, "Started: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"))
                .map_err(|e| ZjlError::IoError(format!("Write error: {}", e)))?;
            writeln!(log_file, "")
                .map_err(|e| ZjlError::IoError(format!("Write error: {}", e)))?;
        }

        println!("✅ Human-readable logging enabled: {}", log_file_path);
        Ok(())
    }

    /// Log event in human-readable format (parallel to binary)
    pub fn log_readable_event(&self, event: &AuditEvent, log_file_path: &str) -> ZjlResult<()> {
        let mut log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)
            .map_err(|e| ZjlError::IoError(format!("Failed to open readable log: {}", e)))?;

        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
        let event_type = Self::event_type_name(event);
        let vm_id = Self::extract_vm_id(event);

        // Write human-readable event
        writeln!(log_file, "[{}] {} - {}", timestamp, event_type, vm_id)
            .map_err(|e| ZjlError::IoError(format!("Write error: {}", e)))?;

        // Write event details in readable format
        match event {
            AuditEvent::VmStart { vm_type, config, .. } => {
                writeln!(log_file, "  VM Type: {:?}", vm_type)?;
                writeln!(log_file, "  Config: {}", serde_json::to_string_pretty(config).unwrap_or_default())?;
            }
            AuditEvent::HttpRequest { method, url, headers, body, .. } => {
                writeln!(log_file, "  Method: {}", method)?;
                writeln!(log_file, "  URL: {}", url)?;
                writeln!(log_file, "  Headers: {}", serde_json::to_string_pretty(headers).unwrap_or_default())?;
                if let Some(body) = body {
                    writeln!(log_file, "  Body: {}", body)?;
                }
            }
            AuditEvent::ContractDeploy { contract_type, contract_id, config, .. } => {
                writeln!(log_file, "  Contract Type: {}", contract_type)?;
                writeln!(log_file, "  Contract ID: {}", contract_id)?;
                writeln!(log_file, "  Config: {}", serde_json::to_string_pretty(config).unwrap_or_default())?;
            }
            AuditEvent::SecurityEvent { event_type, severity, details, .. } => {
                writeln!(log_file, "  Security Event: {}", event_type)?;
                writeln!(log_file, "  Severity: {}", severity)?;
                writeln!(log_file, "  Details: {}", serde_json::to_string_pretty(details).unwrap_or_default())?;
            }
            AuditEvent::AttackDetected { attack_type, confidence, evidence, .. } => {
                writeln!(log_file, "  Attack Type: {:?}", attack_type)?;
                writeln!(log_file, "  Confidence: {}", confidence)?;
                writeln!(log_file, "  Evidence Count: {}", evidence.len())?;
            }
            _ => {
                writeln!(log_file, "  Details: {}", serde_json::to_string_pretty(&Self::event_to_json(event)).unwrap_or_default())?;
            }
        }

        writeln!(log_file, "---")?;
        Ok(())
    }
}

/// Convenience macros for VM audit logging
#[macro_export]
macro_rules! audit_vm_start {
    ($manager:expr, $vm_id:expr, $vm_type:expr, $config:expr) => {
        $manager.log_event(AuditEvent::VmStart {
            vm_id: $vm_id.to_string(),
            vm_type: $vm_type,
            config: $config,
        });
    };
}

#[macro_export]
macro_rules! audit_http_request {
    ($manager:expr, $vm_id:expr, $method:expr, $url:expr, $headers:expr, $body:expr) => {
        $manager.log_event(AuditEvent::HttpRequest {
            vm_id: $vm_id.to_string(),
            method: $method.to_string(),
            url: $url.to_string(),
            headers: $headers,
            body: $body.map(|b| b.to_string()),
        });
    };
}

#[macro_export]
macro_rules! audit_contract_deploy {
    ($manager:expr, $vm_id:expr, $contract_type:expr, $contract_id:expr, $config:expr) => {
        $manager.log_event(AuditEvent::ContractDeploy {
            vm_id: $vm_id.to_string(),
            contract_type: $contract_type.to_string(),
            contract_id: $contract_id.to_string(),
            config: $config,
        });
    };
}

#[macro_export]
macro_rules! audit_traffic_block {
    ($manager:expr, $vm_id:expr, $source:expr, $destination:expr, $reason:expr) => {
        $manager.log_event(AuditEvent::TrafficBlock {
            vm_id: $vm_id.to_string(),
            source: $source.to_string(),
            destination: $destination.to_string(),
            reason: $reason.to_string(),
        });
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_vm_audit_manager() {
        let temp_file = NamedTempFile::new().unwrap();
        let audit_path = temp_file.path().to_str().unwrap();
        
        let mut manager = VmAuditManager::new(audit_path).unwrap();
        
        // Register a VM
        let vm_info = VmInfo {
            vm_id: "test_vm".to_string(),
            vm_type: VmType::HttpCage,
            status: VmStatus::Running,
            start_time: chrono::Utc::now().timestamp() as u64,
            audit_enabled: true,
        };
        manager.register_vm(vm_info);
        
        // Log some events
        manager.log_event(AuditEvent::HttpRequest {
            vm_id: "test_vm".to_string(),
            method: "GET".to_string(),
            url: "/api/test".to_string(),
            headers: json!({"user-agent": "test"}),
            body: None,
        });
        
        manager.log_event(AuditEvent::AttackDetected {
            vm_id: "test_vm".to_string(),
            attack_type: AttackReason::SqlInjection,
            confidence: 0.95,
            evidence: vec!["evidence_hash_1".to_string()],
        });
        
        // Give some time for async processing
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        let stats = manager.get_stats();
        assert!(stats.total_events > 0);
        assert!(stats.events_by_vm.contains_key("test_vm"));
    }
}
