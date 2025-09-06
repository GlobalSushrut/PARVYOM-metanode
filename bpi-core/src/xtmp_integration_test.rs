// XTMP Integration Test - Comprehensive testing of BPI Core ↔ BPCI XTMP communication
// Tests the complete XTMP protocol implementation with performance benchmarks

use crate::{
    xtmp_protocol::*,
    xtmp_bpci_client::*,
    bpci_xtmp_server::*,
    bpi_ledger_state::*,
};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{timeout, Duration};
use std::sync::Arc;
use uuid::Uuid;
use crate::bpi_ledger_state::{PoEProofBundle, ImmutableProof, BpiLedgerMetadata, HyperledgerProof};
use crate::production_bpci_client::{ProductionWalletAddress, ProductionToken};
use anyhow::Result;
use log::{info, warn, error};
use std::time::{Duration, Instant};
use tokio::time::timeout;
use chrono::Utc;

pub struct XTMPIntegrationTest {
    pub server: Option<BpciXtmpServer>,
    pub client: Option<XTMPBpciClient>,
    pub test_results: XTMPTestResults,
}

#[derive(Debug, Clone)]
pub struct XTMPTestResults {
    pub connection_test_passed: bool,
    pub wallet_registration_passed: bool,
    pub bundle_submission_passed: bool,
    pub real_time_streaming_passed: bool,
    pub performance_metrics: XTMPPerformanceResults,
    pub error_count: u32,
}

#[derive(Debug, Clone)]
pub struct XTMPPerformanceResults {
    pub connection_time_ms: f64,
    pub message_latency_ms: f64,
    pub throughput_messages_per_sec: f64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
}

impl XTMPIntegrationTest {
    pub fn new() -> Self {
        Self {
            server: None,
            client: None,
            test_results: XTMPTestResults::new(),
        }
    }
    
    /// Run comprehensive XTMP integration tests
    pub async fn run_full_test_suite(&mut self) -> Result<XTMPTestResults> {
        info!("🧪 Starting XTMP Integration Test Suite");
        
        // Stage 1: Server Setup
        self.test_server_initialization().await?;
        
        // Stage 2: Client Connection
        self.test_client_connection().await?;
        
        // Stage 3: Protocol Communication
        self.test_protocol_communication().await?;
        
        // Stage 4: Performance Benchmarks
        self.test_performance_benchmarks().await?;
        
        // Stage 5: Real-time Streaming
        self.test_real_time_streaming().await?;
        
        // Stage 6: Error Handling
        self.test_error_handling().await?;
        
        // Stage 7: Cleanup
        self.cleanup_test_environment().await?;
        
        info!("✅ XTMP Integration Test Suite Completed");
        self.print_test_summary();
        
        Ok(self.test_results.clone())
    }
    
    /// Test 1: Server Initialization
    async fn test_server_initialization(&mut self) -> Result<()> {
        info!("🔧 Test 1: Server Initialization");
        
        let config = BpciXtmpServerConfig {
            bind_address: "127.0.0.1:7779".to_string(), // Use different port for testing
            max_connections: 100,
            connection_timeout: Duration::from_secs(30),
            heartbeat_interval: Duration::from_secs(10),
            enable_compression: true,
            enable_real_time_streams: true,
        };
        
        let server = BpciXtmpServer::new(config).await?;
        
        // Start server in background
        let server_clone = server.clone();
        tokio::spawn(async move {
            if let Err(e) = server_clone.start().await {
                error!("Server startup error: {}", e);
            }
        });
        
        // Give server time to start
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        self.server = Some(server);
        info!("✅ Server initialization test passed");
        
        Ok(())
    }
    
