//! BISO Agreement Logic for Stamped BPI Wallets
//!
//! This module implements the BISO (Blockchain-Integrated Security Operations) Agreement system
//! for stamped BPI wallets, providing cue-based compliance agreements and communication API layers.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use anyhow::{Result, anyhow};
use tracing::{info, debug, warn};

/// BISO Agreement types for different stamp categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BisoAgreementType {
    /// Government stamped - full BPCI communication API access
    GovernmentStamped {
        government_id: String,
        jurisdiction: String,
        compliance_level: ComplianceLevel,
        api_access_level: ApiAccessLevel,
    },
    /// Bank stamped - full BPCI communication API access
    BankStamped {
        bank_id: String,
        banking_license: String,
        compliance_level: ComplianceLevel,
        api_access_level: ApiAccessLevel,
    },
    /// Other stamps - POE sharing only, no full communication
    OtherStamped {
        stamp_type: String,
        issuer: String,
        restrictions: CommunicationRestrictions,
    },
    /// Unstamped - POE sharing only with mandatory BISO agreement
    Unstamped {
        wallet_id: String,
        mandatory_biso: bool,
    },
}

/// Compliance levels for stamped wallets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplianceLevel {
    Basic,
    Standard,
    Enhanced,
    Maximum,
    Government,
    Banking,
}

/// API access levels based on stamp type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApiAccessLevel {
    /// Full BPCI communication API access
    Full {
        bank_api: bool,
        government_api: bool,
        cross_system_communication: bool,
    },
    /// Limited to POE sharing only
    PoeOnly {
        proof_sharing: bool,
        compliance_reporting: bool,
    },
    /// No API access
    None,
}

/// Communication restrictions for non-government/bank stamps
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommunicationRestrictions {
    pub can_share_poe: bool,
    pub requires_biso_agreement: bool,
    pub compliance_reporting_required: bool,
    pub allowed_endpoints: Vec<String>,
    pub blocked_endpoints: Vec<String>,
}

/// BISO Agreement structure with cue-based compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BisoAgreement {
    pub agreement_id: Uuid,
    pub wallet_id: String,
    pub agreement_type: BisoAgreementType,
    pub cue_based_rules: Vec<CueBasedRule>,
    pub compliance_requirements: ComplianceRequirements,
    pub communication_policy: CommunicationPolicy,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: AgreementStatus,
    pub signature: Option<String>,
    pub compliance_reports: Vec<ComplianceReport>,
}

/// Cue-based compliance rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CueBasedRule {
    pub rule_id: String,
    pub cue_trigger: CueTrigger,
    pub required_action: RequiredAction,
    pub compliance_check: ComplianceCheck,
    pub enforcement_level: EnforcementLevel,
}

/// Triggers for cue-based rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CueTrigger {
    /// Transaction volume threshold
    TransactionVolume { threshold: u64 },
    /// Data classification level
    DataClassification { level: String },
    /// Geographic region access
    GeographicAccess { regions: Vec<String> },
    /// Time-based trigger
    TimeInterval { interval_hours: u64 },
    /// API endpoint access
    ApiEndpoint { endpoint: String },
    /// Custom trigger
    Custom { trigger_type: String, parameters: HashMap<String, String> },
}

/// Required actions when cue is triggered
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequiredAction {
    /// Generate compliance report
    GenerateComplianceReport,
    /// Require additional authentication
    RequireAuthentication,
    /// Restrict communication
    RestrictCommunication,
    /// Escalate to authority
    EscalateToAuthority { authority_type: String },
    /// Log and monitor
    LogAndMonitor,
    /// Custom action
    Custom { action_type: String, parameters: HashMap<String, String> },
}

/// Compliance check types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceCheck {
    /// KYC verification
    KycVerification,
    /// AML screening
    AmlScreening,
    /// Regulatory compliance
    RegulatoryCompliance { framework: String },
    /// Data protection compliance
    DataProtection { regulation: String },
    /// Custom compliance check
    Custom { check_type: String, criteria: HashMap<String, String> },
}

/// Enforcement levels for compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnforcementLevel {
    /// Log only
    Advisory,
    /// Warn but allow
    Warning,
    /// Block action
    Blocking,
    /// Escalate to authority
    Escalation,
}

/// Compliance requirements for agreements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRequirements {
    pub mandatory_reporting: bool,
    pub audit_trail_required: bool,
    pub data_retention_days: u64,
    pub encryption_required: bool,
    pub geographic_restrictions: Vec<String>,
    pub regulatory_frameworks: Vec<String>,
}

