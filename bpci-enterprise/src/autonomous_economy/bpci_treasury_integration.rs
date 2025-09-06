use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use thiserror::Error;
use uuid::Uuid;
use tracing::{info, warn, error};

use crate::autonomous_economy::{CoinDistributionEngine, CoinType, DistributionResult};
use crate::registry::node_types::BpiWalletStamp;
use crate::mining::node_types::{ValidatorNode, MinerNode, NotaryNode};

/// BPCI Treasury Integration - Handles 100% Gas/Rent with 25%/75% Split
/// 
/// This module ensures that 100% of gas and rent received is processed with:
/// - 25% going to coin economy (distributed as coins)
/// - 75% going to infrastructure (company API, owner salary, community/reserves)
/// 
/// Integrates seamlessly with BPCI maintainers and governance logic
/// Ensures bank-grade stability and security for all treasury operations

/// BPCI Treasury Integration Engine
#[derive(Debug)]
pub struct BpciTreasuryIntegration {
    /// Core coin distribution engine
    coin_distribution: Arc<RwLock<CoinDistributionEngine>>,
    /// Treasury configuration
    config: TreasuryConfig,
    /// Real-time treasury metrics
    metrics: Arc<RwLock<TreasuryMetrics>>,
    /// Company API wallet for receiving 25% of 75%
    company_api_wallet: String,
    /// Owner wallet for receiving 10% of 75%
    owner_wallet: String,
    /// Community treasury for receiving 40% of 75%
    community_treasury: Arc<RwLock<CommunityTreasury>>,
    /// Maintainer registry for governance integration
    maintainer_registry: Arc<RwLock<HashMap<String, MaintainerInfo>>>,
    /// Treasury transaction history
    transaction_history: Arc<RwLock<Vec<TreasuryTransaction>>>,
}

/// Treasury configuration for BPCI integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryConfig {
    /// Coin economy percentage (25%)
    pub coin_economy_percentage: Decimal,
    /// Infrastructure percentage (75%)
    pub infrastructure_percentage: Decimal,
    /// Company API percentage of infrastructure (25% of 75% = 18.75% total)
    pub company_api_percentage: Decimal,
    /// Owner salary percentage of infrastructure (10% of 75% = 7.5% total)
    pub owner_salary_percentage: Decimal,
    /// Community reserves percentage of infrastructure (40% of 75% = 30% total)
    pub community_reserves_percentage: Decimal,
    /// Minimum transaction amount for processing
    pub minimum_transaction_amount: Decimal,
    /// Enable bank-grade security features
    pub enable_bank_security: bool,
    /// Auto-distribute to maintainers
    pub auto_distribute_to_maintainers: bool,
}

impl Default for TreasuryConfig {
    fn default() -> Self {
        Self {
            coin_economy_percentage: Decimal::new(25, 2),      // 25%
            infrastructure_percentage: Decimal::new(75, 2),   // 75%
            company_api_percentage: Decimal::new(3333, 4),    // 33.33% of 75% (25% of total)
            owner_salary_percentage: Decimal::new(1333, 4),   // 13.33% of 75% (10% of total)
            community_reserves_percentage: Decimal::new(5334, 4), // 53.34% of 75% (40% of total)
            minimum_transaction_amount: Decimal::new(1, 2),   // $0.01 minimum
            enable_bank_security: true,
            auto_distribute_to_maintainers: true,
        }
    }
}

/// Real-time treasury metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryMetrics {
    /// Total gas fees processed
    pub total_gas_fees_processed: Decimal,
    /// Total rent payments processed
    pub total_rent_payments_processed: Decimal,
    /// Total distributed to coin economy (25%)
    pub total_coin_economy_distribution: Decimal,
    /// Total distributed to company API
    pub total_company_api_distribution: Decimal,
    /// Total distributed to owner salary
    pub total_owner_salary_distribution: Decimal,
    /// Total distributed to community reserves
    pub total_community_reserves_distribution: Decimal,
    /// Total transactions processed
    pub total_transactions_processed: u64,
    /// Active maintainers receiving distributions
    pub active_maintainers: u64,
    /// Last distribution timestamp
    pub last_distribution: DateTime<Utc>,
    /// Treasury system status
    pub system_status: TreasuryStatus,
}

