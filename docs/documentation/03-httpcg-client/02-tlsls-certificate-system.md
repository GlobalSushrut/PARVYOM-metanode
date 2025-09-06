# TLSLS Certificate System - Identity-Bound Transport Security

## Overview

The TLSLS (Transport Layer Security + Ledger Security) certificate system provides identity-bound transport security for httpcg protocol communications. This system combines traditional TLS with blockchain-anchored identity verification, post-quantum cryptography, and policy attestation to create military-grade secure communications.

## Architecture

### Core Components
- **TLSLSManager**: Certificate lifecycle management
- **TLSLSCertificate**: Identity-bound certificate structure
- **TLSLSConnection**: Secure connection with quantum-safe properties
- **BPI Anchoring**: Blockchain-based certificate validation
- **Policy Attestation**: CBOR-encoded policy verification

### Certificate Structure
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TLSLSCertificate {
    pub subject_did: String,           // DID-based subject identity
    pub issuer_did: String,           // Certificate authority DID
    pub public_key: Vec<u8>,          // Ed25519 public key
    pub pq_public_key: Vec<u8>,       // Dilithium5 post-quantum key
    pub policy_hash: String,          // Policy attestation hash
    pub bpi_anchor: String,           // BPI ledger anchor
    pub valid_from: u64,              // Unix timestamp
    pub valid_until: u64,             // Unix timestamp (≤90 days)
    pub signature: Vec<u8>,           // Hybrid signature
    pub certificate_id: String,       // Unique certificate ID
}
```

## Implementation Details

### TLSLS Manager
```rust
use wallet_identity::WalletIdentity;
use crate::client::transport::httpcg_client::TLSLSManager;

// Initialize TLSLS Manager
let wallet = WalletIdentity::new()?;
let tlsls_manager = TLSLSManager::new(wallet);

// Establish secure connection
let connection = tlsls_manager.establish_connection("api.example.com", 443)?;
println!("Connection established: {}", connection.is_valid());
```

### Certificate Creation Process
```rust
impl TLSLSManager {
    pub fn create_tlsls_certificate(&self, host: &str) -> Result<TLSLSCertificate> {
        // 1. Generate hybrid keypair (Ed25519 + Dilithium5)
        let ed25519_keypair = self.wallet.get_keypair()?;
        let pq_keypair = self.generate_pq_keypair()?;
        
        // 2. Create certificate structure
        let cert = TLSLSCertificate {
            subject_did: self.wallet.get_did()?,
            issuer_did: "did:bpi:certificate-authority".to_string(),
            public_key: ed25519_keypair.public.to_bytes().to_vec(),
            pq_public_key: pq_keypair.public_key_bytes(),
            policy_hash: self.calculate_policy_hash(host)?,
            bpi_anchor: self.create_bpi_anchor()?,
            valid_from: chrono::Utc::now().timestamp() as u64,
            valid_until: (chrono::Utc::now() + chrono::Duration::days(90)).timestamp() as u64,
            signature: Vec::new(), // Will be filled by signing
            certificate_id: uuid::Uuid::new_v4().to_string(),
        };
        
        // 3. Sign certificate with hybrid signature
        let signature = self.sign_certificate(&cert)?;
        let mut signed_cert = cert;
        signed_cert.signature = signature;
        
        // 4. Anchor to BPI ledger
        self.anchor_to_bpi(&signed_cert)?;
        
        Ok(signed_cert)
    }
}
```

### Connection Establishment
```rust
impl TLSLSManager {
    pub fn establish_connection(&self, host: &str, port: u16) -> Result<TLSLSConnection> {
        // 1. Get or create certificate for host
        let certificate = self.get_or_create_certificate(host)?;
        
        // 2. Establish TLS connection
        let tls_stream = self.connect_tls(host, port)?;
        
        // 3. Generate TLS exporter for QLOCK
        let tls_exporter = self.generate_tls_exporter(host, &certificate.certificate_id)?;
        
        // 4. Calculate SPKI hash
        let spki_hash = self.calculate_spki_hash(&certificate)?;
        
        // 5. Create TLSLS connection
        let connection = TLSLSConnection {
            host: host.to_string(),
            port,
            certificate,
            tls_exporter,
            spki_hash,
            connection_id: uuid::Uuid::new_v4().to_string(),
            established_at: std::time::Instant::now(),
            last_used: std::time::Instant::now(),
        };
        
        Ok(connection)
    }
}
```

## Cryptographic Implementation

### Hybrid Signature Scheme
```rust
// Ed25519 + Dilithium5 hybrid signatures
pub fn sign_certificate(&self, cert: &TLSLSCertificate) -> Result<Vec<u8>> {
    // 1. Serialize certificate for signing
    let cert_bytes = self.serialize_for_signing(cert)?;
    
    // 2. Ed25519 signature
    let ed25519_sig = self.wallet.sign(&cert_bytes)?;
    
    // 3. Dilithium5 signature
    let pq_sig = self.pq_sign(&cert_bytes)?;
    
    // 4. Combine signatures
    let hybrid_sig = HybridSignature {
        ed25519: ed25519_sig.to_bytes().to_vec(),
        dilithium5: pq_sig,
        algorithm: "Ed25519+Dilithium5".to_string(),
    };
    
    // 5. CBOR encode hybrid signature
    let signature_bytes = serde_cbor::to_vec(&hybrid_sig)?;
    Ok(signature_bytes)
}
```

### Certificate Verification
```rust
pub fn verify_certificate(&self, cert: &TLSLSCertificate) -> Result<bool> {
    // 1. Check validity period
    let now = chrono::Utc::now().timestamp() as u64;
    if now < cert.valid_from || now > cert.valid_until {
        return Ok(false);
    }
    
    // 2. Verify BPI anchor
    if !self.verify_bpi_anchor(&cert.bpi_anchor)? {
        return Ok(false);
    }
    
    // 3. Verify hybrid signature
    let cert_bytes = self.serialize_for_signing(cert)?;
    let hybrid_sig: HybridSignature = serde_cbor::from_slice(&cert.signature)?;
    
    // 4. Verify Ed25519 signature
    let ed25519_valid = self.verify_ed25519(&cert_bytes, &hybrid_sig.ed25519, &cert.public_key)?;
    
    // 5. Verify Dilithium5 signature
    let pq_valid = self.verify_dilithium5(&cert_bytes, &hybrid_sig.dilithium5, &cert.pq_public_key)?;
    
    Ok(ed25519_valid && pq_valid)
}
```

## BPI Ledger Integration

### Certificate Anchoring
```rust
pub fn create_bpi_anchor(&self) -> Result<String> {
    // 1. Create certificate hash
    let cert_hash = self.calculate_certificate_hash()?;
    
    // 2. Submit to BPI ledger
    let anchor_tx = BpiTransaction {
        transaction_type: TransactionType::CertificateAnchor,
        data: cert_hash,
        timestamp: chrono::Utc::now().timestamp() as u64,
        wallet_address: self.wallet.get_address()?,
    };
    
    // 3. Get transaction hash as anchor
    let tx_hash = self.bpi_client.submit_transaction(anchor_tx)?;
    Ok(tx_hash)
}

