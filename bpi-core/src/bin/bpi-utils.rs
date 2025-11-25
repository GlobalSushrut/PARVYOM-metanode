//! BPI Utilities - Standalone CLI Tool
//! 
//! A focused, lightweight CLI tool that provides essential BPI operations
//! without requiring the main binary to compile. This demonstrates the
//! Rust-based approach for CLI utilities.

use anyhow::Result;
use clap::{Parser, Subcommand};
use bpi_core::cli_utilities::{CliConfig, run_cli_command};

#[derive(Parser)]
#[command(name = "bpi-utils")]
#[command(about = "BPI Core Utilities - Essential operations for BPIOS")]
#[command(version = "1.0.0")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Output in JSON format
    #[arg(short, long)]
    json: bool,
    
    /// Dry run - preview operations without execution
    #[arg(short, long)]
    dry_run: bool,
    
    /// Configuration file path
    #[arg(short, long)]
    config: Option<String>,
    
    /// Network to use (mainnet/testnet/devnet)
    #[arg(short, long, default_value = "mainnet")]
    network: String,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },
    /// System status and health
    Status {
        /// Show detailed status information
        #[arg(short, long)]
        detailed: bool,
    },
    /// Node health check
    Health {
        /// Run comprehensive health check
        #[arg(short, long)]
        comprehensive: bool,
    },
    /// Development operations
    Dev {
        #[command(subcommand)]
        action: DevCommands,
    },
    /// Monitoring operations
    Monitor {
        #[command(subcommand)]
        action: MonitorCommands,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Validate configuration file
    Validate {
        /// Path to configuration file
        #[arg(short, long, default_value = "~/.bpi/config.toml")]
        file: String,
    },
    /// Generate configuration template
    Generate {
        /// Template type (basic/enterprise/development/production)
        #[arg(short, long, default_value = "basic")]
        template: String,
        /// Output file path
        #[arg(short, long, default_value = "config.toml")]
        output: String,
    },
    /// Show current configuration
    Show {
        /// Configuration file path
        #[arg(short, long)]
        file: Option<String>,
    },
}

#[derive(Subcommand)]
enum DevCommands {
    /// Run tests
    Test {
        /// Test type (unit/integration/performance/all)
        #[arg(short, long, default_value = "all")]
        test_type: String,
    },
    /// Build project
    Build {
        /// Build mode (debug/release)
        #[arg(short, long, default_value = "debug")]
        mode: String,
    },
    /// Run benchmarks
    Benchmark {
        /// Benchmark type
        #[arg(short, long, default_value = "all")]
        bench_type: String,
    },
}

#[derive(Subcommand)]
enum MonitorCommands {
    /// Get system metrics
    Metrics {
        /// Metrics type (system/application/business)
        #[arg(short, long, default_value = "system")]
        metrics_type: String,
    },
    /// Start Grafana dashboard
    Grafana {
        /// Port for Grafana dashboard
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
    /// Show system logs
    Logs {
        /// Number of log lines to show
        #[arg(short, long, default_value = "100")]
        lines: usize,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let cli_config = CliConfig {
        verbose: cli.verbose,
        json_output: cli.json,
        dry_run: cli.dry_run,
        config_path: cli.config,
        network: Some(cli.network),
    };
    
    match cli.command {
        Commands::Config { action } => {
            match action {
                ConfigCommands::Validate { file } => {
                    run_cli_command("config", vec!["validate", &file], &cli_config).await?;
                }
                ConfigCommands::Generate { template, output } => {
                    run_cli_command("config", vec!["generate", &template, &output], &cli_config).await?;
                }
                ConfigCommands::Show { file } => {
                    if let Some(f) = file {
                        run_cli_command("config", vec!["show", &f], &cli_config).await?;
                    } else {
                        run_cli_command("config", vec!["show"], &cli_config).await?;
                    }
                }
            }
        }
        Commands::Status { detailed: _ } => {
            run_cli_command("status", vec![], &cli_config).await?;
        }
        Commands::Health { comprehensive: _ } => {
            run_cli_command("health", vec![], &cli_config).await?;
        }
        Commands::Dev { action } => {
            match action {
                DevCommands::Test { test_type } => {
                    run_cli_command("test", vec![&test_type], &cli_config).await?;
                }
                DevCommands::Build { mode } => {
                    run_cli_command("build", vec![&mode], &cli_config).await?;
                }
                DevCommands::Benchmark { bench_type } => {
                    println!("Benchmark functionality coming soon for: {}", bench_type);
                }
            }
        }
        Commands::Monitor { action } => {
            match action {
                MonitorCommands::Metrics { metrics_type } => {
                    run_cli_command("metrics", vec![&metrics_type], &cli_config).await?;
                }
                MonitorCommands::Grafana { port } => {
                    run_cli_command("grafana", vec![&port.to_string()], &cli_config).await?;
                }
                MonitorCommands::Logs { lines } => {
                    println!("Showing last {} log lines (functionality coming soon)", lines);
                }
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    
    #[test]
    fn verify_cli() {
        Cli::command().debug_assert()
    }
    
    #[test]
    fn test_cli_parsing() {
        let cli = Cli::try_parse_from(&[
            "bpi-utils",
            "--verbose",
            "--json",
            "config",
            "validate",
            "--file",
            "test.toml"
        ]).unwrap();
        
        assert!(cli.verbose);
        assert!(cli.json);
        
        match cli.command {
            Commands::Config { action } => {
                match action {
                    ConfigCommands::Validate { file } => {
                        assert_eq!(file, "test.toml");
                    }
                    _ => panic!("Wrong config command parsed"),
                }
            }
            _ => panic!("Wrong command parsed"),
        }
    }
}
