// Configuration module for different deployment modes
// Handles dev (localhost), community (remote), enterprise (testnet/mainnet only)

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpciConfig {
    pub network: NetworkConfig,
    pub node: NodeConfig,
    pub connection: ConnectionConfig,
    pub services: ServicesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub mode: DeploymentMode,
    pub network: String,
    pub rpc_endpoint: String,
    pub testnet_rpc: Option<String>,
    pub local_blockchain: bool,
    pub connect_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub node_type: String,
    pub data_dir: String,
    pub log_level: String,
    pub full_node: bool,
    pub light_client: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub allowed_networks: Vec<String>,
    pub forbidden_networks: Vec<String>,
    pub remote_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicesConfig {
    pub wallet_service: bool,
    pub mining_service: bool,
    pub governance_service: bool,
    pub notary_service: bool,
    pub registry_service: bool,
    pub rpc_service: bool,
    pub api_service: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeploymentMode {
    Development,
    Community,
    Enterprise,
    Server,
}

impl BpciConfig {
    /// Load configuration from file with deployment mode validation
    pub fn load_from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: BpciConfig = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }
    
    /// Validate configuration based on deployment mode
    pub fn validate(&self) -> Result<()> {
        match self.network.mode {
            DeploymentMode::Development => self.validate_dev_mode(),
            DeploymentMode::Community => self.validate_community_mode(),
            DeploymentMode::Enterprise => self.validate_enterprise_mode(),
            DeploymentMode::Server => self.validate_server_mode(),
        }
    }
    
    /// Validate development mode: localhost only, full services
    fn validate_dev_mode(&self) -> Result<()> {
        if !self.network.local_blockchain {
            return Err(anyhow!("Dev mode requires local_blockchain = true"));
        }
        
        if !self.node.full_node {
            return Err(anyhow!("Dev mode requires full_node = true"));
        }
        
        if self.network.connect_only {
            return Err(anyhow!("Dev mode cannot be connect_only"));
        }
        
        // Dev mode should have localhost/localnet
        let allowed_dev_networks = ["localhost", "localnet", "devnet"];
        if !allowed_dev_networks.contains(&self.network.network.as_str()) {
            return Err(anyhow!("Dev mode must use localhost/localnet/devnet"));
        }
        
        Ok(())
    }
    
    /// Validate community mode: remote connection only
    fn validate_community_mode(&self) -> Result<()> {
        if self.network.local_blockchain {
            return Err(anyhow!("Community mode cannot run local blockchain"));
        }
        
        if !self.connection.remote_only {
            return Err(anyhow!("Community mode requires remote_only = true"));
        }
        
        self.validate_remote_networks()
    }
    
    /// Validate enterprise mode: testnet/mainnet only, no local blockchain
    fn validate_enterprise_mode(&self) -> Result<()> {
        if self.network.local_blockchain {
            return Err(anyhow!("Enterprise mode cannot run local blockchain"));
        }
        
        if !self.connection.remote_only {
            return Err(anyhow!("Enterprise mode requires remote_only = true"));
        }
        
        // Enterprise can only connect to testnet/mainnet
        let allowed_networks = ["mainnet", "testnet"];
        if !allowed_networks.contains(&self.network.network.as_str()) {
            return Err(anyhow!("Enterprise mode only allows mainnet/testnet"));
        }
        
        // Check forbidden networks
        let forbidden: HashSet<String> = self.connection.forbidden_networks.iter().cloned().collect();
        let local_networks = ["localhost", "localnet", "devnet"];
        for network in &local_networks {
            if !forbidden.contains(*network) {
                return Err(anyhow!("Enterprise mode must forbid local networks"));
            }
        }
        
        self.validate_remote_networks()
    }
    
    /// Validate server mode: full services, can run local or remote
    fn validate_server_mode(&self) -> Result<()> {
        if !self.node.full_node {
            return Err(anyhow!("Server mode requires full_node = true"));
        }
        
        // Server should have all services enabled
        if !self.services.registry_service {
            return Err(anyhow!("Server mode requires registry_service = true"));
        }
        
        Ok(())
    }
    
    /// Validate remote network configuration
    fn validate_remote_networks(&self) -> Result<()> {
        if !self.network.connect_only {
            return Err(anyhow!("Remote modes require connect_only = true"));
        }
        
        if self.node.full_node {
            return Err(anyhow!("Remote modes should use light_client = true"));
        }
        
        Ok(())
    }
    
    /// Check if network is allowed for current mode
    pub fn is_network_allowed(&self, network: &str) -> bool {
        match self.network.mode {
            DeploymentMode::Development => {
                ["localhost", "localnet", "devnet"].contains(&network)
            },
            DeploymentMode::Community | DeploymentMode::Enterprise => {
                ["mainnet", "testnet"].contains(&network) && 
                !["localhost", "localnet", "devnet"].contains(&network)
            },
            DeploymentMode::Server => true, // Server can connect to any network
        }
    }
    
    /// Get appropriate RPC endpoint for network
    pub fn get_rpc_endpoint(&self, network: &str) -> Result<String> {
        if !self.is_network_allowed(network) {
            return Err(anyhow!("Network '{}' not allowed for {:?} mode", network, self.network.mode));
        }
        
        match network {
            "mainnet" => Ok(self.network.rpc_endpoint.clone()),
            "testnet" => {
                self.network.testnet_rpc.clone()
                    .ok_or_else(|| anyhow!("Testnet RPC not configured"))
            },
            "localhost" | "localnet" | "devnet" => {
                if self.network.mode == DeploymentMode::Development {
                    Ok("http://localhost:8545".to_string())
                } else {
                    Err(anyhow!("Local networks not allowed for {:?} mode", self.network.mode))
                }
            },
            _ => Err(anyhow!("Unknown network: {}", network))
        }
    }
}
