use clap::{Parser, Subcommand};
use anyhow::Result;
use tokio;
use serde_json;
use std::path::PathBuf;

use pravyom_enterprise::round_table_oracle::{
    RoundTableOracle, PartnerChainConfig, OracleConfig
};

/// BPCI Round Table CLI - Partner Chain Management
#[derive(Parser)]
#[command(name = "round-table-cli")]
#[command(about = "BPCI Round Table Partnership Management CLI")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Configuration file path
    #[arg(long, default_value = "/opt/bpci/config/round-table.toml")]
    config: PathBuf,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Register a new partner chain
    Register {
        /// Chain ID (e.g., 137 for Polygon)
        #[arg(long)]
        chain_id: u64,
        
        /// Chain name (e.g., "Polygon")
        #[arg(long)]
        name: String,
        
        /// RPC endpoint URL
        #[arg(long)]
        rpc_endpoint: String,
        
        /// WebSocket endpoint URL
        #[arg(long)]
        websocket_endpoint: String,
        
        /// Representative address
        #[arg(long)]
        representative_address: String,
        
        /// Revenue share percentage (default: 25)
        #[arg(long, default_value = "25")]
        revenue_share_percent: u8,
    },
    
    /// List all registered partner chains
    List {
        /// Show only active chains
        #[arg(long)]
        active_only: bool,
        
        /// Output format (table, json)
        #[arg(long, default_value = "table")]
        format: String,
    },
    
    /// Get partner chain statistics
    Stats {
        /// Specific chain ID (optional)
        #[arg(long)]
        chain_id: Option<u64>,
        
        /// Output format (table, json)
        #[arg(long, default_value = "table")]
        format: String,
    },
    
    /// Create a partnership agreement
    CreatePartnership {
        /// Partner chain ID
        #[arg(long)]
        partner_chain_id: u64,
    },
    
    /// Sign a partnership agreement
    SignPartnership {
        /// Partnership ID
        #[arg(long)]
        partnership_id: String,
        
        /// Signature (can be generated or provided)
        #[arg(long)]
        signature: Option<String>,
        
        /// Sign as partner (default: false, signs as BPCI)
        #[arg(long)]
        as_partner: bool,
    },
    
    /// Start the Round Table Oracle monitoring
    Monitor {
        /// Monitoring interval in seconds
        #[arg(long, default_value = "30")]
        interval: u64,
        
        /// Maximum number of partner chains
        #[arg(long, default_value = "50")]
        max_partners: usize,
    },
    
    /// Get oracle status and metrics
    Status,
    
    /// Validate partner chain connectivity
    Validate {
        /// Chain ID to validate
        #[arg(long)]
        chain_id: u64,
    },
    
    /// Export partner chain configurations
    Export {
        /// Output file path
        #[arg(long, default_value = "partner_chains_export.json")]
        output: PathBuf,
    },
    
    /// Import partner chain configurations
    Import {
        /// Input file path
        #[arg(long)]
        input: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    if cli.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    }
    
    // Load configuration
    let oracle_config = load_oracle_config(&cli.config).await?;
    
    // Initialize Round Table Oracle
    let oracle = RoundTableOracle::new(Some(oracle_config));
    
    // Execute command
    match cli.command {
        Commands::Register {
            chain_id,
            name,
            rpc_endpoint,
            websocket_endpoint,
            representative_address,
            revenue_share_percent,
        } => {
            let mut config = PartnerChainConfig::new(
                chain_id,
                name.clone(),
                rpc_endpoint,
                websocket_endpoint,
                representative_address,
            );
            config.revenue_share_percent = revenue_share_percent;
            
            println!("🔄 Registering partner chain: {}", name);
            oracle.register_partner_chain(config).await?;
            println!("✅ Partner chain '{}' registered successfully!", name);
        }
        
        Commands::List { active_only, format } => {
            let chains = oracle.partner_chains.read().await;
            
            let filtered_chains: Vec<_> = if active_only {
                chains.values().filter(|c| c.is_active).collect()
            } else {
                chains.values().collect()
            };
            
            match format.as_str() {
                "json" => {
                    let json_output = serde_json::to_string_pretty(&filtered_chains)?;
                    println!("{}", json_output);
                }
                "table" | _ => {
                    println!("📋 Registered Partner Chains:");
                    println!("{:<10} {:<20} {:<15} {:<10} {:<50}", 
                        "Chain ID", "Name", "Revenue %", "Active", "RPC Endpoint");
                    println!("{}", "-".repeat(110));
                    
                    for chain in filtered_chains {
                        println!("{:<10} {:<20} {:<15} {:<10} {:<50}", 
                            chain.chain_id,
                            chain.name,
                            format!("{}%", chain.revenue_share_percent),
                            if chain.is_active { "✅" } else { "❌" },
                            chain.rpc_endpoint
                        );
                    }
                }
            }
        }
        
        Commands::Stats { chain_id, format } => {
            let stats = oracle.get_partner_statistics().await?;
            
            match format.as_str() {
                "json" => {
                    if let Some(id) = chain_id {
                        if let Some(stat) = stats.get(&id) {
                            let json_output = serde_json::to_string_pretty(stat)?;
                            println!("{}", json_output);
                        } else {
                            eprintln!("❌ Chain {} not found", id);
                            std::process::exit(1);
                        }
                    } else {
                        let json_output = serde_json::to_string_pretty(&stats)?;
                        println!("{}", json_output);
                    }
                }
                "table" | _ => {
                    if let Some(id) = chain_id {
                        if let Some(stat) = stats.get(&id) {
                            println!("📊 Statistics for Chain {}:", id);
                            println!("  Name: {}", stat.name);
                            println!("  Total Revenue: {} wei", stat.total_revenue);
                            println!("  Revenue Share: {}%", stat.revenue_share_percent);
                            println!("  Distributions: {}", stat.distributions_count);
                            println!("  Average Distribution: {:.2} wei", stat.average_distribution);
                            println!("  Active: {}", if stat.is_active { "✅" } else { "❌" });
                            println!("  Joined: {}", stat.joined_at.format("%Y-%m-%d %H:%M:%S UTC"));
                            if let Some(last_dist) = stat.last_distribution {
                                println!("  Last Distribution: {}", last_dist.format("%Y-%m-%d %H:%M:%S UTC"));
                            }
                        } else {
                            eprintln!("❌ Chain {} not found", id);
                            std::process::exit(1);
                        }
                    } else {
                        println!("📊 All Partner Chain Statistics:");
                        println!("{:<10} {:<20} {:<15} {:<15} {:<10} {:<15}", 
                            "Chain ID", "Name", "Total Revenue", "Distributions", "Active", "Revenue %");
                        println!("{}", "-".repeat(90));
                        
                        for (chain_id, stat) in stats {
                            println!("{:<10} {:<20} {:<15} {:<15} {:<10} {:<15}", 
                                chain_id,
                                stat.name,
                                format!("{} wei", stat.total_revenue),
                                stat.distributions_count,
                                if stat.is_active { "✅" } else { "❌" },
                                format!("{}%", stat.revenue_share_percent)
                            );
                        }
                    }
                }
            }
        }
        
        Commands::CreatePartnership { partner_chain_id } => {
            println!("🤝 Creating partnership with chain {}...", partner_chain_id);
            let partnership_id = oracle.create_partnership(partner_chain_id).await?;
            println!("✅ Partnership created: {}", partnership_id);
            println!("📝 Next step: Both parties need to sign the partnership");
            println!("   BPCI: round-table-cli sign-partnership --partnership-id {}", partnership_id);
            println!("   Partner: round-table-cli sign-partnership --partnership-id {} --as-partner", partnership_id);
        }
        
        Commands::SignPartnership { partnership_id, signature, as_partner } => {
            let sig = signature.unwrap_or_else(|| {
                // Generate a simple signature (in production, this would be cryptographic)
                format!("sig_{}_{}", 
                    if as_partner { "partner" } else { "bpci" },
                    chrono::Utc::now().timestamp()
                )
            });
            
            println!("📝 Signing partnership {} as {}...", 
                partnership_id, 
                if as_partner { "partner" } else { "BPCI" }
            );
            
            oracle.sign_partnership(&partnership_id, sig, as_partner).await?;
            println!("✅ Partnership signed successfully!");
        }
        
        Commands::Monitor { interval, max_partners } => {
            let mut config = oracle.oracle_config.clone();
            config.monitoring_interval_secs = interval;
            config.max_partner_chains = max_partners;
            
            println!("🚀 Starting Round Table Oracle monitoring...");
            println!("   Monitoring interval: {}s", interval);
            println!("   Max partner chains: {}", max_partners);
            println!("   Press Ctrl+C to stop");
            
            // Start monitoring (this will run indefinitely)
            oracle.start_monitoring().await?;
        }
        
        Commands::Status => {
            let status = oracle.get_oracle_status().await;
            
            println!("🔮 Round Table Oracle Status:");
            println!("  Partner Chains: {} total, {} active", 
                status.total_partner_chains, status.active_partner_chains);
            println!("  Partnerships: {} total, {} active", 
                status.total_partnerships, status.active_partnerships);
            println!("  Revenue Distributions: {}", status.total_distributions);
            println!("  Total Revenue Distributed: {} wei", status.total_revenue_distributed);
            println!("  Pending Transactions: {}", status.pending_transactions);
            println!("  Oracle Uptime: {}", status.oracle_uptime.format("%Y-%m-%d %H:%M:%S UTC"));
        }
        
        Commands::Validate { chain_id } => {
            let chains = oracle.partner_chains.read().await;
            if let Some(config) = chains.get(&chain_id) {
                println!("🔍 Validating partner chain: {}", config.name);
                
                // Re-validate the chain
                match oracle.validate_partner_chain(config).await {
                    Ok(_) => println!("✅ Partner chain validation successful!"),
                    Err(e) => {
                        eprintln!("❌ Partner chain validation failed: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("❌ Chain {} not found", chain_id);
                std::process::exit(1);
            }
        }
        
        Commands::Export { output } => {
            let chains = oracle.partner_chains.read().await;
            let chains_vec: Vec<_> = chains.values().collect();
            
            let json_data = serde_json::to_string_pretty(&chains_vec)?;
            tokio::fs::write(&output, json_data).await?;
            
            println!("✅ Exported {} partner chains to {}", 
                chains_vec.len(), output.display());
        }
        
        Commands::Import { input } => {
            let json_data = tokio::fs::read_to_string(&input).await?;
            let chains: Vec<PartnerChainConfig> = serde_json::from_str(&json_data)?;
            
            println!("📥 Importing {} partner chains from {}...", 
                chains.len(), input.display());
            
            let mut success_count = 0;
            let mut error_count = 0;
            let total_chains = chains.len();
            
            for config in chains {
                match oracle.register_partner_chain(config.clone()).await {
                    Ok(_) => {
                        println!("✅ Imported: {}", config.name);
                        success_count += 1;
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to import {}: {}", config.name, e);
                        error_count += 1;
                    }
                }
            }
            
            println!("✅ Import completed: {}/{} chains imported successfully", 
                success_count, total_chains);
        }
    }
    
    Ok(())
}

async fn load_oracle_config(config_path: &PathBuf) -> Result<OracleConfig> {
    // Try to load configuration file, fall back to defaults if not found
    if config_path.exists() {
        let config_content = tokio::fs::read_to_string(config_path).await?;
        
        // Parse TOML configuration
        let config_value: toml::Value = toml::from_str(&config_content)?;
        
        let mut oracle_config = OracleConfig::default();
        
        if let Some(bpci_chain_id) = config_value.get("bpci_chain_id").and_then(|v| v.as_integer()) {
            oracle_config.bpci_chain_id = bpci_chain_id as u64;
        }
        
        if let Some(monitoring_interval) = config_value.get("monitoring_interval_secs").and_then(|v| v.as_integer()) {
            oracle_config.monitoring_interval_secs = monitoring_interval as u64;
        }
        
        if let Some(max_partners) = config_value.get("max_partner_chains").and_then(|v| v.as_integer()) {
            oracle_config.max_partner_chains = max_partners as usize;
        }
        
        if let Some(revenue_share) = config_value.get("default_revenue_share").and_then(|v| v.as_integer()) {
            oracle_config.default_revenue_share = revenue_share as u8;
        }
        
        if let Some(min_payout) = config_value.get("min_payout_threshold").and_then(|v| v.as_integer()) {
            oracle_config.min_payout_threshold = min_payout as u64;
        }
        
        Ok(oracle_config)
    } else {
        // Create default configuration file
        let default_config = r#"
# BPCI Round Table Oracle Configuration

# BPCI chain ID
bpci_chain_id = 1337

# Monitoring interval in seconds
monitoring_interval_secs = 30

# Maximum number of partner chains
max_partner_chains = 50

# Default revenue share percentage for new partners
default_revenue_share = 25

# Minimum payout threshold in wei
min_payout_threshold = 100000
"#;
        
        if let Some(parent) = config_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        tokio::fs::write(config_path, default_config).await?;
        println!("📝 Created default configuration file: {}", config_path.display());
        
        Ok(OracleConfig::default())
    }
}
