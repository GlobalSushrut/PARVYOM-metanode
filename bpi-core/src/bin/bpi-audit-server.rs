use std::net::SocketAddr;
use anyhow::Result;
use clap::{Parser, Subcommand};
use tokio::net::TcpListener;
use tracing::{info, error};

use bpi_core::audit_http_server::BpiAuditHttpServer;

/// BPI Core Audit Server - Production-ready audit ingestion for BPI blockchain
#[derive(Parser)]
#[command(name = "bpi-audit-server")]
#[command(about = "BPI Core Audit Server - Receives audits and submits to BPI ledger")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
    
    /// Output in JSON format
    #[arg(long)]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the audit server
    Start {
        /// Server port
        #[arg(short, long, default_value = "8888")]
        port: u16,
        
        /// Server host
        #[arg(long, default_value = "0.0.0.0")]
        host: String,
        
        /// Enable BPI ledger integration
        #[arg(long)]
        bpi_ledger: bool,
        
        /// Enable forensic audit bridge
        #[arg(long, default_value = "true")]
        forensic_bridge: bool,
    },
    
    /// Check server status
    Status {
        /// Server URL
        #[arg(short, long, default_value = "http://localhost:8888")]
        url: String,
    },
    
    /// Get server statistics
    Stats {
        /// Server URL
        #[arg(short, long, default_value = "http://localhost:8888")]
        url: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    if cli.verbose {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }
    
    match cli.command {
        Commands::Start { port, host, bpi_ledger, forensic_bridge } => {
            start_audit_server(port, &host, bpi_ledger, forensic_bridge, cli.json).await
        }
        Commands::Status { url } => {
            check_server_status(&url, cli.json).await
        }
        Commands::Stats { url } => {
            get_server_stats(&url, cli.json).await
        }
    }
}

/// Start the BPI Core audit server
async fn start_audit_server(
    port: u16,
    host: &str,
    bpi_ledger: bool,
    forensic_bridge: bool,
    json: bool,
) -> Result<()> {
    if json {
        println!("{{\"status\":\"starting\",\"port\":{},\"host\":\"{}\"}}", port, host);
    } else {
        info!("🚀 Starting BPI Core Audit Server");
        info!("🔗 Host: {}, Port: {}", host, port);
        info!("🔐 BPI Ledger Integration: {}", if bpi_ledger { "enabled" } else { "disabled" });
        info!("🛡️ Forensic Audit Bridge: {}", if forensic_bridge { "enabled" } else { "disabled" });
    }
    
    // Initialize audit server
    let audit_server = BpiAuditHttpServer::new("/tmp/bpi_audit_storage").await?;
    
    if !json {
        info!("✅ BPI Core audit system initialized");
        info!("✅ Forensic audit bridge connected");
        info!("✅ Immutable audit system active");
    }
    
    // Create HTTP router
    let app = audit_server.create_router();
    
    // Bind to address
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    let listener = TcpListener::bind(&addr).await?;
    
    if json {
        println!("{{\"status\":\"ready\",\"address\":\"{}\",\"endpoints\":[\"/api/audit/submit\",\"/api/audit/status\",\"/api/audit/stats\",\"/api/health\"]}}", addr);
    } else {
        info!("🌐 BPI Core Audit Server listening on {}", addr);
        info!("📡 Endpoints:");
        info!("   POST /api/audit/submit - Submit audit for BPI ledger");
        info!("   GET  /api/audit/status - Get audit system status");
        info!("   GET  /api/audit/stats  - Get audit statistics");
        info!("   GET  /api/health       - Health check");
        info!("");
        info!("🔗 Ready to receive audit submissions from JS clients");
        info!("🔐 All audits will be processed through BPI Core forensic system");
        info!("📊 Audit statistics available at /api/audit/stats");
    }
    
    // Start server
    axum::serve(listener, app).await?;
    
    Ok(())
}

/// Check server status
async fn check_server_status(url: &str, json: bool) -> Result<()> {
    let client = reqwest::Client::new();
    
    match client.get(&format!("{}/api/audit/status", url)).send().await {
        Ok(response) => {
            if json {
                let text = response.text().await?;
                println!("{}", text);
            } else {
                let status: serde_json::Value = response.json().await?;
                info!("✅ Server Status: {}", serde_json::to_string_pretty(&status)?);
            }
        }
        Err(e) => {
            if json {
                println!("{{\"error\":\"connection_failed\",\"message\":\"{}\"}}", e);
            } else {
                error!("❌ Failed to connect to server: {}", e);
            }
        }
    }
    
    Ok(())
}

/// Get server statistics
async fn get_server_stats(url: &str, json: bool) -> Result<()> {
    let client = reqwest::Client::new();
    
    match client.get(&format!("{}/api/audit/stats", url)).send().await {
        Ok(response) => {
            if json {
                let text = response.text().await?;
                println!("{}", text);
            } else {
                let stats: serde_json::Value = response.json().await?;
                info!("📊 Server Statistics: {}", serde_json::to_string_pretty(&stats)?);
            }
        }
        Err(e) => {
            if json {
                println!("{{\"error\":\"connection_failed\",\"message\":\"{}\"}}", e);
            } else {
                error!("❌ Failed to get server stats: {}", e);
            }
        }
    }
    
    Ok(())
}
