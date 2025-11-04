# 🚀 BPI OS Cloud Deployment & End-to-End Testing Plan

## 📋 Overview
Deploy BPI Immutable OS to a new cloud instance, activate it with wallet credentials, connect to existing BPCI infrastructure (159.203.101.136), and demonstrate full capabilities with a production demo app.

---

## 🎯 Phase 1: Cloud Instance Provisioning & BPI OS Installation

### 1.1 Provision New Cloud Instance
**Target**: Fresh Ubuntu 22.04 LTS VM for BPI OS

**Requirements**:
- **OS**: Ubuntu 22.04 LTS (clean install)
- **CPU**: 8+ cores (recommended: 8-16 cores)
- **RAM**: 8GB minimum (recommended: 16GB)
- **Storage**: 100GB+ SSD
- **Network**: Public IP with ports 7777-8777 open
- **Provider**: DigitalOcean/AWS/GCP/Azure (any)

**Action Items**:
```bash
# 1. Create new droplet/instance
# 2. SSH access setup
# 3. Update system
sudo apt update && sudo apt upgrade -y
sudo apt install -y build-essential curl git
```

### 1.2 Transfer BPI OS Installer to Cloud Instance
**Method 1: Build on Cloud Instance**
```bash
# On cloud instance
git clone <bpi-repo-url>
cd bpi-immutable-os
cargo build --release

# Run installer
sudo ./target/release/bpi-immutable-os
```

**Method 2: Transfer Pre-built Binary**
```bash
# On local machine
cd /home/umesh/metanode/bpi-immutable-os
cargo build --release
scp target/release/bpi-immutable-os root@<CLOUD_IP>:/tmp/

# On cloud instance
sudo /tmp/bpi-immutable-os
```

### 1.3 BPI OS Installation Process
**Automated 6-Phase Installation**:
1. ✅ System Analysis & Hardware Detection
2. ✅ Filesystem Immutability Preparation (`/bpi/` namespace)
3. ✅ Military-Grade Security Hardening
4. ✅ NXOS DRX BPI Infrastructure Deployment
5. ✅ Atomic Update System Setup
6. ✅ Final Immutability Lock + Reboot

**Expected Result**:
- `/bpi/` namespace created with 5-layer architecture
- 4 Core Services running (VM Server, HTTP Cage, Shadow Registry, ZKLock)
- Systemd services active and healthy
- System rebooted into immutable BPI Core OS

---

## 🔐 Phase 2: BPI OS Activation & BPCI Connection

### 2.1 Generate BPI Wallet Address & Token
**Location**: After BPI OS installation, on the cloud instance

**Create Activation Script**:
```bash
# File: /bpi/config/activate_bpi_node.sh
#!/bin/bash

# Generate Ed25519 keypair for BPI node identity
BPI_NODE_ID=$(openssl rand -hex 32)
BPI_PRIVATE_KEY=$(openssl genpkey -algorithm ED25519 -outform PEM)
BPI_PUBLIC_KEY=$(echo "$BPI_PRIVATE_KEY" | openssl pkey -pubout -outform PEM)

# Generate authentication token
BPI_AUTH_TOKEN=$(openssl rand -base64 64)

# Create wallet address format: bpi://node/<node_id>
BPI_WALLET_ADDRESS="bpi://node/${BPI_NODE_ID}"

# Save credentials
cat > /bpi/config/node_credentials.json <<EOF
{
  "node_id": "${BPI_NODE_ID}",
  "wallet_address": "${BPI_WALLET_ADDRESS}",
  "auth_token": "${BPI_AUTH_TOKEN}",
  "public_key": "${BPI_PUBLIC_KEY}",
  "created_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
}
EOF

chmod 600 /bpi/config/node_credentials.json

echo "✅ BPI Node Activated!"
echo "Wallet Address: ${BPI_WALLET_ADDRESS}"
echo "Node ID: ${BPI_NODE_ID}"
echo "Credentials saved to: /bpi/config/node_credentials.json"
```

### 2.2 Register with BPCI Infrastructure
**Connect to BPCI Cluster Ledger Server**

