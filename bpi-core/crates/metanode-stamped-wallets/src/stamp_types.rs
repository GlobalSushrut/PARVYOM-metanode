//! Stamp types and core data structures for stamped wallets

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use ed25519_dalek::{Signature, VerifyingKey};

use crate::{StampedWalletError, StampedWalletResult};

/// Types of wallet stamps available
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WalletStampType {
    /// No stamp - regular wallet
    None,
    /// Bank-stamped wallet authorized by core infrastructure maintainer/company
    BankStamped,
    /// Government-stamped wallet authorized by state/country government
    GovernmentStamped,
}

/// Authority types that can issue stamps
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthorityType {
    /// Core infrastructure maintainer/company
    CoreMaintainer,
    /// Commercial bank
    Bank,
    /// Central bank
    CentralBank,
    /// Federal government
    FederalGovernment,
    /// State/provincial government
    StateGovernment,
    /// Local government
    LocalGovernment,
    /// Regulatory agency
    RegulatoryAgency,
    /// International organization
    InternationalOrganization,
}

/// Geographic jurisdiction information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jurisdiction {
    /// ISO 3166-1 alpha-2 country code
    pub country_code: String,
    /// State/province code (optional)
    pub state_code: Option<String>,
    /// City/locality (optional)
    pub locality: Option<String>,
    /// Regulatory zone (optional)
    pub regulatory_zone: Option<String>,
}

/// Stamping authority that can issue wallet stamps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StampingAuthority {
    /// Unique authority identifier
    pub authority_id: Uuid,
    /// Authority name
    pub name: String,
    /// Authority type
    pub authority_type: AuthorityType,
    /// Public key for signature verification
    pub public_key: VerifyingKey,
    /// Geographic jurisdiction
    pub jurisdiction: Jurisdiction,
    /// Authority permissions and capabilities
    pub permissions: AuthorityPermissions,
    /// Authority registration timestamp
    pub registered_at: DateTime<Utc>,
    /// Authority expiration (optional)
    pub expires_at: Option<DateTime<Utc>>,
    /// Authority status
    pub is_active: bool,
    /// Contact information
    pub contact_info: AuthorityContact,
}

/// Authority permissions and capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorityPermissions {
    /// Can issue bank stamps
    pub can_issue_bank_stamps: bool,
    /// Can issue government stamps
    pub can_issue_government_stamps: bool,
    /// Maximum transaction amount this authority can authorize
    pub max_transaction_amount: Decimal,
    /// Geographic boundaries this authority can operate in
    pub geographic_boundaries: Vec<Jurisdiction>,
    /// Regulatory frameworks this authority operates under
    pub regulatory_frameworks: Vec<String>,
    /// Can revoke stamps
    pub can_revoke_stamps: bool,
    /// Can delegate authority
    pub can_delegate_authority: bool,
}

/// Authority contact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorityContact {
    /// Official website
    pub website: Option<String>,
    /// Contact email
    pub email: Option<String>,
    /// Phone number
    pub phone: Option<String>,
    /// Physical address
    pub address: Option<String>,
    /// API endpoint for verification
    pub api_endpoint: Option<String>,
}

/// Wallet stamp issued by an authority
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletStamp {
    pub stamp_id: Uuid,
    pub stamp_type: WalletStampType,
    pub authority_id: Uuid,
    pub wallet_address: String,
    pub authority_signature: Vec<u8>,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub compliance_metadata: ComplianceMetadata,
    pub policy_version: String,
    pub chain_of_trust: Vec<Uuid>,
    pub revocation_status: RevocationStatus,
    pub last_updated: DateTime<Utc>,
    pub stamp_hash: Vec<u8>,
    pub verification_data: VerificationData,
    pub regulatory_flags: Vec<String>,
    pub geographic_scope: Vec<String>,
    // Added fields for compatibility
    pub jurisdiction: String,
    pub core_maintainer_id: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Compliance metadata for stamps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceMetadata {
    pub kyc_status: String,
    pub aml_status: String,
    pub compliance_flags: Vec<String>,
    pub risk_level: String,
    pub verified_at: DateTime<Utc>,
    pub compliance_officer: String,
    // Added fields for compatibility
    pub kyc_level: String,
    pub aml_level: String,
    pub transaction_limits: TransactionLimits,
    pub geographic_restrictions: GeographicRestrictions,
}

/// Transaction limits imposed by stamp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionLimits {
    /// Maximum single transaction amount
    pub max_single_transaction: Decimal,
    /// Maximum daily transaction volume
    pub max_daily_volume: Decimal,
    /// Maximum monthly transaction volume
    pub max_monthly_volume: Decimal,
    /// Maximum yearly transaction volume
    pub max_yearly_volume: Decimal,
    /// Minimum transaction amount
    pub min_transaction: Decimal,
    /// Allowed transaction types
    pub allowed_transaction_types: Vec<String>,
    /// Prohibited transaction types
    pub prohibited_transaction_types: Vec<String>,
    /// Daily limit (compatibility field)
    pub daily_limit: Decimal,
}

