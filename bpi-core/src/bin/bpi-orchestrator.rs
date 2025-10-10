//! BPI Service Orchestrator Binary - One-Click Deployment

use anyhow::Result;
use bpi_core::bpi_service_orchestrator::{BpiServiceOrchestrator, DeploymentConfig, Environment};
use tracing::{info, error};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🚀 BPI Service Orchestrator Starting...");
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).unwrap_or(&"production".to_string()).clone();
    
    // Create deployment configuration
    let config = DeploymentConfig {
        environment: match mode.as_str() {
            "development" => Environment::Development,
            "testing" => Environment::Testing,
            _ => Environment::Production,
        },
        auto_wallet_connect: true,
        enable_dynamic_auth: true,
        enable_monitoring: true,
        services: std::collections::HashMap::new(),
    };
    
    // Create and run orchestrator
    let orchestrator = BpiServiceOrchestrator::new(config);
    
    match orchestrator.deploy_complete_system().await {
        Ok(()) => {
            info!("✅ BPI Complete Deployment Successful!");
            
            // Keep orchestrator running for monitoring
            info!("📊 Orchestrator running in monitoring mode...");
            info!("🌐 Access Dashboard: http://localhost:8888");
            info!("📊 System Status: http://localhost:9999/status");
            
            // Wait indefinitely (or until Ctrl+C)
            tokio::signal::ctrl_c().await?;
            info!("🛑 Shutdown signal received, stopping services...");
            orchestrator.stop_all_services().await?;
        }
        Err(e) => {
            error!("❌ BPI Deployment Failed: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}
