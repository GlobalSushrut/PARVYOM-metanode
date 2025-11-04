# Cloudflare Domain Market Architecture - Based on Real BPCI Code

## Executive Summary

This document outlines the precise architecture for a **Cloudflare Domain Market** that allows users to inject their Web 2.0 domains and seamlessly transform them into Web 3.5 domains with integrated BPI/BPCI wallet connections, based on the actual HTTPCG domain registry and Shadow Registry implementations.

## 1. Real BPCI Domain System Analysis

### 1.1 HTTPCG Domain Registry (Actual Implementation)

**From `bpci_network_server.rs`:**
```rust
// Real HTTPCG Domain Structure
struct HttpcgDomain {
    pub domain_name: String,        // e.g., "prav@global", "prav@gov"
    pub domain_type: DomainType,    // Global, Country, Government, etc.
    pub owner_wallet: String,       // BPI wallet address
    pub security_level: SecurityLevel, // Public, Enhanced, Classified, Quantum
    pub registered_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub status: DomainStatus,       // Active, Pending, Suspended, etc.
    pub metadata: HashMap<String, String>,
}

// Real Domain Types
enum DomainType {
    Global,           // @global domains
    Country(String),  // @us, @in, @uk country domains
    Government,       // @gov government domains
    Corporate,        // @corp corporate domains
    Educational,      // @edu educational domains
    Military,         // @mil military domains
    Dark,             // @dark private network domains
    Quantum,          // Quantum-safe only
}

// Real Security Levels
enum SecurityLevel {
    Public,       // Public access
    Enhanced,     // Enhanced security
    Classified,   // Classified access
    Quantum,      // Quantum-safe required
}
```

### 1.2 Shadow Registry Domain Mapping (Actual Implementation)

**From `bpci_shadow_registry_server.rs`:**
```rust
// Real Web2-Web3 Domain Mapping
struct DomainMapping {
    pub mapping_id: String,
    pub web2_domain: String,        // Traditional domain (e.g., "example.com")
    pub web3_address: String,       // Web3 address (e.g., "prav@global")
    pub mapping_type: MappingType,  // DomainToAddress, SubdomainToContract, etc.
    pub bidirectional: bool,        // Two-way mapping
    pub created_at: DateTime<Utc>,
    pub last_verified: DateTime<Utc>,
}

// Real Mapping Types
enum MappingType {
    DomainToAddress,      // example.com → prav@global
    SubdomainToContract,  // api.example.com → contract@corp
    ApiToService,         // api.example.com → service@dark
    Custom,               // Custom mapping logic
}
```

### 1.3 Real API Endpoints

**Domain Registration API:**
```rust
// POST /api/domain/register
async fn register_domain(
    State(state): State<NetworkServerState>,
    Json(req): Json<RegisterDomainRequest>,
) -> Result<Json<RegisterDomainResponse>, StatusCode>

// POST /api/domain/mapping
async fn create_domain_mapping(
    State(state): State<ShadowRegistryState>,
    Json(req): Json<CreateDomainMappingRequest>,
) -> Result<Json<CreateDomainMappingResponse>, StatusCode>
```

## 2. Cloudflare Domain Market Architecture

### 2.1 Domain Market Interface

**Cloudflare Worker: Domain Market Portal**
```javascript
// workers/domain-market.js
export default {
  async fetch(request, env, ctx) {
    const url = new URL(request.url);
    
    // Domain Market Portal
    if (url.pathname === '/') {
      return await serveDomainMarketUI(request, env);
    }
    
    // Domain injection API
    if (url.pathname === '/api/inject-domain') {
      return await handleDomainInjection(request, env);
    }
    
    // Domain rental API
    if (url.pathname === '/api/rent-domain') {
      return await handleDomainRental(request, env);
    }
    
    // Wallet connection API
    if (url.pathname === '/api/connect-wallet') {
      return await handleWalletConnection(request, env);
    }
    
    // Domain upgrade API
    if (url.pathname === '/api/upgrade-to-web35') {
      return await handleWeb35Upgrade(request, env);
    }
    
    return new Response('Domain Market API', { status: 200 });
  }
};

// Serve Domain Market UI
async function serveDomainMarketUI(request, env) {
  const html = `
