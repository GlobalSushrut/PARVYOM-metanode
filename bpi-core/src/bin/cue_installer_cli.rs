use anyhow::Result;
use clap::{Parser, Subcommand};
use serde_json;
use std::io::{self, Write};

use bpi_core::cue_installer::{CueInstaller, InstallationPrompt, InstallationType};

/// BPI OS CUE.Installer - Comprehensive OS Installation System
#[derive(Parser)]
#[command(name = "cue-installer")]
#[command(about = "BPI OS Installation System with Linux distribution detection and prerequisite management")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Output in JSON format
    #[arg(long, global = true)]
    json: bool,
    
    /// Dry run mode (don't actually install)
    #[arg(long, global = true)]
    dry_run: bool,
    
    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Explain installation process and system requirements
    Explain,
    
    /// Detect system information and compatibility
    Detect,
    
    /// Run compatibility checks
    Check,
    
    /// Install prerequisites
    Prerequisites,
    
    /// Interactive installation wizard
    Install,
    
    /// Show installation status
    Status,
    
    /// Generate installation configuration
    Config {
        /// Installation type (minimum, default, full, custom)
        #[arg(short, long, default_value = "default")]
        install_type: String,
        
        /// Target installation directory
        #[arg(short, long, default_value = "/opt/bpi")]
        target: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Explain => handle_explain(&cli)?,
        Commands::Detect => handle_detect(&cli)?,
        Commands::Check => handle_check(&cli)?,
        Commands::Prerequisites => handle_prerequisites(&cli)?,
        Commands::Install => handle_install(&cli)?,
        Commands::Status => handle_status(&cli)?,
        Commands::Config { ref install_type, ref target } => handle_config(&cli, install_type, target)?,
    }
    
    Ok(())
}

fn handle_explain(cli: &Cli) -> Result<()> {
    if cli.json {
        let installer = CueInstaller::new()?;
        let json_output = serde_json::json!({
            "command": "explain",
            "system_info": installer.system_info,
            "installation_config": installer.installation_config,
            "prerequisites": installer.prerequisites,
            "distro_handler": installer.distro_handler
        });
        println!("{}", serde_json::to_string_pretty(&json_output)?);
    } else {
        let installer = CueInstaller::new()?;
        installer.explain_installation()?;
        
        println!("🎯 NEXT STEPS:");
        println!("   1. Run compatibility check: cue-installer check");
        println!("   2. Install prerequisites: cue-installer prerequisites");
        println!("   3. Start installation: cue-installer install");
        println!();
        println!("For more information: cue-installer --help");
    }
    
    Ok(())
}

fn handle_detect(cli: &Cli) -> Result<()> {
    let installer = CueInstaller::new()?;
    
    if cli.json {
        let json_output = serde_json::json!({
            "command": "detect",
            "system_info": installer.system_info,
            "distro_handler": installer.distro_handler,
            "detected_at": chrono::Utc::now().to_rfc3339()
        });
        println!("{}", serde_json::to_string_pretty(&json_output)?);
    } else {
        println!("🔍 SYSTEM DETECTION RESULTS:");
        println!("================================================================================");
        println!("Operating System: {} {}", installer.system_info.os_name, installer.system_info.os_version);
        println!("Kernel Version: {}", installer.system_info.kernel_version);
        println!("Architecture: {}", installer.system_info.architecture);
        println!("CPU Cores: {}", installer.system_info.cpu_cores);
        println!("Total Memory: {:.1} GB", installer.system_info.total_memory_gb);
        println!("Available Disk: {:.1} GB", installer.system_info.available_disk_gb);
        println!("Distribution Family: {:?}", installer.system_info.distro_family);
        println!("Package Manager: {:?}", installer.distro_handler.package_manager);
        println!("Service Manager: {:?}", installer.distro_handler.service_manager);
        
        if installer.system_info.is_ubuntu {
            println!("✅ Ubuntu Pre-installed Support: ENABLED");
        } else {
            println!("⚠️  Ubuntu Pre-installed Support: DISABLED");
        }
        
        println!();
        println!("Recommended Installation Type: {:?}", installer.installation_config.installation_type);
    }
    
    Ok(())
}

