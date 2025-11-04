# BPCI ENTERPRISE DEPLOYMENT PLAN - PART 3: CORE SERVICES DEPLOYMENT

**Date**: 2025-10-30  
**Status**: CORE SERVICES DEPLOYMENT  
**Phase**: Phase 2 (Days 6-10)

---

## 🎯 PHASE 2: CORE SERVICES DEPLOYMENT (Days 6-10)

### **Critical Startup Order** ⚠️

Based on code analysis, services MUST start in this order due to dependencies:

```
1. BSO-K8 Orchestrator (Port 9090)
   └─> Provides orchestration API for all other services
   
2. Cluster Ledger Server (Port 7000)
   └─> Core coordination via CommuteLock
   
3. Blockchain Server (Port 8080)
   └─> Depends on Cluster Ledger
   
4. Consensus Server (Port 9001)
   └─> Depends on Blockchain Server
   
5. BPI-BPCI Bridge (Port 6001)
   └─> Depends on Blockchain + Consensus
   
6. Network Server (Port varies)
   └─> Depends on Blockchain
   
7. Auction Mempool Server (Port 7002)
   └─> Depends on Blockchain + Consensus
   
8. Auction DB Maintainer (Background)
   └─> Depends on Auction Mempool
   
9. Shadow Registry Server (Port 8081)
   └─> Depends on Cluster Ledger
   
10. XTMP Server (Port 8889)
    └─> Depends on Cluster Ledger
    
11. BSO-K8 Production Server (Port varies)
    └─> Depends on BSO-K8 Orchestrator
    
12. Mojo Server (Port varies)
    └─> Depends on all core services
    
13. Real Blockchain (Port varies)
    └─> Optional production blockchain
```

---

## 📅 DAY 6: BSO-K8 Orchestrator & Foundation Services

### **Step 6.1: Start BSO-K8 Orchestrator**

```bash
# Start the orchestrator
systemctl start bso-k8-orchestrator

# Wait 10 seconds for initialization
sleep 10

# Check status
systemctl status bso-k8-orchestrator

# Verify logs
tail -f /opt/bpci/logs/orchestrator/bso-k8.log

# Expected output:
# 🚀 Starting BSO-K8 Production Orchestrator for BPCI Enterprise
# 🧬 Revolutionary vPod orchestration with cellular replication
# 🔧 Initializing BSO-K8 orchestrator...
# ✅ BSO-K8 orchestrator initialized successfully
# 🚀 Starting orchestrator services...
# ✅ BSO-K8 orchestrator started successfully
# 🎉 BSO-K8 Production Orchestrator is READY!
```

**Verification**:
```bash
# Check API endpoint
curl http://localhost:9090/health
# Expected: "OK"

# Check orchestrator status
curl http://localhost:9090/api/v1/status | jq
# Expected: JSON with orchestrator state

# Check vPod capacity
# Should show calculated vPods based on 8GB RAM
# Formula: (total_ram_mb - 2048) / 512 = (8192 - 2048) / 512 = 12 vPods
```

**Troubleshooting**:
```bash
# If service fails to start:
journalctl -u bso-k8-orchestrator -n 50 --no-pager

# Common issues:
# 1. Shared memory not configured
ls -la /dev/shm/bpci/

# 2. Port already in use
netstat -tulpn | grep 9090

# 3. Permissions
ls -la /opt/bpci/bin/bso_k8_production_orchestrator
```

---

### **Step 6.2: Start Cluster Ledger Server**

```bash
# Start cluster ledger
systemctl start bpci-cluster-ledger

# Wait 15 seconds (this is a large service)
sleep 15

# Check status
systemctl status bpci-cluster-ledger

# Verify logs
tail -f /opt/bpci/logs/servers/cluster-ledger.log

# Expected output:
# 🚀 Starting BPCI Cluster Ledger Server
# 📊 CommuteLock integration enabled
# ✅ Cluster ledger initialized
# 🔗 Listening on port 7000
```

**Verification**:
```bash
# Check port
netstat -tulpn | grep 7000

# Check CommuteLock integration
ls -la /dev/shm/bpci/components/ledger/

# Test API endpoint
curl http://localhost:7000/health
```

**Resource Monitoring**:
```bash
# Monitor resource usage
htop

# Expected resource usage:
# BSO-K8 Orchestrator: ~200-300MB RAM, 5-10% CPU
# Cluster Ledger: ~400-600MB RAM, 10-15% CPU
# Total so far: ~600-900MB RAM
```

---

### **Step 6.3: CommuteLock Verification**

```bash
# Verify CommuteLock is working
ls -la /dev/shm/bpci/components/

# Should show:
# drwxrwxrwx ledger/
# drwxrwxrwx orchestrator/

# Check lock files
ls -la /dev/shm/bpci/components/ledger/
# Should show:
# -rw-rw-rw- lock
# -rw-rw-rw- data

# Test CommuteLock communication
# (This will be tested automatically by services)
```

