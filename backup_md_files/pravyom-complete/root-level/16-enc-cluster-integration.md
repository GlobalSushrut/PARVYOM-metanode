# ⚡ ENC Cluster Integration - StepReceipt to LogBlock Aggregation

**Master ENC Cluster**: Learn to integrate with the real ENC notary system that aggregates StepReceipts from DockLock into cryptographically signed LogBlocks for the BPI blockchain pipeline.

---

## 🎯 **What You'll Learn**

- Understand the real ENC notary LogBlock aggregation system
- Connect DockLock containers to generate StepReceipts  
- Configure ENC notary for LogBlock creation
- Set up Blake3 hashing and Ed25519 signatures
- Integrate with the BPI blockchain pipeline

---

## 🏗️ **Real ENC Cluster Architecture**

Based on the actual implementation in `/bpci-enterprise/crates/enc-orchestration/enc/`:

```
┌─────────────────────────────────────────────────────────────┐
│                 ENC NOTARY SYSTEM (Real)                    │
├─────────────────────────────────────────────────────────────┤
│  StepReceipt Collection                                     │
│  ├── DockLock Container Operations                         │
│  ├── Receipt Generation (container_id, operation, etc.)    │
│  ├── Resource Usage Tracking                               │
│  └── Proof-of-Action Creation                              │
├─────────────────────────────────────────────────────────────┤
│  ENC Notary Aggregation                                    │
│  ├── Pending Receipt Queue (VecDeque<StepReceipt>)         │
│  ├── Batch Processing (max_receipts_per_block)             │
│  ├── Merkle Root Calculation (Blake3)                      │
│  └── LogBlock Creation                                     │
├─────────────────────────────────────────────────────────────┤
│  Cryptographic Layer                                       │
│  ├── Blake3 Hashing for Merkle Trees                       │
│  ├── Ed25519 Digital Signatures                            │
│  ├── Domain-Separated Hashing                              │
│  └── Signature Verification                                │
├─────────────────────────────────────────────────────────────┤
│  BPI Pipeline Integration                                   │
│  ├── LogBlock Channel (mpsc::UnboundedSender)              │
│  ├── Height Tracking (incremental)                         │
│  ├── Time Range Recording                                  │
│  └── Pipeline Submission                                   │
└─────────────────────────────────────────────────────────────┘
```

---

## 🚀 **Understanding the Real Implementation**

### **Key Components (from actual code)**

#### **1. LogBlock Structure**
```rust
pub struct LogBlock {
    /// Version (always 1 for v1.0)
    pub v: u8,
    /// Application identifier  
    pub app: String,
    /// Block height (incremental)
    pub height: u64,
    /// Merkle root of all StepReceipts in this block
    pub merkle_root: String, // blake3:...
    /// Number of StepReceipts in this block
    pub count: u32,
    /// Notary signature (Ed25519)
    pub sig_notary: String, // ed25519:...
    /// Time range of receipts in this block
    pub range: TimeRange,
}
```

#### **2. ENC Notary Configuration**
```rust
pub struct NotaryConfig {
    /// Maximum receipts per LogBlock
    pub max_receipts_per_block: u32,
    /// Maximum time window for LogBlock (seconds)
    pub max_block_window_s: u64,
    /// Application ID this notary serves
    pub app_id: String,
    /// Notary signing key (Ed25519)
    pub signing_key: SigningKey,
}
```

#### **3. DockLock Receipt Structure**
```rust
pub struct DockLockReceipt {
    pub receipt_id: String,
    pub container_id: String,
    pub operation: String,
    pub timestamp: Timestamp,
    pub proof_of_action: ProofOfAction,
    pub resource_usage: ResourceUsage,
    pub receipt_hash: Hash,
}
```

---

## 🔧 **Setting Up ENC Cluster Integration**

### **Step 1: Configure Your Application for Receipt Generation**

Based on the real DockLock implementation:

```rust
// Your application integration with DockLock
use metanode_core::bpi_math::metanode_integration::DockLockManager;
use metanode_core::bpi_math::receipts::{ComponentReceipt, ReceiptType};

// Initialize DockLock manager with receipt channel
let (receipt_tx, receipt_rx) = mpsc::unbounded_channel();
let mut docklock_manager = DockLockManager::new(receipt_tx);

// Deploy container - this generates StepReceipts automatically
let result = docklock_manager.deploy_container(
    "my-app".to_string(), 
    "nginx:latest".to_string()
).await;

// The manager automatically creates:
// 1. Proof-of-Action for the deployment
// 2. Resource usage tracking
// 3. DockLock receipt with Blake3 hash
// 4. Sends receipt to ENC notary via channel
```

### **Step 2: Configure ENC Notary**

```rust
use enc_orchestration::notary::{EncNotary, NotaryConfig};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

// Generate signing key for notary
let mut csprng = OsRng;
let signing_key = SigningKey::generate(&mut csprng);

// Configure notary
let config = NotaryConfig::new("my-app".to_string(), signing_key)
    .with_max_receipts(100)  // Max receipts per LogBlock
    .with_max_window_s(300); // Max 5 minutes per LogBlock

// Create channel for LogBlocks to BPI pipeline
let (logblock_tx, logblock_rx) = mpsc::unbounded_channel();

// Initialize ENC notary
let notary = EncNotary::new(config, logblock_tx);
```

