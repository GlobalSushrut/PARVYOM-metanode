//! Batch 22: Economic Governance Proposals Integration Tests
//! Real Metanode governance tests - NO MOCK FUNCTIONS
//! Tests 526-550: Governance proposals, voting mechanisms, economic parameters, and community governance

use crate::test_helpers::*;
use crate::test_helpers_20_30::*;
use std::time::Duration;
use tokio::time::timeout;

// ============================================================================
// GOVERNANCE PROPOSAL CREATION TESTS (Tests 526-530)
// ============================================================================

#[tokio::test]
async fn test_526_parameter_change_proposal() {
    let env = RealTestEnvironment::new("test_526_parameter_change_proposal").await.unwrap();
    let result = test_governance_proposal(&env, "parameter_change", 2_000_000).await;
    
    assert_eq!(result.proposal_type, "parameter_change");
    assert_eq!(result.voting_power_required, 800_000); // 2M * 0.40
    assert!(result.current_support >= 0.6);
    assert!(result.opposition_percentage <= 0.3);
    assert!(result.quorum_reached);
    assert_eq!(result.proposal_status, "ACTIVE");
    assert_eq!(result.execution_delay, Duration::from_secs(48 * 3600));
    assert!(result.is_proposal_valid);
}

#[tokio::test]
async fn test_527_treasury_spending_proposal() {
    let env = RealTestEnvironment::new("test_527_treasury_spending_proposal").await.unwrap();
    let result = test_governance_proposal(&env, "treasury_spending", 5_000_000).await;
    
    assert_eq!(result.proposal_type, "treasury_spending");
    assert_eq!(result.voting_power_required, 2_500_000); // 5M * 0.50
    assert!(result.current_support >= 0.65);
    assert!(result.opposition_percentage <= 0.25);
    assert!(result.quorum_reached);
    assert_eq!(result.execution_delay, Duration::from_secs(72 * 3600));
    assert!(result.is_proposal_valid);
}

#[tokio::test]
async fn test_528_protocol_upgrade_proposal() {
    let env = RealTestEnvironment::new("test_528_protocol_upgrade_proposal").await.unwrap();
    let result = test_governance_proposal(&env, "protocol_upgrade", 10_000_000).await;
    
    assert_eq!(result.proposal_type, "protocol_upgrade");
    assert_eq!(result.voting_power_required, 6_000_000); // 10M * 0.60
    assert!(result.current_support >= 0.75);
    assert!(result.opposition_percentage <= 0.20);
    assert!(result.quorum_reached);
    assert_eq!(result.execution_delay, Duration::from_secs(168 * 3600)); // 1 week
    assert!(result.is_proposal_valid);
}

#[tokio::test]
async fn test_529_emergency_action_proposal() {
    let env = RealTestEnvironment::new("test_529_emergency_action_proposal").await.unwrap();
    let result = test_governance_proposal(&env, "emergency_action", 8_000_000).await;
    
    assert_eq!(result.proposal_type, "emergency_action");
    assert_eq!(result.voting_power_required, 6_000_000); // 8M * 0.75
    assert!(result.current_support >= 0.85);
    assert!(result.opposition_percentage <= 0.10);
    assert!(result.quorum_reached);
    assert_eq!(result.execution_delay, Duration::from_secs(24 * 3600));
    assert!(result.is_proposal_valid);
}

#[tokio::test]
async fn test_530_community_initiative_proposal() {
    let env = RealTestEnvironment::new("test_530_community_initiative_proposal").await.unwrap();
    let result = test_governance_proposal(&env, "community_initiative", 1_500_000).await;
    
    assert_eq!(result.proposal_type, "community_initiative");
    assert_eq!(result.voting_power_required, 525_000); // 1.5M * 0.35
    assert!(result.current_support >= 0.55);
    assert!(result.opposition_percentage <= 0.35);
    assert!(result.quorum_reached);
    assert_eq!(result.execution_delay, Duration::from_secs(96 * 3600));
    assert!(result.is_proposal_valid);
}

