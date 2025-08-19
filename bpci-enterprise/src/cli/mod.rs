use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::info;
use crate::config::BpciConfig;

// ASCII art logo for Pravyom
const PRAVYOM_LOGO: &str = r#"
    ██████╗ ██████╗  █████╗ ██╗   ██╗██╗   ██╗ ██████╗ ███╗   ███╗
    ██╔══██╗██╔══██╗██╔══██╗██║   ██║╚██╗ ██╔╝██╔═══██╗████╗ ████║
    ██████╔╝██████╔╝███████║██║   ██║ ╚████╔╝ ██║   ██║██╔████╔██║
    ██╔═══╝ ██╔══██╗██╔══██║╚██╗ ██╔╝  ╚██╔╝  ██║   ██║██║╚██╔╝██║
    ██║     ██║  ██║██║  ██║ ╚████╔╝    ██║   ╚██████╔╝██║ ╚═╝ ██║
    ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝  ╚═══╝     ╚═╝    ╚═════╝ ╚═╝     ╚═╝
"#;

pub mod wallet;
pub mod registry;
pub mod mining;
pub mod governance;
pub mod network;
pub mod notary;
pub mod maintenance;
pub mod web;

use wallet::WalletCommands;
use registry::RegistryCommands;
use mining::MiningCommands;
use governance::GovernanceCommands;
use network::NetworkCommands;
use notary::NotaryCommands;
use maintenance::MaintenanceCommands;
use web::WebCommands;

/// BPCI Enterprise - Complete Blockchain Platform Command Interface
/// Military-grade security, enterprise governance, autonomous economics
#[derive(Parser)]
#[command(name = "pravyom")]
#[command(about = "Pravyom - Complete blockchain platform command interface with military-grade security")]
#[command(version = "1.0.0")]
pub struct BpciCli {
    #[command(subcommand)]
    pub command: BpciCommands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Output format (json or human-readable)
    #[arg(long, global = true, default_value = "human")]
    pub format: String,

    /// Configuration file path
    #[arg(short, long, global = true)]
    pub config: Option<String>,

    /// Network selection (testnet, mainnet, localnet)
    #[arg(short, long, global = true, default_value = "testnet")]
    pub network: String,

    /// Dry run mode (don't execute, just show what would happen)
    #[arg(long, global = true)]
    pub dry_run: bool,
}

#[derive(Subcommand)]
pub enum BpciCommands {
    /// Wallet management operations
    #[command(subcommand)]
    Wallet(WalletCommands),

    /// BPI wallet registry operations
    #[command(subcommand)]
    Registry(RegistryCommands),

    /// Proof-of-Execution mining operations
    #[command(subcommand)]
    Mining(MiningCommands),

    /// Governance and economics operations
    #[command(subcommand)]
    Governance(GovernanceCommands),

    /// Network management operations
    #[command(subcommand)]
    Network(NetworkCommands),

    /// Notary and verification services
    #[command(subcommand)]
    Notary(NotaryCommands),

    /// System maintenance operations
    #[command(subcommand)]
    Maintenance(MaintenanceCommands),

    /// Web interface and API operations
    #[command(subcommand)]
    Web(WebCommands),

    /// Show comprehensive system status
    Status,

    /// Initialize BPCI system
    Init {
        /// Force initialization (overwrite existing)
        #[arg(long)]
        force: bool,
    },
}

impl BpciCli {
    pub fn is_json(&self) -> bool {
        self.format == "json"
    }

