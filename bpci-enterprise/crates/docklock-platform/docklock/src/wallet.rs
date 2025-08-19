use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{debug, info};
use uuid::Uuid;
use blake3::Hash;

use ed25519_dalek::{SigningKey, Signer};
use secp256k1::{Secp256k1, SecretKey as Secp256k1SecretKey, PublicKey as Secp256k1PublicKey};

use bpi_enc::CanonicalCbor;
use crate::error::{DockLockError, DockLockResult};
use crate::event_stream::{Event, EventKind, CanonicalEventStream};

/// Microservice identity type
pub type ServiceId = Uuid;

/// Wallet address type
pub type WalletAddress = String;

/// Cryptographic key types supported
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyType {
    Ed25519,
    Secp256k1,
}

/// Cryptographic keypair for identity
#[derive(Debug, Clone)]
pub enum CryptoKeypair {
    Ed25519(SigningKey),
    Secp256k1 {
        secret: Secp256k1SecretKey,
        public: Secp256k1PublicKey,
    },
}

impl CryptoKeypair {
    /// Generate a new keypair of the specified type
    pub fn generate(key_type: KeyType) -> DockLockResult<Self> {
        match key_type {
            KeyType::Ed25519 => {
                let mut csprng = rand::rngs::OsRng;
                let signing_key = SigningKey::generate(&mut csprng);
                Ok(CryptoKeypair::Ed25519(signing_key))
            }
            KeyType::Secp256k1 => {
                use secp256k1::rand::rngs::OsRng;
                let secp = Secp256k1::new();
                let (secret, public) = secp.generate_keypair(&mut OsRng);
                Ok(CryptoKeypair::Secp256k1 { secret, public })
            }
        }
    }
    
    /// Get the public key as bytes
    pub fn public_key_bytes(&self) -> Vec<u8> {
        match self {
            CryptoKeypair::Ed25519(signing_key) => signing_key.verifying_key().to_bytes().to_vec(),
            CryptoKeypair::Secp256k1 { public, .. } => public.serialize().to_vec(),
        }
    }
    
    /// Sign data with this keypair
    pub fn sign(&self, data: &[u8]) -> DockLockResult<Vec<u8>> {
        match self {
            CryptoKeypair::Ed25519(signing_key) => {
                let signature = signing_key.sign(data);
                Ok(signature.to_bytes().to_vec())
            }
            CryptoKeypair::Secp256k1 { secret, .. } => {
                let secp = Secp256k1::new();
                let hash = blake3::hash(data);
                let message = secp256k1::Message::from_digest_slice(&hash.as_bytes()[..32])
                    .map_err(|e| DockLockError::CryptoError(format!("Invalid message: {}", e)))?;
                let signature = secp.sign_ecdsa(&message, secret);
                Ok(signature.serialize_compact().to_vec())
            }
        }
    }
    
    /// Get the key type
    pub fn key_type(&self) -> KeyType {
        match self {
            CryptoKeypair::Ed25519(_) => KeyType::Ed25519,
            CryptoKeypair::Secp256k1 { .. } => KeyType::Secp256k1,
        }
    }
}

/// Microservice identity with cryptographic credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroserviceIdentity {
    /// Unique service identifier
    pub service_id: ServiceId,
    
    /// Human-readable service name
    pub name: String,
    
    /// Service description
    pub description: String,
    
    /// Container image reference
    pub image: String,
    
    /// Service version
    pub version: String,
    
    /// Public key for verification
    pub public_key: Vec<u8>,
    
    /// Key type used
    pub key_type: KeyType,
    
    /// Service metadata
    pub metadata: HashMap<String, String>,
    
    /// Creation timestamp
    pub created_at: u64,
    
    /// Last updated timestamp
    pub updated_at: u64,
}

