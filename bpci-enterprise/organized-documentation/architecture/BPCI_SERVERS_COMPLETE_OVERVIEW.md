# BPCI Servers - Complete Overview
## All 14 BPCI Enterprise Servers

**Date:** November 3, 2025  
**Server:** 134.209.210.181  
**Status:** All servers operational

---

## 📊 Complete Server List

| # | Server Name | Port | Process ID | Status | API Gateway |
|---|-------------|------|------------|--------|-------------|
| 1 | **API Gateway** | 3001 | 98930 | ✅ Running | ⏳ Not integrated |
| 2 | **BPI Bridge** | 6001 | 72114 | ✅ Running | ⏳ Not integrated |
| 3 | **Cluster Ledger** | 6002 | 97576 | ✅ Running | ✅ **Integrated** |
| 4 | **Auction Mempool** | 7002 | 72193 | ✅ Running | ✅ **Integrated** |
| 5 | **XTMP Server** | 7778 | 469465 | ✅ Running | ✅ **Integrated** |
| 6 | **Consensus Server** | 8080 | 95879 | ✅ Running | ⏳ Not integrated |
| 7 | **Network Server** | 8087 | 72163 | ✅ Running | ✅ **Integrated** |
| 8 | **Shadow Registry** | 8088 | 72143 | ✅ Running | ⏳ Not integrated |
| 9 | **Mojo Server** | 8089 | 72173 | ✅ Running | ⏳ Not integrated |
| 10 | **Blockchain Server** | 9000 | 96797 | ✅ Running | ⏳ Not integrated |
| 11 | **Blockchain Server** | 9002 | 96797 | ✅ Running | ⏳ Not integrated |
| 12 | **Blockchain Server** | 9003 | 96797 | ✅ Running | ⏳ Not integrated |
| 13 | **Auction DB Maintainer** | 9004 | 72132 | ✅ Running | ⏳ Not integrated |
| 14 | **Admin Server** | 9014 | 111673 | ✅ Running | ⏳ Not integrated |
| 15 | **Payment Server** | 9015 | 113082 | ✅ Running | ⏳ Not integrated |

**Total:** 15 servers running (14 BPCI + 1 orchestrator)

---

## 🎯 Integration Status

### **✅ Already Integrated (4 servers):**

1. **Cluster Ledger (6002)** - Consensus and blockchain data
   - API: `https://api.pravyom.com/api/v1/consensus/*`
   - Health: ✅ Healthy
   - Status: Production-ready

2. **Auction Mempool (7002)** - Auction and bidding
   - API: `https://api.pravyom.com/api/v1/auction/*`
   - Health: ✅ Healthy
   - Status: Production-ready

3. **XTMP Server (7778)** - Transaction submission
   - API: `https://api.pravyom.com/api/v1/xtmp/*`
   - Health: ✅ Healthy (TCP-based)
   - Status: Production-ready

4. **Network Server (8087)** - HTTPCG domains and SAPI mesh
   - API: `https://api.pravyom.com/api/v1/domains/*` and `/api/v1/nodes/*`
   - Health: ✅ Healthy
   - Status: Production-ready

---

### **⏳ Need Integration (11 servers):**

#### **High Priority:**

1. **API Gateway (3001)** - Internal BPCI API Gateway
   - Purpose: Unified API for internal BPCI services
   - Endpoints: TBD (need to check)
   - Integration: Add to Cloudflare API Gateway

2. **BPI Bridge (6001)** - BPI node bridge
   - Purpose: Bridge between BPI nodes and BPCI
   - Endpoints: TBD
   - Integration: Critical for BPI connectivity

3. **Consensus Server (8080)** - Consensus protocol
   - Purpose: LCCD consensus implementation
   - Endpoints: TBD
   - Integration: Core blockchain functionality

4. **Shadow Registry (8088)** - Web2/Web3 domain mapping
   - Purpose: Domain registry and mapping
   - Endpoints: TBD
   - Integration: Critical for domain market

5. **Blockchain Server (9000/9002/9003)** - Blockchain data
   - Purpose: Blockchain storage and queries
   - Endpoints: TBD
   - Integration: Core blockchain functionality

#### **Medium Priority:**

6. **Mojo Server (8089)** - Mojo protocol
   - Purpose: Advanced protocol features
   - Endpoints: TBD
   - Integration: Enhanced features

7. **Auction DB Maintainer (9004)** - Auction database
   - Purpose: Auction data persistence
   - Endpoints: TBD
   - Integration: Auction system support

8. **Admin Server (9014)** - Administration
   - Purpose: System administration and monitoring
   - Endpoints: TBD
   - Integration: Admin dashboard

9. **Payment Server (9015)** - Payment processing
   - Purpose: Payment and transaction processing
   - Endpoints: TBD
   - Integration: Payment features

---

## 🔥 Firewall Status

### **✅ Open Ports:**

- 6002 (Cluster Ledger) ✅
- 7002 (Auction Mempool) ✅
- 7778 (XTMP Server) ✅
- 8087 (Network Server) ✅

### **⚠️ Need to Open:**

- 3001 (API Gateway)
- 6001 (BPI Bridge)
- 8080 (Consensus Server)
- 8088 (Shadow Registry)
- 8089 (Mojo Server)
- 9000 (Blockchain Server)
- 9002 (Blockchain Server)
- 9003 (Blockchain Server)
- 9004 (Auction DB Maintainer)
- 9014 (Admin Server)
- 9015 (Payment Server)

---

## 📋 Integration Plan

### **Phase 1: Core Services** ✅ COMPLETE
- ✅ Cluster Ledger (6002)
- ✅ Auction Mempool (7002)
- ✅ XTMP Server (7778)
- ✅ Network Server (8087)

