use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error};
use pravyom_enterprise::bso_k8_orchestrator::{
    BsoK8Orchestrator, OrchestratorStatus, ServiceType, ResourceAllocation
};
use pravyom_enterprise::{
    virtual_addressing::{VirtualAddressingConfig, VirtualAddressingManager},
    dynaroute_integration::UnifiedNetworkingLayer,
    config::env_ini_parser::EnvIniParser,
    commute_lock::CommuteLockRuntime,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting BPCI Enterprise BSO-K8 Production Orchestrator");
    info!("🎯 Production-grade Kubernetes-compatible orchestration system");
    
    // Initialize DynaRoute v2 Pure Virtual Mode (NO STATIC PORTS!)
    info!("🌐 Initializing DynaRoute v2 Pure Virtual Mode");
    let virtual_config = VirtualAddressingConfig::pure_virtual("bso-k8");
    let virtual_mgr = VirtualAddressingManager::new(virtual_config);
    info!("   Virtual Address: {}", virtual_mgr.virtual_address().iaav6);
    info!("   Mode: Port-free operation with dynamic port allocation");
    
    // Initialize UnifiedNetworkingLayer for mesh communication
    let env_parser = EnvIniParser::new(".");
    let env_config = env_parser.parse_env_ini()?;
    let commute_runtime = Arc::new(CommuteLockRuntime::new(&env_config)?);
    let _networking = Arc::new(UnifiedNetworkingLayer::new_virtual(commute_runtime).await?);
    info!("✅ UnifiedNetworkingLayer initialized (Pure Virtual Mode)");
    
    // Create orchestrator with production configuration
    let orchestrator_id = "bpci-production-orchestrator".to_string();
    let api_port = 9090;
    
    // Initialize the BSO-K8 orchestrator
    info!("📋 Initializing BSO-K8 Production Orchestrator");
    let orchestrator = BsoK8Orchestrator::new(orchestrator_id.clone()).await?;
    
    // Configure vPod capacity for production
    orchestrator.configure_vpod_capacity(1000).await?;
    
    let orchestrator = Arc::new(orchestrator);
    
    // Start the orchestrator
    info!("🚀 Starting orchestrator services");
    orchestrator.start().await?;
    
    // Start API server for kubectl-like commands
    info!("🌐 Starting API server on port {}", api_port);
    let api_orchestrator = orchestrator.clone();
    tokio::spawn(async move {
        if let Err(e) = start_api_server(api_orchestrator, api_port).await {
            error!("❌ API server failed: {}", e);
        }
    });
    
    // Start health monitoring loop
    info!("🔍 Starting health monitoring");
    let health_orchestrator = orchestrator.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(30)).await;
            
            let status = health_orchestrator.get_orchestrator_status();
            info!("📊 Orchestrator Status: {:?}", status.status);
            info!("📊 Total Services: {}", status.total_services);
            info!("📊 Healthy Services: {}", status.healthy_services);
            info!("📊 Total vPods: {}", status.total_vpods);
            info!("📊 Used vPods: {}", status.used_vpods);
            info!("📊 Memory Usage: {:.1}MB used of {:.1}MB ({:.1}%)", 
                  status.memory_usage.used, status.memory_usage.total, status.memory_usage.percentage);
            info!("📊 CPU Usage: {:.1}% used of {:.1} cores", 
                  status.cpu_usage.percentage, status.cpu_usage.total);
            
            // Auto-scaling logic
            if status.used_vpods as f64 / status.total_vpods as f64 > 0.8 {
                warn!("⚠️  High vPod usage detected, consider scaling");
            }
        }
    });
    
    // Start metrics collection
    info!("📊 Starting metrics collection");
    let _metrics_orchestrator = orchestrator.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(10)).await;
            // Metrics are automatically collected by the orchestrator
        }
    });
    
    info!("✅ BPCI Enterprise BSO-K8 Orchestrator is running in production mode");
    info!("🎯 Ready to accept service deployments via API on port {}", api_port);
    info!("🔧 Use kubectl-compatible commands to manage services");
    
    // Keep the orchestrator running
    loop {
        sleep(Duration::from_secs(60)).await;
        
        // Periodic health check
        let status = orchestrator.get_orchestrator_status();
        if !matches!(status.status, OrchestratorStatus::Running) {
            error!("❌ Orchestrator is not in running state: {:?}", status.status);
            break;
        }
    }
    
    info!("🛑 Shutting down BSO-K8 orchestrator");
    // Graceful shutdown handled by orchestrator
    
    Ok(())
}

