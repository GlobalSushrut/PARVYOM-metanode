use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use base64::{Engine as _, engine::general_purpose};

/// Wallet creation request from email verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletCreationRequest {
    pub email: String,
    pub verification_token: String,
    pub provider_type: WalletProviderType,
    pub verification_level: VerificationLevel,
    pub timestamp: DateTime<Utc>,
}

/// Types of wallet providers supported
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WalletProviderType {
    Pravyom,
    MetaMail,
    Bank,
    Government,
    Custom(String),
}

/// Verification levels for wallets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerificationLevel {
    Unverified,
    Basic,
    Enhanced,
    Full,
    Government,
    Banking,
}

/// Generated wallet credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletCredentials {
    pub wallet_address: String,
    pub sync_address: String,
    pub signing_key: String, // Base64 encoded private key
    pub verifying_key: String, // Base64 encoded public key
    pub did_document: String,
    pub provider: WalletProviderType,
    pub verification_level: VerificationLevel,
    pub created_at: DateTime<Utc>,
}

/// Wallet creation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletCreationResult {
    pub success: bool,
    pub wallet_credentials: Option<WalletCredentials>,
    pub error_message: Option<String>,
    pub onboarding_token: Option<String>,
}

/// Wallet creation trigger service
pub struct WalletCreationTrigger {
    pending_requests: RwLock<HashMap<String, WalletCreationRequest>>,
    created_wallets: RwLock<HashMap<String, WalletCredentials>>,
    provider_configs: HashMap<WalletProviderType, ProviderConfig>,
}

/// Provider-specific configuration
#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub domain_suffix: String,
    pub capabilities: Vec<WalletCapability>,
    pub verification_requirements: VerificationLevel,
    pub smtp_bridge_enabled: bool,
}

/// Wallet capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WalletCapability {
    EncryptedMessaging,
    Payments,
    VideoCalls,
    DeviceAuthorization,
    SmartContracts,
    GovernmentServices,
    BankingServices,
    DecentralizedStorage,
}

impl WalletCreationTrigger {
    /// Create new wallet creation trigger service
    pub fn new() -> Self {
        let mut provider_configs = HashMap::new();
        
        // Configure Pravyom provider
        provider_configs.insert(
            WalletProviderType::Pravyom,
            ProviderConfig {
                domain_suffix: "pravyom.wallet".to_string(),
                capabilities: vec![
                    WalletCapability::EncryptedMessaging,
                    WalletCapability::Payments,
                    WalletCapability::VideoCalls,
                    WalletCapability::DeviceAuthorization,
                    WalletCapability::SmartContracts,
                    WalletCapability::DecentralizedStorage,
                ],
                verification_requirements: VerificationLevel::Basic,
                smtp_bridge_enabled: true,
            }
        );
        
        // Configure MetaMail provider
        provider_configs.insert(
            WalletProviderType::MetaMail,
            ProviderConfig {
                domain_suffix: "metamail.wallet".to_string(),
                capabilities: vec![
                    WalletCapability::EncryptedMessaging,
                    WalletCapability::Payments,
                    WalletCapability::DeviceAuthorization,
                ],
                verification_requirements: VerificationLevel::Enhanced,
                smtp_bridge_enabled: true,
            }
        );
        
        // Configure Bank provider
        provider_configs.insert(
            WalletProviderType::Bank,
            ProviderConfig {
                domain_suffix: "bank.wallet".to_string(),
                capabilities: vec![
                    WalletCapability::Payments,
                    WalletCapability::BankingServices,
                    WalletCapability::EncryptedMessaging,
                ],
                verification_requirements: VerificationLevel::Banking,
                smtp_bridge_enabled: false,
            }
        );
        
        // Configure Government provider
        provider_configs.insert(
            WalletProviderType::Government,
            ProviderConfig {
                domain_suffix: "gov.wallet".to_string(),
                capabilities: vec![
                    WalletCapability::GovernmentServices,
                    WalletCapability::EncryptedMessaging,
                    WalletCapability::DeviceAuthorization,
                ],
                verification_requirements: VerificationLevel::Government,
                smtp_bridge_enabled: false,
            }
        );
        
        Self {
            pending_requests: RwLock::new(HashMap::new()),
            created_wallets: RwLock::new(HashMap::new()),
            provider_configs,
        }
    }
    
