//! # Unified Audit System
//!
//! Cross-system audit trails with privacy preservation for Court, Shadow Registry, and BPI Mesh

use anyhow::Result;
use anyhow::anyhow;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tracing::{info, warn, error, debug};
use crate::bpi_ledger_integration::{BpiLedgerClient, LedgerConnectionType, ProofType};

/// Unified audit system for cross-system integration
#[derive(Debug)]
pub struct UnifiedAuditSystem {
    config: UnifiedAuditConfig,
    audit_events: Arc<RwLock<Vec<UnifiedAuditEvent>>>,
    privacy_manager: Arc<RwLock<PrivacyManager>>,
    compliance_tracker: Arc<RwLock<ComplianceTracker>>,
    /// Real BPI ledger client for audit trail storage
    bpi_client: Arc<BpiLedgerClient>,
}

/// Unified audit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedAuditConfig {
    pub privacy_preservation_enabled: bool,
    pub cross_system_correlation: bool,
    pub real_time_compliance_checking: bool,
    pub zk_proof_generation: bool,
    pub max_audit_events: usize,
}

impl Default for UnifiedAuditConfig {
    fn default() -> Self {
        Self {
            privacy_preservation_enabled: true,
            cross_system_correlation: true,
            real_time_compliance_checking: true,
            zk_proof_generation: true,
            max_audit_events: 1000000,
        }
    }
}

/// Unified audit event across all systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedAuditEvent {
    pub event_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub system_source: SystemSource,
    pub event_type: UnifiedAuditEventType,
    pub correlation_id: Option<Uuid>,
    pub privacy_level: PrivacyLevel,
    pub compliance_status: ComplianceStatus,
    pub event_data: HashMap<String, serde_json::Value>,
    pub zk_proof: Option<String>,
}

/// System sources for audit events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SystemSource {
    CourtNode,
    ShadowRegistry,
    BpiMesh,
    CourtShadowBridge,
    CourtBpiMeshBridge,
    UnifiedAuditSystem,
}

/// Unified audit event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnifiedAuditEventType {
    // Court Node events
    ContractDeployed,
    ContractExecuted,
    // Shadow Registry events
    Web2Web3BridgeUsed,
    PrivacyProofGenerated,
    // BPI Mesh events
    BankingOperationExecuted,
    EconomicTransactionProcessed,
    // Cross-system events
    CrossSystemIntegration,
    ComplianceViolation,
    SecurityAlert,
}

/// Privacy levels for audit events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrivacyLevel {
    Public,
    Internal,
    Confidential,
    Restricted,
    ZkProtected,
}

/// Compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant { violations: Vec<String> },
    UnderReview,
    Exempt,
}

/// Privacy manager for audit events
#[derive(Debug)]
pub struct PrivacyManager {
    privacy_policies: HashMap<SystemSource, PrivacyPolicy>,
    zk_proof_system: Option<String>, // ZK proof system integration
}

/// Privacy policy for system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyPolicy {
    pub default_privacy_level: PrivacyLevel,
    pub zk_proof_required: bool,
    pub data_retention_days: u32,
    pub anonymization_required: bool,
}

/// Compliance tracker
#[derive(Debug)]
pub struct ComplianceTracker {
    compliance_rules: Vec<ComplianceRule>,
    violation_history: Vec<ComplianceViolation>,
}

/// Compliance rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRule {
    pub rule_id: String,
    pub rule_name: String,
    pub applicable_systems: Vec<SystemSource>,
    pub rule_type: ComplianceRuleType,
    pub severity: ComplianceSeverity,
}

/// Compliance rule types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceRuleType {
    DataProtection,
    FinancialRegulation,
    PrivacyRequirement,
    SecurityStandard,
    AuditRequirement,
}

/// Compliance severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Compliance violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    pub violation_id: Uuid,
    pub rule_id: String,
    pub timestamp: DateTime<Utc>,
    pub system_source: SystemSource,
    pub description: String,
    pub severity: ComplianceSeverity,
    pub resolved: bool,
}

impl UnifiedAuditSystem {
    /// Create new unified audit system
    pub async fn new(config: UnifiedAuditConfig) -> Result<Self> {
        let bpi_client = Arc::new(BpiLedgerClient::new().await?);
        
        Ok(Self {
            config,
            audit_events: Arc::new(RwLock::new(Vec::new())),
            privacy_manager: Arc::new(RwLock::new(PrivacyManager {
                privacy_policies: HashMap::new(),
                zk_proof_system: None,
            })),
            compliance_tracker: Arc::new(RwLock::new(ComplianceTracker {
                compliance_rules: Vec::new(),
                violation_history: Vec::new(),
            })),
            bpi_client,
        })
    }

