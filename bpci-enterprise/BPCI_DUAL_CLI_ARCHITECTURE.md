# BPCI Dual CLI Architecture

**Document Date**: 2025-10-26  
**Purpose**: Define two distinct CLI tools with clear separation of concerns  
**Target**: Production-grade access control and operational security

---

## **Executive Summary**

BPCI requires **two separate CLI tools** with distinct purposes, permissions, and user bases:

1. **Wallet Admin CLI** (`bpi-wallet`) - For all BPI developers
2. **Super Admin/CESS CLI** (`bpci-admin`) - For company/maintainer/infrastructure team

These tools have **completely different access levels**, **audit requirements**, and **operational scopes**.

---

## **CLI Tool Comparison**

| Aspect | Wallet Admin CLI | Super Admin/CESS CLI |
|--------|------------------|----------------------|
| **Binary Name** | `bpi-wallet` | `bpci-admin` |
| **User Base** | All BPI developers | Company/maintainer team only |
| **Access Level** | User-level operations | System-level operations |
| **Scope** | Wallet, transactions, DApps | Infrastructure, orchestration, diagnostics |
| **Authentication** | Wallet signature | Multi-factor + hardware key |
| **Audit Level** | Standard logging | Enhanced audit trail |
| **Distribution** | Public (open source) | Private (internal only) |
| **Installation** | `cargo install bpi-wallet` | Restricted deployment |
| **Network Access** | BPI node endpoints | BPCI admin endpoints |
| **Emergency Powers** | None | Full system control |

---

## **1. Wallet Admin CLI (`bpi-wallet`)**

### **Purpose**
Enable BPI developers to manage wallets, submit transactions, deploy DApps, and interact with the BPI network **without any infrastructure access**.

### **Target Users**
- BPI application developers
- DApp creators
- Community members
- Third-party integrators
- Enterprise developers using BPI

### **Access Level**
- ✅ **User-level operations only**
- ✅ **No infrastructure access**
- ✅ **No server configuration**
- ✅ **No system diagnostics**
- ❌ **Cannot modify BPCI servers**
- ❌ **Cannot access admin APIs**
- ❌ **Cannot view system internals**

---

### **Wallet Admin CLI - Command Structure**

#### **1.1 Wallet Management**
```bash
# Create and manage wallets
bpi-wallet create                              # Create new wallet
bpi-wallet import --mnemonic "..."             # Import from mnemonic
bpi-wallet import --private-key "..."          # Import from private key
bpi-wallet list                                # List all wallets
bpi-wallet show <address>                      # Show wallet details
bpi-wallet balance <address>                   # Check balance
bpi-wallet export <address>                    # Export wallet (encrypted)
bpi-wallet delete <address>                    # Delete wallet (with confirmation)

# Wallet security
bpi-wallet change-password <address>           # Change wallet password
bpi-wallet backup <address> --output backup.json  # Backup wallet
bpi-wallet restore --file backup.json          # Restore from backup
```

#### **1.2 Transaction Operations**
```bash
# Send transactions
bpi-wallet send --from <address> --to <address> --amount <amount>
bpi-wallet send --from <address> --to <address> --amount <amount> --token <token>
bpi-wallet send-batch --file transactions.json  # Batch transactions

# Transaction history
bpi-wallet history <address>                   # Transaction history
bpi-wallet history <address> --limit 100       # Last 100 transactions
bpi-wallet tx-status <tx-hash>                 # Check transaction status
bpi-wallet tx-details <tx-hash>                # Transaction details

# Transaction management
bpi-wallet pending <address>                   # Show pending transactions
bpi-wallet cancel <tx-hash>                    # Cancel pending transaction
bpi-wallet speed-up <tx-hash>                  # Speed up transaction (higher fee)
```

#### **1.3 Token Management**
```bash
# Token operations
bpi-wallet token-balance <address> <token>     # Check token balance
bpi-wallet token-list <address>                # List all tokens
bpi-wallet token-info <token-address>          # Token information
bpi-wallet token-transfer --from <addr> --to <addr> --token <token> --amount <amt>

# Token creation (if permitted)
bpi-wallet token-create --name "MyToken" --symbol "MTK" --supply 1000000
bpi-wallet token-mint <token> --amount 1000    # Mint tokens (if owner)
bpi-wallet token-burn <token> --amount 500     # Burn tokens
```

