//! Batch 21: Inflation/Deflation Mechanisms Integration Tests
//! Real Metanode inflation/deflation tests - NO MOCK FUNCTIONS
//! Tests 501-525: Inflation control, deflation mechanisms, monetary policy, and economic stability

use crate::test_helpers::*;
use crate::test_helpers_10_20::*;
use std::time::Duration;
use tokio::time::timeout;

// ============================================================================
// INFLATION MECHANISM CONTROL TESTS (Tests 501-505)
// ============================================================================

#[tokio::test]
async fn test_501_low_inflation_control() {
    let env = RealTestEnvironment::new("test_501_low_inflation_control").await.unwrap();
    let result = test_inflation_mechanism(&env, "low_inflation", 2.5).await;
    
    assert_eq!(result.current_inflation_rate, 2.0);
    assert_eq!(result.target_inflation_rate, 2.5);
    assert_eq!(result.inflation_adjustment, 0.4); // (2.5 - 2.0) * 0.8
    assert_eq!(result.supply_increase, 20_000_000); // 1B * 0.02
    assert!(result.validator_rewards_increase > 0);
    assert!(result.staking_rewards_increase > 0);
    assert!(result.is_inflation_controlled);
}

#[tokio::test]
async fn test_502_moderate_inflation_adjustment() {
    let env = RealTestEnvironment::new("test_502_moderate_inflation_adjustment").await.unwrap();
    let result = test_inflation_mechanism(&env, "moderate_inflation", 4.0).await;
    
    assert_eq!(result.current_inflation_rate, 5.0);
    assert_eq!(result.target_inflation_rate, 4.0);
    assert_eq!(result.inflation_adjustment, -1.0); // (4.0 - 5.0) * 1.0
    assert_eq!(result.supply_increase, 50_000_000); // 1B * 0.05
    assert!(result.validator_rewards_increase > result.staking_rewards_increase);
    assert!(result.is_inflation_controlled);
}

#[tokio::test]
async fn test_503_high_inflation_management() {
    let env = RealTestEnvironment::new("test_503_high_inflation_management").await.unwrap();
    let result = test_inflation_mechanism(&env, "high_inflation", 8.0).await;
    
    assert_eq!(result.current_inflation_rate, 10.0);
    assert_eq!(result.target_inflation_rate, 8.0);
    assert_eq!(result.inflation_adjustment, -3.0); // (8.0 - 10.0) * 1.5
    assert_eq!(result.supply_increase, 100_000_000); // 1B * 0.10
    assert!(result.validator_rewards_increase > 30_000_000);
    assert!(result.is_inflation_controlled);
}

#[tokio::test]
async fn test_504_hyperinflation_emergency_control() {
    let env = RealTestEnvironment::new("test_504_hyperinflation_emergency_control").await.unwrap();
    let result = test_inflation_mechanism(&env, "hyperinflation", 15.0).await;
    
    assert_eq!(result.current_inflation_rate, 25.0);
    assert_eq!(result.target_inflation_rate, 15.0);
    assert_eq!(result.inflation_adjustment, -20.0); // (15.0 - 25.0) * 2.0
    assert_eq!(result.supply_increase, 250_000_000); // 1B * 0.25
    assert!(!result.is_inflation_controlled); // Should fail control due to extreme values
}

#[tokio::test]
async fn test_505_controlled_inflation_precision() {
    let env = RealTestEnvironment::new("test_505_controlled_inflation_precision").await.unwrap();
    let result = test_inflation_mechanism(&env, "controlled_inflation", 3.5).await;
    
    assert_eq!(result.current_inflation_rate, 3.0);
    assert_eq!(result.target_inflation_rate, 3.5);
    assert_eq!(result.inflation_adjustment, 0.45); // (3.5 - 3.0) * 0.9
    assert_eq!(result.supply_increase, 30_000_000); // 1B * 0.03
    assert!(result.is_inflation_controlled);
}

// ============================================================================
// DEFLATION MECHANISM CONTROL TESTS (Tests 506-510)
// ============================================================================

#[tokio::test]
async fn test_506_mild_deflation_control() {
    let env = RealTestEnvironment::new("test_506_mild_deflation_control").await.unwrap();
    let result = test_deflation_mechanism(&env, "mild_deflation", 1.5).await;
    
    assert_eq!(result.current_deflation_rate, 1.0);
    assert_eq!(result.target_deflation_rate, 1.5);
    assert_eq!(result.deflation_adjustment, -0.4); // (1.0 - 1.5) * 0.8
    assert_eq!(result.tokens_burned, 10_000_000); // 1B * 0.01
    assert_eq!(result.supply_decrease, 10_000_000);
    assert_eq!(result.burn_mechanism, "transaction_fee_burn");
    assert!(result.is_deflation_controlled);
}