impl MicroserviceIdentity {
    /// Create a new microservice identity
    pub fn new(
        name: String,
        description: String,
        image: String,
        version: String,
        keypair: &CryptoKeypair,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            service_id: Uuid::new_v4(),
            name,
            description,
            image,
            version,
            public_key: keypair.public_key_bytes(),
            key_type: keypair.key_type(),
            metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Get the wallet address for this identity
    pub fn wallet_address(&self) -> WalletAddress {
        let hash = blake3::hash(&self.public_key);
        format!("0x{}", hex::encode(&hash.as_bytes()[..20]))
    }
    
    /// Add metadata to the identity
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Get canonical encoding
    pub fn encode(&self) -> DockLockResult<Vec<u8>> {
        CanonicalCbor::encode(self)
            .map_err(|e| DockLockError::EncodingError(format!("Failed to encode identity: {}", e)))
    }
    
    /// Get identity hash
    pub fn hash(&self) -> DockLockResult<Hash> {
        let encoded = self.encode()?;
        Ok(blake3::hash(&encoded))
    }
}

/// OCI (Open Container Initiative) operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OciOperation {
    /// Container operations
    ContainerCreate { image: String, config: HashMap<String, String> },
    ContainerStart { container_id: String },
    ContainerStop { container_id: String },
    ContainerRemove { container_id: String },
    
    /// Image operations
    ImagePull { image: String, tag: String },
    ImagePush { image: String, tag: String },
    ImageRemove { image: String },
    
    /// Network operations
    NetworkCreate { name: String, config: HashMap<String, String> },
    NetworkConnect { network: String, container: String },
    NetworkDisconnect { network: String, container: String },
    
    /// Volume operations
    VolumeCreate { name: String, config: HashMap<String, String> },
    VolumeMount { volume: String, container: String, mount_point: String },
    VolumeUnmount { volume: String, container: String },
    
    /// Kubernetes operations
    PodCreate { namespace: String, spec: HashMap<String, String> },
    ServiceCreate { namespace: String, spec: HashMap<String, String> },
    DeploymentCreate { namespace: String, spec: HashMap<String, String> },
    
    /// Custom operations
    Custom { operation: String, params: HashMap<String, String> },
}

/// DockLock wallet for microservice identity and OCI operations
pub struct DockLockWallet {
    /// Wallet configuration
    config: DockLockWalletConfig,
    
    /// Cryptographic keypair
    keypair: CryptoKeypair,
    
    /// Microservice identity
    identity: MicroserviceIdentity,
    
    /// Registered services
    services: Arc<RwLock<HashMap<ServiceId, MicroserviceIdentity>>>,
    
    /// Event stream for operations
    event_stream: Arc<RwLock<CanonicalEventStream>>,
    
    /// OCI operation history
    operation_history: Arc<RwLock<Vec<OciOperation>>>,
}

/// DockLock wallet configuration
#[derive(Debug, Clone)]
pub struct DockLockWalletConfig {
    /// Wallet name
    pub name: String,
    
    /// Key type to use
    pub key_type: KeyType,
    
    /// Enable operation logging
    pub enable_logging: bool,
    
    /// Maximum operations to keep in history
    pub max_operation_history: usize,
}

impl Default for DockLockWalletConfig {
    fn default() -> Self {
        Self {
            name: "DockLock Wallet".to_string(),
            key_type: KeyType::Ed25519,
            enable_logging: true,
            max_operation_history: 10000,
        }
    }
}

impl DockLockWallet {
    /// Create a new DockLock wallet
    pub async fn new(config: DockLockWalletConfig) -> DockLockResult<Self> {
        let keypair = CryptoKeypair::generate(config.key_type.clone())?;
        
        let identity = MicroserviceIdentity::new(
            config.name.clone(),
            "DockLock wallet for microservice identity management".to_string(),
            "docklock/wallet:latest".to_string(),
            "1.0.0".to_string(),
            &keypair,
        );
        
        info!("Created DockLock wallet with address: {}", identity.wallet_address());
        
        Ok(Self {
            config,
            keypair,
            identity,
            services: Arc::new(RwLock::new(HashMap::new())),
            event_stream: Arc::new(RwLock::new(CanonicalEventStream::default())),
            operation_history: Arc::new(RwLock::new(Vec::new())),
        })
    }
    
    /// Get wallet address
    pub fn address(&self) -> WalletAddress {
        self.identity.wallet_address()
    }
    
