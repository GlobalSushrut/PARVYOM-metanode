# 🖥️ BPCI Enterprise CLI Usage Guide - Complete Command Reference

**Master the Pravyom CLI**: Complete guide to using all CLI commands for the BPCI Enterprise platform based on the real implementation.

---

## 🎯 **What You'll Learn**

- Complete CLI command reference for all 19 components
- Real command examples with actual usage patterns
- Advanced CLI features and options
- Troubleshooting and debugging commands
- Production deployment workflows

---

## 🚀 **CLI Overview**

The Pravyom CLI provides complete command-line access to all BPCI Enterprise components:

```bash
# Display the beautiful Pravyom logo and help
cargo run --bin bpci-server -- --help

# Basic command structure
cargo run --bin bpci-server -- [GLOBAL_OPTIONS] <COMMAND> [COMMAND_OPTIONS]
```

### **Global Options**
```bash
-v, --verbose          Enable verbose logging
-c, --config <FILE>    Configuration file path (default: config.toml)
-n, --network <NET>    Network selection (testnet, mainnet, localnet)
--format <FORMAT>      Output format (json or human) [default: human]
--dry-run             Dry run mode (show what would happen)
```

---

## 📋 **Complete Command Reference**

### **🔧 Component 1: Wallet Management**

#### **Create Wallets**
```bash
# Create a basic DockLock wallet
cargo run --bin bpci-server -- wallet create --name "my-wallet" --wallet-type docklock

# Create a bank-stamped wallet
cargo run --bin bpci-server -- wallet create --name "bank-wallet" --wallet-type bank --key-type ed25519

# Create a government wallet
cargo run --bin bpci-server -- wallet create --name "gov-wallet" --wallet-type government
```

#### **Wallet Operations**
```bash
# List all wallets
cargo run --bin bpci-server -- wallet list

# List wallets by type with details
cargo run --bin bpci-server -- wallet list --wallet-type bank --detailed

# Check wallet status
cargo run --bin bpci-server -- wallet status my-wallet

# Check wallet balance
cargo run --bin bpci-server -- wallet balance my-wallet

# Check specific token balance
cargo run --bin bpci-server -- wallet balance my-wallet --token GEN
```

#### **Wallet Security**
```bash
# Backup wallet
cargo run --bin bpci-server -- wallet backup my-wallet --output ./backup.json

# Restore wallet
cargo run --bin bpci-server -- wallet restore --backup ./backup.json

# Export wallet
cargo run --bin bpci-server -- wallet export my-wallet --format json

# Import wallet
cargo run --bin bpci-server -- wallet import --file ./wallet.json
```

#### **Wallet Transactions**
```bash
# Send tokens
cargo run --bin bpci-server -- wallet send my-wallet --to recipient-address --amount 100 --token GEN

# Transaction history
cargo run --bin bpci-server -- wallet history my-wallet

# Stake tokens
cargo run --bin bpci-server -- wallet stake my-wallet --amount 1000
```

---

### **🔧 Component 2: Registry Management**

#### **Node Registration**
```bash
# Register BPI Community node
cargo run --bin bpci-server -- registry register-node \
  --node-type bpi-community \
  --identity "did:bpi:my-node" \
  --endpoints "http://localhost:8080" \
  --capabilities "container_deployment,receipt_generation"

# Register BPCI Enterprise node
cargo run --bin bpci-server -- registry register-node \
  --node-type bpci-enterprise \
  --identity "did:bpci:enterprise-node" \
  --authority-level enterprise \
  --stake 10000

# Register Bank API node
cargo run --bin bpci-server -- registry register-node \
  --node-type bank-api \
  --identity "did:bank:settlement-node" \
  --authority-level bank \
  --compliance-level high
```

#### **Registry Operations**
```bash
# List all registered nodes
cargo run --bin bpci-server -- registry list-nodes

# List nodes by type
cargo run --bin bpci-server -- registry list-nodes --node-type bpi-community

# Get node details
cargo run --bin bpci-server -- registry get-node my-node-id

# Update node information
cargo run --bin bpci-server -- registry update-node my-node-id --endpoints "http://new-endpoint:8080"
```

#### **Identity Management**
```bash
# Create identity proof
cargo run --bin bpci-server -- registry create-identity \
  --did "did:bpi:my-identity" \
  --verification-level basic \
  --crypto-proof "ed25519:signature"

# Verify identity
cargo run --bin bpci-server -- registry verify-identity did:bpi:my-identity

# List identities
cargo run --bin bpci-server -- registry list-identities
```

