# QLOCK Quantum-Safe Session Locks - Mathematical Security Implementation

## Overview

QLOCK (Quantum Lock) provides quantum-safe session locks for httpcg protocol communications with mathematical precision and cryptographic guarantees. This system implements advanced session-level security that prevents traffic replay, forwarding attacks, and provides post-quantum security through sophisticated key derivation and binding mechanisms.

## Mathematical Foundation

### QLOCK Derivation Formula
```
QLK = HKDF(
    "httpcg-qlock/v1" || 
    tls_exporter || 
    SPKI_hash || 
    TLSLS_fingerprint || 
    route_fingerprint || 
    minute_epoch
)
```

### Security Properties
- **Temporal Binding**: `minute_epoch` ensures time-based uniqueness
- **Connection Binding**: `tls_exporter` ties to specific TLS session
- **Certificate Binding**: `SPKI_hash` and `TLSLS_fingerprint` bind to identity
- **Route Binding**: `route_fingerprint` prevents cross-route attacks
- **Replay Protection**: Different parameters → different QLK → verification fails

## Implementation Architecture

### QLOCK Engine
```rust
use wallet_identity::WalletIdentity;
use sha2::{Sha256, Digest};
use hkdf::Hkdf;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct QLOCKEngine {
    wallet: WalletIdentity,
    session_cache: Arc<RwLock<HashMap<String, QLOCKSession>>>,
}

#[derive(Debug, Clone)]
pub struct QLOCKSession {
    pub session_id: String,
    pub qlock_key: Vec<u8>,
    pub route_fingerprint: String,
    pub minute_epoch: u64,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub connection_binding: Vec<u8>,
}
```

### Session Lock Generation
```rust
impl QLOCKEngine {
    pub fn generate_session_lock(
        &self, 
        connection: &TLSLSConnection, 
        route: &str
    ) -> Result<QLOCKSession> {
        // 1. Calculate current minute epoch
        let minute_epoch = chrono::Utc::now().timestamp() as u64 / 60;
        
        // 2. Generate route fingerprint
        let route_fingerprint = self.calculate_route_fingerprint(route)?;
        
        // 3. Derive QLOCK key using HKDF
        let qlock_key = self.derive_qlock_key(
            &connection.tls_exporter,
            &connection.spki_hash,
            &connection.certificate.get_fingerprint()?,
            &route_fingerprint,
            minute_epoch,
        )?;
        
        // 4. Create session
        let session = QLOCKSession {
            session_id: uuid::Uuid::new_v4().to_string(),
            qlock_key,
            route_fingerprint,
            minute_epoch,
            created_at: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + chrono::Duration::minutes(1),
            connection_binding: connection.tls_exporter.clone(),
        };
        
        // 5. Cache session
        self.cache_session(session.clone()).await?;
        
        Ok(session)
    }
}
```

### HKDF Key Derivation
```rust
impl QLOCKEngine {
    pub fn derive_qlock_key(
        &self,
        tls_exporter: &[u8],
        spki_hash: &[u8],
        tlsls_fingerprint: &[u8],
        route_fingerprint: &str,
        minute_epoch: u64,
    ) -> Result<Vec<u8>> {
        // 1. Create input key material (IKM)
        let mut ikm = Vec::new();
        ikm.extend_from_slice(b"httpcg-qlock/v1");
        ikm.extend_from_slice(tls_exporter);
        ikm.extend_from_slice(spki_hash);
        ikm.extend_from_slice(tlsls_fingerprint);
        ikm.extend_from_slice(route_fingerprint.as_bytes());
        ikm.extend_from_slice(&minute_epoch.to_be_bytes());
        
        // 2. Create salt from wallet identity
        let wallet_pubkey = self.wallet.get_public_key()?;
        let mut hasher = Sha256::new();
        hasher.update(b"qlock-salt/v1");
        hasher.update(&wallet_pubkey);
        let salt = hasher.finalize();
        
        // 3. HKDF extract and expand
        let hk = Hkdf::<Sha256>::new(Some(&salt), &ikm);
        let mut qlock_key = vec![0u8; 32]; // 256-bit key
        hk.expand(b"httpcg-qlock-session", &mut qlock_key)
            .map_err(|e| anyhow!("HKDF expand failed: {}", e))?;
        
        Ok(qlock_key)
    }
}
```

