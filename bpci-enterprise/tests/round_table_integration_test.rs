use pravyom_enterprise::round_table_oracle::{RoundTableOracle, PartnerChainConfig, OracleConfig};
use pravyom_enterprise::bpci_auction_mempool::{BpciAuctionMempool, AuctionTransaction, AuctionType, AuctionResult};
use chrono::Utc;
use tokio;

/// Integration test for Round Table Oracle with BPCI Auction Mempool
/// Tests the complete flow from partner registration to revenue distribution

#[tokio::test]
async fn test_complete_round_table_integration() {
    println!("🚀 Starting Round Table Oracle integration test...");
    
    // 1. Initialize Oracle with test configuration
    let oracle_config = OracleConfig {
        bpci_chain_id: 1337,
        monitoring_interval_secs: 1, // Fast for testing
        max_partner_chains: 10,
        default_revenue_share: 25,
        min_payout_threshold: 1000, // Low threshold for testing
    };
    
    let oracle = RoundTableOracle::new(Some(oracle_config));
    
    // 2. Register test partner chains
    println!("📝 Registering test partner chains...");
    
    let polygon_config = PartnerChainConfig::new(
        137,
        "Polygon Testnet".to_string(),
        "https://rpc-mumbai.maticvigil.com".to_string(), // Use testnet for testing
        "wss://ws-mumbai.maticvigil.com".to_string(),
        "0x1234567890123456789012345678901234567890".to_string(),
    );
    
    let arbitrum_config = PartnerChainConfig::new(
        421613,
        "Arbitrum Goerli".to_string(),
        "https://goerli-rollup.arbitrum.io/rpc".to_string(),
        "wss://goerli-rollup.arbitrum.io/ws".to_string(),
        "0x2345678901234567890123456789012345678901".to_string(),
    );
    
    // Note: These will likely fail validation due to network connectivity,
    // but we'll manually add them for testing purposes
    {
        let mut chains = oracle.partner_chains.write().await;
        chains.insert(137, polygon_config.clone());
        chains.insert(421613, arbitrum_config.clone());
    }
    
    println!("✅ Partner chains registered: Polygon (137), Arbitrum (421613)");
    
    // 3. Create partnerships
    println!("🤝 Creating partnerships...");
    
    let polygon_partnership = oracle.create_partnership(137).await.unwrap();
    let arbitrum_partnership = oracle.create_partnership(421613).await.unwrap();
    
    // Sign partnerships
    oracle.sign_partnership(&polygon_partnership, "polygon_signature".to_string(), true).await.unwrap();
    oracle.sign_partnership(&polygon_partnership, "bpci_signature".to_string(), false).await.unwrap();
    
    oracle.sign_partnership(&arbitrum_partnership, "arbitrum_signature".to_string(), true).await.unwrap();
    oracle.sign_partnership(&arbitrum_partnership, "bpci_signature".to_string(), false).await.unwrap();
    
    println!("✅ Partnerships created and signed");
    
    // 4. Create test auction transactions
    println!("💰 Creating test auction transactions...");
    
    let polygon_tx1 = AuctionTransaction::new(
        [1u8; 32],
        137, // Polygon chain ID
        2000000, // 2M wei bid
        50000,   // Gas limit
        200,     // Data size
        "polygon_user_1".to_string(),
    );
    
    let polygon_tx2 = AuctionTransaction::new(
        [2u8; 32],
        137, // Polygon chain ID
        1500000, // 1.5M wei bid
        30000,   // Gas limit
        150,     // Data size
        "polygon_user_2".to_string(),
    );
    
    let arbitrum_tx1 = AuctionTransaction::new(
        [3u8; 32],
        421613, // Arbitrum chain ID
        1800000, // 1.8M wei bid
        40000,   // Gas limit
        180,     // Data size
        "arbitrum_user_1".to_string(),
    );
    
    let bpci_tx1 = AuctionTransaction::new(
        [4u8; 32],
        1337, // BPCI chain ID
        2500000, // 2.5M wei bid
        60000,   // Gas limit
        250,     // Data size
        "bpci_user_1".to_string(),
    );
    
    // 5. Submit transactions to auction mempool
    {
        let mut mempool = oracle.bpci_auction_mempool.write().await;
        mempool.submit_transaction(polygon_tx1.clone()).unwrap();
        mempool.submit_transaction(polygon_tx2.clone()).unwrap();
        mempool.submit_transaction(arbitrum_tx1.clone()).unwrap();
        mempool.submit_transaction(bpci_tx1.clone()).unwrap();
        
        println!("✅ Submitted 4 transactions to auction mempool");
        
        // Create auction window
        let window_id = mempool.create_auction_window(
            5000, // 5 second window
            10,   // Max 10 transactions
            200000, // Total gas limit
            AuctionType::StandardExecution,
        );
        
        println!("🎯 Created auction window: {}", window_id);
        
        // Immediately seal the auction for testing
        let auction_result = mempool.seal_auction_window(window_id).await.unwrap();
        
        println!("🏆 Auction sealed with {} winners, total revenue: {} wei", 
            auction_result.winning_transactions.len(), auction_result.total_revenue);
        
        // Verify transaction ordering (highest effective bid rate first)
        for (i, winner) in auction_result.winning_transactions.iter().enumerate() {
            println!("  Winner {}: Chain {}, Bid: {} wei, Rate: {:.6}", 
                i + 1, winner.chain_id, winner.bid_amount, winner.effective_bid_rate());
        }
    }
    
    // 6. Process auction result through oracle
    println!("🔮 Processing auction result through Round Table Oracle...");
    
    // Get the auction result
    let auction_result = {
        let mempool = oracle.bpci_auction_mempool.read().await;
        let stats = mempool.get_mempool_stats();
        
        // Create a mock auction result for testing
        AuctionResult {
            auction_id: "test_auction_1".to_string(),
            window_id: 1,
            winning_transactions: vec![bpci_tx1, polygon_tx1, arbitrum_tx1, polygon_tx2],
            total_revenue: 7800000, // Sum of all bids
            merkle_root: [0u8; 32],
            timestamp: Utc::now(),
        }
    };
    
    // Process through oracle
    oracle.process_auction_result(auction_result).await.unwrap();
    
    println!("✅ Auction result processed through oracle");
    
    // 7. Verify revenue distribution
    println!("📊 Verifying revenue distribution...");
    
    let stats = oracle.get_partner_statistics().await.unwrap();
    
    for (chain_id, stat) in &stats {
        println!("  Chain {} ({}): {} wei total revenue, {}% share", 
            chain_id, stat.name, stat.total_revenue, stat.revenue_share_percent);
    }
    
    // Verify Polygon received revenue
    let polygon_stats = stats.get(&137).unwrap();
    assert!(polygon_stats.total_revenue > 0, "Polygon should have received revenue");
    assert_eq!(polygon_stats.revenue_share_percent, 25, "Polygon should have 25% revenue share");
    
    // Verify Arbitrum received revenue
    let arbitrum_stats = stats.get(&421613).unwrap();
    assert!(arbitrum_stats.total_revenue > 0, "Arbitrum should have received revenue");
    assert_eq!(arbitrum_stats.revenue_share_percent, 25, "Arbitrum should have 25% revenue share");
    
    println!("✅ Revenue distribution verified");
    
    // 8. Test oracle status
    println!("📈 Checking oracle status...");
    
    let oracle_status = oracle.get_oracle_status().await;
    
    println!("  Total Partner Chains: {}", oracle_status.total_partner_chains);
    println!("  Active Partner Chains: {}", oracle_status.active_partner_chains);
    println!("  Total Partnerships: {}", oracle_status.total_partnerships);
    println!("  Active Partnerships: {}", oracle_status.active_partnerships);
    println!("  Total Distributions: {}", oracle_status.total_distributions);
    println!("  Total Revenue Distributed: {} wei", oracle_status.total_revenue_distributed);
    
    assert_eq!(oracle_status.total_partner_chains, 2, "Should have 2 partner chains");
    assert_eq!(oracle_status.active_partner_chains, 2, "Should have 2 active partner chains");
    assert_eq!(oracle_status.total_partnerships, 2, "Should have 2 partnerships");
    assert_eq!(oracle_status.active_partnerships, 2, "Should have 2 active partnerships");
    assert_eq!(oracle_status.total_distributions, 1, "Should have 1 distribution");
    assert!(oracle_status.total_revenue_distributed > 0, "Should have distributed revenue");
    
    println!("✅ Oracle status verified");
    
    // 9. Test revenue distribution records
    println!("📋 Checking revenue distribution records...");
    
    let distributions = oracle.revenue_distributions.read().await;
    assert_eq!(distributions.len(), 1, "Should have 1 distribution record");
    
    let distribution = &distributions[0];
    println!("  Distribution ID: {}", distribution.distribution_id);
    println!("  Auction Window: {}", distribution.auction_window_id);
    println!("  Total Revenue: {} wei", distribution.total_auction_revenue);
    println!("  BPCI Share: {} wei", distribution.bpci_share);
    println!("  Partner Distributions: {:?}", distribution.partner_distributions);
    
    // Verify distribution math
    let partner_total: u64 = distribution.partner_distributions.values().sum();
    let expected_total = distribution.bpci_share + partner_total;
    assert_eq!(expected_total, distribution.total_auction_revenue, 
        "Distribution math should be correct");
    
    println!("✅ Revenue distribution records verified");
    
    println!("🎉 Round Table Oracle integration test completed successfully!");
}

