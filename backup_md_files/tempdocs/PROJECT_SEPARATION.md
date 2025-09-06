# 🏗️ Metanode Project Separation Architecture

## 📊 Current Project Structure Analysis

```
metanode/
├── rust/                           # ✅ Core Rust infrastructure (shared)
│   ├── crates/                     # All blockchain components
│   │   ├── bpi-shadow-registry/    # ✅ Web2-Web3 bridge
│   │   ├── bpci/                   # ✅ Network layer
│   │   ├── billing-meter/          # ✅ Economic APIs
│   │   ├── docklock/               # ✅ Container runtime
│   │   └── ...                     # All other 60+ crates
│   └── cli/                        # ✅ CLI tools
├── docs/                          # ✅ Documentation
├── examples/                      # ✅ Examples
└── tests/                         # ✅ Tests
```

## 🎯 Target Separation Architecture

### BPCI Server (Hosted Infrastructure)
```
/server/                           # 🆕 BPCI Server (what you host)
├── Cargo.toml                     # Server-specific dependencies
├── src/
│   ├── main.rs                    # BPCI server entry point
│   ├── api/                       # API gateway and endpoints
│   ├── consensus/                 # IBFT consensus engine
│   ├── nodes/                     # Node management
│   ├── registry/                  # Service registry
│   ├── shadow/                    # Shadow registry integration
│   ├── mining/                    # Mining operations
│   ├── wallet/                    # Wallet services
│   ├── auth/                      # Authentication/authorization
│   ├── monitoring/                # Metrics and alerts
│   └── config/                    # Configuration management
├── tests/                         # Server integration tests
├── docs/                          # Server API documentation
├── deploy/                        # Deployment configurations
│   ├── docker/                    # Docker configurations
│   ├── k8s/                       # Kubernetes manifests
│   └── terraform/                 # Infrastructure as code
└── scripts/                       # Deployment scripts
```

### Installer Package (Developer Tools)
```
/installer/                        # 🆕 Installer (what developers install)
├── Cargo.toml                     # Installer dependencies
├── src/
│   ├── main.rs                    # Installer entry point
│   ├── cli/                       # CLI commands
│   ├── dashboard/                 # Dashboard launcher
│   ├── setup/                     # Setup and configuration
│   ├── connect/                   # BPCI connection management
│   └── templates/                 # Project templates
├── dashboard/                     # BPI Dashboard (web app)
│   ├── package.json               # Dashboard dependencies
│   ├── src/                       # React/TypeScript dashboard
│   ├── public/                    # Static assets
│   └── dist/                      # Built dashboard
├── templates/                     # Project templates
│   ├── dapp/                      # DApp template
│   ├── defi/                      # DeFi template
│   ├── enterprise/                # Enterprise template
│   └── bridge/                    # Bridge template
├── docs/                          # User documentation
└── scripts/                       # Installation scripts
```

### Shared Core (Used by Both)
```
/rust/crates/                      # ✅ Shared core libraries
├── bpi-shadow-registry/           # Web2-Web3 bridge
├── bpci/                          # Network layer
├── billing-meter/                 # Economic APIs
├── docklock/                      # Container runtime
├── bpi-receipts/                  # Receipt system
├── bpi-consensus/                 # Consensus algorithms
├── bpi-headers/                   # Block headers
└── ...                            # All other crates
```
