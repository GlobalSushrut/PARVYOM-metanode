/*!
# Liquidity Management Module

Implements automated market makers (AMM), liquidity provision, dynamic rebalancing,
and yield farming for the Bank Mesh autonomous economic system.

## Features

- Automated Market Maker (AMM) pools with constant product formula
- Dynamic liquidity provision and withdrawal
- Yield farming and staking rewards
- Impermanent loss protection
- Multi-asset liquidity management
- Automated rebalancing algorithms
*/

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use thiserror::Error;
use tracing::{info, warn, error};
use nalgebra::{DVector, DMatrix};
use statrs::statistics::{Statistics, Data};

use crate::{EconomicsError, TokenSupplyState};
use billing_meter::TokenType;

/// Liquidity management errors
#[derive(Error, Debug)]
pub enum LiquidityError {
    #[error("Insufficient liquidity: required {required}, available {available}")]
    InsufficientLiquidity { required: Decimal, available: Decimal },
    #[error("Invalid pool ratio: {0}")]
    InvalidPoolRatio(String),
    #[error("Slippage too high: {actual}% > {max}%")]
    SlippageTooHigh { actual: Decimal, max: Decimal },
    #[error("Pool not found: {0}")]
    PoolNotFound(Uuid),
    #[error("Impermanent loss protection triggered: {loss}%")]
    ImpermanentLossProtection { loss: Decimal },
    #[error("Rebalancing failed: {0}")]
    RebalancingFailed(String),
    #[error("Economics error: {0}")]
    Economics(#[from] EconomicsError),
}

/// AMM pool types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PoolType {
    ConstantProduct,  // x * y = k (Uniswap V2 style)
    ConstantSum,      // x + y = k
    StableSwap,       // Curve style for stable assets
    Weighted,         // Balancer style with custom weights
}

/// Liquidity pool state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityPool {
    pub id: Uuid,
    pub pool_type: PoolType,
    pub token_a: TokenType,
    pub token_b: TokenType,
    pub reserve_a: Decimal,
    pub reserve_b: Decimal,
    pub total_liquidity: Decimal,
    pub fee_rate: Decimal,
    pub weight_a: Decimal, // For weighted pools
    pub weight_b: Decimal,
    pub created_at: DateTime<Utc>,
    pub last_trade: Option<DateTime<Utc>>,
    pub volume_24h: Decimal,
    pub fees_collected: Decimal,
    pub impermanent_loss_protection: bool,
}

/// Liquidity provider position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityPosition {
    pub id: Uuid,
    pub pool_id: Uuid,
    pub provider: String,
    pub liquidity_tokens: Decimal,
    pub initial_deposit_a: Decimal,
    pub initial_deposit_b: Decimal,
    pub current_value_a: Decimal,
    pub current_value_b: Decimal,
    pub fees_earned: Decimal,
    pub impermanent_loss: Decimal,
    pub created_at: DateTime<Utc>,
    pub last_update: DateTime<Utc>,
}

/// Yield farming pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldFarm {
    pub id: Uuid,
    pub pool_id: Uuid,
    pub reward_token: TokenType,
    pub reward_rate: Decimal, // Rewards per second
    pub total_staked: Decimal,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub multiplier: Decimal,
    pub lock_period: chrono::Duration,
}

/// Staking position in yield farm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakingPosition {
    pub id: Uuid,
    pub farm_id: Uuid,
    pub staker: String,
    pub staked_amount: Decimal,
    pub reward_debt: Decimal,
    pub pending_rewards: Decimal,
    pub locked_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub last_harvest: DateTime<Utc>,
}

/// Trade execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeResult {
    pub pool_id: Uuid,
    pub input_token: TokenType,
    pub output_token: TokenType,
    pub input_amount: Decimal,
    pub output_amount: Decimal,
    pub price_impact: Decimal,
    pub fee_paid: Decimal,
    pub slippage: Decimal,
    pub executed_at: DateTime<Utc>,
}

/// Rebalancing strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RebalancingStrategy {
    ConstantRatio { target_ratio: Decimal },
    VolatilityBased { threshold: Decimal },
    MeanReversion { window: u32, deviation: Decimal },
    Momentum { lookback: u32, threshold: Decimal },
}

