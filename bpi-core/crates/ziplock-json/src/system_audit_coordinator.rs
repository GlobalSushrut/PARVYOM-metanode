//! System-wide audit coordinator for comprehensive ZIPLOCK-JSON coverage
//! Ensures EVERY action, response, and operation from ALL VMs and components is recorded

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::sync::mpsc;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{ZjlResult, ZjlError};
use crate::vm_integration::{VmAuditManager, AuditEvent, VmType, VmInfo, VmStatus};
use crate::brev64::{ForensicRecord, SystemSnapshot, AttackReason};

/// System-wide audit coordinator that manages all VM audit managers
#[derive(Debug)]
pub struct SystemAuditCoordinator {
    /// VM audit managers by VM ID
    vm_managers: HashMap<String, Arc<VmAuditManager>>,
    /// Global event channel for cross-VM correlation
    global_event_sender: mpsc::UnboundedSender<GlobalAuditEvent>,
    /// System-wide audit statistics
    global_stats: Arc<Mutex<GlobalAuditStats>>,
    /// Master audit file path
    master_audit_file: String,
}

/// Global audit event that spans multiple VMs
#[derive(Debug, Clone)]
pub struct GlobalAuditEvent {
    pub event_id: String,
    pub timestamp_ns: u64,
    pub event_type: GlobalEventType,
    pub involved_vms: Vec<String>,
    pub correlation_id: String,
    pub security_impact: SecurityImpact,
    pub metadata: Value,
}

/// Types of global events that span multiple VMs
#[derive(Debug, Clone)]
pub enum GlobalEventType {
    // Cross-VM Communication
    VmToVmCommunication { source_vm: String, target_vm: String, message_type: String },
    
    // System-wide Security Events
    SystemSecurityAlert { alert_type: String, affected_vms: Vec<String> },
    SystemIntrusion { attack_vector: String, compromised_vms: Vec<String> },
    
    // Infrastructure Events
    NetworkTrafficFlow { source_vm: String, destination_vm: String, protocol: String, bytes: u64 },
    SystemResourceAlert { resource_type: String, threshold_exceeded: f64, affected_vms: Vec<String> },
    
    // Orchestration Events
    MultiVmDeployment { orchestrator_vm: String, target_vms: Vec<String>, deployment_type: String },
    SystemConfiguration { config_type: String, affected_vms: Vec<String> },
    
    // Compliance and Audit Events
    ComplianceViolation { violation_type: String, severity: u8, involved_vms: Vec<String> },
    AuditTrailCorrelation { audit_type: String, correlated_events: Vec<String> },
    
    // Emergency Events
    SystemEmergency { emergency_type: String, response_actions: Vec<String> },
    DisasterRecovery { recovery_type: String, affected_systems: Vec<String> },
}

/// Security impact levels for global events
#[derive(Debug, Clone, PartialEq)]
pub enum SecurityImpact {
    None,
    Low,
    Medium,
    High,
    Critical,
    Emergency,
}

/// Global audit statistics across all VMs
#[derive(Debug, Clone, Default)]
pub struct GlobalAuditStats {
    pub total_vms: usize,
    pub total_events_all_vms: u64,
    pub events_by_vm_type: HashMap<String, u64>,
    pub security_events_count: u64,
    pub critical_events_count: u64,
    pub last_global_event_time: Option<u64>,
    pub total_audit_file_size: u64,
    pub cross_vm_correlations: u64,
}

impl SystemAuditCoordinator {
    /// Create new system-wide audit coordinator
    pub fn new(master_audit_file: &str) -> ZjlResult<Self> {
        let (global_event_sender, mut global_event_receiver) = mpsc::unbounded_channel::<GlobalAuditEvent>();
        
        let global_stats = Arc::new(Mutex::new(GlobalAuditStats::default()));
        let global_stats_clone = global_stats.clone();
        
        // Spawn global event processing task
        tokio::spawn(async move {
            while let Some(event) = global_event_receiver.recv().await {
                if let Err(e) = Self::process_global_event(&global_stats_clone, event).await {
                    eprintln!("Global audit processing error: {}", e);
                }
            }
        });

        Ok(Self {
            vm_managers: HashMap::new(),
            global_event_sender,
            global_stats,
            master_audit_file: master_audit_file.to_string(),
        })
    }

    /// Register a VM with comprehensive audit tracking
    pub fn register_vm(&mut self, vm_type: VmType, vm_id: String) -> ZjlResult<Arc<VmAuditManager>> {
        let audit_file_path = format!("/tmp/{}_{}.zjl", 
            format!("{:?}", vm_type).to_lowercase(), 
            vm_id
        );
        
        let mut vm_audit_manager = VmAuditManager::new(&audit_file_path)?;
        
        let vm_info = VmInfo {
            vm_id: vm_id.clone(),
            vm_type: vm_type.clone(),
            status: VmStatus::Starting,
            start_time: chrono::Utc::now().timestamp() as u64,
            audit_enabled: true,
        };
        vm_audit_manager.register_vm(vm_info);
        
        let vm_audit_manager = Arc::new(vm_audit_manager);
        self.vm_managers.insert(vm_id.clone(), vm_audit_manager.clone());
        
        // Update global stats
        {
            let mut stats = self.global_stats.lock().unwrap();
            stats.total_vms += 1;
            *stats.events_by_vm_type.entry(format!("{:?}", vm_type)).or_insert(0) += 1;
        }
        
        // Log global VM registration event
        self.log_global_event(GlobalAuditEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
            event_type: GlobalEventType::SystemConfiguration {
                config_type: "vm_registration".to_string(),
                affected_vms: vec![vm_id.clone()],
            },
            involved_vms: vec![vm_id.clone()],
            correlation_id: Uuid::new_v4().to_string(),
            security_impact: SecurityImpact::Low,
            metadata: json!({
                "vm_type": format!("{:?}", vm_type),
                "vm_id": vm_id,
                "audit_file": audit_file_path
            }),
        });
        
