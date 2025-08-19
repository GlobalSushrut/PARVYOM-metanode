use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Multi-tier authority system supporting Bank, Community, and Hybrid authority levels
/// Enables different levels of trust and verification based on node requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthorityLevel {
    /// Community-based authority with peer verification
    Community {
        /// Basic identity verification completed
        basic_verification: bool,
        /// Number of community vouchers received
        community_vouching: u32,
        /// Community reputation score
        reputation_score: u32,
        /// Years of community participation
        participation_years: u32,
        /// Community roles held
        roles: Vec<CommunityRole>,
    },
    
    /// Bank-sponsored authority with regulatory compliance
    Bank {
        /// KYC verification completed
        kyc_verified: bool,
        /// AML compliance verified
        aml_compliant: bool,
        /// Regulatory approvals obtained
        regulatory_approval: Vec<String>,
        /// Sponsoring bank information
        sponsoring_bank: BankInfo,
        /// Banking compliance level
        compliance_level: BankingComplianceLevel,
        /// Regulatory audit trail
        audit_trail: Vec<RegulatoryAudit>,
    },
    
    /// Hybrid authority combining bank sponsorship with community operation
    Hybrid {
        /// Bank authority component
        bank_authority: Box<AuthorityLevel>,
        /// Community authority component
        community_authority: Box<AuthorityLevel>,
        /// Hybrid governance model
        governance_model: HybridGovernance,
        /// Authority balance (0.0 = full community, 1.0 = full bank)
        authority_balance: f64,
        /// Conflict resolution mechanism
        conflict_resolution: ConflictResolution,
    },
}

impl AuthorityLevel {
    /// Get the trust score for this authority level
    pub fn trust_score(&self) -> u32 {
        match self {
            AuthorityLevel::Community { reputation_score, community_vouching, participation_years, .. } => {
                let base_score = *reputation_score;
                let voucher_bonus = (*community_vouching * 10).min(200);
                let experience_bonus = (*participation_years * 50).min(300);
                (base_score + voucher_bonus + experience_bonus).min(1000)
            },
            AuthorityLevel::Bank { kyc_verified, aml_compliant, compliance_level, .. } => {
                let mut score = 500; // Base bank score
                if *kyc_verified { score += 200; }
                if *aml_compliant { score += 200; }
                score += match compliance_level {
                    BankingComplianceLevel::Basic => 0,
                    BankingComplianceLevel::Standard => 50,
                    BankingComplianceLevel::Enhanced => 100,
                    BankingComplianceLevel::Enterprise => 150,
                };
                score.min(1000)
            },
            AuthorityLevel::Hybrid { bank_authority, community_authority, authority_balance, .. } => {
                let bank_score = bank_authority.trust_score() as f64;
                let community_score = community_authority.trust_score() as f64;
                let weighted_score = (bank_score * authority_balance) + (community_score * (1.0 - authority_balance));
                (weighted_score as u32).min(1000)
            },
        }
    }

    /// Check if authority level meets minimum requirements for a specific operation
    pub fn meets_requirements(&self, requirements: &AuthorityRequirements) -> bool {
        match requirements {
            AuthorityRequirements::MinimumTrust { min_score } => {
                self.trust_score() >= *min_score
            },
            AuthorityRequirements::BankingCompliance { level } => {
                match self {
                    AuthorityLevel::Bank { compliance_level, .. } => {
                        compliance_level >= level
                    },
                    AuthorityLevel::Hybrid { bank_authority, .. } => {
                        bank_authority.meets_requirements(requirements)
                    },
                    _ => false,
                }
            },
            AuthorityRequirements::CommunityVouching { min_vouchers } => {
                match self {
                    AuthorityLevel::Community { community_vouching, .. } => {
                        community_vouching >= min_vouchers
                    },
                    AuthorityLevel::Hybrid { community_authority, .. } => {
                        community_authority.meets_requirements(requirements)
                    },
                    _ => false,
                }
            },
            AuthorityRequirements::RegulatoryApproval { required_approvals } => {
                match self {
                    AuthorityLevel::Bank { regulatory_approval, .. } => {
                        required_approvals.iter().all(|req| regulatory_approval.contains(req))
                    },
                    AuthorityLevel::Hybrid { bank_authority, .. } => {
                        bank_authority.meets_requirements(requirements)
                    },
                    _ => false,
                }
            },
        }
    }
}

/// Community roles that can be held by community members
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunityRole {
    /// Regular community member
    Member,
    /// Community moderator
    Moderator,
    /// Technical contributor
    Developer,
    /// Community validator
    Validator,
    /// Governance participant
    Governor,
    /// Community leader
    Leader,
    /// Dispute resolver
    Arbiter,
}

