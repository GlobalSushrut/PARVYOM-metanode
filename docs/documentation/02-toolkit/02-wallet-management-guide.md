# Wallet Management Guide - Complete BPCI Wallet Operations

## Overview
BPCI Enterprise provides comprehensive wallet management capabilities with military-grade security, multiple wallet types, and advanced cryptographic features. This guide covers all aspects of wallet creation, management, and operations.

## Wallet Types and Architecture

### Supported Wallet Types
```rust
// From the BPCI codebase - real wallet types
pub enum WalletType {
    DockLock,    // Default - Secure container-based wallet
    Metanode,    // Advanced multi-node wallet
    Dao,         // DAO governance wallet
    Bpi,         // Native BPI blockchain wallet
}

// Cryptographic key types supported
pub enum KeyType {
    Ed25519,     // Default - EdDSA signatures (recommended)
    Secp256k1,   // ECDSA signatures (Bitcoin/Ethereum compatible)
}
```

### Wallet Security Features
- **Military-grade encryption**: Ed25519 cryptographic signatures
- **Hierarchical deterministic (HD) wallets**: BIP32/BIP44 compatible
- **Multi-signature support**: M-of-N signature schemes
- **Hardware security module (HSM) integration**: For enterprise deployments
- **Encrypted backup and recovery**: AES-256 encrypted wallet backups
- **Audit trails**: Complete transaction and operation logging

## Wallet Creation and Setup

### Basic Wallet Creation
```bash
# Create a standard DockLock wallet (recommended for most users)
pravyom wallet create --name "my-primary-wallet" --wallet-type docklock --key-type ed25519

# Create a Metanode wallet for advanced operations
pravyom wallet create --name "enterprise-wallet" --wallet-type metanode --key-type ed25519

# Create a DAO governance wallet
pravyom wallet create --name "dao-governance" --wallet-type dao --key-type ed25519

# Create a native BPI blockchain wallet
pravyom wallet create --name "bpi-native" --wallet-type bpi --key-type ed25519
```

### Advanced Wallet Creation with Custom Parameters
```bash
# Create wallet with specific entropy source
pravyom wallet create \
    --name "high-security-wallet" \
    --wallet-type docklock \
    --key-type ed25519 \
    --entropy-source hardware \
    --backup-encryption true

# Create multi-signature wallet
pravyom wallet create \
    --name "multisig-wallet" \
    --wallet-type metanode \
    --multisig-threshold 2 \
    --multisig-total 3 \
    --key-type ed25519

# Create wallet with custom derivation path
pravyom wallet create \
    --name "custom-path-wallet" \
    --wallet-type bpi \
    --derivation-path "m/44'/60'/0'/0/0" \
    --key-type secp256k1
```

### Wallet Creation Response
```json
{
  "success": true,
  "wallet_id": "bpci_wallet_7f8a9b2c3d4e5f6a7b8c9d0e1f2a3b4c",
  "wallet_name": "my-primary-wallet",
  "wallet_type": "docklock",
  "key_type": "ed25519",
  "public_key": "ed25519:A1B2C3D4E5F6A7B8C9D0E1F2A3B4C5D6E7F8A9B0C1D2E3F4A5B6C7D8E9F0A1B2",
  "address": "bpci1qw2e3r4t5y6u7i8o9p0a1s2d3f4g5h6j7k8l9z0x1c2v3b4n5m6",
  "created_at": "2024-09-05T08:31:12Z",
  "backup_created": true,
  "backup_location": "/home/user/.config/bpci/backups/my-primary-wallet-20240905.json.enc"
}
```

## Wallet Management Operations

### Listing and Inspecting Wallets
```bash
# List all wallets
pravyom wallet list

# List wallets with detailed information
pravyom wallet list --detailed

# Filter wallets by type
pravyom wallet list --wallet-type docklock --detailed

# JSON output for programmatic access
pravyom wallet list --format json
```

### Wallet Status and Information
```bash
# Get comprehensive wallet status
pravyom wallet status my-primary-wallet

# Check wallet with verbose information
pravyom --verbose wallet status bpci_wallet_7f8a9b2c3d4e5f6a7b8c9d0e1f2a3b4c

# Get wallet status in JSON format
pravyom wallet status my-primary-wallet --format json
```

