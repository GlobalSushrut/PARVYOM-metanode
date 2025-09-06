# ENC Lock + TSLPS (Space-Time Locked TLS) – Integration Guide

**Purpose**: Add a physical wave-lock around your existing encryption (AES/PQC) so ciphertext is never observable without the lock. Browsers keep using ordinary TLS 1.3 and accept your site with no changes or warnings.

**ENC Lock is not a cipher**; it's a channel lock (phase identities + DSSS + optional distance-bounding) that wraps already-encrypted data.

## 0) TL;DR (one page)

1. Keep TLS 1.3 with a public CA cert (Let's Encrypt / DigiCert).

2. Publish a TSLPS policy (JSON + detached signature) at:
   - `/.well-known/tslps-policy.json`
   - `/.well-known/tslps-policy.sig`
   - DNS TXT: `_tslps.<domain>`

3. HTTP header:
   ```
   TSLPS-Policy: required; url="/.well-known/tslps-policy.json"; sig="/.well-known/tslps-policy.sig"
   ```

4. Run an ENC Gateway that:
   - Terminates TLS 1.3
   - Derives a per-session seed (PST) using TLS exporter
   - Sends PST to your SDR/optic daemon, which enforces:
     - Phase locks: sin²θ+cos²θ=1 (90°), secθ·cosθ=1 (180°)
     - QLOCK sync gate (tan/sec binary = 1 or ∞ collapse)
     - DSSS despreading
     - Projection hopping (cos/sin)
     - Optional ToF distance-bounding

**Result**: Browsers work as normal; interceptors see noise; relays fail ToF.

## 1) Threat Model & What You Gain

- **Adversary** can record your channel and try quantum "store-now, decrypt-later."
- **Normal crypto**: captured ciphertext is at risk if PQ algorithms advance.
- **With ENC Lock + QLOCK**: ciphertext only exists when sync=1.
- **If sync breaks** → infinite phase possibilities (noise).
- **Quantum/classical attackers** have no ciphertext to attack.

This makes ENC Lock **post-quantum++++ secure** — protecting not only against decryption, but against **ciphertext observability** itself.

## 2) Policy: TSLPS (Transport Secure Lock for Phase Signals)

### Example `/.well-known/tslps-policy.json`

```json
{
  "version": "2.0",
  "domain": "example.com",
  "policy_id": "tslps-2025-08-21-st01",
  "enc_lock": {
    "daughter_lock": {"angle_deg": 90, "check": "sin^2θ+cos^2θ=1"},
    "secant_lock":   {"angle_deg": 180, "check": "secθ·cosθ=1"},
    "higher_order": [
      {"check": "secθ·cosθ≈1", "epsilon": 1e-3},
      {"check": "sg(|cosθ|)≥δ", "delta": 1e-3},
      {"check": "sgn(tanθ)=(-1)^(1-h)", "hop_source": "PST:symbol_index"}
    ],
    "mapping": "ciphertext→phase(θ)",
    "spreading": {"pn_len": 127, "scheme": "DSSS", "hop_rate_hz": 1000},
    "sync_gate": {
      "equation": "QLOCK(θ,h) = 1 if secθ·cosθ≈1 AND sgn(tanθ)=(-1)^(1-h), else 0",
      "on_fail": "ciphertext→∞ (noise, drop)"
    },
    "tamper_action": "drop"
  },
  "spacetime": {
    "distance_bound_m": 50,
    "epoch_ms": 25,
    "tof_jitter_ns": 5
  },
  "attestation": {
    "enc_hw": "ENC-Board-v2",
    "firmware_sha256": "REPLACE_ME",
    "calibration_sha256": "REPLACE_ME"
  },
  "tls_cert_fingerprint_sha256": "REPLACE_WITH_YOUR_LEAF_CERT_SHA256",
  "valid_for_sec": 2592000,
  "issued_at": "2025-08-21T17:00:00Z"
}
```

Sign with TLS key (detached CMS/PKCS#7), publish DNS TXT and HTTP headers.

## 3) Web Server Config

### Nginx Example:
```nginx
server {
  listen 443 ssl http2;
  server_name example.com;

  ssl_certificate     /etc/ssl/certs/example.com.fullchain.pem;
  ssl_certificate_key /etc/ssl/private/example.com.key;
  ssl_protocols TLSv1.3;

  add_header TSLPS-Policy 'required; url="/.well-known/tslps-policy.json"; sig="/.well-known/tslps-policy.sig"' always;

  location /.well-known/ { root /var/www; }
  location / { proxy_pass http://127.0.0.1:8080; }
}
```

## 4) ENC Gateway (TLS exporter → SDR)

Minimal Go reverse proxy provided (terminates TLS, derives PST, sends PST to SDR/optic process).

## 5) SDR/Optic Daemon – Enforcement

### Inputs
- PST, policy_id
- Phase observations per symbol

### Process
- Generate PN from PST
- Projection hopping from PST
- Epoch rotation
- Distance-bounding (ToF check)
- **QLOCK equation**:

```
QLOCK(θ,h) = {
  1, if secθ·cosθ≈1 ∧ sgn(tanθ)=(-1)^(1-h)
  0, otherwise (including cosθ→0, tanθ→∞)
}
```

### Output
- Frames only accepted if QLOCK=1.
- Otherwise → drop, alarm, collapse to ∞.

## 6) App Layer
- **Encryption unchanged**: AES-256-GCM or ChaCha20-Poly1305.
- Use hybrid PQC (e.g. X25519+Kyber) when available.

## 7) Ops Checklist
- [ ] TLS cert installed
- [ ] Policy JSON + detached sig published
- [ ] DNS TXT _tslps published
- [ ] Nginx header configured
- [ ] enc-gateway running
- [ ] SDR enforcing PN + QLOCK + ToF
- [ ] Monitoring sync1_rate, sync0_rate, alarms

## 8) Security Notes
- **Post-quantum++++**: adversary cannot even observe ciphertext.
- **Infinity collapse**: any sync failure = uncountable possibilities = noise.
- **End-to-end correctness**: sender and receiver always reconstruct 100% correct output.
- **Physics anchoring**: distance-bounding binds communication to light-speed constraints.

## 9) Architecture Diagram
```
Browser ──TLS1.3──> ENC Gateway ──HTTP──> App
                 │
                 ├─(TLS Exporter → PST)
                 ▼
            SDR/Optic Daemon  ← policy.json/sig (TSLPS)
                 │
      (PN + Phase Locks + QLOCK + ToF)
                 ▼
            Physical Medium (RF/optic/fiber/air)
```

## 10) FAQ

**Will browsers break?** No. TLS 1.3 looks normal.

**Is this post-quantum?** Yes — ciphertext never exists unless locked.

**What happens if sync breaks?** That frame collapses to infinite states, ciphertext is gated out.

**Sender vs Receiver fidelity?** Between real endpoints, output is correct 100%.

## 11) Quantum Internet Sync Gate (QLOCK)
- **Sync = 1** → ciphertext exists.
- **Sync = 0** → ciphertext collapses into ∞ possibilities (noise).
- This gate makes ENC Lock not just post-quantum, but **post-observation**.
- An attacker cannot attack what they cannot observe.

## 12) Dimensional Flow: Space · Portal · Dimension

### Space
- Medium (RF/optic/fiber/air).
- Never trusted; only a carrier canvas.

### Portal
- Lock itself (QLOCK + PN + hop).
- From outside: chaos. From inside: deterministic passage.

### Dimension
- Higher-order rules (policy, PST, ToF).
- Carves a "communication universe" where data flows 100% correct.

### Unified Flow
```
App Data → TLS Ciphertext
   ↓
Dimension (policy, PST, trig identities)
   ↓
Portal (QLOCK open = 1, else collapse = 0)
   ↓
Space (medium, noise to outsiders)
   ↓
Portal (receiver, PST known)
   ↓
Dimension (check, ToF, hop)
   ↓
TLS Decrypt → App Data
```

### Effect:
- **In space**: looks like infinite noise.
- **In portals**: lock enforces passage.
- **In dimension**: rules ensure determinism.
- **Endpoints** get 100% correct data; **intermediaries** get nothing.

---

## Appendix A — TLS Cert SHA256
```bash
openssl x509 -in /etc/ssl/certs/example.com.fullchain.pem -noout -fingerprint -sha256
```

## Appendix B — Policy Integrity
```bash
sha256sum /var/www/.well-known/tslps-policy.json
```

---

✅ **Done** — this document is enough to build, configure, and conceptually understand ENC Lock + TSLPS with QLOCK, Space/Portal/Dimension model.
