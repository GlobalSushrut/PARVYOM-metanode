# 🚀 cargo.portal - Advanced OS-Level SDK Configuration

**Date**: 2025-10-27  
**Purpose**: Revolutionary configuration system for BPI OS distribution and SDK management  
**Concept**: Like `Cargo.toml` but for entire OS + SDK + dependencies

---

## 🎯 **THE VISION**

**cargo.portal** is a revolutionary configuration file that:
- ✅ Defines BPI OS distribution metadata
- ✅ Manages SDK dependencies and versions
- ✅ Configures OS-level components
- ✅ Handles automatic downloads and updates
- ✅ Ensures reproducible OS deployments
- ✅ Integrates with envtoml.lock for complete determinism

---

## 📋 **CARGO.PORTAL FILE STRUCTURE**

```toml
# cargo.portal - BPI OS Distribution Configuration
# Like Cargo.toml but for entire operating systems

[package]
name = "bpi-immutable-os"
version = "1.0.0"
edition = "2025"
authors = ["Pravyom Team"]
description = "BPI Immutable Operating System with integrated SDK"
license = "MIT OR Apache-2.0"
repository = "https://github.com/pravyom/bpi-os"
homepage = "https://bpi.pravyom.com"

# OS Distribution Metadata
[os]
kernel_version = "6.1.0-bpi"
architecture = ["x86_64", "aarch64", "armv7", "riscv64"]
base_image = "alpine:3.18"
filesystem = "immutable-overlay"
init_system = "systemd"
package_manager = "apk"

# BPI OS SDK Configuration
[sdk]
version = "1.0.0"
language = "rust"
min_rust_version = "1.70.0"
features = ["full", "async", "vpod", "quantum"]
default_features = true

# SDK Components
[sdk.components]
bpi_core = { version = "1.0.0", path = "../bpi-core" }
bpi_vm = { version = "1.0.0", features = ["quantum-safe"] }
vpod_runtime = { version = "1.0.0", features = ["actor-model"] }
enc_cluster = { version = "1.0.0", features = ["cbor", "domain-sep-hash"] }
docklock = { version = "1.0.0", features = ["deterministic"] }
shadow_registry = { version = "1.0.0", features = ["web3-bridge"] }

# OS-Level Dependencies
[dependencies]
# Core System Libraries
libc = "0.2"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# BPI-Specific Dependencies
bpi-consensus = { version = "1.0.0", registry = "bpi-registry" }
bpi-blockchain = { version = "1.0.0", registry = "bpi-registry" }
bpi-wallet = { version = "1.0.0", registry = "bpi-registry" }

# Cryptography
ed25519-dalek = "2.0"
blake3 = "1.5"
sha2 = "0.10"

# Networking
hyper = { version = "0.14", features = ["full"] }
axum = "0.6"
tokio-tungstenite = "0.20"

# vPod Infrastructure
vpod-coordinator = { version = "1.0.0", registry = "bpi-registry" }
arena-allocator = { version = "1.0.0", registry = "bpi-registry" }
spsc-ring-buffer = { version = "1.0.0", registry = "bpi-registry" }

# Development Dependencies
[dev-dependencies]
criterion = "0.5"
proptest = "1.0"
tokio-test = "0.4"

# Build Dependencies
[build-dependencies]
cc = "1.0"
bindgen = "0.69"

# Portal Registry Configuration
[registries]
bpi-registry = { index = "https://registry.bpi.pravyom.com" }
crates-io = { index = "https://github.com/rust-lang/crates.io-index" }

# Download Configuration
[download]
# Primary download sources
sources = [
    "https://downloads.bpi.pravyom.com/os/",
    "https://mirror1.bpi.pravyom.com/os/",
    "https://mirror2.bpi.pravyom.com/os/"
]

# Download verification
verify_checksums = true
verify_signatures = true
signing_key = "bpi-os-signing-key.pub"

# Download strategy
strategy = "fastest"  # fastest, sequential, parallel
max_parallel_downloads = 4
timeout_seconds = 300
retry_count = 3

# OS Components to Download
[download.components]
kernel = { url = "kernel-6.1.0-bpi.tar.gz", checksum = "sha256:..." }
rootfs = { url = "rootfs-1.0.0.tar.gz", checksum = "sha256:..." }
sdk = { url = "bpi-sdk-1.0.0.tar.gz", checksum = "sha256:..." }
vpod_runtime = { url = "vpod-runtime-1.0.0.tar.gz", checksum = "sha256:..." }
tools = { url = "bpi-tools-1.0.0.tar.gz", checksum = "sha256:..." }

# Installation Configuration
[install]
# Installation paths
prefix = "/opt/bpi-os"
bin_dir = "/opt/bpi-os/bin"
lib_dir = "/opt/bpi-os/lib"
include_dir = "/opt/bpi-os/include"
config_dir = "/etc/bpi-os"
data_dir = "/var/lib/bpi-os"

# Installation options
create_symlinks = true
update_path = true
install_systemd_services = true
enable_auto_start = true

# vPod Configuration
[vpod]
# vPod Environment
arena_size_mb = 1024
max_vpods = 1000
isolation_level = "Full"

# vPod Port Range
port_range_start = 10000
port_range_end = 11000

# vPod Resources
default_memory_mb = 10
default_cpu_millicores = 100

# BSO-K8 Orchestrator Configuration
[bso_k8]
orchestrator_id = "bpi-os-orchestrator"
deployment_strategy = "RollingUpdate"
replicas = 1
health_check_enabled = true
health_check_interval_seconds = 30

# Network Configuration
[network]
network_type = "mainnet"  # mainnet, testnet, devnet
enable_p2p_mesh = true
max_peers = 50
listen_address = "0.0.0.0"
discovery_enabled = true

# Port Allocation
[network.ports]
# BPCI Components
consensus_server = 9001
blockchain_server = 8080
auction_mempool = 7002
bso_k8_orchestrator = 9090
bpi_bpci_bridge = 6001
cluster_ledger = 7000
cluster_ledger_ws = 7001
xtmp_server = 8889
shadow_registry = 8081

# BPI OS Core
bpi_vm_server = 7777
http_cage = 8888
shadow_registry_bpi = 8082
zklock_mobile = 8083
enc_cluster = 8084
docklock = 8085
oracle_nodes = 8086

# vPod Infrastructure
vpod_coordinator = 9100
vpod_scheduler = 9101
arena_manager = 9102
spsc_ring_buffer = 9103
epoch_scheduler = 9104

# Security Configuration
[security]
# Cryptographic Settings
signing_algorithm = "Ed25519"
hashing_algorithm = "Blake3"
encryption_algorithm = "ChaCha20-Poly1305"

# Quantum-Safe Settings
enable_quantum_safe = true
post_quantum_algorithm = "Dilithium3"

# Audit Settings
enable_audit_trail = true
audit_log_path = "/var/log/bpi-os/audit.log"
audit_retention_days = 365

# Features Configuration
[features]
default = ["full"]

full = [
    "vpod",
    "quantum-safe",
    "audit",
    "monitoring",
    "auto-update"
]

vpod = ["vpod-coordinator", "arena-allocator", "spsc-ring-buffer"]
quantum-safe = ["post-quantum-crypto", "quantum-channels"]
audit = ["immutable-audit", "forensic-firewall"]
monitoring = ["metrics-collector", "health-monitor"]
auto-update = ["update-checker", "auto-restart"]

# Minimal installation
minimal = []

# Development installation
dev = ["full", "debug-tools", "test-utils"]

# Profile Configuration
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true

[profile.dev]
opt-level = 0
debug = true
strip = false

# Target-Specific Configuration
[target.x86_64-unknown-linux-gnu]
linker = "gcc"
rustflags = ["-C", "target-cpu=native"]

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

# Workspace Configuration
[workspace]
members = [
    "bpi-core",
    "bpi-vm",
    "vpod-runtime",
    "enc-cluster",
    "docklock",
    "shadow-registry"
]

# Metadata for Portal
[metadata]
# Documentation
docs_url = "https://docs.bpi.pravyom.com"
api_docs_url = "https://api.bpi.pravyom.com"

# Support
support_email = "support@pravyom.com"
community_forum = "https://forum.bpi.pravyom.com"
issue_tracker = "https://github.com/pravyom/bpi-os/issues"

# Release Information
release_date = "2025-10-27"
release_notes_url = "https://bpi.pravyom.com/releases/1.0.0"
changelog_url = "https://github.com/pravyom/bpi-os/blob/main/CHANGELOG.md"

# System Requirements
[requirements]
min_ram_mb = 2048
min_disk_gb = 20
min_cpu_cores = 2
supported_architectures = ["x86_64", "aarch64"]
supported_os = ["linux"]

# Lock File Configuration
[lock]
# Generate cargo.portal.lock for reproducible builds
generate_lock_file = true
lock_file_path = "cargo.portal.lock"

# Lock file includes:
# - Exact versions of all dependencies
# - Checksums of all downloaded components
# - Configuration hash
# - Build timestamp
```

