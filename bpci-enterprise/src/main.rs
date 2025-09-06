// BPCI Enterprise - Complete Blockchain Platform Command Interface
// Military-grade security, enterprise governance, autonomous economics

use clap::Parser;
use anyhow::Result;

mod autonomous_economy;
mod registry;
mod wallet_registry;
mod cli;
mod config;
mod stamped_wallet_api_access;
mod mining;
mod blockchain_helpers;
mod cuedb_agreement;
mod cuedb_manager;
mod dbyml_config;
// Cross-system integration modules
mod court_shadow_bridge;
mod court_bpi_mesh_integration;
mod unified_audit_system;
mod bpi_ledger_integration;
// Revolutionary orchestration system modules
mod metanode_cluster_manager;
mod daemon_tree;
mod smartcontract_policy_agreement;

use cli::BpciCli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = BpciCli::parse();
    cli.execute().await
}


