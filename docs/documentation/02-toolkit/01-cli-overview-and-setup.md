# BPCI Enterprise CLI Toolkit - Complete Developer Guide

## Overview
The BPCI Enterprise CLI toolkit provides comprehensive command-line tools for managing wallets, mining operations, governance, network operations, and advanced blockchain features. This guide covers installation, setup, and usage of all CLI components.

## Installation and Setup

### Prerequisites
```bash
# System requirements
- Ubuntu 22.04 LTS or compatible Linux distribution
- Rust 1.70+ with Cargo
- 8GB+ RAM, 4+ CPU cores
- 100GB+ available storage
- Network connectivity for blockchain operations

# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Building the BPCI CLI
```bash
# Clone and build the BPCI Enterprise toolkit
git clone https://github.com/pravyom/bpci-enterprise.git
cd bpci-enterprise

# Build in release mode for optimal performance
cargo build --release

# The CLI binary will be available at:
# ./target/release/bpci-enterprise

# Optional: Install globally
sudo cp target/release/bpci-enterprise /usr/local/bin/pravyom
sudo chmod +x /usr/local/bin/pravyom
```

### Initial Configuration
```bash
# Initialize BPCI system (first-time setup)
pravyom init

# Initialize with force (overwrite existing configuration)
pravyom init --force

# Check system status
pravyom status

# Enable verbose logging for debugging
pravyom --verbose status

# Use JSON output for programmatic access
pravyom --format json status
```

## CLI Command Structure

### Global Options
```bash
# Available for all commands
pravyom [GLOBAL_OPTIONS] <COMMAND> [COMMAND_OPTIONS]

Global Options:
  -v, --verbose          Enable verbose logging
  -c, --config <FILE>    Configuration file path
  -n, --network <NET>    Network selection (testnet, mainnet, localnet)
      --format <FORMAT>  Output format (json, human) [default: human]
      --dry-run          Show what would happen without executing
  -h, --help             Print help information
  -V, --version          Print version information
```

### Network Selection
```bash
# Testnet (default) - Safe for development and testing
pravyom --network testnet wallet create --name test-wallet

# Mainnet - Production blockchain network
pravyom --network mainnet wallet balance wallet-id-here

# Localnet - Local development network
pravyom --network localnet mining start --threads 2
```

## Core CLI Modules

### 1. Wallet Management (`pravyom wallet`)
Complete wallet lifecycle management with cryptographic security.

```bash
# Create a new wallet
pravyom wallet create --name "my-wallet" --wallet-type docklock --key-type ed25519

# List all wallets
pravyom wallet list
pravyom wallet list --wallet-type docklock --detailed

# Check wallet status and information
pravyom wallet status wallet-id-or-name

# Check wallet balance
pravyom wallet balance wallet-id-or-name
pravyom wallet balance wallet-id-or-name --token custom-token

# Backup wallet (encrypted)
pravyom wallet backup wallet-id --output ./backup.json --encrypt

# Restore wallet from backup
pravyom wallet restore --input ./backup.json --name "restored-wallet"

# Verify wallet integrity
pravyom wallet verify wallet-id --signatures

# Send transactions
pravyom wallet send --from sender-wallet --to recipient-wallet --amount 100.0

# Sign arbitrary data
pravyom wallet sign --wallet-id wallet-id --data "message to sign"

# Verify signatures
pravyom wallet verify-signature --wallet-id wallet-id --data "message" --signature "signature-hex"

# Export wallet (various formats)
pravyom wallet export wallet-id --format json
pravyom wallet export wallet-id --format pem

# Import wallet from private key
pravyom wallet import --private-key "hex-key" --name "imported-wallet" --key-type ed25519
```

### 2. Registry Operations (`pravyom registry`)
Manage BPI wallet registry and node registration.

```bash
# Register a new node
pravyom registry register-node --node-type validator --stake-amount 1000

# List registered nodes
pravyom registry list-nodes --node-type all --detailed