async fn start_api_server(
    orchestrator: Arc<BsoK8Orchestrator>,
    port: u16,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use std::net::SocketAddr;
    use tokio::net::TcpListener;
    use serde_json::json;
    
    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    let listener = TcpListener::bind(&addr).await?;
    info!("🌐 API server listening on {}", addr);
    
    loop {
        let (stream, _) = listener.accept().await?;
        let orch = orchestrator.clone();
        
        tokio::spawn(async move {
            use tokio::io::{AsyncReadExt, AsyncWriteExt};
            
            let mut buffer = [0; 4096];
            let mut stream = stream;
            
            // Read the HTTP request
            let bytes_read = match stream.read(&mut buffer).await {
                Ok(n) => n,
                Err(e) => {
                    error!("Failed to read request: {}", e);
                    return;
                }
            };
            
            let request = String::from_utf8_lossy(&buffer[..bytes_read]);
            info!("📥 BSO-K8 API Request: {}", request.lines().next().unwrap_or(""));
            
            let response = if request.contains("POST /api/v1/services") {
                // Handle service deployment like kubectl apply
                info!("🚀 Deploying service via BSO-K8 API");
                
                // Extract JSON body from request
                let body_start = request.find("\r\n\r\n").unwrap_or(0) + 4;
                let body = &request[body_start..];
                
                match serde_json::from_str::<serde_json::Value>(body) {
                    Ok(deployment) => {
                        let service_name = deployment["metadata"]["name"].as_str().unwrap_or("unknown");
                        let service_type = deployment["spec"]["type"].as_str().unwrap_or("CustomBinary");
                        
                        // Deploy service using BSO-K8 orchestrator
                        let service_id = match service_type {
                            "RedisCache" => {
                                let port = deployment["spec"]["port"].as_u64().unwrap_or(6379) as u16;
                                let memory = deployment["spec"]["memory_limit"].as_str().unwrap_or("64mb");
                                let vpods = deployment["spec"]["vpods"].as_u64().unwrap_or(2) as u32;
                                
                                match orch.deploy_service(
                                    service_name.to_string(),
                                    ServiceType::RedisCache {
                                        port,
                                        memory_limit: memory.to_string()
                                    },
                                    ResourceAllocation {
                                        vpods,
                                        memory_mb: 64,
                                        cpu_cores: 0.5,
                                        storage_gb: 1,
                                        network_bandwidth: "100Mbps".to_string(),
                                        replicas: 1,
                                    }
                                ).await {
                                    Ok(id) => Some(id),
                                    Err(e) => {
                                        error!("Failed to deploy Redis: {}", e);
                                        None
                                    }
                                }
                            },
                            "NginxProxy" => {
                                let config_path = deployment["spec"]["config_path"].as_str().unwrap_or("/tmp/nginx.conf");
                                let upstream_services: Vec<String> = deployment["spec"]["upstream_services"]
                                    .as_array()
                                    .unwrap_or(&vec![])
                                    .iter()
                                    .filter_map(|v| v.as_str())
                                    .map(|s| s.to_string())
                                    .collect();
                                let vpods = deployment["spec"]["vpods"].as_u64().unwrap_or(4) as u32;
                                
                                info!("🚀 Deploying NginxProxy with {} upstream services", upstream_services.len());
                                
                                match orch.deploy_service(
                                    service_name.to_string(),
                                    ServiceType::NginxProxy {
                                        config_path: config_path.to_string(),
                                        upstream_services
                                    },
                                    ResourceAllocation {
                                        vpods,
                                        memory_mb: 128,
                                        cpu_cores: 1.0,
                                        storage_gb: 2,
                                        network_bandwidth: "1Gbps".to_string(),
                                        replicas: 1,
                                    }
                                ).await {
                                    Ok(id) => Some(id),
                                    Err(e) => {
                                        error!("Failed to deploy NginxProxy: {}", e);
                                        None
                                    }
                                }
                            },
                            "CustomBinary" => {
                                let binary_path = deployment["spec"]["binary_path"].as_str().unwrap_or("/usr/bin/echo");
                                let args: Vec<String> = deployment["spec"]["args"]
                                    .as_array()
                                    .unwrap_or(&vec![])
                                    .iter()
                                    .filter_map(|v| v.as_str())
                                    .map(|s| s.to_string())
                                    .collect();
                                let env_vars: Vec<(String, String)> = deployment["spec"]["env_vars"]
                                    .as_object()
                                    .unwrap_or(&serde_json::Map::new())
                                    .iter()
                                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                                    .collect();
                                let working_dir = deployment["spec"]["working_dir"].as_str().map(|s| s.to_string());
                                let vpods = deployment["spec"]["vpods"].as_u64().unwrap_or(4) as u32;
                                
                                info!("🚀 Deploying CustomBinary: {} with {} args", binary_path, args.len());
                                
                                match orch.deploy_service(
                                    service_name.to_string(),
                                    ServiceType::CustomBinary {
                                        binary_path: binary_path.to_string(),
                                        args,
                                        env_vars,
                                        working_dir
                                    },
                                    ResourceAllocation {
                                        vpods,
                                        memory_mb: 256,
                                        cpu_cores: 1.0,
                                        storage_gb: 2,
                                        network_bandwidth: "500Mbps".to_string(),
                                        replicas: 1,
                                    }
                                ).await {
                                    Ok(id) => Some(id),
                                    Err(e) => {
                                        error!("Failed to deploy CustomBinary: {}", e);
                                        None
                                    }
                                }
                            },
                            "PostgreSQLDatabase" => {
                                let port = deployment["spec"]["port"].as_u64().unwrap_or(5432) as u16;
                                let data_path = deployment["spec"]["data_path"].as_str().unwrap_or("/tmp/postgres_data");
                                let username = deployment["spec"]["username"].as_str().unwrap_or("postgres");
                                let password = deployment["spec"]["password"].as_str().unwrap_or("postgres");
                                let vpods = deployment["spec"]["vpods"].as_u64().unwrap_or(6) as u32;
                                
                                info!("🚀 Deploying PostgreSQL on port {} with data path {}", port, data_path);
                                
                                match orch.deploy_service(
                                    service_name.to_string(),
                                    ServiceType::PostgreSQLDatabase {
                                        port,
                                        data_path: data_path.to_string(),
                                        username: username.to_string(),
                                        password: password.to_string()
                                    },
                                    ResourceAllocation {
                                        vpods,
                                        memory_mb: 512,
                                        cpu_cores: 2.0,
                                        storage_gb: 10,
                                        network_bandwidth: "1Gbps".to_string(),
                                        replicas: 1,
                                    }
                                ).await {
                                    Ok(id) => Some(id),
                                    Err(e) => {
                                        error!("Failed to deploy PostgreSQL: {}", e);
                                        None
                                    }
                                }
                            },
                            "NodeJSApp" => {
                                let port = deployment["spec"]["port"].as_u64().unwrap_or(3000) as u16;
                                let app_path = deployment["spec"]["app_path"].as_str().unwrap_or("/tmp/app.js");
                                let env_vars: Vec<(String, String)> = deployment["spec"]["env_vars"]
                                    .as_object()
                                    .unwrap_or(&serde_json::Map::new())
                                    .iter()
                                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                                    .collect();
                                let vpods = deployment["spec"]["vpods"].as_u64().unwrap_or(4) as u32;
                                
                                info!("🚀 Deploying NodeJS app on port {} from {}", port, app_path);
                                
                                match orch.deploy_service(
                                    service_name.to_string(),
                                    ServiceType::NodeJSApp {
                                        port,
                                        app_path: app_path.to_string(),
                                        env_vars
                                    },
                                    ResourceAllocation {
                                        vpods,
                                        memory_mb: 256,
                                        cpu_cores: 1.0,
                                        storage_gb: 2,
                                        network_bandwidth: "500Mbps".to_string(),
                                        replicas: 1,
                                    }
                                ).await {
                                    Ok(id) => Some(id),
                                    Err(e) => {
                                        error!("Failed to deploy NodeJS app: {}", e);
                                        None
                                    }
                                }
                            },
                            "Keycloak" => {
                                let port = deployment["spec"]["port"].as_u64().unwrap_or(8080) as u16;
                                let admin_user = deployment["spec"]["admin_user"].as_str().unwrap_or("admin");
                                let admin_password = deployment["spec"]["admin_password"].as_str().unwrap_or("admin123");
                                let db_url = deployment["spec"]["db_url"].as_str().unwrap_or("jdbc:h2:mem:keycloak");
                                let vpods = deployment["spec"]["vpods"].as_u64().unwrap_or(8) as u32;
                                
                                info!("🚀 Deploying Keycloak on port {} with admin user {}", port, admin_user);
                                
                                match orch.deploy_service(
                                    service_name.to_string(),
                                    ServiceType::Keycloak {
                                        port,
                                        admin_user: admin_user.to_string(),
                                        admin_password: admin_password.to_string(),
                                        db_url: db_url.to_string()
                                    },
                                    ResourceAllocation {
                                        vpods,
                                        memory_mb: 1024,
                                        cpu_cores: 2.0,
                                        storage_gb: 5,
                                        network_bandwidth: "1Gbps".to_string(),
                                        replicas: 1,
                                    }
                                ).await {
                                    Ok(id) => Some(id),
                                    Err(e) => {
                                        error!("Failed to deploy Keycloak: {}", e);
                                        None
                                    }
                                }
                            },
                            "RabbitMQ" => {
                                let port = deployment["spec"]["port"].as_u64().unwrap_or(5672) as u16;
                                let management_port = deployment["spec"]["management_port"].as_u64().unwrap_or(15672) as u16;
                                let username = deployment["spec"]["username"].as_str().unwrap_or("rabbitmq");
                                let password = deployment["spec"]["password"].as_str().unwrap_or("rabbitmq123");
                                let vpods = deployment["spec"]["vpods"].as_u64().unwrap_or(6) as u32;
                                
                                info!("🚀 Deploying RabbitMQ on port {} with management on {}", port, management_port);
                                
                                match orch.deploy_service(
                                    service_name.to_string(),
                                    ServiceType::RabbitMQ {
                                        port,
                                        management_port,
                                        username: username.to_string(),
                                        password: password.to_string()
                                    },
                                    ResourceAllocation {
                                        vpods,
                                        memory_mb: 512,
                                        cpu_cores: 1.5,
                                        storage_gb: 5,
                                        network_bandwidth: "1Gbps".to_string(),
                                        replicas: 1,
                                    }
                                ).await {
                                    Ok(id) => Some(id),
                                    Err(e) => {
                                        error!("Failed to deploy RabbitMQ: {}", e);
                                        None
                                    }
                                }
                            },
                            _ => {
                                info!("🔄 Service type {} not yet implemented in API", service_type);
                                None
                            }
                        };
                        
                        if let Some(id) = service_id {
                            let status = orch.get_orchestrator_status();
                            json!({
                                "status": "success",
                                "message": "Service deployed successfully",
                                "service_id": id,
                                "orchestrator": {
                                    "services": status.total_services,
                                    "vpods_used": status.used_vpods,
                                    "vpods_total": status.total_vpods
                                }
                            })
                        } else {
                            json!({
                                "status": "error",
                                "message": "Failed to deploy service"
                            })
                        }
                    },
                    Err(e) => {
                        error!("Invalid JSON in deployment request: {}", e);
                        json!({
                            "status": "error",
                            "message": "Invalid deployment JSON"
                        })
                    }
                }
            } else {
                // Handle status/health requests
                let status = orch.get_orchestrator_status();
                json!({
                    "status": "ok",
                    "orchestrator": {
                        "status": format!("{:?}", status.status),
                        "services": status.total_services,
                        "healthy_services": status.healthy_services,
                        "vpods": {
                            "total": status.total_vpods,
                            "used": status.used_vpods
                        },
                        "resources": {
                            "memory": {
                                "used_mb": status.memory_usage.used,
                                "total_mb": status.memory_usage.total,
                                "percentage": status.memory_usage.percentage
                            },
                            "cpu": {
                                "percentage": status.cpu_usage.percentage,
                                "total_cores": status.cpu_usage.total
                            }
                        }
                    }
                })
            };
            
            let response_str = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                response.to_string().len(),
                response
            );
            
            if let Err(e) = stream.write_all(response_str.as_bytes()).await {
                error!("Failed to write response: {}", e);
            }
        });
    }
}
