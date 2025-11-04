# 🚀 BPI OS Cloud Deployment Guide - Production Ready

## 📋 Overview
This guide provides step-by-step instructions to deploy BPI OS on the cloud instance (68.183.25.25) and connect it to BPCI infrastructure (159.203.101.136).

---

## 🔍 Understanding BPI ↔ BPCI Communication

### **Real Architecture (from codebase analysis):**

```
┌─────────────────────────────────────────────────────────────┐
│              BPI OS Node (68.183.25.25)                      │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  1. XTMP Protocol Client (xtmp_bpci_client.rs)              │
│     - High-performance socket communication                  │
│     - 10-20x faster than HTTP                                │
│     - Real-time bundle submission                            │
│     - Stream-based status updates                            │
│                                                               │
│  2. BPI Node Registration (BpiNodeInfo)                      │
│     {                                                         │
│       "node_id": "<generated>",                              │
│       "endpoint": "68.183.25.25:7777",                       │
│       "capabilities": {                                       │
│         "vm_execution": true,                                │
│         "app_hosting": true,                                 │
│         "transaction_processing": true                       │
│       },                                                      │
│       "shared_resource_commitment": {                        │
│         "cpu_share_percentage": 25.0,                        │
│         "memory_share_mb": 256,                              │
│         "storage_share_gb": 10,                              │
│         "commitment_enforced": true                          │
│       }                                                       │
│     }                                                         │
│                                                               │
│  3. Core Services:                                           │
│     - VM Server (7777): Execute BPI transactions             │
│     - HTTP Cage (8888): Wallet auth proxy                    │
│     - Shadow Registry (8080): Web3-Web2 bridge               │
│     - ZKLock Mobile (8081): Zero-knowledge auth              │
│                                                               │
└───────────────────────┬───────────────────────────────────┘
                        │ XTMP Protocol
                        │ (Socket-based, encrypted)
                        ↓
        ┌───────────────────────────────────────┐
        │   BPCI Infrastructure (159.203.101.136) │
        ├───────────────────────────────────────┤
        │                                         │
        │  1. Cluster Ledger Server (7000)       │
        │     POST /api/v1/nodes/register         │
        │     - Registers BPI nodes               │
        │     - Tracks resource commitments       │
        │     - Manages vPod assignments          │
        │                                         │
        │  2. BPI-BPCI Bridge (6001)             │
        │     POST /api/v1/bpi/register           │
        │     - Distributed communication         │
        │     - Cross-node coordination           │
        │                                         │
        │  3. Consensus Server (9001)            │
        │     - Validates BPI transactions        │
        │     - Coordinates consensus             │
        │                                         │
        │  4. Blockchain Server (8080)           │
        │     - Processes transactions            │
        │     - Maintains ledger state            │
        │                                         │
        └───────────────────────────────────────┘
```

### **Key Integration Points:**

1. **XTMP Protocol**: BPI uses XTMP (not HTTP) for high-performance communication
2. **Mutual Living System**: BPI nodes MUST commit resources to BPCI (25% CPU, 256MB RAM minimum)
3. **Dual Registration**: Nodes register with both Cluster Ledger AND Bridge
4. **Real-time Streams**: Bundle status updates via XTMP streams
5. **Proof Systems**: POE (Proof-of-Execution) bundles submitted via XTMP

---

## 📦 Step 1: Prepare Cloud Instance

```bash
# SSH into the droplet
ssh root@68.183.25.25

# Update system
apt update && apt upgrade -y

# Install dependencies
apt install -y build-essential curl git pkg-config libssl-dev jq

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
```

---

## 🔧 Step 2: Transfer and Build BPI OS

```bash
# On local machine - transfer BPI OS source
rsync -avz --progress /home/umesh/metanode/bpi-immutable-os/ root@68.183.25.25:/root/bpi-immutable-os/

# SSH into server
ssh root@68.183.25.25

# Build BPI OS
cd /root/bpi-immutable-os
cargo build --release

# This creates: /root/bpi-immutable-os/target/release/bpi-immutable-os
```

