# Component 9: Complete Page Designs - Stage 1 (Pages 1-5)

**Total Pages**: 15  
**Stage 1**: Pages 1-5 (Keycloak Auth + Dual-Auth Wizard)  
**Date**: 2025-10-27

---

## **📊 Page Overview**

### **Stage 1 Pages**:
1. **Keycloak Login** (Level 1 Auth)
2. **Basic Dashboard** (After Keycloak login)
3. **Dual-Auth Wizard - Step 1: Generate** (Create BPI address + token)
4. **Dual-Auth Wizard - Step 2: Bind** (Link to Keycloak)
5. **Dual-Auth Wizard - Step 3: Harden** (Security verification)

---

## **Page 1: Keycloak Login (Level 1 Authentication)**

### **Route**: `/login`

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────┐
│                                                              │
│                    [BPCI ENTERPRISE LOGO]                   │
│                  Blockchain Infrastructure                  │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │                                                         │ │
│  │  Welcome Back                                           │ │
│  │  Sign in to access your BPCI dashboard                  │ │
│  │                                                         │ │
│  │  Email Address                                          │ │
│  │  ┌───────────────────────────────────────────────────┐ │ │
│  │  │ user@example.com                                  │ │ │
│  │  └───────────────────────────────────────────────────┘ │ │
│  │                                                         │ │
│  │  Password                                               │ │
│  │  ┌───────────────────────────────────────────────────┐ │ │
│  │  │ ••••••••••••••                                    │ │ │
│  │  └───────────────────────────────────────────────────┘ │ │
│  │                                                         │ │
│  │  [ ] Remember me          Forgot password?             │ │
│  │                                                         │ │
│  │  ┌───────────────────────────────────────────────────┐ │ │
│  │  │         Sign In with Keycloak                     │ │ │
│  │  └───────────────────────────────────────────────────┘ │ │
│  │                                                         │ │
│  │  ─────────────── OR ───────────────                    │ │
│  │                                                         │ │
│  │  ┌───────────────────────────────────────────────────┐ │ │
│  │  │         Create New Account                        │ │ │
│  │  └───────────────────────────────────────────────────┘ │ │
│  │                                                         │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                              │
│  🔒 Powered by Keycloak SSO                                 │
│  • Enterprise-grade security                                │
│  • SAML, OAuth2, OpenID Connect support                     │
│  • Multi-factor authentication available                    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### **Features**:
- Email + password input
- "Remember me" checkbox
- Forgot password link
- Keycloak SSO button
- Sign up link
- ARIA labels for accessibility
- Keyboard navigation (Tab, Enter)

### **State Management** (Zustand):
```typescript
interface AuthSlice {
  level1: {
    isAuthenticated: boolean;
    user: KeycloakUser | null;
    token: string | null;
  };
  login: (email: string, password: string) => Promise<void>;
  logout: () => void;
}
```

### **API Call**:
```typescript
POST /api/auth/keycloak/login
Body: { email, password }
Response: { token, user, redirectUrl }
```

---

## **Page 2: Basic Dashboard (After Keycloak Login)**

### **Route**: `/dashboard`

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ [☰] BPCI Dashboard    user@example.com    [🔔] [👤] [⚙️]            │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Welcome back, John Doe! 👋                                         │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 🎯 Unlock Advanced Features with Mojo Wallet                   ││
│  │                                                                  ││
│  │ Get your BPI wallet address and access token to unlock:         ││
│  │                                                                  ││
│  │ ✅ Real-time transaction monitoring                             ││
│  │ ✅ Advanced metrics & Grafana-style dashboards                  ││
│  │ ✅ 8-component BPCI integration                                 ││
│  │ ✅ Security alerts & compliance tracking                        ││
│  │ ✅ Deploy & manage complete BPI OS infrastructure               ││
│  │                                                                  ││
│  │ ┌──────────────────────────┐  ┌──────────────────────────────┐ ││
│  │ │  Join Mojo Now          │  │  Learn More                  │ ││
│  │ └──────────────────────────┘  └──────────────────────────────┘ ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │
│  │ Your Account │  │ Documentation│  │ Community    │             │
│  │              │  │              │  │              │             │
│  │ ✅ Email     │  │ 📚 API Docs  │  │ 💬 Blog      │             │
│  │ ✅ Profile   │  │ 📖 Tutorials │  │ 🤝 Forum     │             │
│  │ ⏳ Mojo      │  │ 💡 Examples  │  │ 📧 Contact   │             │
│  │              │  │              │  │              │             │
│  │ [Activate]   │  │ [Browse]     │  │ [Explore]    │             │
│  └──────────────┘  └──────────────┘  └──────────────┘             │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Your BPI Connections                                            ││
│  │                                                                  ││
│  │ 📭 No connections yet                                           ││
│  │                                                                  ││
│  │ Generate your first BPI connection to activate Mojo wallet      ││
│  │ and access advanced monitoring features.                        ││
│  │                                                                  ││
│  │ ┌────────────────────────────────────────────────────────────┐ ││
│  │ │  + Generate BPI Connection                                  │ ││
│  │ └────────────────────────────────────────────────────────────┘ ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Recent Community Activity                                       ││
│  │                                                                  ││
│  │ 📝 Latest Blog Posts:                                           ││
│  │ • "How to Deploy BPI Node on Raspberry Pi" - 2h ago             ││
│  │ • "BPCI Security Best Practices" - 5h ago                       ││
│  │ • "Understanding the 4-Coin Economy" - 1d ago                   ││
│  │                                                                  ││
│  │ [View All Posts]  [Create Post]                                 ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Quick Actions (Basic Dashboard)                                ││
│  │                                                                  ││
│  │ [📝 Write Blog Post]  [💬 Join Community]  [📚 Read Docs]      ││
│  │ [🎯 Activate Mojo]    [📧 Contact Us]      [🔧 Settings]       ││
│  │                                                                  ││
│  │ 🔒 Advanced Actions (Requires Mojo Wallet):                     ││
│  │    📊 View Metrics  |  🔐 Security Panel  |  💼 Manage Wallet  ││
│  │                                                                  ││
│  └────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────┘
```

### **Features**:
- Welcome message with user name
- "Join Mojo" call-to-action card (emphasizing BPI OS deployment capability)
- Account status indicators
- Documentation links
- Community resources (Blog, Forum, Contact)
- BPI connection generator button
- **Recent Community Activity** (Blog posts)
- **Quick Actions** for common tasks (node deployment moved to Mojo Wallet section)

### **State Management**:
```typescript
interface DashboardSlice {
  user: KeycloakUser;
  hasMojoWallet: boolean;
  bpiConnections: BpiConnection[];
  recentBlogPosts: BlogPost[];
  loadConnections: () => Promise<void>;
  loadRecentPosts: () => Promise<void>;
}
```

### **Blog Integration**:
```typescript
// Load recent blog posts for dashboard
const loadRecentPosts = async () => {
  const posts = await blogService.getPosts({ 
    sortBy: 'newest', 
    limit: 3 
  });
  setRecentBlogPosts(posts);
};

