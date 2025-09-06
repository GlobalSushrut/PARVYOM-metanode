# Shadow Registry Bridge - Web2-Web3 Communication Gateway

## Overview

The Shadow Registry Bridge provides seamless Web2-Web3 communication by acting as a deterministic resolver from `httpcg://` URLs to `https://` endpoints while preserving all httpcg security guarantees. This system enables transparent integration with existing Web2 applications while providing enhanced security, audit trails, and blockchain anchoring.

## Architecture Components

### Core Infrastructure
- **Shadow Registry Client**: URL resolution and caching
- **Web2 API Gateway**: Rate limiting and security policies
- **TLSLS Integration**: Identity-bound transport security
- **BPI Anchoring**: Blockchain-based record validation
- **RBAC Profiles**: Role-based access control

### Bridge Components
```rust
use crate::shadow_registry_bridge::ShadowRegistryBridge;
use wallet_identity::WalletIdentity;

#[derive(Debug, Clone)]
pub struct ShadowRegistryBridge {
    registry_cache: Arc<RwLock<HashMap<String, ShadowRegistryRecord>>>,
    gateway_endpoints: Vec<String>,
    wallet: WalletIdentity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowRegistryRecord {
    pub httpcg_url: String,           // Original httpcg:// URL
    pub https_mapping: String,        // Mapped https:// URL
    pub rp_did: String,              // Relying Party DID
    pub tlsls_requirements: TLSLSRequirements,
    pub rbac_profiles: Vec<String>,   // Access control profiles
    pub bpi_anchors: Vec<String>,     // BPI ledger anchors
    pub created_at: u64,             // Creation timestamp
    pub expires_at: u64,             // Expiration timestamp
    pub signature: String,           // Cryptographic signature
}
```

## URL Resolution Process

### httpcg to HTTPS Mapping
```rust
impl ShadowRegistryBridge {
    pub async fn resolve_httpcg(&self, httpcg_url: &HttpcgUrl) -> Result<ShadowRegistryRecord> {
        let cache_key = httpcg_url.to_string();
        
        // 1. Check cache first
        {
            let cache = self.registry_cache.read().await;
            if let Some(record) = cache.get(&cache_key) {
                if record.expires_at > chrono::Utc::now().timestamp() as u64 {
                    return Ok(record.clone());
                }
            }
        }
        
        // 2. Fetch from registry
        let record = self.fetch_from_registry(httpcg_url).await?;
        
        // 3. Verify record signature
        self.verify_record_signature(&record)?;
        
        // 4. Cache record
        {
            let mut cache = self.registry_cache.write().await;
            cache.insert(cache_key, record.clone());
        }
        
        Ok(record)
    }
    
    async fn fetch_from_registry(&self, httpcg_url: &HttpcgUrl) -> Result<ShadowRegistryRecord> {
        // Try multiple gateway endpoints for redundancy
        for endpoint in &self.gateway_endpoints {
            match self.try_fetch_from_endpoint(endpoint, httpcg_url).await {
                Ok(record) => return Ok(record),
                Err(e) => {
                    warn!("Failed to fetch from {}: {}", endpoint, e);
                    continue;
                }
            }
        }
        
        Err(anyhow!("Failed to resolve httpcg URL from all endpoints"))
    }
}
```

### Registry Record Structure
```rust
// Example Shadow Registry record for httpcg://app/api.example.com/users
{
    "httpcg_url": "httpcg://app/api.example.com/users",
    "https_mapping": "https://api.example.com/users",
    "rp_did": "did:bpi:api.example.com",
    "tlsls_requirements": {
        "required": true,
        "min_version": "1.3",
        "cipher_suites": ["TLS_AES_256_GCM_SHA384"],
        "certificate_transparency": true,
        "policy_hash": "sha256:abc123..."
    },
    "rbac_profiles": ["public_api", "authenticated_users"],
    "bpi_anchors": ["bpi:tx:def456..."],
    "created_at": 1703001600,
    "expires_at": 1703088000,
    "signature": "ed25519:signature_bytes..."
}
```

## Web2 Compatibility Layer