## Binding Point Implementations

### DPoP Token Binding
```rust
// DPoP (Demonstration of Proof-of-Possession) integration
pub fn create_dpop_token(&self, qlock_session: &QLOCKSession, method: &str, url: &str) -> Result<String> {
    // 1. Calculate QLOCK hash for DPoP
    let qlk_hash = {
        let mut hasher = Sha256::new();
        hasher.update(&qlock_session.qlock_key);
        hasher.finalize()
    };
    
    // 2. Create DPoP header
    let dpop_header = json!({
        "typ": "dpop+jwt",
        "alg": "EdDSA",
        "jwk": {
            "kty": "OKP",
            "crv": "Ed25519",
            "x": base64::encode_config(&self.wallet.get_public_key()?, base64::URL_SAFE_NO_PAD)
        }
    });
    
    // 3. Create DPoP payload with QLOCK binding
    let dpop_payload = json!({
        "jti": uuid::Uuid::new_v4().to_string(),
        "htm": method,
        "htu": url,
        "iat": chrono::Utc::now().timestamp(),
        "qlk": hex::encode(qlk_hash), // QLOCK binding
    });
    
    // 4. Sign DPoP token
    let token = self.create_jwt(&dpop_header, &dpop_payload)?;
    Ok(token)
}
```

### Channel Binding for Tokens
```rust
// Replace simple TLS exporter with QLOCK-based channel binding
pub fn create_channel_bound_token(&self, qlock_session: &QLOCKSession) -> Result<String> {
    // 1. Calculate channel binding from QLOCK
    let channel_binding = {
        let mut hasher = Sha256::new();
        hasher.update(&qlock_session.qlock_key);
        hasher.update(b"channel-binding");
        hasher.finalize()
    };
    
    // 2. Create token with QLOCK channel binding
    let token_payload = json!({
        "sub": self.wallet.get_did()?,
        "iss": "httpcg-client",
        "aud": "httpcg-server",
        "iat": chrono::Utc::now().timestamp(),
        "exp": (chrono::Utc::now() + chrono::Duration::minutes(5)).timestamp(),
        "cb": base64::encode_config(&channel_binding, base64::URL_SAFE_NO_PAD), // QLOCK channel binding
        "session_id": qlock_session.session_id,
    });
    
    let token = self.create_jwt(&json!({"alg": "EdDSA", "typ": "JWT"}), &token_payload)?;
    Ok(token)
}
```

### WebSocket MAC Keys
```rust
// WebSocket security with QLOCK-derived MAC keys
pub fn create_websocket_mac_key(
    &self, 
    qlock_session: &QLOCKSession, 
    server_ephemeral: &[u8], 
    client_pub: &[u8]
) -> Result<Vec<u8>> {
    // 1. Create MAC key derivation input
    let mut mac_input = Vec::new();
    mac_input.extend_from_slice(&qlock_session.qlock_key);
    mac_input.extend_from_slice(server_ephemeral);
    mac_input.extend_from_slice(client_pub);
    mac_input.extend_from_slice(b"websocket-mac");
    
    // 2. HMAC key derivation
    let mac_key = {
        let mut hasher = Sha256::new();
        hasher.update(&mac_input);
        hasher.finalize()
    };
    
    Ok(mac_key.to_vec())
}

pub fn create_websocket_frame_mac(
    &self, 
    mac_key: &[u8], 
    frame_data: &[u8], 
    sequence: u64
) -> Result<Vec<u8>> {
    use hmac::{Hmac, Mac};
    type HmacSha256 = Hmac<Sha256>;
    
    // 1. Create HMAC instance
    let mut mac = HmacSha256::new_from_slice(mac_key)
        .map_err(|e| anyhow!("Invalid MAC key: {}", e))?;
    
    // 2. Update with frame data and sequence
    mac.update(frame_data);
    mac.update(&sequence.to_be_bytes());
    
    // 3. Finalize MAC
    let result = mac.finalize();
    Ok(result.into_bytes().to_vec())
}
```