### Wallet Status Response Example
```json
{
  "wallet_id": "bpci_wallet_7f8a9b2c3d4e5f6a7b8c9d0e1f2a3b4c",
  "name": "my-primary-wallet",
  "type": "docklock",
  "status": "active",
  "balance": {
    "native": "1250.75",
    "tokens": {
      "GEN": "500.00",
      "NEX": "250.25",
      "FLX": "100.50",
      "AUR": "75.00"
    }
  },
  "transaction_count": 42,
  "last_activity": "2024-09-05T07:15:30Z",
  "security": {
    "encryption_enabled": true,
    "backup_status": "current",
    "multisig_enabled": false,
    "hardware_security": false
  },
  "network": {
    "connected": true,
    "sync_status": "synced",
    "peer_count": 8
  }
}
```

## Balance Management

### Checking Balances
```bash
# Check native token balance
pravyom wallet balance my-primary-wallet

# Check specific token balance
pravyom wallet balance my-primary-wallet --token GEN
pravyom wallet balance my-primary-wallet --token NEX

# Check all token balances with details
pravyom wallet balance my-primary-wallet --detailed --format json
```

### Balance Response Example
```json
{
  "wallet_id": "bpci_wallet_7f8a9b2c3d4e5f6a7b8c9d0e1f2a3b4c",
  "balances": {
    "native": {
      "amount": "1250.75",
      "currency": "BPCI",
      "usd_value": "2501.50",
      "last_updated": "2024-09-05T08:30:00Z"
    },
    "tokens": {
      "GEN": {
        "amount": "500.00",
        "contract": "bpci1qgen...",
        "usd_value": "1000.00",
        "staked": "100.00",
        "available": "400.00"
      },
      "NEX": {
        "amount": "250.25",
        "contract": "bpci1qnex...",
        "usd_value": "500.50",
        "locked": "50.25",
        "available": "200.00"
      }
    }
  },
  "total_usd_value": "4002.00"
}
```

## Transaction Operations

### Sending Transactions
```bash
# Send native BPCI tokens
pravyom wallet send \
    --from my-primary-wallet \
    --to bpci1qrecipient... \
    --amount 100.0

# Send specific tokens
pravyom wallet send \
    --from my-primary-wallet \
    --to bpci1qrecipient... \
    --amount 50.0 \
    --token GEN

# Send with custom gas settings
pravyom wallet send \
    --from my-primary-wallet \
    --to bpci1qrecipient... \
    --amount 25.0 \
    --gas-limit 21000 \
    --gas-price 20

# Dry run to estimate costs
pravyom --dry-run wallet send \
    --from my-primary-wallet \
    --to bpci1qrecipient... \
    --amount 100.0
```

### Transaction Response
```json
{
  "success": true,
  "transaction_id": "0x1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b",
  "from": "bpci1qsender...",
  "to": "bpci1qrecipient...",
  "amount": "100.0",
  "token": "BPCI",
  "gas_used": "21000",
  "gas_price": "20",
  "total_cost": "100.00042",
  "status": "pending",
  "block_height": null,
  "timestamp": "2024-09-05T08:31:12Z",
  "confirmation_time_estimate": "30-60 seconds"
}
```

## Cryptographic Operations

### Digital Signatures
```bash
# Sign arbitrary data
pravyom wallet sign \
    --wallet-id my-primary-wallet \
    --data "Important message to sign"

# Sign data from file
pravyom wallet sign \
    --wallet-id my-primary-wallet \
    --file ./document.txt

# Sign with specific key derivation
pravyom wallet sign \
    --wallet-id my-primary-wallet \
    --data "Message" \
    --derivation-path "m/44'/60'/0'/0/1"
```

### Signature Verification
```bash
# Verify signature
pravyom wallet verify-signature \
    --wallet-id my-primary-wallet \
    --data "Important message to sign" \
    --signature "ed25519:A1B2C3..."

# Verify signature from file
pravyom wallet verify-signature \
    --wallet-id my-primary-wallet \
    --file ./document.txt \
    --signature-file ./document.sig

# Verify with public key only
pravyom wallet verify-signature \
    --public-key "ed25519:A1B2C3..." \
    --data "Message" \
    --signature "ed25519:D4E5F6..."
```

