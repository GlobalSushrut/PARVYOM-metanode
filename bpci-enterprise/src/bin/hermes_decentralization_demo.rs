use anyhow::Result;
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::Write as IoWrite;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, warn};

use pravyom_enterprise::hermes_lite_web4_mesh::{
    HermesLiteWeb4Mesh,
    Web4Address,
    MeshNodeId,
    MeshHealthStatus,
};
use pravyom_enterprise::lccd_mathematical_foundation::LccdMathematicalFoundation;

struct PhaseSnapshot {
    index: u64,
    timestamp: DateTime<Utc>,
    health_ratio: f64,
    overall_confidence: f64,
    total_nodes: usize,
    healthy_nodes: usize,
    consensus_rounds: u64,
    messages_throughput: u64,
    cellular_divisions: u64,
    phase: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("hermes_decentralization_demo=info,bpci_enterprise=info")
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("Starting Hermes P2P decentralization phase simulation");

    let report_path = "/tmp/hermes_decentralization_demo_report.txt";
    let total_rounds: u64 = 120; // ~60s with 500ms per round
    let sleep_per_round = Duration::from_millis(500);

    let start_time = Utc::now();

    // Initialize LCCD foundation and base Hermes mesh
    let lccd_foundation = Arc::new(LccdMathematicalFoundation::new());

    let local_address = Web4Address {
        node_id: MeshNodeId::generate(),
        ip_address: "127.0.0.1".to_string(),
        port: 19200,
        quantum_channel: Some("hermes-decentralization-local".to_string()),
        mesh_layer: 0,
    };

    let mesh = HermesLiteWeb4Mesh::new(local_address.clone(), lccd_foundation.clone())?;

    // Seed router topology with local node
    {
        let local_node = mesh.local_node.read().await.clone();
        mesh.router.add_mesh_node(local_node).await?;
    }

    // Prepare a pool of potential BPI OS nodes that can join over time
    let mut bpios_addresses: Vec<Web4Address> = Vec::new();
    for i in 0..16u16 {
        bpios_addresses.push(Web4Address {
            node_id: MeshNodeId::generate(),
            ip_address: format!("127.0.1.{}", i + 1),
            port: 19210 + i,
            quantum_channel: Some(format!("hermes-bpios-{}", i + 1)),
            mesh_layer: 0,
        });
    }

    // Initially connect a very small set (centralized + a couple of satellites)
    mesh.join_mesh(bpios_addresses[0..2].to_vec()).await?;

    info!(
        "Hermes mesh base initialized: mesh_id={} local_ip={} local_port={}",
        mesh.mesh_id,
        local_address.ip_address,
        local_address.port,
    );

    let mut snapshots: Vec<PhaseSnapshot> = Vec::new();

    for round in 0..total_rounds {
        // Gradually connect more BPI OS (BPIOS) nodes as simulation progresses
        if round % 20 == 0 {
            let idx = (round / 20) as usize;
            let start = 2 + idx * 2;
            let end = std::cmp::min(start + 2, bpios_addresses.len());
            if start < end {
                let batch = bpios_addresses[start..end].to_vec();
                info!("[hermes-dec] round={} connecting {} new BPIOS nodes", round, batch.len());
                if let Err(e) = mesh.join_mesh(batch).await {
                    warn!("bpios_join_error={} round={}", e, round);
                }
            }
        }

        // Drive a consensus round
        let confidence = mesh.process_mesh_consensus_round(0.9).await?;
        let health = mesh.get_mesh_health().await?;

        // Trigger multiple cellular division events at different growth stages
        if round == total_rounds / 4
            || round == total_rounds / 2
            || round == (3 * total_rounds) / 4
        {
            {
                let mut local_node = mesh.local_node.write().await;
                local_node.living_state.division_readiness = 1.0;
                local_node.living_state.metabolic_rate = 1.0;
                local_node.cellular_division_ready = true;
            }

            if let Err(e) = mesh.handle_cellular_division().await {
                warn!("cellular_division_error={} round={}", e, round);
            }
        }

        let phase = classify_phase(&health);

        snapshots.push(PhaseSnapshot {
            index: round,
            timestamp: Utc::now(),
            health_ratio: health.health_ratio,
            // Use per-round consensus confidence, as in hermes_mesh_demo
            overall_confidence: confidence.overall_confidence(),
            total_nodes: health.total_nodes,
            healthy_nodes: health.healthy_nodes,
            consensus_rounds: health.consensus_rounds,
            messages_throughput: health.messages_throughput,
            cellular_divisions: health.cellular_divisions,
            phase,
        });

        if round % 10 == 0 {
            info!(
                "[hermes-dec] round={} phase={} nodes={}/{} health_ratio={:.3} overall_conf={:.3} divs={}",
                round,
                snapshots.last().map(|s| s.phase.as_str()).unwrap_or("-"),
                health.healthy_nodes,
                health.total_nodes,
                health.health_ratio,
                confidence.overall_confidence(),
                health.cellular_divisions,
            );
        }

        sleep(sleep_per_round).await;
    }

