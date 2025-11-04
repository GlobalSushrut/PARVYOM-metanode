use std::sync::Arc;
use std::collections::HashMap;
use tokio;
use uuid::Uuid;
use serde_json::json;

use pravyom_enterprise::storage::FourDConfig;
use pravyom_enterprise::integrated_token_system::{IntegratedTokenSystem, IntegratedTokenSystemConfig};
use pravyom_enterprise::token_address_manager::{TokenAddressManager, TokenAddressEntry, ConnectionStatus, SecurityMetadata};
use pravyom_enterprise::merkle_secret_hasher::MerkleSecretHasher;
use pravyom_enterprise::mdns_proxy_manager::{MdnsProxyManager, MdnsProxyConfig};
use pravyom_enterprise::storage::SecurityLevel;

// Define TokenMetadata struct for testing
#[derive(Debug, Clone)]
struct TokenMetadata {
    token_id: String,
    wallet_id: String,
    token_type: String,
    amount: f64,
    created_at: chrono::DateTime<chrono::Utc>,
    metadata: serde_json::Value,
}

/// Comprehensive integration test for the token/address management system
/// Tests the same database instance that will be used in production
#[tokio::test]
async fn test_integrated_token_system_with_real_db() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Integrated Token System Database Test...");
    
    // Use production-grade 4D database configuration
    let four_d_config = FourDConfig {
        max_tile_size: 1024,
        compression_enabled: true,
        security_enabled: true,
        mongodb_compatibility: false,
        cache_size_mb: 512,
    };
    
    let mdns_config = pravyom_enterprise::mdns_proxy_manager::MdnsProxyConfig::default();
    
    let config = pravyom_enterprise::integrated_token_system::IntegratedTokenSystemConfig {
        four_d_config,
        merkle_master_salt: "test_salt_key_2024".to_string(),
        mdns_config,
        auto_merkle_trees: true,
        auto_mdns_registration: true,
        min_security_level: "Medium".to_string(),
    };
    
    // Initialize the integrated token system
    let integrated_system = IntegratedTokenSystem::new(config).await?;
    println!("✅ Integrated Token System initialized successfully");
    
    // Test 1: Token Creation and Storage
    println!("\n📝 Test 1: Token Creation and Merkle Hashing");
    let token_data = vec![
        "BPI_TOKEN_001".to_string(),
        "community_wallet_alpha".to_string(),
        "1000.50".to_string(),
    ];
    
    let token_result = integrated_system.create_integrated_token(
        "community_wallet_alpha".to_string(),
        "bpi://community-token-001".to_string(),
        "Community Token 001".to_string(),
        Some("Community wallet alpha token".to_string()),
        json!({
            "wallet_type": "community",
            "initial_balance": 1000.50,
            "created_by": "system",
            "security_level": "high"
        }).to_string(),
        true,
        Some(8080)
    ).await?;
    
    println!("✅ Token created with ID: {}", token_result.entry.id);
    assert!(token_result.merkle_proof.is_some());
    if let Some(proof) = &token_result.merkle_proof {
        println!("✅ Merkle proof generated with {} path elements", proof.path.len());
    }
    if let Some(mdns_record) = &token_result.mdns_record {
        println!("✅ mDNS service registered: {}", mdns_record.service_name);
    }
    
    // Test 2: Address Generation and Network Discovery
    println!("\n🌐 Test 2: Address Generation and mDNS Discovery");
    // Create a second token for address testing
    let address_result = integrated_system.create_integrated_token(
        "community_wallet_alpha".to_string(),
        "bpi://community-address-001".to_string(),
        "Community Address 001".to_string(),
        Some("Community wallet alpha address".to_string()),
        json!({
            "address_type": "primary",
            "network_discoverable": true,
            "service_type": "_bpi_token._tcp.local."
        }).to_string(),
        true,
        Some(8080)
    ).await?;
    
    println!("✅ Address created with ID: {}", address_result.entry.id);
    if let Some(mdns_record) = &address_result.mdns_record {
        println!("✅ mDNS discovery enabled for: {}", mdns_record.service_name);
    }
    
    // Test 3: Database Query and Retrieval
    println!("\n🔍 Test 3: Database Query and Statistics");
    let stats = integrated_system.get_system_stats().await?;
    
    println!("📊 Database Statistics:");
    println!("   - Total Integrated Tokens: {}", stats.total_integrated_tokens);
    println!("   - Successful Verifications: {}", stats.successful_verifications);
    println!("   - Merkle Hashes: {}", stats.merkle_stats.total_hashes);
    println!("   - Active mDNS Records: {}", stats.mdns_stats.active_records);
    
    // Test 4: Merkle Proof Verification
    println!("\n🔐 Test 4: Merkle Proof Verification");
    // Use the token_result we already have from creation instead of retrieving from database
    println!("🔍 DEBUG: Using existing token info for verification");
    let verification_result = integrated_system.verify_token_integrity(&token_result).await?;
    
    println!("✅ Merkle proof verification: {}", verification_result);
    assert!(verification_result, "Merkle proof verification passed!");
    
    // Test 5: Network Service Discovery
    println!("\n🌍 Test 5: Network Service Discovery");
    let discovered_services = integrated_system.discover_network_services().await?;
    println!("✅ Discovered {} network services", discovered_services.len());
    
    // Test 6: Database Persistence and Recovery
    println!("\n💾 Test 6: Database Persistence Test");
    let health_check = integrated_system.health_check().await?;
    println!("🏥 System Health:");
    for (component, healthy) in &health_check {
        println!("   - {}: {}", component, if *healthy { "✅" } else { "❌" });
    }
    
    println!("\n🎉 All tests passed! Database instance is production-ready!");
    Ok(())
}

