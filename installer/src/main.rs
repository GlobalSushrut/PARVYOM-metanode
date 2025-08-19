use clap::{Arg, Command, SubCommand};
use std::process;
use tokio;

mod commands;
mod config;
mod dashboard;
mod wallet;

use commands::*;

#[tokio::main]
async fn main() {
    let app = Command::new("metanode")
        .version("1.0.0")
        .about("🚀 Metanode - Military-grade blockchain infrastructure made simple")
        .author("Metanode Team")
        .subcommand(
            Command::new("init")
                .about("Initialize a new Metanode project")
                .arg(
                    Arg::new("name")
                        .help("Project name")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("template")
                        .long("template")
                        .short('t')
                        .help("Project template")
                        .value_parser(["dapp", "defi", "nft", "enterprise", "bridge"])
                        .default_value("dapp")
                )
        )
        .subcommand(
            Command::new("start")
                .about("Start local Metanode services")
                .arg(
                    Arg::new("network")
                        .long("network")
                        .short('n')
                        .help("Network to connect to")
                        .value_parser(["local", "testnet", "mainnet"])
                        .default_value("local")
                )
                .arg(
                    Arg::new("dashboard")
                        .long("dashboard")
                        .short('d')
                        .help("Open dashboard automatically")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("dashboard")
                .about("Open Metanode dashboard")
                .arg(
                    Arg::new("type")
                        .long("type")
                        .short('t')
                        .help("Dashboard type")
                        .value_parser(["bpci", "bpi", "wallet", "all"])
                        .default_value("all")
                )
        )
        .subcommand(
            Command::new("wallet")
                .about("Wallet operations")
                .subcommand(
                    Command::new("create")
                        .about("Create new wallet")
                        .arg(
                            Arg::new("type")
                                .long("type")
                                .short('t')
                                .help("Wallet type")
                                .value_parser(["docklock", "dao", "metanode"])
                                .default_value("metanode")
                        )
                )
                .subcommand(
                    Command::new("balance")
                        .about("Check wallet balance")
                )
                .subcommand(
                    Command::new("send")
                        .about("Send tokens")
                        .arg(Arg::new("to").help("Recipient address").required(true))
                        .arg(Arg::new("amount").help("Amount to send").required(true))
                        .arg(Arg::new("token").help("Token type").default_value("GEN"))
                )
        )
        .subcommand(
            Command::new("mine")
                .about("Mining operations")
                .subcommand(Command::new("start").about("Start mining"))
                .subcommand(Command::new("stop").about("Stop mining"))
                .subcommand(Command::new("status").about("Mining status"))
                .subcommand(Command::new("rewards").about("Check mining rewards"))
        )
        .subcommand(
            Command::new("connect")
                .about("Connect to BPCI network")
                .arg(
                    Arg::new("network")
                        .help("Network to connect to")
                        .required(true)
                        .value_parser(["mainnet", "testnet", "local"])
                )
                .arg(
                    Arg::new("endpoint")
                        .long("endpoint")
                        .short('e')
                        .help("Custom endpoint URL")
                )
        )
        .subcommand(
            Command::new("deploy")
                .about("Deploy application")
                .arg(
                    Arg::new("target")
                        .long("target")
                        .short('t')
                        .help("Deployment target")
                        .value_parser(["local", "testnet", "mainnet"])
                        .default_value("local")
                )
        )
        .subcommand(
            Command::new("status")
                .about("Show system status")
        );

    let matches = app.get_matches();

    let result = match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            let template = sub_matches.get_one::<String>("template").unwrap();
            init_project(name, template).await
        }
        Some(("start", sub_matches)) => {
            let network = sub_matches.get_one::<String>("network").unwrap();
            let open_dashboard = sub_matches.get_flag("dashboard");
            start_services(network, open_dashboard).await
        }
        Some(("dashboard", sub_matches)) => {
            let dashboard_type = sub_matches.get_one::<String>("type").unwrap();
            open_dashboard(dashboard_type).await
        }
        Some(("wallet", sub_matches)) => {
            handle_wallet_commands(sub_matches).await
        }
        Some(("mine", sub_matches)) => {
            handle_mining_commands(sub_matches).await
        }
        Some(("connect", sub_matches)) => {
            let network = sub_matches.get_one::<String>("network").unwrap();
            let endpoint = sub_matches.get_one::<String>("endpoint");
            connect_to_network(network, endpoint).await
        }
        Some(("deploy", sub_matches)) => {
            let target = sub_matches.get_one::<String>("target").unwrap();
            deploy_application(target).await
        }
        Some(("status", _)) => {
            show_system_status().await
        }
        _ => {
            println!("🚀 Welcome to Metanode!");
            println!("Run 'metanode --help' for available commands.");
            println!("\n🎯 Quick Start:");
            println!("  metanode init my-project    # Create new project");
            println!("  metanode start --dashboard  # Start with dashboard");
            println!("  metanode wallet create      # Create wallet");
            println!("  metanode mine start         # Start mining");
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("❌ Error: {}", e);
        process::exit(1);
    }
}
