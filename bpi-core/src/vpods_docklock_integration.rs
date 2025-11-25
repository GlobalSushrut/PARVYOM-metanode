use crate::blockchain_os_kernel::commute_link::{CommuteLink, CommuteConfig};
use crate::blockchain_os_kernel::commute_lock::{MessageType, Priority};
use crate::vpods_unix_transport::VpodsUnixClient;
use crate::vpods_daemon::{VpodSpec, VpodResources, VpodStatus};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::sync::Arc;
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use uuid::Uuid;
use tracing::{info, warn, error, debug};
use std::path::PathBuf;
use std::time::Duration;

use crate::blockchain_os_kernel::factorial_tree_communication::{FactorialTreeCommunication, NodeCapabilities};
use crate::blockchain_os_kernel::tetrabolic_hyperbolic_spaces::{ZkQuantumSync, LokaType};

/// Native vPods client for DockLock integration
pub struct VPodsClient {
    /// CommuteLink for mesh communication
    commute_link: Arc<CommuteLink>,
    /// Unix socket client for local daemon
    unix_client: Option<VpodsUnixClient>,
    /// Service name for vPods daemon
    service_name: String,
}

/// vPods request envelope for JSON protocol
#[derive(Serialize, Deserialize)]
struct VpodsEnvelope {
    version: String,
    id: String,
    #[serde(rename = "type")]
    msg_type: String,
    method: String,
    #[serde(default)]
    payload: Value,
}

/// vPods response envelope for JSON protocol
#[derive(Serialize, Deserialize)]
struct VpodsResponseEnvelope {
    version: String,
    id: String,
    #[serde(rename = "type")]
    msg_type: String,
    ok: bool,
    #[serde(default)]
    error: Option<Value>,
    #[serde(default)]
    payload: Option<Value>,
}

impl VPodsClient {
    /// Create new vPods client with CommuteLink
    pub fn new(commute_link: Arc<CommuteLink>) -> Self {
        Self {
            commute_link,
            unix_client: None,
            service_name: "vpods-daemon".to_string(),
        }
    }

    /// Initialize connection to vPods daemon
    pub async fn initialize(&mut self) -> Result<()> {
        // Try Unix socket first (for local daemon)
        let socket_paths = [
            "/era/mutable/var/run/vpods-daemon.sock",
            "/tmp/vpods-daemon.sock",
            "/run/pravyom/vpods.sock",
        ];

        for socket_path in &socket_paths {
            if std::path::Path::new(socket_path).exists() {
                match self.commute_link.connect_to_vpods_daemon(socket_path.to_string()).await {
                    Ok(client) => {
                        self.unix_client = Some(client);
                        info!("Connected to vPods daemon via Unix socket: {}", socket_path);
                        return Ok(());
                    }
                    Err(e) => {
                        warn!("Failed to connect to vPods daemon at {}: {}", socket_path, e);
                    }
                }
            }
        }

        // Fallback to mesh discovery via CommuteLink
        info!("Attempting to discover vPods daemon via CommuteLink mesh: service={}", self.service_name);

        // Try to establish a mesh connection to validate availability
        let connection_id = self
            .commute_link
            .connect_to_service(&self.service_name)
            .await?;

        // Immediately close again; this is just a reachability probe
        if let Err(e) = self.commute_link.close_connection(connection_id).await {
            warn!("Failed to close mesh probe connection {}: {}", connection_id, e);
        }

        info!("vPods daemon reachable via CommuteLink mesh; proceeding without Unix socket");
        Ok(())
    }

