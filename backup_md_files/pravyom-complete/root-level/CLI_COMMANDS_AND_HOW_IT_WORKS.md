# CLI Commands and How It Works - Metanode Enterprise Infrastructure

## Overview

This document systematically documents the actual available CLI commands and how each component works in the Metanode enterprise blockchain infrastructure.

---

## 1. BPI CLI (`bpi`) - Main Blockchain Infrastructure

### Available Commands
```bash
bpi --help
```

**Main Commands:**
- `bank` - Bank and validator operations
- `coin` - Coin lifecycle and management
- `settle` - Cross-border settlement operations
- `receipt` - Receipt operations
- `biso` - BISO policy operations
- `economics` - Economics and PoE operations
- `gov` - Governance operations
- `mesh` - Service mesh operations
- `container` - Container operations
- `testnet` - Testnet operations
- `analytics` - Analytics operations
- `security` - Security operations
- `completion` - Generate shell completion scripts
- `update` - Update operations

### Global Options
- `--json` - Output in JSON format
- `--dry-run` - Show what would be done without executing
- `-y, --yes` - Automatically answer yes to prompts
- `-v, --verbose` - Enable verbose output
- `--metanode-home <PATH>` - Metanode home directory

---

## 2. Mesh Operations (`bpi mesh`)

### Available Subcommands
```bash
bpi mesh --help
```

**Subcommands:**
- `status` - Show mesh status
- `deploy` - Deploy to mesh

### How It Works
The mesh component provides service mesh operations for the blockchain infrastructure. It handles service discovery, load balancing, and inter-service communication.

### Usage Examples
```bash
# Deploy to mesh
bpi mesh deploy --json --verbose

# Check mesh status
bpi mesh status --json
```

---

## 3. Container Operations (`bpi container`)

### Available Subcommands
```bash
bpi container --help
```

**Subcommands:**
- `run` - Run container
- `list` - List containers

### How It Works
The container component provides container orchestration capabilities, likely integrating with Docklock for deterministic execution and security.

### Usage Examples
```bash
# Run a container
bpi container run --json --verbose

# List containers
bpi container list --json
```

---

## 4. Receipt Operations (`bpi receipt`)

### Available Subcommands
```bash
bpi receipt --help
```

**How It Works**
The receipt system provides cryptographic verification and audit trails for blockchain operations. It generates receipts for transactions and operations that can be verified later.

### Usage Examples
```bash
# Generate receipts
bpi receipt generate --json --output-dir /path/to/receipts/
```

---

## 5. Agreement CLI (`agreementc`) - Enterprise Agreement Management

### Available Commands
```bash
agreementc --help
```

**Main Commands:**
- `init` - Initialize a new agreement workspace
- `court` - Court management operations
- `policy` - Policy management operations
- `agreement` - Agreement management operations
- `template` - Agreement template operations
- `enforce` - Enforce agreement policies
- `validate` - Validate agreements and policies
- `wallet` - Wallet box agreement operations
- `export` - Export agreements and policies
- `import` - Import agreements and policies
- `status` - Show workspace status

### Global Options
- `--json` - Output in JSON format
- `--dry-run` - Show what would be done without executing
- `-y, --yes` - Automatically answer yes to prompts
- `-v, --verbose` - Enable verbose output
- `--workspace <PATH>` - Agreement workspace directory

---

## 6. Court Management (`agreementc court`)

### Available Subcommands
```bash
agreementc court --help
```

**Subcommands:**
- `create` - Create a new court
- `list` - List all courts
- `info` - Get court information

### How It Works
Courts are governance entities that host policies and agreements. They provide a framework for policy enforcement and agreement management.

### Usage Examples
```bash
# Create a court
agreementc court create --workspace /path/to/workspace --name "MyCourt" --description "Description"

# List courts
agreementc court list --workspace /path/to/workspace --json

# Get court info
agreementc court info --workspace /path/to/workspace --name "MyCourtName"
```

---

## 7. Policy Management (`agreementc policy`)

### Available Subcommands
```bash
agreementc policy --help
```

**Subcommands:**
- `deploy` - Deploy a policy to a court
- `list` - List policies
- `info` - Get policy information

### How It Works
Policies define rules and constraints that can be enforced through the agreement system. They are deployed using WASM bytecode files.

### Verified Usage Examples
```bash
# Deploy a policy (VERIFIED SYNTAX)
agreementc policy deploy \
    --workspace /path/to/workspace \
    --name "PolicyName" \
    --version "1.0" \
    --wasm-file /path/to/policy.wasm \
    --json

# Optional flags for policy deploy:
# --pre-hook    - Enable as pre-hook
# --post-hook   - Enable as post-hook

# List policies
agreementc policy list --workspace /path/to/workspace --json

# Get policy info
agreementc policy info --workspace /path/to/workspace --name "PolicyName"
```

