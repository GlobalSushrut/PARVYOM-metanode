use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use super::{WalletConfig, BpiIntegration, ComponentManager, WalletMetrics};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    pub address: String,
    pub network: String,
    pub balance: f64,
    pub status: WalletStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletStatus {
    Locked,
    Unlocked,
    Connecting,
    Connected,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub fee: f64,
    pub status: TransactionStatus,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub block_hash: Option<String>,
    pub confirmations: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
    Cancelled,
}

pub struct WalletCore {
    pub config: WalletConfig,
    pub wallet_info: Arc<RwLock<WalletInfo>>,
    pub bpi_integration: Arc<BpiIntegration>,
    pub component_manager: Arc<ComponentManager>,
    pub metrics: Arc<RwLock<WalletMetrics>>,
    pub transactions: Arc<RwLock<Vec<Transaction>>>,
}

impl WalletCore {
    pub async fn new(config: WalletConfig) -> Result<Self> {
        info!("🔧 Initializing Wallet Core");
        
        // Initialize BPI integration
        let bpi_integration = Arc::new(BpiIntegration::new(&config).await?);
        
        // Initialize component manager with all 28+ BPI Core components
        let component_manager = Arc::new(ComponentManager::new(&config, bpi_integration.clone()).await?);
        
        // Initialize metrics collection
        let metrics = Arc::new(RwLock::new(WalletMetrics::new().await?));
        
        // Create initial wallet info
        let wallet_info = Arc::new(RwLock::new(WalletInfo {
            address: Self::generate_wallet_address(),
            network: config.network.clone(),
            balance: 0.0,
            status: WalletStatus::Locked,
            created_at: chrono::Utc::now(),
            last_activity: chrono::Utc::now(),
        }));
        
        let wallet_core = Self {
            config,
            wallet_info,
            bpi_integration,
            component_manager,
            metrics,
            transactions: Arc::new(RwLock::new(Vec::new())),
        };
        
        // Auto-start configured components
        wallet_core.auto_start_components().await?;
        
        info!("✅ Wallet Core initialized successfully");
        Ok(wallet_core)
    }
    
    pub async fn unlock(&self, password: &str) -> Result<()> {
        info!("🔓 Unlocking wallet");
        
        // TODO: Implement proper password verification
        // For now, accept any non-empty password
        if password.is_empty() {
            return Err(anyhow::anyhow!("Password cannot be empty"));
        }
        
        // Update wallet status
        {
            let mut wallet_info = self.wallet_info.write().await;
            wallet_info.status = WalletStatus::Unlocked;
            wallet_info.last_activity = chrono::Utc::now();
        }
        
        // Connect to BPI Core
        self.bpi_integration.connect().await?;
        
        // Update wallet status to connected
        {
            let mut wallet_info = self.wallet_info.write().await;
            wallet_info.status = WalletStatus::Connected;
        }
        
        // Refresh balance and transactions
        self.refresh_wallet_data().await?;
        
        info!("✅ Wallet unlocked and connected to BPI Core");
        Ok(())
    }
    
    pub async fn lock(&self) -> Result<()> {
        info!("🔒 Locking wallet");
        
        {
            let mut wallet_info = self.wallet_info.write().await;
            wallet_info.status = WalletStatus::Locked;
            wallet_info.last_activity = chrono::Utc::now();
        }
        
        // Disconnect from BPI Core
        self.bpi_integration.disconnect().await?;
        
        info!("✅ Wallet locked");
        Ok(())
    }
    
    pub async fn get_wallet_info(&self) -> WalletInfo {
        self.wallet_info.read().await.clone()
    }
    
    pub async fn get_balance(&self) -> Result<f64> {
        if let WalletStatus::Connected = self.wallet_info.read().await.status {
            self.bpi_integration.get_balance().await
        } else {
            Ok(0.0)
        }
    }
    
    pub async fn get_transactions(&self) -> Vec<Transaction> {
        self.transactions.read().await.clone()
    }
    
    pub async fn send_transaction(&self, to: &str, amount: f64) -> Result<String> {
        info!("💸 Sending transaction: {} BPI to {}", amount, to);
        
        // Verify wallet is unlocked and connected
        match self.wallet_info.read().await.status {
            WalletStatus::Connected => {},
            _ => return Err(anyhow::anyhow!("Wallet must be unlocked and connected")),
        }
        
        // Send transaction through BPI integration
        let tx_id = self.bpi_integration.send_transaction(to, amount).await?;
        
        // Add to local transaction history
        let transaction = Transaction {
            id: tx_id.clone(),
            from: self.wallet_info.read().await.address.clone(),
            to: to.to_string(),
            amount,
            fee: 0.001, // TODO: Calculate actual fee
            status: TransactionStatus::Pending,
            timestamp: chrono::Utc::now(),
            block_hash: None,
            confirmations: 0,
        };
        
        self.transactions.write().await.push(transaction);
        
        info!("✅ Transaction sent with ID: {}", tx_id);
        Ok(tx_id)
    }
    
    async fn auto_start_components(&self) -> Result<()> {
        info!("🚀 Auto-starting configured components");
        
        for component_name in &self.config.auto_start_components {
            match self.component_manager.start_component(component_name).await {
                Ok(_) => info!("✅ Started component: {}", component_name),
                Err(e) => warn!("⚠️ Failed to start component {}: {}", component_name, e),
            }
        }
        
        Ok(())
    }
    
    async fn refresh_wallet_data(&self) -> Result<()> {
        // Refresh balance
        if let Ok(balance) = self.get_balance().await {
            let mut wallet_info = self.wallet_info.write().await;
            wallet_info.balance = balance;
            wallet_info.last_activity = chrono::Utc::now();
        }
        
        // TODO: Refresh transaction history from BPI Core
        
        Ok(())
    }
    
    fn generate_wallet_address() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let random_bytes: [u8; 20] = rng.gen();
        use base64::{Engine as _, engine::general_purpose};
        format!("bpi_{}", general_purpose::STANDARD.encode(random_bytes))
    }
}
