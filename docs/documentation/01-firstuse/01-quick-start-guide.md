# BPCI First Use - Quick Start Guide

## Welcome to BPCI Enterprise!

This guide will walk you through your first BPCI deployment, from system installation to running your first decentralized application. By the end of this guide, you'll have a fully functional BPCI node participating in the network.

## Prerequisites

### System Requirements
```
Minimum Requirements:
├── CPU: 8 cores (Intel/AMD x64)
├── RAM: 8 GB DDR4
├── Storage: 100 GB SSD
├── Network: 100 Mbps stable connection
├── OS: Ubuntu 22.04 LTS (recommended)
└── Kernel: 5.15.0 or higher
```

### Network Requirements
- **Open Ports**: 8080 (HTTP), 8443 (HTTPS), 30303 (P2P)
- **Firewall**: UFW or iptables configured
- **DNS**: Reliable DNS resolution
- **Bandwidth**: Minimum 100 Mbps up/down

## Step 1: System Preparation

### 1.1 Update Your System
```bash
# Update package lists and system
sudo apt update && sudo apt upgrade -y

# Install essential tools
sudo apt install -y curl wget git build-essential pkg-config libssl-dev

# Install Rust (required for BPCI)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 1.2 Verify System Compatibility
```bash
# Check CPU cores
nproc

# Check RAM
free -h

# Check storage
df -h

# Check kernel version
uname -r

# Check network connectivity
ping -c 4 8.8.8.8
```

## Step 2: BPCI Installation

### 2.1 Download BPCI Community Installer
```bash
# Create BPCI directory
mkdir -p ~/bpci && cd ~/bpci

# Clone BPCI repository (replace with actual repository)
git clone https://github.com/bpci-enterprise/bpci-community.git
cd bpci-community

# Build BPCI (this may take 10-15 minutes)
cargo build --release
```

### 2.2 Initialize Community Installer OS
```rust
// Example: Initialize the installer programmatically
use bpci_enterprise::community_installer_os::{CommunityInstallerOS, InstallerConfig};

#[tokio::main]
async fn main() -> Result<()> {
    // Create installer with default configuration
    let mut installer = CommunityInstallerOS::new(None);
    
    // Run the complete installation
    installer.install().await?;
    
    println!("✅ BPCI installation completed successfully!");
    Ok(())
}
```

### 2.3 Command Line Installation
```bash
# Run the automated installer
./target/release/bpci-installer --mode=community

# Follow the interactive prompts:
# 1. Accept system requirements check
# 2. Configure security settings
# 3. Set up mining parameters
# 4. Configure monitoring
# 5. Start services
```

## Step 3: First Application Deployment

### 3.1 Create Your First CueDB Agreement
```rust
use bpci_enterprise::cuedb_agreement::{CueDbAgreementBuilder, CueDbAgreementType};

// Create a basic CueDB agreement for your first app
let agreement = CueDbAgreementBuilder::new()
    .wallet_id("your-wallet-id-here")
    .agreement_type(CueDbAgreementType::Developer {
        max_storage_gb: 10,
        max_transactions_per_day: 1000,
        allowed_operations: vec!["read".to_string(), "write".to_string()],
        data_retention_days: 30,
    })
    .add_data_volume_rule(
        5, // 5 GB threshold
        DatabaseAction::Alert,
        EnforcementLevel::Warning
    )
    .expires_at(Utc::now() + chrono::Duration::days(30))
    .build()?;

println!("✅ CueDB Agreement created: {}", agreement.agreement_id);
```

### 3.2 Deploy Your First App
```bash
# Create a simple "Hello BPCI" application
mkdir ~/my-first-bpci-app && cd ~/my-first-bpci-app

# Create app configuration
cat > app.toml << EOF
[app]
name = "hello-bpci"
version = "1.0.0"
description = "My first BPCI application"

[database]
type = "cuedb"
max_storage_gb = 1
max_transactions_per_day = 100

[network]
port = 8080
enable_https = true
EOF

