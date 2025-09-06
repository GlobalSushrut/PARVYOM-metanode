# DockLock Determinism Cage Implementation Guide

## Introduction

The Determinism Cage is the core component of DockLock that provides reproducible, verifiable computation through syscall filtering, witness recording, and deterministic RNG seeding. This guide covers the complete implementation, configuration, and operational aspects of the Determinism Cage system.

## Core Architecture

### 1. Determinism Cage Structure

The Determinism Cage provides a secure execution environment with complete reproducibility:

```rust
pub struct DeterminismCage {
    pub config: CageConfig,
    pub syscall_filter: SyscallFilter,
    pub witness_recorder: WitnessRecorder,
    pub rng_seeder: RngSeeder,
}
```

**Key Components:**
- **CageConfig**: Configuration for execution parameters
- **SyscallFilter**: seccomp-based syscall policy enforcement
- **WitnessRecorder**: I/O and syscall result recording
- **RngSeeder**: Deterministic randomness injection

### 2. Configuration System

```rust
pub struct CageConfig {
    pub enable_syscall_filtering: bool,
    pub enable_witness_recording: bool,
    pub enable_rng_seeding: bool,
    pub allowed_syscalls: Vec<i32>,
    pub witness_output_path: PathBuf,
    pub rng_seed: Option<[u8; 32]>,
    pub max_execution_time: Duration,
    pub max_memory_usage: usize,
    pub enable_network_isolation: bool,
    pub enable_filesystem_isolation: bool,
}
```

**Configuration Parameters:**
- **Syscall Control**: Fine-grained syscall allowlist
- **Resource Limits**: Memory, CPU, and execution time constraints
- **Isolation**: Network and filesystem sandboxing
- **Witness Management**: Output paths and recording options
- **Determinism**: RNG seeding and reproducibility controls

## Syscall Filtering Implementation

### 1. Seccomp Filter Architecture

The syscall filter uses Linux seccomp-bpf to enforce deterministic execution:

```rust
pub struct SyscallFilter {
    filter_program: Vec<sock_filter>,
    allowed_syscalls: HashSet<i32>,
    blocked_syscalls: HashSet<i32>,
}
```

**Blocked Non-Deterministic Syscalls:**
```rust
const BLOCKED_SYSCALLS: &[&str] = &[
    "gettimeofday",     // Non-deterministic time
    "clock_gettime",    // System clock access
    "rdtsc",           // CPU timestamp counter
    "getrandom",       // Kernel randomness
    "random",          // Legacy random syscall
    "time",            // System time
    "times",           // Process times
    "getpid",          // Process ID (varies)
    "getppid",         // Parent process ID
    "getuid",          // User ID (context-dependent)
    "getgid",          // Group ID (context-dependent)
];
```

### 2. Filter Installation

```rust
impl SyscallFilter {
    pub fn install(&self) -> Result<()> {
        let filter = seccomp::ScmpFilterContext::new_filter(
            seccomp::ScmpAction::KillProcess
        )?;
        
        // Allow essential syscalls
        for &syscall in &self.allowed_syscalls {
            filter.add_rule(seccomp::ScmpAction::Allow, syscall)?;
        }
        
        // Block non-deterministic syscalls
        for &syscall in &self.blocked_syscalls {
            filter.add_rule(seccomp::ScmpAction::Errno(libc::EPERM), syscall)?;
        }
        
        filter.load()?;
        Ok(())
    }
}
```

**Filter Enforcement:**
- **Kill Process**: Immediate termination for forbidden syscalls
- **Return Error**: EPERM error for blocked operations
- **Allow**: Permitted syscalls proceed normally
- **Audit**: Log all syscall attempts for analysis

## Witness Recording System

### 1. Witness Recorder Architecture

The witness recorder captures all I/O operations and non-deterministic syscall results:

```rust
pub struct WitnessRecorder {
    pub output_path: PathBuf,
    pub entries: Vec<WitnessEntry>,
    pub merkle_tree: Option<MerkleTree>,
    pub compression_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WitnessEntry {
    pub timestamp: u64,
    pub operation_type: WitnessOperationType,
    pub syscall_number: Option<i32>,
    pub arguments: Vec<u8>,
    pub result: Vec<u8>,
    pub hash: [u8; 32],
}
```

