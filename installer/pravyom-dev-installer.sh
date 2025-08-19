#!/bin/bash
# Metanode Dev Installer - Lightweight development setup
# Version: 1.0.0
# Purpose: Quick setup for developers and testing

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PRAVYOM_VERSION="1.0.0"
INSTALL_DIR="$HOME/.pravyom"
BIN_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.config/pravyom"

echo -e "${BLUE}🚀 Pravyom Dev Installer v${PRAVYOM_VERSION}${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo "Purpose: Lightweight development environment setup"
echo "Target: Developers, testing, local development"
echo ""

# Check system requirements
check_requirements() {
    echo -e "${YELLOW}🔍 Checking system requirements...${NC}"
    
    # Check OS
    if [[ "$OSTYPE" != "linux-gnu"* ]] && [[ "$OSTYPE" != "darwin"* ]]; then
        echo -e "${RED}❌ Unsupported OS: $OSTYPE${NC}"
        exit 1
    fi
    
    # Check Rust
    if ! command -v rustc &> /dev/null; then
        echo -e "${YELLOW}⚠️  Rust not found. Installing Rust...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    # Check Git
    if ! command -v git &> /dev/null; then
        echo -e "${RED}❌ Git is required but not installed${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ System requirements met${NC}"
}

# Create directories
setup_directories() {
    echo -e "${YELLOW}📁 Setting up directories...${NC}"
    mkdir -p "$INSTALL_DIR"
    mkdir -p "$BIN_DIR"
    mkdir -p "$CONFIG_DIR"
    mkdir -p "$INSTALL_DIR/data"
    mkdir -p "$INSTALL_DIR/logs"
    echo -e "${GREEN}✅ Directories created${NC}"
}

# Install Pravyom CLI
install_cli() {
    echo -e "${YELLOW}⚡ Installing Pravyom CLI...${NC}"
    
    # Build and install CLI
    echo -e "${YELLOW}🔨 Building Pravyom CLI...${NC}"
    # Use local pravyom repository
    PRAVYOM_ROOT="$(dirname "$(dirname "$(realpath "$0")")")" 
    cd "$PRAVYOM_ROOT"
    cargo build --release -p pravyom-enterprise
    cp target/release/pravyom-enterprise "$BIN_DIR/pravyom"
    chmod +x "$BIN_DIR/pravyom"
    
    echo -e "${GREEN}✅ Pravyom CLI installed${NC}"
}

# Setup development configuration
setup_dev_config() {
    echo -e "${YELLOW}⚙️  Setting up development configuration...${NC}"
    
    cat > "$CONFIG_DIR/config.toml" << EOF
[network]
mode = "Development"
network = "localhost"
rpc_endpoint = "http://localhost:8545"
local_blockchain = true
connect_only = false

[node]
node_type = "dev-full"
data_dir = "$INSTALL_DIR/data"
log_level = "debug"
full_node = true
light_client = false

[connection]
allowed_networks = ["localhost", "localnet", "devnet"]
forbidden_networks = []
remote_only = false

[services]
# All services enabled for dev
wallet_service = true
mining_service = true
governance_service = true
notary_service = true
registry_service = true
rpc_service = true
api_service = true

[mining]
enabled = true
difficulty = "easy"
auto_start = true

[dev]
# Development-specific features
hot_reload = true
debug_mode = true
test_accounts = true
mock_data = true
participation = true
auto_vote = false

[security]
level = "development"
encryption = "basic"
EOF
    
    echo -e "${GREEN}✅ Development configuration created${NC}"
}

# Setup local testnet
setup_local_testnet() {
    echo -e "${YELLOW}🌐 Setting up local testnet...${NC}"
    
    # Create genesis configuration
    cat > "$CONFIG_DIR/genesis.json" << EOF
{
  "chain_id": "pravyom-dev",
  "initial_validators": [
    {
      "address": "dev_validator_1",
      "stake": 1000000,
      "commission": 0
    }
  ],
  "initial_supply": 1000000000,
  "block_time": 3,
  "epoch_length": 100
}
EOF
    
    echo -e "${GREEN}✅ Local testnet configuration ready${NC}"
}

# Add to PATH
setup_path() {
    echo -e "${YELLOW}🛤️  Setting up PATH...${NC}"
    
    # Add to shell profile
    for profile in ~/.bashrc ~/.zshrc ~/.profile; do
        if [ -f "$profile" ]; then
            if ! grep -q "$BIN_DIR" "$profile"; then
                echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$profile"
            fi
        fi
    done
    
    echo -e "${GREEN}✅ PATH configured${NC}"
}

# Main installation
main() {
    echo -e "${BLUE}Starting Metanode Development Installation...${NC}"
    echo ""
    
    check_requirements
    setup_directories
    install_cli
    setup_dev_config
    setup_local_testnet
    setup_path
    
    echo ""
    echo -e "${GREEN}🎉 Pravyom Development Environment Ready!${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo -e "${BLUE}📋 Next Steps:${NC}"
    echo "1. Restart your terminal or run: source ~/.bashrc"
    echo "2. Test installation: pravyom status"
    echo "4. Start local testnet: pravyom network start --local"
    echo ""
    echo -e "${BLUE}📚 Documentation:${NC}"
    echo "• Configuration: $CONFIG_DIR/config.toml"
    echo "• Logs: $INSTALL_DIR/logs/"
    echo "• Data: $INSTALL_DIR/data/"
    echo ""
    echo -e "${YELLOW}⚠️  Development Mode Features:${NC}"
    echo "• Local testnet with dev tokens"
    echo "• Debug logging enabled"
    echo "• Auto-registration for testing"
    echo "• Simplified security (dev only)"
}

# Run main installation
main "$@"
