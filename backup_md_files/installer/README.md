# BPCI Three-Tier Installer System

## Overview
The BPCI installer system provides three deployment tiers for different use cases:

### 🔧 Dev Installer (`pravyom-dev-installer.sh`)
**Purpose:** Lightweight development environment setup
- **Target Users:** Developers, testing, local development
- **Features:**
  - Quick Rust installation
  - Local CLI build and install
  - Basic configuration
  - PATH setup
- **Usage:** `./pravyom-dev-installer.sh`

### 🌐 Community Installer (`bpci-community-installer.sh`)
**Purpose:** Community node deployment with governance
- **Target Users:** Community participants, validators, miners
- **Features:**
  - System user creation
  - Systemd service setup
  - Firewall configuration (UFW)
  - Security hardening (fail2ban)
  - Governance participation
  - Log rotation
- **Usage:** `sudo ./bpci-community-installer.sh`

### 🏦 Enterprise Installer (`enterprise-installer.sh`)
**Purpose:** Banking-grade enterprise deployment
- **Target Users:** Banks, enterprises, regulated institutions
- **Features:**
  - Military-grade security
  - Banking compliance
  - Regulatory reporting
  - High-availability setup
  - Enterprise identity management
  - Audit trails
- **Usage:** `sudo ./enterprise-installer.sh`

### 🌐 Server Installer (`bpci-server-installer.sh`) - **OWNER ONLY**
**Purpose:** Main hosted server/bridge for BPCI ecosystem (**Single deployment only**)
- **Target Users:** **Ecosystem owner only** (not for general use)
- **Features:**
  - Central ecosystem coordination
  - Community/BPI bridge services
  - Registry and identity management
  - API gateway and dashboard
  - Database integration (PostgreSQL/Redis)
  - Single-deployment protection
  - Owner-only access control
- **Usage:** `sudo ./bpci-server-installer.sh` (**Owner only**)

## Universal Installer
Use the universal installer to select your deployment tier:
```bash
./install.sh
```

## Quick Start
1. **For Development:**
   ```bash
   ./pravyom-dev-installer.sh
   ```

2. **For Community Node:**
   ```bash
   sudo ./bpci-community-installer.sh
   ```

3. **For Enterprise:**
   ```bash
   sudo ./enterprise-installer.sh
   ```

4. **For Server (Hosted):**
   ```bash
   sudo ./bpci-server-installer.sh
   ```

## Requirements
- **OS:** Linux (Ubuntu/Debian recommended)
- **Rust:** Automatically installed if missing
- **Git:** Required for repository access
- **Root:** Required for community/enterprise installers

## Post-Installation
After installation, use the CLI commands:
- `pravyom status` - Check system status
- `pravyom registry register-node` - Register with BPCI registry
- `pravyom wallet create` - Create wallet
- `pravyom mining start` - Start mining
- `pravyom governance list-proposals` - View governance proposals

## Support
- **Dev/Community:** GitHub Issues
- **Enterprise:** enterprise@bpci.io
