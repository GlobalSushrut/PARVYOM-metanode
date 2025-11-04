//! # BSO-K8 Production Orchestrator
//! 
//! Production-ready BSO-K8 orchestrator for BPCI Enterprise deployment
//! Integrates BSO kernel + vPod infrastructure + K8s-like orchestration

use anyhow::Result;
use clap::{Arg, Command};
use std::sync::Arc;
use tokio::signal;
use tracing::{info, error};
use tracing_subscriber;

use pravyom_enterprise::bso_k8_orchestrator::{
    BsoK8Orchestrator, ServiceType, ResourceAllocation
};
use warp::Filter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🚀 Starting BSO-K8 Production Orchestrator for BPCI Enterprise");
    info!("🧬 Revolutionary vPod orchestration with cellular replication");

    let matches = Command::new("BSO-K8 Production Orchestrator")
        .version("1.0.0")
        .author("BPCI Enterprise Team")
        .about("Production BSO-K8 orchestrator for revolutionary BPCI deployment")
        .arg(
            Arg::new("orchestrator-id")
                .long("orchestrator-id")
                .value_name("ID")
                .help("Unique orchestrator identifier")
                .default_value("bpci-production-orchestrator")
        )
        .arg(
            Arg::new("port")
                .long("port")
                .value_name("PORT")
                .help("API server port")
                .default_value("9090")
        )
        .arg(
            Arg::new("enable-cellular")
                .long("enable-cellular")
                .help("Enable cellular replication features")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let orchestrator_id = matches.get_one::<String>("orchestrator-id").unwrap().clone();
    let port: u16 = matches.get_one::<String>("port").unwrap().parse()?;
    let enable_cellular = matches.get_flag("enable-cellular");

    info!("📋 Configuration:");
    info!("   Orchestrator ID: {}", orchestrator_id);
    info!("   API Port: {}", port);
    info!("   Cellular Replication: {}", if enable_cellular { "ENABLED" } else { "DISABLED" });

    // Create BSO-K8 orchestrator
    info!("🔧 Initializing BSO-K8 orchestrator...");
    let orchestrator = Arc::new(
        BsoK8Orchestrator::new(orchestrator_id.clone()).await
            .map_err(|e| {
                error!("Failed to create BSO-K8 orchestrator: {}", e);
                e
            })?
    );

    info!("✅ BSO-K8 orchestrator initialized successfully");

    // Start the orchestrator
    info!("🚀 Starting orchestrator services...");
    orchestrator.start().await?;
    info!("✅ BSO-K8 orchestrator started successfully");

    // Start API server
    info!("🌐 Starting API server on port {}...", port);
    let api_orchestrator = orchestrator.clone();
    let api_handle = tokio::spawn(async move {
        start_api_server(api_orchestrator, port).await
    });

    // Start health monitoring
    info!("🏥 Starting health monitoring...");
    orchestrator.start_health_monitoring().await?;

    // Start metrics collection
    info!("📊 Starting metrics collection...");
    orchestrator.start_metrics_collection().await?;

    info!("🎉 BSO-K8 Production Orchestrator is READY!");
    info!("🔗 API endpoint: http://0.0.0.0:{}", port);
    info!("📈 Status endpoint: http://0.0.0.0:{}/api/v1/status", port);
    info!("🧬 vPod management: http://0.0.0.0:{}/api/v1/vpods", port);

    // Wait for shutdown signal
    info!("⏳ Waiting for shutdown signal...");
    signal::ctrl_c().await?;
    
    info!("🛑 Shutdown signal received, stopping orchestrator...");
    
    // Graceful shutdown
    api_handle.abort();
    
    info!("✅ BSO-K8 Production Orchestrator stopped gracefully");
    Ok(())
}

/// Start the API server for orchestrator management
async fn start_api_server(orchestrator: Arc<BsoK8Orchestrator>, port: u16) -> Result<()> {
    use warp::Filter;

    // Status endpoint
    let status = warp::path!("api" / "v1" / "status")
        .and(warp::get())
        .and(with_orchestrator(orchestrator.clone()))
        .and_then(handle_status);

    // Services endpoint
    let services = warp::path!("api" / "v1" / "services")
        .and(warp::get())
        .and(with_orchestrator(orchestrator.clone()))
        .and_then(handle_list_services);

    // Deploy service endpoint
    let deploy = warp::path!("api" / "v1" / "services" / "deploy")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_orchestrator(orchestrator.clone()))
        .and_then(handle_deploy_service);

    // Health endpoint
    let health = warp::path!("health")
        .and(warp::get())
        .map(|| "OK".to_string());

    let routes = status
        .or(services)
        .or(deploy)
        .or(health)
        .with(warp::cors().allow_any_origin());

    info!("🌐 API server starting on 0.0.0.0:{}", port);
    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;

    Ok(())
}

fn with_orchestrator(
    orchestrator: Arc<BsoK8Orchestrator>,
) -> impl Filter<Extract = (Arc<BsoK8Orchestrator>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || orchestrator.clone())
}

async fn handle_status(
    orchestrator: Arc<BsoK8Orchestrator>,
) -> Result<warp::reply::Json, warp::Rejection> {
    let status = orchestrator.get_orchestrator_status();
    Ok(warp::reply::json(&status))
}

async fn handle_list_services(
    orchestrator: Arc<BsoK8Orchestrator>,
) -> Result<warp::reply::Json, warp::Rejection> {
    let services = orchestrator.list_services().await;
    Ok(warp::reply::json(&services))
}

#[derive(serde::Deserialize)]
struct DeployRequest {
    service_name: String,
    service_type: ServiceType,
    resource_allocation: ResourceAllocation,
}

async fn handle_deploy_service(
    request: DeployRequest,
    orchestrator: Arc<BsoK8Orchestrator>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match orchestrator
        .deploy_service(
            request.service_name,
            request.service_type,
            request.resource_allocation,
        )
        .await
    {
        Ok(service_id) => {
            let response = serde_json::json!({
                "status": "success",
                "service_id": service_id,
                "message": "Service deployed successfully"
            });
            Ok(warp::reply::json(&response))
        }
        Err(e) => {
            let response = serde_json::json!({
                "status": "error",
                "message": format!("Failed to deploy service: {}", e)
            });
            Ok(warp::reply::json(&response))
        }
    }
}