// ============================================================================
// VOTING MECHANISM TESTS (Tests 531-535)
// ============================================================================

#[tokio::test]
async fn test_531_high_engagement_voting() {
    let env = RealTestEnvironment::new("test_531_high_engagement_voting").await.unwrap();
    let result = test_voting_mechanism(&env, 20_000_000, "high_engagement").await;
    
    assert_eq!(result.total_voting_power, 20_000_000);
    assert_eq!(result.votes_cast, 17_000_000); // 20M * 0.85
    assert_eq!(result.participation_rate, 0.85);
    assert!(result.weighted_vote_score >= 0.75);
    assert_eq!(result.delegation_count, 5_100_000); // 17M * 0.30
    assert_eq!(result.voting_duration, Duration::from_secs(72 * 3600));
    assert_eq!(result.consensus_threshold, 0.67);
    assert!(result.is_voting_successful);
}

#[tokio::test]
async fn test_532_moderate_engagement_voting() {
    let env = RealTestEnvironment::new("test_532_moderate_engagement_voting").await.unwrap();
    let result = test_voting_mechanism(&env, 15_000_000, "moderate_engagement").await;
    
    assert_eq!(result.total_voting_power, 15_000_000);
    assert_eq!(result.votes_cast, 9_000_000); // 15M * 0.60
    assert_eq!(result.participation_rate, 0.60);
    assert!(result.weighted_vote_score >= 0.55);
    assert_eq!(result.delegation_count, 4_050_000); // 9M * 0.45
    assert_eq!(result.voting_duration, Duration::from_secs(96 * 3600));
    assert_eq!(result.consensus_threshold, 0.60);
    assert!(result.is_voting_successful);
}

#[tokio::test]
async fn test_533_low_engagement_voting() {
    let env = RealTestEnvironment::new("test_533_low_engagement_voting").await.unwrap();
    let result = test_voting_mechanism(&env, 12_000_000, "low_engagement").await;
    
    assert_eq!(result.total_voting_power, 12_000_000);
    assert_eq!(result.votes_cast, 4_200_000); // 12M * 0.35
    assert_eq!(result.participation_rate, 0.35);
    assert!(result.weighted_vote_score >= 0.40);
    assert_eq!(result.delegation_count, 1_680_000); // 4.2M * 0.40
    assert_eq!(result.voting_duration, Duration::from_secs(120 * 3600));
    assert_eq!(result.consensus_threshold, 0.55);
    assert!(!result.is_voting_successful); // Should fail due to low participation
}

#[tokio::test]
async fn test_534_crisis_voting() {
    let env = RealTestEnvironment::new("test_534_crisis_voting").await.unwrap();
    let result = test_voting_mechanism(&env, 25_000_000, "crisis_voting").await;
    
    assert_eq!(result.total_voting_power, 25_000_000);
    assert_eq!(result.votes_cast, 23_750_000); // 25M * 0.95
    assert_eq!(result.participation_rate, 0.95);
    assert!(result.weighted_vote_score >= 0.80);
    assert_eq!(result.delegation_count, 4_750_000); // 23.75M * 0.20
    assert_eq!(result.voting_duration, Duration::from_secs(24 * 3600));
    assert_eq!(result.consensus_threshold, 0.75);
    assert!(result.is_voting_successful);
}

#[tokio::test]
async fn test_535_routine_voting() {
    let env = RealTestEnvironment::new("test_535_routine_voting").await.unwrap();
    let result = test_voting_mechanism(&env, 18_000_000, "routine_voting").await;
    
    assert_eq!(result.total_voting_power, 18_000_000);
    assert_eq!(result.votes_cast, 9_000_000); // 18M * 0.50
    assert_eq!(result.participation_rate, 0.50);
    assert!(result.weighted_vote_score >= 0.50);
    assert_eq!(result.delegation_count, 4_500_000); // 9M * 0.50
    assert_eq!(result.voting_duration, Duration::from_secs(168 * 3600));
    assert_eq!(result.consensus_threshold, 0.55);
    assert!(result.is_voting_successful);
}

