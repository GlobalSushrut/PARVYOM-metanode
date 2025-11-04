# BPI (Blockchain Protocol Infrastructure) Installer Suite

## Overview

The BPI Installer Suite provides multiple installation methods for the BPI infrastructure, making it as easy to install and use as Docker or Minikube. The suite includes platform-specific installers, a universal package manager, and comprehensive tooling for development and deployment.

## 🚀 Quick Start

### One-Command Installation

```bash
# Linux/macOS - Bash installer
curl -fsSL https://get.bpi.pravyom.com | bash

# Alternative with wget
wget -qO- https://get.bpi.pravyom.com | bash

# Cross-platform Python installer
curl -fsSL https://get.bpi.pravyom.com/install.py | python3

# Windows PowerShell
iwr -useb https://get.bpi.pravyom.com/install.ps1 | iex

# Universal package manager
curl -fsSL https://get.bpi.pravyom.com/bpi-get | bash
```

### Manual Installation

```bash
# Clone and run installer
git clone https://github.com/GlobalSushrut/PARVYOM-metanode.git
cd PARVYOM-metanode

# Choose your installer
./install-bpi.sh          # Linux/macOS
python3 install-bpi.py    # Cross-platform
# install-bpi.ps1         # Windows PowerShell
```

## 📦 Installation Methods

### 1. Bash Installer (`install-bpi.sh`)
- **Platform**: Linux, macOS, WSL
- **Features**: 
  - System compatibility checks
  - Automatic Rust installation
  - Full BPI infrastructure build
  - Desktop integration (Linux)
  - CLI tool creation
  - Health checks and validation

### 2. PowerShell Installer (`install-bpi.ps1`)
- **Platform**: Windows 10/11, Windows Server
- **Features**:
  - Chocolatey package manager integration
  - Visual Studio Build Tools installation
  - Windows service integration
  - Start Menu shortcuts
  - PATH configuration

### 3. Python Installer (`install-bpi.py`)
- **Platform**: Cross-platform (Linux, macOS, Windows)
- **Features**:
  - Universal compatibility
  - Advanced error handling
  - Configurable installation paths
  - Comprehensive logging
  - Platform-specific optimizations

### 4. BPI-GET Package Manager (`bpi-get.sh`)
- **Platform**: Universal
- **Features**:
  - Docker/npm-like package management
  - Environment management
  - Service orchestration
  - Development tools
  - Registry integration

## 🛠 Installation Components

### Core Infrastructure
- **BPI Core**: 6D quantum-topological consensus engine
- **BPCI Enterprise**: Governance and enterprise features
- **Wallet Identity**: DID-based identity and wallet system
- **BPI VM**: Ultra-lightweight virtual machine
- **Orchestrator**: Service coordination and management
- **Analytics**: Monitoring and metrics collection

### Development Tools
- **CLI Tools**: Command-line interface for all operations
- **Configuration**: TOML-based configuration system
- **Logging**: Structured logging and audit trails
- **Health Checks**: System diagnostics and validation

## 📋 System Requirements

### Minimum Requirements
- **OS**: Linux (Ubuntu 18.04+), macOS (10.15+), Windows (10+)
- **RAM**: 4GB minimum, 8GB recommended
- **Storage**: 10GB free space
- **Network**: Internet connection for installation

### Dependencies
- **Git**: Version control (auto-installed if missing)
- **Rust**: Toolchain (auto-installed via rustup)
- **Build Tools**: Platform-specific compilers

### Supported Architectures
- **x86_64** (Intel/AMD 64-bit)
- **ARM64** (Apple Silicon, ARM64 servers)
- **ARMv7** (Raspberry Pi, embedded systems)

## 🎯 Installation Options

### Standard Installation
```bash
# Full BPI infrastructure
./install-bpi.sh

# Custom installation directory
./install-bpi.sh --install-dir /opt/bpi

# Development mode with debug symbols
./install-bpi.sh --dev-mode
```

