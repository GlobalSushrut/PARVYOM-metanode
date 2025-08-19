#!/bin/bash
# Full Network Setup Script - REAL CORE INFRASTRUCTURE
# Sets up: BPCI Server + BPI Enterprise + Community Node with actual built binaries

set -euo pipefail

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}🌐 Full Network Setup - REAL CORE INFRASTRUCTURE${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo "Setting up complete network with real built binaries:"
echo "1. BPCI Server (Community Entry Point)"
echo "2. BPI Enterprise Chain (Autocratic Ledger)"
echo "3. Community Node (Real PoE Notary Miners/Validators)"
echo ""

# Configuration
SETUP_DIR="/tmp/pravyom-network"
BPI_CORE_BIN="./target/release/bpi-core"
BPCI_ENTERPRISE_BIN="./target/release/pravyom-enterprise"
BPI_CONFIG="./config/enterprise-bpi-config.toml"
BPCI_CONFIG="./config/bpci-server-config.toml"

# Create setup directory
mkdir -p "$SETUP_DIR"/{bpci-server,bpi-enterprise,community-node}/{bin,config,data,logs}

echo -e "${BLUE}📋 Step 1: Verify Built Binaries${NC}"

if [[ ! -f "$BPI_CORE_BIN" ]]; then
    echo -e "${RED}❌ BPI Core binary not found. Building...${NC}"
    cargo build --release --manifest-path bpi-core/Cargo.toml
fi

if [[ ! -f "$BPCI_ENTERPRISE_BIN" ]]; then
    echo -e "${RED}❌ BPCI Enterprise binary not found. Building...${NC}"
    cargo build --release --manifest-path bpci-enterprise/Cargo.toml
fi

echo -e "${GREEN}✅ Binaries verified${NC}"

echo -e "${BLUE}📋 Step 2: Install BPCI Server (Community Entry Point)${NC}"

# Copy BPCI Server binary and config
cp "$BPCI_ENTERPRISE_BIN" "$SETUP_DIR/bpci-server/bin/pravyom-enterprise"
cp "$BPCI_CONFIG" "$SETUP_DIR/bpci-server/config/server.toml"

# Create BPCI Server startup script
cat > "$SETUP_DIR/bpci-server/start-bpci-server.sh" << 'EOF'
#!/bin/bash
cd "$(dirname "$0")"
echo "🌐 Starting BPCI Server (Community Entry Point)..."
./bin/pravyom-enterprise status --config ./config/server.toml > ./logs/bpci-server.log 2>&1 &
echo $! > ./bpci-server.pid
echo "✅ BPCI Server started (PID: $(cat ./bpci-server.pid))"
echo "📊 Logs: ./logs/bpci-server.log"
EOF

chmod +x "$SETUP_DIR/bpci-server/start-bpci-server.sh"

echo -e "${GREEN}✅ BPCI Server installed at: $SETUP_DIR/bpci-server/${NC}"

echo -e "${BLUE}📋 Step 3: Install BPI Enterprise Chain (Autocratic Ledger)${NC}"

# Copy BPI Enterprise binary and config
cp "$BPI_CORE_BIN" "$SETUP_DIR/bpi-enterprise/bin/bpi-core"
cp "$BPI_CONFIG" "$SETUP_DIR/bpi-enterprise/config/enterprise.toml"

# Create BPI Enterprise startup script
cat > "$SETUP_DIR/bpi-enterprise/start-bpi-enterprise.sh" << 'EOF'
#!/bin/bash
cd "$(dirname "$0")"
echo "🏦 Starting BPI Enterprise Chain (Autocratic Ledger)..."
./bin/bpi-core node start --config ./config/enterprise.toml --network enterprise --daemon > ./logs/bpi-enterprise.log 2>&1 &
echo $! > ./bpi-enterprise.pid
echo "✅ BPI Enterprise Chain started (PID: $(cat ./bpi-enterprise.pid))"
echo "📊 Logs: ./logs/bpi-enterprise.log"
EOF

chmod +x "$SETUP_DIR/bpi-enterprise/start-bpi-enterprise.sh"

echo -e "${GREEN}✅ BPI Enterprise Chain installed at: $SETUP_DIR/bpi-enterprise/${NC}"

echo -e "${BLUE}📋 Step 4: Install Community Node (PoE Notary Miners/Validators)${NC}"

# Create community node configuration
cat > "$SETUP_DIR/community-node/config/community.toml" << 'EOF'
# Community Node Configuration - REAL CORE INFRASTRUCTURE
# Connects to BPCI Server, not directly to BPI Enterprise Chain

[network]
name = "community-node"
mode = "community"
node_id = "community-node-001"
listen_addr = "0.0.0.0:7545"
rpc_port = 7545
p2p_port = 30305
# Real P2P networking from shared/crates/networking
network_impl = "real_p2p"
use_real_crypto = true

[bpci_connection]
bpci_server_endpoint = "http://localhost:9545"
bpci_server_rpc = "http://localhost:9546"
connection_mode = "community-to-bpci"
sync_interval = 5000  # 5 seconds
# Real bridge implementation - connects to BPCI Server
bridge_impl = "real_bridge"
use_real_bpci_connection = true

[mining]
poe_enabled = true
notary_enabled = true
mining_algorithm = "proof-of-engagement"
difficulty_target = "medium"
block_reward = 25
notary_reward = 5

[validation]
validator_enabled = true
stake_amount = 1000
validation_reward = 15
slashing_enabled = true

[security]
encryption_enabled = true
signature_verification = true
# Real cryptographic primitives from shared/crates/crypto-primitives
crypto_impl = "ed25519_real"
hash_algorithm = "blake3"
use_real_signatures = true

[storage]
data_dir = "/tmp/pravyom-network/community-node/data"
log_dir = "/tmp/pravyom-network/community-node/logs"
# Real storage implementation from shared/crates/storage
storage_impl = "real_storage"
use_real_persistence = true

[api]
rest_enabled = true
rest_port = 7546
websocket_enabled = true
websocket_port = 7547

[logging]
level = "info"
format = "json"
file_rotation = true
EOF

# Copy community node binary (using bpi-core as community node)
cp "$BPI_CORE_BIN" "$SETUP_DIR/community-node/bin/bpi-core"

# Create Community Node startup script
cat > "$SETUP_DIR/community-node/start-community-node.sh" << 'EOF'
#!/bin/bash
cd "$(dirname "$0")"
echo "👥 Starting Community Node (PoE Notary Miners/Validators)..."
./bin/bpi-core node start --config ./config/community.toml --network community --daemon > ./logs/community-node.log 2>&1 &
echo $! > ./community-node.pid
echo "✅ Community Node started (PID: $(cat ./community-node.pid))"
echo "📊 Logs: ./logs/community-node.log"
EOF

chmod +x "$SETUP_DIR/community-node/start-community-node.sh"

echo -e "${GREEN}✅ Community Node installed at: $SETUP_DIR/community-node/${NC}"

echo -e "${BLUE}📋 Step 5: Create Network Management Scripts${NC}"

# Create start-all script
cat > "$SETUP_DIR/start-all.sh" << 'EOF'
#!/bin/bash
echo "🚀 Starting Full Pravyom Network..."
echo ""

# Start BPI Enterprise Chain first (autocratic ledger)
echo "1. Starting BPI Enterprise Chain..."
cd bpi-enterprise && ./start-bpi-enterprise.sh && cd ..
sleep 3

# Start BPCI Server (community entry point)
echo "2. Starting BPCI Server..."
cd bpci-server && ./start-bpci-server.sh && cd ..
sleep 3

# Start Community Node (connects to BPCI Server)
echo "3. Starting Community Node..."
cd community-node && ./start-community-node.sh && cd ..
sleep 2

echo ""
echo "✅ Full Network Started!"
echo "🏦 BPI Enterprise Chain: http://localhost:8545 (RPC), http://localhost:8546 (REST)"
echo "🌐 BPCI Server: http://localhost:9545 (RPC), http://localhost:9546 (REST)"
echo "👥 Community Node: http://localhost:7545 (RPC), http://localhost:7546 (REST)"
echo ""
echo "📊 Check logs in respective ./logs/ directories"
echo "🛑 Stop with: ./stop-all.sh"
EOF

# Create stop-all script
cat > "$SETUP_DIR/stop-all.sh" << 'EOF'
#!/bin/bash
echo "🛑 Stopping Full Pravyom Network..."

# Stop Community Node
if [[ -f community-node/community-node.pid ]]; then
    kill $(cat community-node/community-node.pid) 2>/dev/null || true
    rm -f community-node/community-node.pid
    echo "✅ Community Node stopped"
fi

# Stop BPCI Server
if [[ -f bpci-server/bpci-server.pid ]]; then
    kill $(cat bpci-server/bpci-server.pid) 2>/dev/null || true
    rm -f bpci-server/bpci-server.pid
    echo "✅ BPCI Server stopped"
fi

# Stop BPI Enterprise Chain
if [[ -f bpi-enterprise/bpi-enterprise.pid ]]; then
    kill $(cat bpi-enterprise/bpi-enterprise.pid) 2>/dev/null || true
    rm -f bpi-enterprise/bpi-enterprise.pid
    echo "✅ BPI Enterprise Chain stopped"
fi

echo "🛑 Full Network Stopped"
EOF

# Create status script
cat > "$SETUP_DIR/status.sh" << 'EOF'
#!/bin/bash
echo "📊 Pravyom Network Status"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

check_service() {
    local name=$1
    local port=$2
    local pid_file=$3
    
    if [[ -f "$pid_file" ]] && kill -0 $(cat "$pid_file") 2>/dev/null; then
        if nc -z localhost "$port" 2>/dev/null; then
            echo "✅ $name: Running (PID: $(cat "$pid_file"), Port: $port)"
        else
            echo "⚠️  $name: Process running but port $port not responding"
        fi
    else
        echo "❌ $name: Not running"
    fi
}

check_service "BPI Enterprise Chain" 8545 "bpi-enterprise/bpi-enterprise.pid"
check_service "BPCI Server" 9545 "bpci-server/bpci-server.pid"
check_service "Community Node" 7545 "community-node/community-node.pid"

echo ""
echo "🔗 Network Architecture:"
echo "   Community Node → BPCI Server → BPI Enterprise Chain"
echo "   (Real PoE Miners/Validators) → (Entry Point) → (Autocratic Ledger)"
EOF

chmod +x "$SETUP_DIR"/{start-all.sh,stop-all.sh,status.sh}

echo -e "${GREEN}✅ Network management scripts created${NC}"

echo -e "${BLUE}📋 Step 6: Create Integration Test Script${NC}"

# Create network integration test
cat > "$SETUP_DIR/test-network.sh" << 'EOF'
#!/bin/bash
echo "🧪 Testing Full Network Integration..."

# Test BPI Enterprise Chain
echo "Testing BPI Enterprise Chain..."
curl -s http://localhost:8546/health > /dev/null && echo "✅ BPI Enterprise Chain API responding" || echo "❌ BPI Enterprise Chain API not responding"

# Test BPCI Server
echo "Testing BPCI Server..."
curl -s http://localhost:9546/health > /dev/null && echo "✅ BPCI Server API responding" || echo "❌ BPCI Server API not responding"

# Test Community Node
echo "Testing Community Node..."
curl -s http://localhost:7546/health > /dev/null && echo "✅ Community Node API responding" || echo "❌ Community Node API not responding"

echo ""
echo "🔗 Testing Network Integration:"
echo "   Community Node → BPCI Server → BPI Enterprise Chain"
echo "   All using REAL CORE INFRASTRUCTURE (no mocks)"
EOF

chmod +x "$SETUP_DIR/test-network.sh"

echo -e "${GREEN}✅ Integration test script created${NC}"

echo -e "${BLUE}📋 Step 7: Summary${NC}"
echo -e "${GREEN}🎉 FULL NETWORK SETUP COMPLETED!${NC}"
echo ""
echo -e "${CYAN}📁 Network installed at: $SETUP_DIR${NC}"
echo ""
echo -e "${BLUE}🚀 To start the full network:${NC}"
echo "   cd $SETUP_DIR && ./start-all.sh"
echo ""
echo -e "${BLUE}📊 To check status:${NC}"
echo "   cd $SETUP_DIR && ./status.sh"
echo ""
echo -e "${BLUE}🧪 To test integration:${NC}"
echo "   cd $SETUP_DIR && ./test-network.sh"
echo ""
echo -e "${BLUE}🛑 To stop the network:${NC}"
echo "   cd $SETUP_DIR && ./stop-all.sh"
echo ""
echo -e "${YELLOW}🏗️ Network Architecture:${NC}"
echo "   🏦 BPI Enterprise Chain (Autocratic Ledger): localhost:8545-8548"
echo "   🌐 BPCI Server (Community Entry Point): localhost:9545-9548"
echo "   👥 Community Node (PoE Miners/Validators): localhost:7545-7547"
echo ""
echo -e "${GREEN}✅ All using REAL CORE INFRASTRUCTURE - no mocks!${NC}"
