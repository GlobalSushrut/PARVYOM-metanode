use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use sha2::{Sha256, Digest};

/// Bundle auction status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuctionStatus {
    Pending,
    Active,
    Bidding,
    Finalizing,
    Completed,
    Cancelled,
    Failed,
}

/// Bid status in auction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BidStatus {
    Submitted,
    Valid,
    Invalid,
    Winning,
    Outbid,
    Executed,
    Refunded,
}

/// Bundle type for auction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BundleType {
    Transaction,
    SmartContract,
    DataStorage,
    Computation,
    Validation,
    Governance,
}

/// Bundle auction configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionConfig {
    pub auction_duration_minutes: u64,
    pub min_bid_amount: u64,
    pub bid_increment: u64,
    pub reserve_price: Option<u64>,
    pub max_bidders: usize,
    pub auto_extend_threshold_minutes: u64,
    pub auto_extend_duration_minutes: u64,
    pub settlement_delay_minutes: u64,
    pub fee_percentage: f64,
    pub slashing_percentage: f64,
}

impl Default for AuctionConfig {
    fn default() -> Self {
        Self {
            auction_duration_minutes: 60,
            min_bid_amount: 1000,
            bid_increment: 100,
            reserve_price: None,
            max_bidders: 100,
            auto_extend_threshold_minutes: 5,
            auto_extend_duration_minutes: 10,
            settlement_delay_minutes: 15,
            fee_percentage: 2.5,
            slashing_percentage: 10.0,
        }
    }
}

/// Bundle information for auction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleInfo {
    pub bundle_id: String,
    pub bundle_type: BundleType,
    pub data_hash: String,
    pub size_bytes: u64,
    pub complexity_score: f64,
    pub priority_level: u8,
    pub expiry_time: DateTime<Utc>,
    pub creator_id: String,
    pub metadata: HashMap<String, String>,
    pub integrity_proof: String,
}

/// Bid information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidInfo {
    pub bid_id: String,
    pub auction_id: String,
    pub bidder_id: String,
    pub bid_amount: u64,
    pub bid_time: DateTime<Utc>,
    pub status: BidStatus,
    pub collateral_amount: u64,
    pub execution_guarantee: bool,
    pub performance_bond: u64,
    pub validator_signature: String,
    pub bid_hash: String,
}

/// Auction information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionInfo {
    pub auction_id: String,
    pub bundle: BundleInfo,
    pub status: AuctionStatus,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub current_highest_bid: Option<u64>,
    pub winning_bidder: Option<String>,
    pub total_bids: usize,
    pub participating_validators: Vec<String>,
    pub settlement_time: Option<DateTime<Utc>>,
    pub final_price: Option<u64>,
    pub auction_fees: u64,
    pub config: AuctionConfig,
}

/// Auction settlement result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementResult {
    pub auction_id: String,
    pub winning_bid: Option<BidInfo>,
    pub final_price: u64,
    pub total_fees: u64,
    pub validator_rewards: HashMap<String, u64>,
    pub settlement_time: DateTime<Utc>,
    pub execution_status: ExecutionStatus,
    pub refunds: Vec<RefundInfo>,
}

/// Execution status after settlement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Disputed,
}

/// Refund information for unsuccessful bids
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundInfo {
    pub bidder_id: String,
    pub refund_amount: u64,
    pub refund_time: DateTime<Utc>,
    pub transaction_hash: String,
}

/// Auction metrics and analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionMetrics {
    pub total_auctions: usize,
    pub active_auctions: usize,
    pub completed_auctions: usize,
    pub total_volume: u64,
    pub average_bid_amount: f64,
    pub average_auction_duration: f64,
    pub success_rate: f64,
    pub top_bidders: Vec<(String, u64)>,
    pub bundle_type_distribution: HashMap<BundleType, usize>,
    pub revenue_generated: u64,
    pub fees_collected: u64,
}

