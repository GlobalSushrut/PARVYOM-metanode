# Example 0: MetaAnalytics SaaS - Staged Deployment Guide

## Overview

This guide provides **6 conscious deployment stages** for the MetaAnalytics SaaS platform using the complete Metanode infrastructure stack.

## Architecture Flow
```
Stage 1: DockLock Container → Stage 2: Core Services → Stage 3: ENC Cluster
     ↓                           ↓                        ↓
Stage 4: BPI Consensus → Stage 5: BPCI Network → Stage 6: SaaS Integration
```

---

## Stage 1: DockLock Container Foundation

### Rationale
**Security-first approach**: Start with deterministic execution environment to ensure all subsequent services have witness recording and policy enforcement.

### Implementation
```bash
# Create workspace
mkdir -p metaanalytics-saas/{docklock,services,enc,bpci}
cd metaanalytics-saas

# Workspace Cargo.toml
cat > Cargo.toml << 'EOF'
[workspace]
members = ["docklock/container", "services/*", "enc/cluster", "bpci/transport"]

[workspace.dependencies]
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
bpi-docklock = { path = "../../rust/crates/bpi-docklock" }
bpi-enc = { path = "../../rust/crates/bpi-enc" }
EOF

# DockLock container
mkdir -p docklock/container/src
cd docklock/container
```

### Core Container Code
```rust
// docklock/container/src/main.rs
use bpi_docklock::{DeterminismCage, CageConfig};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct MetaAnalyticsContainer {
    cage: DeterminismCage,
    services: RwLock<HashMap<String, String>>,
}

impl MetaAnalyticsContainer {
    pub async fn new() -> anyhow::Result<Self> {
        let config = CageConfig {
            enable_syscall_filter: true,
            enable_witness_recording: true,
            rng_seed: Some([42u8; 32]),
        };
        
        Ok(Self {
            cage: DeterminismCage::new(config)?,
            services: RwLock::new(HashMap::new()),
        })
    }
    
    pub async fn start_service(&mut self, name: &str) -> anyhow::Result<String> {
        let result = self.cage.execute_command(&["echo", "service_started"])?;
        let service_id = uuid::Uuid::new_v4().to_string();
        self.services.write().await.insert(name.to_string(), service_id.clone());
        Ok(service_id)
    }
}
```

### Verification
```bash
cd docklock/container
cargo build
cargo test
```

**Why First?** DockLock provides the security foundation - deterministic execution with witness recording for all subsequent services.

---

## Stage 2: Core Services Inside DockLock

### Rationale
**Business logic isolation**: Deploy Gateway, Processing, and Storage services inside DockLock to ensure deterministic execution with cryptographic receipts.

### Gateway Service
```bash
mkdir -p services/gateway/src
cd services/gateway

# Minimal gateway with BPCI hooks
cat > src/main.rs << 'EOF'
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Serialize, Deserialize)]
struct AnalyticsJob {
    id: String,
    dataset: String,
    query: String,
}

#[tokio::main]
async fn main() {
    let submit = warp::path("submit")
        .and(warp::post())
        .and(warp::body::json())
        .map(|job: AnalyticsJob| {
            println!("Job submitted: {}", job.id);
            warp::reply::json(&job)
        });
    
    warp::serve(submit).run(([0, 0, 0, 0], 3000)).await;
}
EOF
```

### Processing Engine
```bash
mkdir -p services/processing/src
cd services/processing

# Deterministic processing with witness recording
cat > src/main.rs << 'EOF'
use bpi_docklock::DeterminismCage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut cage = DeterminismCage::new(Default::default())?;
    
    loop {
        // Process analytics jobs deterministically
        let result = cage.execute_function(|| {
            // Mock analytics processing
            Ok(b"analytics_result".to_vec())
        })?;
        
        println!("Processed: {} bytes", result.output.len());
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
EOF
```

**Why Second?** Services need DockLock's deterministic environment before connecting to consensus and transport layers.

---

## Stage 3: ENC Cluster Deployment

### Rationale
**Consensus-driven orchestration**: Deploy ENC cluster to provide blockchain-native service scheduling and consensus-driven deployment decisions.

### ENC Cluster
```bash
mkdir -p enc/cluster/src
cd enc/cluster

# Consensus-driven service orchestration
cat > src/main.rs << 'EOF'
use bpi_consensus::{ConsensusEngine, ConsensusConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ServiceDeployment {
    service_name: String,
    replicas: u32,
    docklock_policy: String,
}

pub struct EncCluster {
    consensus: ConsensusEngine,
}

impl EncCluster {
    pub async fn new() -> anyhow::Result<Self> {
        let config = ConsensusConfig {
            block_time_ms: 5000,
            max_transactions_per_block: 100,
            validator_count: 3,
        };
        
        Ok(Self {
            consensus: ConsensusEngine::new(config)?,
        })
    }
    
    pub async fn deploy_service(&self, deployment: ServiceDeployment) -> anyhow::Result<String> {
        // Create consensus transaction for deployment
        let tx_data = serde_json::to_vec(&deployment)?;
        // Submit to consensus and deploy via Kubernetes
        Ok(uuid::Uuid::new_v4().to_string())
    }
}
EOF
```

**Why Third?** ENC provides the orchestration layer that manages DockLock containers and prepares for BPCI integration.

---

## Stage 4: BPI Consensus & Validator Network