---

## 🔧 **CARGO.PORTAL.LOCK (Auto-Generated)**

```toml
# cargo.portal.lock - Auto-generated, DO NOT EDIT
# This file ensures reproducible BPI OS deployments

version = "1.0.0"
generated_at = "2025-10-27T03:16:00Z"
config_hash = "sha256:a1b2c3d4e5f6..."

[package]
name = "bpi-immutable-os"
version = "1.0.0"

# Locked Dependencies
[[dependencies]]
name = "bpi-consensus"
version = "1.0.0"
source = "registry+https://registry.bpi.pravyom.com"
checksum = "sha256:abc123..."

[[dependencies]]
name = "bpi-blockchain"
version = "1.0.0"
source = "registry+https://registry.bpi.pravyom.com"
checksum = "sha256:def456..."

# Locked OS Components
[[os_components]]
name = "kernel"
version = "6.1.0-bpi"
url = "https://downloads.bpi.pravyom.com/os/kernel-6.1.0-bpi.tar.gz"
checksum = "sha256:kernel123..."
size_bytes = 104857600

[[os_components]]
name = "rootfs"
version = "1.0.0"
url = "https://downloads.bpi.pravyom.com/os/rootfs-1.0.0.tar.gz"
checksum = "sha256:rootfs456..."
size_bytes = 524288000

[[os_components]]
name = "sdk"
version = "1.0.0"
url = "https://downloads.bpi.pravyom.com/os/bpi-sdk-1.0.0.tar.gz"
checksum = "sha256:sdk789..."
size_bytes = 52428800

# Locked Port Allocations
[ports]
consensus_server = 9001
blockchain_server = 8080
# ... all 1032 ports locked

# Build Metadata
[metadata]
rust_version = "1.70.0"
build_timestamp = "2025-10-27T03:16:00Z"
builder = "cargo-portal/1.0.0"
```