/// Geographic restrictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicRestrictions {
    pub allowed_countries: Vec<String>,
    pub prohibited_countries: Vec<String>,
    pub allowed_regions: Vec<String>,
}

/// Revocation status of a stamp
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RevocationStatus {
    NotRevoked,
    Revoked { reason: String },
}

/// Verification data for a stamp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationData {
    // Add fields as necessary
}

impl StampingAuthority {
    /// Create a new stamping authority
    pub fn new(
        name: String,
        authority_type: AuthorityType,
        public_key: VerifyingKey,
        jurisdiction: Jurisdiction,
        permissions: AuthorityPermissions,
        contact_info: AuthorityContact,
    ) -> Self {
        Self {
            authority_id: Uuid::new_v4(),
            name,
            authority_type,
            public_key,
            jurisdiction,
            permissions,
            registered_at: Utc::now(),
            expires_at: None,
            is_active: true,
            contact_info,
        }
    }

    /// Check if authority can issue a specific stamp type
    pub fn can_issue_stamp_type(&self, stamp_type: &WalletStampType) -> bool {
        if !self.is_active {
            return false;
        }

        match stamp_type {
            WalletStampType::None => true,
            WalletStampType::BankStamped => self.permissions.can_issue_bank_stamps,
            WalletStampType::GovernmentStamped => self.permissions.can_issue_government_stamps,
        }
    }

    /// Check if authority has jurisdiction in a specific location
    pub fn has_jurisdiction(&self, jurisdiction: &Jurisdiction) -> bool {
        // Check if the authority's jurisdiction covers the requested jurisdiction
        if self.jurisdiction.country_code != jurisdiction.country_code {
            return false;
        }

        // If authority is country-wide, it covers all states/localities
        if self.jurisdiction.state_code.is_none() {
            return true;
        }

        // Check state-level jurisdiction
        if let (Some(auth_state), Some(req_state)) = 
            (&self.jurisdiction.state_code, &jurisdiction.state_code) {
            if auth_state != req_state {
                return false;
            }
        }

        true
    }
}

impl WalletStamp {
    /// Check if stamp is currently valid
    pub fn is_valid(&self) -> bool {
        self.revocation_status == RevocationStatus::NotRevoked && Utc::now() < self.expires_at
    }

    /// Check if stamp is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    /// Check if the stamp is revoked
    pub fn is_revoked(&self) -> bool {
        matches!(self.revocation_status, RevocationStatus::Revoked { .. })
    }

    /// Check if the stamp is revoked (field-style access for compatibility)
    pub fn is_revoked_field(&self) -> bool {
        self.is_revoked()
    }

    /// Get remaining validity period in seconds
    pub fn remaining_validity(&self) -> i64 {
        (self.expires_at - Utc::now()).num_seconds()
    }
}

impl Default for TransactionLimits {
    fn default() -> Self {
        Self {
            max_single_transaction: Decimal::new(100000, 2), // $1K
            max_daily_volume: Decimal::new(1000000, 2), // $10K
            max_monthly_volume: Decimal::new(10000000, 2), // $100K
            max_yearly_volume: Decimal::new(100000000, 2), // $1M
            min_transaction: Decimal::new(100, 2), // $1.00
            allowed_transaction_types: vec!["transfer".to_string()],
            prohibited_transaction_types: vec![],
            daily_limit: Decimal::new(1000000, 2), // $10K
        }
    }
}