// Quick action: Create blog post
const handleCreatePost = () => {
  navigate('/blog?action=create');
};
```

---

## **Page 3: Dual-Auth Wizard - Step 1 (Generate)**

### **Route**: `/mojo/activate/step-1`

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ Activate Mojo Wallet                                          [✕]   │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Step 1 of 3: Generate BPI Connection                               │
│  ●━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━│
│  ●                     ○                     ○                       │
│  Generate              Bind                  Harden                  │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Create Your BPI Connection                                      ││
│  │                                                                  ││
│  │ This will generate a unique BPI wallet address and access       ││
│  │ token for your account.                                         ││
│  │                                                                  ││
│  │ Connection Name *                                               ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ My Production Node                                           │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ Description (optional)                                          ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ Main BPI OS node for production environment                  │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ Select Pricing Plan                                             ││
│  │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          ││
│  │ │ Testnet      │  │ Developer    │  │ Pilot        │          ││
│  │ │ 10 CAD/month │  │ 25 CAD/month │  │ 50 CAD/month │          ││
│  │ │ 1000 BPI     │  │ 2500 BPI     │  │ 5000 BPI     │          ││
│  │ │ BPI OS Lite  │  │ Full BPI OS  │  │ Full BPI OS  │          ││
│  │ │ [Selected]   │  │ [Select]     │  │ [Select]     │          ││
│  │ └──────────────┘  └──────────────┘  └──────────────┘          ││
│  │                                                                  ││
│  │ ℹ️ You'll receive 200 BPI free for 1 month trial                ││
│  │                                                                  ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌──────────────┐  ┌──────────────────────────────────────────────┐│
│  │   Cancel     │  │   Generate Connection →                      ││
│  └──────────────┘  └──────────────────────────────────────────────┘│
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### **Features**:
- Progress indicator (Step 1 of 3)
- Connection name input (required)
- Description textarea (optional)
- Pricing plan selector (Testnet, Developer, Pilot)
- Free trial information
- Cancel and Next buttons

### **After Generation Success**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ BPI Connection Created Successfully! ✅                              │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Your BPI connection has been created. Please save these            │
│  credentials securely.                                              │
│                                                                      │
│  BPI Wallet Address                                                 │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ bpi:wallet:abc123def456ghi789                                   ││
│  │                                              [Copy] [QR Code]   ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  Access Token                                                       │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...                         ││
│  │                                              [Copy] [Show/Hide] ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ⚠️ IMPORTANT: Save your access token securely!                     │
│  You'll need both the address and token to access your Mojo wallet. │
│                                                                      │
│  ┌──────────────┐  ┌──────────────────────────────────────────────┐│
│  │   I Saved It │  │   Continue to Step 2 →                       ││
│  └──────────────┘  └──────────────────────────────────────────────┘│
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### **State Management**:
```typescript
interface WizardSlice {
  step: 1 | 2 | 3;
  data: {
    connectionName: string;
    description: string;
    pricingPlan: 'testnet' | 'developer' | 'pilot';
    bpiAddress?: string;
    accessToken?: string;
  };
  generateConnection: () => Promise<void>;
}
```

### **API Call**:
```typescript
POST /api/v1/bpi/generate-connection
Body: { name, description, pricingPlan }
Response: { address, token, dashboardUrl }
```

---

## **Page 4: Dual-Auth Wizard - Step 2 (Bind)**

### **Route**: `/mojo/activate/step-2`

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ Activate Mojo Wallet                                          [✕]   │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Step 2 of 3: Bind to Keycloak Account                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━│
│  ●                     ●                     ○                       │
│  Generate              Bind                  Harden                  │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Link BPI Wallet to Your Account                                 ││
│  │                                                                  ││
│  │ We'll link your BPI wallet to your Keycloak account for         ││
│  │ seamless authentication.                                        ││
│  │                                                                  ││
│  │ Your Keycloak Account                                           ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ ✅ john.doe@example.com                                      │││
│  │ │    Verified                                                   │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ BPI Wallet Address                                              ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ bpi:wallet:abc123def456ghi789                                │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ Enter Access Token to Confirm                                   ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ [Paste your access token here]                               │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ ✅ Binding will enable:                                         ││
│  │    • Single sign-on with Keycloak                               ││
│  │    • Automatic token refresh                                    ││
│  │    • Unified session management                                 ││
│  │                                                                  ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌──────────────┐  ┌──────────────────────────────────────────────┐│
│  │   ← Back     │  │   Bind & Continue →                          ││
│  └──────────────┘  └──────────────────────────────────────────────┘│
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### **Features**:
- Progress indicator (Step 2 of 3)
- Keycloak account display (verified)
- BPI address display (from Step 1)
- Access token input field
- Benefits list
- Back and Continue buttons

### **API Call**:
```typescript
POST /api/v1/auth/bind-keycloak
Body: { keycloakId, bpiAddress, accessToken }
Response: { bound: true, sessionToken }
```

---

## **Page 5: Dual-Auth Wizard - Step 3 (Harden)**

### **Route**: `/mojo/activate/step-3`

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ Activate Mojo Wallet                                          [✕]   │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Step 3 of 3: Harden Security                                       │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━│
│  ●                     ●                     ●                       │
│  Generate              Bind                  Harden                  │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Secure Your Mojo Wallet                                         ││
│  │                                                                  ││
│  │ Add an extra layer of security to protect your wallet.          ││
│  │                                                                  ││
│  │ ✅ Enable Two-Factor Authentication (Recommended)               ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ [ ] Email OTP                                                │││
│  │ │ [ ] Authenticator App (Google Authenticator, Authy)          │││
│  │ │ [ ] SMS (Not recommended for production)                     │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ ✅ Set Security Questions (Optional)                            ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ Question 1: What was your first pet's name?                  │││
│  │ │ [Answer]                                                      │││
│  │ │                                                               │││
│  │ │ Question 2: What city were you born in?                      │││
│  │ │ [Answer]                                                      │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ ✅ Backup & Recovery                                            ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ Recovery Email: john.doe@example.com                         │││
│  │ │ [✓] Send me recovery codes                                   │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌──────────────┐  ┌──────────────────────────────────────────────┐│
│  │   ← Back     │  │   Complete Activation →                      ││
│  └──────────────┘  └──────────────────────────────────────────────┘│
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### **Features**:
- Progress indicator (Step 3 of 3)
- 2FA options (Email OTP, Authenticator App, SMS)
- Security questions (optional)
- Recovery email configuration
- Backup codes option
- Back and Complete buttons

### **After Completion**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ 🎉 Mojo Wallet Activated Successfully!                              │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Your Mojo wallet is now active and ready to use!                   │
│                                                                      │
│  ✅ BPI Address: bpi:wallet:abc123def456ghi789                      │
│  ✅ Keycloak Account: Linked                                        │
│  ✅ Security: 2FA Enabled                                           │
│  ✅ Pricing Plan: Testnet (10 CAD/month)                            │
│  ✅ Free Trial: 200 BPI for 30 days                                 │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │   Access Mojo Wallet Dashboard →                                ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### **API Call**:
```typescript
POST /api/v1/auth/harden
Body: { twoFactorMethod, securityQuestions, recoveryEmail }
Response: { activated: true, redirectUrl: '/mojo-wallet/dashboard' }
```

---

## **Page 5B: Mojo Wallet Dashboard - BPI OS Node Management**

### **Route**: `/mojo-wallet/dashboard`

### **Important Clarification**:
**"Running a Node" = Deploying Complete BPI Immutable OS**

When we say "running a node" in the BPCI infrastructure, we mean deploying and activating the **entire BPI Immutable OS** with all its services and components. This is NOT just a simple blockchain node - it's a complete operating system layer that includes:

- **BPI VM Server** (Port 7777) - Virtual machine orchestration
- **HTTP Cage** (Port 8888) - Secure HTTP gateway with wallet authentication
- **Shadow Registry** (Port 8080) - Web2-Web3 bridge
- **ZKLock Mobile** (Port 8081) - Zero-knowledge authentication
- **ENC Cluster** - Execution network with military-grade security
- **DockLock Platform** - Deterministic container execution
- **Oracle Nodes** - Cross-chain communication
- **Storage Nodes** - Distributed CueDB storage
- **Logbook Nodes** - Immutable audit trails
- **Forensic Firewall** - Security monitoring and intrusion detection

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ [☰] Mojo Wallet Dashboard    user@example.com    [🔔] [👤] [⚙️]     │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Welcome to Mojo Wallet! 🎉                                         │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Your BPI Connection                                             ││
│  │                                                                  ││
│  │ Address: bpi:wallet:abc123def456ghi789              [Copy]      ││
│  │ Plan: Developer (25 CAD/month)                                  ││
│  │ Balance: 2500 BPI                                               ││
│  │ Status: ✅ Active                                               ││
│  │                                                                  ││
│  │ [View Transactions] [Manage Subscription] [Security Settings]   ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 🚀 BPI OS Node Deployment                                       ││
│  │                                                                  ││
│  │ Deploy and manage your complete BPI Immutable OS infrastructure ││
│  │                                                                  ││
│  │ Node Status: ⏸️ Not Deployed                                    ││
│  │                                                                  ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ What You'll Get:                                             │││
│  │ │                                                               │││
│  │ │ ✅ Complete BPI Immutable OS (All Services)                  │││
│  │ │ ✅ BPI VM Server (Port 7777)                                 │││
│  │ │ ✅ HTTP Cage Gateway (Port 8888)                             │││
│  │ │ ✅ Shadow Registry Bridge (Port 8080)                        │││
│  │ │ ✅ ZKLock Authentication (Port 8081)                         │││
│  │ │ ✅ ENC Cluster + DockLock Platform                           │││
│  │ │ ✅ Oracle + Storage + Logbook Nodes                          │││
│  │ │ ✅ Forensic Firewall & Security Monitoring                   │││
│  │ │                                                               │││
│  │ │ System Requirements (Ultra-Lightweight vPod Technology):     │││
│  │ │ • 2GB RAM minimum (1GB possible, 2GB recommended)            │││
│  │ │ • 1 vCPU core                                                │││
│  │ │ • 25GB disk space                                            │││
│  │ │ • Linux OS (Ubuntu 20.04+, Debian, CentOS, RHEL)            │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ Deployment Options:                                             ││
│  │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          ││
│  │ │ Local        │  │ Cloud VPS    │  │ Kubernetes   │          ││
│  │ │ (Your PC)    │  │ (DigitalOcean│  │ (K8s Cluster)│          ││
│  │ │              │  │  AWS, etc)   │  │              │          ││
│  │ │ [Deploy]     │  │ [Deploy]     │  │ [Deploy]     │          ││
│  │ └──────────────┘  └──────────────┘  └──────────────┘          ││
│  │                                                                  ││
│  │ [📖 Read Deployment Guide] [💬 Get Help]                        ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 📊 Advanced Features (Available After Node Deployment)         ││
│  │                                                                  ││
│  │ 🔒 Locked - Deploy BPI OS to unlock:                            ││
│  │                                                                  ││
│  │ • Real-time Transaction Monitoring                              ││
│  │ • Grafana-Style Metrics Dashboard                               ││
│  │ • 8-Component BPCI Integration                                  ││
│  │ • Security Alerts & Compliance Tracking                         ││
│  │ • Custom Panel Builder                                          ││
│  │ • API Access & Webhooks                                         ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### **After Clicking "Deploy" - Installation Wizard**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ Deploy BPI Immutable OS                                       [✕]   │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Step 1 of 4: System Check                                          │
│  ●━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━│
│  ●                ○                ○                ○                │
│  System Check     Configure       Download         Deploy           │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Checking System Requirements...                                 ││
│  │                                                                  ││
│  │ ✅ Operating System: Ubuntu 22.04 LTS                           ││
│  │ ✅ RAM: 4GB available (2GB minimum - vPod technology!)          ││
│  │ ✅ CPU: 2 cores (1 vCPU minimum)                                ││
│  │ ✅ Disk Space: 50GB available (25GB minimum)                    ││
│  │ ✅ Network: Internet connection active                          ││
│  │ ✅ Ports: 7777, 8080, 8081, 8888 available                      ││
│  │                                                                  ││
│  │ System Status: ✅ Ready for Deployment                          ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌──────────────┐  ┌──────────────────────────────────────────────┐│
│  │   Cancel     │  │   Continue to Configuration →                ││
│  └──────────────┘  └──────────────────────────────────────────────┘│
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### **Features**:
- Clear explanation that "node" = complete BPI OS deployment
- Visual list of all services included in BPI OS
- System requirements check
- Multiple deployment options (Local, Cloud VPS, Kubernetes)
- Locked advanced features until node is deployed
- Step-by-step installation wizard
- Real-time system compatibility check

### **State Management**:
```typescript
interface MojoWalletSlice {
  bpiConnection: {
    address: string;
    plan: 'testnet' | 'developer' | 'pilot';
    balance: number;
    status: 'active' | 'suspended' | 'inactive';
  };
  nodeDeployment: {
    status: 'not_deployed' | 'deploying' | 'active' | 'stopped' | 'error';
    deploymentType?: 'local' | 'cloud' | 'kubernetes';
    services: {
      vmServer: boolean;
      httpCage: boolean;
      shadowRegistry: boolean;
      zkLock: boolean;
      encCluster: boolean;
      dockLock: boolean;
      oracle: boolean;
      storage: boolean;
      logbook: boolean;
      forensicFirewall: boolean;
    };
  };
  deployNode: (type: string) => Promise<void>;
  checkSystemRequirements: () => Promise<SystemCheckResult>;
}
```

### **API Calls**:
```typescript
// Check system requirements
GET /api/v1/mojo-wallet/system-check
Response: { 
  os: string, 
  ram: number,        // in GB (minimum 2GB recommended, 1GB possible)
  cpu: number,        // number of cores (minimum 1 vCPU)
  disk: number,       // in GB (minimum 25GB)
  portsAvailable: boolean,
  ready: boolean,
  vpodSupported: boolean  // vPod ultra-lightweight mode
}

