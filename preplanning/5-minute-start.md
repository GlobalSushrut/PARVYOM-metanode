# BPI Mesh — 5-Minute Start Guide
Get from zero to your first verifiable Web3 app in under 5 minutes.

---

## What is BPI Mesh?
**Web3 security for Web2 apps.** Run your existing Docker/K8s apps unchanged, but get blockchain-grade finality, receipts, and data availability.

- ✅ **No rewrite needed** — your Node.js, Python, Go apps work as-is
- ✅ **Sub-second finality** — faster than most databases
- ✅ **Verifiable receipts** — cryptographic proof of every request
- ✅ **One file, one command** — `bpicompose.yml` + `bpi up`

---

## Install (60 seconds)

### macOS/Linux
```bash
curl -sSL https://get.bpi.run | bash
```

### Windows
Download and run: [bpi-installer.msi](https://releases.bpi.run/latest/bpi-installer.msi)

### Verify install
```bash
bpi --version
# Should show: bpi 0.1.0
```

---

## Your First App (3 minutes)

### 1. Initialize project
```bash
bpi init --template node-api --name my-first-app
cd my-first-app
```

This creates:
- `bpicompose.yml` — your app config (like docker-compose.yml)
- `services/api/` — a sample Node.js API
- `agreements/basic.yaml` — security policy (10 lines)

### 2. Start everything
```bash
bpi up
```

You'll see:
```
🚀 Starting BPI devnet...
✅ 3 validators running
✅ Relay network active  
✅ API service deployed
📊 Dashboard: http://localhost:3000
🔗 Ready! Try: curl -s "mainnet://registry.submit" -d '{"ping":"mesh"}'
```

### 3. Test your app
```bash
curl -s "mainnet://registry.submit" -d '{"amount":420}' | jq
```

Response:
```json
{
  "id": "req_1699123456_abc123",
  "amount": 420,
  "status": "accepted",
  "processed_at": "2024-01-15T10:30:45Z"
}
```

### 4. Verify it's really Web3
```bash
bpi verify
```

Output:
```
🔍 Verifying BPI chain health...
Block 1 ✅ Chain ✅ BLS ✅ PoH 
Block 2 ✅ Chain ✅ BLS ✅ PoH 
Block 3 ✅ Chain ✅ BLS ✅ PoH 
✅ Chain verification complete
```

**🎉 Congratulations!** Your API now has:
- **Finality** in ~300ms (faster than most databases)
- **Cryptographic receipts** for every request
- **Byzantine fault tolerance** (works even if 1/3 of validators fail)

---

## What Just Happened?

1. **Your API runs unchanged** — same Node.js code, same endpoints
2. **BPI wraps it** — adds consensus, receipts, and verification
3. **Validators finalize** — 3 nodes agree on every request/response
4. **You get proofs** — every API call is cryptographically verifiable

The magic URL `mainnet://registry.submit` routes through BPI's mesh network instead of regular HTTP.

---

## Next Steps (Pick Your Adventure)

### 🛡️ Add Security Policies
```bash
bpi policy wizard
# Answer 6 questions → get custom agreement
bpi agreement pin
bpi verify
```

### 📋 See Receipts
```bash
bpi receipts toggle true
# Make another API call
curl -s "mainnet://registry.submit" -d '{"amount":1000}'
bpi receipts get <request-id>
```

### ☁️ Connect to Testnet
```bash
bpi k8 connect --testnet https://testnet.bpi.dev
bpi verify --last 50
```

### 🚀 Try Other Templates
```bash
bpi templates list
bpi init --template react-spa --name my-frontend
bpi init --template ai-inference --name my-ai-app
```

---

## Common Issues & Fixes

### "Docker daemon not running"
```bash
# macOS: Start Docker Desktop
# Linux: sudo systemctl start docker
# Or auto-fix: bpi doctor --autofix
```

### "Port 8080 already in use"
```bash
# Find what's using it: lsof -i :8080
# Or use different port: bpi up --port 8081
```

### "Validators not starting"
```bash
bpi doctor
# Shows detailed diagnostics + fixes
```

### Need help?
- 📖 **Full docs**: https://docs.bpi.dev
- 💬 **Discord**: https://discord.gg/bpi-mesh (response <10min)
- 🐛 **Issues**: https://github.com/bpi-mesh/bpi/issues

---

## What Makes This Different?

| Traditional API | BPI Mesh API |
|---|---|
| ❌ No finality guarantees | ✅ Sub-second finality |
| ❌ No audit trail | ✅ Cryptographic receipts |
| ❌ Single point of failure | ✅ Byzantine fault tolerant |
| ❌ Trust the server | ✅ Verify everything |
| ❌ Complex blockchain setup | ✅ One command: `bpi up` |

---

## Real-World Use Cases

### 🏦 **Financial APIs**
- Every transaction gets a receipt
- Regulators can verify all activity
- No single point of failure

### 🤖 **AI Inference**
- Prove AI outputs weren't tampered with
- Audit model behavior over time
- Content safety policies enforced

### 📄 **Document Processing**
- Immutable audit trail
- Prove when documents were processed
- GDPR-compliant redaction

### 🎮 **Gaming**
- Verifiable random number generation
- Cheat-proof leaderboards
- Transparent item drops

---

## Under the Hood (For the Curious)

- **Consensus**: IBFT with BLS signatures (sub-second finality)
- **Transport**: QUIC/HTTP3 with anti-eclipse routing
- **Receipts**: Deterministic execution with witness logs
- **Agreements**: YAML policies compiled to WASM
- **Data Availability**: Reed-Solomon encoding with sampling
- **Languages**: Rust core + TypeScript SDK

But you don't need to know any of this to use it! 🎯

---

## Ready to Ship?

Your 5-minute demo app is production-ready:
- ✅ Consensus and finality
- ✅ Cryptographic receipts  
- ✅ Policy enforcement
- ✅ Health monitoring
- ✅ Docker containers

Scale up when you're ready:
```bash
bpi mainnet scale --validators 9
bpi da enable
bpi anchors enable --chains ethereum,polygon
```

**Welcome to Web 3.5** — where Web2 UX meets Web3 trust. 🌐