/// Banking information for bank-sponsored nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankInfo {
    /// Bank name
    pub name: String,
    /// Bank identifier (SWIFT, routing number, etc.)
    pub identifier: String,
    /// Bank type (commercial, investment, central, etc.)
    pub bank_type: BankType,
    /// Regulatory jurisdiction
    pub jurisdiction: String,
    /// Banking licenses held
    pub licenses: Vec<BankingLicense>,
    /// Contact information
    pub contact: ContactInfo,
    /// Sponsorship start date
    pub sponsorship_start: DateTime<Utc>,
    /// Sponsorship level
    pub sponsorship_level: SponsorshipLevel,
}

/// Types of banking institutions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BankType {
    Commercial,
    Investment,
    Central,
    CommunityBank,
    CreditUnion,
    OnlineBank,
    Neobank,
}

/// Banking licenses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankingLicense {
    pub license_type: String,
    pub issuing_authority: String,
    pub license_number: String,
    pub issued_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
    pub status: LicenseStatus,
}

/// License status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LicenseStatus {
    Active,
    Suspended,
    Revoked,
    Expired,
    Pending,
}

/// Banking compliance levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq)]
pub enum BankingComplianceLevel {
    Basic,
    Standard,
    Enhanced,
    Enterprise,
}

/// Sponsorship levels for bank-sponsored nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SponsorshipLevel {
    Bronze {
        max_nodes: u32,
        support_level: SupportLevel,
    },
    Silver {
        max_nodes: u32,
        support_level: SupportLevel,
        priority_access: bool,
    },
    Gold {
        max_nodes: u32,
        support_level: SupportLevel,
        priority_access: bool,
        custom_features: Vec<String>,
    },
    Platinum {
        max_nodes: u32,
        support_level: SupportLevel,
        priority_access: bool,
        custom_features: Vec<String>,
        dedicated_support: bool,
    },
}

/// Support levels provided by sponsors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SupportLevel {
    Basic,
    Standard,
    Premium,
    Enterprise,
}

/// Regulatory audit entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryAudit {
    pub audit_type: AuditType,
    pub auditor: String,
    pub audit_date: DateTime<Utc>,
    pub findings: Vec<AuditFinding>,
    pub compliance_score: u32,
    pub recommendations: Vec<String>,
    pub next_audit_due: DateTime<Utc>,
}

/// Types of regulatory audits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditType {
    Kyc,
    Aml,
    Gdpr,
    Hipaa,
    PciDss,
    Sox,
    Sec,
    Cftc,
    Internal,
    External,
}

/// Audit findings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditFinding {
    pub severity: FindingSeverity,
    pub category: String,
    pub description: String,
    pub remediation_required: bool,
    pub remediation_deadline: Option<DateTime<Utc>>,
    pub status: FindingStatus,
}

/// Severity levels for audit findings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FindingSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Status of audit findings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FindingStatus {
    Open,
    InProgress,
    Resolved,
    Accepted,
    Deferred,
}

/// Hybrid governance models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HybridGovernance {
    /// Bank has majority control
    BankMajority {
        bank_voting_power: f64,
        community_voting_power: f64,
    },
    /// Community has majority control
    CommunityMajority {
        bank_voting_power: f64,
        community_voting_power: f64,
    },
    /// Equal voting power
    Equal {
        tie_breaker: TieBreaker,
    },
    /// Weighted by stake/contribution
    Weighted {
        bank_weight: f64,
        community_weight: f64,
        stake_multiplier: f64,
    },
}

/// Tie breaker mechanisms for equal governance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TieBreaker {
    BankDecides,
    CommunityDecides,
    RandomSelection,
    ExternalArbiter { arbiter_id: String },
    StatusQuo, // No change if tied
}

/// Conflict resolution mechanisms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// Escalate to higher authority
    Escalation {
        escalation_path: Vec<String>,
    },
    /// Use arbitration service
    Arbitration {
        arbiter_pool: Vec<String>,
        arbitration_rules: String,
    },
    /// Community voting
    CommunityVote {
        quorum_threshold: f64,
        majority_threshold: f64,
    },
    /// Automatic resolution based on rules
    Automated {
        resolution_rules: Vec<ResolutionRule>,
    },
}

/// Automated resolution rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionRule {
    pub condition: String,
    pub action: String,
    pub priority: u32,
}

/// Authority requirements for specific operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthorityRequirements {
    /// Minimum trust score required
    MinimumTrust { min_score: u32 },
    /// Banking compliance level required
    BankingCompliance { level: BankingComplianceLevel },
    /// Community vouching required
    CommunityVouching { min_vouchers: u32 },
    /// Regulatory approval required
    RegulatoryApproval { required_approvals: Vec<String> },
}

/// Contact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub website: Option<String>,
    pub contact_person: Option<String>,
    pub emergency_contact: Option<String>,
}

/// Authority record for registry storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorityRecord {
    pub authority_level: AuthorityLevel,
    pub linked_nodes: Vec<String>,
    pub status: AuthorityStatus,
    pub created_at: DateTime<Utc>,
    pub last_verified: DateTime<Utc>,
    pub verification_history: Vec<VerificationEvent>,
}