// Deploy BPI OS
POST /api/v1/mojo-wallet/deploy-node
Body: { 
  deploymentType: 'local' | 'cloud' | 'kubernetes',
  configuration: {...}
}
Response: { 
  deploymentId: string, 
  status: string, 
  installScript: string 
}

// Monitor deployment progress
GET /api/v1/mojo-wallet/deployment-status/:deploymentId
Response: { 
  status: string, 
  progress: number, 
  currentStep: string,
  services: {...}
}
```

---

---

## **Page 6: Blog Posting Form**

### **Route**: `/blog` (with create post modal)

### **Purpose**: Allow community members to post experiences, documentation, bug reports, and general content

### **Existing Implementation**: ✅ Already implemented in `Blog.tsx`

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ [☰] Community Post                user@example.com    [🔔] [👤]     │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ← Back to Community                                                │
│                                                                      │
│  Create Community Post                                              │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Post Type *                                                     ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ [Tutorial ▼]                                                 │││
│  │ │  • Tutorial                                                   │││
│  │ │  • Project Showcase                                          │││
│  │ │  • Question/Help                                             │││
│  │ │  • Announcement                                              │││
│  │ │  • Discussion                                                │││
│  │ │  • Bug Report                                                │││
│  │ │  • Feature Request                                           │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ Title *                                                         ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ How to Deploy BPI Node on Raspberry Pi                      │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ Category *                                                      ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ [Development ▼]                                              │││
│  │ │  • Development                                               │││
│  │ │  • Infrastructure                                            │││
│  │ │  • Security                                                  │││
│  │ │  • Community                                                 │││
│  │ │  • General                                                   │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ Tags (comma-separated)                                          ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ raspberry-pi, tutorial, deployment, beginner                 │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ Content * (Markdown supported)                                  ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ ## Introduction                                              │││
│  │ │                                                               │││
│  │ │ This tutorial will guide you through deploying a BPI node   │││
│  │ │ on a Raspberry Pi 4 with 8GB RAM.                           │││
│  │ │                                                               │││
│  │ │ ### Prerequisites                                            │││
│  │ │ - Raspberry Pi 4 (8GB recommended)                           │││
│  │ │ - MicroSD card (64GB+)                                       │││
│  │ │ - Stable internet connection                                 │││
│  │ │                                                               │││
│  │ │ ### Step 1: Install Dependencies                             │││
│  │ │ ```bash                                                       │││
│  │ │ sudo apt update && sudo apt install -y curl git             │││
│  │ │ ```                                                           │││
│  │ │                                                               │││
│  │ │ [Preview] [Markdown Guide]                                   │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ Attachments (optional)                                          ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ [+ Upload Images/Files]  (Max 10MB per file)                │││
│  │ │                                                               │││
│  │ │ 📎 raspberry-pi-setup.png (2.3 MB)              [Remove]     │││
│  │ │ 📎 deployment-script.sh (15 KB)                 [Remove]     │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ Visibility                                                      ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ ○ Public (Everyone can see)                                  │││
│  │ │ ● Community Members Only                                     │││
│  │ │ ○ Mojo Wallet Holders Only                                   │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ Notifications                                                   ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ [✓] Notify me of comments                                    │││
│  │ │ [✓] Notify me of reactions                                   │││
│  │ │ [ ] Pin this post (Moderators only)                          │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────────┐ │
│  │ Save Draft   │  │   Preview    │  │   Publish Post           │ │
│  └──────────────┘  └──────────────┘  └──────────────────────────┘ │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### **Features**:

1. **Post Type Selection**:
   - Tutorial
   - Project Showcase
   - Question/Help
   - Announcement
   - Discussion
   - Bug Report
   - Feature Request

2. **Category Selection**:
   - Development
   - Infrastructure
   - Security
   - Community
   - General

3. **Rich Text Editor**:
   - Markdown support
   - Live preview
   - Syntax highlighting for code blocks
   - Image embedding
   - Link insertion

4. **Attachments**:
   - Upload images (PNG, JPG, GIF)
   - Upload files (max 10MB per file)
   - Multiple file support
   - Preview thumbnails

5. **Visibility Options**:
   - Public (everyone)
   - Community members only (Keycloak authenticated)
   - Mojo wallet holders only (Level 2 authenticated)

6. **Notifications**:
   - Comment notifications
   - Reaction notifications
   - Pin post (moderators only)

### **Preview Modal**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ Post Preview                                                   [✕]  │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  [Tutorial] How to Deploy BPI Node on Raspberry Pi                  │
│  by John Doe • Just now • Development                               │
│  Tags: raspberry-pi, tutorial, deployment, beginner                 │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ ## Introduction                                                 ││
│  │                                                                  ││
│  │ This tutorial will guide you through deploying a BPI node      ││
│  │ on a Raspberry Pi 4 with 8GB RAM.                              ││
│  │                                                                  ││
│  │ ### Prerequisites                                               ││
│  │ - Raspberry Pi 4 (8GB recommended)                              ││
│  │ - MicroSD card (64GB+)                                          ││
│  │ - Stable internet connection                                    ││
│  │                                                                  ││
│  │ ### Step 1: Install Dependencies                                ││
│  │ ```bash                                                          ││
│  │ sudo apt update && sudo apt install -y curl git                ││
│  │ ```                                                              ││
│  │                                                                  ││
│  │ [Image: raspberry-pi-setup.png]                                 ││
│  │                                                                  ││
│  │ Attachments:                                                    ││
│  │ 📎 deployment-script.sh                                         ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌──────────────┐  ┌──────────────────────────────────────────────┐│
│  │   Edit       │  │   Publish                                    ││
│  └──────────────┘  └──────────────────────────────────────────────┘│
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### **After Publishing**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ 🎉 Post Published Successfully!                                     │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Your post "How to Deploy BPI Node on Raspberry Pi" has been        │
│  published to the community!                                        │
│                                                                      │
│  ✅ Visible to: Community Members                                   │
│  ✅ Category: Development                                           │
│  ✅ Notifications: Enabled                                          │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │   View Post                                                     ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │   Share on Twitter                                              ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │   Create Another Post                                           ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### **State Management**:
```typescript
interface CommunityPostSlice {
  draft: {
    type: PostType;
    title: string;
    category: string;
    tags: string[];
    content: string;
    attachments: File[];
    visibility: 'public' | 'community' | 'mojo';
    notifications: {
      comments: boolean;
      reactions: boolean;
    };
  };
  saveDraft: () => Promise<void>;
  publishPost: () => Promise<void>;
  uploadAttachment: (file: File) => Promise<string>;
}

type PostType = 
  | 'tutorial' 
  | 'project' 
  | 'question' 
  | 'announcement' 
  | 'discussion' 
  | 'bug' 
  | 'feature';
```

### **API Calls**:
```typescript
// Save draft
POST /api/v1/community/posts/draft
Body: { type, title, category, tags, content, attachments, visibility }
Response: { draftId, savedAt }

// Publish post
POST /api/v1/community/posts
Body: { type, title, category, tags, content, attachments, visibility, notifications }
Response: { postId, publishedAt, url }

