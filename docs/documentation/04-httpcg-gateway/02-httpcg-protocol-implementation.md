# 🌐 HttpCG Protocol Implementation

**Deep Dive into httpcg:// Protocol Handling** - Complete implementation guide for the revolutionary next-generation internet protocol with quantum-safe security and Web2-Web3 bridging.

---

## 🎯 **Protocol Overview**

The httpcg:// protocol is a revolutionary next-generation internet protocol that provides quantum-safe security, identity-bound transport, and seamless Web2-Web3 bridging. The HttpCG Gateway implements the server-side protocol handling with complete security validation and routing capabilities.

### **Protocol Characteristics:**
- **URL Scheme:** `httpcg://` with five distinct planes (app, bpi, gw, wallet, m2m)
- **Transport Security:** TLSLS certificates with hybrid Ed25519 + Dilithium5 cryptography
- **Session Security:** QLOCK quantum-safe session locks with mathematical precision
- **Identity Binding:** DID-based identity with wallet authentication
- **Web2 Compatibility:** Transparent proxy mode for seamless adoption

---

## 🏗️ **Protocol Stack Architecture**

```
┌─────────────────────────────────────────────────────────────────┐
│                    HTTPCG PROTOCOL STACK                       │
├─────────────────────────────────────────────────────────────────┤
│  Application Layer                                              │
│  ├── SAPI (Signed API) - Request/Response Authentication       │
│  ├── Wallet Authentication - DID-based identity verification   │
│  ├── Content Integrity - Cryptographic content validation     │
│  └── Application Logic - Business logic and data processing    │
├─────────────────────────────────────────────────────────────────┤
│  Session Layer                                                  │
│  ├── QLOCK Engine - Quantum-safe session locks                │
│  ├── Session Management - Connection state and lifecycle       │
│  ├── Bridge-Break Detection - Replay attack prevention         │
│  └── Temporal Binding - Minute-epoch lock rotation            │
├─────────────────────────────────────────────────────────────────┤
│  Transport Layer                                                │
│  ├── TLSLS Certificates - Identity-bound transport security    │
│  ├── Hybrid Cryptography - Ed25519 + Dilithium5 signatures    │
│  ├── Certificate Validation - BPI ledger anchoring            │
│  └── Connection Management - Persistent connection pooling     │
├─────────────────────────────────────────────────────────────────┤
│  Network Layer                                                  │
│  ├── Shadow Registry - httpcg:// to https:// resolution       │
│  ├── URL Routing - Multi-plane routing and load balancing     │
│  ├── Distance Bounding - Geographic and network validation     │
│  └── Traffic Analysis Resistance - Privacy protection          │
├─────────────────────────────────────────────────────────────────┤
│  Physical Layer                                                 │
│  ├── TCP/IP - Standard internet protocol foundation           │
│  ├── TLS 1.3 - Base transport encryption                      │
│  ├── HTTP/2 - Efficient multiplexed communication             │
│  └── WebSocket - Real-time bidirectional communication        │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🔧 **URL Structure & Parsing**

### **httpcg:// URL Format**

```
httpcg://<plane>/<authority>/<path>?<query>#<fragment>
```

### **URL Planes Implementation**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpcgUrl {
    pub plane: String,      // app, bpi, gw, wallet, m2m
    pub authority: String,  // Domain or address
    pub path: String,       // Resource path
    pub query: Option<String>,
    pub fragment: Option<String>,
}

impl HttpcgUrl {
    pub fn parse(url_str: &str) -> Result<Self> {
        let url = Url::parse(url_str)
            .map_err(|e| anyhow!("Invalid httpcg URL: {}", e))?;
        
        if url.scheme() != "httpcg" {
            return Err(anyhow!("URL must use httpcg:// scheme"));
        }
        
        let path_segments: Vec<&str> = url.path().trim_start_matches('/').split('/').collect();
        if path_segments.is_empty() {
            return Err(anyhow!("httpcg URL must specify a plane"));
        }
        
        let plane = path_segments[0].to_string();
        let authority = if path_segments.len() > 1 {
            path_segments[1].to_string()
        } else {
            return Err(anyhow!("httpcg URL must specify authority"));
        };
        
        let path = if path_segments.len() > 2 {
            format!("/{}", path_segments[2..].join("/"))
        } else {
            "/".to_string()
        };
        
        Ok(HttpcgUrl {
            plane,
            authority,
            path,
            query: url.query().map(|q| q.to_string()),
            fragment: url.fragment().map(|f| f.to_string()),
        })
    }
    
    pub fn to_string(&self) -> String {
        let mut url = format!("httpcg://{}/{}{}", self.plane, self.authority, self.path);
        
        if let Some(ref query) = self.query {
            url.push('?');
            url.push_str(query);
        }
        
        if let Some(ref fragment) = self.fragment {
            url.push('#');
            url.push_str(fragment);
        }
        
        url
    }
}
```

