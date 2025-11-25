use crate::blockchain_os_kernel::commute_link::MessageHandler;
use crate::blockchain_os_kernel::commute_lock::{
    MessageType, Priority, ZeroCopyMessage, CommuteLock,
    LockType, DistributedLock
};
use crate::vpods_daemon::{VpodsDaemon, VpodSpec, VPodResourceLimits};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use std::path::PathBuf;
use anyhow::{Result, anyhow};
use uuid::Uuid;
use std::time::Duration;
use tracing::{info, warn, error, debug};

/// vPods control protocol envelope for requests
#[derive(Debug, Deserialize)]
pub struct VpodsRequestEnvelope {
    pub version: String,
    pub id: String,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub method: String,
    #[serde(default)]
    pub auth: Option<AuthEnvelope>,
    #[serde(default)]
    pub payload: Value,
}

/// vPods control protocol envelope for responses
#[derive(Debug, Serialize)]
pub struct VpodsResponseEnvelope {
    pub version: String,
    pub id: String,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<VpodsError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
}

/// Authentication envelope for future ENC integration
#[derive(Debug, Deserialize)]
pub struct AuthEnvelope {
    pub scheme: String,
    pub token: String,
}

/// Error structure for vPods protocol
#[derive(Debug, Serialize)]
pub struct VpodsError {
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub details: Value,
}

/// vPods Control Handler - processes vPods protocol messages via CommuteLink
pub struct VpodsControlHandler {
    /// Reference to the vPods daemon
    vpods_daemon: Arc<VpodsDaemon>,
    /// CommuteLock for distributed locking
    commute_lock: Arc<CommuteLock>,
    /// Node ID for lock scoping
    node_id: String,
}

impl VpodsControlHandler {
    /// Create new vPods control handler
    pub fn new(
        vpods_daemon: Arc<VpodsDaemon>,
        commute_lock: Arc<CommuteLock>,
        node_id: String,
    ) -> Self {
        Self {
            vpods_daemon,
            commute_lock,
            node_id,
        }
    }

    /// Process vPods control request with distributed locking
    async fn process_vpods_request(&self, envelope: VpodsRequestEnvelope) -> Result<VpodsResponseEnvelope> {
        debug!("Processing vPods request: method={}, id={}", envelope.method, envelope.id);

        let response_payload = match envelope.method.as_str() {
            "node.hello" => self.handle_node_hello().await?,
            "vpod.create" => self.handle_vpod_create(envelope.payload).await?,
            "vpod.stop" => self.handle_vpod_stop(envelope.payload).await?,
            "vpod.list" => self.handle_vpod_list(envelope.payload).await?,
            "vpod.inspect" => self.handle_vpod_inspect(envelope.payload).await?,
            "vpod.exec" => self.handle_vpod_exec(envelope.payload).await?,
            "node.capacity" => self.handle_node_capacity().await?,
            "scheduler.rings" => self.handle_scheduler_rings().await?,
            "epoch.tail" => self.handle_epoch_tail(envelope.payload).await?,
            _ => {
                return Ok(VpodsResponseEnvelope {
                    version: envelope.version,
                    id: envelope.id,
                    msg_type: "response".to_string(),
                    ok: false,
                    error: Some(VpodsError {
                        code: "UNSUPPORTED_METHOD".to_string(),
                        message: format!("Method '{}' not supported", envelope.method),
                        details: json!({"method": envelope.method}),
                    }),
                    payload: None,
                });
            }
        };

        Ok(VpodsResponseEnvelope {
            version: envelope.version,
            id: envelope.id,
            msg_type: "response".to_string(),
            ok: true,
            error: None,
            payload: Some(response_payload),
        })
    }

    /// Handle node.hello - discover node identity and capabilities
    async fn handle_node_hello(&self) -> Result<Value> {
        let cores = detect_cpu_cores();
        let ram_mb = detect_total_memory_mb();
        
        Ok(json!({
            "node_id": self.node_id,
            "hostname": detect_hostname(),
            "os": "linux",
            "kernel": "6.8.5", // Could be dynamic via uname
            "cores": cores,
            "ram_mb": ram_mb,
            "vpods_daemon_version": "0.1.0",
            "features": {
                "ebpf": true,
                "docklock": true,
                "enc_cluster": false,
                "sixd_core": true,
                "bsok8_agent": true
            }
        }))
    }