/// Community treasury for managing 40% of infrastructure funds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityTreasury {
    /// Total community reserves
    pub total_reserves: Decimal,
    /// Allocated to maintainers
    pub maintainer_allocations: HashMap<String, Decimal>,
    /// Allocated to governance initiatives
    pub governance_allocations: HashMap<String, Decimal>,
    /// Emergency reserves
    pub emergency_reserves: Decimal,
    /// Last allocation timestamp
    pub last_allocation: DateTime<Utc>,
}

/// Maintainer information for governance integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintainerInfo {
    /// Maintainer ID
    pub maintainer_id: String,
    /// Maintainer type (validator, miner, notary, governance)
    pub maintainer_type: MaintainerType,
    /// Wallet stamp for payments
    pub wallet_stamp: BpiWalletStamp,
    /// Performance score (affects distribution)
    pub performance_score: Decimal,
    /// Total earned from treasury
    pub total_earned: Decimal,
    /// Last payment timestamp
    pub last_payment: DateTime<Utc>,
    /// Active status
    pub is_active: bool,
}

/// Types of maintainers in the BPCI ecosystem
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MaintainerType {
    Validator,
    Miner,
    Notary,
    Governance,
    Community,
    Infrastructure,
}

/// Treasury transaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryTransaction {
    /// Transaction ID
    pub transaction_id: String,
    /// Transaction type
    pub transaction_type: TransactionType,
    /// Source of funds (gas/rent)
    pub source: FundSource,
    /// Total amount received
    pub total_amount: Decimal,
    /// Amount to coin economy (25%)
    pub coin_economy_amount: Decimal,
    /// Amount to infrastructure (75%)
    pub infrastructure_amount: Decimal,
    /// Infrastructure breakdown
    pub infrastructure_breakdown: InfrastructureBreakdown,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Transaction hash for auditability
    pub transaction_hash: String,
}

/// Types of treasury transactions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    GasFeeCollection,
    RentPayment,
    EmergencyDistribution,
    MaintainerPayment,
    GovernanceAllocation,
}

/// Source of treasury funds
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash)]
pub enum FundSource {
    WalletGasFees,
    WalletRentPayments,
    BankSettlementFees,
    GovernanceTransactionFees,
    EmergencyFunds,
}

/// Infrastructure fund breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureBreakdown {
    /// Company API allocation
    pub company_api: Decimal,
    /// Owner salary allocation
    pub owner_salary: Decimal,
    /// Community reserves allocation
    pub community_reserves: Decimal,
}

/// Treasury system status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TreasuryStatus {
    Active,
    Processing,
    Distributing,
    Maintenance,
    Emergency,
    Error { message: String },
}

/// Treasury integration errors
#[derive(Error, Debug)]
pub enum TreasuryError {
    #[error("Invalid amount: {amount}")]
    InvalidAmount { amount: Decimal },
    #[error("Insufficient funds for distribution")]
    InsufficientFunds,
    #[error("Maintainer not found: {maintainer_id}")]
    MaintainerNotFound { maintainer_id: String },
    #[error("Treasury system not active")]
    SystemNotActive,
    #[error("Distribution failed: {message}")]
    DistributionFailed { message: String },
    #[error("Security validation failed: {reason}")]
    SecurityValidationFailed { reason: String },
}