// Upload attachment
POST /api/v1/community/posts/upload
Body: FormData { file }
Response: { fileId, url, size }
```

### **Validation Rules**:
- Title: Required, 10-200 characters
- Content: Required, 50-50000 characters
- Tags: Optional, max 10 tags
- Attachments: Optional, max 5 files, 10MB per file
- Post type: Required
- Category: Required

### **Access Control**:
- **Public posts**: Anyone can view
- **Community posts**: Keycloak authenticated users only
- **Mojo posts**: Level 2 authenticated (Mojo wallet holders) only

---

## **✅ Stage 1 Complete (Updated)**

**Pages 1-6 Designed**:
1. ✅ Keycloak Login
2. ✅ Basic Dashboard
3. ✅ Dual-Auth Wizard - Step 1 (Generate)
4. ✅ Dual-Auth Wizard - Step 2 (Bind)
5. ✅ Dual-Auth Wizard - Step 3 (Harden)
6. ✅ Community Posting Form

**Next**: Stage 2 (Pages 7-11) - Mojo Wallet Dashboard & Features

---

---

# Component 9: Complete Page Designs - Stage 2 (Pages 7-11)

**Stage 2**: Pages 7-11 (Mojo Wallet Dashboard & Advanced Features)  
**Date**: 2025-10-27

---

## **📊 Stage 2 Page Overview**

### **Stage 2 Pages**:
7. **Mojo Wallet Dashboard** (Main wallet interface with BPI OS deployment)
8. **BPI OS Deployment Wizard** (4-step deployment process)
9. **Node Management Dashboard** (After successful deployment)
10. **Advanced Metrics & Monitoring** (Grafana-style dashboard)
11. **Wallet Settings & Security** (Manage wallet, 2FA, recovery)

---

## **Page 7: Mojo Wallet Dashboard (Main Interface)**

### **Route**: `/mojo-wallet/dashboard`

### **Purpose**: 
Central hub for Mojo wallet holders showing wallet status, BPI OS deployment options, and access to advanced features.

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ [☰] Mojo Wallet          user@example.com    [🔔] [👤] [⚙️]         │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Welcome to Mojo Wallet! 🎉                                         │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 💼 Your Wallet Overview                                         ││
│  │                                                                  ││
│  │ ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐││
│  │ │ BPI Address      │  │ Plan             │  │ Balance          │││
│  │ │ bpi:wallet:abc.. │  │ Developer        │  │ 2500 BPI         │││
│  │ │ [Copy] [QR]      │  │ 25 CAD/month     │  │ ≈ $125 USD       │││
│  │ └──────────────────┘  └──────────────────┘  └──────────────────┘││
│  │                                                                  ││
│  │ Status: ✅ Active  │  Last Activity: 2 hours ago                ││
│  │                                                                  ││
│  │ [View Transactions] [Top Up Balance] [Manage Subscription]      ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 🚀 BPI OS Node Status                                           ││
│  │                                                                  ││
│  │ ⏸️ Not Deployed                                                 ││
│  │                                                                  ││
│  │ Deploy complete BPI Immutable OS to unlock advanced features:   ││
│  │                                                                  ││
│  │ 🔒 Locked Features:                                             ││
│  │ • Real-time Transaction Monitoring                              ││
│  │ • Advanced Metrics Dashboard (Grafana-style)                    ││
│  │ • 8-Component BPCI Integration                                  ││
│  │ • Security Alerts & Compliance Tracking                         ││
│  │ • API Access & Webhooks                                         ││
│  │                                                                  ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │  🚀 Deploy BPI OS Now                                        │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ System Requirements: 2GB RAM, 1 vCPU, 25GB disk                 ││
│  │ [Learn More] [View Deployment Guide]                            ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 📊 Quick Stats                                                  ││
│  │                                                                  ││
│  │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          ││
│  │ │ Transactions │  │ API Calls    │  │ Uptime       │          ││
│  │ │ 0            │  │ 0            │  │ N/A          │          ││
│  │ │ (Deploy node)│  │ (Deploy node)│  │ (Deploy node)│          ││
│  │ └──────────────┘  └──────────────┘  └──────────────┘          ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 🎯 Quick Actions                                                ││
│  │                                                                  ││
│  │ [🚀 Deploy Node] [💳 Top Up] [📊 View Docs] [⚙️ Settings]      ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### **Features**:
- Wallet overview with BPI address, plan, and balance
- Clear indication of node deployment status
- Locked features list to encourage deployment
- Quick stats (all showing "Deploy node" when not deployed)
- Quick action buttons
- Copy/QR code for BPI address
- Balance in both BPI and USD

### **State Management**:
```typescript
interface MojoWalletDashboardSlice {
  wallet: {
    address: string;
    plan: 'testnet' | 'developer' | 'pilot';
    balance: number;
    balanceUsd: number;
    status: 'active' | 'suspended' | 'inactive';
    lastActivity: string;
  };
  nodeStatus: {
    deployed: boolean;
    status: 'not_deployed' | 'deploying' | 'running' | 'stopped' | 'error';
    services: ServiceStatus[];
  };
  stats: {
    transactions: number;
    apiCalls: number;
    uptime: number | null;
  };
  loadDashboard: () => Promise<void>;
  deployNode: () => void;
}
```

---

## **Page 8: BPI OS Deployment Wizard**

### **Route**: `/mojo-wallet/deploy`

### **Purpose**: 
4-step wizard to deploy complete BPI Immutable OS with system checks, configuration, download, and deployment.

### **Step 1: System Check**

```
┌─────────────────────────────────────────────────────────────────────┐
│ Deploy BPI Immutable OS                                       [✕]   │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Step 1 of 4: System Check                                          │
│  ●━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━│
│  ●                ○                ○                ○                │
│  System Check     Configure       Download         Deploy           │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Checking System Requirements...                                 ││
│  │                                                                  ││
│  │ ✅ Operating System: Ubuntu 22.04 LTS                           ││
│  │ ✅ RAM: 4GB available (2GB minimum)                             ││
│  │ ✅ CPU: 2 cores (1 vCPU minimum)                                ││
│  │ ✅ Disk Space: 50GB available (25GB minimum)                    ││
│  │ ✅ Network: Internet connection active                          ││
│  │ ✅ Ports: 7777, 8080, 8081, 8888 available                      ││
│  │ ✅ BSO-K8 Runtime: Available                                    ││
│  │ ✅ vPod Support: Available (100+ vPods in 2GB RAM)              ││
│  │                                                                  ││
│  │ System Status: ✅ Ready for Deployment                          ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌──────────────┐  ┌──────────────────────────────────────────────┐│
│  │   Cancel     │  │   Continue to Configuration →                ││
│  └──────────────┘  └──────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────┘
```

### **Step 2: Configure**
```
┌─────────────────────────────────────────────────────────────────────┐
│  Step 2 of 4: Configure Deployment                                  │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━│
│  ●                ●                ○                ○                │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Deployment Configuration                                        ││
│  │                                                                  ││
│  │ Deployment Type *                                               ││
│  │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          ││
│  │ │● Local       │  │○ Cloud VPS   │  │○ BSO-K8      │          ││
│  │ │  (This PC)   │  │  (Remote)    │  │  (Cluster)   │          ││
│  │ └──────────────┘  └──────────────┘  └──────────────┘          ││
│  │                                                                  ││
│  │ Node Name *                                                     ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ my-bpi-node-01                                               │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ Services to Deploy (All recommended)                            ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ ☑ BPI VM Server (Port 7777)                                  │││
│  │ │ ☑ HTTP Cage Gateway (Port 8888)                              │││
│  │ │ ☑ Shadow Registry (Port 8080)                                │││
│  │ │ ☑ ZKLock Authentication (Port 8081)                          │││
│  │ │ ☑ ENC Cluster + DockLock                                     │││
│  │ │ ☑ Oracle + Storage + Logbook Nodes                           │││
│  │ │ ☑ Forensic Firewall                                          │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ Advanced Options (Optional)                                     ││
│  │ [▼] Show Advanced Configuration                                 ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌──────────────┐  ┌──────────────────────────────────────────────┐│
│  │   ← Back     │  │   Continue to Download →                     ││
│  └──────────────┘  └──────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────┘
```

### **Step 3: Download**
```
┌─────────────────────────────────────────────────────────────────────┐
│  Step 3 of 4: Download BPI OS Components                            │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━│
│  ●                ●                ●                ○                │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Downloading BPI Immutable OS...                                 ││
│  │                                                                  ││
│  │ ✅ BPI Core Runtime (125MB) - Complete                          ││
│  │ ✅ VM Server Components (85MB) - Complete                       ││
│  │ ✅ HTTP Cage & Shadow Registry (45MB) - Complete                ││
│  │ 🔄 ENC Cluster & DockLock (180MB) - 67% [████████░░░]          ││
│  │ ⏳ Oracle & Storage Nodes (95MB) - Waiting...                   ││
│  │ ⏳ Forensic Firewall (35MB) - Waiting...                        ││
│  │                                                                  ││
│  │ Overall Progress: 58% [████████████░░░░░░░░]                    ││
│  │                                                                  ││
│  │ Downloaded: 325MB / 565MB                                       ││
│  │ Speed: 12.5 MB/s                                                ││
│  │ Time Remaining: ~19 seconds                                     ││
│  │                                                                  ││
│  │ 💡 Tip: BPI OS uses vPod technology for ultra-lightweight       ││
│  │    deployment. Total footprint: ~500-870MB in 2GB RAM!          ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌──────────────┐  [Auto-continues when complete]                  │
│  │   Cancel     │                                                   │
│  └──────────────┘                                                   │
└─────────────────────────────────────────────────────────────────────┘
```

### **Step 4: Deploy**
```
┌─────────────────────────────────────────────────────────────────────┐
│  Step 4 of 4: Deploying BPI OS                                      │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━│
│  ●                ●                ●                ●                │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Initializing BPI Immutable OS...                                ││
│  │                                                                  ││
│  │ ✅ Creating vPod runtime environment                            ││
│  │ ✅ Initializing BPI VM Server (Port 7777)                       ││
│  │ ✅ Starting HTTP Cage Gateway (Port 8888)                       ││
│  │ ✅ Deploying Shadow Registry (Port 8080)                        ││
│  │ ✅ Activating ZKLock Authentication (Port 8081)                 ││
│  │ 🔄 Configuring ENC Cluster...                                   ││
│  │ ⏳ Starting DockLock Platform...                                ││
│  │ ⏳ Initializing Oracle Nodes...                                 ││
│  │                                                                  ││
│  │ Deployment Progress: 71% [██████████████░░░░░]                  ││
│  │                                                                  ││
│  │ 📊 Live Logs:                                                   ││
│  │ [2025-10-27 01:15:23] BPI VM Server started successfully        ││
│  │ [2025-10-27 01:15:24] HTTP Cage listening on 0.0.0.0:8888       ││
│  │ [2025-10-27 01:15:25] Shadow Registry initialized               ││
│  │ [2025-10-27 01:15:26] ZKLock authentication active              ││
│  │ [2025-10-27 01:15:27] Configuring ENC Cluster...                ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  [Deployment in progress... Please wait]                            │
└─────────────────────────────────────────────────────────────────────┘
```

### **Deployment Success**
```
┌─────────────────────────────────────────────────────────────────────┐
│ 🎉 BPI OS Deployed Successfully!                                    │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Your BPI Immutable OS is now running!                              │
│                                                                      │
│  ✅ All Services Active (10/10)                                     │
│  ✅ Node ID: my-bpi-node-01                                         │
│  ✅ Memory Usage: 687MB / 2GB (34%)                                 │
│  ✅ CPU Usage: 0.8 vCPU / 1 vCPU (80%)                              │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Service Endpoints:                                              ││
│  │                                                                  ││
│  │ • BPI VM Server: http://localhost:7777                          ││
│  │ • HTTP Cage: http://localhost:8888                              ││
│  │ • Shadow Registry: http://localhost:8080                        ││
│  │ • ZKLock Auth: http://localhost:8081                            ││
│  │                                                                  ││
│  │ [View All Services]                                             ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │   Go to Node Dashboard →                                        ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## **Page 9: Node Management Dashboard**

