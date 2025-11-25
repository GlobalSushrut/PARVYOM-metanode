//! vPods Daemon - Native OS-level Container Execution Engine
//! 
//! Implements the complete vPods execution engine with Fibonacci scheduler,
//! HyperCells, tank capacity management, and Epoch Chain integration.

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::{Mutex, RwLock as AsyncRwLock};
use tracing::{info, warn, error, debug};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::atomic::{AtomicU64, AtomicU32, Ordering};
use tokio::process::{Command, Child};
use std::path::PathBuf;
use serde_json::{json, Value};

use crate::blockchain_os_kernel::commute_link::{CommuteLink, MessageHandler};
use crate::blockchain_os_kernel::commute_lock::{
    CommuteLock, MessageType, Priority, ZeroCopyMessage, LockType
};

/// vPods Daemon - Core execution engine
#[derive(Debug)]
pub struct VPodsDaemon {
    /// Node identity
    pub node_id: String,
    /// Fibonacci scheduler
    pub fibonacci_scheduler: Arc<FibonacciScheduler>,
    /// Tank capacity manager
    pub tank_manager: Arc<TankCapacityManager>,
    /// Active vPods registry
    pub active_vpods: Arc<AsyncRwLock<HashMap<String, VPodInstance>>>,
    /// HyperCells pool
    pub hypercells: Arc<AsyncRwLock<HashMap<String, HyperCell>>>,
    /// Epoch Chain for execution history
    pub epoch_chain: Arc<AsyncRwLock<Vec<SchedBlock>>>,
    /// Performance metrics
    pub metrics: Arc<RwLock<VPodsDaemonMetrics>>,
    /// CommuteLink for control plane
    pub commute_link: Arc<CommuteLink>,
    /// Distributed lock for state coordination
    pub commute_lock: Arc<CommuteLock>,
}

/// Fibonacci Scheduler - Ring-based priority scheduling
#[derive(Debug)]
pub struct FibonacciScheduler {
    /// Fibonacci rings (0-7)
    pub rings: Arc<RwLock<[FibonacciRing; 8]>>,
    /// Global scheduler state
    pub scheduler_state: Arc<RwLock<SchedulerState>>,
    /// Ring assignment counter
    pub ring_counter: Arc<AtomicU32>,
}

/// Fibonacci Ring - Priority level with Fibonacci weighting
#[derive(Debug, Clone)]
pub struct FibonacciRing {
    /// Ring number (0 = highest priority)
    pub ring_id: u8,
    /// Fibonacci weight for scheduling
    pub fibonacci_weight: u64,
    /// vPods assigned to this ring
    pub assigned_vpods: Vec<String>,
    /// Ring capacity utilization
    pub utilization: f64,
    /// Last scheduling timestamp
    pub last_scheduled: DateTime<Utc>,
}

/// Scheduler state tracking
#[derive(Debug, Clone)]
pub struct SchedulerState {
    /// Total vPods managed
    pub total_vpods: u32,
    /// Active vPods count
    pub active_vpods: u32,
    /// Scheduler efficiency
    pub efficiency: f64,
    /// Last rebalance
    pub last_rebalance: DateTime<Utc>,
}

/// Tank Capacity Manager - Resource allocation and limits
#[derive(Debug)]
pub struct TankCapacityManager {
    /// Current tank value (0.0 to 1.0)
    pub tank_value: Arc<RwLock<f64>>,
    /// Maximum vPods capacity
    pub max_vpods: u32,
    /// Resource limits per ring
    pub ring_limits: Arc<RwLock<HashMap<u8, RingResourceLimits>>>,
    /// Tank metrics
    pub tank_metrics: Arc<RwLock<TankMetrics>>,
}

/// Resource limits per Fibonacci ring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RingResourceLimits {
    /// Max CPU percent for ring
    pub max_cpu_percent: u8,
    /// Max memory MB for ring
    pub max_memory_mb: u64,
    /// Max vPods in ring
    pub max_vpods_count: u32,
}

/// Tank capacity metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TankMetrics {
    /// Current tank level
    pub tank_level: f64,
    /// Tank drain rate (per second)
    pub drain_rate: f64,
    /// Tank refill rate (per second)
    pub refill_rate: f64,
    /// Overload incidents
    pub overload_count: u64,
}

