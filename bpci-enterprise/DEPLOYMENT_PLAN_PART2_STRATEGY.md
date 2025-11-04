# BPCI ENTERPRISE DEPLOYMENT PLAN - PART 2: STEP-BY-STEP STRATEGY

**Date**: 2025-10-30  
**Status**: DEPLOYMENT STRATEGY  
**Target**: Production deployment on $70 CAD/month VPS using BSO-K8

---

## 🎯 DEPLOYMENT STRATEGY OVERVIEW

### **Phased Deployment Approach**
1. **Phase 1**: Infrastructure Setup (Days 1-5)
2. **Phase 2**: Core Services Deployment (Days 6-10)
3. **Phase 3**: Integration & Testing (Days 11-15)
4. **Phase 4**: Frontend & Installer (Days 16-20)
5. **Phase 5**: Production Hardening (Days 21-23)

---

## 📅 PHASE 1: INFRASTRUCTURE SETUP (Days 1-5)

### **Day 1: VPS Provisioning & Base System**

#### **Step 1.1: VPS Selection & Provisioning**
**Recommended Provider**: Hetzner, DigitalOcean, or Vultr
**Specifications**:
```yaml
CPU: 4 vCPUs (AMD EPYC or Intel Xeon)
RAM: 8GB DDR4
Storage: 160GB NVMe SSD
Network: 1Gbps unmetered
OS: Ubuntu 22.04 LTS (minimal)
Cost: ~$50-70 USD/month
```

**Commands**:
```bash
# 1. SSH into VPS
ssh root@<vps-ip>

# 2. Update system
apt update && apt upgrade -y

# 3. Install essential packages
apt install -y build-essential curl wget git vim htop \
    net-tools iptables-persistent ufw fail2ban \
    ca-certificates gnupg lsb-release
```

#### **Step 1.2: Rust Toolchain Installation**
```bash
# Install Rust (required for BPCI compilation)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Verify installation
rustc --version  # Should be 1.70.0+
cargo --version

# Install additional tools
cargo install cargo-watch cargo-edit
```

#### **Step 1.3: System Configuration**
```bash
# 1. Configure shared memory for CommuteLock
echo "tmpfs /dev/shm tmpfs defaults,size=2G 0 0" >> /etc/fstab
mount -o remount /dev/shm

# 2. Increase file descriptors
cat >> /etc/security/limits.conf << EOF
* soft nofile 65536
* hard nofile 65536
* soft nproc 32768
* hard nproc 32768
EOF

# 3. Configure kernel parameters for high-performance networking
cat >> /etc/sysctl.conf << EOF
# Network performance
net.core.rmem_max = 134217728
net.core.wmem_max = 134217728
net.ipv4.tcp_rmem = 4096 87380 67108864
net.ipv4.tcp_wmem = 4096 65536 67108864
net.core.netdev_max_backlog = 5000
net.ipv4.tcp_max_syn_backlog = 8192

# Shared memory for CommuteLock
kernel.shmmax = 2147483648
kernel.shmall = 524288
EOF

sysctl -p
```

---

### **Day 2: Directory Structure & CommuteLock Setup**

#### **Step 2.1: Create Directory Structure**
```bash
# Create BPCI directory structure
mkdir -p /opt/bpci/{bin,config,data,logs,cache}
mkdir -p /opt/bpci/data/{blockchain,ledger,auction,shadow-registry}
mkdir -p /opt/bpci/logs/{servers,orchestrator,commute-lock}
mkdir -p /dev/shm/bpci/{locks,events,data}

# Set permissions
chmod 755 /opt/bpci
chmod 777 /dev/shm/bpci
```

#### **Step 2.2: CommuteLock Infrastructure**
```bash
# Create CommuteLock directories
mkdir -p /dev/shm/bpci/components/{blockchain,consensus,ledger,auction,bridge,xtmp,shadow,network,orchestrator}
mkdir -p /dev/shm/bpci/bpi_addresses

# Create lock files for each component
for component in blockchain consensus ledger auction bridge xtmp shadow network orchestrator; do
    touch /dev/shm/bpci/components/$component/lock
    touch /dev/shm/bpci/components/$component/data
    chmod 666 /dev/shm/bpci/components/$component/*
done
```