---

## 📅 DAY 7: Blockchain Core Services

### **Step 7.1: Start Blockchain Server**

```bash
# Start blockchain server
systemctl start bpci-blockchain

# Wait 20 seconds (blockchain initialization)
sleep 20

# Check status
systemctl status bpci-blockchain

# Verify logs
tail -f /opt/bpci/logs/servers/blockchain.log

# Expected output:
# 🚀 Starting BPCI Blockchain Server
# 📦 Initializing blockchain state
# 🔗 Connecting to cluster ledger
# ✅ Blockchain server ready
# 🌐 Listening on port 8080
```

**Verification**:
```bash
# Check blockchain API
curl http://localhost:8080/api/v1/status | jq

# Expected response:
# {
#   "status": "running",
#   "block_height": 0,
#   "peers": 0,
#   "syncing": false
# }

# Check resource usage
ps aux | grep bpci_blockchain_server
# Expected: ~500-800MB RAM
```

---

### **Step 7.2: Start Consensus Server**

```bash
# Start consensus server
systemctl start bpci-consensus

# Wait 10 seconds
sleep 10

# Check status
systemctl status bpci-consensus

# Verify logs
tail -f /opt/bpci/logs/servers/consensus.log

# Expected output:
# 🚀 Starting BPCI Consensus Server
# 🔐 IBFT consensus mechanism
# 🔗 Connecting to blockchain server
# ✅ Consensus server ready
# 🌐 Listening on port 9001
```

**Verification**:
```bash
# Check consensus API
curl http://localhost:9001/api/v1/status | jq

# Check validator status
curl http://localhost:9001/api/v1/validators | jq

# Resource check
ps aux | grep bpci-consensus-server
# Expected: ~200-400MB RAM
```

---

### **Step 7.3: Verify Blockchain + Consensus Integration**

```bash
# Check if consensus is coordinating with blockchain
curl http://localhost:8080/api/v1/consensus/status | jq

# Expected:
# {
#   "consensus_active": true,
#   "validators": 1,
#   "current_round": 0
# }

# Check CommuteLock communication
ls -la /dev/shm/bpci/components/
# Should now show:
# drwxrwxrwx blockchain/
# drwxrwxrwx consensus/
# drwxrwxrwx ledger/
# drwxrwxrwx orchestrator/
```

---

## 📅 DAY 8: Bridge & Integration Services

### **Step 8.1: Start BPI-BPCI Bridge**

```bash
# Start bridge
systemctl start bpci-bpi-bridge

# Wait 10 seconds
sleep 10

# Check status
systemctl status bpci-bpi-bridge

# Verify logs
tail -f /opt/bpci/logs/servers/bpi-bridge.log

# Expected output:
# 🚀 Starting BPI-BPCI Bridge
# 🌉 Connecting to blockchain and consensus
# ✅ Bridge initialized
# 🔗 Listening on port 6001
```

**Verification**:
```bash
# Check bridge API
curl http://localhost:6001/api/v1/status | jq

# Check bridge connections
curl http://localhost:6001/api/v1/connections | jq

# Expected:
# {
#   "blockchain_connected": true,
#   "consensus_connected": true,
#   "bpi_connected": false  # Will be true when BPI Core is deployed
# }
```

---

### **Step 8.2: Start Network Server**

```bash
# Start network server
systemctl start bpci-network

# Wait 10 seconds
sleep 10

# Check status
systemctl status bpci-network

# Verify logs
tail -f /opt/bpci/logs/servers/network.log
```

**Verification**:
```bash
# Check network status
curl http://localhost:<network-port>/api/v1/status | jq

# Check peer connections
curl http://localhost:<network-port>/api/v1/peers | jq
```

---

### **Step 8.3: Resource Check**

```bash
# Check total resource usage
free -h
# Expected: ~2-3GB RAM used (out of 8GB)

# Check CPU usage
top -bn1 | grep "Cpu(s)"
# Expected: 20-40% CPU usage

# Check disk usage
df -h /opt/bpci
# Expected: ~5-10GB used

# Check all services
systemctl list-units --type=service | grep bpci
# Should show 6 services running
```

---

## 📅 DAY 9: Economic & Auction Services

### **Step 9.1: Start Auction Mempool Server**

```bash
# Start auction mempool
systemctl start bpci-auction-mempool

# Wait 10 seconds
sleep 10

# Check status
systemctl status bpci-auction-mempool

# Verify logs
tail -f /opt/bpci/logs/servers/auction-mempool.log

# Expected output:
# 🚀 Starting BPCI Auction Mempool Server
# 💰 Initializing auction mempool
# 🔗 Connecting to blockchain and consensus
# ✅ Auction mempool ready
# 🌐 Listening on port 7002
```