# Update node information
pravyom registry update-node node-id --endpoint "https://new-endpoint.com"

# Deregister node
pravyom registry deregister-node node-id

# Check node status
pravyom registry node-status node-id

# Validate node registration
pravyom registry validate-node node-id

# Show registry statistics
pravyom registry stats --detailed
```

### 3. Mining Operations (`pravyom mining`)
Proof-of-Execution mining with advanced features.

```bash
# Start mining
pravyom mining start --threads 4 --pool optional-pool-id

# Start mining with custom configuration
pravyom mining start --threads 8 --difficulty 1000000 --reward-address wallet-id

# Stop mining
pravyom mining stop
pravyom mining stop --force  # Force stop if needed

# Check mining status
pravyom mining status
pravyom mining status --detailed --refresh 5  # Auto-refresh every 5 seconds

# List available mining pools
pravyom mining list-pools --active-only --detailed

# Join a mining pool
pravyom mining join-pool pool-id --worker-name "my-miner" --power 1000

# Leave mining pool
pravyom mining leave-pool pool-id

# Show mining rewards
pravyom mining rewards --period "last-week" --detailed

# Claim mining rewards
pravyom mining claim-rewards --wallet wallet-id --min-amount 10.0

# Show current difficulty
pravyom mining difficulty --history --blocks 100

# Validate proof-of-execution
pravyom mining validate-proof --proof-data "proof-hex" --block-hash "block-hash"

# Generate proof-of-execution
pravyom mining generate-proof --execution-data "data" --validator-id validator-id

# Show validator statistics
pravyom mining validator-stats --validator-id validator-id --detailed

# Configure mining parameters
pravyom mining configure --max-cpu 80 --max-memory 4096 --priority high --auto-difficulty

# View mining logs
pravyom mining logs --lines 100 --follow --level info

# Benchmark mining performance
pravyom mining benchmark --duration 60 --threads 4
```

### 4. Governance Operations (`pravyom governance`)
Participate in blockchain governance and voting.

```bash
# List active proposals
pravyom governance list-proposals --status active --detailed

# Create a new proposal
pravyom governance create-proposal --title "Upgrade Protocol" --description "..." --voting-period 7

# Vote on a proposal
pravyom governance vote proposal-id --choice yes --wallet-id voter-wallet

# Show proposal details
pravyom governance proposal-details proposal-id

# Check voting power
pravyom governance voting-power wallet-id

# Show governance statistics
pravyom governance stats --period "last-month"

# Delegate voting power
pravyom governance delegate --to delegate-wallet --amount 1000 --wallet-id delegator-wallet

# Undelegate voting power
pravyom governance undelegate --from delegate-wallet --amount 500 --wallet-id delegator-wallet
```

### 5. Network Operations (`pravyom network`)
Network management and peer operations.

```bash
# Show network status
pravyom network status --detailed

# List connected peers
pravyom network peers --detailed --active-only

# Connect to a peer
pravyom network connect --peer-id peer-id --address "ip:port"

# Disconnect from a peer
pravyom network disconnect peer-id

# Show network statistics
pravyom network stats --period "last-hour"

# Test network connectivity
pravyom network test --target "node-address" --timeout 30

# Show blockchain synchronization status
pravyom network sync-status

# Force network resync
pravyom network resync --from-block 1000

# Show network configuration
pravyom network config
```

### 6. Notary Services (`pravyom notary`)
Document notarization and verification services.

```bash
# Notarize a document
pravyom notary notarize --file document.pdf --wallet-id notary-wallet

# Verify a notarized document
pravyom notary verify --file document.pdf --signature signature-hex

# List notarization history
pravyom notary history --wallet-id notary-wallet --limit 50

# Show notary statistics
pravyom notary stats --notary-id notary-wallet --period "last-month"

# Register as a notary
pravyom notary register --wallet-id wallet-id --bond-amount 10000