## Backup and Recovery

### Creating Wallet Backups
```bash
# Create encrypted backup
pravyom wallet backup my-primary-wallet \
    --output ./my-wallet-backup.json \
    --encrypt

# Create unencrypted backup (not recommended for production)
pravyom wallet backup my-primary-wallet \
    --output ./my-wallet-backup.json

# Create backup with custom encryption password
pravyom wallet backup my-primary-wallet \
    --output ./my-wallet-backup.json \
    --encrypt \
    --password-file ./backup-password.txt

# Automated backup with timestamp
pravyom wallet backup my-primary-wallet \
    --output "./backups/wallet-$(date +%Y%m%d-%H%M%S).json" \
    --encrypt
```

### Restoring Wallets
```bash
# Restore from encrypted backup
pravyom wallet restore \
    --input ./my-wallet-backup.json \
    --name "restored-wallet"

# Restore with custom name and verification
pravyom wallet restore \
    --input ./my-wallet-backup.json \
    --name "recovered-primary-wallet" \
    --verify-integrity

# Restore from mnemonic phrase
pravyom wallet restore \
    --mnemonic "word1 word2 word3 ... word24" \
    --name "mnemonic-restored-wallet" \
    --wallet-type docklock
```

### Backup Verification
```bash
# Verify backup integrity
pravyom wallet verify-backup \
    --backup-file ./my-wallet-backup.json

# Test restore without creating wallet
pravyom --dry-run wallet restore \
    --input ./my-wallet-backup.json \
    --name "test-restore"
```

## Advanced Wallet Features

### Multi-Signature Wallets
```bash
# Create 2-of-3 multisig wallet
pravyom wallet create-multisig \
    --name "company-treasury" \
    --threshold 2 \
    --signers "pubkey1,pubkey2,pubkey3" \
    --wallet-type metanode

# Add signer to existing multisig
pravyom wallet multisig add-signer \
    --wallet-id multisig-wallet-id \
    --signer-pubkey "ed25519:NEW_SIGNER_KEY"

# Remove signer from multisig
pravyom wallet multisig remove-signer \
    --wallet-id multisig-wallet-id \
    --signer-pubkey "ed25519:OLD_SIGNER_KEY"

# Create multisig transaction
pravyom wallet multisig create-transaction \
    --wallet-id multisig-wallet-id \
    --to bpci1qrecipient... \
    --amount 1000.0

# Sign multisig transaction
pravyom wallet multisig sign \
    --wallet-id multisig-wallet-id \
    --transaction-id tx-id \
    --signer-wallet my-signer-wallet

# Execute multisig transaction (when threshold met)
pravyom wallet multisig execute \
    --wallet-id multisig-wallet-id \
    --transaction-id tx-id
```

### Hardware Security Module (HSM) Integration
```bash
# Create HSM-backed wallet
pravyom wallet create \
    --name "hsm-secure-wallet" \
    --wallet-type metanode \
    --hsm-provider "aws-cloudhsm" \
    --hsm-key-id "key-12345"

# Import existing HSM key
pravyom wallet import-hsm \
    --name "imported-hsm-wallet" \
    --hsm-provider "azure-keyvault" \
    --hsm-key-id "vault-key-67890"

# Sign with HSM
pravyom wallet sign \
    --wallet-id hsm-secure-wallet \
    --data "HSM-signed message" \
    --use-hsm
```

### Wallet Import and Export

#### Exporting Wallets
```bash
# Export wallet in JSON format
pravyom wallet export my-primary-wallet --format json

# Export wallet in PEM format
pravyom wallet export my-primary-wallet --format pem

# Export public key only
pravyom wallet export my-primary-wallet --format json --public-only

# Export to file
pravyom wallet export my-primary-wallet \
    --format json \
    --output ./exported-wallet.json
```

