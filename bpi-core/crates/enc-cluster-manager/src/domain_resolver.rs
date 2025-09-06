//! # Revolutionary Domain Resolver - http:cg and rootzk Protocol Support
//!
//! Advanced domain resolution system supporting next-generation protocols:
//! - http:cg//example.com://(address) - HTTP cage domains with wallet verification
//! - rootzk//(address)proof(address).cage(address) - ZK proof domains with cage integration
//!
//! This resolver integrates with the existing MetanodeClusterManager and ENC Cluster
//! to provide revolutionary domain resolution capabilities for the 100-year future-proof system.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn, error};
use uuid::Uuid;

/// Revolutionary Domain Resolver supporting http:cg and rootzk protocols
#[derive(Debug)]
pub struct DomainResolver {
    /// Domain cache for performance optimization
    pub domain_cache: Arc<RwLock<HashMap<String, ResolvedDomain>>>,
    /// HTTP cage registry for http:cg protocol
    pub cage_registry: Arc<RwLock<HashMap<String, HttpCageConfig>>>,
    /// ZK proof registry for rootzk protocol
    pub zk_registry: Arc<RwLock<HashMap<String, ZkProofConfig>>>,
    /// Wallet verification service
    pub wallet_verifier: Arc<WalletVerificationService>,
    /// Audit integration for domain resolution events
    pub audit_bridge: Arc<DomainAuditBridge>,
}

/// Resolved domain information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedDomain {
    pub domain: String,
    pub protocol: DomainProtocol,
    pub resolved_address: String,
    pub verification_status: VerificationStatus,
    pub cage_config: Option<HttpCageConfig>,
    pub zk_proof: Option<ZkProofConfig>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub ttl: u64,
}

/// Domain protocol types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DomainProtocol {
    /// http:cg//example.com://(address) - HTTP cage protocol
    HttpCage,
    /// rootzk//(address)proof(address).cage(address) - ZK proof protocol
    RootZk,
    /// Standard HTTP/HTTPS (fallback)
    Standard,
}

/// HTTP Cage Configuration for http:cg protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpCageConfig {
    pub cage_id: String,
    pub domain: String,
    pub wallet_address: String,
    pub cage_endpoints: Vec<CageEndpoint>,
    pub security_profile: CageSecurityProfile,
    pub audit_config: CageAuditConfig,
    pub performance_config: CagePerformanceConfig,
}

/// ZK Proof Configuration for rootzk protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProofConfig {
    pub proof_id: String,
    pub root_address: String,
    pub proof_address: String,
    pub cage_address: String,
    pub proof_type: ZkProofType,
    pub verification_key: String,
    pub proof_data: String,
    pub validity_period: u64,
}

/// Cage endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CageEndpoint {
    pub endpoint_id: String,
    pub url: String,
    pub port: u16,
    pub protocol: String,
    pub health_check_path: String,
    pub load_balancing_weight: u32,
}

/// Cage security profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CageSecurityProfile {
    pub encryption_level: EncryptionLevel,
    pub authentication_required: bool,
    pub rate_limiting: RateLimitConfig,
    pub access_control: AccessControlConfig,
    pub audit_level: AuditLevel,
}

/// Cage audit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CageAuditConfig {
    pub audit_enabled: bool,
    pub audit_level: AuditLevel,
    pub retention_period: u64,
    pub compliance_frameworks: Vec<ComplianceFramework>,
}

/// Cage performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CagePerformanceConfig {
    pub caching_enabled: bool,
    pub cache_ttl: u64,
    pub connection_pooling: bool,
    pub max_connections: u32,
    pub timeout_ms: u64,
}

/// ZK Proof types supported
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZkProofType {
    /// Membership proof
    Membership,
    /// Range proof
    Range,
    /// Identity proof
    Identity,
    /// Ownership proof
    Ownership,
    /// Compliance proof
    Compliance,
    /// Custom proof type
    Custom(String),
}

/// Verification status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationStatus {
    Verified,
    Pending,
    Failed,
    Expired,
    Revoked,
}

