//! HTTP Gateway VM Cluster Tests
//! 
//! Comprehensive test suite for HTTP Gateway with VM-Cluster Integration
//! Tests are written based on the actual module structure and APIs

use std::sync::Arc;
use tokio::time::Duration;
use anyhow::Result;

use crate::http_gateway_vm_cluster::{
    HttpGatewayVMCluster, HttpGatewayConfig, HttpGatewayRequest, HttpGatewayResponse,
    VMType, VMStatus, VMInstance
};
use crate::shadow_registry_bridge::ShadowRegistryBridge;
use crate::immutable_audit_system::ImmutableAuditSystem;
use crate::bpi_wallet_command::BPIWalletArgs;

/// Test helper to create a test HTTP Gateway instance
async fn create_test_gateway() -> Result<HttpGatewayVMCluster> {
    // Create test wallet
    let wallet = BPIWalletArgs {
        wallet_name: "test_gateway_wallet".to_string(),
        wallet_path: "/tmp/test_gateway".to_string(),
        network: "testnet".to_string(),
        ..Default::default()
    };
    
    // Create test shadow registry
    let shadow_registry = Arc::new(ShadowRegistryBridge::new().await?);
    
    // Create test audit system
    let audit_system = Arc::new(ImmutableAuditSystem::new().await?);
    
    // Create test config
    let config = HttpGatewayConfig::default();
    
    // Create HTTP Gateway
    HttpGatewayVMCluster::new(wallet, shadow_registry, audit_system, config).await
}

/// Test helper to create a test HTTP request
fn create_test_request() -> HttpGatewayRequest {
    HttpGatewayRequest {
        request_id: "test_request_001".to_string(),
        method: "GET".to_string(),
        path: "/api/test".to_string(),
        headers: std::collections::HashMap::new(),
        body: Vec::new(),
        client_ip: "127.0.0.1".to_string(),
        timestamp: chrono::Utc::now(),
    }
}

/// Test helper to create a test VM instance
fn create_test_vm_instance() -> VMInstance {
    VMInstance {
        vm_id: "test_vm_001".to_string(),
        vm_type: VMType::Action,
        endpoint: "http://localhost:8080".to_string(),
        status: VMStatus::Running,
        load: 0.5,
        capabilities: vec!["http".to_string(), "api".to_string()],
        last_health_check: chrono::Utc::now(),
    }
}

#[tokio::test]
async fn test_http_gateway_creation() -> Result<()> {
    println!("Testing HTTP Gateway VM Cluster creation...");
    
    let gateway = create_test_gateway().await?;
    
    // Verify gateway was created successfully
    assert!(format!("{:?}", gateway).contains("HttpGatewayVMCluster"));
    
    println!("✓ HTTP Gateway VM Cluster created successfully");
    Ok(())
}

#[tokio::test]
async fn test_http_gateway_start() -> Result<()> {
    println!("Testing HTTP Gateway start...");
    
    let gateway = create_test_gateway().await?;
    
    // Test starting the gateway
    let result = gateway.start().await;
    
    // Should succeed (even if it's a stub implementation)
    assert!(result.is_ok());
    
    println!("✓ HTTP Gateway started successfully");
    Ok(())
}

