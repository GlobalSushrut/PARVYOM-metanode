use crate::{
    stamp_types::{WalletStamp, WalletStampType, StampingAuthority, ComplianceMetadata},
    bank_stamped_simple::BankStampedWallet,
    government_stamped::GovernmentStampedWallet,
    StampedWalletError, StampedWalletResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

/// Integration adapter between stamped wallets and the economics engine
#[derive(Debug, Clone)]
pub struct StampedWalletEconomicsAdapter {
    /// Bank-stamped wallets registry
    bank_wallets: HashMap<String, BankStampedWallet>,
    /// Government-stamped wallets registry
    government_wallets: HashMap<String, GovernmentStampedWallet>,
    /// Stamping authorities registry
    authorities: HashMap<String, StampingAuthority>,
    /// Transaction compliance cache
    compliance_cache: HashMap<String, ComplianceValidation>,
    /// Integration configuration
    config: EconomicsIntegrationConfig,
}

/// Configuration for economics integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicsIntegrationConfig {
    /// Enable compliance checking for all transactions
    pub enforce_compliance: bool,
    /// Maximum transaction amount without additional verification
    pub max_unverified_amount: Decimal,
    /// Enable geographic boundary enforcement
    pub enforce_geographic_boundaries: bool,
    /// Enable KYC/AML compliance checking
    pub enforce_kyc_aml: bool,
    /// Cache compliance validations for performance
    pub cache_compliance_validations: bool,
    /// Compliance cache TTL in seconds
    pub compliance_cache_ttl: u64,
}

impl Default for EconomicsIntegrationConfig {
    fn default() -> Self {
        Self {
            enforce_compliance: true,
            max_unverified_amount: Decimal::new(10000, 2), // $100.00
            enforce_geographic_boundaries: true,
            enforce_kyc_aml: true,
            cache_compliance_validations: true,
            compliance_cache_ttl: 300, // 5 minutes
        }
    }
}

/// Compliance validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceValidation {
    /// Validation ID
    pub validation_id: Uuid,
    /// Wallet address being validated
    pub wallet_address: String,
    /// Transaction amount
    pub amount: Decimal,
    /// Validation result
    pub is_compliant: bool,
    /// Compliance flags that passed
    pub passed_checks: Vec<String>,
    /// Compliance flags that failed
    pub failed_checks: Vec<String>,
    /// Validation timestamp
    pub validated_at: DateTime<Utc>,
    /// Validation expires at
    pub expires_at: DateTime<Utc>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Economic transaction with stamped wallet compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StampedTransaction {
    /// Transaction ID
    pub transaction_id: Uuid,
    /// Source wallet address
    pub from_address: String,
    /// Destination wallet address
    pub to_address: String,
    /// Transaction amount
    pub amount: Decimal,
    /// Transaction type
    pub transaction_type: StampedTransactionType,
    /// Compliance validation
    pub compliance: ComplianceValidation,
    /// Stamped wallet metadata
    pub stamp_metadata: HashMap<String, String>,
    /// Transaction timestamp
    pub timestamp: DateTime<Utc>,
}

/// Types of stamped wallet transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StampedTransactionType {
    /// Standard transfer between wallets
    Transfer,
    /// Staking operation
    Stake,
    /// Unstaking operation
    Unstake,
    /// Reward distribution
    Reward,
    /// Governance voting
    GovernanceVote,
    /// Compliance fee payment
    ComplianceFee,
    /// Cross-border transaction
    CrossBorder,
    /// Multi-signature transaction
    MultiSignature,
}

impl StampedWalletEconomicsAdapter {
    /// Create new economics integration adapter
    pub fn new(config: EconomicsIntegrationConfig) -> Self {
        Self {
            bank_wallets: HashMap::new(),
            government_wallets: HashMap::new(),
            authorities: HashMap::new(),
            compliance_cache: HashMap::new(),
            config,
        }
    }