impl Default for ComplianceMetadata {
    fn default() -> Self {
        Self {
            kyc_status: "not_verified".to_string(),
            aml_status: "not_screened".to_string(),
            compliance_flags: vec![],
            risk_level: "medium".to_string(),
            verified_at: Utc::now(),
            compliance_officer: "system".to_string(),
            kyc_level: "not_verified".to_string(),
            aml_level: "not_screened".to_string(),
            transaction_limits: TransactionLimits::default(),
            geographic_restrictions: GeographicRestrictions {
                allowed_countries: vec![],
                prohibited_countries: vec![],
                allowed_regions: vec![],
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    #[test]
    fn test_stamping_authority_creation() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let public_key = signing_key.verifying_key();

        let jurisdiction = Jurisdiction {
            country_code: "US".to_string(),
            state_code: Some("CA".to_string()),
            locality: Some("San Francisco".to_string()),
            regulatory_zone: None,
        };

        let permissions = AuthorityPermissions {
            can_issue_bank_stamps: true,
            can_issue_government_stamps: false,
            max_transaction_amount: Decimal::from(1000000),
            geographic_boundaries: vec![jurisdiction.clone()],
            regulatory_frameworks: vec!["FDIC".to_string()],
            can_revoke_stamps: true,
            can_delegate_authority: false,
        };

        let contact_info = AuthorityContact {
            website: Some("https://example-bank.com".to_string()),
            email: Some("compliance@example-bank.com".to_string()),
            phone: Some("+1-555-0123".to_string()),
            address: Some("123 Bank St, San Francisco, CA".to_string()),
            api_endpoint: Some("https://api.example-bank.com/verify".to_string()),
        };

        let authority = StampingAuthority::new(
            "Example Bank".to_string(),
            AuthorityType::Bank,
            public_key,
            jurisdiction,
            permissions,
            contact_info,
        );

        assert_eq!(authority.name, "Example Bank");
        assert_eq!(authority.authority_type, AuthorityType::Bank);
        assert!(authority.is_active);
        assert!(authority.can_issue_stamp_type(&WalletStampType::BankStamped));
        assert!(!authority.can_issue_stamp_type(&WalletStampType::GovernmentStamped));
    }

    #[test]
    fn test_jurisdiction_checking() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let public_key = signing_key.verifying_key();

        let authority_jurisdiction = Jurisdiction {
            country_code: "US".to_string(),
            state_code: Some("CA".to_string()),
            locality: None,
            regulatory_zone: None,
        };

        let permissions = AuthorityPermissions {
            can_issue_bank_stamps: true,
            can_issue_government_stamps: false,
            max_transaction_amount: Decimal::from(1000000),
            geographic_boundaries: vec![authority_jurisdiction.clone()],
            regulatory_frameworks: vec!["FDIC".to_string()],
            can_revoke_stamps: true,
            can_delegate_authority: false,
        };

        let contact_info = AuthorityContact {
            website: None,
            email: None,
            phone: None,
            address: None,
            api_endpoint: None,
        };

        let authority = StampingAuthority::new(
            "California Bank".to_string(),
            AuthorityType::Bank,
            public_key,
            authority_jurisdiction,
            permissions,
            contact_info,
        );

        // Should have jurisdiction in California
        let ca_jurisdiction = Jurisdiction {
            country_code: "US".to_string(),
            state_code: Some("CA".to_string()),
            locality: Some("Los Angeles".to_string()),
            regulatory_zone: None,
        };
        assert!(authority.has_jurisdiction(&ca_jurisdiction));

        // Should not have jurisdiction in New York
        let ny_jurisdiction = Jurisdiction {
            country_code: "US".to_string(),
            state_code: Some("NY".to_string()),
            locality: Some("New York".to_string()),
            regulatory_zone: None,
        };
        assert!(!authority.has_jurisdiction(&ny_jurisdiction));
    }

    #[test]
    fn test_wallet_stamp_validity() {
        let stamp = WalletStamp {
            stamp_id: Uuid::new_v4(),
            stamp_type: WalletStampType::BankStamped,
            authority_id: Uuid::new_v4(),
            wallet_address: "test_wallet_123".to_string(),
            authority_signature: Signature::from_bytes(&[0u8; 64]),
            issued_at: Utc::now() - chrono::Duration::hours(1),
            expires_at: Utc::now() + chrono::Duration::days(365),
            compliance_metadata: ComplianceMetadata::default(),
            transaction_limits: TransactionLimits::default(),
            geographic_restrictions: vec![],
            regulatory_requirements: vec![],
            is_active: true,
            revocation_info: None,
        };

        assert!(stamp.is_valid());
        assert!(!stamp.is_expired());
        assert!(!stamp.is_revoked());
        assert!(stamp.remaining_validity() > 0);
    }
}