**Verification**:
```bash
# Check auction API
curl http://localhost:7002/api/v1/status | jq

# Check mempool stats
curl http://localhost:7002/api/v1/mempool/stats | jq

# Expected:
# {
#   "pending_transactions": 0,
#   "total_auctions": 0,
#   "active_auctions": 0
# }
```

---

### **Step 9.2: Start Auction DB Maintainer**

```bash
# Start auction DB maintainer (background service)
systemctl start bpci-auction-db-maintainer

# Wait 5 seconds
sleep 5

# Check status
systemctl status bpci-auction-db-maintainer

# Verify logs
tail -f /opt/bpci/logs/servers/auction-db-maintainer.log
```

---

## 📅 DAY 10: Registry & Protocol Services

### **Step 10.1: Start Shadow Registry Server**

```bash
# Start shadow registry
systemctl start bpci-shadow-registry

# Wait 10 seconds
sleep 10

# Check status
systemctl status bpci-shadow-registry

# Verify logs
tail -f /opt/bpci/logs/servers/shadow-registry.log

# Expected output:
# 🚀 Starting BPCI Shadow Registry Server
# 🌐 Web2-Web3 bridge initialization
# ✅ Shadow registry ready
# 🔗 Listening on port 8081
```

**Verification**:
```bash
# Check shadow registry API
curl http://localhost:8081/api/v1/status | jq

# Check registered services
curl http://localhost:8081/api/v1/services | jq
```

---

### **Step 10.2: Start XTMP Server**

```bash
# Start XTMP server
systemctl start bpci-xtmp

# Wait 10 seconds
sleep 10

# Check status
systemctl status bpci-xtmp

# Verify logs
tail -f /opt/bpci/logs/servers/xtmp.log

# Expected output:
# 🚀 Starting BPCI XTMP Server
# 📡 Cross-transport messaging protocol
# ✅ XTMP server ready
# 🔗 Listening on port 8889
```

---

### **Step 10.3: Start Remaining Services**

```bash
# Start BSO-K8 production server
systemctl start bso-k8-production-server

# Start Mojo server
systemctl start bpci-mojo

# Optional: Start real blockchain
systemctl start bpci-real-blockchain

# Wait 30 seconds for all services to initialize
sleep 30
```

---

## ✅ PHASE 2 COMPLETION VERIFICATION

### **All Services Status Check**

```bash
# Check all BPCI services
systemctl list-units --type=service | grep bpci

# Expected output (all should be "active (running)"):
# bso-k8-orchestrator.service    loaded active running
# bpci-cluster-ledger.service    loaded active running
# bpci-blockchain.service        loaded active running
# bpci-consensus.service         loaded active running
# bpci-bpi-bridge.service        loaded active running
# bpci-network.service           loaded active running
# bpci-auction-mempool.service   loaded active running
# bpci-auction-db-maintainer.service loaded active running
# bpci-shadow-registry.service   loaded active running
# bpci-xtmp.service              loaded active running
# bso-k8-production-server.service loaded active running
# bpci-mojo.service              loaded active running
# bpci-real-blockchain.service   loaded active running
```

### **Resource Usage Check**

```bash
# Check total resource usage
free -h
# Expected: 4-6GB RAM used (out of 8GB)

# Check CPU usage
top -bn1 | grep "Cpu(s)"
# Expected: 30-50% CPU usage

# Check disk usage
df -h /opt/bpci
# Expected: 10-15GB used

# Check network connections
netstat -tulpn | grep bpci
# Should show all 13 services listening
```

### **CommuteLock Verification**

```bash
# Check all CommuteLock components
ls -la /dev/shm/bpci/components/

# Expected:
# drwxrwxrwx auction/
# drwxrwxrwx blockchain/
# drwxrwxrwx bridge/
# drwxrwxrwx consensus/
# drwxrwxrwx ledger/
# drwxrwxrwx network/
# drwxrwxrwx orchestrator/
# drwxrwxrwx shadow/
# drwxrwxrwx xtmp/
```

---

## 📋 PHASE 2 COMPLETION CHECKLIST

- [ ] BSO-K8 Orchestrator running (Port 9090)
- [ ] Cluster Ledger Server running (Port 7000)
- [ ] Blockchain Server running (Port 8080)
- [ ] Consensus Server running (Port 9001)
- [ ] BPI-BPCI Bridge running (Port 6001)
- [ ] Network Server running
- [ ] Auction Mempool Server running (Port 7002)
- [ ] Auction DB Maintainer running
- [ ] Shadow Registry Server running (Port 8081)
- [ ] XTMP Server running (Port 8889)
- [ ] BSO-K8 Production Server running
- [ ] Mojo Server running
- [ ] Real Blockchain running (optional)
- [ ] All services communicating via CommuteLock
- [ ] Resource usage within acceptable limits
- [ ] All API endpoints responding

**Estimated Time**: 5 days  
**Status**: Ready for Phase 3

---

**NEXT**: Part 4 - Integration Testing & Frontend Deployment
