# 🔗 FRONTEND-BACKEND API MAPPING & CONFORMITY

**Date**: 2025-10-30  
**Frontend**: React + Vite + TypeScript (bpci-enterprise-website)  
**Backend**: BPCI Enterprise Backend (Rust)  
**Status**: MAPPING IN PROGRESS

---

## 📊 EXECUTIVE SUMMARY

**Frontend Location**: `/home/umesh/metanode/bpci-enterprise/website/bpci-enterprise-website`  
**Frontend Type**: React + Vite + TypeScript + Tailwind  
**API Service File**: `src/services/bpciApi.ts`

**Backend Endpoints**: 12+ verified operational  
**Frontend Expected Endpoints**: 30+ API calls  
**Match Status**: PARTIAL - Needs configuration update

---

## 🎯 FRONTEND API EXPECTATIONS

### **Configuration (from bpciApi.ts)**

```typescript
const BPCI_CONFIG = {
  BPCI_SERVER: 'https://api.pravyom.com',      // Production
  BPI_CORE_SERVER: 'https://api.pravyom.com',  // BPI Core
  ADMIN_DASHBOARD: 'https://api.pravyom.com',  // Admin
  WALLET_SERVER: 'https://xtmp.pravyom.com',   // XTMP
  RPC_ENDPOINT: 'https://registry.pravyom.com', // Registry
};
```

**Current Issue**: Frontend is configured for production domains, but we need to point it to testnet server (134.209.210.181)

---

## 📋 API ENDPOINT MAPPING

### **Category 1: Authentication APIs**

| Frontend Expects | Backend Provides | Status | Notes |
|------------------|------------------|--------|-------|
| `POST /api/auth/login` | ⚠️ Via Keycloak | PARTIAL | Needs Keycloak config |
| `POST /api/auth/register` | ⚠️ Via Keycloak | PARTIAL | Needs Keycloak config |
| `GET /api/auth/profile` | ⚠️ Via Keycloak | PARTIAL | Needs Keycloak config |

**Action Required**: Configure Keycloak realm and update frontend to use Keycloak OIDC

---

### **Category 2: Dashboard APIs**

| Frontend Expects | Backend Provides | Status | Notes |
|------------------|------------------|--------|-------|
| `GET /api/dashboard/stats` | ❌ Not found | MISSING | Need to implement |
| `GET /api/system/status` | ✅ `/health` | AVAILABLE | Map to /health |
| `GET /api/bpci/system/status` | ✅ `/api/economy/status` | AVAILABLE | Real autonomous economy data |

**Action Required**: 
- Map `/api/dashboard/stats` → aggregate from multiple endpoints
- Map `/api/system/status` → `/health`

---

### **Category 3: Blockchain APIs**

| Frontend Expects | Backend Provides | Status | Notes |
|------------------|------------------|--------|-------|
| `GET /api/network/info` | ✅ `/blockchain/api/v1/blockchain/info` | AVAILABLE | ✅ |
| `GET /api/blockchain/height` | ✅ `/blockchain/api/v1/blockchain/status` | AVAILABLE | ✅ |
| `GET /api/blockchain/peers` | ✅ `/blockchain/api/v1/network` | AVAILABLE | ✅ |

**Status**: ✅ FULLY COMPATIBLE

---

### **Category 4: Wallet APIs**

| Frontend Expects | Backend Provides | Status | Notes |
|------------------|------------------|--------|-------|
| `GET /api/wallets` | ✅ `/api/wallet/registry` | AVAILABLE | ✅ |
| `POST /api/wallet/create` | ✅ `/api/wallet/register` | AVAILABLE | ✅ |
| `GET /api/wallet/:address/balance` | ✅ `/api/wallet/:address` | AVAILABLE | ✅ |
| `POST /api/wallet/send` | ✅ `/bridge/transaction/process` | AVAILABLE | Via BPI Bridge |

**Status**: ✅ FULLY COMPATIBLE

