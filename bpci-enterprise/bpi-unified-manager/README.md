# 🎯 BPI Unified Manager - Tauri Desktop App

**Manage all 32 components (9 BPCI + 23 BPI OS) from one unified interface**

---

## 🚀 Quick Start

### Prerequisites
- Node.js 18+ and npm
- Rust 1.70+
- Tauri CLI

### Installation

```bash
# Navigate to the project directory
cd bpi-unified-manager

# Install npm dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

---

## 📱 Features

### 6 Complete Pages:

1. **Dashboard Overview** - Grid view of all 32 components with status indicators
2. **Component Control Panel** - Start/stop/restart individual components
3. **Real-Time Monitoring** - Live CPU, RAM, and network metrics
4. **Configuration Manager** - Edit env.ini and component configs
5. **Log Viewer** - Real-time log streaming from all components
6. **Health Dashboard** - Health status and alerts for all components

---

## 🏗️ Architecture

### Backend (Rust - Tauri)
- `src-tauri/src/main.rs` - Tauri backend with all commands
- Integrates with `UnifiedComponentManager` from main codebase
- Real-time component management via systemd

### Frontend (React + TypeScript)
- `src/App.tsx` - Main application component
- `src/pages/` - 6 page components
- Real-time updates via Tauri commands
- Responsive UI with modern design

---

## 🔧 Tauri Commands

All commands are defined in `src-tauri/src/main.rs`:

- `start_all_components()` - Start all 32 components
- `stop_all_components()` - Stop all 32 components
- `restart_component(id)` - Restart specific component
- `get_all_component_status()` - Get status of all components
- `get_component_logs(id, lines)` - Get logs from component
- `get_component_metrics(id)` - Get metrics for component
- `start_component(id)` - Start specific component
- `stop_component(id)` - Stop specific component

---

## 📊 Component Categories

1. **BPCI Core** (9 components)
   - Consensus, Blockchain, Auction, BSO-K8, Bridge, Cluster Ledger, XTMP, Shadow Registry, Web

2. **BPI OS Core** (7 components)
   - BPI VM Server, HTTP Cage, Shadow Registry, ZKLock Mobile, ENC Cluster, DockLock, Oracle Nodes

3. **vPod Infrastructure** (5 components)
   - vPod Coordinator, Scheduler, Arena Manager, SPSC Ring Buffer, Epoch Scheduler

4. **Network & Security** (5 components)
   - eBPF/XDP Trust Routing, QLock Session Steering, Forensic Firewall, P2P Mesh, HERMES-Lite

5. **Economy & Governance** (3 components)
   - 4-Coin Economy Engine, Treasury Distribution, Governance Engine

6. **Storage & Data** (3 components)
   - LCCD State Manager, Merkle Tree Storage, Audit Trail System

---

## 🎨 UI Pages

### 1. Dashboard Overview
- Grid view of all 32 components organized by category
- Real-time status indicators (Running/Stopped/Error)
- System-wide statistics (CPU, Memory, Network)
- Quick actions (Start All, Stop All, Refresh)

### 2. Component Control Panel
- Detailed table view of all components
- Individual start/stop/restart controls
- Filter by status and search by name
- Port and endpoint information
- Health status indicators

### 3. Real-Time Monitoring
- Live metrics for selected component
- CPU usage graph
- Memory usage tracking
- Network traffic monitoring
- 1-second refresh rate

### 4. Configuration Manager
- Edit env.ini and component-specific configs
- Syntax highlighting
- Validation before saving
- Support for all configuration files

### 5. Log Viewer
- Real-time log streaming
- Filter by component
- Adjustable line count (50/100/500/1000)
- Auto-refresh toggle
- Search functionality

### 6. Health Dashboard
- Health status overview (Healthy/Degraded/Unhealthy/Unknown)
- Component health list with uptime
- Error message display
- Auto-refresh every 5 seconds

---

## 🔐 Security

- All component operations require proper permissions
- Systemd integration for secure service management
- File system access restricted to `/etc/bpci/` and `/var/log/bpci/`
- HTTP requests scoped to localhost and known endpoints

---

## 📦 Build Output

Production builds will be created in:
- Linux: `src-tauri/target/release/bundle/appimage/`
- macOS: `src-tauri/target/release/bundle/dmg/`
- Windows: `src-tauri/target/release/bundle/msi/`

---

## 🎯 Next Steps

1. Run `npm install` to install dependencies
2. Run `npm run tauri dev` to test the app
3. Verify all 32 components are detected
4. Test start/stop/restart functionality
5. Build for production with `npm run tauri build`

---

## 📚 Documentation

- Tauri Docs: https://tauri.app/
- React Docs: https://react.dev/
- TypeScript Docs: https://www.typescriptlang.org/

---

**Built with ❤️ for the BPI/BPCI Enterprise Platform**
