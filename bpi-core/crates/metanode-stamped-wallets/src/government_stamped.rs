use crate::{stamp_types::{WalletStamp, WalletStampType, StampingAuthority, ComplianceMetadata, TransactionLimits, GeographicRestrictions, RevocationStatus, VerificationData}, StampedWalletError, StampedWalletResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc, Datelike};
use rust_decimal::Decimal;

/// Government-stamped wallet authorized by state/country government authorities
/// Enforces regulatory compliance, geographic boundaries, and legal requirements
/// Designed for future decentralized real banking integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernmentStampedWallet {
    /// Wallet address/identifier
    address: String,
    /// Government stamp with regulatory compliance metadata
    stamp: WalletStamp,
    /// Current balance (multi-currency ready)
    balance: Decimal,
    /// Transaction history for regulatory reporting
    transaction_history: Vec<GovernmentTransaction>,
    /// Regulatory compliance status
    compliance_status: RegulatoryComplianceStatus,
    /// Tax reporting information
    tax_reporting: TaxReportingInfo,
    /// Legal compliance metadata
    legal_compliance: LegalComplianceMetadata,
    /// Cross-border transaction capabilities
    cross_border_config: CrossBorderConfig,
    /// Wallet statistics
    statistics: GovernmentWalletStatistics,
    /// Creation timestamp
    created_at: DateTime<Utc>,
    /// Last activity timestamp
    last_activity: DateTime<Utc>,
}

/// Government transaction with regulatory compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernmentTransaction {
    /// Transaction ID
    pub transaction_id: Uuid,
    /// Transaction type
    pub transaction_type: String,
    /// Counterparty address
    pub counterparty: String,
    /// Transaction amount
    pub amount: Decimal,
    /// Currency code (ISO 4217 ready)
    pub currency: String,
    /// Regulatory compliance flags
    pub compliance_flags: Vec<String>,
    /// Tax implications
    pub tax_category: String,
    /// Geographic region
    pub region: String,
    /// Transaction timestamp
    pub timestamp: DateTime<Utc>,
    /// Regulatory reporting data
    pub regulatory_data: HashMap<String, String>,
}

/// Regulatory compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryComplianceStatus {
    /// Overall compliance status
    pub is_compliant: bool,
    /// KYC verification level
    pub kyc_level: String,
    /// AML risk rating
    pub aml_risk_rating: String,
    /// Regulatory licenses
    pub licenses: Vec<String>,
    /// Compliance expiry date
    pub compliance_expires: DateTime<Utc>,
    /// Last compliance check
    pub last_compliance_check: DateTime<Utc>,
    /// Compliance violations (if any)
    pub violations: Vec<String>,
}

/// Tax reporting information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxReportingInfo {
    /// Tax jurisdiction
    pub tax_jurisdiction: String,
    /// Tax ID number
    pub tax_id: Option<String>,
    /// Tax year
    pub tax_year: i32,
    /// Taxable transactions
    pub taxable_transactions: Vec<Uuid>,
    /// Tax liability
    pub estimated_tax_liability: Decimal,
    /// Reporting requirements
    pub reporting_requirements: Vec<String>,
}

/// Legal compliance metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalComplianceMetadata {
    /// Legal entity type
    pub entity_type: String,
    /// Legal jurisdiction
    pub legal_jurisdiction: String,
    /// Regulatory framework
    pub regulatory_framework: Vec<String>,
    /// Legal representative
    pub legal_representative: Option<String>,
    /// Compliance officer
    pub compliance_officer: Option<String>,
    /// Legal agreements
    pub legal_agreements: Vec<String>,
}

/// Cross-border transaction configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossBorderConfig {
    /// Enabled cross-border transactions
    pub enabled: bool,
    /// Allowed destination countries
    pub allowed_countries: Vec<String>,
    /// Restricted countries
    pub restricted_countries: Vec<String>,
    /// Maximum cross-border amount
    pub max_cross_border_amount: Decimal,
    /// Required documentation
    pub required_documentation: Vec<String>,
    /// Settlement networks (SWIFT, etc.)
    pub settlement_networks: Vec<String>,
}

/// Government wallet statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernmentWalletStatistics {
    /// Total transactions
    pub total_transactions: u64,
    /// Total volume
    pub total_volume: Decimal,
    /// Cross-border transactions
    pub cross_border_transactions: u64,
    /// Compliance violations
    pub compliance_violations: u64,
    /// Tax events
    pub tax_events: u64,
    /// Average transaction amount
    pub average_transaction_amount: Decimal,
    /// Last compliance audit
    pub last_compliance_audit: Option<DateTime<Utc>>,
}

