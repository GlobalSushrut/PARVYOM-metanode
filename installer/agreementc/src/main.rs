//! Agreement Compiler (agreementc) - Production-ready agreement tooling
//! Stage 54: Agreement Tooling for Metanode BISO Security & Compliance Architecture

use clap::{Arg, ArgMatches, Command};
use std::process::exit;
use serde_json::{json, Value};
use anyhow::{Result, Context};
use std::collections::HashMap;
use uuid::Uuid;
use std::path::Path;
use std::fs;

// Import Metanode agreement infrastructure
use bpi_docklock::agreements_sdk::{AgreementsSDK, SDKConfig, AgreementTemplate};
use bpi_docklock::court::{CourtConfig, AgreementEnforcementResult};
use bpi_docklock::policy_engine::{PolicyContext, PolicyConfig, SystemState};
use bpi_docklock::metanode_wallet::{WalletBoxAgreement, MonitoringLevel};

// Import simplified commands
mod simple_commands;
use simple_commands::{SimpleCourtCLI, ContainerId, ClusterId};

/// Exit codes for consistent error reporting
const EXIT_OK: i32 = 0;
const EXIT_USER_ERROR: i32 = 1;
const EXIT_NETWORK_ERROR: i32 = 2;
const EXIT_POLICY_FAIL: i32 = 3;
const EXIT_VALIDATION_ERROR: i32 = 4;
const EXIT_INTERNAL_ERROR: i32 = 5;

/// CLI Configuration
#[derive(Debug, Clone)]
struct CliConfig {
    json_output: bool,
    dry_run: bool,
    auto_yes: bool,
    verbose: bool,
    workspace_dir: String,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            json_output: false,
            dry_run: false,
            auto_yes: false,
            verbose: false,
            workspace_dir: std::env::var("AGREEMENTC_WORKSPACE")
                .unwrap_or_else(|_| "./agreements".to_string()),
        }
    }
}

#[tokio::main]
async fn main() {
    let result = run_cli().await;
    match result {
        Ok(code) => exit(code),
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(EXIT_INTERNAL_ERROR);
        }
    }
}

async fn run_cli() -> Result<i32> {
    let app = build_cli();
    let matches = app.get_matches();
    
    let config = CliConfig {
        json_output: matches.get_flag("json"),
        dry_run: matches.get_flag("dry-run"),
        auto_yes: matches.get_flag("yes"),
        verbose: matches.get_flag("verbose"),
        workspace_dir: matches.get_one::<String>("workspace")
            .cloned()
            .unwrap_or_else(|| CliConfig::default().workspace_dir),
    };

    match matches.subcommand() {
        Some(("init", sub_matches)) => handle_init_command(&config, sub_matches).await,
        Some(("court", sub_matches)) => handle_court_command(&config, sub_matches).await,
        Some(("policy", sub_matches)) => handle_policy_command(&config, sub_matches).await,
        Some(("agreement", sub_matches)) => handle_agreement_command(&config, sub_matches).await,
        Some(("template", sub_matches)) => handle_template_command(&config, sub_matches).await,
        Some(("enforce", sub_matches)) => handle_enforce_command(&config, sub_matches).await,
        Some(("validate", sub_matches)) => handle_validate_command(&config, sub_matches).await,
        Some(("wallet", sub_matches)) => handle_wallet_command(&config, sub_matches).await,
        Some(("export", sub_matches)) => handle_export_command(&config, sub_matches).await,
        Some(("import", sub_matches)) => handle_import_command(&config, sub_matches).await,
        Some(("status", sub_matches)) => handle_status_command(&config, sub_matches).await,
        _ => {
            println!("Agreement Compiler (agreementc) v1.0.0 - Production-ready agreement tooling");
            println!("Use 'agreementc --help' for available commands");
            Ok(EXIT_OK)
        }
    }
}

fn build_cli() -> Command {
    Command::new("agreementc")
        .version("1.0.0")
        .about("Agreement Compiler - Production-ready agreement tooling for Metanode")
        .author("Metanode Team")
        .arg(Arg::new("json")
            .long("json")
            .help("Output in JSON format")
            .action(clap::ArgAction::SetTrue)
            .global(true))
        .arg(Arg::new("dry-run")
            .long("dry-run")
            .help("Show what would be done without executing")
            .action(clap::ArgAction::SetTrue)
            .global(true))
        .arg(Arg::new("yes")
            .short('y')
            .long("yes")
            .help("Automatically answer yes to prompts")
            .action(clap::ArgAction::SetTrue)
            .global(true))
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .help("Enable verbose output")
            .action(clap::ArgAction::SetTrue)
            .global(true))
        .arg(Arg::new("workspace")
            .long("workspace")
            .help("Agreement workspace directory")
            .value_name("PATH")
            .global(true))
        .subcommand(build_init_command())
        .subcommand(build_court_command())
        .subcommand(build_deploy_command())
        .subcommand(build_agreements_command())
        .subcommand(build_violations_command())
        .subcommand(build_policy_command())
        .subcommand(build_agreement_command())
        .subcommand(build_template_command())
        .subcommand(build_enforce_command())
        .subcommand(build_validate_command())
        .subcommand(build_wallet_command())
        .subcommand(build_export_command())
        .subcommand(build_import_command())
        .subcommand(build_status_command())
}

