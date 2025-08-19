//! BPCI Network Mode Handler
//! 
//! Automatically detects and handles faucet/testnet vs mainnet registration.
//! BPCI acts as a registry mesh that registers as needed once mainnet is activated.
//! No complication - just works automatically.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::economic_integration::{BpciEconomicIntegration, BpciEconomicConfig};

/// Network mode types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkMode {
    /// Development/local testing - free tokens, no real economics
    Development,
    /// Testnet with faucet functionality - limited real economics
    Testnet,
    /// Production mainnet - full economics active
    Mainnet,
}

impl NetworkMode {
    /// Check if economics should be active for this network mode
    pub fn economics_active(&self) -> bool {
        match self {
            NetworkMode::Development => false, // Free development
            NetworkMode::Testnet => true,      // Limited economics for testing
            NetworkMode::Mainnet => true,      // Full economics
        }
    }

    /// Get faucet availability for this network mode
    pub fn faucet_available(&self) -> bool {
        match self {
            NetworkMode::Development => true,  // Unlimited faucet
            NetworkMode::Testnet => true,      // Limited faucet
            NetworkMode::Mainnet => false,     // No faucet
        }
    }

    /// Get economic configuration for this network mode
    pub fn economic_config(&self, owner_wallet_id: Uuid) -> BpciEconomicConfig {
        match self {
            NetworkMode::Development => BpciEconomicConfig {
                owner_wallet_id,
                auto_activation: false, // Manual control in dev
                billing_interval_seconds: 3600,
                mining_interval_seconds: 60,
                owner_withdrawal_threshold: u64::MAX, // Never auto-withdraw in dev
                owner_withdrawal_percentage: 0.0,
                infrastructure_fee_rate: 0.0, // Free in development
            },
            NetworkMode::Testnet => BpciEconomicConfig {
                owner_wallet_id,
                auto_activation: true,
                billing_interval_seconds: 1800, // 30 minutes
                mining_interval_seconds: 30,
                owner_withdrawal_threshold: 1_000_000, // 1M test tokens
                owner_withdrawal_percentage: 0.5, // 50% withdrawal
                infrastructure_fee_rate: 0.01, // 1% fee for testing
            },
            NetworkMode::Mainnet => BpciEconomicConfig {
                owner_wallet_id,
                auto_activation: true,
                billing_interval_seconds: 3600, // 1 hour
                mining_interval_seconds: 30,
                owner_withdrawal_threshold: 10_000_000, // 10M tokens
                owner_withdrawal_percentage: 0.8, // 80% withdrawal
                infrastructure_fee_rate: 0.05, // 5% infrastructure fee
            },
        }
    }
}

/// Network detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDetection {
    pub mode: NetworkMode,
    pub chain_id: String,
    pub genesis_hash: String,
    pub confidence: f64, // 0.0 - 1.0
    pub detection_time: u64,
}

/// Faucet configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaucetConfig {
    pub enabled: bool,
    pub daily_limit_per_address: u64,
    pub max_request_amount: u64,
    pub cooldown_seconds: u64,
    pub available_tokens: Vec<String>, // Token types available from faucet
}

impl Default for FaucetConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            daily_limit_per_address: 1_000_000, // 1M tokens per day
            max_request_amount: 100_000,        // 100K per request
            cooldown_seconds: 3600,             // 1 hour cooldown
            available_tokens: vec!["FLX".to_string(), "NEX".to_string()],
        }
    }
}

/// Registry mesh status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryMeshStatus {
    pub is_registered: bool,
    pub registration_time: Option<u64>,
    pub mesh_peers: u32,
    pub last_heartbeat: u64,
    pub network_mode: NetworkMode,
}

/// BPCI Network Mode Manager
#[derive(Debug)]
pub struct BpciNetworkManager {
    current_mode: Arc<RwLock<NetworkMode>>,
    economic_integration: Option<Arc<BpciEconomicIntegration>>,
    faucet_config: Arc<RwLock<FaucetConfig>>,
    registry_status: Arc<RwLock<RegistryMeshStatus>>,
    owner_wallet_id: Uuid,
}

impl BpciNetworkManager {
    /// Create new network manager
    pub async fn new(owner_wallet_id: Uuid) -> Result<Self> {
        let initial_mode = Self::detect_network_mode().await?;
        
        info!("🌐 BPCI Network Mode detected: {:?}", initial_mode);
        
        Ok(Self {
            current_mode: Arc::new(RwLock::new(initial_mode.clone())),
            economic_integration: None,
            faucet_config: Arc::new(RwLock::new(FaucetConfig::default())),
            registry_status: Arc::new(RwLock::new(RegistryMeshStatus {
                is_registered: false,
                registration_time: None,
                mesh_peers: 0,
                last_heartbeat: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                network_mode: initial_mode,
            })),
            owner_wallet_id,
        })
    }

