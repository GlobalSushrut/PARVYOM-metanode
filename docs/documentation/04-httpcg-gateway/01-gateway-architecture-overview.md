# 🌐 HttpCG Gateway Architecture Overview

**Revolutionary httpcg:// Protocol Gateway** - The server-side infrastructure that enables the next-generation internet protocol with quantum-safe security and Web2-Web3 bridging.

---

## 🎯 **What is the HttpCG Gateway?**

The HttpCG Gateway is the server-side infrastructure component that handles incoming `httpcg://` protocol requests and provides the bridge between the revolutionary httpcg protocol and traditional web infrastructure. It combines multiple sophisticated components to deliver quantum-safe, identity-bound communication with seamless Web2 compatibility.

### **Key Capabilities:**
- **httpcg:// Protocol Processing** - Native support for the next-generation internet protocol
- **Quantum-Safe Security** - TLSLS certificates with Ed25519 + Dilithium5 hybrid cryptography
- **Session Lock Validation** - QLOCK quantum-safe session locks with mathematical precision
- **Shadow Registry Integration** - Deterministic httpcg:// to https:// URL resolution
- **Web2 Compatibility** - Transparent proxy mode for seamless adoption
- **Load Balancing & Reliability** - BPI Mesh Gateway Agent with circuit breakers
- **DockLock Integration** - Native deployment on the integrated container platform

---

## 🏗️ **Gateway Architecture Components**

```
┌─────────────────────────────────────────────────────────────────┐
│                    HTTPCG GATEWAY ARCHITECTURE                  │
├─────────────────────────────────────────────────────────────────┤
│  Ingress Layer                                                  │
│  ├── httpcg:// Protocol Handler                                │
│  ├── TLSLS Certificate Validator                               │
│  ├── QLOCK Session Lock Verifier                              │
│  └── Security Policy Enforcer                                 │
├─────────────────────────────────────────────────────────────────┤
│  Routing & Resolution Layer                                     │
│  ├── Shadow Registry Bridge                                    │
│  ├── URL Resolution Engine                                     │
│  ├── Route Fingerprint Calculator                             │
│  └── BPI Mesh Gateway Agent                                   │
├─────────────────────────────────────────────────────────────────┤
│  Processing Layer                                               │
│  ├── Web2 API Gateway                                         │
│  ├── Rate Limiting & Throttling                               │
│  ├── Circuit Breaker Pattern                                  │
│  └── Health Check Monitoring                                  │
├─────────────────────────────────────────────────────────────────┤
│  Backend Integration                                            │
│  ├── HTTPS Proxy Engine                                       │
│  ├── Load Balancer (Round-Robin/Least-Connections)            │
│  ├── Connection Pool Manager                                  │
│  └── Response Security Validator                              │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🔧 **Core Components Deep Dive**

### **1. BPI Mesh Gateway Agent**
*Location: `/bpi-core/crates/metanode-core/gateway/src/bin/gateway.rs`*

**Purpose:** Load balancing, health monitoring, and relay functionality for httpcg requests.

**Key Features:**
- **Load Balancing Strategies:** Round-robin, least-connections, weighted distribution
- **Health Check Monitoring:** Continuous endpoint health validation
- **Circuit Breaker Pattern:** Automatic failure detection and recovery
- **Sidecar Mode:** Service mesh integration capability
- **Metrics Collection:** Prometheus-compatible monitoring
- **Request Routing:** Intelligent request distribution

**Configuration:**
```rust
GatewayConfig {
    gateway_id: "gateway-001",
    listen_addr: "127.0.0.1:8080",
    relay_endpoints: ["http://127.0.0.1:8001", "http://127.0.0.1:8002"],
    health_check_interval_ms: 5000,
    max_connections: 1000,
    request_timeout_ms: 30000,
    retry_attempts: 3,
    circuit_breaker_threshold: 5,
    load_balancing: LoadBalancingStrategy::RoundRobin,
}
```

### **2. Shadow Registry Bridge**
*Location: `/wallet-identity/src/client/transport/httpcg_client.rs`*

**Purpose:** Deterministic resolution of httpcg:// URLs to https:// endpoints while preserving security guarantees.

**Key Features:**
- **URL Resolution:** `httpcg://app/example.com/path` → `https://gateway.pravyom.com/proxy/example.com/path`
- **Registry Caching:** Local cache with TTL for performance
- **Signature Verification:** Cryptographic validation of registry records
- **Fallback Mechanisms:** Multiple registry endpoint support
- **BPI Anchoring:** Immutable audit trails for resolution records