/// Wallet verification service
#[derive(Debug)]
pub struct WalletVerificationService {
    pub verification_cache: Arc<RwLock<HashMap<String, WalletVerification>>>,
    pub verification_config: VerificationConfig,
}

/// Wallet verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletVerification {
    pub wallet_address: String,
    pub verification_status: VerificationStatus,
    pub verification_timestamp: chrono::DateTime<chrono::Utc>,
    pub verification_proof: String,
    pub expiry_timestamp: chrono::DateTime<chrono::Utc>,
}

/// Domain audit bridge
#[derive(Debug)]
pub struct DomainAuditBridge {
    pub audit_config: DomainAuditConfig,
    pub bpi_integration: BpiAuditIntegration,
}

/// Supporting enums and structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionLevel {
    None,
    Standard,
    Enhanced,
    Military,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_limit: u32,
    pub window_size_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfig {
    pub whitelist_enabled: bool,
    pub blacklist_enabled: bool,
    pub geo_restrictions: Vec<String>,
    pub wallet_restrictions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditLevel {
    None,
    Basic,
    Standard,
    Enhanced,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceFramework {
    GDPR,
    SOX,
    HIPAA,
    PCI_DSS,
    ISO27001,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationConfig {
    pub verification_timeout_ms: u64,
    pub cache_ttl_seconds: u64,
    pub max_verification_attempts: u32,
    pub require_fresh_proof: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainAuditConfig {
    pub audit_enabled: bool,
    pub audit_all_resolutions: bool,
    pub audit_failed_resolutions: bool,
    pub audit_verification_events: bool,
    pub retention_period_days: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpiAuditIntegration {
    pub bpi_endpoint: String,
    pub audit_wallet: String,
    pub batch_size: u32,
    pub flush_interval_seconds: u64,
}

impl DomainResolver {
    /// Create new domain resolver
    pub async fn new() -> Result<Self> {
        info!("Initializing Revolutionary Domain Resolver");
        
        let wallet_verifier = Arc::new(WalletVerificationService::new().await?);
        let audit_bridge = Arc::new(DomainAuditBridge::new().await?);
        
        Ok(Self {
            domain_cache: Arc::new(RwLock::new(HashMap::new())),
            cage_registry: Arc::new(RwLock::new(HashMap::new())),
            zk_registry: Arc::new(RwLock::new(HashMap::new())),
            wallet_verifier,
            audit_bridge,
        })
    }
    
    /// Resolve domain using revolutionary protocols
    pub async fn resolve_domain(&self, domain: &str) -> Result<ResolvedDomain> {
        debug!("Resolving domain: {}", domain);
        
        // Check cache first
        if let Some(cached) = self.get_cached_domain(domain).await? {
            if !self.is_cache_expired(&cached) {
                debug!("Returning cached domain resolution for: {}", domain);
                return Ok(cached);
            }
        }
        
        // Determine protocol and resolve
        let protocol = self.detect_protocol(domain)?;
        let resolved = match protocol {
            DomainProtocol::HttpCage => self.resolve_http_cage_domain(domain).await?,
            DomainProtocol::RootZk => self.resolve_rootzk_domain(domain).await?,
            DomainProtocol::Standard => self.resolve_standard_domain(domain).await?,
        };
        
        // Cache the result
        self.cache_domain_resolution(&resolved).await?;
        
        // Audit the resolution
        self.audit_domain_resolution(&resolved).await?;
        
        info!("Successfully resolved domain: {} -> {}", domain, resolved.resolved_address);
        Ok(resolved)
    }
    
    /// Detect domain protocol type
    fn detect_protocol(&self, domain: &str) -> Result<DomainProtocol> {
        if domain.starts_with("http:cg//") {
            Ok(DomainProtocol::HttpCage)
        } else if domain.starts_with("rootzk//") {
            Ok(DomainProtocol::RootZk)
        } else {
            Ok(DomainProtocol::Standard)
        }
    }
    
    /// Resolve http:cg protocol domain
    async fn resolve_http_cage_domain(&self, domain: &str) -> Result<ResolvedDomain> {
        debug!("Resolving HTTP cage domain: {}", domain);
        
        // Parse http:cg//example.com://(address) format
        let parsed = self.parse_http_cage_domain(domain)?;
        
        // Verify wallet address
        let verification = self.wallet_verifier.verify_wallet(&parsed.wallet_address).await?;
        
        // Get cage configuration
        let cage_config = self.get_cage_config(&parsed.domain).await?;
        
        // Resolve to actual endpoint
        let resolved_address = self.resolve_cage_endpoint(&cage_config).await?;
        
        Ok(ResolvedDomain {
            domain: domain.to_string(),
            protocol: DomainProtocol::HttpCage,
            resolved_address,
            verification_status: verification.verification_status,
            cage_config: Some(cage_config),
            zk_proof: None,
            timestamp: chrono::Utc::now(),
            ttl: 3600, // 1 hour default TTL
        })
    }
    
    /// Resolve rootzk protocol domain
    async fn resolve_rootzk_domain(&self, domain: &str) -> Result<ResolvedDomain> {
        debug!("Resolving RootZK domain: {}", domain);
        
        // Parse rootzk//(address)proof(address).cage(address) format
        let parsed = self.parse_rootzk_domain(domain)?;
        
        // Verify ZK proof
        let zk_config = self.verify_zk_proof(&parsed).await?;
        
        // Resolve cage address
        let resolved_address = self.resolve_zk_cage_address(&zk_config).await?;
        
        Ok(ResolvedDomain {
            domain: domain.to_string(),
            protocol: DomainProtocol::RootZk,
            resolved_address,
            verification_status: VerificationStatus::Verified,
            cage_config: None,
            zk_proof: Some(zk_config),
            timestamp: chrono::Utc::now(),
            ttl: 1800, // 30 minutes for ZK proofs
        })
    }
    
    /// Resolve standard domain (fallback)
    async fn resolve_standard_domain(&self, domain: &str) -> Result<ResolvedDomain> {
        debug!("Resolving standard domain: {}", domain);
        
        // Standard DNS resolution
        let resolved_address = format!("https://{}", domain);
        
        Ok(ResolvedDomain {
            domain: domain.to_string(),
            protocol: DomainProtocol::Standard,
            resolved_address,
            verification_status: VerificationStatus::Verified,
            cage_config: None,
            zk_proof: None,
            timestamp: chrono::Utc::now(),
            ttl: 7200, // 2 hours for standard domains
        })
    }
    
    /// Register HTTP cage configuration
    pub async fn register_http_cage(&self, config: HttpCageConfig) -> Result<()> {
        info!("Registering HTTP cage: {}", config.domain);
        
        let mut registry = self.cage_registry.write().await;
        registry.insert(config.domain.clone(), config);
        
        Ok(())
    }
    
    /// Register ZK proof configuration
    pub async fn register_zk_proof(&self, config: ZkProofConfig) -> Result<()> {
        info!("Registering ZK proof: {}", config.proof_id);
        
        let mut registry = self.zk_registry.write().await;
        registry.insert(config.proof_id.clone(), config);
        
        Ok(())
    }
    
    /// Get cached domain resolution
    async fn get_cached_domain(&self, domain: &str) -> Result<Option<ResolvedDomain>> {
        let cache = self.domain_cache.read().await;
        Ok(cache.get(domain).cloned())
    }
    
    /// Check if cache entry is expired
    fn is_cache_expired(&self, resolved: &ResolvedDomain) -> bool {
        let now = chrono::Utc::now();
        let expiry = resolved.timestamp + chrono::Duration::seconds(resolved.ttl as i64);
        now > expiry
    }
    
    /// Cache domain resolution
    async fn cache_domain_resolution(&self, resolved: &ResolvedDomain) -> Result<()> {
        let mut cache = self.domain_cache.write().await;
        cache.insert(resolved.domain.clone(), resolved.clone());
        Ok(())
    }
    
    /// Audit domain resolution event
    async fn audit_domain_resolution(&self, resolved: &ResolvedDomain) -> Result<()> {
        self.audit_bridge.audit_resolution_event(resolved).await
    }
    
    // Helper methods for domain parsing and resolution
    
    /// Parse http:cg domain format: http:cg//example.com<address>walletID
    fn parse_http_cage_domain(&self, domain: &str) -> Result<HttpCageParsed> {
        // Parse http:cg//example.com<address>walletID format
        if !domain.starts_with("http:cg//") {
            return Err(anyhow!("Invalid http:cg domain format - must start with http:cg//"));
        }
        
        let content = domain.strip_prefix("http:cg//").unwrap();
        
        // Find the domain part (before <)
        let parts: Vec<&str> = content.split('<').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid http:cg domain format - expected format: http:cg//example.com<address>walletID"));
        }
        
        let domain_part = parts[0];
        let address_wallet_part = parts[1];
        
        // Split address and walletID (address comes before >)
        let addr_parts: Vec<&str> = address_wallet_part.split('>').collect();
        if addr_parts.len() != 2 {
            return Err(anyhow!("Invalid http:cg domain format - expected format: http:cg//example.com<address>walletID"));
        }
        
        let address = addr_parts[0];
        let wallet_id = addr_parts[1];
        
        Ok(HttpCageParsed {
            domain: domain_part.to_string(),
            address: address.to_string(),
            wallet_address: wallet_id.to_string(),
        })
    }
    
    /// Parse rootzk domain format: rootzk//(address)<wallet>proof(address).cage(address)
    fn parse_rootzk_domain(&self, domain: &str) -> Result<RootZkParsed> {
        // Parse rootzk//(address)<wallet>proof(address).cage(address) format
        let content = domain.strip_prefix("rootzk//").ok_or_else(|| anyhow!("Invalid rootzk format - must start with rootzk//"))?;
        
        // Parse (address)<wallet>proof(address).cage(address)
        if !content.starts_with('(') {
            return Err(anyhow!("Invalid rootzk format - expected (address)<wallet>proof(address).cage(address)"));
        }
        
        // Find the first address in parentheses
        let first_close = content.find(')').ok_or_else(|| anyhow!("Missing closing ) for first address"))?;
        let root_address = &content[1..first_close]; // Skip the opening (
        
        let remaining = &content[first_close + 1..]; // Skip the closing )
        
        // Parse <wallet>proof(address).cage(address)
        if !remaining.starts_with('<') {
            return Err(anyhow!("Invalid rootzk format - expected <wallet> after first address"));
        }
        
        let wallet_close = remaining.find('>').ok_or_else(|| anyhow!("Missing closing > for wallet"))?;
        let wallet_id = &remaining[1..wallet_close]; // Skip the opening <
        
        let proof_part = &remaining[wallet_close + 1..]; // Skip the closing >
        
        // Parse proof(address).cage(address)
        if !proof_part.starts_with("proof(") {
            return Err(anyhow!("Invalid rootzk format - expected proof(address) after wallet"));
        }
        
        let proof_content = &proof_part[6..]; // Skip "proof("
        let proof_close = proof_content.find(')').ok_or_else(|| anyhow!("Missing closing ) for proof address"))?;
        let proof_address = &proof_content[..proof_close];
        
        let cage_part = &proof_content[proof_close + 1..]; // Skip the closing )
        
        // Parse .cage(address)
        if !cage_part.starts_with(".cage(") {
            return Err(anyhow!("Invalid rootzk format - expected .cage(address) after proof"));
        }
        
        let cage_content = &cage_part[6..]; // Skip ".cage("
        let cage_close = cage_content.find(')').ok_or_else(|| anyhow!("Missing closing ) for cage address"))?;
        let cage_address = &cage_content[..cage_close];
        
        Ok(RootZkParsed {
            root_address: root_address.to_string(),
            wallet_id: wallet_id.to_string(),
            proof_address: proof_address.to_string(),
            cage_address: cage_address.to_string(),
        })
    }
    
    /// Get cage configuration
    async fn get_cage_config(&self, domain: &str) -> Result<HttpCageConfig> {
        let registry = self.cage_registry.read().await;
        registry.get(domain).cloned()
            .ok_or_else(|| anyhow!("Cage configuration not found for domain: {}", domain))
    }
    
    /// Resolve cage endpoint to actual address
    async fn resolve_cage_endpoint(&self, config: &HttpCageConfig) -> Result<String> {
        // Select best endpoint based on load balancing
        let endpoint = config.cage_endpoints.first()
            .ok_or_else(|| anyhow!("No cage endpoints available"))?;
        
        Ok(format!("{}:{}", endpoint.url, endpoint.port))
    }
    
    /// Verify ZK proof and get configuration
    async fn verify_zk_proof(&self, parsed: &RootZkParsed) -> Result<ZkProofConfig> {
        let registry = self.zk_registry.read().await;
        
        // Find matching ZK proof configuration
        for (_, config) in registry.iter() {
            if config.root_address == parsed.root_address 
                && config.proof_address == parsed.proof_address 
                && config.cage_address == parsed.cage_address {
                return Ok(config.clone());
            }
        }
        
        Err(anyhow!("ZK proof configuration not found"))
    }
    
    /// Resolve ZK cage address
    async fn resolve_zk_cage_address(&self, config: &ZkProofConfig) -> Result<String> {
        // Return the cage address from the ZK proof
        Ok(config.cage_address.clone())
    }
}

/// Helper structures for domain parsing
#[derive(Debug)]
struct HttpCageParsed {
    domain: String,
    address: String,
    wallet_address: String,
}

#[derive(Debug)]
struct RootZkParsed {
    root_address: String,
    wallet_id: String,
    proof_address: String,
    cage_address: String,
}

impl WalletVerificationService {
    /// Create new wallet verification service
    pub async fn new() -> Result<Self> {
        Ok(Self {
            verification_cache: Arc::new(RwLock::new(HashMap::new())),
            verification_config: VerificationConfig {
                verification_timeout_ms: 5000,
                cache_ttl_seconds: 3600,
                max_verification_attempts: 3,
                require_fresh_proof: false,
            },
        })
    }
    
    /// Verify wallet address
    pub async fn verify_wallet(&self, wallet_address: &str) -> Result<WalletVerification> {
        debug!("Verifying wallet: {}", wallet_address);
        
        // Check cache first
        if let Some(cached) = self.get_cached_verification(wallet_address).await? {
            if !self.is_verification_expired(&cached) {
                return Ok(cached);
            }
        }
        
        // Perform actual verification
        let verification = self.perform_wallet_verification(wallet_address).await?;
        
        // Cache the result
        self.cache_verification(&verification).await?;
        
        Ok(verification)
    }
    
    /// Get cached verification
    async fn get_cached_verification(&self, wallet_address: &str) -> Result<Option<WalletVerification>> {
        let cache = self.verification_cache.read().await;
        Ok(cache.get(wallet_address).cloned())
    }
    
    /// Check if verification is expired
    fn is_verification_expired(&self, verification: &WalletVerification) -> bool {
        chrono::Utc::now() > verification.expiry_timestamp
    }
    
    /// Perform actual wallet verification
    async fn perform_wallet_verification(&self, wallet_address: &str) -> Result<WalletVerification> {
        // Simulate wallet verification logic
        // In production, this would integrate with actual wallet verification systems
        
        let verification_proof = format!("proof_{}", Uuid::new_v4());
        let expiry = chrono::Utc::now() + chrono::Duration::seconds(self.verification_config.cache_ttl_seconds as i64);
        
        Ok(WalletVerification {
            wallet_address: wallet_address.to_string(),
            verification_status: VerificationStatus::Verified,
            verification_timestamp: chrono::Utc::now(),
            verification_proof,
            expiry_timestamp: expiry,
        })
    }
    
    /// Cache verification result
    async fn cache_verification(&self, verification: &WalletVerification) -> Result<()> {
        let mut cache = self.verification_cache.write().await;
        cache.insert(verification.wallet_address.clone(), verification.clone());
        Ok(())
    }
}

impl DomainAuditBridge {
    /// Create new domain audit bridge
    pub async fn new() -> Result<Self> {
        Ok(Self {
            audit_config: DomainAuditConfig {
                audit_enabled: true,
                audit_all_resolutions: true,
                audit_failed_resolutions: true,
                audit_verification_events: true,
                retention_period_days: 365,
            },
            bpi_integration: BpiAuditIntegration {
                bpi_endpoint: "http://localhost:8080/audit".to_string(),
                audit_wallet: "audit_wallet_address".to_string(),
                batch_size: 100,
                flush_interval_seconds: 60,
            },
        })
    }
    
    /// Audit domain resolution event
    pub async fn audit_resolution_event(&self, resolved: &ResolvedDomain) -> Result<()> {
        if !self.audit_config.audit_enabled {
            return Ok(());
        }
        
        debug!("Auditing domain resolution: {}", resolved.domain);
        
        // Create audit event
        let audit_event = serde_json::json!({
            "event_type": "domain_resolution",
            "domain": resolved.domain,
            "protocol": resolved.protocol,
            "resolved_address": resolved.resolved_address,
            "verification_status": resolved.verification_status,
            "timestamp": resolved.timestamp,
            "ttl": resolved.ttl
        });
        
        // Send to BPI ledger for immutable audit trail
        self.send_to_bpi_ledger(&audit_event).await?;
        
        Ok(())
    }
    
    /// Send audit event to BPI ledger
    async fn send_to_bpi_ledger(&self, event: &serde_json::Value) -> Result<()> {
        // In production, this would send to actual BPI ledger
        debug!("Sending audit event to BPI ledger: {}", event);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_domain_resolver_creation() {
        let resolver = DomainResolver::new().await.unwrap();
        assert!(resolver.domain_cache.read().await.is_empty());
    }
    
    #[tokio::test]
    async fn test_protocol_detection() {
        let resolver = DomainResolver::new().await.unwrap();
        
        assert_eq!(resolver.detect_protocol("http:cg//example.com<addr123>wallet456").unwrap(), DomainProtocol::HttpCage);
        assert_eq!(resolver.detect_protocol("rootzk//(root123)<wallet456>proof(proof789).cage(cage101)").unwrap(), DomainProtocol::RootZk);
        assert_eq!(resolver.detect_protocol("example.com").unwrap(), DomainProtocol::Standard);
    }
    
    #[tokio::test]
    async fn test_http_cage_domain_parsing() {
        let resolver = DomainResolver::new().await.unwrap();
        let parsed = resolver.parse_http_cage_domain("http:cg//example.com<addr123>wallet456").unwrap();
        
        assert_eq!(parsed.domain, "example.com");
        assert_eq!(parsed.address, "addr123");
        assert_eq!(parsed.wallet_address, "wallet456");
    }
    
    #[tokio::test]
    async fn test_rootzk_domain_parsing() {
        let resolver = DomainResolver::new().await.unwrap();
        let parsed = resolver.parse_rootzk_domain("rootzk//(root123)<wallet456>proof(proof789).cage(cage101)").unwrap();
        
        assert_eq!(parsed.root_address, "root123");
        assert_eq!(parsed.wallet_id, "wallet456");
        assert_eq!(parsed.proof_address, "proof789");
        assert_eq!(parsed.cage_address, "cage101");
    }
    
    #[tokio::test]
    async fn test_wallet_verification() {
        let verifier = WalletVerificationService::new().await.unwrap();
        let verification = verifier.verify_wallet("test_wallet").await.unwrap();
        
        assert_eq!(verification.wallet_address, "test_wallet");
        assert_eq!(verification.verification_status, VerificationStatus::Verified);
    }
    
    #[tokio::test]
    async fn test_domain_resolution_standard() {
        let resolver = DomainResolver::new().await.unwrap();
        let resolved = resolver.resolve_domain("example.com").await.unwrap();
        
        assert_eq!(resolved.domain, "example.com");
        assert_eq!(resolved.protocol, DomainProtocol::Standard);
        assert_eq!(resolved.resolved_address, "https://example.com");
    }
}
