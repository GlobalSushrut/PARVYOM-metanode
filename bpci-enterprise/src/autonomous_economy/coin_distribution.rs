use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use anyhow::Result;
use thiserror::Error;

/// Formal Mathematical Model for Autonomous Economy Coin Distribution
/// 
/// Based on the mathematical specification:
/// F = C + T where C = 0.25F (Coin Economy) and T = 0.75F (Treasury)
/// 
/// Mother Coin: C_fix^M = 0.125F, C_claim^M = 0.125F
/// Daughter Coin: C_fix^D = 0.075F, C_claim^D = 0.125F, C_fix_to_M = 0.075F
/// Treasury: T_company = 0.1875F, T_owner = 0.10F, T_community = 0.20F, T_infra = 0.20F

/// Coin types in the autonomous economy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum CoinType {
    /// GEN - Mother Coin (Governance Reserve)
    Gen,
    /// NEX - Daughter Coin (PoE Mining)
    Nex,
    /// FLX - Daughter Coin (Network Usage)
    Flx,
    /// AUR - Bank-Stamped Only (Gold-Backed)
    Aur,
}

impl CoinType {
    pub fn is_mother_coin(&self) -> bool {
        matches!(self, CoinType::Gen)
    }
    
    pub fn is_daughter_coin(&self) -> bool {
        matches!(self, CoinType::Nex | CoinType::Flx)
    }
    
    pub fn is_bank_coin(&self) -> bool {
        matches!(self, CoinType::Aur)
    }
}

/// Distribution calculation result for a single fiat inflow F
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionResult {
    /// Original fiat inflow amount
    pub fiat_inflow: Decimal,
    /// Coin type that received the payment
    pub coin_type: CoinType,
    /// Coin economy allocation (25% of F)
    pub coin_allocation: CoinAllocation,
    /// Treasury allocation (75% of F)  
    pub treasury_allocation: TreasuryAllocation,
    /// Timestamp of distribution
    pub timestamp: DateTime<Utc>,
}

/// Coin economy allocation (C = 0.25F)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinAllocation {
    /// Fixed/reserve portion that cannot be withdrawn
    pub fixed_amount: Decimal,
    /// Claimable portion that can be withdrawn
    pub claimable_amount: Decimal,
    /// Amount transferred to mother coin (for daughter coins only)
    pub transfer_to_mother: Option<Decimal>,
    /// Receiving coin type
    pub coin_type: CoinType,
}

/// Treasury allocation (T = 0.75F)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryAllocation {
    /// Company treasury (0.1875F)
    pub company_treasury: Decimal,
    /// Owner salary (0.10F)
    pub owner_salary: Decimal,
    /// Community maintainers (0.20F)
    pub community_maintainers: Decimal,
    /// Infrastructure treasury - governance locked (0.20F)
    pub infrastructure_treasury: Decimal,
}

/// Coin state tracking for continuous growth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinState {
    /// Coin type
    pub coin_type: CoinType,
    /// Total fixed/reserve amount (immutable)
    pub total_fixed: Decimal,
    /// Total claimable amount (withdrawable)
    pub total_claimable: Decimal,
    /// Number of transactions processed
    pub transaction_count: u64,
    /// Total fiat inflow processed
    pub total_fiat_processed: Decimal,
    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
}

impl Default for CoinState {
    fn default() -> Self {
        Self {
            coin_type: CoinType::Gen,
            total_fixed: Decimal::ZERO,
            total_claimable: Decimal::ZERO,
            transaction_count: 0,
            total_fiat_processed: Decimal::ZERO,
            last_updated: Utc::now(),
        }
    }
}

/// Treasury state tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryState {
    /// Company treasury balance
    pub company_balance: Decimal,
    /// Owner salary balance
    pub owner_balance: Decimal,
    /// Community maintainers balance
    pub community_balance: Decimal,
    /// Infrastructure treasury balance (governance-locked)
    pub infrastructure_balance: Decimal,
    /// Total treasury processed
    pub total_processed: Decimal,
    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
}

impl Default for TreasuryState {
    fn default() -> Self {
        Self {
            company_balance: Decimal::ZERO,
            owner_balance: Decimal::ZERO,
            community_balance: Decimal::ZERO,
            infrastructure_balance: Decimal::ZERO,
            total_processed: Decimal::ZERO,
            last_updated: Utc::now(),
        }
    }
}

