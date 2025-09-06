# BPCI Round Table Partnership Framework
## Automated Partner Chain Onboarding & Revenue Sharing System

### Overview
The Round Table Partnership Framework is BPCI's automated system for onboarding partner chains, managing two-way agreements, and distributing auction revenue. It enables seamless multi-chain coordination with transparent governance and automated revenue sharing.

### Core Architecture

#### 1. Round Table Smart Contract
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract RoundTablePartnership is AccessControl, ReentrancyGuard {
    bytes32 public constant ORACLE_ROLE = keccak256("ORACLE_ROLE");
    bytes32 public constant PARTNER_ROLE = keccak256("PARTNER_ROLE");
    
    struct PartnerChain {
        uint256 chainId;
        string name;
        address representative;
        uint256 joinedAt;
        uint256 totalRevenue;
        uint256 revenueShare; // Basis points (2500 = 25%)
        bool isActive;
        string rpcEndpoint;
        string websocketEndpoint;
    }
    
    struct RevenueDistribution {
        uint256 totalAmount;
        uint256 partnerShare;
        uint256 bpciShare;
        uint256 timestamp;
        bytes32 auctionRoot; // Merkle root of auction results
    }
    
    struct Partnership {
        uint256 partnerChainId;
        uint256 bpciChainId;
        bool mutualAgreement;
        uint256 createdAt;
        uint256 lastUpdated;
        mapping(address => bool) signatures;
    }
    
    // State variables
    mapping(uint256 => PartnerChain) public partnerChains;
    mapping(bytes32 => Partnership) public partnerships;
    mapping(uint256 => RevenueDistribution[]) public revenueHistory;
    
    uint256[] public activeChainIds;
    uint256 public totalPartnerships;
    uint256 public totalRevenueDistributed;
    
    // Events
    event PartnerChainRegistered(uint256 indexed chainId, string name, address representative);
    event PartnershipCreated(bytes32 indexed partnershipId, uint256 partnerChainId, uint256 bpciChainId);
    event RevenueDistributed(uint256 indexed chainId, uint256 amount, bytes32 auctionRoot);
    event PartnershipUpdated(bytes32 indexed partnershipId, bool isActive);
    
    constructor() {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(ORACLE_ROLE, msg.sender);
    }
    
    /**
     * @dev Register a new partner chain
     */
    function registerPartnerChain(
        uint256 _chainId,
        string memory _name,
        address _representative,
        string memory _rpcEndpoint,
        string memory _websocketEndpoint
    ) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(_chainId != 0, "Invalid chain ID");
        require(_representative != address(0), "Invalid representative");
        require(!partnerChains[_chainId].isActive, "Chain already registered");
        
        partnerChains[_chainId] = PartnerChain({
            chainId: _chainId,
            name: _name,
            representative: _representative,
            joinedAt: block.timestamp,
            totalRevenue: 0,
            revenueShare: 2500, // 25% default
            isActive: true,
            rpcEndpoint: _rpcEndpoint,
            websocketEndpoint: _websocketEndpoint
        });
        
        activeChainIds.push(_chainId);
        _grantRole(PARTNER_ROLE, _representative);
        
        emit PartnerChainRegistered(_chainId, _name, _representative);
    }
    
    /**
     * @dev Create a two-way partnership agreement
     */
    function createPartnership(
        uint256 _partnerChainId,
        uint256 _bpciChainId
    ) external returns (bytes32) {
        require(partnerChains[_partnerChainId].isActive, "Partner chain not active");
        require(hasRole(PARTNER_ROLE, msg.sender), "Not authorized partner");
        
        bytes32 partnershipId = keccak256(abi.encodePacked(_partnerChainId, _bpciChainId, block.timestamp));
        
        partnerships[partnershipId].partnerChainId = _partnerChainId;
        partnerships[partnershipId].bpciChainId = _bpciChainId;
        partnerships[partnershipId].createdAt = block.timestamp;
        partnerships[partnershipId].lastUpdated = block.timestamp;
        partnerships[partnershipId].signatures[msg.sender] = true;
        
        totalPartnerships++;
        
        emit PartnershipCreated(partnershipId, _partnerChainId, _bpciChainId);
        return partnershipId;
    }
    
    /**
     * @dev Sign partnership agreement (requires both parties)
     */
    function signPartnership(bytes32 _partnershipId) external {
        Partnership storage partnership = partnerships[_partnershipId];
        require(partnership.createdAt > 0, "Partnership does not exist");
        require(hasRole(PARTNER_ROLE, msg.sender) || hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "Not authorized");
        
        partnership.signatures[msg.sender] = true;
        partnership.lastUpdated = block.timestamp;
        
        // Check if both parties have signed
        PartnerChain storage partner = partnerChains[partnership.partnerChainId];
        if (partnership.signatures[partner.representative] && 
            partnership.signatures[msg.sender]) {
            partnership.mutualAgreement = true;
        }
        
        emit PartnershipUpdated(_partnershipId, partnership.mutualAgreement);
    }
    
    /**
     * @dev Distribute auction revenue to partner chains
     */
    function distributeRevenue(
        uint256[] memory _chainIds,
        uint256[] memory _amounts,
        bytes32 _auctionRoot
    ) external onlyRole(ORACLE_ROLE) nonReentrant {
        require(_chainIds.length == _amounts.length, "Array length mismatch");
        
        for (uint i = 0; i < _chainIds.length; i++) {
            uint256 chainId = _chainIds[i];
            uint256 amount = _amounts[i];
            
            require(partnerChains[chainId].isActive, "Partner chain not active");
            
            PartnerChain storage partner = partnerChains[chainId];
            uint256 partnerShare = (amount * partner.revenueShare) / 10000;
            uint256 bpciShare = amount - partnerShare;
            
            // Update partner revenue
            partner.totalRevenue += partnerShare;
            totalRevenueDistributed += amount;
            
            // Record distribution
            revenueHistory[chainId].push(RevenueDistribution({
                totalAmount: amount,
                partnerShare: partnerShare,
                bpciShare: bpciShare,
                timestamp: block.timestamp,
                auctionRoot: _auctionRoot
            }));
            
            emit RevenueDistributed(chainId, partnerShare, _auctionRoot);
        }
    }
    
    /**
     * @dev Get partner chain information
     */
    function getPartnerChain(uint256 _chainId) external view returns (PartnerChain memory) {
        return partnerChains[_chainId];
    }
    
    /**
     * @dev Get all active partner chains
     */
    function getActivePartnerChains() external view returns (uint256[] memory) {
        return activeChainIds;
    }
    
    /**
     * @dev Get revenue history for a partner chain
     */
    function getRevenueHistory(uint256 _chainId) external view returns (RevenueDistribution[] memory) {
        return revenueHistory[_chainId];
    }
    
    /**
     * @dev Update partner revenue share (only admin)
     */
    function updateRevenueShare(uint256 _chainId, uint256 _newShare) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(_newShare <= 5000, "Share cannot exceed 50%"); // Max 50%
        require(partnerChains[_chainId].isActive, "Partner chain not active");
        
        partnerChains[_chainId].revenueShare = _newShare;
    }
    
    /**
     * @dev Deactivate partner chain
     */
    function deactivatePartner(uint256 _chainId) external onlyRole(DEFAULT_ADMIN_ROLE) {
        partnerChains[_chainId].isActive = false;
        
        // Remove from active chains array
        for (uint i = 0; i < activeChainIds.length; i++) {
            if (activeChainIds[i] == _chainId) {
                activeChainIds[i] = activeChainIds[activeChainIds.length - 1];
                activeChainIds.pop();
                break;
            }
        }
    }
}
```

#### 2. Round Table Oracle System
```rust
// Round Table Oracle - Rust Implementation
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use ethers::prelude::*;
use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerChainConfig {
    pub chain_id: u64,
    pub name: String,
    pub rpc_endpoint: String,
    pub websocket_endpoint: String,
    pub contract_address: Address,
    pub revenue_share_percent: u8, // 25% default
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionResult {
    pub window_id: u64,
    pub total_revenue: U256,
    pub partner_distributions: HashMap<u64, U256>,
    pub merkle_root: H256,
    pub timestamp: u64,
}

#[derive(Debug)]
pub struct RoundTableOracle {
    pub partner_chains: Arc<RwLock<HashMap<u64, PartnerChainConfig>>>,
    pub round_table_contract: Arc<RoundTablePartnership<Provider<Http>>>,
    pub bpci_auction_mempool: Arc<RwLock<crate::bpci_auction_mempool::BpciAuctionMempool>>,
    pub revenue_distribution_queue: Arc<RwLock<Vec<AuctionResult>>>,
}

impl RoundTableOracle {
    pub async fn new(
        round_table_contract_address: Address,
        provider: Provider<Http>,
    ) -> Result<Self> {
        let contract = RoundTablePartnership::new(round_table_contract_address, Arc::new(provider));
        
        Ok(Self {
            partner_chains: Arc::new(RwLock::new(HashMap::new())),
            round_table_contract: Arc::new(contract),
            bpci_auction_mempool: Arc::new(RwLock::new(
                crate::bpci_auction_mempool::BpciAuctionMempool::new()
            )),
            revenue_distribution_queue: Arc::new(RwLock::new(Vec::new())),
        })
    }
    
    /// Register a new partner chain
    pub async fn register_partner_chain(&self, config: PartnerChainConfig) -> Result<()> {
        // Validate partner chain connectivity
        self.validate_partner_chain(&config).await?;
        
        // Register on-chain
        let tx = self.round_table_contract
            .register_partner_chain(
                config.chain_id.into(),
                config.name.clone(),
                Address::zero(), // Representative address
                config.rpc_endpoint.clone(),
                config.websocket_endpoint.clone(),
            )
            .send()
            .await?;
        
        tx.await?;
        
        // Store locally
        let mut chains = self.partner_chains.write().await;
        chains.insert(config.chain_id, config);
        
        println!("✅ Partner chain registered: {}", config.name);
        Ok(())
    }
    
    /// Validate partner chain connectivity and compatibility
    async fn validate_partner_chain(&self, config: &PartnerChainConfig) -> Result<()> {
        // Test RPC connectivity
        let provider = Provider::<Http>::try_from(&config.rpc_endpoint)?;
        let chain_id = provider.get_chainid().await?;
        
        if chain_id.as_u64() != config.chain_id {
            return Err(anyhow!("Chain ID mismatch: expected {}, got {}", config.chain_id, chain_id));
        }
        
        // Test WebSocket connectivity
        let ws_provider = Provider::<Ws>::connect(&config.websocket_endpoint).await?;
        let latest_block = ws_provider.get_block_number().await?;
        
        if latest_block.as_u64() == 0 {
            return Err(anyhow!("Partner chain appears to be inactive"));
        }
        
        println!("✅ Partner chain validation successful: {} (block: {})", config.name, latest_block);
        Ok(())
    }
    
    /// Process completed auction and distribute revenue
    pub async fn process_auction_result(&self, auction_result: AuctionResult) -> Result<()> {
        let chains = self.partner_chains.read().await;
        
        // Calculate partner distributions
        let mut chain_ids = Vec::new();
        let mut amounts = Vec::new();
        
        for (chain_id, revenue) in &auction_result.partner_distributions {
            if let Some(partner_config) = chains.get(chain_id) {
                let partner_share = revenue * partner_config.revenue_share_percent as u64 / 100;
                
                chain_ids.push(U256::from(*chain_id));
                amounts.push(partner_share);
                
                println!("💰 Revenue distribution: Chain {} gets {} wei", 
                    chain_id, partner_share);
            }
        }
        
        // Execute on-chain distribution
        if !chain_ids.is_empty() {
            let tx = self.round_table_contract
                .distribute_revenue(chain_ids, amounts, auction_result.merkle_root.into())
                .send()
                .await?;
            
            tx.await?;
            println!("✅ Revenue distributed on-chain for auction {}", auction_result.window_id);
        }
        
        // Queue for partner chain notifications
        let mut queue = self.revenue_distribution_queue.write().await;
        queue.push(auction_result);
        
        Ok(())
    }
    
    /// Notify partner chains of revenue distribution
    pub async fn notify_partner_chains(&self) -> Result<()> {
        let mut queue = self.revenue_distribution_queue.write().await;
        let chains = self.partner_chains.read().await;
        
        for auction_result in queue.drain(..) {
            for (chain_id, revenue) in &auction_result.partner_distributions {
                if let Some(partner_config) = chains.get(chain_id) {
                    self.send_revenue_notification(partner_config, &auction_result, *revenue).await?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Send revenue notification to partner chain
    async fn send_revenue_notification(
        &self,
        partner_config: &PartnerChainConfig,
        auction_result: &AuctionResult,
        revenue: U256,
    ) -> Result<()> {
        let notification = serde_json::json!({
            "type": "revenue_distribution",
            "auction_window_id": auction_result.window_id,
            "total_revenue": auction_result.total_revenue,
            "partner_share": revenue,
            "merkle_root": auction_result.merkle_root,
            "timestamp": auction_result.timestamp,
            "bpci_chain_id": 1337, // BPCI chain ID
            "partner_chain_id": partner_config.chain_id
        });
        
        // Send HTTP notification to partner chain
        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/api/bpci/revenue-notification", partner_config.rpc_endpoint))
            .json(&notification)
            .send()
            .await?;
        
        if response.status().is_success() {
            println!("✅ Revenue notification sent to chain {}", partner_config.chain_id);
        } else {
            println!("⚠️ Failed to notify chain {}: {}", partner_config.chain_id, response.status());
        }
        
        Ok(())
    }
    
    /// Get partner chain statistics
    pub async fn get_partner_statistics(&self) -> Result<HashMap<u64, PartnerStats>> {
        let chains = self.partner_chains.read().await;
        let mut stats = HashMap::new();
        
        for (chain_id, config) in chains.iter() {
            let partner_info = self.round_table_contract
                .get_partner_chain(U256::from(*chain_id))
                .call()
                .await?;
            
            let revenue_history = self.round_table_contract
                .get_revenue_history(U256::from(*chain_id))
                .call()
                .await?;
            
            stats.insert(*chain_id, PartnerStats {
                name: config.name.clone(),
                total_revenue: partner_info.total_revenue,
                revenue_share_percent: config.revenue_share_percent,
                distributions_count: revenue_history.len(),
                is_active: partner_info.is_active,
                joined_at: partner_info.joined_at,
            });
        }
        
        Ok(stats)
    }
    
    /// Start oracle monitoring loop
    pub async fn start_monitoring(&self) -> Result<()> {
        println!("🚀 Round Table Oracle starting monitoring...");
        
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
        
        loop {
            interval.tick().await;
            
            // Process any pending auction results
            if let Err(e) = self.notify_partner_chains().await {
                eprintln!("❌ Error notifying partner chains: {}", e);
            }
            
            // Check for expired auction windows in mempool
            let mut mempool = self.bpci_auction_mempool.write().await;
            match mempool.process_expired_windows() {
                Ok(results) => {
                    drop(mempool); // Release lock before async operations
                    
                    for result in results {
                        let auction_result = AuctionResult {
                            window_id: result.window_id,
                            total_revenue: U256::from(result.total_revenue),
                            partner_distributions: HashMap::new(), // TODO: Calculate from result
                            merkle_root: H256::from(result.merkle_root),
                            timestamp: result.timestamp.timestamp() as u64,
                        };
                        
                        if let Err(e) = self.process_auction_result(auction_result).await {
                            eprintln!("❌ Error processing auction result: {}", e);
                        }
                    }
                }
                Err(e) => eprintln!("❌ Error processing expired windows: {}", e),
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartnerStats {
    pub name: String,
    pub total_revenue: U256,
    pub revenue_share_percent: u8,
    pub distributions_count: usize,
    pub is_active: bool,
    pub joined_at: U256,
}

/// Partner chain onboarding CLI
#[derive(Debug, clap::Parser)]
pub struct PartnerOnboardingCli {
    #[clap(subcommand)]
    pub command: PartnerCommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum PartnerCommand {
    /// Register a new partner chain
    Register {
        #[clap(long)]
        chain_id: u64,
        #[clap(long)]
        name: String,
        #[clap(long)]
        rpc_endpoint: String,
        #[clap(long)]
        websocket_endpoint: String,
        #[clap(long, default_value = "25")]
        revenue_share_percent: u8,
    },
    /// List all partner chains
    List,
    /// Get partner chain statistics
    Stats {
        #[clap(long)]
        chain_id: Option<u64>,
    },
    /// Start oracle monitoring
    Monitor,
}

impl PartnerOnboardingCli {
    pub async fn execute(&self, oracle: &RoundTableOracle) -> Result<()> {
        match &self.command {
            PartnerCommand::Register {
                chain_id,
                name,
                rpc_endpoint,
                websocket_endpoint,
                revenue_share_percent,
            } => {
                let config = PartnerChainConfig {
                    chain_id: *chain_id,
                    name: name.clone(),
                    rpc_endpoint: rpc_endpoint.clone(),
                    websocket_endpoint: websocket_endpoint.clone(),
                    contract_address: Address::zero(),
                    revenue_share_percent: *revenue_share_percent,
                };
                
                oracle.register_partner_chain(config).await?;
                println!("✅ Partner chain '{}' registered successfully!", name);
            }
            
            PartnerCommand::List => {
                let chains = oracle.partner_chains.read().await;
                println!("📋 Registered Partner Chains:");
                for (chain_id, config) in chains.iter() {
                    println!("  {} (ID: {}): {}", config.name, chain_id, config.rpc_endpoint);
                }
            }
            
            PartnerCommand::Stats { chain_id } => {
                let stats = oracle.get_partner_statistics().await?;
                
                if let Some(id) = chain_id {
                    if let Some(stat) = stats.get(id) {
                        println!("📊 Statistics for Chain {}:", id);
                        println!("  Name: {}", stat.name);
                        println!("  Total Revenue: {} wei", stat.total_revenue);
                        println!("  Revenue Share: {}%", stat.revenue_share_percent);
                        println!("  Distributions: {}", stat.distributions_count);
                        println!("  Active: {}", stat.is_active);
                    } else {
                        println!("❌ Chain {} not found", id);
                    }
                } else {
                    println!("📊 All Partner Chain Statistics:");
                    for (chain_id, stat) in stats {
                        println!("  {} ({}): {} wei, {}% share", 
                            stat.name, chain_id, stat.total_revenue, stat.revenue_share_percent);
                    }
                }
            }
            
            PartnerCommand::Monitor => {
                oracle.start_monitoring().await?;
            }
        }
        
        Ok(())
    }
}
```

### Integration with BPCI Auction Mempool

#### 3. Auction-Oracle Bridge
```rust
// Integration between auction mempool and Round Table Oracle
use crate::bpci_auction_mempool::{BpciAuctionMempool, AuctionResult as MempoolAuctionResult};
use crate::round_table_oracle::{RoundTableOracle, AuctionResult as OracleAuctionResult};

impl BpciAuctionMempool {
    /// Enhanced seal_auction_window with Round Table integration
    pub async fn seal_auction_window_with_oracle(
        &mut self, 
        window_id: u64,
        oracle: &RoundTableOracle,
    ) -> Result<MempoolAuctionResult> {
        // Seal auction using existing logic
        let result = self.seal_auction_window(window_id)?;
        
        // Calculate partner chain distributions
        let mut partner_distributions = HashMap::new();
        
        // Analyze winning transactions for partner chain origins
        for winner in &result.winners {
            let partner_share = winner.bid_amount * 25 / 100; // 25% to partners
            *partner_distributions.entry(winner.chain_id).or_insert(0) += partner_share;
        }
        
        // Create oracle-compatible result
        let oracle_result = OracleAuctionResult {
            window_id: result.window_id,
            total_revenue: U256::from(result.total_revenue),
            partner_distributions: partner_distributions.into_iter()
                .map(|(k, v)| (k, U256::from(v)))
                .collect(),
            merkle_root: H256::from(result.merkle_root),
            timestamp: result.timestamp.timestamp() as u64,
        };
        
        // Process through oracle
        oracle.process_auction_result(oracle_result).await?;
        
        Ok(result)
    }
}
```

### Deployment & Usage

#### 4. Deployment Script
```bash
#!/bin/bash
# Round Table Partnership Framework Deployment

set -euo pipefail

echo "🚀 Deploying BPCI Round Table Partnership Framework..."

# 1. Deploy smart contract
echo "📝 Deploying Round Table smart contract..."
forge create --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --constructor-args \
    src/RoundTablePartnership.sol:RoundTablePartnership

# 2. Start Round Table Oracle
echo "🔮 Starting Round Table Oracle..."
cargo build --release --bin round-table-oracle

./target/release/round-table-oracle \
    --contract-address $ROUND_TABLE_CONTRACT \
    --rpc-url $RPC_URL \
    --private-key $ORACLE_PRIVATE_KEY &

# 3. Register initial partner chains
echo "🤝 Registering initial partner chains..."

# Example: Register Polygon as partner
./target/release/round-table-cli register \
    --chain-id 137 \
    --name "Polygon" \
    --rpc-endpoint "https://polygon-rpc.com" \
    --websocket-endpoint "wss://polygon-ws.com" \
    --revenue-share-percent 25

# Example: Register Arbitrum as partner
./target/release/round-table-cli register \
    --chain-id 42161 \
    --name "Arbitrum One" \
    --rpc-endpoint "https://arb1.arbitrum.io/rpc" \
    --websocket-endpoint "wss://arb1.arbitrum.io/ws" \
    --revenue-share-percent 25

echo "✅ Round Table Partnership Framework deployed successfully!"
```

#### 5. Partner Onboarding Process
```bash
# Step 1: Partner chain applies for membership
curl -X POST https://api.bpci.org/partner/apply \
    -H "Content-Type: application/json" \
    -d '{
        "chain_id": 56,
        "name": "BNB Smart Chain",
        "rpc_endpoint": "https://bsc-dataseed.binance.org",
        "websocket_endpoint": "wss://bsc-ws-node.nariox.org:443/ws",
        "representative_address": "0x...",
        "technical_contact": "tech@binance.org",
        "business_contact": "partnerships@binance.org"
    }'

# Step 2: BPCI validates and approves
./round-table-cli register \
    --chain-id 56 \
    --name "BNB Smart Chain" \
    --rpc-endpoint "https://bsc-dataseed.binance.org" \
    --websocket-endpoint "wss://bsc-ws-node.nariox.org:443/ws"

# Step 3: Partner signs agreement
# (Done through web interface or CLI by partner representative)

# Step 4: Automatic revenue sharing begins
# (Oracle automatically processes and distributes auction revenue)
```

### Benefits & Features

#### For Partner Chains:
- **25% Revenue Share** - Automatic distribution of BPCI auction revenue
- **Unlimited Free Calls** - No fees for read operations and basic queries
- **Cross-Chain Liquidity** - Access to BPCI's multi-chain liquidity pools
- **Technical Support** - Integration assistance and ongoing support
- **Governance Participation** - Voice in BPCI network decisions

#### For BPCI Network:
- **Expanded Reach** - Access to partner chain user bases and liquidity
- **Diversified Revenue** - Multiple revenue streams from partner chains
- **Network Effects** - Stronger ecosystem through partnerships
- **Risk Distribution** - Reduced dependency on single chains
- **Innovation Acceleration** - Collaborative development opportunities

#### For Users:
- **Seamless Experience** - Cross-chain operations without complexity
- **Lower Costs** - Competitive fees through auction optimization
- **Better Liquidity** - Access to aggregated liquidity across chains
- **Enhanced Security** - Multi-chain redundancy and validation
- **More Options** - Choice of chains for different use cases

This Round Table Partnership Framework provides a complete solution for automated partner chain onboarding, transparent governance, and fair revenue distribution, enabling BPCI to build a thriving multi-chain ecosystem.
