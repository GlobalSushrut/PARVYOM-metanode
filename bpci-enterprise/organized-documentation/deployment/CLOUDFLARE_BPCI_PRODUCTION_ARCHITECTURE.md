# Cloudflare BPCI Integration - Production Architecture

**Version:** 1.0  
**Date:** November 3, 2025  
**Status:** Production-Ready

---

## 🏗️ System Architecture Overview

The Cloudflare BPCI Integration System establishes a production-ready bridge between Cloudflare's edge network and the BPCI (Blockchain Protocol Communication Infrastructure), enabling Web2 to Web3.5 transformation, BPI node management, and real-time mesh network communication.

---

## 📐 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        Cloudflare Edge Network                       │
│                                                                       │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │                    Cloudflare Workers Layer                     │ │
│  │                                                                  │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌────────────────────┐  │ │
│  │  │ DynaRoutes   │  │ Domain       │  │ BPI Proxy          │  │ │
│  │  │ Gateway      │  │ Market       │  │                    │  │ │
│  │  │              │  │              │  │                    │  │ │
│  │  │ • Service    │  │ • Domain Reg │  │ • BPI Node Proxy   │  │ │
│  │  │   Discovery  │  │ • Web2→Web3.5│  │ • Wallet-based     │  │ │
│  │  │ • HTTP→QUIC  │  │ • Verification│ │   Routing          │  │ │
│  │  │ • Virtual    │  │ • Payment    │  │ • Connection Mgmt  │  │ │
│  │  │   Addressing │  │   Processing │  │                    │  │ │
│  │  └──────────────┘  └──────────────┘  └────────────────────┘  │ │
│  │         │                  │                     │             │ │
│  └─────────┼──────────────────┼─────────────────────┼─────────────┘ │
│            │                  │                     │               │
│  ┌─────────┼──────────────────┼─────────────────────┼─────────────┐ │
│  │         │    Cloudflare KV Storage (Distributed State)        │ │
│  │         │                                                       │ │
│  │  • DYNAROUTES_VIRTUAL_ADDRESSES  • BPI_CONNECTIONS            │ │
│  │  • DYNAROUTES_SERVICE_DISCOVERY  • QUIC_CONNECTION_POOL       │ │
│  │  • DOMAIN_MAPPINGS               • USER_SESSIONS              │ │
│  │  • DOMAIN_VERIFICATION           • PAYMENT_RECORDS            │ │
│  └───────────────────────────────────────────────────────────────┘ │
│            │                  │                     │               │
└────────────┼──────────────────┼─────────────────────┼───────────────┘
             │                  │                     │
             └──────────────────┴─────────────────────┘
                                │
                    ┌───────────▼────────────┐
                    │   BPCI Infrastructure  │
                    │   (134.209.210.181)    │
                    │                        │
                    │  ┌──────────────────┐ │
                    │  │ BPCI Network     │ │
                    │  │ Server           │ │
                    │  │ Port: 8087       │ │
                    │  │                  │ │
                    │  │ • SAPI Mesh      │ │
                    │  │ • HTTPCG Registry│ │
                    │  │ • mDNS Discovery │ │
                    │  │ • Quantum-Safe   │ │
                    │  └──────────────────┘ │
                    │           │            │
                    │  ┌────────▼─────────┐ │
                    │  │ Other BPCI       │ │
                    │  │ Services         │ │
                    │  │                  │ │
                    │  │ • Cluster Ledger │ │
                    │  │   (Port 6002)    │ │
                    │  │ • XTMP Server    │ │
                    │  │   (Port 7778)    │ │
                    │  │ • Auction Server │ │
                    │  │   (Port 7002)    │ │
                    │  └──────────────────┘ │
                    └────────────────────────┘
