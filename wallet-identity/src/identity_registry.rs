use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use sha2::{Sha256, Digest};
use anyhow::{Result, anyhow};

use crate::wallet_identity::{WalletIdentity, WalletProvider, WalletError};

/// Universal Identity Registry
/// Manages wallet discovery, verification, and cross-provider communication
#[derive(Debug, Clone)]
pub struct IdentityRegistry {
    /// Registered wallets by address
    wallets: HashMap<String, WalletRegistration>,
    /// Provider directory for wallet discovery
    provider_directory: HashMap<String, ProviderInfo>,
    /// DID document registry
    did_registry: HashMap<String, DIDDocument>,
    /// Identity verification cache
    verification_cache: HashMap<String, VerificationResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletRegistration {
    /// Wallet identity
    pub wallet: WalletIdentity,
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
    /// Last verification timestamp
    pub last_verified: DateTime<Utc>,
    /// Verification status
    pub verification_status: VerificationStatus,
    /// Associated DID document
    pub did_document: Option<String>,
    /// Provider-specific metadata
    pub provider_metadata: HashMap<String, String>,
    /// Public key for verification
    pub public_key: PublicKey,
    /// Registration signature
    pub registration_signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    /// Provider name
    pub name: String,
    /// Provider type
    pub provider_type: WalletProvider,
    /// Provider endpoints
    pub endpoints: Vec<String>,
    /// Provider public key
    pub public_key: PublicKey,
    /// Provider capabilities
    pub capabilities: Vec<ProviderCapability>,
    /// Trust score (0.0 - 1.0)
    pub trust_score: f64,
    /// Regulatory compliance
    pub compliance: ComplianceInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderCapability {
    /// Basic wallet services
    BasicWallet,
    /// Encrypted messaging
    SecureMessaging,
    /// Payment processing
    PaymentProcessing,
    /// Video calling
    VideoConferencing,
    /// Device authorization
    DeviceAuthorization,
    /// Government integration
    GovernmentServices,
    /// Banking integration
    BankingServices,
    /// Cross-border transfers
    CrossBorderPayments,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceInfo {
    /// KYC compliance level
    pub kyc_level: KYCLevel,
    /// AML compliance
    pub aml_compliant: bool,
    /// Regulatory jurisdictions
    pub jurisdictions: Vec<String>,
    /// Compliance certifications
    pub certifications: Vec<String>,
    /// Last audit date
    pub last_audit: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KYCLevel {
    /// No KYC required
    None,
    /// Basic identity verification
    Basic,
    /// Enhanced due diligence
    Enhanced,
    /// Full regulatory compliance
    Full,
    /// Government-grade verification
    Government,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationStatus {
    /// Pending verification
    Pending,
    /// Successfully verified
    Verified,
    /// Verification failed
    Failed,
    /// Verification expired
    Expired,
    /// Suspended
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIDDocument {
    /// DID identifier
    pub id: String,
    /// DID context
    pub context: Vec<String>,
    /// Public keys
    pub public_keys: Vec<DIDPublicKey>,
    /// Authentication methods
    pub authentication: Vec<String>,
    /// Service endpoints
    pub service: Vec<DIDService>,
    /// Creation timestamp
    pub created: DateTime<Utc>,
    /// Last update timestamp
    pub updated: DateTime<Utc>,
    /// Document signature
    pub proof: DIDProof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIDPublicKey {
    /// Key ID
    pub id: String,
    /// Key type (Ed25519, etc.)
    pub key_type: String,
    /// Key controller
    pub controller: String,
    /// Public key bytes
    pub public_key_base58: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIDService {
    /// Service ID
    pub id: String,
    /// Service type
    pub service_type: String,
    /// Service endpoint
    pub service_endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIDProof {
    /// Proof type
    pub proof_type: String,
    /// Creation timestamp
    pub created: DateTime<Utc>,
    /// Verification method
    pub verification_method: String,
    /// Proof signature
    pub proof_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    /// Verification status
    pub status: VerificationStatus,
    /// Verification timestamp
    pub verified_at: DateTime<Utc>,
    /// Verification method
    pub method: String,
    /// Verification score (0.0 - 1.0)
    pub score: f64,
    /// Verification details
    pub details: HashMap<String, String>,
    /// Expiry timestamp
    pub expires_at: Option<DateTime<Utc>>,
}

impl IdentityRegistry {
    /// Create a new identity registry
    pub fn new() -> Self {
        Self {
            wallets: HashMap::new(),
            provider_directory: HashMap::new(),
            did_registry: HashMap::new(),
            verification_cache: HashMap::new(),
        }
    }

    /// Register a new wallet identity
    pub fn register_wallet(&mut self, wallet: WalletIdentity) -> Result<()> {
        let wallet_address = wallet.wallet_address.clone();
        
        // Check if wallet already exists
        if self.wallets.contains_key(&wallet_address) {
            return Err(anyhow!("Wallet already registered: {}", wallet_address));
        }

        // Create registration signature
        let registration_data = self.create_registration_data(&wallet)?;
        let registration_signature = wallet.keypair.sign(&registration_data).to_bytes().to_vec();

        let registration = WalletRegistration {
            public_key: wallet.public_key,
            wallet,
            registered_at: Utc::now(),
            last_verified: Utc::now(),
            verification_status: VerificationStatus::Pending,
            did_document: None,
            provider_metadata: HashMap::new(),
            registration_signature,
        };

        self.wallets.insert(wallet_address, registration);
        Ok(())
    }

    /// Discover wallet by address
    pub fn discover_wallet(&self, wallet_address: &str) -> Option<&WalletRegistration> {
        self.wallets.get(wallet_address)
    }

    /// Search wallets by provider
    pub fn search_by_provider(&self, provider: &WalletProvider) -> Vec<&WalletRegistration> {
        self.wallets
            .values()
            .filter(|reg| std::mem::discriminant(&reg.wallet.provider) == std::mem::discriminant(provider))
            .collect()
    }

    /// Verify wallet identity
    pub fn verify_wallet(&mut self, wallet_address: &str) -> Result<VerificationResult> {
        let registration = self.wallets.get(wallet_address)
            .ok_or_else(|| anyhow!("Wallet not found: {}", wallet_address))?;

        // Verify registration signature
        let registration_data = self.create_registration_data(&registration.wallet)?;
        let signature = Signature::from_bytes(&registration.registration_signature)
            .map_err(|e| anyhow!("Invalid signature: {}", e))?;

        if registration.public_key.verify(&registration_data, &signature).is_err() {
            return Err(anyhow!("Invalid registration signature"));
        }

        // Create verification result
        let verification = VerificationResult {
            status: VerificationStatus::Verified,
            verified_at: Utc::now(),
            method: "cryptographic_signature".to_string(),
            score: 1.0,
            details: HashMap::new(),
            expires_at: Some(Utc::now() + chrono::Duration::days(30)),
        };

        // Update wallet verification status
        if let Some(reg) = self.wallets.get_mut(wallet_address) {
            reg.verification_status = VerificationStatus::Verified;
            reg.last_verified = Utc::now();
        }

        // Cache verification result
        self.verification_cache.insert(wallet_address.to_string(), verification.clone());

        Ok(verification)
    }

    /// Register a provider
    pub fn register_provider(&mut self, provider_info: ProviderInfo) -> Result<()> {
        let provider_name = provider_info.name.clone();
        
        // Validate provider information
        self.validate_provider(&provider_info)?;
        
        self.provider_directory.insert(provider_name, provider_info);
        Ok(())
    }

    /// Get provider information
    pub fn get_provider(&self, provider_name: &str) -> Option<&ProviderInfo> {
        self.provider_directory.get(provider_name)
    }

    /// Create DID document for wallet
    pub fn create_did_document(&mut self, wallet_address: &str) -> Result<DIDDocument> {
        let registration = self.wallets.get(wallet_address)
            .ok_or_else(|| anyhow!("Wallet not found: {}", wallet_address))?;

        let did_id = format!("did:bpi:{}", wallet_address);
        
        let public_key = DIDPublicKey {
            id: format!("{}#key-1", did_id),
            key_type: "Ed25519VerificationKey2018".to_string(),
            controller: did_id.clone(),
            public_key_base58: hex::encode(&registration.public_key.to_bytes()),
        };

        let service = DIDService {
            id: format!("{}#wallet-service", did_id),
            service_type: "WalletService".to_string(),
            service_endpoint: format!("https://api.pravyom.com/wallet/{}", wallet_address),
        };

        // Create proof
        let proof_data = format!("{}{}", did_id, Utc::now().timestamp());
        let proof_signature = registration.wallet.keypair.sign(proof_data.as_bytes());
        
        let proof = DIDProof {
            proof_type: "Ed25519Signature2018".to_string(),
            created: Utc::now(),
            verification_method: format!("{}#key-1", did_id),
            proof_value: hex::encode(&proof_signature.to_bytes()),
        };

        let did_document = DIDDocument {
            id: did_id.clone(),
            context: vec![
                "https://www.w3.org/ns/did/v1".to_string(),
                "https://w3id.org/security/v1".to_string(),
            ],
            public_keys: vec![public_key],
            authentication: vec![format!("{}#key-1", did_id)],
            service: vec![service],
            created: Utc::now(),
            updated: Utc::now(),
            proof,
        };

        // Store DID document
        self.did_registry.insert(did_id.clone(), did_document.clone());

        // Update wallet registration with DID
        if let Some(reg) = self.wallets.get_mut(wallet_address) {
            reg.did_document = Some(did_id);
        }

        Ok(did_document)
    }

    /// Resolve DID document
    pub fn resolve_did(&self, did: &str) -> Option<&DIDDocument> {
        self.did_registry.get(did)
    }

    /// Create registration data for signing
    fn create_registration_data(&self, wallet: &WalletIdentity) -> Result<Vec<u8>> {
        let mut hasher = Sha256::new();
        hasher.update(wallet.wallet_address.as_bytes());
        hasher.update(wallet.sync_address.as_bytes());
        hasher.update(&wallet.public_key.to_bytes());
        hasher.update(wallet.created_at.timestamp().to_be_bytes());
        Ok(hasher.finalize().to_vec())
    }

    /// Validate provider information
    fn validate_provider(&self, provider: &ProviderInfo) -> Result<()> {
        if provider.name.is_empty() {
            return Err(anyhow!("Provider name cannot be empty"));
        }

        if provider.endpoints.is_empty() {
            return Err(anyhow!("Provider must have at least one endpoint"));
        }

        if provider.trust_score < 0.0 || provider.trust_score > 1.0 {
            return Err(anyhow!("Trust score must be between 0.0 and 1.0"));
        }

        Ok(())
    }

    /// Get wallet statistics
    pub fn get_statistics(&self) -> RegistryStatistics {
        let total_wallets = self.wallets.len();
        let verified_wallets = self.wallets.values()
            .filter(|reg| matches!(reg.verification_status, VerificationStatus::Verified))
            .count();
        let total_providers = self.provider_directory.len();
        let total_dids = self.did_registry.len();

        RegistryStatistics {
            total_wallets,
            verified_wallets,
            total_providers,
            total_dids,
            verification_rate: if total_wallets > 0 {
                verified_wallets as f64 / total_wallets as f64
            } else {
                0.0
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryStatistics {
    pub total_wallets: usize,
    pub verified_wallets: usize,
    pub total_providers: usize,
    pub total_dids: usize,
    pub verification_rate: f64,
}

impl Default for IdentityRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_registration() {
        let mut registry = IdentityRegistry::new();
        
        let wallet = WalletIdentity::new(
            "alice",
            WalletProvider::Pravyom,
            Some("alice@example.com".to_string()),
        ).unwrap();

        assert!(registry.register_wallet(wallet).is_ok());
        assert_eq!(registry.wallets.len(), 1);
    }

    #[test]
    fn test_wallet_discovery() {
        let mut registry = IdentityRegistry::new();
        
        let wallet = WalletIdentity::new(
            "bob",
            WalletProvider::MetaMail,
            None,
        ).unwrap();

        let wallet_address = wallet.wallet_address.clone();
        registry.register_wallet(wallet).unwrap();

        let discovered = registry.discover_wallet(&wallet_address);
        assert!(discovered.is_some());
        assert_eq!(discovered.unwrap().wallet.wallet_address, wallet_address);
    }

    #[test]
    fn test_provider_registration() {
        let mut registry = IdentityRegistry::new();
        
        let provider = ProviderInfo {
            name: "TestProvider".to_string(),
            provider_type: WalletProvider::Custom("test".to_string()),
            endpoints: vec!["https://api.test.com".to_string()],
            public_key: PublicKey::from_bytes(&[0u8; 32]).unwrap(),
            capabilities: vec![ProviderCapability::BasicWallet],
            trust_score: 0.8,
            compliance: ComplianceInfo {
                kyc_level: KYCLevel::Basic,
                aml_compliant: true,
                jurisdictions: vec!["US".to_string()],
                certifications: vec![],
                last_audit: None,
            },
        };

        assert!(registry.register_provider(provider).is_ok());
        assert_eq!(registry.provider_directory.len(), 1);
    }
}
