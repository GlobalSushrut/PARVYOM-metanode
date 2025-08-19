use super::Result;
use std::process::Stdio;
use tokio::process::Command;
use tokio::time::{sleep, Duration};

/// Start core Metanode services
pub async fn start_core_services(network: &str) -> Result<()> {
    println!("🔧 Starting core services for {} network...", network);
    
    // Start BPCI node
    start_bpci_node(network).await?;
    
    // Start shadow registry
    start_shadow_registry().await?;
    
    // Start economic APIs
    start_economic_apis().await?;
    
    // Start dashboard services
    start_dashboard_services().await?;
    
    // Wait for services to be ready
    wait_for_services().await?;
    
    println!("✅ All core services started successfully!");
    Ok(())
}

async fn start_bpci_node(network: &str) -> Result<()> {
    println!("  📡 Starting BPCI node...");
    
    let mut cmd = Command::new("cargo");
    cmd.args(&["run", "--bin", "bpci-node"])
        .arg("--")
        .arg("--network")
        .arg(network)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    
    let mut child = cmd.spawn()?;
    
    // Store process handle for later management
    let pid = child.id();
    println!("    📋 BPCI node PID: {}", pid);
    
    // Write PID to file for process management
    std::fs::write("/tmp/bpci_node.pid", pid.to_string())?;
    
    // Check if process started successfully
    match child.try_wait()? {
        Some(status) => {
            if !status.success() {
                return Err(anyhow::anyhow!("BPCI node failed to start: {}", status));
            }
        }
        None => {
            // Process is still running, which is good
            println!("    ✅ BPCI node started successfully");
        }
    }
    
    // Detach the process so it continues running
    std::mem::forget(child);
    Ok(())
}

async fn start_shadow_registry() -> Result<()> {
    println!("  🔒 Starting Shadow Registry...");
    
    let mut cmd = Command::new("cargo");
    cmd.args(&["run", "--bin", "shadow-registry"])
        .arg("--")
        .arg("--port")
        .arg("8080")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    
    let child = cmd.spawn()?;
    
    println!("    ✅ Shadow Registry started on port 8080");
    Ok(())
}

async fn start_economic_apis() -> Result<()> {
    println!("  💰 Starting Economic APIs...");
    
    let mut cmd = Command::new("cargo");
    cmd.args(&["run", "--bin", "economic-api"])
        .arg("--")
        .arg("--port")
        .arg("8081")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    
    let child = cmd.spawn()?;
    
    println!("    ✅ Economic APIs started on port 8081");
    Ok(())
}

async fn start_dashboard_services() -> Result<()> {
    println!("  🎨 Starting Dashboard services...");
    
    // Start BPCI Dashboard
    start_dashboard("bpci", 3000).await?;
    
    // Start BPI Dashboard  
    start_dashboard("bpi", 3001).await?;
    
    // Start MetaNode Wallet
    start_dashboard("wallet", 3002).await?;
    
    Ok(())
}

async fn start_dashboard(dashboard_type: &str, port: u16) -> Result<()> {
    println!("    🎨 Starting {} dashboard on port {}...", dashboard_type, port);
    
    let mut cmd = Command::new("npm");
    cmd.args(&["run", "dev"])
        .current_dir(format!("dashboards/{}", dashboard_type))
        .env("PORT", port.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    
    let child = cmd.spawn()?;
    
    println!("      ✅ {} dashboard started", dashboard_type);
    Ok(())
}

async fn wait_for_services() -> Result<()> {
    println!("⏳ Waiting for services to be ready...");
    
    // Check BPCI node
    wait_for_service("BPCI node", "http://localhost:8080/health").await?;
    
    // Check Shadow Registry
    wait_for_service("Shadow Registry", "http://localhost:8080/api/v1/health").await?;
    
    // Check Economic APIs
    wait_for_service("Economic APIs", "http://localhost:8081/api/v1/economic/health").await?;
    
    // Check dashboards
    wait_for_service("BPCI Dashboard", "http://localhost:3000").await?;
    wait_for_service("BPI Dashboard", "http://localhost:3001").await?;
    wait_for_service("MetaNode Wallet", "http://localhost:3002").await?;
    
    Ok(())
}

async fn wait_for_service(name: &str, url: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let mut attempts = 0;
    let max_attempts = 30; // 30 seconds timeout
    
    loop {
        match client.get(url).send().await {
            Ok(response) if response.status().is_success() => {
                println!("    ✅ {} is ready", name);
                break;
            }
            _ => {
                attempts += 1;
                if attempts >= max_attempts {
                    return Err(format!("Timeout waiting for {} to be ready", name).into());
                }
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
    
    Ok(())
}