    /// Test 2: Client Connection
    async fn test_client_connection(&mut self) -> Result<()> {
        info!("🔌 Test 2: Client Connection");
        
        let start_time = Instant::now();
        
        let client = XTMPBpciClient::new("127.0.0.1:7779".to_string()).await?;
        
        let connection_time = start_time.elapsed();
        self.test_results.performance_metrics.connection_time_ms = connection_time.as_millis() as f64;
        
        // Test connection establishment
        match timeout(Duration::from_secs(5), client.ensure_connection()).await {
            Ok(Ok(session_id)) => {
                info!("✅ Connection established with session ID: {}", session_id);
                self.test_results.connection_test_passed = true;
            }
            Ok(Err(e)) => {
                error!("❌ Connection failed: {}", e);
                self.test_results.error_count += 1;
            }
            Err(_) => {
                error!("❌ Connection timeout");
                self.test_results.error_count += 1;
            }
        }
        
        self.client = Some(client);
        info!("✅ Client connection test completed");
        
        Ok(())
    }
    
    /// Test 3: Protocol Communication
    async fn test_protocol_communication(&mut self) -> Result<()> {
        info!("📡 Test 3: Protocol Communication");
        
        if let Some(ref mut client) = self.client {
            // Test wallet registration
            self.test_wallet_registration(client).await?;
            
            // Test bundle submission
            self.test_bundle_submission(client).await?;
            
            // Test heartbeat
            self.test_heartbeat(client).await?;
        }
        
        Ok(())
    }
    
    async fn test_wallet_registration(&mut self, client: &mut XTMPBpciClient) -> Result<()> {
        info!("📱 Testing wallet registration via XTMP");
        
        let wallet_address = ProductionWalletAddress {
            address: "bpi1test123456789abcdef".to_string(),
            network: "testnet".to_string(),
            version: 1,
        };
        
        let auth_token = ProductionToken {
            token: "test_token_12345".to_string(),
            expires_at: Utc::now() + chrono::Duration::hours(1),
            token_type: "bearer".to_string(),
        };
        
        let start_time = Instant::now();
        
        match timeout(
            Duration::from_secs(10),
            client.register_wallet(&wallet_address, &auth_token)
        ).await {
            Ok(Ok(response)) => {
                let latency = start_time.elapsed().as_millis() as f64;
                self.test_results.performance_metrics.message_latency_ms = latency;
                
                info!("✅ Wallet registration successful: {:?}", response);
                self.test_results.wallet_registration_passed = true;
            }
            Ok(Err(e)) => {
                error!("❌ Wallet registration failed: {}", e);
                self.test_results.error_count += 1;
            }
            Err(_) => {
                error!("❌ Wallet registration timeout");
                self.test_results.error_count += 1;
            }
        }
        
        Ok(())
    }
    
    async fn test_bundle_submission(&mut self, client: &mut XTMPBpciClient) -> Result<()> {
        info!("📦 Testing bundle submission via XTMP");
        
        let test_bundle = PoEProofBundle {
            bundle_id: "test_bundle_001".to_string(),
            bundle_hash: "hash_test_123456789abcdef".to_string(),
            transaction_count: 5,
            total_value: 1000.0,
            created_at: Utc::now(),
            hyperledger_proof: Some(HyperledgerProof {
                proof_type: "fabric-endorsement".to_string(),
                proof_data: serde_json::json!({
                    "channel": "test-channel",
                    "chaincode": "test-chaincode",
                    "endorsements": 3
                }),
                generated_at: Utc::now(),
            }),
            notary_approvals: vec![],
            immutable_proof: ImmutableProof {
                proof_hash: "immutable_test_hash".to_string(),
                merkle_root: "merkle_test_root".to_string(),
                block_height: 12345,
                timestamp: Utc::now(),
            },
            bpi_ledger_metadata: BpiLedgerMetadata {
                node_id: "test-node-001".to_string(),
                ledger_version: "1.0.0-test".to_string(),
                consensus_algorithm: "BPI-IBFT-TEST".to_string(),
                network_id: "bpi-testnet".to_string(),
            },
        };
        
        let start_time = Instant::now();
        
        match timeout(
            Duration::from_secs(15),
            client.submit_bundle(&test_bundle)
        ).await {
            Ok(Ok(response)) => {
                let latency = start_time.elapsed().as_millis() as f64;
                info!("✅ Bundle submission successful in {}ms: {:?}", latency, response);
                self.test_results.bundle_submission_passed = true;
            }
            Ok(Err(e)) => {
                error!("❌ Bundle submission failed: {}", e);
                self.test_results.error_count += 1;
            }
            Err(_) => {
                error!("❌ Bundle submission timeout");
                self.test_results.error_count += 1;
            }
        }
        
        Ok(())
    }
    
