//! BPCI Testnet Configuration System
//! 
//! Provides clean separation between testnet and mainnet deployments
//! while maintaining single codebase for future mainnet deployment

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use anyhow::{Result, anyhow};

/// Network deployment mode
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkMode {
    /// Testnet mode - mock execution, database storage
    Testnet {
        mock_execution: bool,
        database_storage: bool,
        simulated_partners: bool,
    },
    /// Mainnet mode - real BPI integration, live execution
    Mainnet {
        bpi_integration: bool,
        live_execution: bool,
        real_partners: bool,
    },
}

/// BPCI deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpciConfig {
    pub network_mode: NetworkMode,
    pub server_config: ServerConfig,
    pub auction_config: AuctionConfig,
    pub database_config: DatabaseConfig,
    pub partner_config: PartnerConfig,
    pub monitoring_config: MonitoringConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub listen_address: String,
    pub listen_port: u16,
    pub enable_cors: bool,
    pub enable_websockets: bool,
    pub max_concurrent_connections: usize,
    pub request_timeout_seconds: u64,
}

/// Auction system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionConfig {
    pub window_duration_ms: u64,
    pub max_transactions_per_window: u32,
    pub total_gas_limit: u64,
    pub minimum_bid_amount: u64,
    pub revenue_share_percentage: f64, // 25% for partners
    pub enable_merkle_proofs: bool,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub enable_cuedb: bool,
    pub storage_path: String,
    pub max_storage_gb: u64,
    pub enable_audit_trail: bool,
    pub compliance_level: ComplianceLevel,
}

/// Partner chain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerConfig {
    pub mock_partners: Vec<MockPartnerChain>,
    pub enable_notifications: bool,
    pub revenue_distribution_interval_ms: u64,
    pub max_partner_chains: u32,
}

/// Monitoring and metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub enable_metrics: bool,
    pub metrics_port: u16,
    pub enable_dashboard: bool,
    pub update_interval_ms: u64,
    pub enable_alerts: bool,
}

/// Mock partner chain for testnet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockPartnerChain {
    pub chain_id: u64,
    pub name: String,
    pub revenue_share_percentage: f64,
    pub simulated_tps: u32,
    pub mock_endpoint: String,
}

/// Compliance level for database operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceLevel {
    Basic,
    Enhanced,
    Enterprise,
    Government,
}

impl BpciConfig {
    /// Create testnet configuration
    pub fn testnet() -> Self {
        Self {
            network_mode: NetworkMode::Testnet {
                mock_execution: true,
                database_storage: true,
                simulated_partners: true,
            },
            server_config: ServerConfig {
                listen_address: "0.0.0.0".to_string(),
                listen_port: 8080,
                enable_cors: true,
                enable_websockets: true,
                max_concurrent_connections: 1000,
                request_timeout_seconds: 30,
            },
            auction_config: AuctionConfig {
                window_duration_ms: 10000, // 10 seconds for testnet
                max_transactions_per_window: 100,
                total_gas_limit: 10_000_000,
                minimum_bid_amount: 1000, // Lower for testnet
                revenue_share_percentage: 0.25, // 25% to partners
                enable_merkle_proofs: true,
            },
            database_config: DatabaseConfig {
                enable_cuedb: true,
                storage_path: "./testnet_data".to_string(),
                max_storage_gb: 10, // Smaller for testnet
                enable_audit_trail: true,
                compliance_level: ComplianceLevel::Enhanced,
            },
            partner_config: PartnerConfig {
                mock_partners: vec![
                    MockPartnerChain {
                        chain_id: 137,
                        name: "Polygon Testnet".to_string(),
                        revenue_share_percentage: 0.25,
                        simulated_tps: 50,
                        mock_endpoint: "https://polygon-mumbai.g.alchemy.com/v2/demo".to_string(),
                    },
                    MockPartnerChain {
                        chain_id: 56,
                        name: "BSC Testnet".to_string(),
                        revenue_share_percentage: 0.25,
                        simulated_tps: 30,
                        mock_endpoint: "https://data-seed-prebsc-1-s1.binance.org:8545".to_string(),
                    },
                    MockPartnerChain {
                        chain_id: 43113,
                        name: "Avalanche Fuji".to_string(),
                        revenue_share_percentage: 0.25,
                        simulated_tps: 40,
                        mock_endpoint: "https://api.avax-test.network/ext/bc/C/rpc".to_string(),
                    },
                ],
                enable_notifications: true,
                revenue_distribution_interval_ms: 30000, // 30 seconds
                max_partner_chains: 10,
            },
            monitoring_config: MonitoringConfig {
                enable_metrics: true,
                metrics_port: 9090,
                enable_dashboard: true,
                update_interval_ms: 2000, // 2 seconds
                enable_alerts: false, // Disabled for testnet
            },
        }
    }