### Transparent Proxy Mode
```rust
pub struct Web2CompatibilityLayer {
    shadow_bridge: Arc<ShadowRegistryBridge>,
    api_gateway: Arc<Web2ApiGateway>,
    tlsls_manager: Arc<TLSLSManager>,
}

impl Web2CompatibilityLayer {
    pub async fn handle_web2_request(&self, request: &HttpRequest) -> Result<HttpResponse> {
        // 1. Convert HTTP request to httpcg
        let httpcg_url = self.convert_to_httpcg(&request.url)?;
        
        // 2. Resolve through Shadow Registry
        let registry_record = self.shadow_bridge.resolve_httpcg(&httpcg_url).await?;
        
        // 3. Apply security enhancements
        let enhanced_request = self.enhance_request(request, &registry_record)?;
        
        // 4. Forward to target with TLSLS if required
        let response = if registry_record.tlsls_requirements.required {
            self.send_tlsls_request(&enhanced_request, &registry_record).await?
        } else {
            self.send_https_request(&enhanced_request, &registry_record).await?
        };
        
        // 5. Add Pravyom headers to response
        let enhanced_response = self.enhance_response(response, &registry_record)?;
        
        Ok(enhanced_response)
    }
    
    fn enhance_request(&self, request: &HttpRequest, record: &ShadowRegistryRecord) -> Result<HttpRequest> {
        let mut enhanced = request.clone();
        
        // Add SAPI (Secure API) headers
        enhanced.headers.insert("X-Pravyom-Gateway".to_string(), "true".to_string());
        enhanced.headers.insert("X-BPI-Anchor".to_string(), record.bpi_anchors[0].clone());
        enhanced.headers.insert("X-Registry-DID".to_string(), record.rp_did.clone());
        
        // Add authentication if available
        if let Ok(auth_header) = self.generate_gateway_auth() {
            enhanced.headers.insert("Authorization".to_string(), auth_header);
        }
        
        Ok(enhanced)
    }
}
```

### Progressive Enhancement
```rust
// JavaScript SDK for Web2 applications
pub fn generate_pravyom_js_sdk() -> String {
    r#"
// Pravyom.js - Progressive Enhancement SDK
(function(window) {
    'use strict';
    
    const Pravyom = {
        // Initialize Pravyom enhancement
        init: function(config = {}) {
            this.config = {
                gateway: config.gateway || 'https://gateway.pravyom.com',
                autoEnhance: config.autoEnhance !== false,
                walletIntegration: config.walletIntegration || false,
                ...config
            };
            
            if (this.config.autoEnhance) {
                this.enhanceExistingRequests();
            }
            
            return this;
        },
        
        // Enhance existing fetch/XMLHttpRequest
        enhanceExistingRequests: function() {
            const originalFetch = window.fetch;
            
            window.fetch = async function(url, options = {}) {
                // Add Pravyom headers
                options.headers = {
                    'X-Pravyom-Enhanced': 'true',
                    'X-Pravyom-Version': '1.0',
                    ...options.headers
                };
                
                // Route through Pravyom gateway if configured
                if (Pravyom.config.gateway && !url.startsWith('http')) {
                    url = `${Pravyom.config.gateway}/proxy?url=${encodeURIComponent(url)}`;
                }
                
                return originalFetch(url, options);
            };
        },
        
        // Wallet integration
        connectWallet: async function() {
            if (!this.config.walletIntegration) {
                throw new Error('Wallet integration not enabled');
            }
            
            // Implementation for wallet connection
            return { address: 'wallet_address', did: 'did:bpi:wallet...' };
        },
        
        // Make httpcg request
        httpcg: async function(url, options = {}) {
            const httpcgUrl = url.startsWith('httpcg://') ? url : `httpcg://app/${url}`;
            
            return fetch(`${this.config.gateway}/httpcg?url=${encodeURIComponent(httpcgUrl)}`, {
                method: options.method || 'GET',
                headers: {
                    'Content-Type': 'application/json',
                    'X-Pravyom-Protocol': 'httpcg',
                    ...options.headers
                },
                body: options.body
            });
        }
    };
    
    // Auto-initialize if script has data-auto-init
    if (document.currentScript && document.currentScript.hasAttribute('data-auto-init')) {
        document.addEventListener('DOMContentLoaded', () => {
            Pravyom.init();
        });
    }
    
    window.Pravyom = Pravyom;
})(window);
"#.to_string()
}
```

## Security Integration

### RBAC Profile Enforcement
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RBACProfile {
    pub profile_name: String,
    pub allowed_methods: Vec<String>,
    pub allowed_paths: Vec<String>,
    pub required_roles: Vec<String>,
    pub rate_limits: RateLimitConfig,
    pub security_level: SecurityLevel,
}

impl Web2ApiGateway {
    pub async fn enforce_rbac(&self, request: &HttpRequest, profiles: &[String]) -> Result<bool> {
        for profile_name in profiles {
            let profile = self.get_rbac_profile(profile_name)?;
            
            // Check method authorization
            if !profile.allowed_methods.contains(&request.method) {
                return Ok(false);
            }
            
            // Check path authorization
            if !self.path_matches_any(&request.path, &profile.allowed_paths)? {
                return Ok(false);
            }
            
            // Check rate limits
            if !self.check_rate_limit(&request.client_id, &profile.rate_limits).await? {
                return Ok(false);
            }
            
            // Check security level
            if !self.validate_security_level(request, &profile.security_level)? {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
}
```