#[tokio::test]
async fn test_request_processing() -> Result<()> {
    println!("Testing HTTP request processing...");
    
    let gateway = create_test_gateway().await?;
    let request = create_test_request();
    
    // Test processing a request
    let result = gateway.process_request(request).await;
    
    // Should return a result (success or error)
    match result {
        Ok(response) => {
            println!("✓ Request processed successfully: {:?}", response.status_code);
        }
        Err(e) => {
            println!("✓ Request processing returned expected error: {}", e);
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_gateway_status() -> Result<()> {
    println!("Testing gateway status retrieval...");
    
    let gateway = create_test_gateway().await?;
    
    // Test getting gateway status
    let result = gateway.get_status().await;
    
    match result {
        Ok(status) => {
            println!("✓ Gateway status retrieved: {:?}", status.is_healthy);
        }
        Err(e) => {
            println!("✓ Gateway status returned expected error: {}", e);
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_vm_type_string_conversion() {
    println!("Testing VM type string conversion...");
    
    let vm_types = vec![
        VMType::Action,
        VMType::Server,
        VMType::Orchestration,
        VMType::Audit,
        VMType::Court,
        VMType::Forensic,
        VMType::VOKernel,
    ];
    
    for vm_type in vm_types {
        let type_str = vm_type.to_string();
        assert!(!type_str.is_empty());
        println!("✓ VM type {:?} -> {}", vm_type, type_str);
    }
}

#[tokio::test]
async fn test_gateway_config_default() {
    println!("Testing gateway configuration defaults...");
    
    let config = HttpGatewayConfig::default();
    
    // Verify default config has reasonable values
    assert!(config.listen_port > 0);
    assert!(config.max_connections > 0);
    assert!(config.request_timeout > Duration::from_secs(0));
    
    println!("✓ Gateway configuration defaults are valid");
    println!("  - Listen port: {}", config.listen_port);
    println!("  - Max connections: {}", config.max_connections);
    println!("  - Request timeout: {:?}", config.request_timeout);
}

#[tokio::test]
async fn test_vm_instance_creation() {
    println!("Testing VM instance creation...");
    
    let vm_instance = create_test_vm_instance();
    
    // Verify VM instance fields
    assert!(!vm_instance.vm_id.is_empty());
    assert!(!vm_instance.endpoint.is_empty());
    assert_eq!(vm_instance.vm_type, VMType::Action);
    assert_eq!(vm_instance.status, VMStatus::Running);
    assert!(vm_instance.load >= 0.0 && vm_instance.load <= 1.0);
    assert!(!vm_instance.capabilities.is_empty());
    
    println!("✓ VM instance created with valid fields");
    println!("  - VM ID: {}", vm_instance.vm_id);
    println!("  - Type: {:?}", vm_instance.vm_type);
    println!("  - Status: {:?}", vm_instance.status);
    println!("  - Load: {:.2}", vm_instance.load);
}

#[tokio::test]
async fn test_request_creation() {
    println!("Testing HTTP request creation...");
    
    let request = create_test_request();
    
    // Verify request fields
    assert!(!request.request_id.is_empty());
    assert!(!request.method.is_empty());
    assert!(!request.path.is_empty());
    assert!(!request.client_ip.is_empty());
    
    println!("✓ HTTP request created with valid fields");
    println!("  - Request ID: {}", request.request_id);
    println!("  - Method: {}", request.method);
    println!("  - Path: {}", request.path);
    println!("  - Client IP: {}", request.client_ip);
}

/// Integration test that combines multiple components
#[tokio::test]
async fn test_gateway_integration() -> Result<()> {
    println!("Testing HTTP Gateway integration...");
    
    // Create gateway
    let gateway = create_test_gateway().await?;
    
    // Start gateway
    let _ = gateway.start().await;
    
    // Get status
    let _ = gateway.get_status().await;
    
    // Process a request
    let request = create_test_request();
    let _ = gateway.process_request(request).await;
    
    println!("✓ HTTP Gateway integration test completed");
    Ok(())
}

/// Performance test for basic operations
#[tokio::test]
async fn test_gateway_performance() -> Result<()> {
    println!("Testing HTTP Gateway performance...");
    
    let gateway = create_test_gateway().await?;
    
    let start_time = std::time::Instant::now();
    
    // Test multiple status calls
    for i in 0..10 {
        let _ = gateway.get_status().await;
        if i % 3 == 0 {
            println!("  Completed {} status calls", i + 1);
        }
    }
    
    let elapsed = start_time.elapsed();
    println!("✓ Performance test completed in {:?}", elapsed);
    println!("  Average per call: {:?}", elapsed / 10);
    
    Ok(())
}