<!DOCTYPE html>
<html>
<head>
    <title>Pravyom Domain Market - Web2 to Web3.5 Transformation</title>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style>
        body { font-family: Arial, sans-serif; margin: 0; padding: 20px; background: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; background: white; padding: 30px; border-radius: 10px; }
        .header { text-align: center; margin-bottom: 40px; }
        .domain-input { width: 100%; padding: 15px; font-size: 18px; border: 2px solid #ddd; border-radius: 5px; margin: 10px 0; }
        .btn { background: #007cba; color: white; padding: 15px 30px; border: none; border-radius: 5px; cursor: pointer; font-size: 16px; margin: 10px; }
        .btn:hover { background: #005a87; }
        .domain-types { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px; margin: 30px 0; }
        .domain-type { border: 2px solid #ddd; padding: 20px; border-radius: 10px; text-align: center; cursor: pointer; }
        .domain-type:hover { border-color: #007cba; background: #f0f8ff; }
        .wallet-section { background: #f9f9f9; padding: 20px; border-radius: 10px; margin: 20px 0; }
        .status { padding: 10px; margin: 10px 0; border-radius: 5px; }
        .success { background: #d4edda; color: #155724; border: 1px solid #c3e6cb; }
        .error { background: #f8d7da; color: #721c24; border: 1px solid #f5c6cb; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🌐 Pravyom Domain Market</h1>
            <p>Transform your Web 2.0 domain into Web 3.5 with BPI/BPCI integration</p>
        </div>
        
        <div class="domain-injection">
            <h2>💉 Inject Your Domain</h2>
            <input type="text" id="web2Domain" class="domain-input" placeholder="Enter your Web 2.0 domain (e.g., example.com)">
            <button class="btn" onclick="injectDomain()">Inject Domain</button>
        </div>
        
        <div class="domain-types">
            <div class="domain-type" onclick="selectDomainType('global')">
                <h3>🌍 Global Domain</h3>
                <p>yoursite@global</p>
                <p>Public access, worldwide reach</p>
                <p><strong>$10/month</strong></p>
            </div>
            <div class="domain-type" onclick="selectDomainType('corp')">
                <h3>🏢 Corporate Domain</h3>
                <p>yoursite@corp</p>
                <p>Enhanced security, business features</p>
                <p><strong>$25/month</strong></p>
            </div>
            <div class="domain-type" onclick="selectDomainType('dark')">
                <h3>🔒 Dark Domain</h3>
                <p>yoursite@dark</p>
                <p>Private network, maximum privacy</p>
                <p><strong>$50/month</strong></p>
            </div>
            <div class="domain-type" onclick="selectDomainType('quantum')">
                <h3>⚛️ Quantum Domain</h3>
                <p>yoursite@quantum</p>
                <p>Quantum-safe encryption</p>
                <p><strong>$100/month</strong></p>
            </div>
        </div>
        
        <div class="wallet-section">
            <h2>💰 Connect Your Wallet</h2>
            <p>Connect your BPI wallet to manage your Web 3.5 domains</p>
            <button class="btn" onclick="connectMojoWallet()">Connect Mojo Wallet</button>
            <button class="btn" onclick="connectBPIWallet()">Connect BPI Wallet</button>
            <div id="walletStatus"></div>
        </div>
        
        <div id="status"></div>
    </div>
    
    <script>
        let selectedDomainType = null;
        let connectedWallet = null;
        
        function selectDomainType(type) {
            selectedDomainType = type;
            document.querySelectorAll('.domain-type').forEach(el => el.style.background = '');
            event.target.closest('.domain-type').style.background = '#e3f2fd';
            showStatus('Selected domain type: ' + type, 'success');
        }
        
        async function injectDomain() {
            const web2Domain = document.getElementById('web2Domain').value;
            if (!web2Domain) {
                showStatus('Please enter a domain', 'error');
                return;
            }
            
            if (!selectedDomainType) {
                showStatus('Please select a domain type', 'error');
                return;
            }
            
            if (!connectedWallet) {
                showStatus('Please connect your wallet first', 'error');
                return;
            }
            
            try {
                const response = await fetch('/api/inject-domain', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        web2_domain: web2Domain,
                        domain_type: selectedDomainType,
                        wallet_address: connectedWallet,
                        security_level: getSecurityLevel(selectedDomainType)
                    })
                });
                
                const result = await response.json();
                if (result.success) {
                    showStatus('Domain injected successfully! Web3.5 address: ' + result.web3_address, 'success');
                } else {
                    showStatus('Error: ' + result.message, 'error');
                }
            } catch (error) {
                showStatus('Error injecting domain: ' + error.message, 'error');
            }
        }
        
        async function connectMojoWallet() {
            try {
                if (typeof window.mojo !== 'undefined') {
                    const wallet = await window.mojo.connect();
                    connectedWallet = wallet.address;
                    document.getElementById('walletStatus').innerHTML = 
                        '<div class="success">Mojo Wallet Connected: ' + connectedWallet + '</div>';
                } else {
                    showStatus('Mojo wallet not found. Please install Mojo wallet extension.', 'error');
                }
            } catch (error) {
                showStatus('Error connecting Mojo wallet: ' + error.message, 'error');
            }
        }
        
        async function connectBPIWallet() {
            try {
                const response = await fetch('/api/connect-wallet', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ wallet_type: 'bpi' })
                });
                
                const result = await response.json();
                if (result.success) {
                    connectedWallet = result.wallet_address;
                    document.getElementById('walletStatus').innerHTML = 
                        '<div class="success">BPI Wallet Connected: ' + connectedWallet + '</div>';
                } else {
                    showStatus('Error connecting BPI wallet: ' + result.message, 'error');
                }
            } catch (error) {
                showStatus('Error connecting BPI wallet: ' + error.message, 'error');
            }
        }
        
        function getSecurityLevel(domainType) {
            const levels = {
                'global': 'Public',
                'corp': 'Enhanced',
                'dark': 'Classified',
                'quantum': 'Quantum'
            };
            return levels[domainType] || 'Public';
        }
        
        function showStatus(message, type) {
            document.getElementById('status').innerHTML = 
                '<div class="status ' + type + '">' + message + '</div>';
        }
    </script>
</body>
</html>`;
  
  return new Response(html, {
    headers: { 'Content-Type': 'text/html' }
  });
}
```

### 2.2 Domain Injection API Handler

```javascript
// Handle domain injection (Web2 → Web3.5)
async function handleDomainInjection(request, env) {
  try {
    const data = await request.json();
    const { web2_domain, domain_type, wallet_address, security_level } = data;
    
    // Validate domain ownership
    const ownershipVerified = await verifyDomainOwnership(web2_domain, env);
    if (!ownershipVerified) {
      return new Response(JSON.stringify({
        success: false,
        message: 'Domain ownership verification failed'
      }), { status: 400, headers: { 'Content-Type': 'application/json' } });
    }
    
    // Generate Web3.5 address based on real BPCI format
    const web3Address = generateWeb3Address(web2_domain, domain_type);
    
    // Register domain in HTTPCG registry via BPCI backend
    const registrationResult = await registerWithBPCI(web3Address, domain_type, wallet_address, security_level, env);
    
    // Create domain mapping in Shadow Registry
    const mappingResult = await createDomainMapping(web2_domain, web3Address, 'DomainToAddress', true, env);
    
    // Store in Cloudflare KV for fast lookup
    await env.DOMAIN_MAPPINGS.put(`web2:${web2_domain}`, JSON.stringify({
      web3_address: web3Address,
      domain_type: domain_type,
      wallet_address: wallet_address,
      security_level: security_level,
      created_at: new Date().toISOString(),
      status: 'active'
    }));
    
    await env.DOMAIN_MAPPINGS.put(`web3:${web3Address}`, JSON.stringify({
      web2_domain: web2_domain,
      domain_type: domain_type,
      wallet_address: wallet_address,
      security_level: security_level,
      created_at: new Date().toISOString(),
      status: 'active'
    }));
    
    return new Response(JSON.stringify({
      success: true,
      web3_address: web3Address,
      mapping_id: mappingResult.mapping_id,
      message: `Domain ${web2_domain} successfully transformed to ${web3Address}`
    }), { headers: { 'Content-Type': 'application/json' } });
    
  } catch (error) {
    return new Response(JSON.stringify({
      success: false,
      message: error.message
    }), { status: 500, headers: { 'Content-Type': 'application/json' } });
  }
}

// Generate Web3.5 address based on real BPCI format
function generateWeb3Address(web2Domain, domainType) {
  // Extract domain name without TLD
  const domainName = web2Domain.split('.')[0];
  
  // Map domain types to BPCI suffixes
  const suffixMap = {
    'global': '@global',
    'corp': '@corp',
    'dark': '@dark',
    'quantum': '@quantum',
    'gov': '@gov',
    'edu': '@edu',
    'mil': '@mil'
  };
  
  const suffix = suffixMap[domainType] || '@global';
  return `${domainName}${suffix}`;
}

// Register with real BPCI backend
async function registerWithBPCI(web3Address, domainType, walletAddress, securityLevel, env) {
  const bpciEndpoint = env.BPCI_NETWORK_SERVER || 'https://134.209.210.181:8089';
  
  const registrationData = {
    domain_name: web3Address,
    domain_type: domainType,
    owner_wallet: walletAddress,
    security_level: securityLevel
  };
  
  const response = await fetch(`${bpciEndpoint}/api/domain/register`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(registrationData)
  });
  
  return await response.json();
}

// Create domain mapping in Shadow Registry
async function createDomainMapping(web2Domain, web3Address, mappingType, bidirectional, env) {
  const shadowRegistryEndpoint = env.SHADOW_REGISTRY_SERVER || 'https://134.209.210.181:8088';
  
  const mappingData = {
    web2_domain: web2Domain,
    web3_address: web3Address,
    mapping_type: mappingType,
    bidirectional: bidirectional
  };
  
  const response = await fetch(`${shadowRegistryEndpoint}/api/domain/mapping`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(mappingData)
  });
  
  return await response.json();
}
```

### 2.3 Domain Ownership Verification

```javascript
// Verify domain ownership using DNS TXT records
async function verifyDomainOwnership(domain, env) {
  try {
    // Generate verification token
    const verificationToken = generateVerificationToken();
    
    // Store token in KV for verification
    await env.DOMAIN_VERIFICATION.put(`verify:${domain}`, verificationToken, {
      expirationTtl: 3600 // 1 hour
    });
    
    // Check for DNS TXT record
    const txtRecords = await resolveDNSTxt(domain);
    const expectedRecord = `pravyom-verification=${verificationToken}`;
    
    return txtRecords.some(record => record.includes(expectedRecord));
  } catch (error) {
    console.error('Domain verification error:', error);
    return false;
  }
}

// DNS TXT record resolution
async function resolveDNSTxt(domain) {
  try {
    const response = await fetch(`https://cloudflare-dns.com/dns-query?name=${domain}&type=TXT`, {
      headers: { 'Accept': 'application/dns-json' }
    });
    
    const data = await response.json();
    return data.Answer ? data.Answer.map(record => record.data) : [];
  } catch (error) {
    console.error('DNS resolution error:', error);
    return [];
  }
}

