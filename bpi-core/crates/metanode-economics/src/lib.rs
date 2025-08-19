// Metanode Economics Supercrate
// Consolidated economic systems and governance for BPI Core

//! # Metanode Economics
//! 
//! This supercrate consolidates all economic and governance functionality including:
//! - Autonomous economics with AI-driven decision making
//! - Billing and metering for resource usage
//! - Governance system with stake-weighted voting
//! - Economic incentives and reward distribution
//! - Cross-chain economic settlement

use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Error, Debug)]
pub enum EconomicsError {
    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    #[error("Governance proposal failed: {0}")]
    GovernanceProposalFailed(String),
    #[error("Economic calculation error: {0}")]
    CalculationError(String),
    #[error("Billing error: {0}")]
    BillingError(String),
}

/// Economic configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicsConfig {
    pub base_fee: Decimal,
    pub gas_price: Decimal,
    pub reward_rate: Decimal,
    pub inflation_rate: Decimal,
    pub governance_enabled: bool,
    pub autonomous_economics: bool,
}

impl Default for EconomicsConfig {
    fn default() -> Self {
        EconomicsConfig {
            base_fee: Decimal::from(1),
            gas_price: Decimal::from(10),
            reward_rate: Decimal::new(5, 2), // 0.05 (5%)
            inflation_rate: Decimal::new(2, 2), // 0.02 (2%)
            governance_enabled: true,
            autonomous_economics: true,
        }
    }
}

/// Account balance and economic state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub address: String,
    pub balance: Decimal,
    pub stake: Decimal,
    pub rewards_earned: Decimal,
    pub last_activity: DateTime<Utc>,
}

impl Account {
    pub fn new(address: String, initial_balance: Decimal) -> Self {
        Account {
            address,
            balance: initial_balance,
            stake: Decimal::ZERO,
            rewards_earned: Decimal::ZERO,
            last_activity: Utc::now(),
        }
    }

    pub fn transfer(&mut self, amount: Decimal) -> Result<(), EconomicsError> {
        if self.balance < amount {
            return Err(EconomicsError::InsufficientFunds(
                format!("Balance: {}, Required: {}", self.balance, amount)
            ));
        }
        self.balance -= amount;
        self.last_activity = Utc::now();
        Ok(())
    }

    pub fn receive(&mut self, amount: Decimal) {
        self.balance += amount;
        self.last_activity = Utc::now();
    }

    pub fn stake(&mut self, amount: Decimal) -> Result<(), EconomicsError> {
        if self.balance < amount {
            return Err(EconomicsError::InsufficientFunds(
                "Insufficient balance for staking".to_string()
            ));
        }
        self.balance -= amount;
        self.stake += amount;
        self.last_activity = Utc::now();
        Ok(())
    }

    pub fn unstake(&mut self, amount: Decimal) -> Result<(), EconomicsError> {
        if self.stake < amount {
            return Err(EconomicsError::InsufficientFunds(
                "Insufficient stake for unstaking".to_string()
            ));
        }
        self.stake -= amount;
        self.balance += amount;
        self.last_activity = Utc::now();
        Ok(())
    }
}

/// Economic engine for autonomous decision making
pub struct EconomicsEngine {
    config: EconomicsConfig,
    accounts: HashMap<String, Account>,
    total_supply: Decimal,
    governance_proposals: HashMap<Uuid, GovernanceProposal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceProposal {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub votes_for: Decimal,
    pub votes_against: Decimal,
    pub voting_deadline: DateTime<Utc>,
    pub status: ProposalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
}

impl EconomicsEngine {
    pub fn new(config: EconomicsConfig) -> Self {
        EconomicsEngine {
            config,
            accounts: HashMap::new(),
            total_supply: Decimal::ZERO, // Start with zero supply, tokens created via create_account
            governance_proposals: HashMap::new(),
        }
    }