    /// Register a bank-stamped wallet
    pub fn register_bank_wallet(
        &mut self,
        wallet: BankStampedWallet,
    ) -> StampedWalletResult<()> {
        let address = wallet.get_address().to_string();
        self.bank_wallets.insert(address, wallet);
        Ok(())
    }

    /// Register a government-stamped wallet
    pub fn register_government_wallet(
        &mut self,
        wallet: GovernmentStampedWallet,
    ) -> StampedWalletResult<()> {
        let address = wallet.get_address().to_string();
        self.government_wallets.insert(address, wallet);
        Ok(())
    }

    /// Register a stamping authority
    pub fn register_authority(
        &mut self,
        authority: StampingAuthority,
    ) -> StampedWalletResult<()> {
        let authority_id = authority.authority_id.clone();
        self.authorities.insert(authority_id.to_string(), authority);
        Ok(())
    }

    /// Validate transaction compliance for stamped wallets
    pub fn validate_transaction_compliance(
        &mut self,
        from_address: &str,
        to_address: &str,
        amount: Decimal,
        transaction_type: StampedTransactionType,
    ) -> StampedWalletResult<ComplianceValidation> {
        let validation_id = Uuid::new_v4();
        let now = Utc::now();
        let expires_at = now + chrono::Duration::seconds(self.config.compliance_cache_ttl as i64);

        // Check if validation is cached
        let cache_key = format!("{}:{}:{}", from_address, to_address, amount);
        if self.config.cache_compliance_validations {
            if let Some(cached) = self.compliance_cache.get(&cache_key) {
                if cached.expires_at > now {
                    return Ok(cached.clone());
                }
            }
        }

        let mut passed_checks = Vec::new();
        let mut failed_checks = Vec::new();
        let mut metadata = HashMap::new();

        // Basic compliance checks
        if self.config.enforce_compliance {
            // Check if wallets exist and are properly stamped
            let from_stamp_valid = self.validate_wallet_stamp(from_address)?;
            let to_stamp_valid = self.validate_wallet_stamp(to_address)?;

            if from_stamp_valid {
                passed_checks.push("from_wallet_stamp_valid".to_string());
            } else {
                failed_checks.push("from_wallet_stamp_invalid".to_string());
            }

            if to_stamp_valid {
                passed_checks.push("to_wallet_stamp_valid".to_string());
            } else {
                failed_checks.push("to_wallet_stamp_invalid".to_string());
            }

            // Amount validation
            if amount > self.config.max_unverified_amount {
                if self.validate_high_value_transaction(from_address, amount)? {
                    passed_checks.push("high_value_transaction_approved".to_string());
                } else {
                    failed_checks.push("high_value_transaction_rejected".to_string());
                }
            } else {
                passed_checks.push("amount_within_limits".to_string());
            }

            // Geographic boundary enforcement
            if self.config.enforce_geographic_boundaries {
                if self.validate_geographic_compliance(from_address, to_address)? {
                    passed_checks.push("geographic_boundaries_compliant".to_string());
                } else {
                    failed_checks.push("geographic_boundaries_violation".to_string());
                }
            }

            // KYC/AML compliance
            if self.config.enforce_kyc_aml {
                if self.validate_kyc_aml_compliance(from_address, to_address, amount)? {
                    passed_checks.push("kyc_aml_compliant".to_string());
                } else {
                    failed_checks.push("kyc_aml_violation".to_string());
                }
            }

            // Transaction type specific validation
            match transaction_type {
                StampedTransactionType::CrossBorder => {
                    if self.validate_cross_border_transaction(from_address, to_address, amount)? {
                        passed_checks.push("cross_border_compliant".to_string());
                    } else {
                        failed_checks.push("cross_border_violation".to_string());
                    }
                }
                StampedTransactionType::MultiSignature => {
                    if self.validate_multi_signature_transaction(from_address, amount)? {
                        passed_checks.push("multi_signature_compliant".to_string());
                    } else {
                        failed_checks.push("multi_signature_violation".to_string());
                    }
                }
                _ => {
                    passed_checks.push("transaction_type_valid".to_string());
                }
            }
        }

        let is_compliant = failed_checks.is_empty();
        metadata.insert("validation_version".to_string(), "1.0".to_string());
        metadata.insert("enforcement_level".to_string(), 
            if self.config.enforce_compliance { "strict" } else { "permissive" }.to_string());

        let validation = ComplianceValidation {
            validation_id,
            wallet_address: from_address.to_string(),
            amount,
            is_compliant,
            passed_checks,
            failed_checks,
            validated_at: now,
            expires_at,
            metadata,
        };

        // Cache the validation
        if self.config.cache_compliance_validations {
            self.compliance_cache.insert(cache_key, validation.clone());
        }

        Ok(validation)
    }