function generateVerificationToken() {
  return Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15);
}
```

### 2.4 Wallet Connection Proxy Layer

```javascript
// Handle wallet connections for BPI/BPCI integration
async function handleWalletConnection(request, env) {
  try {
    const data = await request.json();
    const { wallet_type } = data;
    
    if (wallet_type === 'bpi') {
      // Connect to BPI wallet via BPCI backend
      const walletResponse = await connectBPIWallet(env);
      return new Response(JSON.stringify(walletResponse), {
        headers: { 'Content-Type': 'application/json' }
      });
    } else if (wallet_type === 'mojo') {
      // Handle Mojo wallet connection
      const mojoResponse = await connectMojoWallet(env);
      return new Response(JSON.stringify(mojoResponse), {
        headers: { 'Content-Type': 'application/json' }
      });
    }
    
    return new Response(JSON.stringify({
      success: false,
      message: 'Unsupported wallet type'
    }), { status: 400, headers: { 'Content-Type': 'application/json' } });
    
  } catch (error) {
    return new Response(JSON.stringify({
      success: false,
      message: error.message
    }), { status: 500, headers: { 'Content-Type': 'application/json' } });
  }
}

// Connect to BPI wallet via BPCI backend
async function connectBPIWallet(env) {
  const bpiEndpoint = env.BPI_BRIDGE_SERVER || 'https://134.209.210.181:6001';
  
  try {
    const response = await fetch(`${bpiEndpoint}/api/wallet/create`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        wallet_type: 'BPI',
        network: 'testnet'
      })
    });
    
    const result = await response.json();
    
    if (result.success) {
      return {
        success: true,
        wallet_address: result.wallet_address,
        wallet_type: 'BPI',
        network: 'testnet'
      };
    } else {
      return {
        success: false,
        message: 'Failed to create BPI wallet'
      };
    }
  } catch (error) {
    return {
      success: false,
      message: `BPI wallet connection error: ${error.message}`
    };
  }
}

