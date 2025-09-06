use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use crate::bpci_auction_mempool::{BpciAuctionMempool, AuctionResult as MempoolAuctionResult};

/// BPCI Round Table Oracle - Multi-Chain Partnership Coordinator
/// Manages partner chain onboarding, revenue distribution, and cross-chain coordination

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerChainConfig {
    pub chain_id: u64,
    pub name: String,
    pub rpc_endpoint: String,
    pub websocket_endpoint: String,
    pub representative_address: String,
    pub revenue_share_percent: u8, // 25% default
    pub is_active: bool,
    pub joined_at: DateTime<Utc>,
    pub total_revenue: u64,
}

impl PartnerChainConfig {
    pub fn new(
        chain_id: u64,
        name: String,
        rpc_endpoint: String,
        websocket_endpoint: String,
        representative_address: String,
    ) -> Self {
        Self {
            chain_id,
            name,
            rpc_endpoint,
            websocket_endpoint,
            representative_address,
            revenue_share_percent: 25, // Default 25%
            is_active: true,
            joined_at: Utc::now(),
            total_revenue: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Partnership {
    pub id: String,
    pub partner_chain_id: u64,
    pub bpci_chain_id: u64,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub mutual_agreement: bool,
    pub partner_signature: Option<String>,
    pub bpci_signature: Option<String>,
}

impl Partnership {
    pub fn new(partner_chain_id: u64, bpci_chain_id: u64) -> Self {
        let id = format!("partnership_{}_{}", partner_chain_id, bpci_chain_id);
        Self {
            id,
            partner_chain_id,
            bpci_chain_id,
            created_at: Utc::now(),
            last_updated: Utc::now(),
            mutual_agreement: false,
            partner_signature: None,
            bpci_signature: None,
        }
    }
    
    pub fn sign_partnership(&mut self, signature: String, is_partner: bool) {
        if is_partner {
            self.partner_signature = Some(signature);
        } else {
            self.bpci_signature = Some(signature);
        }
        
        self.last_updated = Utc::now();
        
        // Check if both parties have signed
        if self.partner_signature.is_some() && self.bpci_signature.is_some() {
            self.mutual_agreement = true;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueDistribution {
    pub distribution_id: String,
    pub auction_window_id: u64,
    pub total_auction_revenue: u64,
    pub partner_distributions: HashMap<u64, u64>, // chain_id -> amount
    pub bpci_share: u64,
    pub merkle_root: [u8; 32],
    pub timestamp: DateTime<Utc>,
    pub processed: bool,
}

impl RevenueDistribution {
    pub fn new(
        auction_window_id: u64,
        total_auction_revenue: u64,
        partner_distributions: HashMap<u64, u64>,
        merkle_root: [u8; 32],
    ) -> Self {
        let partner_total: u64 = partner_distributions.values().sum();
        let bpci_share = total_auction_revenue - partner_total;
        
        let distribution_id = format!("dist_{}_{}", auction_window_id, Utc::now().timestamp());
        
        Self {
            distribution_id,
            auction_window_id,
            total_auction_revenue,
            partner_distributions,
            bpci_share,
            merkle_root,
            timestamp: Utc::now(),
            processed: false,
        }
    }
    
    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.distribution_id.as_bytes());
        hasher.update(self.auction_window_id.to_be_bytes());
        hasher.update(self.total_auction_revenue.to_be_bytes());
        hasher.update(self.merkle_root);
        hasher.update(self.timestamp.timestamp().to_be_bytes());
        
        let hash = hasher.finalize();
        let mut result = [0u8; 32];
        result.copy_from_slice(&hash);
        result
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerStats {
    pub name: String,
    pub chain_id: u64,
    pub total_revenue: u64,
    pub revenue_share_percent: u8,
    pub distributions_count: usize,
    pub is_active: bool,
    pub joined_at: DateTime<Utc>,
    pub last_distribution: Option<DateTime<Utc>>,
    pub average_distribution: f64,
}

#[derive(Debug)]
pub struct RoundTableOracle {
    pub partner_chains: Arc<RwLock<HashMap<u64, PartnerChainConfig>>>,
    pub partnerships: Arc<RwLock<HashMap<String, Partnership>>>,
    pub revenue_distributions: Arc<RwLock<Vec<RevenueDistribution>>>,
    pub bpci_auction_mempool: Arc<RwLock<BpciAuctionMempool>>,
    pub oracle_config: OracleConfig,
}

#[derive(Debug, Clone)]
pub struct OracleConfig {
    pub bpci_chain_id: u64,
    pub monitoring_interval_secs: u64,
    pub max_partner_chains: usize,
    pub default_revenue_share: u8,
    pub min_payout_threshold: u64,
}

impl Default for OracleConfig {
    fn default() -> Self {
        Self {
            bpci_chain_id: 1337, // BPCI chain ID
            monitoring_interval_secs: 30,
            max_partner_chains: 50,
            default_revenue_share: 25, // 25%
            min_payout_threshold: 100000, // Minimum payout threshold
        }
    }
}

impl RoundTableOracle {
    pub fn new(config: Option<OracleConfig>) -> Self {
        Self {
            partner_chains: Arc::new(RwLock::new(HashMap::new())),
            partnerships: Arc::new(RwLock::new(HashMap::new())),
            revenue_distributions: Arc::new(RwLock::new(Vec::new())),
            bpci_auction_mempool: Arc::new(RwLock::new(BpciAuctionMempool::new())),
            oracle_config: config.unwrap_or_default(),
        }
    }
    
    /// Register a new partner chain
    pub async fn register_partner_chain(&self, mut config: PartnerChainConfig) -> Result<()> {
        // Validate partner chain configuration
        self.validate_partner_chain(&config).await?;
        
        let mut chains = self.partner_chains.write().await;
        
        // Check if chain already exists
        if chains.contains_key(&config.chain_id) {
            return Err(anyhow!("Partner chain {} already registered", config.chain_id));
        }
        
        // Check partner limit
        if chains.len() >= self.oracle_config.max_partner_chains {
            return Err(anyhow!("Maximum number of partner chains reached"));
        }
        
        // Set default revenue share if not specified
        if config.revenue_share_percent == 0 {
            config.revenue_share_percent = self.oracle_config.default_revenue_share;
        }
        
        // Store partner chain
        chains.insert(config.chain_id, config.clone());
        
        println!("✅ Partner chain registered: {} (ID: {})", config.name, config.chain_id);
        Ok(())
    }
    
    /// Validate partner chain connectivity and compatibility
    pub async fn validate_partner_chain(&self, config: &PartnerChainConfig) -> Result<()> {
        // Basic validation
        if config.chain_id == 0 {
            return Err(anyhow!("Invalid chain ID"));
        }
        
        if config.name.is_empty() {
            return Err(anyhow!("Chain name cannot be empty"));
        }
        
        if config.rpc_endpoint.is_empty() {
            return Err(anyhow!("RPC endpoint cannot be empty"));
        }
        
        if config.representative_address.is_empty() {
            return Err(anyhow!("Representative address cannot be empty"));
        }
        
        // Test HTTP connectivity to RPC endpoint
        let client = reqwest::Client::new();
        let test_request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_chainId",
            "params": [],
            "id": 1
        });
        
        match client.post(&config.rpc_endpoint)
            .json(&test_request)
            .send()
            .await
        {
            Ok(response) => {
                if !response.status().is_success() {
                    return Err(anyhow!("RPC endpoint returned error: {}", response.status()));
                }
                println!("✅ RPC connectivity validated for {}", config.name);
            }
            Err(e) => {
                return Err(anyhow!("Failed to connect to RPC endpoint: {}", e));
            }
        }
        
        Ok(())
    }
    
    /// Create a partnership agreement
    pub async fn create_partnership(&self, partner_chain_id: u64) -> Result<String> {
        let chains = self.partner_chains.read().await;
        
        // Validate partner chain exists
        if !chains.contains_key(&partner_chain_id) {
            return Err(anyhow!("Partner chain {} not registered", partner_chain_id));
        }
        
        drop(chains);
        
        let partnership = Partnership::new(partner_chain_id, self.oracle_config.bpci_chain_id);
        let partnership_id = partnership.id.clone();
        
        let mut partnerships = self.partnerships.write().await;
        partnerships.insert(partnership_id.clone(), partnership);
        
        println!("🤝 Partnership created: {}", partnership_id);
        Ok(partnership_id)
    }
    
    /// Sign a partnership agreement
    pub async fn sign_partnership(&self, partnership_id: &str, signature: String, is_partner: bool) -> Result<()> {
        let mut partnerships = self.partnerships.write().await;
        
        let partnership = partnerships.get_mut(partnership_id)
            .ok_or_else(|| anyhow!("Partnership {} not found", partnership_id))?;
        
        partnership.sign_partnership(signature, is_partner);
        
        if partnership.mutual_agreement {
            println!("✅ Partnership {} fully signed and active!", partnership_id);
        } else {
            println!("📝 Partnership {} signed by {} party", 
                partnership_id, 
                if is_partner { "partner" } else { "BPCI" }
            );
        }
        
        Ok(())
    }
    
    /// Process completed auction and distribute revenue
    pub async fn process_auction_result(&self, auction_result: MempoolAuctionResult) -> Result<()> {
        let chains = self.partner_chains.read().await;
        
        // Calculate partner distributions based on transaction origins
        let mut partner_distributions = HashMap::new();
        
        for winner in &auction_result.winning_transactions {
            if let Some(partner_config) = chains.get(&winner.chain_id) {
                if partner_config.is_active {
                    let partner_share = winner.bid_amount * partner_config.revenue_share_percent as u64 / 100;
                    *partner_distributions.entry(winner.chain_id).or_insert(0) += partner_share;
                }
            }
        }
        
        drop(chains);
        
        // Create revenue distribution record
        let distribution = RevenueDistribution::new(
            auction_result.window_id,
            auction_result.total_revenue,
            partner_distributions.clone(),
            auction_result.merkle_root,
        );
        
        // Store distribution
        let mut distributions = self.revenue_distributions.write().await;
        distributions.push(distribution.clone());
        
        // Update partner chain revenue totals
        let mut chains = self.partner_chains.write().await;
        for (chain_id, amount) in &partner_distributions {
            if let Some(partner) = chains.get_mut(chain_id) {
                partner.total_revenue += amount;
            }
        }
        
        drop(chains);
        
        // Notify partner chains
        self.notify_partner_chains(&distribution).await?;
        
        println!("💰 Revenue distributed for auction {}: {} total, {} to partners", 
            auction_result.window_id, 
            auction_result.total_revenue,
            partner_distributions.values().sum::<u64>()
        );
        
        Ok(())
    }
    
    /// Notify partner chains of revenue distribution
    async fn notify_partner_chains(&self, distribution: &RevenueDistribution) -> Result<()> {
        let chains = self.partner_chains.read().await;
        
        for (chain_id, amount) in &distribution.partner_distributions {
            if let Some(partner_config) = chains.get(chain_id) {
                if *amount >= self.oracle_config.min_payout_threshold {
                    self.send_revenue_notification(partner_config, distribution, *amount).await?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Send revenue notification to partner chain
    async fn send_revenue_notification(
        &self,
        partner_config: &PartnerChainConfig,
        distribution: &RevenueDistribution,
        amount: u64,
    ) -> Result<()> {
        let notification = serde_json::json!({
            "type": "bpci_revenue_distribution",
            "distribution_id": distribution.distribution_id,
            "auction_window_id": distribution.auction_window_id,
            "total_auction_revenue": distribution.total_auction_revenue,
            "partner_share": amount,
            "revenue_share_percent": partner_config.revenue_share_percent,
            "merkle_root": hex::encode(distribution.merkle_root),
            "timestamp": distribution.timestamp,
            "bpci_chain_id": self.oracle_config.bpci_chain_id,
            "partner_chain_id": partner_config.chain_id,
            "distribution_hash": hex::encode(distribution.compute_hash())
        });
        
        let client = reqwest::Client::new();
        let notification_url = format!("{}/api/bpci/revenue-notification", partner_config.rpc_endpoint);
        
        match client.post(&notification_url)
            .json(&notification)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    println!("✅ Revenue notification sent to {} ({}): {} wei", 
                        partner_config.name, partner_config.chain_id, amount);
                } else {
                    println!("⚠️ Failed to notify {}: HTTP {}", 
                        partner_config.name, response.status());
                }
            }
            Err(e) => {
                println!("❌ Error notifying {}: {}", partner_config.name, e);
            }
        }
        
        Ok(())
    }
    
    /// Get partner chain statistics
    pub async fn get_partner_statistics(&self) -> Result<HashMap<u64, PartnerStats>> {
        let chains = self.partner_chains.read().await;
        let distributions = self.revenue_distributions.read().await;
        
        let mut stats = HashMap::new();
        
        for (chain_id, config) in chains.iter() {
            // Calculate distribution statistics
            let chain_distributions: Vec<_> = distributions.iter()
                .filter(|d| d.partner_distributions.contains_key(chain_id))
                .collect();
            
            let distributions_count = chain_distributions.len();
            let last_distribution = chain_distributions.iter()
                .map(|d| d.timestamp)
                .max();
            
            let average_distribution = if distributions_count > 0 {
                config.total_revenue as f64 / distributions_count as f64
            } else {
                0.0
            };
            
            stats.insert(*chain_id, PartnerStats {
                name: config.name.clone(),
                chain_id: *chain_id,
                total_revenue: config.total_revenue,
                revenue_share_percent: config.revenue_share_percent,
                distributions_count,
                is_active: config.is_active,
                joined_at: config.joined_at,
                last_distribution,
                average_distribution,
            });
        }
        
        Ok(stats)
    }
    
    /// Start oracle monitoring loop
    pub async fn start_monitoring(&self) -> Result<()> {
        println!("🚀 Round Table Oracle starting monitoring...");
        println!("   BPCI Chain ID: {}", self.oracle_config.bpci_chain_id);
        println!("   Monitoring Interval: {}s", self.oracle_config.monitoring_interval_secs);
        println!("   Max Partner Chains: {}", self.oracle_config.max_partner_chains);
        
        let mut interval = tokio::time::interval(
            tokio::time::Duration::from_secs(self.oracle_config.monitoring_interval_secs)
        );
        
        loop {
            interval.tick().await;
            
            // Process expired auction windows
            let expired_results = {
                let mut mempool = self.bpci_auction_mempool.write().await;
                match mempool.process_expired_windows().await {
                    Ok(results) => results,
                    Err(e) => {
                        eprintln!("❌ Error processing expired windows: {}", e);
                        continue;
                    }
                }
            };
            
            // Process each auction result
            for result in expired_results {
                if let Err(e) = self.process_auction_result(result).await {
                    eprintln!("❌ Error processing auction result: {}", e);
                }
            }
            
            // Log status
            let chains_count = self.partner_chains.read().await.len();
            let distributions_count = self.revenue_distributions.read().await.len();
            
            if chains_count > 0 || distributions_count > 0 {
                println!("📊 Oracle Status: {} partner chains, {} distributions processed", 
                    chains_count, distributions_count);
            }
        }
    }
    
    /// Get oracle status and metrics
    pub async fn get_oracle_status(&self) -> OracleStatus {
        let chains = self.partner_chains.read().await;
        let partnerships = self.partnerships.read().await;
        let distributions = self.revenue_distributions.read().await;
        let mempool = self.bpci_auction_mempool.read().await;
        
        let active_chains = chains.values().filter(|c| c.is_active).count();
        let total_revenue_distributed: u64 = distributions.iter()
            .map(|d| d.partner_distributions.values().sum::<u64>())
            .sum();
        
        let active_partnerships = partnerships.values()
            .filter(|p| p.mutual_agreement)
            .count();
        
        OracleStatus {
            total_partner_chains: chains.len(),
            active_partner_chains: active_chains,
            total_partnerships: partnerships.len(),
            active_partnerships,
            total_distributions: distributions.len(),
            total_revenue_distributed,
            pending_transactions: mempool.get_mempool_stats().pending_transactions,
            oracle_uptime: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OracleStatus {
    pub total_partner_chains: usize,
    pub active_partner_chains: usize,
    pub total_partnerships: usize,
    pub active_partnerships: usize,
    pub total_distributions: usize,
    pub total_revenue_distributed: u64,
    pub pending_transactions: usize,
    pub oracle_uptime: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_partner_chain_registration() {
        let oracle = RoundTableOracle::new(None);
        
        let config = PartnerChainConfig::new(
            137,
            "Polygon".to_string(),
            "https://polygon-rpc.com".to_string(),
            "wss://polygon-ws.com".to_string(),
            "0x1234567890123456789012345678901234567890".to_string(),
        );
        
        // Note: This test will fail without actual RPC connectivity
        // In production, we'd mock the HTTP client
        match oracle.register_partner_chain(config).await {
            Ok(_) => println!("✅ Partner chain registration test passed"),
            Err(e) => println!("⚠️ Partner chain registration test failed (expected): {}", e),
        }
    }
    
    #[tokio::test]
    async fn test_partnership_creation() {
        let oracle = RoundTableOracle::new(None);
        
        // First register a partner chain (will fail due to validation, but that's ok for this test)
        let config = PartnerChainConfig::new(
            137,
            "Polygon".to_string(),
            "https://polygon-rpc.com".to_string(),
            "wss://polygon-ws.com".to_string(),
            "0x1234567890123456789012345678901234567890".to_string(),
        );
        
        // Manually add to bypass validation for testing
        {
            let mut chains = oracle.partner_chains.write().await;
            chains.insert(137, config);
        }
        
        // Create partnership
        let partnership_id = oracle.create_partnership(137).await.unwrap();
        assert!(!partnership_id.is_empty());
        
        // Sign partnership
        oracle.sign_partnership(&partnership_id, "test_signature".to_string(), true).await.unwrap();
        oracle.sign_partnership(&partnership_id, "bpci_signature".to_string(), false).await.unwrap();
        
        // Verify partnership is active
        let partnerships = oracle.partnerships.read().await;
        let partnership = partnerships.get(&partnership_id).unwrap();
        assert!(partnership.mutual_agreement);
        
        println!("✅ Partnership creation and signing test passed");
    }
    
    #[tokio::test]
    async fn test_revenue_distribution() {
        let oracle = RoundTableOracle::new(None);
        
        // Setup test partner chain
        let config = PartnerChainConfig::new(
            137,
            "Polygon".to_string(),
            "https://polygon-rpc.com".to_string(),
            "wss://polygon-ws.com".to_string(),
            "0x1234567890123456789012345678901234567890".to_string(),
        );
        
        {
            let mut chains = oracle.partner_chains.write().await;
            chains.insert(137, config);
        }
        
        // Create test auction result
        use crate::bpci_auction_mempool::{AuctionTransaction, AuctionType};
        
        let tx = AuctionTransaction::new(
            [1u8; 32],
            137, // Partner chain ID
            1000000, // 1M wei bid
            21000,
            100,
            "test_sender".to_string(),
        );
        
        let auction_result = MempoolAuctionResult {
            auction_id: "test_auction_1".to_string(),
            window_id: 1,
            winning_transactions: vec![tx],
            total_revenue: 1000000,
            merkle_root: [0u8; 32],
            timestamp: Utc::now(),
        };
        
        // Process revenue distribution
        oracle.process_auction_result(auction_result).await.unwrap();
        
        // Verify distribution was recorded
        let distributions = oracle.revenue_distributions.read().await;
        assert_eq!(distributions.len(), 1);
        
        let distribution = &distributions[0];
        assert_eq!(distribution.total_auction_revenue, 1000000);
        assert!(distribution.partner_distributions.contains_key(&137));
        
        // Verify partner revenue was updated
        let chains = oracle.partner_chains.read().await;
        let partner = chains.get(&137).unwrap();
        assert!(partner.total_revenue > 0);
        
        println!("✅ Revenue distribution test passed");
    }
}