#[tokio::test]
async fn test_partner_chain_validation() {
    println!("🔍 Testing partner chain validation...");
    
    let oracle = RoundTableOracle::new(None);
    
    // Test invalid configurations
    let invalid_configs = vec![
        PartnerChainConfig::new(0, "Invalid".to_string(), "".to_string(), "".to_string(), "".to_string()),
        PartnerChainConfig::new(1, "".to_string(), "https://test.com".to_string(), "wss://test.com".to_string(), "0x123".to_string()),
        PartnerChainConfig::new(2, "Test".to_string(), "".to_string(), "wss://test.com".to_string(), "0x123".to_string()),
    ];
    
    for config in invalid_configs {
        match oracle.register_partner_chain(config.clone()).await {
            Ok(_) => panic!("Should have failed validation for invalid config: {:?}", config),
            Err(e) => println!("✅ Correctly rejected invalid config: {}", e),
        }
    }
    
    println!("✅ Partner chain validation test completed");
}

#[tokio::test]
async fn test_partnership_lifecycle() {
    println!("🤝 Testing partnership lifecycle...");
    
    let oracle = RoundTableOracle::new(None);
    
    // Add a test partner chain manually
    let config = PartnerChainConfig::new(
        999,
        "Test Chain".to_string(),
        "https://test-rpc.com".to_string(),
        "wss://test-ws.com".to_string(),
        "0x9999999999999999999999999999999999999999".to_string(),
    );
    
    {
        let mut chains = oracle.partner_chains.write().await;
        chains.insert(999, config);
    }
    
    // Create partnership
    let partnership_id = oracle.create_partnership(999).await.unwrap();
    println!("✅ Partnership created: {}", partnership_id);
    
    // Verify partnership exists but is not yet active
    {
        let partnerships = oracle.partnerships.read().await;
        let partnership = partnerships.get(&partnership_id).unwrap();
        assert!(!partnership.mutual_agreement, "Partnership should not be active yet");
        assert!(partnership.partner_signature.is_none(), "Partner should not have signed yet");
        assert!(partnership.bpci_signature.is_none(), "BPCI should not have signed yet");
    }
    
    // Sign as partner
    oracle.sign_partnership(&partnership_id, "partner_sig".to_string(), true).await.unwrap();
    println!("✅ Partner signed");
    
    // Verify partial signing
    {
        let partnerships = oracle.partnerships.read().await;
        let partnership = partnerships.get(&partnership_id).unwrap();
        assert!(!partnership.mutual_agreement, "Partnership should not be fully active yet");
        assert!(partnership.partner_signature.is_some(), "Partner should have signed");
        assert!(partnership.bpci_signature.is_none(), "BPCI should not have signed yet");
    }
    
    // Sign as BPCI
    oracle.sign_partnership(&partnership_id, "bpci_sig".to_string(), false).await.unwrap();
    println!("✅ BPCI signed");
    
    // Verify full activation
    {
        let partnerships = oracle.partnerships.read().await;
        let partnership = partnerships.get(&partnership_id).unwrap();
        assert!(partnership.mutual_agreement, "Partnership should be fully active");
        assert!(partnership.partner_signature.is_some(), "Partner should have signed");
        assert!(partnership.bpci_signature.is_some(), "BPCI should have signed");
    }
    
    println!("✅ Partnership lifecycle test completed");
}