#[tokio::test]
async fn test_507_moderate_deflation_buyback() {
    let env = RealTestEnvironment::new("test_507_moderate_deflation_buyback").await.unwrap();
    let result = test_deflation_mechanism(&env, "moderate_deflation", 2.5).await;
    
    assert_eq!(result.current_deflation_rate, 3.0);
    assert_eq!(result.target_deflation_rate, 2.5);
    assert_eq!(result.deflation_adjustment, 0.5); // (3.0 - 2.5) * 1.0
    assert_eq!(result.tokens_burned, 30_000_000); // 1B * 0.03
    assert_eq!(result.burn_mechanism, "buyback_and_burn");
    assert!(result.is_deflation_controlled);
}

#[tokio::test]
async fn test_508_strong_deflation_penalties() {
    let env = RealTestEnvironment::new("test_508_strong_deflation_penalties").await.unwrap();
    let result = test_deflation_mechanism(&env, "strong_deflation", 5.0).await;
    
    assert_eq!(result.current_deflation_rate, 6.0);
    assert_eq!(result.target_deflation_rate, 5.0);
    assert_eq!(result.deflation_adjustment, 1.2); // (6.0 - 5.0) * 1.2
    assert_eq!(result.tokens_burned, 60_000_000); // 1B * 0.06
    assert_eq!(result.burn_mechanism, "staking_penalty_burn");
    assert!(result.is_deflation_controlled);
}

#[tokio::test]
async fn test_509_extreme_deflation_protocol_burn() {
    let env = RealTestEnvironment::new("test_509_extreme_deflation_protocol_burn").await.unwrap();
    let result = test_deflation_mechanism(&env, "extreme_deflation", 10.0).await;
    
    assert_eq!(result.current_deflation_rate, 12.0);
    assert_eq!(result.target_deflation_rate, 10.0);
    assert_eq!(result.deflation_adjustment, 3.6); // (12.0 - 10.0) * 1.8
    assert_eq!(result.tokens_burned, 120_000_000); // 1B * 0.12
    assert_eq!(result.burn_mechanism, "protocol_revenue_burn");
    assert!(!result.is_deflation_controlled); // Should fail due to extreme adjustment
}

#[tokio::test]
async fn test_510_governance_directed_deflation() {
    let env = RealTestEnvironment::new("test_510_governance_directed_deflation").await.unwrap();
    let result = test_deflation_mechanism(&env, "controlled_deflation", 2.0).await;
    
    assert_eq!(result.current_deflation_rate, 2.0);
    assert_eq!(result.target_deflation_rate, 2.0);
    assert_eq!(result.deflation_adjustment, 0.0); // (2.0 - 2.0) * 0.9
    assert_eq!(result.tokens_burned, 20_000_000); // 1B * 0.02
    assert_eq!(result.burn_mechanism, "governance_directed_burn");
    assert!(result.is_deflation_controlled);
}

// ============================================================================
// MONETARY POLICY EFFECTIVENESS TESTS (Tests 511-515)
// ============================================================================

#[tokio::test]
async fn test_511_expansionary_policy_recession() {
    let env = RealTestEnvironment::new("test_511_expansionary_policy_recession").await.unwrap();
    let result = test_monetary_policy(&env, "expansionary", "recession").await;
    
    assert_eq!(result.policy_type, "expansionary");
    assert_eq!(result.policy_effectiveness, 0.85);
    assert_eq!(result.economic_stability, 0.70);
    assert_eq!(result.price_stability_impact, 0.60);
    assert_eq!(result.market_response, 0.80);
    assert_eq!(result.policy_duration, Duration::from_secs(72 * 3600));
    assert!(result.is_policy_successful);
}

#[tokio::test]
async fn test_512_contractionary_policy_boom() {
    let env = RealTestEnvironment::new("test_512_contractionary_policy_boom").await.unwrap();
    let result = test_monetary_policy(&env, "contractionary", "boom").await;
    
    assert_eq!(result.policy_type, "contractionary");
    assert_eq!(result.policy_effectiveness, 0.90);
    assert_eq!(result.economic_stability, 0.85);
    assert_eq!(result.price_stability_impact, 0.70);
    assert_eq!(result.market_response, 0.85);
    assert_eq!(result.policy_duration, Duration::from_secs(96 * 3600));
    assert!(result.is_policy_successful);
}

