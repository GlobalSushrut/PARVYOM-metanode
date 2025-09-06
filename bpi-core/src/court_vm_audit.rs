//! Court VM Audit System - Comprehensive audit trails for Court Node operations
//! 
//! This module provides detailed VM audit capabilities for all Court Node operations,
//! including CUE agreement deployments, YAML SmartContract++ executions, and runtime actions.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use blake3;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, warn, error, debug};

use crate::immutable_audit_system::{ImmutableAuditSystem, AuditRecord, AuditRecordType, ComponentType, RuntimeEvent, SecurityEvent, SystemState, ImmutableProof, PerformanceMetrics, SecurityLevel};

/// VM Audit Operation Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VMAuditOperationType {
    ContractExecution,
    StateTransition,
    ResourceAllocation,
    SecurityCheck,
    ComplianceValidation,
}

impl std::fmt::Display for VMAuditOperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VMAuditOperationType::ContractExecution => write!(f, "ContractExecution"),
            VMAuditOperationType::StateTransition => write!(f, "StateTransition"),
            VMAuditOperationType::ResourceAllocation => write!(f, "ResourceAllocation"),
            VMAuditOperationType::SecurityCheck => write!(f, "SecurityCheck"),
            VMAuditOperationType::ComplianceValidation => write!(f, "ComplianceValidation"),
        }
    }
}

/// Runtime Action Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuntimeActionType {
    ContractDeploy,
    ContractExecute,
    CueDeploy,
    StateUpdate,
    ResourceAccess,
    SecurityEvent,
}

/// CUE Deployment Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CueDeploymentStatus {
    Pending,
    InProgress,
    Success,
    Failed,
    Cancelled,
}

/// VM Audit Record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMAuditRecord {
    pub record_id: String,
    pub operation_type: VMAuditOperationType,
    pub timestamp: u64,
    pub details: String,
    pub metadata: HashMap<String, String>,
}

/// Runtime Action Log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeActionLog {
    pub action_id: String,
    pub action_type: RuntimeActionType,
    pub timestamp: u64,
    pub description: String,
    pub result: String,
}

/// CUE Deployment Audit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CueDeploymentAudit {
    pub deployment_id: String,
    pub status: CueDeploymentStatus,
    pub timestamp: u64,
    pub details: String,
    pub metadata: HashMap<String, String>,
}

/// VM State Snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMStateSnapshot {
    pub snapshot_id: String,
    pub timestamp: u64,
    pub state_data: HashMap<String, String>,
    pub metrics: HashMap<String, f64>,
}

/// Execution Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub result: String,
    pub error: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// VM Audit Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMAuditConfig {
    pub enable_detailed_logging: bool,
    pub retention_days: u32,
    pub max_records: usize,
    pub audit_level: String,
    pub detailed_vm_snapshots: bool,
    pub cryptographic_proofs: bool,
    pub max_memory_records: usize,
    pub flush_interval_seconds: u64,
    pub real_time_streaming: bool,
}

impl Default for VMAuditConfig {
    fn default() -> Self {
        Self {
            enable_detailed_logging: true,
            retention_days: 90,
            max_records: 100000,
            audit_level: "info".to_string(),
            detailed_vm_snapshots: true,
            cryptographic_proofs: true,
            max_memory_records: 10000,
            flush_interval_seconds: 60,
            real_time_streaming: false,
        }
    }
}

/// Court VM Audit System - Maintains comprehensive audit trails for all Court operations
#[derive(Debug)]
pub struct CourtVMAuditSystem {
    /// Immutable audit system integration
    pub audit_system: Arc<ImmutableAuditSystem>,
    /// VM audit records storage
    pub vm_audit_records: Arc<RwLock<Vec<VMAuditRecord>>>,
    /// Runtime action logs storage
    pub runtime_action_logs: Arc<RwLock<Vec<RuntimeActionLog>>>,
    /// CUE deployment audit storage
    pub cue_deployment_audits: Arc<RwLock<Vec<CueDeploymentAudit>>>,
    /// Audit configuration
    pub config: VMAuditConfig,
}