impl GovernmentStampedWallet {
    /// Create new government-stamped wallet
    pub fn new(
        address: String,
        government_authority_id: String,
        jurisdiction: String,
        initial_balance: Decimal,
    ) -> StampedWalletResult<Self> {
        let now = Utc::now();
        
        // Create government stamp
        let stamp = WalletStamp {
            stamp_id: Uuid::new_v4(),
            stamp_type: WalletStampType::GovernmentStamped,
            authority_id: Uuid::parse_str(&government_authority_id).unwrap_or_else(|_| Uuid::new_v4()),
            jurisdiction: jurisdiction.clone(),
            issued_at: now,
            expires_at: now + chrono::Duration::days(365), // 1 year validity
            revocation_status: RevocationStatus::NotRevoked,
            compliance_metadata: ComplianceMetadata {
                kyc_status: "government_verified".to_string(),
                aml_status: "compliant".to_string(),
                compliance_flags: vec![
                    "GOVERNMENT_AUTHORIZED".to_string(),
                    "REGULATORY_COMPLIANT".to_string(),
                    "CROSS_BORDER_ENABLED".to_string(),
                ],
                risk_level: "low".to_string(),
                verified_at: now,
                compliance_officer: "government_authority".to_string(),
                kyc_level: "government_verified".to_string(),
                aml_level: "compliant".to_string(),
                transaction_limits: TransactionLimits {
                    max_single_transaction: Decimal::new(100000000, 2), // $1M
                    max_daily_volume: Decimal::new(100000000, 2), // $1M daily limit
                    max_monthly_volume: Decimal::new(1000000000, 2), // $10M monthly limit
                    max_yearly_volume: Decimal::new(10000000000, 2), // $100M annual limit
                    min_transaction: Decimal::new(100, 2), // $1.00
                    allowed_transaction_types: vec!["transfer".to_string(), "cross_border".to_string()],
                    prohibited_transaction_types: vec![],
                    daily_limit: Decimal::new(100000000, 2), // $1M
                },
                geographic_restrictions: GeographicRestrictions {
                    allowed_countries: vec![jurisdiction.clone()],
                    prohibited_countries: vec![], // Government wallets have fewer restrictions
                    allowed_regions: vec!["GLOBAL".to_string()],
                },
            },
            policy_version: "1.0".to_string(),
            chain_of_trust: vec![],
            last_updated: now,
            stamp_hash: vec![0u8; 32],
            verification_data: VerificationData {},
            regulatory_flags: vec!["GOVERNMENT_ISSUED".to_string()],
            geographic_scope: vec![jurisdiction.clone()],
            wallet_address: address.clone(),
            authority_signature: vec![0u8; 64],
            core_maintainer_id: None,
            metadata: HashMap::new(),
        };

        let compliance_status = RegulatoryComplianceStatus {
            is_compliant: true,
            kyc_level: "government_verified".to_string(),
            aml_risk_rating: "low".to_string(),
            licenses: vec!["government_banking_license".to_string()],
            compliance_expires: now + chrono::Duration::days(365),
            last_compliance_check: now,
            violations: vec![],
        };

        let tax_reporting = TaxReportingInfo {
            tax_jurisdiction: jurisdiction.clone(),
            tax_id: None, // Government entities may be tax-exempt
            tax_year: now.year(),
            taxable_transactions: vec![],
            estimated_tax_liability: Decimal::ZERO,
            reporting_requirements: vec![
                "annual_financial_report".to_string(),
                "transaction_summary".to_string(),
            ],
        };

        let legal_compliance = LegalComplianceMetadata {
            entity_type: "government_entity".to_string(),
            legal_jurisdiction: jurisdiction.clone(),
            regulatory_framework: vec![
                "banking_regulations".to_string(),
                "anti_money_laundering".to_string(),
                "data_protection".to_string(),
            ],
            legal_representative: None,
            compliance_officer: None,
            legal_agreements: vec![],
        };

        let cross_border_config = CrossBorderConfig {
            enabled: true,
            allowed_countries: vec![], // Will be populated based on treaties
            restricted_countries: vec![], // Will be populated based on sanctions
            max_cross_border_amount: Decimal::new(10000000000, 2), // $100M
            required_documentation: vec![
                "government_authorization".to_string(),
                "diplomatic_clearance".to_string(),
            ],
            settlement_networks: vec![
                "SWIFT".to_string(),
                "central_bank_network".to_string(),
            ],
        };

        let statistics = GovernmentWalletStatistics {
            total_transactions: 0,
            total_volume: Decimal::ZERO,
            cross_border_transactions: 0,
            compliance_violations: 0,
            tax_events: 0,
            average_transaction_amount: Decimal::ZERO,
            last_compliance_audit: None,
        };

        Ok(GovernmentStampedWallet {
            address,
            stamp,
            balance: initial_balance,
            transaction_history: vec![],
            compliance_status,
            tax_reporting,
            legal_compliance,
            cross_border_config,
            statistics,
            created_at: now,
            last_activity: now,
        })
    }

