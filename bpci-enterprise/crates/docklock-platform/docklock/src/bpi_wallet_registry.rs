use std::collections::{HashMap, BTreeMap};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use uuid::Uuid;
use ed25519_dalek::{SigningKey, Signer, VerifyingKey, Verifier, Signature};

use bpi_enc::{CanonicalCbor, domain_hash};
use crate::error::{DockLockError, DockLockResult};
use crate::wallet::{CryptoKeypair, KeyType, WalletAddress, ServiceId};
use crate::metanode_wallet::{MetaNodeWallet, IdentityId, VerificationLevel};
use crate::dao_wallet::DaoWallet;

/// Domain separation constants for BPI wallet registry
const BPI_WALLET_REGISTRY_HASH: u8 = 0x21;
const WALLET_COMMUNICATION_HASH: u8 = 0x22;
const BPCI_WALLET_MESSAGE_HASH: &str = "BPCI_WALLET_MESSAGE";

/// BPI Wallet Registry - Enterprise-grade military wallet communication system
#[derive(Debug)]
pub struct BpiWalletRegistry {
    pub id: Uuid,
    pub name: String,
    wallets: Arc<RwLock<HashMap<Uuid, RegisteredWallet>>>,
    service_wallets: Arc<RwLock<HashMap<ServiceId, Uuid>>>,
    bpci_channels: Arc<RwLock<HashMap<String, BpciChannel>>>,
    bci_channels: Arc<RwLock<HashMap<String, BciChannel>>>,
    wallet_index: Arc<RwLock<WalletIndex>>,
    message_storage: Arc<RwLock<MessageStorage>>,
    config: BpiWalletRegistryConfig,
    stats: Arc<RwLock<RegistryStats>>,
    signing_key: SigningKey,
}

/// Registered wallet in the BPI registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredWallet {
    pub id: Uuid,
    pub wallet_type: WalletType,
    pub address: WalletAddress,
    pub service_id: Option<ServiceId>,
    pub verification_level: VerificationLevel,
    pub public_key: Vec<u8>,
    pub key_type: KeyType,
    pub bpci_endpoint: Option<String>,
    pub bci_endpoint: Option<String>,
    pub capabilities: WalletCapabilities,
    pub registered_at: u64,
    pub last_activity: u64,
    pub status: WalletStatus,
    pub metadata: HashMap<String, String>,
    pub signature: Option<String>,
}

/// Wallet types supported in the registry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WalletType {
    DockLock,
    Dao,
    MetaNode,
    BpciService,
    BciBlockchain,
    Enterprise,
    Military,
}

/// Wallet capabilities for communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletCapabilities {
    pub bpci_messaging: bool,
    pub bpci_receiving: bool,
    pub bci_transactions: bool,
    pub bci_receiving: bool,
    pub encryption: bool,
    pub multisig: bool,
    pub governance: bool,
    pub policy_enforcement: bool,
    pub max_message_size: usize,
    pub encryption_schemes: Vec<String>,
}

/// Wallet status in the registry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WalletStatus {
    Active,
    Inactive,
    Suspended,
    Revoked,
    UnderReview,
}

/// BPCI communication channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpciChannel {
    pub id: Uuid,
    pub name: String,
    pub mesh_url: String,
    pub service_name: String,
    pub connected_wallets: Vec<Uuid>,
    pub status: ChannelStatus,
    pub message_stats: MessageStats,
    pub created_at: u64,
}

/// BCI communication channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BciChannel {
    pub id: Uuid,
    pub name: String,
    pub network_endpoint: String,
    pub chain_id: String,
    pub connected_wallets: Vec<Uuid>,
    pub status: ChannelStatus,
    pub transaction_stats: TransactionStats,
    pub created_at: u64,
}

/// Communication channel status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChannelStatus {
    Active,
    Inactive,
    Maintenance,
    Error,
}

/// Message statistics for BPCI channels
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessageStats {
    pub total_sent: u64,
    pub total_received: u64,
    pub total_failed: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub last_message_at: Option<u64>,
}

/// Transaction statistics for BCI channels
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionStats {
    pub total_sent: u64,
    pub total_received: u64,
    pub total_confirmed: u64,
    pub total_failed: u64,
    pub gas_used: u64,
    pub last_transaction_at: Option<u64>,
}

/// Wallet indexing system for fast lookups
#[derive(Debug, Default)]
pub struct WalletIndex {
    by_address: HashMap<WalletAddress, Uuid>,
    by_service_id: HashMap<ServiceId, Uuid>,
    by_type: HashMap<WalletType, Vec<Uuid>>,
    by_verification_level: HashMap<VerificationLevel, Vec<Uuid>>,
    by_status: HashMap<WalletStatus, Vec<Uuid>>,
    by_bpci_endpoint: HashMap<String, Vec<Uuid>>,
    by_bci_endpoint: HashMap<String, Vec<Uuid>>,
}