    /// Send vPods request and receive response
    async fn send_vpods_request(&self, method: &str, payload: Value) -> Result<Value> {
        let req_id = Uuid::new_v4().to_string();

        let envelope = VpodsEnvelope {
            version: "0.1".to_string(),
            id: req_id.clone(),
            msg_type: "request".to_string(),
            method: method.to_string(),
            payload,
        };

        let request_bytes = serde_json::to_vec(&envelope)?;

        // Send via Unix socket if available
        if let Some(unix_client) = &self.unix_client {
            let response_bytes = unix_client.send_message(&request_bytes).await?;
            
            if response_bytes.is_empty() {
                return Err(anyhow!("Empty response from vPods daemon"));
            }

            let response: VpodsResponseEnvelope = serde_json::from_slice(&response_bytes)?;

            if !response.ok {
                let error = response.error.unwrap_or_else(|| json!({"code": "INTERNAL_ERROR", "message": "Unknown"}));
                let code = error["code"].as_str().unwrap_or("INTERNAL_ERROR");
                let msg = error["message"].as_str().unwrap_or("vPods daemon error");
                return Err(anyhow!("{}: {}", code, msg));
            }

            return Ok(response.payload.unwrap_or(Value::Null));
        }

        // Fallback to CommuteLink mesh communication
        info!("No Unix socket client for vPods; using CommuteLink mesh for service {}", self.service_name);

        // Connect to vPods daemon service via mesh
        let connection_id = self
            .commute_link
            .connect_to_service(&self.service_name)
            .await?;

        // Send control message over CommuteLink
        self.commute_link
            .send_message(
                connection_id,
                &request_bytes,
                MessageType::Control,
                Priority::Critical,
            )
            .await?;

        // Receive response
        let response_bytes_opt = self
            .commute_link
            .receive_message(connection_id)
            .await?;

        // Close connection (best-effort)
        if let Err(e) = self.commute_link.close_connection(connection_id).await {
            warn!("Failed to close CommuteLink connection {}: {}", connection_id, e);
        }

        let response_bytes = response_bytes_opt
            .ok_or_else(|| anyhow!("No response from vPods daemon via CommuteLink"))?;

        let response: VpodsResponseEnvelope = serde_json::from_slice(&response_bytes)?;

        if !response.ok {
            let error = response.error.unwrap_or_else(|| json!({"code": "INTERNAL_ERROR", "message": "Unknown"}));
            let code = error["code"].as_str().unwrap_or("INTERNAL_ERROR");
            let msg = error["message"].as_str().unwrap_or("vPods daemon error" );
            return Err(anyhow!("{}: {}", code, msg));
        }

        Ok(response.payload.unwrap_or(Value::Null))
    }

    /// Create vPod from spec
    pub async fn create_vpod(&self, spec: &VpodSpec) -> Result<String> {
        debug!("Creating vPod: name={}, cmd={:?}", spec.name, spec.cmd);

        // Map VpodSpec to vpod.create payload
        let env_map: serde_json::Map<String, Value> = spec
            .env
            .iter()
            .map(|(k, v)| (k.clone(), json!(v)))
            .collect();

        let payload = json!({
            "name": spec.name,
            "cmd": spec.cmd,
            "env": env_map,
            "cwd": spec.cwd.as_ref().map(|p| p.to_string_lossy().to_string()),
            "resources": {
                "cpu_percent": spec.resources.cpu_percent,
                "mem_mb": spec.resources.mem_mb,
            }
        });

        let response = self.send_vpods_request("vpod.create", payload).await?;
        let vpod_id = response["vpod_id"]
            .as_str()
            .ok_or_else(|| anyhow!("vpod.create: missing vpod_id in response"))?
            .to_string();

        info!("Created vPod: id={}, name={}", vpod_id, spec.name);
        Ok(vpod_id)
    }

    /// Stop vPod
    pub async fn stop_vpod(&self, vpod_id: &str) -> Result<()> {
        debug!("Stopping vPod: id={}", vpod_id);

        let payload = json!({
            "vpod_id": vpod_id,
            "force": false
        });

        let _ = self.send_vpods_request("vpod.stop", payload).await?;
        info!("Stopped vPod: id={}", vpod_id);
        Ok(())
    }

    /// Execute command in vPod
    pub async fn exec_in_vpod(&self, vpod_id: &str, command: &[String]) -> Result<Value> {
        debug!("Executing in vPod: id={}, cmd={:?}", vpod_id, command);

        let payload = json!({
            "vpod_id": vpod_id,
            "cmd": command,
        });

        let response = self.send_vpods_request("vpod.exec", payload).await?;
        Ok(response)
    }

    /// Get vPod status
    pub async fn get_vpod_status(&self, vpod_id: &str) -> Result<VpodStatus> {
        debug!("Getting vPod status: id={}", vpod_id);

        let payload = json!({ "vpod_id": vpod_id });
        let response = self.send_vpods_request("vpod.inspect", payload).await?;
        
        let vpod_info = response["vpod"].as_object()
            .ok_or_else(|| anyhow!("Invalid vpod.inspect response"))?;

        let status_str = vpod_info["status"].as_str()
            .ok_or_else(|| anyhow!("Missing status in vpod.inspect response"))?;

        let status = match status_str {
            "Running" => VpodStatus::Running,
            "Stopped" => VpodStatus::Stopped,
            "Pending" => VpodStatus::Pending,
            "Failed" => VpodStatus::Failed("Failed".to_string()),
            other => VpodStatus::Failed(other.to_string()),
        };

        debug!("vPod status: id={}, status={:?}", vpod_id, status);
        Ok(status)
    }

    /// List all vPods
    pub async fn list_vpods(&self) -> Result<Vec<Value>> {
        debug!("Listing vPods");

        let payload = json!({});
        let response = self.send_vpods_request("vpod.list", payload).await?;
        
        let vpods = response["vpods"].as_array()
            .ok_or_else(|| anyhow!("Invalid vpod.list response"))?
            .clone();

        debug!("Listed {} vPods", vpods.len());
        Ok(vpods)
    }