    /// Queue wallet creation request from email verification
    pub async fn queue_wallet_creation(&self, request: WalletCreationRequest) -> Result<String> {
        let request_id = Uuid::new_v4().to_string();
        
        // Validate provider type
        if !self.provider_configs.contains_key(&request.provider_type) {
            return Err(anyhow!("Unsupported provider type: {:?}", request.provider_type));
        }
        
        // Store pending request
        let mut pending = self.pending_requests.write().await;
        pending.insert(request_id.clone(), request);
        
        println!("✅ Queued wallet creation request: {}", request_id);
        Ok(request_id)
    }
    
    /// Process wallet creation request
    pub async fn process_wallet_creation(&self, request_id: &str) -> Result<WalletCreationResult> {
        // Get pending request
        let request = {
            let mut pending = self.pending_requests.write().await;
            pending.remove(request_id)
                .ok_or_else(|| anyhow!("Wallet creation request not found: {}", request_id))?
        };
        
        // Validate verification token (in real implementation, verify with email service)
        if request.verification_token.is_empty() {
            return Ok(WalletCreationResult {
                success: false,
                wallet_credentials: None,
                error_message: Some("Invalid verification token".to_string()),
                onboarding_token: None,
            });
        }
        
        // Generate wallet credentials
        match self.generate_wallet_credentials(&request).await {
            Ok(credentials) => {
                // Store created wallet
                let mut created = self.created_wallets.write().await;
                created.insert(credentials.wallet_address.clone(), credentials.clone());
                
                // Generate onboarding token
                let onboarding_token = Uuid::new_v4().to_string();
                
                println!("✅ Created wallet: {} for email: {}", 
                    credentials.wallet_address, request.email);
                
                Ok(WalletCreationResult {
                    success: true,
                    wallet_credentials: Some(credentials),
                    error_message: None,
                    onboarding_token: Some(onboarding_token),
                })
            }
            Err(e) => {
                println!("❌ Failed to create wallet for {}: {}", request.email, e);
                Ok(WalletCreationResult {
                    success: false,
                    wallet_credentials: None,
                    error_message: Some(e.to_string()),
                    onboarding_token: None,
                })
            }
        }
    }
    
    /// Generate cryptographic wallet credentials
    async fn generate_wallet_credentials(&self, request: &WalletCreationRequest) -> Result<WalletCredentials> {
        // Generate Ed25519 keypair
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        
        // Get provider config
        let provider_config = self.provider_configs.get(&request.provider_type)
            .ok_or_else(|| anyhow!("Provider config not found"))?;
        
        // Extract username from email
        let username = request.email.split('@').next()
            .ok_or_else(|| anyhow!("Invalid email format"))?;
        
        // Generate wallet address in universal format
        let wallet_address = format!("{}@{}", username, provider_config.domain_suffix);
        
        // Generate sync address for on-chain operations
        let sync_address = format!("bpi_{}", hex::encode(&verifying_key.to_bytes()[..16]));
        
        // Generate DID document
        let did_document = self.generate_did_document(&wallet_address, &verifying_key).await?;
        
        // Encode keys as base64
        let signing_key_b64 = general_purpose::STANDARD.encode(signing_key.to_bytes());
        let verifying_key_b64 = general_purpose::STANDARD.encode(verifying_key.to_bytes());
        
        Ok(WalletCredentials {
            wallet_address,
            sync_address,
            signing_key: signing_key_b64,
            verifying_key: verifying_key_b64,
            did_document,
            provider: request.provider_type.clone(),
            verification_level: request.verification_level.clone(),
            created_at: Utc::now(),
        })
    }
    
    /// Generate DID document for wallet
    async fn generate_did_document(&self, wallet_address: &str, verifying_key: &VerifyingKey) -> Result<String> {
        let did_id = format!("did:bpi:{}", hex::encode(&verifying_key.to_bytes()[..16]));
        let public_key_hex = hex::encode(verifying_key.to_bytes());
        
        let did_doc = serde_json::json!({
            "@context": [
                "https://www.w3.org/ns/did/v1",
                "https://w3id.org/security/suites/ed25519-2020/v1"
            ],
            "id": did_id,
            "verificationMethod": [{
                "id": format!("{}#key-1", did_id),
                "type": "Ed25519VerificationKey2020",
                "controller": did_id,
                "publicKeyMultibase": format!("z{}", public_key_hex)
            }],
            "authentication": [format!("{}#key-1", did_id)],
            "assertionMethod": [format!("{}#key-1", did_id)],
            "service": [{
                "id": format!("{}#wallet", did_id),
                "type": "BpiWalletService",
                "serviceEndpoint": wallet_address
            }]
        });
        
        Ok(did_doc.to_string())
    }
    