**Create Registration Script**:
```bash
# File: /bpi/config/register_with_bpci.sh
#!/bin/bash

source /bpi/config/node_credentials.json

# BPCI Infrastructure Endpoints
BPCI_CLUSTER_LEDGER="http://159.203.101.136:7000"
BPCI_CONSENSUS="http://159.203.101.136:9001"
BPCI_BRIDGE="http://159.203.101.136:6001"

# Register BPI node with BPCI Cluster Ledger
curl -X POST "${BPCI_CLUSTER_LEDGER}/api/v1/nodes/register" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer ${BPI_AUTH_TOKEN}" \
  -d "{
    \"node_id\": \"${BPI_NODE_ID}\",
    \"wallet_address\": \"${BPI_WALLET_ADDRESS}\",
    \"endpoint\": \"http://$(curl -s ifconfig.me):7777\",
    \"node_type\": \"bpi_os_testnet\",
    \"capabilities\": [\"vm_execution\", \"app_hosting\", \"transaction_processing\"],
    \"public_key\": \"${BPI_PUBLIC_KEY}\"
  }"

echo "✅ Registered with BPCI Infrastructure"
```

### 2.3 Configure BPCI Connection
**Create BPCI Configuration**:
```bash
# File: /bpi/config/bpci_connection.toml
[bpci_infrastructure]
cluster_ledger_endpoint = "http://159.203.101.136:7000"
consensus_endpoint = "http://159.203.101.136:9001"
blockchain_endpoint = "http://159.203.101.136:8080"
auction_mempool_endpoint = "http://159.203.101.136:7002"
bso_orchestrator_endpoint = "http://159.203.101.136:9090"
bpi_bridge_endpoint = "http://159.203.101.136:6001"

[node_identity]
node_id = "<from node_credentials.json>"
wallet_address = "<from node_credentials.json>"
auth_token = "<from node_credentials.json>"

[services]
vm_server_port = 7777
http_cage_port = 8888
shadow_registry_port = 8080
zklock_mobile_port = 8081

[network]
enable_trust_routing = true
enable_qlock_steering = true
enable_proof_of_forward = true
```

---

## 🎨 Phase 3: Demo Application Development

### 3.1 Demo App: "BPI Decentralized Task Manager"
**Purpose**: Showcase BPI OS capabilities with real-world use case

**Features**:
1. **Task Management**: Create, update, delete tasks
2. **BPI Transaction Integration**: Each task operation creates a BPI transaction
3. **Immutable Audit Trail**: All operations recorded in BPI ledger
4. **VM Execution**: Tasks processed through BPI Action VM
5. **Real-time Updates**: WebSocket integration with HTTP Cage
6. **Wallet Authentication**: ZKLock integration for user auth

**Tech Stack**:
- **Backend**: Rust + Axum (running inside BPI OS)
- **Frontend**: React + TypeScript
- **Storage**: BPI immutable storage layer
- **Auth**: ZKLock Mobile (port 8081)
- **Transactions**: BPI VM Server (port 7777)

### 3.2 Demo App Architecture
```
┌─────────────────────────────────────────────────────────────┐
│                    BPI OS Cloud Instance                     │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────────────────────────────────────────────┐   │
│  │         Demo App: Task Manager                       │   │
│  │  - Frontend: React (port 3000)                       │   │
│  │  - Backend: Rust/Axum (port 4000)                    │   │
│  └─────────────────────────────────────────────────────┘   │
│                          ↓                                    │
│  ┌─────────────────────────────────────────────────────┐   │
│  │         BPI Core Services                            │   │
│  │  - VM Server (7777): Execute task operations         │   │
│  │  - HTTP Cage (8888): Proxy & auth                    │   │
│  │  - Shadow Registry (8080): Web3 bridge               │   │
│  │  - ZKLock (8081): User authentication                │   │
│  └─────────────────────────────────────────────────────┘   │
│                          ↓                                    │
│  ┌─────────────────────────────────────────────────────┐   │
│  │         BPI Storage & Audit                          │   │
│  │  - /bpi/data/immutable: Task data                    │   │
│  │  - /bpi/data/audit: Operation logs                   │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                               │
└───────────────────────┬───────────────────────────────────┘
                        ↓
        ┌───────────────────────────────────────┐
        │   BPCI Infrastructure (159.203.101.136) │
        │  - Cluster Ledger (7000)                │
        │  - Consensus (9001)                     │
        │  - Blockchain (8080)                    │
        │  - Bridge (6001)                        │
        └───────────────────────────────────────┘
```

