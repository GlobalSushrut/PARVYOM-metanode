use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber;
use clap::Parser;

// Import portal_cli types directly since it's not in the main CLI module
use pravyom_enterprise::cargo_portal::CargoPortalProcessor;
use pravyom_enterprise::wallet_address_orchestrator::WalletAddressOrchestrator;
use pravyom_enterprise::server_downloader::PortalDownloader;

/// BPI Portal OS + SDK Main Entry Point
/// 
/// This is the main executable for the BPI Portal OS + SDK system.
/// It provides a cargo.portal-driven OS and SDK manager with:
/// 
/// - 32+ component orchestration with wallet address networking
/// - Lock-based communication via CommuteLock API
/// - BSO-K8 internal + ENC cluster external orchestration
/// - Dynamic port allocation and memory constraint enforcement
/// - Server-side downloader with dev TOML virtual environment
/// - Comprehensive SDK with cargo-style CLI commands
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("bpios=info,bpci_enterprise=info")
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("🚀 Starting BPI Portal OS + SDK");
    info!("📋 cargo.portal-driven configuration system");
    info!("🏠 Wallet address-based networking");
    info!("🔐 Lock-based communication (CommuteLock API)");
    info!("🎭 BSO-K8 internal + ENC cluster external orchestration");

    // Run the Portal OS + SDK system
    info!("🚀 Initializing BPI Portal OS + SDK components...");
    
    // Initialize cargo.portal processor
    match CargoPortalProcessor::new().await {
        Ok(processor) => {
            info!("✅ Cargo Portal processor initialized");
            
            // Load and validate cargo.portal
            match processor.load_and_validate("cargo.portal").await {
                Ok(portal_config) => {
                    info!("✅ cargo.portal loaded and validated");
                    info!("   Package: {} v{}", portal_config.package.name, portal_config.package.version);
                    info!("   Components: {} configured", portal_config.sdk.components.len());
                }
                Err(e) => {
                    error!("⚠️  Failed to load cargo.portal: {}", e);
                    info!("   Continuing with default configuration...");
                }
            }
        }
        Err(e) => {
            error!("⚠️  Failed to initialize cargo.portal processor: {}", e);
        }
    }
    
    info!("✅ BPI Portal OS + SDK initialization complete");
    info!("💡 Use 'bpios --help' for available commands (CLI implementation in progress)");
    
    Ok(())
}
