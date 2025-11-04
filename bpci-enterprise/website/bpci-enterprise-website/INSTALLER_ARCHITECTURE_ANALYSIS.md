# 🏗️ BPCI Installer Architecture - Real Backend Analysis

## 🎯 What We Currently Have (WRONG)

### Current UI (`BPIInstaller.tsx`):
- ❌ Tries to install nodes via GitHub repos
- ❌ Uses fake `/api/installer/*` endpoints that don't exist
- ❌ Shows progress bars for downloading/installing
- ❌ Completely disconnected from real backend

## ✅ What Actually Exists (REAL BACKEND)

### 1. **Community Installer Web Server** (`community_installer_web.rs`)
**Purpose**: Web interface for BPCI Community OS installation
**Port**: Dynamic (Pure Virtual Addressing Mode - NO static ports!)
**Key Features**:
- User authentication and session management
- BPI wallet creation and activation
- Community OS installation orchestration
- Real BPI ledger integration
- CommuteLock runtime for secure communication

**Real API Endpoints**:
```rust
POST /api/auth/register       // Register new user
POST /api/auth/login          // Login user
POST /api/auth/logout         // Logout user
GET  /api/auth/verify         // Verify session

POST /api/wallet/create       // Create BPI wallet
GET  /api/wallet/list         // List user's wallets
GET  /api/wallet/:id          // Get specific wallet
POST /api/wallet/:id/activate // Activate wallet
GET  /api/wallet/:id/balance  // Get wallet balance

GET  /api/status              // Installation status
POST /api/install             // Start installation
GET  /api/config              // Get configuration
POST /api/config              // Update configuration
GET  /api/logs                // Get installation logs
```

### 2. **Unified Community Installer** (`unified_community_installer.rs`)
**Purpose**: CLI-based one-click installer for mainnet deployment
**Installation Types**:
1. **Community Node** - Mining + Auctions
2. **Roundtable Partner** - Governance + Revenue Sharing
3. **Enterprise Node** - All features

**Real Installation Flow**:
```
1. System Requirements Check
   - CPU, RAM, Disk space
   - Network connectivity
   - File system permissions

2. Service Installation
   - BPCI Core services
   - Mining components (if enabled)
   - Auction mempool (if enabled)
   - Roundtable oracle (for partners)

3. Configuration
   - Node type selection
   - Mining/auction settings
   - Network configuration
   - Partner chain integration (for roundtable)

4. Verification
   - Service health checks
   - Network connectivity tests
   - File system validation
```

### 3. **Mojo Wallet System** (`bpci_mojo_server.rs`)
**Purpose**: Mainnet monitoring activation (NOT node installation!)
**Port**: 8089
**What It Actually Does**:
- Creates Grafana dashboard for BPI node
- Sets up Prometheus monitoring job
- Generates token-based authentication
- Provides isolated monitoring per wallet

**Real API**:
```rust
POST /api/v1/wallet           // Create Mojo wallet (activate monitoring)
GET  /api/v1/wallets          // List all Mojo wallets
GET  /api/v1/health           // Health check
```

**Request/Response**:
```json
// Request
{
  "bpi_wallet_address": "0x1234...",
  "node_id": "my-node"
}

// Response
{
  "success": true,
  "mojo_wallet_id": "uuid",
  "dashboard_url": "http://localhost:3000/d/uuid?auth_token=xxx",
  "access_token": "grafana-token"
}
```

### 4. **Mother Coin System** (Economic Integration)
**Purpose**: Community installer registration and rewards
**Key Concepts**:
- Community installers register with 5 node IDs
- Receive 25k GEN @ $10/coin allocation
- Part of the 4-coin autonomous economy (GEN/NEX/FLX/AUR)

## 🔄 Actual Installation Flow (How It Really Works)

### Step 1: User Registration & Authentication
```
User → Web UI → POST /api/auth/register
                → Session created
                → User stored in memory
```

### Step 2: BPI Wallet Creation
```
User → Dashboard → POST /api/wallet/create
                  → Ed25519 keypair generated
                  → Wallet stored with encrypted private key
                  → BPI address generated
```