### **Route**: `/mojo-wallet/node`

### **Purpose**: 
Comprehensive node management interface showing all deployed services, real-time metrics, and control options.

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ [☰] Node Dashboard       user@example.com    [🔔] [👤] [⚙️]         │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Node: my-bpi-node-01  ●  Status: Running  ●  Uptime: 2h 34m        │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 📊 System Overview                                              ││
│  │                                                                  ││
│  │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          ││
│  │ │ Memory       │  │ CPU          │  │ Disk         │          ││
│  │ │ 687MB / 2GB  │  │ 0.8 / 1 vCPU │  │ 12GB / 25GB  │          ││
│  │ │ 34% [████░░░]│  │ 80% [████████]│  │ 48% [████░░░]│          ││
│  │ └──────────────┘  └──────────────┘  └──────────────┘          ││
│  │                                                                  ││
│  │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          ││
│  │ │ Network I/O  │  │ Transactions │  │ API Calls    │          ││
│  │ │ ↓ 125 KB/s   │  │ 1,247        │  │ 3,891        │          ││
│  │ │ ↑ 89 KB/s    │  │ (24h)        │  │ (24h)        │          ││
│  │ └──────────────┘  └──────────────┘  └──────────────┘          ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 🔧 Services Status (10/10 Active)                               ││
│  │                                                                  ││
│  │ ✅ BPI VM Server (Port 7777)          [Logs] [Restart] [Stop]  ││
│  │    Uptime: 2h 34m  │  Memory: 95MB  │  CPU: 12%                ││
│  │                                                                  ││
│  │ ✅ HTTP Cage Gateway (Port 8888)      [Logs] [Restart] [Stop]  ││
│  │    Uptime: 2h 34m  │  Memory: 48MB  │  CPU: 8%                 ││
│  │                                                                  ││
│  │ ✅ Shadow Registry (Port 8080)        [Logs] [Restart] [Stop]  ││
│  │    Uptime: 2h 34m  │  Memory: 32MB  │  CPU: 5%                 ││
│  │                                                                  ││
│  │ ✅ ZKLock Authentication (Port 8081)  [Logs] [Restart] [Stop]  ││
│  │    Uptime: 2h 34m  │  Memory: 28MB  │  CPU: 4%                 ││
│  │                                                                  ││
│  │ [Show All Services ▼]                                           ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 🎯 Quick Actions                                                ││
│  │                                                                  ││
│  │ [📊 View Metrics] [🔄 Restart All] [⏸️ Stop Node] [⚙️ Configure]││
│  │ [📝 View Logs] [🔐 Security] [📈 Analytics] [💾 Backup]        ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```


---

## **Page 9: Node Management Dashboard**

### **Route**: `/mojo-wallet/node`

### **Purpose**: 
Comprehensive node management interface showing all deployed services, real-time metrics, and control options using BSO-K8 orchestration.

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ [☰] Node Dashboard       user@example.com    [🔔] [👤] [⚙️]         │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Node: my-bpi-node-01  ●  Status: Running  ●  Uptime: 2h 34m        │
│  BSO-K8 Cluster: Active  │  vPods: 127/200  │  Memory: 687MB/2GB    │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 📊 System Overview                                              ││
│  │                                                                  ││
│  │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          ││
│  │ │ Memory       │  │ CPU          │  │ Disk         │          ││
│  │ │ 687MB / 2GB  │  │ 0.8 / 1 vCPU │  │ 12GB / 25GB  │          ││
│  │ │ 34% [████░░░]│  │ 80% [████████]│  │ 48% [████░░░]│          ││
│  │ └──────────────┘  └──────────────┘  └──────────────┘          ││
│  │                                                                  ││
│  │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          ││
│  │ │ vPod Count   │  │ Transactions │  │ API Calls    │          ││
│  │ │ 127 active   │  │ 1,247        │  │ 3,891        │          ││
│  │ │ 73 available │  │ (24h)        │  │ (24h)        │          ││
│  │ └──────────────┘  └──────────────┘  └──────────────┘          ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 🔧 BPI OS Services (10/10 Active)                               ││
│  │                                                                  ││
│  │ ✅ BPI VM Server (Port 7777)          [Logs] [Restart] [Stop]  ││
│  │    vPods: 15  │  Memory: 95MB  │  CPU: 12%  │  Uptime: 2h 34m  ││
│  │                                                                  ││
│  │ ✅ HTTP Cage Gateway (Port 8888)      [Logs] [Restart] [Stop]  ││
│  │    vPods: 8   │  Memory: 48MB  │  CPU: 8%   │  Uptime: 2h 34m  ││
│  │                                                                  ││
│  │ ✅ Shadow Registry (Port 8080)        [Logs] [Restart] [Stop]  ││
│  │    vPods: 6   │  Memory: 32MB  │  CPU: 5%   │  Uptime: 2h 34m  ││
│  │                                                                  ││
│  │ ✅ ZKLock Authentication (Port 8081)  [Logs] [Restart] [Stop]  ││
│  │    vPods: 5   │  Memory: 28MB  │  CPU: 4%   │  Uptime: 2h 34m  ││
│  │                                                                  ││
│  │ ✅ ENC Cluster                        [Logs] [Restart] [Stop]  ││
│  │    vPods: 25  │  Memory: 145MB │  CPU: 18%  │  Uptime: 2h 34m  ││
│  │                                                                  ││
│  │ ✅ DockLock Platform                  [Logs] [Restart] [Stop]  ││
│  │    vPods: 20  │  Memory: 98MB  │  CPU: 15%  │  Uptime: 2h 34m  ││
│  │                                                                  ││
│  │ ✅ Oracle Nodes                       [Logs] [Restart] [Stop]  ││
│  │    vPods: 12  │  Memory: 67MB  │  CPU: 9%   │  Uptime: 2h 34m  ││
│  │                                                                  ││
│  │ ✅ Storage Nodes                      [Logs] [Restart] [Stop]  ││
│  │    vPods: 18  │  Memory: 89MB  │  CPU: 11%  │  Uptime: 2h 34m  ││
│  │                                                                  ││
│  │ ✅ Logbook Nodes                      [Logs] [Restart] [Stop]  ││
│  │    vPods: 10  │  Memory: 52MB  │  CPU: 7%   │  Uptime: 2h 34m  ││
│  │                                                                  ││
│  │ ✅ Forensic Firewall                  [Logs] [Restart] [Stop]  ││
│  │    vPods: 8   │  Memory: 33MB  │  CPU: 5%   │  Uptime: 2h 34m  ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 🎯 Quick Actions                                                ││
│  │                                                                  ││
│  │ [📊 View Metrics] [🔄 Restart All] [⏸️ Stop Node] [⚙️ Configure]││
│  │ [📝 View Logs] [🔐 Security] [📈 Analytics] [💾 Backup]        ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### **Features**:
- Real-time system metrics (Memory, CPU, Disk)
- BSO-K8 cluster status with vPod count
- All 10 BPI OS services with individual vPod counts
- Per-service resource usage and uptime
- Individual service controls (Logs, Restart, Stop)
- Quick actions for node management

### **State Management**:
```typescript
interface NodeDashboardSlice {
  node: {
    name: string;
    status: 'running' | 'stopped' | 'error' | 'deploying';
    uptime: number;
    bsoK8Cluster: {
      active: boolean;
      vPodsActive: number;
      vPodsTotal: number;
    };
  };
  systemMetrics: {
    memory: { used: number; total: number; percentage: number };
    cpu: { used: number; total: number; percentage: number };
    disk: { used: number; total: number; percentage: number };
  };
  services: Array<{
    name: string;
    port: number;
    status: 'active' | 'stopped' | 'error';
    vPods: number;
    memory: number;
    cpu: number;
    uptime: number;
  }>;
  stats: {
    transactions24h: number;
    apiCalls24h: number;
  };
  restartService: (serviceName: string) => Promise<void>;
  stopService: (serviceName: string) => Promise<void>;
  viewLogs: (serviceName: string) => void;
}
```

### **API Calls**:
```typescript
// Get node status
GET /api/v1/mojo-wallet/node/status
Response: {
  node: { name, status, uptime, bsoK8Cluster },
  systemMetrics: { memory, cpu, disk },
  services: [...],
  stats: { transactions24h, apiCalls24h }
}