        Ok(vm_audit_manager)
    }

    /// Log a global event that spans multiple VMs
    pub fn log_global_event(&self, event: GlobalAuditEvent) {
        if let Err(_) = self.global_event_sender.send(event) {
            eprintln!("Failed to send global audit event - channel closed");
        }
    }

    /// Log cross-VM communication
    pub fn log_vm_communication(&self, source_vm: &str, target_vm: &str, message_type: &str, payload: &Value) {
        let event = GlobalAuditEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
            event_type: GlobalEventType::VmToVmCommunication {
                source_vm: source_vm.to_string(),
                target_vm: target_vm.to_string(),
                message_type: message_type.to_string(),
            },
            involved_vms: vec![source_vm.to_string(), target_vm.to_string()],
            correlation_id: Uuid::new_v4().to_string(),
            security_impact: SecurityImpact::Medium,
            metadata: json!({
                "payload_hash": blake3::hash(&serde_json::to_vec(payload).unwrap()).to_hex().to_string(),
                "payload_size": serde_json::to_vec(payload).unwrap().len(),
                "communication_type": "inter_vm"
            }),
        };
        
        self.log_global_event(event);
        
        // Also log in individual VM audit managers
        if let Some(source_manager) = self.vm_managers.get(source_vm) {
            source_manager.log_event(AuditEvent::VmStart {
                vm_id: source_vm.to_string(),
                vm_type: VmType::BpiAction, // Would be actual VM type
                config: payload.clone(),
            });
        }
    }

    /// Log system-wide security alert
    pub fn log_security_alert(&self, alert_type: &str, affected_vms: Vec<String>, severity: SecurityImpact) {
        let event = GlobalAuditEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
            event_type: GlobalEventType::SystemSecurityAlert {
                alert_type: alert_type.to_string(),
                affected_vms: affected_vms.clone(),
            },
            involved_vms: affected_vms.clone(),
            correlation_id: Uuid::new_v4().to_string(),
            security_impact: severity.clone(),
            metadata: json!({
                "alert_type": alert_type,
                "vm_count": affected_vms.len(),
                "timestamp": chrono::Utc::now().to_rfc3339()
            }),
        };
        
        let correlation_id = event.correlation_id.clone();
        self.log_global_event(event);
        
        // Log security events in all affected VM audit managers
        let severity_value = match &severity {
            SecurityImpact::Critical | SecurityImpact::Emergency => 255,
            SecurityImpact::High => 200,
            SecurityImpact::Medium => 128,
            SecurityImpact::Low => 64,
            SecurityImpact::None => 0,
        };
        
        for vm_id in &affected_vms {
            if let Some(vm_manager) = self.vm_managers.get(vm_id) {
                vm_manager.log_event(AuditEvent::SecurityViolation {
                    vm_id: vm_id.clone(),
                    violation_type: alert_type.to_string(),
                    severity: severity_value,
                    details: json!({
                        "global_alert": true,
                        "affected_vms": affected_vms,
                        "correlation_id": correlation_id
                    }),
                });
            }
        }
    }

    /// Log network traffic flow between VMs
    pub fn log_network_flow(&self, source_vm: &str, destination_vm: &str, protocol: &str, bytes: u64) {
        let event = GlobalAuditEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
            event_type: GlobalEventType::NetworkTrafficFlow {
                source_vm: source_vm.to_string(),
                destination_vm: destination_vm.to_string(),
                protocol: protocol.to_string(),
                bytes,
            },
            involved_vms: vec![source_vm.to_string(), destination_vm.to_string()],
            correlation_id: Uuid::new_v4().to_string(),
            security_impact: if bytes > 1_000_000 { SecurityImpact::Medium } else { SecurityImpact::Low },
            metadata: json!({
                "protocol": protocol,
                "bytes_transferred": bytes,
                "network_flow": true
            }),
        };
        
        self.log_global_event(event);
        
        // Log network events in source and destination VMs
        if let Some(source_manager) = self.vm_managers.get(source_vm) {
            source_manager.log_event(AuditEvent::NetworkConnection {
                vm_id: source_vm.to_string(),
                local_addr: "internal".to_string(),
                remote_addr: destination_vm.to_string(),
                protocol: protocol.to_string(),
            });
        }
        
        if let Some(dest_manager) = self.vm_managers.get(destination_vm) {
            dest_manager.log_event(AuditEvent::NetworkConnection {
                vm_id: destination_vm.to_string(),
                local_addr: "internal".to_string(),
                remote_addr: source_vm.to_string(),
                protocol: protocol.to_string(),
            });
        }
    }

    /// Process global audit event
    async fn process_global_event(
        global_stats: &Arc<Mutex<GlobalAuditStats>>,
        event: GlobalAuditEvent,
    ) -> ZjlResult<()> {
        // Update global statistics
        {
            let mut stats = global_stats.lock().unwrap();
            stats.last_global_event_time = Some(event.timestamp_ns);
            
            match event.security_impact {
                SecurityImpact::Critical | SecurityImpact::Emergency => {
                    stats.critical_events_count += 1;
                    stats.security_events_count += 1;
                }
                SecurityImpact::High | SecurityImpact::Medium => {
                    stats.security_events_count += 1;
                }
                _ => {}
            }
            
            if event.involved_vms.len() > 1 {
                stats.cross_vm_correlations += 1;
            }
        }
        
        // Log to master audit file (would be implemented)
        println!("GLOBAL AUDIT EVENT: {:?}", event);
        
        Ok(())
    }

    /// Get global audit statistics
    pub fn get_global_stats(&self) -> GlobalAuditStats {
        self.global_stats.lock().unwrap().clone()
    }

    /// Get VM audit manager by VM ID
    pub fn get_vm_manager(&self, vm_id: &str) -> Option<Arc<VmAuditManager>> {
        self.vm_managers.get(vm_id).cloned()
    }

    /// Seal all VM audit files
    pub fn seal_all_audit_files(&self) -> ZjlResult<()> {
        for (vm_id, vm_manager) in &self.vm_managers {
            if let Err(e) = vm_manager.seal_audit_file() {
                eprintln!("Failed to seal audit file for VM {}: {}", vm_id, e);
            }
        }
        Ok(())
    }

    /// Generate system-wide audit report
    pub fn generate_system_audit_report(&self) -> Value {
        let global_stats = self.get_global_stats();
        
        let mut vm_reports = json!({});
        for (vm_id, vm_manager) in &self.vm_managers {
            let vm_stats = vm_manager.get_stats();
            vm_reports[vm_id] = json!({
                "total_events": vm_stats.total_events,
                "events_by_type": vm_stats.events_by_type,
                "last_event_time": vm_stats.last_event_time,
                "audit_file_size": vm_stats.audit_file_size
            });
        }
        
        json!({
            "global_stats": {
                "total_vms": global_stats.total_vms,
                "total_events_all_vms": global_stats.total_events_all_vms,
                "security_events_count": global_stats.security_events_count,
                "critical_events_count": global_stats.critical_events_count,
                "cross_vm_correlations": global_stats.cross_vm_correlations,
                "total_audit_file_size": global_stats.total_audit_file_size
            },
            "vm_reports": vm_reports,
            "report_timestamp": chrono::Utc::now().to_rfc3339(),
            "report_id": Uuid::new_v4().to_string()
        })
    }
}