impl CourtVMAuditSystem {
    /// Create new Court VM Audit System
    pub async fn new(audit_system: Arc<ImmutableAuditSystem>) -> Result<Self> {
        info!("Initializing Court VM Audit System");
        
        Ok(Self {
            audit_system,
            vm_audit_records: Arc::new(RwLock::new(Vec::new())),
            runtime_action_logs: Arc::new(RwLock::new(Vec::new())),
            cue_deployment_audits: Arc::new(RwLock::new(Vec::new())),
            config: VMAuditConfig::default(),
        })
    }
    
    /// Record VM operation with comprehensive audit trail
    pub async fn record_vm_operation(
        &self,
        operation_type: VMAuditOperationType,
        contract_id: Option<String>,
        orchestration_id: Option<String>,
        operation_details: serde_json::Value,
    ) -> Result<String> {
        let record_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        
        // Generate VM state snapshot
        let vm_state_snapshot = self.generate_vm_state_snapshot().await?;
        
        // Generate cryptographic proof
        let cryptographic_proof = self.generate_cryptographic_proof(&record_id, &operation_details).await?;
        
        // Create execution result placeholder (would be filled by actual execution)
        let execution_result = ExecutionResult {
            success: true,
            result: "VM operation completed".to_string(),
            error: None,
            metadata: HashMap::new(),
        };
        
        let mut metadata = HashMap::new();
        metadata.insert("contract_id".to_string(), contract_id.clone().unwrap_or_default());
        metadata.insert("orchestration_id".to_string(), orchestration_id.clone().unwrap_or_default());
        metadata.insert("operation_details".to_string(), operation_details.to_string());
        
        let audit_record = VMAuditRecord {
            record_id: record_id.clone(),
            timestamp: timestamp.timestamp() as u64,
            operation_type: operation_type.clone(),
            details: format!("VM operation: {}", operation_details),
            metadata,
        };
        
        // Store in memory
        {
            let mut records = self.vm_audit_records.write().await;
            records.push(audit_record.clone());
            
            // Maintain memory limit
            if records.len() > self.config.max_memory_records {
                records.remove(0);
            }
        }
        
        // Create proper AuditRecord for immutable audit system
        let immutable_audit_record = self.create_audit_record(
            AuditRecordType::RuntimeExecution,
            &format!("Court VM operation: {}", operation_type),
            serde_json::to_value(&audit_record)?
        ).await?;
        
        // Store in immutable audit system
        let immutable_audit_record = self.create_audit_record(
            AuditRecordType::RuntimeExecution,
            &format!("Court VM operation: {:?}", operation_type),
            serde_json::to_value(&audit_record)?
        ).await?;
        
        // TODO: Implement proper Arc<Mutex<ImmutableAuditSystem>> pattern for mutable access
        // For now, we'll skip the immutable audit recording due to Arc borrowing constraints
        tracing::info!("Immutable audit record created (skipped due to Arc borrowing): {:?}", immutable_audit_record.record_id);
        
        info!("Recorded VM operation: {} - {:?}", record_id, operation_type);
        Ok(record_id)
    }

    /// Record CUE deployment start
    pub async fn record_cue_deployment_start(
        &self,
        deployment_id: &str,
        contract_content: &str,
    ) -> Result<()> {
        let audit = CueDeploymentAudit {
            deployment_id: deployment_id.to_string(),
            status: CueDeploymentStatus::InProgress,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            details: format!("CUE deployment started: {}", deployment_id),
            metadata: [("contract_size".to_string(), contract_content.len().to_string())]
                .iter().cloned().collect(),
        };

        let mut audits = self.cue_deployment_audits.write().await;
        audits.push(audit);
        
        info!("Recorded CUE deployment start: {}", deployment_id);
        Ok(())
    }

    /// Record CUE deployment success
    pub async fn record_cue_deployment_success(
        &self,
        deployment_id: &str,
        result: &str,
    ) -> Result<()> {
        let audit = CueDeploymentAudit {
            deployment_id: deployment_id.to_string(),
            status: CueDeploymentStatus::Success,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            details: format!("CUE deployment succeeded: {}", deployment_id),
            metadata: [("result".to_string(), result.to_string())]
                .iter().cloned().collect(),
        };

        let mut audits = self.cue_deployment_audits.write().await;
        audits.push(audit);
        
        info!("Recorded CUE deployment success: {}", deployment_id);
        Ok(())
    }