---

### **🔧 Component 3: Mining Operations**

#### **Mining Management**
```bash
# Start mining session
cargo run --bin bpci-server -- mining start --miner-id my-miner --hashpower 1000

# Stop mining session
cargo run --bin bpci-server -- mining stop my-session-id

# List active mining sessions
cargo run --bin bpci-server -- mining list-sessions

# Get mining session status
cargo run --bin bpci-server -- mining session-status my-session-id
```

#### **Mining Performance**
```bash
# Get mining statistics
cargo run --bin bpci-server -- mining stats

# Get miner performance
cargo run --bin bpci-server -- mining performance my-miner-id

# Get hashpower distribution
cargo run --bin bpci-server -- mining hashpower-distribution

# Calculate mining rewards
cargo run --bin bpci-server -- mining calculate-rewards my-session-id
```

#### **Work Proof Management**
```bash
# Submit work proof
cargo run --bin bpci-server -- mining submit-proof \
  --session-id my-session-id \
  --work-proof "proof-data" \
  --resource-usage "cpu:1000,memory:512MB"

# Verify work proof
cargo run --bin bpci-server -- mining verify-proof proof-id

# List work proofs
cargo run --bin bpci-server -- mining list-proofs --miner-id my-miner-id
```

---

### **🔧 Component 4: Governance Operations**

#### **Proposal Management**
```bash
# Create governance proposal
cargo run --bin bpci-server -- governance create-proposal \
  --title "Increase Mining Rewards" \
  --description "Proposal to increase mining rewards by 10%" \
  --proposal-type economic \
  --voting-period 7d

# List proposals
cargo run --bin bpci-server -- governance list-proposals

# Get proposal details
cargo run --bin bpci-server -- governance get-proposal proposal-id

# Vote on proposal
cargo run --bin bpci-server -- governance vote proposal-id --vote approve --justification "Supports network growth"
```

#### **Governance Participation**
```bash
# Register as governance participant
cargo run --bin bpci-server -- governance register-participant \
  --participant-id my-participant \
  --stake 1000 \
  --reputation 0.8

# Get voting power
cargo run --bin bpci-server -- governance voting-power my-participant

# Get governance statistics
cargo run --bin bpci-server -- governance stats

# Get participation history
cargo run --bin bpci-server -- governance participation-history my-participant
```

#### **Treasury Management**
```bash
# Get treasury status
cargo run --bin bpci-server -- governance treasury-status

# Propose treasury allocation
cargo run --bin bpci-server -- governance propose-allocation \
  --infrastructure-percent 75 \
  --economy-percent 25

# Get treasury history
cargo run --bin bpci-server -- governance treasury-history
```

---

### **🔧 Component 5: Network Management**

#### **Network Operations**
```bash
# Get network status
cargo run --bin bpci-server -- network status

# List connected peers
cargo run --bin bpci-server -- network list-peers

# Connect to peer
cargo run --bin bpci-server -- network connect-peer --endpoint "http://peer:8080"

# Disconnect from peer
cargo run --bin bpci-server -- network disconnect-peer peer-id
```

#### **Network Monitoring**
```bash
# Get network health
cargo run --bin bpci-server -- network health

# Get network topology
cargo run --bin bpci-server -- network topology

# Monitor network traffic
cargo run --bin bpci-server -- network monitor-traffic

# Get network statistics
cargo run --bin bpci-server -- network stats
```

---

### **🔧 Component 6: Notary Services**

#### **Notary Operations**
```bash
# Start notary service
cargo run --bin bpci-server -- notary start --notary-id my-notary

# Stop notary service
cargo run --bin bpci-server -- notary stop my-notary

# List active notaries
cargo run --bin bpci-server -- notary list

# Get notary status
cargo run --bin bpci-server -- notary status my-notary
```

#### **Verification Services**
```bash
# Submit verification request
cargo run --bin bpci-server -- notary verify \
  --data "data-to-verify" \
  --signature "signature" \
  --public-key "public-key"

# Get verification result
cargo run --bin bpci-server -- notary verification-result request-id

# List verification requests
cargo run --bin bpci-server -- notary list-verifications

# Create audit trail
cargo run --bin bpci-server -- notary create-audit-trail \
  --operation "container-deployment" \
  --details "Deployed nginx container"
```

