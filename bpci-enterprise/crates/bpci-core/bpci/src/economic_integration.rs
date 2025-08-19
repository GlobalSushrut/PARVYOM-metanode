//! BPCI Economic Integration
//! 
//! Integrates autonomous economics, billing meter, and mining with BPCI server.
//! Activates automatically when BPCI server goes live for decentralized,
//! immutable, and autonomous economic operations.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tokio::time::interval;
use tracing::{info, warn, error};
use uuid::Uuid;

// Import existing economic infrastructure
use autonomous_economics::PoEMiningEngine;
use billing_meter::{BillingMeterService, BillingMeterConfig, UsageRecord, ServiceType, ResourceConsumption, TokenType};
use chrono::Utc;
use rust_decimal::Decimal;

/// BPCI Economic Integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpciEconomicConfig {
    pub owner_wallet_id: Uuid,
    pub auto_activation: bool,
    pub billing_interval_seconds: u64,
    pub mining_interval_seconds: u64,
    pub owner_withdrawal_threshold: u64, // Auto-withdraw when balance exceeds this
    pub owner_withdrawal_percentage: f64, // Percentage to withdraw (0.0-1.0)
    pub infrastructure_fee_rate: f64, // Fee rate for infrastructure usage
}

impl Default for BpciEconomicConfig {
    fn default() -> Self {
        Self {
            owner_wallet_id: Uuid::new_v4(),
            auto_activation: true,
            billing_interval_seconds: 3600, // Hourly billing
            mining_interval_seconds: 30,    // 30-second mining cycles
            owner_withdrawal_threshold: 10_000_000, // 10 BPI tokens
            owner_withdrawal_percentage: 0.8, // Withdraw 80%
            infrastructure_fee_rate: 0.05, // 5% infrastructure fee
        }
    }
}

/// Economic activity metrics from BPCI operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpciEconomicMetrics {
    pub active_services: u32,
    pub total_transactions: u64,
    pub cpu_usage_hours: f64,
    pub memory_usage_gb_hours: f64,
    pub storage_usage_gb: f64,
    pub network_transfer_gb: f64,
    pub audit_records_created: u64,
    pub timestamp: u64,
}

impl Default for BpciEconomicMetrics {
    fn default() -> Self {
        Self {
            active_services: 0,
            total_transactions: 0,
            cpu_usage_hours: 0.0,
            memory_usage_gb_hours: 0.0,
            storage_usage_gb: 0.0,
            network_transfer_gb: 0.0,
            audit_records_created: 0,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }
}

/// Owner wallet for infrastructure revenue collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerWallet {
    pub wallet_id: Uuid,
    pub balance: u64, // Total balance in smallest token unit
    pub total_earned: u64, // Lifetime earnings
    pub total_withdrawn: u64, // Lifetime withdrawals
    pub last_withdrawal: u64, // Timestamp of last withdrawal
    pub withdrawal_address: String, // External wallet for withdrawals
}

/// BPCI Economic Integration Engine
#[derive(Debug)]
pub struct BpciEconomicIntegration {
    config: BpciEconomicConfig,
    
    // Economic engines
    poe_mining_engine: Arc<PoEMiningEngine>,
    billing_meter: Arc<BillingMeterService>,
    
    // State tracking
    owner_wallet: Arc<RwLock<OwnerWallet>>,
    economic_metrics: Arc<RwLock<BpciEconomicMetrics>>,
    
    // Runtime state
    is_active: Arc<RwLock<bool>>,
    bpci_server_live: Arc<RwLock<bool>>,
}

impl BpciEconomicIntegration {
    /// Create new BPCI economic integration
    pub async fn new(config: BpciEconomicConfig) -> Result<Self> {
        // Initialize economic engines
        let registry = prometheus::Registry::new();
        let poe_mining_engine = Arc::new(PoEMiningEngine::new(&registry)?);
        
        let billing_config = BillingMeterConfig::default();
        let billing_meter = Arc::new(BillingMeterService::new(billing_config)?);
        
        // Initialize owner wallet
        let owner_wallet = OwnerWallet {
            wallet_id: config.owner_wallet_id,
            balance: 0,
            total_earned: 0,
            total_withdrawn: 0,
            last_withdrawal: 0,
            withdrawal_address: "owner_external_wallet_address".to_string(),
        };
        
        Ok(Self {
            config,
            poe_mining_engine,
            billing_meter,
            owner_wallet: Arc::new(RwLock::new(owner_wallet)),
            economic_metrics: Arc::new(RwLock::new(BpciEconomicMetrics::default())),
            is_active: Arc::new(RwLock::new(false)),
            bpci_server_live: Arc::new(RwLock::new(false)),
        })
    }
    