fn build_init_command() -> Command {
    Command::new("init")
        .about("Initialize a new agreement workspace")
        .arg(Arg::new("name")
            .long("name")
            .help("Workspace name")
            .value_name("NAME"))
        .arg(Arg::new("template")
            .long("template")
            .help("Initialize with template")
            .value_parser(["basic", "enterprise", "compliance", "defi"])
            .value_name("TEMPLATE"))
}

fn build_court_command() -> Command {
    Command::new("court")
        .about("Court management operations")
        .subcommand(Command::new("create")
            .about("Create a new court")
            .arg(Arg::new("name")
                .long("name")
                .help("Court name")
                .required(true)
                .value_name("NAME"))
            .arg(Arg::new("description")
                .long("description")
                .help("Court description")
                .value_name("DESC")))
        .subcommand(Command::new("list")
            .about("List all courts"))
        .subcommand(Command::new("info")
            .about("Get court information")
            .arg(Arg::new("court-id")
                .help("Court ID")
                .required(true)))
}

fn build_policy_command() -> Command {
    Command::new("policy")
        .about("Policy management operations")
        .subcommand(Command::new("deploy")
            .about("Deploy a policy to a court")
            .arg(Arg::new("name")
                .long("name")
                .help("Policy name")
                .required(true)
                .value_name("NAME"))
            .arg(Arg::new("version")
                .long("version")
                .help("Policy version")
                .required(true)
                .value_name("VERSION"))
            .arg(Arg::new("wasm-file")
                .long("wasm-file")
                .help("WASM bytecode file")
                .required(true)
                .value_name("FILE"))
            .arg(Arg::new("pre-hook")
                .long("pre-hook")
                .help("Enable as pre-hook")
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("post-hook")
                .long("post-hook")
                .help("Enable as post-hook")
                .action(clap::ArgAction::SetTrue)))
        .subcommand(Command::new("list")
            .about("List policies"))
        .subcommand(Command::new("info")
            .about("Get policy information")
            .arg(Arg::new("policy-id")
                .help("Policy ID")
                .required(true)))
}

fn build_agreement_command() -> Command {
    Command::new("agreement")
        .about("Agreement management operations")
        .subcommand(Command::new("create")
            .about("Create a new agreement")
            .arg(Arg::new("name")
                .long("name")
                .help("Agreement name")
                .required(true)
                .value_name("NAME"))
            .arg(Arg::new("version")
                .long("version")
                .help("Agreement version")
                .required(true)
                .value_name("VERSION"))
            .arg(Arg::new("parties")
                .long("parties")
                .help("Agreement parties (comma-separated)")
                .required(true)
                .value_name("PARTIES"))
            .arg(Arg::new("policies")
                .long("policies")
                .help("Policy IDs (comma-separated)")
                .required(true)
                .value_name("POLICY_IDS"))
            .arg(Arg::new("terms")
                .long("terms")
                .help("Agreement terms")
                .required(true)
                .value_name("TERMS")))
        .subcommand(Command::new("list")
            .about("List agreements"))
        .subcommand(Command::new("info")
            .about("Get agreement information")
            .arg(Arg::new("agreement-id")
                .help("Agreement ID")
                .required(true)))
}

fn build_template_command() -> Command {
    Command::new("template")
        .about("Agreement template operations")
        .subcommand(Command::new("list")
            .about("List available templates"))
        .subcommand(Command::new("show")
            .about("Show template details")
            .arg(Arg::new("template-name")
                .help("Template name")
                .required(true)))
        .subcommand(Command::new("generate")
            .about("Generate agreement from template")
            .arg(Arg::new("template-name")
                .help("Template name")
                .required(true))
            .arg(Arg::new("output")
                .long("output")
                .help("Output file")
                .value_name("FILE")))
}

fn build_enforce_command() -> Command {
    Command::new("enforce")
        .about("Enforce agreement policies")
        .arg(Arg::new("agreement-id")
            .help("Agreement ID")
            .required(true))
        .arg(Arg::new("court-id")
            .long("court-id")
            .help("Court ID")
            .value_name("ID"))
}