    /// Create mainnet configuration
    pub fn mainnet() -> Self {
        Self {
            network_mode: NetworkMode::Mainnet {
                bpi_integration: true,
                live_execution: true,
                real_partners: true,
            },
            server_config: ServerConfig {
                listen_address: "0.0.0.0".to_string(),
                listen_port: 443, // HTTPS for mainnet
                enable_cors: false, // Stricter for mainnet
                enable_websockets: true,
                max_concurrent_connections: 10000,
                request_timeout_seconds: 60,
            },
            auction_config: AuctionConfig {
                window_duration_ms: 60000, // 1 minute for mainnet
                max_transactions_per_window: 1000,
                total_gas_limit: 100_000_000,
                minimum_bid_amount: 10000, // Higher for mainnet
                revenue_share_percentage: 0.25, // 25% to partners
                enable_merkle_proofs: true,
            },
            database_config: DatabaseConfig {
                enable_cuedb: true,
                storage_path: "/var/lib/bpci/mainnet_data".to_string(),
                max_storage_gb: 1000, // Larger for mainnet
                enable_audit_trail: true,
                compliance_level: ComplianceLevel::Enterprise,
            },
            partner_config: PartnerConfig {
                mock_partners: vec![], // No mock partners in mainnet
                enable_notifications: true,
                revenue_distribution_interval_ms: 300000, // 5 minutes
                max_partner_chains: 100,
            },
            monitoring_config: MonitoringConfig {
                enable_metrics: true,
                metrics_port: 9090,
                enable_dashboard: true,
                update_interval_ms: 5000, // 5 seconds
                enable_alerts: true, // Enabled for mainnet
            },
        }
    }

    /// Load configuration from environment
    pub fn from_env() -> Result<Self> {
        let network_mode = std::env::var("BPCI_NETWORK_MODE")
            .unwrap_or_else(|_| "testnet".to_string());

        match network_mode.as_str() {
            "testnet" => Ok(Self::testnet()),
            "mainnet" => Ok(Self::mainnet()),
            _ => Err(anyhow!("Invalid network mode: {}", network_mode)),
        }
    }

    /// Check if running in testnet mode
    pub fn is_testnet(&self) -> bool {
        matches!(self.network_mode, NetworkMode::Testnet { .. })
    }

    /// Check if running in mainnet mode
    pub fn is_mainnet(&self) -> bool {
        matches!(self.network_mode, NetworkMode::Mainnet { .. })
    }

    /// Get BPI endpoints based on network mode
    pub fn get_bpi_endpoints(&self) -> HashMap<String, String> {
        let mut endpoints = HashMap::new();

        match &self.network_mode {
            NetworkMode::Testnet { .. } => {
                // Testnet BPI endpoints (local or testnet)
                endpoints.insert("enc_cluster".to_string(), "http://127.0.0.1:9001".to_string());
                endpoints.insert("oracle_node".to_string(), "http://127.0.0.1:9002".to_string());
                endpoints.insert("shadow_registry".to_string(), "http://127.0.0.1:9003".to_string());
                endpoints.insert("pipeline_api".to_string(), "http://127.0.0.1:9004".to_string());
                endpoints.insert("storage_node".to_string(), "http://127.0.0.1:9005".to_string());
                endpoints.insert("proof_node".to_string(), "http://127.0.0.1:9006".to_string());
                endpoints.insert("audit_node".to_string(), "http://127.0.0.1:9007".to_string());
                endpoints.insert("logbook_node".to_string(), "http://127.0.0.1:9008".to_string());
            }
            NetworkMode::Mainnet { .. } => {
                // Mainnet BPI endpoints (production)
                endpoints.insert("enc_cluster".to_string(), "https://bpi-mainnet.network:9001".to_string());
                endpoints.insert("oracle_node".to_string(), "https://oracle.bpi-mainnet.network:9002".to_string());
                endpoints.insert("shadow_registry".to_string(), "https://registry.bpi-mainnet.network:9003".to_string());
                endpoints.insert("pipeline_api".to_string(), "https://pipeline.bpi-mainnet.network:9004".to_string());
                endpoints.insert("storage_node".to_string(), "https://storage.bpi-mainnet.network:9005".to_string());
                endpoints.insert("proof_node".to_string(), "https://proof.bpi-mainnet.network:9006".to_string());
                endpoints.insert("audit_node".to_string(), "https://audit.bpi-mainnet.network:9007".to_string());
                endpoints.insert("logbook_node".to_string(), "https://logbook.bpi-mainnet.network:9008".to_string());
            }
        }

        endpoints
    }
}

impl Default for BpciConfig {
    fn default() -> Self {
        Self::testnet()
    }
}

/// Global configuration instance
static mut GLOBAL_CONFIG: Option<BpciConfig> = None;
static CONFIG_INIT: std::sync::Once = std::sync::Once::new();

/// Initialize global configuration
pub fn init_config() -> Result<()> {
    CONFIG_INIT.call_once(|| {
        let config = BpciConfig::from_env().unwrap_or_default();
        unsafe {
            GLOBAL_CONFIG = Some(config);
        }
    });
    Ok(())
}

/// Get global configuration
pub fn get_config() -> &'static BpciConfig {
    unsafe {
        GLOBAL_CONFIG.as_ref().expect("Configuration not initialized. Call init_config() first.")
    }
}

/// Check if running in testnet mode (global)
pub fn is_testnet() -> bool {
    get_config().is_testnet()
}

/// Check if running in mainnet mode (global)
pub fn is_mainnet() -> bool {
    get_config().is_mainnet()
}
