use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::info;

pub mod wallet_core;
pub mod bpi_integration;
pub mod component_manager;
pub mod metrics;

pub use wallet_core::{WalletCore, WalletStatus};
pub use bpi_integration::BpiIntegration;
pub use component_manager::{ComponentManager, ComponentCategory, ComponentStatus};
pub use metrics::WalletMetrics;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConfig {
    pub wallet_name: String,
    pub network: String,
    pub bpi_core_url: String,
    pub bpci_server_url: String,
    pub auto_start_components: Vec<String>,
    pub security: SecurityConfig,
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub require_password: bool,
    pub session_timeout_minutes: u64,
    pub enable_biometric: bool,
    pub auto_lock: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
    pub language: String,
    pub show_advanced_features: bool,
    pub refresh_interval_ms: u64,
}

impl WalletConfig {
    pub async fn load(path: &str) -> Result<Self> {
        match tokio::fs::read_to_string(path).await {
            Ok(content) => {
                let config: WalletConfig = serde_yaml::from_str(&content)?;
                Ok(config)
            }
            Err(_) => {
                info!("Config file not found, creating default configuration");
                let config = Self::default();
                config.save(path).await?;
                Ok(config)
            }
        }
    }
    
    pub async fn load_or_default() -> Result<Self> {
        Self::load("wallet-config.yaml").await
    }
    
    pub async fn save(&self, path: &str) -> Result<()> {
        let content = serde_yaml::to_string(self)?;
        tokio::fs::write(path, content).await?;
        Ok(())
    }
}

impl Default for WalletConfig {
    fn default() -> Self {
        Self {
            wallet_name: "Pravyom Wallet".to_string(),
            network: "mainnet".to_string(),
            bpi_core_url: "https://www.pravyom.com/bpi-core".to_string(),
            bpci_server_url: "https://www.pravyom.com/connect".to_string(),
            auto_start_components: vec![
                "blockchain_core".to_string(),
                "wallet_registry".to_string(),
                "security_framework".to_string(),
                "consensus_engine".to_string(),
                "network_manager".to_string(),
            ],
            security: SecurityConfig {
                require_password: true,
                session_timeout_minutes: 30,
                enable_biometric: false,
                auto_lock: true,
            },
            ui: UiConfig {
                theme: "dark".to_string(),
                language: "en".to_string(),
                show_advanced_features: false,
                refresh_interval_ms: 1000,
            },
        }
    }
}