# Deploy the application
bpci deploy --config app.toml --wallet-id your-wallet-id
```

## Step 4: Verification and Testing

### 4.1 Check Installation Status
```bash
# Check BPCI services status
systemctl status bpci-node
systemctl status bpci-auction-mempool
systemctl status bpci-consensus

# Check network connectivity
bpci network status

# Check wallet status
bpci wallet status
```

### 4.2 Run Health Checks
```rust
use bpci_enterprise::community_installer_os::CommunityInstallerOS;

// Check system health
let installer = CommunityInstallerOS::new(None);

// Test network connectivity
installer.test_network_connectivity().await?;

// Test system resources
installer.test_system_resources().await?;

println!("✅ All health checks passed!");
```

### 4.3 Monitor Your Node
```bash
# View real-time logs
journalctl -f -u bpci-node

# Check auction participation
bpci auction status

# Monitor consensus participation
bpci consensus metrics

# View economic metrics
bpci economy status
```

## Step 5: Join the Network

### 5.1 Register as Community Miner
```bash
# Register your node with the BPCI network
bpci register --type=community-miner --stake=1000

# Start participating in auctions
bpci auction join --auto-bid=true --max-bid=100

# Enable consensus participation
bpci consensus enable --validator=true
```

### 5.2 Connect to Partner Chains
```bash
# List available partner chains
bpci partners list

# Connect to Ethereum testnet
bpci partners connect ethereum-goerli

# Connect to Polygon testnet
bpci partners connect polygon-mumbai

# Verify connections
bpci partners status
```

## Common First-Use Issues

### Issue 1: System Requirements Not Met
**Error**: `System requirements check failed`
**Solution**:
```bash
# Check specific requirement that failed
bpci system-check --verbose

# Upgrade system if needed
sudo apt upgrade -y

# Add more storage if needed
sudo resize2fs /dev/sda1
```

### Issue 2: Network Connectivity Problems
**Error**: `Failed to connect to BPCI network`
**Solution**:
```bash
# Check firewall settings
sudo ufw status

# Open required ports
sudo ufw allow 8080
sudo ufw allow 8443
sudo ufw allow 30303

# Test connectivity
telnet bpci-network.com 30303
```

### Issue 3: Wallet Creation Issues
**Error**: `Failed to create wallet`
**Solution**:
```bash
# Create wallet manually
bpci wallet create --name=my-wallet

# Import existing wallet
bpci wallet import --file=wallet.json

# Check wallet status
bpci wallet list
```

## Next Steps

### 1. Explore Advanced Features
- **Cross-Chain Operations**: Learn about multi-chain deployments
- **Advanced Auctions**: Participate in specialized auction types
- **Governance**: Vote on network proposals
- **Treasury**: Manage economic operations

### 2. Development Resources
- **API Documentation**: `/documentation/15-gov-api/`
- **SDK Examples**: `/documentation/02-toolkit/`
- **Testing Framework**: `/documentation/29-testing-framework/`

### 3. Community Engagement
- **Discord**: Join the BPCI community
- **GitHub**: Contribute to development
- **Forums**: Ask questions and share experiences

## Quick Reference Commands

```bash
# Essential BPCI commands
bpci status                    # Overall system status
bpci wallet balance           # Check wallet balance
bpci auction participate      # Join current auction
bpci consensus status         # Consensus participation
bpci partners list           # Available partner chains
bpci logs --tail=100         # Recent logs
bpci update                  # Update BPCI software
bpci backup                  # Backup configuration
bpci restore                 # Restore from backup
bpci help                    # Command help
```

## Success Indicators

You've successfully completed your first BPCI setup when you see:

✅ **System Status**: All services running  
✅ **Network**: Connected to BPCI network  
✅ **Wallet**: Created and funded  
✅ **Auctions**: Participating in auctions  
✅ **Consensus**: Contributing to consensus  
✅ **Partners**: Connected to partner chains  
✅ **App**: First application deployed  

**Congratulations! You're now part of the BPCI decentralized network!** 🎉

---

**Next**: [Detailed Installation Guide](02-detailed-installation-guide.md)  
**Related**: [System Requirements](03-system-requirements-and-setup.md), [CueDB Setup](04-cuedb-agreement-setup.md)