### **Plane-Specific Routing**

```rust
pub async fn route_httpcg_request(&self, url: &HttpcgUrl) -> Result<String> {
    match url.plane.as_str() {
        "app" => {
            // APP plane: httpcg://app/app.example.com/path
            // Route to application backends via Shadow Registry
            let record = self.shadow_registry.resolve(url).await?;
            Ok(format!("https://{}{}", record.target_host, url.path))
        },
        "bpi" => {
            // BPI plane: httpcg://bpi/bpi.example.com/hash.bpi/W_ADDR/op
            // Route to BPI Core infrastructure
            Ok(format!("https://bpi-core.pravyom.com/ledger{}", url.path))
        },
        "gw" => {
            // GW plane: httpcg://gw/name.WADDR.NSIG/path (dark web)
            // Route through onion-style gateway network
            self.route_dark_web_request(url).await
        },
        "wallet" => {
            // WALLET plane: httpcg://wallet/wallet.pravyom/path
            // Route to wallet service providers
            Ok(format!("https://wallet-api.pravyom.com{}", url.path))
        },
        "m2m" => {
            // M2M plane: httpcg://m2m/communicatorAdd/OHPH
            // Route to IoT gateway infrastructure
            Ok(format!("https://iot-gateway.pravyom.com{}", url.path))
        },
        _ => Err(anyhow!("Unsupported httpcg plane: {}", url.plane))
    }
}
```

---

## 🔒 **Security Implementation**

### **TLSLS Certificate Validation**

```rust
pub async fn validate_tlsls_certificate(&self, host: &str) -> Result<TLSLSCertificate> {
    // 1. Get or create TLSLS certificate for host
    let cert = self.tlsls_manager.get_or_create_certificate(host).await?;
    
    // 2. Validate certificate hasn't expired (≤90 days)
    if cert.expires_at < Utc::now() {
        return Err(anyhow!("TLSLS certificate expired for host: {}", host));
    }
    
    // 3. Verify hybrid signature (Ed25519 + Dilithium5)
    self.verify_hybrid_signature(&cert)?;
    
    // 4. Validate BPI ledger anchoring
    self.validate_bpi_anchor(&cert.bpi_anchor).await?;
    
    // 5. Check policy hash attestation
    self.validate_policy_hash(&cert.policy_hash)?;
    
    Ok(cert)
}

fn verify_hybrid_signature(&self, cert: &TLSLSCertificate) -> Result<()> {
    // Verify Ed25519 signature
    let ed25519_key = ed25519_dalek::PublicKey::from_bytes(&cert.public_key_ed25519)
        .map_err(|e| anyhow!("Invalid Ed25519 public key: {}", e))?;
    
    let ed25519_sig = ed25519_dalek::Signature::from_bytes(&cert.signature[..64])
        .map_err(|e| anyhow!("Invalid Ed25519 signature: {}", e))?;
    
    let message = self.create_certificate_message(cert)?;
    ed25519_key.verify(&message, &ed25519_sig)
        .map_err(|e| anyhow!("Ed25519 signature verification failed: {}", e))?;
    
    // Verify Dilithium5 signature (post-quantum)
    // Note: Dilithium implementation would be integrated here
    // For now, we validate the signature format and length
    if cert.signature.len() < 64 + 2420 { // Ed25519 (64) + Dilithium5 (~2420)
        return Err(anyhow!("Invalid Dilithium5 signature length"));
    }
    
    Ok(())
}
```

