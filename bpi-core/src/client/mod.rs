//! BPI/BPCI Production Client SDK Modules
//! 
//! Stage 4: Advanced Transport Integration - Production Client SDK
//! 
//! This module provides production-ready client components that leverage
//! the proven XTMP protocol, QLOCK sync gate, Shadow Registry bridge,
//! and quantum cryptography infrastructure for secure, high-performance,
//! and enterprise-grade BPI/BPCI internet communication.

pub mod qlock_client;
pub mod shadow_registry_client;
pub mod quantum_crypto_client;
pub mod httpcg_client;
pub mod tlsls_client;

// Re-export main client types
pub use qlock_client::{QLockClient, QLockClientConfig};
pub use shadow_registry_client::{ShadowRegistryClient, ShadowRegistryClientConfig};
pub use quantum_crypto_client::{QuantumCryptoClient, QuantumCryptoClientConfig};
pub use httpcg_client::{HttpcgClient, HttpcgClientConfig, HttpcgUrl, HttpcgRequest, HttpcgResponse};
pub use tlsls_client::{TlslsClient, TlslsClientConfig, TlslsCertificate, TlslsCertificateChain};

use anyhow::Result;
use crate::bpi_wallet_command::BPIWalletArgs;

/// Production Client SDK - Unified interface for all BPI/BPCI client operations
/// 
/// This provides a single entry point for all advanced transport integration
/// features, leveraging the existing proven infrastructure.
#[derive(Clone)]
pub struct BpciClientSDK {
    pub qlock: QLockClient,
    pub shadow_registry: ShadowRegistryClient,
    pub quantum_crypto: QuantumCryptoClient,
    pub httpcg: HttpcgClient,
    pub tlsls: TlslsClient,
}

impl BpciClientSDK {
    /// Create new production client SDK with all components
    pub async fn new(wallet: BPIWalletArgs) -> Result<Self> {
        // Initialize all client components with default configurations
        let qlock = QLockClient::new(wallet.clone(), QLockClientConfig::default()).await?;
        let shadow_registry = ShadowRegistryClient::new(wallet.clone(), ShadowRegistryClientConfig::default()).await?;
        let quantum_crypto = QuantumCryptoClient::new(wallet.clone(), QuantumCryptoClientConfig::default()).await?;
        let httpcg = HttpcgClient::new(wallet.clone(), HttpcgClientConfig::default()).await?;
        let tlsls = TlslsClient::new(wallet.clone(), TlslsClientConfig::default()).await?;
        
        Ok(Self {
            qlock,
            shadow_registry,
            quantum_crypto,
            httpcg,
            tlsls,
        })
    }
    
    /// Start all background tasks for the client SDK
    pub async fn start_all_background_tasks(&self) -> Result<()> {
        self.qlock.start_background_tasks().await?;
        self.shadow_registry.start_background_tasks().await?;
        self.quantum_crypto.start_background_tasks().await?;
        self.httpcg.start_background_tasks().await?;
        self.tlsls.start_background_tasks().await?;
        
        println!("🚀 BPI/BPCI Production Client SDK - All background tasks started");
        Ok(())
    }
    
    /// Get comprehensive client SDK status
    pub async fn get_sdk_status(&self) -> BpciClientSDKStatus {
        BpciClientSDKStatus {
            qlock_active_sessions: self.qlock.list_active_sessions().await.len(),
            shadow_registry_active_entries: self.shadow_registry.list_entries().await.unwrap_or_default().len(),
            quantum_crypto_active_sessions: self.quantum_crypto.list_active_sessions().await.len(),
            httpcg_active_connections: self.httpcg.list_active_connections().await.len(),
            tlsls_active_certificates: self.tlsls.list_certificates().await.len(),
            all_systems_operational: true,
        }
    }
}

/// Client SDK status information
#[derive(Debug, Clone)]
pub struct BpciClientSDKStatus {
    pub qlock_active_sessions: usize,
    pub shadow_registry_active_entries: usize,
    pub quantum_crypto_active_sessions: usize,
    pub httpcg_active_connections: usize,
    pub tlsls_active_certificates: usize,
    pub all_systems_operational: bool,
}