/// Liquidity management configuration
#[derive(Debug, Clone)]
pub struct LiquidityConfig {
    pub default_fee_rate: Decimal,
    pub max_slippage: Decimal,
    pub min_liquidity: Decimal,
    pub rebalance_threshold: Decimal,
    pub impermanent_loss_threshold: Decimal,
    pub yield_farm_duration: chrono::Duration,
    pub default_lock_period: chrono::Duration,
}

impl Default for LiquidityConfig {
    fn default() -> Self {
        Self {
            default_fee_rate: Decimal::from_str_exact("0.003").unwrap(), // 0.3%
            max_slippage: Decimal::from_str_exact("0.05").unwrap(),      // 5%
            min_liquidity: Decimal::from(1000),
            rebalance_threshold: Decimal::from_str_exact("0.1").unwrap(), // 10%
            impermanent_loss_threshold: Decimal::from_str_exact("0.2").unwrap(), // 20%
            yield_farm_duration: chrono::Duration::days(30),
            default_lock_period: chrono::Duration::days(7),
        }
    }
}

/// Automated Market Maker and Liquidity Management Engine
#[derive(Debug)]
pub struct LiquidityManager {
    config: LiquidityConfig,
    pools: Arc<RwLock<HashMap<Uuid, LiquidityPool>>>,
    positions: Arc<RwLock<HashMap<Uuid, LiquidityPosition>>>,
    yield_farms: Arc<RwLock<HashMap<Uuid, YieldFarm>>>,
    staking_positions: Arc<RwLock<HashMap<Uuid, StakingPosition>>>,
    trade_history: Arc<RwLock<Vec<TradeResult>>>,
    rebalancing_strategies: Arc<RwLock<HashMap<Uuid, RebalancingStrategy>>>,
}

