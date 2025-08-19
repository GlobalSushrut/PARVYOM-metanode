/*!
# Cross-Chain Settlement Module

Implements bridge protocols, settlement verification, and cross-chain liquidity management
for the Bank Mesh autonomous economic system.

## Features

- Multi-chain bridge protocols (Ethereum, Polygon, BSC, Arbitrum)
- Atomic cross-chain swaps with HTLC support
- Settlement verification and finality tracking
- Cross-chain transaction routing and optimization
- Liquidity pool management across chains
*/

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use ethers::prelude::*;
use thiserror::Error;
use tracing::{info, warn, error};

use crate::{EconomicsError, TokenSupplyState};
use billing_meter::TokenType;

/// Supported blockchain networks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChainId {
    Ethereum = 1,
    Polygon = 137,
    BSC = 56,
    Arbitrum = 42161,
    Optimism = 10,
    Avalanche = 43114,
}

impl ChainId {
    pub fn name(&self) -> &'static str {
        match self {
            ChainId::Ethereum => "Ethereum",
            ChainId::Polygon => "Polygon",
            ChainId::BSC => "BSC",
            ChainId::Arbitrum => "Arbitrum",
            ChainId::Optimism => "Optimism",
            ChainId::Avalanche => "Avalanche",
        }
    }

    pub fn rpc_url(&self) -> &'static str {
        match self {
            ChainId::Ethereum => "https://eth.llamarpc.com",
            ChainId::Polygon => "https://polygon.llamarpc.com",
            ChainId::BSC => "https://bsc.llamarpc.com",
            ChainId::Arbitrum => "https://arbitrum.llamarpc.com",
            ChainId::Optimism => "https://optimism.llamarpc.com",
            ChainId::Avalanche => "https://avalanche.llamarpc.com",
        }
    }
}

/// Cross-chain settlement errors
#[derive(Error, Debug)]
pub enum SettlementError {
    #[error("Chain not supported: {0:?}")]
    UnsupportedChain(ChainId),
    #[error("Insufficient liquidity: required {required}, available {available}")]
    InsufficientLiquidity { required: Decimal, available: Decimal },
    #[error("Settlement timeout: {0}")]
    SettlementTimeout(String),
    #[error("Bridge protocol error: {0}")]
    BridgeProtocolError(String),
    #[error("HTLC error: {0}")]
    HTLCError(String),
    #[error("Economics error: {0}")]
    Economics(#[from] EconomicsError),
}

/// Cross-chain transaction status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Finalized,
    Failed,
    Expired,
}

/// Hash Time Locked Contract (HTLC) for atomic swaps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTLC {
    pub id: Uuid,
    pub hash_lock: String,
    pub time_lock: DateTime<Utc>,
    pub sender: String,
    pub receiver: String,
    pub amount: Decimal,
    pub source_chain: ChainId,
    pub target_chain: ChainId,
    pub status: TransactionStatus,
    pub created_at: DateTime<Utc>,
}

/// Cross-chain bridge transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeTransaction {
    pub id: Uuid,
    pub source_chain: ChainId,
    pub target_chain: ChainId,
    pub source_tx_hash: Option<String>,
    pub target_tx_hash: Option<String>,
    pub sender: String,
    pub receiver: String,
    pub token_address: String,
    pub amount: Decimal,
    pub fee: Decimal,
    pub status: TransactionStatus,
    pub confirmations: u32,
    pub required_confirmations: u32,
    pub created_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub finalized_at: Option<DateTime<Utc>>,
}

/// Liquidity pool for cross-chain operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainLiquidityPool {
    pub id: Uuid,
    pub chain_id: ChainId,
    pub token_address: String,
    pub total_liquidity: Decimal,
    pub available_liquidity: Decimal,
    pub reserved_liquidity: Decimal,
    pub fee_rate: Decimal,
    pub last_rebalance: DateTime<Utc>,
}

/// Settlement verification proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementProof {
    pub transaction_id: Uuid,
    pub chain_id: ChainId,
    pub block_hash: String,
    pub block_number: u64,
    pub transaction_index: u32,
    pub merkle_proof: Vec<String>,
    pub verified_at: DateTime<Utc>,
}

/// Cross-chain settlement configuration
#[derive(Debug, Clone)]
pub struct SettlementConfig {
    pub supported_chains: Vec<ChainId>,
    pub min_confirmations: HashMap<ChainId, u32>,
    pub max_settlement_time: chrono::Duration,
    pub default_fee_rate: Decimal,
    pub htlc_timeout: chrono::Duration,
    pub rebalance_threshold: Decimal,
}