---

## 8. Template Operations (`agreementc template`)

### Available Subcommands
```bash
agreementc template --help
```

**Subcommands:**
- `list` - List available templates
- `show` - Show template details
- `generate` - Generate agreement from template

### How It Works
Templates provide pre-defined agreement structures that can be used to create standardized agreements for common use cases.

### Usage Examples
```bash
# List available templates
agreementc template list --workspace /path/to/workspace --json

# Show template details
agreementc template show --workspace /path/to/workspace --name "TemplateName"

# Generate agreement from template
agreementc template generate --workspace /path/to/workspace --template "TemplateName" --output "agreement.json"
```

---

## 9. Agreement Management (`agreementc agreement`)

### Available Subcommands
```bash
agreementc agreement --help
```

**How It Works**
Agreements are contracts between parties that can be enforced through the policy system. They define terms, conditions, and enforcement mechanisms.

### Verified Usage Examples
```bash
# Create an agreement (VERIFIED SYNTAX)
agreementc agreement create \
    --workspace /path/to/workspace \
    --name "AgreementName" \
    --version "1.0" \
    --parties "party1,party2,party3" \
    --policies "policy-id-1,policy-id-2" \
    --terms "Agreement terms and conditions" \
    --json
```

---

## 10. Agreement Enforcement (`agreementc enforce`)

### Available Subcommands
```bash
agreementc enforce --help
```

**How It Works**
The enforcement system monitors agreements to ensure they comply with their terms and policies. It takes an agreement ID as the primary argument.

### Verified Usage Examples
```bash
# Enforce an agreement (VERIFIED SYNTAX)
agreementc enforce \
    --workspace /path/to/workspace \
    --court-id "court-id-123" \
    agreement-id-456 \
    --json
```

---

## 11. Receipt Operations (`bpi receipt`)

### Available Subcommands
```bash
bpi receipt --help
```

**Verified Subcommands:**
- `verify` - Verify a receipt
- `export` - Export receipts

**How It Works**
The receipt system provides cryptographic verification and audit trails. Note: There is NO `generate` subcommand - receipts are likely generated automatically by other operations.

### Verified Usage Examples
```bash
# Verify a receipt
bpi receipt verify --json

# Export receipts
bpi receipt export --json
```

---

## 12. Workspace Management (`agreementc init` and `agreementc status`)

### How It Works
Workspaces provide isolated environments for managing agreements, courts, and policies. They contain all the necessary files and configurations.

### Usage Examples
```bash
# Initialize workspace
agreementc init --workspace /path/to/workspace --name "WorkspaceName"

# Check workspace status
agreementc status --workspace /path/to/workspace --json
```

---

## 13. Gateway Service (`gateway`)

### Available Options
```bash
gateway --help
```

**How It Works**
The gateway service (BPI Mesh Gateway Agent) provides API management, load balancing, and routing for the blockchain infrastructure with advanced features like circuit breakers and health checks.

### Verified Usage Examples
```bash
# Start gateway service (VERIFIED SYNTAX)
gateway \
    --gateway-id "gateway-001" \
    --listen-addr "127.0.0.1:18000" \
    --relay-endpoints "http://127.0.0.1:8001,http://127.0.0.1:8002" \
    --health-check-interval-ms 5000 \
    --max-connections 1000 \
    --load-balancing "round-robin" \
    --daemon \
    --metrics-enabled \
    --log-level "info"

# Minimal start
gateway --listen-addr "127.0.0.1:18000" --daemon
```

---

## 14. Mempool Service (`mempool`)

### Available Options
```bash
mempool --help
```

**How It Works**
The mempool service (BPI Encrypted Mempool Service) manages encrypted transaction pools with DoS protection and batch processing capabilities.

### Verified Usage Examples
```bash
# Start mempool service (VERIFIED SYNTAX)
mempool \
    --port 19000 \
    --max-pending 10000 \
    --reveal-timeout 30000 \
    --dos-limit 100 \
    --batch-size 100 \
    --daemon

# Minimal start
mempool --port 19000 --daemon
```

---

## Next Steps

1. **Verify CLI Command Syntax**: Test each command with `--help` to get exact parameter requirements
2. **Create Working Examples**: Build deployment scripts using only verified CLI commands
3. **Test Integration**: Ensure all components work together properly
4. **Document Limitations**: Note any CLI commands that don't work as expected

---

## Notes

- All CLI commands support `--json` for machine-readable output
- Most commands support `--dry-run` for testing without execution
- Workspace paths are required for most `agreementc` operations
- Some commands may require additional parameters not yet documented