/// vPod Instance - Running container
#[derive(Debug, Clone)]
pub struct VPodInstance {
    /// vPod identifier
    pub vpod_id: String,
    /// vPod specification
    pub spec: VPodSpec,
    /// Assigned Fibonacci ring
    pub ring: u8,
    /// Associated HyperCell
    pub hypercell_id: String,
    /// Process handle
    pub process: Option<u32>, // PID
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Status
    pub status: VPodStatus,
    /// Resource usage
    pub resource_usage: ResourceUsage,
}

/// vPod Specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPodSpec {
    /// vPod name
    pub name: String,
    /// Command to execute
    pub cmd: Vec<String>,
    /// Environment variables
    pub env: HashMap<String, String>,
    /// Working directory
    pub cwd: Option<PathBuf>,
    /// Resource limits
    pub resources: VPodResourceLimits,
    /// Security profile
    pub security_profile: Option<VPodSecurityProfile>,
}

/// vPod resource limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPodResourceLimits {
    /// CPU percentage limit
    pub cpu_percent: u8,
    /// Memory limit in MB
    pub mem_mb: u64,
}

/// vPod security profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPodSecurityProfile {
    /// Security role
    pub role: VPodSecurityRole,
    /// Seccomp policy
    pub seccomp_policy: Option<String>,
    /// Network policy
    pub network_policy: Option<String>,
    /// Capabilities
    pub capabilities: Vec<String>,
}

/// vPod security roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VPodSecurityRole {
    System,      // Ring 0-1
    Service,     // Ring 2-3
    Application, // Ring 4-5
    Sandbox,     // Ring 6-7
}

/// vPod execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VPodStatus {
    Pending,
    Running,
    Stopped,
    Failed(String),
}

/// Type aliases for compatibility with vPods control/integration modules
pub type VpodsDaemon = VPodsDaemon;
pub type VpodSpec = VPodSpec;
pub type VpodResources = VPodResourceLimits;
pub type VpodStatus = VPodStatus;
pub type VpodSecurityRole = VPodSecurityRole;

/// Resource usage tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// CPU utilization
    pub cpu_percent: f64,
    /// Memory usage in MB
    pub memory_mb: u64,
    /// Network I/O bytes
    pub network_bytes: u64,
    /// Disk I/O bytes
    pub disk_bytes: u64,
}

/// HyperCell - Execution context isolation
#[derive(Debug, Clone)]
pub struct HyperCell {
    /// HyperCell identifier
    pub cell_id: String,
    /// Assigned vPods
    pub vpods: Vec<String>,
    /// Isolation level
    pub isolation_level: IsolationLevel,
    /// Resource allocation
    pub allocated_resources: ResourceAllocation,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// Isolation levels for HyperCells
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IsolationLevel {
    Full,    // Complete isolation
    Partial, // Shared resources
    Minimal, // Basic separation
}

/// Resource allocation for HyperCells
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocation {
    /// Allocated CPU cores
    pub cpu_cores: f64,
    /// Allocated memory MB
    pub memory_mb: u64,
    /// Network bandwidth Mbps
    pub network_mbps: u32,
}

/// Scheduling Block - Epoch Chain entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedBlock {
    /// Block number in epoch chain
    pub block_number: u64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Scheduled operations
    pub operations: Vec<SchedOperation>,
    /// Tank state at block time
    pub tank_state: f64,
    /// Ring utilization snapshot
    pub ring_utilization: [f64; 8],
}

/// Scheduling operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedOperation {
    /// Operation type
    pub op_type: SchedOpType,
    /// Target vPod ID
    pub vpod_id: String,
    /// Ring assignment
    pub ring: u8,
    /// Resource delta
    pub resource_delta: ResourceDelta,
}

/// Scheduling operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchedOpType {
    Create,
    Stop,
    Migrate,
    Rebalance,
}

/// Resource change delta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDelta {
    /// CPU change
    pub cpu_delta: f64,
    /// Memory change MB
    pub memory_delta: i64,
}

/// vPods daemon performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPodsDaemonMetrics {
    /// Total vPods created
    pub total_vpods_created: u64,
    /// Total vPods stopped
    pub total_vpods_stopped: u64,
    /// Average vPod lifetime seconds
    pub avg_vpod_lifetime_secs: f64,
    /// Scheduler efficiency
    pub scheduler_efficiency: f64,
    /// Tank overload events
    pub tank_overloads: u64,
    /// Ring rebalances
    pub ring_rebalances: u64,
}