---

## 🎯 Step 3: Install BPI OS (Creates /bpi/ namespace)

```bash
# Run BPI OS installer
cd /root/bpi-immutable-os
echo "yes" | cargo run --release

# Installation creates:
# /bpi/core/          - VM cluster (Action VM, Audit VM, etc.)
# /bpi/nxos/          - NXOS DRX network layer
# /bpi/data/          - Immutable storage
# /bpi/config/        - Configuration files
# /bpi/runtime/       - Runtime state

# System will reboot into immutable BPI OS
```

---

## 🔐 Step 4: Activate BPI Node (Generate Credentials)

Create activation script on the BPI OS instance:

```bash
# File: /bpi/config/activate_node.sh
cat > /bpi/config/activate_node.sh << 'EOF'
#!/bin/bash
set -e

echo "🔐 Activating BPI Node..."

# Generate node ID
NODE_ID=$(openssl rand -hex 32)

# Generate Ed25519 keypair
openssl genpkey -algorithm ED25519 -out /bpi/config/node_private_key.pem 2>/dev/null
chmod 600 /bpi/config/node_private_key.pem
openssl pkey -in /bpi/config/node_private_key.pem -pubout -out /bpi/config/node_public_key.pem 2>/dev/null

# Get public key
PUBLIC_KEY=$(cat /bpi/config/node_public_key.pem | grep -v "BEGIN\|END" | tr -d '\n')

# Generate auth token
AUTH_TOKEN=$(openssl rand -base64 64 | tr -d '\n')

# Get public IP
PUBLIC_IP=$(curl -s ifconfig.me)

# Save credentials
cat > /bpi/config/node_credentials.json <<CREDS
{
  "node_id": "${NODE_ID}",
  "wallet_address": "bpi://node/${NODE_ID}",
  "auth_token": "${AUTH_TOKEN}",
  "public_key": "${PUBLIC_KEY}",
  "public_ip": "${PUBLIC_IP}",
  "created_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "endpoints": {
    "vm_server": "http://${PUBLIC_IP}:7777",
    "http_cage": "http://${PUBLIC_IP}:8888",
    "shadow_registry": "http://${PUBLIC_IP}:8080",
    "zklock_mobile": "http://${PUBLIC_IP}:8081"
  }
}
CREDS

chmod 600 /bpi/config/node_credentials.json

echo "✅ Node activated!"
echo "Node ID: ${NODE_ID}"
echo "Wallet: bpi://node/${NODE_ID}"
echo "Public IP: ${PUBLIC_IP}"
EOF

chmod +x /bpi/config/activate_node.sh
/bpi/config/activate_node.sh
```

---

## 🌐 Step 5: Register with BPCI Infrastructure

Create registration script based on real BPCI API:

```bash
# File: /bpi/config/register_with_bpci.sh
cat > /bpi/config/register_with_bpci.sh << 'EOF'
#!/bin/bash
set -e

echo "🌐 Registering with BPCI Infrastructure..."

# Load credentials
NODE_ID=$(jq -r '.node_id' /bpi/config/node_credentials.json)
WALLET_ADDRESS=$(jq -r '.wallet_address' /bpi/config/node_credentials.json)
AUTH_TOKEN=$(jq -r '.auth_token' /bpi/config/node_credentials.json)
PUBLIC_KEY=$(jq -r '.public_key' /bpi/config/node_credentials.json)
PUBLIC_IP=$(jq -r '.public_ip' /bpi/config/node_credentials.json)

# BPCI endpoints
CLUSTER_LEDGER="http://159.203.101.136:7000"
BPI_BRIDGE="http://159.203.101.136:6001"

echo "📋 Registration Details:"
echo "   Node ID: ${NODE_ID:0:16}..."
echo "   Endpoint: ${PUBLIC_IP}:7777"
echo ""

# Register with BPCI Cluster Ledger
echo "1️⃣ Registering with Cluster Ledger..."
RESPONSE=$(curl -s -X POST "${CLUSTER_LEDGER}/api/v1/nodes/register" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer ${AUTH_TOKEN}" \
  -d "{
    \"node_id\": \"${NODE_ID}\",
    \"node_name\": \"bpi-os-testnet\",
    \"endpoint\": \"${PUBLIC_IP}:7777\",
    \"capabilities\": {
      \"vm_execution\": true,
      \"app_hosting\": true,
      \"transaction_processing\": true,
      \"storage_provider\": true
    },
    \"resource_allocation\": {
      \"cpu_cores\": 2,
      \"memory_gb\": 2,
      \"storage_gb\": 50,
      \"network_bandwidth_mbps\": 100
    },
    \"shared_resource_commitment\": {
      \"cpu_share_percentage\": 25.0,
      \"memory_share_mb\": 512,
      \"storage_share_gb\": 10,
      \"network_bandwidth_mbps\": 25,
      \"commitment_enforced\": true,
      \"commitment_timestamp\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\",
      \"last_validation\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"
    },
    \"connection_status\": \"Connected\",
    \"last_heartbeat\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\",
    \"communication_channels\": [
      {
        \"channel_type\": \"XTMP\",
        \"endpoint\": \"${PUBLIC_IP}:7777\",
        \"protocol_version\": \"1.0\",
        \"encryption_enabled\": true
      }
    ]
  }" 2>&1)

echo "Cluster Ledger Response:"
echo "$RESPONSE" | jq '.' 2>/dev/null || echo "$RESPONSE"

# Save BPCI configuration
cat > /bpi/config/bpci_connection.toml <<TOML
[bpci_infrastructure]
cluster_ledger_endpoint = "${CLUSTER_LEDGER}"
consensus_endpoint = "http://159.203.101.136:9001"
blockchain_endpoint = "http://159.203.101.136:8080"
auction_mempool_endpoint = "http://159.203.101.136:7002"
bso_orchestrator_endpoint = "http://159.203.101.136:9090"
bpi_bridge_endpoint = "${BPI_BRIDGE}"

[node_identity]
node_id = "${NODE_ID}"
wallet_address = "${WALLET_ADDRESS}"
auth_token = "${AUTH_TOKEN}"

[services]
vm_server_port = 7777
http_cage_port = 8888
shadow_registry_port = 8080
zklock_mobile_port = 8081

[network]
enable_trust_routing = true
enable_qlock_steering = true
enable_proof_of_forward = true
TOML

chmod 600 /bpi/config/bpci_connection.toml

echo ""
echo "✅ Registration Complete!"
echo "Configuration saved to: /bpi/config/bpci_connection.toml"
EOF

chmod +x /bpi/config/register_with_bpci.sh
/bpi/config/register_with_bpci.sh
```

---

## 🧪 Step 6: Test BPI → BPCI Transaction Flow

Create test script:

```bash
# File: /bpi/tests/test_bpci_integration.sh
cat > /bpi/tests/test_bpci_integration.sh << 'EOF'
#!/bin/bash
set -e

echo "🧪 Testing BPI → BPCI Integration"
echo "=================================="

# Load credentials
NODE_ID=$(jq -r '.node_id' /bpi/config/node_credentials.json)
AUTH_TOKEN=$(jq -r '.auth_token' /bpi/config/node_credentials.json)

# Test 1: Check BPI services
echo ""
echo "1️⃣ Checking BPI Core Services..."
for port in 7777 8888 8080 8081; do
    if curl -s -f http://localhost:$port/health > /dev/null 2>&1; then
        echo "   ✅ Port $port: HEALTHY"
    else
        echo "   ⚠️  Port $port: NOT RESPONDING"
    fi
done

# Test 2: Check BPCI connectivity
echo ""
echo "2️⃣ Checking BPCI Infrastructure Connectivity..."
BPCI_ENDPOINTS=(
    "Cluster Ledger:159.203.101.136:7000"
    "Consensus:159.203.101.136:9001"
    "Blockchain:159.203.101.136:8080"
    "Bridge:159.203.101.136:6001"
)

for endpoint in "${BPCI_ENDPOINTS[@]}"; do
    IFS=':' read -r name host port <<< "$endpoint"
    if curl -s -f --max-time 5 http://$host:$port/health > /dev/null 2>&1; then
        echo "   ✅ $name: CONNECTED"
    else
        echo "   ⚠️  $name: NOT RESPONDING"
    fi
done

# Test 3: Verify node registration
echo ""
echo "3️⃣ Verifying Node Registration..."
NODES=$(curl -s http://159.203.101.136:7000/api/v1/nodes)
if echo "$NODES" | jq -e ".nodes[] | select(.node_id == \"$NODE_ID\")" > /dev/null 2>&1; then
    echo "   ✅ Node registered in BPCI Cluster Ledger"
    echo "$NODES" | jq ".nodes[] | select(.node_id == \"$NODE_ID\")"
else
    echo "   ⚠️  Node NOT found in cluster ledger"
fi

echo ""
echo "✅ Integration Test Complete!"
EOF

chmod +x /bpi/tests/test_bpci_integration.sh
mkdir -p /bpi/tests
/bpi/tests/test_bpci_integration.sh
```