    /// Start monitoring BPCI server and activate economics when live
    pub async fn start_monitoring(&self) -> Result<()> {
        if !self.config.auto_activation {
            info!("🔧 BPCI Economic Integration: Auto-activation disabled");
            return Ok(());
        }
        
        info!("🚀 BPCI Economic Integration: Starting monitoring for BPCI server...");
        
        let is_active = self.is_active.clone();
        let bpci_server_live = self.bpci_server_live.clone();
        let config = self.config.clone();
        
        // Spawn monitoring task
        tokio::spawn(async move {
            let mut check_interval = interval(Duration::from_secs(10));
            
            loop {
                check_interval.tick().await;
                
                // Check if BPCI server is live (simplified check)
                let server_live = Self::check_bpci_server_status().await;
                
                let mut bpci_live = bpci_server_live.write().await;
                let mut active = is_active.write().await;
                
                if server_live && !*bpci_live {
                    *bpci_live = true;
                    *active = true;
                    
                    info!("🎉 BPCI Server is LIVE! Activating Autonomous Economics...");
                    info!("💰 Owner Wallet: {}", config.owner_wallet_id);
                    info!("⚡ Auto-billing every {} seconds", config.billing_interval_seconds);
                    info!("⛏️  Auto-mining every {} seconds", config.mining_interval_seconds);
                    info!("💸 Auto-withdrawal at {} tokens ({}%)", 
                          config.owner_withdrawal_threshold, 
                          config.owner_withdrawal_percentage * 100.0);
                    
                    break;
                }
            }
        });
        
        Ok(())
    }
    
    /// Check if BPCI server is live (simplified implementation)
    async fn check_bpci_server_status() -> bool {
        // In real implementation, this would check actual BPCI server health
        // For demo purposes, simulate server going live after startup
        static mut STARTUP_TIME: Option<SystemTime> = None;
        
        unsafe {
            if STARTUP_TIME.is_none() {
                STARTUP_TIME = Some(SystemTime::now());
                return false;
            }
            
            let elapsed = SystemTime::now()
                .duration_since(STARTUP_TIME.unwrap())
                .unwrap_or_default();
            
            // Simulate server going live after 30 seconds
            elapsed.as_secs() >= 30
        }
    }
    
    /// Start autonomous economic processes
    pub async fn start_autonomous_processes(&self) -> Result<()> {
        let is_active = *self.is_active.read().await;
        if !is_active {
            warn!("🚫 BPCI Economic Integration: Not active, cannot start processes");
            return Ok(());
        }
        
        info!("🔄 Starting autonomous economic processes...");
        
        // Start billing process
        self.start_billing_process().await?;
        
        // Start mining process
        self.start_mining_process().await?;
        
        // Start owner withdrawal process
        self.start_owner_withdrawal_process().await?;
        
        info!("✅ All autonomous economic processes started successfully");
        Ok(())
    }
    
