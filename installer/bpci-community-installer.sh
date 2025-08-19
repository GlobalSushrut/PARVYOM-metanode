#!/bin/bash
# BPCI Community Installer - Community node deployment
# Version: 1.0.0
# Purpose: Community participation and governance setup

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

# Configuration
BPCI_VERSION="1.0.0"
INSTALL_DIR="/opt/bpci"
BIN_DIR="/usr/local/bin"
CONFIG_DIR="/etc/bpci"
SERVICE_DIR="/etc/systemd/system"
USER="bpci"

echo -e "${PURPLE}🏛️ BPCI Community Installer v${BPCI_VERSION}${NC}"
echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo "Purpose: Community node deployment and governance"
echo "Target: Community participants, DAO members, governance"
echo ""

# Check if running as root
check_permissions() {
    if [[ $EUID -ne 0 ]]; then
        echo -e "${RED}❌ This installer must be run as root${NC}"
        echo "Please run: sudo $0"
        exit 1
    fi
}

# System requirements check
check_requirements() {
    echo -e "${YELLOW}🔍 Checking system requirements...${NC}"
    
    # Check OS
    if [[ ! -f /etc/os-release ]]; then
        echo -e "${RED}❌ Cannot determine OS version${NC}"
        exit 1
    fi
    
    # Check memory (minimum 4GB)
    total_mem=$(grep MemTotal /proc/meminfo | awk '{print $2}')
    if [[ $total_mem -lt 4000000 ]]; then
        echo -e "${RED}❌ Insufficient memory. Minimum 4GB required${NC}"
        exit 1
    fi
    
    # Check disk space (minimum 50GB)
    available_space=$(df / | tail -1 | awk '{print $4}')
    if [[ $available_space -lt 50000000 ]]; then
        echo -e "${RED}❌ Insufficient disk space. Minimum 50GB required${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ System requirements met${NC}"
}

# Install dependencies
install_dependencies() {
    echo -e "${YELLOW}📦 Installing dependencies...${NC}"
    
    # Update package list
    apt-get update -qq
    
    # Install required packages
    apt-get install -y \
        curl \
        wget \
        git \
        build-essential \
        pkg-config \
        libssl-dev \
        systemd \
        ufw \
        fail2ban \
        logrotate
    
    # Install Rust for building
    if ! command -v rustc &> /dev/null; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    echo -e "${GREEN}✅ Dependencies installed${NC}"
}

# Create system user
create_user() {
    echo -e "${YELLOW}👤 Creating BPCI user...${NC}"
    
    if ! id "$USER" &>/dev/null; then
        useradd --system --home "$INSTALL_DIR" --shell /bin/bash "$USER"
        usermod -aG systemd-journal "$USER"
    fi
    
    echo -e "${GREEN}✅ BPCI user created${NC}"
}

# Setup directories
setup_directories() {
    echo -e "${YELLOW}📁 Setting up directories...${NC}"
    
    mkdir -p "$INSTALL_DIR"/{bin,data,logs,config,keys}
    mkdir -p "$CONFIG_DIR"
    
    # Set permissions
    chown -R "$USER:$USER" "$INSTALL_DIR"
    chmod 750 "$INSTALL_DIR"
    chmod 700 "$INSTALL_DIR/keys"
    
    echo -e "${GREEN}✅ Directories configured${NC}"
}

# Install BPCI Community Node
install_bpci_node() {
    echo -e "${YELLOW}⚡ Installing BPCI Community Node...${NC}"
    
    # Use local pravyom repository
    PRAVYOM_ROOT="$(dirname "$(dirname "$(realpath "$0")")")" 
    cd "$PRAVYOM_ROOT"
    
    # Build community node
    cargo build --release -p pravyom-enterprise
    
    # Install binary
    cp target/release/pravyom-enterprise "$INSTALL_DIR/bin/pravyom-enterprise"
    ln -sf "$INSTALL_DIR/bin/pravyom-enterprise" "$BIN_DIR/pravyom-community"
    
    # Set permissions
    chown "$USER:$USER" "$INSTALL_DIR/bin/pravyom-enterprise"
    chmod 755 "$INSTALL_DIR/bin/pravyom-enterprise"
    
    echo -e "${GREEN}✅ BPCI Community Node installed${NC}"
}

