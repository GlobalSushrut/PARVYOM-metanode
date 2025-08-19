use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;
use rcgen::Certificate;
use tokio::signal;

use bpi_relay::{start_observability_server_on, Relay, RelayConfig};
use bpi_relay::net::QuicServer;

#[derive(Parser, Debug)]
#[command(name = "bpi-relay", about = "BPI Relay service")] 
struct Args {
    /// Metrics/health listen address (e.g., 127.0.0.1:9090)
    #[arg(long, default_value = "127.0.0.1:0")]
    metrics_addr: SocketAddr,

    /// QUIC listen address (e.g., 127.0.0.1:7000)
    #[arg(long, default_value = "127.0.0.1:0")]
    listen: SocketAddr,

    /// Optional RocksDB path for persistent dedup
    #[arg(long)]
    db_path: Option<PathBuf>,

    /// TTL for persistent dedup entries (seconds)
    #[arg(long, default_value_t = 24 * 60 * 60)]
    dedup_ttl: u64,

    /// In-memory dedup cache size
    #[arg(long, default_value_t = 4096)]
    dedup_cache: usize,

    /// Rate limit tokens per second per source
    #[arg(long, default_value_t = 10_000)]
    rate: u32,

    /// Rate limit burst size per source
    #[arg(long)]
    burst: Option<u32>,

    /// Simulated loss probability [0.0, 1.0] (testing)
    #[arg(long, default_value_t = 0.0)]
    loss: f32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Start observability HTTP server
    let metrics_addr = start_observability_server_on(args.metrics_addr).await;
    println!("metrics listening on {}", metrics_addr);

    let cfg = RelayConfig {
        dedup_cache: args.dedup_cache,
        rate_limit_per_sec: args.rate as f64,
        rate_limit_burst: args.burst.unwrap_or(args.rate) as f64,
        loss_probability: args.loss,
        // Stage 19 defaults
        max_clients: 1000,
        anti_eclipse_min_relays: 3,
        partition_recovery_timeout_ms: 2000,
        routing_table_size: 10000,
        connection_timeout_ms: 30000,
    };

    let relay = if let Some(path) = args.db_path {
        Arc::new(tokio::sync::Mutex::new(Relay::new_with_persistent(cfg, path, args.dedup_ttl)))
    } else {
        Arc::new(tokio::sync::Mutex::new(Relay::new(cfg)))
    };

    // QUIC server
    let cert = Arc::new(Certificate::from_params(rcgen::CertificateParams::new(vec!["localhost".into()]))?);
    let server_cfg = {
        // The API uses a self-signed certificate; no extra config here.
        cert.clone()
    };

    // bind; ignore args.listen for now since QuicServer uses localhost binding internally
    // Future improvement: propagate listen addr into QuicServer
    let (_server, addr) = QuicServer::bind_and_run_with_cert(relay.clone(), server_cfg).await?;
    println!("quic listening on {}", addr);

    // Wait for Ctrl+C
    signal::ctrl_c().await?;
    println!("shutting down");
    Ok(())
}