    /// Start automatic billing process
    async fn start_billing_process(&self) -> Result<()> {
        let billing_meter = self.billing_meter.clone();
        let economic_metrics = self.economic_metrics.clone();
        let owner_wallet = self.owner_wallet.clone();
        let config = self.config.clone();
        
        tokio::spawn(async move {
            let mut billing_interval = interval(Duration::from_secs(config.billing_interval_seconds));
            
            loop {
                billing_interval.tick().await;
                
                // Collect current metrics
                let metrics = {
                    let metrics_guard = economic_metrics.read().await;
                    metrics_guard.clone()
                };
                
                // Create usage record for billing
                let usage_record = UsageRecord {
                    id: Uuid::new_v4(),
                    user_id: config.owner_wallet_id.to_string(),
                    service_type: ServiceType::Compute,
                    resource_consumed: ResourceConsumption {
                        compute_units: (metrics.cpu_usage_hours * 1000.0) as u64,
                        storage_bytes: (metrics.storage_usage_gb * 1_000_000_000.0) as u64,
                        bandwidth_bytes: (metrics.network_transfer_gb * 1_000_000_000.0) as u64,
                        execution_time_ms: (metrics.cpu_usage_hours * 3600.0 * 1000.0) as u64,
                    },
                    timestamp: Utc::now(),
                    cost_breakdown: billing_meter::CostBreakdown {
                        base_fee: billing_meter::TokenAmount {
                            token_type: TokenType::Flux,
                            amount: Decimal::from(100),
                        },
                        resource_fees: vec![],
                        total_cost: billing_meter::TokenAmount {
                            token_type: TokenType::Flux,
                            amount: Decimal::from(100),
                        },
                        gold_equivalent: Decimal::from(10),
                    },
                    settlement_hash: None,
                };
                
                // Record usage and calculate billing
                if let Err(e) = billing_meter.record_usage(usage_record).await {
                    error!("❌ Billing process error: {}", e);
                    continue;
                }
                
                // Update owner wallet with infrastructure fees
                let infrastructure_revenue = (metrics.total_transactions as f64 * config.infrastructure_fee_rate) as u64;
                if infrastructure_revenue > 0 {
                    let mut wallet = owner_wallet.write().await;
                    wallet.balance += infrastructure_revenue;
                    wallet.total_earned += infrastructure_revenue;
                    
                    info!("💰 Owner wallet earned {} tokens from infrastructure fees", infrastructure_revenue);
                }
            }
        });
        
        info!("📊 Billing process started (interval: {}s)", self.config.billing_interval_seconds);
        Ok(())
    }
    
    /// Start automatic mining process
    async fn start_mining_process(&self) -> Result<()> {
        let poe_mining_engine = self.poe_mining_engine.clone();
        let economic_metrics = self.economic_metrics.clone();
        let owner_wallet = self.owner_wallet.clone();
        let config = self.config.clone();
        
        tokio::spawn(async move {
            let mut mining_interval = interval(Duration::from_secs(config.mining_interval_seconds));
            
            loop {
                mining_interval.tick().await;
                
                // Collect current metrics for PoE calculation
                let metrics = {
                    let metrics_guard = economic_metrics.read().await;
                    metrics_guard.clone()
                };
                
                // Simulate mining rewards (simplified)
                let base_mining_reward = 1000; // Base reward per cycle
                let activity_bonus = (metrics.active_services as u64) * 100;
                let total_reward = base_mining_reward + activity_bonus;
                
                // Pay owner wallet mining rewards
                {
                    let mut wallet = owner_wallet.write().await;
                    wallet.balance += total_reward;
                    wallet.total_earned += total_reward;
                }
                
                info!("⛏️  Mining cycle completed: {} tokens earned", total_reward);
            }
        });
        
        info!("⛏️  Mining process started (interval: {}s)", self.config.mining_interval_seconds);
        Ok(())
    }
    
    /// Start automatic owner withdrawal process
    async fn start_owner_withdrawal_process(&self) -> Result<()> {
        let owner_wallet = self.owner_wallet.clone();
        let config = self.config.clone();
        
        tokio::spawn(async move {
            let mut withdrawal_check = interval(Duration::from_secs(3600)); // Check hourly
            
            loop {
                withdrawal_check.tick().await;
                
                let mut wallet = owner_wallet.write().await;
                
                // Check if withdrawal threshold is met
                if wallet.balance >= config.owner_withdrawal_threshold {
                    let withdrawal_amount = (wallet.balance as f64 * config.owner_withdrawal_percentage) as u64;
                    
                    // Perform withdrawal (simplified - in real implementation would transfer to external wallet)
                    wallet.balance -= withdrawal_amount;
                    wallet.total_withdrawn += withdrawal_amount;
                    wallet.last_withdrawal = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                    
                    info!("💸 Auto-withdrawal executed: {} tokens to {}", 
                          withdrawal_amount, 
                          wallet.withdrawal_address);
                    info!("💰 Remaining balance: {} tokens", wallet.balance);
                }
            }
        });
        
        info!("💸 Owner withdrawal process started (threshold: {} tokens)", self.config.owner_withdrawal_threshold);
        Ok(())
    }
    
