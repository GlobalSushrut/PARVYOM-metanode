use anyhow::{Result, anyhow};
use serde_json::json;
use uuid::Uuid;
use std::fs;
use std::path::Path;

use crate::vpods_docklock_integration::{VPodsClient, create_mesh_vpods_client};

/// High-level vPods cluster commands (k8++ style control-plane)
#[derive(Debug, Clone)]
pub enum VpodsClusterCommands {
    /// Create a new vPods cluster and initialize control-plane state
    Deploy,
    /// Show vPods cluster status
    Status,
    /// List vPods cluster nodes
    Nodes,
    /// Scale desired vPod replicas
    Scale { replicas: u32 },
    /// Register a vPods node with optional Unix/mesh endpoint metadata
    AddNode {
        node_id: String,
        unix_sock: Option<String>,
        mesh_service: Option<String>,
    },
    /// Remove a vPods node from the cluster
    RemoveNode {
        node_id: String,
    },
    /// Show vPods cluster capacity and ring statistics
    Metrics,
}

fn vpods_cluster_root() -> String {
    let era_root = "/era/mutable/var/bpi/vpods_cluster";
    if Path::new(era_root).exists() {
        era_root.to_string()
    } else {
        "/tmp/bpi_vpods_cluster".to_string()
    }
}

fn clusters_root() -> String {
    format!("{}/clusters", vpods_cluster_root())
}

fn current_cluster_path() -> String {
    format!("{}/current_cluster.json", vpods_cluster_root())
}

fn cluster_state_path(cluster_id: &str) -> String {
    format!("{}/clusters/{}/cluster_state.json", vpods_cluster_root(), cluster_id)
}

fn save_current_cluster_id(cluster_id: &str) -> Result<()> {
    let root = vpods_cluster_root();
    fs::create_dir_all(&root)?;
    let value = json!({ "cluster_id": cluster_id });
    fs::write(current_cluster_path(), serde_json::to_string_pretty(&value)?)?;
    Ok(())
}

fn load_current_cluster_id() -> Option<String> {
    let path = current_cluster_path();
    let content = fs::read_to_string(&path).ok()?;
    let value: serde_json::Value = serde_json::from_str(&content).ok()?;
    value.get("cluster_id").and_then(|v| v.as_str()).map(|s| s.to_string())
}

fn load_cluster_state() -> Option<(String, serde_json::Value)> {
    let cluster_id = load_current_cluster_id()?;
    let path = cluster_state_path(&cluster_id);
    let content = fs::read_to_string(&path).ok()?;
    let value = serde_json::from_str(&content).ok()?;
    Some((cluster_id, value))
}

fn save_cluster_state(cluster_id: &str, state: &serde_json::Value) -> Result<()> {
    let dir = format!("{}/{}", clusters_root(), cluster_id);
    fs::create_dir_all(&dir)?;
    let path = cluster_state_path(cluster_id);
    fs::write(path, serde_json::to_string_pretty(state)?)?;
    Ok(())
}

fn init_vpods_cluster_state(cluster_id: &str) -> Result<()> {
    let now = chrono::Utc::now().timestamp();
    let control_plane_node = json!({
        "node_id": format!("vpods-node-{}", Uuid::new_v4().simple()),
        "status": "active",
        "roles": ["control-plane"],
        "created_at": now,
        "transport": "unix",
        "endpoint": "/era/mutable/var/run/vpods-daemon.sock",
    });

    let state = json!({
        "cluster_id": cluster_id,
        "created_at": now,
        "desired_replicas": 1,
        "nodes": [control_plane_node],
    });

    save_cluster_state(cluster_id, &state)
}

fn get_active_node_count_from_state() -> Option<u32> {
    let (_cluster_id, state) = load_cluster_state()?;
    let nodes = state.get("nodes")?.as_array()?;
    Some(nodes.len() as u32)
}

fn get_cluster_uptime_from_state() -> Option<u64> {
    let (_cluster_id, state) = load_cluster_state()?;
    let created_at = state.get("created_at")?.as_i64().unwrap_or(0);
    let now = chrono::Utc::now().timestamp();
    if now > created_at {
        Some((now - created_at) as u64)
    } else {
        Some(0)
    }
}

fn get_cluster_nodes_from_state() -> Option<Vec<serde_json::Value>> {
    let (_cluster_id, state) = load_cluster_state()?;
    let nodes = state.get("nodes")?.as_array()?;
    Some(nodes.clone())
}