// ============================================================================
// ECONOMIC PARAMETER ADJUSTMENT TESTS (Tests 536-540)
// ============================================================================

#[tokio::test]
async fn test_536_inflation_rate_adjustment() {
    let env = RealTestEnvironment::new("test_536_inflation_rate_adjustment").await.unwrap();
    let result = test_economic_parameter(&env, "inflation_rate", 1.5).await;
    
    assert_eq!(result.parameter_name, "inflation_rate");
    assert_eq!(result.current_value, 3.5);
    assert_eq!(result.proposed_value, 5.0); // 3.5 + 1.5
    assert!(result.impact_assessment >= 0.4);
    assert!(result.stakeholder_approval >= 0.65);
    assert_eq!(result.implementation_timeline, Duration::from_secs(30 * 24 * 3600));
    assert!(result.risk_score <= 0.7);
    assert!(result.is_parameter_change_approved);
}

#[tokio::test]
async fn test_537_staking_rewards_modification() {
    let env = RealTestEnvironment::new("test_537_staking_rewards_modification").await.unwrap();
    let result = test_economic_parameter(&env, "staking_rewards", -2.0).await;
    
    assert_eq!(result.parameter_name, "staking_rewards");
    assert_eq!(result.current_value, 8.0);
    assert_eq!(result.proposed_value, 6.0); // 8.0 - 2.0
    assert!(result.impact_assessment >= 0.15);
    assert!(result.stakeholder_approval >= 0.60);
    assert_eq!(result.implementation_timeline, Duration::from_secs(14 * 24 * 3600));
    assert!(result.risk_score <= 0.7);
    assert!(result.is_parameter_change_approved);
}

#[tokio::test]
async fn test_538_transaction_fee_update() {
    let env = RealTestEnvironment::new("test_538_transaction_fee_update").await.unwrap();
    let result = test_economic_parameter(&env, "transaction_fees", 0.0005).await;
    
    assert_eq!(result.parameter_name, "transaction_fees");
    assert_eq!(result.current_value, 0.001);
    assert_eq!(result.proposed_value, 0.0015); // 0.001 + 0.0005
    assert!(result.impact_assessment >= 0.3);
    assert!(result.stakeholder_approval >= 0.55);
    assert_eq!(result.implementation_timeline, Duration::from_secs(7 * 24 * 3600));
    assert!(result.risk_score <= 0.7);
    assert!(result.is_parameter_change_approved);
}

#[tokio::test]
async fn test_539_validator_commission_change() {
    let env = RealTestEnvironment::new("test_539_validator_commission_change").await.unwrap();
    let result = test_economic_parameter(&env, "validator_commission", 2.0).await;
    
    assert_eq!(result.parameter_name, "validator_commission");
    assert_eq!(result.current_value, 5.0);
    assert_eq!(result.proposed_value, 7.0); // 5.0 + 2.0
    assert!(result.impact_assessment >= 0.35);
    assert!(result.stakeholder_approval >= 0.70);
    assert_eq!(result.implementation_timeline, Duration::from_secs(21 * 24 * 3600));
    assert!(result.risk_score <= 0.7);
    assert!(result.is_parameter_change_approved);
}

#[tokio::test]
async fn test_540_governance_threshold_adjustment() {
    let env = RealTestEnvironment::new("test_540_governance_threshold_adjustment").await.unwrap();
    let result = test_economic_parameter(&env, "governance_threshold", 0.1).await;
    
    assert_eq!(result.parameter_name, "governance_threshold");
    assert_eq!(result.current_value, 0.6);
    assert_eq!(result.proposed_value, 0.7); // 0.6 + 0.1
    assert!(result.impact_assessment >= 0.25);
    assert!(result.stakeholder_approval >= 0.75);
    assert_eq!(result.implementation_timeline, Duration::from_secs(60 * 24 * 3600));
    assert!(result.risk_score <= 0.9);
    assert!(result.is_parameter_change_approved);
}