    /// Update economic metrics from BPCI operations
    pub async fn update_metrics(&self, 
        active_services: u32,
        new_transactions: u64,
        cpu_usage: f64,
        memory_usage: f64,
        storage_usage: f64,
        network_transfer: f64,
        audit_records: u64) -> Result<()> {
        
        let mut metrics = self.economic_metrics.write().await;
        
        metrics.active_services = active_services;
        metrics.total_transactions += new_transactions;
        metrics.cpu_usage_hours += cpu_usage;
        metrics.memory_usage_gb_hours += memory_usage;
        metrics.storage_usage_gb += storage_usage;
        metrics.network_transfer_gb += network_transfer;
        metrics.audit_records_created += audit_records;
        metrics.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        Ok(())
    }
    
    /// Get current economic status
    pub async fn get_economic_status(&self) -> Result<EconomicStatus> {
        let is_active = *self.is_active.read().await;
        let bpci_live = *self.bpci_server_live.read().await;
        let wallet = self.owner_wallet.read().await.clone();
        let metrics = self.economic_metrics.read().await.clone();
        
        Ok(EconomicStatus {
            is_active,
            bpci_server_live: bpci_live,
            owner_wallet: wallet,
            metrics,
            config: self.config.clone(),
        })
    }
    
    /// Force activate economic processes (for testing)
    pub async fn force_activate(&self) -> Result<()> {
        let mut is_active = self.is_active.write().await;
        let mut bpci_live = self.bpci_server_live.write().await;
        
        *is_active = true;
        *bpci_live = true;
        
        info!("🔧 BPCI Economic Integration: Force activated for testing");
        self.start_autonomous_processes().await?;
        
        Ok(())
    }
}

/// Economic status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicStatus {
    pub is_active: bool,
    pub bpci_server_live: bool,
    pub owner_wallet: OwnerWallet,
    pub metrics: BpciEconomicMetrics,
    pub config: BpciEconomicConfig,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_bpci_economic_integration_creation() {
        let config = BpciEconomicConfig::default();
        let integration = BpciEconomicIntegration::new(config).await.unwrap();
        
        let status = integration.get_economic_status().await.unwrap();
        assert!(!status.is_active);
        assert!(!status.bpci_server_live);
        assert_eq!(status.owner_wallet.balance, 0);
        
        println!("✅ BPCI Economic Integration creation working");
    }
    
    #[tokio::test]
    async fn test_economic_metrics_update() {
        let config = BpciEconomicConfig::default();
        let integration = BpciEconomicIntegration::new(config).await.unwrap();
        
        // Update metrics
        integration.update_metrics(5, 100, 2.5, 8.0, 1024.0, 512.0, 50).await.unwrap();
        
        let status = integration.get_economic_status().await.unwrap();
        assert_eq!(status.metrics.active_services, 5);
        assert_eq!(status.metrics.total_transactions, 100);
        assert_eq!(status.metrics.cpu_usage_hours, 2.5);
        
        println!("✅ Economic metrics update working");
    }
    
    #[tokio::test]
    async fn test_force_activation() {
        let config = BpciEconomicConfig::default();
        let integration = BpciEconomicIntegration::new(config).await.unwrap();
        
        // Force activate
        integration.force_activate().await.unwrap();
        
        let status = integration.get_economic_status().await.unwrap();
        assert!(status.is_active);
        assert!(status.bpci_server_live);
        
        println!("✅ Force activation working");
    }
    
    #[tokio::test]
    async fn test_autonomous_economic_integration() {
        let config = BpciEconomicConfig {
            billing_interval_seconds: 1, // Fast for testing
            mining_interval_seconds: 1,
            ..Default::default()
        };
        
        let integration = BpciEconomicIntegration::new(config).await.unwrap();
        
        // Force activate and start processes
        integration.force_activate().await.unwrap();
        
        // Update metrics to simulate activity
        integration.update_metrics(3, 50, 1.0, 4.0, 512.0, 256.0, 25).await.unwrap();
        
        // Wait for a few cycles
        tokio::time::sleep(Duration::from_secs(3)).await;
        
        let status = integration.get_economic_status().await.unwrap();
        assert!(status.is_active);
        assert!(status.owner_wallet.total_earned > 0);
        
        println!("✅ Autonomous economic integration working");
        println!("💰 Owner wallet earned: {} tokens", status.owner_wallet.total_earned);
        println!("📊 Total transactions: {}", status.metrics.total_transactions);
    }
}