    /// Execute a stamped wallet transaction with compliance checking
    pub fn execute_stamped_transaction(
        &mut self,
        from_address: &str,
        to_address: &str,
        amount: Decimal,
        transaction_type: StampedTransactionType,
    ) -> StampedWalletResult<StampedTransaction> {
        // Validate compliance first
        let compliance = self.validate_transaction_compliance(
            from_address, to_address, amount, transaction_type.clone()
        )?;

        if !compliance.is_compliant {
            return Err(StampedWalletError::ComplianceViolation(
                format!("Transaction failed compliance: {:?}", compliance.failed_checks)
            ));
        }

        // Execute the transaction based on wallet types
        let mut stamp_metadata = HashMap::new();

        // Handle bank-stamped wallet transactions
        if let Some(bank_wallet) = self.bank_wallets.get_mut(from_address) {
            bank_wallet.execute_transaction(to_address, amount, "economics_transfer")?;
            stamp_metadata.insert("wallet_type".to_string(), "bank_stamped".to_string());
            stamp_metadata.insert("core_maintainer".to_string(), 
                bank_wallet.get_stamp().core_maintainer_id.clone().unwrap_or_default());
        }

        // Handle government-stamped wallet transactions
        if let Some(gov_wallet) = self.government_wallets.get_mut(from_address) {
            gov_wallet.execute_transaction(to_address, amount, "economics_transfer")?;
            stamp_metadata.insert("wallet_type".to_string(), "government_stamped".to_string());
            stamp_metadata.insert("jurisdiction".to_string(), 
                gov_wallet.get_stamp().jurisdiction.clone());
        }

        let transaction = StampedTransaction {
            transaction_id: Uuid::new_v4(),
            from_address: from_address.to_string(),
            to_address: to_address.to_string(),
            amount,
            transaction_type,
            compliance,
            stamp_metadata,
            timestamp: Utc::now(),
        };

        Ok(transaction)
    }

    /// Get wallet stamp information
    pub fn get_wallet_stamp(&self, address: &str) -> Option<WalletStamp> {
        if let Some(bank_wallet) = self.bank_wallets.get(address) {
            Some(bank_wallet.get_stamp().clone())
        } else if let Some(gov_wallet) = self.government_wallets.get(address) {
            Some(gov_wallet.get_stamp().clone())
        } else {
            None
        }
    }

    /// Check if wallet is stamped and valid
    pub fn is_wallet_stamped(&self, address: &str) -> bool {
        self.bank_wallets.contains_key(address) || self.government_wallets.contains_key(address)
    }

    /// Get wallet type
    pub fn get_wallet_type(&self, address: &str) -> Option<WalletStampType> {
        if self.bank_wallets.contains_key(address) {
            Some(WalletStampType::BankStamped)
        } else if self.government_wallets.contains_key(address) {
            Some(WalletStampType::GovernmentStamped)
        } else {
            Some(WalletStampType::None)
        }
    }

    /// Clean expired compliance validations
    pub fn cleanup_expired_validations(&mut self) {
        let now = Utc::now();
        self.compliance_cache.retain(|_, validation| validation.expires_at > now);
    }

