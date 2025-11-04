# Component 9: Mojo Wallet — UI/UX Spec (v2)

**Date**: 2025-10-27  
**Version**: 2.0 (Production-Ready)  
**Purpose**: Complete page-by-page wireframe design for Mojo Wallet  
**Features**: Dual authentication (Keycloak + Mojo), Grafana-style Panel Builder, real-time monitoring, scalable for millions of wallets

---

## **🎯 v2 Highlights**

### **Architecture Improvements**:
- ✅ **Left-rail navigation** + context inspector + command palette
- ✅ **L1→L2 dual-auth wizard** (generate → bind → harden) with edge-case handling
- ✅ **Grafana-style Panel Builder (MVP)** with JSON schema persistence
- ✅ **Real-time patterns** (WebSocket/SSE), skeletons, optimistic TX
- ✅ **Virtualized TX table** (50k+ rows), Health Matrix with sparklines
- ✅ **Mojo Super suite** for admin/owner management
- ✅ **API contracts**, Zustand slice map, file scaffolding
- ✅ **Accessibility** (ARIA, keyboard nav) and **performance budgets**

---

## **📊 Current Website State (Analyzed from Code)**

### **✅ What Already Exists**:

1. **Keycloak Integration** ✅
   - `KeycloakAuthContainer.tsx` - SSO authentication
   - `useKeycloak.tsx` - React hooks for Keycloak
   - `keycloakService.ts` - Keycloak service layer
   - Email/password authentication via Keycloak

2. **Basic Dashboard** ✅
   - `Dashboard.tsx` - Main dashboard page
   - Real-time data service
   - BPI connection generation
   - Token management

3. **Existing Pages** ✅
   - Home, About, Technology, Enterprise
   - Community, Blog, Get Started, Contact
   - Privacy Policy, Terms of Service, Legal, Research

4. **Protected Routes** ✅
   - `/dashboard` - Requires authentication
   - `/registry` - Registry dashboard
   - `/wallet` - Wallet manager
   - `/installer` - BPI installer

5. **Components** ✅
   - Auth components (Login, Signup, WalletActivation)
   - Wallet components (WalletManager, BpiWalletSystem)
   - Registry components (RegistryDashboard)

### **⚠️ What Needs to Be Built**:

1. **Dual Authentication Flow** ⚠️
   - Level 1: Email/Password (Keycloak) → Basic Dashboard
   - Level 2: Join Mojo → Create Token → Activate Mojo Wallet

2. **Mojo Wallet Pages** ⚠️
   - Mojo wallet-specific dashboard
   - 8-component integration views
   - Grafana-like monitoring

3. **Mojo Super (Admin Panel)** ⚠️
   - Company/owner admin dashboard
   - Manage all Mojo wallets
   - System-wide monitoring

---

## **🎯 v2 Design Principles**

### **1. Information Architecture**
- **Left-rail navigation** - Persistent, collapsible sidebar
- **Context inspector** - Right panel for detailed views
- **Command palette** - Cmd+K for quick actions
- **Breadcrumbs** - Clear navigation hierarchy

### **2. Dual Authentication Wizard (L1→L2)**
- **Generate** - Create BPI address + token (Component 5)
- **Bind** - Link to Keycloak account
- **Harden** - Security verification + 2FA
- **Edge-case handling** - Token expiry, network errors, retry logic

### **3. Grafana-Style Panel Builder (MVP)**
- **JSON schema** - Persistent panel configurations
- **Widget library** - Charts, gauges, tables, sparklines
- **Drag-and-drop** - Visual panel arrangement
- **Real-time data** - WebSocket/SSE integration

### **4. Real-Time Patterns**
- **WebSocket/SSE** - Live data streaming
- **Skeleton loaders** - Smooth loading states
- **Optimistic updates** - Instant TX feedback
- **Per-widget refresh** - Granular data updates

### **5. Performance & Scalability**
- **Virtualized tables** - Handle 50k+ rows
- **Health Matrix** - Component status with sparklines
- **Code splitting** - Lazy-loaded routes
- **Performance budgets** - <3s initial load, <100ms interactions

