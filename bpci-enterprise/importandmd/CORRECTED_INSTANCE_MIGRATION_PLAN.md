# Corrected Instance Migration Plan
## Fix Wrong Configuration & Migrate Backend/Frontend

### 🔄 **Current Reality vs Planned Architecture**

#### **CURRENT SITUATION (Wrong Configuration)**
```
Current System (where we are now):
- Instance: 4GB RAM (backend/frontend BPCI testnet)
- Services: pravyom-enterprise (8545), bpci-node (8080), nginx, grafana
- Status: Running but needs to be migrated to 2GB instance

Existing Cloud Instances:
- Instance A: 1GB RAM (unknown usage)
- Instance B: 4GB RAM (unknown usage) 
- Instance C: 4GB RAM (current system - needs migration)
```

#### **CORRECTED ARCHITECTURE (Target)**
```
Instance 1: 2GB RAM (NEW) - Frontend/Backend (migrate current services here)
Instance 2: 4GB RAM - Database Server
Instance 3: 2GB RAM - BPI Downloader  
Instance 4: 4GB RAM - Advanced Infrastructure (current 4GB instance repurposed)
```

### 🚀 **Migration Strategy**

#### **Phase 1: Create New 2GB Instance (Instance 1)**
```bash
# Create new 2GB DigitalOcean droplet for frontend/backend
# This will become Instance 1 in our architecture

Instance 1 Specifications:
- RAM: 2GB
- CPU: 1 vCPU  
- Storage: 25GB SSD
- Cost: ~$18 CAD/month
- Role: Frontend/Backend BPCI Testnet
```

#### **Phase 2: Migrate Services from 4GB to 2GB Instance**
```bash
# Services to migrate:
1. pravyom-enterprise (port 8545)
2. bpci-node (port 8080) 
3. nginx configuration
4. docker containers (port 3000)
5. grafana monitoring
6. BSO-K8 controller (new)

# Migration steps:
1. Setup new 2GB instance
2. Copy configurations and data
3. Test services on new instance
4. Update DNS to point to new instance
5. Shutdown services on old 4GB instance
```

#### **Phase 3: Repurpose Current 4GB Instance as Instance 4**
```bash
# Current 4GB instance becomes Instance 4 (Advanced Infrastructure)
# Deploy advanced services:
1. Neural Blockchain Cluster
2. LCCD Consensus
3. Shadow Registry  
4. HTTPCG Services (7777, 7778, 8888)
5. BSO-K8 Advanced Controller
```

### 📋 **Detailed Migration Steps**

#### **Step 1: Prepare New 2GB Instance (Instance 1)**
```bash
# Create and setup new 2GB droplet
ssh new-instance-1-2gb

# Install dependencies
sudo apt update && sudo apt upgrade -y
sudo apt install -y docker.io nginx redis-server curl wget

# Install Rust and build tools
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone repository
git clone https://github.com/GlobalSushrut/PARVYOM-metanode.git
cd PARVYOM-metanode/bpci-enterprise
```

#### **Step 2: Build Services on New Instance**
```bash
# Build all required binaries
cargo build --release --bin pravyom-enterprise
cargo build --release --bin bpci-node  
cargo build --release --bin bso_k8_controller

# Verify binaries
ls -la target/release/
```

#### **Step 3: Copy Configuration from Current Instance**
```bash
# From current 4GB instance, copy configurations:
scp /etc/parvyom-testnet/config.toml new-instance-1:/etc/parvyom-testnet/
scp /opt/bpci/community.toml new-instance-1:/opt/bpci/
scp -r /etc/nginx/sites-available/ new-instance-1:/etc/nginx/
scp -r docker-compose.yml new-instance-1:~/

# Copy any database files if needed
scp -r /var/lib/bpci/ new-instance-1:/var/lib/
```

#### **Step 4: Deploy BSO-K8 on New 2GB Instance**
```bash
# On new 2GB instance, deploy minimal BSO-K8
cd /home/umesh/metanode/bpci-enterprise

# Create BSO-K8 config for 2GB instance
cat > config/bso-k8-instance1-2gb.toml << EOF
[vpods]
count = 20
memory_per_vpod = "8MB"
arena_size = "160MB"

[instance]
role = "frontend_backend"
port = 9090
memory_limit = "256MB"

[services]
pravyom_enterprise = { port = 8545, memory = "512MB" }
bpci_node = { port = 8080, memory = "256MB" }
nginx = { memory = "128MB" }
EOF

# Start BSO-K8 controller
./target/release/bso_k8_controller --config config/bso-k8-instance1-2gb.toml &
```