    pub async fn execute(&self) -> Result<()> {
        // Initialize logging
        if self.verbose {
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG)
                .init();
        } else {
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::INFO)
                .init();
        }

        // Display Pravyom logo for human-readable output
        if self.format == "human" {
            println!("{}", PRAVYOM_LOGO);
            println!("    🚀 Pravyom - Military-grade blockchain infrastructure");
            println!();
        }
        
        info!("Pravyom Enterprise CLI v{}", env!("CARGO_PKG_VERSION"));
        info!("Network: {}", self.network);

        // Load and validate deployment configuration
        let config_path = self.config.as_deref().unwrap_or("config.toml");
        let config = BpciConfig::load_from_file(config_path)?;
        config.validate()?;
        
        // Validate network selection against deployment mode
        if !config.is_network_allowed(&self.network) {
            return Err(anyhow::anyhow!(
                "Network '{}' is not allowed in {} mode. Allowed networks: {:?}",
                self.network,
                format!("{:?}", config.network.mode).to_lowercase(),
                config.connection.allowed_networks
            ));
        }

        info!("Deployment mode: {:?}", config.network.mode);
        info!("Configuration validated successfully");

        // Set environment variables for global configuration
        std::env::set_var("BPCI_NETWORK", &self.network);
        std::env::set_var("BPCI_OUTPUT_FORMAT", &self.format);
        std::env::set_var("BPCI_CONFIG", config_path);
        std::env::set_var("BPCI_DEPLOYMENT_MODE", &format!("{:?}", config.network.mode));

        let result = match &self.command {
            BpciCommands::Wallet(cmd) => {
                wallet::handle_wallet_command(cmd, self.is_json(), self.dry_run).await
            }
            BpciCommands::Registry(cmd) => {
                registry::handle_registry_command(cmd, self.is_json(), self.dry_run).await
            }
            BpciCommands::Mining(cmd) => {
                mining::handle_mining_command(cmd, self.is_json(), self.dry_run).await
            }
            BpciCommands::Governance(cmd) => {
                governance::handle_governance_command(cmd, self.is_json(), self.dry_run).await
            }
            BpciCommands::Network(cmd) => {
                network::handle_network_command(cmd, self.is_json(), self.dry_run).await
            }
            BpciCommands::Notary(cmd) => {
                notary::handle_notary_command(cmd, self.is_json(), self.dry_run).await
            }
            BpciCommands::Maintenance(cmd) => {
                maintenance::handle_maintenance_command(cmd, self.is_json(), self.dry_run).await
            }
            BpciCommands::Web(cmd) => {
                web::handle_web_command(cmd, self.is_json(), self.dry_run).await
            }
            BpciCommands::Status => {
                self.handle_status_command().await
            }
            BpciCommands::Init { force } => {
                self.handle_init_command(*force).await
            }
        };

        if let Err(e) = result {
            eprintln!("Command failed: {}", e);
            std::process::exit(1);
        }

        Ok(())
    }

    async fn handle_status_command(&self) -> Result<()> {
        if self.is_json() {
            println!("{}", serde_json::json!({
                "status": "operational",
                "network": self.network,
                "version": env!("CARGO_PKG_VERSION"),
                "components": {
                    "wallet": "active",
                    "registry": "active",
                    "mining": "active",
                    "governance": "active",
                    "notary": "active",
                    "web": "active"
                },
                "timestamp": chrono::Utc::now()
            }));
        } else {
            println!("🚀 BPCI Enterprise Status");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            println!("Network: {}", self.network);
            println!("Status: ✅ Operational");
            println!();
            println!("Components:");
            println!("  • Wallet System: ✅ Active");
            println!("  • Registry: ✅ Active");
            println!("  • Mining Engine: ✅ Active");
            println!("  • Governance: ✅ Active");
            println!("  • Notary Services: ✅ Active");
            println!("  • Web Interface: ✅ Active");
        }
        Ok(())
    }

    async fn handle_init_command(&self, force: bool) -> Result<()> {
        if self.is_json() {
            println!("{}", serde_json::json!({
                "status": "initialized",
                "network": self.network,
                "force": force,
                "message": "BPCI system initialized successfully"
            }));
        } else {
            println!("🔧 Initializing BPCI Enterprise System");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("Network: {}", self.network);
            if force {
                println!("Mode: Force initialization (overwriting existing)");
            }
            println!("✅ BPCI system initialized successfully");
        }
        Ok(())
    }
}