### 3.3 Demo App Implementation

**Backend API Endpoints**:
```rust
// File: /bpi/apps/task-manager/src/main.rs

use axum::{Router, Json, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: String,
    title: String,
    description: String,
    status: TaskStatus,
    created_at: String,
    bpi_tx_hash: String,  // BPI transaction hash
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

struct AppState {
    tasks: Arc<RwLock<Vec<Task>>>,
    bpi_client: BpiVmClient,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        tasks: Arc::new(RwLock::new(Vec::new())),
        bpi_client: BpiVmClient::new("http://localhost:7777"),
    };

    let app = Router::new()
        .route("/api/tasks", get(list_tasks).post(create_task))
        .route("/api/tasks/:id", get(get_task).put(update_task).delete(delete_task))
        .route("/api/health", get(health_check))
        .with_state(Arc::new(state));

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_task(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTaskRequest>,
) -> Json<Task> {
    // 1. Create task
    let task_id = uuid::Uuid::new_v4().to_string();
    
    // 2. Submit to BPI VM for execution
    let bpi_tx = state.bpi_client.execute_action(BpiAction::CreateTask {
        task_id: task_id.clone(),
        title: payload.title.clone(),
        description: payload.description.clone(),
    }).await.unwrap();
    
    // 3. Store in immutable storage
    let task = Task {
        id: task_id,
        title: payload.title,
        description: payload.description,
        status: TaskStatus::Pending,
        created_at: chrono::Utc::now().to_rfc3339(),
        bpi_tx_hash: bpi_tx.hash,
    };
    
    state.tasks.write().await.push(task.clone());
    
    // 4. Write to BPI immutable storage
    std::fs::write(
        format!("/bpi/data/immutable/tasks/{}.json", task.id),
        serde_json::to_string_pretty(&task).unwrap()
    ).unwrap();
    
    Json(task)
}
```

**Frontend React Component**:
```typescript
// File: /bpi/apps/task-manager/frontend/src/App.tsx

import React, { useState, useEffect } from 'react';
import axios from 'axios';

interface Task {
  id: string;
  title: string;
  description: string;
  status: 'Pending' | 'InProgress' | 'Completed';
  created_at: string;
  bpi_tx_hash: string;
}

function App() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [newTask, setNewTask] = useState({ title: '', description: '' });

  useEffect(() => {
    fetchTasks();
  }, []);

  const fetchTasks = async () => {
    const response = await axios.get('http://localhost:4000/api/tasks');
    setTasks(response.data);
  };

  const createTask = async () => {
    const response = await axios.post('http://localhost:4000/api/tasks', newTask);
    setTasks([...tasks, response.data]);
    setNewTask({ title: '', description: '' });
  };

  return (
    <div className="app">
      <h1>🚀 BPI Decentralized Task Manager</h1>
      <p>Running on BPI Immutable OS with blockchain-backed audit trail</p>
      
      <div className="create-task">
        <input
          placeholder="Task Title"
          value={newTask.title}
          onChange={(e) => setNewTask({ ...newTask, title: e.target.value })}
        />
        <textarea
          placeholder="Task Description"
          value={newTask.description}
          onChange={(e) => setNewTask({ ...newTask, description: e.target.value })}
        />
        <button onClick={createTask}>Create Task (BPI Transaction)</button>
      </div>

      <div className="tasks-list">
        {tasks.map(task => (
          <div key={task.id} className="task-card">
            <h3>{task.title}</h3>
            <p>{task.description}</p>
            <div className="task-meta">
              <span>Status: {task.status}</span>
              <span>BPI TX: {task.bpi_tx_hash.substring(0, 16)}...</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;
```

---

## 🧪 Phase 4: BPI Transaction Testing