/// Distribution engine errors
#[derive(Error, Debug)]
pub enum DistributionError {
    #[error("Invalid fiat amount: {0}")]
    InvalidFiatAmount(Decimal),
    #[error("Bank coin access denied for non-bank wallet")]
    BankCoinAccessDenied,
    #[error("Calculation error: {0}")]
    CalculationError(String),
    #[error("State update error: {0}")]
    StateUpdateError(String),
}

/// Formal coin distribution engine implementing the mathematical model
#[derive(Debug)]
pub struct CoinDistributionEngine {
    /// Current state of all coins
    coin_states: HashMap<CoinType, CoinState>,
    /// Treasury state
    treasury_state: TreasuryState,
    /// Distribution history
    distribution_history: Vec<DistributionResult>,
}

impl CoinDistributionEngine {
    /// Create new distribution engine with zero initial values
    pub fn new() -> Self {
        let mut coin_states = HashMap::new();
        
        // Initialize all coins with zero values (earn value through real work)
        for coin_type in [CoinType::Gen, CoinType::Nex, CoinType::Flx, CoinType::Aur] {
            coin_states.insert(coin_type, CoinState {
                coin_type,
                ..Default::default()
            });
        }
        
        Self {
            coin_states,
            treasury_state: TreasuryState::default(),
            distribution_history: Vec::new(),
        }
    }
    
    /// Process fiat inflow according to formal mathematical model
    /// Treasury Distribution: 75% to infrastructure, 25% to coins
    /// Infrastructure Split: 25% Company API, 10% Owner Salary, 40% Community/Reserves
    pub fn process_fiat_inflow(
        &mut self, 
        fiat_amount: Decimal, 
        target_coin: CoinType, 
        is_emergency: bool
    ) -> Result<DistributionResult, DistributionError> {
        // Validate fiat amount
        if fiat_amount <= Decimal::ZERO {
            return Err(DistributionError::InvalidFiatAmount(fiat_amount));
        }

        // Validate coin access restrictions
        // AUR coins are restricted to bank/emergency access only
        if target_coin == CoinType::Aur && !is_emergency {
            return Err(DistributionError::BankCoinAccessDenied);
        }

        // Calculate treasury splits (75% to infrastructure, 25% to coins)
        let infrastructure_amount = fiat_amount * Decimal::new(75, 2); // 75%
        let coin_amount = fiat_amount * Decimal::new(25, 2);           // 25%

        // Infrastructure distribution (75% total, distributed proportionally):
        // Based on the 75% infrastructure amount, distribute as specified ratios
        let company_api_treasury = infrastructure_amount * Decimal::new(25, 2);    // 25% of 75%
        let owner_salary = infrastructure_amount * Decimal::new(1333, 4);          // 13.33% of 75%  
        let community_reserves = infrastructure_amount * Decimal::new(2667, 4);    // 26.67% of 75%
        let infrastructure_treasury = infrastructure_amount * Decimal::new(35, 2); // 35% of 75%

        // Update treasury state with correct distribution
        self.treasury_state.company_balance += company_api_treasury;
        self.treasury_state.owner_balance += owner_salary;
        self.treasury_state.community_balance += community_reserves;
        self.treasury_state.infrastructure_balance += infrastructure_treasury;
        self.treasury_state.total_processed += fiat_amount;

        // Calculate coin allocation based on mathematical model
        let coin_allocation = self.calculate_coin_allocation(coin_amount, target_coin)?;

        // Create treasury allocation
        let treasury_allocation = TreasuryAllocation {
            company_treasury: company_api_treasury,
            owner_salary,
            community_maintainers: community_reserves,
            infrastructure_treasury,
        };

        // Update coin states
        self.update_coin_states(&coin_allocation, fiat_amount)?;

        // Create distribution result
        let result = DistributionResult {
            fiat_inflow: fiat_amount,
            coin_type: target_coin,
            coin_allocation,
            treasury_allocation,
            timestamp: Utc::now(),
        };
        
        // Store in history
        self.distribution_history.push(result.clone());
        
        Ok(result)
    }
    
