# Real Cloud Instances BSO-K8 Deployment Plan
## Actual Infrastructure: 1GB + 4GB + 4GB + 2GB (New)

### 🏗️ **Actual Cloud Instance Configuration**

#### **Instance 1: 1GB RAM (Current System)**
```yaml
Current Role: Frontend/Backend (we're here now)
Available RAM: 1GB total
Current Usage: ~900MB used, ~100MB available
BSO-K8 Allocation: MINIMAL (10 vPods × 6MB = 60MB)

Services to Deploy:
  - BSO-K8 Controller (Minimal mode)
  - HTTPCG Proxy (Lightweight)
  - Basic monitoring
  
Resource Allocation:
  - System: 600MB (current usage)
  - BSO-K8: 60MB (10 vPods × 6MB)
  - Buffer: 340MB (safety margin)
```

#### **Instance 2: 4GB RAM (Database Server)**
```yaml
Role: Primary Database + BSO-K8 Database Controller
Available RAM: 4GB total
BSO-K8 Allocation: 40 vPods × 12MB = 480MB

Services to Deploy:
  - MongoDB (2GB allocation)
  - LCCD Database (512MB)
  - 4D Database + CUE Sync (512MB)
  - BSO-K8 Database Controller (480MB)
  
Resource Allocation:
  - System: 512MB
  - MongoDB: 2GB
  - LCCD + 4D DB: 1GB
  - BSO-K8: 480MB (40 vPods)
  - Buffer: 8MB
```

#### **Instance 3: 4GB RAM (Advanced Infrastructure)**
```yaml
Role: Neural Blockchain + Advanced Services
Available RAM: 4GB total
BSO-K8 Allocation: 60 vPods × 10MB = 600MB

Services to Deploy:
  - Neural Blockchain Cluster (1.5GB)
  - LCCD Consensus (512MB)
  - Shadow Registry (256MB)
  - Mesh Network (256MB)
  - BSO-K8 Advanced Controller (600MB)
  
Resource Allocation:
  - System: 512MB
  - Neural Blockchain: 1.5GB
  - LCCD Consensus: 512MB
  - Other Services: 512MB
  - BSO-K8: 600MB (60 vPods)
  - Buffer: 308MB
```

#### **Instance 4: 2GB RAM (NEW - BPI Downloader)**
```yaml
Role: BPI Core Downloader + HTTPCG Services
Available RAM: 2GB total
BSO-K8 Allocation: 25 vPods × 8MB = 200MB

Services to Deploy:
  - BPI Core Downloader (1GB)
  - HTTPCG VM Server (Port 7777)
  - HTTPCG Admin Dashboard (Port 8888)
  - HTTPCG Wallet System (Port 7778)
  - BSO-K8 Downloader Controller (200MB)
  
Resource Allocation:
  - System: 512MB
  - BPI Downloader: 1GB
  - HTTPCG Services: 288MB
  - BSO-K8: 200MB (25 vPods)
  - Buffer: 0MB (tight fit)
```

### 📊 **Corrected Total Resource Allocation**

**Total BSO-K8 vPods**: 135 vPods (10+40+60+25)
**Total BSO-K8 Memory**: 1.34GB across all instances
**Total Available RAM**: 11GB (1+4+4+2)
**Cost**: Under 100 CAD/month

### 🚀 **Deployment Strategy by Instance**

#### **Phase 1: Instance 1 (1GB) - Minimal BSO-K8**
```bash
# Deploy minimal BSO-K8 on current system
cd /home/umesh/metanode/bpci-enterprise

# Build minimal BSO-K8 controller
cargo build --release --bin bso_k8_controller

# Start with minimal configuration
./target/release/bso_k8_controller \
  --vpods 10 \
  --arena-size 60MB \
  --instance-role frontend \
  --port 9090 &

# Test minimal deployment
curl http://localhost:9090/health
curl http://localhost:9090/vpods/status
```

**Instance 1 Testing Checklist:**
- [ ] BSO-K8 starts with only 60MB memory usage
- [ ] 10 vPods initialize successfully
- [ ] System remains stable with <100MB available RAM
- [ ] Health endpoint responds
- [ ] No memory pressure warnings

#### **Phase 2: Instance 2 (4GB) - Database BSO-K8**
```bash
# SSH to Instance 2 (4GB database server)
ssh instance2

# Deploy database BSO-K8 controller
./target/release/bso_k8_controller \
  --vpods 40 \
  --arena-size 480MB \
  --instance-role database \
  --port 9091 \
  --mongodb-endpoint localhost:27017 &

# Start database services with BSO-K8 integration
./target/release/mongodb_bso_controller --bso-endpoint http://localhost:9091 &
./target/release/lccd_database --bso-endpoint http://localhost:9091 &
```

**Instance 2 Testing Checklist:**
- [ ] BSO-K8 database controller starts (480MB)
- [ ] 40 vPods for database operations
- [ ] MongoDB integration works
- [ ] LCCD database connects
- [ ] 4D database + CUE sync active
- [ ] Cross-instance communication with Instance 1