impl Default for SettlementConfig {
    fn default() -> Self {
        let mut min_confirmations = HashMap::new();
        min_confirmations.insert(ChainId::Ethereum, 12);
        min_confirmations.insert(ChainId::Polygon, 20);
        min_confirmations.insert(ChainId::BSC, 15);
        min_confirmations.insert(ChainId::Arbitrum, 1);
        min_confirmations.insert(ChainId::Optimism, 1);
        min_confirmations.insert(ChainId::Avalanche, 5);

        Self {
            supported_chains: vec![
                ChainId::Ethereum,
                ChainId::Polygon,
                ChainId::BSC,
                ChainId::Arbitrum,
                ChainId::Optimism,
                ChainId::Avalanche,
            ],
            min_confirmations,
            max_settlement_time: chrono::Duration::hours(2),
            default_fee_rate: Decimal::from_str_exact("0.003").unwrap(), // 0.3%
            htlc_timeout: chrono::Duration::hours(24),
            rebalance_threshold: Decimal::from_str_exact("0.1").unwrap(), // 10%
        }
    }
}

/// Cross-chain settlement engine
#[derive(Debug)]
pub struct CrossChainSettlement {
    config: SettlementConfig,
    liquidity_pools: Arc<RwLock<HashMap<ChainId, CrossChainLiquidityPool>>>,
    pending_transactions: Arc<RwLock<HashMap<Uuid, BridgeTransaction>>>,
    active_htlcs: Arc<RwLock<HashMap<Uuid, HTLC>>>,
    settlement_proofs: Arc<RwLock<HashMap<Uuid, SettlementProof>>>,
    providers: HashMap<ChainId, Arc<Provider<Http>>>,
}

impl CrossChainSettlement {
    /// Create new cross-chain settlement engine
    pub async fn new(config: SettlementConfig) -> Result<Self, SettlementError> {
        let mut providers = HashMap::new();
        
        // Initialize providers for supported chains
        for &chain_id in &config.supported_chains {
            let provider = Provider::<Http>::try_from(chain_id.rpc_url())
                .map_err(|e| SettlementError::BridgeProtocolError(format!("Failed to connect to {}: {}", chain_id.name(), e)))?;
            providers.insert(chain_id, Arc::new(provider));
        }

        info!("Initialized cross-chain settlement for {} chains", config.supported_chains.len());

        Ok(Self {
            config,
            liquidity_pools: Arc::new(RwLock::new(HashMap::new())),
            pending_transactions: Arc::new(RwLock::new(HashMap::new())),
            active_htlcs: Arc::new(RwLock::new(HashMap::new())),
            settlement_proofs: Arc::new(RwLock::new(HashMap::new())),
            providers,
        })
    }

    /// Initialize liquidity pool for a chain
    pub async fn initialize_liquidity_pool(
        &self,
        chain_id: ChainId,
        token_address: String,
        initial_liquidity: Decimal,
    ) -> Result<Uuid, SettlementError> {
        if !self.config.supported_chains.contains(&chain_id) {
            return Err(SettlementError::UnsupportedChain(chain_id));
        }

        let pool_id = Uuid::new_v4();
        let pool = CrossChainLiquidityPool {
            id: pool_id,
            chain_id,
            token_address,
            total_liquidity: initial_liquidity,
            available_liquidity: initial_liquidity,
            reserved_liquidity: Decimal::ZERO,
            fee_rate: self.config.default_fee_rate,
            last_rebalance: Utc::now(),
        };

        self.liquidity_pools.write().await.insert(chain_id, pool);
        info!("Initialized liquidity pool for {} with {} tokens", chain_id.name(), initial_liquidity);

        Ok(pool_id)
    }

