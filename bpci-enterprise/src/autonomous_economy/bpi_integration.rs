use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use thiserror::Error;
use uuid::Uuid;

use crate::autonomous_economy::{CoinDistributionEngine, CoinType, WorkProofValidator, DistributionResult, BpciTreasuryIntegration, FundSource, TreasuryConfig};
use crate::registry::node_types::{
    BpiWalletStamp, TransactionLimits, BankType, BankLicense, BankLicenseType, RiskManagement,
};

/// BPI Integration with Autonomous Economy
/// Implements BPI wallet rent + gas fee model feeding into 4-coin distribution system

/// BPI wallet session tracking for rent calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpiWalletSession {
    /// Wallet identifier
    pub wallet_id: String,
    /// Session start time
    pub session_start: DateTime<Utc>,
    /// Last activity time
    pub last_activity: DateTime<Utc>,
    /// Session status
    pub status: SessionStatus,
    /// Wallet stamp type (determines coin access)
    pub wallet_stamp: BpiWalletStamp,
    /// Accumulated rent owed
    pub rent_owed: Decimal,
    /// Gas fees accumulated
    pub gas_fees_accumulated: Decimal,
    /// Total fiat value generated
    pub total_fiat_generated: Decimal,
}

/// BPI wallet session status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionStatus {
    Active,
    Idle,
    Disconnected,
    PaymentPending,
    Suspended,
}

/// BPI transaction for gas fee calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpiTransaction {
    /// Transaction ID
    pub transaction_id: String,
    /// Wallet that initiated transaction
    pub wallet_id: String,
    /// Transaction type
    pub transaction_type: BpiTransactionType,
    /// Gas fee amount in USD
    pub gas_fee: Decimal,
    /// Transaction timestamp
    pub timestamp: DateTime<Utc>,
    /// Target coin for fee distribution
    pub target_coin: CoinType,
}

/// Types of BPI transactions that incur gas fees
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BpiTransactionType {
    /// Deploy container to BPCI server
    ContainerDeploy { container_id: String },
    /// PoE bundle submission
    PoEBundle { bundle_id: String },
    /// Notarization request
    Notarization { document_hash: String },
    /// Validation request
    Validation { validation_id: String },
    /// Cross-chain bridge operation
    CrossChainBridge { source_chain: String, target_chain: String },
    /// Governance proposal submission
    GovernanceProposal { proposal_id: String },
    /// Community voting
    CommunityVoting { vote_id: String },
}

/// BPI rent and gas fee configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpiEconomicConfig {
    /// Rent rate per hour in USD
    pub rent_rate_per_hour: Decimal,
    /// Gas fee rates by transaction type
    pub gas_fee_rates: HashMap<String, Decimal>,
    /// Minimum session duration for rent (minutes)
    pub minimum_session_minutes: u32,
    /// Payment collection interval (minutes)
    pub payment_interval_minutes: u32,
    /// Grace period before suspension (hours)
    pub grace_period_hours: u32,
    /// Default target coin for rent payments
    pub default_rent_coin: CoinType,
    /// Default target coin for gas payments
    pub default_gas_coin: CoinType,
}

impl Default for BpiEconomicConfig {
    fn default() -> Self {
        let mut gas_fee_rates = HashMap::new();
        gas_fee_rates.insert("ContainerDeploy".to_string(), Decimal::new(5, 0)); // $5
        gas_fee_rates.insert("PoEBundle".to_string(), Decimal::new(2, 0));       // $2
        gas_fee_rates.insert("Notarization".to_string(), Decimal::new(1, 0));    // $1
        gas_fee_rates.insert("Validation".to_string(), Decimal::new(1, 0));      // $1
        gas_fee_rates.insert("CrossChainBridge".to_string(), Decimal::new(10, 0)); // $10
        gas_fee_rates.insert("GovernanceProposal".to_string(), Decimal::new(25, 0)); // $25
        gas_fee_rates.insert("CommunityVoting".to_string(), Decimal::new(5, 1)); // $0.5
        
        Self {
            rent_rate_per_hour: Decimal::new(2, 0), // $2/hour default
            gas_fee_rates,
            minimum_session_minutes: 15, // 15 minute minimum
            payment_interval_minutes: 60, // Collect every hour
            grace_period_hours: 24, // 24 hour grace period
            default_rent_coin: CoinType::Flx, // Rent goes to FLX (network usage)
            default_gas_coin: CoinType::Flx,  // Gas goes to FLX (network usage)
        }
    }
}

