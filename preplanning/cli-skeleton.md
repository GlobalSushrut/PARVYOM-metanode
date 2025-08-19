# Metanode / BPI Mesh — CLI Command Skeleton
Rust-based CLI structure with TypeScript interop for the `bpi` command. Implements the 90-day onboarding plan.

---

## Core Commands Structure

```rust
// src/main.rs - Entry point
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bpi")]
#[command(about = "BPI Mesh CLI - Web3 security for Web2 apps")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(long, global = true)]
    verbose: bool,
    
    #[arg(long, global = true)]
    config: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new BPI project
    Init {
        #[arg(long)]
        template: Option<String>,
        #[arg(long)]
        name: Option<String>,
    },
    
    /// Start the local devnet and services
    Up {
        #[arg(long)]
        detach: bool,
        #[arg(long)]
        validators: Option<u8>,
    },
    
    /// Stop the local devnet
    Down {
        #[arg(long)]
        volumes: bool,
    },
    
    /// Verify chain health and finality
    Verify {
        #[arg(long)]
        last: Option<u32>,
        #[arg(long)]
        rpc: Option<String>,
    },
    
    /// Diagnose and fix common issues
    Doctor {
        #[arg(long)]
        autofix: bool,
    },
    
    /// Template management
    Templates {
        #[command(subcommand)]
        action: TemplateAction,
    },
    
    /// Agreement/policy management
    Agreement {
        #[command(subcommand)]
        action: AgreementAction,
    },
    
    /// Policy wizard and management
    Policy {
        #[command(subcommand)]
        action: PolicyAction,
    },
    
    /// Receipt management
    Receipts {
        #[command(subcommand)]
        action: ReceiptAction,
    },
    
    /// Kubernetes integration
    K8 {
        #[command(subcommand)]
        action: K8Action,
    },
    
    /// Mainnet scaling and validator management
    Mainnet {
        #[command(subcommand)]
        action: MainnetAction,
    },
    
    /// Data availability controls
    Da {
        #[command(subcommand)]
        action: DaAction,
    },
    
    /// External anchor management
    Anchors {
        #[command(subcommand)]
        action: AnchorAction,
    },
    
    /// Run services with DockLock
    Run {
        service: String,
        #[arg(long)]
        deterministic: bool,
        #[arg(long)]
        agreement: Option<String>,
    },
    
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum TemplateAction {
    /// List available templates
    List,
    /// Use a specific template
    Use { name: String },
    /// Create a custom template
    Create { name: String, path: String },
}

#[derive(Subcommand)]
enum AgreementAction {
    /// Create a new agreement
    New { name: String },
    /// Build agreement YAML to WASM
    Build { 
        input: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Pin compiled agreement
    Pin { wasm_path: String },
    /// Simulate agreement on sample data
    Simulate {
        agreement: String,
        #[arg(long)]
        input: Option<String>,
    },
    /// List pinned agreements
    List,
}

#[derive(Subcommand)]
enum PolicyAction {
    /// Interactive policy wizard
    Wizard,
    /// Show current policy
    Show { 
        #[arg(long)]
        inclusion: bool,
    },
    /// Update policy settings
    Update { key: String, value: String },
}

#[derive(Subcommand)]
enum ReceiptAction {
    /// Toggle receipts on/off
    Toggle { enabled: bool },
    /// Get receipt by ID
    Get { 
        id: String,
        #[arg(long)]
        raw: bool,
    },
    /// List recent receipts
    List {
        #[arg(long)]
        limit: Option<u32>,
    },
}

#[derive(Subcommand)]
enum K8Action {
    /// Connect to existing cluster
    Connect {
        #[arg(long)]
        name: String,
        #[arg(long)]
        testnet: Option<String>,
    },
    /// Create local k3d cluster
    Create {
        #[arg(long)]
        local: bool,
        #[arg(long)]
        name: Option<String>,
    },
    /// Show cluster status
    Status,
}

#[derive(Subcommand)]
enum MainnetAction {
    /// Scale validator count
    Scale {
        #[arg(long)]
        validators: u8,
    },
    /// Show mainnet status
    Status,
}

#[derive(Subcommand)]
enum DaAction {
    /// Enable data availability
    Enable,
    /// Disable data availability
    Disable,
    /// Show DA health
    Status,
}

#[derive(Subcommand)]
enum AnchorAction {
    /// Enable anchors to L1 chains
    Enable {
        #[arg(long)]
        chains: String, // comma-separated
    },
    /// Disable anchors
    Disable,
    /// Show anchor status
    Status,
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Get config value
    Get { key: String },
    /// Set config value
    Set { key: String, value: String },
    /// Show all config
    Show,
}
```