# Revoke notarization
pravyom notary revoke --document-hash hash --reason "error in document"
```

### 7. System Maintenance (`pravyom maintenance`)
System maintenance and administrative operations.

```bash
# Show system health
pravyom maintenance health --detailed

# Clean up temporary files
pravyom maintenance cleanup --temp-files --logs-older-than 30

# Backup system data
pravyom maintenance backup --output ./system-backup.tar.gz --include-wallets

# Restore system data
pravyom maintenance restore --input ./system-backup.tar.gz

# Update system components
pravyom maintenance update --component all --check-only

# Show system logs
pravyom maintenance logs --component mining --level error --lines 100

# Restart system services
pravyom maintenance restart --service mining --service network

# Show system metrics
pravyom maintenance metrics --live --refresh 5

# Database maintenance
pravyom maintenance database --vacuum --reindex --analyze
```

## Advanced CLI Modules

### 8. CueDB Operations (`pravyom cuedb`)
Advanced database operations with cue-based rules.

```bash
# Create a CueDB agreement
pravyom cuedb create-agreement --wallet-id wallet-id --type developer --storage-gb 10

# Deploy agreement to network
pravyom cuedb deploy-agreement --file agreement.json

# List active agreements
pravyom cuedb list-agreements --wallet-id wallet-id

# Execute database operations
pravyom cuedb execute-operation --agreement-id agreement-id --operation read --table users

# Parse DBYML configuration
pravyom cuedb parse-dbyml --file database-config.dbyml

# Test CueDB system
pravyom cuedb test-system --agreement-id agreement-id

# Show CueDB metrics
pravyom cuedb metrics --agreement-id agreement-id --live

# Query CueDB data
pravyom cuedb query --agreement-id agreement-id --sql "SELECT * FROM users LIMIT 10"

# Show CueDB statistics
pravyom cuedb stats --agreement-id agreement-id --detailed
```

### 9. Cross-System Integration (`pravyom cross-system`)
Integration with Court, Shadow Registry, and BPI Mesh.

```bash
# Test Court-Shadow bridge
pravyom cross-system test-court-shadow --operation shadow-sync --wallet-id wallet-id

# Test Court-BPI Mesh integration
pravyom cross-system test-court-bpi-mesh --operation banking --amount 1000

# Test unified audit system
pravyom cross-system test-unified-audit --audit-type transaction --data "test-data"

# Show cross-system status
pravyom cross-system status --detailed

# Sync with external systems
pravyom cross-system sync --target shadow-registry --force

# Show integration metrics
pravyom cross-system metrics --system all --period "last-day"
```

### 10. Orchestration System (`pravyom orchestration`)
Revolutionary orchestration with MetanodeClusterManager.

```bash
# Deploy orchestration cluster
pravyom orchestration deploy-cluster --config cluster-config.yaml

# Show cluster status
pravyom orchestration cluster-status --cluster-id cluster-id

# Scale cluster resources
pravyom orchestration scale --cluster-id cluster-id --nodes 5 --cpu 16 --memory 32

# Update cluster configuration
pravyom orchestration update-cluster --cluster-id cluster-id --config new-config.yaml

# Show orchestration metrics
pravyom orchestration metrics --cluster-id cluster-id --live

# Manage daemon tree
pravyom orchestration daemon-tree --operation start --service-name bpi-core

# Show orchestration logs
pravyom orchestration logs --cluster-id cluster-id --service bpi-core --follow
```

## Configuration and Environment

### Configuration File Structure
```toml
# ~/.config/bpci/config.toml
[network]
default = "testnet"
testnet_endpoint = "https://testnet.bpci.network"
mainnet_endpoint = "https://mainnet.bpci.network"
localnet_endpoint = "http://localhost:8080"

[wallet]
default_key_type = "ed25519"
backup_encryption = true
auto_backup = true
backup_interval_hours = 24

[mining]
default_threads = 4
auto_difficulty = true
pool_preference = "community"
reward_threshold = 10.0

