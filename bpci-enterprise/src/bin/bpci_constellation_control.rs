use std::fmt::Write as FmtWrite;

/// Simple dev-time constellation controller for local laptop runs.
///
/// This does **not** start processes yet. Instead, it centralizes the
/// recommended ports/URLs and prints:
/// - A summary table of what should run where
/// - Environment exports for BSO-K8 + DynaRoute-style deployment
/// - Exact `cargo run` commands for each BPCI component
///
/// Goal: one place to see the constellation layout and copy/paste the
/// commands, avoiding port conflicts and scattered notes.
#[derive(Debug, Clone)]
struct ConstellationConfig {
    consensus_port: u16,
    blockchain_api_port: u16,
    auction_mempool_port: u16,
    auction_db_url: &'static str,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = ConstellationConfig {
        consensus_port: 9001,
        // Use 8082 for blockchain API to avoid conflict with any other 8080 service
        blockchain_api_port: 8082,
        auction_mempool_port: 9004,
        // Local laptop auction DB server (bpci_auction_db_server)
        auction_db_url: "http://127.0.0.1:7002",
    };

    let mut out = String::new();

    writeln!(out, "BPCI Constellation Control (Local Dev Profile)").ok();
    writeln!(out, "================================================").ok();
    writeln!(out, "").ok();

    writeln!(out, "1. Component Layout (Ports / URLs)").ok();
    writeln!(out, "----------------------------------").ok();
    writeln!(out, "Component           | Role                | Address / URL").ok();
    writeln!(out, "--------------------+---------------------+------------------------------").ok();
    writeln!(
        out,
        "Consensus (Comp 1)  | LCCD server        | http://127.0.0.1:{}",
        cfg.consensus_port,
    )
    .ok();
    writeln!(
        out,
        "Blockchain (Comp 2) | API server         | http://127.0.0.1:{}",
        cfg.blockchain_api_port,
    )
    .ok();
    writeln!(
        out,
        "Auction (Comp 3)    | Mempool HTTP       | http://127.0.0.1:{}",
        cfg.auction_mempool_port,
    )
    .ok();
    writeln!(
        out,
        "Auction DB (Comp 4) | Maintainer API     | {}",
        cfg.auction_db_url,
    )
    .ok();
    writeln!(out, "Bridge (Comp 5)    | BPI↔BPCI Bridge     | http://127.0.0.1:6001").ok();
    writeln!(out, "").ok();

    writeln!(out, "2. Environment Exports (BSO-K8 + DynaRoute style)").ok();
    writeln!(out, "--------------------------------------------------").ok();
    writeln!(out, "Export these in your shell **before** starting components:").ok();
    writeln!(out, "").ok();
    writeln!(
        out,
        "export DEPLOYMENT_TYPE=\"BSO-K8 orchestrator\""
    )
    .ok();
    writeln!(out, "export NETWORK_BINDING=\"0.0.0.0 (external access)\"").ok();
    writeln!(out, "export CLUSTER_NAME=\"bpci-local-dev\"").ok();
    writeln!(out, "export NAMESPACE=\"bpci-enterprise\"").ok();
    writeln!(
        out,
        "export BPCI_CONSENSUS_URL=\"http://127.0.0.1:{}\"",
        cfg.consensus_port,
    )
    .ok();
    writeln!(
        out,
        "export BPCI_BLOCKCHAIN_URL=\"http://127.0.0.1:{}\"",
        cfg.blockchain_api_port,
    )
    .ok();
    writeln!(
        out,
        "export BPCI_AUCTION_MEMPOOL_URL=\"http://127.0.0.1:{}\"",
        cfg.auction_mempool_port,
    )
    .ok();
    writeln!(
        out,
        "export BPCI_AUCTION_DB_URL=\"{}\"",
        cfg.auction_db_url,
    )
    .ok();
    writeln!(out, "").ok();

    writeln!(out, "3. Recommended Startup Commands (one per terminal)").ok();
    writeln!(out, "-------------------------------------------------").ok();
    writeln!(out, "From /home/umesh/metanode:").ok();
    writeln!(out, "").ok();

    // Consensus
    writeln!(out, "# Component 1: LCCD Consensus Server").ok();
    writeln!(
        out,
        "cargo run -p pravyom-enterprise --bin bpci-consensus-server -- --port {}",
        cfg.consensus_port,
    )
    .ok();
    writeln!(out, "").ok();

    // Blockchain
    writeln!(out, "# Component 2: BPCI Blockchain Server").ok();
    writeln!(
        out,
        "cargo run -p pravyom-enterprise --bin bpci_blockchain_server -- --api-port {} --consensus-server-url http://127.0.0.1:{}",
        cfg.blockchain_api_port,
        cfg.consensus_port,
    )
    .ok();
    writeln!(out, "").ok();

    // Auction mempool
    writeln!(out, "# Component 3: BPCI Auction Mempool Server").ok();
    writeln!(
        out,
        "cargo run -p pravyom-enterprise --bin bpci_auction_mempool_server -- --api-port {}",
        cfg.auction_mempool_port,
    )
    .ok();
    writeln!(out, "").ok();

    // Auction DB
    writeln!(out, "# Component 4: BPCI Auction DB Server").ok();
    writeln!(
        out,
        "cargo run -p pravyom-enterprise --bin bpci_auction_db_server",
    )
    .ok();
    writeln!(out, "").ok();

    // Bridge
    writeln!(out, "# Component 5: BPCI-BPI Bridge (HTTP on 6001)").ok();
    writeln!(
        out,
        "cargo run -p pravyom-enterprise --bin bpci_bpi_bridge -- --port 6001",
    )
    .ok();
    writeln!(out, "").ok();

    writeln!(out, "Once all components are up and healthy, you can run:").ok();
    writeln!(out, "").ok();
    writeln!(
        out,
        "cargo run -p pravyom-enterprise --bin bpci_constellation_demo"
    )
    .ok();
    writeln!(out, "").ok();

    writeln!(out, "This controller binary is intentionally non-invasive:").ok();
    writeln!(out, "- It uses the same DynaRoute/CommuteLock/BSO-K8 env pattern as the components.").ok();
    writeln!(out, "- It centralizes ports and URLs so there are no accidental conflicts.").ok();
    writeln!(out, "- It prints everything in one place for quick visual inspection.").ok();

    println!("{}", out);

    Ok(())
}
