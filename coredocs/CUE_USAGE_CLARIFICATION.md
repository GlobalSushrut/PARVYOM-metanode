# CUE Usage in Metanode Project
## Clear Definition of Where and How CUE Will Be Used

### 🎯 Specific CUE Use Cases in Metanode

**CUE will be used by developers to write:**

1. **Agreement Smart Contracts (YAML-like files)**
   - Court Node YAML SmartContracts++
   - Business logic agreements
   - SLA definitions and enforcement rules

2. **DockLock Container Specifications**
   - Container deployment configs
   - Resource allocation
   - Network and security settings

3. **Infrastructure Configuration Files**
   - ENC cluster orchestration
   - BPI consensus parameters
   - BPCI server settings

---

## 📁 Project Structure with CUE Integration

```
metanode/
├── rust/                           # Core Rust implementation
│   ├── crates/                     # Existing 33 crates (to be consolidated)
│   └── ...
├── agreements/                     # NEW: CUE-based agreements
│   ├── medical-data.cue           # Example: Medical data processing agreement
│   ├── financial-audit.cue        # Example: Financial audit agreement
│   └── supply-chain.cue           # Example: Supply chain agreement
├── docklock-specs/                # NEW: CUE-based container specs
│   ├── webapp-deployment.cue      # Web application containers
│   ├── database-cluster.cue       # Database cluster containers
│   └── microservice-mesh.cue      # Microservice deployment
├── court-contracts/               # NEW: CUE-based YAML smart contracts
│   ├── dispute-resolution.cue     # Dispute handling logic
│   ├── sla-enforcement.cue        # SLA violation handling
│   └── payment-settlement.cue     # Payment and settlement logic
└── generated/                     # Auto-generated from CUE (git-ignored)
    ├── solidity/                  # Generated Solidity contracts
    ├── yaml/                      # Generated YAML configs
    └── toml/                      # Generated TOML configs
```

---

## 🔧 How Developers Will Use CUE

### 1. Writing Agreement Smart Contracts

**Developer writes:** `agreements/medical-data.cue`
```cue
package agreements

medical_agreement: {
    id: "medical-data-processing-2025"
    parties: [
        {role: "hospital", stake: 1000},
        {role: "ai_vendor", stake: 500},
        {role: "validator", stake: 200}
    ]
    
    sla: {
        max_processing_time: "30s"
        uptime_requirement: "99.9%"
        data_privacy: "HIPAA_compliant"
    }
    
    penalties: {
        sla_breach: "slash_10_percent"
        data_leak: "slash_100_percent"
        downtime: "linear_penalty"
    }
}
```

**Auto-generates:** 
- `generated/yaml/medical-data-contract.yaml` (Court Node)
- `generated/solidity/MedicalDataAgreement.sol` (Blockchain)

### 2. Writing DockLock Container Specs

**Developer writes:** `docklock-specs/webapp-deployment.cue`
```cue
package docklock

webapp_deployment: {
    containers: [
        {
            name: "frontend"
            image: "nginx:alpine"
            cpu: "500m"
            memory: "512Mi"
            ports: [80, 443]
            security: "restricted"
        },
        {
            name: "backend"
            image: "node:18-alpine"
            cpu: "1000m"
            memory: "1Gi"
            ports: [3000]
            env: {
                NODE_ENV: "production"
                DB_HOST: "postgres-service"
            }
        }
    ]
    
    network: {
        type: "bridge"
        isolation: "strict"
    }
}
```

**Auto-generates:**
- `generated/yaml/webapp-docklock.yaml` (DockLock deployment)
- `generated/yaml/webapp-enc-cluster.yaml` (ENC orchestration)

### 3. Writing Court Node YAML Smart Contracts

**Developer writes:** `court-contracts/dispute-resolution.cue`
```cue
package court

dispute_contract: {
    name: "DisputeResolution"
    version: "1.0"
    
    triggers: [
        {
            event: "sla_violation"
            condition: "response_time > agreement.sla.max_processing_time"
            action: "initiate_dispute"
        },
        {
            event: "data_breach"
            condition: "privacy_violation_detected"
            action: "emergency_halt"
        }
    ]
    
    resolution_process: {
        steps: [
            "evidence_collection",
            "validator_review",
            "automated_decision",
            "penalty_execution"
        ]
        timeout: "24h"
        appeals_allowed: 1
    }
}
```

**Auto-generates:**
- `generated/yaml/dispute-resolution.yaml` (Court Node YAML contract)
- `generated/rust/dispute_handlers.rs` (Rust integration code)

---

## 🚀 Developer Workflow

### Step 1: Developer Creates CUE Files
```bash
# Developer creates agreement
nano agreements/my-project.cue

# Developer creates container specs  
nano docklock-specs/my-containers.cue

# Developer creates smart contracts
nano court-contracts/my-logic.cue
```

### Step 2: Auto-Generation
```bash
# Single command generates everything
make generate-all

# Or generate specific components
make generate-agreements
make generate-docklock
make generate-court-contracts
```

### Step 3: Deploy with Generated Files
```bash
# Deploy using generated configurations
metanode deploy generated/yaml/my-project.yaml
```

---

## 📊 Size Impact on 150MB Installer

### What CUE Replaces
- ❌ **Dashboard bloat:** 2.2GB → 0MB (eliminated)
- ❌ **Manual config files:** 33+ files → 1 CUE spec per project
- ❌ **Duplicate YAML/TOML/JSON:** Multiple formats → Single CUE source

### What CUE Adds
- ✅ **CUE runtime:** ~2MB
- ✅ **Schema definitions:** ~1MB
- ✅ **Generation tools:** ~2MB
- ✅ **Total CUE system:** ~5MB

### Net Impact
- **Before:** 2.2GB+ bloat
- **After:** 5MB CUE system
- **Reduction:** 99.8% size reduction
- **150MB installer:** Now easily achievable ✅

---

## 🎯 Benefits for Developers

### Single Source of Truth
- Write once in CUE
- Generate all required formats automatically
- No manual config maintenance

### Type Safety
- CUE validates all configurations
- Catch errors before deployment
- Consistent structure across all components

### Simplified Deployment
- One command generates everything
- No need to learn multiple config formats
- Automatic integration with all Metanode components

### Maintainability
- Change CUE spec → all configs update
- Version control on single files
- Easy to review and audit

This CUE integration specifically targets the developer experience in Metanode while solving our 150MB installer size constraint by eliminating massive configuration bloat.