    pub fn create_account(&mut self, address: String, initial_balance: Decimal) -> Result<(), EconomicsError> {
        if self.accounts.contains_key(&address) {
            return Err(EconomicsError::InvalidTransaction(
                "Account already exists".to_string()
            ));
        }

        let account = Account::new(address.clone(), initial_balance);
        self.accounts.insert(address, account);
        self.total_supply += initial_balance;
        Ok(())
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: Decimal) -> Result<(), EconomicsError> {
        // Get sender account
        let sender = self.accounts.get_mut(from)
            .ok_or_else(|| EconomicsError::InvalidTransaction("Sender account not found".to_string()))?;
        
        sender.transfer(amount)?;

        // Get or create receiver account
        if !self.accounts.contains_key(to) {
            self.create_account(to.to_string(), Decimal::ZERO)?;
        }

        let receiver = self.accounts.get_mut(to)
            .ok_or_else(|| EconomicsError::InvalidTransaction("Receiver account not found".to_string()))?;
        
        receiver.receive(amount);
        Ok(())
    }

    pub fn calculate_rewards(&self, validator: &str) -> Result<Decimal, EconomicsError> {
        let account = self.accounts.get(validator)
            .ok_or_else(|| EconomicsError::CalculationError("Validator not found".to_string()))?;

        let reward = account.stake * self.config.reward_rate;
        Ok(reward)
    }

    pub fn distribute_rewards(&mut self, validator: &str) -> Result<(), EconomicsError> {
        let reward = self.calculate_rewards(validator)?;
        
        let account = self.accounts.get_mut(validator)
            .ok_or_else(|| EconomicsError::CalculationError("Validator not found".to_string()))?;

        account.rewards_earned += reward;
        account.balance += reward;
        self.total_supply += reward; // Inflation
        Ok(())
    }

    pub fn create_governance_proposal(
        &mut self,
        title: String,
        description: String,
        proposer: String,
    ) -> Result<Uuid, EconomicsError> {
        if !self.config.governance_enabled {
            return Err(EconomicsError::GovernanceProposalFailed(
                "Governance is disabled".to_string()
            ));
        }

        let proposal_id = Uuid::new_v4();
        let proposal = GovernanceProposal {
            id: proposal_id,
            title,
            description,
            proposer,
            votes_for: Decimal::ZERO,
            votes_against: Decimal::ZERO,
            voting_deadline: Utc::now() + chrono::Duration::days(7),
            status: ProposalStatus::Active,
        };

        self.governance_proposals.insert(proposal_id, proposal);
        Ok(proposal_id)
    }

    pub fn vote_on_proposal(
        &mut self,
        proposal_id: Uuid,
        voter: &str,
        vote_for: bool,
    ) -> Result<(), EconomicsError> {
        let proposal = self.governance_proposals.get_mut(&proposal_id)
            .ok_or_else(|| EconomicsError::GovernanceProposalFailed("Proposal not found".to_string()))?;

        let account = self.accounts.get(voter)
            .ok_or_else(|| EconomicsError::GovernanceProposalFailed("Voter not found".to_string()))?;

        // Vote weight is based on stake
        let vote_weight = account.stake;

        if vote_for {
            proposal.votes_for += vote_weight;
        } else {
            proposal.votes_against += vote_weight;
        }

        Ok(())
    }

    pub fn finalize_proposal(&mut self, proposal_id: Uuid) -> Result<ProposalStatus, EconomicsError> {
        let proposal = self.governance_proposals.get_mut(&proposal_id)
            .ok_or_else(|| EconomicsError::GovernanceProposalFailed("Proposal not found".to_string()))?;

        if Utc::now() < proposal.voting_deadline {
            return Err(EconomicsError::GovernanceProposalFailed(
                "Voting period not ended".to_string()
            ));
        }

        let status = if proposal.votes_for > proposal.votes_against {
            ProposalStatus::Passed
        } else {
            ProposalStatus::Rejected
        };

        proposal.status = status.clone();
        Ok(status)
    }

    pub fn get_account(&self, address: &str) -> Option<&Account> {
        self.accounts.get(address)
    }