### 4.1 Test BPI → BPCI Transaction Flow

**Test Script**:
```bash
# File: /bpi/tests/test_bpi_to_bpci_transaction.sh
#!/bin/bash

echo "🧪 Testing BPI → BPCI Transaction Flow"

# 1. Create test transaction in BPI OS
echo "1️⃣ Creating test transaction in BPI VM Server..."
TX_RESPONSE=$(curl -X POST http://localhost:7777/api/vm/execute \
  -H "Content-Type: application/json" \
  -d '{
    "action_type": "test_transaction",
    "payload": {
      "operation": "create_task",
      "data": "Test task from BPI OS"
    }
  }')

TX_HASH=$(echo $TX_RESPONSE | jq -r '.transaction_hash')
echo "✅ Transaction created: $TX_HASH"

# 2. Verify transaction in BPI immutable storage
echo "2️⃣ Verifying transaction in BPI immutable storage..."
if [ -f "/bpi/data/immutable/transactions/${TX_HASH}.json" ]; then
    echo "✅ Transaction stored in immutable storage"
else
    echo "❌ Transaction NOT found in immutable storage"
    exit 1
fi

# 3. Submit transaction to BPCI Bridge
echo "3️⃣ Submitting transaction to BPCI Bridge..."
BRIDGE_RESPONSE=$(curl -X POST http://159.203.101.136:6001/bpi/submit_transaction \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $(cat /bpi/config/node_credentials.json | jq -r '.auth_token')" \
  -d "{
    \"transaction_hash\": \"${TX_HASH}\",
    \"source_node\": \"$(cat /bpi/config/node_credentials.json | jq -r '.node_id')\",
    \"transaction_data\": $(cat /bpi/data/immutable/transactions/${TX_HASH}.json)
  }")

echo "✅ Bridge response: $BRIDGE_RESPONSE"

# 4. Verify transaction in BPCI Cluster Ledger
echo "4️⃣ Verifying transaction in BPCI Cluster Ledger..."
sleep 2  # Wait for processing
LEDGER_STATUS=$(curl -s http://159.203.101.136:7000/api/v1/transactions/${TX_HASH}/status)
echo "✅ Ledger status: $LEDGER_STATUS"

# 5. Check consensus validation
echo "5️⃣ Checking consensus validation..."
CONSENSUS_STATUS=$(curl -s http://159.203.101.136:9001/consensus/transaction/${TX_HASH})
echo "✅ Consensus status: $CONSENSUS_STATUS"

echo "🎉 BPI → BPCI Transaction Flow Test Complete!"
```

### 4.2 Performance & Load Testing

**Load Test Script**:
```bash
# File: /bpi/tests/load_test.sh
#!/bin/bash

echo "⚡ BPI OS Load Testing"

# Test 1: VM Server throughput
echo "Test 1: VM Server Throughput (100 concurrent requests)"
ab -n 1000 -c 100 http://localhost:7777/api/health

# Test 2: HTTP Cage proxy performance
echo "Test 2: HTTP Cage Performance"
ab -n 1000 -c 100 http://localhost:8888/api/health

# Test 3: Transaction processing rate
echo "Test 3: Transaction Processing Rate"
for i in {1..100}; do
    curl -X POST http://localhost:7777/api/vm/execute \
      -H "Content-Type: application/json" \
      -d "{\"action_type\": \"test\", \"payload\": {\"index\": $i}}" &
done
wait

echo "✅ Load testing complete"
```

---

## 📊 Phase 5: Monitoring & Validation

### 5.1 Service Health Checks