    /// Get wallet identity
    pub fn identity(&self) -> &MicroserviceIdentity {
        &self.identity
    }
    
    /// Register a new microservice
    pub async fn register_service(
        &self,
        name: String,
        description: String,
        image: String,
        version: String,
    ) -> DockLockResult<ServiceId> {
        let service_keypair = CryptoKeypair::generate(self.config.key_type.clone())?;
        let service_identity = MicroserviceIdentity::new(
            name,
            description,
            image,
            version,
            &service_keypair,
        );
        
        let service_id = service_identity.service_id;
        
        // Add to services registry
        {
            let mut services = self.services.write().await;
            services.insert(service_id, service_identity.clone());
        }
        
        // Log identity creation event
        if self.config.enable_logging {
            let event = Event::new(
                service_id.as_u128(),
                None,
                0,
                EventKind::IdentityCreate,
                &service_identity.encode()?,
            ).with_metadata("service_name".to_string(), service_identity.name.clone())
             .with_metadata("wallet_address".to_string(), self.address());
            
            let mut stream = self.event_stream.write().await;
            stream.add_event(event)?;
        }
        
        info!("Registered microservice {} with ID {}", service_identity.name, service_id);
        Ok(service_id)
    }
    
    /// Get a registered service
    pub async fn get_service(&self, service_id: &ServiceId) -> Option<MicroserviceIdentity> {
        let services = self.services.read().await;
        services.get(service_id).cloned()
    }
    
    /// List all registered services
    pub async fn list_services(&self) -> Vec<MicroserviceIdentity> {
        let services = self.services.read().await;
        services.values().cloned().collect()
    }
    
    /// Execute an OCI operation
    pub async fn execute_oci_operation(&self, operation: OciOperation) -> DockLockResult<String> {
        debug!("Executing OCI operation: {:?}", operation);
        
        // Sign the operation
        let operation_data = CanonicalCbor::encode(&operation)
            .map_err(|e| DockLockError::EncodingError(format!("Failed to encode operation: {}", e)))?;
        
        let signature = self.keypair.sign(&operation_data)?;
        
        // Log the operation
        if self.config.enable_logging {
            let event = Event::new(
                rand::random::<u128>(),
                None,
                0,
                match &operation {
                    OciOperation::ContainerCreate { .. } => EventKind::ContainerStart,
                    OciOperation::ContainerStop { .. } => EventKind::ContainerStop,
                    OciOperation::PodCreate { .. } => EventKind::ServiceDeploy,
                    _ => EventKind::Custom("oci_operation".to_string()),
                },
                &operation_data,
            ).with_metadata("operation_type".to_string(), format!("{:?}", operation))
             .with_metadata("wallet_address".to_string(), self.address())
             .with_metadata("signature".to_string(), hex::encode(&signature));
            
            let mut stream = self.event_stream.write().await;
            stream.add_event(event)?;
        }
        
        // Add to operation history
        {
            let mut history = self.operation_history.write().await;
            history.push(operation.clone());
            
            // Cleanup old operations
            if history.len() > self.config.max_operation_history {
                history.remove(0);
            }
        }
        
        // In a real implementation, this would execute the actual OCI operation
        // For now, we return a mock operation ID
        let operation_id = format!("op_{}", Uuid::new_v4());
        info!("Executed OCI operation with ID: {}", operation_id);
        
        Ok(operation_id)
    }
    
    /// Get operation history
    pub async fn get_operation_history(&self) -> Vec<OciOperation> {
        let history = self.operation_history.read().await;
        history.clone()
    }
    
    /// Sign arbitrary data
    pub fn sign_data(&self, data: &[u8]) -> DockLockResult<Vec<u8>> {
        self.keypair.sign(data)
    }
    
    /// Get current event stream stats
    pub async fn get_event_stats(&self) -> crate::event_stream::EventStreamStats {
        let stream = self.event_stream.read().await;
        stream.stats()
    }
    