### **6. Accessibility & UX**
- **ARIA labels** - Screen reader support
- **Keyboard navigation** - Full keyboard access
- **Focus management** - Clear focus indicators
- **Color contrast** - WCAG AA compliance

---

## **📋 Dual Authentication Flow**

```
User Journey:
├── Level 1: Keycloak Authentication (Email/Password)
│   ├── 1. Login Page (Keycloak SSO)
│   ├── 2. Basic Dashboard (Website features)
│   └── 3. "Join Mojo" Button
│
└── Level 2: Mojo Wallet Activation (Token-based)
    ├── 4. Generate BPI Connection (Address + Token)
    ├── 5. Activate Mojo Wallet
    └── 6. Mojo Wallet Dashboard (Full features)
```

---

## **📋 Page Structure Overview**

```
BPCI Website (After Keycloak Login)
├── 1. Keycloak Login Page (Email/Password)
├── 2. Basic Dashboard (Website features)
│   ├── BPI Connection Generator
│   ├── "Join Mojo" Section
│   └── Basic website features
│
└── Mojo Wallet (After Token Activation)
    ├── 3. Mojo Wallet Dashboard (Overview)
    ├── 4. Wallet Management
    ├── 5. Transactions
    ├── 6. Monitoring & Metrics
    ├── 7. Security & Alerts
    ├── 8. Network & Nodes
    ├── 9. Settings
    └── 10. Admin Panel (Mojo Super)
```

---

## **Page 1: Keycloak Login (Level 1 Auth)**

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────┐
│                    BPCI ENTERPRISE                      │
│              Blockchain Infrastructure                  │
├─────────────────────────────────────────────────────────┤
│                                                          │
│              [BPCI Logo]                                │
│                                                          │
│         ┌────────────────────────────────┐             │
│         │  Email Address                 │             │
│         │  [user@example.com_______]     │             │
│         └────────────────────────────────┘             │
│                                                          │
│         ┌────────────────────────────────┐             │
│         │  Password                      │             │
│         │  [••••••••••••••••••••••]     │             │
│         └────────────────────────────────┘             │
│                                                          │
│         [  Sign In with Keycloak  ]                     │
│                                                          │
│         ─── OR ───                                      │
│                                                          │
│         [  Sign Up  ]                                   │
│                                                          │
│  ─────────────────────────────────────────────────     │
│  🔒 Powered by Keycloak SSO                             │
│  • Enterprise-grade security                            │
│  • SAML, OAuth2 support                                 │
│  • Role-based access control                            │
└─────────────────────────────────────────────────────────┘
```

### **Features**:
- Keycloak SSO authentication
- Email + password login
- Sign up for new users
- Forgot password recovery
- OAuth2/SAML support
- Session management

### **After Login**: Redirect to Basic Dashboard (Level 1 access)

---

## **Page 2: Basic Dashboard (Level 1 - After Keycloak Login)**

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ [☰] BPCI Dashboard    user@example.com    [🔔] [👤] [⚙️]            │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Welcome to BPCI Enterprise!                                        │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 🎯 Join Mojo - Unlock Advanced Features                        ││
│  │                                                                  ││
│  │ Get your BPI wallet address and token to access:                ││
│  │ • Advanced monitoring and metrics                               ││
│  │ • Real-time transaction processing                              ││
│  │ • 8-component BPCI integration                                  ││
│  │ • Grafana-like dashboards                                       ││
│  │                                                                  ││
│  │ [  Generate BPI Connection  ]  [  Learn More  ]                 ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │
│  │ Your Account │  │ Documentation│  │ Support      │             │
│  │              │  │              │  │              │             │
│  │ Email: ✓     │  │ API Docs     │  │ Contact Us   │             │
│  │ Profile: ✓   │  │ Tutorials    │  │ Community    │             │
│  │ Mojo: ✗      │  │ Examples     │  │ FAQ          │             │
│  └──────────────┘  └──────────────┘  └──────────────┘             │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Your BPI Connections                                            ││
│  │                                                                  ││
│  │ No connections yet. Generate your first BPI connection to       ││
│  │ activate Mojo wallet and access advanced features.              ││
│  │                                                                  ││
│  │ [+ Generate BPI Connection]                                     ││
│  └────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────┘
```

