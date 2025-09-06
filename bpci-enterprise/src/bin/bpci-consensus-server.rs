//! BPCI Consensus Server - Testnet Ready
//! 
//! Production-ready BPCI server with enhanced 3rd consensus layer
//! (Tranverse Auction) integrated with IBFT and HotStuff consensus.
//! 
//! Features:
//! - Triple Consensus Architecture coordination
//! - Real-time consensus monitoring via HTTP API
//! - WebSocket streaming for live updates
//! - Testnet mode with mock auction settlement
//! - Development endpoints for testing
//! 
//! Usage:
//!   cargo run --bin bpci-consensus-server
//!   cargo run --bin bpci-consensus-server -- --config testnet.toml
//!   cargo run --bin bpci-consensus-server -- --dev-mode

use anyhow::{anyhow, Result};
use axum::serve;
use clap::{Arg, Command};
use std::net::SocketAddr;
use tokio::signal;
use tracing::{info, warn, error};
use tracing_subscriber;

use pravyom_enterprise::{
    BpciConsensusServerState, BpciServerConfig, ServerMode,
    create_bpci_consensus_router, initialize_bpci_enterprise,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("bpci_consensus_server=info,bpci_enterprise=info")
        .init();
    
    info!("🚀 Starting BPCI Consensus Server with Enhanced 3rd Consensus Layer");
    
    // Parse command line arguments
    let matches = Command::new("bpci-consensus-server")
        .version("1.0.0")
        .about("BPCI Consensus Server with Triple Consensus Architecture")
        .arg(
            Arg::new("config")
                .long("config")
                .value_name("FILE")
                .help("Configuration file path")
                .default_value("config.toml")
        )
        .arg(
            Arg::new("dev-mode")
                .long("dev-mode")
                .help("Run in development mode")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .value_name("PORT")
                .help("Server port")
                .default_value("8080")
        )
        .arg(
            Arg::new("host")
                .long("host")
                .value_name("HOST")
                .help("Server host")
                .default_value("127.0.0.1")
        )
        .get_matches();
    
    // Determine server configuration
    let server_config = if matches.get_flag("dev-mode") {
        info!("🔧 Running in DEVELOPMENT mode");
        BpciServerConfig {
            server_mode: ServerMode::Development {
                auto_generate_bundles: true,
                debug_logging: true,
            },
            listen_address: matches.get_one::<String>("host").unwrap().clone(),
            listen_port: matches.get_one::<String>("port").unwrap().parse()?,
            max_concurrent_rounds: 5,
            round_timeout_seconds: 15,
            enable_websocket_monitoring: true,
            enable_metrics_endpoint: true,
        }
    } else {
        info!("🌐 Running in TESTNET mode");
        BpciServerConfig {
            server_mode: ServerMode::Testnet {
                mock_validators: 5,
                simulate_network_delays: true,
            },
            listen_address: matches.get_one::<String>("host").unwrap().clone(),
            listen_port: matches.get_one::<String>("port").unwrap().parse()?,
            max_concurrent_rounds: 10,
            round_timeout_seconds: 30,
            enable_websocket_monitoring: true,
            enable_metrics_endpoint: true,
        }
    };
    
    // Initialize BPCI Enterprise system
    initialize_bpci_enterprise().await?;
    
    // Create server state
    info!("🔧 Initializing BPCI Consensus Server state...");
    let server_state = BpciConsensusServerState::new(server_config.clone()).await?;
    
    // Create router with all endpoints
    let app = create_bpci_consensus_router(server_state);
    
    // Start server
    let addr = SocketAddr::new(
        server_config.listen_address.parse()?,
        server_config.listen_port,
    );
    
    info!("🎯 BPCI Consensus Server starting on {}", addr);
    info!("📊 API endpoints available:");
    info!("   POST /api/v1/consensus/start - Start new consensus round");
    info!("   GET  /api/v1/consensus/status/:id - Get round status");
    info!("   GET  /api/v1/auction/mode - Get auction mode");
    info!("   GET  /api/v1/metrics - Get consensus metrics");
    info!("   GET  /api/v1/health - Health check");
    
    if server_config.enable_websocket_monitoring {
        info!("   WS   /ws/consensus - Real-time monitoring");
    }
    
    match &server_config.server_mode {
        ServerMode::Development { .. } => {
            info!("🔧 Development endpoints:");
            info!("   POST /api/v1/dev/generate-bundles - Generate test bundles");
            info!("   POST /api/v1/dev/simulate-round - Simulate consensus round");
        }
        ServerMode::Testnet { mock_validators, .. } => {
            info!("🌐 Testnet configuration:");
            info!("   Mock validators: {}", mock_validators);
            info!("   Auction mode: Testnet (mock settlement to BPI DB)");
            info!("   Triple consensus: IBFT + HotStuff + Tranverse Auction");
        }
    }
    
    // Start the server with graceful shutdown
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let server = serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal());
    
    info!("✅ BPCI Consensus Server is running!");
    info!("🎯 Ready to process consensus rounds with enhanced 3rd consensus layer");
    
    // Run server
    if let Err(e) = server.await {
        error!("Server error: {}", e);
        return Err(anyhow!("Server failed: {}", e));
    }
    
    info!("🛑 BPCI Consensus Server shutdown complete");
    Ok(())
}

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("🛑 Received Ctrl+C, shutting down gracefully...");
        },
        _ = terminate => {
            info!("🛑 Received terminate signal, shutting down gracefully...");
        },
    }
}
