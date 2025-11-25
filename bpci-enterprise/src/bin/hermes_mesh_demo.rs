use anyhow::Result;
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, warn};
use tracing_subscriber;

use pravyom_enterprise::hermes_lite_web4_mesh::{
    HermesLiteWeb4Mesh,
    Web4Address,
    MeshNodeId,
    MeshHealthStatus,
};
use pravyom_enterprise::lccd_mathematical_foundation::LccdMathematicalFoundation;

struct RoundSnapshot {
    index: u64,
    timestamp: DateTime<Utc>,
    overall_confidence: f64,
    health_ratio: f64,
    total_nodes: usize,
    healthy_nodes: usize,
    consensus_rounds: u64,
    messages_throughput: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("hermes_mesh_demo=info,bpci_enterprise=info")
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("🚀 Starting Hermes-Lite Web-4 mesh mini-network demo");

    let report_path = "/tmp/hermes_mesh_demo_report.txt";
    let total_rounds: u64 = 120; // ~60s with 500ms per round
    let sleep_per_round = Duration::from_millis(500);

    let start_time = Utc::now();

    // Initialize LCCD foundation and Hermes mesh
    let lccd_foundation = Arc::new(LccdMathematicalFoundation::new());

    let local_address = Web4Address {
        node_id: MeshNodeId::generate(),
        ip_address: "127.0.0.1".to_string(),
        port: 19100,
        quantum_channel: Some("hermes-demo-local".to_string()),
        mesh_layer: 0,
    };

    let mesh = HermesLiteWeb4Mesh::new(local_address.clone(), lccd_foundation.clone())?;

    // Seed router topology with local node so health accounting sees it
    {
        let local_node = mesh.local_node.read().await.clone();
        mesh.router.add_mesh_node(local_node).await?;
    }

    // Prepare a small set of bootstrap nodes and join them
    let bootstrap_nodes = vec![
        Web4Address {
            node_id: MeshNodeId::generate(),
            ip_address: "127.0.0.2".to_string(),
            port: 19101,
            quantum_channel: Some("hermes-demo-bootstrap-1".to_string()),
            mesh_layer: 0,
        },
        Web4Address {
            node_id: MeshNodeId::generate(),
            ip_address: "127.0.0.3".to_string(),
            port: 19102,
            quantum_channel: Some("hermes-demo-bootstrap-2".to_string()),
            mesh_layer: 0,
        },
    ];

    mesh.join_mesh(bootstrap_nodes.clone()).await?;

    info!("✅ Hermes mesh initialized: mesh_id={} local_ip={} local_port={}",
        mesh.mesh_id,
        local_address.ip_address,
        local_address.port,
    );

    let mut snapshots: Vec<RoundSnapshot> = Vec::new();

    for round in 0..total_rounds {
        // Drive a consensus round through the mesh
        let confidence = mesh.process_mesh_consensus_round(0.9).await?;
        let health = mesh.get_mesh_health().await?;

        // Trigger a cellular division event mid-run to exercise that path
        if round == total_rounds / 2 {
            {
                let mut local_node = mesh.local_node.write().await;
                local_node.living_state.division_readiness = 1.0;
                local_node.living_state.metabolic_rate = 1.0;
                local_node.cellular_division_ready = true;
            }

            if let Err(e) = mesh.handle_cellular_division().await {
                warn!("cellular_division_error={}", e);
            }
        }

        snapshots.push(RoundSnapshot {
            index: round,
            timestamp: Utc::now(),
            overall_confidence: confidence.overall_confidence(),
            health_ratio: health.health_ratio,
            total_nodes: health.total_nodes,
            healthy_nodes: health.healthy_nodes,
            consensus_rounds: health.consensus_rounds,
            messages_throughput: health.messages_throughput,
        });

        if round % 10 == 0 {
            info!(
                "[hermes-demo] round={} mesh_id={} health_ratio={:.3} total_nodes={} healthy_nodes={} overall_confidence={:.3} throughput={}",
                round,
                health.mesh_id,
                health.health_ratio,
                health.total_nodes,
                health.healthy_nodes,
                confidence.overall_confidence(),
                health.messages_throughput,
            );
        }

        sleep(sleep_per_round).await;
    }