// Control service
POST /api/v1/mojo-wallet/node/service/:serviceName/restart
POST /api/v1/mojo-wallet/node/service/:serviceName/stop
GET /api/v1/mojo-wallet/node/service/:serviceName/logs
```

---

## **Page 10: Advanced Metrics & Monitoring**

### **Route**: `/mojo-wallet/metrics`

### **Purpose**: 
Grafana-style advanced metrics dashboard with real-time charts, performance analytics, and system monitoring.

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ [☰] Metrics & Monitoring user@example.com    [🔔] [👤] [⚙️]         │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  📊 Advanced Metrics Dashboard                                      │
│                                                                      │
│  Time Range: [Last 24 Hours ▼]  Auto-refresh: [ON]  Interval: 30s  │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 📈 System Performance (Real-time)                               ││
│  │                                                                  ││
│  │ CPU Usage (%)                    Memory Usage (MB)              ││
│  │ 100│                             2048│                          ││
│  │  80│     ╱╲                      1536│         ╱‾‾╲             ││
│  │  60│    ╱  ╲  ╱╲                1024│      ╱‾╲╱    ╲            ││
│  │  40│ ╱‾╲    ╲╱  ╲╱‾╲              512│   ╱‾╲╱         ╲╱‾╲       ││
│  │  20│╱                ╲               0│╱‾╲                  ╲    ││
│  │   0└────────────────────            └────────────────────       ││
│  │     12:00  14:00  16:00              12:00  14:00  16:00       ││
│  │                                                                  ││
│  │ Network I/O (KB/s)               vPod Count                     ││
│  │ 500│                              200│                          ││
│  │ 400│  ↑ Upload                    150│      ╱‾‾‾‾‾‾╲            ││
│  │ 300│  ↓ Download                  100│   ╱‾╲        ╲           ││
│  │ 200│                                50│╱‾╲            ╲╱‾╲       ││
│  │ 100│                                 0└────────────────────      ││
│  │   0└────────────────────            12:00  14:00  16:00        ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 🔥 Service Performance Metrics                                  ││
│  │                                                                  ││
│  │ Service              Requests/s  Latency(ms)  Error Rate  vPods ││
│  │ ─────────────────────────────────────────────────────────────  ││
│  │ BPI VM Server        125.3       12.5         0.01%       15   ││
│  │ HTTP Cage            89.7        8.3          0.00%       8    ││
│  │ Shadow Registry      45.2        15.7         0.02%       6    ││
│  │ ZKLock Auth          67.8        6.2          0.00%       5    ││
│  │ ENC Cluster          234.5       22.1         0.03%       25   ││
│  │ DockLock             156.9       18.4         0.01%       20   ││
│  │ Oracle Nodes         78.3        25.6         0.05%       12   ││
│  │ Storage Nodes        198.7       31.2         0.02%       18   ││
│  │ Logbook Nodes        92.4        14.8         0.01%       10   ││
│  │ Forensic Firewall    34.6        9.7          0.00%       8    ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 🎯 Quick Stats                                                  ││
│  │                                                                  ││
│  │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          ││
│  │ │ Total Req/s  │  │ Avg Latency  │  │ Success Rate │          ││
│  │ │ 1,123.4      │  │ 16.4ms       │  │ 99.98%       │          ││
│  │ └──────────────┘  └──────────────┘  └──────────────┘          ││
│  │                                                                  ││
│  │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          ││
│  │ │ Active vPods │  │ Memory Eff.  │  │ CPU Eff.     │          ││
│  │ │ 127 / 200    │  │ 34%          │  │ 80%          │          ││
│  │ └──────────────┘  └──────────────┘  └──────────────┘          ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  [📥 Export Data] [📊 Custom Dashboard] [🔔 Set Alerts] [⚙️ Config]│
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### **Features**:
- Real-time performance charts (CPU, Memory, Network, vPods)
- Service-level metrics table with requests/s, latency, error rates
- Time range selector and auto-refresh
- Quick stats summary
- Export data and custom dashboard options
- Alert configuration

---

## **Page 11: Wallet Settings & Security**

### **Route**: `/mojo-wallet/settings`

### **Purpose**: 
Comprehensive wallet settings including security, 2FA, recovery options, subscription management, and API access.

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ [☰] Wallet Settings      user@example.com    [🔔] [👤] [⚙️]         │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ⚙️ Mojo Wallet Settings                                            │
│                                                                      │
│  ┌─────────────────────┐                                            │
│  │ [🔐 Security]       │  ┌────────────────────────────────────────┐│
│  │  🔑 Authentication  │  │ 🔐 Security Settings                   ││
│  │  💳 Subscription    │  │                                        ││
│  │  🔌 API Access      │  │ Two-Factor Authentication (2FA)        ││
│  │  🔔 Notifications   │  │ Status: ✅ Enabled (Authenticator App) ││
│  │  👤 Profile         │  │                                        ││
│  │  🎨 Preferences     │  │ [Change 2FA Method] [Disable 2FA]      ││
│  └─────────────────────┘  │                                        ││
│                            │ Recovery Options                       ││
│                            │ ✅ Recovery Email: john@example.com    ││
│                            │ ✅ Backup Codes: 8 remaining           ││
│                            │                                        ││
│                            │ [Update Recovery Email]                ││
│                            │ [Generate New Backup Codes]            ││
│                            │                                        ││
│                            │ Password & Access                      ││
│                            │ Last Changed: 45 days ago              ││
│                            │                                        ││
│                            │ [Change Password]                      ││
│                            │ [View Active Sessions]                 ││
│                            │                                        ││
│                            │ Wallet Security                        ││
│                            │ BPI Address: bpi:wallet:abc123...      ││
│                            │ Access Token: ••••••••••••••••         ││
│                            │                                        ││
│                            │ [Regenerate Access Token]              ││
│                            │ [Export Wallet Backup]                 ││
│                            └────────────────────────────────────────┘│
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### **When clicking "Subscription" tab**:
```
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 💳 Subscription Management                                      ││
│  │                                                                  ││
│  │ Current Plan: Developer                                         ││
│  │ Price: 25 CAD/month                                             ││
│  │ BPI Balance: 2500 BPI                                           ││
│  │ Next Billing: November 27, 2025                                 ││
│  │                                                                  ││
│  │ Plan Comparison                                                 ││
│  │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          ││
│  │ │ Testnet      │  │ Developer ✓  │  │ Pilot        │          ││
│  │ │ 10 CAD/month │  │ 25 CAD/month │  │ 50 CAD/month │          ││
│  │ │ 1000 BPI     │  │ 2500 BPI     │  │ 5000 BPI     │          ││
│  │ │ BPI OS Lite  │  │ Full BPI OS  │  │ Full BPI OS  │          ││
│  │ │              │  │ + Support    │  │ + Priority   │          ││
│  │ │ [Downgrade]  │  │ [Current]    │  │ [Upgrade]    │          ││
│  │ └──────────────┘  └──────────────┘  └──────────────┘          ││
│  │                                                                  ││
│  │ [Top Up Balance] [View Billing History] [Cancel Subscription]   ││
│  └────────────────────────────────────────────────────────────────┘│
```

### **When clicking "API Access" tab**:
```
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 🔌 API Access & Webhooks                                        ││
│  │                                                                  ││
│  │ API Keys                                                        ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ Production Key                              [Regenerate] [✕] │││
│  │ │ pk_live_••••••••••••••••••••••••••••                         │││
│  │ │ Created: Oct 1, 2025  │  Last Used: 2 hours ago              │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ [+ Create New API Key]                                          ││
│  │                                                                  ││
│  │ Webhooks                                                        ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ Transaction Events                          [Edit] [Delete]  │││
│  │ │ https://myapp.com/webhooks/transactions                      │││
│  │ │ Events: transaction.created, transaction.confirmed           │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ [+ Add Webhook]                                                 ││
│  │                                                                  ││
│  │ API Documentation                                               ││
│  │ [📚 View API Docs] [💡 Code Examples] [🔧 Test API]            ││
│  └────────────────────────────────────────────────────────────────┘│
```

### **Features**:
- Tabbed interface for different settings categories
- 2FA management with multiple methods
- Recovery options (email, backup codes)
- Password and session management
- Wallet security (address, access token)
- Subscription management with plan comparison
- API key management
- Webhook configuration
- Profile and notification preferences

---

## **✅ Stage 2 Complete**

**Pages 7-11 Designed**:
7. ✅ Mojo Wallet Dashboard (Main interface with deployment CTA)
8. ✅ BPI OS Deployment Wizard (4-step: Check, Configure, Download, Deploy)
9. ✅ Node Management Dashboard (BSO-K8 cluster with vPod metrics)
10. ✅ Advanced Metrics & Monitoring (Grafana-style dashboard)
11. ✅ Wallet Settings & Security (Complete settings interface)

**Key Features Implemented**:
- ✅ BSO-K8 infrastructure (not Docker/Kubernetes)
- ✅ vPod technology with 100+ vPods in 2GB RAM
- ✅ Real-time metrics and monitoring
- ✅ Complete BPI OS service management
- ✅ 2GB RAM / 1 vCPU system requirements
- ✅ Comprehensive security and API access

**Next**: Stage 3 (Pages 12-15) - Advanced Features & Admin Panel

---

---

# Component 9: Complete Page Designs - Stage 3 (Pages 12-15)

**Stage 3**: Pages 12-15 (Advanced Features & Admin Panel)  
**Date**: 2025-10-27

---

## **📊 Stage 3 Page Overview**

### **Stage 3 Pages**:
12. **Transaction History & Explorer** (Blockchain transaction viewer)
13. **API Documentation & Testing** (Interactive API docs)
14. **Community Forum & Support** (Help center and community)
15. **Admin Panel** (For admin users - node management, user management)

---

## **Page 12: Transaction History & Explorer**

### **Route**: `/mojo-wallet/transactions`

### **Purpose**: 
Comprehensive transaction history viewer with blockchain explorer functionality, filters, and export options.

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ [☰] Transactions         user@example.com    [🔔] [👤] [⚙️]         │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  📊 Transaction History & Blockchain Explorer                       │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Filters                                                         ││
│  │                                                                  ││
│  │ Type: [All ▼]  Status: [All ▼]  Date: [Last 30 Days ▼]         ││
│  │ Search: [Transaction ID or Address...]              [Search]   ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Summary (Last 30 Days)                                          ││
│  │                                                                  ││
│  │ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          ││
│  │ │ Total Txns   │  │ Total Volume │  │ Avg Fee      │          ││
│  │ │ 1,247        │  │ 12,450 BPI   │  │ 0.5 BPI      │          ││
│  │ └──────────────┘  └──────────────┘  └──────────────┘          ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Recent Transactions                                             ││
│  │                                                                  ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ ✅ Confirmed                                    2 hours ago   │││
│  │ │ TX: 0x7a3f...b2e9                                            │││
│  │ │ From: bpi:wallet:abc123...                                   │││
│  │ │ To: bpi:wallet:def456...                                     │││
│  │ │ Amount: 100 BPI  │  Fee: 0.5 BPI  │  Block: #1,234,567      │││
│  │ │ [View Details] [Export]                                      │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ 🔄 Pending                                      5 hours ago   │││
│  │ │ TX: 0x9c2d...f4a1                                            │││
│  │ │ From: bpi:wallet:abc123...                                   │││
│  │ │ To: bpi:wallet:ghi789...                                     │││
│  │ │ Amount: 50 BPI  │  Fee: 0.5 BPI  │  Confirmations: 2/6      │││
│  │ │ [View Details] [Export]                                      │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ ✅ Confirmed                                    1 day ago     │││
│  │ │ TX: 0x4e8b...c3d7                                            │││
│  │ │ From: bpi:wallet:jkl012...                                   │││
│  │ │ To: bpi:wallet:abc123...                                     │││
│  │ │ Amount: 200 BPI  │  Fee: 0.5 BPI  │  Block: #1,234,123      │││
│  │ │ [View Details] [Export]                                      │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ [Load More] [Export All] [Subscribe to Alerts]                 ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### **Transaction Detail Modal**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ Transaction Details                                           [✕]   │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Transaction ID: 0x7a3f2b8c9d1e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2│
│                                                                      │
│  Status: ✅ Confirmed (6/6 confirmations)                           │
│  Timestamp: 2025-10-27 01:15:23 UTC                                 │
│  Block: #1,234,567                                                  │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Transaction Details                                             ││
│  │                                                                  ││
│  │ From: bpi:wallet:abc123def456ghi789                             ││
│  │       (Your Wallet)                                             ││
│  │                                                                  ││
│  │ To: bpi:wallet:def456ghi789jkl012                               ││
│  │     (External Wallet)                                           ││
│  │                                                                  ││
│  │ Amount: 100 BPI                                                 ││
│  │ Fee: 0.5 BPI                                                    ││
│  │ Total: 100.5 BPI                                                ││
│  │                                                                  ││
│  │ Gas Used: 21,000                                                ││
│  │ Gas Price: 23.8 Gwei                                            ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Blockchain Information                                          ││
│  │                                                                  ││
│  │ Block Hash: 0x1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5│
│  │ Block Height: #1,234,567                                        ││
│  │ Confirmations: 6                                                ││
│  │ Network: BPI Mainnet                                            ││
│  │                                                                  ││
│  │ Proof Type: PoE (Proof-of-Execution)                            ││
│  │ Validator: validator-node-42                                    ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  [📥 Export JSON] [📋 Copy TX ID] [🔗 View on Explorer] [Close]    │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### **Features**:
- Transaction list with filters (type, status, date)
- Search by transaction ID or address
- Summary statistics
- Transaction status indicators (Confirmed, Pending, Failed)
- Detailed transaction modal with blockchain info
- Export functionality (JSON, CSV)
- Real-time updates for pending transactions

---

## **Page 13: API Documentation & Testing**

### **Route**: `/mojo-wallet/api-docs`

### **Purpose**: 
Interactive API documentation with code examples, testing playground, and authentication guides.

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ [☰] API Documentation    user@example.com    [🔔] [👤] [⚙️]         │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  📚 BPI OS API Documentation                                        │
│                                                                      │
│  ┌─────────────────────┐                                            │
│  │ [📖 Overview]       │  ┌────────────────────────────────────────┐│
│  │  🔐 Authentication  │  │ 📖 API Overview                        ││
│  │  💼 Wallet API      │  │                                        ││
│  │  📊 Transactions    │  │ The BPI OS API provides programmatic   ││
│  │  🔧 Node Management │  │ access to your deployed BPI Immutable  ││
│  │  📈 Metrics         │  │ OS infrastructure.                     ││
│  │  🔔 Webhooks        │  │                                        ││
│  │  💡 Code Examples   │  │ Base URL:                              ││
│  │  🧪 API Playground  │  │ https://api.bpi.pravyom.com/v1         ││
│  └─────────────────────┘  │                                        ││
│                            │ Authentication: API Key (Bearer Token) ││
│                            │                                        ││
│                            │ Rate Limits:                           ││
│                            │ • Testnet: 100 req/min                 ││
│                            │ • Developer: 1000 req/min              ││
│                            │ • Pilot: 5000 req/min                  ││
│                            │                                        ││
│                            │ Quick Start:                           ││
│                            │ ```bash                                ││
│                            │ curl -H "Authorization: Bearer $KEY" \ ││
│                            │   https://api.bpi.pravyom.com/v1/wallet││
│                            │ ```                                    ││
│                            └────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────┘
```