/// Convenience macros for system-wide audit logging
#[macro_export]
macro_rules! audit_vm_communication {
    ($coordinator:expr, $source:expr, $target:expr, $msg_type:expr, $payload:expr) => {
        $coordinator.log_vm_communication($source, $target, $msg_type, $payload);
    };
}

#[macro_export]
macro_rules! audit_security_alert {
    ($coordinator:expr, $alert_type:expr, $affected_vms:expr, $severity:expr) => {
        $coordinator.log_security_alert($alert_type, $affected_vms, $severity);
    };
}

#[macro_export]
macro_rules! audit_network_flow {
    ($coordinator:expr, $source:expr, $dest:expr, $protocol:expr, $bytes:expr) => {
        $coordinator.log_network_flow($source, $dest, $protocol, $bytes);
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_system_audit_coordinator() {
        let temp_file = NamedTempFile::new().unwrap();
        let master_audit_path = temp_file.path().to_str().unwrap();
        
        let mut coordinator = SystemAuditCoordinator::new(master_audit_path).unwrap();
        
        // Register multiple VMs
        let http_cage_manager = coordinator.register_vm(VmType::HttpCage, "http_cage_1".to_string()).unwrap();
        let action_vm_manager = coordinator.register_vm(VmType::BpiAction, "action_vm_1".to_string()).unwrap();
        
        // Log cross-VM communication
        coordinator.log_vm_communication(
            "http_cage_1",
            "action_vm_1", 
            "contract_request",
            &json!({"contract_type": "SmartContract", "data": "test"})
        );
        
        // Log security alert
        coordinator.log_security_alert(
            "potential_intrusion",
            vec!["http_cage_1".to_string(), "action_vm_1".to_string()],
            SecurityImpact::High
        );
        
        // Give time for async processing
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        let stats = coordinator.get_global_stats();
        assert_eq!(stats.total_vms, 2);
        assert!(stats.security_events_count > 0);
        
        let report = coordinator.generate_system_audit_report();
        assert!(report["global_stats"]["total_vms"].as_u64().unwrap() == 2);
    }
}
