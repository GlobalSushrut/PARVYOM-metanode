# 🏗️ Metanode Project Structure & Component Separation

## 📊 Current Analysis

Based on the existing codebase structure, here's how we'll organize the BPCI server vs installer package separation:

```
metanode/
├── rust/                           # Core Rust infrastructure
│   ├── crates/                     # All blockchain components
│   │   ├── bpi-shadow-registry/    # ✅ Web2-Web3 bridge
│   │   ├── bpci/                   # ✅ Network layer
│   │   ├── billing-meter/          # ✅ Economic APIs
│   │   ├── docklock/               # ✅ Container runtime
│   │   └── ...                     # All other crates
│   └── cli/                        # CLI tools
├── server/                         # 🆕 BPCI Server (Hosted)
├── installer/                      # 🆕 Installer Package
├── dashboards/                     # 🆕 Web Dashboards
└── docs/                          # Documentation
```

## 🔄 Component Separation Strategy

### BPCI Server (Hosted Infrastructure)
**Location:** `/server/`
**Components:**
- Network consensus nodes (IBFT validators)
- Shadow registry service endpoints
- Economic API gateway
- Mining pool coordination
- Registry & discovery services
- Compliance & policy enforcement

### Installer Package (Developer Tools)
**Location:** `/installer/`
**Components:**
- CLI binary (`metanode` command)
- Local node management
- Dashboard applications
- Wallet applications
- Project templates
- Development tools

### Shared Core Libraries
**Location:** `/rust/crates/`
**Usage:** Both server and installer use these
- All existing BPI/BPCI crates
- Shadow registry
- Economic APIs
- Wallet systems
- Security frameworks

## 🎯 Implementation Priority

### Phase 1: CLI Integration (Days 1-2)
1. Create unified CLI in `/rust/cli/`
2. Integrate all existing crates
3. Implement basic commands:
   - `metanode init`
   - `metanode start`
   - `metanode dashboard`

### Phase 2: Dashboard Development (Days 3-5)
1. Create `/dashboards/` directory
2. Build BPCI dashboard
3. Build BPI dashboard
4. Create MetaNode wallet interface

### Phase 3: Server Architecture (Days 6-8)
1. Create `/server/` directory
2. Implement BPCI server components
3. Set up API gateway
4. Configure hosting infrastructure

### Phase 4: Installer Package (Days 9-10)
1. Create `/installer/` directory
2. Build cross-platform installer
3. Package all components
4. Create one-line install script

## 🚀 Viral-Ready Features

### One-Command Installation
```bash
curl -sSL install.metanode.io | bash
```

### One-Command Project Setup
```bash
metanode init my-project
cd my-project
metanode start
```

### Automatic Dashboard Access
- BPCI Dashboard: http://localhost:3000
- BPI Dashboard: http://localhost:3001
- MetaNode Wallet: http://localhost:3002

### Seamless BPCI Connection
- Auto-detect mainnet/testnet
- Automatic node registration
- Built-in mining setup
- Compliance monitoring

This structure leverages all existing work while creating the clear separation needed for viral adoption.