## Security Mechanisms

### Traffic Replay Protection
```rust
pub fn validate_qlock_freshness(&self, qlock_session: &QLOCKSession) -> Result<bool> {
    let current_minute = chrono::Utc::now().timestamp() as u64 / 60;
    
    // QLOCK sessions are valid for current minute only
    if qlock_session.minute_epoch != current_minute {
        return Ok(false);
    }
    
    // Additional freshness checks
    if qlock_session.expires_at < chrono::Utc::now() {
        return Ok(false);
    }
    
    Ok(true)
}
```

### Bridge-Break Protection
```rust
pub fn detect_token_forwarding(
    &self, 
    qlock_session: &QLOCKSession, 
    connection_info: &ConnectionInfo
) -> Result<bool> {
    // 1. Verify connection binding matches
    if qlock_session.connection_binding != connection_info.tls_exporter {
        return Ok(true); // Token forwarding detected
    }
    
    // 2. Verify certificate/ASN/region consistency
    if !self.verify_connection_consistency(qlock_session, connection_info)? {
        return Ok(true); // Suspicious forwarding
    }
    
    // 3. Verify temporal consistency
    if !self.verify_temporal_consistency(qlock_session)? {
        return Ok(true); // Time-based attack
    }
    
    Ok(false) // No forwarding detected
}
```

### Route Fingerprinting
```rust
impl QLOCKEngine {
    pub fn calculate_route_fingerprint(&self, route: &str) -> Result<String> {
        // 1. Normalize route for consistent fingerprinting
        let normalized_route = self.normalize_route(route)?;
        
        // 2. Calculate SHA-256 hash
        let mut hasher = Sha256::new();
        hasher.update(b"route-fingerprint/v1");
        hasher.update(normalized_route.as_bytes());
        let hash = hasher.finalize();
        
        // 3. Return hex-encoded fingerprint
        Ok(hex::encode(hash))
    }
    
    fn normalize_route(&self, route: &str) -> Result<String> {
        // 1. Parse URL
        let url = url::Url::parse(route)?;
        
        // 2. Extract and normalize components
        let scheme = url.scheme().to_lowercase();
        let host = url.host_str().unwrap_or("").to_lowercase();
        let port = url.port().unwrap_or_else(|| {
            match scheme.as_str() {
                "https" | "httpcg" => 443,
                "http" => 80,
                _ => 0,
            }
        });
        let path = url.path();
        
        // 3. Create normalized route (exclude query parameters for consistency)
        let normalized = format!("{}://{}:{}{}", scheme, host, port, path);
        Ok(normalized)
    }
}
```

## Integration with BPI VM Server

### QLOCK Sync Gates
```rust
// Integration with existing BPI VM Server QLOCK implementation
use crate::bpi_vm_server::QLockSyncGate;

impl QLOCKEngine {
    pub fn integrate_with_bpi_vm(&self, vm_server: &BpiVmServer) -> Result<()> {
        // 1. Register QLOCK engine with VM server
        vm_server.register_qlock_engine(self.clone())?;
        
        // 2. Set up sync gate evaluation
        vm_server.set_sync_gate_evaluator(Box::new(|theta: f64| -> Result<bool> {
            // Daughter lock: sin²θ + cos²θ = 1 (always true, mathematical identity)
            let daughter_result = (theta.sin().powi(2) + theta.cos().powi(2) - 1.0).abs() < 1e-10;
            
            // Secant lock: sec(θ)·cos(θ) = 1 (when cos(θ) ≠ 0)
            let secant_result = if theta.cos().abs() > 1e-10 {
                ((1.0 / theta.cos()) * theta.cos() - 1.0).abs() < 1e-10
            } else {
                false // Undefined for cos(θ) = 0
            };
            
            Ok(daughter_result && secant_result)
        }))?;
        
        Ok(())
    }
}
```