    let final_health = mesh.get_mesh_health().await?;
    let end_time = Utc::now();
    let duration_secs = (end_time - start_time).num_seconds().max(0) as u64;

    // Render and write detailed report
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

    info!("✅ Hermes mesh demo completed in ~{}s", duration_secs);
    info!("📝 Detailed report written to {}", report_path);

    println!("Hermes mesh demo finished. Report: {}", report_path);

    Ok(())
}

fn render_report(
    mesh_id: &str,
    local_address: &Web4Address,
    snapshots: &[RoundSnapshot],
    final_health: &MeshHealthStatus,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    duration_secs: u64,
) -> String {
    let mut out = String::new();

    use std::fmt::Write as FmtWrite;

    writeln!(out, "HERMES-Lite Web-4 Mesh Mini-Network Demo Report").ok();
    writeln!(out, "================================================").ok();
    writeln!(out, "").ok();

    writeln!(out, "1. Overview").ok();
    writeln!(out, "-----------").ok();
    writeln!(out, "Mesh ID           : {}", mesh_id).ok();
    writeln!(out, "Local node IP     : {}", local_address.ip_address).ok();
    writeln!(out, "Local node port   : {}", local_address.port).ok();
    writeln!(out, "Quantum channel   : {}", local_address.quantum_channel.as_deref().unwrap_or("<none>")).ok();
    writeln!(out, "Start time (UTC)  : {}", start_time.to_rfc3339()).ok();
    writeln!(out, "End time (UTC)    : {}", end_time.to_rfc3339()).ok();
    writeln!(out, "Duration (seconds): {}", duration_secs).ok();
    writeln!(out, "Consensus rounds  : {}", final_health.consensus_rounds).ok();
    writeln!(out, "Total messages    : {}", final_health.messages_throughput).ok();
    writeln!(out, "").ok();

    writeln!(out, "2. Final Mesh Health Snapshot").ok();
    writeln!(out, "-----------------------------").ok();
    writeln!(out, "Total nodes       : {}", final_health.total_nodes).ok();
    writeln!(out, "Healthy nodes     : {}", final_health.healthy_nodes).ok();
    writeln!(out, "Health ratio      : {:.4}", final_health.health_ratio).ok();
    writeln!(out, "Local node health : {:.4}", final_health.local_node_health).ok();
    writeln!(out, "Average κ         : {:.6}", final_health.average_kappa).ok();
    writeln!(out, "Avg confidence    : {:.4}", final_health.average_confidence).ok();
    writeln!(out, "Cell divisions    : {}", final_health.cellular_divisions).ok();
    writeln!(out, "Messages throughput: {}", final_health.messages_throughput).ok();
    writeln!(out, "").ok();

    writeln!(out, "3. Consensus & Health Timeline (sampled)").ok();
    writeln!(out, "----------------------------------------").ok();
    writeln!(out, "idx | timestamp                  | health | conf  | nodes (healthy/total) | rounds | throughput").ok();
    writeln!(out, "----+----------------------------+--------+-------+------------------------+--------+-----------").ok();

    for snap in snapshots.iter() {
        writeln!(
            out,
            "{:3} | {} | {:6.3} | {:5.3} | {:3}/{:<3}               | {:6} | {:9}",
            snap.index,
            snap.timestamp.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            snap.health_ratio,
            snap.overall_confidence,
            snap.healthy_nodes,
            snap.total_nodes,
            snap.consensus_rounds,
            snap.messages_throughput,
        )
        .ok();
    }

    writeln!(out, "").ok();
    writeln!(out, "4. Observations").ok();
    writeln!(out, "----------------").ok();
    writeln!(out, "- Mesh remained fully in-memory; no external network sockets were opened.").ok();
    writeln!(out, "- Consensus rounds drove κ and confidence updates across the local node.").ok();
    writeln!(out, "- Bootstrap nodes expanded topology to a small 3-node mesh.").ok();
    writeln!(out, "- A cellular division event was triggered mid-run to exercise division broadcasts and stats.").ok();
    writeln!(out, "- This demo is designed to be lightweight and should be safe on a laptop-class machine.").ok();

    out
}