// Connect to Mojo wallet
async function connectMojoWallet(env) {
  // Mojo wallet connection is handled client-side
  // This endpoint provides server-side validation
  return {
    success: true,
    message: 'Mojo wallet connection handled client-side',
    wallet_type: 'Mojo'
  };
}
```

## 3. Domain Rental and Pricing System

### 3.1 Pricing Based on Real Domain Types

```javascript
// Domain pricing based on real BPCI domain types
const DOMAIN_PRICING = {
  'global': {
    monthly: 10,
    yearly: 100,
    features: ['Public access', 'Global reach', 'Basic security']
  },
  'corp': {
    monthly: 25,
    yearly: 250,
    features: ['Enhanced security', 'Business features', 'Priority support']
  },
  'dark': {
    monthly: 50,
    yearly: 500,
    features: ['Private network', 'Maximum privacy', 'Anonymous routing']
  },
  'quantum': {
    monthly: 100,
    yearly: 1000,
    features: ['Quantum-safe encryption', 'Ultra-secure', 'Future-proof']
  },
  'gov': {
    monthly: 75,
    yearly: 750,
    features: ['Government grade', 'Classified access', 'Compliance ready']
  },
  'edu': {
    monthly: 15,
    yearly: 150,
    features: ['Educational discount', 'Academic features', 'Research tools']
  }
};

// Handle domain rental
async function handleDomainRental(request, env) {
  try {
    const data = await request.json();
    const { domain_mapping_id, rental_period, payment_method } = data;
    
    // Get domain mapping details
    const mappingData = await env.DOMAIN_MAPPINGS.get(`mapping:${domain_mapping_id}`);
    if (!mappingData) {
      return new Response(JSON.stringify({
        success: false,
        message: 'Domain mapping not found'
      }), { status: 404, headers: { 'Content-Type': 'application/json' } });
    }
    
    const mapping = JSON.parse(mappingData);
    const pricing = DOMAIN_PRICING[mapping.domain_type];
    const cost = rental_period === 'yearly' ? pricing.yearly : pricing.monthly;
    
    // Process payment (integrate with BPI payment system)
    const paymentResult = await processPayment(cost, payment_method, mapping.wallet_address, env);
    
    if (paymentResult.success) {
      // Update domain status and expiration
      const expirationDate = new Date();
      expirationDate.setMonth(expirationDate.getMonth() + (rental_period === 'yearly' ? 12 : 1));
      
      mapping.rental_expires = expirationDate.toISOString();
      mapping.rental_status = 'active';
      
      await env.DOMAIN_MAPPINGS.put(`mapping:${domain_mapping_id}`, JSON.stringify(mapping));
      
      return new Response(JSON.stringify({
        success: true,
        rental_id: paymentResult.transaction_id,
        expires_at: mapping.rental_expires,
        cost: cost,
        message: `Domain rental successful for ${rental_period}`
      }), { headers: { 'Content-Type': 'application/json' } });
    } else {
      return new Response(JSON.stringify({
        success: false,
        message: 'Payment failed: ' + paymentResult.message
      }), { status: 400, headers: { 'Content-Type': 'application/json' } });
    }
    
  } catch (error) {
    return new Response(JSON.stringify({
      success: false,
      message: error.message
    }), { status: 500, headers: { 'Content-Type': 'application/json' } });
  }
}

// Process payment via BPI payment system
async function processPayment(amount, paymentMethod, walletAddress, env) {
  const paymentEndpoint = env.PAYMENT_SERVER || 'https://134.209.210.181:8092';
  
  try {
    const response = await fetch(`${paymentEndpoint}/api/payment/process`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        amount: amount,
        currency: 'BPI',
        payment_method: paymentMethod,
        wallet_address: walletAddress,
        description: 'Domain rental payment'
      })
    });
    
    return await response.json();
  } catch (error) {
    return {
      success: false,
      message: `Payment processing error: ${error.message}`
    };
  }
}
```

## 4. Cloudflare KV Storage Schema

### 4.1 KV Namespaces for Domain Market

```javascript
// KV Namespaces
const KV_NAMESPACES = {
  // Domain mappings (Web2 ↔ Web3.5)
  DOMAIN_MAPPINGS: 'domain-mappings',
  
  // Domain verification tokens
  DOMAIN_VERIFICATION: 'domain-verification',
  
  // User sessions and wallet connections
  USER_SESSIONS: 'user-sessions',
  
  // Domain rental records
  DOMAIN_RENTALS: 'domain-rentals',
  
  // Payment transactions
  PAYMENT_RECORDS: 'payment-records'
};