#### **Step 2.3: Configuration Files**
```bash
# Create env.ini configuration for TESTNET with 1M+ node support
cat > /opt/bpci/config/env.ini << 'EOF'
[commute_lock]
enabled = true
lock_dir = "/dev/shm/bpci/locks"
data_dir = "/dev/shm/bpci/data"
event_dir = "/dev/shm/bpci/events"
bpi_data_dir = "/dev/shm/bpci/bpi_addresses"

[network]
http_range_start = 18080
http_range_end = 18120
grpc_range_start = 19100
grpc_range_end = 19150
internal_range_start = 25000
internal_range_end = 25100

[memory]
min_constraint_mb = 1024
dev_constraint_mb = 2048
adaptive_scaling = true

[storage]
docklock_root = "/opt/bpci/data/docklock"
enc_root = "/opt/bpci/data/enc"
cache_root = "/opt/bpci/cache"
logs_root = "/opt/bpci/logs"

# TESTNET CONFIGURATION for 1M+ BPIOS nodes
[testnet]
enabled = true
mode = "testnet"
mock_auctions = true
simulate_community_bidding = true
world_testnet_mode = true

# SCALING CONFIGURATION for 1M+ nodes
[cluster_ledger]
max_bpi_nodes = 1000000
batch_processing_size = 10000
concurrent_pipeline_workers = 100
vpod_allocation_strategy = "cellular"
cellular_replication = true

[bpi_bridge]
address_pool_size = 1000000
cellular_replication = true
auto_scale = true

# CELLULAR REPLICATION (BSO Growth)
[cellular_replication]
enabled = true
replication_factor = 2
growth_pattern = "organic"
cpu_threshold = 70
memory_threshold = 80
connection_threshold = 10000
EOF
```

---

### **Day 3: BPCI Codebase Deployment**

#### **Step 3.1: Clone Repository**
```bash
cd /opt
git clone <bpci-enterprise-repo-url> bpci-source
cd bpci-source

# Or copy from local machine
# scp -r /home/umesh/metanode/bpci-enterprise root@<vps-ip>:/opt/bpci-source
```

#### **Step 3.2: Compilation**
```bash
cd /opt/bpci-source

# Build all binaries (this will take 20-30 minutes)
cargo build --release --bins

# Verify binaries
ls -lh target/release/ | grep bpci_

# Expected output:
# bpci_blockchain_server
# bpci_consensus_server (bpci-consensus-server)
# bpci_cluster_ledger_server
# bpci_auction_mempool_server
# bpci_auction_db_maintainer
# bpci_bpi_bridge
# bpci_shadow_registry_server
# bpci_xtmp_server
# bpci_network_server
# bpci_real_blockchain
# bso_k8_production_orchestrator
# bso_k8_production_server
# bpci_mojo_server
# bpios
```

#### **Step 3.3: Install Binaries**
```bash
# Copy binaries to /opt/bpci/bin
cp target/release/bpci_* /opt/bpci/bin/
cp target/release/bso_k8_* /opt/bpci/bin/
cp target/release/bpios /opt/bpci/bin/

# Make executable
chmod +x /opt/bpci/bin/*

# Verify
ls -lh /opt/bpci/bin/
```

---

### **Day 4: Systemd Service Configuration**

#### **Step 4.1: Create Systemd Services**

**1. BSO-K8 Orchestrator Service**
```bash
cat > /etc/systemd/system/bso-k8-orchestrator.service << 'EOF'
[Unit]
Description=BSO-K8 Production Orchestrator
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/bpci
Environment="RUST_LOG=info"
Environment="BPCI_CONFIG=/opt/bpci/config/env.ini"
ExecStart=/opt/bpci/bin/bso_k8_production_orchestrator \
    --orchestrator-id=bpci-prod-001 \
    --port=9090 \
    --enable-cellular
Restart=always
RestartSec=10
StandardOutput=append:/opt/bpci/logs/orchestrator/bso-k8.log
StandardError=append:/opt/bpci/logs/orchestrator/bso-k8-error.log

[Install]
WantedBy=multi-user.target
EOF
```

**2. Cluster Ledger Server Service**
```bash
cat > /etc/systemd/system/bpci-cluster-ledger.service << 'EOF'
[Unit]
Description=BPCI Cluster Ledger Server
After=network.target bso-k8-orchestrator.service
Requires=bso-k8-orchestrator.service

[Service]
Type=simple
User=root
WorkingDirectory=/opt/bpci
Environment="RUST_LOG=info"
Environment="BPCI_CONFIG=/opt/bpci/config/env.ini"
ExecStart=/opt/bpci/bin/bpci_cluster_ledger_server \
    --port=7000 \
    --data-dir=/opt/bpci/data/ledger
Restart=always
RestartSec=10
StandardOutput=append:/opt/bpci/logs/servers/cluster-ledger.log
StandardError=append:/opt/bpci/logs/servers/cluster-ledger-error.log

[Install]
WantedBy=multi-user.target
EOF
```