### Step 3: Wallet Activation (Optional - For Mainnet)
```
User → Dashboard → POST /api/wallet/:id/activate
                  → Transaction sent to BPI ledger
                  → Wallet activated on blockchain
                  → Activation TX hash returned
```

### Step 4: Community OS Installation
```
User → Dashboard → POST /api/install
                  → CommunityInstallerOS initialized
                  → System requirements checked
                  → Services installed (mining, auctions, etc.)
                  → Configuration applied
                  → Verification performed
```

### Step 5: Mojo Wallet Activation (For Mainnet Monitoring)
```
User → Dashboard → POST http://localhost:8089/api/v1/wallet
                  → Grafana dashboard created
                  → Prometheus job configured
                  → Access token generated
                  → Monitoring activated
```

## 🎨 What the UI Should Actually Show

### 1. **Installation Wizard** (Multi-Step)
**Step 1: Account Setup**
- Email/password registration
- Session management
- User profile

**Step 2: Wallet Creation**
- Generate BPI wallet
- Save encrypted private key
- Display wallet address

**Step 3: Node Type Selection**
- Community Node (mining + auctions)
- Roundtable Partner (governance + revenue)
- Enterprise Node (all features)

**Step 4: Configuration**
- Mining settings (if applicable)
- Auction participation
- Network configuration
- Partner chain details (for roundtable)

**Step 5: Installation**
- Real-time progress from `/api/status`
- Live logs from `/api/logs`
- Service health checks
- Verification results

**Step 6: Mojo Wallet Activation** (Optional - Mainnet Only)
- Enter BPI wallet address
- Activate monitoring
- Receive Grafana dashboard URL
- Get access token

### 2. **Dashboard Features**
- Installation status monitoring
- Wallet management
- Service health checks
- Logs viewer
- Configuration editor
- Mojo wallet dashboard access

## 🚀 Correct Implementation Approach

### Phase 1: Fix Authentication Flow
- Integrate with real `/api/auth/*` endpoints
- Implement session management
- Add user profile management

### Phase 2: Fix Wallet Management
- Use real `/api/wallet/*` endpoints
- Show wallet creation flow
- Display wallet balance from BPI ledger
- Add wallet activation for mainnet

### Phase 3: Fix Installation Flow
- Remove fake GitHub-based installer
- Use real `/api/install` endpoint
- Show real-time status from `/api/status`
- Display live logs from `/api/logs`
- Add configuration management

### Phase 4: Add Mojo Wallet Integration
- Separate component for mainnet monitoring
- Connect to Mojo server (port 8089)
- Show Grafana dashboard integration
- Manage access tokens

## 📊 Key Differences: Current vs Real

| Feature | Current (Wrong) | Real Backend |
|---------|----------------|--------------|
| Installation Method | GitHub clone | System service installation |
| API Endpoints | `/api/installer/*` (fake) | `/api/install`, `/api/status`, etc. |
| Authentication | None | Full user auth with sessions |
| Wallet Integration | Mock registry service | Real BPI ledger client |
| Monitoring | None | Mojo wallet + Grafana + Prometheus |
| Node Types | Generic "node" | Community/Roundtable/Enterprise |
| Configuration | Simple form | Full CommunityInstallerOS config |
| Progress Tracking | Fake progress bar | Real status from backend |

## 🎯 Next Steps

1. **Analyze** the complete `community_installer_web.rs` API
2. **Map** all real endpoints to UI components
3. **Redesign** the installer flow based on real backend
4. **Integrate** with actual authentication system
5. **Connect** to real BPI ledger for wallet operations
6. **Add** Mojo wallet activation for mainnet
7. **Test** with real backend running

## 🔑 Critical Understanding

The installer is NOT about:
- ❌ Downloading code from GitHub
- ❌ Installing software packages
- ❌ Running shell scripts

The installer IS about:
- ✅ User registration and authentication
- ✅ BPI wallet creation and management
- ✅ Community OS configuration
- ✅ Service orchestration
- ✅ Mainnet monitoring activation (Mojo wallet)
- ✅ Integration with BPI ledger
- ✅ Real-time status and logging

---

**Conclusion**: The current UI is completely disconnected from the real backend architecture. We need to rebuild it based on the actual API endpoints and flow described above.