### **When clicking "Wallet API" section**:
```
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 💼 Wallet API                                                   ││
│  │                                                                  ││
│  │ GET /v1/wallet                                                  ││
│  │ Get wallet information                                          ││
│  │                                                                  ││
│  │ Request:                                                        ││
│  │ ```bash                                                         ││
│  │ curl -X GET \                                                   ││
│  │   -H "Authorization: Bearer YOUR_API_KEY" \                     ││
│  │   https://api.bpi.pravyom.com/v1/wallet                         ││
│  │ ```                                                             ││
│  │                                                                  ││
│  │ Response (200 OK):                                              ││
│  │ ```json                                                         ││
│  │ {                                                               ││
│  │   "address": "bpi:wallet:abc123def456ghi789",                   ││
│  │   "balance": 2500,                                              ││
│  │   "plan": "developer",                                          ││
│  │   "status": "active"                                            ││
│  │ }                                                               ││
│  │ ```                                                             ││
│  │                                                                  ││
│  │ [▶ Try it out]                                                  ││
│  │                                                                  ││
│  │ ─────────────────────────────────────────────────────────────  ││
│  │                                                                  ││
│  │ POST /v1/wallet/transfer                                        ││
│  │ Transfer BPI to another wallet                                  ││
│  │                                                                  ││
│  │ Request Body:                                                   ││
│  │ ```json                                                         ││
│  │ {                                                               ││
│  │   "to": "bpi:wallet:def456ghi789jkl012",                        ││
│  │   "amount": 100,                                                ││
│  │   "memo": "Payment for services"                                ││
│  │ }                                                               ││
│  │ ```                                                             ││
│  │                                                                  ││
│  │ [▶ Try it out]                                                  ││
│  └────────────────────────────────────────────────────────────────┘│
```

### **API Playground (Try it out)**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ 🧪 API Playground - GET /v1/wallet                           [✕]   │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Request                                                             │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Method: GET                                                     ││
│  │ URL: https://api.bpi.pravyom.com/v1/wallet                      ││
│  │                                                                  ││
│  │ Headers:                                                        ││
│  │ Authorization: Bearer pk_live_••••••••••••••••••••              ││
│  │ Content-Type: application/json                                  ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  [🚀 Send Request]                                                  │
│                                                                      │
│  Response                                                            │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ Status: 200 OK                                                  ││
│  │ Time: 145ms                                                     ││
│  │                                                                  ││
│  │ ```json                                                         ││
│  │ {                                                               ││
│  │   "address": "bpi:wallet:abc123def456ghi789",                   ││
│  │   "balance": 2500,                                              ││
│  │   "plan": "developer",                                          ││
│  │   "status": "active",                                           ││
│  │   "created_at": "2025-10-01T12:00:00Z",                         ││
│  │   "last_activity": "2025-10-27T01:15:23Z"                       ││
│  │ }                                                               ││
│  │ ```                                                             ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  [📋 Copy Response] [📥 Export] [Close]                             │
└─────────────────────────────────────────────────────────────────────┘
```

### **Features**:
- Comprehensive API documentation with all endpoints
- Interactive code examples (curl, JavaScript, Python, Rust)
- API playground for testing requests
- Authentication guides
- Rate limit information
- Webhook documentation
- Response schema documentation
- Error code reference

---

## **Page 14: Community Forum & Support**

### **Route**: `/community`

### **Purpose**: 
Community help center with forum, documentation, tutorials, and support ticket system.

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ [☰] Community & Support  user@example.com    [🔔] [👤] [⚙️]         │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  💬 Community & Support Center                                      │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 🔍 Search: [How to deploy BPI OS...]              [Search]     ││
│  └────────────────────────────────────────────────────────────────┘│
│                                                                      │
│  ┌─────────────────────┐                                            │
│  │ [📚 Documentation]  │  ┌────────────────────────────────────────┐│
│  │  💬 Forum           │  │ 📚 Popular Documentation               ││
│  │  🎓 Tutorials       │  │                                        ││
│  │  🎫 Support Tickets │  │ ┌────────────────────────────────────┐││
│  │  ❓ FAQ             │  │ │ 🚀 Getting Started Guide           │││
│  │  📹 Video Guides    │  │ │ Learn how to deploy your first BPI │││
│  └─────────────────────┘  │ │ OS node in 10 minutes.             │││
│                            │ │ [Read More →]                      │││
│                            │ └────────────────────────────────────┘││
│                            │                                        ││
│                            │ ┌────────────────────────────────────┐││
│                            │ │ 🔧 BSO-K8 Configuration Guide      │││
│                            │ │ Advanced configuration options for │││
│                            │ │ BSO-K8 orchestration.              │││
│                            │ │ [Read More →]                      │││
│                            │ └────────────────────────────────────┘││
│                            │                                        ││
│                            │ ┌────────────────────────────────────┐││
│                            │ │ 💡 vPod Technology Explained       │││
│                            │ │ Understanding ultra-lightweight    │││
│                            │ │ vPod architecture.                 │││
│                            │ │ [Read More →]                      │││
│                            │ └────────────────────────────────────┘││
│                            │                                        ││
│                            │ [View All Documentation →]            ││
│                            └────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────┘
```

### **Forum View**:
```
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 💬 Community Forum                                              ││
│  │                                                                  ││
│  │ Categories:                                                     ││
│  │ [All] [General] [Deployment] [Development] [Troubleshooting]   ││
│  │                                                                  ││
│  │ [+ New Discussion]                                              ││
│  │                                                                  ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ 🔥 How to optimize vPod performance?                         │││
│  │ │ by @alice_dev • 2 hours ago • 12 replies • Development       │││
│  │ │ Looking for tips to optimize vPod performance in production...│││
│  │ │ [View Discussion →]                                          │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ ❓ BSO-K8 deployment failing on Ubuntu 22.04                 │││
│  │ │ by @bob_ops • 5 hours ago • 8 replies • Troubleshooting      │││
│  │ │ Getting error during BSO-K8 initialization. Any ideas?        │││
│  │ │ [View Discussion →]                                          │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ 💡 Sharing my BPI OS deployment on Raspberry Pi              │││
│  │ │ by @charlie_pi • 1 day ago • 24 replies • Deployment         │││
│  │ │ Successfully deployed BPI OS on Raspberry Pi 4 (4GB RAM)...  │││
│  │ │ [View Discussion →]                                          │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ [Load More Discussions]                                         ││
│  └────────────────────────────────────────────────────────────────┘│
```

