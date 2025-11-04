#!/bin/bash
# BPCI Enterprise - Phase 1: Initial Setup
# Server: 134.209.210.181
# Date: 2025-10-30

set -e  # Exit on error

echo "=========================================="
echo "BPCI ENTERPRISE - PHASE 1: INITIAL SETUP"
echo "=========================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Step 1: System Update
log_info "Step 1: Updating system packages..."
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq
apt-get upgrade -y -qq -o Dpkg::Options::="--force-confdef" -o Dpkg::Options::="--force-confold"
log_success "System updated successfully"

# Step 2: Install Essential Tools
log_info "Step 2: Installing essential tools..."
DEBIAN_FRONTEND=noninteractive apt-get install -y -qq \
    curl \
    wget \
    git \
    vim \
    htop \
    net-tools \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
    gnupg \
    lsb-release \
    ufw \
    fail2ban
log_success "Essential tools installed"

# Step 3: Configure Firewall
log_info "Step 3: Configuring firewall..."
ufw --force reset
ufw default deny incoming
ufw default allow outgoing
ufw allow ssh
ufw allow 80/tcp    # HTTP
ufw allow 443/tcp   # HTTPS
ufw allow 8080/tcp  # Blockchain Server
ufw allow 9001/tcp  # Consensus Server
ufw allow 7000/tcp  # Cluster Ledger
ufw allow 8889/tcp  # XTMP Server
ufw allow 8180/tcp  # Keycloak
ufw --force enable
log_success "Firewall configured"

# Step 4: Configure System Limits
log_info "Step 4: Configuring system limits..."
cat >> /etc/security/limits.conf << EOF

# BPCI Enterprise - Increased limits
* soft nofile 65536
* hard nofile 65536
* soft nproc 32768
* hard nproc 32768
root soft nofile 65536
root hard nofile 65536
root soft nproc 32768
root hard nproc 32768
EOF

# Update sysctl for better performance
cat >> /etc/sysctl.conf << EOF

# BPCI Enterprise - Performance tuning
fs.file-max = 2097152
net.core.somaxconn = 65535
net.ipv4.tcp_max_syn_backlog = 8192
net.core.netdev_max_backlog = 5000
vm.swappiness = 10
EOF

sysctl -p > /dev/null 2>&1
log_success "System limits configured"

# Step 5: Create Directory Structure
log_info "Step 5: Creating directory structure..."
mkdir -p /opt/bpci/{bin,config,data,logs}
mkdir -p /opt/bpci/data/{postgresql,redis,keycloak,blockchain}
mkdir -p /var/www/bpci-frontend
mkdir -p /etc/nginx/sites-available
mkdir -p /etc/nginx/sites-enabled
log_success "Directory structure created"

# Step 6: Setup /dev/shm for CommuteLock
log_info "Step 6: Setting up CommuteLock shared memory..."
mkdir -p /dev/shm/bpci
chmod 777 /dev/shm/bpci

# Make it persistent
if ! grep -q "/dev/shm/bpci" /etc/fstab; then
    echo "tmpfs /dev/shm/bpci tmpfs defaults,size=2G,mode=0777 0 0" >> /etc/fstab
fi
log_success "CommuteLock shared memory configured"

# Step 7: Install Rust (for building BPCI)
log_info "Step 7: Installing Rust..."
if ! command -v rustc &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    log_success "Rust installed successfully"
else
    log_success "Rust already installed"
fi

# Step 8: System Information
log_info "Step 8: Gathering system information..."
echo ""
echo "=========================================="
echo "SYSTEM INFORMATION"
echo "=========================================="
echo "Hostname: $(hostname)"
echo "OS: $(lsb_release -d | cut -f2)"
echo "Kernel: $(uname -r)"
echo "CPU Cores: $(nproc)"
echo "Total RAM: $(free -h | awk '/^Mem:/ {print $2}')"
echo "Available RAM: $(free -h | awk '/^Mem:/ {print $7}')"
echo "Disk Space: $(df -h / | awk 'NR==2 {print $4}') available"
echo "=========================================="
echo ""

# Step 9: Create deployment user (optional, running as root for now)
log_info "Step 9: Creating deployment user..."
if ! id -u bpci &> /dev/null; then
    useradd -m -s /bin/bash bpci
    usermod -aG sudo bpci
    log_success "User 'bpci' created"
else
    log_success "User 'bpci' already exists"
fi

# Step 10: Install Node.js (for frontend build)
log_info "Step 10: Installing Node.js..."
if ! command -v node &> /dev/null; then
    curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
    apt-get install -y nodejs
    log_success "Node.js $(node --version) installed"
else
    log_success "Node.js already installed: $(node --version)"
fi

echo ""
echo "=========================================="
echo "PHASE 1 COMPLETE!"
echo "=========================================="
echo ""
log_success "Initial setup completed successfully"
log_info "Server is ready for Phase 2: Infrastructure Installation"
echo ""
echo "Next steps:"
echo "  1. Run deploy_phase2_infrastructure.sh"
echo "  2. Install Docker, Nginx, PostgreSQL, Redis, Keycloak"
echo ""