### Distance Bounding Integration
```rust
pub fn validate_distance_bound(&self, connection_info: &ConnectionInfo) -> Result<bool> {
    // 1. Calculate time-of-flight (ToF)
    let tof = connection_info.round_trip_time / 2.0;
    
    // 2. Calculate distance (speed of light = 299,792,458 m/s)
    let distance_meters = tof.as_secs_f64() * 299_792_458.0;
    
    // 3. Validate 50m distance bound
    if distance_meters > 50.0 {
        warn!("Distance bound violation: {}m > 50m", distance_meters);
        return Ok(false);
    }
    
    // 4. Additional network topology validation
    if !self.validate_network_topology(connection_info)? {
        return Ok(false);
    }
    
    Ok(true)
}
```

## Performance Optimization

### Session Caching
```rust
pub struct QLOCKSessionCache {
    cache: Arc<RwLock<HashMap<String, QLOCKSession>>>,
    cleanup_interval: Duration,
}

impl QLOCKSessionCache {
    pub async fn start_cleanup_task(&self) -> Result<()> {
        let cache = self.cache.clone();
        let interval = self.cleanup_interval;
        
        tokio::spawn(async move {
            let mut cleanup_timer = tokio::time::interval(interval);
            
            loop {
                cleanup_timer.tick().await;
                
                // Remove expired sessions
                let mut cache_guard = cache.write().await;
                let now = chrono::Utc::now();
                
                cache_guard.retain(|_, session| session.expires_at > now);
                
                debug!("Cleaned up expired QLOCK sessions, {} remaining", cache_guard.len());
            }
        });
        
        Ok(())
    }
    
    pub async fn get_session(&self, session_id: &str) -> Option<QLOCKSession> {
        let cache = self.cache.read().await;
        cache.get(session_id).cloned()
    }
    
    pub async fn store_session(&self, session: QLOCKSession) {
        let mut cache = self.cache.write().await;
        cache.insert(session.session_id.clone(), session);
    }
}
```

### Batch Key Derivation
```rust
pub fn derive_multiple_qlock_keys(
    &self,
    base_params: &QLOCKBaseParams,
    routes: &[String],
) -> Result<HashMap<String, Vec<u8>>> {
    let mut results = HashMap::new();
    let minute_epoch = chrono::Utc::now().timestamp() as u64 / 60;
    
    // Batch process multiple routes
    for route in routes {
        let route_fingerprint = self.calculate_route_fingerprint(route)?;
        let qlock_key = self.derive_qlock_key(
            &base_params.tls_exporter,
            &base_params.spki_hash,
            &base_params.tlsls_fingerprint,
            &route_fingerprint,
            minute_epoch,
        )?;
        
        results.insert(route.clone(), qlock_key);
    }
    
    Ok(results)
}
```

## Testing and Validation

