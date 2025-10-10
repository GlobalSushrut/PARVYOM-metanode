# ERA-FS BPI OS Implementation Blueprint

## Design Goals (Locked ✅)
- **Content-addressed & immutable by default**; writable state is explicitly scoped
- **Verifiable lineage**: every file/dir has cryptographic identity + optional L2/L3 chain anchors
- **Capability security**: paths map to least-privilege capabilities (no ambient "/bin" access)
- **Reproducible**: packages built into immutable stores; rollbacks are trivial
- **FHS compatibility**: legacy /bin, /lib, /usr exist via shims/bind mounts, not source of truth

## 1) New Top-Level Layout - BPI OS

```
/era/                           # ERA-FS root namespace
├── store/                      # Content-addressed immutable store
│   ├── objects/               # Content-addressed objects by hash
│   │   ├── sha256-abc123.../  # Immutable file/directory objects
│   │   └── blake3-def456.../  # Alternative hash algorithms
│   ├── packages/              # Immutable package store
│   │   ├── nixos-23.05/       # OS generation snapshots
│   │   ├── bpi-core-v2.1/     # BPI Core immutable packages
│   │   └── kernel-6.8.1/      # Kernel modules and drivers
│   └── chains/                # Blockchain anchoring
│       ├── bpi-mainnet/       # BPI blockchain anchors
│       ├── ethereum/          # Ethereum L2 anchors
│       └── local/             # Local verification chains
│
├── capabilities/              # Capability-based security
│   ├── domains/               # Security domains
│   │   ├── system/            # System-level capabilities
│   │   ├── user/              # User-level capabilities
│   │   └── bpi/               # BPI-specific capabilities
│   ├── grants/                # Active capability grants
│   └── policies/              # Security policies
│
├── current/                   # Current system state (symlinks to /era/store)
│   ├── system -> /era/store/packages/nixos-23.05/
│   ├── kernel -> /era/store/packages/kernel-6.8.1/
│   ├── bpi -> /era/store/packages/bpi-core-v2.1/
│   └── userland -> /era/store/packages/userland-base/
│
├── generations/               # System generations for rollback
│   ├── 001-initial/           # Initial installation
│   ├── 002-bpi-upgrade/       # BPI Core upgrade
│   └── current -> 002-bpi-upgrade/
│
├── mutable/                   # Explicitly mutable state
│   ├── var/                   # Variable data (/var equivalent)
│   ├── home/                  # User home directories
│   ├── tmp/                   # Temporary files
│   └── etc-overlay/           # Configuration overlays
│
└── legacy/                    # FHS compatibility shims
    ├── bin -> /era/current/system/bin/
    ├── lib -> /era/current/system/lib/
    ├── usr -> /era/current/system/usr/
    └── etc -> /era/mutable/etc-overlay/
```

## 2) BPI FS Kernel Integration Plan

### **BPI FS Kernel Architecture**
BPI uses its own **FS (Filesystem) Kernel** - a specialized blockchain-based operating system kernel

## **Core Architecture Components**

### **🔧 BPI FS Kernel (Blockchain OS Kernel)**
The revolutionary blockchain-based operating system kernel specifically designed for BPI immutable OS operations:
- **Smart Contract Process Scheduling**: All processes are managed through smart contracts
- **Blockchain Consensus Resource Allocation**: Resources allocated through consensus mechanisms
- **Quantum Cryptography Security Enforcement**: All security operations use quantum-safe cryptography
- **VM-based Application Orchestration**: Applications run in secure virtual machines
- **ERA-FS Integration**: Deep integration with immutable filesystem operations
- **Part of Three-Tier Kernel Architecture**: Works alongside BPCI BSO Standard Kernel and CN Kernel

### **Phase 1: ERA-FS Kernel Module Integration**
```rust
// /era/store/packages/bpi-fs-kernel-modules/
├── era_filesystem.ko          # ERA-FS kernel module
├── era_capability.ko          # Capability security module
├── era_blockchain.ko          # Blockchain verification module
├── era_immutable.ko           # Immutability enforcement
├── era_scheduler.ko           # ERA-FS process scheduler
└── bpi_fs_bridge.ko           # Bridge to BPI FS kernel
```

### **Phase 2: System Call Integration**
```rust
// New system calls for ERA-FS
sys_era_resolve()              // Resolve content-addressed path
sys_era_verify()               // Verify blockchain lineage
sys_era_capability_grant()     // Grant capabilities
sys_era_immutable_write()      // Write to immutable store
sys_era_generation_switch()    // Switch system generation
```