### 2. Witness Entry Types

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WitnessOperationType {
    FileRead { path: String, offset: u64, size: usize },
    FileWrite { path: String, offset: u64, data_hash: [u8; 32] },
    NetworkReceive { socket_fd: i32, data_hash: [u8; 32] },
    NetworkSend { socket_fd: i32, data_hash: [u8; 32] },
    SyscallResult { syscall: i32, return_value: i64 },
    EnvironmentAccess { variable: String, value: String },
    RandomnessInjection { seed: [u8; 32], sequence: u64 },
}
```

### 3. Merkle Tree Verification

```rust
impl WitnessRecorder {
    pub fn compute_merkle_root(&mut self) -> Result<[u8; 32]> {
        let leaves: Vec<[u8; 32]> = self.entries
            .iter()
            .map(|entry| entry.hash)
            .collect();
            
        let merkle_tree = MerkleTree::new(leaves)?;
        self.merkle_tree = Some(merkle_tree.clone());
        
        Ok(merkle_tree.root())
    }
    
    pub fn generate_proof(&self, entry_index: usize) -> Result<MerkleProof> {
        let merkle_tree = self.merkle_tree.as_ref()
            .ok_or_else(|| anyhow!("Merkle tree not computed"))?;
            
        merkle_tree.generate_proof(entry_index)
    }
}
```

## RNG Seeding System

### 1. Deterministic Randomness

The RNG seeder provides reproducible randomness for deterministic execution:

```rust
pub struct RngSeeder {
    pub master_seed: [u8; 32],
    pub sequence_counter: u64,
    pub rng_state: ChaCha20Rng,
}

impl RngSeeder {
    pub fn new(seed: [u8; 32]) -> Self {
        let rng_state = ChaCha20Rng::from_seed(seed);
        Self {
            master_seed: seed,
            sequence_counter: 0,
            rng_state,
        }
    }
    
    pub fn next_bytes(&mut self, length: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; length];
        self.rng_state.fill_bytes(&mut bytes);
        self.sequence_counter += 1;
        bytes
    }
}
```

### 2. RNG Injection Mechanism

```rust
impl RngSeeder {
    pub fn inject_randomness(&mut self, syscall: i32) -> Result<Vec<u8>> {
        match syscall {
            libc::SYS_getrandom => {
                let random_bytes = self.next_bytes(32);
                self.record_injection(&random_bytes)?;
                Ok(random_bytes)
            },
            libc::SYS_random => {
                let random_value = self.rng_state.next_u32();
                Ok(random_value.to_le_bytes().to_vec())
            },
            _ => Err(anyhow!("Unsupported randomness syscall: {}", syscall))
        }
    }
    
    fn record_injection(&self, bytes: &[u8]) -> Result<()> {
        // Record randomness injection in witness log
        let entry = WitnessEntry {
            timestamp: self.sequence_counter,
            operation_type: WitnessOperationType::RandomnessInjection {
                seed: self.master_seed,
                sequence: self.sequence_counter,
            },
            syscall_number: Some(libc::SYS_getrandom),
            arguments: vec![],
            result: bytes.to_vec(),
            hash: blake3::hash(bytes).into(),
        };
        
        // Add to witness recorder
        Ok(())
    }
}
```

## Execution Environment

### 1. Cage Builder Pattern

```rust
pub struct CageBuilder {
    config: CageConfig,
}

impl CageBuilder {
    pub fn new() -> Self {
        Self {
            config: CageConfig::default(),
        }
    }
    
    pub fn with_syscall_filtering(mut self, enabled: bool) -> Self {
        self.config.enable_syscall_filtering = enabled;
        self
    }
    
    pub fn with_witness_recording(mut self, path: PathBuf) -> Self {
        self.config.enable_witness_recording = true;
        self.config.witness_output_path = path;
        self
    }
    
    pub fn with_rng_seed(mut self, seed: [u8; 32]) -> Self {
        self.config.rng_seed = Some(seed);
        self.config.enable_rng_seeding = true;
        self
    }
    