#### **1.4 DApp Deployment**
```bash
# Deploy DApps
bpi-wallet dapp-deploy --file contract.wasm --from <address>
bpi-wallet dapp-deploy --file contract.wasm --from <address> --args "arg1,arg2"
bpi-wallet dapp-upgrade <contract-address> --file new-contract.wasm

# DApp interaction
bpi-wallet dapp-call <contract-address> --method "transfer" --args "addr,100"
bpi-wallet dapp-query <contract-address> --method "balance" --args "addr"
bpi-wallet dapp-info <contract-address>        # Contract information
bpi-wallet dapp-list <owner-address>           # List owned contracts

# DApp management
bpi-wallet dapp-pause <contract-address>       # Pause contract (if owner)
bpi-wallet dapp-resume <contract-address>      # Resume contract
bpi-wallet dapp-transfer-ownership <contract> --to <new-owner>
```

#### **1.5 Auction Participation**
```bash
# Auction operations
bpi-wallet auction-list                        # List active auctions
bpi-wallet auction-info <auction-id>           # Auction details
bpi-wallet auction-bid <auction-id> --amount <amount> --from <address>
bpi-wallet auction-history <address>           # Auction participation history
bpi-wallet auction-winnings <address>          # Check winnings
bpi-wallet auction-claim <auction-id>          # Claim auction winnings
```

#### **1.6 Staking & Governance**
```bash
# Staking operations
bpi-wallet stake --from <address> --amount <amount>
bpi-wallet unstake --from <address> --amount <amount>
bpi-wallet staking-info <address>              # Staking information
bpi-wallet rewards <address>                   # Check staking rewards
bpi-wallet claim-rewards <address>             # Claim rewards

# Governance participation
bpi-wallet governance-proposals                # List proposals
bpi-wallet governance-vote <proposal-id> --vote yes --from <address>
bpi-wallet governance-create-proposal --file proposal.json --from <address>
bpi-wallet governance-history <address>        # Voting history
```

#### **1.7 Network Information**
```bash
# Network queries (read-only)
bpi-wallet network-info                        # Network information
bpi-wallet network-status                      # Network status
bpi-wallet node-list                           # List available nodes
bpi-wallet node-connect <node-url>             # Connect to specific node
bpi-wallet gas-price                           # Current gas price
bpi-wallet block-info <block-number>           # Block information
bpi-wallet block-latest                        # Latest block
```

#### **1.8 Account Management**
```bash
# Account operations
bpi-wallet account-info <address>              # Account information
bpi-wallet account-nonce <address>             # Account nonce
bpi-wallet account-permissions <address>       # Account permissions
bpi-wallet account-update --from <address> --name "MyAccount"

# Multi-signature
bpi-wallet multisig-create --signers <addr1,addr2,addr3> --threshold 2
bpi-wallet multisig-sign <multisig-address> <tx-hash> --from <address>
bpi-wallet multisig-execute <multisig-address> <tx-hash>
```

#### **1.9 Configuration**
```bash
# CLI configuration (local only)
bpi-wallet config-set --key network --value mainnet
bpi-wallet config-set --key node-url --value "https://node.example.com"
bpi-wallet config-get --key network
bpi-wallet config-list                         # List all config
bpi-wallet config-reset                        # Reset to defaults
```

---

### **Wallet Admin CLI - Security Model**

#### **Authentication**
- **Wallet Signature**: All operations require wallet signature
- **Password Protection**: Wallets encrypted with user password
- **No System Access**: Cannot access BPCI infrastructure
- **Rate Limited**: Standard rate limits apply

#### **Permissions**
```yaml
wallet_admin_permissions:
  can_do:
    - Create/manage own wallets
    - Send/receive transactions
    - Deploy/interact with DApps
    - Participate in auctions
    - Stake tokens
    - Vote on governance proposals
    - Query network information (read-only)
  
  cannot_do:
    - Access BPCI admin APIs
    - Modify server configuration
    - View system diagnostics
    - Control infrastructure
    - Access other users' data
    - Bypass rate limits
    - Emergency operations
```