---

## Command Implementations (Stubs)

### Init Command
```rust
// src/commands/init.rs
use anyhow::Result;
use std::path::Path;

pub async fn init_project(template: Option<String>, name: Option<String>) -> Result<()> {
    println!("🚀 Initializing BPI project...");
    
    let project_name = name.unwrap_or_else(|| "my-bpi-app".to_string());
    let template_name = template.unwrap_or_else(|| "node-api".to_string());
    
    // Create directory structure
    create_project_structure(&project_name)?;
    
    // Generate bpicompose.yml
    generate_compose_file(&project_name, &template_name)?;
    
    // Generate basic agreement
    generate_basic_agreement()?;
    
    // Generate template-specific files
    generate_template_files(&template_name)?;
    
    println!("✅ Project '{}' initialized with template '{}'", project_name, template_name);
    println!("📝 Next steps:");
    println!("   cd {}", project_name);
    println!("   bpi up");
    println!("   bpi verify");
    
    Ok(())
}

fn create_project_structure(name: &str) -> Result<()> {
    std::fs::create_dir_all(format!("{}/agreements", name))?;
    std::fs::create_dir_all(format!("{}/services", name))?;
    std::fs::create_dir_all(format!("{}/config", name))?;
    Ok(())
}

fn generate_compose_file(name: &str, template: &str) -> Result<()> {
    let compose_content = format!(r#"
version: "1.0"
name: "{}"
template: "{}"

services:
  api:
    build: ./services/api
    ports:
      - "8080:8080"
    environment:
      - NODE_ENV=development
    agreements:
      - basic

agreements:
  basic:
    file: ./agreements/basic.yaml
    
network:
  validators: 3
  da: false
  anchors: false
  receipts: false
"#, name, template);
    
    std::fs::write(format!("{}/bpicompose.yml", name), compose_content)?;
    Ok(())
}
```

### Up Command
```rust
// src/commands/up.rs
use anyhow::Result;

pub async fn up_devnet(detach: bool, validators: Option<u8>) -> Result<()> {
    println!("🔄 Starting BPI devnet...");
    
    let validator_count = validators.unwrap_or(3);
    
    // Check prerequisites
    check_prerequisites()?;
    
    // Start validators
    start_validators(validator_count).await?;
    
    // Start relays
    start_relays().await?;
    
    // Start services
    start_services().await?;
    
    // Wait for readiness
    wait_for_readiness().await?;
    
    println!("✅ BPI devnet is running!");
    println!("📊 Dashboard: http://localhost:3000");
    println!("🔗 API endpoint: mainnet://registry.submit");
    println!("🧪 Test with: curl -s 'mainnet://registry.submit' -d '{\"ping\":\"mesh\"}'");
    
    if !detach {
        println!("Press Ctrl+C to stop...");
        tokio::signal::ctrl_c().await?;
        println!("🛑 Stopping devnet...");
    }
    
    Ok(())
}

async fn check_prerequisites() -> Result<()> {
    // Check Docker
    // Check ports
    // Check resources
    Ok(())
}
```

### Verify Command
```rust
// src/commands/verify.rs
use anyhow::Result;

pub async fn verify_chain(last: Option<u32>, rpc: Option<String>) -> Result<()> {
    println!("🔍 Verifying BPI chain health...");
    
    let rpc_url = rpc.unwrap_or_else(|| "http://localhost:8547".to_string());
    let block_count = last.unwrap_or(10);
    
    // Connect to RPC
    let client = connect_rpc(&rpc_url).await?;
    
    // Verify headers
    let headers = fetch_headers(&client, block_count).await?;
    
    for (i, header) in headers.iter().enumerate() {
        print!("Block {} ", header.height);
        
        // Check prev_hash chain
        if verify_prev_hash_chain(header, headers.get(i.saturating_sub(1))) {
            print!("✅ Chain ");
        } else {
            print!("❌ Chain ");
        }
        
        // Check BLS signature
        if verify_bls_signature(header).await? {
            print!("✅ BLS ");
        } else {
            print!("❌ BLS ");
        }
        
        // Check PoH root
        if verify_poh_root(header).await? {
            print!("✅ PoH ");
        } else {
            print!("❌ PoH ");
        }
        
        // Check DA (if enabled)
        if header.da_root != [0u8; 32] {
            if verify_da_samples(header).await? {
                print!("✅ DA ");
            } else {
                print!("❌ DA ");
            }
        }
        
        println!();
    }
    
    println!("✅ Chain verification complete");
    Ok(())
}
```