### **Phase 3: Filesystem Driver**
```rust
// ERA-FS filesystem driver
struct EraFS {
    store: ContentAddressedStore,
    capabilities: CapabilityManager,
    blockchain: BlockchainVerifier,
    generations: GenerationManager,
}

impl FileSystem for EraFS {
    fn mount() -> Result<Self>;
    fn resolve_path(path: &Path) -> Result<ContentAddress>;
    fn verify_lineage(addr: &ContentAddress) -> Result<Proof>;
    fn check_capability(domain: &Domain, path: &Path) -> Result<bool>;
}
```

## 3) Content-Addressed Store Implementation

### **Object Storage**
```rust
// Content addressing with multiple hash algorithms
pub enum ContentAddress {
    Sha256([u8; 32]),
    Blake3([u8; 32]),
    Sha3_256([u8; 32]),
}

pub struct ImmutableObject {
    address: ContentAddress,
    content: Vec<u8>,
    metadata: ObjectMetadata,
    blockchain_anchor: Option<BlockchainProof>,
}
```

### **Package Management**
```rust
// Immutable package system
pub struct Package {
    name: String,
    version: String,
    content_root: ContentAddress,
    dependencies: Vec<ContentAddress>,
    signature: CryptographicSignature,
    blockchain_proof: Option<BlockchainProof>,
}
```

## 4) Capability Security System

### **Domain-Based Security**
```rust
pub enum SecurityDomain {
    System,        // System-level operations
    User(UserId),  // User-specific operations
    BPI,           // BPI Core operations
    Network,       // Network operations
    Hardware,      // Hardware access
}

pub struct Capability {
    domain: SecurityDomain,
    permissions: Vec<Permission>,
    resource_pattern: PathPattern,
    expiry: Option<SystemTime>,
}
```

### **Least-Privilege Enforcement**
```rust
// No ambient authority - everything requires explicit capability
pub struct CapabilityGrant {
    process_id: ProcessId,
    capability: Capability,
    granted_at: SystemTime,
    granted_by: SecurityDomain,
}
```

## 5) Blockchain Verification Integration

### **Lineage Tracking**
```rust
pub struct FileLineage {
    content_address: ContentAddress,
    parent_addresses: Vec<ContentAddress>,
    creation_time: SystemTime,
    creator_signature: CryptographicSignature,
    blockchain_anchors: Vec<BlockchainAnchor>,
}
```

### **Multi-Chain Anchoring**
```rust
pub enum BlockchainAnchor {
    BPIMainnet { block_hash: H256, tx_hash: H256 },
    EthereumL2 { block_number: u64, proof: MerkleProof },
    LocalChain { block_id: u64, consensus_proof: ConsensusProof },
}
```

## 6) Generation Management & Rollbacks

### **Atomic System Updates**
```rust
pub struct SystemGeneration {
    id: GenerationId,
    packages: HashMap<String, ContentAddress>,
    configuration: ContentAddress,
    kernel: ContentAddress,
    created_at: SystemTime,
    parent_generation: Option<GenerationId>,
}
```

### **Instant Rollbacks**
```rust
// Rollback is just changing symlinks
pub fn rollback_to_generation(gen_id: GenerationId) -> Result<()> {
    let generation = load_generation(gen_id)?;
    
    // Atomic symlink updates
    update_symlink("/era/current/system", &generation.system_package)?;
    update_symlink("/era/current/kernel", &generation.kernel_package)?;
    update_symlink("/era/generations/current", &format!("era/generations/{}", gen_id))?;
    
    // Reboot into new generation
    schedule_reboot()?;
    Ok(())
}
```

## 7) FHS Compatibility Layer

### **Legacy Bind Mounts**
```bash
# Traditional paths are bind mounts to ERA-FS
/bin -> /era/current/system/bin/
/lib -> /era/current/system/lib/
/usr -> /era/current/system/usr/
/etc -> /era/mutable/etc-overlay/
/var -> /era/mutable/var/
/home -> /era/mutable/home/
```

### **Transparent Migration**
```rust
// Legacy applications work unchanged
pub struct FHSCompatibility {
    era_fs: EraFS,
    legacy_mounts: HashMap<PathBuf, ContentAddress>,
}

impl FHSCompatibility {
    pub fn setup_legacy_mounts(&self) -> Result<()> {
        // Create bind mounts for FHS compatibility
        self.bind_mount("/era/current/system/bin", "/bin")?;
        self.bind_mount("/era/current/system/lib", "/lib")?;
        // ... etc
        Ok(())
    }
}
```

## Implementation Priority

1. **Kernel Module Development** - ERA-FS filesystem driver
2. **Content Store Implementation** - Immutable object storage
3. **Capability Security** - Domain-based access control
4. **Blockchain Integration** - Lineage verification
5. **Generation Management** - Atomic updates and rollbacks
6. **FHS Compatibility** - Legacy application support

This ERA-FS design provides the foundation for both BPI OS and BPCI layer implementation, ensuring blockchain verifiability, capability security, and production-grade immutability while maintaining compatibility with existing Linux userspace.