### **Features**:
- Welcome message
- "Join Mojo" call-to-action
- Account status (Keycloak authenticated, Mojo not activated)
- BPI connection generator
- Basic website features
- Documentation and support links

### **"Generate BPI Connection" Flow**:
```
User clicks "Generate BPI Connection"
    ↓
Modal opens: "Create BPI Connection"
    ↓
User enters: Connection Name
    ↓
System generates:
    - BPI Address: bpi:wallet:abc123
    - Access Token: encrypted_key_xyz
    - Dashboard URL: /mojo-wallet/bpi:wallet:abc123
    ↓
User receives credentials
    ↓
"Activate Mojo Wallet" button appears
```

---

## **Page 3: BPI Connection Generator Modal**

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────┐
│  Generate BPI OS Connection                        [✕]  │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  Create a secure connection for your BPI OS node        │
│                                                          │
│  Connection Name:                                       │
│  [My BPI Node___________________________]               │
│                                                          │
│  Description (optional):                                │
│  [Production node for testing__________]                │
│                                                          │
│  [Cancel]  [Generate Connection]                        │
│                                                          │
└─────────────────────────────────────────────────────────┘

After Generation:
┌─────────────────────────────────────────────────────────┐
│  BPI Connection Created Successfully!              [✕]  │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ✅ Your BPI connection has been created                │
│                                                          │
│  BPI Address:                                           │
│  bpi:wallet:abc123def456                                │
│  [Copy] [QR Code]                                       │
│                                                          │
│  Access Token:                                          │
│  eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...                │
│  [Copy] [Show/Hide]                                     │
│                                                          │
│  ⚠️ IMPORTANT: Save your access token securely!         │
│  You'll need both the address and token to access       │
│  your Mojo wallet.                                      │
│                                                          │
│  [  Activate Mojo Wallet Now  ]                         │
│  [  I'll Activate Later  ]                              │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### **Features**:
- Connection name input
- Optional description
- BPI address generation (from Component 5)
- Access token generation
- Copy to clipboard
- QR code display
- "Activate Mojo Wallet" button

---

## **Page 4: Mojo Wallet Activation (Level 2 Auth)**

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────┐
│                    MOJO WALLET                          │
│              Secure BPI Wallet Access                   │
├─────────────────────────────────────────────────────────┤
│                                                          │
│              [Mojo Wallet Logo]                         │
│                                                          │
│         ┌────────────────────────────────┐             │
│         │  BPI Wallet Address            │             │
│         │  [bpi:wallet:_____________]    │             │
│         └────────────────────────────────┘             │
│                                                          │
│         ┌────────────────────────────────┐             │
│         │  Access Token                  │             │
│         │  [••••••••••••••••••••••]     │             │
│         └────────────────────────────────┘             │
│                                                          │
│         [  Access Wallet  ]                             │
│                                                          │
│         Don't have a wallet?                            │
│         [Generate New Wallet]                           │
│                                                          │
│  ─────────────────────────────────────────────────     │
│  🔒 Secure Authentication                               │
│  • No passwords required                                │
│  • Token-based access                                   │
│  • Wallet-specific isolation                            │
└─────────────────────────────────────────────────────────┘
```

### **Features**:
- BPI Address input (format: `bpi:wallet:xxx`)
- Access Token input (secure, masked)
- "Generate New Wallet" link → redirects to BPCI dashboard
- Token validation via Component 5 (BPI Bridge)
- Session management with JWT

### **Access Control**:
```typescript
// Authentication Flow
const authenticate = async (address: string, token: string) => {
  // Verify with Component 5
  const response = await axios.post('http://localhost:6001/wallet/auth', {
    address,
    token
  });
  
  if (response.data.valid) {
    // Store session
    localStorage.setItem('mojo_session', response.data.session_token);
    localStorage.setItem('bpi_address', address);
    
    // Redirect to dashboard
    navigate('/dashboard');
  }
};
```

---

## **Page 2: Dashboard (Overview)**

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ [☰] Mojo Wallet    bpi:wallet:abc123    [🔔] [👤] [⚙️]             │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Dashboard Overview                                  Last 24 hours  │
│                                                                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌────────┐│
│  │ Token Balance│  │ Transactions │  │ Consensus    │  │Network │││
│  │              │  │              │  │              │  │        │││
│  │   850 BPI    │  │      45      │  │  Round #123  │  │  ✅    │││
│  │              │  │   12 pending │  │   Active     │  │ Healthy│││
│  │ ──────────── │  │ ──────────── │  │ ──────────── │  │────────│││
│  │ ↑ +50 today  │  │ ↑ +5 today   │  │ 98% success  │  │ 45 ms  │││
│  └──────────────┘  └──────────────┘  └──────────────┘  └────────┘││
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Transaction Activity (Last 7 Days)                             ││
│  │                                                                 ││
│  │  [Line Chart: Transactions over time]                          ││
│  │                                                                 ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌──────────────────────────┐  ┌──────────────────────────────────┐│
│  │ Recent Transactions      │  │ Component Health Status          ││
│  │                          │  │                                  ││
│  │ • TX #abc... +50 BPI     │  │ ✅ Consensus Server (9001)      ││
│  │ • TX #def... -20 BPI     │  │ ✅ Blockchain Server (8080)     ││
│  │ • TX #ghi... +10 BPI     │  │ ✅ BPI Bridge (6001)            ││
│  │                          │  │ ✅ Cluster Ledger (8086)        ││
│  │ [View All →]             │  │ ✅ Network Server (7001)        ││
│  └──────────────────────────┘  │ ✅ Shadow Registry (7003)       ││
│                                 │                                  ││
│                                 │ [View Details →]                 ││
│                                 └──────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────┘
```

### **Features**:
- **Top Navigation**: Wallet address, notifications, profile, settings
- **Key Metrics Cards**: Token balance, transactions, consensus, network
- **Real-time Charts**: Transaction activity (Recharts)
- **Recent Activity**: Latest transactions
- **Component Status**: Health of all 8 BPCI components
- **Auto-refresh**: Every 30 seconds

### **Data Sources**:
```typescript
// Dashboard Data Aggregation
const loadDashboard = async () => {
  const [balance, transactions, consensus, components] = await Promise.all([
    axios.get(`http://localhost:6001/account/${address}`),      // Component 5
    axios.get(`http://localhost:8080/blockchain/transactions`), // Component 2
    axios.get('http://localhost:9001/api/v1/consensus/status'), // Component 1
    axios.get('http://localhost:8089/api/v1/components')        // Mojo Server
  ]);
  
  setDashboardData({ balance, transactions, consensus, components });
};
```

---

## **Page 3: Wallet Management**

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ [☰] Wallet Management                                               │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Your Wallet: bpi:wallet:abc123                                     │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Balance & Tokens                                                ││
│  │                                                                  ││
│  │  Available Balance:  850 BPI                                    ││
│  │  Reserved:           50 BPI  (for gas fees)                     ││
│  │  Total:              900 BPI                                    ││
│  │                                                                  ││
│  │  Monthly Allocation: 1000 BPI (Testnet Plan)                    ││
│  │  Used This Month:    150 BPI                                    ││
│  │  [Progress Bar: 15% used]                                       ││
│  │                                                                  ││
│  │  [Send Tokens]  [Receive]  [View History]                       ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Pricing Plan                                                    ││
│  │                                                                  ││
│  │  Current Plan: Testnet                                          ││
│  │  Monthly Cost: 10 CAD                                           ││
│  │  Free Period:  25 days remaining                                ││
│  │                                                                  ││
│  │  [Upgrade to Pilot]  [View All Plans]                           ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Send Tokens                                                     ││
│  │                                                                  ││
│  │  To Address:  [bpi:wallet:_________________]                    ││
│  │  Amount:      [________] BPI                                    ││
│  │  Gas Fee:     ~0.5 BPI (estimated)                              ││
│  │  Total:       [________] BPI                                    ││
│  │                                                                  ││
│  │  [Cancel]  [Send Transaction]                                   ││
│  └────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────┘
```