---

### **🔧 Component 7: Maintenance Operations**

#### **System Maintenance**
```bash
# Get system health
cargo run --bin bpci-server -- maintenance health

# Run system diagnostics
cargo run --bin bpci-server -- maintenance diagnostics

# Schedule maintenance task
cargo run --bin bpci-server -- maintenance schedule \
  --task "database-cleanup" \
  --schedule "0 2 * * *"

# List maintenance tasks
cargo run --bin bpci-server -- maintenance list-tasks
```

#### **Performance Monitoring**
```bash
# Get performance metrics
cargo run --bin bpci-server -- maintenance metrics

# Monitor resource usage
cargo run --bin bpci-server -- maintenance monitor-resources

# Get system logs
cargo run --bin bpci-server -- maintenance logs --level info --lines 100

# Clean up system
cargo run --bin bpci-server -- maintenance cleanup --dry-run
```

---

### **🔧 Component 8: Web Interface Operations**

#### **Web Server Management**
```bash
# Start web server
cargo run --bin bpci-server -- web start --port 8081

# Stop web server
cargo run --bin bpci-server -- web stop

# Get web server status
cargo run --bin bpci-server -- web status

# Reload web configuration
cargo run --bin bpci-server -- web reload-config
```

#### **API Management**
```bash
# List API endpoints
cargo run --bin bpci-server -- web list-endpoints

# Get API statistics
cargo run --bin bpci-server -- web api-stats

# Test API endpoint
cargo run --bin bpci-server -- web test-endpoint /api/status

# Generate API documentation
cargo run --bin bpci-server -- web generate-docs --output ./api-docs
```

---

### **🔧 Component 9: CueDB Operations**

#### **Database Management**
```bash
# Initialize CueDB
cargo run --bin bpci-server -- cuedb init --db-name my-database

# Create database instance
cargo run --bin bpci-server -- cuedb create \
  --db-id my-db \
  --storage-backend multicloud \
  --replication-factor 3

# List databases
cargo run --bin bpci-server -- cuedb list

# Get database status
cargo run --bin bpci-server -- cuedb status my-db
```

#### **Data Operations**
```bash
# Store data
cargo run --bin bpci-server -- cuedb store \
  --db-id my-db \
  --key "my-key" \
  --data "my-data"

# Retrieve data
cargo run --bin bpci-server -- cuedb get --db-id my-db --key "my-key"

# Delete data
cargo run --bin bpci-server -- cuedb delete --db-id my-db --key "my-key"

# Query data
cargo run --bin bpci-server -- cuedb query --db-id my-db --query "SELECT * FROM data"
```

---

### **🔧 Component 10: Cross-System Integration**

#### **Bridge Operations**
```bash
# Initialize Court-BPI Bridge
cargo run --bin bpci-server -- cross-system init-bridge \
  --bridge-id court-bpi \
  --source-system court \
  --target-system bpi

# Send cross-system message
cargo run --bin bpci-server -- cross-system send-message \
  --bridge-id court-bpi \
  --message-type "logblock-submission" \
  --payload "message-data"

# Get bridge statistics
cargo run --bin bpci-server -- cross-system bridge-stats court-bpi

# List active bridges
cargo run --bin bpci-server -- cross-system list-bridges
```

#### **Audit System Operations**
```bash
# Create unified audit entry
cargo run --bin bpci-server -- cross-system create-audit \
  --system "ENC-to-BPI" \
  --operation "logblock-submission" \
  --details "LogBlock height: 42"

# Query audit trails
cargo run --bin bpci-server -- cross-system query-audit \
  --system "ENC-to-BPI" \
  --from "2024-01-01" \
  --to "2024-01-31"

# Validate audit integrity
cargo run --bin bpci-server -- cross-system validate-audit
```

---

### **🔧 Component 11: Orchestration Engine**

#### **Cluster Management**
```bash
# Initialize metanode cluster
cargo run --bin bpci-server -- orchestration init-cluster \
  --cluster-id my-cluster \
  --nodes "node1,node2,node3"

# Deploy CUE agreement
cargo run --bin bpci-server -- orchestration deploy-agreement \
  --agreement-file "./my-app.cueyaml" \
  --cluster-id my-cluster

# List active agreements
cargo run --bin bpci-server -- orchestration list-agreements

# Get cluster status
cargo run --bin bpci-server -- orchestration cluster-status my-cluster
```