**3. Blockchain Server Service**
```bash
cat > /etc/systemd/system/bpci-blockchain.service << 'EOF'
[Unit]
Description=BPCI Blockchain Server
After=network.target bpci-cluster-ledger.service
Requires=bpci-cluster-ledger.service

[Service]
Type=simple
User=root
WorkingDirectory=/opt/bpci
Environment="RUST_LOG=info"
Environment="BPCI_CONFIG=/opt/bpci/config/env.ini"
ExecStart=/opt/bpci/bin/bpci_blockchain_server \
    --port=8080 \
    --data-dir=/opt/bpci/data/blockchain
Restart=always
RestartSec=10
StandardOutput=append:/opt/bpci/logs/servers/blockchain.log
StandardError=append:/opt/bpci/logs/servers/blockchain-error.log

[Install]
WantedBy=multi-user.target
EOF
```

**4. Consensus Server Service**
```bash
cat > /etc/systemd/system/bpci-consensus.service << 'EOF'
[Unit]
Description=BPCI Consensus Server
After=network.target bpci-blockchain.service
Requires=bpci-blockchain.service

[Service]
Type=simple
User=root
WorkingDirectory=/opt/bpci
Environment="RUST_LOG=info"
Environment="BPCI_CONFIG=/opt/bpci/config/env.ini"
ExecStart=/opt/bpci/bin/bpci-consensus-server \
    --port=9001
Restart=always
RestartSec=10
StandardOutput=append:/opt/bpci/logs/servers/consensus.log
StandardError=append:/opt/bpci/logs/servers/consensus-error.log

[Install]
WantedBy=multi-user.target
EOF
```

**Continue for all 13 services...**

#### **Step 4.2: Enable Services**
```bash
# Reload systemd
systemctl daemon-reload

# Enable services (will start on boot)
systemctl enable bso-k8-orchestrator
systemctl enable bpci-cluster-ledger
systemctl enable bpci-blockchain
systemctl enable bpci-consensus
# ... enable all 13 services
```

---

### **Day 5: Firewall & Security Configuration**

#### **Step 5.1: UFW Firewall Setup**
```bash
# Reset UFW
ufw --force reset

# Default policies
ufw default deny incoming
ufw default allow outgoing

# Allow SSH
ufw allow 22/tcp

# Allow BPCI ports
ufw allow 6001/tcp  # BPI-BPCI Bridge
ufw allow 7000/tcp  # Cluster Ledger
ufw allow 7002/tcp  # Auction Mempool
ufw allow 8080/tcp  # Blockchain Server
ufw allow 8081/tcp  # Shadow Registry
ufw allow 8889/tcp  # XTMP Server
ufw allow 9001/tcp  # Consensus Server
ufw allow 9090/tcp  # BSO-K8 Orchestrator

# Allow dynamic port ranges (internal only - restrict to VPS IP)
ufw allow from <vps-ip> to any port 18080:18120 proto tcp
ufw allow from <vps-ip> to any port 19100:19150 proto tcp
ufw allow from <vps-ip> to any port 25000:25100 proto tcp

# Enable firewall
ufw --force enable

# Verify
ufw status verbose
```

#### **Step 5.2: Fail2Ban Configuration**
```bash
# Configure fail2ban for SSH protection
cat > /etc/fail2ban/jail.local << 'EOF'
[DEFAULT]
bantime = 3600
findtime = 600
maxretry = 5

[sshd]
enabled = true
port = 22
logpath = /var/log/auth.log
EOF

systemctl restart fail2ban
systemctl enable fail2ban
```

---

## 📋 PHASE 1 COMPLETION CHECKLIST

- [ ] VPS provisioned with correct specifications
- [ ] Base system updated and configured
- [ ] Rust toolchain installed (1.70.0+)
- [ ] Shared memory configured for CommuteLock
- [ ] Directory structure created
- [ ] CommuteLock infrastructure setup
- [ ] Configuration files created (env.ini)
- [ ] BPCI codebase cloned and compiled
- [ ] All 13 binaries installed to /opt/bpci/bin
- [ ] Systemd services created for all components
- [ ] Firewall configured and enabled
- [ ] Fail2ban configured for SSH protection

**Estimated Time**: 5 days  
**Status**: Ready for Phase 2

---

**NEXT**: Part 3 - Core Services Deployment & Startup Sequence
