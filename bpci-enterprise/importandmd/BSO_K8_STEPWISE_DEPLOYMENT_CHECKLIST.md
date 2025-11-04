# BSO-K8 Stepwise Deployment & Testing Checklist
## Systematic Deployment to Avoid Chaos

### 🔍 **Phase 1: Instance Configuration Validation**

#### **Current System Analysis**
```bash
# What we currently have running:
- Port 3000: Docker proxy (Website)
- Port 8080: BPCI node
- Port 8545: Pravyom Enterprise
- Nginx: HTTPCG configuration enabled
- Missing: Ports 7777, 7778, 8888 (HTTPCG services)
```

#### **Instance Configuration Validation Checklist**
- [ ] **Instance 1 (Current System)**: Validate 2GB RAM allocation
  - [ ] Check current memory usage: `free -h`
  - [ ] Check CPU usage: `top`
  - [ ] Validate BSO-K8 can fit in remaining resources
  - [ ] Calculate vPod allocation (target: 20 vPods × 8MB = 160MB)

- [ ] **Instance 2 (Database)**: Plan 4GB RAM allocation
  - [ ] Identify database requirements
  - [ ] Plan MongoDB, LCCD, 4D DB allocation
  - [ ] Calculate BSO-K8 database controller resources

- [ ] **Instance 3 (BPI Downloader)**: Plan 2GB RAM allocation
  - [ ] Design BPI downloader service
  - [ ] Plan BSO-K8 downloader controller
  - [ ] Calculate download cache requirements

- [ ] **Instance 4 (Advanced Infrastructure)**: Plan 4GB RAM allocation
  - [ ] Plan Neural Blockchain cluster
  - [ ] Plan LCCD consensus
  - [ ] Plan Shadow Registry integration

### 🚀 **Phase 2: Sequential Service Deployment**

#### **Step 1: Start BSO-K8 Controller (Instance 1)**
```bash
# Commands to run:
cd /home/umesh/metanode/bpci-enterprise
cargo build --release --bin bso_k8_controller
./target/release/bso_k8_controller --config config/bso-k8-instance1.toml
```

**Testing Checklist:**
- [ ] BSO-K8 controller starts without errors
- [ ] vPod arena initializes (20 vPods, 256MB total)
- [ ] Memory usage stays under 512MB
- [ ] Health endpoint responds: `curl http://localhost:9090/health`
- [ ] vPod status endpoint: `curl http://localhost:9090/vpods/status`

#### **Step 2: Start HTTPCG VM Server (Port 7777)**
```bash
# Commands to run:
cargo build --release --bin vm_server
./target/release/vm_server --port 7777 --bso-k8-endpoint http://localhost:9090
```

**Testing Checklist:**
- [ ] VM Server starts and binds to port 7777
- [ ] Connects to BSO-K8 controller successfully
- [ ] HTTPCG protocol endpoints respond
- [ ] Test endpoint: `curl http://localhost:7777/httpcg/health`
- [ ] Nginx proxy routes correctly: `curl http://localhost/httpcg/health`

#### **Step 3: Start HTTPCG Admin Dashboard (Port 8888)**
```bash
# Commands to run:
cargo build --release --bin httpcg_admin_server
./target/release/httpcg_admin_server --port 8888 --vm-endpoint http://localhost:7777
```

**Testing Checklist:**
- [ ] Admin server starts on port 8888
- [ ] Admin dashboard loads: `curl http://localhost:8888/`
- [ ] Shadow Registry integration works
- [ ] Nginx proxy routes: `curl http://localhost/httpcg-admin/`
- [ ] Domain registry functions work

#### **Step 4: Start HTTPCG Wallet System (Port 7778)**
```bash
# Commands to run:
cargo build --release --bin httpcg_wallet_server
./target/release/httpcg_wallet_server --port 7778 --admin-endpoint http://localhost:8888
```

**Testing Checklist:**
- [ ] Wallet server starts on port 7778
- [ ] Wallet dashboard loads: `curl http://localhost:7778/`
- [ ] Economic system integration works
- [ ] Nginx proxy routes: `curl http://localhost/httpcg-wallet/`
- [ ] Autonomous runes system responds

### 🔧 **Phase 3: BSO-K8 Integration Testing**

#### **Step 5: Test BSO-K8 vPod Allocation**
```bash
# Test commands:
curl http://localhost:9090/vpods/allocate -X POST -d '{"service": "httpcg-vm", "memory": "64MB"}'
curl http://localhost:9090/vpods/list
curl http://localhost:9090/arena/stats
```

**Testing Checklist:**
- [ ] vPod allocation works correctly
- [ ] Memory allocation stays within limits
- [ ] Arena statistics show proper usage
- [ ] vPod isolation works (test with multiple allocations)
- [ ] vPod deallocation works properly

#### **Step 6: Test Service Integration**
```bash
# Integration test commands:
curl http://localhost:7777/httpcg/vm/status
curl http://localhost:8888/shadow/registry/status
curl http://localhost:7778/wallet/economic/status
```

**Testing Checklist:**
- [ ] All services communicate properly
- [ ] BSO-K8 orchestration works
- [ ] HTTPCG protocol routing functions
- [ ] Shadow Registry integration complete
- [ ] Economic system integration active

### 🌐 **Phase 4: DNS and External Access**