[logging]
level = "info"
file = "/var/log/bpci/cli.log"
max_size_mb = 100
rotate_count = 5

[security]
require_confirmation = true
session_timeout_minutes = 30
auto_lock = true
```

### Environment Variables
```bash
# Set environment variables for CLI operation
export BPCI_CONFIG="/path/to/config.toml"
export BPCI_NETWORK="testnet"
export BPCI_LOG_LEVEL="debug"
export BPCI_WALLET_DIR="/secure/path/to/wallets"
export BPCI_DATA_DIR="/path/to/data"

# For automated scripts
export BPCI_AUTO_CONFIRM="true"
export BPCI_JSON_OUTPUT="true"
```

## Scripting and Automation

### Bash Script Example
```bash
#!/bin/bash
# automated-mining-setup.sh

set -e

# Configuration
WALLET_NAME="mining-wallet-$(date +%s)"
MINING_THREADS=4
POOL_ID="community-pool-1"

echo "🚀 Setting up automated mining..."

# Create wallet
echo "Creating wallet: $WALLET_NAME"
WALLET_ID=$(pravyom wallet create --name "$WALLET_NAME" --format json | jq -r '.wallet_id')

if [ -z "$WALLET_ID" ]; then
    echo "❌ Failed to create wallet"
    exit 1
fi

echo "✅ Wallet created: $WALLET_ID"

# Register with mining pool
echo "Joining mining pool: $POOL_ID"
pravyom mining join-pool "$POOL_ID" --worker-name "auto-miner-$WALLET_ID" --power 1000

# Start mining
echo "Starting mining with $MINING_THREADS threads"
pravyom mining start --threads "$MINING_THREADS" --pool "$POOL_ID"

# Monitor mining status
echo "Mining started successfully. Monitoring status..."
while true; do
    STATUS=$(pravyom mining status --format json)
    IS_MINING=$(echo "$STATUS" | jq -r '.is_mining')
    HASHRATE=$(echo "$STATUS" | jq -r '.hashrate')
    
    if [ "$IS_MINING" = "true" ]; then
        echo "⛏️  Mining active - Hashrate: $HASHRATE H/s"
    else
        echo "⚠️  Mining stopped - Restarting..."
        pravyom mining start --threads "$MINING_THREADS" --pool "$POOL_ID"
    fi
    
    sleep 60
done
```

### Python Integration Example
```python
#!/usr/bin/env python3
# bpci_automation.py

import subprocess
import json
import time
from typing import Dict, Any

class BPCIClient:
    def __init__(self, network: str = "testnet", verbose: bool = False):
        self.network = network
        self.verbose = verbose
        self.base_cmd = ["pravyom", "--network", network, "--format", "json"]
        if verbose:
            self.base_cmd.append("--verbose")
    
    def run_command(self, cmd: list) -> Dict[Any, Any]:
        """Execute BPCI CLI command and return JSON result"""
        full_cmd = self.base_cmd + cmd
        try:
            result = subprocess.run(full_cmd, capture_output=True, text=True, check=True)
            return json.loads(result.stdout)
        except subprocess.CalledProcessError as e:
            print(f"Command failed: {' '.join(full_cmd)}")
            print(f"Error: {e.stderr}")
            raise
    
    def create_wallet(self, name: str, wallet_type: str = "docklock") -> str:
        """Create a new wallet and return wallet ID"""
        result = self.run_command([
            "wallet", "create", 
            "--name", name, 
            "--wallet-type", wallet_type
        ])
        return result["wallet_id"]
    
    def get_wallet_balance(self, wallet_id: str) -> float:
        """Get wallet balance"""
        result = self.run_command(["wallet", "balance", wallet_id])
        return float(result["balance"])
    
    def start_mining(self, threads: int = 4, pool_id: str = None) -> bool:
        """Start mining operation"""
        cmd = ["mining", "start", "--threads", str(threads)]
        if pool_id:
            cmd.extend(["--pool", pool_id])
        
        result = self.run_command(cmd)
        return result.get("success", False)
    
    def get_mining_status(self) -> Dict[str, Any]:
        """Get current mining status"""
        return self.run_command(["mining", "status"])
    
    def monitor_mining(self, interval: int = 60):
        """Monitor mining status continuously"""
        print("🔍 Starting mining monitor...")
        while True:
            try:
                status = self.get_mining_status()
                if status["is_mining"]:
                    print(f"⛏️  Mining: {status['hashrate']:.2f} H/s | "
                          f"Blocks: {status['blocks_mined']} | "
                          f"Rewards: {status['rewards_earned']:.4f}")
                else:
                    print("⚠️  Mining stopped")
                
                time.sleep(interval)
            except KeyboardInterrupt:
                print("\n👋 Monitoring stopped")
                break
            except Exception as e:
                print(f"❌ Error: {e}")
                time.sleep(interval)