#[tokio::test]
async fn test_513_neutral_policy_stable_conditions() {
    let env = RealTestEnvironment::new("test_513_neutral_policy_stable_conditions").await.unwrap();
    let result = test_monetary_policy(&env, "neutral", "stable").await;
    
    assert_eq!(result.policy_type, "neutral");
    assert_eq!(result.policy_effectiveness, 0.95);
    assert_eq!(result.economic_stability, 0.95);
    assert_eq!(result.price_stability_impact, 0.30);
    assert_eq!(result.market_response, 0.90);
    assert_eq!(result.policy_duration, Duration::from_secs(24 * 3600));
    assert!(result.is_policy_successful);
}

#[tokio::test]
async fn test_514_adaptive_policy_volatile_markets() {
    let env = RealTestEnvironment::new("test_514_adaptive_policy_volatile_markets").await.unwrap();
    let result = test_monetary_policy(&env, "adaptive", "volatile").await;
    
    assert_eq!(result.policy_type, "adaptive");
    assert_eq!(result.policy_effectiveness, 0.70);
    assert_eq!(result.economic_stability, 0.60);
    assert_eq!(result.price_stability_impact, 0.80);
    assert_eq!(result.market_response, 0.65);
    assert_eq!(result.policy_duration, Duration::from_secs(120 * 3600));
    assert!(result.is_policy_successful);
}

#[tokio::test]
async fn test_515_expansionary_policy_normal_conditions() {
    let env = RealTestEnvironment::new("test_515_expansionary_policy_normal_conditions").await.unwrap();
    let result = test_monetary_policy(&env, "expansionary", "normal").await;
    
    assert_eq!(result.policy_effectiveness, 0.75);
    assert_eq!(result.economic_stability, 0.80);
    assert_eq!(result.market_response, 0.70);
    assert!(result.policy_effectiveness >= 0.7);
    assert!(result.is_policy_successful);
}

// ============================================================================
// SUPPLY ELASTICITY TESTS (Tests 516-520)
// ============================================================================

#[tokio::test]
async fn test_516_high_demand_flexible_supply() {
    let env = RealTestEnvironment::new("test_516_high_demand_flexible_supply").await.unwrap();
    let result = test_supply_elasticity(&env, "high_demand", "flexible").await;
    
    assert_eq!(result.demand_change, 50.0);
    assert_eq!(result.supply_response, 40.0); // 50.0 * 0.8
    assert_eq!(result.elasticity_coefficient, 0.8); // 40.0 / 50.0
    assert_eq!(result.price_impact, 5.0); // 50.0 * (1.0 - 0.8) * 0.5
    assert_eq!(result.market_equilibrium, 0.95); // 1.0 - (5.0 / 100.0)
    assert_eq!(result.adjustment_speed, Duration::from_secs(30 * 60));
    assert!(result.is_elastic_response);
}

#[tokio::test]
async fn test_517_high_demand_rigid_supply() {
    let env = RealTestEnvironment::new("test_517_high_demand_rigid_supply").await.unwrap();
    let result = test_supply_elasticity(&env, "high_demand", "rigid").await;
    
    assert_eq!(result.demand_change, 50.0);
    assert_eq!(result.supply_response, 15.0); // 50.0 * 0.3
    assert_eq!(result.elasticity_coefficient, 0.3); // 15.0 / 50.0
    assert_eq!(result.price_impact, 17.5); // 50.0 * (1.0 - 0.3) * 0.5
    assert_eq!(result.market_equilibrium, 0.825); // 1.0 - (17.5 / 100.0)
    assert_eq!(result.adjustment_speed, Duration::from_secs(120 * 60));
    assert!(!result.is_elastic_response); // Should fail due to low elasticity
}

#[tokio::test]
async fn test_518_low_demand_flexible_response() {
    let env = RealTestEnvironment::new("test_518_low_demand_flexible_response").await.unwrap();
    let result = test_supply_elasticity(&env, "low_demand", "flexible").await;
    
    assert_eq!(result.demand_change, -30.0);
    assert_eq!(result.supply_response, -27.0); // -30.0 * 0.9
    assert_eq!(result.elasticity_coefficient, 0.9); // -27.0 / -30.0
    assert_eq!(result.price_impact, -1.5); // -30.0 * (1.0 - 0.9) * 0.5
    assert_eq!(result.market_equilibrium, 0.985); // 1.0 - (1.5 / 100.0)
    assert_eq!(result.adjustment_speed, Duration::from_secs(20 * 60));
    assert!(result.is_elastic_response);
}

