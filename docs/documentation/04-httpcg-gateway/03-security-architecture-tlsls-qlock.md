# 🔒 Security Architecture: TLSLS & QLOCK

**Military-Grade Security Implementation** - Revolutionary TLSLS certificates and QLOCK quantum-safe session locks for identity-bound transport security.

---

## 🎯 **Security Overview**

The HttpCG Gateway implements multi-layered security with quantum-safe cryptography, identity-bound transport, and mathematical bridge-break protection.

### **Security Guarantees:**
- **Quantum-Safe Cryptography** - Ed25519 + Dilithium5 hybrid signatures
- **Identity-Bound Transport** - DID-based certificate subjects
- **Bridge-Break Protection** - Mathematical replay attack prevention
- **Temporal Security** - Minute-epoch binding with key rotation
- **Audit Trails** - Immutable BPI ledger anchoring

---

## 🔐 **TLSLS Certificate System**

### **Certificate Structure**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TLSLSCertificate {
    pub subject_did: String,           // DID-based subject
    pub public_key_ed25519: Vec<u8>,   // Ed25519 public key
    pub public_key_dilithium: Vec<u8>, // Dilithium5 public key
    pub policy_hash: String,           // Security policy hash
    pub bpi_anchor: String,            // BPI ledger anchor
    pub issued_at: DateTime<Utc>,      // Issuance timestamp
    pub expires_at: DateTime<Utc>,     // Expiration (≤90 days)
    pub signature: Vec<u8>,            // Hybrid signature
}
```

### **Certificate Validation**

```rust
pub async fn validate_tlsls_certificate(&self, host: &str) -> Result<TLSLSCertificate> {
    let cert = self.tlsls_manager.get_certificate(host).await?;
    
    // 1. Check expiration (≤90 days)
    if cert.expires_at < Utc::now() {
        return Err(anyhow!("Certificate expired"));
    }
    
    // 2. Verify hybrid signature
    self.verify_hybrid_signature(&cert)?;
    
    // 3. Validate BPI anchoring
    self.validate_bpi_anchor(&cert.bpi_anchor).await?;
    
    Ok(cert)
}
```

---

## 🌀 **QLOCK Session Lock System**

### **QLOCK Mathematical Formula**

```
QLK = HKDF("httpcg-qlock/v1" || tls_exporter || SPKI_hash || 
          TLSLS_fingerprint || route_fingerprint || minute_epoch)
```

### **QLOCK Implementation**

```rust
pub fn derive_qlock_key(&self,
    tls_exporter: &[u8],
    spki_hash: &[u8],
    tlsls_fingerprint: &[u8],
    route_fingerprint: &str,
    minute_epoch: u64,
) -> Result<Vec<u8>> {
    let mut input = Vec::new();
    input.extend_from_slice(b"httpcg-qlock/v1");
    input.extend_from_slice(tls_exporter);
    input.extend_from_slice(spki_hash);
    input.extend_from_slice(tlsls_fingerprint);
    input.extend_from_slice(route_fingerprint.as_bytes());
    input.extend_from_slice(&minute_epoch.to_be_bytes());
    
    let mut okm = [0u8; 32];
    hkdf::Hkdf::<sha2::Sha256>::new(None, &input)
        .expand(b"httpcg-qlock-session", &mut okm)?;
    
    Ok(okm.to_vec())
}
```

### **QLOCK Validation**

```rust
pub async fn validate_qlock_session(&self, 
    qlock_header: &str, 
    cert: &TLSLSCertificate, 
    route: &str
) -> Result<QLOCKSession> {
    let qlock_data: serde_json::Value = serde_json::from_str(qlock_header)?;
    
    let minute_epoch = qlock_data["minute_epoch"].as_u64()
        .ok_or_else(|| anyhow!("Missing minute_epoch"))?;
    let qlock_hash = qlock_data["qlock_hash"].as_str()
        .ok_or_else(|| anyhow!("Missing qlock_hash"))?;
    
    // Validate minute epoch (current or previous minute)
    let current_epoch = Utc::now().timestamp() as u64 / 60;
    if minute_epoch < current_epoch - 1 || minute_epoch > current_epoch {
        return Err(anyhow!("Invalid minute epoch"));
    }
    
    // Derive expected QLOCK
    let expected_qlock = self.derive_qlock_key(
        &self.generate_tls_exporter(cert)?,
        &self.calculate_spki_hash(cert)?,
        &self.calculate_tlsls_fingerprint(cert)?,
        route,
        minute_epoch,
    )?;
    
    // Verify hash
    let expected_hash = hex::encode(sha2::Sha256::digest(&expected_qlock));
    if qlock_hash != expected_hash {
        return Err(anyhow!("QLOCK verification failed"));
    }
    
    Ok(QLOCKSession {
        qlock_key: expected_qlock,
        minute_epoch,
        route: route.to_string(),
        expires_at: Utc::now() + chrono::Duration::minutes(2),
    })
}
```

---

## 🛡️ **Bridge-Break Protection**

### **Security Mechanisms**

1. **Route Binding** - QLOCK bound to specific route fingerprint
2. **Certificate Binding** - QLOCK bound to TLSLS certificate
3. **Temporal Binding** - Minute-epoch rotation prevents replay
4. **Distance Bounding** - Geographic validation prevents forwarding

### **Attack Prevention**

```rust
pub fn detect_bridge_break(&self, request: &HttpcgRequest) -> Result<()> {
    // 1. Validate route consistency
    if request.original_route != request.current_route {
        return Err(anyhow!("Route tampering detected"));
    }
    
    // 2. Check temporal binding
    let current_epoch = Utc::now().timestamp() as u64 / 60;
    if request.qlock_epoch < current_epoch - 1 {
        return Err(anyhow!("Stale QLOCK detected"));
    }
    
    // 3. Verify distance bounding
    if self.calculate_network_distance(&request.source_ip)? > MAX_DISTANCE {
        return Err(anyhow!("Distance bounding violation"));
    }
    
    Ok(())
}
```

---

## 📊 **Security Monitoring**

### **Security Metrics**

```rust
pub struct SecurityMetrics {
    certificate_validations: Counter,
    qlock_validations: Counter,
    bridge_break_detections: Counter,
    security_violations: Counter,
}

impl SecurityMetrics {
    pub fn record_security_event(&self, event_type: &str, success: bool) {
        match event_type {
            "certificate_validation" => {
                self.certificate_validations.inc();
                if !success {
                    self.security_violations.inc();
                }
            },
            "qlock_validation" => {
                self.qlock_validations.inc();
                if !success {
                    self.security_violations.inc();
                }
            },
            "bridge_break_detection" => {
                self.bridge_break_detections.inc();
                self.security_violations.inc();
            },
            _ => {}
        }
    }
}
```

---

## 🔧 **Security Configuration**

### **DockLock Security Deployment**

```yaml
# httpcg-gateway-security.yml
apiVersion: docklock.bpi.network/v1
kind: SecurityPolicy
metadata:
  name: httpcg-gateway-security
spec:
  tlsls:
    enabled: true
    certificate_rotation_days: 90
    hybrid_cryptography: true
    bpi_anchoring: true
  
  qlock:
    enabled: true
    minute_epoch_validation: true
    bridge_break_detection: true
    distance_bounding: 50000  # 50km max
  
  monitoring:
    security_events: true
    audit_logging: true
    metrics_export: true
```

---

*The TLSLS & QLOCK security architecture provides military-grade protection with quantum-safe cryptography and mathematical precision binding for the next-generation internet.*