```

---

## 🔧 Component Details

### 1. DynaRoutes Gateway Worker

**Purpose:** HTTP-to-QUIC bridge with virtual address resolution and service discovery

**Key Features:**
- Service name resolution via KV lookup
- Virtual address mapping
- Round-robin load balancing
- QUIC connection pooling
- DynaRoutes protocol header injection

**API Endpoints:**
- All requests to `*.bpci.pravyom.com` are routed through this gateway

**KV Namespaces Used:**
- `DYNAROUTES_VIRTUAL_ADDRESSES` - Virtual address mappings
- `DYNAROUTES_SERVICE_DISCOVERY` - Service endpoint registry
- `QUIC_CONNECTION_POOL` - Connection metadata

**Request Flow:**
```
1. Client Request → *.bpci.pravyom.com
2. Extract service name from subdomain
3. Lookup service endpoints in KV
4. Select endpoint (round-robin)
5. Create proxy request with DynaRoutes headers
6. Forward to BPCI service
7. Return response to client
```

**Registered as Mesh Node:**
- Node ID: `eb5f55d2-f0ea-4c4d-8878-5347e65f5cfe`
- Node Type: Gateway
- Capabilities: http, quic, dynaroutes, service-discovery

---

### 2. Domain Market Worker

**Purpose:** Web2 to Web3.5 domain transformation with BPI wallet integration

**Key Features:**
- Domain registration API
- Domain verification system
- Web2 → Web3.5 address mapping
- BPI wallet integration
- Payment processing

**API Endpoints:**
- `GET /` - Domain market homepage
- `POST /api/domain/register` - Register new domain
- `POST /api/domain/verify` - Verify domain ownership
- `GET /api/domain/status` - Check domain status

**KV Namespaces Used:**
- `DOMAIN_MAPPINGS` - Domain to Web3 address mappings
- `DOMAIN_VERIFICATION` - Verification tokens and status
- `USER_SESSIONS` - User session management
- `PAYMENT_RECORDS` - Payment transaction records

**Domain Registration Flow:**
```
1. User submits domain + wallet address
2. Generate verification token
3. Store domain mapping in KV
4. Create Web3 address: {domain}.pravyom.@global
5. Return verification token to user
6. User adds TXT record to domain
7. System verifies TXT record
8. Domain status: pending → verified
9. Domain is now accessible via Web3 address
```

**Registered as Mesh Node:**
- Node ID: `7c8fdfa5-0e2c-40e2-a5a4-249d793a5783`
- Node Type: Endpoint
- Capabilities: http, domain-registration, web3-bridge

---

### 3. BPI Proxy Worker

**Purpose:** Auto-proxy for BPI nodes with custom domains and wallet-based routing

**Key Features:**
- BPI node connection management
- Wallet-based proxy setup
- Custom domain routing
- Connection statistics tracking
- Health monitoring

**API Endpoints:**
- All requests to `*.bpi.pravyom.com` are routed through this proxy

**KV Namespaces Used:**
- `BPI_CONNECTIONS` - BPI node connection metadata
- `DOMAIN_MAPPINGS` - Domain to BPI node mappings

**Proxy Flow:**
```
1. Client Request → {node}.bpi.pravyom.com
2. Extract BPI node ID from subdomain
3. Lookup node connection in KV
4. Resolve BPI node endpoint
5. Create proxy request with BPI headers
6. Forward to BPI node
7. Update connection statistics
8. Return response to client
```

**Registered as Mesh Node:**
- Node ID: `375e05bd-1937-4b6e-bafc-dee16ce76379`
- Node Type: Bridge
- Capabilities: quic, http, bpi-proxy

---

## 🗄️ KV Namespace Architecture

### 1. DYNAROUTES_VIRTUAL_ADDRESSES
**Purpose:** Virtual address to physical endpoint mappings

**Schema:**
```json
{
  "service_name": "consensus",
  "virtual_address": {
    "iaav6": "virtual://consensus.bpci.pravyom.com",
    "address": "134.209.210.181",
    "port": 6002
  }
}
```

### 2. DYNAROUTES_SERVICE_DISCOVERY
**Purpose:** Service discovery registry with load balancing

**Schema:**
```json
{
  "service_name": "consensus",
  "endpoints": [
    {
      "address": "134.209.210.181",
      "port": 6002,
      "weight": 1,
      "health": "healthy"
    }
  ]
}
```

### 3. BPI_CONNECTIONS
**Purpose:** BPI node connection metadata and statistics

**Schema:**
```json
{
  "connection_id": "conn_cloudflare-gateway_1730000000",
  "bpi_address": "bpi1abc123...",
  "endpoint": "134.209.210.181:8080",
  "status": "active",
  "last_used": "2025-11-03T18:00:00Z",
  "request_count": 42
}
```

### 4. QUIC_CONNECTION_POOL
**Purpose:** QUIC connection pooling and management

**Schema:**
```json
{
  "connection_id": "quic_conn_123",
  "endpoint": "134.209.210.181:8443",
  "protocol": "QUIC",
  "created_at": "2025-11-03T18:00:00Z",
  "last_used": "2025-11-03T18:05:00Z"
}
```

### 5. DOMAIN_MAPPINGS
**Purpose:** Web2 domain to Web3 address mappings

**Schema:**
```json
{
  "domain": "example.com",
  "domain_type": "global",
  "wallet": "bpi1abc123...",
  "web3_address": "example.pravyom.@global",
  "status": "verified",
  "created_at": "2025-11-03T18:00:00Z",
  "verification_token": "pravyom_abc123xyz"
}
```

### 6. DOMAIN_VERIFICATION
**Purpose:** Domain verification tokens and status

**Schema:**
```json
{
  "verification_token": "pravyom_abc123xyz",
  "domain": "example.com",
  "wallet": "bpi1abc123...",
  "status": "pending",
  "created_at": "2025-11-03T18:00:00Z"
}
```

### 7. USER_SESSIONS
**Purpose:** User session management and authentication

**Schema:**
```json
{
  "session_id": "sess_abc123",
  "user_id": "user_123",
  "wallet": "bpi1abc123...",
  "created_at": "2025-11-03T18:00:00Z",
  "expires_at": "2025-11-03T19:00:00Z"
}
```

### 8. PAYMENT_RECORDS
**Purpose:** Payment transaction records and history

**Schema:**
```json
{
  "payment_id": "pay_abc123",
  "domain": "example.com",
  "amount": "100",
  "currency": "BPI",
  "status": "completed",
  "created_at": "2025-11-03T18:00:00Z"
}
```

---

## 🌐 DNS Configuration

### BPCI Service Endpoints

```
quic.pravyom.com        → 134.209.210.181 (QUIC Gateway)
consensus.pravyom.com   → 134.209.210.181 (Cluster Ledger)
auction.pravyom.com     → 134.209.210.181 (Auction Server)
xtmp.pravyom.com        → 134.209.210.181 (XTMP Server)
```

### Wildcard Routing

```
*.bpci.pravyom.com      → DynaRoutes Gateway Worker
*.bpi.pravyom.com       → BPI Proxy Worker
domain-market.pravyom.com → Domain Market Worker
```

---

## 🔒 Security Architecture

### Firewall Configuration

**BPCI Server (134.209.210.181):**
```bash
# Critical BPCI services
8087/tcp    ALLOW    Anywhere  # BPCI Network Server (SAPI Mesh)
6002/tcp    ALLOW    Anywhere  # Cluster Ledger
7778/tcp    ALLOW    Anywhere  # XTMP Server
7002/tcp    ALLOW    Anywhere  # Auction Server