    /// Get wallet by address
    pub async fn get_wallet(&self, wallet_address: &str) -> Option<WalletCredentials> {
        let wallets = self.created_wallets.read().await;
        wallets.get(wallet_address).cloned()
    }
    
    /// List all created wallets
    pub async fn list_wallets(&self) -> Vec<WalletCredentials> {
        let wallets = self.created_wallets.read().await;
        wallets.values().cloned().collect()
    }
    
    /// Get provider capabilities
    pub fn get_provider_capabilities(&self, provider: &WalletProviderType) -> Option<&Vec<WalletCapability>> {
        self.provider_configs.get(provider).map(|config| &config.capabilities)
    }
    
    /// Validate wallet address format
    pub fn validate_wallet_address(&self, address: &str) -> bool {
        // Check if address matches universal wallet format
        if let Some((username, domain)) = address.split_once('@') {
            // Validate username
            if username.is_empty() || username.len() > 64 {
                return false;
            }
            
            // Check if domain is supported
            return self.provider_configs.values()
                .any(|config| config.domain_suffix == domain);
        }
        false
    }
    
    /// Get pending requests count
    pub async fn get_pending_count(&self) -> usize {
        let pending = self.pending_requests.read().await;
        pending.len()
    }
    
    /// Get created wallets count
    pub async fn get_created_count(&self) -> usize {
        let wallets = self.created_wallets.read().await;
        wallets.len()
    }
}

impl Default for WalletCreationTrigger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_wallet_creation_trigger() {
        let trigger = WalletCreationTrigger::new();
        
        // Test wallet creation request
        let request = WalletCreationRequest {
            email: "alice@example.com".to_string(),
            verification_token: "test_token_123".to_string(),
            provider_type: WalletProviderType::Pravyom,
            verification_level: VerificationLevel::Basic,
            timestamp: Utc::now(),
        };
        
        // Queue wallet creation
        let request_id = trigger.queue_wallet_creation(request).await.unwrap();
        assert_eq!(trigger.get_pending_count().await, 1);
        
        // Process wallet creation
        let result = trigger.process_wallet_creation(&request_id).await.unwrap();
        assert!(result.success);
        assert!(result.wallet_credentials.is_some());
        assert!(result.onboarding_token.is_some());
        
        let credentials = result.wallet_credentials.unwrap();
        assert_eq!(credentials.wallet_address, "alice@pravyom.wallet");
        assert!(credentials.sync_address.starts_with("bpi_"));
        assert!(!credentials.signing_key.is_empty());
        assert!(!credentials.verifying_key.is_empty());
        assert!(!credentials.did_document.is_empty());
        
        // Verify wallet is stored
        assert_eq!(trigger.get_created_count().await, 1);
        assert_eq!(trigger.get_pending_count().await, 0);
        
        // Test wallet retrieval
        let retrieved = trigger.get_wallet(&credentials.wallet_address).await;
        assert!(retrieved.is_some());
    }
    
    #[tokio::test]
    async fn test_wallet_address_validation() {
        let trigger = WalletCreationTrigger::new();
        
        // Valid addresses
        assert!(trigger.validate_wallet_address("alice@pravyom.wallet"));
        assert!(trigger.validate_wallet_address("bob@metamail.wallet"));
        assert!(trigger.validate_wallet_address("gov@gov.wallet"));
        
        // Invalid addresses
        assert!(!trigger.validate_wallet_address("invalid"));
        assert!(!trigger.validate_wallet_address("@pravyom.wallet"));
        assert!(!trigger.validate_wallet_address("alice@unknown.wallet"));
        assert!(!trigger.validate_wallet_address("alice@"));
    }
    
    #[tokio::test]
    async fn test_provider_capabilities() {
        let trigger = WalletCreationTrigger::new();
        
        // Test Pravyom capabilities
        let pravyom_caps = trigger.get_provider_capabilities(&WalletProviderType::Pravyom);
        assert!(pravyom_caps.is_some());
        assert!(pravyom_caps.unwrap().contains(&WalletCapability::EncryptedMessaging));
        assert!(pravyom_caps.unwrap().contains(&WalletCapability::SmartContracts));
        
        // Test Bank capabilities
        let bank_caps = trigger.get_provider_capabilities(&WalletProviderType::Bank);
        assert!(bank_caps.is_some());
        assert!(bank_caps.unwrap().contains(&WalletCapability::BankingServices));
        assert!(!bank_caps.unwrap().contains(&WalletCapability::SmartContracts));
    }
}