### **Features**:
- Balance display (available, reserved, total)
- Monthly allocation tracking
- Pricing plan information
- Send/receive tokens
- Transaction fee estimation
- Plan upgrade options

---

## **Page 4: Transactions**

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ [☰] Transactions                                                    │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  [All] [Sent] [Received] [Pending]          [Search: ________]     │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ TX Hash         │ Type    │ Amount  │ Status    │ Time         ││
│  ├────────────────────────────────────────────────────────────────┤│
│  │ 0xabc...123     │ Sent    │ -50 BPI │ ✅ Confirmed │ 2 min ago ││
│  │ 0xdef...456     │ Received│ +100 BPI│ ✅ Confirmed │ 1 hour ago││
│  │ 0xghi...789     │ Sent    │ -20 BPI │ ⏳ Pending  │ Just now  ││
│  │ 0xjkl...012     │ Received│ +75 BPI │ ✅ Confirmed │ 3 hours   ││
│  │ 0xmno...345     │ Sent    │ -10 BPI │ ✅ Confirmed │ 1 day ago ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  [Load More]                                                        │
│                                                                      │
│  Transaction Details (Click to expand):                             │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Transaction: 0xabc...123                                        ││
│  │                                                                  ││
│  │ From:        bpi:wallet:abc123 (You)                            ││
│  │ To:          bpi:wallet:xyz789                                  ││
│  │ Amount:      50 BPI                                             ││
│  │ Gas Fee:     0.5 BPI                                            ││
│  │ Total:       50.5 BPI                                           ││
│  │ Status:      ✅ Confirmed                                       ││
│  │ Block:       #12345                                             ││
│  │ Timestamp:   2025-10-27 00:20:15 UTC                            ││
│  │ Confirmations: 12                                               ││
│  │                                                                  ││
│  │ [View on Explorer]  [Export Receipt]                            ││
│  └────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────┘
```

### **Features**:
- Transaction list with filters
- Real-time status updates
- Transaction details expansion
- Search functionality
- Export receipts
- Block explorer integration

---

## **Page 5: Monitoring & Metrics (Grafana-Like)**

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ [☰] Monitoring & Metrics                    [⏱ Last 24h ▼]         │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ System Performance                                            │  │
│  │                                                                │  │
│  │ [Multi-line Chart: CPU, Memory, Network over time]            │  │
│  │                                                                │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌────────────────────┐  ┌────────────────────┐  ┌──────────────┐ │
│  │ Transaction Rate   │  │ Consensus Success  │  │ Network Lag  │ │
│  │                    │  │                    │  │              │ │
│  │ [Gauge: 45 tx/s]   │  │ [Gauge: 98%]       │  │ [Gauge: 45ms]│ │
│  └────────────────────┘  └────────────────────┘  └──────────────┘ │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ Component Health Matrix                                       │  │
│  │                                                                │  │
│  │  Component 1 (Consensus):    ████████████████████ 100%        │  │
│  │  Component 2 (Blockchain):   ████████████████████ 100%        │  │
│  │  Component 3 (Auction):      ███████████████████░  95%        │  │
│  │  Component 4 (Auction DB):   ████████████████████ 100%        │  │
│  │  Component 5 (BPI Bridge):   ████████████████████ 100%        │  │
│  │  Component 6 (Cluster):      ████████████████████ 100%        │  │
│  │  Component 7 (Network):      ████████████████████ 100%        │  │
│  │  Component 8 (Shadow):       ████████████████████ 100%        │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ Custom Metrics Panel                                          │  │
│  │                                                                │  │
│  │ [+ Add Panel]  [Edit Layout]  [Export Dashboard]              │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### **Features**:
- Real-time performance charts
- Component health matrix
- Custom metric panels
- Time range selector
- Grafana-like visualization
- Alert thresholds

---

## **Page 6: Security & Alerts**

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ [☰] Security & Alerts                                               │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Security Status: ✅ All Systems Secure                             │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Active Alerts                                                   ││
│  │                                                                  ││
│  │ ⚠️  High transaction volume detected (2 min ago)                ││
│  │ ℹ️  New login from different location (1 hour ago)              ││
│  │ ✅ Consensus round completed successfully (5 min ago)           ││
│  │                                                                  ││
│  │ [View All Alerts]  [Configure Notifications]                    ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Security Events (Last 7 Days)                                   ││
│  │                                                                  ││
│  │ [Bar Chart: Login attempts, transactions, alerts]               ││
│  │                                                                  ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Access Control                                                  ││
│  │                                                                  ││
│  │ Current Token:  ••••••••••••••••••••••                          ││
│  │ Expires:        30 days                                         ││
│  │ Permissions:    Read, Write, Transfer                           ││
│  │                                                                  ││
│  │ [Regenerate Token]  [Revoke Access]                             ││
│  └────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────┘
```