    /// Log unified audit event
    pub async fn log_audit_event(
        &self,
        system_source: SystemSource,
        event_type: UnifiedAuditEventType,
        event_data: HashMap<String, serde_json::Value>,
        correlation_id: Option<Uuid>,
    ) -> Result<Uuid> {
        let event_id = Uuid::new_v4();
        
        // Determine privacy level
        let privacy_level = self.determine_privacy_level(&system_source, &event_type).await;
        
        // Check compliance
        let compliance_status = self.check_compliance(&system_source, &event_type, &event_data).await?;
        
        // Generate real ZK proof if required
        let zk_proof = if self.config.zk_proof_generation && privacy_level == PrivacyLevel::ZkProtected {
            let proof = self.bpi_client.zk_proof_system.generate_proof(
                ProofType::AuditTrail,
                &serde_json::to_vec(&event_data)?,
            ).await?;
            Some(format!("ZK Proof generated: type={:?}, valid={}", proof.proof.proof_type, proof.proof.is_valid))
        } else {
            None
        };

        // Clone values before moving them into the event struct
        let system_source_for_json = system_source.clone();
        let privacy_level_for_json = privacy_level.clone();
        let compliance_status_for_json = compliance_status.clone();
        let zk_proof_for_json = zk_proof.clone();

        let event = UnifiedAuditEvent {
            event_id,
            timestamp: Utc::now(),
            system_source,
            event_type,
            correlation_id,
            privacy_level,
            compliance_status,
            event_data: event_data.clone(),
            zk_proof: zk_proof.clone(),
        };

        // Store audit event in BPI ledger for immutable audit trail
        let audit_transaction_data = serde_json::json!({
            "audit_event": event,
            "system_source": system_source_for_json,
            "privacy_level": privacy_level_for_json,
            "compliance_status": compliance_status_for_json,
            "correlation_id": correlation_id,
            "zk_proof": zk_proof_for_json
        });

        // Store locally first for immediate access
        {
            let mut events = self.audit_events.write().await;
            events.push(event);
            
            // Maintain max events limit
            if events.len() > self.config.max_audit_events {
                events.remove(0);
            }
        }

        // Submit to BPI ledger for immutable audit storage (async, non-blocking)
        let ledger_result = self.bpi_client.submit_transaction_with_proof(
            "unified_audit_event",
            audit_transaction_data,
            zk_proof.clone(),
        ).await;

        match ledger_result {
            Ok(tx_result) => {
                info!("Audit event {} stored in BPI ledger with confirmation {}", 
                    event_id, tx_result.confirmation_hash);
            }
            Err(e) => {
                error!("Failed to store audit event {} in BPI ledger: {}", event_id, e);
                // Event is already stored locally, so this is not critical
            }
        }

        Ok(event_id)
    }

    /// Get audit trail with privacy filtering
    pub async fn get_audit_trail(
        &self,
        system_filter: Option<SystemSource>,
        privacy_clearance: PrivacyLevel,
    ) -> Result<Vec<UnifiedAuditEvent>> {
        let events = self.audit_events.read().await;
        
        let filtered_events: Vec<UnifiedAuditEvent> = events
            .iter()
            .filter(|event| {
                // Filter by system if specified
                if let Some(ref filter) = system_filter {
                    if event.system_source != *filter {
                        return false;
                    }
                }
                
                // Filter by privacy clearance
                self.has_privacy_clearance(&event.privacy_level, &privacy_clearance)
            })
            .cloned()
            .collect();

        Ok(filtered_events)
    }

    /// Generate compliance report
    pub async fn generate_compliance_report(&self) -> Result<ComplianceReport> {
        let events = self.audit_events.read().await;
        let compliance_tracker = self.compliance_tracker.read().await;
        
        let total_events = events.len();
        let compliant_events = events.iter()
            .filter(|e| matches!(e.compliance_status, ComplianceStatus::Compliant | ComplianceStatus::Exempt))
            .count();
        
        let compliance_rate = if total_events > 0 {
            (compliant_events as f64 / total_events as f64) * 100.0
        } else {
            100.0
        };

        Ok(ComplianceReport {
            report_id: Uuid::new_v4(),
            generated_at: Utc::now(),
            total_events,
            compliant_events,
            compliance_rate,
            active_violations: compliance_tracker.violation_history.iter()
                .filter(|v| !v.resolved)
                .count(),
            system_compliance: self.calculate_system_compliance(&events).await,
        })
    }