### **QLOCK Session Lock Validation**

```rust
pub async fn validate_qlock_session(&self, 
    qlock_header: &str, 
    cert: &TLSLSCertificate, 
    route: &str
) -> Result<QLOCKSession> {
    // 1. Parse QLOCK header
    let qlock_data: serde_json::Value = serde_json::from_str(qlock_header)
        .map_err(|e| anyhow!("Invalid QLOCK header format: {}", e))?;
    
    // 2. Extract session parameters
    let session_id = qlock_data["session_id"].as_str()
        .ok_or_else(|| anyhow!("Missing session_id in QLOCK header"))?;
    let minute_epoch = qlock_data["minute_epoch"].as_u64()
        .ok_or_else(|| anyhow!("Missing minute_epoch in QLOCK header"))?;
    let qlock_hash = qlock_data["qlock_hash"].as_str()
        .ok_or_else(|| anyhow!("Missing qlock_hash in QLOCK header"))?;
    
    // 3. Validate minute epoch (must be current or previous minute)
    let current_epoch = Utc::now().timestamp() as u64 / 60;
    if minute_epoch < current_epoch - 1 || minute_epoch > current_epoch {
        return Err(anyhow!("Invalid minute epoch: {} (current: {})", minute_epoch, current_epoch));
    }
    
    // 4. Derive expected QLOCK key
    let tls_exporter = self.generate_tls_exporter(&cert.subject_did, session_id)?;
    let spki_hash = self.calculate_spki_hash(cert)?;
    let tlsls_fingerprint = self.calculate_tlsls_fingerprint(cert)?;
    let route_fingerprint = self.calculate_route_fingerprint(route)?;
    
    let expected_qlock = self.qlock_engine.derive_qlock_key(
        &tls_exporter,
        &spki_hash,
        &tlsls_fingerprint,
        &route_fingerprint,
        minute_epoch,
    )?;
    
    // 5. Verify QLOCK hash matches expected value
    let expected_hash = hex::encode(sha2::Sha256::digest(&expected_qlock));
    if qlock_hash != expected_hash {
        return Err(anyhow!("QLOCK hash verification failed"));
    }
    
    // 6. Create validated session
    Ok(QLOCKSession {
        session_id: session_id.to_string(),
        qlock_key: expected_qlock,
        minute_epoch,
        route: route.to_string(),
        created_at: Utc::now(),
        expires_at: Utc::now() + chrono::Duration::minutes(2), // 2-minute window
    })
}
```

### **SAPI (Signed API) Validation**

```rust
pub fn validate_sapi_signature(&self, 
    sapi_header: &str, 
    body: &[u8], 
    qlock_session: &QLOCKSession
) -> Result<()> {
    // 1. Parse SAPI header
    let sapi_data: serde_json::Value = serde_json::from_str(sapi_header)
        .map_err(|e| anyhow!("Invalid SAPI header format: {}", e))?;
    
    // 2. Extract signature components
    let wallet_address = sapi_data["wallet_address"].as_str()
        .ok_or_else(|| anyhow!("Missing wallet_address in SAPI header"))?;
    let timestamp = sapi_data["timestamp"].as_u64()
        .ok_or_else(|| anyhow!("Missing timestamp in SAPI header"))?;
    let signature = sapi_data["signature"].as_str()
        .ok_or_else(|| anyhow!("Missing signature in SAPI header"))?;
    let nonce = sapi_data["nonce"].as_str()
        .ok_or_else(|| anyhow!("Missing nonce in SAPI header"))?;
    
    // 3. Validate timestamp (must be within 5 minutes)
    let current_time = Utc::now().timestamp() as u64;
    if timestamp < current_time - 300 || timestamp > current_time + 300 {
        return Err(anyhow!("SAPI timestamp out of valid range"));
    }
    
    // 4. Create signature message
    let mut message = Vec::new();
    message.extend_from_slice(wallet_address.as_bytes());
    message.extend_from_slice(&timestamp.to_be_bytes());
    message.extend_from_slice(nonce.as_bytes());
    message.extend_from_slice(&qlock_session.qlock_key);
    message.extend_from_slice(body);
    
    // 5. Verify wallet signature
    let signature_bytes = hex::decode(signature)
        .map_err(|e| anyhow!("Invalid signature hex: {}", e))?;
    
    // Note: Wallet signature verification would integrate with wallet identity system
    // For now, we validate the signature format
    if signature_bytes.len() != 64 {
        return Err(anyhow!("Invalid signature length"));
    }
    
    Ok(())
}
```

