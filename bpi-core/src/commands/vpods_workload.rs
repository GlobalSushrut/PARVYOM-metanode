use anyhow::{Result, anyhow};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use uuid::Uuid;

use crate::vpods_daemon::{VpodSpec, VpodResources, VpodStatus};
use crate::vpods_docklock_integration::{VPodsClient, create_mesh_vpods_client};

/// vPods workload-level commands (Phase 1: create vPods and persist workload state)
#[derive(Debug, Clone)]
pub enum VpodsWorkloadCommands {
    /// Deploy a vPods workload by creating a vPod from a shell command
    Deploy {
        name: String,
        command: String,
    },
    /// List vPods workloads tracked by this control-plane
    List,
    /// Show status of a specific vPods workload
    Status {
        workload_id: String,
    },
}

fn vpods_cluster_root() -> String {
    let era_root = "/era/mutable/var/bpi/vpods_cluster";
    if Path::new(era_root).exists() {
        era_root.to_string()
    } else {
        "/tmp/bpi_vpods_cluster".to_string()
    }
}

fn workloads_root() -> String {
    format!("{}/workloads", vpods_cluster_root())
}

fn workload_state_path(workload_id: &str) -> String {
    format!("{}/{}.json", workloads_root(), workload_id)
}

fn current_cluster_id() -> Option<String> {
    let path = format!("{}/current_cluster.json", vpods_cluster_root());
    let content = fs::read_to_string(&path).ok()?;
    let value: serde_json::Value = serde_json::from_str(&content).ok()?;
    value.get("cluster_id").and_then(|v| v.as_str()).map(|s| s.to_string())
}

fn load_cluster_state() -> Option<serde_json::Value> {
    let cluster_id = current_cluster_id()?;
    let path = format!("{}/clusters/{}/cluster_state.json", vpods_cluster_root(), cluster_id);
    let content = fs::read_to_string(&path).ok()?;
    serde_json::from_str(&content).ok()
}

fn select_node_for_workload() -> Option<serde_json::Value> {
    let state = load_cluster_state()?;
    let nodes = state.get("nodes")?.as_array()?.clone();

    if nodes.is_empty() {
        return None;
    }

    // Load existing workloads and count how many are bound to each node_id
    let workloads = load_all_workloads();
    let mut counts: HashMap<String, usize> = HashMap::new();

    for w in workloads {
        if let Some(node_id) = w
            .get("node")
            .and_then(|n| n.get("node_id"))
            .and_then(|v| v.as_str())
        {
            *counts.entry(node_id.to_string()).or_insert(0) += 1;
        }
    }

    // Pick the node with the fewest workloads bound to it (simple least-loaded policy)
    let mut best_node: Option<serde_json::Value> = None;
    let mut best_count: usize = usize::MAX;

    for node in nodes {
        let node_id = match node.get("node_id").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => continue,
        };

        let count = *counts.get(node_id).unwrap_or(&0);

        if count < best_count {
            best_count = count;
            best_node = Some(node.clone());
        }
    }

    best_node
}

fn ensure_cluster_exists() -> Result<String> {
    if let Some(cluster_id) = current_cluster_id() {
        Ok(cluster_id)
    } else {
        Err(anyhow!("No active vPods cluster found; run 'bpi-core vpods-cluster deploy' first"))
    }
}

fn persist_workload_state(workload_id: &str, state: &serde_json::Value) -> Result<()> {
    let root = workloads_root();
    fs::create_dir_all(&root)?;
    let path = workload_state_path(workload_id);
    fs::write(path, serde_json::to_string_pretty(state)?)?;
    Ok(())
}

fn load_all_workloads() -> Vec<serde_json::Value> {
    let root = workloads_root();
    if !Path::new(&root).exists() {
        return Vec::new();
    }

    let mut workloads = Vec::new();
    if let Ok(entries) = fs::read_dir(&root) {
        for entry in entries.flatten() {
            if entry.path().extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            if let Ok(content) = fs::read_to_string(entry.path()) {
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(&content) {
                    workloads.push(value);
                }
            }
        }
    }
    workloads
}

fn load_workload(workload_id: &str) -> Option<serde_json::Value> {
    let path = workload_state_path(workload_id);
    let content = fs::read_to_string(&path).ok()?;
    serde_json::from_str(&content).ok()
}