### **Support Ticket View**:
```
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 🎫 Support Tickets                                              ││
│  │                                                                  ││
│  │ [+ Create New Ticket]                                           ││
│  │                                                                  ││
│  │ Your Tickets:                                                   ││
│  │                                                                  ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ 🟢 Open                                        Ticket #1247   │││
│  │ │ Node deployment stuck at 67%                                 │││
│  │ │ Created: 2 hours ago  │  Last Update: 30 min ago             │││
│  │ │ Priority: High  │  Category: Deployment                      │││
│  │ │ [View Ticket →]                                              │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ ✅ Resolved                                    Ticket #1198   │││
│  │ │ API rate limit question                                      │││
│  │ │ Created: 3 days ago  │  Resolved: 2 days ago                │││
│  │ │ Priority: Low  │  Category: API                              │││
│  │ │ [View Ticket →]                                              │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ Response Time:                                                  ││
│  │ • Priority High: < 2 hours                                      ││
│  │ • Priority Normal: < 24 hours                                   ││
│  │ • Priority Low: < 48 hours                                      ││
│  └────────────────────────────────────────────────────────────────┘│
```

### **Features**:
- Searchable documentation library
- Community forum with categories
- Tutorial library with video guides
- Support ticket system with priority levels
- FAQ section
- Real-time chat support (for Pilot plan)
- Knowledge base articles
- Community voting and best answers

---

## **Page 15: Admin Panel**

### **Route**: `/admin` (Admin users only)

### **Purpose**: 
Administrative interface for managing users, nodes, system health, and platform configuration.

### **Access Control**: 
Only accessible to users with `admin` role in Keycloak.

### **Wireframe**:
```
┌─────────────────────────────────────────────────────────────────────┐
│ [☰] Admin Panel          admin@pravyom.com   [🔔] [👤] [⚙️]         │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  🛡️ BPCI Enterprise Admin Panel                                     │
│                                                                      │
│  ┌─────────────────────┐                                            │
│  │ [📊 Dashboard]      │  ┌────────────────────────────────────────┐│
│  │  👥 Users           │  │ 📊 Platform Overview                   ││
│  │  🖥️ Nodes           │  │                                        ││
│  │  💼 Subscriptions   │  │ ┌──────────┐  ┌──────────┐  ┌────────┐││
│  │  📈 Analytics       │  │ │ Users    │  │ Nodes    │  │ Revenue│││
│  │  ⚙️ Settings        │  │ │ 1,247    │  │ 892      │  │ $45.2K │││
│  │  🔐 Security        │  │ │ +12 (24h)│  │ +8 (24h) │  │ +$2.1K │││
│  │  📝 Audit Logs      │  │ └──────────┘  └──────────┘  └────────┘││
│  └─────────────────────┘  │                                        ││
│                            │ ┌──────────────────────────────────────┐││
│                            │ │ System Health                        │││
│                            │ │ ✅ All Services Operational          │││
│                            │ │                                      │││
│                            │ │ API: ✅ 99.98% uptime                │││
│                            │ │ Database: ✅ Healthy                 │││
│                            │ │ BSO-K8 Cluster: ✅ 892 nodes active  │││
│                            │ │ Storage: ⚠️ 78% capacity             │││
│                            │ └──────────────────────────────────────┘││
│                            │                                        ││
│                            │ Recent Activity:                       ││
│                            │ • New user registration: alice@ex.com  ││
│                            │ • Node deployed: node-1248             ││
│                            │ • Subscription upgraded: bob@ex.com    ││
│                            │ • Support ticket created: #1247        ││
│                            └────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────┘
```

### **User Management View**:
```
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 👥 User Management                                              ││
│  │                                                                  ││
│  │ Search: [Email or username...]  [Search]  [+ Add User]         ││
│  │                                                                  ││
│  │ Filters: [All Users ▼] [All Plans ▼] [All Status ▼]            ││
│  │                                                                  ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ User                Plan        Status    Actions            │││
│  │ │ ──────────────────────────────────────────────────────────  │││
│  │ │ alice@example.com   Developer   ✅ Active  [Edit] [Suspend] │││
│  │ │ Joined: Oct 1, 2025 │ Node: Yes │ Last: 2h ago              │││
│  │ │                                                              │││
│  │ │ bob@example.com     Pilot       ✅ Active  [Edit] [Suspend] │││
│  │ │ Joined: Sep 15, 2025│ Node: Yes │ Last: 5h ago              │││
│  │ │                                                              │││
│  │ │ charlie@example.com Testnet     ⏸️ Suspended [Edit] [Activate]│││
│  │ │ Joined: Aug 20, 2025│ Node: No  │ Last: 15d ago             │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ [Export Users] [Bulk Actions] [View Analytics]                 ││
│  └────────────────────────────────────────────────────────────────┘│
```

### **Node Management View**:
```
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 🖥️ Node Management                                              ││
│  │                                                                  ││
│  │ Total Nodes: 892  │  Active: 867  │  Stopped: 15  │  Error: 10 ││
│  │                                                                  ││
│  │ Search: [Node ID or owner...]  [Search]                        ││
│  │                                                                  ││
│  │ ┌──────────────────────────────────────────────────────────────┐││
│  │ │ Node ID         Owner           Status    Resources  Actions │││
│  │ │ ──────────────────────────────────────────────────────────  │││
│  │ │ node-1248       alice@ex.com    ✅ Running 687MB/2GB [Manage]│││
│  │ │ Uptime: 2h 34m  │ vPods: 127/200│ CPU: 80%                  │││
│  │ │                                                              │││
│  │ │ node-1247       bob@ex.com      ✅ Running 1.2GB/4GB [Manage]│││
│  │ │ Uptime: 15h 12m │ vPods: 189/400│ CPU: 65%                  │││
│  │ │                                                              │││
│  │ │ node-1246       charlie@ex.com  ❌ Error   N/A       [Debug] │││
│  │ │ Error: BSO-K8 initialization failed                          │││
│  │ └──────────────────────────────────────────────────────────────┘││
│  │                                                                  ││
│  │ [Export Nodes] [Bulk Actions] [System Health]                  ││
│  └────────────────────────────────────────────────────────────────┘│
```

### **Analytics View**:
```
│  ┌────────────────────────────────────────────────────────────────┐│
│  │ 📈 Platform Analytics                                           ││
│  │                                                                  ││
│  │ Time Range: [Last 30 Days ▼]                                   ││
│  │                                                                  ││
│  │ User Growth                      Revenue                        ││
│  │ 1500│                            $50K│                          ││
│  │ 1200│      ╱‾‾‾╲                 $40K│        ╱‾‾╲              ││
│  │  900│   ╱‾╲     ╲                $30K│     ╱‾╲    ╲             ││
│  │  600│╱‾╲          ╲               $20K│  ╱‾╲        ╲            ││
│  │  300│              ╲              $10K│╱‾╲            ╲          ││
│  │    0└────────────────            $0└────────────────            ││
│  │                                                                  ││
│  │ Node Deployments                 API Usage                      ││
│  │ 1000│                            500K│                          ││
│  │  800│     ╱‾‾╲                   400K│      ╱‾‾‾╲               ││
│  │  600│  ╱‾╲    ╲                  300K│   ╱‾╲     ╲              ││
│  │  400│╱‾╲       ╲                 200K│╱‾╲         ╲             ││
│  │  200│           ╲                100K│             ╲            ││
│  │    0└────────────────              0└────────────────           ││
│  │                                                                  ││
│  │ [Export Report] [Custom Dashboard] [Schedule Email]            ││
│  └────────────────────────────────────────────────────────────────┘│
```

### **Features**:
- Platform overview dashboard with key metrics
- User management (view, edit, suspend, activate)
- Node management with health monitoring
- Subscription management and billing
- Platform analytics with charts
- Security settings and audit logs
- System configuration
- Bulk actions for users and nodes
- Export functionality for reports

### **Access Control**:
```typescript
// Admin route protection
if (!user.roles.includes('admin')) {
  redirect('/dashboard');
}
```

---

## **✅ Stage 3 Complete**

**Pages 12-15 Designed**:
12. ✅ Transaction History & Explorer (Blockchain viewer with filters)
13. ✅ API Documentation & Testing (Interactive docs with playground)
14. ✅ Community Forum & Support (Help center, forum, tickets)
15. ✅ Admin Panel (User/node management, analytics)

**All 15 Pages Complete!**

### **Summary of All Pages**:

**Stage 1 (Pages 1-6)**: Authentication & Basic Features
1. Keycloak Login
2. Basic Dashboard
3. Dual-Auth Wizard - Step 1 (Generate)
4. Dual-Auth Wizard - Step 2 (Bind)
5. Dual-Auth Wizard - Step 3 (Harden)
6. Community Posting Form

**Stage 2 (Pages 7-11)**: Mojo Wallet & Node Management
7. Mojo Wallet Dashboard
8. BPI OS Deployment Wizard (BSO-K8)
9. Node Management Dashboard (vPod metrics)
10. Advanced Metrics & Monitoring
11. Wallet Settings & Security

**Stage 3 (Pages 12-15)**: Advanced Features & Admin
12. Transaction History & Explorer
13. API Documentation & Testing
14. Community Forum & Support
15. Admin Panel

**Key Technologies Highlighted**:
- ✅ BSO-K8 (Binary Saturated OSI Kubernetes) - Not Docker/K8s
- ✅ vPod Technology - 100+ vPods in 2GB RAM
- ✅ 2GB RAM / 1 vCPU minimum requirements
- ✅ Complete BPI Immutable OS with 10 services
- ✅ Dual authentication (Keycloak + BPI Wallet)
- ✅ Real-time metrics and monitoring
- ✅ API access and webhooks
- ✅ Community and support features

**Ready for Implementation!** 🚀