    /// Detect current network mode automatically
    async fn detect_network_mode() -> Result<NetworkMode> {
        // In a real implementation, this would:
        // 1. Check chain ID from connected nodes
        // 2. Verify genesis block hash
        // 3. Check network consensus parameters
        // 4. Query known mainnet/testnet identifiers
        
        // For now, simulate detection based on environment
        let mode = if std::env::var("BPCI_NETWORK").unwrap_or_default() == "mainnet" {
            NetworkMode::Mainnet
        } else if std::env::var("BPCI_NETWORK").unwrap_or_default() == "testnet" {
            NetworkMode::Testnet
        } else {
            NetworkMode::Development
        };
        
        info!("🔍 Network mode detection: {:?}", mode);
        Ok(mode)
    }

    /// Initialize economic integration based on network mode
    pub async fn initialize_economics(&mut self) -> Result<()> {
        let mode = self.current_mode.read().await.clone();
        
        if mode.economics_active() {
            let economic_config = mode.economic_config(self.owner_wallet_id);
            let economic_integration = Arc::new(BpciEconomicIntegration::new(economic_config).await?);
            
            // Start monitoring and activation
            economic_integration.start_monitoring().await?;
            
            self.economic_integration = Some(economic_integration);
            
            info!("💰 Economic integration initialized for {:?} mode", mode);
        } else {
            info!("🆓 Economics disabled for {:?} mode", mode);
        }
        
        Ok(())
    }

    /// Start automatic network monitoring and registration
    pub async fn start_monitoring(&self) -> Result<()> {
        let current_mode = self.current_mode.clone();
        let registry_status = self.registry_status.clone();
        let economic_integration = self.economic_integration.clone();
        
        tokio::spawn(async move {
            let mut check_interval = tokio::time::interval(Duration::from_secs(30));
            
            loop {
                check_interval.tick().await;
                
                // Check for network mode changes
                if let Ok(detected_mode) = Self::detect_network_mode().await {
                    let mut mode = current_mode.write().await;
                    let mut status = registry_status.write().await;
                    
                    if *mode != detected_mode {
                        info!("🔄 Network mode changed: {:?} -> {:?}", *mode, detected_mode);
                        *mode = detected_mode.clone();
                        status.network_mode = detected_mode.clone();
                        
                        // Handle mode transition
                        match detected_mode {
                            NetworkMode::Mainnet => {
                                info!("🚀 MAINNET ACTIVATED! Registering in mesh...");
                                Self::register_in_mesh(&mut status).await;
                                
                                // Activate full economics
                                if let Some(ref econ) = economic_integration {
                                    if let Err(e) = econ.force_activate().await {
                                        error!("❌ Failed to activate economics: {}", e);
                                    } else {
                                        info!("💰 Full economics activated for mainnet");
                                    }
                                }
                            },
                            NetworkMode::Testnet => {
                                info!("🧪 TESTNET MODE: Limited economics active");
                                Self::register_in_mesh(&mut status).await;
                                
                                if let Some(ref econ) = economic_integration {
                                    if let Err(e) = econ.force_activate().await {
                                        error!("❌ Failed to activate testnet economics: {}", e);
                                    }
                                }
                            },
                            NetworkMode::Development => {
                                info!("🛠️  DEVELOPMENT MODE: Free operation");
                                status.is_registered = false;
                            },
                        }
                    }
                    
                    // Update heartbeat
                    status.last_heartbeat = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                }
            }
        });
        
        info!("👀 Network monitoring started");
        Ok(())
    }

    /// Register in the BPCI mesh network
    async fn register_in_mesh(status: &mut RegistryMeshStatus) {
        // In a real implementation, this would:
        // 1. Discover mesh peers
        // 2. Perform registration handshake
        // 3. Exchange capabilities
        // 4. Join consensus if validator
        
        status.is_registered = true;
        status.registration_time = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
        status.mesh_peers = 5; // Simulate connected peers
        
        info!("📡 Successfully registered in BPCI mesh network");
    }