pub fn verify_bpi_anchor(&self, anchor: &str) -> Result<bool> {
    // 1. Query BPI ledger for anchor
    let tx_result = self.bpi_client.get_transaction(anchor)?;
    
    // 2. Verify transaction exists and is valid
    match tx_result {
        Some(tx) => {
            // Verify transaction type and data
            Ok(tx.transaction_type == TransactionType::CertificateAnchor)
        },
        None => Ok(false),
    }
}
```

### Policy Attestation
```rust
pub fn calculate_policy_hash(&self, host: &str) -> Result<String> {
    // 1. Get security policy for host
    let policy = SecurityPolicy {
        host: host.to_string(),
        require_tls_13: true,
        require_perfect_forward_secrecy: true,
        allowed_cipher_suites: vec![
            "TLS_AES_256_GCM_SHA384".to_string(),
            "TLS_CHACHA20_POLY1305_SHA256".to_string(),
        ],
        require_certificate_transparency: true,
        max_certificate_lifetime: 90 * 24 * 3600, // 90 days
        require_ocsp_stapling: true,
    };
    
    // 2. CBOR encode policy
    let policy_bytes = serde_cbor::to_vec(&policy)?;
    
    // 3. Calculate SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(&policy_bytes);
    let hash = hasher.finalize();
    
    Ok(hex::encode(hash))
}
```

## HTTPS Compatibility

### TLS Extension Stapling
```rust
pub fn create_tls_extension(&self, cert: &TLSLSCertificate) -> Result<Vec<u8>> {
    // 1. Create TLSLS extension data
    let extension_data = TLSLSExtension {
        certificate_id: cert.certificate_id.clone(),
        bpi_anchor: cert.bpi_anchor.clone(),
        policy_hash: cert.policy_hash.clone(),
        issuer_did: cert.issuer_did.clone(),
    };
    
    // 2. CBOR encode extension
    let extension_bytes = serde_cbor::to_vec(&extension_data)?;
    
    // 3. Create TLS extension format
    let tls_extension = TLSExtension {
        extension_type: 65001, // TLSLS extension type
        extension_data: extension_bytes,
    };
    
    Ok(serde_cbor::to_vec(&tls_extension)?)
}
```

### Fallback Support
```rust
pub fn send_https_request_with_tlsls(&self, url: &str, method: &str, body: Option<&[u8]>) -> Result<HttpcgResponse> {
    // 1. Parse URL and establish TLSLS connection
    let parsed_url = url::Url::parse(url)?;
    let host = parsed_url.host_str().ok_or_else(|| anyhow!("Invalid host"))?;
    let port = parsed_url.port().unwrap_or(443);
    
    let connection = self.establish_connection(host, port)?;
    
    // 2. Create HTTPS request with TLSLS extension
    let mut headers = HashMap::new();
    headers.insert("User-Agent".to_string(), "HttpCG-Client/1.0".to_string());
    headers.insert("Accept".to_string(), "application/json, */*".to_string());
    
    // 3. Add TLSLS authentication header
    let auth_header = self.generate_auth_header()?;
    headers.insert("Authorization".to_string(), format!("TLSLS {}", auth_header));
    
    // 4. Send request
    self.send_https_request(url, method, body, &headers)
}
```

## Configuration and Management

### Certificate Lifecycle
```rust
pub struct CertificateManager {
    certificates: Arc<RwLock<HashMap<String, TLSLSCertificate>>>,
    renewal_threshold: Duration, // 7 days before expiry
}