/// BPCI Bundle Auction System
pub struct BpciBundleAuctionSystem {
    auctions: Arc<RwLock<HashMap<String, AuctionInfo>>>,
    bids: Arc<RwLock<HashMap<String, Vec<BidInfo>>>>,
    settlements: Arc<RwLock<HashMap<String, SettlementResult>>>,
    active_auctions: Arc<RwLock<BTreeMap<DateTime<Utc>, String>>>,
    bidder_balances: Arc<RwLock<HashMap<String, u64>>>,
    validator_stakes: Arc<RwLock<HashMap<String, u64>>>,
    auction_history: Arc<RwLock<VecDeque<AuctionInfo>>>,
    config: AuctionConfig,
    system_enabled: Arc<RwLock<bool>>,
    emergency_mode: Arc<RwLock<bool>>,
}

impl BpciBundleAuctionSystem {
    /// Create new BPCI bundle auction system
    pub fn new(config: Option<AuctionConfig>) -> Self {
        Self {
            auctions: Arc::new(RwLock::new(HashMap::new())),
            bids: Arc::new(RwLock::new(HashMap::new())),
            settlements: Arc::new(RwLock::new(HashMap::new())),
            active_auctions: Arc::new(RwLock::new(BTreeMap::new())),
            bidder_balances: Arc::new(RwLock::new(HashMap::new())),
            validator_stakes: Arc::new(RwLock::new(HashMap::new())),
            auction_history: Arc::new(RwLock::new(VecDeque::new())),
            config: config.unwrap_or_default(),
            system_enabled: Arc::new(RwLock::new(true)),
            emergency_mode: Arc::new(RwLock::new(false)),
        }
    }

    /// Create new bundle auction
    pub async fn create_auction(&self, bundle: BundleInfo, config: Option<AuctionConfig>) -> Result<String> {
        let system_enabled = *self.system_enabled.read().await;
        if !system_enabled {
            return Err(anyhow!("Auction system is disabled"));
        }

        let auction_config = config.unwrap_or_else(|| self.config.clone());
        let auction_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let end_time = now + Duration::minutes(auction_config.auction_duration_minutes as i64);

        // Validate bundle integrity
        self.validate_bundle_integrity(&bundle).await?;

        let auction_info = AuctionInfo {
            auction_id: auction_id.clone(),
            bundle,
            status: AuctionStatus::Pending,
            start_time: now,
            end_time,
            current_highest_bid: None,
            winning_bidder: None,
            total_bids: 0,
            participating_validators: Vec::new(),
            settlement_time: None,
            final_price: None,
            auction_fees: 0,
            config: auction_config,
        };

        // Store auction
        let mut auctions = self.auctions.write().await;
        auctions.insert(auction_id.clone(), auction_info.clone());

        // Add to active auctions timeline
        let mut active_auctions = self.active_auctions.write().await;
        active_auctions.insert(end_time, auction_id.clone());

        // Initialize bid list for this auction
        let mut bids = self.bids.write().await;
        bids.insert(auction_id.clone(), Vec::new());

        info!("Created auction {} for bundle {}", auction_id, auction_info.bundle.bundle_id);
        Ok(auction_id)
    }