---

### **Category 5: BPI Core Integration**

| Frontend Expects | Backend Provides | Status | Notes |
|------------------|------------------|--------|-------|
| `GET /api/bpi/status` | ✅ `/bridge/health` | AVAILABLE | BPI Bridge status |
| `POST /api/bpi/connect` | ✅ `/bridge/account/create` | AVAILABLE | ✅ |
| `POST /api/bpi/disconnect` | ❌ Not found | MISSING | Need to implement |

**Action Required**: Implement BPI disconnect endpoint

---

### **Category 6: Developer Environment**

| Frontend Expects | Backend Provides | Status | Notes |
|------------------|------------------|--------|-------|
| `GET /api/dev/profile` | ❌ Not found | MISSING | Need to implement |
| `POST /api/dev/environment` | ❌ Not found | MISSING | Need to implement |
| `POST /api/dev/wallet` | ✅ `/api/wallet/register` | AVAILABLE | Can reuse |
| `GET /api/dev/wallets` | ✅ `/api/wallets` | AVAILABLE | ✅ |

**Action Required**: Implement developer profile endpoints

---

### **Category 7: Test Network Management**

| Frontend Expects | Backend Provides | Status | Notes |
|------------------|------------------|--------|-------|
| `POST /api/testnet/create` | ❌ Not found | MISSING | Need to implement |
| `GET /api/testnet/list` | ❌ Not found | MISSING | Need to implement |
| `POST /api/testnet/:id/start` | ❌ Not found | MISSING | Need to implement |
| `POST /api/testnet/:id/stop` | ❌ Not found | MISSING | Need to implement |

**Action Required**: Implement test network management endpoints

---

### **Category 8: HTTPCG Protocol**

| Frontend Expects | Backend Provides | Status | Notes |
|------------------|------------------|--------|-------|
| `POST /api/httpcg/enable` | ❌ Not found | MISSING | Need to implement |
| `POST /api/httpcg/disable` | ❌ Not found | MISSING | Need to implement |
| `GET /api/httpcg/status` | ❌ Not found | MISSING | Need to implement |

**Action Required**: Implement HTTPCG protocol endpoints

---

### **Category 9: Shadow Registry**

| Frontend Expects | Backend Provides | Status | Notes |
|------------------|------------------|--------|-------|
| `GET /api/shadow/status` | ✅ `/api/shadow/registry` | AVAILABLE | ✅ |
| `POST /api/shadow/register` | ❌ Not found | MISSING | Need to implement |

**Action Required**: Implement shadow registry registration

---

### **Category 10: Registry/Domain APIs**

| Frontend Expects | Backend Provides | Status | Notes |
|------------------|------------------|--------|-------|
| `GET /api/domains` | ✅ `/api/registry/nodes` | AVAILABLE | ✅ |
| `POST /api/domain/register` | ❌ Not found | MISSING | Need to implement |

**Action Required**: Implement domain registration endpoint

---

### **Category 11: Installer APIs**

| Frontend Expects | Backend Provides | Status | Notes |
|------------------|------------------|--------|-------|
| `GET /api/installer/status` | ❌ Not found | MISSING | Need to implement |
| `POST /api/installer/start` | ❌ Not found | MISSING | Need to implement |
| `GET /api/bpi/installer/status` | ❌ Not found | MISSING | Need to implement |
| `GET /api/bpi/installer/download` | ❌ Not found | MISSING | Need to implement |

**Action Required**: Implement installer management endpoints

---

## 📊 COMPATIBILITY SUMMARY

### **Available & Compatible** ✅
- Health check
- Blockchain info & status
- Network information
- Wallet registry
- Wallet operations
- BPI Bridge status
- Basic registry operations

**Total**: 12+ endpoints

### **Partially Available** ⚠️
- Authentication (needs Keycloak config)
- Dashboard stats (needs aggregation)
- System status (needs mapping)