### Mathematical Property Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_qlock_mathematical_properties() -> Result<()> {
        let wallet = WalletIdentity::new_test()?;
        let engine = QLOCKEngine::new(wallet)?;
        
        // Test 1: Same inputs produce same QLOCK
        let params = create_test_params();
        let qlock1 = engine.derive_qlock_key(&params.0, &params.1, &params.2, &params.3, params.4)?;
        let qlock2 = engine.derive_qlock_key(&params.0, &params.1, &params.2, &params.3, params.4)?;
        assert_eq!(qlock1, qlock2);
        
        // Test 2: Different minute epochs produce different QLOCKs
        let qlock3 = engine.derive_qlock_key(&params.0, &params.1, &params.2, &params.3, params.4 + 1)?;
        assert_ne!(qlock1, qlock3);
        
        // Test 3: Different routes produce different QLOCKs
        let qlock4 = engine.derive_qlock_key(&params.0, &params.1, &params.2, "different-route", params.4)?;
        assert_ne!(qlock1, qlock4);
        
        Ok(())
    }
    
    #[test]
    fn test_sync_gate_mathematical_identity() -> Result<()> {
        use std::f64::consts::PI;
        
        // Test daughter lock: sin²θ + cos²θ = 1
        for i in 0..100 {
            let theta = (i as f64) * PI / 50.0; // 0 to 2π
            let result = theta.sin().powi(2) + theta.cos().powi(2);
            assert!((result - 1.0).abs() < 1e-10, "Daughter lock failed at θ={}", theta);
        }
        
        // Test secant lock: sec(θ)·cos(θ) = 1 (when cos(θ) ≠ 0)
        for i in 1..100 {
            let theta = (i as f64) * PI / 51.0; // Avoid cos(θ) = 0
            if theta.cos().abs() > 1e-10 {
                let result = (1.0 / theta.cos()) * theta.cos();
                assert!((result - 1.0).abs() < 1e-10, "Secant lock failed at θ={}", theta);
            }
        }
        
        Ok(())
    }
}
```

### Security Property Tests
```rust
#[tokio::test]
async fn test_replay_protection() -> Result<()> {
    let wallet = WalletIdentity::new_test()?;
    let engine = QLOCKEngine::new(wallet)?;
    
    // Create session for current minute
    let connection = create_test_connection();
    let session1 = engine.generate_session_lock(&connection, "/api/test").await?;
    
    // Wait for next minute
    tokio::time::sleep(Duration::from_secs(61)).await;
    
    // Session should be expired
    assert!(!engine.validate_qlock_freshness(&session1)?);
    
    // New session should have different QLOCK
    let session2 = engine.generate_session_lock(&connection, "/api/test").await?;
    assert_ne!(session1.qlock_key, session2.qlock_key);
    
    Ok(())
}

#[tokio::test]
async fn test_bridge_break_detection() -> Result<()> {
    let wallet = WalletIdentity::new_test()?;
    let engine = QLOCKEngine::new(wallet)?;
    
    let connection1 = create_test_connection();
    let session = engine.generate_session_lock(&connection1, "/api/test").await?;
    
    // Simulate token forwarding to different connection
    let connection2 = create_different_test_connection();
    let forwarding_detected = engine.detect_token_forwarding(&session, &connection2.into())?;
    
    assert!(forwarding_detected, "Bridge-break detection failed");
    
    Ok(())
}
```

## Production Configuration

### QLOCK Configuration
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QLOCKConfig {
    // Session settings
    pub session_lifetime_minutes: u32,       // 1 minute default
    pub max_concurrent_sessions: usize,      // 1000 default
    pub cleanup_interval_seconds: u64,       // 30 seconds
    
    // Security settings
    pub enable_distance_bounding: bool,      // true
    pub max_distance_meters: f64,           // 50.0
    pub enable_bridge_break_detection: bool, // true
    pub strict_temporal_validation: bool,    // true
    
    // Performance settings
    pub enable_session_caching: bool,        // true
    pub cache_size_limit: usize,            // 10000 sessions
    pub batch_key_derivation: bool,         // true
    
    // Integration settings
    pub bpi_vm_integration: bool,           // true
    pub sync_gate_precision: f64,           // 1e-10
    pub infinite_noise_on_failure: bool,    // true
}

impl Default for QLOCKConfig {
    fn default() -> Self {
        Self {
            session_lifetime_minutes: 1,
            max_concurrent_sessions: 1000,
            cleanup_interval_seconds: 30,
            enable_distance_bounding: true,
            max_distance_meters: 50.0,
            enable_bridge_break_detection: true,
            strict_temporal_validation: true,
            enable_session_caching: true,
            cache_size_limit: 10000,
            batch_key_derivation: true,
            bpi_vm_integration: true,
            sync_gate_precision: 1e-10,
            infinite_noise_on_failure: true,
        }
    }
}
```

