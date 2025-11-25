// BPCI Enterprise - Complete Blockchain Platform Command Interface
// Military-grade security, enterprise governance, autonomous economics

use clap::Parser;
use anyhow::Result;

// Use library modules instead of declaring them as separate modules
use pravyom_enterprise::cli::BpciCli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = BpciCli::parse();
    cli.execute().await
}