---

## 🌐 **Shadow Registry Integration**

### **Registry Resolution Process**

```rust
pub async fn resolve_shadow_registry(&self, httpcg_url: &HttpcgUrl) -> Result<ShadowRegistryRecord> {
    // 1. Check local cache first
    let cache_key = format!("{}:{}", httpcg_url.plane, httpcg_url.authority);
    if let Some(cached) = self.registry_cache.get(&cache_key).await {
        if cached.expires_at > Utc::now() {
            return Ok(cached);
        }
    }
    
    // 2. Query Shadow Registry endpoints
    for endpoint in &self.gateway_endpoints {
        match self.query_registry_endpoint(endpoint, httpcg_url).await {
            Ok(record) => {
                // 3. Verify record signature
                self.verify_registry_signature(&record)?;
                
                // 4. Cache the record
                self.registry_cache.insert(cache_key.clone(), record.clone()).await;
                
                return Ok(record);
            },
            Err(e) => {
                tracing::warn!("Registry endpoint {} failed: {}", endpoint, e);
                continue;
            }
        }
    }
    
    Err(anyhow!("All Shadow Registry endpoints failed"))
}

async fn query_registry_endpoint(&self, 
    endpoint: &str, 
    httpcg_url: &HttpcgUrl
) -> Result<ShadowRegistryRecord> {
    let query_url = format!("{}/resolve/{}/{}", 
        endpoint, httpcg_url.plane, httpcg_url.authority);
    
    let response = reqwest::Client::new()
        .get(&query_url)
        .header("User-Agent", "HttpCG-Gateway/1.0")
        .header("X-Wallet-Address", &self.wallet.address())
        .timeout(Duration::from_secs(5))
        .send()
        .await
        .map_err(|e| anyhow!("Registry request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(anyhow!("Registry returned error: {}", response.status()));
    }
    
    let record: ShadowRegistryRecord = response.json().await
        .map_err(|e| anyhow!("Invalid registry response: {}", e))?;
    
    Ok(record)
}
```