**Resolution Process:**
```rust
pub async fn resolve_httpcg(&self, httpcg_url: &HttpcgUrl) -> Result<ShadowRegistryRecord> {
    // 1. Check local cache first
    if let Some(cached) = self.get_cached_record(&httpcg_url.host).await? {
        return Ok(cached);
    }
    
    // 2. Fetch from Shadow Registry
    let record = self.fetch_from_registry(httpcg_url).await?;
    
    // 3. Verify cryptographic signature
    self.verify_record_signature(&record)?;
    
    // 4. Cache for future use
    self.cache_record(&httpcg_url.host, &record).await?;
    
    Ok(record)
}
```

### **3. TLSLS Certificate System**
*Location: `/wallet-identity/src/client/transport/httpcg_client.rs`*

**Purpose:** Identity-bound transport security with hybrid post-quantum cryptography.

**Key Features:**
- **Hybrid Cryptography:** Ed25519 + Dilithium5 for quantum resistance
- **Identity Binding:** Certificates bound to DID subjects
- **BPI Anchoring:** Certificate issuance receipts on BPI ledger
- **Certificate Rotation:** ≤90 days with mutual handshake
- **HTTPS Compatibility:** TLS extension stapling for interoperability

**Certificate Structure:**
```rust
pub struct TLSLSCertificate {
    pub subject_did: String,           // DID-based subject
    pub public_key_ed25519: Vec<u8>,   // Ed25519 public key
    pub public_key_dilithium: Vec<u8>, // Dilithium5 public key
    pub policy_hash: String,           // Policy attestation
    pub bpi_anchor: String,            // BPI ledger receipt
    pub issued_at: DateTime<Utc>,      // Issuance timestamp
    pub expires_at: DateTime<Utc>,     // Expiration (≤90 days)
    pub signature: Vec<u8>,            // Hybrid signature
}
```

### **4. QLOCK Engine**
*Location: `/wallet-identity/src/client/transport/httpcg_client.rs`*

**Purpose:** Quantum-safe session locks with mathematical precision binding.

**Key Features:**
- **Mathematical Derivation:** HKDF-based key derivation with multiple binding points
- **Session Binding:** Locks bound to TLS session, certificate, route, and time
- **Bridge-Break Protection:** Prevents replay and forwarding attacks
- **Minute Epoch Binding:** Time-based lock rotation for freshness
- **Distance Bounding:** Geographic and network topology validation

**QLOCK Derivation Formula:**
```rust
pub fn derive_qlock_key(
    &self,
    tls_exporter: &[u8],        // TLS session exporter
    spki_hash: &[u8],           // Certificate SPKI hash
    tlsls_fingerprint: &[u8],   // TLSLS certificate fingerprint
    route_fingerprint: &str,    // Route-specific fingerprint
    minute_epoch: u64,          // Current minute epoch
) -> Result<Vec<u8>> {
    // QLK = HKDF("httpcg-qlock/v1" || tls_exporter || SPKI_hash || 
    //           TLSLS_fingerprint || route_fingerprint || minute_epoch)
    
    let mut input = Vec::new();
    input.extend_from_slice(b"httpcg-qlock/v1");
    input.extend_from_slice(tls_exporter);
    input.extend_from_slice(spki_hash);
    input.extend_from_slice(tlsls_fingerprint);
    input.extend_from_slice(route_fingerprint.as_bytes());
    input.extend_from_slice(&minute_epoch.to_be_bytes());
    
    let mut okm = [0u8; 32];
    hkdf::Hkdf::<sha2::Sha256>::new(None, &input)
        .expand(b"httpcg-qlock-session", &mut okm)
        .map_err(|_| anyhow!("HKDF expansion failed"))?;
    
    Ok(okm.to_vec())
}
```