/// Communication policy based on stamp type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationPolicy {
    pub allowed_apis: Vec<String>,
    pub blocked_apis: Vec<String>,
    pub poe_sharing_enabled: bool,
    pub full_communication_enabled: bool,
    pub requires_biso_agreement: bool,
    pub compliance_reporting_endpoint: Option<String>,
}

/// Agreement status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgreementStatus {
    Draft,
    Active,
    Suspended,
    Expired,
    Revoked,
    UnderReview,
}

/// Compliance report structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub report_id: Uuid,
    pub agreement_id: Uuid,
    pub report_type: ComplianceReportType,
    pub generated_at: DateTime<Utc>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub compliance_status: ComplianceStatus,
    pub violations: Vec<ComplianceViolation>,
    pub metrics: ComplianceMetrics,
    pub audit_trail: Vec<AuditEvent>,
}

/// Types of compliance reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceReportType {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annual,
    Triggered,
    Audit,
}

/// Compliance status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    PartiallyCompliant,
    UnderReview,
    RequiresAction,
}

/// Compliance violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    pub violation_id: Uuid,
    pub violation_type: String,
    pub severity: ViolationSeverity,
    pub description: String,
    pub detected_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub remediation_actions: Vec<String>,
}

/// Violation severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Compliance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceMetrics {
    pub total_transactions: u64,
    pub compliant_transactions: u64,
    pub violation_count: u64,
    pub api_calls_made: u64,
    pub poe_shared_count: u64,
    pub compliance_score: f64,
}

/// Audit events for compliance tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub event_id: Uuid,
    pub event_type: String,
    pub timestamp: DateTime<Utc>,
    pub actor: String,
    pub action: String,
    pub resource: String,
    pub outcome: String,
    pub metadata: HashMap<String, String>,
}

/// Developer Configuration Builder for BISO Agreements
#[derive(Debug, Clone, Default)]
pub struct BisoAgreementBuilder {
    wallet_id: Option<String>,
    agreement_type: Option<BisoAgreementType>,
    cue_rules: Vec<CueBasedRule>,
    compliance_requirements: Option<ComplianceRequirements>,
    communication_policy: Option<CommunicationPolicy>,
    expires_at: Option<DateTime<Utc>>,
}

impl BisoAgreementBuilder {
    /// Create a new BISO Agreement builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the wallet ID for the agreement
    pub fn wallet_id(mut self, wallet_id: impl Into<String>) -> Self {
        self.wallet_id = Some(wallet_id.into());
        self
    }

    /// Set the agreement type (Government, Bank, Enterprise, etc.)
    pub fn agreement_type(mut self, agreement_type: BisoAgreementType) -> Self {
        self.agreement_type = Some(agreement_type);
        self
    }

    /// Add a custom cue-based rule
    pub fn add_cue_rule(mut self, rule: CueBasedRule) -> Self {
        self.cue_rules.push(rule);
        self
    }

    /// Add a transaction volume trigger rule
    pub fn add_volume_rule(mut self, threshold: u64, action: RequiredAction, enforcement: EnforcementLevel) -> Self {
        let rule = CueBasedRule {
            rule_id: format!("volume_rule_{}", Uuid::new_v4()),
            cue_trigger: CueTrigger::TransactionVolume { threshold },
            required_action: action,
            compliance_check: ComplianceCheck::RegulatoryCompliance { 
                framework: "Volume Monitoring".to_string() 
            },
            enforcement_level: enforcement,
        };
        self.cue_rules.push(rule);
        self
    }

    /// Add a geographic access rule
    pub fn add_geographic_rule(mut self, regions: Vec<String>, action: RequiredAction, enforcement: EnforcementLevel) -> Self {
        let rule = CueBasedRule {
            rule_id: format!("geo_rule_{}", Uuid::new_v4()),
            cue_trigger: CueTrigger::GeographicAccess { regions },
            required_action: action,
            compliance_check: ComplianceCheck::RegulatoryCompliance { 
                framework: "Geographic Compliance".to_string() 
            },
            enforcement_level: enforcement,
        };
        self.cue_rules.push(rule);
        self
    }