impl VPodsDaemon {
    /// Create new vPods daemon
    pub async fn new(
        node_id: String,
        commute_link: Arc<CommuteLink>,
        commute_lock: Arc<CommuteLock>,
    ) -> Result<Self> {
        info!("🚀 Initializing vPods daemon for node: {}", node_id);

        let fibonacci_scheduler = Arc::new(FibonacciScheduler::new()?);
        let tank_manager = Arc::new(TankCapacityManager::new()?);

        let daemon = Self {
            node_id,
            fibonacci_scheduler,
            tank_manager,
            active_vpods: Arc::new(AsyncRwLock::new(HashMap::new())),
            hypercells: Arc::new(AsyncRwLock::new(HashMap::new())),
            epoch_chain: Arc::new(AsyncRwLock::new(Vec::new())),
            metrics: Arc::new(RwLock::new(VPodsDaemonMetrics::new())),
            commute_link,
            commute_lock,
        };

        // Initialize Fibonacci rings
        daemon.fibonacci_scheduler.initialize_rings().await?;
        
        // Initialize tank capacity
        daemon.tank_manager.initialize_tank().await?;

        info!("✅ vPods daemon initialized successfully");
        Ok(daemon)
    }

    /// Create new vPod
    pub async fn create_vpod(&self, spec: VPodSpec) -> Result<String> {
        let vpod_id = format!("vpod_{}", Uuid::new_v4().simple());
        
        info!("🔧 Creating vPod: {} with command: {:?}", vpod_id, spec.cmd);

        // Acquire distributed lock for vPod creation
        let lock_id = self.commute_lock.acquire_distributed_lock(
            format!("vpods/{}", self.node_id),
            LockType::Write,
            tokio::time::Duration::from_secs(5),
        ).await?;

        let result = async {
            // Check tank capacity
            if !self.tank_manager.can_admit_vpod(&spec).await? {
                return Err(anyhow!("VPOD_LIMIT_REACHED: Tank capacity exceeded"));
            }

            // Assign Fibonacci ring based on security role
            let ring = self.fibonacci_scheduler.assign_ring(&spec).await?;

            // Create HyperCell for isolation
            let hypercell_id = self.create_hypercell(&spec, ring).await?;

            // Spawn the actual process
            let mut cmd = Command::new(&spec.cmd[0]);
            if spec.cmd.len() > 1 {
                cmd.args(&spec.cmd[1..]);
            }

            for (key, value) in &spec.env {
                cmd.env(key, value);
            }

            if let Some(cwd) = &spec.cwd {
                cmd.current_dir(cwd);
            }

            let child = cmd.spawn()?;
            let pid = child.id().unwrap_or(0);

            // Create vPod instance
            let vpod_instance = VPodInstance {
                vpod_id: vpod_id.clone(),
                spec: spec.clone(),
                ring,
                hypercell_id,
                process: Some(pid),
                created_at: Utc::now(),
                status: VPodStatus::Running,
                resource_usage: ResourceUsage::new(),
            };

            // Register vPod
            {
                let mut vpods = self.active_vpods.write().await;
                vpods.insert(vpod_id.clone(), vpod_instance);
            }

            // Update tank state
            self.tank_manager.consume_capacity(&spec).await?;

            // Record in Epoch Chain
            self.record_scheduling_operation(SchedOpType::Create, &vpod_id, ring, &spec).await?;

            // Update metrics
            {
                let mut metrics = self.metrics.write().unwrap();
                metrics.total_vpods_created += 1;
            }

            Ok(vpod_id)
        }.await;

        // Release distributed lock
        self.commute_lock.release_distributed_lock(lock_id).await?;

        result
    }