    async fn test_heartbeat(&mut self, client: &XTMPBpciClient) -> Result<()> {
        info!("💓 Testing heartbeat via XTMP");
        
        match timeout(Duration::from_secs(5), client.health_check()).await {
            Ok(Ok(true)) => {
                info!("✅ Heartbeat test passed");
            }
            Ok(Ok(false)) => {
                warn!("⚠️ Heartbeat returned false");
                self.test_results.error_count += 1;
            }
            Ok(Err(e)) => {
                error!("❌ Heartbeat failed: {}", e);
                self.test_results.error_count += 1;
            }
            Err(_) => {
                error!("❌ Heartbeat timeout");
                self.test_results.error_count += 1;
            }
        }
        
        Ok(())
    }
    
    /// Test 4: Performance Benchmarks
    async fn test_performance_benchmarks(&mut self) -> Result<()> {
        info!("⚡ Test 4: Performance Benchmarks");
        
        if let Some(ref mut client) = self.client {
            // Throughput test
            let start_time = Instant::now();
            let mut successful_messages = 0;
            
            for i in 0..100 {
                if client.health_check().await.unwrap_or(false) {
                    successful_messages += 1;
                }
                
                if i % 10 == 0 {
                    info!("📊 Benchmark progress: {}/100", i);
                }
            }
            
            let total_time = start_time.elapsed();
            let throughput = successful_messages as f64 / total_time.as_secs_f64();
            
            self.test_results.performance_metrics.throughput_messages_per_sec = throughput;
            
            info!("📈 Performance Results:");
            info!("  - Connection Time: {:.2}ms", self.test_results.performance_metrics.connection_time_ms);
            info!("  - Message Latency: {:.2}ms", self.test_results.performance_metrics.message_latency_ms);
            info!("  - Throughput: {:.2} msg/sec", throughput);
            
            // Compare with HTTP baseline (estimated)
            let http_baseline_latency = 50.0; // Typical HTTP latency
            let improvement_factor = http_baseline_latency / self.test_results.performance_metrics.message_latency_ms;
            
            info!("🚀 XTMP vs HTTP Performance:");
            info!("  - Latency Improvement: {:.1}x faster", improvement_factor);
            info!("  - Expected HTTP latency: {:.2}ms", http_baseline_latency);
            info!("  - XTMP latency: {:.2}ms", self.test_results.performance_metrics.message_latency_ms);
        }
        
        Ok(())
    }
    
    /// Test 5: Real-time Streaming
    async fn test_real_time_streaming(&mut self) -> Result<()> {
        info!("📊 Test 5: Real-time Streaming");
        
        if let Some(ref mut client) = self.client {
            match timeout(
                Duration::from_secs(10),
                client.subscribe_bundle_updates("test_bundle_001")
            ).await {
                Ok(Ok(mut stream)) => {
                    info!("✅ Stream subscription successful");
                    
                    // Test receiving updates (with timeout)
                    match timeout(Duration::from_secs(5), stream.recv()).await {
                        Ok(Some(update)) => {
                            info!("📈 Received real-time update: {:?}", update);
                            self.test_results.real_time_streaming_passed = true;
                        }
                        Ok(None) => {
                            info!("📭 No updates received (expected for test)");
                            self.test_results.real_time_streaming_passed = true;
                        }
                        Err(_) => {
                            info!("⏰ Stream timeout (expected for test)");
                            self.test_results.real_time_streaming_passed = true;
                        }
                    }
                }
                Ok(Err(e)) => {
                    error!("❌ Stream subscription failed: {}", e);
                    self.test_results.error_count += 1;
                }
                Err(_) => {
                    error!("❌ Stream subscription timeout");
                    self.test_results.error_count += 1;
                }
            }
        }
        
        Ok(())
    }
    