    /// Create cross-chain bridge transaction
    pub async fn create_bridge_transaction(
        &self,
        source_chain: ChainId,
        target_chain: ChainId,
        sender: String,
        receiver: String,
        token_address: String,
        amount: Decimal,
    ) -> Result<Uuid, SettlementError> {
        // Validate chains are supported
        if !self.config.supported_chains.contains(&source_chain) {
            return Err(SettlementError::UnsupportedChain(source_chain));
        }
        if !self.config.supported_chains.contains(&target_chain) {
            return Err(SettlementError::UnsupportedChain(target_chain));
        }

        // Check liquidity availability
        let pools = self.liquidity_pools.read().await;
        if let Some(target_pool) = pools.get(&target_chain) {
            if target_pool.available_liquidity < amount {
                return Err(SettlementError::InsufficientLiquidity {
                    required: amount,
                    available: target_pool.available_liquidity,
                });
            }
        } else {
            return Err(SettlementError::InsufficientLiquidity {
                required: amount,
                available: Decimal::ZERO,
            });
        }

        // Calculate fee
        let fee = amount * self.config.default_fee_rate;
        let required_confirmations = *self.config.min_confirmations.get(&source_chain).unwrap_or(&12);

        let transaction_id = Uuid::new_v4();
        let bridge_tx = BridgeTransaction {
            id: transaction_id,
            source_chain,
            target_chain,
            source_tx_hash: None,
            target_tx_hash: None,
            sender,
            receiver,
            token_address,
            amount,
            fee,
            status: TransactionStatus::Pending,
            confirmations: 0,
            required_confirmations,
            created_at: Utc::now(),
            confirmed_at: None,
            finalized_at: None,
        };

        self.pending_transactions.write().await.insert(transaction_id, bridge_tx);
        info!("Created bridge transaction {} from {} to {}", transaction_id, source_chain.name(), target_chain.name());

        Ok(transaction_id)
    }

    /// Create Hash Time Locked Contract for atomic swap
    pub async fn create_htlc(
        &self,
        source_chain: ChainId,
        target_chain: ChainId,
        sender: String,
        receiver: String,
        amount: Decimal,
        hash_lock: String,
    ) -> Result<Uuid, SettlementError> {
        let htlc_id = Uuid::new_v4();
        let time_lock = Utc::now() + self.config.htlc_timeout;

        let htlc = HTLC {
            id: htlc_id,
            hash_lock,
            time_lock,
            sender,
            receiver,
            amount,
            source_chain,
            target_chain,
            status: TransactionStatus::Pending,
            created_at: Utc::now(),
        };

        self.active_htlcs.write().await.insert(htlc_id, htlc);
        info!("Created HTLC {} for atomic swap", htlc_id);

        Ok(htlc_id)
    }

    /// Process settlement verification
    pub async fn verify_settlement(
        &self,
        transaction_id: Uuid,
        chain_id: ChainId,
        tx_hash: String,
    ) -> Result<SettlementProof, SettlementError> {
        let provider = self.providers.get(&chain_id)
            .ok_or(SettlementError::UnsupportedChain(chain_id))?;

        // Get transaction receipt
        let receipt = provider.get_transaction_receipt(H256::from_slice(&hex::decode(&tx_hash).map_err(|e| {
            SettlementError::BridgeProtocolError(format!("Invalid tx hash: {}", e))
        })?))
            .await
            .map_err(|e| SettlementError::BridgeProtocolError(format!("Failed to get receipt: {}", e)))?
            .ok_or(SettlementError::BridgeProtocolError("Transaction not found".to_string()))?;

        let proof = SettlementProof {
            transaction_id,
            chain_id,
            block_hash: format!("{:?}", receipt.block_hash.unwrap_or_default()),
            block_number: receipt.block_number.unwrap_or_default().as_u64(),
            transaction_index: receipt.transaction_index.as_u32(),
            merkle_proof: vec![], // Would implement actual merkle proof generation
            verified_at: Utc::now(),
        };

        self.settlement_proofs.write().await.insert(transaction_id, proof.clone());
        info!("Verified settlement for transaction {}", transaction_id);

        Ok(proof)
    }

    /// Get cross-chain settlement statistics
    pub async fn get_settlement_stats(&self) -> HashMap<String, serde_json::Value> {
        let pools = self.liquidity_pools.read().await;
        let pending_txs = self.pending_transactions.read().await;
        let active_htlcs = self.active_htlcs.read().await;

        let mut stats = HashMap::new();
        stats.insert("total_liquidity_pools".to_string(), serde_json::Value::Number(pools.len().into()));
        stats.insert("pending_transactions".to_string(), serde_json::Value::Number(pending_txs.len().into()));
        stats.insert("active_htlcs".to_string(), serde_json::Value::Number(active_htlcs.len().into()));
        stats.insert("supported_chains".to_string(), serde_json::Value::Number(self.config.supported_chains.len().into()));

        let total_liquidity: Decimal = pools.values().map(|p| p.total_liquidity).sum();
        stats.insert("total_liquidity".to_string(), serde_json::Value::String(total_liquidity.to_string()));

        stats
    }

    /// Get liquidity pool by chain
    pub async fn get_liquidity_pool(&self, chain_id: ChainId) -> Option<CrossChainLiquidityPool> {
        self.liquidity_pools.read().await.get(&chain_id).cloned()
    }