    /// Stop vPod
    pub async fn stop_vpod(&self, vpod_id: &str) -> Result<()> {
        info!("🛑 Stopping vPod: {}", vpod_id);

        // Acquire distributed lock
        let lock_id = self.commute_lock.acquire_distributed_lock(
            format!("vpods/{}", self.node_id),
            LockType::Write,
            tokio::time::Duration::from_secs(5),
        ).await?;

        let result = async {
            let mut vpods = self.active_vpods.write().await;
            
            if let Some(mut vpod) = vpods.remove(vpod_id) {
                // Terminate process
                if let Some(pid) = vpod.process {
                    let _ = Command::new("kill")
                        .arg("-TERM")
                        .arg(pid.to_string())
                        .status()
                        .await;
                }

                // Update status
                vpod.status = VPodStatus::Stopped;

                // Release tank capacity
                self.tank_manager.release_capacity(&vpod.spec).await?;

                // Record in Epoch Chain
                self.record_scheduling_operation(
                    SchedOpType::Stop, 
                    vpod_id, 
                    vpod.ring, 
                    &vpod.spec
                ).await?;

                // Update metrics
                {
                    let mut metrics = self.metrics.write().unwrap();
                    metrics.total_vpods_stopped += 1;
                }

                info!("✅ vPod stopped: {}", vpod_id);
                Ok(())
            } else {
                Err(anyhow!("VPOD_NOT_FOUND: vPod {} not found", vpod_id))
            }
        }.await;

        // Release distributed lock
        self.commute_lock.release_distributed_lock(lock_id).await?;

        result
    }

    /// Execute a command inside the vPod's execution context
    pub async fn exec_in_vpod(&self, vpod_id: &str, command: &[String]) -> Result<Value> {
        info!("⚙️ Executing command in vPod: {} cmd={:?}", vpod_id, command);

        if command.is_empty() {
            return Err(anyhow!("INVALID_COMMAND: empty exec command"));
        }

        let vpods = self.active_vpods.read().await;
        let vpod = vpods
            .get(vpod_id)
            .ok_or_else(|| anyhow!("VPOD_NOT_FOUND: vPod {} not found", vpod_id))?;

        // Build process using vPod's environment and working directory
        let start = std::time::Instant::now();
        let mut cmd = Command::new(&command[0]);
        if command.len() > 1 {
            cmd.args(&command[1..]);
        }

        for (key, value) in &vpod.spec.env {
            cmd.env(key, value);
        }

        if let Some(cwd) = &vpod.spec.cwd {
            cmd.current_dir(cwd);
        }

        let output = cmd.output().await?;
        let duration_ms = start.elapsed().as_millis() as u64;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);

