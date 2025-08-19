# Metanode CUE Integration Plan
## Single Source of Truth for 150MB Installer

### 🎯 Core Objective
Replace the 2.2GB dashboard bloat and 33-crate complexity with a **CUE-first single source of truth** that auto-generates all artifacts needed for Metanode's core functionality.

---

## 📋 Metanode-Specific Requirements

### What We Actually Need (Not Generic)
```
Metanode Core Components:
├── DockLock Containers (Docker alternative)
├── ENC Cluster Orchestration (K8s alternative) 
├── BPI Consensus Layer (blockchain)
├── BPCI Enterprise Server (economy + banking)
├── Court Node (YAML SmartContracts++)
├── Bank Mesh (autonomous economy)
├── Relay Storage (10x IPFS performance)
└── CLI + Minimal Dashboard
```

### What We DON'T Need
- ❌ Generic Kubernetes manifests
- ❌ Generic Docker Compose files  
- ❌ Generic Solidity contracts
- ❌ Multiple dashboard frameworks
- ❌ External dependencies
- ❌ Over-engineered abstractions

---

## 🔧 Metanode CUE Structure

### Optimized Directory Structure
```
metanode-spec/
├── cue.mod/module.cue
├── schema/
│   └── metanode.cue          # Core Metanode types only
├── agreements/
│   └── sample.cue            # Single agreement example
└── tools/                    # Metanode-specific generators
    ├── docklock.cue          # → DockLock container specs
    ├── enc_cluster.cue       # → ENC orchestration config
    ├── bpi_config.cue        # → BPI consensus settings
    ├── bpci_config.cue       # → BPCI server config
    ├── court_node.cue        # → YAML smart contracts
    ├── bank_mesh.cue         # → Economic configuration
    ├── relay_config.cue      # → Storage layer settings
    └── cli_config.cue        # → CLI command generation
```

---

## 📊 Size Budget Allocation

### CUE Integration Size Impact
```
BEFORE CUE (Current):
├── Dashboards: 2.2GB (BLOAT)
├── 33 Rust crates: ~80MB
├── Config files: ~20MB
├── Dependencies: ~50MB
└── TOTAL: 2.35GB+ (MASSIVE OVERAGE)

AFTER CUE (Target):
├── Single CUE spec: 1MB
├── Generated configs: 5MB
├── 8 consolidated crates: 40MB
├── Embedded dashboard: 10MB
├── Dependencies: 30MB
└── TOTAL: 86MB (UNDER BUDGET!)
```

---

## 🎯 Metanode-Specific CUE Schema

### Core Types (Minimal)
```cue
package metanode

// Only what Metanode actually uses
#MetanodeAgreement: {
    id: string & !=""
    parties: [...#Party] & minItems(2)
    
    // DockLock configuration
    containers: [...#Container]
    
    // ENC Cluster settings  
    orchestration: #EncConfig
    
    // BPI consensus
    consensus: #BpiConfig
    
    // BPCI economy
    economy: #BpciConfig
    
    // Court Node (YAML contracts)
    court: #CourtConfig
    
    // Bank Mesh (autonomous economy)
    banking: #BankConfig
    
    // Relay storage
    storage: #RelayConfig
}
```

---

## 🔄 Generation Pipeline

### Metanode-Specific Generators
```bash
# Single command generates everything
make metanode-gen

# Outputs:
├── docklock/containers.yaml     # DockLock specs
├── enc/cluster-config.yaml      # ENC orchestration
├── bpi/consensus.toml          # BPI settings
├── bpci/server.toml            # BPCI config
├── court/contracts.yaml        # YAML smart contracts
├── bank/mesh-config.toml       # Banking setup
├── relay/storage.toml          # Storage config
└── cli/commands.rs             # CLI generation
```

### Integration with Existing Codebase
```rust
// Generated configs integrate directly
use metanode_generated::*;

fn main() {
    let config = load_generated_config();
    let docklock = DockLock::from_config(&config.docklock);
    let enc_cluster = EncCluster::from_config(&config.enc);
    let bpi = Bpi::from_config(&config.bpi);
    // etc.
}
```

---

## 📋 Implementation Steps

### Phase 1: Core Schema (Day 1)
- [ ] Create minimal Metanode CUE schema
- [ ] Define only essential types (no over-engineering)
- [ ] Focus on DockLock, ENC, BPI, BPCI integration

### Phase 2: Essential Generators (Day 2)
- [ ] DockLock container generator
- [ ] ENC cluster config generator  
- [ ] BPI consensus generator
- [ ] BPCI server generator

### Phase 3: Advanced Features (Day 3)
- [ ] Court Node YAML contract generator
- [ ] Bank Mesh economic config generator
- [ ] Relay storage config generator
- [ ] CLI command generator

### Phase 4: Integration & Testing (Day 4)
- [ ] Integrate with existing Rust codebase
- [ ] Replace current config files with generated ones
- [ ] Test end-to-end pipeline
- [ ] Validate size reduction (2.2GB → <10MB)

---

## 🎯 Success Metrics

### Size Reduction
- **Dashboard bloat:** 2.2GB → 0MB (eliminated)
- **Config complexity:** 33 files → 1 CUE spec
- **Total size impact:** -2.15GB reduction
- **Final installer:** <150MB ✅

### Functionality Preservation
- ✅ All DockLock container functionality
- ✅ All ENC cluster orchestration
- ✅ All BPI consensus features
- ✅ All BPCI server capabilities
- ✅ Court Node YAML contracts
- ✅ Bank Mesh autonomous economy
- ✅ Relay storage performance
- ✅ CLI command completeness

### Development Experience
- ✅ Single file to edit (CUE spec)
- ✅ Auto-generation of all configs
- ✅ Type safety and validation
- ✅ Zero manual config maintenance
- ✅ Instant deployment pipeline

---

## 🚀 Expected Impact

### Before CUE Integration
```
Developer Experience:
- Edit 33+ config files manually
- Maintain dashboard bloat (2.2GB)
- Debug config inconsistencies
- Manual deployment steps
- Size: 2.35GB+ (OVER BUDGET)
```

### After CUE Integration  
```
Developer Experience:
- Edit 1 CUE file
- Auto-generate all configs
- Type-safe validation
- One-command deployment
- Size: <86MB (UNDER BUDGET)
```

This CUE integration plan will eliminate the massive bloat discovered in our analysis while preserving all Metanode functionality in a compact, maintainable form.
