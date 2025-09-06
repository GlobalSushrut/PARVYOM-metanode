use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::registry::geodid::{GeoDID, GeoScope};

/// StateWallet - Exactly one per state (geo-bound), constructed as:
/// StateWallet = CourtDID + 5×BPIWallets (independent cross-credibility)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateWallet {
    /// State identifier (ISO code or custom identifier)
    pub state_id: String,
    /// Geographic scope this StateWallet covers
    pub geo_scope: GeoScope,
    /// Court DID for judicial/arbiter access
    pub court_did: CourtDID,
    /// Five independent BPI wallets for cross-credibility
    pub bpi_wallets: [BpiWallet; 5],
    /// Wallet creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last verification timestamp
    pub last_verified: DateTime<Utc>,
    /// Wallet status
    pub status: StateWalletStatus,
}

/// Court DID for judicial and arbiter functions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtDID {
    /// Court DID identifier
    pub did: String,
    /// Court name and jurisdiction
    pub court_name: String,
    /// Judicial authority level
    pub authority_level: JudicialAuthority,
    /// Court type (supreme, appellate, district, etc.)
    pub court_type: CourtType,
    /// Geographic jurisdiction
    pub jurisdiction: GeoScope,
    /// Read-only access to integrity proofs
    pub integrity_access: IntegrityAccess,
    /// Court establishment date
    pub established_at: DateTime<Utc>,
    /// Court verification status
    pub verification_status: CourtVerificationStatus,
}

/// BPI Wallet for independent cross-credibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpiWallet {
    /// Wallet identifier
    pub wallet_id: String,
    /// Wallet address (cryptographic)
    pub address: String,
    /// Organization operating this wallet
    pub organization: WalletOrganization,
    /// Wallet type and capabilities
    pub wallet_type: BpiWalletType,
    /// Independence verification
    pub independence: IndependenceProof,
    /// Wallet creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last activity timestamp
    pub last_active: DateTime<Utc>,
    /// Wallet status
    pub status: WalletStatus,
}

/// Judicial authority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JudicialAuthority {
    /// Supreme court level (highest authority)
    Supreme,
    /// Constitutional court
    Constitutional,
    /// Appellate court
    Appellate,
    /// District/Regional court
    District,
    /// Specialized court (tax, administrative, etc.)
    Specialized { specialty: String },
    /// Arbitration panel
    Arbitration,
}

/// Types of courts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CourtType {
    /// Supreme Court
    Supreme,
    /// Constitutional Court
    Constitutional,
    /// Court of Appeals
    Appeals,
    /// District Court
    District,
    /// Family Court
    Family,
    /// Commercial Court
    Commercial,
    /// Administrative Court
    Administrative,
    /// Tax Court
    Tax,
    /// Arbitration Panel
    Arbitration,
    /// International Court
    International,
}

/// Integrity access configuration for courts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityAccess {
    /// Read-only access to pipeline integrity proofs
    pub pipeline_access: bool,
    /// Access to audit trails
    pub audit_access: bool,
    /// Access to cryptographic proofs
    pub crypto_proof_access: bool,
    /// Access level restrictions
    pub access_restrictions: Vec<AccessRestriction>,
    /// Last access timestamp
    pub last_accessed: Option<DateTime<Utc>>,
}

/// Access restrictions for court integrity access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessRestriction {
    /// Time-based restrictions
    TimeRestricted { start: DateTime<Utc>, end: DateTime<Utc> },
    /// Case-specific access
    CaseSpecific { case_id: String },
    /// Emergency access only
    EmergencyOnly,
    /// Requires additional authorization
    RequiresAuthorization { authority: String },
}

/// Court verification status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CourtVerificationStatus {
    /// Verified and active
    Verified,
    /// Pending verification
    Pending,
    /// Suspended
    Suspended,
    /// Revoked
    Revoked,
}

/// Organization operating a BPI wallet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletOrganization {
    /// Organization name
    pub name: String,
    /// Organization type
    pub org_type: OrganizationType,
    /// Legal registration details
    pub registration: LegalRegistration,
    /// Key custody information
    pub key_custody: KeyCustodyInfo,
    /// Independence verification
    pub independence_score: f64,
}

/// Types of organizations that can operate BPI wallets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrganizationType {
    /// Government agency
    Government { agency_type: String },
    /// Non-profit organization
    NonProfit { mission: String },
    /// Academic institution
    Academic { institution_type: String },
    /// Professional services firm
    Professional { service_type: String },
    /// Technology company
    Technology { specialization: String },
    /// Financial institution
    Financial { institution_type: String },
}

/// Legal registration information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalRegistration {
    /// Registration number
    pub registration_number: String,
    /// Jurisdiction of registration
    pub jurisdiction: String,
    /// Registration date
    pub registered_at: DateTime<Utc>,
    /// Regulatory oversight body
    pub oversight_body: String,
    /// License status
    pub license_status: LicenseStatus,
}