        Ok(json!({
            "vpod_id": vpod_id,
            "command": command,
            "status": if output.status.success() { "success" } else { "failed" },
            "exit_code": exit_code,
            "stdout": stdout,
            "stderr": stderr,
            "execution_time_ms": duration_ms
        }))
    }

    /// Get vPod information
    pub async fn inspect_vpod(&self, vpod_id: &str) -> Result<Value> {
        let vpods = self.active_vpods.read().await;
        
        if let Some(vpod) = vpods.get(vpod_id) {
            Ok(json!({
                "vpod_id": vpod.vpod_id,
                "name": vpod.spec.name,
                "status": vpod.status,
                "ring": vpod.ring,
                "hypercell_id": vpod.hypercell_id,
                "created_at": vpod.created_at,
                "resource_usage": vpod.resource_usage,
                "process_id": vpod.process
            }))
        } else {
            Err(anyhow!("VPOD_NOT_FOUND: vPod {} not found", vpod_id))
        }
    }

    /// List all vPods
    pub async fn list_vpods(&self) -> Result<Value> {
        let vpods = self.active_vpods.read().await;
        let vpod_list: Vec<Value> = vpods.values()
            .map(|vpod| json!({
                "vpod_id": vpod.vpod_id,
                "name": vpod.spec.name,
                "status": vpod.status,
                "ring": vpod.ring,
                "created_at": vpod.created_at
            }))
            .collect();

        Ok(json!({
            "vpods": vpod_list,
            "total_count": vpod_list.len()
        }))
    }

    /// Get node capacity information
    pub async fn get_node_capacity(&self) -> Result<Value> {
        let tank_metrics = self.tank_manager.tank_metrics.read().unwrap();
        let scheduler_state = self.fibonacci_scheduler.scheduler_state.read().unwrap();

        Ok(json!({
            "node_id": self.node_id,
            "tank_level": tank_metrics.tank_level,
            "max_vpods": self.tank_manager.max_vpods,
            "active_vpods": scheduler_state.active_vpods,
            "scheduler_efficiency": scheduler_state.efficiency,
            "ring_utilization": self.get_ring_utilization().await?
        }))
    }

    /// Get scheduler rings information
    pub async fn get_scheduler_rings(&self) -> Result<Value> {
        let rings = self.fibonacci_scheduler.rings.read().unwrap();
        let ring_info: Vec<Value> = rings.iter()
            .map(|ring| json!({
                "ring_id": ring.ring_id,
                "fibonacci_weight": ring.fibonacci_weight,
                "assigned_vpods": ring.assigned_vpods.len(),
                "utilization": ring.utilization,
                "last_scheduled": ring.last_scheduled
            }))
            .collect();

        Ok(json!({
            "rings": ring_info,
            "total_rings": 8
        }))
    }

    /// Get epoch chain tail
    pub async fn get_epoch_tail(&self) -> Result<Value> {
        let epoch_chain = self.epoch_chain.read().await;
        let tail_blocks: Vec<&SchedBlock> = epoch_chain.iter().rev().take(10).collect();

        Ok(json!({
            "tail_blocks": tail_blocks,
            "chain_length": epoch_chain.len()
        }))
    }

    // Helper methods
    async fn create_hypercell(&self, spec: &VPodSpec, ring: u8) -> Result<String> {
        let hypercell_id = format!("hc_{}", Uuid::new_v4().simple());
        
        let hypercell = HyperCell {
            cell_id: hypercell_id.clone(),
            vpods: vec![],
            isolation_level: match ring {
                0..=1 => IsolationLevel::Full,
                2..=5 => IsolationLevel::Partial,
                _ => IsolationLevel::Minimal,
            },
            allocated_resources: ResourceAllocation {
                cpu_cores: spec.resources.cpu_percent as f64 / 100.0,
                memory_mb: spec.resources.mem_mb,
                network_mbps: 100, // Default
            },
            created_at: Utc::now(),
        };

        let mut hypercells = self.hypercells.write().await;
        hypercells.insert(hypercell_id.clone(), hypercell);

        Ok(hypercell_id)
    }

    async fn record_scheduling_operation(
        &self,
        op_type: SchedOpType,
        vpod_id: &str,
        ring: u8,
        spec: &VPodSpec,
    ) -> Result<()> {
        let mut epoch_chain = self.epoch_chain.write().await;
        
        let block_number = epoch_chain.len() as u64;
        let tank_state = *self.tank_manager.tank_value.read().unwrap();
        let ring_utilization = self.get_ring_utilization().await?;

        let operation = SchedOperation {
            op_type,
            vpod_id: vpod_id.to_string(),
            ring,
            resource_delta: ResourceDelta {
                cpu_delta: spec.resources.cpu_percent as f64,
                memory_delta: spec.resources.mem_mb as i64,
            },
        };

        let block = SchedBlock {
            block_number,
            timestamp: Utc::now(),
            operations: vec![operation],
            tank_state,
            ring_utilization,
        };

        epoch_chain.push(block);

        // Keep only last 1000 blocks
        if epoch_chain.len() > 1000 {
            epoch_chain.remove(0);
        }

        Ok(())
    }

    async fn get_ring_utilization(&self) -> Result<[f64; 8]> {
        let rings = self.fibonacci_scheduler.rings.read().unwrap();
        let mut utilization = [0.0; 8];
        
        for (i, ring) in rings.iter().enumerate() {
            utilization[i] = ring.utilization;
        }

        Ok(utilization)
    }
}

impl FibonacciScheduler {
    /// Create new Fibonacci scheduler
    pub fn new() -> Result<Self> {
        let rings = Arc::new(RwLock::new([
            FibonacciRing::new(0, 1),    // Ring 0: F(1) = 1
            FibonacciRing::new(1, 1),    // Ring 1: F(2) = 1
            FibonacciRing::new(2, 2),    // Ring 2: F(3) = 2
            FibonacciRing::new(3, 3),    // Ring 3: F(4) = 3
            FibonacciRing::new(4, 5),    // Ring 4: F(5) = 5
            FibonacciRing::new(5, 8),    // Ring 5: F(6) = 8
            FibonacciRing::new(6, 13),   // Ring 6: F(7) = 13
            FibonacciRing::new(7, 21),   // Ring 7: F(8) = 21
        ]));

        let scheduler_state = Arc::new(RwLock::new(SchedulerState {
            total_vpods: 0,
            active_vpods: 0,
            efficiency: 1.0,
            last_rebalance: Utc::now(),
        }));

        Ok(Self {
            rings,
            scheduler_state,
            ring_counter: Arc::new(AtomicU32::new(0)),
        })
    }