### BPI Anchoring Verification
```rust
impl ShadowRegistryBridge {
    pub async fn verify_bpi_anchors(&self, anchors: &[String]) -> Result<bool> {
        for anchor in anchors {
            // Parse BPI anchor format: bpi:tx:transaction_hash
            let parts: Vec<&str> = anchor.split(':').collect();
            if parts.len() != 3 || parts[0] != "bpi" || parts[1] != "tx" {
                return Ok(false);
            }
            
            let tx_hash = parts[2];
            
            // Verify transaction exists on BPI ledger
            match self.bpi_client.get_transaction(tx_hash).await {
                Ok(Some(tx)) => {
                    // Verify transaction type and content
                    if tx.transaction_type != TransactionType::RegistryAnchor {
                        return Ok(false);
                    }
                },
                Ok(None) => return Ok(false),
                Err(_) => return Ok(false),
            }
        }
        
        Ok(true)
    }
}
```

## Gateway Behavior Implementation

### HTTPS Ingress with TLSLS Stapling
```rust
pub struct HttpsIngressGateway {
    shadow_bridge: Arc<ShadowRegistryBridge>,
    tlsls_manager: Arc<TLSLSManager>,
    certificate_store: Arc<RwLock<HashMap<String, TLSLSCertificate>>>,
}

impl HttpsIngressGateway {
    pub async fn handle_https_request(&self, request: HttpRequest) -> Result<HttpResponse> {
        // 1. Extract host from request
        let host = request.headers.get("Host")
            .ok_or_else(|| anyhow!("Missing Host header"))?;
        
        // 2. Convert to httpcg URL
        let httpcg_url = HttpcgUrl {
            scheme: "httpcg".to_string(),
            app_id: "app".to_string(),
            domain: host.clone(),
            path: request.path.clone(),
            query: request.query.clone(),
        };
        
        // 3. Resolve through Shadow Registry
        let registry_record = self.shadow_bridge.resolve_httpcg(&httpcg_url).await?;
        
        // 4. Enforce TLSLS requirements
        if registry_record.tlsls_requirements.required {
            self.enforce_tlsls_requirements(&request, &registry_record)?;
        }
        
        // 5. Apply DPoP and QLOCK validation
        self.validate_dpop_and_qlock(&request)?;
        
        // 6. Forward to internal httpcg handler
        self.forward_to_httpcg_handler(&request, &registry_record).await
    }
    
    fn enforce_tlsls_requirements(
        &self, 
        request: &HttpRequest, 
        record: &ShadowRegistryRecord
    ) -> Result<()> {
        // Verify TLSLS certificate is present and valid
        let tlsls_header = request.headers.get("X-TLSLS-Certificate")
            .ok_or_else(|| anyhow!("TLSLS certificate required but not provided"))?;
        
        let certificate: TLSLSCertificate = serde_cbor::from_slice(
            &base64::decode(tlsls_header)?
        )?;
        
        // Verify certificate matches requirements
        if !self.tlsls_manager.verify_certificate(&certificate)? {
            return Err(anyhow!("Invalid TLSLS certificate"));
        }
        
        // Verify policy hash matches
        if certificate.policy_hash != record.tlsls_requirements.policy_hash.as_ref().unwrap_or(&String::new()) {
            return Err(anyhow!("Policy hash mismatch"));
        }
        
        Ok(())
    }
}
```