#### **Audit Trail**
- Standard transaction logging
- User action history
- No sensitive system information
- Privacy-preserving logs

---

## **2. Super Admin/CESS CLI (`bpci-admin`)**

### **Purpose**
Provide **complete infrastructure control** for company/maintainer team to manage, monitor, diagnose, and orchestrate all BPCI servers **without touching server code**.

### **Target Users**
- Company infrastructure team
- System maintainers
- DevOps engineers
- Security team
- Emergency responders

### **Access Level**
- ✅ **Full system access**
- ✅ **Infrastructure control**
- ✅ **Server configuration**
- ✅ **System diagnostics**
- ✅ **Emergency powers**
- ✅ **Orchestration control**
- ✅ **Security operations**

---

### **Super Admin CLI - Command Structure**

#### **2.1 Configuration Management**
```bash
# Configuration operations
bpci-admin config reload --server <server-id>
bpci-admin config reload --all                 # Reload all servers
bpci-admin config get --server <server-id>
bpci-admin config get --server <server-id> --key policies.consensus.min_validators
bpci-admin config set --server <server-id> --key <key> --value <value>
bpci-admin config validate --file config.yaml
bpci-admin config diff --server <server-id> --file new-config.yaml
bpci-admin config apply --server <server-id> --file config.yaml
bpci-admin config history --server <server-id>  # Configuration change history
bpci-admin config rollback --server <server-id> --version <version>
bpci-admin config export --server <server-id> --output config.yaml
bpci-admin config import --server <server-id> --file config.yaml
```

#### **2.2 Orchestration Management**
```bash
# Flow management
bpci-admin flow list --server <server-id>
bpci-admin flow list --all                     # All flows across all servers
bpci-admin flow create --file flow.yaml
bpci-admin flow update --id <flow-id> --file updated-flow.yaml
bpci-admin flow delete --id <flow-id>
bpci-admin flow test --id <flow-id> --dry-run
bpci-admin flow activate --id <flow-id>
bpci-admin flow deactivate --id <flow-id>
bpci-admin flow status --id <flow-id>
bpci-admin flow history --id <flow-id>         # Flow execution history
bpci-admin flow metrics --id <flow-id>         # Flow performance metrics

# Flow debugging
bpci-admin flow trace --execution-id <exec-id>
bpci-admin flow logs --id <flow-id> --tail 100
bpci-admin flow errors --id <flow-id>
```

#### **2.3 Communication Management**
```bash
# Message routing
bpci-admin comm routes --server <server-id>
bpci-admin comm routes --all                   # All routes
bpci-admin comm add-route --from <component> --to <component> --message-type <type>
bpci-admin comm update-route --id <route-id> --timeout 5000
bpci-admin comm delete-route --id <route-id>
bpci-admin comm test --from <component> --to <component> --message-type <type>

# Communication monitoring
bpci-admin comm stats --server <server-id>
bpci-admin comm stats --all                    # System-wide stats
bpci-admin comm latency --from <comp> --to <comp>
bpci-admin comm errors --server <server-id>
bpci-admin comm trace --message-id <msg-id>

# Communication control
bpci-admin comm pause --from <comp> --to <comp>
bpci-admin comm resume --from <comp> --to <comp>
bpci-admin comm clear-queue --server <server-id>
```

#### **2.4 Endpoint Management**
```bash
# Endpoint operations
bpci-admin endpoint list --server <server-id>
bpci-admin endpoint register --server <server-id> --path "/api/v1/custom" --handler <handler>
bpci-admin endpoint unregister --server <server-id> --path "/api/v1/custom"
bpci-admin endpoint update --server <server-id> --path <path> --rate-limit 2000
bpci-admin endpoint enable --server <server-id> --path <path>
bpci-admin endpoint disable --server <server-id> --path <path>

# Endpoint monitoring
bpci-admin endpoint stats --server <server-id> --path <path>
bpci-admin endpoint logs --server <server-id> --path <path> --tail 100
bpci-admin endpoint errors --server <server-id> --path <path>
```