# Setup community configuration
setup_community_config() {
    echo -e "${YELLOW}⚙️  Setting up community configuration...${NC}"
    
    cat > "$CONFIG_DIR/config.toml" << EOF
[network]
mode = "Community"
network = "mainnet"
rpc_endpoint = "https://mainnet-rpc.bpci.io"
testnet_rpc = "https://testnet-rpc.bpci.io"
local_blockchain = false
connect_only = true

[node]
node_type = "bpci-community-client"
data_dir = "$INSTALL_DIR/data"
log_level = "info"
full_node = false
light_client = true

[connection]
allowed_networks = ["mainnet", "testnet"]
forbidden_networks = ["localhost", "localnet", "devnet"]
remote_only = true

[services]
wallet_service = true
mining_service = false
governance_service = true
notary_service = false
registry_service = true
rpc_service = true
api_service = true

[identity]
auto_register = true
kyc_level = "basic"
governance_participation = true

[capabilities]
app_hosting = true
governance = true
community_services = true

[security]
level = "community"
encryption = "standard"
firewall = true

[governance]
auto_participate = true
voting_power_delegation = false
proposal_threshold = 1000

[economics]
fee_sharing = true
community_rewards = true
staking_enabled = false
EOF
    
    chown "$USER:$USER" "$CONFIG_DIR/config.toml"
    chmod 640 "$CONFIG_DIR/config.toml"
    
    echo -e "${GREEN}✅ Community configuration created${NC}"
}

# Setup systemd service
setup_service() {
    echo -e "${YELLOW}🔧 Setting up systemd service...${NC}"
    
    cat > "$SERVICE_DIR/bpci-community.service" << EOF
[Unit]
Description=BPCI Community Node
After=network.target
Wants=network.target

[Service]
Type=simple
User=$USER
Group=$USER
WorkingDirectory=$INSTALL_DIR
ExecStart=$INSTALL_DIR/bin/bpci-node --config $CONFIG_DIR/community.toml
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=bpci-community

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=$INSTALL_DIR

[Install]
WantedBy=multi-user.target
EOF
    
    # Enable and start service
    systemctl daemon-reload
    systemctl enable bpci-community.service
    
    echo -e "${GREEN}✅ Systemd service configured${NC}"
}

# Setup firewall
setup_firewall() {
    echo -e "${YELLOW}🔥 Configuring firewall...${NC}"
    
    # Enable UFW
    ufw --force enable
    
    # Allow SSH
    ufw allow ssh
    
    # Allow BPCI ports
    ufw allow 8545/tcp comment "BPCI RPC"
    ufw allow 8546/tcp comment "BPCI WS"
    ufw allow 30303/tcp comment "BPCI P2P"
    
    # Allow governance ports
    ufw allow 9090/tcp comment "BPCI Governance"
    
    echo -e "${GREEN}✅ Firewall configured${NC}"
}

# Setup monitoring
setup_monitoring() {
    echo -e "${YELLOW}📊 Setting up monitoring...${NC}"
    
    # Logrotate configuration
    cat > /etc/logrotate.d/bpci-community << EOF
$INSTALL_DIR/logs/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    copytruncate
    su $USER $USER
}
EOF
    
    # Health check script
    cat > "$INSTALL_DIR/bin/health-check.sh" << 'EOF'
#!/bin/bash
# BPCI Community Node Health Check

LOG_FILE="/opt/bpci/logs/health.log"
DATE=$(date '+%Y-%m-%d %H:%M:%S')

# Check if service is running
if systemctl is-active --quiet bpci-community; then
    echo "[$DATE] ✅ Service: Running" >> "$LOG_FILE"
else
    echo "[$DATE] ❌ Service: Stopped" >> "$LOG_FILE"
    systemctl restart bpci-community