### Policy Profile Mapping
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyProfile {
    pub profile_id: String,
    pub security_requirements: SecurityRequirements,
    pub access_controls: AccessControls,
    pub audit_requirements: AuditRequirements,
    pub compliance_level: ComplianceLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequirements {
    pub min_tls_version: String,
    pub require_client_certificates: bool,
    pub allowed_cipher_suites: Vec<String>,
    pub require_perfect_forward_secrecy: bool,
    pub max_request_size: usize,
}

impl ShadowRegistryBridge {
    pub fn map_policy_profile(&self, record: &ShadowRegistryRecord) -> Result<PolicyProfile> {
        // Create policy profile from registry record
        let profile = PolicyProfile {
            profile_id: format!("profile_{}", record.rp_did),
            security_requirements: SecurityRequirements {
                min_tls_version: record.tlsls_requirements.min_version.clone(),
                require_client_certificates: record.tlsls_requirements.required,
                allowed_cipher_suites: record.tlsls_requirements.cipher_suites.clone(),
                require_perfect_forward_secrecy: true,
                max_request_size: 10 * 1024 * 1024, // 10MB default
            },
            access_controls: self.create_access_controls(&record.rbac_profiles)?,
            audit_requirements: AuditRequirements {
                log_all_requests: true,
                require_audit_trail: true,
                retention_days: 90,
            },
            compliance_level: ComplianceLevel::Standard,
        };
        
        Ok(profile)
    }
}
```

## Performance and Scalability

### Caching Strategy
```rust
pub struct ShadowRegistryCache {
    memory_cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    persistent_cache: Option<Arc<dyn PersistentCache>>,
    cache_config: CacheConfig,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    record: ShadowRegistryRecord,
    cached_at: Instant,
    access_count: AtomicU64,
    last_accessed: AtomicU64,
}

impl ShadowRegistryCache {
    pub async fn get(&self, key: &str) -> Option<ShadowRegistryRecord> {
        // 1. Check memory cache
        {
            let cache = self.memory_cache.read().await;
            if let Some(entry) = cache.get(key) {
                // Update access statistics
                entry.access_count.fetch_add(1, Ordering::Relaxed);
                entry.last_accessed.store(
                    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                    Ordering::Relaxed
                );
                
                // Check if still valid
                if entry.cached_at.elapsed() < self.cache_config.ttl {
                    return Some(entry.record.clone());
                }
            }
        }
        
        // 2. Check persistent cache if available
        if let Some(persistent) = &self.persistent_cache {
            if let Ok(Some(record)) = persistent.get(key).await {
                // Update memory cache
                self.put(key.to_string(), record.clone()).await;
                return Some(record);
            }
        }
        
        None
    }
    
    pub async fn put(&self, key: String, record: ShadowRegistryRecord) {
        let entry = CacheEntry {
            record: record.clone(),
            cached_at: Instant::now(),
            access_count: AtomicU64::new(0),
            last_accessed: AtomicU64::new(
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
            ),
        };
        
        // Update memory cache
        {
            let mut cache = self.memory_cache.write().await;
            cache.insert(key.clone(), entry);
            
            // Evict if over size limit
            if cache.len() > self.cache_config.max_entries {
                self.evict_lru(&mut cache);
            }
        }
        
        // Update persistent cache
        if let Some(persistent) = &self.persistent_cache {
            let _ = persistent.put(&key, &record).await;
        }
    }
}
```

### Load Balancing
```rust
pub struct GatewayLoadBalancer {
    endpoints: Vec<GatewayEndpoint>,
    health_checker: Arc<HealthChecker>,
    load_balancer_strategy: LoadBalancerStrategy,
}

#[derive(Debug, Clone)]
struct GatewayEndpoint {
    url: String,
    weight: u32,
    current_connections: AtomicU32,
    response_time_ms: AtomicU64,
    error_rate: AtomicU32,
    last_health_check: AtomicU64,
}

impl GatewayLoadBalancer {
    pub async fn select_endpoint(&self) -> Result<&GatewayEndpoint> {
        let healthy_endpoints: Vec<&GatewayEndpoint> = self.endpoints
            .iter()
            .filter(|ep| self.health_checker.is_healthy(ep))
            .collect();
        
        if healthy_endpoints.is_empty() {
            return Err(anyhow!("No healthy endpoints available"));
        }
        
        match self.load_balancer_strategy {
            LoadBalancerStrategy::RoundRobin => {
                // Simple round-robin implementation
                let index = self.get_next_index() % healthy_endpoints.len();
                Ok(healthy_endpoints[index])
            },
            LoadBalancerStrategy::WeightedRoundRobin => {
                // Weighted selection based on endpoint weights
                self.select_weighted_endpoint(&healthy_endpoints)
            },
            LoadBalancerStrategy::LeastConnections => {
                // Select endpoint with least active connections
                healthy_endpoints.iter()
                    .min_by_key(|ep| ep.current_connections.load(Ordering::Relaxed))
                    .copied()
                    .ok_or_else(|| anyhow!("No endpoints available"))
            },
            LoadBalancerStrategy::ResponseTime => {
                // Select endpoint with best response time
                healthy_endpoints.iter()
                    .min_by_key(|ep| ep.response_time_ms.load(Ordering::Relaxed))
                    .copied()
                    .ok_or_else(|| anyhow!("No endpoints available"))
            }
        }
    }
}
```

## Monitoring and Observability

### Metrics Collection
```rust
#[derive(Debug, Clone)]
pub struct ShadowRegistryMetrics {
    pub requests_total: Arc<AtomicU64>,
    pub cache_hits: Arc<AtomicU64>,
    pub cache_misses: Arc<AtomicU64>,
    pub resolution_time_ms: Arc<AtomicU64>,
    pub errors_total: Arc<AtomicU64>,
    pub bpi_anchor_verifications: Arc<AtomicU64>,
    pub tlsls_enforcements: Arc<AtomicU64>,
}

impl ShadowRegistryMetrics {
    pub fn record_request(&self, duration_ms: u64, cache_hit: bool, error: bool) {
        self.requests_total.fetch_add(1, Ordering::Relaxed);
        self.resolution_time_ms.store(duration_ms, Ordering::Relaxed);
        
        if cache_hit {
            self.cache_hits.fetch_add(1, Ordering::Relaxed);
        } else {
            self.cache_misses.fetch_add(1, Ordering::Relaxed);
        }
        
        if error {
            self.errors_total.fetch_add(1, Ordering::Relaxed);
        }
    }
    
    pub fn export_prometheus(&self) -> String {
        format!(
            "# HELP shadow_registry_requests_total Total requests processed\n\
             # TYPE shadow_registry_requests_total counter\n\
             shadow_registry_requests_total {}\n\
             # HELP shadow_registry_cache_hits_total Cache hits\n\
             # TYPE shadow_registry_cache_hits_total counter\n\
             shadow_registry_cache_hits_total {}\n\
             # HELP shadow_registry_resolution_time_ms Resolution time in milliseconds\n\
             # TYPE shadow_registry_resolution_time_ms gauge\n\
             shadow_registry_resolution_time_ms {}\n",
            self.requests_total.load(Ordering::Relaxed),
            self.cache_hits.load(Ordering::Relaxed),
            self.resolution_time_ms.load(Ordering::Relaxed),
        )
    }
}
```

### Health Monitoring
```rust
pub struct ShadowRegistryHealthCheck {
    endpoints: Vec<String>,
    check_interval: Duration,
    timeout: Duration,
}

impl ShadowRegistryHealthCheck {
    pub async fn start_health_monitoring(&self) -> Result<()> {
        let endpoints = self.endpoints.clone();
        let interval = self.check_interval;
        let timeout = self.timeout;
        
        tokio::spawn(async move {
            let mut check_timer = tokio::time::interval(interval);
            
            loop {
                check_timer.tick().await;
                
                for endpoint in &endpoints {
                    let health_result = tokio::time::timeout(
                        timeout,
                        Self::check_endpoint_health(endpoint)
                    ).await;
                    
                    match health_result {
                        Ok(Ok(healthy)) => {
                            if healthy {
                                info!("Endpoint {} is healthy", endpoint);
                            } else {
                                warn!("Endpoint {} is unhealthy", endpoint);
                            }
                        },
                        Ok(Err(e)) => {
                            error!("Health check error for {}: {}", endpoint, e);
                        },
                        Err(_) => {
                            error!("Health check timeout for {}", endpoint);
                        }
                    }
                }
            }
        });
        
        Ok(())
    }
    
    async fn check_endpoint_health(endpoint: &str) -> Result<bool> {
        let health_url = format!("{}/health", endpoint);
        let response = reqwest::get(&health_url).await?;
        Ok(response.status().is_success())
    }
}
```

## Testing and Validation

### Integration Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_shadow_registry_resolution() -> Result<()> {
        let wallet = WalletIdentity::new_test()?;
        let bridge = ShadowRegistryBridge::new(wallet).await?;
        
        let httpcg_url = HttpcgUrl::parse("httpcg://app/api.example.com/users")?;
        let record = bridge.resolve_httpcg(&httpcg_url).await?;
        
        assert_eq!(record.httpcg_url, "httpcg://app/api.example.com/users");
        assert!(record.https_mapping.starts_with("https://"));
        assert!(!record.rp_did.is_empty());
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_web2_compatibility() -> Result<()> {
        let bridge = create_test_bridge().await?;
        let compat_layer = Web2CompatibilityLayer::new(bridge);
        
        let request = HttpRequest {
            method: "GET".to_string(),
            url: "https://api.example.com/users".to_string(),
            headers: HashMap::new(),
            body: None,
        };
        
        let response = compat_layer.handle_web2_request(&request).await?;
        
        assert_eq!(response.status_code, 200);
        assert!(response.headers.contains_key("X-Pravyom-Gateway"));
        
        Ok(())
    }
}
```

## Configuration Examples

### Production Configuration
```toml
[shadow_registry]
# Gateway endpoints for redundancy
gateway_endpoints = [
    "https://gateway1.pravyom.com",
    "https://gateway2.pravyom.com", 
    "https://gateway3.pravyom.com"
]

# Cache configuration
[shadow_registry.cache]
memory_cache_size = 10000
ttl_seconds = 300
persistent_cache_enabled = true
persistent_cache_path = "/var/cache/pravyom/shadow_registry"

# Security configuration
[shadow_registry.security]
require_bpi_anchoring = true
verify_signatures = true
max_record_age_hours = 24
allow_self_signed = false

# Performance configuration
[shadow_registry.performance]
max_concurrent_requests = 1000
request_timeout_seconds = 30
retry_attempts = 3
retry_backoff_ms = [100, 500, 2000]

# Web2 compatibility
[web2_compatibility]
enabled = true
auto_enhance_requests = true
progressive_enhancement = true
fallback_to_https = true

# Load balancing
[load_balancer]
strategy = "least_connections"
health_check_interval_seconds = 30
health_check_timeout_seconds = 5
```

## Next Steps

1. **[Deployment Guide](./05-deployment-and-configuration.md)** - Production deployment setup
2. **[Troubleshooting Guide](./06-troubleshooting-and-monitoring.md)** - Operations and debugging

## References

- **Shadow Registry Implementation**: `/home/umesh/metanode/wallet-identity/src/client/transport/httpcg_client.rs` (Lines 25-165)
- **Court-Shadow Bridge**: `/home/umesh/metanode/bpci-enterprise/src/court_shadow_bridge.rs`
- **BPI Integration**: `/home/umesh/metanode/bpi-core/src/bpi_ledger_integration.rs`
- **Web2 Compatibility**: Web2 Compatibility Guide documentation