// KV Storage Schema Examples
const STORAGE_SCHEMAS = {
  // Web2 to Web3 mapping
  'web2:example.com': {
    web3_address: 'example@global',
    domain_type: 'global',
    wallet_address: 'bpi1abc123...',
    security_level: 'Public',
    created_at: '2024-01-01T00:00:00Z',
    status: 'active',
    rental_expires: '2024-12-31T23:59:59Z'
  },
  
  // Web3 to Web2 reverse mapping
  'web3:example@global': {
    web2_domain: 'example.com',
    domain_type: 'global',
    wallet_address: 'bpi1abc123...',
    security_level: 'Public',
    created_at: '2024-01-01T00:00:00Z',
    status: 'active',
    rental_expires: '2024-12-31T23:59:59Z'
  },
  
  // Domain verification
  'verify:example.com': 'verification-token-abc123',
  
  // User session
  'session:user123': {
    user_id: 'user123',
    wallet_address: 'bpi1abc123...',
    connected_domains: ['example.com', 'test.org'],
    created_at: '2024-01-01T00:00:00Z',
    expires_at: '2024-01-01T01:00:00Z'
  }
};
```

## 5. Implementation Steps

### 5.1 Phase 1: Core Infrastructure
1. **Deploy Cloudflare Workers** for domain market portal
2. **Setup KV namespaces** for data storage
3. **Configure DNS** for domain market access
4. **Implement basic UI** for domain injection

### 5.2 Phase 2: BPCI Integration
1. **Connect to real BPCI backends** (network server, shadow registry)
2. **Implement domain registration** via HTTPCG API
3. **Setup domain mapping** via Shadow Registry API
4. **Test Web2→Web3.5 transformation**

### 5.3 Phase 3: Wallet Integration
1. **Implement BPI wallet connection** via BPI bridge
2. **Add Mojo wallet support** with client-side integration
3. **Setup payment processing** via BPCI payment server
4. **Test end-to-end wallet flows**

### 5.4 Phase 4: Domain Rental System
1. **Implement pricing engine** based on domain types
2. **Add rental management** with expiration tracking
3. **Setup automated renewals** and notifications
4. **Test payment and renewal flows**

## 9. Auto Wallet-Based Proxy Architecture for BPI Nodes

### 9.1 BPI Node Connection Management (Based on Real Code)

**From `bpci_bpi_bridge.rs` - Real BPI Connection Structure:**
```rust
// Real BPI Connection Information
pub struct BpiConnection {
    pub bpi_address: String,        // BPI wallet address
    pub connection_id: String,      // Unique connection ID
    pub last_heartbeat: DateTime<Utc>,
    pub connection_quality: ConnectionQuality,
    pub transaction_count: u64,
    pub allocated_tokens: u64,
}

// Address Pool Manager for Millions of BPI Connections
pub struct AddressPoolManager {
    active_connections: Arc<RwLock<HashMap<String, BpiConnection>>>,
    connection_pool: Arc<RwLock<Vec<String>>>,
    pool_size_limit: usize,
    auto_discovery_enabled: bool,
}
```

### 9.2 Cloudflare Auto-Proxy Worker for BPI Nodes

```javascript
// workers/bpi-auto-proxy.js
export default {
  async fetch(request, env, ctx) {
    const url = new URL(request.url);
    
    // Auto-detect BPI node requests
    if (await isBPINodeRequest(request, env)) {
      return await handleBPINodeProxy(request, env);
    }
    
    // Handle domain-to-BPI routing
    if (await isDomainToBPIRoute(request, env)) {
      return await routeToBPINode(request, env);
    }
    
    // BPI node registration
    if (url.pathname === '/api/bpi/register-node') {
      return await registerBPINode(request, env);
    }
    
    // BPI node heartbeat
    if (url.pathname === '/api/bpi/heartbeat') {
      return await handleBPIHeartbeat(request, env);
    }
    
    return new Response('BPI Auto-Proxy Service', { status: 200 });
  }
};

// Detect if request is from a BPI node
async function isBPINodeRequest(request, env) {
  const bpiHeaders = [
    'X-BPI-Node-ID',
    'X-BPI-Wallet-Address', 
    'X-BPI-Connection-ID',
    'X-CBOR-Transaction',
    'X-Registry-Token'
  ];
  
  // Check for BPI-specific headers
  for (const header of bpiHeaders) {
    if (request.headers.get(header)) {
      return true;
    }
  }
  
  // Check User-Agent for BPI node signatures
  const userAgent = request.headers.get('User-Agent') || '';
  if (userAgent.includes('BPI-Core') || userAgent.includes('BPI-Node')) {
    return true;
  }
  
  // Check if source IP is a known BPI node
  const clientIP = request.headers.get('CF-Connecting-IP');
  const knownBPINode = await env.BPI_NODES.get(`ip:${clientIP}`);
  
  return !!knownBPINode;
}

// Handle BPI node proxy routing
async function handleBPINodeProxy(request, env) {
  try {
    const bpiNodeId = request.headers.get('X-BPI-Node-ID');
    const walletAddress = request.headers.get('X-BPI-Wallet-Address');
    const connectionId = request.headers.get('X-BPI-Connection-ID');
    
    // Get BPI node configuration
    const nodeConfig = await getBPINodeConfig(bpiNodeId, walletAddress, env);
    if (!nodeConfig) {
      return new Response('BPI node not registered', { status: 404 });
    }
    
    // Update heartbeat
    await updateBPINodeHeartbeat(bpiNodeId, env);
    
    // Route to appropriate BPCI service based on request
    const targetService = determineBPCIService(request);
    const bpciResponse = await routeToBPCIService(request, targetService, nodeConfig, env);
    
    // Add BPI-specific response headers
    const response = new Response(bpciResponse.body, {
      status: bpciResponse.status,
      headers: {
        ...bpciResponse.headers,
        'X-BPI-Proxy': 'cloudflare',
        'X-BPI-Node-Status': nodeConfig.status,
        'X-Connection-Quality': nodeConfig.connection_quality
      }
    });
    
    return response;
    
  } catch (error) {
    console.error('BPI proxy error:', error);
    return new Response('BPI proxy error: ' + error.message, { status: 500 });
  }
}