pub async fn handle(cmd: VpodsWorkloadCommands, json_output: bool, dry_run: bool) -> Result<()> {
    match cmd {
        VpodsWorkloadCommands::Deploy { name, command } => {
            let cluster_id = ensure_cluster_exists()?;

            // Select a node for this workload from the vPods cluster state
            let node = select_node_for_workload()
                .ok_or_else(|| anyhow!("No nodes registered in vPods cluster; add at least one node with 'vpods-cluster add-node'"))?;

            if dry_run {
                if json_output {
                    println!("{}", json!({
                        "status": "dry_run",
                        "command": "vpods_workload_deploy",
                        "cluster_id": cluster_id,
                        "name": name,
                        "shell": command,
                        "node": node,
                    }));
                } else {
                    let node_id = node["node_id"].as_str().unwrap_or("<unknown-node>");
                    println!("[DRY RUN] Would deploy vPods workload '{}' in cluster {} on node {} with shell: {}", name, cluster_id, node_id, command);
                }
                return Ok(());
            }

            let client: VPodsClient = create_mesh_vpods_client("vpods-workload-node").await?;

            // Consult real node capacity from the vPods daemon (TankCapacityManager)
            // before admitting this workload. This gates placement on the same
            // tank_level / max_vpods / active_vpods metrics exposed via node.capacity.
            let capacity = client.get_node_capacity().await?;
            let tank_level = capacity
                .get("tank_level")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let max_vpods = capacity
                .get("max_vpods")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            let active_vpods = capacity
                .get("active_vpods")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);

            // Simple admission control: reject when tank is too low or we've
            // reached the advertised max_vpods for this node.
            if tank_level < 0.15 || active_vpods >= max_vpods {
                return Err(anyhow!(
                    "vPods node at capacity: tank_level={:.3}, active_vpods={}, max_vpods={}",
                    tank_level,
                    active_vpods,
                    max_vpods,
                ));
            }

            let cmd_vec = vec![
                "/bin/sh".to_string(),
                "-c".to_string(),
                command.clone(),
            ];

            let spec = VpodSpec {
                name: name.clone(),
                cmd: cmd_vec.clone(),
                env: HashMap::new(),
                cwd: None,
                resources: VpodResources {
                    cpu_percent: 10,
                    mem_mb: 512,
                },
                security_profile: None,
            };

            let vpod_id = client.create_vpod(&spec).await?;
            let workload_id = format!("vpods-workload-{}", Uuid::new_v4().simple());

            let state = json!({
                "workload_id": workload_id,
                "name": name,
                "cluster_id": cluster_id,
                "vpod_id": vpod_id,
                "spec": spec,
                "shell": command,
                 "node": node,
                "created_at": chrono::Utc::now().to_rfc3339(),
            });

            persist_workload_state(&state["workload_id"].as_str().unwrap().to_string(), &state)?;

            if json_output {
                println!("{}", state);
            } else {
                println!("vPods workload deployed:");
                println!("  workload_id: {}", state["workload_id"].as_str().unwrap_or("<unknown>"));
                println!("  vpod_id: {}", state["vpod_id"].as_str().unwrap_or("<unknown>"));
                if let Some(node_id) = state["node"]["node_id"].as_str() {
                    println!("  node_id: {}", node_id);
                }
            }
        }

        VpodsWorkloadCommands::List => {
            let workloads = load_all_workloads();

            if json_output {
                println!("{}", json!({
                    "workloads": workloads,
                    "total": workloads.len(),
                }));
            } else {
                if workloads.is_empty() {
                    println!("No vPods workloads found.");
                } else {
                    println!("vPods workloads:");
                    for w in &workloads {
                        let id = w["workload_id"].as_str().unwrap_or("<unknown>");
                        let name = w["name"].as_str().unwrap_or("<unnamed>");
                        let vpod_id = w["vpod_id"].as_str().unwrap_or("<unknown>");
                        println!("  {}: {} (vpod_id: {})", id, name, vpod_id);
                    }
                }
            }
        }

        VpodsWorkloadCommands::Status { workload_id } => {
            let workload = load_workload(&workload_id)
                .ok_or_else(|| anyhow!("Workload {} not found", workload_id))?;

            let vpod_id = workload
                .get("vpod_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Workload {} is missing vpod_id", workload_id))?;

            let client: VPodsClient = create_mesh_vpods_client("vpods-workload-status").await?;
            let status = client.get_vpod_status(vpod_id).await?;

            let status_str = match status {
                VpodStatus::Running => "Running",
                VpodStatus::Stopped => "Stopped",
                VpodStatus::Pending => "Pending",
                VpodStatus::Failed(_) => "Failed",
            };

            if json_output {
                println!("{}", json!({
                    "workload": workload,
                    "vpod_status": status_str,
                }));
            } else {
                println!("vPods workload status:");
                let name = workload.get("name").and_then(|v| v.as_str()).unwrap_or("<unnamed>");
                println!("  workload_id: {}", workload_id);
                println!("  name: {}", name);
                println!("  vpod_id: {}", vpod_id);
                if let Some(node_id) = workload.get("node").and_then(|n| n.get("node_id")).and_then(|v| v.as_str()) {
                    println!("  node_id: {}", node_id);
                }
                println!("  status: {}", status_str);
            }
        }
    }

    Ok(())
}