#[tokio::test]
async fn test_519_volatile_demand_adaptive_supply() {
    let env = RealTestEnvironment::new("test_519_volatile_demand_adaptive_supply").await.unwrap();
    let result = test_supply_elasticity(&env, "volatile_demand", "adaptive").await;
    
    assert_eq!(result.demand_change, 25.0);
    assert_eq!(result.supply_response, 17.5); // 25.0 * 0.7
    assert_eq!(result.elasticity_coefficient, 0.7); // 17.5 / 25.0
    assert_eq!(result.price_impact, 3.75); // 25.0 * (1.0 - 0.7) * 0.5
    assert_eq!(result.market_equilibrium, 0.9625); // 1.0 - (3.75 / 100.0)
    assert!(result.is_elastic_response);
}

#[tokio::test]
async fn test_520_stable_demand_conservative_supply() {
    let env = RealTestEnvironment::new("test_520_stable_demand_conservative_supply").await.unwrap();
    let result = test_supply_elasticity(&env, "stable_demand", "conservative").await;
    
    assert_eq!(result.demand_change, 5.0);
    assert_eq!(result.supply_response, 2.5); // 5.0 * 0.5
    assert_eq!(result.elasticity_coefficient, 0.5); // 2.5 / 5.0
    assert_eq!(result.price_impact, 1.25); // 5.0 * (1.0 - 0.5) * 0.5
    assert_eq!(result.market_equilibrium, 0.9875); // 1.0 - (1.25 / 100.0)
    assert!(result.is_elastic_response); // Should pass with exactly 0.5 coefficient
}

// ============================================================================
// ECONOMIC STABILITY TESTS (Tests 521-525)
// ============================================================================

#[tokio::test]
async fn test_521_stable_growth_conditions() {
    let env = RealTestEnvironment::new("test_521_stable_growth_conditions").await.unwrap();
    let result = test_economic_stability(&env, "stable_growth").await;
    
    assert_eq!(result.inflation_volatility, 0.05);
    assert_eq!(result.price_stability_score, 0.95);
    assert_eq!(result.economic_growth_rate, 3.5);
    assert_eq!(result.market_confidence, 0.90);
    assert_eq!(result.long_term_sustainability, 0.95);
    assert_eq!(result.stability_metrics.len(), 5);
    assert!(result.is_economically_stable);
}

#[tokio::test]
async fn test_522_moderate_volatility_management() {
    let env = RealTestEnvironment::new("test_522_moderate_volatility_management").await.unwrap();
    let result = test_economic_stability(&env, "moderate_volatility").await;
    
    assert_eq!(result.inflation_volatility, 0.15);
    assert_eq!(result.price_stability_score, 0.80);
    assert_eq!(result.economic_growth_rate, 2.8);
    assert_eq!(result.market_confidence, 0.75);
    assert_eq!(result.long_term_sustainability, 0.85);
    assert!(result.is_economically_stable);
}

#[tokio::test]
async fn test_523_high_volatility_stress_test() {
    let env = RealTestEnvironment::new("test_523_high_volatility_stress_test").await.unwrap();
    let result = test_economic_stability(&env, "high_volatility").await;
    
    assert_eq!(result.inflation_volatility, 0.35);
    assert_eq!(result.price_stability_score, 0.60);
    assert_eq!(result.economic_growth_rate, 1.5);
    assert_eq!(result.market_confidence, 0.55);
    assert_eq!(result.long_term_sustainability, 0.70);
    assert!(!result.is_economically_stable); // Should fail due to high volatility
}

#[tokio::test]
async fn test_524_recession_recovery_dynamics() {
    let env = RealTestEnvironment::new("test_524_recession_recovery_dynamics").await.unwrap();
    let result = test_economic_stability(&env, "recession_recovery").await;
    
    assert_eq!(result.inflation_volatility, 0.25);
    assert_eq!(result.price_stability_score, 0.70);
    assert_eq!(result.economic_growth_rate, -1.0);
    assert_eq!(result.market_confidence, 0.60);
    assert_eq!(result.long_term_sustainability, 0.75);
    assert!(result.is_economically_stable); // Should pass despite negative growth
}

#[tokio::test]
async fn test_525_boom_cycle_sustainability() {
    let env = RealTestEnvironment::new("test_525_boom_cycle_sustainability").await.unwrap();
    let result = test_economic_stability(&env, "boom_cycle").await;
    
    assert_eq!(result.inflation_volatility, 0.20);
    assert_eq!(result.price_stability_score, 0.75);
    assert_eq!(result.economic_growth_rate, 6.0);
    assert_eq!(result.market_confidence, 0.85);
    assert_eq!(result.long_term_sustainability, 0.80);
    assert!(result.stability_metrics.len() == 5);
    assert!(result.is_economically_stable);
}