/// Message storage for wallet communications
#[derive(Debug, Default)]
pub struct MessageStorage {
    bpci_messages: HashMap<Uuid, BpciMessage>,
    bci_transactions: HashMap<Uuid, BciTransaction>,
    by_sender: HashMap<Uuid, Vec<Uuid>>,
    by_receiver: HashMap<Uuid, Vec<Uuid>>,
    by_timestamp: BTreeMap<u64, Vec<Uuid>>,
}

/// BPCI message for wallet communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpciMessage {
    pub id: Uuid,
    pub sender_wallet_id: Uuid,
    pub receiver_wallet_id: Uuid,
    pub message_type: MessageType,
    pub payload: Vec<u8>,
    pub metadata: HashMap<String, String>,
    pub timestamp: u64,
    pub status: MessageStatus,
    pub signature: String,
    pub delivery_confirmation: Option<DeliveryConfirmation>,
}

/// BCI transaction for blockchain communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BciTransaction {
    pub id: Uuid,
    pub sender_wallet_id: Uuid,
    pub receiver_wallet_id: Option<Uuid>,
    pub transaction_type: TransactionType,
    pub data: Vec<u8>,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub nonce: u64,
    pub timestamp: u64,
    pub status: TransactionStatus,
    pub block_hash: Option<String>,
    pub block_number: Option<u64>,
    pub transaction_hash: String,
    pub signature: String,
}

/// Message types for BPCI communication
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageType {
    DirectMessage,
    ServiceDiscovery,
    PolicyEnforcement,
    GovernanceProposal,
    IdentityVerification,
    ComplianceReport,
    EmergencyAlert,
    SystemNotification,
}

/// Transaction types for BCI communication
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionType {
    Transfer,
    ContractDeployment,
    ContractCall,
    GovernanceVote,
    Staking,
    IdentityRegistration,
    PolicyDeployment,
}

/// Message delivery status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
    Expired,
}

/// Transaction status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionStatus {
    Pending,
    Submitted,
    Confirmed,
    Failed,
    Reverted,
}

/// Delivery confirmation for messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryConfirmation {
    pub confirmed_at: u64,
    pub receiver_signature: String,
    pub receipt: String,
}

/// BPI wallet registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpiWalletRegistryConfig {
    pub max_wallets: usize,
    pub max_message_size: usize,
    pub message_retention_seconds: u64,
    pub transaction_retention_seconds: u64,
    pub default_encryption: bool,
    pub require_verification: bool,
    pub max_bpci_channels: usize,
    pub max_bci_channels: usize,
    pub require_registry_signature: bool,
}

/// Registry statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegistryStats {
    pub total_wallets: u64,
    pub active_wallets: u64,
    pub total_bpci_messages: u64,
    pub total_bci_transactions: u64,
    pub total_bpci_channels: u64,
    pub total_bci_channels: u64,
    pub uptime_seconds: u64,
    pub last_activity: u64,
}

impl Default for BpiWalletRegistryConfig {
    fn default() -> Self {
        Self {
            max_wallets: 10000,
            max_message_size: 1024 * 1024, // 1MB
            message_retention_seconds: 30 * 24 * 60 * 60, // 30 days
            transaction_retention_seconds: 90 * 24 * 60 * 60, // 90 days
            default_encryption: true,
            require_verification: true,
            max_bpci_channels: 100,
            max_bci_channels: 50,
            require_registry_signature: true,
        }
    }
}

impl Default for WalletCapabilities {
    fn default() -> Self {
        Self {
            bpci_messaging: true,
            bpci_receiving: true,
            bci_transactions: true,
            bci_receiving: true,
            encryption: true,
            multisig: false,
            governance: false,
            policy_enforcement: false,
            max_message_size: 64 * 1024, // 64KB
            encryption_schemes: vec!["ChaCha20-Poly1305".to_string(), "AES-GCM".to_string()],
        }
    }
}

impl BpiWalletRegistry {
    /// Create a new BPI wallet registry
    pub fn new(name: String, config: BpiWalletRegistryConfig) -> Self {
        let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
        let id = Uuid::new_v4();
        
        info!("Creating BPI wallet registry: {} ({})", name, id);
        
        Self {
            id,
            name,
            wallets: Arc::new(RwLock::new(HashMap::new())),
            service_wallets: Arc::new(RwLock::new(HashMap::new())),
            bpci_channels: Arc::new(RwLock::new(HashMap::new())),
            bci_channels: Arc::new(RwLock::new(HashMap::new())),
            wallet_index: Arc::new(RwLock::new(WalletIndex::default())),
            message_storage: Arc::new(RwLock::new(MessageStorage::default())),
            config,
            stats: Arc::new(RwLock::new(RegistryStats::default())),
            signing_key,
        }
    }

