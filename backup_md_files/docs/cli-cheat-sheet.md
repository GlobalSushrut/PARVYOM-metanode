# Metanode CLI Cheat Sheet - Production Ready

## 🚀 Quick Start (Copy-Paste Demo)

```bash
# Day-0 Demo (Complete BankCoin + BISO workflow)
metanode bank register --name "BRICS Bank A" --jurisdiction BR
metanode bank por run --fiat BRL --gold LBMA --publish
metanode coin issue --type mother
metanode coin activate <coin-id> --job <rid>
metanode settle xborder --from INR --to USD --amount 50000 --via gold --receipt
metanode receipt verify <rid> --json
```

## 🏦 Banking & Finance

| Command | Description |
|---------|-------------|
| `metanode bank register --name "Bank" --jurisdiction US` | Register validator bank |
| `metanode bank por run --fiat USD --gold COMEX --publish` | Run proof of reserves |
| `metanode settle xborder --from EUR --to JPY --via gold` | Cross-border via gold |
| `metanode coin redeem --fiat USD --amount 1000` | Local redemption |

## 🪙 Coin Lifecycle

| Command | Description |
|---------|-------------|
| `metanode coin issue --type mother\|branch\|leaf` | Issue new coin |
| `metanode coin activate <id> --job <rid>` | Activate with PoE job |
| `metanode coin status <id>` | Check coin status |
| `metanode coin lineage <id> --tree-view` | Show ancestry tree |

## 📊 Economics & PoE

| Command | Description |
|---------|-------------|
| `metanode economics poe --show` | Show PoE index (Φ) |
| `metanode economics issue-window --preview` | Minting capacity |
| `metanode analytics poe --by-epoch --heatmap` | PoE analytics |
| `metanode coin heatmap --by-ancestry` | Coin health map |

## 🛡️ BISO & Compliance

| Command | Description |
|---------|-------------|
| `metanode biso lint policy.hcl` | Validate policy |
| `metanode biso apply policy.hcl` | Deploy policy |
| `metanode biso dashboard --red-yellow-green-counts` | Traffic light status |
| `metanode security posture` | Overall security |

## 🏛️ Governance

| Command | Description |
|---------|-------------|
| `metanode gov propose set-threshold --tau1 100` | Adjust PoE thresholds |
| `metanode gov vote <id> for --reason "..."` | Vote on proposal |
| `metanode gov stats` | Governance analytics |

## 🧾 Receipts & Attestations

| Command | Description |
|---------|-------------|
| `metanode receipt verify <id> --json` | Verify receipt |
| `metanode receipt export --attestation ssa` | Export attestation |
| `metanode attest publish --type por --signed` | Publish PoR attestation |

## 🌐 Testnet & Development

| Command | Description |
|---------|-------------|
| `metanode testnet connect` | Connect to testnet |
| `metanode testnet faucet request <address>` | Get test tokens |
| `metanode testnet deploy contract.wasm` | Deploy contract |

## 🚀 App & Cluster Management

| Command | Description |
|---------|-------------|
| `metanode deploy app <name>` | Deploy application |
| `metanode cluster create <name> --nodes 5` | Create cluster |
| `metanode oci cluster create --provider aws` | OCI cluster |
| `metanode dashboard open --service banking` | Launch dashboard |

## 🔧 Universal Flags

| Flag | Description |
|------|-------------|
| `--dry-run` | Preview without executing |
| `--yes` | Skip confirmations |
| `--json` | Machine-readable output |
| `--paginate --limit 50` | Pagination |

## 📋 Essential Environment Variables

```bash
export METANODE_NETWORK=mainnet
export METANODE_PROFILE=production
export METANODE_API_KEY=<your-key>
export METANODE_LOG_LEVEL=info
```

## 🆘 Quick Help

| Command | Description |
|---------|-------------|
| `metanode help` | General help |
| `metanode help bank` | Banking commands |
| `metanode examples list` | Built-in examples |
| `metanode version --json` | Version info |

## 🎯 Golden Paths

### First Validator Bank
```bash
metanode bank register --name "My Bank" --jurisdiction US
metanode bank por run --fiat USD --gold COMEX --publish
metanode coin issue --type mother
metanode coin activate <coin-id> --job <rid>
```

### Cross-Border Payment
```bash
metanode settle xborder --from EUR --to USD --amount 10000 --via gold --receipt
metanode receipt verify <receipt-id> --json
```

### BISO Quickstart
```bash
metanode biso create --template gdpr --jurisdiction EU
metanode biso lint policy.hcl
metanode biso apply policy.hcl
metanode biso dashboard
```

## 🔍 Monitoring & Observability

| Command | Description |
|---------|-------------|
| `metanode bank por-status --failures-only` | PoR monitoring |
| `metanode analytics poe --by-epoch` | PoE trends |
| `metanode security posture` | Security overview |
| `metanode health check --comprehensive` | System health |

## 📊 Exit Codes

- **0**: Success
- **1**: User error (invalid args)
- **2**: Network error
- **3**: Policy failure (BISO)
- **4**: Validation error
- **5**: Internal error

---

**🎉 That's it! You now have the complete Metanode CLI at your fingertips.**

*For full documentation: `metanode help` or visit https://docs.metanode.io*