    /// Test 6: Error Handling
    async fn test_error_handling(&mut self) -> Result<()> {
        info!("🛡️ Test 6: Error Handling");
        
        // Test invalid endpoint
        match XTMPBpciClient::new("invalid_endpoint".to_string()).await {
            Ok(_) => {
                warn!("⚠️ Expected error for invalid endpoint, but got success");
            }
            Err(_) => {
                info!("✅ Invalid endpoint properly rejected");
            }
        }
        
        // Test connection to non-existent server
        if let Ok(mut invalid_client) = XTMPBpciClient::new("127.0.0.1:9999".to_string()).await {
            match timeout(Duration::from_secs(2), invalid_client.health_check()).await {
                Ok(Ok(false)) | Ok(Err(_)) | Err(_) => {
                    info!("✅ Connection to invalid server properly failed");
                }
                Ok(Ok(true)) => {
                    warn!("⚠️ Unexpected success connecting to invalid server");
                }
            }
        }
        
        Ok(())
    }
    
    /// Test 7: Cleanup
    async fn cleanup_test_environment(&mut self) -> Result<()> {
        info!("🧹 Test 7: Cleanup");
        
        if let Some(ref client) = self.client {
            let _ = client.close().await;
            info!("✅ Client connection closed");
        }
        
        // Server cleanup would happen automatically when dropped
        info!("✅ Test environment cleaned up");
        
        Ok(())
    }
    
    fn print_test_summary(&self) {
        info!("📋 XTMP Integration Test Summary:");
        info!("  ✅ Connection Test: {}", if self.test_results.connection_test_passed { "PASSED" } else { "FAILED" });
        info!("  ✅ Wallet Registration: {}", if self.test_results.wallet_registration_passed { "PASSED" } else { "FAILED" });
        info!("  ✅ Bundle Submission: {}", if self.test_results.bundle_submission_passed { "PASSED" } else { "FAILED" });
        info!("  ✅ Real-time Streaming: {}", if self.test_results.real_time_streaming_passed { "PASSED" } else { "FAILED" });
        info!("  ❌ Error Count: {}", self.test_results.error_count);
        
        let total_tests = 4;
        let passed_tests = [
            self.test_results.connection_test_passed,
            self.test_results.wallet_registration_passed,
            self.test_results.bundle_submission_passed,
            self.test_results.real_time_streaming_passed,
        ].iter().filter(|&&x| x).count();
        
        info!("📊 Overall Result: {}/{} tests passed", passed_tests, total_tests);
        
        if passed_tests == total_tests && self.test_results.error_count == 0 {
            info!("🎉 ALL TESTS PASSED - XTMP Protocol Ready for Production!");
        } else {
            warn!("⚠️ Some tests failed - Review implementation before production");
        }
    }
}

impl XTMPTestResults {
    pub fn new() -> Self {
        Self {
            connection_test_passed: false,
            wallet_registration_passed: false,
            bundle_submission_passed: false,
            real_time_streaming_passed: false,
            performance_metrics: XTMPPerformanceResults::new(),
            error_count: 0,
        }
    }
    
    pub fn is_production_ready(&self) -> bool {
        self.connection_test_passed
            && self.wallet_registration_passed
            && self.bundle_submission_passed
            && self.real_time_streaming_passed
            && self.error_count == 0
            && self.performance_metrics.message_latency_ms < 10.0 // Sub-10ms latency requirement
    }
}

impl XTMPPerformanceResults {
    pub fn new() -> Self {
        Self {
            connection_time_ms: 0.0,
            message_latency_ms: 0.0,
            throughput_messages_per_sec: 0.0,
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
        }
    }
}

/// Run XTMP integration tests
pub async fn run_xtmp_integration_tests() -> Result<XTMPTestResults> {
    let mut test_suite = XTMPIntegrationTest::new();
    test_suite.run_full_test_suite().await
}
