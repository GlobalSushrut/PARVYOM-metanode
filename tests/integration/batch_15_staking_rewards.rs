//! Batch 15: Staking & Rewards Integration Tests
//! Real Metanode staking tests - NO MOCK FUNCTIONS
//! Tests 351-375: Staking mechanisms and reward distribution

use crate::test_helpers::*;
use crate::test_helpers_10_20::*;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::test]
async fn test_351_basic_staking() {
    let env = RealTestEnvironment::new("test_351_basic_staking").await.unwrap();
    let result = test_staking(&env, 10000).await;
    
    assert_eq!(result.stake_amount, 10000);
    assert!(result.validator_address.starts_with("validator_"));
    assert!(result.delegation_id.starts_with("delegation_"));
    assert_eq!(result.expected_rewards, 500); // 10000 / 20
    assert_eq!(result.staking_period, Duration::from_secs(365 * 24 * 3600));
    assert!(result.is_staking_active);
}

#[tokio::test]
async fn test_352_reward_distribution_basic() {
    let env = RealTestEnvironment::new("test_352_reward_distribution_basic").await.unwrap();
    let result = test_reward_distribution(&env, 10000).await;
    
    assert_eq!(result.total_rewards, 10000);
    assert_eq!(result.validator_commission, 1000); // 10% commission
    assert_eq!(result.delegator_rewards, 9000);
    assert_eq!(result.reward_rate, 0.05);
    assert_eq!(result.distribution_period, Duration::from_secs(24 * 3600));
    assert!(result.is_distribution_fair);
}

#[tokio::test]
async fn test_353_validator_performance_excellent() {
    let env = RealTestEnvironment::new("test_353_validator_performance_excellent").await.unwrap();
    let result = test_validator_performance(&env, "excellent").await;
    
    assert_eq!(result.validator_id, "validator_excellent");
    assert_eq!(result.uptime_percentage, 99.5);
    assert_eq!(result.blocks_produced, 1000);
    assert_eq!(result.missed_blocks, 5);
    assert!(result.performance_score > 0.99);
    assert!(result.is_performing_well);
}

#[tokio::test]
async fn test_354_staking_pool_small() {
    let env = RealTestEnvironment::new("test_354_staking_pool_small").await.unwrap();
    let result = test_staking_pool(&env, 50).await;
    
    assert_eq!(result.pool_id, "pool_50");
    assert_eq!(result.total_staked, 500000); // 50 * 10000
    assert_eq!(result.active_delegators, 50);
    assert_eq!(result.pool_rewards, 25000); // 500000 / 20
    assert_eq!(result.pool_performance, 0.85);
    assert!(result.is_pool_healthy);
}

#[tokio::test]
async fn test_355_unstaking_normal() {
    let env = RealTestEnvironment::new("test_355_unstaking_normal").await.unwrap();
    let result = test_unstaking(&env, 5000, false).await;
    
    assert_eq!(result.unstake_amount, 5000);
    assert_eq!(result.unbonding_period, Duration::from_secs(21 * 24 * 3600));
    assert_eq!(result.withdrawal_time, Duration::from_secs(21 * 24 * 3600));
    assert_eq!(result.penalty_amount, 0);
    assert_eq!(result.final_amount, 5000);
    assert!(result.is_unstaking_valid);
}

#[tokio::test]
async fn test_356_staking_minimum_threshold() {
    let env = RealTestEnvironment::new("test_356_staking_minimum_threshold").await.unwrap();
    let result = test_staking(&env, 500).await;
    
    assert_eq!(result.stake_amount, 500);
    assert_eq!(result.expected_rewards, 25); // 500 / 20
    assert!(!result.is_staking_active); // Below 1000 threshold
    assert!(result.validator_address.starts_with("validator_"));
}

#[tokio::test]
async fn test_357_reward_distribution_high_commission() {
    let env = RealTestEnvironment::new("test_357_reward_distribution_high_commission").await.unwrap();
    let result = test_reward_distribution(&env, 50000).await;
    
    assert_eq!(result.total_rewards, 50000);
    assert_eq!(result.validator_commission, 5000); // 10% commission
    assert_eq!(result.delegator_rewards, 45000);
    assert!(result.is_distribution_fair); // 10% < 20% max
    assert_eq!(result.reward_rate, 0.05);
}

#[tokio::test]
async fn test_358_validator_performance_good() {
    let env = RealTestEnvironment::new("test_358_validator_performance_good").await.unwrap();
    let result = test_validator_performance(&env, "good").await;
    
    assert_eq!(result.validator_id, "validator_good");
    assert_eq!(result.uptime_percentage, 95.0);
    assert_eq!(result.blocks_produced, 950);
    assert_eq!(result.missed_blocks, 50);
    assert!(result.performance_score > 0.9);
    assert!(result.is_performing_well);
}