**Health Check Script**:
```bash
# File: /bpi/scripts/health_check.sh
#!/bin/bash

echo "🏥 BPI OS Health Check"

# Check all core services
SERVICES=(
    "VM Server:7777"
    "HTTP Cage:8888"
    "Shadow Registry:8080"
    "ZKLock Mobile:8081"
)

for service in "${SERVICES[@]}"; do
    IFS=':' read -r name port <<< "$service"
    if curl -s http://localhost:$port/health > /dev/null; then
        echo "✅ $name (port $port): HEALTHY"
    else
        echo "❌ $name (port $port): UNHEALTHY"
    fi
done

# Check BPCI connectivity
echo ""
echo "🌐 BPCI Infrastructure Connectivity"
BPCI_ENDPOINTS=(
    "Cluster Ledger:159.203.101.136:7000"
    "Consensus:159.203.101.136:9001"
    "Blockchain:159.203.101.136:8080"
    "Bridge:159.203.101.136:6001"
)

for endpoint in "${BPCI_ENDPOINTS[@]}"; do
    IFS=':' read -r name host port <<< "$endpoint"
    if curl -s http://$host:$port/health > /dev/null; then
        echo "✅ $name: CONNECTED"
    else
        echo "❌ $name: DISCONNECTED"
    fi
done
```

### 5.2 Metrics Collection

**Prometheus Configuration**:
```yaml
# File: /bpi/config/prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'bpi_vm_server'
    static_configs:
      - targets: ['localhost:7777']
  
  - job_name: 'bpi_http_cage'
    static_configs:
      - targets: ['localhost:8888']
  
  - job_name: 'bpi_shadow_registry'
    static_configs:
      - targets: ['localhost:8080']
  
  - job_name: 'bpi_zklock'
    static_configs:
      - targets: ['localhost:8081']
  
  - job_name: 'demo_app'
    static_configs:
      - targets: ['localhost:4000']
```

---

## 🎯 Phase 6: Demo Execution & Documentation

### 6.1 Demo Script

**Live Demo Flow**:
```bash
#!/bin/bash
# File: /bpi/demo/run_demo.sh

echo "🎬 BPI OS Live Demo - Decentralized Task Manager"
echo "================================================"

# 1. Show BPI OS status
echo "1️⃣ BPI OS Status:"
/bpi/scripts/health_check.sh

# 2. Show BPCI connectivity
echo ""
echo "2️⃣ BPCI Infrastructure Connection:"
curl -s http://159.203.101.136:7000/api/v1/nodes | jq '.nodes[] | select(.node_type == "bpi_os_testnet")'

# 3. Start demo app
echo ""
echo "3️⃣ Starting Demo App..."
cd /bpi/apps/task-manager
cargo run --release &
APP_PID=$!
cd frontend && npm start &
FRONTEND_PID=$!

sleep 5

# 4. Create sample tasks
echo ""
echo "4️⃣ Creating sample tasks (with BPI transactions)..."
for i in {1..5}; do
    curl -X POST http://localhost:4000/api/tasks \
      -H "Content-Type: application/json" \
      -d "{
        \"title\": \"Demo Task $i\",
        \"description\": \"This task creates a BPI transaction and stores data immutably\"
      }"
    echo "✅ Task $i created"
    sleep 1
done

# 5. Show immutable storage
echo ""
echo "5️⃣ Immutable Storage Contents:"
ls -lh /bpi/data/immutable/tasks/

# 6. Show BPI transactions in BPCI
echo ""
echo "6️⃣ BPI Transactions in BPCI Cluster Ledger:"
curl -s http://159.203.101.136:7000/api/v1/transactions | jq '.transactions[] | select(.source_node == "'$(cat /bpi/config/node_credentials.json | jq -r '.node_id')'")' | head -20

echo ""
echo "🎉 Demo Complete!"
echo "Frontend: http://$(curl -s ifconfig.me):3000"
echo "Backend API: http://$(curl -s ifconfig.me):4000"
```

### 6.2 Documentation & Results