    /// Add a time-based rule
    pub fn add_time_rule(mut self, interval_hours: u64, action: RequiredAction, enforcement: EnforcementLevel) -> Self {
        let rule = CueBasedRule {
            rule_id: format!("time_rule_{}", Uuid::new_v4()),
            cue_trigger: CueTrigger::TimeInterval { interval_hours },
            required_action: action,
            compliance_check: ComplianceCheck::RegulatoryCompliance { 
                framework: "Time-based Monitoring".to_string() 
            },
            enforcement_level: enforcement,
        };
        self.cue_rules.push(rule);
        self
    }

    /// Add a custom rule with developer-defined parameters
    pub fn add_custom_rule(
        mut self, 
        trigger_type: impl Into<String>, 
        trigger_params: HashMap<String, String>,
        action_type: impl Into<String>,
        action_params: HashMap<String, String>,
        enforcement: EnforcementLevel
    ) -> Self {
        let rule = CueBasedRule {
            rule_id: format!("custom_rule_{}", Uuid::new_v4()),
            cue_trigger: CueTrigger::Custom { 
                trigger_type: trigger_type.into(), 
                parameters: trigger_params 
            },
            required_action: RequiredAction::Custom { 
                action_type: action_type.into(), 
                parameters: action_params 
            },
            compliance_check: ComplianceCheck::Custom { 
                check_type: "Developer Defined".to_string(), 
                criteria: HashMap::new() 
            },
            enforcement_level: enforcement,
        };
        self.cue_rules.push(rule);
        self
    }

    /// Set compliance requirements
    pub fn compliance_requirements(mut self, requirements: ComplianceRequirements) -> Self {
        self.compliance_requirements = Some(requirements);
        self
    }

    /// Set communication policy
    pub fn communication_policy(mut self, policy: CommunicationPolicy) -> Self {
        self.communication_policy = Some(policy);
        self
    }

    /// Set expiration date
    pub fn expires_at(mut self, expires_at: DateTime<Utc>) -> Self {
        self.expires_at = Some(expires_at);
        self
    }

    /// Build the BISO Agreement
    pub fn build(self) -> Result<BisoAgreement> {
        let wallet_id = self.wallet_id.ok_or_else(|| anyhow!("Wallet ID is required"))?;
        let agreement_type = self.agreement_type.ok_or_else(|| anyhow!("Agreement type is required"))?;
        
        let compliance_requirements = self.compliance_requirements.unwrap_or(ComplianceRequirements {
            mandatory_reporting: true,
            audit_trail_required: true,
            data_retention_days: 365,
            encryption_required: true,
            geographic_restrictions: Vec::new(),
            regulatory_frameworks: Vec::new(),
        });

        let communication_policy = self.communication_policy.unwrap_or(CommunicationPolicy {
            allowed_apis: Vec::new(),
            blocked_apis: Vec::new(),
            poe_sharing_enabled: true,
            full_communication_enabled: true,
            requires_biso_agreement: true,
            compliance_reporting_endpoint: None,
        });

        Ok(BisoAgreement {
            agreement_id: Uuid::new_v4(),
            wallet_id,
            agreement_type,
            cue_based_rules: self.cue_rules,
            compliance_requirements,
            communication_policy,
            created_at: Utc::now(),
            expires_at: self.expires_at,
            status: AgreementStatus::Active,
            signature: None,
            compliance_reports: Vec::new(),
        })
    }
}

/// BISO Agreement Manager for handling stamped BPI wallets
#[derive(Debug, Clone)]
pub struct BisoAgreementManager {
    pub agreements: Arc<RwLock<HashMap<String, BisoAgreement>>>,
    pub compliance_reports: Arc<RwLock<HashMap<Uuid, ComplianceReport>>>,
    pub audit_events: Arc<RwLock<Vec<AuditEvent>>>,
}

