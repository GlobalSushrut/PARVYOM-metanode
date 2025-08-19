# Metanode CLI Commands Reference - Stage 53 Complete

This document provides a comprehensive reference for ALL Metanode CLI commands covering every service and capability in the ecosystem.

---

## Table of Contents

1. [Installation & Setup](#installation--setup)
2. [Project Management](#project-management)
3. [Network & Node Commands](#network--node-commands)
4. [Wallet & Account Commands](#wallet--account-commands)
5. [Governance Commands](#governance-commands)
6. [Mining & Economics Commands](#mining--economics-commands)
7. [Container Management (DockLock)](#container-management-docklock)
8. [Mesh & Networking](#mesh--networking)
9. [Monitoring & Analytics](#monitoring--analytics)
10. [Developer Tools](#developer-tools)
11. [Security & Compliance (BISO)](#security--compliance-biso)
12. [Faucet & Testnet](#faucet--testnet)
13. [Autonomous Economics](#autonomous-economics)
14. [Headers & Light Client](#headers--light-client)
15. [Data Availability](#data-availability)
16. [Receipts & Validation](#receipts--validation)
17. [Anchor & Cross-Chain](#anchor--cross-chain)
18. [Slashing & Evidence](#slashing--evidence)
19. [App Deployment & Management](#app-deployment--management)
20. [Cluster Management & Orchestration](#cluster-management--orchestration)
21. [Decentralization & Mesh Management](#decentralization--mesh-management)
22. [Dashboards & UI Management](#dashboards--ui-management)
23. [In-House & Enterprise Setup](#in-house--enterprise-setup)
24. [Advanced Automation & CI/CD](#advanced-automation--cicd)
25. [Future-Ready Extensions](#future-ready-extensions)
26. [Agreements & Legal Framework](#agreements--legal-framework)
27. [Package Management](#package-management)
28. [API & Services](#api--services)
29. [Backup & Recovery](#backup--recovery)
30. [Observability & Security Posture](#observability--security-posture)
31. [UX & Safety Polish](#ux--safety-polish)
32. [Version Management & Updates](#version-management--updates)
33. [Documentation & Help](#documentation--help)
34. [Advanced Usage](#advanced-usage)

---

## 🚀 **Installation Commands**

### **Quick Install (Recommended)**
```bash
# One-line installer (like Docker)
curl -fsSL https://get.metanode.io | sh

# Alternative: wget
wget -qO- https://get.metanode.io | sh

# Manual download and install
wget https://releases.metanode.io/latest/metanode-cli-linux-x64.tar.gz
tar -xzf metanode-cli-linux-x64.tar.gz
sudo ./install.sh
```

### **Package Manager Install**
```bash
# Ubuntu/Debian
curl -fsSL https://packages.metanode.io/gpg | sudo gpg --dearmor -o /usr/share/keyrings/metanode.gpg
echo "deb [signed-by=/usr/share/keyrings/metanode.gpg] https://packages.metanode.io/apt stable main" | sudo tee /etc/apt/sources.list.d/metanode.list
sudo apt update
sudo apt install metanode-cli

# CentOS/RHEL/Fedora
sudo dnf config-manager --add-repo https://packages.metanode.io/rpm/metanode.repo
sudo dnf install metanode-cli

# Arch Linux
yay -S metanode-cli

# macOS (Homebrew)
brew tap metanode/tap
brew install metanode-cli
```

### **Docker Install**
```bash
# Pull official image
docker pull metanode/cli:latest

# Create alias for easy use
echo 'alias metanode="docker run --rm -it -v $(pwd):/workspace metanode/cli"' >> ~/.bashrc
source ~/.bashrc

# Verify installation
metanode version
```

---

## 🎯 **Core CLI Commands**

### **Global Options**
```bash
# Available for all commands
--verbose, -v          # Enable verbose logging
--quiet, -q            # Suppress non-error output
--config PATH          # Use custom config file
--profile NAME         # Use named profile
--no-color             # Disable colored output
--json                 # Output in JSON format
--help, -h             # Show help
--version              # Show version
```

### **Help and Information**
```bash
# Show main help
metanode help
metanode --help
metanode -h

# Show command-specific help
metanode <command> --help
metanode help <command>

# Show version information
metanode version
metanode --version

# Show system information
metanode system info
metanode system requirements
metanode system check
```

---

## 🏗️ **Project Management Commands**

### **Initialize New Project**
```bash
# Basic initialization
metanode init
metanode init my-project
metanode init --name my-project

# Initialize with template
metanode init --template basic
metanode init --template banking
metanode init --template defi
metanode init --template enterprise
metanode init --template developer

# Initialize in specific directory
metanode init /path/to/project --name my-project

# Interactive initialization
metanode init --interactive
```

### **Project Configuration**
```bash
# Show current configuration
metanode config show
metanode config list
metanode config get <key>

# Set configuration values
metanode config set <key> <value>
metanode config set network.rpc_url "https://rpc.metanode.io"
metanode config set wallet.default_address "0x..."

# Configuration profiles
metanode config profile create <name>
metanode config profile switch <name>
metanode config profile list
metanode config profile delete <name>

# Reset configuration
metanode config reset
metanode config reset --all
```

---

## 🌐 **Network and Node Commands**

### **Local Development Network**
```bash
# Start local devnet
metanode up
metanode start
metanode devnet start

# Start with options
metanode up --validators 5
metanode up --detach
metanode up --clean
metanode up --port 8545

# Stop local devnet
metanode down
metanode stop
metanode devnet stop

# Stop with cleanup
metanode down --volumes
metanode down --clean
metanode down --force

# Restart devnet
metanode restart
metanode devnet restart
```

### **Node Management**
```bash
# Node status
metanode node status
metanode node info
metanode node health

# Start/stop node services
metanode node start
metanode node stop
metanode node restart

# Node logs
metanode node logs
metanode node logs --follow
metanode node logs --tail 100
metanode node logs --service consensus

# Node configuration
metanode node config show
metanode node config set <key> <value>
metanode node config reset
```

### **Network Connection**
```bash
# Connect to networks
metanode connect mainnet
metanode connect testnet
metanode connect devnet
metanode connect --rpc-url <url>

# Network status
metanode network status
metanode network info
metanode network peers
metanode network sync

# Switch networks
metanode network switch mainnet
metanode network switch testnet
metanode network list
```

---

## 🏦 **Bank & Validator Commands**

### **Bank Registration & Management**
```bash
# Register as validator bank
metanode bank register --name "BRICS Bank A" --jurisdiction BR
metanode bank register --name "Central Bank EU" --jurisdiction EU --type central
metanode bank register --interactive

# Bank information
metanode bank info
metanode bank info <bank-id>
metanode bank list
metanode bank list --jurisdiction BR
metanode bank list --type commercial

# Bank status
metanode bank status
metanode bank status <bank-id>
metanode bank health-check
```

### **Proof of Reserves (PoR)**
```bash
# Run proof of reserves
metanode bank por run --fiat BRL --gold LBMA --publish
metanode bank por run --fiat USD,EUR --auto-publish
metanode bank por run --dry-run

# Verify proof of reserves
metanode bank por verify <bank-id>
metanode bank por verify <bank-id> --attestation-type fiat
metanode bank por verify <bank-id> --attestation-type gold

# PoR status and history
metanode bank por status
metanode bank por status --failures-only
metanode bank por history
metanode bank por history <bank-id>
```

### **FX & Gold Bridge**
```bash
# Publish FX rates
metanode bank fx publish --pairs USD/EUR,USD/BRL --source reuters,bloomberg
metanode bank fx publish --gold-price LBMA --auto-update
metanode bank fx rates

# Gold bridge operations
metanode bank gold-bridge status
metanode bank gold-bridge reserves
metanode bank gold-bridge audit-trail
```

---

## 🪙 **Coin Lifecycle Commands**

### **Coin Issuance**
```bash
# Issue new coins (governance-gated)
metanode coin issue --type mother
metanode coin issue --type branch --parent <mother-coin-id>
metanode coin issue --type leaf --parent <branch-coin-id>

# Issue with parameters
metanode coin issue --type mother --amount 1000000 --purpose "Central Bank Digital Currency"
metanode coin issue --type branch --parent <id> --jurisdiction EU
```

### **Coin Activation**
```bash
# Activate coin (locks value, flips to active)
metanode coin activate <coin-id> --job <receipt-id>
metanode coin activate <coin-id> --job <receipt-id> --confirm

# Activation status
metanode coin activation-status <coin-id>
metanode coin activation-queue
```

### **Coin Status & Management**
```bash
# Coin information
metanode coin status <coin-id>
metanode coin info <coin-id>
metanode coin list
metanode coin list --type mother
metanode coin list --status active
metanode coin list --status empty

# Coin history and lineage
metanode coin history <coin-id>
metanode coin lineage <coin-id>
metanode coin lineage <coin-id> --tree-view
metanode coin ancestry <coin-id>

# Coin analytics
metanode coin heatmap --by-ancestry
metanode coin heatmap --by-jurisdiction
metanode coin prestige <coin-id>
```

### **Gift Emissions**
```bash
# Gift emissions for empty coins
metanode coin gift-emissions --epoch now
metanode coin gift-emissions --epoch <epoch-id> --dry-run
metanode coin gift-emissions status
```

### **Coin Operations**
```bash
# Redeem coins for fiat
metanode coin redeem <coin-id> --fiat USD --amount 1000
metanode coin redeem <coin-id> --gold --amount 10oz

# Transfer coins
metanode coin transfer <coin-id> <to-address> <amount>
metanode coin transfer <coin-id> <to-address> <amount> --receipt
```

---

## 💸 **Settlement Commands**

### **Cross-Border Settlement**
```bash
# Cross-border payments via gold bridge
metanode settle xborder --from INR --to USD --amount 50000 --via gold --receipt
metanode settle xborder --from BRL --to EUR --amount 25000 --bank <bank-id>

# Payment alias (marketing-friendly)
metanode pay --from INR --to USD --amount 50000 --via gold
metanode pay <to-address> <amount> --currency USD
```

### **Settlement Status**
```bash
# Settlement status
metanode settle status <settlement-id>
metanode settle history
metanode settle history --currency USD
metanode settle pending

# Settlement analytics
metanode settle analytics --by-corridor
metanode settle analytics --by-currency
metanode settle volume --period month
```

---

## 🛡️ **BISO Compliance Commands**

### **BISO Policy Management**
```bash
# Policy operations
metanode biso lint <policy-file>
metanode biso apply <policy-file>
metanode biso diff <policy-file>
metanode biso validate <policy-file>

# Policy status
metanode biso status
metanode biso policies
metanode biso policies --active
metanode biso violations

# Compliance checking
metanode biso check <data-flow>
metanode biso audit <time-range>
metanode biso compliance-report --format pdf
```

### **Traffic Light System**
```bash
# Traffic light status
metanode biso traffic-light status
metanode biso traffic-light stats
metanode biso traffic-light --region EU
metanode biso traffic-light --classification PII

# Security posture
metanode security posture
metanode security posture --red-only
metanode security posture --summary
```

---

## 📋 **Receipt & Attestation Commands**

### **Receipt Management**
```bash
# List and show receipts
metanode receipt list
metanode receipt list --type settlement
metanode receipt list --date today
metanode receipt show <receipt-id>

# Verify receipts
metanode receipt verify <receipt-id>
metanode receipt verify <receipt-id> --json
metanode receipt verify-batch <receipt-ids>

# Export receipts
metanode receipt export <receipt-id> --format json
metanode receipt export <receipt-id> --attestation ssa
metanode receipt export <receipt-id> --attestation zk
metanode receipt export-batch <receipt-ids> --format pdf
```

### **Attestations**
```bash
# Publish attestations
metanode attest publish --type por --data <por-data>
metanode attest publish --type fx --data <fx-data>
metanode attest publish --type compliance --data <compliance-data>

# Verify attestations
metanode attest verify <attestation-id>
metanode attest list
metanode attest list --type por
```

---

## 💰 **Wallet and Account Commands**

### **Wallet Management**
```bash
# Create new wallet
metanode wallet create
metanode wallet create --name my-wallet
metanode wallet create --mnemonic

# Import existing wallet
metanode wallet import --private-key <key>
metanode wallet import --mnemonic "<mnemonic>"
metanode wallet import --keystore <file>

# List wallets
metanode wallet list
metanode wallet show
metanode wallet info <address>

# Set default wallet
metanode wallet default <address>
metanode wallet switch <address>
```

### **Account Operations**
```bash
# Check balance
metanode balance
metanode balance <address>
metanode balance --token GEN
metanode balance --token NEX
metanode balance --token FLX
metanode balance --token AUR

# Send tokens
metanode send <to> <amount>
metanode send <to> <amount> --token GEN
metanode send <to> <amount> --gas-price <price>
metanode send <to> <amount> --confirm

# Transaction history
metanode history
metanode history <address>
metanode history --limit 50
metanode history --token GEN
metanode history --token GEN
```

---

## 🏛️ **Governance Commands**

### **Proposal Management**
```bash
# List proposals
metanode gov proposals
metanode gov proposals --status active
metanode gov proposals --status passed
metanode gov proposals --limit 20

# Show proposal details
metanode gov proposal <id>
metanode gov proposal <id> --votes
metanode gov proposal <id> --details

# Create proposal
metanode gov propose parameter-update \
  --parameter "job_fee_rate" \
  --value "0.015" \
  --title "Increase Job Fee Rate" \
  --description "Proposal to increase job fee rate from 1% to 1.5%"

# PoE threshold proposals (governance-grade controls)
metanode gov propose set-threshold --tau1 100 --tau2 250 \
  --title "Update PoE Thresholds" \
  --description "Adjust PoE difficulty thresholds for current market conditions"

metanode gov propose set-threshold --tau3 500 --tau4 1000 \
  --title "Set Advanced PoE Thresholds"

metanode gov propose treasury-allocation \
  --recipient "0x..." \
  --amount "10000" \
  --purpose "Developer grants"

metanode gov propose protocol-upgrade \
  --type "consensus" \
  --description "Upgrade to IBFT 2.0"

# Economics policy proposals
metanode gov propose economics-policy \
  --type "owner-salary-cap" \
  --value "1000000" \
  --description "Set owner salary cap"

metanode gov propose economics-policy \
  --type "docklock-revenue-split" \
  --owner-pct "0.2" \
  --treasury-pct "0.3"
```

### **Voting**
```bash
# Vote on proposal
metanode gov vote <proposal-id> for
metanode gov vote <proposal-id> against
metanode gov vote <proposal-id> abstain

# Vote with reason
metanode gov vote <proposal-id> for --reason "Supports network sustainability"

# Check voting power
metanode gov voting-power
metanode gov voting-power <address>

# Voting history
metanode gov votes
metanode gov votes <address>
```

### **Governance Statistics**
```bash
# Governance stats
metanode gov stats
metanode gov participation
metanode gov parameters

# Parameter history
metanode gov parameter-history <parameter>
metanode gov parameter-current <parameter>
```

---

## ⛏️ **Mining and Economics Commands**

### **PoE Mining**
```bash
# Start mining
metanode mine start
metanode mine start --threads 4
metanode mine start --address <address>

# Stop mining
metanode mine stop

# Mining status
metanode mine status
metanode mine stats
metanode mine rewards

# Mining configuration
metanode mine config show
metanode mine config set threads 8
metanode mine config set difficulty auto
```

### **Economic Statistics**
```bash
# Token supply information
metanode economics supply
metanode economics supply --token GEN
metanode economics supply --historical

# PoE index and components
metanode economics poe --show
metanode economics poe-index
metanode economics poe-index --historical
metanode economics poe-components

# Issue window and capacity
metanode economics issue-window --preview
metanode economics issue-window --epoch current
metanode economics minting-capacity

# Fee distribution
metanode economics fees
metanode economics fees --period month
metanode economics treasury
metanode economics owner-salary --status
```

### **PoE Analytics & Monitoring**
```bash
# PoE analytics
metanode analytics poe --by-epoch
metanode analytics poe --by-epoch --heatmap
metanode analytics poe --threshold-analysis
metanode analytics poe --job-distribution

# Economics monitoring
metanode economics monitor --live
metanode economics thresholds
metanode economics policy-levers
```

### **Staking**
```bash
# Stake tokens
metanode stake <amount>
metanode stake <amount> --validator <address>
metanode stake <amount> --lock-period 30d

# Unstake tokens
metanode unstake <amount>
metanode unstake <amount> --validator <address>

# Staking information
metanode stake info
metanode stake rewards
metanode stake validators
```

---

## 🐳 **DockLock Container Commands**

### **Container Management**
```bash
# List containers
metanode container list
metanode container ls
metanode container ps

# Run container
metanode container run <image>
metanode container run <image> --policy <policy>
metanode container run <image> --receipt-mode enabled

# Container operations
metanode container start <id>
metanode container stop <id>
metanode container restart <id>
metanode container remove <id>

# Container logs
metanode container logs <id>
metanode container logs <id> --follow
metanode container logs <id> --tail 100
```

### **Policy Management**
```bash
# List policies
metanode policy list
metanode policy show <name>

# Create policy
metanode policy create <name> --file <policy.hcl>
metanode policy create <name> --template basic
metanode policy create <name> --interactive

# Apply policy
metanode policy apply <name> --container <id>
metanode policy validate <file>

# Policy templates
metanode policy templates
metanode policy template <name>
```

### **Receipt System**
```bash
# List receipts
metanode receipt list
metanode receipt list --container <id>
metanode receipt list --date today

# Show receipt
metanode receipt show <id>
metanode receipt verify <id>
metanode receipt export <id> --format json

# Receipt statistics
metanode receipt stats
metanode receipt stats --period week
```

---

## 🌍 **Mesh Interaction Commands**

### **Light Client Network**
```bash
# Connect to light client network
metanode mesh connect
metanode mesh status
metanode mesh peers

# Header verification
metanode mesh verify-header <hash>
metanode mesh get-header <height>
metanode mesh sync-status

# Network statistics
metanode mesh stats
metanode mesh health
metanode mesh latency
```

### **Directory Service**
```bash
# Validator discovery
metanode directory validators
metanode directory validators --region us-east
metanode directory validators --type institutional

# Register validator
metanode directory register \
  --address <address> \
  --region us-east \
  --type individual \
  --asn 12345

# Validator information
metanode directory info <address>
metanode directory diversity-stats
```

---

## 🧪 **Testnet Commands**

### **Testnet Management**
```bash
# Testnet operations
metanode testnet start
metanode testnet stop
metanode testnet reset
metanode testnet status

# Testnet configuration
metanode testnet config
metanode testnet config --validators 5
metanode testnet config --reset
```

### **Faucet Service**
```bash
# Request testnet tokens
metanode testnet faucet request
metanode testnet faucet request <address>
metanode testnet faucet request --amount 10 --token GEN

# Faucet status
metanode testnet faucet status
metanode testnet faucet history
metanode testnet faucet limits

# Check faucet eligibility
metanode testnet faucet check <address>
metanode testnet faucet admin-refill --amount 1000000
```

### **Testnet Analytics**
```bash
# Testnet statistics
metanode testnet stats
metanode testnet activity
metanode testnet users
metanode testnet volume
```

---

## 📱 **Dashboard & UI Commands**

### **Dashboard Operations**
```bash
# Open dashboards
metanode dashboard open
metanode dashboard open --type governance
metanode dashboard open --type mining
metanode dashboard open --type network
metanode dashboard open --type economics

# Dashboard management
metanode dashboard list
metanode dashboard status
metanode dashboard config
metanode dashboard customize --layout grid
```

### **UI Management**
```bash
# Launch UI applications
metanode ui launch --type admin
metanode ui launch --type mobile
metanode ui launch --type analytics

# UI configuration
metanode ui config
metanode ui themes
metanode ui customize --theme dark
```

---

## 🚀 **App Deployment Commands**

### **Application Deployment**
```bash
# Deploy applications (one-command deployment)
metanode deploy app
metanode deploy app --name my-app --image nginx
metanode deploy app --file app.yaml
metanode deploy app --template webapp

# App management
metanode app list
metanode app status <app-name>
metanode app logs <app-name>
metanode app scale <app-name> --replicas 3

# App operations
metanode app start <app-name>
metanode app stop <app-name>
metanode app restart <app-name>
metanode app delete <app-name>
```

### **App Templates & Configuration**
```bash
# App templates
metanode app templates
metanode app template create <name>
metanode app template apply <template>

# App configuration
metanode app config <app-name>
metanode app config <app-name> --set key=value
metanode app env <app-name>
```

---

## ☸️ **Cluster Management Commands**

### **Cluster Operations**
```bash
# Create and manage clusters
metanode cluster create
metanode cluster create --name production --nodes 5
metanode cluster create --provider aws --region us-east-1

# Cluster management
metanode cluster list
metanode cluster status <cluster-name>
metanode cluster info <cluster-name>
metanode cluster delete <cluster-name>

# Cluster scaling
metanode cluster scale <cluster-name> --nodes 10
metanode cluster autoscale <cluster-name> --min 3 --max 20
```

### **OCI & Kubernetes Integration**
```bash
# OCI cluster operations
metanode cluster oci create --provider gcp
metanode cluster oci list
metanode cluster oci connect <cluster-name>

# Kubernetes integration
metanode cluster k8s deploy
metanode cluster k8s status
metanode cluster k8s config
metanode cluster k8s operator install

# Docker Swarm support
metanode cluster swarm init
metanode cluster swarm join <token>
metanode cluster swarm leave
```

### **Node Management**
```bash
# Node operations
metanode node add <cluster-name>
metanode node remove <cluster-name> <node-id>
metanode node list <cluster-name>
metanode node drain <node-id>
metanode node cordon <node-id>
```

---

## 🌐 **Decentralization & Mesh Commands**

### **Mesh Creation & Management**
```bash
# Create decentralized mesh
metanode mesh create --name global-mesh
metanode mesh create --regions us,eu,asia --redundancy 3

# Mesh operations
metanode mesh list
metanode mesh status <mesh-name>
metanode mesh topology <mesh-name>
metanode mesh optimize <mesh-name>

# Node distribution
metanode mesh distribute --strategy geographic
metanode mesh distribute --strategy load-balanced
metanode mesh rebalance <mesh-name>
```

### **Decentralization Control**
```bash
# Decentralization metrics
metanode decentralize status
metanode decentralize metrics
metanode decentralize score

# Geographic distribution
metanode decentralize regions
metanode decentralize add-region <region>
metanode decentralize balance-regions
```

---

## 🏢 **Enterprise & In-House Setup Commands**

### **Banking Integration**
```bash
# Banking setup
metanode enterprise banking-setup
metanode enterprise banking-setup --type central-bank
metanode enterprise banking-setup --compliance-level high

# Compliance integration
metanode enterprise compliance-setup
metanode enterprise audit-setup
metanode enterprise regulatory-reporting
```

### **High Availability & Disaster Recovery**
```bash
# HA/DR setup
metanode enterprise ha-setup
metanode enterprise ha-setup --replicas 5 --regions 3
metanode enterprise dr-setup --backup-regions us,eu

# Backup and recovery
metanode enterprise backup create
metanode enterprise backup restore <backup-id>
metanode enterprise backup schedule --daily
```

### **Enterprise Security**
```bash
# Security setup
metanode enterprise security-setup
metanode enterprise security-setup --level enterprise
metanode enterprise security-audit

# Access control
metanode enterprise rbac setup
metanode enterprise rbac add-role <role>
metanode enterprise rbac assign <user> <role>
```

---

## 🤖 **Advanced Automation Commands**

### **CI/CD Integration**
```bash
# CI/CD setup
metanode cicd setup
metanode cicd setup --provider github-actions
metanode cicd setup --provider jenkins

# Pipeline management
metanode cicd pipeline create <name>
metanode cicd pipeline run <name>
metanode cicd pipeline status <name>
```

### **GitOps & Workflow Automation**
```bash
# GitOps setup
metanode gitops setup --repo <repo-url>
metanode gitops sync
metanode gitops status

# Workflow automation
metanode workflow create <name>
metanode workflow run <name>
metanode workflow schedule <name> --cron "0 0 * * *"
```

---

## 🔮 **Future-Ready Extensions**

### **AI & ML Integration**
```bash
# AI/ML model deployment
metanode ai deploy-model <model-file>
metanode ai inference <model-id> --input <data>
metanode ai training start <dataset>

# AI operations
metanode ai models
metanode ai model-status <model-id>
metanode ai scale-inference <model-id> --replicas 5
```

### **IoT & Edge Computing**
```bash
# IoT device management
metanode iot devices
metanode iot device-register <device-id>
metanode iot device-status <device-id>

# Edge deployment
metanode edge deploy <app> --location <edge-location>
metanode edge locations
metanode edge status
```

### **Quantum-Ready Features**
```bash
# Quantum readiness
metanode quantum check-readiness
metanode quantum upgrade-crypto
metanode quantum test-resistance
```

---

## 📊 **Monitoring and Analytics Commands**

### **System Monitoring**
```bash
# System status
metanode status
metanode health
metanode metrics

# Performance monitoring
metanode monitor start
metanode monitor stop
metanode monitor dashboard

# Resource usage
metanode resources
metanode resources --detailed
metanode resources --history
```

### **Network Analytics**
```bash
# Transaction analytics
metanode analytics transactions
metanode analytics transactions --period day
metanode analytics volume

# Network performance
metanode analytics performance
metanode analytics latency
metanode analytics throughput

# Economic analytics
metanode analytics economics
metanode analytics mining
metanode analytics governance
```

### **Logs and Debugging**
```bash
# View logs
metanode logs
metanode logs --service consensus
metanode logs --level error
metanode logs --follow
metanode logs --tail 1000

# Debug information
metanode debug info
metanode debug dump
metanode debug trace <transaction>

# Export logs
metanode logs export --format json
metanode logs export --period week
```

---

## 🔧 **Developer Commands**

### **Development Tools**
```bash
# Generate keys
metanode dev keygen
metanode dev keygen --type ed25519
metanode dev keygen --output keys.json

# Encode/decode data
metanode dev encode <data>
metanode dev decode <encoded>
metanode dev hash <data>

# Test transactions
metanode dev send-test-tx
metanode dev simulate-tx <data>
```

### **Smart Contract Tools**
```bash
# Deploy contract
metanode contract deploy <bytecode>
metanode contract deploy <file> --args <args>

# Call contract
metanode contract call <address> <method> <args>
metanode contract view <address> <method> <args>

# Contract information
metanode contract info <address>
metanode contract events <address>
```

### **Testing and Simulation**
```bash
# Load testing
metanode test load --tps 100
metanode test load --duration 60s
metanode test stress

# Network simulation
metanode test network-partition
metanode test byzantine-fault
metanode test recovery
```

---

## 🔐 **Security Commands**

### **Key Management**
```bash
# Key operations
metanode key generate
metanode key import <file>
metanode key export <address>
metanode key list

# Signing
metanode sign <message>
metanode sign <message> --key <address>
metanode verify <signature> <message>

# Encryption
metanode encrypt <data> --recipient <pubkey>
metanode decrypt <encrypted> --key <privkey>
```

### **Security Auditing**
```bash
# Security scan
metanode security scan
metanode security audit
metanode security check-config

# Vulnerability assessment
metanode security vulnerabilities
metanode security recommendations
metanode security report
```

---

## 📦 **Package and Plugin Commands**

### **Package Management**
```bash
# List packages
metanode package list
metanode package search <query>
metanode package info <name>

# Install packages
metanode package install <name>
metanode package install <name>@<version>
metanode package update <name>
metanode package remove <name>

# Package development
metanode package create <name>
metanode package publish
metanode package validate
```

### **Plugin System**
```bash
# Plugin management
metanode plugin list
metanode plugin install <name>
metanode plugin enable <name>
metanode plugin disable <name>
metanode plugin remove <name>

# Plugin development
metanode plugin create <name>
metanode plugin build
metanode plugin test
```

---

## 🌐 **API and Integration Commands**

### **API Server**
```bash
# Start API server
metanode api start
metanode api start --port 8080
metanode api start --cors-origin "*"

# API management
metanode api stop
metanode api status
metanode api docs

# API testing
metanode api test
metanode api benchmark
```

### **WebSocket Server**
```bash
# WebSocket server
metanode ws start
metanode ws start --port 8081
metanode ws status
metanode ws clients
```

### **Integration Tools**
```bash
# Export data
metanode export transactions --format csv
metanode export blocks --range 1000-2000
metanode export accounts --format json

# Import data
metanode import <file>
metanode import <file> --format csv
metanode import <file> --validate
```

---

## 🔄 **Backup and Recovery Commands**

### **Backup Operations**
```bash
# Create backup
metanode backup create
metanode backup create --name daily-backup
metanode backup create --include-keys

# List backups
metanode backup list
metanode backup info <name>

# Restore backup
metanode backup restore <name>
metanode backup restore <name> --confirm
```

### **Data Management**
```bash
# Database operations
metanode db status
metanode db compact
metanode db repair
metanode db migrate

# State management
metanode state export
metanode state import <file>
metanode state verify
```

---

## 🎛️ **Advanced Configuration Commands**

### **Performance Tuning**
```bash
# Performance configuration
metanode perf tune
metanode perf benchmark
metanode perf profile

# Resource limits
metanode limits set memory 8GB
metanode limits set cpu 4
metanode limits show
```

### **Network Configuration**
```bash
# Network settings
metanode network config show
metanode network config set max-peers 50
metanode network config set listen-port 30303

# Firewall configuration
metanode firewall status
metanode firewall configure
metanode firewall test
```

---

## 📋 **Batch and Automation Commands**

### **Batch Operations**
```bash
# Batch transactions
metanode batch send <file>
metanode batch execute <script>

# Scheduled operations
metanode schedule backup daily
metanode schedule sync hourly
metanode schedule list
metanode schedule remove <id>
```

### **Automation Scripts**
```bash
# Script management
metanode script run <file>
metanode script validate <file>
metanode script list

# Automation
metanode auto-update enable
metanode auto-update disable
metanode auto-update status
```

---

## 🆘 **Support and Maintenance Commands**

### **Diagnostics**
```bash
# System diagnostics
metanode doctor
metanode diagnose
metanode check-health

# Troubleshooting
metanode troubleshoot network
metanode troubleshoot consensus
metanode troubleshoot performance
```

### **Maintenance**
```bash
# Update system
metanode update
metanode update --check
metanode update --force

# Cleanup
metanode cleanup
metanode cleanup --cache
metanode cleanup --logs
metanode cleanup --force

# Reset
metanode reset
metanode reset --confirm
metanode reset --keep-config
```

---

## 🔗 **Command Chaining and Pipes**

### **Advanced Usage**
```bash
# Command chaining
metanode balance | metanode send 0x... 10
metanode node status && metanode mine start

# JSON processing with jq
metanode balance --json | jq '.GEN'
metanode proposals --json | jq '.[] | select(.status == "active")'

# Batch processing
metanode wallet list --json | jq -r '.[].address' | xargs -I {} metanode balance {}

# Monitoring loops
watch -n 5 'metanode status'
while true; do metanode mine stats; sleep 60; done
```

---

This comprehensive reference covers **every command** in the Metanode CLI, ensuring users can interact with all aspects of the system, from basic operations to advanced mesh interactions.