    /// Submit bid to auction
    pub async fn submit_bid(
        &self,
        auction_id: &str,
        bidder_id: String,
        bid_amount: u64,
        collateral_amount: u64,
        validator_signature: String,
    ) -> Result<String> {
        let emergency_mode = *self.emergency_mode.read().await;
        if emergency_mode {
            return Err(anyhow!("Auction system in emergency mode"));
        }

        // Validate auction exists and is active
        let mut auctions = self.auctions.write().await;
        let auction = auctions.get_mut(auction_id)
            .ok_or_else(|| anyhow!("Auction {} not found", auction_id))?;

        if auction.status != AuctionStatus::Active && auction.status != AuctionStatus::Bidding {
            return Err(anyhow!("Auction {} is not accepting bids", auction_id));
        }

        // Check auction hasn't expired
        let now = Utc::now();
        if now > auction.end_time {
            auction.status = AuctionStatus::Finalizing;
            return Err(anyhow!("Auction {} has expired", auction_id));
        }

        // Validate bid amount
        let min_bid = auction.current_highest_bid
            .map(|current| current + auction.config.bid_increment)
            .unwrap_or(auction.config.min_bid_amount);

        if bid_amount < min_bid {
            return Err(anyhow!("Bid amount {} below minimum {}", bid_amount, min_bid));
        }

        // Check bidder balance
        let bidder_balances = self.bidder_balances.read().await;
        let available_balance = bidder_balances.get(&bidder_id).copied().unwrap_or(0);
        if available_balance < bid_amount + collateral_amount {
            return Err(anyhow!("Insufficient balance for bid"));
        }
        drop(bidder_balances);

        // Create bid
        let bid_id = Uuid::new_v4().to_string();
        let bid_data = format!("{}:{}:{}:{}", auction_id, bidder_id, bid_amount, now.timestamp());
        let bid_hash = format!("{:x}", Sha256::digest(bid_data.as_bytes()));

        let bid_info = BidInfo {
            bid_id: bid_id.clone(),
            auction_id: auction_id.to_string(),
            bidder_id: bidder_id.clone(),
            bid_amount,
            bid_time: now,
            status: BidStatus::Submitted,
            collateral_amount,
            execution_guarantee: true,
            performance_bond: collateral_amount,
            validator_signature,
            bid_hash,
        };

        // Update auction with new highest bid
        if auction.current_highest_bid.is_none() || bid_amount > auction.current_highest_bid.unwrap() {
            // Mark previous winning bidder as outbid
            if let Some(previous_winner) = &auction.winning_bidder {
                self.update_previous_bids_status(auction_id, previous_winner, BidStatus::Outbid).await?;
            }

            auction.current_highest_bid = Some(bid_amount);
            auction.winning_bidder = Some(bidder_id.clone());
            auction.status = AuctionStatus::Bidding;
        }

        auction.total_bids += 1;

        // Auto-extend auction if bid is close to end time
        if now > auction.end_time - Duration::minutes(auction.config.auto_extend_threshold_minutes as i64) {
            auction.end_time = auction.end_time + Duration::minutes(auction.config.auto_extend_duration_minutes as i64);
            info!("Auto-extended auction {} due to late bid", auction_id);
        }

        // Store bid
        let mut bids = self.bids.write().await;
        if let Some(auction_bids) = bids.get_mut(auction_id) {
            auction_bids.push(bid_info);
        }

        // Reserve bidder funds
        let mut bidder_balances = self.bidder_balances.write().await;
        if let Some(balance) = bidder_balances.get_mut(&bidder_id) {
            *balance -= bid_amount + collateral_amount;
        }

        info!("Submitted bid {} for auction {} by bidder {}", bid_id, auction_id, bidder_id);
        Ok(bid_id)
    }