    /// Execute a government transaction with regulatory compliance
    pub fn execute_transaction(
        &mut self,
        to_address: &str,
        amount: Decimal,
        transaction_type: &str,
    ) -> StampedWalletResult<Uuid> {
        // Validate transaction compliance
        self.validate_transaction_compliance(to_address, amount, transaction_type)?;

        // Check balance
        if self.balance < amount {
            return Err(StampedWalletError::InsufficientFunds(
                format!("Balance: {}, Required: {}", self.balance, amount)
            ));
        }

        // Create transaction record
        let transaction_id = Uuid::new_v4();
        let now = Utc::now();

        let mut regulatory_data = HashMap::new();
        regulatory_data.insert("government_authority".to_string(), self.stamp.authority_id.to_string());
        regulatory_data.insert("jurisdiction".to_string(), self.stamp.jurisdiction.clone());
        regulatory_data.insert("compliance_level".to_string(), "government_verified".to_string());

        let transaction = GovernmentTransaction {
            transaction_id,
            transaction_type: transaction_type.to_string(),
            counterparty: to_address.to_string(),
            amount,
            currency: "USD".to_string(), // Default to USD, multi-currency ready
            compliance_flags: vec![
                "government_authorized".to_string(),
                "regulatory_compliant".to_string(),
            ],
            tax_category: self.determine_tax_category(transaction_type, amount),
            region: self.stamp.jurisdiction.clone(),
            timestamp: now,
            regulatory_data,
        };

        // Execute transaction
        self.balance -= amount;
        self.transaction_history.push(transaction);
        self.last_activity = now;

        // Update statistics
        self.statistics.total_transactions += 1;
        self.statistics.total_volume += amount;
        self.statistics.average_transaction_amount = 
            self.statistics.total_volume / Decimal::from(self.statistics.total_transactions);

        // Check if cross-border transaction
        if self.is_cross_border_transaction(to_address) {
            self.statistics.cross_border_transactions += 1;
        }

        // Add to taxable transactions if applicable
        if self.is_taxable_transaction(transaction_type, amount) {
            self.tax_reporting.taxable_transactions.push(transaction_id);
            self.statistics.tax_events += 1;
        }

        Ok(transaction_id)
    }

    /// Validate transaction compliance for government wallet
    pub fn validate_transaction_compliance(
        &self,
        to_address: &str,
        amount: Decimal,
        transaction_type: &str,
    ) -> StampedWalletResult<()> {
        // Check if wallet is compliant
        if !self.compliance_status.is_compliant {
            return Err(StampedWalletError::ComplianceViolation(
                "Wallet is not in compliance".to_string()
            ));
        }

        // Check if stamp is valid
        if self.stamp.is_revoked() || self.stamp.expires_at < Utc::now() {
            return Err(StampedWalletError::InvalidStamp(
                "Government stamp is revoked or expired".to_string()
            ));
        }

        // Check transaction limits
        let daily_limit = self.stamp.compliance_metadata.transaction_limits.daily_limit;
        if amount > daily_limit {
            return Err(StampedWalletError::TransactionLimitExceeded(
                format!("Amount {} exceeds daily limit {}", amount, daily_limit)
            ));
        }

        // Check cross-border restrictions
        if self.is_cross_border_transaction(to_address) {
            if !self.cross_border_config.enabled {
                return Err(StampedWalletError::ComplianceViolation(
                    "Cross-border transactions not enabled".to_string()
                ));
            }

            if amount > self.cross_border_config.max_cross_border_amount {
                return Err(StampedWalletError::TransactionLimitExceeded(
                    format!("Cross-border amount {} exceeds limit {}", 
                        amount, self.cross_border_config.max_cross_border_amount)
                ));
            }
        }

        // Additional government-specific validations
        match transaction_type {
            "international_aid" | "diplomatic_payment" => {
                // These require special authorization
                if !self.has_diplomatic_authorization() {
                    return Err(StampedWalletError::ComplianceViolation(
                        "Diplomatic authorization required".to_string()
                    ));
                }
            }
            "sanctions_related" => {
                return Err(StampedWalletError::ComplianceViolation(
                    "Sanctions-related transactions prohibited".to_string()
                ));
            }
            _ => {} // Standard transactions allowed
        }

        Ok(())
    }