### **Step 3: Connect Receipt Flow**

```rust
// Connect DockLock receipts to ENC notary
tokio::spawn(async move {
    while let Some(component_receipt) = receipt_rx.recv().await {
        match component_receipt.receipt_data {
            ReceiptType::DockLock(docklock_receipt) => {
                // Convert to StepReceipt format
                let step_receipt = StepReceipt {
                    v: 1,
                    app: component_receipt.component_id,
                    container: docklock_receipt.container_id,
                    op: docklock_receipt.operation,
                    ts: docklock_receipt.timestamp.to_rfc3339(),
                    resource: Resource {
                        cpu_ms: docklock_receipt.resource_usage.cpu_time,
                        memory_mb_s: docklock_receipt.resource_usage.memory_peak / (1024 * 1024),
                        storage_gb_day: docklock_receipt.resource_usage.storage_bytes as f64 / (1024.0 * 1024.0 * 1024.0),
                        egress_mb: docklock_receipt.resource_usage.network_bytes as f64 / (1024.0 * 1024.0),
                    },
                    labels: HashMap::new(),
                    prev_hash: "blake3:genesis".to_string(), // Link to previous receipt
                    hash: docklock_receipt.receipt_hash.clone(),
                    sig: format!("ed25519:{}", docklock_receipt.receipt_id),
                };
                
                // Send to ENC notary for aggregation
                if let Err(e) = notary.process_receipt(step_receipt).await {
                    eprintln!("Failed to process receipt: {}", e);
                }
            }
            _ => {} // Handle other receipt types
        }
    }
});
```

---

## 📊 **Testing the Integration**

### **Deploy and Monitor**

```bash
# Deploy your application with DockLock
cargo run --bin my-app

# Monitor ENC notary activity
# The notary will automatically:
# 1. Collect StepReceipts from DockLock operations
# 2. Aggregate them into LogBlocks when batch size is reached
# 3. Create Merkle roots using Blake3
# 4. Sign LogBlocks with Ed25519
# 5. Send to BPI pipeline
```

### **Expected LogBlock Output**

```json
{
  "v": 1,
  "app": "my-app",
  "height": 1,
  "merkle_root": "blake3:7d865e959b2466918c9863afca942d0fb89d7c9ac0c99bafc3749504ded97730",
  "count": 3,
  "sig_notary": "ed25519:a1b2c3d4e5f6...",
  "range": {
    "from_ts": "2024-01-15T10:00:00Z",
    "to_ts": "2024-01-15T10:05:00Z"
  }
}
```

---

## 🔍 **Real Implementation Details**

### **LogBlock Creation Process**

From the actual `create_logblock()` function:

1. **Collect Pending Receipts**: Takes up to `max_receipts_per_block` from the queue
2. **Calculate Merkle Root**: Uses Blake3 to hash all receipts into a Merkle tree
3. **Create LogBlock**: Constructs the LogBlock with incremental height
4. **Sign LogBlock**: Uses Ed25519 to sign the LogBlock data
5. **Send to Pipeline**: Sends via `mpsc::UnboundedSender` to BPI pipeline

### **Automatic Triggering**

LogBlocks are created automatically when:
- Receipt count reaches `max_receipts_per_block` (default: 100)
- Time window reaches `max_block_window_s` (default: 300 seconds)
- Manual trigger via `force_create_logblock()`

### **Resource Usage Tracking**

Each StepReceipt tracks:
- **CPU time**: Milliseconds of CPU usage
- **Memory**: Peak memory usage in MB-seconds  
- **Storage**: GB-days of storage usage
- **Network**: Egress traffic in MB

---

## 🚀 **Next Steps**

Now that you understand ENC Cluster integration:

1. **[BPI Ledger Connection](17-bpi-ledger-connection.md)** - Connect LogBlocks to blockchain
2. **[BPCI Enterprise Integration](18-bpci-ledger-integration.md)** - Enterprise governance
3. **[Advanced Monitoring](../backup_md_files/coredocs/DOCKLOCK_ENC_CLUSTER_GUIDE.md)** - Production monitoring

---

## 🆘 **Troubleshooting**

### **Common Issues**

**No LogBlocks Generated**
```rust
// Check notary stats
let stats = notary.get_stats().await;
println!("Pending receipts: {}", stats.pending_receipts);
println!("Current height: {}", stats.current_height);
```

**Receipt Processing Errors**
```rust
// Enable debug logging
env_logger::init();
// Check logs for receipt processing errors
```

**Signature Verification Failures**
```rust
// Verify signing key is properly configured
let verifying_key = VerifyingKey::from(&config.signing_key);
// Check signature format: "ed25519:..."
```

---

**🎉 Congratulations! You've integrated with the real ENC Cluster system!**

*Continue with [BPI Ledger Connection](17-bpi-ledger-connection.md) to complete the blockchain pipeline.*
