//! BPI Oracle Node Demo
//!
//! Demonstrates the complete functionality of the BPI Oracle Node including
//! cross-system communication, node discovery, message verification, and API services.

use anyhow::Result;
use bpi_oracle_node::{BpiOracleNode, OracleConfig, ConsensusConfig, SecurityConfig, PerformanceConfig};
use tokio::time::{sleep, Duration};
use tracing::{info, warn};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .init();

    info!("🚀 Starting BPI Oracle Node Demo");

    // Create Oracle Node configuration
    let config = OracleConfig {
        node_id: format!("oracle-node-{}", Uuid::new_v4()),
        api_port: 8090,
        ws_port: 8091,
        max_connections: 100,
        relay_timeout_secs: 30,
        consensus_config: ConsensusConfig {
            enable_consensus_bridge: true,
            min_consensus_nodes: 3,
            consensus_timeout_secs: 60,
            vote_threshold: 0.67,
        },
        security_config: SecurityConfig {
            require_signatures: true,
            max_message_age_secs: 300,
            rate_limit_per_node: 100,
            enable_encryption: true,
        },
        performance_config: PerformanceConfig {
            batch_size: 100,
            connection_pool_size: 50,
            message_cache_size: 10000,
            high_throughput_mode: true,
        },
    };

    info!("📋 Oracle Node Configuration:");
    info!("  Node ID: {}", config.node_id);
    info!("  API Port: {}", config.api_port);
    info!("  WebSocket Port: {}", config.ws_port);
    info!("  Max Connections: {}", config.max_connections);

    // Create and start Oracle Node
    let oracle_node = BpiOracleNode::new(config.clone()).await?;
    
    info!("🔧 Starting Oracle Node services...");
    oracle_node.start().await?;

    // Demonstrate Oracle Node functionality
    demonstrate_oracle_functionality(&oracle_node).await?;

    // Keep the demo running
    info!("✅ BPI Oracle Node Demo running successfully!");
    info!("🌐 API Server: http://localhost:{}", config.api_port);
    info!("🔌 WebSocket Server: ws://localhost:{}", config.ws_port);
    info!("📊 Health Check: http://localhost:{}/health", config.api_port);
    info!("📈 Statistics: http://localhost:{}/api/stats", config.api_port);
    
    // Run for demonstration period
    info!("⏰ Running demo for 2 minutes...");
    for i in 1..=12 {
        sleep(Duration::from_secs(10)).await;
        
        // Show periodic statistics
        let stats = oracle_node.get_stats().await;
        info!("📊 Stats Update {} - Nodes: {}, Messages: {}, Uptime: {}s", 
              i, stats.total_nodes, stats.messages_relayed, stats.uptime_seconds);
        
        // Simulate some activity every 30 seconds
        if i % 3 == 0 {
            simulate_oracle_activity(&oracle_node).await?;
        }
    }

    info!("🛑 Shutting down Oracle Node Demo...");
    oracle_node.shutdown().await?;
    info!("✅ BPI Oracle Node Demo completed successfully!");

    Ok(())
}