### **Features**:
- Real-time security alerts
- Security event timeline
- Access control management
- Token regeneration
- Alert configuration

---

## **🔐 Token + Address-Based Access Control**

### **Architecture**:

```typescript
// Access Control System
interface MojoAccessControl {
  bpiAddress: string;           // Wallet address
  accessToken: string;          // JWT token
  permissions: Permission[];    // Read, Write, Transfer, Admin
  expiresAt: Date;             // Token expiration
  rateLimit: number;           // Requests per minute
  ipWhitelist?: string[];      // Optional IP restrictions
}

// Scalable for Millions of Wallets
class WalletAccessManager {
  // Redis cache for fast lookups
  private cache: Redis;
  
  // Database for persistent storage
  private db: PostgreSQL;
  
  async validateAccess(address: string, token: string): Promise<boolean> {
    // Check cache first (O(1) lookup)
    const cached = await this.cache.get(`access:${address}`);
    if (cached) return this.verifyToken(cached, token);
    
    // Fallback to database
    const access = await this.db.query(
      'SELECT * FROM wallet_access WHERE address = $1',
      [address]
    );
    
    // Cache for future requests
    await this.cache.set(`access:${address}`, access, 'EX', 3600);
    
    return this.verifyToken(access, token);
  }
}
```

---