    let final_health = mesh.get_mesh_health().await?;
    let end_time = Utc::now();
    let duration_secs = (end_time - start_time).num_seconds().max(0) as u64;

    let report = render_report(
        &mesh.mesh_id,
        &local_address,
        &snapshots,
        &final_health,
        start_time,
        end_time,
        duration_secs,
    );

    let mut file = File::create(report_path)?;
    file.write_all(report.as_bytes())?;

    info!(
        "Hermes decentralization simulation completed in ~{}s (report: {})",
        duration_secs,
        report_path
    );

    println!("Hermes decentralization simulation finished. Report: {}", report_path);

    Ok(())
}

fn classify_phase(health: &MeshHealthStatus) -> String {
    let nodes = health.total_nodes as u64;
    let divs = health.cellular_divisions;
    let hr = health.health_ratio;

    // Very small mesh: effectively centralized / constellation phase
    if nodes <= 3 {
        return "Centralized Constellation".to_string();
    }

    // Nodes starting to join but health still stabilizing
    if nodes > 3 && nodes < 8 {
        return "NodeConnectionSync (Hermes P2P forming)".to_string();
    }

    // Many nodes, but either health or cellular replication not yet mature
    if nodes >= 8 && (hr < 0.9 || divs < 3) {
        return "MeshEvolution (decentralization in progress)".to_string();
    }

    // High node count, good health, and multiple cellular divisions: full decentralization
    if nodes >= 8 && hr >= 0.9 && divs >= 3 {
        return "AutonomousMesh (full decentralization)".to_string();
    }

    "UnknownPhase".to_string()
}

