//! Autonomous Runes Engine - Economic Incentives for httpcg Domain Operations
//! 
//! This module provides autonomous economic incentives through rune-based staking,
//! governance tokens, and reward distribution for domain registry operations.

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, debug};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

use crate::httpcg_domain_registry::{RuneType, RunePool, StakingContract, DomainRegistrationRequest};

/// Domain pricing structure
#[derive(Debug, Clone)]
pub struct DomainPricing {
    pub base_price: f64,
    pub premium_multiplier: f64,
    pub total_price: f64,
    pub annual_cost: f64,
    pub staking_requirement: f64,
    pub governance_fee: f64,
    pub security_deposit: f64,
}

/// Staking result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakingResult {
    pub contract_id: String,
    pub staked_amount: f64,
    pub governance_tokens_issued: f64,
    pub estimated_annual_yield: f64,
    pub expected_rewards: f64,
    pub governance_tokens_earned: f64,
    pub lock_period: Duration,
}

/// Staking status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StakingStatus {
    Active,
    Locked,
    Withdrawn,
    Slashed,
}

/// Autonomous Runes Engine Implementation
#[derive(Debug)]
pub struct AutonomousRunesEngine {
    rune_pools: Arc<RwLock<HashMap<RuneType, RunePool>>>,
    staking_contracts: Arc<RwLock<HashMap<String, StakingContract>>>,
    reward_distribution: Arc<RwLock<RewardDistributionEngine>>,
    governance_tokens: Arc<RwLock<HashMap<String, GovernanceToken>>>,
    economic_coordinator: Arc<RuneEconomicCoordinator>,
}

/// Reward Distribution Engine
#[derive(Debug)]
pub struct RewardDistributionEngine {
    distribution_schedule: HashMap<RuneType, DistributionSchedule>,
    pending_rewards: HashMap<String, f64>,
    reward_history: Vec<RewardDistribution>,
    total_distributed: f64,
}

/// Governance Token for domain voting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceToken {
    pub token_id: String,
    pub holder_did: String,
    pub voting_power: f64,
    pub domain_associations: Vec<String>,
    pub earned_from: TokenEarnSource,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
}

/// Rune Economic Coordinator
#[derive(Debug)]
pub struct RuneEconomicCoordinator {
    coin_integration: Arc<CoinIntegration>,
    treasury_bridge: Arc<TreasuryBridge>,
    market_dynamics: Arc<RwLock<MarketDynamics>>,
    economic_metrics: Arc<RwLock<RuneEconomicMetrics>>,
}

/// Distribution Schedule for rune rewards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionSchedule {
    pub rune_type: RuneType,
    pub base_reward_rate: f64,
    pub distribution_frequency: Duration,
    pub multiplier_factors: HashMap<String, f64>,
    pub max_rewards_per_period: f64,
}

/// Reward Distribution Record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardDistribution {
    pub distribution_id: String,
    pub rune_type: RuneType,
    pub recipient_did: String,
    pub amount: f64,
    pub reason: RewardReason,
    pub distributed_at: DateTime<Utc>,
}

/// Token Earning Sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenEarnSource {
    DomainRegistration,
    StakingRewards,
    GovernanceParticipation,
    SecurityContribution,
    ResolutionServices,
}

/// Reward Reasons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RewardReason {
    StakingReward,
    GovernanceVoting,
    DomainMaintenance,
    SecurityValidation,
    NetworkContribution,
}

/// Coin Integration with existing autonomous economy
#[derive(Debug)]
pub struct CoinIntegration {
    gen_allocation: f64,  // GEN coin for governance
    nex_allocation: f64,  // NEX coin for network operations
    flx_allocation: f64,  // FLX coin for flexibility rewards
    aur_allocation: f64,  // AUR coin for premium features
}

/// Treasury Bridge to existing BPCI treasury
#[derive(Debug)]
pub struct TreasuryBridge {
    treasury_endpoint: String,
    allocation_ratios: HashMap<RuneType, f64>,
    reserve_requirements: HashMap<RuneType, f64>,
}

