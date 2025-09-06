//! Simple Configuration for Web-4 Mainnet

use crate::NodeId;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

/// Mainnet-ready configuration (simple, reliable)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HermesConfig {
    pub node_id: NodeId,
    pub listen_port: u16,
    pub max_neighbors: usize,
    pub bootstrap_nodes: Vec<String>,
    pub network_mode: NetworkMode,
    pub bpci_integration: BpciConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMode {
    Testnet,
    Mainnet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpciConfig {
    pub enable_consensus_priority: bool,
    pub enable_auction_retry: bool,
    pub shadow_data_background: bool,
    pub consensus_timeout_ms: u64,
    pub auction_timeout_ms: u64,
}

impl Default for HermesConfig {
    fn default() -> Self {
        Self {
            node_id: NodeId::random(),
            listen_port: 9000,
            max_neighbors: 8,
            bootstrap_nodes: vec![
                "node_bootstrap_001".to_string(),
                "node_bootstrap_002".to_string(),
                "node_bootstrap_003".to_string(),
            ],
            network_mode: NetworkMode::Testnet,
            bpci_integration: BpciConfig::default(),
        }
    }
}

impl Default for BpciConfig {
    fn default() -> Self {
        Self {
            enable_consensus_priority: true,
            enable_auction_retry: true,
            shadow_data_background: true,
            consensus_timeout_ms: 1000,  // 1 second for consensus
            auction_timeout_ms: 5000,    // 5 seconds for auctions
        }
    }
}

impl HermesConfig {
    /// Load config from TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: HermesConfig = toml::from_str(&content)?;
        Ok(config)
    }
    
    /// Save config to TOML file
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
    
    /// Create mainnet config
    pub fn mainnet() -> Self {
        Self {
            network_mode: NetworkMode::Mainnet,
            bootstrap_nodes: vec![
                "mainnet.pravyom.net:9000".to_string(),
                "mainnet2.pravyom.net:9000".to_string(),
                "mainnet3.pravyom.net:9000".to_string(),
            ],
            ..Default::default()
        }
    }
    
    /// Create testnet config
    pub fn testnet() -> Self {
        Self {
            network_mode: NetworkMode::Testnet,
            listen_port: 9100, // Different port for testnet
            ..Default::default()
        }
    }
}
