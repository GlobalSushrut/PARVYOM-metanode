// BPCI Enterprise - Complete Blockchain Platform Command Interface
// Military-grade security, enterprise governance, autonomous economics

use clap::Parser;
use anyhow::Result;

mod cli;
mod registry;
mod config;
mod mining;

use cli::BpciCli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = BpciCli::parse();
    cli.execute().await
}