    /// Handle vpod.create - create new vPod with distributed locking
    async fn handle_vpod_create(&self, payload: Value) -> Result<Value> {
        // Acquire distributed lock for vPod creation
        let lock_id = format!("vpods/{}", self.node_id);
        let _lock = self.commute_lock.acquire_distributed_lock(
            lock_id,
            LockType::Write,
            Duration::from_secs(5),
        ).await.map_err(|e| anyhow!("Failed to acquire vPods lock: {}", e))?;

        // Parse vpod.create payload
        let name = payload["name"].as_str()
            .ok_or_else(|| anyhow!("Missing 'name' in vpod.create payload"))?;
        
        let cmd: Vec<String> = payload["cmd"].as_array()
            .ok_or_else(|| anyhow!("Missing 'cmd' in vpod.create payload"))?
            .iter()
            .map(|v| v.as_str().unwrap_or("").to_string())
            .collect();

        let env = payload["env"].as_object()
            .map(|obj| {
                obj.iter()
                    .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let cwd = payload["cwd"].as_str().map(|s| PathBuf::from(s));

        let cpu_percent = payload["resources"]["cpu_percent"].as_u64().unwrap_or(10) as u8;
        let mem_mb = payload["resources"]["mem_mb"].as_u64().unwrap_or(512);

        let resources = VPodResourceLimits {
            cpu_percent,
            mem_mb,
        };

        let spec = VpodSpec {
            name: name.to_string(),
            cmd,
            env,
            cwd,
            resources,
            security_profile: None,
        };

        // Create vPod via daemon (returns vpod_id)
        match self.vpods_daemon.create_vpod(spec).await {
            Ok(vpod_id) => {
                info!("Created vPod: id={}", vpod_id);
                Ok(json!({
                    "vpod_id": vpod_id,
                    "node_id": self.node_id,
                    "status": "Running",
                }))
            }
            Err(e) => {
                let error_code = if e.to_string().contains("capacity") {
                    "VPOD_LIMIT_REACHED"
                } else if e.to_string().contains("ring") {
                    "RING_SATURATED"
                } else {
                    "INTERNAL_ERROR"
                };

                Err(anyhow!("vPod creation failed: {} ({})", e, error_code))
            }
        }
    }

    /// Handle vpod.stop - stop vPod
    async fn handle_vpod_stop(&self, payload: Value) -> Result<Value> {
        let vpod_id = payload["vpod_id"].as_str()
            .ok_or_else(|| anyhow!("Missing 'vpod_id' in vpod.stop payload"))?;
        
        // The daemon already performs its own locking
        self.vpods_daemon.stop_vpod(vpod_id).await?;
        
        info!("Stopped vPod: id={}", vpod_id);
        Ok(json!({
            "vpod_id": vpod_id,
            "status": "Stopped"
        }))
    }

    /// Handle vpod.list - list vPods
    async fn handle_vpod_list(&self, payload: Value) -> Result<Value> {
        // Currently we ignore any status filter and delegate to the daemon's JSON API
        let _ = payload; // reserved for future filter support
        let vpods_json = self.vpods_daemon.list_vpods().await?;
        Ok(vpods_json)
    }

    /// Handle vpod.inspect - detailed vPod info
    async fn handle_vpod_inspect(&self, payload: Value) -> Result<Value> {
        let vpod_id = payload["vpod_id"].as_str()
            .ok_or_else(|| anyhow!("Missing 'vpod_id' in vpod.inspect payload"))?;

        // Delegate to daemon's JSON API
        let vpod_info = self.vpods_daemon.inspect_vpod(vpod_id).await?;
        Ok(json!({ "vpod": vpod_info }))
    }

    /// Handle vpod.exec - execute command inside vPod
    async fn handle_vpod_exec(&self, payload: Value) -> Result<Value> {
        let vpod_id = payload["vpod_id"].as_str()
            .ok_or_else(|| anyhow!("Missing 'vpod_id' in vpod.exec payload"))?;

        let cmd_array = payload["cmd"].as_array()
            .ok_or_else(|| anyhow!("Missing 'cmd' in vpod.exec payload"))?;

        let command: Vec<String> = cmd_array
            .iter()
            .map(|v| v.as_str().unwrap_or("").to_string())
            .collect();

        let result = self.vpods_daemon.exec_in_vpod(vpod_id, &command).await?;
        Ok(result)
    }

    /// Handle node.capacity - get capacity and tank state
    async fn handle_node_capacity(&self) -> Result<Value> {
        // Use daemon's JSON capacity API directly
        let capacity = self.vpods_daemon.get_node_capacity().await?;
        Ok(capacity)
    }

    /// Handle scheduler.rings - Fibonacci stability graph state
    async fn handle_scheduler_rings(&self) -> Result<Value> {
        let rings = self.vpods_daemon.get_scheduler_rings().await?;
        Ok(rings)
    }

    /// Handle epoch.tail - read Epoch Chain blocks
    async fn handle_epoch_tail(&self, payload: Value) -> Result<Value> {
        let _limit = payload["limit"].as_u64().unwrap_or(10) as usize;
        // Current daemon keeps its own fixed-size tail; limit is reserved for future use
        let blocks = self.vpods_daemon.get_epoch_tail().await?;
        Ok(blocks)
    }
}

/// Detect CPU core count from /proc/cpuinfo
fn detect_cpu_cores() -> u64 {
    if let Ok(contents) = std::fs::read_to_string("/proc/cpuinfo") {
        let count = contents
            .lines()
            .filter(|l| l.starts_with("processor"))
            .count();
        if count > 0 {
            return count as u64;
        }
    }
    1
}

/// Detect total RAM in MB from /proc/meminfo
fn detect_total_memory_mb() -> u64 {
    if let Ok(contents) = std::fs::read_to_string("/proc/meminfo") {
        for line in contents.lines() {
            if line.starts_with("MemTotal:") {
                let parts: Vec<_> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(kb) = parts[1].parse::<u64>() {
                        return kb / 1024;
                    }
                }
            }
        }
    }
    0
}

/// Detect hostname using environment or fallback
fn detect_hostname() -> String {
    std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string())
}