    pub fn build(self) -> Result<DeterminismCage> {
        DeterminismCage::new(self.config)
    }
}
```

### 2. Command Execution

```rust
impl DeterminismCage {
    pub fn execute_command(&mut self, command: &str, args: &[&str]) -> Result<ExecutionResult> {
        // Install syscall filter
        if self.config.enable_syscall_filtering {
            self.syscall_filter.install()?;
        }
        
        // Initialize witness recording
        if self.config.enable_witness_recording {
            self.witness_recorder.start_recording()?;
        }
        
        // Set up RNG seeding
        if self.config.enable_rng_seeding {
            if let Some(seed) = self.config.rng_seed {
                self.rng_seeder = Some(RngSeeder::new(seed));
            }
        }
        
        // Execute command in isolated environment
        let start_time = Instant::now();
        let mut child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
            
        let output = child.wait_with_output()?;
        let execution_time = start_time.elapsed();
        
        // Finalize witness recording
        let witness_hash = if self.config.enable_witness_recording {
            Some(self.witness_recorder.compute_merkle_root()?)
        } else {
            None
        };
        
        Ok(ExecutionResult {
            exit_code: output.status.code().unwrap_or(-1),
            stdout: output.stdout,
            stderr: output.stderr,
            execution_time,
            witness_hash,
            determinism_verified: true,
        })
    }
}
```

## Verification and Replay

### 1. Execution Verification

```rust
impl ExecutionResult {
    pub fn compute_hash(&self) -> Result<[u8; 32]> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.exit_code.to_le_bytes());
        hasher.update(&self.stdout);
        hasher.update(&self.stderr);
        hasher.update(&self.execution_time.as_nanos().to_le_bytes());
        
        if let Some(witness_hash) = &self.witness_hash {
            hasher.update(witness_hash);
        }
        
        Ok(hasher.finalize().into())
    }
    
    pub fn verify_hash(&self, expected_hash: &[u8; 32]) -> Result<bool> {
        let computed_hash = self.compute_hash()?;
        Ok(computed_hash == *expected_hash)
    }
}
```

### 2. Replay Mechanism

```rust
impl DeterminismCage {
    pub fn replay_execution(&mut self, witness_log: &Path) -> Result<ExecutionResult> {
        // Load witness entries
        let witness_entries = self.witness_recorder.load_from_file(witness_log)?;
        
        // Verify merkle tree integrity
        let computed_root = self.witness_recorder.compute_merkle_root()?;
        let expected_root = witness_entries.merkle_root;
        
        if computed_root != expected_root {
            return Err(anyhow!("Witness log integrity verification failed"));
        }
        
        // Replay execution with witness data
        self.replay_with_witness(witness_entries)
    }
    
    fn replay_with_witness(&mut self, entries: Vec<WitnessEntry>) -> Result<ExecutionResult> {
        // Set up replay environment
        for entry in entries {
            match entry.operation_type {
                WitnessOperationType::RandomnessInjection { seed, sequence } => {
                    self.rng_seeder.as_mut().unwrap().replay_injection(seed, sequence)?;
                },
                WitnessOperationType::FileRead { path, offset, size } => {
                    // Prepare file system state for replay
                },
                _ => {
                    // Handle other witness entry types
                }
            }
        }
        
        // Execute with replayed state
        // ... replay logic
        
        Ok(ExecutionResult::default())
    }
}
```

## DockLock YAML Configuration

### 1. Container Specification

```yaml
apiVersion: docklock.bpi.dev/v1
kind: DeterminismCage
metadata:
  name: secure-computation
  namespace: production
spec:
  image: "registry.bpi.dev/secure-app:v1.2.3"
  
  # Determinism Configuration
  determinism:
    syscall_filtering: true
    witness_recording: true
    rng_seeding: true
    rng_seed: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    
  # Security Policies
  security:
    seccomp_profile: "strict"
    allowed_syscalls:
      - "read"
      - "write" 
      - "open"
      - "close"
      - "mmap"
      - "munmap"
    blocked_syscalls:
      - "gettimeofday"
      - "getrandom"
      - "rdtsc"
      
  # Resource Limits
  resources:
    limits:
      cpu: "2000m"
      memory: "4Gi"
      execution_time: "1h"
    requests:
      cpu: "1000m"
      memory: "2Gi"
      
  # Witness Configuration
  witness:
    output_path: "/var/log/docklock/witness"
    compression: true
    merkle_verification: true
    
  # Network Isolation
  network:
    isolation: true
    allowed_endpoints:
      - "api.bpi.dev:443"
      - "registry.bpi.dev:443"
      
  # Filesystem Isolation
  filesystem:
    isolation: true
    read_only_paths:
      - "/usr"
      - "/lib"
      - "/bin"
    writable_paths:
      - "/tmp"
      - "/var/log"
```

### 2. Deployment Configuration

```yaml
apiVersion: docklock.bpi.dev/v1
kind: CageDeployment
metadata:
  name: secure-service