fn render_report(
    mesh_id: &str,
    local_address: &Web4Address,
    snapshots: &[PhaseSnapshot],
    final_health: &MeshHealthStatus,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    duration_secs: u64,
) -> String {
    use std::fmt::Write as FmtWrite;

    let mut out = String::new();

    writeln!(out, "Hermes P2P Decentralization Phase Simulation Report").ok();
    writeln!(out, "====================================================").ok();
    writeln!(out, "").ok();

    writeln!(out, "1. Overview").ok();
    writeln!(out, "-----------").ok();
    writeln!(out, "Mesh ID              : {}", mesh_id).ok();
    writeln!(out, "Local node IP        : {}", local_address.ip_address).ok();
    writeln!(out, "Local node port      : {}", local_address.port).ok();
    writeln!(out, "Quantum channel      : {}", local_address.quantum_channel.as_deref().unwrap_or("<none>")).ok();
    writeln!(out, "Start time (UTC)     : {}", start_time.to_rfc3339()).ok();
    writeln!(out, "End time (UTC)       : {}", end_time.to_rfc3339()).ok();
    writeln!(out, "Duration (seconds)   : {}", duration_secs).ok();
    writeln!(out, "Final total nodes    : {}", final_health.total_nodes).ok();
    writeln!(out, "Final healthy nodes  : {}", final_health.healthy_nodes).ok();
    writeln!(out, "Final health ratio   : {:.4}", final_health.health_ratio).ok();
    writeln!(out, "Cellular divisions   : {}", final_health.cellular_divisions).ok();
    writeln!(out, "Consensus rounds     : {}", final_health.consensus_rounds).ok();
    writeln!(out, "Total messages       : {}", final_health.messages_throughput).ok();
    writeln!(out, "").ok();

    writeln!(out, "2. Phase Timeline (sampled)").ok();
    writeln!(out, "---------------------------").ok();
    writeln!(out, "idx | timestamp                  | phase                                     | nodes (healthy/total) | health | conf  | divs | rounds | throughput").ok();
    writeln!(out, "----+----------------------------+-------------------------------------------+------------------------+--------+-------+------+--------+-----------").ok();

    for snap in snapshots.iter() {
        writeln!(
            out,
            "{:3} | {} | {:41} | {:3}/{:<3}               | {:6.3} | {:5.3} | {:4} | {:6} | {:9}",
            snap.index,
            snap.timestamp.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            snap.phase,
            snap.healthy_nodes,
            snap.total_nodes,
            snap.health_ratio,
            snap.overall_confidence,
            snap.cellular_divisions,
            snap.consensus_rounds,
            snap.messages_throughput,
        )
        .ok();
    }

    writeln!(out, "").ok();
    writeln!(out, "3. Phase Summary").ok();
    writeln!(out, "----------------").ok();

    let mut first_centralized: Option<&PhaseSnapshot> = None;
    let mut first_sync: Option<&PhaseSnapshot> = None;
    let mut first_evolution: Option<&PhaseSnapshot> = None;
    let mut first_autonomous: Option<&PhaseSnapshot> = None;

    for snap in snapshots.iter() {
        match snap.phase.as_str() {
            "Centralized Constellation" => {
                if first_centralized.is_none() {
                    first_centralized = Some(snap);
                }
            }
            "NodeConnectionSync (Hermes P2P forming)" => {
                if first_sync.is_none() {
                    first_sync = Some(snap);
                }
            }
            "MeshEvolution (decentralization in progress)" => {
                if first_evolution.is_none() {
                    first_evolution = Some(snap);
                }
            }
            "AutonomousMesh (full decentralization)" => {
                if first_autonomous.is_none() {
                    first_autonomous = Some(snap);
                }
            }
            _ => {}
        }
    }

    if let Some(s) = first_centralized {
        writeln!(
            out,
            "- Centralized Constellation: round {} (nodes={} health_ratio={:.3})",
            s.index,
            s.total_nodes,
            s.health_ratio,
        )
        .ok();
    }
    if let Some(s) = first_sync {
        writeln!(
            out,
            "- NodeConnectionSync: round {} (nodes={} health_ratio={:.3})",
            s.index,
            s.total_nodes,
            s.health_ratio,
        )
        .ok();
    }
    if let Some(s) = first_evolution {
        writeln!(
            out,
            "- MeshEvolution: round {} (nodes={} divs={} health_ratio={:.3})",
            s.index,
            s.total_nodes,
            s.cellular_divisions,
            s.health_ratio,
        )
        .ok();
    }
    if let Some(s) = first_autonomous {
        writeln!(
            out,
            "- AutonomousMesh (full decentralization): round {} (nodes={} divs={} health_ratio={:.3})",
            s.index,
            s.total_nodes,
            s.cellular_divisions,
            s.health_ratio,
        )
        .ok();
    } else {
        writeln!(
            out,
            "- AutonomousMesh not reached under current thresholds (nodes>=8, health_ratio>=0.9, divs>=3).",
        )
        .ok();
    }

    writeln!(out, "").ok();
    writeln!(out, "4. Observations").ok();
    writeln!(out, "----------------").ok();
    writeln!(out, "- Hermes mesh remained in-memory (no external sockets), safe for laptop runs.").ok();
    writeln!(out, "- BPI OS nodes were gradually attached to the mesh to simulate community growth.").ok();
    writeln!(out, "- Multiple cellular division events model how consensus cells replicate into the mesh.").ok();
    writeln!(out, "- The AutonomousMesh phase marks the point where BPCI runs as a fully decentralized Hermès P2P organism.").ok();

    out
}