#### Importing Wallets
```bash
# Import from private key
pravyom wallet import \
    --private-key "ed25519:PRIVATE_KEY_HEX" \
    --name "imported-wallet" \
    --key-type ed25519

# Import from JSON file
pravyom wallet import \
    --file ./exported-wallet.json \
    --name "imported-from-file"

# Import from mnemonic
pravyom wallet import \
    --mnemonic "abandon abandon abandon ... art" \
    --name "mnemonic-wallet" \
    --derivation-path "m/44'/60'/0'/0/0"

# Import with verification
pravyom wallet import \
    --private-key "PRIVATE_KEY" \
    --name "verified-import" \
    --verify-balance \
    --expected-address "bpci1qexpected..."
```

## Wallet Security and Verification

### Wallet Integrity Verification
```bash
# Verify wallet integrity
pravyom wallet verify my-primary-wallet

# Verify with signature checks
pravyom wallet verify my-primary-wallet --signatures

# Comprehensive verification
pravyom wallet verify my-primary-wallet \
    --signatures \
    --balance-check \
    --network-sync \
    --detailed
```

### Security Auditing
```bash
# Audit wallet security
pravyom wallet audit my-primary-wallet

# Security scan with recommendations
pravyom wallet security-scan my-primary-wallet --recommendations

# Check for security vulnerabilities
pravyom wallet vulnerability-check my-primary-wallet --detailed
```

### Access Control and Permissions
```bash
# Set wallet permissions
pravyom wallet set-permissions my-primary-wallet \
    --read-only false \
    --require-confirmation true \
    --max-daily-amount 1000.0

# Lock wallet temporarily
pravyom wallet lock my-primary-wallet --duration 3600  # 1 hour

# Unlock wallet
pravyom wallet unlock my-primary-wallet

# Change wallet password
pravyom wallet change-password my-primary-wallet
```

## Wallet Monitoring and Analytics

### Transaction History
```bash
# Show recent transactions
pravyom wallet history my-primary-wallet --limit 20

# Show transactions for specific period
pravyom wallet history my-primary-wallet \
    --from "2024-09-01" \
    --to "2024-09-05" \
    --detailed

# Filter by transaction type
pravyom wallet history my-primary-wallet \
    --type "send" \
    --token "GEN" \
    --limit 50

# Export transaction history
pravyom wallet history my-primary-wallet \
    --format csv \
    --output ./transaction-history.csv
```

### Wallet Analytics
```bash
# Show wallet analytics
pravyom wallet analytics my-primary-wallet

# Performance metrics
pravyom wallet metrics my-primary-wallet \
    --period "last-month" \
    --include-staking \
    --include-defi

# Portfolio analysis
pravyom wallet portfolio my-primary-wallet \
    --breakdown \
    --performance \
    --risk-analysis
```

## Automation and Scripting

### Automated Wallet Operations
```bash
#!/bin/bash
# automated-wallet-management.sh

WALLET_NAME="auto-managed-wallet"
BACKUP_DIR="/secure/backups"
LOG_FILE="/var/log/bpci/wallet-automation.log"

# Function to log with timestamp
log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $1" >> "$LOG_FILE"
}

# Create daily backup
create_daily_backup() {
    local backup_file="$BACKUP_DIR/wallet-$(date +%Y%m%d).json"
    
    log "Creating daily backup: $backup_file"
    
    if pravyom wallet backup "$WALLET_NAME" \
        --output "$backup_file" \
        --encrypt; then
        log "Backup created successfully"
        
        # Verify backup
        if pravyom wallet verify-backup --backup-file "$backup_file"; then
            log "Backup verified successfully"
        else
            log "ERROR: Backup verification failed"
            return 1
        fi
    else
        log "ERROR: Backup creation failed"
        return 1
    fi
}

# Monitor wallet balance
monitor_balance() {
    local balance=$(pravyom wallet balance "$WALLET_NAME" --format json | jq -r '.balances.native.amount')
    local threshold="100.0"
    
    if (( $(echo "$balance < $threshold" | bc -l) )); then
        log "WARNING: Wallet balance ($balance) below threshold ($threshold)"
        # Send alert (implement your notification system)
        # send_alert "Low wallet balance: $balance BPCI"
    else
        log "Balance check OK: $balance BPCI"
    fi
}

# Main execution
main() {
    log "Starting automated wallet management"
    
    # Check wallet status
    if pravyom wallet status "$WALLET_NAME" >/dev/null 2>&1; then
        log "Wallet status: OK"
        
        # Create backup
        create_daily_backup
        
        # Monitor balance
        monitor_balance
        
        # Cleanup old backups (keep 30 days)
        find "$BACKUP_DIR" -name "wallet-*.json" -mtime +30 -delete
        log "Cleaned up old backups"
        
    else
        log "ERROR: Wallet not accessible"
        exit 1
    fi
    
    log "Automated wallet management completed"
}

# Run main function
main "$@"
```

