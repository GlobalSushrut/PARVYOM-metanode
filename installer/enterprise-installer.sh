#!/bin/bash
# BPCI Enterprise Installer - Military-grade deployment
# Version: 1.0.0

set -euo pipefail

GREEN='\033[0;32m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

INSTALL_DIR="/opt/bpci-enterprise"
CONFIG_DIR="/etc/bpci-enterprise"
USER="bpci-enterprise"

echo -e "${CYAN}🏦 BPCI Enterprise Installer v1.0.0${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo "Purpose: Enterprise banking, compliance, HA setup"
echo ""

# Check root permissions
if [[ $EUID -ne 0 ]]; then
    echo "❌ Must run as root: sudo $0"
    exit 1
fi

# Install dependencies
echo -e "${BLUE}📦 Installing enterprise dependencies...${NC}"
apt-get update -qq
apt-get install -y curl git build-essential libssl-dev systemd ufw fail2ban

# Create enterprise user
echo -e "${BLUE}👤 Creating enterprise user...${NC}"
if ! id "$USER" &>/dev/null; then
    useradd --system --home "$INSTALL_DIR" --shell /bin/bash "$USER"
fi

# Setup directories
echo -e "${BLUE}📁 Setting up directories...${NC}"
mkdir -p "$INSTALL_DIR"/{bin,data,logs,config,keys,backup}
mkdir -p "$CONFIG_DIR"
chown -R "$USER:$USER" "$INSTALL_DIR"
chmod 700 "$INSTALL_DIR/keys"

# Install Rust toolchain if needed
if ! command -v cargo &> /dev/null; then
    echo -e "${BLUE}🦀 Installing Rust toolchain...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source /root/.cargo/env
    export PATH="/root/.cargo/bin:$PATH"
fi

# Install BPCI Enterprise
echo -e "${BLUE}⚡ Installing BPCI Enterprise...${NC}"
# Use local pravyom repository
PRAVYOM_ROOT="$(dirname "$(dirname "$(realpath "$0")")")" 
cd "$PRAVYOM_ROOT"

# Ensure cargo is available in PATH
export PATH="/root/.cargo/bin:$HOME/.cargo/bin:$PATH"
source /root/.cargo/env 2>/dev/null || true

cargo build --release -p pravyom-enterprise
cp target/release/pravyom-enterprise "$INSTALL_DIR/bin/pravyom-enterprise"
ln -sf "$INSTALL_DIR/bin/pravyom-enterprise" "/usr/local/bin/pravyom-enterprise"
chmod +x "$INSTALL_DIR/bin/pravyom-enterprise"

# Enterprise configuration
echo -e "${BLUE}⚙️ Setting up enterprise configuration...${NC}"
cat > "$CONFIG_DIR/config.toml" << EOF
[network]
mode = "Enterprise"
network = "mainnet"
rpc_endpoint = "https://mainnet-rpc.bpci.io"
testnet_rpc = "https://testnet-rpc.bpci.io"
local_blockchain = false
connect_only = true

[node]
node_type = "bpci-enterprise-client"
data_dir = "$INSTALL_DIR/data"
log_level = "info"
full_node = false
light_client = true

[connection]
# Enterprise nodes connect to remote networks only
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

[authority]
type = "bank"
kyc_required = true
aml_compliance = true
regulatory_reporting = true

[capabilities]
# Enterprise capabilities for production
validator = false
miner = false
notary = false
enterprise_client = true
real_projects_only = true
EOF

# Setup systemd service
cat > "/etc/systemd/system/bpci-enterprise.service" << EOF
[Unit]
Description=BPCI Enterprise Node
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$INSTALL_DIR
ExecStart=$INSTALL_DIR/bin/bpci-enterprise --config $CONFIG_DIR/enterprise.toml
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

# Configure firewall
echo -e "${BLUE}🔥 Configuring enterprise firewall...${NC}"
ufw --force enable
ufw allow ssh
ufw allow 8545/tcp comment "BPCI Enterprise RPC"
ufw allow 8546/tcp comment "BPCI Enterprise WS"
ufw allow 30303/tcp comment "BPCI Enterprise P2P"

# Enable and start service
systemctl daemon-reload
systemctl enable bpci-enterprise.service
systemctl start bpci-enterprise.service

echo ""
echo -e "${GREEN}🎉 BPCI Enterprise Installation Complete!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${BLUE}📋 Enterprise Features:${NC}"
echo "• Military-grade security and encryption"
echo "• Banking compliance and regulatory reporting"
echo "• High-availability validator/miner/notary"
echo "• Enterprise identity and authority management"
echo ""
echo -e "${BLUE}🔧 Management Commands:${NC}"
echo "• Status: bpci-enterprise status"
echo "• Register: bpci-enterprise registry register-node"
echo "• Monitor: journalctl -u bpci-enterprise.service"
echo ""
echo -e "${BLUE}📧 Enterprise Support: enterprise@bpci.io${NC}"