# Standard services
22/tcp      ALLOW    Anywhere  # SSH
80/tcp      ALLOW    Anywhere  # HTTP
443/tcp     ALLOW    Anywhere  # HTTPS
8080/tcp    ALLOW    Anywhere  # General HTTP

# QUIC services
8443/udp    ALLOW    Anywhere  # QUIC
8444/udp    ALLOW    Anywhere  # QUIC
8445/udp    ALLOW    Anywhere  # QUIC
8446/udp    ALLOW    Anywhere  # QUIC
```

### Cloudflare API Security

**API Token Permissions:**
- Workers KV Storage: Edit
- Workers Scripts: Edit
- DNS Settings: Edit
- Account Settings: Edit
- Zone: Edit
- DNS: Edit
- Analytics: Read

**IP Filtering:**
- IPv4: 134.209.210.181, 68.183.25.25, 99.246.124.40, 10.0.0.19
- IPv6: 2604:a880:400:d1:0:3:1df5:b001, 2607:fea8:4c81:cb00::9b87, etc.

---

## 🔄 Data Flow Diagrams

### Domain Registration Flow

```
User Browser
    │
    │ POST /api/domain/register
    │ { domain, wallet, domain_type }
    ▼
Domain Market Worker
    │
    │ 1. Generate verification token
    │ 2. Create domain mapping
    │ 3. Store in KV
    │
    ├─► DOMAIN_MAPPINGS KV
    │   { domain → web3_address }
    │
    └─► DOMAIN_VERIFICATION KV
        { token → domain }
    │
    │ Return verification token
    ▼
User Browser
    │
    │ Add TXT record to domain
    │ TXT: pravyom-verify={token}
    ▼
Domain Market Worker
    │
    │ POST /api/domain/verify
    │ { domain, token }
    │
    │ 1. Verify TXT record
    │ 2. Update domain status
    │ 3. Activate Web3 address
    ▼
Domain is now accessible via Web3!
```

### BPI Node Proxy Flow

```
Client Request
    │
    │ https://{node}.bpi.pravyom.com/api
    ▼
BPI Proxy Worker
    │
    │ 1. Extract node ID from subdomain
    │ 2. Lookup node connection in KV
    │
    ├─► BPI_CONNECTIONS KV
    │   { node_id → endpoint }
    │
    │ 3. Create proxy request
    │ 4. Add BPI headers
    │
    ▼
BPI Node (via BPCI)
    │
    │ Process request
    │
    ▼
BPI Proxy Worker
    │
    │ 5. Update connection stats
    │ 6. Return response
    │
    ▼
Client Response
```

### DynaRoutes Service Discovery Flow

```
Client Request
    │
    │ https://consensus.bpci.pravyom.com/api
    ▼