/// Authority status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthorityStatus {
    Active,
    Suspended,
    Revoked,
    UnderReview,
    Expired,
}

/// Verification events for authority records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationEvent {
    pub event_type: VerificationEventType,
    pub verifier: String,
    pub timestamp: DateTime<Utc>,
    pub details: String,
    pub signature: String,
}

/// Types of verification events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VerificationEventType {
    InitialVerification,
    Renewal,
    Upgrade,
    Downgrade,
    Suspension,
    Reinstatement,
    Revocation,
}

/// Authority manager for handling authority operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorityManager {
    pub authority_records: HashMap<String, AuthorityRecord>,
    pub verification_providers: HashMap<String, VerificationProvider>,
    pub requirements_registry: HashMap<String, AuthorityRequirements>,
}

impl AuthorityManager {
    /// Create a new authority manager
    pub fn new() -> Self {
        Self {
            authority_records: HashMap::new(),
            verification_providers: HashMap::new(),
            requirements_registry: HashMap::new(),
        }
    }

    /// Register a new authority
    pub async fn register_authority(&mut self, 
        authority_id: String, 
        authority_level: AuthorityLevel
    ) -> Result<(), String> {
        let record = AuthorityRecord {
            authority_level,
            linked_nodes: Vec::new(),
            status: AuthorityStatus::Active,
            created_at: Utc::now(),
            last_verified: Utc::now(),
            verification_history: Vec::new(),
        };

        self.authority_records.insert(authority_id, record);
        Ok(())
    }

    /// Verify authority meets requirements
    pub async fn verify_authority(&self, 
        authority_id: &str, 
        operation: &str
    ) -> Result<bool, String> {
        let authority_record = self.authority_records.get(authority_id)
            .ok_or("Authority not found")?;

        if authority_record.status != AuthorityStatus::Active {
            return Ok(false);
        }

        if let Some(requirements) = self.requirements_registry.get(operation) {
            Ok(authority_record.authority_level.meets_requirements(requirements))
        } else {
            // No specific requirements, allow if authority is active
            Ok(true)
        }
    }

    /// Update authority status
    pub async fn update_authority_status(&mut self, 
        authority_id: &str, 
        new_status: AuthorityStatus,
        verifier: String
    ) -> Result<(), String> {
        let record = self.authority_records.get_mut(authority_id)
            .ok_or("Authority not found")?;

        let verification_event = VerificationEvent {
            event_type: match new_status {
                AuthorityStatus::Active => VerificationEventType::Reinstatement,
                AuthorityStatus::Suspended => VerificationEventType::Suspension,
                AuthorityStatus::Revoked => VerificationEventType::Revocation,
                AuthorityStatus::UnderReview => VerificationEventType::InitialVerification,
                AuthorityStatus::Expired => VerificationEventType::InitialVerification,
            },
            verifier,
            timestamp: Utc::now(),
            details: format!("Status changed to {:?}", new_status),
            signature: "placeholder_signature".to_string(),
        };

        record.status = new_status;
        record.last_verified = Utc::now();
        record.verification_history.push(verification_event);

        Ok(())
    }

    /// Get authority statistics
    pub fn get_authority_stats(&self) -> AuthorityStats {
        let mut stats = AuthorityStats::default();
        
        for (_, record) in &self.authority_records {
            match &record.authority_level {
                AuthorityLevel::Community { .. } => stats.community_authorities += 1,
                AuthorityLevel::Bank { .. } => stats.bank_authorities += 1,
                AuthorityLevel::Hybrid { .. } => stats.hybrid_authorities += 1,
            }

            match record.status {
                AuthorityStatus::Active => stats.active_authorities += 1,
                AuthorityStatus::Suspended => stats.suspended_authorities += 1,
                AuthorityStatus::Revoked => stats.revoked_authorities += 1,
                AuthorityStatus::UnderReview => stats.under_review_authorities += 1,
                AuthorityStatus::Expired => stats.expired_authorities += 1,
            }
        }

        stats.total_authorities = self.authority_records.len();
        stats
    }
}

/// Verification provider information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationProvider {
    pub provider_id: String,
    pub name: String,
    pub provider_type: VerificationProviderType,
    pub supported_verifications: Vec<VerificationType>,
    pub trust_score: u32,
    pub active: bool,
}

/// Types of verification providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationProviderType {
    Community,
    Bank,
    Government,
    ThirdParty,
    Automated,
}

/// Types of verifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationType {
    Identity,
    Kyc,
    Aml,
    Regulatory,
    Technical,
    Reputation,
}

/// Authority statistics
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AuthorityStats {
    pub total_authorities: usize,
    pub community_authorities: usize,
    pub bank_authorities: usize,
    pub hybrid_authorities: usize,
    pub active_authorities: usize,
    pub suspended_authorities: usize,
    pub revoked_authorities: usize,
    pub under_review_authorities: usize,
    pub expired_authorities: usize,
}