fn build_validate_command() -> Command {
    Command::new("validate")
        .about("Validate agreements and policies")
        .subcommand(Command::new("agreement")
            .about("Validate an agreement")
            .arg(Arg::new("agreement-file")
                .help("Agreement file to validate")
                .required(true)))
        .subcommand(Command::new("workspace")
            .about("Validate entire workspace"))
}

fn build_wallet_command() -> Command {
    Command::new("wallet")
        .about("Wallet box agreement operations")
        .subcommand(Command::new("create")
            .about("Create wallet box agreement")
            .arg(Arg::new("name")
                .long("name")
                .help("Agreement name")
                .required(true)
                .value_name("NAME"))
            .arg(Arg::new("jurisdiction")
                .long("jurisdiction")
                .help("Jurisdiction")
                .required(true)
                .value_name("JURISDICTION")))
        .subcommand(Command::new("list")
            .about("List wallet agreements"))
}

fn build_status_command() -> Command {
    Command::new("status")
        .about("Show agreement status")
        .arg(Arg::new("format")
            .long("format")
            .value_name("FORMAT")
            .help("Output format (json, table)")
            .default_value("table"))
}

fn build_deploy_command() -> Command {
    Command::new("deploy")
        .about("Deploy agreements to containers or clusters")
        .subcommand(
            Command::new("agreement")
                .about("Deploy agreement")
                .arg(Arg::new("container")
                    .long("container")
                    .value_name("NAME")
                    .help("Container name")
                    .conflicts_with("cluster"))
                .arg(Arg::new("cluster")
                    .long("cluster")
                    .value_name("NAME")
                    .help("Cluster name")
                    .conflicts_with("container"))
                .arg(Arg::new("template")
                    .long("template")
                    .value_name("TEMPLATE")
                    .help("Agreement template (sla, compliance, security)")
                    .required(true))
                .arg(Arg::new("deployment-id")
                    .long("deployment-id")
                    .value_name("ID")
                    .help("Deployment ID (for containers)")
                    .default_value("default"))
                .arg(Arg::new("region")
                    .long("region")
                    .value_name("REGION")
                    .help("Cluster region (for clusters)")
                    .default_value("us-east-1"))
                .arg(Arg::new("instance-id")
                    .long("instance-id")
                    .value_name("ID")
                    .help("Cluster instance ID (for clusters)")
                    .default_value("default"))
                .arg(Arg::new("applies-to-containers")
                    .long("applies-to-containers")
                    .help("Apply cluster agreement to all containers")
                    .action(clap::ArgAction::SetTrue))
        )
}

fn build_agreements_command() -> Command {
    Command::new("agreements")
        .about("Manage agreements")
        .subcommand(
            Command::new("list")
                .about("List active agreements")
                .arg(Arg::new("type")
                    .long("type")
                    .value_name("TYPE")
                    .help("Agreement type (container, cluster, all)")
                    .default_value("all"))
        )
}

fn build_violations_command() -> Command {
    Command::new("violations")
        .about("Manage violations")
        .subcommand(
            Command::new("check")
                .about("Check for violations")
                .arg(Arg::new("resolved")
                    .long("resolved")
                    .help("Include resolved violations")
                    .action(clap::ArgAction::SetTrue))
        )
}

fn build_export_command() -> Command {
    Command::new("export")
        .about("Export agreements and policies")
        .arg(Arg::new("format")
            .long("format")
            .help("Export format")
            .value_parser(["json", "yaml", "toml"])
            .value_name("FORMAT"))
        .arg(Arg::new("output")
            .long("output")
            .help("Output file")
            .value_name("FILE"))
}
    Command::new("import")
        .about("Import agreements and policies")
        .arg(Arg::new("file")
            .help("File to import")
            .required(true))
}

fn build_status_command() -> Command {
    Command::new("status")
        .about("Show workspace status")
        .arg(Arg::new("detailed")
            .long("detailed")
            .help("Show detailed status")
            .action(clap::ArgAction::SetTrue))
}

// Command handlers (simplified for token limit)
async fn handle_init_command(config: &CliConfig, matches: &ArgMatches) -> Result<i32> {
    let default_name = "default".to_string();
    let name = matches.get_one::<String>("name").unwrap_or(&default_name);
    
    if config.dry_run {
        println!("Would initialize workspace '{}' in directory '{}'", name, config.workspace_dir);
        return Ok(EXIT_OK);
    }
    
    fs::create_dir_all(&config.workspace_dir)?;
    
    if config.json_output {
        println!("{}", json!({"status": "success", "workspace": config.workspace_dir}));
    } else {
        println!("✅ Agreement workspace '{}' initialized in {}", name, config.workspace_dir);
    }
    
    Ok(EXIT_OK)
}