    /// Calculate coin allocation based on mathematical model
    fn calculate_coin_allocation(
        &self, 
        coin_share: Decimal, 
        coin_type: CoinType
    ) -> Result<CoinAllocation, DistributionError> {
        match coin_type {
            // Mother Coin: C_fix^M = 0.5C, C_claim^M = 0.5C
            CoinType::Gen => {
                let fixed_amount = coin_share * Decimal::new(5, 1);    // 0.5C = 0.125F
                let claimable_amount = coin_share * Decimal::new(5, 1); // 0.5C = 0.125F
                
                Ok(CoinAllocation {
                    fixed_amount,
                    claimable_amount,
                    transfer_to_mother: None,
                    coin_type,
                })
            },
            
            // Daughter Coins: C_fix^D = 0.3C, C_claim^D = 0.5C, C_fix_to_M = 0.3C
            CoinType::Nex | CoinType::Flx => {
                let fixed_amount = coin_share * Decimal::new(3, 1);    // 0.3C = 0.075F
                let claimable_amount = coin_share * Decimal::new(5, 1); // 0.5C = 0.125F
                let transfer_to_mother = coin_share * Decimal::new(3, 1); // 0.3C = 0.075F
                
                Ok(CoinAllocation {
                    fixed_amount,
                    claimable_amount,
                    transfer_to_mother: Some(transfer_to_mother),
                    coin_type,
                })
            },
            
            // Bank Coin: 100% stays in AUR (no mother transfer)
            CoinType::Aur => {
                let fixed_amount = coin_share * Decimal::new(5, 1);    // 0.5C
                let claimable_amount = coin_share * Decimal::new(5, 1); // 0.5C
                
                Ok(CoinAllocation {
                    fixed_amount,
                    claimable_amount,
                    transfer_to_mother: None,
                    coin_type,
                })
            },
        }
    }
    
    /// Calculate treasury allocation: T_company = 0.1875F, T_owner = 0.10F, etc.
    fn calculate_treasury_allocation(
        &self, 
        treasury_share: Decimal
    ) -> Result<TreasuryAllocation, DistributionError> {
        // Treasury split ratios (T = 0.75F)
        let company_ratio = Decimal::new(25, 2);    // 25% of T = 0.1875F
        let owner_ratio = Decimal::new(1333, 4);    // 13.33% of T ≈ 0.10F
        let community_ratio = Decimal::new(2666, 4); // 26.66% of T ≈ 0.20F
        let infra_ratio = Decimal::new(2666, 4);     // 26.66% of T ≈ 0.20F
        
        Ok(TreasuryAllocation {
            company_treasury: treasury_share * company_ratio,
            owner_salary: treasury_share * owner_ratio,
            community_maintainers: treasury_share * community_ratio,
            infrastructure_treasury: treasury_share * infra_ratio,
        })
    }
    
    /// Update coin states with new allocation
    fn update_coin_states(
        &mut self, 
        allocation: &CoinAllocation, 
        fiat_amount: Decimal
    ) -> Result<(), DistributionError> {
        let now = Utc::now();
        
        // Update receiving coin state
        if let Some(coin_state) = self.coin_states.get_mut(&allocation.coin_type) {
            coin_state.total_fixed += allocation.fixed_amount;
            coin_state.total_claimable += allocation.claimable_amount;
            coin_state.transaction_count += 1;
            coin_state.total_fiat_processed += fiat_amount;
            coin_state.last_updated = now;
        }
        
        // Update mother coin if transfer exists (daughter coin case)
        if let Some(transfer_amount) = allocation.transfer_to_mother {
            if let Some(mother_state) = self.coin_states.get_mut(&CoinType::Gen) {
                mother_state.total_fixed += transfer_amount;
                mother_state.last_updated = now;
            }
        }
        
        Ok(())
    }
    
    /// Update treasury state with new allocation
    fn update_treasury_state(
        &mut self, 
        allocation: &TreasuryAllocation
    ) -> Result<(), DistributionError> {
        self.treasury_state.company_balance += allocation.company_treasury;
        self.treasury_state.owner_balance += allocation.owner_salary;
        self.treasury_state.community_balance += allocation.community_maintainers;
        self.treasury_state.infrastructure_balance += allocation.infrastructure_treasury;
        
        self.treasury_state.total_processed += 
            allocation.company_treasury + 
            allocation.owner_salary + 
            allocation.community_maintainers + 
            allocation.infrastructure_treasury;
            
        self.treasury_state.last_updated = Utc::now();
        
        Ok(())
    }
    
    /// Get current coin state
    pub fn get_coin_state(&self, coin_type: CoinType) -> Option<&CoinState> {
        self.coin_states.get(&coin_type)
    }
    
    /// Get current treasury state
    pub fn get_treasury_state(&self) -> &TreasuryState {
        &self.treasury_state
    }
    
    /// Get distribution history
    pub fn get_distribution_history(&self) -> &[DistributionResult] {
        &self.distribution_history
    }
    
    /// Calculate continuous growth function for mother coin reserve
    /// M(n) = Σ(i=1 to n) [0.125F_i + 0.075F_i^D] where F_i^D are daughter coin transactions
    pub fn calculate_mother_coin_growth(&self) -> Decimal {
        self.coin_states.get(&CoinType::Gen)
            .map(|state| state.total_fixed)
            .unwrap_or(Decimal::ZERO)
    }
    