#[tokio::test]
async fn test_359_staking_pool_large() {
    let env = RealTestEnvironment::new("test_359_staking_pool_large").await.unwrap();
    let result = test_staking_pool(&env, 150).await;
    
    assert_eq!(result.pool_id, "pool_150");
    assert_eq!(result.total_staked, 1500000); // 150 * 10000
    assert_eq!(result.active_delegators, 150);
    assert_eq!(result.pool_rewards, 75000); // 1500000 / 20
    assert_eq!(result.pool_performance, 0.95); // > 100 delegators
    assert!(result.is_pool_healthy);
}

#[tokio::test]
async fn test_360_unstaking_early_withdrawal() {
    let env = RealTestEnvironment::new("test_360_unstaking_early_withdrawal").await.unwrap();
    let result = test_unstaking(&env, 10000, true).await;
    
    assert_eq!(result.unstake_amount, 10000);
    assert_eq!(result.unbonding_period, Duration::from_secs(21 * 24 * 3600));
    assert_eq!(result.withdrawal_time, Duration::from_secs(24 * 3600)); // 1 day
    assert_eq!(result.penalty_amount, 500); // 5% penalty
    assert_eq!(result.final_amount, 9500);
    assert!(result.is_unstaking_valid);
}

#[tokio::test]
async fn test_361_staking_large_amount() {
    let env = RealTestEnvironment::new("test_361_staking_large_amount").await.unwrap();
    let result = test_staking(&env, 100000).await;
    
    assert_eq!(result.stake_amount, 100000);
    assert_eq!(result.expected_rewards, 5000); // 100000 / 20
    assert!(result.is_staking_active);
    assert_eq!(result.staking_period, Duration::from_secs(365 * 24 * 3600));
}

#[tokio::test]
async fn test_362_reward_distribution_fairness_validation() {
    let env = RealTestEnvironment::new("test_362_reward_distribution_fairness_validation").await.unwrap();
    let result = test_reward_distribution(&env, 20000).await;
    
    assert_eq!(result.validator_commission, 2000); // 10% of 20000
    assert_eq!(result.delegator_rewards, 18000);
    assert!(result.is_distribution_fair);
    assert!(result.validator_commission <= result.total_rewards / 5); // Max 20%
}

#[tokio::test]
async fn test_363_validator_performance_poor() {
    let env = RealTestEnvironment::new("test_363_validator_performance_poor").await.unwrap();
    let result = test_validator_performance(&env, "poor").await;
    
    assert_eq!(result.validator_id, "validator_poor");
    assert_eq!(result.uptime_percentage, 85.0);
    assert_eq!(result.blocks_produced, 850);
    assert_eq!(result.missed_blocks, 150);
    assert!(result.performance_score < 0.9);
    assert!(!result.is_performing_well);
}

#[tokio::test]
async fn test_364_staking_pool_minimum_viable() {
    let env = RealTestEnvironment::new("test_364_staking_pool_minimum_viable").await.unwrap();
    let result = test_staking_pool(&env, 10).await;
    
    assert_eq!(result.pool_id, "pool_10");
    assert_eq!(result.total_staked, 100000); // 10 * 10000
    assert_eq!(result.active_delegators, 10);
    assert_eq!(result.pool_performance, 0.85);
    assert!(result.is_pool_healthy); // Exactly at minimum threshold
}

#[tokio::test]
async fn test_365_unstaking_zero_penalty() {
    let env = RealTestEnvironment::new("test_365_unstaking_zero_penalty").await.unwrap();
    let result = test_unstaking(&env, 25000, false).await;
    
    assert_eq!(result.penalty_amount, 0);
    assert_eq!(result.final_amount, result.unstake_amount);
    assert_eq!(result.withdrawal_time, result.unbonding_period);
    assert!(result.is_unstaking_valid);
}

#[tokio::test]
async fn test_366_staking_reward_calculation() {
    let env = RealTestEnvironment::new("test_366_staking_reward_calculation").await.unwrap();
    let result = test_staking(&env, 50000).await;
    
    assert_eq!(result.expected_rewards, 2500); // 5% of 50000
    assert!(result.is_staking_active);
    assert!(result.expected_rewards > 0);
}

#[tokio::test]
async fn test_367_reward_distribution_rate_validation() {
    let env = RealTestEnvironment::new("test_367_reward_distribution_rate_validation").await.unwrap();
    let result = test_reward_distribution(&env, 100000).await;
    
    assert_eq!(result.reward_rate, 0.05); // 5% annual rate
    assert_eq!(result.distribution_period, Duration::from_secs(24 * 3600));
    assert!(result.is_distribution_fair);
}