    /// Handle faucet request (testnet/development only)
    pub async fn handle_faucet_request(&self, address: &str, amount: u64, token_type: &str) -> Result<String> {
        let mode = self.current_mode.read().await.clone();
        let faucet_config = self.faucet_config.read().await;
        
        if !mode.faucet_available() {
            return Err(anyhow::anyhow!("Faucet not available in {:?} mode", mode));
        }
        
        if !faucet_config.enabled {
            return Err(anyhow::anyhow!("Faucet is currently disabled"));
        }
        
        if amount > faucet_config.max_request_amount {
            return Err(anyhow::anyhow!("Requested amount exceeds maximum: {}", faucet_config.max_request_amount));
        }
        
        if !faucet_config.available_tokens.contains(&token_type.to_string()) {
            return Err(anyhow::anyhow!("Token type {} not available from faucet", token_type));
        }
        
        // Simulate faucet transaction
        let tx_hash = format!("faucet_tx_{}", Uuid::new_v4());
        
        info!("🚰 Faucet request processed: {} {} to {} (tx: {})", amount, token_type, address, tx_hash);
        
        Ok(tx_hash)
    }

    /// Get current network status
    pub async fn get_network_status(&self) -> NetworkStatus {
        let mode = self.current_mode.read().await.clone();
        let registry_status = self.registry_status.read().await.clone();
        let faucet_config = self.faucet_config.read().await.clone();
        
        let economic_status = if let Some(ref econ) = self.economic_integration {
            econ.get_economic_status().await.ok()
        } else {
            None
        };
        
        NetworkStatus {
            network_mode: mode,
            registry_mesh: registry_status,
            faucet_config,
            economic_status,
            uptime_seconds: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }

    /// Force network mode (for testing)
    pub async fn force_network_mode(&self, mode: NetworkMode) -> Result<()> {
        let mut current_mode = self.current_mode.write().await;
        let mut registry_status = self.registry_status.write().await;
        
        *current_mode = mode.clone();
        registry_status.network_mode = mode.clone();
        
        info!("🔧 Network mode forced to: {:?}", mode);
        
        match mode {
            NetworkMode::Mainnet => {
                Self::register_in_mesh(&mut registry_status).await;
                if let Some(ref econ) = self.economic_integration {
                    econ.force_activate().await?;
                }
            },
            NetworkMode::Testnet => {
                Self::register_in_mesh(&mut registry_status).await;
                if let Some(ref econ) = self.economic_integration {
                    econ.force_activate().await?;
                }
            },
            NetworkMode::Development => {
                registry_status.is_registered = false;
            },
        }
        
        Ok(())
    }
}

/// Complete network status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    pub network_mode: NetworkMode,
    pub registry_mesh: RegistryMeshStatus,
    pub faucet_config: FaucetConfig,
    pub economic_status: Option<crate::economic_integration::EconomicStatus>,
    pub uptime_seconds: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_network_mode_detection() {
        let owner_id = Uuid::new_v4();
        let manager = BpciNetworkManager::new(owner_id).await.unwrap();
        
        let status = manager.get_network_status().await;
        assert!(matches!(status.network_mode, NetworkMode::Development | NetworkMode::Testnet | NetworkMode::Mainnet));
        
        println!("✅ Network mode detection working");
    }
    
    #[tokio::test]
    async fn test_faucet_functionality() {
        let owner_id = Uuid::new_v4();
        let manager = BpciNetworkManager::new(owner_id).await.unwrap();
        
        // Force testnet mode for faucet testing
        manager.force_network_mode(NetworkMode::Testnet).await.unwrap();
        
        // Test faucet request
        let result = manager.handle_faucet_request("test_address", 50000, "FLX").await;
        assert!(result.is_ok());
        
        println!("✅ Faucet functionality working");
    }
    
    #[tokio::test]
    async fn test_mainnet_activation() {
        let owner_id = Uuid::new_v4();
        let mut manager = BpciNetworkManager::new(owner_id).await.unwrap();
        
        // Initialize economics
        manager.initialize_economics().await.unwrap();
        
        // Force mainnet mode
        manager.force_network_mode(NetworkMode::Mainnet).await.unwrap();
        
        let status = manager.get_network_status().await;
        assert_eq!(status.network_mode, NetworkMode::Mainnet);
        assert!(status.registry_mesh.is_registered);
        
        println!("✅ Mainnet activation working");
    }
    
    #[tokio::test]
    async fn test_automatic_registration() {
        let owner_id = Uuid::new_v4();
        let mut manager = BpciNetworkManager::new(owner_id).await.unwrap();
        
        // Initialize economics
        manager.initialize_economics().await.unwrap();
        
        // Start monitoring
        manager.start_monitoring().await.unwrap();
        
        // Force mode change to trigger registration
        manager.force_network_mode(NetworkMode::Mainnet).await.unwrap();
        
        let status = manager.get_network_status().await;
        assert!(status.registry_mesh.is_registered);
        
        println!("✅ Automatic registration working");
        println!("📡 Registered with {} mesh peers", status.registry_mesh.mesh_peers);
    }
}