    /// Register a new wallet in the registry
    pub async fn register_wallet(&self, wallet: RegisteredWallet) -> DockLockResult<()> {
        let mut wallets = self.wallets.write().await;
        let mut index = self.wallet_index.write().await;
        let mut stats = self.stats.write().await;

        // Check if wallet already exists
        if wallets.contains_key(&wallet.id) {
            return Err(DockLockError::AlreadyExists(format!("Wallet {} already registered", wallet.id)));
        }

        // Check registry capacity
        if wallets.len() >= self.config.max_wallets {
            return Err(DockLockError::CapacityExceeded("Registry at maximum wallet capacity".to_string()));
        }

        // Update indexes
        index.by_address.insert(wallet.address.clone(), wallet.id);
        if let Some(service_id) = wallet.service_id {
            index.by_service_id.insert(service_id, wallet.id);
        }
        index.by_type.entry(wallet.wallet_type.clone()).or_insert_with(Vec::new).push(wallet.id);
        index.by_verification_level.entry(wallet.verification_level.clone()).or_insert_with(Vec::new).push(wallet.id);
        index.by_status.entry(wallet.status.clone()).or_insert_with(Vec::new).push(wallet.id);

        // Store wallet ID and status before moving
        let wallet_id = wallet.id;
        let is_active = wallet.status == WalletStatus::Active;
        
        // Store wallet
        wallets.insert(wallet_id, wallet);

        // Update statistics
        stats.total_wallets += 1;
        if is_active {
            stats.active_wallets += 1;
        }
        stats.last_activity = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        info!("Registered wallet {} in BPI registry", wallet_id);
        Ok(())
    }

    /// Get wallet by ID
    pub async fn get_wallet(&self, wallet_id: &Uuid) -> DockLockResult<RegisteredWallet> {
        let wallets = self.wallets.read().await;
        wallets.get(wallet_id)
            .cloned()
            .ok_or_else(|| DockLockError::NotFound(format!("Wallet {} not found", wallet_id)))
    }

    /// Find wallet by address
    pub async fn find_wallet_by_address(&self, address: &WalletAddress) -> DockLockResult<RegisteredWallet> {
        let index = self.wallet_index.read().await;
        let wallet_id = *index.by_address.get(address)
            .ok_or_else(|| DockLockError::NotFound(format!("Wallet with address {} not found", address)))?;
        
        drop(index);
        self.get_wallet(&wallet_id).await
    }

    /// Create BPCI communication channel
    pub async fn create_bpci_channel(&self, name: String, mesh_url: String, service_name: String) -> DockLockResult<Uuid> {
        let mut channels = self.bpci_channels.write().await;
        let mut stats = self.stats.write().await;

        let channel = BpciChannel {
            id: Uuid::new_v4(),
            name,
            mesh_url,
            service_name,
            connected_wallets: Vec::new(),
            status: ChannelStatus::Active,
            message_stats: MessageStats::default(),
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };

        let channel_id = channel.id;
        channels.insert(channel.name.clone(), channel);
        stats.total_bpci_channels += 1;

        info!("Created BPCI channel: {}", channel_id);
        Ok(channel_id)
    }

    /// Send BPCI message between wallets
    pub async fn send_bpci_message(&self, sender_wallet_id: Uuid, receiver_wallet_id: Uuid, 
                                   message_type: MessageType, payload: Vec<u8>, 
                                   metadata: HashMap<String, String>) -> DockLockResult<Uuid> {
        let mut storage = self.message_storage.write().await;
        let mut stats = self.stats.write().await;

        // Verify sender wallet exists and is active
        let sender_wallet = self.get_wallet(&sender_wallet_id).await?;
        if sender_wallet.status != WalletStatus::Active {
            return Err(DockLockError::InvalidState(format!("Sender wallet {} is not active", sender_wallet_id)));
        }

        // Create message
        let message = BpciMessage {
            id: Uuid::new_v4(),
            sender_wallet_id,
            receiver_wallet_id,
            message_type,
            payload,
            metadata,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            status: MessageStatus::Pending,
            signature: self.sign_message_data(&sender_wallet_id, &receiver_wallet_id)?,
            delivery_confirmation: None,
        };

        let message_id = message.id;
        storage.bpci_messages.insert(message_id, message);
        stats.total_bpci_messages += 1;

        info!("Sent BPCI message {} from wallet {} to wallet {}", message_id, sender_wallet_id, receiver_wallet_id);
        Ok(message_id)
    }