DynaRoutes Gateway Worker
    │
    │ 1. Extract service name: "consensus"
    │ 2. Lookup service endpoints
    │
    ├─► DYNAROUTES_SERVICE_DISCOVERY KV
    │   { service_name → endpoints[] }
    │
    │ 3. Select endpoint (round-robin)
    │ 4. Create proxy request
    │ 5. Add DynaRoutes headers
    │
    ▼
BPCI Service (Cluster Ledger)
    │
    │ Process request
    │
    ▼
DynaRoutes Gateway Worker
    │
    │ 6. Update connection pool
    │ 7. Return response
    │
    ▼
Client Response
```

---

## 🔌 BPCI Network Server Integration

### SAPI Mesh Registration

**Endpoint:** `POST http://134.209.210.181:8087/api/v1/mesh/nodes`

**Request Structure:**
```json
{
  "node_address": "cloudflare-gateway",
  "node_type": "Gateway",
  "capabilities": ["http", "quic", "dynaroutes", "service-discovery"]
}
```

**Response Structure:**
```json
{
  "success": true,
  "node_id": "eb5f55d2-f0ea-4c4d-8878-5347e65f5cfe",
  "message": "Mesh node cloudflare-gateway registered successfully"
}
```

**Node Types:**
- `Gateway` - Entry point for external traffic
- `Router` - Internal routing and forwarding
- `Endpoint` - Service endpoint
- `Bridge` - Bridge between networks

**Registered Nodes:**
1. `cloudflare-gateway` (Gateway) - DynaRoutes Gateway Worker
2. `quic-proxy` (Bridge) - BPI Proxy Worker
3. `domain-market` (Endpoint) - Domain Market Worker

---

## 📊 Monitoring & Health Checks

### Health Check Endpoints

**BPCI Network Server:**
```bash
GET http://134.209.210.181:8087/health
```

**Response:**
```json
{
  "status": "healthy",
  "uptime_seconds": 12345,
  "components": {
    "httpcg": true,
    "sapi_mesh": true,
    "mdns": true,
    "quantum_safe": true
  }
}
```

### Metrics Endpoints

**BPCI Network Server:**
```bash
GET http://134.209.210.181:8087/api/v1/metrics
```

**Mesh Node Statistics:**
```bash
GET http://134.209.210.181:8087/api/v1/mesh/stats
```

---

## 🚀 Deployment Process

### Prerequisites

1. Cloudflare account with API token
2. BPCI infrastructure running on 134.209.210.181
3. Firewall port 8087 open
4. DNS records configured

### Deployment Steps

```bash
# 1. Clone repository
git clone <repo>
cd cloudflare-bpci-integration

# 2. Configure API token
echo "CLOUDFLARE_API_TOKEN=your_token_here" > .secret

# 3. Build and run
cargo build --release
cargo run --release

# 4. Verify deployment
curl http://134.209.210.181:8087/api/v1/mesh/nodes
```

### Verification Checklist

- [ ] All 3 Cloudflare Workers deployed
- [ ] All 8 KV namespaces created
- [ ] All 4 DNS records configured
- [ ] All 3 mesh nodes registered
- [ ] Health checks passing
- [ ] Firewall port 8087 open

---

## 🔧 Troubleshooting

### Common Issues

**Issue: Connection timeout to BPCI Network Server**
- **Cause:** Firewall blocking port 8087
- **Solution:** `ufw allow 8087/tcp`

**Issue: 404 Not Found on mesh registration**
- **Cause:** Incorrect API endpoint
- **Solution:** Use `/api/v1/mesh/nodes` (not `/mesh/register`)

**Issue: JavaScript syntax error in Worker**
- **Cause:** ES6 export syntax or duplicate methods
- **Solution:** Use `addEventListener('fetch', ...)` pattern

**Issue: KV namespace not found**
- **Cause:** Missing Workers KV Storage:Edit permission
- **Solution:** Update API token permissions

---

## 📚 References

- [Cloudflare Workers Documentation](https://developers.cloudflare.com/workers/)
- [Cloudflare KV Documentation](https://developers.cloudflare.com/workers/runtime-apis/kv/)
- [BPCI Architecture Documentation](./CLOUDFLARE_BPCI_BPI_INTEGRATION_DOCS/)
- [Deployment Report](./CLOUDFLARE_BPCI_DEPLOYMENT_REPORT.md)

---

## 📞 Support

For issues or questions:
- Review deployment report: `CLOUDFLARE_BPCI_DEPLOYMENT_REPORT.md`
- Check integration docs: `CLOUDFLARE_BPCI_BPI_INTEGRATION_DOCS/`
- Contact BPCI/BPI infrastructure team

---

**Document Version:** 1.0  
**Last Updated:** November 3, 2025  
**Status:** Production-Ready ✅