#### **2.5 Policy Management**
```bash
# Policy operations
bpci-admin policy list --server <server-id>
bpci-admin policy create --file policy.yaml
bpci-admin policy update --id <policy-id> --file updated-policy.yaml
bpci-admin policy delete --id <policy-id>
bpci-admin policy test --id <policy-id> --input test-data.json
bpci-admin policy activate --id <policy-id>
bpci-admin policy deactivate --id <policy-id>

# Policy enforcement
bpci-admin policy enforce --id <policy-id> --target <component>
bpci-admin policy violations --server <server-id>
bpci-admin policy audit --id <policy-id>
```

#### **2.6 Diagnostics & Debugging**
```bash
# Health checks
bpci-admin diag health --server <server-id>
bpci-admin diag health --all                   # All servers
bpci-admin diag health --component <component>

# Metrics
bpci-admin diag metrics --server <server-id>
bpci-admin diag metrics --all
bpci-admin diag metrics --component <component> --metric cpu_usage

# Logs
bpci-admin diag logs --server <server-id> --tail 1000
bpci-admin diag logs --server <server-id> --level error
bpci-admin diag logs --server <server-id> --grep "consensus"
bpci-admin diag logs --all --level critical

# Tracing
bpci-admin diag trace --request-id <req-id>
bpci-admin diag trace --transaction-id <tx-id>
bpci-admin diag trace --flow-execution-id <exec-id>

# System information
bpci-admin diag connections --server <server-id>
bpci-admin diag threads --server <server-id>
bpci-admin diag memory --server <server-id>
bpci-admin diag cpu --server <server-id>
bpci-admin diag disk --server <server-id>
bpci-admin diag network --server <server-id>

# Performance profiling
bpci-admin diag profile --server <server-id> --duration 60s
bpci-admin diag flamegraph --server <server-id> --output flame.svg
bpci-admin diag bottlenecks --server <server-id>
```

#### **2.7 Emergency Controls**
```bash
# Circuit breaker
bpci-admin emergency circuit-breaker open --server <server-id>
bpci-admin emergency circuit-breaker close --server <server-id>
bpci-admin emergency circuit-breaker status --server <server-id>

# Rate limiting
bpci-admin emergency rate-limit adjust --server <server-id> --limit 500
bpci-admin emergency rate-limit disable --server <server-id>
bpci-admin emergency rate-limit enable --server <server-id>

# Component control
bpci-admin emergency isolate --component <component>
bpci-admin emergency restore --component <component>
bpci-admin emergency restart --server <server-id>
bpci-admin emergency shutdown --server <server-id> --graceful
bpci-admin emergency force-shutdown --server <server-id>

# System-wide emergency
bpci-admin emergency system-pause                # Pause entire system
bpci-admin emergency system-resume               # Resume system
bpci-admin emergency system-rollback --version <version>
bpci-admin emergency system-snapshot             # Create system snapshot
```

#### **2.8 Plugin Management**
```bash
# Plugin operations
bpci-admin plugin list --server <server-id>
bpci-admin plugin load --server <server-id> --file plugin.so
bpci-admin plugin unload --server <server-id> --plugin <plugin-name>
bpci-admin plugin reload --server <server-id> --plugin <plugin-name>
bpci-admin plugin enable --server <server-id> --plugin <plugin-name>
bpci-admin plugin disable --server <server-id> --plugin <plugin-name>
bpci-admin plugin info --plugin <plugin-name>
bpci-admin plugin logs --plugin <plugin-name> --tail 100
```

#### **2.9 Self-Healing Management**
```bash
# Self-healing operations
bpci-admin healing status --server <server-id>
bpci-admin healing enable --server <server-id>
bpci-admin healing disable --server <server-id>
bpci-admin healing strategies --server <server-id>
bpci-admin healing add-strategy --file strategy.yaml
bpci-admin healing test-strategy --id <strategy-id>
bpci-admin healing history --server <server-id>
bpci-admin healing metrics --server <server-id>
```

#### **2.10 Cluster Management**
```bash
# Cluster operations
bpci-admin cluster status                      # Cluster-wide status
bpci-admin cluster nodes                       # List all nodes
bpci-admin cluster add-node --server <server-id> --endpoint <url>
bpci-admin cluster remove-node --server <server-id>
bpci-admin cluster rebalance                   # Rebalance load
bpci-admin cluster failover --from <server> --to <server>

# Cluster coordination
bpci-admin cluster sync                        # Force sync
bpci-admin cluster leader                      # Show cluster leader
bpci-admin cluster election                    # Trigger leader election
```

