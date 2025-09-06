use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;

use crate::immutable_audit_system::{
    ImmutableAuditSystem, AuditRecord, SecurityEvent, RuntimeEvent, ComponentType,
    AuditRecordType, SecurityLevel, SystemState, ImmutableProof, PerformanceMetrics,
    CpuState, MemoryState, ProcessState, NetworkState
};
use crate::forensic_firewall::{
    behavioral_analysis::{BehavioralConfig, UserActivity, BehavioralAnalysisResult, DetectedAnomaly},
    dynamic_response::{DynamicResponseConfig, ResponseType},
    cue_engine::{CueRuleEngine, SecurityDecision, SecurityAction},
    threat_intel::{ThreatIntelligence, ThreatClassification},
};

/// Audit bridge for forensic firewall integration with immutable audit system
#[derive(Debug, Clone)]
pub struct ForensicAuditBridge {
    pub id: Uuid,
    pub audit_system: Arc<RwLock<ImmutableAuditSystem>>,
    pub cue_engine: Arc<CueRuleEngine>,
    pub forensic_events: Arc<RwLock<HashMap<Uuid, ForensicEvent>>>,
    pub evidence_chain: Arc<RwLock<Vec<EvidenceLink>>>,
    pub config: AuditBridgeConfig,
}

/// Forensic event for audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForensicEvent {
    pub event_id: Uuid,
    pub event_type: ForensicEventType,
    pub timestamp: DateTime<Utc>,
    pub source_component: ComponentType,
    pub severity: ForensicSeverity,
    pub description: String,
    pub evidence: ForensicEvidence,
    pub cue_evaluation: Option<SecurityDecision>,
    pub behavioral_analysis: Option<BehavioralAnalysisResult>,
    pub threat_intelligence: Option<ThreatClassification>,
    pub chain_of_custody: Vec<CustodyTransfer>,
    pub immutable_hash: String,
    pub digital_signature: String,
}

/// Types of forensic events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForensicEventType {
    SecurityThreatDetected,
    BehavioralAnomalyDetected,
    CueRuleViolation,
    PolicyEnforcementAction,
    ForensicEvidenceCollected,
    IncidentResponse,
    ComplianceViolation,
    SystemCompromise,
    DataExfiltration,
    UnauthorizedAccess,
    MaliciousActivity,
    SuspiciousPattern,
}

/// Forensic severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForensicSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
    Emergency,
}

/// Forensic evidence collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForensicEvidence {
    pub evidence_id: Uuid,
    pub evidence_type: EvidenceType,
    pub collected_at: DateTime<Utc>,
    pub collector: String,
    pub integrity_hash: String,
    pub digital_signature: String,
    pub metadata: HashMap<String, String>,
    pub raw_data: Vec<u8>,
    pub processed_data: HashMap<String, serde_json::Value>,
    pub chain_of_custody_id: Uuid,
}

/// Types of forensic evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    NetworkPacket,
    SystemLog,
    ProcessMemory,
    FileSystemArtifact,
    RegistryEntry,
    UserActivity,
    NetworkFlow,
    CryptographicProof,
    BehavioralPattern,
    ThreatIndicator,
    ComplianceRecord,
    AuditTrail,
}

/// Chain of custody transfer record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustodyTransfer {
    pub transfer_id: Uuid,
    pub from_entity: String,
    pub to_entity: String,
    pub transferred_at: DateTime<Utc>,
    pub transfer_reason: String,
    pub integrity_verified: bool,
    pub digital_signature: String,
    pub witness_signatures: Vec<String>,
}

/// Evidence link in the forensic chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceLink {
    pub link_id: Uuid,
    pub previous_hash: String,
    pub current_hash: String,
    pub evidence_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub merkle_proof: String,
    pub block_height: u64,
    pub validator_signatures: Vec<String>,
}