    /// Get bridge transaction by ID
    pub async fn get_bridge_transaction(&self, transaction_id: Uuid) -> Option<BridgeTransaction> {
        self.pending_transactions.read().await.get(&transaction_id).cloned()
    }

    /// Get HTLC by ID
    pub async fn get_htlc(&self, htlc_id: Uuid) -> Option<HTLC> {
        self.active_htlcs.read().await.get(&htlc_id).cloned()
    }

    /// Update transaction status
    pub async fn update_transaction_status(
        &self,
        transaction_id: Uuid,
        status: TransactionStatus,
        tx_hash: Option<String>,
    ) -> Result<(), SettlementError> {
        let mut pending_txs = self.pending_transactions.write().await;
        if let Some(tx) = pending_txs.get_mut(&transaction_id) {
            tx.status = status.clone();
            
            match status {
                TransactionStatus::Confirmed => {
                    tx.confirmed_at = Some(Utc::now());
                    if let Some(hash) = tx_hash {
                        if tx.source_tx_hash.is_none() {
                            tx.source_tx_hash = Some(hash);
                        } else {
                            tx.target_tx_hash = Some(hash);
                        }
                    }
                },
                TransactionStatus::Finalized => {
                    tx.finalized_at = Some(Utc::now());
                },
                _ => {}
            }
            
            info!("Updated transaction {} status to {:?}", transaction_id, status);
            Ok(())
        } else {
            Err(SettlementError::BridgeProtocolError("Transaction not found".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cross_chain_settlement_creation() {
        let config = SettlementConfig::default();
        let settlement = CrossChainSettlement::new(config).await;
        assert!(settlement.is_ok());
    }

    #[tokio::test]
    async fn test_liquidity_pool_initialization() {
        let config = SettlementConfig::default();
        let settlement = CrossChainSettlement::new(config).await.unwrap();
        
        let pool_id = settlement.initialize_liquidity_pool(
            ChainId::Ethereum,
            "0x1234567890123456789012345678901234567890".to_string(),
            Decimal::from(1000000),
        ).await.unwrap();
        
        let pool = settlement.get_liquidity_pool(ChainId::Ethereum).await;
        assert!(pool.is_some());
        assert_eq!(pool.unwrap().id, pool_id);
    }

    #[tokio::test]
    async fn test_bridge_transaction_creation() {
        let config = SettlementConfig::default();
        let settlement = CrossChainSettlement::new(config).await.unwrap();
        
        // Initialize liquidity pools first
        settlement.initialize_liquidity_pool(
            ChainId::Ethereum,
            "0x1234567890123456789012345678901234567890".to_string(),
            Decimal::from(1000000),
        ).await.unwrap();
        
        settlement.initialize_liquidity_pool(
            ChainId::Polygon,
            "0x1234567890123456789012345678901234567890".to_string(),
            Decimal::from(1000000),
        ).await.unwrap();
        
        let tx_id = settlement.create_bridge_transaction(
            ChainId::Ethereum,
            ChainId::Polygon,
            "0xsender".to_string(),
            "0xreceiver".to_string(),
            "0x1234567890123456789012345678901234567890".to_string(),
            Decimal::from(1000),
        ).await.unwrap();
        
        let bridge_tx = settlement.get_bridge_transaction(tx_id).await;
        assert!(bridge_tx.is_some());
        assert_eq!(bridge_tx.unwrap().amount, Decimal::from(1000));
    }

    #[tokio::test]
    async fn test_htlc_creation() {
        let config = SettlementConfig::default();
        let settlement = CrossChainSettlement::new(config).await.unwrap();
        
        let htlc_id = settlement.create_htlc(
            ChainId::Ethereum,
            ChainId::Polygon,
            "0xsender".to_string(),
            "0xreceiver".to_string(),
            Decimal::from(1000),
            "0xhashlock".to_string(),
        ).await.unwrap();
        
        let htlc = settlement.get_htlc(htlc_id).await;
        assert!(htlc.is_some());
        assert_eq!(htlc.unwrap().amount, Decimal::from(1000));
    }

    #[tokio::test]
    async fn test_settlement_stats() {
        let config = SettlementConfig::default();
        let settlement = CrossChainSettlement::new(config).await.unwrap();
        
        let stats = settlement.get_settlement_stats().await;
        assert!(stats.contains_key("supported_chains"));
        assert!(stats.contains_key("total_liquidity_pools"));
    }
}