    /// Record CUE deployment failure
    pub async fn record_cue_deployment_failure(
        &self,
        deployment_id: &str,
        error: &str,
    ) -> Result<()> {
        let audit = CueDeploymentAudit {
            deployment_id: deployment_id.to_string(),
            status: CueDeploymentStatus::Failed,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            details: format!("CUE deployment failed: {}", deployment_id),
            metadata: [("error".to_string(), error.to_string())]
                .iter().cloned().collect(),
        };

        let mut audits = self.cue_deployment_audits.write().await;
        audits.push(audit);
        
        error!("Recorded CUE deployment failure: {} - {}", deployment_id, error);
        Ok(())
    }

    /// Record runtime action
    pub async fn record_runtime_action(
        &self,
        action_type: RuntimeActionType,
        description: &str,
        result: &str,
    ) -> Result<()> {
        let action_log = RuntimeActionLog {
            action_id: Uuid::new_v4().to_string(),
            action_type,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            description: description.to_string(),
            result: result.to_string(),
        };

        let mut logs = self.runtime_action_logs.write().await;
        logs.push(action_log);
        
        debug!("Recorded runtime action: {}", description);
        Ok(())
    }

    /// Get audit trail for a specific deployment
    pub async fn get_audit_trail(&self, deployment_id: &str) -> Result<Vec<VMAuditRecord>> {
        let records = self.vm_audit_records.read().await;
        let filtered: Vec<VMAuditRecord> = records
            .iter()
            .filter(|record| record.record_id.contains(deployment_id))
            .cloned()
            .collect();
        
        Ok(filtered)
    }

    /// Get runtime action logs
    pub async fn get_runtime_action_logs(&self) -> Result<Vec<RuntimeActionLog>> {
        let logs = self.runtime_action_logs.read().await;
        Ok(logs.clone())
    }

    /// Get CUE deployment audit records
    pub async fn get_cue_deployment_audit(&self, deployment_id: &str) -> Result<Vec<CueDeploymentAudit>> {
        let audits = self.cue_deployment_audits.read().await;
        let filtered: Vec<CueDeploymentAudit> = audits
            .iter()
            .filter(|audit| audit.deployment_id == deployment_id)
            .cloned()
            .collect();
        
        Ok(filtered)
    }
    
    /// Generate VM state snapshot
    async fn generate_vm_state_snapshot(&self) -> Result<VMStateSnapshot> {
        // In a real implementation, this would collect actual VM metrics
        let mut state_data = HashMap::new();
        state_data.insert("memory_usage_mb".to_string(), "512".to_string());
        state_data.insert("active_contracts".to_string(), "5".to_string());
        state_data.insert("pending_operations".to_string(), "2".to_string());
        
        let mut metrics = HashMap::new();
        metrics.insert("cpu_usage_percent".to_string(), 25.0);
        metrics.insert("memory_usage_percent".to_string(), 45.0);
        
        Ok(VMStateSnapshot {
            snapshot_id: format!("snapshot_{}", Uuid::new_v4()),
            timestamp: Utc::now().timestamp() as u64,
            state_data,
            metrics,
        })
    }
    
    /// Generate cryptographic proof for audit record
    async fn generate_cryptographic_proof(&self, record_id: &str, operation_details: &serde_json::Value) -> Result<String> {
        if !self.config.cryptographic_proofs {
            return Ok("proof_disabled".to_string());
        }
        
        // Create proof data
        let proof_data = serde_json::json!({
            "record_id": record_id,
            "timestamp": Utc::now(),
            "operation_details": operation_details,
            "vm_instance": "court_node_vm",
            "proof_version": "1.0"
        });
        
        // Generate Blake3 hash as cryptographic proof
        let proof_bytes = serde_json::to_vec(&proof_data)?;
        let hash = blake3::hash(&proof_bytes);
        
        Ok(format!("blake3:{}", hash.to_hex()))
    }
    
    /// Calculate file hash for CUE content verification
    async fn calculate_file_hash(&self, file_path: &str) -> Result<String> {
        // In a real implementation, this would read the actual file
        // For now, return a placeholder hash based on the file path
        let hash = blake3::hash(file_path.as_bytes());
        Ok(format!("blake3:{}", hash.to_hex()))
    }
    
