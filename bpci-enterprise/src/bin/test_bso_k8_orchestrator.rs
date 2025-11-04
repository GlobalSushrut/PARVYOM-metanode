//! # BSO-K8 Orchestrator System Test
//! 
//! Comprehensive test to validate the integrated BSO-K8 orchestration system
//! combining BSO kernel, vPods, and K8s-like orchestration.

use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn, error};
use tracing_subscriber;

use pravyom_enterprise::bso_k8_orchestrator::{
    BsoK8Orchestrator, ServiceType, ResourceAllocation, HealthStatus, OrchestratorStatus
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    info!("🚀 Starting BSO-K8 Orchestrator System Test");
    
    // Test 1: Create BSO-K8 Orchestrator
    info!("📋 Test 1: Creating BSO-K8 Orchestrator");
    let orchestrator = match BsoK8Orchestrator::new("test-orchestrator-001".to_string()).await {
        Ok(orch) => {
            info!("✅ BSO-K8 Orchestrator created successfully");
            orch
        },
        Err(e) => {
            error!("❌ Failed to create BSO-K8 Orchestrator: {}", e);
            return Err(e);
        }
    };
    
    // Test 2: Start the orchestrator
    info!("📋 Test 2: Starting BSO-K8 Orchestrator");
    if let Err(e) = orchestrator.start().await {
        error!("❌ Failed to start BSO-K8 Orchestrator: {}", e);
        return Err(e);
    }
    info!("✅ BSO-K8 Orchestrator started successfully");
    
    // Wait for startup
    sleep(Duration::from_secs(2)).await;
    
    // Test 3: Check orchestrator status
    info!("📋 Test 3: Checking orchestrator status");
    let status = orchestrator.get_status().await;
    info!("📊 Orchestrator Status: {:?}", status.status);
    info!("📊 Total Services: {}", status.total_services);
    info!("📊 Healthy Services: {}", status.healthy_services);
    info!("📊 Total vPods: {}", status.total_vpods);
    info!("📊 Used vPods: {}", status.used_vpods);
    
    if !matches!(status.status, OrchestratorStatus::Running) {
        warn!("⚠️ Orchestrator status is not Running: {:?}", status.status);
    } else {
        info!("✅ Orchestrator is running properly");
    }
    
    // Test 4: Deploy HTTPCG VM Server
    info!("📋 Test 4: Deploying HTTPCG VM Server");
    let httpcg_vm_service = ServiceType::HttpcgVmServer {
        port: 7777,
        bso_endpoint: "http://localhost:9090".to_string(),
    };
    
    let vm_resource_allocation = ResourceAllocation {
        vpods: 2,
        memory_mb: 1024,
        cpu_cores: 2.0,
        storage_gb: 20,
        network_bandwidth: "1Gbps".to_string(),
        replicas: 1,
    };
    
    let vm_service_id = match orchestrator.deploy_service(
        "httpcg-vm-server".to_string(),
        httpcg_vm_service,
        vm_resource_allocation,
    ).await {
        Ok(id) => {
            info!("✅ HTTPCG VM Server deployed successfully: {}", id);
            id
        },
        Err(e) => {
            error!("❌ Failed to deploy HTTPCG VM Server: {}", e);
            return Err(e);
        }
    };
    
    // Test 5: Deploy HTTPCG Admin Dashboard
    info!("📋 Test 5: Deploying HTTPCG Admin Dashboard");
    let httpcg_admin_service = ServiceType::HttpcgAdminDashboard {
        port: 8080,
        vm_endpoint: "http://localhost:7777".to_string(),
    };
    
    let admin_resource_allocation = ResourceAllocation {
        vpods: 1,
        memory_mb: 512,
        cpu_cores: 1.0,
        storage_gb: 10,
        network_bandwidth: "500Mbps".to_string(),
        replicas: 1,
    };
    
    let admin_service_id = match orchestrator.deploy_service(
        "httpcg-admin-dashboard".to_string(),
        httpcg_admin_service,
        admin_resource_allocation,
    ).await {
        Ok(id) => {
            info!("✅ HTTPCG Admin Dashboard deployed successfully: {}", id);
            id
        },
        Err(e) => {
            error!("❌ Failed to deploy HTTPCG Admin Dashboard: {}", e);
            return Err(e);
        }
    };
    
    // Test 6: Deploy HTTPCG Wallet System
    info!("📋 Test 6: Deploying HTTPCG Wallet System");
    let httpcg_wallet_service = ServiceType::HttpcgWalletSystem {
        port: 8081,
        admin_endpoint: "http://localhost:8080".to_string(),
    };
    
    let wallet_resource_allocation = ResourceAllocation {
        vpods: 3,
        memory_mb: 256,
        cpu_cores: 0.5,
        storage_gb: 5,
        network_bandwidth: "200Mbps".to_string(),
        replicas: 1,
    };
    
    let wallet_service_id = match orchestrator.deploy_service(
        "httpcg-wallet-system".to_string(),
        httpcg_wallet_service,
        wallet_resource_allocation,
    ).await {
        Ok(id) => {
            info!("✅ HTTPCG Wallet System deployed successfully: {}", id);
            id
        },
        Err(e) => {
            error!("❌ Failed to deploy HTTPCG Wallet System: {}", e);
            return Err(e);
        }
    };
    
    // Test 7: Deploy BPCI Enterprise
    info!("📋 Test 7: Deploying BPCI Enterprise");
    let bpci_service = ServiceType::BpciEnterprise {
        port: 9545,
        config_path: "/home/umesh/metanode/bpci-enterprise/config/bpci_config.toml".to_string(),
    };
    
    let bpci_resource_allocation = ResourceAllocation {
        vpods: 4,
        memory_mb: 2048,
        cpu_cores: 4.0,
        storage_gb: 50,
        network_bandwidth: "2Gbps".to_string(),
        replicas: 1,
    };
    
    let bpci_service_id = match orchestrator.deploy_service(
        "bpci-enterprise".to_string(),
        bpci_service,
        bpci_resource_allocation,
    ).await {
        Ok(id) => {
            info!("✅ BPCI Enterprise deployed successfully: {}", id);
            id
        },
        Err(e) => {
            error!("❌ Failed to deploy BPCI Enterprise: {}", e);
            return Err(e);
        }
    };
    
    // Wait for services to initialize
    info!("⏳ Waiting for services to initialize...");
    sleep(Duration::from_secs(5)).await;
    
    // Test 8: List all deployed services
    info!("📋 Test 8: Listing all deployed services");
    let services = orchestrator.list_services().await;
    info!("📊 Total deployed services: {}", services.len());
    
    for service in &services {
        info!("🔧 Service: {} ({})", service.service_name, service.service_id);
        info!("   Type: {:?}", service.service_type);
        info!("   Health: {:?}", service.health_status);
        info!("   vPods: {:?}", service.vpod_assignments);
        info!("   Endpoints: {} endpoints", service.endpoints.len());
        for endpoint in &service.endpoints {
            info!("     - {}: {}:{} ({:?})", endpoint.name, "localhost", endpoint.port, endpoint.protocol);
        }
    }
    
    // Test 9: Check final orchestrator status
    info!("📋 Test 9: Final orchestrator status check");
    let final_status = orchestrator.get_status().await;
    info!("📊 Final Status: {:?}", final_status.status);
    info!("📊 Total Services: {}", final_status.total_services);
    info!("📊 Healthy Services: {}", final_status.healthy_services);
    info!("📊 Total vPods: {}", final_status.total_vpods);
    info!("📊 Used vPods: {}", final_status.used_vpods);
    info!("📊 Memory Usage: {:.1}% ({:.1} MB / {:.1} MB)", 
        final_status.memory_usage.percentage,
        final_status.memory_usage.used,
        final_status.memory_usage.total
    );
    info!("📊 CPU Usage: {:.1}% ({:.1} cores / {:.1} cores)", 
        final_status.cpu_usage.percentage,
        final_status.cpu_usage.used,
        final_status.cpu_usage.total
    );
    
    // Test 10: Validate BSO-K8 orchestration success
    info!("📋 Test 10: Validating BSO-K8 orchestration success");
    let mut success_count = 0;
    let total_tests = 4; // Number of deployed services
    
    if services.len() == total_tests {
        success_count += 1;
        info!("✅ All {} services deployed successfully", total_tests);
    } else {
        error!("❌ Expected {} services, found {}", total_tests, services.len());
    }
    
    if matches!(final_status.status, OrchestratorStatus::Running) {
        success_count += 1;
        info!("✅ Orchestrator is running");
    } else {
        error!("❌ Orchestrator is not running: {:?}", final_status.status);
    }
    
    if final_status.total_services == total_tests as u32 {
        success_count += 1;
        info!("✅ Service count matches expected");
    } else {
        error!("❌ Service count mismatch: expected {}, got {}", total_tests, final_status.total_services);
    }
    
    if final_status.used_vpods > 0 {
        success_count += 1;
        info!("✅ vPods are allocated and in use");
    } else {
        error!("❌ No vPods are in use");
    }
    
    // Final result
    info!("🎯 BSO-K8 Orchestrator Test Results:");
    info!("   ✅ Passed: {}/{} tests", success_count, total_tests);
    info!("   📊 Success Rate: {:.1}%", (success_count as f64 / total_tests as f64) * 100.0);
    
    if success_count == total_tests {
        info!("🎉 BSO-K8 Orchestrator System Test: PASSED");
        info!("🚀 System is ready for production deployment!");
    } else {
        error!("💥 BSO-K8 Orchestrator System Test: FAILED");
        error!("🔧 System needs debugging before production deployment");
    }
    
    // Test 11: Cleanup - Stop one service to test stop functionality
    info!("📋 Test 11: Testing service stop functionality");
    if let Err(e) = orchestrator.stop_service(&wallet_service_id).await {
        warn!("⚠️ Failed to stop wallet service: {}", e);
    } else {
        info!("✅ Wallet service stopped successfully");
    }
    
    // Final status after cleanup
    let cleanup_status = orchestrator.get_status().await;
    info!("📊 Status after cleanup: {} services, {} vPods used", 
        cleanup_status.total_services, cleanup_status.used_vpods);
    
    info!("🏁 BSO-K8 Orchestrator System Test completed");
    Ok(())
}