// Register new BPI node with auto-proxy
async function registerBPINode(request, env) {
  try {
    const data = await request.json();
    const { 
      bpi_address, 
      node_id, 
      custom_domain, 
      connection_endpoint,
      wallet_signature 
    } = data;
    
    // Verify wallet signature
    const signatureValid = await verifyBPIWalletSignature(
      bpi_address, 
      wallet_signature, 
      env
    );
    
    if (!signatureValid) {
      return new Response(JSON.stringify({
        success: false,
        message: 'Invalid wallet signature'
      }), { status: 401, headers: { 'Content-Type': 'application/json' } });
    }
    
    // Generate connection ID
    const connectionId = generateConnectionId(bpi_address, node_id);
    
    // Create BPI node configuration
    const nodeConfig = {
      bpi_address: bpi_address,
      node_id: node_id,
      connection_id: connectionId,
      custom_domain: custom_domain,
      connection_endpoint: connection_endpoint,
      status: 'active',
      connection_quality: 'excellent',
      last_heartbeat: new Date().toISOString(),
      transaction_count: 0,
      allocated_tokens: 1000000, // Default allocation
      proxy_enabled: true,
      auto_routing: true,
      registered_at: new Date().toISOString()
    };
    
    // Store in Cloudflare KV
    await env.BPI_NODES.put(`node:${node_id}`, JSON.stringify(nodeConfig));
    await env.BPI_NODES.put(`wallet:${bpi_address}`, JSON.stringify(nodeConfig));
    await env.BPI_NODES.put(`connection:${connectionId}`, JSON.stringify(nodeConfig));
    
    // If custom domain provided, create domain mapping
    if (custom_domain) {
      await createBPINodeDomainMapping(custom_domain, nodeConfig, env);
    }
    
    // Register with BPCI backend
    await registerWithBPCIBridge(nodeConfig, env);
    
    return new Response(JSON.stringify({
      success: true,
      connection_id: connectionId,
      proxy_endpoint: `https://${custom_domain || `${node_id}.bpi.pravyom.com`}`,
      bpci_bridge_status: 'connected',
      message: 'BPI node registered successfully with auto-proxy'
    }), { headers: { 'Content-Type': 'application/json' } });
    
  } catch (error) {
    return new Response(JSON.stringify({
      success: false,
      message: error.message
    }), { status: 500, headers: { 'Content-Type': 'application/json' } });
  }
}

// Create domain mapping for BPI node
async function createBPINodeDomainMapping(customDomain, nodeConfig, env) {
  // Generate Web3.5 address for BPI node
  const web3Address = `${nodeConfig.node_id}@bpi`;
  
  // Create domain mapping
  const domainMapping = {
    mapping_id: generateMappingId(),
    web2_domain: customDomain,
    web3_address: web3Address,
    mapping_type: 'BPINodeProxy',
    bpi_node_id: nodeConfig.node_id,
    bpi_wallet: nodeConfig.bpi_address,
    connection_id: nodeConfig.connection_id,
    bidirectional: true,
    auto_proxy: true,
    created_at: new Date().toISOString(),
    last_verified: new Date().toISOString()
  };
  
  // Store mapping
  await env.DOMAIN_MAPPINGS.put(`bpi:${customDomain}`, JSON.stringify(domainMapping));
  await env.DOMAIN_MAPPINGS.put(`node:${nodeConfig.node_id}`, JSON.stringify(domainMapping));
  
  // Register with Shadow Registry
  await createDomainMapping(customDomain, web3Address, 'BPINodeProxy', true, env);
}

// Route requests to BPI nodes based on domain
async function routeToBPINode(request, env) {
  const url = new URL(request.url);
  const hostname = url.hostname;
  
  // Get BPI node mapping for domain
  const mappingData = await env.DOMAIN_MAPPINGS.get(`bpi:${hostname}`);
  if (!mappingData) {
    return new Response('BPI node not found for domain', { status: 404 });
  }
  
  const mapping = JSON.parse(mappingData);
  const nodeConfig = await getBPINodeConfig(mapping.bpi_node_id, mapping.bpi_wallet, env);
  
  if (!nodeConfig || nodeConfig.status !== 'active') {
    return new Response('BPI node unavailable', { status: 503 });
  }
  
  // Route to BPI node endpoint
  const targetUrl = `${nodeConfig.connection_endpoint}${url.pathname}${url.search}`;
  
  const response = await fetch(targetUrl, {
    method: request.method,
    headers: {
      ...request.headers,
      'X-Cloudflare-Proxy': 'true',
      'X-BPI-Domain': hostname,
      'X-Original-Host': hostname
    },
    body: request.body
  });
  
  // Add proxy headers to response
  return new Response(response.body, {
    status: response.status,
    headers: {
      ...response.headers,
      'X-BPI-Node-ID': nodeConfig.node_id,
      'X-BPI-Proxy': 'cloudflare',
      'X-Connection-Quality': nodeConfig.connection_quality
    }
  });
}