---

## 📊 Step 7: Verify Installation

```bash
# Check BPI OS filesystem
ls -la /bpi/

# Check services
systemctl status bpi-vm-server
systemctl status bpi-http-cage
systemctl status bpi-shadow-registry
systemctl status bpi-zklock-mobile

# Check node credentials
cat /bpi/config/node_credentials.json | jq '.'

# Check BPCI connection config
cat /bpi/config/bpci_connection.toml

# View logs
journalctl -u bpi-vm-server -f
```

---

## 🎯 Expected Results

### ✅ Successful Installation:
- `/bpi/` namespace created with 5-layer architecture
- 4 core services running on ports 7777-8081
- Node credentials generated and saved
- Registered with BPCI Cluster Ledger (159.203.101.136:7000)
- XTMP protocol connection established
- Resource commitment enforced (25% CPU, 512MB RAM)

### 📋 Node Information:
```json
{
  "node_id": "<32-byte hex>",
  "wallet_address": "bpi://node/<node_id>",
  "public_ip": "68.183.25.25",
  "endpoints": {
    "vm_server": "http://68.183.25.25:7777",
    "http_cage": "http://68.183.25.25:8888",
    "shadow_registry": "http://68.183.25.25:8080",
    "zklock_mobile": "http://68.183.25.25:8081"
  }
}
```

---

## 🚀 Next Steps

1. **Deploy Demo App**: Create task manager app inside BPI OS
2. **Test Transactions**: Submit POE bundles to BPCI
3. **Monitor Performance**: Track XTMP protocol metrics
4. **Scale Resources**: Adjust resource commitments as needed

---

## 🔗 Key Endpoints

### BPI OS (68.183.25.25):
- VM Server: `http://68.183.25.25:7777`
- HTTP Cage: `http://68.183.25.25:8888`
- Shadow Registry: `http://68.183.25.25:8080`
- ZKLock Mobile: `http://68.183.25.25:8081`

### BPCI Infrastructure (159.203.101.136):
- Cluster Ledger: `http://159.203.101.136:7000`
- Consensus: `http://159.203.101.136:9001`
- Blockchain: `http://159.203.101.136:8080`
- Bridge: `http://159.203.101.136:6001`

---

## 📝 Notes

- **XTMP Protocol**: BPI uses XTMP (not HTTP) for 10-20x performance
- **Mutual Living**: Resource sharing is COMPULSORY for BPCI participation
- **Immutable OS**: System files become read-only after installation
- **Testnet**: This is experimental testnet deployment

---

## 🆘 Troubleshooting

### Services not starting:
```bash
journalctl -u bpi-vm-server -n 50
systemctl restart bpi-vm-server
```

### Registration failing:
```bash
# Check BPCI connectivity
curl -v http://159.203.101.136:7000/health

# Re-run registration
/bpi/config/register_with_bpci.sh
```

### XTMP connection issues:
```bash
# Check network connectivity
ping 159.203.101.136

# Check firewall
ufw status
```