/// Audit bridge configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditBridgeConfig {
    pub enable_real_time_audit: bool,
    pub enable_evidence_collection: bool,
    pub enable_chain_of_custody: bool,
    pub evidence_retention_days: u32,
    pub max_evidence_size_mb: u64,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
    pub digital_signature_required: bool,
    pub witness_signatures_required: u32,
}

impl ForensicAuditBridge {
    /// Create new forensic audit bridge
    pub fn new(
        audit_system: Arc<RwLock<ImmutableAuditSystem>>,
        cue_engine: Arc<CueRuleEngine>,
        config: AuditBridgeConfig,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            audit_system,
            cue_engine,
            forensic_events: Arc::new(RwLock::new(HashMap::new())),
            evidence_chain: Arc::new(RwLock::new(Vec::new())),
            config,
        }
    }

    /// Record forensic security event
    pub async fn record_security_event(
        &self,
        event_type: ForensicEventType,
        source_component: ComponentType,
        severity: ForensicSeverity,
        description: String,
        evidence: Option<ForensicEvidence>,
        cue_evaluation: Option<SecurityDecision>,
        behavioral_analysis: Option<BehavioralAnalysisResult>,
        threat_intelligence: Option<ThreatClassification>,
    ) -> Result<Uuid> {
        let event_id = Uuid::new_v4();
        let timestamp = Utc::now();

        // Create forensic evidence if provided
        let forensic_evidence = if let Some(evidence) = evidence {
            evidence
        } else {
            self.create_default_evidence(&event_type, &source_component).await?
        };

        // Calculate immutable hash
        let immutable_hash = self.calculate_event_hash(&event_id, &timestamp, &description, &forensic_evidence).await?;

        // Generate digital signature
        let digital_signature = self.generate_digital_signature(&immutable_hash).await?;

        // Create forensic event
        let forensic_event = ForensicEvent {
            event_id,
            event_type: event_type.clone(),
            timestamp,
            source_component: source_component.clone(),
            severity: severity.clone(),
            description: description.clone(),
            evidence: forensic_evidence.clone(),
            cue_evaluation,
            behavioral_analysis,
            threat_intelligence,
            chain_of_custody: Vec::new(),
            immutable_hash: immutable_hash.clone(),
            digital_signature: digital_signature.clone(),
        };

        // Store forensic event
        {
            let mut events = self.forensic_events.write().await;
            events.insert(event_id, forensic_event.clone());
        }

        // Create audit record for immutable audit system
        let audit_record = self.create_audit_record(&forensic_event).await?;

        // Submit to immutable audit system
        {
            let mut audit_system = self.audit_system.write().await;
            let record_id = audit_system.record_immutable_event(source_component, audit_record).await?;
            tracing::info!("Forensic event {} recorded with audit record ID: {}", event_id, record_id);
        }

        // Add to evidence chain if enabled
        if self.config.enable_chain_of_custody {
            self.add_to_evidence_chain(&forensic_event).await?;
        }

        // Trigger real-time notifications if enabled
        if self.config.enable_real_time_audit {
            self.trigger_real_time_notification(&forensic_event).await?;
        }

        Ok(event_id)
    }

    /// Record behavioral anomaly detection
    pub async fn record_behavioral_anomaly(
        &self,
        analysis_result: &BehavioralAnalysisResult,
        source_component: ComponentType,
    ) -> Result<Uuid> {
        let severity = match analysis_result.risk_level {
            crate::forensic_firewall::behavioral_analysis::RiskLevel::Low => ForensicSeverity::Low,
            crate::forensic_firewall::behavioral_analysis::RiskLevel::Medium => ForensicSeverity::Medium,
            crate::forensic_firewall::behavioral_analysis::RiskLevel::High => ForensicSeverity::High,
            crate::forensic_firewall::behavioral_analysis::RiskLevel::Critical => ForensicSeverity::Critical,
        };

        let description = format!(
            "Behavioral anomaly detected for entity: {} (type: {}, score: {:.3})",
            analysis_result.entity_id,
            analysis_result.analysis_type,
            analysis_result.anomaly_score
        );

        // Create evidence from behavioral analysis
        let evidence = self.create_behavioral_evidence(analysis_result).await?;

        self.record_security_event(
            ForensicEventType::BehavioralAnomalyDetected,
            source_component,
            severity,
            description,
            Some(evidence),
            None,
            Some(analysis_result.clone()),
            None,
        ).await
    }

    /// Record CUE rule violation
    pub async fn record_cue_violation(
        &self,
        cue_evaluation: &SecurityDecision,
        source_component: ComponentType,
    ) -> Result<Uuid> {
        let severity = match &cue_evaluation.action {
            SecurityAction::Allow => ForensicSeverity::Info,
            SecurityAction::Monitor => ForensicSeverity::Low,
            SecurityAction::Block => ForensicSeverity::High,
            SecurityAction::Quarantine => ForensicSeverity::Critical,
            SecurityAction::Escalate => ForensicSeverity::High,
            SecurityAction::EmergencyBlock => ForensicSeverity::Emergency,
        };

        let description = format!(
            "CUE rule violation detected: {} (contract: {}, decision: {:?})",
            "unknown_rule",
            "unknown_contract".to_string(),
            cue_evaluation.action
        );

        // Create evidence from CUE evaluation
        let evidence = self.create_cue_evidence(cue_evaluation).await?;

        self.record_security_event(
            ForensicEventType::CueRuleViolation,
            source_component,
            severity,
            description,
            Some(evidence),
            Some(cue_evaluation.clone()),
            None,
            None,
        ).await
    }

    /// Record threat intelligence detection
    pub async fn record_threat_detection(
        &self,
        threat_classification: &ThreatClassification,
        source_component: ComponentType,
    ) -> Result<Uuid> {
        let severity = match threat_classification.threat_level {
            crate::forensic_firewall::threat_intel::ThreatLevel::Low => ForensicSeverity::Low,
            crate::forensic_firewall::threat_intel::ThreatLevel::Medium => ForensicSeverity::Medium,
            crate::forensic_firewall::threat_intel::ThreatLevel::High => ForensicSeverity::High,
            crate::forensic_firewall::threat_intel::ThreatLevel::Critical => ForensicSeverity::Critical,
            crate::forensic_firewall::threat_intel::ThreatLevel::Emergency => ForensicSeverity::Critical,
        };

        let description = format!(
            "Threat detected: {} (type: {}, confidence: {:.3})",
            format!("{:?}", threat_classification.threat_type),
            format!("{:?}", threat_classification.threat_type),
            chrono::Utc::now().timestamp() as u64,
        );

        // Create evidence from threat intelligence
        let evidence = self.create_threat_evidence(threat_classification).await?;

        self.record_security_event(
            ForensicEventType::SecurityThreatDetected,
            source_component,
            severity,
            description,
            Some(evidence),
            None,
            None,
            Some(threat_classification.clone()),
        ).await
    }

    /// Get forensic event by ID
    pub async fn get_forensic_event(&self, event_id: &Uuid) -> Result<Option<ForensicEvent>> {
        let events = self.forensic_events.read().await;
        Ok(events.get(event_id).cloned())
    }

    /// Get forensic events by type
    pub async fn get_events_by_type(&self, event_type: &ForensicEventType) -> Result<Vec<ForensicEvent>> {
        let events = self.forensic_events.read().await;
        Ok(events.values()
            .filter(|event| std::mem::discriminant(&event.event_type) == std::mem::discriminant(event_type))
            .cloned()
            .collect())
    }

    /// Get forensic events by severity
    pub async fn get_events_by_severity(&self, severity: &ForensicSeverity) -> Result<Vec<ForensicEvent>> {
        let events = self.forensic_events.read().await;
        Ok(events.values()
            .filter(|event| std::mem::discriminant(&event.severity) == std::mem::discriminant(severity))
            .cloned()
            .collect())
    }

    /// Get evidence chain
    pub async fn get_evidence_chain(&self) -> Result<Vec<EvidenceLink>> {
        let chain = self.evidence_chain.read().await;
        Ok(chain.clone())
    }

    /// Verify evidence integrity
    pub async fn verify_evidence_integrity(&self, evidence_id: &Uuid) -> Result<bool> {
        let events = self.forensic_events.read().await;
        for event in events.values() {
            if event.evidence.evidence_id == *evidence_id {
                // Verify hash integrity
                let calculated_hash = self.calculate_evidence_hash(&event.evidence).await?;
                return Ok(calculated_hash == event.evidence.integrity_hash);
            }
        }
        Ok(false)
    }

    /// Create audit record from forensic event
    async fn create_audit_record(&self, forensic_event: &ForensicEvent) -> Result<AuditRecord> {
        let security_event = SecurityEvent {
            event_id: forensic_event.event_id.to_string(),
            security_level: SecurityLevel::High, // Default security level
            threat_classification: vec![format!("{:?}", forensic_event.event_type)],
            indicators_of_compromise: vec![], // Empty for now
            mitre_attack_techniques: vec![], // Empty for now
            security_policies_violated: vec![], // Empty for now
            behavioral_anomalies: vec![], // Empty for now
        };

        let runtime_event = RuntimeEvent {
            event_id: forensic_event.event_id.to_string(),
            process_id: std::process::id(),
            binary_path: "/usr/bin/bpi-core".to_string(),
            binary_hash: "sha256:abcd1234".to_string(), // Placeholder
            command_line: vec!["bpi-core".to_string(), "forensic".to_string()],
            system_calls: vec![], // Empty for now
            memory_operations: vec![], // Empty for now
            file_operations: vec![], // Empty for now
            network_operations: vec![], // Empty for now
            execution_flow: vec![], // Empty for now
            performance_metrics: PerformanceMetrics {
                cpu_usage: 0.0,
                memory_usage: 0,
                disk_io: 0,
                network_io: 0,
            },
        };

        Ok(AuditRecord {
            timestamp: forensic_event.timestamp.timestamp() as u64,
            record_id: Uuid::new_v4().to_string(),
            record_type: AuditRecordType::SecurityViolation,
            component: forensic_event.source_component.clone(),
            runtime_event,
            security_event,
            vulnerability_event: None,
            attack_event: None,
            bug_event: None,
            system_state: SystemState {
                state_id: uuid::Uuid::new_v4().to_string(),
                cpu_state: CpuState {
                    usage_percent: 0.0,
                    load_average: vec![0.0, 0.0, 0.0],
                },
                memory_state: MemoryState {
                    total_bytes: 0,
                    used_bytes: 0,
                    available_bytes: 0,
                },
                process_state: ProcessState {
                    running_processes: 0,
                    zombie_processes: 0,
                },
                network_state: NetworkState {
                    active_connections: 0,
                    bytes_sent: 0,
                    bytes_received: 0,
                },
                timestamp: chrono::Utc::now().timestamp() as u64,
                state_hash: "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            },
            immutable_proof: ImmutableProof {
                proof_type: "audit_record".to_string(),
                cryptographic_hash: "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
                digital_signature: "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            },
        })
    }

    /// Create default evidence for event
    async fn create_default_evidence(
        &self,
        event_type: &ForensicEventType,
        source_component: &ComponentType,
    ) -> Result<ForensicEvidence> {
        let evidence_id = Uuid::new_v4();
        let timestamp = Utc::now();
        
        let evidence_type = match event_type {
            ForensicEventType::SecurityThreatDetected => EvidenceType::ThreatIndicator,
            ForensicEventType::BehavioralAnomalyDetected => EvidenceType::BehavioralPattern,
            ForensicEventType::CueRuleViolation => EvidenceType::ComplianceRecord,
            _ => EvidenceType::AuditTrail,
        };

        let raw_data = format!("Default evidence for {:?} from {:?}", event_type, source_component).into_bytes();
        let integrity_hash = self.calculate_data_hash(&raw_data).await?;
        let digital_signature = self.generate_digital_signature(&integrity_hash).await?;

        Ok(ForensicEvidence {
            evidence_id,
            evidence_type,
            collected_at: timestamp,
            collector: "forensic_audit_bridge".to_string(),
            integrity_hash,
            digital_signature,
            metadata: HashMap::new(),
            raw_data,
            processed_data: HashMap::new(),
            chain_of_custody_id: Uuid::new_v4(),
        })
    }

    /// Create behavioral evidence
    async fn create_behavioral_evidence(&self, analysis_result: &BehavioralAnalysisResult) -> Result<ForensicEvidence> {
        let evidence_id = Uuid::new_v4();
        let raw_data = serde_json::to_vec(analysis_result)?;
        let integrity_hash = self.calculate_data_hash(&raw_data).await?;
        let digital_signature = self.generate_digital_signature(&integrity_hash).await?;

        let mut processed_data = HashMap::new();
        processed_data.insert("entity_id".to_string(), serde_json::Value::String(analysis_result.entity_id.clone()));
        processed_data.insert("analysis_type".to_string(), serde_json::Value::String(analysis_result.analysis_type.clone()));
        processed_data.insert("anomaly_score".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(analysis_result.anomaly_score).unwrap()));
        processed_data.insert("confidence".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(analysis_result.confidence).unwrap()));

        Ok(ForensicEvidence {
            evidence_id,
            evidence_type: EvidenceType::BehavioralPattern,
            collected_at: analysis_result.analyzed_at,
            collector: "behavioral_analyzer".to_string(),
            integrity_hash,
            digital_signature,
            metadata: HashMap::new(),
            raw_data,
            processed_data,
            chain_of_custody_id: Uuid::new_v4(),
        })
    }

    /// Create CUE evidence
    async fn create_cue_evidence(&self, cue_evaluation: &SecurityDecision) -> Result<ForensicEvidence> {
        let evidence_id = Uuid::new_v4();
        let raw_data = serde_json::to_vec(cue_evaluation)?;
        let integrity_hash = self.calculate_data_hash(&raw_data).await?;
        let digital_signature = self.generate_digital_signature(&integrity_hash).await?;

        let mut processed_data = HashMap::new();
        processed_data.insert("contract_id".to_string(), serde_json::Value::String("unknown_contract".to_string()));
        processed_data.insert("rule_name".to_string(), serde_json::Value::String("unknown_rule".to_string()));
        processed_data.insert("decision".to_string(), serde_json::Value::String(format!("{:?}", cue_evaluation.action)));
        processed_data.insert("confidence".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(cue_evaluation.confidence).unwrap()));

        Ok(ForensicEvidence {
            evidence_id,
            evidence_type: EvidenceType::ComplianceRecord,
            collected_at: chrono::Utc::now(),
            collector: "cue_engine".to_string(),
            integrity_hash,
            digital_signature,
            metadata: HashMap::new(),
            raw_data,
            processed_data,
            chain_of_custody_id: Uuid::new_v4(),
        })
    }

    /// Create threat evidence
    async fn create_threat_evidence(&self, threat_classification: &ThreatClassification) -> Result<ForensicEvidence> {
        let evidence_id = Uuid::new_v4();
        let raw_data = serde_json::to_vec(threat_classification)?;
        let integrity_hash = self.calculate_data_hash(&raw_data).await?;
        let digital_signature = self.generate_digital_signature(&integrity_hash).await?;

        let mut processed_data = HashMap::new();
        processed_data.insert("threat_name".to_string(), serde_json::Value::String(format!("{:?}", threat_classification.threat_type)));
        processed_data.insert("threat_type".to_string(), serde_json::Value::String(format!("{:?}", threat_classification.threat_type)));
        processed_data.insert("confidence".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(threat_classification.confidence).unwrap()));

        Ok(ForensicEvidence {
            evidence_id,
            evidence_type: EvidenceType::ThreatIndicator,
            collected_at: chrono::Utc::now(),
            collector: "threat_intelligence".to_string(),
            integrity_hash,
            digital_signature,
            metadata: HashMap::new(),
            raw_data,
            processed_data,
            chain_of_custody_id: Uuid::new_v4(),
        })
    }

    /// Add forensic event to evidence chain
    async fn add_to_evidence_chain(&self, forensic_event: &ForensicEvent) -> Result<()> {
        let mut chain = self.evidence_chain.write().await;
        
        let previous_hash = if let Some(last_link) = chain.last() {
            last_link.current_hash.clone()
        } else {
            "genesis".to_string()
        };

        let current_hash = self.calculate_chain_hash(&previous_hash, &forensic_event.immutable_hash).await?;
        let merkle_proof = self.generate_merkle_proof(&forensic_event.evidence).await?;

        let evidence_link = EvidenceLink {
            link_id: Uuid::new_v4(),
            previous_hash,
            current_hash,
            evidence_id: forensic_event.evidence.evidence_id,
            timestamp: forensic_event.timestamp,
            merkle_proof,
            block_height: chain.len() as u64 + 1,
            validator_signatures: Vec::new(), // Could be populated with actual validator signatures
        };

        chain.push(evidence_link);
        Ok(())
    }

    /// Trigger real-time notification
    async fn trigger_real_time_notification(&self, forensic_event: &ForensicEvent) -> Result<()> {
        // Implementation would send real-time notifications to security teams
        tracing::warn!(
            "FORENSIC ALERT: {:?} - {} (Severity: {:?})",
            forensic_event.event_type,
            forensic_event.description,
            forensic_event.severity
        );
        Ok(())
    }

    /// Calculate event hash
    async fn calculate_event_hash(
        &self,
        event_id: &Uuid,
        timestamp: &DateTime<Utc>,
        description: &str,
        evidence: &ForensicEvidence,
    ) -> Result<String> {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(event_id.as_bytes());
        hasher.update(timestamp.to_rfc3339().as_bytes());
        hasher.update(description.as_bytes());
        hasher.update(&evidence.raw_data);
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Calculate evidence hash
    async fn calculate_evidence_hash(&self, evidence: &ForensicEvidence) -> Result<String> {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(evidence.evidence_id.as_bytes());
        hasher.update(evidence.collected_at.to_rfc3339().as_bytes());
        hasher.update(&evidence.raw_data);
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Calculate data hash
    async fn calculate_data_hash(&self, data: &[u8]) -> Result<String> {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Calculate chain hash
    async fn calculate_chain_hash(&self, previous_hash: &str, current_hash: &str) -> Result<String> {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(previous_hash.as_bytes());
        hasher.update(current_hash.as_bytes());
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Generate digital signature
    async fn generate_digital_signature(&self, data: &str) -> Result<String> {
        // Simplified signature generation - in production would use proper cryptographic signing
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hasher.update("forensic_firewall_signature_key".as_bytes());
        Ok(format!("sig_{:x}", hasher.finalize()))
    }

    /// Generate Merkle proof
    async fn generate_merkle_proof(&self, evidence: &ForensicEvidence) -> Result<String> {
        // Simplified Merkle proof generation
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(evidence.evidence_id.as_bytes());
        hasher.update(&evidence.raw_data);
        Ok(format!("merkle_{:x}", hasher.finalize()))
    }
}

impl Default for AuditBridgeConfig {
    fn default() -> Self {
        Self {
            enable_real_time_audit: true,
            enable_evidence_collection: true,
            enable_chain_of_custody: true,
            evidence_retention_days: 365,
            max_evidence_size_mb: 100,
            compression_enabled: true,
            encryption_enabled: true,
            digital_signature_required: true,
            witness_signatures_required: 2,
        }
    }
}