fn set_desired_replicas_for_active_cluster(replicas: u32) -> Result<()> {
    let (cluster_id, mut state) = load_cluster_state().ok_or_else(|| anyhow!("No active vPods cluster found"))?;
    state["desired_replicas"] = json!(replicas);
    save_cluster_state(&cluster_id, &state)
}

fn add_node_to_active_cluster(node_id: &str, unix_sock: &Option<String>, mesh_service: &Option<String>) -> Result<()> {
    let (cluster_id, mut state) = load_cluster_state().ok_or_else(|| anyhow!("No active vPods cluster found"))?;

    if state.get("nodes").is_none() {
        state["nodes"] = json!([]);
    }

    let now = chrono::Utc::now().timestamp();

    if let Some(nodes) = state.get_mut("nodes").and_then(|v| v.as_array_mut()) {
        if !nodes.iter().any(|n| n["node_id"].as_str() == Some(node_id)) {
            nodes.push(json!({
                "node_id": node_id,
                "status": "active",
                "roles": ["worker"],
                "created_at": now,
                "transport": if unix_sock.is_some() { "unix" } else if mesh_service.is_some() { "mesh" } else { "unknown" },
                "endpoint": unix_sock.as_ref().cloned().or_else(|| mesh_service.as_ref().cloned()).unwrap_or_default(),
            }));
        }
    }

    save_cluster_state(&cluster_id, &state)
}

fn remove_node_from_active_cluster(node_id: &str) -> Result<()> {
    let (cluster_id, mut state) = load_cluster_state().ok_or_else(|| anyhow!("No active vPods cluster found"))?;

    if let Some(nodes) = state.get_mut("nodes").and_then(|v| v.as_array_mut()) {
        nodes.retain(|n| n["node_id"].as_str() != Some(node_id));
    }

    save_cluster_state(&cluster_id, &state)
}