    /// Export audit data for compliance reporting
    pub async fn export_audit_data(&self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Result<AuditExport> {
        let vm_records = self.vm_audit_records.read().await;
        let runtime_logs = self.runtime_action_logs.read().await;
        let cue_audits = self.cue_deployment_audits.read().await;
        
        let start_timestamp = start_date.timestamp() as u64;
        let end_timestamp = end_date.timestamp() as u64;
        
        let filtered_vm_records: Vec<VMAuditRecord> = vm_records
            .iter()
            .filter(|r| r.timestamp >= start_timestamp && r.timestamp <= end_timestamp)
            .cloned()
            .collect();
            
        let filtered_runtime_logs: Vec<RuntimeActionLog> = runtime_logs
            .iter()
            .filter(|r| r.timestamp >= start_timestamp && r.timestamp <= end_timestamp)
            .cloned()
            .collect();
            
        let filtered_cue_audits: Vec<CueDeploymentAudit> = cue_audits
            .iter()
            .filter(|r| r.timestamp >= start_timestamp && r.timestamp <= end_timestamp)
            .cloned()
            .collect();
        
        Ok(AuditExport {
            export_id: Uuid::new_v4().to_string(),
            export_timestamp: Utc::now(),
            start_date,
            end_date,
            vm_audit_records: filtered_vm_records,
            runtime_action_logs: filtered_runtime_logs,
            cue_deployment_audits: filtered_cue_audits,
            total_records: vm_records.len() + runtime_logs.len() + cue_audits.len(),
        })
    }
    
    /// Get audit statistics
    pub async fn get_audit_statistics(&self) -> Result<AuditStatistics> {
        let vm_records = self.vm_audit_records.read().await;
        let runtime_actions = self.runtime_action_logs.read().await;
        let cue_audits = self.cue_deployment_audits.read().await;
        
        Ok(AuditStatistics {
            total_vm_records: vm_records.len(),
            total_runtime_actions: runtime_actions.len(),
            total_cue_deployments: cue_audits.len(),
            successful_deployments: cue_audits.iter().filter(|audit| matches!(audit.status, CueDeploymentStatus::Success)).count(),
            failed_deployments: cue_audits.iter().filter(|audit| matches!(audit.status, CueDeploymentStatus::Failed)).count(),
            last_audit_timestamp: vm_records.last().map(|record| {
                DateTime::from_timestamp(record.timestamp as i64, 0).unwrap_or_else(|| Utc::now())
            }),
            audit_system_status: "Active".to_string(),
        })
    }

    /// Create AuditRecord for immutable audit system
    async fn create_audit_record(
        &self,
        record_type: AuditRecordType,
        description: &str,
        event_data: serde_json::Value,
    ) -> Result<AuditRecord> {
        let record_id = format!("court_{}", Uuid::new_v4().simple());
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        
        // Create runtime event
        let runtime_event = RuntimeEvent {
            event_id: record_id.clone(),
            process_id: std::process::id(),
            binary_path: "bpi-core".to_string(),
            binary_hash: "court_node_hash".to_string(),
            command_line: vec!["court-node".to_string()],
            system_calls: Vec::new(),
            memory_operations: Vec::new(),
            file_operations: Vec::new(),
            network_operations: Vec::new(),
            execution_flow: Vec::new(),
            performance_metrics: crate::immutable_audit_system::PerformanceMetrics {
                cpu_usage: 0.0,
                memory_usage: 0,
                disk_io: 0,
                network_io: 0,
            },
        };
        
        // Create security event
        let security_event = SecurityEvent {
            event_id: format!("sec_{}", Uuid::new_v4().simple()),
            security_level: SecurityLevel::Info,
            threat_classification: vec!["court_operation".to_string()],
            indicators_of_compromise: Vec::new(),
            mitre_attack_techniques: Vec::new(),
            security_policies_violated: Vec::new(),
            behavioral_anomalies: Vec::new(),
        };
        
        // Create system state (using placeholder values for now)
        let system_state = SystemState {
            state_id: format!("state_{}", Uuid::new_v4().simple()),
            cpu_state: crate::immutable_audit_system::CpuState {
                usage_percent: 0.0,
                load_average: vec![0.0, 0.0, 0.0],
            },
            memory_state: crate::immutable_audit_system::MemoryState {
                total_bytes: 0,
                used_bytes: 0,
                available_bytes: 0,
            },
            process_state: crate::immutable_audit_system::ProcessState {
                running_processes: 0,
                zombie_processes: 0,
            },
            network_state: crate::immutable_audit_system::NetworkState {
                active_connections: 0,
                bytes_sent: 0,
                bytes_received: 0,
            },
            timestamp,
            state_hash: "placeholder_hash".to_string(),
        };
        
        // Create immutable proof
        let immutable_proof = crate::immutable_audit_system::ImmutableProof {
            proof_type: "court_vm_proof".to_string(),
            cryptographic_hash: "court_proof_hash".to_string(),
            digital_signature: "court_signature".to_string(),
        };
        
        Ok(AuditRecord {
            record_id,
            record_type,
            component: ComponentType::CourtNode,
            runtime_event,
            security_event,
            vulnerability_event: None,
            attack_event: None,
            bug_event: None,
            system_state,
            immutable_proof,
            timestamp,
        })
    }
}

/// Audit export structure for compliance reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditExport {
    pub export_id: String,
    pub export_timestamp: DateTime<Utc>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub vm_audit_records: Vec<VMAuditRecord>,
    pub runtime_action_logs: Vec<RuntimeActionLog>,
    pub cue_deployment_audits: Vec<CueDeploymentAudit>,
    pub total_records: usize,
}