/// Demonstrate Oracle Node functionality
async fn demonstrate_oracle_functionality(oracle_node: &BpiOracleNode) -> Result<()> {
    info!("🧪 Demonstrating Oracle Node functionality...");

    // 1. Node Discovery Demo
    info!("1️⃣ Testing Node Discovery...");
    let discovered_nodes = oracle_node.get_discovered_nodes().await;
    info!("   Discovered {} nodes in the network", discovered_nodes.len());

    // 2. Cross-System Communication Demo
    info!("2️⃣ Testing Cross-System Communication...");
    let test_message = serde_json::json!({
        "type": "test_communication",
        "data": "Hello from Oracle Node",
        "timestamp": chrono::Utc::now()
    });
    
    match oracle_node.send_cross_system_message(
        "test-source-system",
        "test-target-system", 
        test_message
    ).await {
        Ok(message_id) => info!("   ✅ Cross-system message sent: {}", message_id),
        Err(e) => warn!("   ⚠️ Cross-system message failed: {}", e),
    }

    // 3. Data Query Demo
    info!("3️⃣ Testing Data Query capabilities...");
    let query_result = oracle_node.query_network_data(
        "network_status",
        serde_json::json!({"include_metrics": true})
    ).await;
    
    match query_result {
        Ok(data) => info!("   ✅ Network data query successful: {} results", 
                         data.get("results").and_then(|r| r.as_array()).map(|a| a.len()).unwrap_or(0)),
        Err(e) => warn!("   ⚠️ Network data query failed: {}", e),
    }

    // 4. Message Verification Demo
    info!("4️⃣ Testing Message Verification...");
    let verification_stats = oracle_node.get_verification_stats().await;
    info!("   📊 Verification Stats - Total: {}, Success: {}, Failed: {}", 
          verification_stats.total_verifications,
          verification_stats.successful_verifications,
          verification_stats.failed_verifications);

    // 5. Consensus Bridge Demo
    info!("5️⃣ Testing Consensus Bridge...");
    let consensus_stats = oracle_node.get_consensus_stats().await;
    info!("   📊 Consensus Stats - Proposals: {}, Successful: {}, Failed: {}, Avg Time: {:.1}ms", 
          consensus_stats.total_proposals,
          consensus_stats.successful_consensus,
          consensus_stats.failed_consensus,
          consensus_stats.average_consensus_time_ms);

    info!("✅ Oracle Node functionality demonstration completed!");
    Ok(())
}

/// Simulate Oracle Node activity
async fn simulate_oracle_activity(oracle_node: &BpiOracleNode) -> Result<()> {
    info!("🎭 Simulating Oracle Node activity...");

    // Simulate incoming cross-system message
    let incoming_message = serde_json::json!({
        "source": "external-system-1",
        "target": "bpi-node-cluster",
        "type": "data_sync_request",
        "payload": {
            "sync_type": "incremental",
            "last_sync": chrono::Utc::now() - chrono::Duration::minutes(5),
            "data_types": ["transactions", "state_updates"]
        }
    });

    // Convert JSON to OracleMessage for processing
    let oracle_message = bpi_oracle_node::OracleMessage {
        message_id: uuid::Uuid::new_v4().to_string(),
        from_node: "external-system-1".to_string(),
        to_node: Some("bpi-node-cluster".to_string()),
        message_type: bpi_oracle_node::MessageType::DataSync,
        payload: incoming_message,
        timestamp: chrono::Utc::now(),
        priority: bpi_oracle_node::MessagePriority::Normal,
        signature: None,
        encryption_key: None,
        ttl_seconds: 300,
    };

    match oracle_node.process_incoming_message(oracle_message).await {
        Ok(response) => info!("   ✅ Processed incoming message: {:?}", response),
        Err(e) => warn!("   ⚠️ Failed to process incoming message: {}", e),
    }

    // Simulate consensus proposal
    let proposal = serde_json::json!({
        "proposal_id": Uuid::new_v4().to_string(),
        "type": "network_upgrade",
        "description": "Enable new cross-chain communication protocol",
        "voting_deadline": chrono::Utc::now() + chrono::Duration::hours(24),
        "required_consensus": 0.75
    });

    match oracle_node.submit_consensus_proposal(proposal).await {
        Ok(proposal_id) => info!("   ✅ Submitted consensus proposal: {}", proposal_id),
        Err(e) => warn!("   ⚠️ Failed to submit consensus proposal: {}", e),
    }

    // Simulate data relay operation
    let relay_data = serde_json::json!({
        "operation": "bulk_sync",
        "source_nodes": ["node-1", "node-2", "node-3"],
        "target_nodes": ["node-4", "node-5"],
        "data_size": 1024 * 512, // 512KB
        "compression": true
    });

    match oracle_node.initiate_data_relay(relay_data).await {
        Ok(relay_id) => info!("   ✅ Initiated data relay: {}", relay_id),
        Err(e) => warn!("   ⚠️ Failed to initiate data relay: {}", e),
    }

    info!("✅ Oracle Node activity simulation completed!");
    Ok(())
}