# Example usage
if __name__ == "__main__":
    # Initialize client
    client = BPCIClient(network="testnet", verbose=True)
    
    # Create wallet
    wallet_id = client.create_wallet("auto-mining-wallet")
    print(f"✅ Created wallet: {wallet_id}")
    
    # Start mining
    success = client.start_mining(threads=2)
    if success:
        print("✅ Mining started successfully")
        client.monitor_mining(interval=30)
    else:
        print("❌ Failed to start mining")
```

## Troubleshooting Common Issues

### Issue 1: CLI Not Found
```bash
# Check if binary exists
ls -la ./target/release/bpci-enterprise

# Add to PATH temporarily
export PATH=$PATH:$(pwd)/target/release

# Or install globally
sudo cp target/release/bpci-enterprise /usr/local/bin/pravyom
```

### Issue 2: Permission Denied
```bash
# Fix binary permissions
chmod +x ./target/release/bpci-enterprise

# Fix data directory permissions
sudo chown -R $USER:$USER ~/.config/bpci
sudo chown -R $USER:$USER ~/.local/share/bpci
```

### Issue 3: Network Connection Issues
```bash
# Test network connectivity
pravyom network test --target "testnet.bpci.network"

# Check firewall settings
sudo ufw status
sudo ufw allow 8080/tcp  # BPCI default port

# Verify DNS resolution
nslookup testnet.bpci.network
```

### Issue 4: Wallet Issues
```bash
# Verify wallet integrity
pravyom wallet verify wallet-id --signatures

# Backup wallet before troubleshooting
pravyom wallet backup wallet-id --output emergency-backup.json --encrypt

# Check wallet permissions
ls -la ~/.local/share/bpci/wallets/
```

### Issue 5: Mining Problems
```bash
# Check mining logs
pravyom mining logs --lines 50 --level error

# Verify system resources
pravyom maintenance health --detailed

# Test mining benchmark
pravyom mining benchmark --duration 30 --threads 2
```

## Best Practices

### Security Best Practices
1. **Always backup wallets** before major operations
2. **Use encrypted backups** for wallet storage
3. **Verify signatures** for important transactions
4. **Use dry-run mode** for testing commands
5. **Keep CLI updated** to latest version

### Performance Optimization
1. **Use appropriate thread counts** for mining (typically CPU cores - 1)
2. **Monitor system resources** during operations
3. **Use JSON output** for automated scripts
4. **Enable verbose logging** only when debugging
5. **Regular maintenance** with cleanup commands

### Operational Guidelines
1. **Test on testnet** before mainnet operations
2. **Use configuration files** for consistent settings
3. **Implement monitoring** for long-running operations
4. **Document custom scripts** and configurations
5. **Regular backups** of system data and wallets

---

**Next**: [Wallet Management Guide](02-wallet-management-guide.md)  
**Related**: [Mining Operations](03-mining-operations-guide.md), [Network Management](04-network-management-guide.md)
