use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use thiserror::Error;
use uuid::Uuid;

use crate::autonomous_economy::CoinType;
use crate::registry::node_types::{BpiWalletStamp, NodeType};

/// Mother Coin (GEN) Distribution System
/// Target: Raise $1M safely while maintaining decentralization
/// Total: 100,000 GEN coins with community-first allocation

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotherCoinDistribution {
    /// Total GEN supply (100,000)
    pub total_supply: u64,
    /// Current distribution state
    pub distribution_tiers: HashMap<DistributionTier, TierState>,
    /// Fundraising progress
    pub fundraising: FundraisingProgress,
    /// Decentralization metrics
    pub decentralization_metrics: DecentralizationMetrics,
    /// Network registries
    pub testnet_registry: NetworkRegistry,
    pub mainnet_registry: NetworkRegistry,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum DistributionTier {
    /// Phase 1: 25k GEN for community installers ($10/coin = $250k)
    CommunityInstallers,
    /// Phase 2: 25k GEN for early investors ($30/coin = $750k)
    EarlyInvestors,
    /// Phase 3: 15k GEN post-mainnet ($20/coin = $300k)
    PostMainnet,
    /// Phase 4: 25k GEN marketplace ($40/coin = $1M)
    Marketplace,
    /// Phase 5: 10k GEN governance team (allocated, not sold)
    GovernanceTeam,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierState {
    /// Total allocation for this tier
    pub total_allocation: u64,
    /// Already distributed
    pub distributed: u64,
    /// Price per GEN coin (USD)
    pub price_per_coin: Decimal,
    /// Target fundraising amount
    pub target_amount: Decimal,
    /// Current raised amount
    pub raised_amount: Decimal,
    /// Whether tier is active
    pub active: bool,
    /// Node set requirements per coin
    pub node_sets_required: u32,
    /// Decentralization requirements
    pub max_per_wallet: u64,
    /// Anti-whale protection
    pub whale_protection: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundraisingProgress {
    /// Total target ($1M)
    pub total_target: Decimal,
    /// Current raised amount
    pub total_raised: Decimal,
    /// Fundraising phases
    pub phases: Vec<FundraisingPhase>,
    /// Current active phase
    pub current_phase: usize,
    /// Safety mechanisms
    pub safety_mechanisms: SafetyMechanisms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundraisingPhase {
    /// Phase name
    pub name: String,
    /// Target amount for this phase
    pub target: Decimal,
    /// Raised in this phase
    pub raised: Decimal,
    /// GEN coins allocated
    pub gen_allocated: u64,
    /// Price per coin
    pub price: Decimal,
    /// Start date
    pub start_date: DateTime<Utc>,
    /// End date
    pub end_date: Option<DateTime<Utc>>,
    /// Status
    pub status: PhaseStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhaseStatus {
    Planned,
    Active,
    Completed,
    Paused,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyMechanisms {
    /// Maximum investment per wallet (anti-whale)
    pub max_investment_per_wallet: Decimal,
    /// Minimum community participation required
    pub min_community_participation: f64,
    /// Escrow protection
    pub escrow_enabled: bool,
    /// Refund policy
    pub refund_policy: RefundPolicy,
    /// KYC requirements
    pub kyc_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundPolicy {
    /// Refund window (days)
    pub refund_window_days: u32,
    /// Conditions for refund
    pub conditions: Vec<String>,
    /// Partial refund allowed
    pub partial_refund: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecentralizationMetrics {
    /// Gini coefficient (0 = perfect equality, 1 = maximum inequality)
    pub gini_coefficient: f64,
    /// Number of unique holders
    pub unique_holders: u64,
    /// Largest holder percentage
    pub largest_holder_percentage: f64,
    /// Top 10 holders percentage
    pub top_10_percentage: f64,
    /// Community vs investor ratio
    pub community_ratio: f64,
    /// Geographic distribution
    pub geographic_distribution: HashMap<String, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRegistry {
    /// Network type
    pub network_type: NetworkType,
    /// Registered participants
    pub participants: HashMap<String, Participant>,
    /// Node sets tracking
    pub node_sets: HashMap<String, NodeSetCluster>,
    /// GEN allocations
    pub gen_allocations: HashMap<String, GenAllocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkType {
    Testnet,
    Mainnet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    /// Wallet address
    pub wallet_address: String,
    /// Participant type
    pub participant_type: ParticipantType,
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
    /// KYC status
    pub kyc_status: KycStatus,
    /// Investment amount (USD)
    pub investment_amount: Decimal,
    /// GEN coins allocated
    pub gen_allocated: u64,
    /// Node sets owned
    pub node_sets: Vec<String>,
    /// Geographic location (ISO country code)
    pub country_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipantType {
    CommunityInstaller,
    EarlyInvestor,
    PostMainnetBuyer,
    MarketplaceBuyer,
    GovernanceTeam,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KycStatus {
    NotRequired,
    Pending,
    Verified,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSetCluster {
    /// Cluster ID
    pub id: String,
    /// Owner wallet
    pub owner: String,
    /// All 5 nodes in the set
    pub nodes: Vec<NodeInfo>,
    /// Cluster status
    pub status: ClusterStatus,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Performance metrics
    pub performance: NodePerformance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Node ID
    pub id: String,
    /// Node type
    pub node_type: NodeType,
    /// Endpoint URL
    pub endpoint: String,
    /// Status
    pub status: String,
    /// Performance score
    pub performance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClusterStatus {
    /// All 5 nodes operational and validated
    Complete,
    /// Missing nodes (count)
    Incomplete(u32),
    /// Failed validation
    Failed,
    /// Under maintenance
    Maintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePerformance {
    /// Uptime percentage
    pub uptime: f64,
    /// Response time (ms)
    pub avg_response_time: u64,
    /// Successful transactions
    pub successful_transactions: u64,
    /// Failed transactions
    pub failed_transactions: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenAllocation {
    /// Wallet address
    pub wallet: String,
    /// Number of GEN coins
    pub gen_amount: u64,
    /// Distribution tier
    pub tier: DistributionTier,
    /// Investment amount (USD)
    pub investment_amount: Decimal,
    /// Vesting schedule
    pub vesting: VestingSchedule,
    /// Allocated timestamp
    pub allocated_at: DateTime<Utc>,
    /// Governance weight
    pub governance_weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VestingSchedule {
    /// Total vesting period (months)
    pub total_months: u32,
    /// Cliff period (months)
    pub cliff_months: u32,
    /// Already vested amount
    pub vested_amount: u64,
    /// Next vesting date
    pub next_vesting_date: DateTime<Utc>,
}

/// Distribution system errors
#[derive(Error, Debug)]
pub enum DistributionError {
    #[error("Tier not active: {tier:?}")]
    TierNotActive { tier: DistributionTier },
    #[error("Insufficient allocation remaining: {remaining} < {requested}")]
    InsufficientAllocation { remaining: u64, requested: u64 },
    #[error("Investment exceeds maximum per wallet: ${amount} > ${max}")]
    ExceedsMaxInvestment { amount: Decimal, max: Decimal },
    #[error("Node set validation failed: {reason}")]
    NodeSetValidationFailed { reason: String },
    #[error("KYC verification required")]
    KycRequired,
    #[error("Decentralization threshold violated")]
    DecentralizationViolated,
    #[error("Fundraising target exceeded")]
    FundraisingTargetExceeded,
}

/// Mother Coin Distribution Engine
#[derive(Debug)]
pub struct MotherCoinDistributionEngine {
    /// Distribution state
    pub distribution: MotherCoinDistribution,
}

impl MotherCoinDistributionEngine {
    /// Create new distribution engine with safe $1M fundraising plan
    pub fn new() -> Self {
        let mut distribution_tiers = HashMap::new();
        
        // Phase 1: 25k GEN for community installers ($10/coin = $250k)
        distribution_tiers.insert(DistributionTier::CommunityInstallers, TierState {
            total_allocation: 25_000,
            distributed: 0,
            price_per_coin: Decimal::from(10), // $10/coin
            target_amount: Decimal::from(250_000), // $250k
            raised_amount: Decimal::ZERO,
            active: true,
            node_sets_required: 5, // 5 node sets per coin
            max_per_wallet: 100, // Max 100 GEN per community wallet
            whale_protection: true,
        });
        
        // Phase 2: 25k GEN for early investors ($30/coin = $750k)
        distribution_tiers.insert(DistributionTier::EarlyInvestors, TierState {
            total_allocation: 25_000,
            distributed: 0,
            price_per_coin: Decimal::from(30), // $30/coin
            target_amount: Decimal::from(750_000), // $750k
            raised_amount: Decimal::ZERO,
            active: false, // Activates after community phase
            node_sets_required: 5,
            max_per_wallet: 1_000, // Max 1k GEN per investor
            whale_protection: true,
        });
        
        // Phase 3: 15k GEN post-mainnet ($20/coin = $300k)
        distribution_tiers.insert(DistributionTier::PostMainnet, TierState {
            total_allocation: 15_000,
            distributed: 0,
            price_per_coin: Decimal::from(20),
            target_amount: Decimal::from(300_000),
            raised_amount: Decimal::ZERO,
            active: false,
            node_sets_required: 5,
            max_per_wallet: 500,
            whale_protection: true,
        });
        
        // Phase 4: 25k GEN marketplace ($40/coin = $1M)
        distribution_tiers.insert(DistributionTier::Marketplace, TierState {
            total_allocation: 25_000,
            distributed: 0,
            price_per_coin: Decimal::from(40),
            target_amount: Decimal::from(1_000_000),
            raised_amount: Decimal::ZERO,
            active: false,
            node_sets_required: 5,
            max_per_wallet: 2_000,
            whale_protection: true,
        });
        
        // Phase 5: 10k GEN governance team (allocated, not sold)
        distribution_tiers.insert(DistributionTier::GovernanceTeam, TierState {
            total_allocation: 10_000,
            distributed: 0,
            price_per_coin: Decimal::ZERO, // Not sold
            target_amount: Decimal::ZERO,
            raised_amount: Decimal::ZERO,
            active: false,
            node_sets_required: 0,
            max_per_wallet: 1_000,
            whale_protection: false,
        });

        let fundraising_phases = vec![
            FundraisingPhase {
                name: "Community Bootstrap".to_string(),
                target: Decimal::from(250_000),
                raised: Decimal::ZERO,
                gen_allocated: 25_000,
                price: Decimal::from(10),
                start_date: Utc::now(),
                end_date: None,
                status: PhaseStatus::Active,
            },
            FundraisingPhase {
                name: "Early Investment".to_string(),
                target: Decimal::from(750_000),
                raised: Decimal::ZERO,
                gen_allocated: 25_000,
                price: Decimal::from(30),
                start_date: Utc::now(),
                end_date: None,
                status: PhaseStatus::Planned,
            },
        ];

        let safety_mechanisms = SafetyMechanisms {
            max_investment_per_wallet: Decimal::from(50_000), // $50k max per wallet
            min_community_participation: 0.6, // 60% must be community
            escrow_enabled: true,
            refund_policy: RefundPolicy {
                refund_window_days: 30,
                conditions: vec![
                    "Mainnet launch delayed > 6 months".to_string(),
                    "Technical failure preventing node operation".to_string(),
                    "Regulatory compliance issues".to_string(),
                ],
                partial_refund: true,
            },
            kyc_required: true,
        };

        let fundraising = FundraisingProgress {
            total_target: Decimal::from(1_000_000), // $1M target
            total_raised: Decimal::ZERO,
            phases: fundraising_phases,
            current_phase: 0,
            safety_mechanisms,
        };

        let decentralization_metrics = DecentralizationMetrics {
            gini_coefficient: 0.0,
            unique_holders: 0,
            largest_holder_percentage: 0.0,
            top_10_percentage: 0.0,
            community_ratio: 1.0, // Start with 100% community focus
            geographic_distribution: HashMap::new(),
        };

        let distribution = MotherCoinDistribution {
            total_supply: 100_000,
            distribution_tiers,
            fundraising,
            decentralization_metrics,
            testnet_registry: NetworkRegistry {
                network_type: NetworkType::Testnet,
                participants: HashMap::new(),
                node_sets: HashMap::new(),
                gen_allocations: HashMap::new(),
            },
            mainnet_registry: NetworkRegistry {
                network_type: NetworkType::Mainnet,
                participants: HashMap::new(),
                node_sets: HashMap::new(),
                gen_allocations: HashMap::new(),
            },
        };

        Self { distribution }
    }

    /// Register community installer for GEN allocation
    pub fn register_community_installer(
        &mut self,
        wallet_address: String,
        country_code: String,
        node_set_proof: Vec<String>,
    ) -> Result<GenAllocation, DistributionError> {
        // Validate community tier is active and extract needed values
        let (tier_active, tier_price, remaining_allocation) = {
            let tier = self.distribution.distribution_tiers
                .get(&DistributionTier::CommunityInstallers)
                .ok_or(DistributionError::TierNotActive { 
                    tier: DistributionTier::CommunityInstallers 
                })?;

            if !tier.active {
                return Err(DistributionError::TierNotActive { 
                    tier: DistributionTier::CommunityInstallers 
                });
            }

            let remaining = tier.total_allocation - tier.distributed;
            if remaining == 0 {
                return Err(DistributionError::InsufficientAllocation {
                    remaining,
                    requested: 1,
                });
            }

            (tier.active, tier.price_per_coin, remaining)
        };

        // Validate node set (5 nodes required per GEN coin)
        if node_set_proof.len() != 5 {
            return Err(DistributionError::NodeSetValidationFailed {
                reason: format!("Expected 5 nodes, got {}", node_set_proof.len())
            });
        }

        // Create participant
        let participant = Participant {
            wallet_address: wallet_address.clone(),
            participant_type: ParticipantType::CommunityInstaller,
            registered_at: Utc::now(),
            kyc_status: KycStatus::NotRequired, // Community doesn't need KYC
            investment_amount: tier_price,
            gen_allocated: 1, // 1 GEN per installer
            node_sets: node_set_proof.clone(),
            country_code: country_code.clone(),
        };

        // Create GEN allocation
        let allocation = GenAllocation {
            wallet: wallet_address.clone(),
            gen_amount: 1,
            tier: DistributionTier::CommunityInstallers,
            investment_amount: tier_price,
            vesting: VestingSchedule {
                total_months: 12, // 1 year vesting
                cliff_months: 3,  // 3 month cliff
                vested_amount: 0,
                next_vesting_date: Utc::now() + chrono::Duration::days(90),
            },
            allocated_at: Utc::now(),
            governance_weight: 1.0, // Community gets full voting power
        };

        // Update state
        self.distribution.testnet_registry.participants.insert(wallet_address.clone(), participant);
        self.distribution.testnet_registry.gen_allocations.insert(wallet_address, allocation.clone());
        
        // Update tier state
        if let Some(tier_state) = self.distribution.distribution_tiers.get_mut(&DistributionTier::CommunityInstallers) {
            tier_state.distributed += 1;
            tier_state.raised_amount += tier_state.price_per_coin;
        }

        // Update fundraising progress
        self.distribution.fundraising.total_raised += tier_price;
        
        // Update decentralization metrics
        self.update_decentralization_metrics();

        Ok(allocation)
    }

    /// Register early investor for GEN allocation
    pub fn register_early_investor(
        &mut self,
        wallet_address: String,
        investment_amount: Decimal,
        country_code: String,
        kyc_proof: String,
    ) -> Result<GenAllocation, DistributionError> {
        // Validate early investor tier is active
        let tier = self.distribution.distribution_tiers
            .get(&DistributionTier::EarlyInvestors)
            .ok_or(DistributionError::TierNotActive { 
                tier: DistributionTier::EarlyInvestors 
            })?;

        if !tier.active {
            return Err(DistributionError::TierNotActive { 
                tier: DistributionTier::EarlyInvestors 
            });
        }

        // Validate investment amount
        if investment_amount > self.distribution.fundraising.safety_mechanisms.max_investment_per_wallet {
            return Err(DistributionError::ExceedsMaxInvestment {
                amount: investment_amount,
                max: self.distribution.fundraising.safety_mechanisms.max_investment_per_wallet,
            });
        }

        // Calculate GEN allocation
        let gen_amount = (investment_amount / tier.price_per_coin).to_u64().unwrap_or(0);
        
        // Check remaining allocation
        let remaining = tier.total_allocation - tier.distributed;
        if remaining < gen_amount {
            return Err(DistributionError::InsufficientAllocation {
                remaining,
                requested: gen_amount,
            });
        }

        // Create participant
        let participant = Participant {
            wallet_address: wallet_address.clone(),
            participant_type: ParticipantType::EarlyInvestor,
            registered_at: Utc::now(),
            kyc_status: KycStatus::Verified, // Investors need KYC
            investment_amount,
            gen_allocated: gen_amount,
            node_sets: vec![], // Investors don't need node sets initially
            country_code: country_code.clone(),
        };

        // Create GEN allocation with longer vesting for investors
        let allocation = GenAllocation {
            wallet: wallet_address.clone(),
            gen_amount,
            tier: DistributionTier::EarlyInvestors,
            investment_amount,
            vesting: VestingSchedule {
                total_months: 24, // 2 year vesting for investors
                cliff_months: 6,  // 6 month cliff
                vested_amount: 0,
                next_vesting_date: Utc::now() + chrono::Duration::days(180),
            },
            allocated_at: Utc::now(),
            governance_weight: 0.8, // Investors get slightly reduced voting power
        };

        // Update state
        self.distribution.mainnet_registry.participants.insert(wallet_address.clone(), participant);
        self.distribution.mainnet_registry.gen_allocations.insert(wallet_address, allocation.clone());
        
        // Update tier state
        if let Some(tier_state) = self.distribution.distribution_tiers.get_mut(&DistributionTier::EarlyInvestors) {
            tier_state.distributed += gen_amount;
            tier_state.raised_amount += investment_amount;
        }

        // Update fundraising progress
        self.distribution.fundraising.total_raised += investment_amount;
        
        // Update decentralization metrics
        self.update_decentralization_metrics();

        Ok(allocation)
    }

    /// Update decentralization metrics
    fn update_decentralization_metrics(&mut self) {
        let mut all_allocations = Vec::new();
        
        // Collect all allocations
        for allocation in self.distribution.testnet_registry.gen_allocations.values() {
            all_allocations.push(allocation.gen_amount);
        }
        for allocation in self.distribution.mainnet_registry.gen_allocations.values() {
            all_allocations.push(allocation.gen_amount);
        }

        if all_allocations.is_empty() {
            return;
        }

        // Sort for Gini calculation
        all_allocations.sort();
        let n = all_allocations.len() as f64;
        let total: u64 = all_allocations.iter().sum();
        
        // Calculate Gini coefficient
        let mut gini_sum = 0.0;
        for (i, &allocation) in all_allocations.iter().enumerate() {
            gini_sum += (2.0 * (i as f64 + 1.0) - n - 1.0) * allocation as f64;
        }
        let gini = gini_sum / (n * total as f64);

        // Calculate other metrics
        let largest_holder = *all_allocations.iter().max().unwrap_or(&0);
        let largest_percentage = (largest_holder as f64 / total as f64) * 100.0;
        
        // Top 10 holders
        all_allocations.sort_by(|a, b| b.cmp(a)); // Descending
        let top_10_sum: u64 = all_allocations.iter().take(10).sum();
        let top_10_percentage = (top_10_sum as f64 / total as f64) * 100.0;

        // Community ratio
        let community_count = self.distribution.testnet_registry.participants.len();
        let total_participants = community_count + self.distribution.mainnet_registry.participants.len();
        let community_ratio = if total_participants > 0 {
            community_count as f64 / total_participants as f64
        } else {
            1.0
        };

        // Update metrics
        self.distribution.decentralization_metrics = DecentralizationMetrics {
            gini_coefficient: gini,
            unique_holders: all_allocations.len() as u64,
            largest_holder_percentage: largest_percentage,
            top_10_percentage,
            community_ratio,
            geographic_distribution: HashMap::new(), // TODO: Calculate from participants
        };
    }

    /// Get distribution status
    pub fn get_distribution_status(&self) -> &MotherCoinDistribution {
        &self.distribution
    }

    /// Get fundraising progress
    pub fn get_fundraising_progress(&self) -> &FundraisingProgress {
        &self.distribution.fundraising
    }

    /// Get decentralization metrics
    pub fn get_decentralization_metrics(&self) -> &DecentralizationMetrics {
        &self.distribution.decentralization_metrics
    }

    /// Activate next fundraising phase
    pub fn activate_next_phase(&mut self) -> Result<(), DistributionError> {
        // Check if community phase is sufficiently complete
        let community_tier = self.distribution.distribution_tiers
            .get(&DistributionTier::CommunityInstallers)
            .unwrap();
        
        let community_completion = community_tier.distributed as f64 / community_tier.total_allocation as f64;
        
        if community_completion >= 0.8 { // 80% of community phase complete
            // Activate early investor phase
            if let Some(investor_tier) = self.distribution.distribution_tiers.get_mut(&DistributionTier::EarlyInvestors) {
                investor_tier.active = true;
            }
            
            // Update current phase
            self.distribution.fundraising.current_phase = 1;
            
            if let Some(phase) = self.distribution.fundraising.phases.get_mut(1) {
                phase.status = PhaseStatus::Active;
            }
        }

        Ok(())
    }
}

impl Default for MotherCoinDistributionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_community_installer_registration() {
        let mut engine = MotherCoinDistributionEngine::new();
        
        let result = engine.register_community_installer(
            "community_wallet_1".to_string(),
            "US".to_string(),
            vec!["node1".to_string(), "node2".to_string(), "node3".to_string(), "node4".to_string(), "node5".to_string()],
        );
        
        assert!(result.is_ok());
        let allocation = result.unwrap();
        assert_eq!(allocation.gen_amount, 1);
        assert_eq!(allocation.tier, DistributionTier::CommunityInstallers);
    }

    #[test]
    fn test_early_investor_registration() {
        let mut engine = MotherCoinDistributionEngine::new();
        
        // Activate investor phase
        if let Some(tier) = engine.distribution.distribution_tiers.get_mut(&DistributionTier::EarlyInvestors) {
            tier.active = true;
        }
        
        let result = engine.register_early_investor(
            "investor_wallet_1".to_string(),
            Decimal::from(30_000), // $30k investment = 1000 GEN
            "US".to_string(),
            "kyc_proof_123".to_string(),
        );
        
        assert!(result.is_ok());
        let allocation = result.unwrap();
        assert_eq!(allocation.gen_amount, 1000);
        assert_eq!(allocation.tier, DistributionTier::EarlyInvestors);
    }

    #[test]
    fn test_fundraising_target() {
        let engine = MotherCoinDistributionEngine::new();
        
        // Verify total fundraising target is $1M
        assert_eq!(engine.distribution.fundraising.total_target, Decimal::from(1_000_000));
        
        // Verify phase targets sum correctly
        let phase_total: Decimal = engine.distribution.fundraising.phases
            .iter()
            .map(|p| p.target)
            .sum();
        assert_eq!(phase_total, Decimal::from(1_000_000));
    }

    #[test]
    fn test_decentralization_protection() {
        let mut engine = MotherCoinDistributionEngine::new();
        
        // Activate investor phase
        if let Some(tier) = engine.distribution.distribution_tiers.get_mut(&DistributionTier::EarlyInvestors) {
            tier.active = true;
        }
        
        // Try to invest more than maximum allowed
        let result = engine.register_early_investor(
            "whale_wallet".to_string(),
            Decimal::from(100_000), // $100k > $50k max
            "US".to_string(),
            "kyc_proof_whale".to_string(),
        );
        
        assert!(result.is_err());
        match result.unwrap_err() {
            DistributionError::ExceedsMaxInvestment { .. } => {},
            _ => panic!("Expected ExceedsMaxInvestment error"),
        }
    }
}