#### **Policy Management**
```bash
# Create jurisdiction policy
cargo run --bin bpci-server -- orchestration policy create-policy \
  --jurisdiction "US" \
  --policy-file "./us-policy.yaml"

# List policies
cargo run --bin bpci-server -- orchestration policy list-policies

# Validate compliance
cargo run --bin bpci-server -- orchestration policy validate-compliance \
  --node-id my-node \
  --jurisdiction "US"

# Test policy distribution
cargo run --bin bpci-server -- orchestration policy test-distribution
```

---

### **🔧 Component 12: Mother Coin Distribution**

#### **GEN Coin Management**
```bash
# Initialize GEN distribution
cargo run --bin bpci-server -- mother-coin init \
  --total-supply 1000000 \
  --distribution-rounds 4

# Create distribution round
cargo run --bin bpci-server -- mother-coin create-round \
  --round-id "seed-round" \
  --allocation 250000 \
  --price-per-token 0.10

# List distribution rounds
cargo run --bin bpci-server -- mother-coin list-rounds

# Get distribution status
cargo run --bin bpci-server -- mother-coin status
```

#### **Investor Management**
```bash
# Register investor
cargo run --bin bpci-server -- mother-coin register-investor \
  --investor-id "investor-001" \
  --wallet-id "investor-wallet" \
  --kyc-status "verified"

# Process investment
cargo run --bin bpci-server -- mother-coin process-investment \
  --investor-id "investor-001" \
  --amount 10000 \
  --round-id "seed-round"

# Get investor status
cargo run --bin bpci-server -- mother-coin investor-status "investor-001"
```

---

### **🔧 Component 13: Wallet Registry System**

#### **Wallet Registration**
```bash
# Register wallet with mandatory ID
cargo run --bin bpci-server -- wallet-registry register \
  --wallet-id "my-wallet" \
  --registration-id "REG-001" \
  --stakeholder-type "community" \
  --compliance-level "standard"

# List registered wallets
cargo run --bin bpci-server -- wallet-registry list

# Get wallet registration status
cargo run --bin bpci-server -- wallet-registry status "my-wallet"

# Update registration
cargo run --bin bpci-server -- wallet-registry update \
  --wallet-id "my-wallet" \
  --compliance-level "enhanced"
```

#### **Compliance Management**
```bash
# Check compliance status
cargo run --bin bpci-server -- wallet-registry check-compliance "my-wallet"

# Generate compliance report
cargo run --bin bpci-server -- wallet-registry compliance-report \
  --from "2024-01-01" \
  --to "2024-01-31"

# Validate registration
cargo run --bin bpci-server -- wallet-registry validate "my-wallet"
```

---

### **🔧 Component 14: Internal Governance**

#### **Treasury Distribution**
```bash
# Get treasury distribution status
cargo run --bin bpci-server -- internal-governance treasury-status

# Configure distribution
cargo run --bin bpci-server -- internal-governance configure-distribution \
  --infrastructure-percent 75 \
  --economy-percent 25 \
  --automated true

# Execute distribution
cargo run --bin bpci-server -- internal-governance execute-distribution

# Get distribution history
cargo run --bin bpci-server -- internal-governance distribution-history
```

#### **Community Tickets**
```bash
# Create community ticket
cargo run --bin bpci-server -- internal-governance create-ticket \
  --title "Feature Request" \
  --description "Add new API endpoint" \
  --priority "medium"

# List tickets
cargo run --bin bpci-server -- internal-governance list-tickets

# Update ticket status
cargo run --bin bpci-server -- internal-governance update-ticket \
  --ticket-id "TICKET-001" \
  --status "in-progress"
```

---

### **🔧 Advanced CLI Features**

#### **JSON Output Mode**
```bash
# Get JSON output for any command
cargo run --bin bpci-server -- --format json wallet list

# Pipe JSON to jq for processing
cargo run --bin bpci-server -- --format json governance stats | jq '.voting_power'

# Save JSON output to file
cargo run --bin bpci-server -- --format json registry list-nodes > nodes.json
```

#### **Dry Run Mode**
```bash
# Test commands without execution
cargo run --bin bpci-server -- --dry-run wallet create --name "test-wallet"

# Preview governance proposal
cargo run --bin bpci-server -- --dry-run governance create-proposal \
  --title "Test Proposal" \
  --description "Test description"
```

