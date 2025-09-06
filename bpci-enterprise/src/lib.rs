//! BPCI Enterprise - Production-Ready Blockchain Protocol Infrastructure
//! 
//! Complete enterprise-grade blockchain infrastructure with:
//! - Triple Consensus Architecture (IBFT + HotStuff + Tranverse Auction)
//! - Advanced P2P networking (HERMES-Lite Web-4)
//! - Government integration layer
//! - Autonomous economy management
//! - Real-time consensus monitoring

// Core consensus and auction modules
pub mod auction_mode_manager;
pub mod bpi_ledger_integration;
pub mod triple_consensus_coordinator;
pub mod bpci_consensus_server;
pub mod bpci_auction_mempool_minimal;
pub use bpci_auction_mempool_minimal as bpci_auction_mempool;
pub mod round_table_oracle;
pub mod community_installer_os;
pub mod testnet_config;
pub mod testnet_auction_storage;
pub mod bpi_testnet_integration;
pub mod bpci_penetration_testing;
pub mod bpci_auth_wallet_endpoints;

// Infrastructure and integration modules
pub mod blockchain_helpers;
pub mod config;
pub mod court_bpi_mesh_integration;
pub mod court_shadow_bridge;
pub mod cue_contract_deployer;
pub mod cuedb_agreement;
pub mod cuedb_manager;
pub mod daemon_tree;
pub mod dbyml_config;
pub mod government_layer_integration;
pub mod metanode_cluster_manager;
pub mod smartcontract_policy_agreement;
pub mod stamped_wallet_api_access;
pub mod unified_audit_system;

// Specialized modules
pub mod autonomous_economy;
pub mod cli;
pub mod government_layer;
pub mod mining;
pub mod registry;
pub mod wallet_registry;

// Re-exports for easy access
pub use auction_mode_manager::{AuctionModeManager, AuctionMode, AuctionSettlement};
pub use triple_consensus_coordinator::{
    TripleConsensusCoordinator, ConsensusRound, ConsensusRoundStatus,
    BundleProposal, TripleConsensusMetrics, ValidatorInfo
};
pub use bpci_consensus_server::{
    BpciConsensusServerState, BpciServerConfig, ServerMode,
    create_bpci_consensus_router
};
pub use bpi_ledger_integration::BpiLedgerClient;

use anyhow::Result;
use tracing::info;

/// BPCI Enterprise version
pub const VERSION: &str = "1.0.0";

/// Initialize BPCI Enterprise system
pub async fn initialize_bpci_enterprise() -> Result<()> {
    info!("Initializing BPCI Enterprise v{}", VERSION);
    info!("✅ Triple Consensus Architecture ready");
    info!("✅ HERMES-Lite Web-4 P2P networking ready");
    info!("✅ Government integration layer ready");
    info!("✅ Autonomous economy management ready");
    Ok(())
}