/// Market Dynamics for rune pricing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDynamics {
    pub supply_demand_ratio: HashMap<RuneType, f64>,
    pub price_volatility: HashMap<RuneType, f64>,
    pub staking_pressure: f64,
    pub governance_activity: f64,
    pub last_updated: DateTime<Utc>,
}

/// Rune Economic Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuneEconomicMetrics {
    pub total_value_locked: f64,
    pub active_stakers: u64,
    pub governance_participation: f64,
    pub reward_distribution_rate: f64,
    pub economic_health_score: f64,
}

// StakingResult struct already defined above - removing duplicate

/// Rune Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuneStatistics {
    pub total_staked: f64,
    pub active_contracts: u64,
    pub total_rewards_distributed: f64,
    pub governance_tokens_issued: u64,
    pub economic_health: f64,
}

impl AutonomousRunesEngine {
    /// Create new Autonomous Runes Engine
    pub async fn new() -> Result<Self> {
        info!("🎯 Initializing Autonomous Runes Engine");

        let mut rune_pools = HashMap::new();
        
        // Initialize rune pools for different operations
        rune_pools.insert(RuneType::RegistrationRune, RunePool {
            rune_type: RuneType::RegistrationRune,
            total_staked: 0.0,
            reward_rate: 0.05, // 5% annual reward
            participants: HashMap::new(),
            last_distribution: Utc::now(),
        });

        rune_pools.insert(RuneType::GovernanceRune, RunePool {
            rune_type: RuneType::GovernanceRune,
            total_staked: 0.0,
            reward_rate: 0.08, // 8% annual reward for governance
            participants: HashMap::new(),
            last_distribution: Utc::now(),
        });

        rune_pools.insert(RuneType::SecurityRune, RunePool {
            rune_type: RuneType::SecurityRune,
            total_staked: 0.0,
            reward_rate: 0.12, // 12% annual reward for security
            participants: HashMap::new(),
            last_distribution: Utc::now(),
        });

        let reward_distribution = RewardDistributionEngine {
            distribution_schedule: Self::create_distribution_schedules(),
            pending_rewards: HashMap::new(),
            reward_history: Vec::new(),
            total_distributed: 0.0,
        };

        let economic_coordinator = Arc::new(RuneEconomicCoordinator::new().await?);

        Ok(Self {
            rune_pools: Arc::new(RwLock::new(rune_pools)),
            staking_contracts: Arc::new(RwLock::new(HashMap::new())),
            reward_distribution: Arc::new(RwLock::new(reward_distribution)),
            governance_tokens: Arc::new(RwLock::new(HashMap::new())),
            economic_coordinator,
        })
    }

    /// Process domain staking for registration
    pub async fn process_domain_staking(
        &self,
        domain_request: &DomainRegistrationRequest,
        pricing: &DomainPricing,
    ) -> Result<StakingResult> {
        info!("💰 Processing domain staking for: {}", domain_request.domain_name);

        // Calculate staking requirements
        let staking_amount = self.calculate_staking_amount(domain_request, pricing).await?;
        
        // Create staking contract
        let contract_id = Uuid::new_v4().to_string();
        let staking_contract = StakingContract {
            contract_id: contract_id.clone(),
            domain_name: domain_request.domain_name.clone(),
            staker_did: domain_request.owner_did.clone(),
            staked_amount: staking_amount,
            rune_type: RuneType::RegistrationRune,
            lock_period: Duration::days(365), // 1 year lock
            reward_multiplier: self.calculate_reward_multiplier(domain_request).await?,
            created_at: Utc::now(),
        };

        // Store staking contract
        {
            let mut contracts = self.staking_contracts.write().await;
            contracts.insert(contract_id.clone(), staking_contract);
        }

        // Update rune pool
        self.update_rune_pool(&RuneType::RegistrationRune, &domain_request.owner_did, staking_amount).await?;

        // Issue governance tokens
        let governance_tokens_earned = self.calculate_governance_tokens(staking_amount).await?;
        self.issue_governance_tokens(&domain_request.owner_did, governance_tokens_earned, &domain_request.domain_name).await?;

        Ok(StakingResult {
            contract_id,
            staked_amount: staking_amount,
            governance_tokens_issued: governance_tokens_earned as f64,
            estimated_annual_yield: 0.05, // 5% APY
            expected_rewards: staking_amount * 0.05, // 5% of staked amount
            governance_tokens_earned: governance_tokens_earned as f64,
            lock_period: Duration::days(365), // 1 year lock
        })
    }