    /// Finalize auction and determine winner
    pub async fn finalize_auction(&self, auction_id: &str) -> Result<SettlementResult> {
        let mut auctions = self.auctions.write().await;
        let auction = auctions.get_mut(auction_id)
            .ok_or_else(|| anyhow!("Auction {} not found", auction_id))?;

        let now = Utc::now();
        if now < auction.end_time && auction.status != AuctionStatus::Finalizing {
            return Err(anyhow!("Auction {} not ready for finalization", auction_id));
        }

        auction.status = AuctionStatus::Finalizing;

        // Get all bids for this auction
        let bids = self.bids.read().await;
        let auction_bids = bids.get(auction_id).cloned().unwrap_or_default();
        drop(bids);

        // Determine winning bid
        let winning_bid = auction_bids.iter()
            .filter(|bid| bid.status == BidStatus::Submitted || bid.status == BidStatus::Valid)
            .max_by_key(|bid| bid.bid_amount)
            .cloned();

        let (final_price, total_fees, execution_status) = if let Some(ref winner) = winning_bid {
            let price = winner.bid_amount;
            let fees = (price as f64 * self.config.fee_percentage / 100.0) as u64;
            auction.final_price = Some(price);
            auction.auction_fees = fees;
            (price, fees, ExecutionStatus::Pending)
        } else {
            auction.status = AuctionStatus::Failed;
            (0, 0, ExecutionStatus::Failed)
        };

        // Calculate validator rewards (distribute fees among participating validators)
        let mut validator_rewards = HashMap::new();
        if total_fees > 0 && !auction.participating_validators.is_empty() {
            let reward_per_validator = total_fees / auction.participating_validators.len() as u64;
            for validator_id in &auction.participating_validators {
                validator_rewards.insert(validator_id.clone(), reward_per_validator);
            }
        }

        // Process refunds for unsuccessful bidders
        let mut refunds = Vec::new();
        for bid in &auction_bids {
            if Some(&bid.bidder_id) != winning_bid.as_ref().map(|w| &w.bidder_id) {
                let refund = RefundInfo {
                    bidder_id: bid.bidder_id.clone(),
                    refund_amount: bid.bid_amount + bid.collateral_amount,
                    refund_time: now,
                    transaction_hash: format!("{:x}", Sha256::digest(format!("refund:{}:{}", bid.bid_id, now.timestamp()).as_bytes())),
                };
                refunds.push(refund);

                // Process actual refund
                self.process_refund(&bid.bidder_id, bid.bid_amount + bid.collateral_amount).await?;
            }
        }

        let settlement_result = SettlementResult {
            auction_id: auction_id.to_string(),
            winning_bid: winning_bid.clone(),
            final_price,
            total_fees,
            validator_rewards,
            settlement_time: now,
            execution_status,
            refunds,
        };

        // Store settlement
        let mut settlements = self.settlements.write().await;
        settlements.insert(auction_id.to_string(), settlement_result.clone());

        // Update auction status
        auction.status = if winning_bid.is_some() {
            AuctionStatus::Completed
        } else {
            AuctionStatus::Failed
        };
        auction.settlement_time = Some(now);

        // Move to history
        let mut auction_history = self.auction_history.write().await;
        auction_history.push_back(auction.clone());
        if auction_history.len() > 10000 {
            auction_history.pop_front();
        }

        // Remove from active auctions
        let mut active_auctions = self.active_auctions.write().await;
        active_auctions.retain(|_, id| id != auction_id);

        info!("Finalized auction {} with final price {}", auction_id, final_price);
        Ok(settlement_result)
    }

    /// Process automatic auction finalization for expired auctions
    pub async fn process_expired_auctions(&self) -> Result<Vec<String>> {
        let now = Utc::now();
        let mut finalized_auctions = Vec::new();

        // Find expired auctions
        let active_auctions = self.active_auctions.read().await;
        let expired_auction_ids: Vec<String> = active_auctions
            .range(..=now)
            .map(|(_, auction_id)| auction_id.clone())
            .collect();
        drop(active_auctions);

        // Finalize each expired auction
        for auction_id in expired_auction_ids {
            match self.finalize_auction(&auction_id).await {
                Ok(_) => {
                    finalized_auctions.push(auction_id);
                }
                Err(e) => {
                    error!("Failed to finalize expired auction {}: {}", auction_id, e);
                }
            }
        }

        if !finalized_auctions.is_empty() {
            info!("Finalized {} expired auctions", finalized_auctions.len());
        }

        Ok(finalized_auctions)
    }