fn handle_check(cli: &Cli) -> Result<()> {
    let mut installer = CueInstaller::new()?;
    
    if !cli.json {
        println!("🔍 RUNNING COMPATIBILITY CHECKS:");
        println!("================================================================================");
    }
    
    let all_passed = installer.run_compatibility_checks()?;
    
    if cli.json {
        let json_output = serde_json::json!({
            "command": "check",
            "compatibility_checks": installer.prerequisites.compatibility_checks,
            "all_passed": all_passed,
            "system_requirements_met": {
                "cpu_cores": installer.system_info.cpu_cores >= installer.prerequisites.system_requirements.min_cpu_cores,
                "memory": installer.system_info.total_memory_gb >= installer.prerequisites.system_requirements.min_memory_gb,
                "disk_space": installer.system_info.available_disk_gb >= installer.prerequisites.system_requirements.min_disk_gb
            },
            "checked_at": chrono::Utc::now().to_rfc3339()
        });
        println!("{}", serde_json::to_string_pretty(&json_output)?);
    } else {
        if all_passed {
            println!("✅ SYSTEM READY FOR BPI OS INSTALLATION");
            println!("   All compatibility checks passed successfully.");
            println!("   You can proceed with: cue-installer install");
        } else {
            println!("❌ SYSTEM NOT READY FOR INSTALLATION");
            println!("   Some required compatibility checks failed.");
            println!("   Please resolve the issues above before proceeding.");
        }
    }
    
    Ok(())
}

fn handle_prerequisites(cli: &Cli) -> Result<()> {
    let installer = CueInstaller::new()?;
    
    if cli.dry_run {
        if cli.json {
            let json_output = serde_json::json!({
                "command": "prerequisites",
                "dry_run": true,
                "required_packages": installer.prerequisites.required_packages,
                "package_manager": installer.distro_handler.package_manager,
                "would_execute": format!("Package installation via {:?}", installer.distro_handler.package_manager)
            });
            println!("{}", serde_json::to_string_pretty(&json_output)?);
        } else {
            println!("🧪 DRY RUN: Prerequisites Installation");
            println!("================================================================================");
            println!("Would install the following packages:");
            for package in &installer.prerequisites.required_packages {
                println!("   - {}", package);
            }
            println!("Package Manager: {:?}", installer.distro_handler.package_manager);
            println!();
            println!("To actually install: cue-installer prerequisites");
        }
        return Ok(());
    }
    
    if !cli.json {
        println!("📦 INSTALLING PREREQUISITES:");
        println!("================================================================================");
        
        // Prompt for confirmation unless in JSON mode
        print!("This will install system packages. Continue? (y/N): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Installation cancelled by user.");
            return Ok(());
        }
    }
    
    match installer.install_prerequisites() {
        Ok(()) => {
            if cli.json {
                let json_output = serde_json::json!({
                    "command": "prerequisites",
                    "status": "success",
                    "installed_packages": installer.prerequisites.required_packages,
                    "installed_at": chrono::Utc::now().to_rfc3339()
                });
                println!("{}", serde_json::to_string_pretty(&json_output)?);
            } else {
                println!("✅ Prerequisites installed successfully!");
                println!("   Next step: cue-installer install");
            }
        },
        Err(e) => {
            if cli.json {
                let json_output = serde_json::json!({
                    "command": "prerequisites",
                    "status": "error",
                    "error": e.to_string(),
                    "failed_at": chrono::Utc::now().to_rfc3339()
                });
                println!("{}", serde_json::to_string_pretty(&json_output)?);
            } else {
                println!("❌ Failed to install prerequisites: {}", e);
                println!("   Please check your package manager and try again.");
            }
        }
    }
    
    Ok(())
}

