use std::sync::Arc;
use tokio;
use tracing::{info, error};
use pravyom_enterprise::bso_k8_orchestrator::{
    BsoK8Orchestrator, ServiceType, ResourceAllocation
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🚀 Deploying Service via BSO-K8 Orchestrator - REAL TEST");
    info!("🎯 This will deploy an actual service managed as vPods");
    
    // Create orchestrator instance
    let orchestrator_id = "bpci-production-orchestrator".to_string();
    let orchestrator = Arc::new(BsoK8Orchestrator::new(orchestrator_id).await?);
    
    // Start the orchestrator
    info!("🚀 Starting BSO-K8 orchestrator");
    orchestrator.start().await?;
    
    // Check initial status
    let initial_status = orchestrator.get_status().await;
    info!("📊 Initial Status - Services: {}, vPods: {}/{}", 
          initial_status.total_services, 
          initial_status.used_vpods, 
          initial_status.total_vpods);
    
    // Deploy Redis as a BSO-K8 managed service
    info!("🚀 Deploying Redis Cache via BSO-K8");
    let redis_service_id = orchestrator.deploy_service(
        "redis-bso-managed".to_string(),
        ServiceType::RedisCache {
            port: 6380,
            memory_limit: "64mb".to_string()
        },
        ResourceAllocation {
            vpods: 2,
            memory_mb: 64,
            cpu_cores: 0.5,
            replicas: 1,
            storage_gb: 1,
            network_bandwidth: "100Mbps".to_string(),
        }
    ).await?;
    
    info!("✅ Redis deployed with service ID: {}", redis_service_id);
    
    // Deploy Nginx as a BSO-K8 managed service
    info!("🚀 Deploying Nginx Proxy via BSO-K8");
    let nginx_service_id = orchestrator.deploy_service(
        "nginx-bso-managed".to_string(),
        ServiceType::NginxProxy {
            config_path: "/tmp/nginx-bso.conf".to_string(),
            upstream_services: vec!["redis-bso-managed:6380".to_string()]
        },
        ResourceAllocation {
            vpods: 3,
            memory_mb: 96,
            cpu_cores: 0.3,
            replicas: 1,
            storage_gb: 2,
            network_bandwidth: "200Mbps".to_string(),
        }
    ).await?;
    
    info!("✅ Nginx deployed with service ID: {}", nginx_service_id);
    
    // Check final status
    let final_status = orchestrator.get_status().await;
    info!("📊 Final Status - Services: {}, vPods: {}/{}", 
          final_status.total_services, 
          final_status.used_vpods, 
          final_status.total_vpods);
    
    // Validate deployment
    if final_status.total_services > 0 && final_status.used_vpods > 0 {
        info!("🎉 SUCCESS: BSO-K8 orchestrator is managing {} services with {} vPods!", 
              final_status.total_services, final_status.used_vpods);
        info!("✅ Real BSO-K8 orchestration capabilities CONFIRMED!");
    } else {
        error!("❌ FAILED: Services not properly managed by BSO-K8");
        return Err("BSO-K8 orchestration failed".into());
    }
    
    // List deployed services
    let services = orchestrator.list_services().await;
    info!("📋 Deployed services: {:?}", services);
    
    info!("🚀 BSO-K8 orchestration test completed successfully!");
    
    Ok(())
}
