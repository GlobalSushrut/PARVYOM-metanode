//! BPI Immutable OS Installer
//! 
//! Transform any Linux system into an immutable, secure BPI Core OS
//! with military-grade security and post-quantum cryptography.

use anyhow::{Result, anyhow};
use std::process;
use tracing::{info, warn, error};
use tokio;

mod hardware_detection;
mod filesystem_engine;
mod security_hardening;
mod atomic_updates;
mod bpi_integration;

use hardware_detection::HardwareDetectionEngine;
use filesystem_engine::FilesystemImmutabilityEngine;
use security_hardening::SecurityHardeningEngine;
use atomic_updates::AtomicUpdateSystem;
use bpi_integration::NxosDrxBpiIntegration;

/// BPI Immutable OS Installer - Main orchestrator
#[derive(Debug)]
pub struct BpiImmutableInstaller {
    hardware_engine: HardwareDetectionEngine,
    filesystem_engine: FilesystemImmutabilityEngine,
    security_engine: SecurityHardeningEngine,
    update_system: AtomicUpdateSystem,
    bpi_integration: NxosDrxBpiIntegration,
}

impl BpiImmutableInstaller {
    /// Create new immutable OS installer
    pub async fn new() -> Result<Self> {
        info!("Initializing BPI Immutable OS Installer");
        
        Ok(Self {
            hardware_engine: HardwareDetectionEngine::new().await?,
            filesystem_engine: FilesystemImmutabilityEngine::new().await?,
            security_engine: SecurityHardeningEngine::new().await?,
            update_system: AtomicUpdateSystem::new().await?,
            bpi_integration: NxosDrxBpiIntegration::new().await?,
        })
    }

    /// Install immutable BPI OS on current system
    pub async fn install_immutable_os(&mut self) -> Result<()> {
        info!("🔥 Starting BPI Immutable OS Installation");
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        // Phase 1: System Analysis and Hardware Detection
        info!("📋 Phase 1: System Analysis and Hardware Detection");
        let hardware_profile = self.hardware_engine.detect_hardware().await?;
        info!("✅ Hardware detection completed: {}", hardware_profile.summary());
        
        // Phase 2: Filesystem Immutability Preparation
        info!("💾 Phase 2: Filesystem Immutability Preparation");
        self.filesystem_engine.prepare_immutable_filesystem(&hardware_profile).await?;
        info!("✅ Filesystem immutability prepared");
        
        // Phase 3: Security Hardening
        info!("🛡️ Phase 3: Military-Grade Security Hardening");
        self.security_engine.apply_security_hardening(&hardware_profile).await?;
        info!("✅ Security hardening applied");
        
        // Phase 4: NXOS DRX BPI Infrastructure Deployment
        info!("🌐 Phase 4: NXOS DRX BPI Infrastructure Deployment");
        self.bpi_integration.deploy_infrastructure(&hardware_profile).await?;
        info!("✅ NXOS DRX BPI infrastructure deployment completed");
        
        // Phase 5: Atomic Update System Setup
        info!("🔄 Phase 5: Atomic Update System Setup");
        self.update_system.setup_atomic_updates(&hardware_profile).await?;
        info!("✅ Atomic update system configured");
        
        // Phase 6: Final Immutability Lock
        info!("🔒 Phase 6: Final Immutability Lock");
        self.filesystem_engine.lock_immutable_filesystem().await?;
        info!("✅ System locked in immutable state");
        
        info!("🎉 BPI Immutable OS Installation Complete!");
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("🔄 System will reboot into immutable BPI Core OS in 10 seconds...");
        
        // Schedule reboot
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        self.reboot_system().await?;
        
        Ok(())
    }

    /// Reboot system into immutable BPI OS
    async fn reboot_system(&self) -> Result<()> {
        info!("🔄 Rebooting into immutable BPI Core OS...");
        
        // Sync filesystems
        std::process::Command::new("sync")
            .status()
            .map_err(|e| anyhow!("Failed to sync filesystems: {}", e))?;
        
        // Reboot system
        std::process::Command::new("reboot")
            .status()
            .map_err(|e| anyhow!("Failed to reboot system: {}", e))?;
        
        Ok(())
    }
}

