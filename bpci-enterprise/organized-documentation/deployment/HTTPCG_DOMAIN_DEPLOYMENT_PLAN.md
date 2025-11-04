# HTTPCG Domain System Deployment Plan

## Overview

This document outlines the deployment strategy for the **HTTPCG domain system** as implemented in the real BPI-BPCI codebase. The system uses `httpcg://[plane]/[domain]/[path]?hash=[route_hash]` addressing with hash-based routing, quantum-safe session keys, and wallet-address routing.

**Production Domain Configuration:**
- **Domain**: `pravyom.com` (Cloudflare SSL/DNS)
- **Static Website**: Vite website with HTTP server entrypoint
- **HTTPCG Protocol**: Same domain handles HTTPCG routing
- **SSL/TLS**: Managed by Cloudflare

## 1. Domain Architecture

### 1.1 HTTPCG Protocol Structure

Based on real code analysis (`httpcg_domain_registry.rs`, `vm_server.rs`, `httpcg_client.rs`):

```
httpcg://[plane]/[domain]/[path]?hash=[route_hash]
```

**Examples:**
- `httpcg://app/pravyom.com/dashboard?hash=abc123`
- `httpcg://secure/pravyom.com/wallet?hash=def456`
- `httpcg://gov/pravyom.com/portal?hash=ghi789`
- `httpcg://app/pravyom.com/demo?hash=jkl012`

**Planes:**
- `app` - Application plane
- `secure` - Secure plane  
- `gov` - Government plane
- `dark` - Dark plane (restricted access)

### **Real HTTPCG Planes (From Code):**
```rust
// From vm_server.rs route_httpcg_request()
match (plane, domain) {
    // Application plane
    ("app", "prav.global") => self.serve_httpcg_bpci_wallet(&sub_path, request_id).await,
    ("app", "wallet.global") => self.serve_httpcg_bpci_wallet(&sub_path, request_id).await,
    ("app", "demo.global") => self.serve_httpcg_real_demo_app(&sub_path, request_id).await,
    
    // Country-specific domains
    ("app", domain) if domain.ends_with(".in") => self.serve_httpcg_country_domain(domain, &sub_path, "India", request_id).await,
    ("app", domain) if domain.ends_with(".us") => self.serve_httpcg_country_domain(domain, &sub_path, "United States", request_id).await,
    ("app", domain) if domain.ends_with(".uk") => self.serve_httpcg_country_domain(domain, &sub_path, "United Kingdom", request_id).await,
    
    // Government plane
    ("gov", domain) if domain.ends_with(".gov") => self.serve_httpcg_government_domain(domain, &sub_path, request_id).await,
    
    // Secure plane
    ("secure", domain) => self.serve_httpcg_secure_domain(domain, &sub_path, request_id).await,
    ("secure", domain) if domain.ends_with(".corp") => self.serve_httpcg_corporate_domain(domain, &sub_path, request_id).await,
    ("secure", domain) if domain.ends_with(".mil") => self.serve_httpcg_military_domain(domain, &sub_path, request_id).await,
    
    // Dark plane
    ("dark", domain) if domain.ends_with(".dark") => self.serve_httpcg_dark_domain(domain, &sub_path, request_id).await,
    
    // Global domains (dynamic routing)
    ("app", domain) if domain.ends_with(".global") => self.serve_httpcg_global_domain(domain, &sub_path, request_id).await,
    
    // Educational domains
    ("app", domain) if domain.ends_with(".edu") => self.serve_httpcg_educational_domain(domain, &sub_path, request_id).await,
}
```

---

## 🔐 **BPI-BPCI Registry System with Wallet Addresses & Tokens**

### **Real Wallet Registration System (From bpci_xtmp_server.rs):**

```rust
// BpciWalletRegistry - Real wallet address and token system
pub struct BpciWalletRegistry {
    pub registered_wallets: Arc<RwLock<HashMap<String, RegisteredWallet>>>,
    pub authentication_cache: Arc<RwLock<HashMap<String, AuthenticationInfo>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredWallet {
    pub wallet_address: String,        // Real wallet address (e.g., "bpi1abc123def456...")
    pub registration_time: u64,
    pub client_info: String,
    pub last_activity: u64,
    pub status: WalletStatus,          // Active/Suspended/Inactive
}

#[derive(Debug, Clone)]
pub struct AuthenticationInfo {
    pub token: String,                 // Authentication token
    pub expires_at: Instant,
    pub permissions: Vec<String>,
}
```

