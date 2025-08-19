# BPCI Server Installer - OWNER ONLY

## ⚠️ RESTRICTED ACCESS
This directory contains the **BPCI Server Installer** - for **ecosystem owner only**.

### 🔒 Single Deployment Policy
- **Only ONE server** can be deployed per ecosystem
- **Owner-only access** - not for general distribution
- **Single-use installer** - prevents duplicate deployments

### 📁 Files
- `bpci-server-installer.sh` - Main server installer (owner only)
- `server-management.sh` - Server operations management
- `server-config.toml` - Server configuration template

### 🚀 Server Deployment (Owner Only)
```bash
# Deploy the main BPCI server (once only)
sudo ./bpci-server-installer.sh

# Manage server operations
sudo ./server-management.sh
```

### 🌐 After Server Deployment
Once your server is live:
- **Community users:** Use community installer to connect
- **Enterprise users:** Use enterprise installer to connect
- **Developers:** Use dev installer for local testing
- **All connect to YOUR server** as the central hub

### 🔐 Security Features
- **Single-deployment protection:** Prevents multiple server installations
- **Owner verification:** Restricted access controls
- **Ecosystem coordination:** Central bridge for all node types

**Note:** Keep this directory private and secure!