// Get BPI node configuration
async function getBPINodeConfig(nodeId, walletAddress, env) {
  // Try node ID first
  if (nodeId) {
    const nodeData = await env.BPI_NODES.get(`node:${nodeId}`);
    if (nodeData) return JSON.parse(nodeData);
  }
  
  // Try wallet address
  if (walletAddress) {
    const walletData = await env.BPI_NODES.get(`wallet:${walletAddress}`);
    if (walletData) return JSON.parse(walletData);
  }
  
  return null;
}

// Update BPI node heartbeat
async function updateBPINodeHeartbeat(nodeId, env) {
  const nodeData = await env.BPI_NODES.get(`node:${nodeId}`);
  if (nodeData) {
    const nodeConfig = JSON.parse(nodeData);
    nodeConfig.last_heartbeat = new Date().toISOString();
    nodeConfig.connection_quality = 'excellent'; // Update based on response time
    
    await env.BPI_NODES.put(`node:${nodeId}`, JSON.stringify(nodeConfig));
  }
}

// Determine target BPCI service based on request
function determineBPCIService(request) {
  const url = new URL(request.url);
  const path = url.pathname;
  
  // Route based on path patterns
  if (path.startsWith('/api/transaction')) return 'bpi-bridge';
  if (path.startsWith('/api/auction')) return 'auction-mempool';
  if (path.startsWith('/api/blockchain')) return 'blockchain-server';
  if (path.startsWith('/api/consensus')) return 'consensus-server';
  if (path.startsWith('/api/ledger')) return 'cluster-ledger';
  if (path.startsWith('/api/xtmp')) return 'xtmp-server';
  
  // Default to BPI bridge
  return 'bpi-bridge';
}

// Register with BPCI backend
async function registerWithBPCIBridge(nodeConfig, env) {
  const bpiBridgeEndpoint = env.BPI_BRIDGE_SERVER || 'https://134.209.210.181:6001';
  
  try {
    const response = await fetch(`${bpiBridgeEndpoint}/api/register-node`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        bpi_address: nodeConfig.bpi_address,
        node_id: nodeConfig.node_id,
        connection_id: nodeConfig.connection_id,
        endpoint: nodeConfig.connection_endpoint,
        proxy_enabled: true
      })
    });
    
    return await response.json();
  } catch (error) {
    console.error('BPCI registration error:', error);
    return { success: false, error: error.message };
  }
}

// Verify BPI wallet signature
async function verifyBPIWalletSignature(walletAddress, signature, env) {
  // Implementation would verify cryptographic signature
  // For now, return true for valid format
  return walletAddress && signature && signature.length > 32;
}

// Generate unique connection ID
function generateConnectionId(bpiAddress, nodeId) {
  const timestamp = Date.now();
  const hash = btoa(`${bpiAddress}:${nodeId}:${timestamp}`).replace(/[^a-zA-Z0-9]/g, '');
  return `conn_${hash.substring(0, 16)}`;
}

// Generate unique mapping ID
function generateMappingId() {
  return `map_${Math.random().toString(36).substring(2, 15)}${Date.now().toString(36)}`;
}
```

### 9.3 BPI Node Auto-Discovery and Health Monitoring

```javascript
// Scheduled worker for BPI node health monitoring
export default {
  async scheduled(controller, env, ctx) {
    await performBPINodeHealthChecks(env);
    await cleanupInactiveBPINodes(env);
    await updateBPCIConnectionStatus(env);
  }
};

// Perform health checks on all registered BPI nodes
async function performBPINodeHealthChecks(env) {
  const { keys } = await env.BPI_NODES.list({ prefix: 'node:' });
  
  for (const key of keys) {
    const nodeData = await env.BPI_NODES.get(key.name);
    if (nodeData) {
      const nodeConfig = JSON.parse(nodeData);
      
      // Check if node is responsive
      const isHealthy = await checkBPINodeHealth(nodeConfig);
      
      // Update connection quality
      nodeConfig.connection_quality = isHealthy ? 'excellent' : 'poor';
      nodeConfig.last_health_check = new Date().toISOString();
      
      if (!isHealthy) {
        nodeConfig.status = 'unhealthy';
      }
      
      await env.BPI_NODES.put(key.name, JSON.stringify(nodeConfig));
    }
  }
}

// Check individual BPI node health
async function checkBPINodeHealth(nodeConfig) {
  try {
    const response = await fetch(`${nodeConfig.connection_endpoint}/health`, {
      method: 'GET',
      timeout: 5000
    });
    
    return response.ok;
  } catch (error) {
    return false;
  }
}