### Package-Specific Installation
```bash
# Install only core components
bpi-get install bpi-core

# Install enterprise features
bpi-get install bpci-enterprise

# Install wallet system
bpi-get install bpi-wallet

# Install analytics
bpi-get install bpi-analytics
```

### Environment Management
```bash
# Create development environment
bpi-get env create development

# Create production environment
bpi-get env create production

# Switch environments
bpi-get env switch testnet
```

## 🚀 Post-Installation Usage

### Starting BPI Infrastructure
```bash
# Start all services
bpi start

# Start specific service
bpi start bpi-core

# Check status
bpi status

# View logs
bpi logs
```

### Web Interfaces
- **BPCI Enterprise Dashboard**: http://localhost:8080
- **BPI Core API**: http://localhost:7777
- **Wallet Interface**: http://localhost:7778
- **Analytics Dashboard**: http://localhost:9090

### CLI Commands
```bash
# System management
bpi start                 # Start infrastructure
bpi stop                  # Stop infrastructure
bpi restart               # Restart infrastructure
bpi status                # Show status
bpi logs [service]        # View logs
bpi config                # Manage configuration

# Development
bpi init my-dapp          # Create new dApp
bpi build                 # Build project
bpi test                  # Run tests
bpi deploy                # Deploy to network

# Package management (with bpi-get)
bpi-get search <query>    # Search packages
bpi-get install <pkg>     # Install package
bpi-get update <pkg>      # Update package
bpi-get list              # List installed packages
```

## 🔧 Configuration

### Default Configuration Location
- **Linux/macOS**: `~/.bpi/config/bpi.toml`
- **Windows**: `%USERPROFILE%\.bpi\config\bpi.toml`

### Key Configuration Sections
```toml
[bpi_core]
host = "127.0.0.1"
port = 7777
data_dir = "~/.bpi/data"
log_level = "info"

[consensus]
algorithm = "6d_quantum_topological"
quantum_entanglement = true
knot_theory_validation = true

[networking]
httpcg_enabled = true
xtmp_enabled = true
shadow_registry = true

[security]
post_quantum_crypto = true
audit_enabled = true
forensic_mode = true
```

## 🏥 Health Checks and Diagnostics

### System Health Check
```bash
# Run comprehensive diagnostics
bpi doctor

# Check specific component
bpi health bpi-core

# Validate configuration
bpi config validate

# Test network connectivity
bpi network test
```

### Common Issues and Solutions

#### Installation Issues
```bash
# Permission errors
sudo chown -R $USER ~/.bpi

# Missing dependencies
bpi-get doctor

# Build failures
export RUST_BACKTRACE=1
cargo clean && cargo build --release
```

#### Runtime Issues
```bash
# Port conflicts
bpi config set bpi_core.port 7778

# Memory issues
bpi config set bpi_core.memory_limit 4GB

# Network issues
bpi network diagnose
```

## 🔄 Updates and Maintenance

### Updating BPI
```bash
# Update all components
bpi-get upgrade

# Update specific component
bpi-get update bpi-core

# Check for updates
bpi-get list --outdated
```

### Maintenance Tasks
```bash
# Clean cache and logs
bpi clean

# Backup configuration
bpi backup create

# Restore from backup
bpi backup restore <backup-id>

# Reset to defaults
bpi reset --confirm
```

## 🌐 Network and Registry

### Registry Management
```bash
# List available registries
bpi-get registry list

# Add custom registry
bpi-get registry add https://my-registry.com

# Login to registry
bpi-get login

# Publish package
bpi-get publish
```

### Network Configuration
```bash
# Join testnet
bpi network join testnet

# Join mainnet
bpi network join mainnet

# Create private network
bpi network create my-network
```

## 🛡 Security Features

### Post-Quantum Cryptography
- **Encryption**: Lattice-based cryptography
- **Signatures**: Hash-based signatures
- **Key Exchange**: CRYSTALS-Kyber