    /// Get auction metrics and analytics
    pub async fn get_auction_metrics(&self) -> Result<AuctionMetrics> {
        let auctions = self.auctions.read().await;
        let auction_history = self.auction_history.read().await;
        let settlements = self.settlements.read().await;

        let total_auctions = auctions.len() + auction_history.len();
        let active_auctions = auctions.values()
            .filter(|a| a.status == AuctionStatus::Active || a.status == AuctionStatus::Bidding)
            .count();
        let completed_auctions = auction_history.len() + auctions.values()
            .filter(|a| a.status == AuctionStatus::Completed)
            .count();

        let total_volume: u64 = settlements.values()
            .map(|s| s.final_price)
            .sum();

        let average_bid_amount = if completed_auctions > 0 {
            total_volume as f64 / completed_auctions as f64
        } else {
            0.0
        };

        let success_rate = if total_auctions > 0 {
            completed_auctions as f64 / total_auctions as f64
        } else {
            0.0
        };

        // Calculate average auction duration
        let total_duration: i64 = auction_history.iter()
            .filter_map(|a| a.settlement_time.map(|st| st.signed_duration_since(a.start_time).num_minutes()))
            .sum();
        let average_auction_duration = if auction_history.len() > 0 {
            total_duration as f64 / auction_history.len() as f64
        } else {
            0.0
        };

        // Calculate bundle type distribution
        let mut bundle_type_distribution = HashMap::new();
        for auction in auctions.values().chain(auction_history.iter()) {
            *bundle_type_distribution.entry(auction.bundle.bundle_type.clone()).or_insert(0) += 1;
        }

        let fees_collected: u64 = settlements.values()
            .map(|s| s.total_fees)
            .sum();

        let revenue_generated = total_volume;

        // Top bidders (simplified - would need bid analysis in real implementation)
        let top_bidders = Vec::new();

        Ok(AuctionMetrics {
            total_auctions,
            active_auctions,
            completed_auctions,
            total_volume,
            average_bid_amount,
            average_auction_duration,
            success_rate,
            top_bidders,
            bundle_type_distribution,
            revenue_generated,
            fees_collected,
        })
    }

    /// Validate bundle integrity
    async fn validate_bundle_integrity(&self, bundle: &BundleInfo) -> Result<()> {
        // Verify integrity proof
        let bundle_data = format!("{}:{}:{}:{}", 
            bundle.bundle_id, bundle.data_hash, bundle.size_bytes, bundle.complexity_score);
        let expected_proof = format!("{:x}", Sha256::digest(bundle_data.as_bytes()));
        
        if bundle.integrity_proof != expected_proof {
            return Err(anyhow!("Bundle integrity proof validation failed"));
        }

        // Check expiry
        if bundle.expiry_time < Utc::now() {
            return Err(anyhow!("Bundle has expired"));
        }

        Ok(())
    }

    /// Update status of previous bids when outbid
    async fn update_previous_bids_status(&self, auction_id: &str, bidder_id: &str, new_status: BidStatus) -> Result<()> {
        let mut bids = self.bids.write().await;
        if let Some(auction_bids) = bids.get_mut(auction_id) {
            for bid in auction_bids.iter_mut() {
                if bid.bidder_id == bidder_id && bid.status == BidStatus::Submitted {
                    bid.status = new_status.clone();
                }
            }
        }
        Ok(())
    }

    /// Process refund to bidder
    async fn process_refund(&self, bidder_id: &str, amount: u64) -> Result<()> {
        let mut bidder_balances = self.bidder_balances.write().await;
        let balance = bidder_balances.entry(bidder_id.to_string()).or_insert(0);
        *balance += amount;
        Ok(())
    }

    /// Set bidder balance (for testing/initialization)
    pub async fn set_bidder_balance(&self, bidder_id: String, balance: u64) -> Result<()> {
        let mut bidder_balances = self.bidder_balances.write().await;
        bidder_balances.insert(bidder_id, balance);
        Ok(())
    }

    /// Get auction information
    pub async fn get_auction(&self, auction_id: &str) -> Result<Option<AuctionInfo>> {
        let auctions = self.auctions.read().await;
        Ok(auctions.get(auction_id).cloned())
    }