/// BPI integration errors
#[derive(Error, Debug)]
pub enum BpiIntegrationError {
    #[error("Wallet session not found: {0}")]
    SessionNotFound(String),
    #[error("Invalid wallet stamp for coin access: {stamp:?} cannot access {coin:?}")]
    InvalidWalletStamp { stamp: BpiWalletStamp, coin: CoinType },
    #[error("Payment processing failed: {0}")]
    PaymentFailed(String),
    #[error("Insufficient session duration: {0} minutes")]
    InsufficientSessionDuration(u32),
    #[error("Work proof creation failed: {0}")]
    WorkProofCreationFailed(String),
}

/// BPI autonomous economy integration engine
#[derive(Debug)]
pub struct BpiEconomicIntegration {
    /// Configuration
    config: BpiEconomicConfig,
    /// Active wallet sessions
    active_sessions: Arc<RwLock<HashMap<String, BpiWalletSession>>>,
    /// Treasury integration for strict 25%/75% split
    treasury_integration: Arc<BpciTreasuryIntegration>,
    /// Work proof validator
    work_validator: Arc<RwLock<WorkProofValidator>>,
    /// Transaction history
    transaction_history: Arc<RwLock<Vec<BpiTransaction>>>,
}

impl BpiEconomicIntegration {
    /// Create new BPI economic integration with treasury system
    pub fn new(config: BpiEconomicConfig, treasury_integration: Arc<BpciTreasuryIntegration>) -> Self {
        Self {
            config,
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            treasury_integration,
            work_validator: Arc::new(RwLock::new(WorkProofValidator::new())),
            transaction_history: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Start BPI wallet session and begin rent accumulation
    pub async fn start_wallet_session(
        &self,
        wallet_id: String,
        wallet_stamp: BpiWalletStamp,
    ) -> Result<(), BpiIntegrationError> {
        let now = Utc::now();
        
        let session = BpiWalletSession {
            wallet_id: wallet_id.clone(),
            session_start: now,
            last_activity: now,
            status: SessionStatus::Active,
            wallet_stamp,
            rent_owed: Decimal::ZERO,
            gas_fees_accumulated: Decimal::ZERO,
            total_fiat_generated: Decimal::ZERO,
        };
        
        let mut sessions = self.active_sessions.write().await;
        sessions.insert(wallet_id, session);
        
        Ok(())
    }
    
    /// Process BPI transaction and collect gas fees
    pub async fn process_bpi_transaction(
        &self,
        wallet_id: String,
        transaction_type: BpiTransactionType,
        custom_gas_fee: Option<Decimal>,
    ) -> Result<BpiTransaction, BpiIntegrationError> {
        // Get or create session
        let mut sessions = self.active_sessions.write().await;
        let session = sessions.get_mut(&wallet_id)
            .ok_or_else(|| BpiIntegrationError::SessionNotFound(wallet_id.clone()))?;
        
        // Calculate gas fee
        let gas_fee = custom_gas_fee.unwrap_or_else(|| {
            let tx_type_str = format!("{:?}", transaction_type).split(' ').next().unwrap_or("Unknown").to_string();
            self.config.gas_fee_rates.get(&tx_type_str)
                .copied()
                .unwrap_or(Decimal::new(1, 0)) // Default $1
        });
        
        // Determine target coin based on wallet stamp
        let target_coin = self.determine_target_coin(&session.wallet_stamp, true)?;
        
        // Create transaction
        let transaction = BpiTransaction {
            transaction_id: Uuid::new_v4().to_string(),
            wallet_id: wallet_id.clone(),
            transaction_type,
            gas_fee,
            timestamp: Utc::now(),
            target_coin,
        };
        
        // Update session
        session.last_activity = Utc::now();
        session.gas_fees_accumulated += gas_fee;
        session.total_fiat_generated += gas_fee;
        
        // Store transaction
        let mut history = self.transaction_history.write().await;
        history.push(transaction.clone());
        
        // Process payment immediately for gas fees
        self.process_gas_payment(&transaction).await?;
        
        Ok(transaction)
    }
    
    /// Calculate and collect rent for active sessions
    pub async fn collect_rent_payments(&self) -> Result<Vec<DistributionResult>, BpiIntegrationError> {
        let mut results = Vec::new();
        let mut sessions = self.active_sessions.write().await;
        
        for (wallet_id, session) in sessions.iter_mut() {
            if session.status != SessionStatus::Active {
                continue;
            }
            
            // Calculate rent owed since last collection
            let now = Utc::now();
            let hours_since_start = (now - session.session_start).num_minutes() as f64 / 60.0;
            let total_rent_owed = Decimal::from_f64(hours_since_start).unwrap_or(Decimal::ZERO) * self.config.rent_rate_per_hour;
            let new_rent = total_rent_owed - session.rent_owed;
            
            if new_rent > Decimal::ZERO {
                // Determine target coin
                let target_coin = self.determine_target_coin(&session.wallet_stamp, false)?;
                
                // Create work proof for rent
                let work_proof = {
                    let validator = self.work_validator.read().await;
                    validator.create_bpi_rent_proof(
                        wallet_id.clone(),
                        hours_since_start.ceil() as u32,
                        self.config.rent_rate_per_hour,
                        target_coin,
                    )
                };
                
                // Validate work proof
                let validated_proof = {
                    let mut validator = self.work_validator.write().await;
                    validator.validate_work_proof(work_proof)
                        .map_err(|e| BpiIntegrationError::WorkProofCreationFailed(e.to_string()))?
                };
                
                // Process payment through treasury system (strict 25%/75% split)
                let treasury_transaction = self.treasury_integration
                    .process_fiat_inflow(new_rent, FundSource::WalletRentPayments, &wallet_id)
                    .await
                    .map_err(|e| BpiIntegrationError::PaymentFailed(e.to_string()))?;
                
                // Create distribution result for compatibility
                let distribution_result = DistributionResult {
                    fiat_inflow: new_rent,
                    coin_type: target_coin,
                    coin_allocation: crate::autonomous_economy::CoinAllocation {
                        fixed_amount: treasury_transaction.coin_economy_amount,
                        claimable_amount: Decimal::ZERO,
                        transfer_to_mother: None,
                        coin_type: target_coin,
                    },
                    treasury_allocation: crate::autonomous_economy::TreasuryAllocation {
                        company_treasury: treasury_transaction.infrastructure_breakdown.company_api,
                        owner_salary: treasury_transaction.infrastructure_breakdown.owner_salary,
                        community_maintainers: treasury_transaction.infrastructure_breakdown.community_reserves,
                        infrastructure_treasury: Decimal::ZERO,
                    },
                    timestamp: treasury_transaction.timestamp,
                };
                
                // Update session
                session.rent_owed = total_rent_owed;
                session.total_fiat_generated += new_rent;
                
                results.push(distribution_result);
            }
        }
        
        Ok(results)
    }
    
    /// Process gas payment through coin distribution
    async fn process_gas_payment(&self, transaction: &BpiTransaction) -> Result<DistributionResult, BpiIntegrationError> {
        // Create work proof for gas
        let work_proof = {
            let validator = self.work_validator.read().await;
            validator.create_bpi_gas_proof(
                transaction.transaction_id.clone(),
                format!("{:?}", transaction.transaction_type),
                transaction.gas_fee,
                transaction.wallet_id.clone(),
                transaction.target_coin,
            )
        };
        
        // Validate work proof
        let validated_proof = {
            let mut validator = self.work_validator.write().await;
            validator.validate_work_proof(work_proof)
                .map_err(|e| BpiIntegrationError::WorkProofCreationFailed(e.to_string()))?
        };
        
        // Process payment through treasury system (strict 25%/75% split)
        let treasury_transaction = self.treasury_integration
            .process_fiat_inflow(transaction.gas_fee, FundSource::WalletGasFees, &transaction.wallet_id)
            .await
            .map_err(|e| BpiIntegrationError::PaymentFailed(e.to_string()))?;
        
        // Create distribution result for compatibility
        let distribution_result = DistributionResult {
            fiat_inflow: transaction.gas_fee,
            coin_type: transaction.target_coin,
            coin_allocation: crate::autonomous_economy::CoinAllocation {
                fixed_amount: transaction.gas_fee,
                claimable_amount: Decimal::ZERO,
                transfer_to_mother: None,
                coin_type: transaction.target_coin,
            },
            treasury_allocation: crate::autonomous_economy::TreasuryAllocation {
                company_treasury: treasury_transaction.infrastructure_breakdown.company_api,
                owner_salary: treasury_transaction.infrastructure_breakdown.owner_salary,
                community_maintainers: treasury_transaction.infrastructure_breakdown.community_reserves,
                infrastructure_treasury: Decimal::ZERO,
            },
            timestamp: treasury_transaction.timestamp,
        };
        
        Ok(distribution_result)
    }
    
    /// Determine target coin based on wallet stamp and transaction type
    fn determine_target_coin(&self, wallet_stamp: &BpiWalletStamp, is_gas: bool) -> Result<CoinType, BpiIntegrationError> {
        match wallet_stamp {
            // Bank wallets can access AUR coin
            BpiWalletStamp::Bank { .. } => Ok(CoinType::Aur),
            
            // Government wallets prefer GEN (governance)
            BpiWalletStamp::Government { .. } | BpiWalletStamp::Emergency { .. } => Ok(CoinType::Gen),
            
            // Regulated/Compliance wallets use NEX (mining/validation)
            BpiWalletStamp::Regulated { .. } | BpiWalletStamp::Compliance { .. } => Ok(CoinType::Nex),
            
            // Normal/Community wallets use FLX (network usage)
            BpiWalletStamp::Normal { .. } | BpiWalletStamp::Community { .. } => Ok(CoinType::Flx),
        }
    }
    
    /// End wallet session and process final payments
    pub async fn end_wallet_session(&self, wallet_id: &str) -> Result<Option<DistributionResult>, BpiIntegrationError> {
        let mut sessions = self.active_sessions.write().await;
        
        if let Some(mut session) = sessions.remove(wallet_id) {
            // Check minimum session duration
            let session_duration_minutes = (Utc::now() - session.session_start).num_minutes();
            if session_duration_minutes < self.config.minimum_session_minutes as i64 {
                return Err(BpiIntegrationError::InsufficientSessionDuration(session_duration_minutes as u32));
            }
            
            // Process any remaining rent
            let final_rent = {
                let hours = session_duration_minutes as f64 / 60.0;
                let total_rent = Decimal::from_f64(hours).unwrap_or(Decimal::ZERO) * self.config.rent_rate_per_hour;
                total_rent - session.rent_owed
            };
            
            if final_rent > Decimal::ZERO {
                let target_coin = self.determine_target_coin(&session.wallet_stamp, false)?;
                // Use treasury integration for coin distribution
                let treasury = &self.treasury_integration;
                let is_bank_wallet = matches!(session.wallet_stamp, BpiWalletStamp::Bank { .. });
                
                let result = treasury.process_fiat_inflow(final_rent, crate::autonomous_economy::FundSource::WalletRentPayments, &session.wallet_id)
                    .await.map_err(|e| BpiIntegrationError::PaymentFailed(e.to_string()))?;
                    
                // Convert treasury transaction to distribution result format
                let distribution_result = DistributionResult {
                    fiat_inflow: final_rent,
                    coin_type: target_coin,
                    coin_allocation: crate::autonomous_economy::CoinAllocation {
                        fixed_amount: result.coin_economy_amount,
                        claimable_amount: Decimal::ZERO,
                        transfer_to_mother: None,
                        coin_type: target_coin,
                    },
                    treasury_allocation: crate::autonomous_economy::TreasuryAllocation {
                        company_treasury: result.infrastructure_breakdown.company_api,
                        owner_salary: result.infrastructure_breakdown.owner_salary,
                        community_maintainers: result.infrastructure_breakdown.community_reserves,
                        infrastructure_treasury: result.infrastructure_amount,
                    },
                    timestamp: chrono::Utc::now(),
                };
                return Ok(Some(distribution_result));
            }
        }
        
        Ok(None)
    }
    
    /// Get session statistics
    pub async fn get_session_stats(&self) -> BpiSessionStats {
        let sessions = self.active_sessions.read().await;
        let transactions = self.transaction_history.read().await;
        
        let active_sessions = sessions.len();
        let total_rent_owed: Decimal = sessions.values().map(|s| s.rent_owed).sum();
        let total_gas_collected: Decimal = transactions.iter().map(|t| t.gas_fee).sum();
        let total_fiat_generated = total_rent_owed + total_gas_collected;
        
        BpiSessionStats {
            active_sessions,
            total_rent_owed,
            total_gas_collected,
            total_fiat_generated,
            total_transactions: transactions.len(),
        }
    }
    
    /// Get treasury status (replaces coin distribution)
    pub async fn get_treasury_status(&self) -> Result<serde_json::Value, BpiIntegrationError> {
        self.treasury_integration.get_treasury_status()
            .await.map_err(|e| BpiIntegrationError::PaymentFailed(e.to_string()))
    }
    
    /// Get work validator (read-only)
    pub async fn get_work_validator(&self) -> tokio::sync::RwLockReadGuard<WorkProofValidator> {
        self.work_validator.read().await
    }
}

/// BPI session statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpiSessionStats {
    pub active_sessions: usize,
    pub total_rent_owed: Decimal,
    pub total_gas_collected: Decimal,
    pub total_fiat_generated: Decimal,
    pub total_transactions: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_wallet_session_rent_calculation() {
        let config = BpiEconomicConfig::default();
        
        // Create real treasury integration for test
        let coin_distribution = Arc::new(RwLock::new(CoinDistributionEngine::new()));
        let treasury_config = TreasuryConfig::default();
        let treasury_integration = Arc::new(BpciTreasuryIntegration::new(
            coin_distribution,
            treasury_config,
            "test_company_wallet".to_string(),
            "test_owner_wallet".to_string(),
        ));
        
        let integration = BpiEconomicIntegration::new(config, treasury_integration);
        
        // Start session
        integration.start_wallet_session(
            "wallet123".to_string(),
            BpiWalletStamp::Normal {
                basic_verification: true,
                transaction_limits: TransactionLimits {
                    daily_limit: 10000,
                    monthly_limit: 100000,
                    single_transaction_limit: 5000,
                    cross_border_limit: 2000,
                },
            },
        ).await.unwrap();
        
        // Simulate 2 hours passing
        {
            let mut sessions = integration.active_sessions.write().await;
            let session = sessions.get_mut("wallet123").unwrap();
            session.session_start = Utc::now() - Duration::hours(2);
        }
        
        // Collect rent
        let results = integration.collect_rent_payments().await.unwrap();
        assert_eq!(results.len(), 1);
        
        // Should be $4 (2 hours * $2/hour)
        assert_eq!(results[0].fiat_inflow, Decimal::from(4));
    }
    
    #[tokio::test]
    async fn test_bpi_transaction_gas_fees() {
        let config = BpiEconomicConfig::default();
        
        // Create real treasury integration for test
        let coin_distribution = Arc::new(RwLock::new(CoinDistributionEngine::new()));
        let treasury_config = TreasuryConfig::default();
        let treasury_integration = Arc::new(BpciTreasuryIntegration::new(
            coin_distribution,
            treasury_config,
            "test_company_wallet".to_string(),
            "test_owner_wallet".to_string(),
        ));
        
        let integration = BpiEconomicIntegration::new(config, treasury_integration);
        
        // Start session
        integration.start_wallet_session(
            "wallet123".to_string(),
            BpiWalletStamp::Normal {
                basic_verification: true,
                transaction_limits: TransactionLimits {
                    daily_limit: 10000,
                    monthly_limit: 100000,
                    single_transaction_limit: 5000,
                    cross_border_limit: 2000,
                },
            },
        ).await.unwrap();
        
        // Process container deploy transaction
        let transaction = integration.process_bpi_transaction(
            "wallet123".to_string(),
            BpiTransactionType::ContainerDeploy { container_id: "container123".to_string() },
            None,
        ).await.unwrap();
        
        // Should charge $5 for container deploy
        assert_eq!(transaction.gas_fee, Decimal::from(5));
        assert_eq!(transaction.target_coin, CoinType::Flx); // Normal wallet uses FLX
    }
    
    #[tokio::test]
    async fn test_bank_wallet_aur_access() {
        let config = BpiEconomicConfig::default();
        
        // Create real treasury integration for test
        let coin_distribution = Arc::new(RwLock::new(CoinDistributionEngine::new()));
        let treasury_config = TreasuryConfig::default();
        let treasury_integration = Arc::new(BpciTreasuryIntegration::new(
            coin_distribution,
            treasury_config,
            "test_company_wallet".to_string(),
            "test_owner_wallet".to_string(),
        ));
        
        let integration = BpiEconomicIntegration::new(config, treasury_integration);
        
        // Start bank wallet session
        integration.start_wallet_session(
            "bank_wallet".to_string(),
            BpiWalletStamp::Bank {
                bank_type: BankType::Commercial,
                banking_license: BankLicense {
                    license_number: "TEST_LICENSE_001".to_string(),
                    issuing_authority: "Test Banking Authority".to_string(),
                    license_type: BankLicenseType::CommercialBank,
                    valid_until: chrono::Utc::now() + chrono::Duration::days(365),
                    jurisdiction: "Test Jurisdiction".to_string(),
                },
                regulatory_compliance: vec![],
                risk_management: RiskManagement {
                    risk_framework: "Standard Risk Framework".to_string(),
                    risk_tolerance: "Medium".to_string(),
                    monitoring_systems: vec!["Real-time Monitoring".to_string()],
                },
            },
        ).await.unwrap();
        
        // Process transaction
        let transaction = integration.process_bpi_transaction(
            "bank_wallet".to_string(),
            BpiTransactionType::CrossChainBridge { 
                source_chain: "ETH".to_string(), 
                target_chain: "BTC".to_string() 
            },
            None,
        ).await.unwrap();
        
        // Bank wallet should use AUR coin
        assert_eq!(transaction.target_coin, CoinType::Aur);
    }
    
    #[tokio::test]
    async fn test_formal_math_integration() {
        let config = BpiEconomicConfig::default();
        
        // Create real treasury integration for test
        let coin_distribution = Arc::new(RwLock::new(CoinDistributionEngine::new()));
        let treasury_config = TreasuryConfig::default();
        let treasury_integration = Arc::new(BpciTreasuryIntegration::new(
            coin_distribution,
            treasury_config,
            "test_company_wallet".to_string(),
            "test_owner_wallet".to_string(),
        ));
        
        let integration = BpiEconomicIntegration::new(config, treasury_integration);
        
        // Start session and process $100 worth of activity
        integration.start_wallet_session(
            "wallet123".to_string(),
            BpiWalletStamp::Normal {
                basic_verification: true,
                transaction_limits: TransactionLimits {
                    daily_limit: 10000,
                    monthly_limit: 100000,
                    single_transaction_limit: 5000,
                    cross_border_limit: 2000,
                },
            },
        ).await.unwrap();
        
        // Process transaction worth $100
        let transaction = integration.process_bpi_transaction(
            "wallet123".to_string(),
            BpiTransactionType::ContainerDeploy { container_id: "container123".to_string() },
            Some(Decimal::from(100)), // Custom $100 gas fee
        ).await.unwrap();
        
        // Check session stats and treasury status
        let stats = integration.get_session_stats().await;
        assert_eq!(stats.active_sessions, 1);
        assert!(stats.total_gas_collected > Decimal::ZERO);
        
        // Check treasury status is accessible
        let treasury_status = integration.get_treasury_status().await.unwrap();
        assert!(treasury_status.is_object());
        
        // Verify transaction was processed successfully
        assert!(transaction.transaction_id.len() > 0);
        assert_eq!(transaction.gas_fee, Decimal::from(100));
        
        // Check that session is still active
        let updated_stats = integration.get_session_stats().await;
        assert_eq!(updated_stats.active_sessions, 1);
    }
}
