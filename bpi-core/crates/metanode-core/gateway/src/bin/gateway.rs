use anyhow::Result;
use bpi_gateway::{GatewayAgent, GatewayConfig, LoadBalancingStrategy};
use clap::Parser;
use serde_json;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, error, Level};
use tracing_subscriber;
use warp::Filter;
use warp::reply::json;

#[derive(Parser, Debug)]
#[command(name = "gateway")]
#[command(about = "BPI Mesh Gateway Agent - Stage 20")]
struct Args {
    /// Gateway ID
    #[arg(long, default_value = "gateway-001")]
    gateway_id: String,

    /// Listen address
    #[arg(long, default_value = "127.0.0.1:8080")]
    listen_addr: SocketAddr,

    /// Relay endpoints (comma-separated)
    #[arg(long, default_value = "http://127.0.0.1:8001,http://127.0.0.1:8002,http://127.0.0.1:8003")]
    relay_endpoints: String,

    /// Health check interval in milliseconds
    #[arg(long, default_value = "5000")]
    health_check_interval_ms: u64,

    /// Maximum connections
    #[arg(long, default_value = "1000")]
    max_connections: usize,

    /// Request timeout in milliseconds
    #[arg(long, default_value = "30000")]
    request_timeout_ms: u64,

    /// Retry attempts
    #[arg(long, default_value = "3")]
    retry_attempts: u32,

    /// Circuit breaker threshold
    #[arg(long, default_value = "5")]
    circuit_breaker_threshold: u32,

    /// Load balancing strategy
    #[arg(long, default_value = "round-robin")]
    load_balancing: String,

    /// Enable sidecar mode
    #[arg(long)]
    sidecar_mode: bool,

    /// Enable metrics
    #[arg(long)]
    metrics_enabled: bool,

    /// Run as daemon service
    #[arg(long)]
    daemon: bool,

    /// Log level
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize tracing
    let log_level = match args.log_level.as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .init();

    info!("Starting BPI Mesh Gateway Agent - Stage 20");
    info!("Gateway ID: {}", args.gateway_id);
    info!("Listen Address: {}", args.listen_addr);
    info!("Sidecar Mode: {}", args.sidecar_mode);

    // Parse relay endpoints
    let relay_endpoints: Vec<String> = args.relay_endpoints
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    // Parse load balancing strategy
    let load_balancing_strategy = match args.load_balancing.as_str() {
        "round-robin" => LoadBalancingStrategy::RoundRobin,
        "least-connections" => LoadBalancingStrategy::LeastConnections,
        "health-based" => LoadBalancingStrategy::HealthBased,
        "random" => LoadBalancingStrategy::Random,
        _ => {
            eprintln!("Invalid load balancing strategy: {}", args.load_balancing);
            std::process::exit(1);
        }
    };

    // Create gateway configuration
    let config = GatewayConfig {
        gateway_id: args.gateway_id,
        listen_addr: args.listen_addr,
        relay_endpoints,
        health_check_interval_ms: args.health_check_interval_ms,
        max_connections: args.max_connections,
        request_timeout_ms: args.request_timeout_ms,
        retry_attempts: args.retry_attempts,
        circuit_breaker_threshold: args.circuit_breaker_threshold,
        load_balancing_strategy,
        sidecar_mode: args.sidecar_mode,
        metrics_enabled: args.metrics_enabled,
    };

    info!("Configuration: {:?}", config);

    // Create gateway agent
    let agent = Arc::new(GatewayAgent::new(config.clone()));
    
    if args.daemon {
        // Run as HTTP daemon service
        start_http_server(agent, config.listen_addr.port()).await?;
    } else {
        // Run demo mode
        if let Err(e) = agent.start().await {
            eprintln!("Gateway Agent failed to start: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn start_http_server(agent: Arc<GatewayAgent>, port: u16) -> Result<()> {
    info!("Starting Gateway HTTP server on port {}", port);

    let agent_filter = warp::any().map(move || agent.clone());

    // GET /health - Health check
    let health = warp::path("health")
        .and(warp::get())
        .and(agent_filter.clone())
        .and_then(|agent: Arc<GatewayAgent>| async move {
            let status = agent.get_gateway_status().await;
            Ok::<_, warp::Rejection>(json(&status))
        });

    // GET /status - Gateway status
    let status = warp::path("status")
        .and(warp::get())
        .and(agent_filter.clone())
        .and_then(|agent: Arc<GatewayAgent>| async move {
            let status = agent.get_gateway_status().await;
            Ok::<_, warp::Rejection>(json(&status))
        });

    // POST /request - Process gateway request
    let request = warp::path("request")
        .and(warp::post())
        .and(warp::body::json())
        .and(agent_filter.clone())
        .and_then(|req: serde_json::Value, agent: Arc<GatewayAgent>| async move {
            // Convert JSON to GatewayRequest
            let gateway_request = bpi_gateway::GatewayRequest {
                id: req.get("request_id").and_then(|v| v.as_str()).unwrap_or("unknown").to_string(),
                method: req.get("method").and_then(|v| v.as_str()).unwrap_or("GET").to_string(),
                path: req.get("path").and_then(|v| v.as_str()).unwrap_or("/").to_string(),
                headers: std::collections::HashMap::new(),
                body: req.get("body").and_then(|v| v.as_str()).unwrap_or("").as_bytes().to_vec(),
                timestamp: chrono::Utc::now(),
            };

            match agent.process_request(gateway_request).await {
                Ok(response) => {
                    Ok::<_, warp::Rejection>(json(&response))
                }
                Err(e) => {
                    error!("Request processing failed: {}", e);
                    Ok(json(&serde_json::json!({
                        "status": "error",
                        "message": format!("Request processing failed: {}", e)
                    })))
                }
            }
        });

    // GET /endpoints - List relay endpoints
    let endpoints = warp::path("endpoints")
        .and(warp::get())
        .and(agent_filter.clone())
        .and_then(|agent: Arc<GatewayAgent>| async move {
            let status = agent.get_gateway_status().await;
            Ok::<_, warp::Rejection>(json(&serde_json::json!({
                "endpoints": status.endpoint_statuses,
                "total_endpoints": status.total_endpoints,
                "healthy_endpoints": status.healthy_endpoints
            })))
        });

    let routes = health
        .or(status)
        .or(request)
        .or(endpoints)
        .with(warp::cors().allow_any_origin().allow_headers(vec!["content-type"]).allow_methods(vec!["GET", "POST"]));

    info!("✅ Gateway HTTP Server running on http://127.0.0.1:{}", port);
    info!("Available endpoints:");
    info!("  GET  /health - Health check");
    info!("  GET  /status - Gateway status");
    info!("  GET  /endpoints - List relay endpoints");
    info!("  POST /request - Process gateway request");

    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;

    Ok(())
}