/// License status for organizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LicenseStatus {
    Active,
    Suspended,
    Revoked,
    Pending,
    Expired,
}

/// Key custody information for security
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyCustodyInfo {
    /// Custody model (single, multi-sig, HSM, etc.)
    pub custody_model: CustodyModel,
    /// Number of key holders
    pub key_holders: u32,
    /// Threshold for operations (m-of-n)
    pub threshold: u32,
    /// Hardware security module usage
    pub hsm_protected: bool,
    /// Geographic distribution of keys
    pub geographic_distribution: Vec<String>,
}

/// Key custody models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustodyModel {
    /// Single signature
    SingleSig,
    /// Multi-signature
    MultiSig { m: u32, n: u32 },
    /// Hardware Security Module
    HSM,
    /// Threshold signature scheme
    Threshold { threshold: u32, total: u32 },
    /// Distributed key generation
    DKG,
}

/// BPI Wallet types with different capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BpiWalletType {
    /// Primary governance wallet
    Primary {
        voting_power: u64,
        proposal_rights: bool,
    },
    /// Secondary backup wallet
    Secondary {
        backup_priority: u32,
    },
    /// Audit and transparency wallet
    Audit {
        audit_capabilities: Vec<AuditCapability>,
    },
    /// Emergency response wallet
    Emergency {
        emergency_powers: Vec<EmergencyPower>,
        activation_threshold: u32,
    },
    /// Cross-verification wallet
    CrossVerification {
        verification_scope: Vec<String>,
    },
}

/// Audit capabilities for audit wallets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditCapability {
    TransactionAudit,
    GovernanceAudit,
    ComplianceAudit,
    SecurityAudit,
    PerformanceAudit,
}

/// Emergency powers for emergency wallets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmergencyPower {
    /// Suspend operations
    Suspend,
    /// Override governance decisions
    Override,
    /// Emergency fund access
    EmergencyFunds,
    /// System recovery
    SystemRecovery,
    /// Crisis communication
    CrisisCommunication,
}

/// Independence proof for wallet organizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndependenceProof {
    /// Organizational diversity score
    pub org_diversity_score: f64,
    /// Key custody diversity score
    pub custody_diversity_score: f64,
    /// Geographic diversity score
    pub geographic_diversity_score: f64,
    /// Financial independence score
    pub financial_independence_score: f64,
    /// Overall independence score
    pub overall_score: f64,
    /// Last independence assessment
    pub assessed_at: DateTime<Utc>,
    /// Assessment authority
    pub assessed_by: String,
}

/// Wallet status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletStatus {
    Active,
    Inactive,
    Suspended,
    Compromised,
    UnderReview,
}

/// StateWallet status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateWalletStatus {
    /// Fully operational
    Operational,
    /// Partially operational (some wallets inactive)
    PartiallyOperational { active_wallets: u32 },
    /// Under maintenance
    Maintenance,
    /// Suspended due to security concerns
    Suspended,
    /// Compromised and needs reconstruction
    Compromised,
}

impl StateWallet {
    /// Create a new StateWallet
    pub fn new(
        state_id: String,
        geo_scope: GeoScope,
        court_did: CourtDID,
        bpi_wallets: [BpiWallet; 5],
    ) -> Result<Self, String> {
        // Validate independence constraints
        Self::validate_independence(&bpi_wallets)?;
        
        let now = Utc::now();
        Ok(Self {
            state_id,
            geo_scope,
            court_did,
            bpi_wallets,
            created_at: now,
            last_verified: now,
            status: StateWalletStatus::Operational,
        })
    }

    /// Validate independence constraints for the 5 BPI wallets
    fn validate_independence(wallets: &[BpiWallet; 5]) -> Result<(), String> {
        // Check organizational diversity
        let mut org_types = std::collections::HashSet::new();
        let mut custody_models = std::collections::HashSet::new();
        let mut jurisdictions = std::collections::HashSet::new();

        for wallet in wallets {
            // Organizational diversity
            let org_type_key = format!("{:?}", wallet.organization.org_type);
            org_types.insert(org_type_key);

            // Custody diversity
            let custody_key = format!("{:?}", wallet.organization.key_custody.custody_model);
            custody_models.insert(custody_key);

            // Jurisdictional diversity
            jurisdictions.insert(wallet.organization.registration.jurisdiction.clone());

            // Check minimum independence score
            if wallet.independence.overall_score < 0.6 {
                return Err(format!("Wallet {} has insufficient independence score: {}", 
                    wallet.wallet_id, wallet.independence.overall_score));
            }
        }

        // Require at least 3 different organization types
        if org_types.len() < 3 {
            return Err("Insufficient organizational diversity: need at least 3 different organization types".to_string());
        }

        // Require at least 2 different custody models
        if custody_models.len() < 2 {
            return Err("Insufficient custody diversity: need at least 2 different custody models".to_string());
        }

        // Require at least 2 different jurisdictions
        if jurisdictions.len() < 2 {
            return Err("Insufficient jurisdictional diversity: need at least 2 different jurisdictions".to_string());
        }

        Ok(())
    }