    /// Get registry statistics
    pub async fn get_stats(&self) -> RegistryStats {
        self.stats.read().await.clone()
    }

    /// Sign message data for integrity
    fn sign_message_data(&self, sender_id: &Uuid, receiver_id: &Uuid) -> DockLockResult<String> {
        let data = format!("{}{}{}", sender_id, receiver_id, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos());
        let hash = domain_hash(BPCI_WALLET_MESSAGE_HASH, data.as_bytes());
        let signature = self.signing_key.sign(&hash);
        Ok(hex::encode(signature.to_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bpi_wallet_registry_creation() {
        let config = BpiWalletRegistryConfig::default();
        let registry = BpiWalletRegistry::new("test_registry".to_string(), config);
        
        assert_eq!(registry.name, "test_registry");
        let stats = registry.get_stats().await;
        assert_eq!(stats.total_wallets, 0);
    }

    #[tokio::test]
    async fn test_wallet_registration() {
        let config = BpiWalletRegistryConfig::default();
        let registry = BpiWalletRegistry::new("test_registry".to_string(), config);
        
        let wallet = RegisteredWallet {
            id: Uuid::new_v4(),
            wallet_type: WalletType::DockLock,
            address: "test_address".to_string(),
            service_id: None,
            verification_level: VerificationLevel::Basic,
            public_key: vec![0u8; 32],
            key_type: KeyType::Ed25519,
            bpci_endpoint: Some("http://localhost:8080".to_string()),
            bci_endpoint: None,
            capabilities: WalletCapabilities::default(),
            registered_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            last_activity: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            status: WalletStatus::Active,
            metadata: HashMap::new(),
            signature: None,
        };

        let wallet_id = wallet.id;
        registry.register_wallet(wallet).await.unwrap();
        
        let retrieved_wallet = registry.get_wallet(&wallet_id).await.unwrap();
        assert_eq!(retrieved_wallet.id, wallet_id);
        assert_eq!(retrieved_wallet.wallet_type, WalletType::DockLock);
    }

    #[tokio::test]
    async fn test_bpci_channel_creation() {
        let config = BpiWalletRegistryConfig::default();
        let registry = BpiWalletRegistry::new("test_registry".to_string(), config);
        
        let channel_id = registry.create_bpci_channel(
            "test_channel".to_string(),
            "http://localhost:8080".to_string(),
            "test_service".to_string()
        ).await.unwrap();
        
        assert!(channel_id != Uuid::nil());
        let stats = registry.get_stats().await;
        assert_eq!(stats.total_bpci_channels, 1);
    }

    #[tokio::test]
    async fn test_bpci_message_sending() {
        let config = BpiWalletRegistryConfig::default();
        let registry = BpiWalletRegistry::new("test_registry".to_string(), config);
        
        // Register sender wallet
        let sender_wallet = RegisteredWallet {
            id: Uuid::new_v4(),
            wallet_type: WalletType::DockLock,
            address: "sender_address".to_string(),
            service_id: None,
            verification_level: VerificationLevel::Basic,
            public_key: vec![0u8; 32],
            key_type: KeyType::Ed25519,
            bpci_endpoint: Some("http://localhost:8080".to_string()),
            bci_endpoint: None,
            capabilities: WalletCapabilities::default(),
            registered_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            last_activity: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            status: WalletStatus::Active,
            metadata: HashMap::new(),
            signature: None,
        };

        // Register receiver wallet
        let receiver_wallet = RegisteredWallet {
            id: Uuid::new_v4(),
            wallet_type: WalletType::MetaNode,
            address: "receiver_address".to_string(),
            service_id: None,
            verification_level: VerificationLevel::Enhanced,
            public_key: vec![1u8; 32],
            key_type: KeyType::Ed25519,
            bpci_endpoint: Some("http://localhost:8081".to_string()),
            bci_endpoint: None,
            capabilities: WalletCapabilities::default(),
            registered_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            last_activity: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            status: WalletStatus::Active,
            metadata: HashMap::new(),
            signature: None,
        };

        let sender_id = sender_wallet.id;
        let receiver_id = receiver_wallet.id;

        registry.register_wallet(sender_wallet).await.unwrap();
        registry.register_wallet(receiver_wallet).await.unwrap();

        // Send message
        let message_id = registry.send_bpci_message(
            sender_id,
            receiver_id,
            MessageType::DirectMessage,
            b"Hello, world!".to_vec(),
            HashMap::new()
        ).await.unwrap();

        assert!(message_id != Uuid::nil());
        let stats = registry.get_stats().await;
        assert_eq!(stats.total_bpci_messages, 1);
    }
}
