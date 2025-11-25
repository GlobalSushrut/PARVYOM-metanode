pub mod id {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct NodeId(pub Uuid);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct VpodId(pub Uuid);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct HyperCellId(pub Uuid);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct RingLevel(pub u8); // 0 = highest priority
}

pub mod capacity {
    use super::id::RingLevel;

    #[derive(Debug, Clone, Copy)]
    pub struct NodeCapacity {
        pub cores: u32,
        pub ram_mb: u64,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct TankConfig {
        pub value: f32,
        pub base_vpods_per_core: u32,
        pub alpha_cpu: f32,
        pub beta_ram: f32,
        pub delta_cells: f32,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct RingCapacity {
        pub ring: RingLevel,
        pub fib_value: u32,
        pub max_hypercells: u32,
        pub cpu_share: f32,
    }

    #[derive(Debug, Clone)]
    pub struct CapacityLimits {
        pub vpods_max: u32,
        pub hypercells_max: u32,
        pub ring_caps: Vec<RingCapacity>,
    }

    #[derive(Debug, Clone)]
    pub struct CapacityGovernor {
        pub node_capacity: NodeCapacity,
        pub tank: TankConfig,
        pub limits: CapacityLimits,
    }

    impl CapacityGovernor {
        pub fn new(node_capacity: NodeCapacity, tank: TankConfig) -> Self {
            let base_local_vpods = tank.base_vpods_per_core * node_capacity.cores;
            let math_limit_vpods = (tank.value * base_local_vpods as f32) as u32;

            // CPU safety bound
            let b: u32 = 16; // baseline safe jobs per core
            let cpu_limit_vpods = (tank.alpha_cpu * (node_capacity.cores * b) as f32) as u32;

            // RAM safety bound
            let mem_mb_per_vpod: u64 = 128;
            let raw_ram_limit_vpods = if mem_mb_per_vpod == 0 {
                u32::MAX
            } else {
                (node_capacity.ram_mb / mem_mb_per_vpod) as u32
            };
            let vpods_ram_limit = (tank.beta_ram * raw_ram_limit_vpods as f32) as u32;

            let vpods_max = math_limit_vpods
                .min(cpu_limit_vpods)
                .min(vpods_ram_limit);

            // Simple global HyperCell limit for v0.1
            let hypercells_max = (tank.delta_cells * (node_capacity.cores as f32) * 10_000.0) as u32;

            let ring_caps = Self::default_ring_caps();

            Self {
                node_capacity,
                tank,
                limits: CapacityLimits {
                    vpods_max,
                    hypercells_max,
                    ring_caps,
                },
            }
        }

        fn default_ring_caps() -> Vec<RingCapacity> {
            // First few Fibonacci numbers for rings 0..7
            let fib: [u32; 8] = [1, 1, 2, 3, 5, 8, 13, 21];
            let sum_f: u32 = fib.iter().sum();
            let lambda: u32 = 32; // hypercells factor
            let mu: f32 = 1.0;    // total CPU share budget (1.0 = 100%)

            fib.iter()
                .enumerate()
                .map(|(idx, &f_k)| {
                    let ring = RingLevel(idx as u8);
                    let max_hypercells = lambda * f_k;
                    let cpu_share = if sum_f == 0 {
                        0.0
                    } else {
                        mu * (f_k as f32) / (sum_f as f32)
                    };

                    RingCapacity {
                        ring,
                        fib_value: f_k,
                        max_hypercells,
                        cpu_share,
                    }
                })
                .collect()
        }
    }
}

pub mod vpod {
    use super::id::{NodeId, RingLevel, VpodId};
    use serde::{Deserialize, Serialize};
    use std::path::PathBuf;

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct ResourceLimits {
        pub cpu_percent: u8,
        pub mem_mb: u64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum VpodStatus {
        Pending,
        Running,
        Stopped,
        Failed(String),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VpodSpec {
        pub id: VpodId,
        pub name: String,
        pub cmd: Vec<String>,
        pub env: Vec<(String, String)>,
        pub cwd: Option<PathBuf>,
        pub resources: ResourceLimits,
        pub ring_hint: Option<RingLevel>,
        pub security_profile: Option<SecurityProfile>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SecurityProfile {
        pub role: SecurityRole,
        pub seccomp_policy: Option<String>,
        pub network_policy: Option<String>,
        pub capabilities: Vec<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum SecurityRole {
        System,      // Full system access (ring 0-1)
        Service,     // Network services (ring 2-3)
        Application, // User applications (ring 4-5)
        Sandbox,     // Isolated workloads (ring 6-7)
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Vpod {
        pub spec: VpodSpec,
        pub status: VpodStatus,
        pub node: NodeId,
        pub created_at_ns: u64,
    }
}

pub mod hypercell {
    use super::id::{HyperCellId, RingLevel, VpodId};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HyperCellSpec {
        pub id: HyperCellId,
        pub vpod_id: VpodId,
        pub ring: RingLevel,
        pub cpu_weight: u32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HyperCellState {
        pub spec: HyperCellSpec,
        pub pid: Option<i32>,
        pub alive: bool,
        pub cpu_nanos: u64,
    }
}

pub mod epoch {
    use super::id::{HyperCellId, RingLevel, VpodId};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RingStats {
        pub ring: RingLevel,
        pub hypercells: u32,
        pub cpu_percent: f32,
        pub mem_mb: f32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum SchedEventKind {
        HyperCellSpawn,
        HyperCellExit,
        HyperCellMoveRing { from: RingLevel, to: RingLevel },
        VpodStart,
        VpodStop,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SchedEvent {
        pub kind: SchedEventKind,
        pub hypercell_id: HyperCellId,
        pub vpod_id: VpodId,
        pub ring: RingLevel,
        pub timestamp_ns: u64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SchedBlock {
        pub index: u64,
        pub prev_hash: [u8; 32],
        pub root_hypercell_hash: [u8; 32],
        pub ring_stats: Vec<RingStats>,
        pub tank_value: f32,
        pub events: Vec<SchedEvent>,
        pub timestamp_ns: u64,
    }

    pub trait EpochLog {
        fn append_block(&mut self, block: &SchedBlock) -> anyhow::Result<()>;
        fn last_block(&self) -> anyhow::Result<Option<SchedBlock>>;
        fn get_block(&self, index: u64) -> anyhow::Result<Option<SchedBlock>>;
    }

    impl SchedBlock {
        pub fn compute_hash(&self) -> [u8; 32] {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};

            let mut hasher = DefaultHasher::new();
            self.index.hash(&mut hasher);
            self.prev_hash.hash(&mut hasher);
            self.root_hypercell_hash.hash(&mut hasher);
            self.tank_value.to_bits().hash(&mut hasher);
            self.timestamp_ns.hash(&mut hasher);
            
            let hash_value = hasher.finish();
            let mut result = [0u8; 32];
            result[..8].copy_from_slice(&hash_value.to_le_bytes());
            result
        }
    }
}