    /// Distribute rewards to stakers
    pub async fn distribute_rewards(&self) -> Result<Vec<RewardDistribution>> {
        info!("🎁 Distributing rune rewards");

        let mut distributions = Vec::new();
        let mut reward_engine = self.reward_distribution.write().await;

        // Process each rune pool
        let rune_pools = self.rune_pools.read().await;
        for (rune_type, pool) in rune_pools.iter() {
            for (participant_did, staked_amount) in &pool.participants {
                let reward_amount = staked_amount * pool.reward_rate / 365.0; // Daily reward
                
                let distribution = RewardDistribution {
                    distribution_id: Uuid::new_v4().to_string(),
                    rune_type: rune_type.clone(),
                    recipient_did: participant_did.clone(),
                    amount: reward_amount,
                    reason: RewardReason::StakingReward,
                    distributed_at: Utc::now(),
                };

                distributions.push(distribution.clone());
                reward_engine.reward_history.push(distribution);
                reward_engine.total_distributed += reward_amount;
            }
        }

        info!("✅ Distributed {} rewards", distributions.len());
        Ok(distributions)
    }



    /// Get rune statistics for monitoring
    pub async fn get_rune_statistics(&self) -> Result<RuneStatistics> {
        let rune_pools = self.rune_pools.read().await;
        let contracts = self.staking_contracts.read().await;
        let reward_engine = self.reward_distribution.read().await;
        let governance_tokens = self.governance_tokens.read().await;

        let total_staked: f64 = rune_pools.values().map(|pool| pool.total_staked).sum();
        let economic_metrics = self.economic_coordinator.get_economic_metrics().await?;

        Ok(RuneStatistics {
            total_staked,
            active_contracts: contracts.len() as u64,
            total_rewards_distributed: reward_engine.total_distributed,
            governance_tokens_issued: governance_tokens.len() as u64,
            economic_health: economic_metrics.economic_health_score,
        })
    }

    /// Private helper methods
    async fn calculate_staking_amount(
        &self,
        domain_request: &DomainRegistrationRequest,
        pricing: &DomainPricing,
    ) -> Result<f64> {
        // Base staking is 10x the annual cost
        let base_staking = pricing.annual_cost * 10.0;
        
        // Apply multipliers based on domain type and security requirements
        let multiplier = match domain_request.domain_type {
            crate::httpcg_domain_registry::DomainType::Global => 2.0,
            crate::httpcg_domain_registry::DomainType::Government => 1.5,
            crate::httpcg_domain_registry::DomainType::International => 1.8,
            crate::httpcg_domain_registry::DomainType::Country => 1.2,
        };

        Ok(base_staking * multiplier)
    }

    async fn calculate_reward_multiplier(&self, domain_request: &DomainRegistrationRequest) -> Result<f64> {
        // Higher multipliers for more valuable domain types
        let multiplier = match domain_request.domain_type {
            crate::httpcg_domain_registry::DomainType::Global => 1.5,
            crate::httpcg_domain_registry::DomainType::Government => 1.3,
            crate::httpcg_domain_registry::DomainType::International => 1.4,
            crate::httpcg_domain_registry::DomainType::Country => 1.1,
        };

        Ok(multiplier)
    }

