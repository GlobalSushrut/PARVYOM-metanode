use anyhow::Result;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

use blake3;
use hex;
use serde_json;

use vpods_agent::{LinuxVpodRuntime, LinuxVpodRuntimeConfig, VpodRuntime};
use vpods_core::id::NodeId;
use vpods_core::capacity::{CapacityGovernor, TankConfig, NodeCapacity};
use vpods_core::epoch::SchedBlock;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let node_id = NodeId(Uuid::new_v4());

    let config = LinuxVpodRuntimeConfig::default();
    let runtime = LinuxVpodRuntime::with_config(node_id, config)?;

    // Detect node capacity and log tank-based limits once at startup
    let node_capacity = runtime.detect_node_capacity()?;
    let tank = TankConfig {
        value: 1.0,
        base_vpods_per_core: 8,
        alpha_cpu: 0.8,
        beta_ram: 0.8,
        delta_cells: 0.5,
    };
    let governor = CapacityGovernor::new(node_capacity, tank);
    let tank_value = tank.value as f32;

    let mut sched_logger = SchedBlockLogger::new(None)?;

    info!(
        cores = node_capacity.cores,
        ram_mb = node_capacity.ram_mb,
        vpods_max = governor.limits.vpods_max,
        hypercells_max = governor.limits.hypercells_max,
        "vpods-daemon started with computed capacity"
    );

    // Simple metrics loop for now
    loop {
        match runtime.collect_system_metrics() {
            Ok(m) => {
                info!(
                    cpu = m.cpu_usage_percent,
                    mem_mb = m.memory_usage_mb,
                    mem_total_mb = m.memory_total_mb,
                    load1 = m.load_average_1min,
                    procs = m.processes_running,
                    "system metrics snapshot"
                );
            }
            Err(e) => {
                warn!(error = ?e, "failed to collect system metrics");
            }
        }

        if let Err(e) = sched_logger.log_block(tank_value) {
            warn!(error = ?e, "failed to log schedblock");
        }

        sleep(Duration::from_secs(5)).await;
    }
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
}

struct SchedBlockLogger {
    next_index: u64,
    prev_hash: [u8; 32],
    era_root: PathBuf,
    index_file: PathBuf,
}

impl SchedBlockLogger {
    fn new(root: Option<PathBuf>) -> Result<Self> {
        let era_root = root.unwrap_or_else(|| PathBuf::from("/era"));

        // Ensure ERA-FS logging dirs exist (parasite OS style: we just attach to existing /era)
        let store_root = era_root.join("store").join("objects");
        fs::create_dir_all(&store_root)?;

        let sched_dir = era_root
            .join("mutable")
            .join("var")
            .join("vpods")
            .join("schedblocks");
        fs::create_dir_all(&sched_dir)?;

        let index_file = sched_dir.join("index.log");

        Ok(Self {
            next_index: 0,
            prev_hash: [0u8; 32],
            era_root,
            index_file,
        })
    }

    fn log_block(&mut self, tank_value: f32) -> Result<()> {
        let index = self.next_index;
        self.next_index = self.next_index.wrapping_add(1);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0));
        let timestamp_ns = now.as_nanos() as u64;

        let block = SchedBlock {
            index,
            prev_hash: self.prev_hash,
            root_hypercell_hash: [0u8; 32],
            ring_stats: Vec::new(),
            tank_value,
            events: Vec::new(),
            timestamp_ns,
        };

        let bytes = serde_json::to_vec(&block)?;
        let hash = blake3::hash(&bytes);
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(hash.as_bytes());
        self.prev_hash = hash_bytes;

        let hash_hex = hex::encode(hash_bytes);
        let obj_dir = self
            .era_root
            .join("store")
            .join("objects")
            .join(format!("blake3-{}", hash_hex));
        fs::create_dir_all(&obj_dir)?;

        let data_path = obj_dir.join("data");
        fs::write(&data_path, &bytes)?;

        // Append to index log
        let mut index_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.index_file)?;
        writeln!(index_file, "{} blake3-{}", index, hash_hex)?;

        Ok(())
    }
}