**Total**: 3 endpoints

### **Missing** ❌
- Developer profile management
- Test network management
- HTTPCG protocol control
- Shadow registry registration
- Domain registration
- Installer management
- BPI disconnect

**Total**: 15+ endpoints

---

## 🔧 REQUIRED ACTIONS

### **Priority 1: Configuration Update** (5 min)

Update frontend configuration to point to testnet:

```typescript
// src/services/bpciApi.ts
const BPCI_CONFIG = {
  BPCI_SERVER: 'http://134.209.210.181',
  BPI_CORE_SERVER: 'http://134.209.210.181',
  ADMIN_DASHBOARD: 'http://134.209.210.181',
  WALLET_SERVER: 'http://134.209.210.181',
  RPC_ENDPOINT: 'http://134.209.210.181',
};
```

### **Priority 2: Keycloak Configuration** (10 min)

1. Configure Keycloak realm: `bpci`
2. Create clients: `bpci-frontend`, `bpci-web`
3. Update frontend Keycloak service with correct URLs

### **Priority 3: API Endpoint Mapping** (15 min)

Create API adapter layer to map frontend expectations to backend reality:

```typescript
// src/services/apiAdapter.ts
export const apiAdapter = {
  '/api/dashboard/stats': async () => {
    // Aggregate from multiple endpoints
    const [health, blockchain, bridge] = await Promise.all([
      fetch('/health'),
      fetch('/blockchain/api/v1/blockchain/info'),
      fetch('/bridge/health')
    ]);
    return aggregateStats(health, blockchain, bridge);
  },
  
  '/api/system/status': () => fetch('/health'),
  
  '/api/network/info': () => fetch('/blockchain/api/v1/blockchain/info'),
  
  // ... more mappings
};
```

### **Priority 4: Implement Missing Endpoints** (1-2 hours)

Add missing endpoints to backend:
- Developer profile management
- Test network management
- HTTPCG protocol control
- Installer management

---

## 🚀 DEPLOYMENT STRATEGY

### **Phase 1: Quick Deploy** (30 min)
1. Update frontend config to testnet server
2. Build frontend: `npm run build`
3. Deploy dist folder to `/var/www/html`
4. Test basic functionality

### **Phase 2: Keycloak Integration** (30 min)
1. Configure Keycloak realm
2. Update frontend Keycloak config
3. Test authentication flow

### **Phase 3: API Adapter** (1 hour)
1. Create API adapter layer
2. Map existing endpoints
3. Test all mapped endpoints

### **Phase 4: Missing Endpoints** (2-3 hours)
1. Implement missing backend endpoints
2. Test each endpoint
3. Update frontend to use new endpoints

---

## 📝 FRONTEND BUILD & DEPLOY COMMANDS

```bash
# Navigate to frontend
cd /home/umesh/metanode/bpci-enterprise/website/bpci-enterprise-website

# Install dependencies (if needed)
npm install

# Update config for testnet
# Edit src/services/bpciApi.ts

# Build for production
npm run build

# Deploy to server
scp -r dist/* root@134.209.210.181:/var/www/html/

# Test deployment
curl http://134.209.210.181/
```

---

## 🎯 NEXT STEPS

1. **Update frontend configuration** to point to testnet
2. **Build and deploy** React frontend
3. **Configure Keycloak** manually
4. **Create API adapter** for endpoint mapping
5. **Implement missing endpoints** as needed
6. **Test end-to-end** functionality

---

## 📊 COMPATIBILITY SCORE

**Current**: 40% (12/30 endpoints)  
**With Keycloak**: 50% (15/30 endpoints)  
**With Adapter**: 70% (21/30 endpoints)  
**Full Implementation**: 100% (30/30 endpoints)

---

**Status**: Ready to proceed with Phase 1 (Quick Deploy)  
**Estimated Time**: 30 minutes for basic deployment  
**Full Integration**: 4-5 hours for complete implementation