#### **Step 7: Validate Nginx Configuration**
```bash
# Nginx validation:
sudo nginx -t
sudo systemctl reload nginx
curl -H "Host: pravyom.com" http://localhost/
curl -H "Host: pravyom.com" http://localhost/httpcg/health
```

**Testing Checklist:**
- [ ] Nginx configuration is valid
- [ ] All proxy routes work correctly
- [ ] HTTPCG headers are added properly
- [ ] Domain routing functions correctly
- [ ] SSL/TLS certificates work (if configured)

#### **Step 8: Cloudflare DNS Configuration**
```bash
# DNS validation commands:
dig pravyom.com
dig www.pravyom.com
nslookup pravyom.com
```

**Testing Checklist:**
- [ ] DNS A records point to correct IP
- [ ] Cloudflare proxy is enabled
- [ ] SSL/TLS encryption works
- [ ] CDN caching is configured
- [ ] External access works: `curl https://pravyom.com/health`

### 📊 **Phase 5: Performance and Resource Validation**

#### **Step 9: Resource Usage Validation**
```bash
# Resource monitoring:
free -h
top -p $(pgrep -f "bso_k8\|vm_server\|httpcg")
df -h
netstat -tlnp | grep -E "(7777|7778|8888|9090)"
```

**Testing Checklist:**
- [ ] Total memory usage < 1.5GB (leaving 500MB buffer)
- [ ] CPU usage reasonable under load
- [ ] Disk usage acceptable
- [ ] All required ports are listening
- [ ] No memory leaks detected

#### **Step 10: Load Testing**
```bash
# Basic load testing:
for i in {1..100}; do curl -s http://localhost/health > /dev/null & done
ab -n 1000 -c 10 http://localhost/httpcg/health
```

**Testing Checklist:**
- [ ] System handles 100 concurrent requests
- [ ] Response times remain reasonable
- [ ] No service crashes under load
- [ ] BSO-K8 vPod allocation remains stable
- [ ] Memory usage doesn't spike excessively

### 🎯 **Phase 6: Final Validation**

#### **Step 11: End-to-End Testing**
```bash
# Full system test:
curl https://pravyom.com/
curl https://pravyom.com/httpcg/health
curl https://pravyom.com/httpcg-admin/
curl https://pravyom.com/httpcg-wallet/
```

**Final Checklist:**
- [ ] All services accessible externally
- [ ] HTTPCG protocol fully functional
- [ ] Shadow Registry operational
- [ ] Economic system active
- [ ] BSO-K8 orchestration working
- [ ] Performance meets requirements
- [ ] Resource usage within budget constraints

### 🚨 **Rollback Plan**

If any step fails:
1. **Stop the failing service**: `pkill -f [service_name]`
2. **Check logs**: `journalctl -f` or service-specific logs
3. **Validate configuration**: Check config files and ports
4. **Restart previous working state**: Restart only confirmed working services
5. **Document the issue**: Add to troubleshooting section

### 📝 **Service Management Commands**

#### **Start All Services (After Testing)**
```bash
#!/bin/bash
# start-bso-k8-services.sh

echo "Starting BSO-K8 Cost-Optimized Infrastructure..."

# Start BSO-K8 Controller
./target/release/bso_k8_controller --config config/bso-k8-instance1.toml &
sleep 5

# Start HTTPCG VM Server
./target/release/vm_server --port 7777 --bso-k8-endpoint http://localhost:9090 &
sleep 3

# Start HTTPCG Admin Dashboard
./target/release/httpcg_admin_server --port 8888 --vm-endpoint http://localhost:7777 &
sleep 3

# Start HTTPCG Wallet System
./target/release/httpcg_wallet_server --port 7778 --admin-endpoint http://localhost:8888 &

echo "All BSO-K8 services started. Testing..."
sleep 5

# Basic health checks
curl http://localhost:9090/health
curl http://localhost:7777/httpcg/health
curl http://localhost:8888/
curl http://localhost:7778/

echo "BSO-K8 Infrastructure Ready!"
```

#### **Stop All Services**
```bash
#!/bin/bash
# stop-bso-k8-services.sh

echo "Stopping BSO-K8 services..."
pkill -f "bso_k8_controller"
pkill -f "vm_server"
pkill -f "httpcg_admin_server"
pkill -f "httpcg_wallet_server"
echo "All BSO-K8 services stopped."
```

### 📋 **Current Status Tracking**

**Phase 1: Instance Validation**
- [ ] Instance 1 validation complete
- [ ] Instance 2 planning complete
- [ ] Instance 3 planning complete
- [ ] Instance 4 planning complete

**Phase 2: Service Deployment**
- [ ] BSO-K8 Controller deployed and tested
- [ ] HTTPCG VM Server deployed and tested
- [ ] HTTPCG Admin deployed and tested
- [ ] HTTPCG Wallet deployed and tested

**Phase 3: Integration Testing**
- [ ] vPod allocation tested
- [ ] Service integration tested

**Phase 4: DNS and External Access**
- [ ] Nginx configuration validated
- [ ] Cloudflare DNS configured

**Phase 5: Performance Validation**
- [ ] Resource usage validated
- [ ] Load testing completed

**Phase 6: Final Validation**
- [ ] End-to-end testing completed
- [ ] System ready for production

---

**Next Step**: Start with Phase 1 - Instance Configuration Validation