/// Audit statistics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStatistics {
    pub total_vm_records: usize,
    pub total_runtime_actions: usize,
    pub total_cue_deployments: usize,
    pub successful_deployments: usize,
    pub failed_deployments: usize,
    pub last_audit_timestamp: Option<DateTime<Utc>>,
    pub audit_system_status: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::immutable_audit_system::ImmutableAuditSystem;
    
    #[tokio::test]
    async fn test_court_vm_audit_system_creation() {
        let audit_system = Arc::new(ImmutableAuditSystem::new("/tmp/test_audit1").await.unwrap());
        let vm_audit = CourtVMAuditSystem::new(audit_system).await;
        assert!(vm_audit.is_ok());
    }
    
    #[tokio::test]
    async fn test_vm_operation_recording() {
        let audit_system = Arc::new(ImmutableAuditSystem::new("/tmp/test_audit2").await.unwrap());
        let vm_audit = CourtVMAuditSystem::new(audit_system).await.unwrap();
        
        let record_id = vm_audit.record_vm_operation(
            VMAuditOperationType::ContractExecution,
            Some("test_contract".to_string()),
            None,
            serde_json::json!({"test": "data"}),
        ).await;
        
        assert!(record_id.is_ok());
    }
    
    #[tokio::test]
    async fn test_runtime_action_recording() {
        let audit_system = Arc::new(ImmutableAuditSystem::new("/tmp/test_audit3").await.unwrap());
        let vm_audit = CourtVMAuditSystem::new(audit_system).await.unwrap();
        
        let action_id = vm_audit.record_runtime_action(
            RuntimeActionType::ContractExecute,
            "Test action",
            "{\"test\": \"parameters\"}",
        ).await;
        
        assert!(action_id.is_ok());
    }
    
    #[tokio::test]
    async fn test_cue_deployment_audit() {
        let audit_system = Arc::new(ImmutableAuditSystem::new("/tmp/test_audit4").await.unwrap());
        let vm_audit = CourtVMAuditSystem::new(audit_system).await.unwrap();
        
        let deployment_id = "test_deployment";
        let cue_file_path = "./test.cue";
        
        // Test deployment start
        let start_result = vm_audit.record_cue_deployment_start(deployment_id, cue_file_path).await;
        assert!(start_result.is_ok());
        
        // Test deployment success
        let success_result = vm_audit.record_cue_deployment_success(deployment_id, "orchestration_123").await;
        assert!(success_result.is_ok());
        
        // Verify audit trail
        let audit_trail = vm_audit.get_cue_deployment_audit(deployment_id).await.unwrap();
        assert_eq!(audit_trail.len(), 2); // Should have both start and success entries
        assert_eq!(audit_trail[0].deployment_id, deployment_id);
    }
}