    pub fn get_total_supply(&self) -> Decimal {
        self.total_supply
    }
}

// Re-export consolidated modules (these would be implemented from moved crates)
pub mod autonomous_economics {
    //! Autonomous economics with AI-driven decision making
    pub use super::*;
    
    pub fn make_economic_decision(market_data: &str) -> Result<String, EconomicsError> {
        // AI-driven economic decision logic would be here
        Ok(format!("Economic decision based on: {}", market_data))
    }
    
    pub fn optimize_resource_pricing(usage: f64) -> Result<Decimal, EconomicsError> {
        // Resource pricing optimization logic would be here
        Ok(Decimal::from_f64_retain(usage * 1.1).unwrap_or(Decimal::ONE))
    }
}

pub mod billing_meter {
    //! Billing and metering for resource usage
    pub use super::*;
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UsageRecord {
        pub user_id: String,
        pub resource_type: String,
        pub usage_amount: Decimal,
        pub timestamp: DateTime<Utc>,
    }
    
    pub fn record_usage(record: UsageRecord) -> Result<(), EconomicsError> {
        // Usage recording logic would be here
        Ok(())
    }
    
    pub fn calculate_bill(user_id: &str) -> Result<Decimal, EconomicsError> {
        // Bill calculation logic would be here
        Ok(Decimal::from(100))
    }
}

pub mod governance {
    //! Governance system with stake-weighted voting
    pub use super::*;
    
    pub fn initialize_governance() -> Result<(), EconomicsError> {
        // Governance initialization logic would be here
        Ok(())
    }
    
    pub fn execute_proposal(proposal_id: Uuid) -> Result<(), EconomicsError> {
        // Proposal execution logic would be here
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_economics_engine_creation() {
        let config = EconomicsConfig::default();
        let engine = EconomicsEngine::new(config);
        assert_eq!(engine.total_supply, Decimal::ZERO);
    }

    #[test]
    fn test_account_creation() {
        let mut engine = EconomicsEngine::new(EconomicsConfig::default());
        let result = engine.create_account("alice".to_string(), Decimal::from(1000));
        assert!(result.is_ok());
        assert_eq!(engine.total_supply, Decimal::from(1000));
    }

    #[test]
    fn test_transfer() {
        let mut engine = EconomicsEngine::new(EconomicsConfig::default());
        engine.create_account("alice".to_string(), Decimal::from(1000)).unwrap();
        engine.create_account("bob".to_string(), Decimal::from(500)).unwrap();
        
        let result = engine.transfer("alice", "bob", Decimal::from(200));
        assert!(result.is_ok());
        
        let alice = engine.get_account("alice").unwrap();
        let bob = engine.get_account("bob").unwrap();
        
        assert_eq!(alice.balance, Decimal::from(800));
        assert_eq!(bob.balance, Decimal::from(700));
    }

    #[test]
    fn test_staking() {
        let mut account = Account::new("alice".to_string(), Decimal::from(1000));
        let result = account.stake(Decimal::from(500));
        assert!(result.is_ok());
        assert_eq!(account.balance, Decimal::from(500));
        assert_eq!(account.stake, Decimal::from(500));
    }

    #[test]
    fn test_governance_proposal() {
        let mut engine = EconomicsEngine::new(EconomicsConfig::default());
        let proposal_id = engine.create_governance_proposal(
            "Test Proposal".to_string(),
            "A test governance proposal".to_string(),
            "alice".to_string(),
        ).unwrap();
        
        assert!(engine.governance_proposals.contains_key(&proposal_id));
    }

    #[test]
    fn test_reward_calculation() {
        let mut engine = EconomicsEngine::new(EconomicsConfig::default());
        engine.create_account("validator".to_string(), Decimal::from(1000)).unwrap();
        
        // Stake some tokens
        let validator = engine.accounts.get_mut("validator").unwrap();
        validator.stake(Decimal::from(500)).unwrap();
        
        let reward = engine.calculate_rewards("validator").unwrap();
        assert_eq!(reward, Decimal::from(500) * Decimal::new(5, 2)); // 5% of stake
    }
}