### **Phase 2: Critical Services** 🚀 NEXT
1. **BPI Bridge (6001)** - Critical for BPI connectivity
2. **Shadow Registry (8088)** - Critical for domain market
3. **Consensus Server (8080)** - Core consensus
4. **Blockchain Server (9000)** - Core blockchain data

### **Phase 3: Enhanced Services**
5. **API Gateway (3001)** - Internal API unification
6. **Mojo Server (8089)** - Advanced features
7. **Auction DB Maintainer (9004)** - Auction persistence
8. **Admin Server (9014)** - Administration
9. **Payment Server (9015)** - Payment processing

### **Phase 4: Additional Blockchain Servers**
10. **Blockchain Server (9002)** - Additional blockchain instance
11. **Blockchain Server (9003)** - Additional blockchain instance

---

## 🎯 Next Steps

### **Immediate Actions:**

1. **Check endpoints for each server** - Determine what APIs each server provides
2. **Open firewall ports** - Allow external access to all servers
3. **Add to API Gateway** - Integrate all servers into Cloudflare API Gateway
4. **Test integration** - Verify all endpoints are working
5. **Update documentation** - Document all APIs

### **Integration Commands:**

```bash
# 1. Check server endpoints
for port in 3001 6001 8080 8088 8089 9000 9002 9003 9004 9014 9015; do
  echo "Testing port $port:"
  curl -s http://134.209.210.181:$port/health || echo "No health endpoint"
  echo ""
done

# 2. Open firewall ports
for port in 3001 6001 8080 8088 8089 9000 9002 9003 9004 9014 9015; do
  ufw allow $port/tcp
done

# 3. Update API Gateway Worker
# Add new service configurations to workers/api-gateway.js

# 4. Test all endpoints
curl https://api.pravyom.com/api/v1/health
```

---

## 📊 Server Details

### **1. API Gateway (3001)**
- **Purpose:** Internal BPCI API Gateway
- **Type:** HTTP/REST API
- **Status:** Running
- **Integration:** Not yet integrated

### **2. BPI Bridge (6001)**
- **Purpose:** Bridge between BPI nodes and BPCI
- **Type:** TCP/HTTP hybrid
- **Status:** Running
- **Integration:** Critical for BPI connectivity

### **3. Cluster Ledger (6002)** ✅
- **Purpose:** Consensus and blockchain ledger
- **Type:** HTTP/REST API
- **Status:** Running and integrated
- **API:** `https://api.pravyom.com/api/v1/consensus/*`

### **4. Auction Mempool (7002)** ✅
- **Purpose:** Auction and bidding system
- **Type:** HTTP/REST API
- **Status:** Running and integrated
- **API:** `https://api.pravyom.com/api/v1/auction/*`

### **5. XTMP Server (7778)** ✅
- **Purpose:** Transaction submission via XTMP protocol
- **Type:** TCP-based XTMP protocol
- **Status:** Running and integrated
- **API:** `https://api.pravyom.com/api/v1/xtmp/*`

### **6. Consensus Server (8080)**
- **Purpose:** LCCD consensus implementation
- **Type:** HTTP/REST API
- **Status:** Running
- **Integration:** High priority

### **7. Network Server (8087)** ✅
- **Purpose:** HTTPCG domains and SAPI mesh
- **Type:** HTTP/REST API
- **Status:** Running and integrated
- **API:** `https://api.pravyom.com/api/v1/domains/*` and `/api/v1/nodes/*`

### **8. Shadow Registry (8088)**
- **Purpose:** Web2/Web3 domain mapping
- **Type:** HTTP/REST API
- **Status:** Running
- **Integration:** Critical for domain market

### **9. Mojo Server (8089)**
- **Purpose:** Advanced Mojo protocol features
- **Type:** HTTP/REST API
- **Status:** Running
- **Integration:** Enhanced features

### **10-12. Blockchain Server (9000/9002/9003)**
- **Purpose:** Blockchain data storage and queries
- **Type:** HTTP/REST API
- **Status:** Running (3 instances)
- **Integration:** Core blockchain functionality

### **13. Auction DB Maintainer (9004)**
- **Purpose:** Auction database maintenance
- **Type:** Background service with API
- **Status:** Running
- **Integration:** Auction system support

### **14. Admin Server (9014)**
- **Purpose:** System administration and monitoring
- **Type:** HTTP/REST API
- **Status:** Running
- **Integration:** Admin dashboard

### **15. Payment Server (9015)**
- **Purpose:** Payment and transaction processing
- **Type:** HTTP/REST API
- **Status:** Running
- **Integration:** Payment features

---

## 🎯 Recommended Integration Order

1. **BPI Bridge (6001)** - Critical for BPI node connectivity
2. **Shadow Registry (8088)** - Critical for domain market
3. **Consensus Server (8080)** - Core consensus functionality
4. **Blockchain Server (9000)** - Core blockchain data
5. **Admin Server (9014)** - System monitoring
6. **Payment Server (9015)** - Payment processing
7. **API Gateway (3001)** - Internal API unification
8. **Mojo Server (8089)** - Advanced features
9. **Auction DB Maintainer (9004)** - Auction persistence
10. **Blockchain Servers (9002/9003)** - Additional instances

---

## 📞 Support

For more information about each server, check:
- Server logs: `journalctl -u <service_name>`
- Process info: `ps aux | grep bpci`
- Port info: `netstat -tlnp | grep <port>`

---

**Last Updated:** November 3, 2025  
**Status:** 4/15 servers integrated into API Gateway  
**Next:** Integrate remaining 11 servers