    /// Calculate total system value across all coins
    pub fn calculate_total_system_value(&self) -> Decimal {
        self.coin_states.values()
            .map(|state| state.total_fixed + state.total_claimable)
            .sum()
    }
    
    /// Get system statistics
    pub fn get_system_stats(&self) -> SystemStats {
        let total_transactions: u64 = self.coin_states.values()
            .map(|state| state.transaction_count)
            .sum();
            
        let total_fiat_processed: Decimal = self.coin_states.values()
            .map(|state| state.total_fiat_processed)
            .sum();
            
        SystemStats {
            total_transactions,
            total_fiat_processed,
            total_coin_value: self.calculate_total_system_value(),
            total_treasury_value: self.treasury_state.total_processed,
            mother_coin_reserve: self.calculate_mother_coin_growth(),
        }
    }
}

/// System statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub total_transactions: u64,
    pub total_fiat_processed: Decimal,
    pub total_coin_value: Decimal,
    pub total_treasury_value: Decimal,
    pub mother_coin_reserve: Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mother_coin_distribution() {
        let mut engine = CoinDistributionEngine::new();
        let result = engine.process_fiat_inflow(
            Decimal::from(100), 
            CoinType::Gen, 
            false
        ).unwrap();
        
        // Test mathematical model: F = 100
        assert_eq!(result.fiat_inflow, Decimal::from(100));
        assert_eq!(result.coin_allocation.fixed_amount, Decimal::new(125, 1)); // 12.5
        assert_eq!(result.coin_allocation.claimable_amount, Decimal::new(125, 1)); // 12.5
        assert_eq!(result.treasury_allocation.company_treasury, Decimal::new(1875, 2)); // 18.75
    }
    
    #[test]
    fn test_daughter_coin_distribution() {
        let mut engine = CoinDistributionEngine::new();
        let result = engine.process_fiat_inflow(
            Decimal::from(100), 
            CoinType::Nex, 
            false
        ).unwrap();
        
        // Test daughter coin math
        assert_eq!(result.coin_allocation.fixed_amount, Decimal::new(75, 1)); // 7.5
        assert_eq!(result.coin_allocation.claimable_amount, Decimal::new(125, 1)); // 12.5
        assert_eq!(result.coin_allocation.transfer_to_mother.unwrap(), Decimal::new(75, 1)); // 7.5
        
        // Check mother coin received transfer
        let mother_state = engine.get_coin_state(CoinType::Gen).unwrap();
        assert_eq!(mother_state.total_fixed, Decimal::new(75, 1)); // 7.5
    }
    
    #[test]
    fn test_treasury_split() {
        let mut engine = CoinDistributionEngine::new();
        let result = engine.process_fiat_inflow(
            Decimal::from(100), 
            CoinType::Gen, 
            false
        ).unwrap();
        
        let treasury = &result.treasury_allocation;
        
        // Verify treasury split adds up to 75% of F
        let total_treasury = treasury.company_treasury + 
                           treasury.owner_salary + 
                           treasury.community_maintainers + 
                           treasury.infrastructure_treasury;
        assert_eq!(total_treasury, Decimal::from(75));
    }
    
    #[test]
    fn test_continuous_growth() {
        let mut engine = CoinDistributionEngine::new();
        
        // Process multiple transactions
        engine.process_fiat_inflow(Decimal::from(100), CoinType::Gen, false).unwrap();
        engine.process_fiat_inflow(Decimal::from(100), CoinType::Nex, false).unwrap();
        engine.process_fiat_inflow(Decimal::from(100), CoinType::Flx, false).unwrap();
        
        // Mother coin should have: 12.5 (direct) + 7.5 (from NEX) + 7.5 (from FLX) = 27.5
        let mother_growth = engine.calculate_mother_coin_growth();
        assert_eq!(mother_growth, Decimal::new(275, 1)); // 27.5
    }
    
    #[test]
    fn test_bank_coin_restriction() {
        let mut engine = CoinDistributionEngine::new();
        
        // Should fail for non-bank wallet
        let result = engine.process_fiat_inflow(
            Decimal::from(100), 
            CoinType::Aur, 
            false
        );
        assert!(result.is_err());
        
        // Should succeed for bank wallet
        let result = engine.process_fiat_inflow(
            Decimal::from(100), 
            CoinType::Aur, 
            true
        );
        assert!(result.is_ok());
    }
}
