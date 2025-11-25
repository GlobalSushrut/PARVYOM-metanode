use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::info;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

use vpods_agent::{LinuxVpodRuntime, LinuxVpodRuntimeConfig, VpodRuntime};
use vpods_core::id::{NodeId, VpodId, RingLevel};
use vpods_core::vpod::{ResourceLimits, VpodSpec, VpodStatus, Vpod};

#[derive(Parser, Debug)]
#[command(name = "vpods-cli")]
#[command(about = "vPods control CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a new vPod with the given command
    Run {
        /// CPU share in percent (1-100)
        #[arg(long, default_value_t = 10)]
        cpu: u8,
        /// Memory limit in MB
        #[arg(long, default_value_t = 256)]
        mem: u64,
        /// Optional vPod name
        #[arg(long, default_value = "vpod")]
        name: String,
        /// Optional ring level (0 = highest priority)
        #[arg(long)]
        ring: Option<u8>,
        /// Command and arguments to execute
        #[arg(required = true, trailing_var_arg = true)]
        cmd: Vec<String>,
    },
}

fn main() -> Result<()> {
    init_tracing();

    let cli = Cli::parse();

    match cli.command {
        Commands::Run { cpu, mem, name, ring, cmd } => {
            run_vpod(cpu, mem, name, ring, cmd)?;
        }
    }

    Ok(())
}

fn run_vpod(cpu: u8, mem: u64, name: String, ring: Option<u8>, cmd: Vec<String>) -> Result<()> {
    let node_id = NodeId(Uuid::new_v4());
    let config = LinuxVpodRuntimeConfig::default();
    let runtime = LinuxVpodRuntime::with_config(node_id, config)?;

    let vpod_id = VpodId(Uuid::new_v4());

    let spec = VpodSpec {
        id: vpod_id,
        name,
        cmd,
        env: Vec::new(),
        cwd: None,
        resources: ResourceLimits {
            cpu_percent: cpu.max(1).min(100),
            mem_mb: mem,
        },
        ring_hint: ring.map(RingLevel),
        security_profile: None,
    };

    let vpod = runtime.create_vpod(&spec)?;
    let pid = runtime.start_vpod(&vpod)?;

    info!(
        vpod_id = ?vpod.spec.id,
        pid,
        cpu_percent = vpod.spec.resources.cpu_percent,
        mem_mb = vpod.spec.resources.mem_mb,
        ring = vpod.spec.ring_hint.map(|r| r.0),
        "started vPod",
    );

    Ok(())
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
}