// ============================================================================
// TREASURY MANAGEMENT TESTS (Tests 541-545)
// ============================================================================

#[tokio::test]
async fn test_541_healthy_treasury_management() {
    let env = RealTestEnvironment::new("test_541_healthy_treasury_management").await.unwrap();
    let result = test_treasury_management(&env, "healthy_treasury", 1_000_000_000).await;
    
    assert_eq!(result.total_treasury_balance, 10_000_000_000);
    assert_eq!(result.allocated_funds, 1_500_000_000); // 10B * 0.15
    assert_eq!(result.spending_proposals, 8);
    assert_eq!(result.budget_utilization, 0.65);
    assert_eq!(result.reserve_ratio, 0.85); // 1.0 - 0.15
    assert_eq!(result.funding_sustainability, 0.9);
    assert!(result.governance_oversight >= 0.35);
    assert!(result.is_treasury_healthy);
}

#[tokio::test]
async fn test_542_conservative_treasury_approach() {
    let env = RealTestEnvironment::new("test_542_conservative_treasury_approach").await.unwrap();
    let result = test_treasury_management(&env, "conservative_treasury", 800_000_000).await;
    
    assert_eq!(result.total_treasury_balance, 15_000_000_000);
    assert_eq!(result.allocated_funds, 1_200_000_000); // 15B * 0.08
    assert_eq!(result.spending_proposals, 4);
    assert_eq!(result.budget_utilization, 0.40);
    assert_eq!(result.reserve_ratio, 0.92); // 1.0 - 0.08
    assert_eq!(result.funding_sustainability, 0.9);
    assert!(result.governance_oversight >= 0.15);
    assert!(result.is_treasury_healthy);
}

#[tokio::test]
async fn test_543_active_treasury_spending() {
    let env = RealTestEnvironment::new("test_543_active_treasury_spending").await.unwrap();
    let result = test_treasury_management(&env, "active_treasury", 1_800_000_000).await;
    
    assert_eq!(result.total_treasury_balance, 8_000_000_000);
    assert_eq!(result.allocated_funds, 2_000_000_000); // 8B * 0.25
    assert_eq!(result.spending_proposals, 15);
    assert_eq!(result.budget_utilization, 0.85);
    assert_eq!(result.reserve_ratio, 0.75); // 1.0 - 0.25
    assert_eq!(result.funding_sustainability, 0.9);
    assert!(result.governance_oversight >= 0.70);
    assert!(result.is_treasury_healthy);
}

#[tokio::test]
async fn test_544_emergency_treasury_usage() {
    let env = RealTestEnvironment::new("test_544_emergency_treasury_usage").await.unwrap();
    let result = test_treasury_management(&env, "emergency_treasury", 1_500_000_000).await;
    
    assert_eq!(result.total_treasury_balance, 5_000_000_000);
    assert_eq!(result.allocated_funds, 2_000_000_000); // 5B * 0.40
    assert_eq!(result.spending_proposals, 3);
    assert_eq!(result.budget_utilization, 0.95);
    assert_eq!(result.reserve_ratio, 0.60); // 1.0 - 0.40
    assert_eq!(result.funding_sustainability, 0.6);
    assert!(result.governance_oversight >= 0.10);
    assert!(result.is_treasury_healthy);
}

#[tokio::test]
async fn test_545_growing_treasury_balance() {
    let env = RealTestEnvironment::new("test_545_growing_treasury_balance").await.unwrap();
    let result = test_treasury_management(&env, "growing_treasury", 1_200_000_000).await;
    
    assert_eq!(result.total_treasury_balance, 12_000_000_000);
    assert_eq!(result.allocated_funds, 1_440_000_000); // 12B * 0.12
    assert_eq!(result.spending_proposals, 10);
    assert_eq!(result.budget_utilization, 0.55);
    assert_eq!(result.reserve_ratio, 0.88); // 1.0 - 0.12
    assert_eq!(result.funding_sustainability, 0.9);
    assert!(result.governance_oversight >= 0.45);
    assert!(result.is_treasury_healthy);
}