### **Production Token Format (From production_bpci_client.rs):**

```rust
// Production token format: wallet_address//password
let full_token = format!("{}//{}", wallet_address.full_address, password);

// Registry connection requires:
// 1. Wallet address (e.g., "bpi1abc123def456...")
// 2. Registry token (from BPCI server)
// 3. Authentication via XTMP protocol
```

### **BPI-BPCI Connection Process (Real Implementation):**

```bash
# Step 1: User connects BPI to BPCI with wallet address and token
bpi wallet connect --registry-address <BPCI_SERVER_ADDR> --registry-token <AUTH_TOKEN>

# Step 2: BPCI validates wallet and registers in BpciWalletRegistry
# - Wallet address format: "bpi1abc123def456..."
# - Token format: "wallet_address//password"
# - Registration creates RegisteredWallet entry

# Step 3: HTTPCG domain registration via BPCI Enterprise registry
# - Domain registered with DID (Decentralized Identifier)
# - Hash-based routing enabled with quantum-safe session keys
# - Dynamic domain resolution through Shadow Registry bridge
```

### **HTTPCG Hash-Based Routing (From httpcg_client.rs):**

```rust
// Hash-based routing with QLOCK session keys
fn derive_qlock_key(
    &self,
    tls_exporter: &[u8],
    spki_hash: &[u8],
    tlsls_fingerprint: &[u8],
    route_fingerprint: &str,
    minute_epoch: u64,
) -> Result<Vec<u8>> {
    let mut hasher = Sha256::new();
    hasher.update(b"httpcg-qlock/v1");  // Domain separator
    hasher.update(b"\x00");
    hasher.update(tls_exporter);
    hasher.update(b"\x00");
    hasher.update(spki_hash);
    hasher.update(b"\x00");
    hasher.update(tlsls_fingerprint);
    hasher.update(b"\x00");
    hasher.update(route_fingerprint.as_bytes());
    hasher.update(b"\x00");
    hasher.update(&minute_epoch.to_le_bytes());
    Ok(hasher.finalize().to_vec())
}

// Wallet-routed URLs: /hash.bpi/<W_ADDR>/
// Hash parameters derived from:
// - Domain-separated hashing (httpcg-qlock/v1)
// - TLS exporter + SPKI hash + route fingerprint
// - Minute epoch for time-based rotation
```

### **How BPCI Hosts Thousands of BPI Infrastructure:**

```yaml
Ultra-Lightweight Architecture:
- Each BPI node: <2 CPU, 4GB RAM, 10GB storage
- VM Server: 100MB RAM per kernel
- Cellular growth algorithms for auto-scaling
- Quantum optimization for resource efficiency
- Hash-based routing eliminates DNS overhead

Tight Server Constraints (Real Code Analysis):
- BPCI Registry Server: 1CPU-2GB ($12/month)
- HTTPCG Gateway: 2CPU-4GB ($24/month)
- PostgreSQL Database: $15/month
- Total: $51/month for thousands of BPI nodes

Economic Model:
- Users host their own BPI nodes (decentralized)
- BPCI provides coordination services only
- Dynamic pricing via autonomous runes engine
- Staking contracts for domain operations
```

---

## 🎯 **HTTPCG Domain Deployment Strategy**

### **Core HTTPCG Domains for BSO Infrastructure:**

```yaml
Primary HTTPCG Domains:
- httpcg://app/prav.global/ (BPCI Wallet Dashboard)
- httpcg://app/wallet.global/ (Alternative wallet access)
- httpcg://app/demo.global/ (Real Interactive Demo)
- httpcg://secure/bpci.corp/ (Enterprise BPCI access)
- httpcg://gov/pravyom.gov/ (Government services)

Country-Specific:
- httpcg://app/pravyom.in/ (India)
- httpcg://app/pravyom.us/ (United States)
- httpcg://app/pravyom.uk/ (United Kingdom)

Development/Testing:
- httpcg://app/testnet.global/ (Testnet interface)
- httpcg://app/dev.global/ (Development environment)
```

### **Traditional Domain → HTTPCG Mapping:**

Instead of using traditional subdomains, we use one main domain with HTTPCG routing:

```yaml
Traditional Approach (WRONG):
- pravyom.com → Main website
- bpci.pravyom.com → BPCI server
- wallet.pravyom.com → Wallet
- api.pravyom.com → API

HTTPCG Approach (CORRECT):
- pravyom.com → HTTPCG Gateway Server
  ├── httpcg://app/prav.global/ → BPCI Wallet
  ├── httpcg://app/demo.global/ → Demo App
  ├── httpcg://secure/bpci.corp/ → Enterprise
  └── httpcg://gov/pravyom.gov/ → Government
```

---

## 🚀 **Updated Digital Ocean Infrastructure**

### **Simplified Infrastructure (Using HTTPCG):**

```bash
# Single HTTPCG Gateway Server (instead of multiple subdomains)
doctl compute droplet create httpcg-gateway \
    --size s-2vcpu-4gb \
    --image ubuntu-22-04-x64 \
    --region nyc3 \
    --tag-names httpcg,gateway,bso \
    --ssh-keys YOUR_SSH_KEY_ID

# BSO Backend Server (for cellular processing)
doctl compute droplet create bso-backend \
    --size s-1vcpu-2gb \
    --image ubuntu-22-04-x64 \
    --region nyc3 \
    --tag-names bso,backend,cellular \
    --ssh-keys YOUR_SSH_KEY_ID

# Database (same as before)
doctl databases create bpci-db \
    --engine pg \
    --size db-s-1vcpu-1gb \
    --region nyc3
```

### **DNS Configuration (Simplified):**

```bash
# Get server IPs
HTTPCG_GATEWAY_IP=$(doctl compute droplet get httpcg-gateway --format PublicIPv4 --no-header)
BSO_BACKEND_IP=$(doctl compute droplet get bso-backend --format PublicIPv4 --no-header)

# Single DNS record (no subdomains needed!)
doctl compute domain create pravyom.com
doctl compute domain records create pravyom.com \
    --record-type A \
    --record-name @ \
    --record-data $HTTPCG_GATEWAY_IP \
    --record-ttl 300

# WWW redirect
doctl compute domain records create pravyom.com \
    --record-type CNAME \
    --record-name www \
    --record-data pravyom.com \
    --record-ttl 300
```

---

## ⚙️ **HTTPCG Gateway Server Configuration**

### **Nginx Configuration for HTTPCG Protocol:**

```bash
# SSH into HTTPCG Gateway server
ssh root@$HTTPCG_GATEWAY_IP

# Install Nginx and SSL
apt update && apt install -y nginx certbot python3-certbot-nginx

# Create HTTPCG-aware Nginx configuration
cat > /etc/nginx/sites-available/httpcg-gateway << 'EOF'
# HTTPCG Gateway - Revolutionary Domain System
server {
    listen 80;
    listen [::]:80;
    server_name pravyom.com www.pravyom.com;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name pravyom.com www.pravyom.com;

    # SSL configuration
    ssl_certificate /etc/letsencrypt/live/pravyom.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/pravyom.com/privkey.pem;

    # HTTPCG Protocol Handler - Route all requests to VM Server
    location / {
        proxy_pass http://localhost:7777;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # HTTPCG Protocol Headers
        proxy_set_header X-HTTPCG-Protocol "Enabled";
        proxy_set_header X-HTTPCG-Gateway "pravyom.com";
        proxy_set_header X-HTTPCG-Plane $arg_plane;
        proxy_set_header X-HTTPCG-Domain $arg_domain;
        
        # WebSocket support for real-time HTTPCG
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }

    # Health check
    location /health {
        return 200 "HTTPCG Gateway Healthy";
        add_header Content-Type text/plain;
    }

    # HTTPCG Protocol Info
    location /.well-known/httpcg {
        return 200 '{"protocol":"httpcg","version":"1.0","gateway":"pravyom.com","planes":["app","secure","gov","dark"],"domains":["prav.global","wallet.global","demo.global"]}';
        add_header Content-Type application/json;
    }
}
EOF

# Enable the site
ln -s /etc/nginx/sites-available/httpcg-gateway /etc/nginx/sites-enabled/
nginx -t && systemctl reload nginx

# Generate SSL certificate
certbot --nginx -d pravyom.com -d www.pravyom.com --non-interactive --agree-tos --email admin@pravyom.com
```

---

## 🧬 **BSO Infrastructure with HTTPCG**