**Create Results Document**:
```markdown
# BPI OS Deployment Results

## Instance Information
- **Cloud Provider**: [Provider Name]
- **Instance IP**: [Public IP]
- **BPI Node ID**: [From node_credentials.json]
- **Wallet Address**: [From node_credentials.json]

## Installation Results
- ✅ BPI OS installed successfully
- ✅ All core services running
- ✅ Connected to BPCI infrastructure
- ✅ Demo app deployed and functional

## Service Endpoints
- VM Server: http://[IP]:7777
- HTTP Cage: http://[IP]:8888
- Shadow Registry: http://[IP]:8080
- ZKLock Mobile: http://[IP]:8081
- Demo App: http://[IP]:3000

## Transaction Test Results
- Total transactions submitted: [Count]
- Successful BPCI submissions: [Count]
- Average latency: [ms]
- Consensus validation rate: [%]

## Performance Metrics
- VM Server throughput: [req/s]
- HTTP Cage throughput: [req/s]
- Transaction processing rate: [tx/s]
- Storage utilization: [GB]

## Demo App Capabilities Demonstrated
1. ✅ Task creation with BPI transaction
2. ✅ Immutable storage of task data
3. ✅ Real-time updates via WebSocket
4. ✅ Wallet authentication via ZKLock
5. ✅ BPCI infrastructure integration
6. ✅ Audit trail in BPI ledger
```

---

## 📝 Implementation Checklist

### Pre-Deployment
- [ ] Provision cloud instance (Ubuntu 22.04, 8+ cores, 8GB+ RAM)
- [ ] Configure firewall (ports 7777-8777, 3000, 4000)
- [ ] Install build dependencies (Rust, Node.js, build-essential)

### BPI OS Installation
- [ ] Transfer BPI OS installer to cloud instance
- [ ] Run BPI OS installation (6-phase process)
- [ ] Verify system reboot into immutable OS
- [ ] Confirm all core services running

### Activation & Registration
- [ ] Generate wallet address and authentication token
- [ ] Register with BPCI Cluster Ledger (159.203.101.136:7000)
- [ ] Configure BPCI connection endpoints
- [ ] Verify connectivity to all BPCI components

### Demo App Deployment
- [ ] Build demo app backend (Rust/Axum)
- [ ] Build demo app frontend (React)
- [ ] Deploy to BPI OS (`/bpi/apps/task-manager/`)
- [ ] Start services and verify health

### Testing
- [ ] Run BPI → BPCI transaction test
- [ ] Execute load testing scripts
- [ ] Verify immutable storage writes
- [ ] Confirm BPCI ledger integration

### Validation
- [ ] Health check all services
- [ ] Monitor metrics via Prometheus
- [ ] Review audit trails
- [ ] Document results

### Demo Execution
- [ ] Run live demo script
- [ ] Create sample tasks
- [ ] Show BPI transactions in BPCI
- [ ] Demonstrate immutable storage

---

## 🚀 Quick Start Commands

```bash
# 1. SSH into cloud instance
ssh root@<CLOUD_IP>

# 2. Install BPI OS
sudo /tmp/bpi-immutable-os

# 3. Activate node
sudo /bpi/config/activate_bpi_node.sh

# 4. Register with BPCI
sudo /bpi/config/register_with_bpci.sh

# 5. Deploy demo app
cd /bpi/apps/task-manager
cargo build --release
cargo run --release &

# 6. Run demo
/bpi/demo/run_demo.sh

# 7. Access demo app
# Open browser: http://<CLOUD_IP>:3000
```

---

## 📊 Expected Outcomes

### Technical Validation
- ✅ BPI OS running in production cloud environment
- ✅ All 4 core services operational (VM, HTTP Cage, Shadow Registry, ZKLock)
- ✅ Successful registration with BPCI infrastructure
- ✅ BPI transactions flowing to BPCI Cluster Ledger
- ✅ Immutable storage functioning correctly
- ✅ Demo app showcasing real-world capabilities

### Business Validation
- ✅ Proof of concept for BPI OS deployment
- ✅ Demonstration of BPCI integration
- ✅ Showcase for potential partners/investors
- ✅ Foundation for testnet expansion

### Next Steps
1. Scale to multiple BPI OS instances
2. Implement advanced demo apps (DeFi, NFT marketplace, etc.)
3. Performance optimization and tuning
4. Security audit and hardening
5. Documentation for community deployment

---

## 🔗 References

- BPI Immutable OS: `/home/umesh/metanode/bpi-immutable-os/`
- BPCI Infrastructure: `159.203.101.136` (ports 6001, 7000, 8080, 9001)
- Demo App Source: `/bpi/apps/task-manager/`
- Configuration: `/bpi/config/`
- Logs: `/var/log/bpi-os/`