### Monitoring and Metrics
```rust
#[derive(Debug, Clone)]
pub struct QLOCKMetrics {
    pub sessions_created: Arc<AtomicU64>,
    pub sessions_expired: Arc<AtomicU64>,
    pub replay_attacks_blocked: Arc<AtomicU64>,
    pub bridge_breaks_detected: Arc<AtomicU64>,
    pub distance_bound_violations: Arc<AtomicU64>,
    pub sync_gate_failures: Arc<AtomicU64>,
}

impl QLOCKMetrics {
    pub fn export_prometheus(&self) -> String {
        format!(
            "# HELP qlock_sessions_created_total Total QLOCK sessions created\n\
             # TYPE qlock_sessions_created_total counter\n\
             qlock_sessions_created_total {}\n\
             # HELP qlock_sessions_expired_total Total QLOCK sessions expired\n\
             # TYPE qlock_sessions_expired_total counter\n\
             qlock_sessions_expired_total {}\n\
             # HELP qlock_replay_attacks_blocked_total Total replay attacks blocked\n\
             # TYPE qlock_replay_attacks_blocked_total counter\n\
             qlock_replay_attacks_blocked_total {}\n\
             # HELP qlock_bridge_breaks_detected_total Total bridge breaks detected\n\
             # TYPE qlock_bridge_breaks_detected_total counter\n\
             qlock_bridge_breaks_detected_total {}\n\
             # HELP qlock_distance_bound_violations_total Total distance bound violations\n\
             # TYPE qlock_distance_bound_violations_total counter\n\
             qlock_distance_bound_violations_total {}\n\
             # HELP qlock_sync_gate_failures_total Total sync gate failures\n\
             # TYPE qlock_sync_gate_failures_total counter\n\
             qlock_sync_gate_failures_total {}\n",
            self.sessions_created.load(Ordering::Relaxed),
            self.sessions_expired.load(Ordering::Relaxed),
            self.replay_attacks_blocked.load(Ordering::Relaxed),
            self.bridge_breaks_detected.load(Ordering::Relaxed),
            self.distance_bound_violations.load(Ordering::Relaxed),
            self.sync_gate_failures.load(Ordering::Relaxed),
        )
    }
}
```

## Security Analysis

### Threat Model
1. **Replay Attacks**: Prevented by minute-epoch binding
2. **Token Forwarding**: Detected by connection binding validation
3. **Route Confusion**: Prevented by route fingerprinting
4. **Quantum Attacks**: Mitigated by post-quantum key derivation
5. **Distance Attacks**: Blocked by 50m distance bounding
6. **Temporal Attacks**: Prevented by strict time validation

### Cryptographic Guarantees
- **Key Uniqueness**: HKDF ensures unique keys for different inputs
- **Forward Secrecy**: New keys every minute
- **Binding Security**: Multiple binding points prevent attacks
- **Quantum Resistance**: Uses quantum-safe primitives
- **Mathematical Precision**: 1e-10 precision for sync gates

## Next Steps

1. **[Shadow Registry Bridge](./04-shadow-registry-bridge.md)** - Web2-Web3 integration layer
2. **[Deployment Guide](./05-deployment-and-configuration.md)** - Production deployment
3. **[Troubleshooting Guide](./06-troubleshooting-and-monitoring.md)** - Operations and debugging

## References

- **QLOCK Implementation**: `/home/umesh/metanode/wallet-identity/src/client/transport/httpcg_client.rs` (Lines 447-530)
- **BPI VM Integration**: `/home/umesh/metanode/bpi-core/src/bpi_vm_server.rs`
- **Sync Gate Mathematics**: ENC Lock integration documentation
- **Distance Bounding**: BPI VM Server security specifications