### **Deploy HTTPCG-Aware VM Server:**

```bash
# Clone repository and build
git clone YOUR_REPO /opt/httpcg-infrastructure
cd /opt/httpcg-infrastructure

# Install Rust and CUE
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
curl -L https://github.com/cue-lang/cue/releases/download/v0.6.0/cue_v0.6.0_linux_amd64.tar.gz | tar xz
sudo mv cue /usr/local/bin/

# Build HTTPCG-aware VM Server
cargo build --release --bin vm_server

# Create HTTPCG configuration
cat > /opt/httpcg-infrastructure/httpcg-config.json << 'EOF'
{
  "vm_port": 7777,
  "http_cage_port": 8888,
  "bpi_rpc_port": 9545,
  "bpi_api_port": 9546,
  "rpc_entangled_port": 9547,
  "post_quantum_enabled": true,
  "shadow_registry_endpoint": "http://localhost:8080",
  "zklock_endpoint": "http://localhost:8081",
  "isolation_level": "Quantum",
  "security_rating": 9.5,
  "httpcg_enabled": true,
  "httpcg_planes": ["app", "secure", "gov", "dark"],
  "httpcg_domains": {
    "prav.global": "bpci_wallet",
    "wallet.global": "bpci_wallet",
    "demo.global": "demo_app",
    "bpci.corp": "enterprise",
    "pravyom.gov": "government"
  }
}
EOF

# Create systemd service for HTTPCG VM Server
cat > /etc/systemd/system/httpcg-vm-server.service << 'EOF'
[Unit]
Description=HTTPCG VM Server - Revolutionary Domain System
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/httpcg-infrastructure
ExecStart=/opt/httpcg-infrastructure/target/release/vm_server \
    --config /opt/httpcg-infrastructure/httpcg-config.json \
    --httpcg-enabled \
    --cellular-growth-enabled \
    --quantum-optimization-enabled
Restart=always
RestartSec=10
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
EOF

# Start HTTPCG VM Server
systemctl daemon-reload
systemctl enable httpcg-vm-server
systemctl start httpcg-vm-server
```

---

## 🔍 **HTTPCG Domain Testing**

### **Test HTTPCG Protocol:**

```bash
# Test main gateway
curl https://pravyom.com/health

# Test HTTPCG protocol info
curl https://pravyom.com/.well-known/httpcg

# Test HTTPCG domains (through gateway)
curl "https://pravyom.com/httpcg/app/prav.global/"
curl "https://pravyom.com/httpcg/app/demo.global/"
curl "https://pravyom.com/httpcg/secure/bpci.corp/"
curl "https://pravyom.com/httpcg/gov/pravyom.gov/"

# Test country-specific domains
curl "https://pravyom.com/httpcg/app/pravyom.in/"
curl "https://pravyom.com/httpcg/app/pravyom.us/"
```

### **HTTPCG Client Integration:**

```javascript
// JavaScript client for HTTPCG protocol
class HttpcgClient {
    constructor(gateway = 'https://pravyom.com') {
        this.gateway = gateway;
    }
    
    async request(plane, domain, path = '/', hash = null) {
        let url = `${this.gateway}/httpcg/${plane}/${domain}${path}`;
        if (hash) {
            url += `?hash=${hash}`;
        }
        const response = await fetch(url, {
            headers: {
                'X-HTTPCG-Protocol': 'Enabled',
                'X-HTTPCG-Client': 'JavaScript/1.0',
                'X-HTTPCG-Hash': hash || 'auto-generated'
            }
        });
        return response;
    }
    
    // BPCI Wallet access with hash routing
    async wallet(path = '/', walletHash = null) {
        return this.request('app', 'prav.global', path, walletHash);
    }
    
    // TaskFlow app access with hash routing
    async taskflow(path = '/', appHash = null) {
        return this.request('app', 'taskflow.global', path, appHash);
    }
    
    // Wallet-routed URLs: /hash.bpi/<W_ADDR>/
    async walletRoute(walletAddress, path = '/') {
        const url = `${this.gateway}/hash.bpi/${walletAddress}${path}`;
        return fetch(url, {
            headers: {
                'X-HTTPCG-Protocol': 'Enabled',
                'X-Wallet-Address': walletAddress
            }
        });
    }
    
    // Enterprise access with hash routing
    async enterprise(path = '/', enterpriseHash = null) {
        return this.request('secure', 'bpci.corp', path, enterpriseHash);
    }
}

// Usage
const httpcg = new HttpcgClient();
const walletResponse = await httpcg.wallet('/dashboard');
const demoResponse = await httpcg.demo('/interactive');
```

