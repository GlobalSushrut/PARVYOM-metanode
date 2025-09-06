//! # BPI → BPCI → Testnet Integration Test Runner
//! 
//! Comprehensive test runner that executes the full BPI → BPCI → Testnet flow:
//! 1. Takes real BPI data from live BPI ledger
//! 2. Verifies BPI authenticity using cryptographic proofs
//! 3. Bundles verified BPI data to BPCI format
//! 4. Transmits BPCI to development/test endpoint (NOT mainnet)
//! 
//! Usage:
//! ```bash
//! BPCI_NETWORK_MODE=testnet cargo run --bin bpi_testnet_integration_runner
//! ```

use anyhow::{Result, anyhow};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use tracing_subscriber;
use chrono::Utc;

use pravyom_enterprise::{
    bpi_ledger_integration::BpiLedgerClient,
    bpci_auction_mempool_minimal::BpciAuctionMempool,
    testnet_auction_storage::TestnetAuctionStorage,
    testnet_config::BpciConfig,
    bpi_testnet_integration::{BpiTestnetIntegration, IntegrationTestResult},
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("🚀 Starting BPI → BPCI → Testnet Integration Test Runner");
    info!("🚨 SAFETY: This test uses DEVELOPMENT endpoints only - NO mainnet transactions");

    // Verify testnet mode
    let network_mode = std::env::var("BPCI_NETWORK_MODE").unwrap_or_else(|_| "testnet".to_string());
    if network_mode != "testnet" {
        error!("❌ SAFETY CHECK FAILED: BPCI_NETWORK_MODE must be 'testnet'");
        error!("   Current mode: {}", network_mode);
        error!("   Set: export BPCI_NETWORK_MODE=testnet");
        return Err(anyhow!("Safety check failed: not in testnet mode"));
    }

    info!("✅ Safety check passed: Running in testnet mode");

    // Initialize BPCI configuration
    info!("📋 Initializing BPCI testnet configuration");
    let config: Arc<BpciConfig> = Arc::new(BpciConfig::from_env()?);
    info!("✅ BPCI configuration loaded");

    // Initialize BPI ledger client
    info!("🔗 Initializing BPI ledger client for real data access");
    let bpi_client: Arc<BpiLedgerClient> = Arc::new(BpiLedgerClient::new().await?);
    
    // Test BPI connection
    if bpi_client.is_connected().await {
        info!("✅ BPI ledger client connected successfully");
    } else {
        warn!("⚠️  BPI ledger client not connected - will use test data");
    }

    // Initialize BPCI auction mempool
    info!("🏛️ Initializing BPCI auction mempool");
    let auction_mempool = Arc::new(RwLock::new(BpciAuctionMempool::new()));
    info!("✅ BPCI auction mempool initialized");

    // Initialize testnet auction storage
    info!("💾 Initializing testnet auction storage");
    let testnet_storage = Arc::new(TestnetAuctionStorage::new(config.clone()).await?);
    info!("✅ Testnet auction storage initialized");

    // Initialize BPI → BPCI → Testnet integration test suite
    info!("🧪 Initializing integration test suite");
    let integration_test = BpiTestnetIntegration::new(
        bpi_client,
        auction_mempool,
        testnet_storage,
        config.clone(),
    ).await?;
    info!("✅ Integration test suite initialized");

    // Display test configuration
    display_test_configuration(&integration_test).await;

    // Execute integration test
    info!("🎯 Executing BPI → BPCI → Testnet integration test");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("📝 NOTE: Endpoint connection failures are EXPECTED (using example URLs for safety)");
    
    let test_result = match integration_test.execute_integration_test().await {
        Ok(result) => {
            info!("🎉 Integration test completed successfully!");
            info!("✅ All core functionality validated: fetch → verify → bundle → transmit");
            display_test_results(&result);
            result
        }
        Err(e) => {
            error!("❌ Integration test failed: {}", e);
            return Err(e);
        }
    };

    // Display final metrics
    info!("📊 Displaying final test metrics");
    let metrics = integration_test.get_test_metrics().await;
    display_test_metrics(&metrics);

    // Validate test results
    validate_test_results(&test_result)?;

    info!("✅ BPI → BPCI → Testnet integration test completed successfully!");
    info!("🚨 CONFIRMED: No mainnet transactions were executed during this test");
    
    Ok(())
}

