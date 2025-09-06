use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use sha2::{Sha256, Digest};
use anyhow::{Result, anyhow};
use uuid::Uuid;

// Custom serialization for Keypair
mod keypair_serde {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    
    pub fn serialize<S>(keypair: &Keypair, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        keypair.to_bytes().serialize(serializer)
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Keypair, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
        let bytes: [u8; 64] = bytes.try_into().map_err(|_| serde::de::Error::custom("Invalid keypair length"))?;
        Keypair::from_bytes(&bytes).map_err(serde::de::Error::custom)
    }
}

/// Universal Wallet Identity System
/// Format: user@provider.wallet<sync_address>{smtp_email, auth_token}
/// 
/// This system provides email-like wallet addresses that serve as universal internet IDs,
/// enabling encrypted messaging, payments, video calls, and device authorization.

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletIdentity {
    /// Human-readable wallet identity (e.g., "alice@pravyom.wallet")
    pub wallet_address: String,
    /// BPI wallet address for on-chain operations
    pub sync_address: String,
    /// Legacy email bridge for compatibility
    pub smtp_email: Option<String>,
    /// Encrypted authentication token
    pub auth_token: String,
    /// Wallet provider (pravyom, metamail, bank, government)
    pub provider: WalletProvider,
    /// Ed25519 keypair for signing
    #[serde(with = "keypair_serde")]
    pub keypair: Keypair,
    /// Public key derived from keypair
    pub public_key: PublicKey,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last activity timestamp
    pub last_active: DateTime<Utc>,
    /// Wallet capabilities
    pub capabilities: Vec<WalletCapability>,
    /// Identity verification level
    pub verification_level: VerificationLevel,
    /// Associated DID (Decentralized Identifier)
    pub did: Option<String>,
    /// Wallet metadata
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletProvider {
    Pravyom,
    MetaMail,
    Bank(String),      // Bank name
    Government(String), // Government entity
    Custom(String),    // Custom provider
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WalletCapability {
    /// Basic wallet operations
    BasicWallet,
    /// Encrypted messaging
    SecureMessaging,
    /// Payment processing
    PaymentProcessing,
    /// Video calling
    VideoConferencing,
    /// Device authorization
    DeviceAuthorization,
    /// Cross-border payments
    CrossBorderPayments,
    /// Government services
    GovernmentServices,
    /// Banking services
    BankingServices,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationLevel {
    /// Unverified wallet
    None,
    /// Basic email verification
    Email,
    /// Phone number verification
    Phone,
    /// Government ID verification
    GovernmentID,
    /// Bank account verification
    BankAccount,
    /// Full KYC compliance
    FullKYC,
    /// Government-issued wallet
    Government,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletSummary {
    pub wallet_address: String,
    pub provider: WalletProvider,
    pub verification_level: VerificationLevel,
    pub capabilities: Vec<WalletCapability>,
    pub has_smtp: bool,
    pub has_did: bool,
    pub is_active: bool,
    pub age_days: i64,
    pub created_at: DateTime<Utc>,
}

impl Clone for WalletIdentity {
    fn clone(&self) -> Self {
        let keypair = Keypair::from_bytes(&self.keypair.to_bytes()).unwrap();
        Self {
            wallet_address: self.wallet_address.clone(),
            sync_address: self.sync_address.clone(),
            smtp_email: self.smtp_email.clone(),
            auth_token: self.auth_token.clone(),
            provider: self.provider.clone(),
            keypair,
            public_key: self.public_key,
            created_at: self.created_at,
            last_active: self.last_active,
            capabilities: self.capabilities.clone(),
            verification_level: self.verification_level.clone(),
            did: self.did.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

impl WalletIdentity {
    /// Create a new wallet identity with enhanced capabilities
    pub fn new(
        username: &str,
        provider: WalletProvider,
        smtp_email: Option<String>,
    ) -> Result<Self, WalletError> {
        Self::new_with_capabilities(
            username,
            provider,
            smtp_email,
            vec![WalletCapability::BasicWallet],
            VerificationLevel::None,
        )
    }

    /// Create a new wallet identity with specific capabilities
    pub fn new_with_capabilities(
        username: &str,
        provider: WalletProvider,
        smtp_email: Option<String>,
        capabilities: Vec<WalletCapability>,
        verification_level: VerificationLevel,
    ) -> Result<Self, WalletError> {
        // Generate keypair using a simple approach compatible with ed25519-dalek v1.0
        let secret_key_bytes: [u8; 32] = rand::random();
        let secret_key = ed25519_dalek::SecretKey::from_bytes(&secret_key_bytes)
            .map_err(|_| WalletError::InvalidWalletFormat)?;
        let public_key = ed25519_dalek::PublicKey::from(&secret_key);
        let keypair = Keypair { secret: secret_key, public: public_key };
        let sync_address = format!("0x{}", hex::encode(&keypair.public.to_bytes()));
        
        let provider_domain = match &provider {
            WalletProvider::Pravyom => "pravyom.wallet",
            WalletProvider::MetaMail => "metamail.wallet",
            WalletProvider::Bank(name) => &format!("{}.wallet", name.to_lowercase()),
            WalletProvider::Government(entity) => &format!("{}.wallet", entity.to_lowercase()),
            WalletProvider::Custom(domain) => domain,
        };
        
        let wallet_address = format!("{}@{}", username, provider_domain);
        let auth_token = format!("auth_{}", hex::encode(&rand::random::<[u8; 16]>()));
        
        let now = Utc::now();
        
        Ok(WalletIdentity {
            wallet_address,
            sync_address,
            smtp_email,
            auth_token,
            provider,
            keypair,
            public_key,
            created_at: now,
            last_active: now,
            capabilities,
            verification_level,
            did: None,
            metadata: HashMap::new(),
        })
    }
    
    /// Generate encrypted authentication token
    fn generate_auth_token(keypair: &Keypair) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&keypair.public.to_bytes());
        hasher.update(&Utc::now().timestamp().to_be_bytes());
        hex::encode(hasher.finalize())
    }
    
    /// Get full wallet identifier with sync address and email
    pub fn full_identifier(&self) -> String {
        match &self.smtp_email {
            Some(email) => format!("{}<{}>{{{}, {}}}", 
                self.wallet_address, 
                self.sync_address, 
                email, 
                self.auth_token
            ),
            None => format!("{}<{}>{{{}}}", 
                self.wallet_address, 
                self.sync_address, 
                self.auth_token
            ),
        }
    }

    /// Update wallet capabilities
    pub fn add_capability(&mut self, capability: WalletCapability) {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
            self.last_active = Utc::now();
        }
    }

    /// Remove wallet capability
    pub fn remove_capability(&mut self, capability: &WalletCapability) {
        self.capabilities.retain(|c| c != capability);
        self.last_active = Utc::now();
    }

    /// Check if wallet has specific capability
    pub fn has_capability(&self, capability: &WalletCapability) -> bool {
        self.capabilities.contains(capability)
    }

    /// Upgrade verification level
    pub fn upgrade_verification(&mut self, new_level: VerificationLevel) -> Result<(), WalletError> {
        // Only allow upgrades, not downgrades
        let current_level = self.verification_level_score();
        let new_level_score = Self::verification_level_score_static(&new_level);
        
        if new_level_score > current_level {
            self.verification_level = new_level;
            self.last_active = Utc::now();
            Ok(())
        } else {
            Err(WalletError::InvalidWalletFormat) // Reusing existing error for now
        }
    }

    /// Get verification level score for comparison
    pub fn verification_level_score(&self) -> u8 {
        Self::verification_level_score_static(&self.verification_level)
    }

    /// Get verification level score for any level
    fn verification_level_score_static(level: &VerificationLevel) -> u8 {
        match level {
            VerificationLevel::None => 0,
            VerificationLevel::Email => 1,
            VerificationLevel::Phone => 2,
            VerificationLevel::GovernmentID => 3,
            VerificationLevel::BankAccount => 4,
            VerificationLevel::FullKYC => 5,
            VerificationLevel::Government => 6,
        }
    }

    /// Set DID (Decentralized Identifier)
    pub fn set_did(&mut self, did: String) {
        self.did = Some(did);
        self.last_active = Utc::now();
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.last_active = Utc::now();
    }

    /// Get metadata
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Check if wallet is active (used within last 30 days)
    pub fn is_active(&self) -> bool {
        let thirty_days_ago = Utc::now() - chrono::Duration::days(30);
        self.last_active > thirty_days_ago
    }

    /// Get wallet age in days
    pub fn age_days(&self) -> i64 {
        (Utc::now() - self.created_at).num_days()
    }

    /// Generate wallet summary for display
    pub fn summary(&self) -> WalletSummary {
        WalletSummary {
            wallet_address: self.wallet_address.clone(),
            provider: self.provider.clone(),
            verification_level: self.verification_level.clone(),
            capabilities: self.capabilities.clone(),
            has_smtp: self.smtp_email.is_some(),
            has_did: self.did.is_some(),
            is_active: self.is_active(),
            age_days: self.age_days(),
            created_at: self.created_at,
        }
    }
    
    /// Sign a message with the wallet's private key
    pub fn sign_message(&self, message: &[u8]) -> Signature {
        self.keypair.sign(message)
    }
    
    /// Verify a signature against this wallet's public key
    pub fn verify_signature(&self, message: &[u8], signature: &Signature) -> bool {
        self.keypair.public.verify(message, signature).is_ok()
    }
    

    
    /// Check if wallet is from a trusted provider
    pub fn is_trusted_provider(&self) -> bool {
        matches!(self.provider, 
            WalletProvider::Pravyom | 
            WalletProvider::Bank(_) | 
            WalletProvider::Government(_)
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletRegistry {
    /// Map of wallet addresses to wallet identities
    wallets: HashMap<String, WalletIdentity>,
    /// Map of sync addresses to wallet addresses
    sync_address_map: HashMap<String, String>,
    /// Map of email addresses to wallet addresses
    email_map: HashMap<String, String>,
}

impl WalletRegistry {
    pub fn new() -> Self {
        Self {
            wallets: HashMap::new(),
            sync_address_map: HashMap::new(),
            email_map: HashMap::new(),
        }
    }
    
    /// Register a new wallet identity
    pub fn register_wallet(&mut self, wallet: WalletIdentity) -> Result<(), WalletError> {
        // Check for duplicates
        if self.wallets.contains_key(&wallet.wallet_address) {
            return Err(WalletError::WalletAlreadyExists);
        }
        
        if self.sync_address_map.contains_key(&wallet.sync_address) {
            return Err(WalletError::SyncAddressAlreadyExists);
        }
        
        // Register mappings
        self.sync_address_map.insert(wallet.sync_address.clone(), wallet.wallet_address.clone());
        
        if let Some(email) = &wallet.smtp_email {
            self.email_map.insert(email.clone(), wallet.wallet_address.clone());
        }
        
        self.wallets.insert(wallet.wallet_address.clone(), wallet);
        Ok(())
    }
    
    /// Lookup wallet by wallet address
    pub fn get_wallet(&self, wallet_address: &str) -> Option<&WalletIdentity> {
        self.wallets.get(wallet_address)
    }
    
    /// Lookup wallet by sync address
    pub fn get_wallet_by_sync_address(&self, sync_address: &str) -> Option<&WalletIdentity> {
        self.sync_address_map.get(sync_address)
            .and_then(|addr| self.wallets.get(addr))
    }
    
    /// Lookup wallet by email address
    pub fn get_wallet_by_email(&self, email: &str) -> Option<&WalletIdentity> {
        self.email_map.get(email)
            .and_then(|addr| self.wallets.get(addr))
    }
    
    /// Update wallet activity
    pub fn update_wallet_activity(&mut self, wallet_address: &str) -> Result<(), WalletError> {
        match self.wallets.get_mut(wallet_address) {
            Some(wallet) => {
                wallet.last_active = Utc::now();
                Ok(())
            }
            None => Err(WalletError::WalletNotFound),
        }
    }
    
    /// Get all wallets for a provider
    pub fn get_wallets_by_provider(&self, provider: &WalletProvider) -> Vec<&WalletIdentity> {
        self.wallets.values()
            .filter(|wallet| std::mem::discriminant(&wallet.provider) == std::mem::discriminant(provider))
            .collect()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WalletError {
    #[error("Wallet already exists")]
    WalletAlreadyExists,
    #[error("Sync address already exists")]
    SyncAddressAlreadyExists,
    #[error("Wallet not found")]
    WalletNotFound,
    #[error("Invalid wallet format")]
    InvalidWalletFormat,
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("Provider not supported")]
    ProviderNotSupported,
}

/// Parse a full wallet identifier string
pub fn parse_wallet_identifier(identifier: &str) -> Result<(String, String, Option<String>), WalletError> {
    // Format: user@provider.wallet<sync_address>{smtp_email, auth_token}
    let parts: Vec<&str> = identifier.split('<').collect();
    if parts.len() != 2 {
        return Err(WalletError::InvalidWalletFormat);
    }
    
    let wallet_address = parts[0].to_string();
    
    let remaining = parts[1];
    let sync_parts: Vec<&str> = remaining.split('>').collect();
    if sync_parts.len() != 2 {
        return Err(WalletError::InvalidWalletFormat);
    }
    
    let sync_address = sync_parts[0].to_string();
    
    // Parse email and auth token from {email, token}
    let auth_part = sync_parts[1];
    if !auth_part.starts_with('{') || !auth_part.ends_with('}') {
        return Err(WalletError::InvalidWalletFormat);
    }
    
    let inner = &auth_part[1..auth_part.len()-1];
    let auth_parts: Vec<&str> = inner.split(", ").collect();
    
    let smtp_email = if auth_parts.len() == 2 {
        Some(auth_parts[0].to_string())
    } else {
        None
    };
    
    Ok((wallet_address, sync_address, smtp_email))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wallet_creation() {
        let wallet = WalletIdentity::new(
            "alice",
            WalletProvider::Pravyom,
            Some("alice@gmail.com".to_string()),
        ).unwrap();
        
        assert_eq!(wallet.wallet_address, "alice@pravyom.wallet");
        assert!(wallet.sync_address.starts_with("0x"));
        assert_eq!(wallet.smtp_email, Some("alice@gmail.com".to_string()));
        assert!(wallet.is_trusted_provider());
    }
    
    #[test]
    fn test_wallet_registry() {
        let mut registry = WalletRegistry::new();
        
        let wallet = WalletIdentity::new(
            "bob",
            WalletProvider::MetaMail,
            Some("bob@outlook.com".to_string()),
        ).unwrap();
        
        let wallet_address = wallet.wallet_address.clone();
        let sync_address = wallet.sync_address.clone();
        
        registry.register_wallet(wallet).unwrap();
        
        assert!(registry.get_wallet(&wallet_address).is_some());
        assert!(registry.get_wallet_by_sync_address(&sync_address).is_some());
        assert!(registry.get_wallet_by_email("bob@outlook.com").is_some());
    }
    
    #[test]
    fn test_wallet_identifier_parsing() {
        let identifier = "alice@pravyom.wallet<0x1234...>{alice@gmail.com, auth_token_123}";
        let (wallet_address, sync_address, smtp_email) = parse_wallet_identifier(identifier).unwrap();
        
        assert_eq!(wallet_address, "alice@pravyom.wallet");
        assert_eq!(sync_address, "0x1234...");
        assert_eq!(smtp_email, Some("alice@gmail.com".to_string()));
    }
    
    #[test]
    fn test_message_signing() {
        let wallet = WalletIdentity::new(
            "charlie",
            WalletProvider::Bank("chase".to_string()),
            None,
        ).unwrap();
        
        let message = b"Hello, world!";
        let signature = wallet.sign_message(message);
        
        assert!(wallet.verify_signature(message, &signature));
        assert!(!wallet.verify_signature(b"Different message", &signature));
    }
}