### Python Wallet Management
```python
#!/usr/bin/env python3
# wallet_manager.py

import subprocess
import json
import logging
from datetime import datetime, timedelta
from typing import Dict, List, Optional

class BPCIWalletManager:
    def __init__(self, network: str = "testnet"):
        self.network = network
        self.base_cmd = ["pravyom", "--network", network, "--format", "json"]
        
        # Setup logging
        logging.basicConfig(
            level=logging.INFO,
            format='%(asctime)s - %(levelname)s - %(message)s',
            handlers=[
                logging.FileHandler('/var/log/bpci/wallet-manager.log'),
                logging.StreamHandler()
            ]
        )
        self.logger = logging.getLogger(__name__)
    
    def run_command(self, cmd: List[str]) -> Dict:
        """Execute BPCI CLI command and return JSON result"""
        full_cmd = self.base_cmd + cmd
        try:
            result = subprocess.run(full_cmd, capture_output=True, text=True, check=True)
            return json.loads(result.stdout)
        except subprocess.CalledProcessError as e:
            self.logger.error(f"Command failed: {' '.join(full_cmd)}")
            self.logger.error(f"Error: {e.stderr}")
            raise
    
    def create_wallet(self, name: str, wallet_type: str = "docklock", 
                     key_type: str = "ed25519") -> str:
        """Create a new wallet"""
        self.logger.info(f"Creating wallet: {name}")
        
        result = self.run_command([
            "wallet", "create",
            "--name", name,
            "--wallet-type", wallet_type,
            "--key-type", key_type
        ])
        
        wallet_id = result["wallet_id"]
        self.logger.info(f"Wallet created successfully: {wallet_id}")
        return wallet_id
    
    def get_wallet_balance(self, wallet_id: str) -> Dict:
        """Get comprehensive wallet balance"""
        return self.run_command(["wallet", "balance", wallet_id, "--detailed"])
    
    def send_transaction(self, from_wallet: str, to_address: str, 
                        amount: float, token: Optional[str] = None) -> Dict:
        """Send transaction from wallet"""
        cmd = [
            "wallet", "send",
            "--from", from_wallet,
            "--to", to_address,
            "--amount", str(amount)
        ]
        
        if token:
            cmd.extend(["--token", token])
        
        self.logger.info(f"Sending {amount} {token or 'BPCI'} from {from_wallet} to {to_address}")
        return self.run_command(cmd)
    
    def backup_wallet(self, wallet_id: str, output_path: str, 
                     encrypt: bool = True) -> bool:
        """Create wallet backup"""
        cmd = ["wallet", "backup", wallet_id, "--output", output_path]
        if encrypt:
            cmd.append("--encrypt")
        
        try:
            self.run_command(cmd)
            self.logger.info(f"Wallet backup created: {output_path}")
            return True
        except Exception as e:
            self.logger.error(f"Backup failed: {e}")
            return False
    
    def monitor_wallets(self, wallet_ids: List[str], 
                       balance_threshold: float = 100.0) -> Dict:
        """Monitor multiple wallets for balance and status"""
        results = {}
        
        for wallet_id in wallet_ids:
            try:
                # Get wallet status
                status = self.run_command(["wallet", "status", wallet_id])
                balance_info = self.get_wallet_balance(wallet_id)
                
                native_balance = float(balance_info["balances"]["native"]["amount"])
                
                results[wallet_id] = {
                    "status": status["status"],
                    "balance": native_balance,
                    "below_threshold": native_balance < balance_threshold,
                    "last_activity": status["last_activity"],
                    "transaction_count": status["transaction_count"]
                }
                
                if native_balance < balance_threshold:
                    self.logger.warning(
                        f"Wallet {wallet_id} balance ({native_balance}) below threshold ({balance_threshold})"
                    )
                
            except Exception as e:
                self.logger.error(f"Failed to monitor wallet {wallet_id}: {e}")
                results[wallet_id] = {"error": str(e)}
        
        return results
    
    def automated_backup_rotation(self, wallet_ids: List[str], 
                                 backup_dir: str, retention_days: int = 30):
        """Automated backup with rotation"""
        import os
        from pathlib import Path
        
        backup_path = Path(backup_dir)
        backup_path.mkdir(parents=True, exist_ok=True)
        
        # Create new backups
        for wallet_id in wallet_ids:
            timestamp = datetime.now().strftime("%Y%m%d-%H%M%S")
            backup_file = backup_path / f"{wallet_id}-{timestamp}.json"
            
            if self.backup_wallet(wallet_id, str(backup_file)):
                self.logger.info(f"Backup created: {backup_file}")
        
        # Clean old backups
        cutoff_date = datetime.now() - timedelta(days=retention_days)
        
        for backup_file in backup_path.glob("*.json"):
            if backup_file.stat().st_mtime < cutoff_date.timestamp():
                backup_file.unlink()
                self.logger.info(f"Removed old backup: {backup_file}")

# Example usage
if __name__ == "__main__":
    manager = BPCIWalletManager(network="testnet")
    
    # Create test wallet
    wallet_id = manager.create_wallet("test-automation-wallet")
    
    # Monitor wallet
    results = manager.monitor_wallets([wallet_id])
    print(f"Monitoring results: {results}")
    
    # Create backup
    manager.backup_wallet(wallet_id, f"./backup-{wallet_id}.json")
```