---

## Exit Codes (Consistent)
```rust
// src/exit_codes.rs
pub const SUCCESS: i32 = 0;
pub const POLICY_DENY: i32 = 10;
pub const CONSENSUS_FAIL: i32 = 20;
pub const DA_FAIL: i32 = 30;
pub const ANCHOR_FAIL: i32 = 40;
pub const CONFIG_ERROR: i32 = 50;
pub const NETWORK_ERROR: i32 = 60;
pub const DOCKER_ERROR: i32 = 70;
```

---

## Error Handling with Helpful Messages
```rust
// src/error.rs
use anyhow::{Context, Result};

pub fn handle_docker_error() -> Result<()> {
    Err(anyhow::anyhow!(
        "❌ Docker daemon not running\n\
         💡 Fix: Start Docker Desktop or run 'sudo systemctl start docker'\n\
         🔧 Auto-fix: bpi doctor --autofix"
    ))
}

pub fn handle_port_conflict(port: u16) -> Result<()> {
    Err(anyhow::anyhow!(
        "❌ Port {} already in use\n\
         💡 Fix: Stop the conflicting service or use --port {}\n\
         🔧 Auto-fix: bpi doctor --autofix",
        port, port + 1000
    ))
}
```

---

## TypeScript Interop (WASM/Node-API)
```rust
// src/wasm.rs - For browser integration
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn verify_receipt(receipt_bytes: &[u8]) -> bool {
    // Verify receipt signature and structure
    true
}

#[wasm_bindgen]
pub fn verify_bls_signature(header_hash: &[u8], signature: &[u8], pubkeys: &[u8]) -> bool {
    // BLS aggregate verification
    true
}
```

```rust
// src/napi.rs - For Node.js integration
use napi_derive::napi;

#[napi]
pub fn light_client_verify(header: String, commit: String) -> bool {
    // Full light client verification
    true
}

#[napi]
pub async fn send_bpci_frame(frame: String) -> String {
    // Send BPCI frame via QUIC
    "receipt_id".to_string()
}
```

---

## Build Configuration
```toml
# Cargo.toml
[package]
name = "bpi"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "bpi"
path = "src/main.rs"

[dependencies]
clap = { version = "4.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"
quinn = "0.10"  # QUIC
ring = "0.16"   # Crypto
blake3 = "1.0"  # Hashing

# WASM support
wasm-bindgen = { version = "0.2", optional = true }
wasm-bindgen-futures = { version = "0.4", optional = true }

# Node-API support  
napi = { version = "2.0", optional = true }
napi-derive = { version = "2.0", optional = true }

[features]
default = []
wasm = ["wasm-bindgen", "wasm-bindgen-futures"]
napi = ["dep:napi", "napi-derive"]
```

---

## Usage Examples

### Basic Flow
```bash
# Install
curl -sSL https://get.bpi.run | bash

# Initialize project
bpi init --template node-api --name my-app
cd my-app

# Start devnet
bpi up

# Verify chain
bpi verify

# Test API
curl -s "mainnet://registry.submit" -d '{"ping":"mesh"}'

# Get receipt
bpi receipts get <id>
```

### Agreement Flow
```bash
# Create policy
bpi policy wizard

# Simulate
bpi agreement simulate basic --input '{"amount": 1000}'

# Pin agreement
bpi agreement pin agreements/basic.wasm

# Enable receipts
bpi receipts toggle true
```

### Production Flow
```bash
# Connect to testnet
bpi k8 connect --name prod --testnet https://testnet.bpi.dev

# Scale up
bpi mainnet scale --validators 9

# Enable DA and anchors
bpi da enable
bpi anchors enable --chains sepolia,polygon

# Verify production setup
bpi verify --last 100
```