spec:
  replicas: 3
  
  template:
    spec:
      cage:
        image: "registry.bpi.dev/service:latest"
        determinism:
          syscall_filtering: true
          witness_recording: true
          rng_seeding: true
          
  # BPI Integration
  bpi_integration:
    ledger_audit: true
    wallet_authentication: true
    shadow_registry: true
    
  # Load Balancing
  load_balancer:
    algorithm: "consistent_hashing"
    health_check:
      path: "/health"
      interval: "10s"
      timeout: "5s"
      
  # Auto Scaling
  auto_scaling:
    enabled: true
    min_replicas: 2
    max_replicas: 10
    target_cpu_utilization: 70
    target_memory_utilization: 80
```

## Operational Commands

### 1. DockLock CLI Usage

```bash
# Create determinism cage
docklock cage create --name secure-app \
  --image registry.bpi.dev/app:v1.0.0 \
  --syscall-filtering \
  --witness-recording \
  --rng-seed $(openssl rand -hex 32)

# Execute command in cage
docklock cage exec secure-app -- /bin/myapp --config /etc/config.yaml

# Verify execution
docklock cage verify secure-app \
  --witness-log /var/log/docklock/witness/secure-app.log \
  --expected-hash abc123def456...

# Replay execution
docklock cage replay secure-app \
  --witness-log /var/log/docklock/witness/secure-app.log

# Export witness proof
docklock cage export-proof secure-app \
  --output /tmp/execution-proof.json \
  --format merkle-proof
```

### 2. Cluster Management

```bash
# Deploy cage to cluster
docklock deploy --file secure-service.yaml

# Scale deployment
docklock scale secure-service --replicas 5

# Check deployment status
docklock get cages --namespace production

# View cage logs
docklock logs secure-service-pod-1 --follow

# Inspect cage configuration
docklock describe cage secure-service-pod-1
```

## Performance Optimization

### 1. Syscall Filter Optimization

```rust
// Optimized syscall filter for performance
const HIGH_FREQUENCY_SYSCALLS: &[i32] = &[
    libc::SYS_read,
    libc::SYS_write,
    libc::SYS_mmap,
    libc::SYS_munmap,
];

impl SyscallFilter {
    pub fn create_optimized_filter() -> Result<Self> {
        let mut filter = SyscallFilter::new();
        
        // Allow high-frequency syscalls without overhead
        for &syscall in HIGH_FREQUENCY_SYSCALLS {
            filter.add_fast_path_rule(syscall)?;
        }
        
        Ok(filter)
    }
}
```

### 2. Witness Recording Optimization

```rust
// Batched witness recording for performance
impl WitnessRecorder {
    pub fn record_batch(&mut self, entries: Vec<WitnessEntry>) -> Result<()> {
        // Batch hash computation
        let hashes: Vec<[u8; 32]> = entries
            .par_iter()
            .map(|entry| self.compute_entry_hash(entry))
            .collect();
            
        // Batch merkle tree update
        self.merkle_tree.append_leaves(&hashes)?;
        
        Ok(())
    }
}
```

## Troubleshooting Guide

### 1. Common Issues

**Syscall Blocked Error:**
```
Error: syscall 'gettimeofday' blocked by seccomp filter
Solution: Remove time-dependent code or use deterministic time injection
```

**Witness Verification Failed:**
```
Error: Witness log integrity verification failed
Solution: Check for file corruption or replay with original environment
```

**RNG Seeding Error:**
```
Error: RNG seed not provided for deterministic execution
Solution: Specify rng_seed in cage configuration
```

### 2. Debug Mode

```bash
# Enable debug logging
export DOCKLOCK_LOG_LEVEL=debug
export RUST_LOG=docklock=debug

# Run with syscall tracing
docklock cage exec --debug --trace-syscalls secure-app -- /bin/myapp

# Analyze witness log
docklock cage analyze-witness /var/log/docklock/witness/secure-app.log
```

## Conclusion

The DockLock Determinism Cage provides unprecedented levels of reproducibility and security for container workloads. By combining syscall filtering, witness recording, and deterministic RNG seeding, it enables verifiable computation that can be cryptographically proven and replayed exactly.

This implementation guide provides the foundation for deploying secure, deterministic applications in the BPI ecosystem, ensuring that all computation is auditable, reproducible, and quantum-safe for the future of decentralized computing.