## Troubleshooting Common Issues

### Issue 1: Wallet Creation Failed
```bash
# Check system requirements
pravyom --verbose wallet create --name test-wallet

# Verify entropy source
cat /proc/sys/kernel/random/entropy_avail

# Check disk space
df -h ~/.config/bpci

# Verify permissions
ls -la ~/.config/bpci/wallets/
```

### Issue 2: Transaction Failures
```bash
# Check wallet balance
pravyom wallet balance wallet-id --detailed

# Verify network connectivity
pravyom network status

# Check gas settings
pravyom --dry-run wallet send --from wallet-id --to recipient --amount 1.0

# Review transaction logs
pravyom wallet history wallet-id --limit 10
```

### Issue 3: Backup/Restore Issues
```bash
# Verify backup integrity
pravyom wallet verify-backup --backup-file backup.json

# Test restore in dry-run mode
pravyom --dry-run wallet restore --input backup.json --name test-restore

# Check backup file permissions
ls -la backup.json

# Verify encryption status
file backup.json
```

### Issue 4: Performance Issues
```bash
# Check wallet database integrity
pravyom wallet verify wallet-id --signatures

# Optimize wallet database
pravyom maintenance database --wallet-id wallet-id --optimize

# Monitor system resources
pravyom maintenance metrics --component wallet --live
```

## Best Practices

### Security Best Practices
1. **Always use encrypted backups** for production wallets
2. **Regular backup rotation** with secure storage
3. **Multi-signature wallets** for high-value operations
4. **Hardware security modules** for enterprise deployments
5. **Regular security audits** and vulnerability checks

### Operational Best Practices
1. **Test operations on testnet** before mainnet deployment
2. **Use dry-run mode** for transaction validation
3. **Monitor wallet balances** and set up alerts
4. **Implement automated backup** strategies
5. **Document wallet management** procedures

### Performance Optimization
1. **Regular wallet verification** to maintain integrity
2. **Database optimization** for large transaction volumes
3. **Efficient key derivation** paths for HD wallets
4. **Batch operations** when possible
5. **Monitor system resources** during operations

---

**Previous**: [CLI Overview and Setup](01-cli-overview-and-setup.md)  
**Next**: [Mining Operations Guide](03-mining-operations-guide.md)  
**Related**: [Network Management](04-network-management-guide.md), [Security Best Practices](../07-firewall-and-security/)