## **📱 Responsive Design**

### **Desktop** (1920x1080):
- Full dashboard with all panels
- Side navigation
- Multi-column layout

### **Tablet** (768x1024):
- Collapsible navigation
- 2-column layout
- Touch-optimized controls

### **Mobile** (375x667):
- Bottom navigation
- Single-column layout
- Swipe gestures

---

## **🎨 Design System**

### **Colors**:
```
Primary:   #1890ff (Blue)
Success:   #52c41a (Green)
Warning:   #faad14 (Orange)
Error:     #ff4d4f (Red)
Dark:      #001529 (Navy)
Light:     #f0f2f5 (Gray)
```

### **Typography**:
```
Headings:  Inter, -apple-system, sans-serif
Body:      Inter, -apple-system, sans-serif
Code:      'Fira Code', monospace
```

### **Components**:
- Ant Design (UI framework)
- Recharts (charts)
- React Router (navigation)
- Zustand (state management)

---

## **✅ Summary**

**Complete Wireframe Design** for:
1. ✅ Login/Authentication (token + address)
2. ✅ Dashboard (overview with metrics)
3. ✅ Wallet Management (balance, send/receive)
4. ✅ Transactions (history, details)
5. ✅ Monitoring (Grafana-like metrics)
6. ✅ Security & Alerts (access control)
7. ✅ Network & Nodes (8-component integration)
8. ✅ Settings (preferences, theme)
9. ✅ Admin Panel (Mojo Super)

**Scalability**: Designed for millions of wallets with Redis caching and efficient database queries

**Next**: Implement UI based on these wireframes! 🚀
