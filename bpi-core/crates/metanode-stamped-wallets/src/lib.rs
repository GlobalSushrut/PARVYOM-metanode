//! # Metanode Stamped Wallets
//! 
//! Bank-stamped and government-stamped wallet implementations for regulated financial operations.
//! This crate extends the existing DockLock wallet infrastructure with compliance, authorization,
//! and regulatory boundary enforcement capabilities.
//! 
//! ## Features
//! 
//! - **Bank-Stamped Wallets**: Authorized by core infrastructure maintainer/company
//! - **Government-Stamped Wallets**: Authorized by state/country government authorities
//! - **Geographic Boundary Enforcement**: IP-based and jurisdiction-aware restrictions
//! - **Regulatory Compliance**: KYC/AML integration and reporting
//! - **Multi-Signature Authorization**: Enhanced security for high-value operations
//! - **Cross-Border Controls**: International transaction monitoring and compliance

use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

pub mod stamp_types;
pub mod bank_stamped_simple;
pub mod government_stamped;
pub mod economics_integration;
// Future modules - not implemented yet
// pub mod stamp_registry;
// pub mod compliance;
// pub mod verification;

pub use stamp_types::*;
pub use bank_stamped_simple::*;
pub use government_stamped::*;
pub use economics_integration::*;
// Future module exports
// pub use stamp_registry::*;
// pub use compliance::*;
// pub use verification::*;

/// Errors that can occur in stamped wallet operations
#[derive(Error, Debug)]
pub enum StampedWalletError {
    #[error("Authorization failed: {0}")]
    AuthorizationFailed(String),
    
    #[error("Invalid stamp: {0}")]
    InvalidStamp(String),
    
    #[error("Geographic restriction violated: {0}")]
    ComplianceViolation(String),
    #[error("Transaction limit exceeded: {0}")]
    TransactionLimitExceeded(String),
    #[error("Geographic restriction: {0}")]
    GeographicRestriction(String),
    #[error("Authority not found: {0}")]
    AuthorityNotFound(String),
    #[error("Verification failed: {0}")]
    VerificationFailed(String),
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
}

/// Result type for stamped wallet operations
pub type StampedWalletResult<T> = Result<T, StampedWalletError>;

/// Configuration for stamped wallet system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StampedWalletConfig {
    /// Enable bank-stamped wallet functionality
    pub bank_stamped_enabled: bool,
    
    /// Enable government-stamped wallet functionality
    pub government_stamped_enabled: bool,
    
    /// Geographic restriction enforcement
    pub geographic_enforcement: bool,
    
    /// Regulatory compliance checking
    pub compliance_checking: bool,
    
    /// Multi-signature requirement threshold
    pub multisig_threshold: u32,
    
    /// Maximum transaction amount without additional authorization
    pub max_transaction_amount: Decimal,
    
    /// Stamp verification timeout in seconds
    pub verification_timeout: u64,
    
    /// Registry update interval in seconds
    pub registry_update_interval: u64,
}

impl Default for StampedWalletConfig {
    fn default() -> Self {
        Self {
            bank_stamped_enabled: true,
            government_stamped_enabled: true,
            geographic_enforcement: true,
            compliance_checking: true,
            multisig_threshold: 2,
            max_transaction_amount: Decimal::from(10000),
            verification_timeout: 30,
            registry_update_interval: 3600,
        }
    }
}

/// Stamped wallet system statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StampedWalletStats {
    /// Total number of bank-stamped wallets
    pub bank_stamped_count: u64,
    
    /// Total number of government-stamped wallets
    pub government_stamped_count: u64,
    
    /// Total number of registered authorities
    pub authority_count: u64,
    
    /// Total number of active stamps
    pub active_stamp_count: u64,
    
    /// Total number of expired stamps
    pub expired_stamp_count: u64,
    
    /// Total number of compliance violations detected
    pub compliance_violations: u64,
    
    /// Total number of geographic restrictions enforced
    pub geographic_restrictions: u64,
    
    /// Total transaction volume processed
    pub total_transaction_volume: Decimal,
    
    /// Last registry update timestamp
    pub last_registry_update: DateTime<Utc>,
}

impl Default for StampedWalletStats {
    fn default() -> Self {
        Self {
            bank_stamped_count: 0,
            government_stamped_count: 0,
            authority_count: 0,
            active_stamp_count: 0,
            expired_stamp_count: 0,
            compliance_violations: 0,
            geographic_restrictions: 0,
            total_transaction_volume: Decimal::ZERO,
            last_registry_update: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stamped_wallet_config_default() {
        let config = StampedWalletConfig::default();
        assert!(config.bank_stamped_enabled);
        assert!(config.government_stamped_enabled);
        assert!(config.geographic_enforcement);
        assert!(config.compliance_checking);
        assert_eq!(config.multisig_threshold, 2);
        assert_eq!(config.max_transaction_amount, Decimal::from(10000));
    }

    #[test]
    fn test_stamped_wallet_stats_default() {
        let stats = StampedWalletStats::default();
        assert_eq!(stats.bank_stamped_count, 0);
        assert_eq!(stats.government_stamped_count, 0);
        assert_eq!(stats.authority_count, 0);
        assert_eq!(stats.total_transaction_volume, Decimal::ZERO);
    }
}