    /// Get all active auctions
    pub async fn get_active_auctions(&self) -> Result<Vec<AuctionInfo>> {
        let auctions = self.auctions.read().await;
        let active: Vec<AuctionInfo> = auctions.values()
            .filter(|a| a.status == AuctionStatus::Active || a.status == AuctionStatus::Bidding)
            .cloned()
            .collect();
        Ok(active)
    }

    /// Enable/disable auction system
    pub async fn set_system_enabled(&self, enabled: bool) -> Result<()> {
        let mut system_enabled = self.system_enabled.write().await;
        *system_enabled = enabled;
        info!("Auction system {}", if enabled { "enabled" } else { "disabled" });
        Ok(())
    }

    /// Set emergency mode
    pub async fn set_emergency_mode(&self, emergency: bool) -> Result<()> {
        let mut emergency_mode = self.emergency_mode.write().await;
        *emergency_mode = emergency;
        warn!("Auction system emergency mode {}", if emergency { "activated" } else { "deactivated" });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_bundle() -> BundleInfo {
        let bundle_data = "test_bundle:hash123:1000:0.5";
        let integrity_proof = format!("{:x}", Sha256::digest(bundle_data.as_bytes()));
        
        BundleInfo {
            bundle_id: "test_bundle".to_string(),
            bundle_type: BundleType::Transaction,
            data_hash: "hash123".to_string(),
            size_bytes: 1000,
            complexity_score: 0.5,
            priority_level: 1,
            expiry_time: Utc::now() + Duration::hours(24),
            creator_id: "creator1".to_string(),
            metadata: HashMap::new(),
            integrity_proof,
        }
    }

    #[tokio::test]
    async fn test_auction_creation() {
        let auction_system = BpciBundleAuctionSystem::new(None);
        let bundle = create_test_bundle();
        
        let auction_id = auction_system.create_auction(bundle, None).await.unwrap();
        assert!(!auction_id.is_empty());
        
        let auction = auction_system.get_auction(&auction_id).await.unwrap();
        assert!(auction.is_some());
        assert_eq!(auction.unwrap().status, AuctionStatus::Pending);
    }

    #[tokio::test]
    async fn test_bid_submission() {
        let auction_system = BpciBundleAuctionSystem::new(None);
        let bundle = create_test_bundle();
        
        let auction_id = auction_system.create_auction(bundle, None).await.unwrap();
        
        // Set bidder balance
        auction_system.set_bidder_balance("bidder1".to_string(), 10000).await.unwrap();
        
        // Activate auction manually for test
        {
            let mut auctions = auction_system.auctions.write().await;
            if let Some(auction) = auctions.get_mut(&auction_id) {
                auction.status = AuctionStatus::Active;
            }
        }
        
        let bid_id = auction_system.submit_bid(
            &auction_id,
            "bidder1".to_string(),
            2000,
            500,
            "signature123".to_string(),
        ).await.unwrap();
        
        assert!(!bid_id.is_empty());
        
        let auction = auction_system.get_auction(&auction_id).await.unwrap().unwrap();
        assert_eq!(auction.current_highest_bid, Some(2000));
        assert_eq!(auction.winning_bidder, Some("bidder1".to_string()));
    }

    #[tokio::test]
    async fn test_auction_metrics() {
        let auction_system = BpciBundleAuctionSystem::new(None);
        
        // Create multiple auctions
        for i in 0..3 {
            let mut bundle = create_test_bundle();
            bundle.bundle_id = format!("bundle_{}", i);
            auction_system.create_auction(bundle, None).await.unwrap();
        }
        
        let metrics = auction_system.get_auction_metrics().await.unwrap();
        assert_eq!(metrics.total_auctions, 3);
        assert!(metrics.bundle_type_distribution.contains_key(&BundleType::Transaction));
    }
}