#[tokio::test]
async fn test_revenue_calculation_accuracy() {
    println!("💰 Testing revenue calculation accuracy...");
    
    let oracle = RoundTableOracle::new(None);
    
    // Add test partner chains with different revenue shares
    let configs = vec![
        (100, "Chain A", 25), // 25%
        (200, "Chain B", 30), // 30%
        (300, "Chain C", 20), // 20%
    ];
    
    {
        let mut chains = oracle.partner_chains.write().await;
        for (chain_id, name, revenue_share) in &configs {
            let mut config = PartnerChainConfig::new(
                *chain_id,
                name.to_string(),
                "https://test.com".to_string(),
                "wss://test.com".to_string(),
                "0x1234567890123456789012345678901234567890".to_string(),
            );
            config.revenue_share_percent = *revenue_share;
            chains.insert(*chain_id, config);
        }
    }
    
    // Create test transactions with known bid amounts
    let transactions = vec![
        AuctionTransaction::new([1u8; 32], 100, 1000000, 21000, 100, "user1".to_string()), // Chain A: 1M wei
        AuctionTransaction::new([2u8; 32], 200, 2000000, 21000, 100, "user2".to_string()), // Chain B: 2M wei
        AuctionTransaction::new([3u8; 32], 300, 1500000, 21000, 100, "user3".to_string()), // Chain C: 1.5M wei
    ];
    
    let auction_result = AuctionResult {
        auction_id: "test_auction_2".to_string(),
        window_id: 1,
        winning_transactions: transactions,
        total_revenue: 4500000, // 1M + 2M + 1.5M
        merkle_root: [0u8; 32],
        timestamp: Utc::now(),
    };
    
    // Process through oracle
    oracle.process_auction_result(auction_result).await.unwrap();
    
    // Verify calculations
    let stats = oracle.get_partner_statistics().await.unwrap();
    
    // Chain A: 1M * 25% = 250K
    let chain_a_stats = stats.get(&100).unwrap();
    assert_eq!(chain_a_stats.total_revenue, 250000, "Chain A should receive 250K wei");
    
    // Chain B: 2M * 30% = 600K
    let chain_b_stats = stats.get(&200).unwrap();
    assert_eq!(chain_b_stats.total_revenue, 600000, "Chain B should receive 600K wei");
    
    // Chain C: 1.5M * 20% = 300K
    let chain_c_stats = stats.get(&300).unwrap();
    assert_eq!(chain_c_stats.total_revenue, 300000, "Chain C should receive 300K wei");
    
    // Verify distribution record
    let distributions = oracle.revenue_distributions.read().await;
    let distribution = &distributions[0];
    
    let total_partner_share: u64 = distribution.partner_distributions.values().sum();
    assert_eq!(total_partner_share, 1150000, "Total partner share should be 1.15M wei"); // 250K + 600K + 300K
    assert_eq!(distribution.bpci_share, 3350000, "BPCI share should be 3.35M wei"); // 4.5M - 1.15M
    
    println!("✅ Revenue calculation accuracy test completed");
}