/// Check if running as root
fn check_root_permissions() -> Result<()> {
    if !nix::unistd::Uid::effective().is_root() {
        return Err(anyhow!("❌ BPI Immutable OS Installer must be run as root\nPlease run: sudo ./bpi-immutable-installer"));
    }
    Ok(())
}

/// Check system compatibility
async fn check_system_compatibility() -> Result<()> {
    // Check if running on Linux
    if !cfg!(target_os = "linux") {
        return Err(anyhow!("❌ BPI Immutable OS only supports Linux systems"));
    }
    
    // Check minimum system requirements
    let sys = sysinfo::System::new_all();
    
    // Check memory (minimum 8GB for immutable OS)
    let total_memory = sys.total_memory();
    let min_memory = 8 * 1024 * 1024 * 1024; // 8GB in bytes
    
    if total_memory < min_memory {
        return Err(anyhow!("❌ Minimum 8GB RAM required for BPI Immutable OS"));
    }
    
    // Check available disk space (minimum 100GB)
    // This is a simplified check - real implementation would check all mount points
    let available_space = std::fs::metadata("/")
        .map_err(|e| anyhow!("Failed to check disk space: {}", e))?
        .len();
    
    let min_disk_space = 100 * 1024 * 1024 * 1024; // 100GB in bytes
    if available_space < min_disk_space {
        warn!("⚠️ Less than 100GB available - installation may fail");
    }
    
    info!("✅ System compatibility verified");
    Ok(())
}

/// Display installation banner
fn display_banner() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                  BPI IMMUTABLE OS INSTALLER                 ║");
    println!("║                         Version 1.0.0                       ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Transform any Linux system into immutable BPI Core OS      ║");
    println!("║  • Military-grade security with post-quantum cryptography   ║");
    println!("║  • Immutable filesystem with atomic updates                 ║");
    println!("║  • Complete BPI Core integration                            ║");
    println!("║  • Hardware-agnostic installation                           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
}

/// Display system information
async fn display_system_info() {
    let sys = sysinfo::System::new_all();
    
    println!("📊 System Information:");
    println!("  • OS: {} {}", sysinfo::System::name().unwrap_or("Unknown".to_string()), sysinfo::System::os_version().unwrap_or("Unknown".to_string()));
    println!("  • Kernel: {}", sysinfo::System::kernel_version().unwrap_or("Unknown".to_string()));
    println!("  • CPU: {} cores", sys.cpus().len());
    println!("  • Memory: {:.2} GB", sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0));
    println!("  • Architecture: {}", std::env::consts::ARCH);
    println!();
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    // Display banner
    display_banner();
    
    // Check root permissions
    if let Err(e) = check_root_permissions() {
        eprintln!("{}", e);
        process::exit(1);
    }
    
    // Display system information
    display_system_info().await;
    
    // Check system compatibility
    if let Err(e) = check_system_compatibility().await {
        error!("{}", e);
        process::exit(1);
    }
    
    // Confirm installation
    println!("⚠️  WARNING: This will transform your current Linux system into an immutable BPI Core OS.");
    println!("   All system files will become read-only, but user data will be preserved.");
    println!("   This operation cannot be easily undone.");
    println!();
    print!("Do you want to continue? (yes/no): ");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .map_err(|e| anyhow!("Failed to read input: {}", e))?;
    
    if input.trim().to_lowercase() != "yes" {
        println!("Installation cancelled.");
        return Ok(());
    }
    
    // Create and run installer
    match BpiImmutableInstaller::new().await {
        Ok(mut installer) => {
            if let Err(e) = installer.install_immutable_os().await {
                error!("Installation failed: {}", e);
                process::exit(1);
            }
        }
        Err(e) => {
            error!("Failed to initialize installer: {}", e);
            process::exit(1);
        }
    }
    
    Ok(())
}