    /// Get node capacity information
    pub async fn get_node_capacity(&self) -> Result<Value> {
        debug!("Getting node capacity");

        let payload = json!({});
        let response = self.send_vpods_request("node.capacity", payload).await?;
        
        debug!("Retrieved node capacity information");
        Ok(response)
    }

    /// Get Fibonacci ring statistics
    pub async fn get_ring_stats(&self) -> Result<Value> {
        debug!("Getting ring statistics");

        let payload = json!({});
        let response = self.send_vpods_request("scheduler.rings", payload).await?;
        
        debug!("Retrieved ring statistics");
        Ok(response)
    }
}

/// Create a mesh-capable VPods client for core components (DockLock, vPods orchestrator, etc.).
/// This uses the same CommuteLink + tetrabolic mesh configuration everywhere so that all
/// callers talk to the same vPods daemon and mesh fabric.
pub async fn create_mesh_vpods_client(node_id_prefix: &str) -> Result<VPodsClient> {
    let zk_sync = Arc::new(ZkQuantumSync::new()?);
    let factorial_comm = Arc::new(FactorialTreeCommunication::new()?);

    let node_capabilities = NodeCapabilities {
        cpu_cores: num_cpus::get() as u32,
        memory_gb: 8,
        storage_gb: 100,
        bandwidth_mbps: 100,
        protocols: vec![
            "vpods-control".to_string(),
            "vpods-workload".to_string(),
            "docklock".to_string(),
        ],
    };

    let node_config = CommuteConfig {
        node_id: format!("{}-{}", node_id_prefix, Uuid::new_v4().simple()),
        capabilities: node_capabilities,
        supported_lokas: vec![LokaType::Bhuloka],
        max_connections: 128,
        connection_timeout: Duration::from_secs(30),
        heartbeat_interval: Duration::from_secs(30),
        discovery_interval: Duration::from_secs(10),
    };

    let commute_link = Arc::new(CommuteLink::new(zk_sync, factorial_comm, node_config).await?);
    let mut client = VPodsClient::new(commute_link);
    client.initialize().await?;
    Ok(client)
}

/// DockLock integration functions using vPods
pub mod docklock_vpods {
    use super::*;
    use std::path::Path;

    /// Create VpodSpec from DockLock container configuration
    pub fn create_vpod_spec_from_docklock(
        container_id: &str,
        image: &str,
        command: Option<Vec<String>>,
        env: Option<HashMap<String, String>>,
        working_dir: Option<String>,
    ) -> VpodSpec {
        VpodSpec {
            name: format!("docklock-{}", container_id),
            cmd: command.unwrap_or_else(|| vec!["sh".to_string(), "-c".to_string(), "sleep 3600".to_string()]),
            env: env.unwrap_or_default(),
            cwd: working_dir.map(PathBuf::from),
            resources: VpodResources {
                cpu_percent: 10, // Default 10% CPU
                mem_mb: 512,       // Default 512MB RAM
            },
            security_profile: None,
        }
    }

    /// Deploy secure container using vPods
    pub async fn deploy_secure_container_vpods(
        vpods_client: &VPodsClient,
        container_id: &str,
        image: &str,
        command: Option<Vec<String>>,
        env: Option<HashMap<String, String>>,
        working_dir: Option<String>,
    ) -> Result<String> {
        info!("Deploying secure container via vPods: id={}, image={}", container_id, image);

        // Create vPod spec
        let spec = create_vpod_spec_from_docklock(container_id, image, command, env, working_dir);

        // Create vPod
        let vpod_id = vpods_client.create_vpod(&spec).await?;

        // Store deployment record
        let deployment_record = json!({
            "container_id": container_id,
            "vpod_id": vpod_id,
            "image": image,
            "spec": spec,
            "deployed_at": chrono::Utc::now().to_rfc3339(),
            "deployment_method": "vpods"
        });

        let container_dir = format!("/era/mutable/var/bpi/docklock/containers/{}", container_id);
        std::fs::create_dir_all(&container_dir)?;
        
        let deployment_file = format!("{}/deployment_record.json", container_dir);
        std::fs::write(&deployment_file, serde_json::to_string_pretty(&deployment_record)?)?;

        info!("Deployed secure container: container_id={}, vpod_id={}", container_id, vpod_id);
        Ok(vpod_id)
    }