impl CertificateManager {
    pub async fn start_renewal_service(&self) -> Result<()> {
        let certificates = self.certificates.clone();
        let threshold = self.renewal_threshold;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_hours(6));
            
            loop {
                interval.tick().await;
                
                // Check for certificates nearing expiry
                let certs = certificates.read().await;
                let now = chrono::Utc::now().timestamp() as u64;
                
                for (host, cert) in certs.iter() {
                    let expires_in = cert.valid_until - now;
                    if expires_in < threshold.as_secs() {
                        // Schedule renewal
                        tokio::spawn(Self::renew_certificate(host.clone()));
                    }
                }
            }
        });
        
        Ok(())
    }
    
    async fn renew_certificate(host: String) -> Result<()> {
        // Implementation for certificate renewal
        println!("Renewing certificate for host: {}", host);
        Ok(())
    }
}
```

### Security Configuration
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TLSLSConfig {
    // Certificate settings
    pub certificate_lifetime_days: u32,      // 90 days max
    pub renewal_threshold_days: u32,         // 7 days before expiry
    pub auto_renewal: bool,                  // true
    
    // Cryptographic settings
    pub use_hybrid_signatures: bool,         // true
    pub require_pq_crypto: bool,            // true
    pub min_key_size: u32,                  // 256 bits for Ed25519
    
    // Policy settings
    pub require_policy_attestation: bool,    // true
    pub require_bpi_anchoring: bool,        // true
    pub allow_self_signed: bool,            // false
    
    // Network settings
    pub connection_timeout: Duration,        // 30 seconds
    pub handshake_timeout: Duration,        // 10 seconds
    pub max_retries: u32,                   // 3
}

impl Default for TLSLSConfig {
    fn default() -> Self {
        Self {
            certificate_lifetime_days: 90,
            renewal_threshold_days: 7,
            auto_renewal: true,
            use_hybrid_signatures: true,
            require_pq_crypto: true,
            min_key_size: 256,
            require_policy_attestation: true,
            require_bpi_anchoring: true,
            allow_self_signed: false,
            connection_timeout: Duration::from_secs(30),
            handshake_timeout: Duration::from_secs(10),
            max_retries: 3,
        }
    }
}
```

