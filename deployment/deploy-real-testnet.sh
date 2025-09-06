#!/bin/bash
# PARVYOM Metanode Real Testnet Deployment Script
# Native deployment using existing BPCI server infrastructure

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}🚀 PARVYOM Metanode Real Testnet Deployment${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo "Deploying production-grade cloud-hosted testnet"
echo ""

# Configuration
TESTNET_DIR="/opt/parvyom-testnet"
CONFIG_DIR="/etc/parvyom-testnet"
LOG_DIR="/var/log/parvyom-testnet"
SERVICE_NAME="parvyom-testnet"

# Check if running as root
if [[ $EUID -ne 0 ]]; then
    echo -e "${RED}❌ Must run as root: sudo $0${NC}"
    exit 1
fi

# Create directories
echo -e "${YELLOW}📁 Creating testnet directories...${NC}"
mkdir -p "$TESTNET_DIR"
mkdir -p "$CONFIG_DIR"
mkdir -p "$LOG_DIR"

# Copy configuration
echo -e "${YELLOW}⚙️ Setting up configuration...${NC}"
cp cloud-testnet.toml "$CONFIG_DIR/config.toml"

# Copy binaries (already built)
echo -e "${YELLOW}📦 Installing binaries...${NC}"
cp /home/umesh/metanode/target/release/bpci-server "$TESTNET_DIR/" 2>/dev/null || echo "bpci-server binary not found"
cp /home/umesh/metanode/target/release/metanode-validator "$TESTNET_DIR/" 2>/dev/null || echo "validator binary not found"

# Create a simple testnet launcher script
echo -e "${YELLOW}🚀 Creating testnet launcher...${NC}"
cat > "$TESTNET_DIR/start-testnet.sh" << 'EOF'
#!/bin/bash
cd /home/umesh/metanode
export RUST_LOG=info
./target/release/pravyom-enterprise --config /etc/parvyom-testnet/config.toml
EOF
chmod +x "$TESTNET_DIR/start-testnet.sh"

# Create systemd service
echo -e "${YELLOW}🔧 Creating systemd service...${NC}"
cat > /etc/systemd/system/parvyom-testnet.service << EOF
[Unit]
Description=PARVYOM Metanode Testnet
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/home/umesh/metanode
ExecStart=/home/umesh/metanode/target/release/pravyom-enterprise --config /etc/parvyom-testnet/config.toml
Environment=RUST_LOG=info
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF

# Enable and start service
echo -e "${YELLOW}🚀 Starting testnet service...${NC}"
systemctl daemon-reload
systemctl enable $SERVICE_NAME
systemctl start $SERVICE_NAME

echo -e "${GREEN}✅ PARVYOM Metanode Real Testnet Deployed Successfully!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo "Service: $SERVICE_NAME"
echo "Config: $CONFIG_DIR/config.toml"
echo "Logs: journalctl -u $SERVICE_NAME -f"
echo ""
echo "🌐 Testnet Endpoints:"
echo "  JSON-RPC: http://localhost:8545"
echo "  WebSocket: ws://localhost:8546"
echo "  Dashboard: http://localhost:3000"
echo ""