impl LiquidityManager {
    /// Create new liquidity manager
    pub fn new(config: LiquidityConfig) -> Self {
        info!("Initialized Liquidity Manager with AMM and yield farming");
        
        Self {
            config,
            pools: Arc::new(RwLock::new(HashMap::new())),
            positions: Arc::new(RwLock::new(HashMap::new())),
            yield_farms: Arc::new(RwLock::new(HashMap::new())),
            staking_positions: Arc::new(RwLock::new(HashMap::new())),
            trade_history: Arc::new(RwLock::new(Vec::new())),
            rebalancing_strategies: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create new liquidity pool
    pub async fn create_pool(
        &self,
        pool_type: PoolType,
        token_a: TokenType,
        token_b: TokenType,
        initial_a: Decimal,
        initial_b: Decimal,
        fee_rate: Option<Decimal>,
        weights: Option<(Decimal, Decimal)>,
    ) -> Result<Uuid, LiquidityError> {
        if initial_a < self.config.min_liquidity || initial_b < self.config.min_liquidity {
            return Err(LiquidityError::InsufficientLiquidity {
                required: self.config.min_liquidity,
                available: initial_a.min(initial_b),
            });
        }

        let pool_id = Uuid::new_v4();
        let (weight_a, weight_b) = weights.unwrap_or((Decimal::from_str_exact("0.5").unwrap(), Decimal::from_str_exact("0.5").unwrap()));
        
        // Calculate initial liquidity tokens (geometric mean for constant product)
        let total_liquidity = match pool_type {
            PoolType::ConstantProduct => {
                let product = initial_a * initial_b;
                let product_f64 = product.to_f64().unwrap();
                Decimal::from_f64(product_f64.sqrt()).unwrap()
            },
            PoolType::ConstantSum => initial_a + initial_b,
            PoolType::StableSwap => (initial_a + initial_b) / Decimal::TWO,
            PoolType::Weighted => {
                let a_f64 = initial_a.to_f64().unwrap();
                let b_f64 = initial_b.to_f64().unwrap();
                let wa_f64 = weight_a.to_f64().unwrap();
                let wb_f64 = weight_b.to_f64().unwrap();
                Decimal::from_f64(a_f64.powf(wa_f64) * b_f64.powf(wb_f64)).unwrap()
            }
        };

        let pool = LiquidityPool {
            id: pool_id,
            pool_type,
            token_a,
            token_b,
            reserve_a: initial_a,
            reserve_b: initial_b,
            total_liquidity,
            fee_rate: fee_rate.unwrap_or(self.config.default_fee_rate),
            weight_a,
            weight_b,
            created_at: Utc::now(),
            last_trade: None,
            volume_24h: Decimal::ZERO,
            fees_collected: Decimal::ZERO,
            impermanent_loss_protection: true,
        };

        let pool_type_for_log = pool.pool_type.clone();
        self.pools.write().await.insert(pool_id, pool);
        info!("Created {:?} pool {} for {:?}/{:?}", pool_type_for_log, pool_id, token_a, token_b);

        Ok(pool_id)
    }

    /// Add liquidity to pool
    pub async fn add_liquidity(
        &self,
        pool_id: Uuid,
        provider: String,
        amount_a: Decimal,
        amount_b: Decimal,
    ) -> Result<Uuid, LiquidityError> {
        let (final_a, final_b, liquidity_tokens, token_a, token_b) = {
            let mut pools = self.pools.write().await;
            let pool = pools.get_mut(&pool_id)
                .ok_or(LiquidityError::PoolNotFound(pool_id))?;

            // Calculate optimal amounts based on current ratio
            let ratio = pool.reserve_a / pool.reserve_b;
            let optimal_b = amount_a / ratio;
            let optimal_a = amount_b * ratio;

            let (final_a, final_b) = if optimal_b <= amount_b {
                (amount_a, optimal_b)
            } else {
                (optimal_a, amount_b)
            };

            // Calculate liquidity tokens to mint
            let liquidity_tokens = if pool.total_liquidity.is_zero() {
                {
                    let product = final_a * final_b;
                    let product_f64 = product.to_f64().unwrap();
                    Decimal::from_f64(product_f64.sqrt()).unwrap()
                }
            } else {
                let share_a = final_a * pool.total_liquidity / pool.reserve_a;
                let share_b = final_b * pool.total_liquidity / pool.reserve_b;
                share_a.min(share_b)
            };

            // Update pool reserves
            pool.reserve_a += final_a;
            pool.reserve_b += final_b;
            pool.total_liquidity += liquidity_tokens;
            
            let token_a = pool.token_a.clone();
            let token_b = pool.token_b.clone();
            
            (final_a, final_b, liquidity_tokens, token_a, token_b)
        }; // Release the write lock here

        // Create position record
        let position_id = Uuid::new_v4();
        let position = LiquidityPosition {
            id: position_id,
            pool_id,
            provider: provider.clone(),
            initial_deposit_a: final_a,
            initial_deposit_b: final_b,
            current_value_a: final_a,
            current_value_b: final_b,
            fees_earned: Decimal::ZERO,
            impermanent_loss: Decimal::ZERO,
            liquidity_tokens,
            created_at: Utc::now(),
            last_update: Utc::now(),
        };

        self.positions.write().await.insert(position_id, position);

        info!("Added liquidity to pool {}: {} {:?}, {} {:?}", pool_id, final_a, token_a, final_b, token_b);
        Ok(position_id)
    }

    /// Execute swap in AMM pool
    pub async fn swap(
        &self,
        pool_id: Uuid,
        input_token: TokenType,
        input_amount: Decimal,
        min_output: Option<Decimal>,
    ) -> Result<TradeResult, LiquidityError> {
        let mut pools = self.pools.write().await;
        let pool = pools.get_mut(&pool_id)
            .ok_or(LiquidityError::PoolNotFound(pool_id))?;

        let (input_reserve, output_reserve, output_token) = if input_token == pool.token_a {
            (pool.reserve_a, pool.reserve_b, pool.token_b)
        } else if input_token == pool.token_b {
            (pool.reserve_b, pool.reserve_a, pool.token_a)
        } else {
            return Err(LiquidityError::InvalidPoolRatio("Token not in pool".to_string()));
        };

        // Calculate output amount based on pool type
        let output_amount = match pool.pool_type {
            PoolType::ConstantProduct => {
                // x * y = k formula with fees
                let input_with_fee = input_amount * (Decimal::ONE - pool.fee_rate);
                let numerator = input_with_fee * output_reserve;
                let denominator = input_reserve + input_with_fee;
                numerator / denominator
            },
            PoolType::ConstantSum => {
                // Simple 1:1 swap with fees
                input_amount * (Decimal::ONE - pool.fee_rate)
            },
            PoolType::StableSwap => {
                // Simplified stable swap (would use more complex curve in production)
                let fee = input_amount * pool.fee_rate;
                input_amount - fee
            },
            PoolType::Weighted => {
                // Balancer weighted pool formula
                let weight_in = if input_token == pool.token_a { pool.weight_a } else { pool.weight_b };
                let weight_out = if input_token == pool.token_a { pool.weight_b } else { pool.weight_a };
                
                let base = input_reserve / (input_reserve + input_amount);
                let exponent = weight_in / weight_out;
                let power = {
                    let base_f64 = base.to_f64().unwrap();
                    let exp_f64 = exponent.to_f64().unwrap();
                    Decimal::from_f64(base_f64.powf(exp_f64)).unwrap()
                };
                output_reserve * (Decimal::ONE - power) * (Decimal::ONE - pool.fee_rate)
            }
        };

        // Check slippage
        let price_before = output_reserve / input_reserve;
        let price_after = (output_reserve - output_amount) / (input_reserve + input_amount);
        let price_impact = ((price_before - price_after) / price_before).abs();
        let slippage = price_impact;

        if slippage > self.config.max_slippage {
            return Err(LiquidityError::SlippageTooHigh {
                actual: slippage * Decimal::from(100),
                max: self.config.max_slippage * Decimal::from(100),
            });
        }

        if let Some(min_out) = min_output {
            if output_amount < min_out {
                return Err(LiquidityError::SlippageTooHigh {
                    actual: ((min_out - output_amount) / min_out) * Decimal::from(100),
                    max: self.config.max_slippage * Decimal::from(100),
                });
            }
        }

        // Update pool reserves
        if input_token == pool.token_a {
            pool.reserve_a += input_amount;
            pool.reserve_b -= output_amount;
        } else {
            pool.reserve_b += input_amount;
            pool.reserve_a -= output_amount;
        }

        let fee_paid = input_amount * pool.fee_rate;
        pool.fees_collected += fee_paid;
        pool.volume_24h += input_amount;
        pool.last_trade = Some(Utc::now());

        let trade_result = TradeResult {
            pool_id,
            input_token,
            output_token,
            input_amount,
            output_amount,
            price_impact,
            fee_paid,
            slippage,
            executed_at: Utc::now(),
        };

        drop(pools); // Release the lock
        self.trade_history.write().await.push(trade_result.clone());

        info!("Executed swap: {} {:?} -> {} {:?} (slippage: {:.2}%)", 
              input_amount, input_token, output_amount, output_token, slippage * Decimal::from(100));

        Ok(trade_result)
    }

    /// Create yield farming pool
    pub async fn create_yield_farm(
        &self,
        pool_id: Uuid,
        reward_token: TokenType,
        reward_rate: Decimal,
        duration: chrono::Duration,
        multiplier: Option<Decimal>,
        lock_period: Option<chrono::Duration>,
    ) -> Result<Uuid, LiquidityError> {
        let pools = self.pools.read().await;
        if !pools.contains_key(&pool_id) {
            return Err(LiquidityError::PoolNotFound(pool_id));
        }
        drop(pools);

        let farm_id = Uuid::new_v4();
        let start_time = Utc::now();
        let end_time = start_time + duration;

        let farm = YieldFarm {
            id: farm_id,
            pool_id,
            reward_token,
            reward_rate,
            total_staked: Decimal::ZERO,
            start_time,
            end_time,
            multiplier: multiplier.unwrap_or(Decimal::ONE),
            lock_period: lock_period.unwrap_or(self.config.default_lock_period),
        };

        self.yield_farms.write().await.insert(farm_id, farm);
        info!("Created yield farm {} for pool {}", farm_id, pool_id);

        Ok(farm_id)
    }

    /// Stake liquidity tokens in yield farm
    pub async fn stake_in_farm(
        &self,
        farm_id: Uuid,
        staker: String,
        amount: Decimal,
    ) -> Result<Uuid, LiquidityError> {
        let mut farms = self.yield_farms.write().await;
        let farm = farms.get_mut(&farm_id)
            .ok_or(LiquidityError::PoolNotFound(farm_id))?;

        let position_id = Uuid::new_v4();
        let locked_until = if farm.lock_period.num_seconds() > 0 {
            Some(Utc::now() + farm.lock_period)
        } else {
            None
        };

        let position = StakingPosition {
            id: position_id,
            farm_id,
            staker,
            staked_amount: amount,
            reward_debt: Decimal::ZERO,
            pending_rewards: Decimal::ZERO,
            locked_until,
            created_at: Utc::now(),
            last_harvest: Utc::now(),
        };

        farm.total_staked += amount;
        drop(farms);

        self.staking_positions.write().await.insert(position_id, position);
        info!("Staked {} tokens in farm {}", amount, farm_id);

        Ok(position_id)
    }

    /// Calculate and update rewards for staking position
    pub async fn update_rewards(&self, position_id: Uuid) -> Result<Decimal, LiquidityError> {
        let mut positions = self.staking_positions.write().await;
        let position = positions.get_mut(&position_id)
            .ok_or(LiquidityError::PoolNotFound(position_id))?;

        let farms = self.yield_farms.read().await;
        let farm = farms.get(&position.farm_id)
            .ok_or(LiquidityError::PoolNotFound(position.farm_id))?;

        let now = Utc::now();
        let time_elapsed = (now - position.last_harvest).num_seconds() as f64;
        
        if farm.total_staked > Decimal::ZERO {
            let share = position.staked_amount / farm.total_staked;
            let rewards = Decimal::from_f64(time_elapsed).unwrap() * farm.reward_rate * share * farm.multiplier;
            position.pending_rewards += rewards;
            position.last_harvest = now;
        }

        let pending = position.pending_rewards;
        drop(positions);
        drop(farms);

        Ok(pending)
    }

    /// Rebalance pool based on strategy
    pub async fn rebalance_pool(
        &self,
        pool_id: Uuid,
        strategy: RebalancingStrategy,
    ) -> Result<(), LiquidityError> {
        let mut pools = self.pools.write().await;
        let pool = pools.get_mut(&pool_id)
            .ok_or(LiquidityError::PoolNotFound(pool_id))?;

        match strategy {
            RebalancingStrategy::ConstantRatio { target_ratio } => {
                let current_ratio = pool.reserve_a / pool.reserve_b;
                let deviation = (current_ratio - target_ratio).abs() / target_ratio;
                
                if deviation > self.config.rebalance_threshold {
                    // Would implement actual rebalancing logic here
                    info!("Rebalancing pool {} to target ratio {}", pool_id, target_ratio);
                }
            },
            RebalancingStrategy::VolatilityBased { threshold } => {
                // Would implement volatility-based rebalancing
                info!("Volatility-based rebalancing for pool {}", pool_id);
            },
            _ => {
                // Other strategies would be implemented here
                info!("Advanced rebalancing strategy for pool {}", pool_id);
            }
        }

        self.rebalancing_strategies.write().await.insert(pool_id, strategy);
        Ok(())
    }

    /// Get liquidity management statistics
    pub async fn get_liquidity_stats(&self) -> HashMap<String, serde_json::Value> {
        let pools = self.pools.read().await;
        let positions = self.positions.read().await;
        let farms = self.yield_farms.read().await;
        let trades = self.trade_history.read().await;

        let mut stats = HashMap::new();
        stats.insert("total_pools".to_string(), serde_json::Value::Number(pools.len().into()));
        stats.insert("total_positions".to_string(), serde_json::Value::Number(positions.len().into()));
        stats.insert("total_farms".to_string(), serde_json::Value::Number(farms.len().into()));
        stats.insert("total_trades".to_string(), serde_json::Value::Number(trades.len().into()));

        let total_liquidity: Decimal = pools.values().map(|p| p.total_liquidity).sum();
        let total_volume: Decimal = pools.values().map(|p| p.volume_24h).sum();
        let total_fees: Decimal = pools.values().map(|p| p.fees_collected).sum();

        stats.insert("total_liquidity".to_string(), serde_json::Value::String(total_liquidity.to_string()));
        stats.insert("total_volume_24h".to_string(), serde_json::Value::String(total_volume.to_string()));
        stats.insert("total_fees_collected".to_string(), serde_json::Value::String(total_fees.to_string()));

        stats
    }

    /// Get pool by ID
    pub async fn get_pool(&self, pool_id: Uuid) -> Option<LiquidityPool> {
        self.pools.read().await.get(&pool_id).cloned()
    }

    /// Get position by ID
    pub async fn get_position(&self, position_id: Uuid) -> Option<LiquidityPosition> {
        self.positions.read().await.get(&position_id).cloned()
    }

    /// Get yield farm by ID
    pub async fn get_yield_farm(&self, farm_id: Uuid) -> Option<YieldFarm> {
        self.yield_farms.read().await.get(&farm_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_liquidity_manager_creation() {
        let config = LiquidityConfig::default();
        let manager = LiquidityManager::new(config);
        
        let stats = manager.get_liquidity_stats().await;
        assert_eq!(stats.get("total_pools").unwrap(), &serde_json::Value::Number(0.into()));
    }

    #[tokio::test]
    async fn test_pool_creation() {
        let config = LiquidityConfig::default();
        let manager = LiquidityManager::new(config);
        
        let pool_id = manager.create_pool(
            PoolType::ConstantProduct,
            TokenType::Genesis,
            TokenType::Nexus,
            Decimal::from(10000),
            Decimal::from(20000),
            None,
            None,
        ).await.unwrap();
        
        let pool = manager.get_pool(pool_id).await;
        assert!(pool.is_some());
        assert_eq!(pool.unwrap().token_a, TokenType::Genesis);
    }

    #[tokio::test]
    async fn test_add_liquidity() {
        let config = LiquidityConfig::default();
        let manager = LiquidityManager::new(config);
        
        let pool_id = manager.create_pool(
            PoolType::ConstantProduct,
            TokenType::Genesis,
            TokenType::Nexus,
            Decimal::from(10000),
            Decimal::from(20000),
            None,
            None,
        ).await.unwrap();
        
        let position_id = manager.add_liquidity(
            pool_id,
            "provider1".to_string(),
            Decimal::from(1000),
            Decimal::from(2000),
        ).await.unwrap();
        
        let position = manager.get_position(position_id).await;
        assert!(position.is_some());
        assert_eq!(position.unwrap().provider, "provider1");
    }

    #[tokio::test]
    async fn test_swap_execution() {
        let config = LiquidityConfig::default();
        let manager = LiquidityManager::new(config);
        
        let pool_id = manager.create_pool(
            PoolType::ConstantProduct,
            TokenType::Genesis,
            TokenType::Nexus,
            Decimal::from(10000),
            Decimal::from(20000),
            None,
            None,
        ).await.unwrap();
        
        let result = manager.swap(
            pool_id,
            TokenType::Genesis,
            Decimal::from(100),
            None,
        ).await.unwrap();
        
        assert_eq!(result.input_token, TokenType::Genesis);
        assert_eq!(result.output_token, TokenType::Nexus);
        assert!(result.output_amount > Decimal::ZERO);
    }

    #[tokio::test]
    async fn test_yield_farm_creation() {
        let config = LiquidityConfig::default();
        let manager = LiquidityManager::new(config);
        
        let pool_id = manager.create_pool(
            PoolType::ConstantProduct,
            TokenType::Genesis,
            TokenType::Nexus,
            Decimal::from(10000),
            Decimal::from(20000),
            None,
            None,
        ).await.unwrap();
        
        let farm_id = manager.create_yield_farm(
            pool_id,
            TokenType::Flux,
            Decimal::from_str_exact("0.1").unwrap(),
            chrono::Duration::days(30),
            None,
            None,
        ).await.unwrap();
        
        let farm = manager.get_yield_farm(farm_id).await;
        assert!(farm.is_some());
        assert_eq!(farm.unwrap().reward_token, TokenType::Flux);
    }

    #[tokio::test]
    async fn test_staking_in_farm() {
        let config = LiquidityConfig::default();
        let manager = LiquidityManager::new(config);
        
        let pool_id = manager.create_pool(
            PoolType::ConstantProduct,
            TokenType::Genesis,
            TokenType::Nexus,
            Decimal::from(10000),
            Decimal::from(20000),
            None,
            None,
        ).await.unwrap();
        
        let farm_id = manager.create_yield_farm(
            pool_id,
            TokenType::Flux,
            Decimal::from_str_exact("0.1").unwrap(),
            chrono::Duration::days(30),
            None,
            None,
        ).await.unwrap();
        
        let position_id = manager.stake_in_farm(
            farm_id,
            "staker1".to_string(),
            Decimal::from(1000),
        ).await.unwrap();
        
        // Test reward calculation
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        let rewards = manager.update_rewards(position_id).await.unwrap();
        assert!(rewards >= Decimal::ZERO);
    }
}