## Testing and Validation

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_certificate_creation() -> Result<()> {
        let wallet = WalletIdentity::new_test()?;
        let manager = TLSLSManager::new(wallet);
        
        let cert = manager.create_tlsls_certificate("test.example.com")?;
        
        assert!(!cert.certificate_id.is_empty());
        assert!(cert.valid_until > cert.valid_from);
        assert_eq!(cert.subject_did, manager.wallet.get_did()?);
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_certificate_verification() -> Result<()> {
        let wallet = WalletIdentity::new_test()?;
        let manager = TLSLSManager::new(wallet);
        
        let cert = manager.create_tlsls_certificate("test.example.com")?;
        let is_valid = manager.verify_certificate(&cert)?;
        
        assert!(is_valid);
        Ok(())
    }
    
    #[tokio::test]
    async fn test_connection_establishment() -> Result<()> {
        let wallet = WalletIdentity::new_test()?;
        let manager = TLSLSManager::new(wallet);
        
        // Note: This would require a test server
        // let connection = manager.establish_connection("localhost", 8443)?;
        // assert!(connection.is_valid());
        
        Ok(())
    }
}
```

### Integration Testing
```rust
// Integration test with real TLSLS server
#[tokio::test]
async fn test_tlsls_integration() -> Result<()> {
    // Start test TLSLS server
    let server = start_test_tlsls_server().await?;
    
    // Create client
    let wallet = WalletIdentity::new_test()?;
    let manager = TLSLSManager::new(wallet);
    
    // Test connection
    let connection = manager.establish_connection("localhost", server.port()).await?;
    assert!(connection.is_valid());
    
    // Test request
    let response = manager.send_https_request(
        &format!("https://localhost:{}/test", server.port()),
        "GET",
        None,
        &HashMap::new()
    ).await?;
    
    assert_eq!(response.status_code, 200);
    
    server.shutdown().await?;
    Ok(())
}
```

## Performance Optimization

### Connection Pooling
```rust
pub struct TLSLSConnectionPool {
    connections: Arc<RwLock<HashMap<String, Vec<TLSLSConnection>>>>,
    max_connections_per_host: usize,
    connection_timeout: Duration,
}

impl TLSLSConnectionPool {
    pub async fn get_connection(&self, host: &str, port: u16) -> Result<TLSLSConnection> {
        let key = format!("{}:{}", host, port);
        
        // Try to get existing connection
        {
            let mut connections = self.connections.write().await;
            if let Some(host_connections) = connections.get_mut(&key) {
                while let Some(conn) = host_connections.pop() {
                    if conn.is_valid() {
                        return Ok(conn);
                    }
                }
            }
        }
        
        // Create new connection
        let manager = TLSLSManager::new(self.wallet.clone());
        let connection = manager.establish_connection(host, port)?;
        
        Ok(connection)
    }
    
    pub async fn return_connection(&self, connection: TLSLSConnection) {
        let key = format!("{}:{}", connection.host, connection.port);
        
        let mut connections = self.connections.write().await;
        let host_connections = connections.entry(key).or_insert_with(Vec::new);
        
        if host_connections.len() < self.max_connections_per_host {
            host_connections.push(connection);
        }
    }
}
```

### Certificate Caching
```rust
pub struct CertificateCache {
    cache: Arc<RwLock<HashMap<String, (TLSLSCertificate, Instant)>>>,
    cache_duration: Duration,
}

impl CertificateCache {
    pub async fn get_certificate(&self, host: &str) -> Option<TLSLSCertificate> {
        let cache = self.cache.read().await;
        
        if let Some((cert, cached_at)) = cache.get(host) {
            if cached_at.elapsed() < self.cache_duration {
                return Some(cert.clone());
            }
        }
        
        None
    }
    
    pub async fn cache_certificate(&self, host: String, cert: TLSLSCertificate) {
        let mut cache = self.cache.write().await;
        cache.insert(host, (cert, Instant::now()));
    }
}
```

## Security Considerations

### Certificate Pinning
- **Host Pinning**: Pin certificates to specific hosts
- **Policy Pinning**: Pin security policies to prevent downgrade attacks
- **BPI Anchor Validation**: Always verify BPI ledger anchoring
- **Signature Validation**: Verify both Ed25519 and Dilithium5 signatures

### Attack Mitigation
- **Replay Attacks**: QLOCK session locks prevent replay
- **Man-in-the-Middle**: Certificate pinning and BPI anchoring
- **Quantum Attacks**: Dilithium5 post-quantum signatures
- **Certificate Forgery**: BPI ledger anchoring prevents forgery

### Compliance
- **FIPS 140-2**: Use FIPS-approved cryptographic modules
- **Common Criteria**: EAL4+ security evaluation
- **NIST Post-Quantum**: Dilithium5 NIST-approved algorithm
- **Industry Standards**: Compliance with banking and government regulations

## Next Steps

1. **[QLOCK Quantum-Safe Locks](./03-qlock-quantum-safe-locks.md)** - Session-level security
2. **[Shadow Registry Bridge](./04-shadow-registry-bridge.md)** - Web2-Web3 integration
3. **[Deployment Guide](./05-deployment-and-configuration.md)** - Production setup

## References

- **TLSLS Implementation**: `/home/umesh/metanode/wallet-identity/src/client/transport/httpcg_client.rs`
- **Certificate Management**: Lines 264-430 in httpcg_client.rs
- **BPI Integration**: `/home/umesh/metanode/bpi-core/src/bpi_ledger_integration.rs`
- **Security Engine**: `/home/umesh/metanode/bpi-core/src/security/`