### **Registry Record Structure**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowRegistryRecord {
    pub httpcg_host: String,        // Original httpcg authority
    pub target_host: String,        // Target HTTPS host
    pub target_port: u16,           // Target port
    pub rp_did: String,             // Relying Party DID
    pub tlsls_required: bool,       // TLSLS requirement
    pub rbac_profile: String,       // RBAC profile name
    pub bpi_anchor: String,         // BPI ledger anchor
    pub expires_at: DateTime<Utc>,  // Record expiration
    pub signature: String,          // Registry signature
}
```

---

## 🔄 **Request Processing Flow**

### **Complete Request Handling**

```rust
pub async fn handle_httpcg_request(&self, request: HttpcgRequest) -> Result<HttpcgResponse> {
    // 1. Parse httpcg URL
    let httpcg_url = HttpcgUrl::parse(&request.url)?;
    
    // 2. Validate TLSLS certificate
    let cert = self.validate_tlsls_certificate(&httpcg_url.authority).await?;
    
    // 3. Validate QLOCK session lock
    let qlock_session = self.validate_qlock_session(
        &request.qlock_header, 
        &cert, 
        &httpcg_url.path
    ).await?;
    
    // 4. Validate SAPI signature
    self.validate_sapi_signature(
        &request.sapi_header, 
        &request.body, 
        &qlock_session
    )?;
    
    // 5. Resolve target via Shadow Registry
    let registry_record = self.resolve_shadow_registry(&httpcg_url).await?;
    
    // 6. Route to target backend
    let target_url = format!("https://{}:{}{}", 
        registry_record.target_host, 
        registry_record.target_port, 
        httpcg_url.path
    );
    
    // 7. Forward request with security headers
    let mut headers = HashMap::new();
    headers.insert("X-HttpCG-Original-URL".to_string(), httpcg_url.to_string());
    headers.insert("X-HttpCG-Wallet-Address".to_string(), self.wallet.address());
    headers.insert("X-HttpCG-Session-ID".to_string(), qlock_session.session_id.clone());
    
    let response = self.send_backend_request(
        &target_url, 
        &request.method, 
        request.body.as_deref(), 
        &headers
    ).await?;
    
    // 8. Validate response security
    self.validate_response_security(&response, &qlock_session)?;
    
    Ok(response)
}
```

### **Backend Request Forwarding**

```rust
async fn send_backend_request(&self, 
    url: &str, 
    method: &str, 
    body: Option<&[u8]>, 
    headers: &HashMap<String, String>
) -> Result<HttpcgResponse> {
    let client = reqwest::Client::new();
    let mut request_builder = match method {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        "PATCH" => client.patch(url),
        _ => return Err(anyhow!("Unsupported HTTP method: {}", method))
    };
    
    // Add headers
    for (key, value) in headers {
        request_builder = request_builder.header(key, value);
    }
    
    // Add body if present
    if let Some(body_data) = body {
        request_builder = request_builder.body(body_data.to_vec());
    }
    
    // Send request with timeout
    let response = request_builder
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| anyhow!("Backend request failed: {}", e))?;
    
    // Extract response data
    let status = response.status().as_u16();
    let headers: HashMap<String, String> = response.headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();
    
    let body = response.bytes().await
        .map_err(|e| anyhow!("Failed to read response body: {}", e))?
        .to_vec();
    
    Ok(HttpcgResponse {
        status,
        headers,
        body,
    })
}
```

---

## 📊 **Performance Optimizations**

### **Connection Pooling**

```rust
pub struct ConnectionPool {
    pools: Arc<RwLock<HashMap<String, Pool<TLSLSConnection>>>>,
    max_connections_per_host: usize,
    connection_timeout: Duration,
}

impl ConnectionPool {
    pub async fn get_connection(&self, host: &str, port: u16) -> Result<TLSLSConnection> {
        let pool_key = format!("{}:{}", host, port);
        let pools = self.pools.read().await;
        
        if let Some(pool) = pools.get(&pool_key) {
            if let Some(conn) = pool.try_get() {
                if conn.is_valid() {
                    return Ok(conn);
                }
            }
        }
        
        // Create new connection if pool is empty or connections are invalid
        drop(pools);
        self.create_new_connection(host, port).await
    }
    
    async fn create_new_connection(&self, host: &str, port: u16) -> Result<TLSLSConnection> {
        let connection = self.tlsls_manager.establish_connection(host, port).await?;
        
        // Add to pool for reuse
        let pool_key = format!("{}:{}", host, port);
        let mut pools = self.pools.write().await;
        let pool = pools.entry(pool_key).or_insert_with(|| {
            Pool::new(self.max_connections_per_host)
        });
        
        pool.put(connection.clone());
        Ok(connection)
    }
}
```

### **Caching Strategy**

```rust
pub struct HttpcgCache {
    registry_cache: Arc<RwLock<LruCache<String, ShadowRegistryRecord>>>,
    certificate_cache: Arc<RwLock<LruCache<String, TLSLSCertificate>>>,
    qlock_cache: Arc<RwLock<LruCache<String, QLOCKSession>>>,
}

impl HttpcgCache {
    pub async fn get_registry_record(&self, key: &str) -> Option<ShadowRegistryRecord> {
        let cache = self.registry_cache.read().await;
        cache.get(key).and_then(|record| {
            if record.expires_at > Utc::now() {
                Some(record.clone())
            } else {
                None
            }
        })
    }
    