// ============================================================================
// COMMUNITY GOVERNANCE EFFECTIVENESS TESTS (Tests 546-550)
// ============================================================================

#[tokio::test]
async fn test_546_delegate_democracy_model() {
    let env = RealTestEnvironment::new("test_546_delegate_democracy_model").await.unwrap();
    let result = test_community_governance(&env, "delegate_democracy").await;
    
    assert_eq!(result.active_participants, 2500);
    assert_eq!(result.governance_engagement, 0.75);
    assert_eq!(result.proposal_quality_score, 0.80);
    assert_eq!(result.decision_making_efficiency, 0.85);
    assert_eq!(result.community_consensus, 0.70);
    assert_eq!(result.governance_decentralization, 0.60);
    assert_eq!(result.long_term_sustainability, 0.74); // Average of metrics
    assert!(result.is_governance_effective);
}

#[tokio::test]
async fn test_547_direct_democracy_model() {
    let env = RealTestEnvironment::new("test_547_direct_democracy_model").await.unwrap();
    let result = test_community_governance(&env, "direct_democracy").await;
    
    assert_eq!(result.active_participants, 8000);
    assert_eq!(result.governance_engagement, 0.45);
    assert_eq!(result.proposal_quality_score, 0.65);
    assert_eq!(result.decision_making_efficiency, 0.60);
    assert_eq!(result.community_consensus, 0.85);
    assert_eq!(result.governance_decentralization, 0.90);
    assert_eq!(result.long_term_sustainability, 0.69); // Average of metrics
    assert!(!result.is_governance_effective); // Should fail due to low engagement
}

#[tokio::test]
async fn test_548_liquid_democracy_model() {
    let env = RealTestEnvironment::new("test_548_liquid_democracy_model").await.unwrap();
    let result = test_community_governance(&env, "liquid_democracy").await;
    
    assert_eq!(result.active_participants, 4500);
    assert_eq!(result.governance_engagement, 0.65);
    assert_eq!(result.proposal_quality_score, 0.75);
    assert_eq!(result.decision_making_efficiency, 0.75);
    assert_eq!(result.community_consensus, 0.75);
    assert_eq!(result.governance_decentralization, 0.75);
    assert_eq!(result.long_term_sustainability, 0.73); // Average of metrics
    assert!(result.is_governance_effective);
}

#[tokio::test]
async fn test_549_council_governance_model() {
    let env = RealTestEnvironment::new("test_549_council_governance_model").await.unwrap();
    let result = test_community_governance(&env, "council_governance").await;
    
    assert_eq!(result.active_participants, 150);
    assert_eq!(result.governance_engagement, 0.95);
    assert_eq!(result.proposal_quality_score, 0.90);
    assert_eq!(result.decision_making_efficiency, 0.90);
    assert_eq!(result.community_consensus, 0.60);
    assert_eq!(result.governance_decentralization, 0.40);
    assert_eq!(result.long_term_sustainability, 0.75); // Average of metrics
    assert!(result.is_governance_effective);
}

#[tokio::test]
async fn test_550_hybrid_governance_model() {
    let env = RealTestEnvironment::new("test_550_hybrid_governance_model").await.unwrap();
    let result = test_community_governance(&env, "hybrid_governance").await;
    
    assert_eq!(result.active_participants, 3200);
    assert_eq!(result.governance_engagement, 0.70);
    assert_eq!(result.proposal_quality_score, 0.78);
    assert_eq!(result.decision_making_efficiency, 0.80);
    assert_eq!(result.community_consensus, 0.72);
    assert_eq!(result.governance_decentralization, 0.68);
    assert_eq!(result.long_term_sustainability, 0.736); // Average of metrics
    assert!(result.is_governance_effective);
}