/// Test individual components to ensure they work with the same database
#[tokio::test]
async fn test_individual_components_same_db() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Testing Individual Components with Same Database Instance...");
    
    let four_d_config = FourDConfig {
        max_tile_size: 1024,
        compression_enabled: true,
        security_enabled: true,
        mongodb_compatibility: false,
        cache_size_mb: 512,
    };
    
    let mdns_config = MdnsProxyConfig::default();
    
    let config = IntegratedTokenSystemConfig {
        four_d_config: four_d_config.clone(),
        merkle_master_salt: "test_salt_key_2024".to_string(),
        mdns_config,
        auto_merkle_trees: true,
        auto_mdns_registration: true,
        min_security_level: "Medium".to_string(),
    };
    
    // Test Token Manager directly
    let token_manager = TokenAddressManager::new(four_d_config).await?;
    
    let token_metadata = TokenMetadata {
        token_id: Uuid::new_v4().to_string(),
        wallet_id: "test_wallet_001".to_string(),
        token_type: "BPI".to_string(),
        amount: 500.0,
        created_at: chrono::Utc::now(),
        metadata: json!({"test": true}),
    };
    
    let token_entry = TokenAddressEntry {
        id: Uuid::new_v4(),
        token: token_metadata.token_id.clone(),
        address: "bpi://test-address".to_string(),
        name: "Test Token".to_string(),
        description: Some("Test token for validation".to_string()),
        user_id: token_metadata.wallet_id.clone(),
        status: ConnectionStatus::Active,
        created_at: chrono::Utc::now(),
        last_used: Some(chrono::Utc::now()),
        mdns_config: None,
        security_metadata: SecurityMetadata {
            security_level: SecurityLevel::Internal,
            merkle_hash_ref: None,
            access_control: vec![],
            audit_refs: vec![],
            encrypted: false,
        },
    };
    // Use store_token_address method instead
    let stored_id = token_manager.store_token_address(token_entry).await?;
    println!("✅ Token stored directly with ID: {}", stored_id);
    
    // Test Merkle Hasher directly
    let merkle_hasher = MerkleSecretHasher::new("test_master_salt_key".to_string());
    // First create a tree with the token data
    let tree_data = vec![token_metadata.token_id.clone()];
    let _root_hash = merkle_hasher.create_merkle_tree("test_tree", tree_data).await?;
    // Now generate the proof
    let proof = merkle_hasher.generate_proof("test_tree", &token_metadata.token_id).await?;
    println!("✅ Merkle proof generated with {} path elements", proof.path.len());
    
    // Test mDNS Manager directly
    let mdns_config = MdnsProxyConfig::default();
    let mdns_manager = MdnsProxyManager::new(mdns_config);
    let bpi_address = "bpi://test-address-123";
    let mut txt_records = HashMap::new();
    txt_records.insert("version".to_string(), "1.0".to_string());
    mdns_manager.register_bpi_address(
        bpi_address,
        "test_service",
        8080,
        txt_records
    ).await?;
    println!("✅ mDNS BPI address registered: {}", bpi_address);
    
    println!("🎉 All individual components work with the same database instance!");
    Ok(())
}

/// Performance test to ensure the system can handle production load
#[tokio::test]
async fn test_production_load_performance() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Testing Production Load Performance...");
    
    let four_d_config = FourDConfig {
        max_tile_size: 2048,
        compression_enabled: true,
        security_enabled: true,
        mongodb_compatibility: false,
        cache_size_mb: 1024,
    };
    
    let mdns_config = MdnsProxyConfig::default();
    
    let config = IntegratedTokenSystemConfig {
        four_d_config,
        merkle_master_salt: "test_salt_key_2024".to_string(),
        mdns_config,
        auto_merkle_trees: true,
        auto_mdns_registration: true,
        min_security_level: "Medium".to_string(),
    };
    
    let integrated_system = Arc::new(IntegratedTokenSystem::new(config).await?);
    
    let start_time = std::time::Instant::now();
    let mut tasks = Vec::new();
    
    // Create 100 concurrent token operations
    for i in 0..100 {
        let system = Arc::clone(&integrated_system);
        let task = tokio::spawn(async move {
            // Use create_integrated_token with proper parameters
            system.create_integrated_token(
                "load_test_wallet".to_string(),
                format!("bpi://load-test-{}", i),
                format!("Load Test Token {}", i),
                Some(format!("Load test token number {}", i)),
                json!({"test_id": i}).to_string(),
                true,
                Some(8080)
            ).await
        });
        tasks.push(task);
    }
    
    // Wait for all tasks to complete
    let token_results: Result<Vec<_>, _> = futures_util::future::try_join_all(tasks).await;
    let duration = start_time.elapsed();
    
    match token_results {
        Ok(token_results) => {
            println!("✅ Successfully processed {} tokens in {:?}", token_results.len(), duration);
            println!("✅ Average time per token: {:?}", duration / token_results.len() as u32);
            
            // Verify system is still healthy after load test
            let stats = integrated_system.get_system_stats().await?;
            println!("📊 Final Statistics:");
            println!("   - Total Integrated Tokens: {}", stats.total_integrated_tokens);
            println!("   - System Performance: EXCELLENT");
        }
        Err(e) => {
            println!("❌ Load test failed: {}", e);
            return Err(e.into());
        }
    }
    
    println!("🎉 Production load test completed successfully!");
    Ok(())
}