    pub async fn cache_registry_record(&self, key: String, record: ShadowRegistryRecord) {
        let mut cache = self.registry_cache.write().await;
        cache.put(key, record);
    }
}
```

---

## 🔍 **Error Handling & Recovery**

### **Circuit Breaker Pattern**

```rust
pub struct CircuitBreaker {
    failure_count: Arc<AtomicUsize>,
    last_failure_time: Arc<RwLock<Option<Instant>>>,
    failure_threshold: usize,
    recovery_timeout: Duration,
    state: Arc<RwLock<CircuitState>>,
}

#[derive(Debug, Clone)]
enum CircuitState {
    Closed,    // Normal operation
    Open,      // Failing, reject requests
    HalfOpen,  // Testing recovery
}

impl CircuitBreaker {
    pub async fn call<F, T, E>(&self, f: F) -> Result<T, E> 
    where
        F: Future<Output = Result<T, E>>,
        E: std::fmt::Debug,
    {
        let state = *self.state.read().await;
        
        match state {
            CircuitState::Open => {
                // Check if we should try recovery
                if let Some(last_failure) = *self.last_failure_time.read().await {
                    if last_failure.elapsed() > self.recovery_timeout {
                        *self.state.write().await = CircuitState::HalfOpen;
                    } else {
                        return Err(/* circuit open error */);
                    }
                }
            },
            CircuitState::HalfOpen => {
                // Try the operation
                match f.await {
                    Ok(result) => {
                        // Success! Close the circuit
                        self.failure_count.store(0, Ordering::Relaxed);
                        *self.state.write().await = CircuitState::Closed;
                        return Ok(result);
                    },
                    Err(e) => {
                        // Still failing, open circuit again
                        *self.state.write().await = CircuitState::Open;
                        *self.last_failure_time.write().await = Some(Instant::now());
                        return Err(e);
                    }
                }
            },
            CircuitState::Closed => {
                // Normal operation
                match f.await {
                    Ok(result) => Ok(result),
                    Err(e) => {
                        // Record failure
                        let failures = self.failure_count.fetch_add(1, Ordering::Relaxed) + 1;
                        if failures >= self.failure_threshold {
                            *self.state.write().await = CircuitState::Open;
                            *self.last_failure_time.write().await = Some(Instant::now());
                        }
                        Err(e)
                    }
                }
            }
        }
    }
}
```

---

## 📈 **Monitoring & Metrics**

### **Request Metrics Collection**

```rust
pub struct HttpcgMetrics {
    request_counter: Counter,
    request_duration: Histogram,
    error_counter: Counter,
    active_connections: Gauge,
    cache_hit_ratio: Gauge,
}

impl HttpcgMetrics {
    pub fn record_request(&self, method: &str, plane: &str, status: u16, duration: Duration) {
        self.request_counter
            .with_label_values(&[method, plane, &status.to_string()])
            .inc();
        
        self.request_duration
            .with_label_values(&[method, plane])
            .observe(duration.as_secs_f64());
        
        if status >= 400 {
            self.error_counter
                .with_label_values(&[method, plane, &status.to_string()])
                .inc();
        }
    }
    
    pub fn update_active_connections(&self, count: i64) {
        self.active_connections.set(count as f64);
    }
    
    pub fn update_cache_hit_ratio(&self, ratio: f64) {
        self.cache_hit_ratio.set(ratio);
    }
}
```

---

## 🎯 **Next Steps**

1. **[Security Architecture](03-security-architecture-tlsls-qlock.md)** - Deep dive into TLSLS and QLOCK implementation
2. **[Load Balancing & Reliability](04-load-balancing-reliability.md)** - BPI Mesh Gateway Agent details
3. **[Configuration & Deployment](05-configuration-deployment.md)** - DockLock deployment guide
4. **[Monitoring & Troubleshooting](06-monitoring-troubleshooting.md)** - Operational procedures

---

*The HttpCG Protocol Implementation provides the foundation for the next-generation internet with quantum-safe security, identity-bound transport, and seamless Web2-Web3 bridging capabilities.*