    /// Determine privacy level for event
    async fn determine_privacy_level(
        &self,
        system_source: &SystemSource,
        _event_type: &UnifiedAuditEventType,
    ) -> PrivacyLevel {
        let privacy_manager = self.privacy_manager.read().await;
        
        if let Some(policy) = privacy_manager.privacy_policies.get(system_source) {
            policy.default_privacy_level.clone()
        } else {
            PrivacyLevel::Internal // Default
        }
    }

    /// Check compliance for event
    async fn check_compliance(
        &self,
        system_source: &SystemSource,
        _event_type: &UnifiedAuditEventType,
        _event_data: &HashMap<String, serde_json::Value>,
    ) -> Result<ComplianceStatus> {
        let compliance_tracker = self.compliance_tracker.read().await;
        
        // Check applicable compliance rules
        let applicable_rules: Vec<&ComplianceRule> = compliance_tracker.compliance_rules
            .iter()
            .filter(|rule| rule.applicable_systems.contains(system_source))
            .collect();

        if applicable_rules.is_empty() {
            return Ok(ComplianceStatus::Exempt);
        }

        // For now, assume compliant (real implementation would check rules)
        Ok(ComplianceStatus::Compliant)
    }

    /// Check privacy clearance
    fn has_privacy_clearance(&self, event_privacy: &PrivacyLevel, clearance: &PrivacyLevel) -> bool {
        use PrivacyLevel::*;
        
        let event_level = match event_privacy {
            Public => 1,
            Internal => 2,
            Confidential => 3,
            Restricted => 4,
            ZkProtected => 5,
        };

        let clearance_level = match clearance {
            Public => 1,
            Internal => 2,
            Confidential => 3,
            Restricted => 4,
            ZkProtected => 5,
        };

        clearance_level >= event_level
    }

    /// Calculate system compliance
    async fn calculate_system_compliance(&self, events: &[UnifiedAuditEvent]) -> HashMap<SystemSource, f64> {
        let mut system_compliance = HashMap::new();
        
        for system in [SystemSource::CourtNode, SystemSource::ShadowRegistry, SystemSource::BpiMesh] {
            let system_events: Vec<&UnifiedAuditEvent> = events.iter()
                .filter(|e| e.system_source == system)
                .collect();
            
            if system_events.is_empty() {
                system_compliance.insert(system, 100.0);
                continue;
            }
            
            let compliant_count = system_events.iter()
                .filter(|e| matches!(e.compliance_status, ComplianceStatus::Compliant))
                .count();
            
            let compliance_rate = (compliant_count as f64 / system_events.len() as f64) * 100.0;
            system_compliance.insert(system, compliance_rate);
        }
        
        system_compliance
    }
}

/// Compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub report_id: Uuid,
    pub generated_at: DateTime<Utc>,
    pub total_events: usize,
    pub compliant_events: usize,
    pub compliance_rate: f64,
    pub active_violations: usize,
    pub system_compliance: HashMap<SystemSource, f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_unified_audit_system_creation() {
        let config = UnifiedAuditConfig::default();
        let audit_system = UnifiedAuditSystem::new(config).await.unwrap();
        
        let trail = audit_system.get_audit_trail(None, PrivacyLevel::Internal).await.unwrap();
        assert_eq!(trail.len(), 0);
    }

    #[tokio::test]
    async fn test_audit_event_logging() {
        let audit_system = UnifiedAuditSystem::new(UnifiedAuditConfig::default()).await.unwrap();
        
        let mut event_data = HashMap::new();
        event_data.insert("test_key".to_string(), serde_json::json!("test_value"));
        
        let event_id = audit_system.log_audit_event(
            SystemSource::CourtNode,
            UnifiedAuditEventType::ContractDeployed,
            event_data,
            None,
        ).await.unwrap();
        
        assert!(!event_id.is_nil());
        
        let trail = audit_system.get_audit_trail(None, PrivacyLevel::Internal).await.unwrap();
        assert_eq!(trail.len(), 1);
        assert_eq!(trail[0].event_id, event_id);
    }

    #[tokio::test]
    async fn test_compliance_report_generation() {
        let audit_system = UnifiedAuditSystem::new(UnifiedAuditConfig::default()).await.unwrap();
        
        // Log some test events
        for i in 0..5 {
            let mut event_data = HashMap::new();
            event_data.insert("iteration".to_string(), serde_json::json!(i));
            
            audit_system.log_audit_event(
                SystemSource::CourtNode,
                UnifiedAuditEventType::ContractExecuted,
                event_data,
                None,
            ).await.unwrap();
        }
        
        let report = audit_system.generate_compliance_report().await.unwrap();
        assert_eq!(report.total_events, 5);
        assert_eq!(report.compliant_events, 5);
        assert_eq!(report.compliance_rate, 100.0);
    }
}