impl MessageHandler for VpodsControlHandler {
    fn can_handle(&self, message_type: &MessageType) -> bool {
        matches!(message_type, MessageType::Control)
    }

    fn priority(&self) -> Priority {
        Priority::Critical // vPods control is system-level
    }

    fn handle_message(&self, message: &ZeroCopyMessage) -> Result<Option<Vec<u8>>> {
        // Bridge sync trait method to async implementation using Tokio runtime
        let handle = tokio::runtime::Handle::try_current()
            .map_err(|e| anyhow!("Tokio runtime not available for vPods control handler: {}", e))?;
        handle.block_on(self.handle_message_async(message))
    }
}

impl VpodsControlHandler {
    /// Async implementation of message handling, invoked from sync trait method
    async fn handle_message_async(&self, message: &ZeroCopyMessage) -> Result<Option<Vec<u8>>> {
        // Extract message content from zero-copy memory block
        let data: &[u8] = unsafe {
            std::slice::from_raw_parts(
                message.memory_block.ptr.as_ptr(),
                message.metadata.content_length,
            )
        };

        // Parse as UTF-8 JSON
        let json_str = std::str::from_utf8(data)
            .map_err(|e| anyhow!("Invalid UTF-8 in vPods message: {}", e))?;

        // Deserialize request envelope
        let request: VpodsRequestEnvelope = serde_json::from_str(json_str)
            .map_err(|e| anyhow!("Invalid vPods request JSON: {}", e))?;

        debug!("Received vPods request: method={}, id={}", request.method, request.id);

        // Process request and generate response
        let response = match self.process_vpods_request(request).await {
            Ok(resp) => resp,
            Err(e) => {
                error!("vPods request processing failed: {}", e);
                VpodsResponseEnvelope {
                    version: "0.1".to_string(),
                    id: Uuid::new_v4().to_string(),
                    msg_type: "response".to_string(),
                    ok: false,
                    error: Some(VpodsError {
                        code: "INTERNAL_ERROR".to_string(),
                        message: e.to_string(),
                        details: json!({}),
                    }),
                    payload: None,
                }
            }
        };

        // Serialize response to bytes
        let response_bytes = serde_json::to_vec(&response)
            .map_err(|e| anyhow!("Failed to serialize vPods response: {}", e))?;

        Ok(Some(response_bytes))
    }
}