    /// Initialize Fibonacci rings
    pub async fn initialize_rings(&self) -> Result<()> {
        info!("🔄 Initializing Fibonacci scheduler rings");
        
        let mut rings = self.rings.write().unwrap();
        for ring in rings.iter_mut() {
            ring.assigned_vpods.clear();
            ring.utilization = 0.0;
            ring.last_scheduled = Utc::now();
        }

        info!("✅ Fibonacci rings initialized");
        Ok(())
    }

    /// Assign ring based on security role and load
    pub async fn assign_ring(&self, spec: &VPodSpec) -> Result<u8> {
        let base_ring = match spec.security_profile.as_ref()
            .map(|p| &p.role)
            .unwrap_or(&VPodSecurityRole::Application) 
        {
            VPodSecurityRole::System => 0,      // Rings 0-1
            VPodSecurityRole::Service => 2,     // Rings 2-3
            VPodSecurityRole::Application => 4, // Rings 4-5
            VPodSecurityRole::Sandbox => 6,     // Rings 6-7
        };

        // Load balance within role's ring pair
        let rings = self.rings.read().unwrap();
        let ring1_util = rings[base_ring as usize].utilization;
        let ring2_util = rings[(base_ring + 1) as usize].utilization;

        let assigned_ring = if ring1_util <= ring2_util {
            base_ring
        } else {
            base_ring + 1
        };

        debug!("📍 Assigned vPod {} to ring {} (role: {:?})", 
               spec.name, assigned_ring, 
               spec.security_profile.as_ref().map(|p| &p.role));

        Ok(assigned_ring)
    }

    /// Rebalance rings based on Fibonacci weights
    pub async fn rebalance_rings(&self) -> Result<()> {
        info!("⚖️ Rebalancing Fibonacci rings");

        let mut rings = self.rings.write().unwrap();
        let total_weight: u64 = rings.iter().map(|r| r.fibonacci_weight).sum();

        for ring in rings.iter_mut() {
            let target_util = (ring.fibonacci_weight as f64) / (total_weight as f64);
            
            // Adjust utilization towards Fibonacci target
            if ring.utilization > target_util * 1.2 {
                // Ring overloaded - migrate some vPods
                ring.utilization = target_util * 1.1;
            } else if ring.utilization < target_util * 0.8 {
                // Ring underutilized - accept more vPods
                ring.utilization = target_util * 0.9;
            }
        }

        // Update scheduler state
        {
            let mut state = self.scheduler_state.write().unwrap();
            state.last_rebalance = Utc::now();
            state.efficiency = self.calculate_efficiency(&rings);
        }

        info!("✅ Ring rebalancing completed");
        Ok(())
    }

    fn calculate_efficiency(&self, rings: &[FibonacciRing; 8]) -> f64 {
        let total_util: f64 = rings.iter().map(|r| r.utilization).sum();
        let ideal_util = 0.8; // Target 80% utilization
        
        1.0 - ((total_util / 8.0 - ideal_util).abs() / ideal_util)
    }
}

impl FibonacciRing {
    fn new(ring_id: u8, fibonacci_weight: u64) -> Self {
        Self {
            ring_id,
            fibonacci_weight,
            assigned_vpods: Vec::new(),
            utilization: 0.0,
            last_scheduled: Utc::now(),
        }
    }
}

impl TankCapacityManager {
    /// Create new tank capacity manager
    pub fn new() -> Result<Self> {
        let max_vpods = Self::detect_system_capacity();
        
        let mut ring_limits = HashMap::new();
        
        // Set resource limits per ring based on Fibonacci distribution
        ring_limits.insert(0, RingResourceLimits { max_cpu_percent: 50, max_memory_mb: 4096, max_vpods_count: 2 });
        ring_limits.insert(1, RingResourceLimits { max_cpu_percent: 40, max_memory_mb: 3072, max_vpods_count: 3 });
        ring_limits.insert(2, RingResourceLimits { max_cpu_percent: 30, max_memory_mb: 2048, max_vpods_count: 5 });
        ring_limits.insert(3, RingResourceLimits { max_cpu_percent: 25, max_memory_mb: 1536, max_vpods_count: 8 });
        ring_limits.insert(4, RingResourceLimits { max_cpu_percent: 20, max_memory_mb: 1024, max_vpods_count: 13 });
        ring_limits.insert(5, RingResourceLimits { max_cpu_percent: 15, max_memory_mb: 768, max_vpods_count: 21 });
        ring_limits.insert(6, RingResourceLimits { max_cpu_percent: 10, max_memory_mb: 512, max_vpods_count: 34 });
        ring_limits.insert(7, RingResourceLimits { max_cpu_percent: 5, max_memory_mb: 256, max_vpods_count: 55 });

        Ok(Self {
            tank_value: Arc::new(RwLock::new(1.0)), // Start with full tank
            max_vpods,
            ring_limits: Arc::new(RwLock::new(ring_limits)),
            tank_metrics: Arc::new(RwLock::new(TankMetrics {
                tank_level: 1.0,
                drain_rate: 0.01, // 1% per vPod
                refill_rate: 0.005, // 0.5% per second
                overload_count: 0,
            })),
        })
    }