#### **2.11 Security Operations**
```bash
# Security management
bpci-admin security audit --server <server-id>
bpci-admin security scan --server <server-id>
bpci-admin security vulnerabilities --server <server-id>
bpci-admin security patch --server <server-id> --patch <patch-id>

# Access control
bpci-admin security permissions --user <user-id>
bpci-admin security grant --user <user-id> --permission <perm>
bpci-admin security revoke --user <user-id> --permission <perm>
bpci-admin security audit-log --user <user-id>

# Certificate management
bpci-admin security cert-list --server <server-id>
bpci-admin security cert-renew --server <server-id>
bpci-admin security cert-rotate --server <server-id>
```

#### **2.12 Monitoring & Alerting**
```bash
# Monitoring setup
bpci-admin monitor enable --server <server-id>
bpci-admin monitor disable --server <server-id>
bpci-admin monitor status --server <server-id>
bpci-admin monitor metrics --server <server-id>

# Alerting
bpci-admin alert list --server <server-id>
bpci-admin alert create --file alert-rule.yaml
bpci-admin alert update --id <alert-id> --file updated-rule.yaml
bpci-admin alert delete --id <alert-id>
bpci-admin alert test --id <alert-id>
bpci-admin alert history --server <server-id>
```

---

### **Super Admin CLI - Security Model**

#### **Authentication**
- **Multi-Factor Authentication**: Required for all operations
- **Hardware Key**: YubiKey or similar required
- **IP Whitelist**: Only from authorized networks
- **Session Timeout**: 15 minutes of inactivity
- **Audit Trail**: Every command logged with full context

#### **Permissions**
```yaml
super_admin_permissions:
  full_access:
    - All configuration management
    - All orchestration control
    - All communication management
    - All endpoint management
    - All policy management
    - All diagnostics and debugging
    - All emergency controls
    - All plugin management
    - All self-healing management
    - All cluster management
    - All security operations
    - All monitoring and alerting
  
  restrictions:
    - Cannot access user wallet private keys
    - Cannot modify user transactions (immutable)
    - Cannot bypass audit logging
    - Requires approval for destructive operations
    - Time-limited emergency access
```

#### **Audit Trail**
- **Enhanced Logging**: Every command with full context
- **Immutable Audit**: Cannot be modified or deleted
- **Real-time Alerts**: Critical operations trigger alerts
- **Compliance**: SOC2, ISO27001, PCI-DSS compliant
- **Forensics**: Complete audit trail for investigation

---

## **Access Control Matrix**

| Operation | Wallet Admin CLI | Super Admin CLI |
|-----------|------------------|-----------------|
| **Wallet Management** | ✅ Own wallets only | ❌ No wallet access |
| **Send Transactions** | ✅ Own transactions | ❌ Cannot send |
| **Deploy DApps** | ✅ Yes | ❌ No |
| **View Network Info** | ✅ Read-only | ✅ Full access |
| **Server Configuration** | ❌ No access | ✅ Full control |
| **Orchestration Flows** | ❌ No access | ✅ Full control |
| **Communication Routes** | ❌ No access | ✅ Full control |
| **Diagnostics** | ❌ No access | ✅ Full access |
| **Emergency Controls** | ❌ No access | ✅ Full access |
| **Plugin Management** | ❌ No access | ✅ Full control |
| **Security Operations** | ❌ No access | ✅ Full access |
| **Audit Logs** | ✅ Own actions | ✅ All actions |

---

## **Distribution & Installation**

### **Wallet Admin CLI**
```bash
# Public installation (open source)
cargo install bpi-wallet

# Or download binary
wget https://releases.bpi.network/bpi-wallet-latest-linux-x64
chmod +x bpi-wallet-latest-linux-x64
sudo mv bpi-wallet-latest-linux-x64 /usr/local/bin/bpi-wallet

# First-time setup
bpi-wallet init
bpi-wallet config-set --key network --value mainnet
```