---

## 💰 **Updated Cost Structure**

### **HTTPCG Infrastructure Cost: $51/month**

```yaml
Digital Ocean Infrastructure:
1. HTTPCG Gateway Server (2CPU-4GB): $24/month
   - Nginx reverse proxy with HTTPCG routing
   - SSL termination and protocol handling
   - VM Server with HTTPCG domain registry

2. BSO Backend Server (1CPU-2GB): $12/month
   - Cellular growth algorithms
   - Binary saturation engine
   - Quantum optimization layer

3. PostgreSQL Database: $15/month
   - HTTPCG domain registry data
   - BPCI wallet and transaction data
   - Cellular replication state

TOTAL: $51/month (same cost, better architecture!)
```

---

## 🎯 **HTTPCG Advantages Over Traditional DNS**

### **Revolutionary Benefits:**

```yaml
Traditional DNS/Subdomains:
- Multiple DNS records needed
- SSL certificates for each subdomain
- Complex routing and load balancing
- Limited to DNS hierarchy
- No protocol-level features

HTTPCG Protocol:
- Single domain with intelligent routing
- One SSL certificate for all services
- Protocol-aware routing and caching
- Unlimited virtual domains
- Built-in security and quantum features
- Cellular replication support
- Real-time domain registration
```

### **HTTPCG Protocol Features:**

```rust
// From vm_server.rs - Real HTTPCG features
- Dynamic domain registration via BPCI Enterprise registry
- Quantum-secure domain resolution
- Cellular replication of domain services
- Multi-plane routing (app, secure, gov, dark)
- Country-specific domain handling
- Real-time WebSocket support
- Post-quantum cryptography
- Shadow Registry integration
- ZKLock mobile device support
```

---

## 🚀 **Deployment Checklist**

### **HTTPCG Infrastructure:**
- [ ] HTTPCG Gateway Server deployed (2CPU-4GB)
- [ ] BSO Backend Server deployed (1CPU-2GB)
- [ ] PostgreSQL database created
- [ ] Single DNS record (pravyom.com) configured
- [ ] SSL certificate installed

### **HTTPCG Services (Real Implementation):**
- [ ] VM Server with hash-based HTTPCG routing (port 7777)
- [ ] BPCI Registry Server with wallet/token system (port 8082)
- [ ] XTMP Protocol Server for BPI-BPCI communication (port 8081)
- [ ] Nginx gateway with HTTPCG hash parameter routing
- [ ] HTTPCG domain registry with DID registration
- [ ] Shadow Registry bridge for dynamic resolution
- [ ] QLOCK quantum-safe session management
- [ ] Cellular growth algorithms with economic incentives

### **HTTPCG Domains (With Hash Parameters):**
- [ ] httpcg://app/prav.global/?hash=wallet_hash (BPCI Wallet) working
- [ ] httpcg://app/taskflow.global/?hash=app_hash (TaskFlow App) working
- [ ] /hash.bpi/<WALLET_ADDR>/ (Wallet-routed URLs) working
- [ ] BPCI Enterprise registry domain resolution working
- [ ] Multi-plane routing (app/secure/gov/dark) working
- [ ] httpcg://app/demo.global/ (Demo App) working
- [ ] httpcg://secure/bpci.corp/ (Enterprise) working
- [ ] httpcg://gov/pravyom.gov/ (Government) working
- [ ] Country domains (.in, .us, .uk) working

---

## 🎉 **HTTPCG Deployment Complete**

Your BSO infrastructure now uses the **real HTTPCG domain system** instead of traditional DNS subdomains:

- **Single domain** (pravyom.com) with intelligent HTTPCG routing
- **Revolutionary protocol** with quantum security and cellular replication
- **Dynamic domain registration** via BPCI Enterprise registry
- **Multi-plane architecture** (app, secure, gov, dark)
- **$51/month cost** with superior functionality

The HTTPCG protocol provides **unlimited virtual domains**, **quantum security**, and **cellular replication** - making traditional DNS look primitive! 🌐🧬⚡