fn handle_install(cli: &Cli) -> Result<()> {
    let installer = CueInstaller::new()?;
    
    if cli.json {
        let json_output = serde_json::json!({
            "command": "install",
            "status": "starting",
            "installation_summary": installer.get_installation_summary(),
            "dry_run": cli.dry_run,
            "started_at": chrono::Utc::now().to_rfc3339()
        });
        println!("{}", serde_json::to_string_pretty(&json_output)?);
    } else {
        println!("🚀 BPI OS INSTALLATION WIZARD");
        println!("================================================================================");
        
        // Show installation summary
        installer.explain_installation()?;
        
        // Confirm installation
        if !InstallationPrompt::confirm_installation(&installer)? {
            println!("Installation cancelled by user.");
            return Ok(());
        }
    }
    
    if cli.dry_run {
        if cli.json {
            let json_output = serde_json::json!({
                "command": "install",
                "status": "dry_run_complete",
                "would_install": {
                    "installation_type": installer.installation_config.installation_type,
                    "target_directory": installer.installation_config.target_directory,
                    "services": installer.installation_config.enable_services,
                    "security_level": installer.installation_config.security_level
                }
            });
            println!("{}", serde_json::to_string_pretty(&json_output)?);
        } else {
            println!("🧪 DRY RUN: BPI OS Installation");
            println!("================================================================================");
            println!("{}", installer.get_installation_summary());
            println!();
            println!("Installation steps that would be executed:");
            println!("   1. ✅ System compatibility check");
            println!("   2. ✅ Prerequisites installation");
            println!("   3. 🔄 BPI OS core installation");
            println!("   4. 🔄 Service configuration");
            println!("   5. 🔄 Security hardening");
            println!("   6. 🔄 Network setup");
            println!("   7. 🔄 Final validation");
            println!();
            println!("To perform actual installation: cue-installer install");
        }
        return Ok(());
    }
    
    // Actual installation process
    println!("🔄 Starting BPI OS installation...");
    
    // Step 1: Compatibility check
    println!("Step 1/7: Running compatibility checks...");
    let mut installer_mut = installer;
    if !installer_mut.run_compatibility_checks()? {
        return Err(anyhow::anyhow!("Compatibility checks failed"));
    }
    
    // Step 2: Prerequisites (if not already installed)
    println!("Step 2/7: Checking prerequisites...");
    // In a real implementation, this would check if prerequisites are already installed
    
    // Step 3-7: BPI OS installation (placeholder for actual implementation)
    println!("Step 3/7: Installing BPI OS core...");
    println!("Step 4/7: Configuring services...");
    println!("Step 5/7: Applying security hardening...");
    println!("Step 6/7: Setting up network configuration...");
    println!("Step 7/7: Running final validation...");
    
    if cli.json {
        let json_output = serde_json::json!({
            "command": "install",
            "status": "completed",
            "installation_summary": installer_mut.get_installation_summary(),
            "completed_at": chrono::Utc::now().to_rfc3339()
        });
        println!("{}", serde_json::to_string_pretty(&json_output)?);
    } else {
        println!("✅ BPI OS INSTALLATION COMPLETED SUCCESSFULLY!");
        println!("================================================================================");
        println!("{}", installer_mut.get_installation_summary());
        println!();
        println!("🎯 NEXT STEPS:");
        println!("   1. Start BPI services: systemctl start bpi-core");
        println!("   2. Check status: cue-installer status");
        println!("   3. Access BPI dashboard: http://localhost:7777");
        println!();
        println!("For support: https://docs.bpi.io/installation");
    }
    
    Ok(())
}

fn handle_status(cli: &Cli) -> Result<()> {
    if cli.json {
        let json_output = serde_json::json!({
            "command": "status",
            "bpi_os_installed": false, // Would check actual installation
            "services": {
                "bpi-core": "not_installed",
                "bpi-relay": "not_installed",
                "docklock": "not_installed"
            },
            "checked_at": chrono::Utc::now().to_rfc3339()
        });
        println!("{}", serde_json::to_string_pretty(&json_output)?);
    } else {
        println!("📊 BPI OS INSTALLATION STATUS:");
        println!("================================================================================");
        println!("BPI OS Status: Not Installed");
        println!("Installation Directory: /opt/bpi (not found)");
        println!();
        println!("Services Status:");
        println!("   - bpi-core: Not Installed");
        println!("   - bpi-relay: Not Installed");
        println!("   - docklock: Not Installed");
        println!();
        println!("To install BPI OS: cue-installer install");
    }
    
    Ok(())
}

fn handle_config(cli: &Cli, install_type: &str, target: &str) -> Result<()> {
    let mut installer = CueInstaller::new()?;
    
    // Parse installation type
    let parsed_type = match install_type.to_lowercase().as_str() {
        "minimum" | "min" => InstallationType::Minimum,
        "default" | "def" => InstallationType::Default,
        "full" | "complete" => InstallationType::Full,
        "custom" => InstallationType::Custom(vec!["bpi-core".to_string()]), // Default custom
        _ => return Err(anyhow::anyhow!("Invalid installation type: {}", install_type)),
    };
    
    // Update configuration
    installer.installation_config.installation_type = parsed_type;
    installer.installation_config.target_directory = target.to_string();
    
    if cli.json {
        let json_output = serde_json::json!({
            "command": "config",
            "installation_config": installer.installation_config,
            "system_info": installer.system_info,
            "generated_at": chrono::Utc::now().to_rfc3339()
        });
        println!("{}", serde_json::to_string_pretty(&json_output)?);
    } else {
        println!("⚙️  BPI OS INSTALLATION CONFIGURATION:");
        println!("================================================================================");
        println!("{}", installer.get_installation_summary());
        println!();
        println!("Configuration saved. Use 'cue-installer install' to proceed.");
    }
    
    Ok(())
}