    /// Check if StateWallet is valid and operational
    pub fn is_valid(&self) -> bool {
        match self.status {
            StateWalletStatus::Operational => true,
            StateWalletStatus::PartiallyOperational { active_wallets } => active_wallets >= 3,
            _ => false,
        }
    }

    /// Get active BPI wallets
    pub fn get_active_wallets(&self) -> Vec<&BpiWallet> {
        self.bpi_wallets.iter()
            .filter(|wallet| matches!(wallet.status, WalletStatus::Active))
            .collect()
    }

    /// Update wallet status based on individual wallet states
    pub fn update_status(&mut self) {
        let active_count = self.get_active_wallets().len() as u32;
        
        self.status = match active_count {
            5 => StateWalletStatus::Operational,
            3..=4 => StateWalletStatus::PartiallyOperational { active_wallets: active_count },
            _ => StateWalletStatus::Suspended,
        };
        
        self.last_verified = Utc::now();
    }

    /// Verify court DID has proper integrity access
    pub fn verify_court_access(&self) -> bool {
        self.court_did.verification_status == CourtVerificationStatus::Verified &&
        self.court_did.integrity_access.pipeline_access &&
        self.court_did.integrity_access.audit_access
    }

    /// Calculate overall independence score
    pub fn calculate_independence_score(&self) -> f64 {
        let scores: Vec<f64> = self.bpi_wallets.iter()
            .map(|wallet| wallet.independence.overall_score)
            .collect();
        
        // Use harmonic mean to ensure all wallets maintain high independence
        let n = scores.len() as f64;
        let harmonic_sum: f64 = scores.iter().map(|&score| 1.0 / score.max(0.01)).sum();
        n / harmonic_sum
    }
}

impl CourtDID {
    /// Create a new CourtDID
    pub fn new(
        did: String,
        court_name: String,
        authority_level: JudicialAuthority,
        court_type: CourtType,
        jurisdiction: GeoScope,
    ) -> Self {
        Self {
            did,
            court_name,
            authority_level,
            court_type,
            jurisdiction,
            integrity_access: IntegrityAccess {
                pipeline_access: true,
                audit_access: true,
                crypto_proof_access: true,
                access_restrictions: Vec::new(),
                last_accessed: None,
            },
            established_at: Utc::now(),
            verification_status: CourtVerificationStatus::Pending,
        }
    }

    /// Verify court DID
    pub fn verify(&mut self, verifying_authority: String) -> Result<(), String> {
        // In a real implementation, this would involve cryptographic verification
        self.verification_status = CourtVerificationStatus::Verified;
        Ok(())
    }

    /// Access integrity proof (read-only)
    pub fn access_integrity_proof(&mut self, proof_type: &str) -> Result<String, String> {
        if !matches!(self.verification_status, CourtVerificationStatus::Verified) {
            return Err("Court DID not verified".to_string());
        }

        self.integrity_access.last_accessed = Some(Utc::now());
        
        // Return mock integrity proof
        Ok(format!("INTEGRITY_PROOF_{}_{}", proof_type, Utc::now().timestamp()))
    }
}

impl BpiWallet {
    /// Create a new BPI wallet
    pub fn new(
        wallet_id: String,
        address: String,
        organization: WalletOrganization,
        wallet_type: BpiWalletType,
    ) -> Self {
        let now = Utc::now();
        Self {
            wallet_id,
            address,
            organization,
            wallet_type,
            independence: IndependenceProof {
                org_diversity_score: 0.8,
                custody_diversity_score: 0.8,
                geographic_diversity_score: 0.8,
                financial_independence_score: 0.8,
                overall_score: 0.8,
                assessed_at: now,
                assessed_by: "BPCI_INDEPENDENCE_ASSESSOR".to_string(),
            },
            created_at: now,
            last_active: now,
            status: WalletStatus::Active,
        }
    }

    /// Update independence assessment
    pub fn update_independence(&mut self, assessment: IndependenceProof) {
        self.independence = assessment;
    }

    /// Check if wallet is operational
    pub fn is_operational(&self) -> bool {
        matches!(self.status, WalletStatus::Active) && self.independence.overall_score >= 0.6
    }
}

/// StateWallet registry for managing state-level governance wallets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateWalletRegistry {
    /// Registered StateWallets by state ID
    pub state_wallets: HashMap<String, StateWallet>,
    /// Registry creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