impl BisoAgreementManager {
    /// Create a new BISO Agreement Manager
    pub fn new() -> Self {
        Self {
            agreements: Arc::new(RwLock::new(HashMap::new())),
            compliance_reports: Arc::new(RwLock::new(HashMap::new())),
            audit_events: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Create a new BISO Agreement builder for developers
    pub fn create_agreement_builder() -> BisoAgreementBuilder {
        BisoAgreementBuilder::new()
    }

    /// Register a custom BISO Agreement created by developers
    pub async fn register_custom_agreement(&self, agreement: BisoAgreement) -> Result<Uuid> {
        let agreement_id = agreement.agreement_id;
        let wallet_id = agreement.wallet_id.clone();
        
        info!("🔧 Registering custom BISO agreement for wallet: {} with {} cue rules", 
              wallet_id, agreement.cue_based_rules.len());

        // Store the agreement
        let mut agreements = self.agreements.write().await;
        agreements.insert(wallet_id.clone(), agreement.clone());
        drop(agreements);

        // Log audit event
        self.log_audit_event(
            "AgreementCreated".to_string(),
            wallet_id,
            format!("Custom BISO agreement registered with {} cue-based rules", agreement.cue_based_rules.len()),
            "BISO Agreement Registration".to_string(),
            "Success".to_string(),
        ).await;

        info!("✅ Custom BISO agreement registered: {}", agreement_id);
        Ok(agreement_id)
    }

    /// Create a BISO agreement for a stamped wallet
    pub async fn create_agreement(
        &self,
        wallet_id: String,
        agreement_type: BisoAgreementType,
    ) -> Result<BisoAgreement> {
        info!("🤝 Creating BISO agreement for wallet: {}", wallet_id);

        let communication_policy = self.determine_communication_policy(&agreement_type);
        let compliance_requirements = self.determine_compliance_requirements(&agreement_type);
        let cue_based_rules = self.generate_cue_based_rules(&agreement_type);

        let agreement = BisoAgreement {
            agreement_id: Uuid::new_v4(),
            wallet_id: wallet_id.clone(),
            agreement_type: agreement_type.clone(),
            cue_based_rules,
            compliance_requirements,
            communication_policy,
            created_at: Utc::now(),
            expires_at: self.calculate_expiry(&agreement_type),
            status: AgreementStatus::Active,
            signature: None,
            compliance_reports: Vec::new(),
        };

        // Store the agreement
        let mut agreements = self.agreements.write().await;
        agreements.insert(wallet_id.clone(), agreement.clone());

        // Log audit event
        self.log_audit_event(
            "agreement_created".to_string(),
            wallet_id.clone(),
            "create_agreement".to_string(),
            format!("agreement_{}", agreement.agreement_id),
            "success".to_string(),
        ).await;

        info!("✅ BISO agreement created: {}", agreement.agreement_id);
        Ok(agreement)
    }

    /// Evaluate communication permissions for a wallet
    pub async fn evaluate_communication_permission(
        &self,
        wallet_id: &str,
        api_endpoint: &str,
        operation_type: &str,
    ) -> Result<CommunicationPermission> {
        debug!("🔍 Evaluating communication permission for wallet: {} -> {}", wallet_id, api_endpoint);

        let agreements = self.agreements.read().await;
        let agreement = agreements.get(wallet_id);
        
        // Determine permission based on agreement
        let permission = if agreement.is_none() {
            warn!("No BISO agreement found for wallet: {}, assuming unstamped", wallet_id);
            if self.is_poe_endpoint(api_endpoint) {
                CommunicationPermission {
                    allowed: true,
                    access_level: AccessLevel::PoeOnly,
                    restrictions: vec!["POE sharing only - no BISO agreement".to_string()],
                    requires_compliance_report: true,
                }
            } else {
                CommunicationPermission {
                    allowed: false,
                    access_level: AccessLevel::None,
                    restrictions: vec!["No BISO agreement found - access denied".to_string()],
                    requires_compliance_report: true,
                }
            }
        } else {
            let agreement = agreement.unwrap();

            match &agreement.agreement_type {
                BisoAgreementType::GovernmentStamped { api_access_level, .. } |
                BisoAgreementType::BankStamped { api_access_level, .. } => {
                    match api_access_level {
                        ApiAccessLevel::Full { .. } => {
                            // Government and bank stamps get full API access
                            CommunicationPermission {
                                allowed: true,
                                access_level: AccessLevel::Full,
                                restrictions: Vec::new(),
                                requires_compliance_report: true,
                            }
                        }
                        _ => self.evaluate_restricted_access(agreement, api_endpoint, operation_type),
                    }
                }
                BisoAgreementType::OtherStamped { restrictions: _, .. } |
                BisoAgreementType::Unstamped { .. } => {
                    // Other stamps and unstamped only get POE sharing
                    if self.is_poe_endpoint(api_endpoint) {
                        CommunicationPermission {
                            allowed: true,
                            access_level: AccessLevel::PoeOnly,
                            restrictions: vec!["POE sharing only".to_string()],
                            requires_compliance_report: true,
                        }
                    } else {
                        CommunicationPermission {
                            allowed: false,
                            access_level: AccessLevel::None,
                            restrictions: vec!["Only POE sharing allowed for non-government/bank stamps".to_string()],
                            requires_compliance_report: true,
                        }
                    }
                }
            }
        };

        // Check cue-based rules (only if we have an agreement)
        if let Some(agreement) = agreements.get(wallet_id) {
            let _cue_evaluation = self.evaluate_cue_based_rules(agreement, api_endpoint, operation_type).await?;
        }
        
        // Log audit event
        self.log_audit_event(
            "communication_evaluated".to_string(),
            wallet_id.to_string(),
            operation_type.to_string(),
            api_endpoint.to_string(),
            if permission.allowed { "allowed" } else { "denied" }.to_string(),
        ).await;

        Ok(permission)
    }

    /// Generate compliance report for an agreement
    pub async fn generate_compliance_report(
        &self,
        agreement_id: Uuid,
        report_type: ComplianceReportType,
    ) -> Result<ComplianceReport> {
        info!("📊 Generating compliance report for agreement: {}", agreement_id);

        let agreements = self.agreements.read().await;
        let agreement = agreements.values()
            .find(|a| a.agreement_id == agreement_id)
            .ok_or_else(|| anyhow!("Agreement not found: {}", agreement_id))?;

        let report = ComplianceReport {
            report_id: Uuid::new_v4(),
            agreement_id,
            report_type,
            generated_at: Utc::now(),
            period_start: Utc::now() - chrono::Duration::days(30), // Last 30 days
            period_end: Utc::now(),
            compliance_status: self.calculate_compliance_status(agreement).await,
            violations: self.collect_violations(agreement).await,
            metrics: self.calculate_compliance_metrics(agreement).await,
            audit_trail: self.get_audit_trail(agreement).await,
        };

        // Store the report
        let mut reports = self.compliance_reports.write().await;
        reports.insert(report.report_id, report.clone());

        info!("✅ Compliance report generated: {}", report.report_id);
        Ok(report)
    }

    /// Helper methods
    fn determine_communication_policy(&self, agreement_type: &BisoAgreementType) -> CommunicationPolicy {
        match agreement_type {
            BisoAgreementType::GovernmentStamped { .. } => CommunicationPolicy {
                allowed_apis: vec![
                    "/api/government/*".to_string(),
                    "/api/bank/*".to_string(),
                    "/api/poe/*".to_string(),
                    "/api/compliance/*".to_string(),
                ],
                blocked_apis: Vec::new(),
                poe_sharing_enabled: true,
                full_communication_enabled: true,
                requires_biso_agreement: true,
                compliance_reporting_endpoint: Some("/api/compliance/government".to_string()),
            },
            BisoAgreementType::BankStamped { .. } => CommunicationPolicy {
                allowed_apis: vec![
                    "/api/bank/*".to_string(),
                    "/api/poe/*".to_string(),
                    "/api/compliance/*".to_string(),
                ],
                blocked_apis: Vec::new(),
                poe_sharing_enabled: true,
                full_communication_enabled: true,
                requires_biso_agreement: true,
                compliance_reporting_endpoint: Some("/api/compliance/bank".to_string()),
            },
            _ => CommunicationPolicy {
                allowed_apis: vec!["/api/poe/*".to_string()],
                blocked_apis: vec![
                    "/api/bank/*".to_string(),
                    "/api/government/*".to_string(),
                ],
                poe_sharing_enabled: true,
                full_communication_enabled: false,
                requires_biso_agreement: true,
                compliance_reporting_endpoint: Some("/api/compliance/general".to_string()),
            },
        }
    }

    fn determine_compliance_requirements(&self, agreement_type: &BisoAgreementType) -> ComplianceRequirements {
        match agreement_type {
            BisoAgreementType::GovernmentStamped { .. } => ComplianceRequirements {
                mandatory_reporting: true,
                audit_trail_required: true,
                data_retention_days: 2555, // 7 years
                encryption_required: true,
                geographic_restrictions: Vec::new(),
                regulatory_frameworks: vec!["SOX".to_string(), "GDPR".to_string(), "HIPAA".to_string()],
            },
            BisoAgreementType::BankStamped { .. } => ComplianceRequirements {
                mandatory_reporting: true,
                audit_trail_required: true,
                data_retention_days: 2555, // 7 years
                encryption_required: true,
                geographic_restrictions: Vec::new(),
                regulatory_frameworks: vec!["PCI-DSS".to_string(), "Basel III".to_string(), "GDPR".to_string()],
            },
            _ => ComplianceRequirements {
                mandatory_reporting: true,
                audit_trail_required: true,
                data_retention_days: 365, // 1 year
                encryption_required: true,
                geographic_restrictions: Vec::new(),
                regulatory_frameworks: vec!["GDPR".to_string()],
            },
        }
    }

    fn generate_cue_based_rules(&self, agreement_type: &BisoAgreementType) -> Vec<CueBasedRule> {
        let mut rules = Vec::new();

        // Volume-based rule
        rules.push(CueBasedRule {
            rule_id: "volume_threshold".to_string(),
            cue_trigger: CueTrigger::TransactionVolume { threshold: 10000 },
            required_action: RequiredAction::GenerateComplianceReport,
            compliance_check: ComplianceCheck::AmlScreening,
            enforcement_level: EnforcementLevel::Warning,
        });

        // Time-based reporting rule
        rules.push(CueBasedRule {
            rule_id: "periodic_reporting".to_string(),
            cue_trigger: CueTrigger::TimeInterval { interval_hours: 24 },
            required_action: RequiredAction::GenerateComplianceReport,
            compliance_check: ComplianceCheck::RegulatoryCompliance { framework: "BISO".to_string() },
            enforcement_level: EnforcementLevel::Advisory,
        });

        match agreement_type {
            BisoAgreementType::GovernmentStamped { .. } => {
                rules.push(CueBasedRule {
                    rule_id: "government_escalation".to_string(),
                    cue_trigger: CueTrigger::DataClassification { level: "classified".to_string() },
                    required_action: RequiredAction::EscalateToAuthority { authority_type: "government".to_string() },
                    compliance_check: ComplianceCheck::Custom { 
                        check_type: "security_clearance".to_string(),
                        criteria: HashMap::new(),
                    },
                    enforcement_level: EnforcementLevel::Blocking,
                });
            }
            BisoAgreementType::BankStamped { .. } => {
                rules.push(CueBasedRule {
                    rule_id: "banking_compliance".to_string(),
                    cue_trigger: CueTrigger::TransactionVolume { threshold: 50000 },
                    required_action: RequiredAction::RequireAuthentication,
                    compliance_check: ComplianceCheck::AmlScreening,
                    enforcement_level: EnforcementLevel::Blocking,
                });
            }
            _ => {}
        }

        rules
    }

    fn calculate_expiry(&self, agreement_type: &BisoAgreementType) -> Option<DateTime<Utc>> {
        match agreement_type {
            BisoAgreementType::GovernmentStamped { .. } |
            BisoAgreementType::BankStamped { .. } => {
                Some(Utc::now() + chrono::Duration::days(365)) // 1 year
            }
            _ => {
                Some(Utc::now() + chrono::Duration::days(90)) // 90 days
            }
        }
    }

    fn evaluate_restricted_access(
        &self,
        _agreement: &BisoAgreement,
        _api_endpoint: &str,
        _operation_type: &str,
    ) -> CommunicationPermission {
        CommunicationPermission {
            allowed: false,
            access_level: AccessLevel::None,
            restrictions: vec!["Restricted access".to_string()],
            requires_compliance_report: true,
        }
    }

    fn is_poe_endpoint(&self, api_endpoint: &str) -> bool {
        api_endpoint.contains("/api/poe/") || api_endpoint.contains("/proof/")
    }

    async fn evaluate_cue_based_rules(
        &self,
        agreement: &BisoAgreement,
        _api_endpoint: &str,
        _operation_type: &str,
    ) -> Result<Vec<String>> {
        let mut triggered_rules = Vec::new();

        for rule in &agreement.cue_based_rules {
            // Simplified cue evaluation - in production this would be more sophisticated
            match &rule.cue_trigger {
                CueTrigger::TransactionVolume { threshold } => {
                    // Check if volume threshold is exceeded
                    if *threshold > 5000 { // Simplified check
                        triggered_rules.push(format!("Volume threshold rule triggered: {}", rule.rule_id));
                    }
                }
                CueTrigger::TimeInterval { .. } => {
                    // Time-based rules always trigger for demonstration
                    triggered_rules.push(format!("Time interval rule triggered: {}", rule.rule_id));
                }
                _ => {}
            }
        }

        Ok(triggered_rules)
    }

    async fn calculate_compliance_status(&self, _agreement: &BisoAgreement) -> ComplianceStatus {
        // Simplified compliance calculation
        ComplianceStatus::Compliant
    }

    async fn collect_violations(&self, _agreement: &BisoAgreement) -> Vec<ComplianceViolation> {
        // Return empty violations for now
        Vec::new()
    }

    async fn calculate_compliance_metrics(&self, _agreement: &BisoAgreement) -> ComplianceMetrics {
        ComplianceMetrics {
            total_transactions: 1000,
            compliant_transactions: 950,
            violation_count: 0,
            api_calls_made: 500,
            poe_shared_count: 100,
            compliance_score: 0.95,
        }
    }

    async fn get_audit_trail(&self, agreement: &BisoAgreement) -> Vec<AuditEvent> {
        let events = self.audit_events.read().await;
        events.iter()
            .filter(|event| event.resource.contains(&agreement.agreement_id.to_string()))
            .cloned()
            .collect()
    }

    async fn log_audit_event(
        &self,
        event_type: String,
        actor: String,
        action: String,
        resource: String,
        outcome: String,
    ) {
        let event = AuditEvent {
            event_id: Uuid::new_v4(),
            event_type,
            timestamp: Utc::now(),
            actor,
            action,
            resource,
            outcome,
            metadata: HashMap::new(),
        };

        let mut events = self.audit_events.write().await;
        events.push(event);
    }
}

/// Communication permission result
#[derive(Debug, Clone)]
pub struct CommunicationPermission {
    pub allowed: bool,
    pub access_level: AccessLevel,
    pub restrictions: Vec<String>,
    pub requires_compliance_report: bool,
}

/// Access levels for communication
#[derive(Debug, Clone, PartialEq)]
pub enum AccessLevel {
    Full,
    PoeOnly,
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_government_stamped_agreement() {
        let manager = BisoAgreementManager::new();
        
        let agreement_type = BisoAgreementType::GovernmentStamped {
            government_id: "US-GOV-001".to_string(),
            jurisdiction: "United States".to_string(),
            compliance_level: ComplianceLevel::Government,
            api_access_level: ApiAccessLevel::Full {
                bank_api: true,
                government_api: true,
                cross_system_communication: true,
            },
        };

        let agreement = manager.create_agreement("gov_wallet_001".to_string(), agreement_type).await.unwrap();
        assert_eq!(agreement.status, AgreementStatus::Active);
        assert!(agreement.communication_policy.full_communication_enabled);
    }

    #[tokio::test]
    async fn test_unstamped_wallet_restrictions() {
        let manager = BisoAgreementManager::new();
        
        let agreement_type = BisoAgreementType::Unstamped {
            wallet_id: "unstamped_001".to_string(),
            mandatory_biso: true,
        };

        let agreement = manager.create_agreement("unstamped_001".to_string(), agreement_type).await.unwrap();
        
        // Test POE endpoint access (should be allowed)
        let poe_permission = manager.evaluate_communication_permission(
            "unstamped_001",
            "/api/poe/share",
            "share_proof"
        ).await.unwrap();
        assert!(poe_permission.allowed);
        assert_eq!(poe_permission.access_level, AccessLevel::PoeOnly);

        // Test bank API access (should be denied)
        let bank_permission = manager.evaluate_communication_permission(
            "unstamped_001",
            "/api/bank/settlement",
            "initiate_settlement"
        ).await.unwrap();
        assert!(!bank_permission.allowed);
    }

    #[tokio::test]
    async fn test_compliance_report_generation() {
        let manager = BisoAgreementManager::new();
        
        let agreement_type = BisoAgreementType::BankStamped {
            bank_id: "BANK-001".to_string(),
            banking_license: "US-BANKING-LIC-001".to_string(),
            compliance_level: ComplianceLevel::Banking,
            api_access_level: ApiAccessLevel::Full {
                bank_api: true,
                government_api: false,
                cross_system_communication: true,
            },
        };

        let agreement = manager.create_agreement("bank_wallet_001".to_string(), agreement_type).await.unwrap();
        
        let report = manager.generate_compliance_report(
            agreement.agreement_id,
            ComplianceReportType::Daily
        ).await.unwrap();
        
        assert_eq!(report.agreement_id, agreement.agreement_id);
        assert_eq!(report.compliance_status, ComplianceStatus::Compliant);
    }
}