### Rationale
**Blockchain foundation**: Deploy BPI consensus engine with validator set, mempool, and gateway to provide the blockchain backbone for the SaaS platform.

### BPI Node
```bash
mkdir -p bpci/consensus/src
cd bpci/consensus

# BPI consensus node with validator set
cat > src/main.rs << 'EOF'
use bpi_consensus::{ConsensusEngine, Block, Transaction};
use bpi_validator_set::{ValidatorSet, ValidatorInfo};
use std::collections::HashMap;

pub struct BpiNode {
    consensus: ConsensusEngine,
    validator_set: ValidatorSet,
    mempool: HashMap<String, Transaction>,
}

impl BpiNode {
    pub async fn new() -> anyhow::Result<Self> {
        let consensus = ConsensusEngine::new(Default::default())?;
        let validator_set = ValidatorSet::new();
        
        Ok(Self {
            consensus,
            validator_set,
            mempool: HashMap::new(),
        })
    }
    
    pub async fn submit_transaction(&mut self, tx: Transaction) -> anyhow::Result<()> {
        self.mempool.insert(tx.id.clone(), tx);
        Ok(())
    }
    
    pub async fn produce_block(&mut self) -> anyhow::Result<Block> {
        let txs: Vec<Transaction> = self.mempool.drain().map(|(_, tx)| tx).collect();
        self.consensus.propose_block(txs).await
    }
}
EOF
```

**Why Fourth?** BPI consensus provides the blockchain layer that BPCI transport will connect to.

---

## Stage 5: BPCI Network & ENC Integration

### Rationale
**Secure transport**: Deploy BPCI transport layer and connect ENC cluster to enable secure, authenticated communication between services.

### BPCI Transport
```bash
mkdir -p bpci/transport/src
cd bpci/transport

# BPCI transport with ENC integration
cat > src/main.rs << 'EOF'
use bpi_bpci::{BpciTransport, BpciConfig, TransportMessage};
use std::collections::HashMap;

pub struct MetaAnalyticsBpci {
    transport: BpciTransport,
    service_registry: HashMap<String, String>,
}

impl MetaAnalyticsBpci {
    pub async fn new() -> anyhow::Result<Self> {
        let config = BpciConfig {
            listen_port: 8080,
            max_peers: 100,
            enable_encryption: true,
        };
        
        let mut transport = BpciTransport::new(config)?;
        transport.start()?;
        
        Ok(Self {
            transport,
            service_registry: HashMap::new(),
        })
    }
    
    pub async fn register_service(&mut self, service_id: &str, endpoint: &str) -> anyhow::Result<()> {
        self.service_registry.insert(service_id.to_string(), endpoint.to_string());
        Ok(())
    }
    
    pub async fn send_to_service(&self, service_id: &str, message: TransportMessage) -> anyhow::Result<()> {
        if let Some(endpoint) = self.service_registry.get(service_id) {
            self.transport.send_to_peer(endpoint, message)?;
        }
        Ok(())
    }
}
EOF
```

**Why Fifth?** BPCI enables secure communication between all components deployed in previous stages.

---

## Stage 6: SaaS Integration & Verification

### Rationale
**End-to-end integration**: Connect all components and verify the complete SaaS workflow with tracing, receipts, and CLI management.

### Integration Script
```bash
# Create integration script
cat > scripts/deploy_metaanalytics.sh << 'EOF'
#!/bin/bash
set -e

echo "🚀 Deploying MetaAnalytics SaaS Platform"

# Stage 1: Start DockLock container
echo "Stage 1: DockLock Container"
cd docklock/container && cargo run &
DOCKLOCK_PID=$!

# Stage 2: Deploy services
echo "Stage 2: Core Services"
cd ../../services/gateway && cargo run &
cd ../processing && cargo run &

# Stage 3: Start ENC cluster
echo "Stage 3: ENC Cluster"
cd ../../enc/cluster && cargo run &

# Stage 4: Start BPI consensus
echo "Stage 4: BPI Consensus"
cd ../../bpci/consensus && cargo run &

# Stage 5: Start BPCI transport
echo "Stage 5: BPCI Transport"
cd ../transport && cargo run &

# Stage 6: Verify integration
echo "Stage 6: Integration Test"
sleep 10
curl -X POST http://localhost:3000/submit -H "Content-Type: application/json" -d '{"dataset":"users","query":"SELECT COUNT(*) FROM users"}'

echo "✅ MetaAnalytics SaaS Platform deployed successfully"
EOF

chmod +x scripts/deploy_metaanalytics.sh
```

### BPI CLI Integration
```bash
# Use BPI CLI for management
bpi container start --name metaanalytics --policy strict-determinism
bpi analytics submit --dataset users --query "SELECT COUNT(*) FROM users"
bpi receipt verify --job-id <job-id>
bpi security export-audit --format json
```

**Why Last?** Final integration ensures all components work together with proper tracing and receipt generation.

---

## Summary

**Stage 1**: DockLock container (security foundation)
**Stage 2**: Core services (business logic in deterministic environment)
**Stage 3**: ENC cluster (consensus-driven orchestration)
**Stage 4**: BPI consensus (blockchain backbone)
**Stage 5**: BPCI transport (secure communication)
**Stage 6**: Integration (end-to-end SaaS verification)

Each stage builds consciously on the previous one, following Rust best practices and Metanode architecture principles.