fi

# Check network connectivity
if curl -s --max-time 5 https://registry.bpci.io/health > /dev/null; then
    echo "[$DATE] ✅ Network: Connected" >> "$LOG_FILE"
else
    echo "[$DATE] ❌ Network: Disconnected" >> "$LOG_FILE"
fi

# Check disk space
DISK_USAGE=$(df /opt/bpci | tail -1 | awk '{print $5}' | sed 's/%//')
if [ "$DISK_USAGE" -lt 80 ]; then
    echo "[$DATE] ✅ Disk: ${DISK_USAGE}% used" >> "$LOG_FILE"
else
    echo "[$DATE] ⚠️ Disk: ${DISK_USAGE}% used (WARNING)" >> "$LOG_FILE"
fi
EOF
    
    chmod +x "$INSTALL_DIR/bin/health-check.sh"
    chown "$USER:$USER" "$INSTALL_DIR/bin/health-check.sh"
    
    # Add cron job for health checks
    echo "*/5 * * * * $USER $INSTALL_DIR/bin/health-check.sh" > /etc/cron.d/bpci-health
    
    echo -e "${GREEN}✅ Monitoring configured${NC}"
}

# Community registration
community_registration() {
    echo -e "${YELLOW}📝 Community registration...${NC}"
    
    # Generate node identity
    sudo -u "$USER" "$INSTALL_DIR/bin/pravyom-enterprise" --config "$CONFIG_DIR/config.toml" registry create-identity \
        --did "did:bpci:community:$(hostname)" \
        --kyc-level basic \
        --aml-compliant
    
    # Register community node
    sudo -u "$USER" "$INSTALL_DIR/bin/pravyom-enterprise" --config "$CONFIG_DIR/config.toml" registry register-node \
        --node-type bpi-community \
        --did "did:bpci:community:$(hostname)" \
        --endpoint "https://$(hostname).bpci.io:8545" \
        --app-hosting \
        --name "Community Node $(hostname)"
    
    echo -e "${GREEN}✅ Community registration complete${NC}"
}

# Start services
start_services() {
    echo -e "${YELLOW}🚀 Starting services...${NC}"
    
    systemctl start bpci-community.service
    
    # Wait for service to start
    sleep 5
    
    if systemctl is-active --quiet bpci-community; then
        echo -e "${GREEN}✅ BPCI Community Node is running${NC}"
    else
        echo -e "${RED}❌ Failed to start BPCI Community Node${NC}"
        echo "Check logs: journalctl -u bpci-community.service"
        exit 1
    fi
}

# Main installation
main() {
    echo -e "${PURPLE}Starting BPCI Community Installation...${NC}"
    echo ""
    
    check_permissions
    check_requirements
    install_dependencies
    create_user
    setup_directories
    install_bpci_node
    setup_community_config
    setup_service
    setup_firewall
    setup_monitoring
    community_registration
    start_services
    
    echo ""
    echo -e "${GREEN}🎉 BPCI Community Installation Complete!${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo -e "${BLUE}📋 Community Node Status:${NC}"
    echo "• Service: $(systemctl is-active bpci-community.service)"
    echo "• Configuration: $CONFIG_DIR/community.toml"
    echo "• Logs: journalctl -u bpci-community.service"
    echo "• Health: $INSTALL_DIR/bin/health-check.sh"
    echo ""
    echo -e "${BLUE}🗳️ Governance Participation:${NC}"
    echo "• View proposals: bpci-node governance list-proposals"
    echo "• Vote on proposals: bpci-node governance vote <proposal-id> <yes/no>"
    echo "• Create proposals: bpci-node governance create-proposal"
    echo ""
    echo -e "${BLUE}🏠 App Hosting:${NC}"
    echo "• Deploy apps: bpci-node deploy <app-path>"
    echo "• Manage apps: bpci-node app list"
    echo "• Monitor resources: bpci-node node-health"
    echo ""
    echo -e "${YELLOW}📧 Support: community@bpci.io${NC}"
}

# Run main installation
main "$@"