#### **Phase 3: Instance 3 (4GB) - Advanced Infrastructure**
```bash
# SSH to Instance 3 (4GB advanced infrastructure)
ssh instance3

# Deploy advanced BSO-K8 controller
./target/release/bso_k8_controller \
  --vpods 60 \
  --arena-size 600MB \
  --instance-role advanced \
  --port 9092 \
  --neural-blockchain-enabled \
  --lccd-consensus-enabled &

# Start advanced services
./target/release/neural_blockchain_cluster --bso-endpoint http://localhost:9092 &
./target/release/lccd_consensus_server --bso-endpoint http://localhost:9092 &
./target/release/shadow_registry --bso-endpoint http://localhost:9092 &
```

**Instance 3 Testing Checklist:**
- [ ] BSO-K8 advanced controller starts (600MB)
- [ ] 60 vPods for advanced operations
- [ ] Neural blockchain cluster active
- [ ] LCCD consensus running
- [ ] Shadow Registry operational
- [ ] Mesh network connectivity

#### **Phase 4: Instance 4 (2GB) - BPI Downloader + HTTPCG**
```bash
# Deploy NEW Instance 4 (2GB)
# First create the instance, then:

ssh instance4

# Deploy BPI downloader BSO-K8
./target/release/bso_k8_controller \
  --vpods 25 \
  --arena-size 200MB \
  --instance-role downloader \
  --port 9093 &

# Deploy HTTPCG services (these were missing!)
./target/release/vm_server --port 7777 --bso-endpoint http://localhost:9093 &
./target/release/httpcg_admin_server --port 8888 --vm-endpoint http://localhost:7777 &
./target/release/httpcg_wallet_server --port 7778 --admin-endpoint http://localhost:8888 &

# Deploy BPI downloader
./target/release/bpi_core_downloader --port 11500 --bso-endpoint http://localhost:9093 &
```

**Instance 4 Testing Checklist:**
- [ ] BSO-K8 downloader controller starts (200MB)
- [ ] 25 vPods for downloader operations
- [ ] HTTPCG VM Server active (Port 7777)
- [ ] HTTPCG Admin Dashboard (Port 8888)
- [ ] HTTPCG Wallet System (Port 7778)
- [ ] BPI Core Downloader operational

### 🌐 **Cross-Instance Integration**

#### **BSO-K8 Cluster Configuration**
```toml
# config/bso-k8-cluster.toml
[cluster]
instances = [
  { id = 1, endpoint = "http://instance1:9090", role = "frontend", vpods = 10 },
  { id = 2, endpoint = "http://instance2:9091", role = "database", vpods = 40 },
  { id = 3, endpoint = "http://instance3:9092", role = "advanced", vpods = 60 },
  { id = 4, endpoint = "http://instance4:9093", role = "downloader", vpods = 25 }
]

[networking]
cluster_communication = true
vpod_migration = true
load_balancing = true
```

#### **Nginx Configuration Update (Instance 1)**
```nginx
# Update /etc/nginx/sites-available/httpcg-pravyom
upstream httpcg_vm_cluster {
    server instance4:7777;  # HTTPCG VM Server on Instance 4
}

upstream httpcg_admin_cluster {
    server instance4:8888;  # HTTPCG Admin on Instance 4
}

upstream httpcg_wallet_cluster {
    server instance4:7778;  # HTTPCG Wallet on Instance 4
}

# Update proxy_pass directives to use upstream clusters
location /httpcg/ {
    proxy_pass http://httpcg_vm_cluster;
    # ... rest of config
}

location /httpcg-admin/ {
    proxy_pass http://httpcg_admin_cluster;
    # ... rest of config
}

location /httpcg-wallet/ {
    proxy_pass http://httpcg_wallet_cluster;
    # ... rest of config
}
```

### 🎯 **Deployment Commands (Real Infrastructure)**

```bash
#!/bin/bash
# deploy-real-instances.sh

echo "🚀 Deploying BSO-K8 on Real Cloud Instances..."

# Instance 1 (1GB) - Current system
echo "📱 Instance 1 (1GB): Minimal BSO-K8..."
./target/release/bso_k8_controller --config config/instance1-1gb.toml &
sleep 5

# Instance 2 (4GB) - Database
echo "🗄️ Instance 2 (4GB): Database BSO-K8..."
ssh instance2 './deploy-instance2-database.sh' &

# Instance 3 (4GB) - Advanced
echo "🧠 Instance 3 (4GB): Advanced BSO-K8..."
ssh instance3 './deploy-instance3-advanced.sh' &

# Instance 4 (2GB) - NEW - Downloader + HTTPCG
echo "⬇️ Instance 4 (2GB): Creating and deploying..."
# First create the instance via cloud provider
# Then deploy services
ssh instance4 './deploy-instance4-downloader.sh' &

echo "✅ All instances deploying..."
echo "💰 Total Cost: ~$96 CAD/month"
echo "🔧 Total vPods: 135 across 4 instances"
```

### 📋 **Next Steps**

1. **Start with Instance 1** (current system) - Deploy minimal BSO-K8
2. **Test Instance 1** - Validate 60MB BSO-K8 works in 1GB RAM
3. **Move to Instance 2** - Deploy database BSO-K8 controller
4. **Move to Instance 3** - Deploy advanced infrastructure
5. **Create Instance 4** - Deploy new 2GB instance with HTTPCG services
6. **Update DNS** - Point Cloudflare to Instance 4 for HTTPCG services

**Current Focus**: Start minimal BSO-K8 deployment on Instance 1 (1GB RAM)