#### **Step 5: Start Services on New Instance**
```bash
# Start services with BSO-K8 integration
./target/release/pravyom-enterprise \
  --config /etc/parvyom-testnet/config.toml \
  --network testnet web start \
  --port 8545 --host 0.0.0.0 \
  --bso-k8-endpoint http://localhost:9090 &

./target/release/bpci-node web start \
  --config /opt/bpci/community.toml \
  --bso-k8-endpoint http://localhost:9090 &

# Start nginx
sudo systemctl start nginx
sudo systemctl enable nginx

# Start docker containers
docker-compose up -d
```

#### **Step 6: Test New Instance**
```bash
# Health checks on new 2GB instance
curl http://localhost:9090/health  # BSO-K8
curl http://localhost:8545/health  # Pravyom Enterprise  
curl http://localhost:8080/health  # BPCI Node
curl http://localhost:3000/        # Website
curl http://localhost/health       # Nginx

# Memory usage check
free -h
ps aux --sort=-%mem | head -10
```

#### **Step 7: Update DNS (Cloudflare)**
```bash
# Update Cloudflare DNS to point to new 2GB instance
# A record: pravyom.com -> NEW_INSTANCE_1_IP
# CNAME: www.pravyom.com -> pravyom.com

# Test external access
curl https://pravyom.com/health
curl https://pravyom.com/api/health
```

#### **Step 8: Repurpose Current 4GB Instance (Instance 4)**
```bash
# On current 4GB instance (now Instance 4)
# Stop old services
sudo systemctl stop nginx
pkill -f pravyom-enterprise
pkill -f bpci-node
docker-compose down

# Deploy advanced infrastructure
cargo build --release --bin neural_blockchain_cluster
cargo build --release --bin lccd_consensus_server
cargo build --release --bin shadow_registry
cargo build --release --bin vm_server
cargo build --release --bin httpcg_admin_server
cargo build --release --bin httpcg_wallet_server

# Start BSO-K8 advanced controller
./target/release/bso_k8_controller \
  --vpods 60 \
  --arena-size 600MB \
  --instance-role advanced \
  --port 9092 &

# Start HTTPCG services (finally!)
./target/release/vm_server --port 7777 --bso-endpoint http://localhost:9092 &
./target/release/httpcg_admin_server --port 8888 --vm-endpoint http://localhost:7777 &
./target/release/httpcg_wallet_server --port 7778 --admin-endpoint http://localhost:8888 &

# Start advanced infrastructure
./target/release/neural_blockchain_cluster --bso-endpoint http://localhost:9092 &
./target/release/lccd_consensus_server --bso-endpoint http://localhost:9092 &
./target/release/shadow_registry --bso-endpoint http://localhost:9092 &
```

### 🎯 **Final Architecture After Migration**

```
Instance 1 (NEW 2GB): Frontend/Backend + BSO-K8 (20 vPods)
├── pravyom-enterprise (8545)
├── bpci-node (8080)  
├── nginx proxy
├── docker website (3000)
├── BSO-K8 controller (9090)
└── Cost: $18 CAD/month

Instance 2 (4GB): Database + BSO-K8 (40 vPods)
├── MongoDB
├── LCCD Database
├── 4D Database + CUE Sync
├── BSO-K8 database controller (9091)
└── Cost: $36 CAD/month

Instance 3 (2GB): BPI Downloader + BSO-K8 (25 vPods)  
├── BPI Core Downloader
├── Download cache
├── BSO-K8 downloader controller (9093)
└── Cost: $18 CAD/month

Instance 4 (CURRENT 4GB): Advanced Infrastructure + BSO-K8 (60 vPods)
├── Neural Blockchain Cluster
├── LCCD Consensus
├── Shadow Registry
├── HTTPCG VM Server (7777)
├── HTTPCG Admin (8888)
├── HTTPCG Wallet (7778)
├── BSO-K8 advanced controller (9092)
└── Cost: $36 CAD/month

Total: $108 CAD/month (8% over budget - need optimization)
```

### 🔧 **Cost Optimization After Migration**

To get under 100 CAD budget:
1. **Downsize Instance 2** from 4GB to 2GB (-$18 CAD)
2. **Use shared CPU droplets** instead of dedicated (-$8 CAD)
3. **Final cost: $92 CAD/month** ✅

### 📋 **Migration Checklist**

**Pre-Migration:**
- [ ] Create new 2GB instance (Instance 1)
- [ ] Install dependencies and build tools
- [ ] Build all required binaries
- [ ] Copy configurations from current instance

**Migration:**
- [ ] Deploy BSO-K8 on new 2GB instance
- [ ] Start services on new instance
- [ ] Test all endpoints and functionality
- [ ] Update Cloudflare DNS
- [ ] Verify external access works

**Post-Migration:**
- [ ] Repurpose current 4GB instance as Instance 4
- [ ] Deploy advanced infrastructure services
- [ ] Deploy missing HTTPCG services (7777, 7778, 8888)
- [ ] Test cross-instance communication
- [ ] Validate total cost under 100 CAD

**Current Next Step**: Create new 2GB instance and begin migration process