    async fn calculate_governance_tokens(&self, staking_amount: f64) -> Result<u64> {
        // 1 governance token per 100 units staked
        Ok((staking_amount / 100.0) as u64)
    }

    async fn issue_governance_tokens(
        &self,
        holder_did: &str,
        amount: u64,
        domain_name: &str,
    ) -> Result<()> {
        let mut tokens = self.governance_tokens.write().await;
        
        for _ in 0..amount {
            let token = GovernanceToken {
                token_id: Uuid::new_v4().to_string(),
                holder_did: holder_did.to_string(),
                voting_power: 1.0,
                domain_associations: vec![domain_name.to_string()],
                earned_from: TokenEarnSource::DomainRegistration,
                created_at: Utc::now(),
                last_used: None,
            };
            
            tokens.insert(token.token_id.clone(), token);
        }

        Ok(())
    }

    async fn update_rune_pool(
        &self,
        rune_type: &RuneType,
        participant_did: &str,
        amount: f64,
    ) -> Result<()> {
        let mut pools = self.rune_pools.write().await;
        
        if let Some(pool) = pools.get_mut(rune_type) {
            pool.total_staked += amount;
            *pool.participants.entry(participant_did.to_string()).or_insert(0.0) += amount;
        }

        Ok(())
    }

    async fn calculate_expected_rewards(&self, staking_amount: f64, rune_type: &RuneType) -> Result<f64> {
        let pools = self.rune_pools.read().await;
        
        if let Some(pool) = pools.get(rune_type) {
            Ok(staking_amount * pool.reward_rate)
        } else {
            Ok(0.0)
        }
    }

    fn create_distribution_schedules() -> HashMap<RuneType, DistributionSchedule> {
        let mut schedules = HashMap::new();
        
        schedules.insert(RuneType::RegistrationRune, DistributionSchedule {
            rune_type: RuneType::RegistrationRune,
            base_reward_rate: 0.05,
            distribution_frequency: Duration::days(1),
            multiplier_factors: HashMap::new(),
            max_rewards_per_period: 1000.0,
        });

        schedules.insert(RuneType::GovernanceRune, DistributionSchedule {
            rune_type: RuneType::GovernanceRune,
            base_reward_rate: 0.08,
            distribution_frequency: Duration::days(1),
            multiplier_factors: HashMap::new(),
            max_rewards_per_period: 500.0,
        });

        schedules
    }
}

impl RuneEconomicCoordinator {
    /// Create new Rune Economic Coordinator
    pub async fn new() -> Result<Self> {
        let coin_integration = Arc::new(CoinIntegration {
            gen_allocation: 0.25, // 25% GEN for governance
            nex_allocation: 0.30, // 30% NEX for network
            flx_allocation: 0.25, // 25% FLX for flexibility
            aur_allocation: 0.20, // 20% AUR for premium
        });

        let treasury_bridge = Arc::new(TreasuryBridge {
            treasury_endpoint: "http://localhost:8081/api/treasury".to_string(),
            allocation_ratios: HashMap::new(),
            reserve_requirements: HashMap::new(),
        });

        let market_dynamics = MarketDynamics {
            supply_demand_ratio: HashMap::new(),
            price_volatility: HashMap::new(),
            staking_pressure: 0.0,
            governance_activity: 0.0,
            last_updated: Utc::now(),
        };

        let economic_metrics = RuneEconomicMetrics {
            total_value_locked: 0.0,
            active_stakers: 0,
            governance_participation: 0.0,
            reward_distribution_rate: 0.0,
            economic_health_score: 100.0,
        };

        Ok(Self {
            coin_integration,
            treasury_bridge,
            market_dynamics: Arc::new(RwLock::new(market_dynamics)),
            economic_metrics: Arc::new(RwLock::new(economic_metrics)),
        })
    }

    /// Get economic metrics
    pub async fn get_economic_metrics(&self) -> Result<RuneEconomicMetrics> {
        let metrics = self.economic_metrics.read().await;
        Ok(metrics.clone())
    }
}