impl StateWalletRegistry {
    /// Create a new StateWallet registry
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            state_wallets: HashMap::new(),
            created_at: now,
            last_updated: now,
        }
    }

    /// Register a new StateWallet
    pub fn register_state_wallet(&mut self, state_wallet: StateWallet) -> Result<(), String> {
        if self.state_wallets.contains_key(&state_wallet.state_id) {
            return Err(format!("StateWallet for {} already exists", state_wallet.state_id));
        }

        if !state_wallet.is_valid() {
            return Err(format!("StateWallet for {} is not valid", state_wallet.state_id));
        }

        self.state_wallets.insert(state_wallet.state_id.clone(), state_wallet);
        self.last_updated = Utc::now();
        Ok(())
    }

    /// Get StateWallet by state ID
    pub fn get_state_wallet(&self, state_id: &str) -> Option<&StateWallet> {
        self.state_wallets.get(state_id)
    }

    /// Enforce one-state-one-wallet constraint
    pub fn enforce_one_state_one_wallet(&self) -> Result<(), String> {
        // Check for duplicate geographic scopes
        let mut geo_scopes = Vec::new();
        for (state_id, wallet) in &self.state_wallets {
            for existing_scope in &geo_scopes {
                if wallet.geo_scope.area_overlap(existing_scope) > 0.1 {
                    return Err(format!("Geographic overlap detected for state {}", state_id));
                }
            }
            geo_scopes.push(wallet.geo_scope.clone());
        }
        Ok(())
    }

    /// Get registry statistics
    pub fn get_stats(&self) -> StateWalletStats {
        let total_wallets = self.state_wallets.len();
        let operational_wallets = self.state_wallets.values()
            .filter(|w| w.is_valid())
            .count();
        
        StateWalletStats {
            total_state_wallets: total_wallets,
            operational_wallets,
            suspended_wallets: total_wallets - operational_wallets,
        }
    }
}

/// Statistics for StateWallet registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateWalletStats {
    pub total_state_wallets: usize,
    pub operational_wallets: usize,
    pub suspended_wallets: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::geodid::GeoScope;

    #[test]
    fn test_state_wallet_creation() {
        let geo_scope = GeoScope::state("US".to_string(), "CA".to_string(), "America/Los_Angeles".to_string());
        let court_did = CourtDID::new(
            "did:court:us:ca:supreme".to_string(),
            "California Supreme Court".to_string(),
            JudicialAuthority::Supreme,
            CourtType::Supreme,
            geo_scope.clone(),
        );

        // Create 5 diverse BPI wallets
        let wallets = [
            create_test_wallet("wallet1", OrganizationType::Government { agency_type: "Treasury".to_string() }),
            create_test_wallet("wallet2", OrganizationType::NonProfit { mission: "Governance".to_string() }),
            create_test_wallet("wallet3", OrganizationType::Academic { institution_type: "University".to_string() }),
            create_test_wallet("wallet4", OrganizationType::Professional { service_type: "Legal".to_string() }),
            create_test_wallet("wallet5", OrganizationType::Technology { specialization: "Blockchain".to_string() }),
        ];

        let state_wallet = StateWallet::new(
            "US-CA".to_string(),
            geo_scope,
            court_did,
            wallets,
        );

        assert!(state_wallet.is_ok());
        let wallet = state_wallet.unwrap();
        assert!(wallet.is_valid());
        assert_eq!(wallet.get_active_wallets().len(), 5);
    }

    fn create_test_wallet(id: &str, org_type: OrganizationType) -> BpiWallet {
        // Create diverse jurisdictions and custody models for validation
        let (jurisdiction, custody_model) = match id {
            "wallet1" => ("US".to_string(), CustodyModel::MultiSig { m: 2, n: 3 }),
            "wallet2" => ("CA".to_string(), CustodyModel::SingleSig),
            "wallet3" => ("UK".to_string(), CustodyModel::MultiSig { m: 3, n: 5 }),
            "wallet4" => ("DE".to_string(), CustodyModel::SingleSig),
            _ => ("FR".to_string(), CustodyModel::MultiSig { m: 2, n: 4 }),
        };
        
        let organization = WalletOrganization {
            name: format!("Test Org {}", id),
            org_type,
            registration: LegalRegistration {
                registration_number: format!("REG{}", id),
                jurisdiction,
                registered_at: Utc::now(),
                oversight_body: "Test Regulator".to_string(),
                license_status: LicenseStatus::Active,
            },
            key_custody: KeyCustodyInfo {
                custody_model,
                key_holders: 3,
                threshold: 2,
                hsm_protected: true,
                geographic_distribution: vec!["US".to_string(), "CA".to_string()],
            },
            independence_score: 0.8,
        };

        BpiWallet::new(
            id.to_string(),
            format!("addr_{}", id),
            organization,
            BpiWalletType::Primary { voting_power: 100, proposal_rights: true },
        )
    }
}
