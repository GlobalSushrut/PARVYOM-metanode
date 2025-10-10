// BPCI XTMP Server Binary - Start the real BPI → BPCI transaction bridge
// Enables high-performance XTMP protocol for BPI Core ↔ BPCI communication

use bpi_core::bpci_xtmp_server::{BpciXtmpServer, BpciXtmpServerConfig};
use clap::{Arg, Command};
use tracing::{info, error};
use std::time::Duration;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let matches = Command::new("bpci-xtmp-server")
        .version("1.0.0")
        .about("BPCI XTMP Server - Real BPI → BPCI transaction bridge")
        .arg(
            Arg::new("port")
                .long("port")
                .value_name("PORT")
                .help("Port to bind the XTMP server to")
                .default_value("7778")
        )
        .arg(
            Arg::new("host")
                .long("host")
                .value_name("HOST")
                .help("Host address to bind to")
                .default_value("127.0.0.1")
        )
        .arg(
            Arg::new("max-connections")
                .long("max-connections")
                .value_name("COUNT")
                .help("Maximum number of concurrent connections")
                .default_value("1000")
        )
        .get_matches();

    let port = matches.get_one::<String>("port").unwrap();
    let host = matches.get_one::<String>("host").unwrap();
    let max_connections = matches.get_one::<String>("max-connections").unwrap()
        .parse::<usize>()
        .unwrap_or(1000);

    let bind_address = format!("{}:{}", host, port);

    info!("🚀 Starting BPCI XTMP Server for real BPI → BPCI transaction bridge");
    info!("📡 Bind address: {}", bind_address);
    info!("🔗 Max connections: {}", max_connections);

    // Create XTMP server configuration
    let config = BpciXtmpServerConfig {
        bind_address: bind_address.clone(),
        max_connections,
        connection_timeout: Duration::from_secs(30),
        heartbeat_interval: Duration::from_secs(10),
        enable_compression: true,
        enable_real_time_streams: true,
    };

    // Create and start the XTMP server
    match BpciXtmpServer::new(config).await {
        Ok(server) => {
            info!("✅ BPCI XTMP Server initialized successfully");
            info!("🌐 Ready to receive real BPI transactions for BPCI bundle proof creation");
            
            // Start the server (this will block)
            if let Err(e) = server.start().await {
                error!("❌ Failed to start BPCI XTMP Server: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            error!("❌ Failed to initialize BPCI XTMP Server: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