    /// Get wallet stamp
    pub fn get_stamp(&self) -> &WalletStamp {
        &self.stamp
    }

    /// Get wallet address
    pub fn get_address(&self) -> &str {
        &self.address
    }

    /// Get current balance
    pub fn get_balance(&self) -> Decimal {
        self.balance
    }

    /// Get transaction history
    pub fn get_transaction_history(&self) -> &[GovernmentTransaction] {
        &self.transaction_history
    }

    /// Get compliance status
    pub fn get_compliance_status(&self) -> &RegulatoryComplianceStatus {
        &self.compliance_status
    }

    /// Get wallet statistics
    pub fn get_statistics(&self) -> &GovernmentWalletStatistics {
        &self.statistics
    }

    /// Update compliance status
    pub fn update_compliance_status(&mut self, status: RegulatoryComplianceStatus) {
        self.compliance_status = status;
        self.last_activity = Utc::now();
    }

    /// Add cross-border country authorization
    pub fn authorize_cross_border_country(&mut self, country: String) {
        if !self.cross_border_config.allowed_countries.contains(&country) {
            self.cross_border_config.allowed_countries.push(country);
        }
    }

    /// Revoke cross-border country authorization
    pub fn revoke_cross_border_country(&mut self, country: &str) {
        self.cross_border_config.allowed_countries.retain(|c| c != country);
        if !self.cross_border_config.restricted_countries.contains(&country.to_string()) {
            self.cross_border_config.restricted_countries.push(country.to_string());
        }
    }

    /// Generate regulatory report
    pub fn generate_regulatory_report(&self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> RegulatoryReport {
        let transactions_in_period: Vec<_> = self.transaction_history
            .iter()
            .filter(|tx| tx.timestamp >= start_date && tx.timestamp <= end_date)
            .collect();

        let total_volume: Decimal = transactions_in_period
            .iter()
            .map(|tx| tx.amount)
            .sum();

        let cross_border_volume: Decimal = transactions_in_period
            .iter()
            .filter(|tx| self.is_cross_border_transaction(&tx.counterparty))
            .map(|tx| tx.amount)
            .sum();

        RegulatoryReport {
            report_id: Uuid::new_v4(),
            wallet_address: self.address.clone(),
            jurisdiction: self.stamp.jurisdiction.clone(),
            report_period_start: start_date,
            report_period_end: end_date,
            total_transactions: transactions_in_period.len() as u64,
            total_volume,
            cross_border_transactions: transactions_in_period
                .iter()
                .filter(|tx| self.is_cross_border_transaction(&tx.counterparty))
                .count() as u64,
            cross_border_volume,
            compliance_violations: self.statistics.compliance_violations,
            tax_events: transactions_in_period
                .iter()
                .filter(|tx| self.tax_reporting.taxable_transactions.contains(&tx.transaction_id))
                .count() as u64,
            generated_at: Utc::now(),
        }
    }

    // Private helper methods

    fn determine_tax_category(&self, transaction_type: &str, amount: Decimal) -> String {
        match transaction_type {
            "salary" | "payment" => "income".to_string(),
            "purchase" | "expense" => "deductible".to_string(),
            "investment" => "capital_gains".to_string(),
            "international_aid" => "exempt".to_string(),
            _ => if amount > Decimal::new(1000000, 2) { "reportable".to_string() } else { "standard".to_string() }
        }
    }

    fn is_cross_border_transaction(&self, to_address: &str) -> bool {
        // In a real implementation, this would check the jurisdiction of the destination address
        // For now, we'll use a simple heuristic
        to_address.contains("_foreign_") || to_address.contains("_intl_")
    }

    fn is_taxable_transaction(&self, transaction_type: &str, amount: Decimal) -> bool {
        match transaction_type {
            "international_aid" | "diplomatic_payment" => false, // Government transactions may be tax-exempt
            _ => amount > Decimal::new(1000000, 2) // Large transactions are taxable
        }
    }

    fn has_diplomatic_authorization(&self) -> bool {
        self.legal_compliance.regulatory_framework.contains(&"diplomatic_immunity".to_string()) ||
        self.compliance_status.licenses.contains(&"diplomatic_license".to_string())
    }
}

/// Regulatory report for government compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryReport {
    pub report_id: Uuid,
    pub wallet_address: String,
    pub jurisdiction: String,
    pub report_period_start: DateTime<Utc>,
    pub report_period_end: DateTime<Utc>,
    pub total_transactions: u64,
    pub total_volume: Decimal,
    pub cross_border_transactions: u64,
    pub cross_border_volume: Decimal,
    pub compliance_violations: u64,
    pub tax_events: u64,
    pub generated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_government_wallet_creation() {
        let wallet = GovernmentStampedWallet::new(
            "gov_wallet_001".to_string(),
            "us_treasury".to_string(),
            "US".to_string(),
            Decimal::new(100000000, 2), // $1M
        ).unwrap();

        assert_eq!(wallet.get_address(), "gov_wallet_001");
        assert_eq!(wallet.get_balance(), Decimal::new(100000000, 2));
        assert_eq!(wallet.get_stamp().stamp_type, WalletStampType::GovernmentStamped);
        assert_eq!(wallet.get_stamp().jurisdiction, "US");
        assert!(wallet.get_compliance_status().is_compliant);
    }