    /// Get wallet statistics
    pub async fn get_wallet_stats(&self) -> DockLockWalletStats {
        let services = self.services.read().await;
        let history = self.operation_history.read().await;
        let stream = self.event_stream.read().await;
        
        DockLockWalletStats {
            wallet_address: self.address(),
            total_services: services.len(),
            total_operations: history.len(),
            total_events: stream.event_count(),
            current_sequence: stream.current_sequence(),
        }
    }
}

/// DockLock wallet statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockLockWalletStats {
    pub wallet_address: WalletAddress,
    pub total_services: usize,
    pub total_operations: usize,
    pub total_events: usize,
    pub current_sequence: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    
    #[tokio::test]
    async fn test_docklock_wallet_creation() {
        let config = DockLockWalletConfig::default();
        let wallet = DockLockWallet::new(config).await.unwrap();
        
        assert!(!wallet.address().is_empty());
        assert_eq!(wallet.identity().name, "DockLock Wallet");
    }
    
    #[tokio::test]
    async fn test_service_registration() {
        let config = DockLockWalletConfig::default();
        let wallet = DockLockWallet::new(config).await.unwrap();
        
        let service_id = wallet.register_service(
            "test-service".to_string(),
            "A test microservice".to_string(),
            "nginx:latest".to_string(),
            "1.0.0".to_string(),
        ).await.unwrap();
        
        let service = wallet.get_service(&service_id).await.unwrap();
        assert_eq!(service.name, "test-service");
        assert_eq!(service.image, "nginx:latest");
        
        let services = wallet.list_services().await;
        assert_eq!(services.len(), 1);
    }
    
    #[tokio::test]
    async fn test_oci_operations() {
        let config = DockLockWalletConfig::default();
        let wallet = DockLockWallet::new(config).await.unwrap();
        
        let operation = OciOperation::ContainerCreate {
            image: "nginx:latest".to_string(),
            config: HashMap::new(),
        };
        
        let op_id = wallet.execute_oci_operation(operation).await.unwrap();
        assert!(op_id.starts_with("op_"));
        
        let history = wallet.get_operation_history().await;
        assert_eq!(history.len(), 1);
    }
    
    #[tokio::test]
    async fn test_wallet_stats() {
        let config = DockLockWalletConfig::default();
        let wallet = DockLockWallet::new(config).await.unwrap();
        
        // Register a service
        wallet.register_service(
            "test-service".to_string(),
            "A test microservice".to_string(),
            "nginx:latest".to_string(),
            "1.0.0".to_string(),
        ).await.unwrap();
        
        // Execute an operation
        let operation = OciOperation::ContainerCreate {
            image: "nginx:latest".to_string(),
            config: HashMap::new(),
        };
        wallet.execute_oci_operation(operation).await.unwrap();
        
        let stats = wallet.get_wallet_stats().await;
        assert_eq!(stats.total_services, 1);
        assert_eq!(stats.total_operations, 1);
        assert!(stats.total_events > 0);
    }
    
    #[test]
    fn test_microservice_identity() {
        let keypair = CryptoKeypair::generate(KeyType::Ed25519).unwrap();
        let identity = MicroserviceIdentity::new(
            "test-service".to_string(),
            "A test service".to_string(),
            "nginx:latest".to_string(),
            "1.0.0".to_string(),
            &keypair,
        );
        
        assert_eq!(identity.name, "test-service");
        assert!(!identity.wallet_address().is_empty());
        assert!(identity.wallet_address().starts_with("0x"));
    }
    
    #[test]
    fn test_crypto_keypair_generation() {
        let ed25519_keypair = CryptoKeypair::generate(KeyType::Ed25519).unwrap();
        let secp256k1_keypair = CryptoKeypair::generate(KeyType::Secp256k1).unwrap();
        
        assert!(matches!(ed25519_keypair.key_type(), KeyType::Ed25519));
        assert!(matches!(secp256k1_keypair.key_type(), KeyType::Secp256k1));
        
        let data = b"test data";
        let ed25519_sig = ed25519_keypair.sign(data).unwrap();
        let secp256k1_sig = secp256k1_keypair.sign(data).unwrap();
        
        assert!(!ed25519_sig.is_empty());
        assert!(!secp256k1_sig.is_empty());
    }
}