#### **Network Selection**
```bash
# Use testnet (default)
cargo run --bin bpci-server -- --network testnet wallet list

# Use mainnet
cargo run --bin bpci-server -- --network mainnet wallet list

# Use local development network
cargo run --bin bpci-server -- --network localnet wallet list
```

#### **Configuration Management**
```bash
# Use custom config file
cargo run --bin bpci-server -- --config ./custom-config.toml wallet list

# Verbose logging
cargo run --bin bpci-server -- --verbose wallet create --name "debug-wallet"
```

---

### **🔧 System Status and Initialization**

#### **System Status**
```bash
# Get comprehensive system status
cargo run --bin bpci-server -- status

# JSON format system status
cargo run --bin bpci-server -- --format json status
```

#### **System Initialization**
```bash
# Initialize BPCI system
cargo run --bin bpci-server -- init

# Force initialization (overwrite existing)
cargo run --bin bpci-server -- init --force
```

---

## 🚀 **Common Workflows**

### **Complete Node Setup Workflow**
```bash
# 1. Initialize system
cargo run --bin bpci-server -- init

# 2. Create wallet
cargo run --bin bpci-server -- wallet create --name "node-wallet" --wallet-type community

# 3. Register node
cargo run --bin bpci-server -- registry register-node \
  --node-type bpi-community \
  --identity "did:bpi:my-node" \
  --endpoints "http://localhost:8080"

# 4. Start mining
cargo run --bin bpci-server -- mining start --miner-id my-miner --hashpower 1000

# 5. Start web server
cargo run --bin bpci-server -- web start --port 8081

# 6. Check status
cargo run --bin bpci-server -- status
```

### **Governance Participation Workflow**
```bash
# 1. Register as participant
cargo run --bin bpci-server -- governance register-participant \
  --participant-id my-participant \
  --stake 1000

# 2. Create proposal
cargo run --bin bpci-server -- governance create-proposal \
  --title "Network Upgrade" \
  --description "Upgrade network protocol" \
  --proposal-type technical

# 3. Vote on proposals
cargo run --bin bpci-server -- governance vote proposal-id --vote approve

# 4. Check treasury status
cargo run --bin bpci-server -- governance treasury-status
```

### **Enterprise Integration Workflow**
```bash
# 1. Create enterprise wallet
cargo run --bin bpci-server -- wallet create --name "enterprise-wallet" --wallet-type enterprise

# 2. Register enterprise node
cargo run --bin bpci-server -- registry register-node \
  --node-type bpci-enterprise \
  --authority-level enterprise \
  --stake 10000

# 3. Deploy CUE agreement
cargo run --bin bpci-server -- orchestration deploy-agreement \
  --agreement-file "./enterprise-app.cueyaml"

# 4. Monitor performance
cargo run --bin bpci-server -- maintenance metrics
```

---

## 🆘 **Troubleshooting Commands**

### **Debug Information**
```bash
# Verbose logging for any command
cargo run --bin bpci-server -- --verbose <command>

# System diagnostics
cargo run --bin bpci-server -- maintenance diagnostics

# Network health check
cargo run --bin bpci-server -- network health

# Check configuration
cargo run --bin bpci-server -- --config ./config.toml status
```

### **Common Issues**
```bash
# Check if services are running
cargo run --bin bpci-server -- status

# Validate configuration
cargo run --bin bpci-server -- --dry-run init

# Test network connectivity
cargo run --bin bpci-server -- network list-peers

# Check wallet balance issues
cargo run --bin bpci-server -- wallet balance <wallet-id> --verbose
```

---

## 🔗 **Next Steps**

Now that you've mastered the CLI:

1. **[Production Deployment](../backup_md_files/coredocs/PRODUCTION_DEPLOYMENT_GUIDE.md)** - Deploy in production
2. **[Advanced Integration](../backup_md_files/coredocs/ADVANCED_INTEGRATION_GUIDE.md)** - Custom integrations
3. **[Monitoring & Maintenance](../backup_md_files/coredocs/MONITORING_GUIDE.md)** - Production monitoring

---

**🎉 Congratulations! You've mastered the complete Pravyom CLI!**

*You now have complete command-line control over all 19 metanode components and can operate the entire BPCI Enterprise platform from the terminal.*