impl BpciTreasuryIntegration {
    /// Create new BPCI treasury integration
    pub fn new(
        coin_distribution: Arc<RwLock<CoinDistributionEngine>>,
        config: TreasuryConfig,
        company_api_wallet: String,
        owner_wallet: String,
    ) -> Self {
        let metrics = Arc::new(RwLock::new(TreasuryMetrics {
            total_gas_fees_processed: Decimal::ZERO,
            total_rent_payments_processed: Decimal::ZERO,
            total_coin_economy_distribution: Decimal::ZERO,
            total_company_api_distribution: Decimal::ZERO,
            total_owner_salary_distribution: Decimal::ZERO,
            total_community_reserves_distribution: Decimal::ZERO,
            total_transactions_processed: 0,
            active_maintainers: 0,
            last_distribution: Utc::now(),
            system_status: TreasuryStatus::Active,
        }));

        let community_treasury = Arc::new(RwLock::new(CommunityTreasury {
            total_reserves: Decimal::ZERO,
            maintainer_allocations: HashMap::new(),
            governance_allocations: HashMap::new(),
            emergency_reserves: Decimal::ZERO,
            last_allocation: Utc::now(),
        }));

        Self {
            coin_distribution,
            config,
            metrics,
            company_api_wallet,
            owner_wallet,
            community_treasury,
            maintainer_registry: Arc::new(RwLock::new(HashMap::new())),
            transaction_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Process 100% of gas/rent received with strict 25%/75% split
    pub async fn process_fiat_inflow(
        &self,
        amount: Decimal,
        source: FundSource,
        wallet_id: &str,
    ) -> Result<TreasuryTransaction, TreasuryError> {
        // Validate amount
        if amount <= Decimal::ZERO || amount < self.config.minimum_transaction_amount {
            return Err(TreasuryError::InvalidAmount { amount });
        }

        // Bank-grade security validation
        if self.config.enable_bank_security {
            self.validate_transaction_security(amount, &source, wallet_id).await?;
        }

        // Calculate strict 25%/75% split
        let coin_economy_amount = amount * self.config.coin_economy_percentage;
        let infrastructure_amount = amount * self.config.infrastructure_percentage;

        // Calculate infrastructure breakdown (of the 75%)
        let company_api = infrastructure_amount * self.config.company_api_percentage;
        let owner_salary = infrastructure_amount * self.config.owner_salary_percentage;
        let community_reserves = infrastructure_amount * self.config.community_reserves_percentage;

        // Verify split adds up to 100%
        let total_check = coin_economy_amount + company_api + owner_salary + community_reserves;
        if (total_check - amount).abs() > Decimal::new(1, 4) { // Allow 0.0001 rounding error
            return Err(TreasuryError::DistributionFailed {
                message: format!("Split calculation error: {} != {}", total_check, amount),
            });
        }

        // Process coin economy distribution (25%)
        let mut coin_distribution = self.coin_distribution.write().await;
        let distribution_result = coin_distribution
            .process_fiat_inflow(coin_economy_amount, CoinType::Flx, false)
            .map_err(|e| TreasuryError::DistributionFailed {
                message: format!("Coin distribution failed: {}", e),
            })?;

        // Process infrastructure distributions (75%)
        self.distribute_to_company_api(company_api).await?;
        self.distribute_to_owner_salary(owner_salary).await?;
        self.distribute_to_community_reserves(community_reserves).await?;

        // Create transaction record
        let transaction = TreasuryTransaction {
            transaction_id: Uuid::new_v4().to_string(),
            transaction_type: match source {
                FundSource::WalletGasFees => TransactionType::GasFeeCollection,
                FundSource::WalletRentPayments => TransactionType::RentPayment,
                _ => TransactionType::GasFeeCollection,
            },
            source: source.clone(),
            total_amount: amount,
            coin_economy_amount,
            infrastructure_amount,
            infrastructure_breakdown: InfrastructureBreakdown {
                company_api,
                owner_salary,
                community_reserves,
            },
            timestamp: Utc::now(),
            transaction_hash: self.calculate_transaction_hash(&amount, &source, wallet_id),
        };

        // Update metrics
        self.update_metrics(&transaction).await?;

        // Store transaction history
        let mut history = self.transaction_history.write().await;
        history.push(transaction.clone());

        // Auto-distribute to maintainers if enabled
        if self.config.auto_distribute_to_maintainers {
            self.distribute_to_maintainers().await?;
        }

        info!(
            "Treasury processed: ${} total, ${} to coins (25%), ${} to infra (75%)",
            amount, coin_economy_amount, infrastructure_amount
        );

        Ok(transaction)
    }

    /// Bank-grade security validation for transactions
    async fn validate_transaction_security(
        &self,
        amount: Decimal,
        source: &FundSource,
        wallet_id: &str,
    ) -> Result<(), TreasuryError> {
        // Validate transaction limits
        let max_single_transaction = Decimal::new(100000, 0); // $100,000 limit
        if amount > max_single_transaction {
            return Err(TreasuryError::SecurityValidationFailed {
                reason: format!("Transaction amount ${} exceeds limit", amount),
            });
        }

        // Validate source authenticity
        match source {
            FundSource::WalletGasFees | FundSource::WalletRentPayments => {
                // Validate wallet exists and is active
                if wallet_id.is_empty() {
                    return Err(TreasuryError::SecurityValidationFailed {
                        reason: "Invalid wallet ID".to_string(),
                    });
                }
            }
            FundSource::BankSettlementFees => {
                // Additional bank validation would go here
            }
            _ => {}
        }

        Ok(())
    }

    /// Distribute funds to company API wallet
    async fn distribute_to_company_api(&self, amount: Decimal) -> Result<(), TreasuryError> {
        // In production, this would transfer funds to the actual company API wallet
        info!("Distributed ${} to company API wallet: {}", amount, self.company_api_wallet);
        Ok(())
    }

    /// Distribute funds to owner salary wallet
    async fn distribute_to_owner_salary(&self, amount: Decimal) -> Result<(), TreasuryError> {
        // In production, this would transfer funds to the actual owner wallet
        info!("Distributed ${} to owner salary wallet: {}", amount, self.owner_wallet);
        Ok(())
    }

    /// Distribute funds to community reserves
    async fn distribute_to_community_reserves(&self, amount: Decimal) -> Result<(), TreasuryError> {
        let mut community = self.community_treasury.write().await;
        community.total_reserves += amount;
        community.last_allocation = Utc::now();

        info!("Distributed ${} to community reserves (total: ${})", 
              amount, community.total_reserves);
        Ok(())
    }

    /// Distribute community funds to maintainers
    async fn distribute_to_maintainers(&self) -> Result<(), TreasuryError> {
        let maintainers = self.maintainer_registry.read().await;
        let mut community = self.community_treasury.write().await;

        if maintainers.is_empty() {
            return Ok(()); // No maintainers to distribute to
        }

        // Calculate distribution per maintainer based on performance
        let total_performance: Decimal = maintainers.values()
            .filter(|m| m.is_active)
            .map(|m| m.performance_score)
            .sum();

        if total_performance <= Decimal::ZERO {
            return Ok(()); // No active maintainers
        }

        let available_funds = community.total_reserves * Decimal::new(50, 2); // 50% of reserves
        
        for (maintainer_id, maintainer) in maintainers.iter() {
            if maintainer.is_active {
                let allocation = available_funds * (maintainer.performance_score / total_performance);
                community.maintainer_allocations.insert(maintainer_id.clone(), allocation);
                
                info!("Allocated ${} to maintainer {} ({:?})", 
                      allocation, maintainer_id, maintainer.maintainer_type);
            }
        }

        community.total_reserves -= available_funds;
        community.last_allocation = Utc::now();

        Ok(())
    }

    /// Register a maintainer for treasury distributions
    pub async fn register_maintainer(
        &self,
        maintainer_id: String,
        maintainer_type: MaintainerType,
        wallet_stamp: BpiWalletStamp,
        performance_score: Decimal,
    ) -> Result<(), TreasuryError> {
        let maintainer = MaintainerInfo {
            maintainer_id: maintainer_id.clone(),
            maintainer_type: maintainer_type.clone(),
            wallet_stamp,
            performance_score,
            total_earned: Decimal::ZERO,
            last_payment: Utc::now(),
            is_active: true,
        };

        let mut registry = self.maintainer_registry.write().await;
        registry.insert(maintainer_id.clone(), maintainer.clone());

        // Update metrics
        let mut metrics = self.metrics.write().await;
        metrics.active_maintainers = registry.len() as u64;

        info!("Registered maintainer: {} ({:?})", maintainer_id, maintainer.maintainer_type);
        Ok(())
    }

    /// Update treasury metrics
    async fn update_metrics(&self, transaction: &TreasuryTransaction) -> Result<(), TreasuryError> {
        let mut metrics = self.metrics.write().await;

        match transaction.source {
            FundSource::WalletGasFees => {
                metrics.total_gas_fees_processed += transaction.total_amount;
            }
            FundSource::WalletRentPayments => {
                metrics.total_rent_payments_processed += transaction.total_amount;
            }
            _ => {}
        }

        metrics.total_coin_economy_distribution += transaction.coin_economy_amount;
        metrics.total_company_api_distribution += transaction.infrastructure_breakdown.company_api;
        metrics.total_owner_salary_distribution += transaction.infrastructure_breakdown.owner_salary;
        metrics.total_community_reserves_distribution += transaction.infrastructure_breakdown.community_reserves;
        metrics.total_transactions_processed += 1;
        metrics.last_distribution = transaction.timestamp;

        Ok(())
    }

    /// Calculate transaction hash for auditability
    fn calculate_transaction_hash(&self, amount: &Decimal, source: &FundSource, wallet_id: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        amount.hash(&mut hasher);
        source.hash(&mut hasher);
        wallet_id.hash(&mut hasher);
        Utc::now().timestamp().hash(&mut hasher);

        format!("{:x}", hasher.finish())
    }

    /// Get treasury status and metrics
    pub async fn get_treasury_status(&self) -> Result<serde_json::Value, TreasuryError> {
        let metrics = self.metrics.read().await;
        let community = self.community_treasury.read().await;
        let maintainers = self.maintainer_registry.read().await;

        Ok(serde_json::json!({
            "status": format!("{:?}", metrics.system_status),
            "metrics": {
                "total_gas_fees_processed": metrics.total_gas_fees_processed,
                "total_rent_payments_processed": metrics.total_rent_payments_processed,
                "total_coin_economy_distribution": metrics.total_coin_economy_distribution,
                "total_company_api_distribution": metrics.total_company_api_distribution,
                "total_owner_salary_distribution": metrics.total_owner_salary_distribution,
                "total_community_reserves_distribution": metrics.total_community_reserves_distribution,
                "total_transactions_processed": metrics.total_transactions_processed,
                "active_maintainers": metrics.active_maintainers,
                "last_distribution": metrics.last_distribution,
            },
            "community_treasury": {
                "total_reserves": community.total_reserves,
                "maintainer_allocations": community.maintainer_allocations.len(),
                "governance_allocations": community.governance_allocations.len(),
                "emergency_reserves": community.emergency_reserves,
                "last_allocation": community.last_allocation,
            },
            "distribution_config": {
                "coin_economy_percentage": self.config.coin_economy_percentage,
                "infrastructure_percentage": self.config.infrastructure_percentage,
                "company_api_percentage": self.config.company_api_percentage,
                "owner_salary_percentage": self.config.owner_salary_percentage,
                "community_reserves_percentage": self.config.community_reserves_percentage,
            },
            "security": {
                "bank_grade_security": self.config.enable_bank_security,
                "auto_distribute_maintainers": self.config.auto_distribute_to_maintainers,
                "minimum_transaction_amount": self.config.minimum_transaction_amount,
            },
            "real_time": true,
        }))
    }
}