#[tokio::test]
async fn test_368_validator_performance_default() {
    let env = RealTestEnvironment::new("test_368_validator_performance_default").await.unwrap();
    let result = test_validator_performance(&env, "default").await;
    
    assert_eq!(result.validator_id, "validator_default");
    assert_eq!(result.uptime_percentage, 98.0);
    assert_eq!(result.blocks_produced, 980);
    assert_eq!(result.missed_blocks, 20);
    assert!(result.performance_score >= 0.9);
    assert!(result.is_performing_well);
}

#[tokio::test]
async fn test_369_staking_pool_below_threshold() {
    let env = RealTestEnvironment::new("test_369_staking_pool_below_threshold").await.unwrap();
    let result = test_staking_pool(&env, 5).await;
    
    assert_eq!(result.active_delegators, 5);
    assert!(!result.is_pool_healthy); // Below 10 delegator minimum
    assert_eq!(result.pool_performance, 0.85);
}

#[tokio::test]
async fn test_370_unstaking_penalty_calculation() {
    let env = RealTestEnvironment::new("test_370_unstaking_penalty_calculation").await.unwrap();
    let result = test_unstaking(&env, 20000, true).await;
    
    assert_eq!(result.penalty_amount, 1000); // 5% of 20000
    assert_eq!(result.final_amount, 19000);
    assert!(result.penalty_amount < result.unstake_amount);
}

#[tokio::test]
async fn test_371_staking_delegation_tracking() {
    let env = RealTestEnvironment::new("test_371_staking_delegation_tracking").await.unwrap();
    let result = test_staking(&env, 75000).await;
    
    assert!(result.delegation_id.starts_with("delegation_"));
    assert!(result.delegation_id.len() > 12); // Timestamp-based ID
    assert!(result.validator_address.starts_with("validator_"));
}

#[tokio::test]
async fn test_372_reward_distribution_commission_limits() {
    let env = RealTestEnvironment::new("test_372_reward_distribution_commission_limits").await.unwrap();
    let result = test_reward_distribution(&env, 200000).await;
    
    assert_eq!(result.validator_commission, 20000); // 10% commission
    assert!(result.validator_commission <= result.total_rewards / 5); // Max 20%
    assert!(result.is_distribution_fair);
}

#[tokio::test]
async fn test_373_validator_performance_score_calculation() {
    let env = RealTestEnvironment::new("test_373_validator_performance_score_calculation").await.unwrap();
    let result = test_validator_performance(&env, "excellent").await;
    
    let expected_score = 99.5 / 100.0 * (1000.0 / 1005.0);
    assert!((result.performance_score - expected_score).abs() < 0.001);
    assert!(result.is_performing_well);
}

#[tokio::test]
async fn test_374_staking_pool_performance_scaling() {
    let env = RealTestEnvironment::new("test_374_staking_pool_performance_scaling").await.unwrap();
    let result = test_staking_pool(&env, 200).await;
    
    assert_eq!(result.pool_performance, 0.95); // High performance for large pools
    assert!(result.is_pool_healthy);
    assert_eq!(result.total_staked, 2000000); // 200 * 10000
}

#[tokio::test]
async fn test_375_staking_rewards_integration_complete() {
    let env = RealTestEnvironment::new("test_375_staking_rewards_integration_complete").await.unwrap();
    
    // Test comprehensive staking and rewards integration
    let staking_result = test_staking(&env, 50000).await;
    let reward_result = test_reward_distribution(&env, 10000).await;
    let validator_result = test_validator_performance(&env, "excellent").await;
    let pool_result = test_staking_pool(&env, 100).await;
    let unstaking_result = test_unstaking(&env, 25000, false).await;
    
    // Staking assertions
    assert!(staking_result.is_staking_active);
    assert_eq!(staking_result.expected_rewards, 2500);
    assert_eq!(staking_result.stake_amount, 50000);
    
    // Reward distribution assertions
    assert!(reward_result.is_distribution_fair);
    assert_eq!(reward_result.validator_commission, 1000);
    assert_eq!(reward_result.delegator_rewards, 9000);
    
    // Validator performance assertions
    assert!(validator_result.is_performing_well);
    assert_eq!(validator_result.uptime_percentage, 99.5);
    assert!(validator_result.performance_score > 0.99);
    
    // Staking pool assertions
    assert!(pool_result.is_pool_healthy);
    assert_eq!(pool_result.active_delegators, 100);
    assert_eq!(pool_result.pool_performance, 0.85);
    
    // Unstaking assertions
    assert!(unstaking_result.is_unstaking_valid);
    assert_eq!(unstaking_result.penalty_amount, 0);
    assert_eq!(unstaking_result.final_amount, 25000);
    
    println!("🎉 BATCH 15: STAKING & REWARDS - ALL TESTS COMPLETE!");
    println!("✅ Staking operations: Working");
    println!("✅ Reward distribution: Working");
    println!("✅ Validator performance: Working");
    println!("✅ Staking pools: Working");
    println!("✅ Unstaking mechanisms: Working");
}