### Audit and Forensics
- **Immutable Logs**: All operations logged
- **Forensic Mode**: Detailed audit trails
- **Compliance**: Government-grade compliance

### Identity and Access
- **DID-based Identity**: Decentralized identifiers
- **Multi-signature**: Enterprise-grade security
- **Role-based Access**: Granular permissions

## 📚 Documentation and Support

### Documentation
- **Official Docs**: https://globalsushrut.github.io/PARVYOM-metanode/
- **API Reference**: https://api.bpi.pravyom.com/docs
- **Tutorials**: https://learn.bpi.pravyom.com

### Community and Support
- **GitHub Issues**: https://github.com/GlobalSushrut/PARVYOM-metanode/issues
- **Discord**: https://discord.gg/bpi-community
- **Forum**: https://forum.bpi.pravyom.com

### Enterprise Support
- **Enterprise Docs**: https://enterprise.bpi.pravyom.com
- **Support Portal**: https://support.bpi.pravyom.com
- **Professional Services**: https://services.bpi.pravyom.com

## 🚀 Advanced Features

### Container Integration
```bash
# Docker-like container management
bpi container run my-app
bpi container list
bpi container logs my-app

# Kubernetes integration
bpi k8s deploy my-app.yaml
bpi k8s scale my-app --replicas=3
```

### Cloud Deployment
```bash
# Deploy to cloud providers
bpi cloud deploy aws
bpi cloud deploy gcp
bpi cloud deploy azure

# Multi-cloud orchestration
bpi cloud orchestrate --providers=aws,gcp
```

### Enterprise Features
```bash
# Enterprise cluster management
bpi cluster create production
bpi cluster join worker-node-1
bpi cluster scale --nodes=10

# Governance and compliance
bpi governance propose upgrade-v2
bpi compliance audit --standard=soc2
```

## 📊 Monitoring and Analytics

### Built-in Monitoring
- **Metrics Collection**: Prometheus-compatible
- **Log Aggregation**: Structured JSON logs
- **Health Monitoring**: Real-time health checks
- **Performance Metrics**: Consensus and VM metrics

### Integration
```bash
# Prometheus integration
bpi metrics export prometheus

# Grafana dashboards
bpi dashboard install grafana

# Custom monitoring
bpi monitor add custom-metric
```

## 🔮 Future Roadmap

### Planned Features
- **GUI Installer**: Graphical installation interface
- **Mobile Support**: iOS and Android installers
- **IoT Integration**: Embedded device support
- **AI/ML Tools**: Machine learning integration

### Version Compatibility
- **Semantic Versioning**: Major.Minor.Patch
- **Backward Compatibility**: API versioning
- **Migration Tools**: Automated upgrades

## 📄 License and Legal

### Open Source License
- **License**: MIT License
- **Commercial Use**: Permitted
- **Modification**: Permitted
- **Distribution**: Permitted

### Enterprise License
- **Commercial Support**: Available
- **SLA Options**: 99.9% uptime guarantee
- **Custom Features**: Enterprise-specific development

---

## Quick Reference Card

```bash
# Installation
curl -fsSL https://get.bpi.pravyom.com | bash

# Basic Operations
bpi start                 # Start infrastructure
bpi status                # Check status
bpi logs                  # View logs
bpi stop                  # Stop infrastructure

# Package Management
bpi-get install <pkg>     # Install package
bpi-get list              # List packages
bpi-get update <pkg>      # Update package
bpi-get search <query>    # Search packages

# Development
bpi init my-project       # New project
bpi build                 # Build project
bpi test                  # Run tests
bpi deploy                # Deploy project

# Maintenance
bpi doctor                # System diagnostics
bpi clean                 # Clean cache
bpi backup create         # Create backup
bpi-get upgrade           # Update all
```

For more information, visit: https://globalsushrut.github.io/PARVYOM-metanode/