    /// Get compliance statistics
    pub fn get_compliance_statistics(&self) -> ComplianceStatistics {
        let total_validations = self.compliance_cache.len();
        let compliant_validations = self.compliance_cache.values()
            .filter(|v| v.is_compliant)
            .count();
        
        ComplianceStatistics {
            total_validations,
            compliant_validations,
            non_compliant_validations: total_validations - compliant_validations,
            compliance_rate: if total_validations > 0 {
                (compliant_validations as f64 / total_validations as f64) * 100.0
            } else {
                0.0
            },
            cache_size: self.compliance_cache.len(),
            registered_bank_wallets: self.bank_wallets.len(),
            registered_government_wallets: self.government_wallets.len(),
            registered_authorities: self.authorities.len(),
        }
    }

    // Private helper methods for compliance validation

    fn validate_wallet_stamp(&self, address: &str) -> StampedWalletResult<bool> {
        if let Some(stamp) = self.get_wallet_stamp(address) {
            Ok(!stamp.is_revoked() && stamp.expires_at > Utc::now())
        } else {
            Ok(false)
        }
    }

    fn validate_high_value_transaction(&self, address: &str, amount: Decimal) -> StampedWalletResult<bool> {
        // Check if wallet has high-value transaction permissions
        if let Some(stamp) = self.get_wallet_stamp(address) {
            Ok(stamp.compliance_metadata.transaction_limits.daily_limit >= amount)
        } else {
            Ok(false)
        }
    }

    fn validate_geographic_compliance(&self, from_address: &str, to_address: &str) -> StampedWalletResult<bool> {
        let from_stamp = self.get_wallet_stamp(from_address);
        let to_stamp = self.get_wallet_stamp(to_address);

        match (from_stamp, to_stamp) {
            (Some(from), Some(to)) => {
                // Check if jurisdictions allow cross-border transactions
                Ok(from.jurisdiction == to.jurisdiction || 
                   from.compliance_metadata.geographic_restrictions.allowed_countries
                       .contains(&to.jurisdiction))
            }
            _ => Ok(true) // Allow if stamps are not available
        }
    }

    fn validate_kyc_aml_compliance(&self, from_address: &str, to_address: &str, amount: Decimal) -> StampedWalletResult<bool> {
        let from_stamp = self.get_wallet_stamp(from_address);
        
        if let Some(stamp) = from_stamp {
            let kyc_level = stamp.compliance_metadata.kyc_level.clone();
            let aml_level = stamp.compliance_metadata.aml_level.clone();
            
            // Basic KYC/AML validation based on amount and compliance levels
            Ok(match (kyc_level.as_str(), aml_level.as_str()) {
                ("verified", "compliant") => true,
                ("basic", "basic") => amount <= Decimal::new(1000, 2), // $10.00
                ("verified", "basic") => amount <= Decimal::new(5000, 2), // $50.00
                _ => false,
            })
        } else {
            Ok(false)
        }
    }

    fn validate_cross_border_transaction(&self, from_address: &str, to_address: &str, amount: Decimal) -> StampedWalletResult<bool> {
        // Enhanced validation for cross-border transactions
        let geographic_ok = self.validate_geographic_compliance(from_address, to_address)?;
        let kyc_ok = self.validate_kyc_aml_compliance(from_address, to_address, amount)?;
        
        Ok(geographic_ok && kyc_ok)
    }

    fn validate_multi_signature_transaction(&self, address: &str, amount: Decimal) -> StampedWalletResult<bool> {
        if let Some(bank_wallet) = self.bank_wallets.get(address) {
            Ok(bank_wallet.get_multi_sig_threshold() > 0 && 
               amount <= bank_wallet.get_stamp().compliance_metadata.transaction_limits.daily_limit)
        } else {
            Ok(false)
        }
    }
}