    /// Start container using vPods
    pub async fn start_container_vpods(
        vpods_client: &VPodsClient,
        container_id: &str,
    ) -> Result<()> {
        info!("Starting container via vPods: id={}", container_id);

        // Load deployment record to get vpod_id
        let container_dir = format!("/era/mutable/var/bpi/docklock/containers/{}", container_id);
        let deployment_file = format!("{}/deployment_record.json", container_dir);
        
        if !Path::new(&deployment_file).exists() {
            return Err(anyhow!("Container {} not deployed via vPods", container_id));
        }

        let deployment_data = std::fs::read_to_string(&deployment_file)?;
        let deployment_record: Value = serde_json::from_str(&deployment_data)?;
        
        let vpod_id = deployment_record["vpod_id"].as_str()
            .ok_or_else(|| anyhow!("Missing vpod_id in deployment record"))?;

        // Check if vPod is already running
        let status = vpods_client.get_vpod_status(vpod_id).await?;
        match status {
            VpodStatus::Running => {
                info!("Container already running: container_id={}, vpod_id={}", container_id, vpod_id);
                return Ok(());
            }
            _ => {
                // vPod creation automatically starts it, so if it's not running,
                // we may need to recreate it or handle the error
                warn!("vPod not running, may need to recreate: vpod_id={}", vpod_id);
            }
        }

        // Store runtime status
        let runtime_status = json!({
            "container_id": container_id,
            "vpod_id": vpod_id,
            "status": "running",
            "started_at": chrono::Utc::now().to_rfc3339(),
            "runtime_method": "vpods"
        });

        let runtime_file = format!("{}/runtime_status.json", container_dir);
        std::fs::write(&runtime_file, serde_json::to_string_pretty(&runtime_status)?)?;

        info!("Started container: container_id={}, vpod_id={}", container_id, vpod_id);
        Ok(())
    }

    /// Stop container using vPods
    pub async fn stop_container_vpods(
        vpods_client: &VPodsClient,
        container_id: &str,
    ) -> Result<()> {
        info!("Stopping container via vPods: id={}", container_id);

        // Load deployment record to get vpod_id
        let container_dir = format!("/era/mutable/var/bpi/docklock/containers/{}", container_id);
        let deployment_file = format!("{}/deployment_record.json", container_dir);
        
        if !Path::new(&deployment_file).exists() {
            return Err(anyhow!("Container {} not deployed via vPods", container_id));
        }

        let deployment_data = std::fs::read_to_string(&deployment_file)?;
        let deployment_record: Value = serde_json::from_str(&deployment_data)?;
        
        let vpod_id = deployment_record["vpod_id"].as_str()
            .ok_or_else(|| anyhow!("Missing vpod_id in deployment record"))?;

        // Stop vPod
        vpods_client.stop_vpod(vpod_id).await?;

        // Update runtime status
        let runtime_status = json!({
            "container_id": container_id,
            "vpod_id": vpod_id,
            "status": "stopped",
            "stopped_at": chrono::Utc::now().to_rfc3339(),
            "runtime_method": "vpods"
        });

        let runtime_file = format!("{}/runtime_status.json", container_dir);
        std::fs::write(&runtime_file, serde_json::to_string_pretty(&runtime_status)?)?;

        info!("Stopped container: container_id={}, vpod_id={}", container_id, vpod_id);
        Ok(())
    }

    /// Execute command in container using vPods
    pub async fn execute_in_container_vpods(
        vpods_client: &VPodsClient,
        container_id: &str,
        command: &[String],
    ) -> Result<Value> {
        info!("Executing in container via vPods: id={}, cmd={:?}", container_id, command);

        // Load deployment record to get vpod_id
        let container_dir = format!("/era/mutable/var/bpi/docklock/containers/{}", container_id);
        let deployment_file = format!("{}/deployment_record.json", container_dir);
        
        if !Path::new(&deployment_file).exists() {
            return Err(anyhow!("Container {} not deployed via vPods", container_id));
        }

        let deployment_data = std::fs::read_to_string(&deployment_file)?;
        let deployment_record: Value = serde_json::from_str(&deployment_data)?;
        
        let vpod_id = deployment_record["vpod_id"].as_str()
            .ok_or_else(|| anyhow!("Missing vpod_id in deployment record"))?;

        // Execute command in vPod
        let result = vpods_client.exec_in_vpod(vpod_id, command).await?;

        // Store execution record
        let execution_record = json!({
            "container_id": container_id,
            "vpod_id": vpod_id,
            "command": command,
            "result": result,
            "executed_at": chrono::Utc::now().to_rfc3339(),
            "execution_method": "vpods"
        });

        let execution_file = format!("{}/execution_record.json", container_dir);
        std::fs::write(&execution_file, serde_json::to_string_pretty(&execution_record)?)?;

        info!("Executed command in container: container_id={}, vpod_id={}", container_id, vpod_id);
        Ok(result)
    }
}