/// Handle vPods cluster control-plane commands (Phase 1: control-plane state only).
pub async fn handle(cmd: VpodsClusterCommands, json_output: bool, dry_run: bool) -> Result<()> {
    match cmd {
        VpodsClusterCommands::Deploy => {
            if dry_run {
                if json_output {
                    println!("{}", json!({
                        "status": "dry_run",
                        "command": "vpods_cluster_deploy",
                    }));
                } else {
                    println!("[DRY RUN] Would deploy vPods cluster (control-plane only)");
                }
                return Ok(());
            }

            let cluster_id = format!("vpods_cluster_{}", Uuid::new_v4().simple());
            init_vpods_cluster_state(&cluster_id)?;
            save_current_cluster_id(&cluster_id)?;

            if json_output {
                println!("{}", json!({
                    "status": "deployed",
                    "cluster_id": cluster_id,
                    "control_plane": true,
                    "desired_replicas": 1,
                }));
            } else {
                println!("🚀 vPods Cluster Deployed: {}", cluster_id);
                println!("   Control-plane node initialized");
            }
        }

        VpodsClusterCommands::Status => {
            let (cluster_id, _state) = load_cluster_state().ok_or_else(|| anyhow!("No active vPods cluster found"))?;
            let nodes = get_cluster_nodes_from_state().unwrap_or_default();
            let node_count = nodes.len();
            let uptime = get_cluster_uptime_from_state().unwrap_or(0);

            if json_output {
                println!("{}", json!({
                    "status": "active",
                    "cluster_id": cluster_id,
                    "nodes_active": node_count,
                    "uptime_seconds": uptime,
                }));
            } else {
                println!("vPods Cluster Status: ACTIVE");
                println!("   Cluster ID: {}", cluster_id);
                println!("   Nodes: {}", node_count);
                println!("   Uptime: {}s", uptime);
            }
        }

        VpodsClusterCommands::Nodes => {
            let (_cluster_id, _state) = load_cluster_state().ok_or_else(|| anyhow!("No active vPods cluster found"))?;
            let nodes = get_cluster_nodes_from_state().unwrap_or_default();

            if json_output {
                println!("{}", json!({
                    "nodes": nodes,
                    "total_nodes": nodes.len(),
                }));
            } else {
                println!("vPods Cluster Nodes:");
                for (i, node) in nodes.iter().enumerate() {
                    let id = node.get("node_id").and_then(|v| v.as_str()).unwrap_or("<unknown>");
                    let status = node.get("status").and_then(|v| v.as_str()).unwrap_or("<unknown>");
                    let roles = node.get("roles").cloned().unwrap_or(json!([]));
                    let transport = node.get("transport").and_then(|v| v.as_str()).unwrap_or("<unknown>");
                    let endpoint = node.get("endpoint").and_then(|v| v.as_str()).unwrap_or("");
                    println!("  {}. {} (status: {}, roles: {}, transport: {}, endpoint: {})", i + 1, id, status, roles, transport, endpoint);
                }
            }
        }

        VpodsClusterCommands::Scale { replicas } => {
            if dry_run {
                if json_output {
                    println!("{}", json!({
                        "status": "dry_run",
                        "command": "vpods_cluster_scale",
                        "target_replicas": replicas,
                    }));
                } else {
                    println!("[DRY RUN] Would scale vPods cluster to {} replicas", replicas);
                }
                return Ok(());
            }

            set_desired_replicas_for_active_cluster(replicas)?;
            let current_nodes = get_active_node_count_from_state().unwrap_or(0);

            if json_output {
                println!("{}", json!({
                    "status": "scaling",
                    "target_replicas": replicas,
                    "current_nodes": current_nodes,
                }));
            } else {
                println!("vPods Cluster scaling request: target replicas = {}", replicas);
                println!("   Current nodes: {}", current_nodes);
            }
        }

        VpodsClusterCommands::AddNode { node_id, unix_sock, mesh_service } => {
            if dry_run {
                if json_output {
                    println!("{}", json!({
                        "status": "dry_run",
                        "command": "vpods_cluster_add_node",
                        "node_id": node_id,
                        "unix_sock": unix_sock,
                        "mesh_service": mesh_service,
                    }));
                } else {
                    println!("[DRY RUN] Would add vPods node {} (unix_sock: {:?}, mesh_service: {:?})", node_id, unix_sock, mesh_service);
                }
                return Ok(());
            }

            add_node_to_active_cluster(&node_id, &unix_sock, &mesh_service)?;

            if json_output {
                println!("{}", json!({
                    "status": "node_added",
                    "node_id": node_id,
                    "unix_sock": unix_sock,
                    "mesh_service": mesh_service,
                }));
            } else {
                println!("vPods node added: {}", node_id);
                if let Some(sock) = unix_sock {
                    println!("   Unix socket: {}", sock);
                }
                if let Some(svc) = mesh_service {
                    println!("   Mesh service: {}", svc);
                }
            }
        }

        VpodsClusterCommands::RemoveNode { node_id } => {
            if dry_run {
                if json_output {
                    println!("{}", json!({
                        "status": "dry_run",
                        "command": "vpods_cluster_remove_node",
                        "node_id": node_id,
                    }));
                } else {
                    println!("[DRY RUN] Would remove vPods node {}", node_id);
                }
                return Ok(());
            }

            remove_node_from_active_cluster(&node_id)?;

            if json_output {
                println!("{}", json!({
                    "status": "node_removed",
                    "node_id": node_id,
                }));
            } else {
                println!("vPods node removed: {}", node_id);
            }
        }

        VpodsClusterCommands::Metrics => {
            let (cluster_id, state) = load_cluster_state().ok_or_else(|| anyhow!("No active vPods cluster found"))?;
            let nodes = state
                .get("nodes")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            let desired_replicas = state
                .get("desired_replicas")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u32;
            let uptime = get_cluster_uptime_from_state().unwrap_or(0);

            let client: VPodsClient = create_mesh_vpods_client("vpods-cluster-node").await?;
            let capacity = client.get_node_capacity().await?;
            let ring_stats = client.get_ring_stats().await?;

            if json_output {
                println!("{}", json!({
                    "cluster_id": cluster_id,
                    "nodes": nodes,
                    "desired_replicas": desired_replicas,
                    "uptime_seconds": uptime,
                    "node_capacity": capacity,
                    "ring_stats": ring_stats,
                }));
            } else {
                println!("vPods Cluster Metrics:");
                println!("  Cluster ID: {}", cluster_id);
                println!("  Nodes: {}", nodes.len());
                println!("  Desired replicas: {}", desired_replicas);
                println!("  Uptime: {}s", uptime);
                println!("  Node capacity: {}", capacity);
                println!("  Ring stats: {}", ring_stats);
            }
        }
    }

    Ok(())
}