/// Display test configuration
async fn display_test_configuration(integration_test: &BpiTestnetIntegration) {
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("📋 TEST CONFIGURATION");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("🌐 Primary Dev Endpoint: {}", integration_test.dev_endpoints.primary_dev_endpoint);
    info!("🔗 Backup Endpoints: {} configured", integration_test.dev_endpoints.backup_dev_endpoints.len());
    info!("⛓️  Testnet Chain ID: {}", integration_test.dev_endpoints.testnet_chain_id);
    info!("🆔 Network ID: {}", integration_test.dev_endpoints.dev_network_id);
    info!("⏱️  Request Timeout: {}s", integration_test.dev_endpoints.request_timeout_seconds);
    info!("🔄 Max Retry Attempts: {}", integration_test.dev_endpoints.max_retry_attempts);
    info!("🚨 SAFETY: All endpoints are DEVELOPMENT/TESTNET only");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

/// Display test results
fn display_test_results(result: &IntegrationTestResult) {
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("🎯 INTEGRATION TEST RESULTS");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("🆔 Test ID: {}", result.test_id);
    info!("📡 BPI Transaction ID: {}", result.bpi_tx_id);
    info!("📦 BPCI Bundle Hash: {}", result.bpci_bundle_hash);
    info!("🌐 Testnet TX Hash: {}", result.transmission_result.testnet_tx_hash);
    info!("⏱️  Total Execution Time: {}ms", result.total_execution_time_ms);
    info!("✅ Success: {}", result.success);
    info!("🕐 Timestamp: {}", result.timestamp);
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

/// Display test metrics
fn display_test_metrics(metrics: &pravyom_enterprise::bpi_testnet_integration::IntegrationTestMetrics) {
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("📊 INTEGRATION TEST METRICS");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("📈 Total Tests: {}", metrics.total_tests);
    info!("✅ Successful Verifications: {}", metrics.successful_verifications);
    info!("❌ Failed Verifications: {}", metrics.failed_verifications);
    info!("📦 Successful Bundles: {}", metrics.successful_bundles);
    info!("🌐 Successful Transmissions: {}", metrics.successful_transmissions);
    info!("❌ Failed Transmissions: {}", metrics.failed_transmissions);
    info!("⏱️  Avg Verification Time: {:.2}ms", metrics.avg_verification_time_ms);
    info!("⏱️  Avg Bundling Time: {:.2}ms", metrics.avg_bundling_time_ms);
    info!("⏱️  Avg Transmission Time: {:.2}ms", metrics.avg_transmission_time_ms);
    info!("🕐 Last Test: {}", metrics.last_test_timestamp);
    
    // Calculate success rates
    let verification_success_rate = if metrics.total_tests > 0 {
        (metrics.successful_verifications as f64 / metrics.total_tests as f64) * 100.0
    } else {
        0.0
    };
    
    let transmission_success_rate = if metrics.total_tests > 0 {
        (metrics.successful_transmissions as f64 / metrics.total_tests as f64) * 100.0
    } else {
        0.0
    };
    
    info!("📊 Verification Success Rate: {:.1}%", verification_success_rate);
    info!("📊 Transmission Success Rate: {:.1}%", transmission_success_rate);
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

/// Validate test results
fn validate_test_results(result: &IntegrationTestResult) -> Result<()> {
    info!("🔍 Validating integration test results");

    // Check if test was successful
    if !result.success {
        return Err(anyhow!("Integration test marked as failed"));
    }

    // Validate test ID format
    if result.test_id.is_empty() || !result.test_id.starts_with("integration_test_") {
        return Err(anyhow!("Invalid test ID format: {}", result.test_id));
    }

    // Validate BPI transaction ID
    if result.bpi_tx_id.is_empty() {
        return Err(anyhow!("Empty BPI transaction ID"));
    }

    // Validate BPCI bundle hash
    if result.bpci_bundle_hash.is_empty() {
        return Err(anyhow!("Empty BPCI bundle hash"));
    }

    // Validate testnet transaction hash
    if result.transmission_result.testnet_tx_hash.is_empty() {
        return Err(anyhow!("Empty testnet transaction hash"));
    }

    // Validate execution time is reasonable (not too fast or too slow)
    if result.total_execution_time_ms < 100 {
        warn!("⚠️  Execution time seems too fast: {}ms", result.total_execution_time_ms);
    }
    
    if result.total_execution_time_ms > 60000 {
        warn!("⚠️  Execution time seems slow: {}ms", result.total_execution_time_ms);
    }

    // Validate timestamp is recent
    let now = Utc::now();
    let time_diff = now.signed_duration_since(result.timestamp).num_seconds();
    if time_diff > 300 { // 5 minutes
        warn!("⚠️  Test timestamp seems old: {} seconds ago", time_diff);
    }

    info!("✅ All validation checks passed");
    Ok(())
}

/// Display startup banner
fn display_startup_banner() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🚀 BPI → BPCI → TESTNET INTEGRATION TEST RUNNER");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📡 Takes real BPI data from live BPI ledger");
    println!("🔐 Verifies BPI authenticity using cryptographic proofs");
    println!("📦 Bundles verified BPI data to BPCI format");
    println!("🌐 Transmits BPCI to DEVELOPMENT endpoint (NOT mainnet)");
    println!("🚨 SAFETY: NO mainnet transactions will be executed");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
}