/// Compliance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatistics {
    pub total_validations: usize,
    pub compliant_validations: usize,
    pub non_compliant_validations: usize,
    pub compliance_rate: f64,
    pub cache_size: usize,
    pub registered_bank_wallets: usize,
    pub registered_government_wallets: usize,
    pub registered_authorities: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stamp_types::*;

    #[test]
    fn test_economics_adapter_creation() {
        let config = EconomicsIntegrationConfig::default();
        let adapter = StampedWalletEconomicsAdapter::new(config);
        
        assert_eq!(adapter.bank_wallets.len(), 0);
        assert_eq!(adapter.government_wallets.len(), 0);
        assert_eq!(adapter.authorities.len(), 0);
    }

    #[test]
    fn test_wallet_registration() {
        let mut adapter = StampedWalletEconomicsAdapter::new(EconomicsIntegrationConfig::default());
        
        // Create test bank wallet using simplified API
        let bank_wallet = BankStampedWallet::new(
            "bank_wallet_001".to_string(),
            "test_bank".to_string(),
            Decimal::new(100000, 2), // $1000.00
        ).unwrap();

        // Register wallet
        adapter.register_bank_wallet(bank_wallet).unwrap();

        assert_eq!(adapter.bank_wallets.len(), 1);
        assert!(adapter.is_wallet_stamped("bank_wallet_001"));
        assert_eq!(adapter.get_wallet_type("bank_wallet_001"), Some(WalletStampType::BankStamped));
    }

    #[test]
    fn test_compliance_validation() {
        let mut adapter = StampedWalletEconomicsAdapter::new(EconomicsIntegrationConfig::default());
        
        // Create and register test wallet using simplified API
        let bank_wallet = BankStampedWallet::new(
            "bank_wallet_001".to_string(),
            "test_bank".to_string(),
            Decimal::new(100000, 2),
        ).unwrap();
        
        adapter.register_bank_wallet(bank_wallet).unwrap();

        // Test compliance validation
        let validation = adapter.validate_transaction_compliance(
            "bank_wallet_001",
            "regular_wallet_001",
            Decimal::new(5000, 2), // $50.00
            StampedTransactionType::Transfer,
        ).unwrap();

        assert!(validation.is_compliant);
        assert!(!validation.passed_checks.is_empty());
    }

    #[test]
    fn test_stamped_transaction_execution() {
        let mut adapter = StampedWalletEconomicsAdapter::new(EconomicsIntegrationConfig::default());
        
        // Create and register test wallet using simplified API
        let bank_wallet = BankStampedWallet::new(
            "bank_wallet_001".to_string(),
            "test_bank".to_string(),
            Decimal::new(100000, 2),
        ).unwrap();
        
        adapter.register_bank_wallet(bank_wallet).unwrap();

        // Execute stamped transaction
        let transaction = adapter.execute_stamped_transaction(
            "bank_wallet_001",
            "regular_wallet_001",
            Decimal::new(2500, 2), // $25.00
            StampedTransactionType::Transfer,
        ).unwrap();

        assert_eq!(transaction.from_address, "bank_wallet_001");
        assert_eq!(transaction.to_address, "regular_wallet_001");
        assert_eq!(transaction.amount, Decimal::new(2500, 2));
        assert!(transaction.compliance.is_compliant);
        assert_eq!(transaction.stamp_metadata.get("wallet_type"), Some(&"bank_stamped".to_string()));
    }

    #[test]
    fn test_compliance_statistics() {
        let mut adapter = StampedWalletEconomicsAdapter::new(EconomicsIntegrationConfig::default());
        
        // Register test wallet using simplified API
        let bank_wallet = BankStampedWallet::new(
            "bank_wallet_001".to_string(),
            "test_bank".to_string(),
            Decimal::new(100000, 2),
        ).unwrap();
        
        adapter.register_bank_wallet(bank_wallet).unwrap();

        // Perform some validations
        let _validation1 = adapter.validate_transaction_compliance(
            "bank_wallet_001", "wallet_002", Decimal::new(1000, 2), StampedTransactionType::Transfer
        ).unwrap();
        
        let _validation2 = adapter.validate_transaction_compliance(
            "bank_wallet_001", "wallet_003", Decimal::new(2000, 2), StampedTransactionType::Transfer
        ).unwrap();

        let stats = adapter.get_compliance_statistics();
        assert_eq!(stats.registered_bank_wallets, 1);
        assert_eq!(stats.registered_government_wallets, 0);
        assert!(stats.total_validations > 0);
    }
}