### **Super Admin CLI**
```bash
# Restricted installation (internal only)
# Requires authentication and authorization

# Step 1: Request access
bpci-admin-installer request-access --email admin@company.com

# Step 2: Receive approval and download token
# (Sent via secure channel)

# Step 3: Install with token
bpci-admin-installer install --token <secure-token>

# Step 4: Setup MFA
bpci-admin setup-mfa --hardware-key

# Step 5: Verify installation
bpci-admin verify-installation
```

---

## **Configuration Files**

### **Wallet Admin CLI Config**
```yaml
# ~/.bpi-wallet/config.yaml
network: mainnet
node_url: https://node.bpi.network
default_wallet: 0x1234...
gas_price_strategy: medium
timeout_seconds: 30
retry_attempts: 3
```

### **Super Admin CLI Config**
```yaml
# /etc/bpci-admin/config.yaml
authentication:
  mfa_required: true
  hardware_key_required: true
  session_timeout_minutes: 15
  ip_whitelist:
    - 10.0.0.0/8
    - 192.168.1.0/24

audit:
  enabled: true
  log_level: debug
  immutable: true
  real_time_alerts: true

servers:
  - id: consensus-001
    endpoint: https://consensus-001.internal:19001
    component: Consensus
  - id: blockchain-001
    endpoint: https://blockchain-001.internal:18080
    component: Blockchain
  # ... more servers

emergency:
  approval_required: true
  approvers:
    - admin1@company.com
    - admin2@company.com
  timeout_minutes: 60
```

---

## **Audit & Compliance**

### **Wallet Admin CLI Audit**
```json
{
  "timestamp": "2025-10-26T10:22:57Z",
  "user": "0x1234...",
  "command": "bpi-wallet send",
  "args": {
    "from": "0x1234...",
    "to": "0x5678...",
    "amount": "100"
  },
  "result": "success",
  "tx_hash": "0xabcd..."
}
```

### **Super Admin CLI Audit**
```json
{
  "timestamp": "2025-10-26T10:22:57Z",
  "admin_id": "admin@company.com",
  "session_id": "sess_abc123",
  "mfa_verified": true,
  "hardware_key": "yubikey_xyz",
  "ip_address": "10.0.0.5",
  "command": "bpci-admin config set",
  "args": {
    "server": "consensus-001",
    "key": "policies.consensus.min_validators",
    "value": "5"
  },
  "previous_value": "3",
  "new_value": "5",
  "result": "success",
  "approval_required": false,
  "approver": null,
  "impact_assessment": "medium",
  "rollback_available": true,
  "rollback_version": "v1.2.3"
}
```

---

## **Implementation Priority**

### **Phase 1: Wallet Admin CLI (Week 1-2)**
1. Wallet management commands
2. Transaction operations
3. Token management
4. DApp deployment
5. Network queries

### **Phase 2: Super Admin CLI (Week 3-4)**
1. Configuration management
2. Orchestration control
3. Communication management
4. Diagnostics
5. Emergency controls

### **Phase 3: Security & Audit (Week 5)**
1. MFA implementation
2. Hardware key support
3. Audit logging
4. Access control
5. Compliance reporting

---

## **Success Criteria**

### **Wallet Admin CLI**
- ✅ All user operations available
- ✅ No infrastructure access
- ✅ Easy to use for developers
- ✅ Well-documented
- ✅ Open source ready

### **Super Admin CLI**
- ✅ Complete infrastructure control
- ✅ Zero-touch maintenance capability
- ✅ Enhanced security
- ✅ Full audit trail
- ✅ Emergency response ready

---

## **Conclusion**

The dual CLI architecture provides:

1. **Clear Separation**: User operations vs infrastructure operations
2. **Security**: Different authentication and authorization levels
3. **Audit**: Appropriate logging for each access level
4. **Usability**: Right tool for the right user
5. **Compliance**: Enterprise-grade access control

**Wallet Admin CLI** (`bpi-wallet`): For developers to build on BPI  
**Super Admin CLI** (`bpci-admin`): For maintainers to operate BPCI

Both tools enable their respective users to be **fully productive** without overlapping concerns or security risks.

---

**Document Status**: ✅ Complete  
**Next Steps**: Begin implementation  
**Review Date**: 2025-11-02