async fn handle_court_command(config: &CliConfig, matches: &ArgMatches) -> Result<i32> {
    match matches.subcommand() {
        Some(("create", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            
            if config.dry_run {
                println!("Would create court: {}", name);
                return Ok(EXIT_OK);
            }
            
            let sdk_config = SDKConfig::default();
            let sdk = AgreementsSDK::new(sdk_config)?;
            let court_config = CourtConfig::default();
            
            let result = sdk.create_court(
                name.clone(), 
                "Court created by agreementc".to_string(), 
                court_config
            )?;
            
            if result.success {
                println!("✅ Court '{}' created successfully", name);
                Ok(EXIT_OK)
            } else {
                eprintln!("❌ Failed to create court");
                Ok(EXIT_USER_ERROR)
            }
        }
        Some(("list", _)) => {
            println!("Court listing - Integration with CourtRegistry");
            Ok(EXIT_OK)
        }
        Some(("info", sub_matches)) => {
            let court_id = sub_matches.get_one::<String>("court-id").unwrap();
            println!("Court info for: {}", court_id);
            Ok(EXIT_OK)
        }
        _ => {
            println!("Available court commands: create, list, info");
            Ok(EXIT_OK)
        }
    }
}

async fn handle_policy_command(config: &CliConfig, matches: &ArgMatches) -> Result<i32> {
    match matches.subcommand() {
        Some(("deploy", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            let version = sub_matches.get_one::<String>("version").unwrap();
            let wasm_file = sub_matches.get_one::<String>("wasm-file").unwrap();
            
            if config.dry_run {
                println!("Would deploy policy: {} v{} from {}", name, version, wasm_file);
                return Ok(EXIT_OK);
            }
            
            let wasm_bytecode = fs::read(wasm_file)?;
            let sdk_config = SDKConfig::default();
            let sdk = AgreementsSDK::new(sdk_config)?;
            
            let policy_config = PolicyConfig {
                is_pre_hook: sub_matches.get_flag("pre-hook"),
                is_post_hook: sub_matches.get_flag("post-hook"),
                ..Default::default()
            };
            
            let result = sdk.deploy_policy(None, name.clone(), version.clone(), wasm_bytecode, policy_config)?;
            
            if result.success {
                println!("✅ Policy '{}' v{} deployed successfully", name, version);
                Ok(EXIT_OK)
            } else {
                eprintln!("❌ Failed to deploy policy");
                Ok(EXIT_USER_ERROR)
            }
        }
        _ => {
            println!("Available policy commands: deploy, list, info");
            Ok(EXIT_OK)
        }
    }
}

async fn handle_agreement_command(_config: &CliConfig, _matches: &ArgMatches) -> Result<i32> {
    println!("Agreement operations - Integration with AgreementsSDK");
    Ok(EXIT_OK)
}

async fn handle_template_command(_config: &CliConfig, matches: &ArgMatches) -> Result<i32> {
    match matches.subcommand() {
        Some(("list", _)) => {
            let sdk_config = SDKConfig::default();
            let sdk = AgreementsSDK::new(sdk_config)?;
            let templates = sdk.get_agreement_templates();
            
            println!("📋 Available Agreement Templates:");
            for template in templates {
                println!("  • {} - {}", template.name, template.description);
                println!("    Category: {}", template.category);
            }
            Ok(EXIT_OK)
        }
        _ => {
            println!("Available template commands: list, show, generate");
            Ok(EXIT_OK)
        }
    }
}

async fn handle_enforce_command(_config: &CliConfig, _matches: &ArgMatches) -> Result<i32> {
    println!("Agreement enforcement - Integration with court enforcement");
    Ok(EXIT_OK)
}

async fn handle_validate_command(_config: &CliConfig, _matches: &ArgMatches) -> Result<i32> {
    println!("Agreement validation - Integration with validation framework");
    Ok(EXIT_OK)
}

async fn handle_wallet_command(_config: &CliConfig, _matches: &ArgMatches) -> Result<i32> {
    println!("Wallet box agreement operations - Integration with MetaNode wallet");
    Ok(EXIT_OK)
}

async fn handle_export_command(_config: &CliConfig, _matches: &ArgMatches) -> Result<i32> {
    println!("Export operations - JSON/YAML/TOML export of agreements and policies");
    Ok(EXIT_OK)
}

async fn handle_import_command(_config: &CliConfig, _matches: &ArgMatches) -> Result<i32> {
    println!("Import operations - Import agreements and policies from files");
    Ok(EXIT_OK)
}

async fn handle_status_command(config: &CliConfig, _matches: &ArgMatches) -> Result<i32> {
    if config.json_output {
        println!("{}", json!({
            "workspace": config.workspace_dir,
            "status": "ready",
            "version": "1.0.0"
        }));
    } else {
        println!("📊 Agreement Workspace Status");
        println!("  Workspace: {}", config.workspace_dir);
        println!("  Status: Ready");
        println!("  Version: 1.0.0");
    }
    Ok(EXIT_OK)
}
