# Metanode CLI Cheat Sheet

## Quick Reference - Golden Paths

### 🏦 Bank Onboarding & PoR
```bash
# Register a new validator bank
metanode bank register --name "BRICS Bank A" --jurisdiction BR

# Run proof-of-reserves for fiat and gold
metanode bank por run --fiat BRL --gold LBMA --publish

# Publish FX rates (multi-oracle)
metanode bank fx publish --pairs USD/BRL,EUR/BRL --oracles 3
```

### 🪙 Coin Lifecycle (Empty → Active → Tree)
```bash
# Issue a mother coin (governance-gated)
metanode coin issue --type mother

# Activate coin with PoE job
metanode coin activate <coin-id> --job <rid>

# Check coin status and lineage
metanode coin status <coin-id>
metanode coin lineage <coin-id> --tree-view

# Redeem coin locally
metanode coin redeem <coin-id> --fiat USD
```

### 💸 Cross-Border Settlement
```bash
# Gold-backed settlement (one command)
metanode settle xborder --from INR --to USD --amount 50000 --via gold --receipt

# Check settlement status
metanode settle status <settlement-id>

# Marketing alias
metanode pay --to 0x123... --amount 1000 --currency USD
```

### 📋 BISO Compliance
```bash
# Lint BISO policies
metanode biso lint --policy gdpr,hipaa

# Apply compliance rules
metanode biso apply --region EU --classification PII

# Check compliance diff
metanode biso diff --before <hash> --after <hash>
```

### 🧾 Receipts & Attestations
```bash
# Export receipt with attestation
metanode receipt export <rid> --attestation ssa,zk

# Verify receipt integrity
metanode receipt verify <rid> --json

# Publish attestation
metanode attest publish --type por --data <hash>
```

### 📊 Economics & PoE Analytics
```bash
# Show PoE components and thresholds
metanode economics poe --show

# Preview issue window (minting capacity)
metanode economics issue-window --preview

# Analytics with heatmaps
metanode analytics poe --by-epoch --heatmap
metanode coin heatmap --by-ancestry
```

### 🗳️ Governance
```bash
# Propose parameter change
metanode gov propose set-threshold --tau1 100 --tau2 250

# Vote on proposal
metanode gov vote <proposal-id> --vote yes --reason "Economic stability"

# Check governance status
metanode gov proposals --status active
metanode gov delegates --top 10
```

### 🧪 Testnet Operations
```bash
# Request testnet tokens
metanode testnet faucet request --address 0x123... --amount 1000

# Check faucet status
metanode testnet faucet status --address 0x123...

# Reset testnet state
metanode testnet reset --confirm
```

### 🚀 Application Deployment
```bash
# Deploy application
metanode deploy app --name myapp --image nginx:latest --replicas 3

# Manage applications
metanode app list
metanode app status myapp
metanode app scale myapp --replicas 5
metanode app logs myapp --follow
```

### 🔗 Cluster Management
```bash
# Initialize cluster
metanode cluster init --name production

# Join cluster
metanode cluster join --token <token> --endpoint <url>

# Check cluster health
metanode cluster health --detailed
metanode cluster nodes --status
```

### 🌐 Mesh Networking
```bash
# Connect to mesh
metanode mesh connect --peer <peer-id>

# Check mesh status
metanode mesh status --topology
metanode mesh peers --latency
```

## Global Flags

| Flag | Description | Example |
|------|-------------|---------|
| `--dry-run` | Preview without executing | `metanode --dry-run bank register ...` |
| `--json` | JSON output format | `metanode --json coin status <id>` |
| `--verbose` | Detailed logging | `metanode -v cluster init` |
| `--quiet` | Suppress output | `metanode -q deploy app ...` |
| `--yes` | Auto-confirm prompts | `metanode --yes testnet reset` |
| `--no-color` | Disable colors | `metanode --no-color help` |

## Configuration

### Config File: `~/.metanode/config.yaml`
```yaml
api_endpoint: https://api.metanode.io
network: mainnet
profile: default
log_level: info
telemetry_enabled: false
```

### Environment Variables
```bash
export METANODE_API_URL="https://api.metanode.io"
export METANODE_API_TIMEOUT="30"
export METANODE_PROFILE="production"
```

## Exit Codes
- `0`: Success
- `1`: User error (invalid arguments)
- `2`: Network error (API unreachable)
- `3`: Policy failure (BISO violation)
- `4`: Validation error (invalid data)
- `5`: Internal error (CLI bug)

## Demo Workflows

### Complete Banking Workflow
```bash
# 1. Register bank
metanode bank register --name "Global Bank" --jurisdiction US

# 2. Setup PoR
metanode bank por run --fiat USD --gold COMEX --publish

# 3. Issue mother coin
metanode coin issue --type mother

# 4. Cross-border payment
metanode settle xborder --from USD --to EUR --amount 10000 --via gold

# 5. Verify receipt
metanode receipt verify <rid> --attestation zk
```

### Governance Participation
```bash
# 1. Check active proposals
metanode gov proposals --status active

# 2. Submit parameter proposal
metanode gov propose set-threshold --tau1 150 --description "Increase security"

# 3. Vote on proposals
metanode gov vote <proposal-id> --vote yes

# 4. Check voting power
metanode gov delegates --address <my-address>
```

## Help & Documentation
- `metanode help` - Main help
- `metanode <command> --help` - Command help
- `man metanode` - Manual page
- Visit: https://docs.metanode.io

## Troubleshooting
```bash
# Check CLI version
metanode --version

# Test connectivity
metanode --dry-run bank list

# View logs
tail -f ~/.metanode/cli.log

# Reset configuration
rm ~/.metanode/config.yaml && metanode help
```