    #[test]
    fn test_government_transaction_execution() {
        let mut wallet = GovernmentStampedWallet::new(
            "gov_wallet_001".to_string(),
            "us_treasury".to_string(),
            "US".to_string(),
            Decimal::new(100000000, 2),
        ).unwrap();

        let transaction_id = wallet.execute_transaction(
            "recipient_wallet_001",
            Decimal::new(5000000, 2), // $50K
            "government_payment",
        ).unwrap();

        assert_eq!(wallet.get_balance(), Decimal::new(95000000, 2));
        assert_eq!(wallet.get_transaction_history().len(), 1);
        assert_eq!(wallet.get_statistics().total_transactions, 1);
        assert_eq!(wallet.get_statistics().total_volume, Decimal::new(5000000, 2));

        let transaction = &wallet.get_transaction_history()[0];
        assert_eq!(transaction.transaction_id, transaction_id);
        assert_eq!(transaction.counterparty, "recipient_wallet_001");
        assert_eq!(transaction.amount, Decimal::new(5000000, 2));
    }

    #[test]
    fn test_cross_border_authorization() {
        let mut wallet = GovernmentStampedWallet::new(
            "gov_wallet_001".to_string(),
            "us_treasury".to_string(),
            "US".to_string(),
            Decimal::new(100000000, 2),
        ).unwrap();

        // Initially no countries authorized
        assert_eq!(wallet.cross_border_config.allowed_countries.len(), 0);

        // Authorize Canada
        wallet.authorize_cross_border_country("CA".to_string());
        assert!(wallet.cross_border_config.allowed_countries.contains(&"CA".to_string()));

        // Revoke Canada
        wallet.revoke_cross_border_country("CA");
        assert!(!wallet.cross_border_config.allowed_countries.contains(&"CA".to_string()));
        assert!(wallet.cross_border_config.restricted_countries.contains(&"CA".to_string()));
    }

    #[test]
    fn test_regulatory_report_generation() {
        let mut wallet = GovernmentStampedWallet::new(
            "gov_wallet_001".to_string(),
            "us_treasury".to_string(),
            "US".to_string(),
            Decimal::new(100000000, 2),
        ).unwrap();

        // Execute some transactions
        let _tx1 = wallet.execute_transaction(
            "recipient_001",
            Decimal::new(1000000, 2),
            "government_payment",
        ).unwrap();

        let _tx2 = wallet.execute_transaction(
            "recipient_foreign_002",
            Decimal::new(2000000, 2),
            "international_aid",
        ).unwrap();

        let start_date = Utc::now() - chrono::Duration::hours(1);
        let end_date = Utc::now() + chrono::Duration::hours(1);
        
        let report = wallet.generate_regulatory_report(start_date, end_date);

        assert_eq!(report.wallet_address, "gov_wallet_001");
        assert_eq!(report.jurisdiction, "US");
        assert_eq!(report.total_transactions, 2);
        assert_eq!(report.total_volume, Decimal::new(3000000, 2));
        assert_eq!(report.cross_border_transactions, 1); // The foreign transaction
    }

    #[test]
    fn test_compliance_validation() {
        let wallet = GovernmentStampedWallet::new(
            "gov_wallet_001".to_string(),
            "us_treasury".to_string(),
            "US".to_string(),
            Decimal::new(100000000, 2),
        ).unwrap();

        // Valid transaction should pass
        let result = wallet.validate_transaction_compliance(
            "recipient_001",
            Decimal::new(1000000, 2),
            "government_payment",
        );
        assert!(result.is_ok());

        // Sanctions-related transaction should fail
        let result = wallet.validate_transaction_compliance(
            "recipient_001",
            Decimal::new(1000000, 2),
            "sanctions_related",
        );
        assert!(result.is_err());
    }
}