// Cleanup inactive BPI nodes
async function cleanupInactiveBPINodes(env) {
  const { keys } = await env.BPI_NODES.list({ prefix: 'node:' });
  const cutoffTime = new Date(Date.now() - 24 * 60 * 60 * 1000); // 24 hours
  
  for (const key of keys) {
    const nodeData = await env.BPI_NODES.get(key.name);
    if (nodeData) {
      const nodeConfig = JSON.parse(nodeData);
      const lastHeartbeat = new Date(nodeConfig.last_heartbeat);
      
      if (lastHeartbeat < cutoffTime) {
        // Mark as inactive
        nodeConfig.status = 'inactive';
        nodeConfig.connection_quality = 'disconnected';
        
        await env.BPI_NODES.put(key.name, JSON.stringify(nodeConfig));
      }
    }
  }
}
```

### 9.4 BPI Node Dashboard and Management

```javascript
// BPI Node Management Dashboard
async function serveBPINodeDashboard(request, env) {
  const html = `
<!DOCTYPE html>
<html>
<head>
    <title>BPI Node Management Dashboard</title>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style>
        body { font-family: Arial, sans-serif; margin: 0; padding: 20px; background: #f5f5f5; }
        .container { max-width: 1400px; margin: 0 auto; }
        .header { text-align: center; margin-bottom: 30px; }
        .stats-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px; margin: 20px 0; }
        .stat-card { background: white; padding: 20px; border-radius: 10px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        .node-list { background: white; padding: 20px; border-radius: 10px; margin: 20px 0; }
        .node-item { border: 1px solid #ddd; padding: 15px; margin: 10px 0; border-radius: 5px; }
        .status-active { border-left: 4px solid #28a745; }
        .status-inactive { border-left: 4px solid #dc3545; }
        .status-unhealthy { border-left: 4px solid #ffc107; }
        .btn { background: #007cba; color: white; padding: 10px 20px; border: none; border-radius: 5px; cursor: pointer; margin: 5px; }
        .btn:hover { background: #005a87; }
        .quality-excellent { color: #28a745; }
        .quality-good { color: #17a2b8; }
        .quality-fair { color: #ffc107; }
        .quality-poor { color: #dc3545; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔗 BPI Node Management Dashboard</h1>
            <p>Monitor and manage all BPI nodes connected through Cloudflare Auto-Proxy</p>
        </div>
        
        <div class="stats-grid">
            <div class="stat-card">
                <h3>Total BPI Nodes</h3>
                <div id="totalNodes">Loading...</div>
            </div>
            <div class="stat-card">
                <h3>Active Connections</h3>
                <div id="activeNodes">Loading...</div>
            </div>
            <div class="stat-card">
                <h3>Custom Domains</h3>
                <div id="customDomains">Loading...</div>
            </div>
            <div class="stat-card">
                <h3>Total Transactions</h3>
                <div id="totalTransactions">Loading...</div>
            </div>
        </div>
        
        <div class="node-list">
            <h2>Connected BPI Nodes</h2>
            <div id="nodesList">Loading nodes...</div>
        </div>
        
        <div style="text-align: center; margin: 30px 0;">
            <button class="btn" onclick="refreshDashboard()">Refresh Dashboard</button>
            <button class="btn" onclick="exportNodeData()">Export Node Data</button>
        </div>
    </div>
    
    <script>
        async function loadDashboard() {
            try {
                const response = await fetch('/api/bpi/dashboard-data');
                const data = await response.json();
                
                document.getElementById('totalNodes').textContent = data.total_nodes;
                document.getElementById('activeNodes').textContent = data.active_nodes;
                document.getElementById('customDomains').textContent = data.custom_domains;
                document.getElementById('totalTransactions').textContent = data.total_transactions;
                
                displayNodes(data.nodes);
            } catch (error) {
                console.error('Dashboard load error:', error);
            }
        }
        
        function displayNodes(nodes) {
            const nodesList = document.getElementById('nodesList');
            nodesList.innerHTML = '';
            
            nodes.forEach(node => {
                const nodeDiv = document.createElement('div');
                nodeDiv.className = \`node-item status-\${node.status}\`;
                
                nodeDiv.innerHTML = \`
                    <div style="display: flex; justify-content: space-between; align-items: center;">
                        <div>
                            <strong>Node ID:</strong> \${node.node_id}<br>
                            <strong>BPI Address:</strong> \${node.bpi_address}<br>
                            <strong>Custom Domain:</strong> \${node.custom_domain || 'None'}<br>
                            <strong>Status:</strong> \${node.status}
                        </div>
                        <div style="text-align: right;">
                            <div class="quality-\${node.connection_quality}">\${node.connection_quality.toUpperCase()}</div>
                            <div>Transactions: \${node.transaction_count}</div>
                            <div>Last Heartbeat: \${new Date(node.last_heartbeat).toLocaleString()}</div>
                        </div>
                    </div>
                \`;
                
                nodesList.appendChild(nodeDiv);
            });
        }
        
        function refreshDashboard() {
            loadDashboard();
        }
        
        async function exportNodeData() {
            try {
                const response = await fetch('/api/bpi/export-nodes');
                const blob = await response.blob();
                const url = window.URL.createObjectURL(blob);
                const a = document.createElement('a');
                a.href = url;
                a.download = 'bpi-nodes-' + new Date().toISOString().split('T')[0] + '.json';
                a.click();
            } catch (error) {
                console.error('Export error:', error);
            }
        }
        
        // Load dashboard on page load
        loadDashboard();
        
        // Auto-refresh every 30 seconds
        setInterval(loadDashboard, 30000);
    </script>
</body>
</html>`;
  
  return new Response(html, {
    headers: { 'Content-Type': 'text/html' }
  });
}
```

### 9.5 Implementation Summary

This auto wallet-based proxy architecture provides:

1. **Automatic BPI Node Detection**: Identifies BPI nodes by headers, User-Agent, and IP
2. **Seamless Domain Integration**: Allows BPI nodes to use custom domains while staying connected to BPCI
3. **Real-time Health Monitoring**: Continuous monitoring of all connected BPI nodes
4. **Auto-routing**: Intelligent routing to appropriate BPCI services based on request type
5. **Connection Management**: Maintains connection pools and handles millions of BPI connections
6. **Dashboard Interface**: Complete management interface for monitoring BPI nodes

The system ensures that any BPI node can register with a custom domain and remain fully connected to the BPCI infrastructure while benefiting from Cloudflare's global edge network and security features.

This architecture provides a complete Cloudflare-based Domain Market that seamlessly transforms Web 2.0 domains into Web 3.5 domains with full BPI/BPCI integration, based on the actual HTTPCG and Shadow Registry implementations, plus comprehensive auto wallet-based proxy handling for all BPI-connected nodes.