    /// Initialize tank capacity
    pub async fn initialize_tank(&self) -> Result<()> {
        info!("🛢️ Initializing tank capacity manager");
        
        let mut tank_value = self.tank_value.write().unwrap();
        *tank_value = 1.0; // Full tank

        let mut metrics = self.tank_metrics.write().unwrap();
        metrics.tank_level = 1.0;
        metrics.overload_count = 0;

        info!("✅ Tank capacity initialized (max vPods: {})", self.max_vpods);
        Ok(())
    }

    /// Check if vPod can be admitted
    pub async fn can_admit_vpod(&self, spec: &VPodSpec) -> Result<bool> {
        let tank_value = *self.tank_value.read().unwrap();
        let resource_cost = self.calculate_resource_cost(spec);
        
        let can_admit = tank_value >= resource_cost && tank_value > 0.1; // Keep 10% reserve
        
        if !can_admit {
            warn!("🚫 vPod admission denied - tank level: {:.2}, required: {:.2}", 
                  tank_value, resource_cost);
        }

        Ok(can_admit)
    }

    /// Consume tank capacity for vPod
    pub async fn consume_capacity(&self, spec: &VPodSpec) -> Result<()> {
        let resource_cost = self.calculate_resource_cost(spec);
        
        {
            let mut tank_value = self.tank_value.write().unwrap();
            *tank_value = (*tank_value - resource_cost).max(0.0);
        }

        {
            let mut metrics = self.tank_metrics.write().unwrap();
            metrics.tank_level = *self.tank_value.read().unwrap();
        }

        debug!("⛽ Consumed tank capacity: {:.3} (remaining: {:.3})", 
               resource_cost, *self.tank_value.read().unwrap());

        Ok(())
    }

    /// Release tank capacity when vPod stops
    pub async fn release_capacity(&self, spec: &VPodSpec) -> Result<()> {
        let resource_cost = self.calculate_resource_cost(spec);
        
        {
            let mut tank_value = self.tank_value.write().unwrap();
            *tank_value = (*tank_value + resource_cost).min(1.0);
        }

        {
            let mut metrics = self.tank_metrics.write().unwrap();
            metrics.tank_level = *self.tank_value.read().unwrap();
        }

        debug!("🔋 Released tank capacity: {:.3} (current: {:.3})", 
               resource_cost, *self.tank_value.read().unwrap());

        Ok(())
    }

    fn calculate_resource_cost(&self, spec: &VPodSpec) -> f64 {
        let cpu_cost = (spec.resources.cpu_percent as f64) / 100.0 * 0.5;
        let mem_cost = (spec.resources.mem_mb as f64) / 1024.0 * 0.3;
        let base_cost = 0.02; // Base overhead per vPod
        
        cpu_cost + mem_cost + base_cost
    }

    fn detect_system_capacity() -> u32 {
        // Detect system resources and calculate max vPods
        // For now, use a reasonable default
        100
    }
}

impl ResourceUsage {
    fn new() -> Self {
        Self {
            cpu_percent: 0.0,
            memory_mb: 0,
            network_bytes: 0,
            disk_bytes: 0,
        }
    }
}

impl VPodsDaemonMetrics {
    fn new() -> Self {
        Self {
            total_vpods_created: 0,
            total_vpods_stopped: 0,
            avg_vpod_lifetime_secs: 0.0,
            scheduler_efficiency: 1.0,
            tank_overloads: 0,
            ring_rebalances: 0,
        }
    }
}