---

## 🚀 **CARGO-PORTAL CLI TOOL**

```bash
# Install cargo-portal
cargo install cargo-portal

# Initialize new BPI OS project
cargo portal init my-bpi-os

# Download OS and SDK
cargo portal download

# Install BPI OS
cargo portal install

# Update dependencies
cargo portal update

# Build OS image
cargo portal build

# Generate lock file
cargo portal lock

# Verify installation
cargo portal verify

# Show status
cargo portal status

# Clean downloads
cargo portal clean
```

---

## 📊 **IMPLEMENTATION PLAN**

### **Phase 1: cargo-portal CLI Tool (Rust)**

```rust
// src/main.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cargo-portal")]
#[command(about = "BPI OS Distribution Manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize new BPI OS project
    Init { name: String },
    
    /// Download OS components and SDK
    Download {
        #[arg(long)]
        verify: bool,
    },
    
    /// Install BPI OS
    Install {
        #[arg(long)]
        prefix: Option<String>,
    },
    
    /// Update dependencies
    Update,
    
    /// Build OS image
    Build {
        #[arg(long)]
        release: bool,
    },
    
    /// Generate lock file
    Lock,
    
    /// Verify installation
    Verify,
    
    /// Show status
    Status,
    
    /// Clean downloads
    Clean,
}
```

### **Phase 2: Portal Configuration Parser**

```rust
// src/config/portal_config.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PortalConfig {
    pub package: PackageMetadata,
    pub os: OsConfig,
    pub sdk: SdkConfig,
    pub dependencies: HashMap<String, Dependency>,
    pub download: DownloadConfig,
    pub install: InstallConfig,
    pub vpod: VPodConfig,
    pub bso_k8: BsoK8Config,
    pub network: NetworkConfig,
    pub security: SecurityConfig,
}

impl PortalConfig {
    pub fn from_file(path: &Path) -> Result<Self>;
    pub fn generate_lock_file(&self) -> Result<PortalLock>;
    pub fn verify_checksums(&self) -> Result<bool>;
}
```

---

## 🎯 **KEY FEATURES**

1. **OS Distribution Management**: Complete OS + SDK in one config
2. **Dependency Resolution**: Like Cargo but for OS components
3. **Reproducible Deployments**: Lock file ensures determinism
4. **Multi-Source Downloads**: Mirrors and fallbacks
5. **Checksum Verification**: Cryptographic verification
6. **Port Allocation**: All 1032 ports managed
7. **vPod Integration**: Complete vPod configuration
8. **BSO-K8 Support**: Orchestrator integration
9. **SDK Bundling**: OS comes with integrated SDK
10. **One-Command Install**: `cargo portal install`

---

**End of Specification**