### **5. Web2 API Gateway**
*Location: `/wallet-identity/src/client/transport/httpcg_client.rs`*

**Purpose:** Compatibility layer for traditional web services with progressive enhancement.

**Key Features:**
- **Transparent Proxy:** Zero-config Web2 service access
- **Rate Limiting:** Per-client request throttling
- **Security Policies:** Configurable security enforcement
- **Progressive Enhancement:** Gradual httpcg adoption path
- **API Endpoint Mapping:** Flexible routing configuration

**Rate Limiting Implementation:**
```rust
pub async fn check_rate_limit(&self, client_id: &str) -> Result<bool> {
    let mut limiter = self.rate_limiter.write().await;
    let now = Utc::now();
    
    let state = limiter.entry(client_id.to_string()).or_insert(RateLimitState {
        requests: 0,
        window_start: now,
        limit: 1000, // requests per minute
    });
    
    // Reset window if needed
    if now.signed_duration_since(state.window_start).num_minutes() >= 1 {
        state.requests = 0;
        state.window_start = now;
    }
    
    if state.requests >= state.limit {
        return Ok(false); // Rate limit exceeded
    }
    
    state.requests += 1;
    Ok(true)
}
```

---

## 🌐 **httpcg:// URL Planes**

The HttpCG Gateway handles five distinct URL planes, each with specific routing and security requirements:

### **APP Plane: `httpcg://app/app.example.com/<path>`**
- **Purpose:** Application-level services and APIs
- **Security:** Full TLSLS + QLOCK validation required
- **Routing:** Shadow Registry resolution to application backends
- **Use Cases:** Web applications, mobile app backends, enterprise APIs

### **BPI Plane: `httpcg://bpi/bpi.example.com/hash.bpi/<W_ADDR>/<op>`**
- **Purpose:** Direct BPI ledger operations and queries
- **Security:** Enhanced validation with wallet address verification
- **Routing:** Direct routing to BPI Core infrastructure
- **Use Cases:** Blockchain operations, ledger queries, consensus participation

### **GW Plane: `httpcg://gw/<name.WADDR.NSIG>/<path>`**
- **Purpose:** Dark web and privacy-focused services
- **Security:** Maximum security with additional anonymization
- **Routing:** Onion-style routing through multiple gateways
- **Use Cases:** Privacy applications, whistleblowing, secure communications

### **WALLET Plane: `httpcg://wallet/wallet.pravyom/<path>`**
- **Purpose:** Wallet services and identity management
- **Security:** Identity-bound with strict wallet verification
- **Routing:** Wallet service provider infrastructure
- **Use Cases:** Identity management, credential issuance, wallet operations

### **M2M Plane: `httpcg://m2m/<communicatorAdd>/<OHPH>`**
- **Purpose:** Machine-to-machine and IoT communications
- **Security:** Device-specific certificates and attestation
- **Routing:** IoT gateway infrastructure with device validation
- **Use Cases:** IoT devices, automated systems, sensor networks

---

## 🔒 **Security Architecture**

### **Multi-Layer Security Model**

1. **Transport Layer Security (TLSLS)**
   - Hybrid Ed25519 + Dilithium5 cryptography
   - Identity-bound certificates with DID subjects
   - Certificate rotation ≤90 days
   - BPI ledger anchoring for audit trails

2. **Session Layer Security (QLOCK)**
   - Quantum-safe session locks with mathematical binding
   - Bridge-break protection against replay attacks
   - Minute-epoch rotation for temporal security
   - Route-specific fingerprinting

3. **Application Layer Security (SAPI)**
   - Signed API requests with wallet authentication
   - Request/response integrity validation
   - Replay attack prevention
   - Content authenticity verification

4. **Network Layer Security**
   - Distance bounding for geographic validation
   - Network topology verification
   - Traffic analysis resistance
   - DDoS protection and rate limiting

### **Security Validation Flow**

```rust
pub async fn validate_httpcg_request(&self, request: &HttpcgRequest) -> Result<()> {
    // 1. Validate TLSLS certificate
    let cert = self.tlsls_manager.validate_certificate(&request.host).await?;
    
    // 2. Verify QLOCK session lock
    let qlock = self.qlock_engine.validate_session_lock(
        &request.qlock_header,
        &cert,
        &request.route,
    ).await?;
    
    // 3. Check SAPI signature
    self.validate_sapi_signature(&request.sapi_header, &request.body, &qlock)?;
    
    // 4. Enforce security policies
    self.enforce_security_policies(&request, &cert).await?;
    
    Ok(())
}
```

---

## 📊 **Performance & Scalability**

### **Performance Characteristics**
- **Request Latency:** <50ms for cached Shadow Registry lookups
- **Throughput:** >10,000 requests/second per gateway instance
- **Connection Pooling:** Persistent connections with automatic cleanup
- **Circuit Breaker:** <1s failure detection and recovery
- **Load Balancing:** Sub-millisecond routing decisions

### **Scalability Features**
- **Horizontal Scaling:** Multiple gateway instances with shared state
- **Connection Pooling:** Efficient backend connection management
- **Caching:** Multi-layer caching for Shadow Registry and certificates
- **Health Monitoring:** Automatic unhealthy backend removal
- **Auto-scaling:** DockLock-based automatic scaling based on load

### **Monitoring & Observability**
- **Metrics:** Prometheus-compatible metrics export
- **Logging:** Structured JSON logging with correlation IDs
- **Tracing:** Distributed tracing for request flow analysis
- **Health Checks:** Comprehensive health check endpoints
- **Alerting:** Real-time alerting for critical failures

---

## 🚀 **Production Deployment**

The HttpCG Gateway is designed for production deployment using the integrated DockLock platform, providing deterministic execution, complete audit trails, and military-grade security.

### **Deployment Architecture**
- **DockLock Containers:** Secure, deterministic execution environment
- **Service Mesh:** BPI Mesh integration for inter-service communication
- **Load Balancing:** Native DockLock load balancer with health checks
- **Auto-scaling:** Dynamic scaling based on request volume and latency
- **High Availability:** Multi-zone deployment with automatic failover

### **Integration Points**
- **BPI Core:** Direct integration with BPI ledger and consensus
- **Shadow Registry:** Real-time registry synchronization
- **Wallet Services:** Identity and authentication integration
- **Monitoring Stack:** Prometheus, Grafana, and alerting integration

---

## 🎯 **Use Cases & Applications**

### **Enterprise Integration**
- **API Gateway:** Secure API access for enterprise applications
- **Legacy Migration:** Gradual migration from HTTP to httpcg protocol
- **Compliance:** Regulatory compliance with audit trails and encryption
- **Multi-tenant:** Secure isolation for multiple enterprise customers

### **Web3 Applications**
- **DApp Backends:** Secure backend services for decentralized applications
- **Cross-chain Communication:** Inter-blockchain communication gateway
- **Identity Services:** Decentralized identity and authentication
- **Oracle Services:** Secure data feeds and external API access

### **IoT & Edge Computing**
- **Device Communication:** Secure IoT device communication
- **Edge Gateway:** Edge computing gateway with local processing
- **Sensor Networks:** Large-scale sensor network coordination
- **Industrial IoT:** Manufacturing and industrial automation

---

## 📚 **Next Steps**

1. **[Protocol Implementation](02-httpcg-protocol-implementation.md)** - Deep dive into httpcg protocol handling
2. **[Security Architecture](03-security-architecture-tlsls-qlock.md)** - Detailed security implementation
3. **[Load Balancing & Reliability](04-load-balancing-reliability.md)** - BPI Mesh Gateway Agent details
4. **[Configuration & Deployment](05-configuration-deployment.md)** - DockLock deployment guide
5. **[Monitoring & Troubleshooting](06-monitoring-troubleshooting.md)** - Operational procedures

---

*The HttpCG Gateway represents a revolutionary leap in internet infrastructure, combining quantum-safe security, identity-bound transport, and seamless Web2-Web3 bridging in a production-ready, enterprise-grade platform.*
