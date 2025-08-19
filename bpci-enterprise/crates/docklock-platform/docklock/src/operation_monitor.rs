//! # Container Operation Monitor
//!
//! Monitors all container operations and generates StepReceipts for the blockchain pipeline.
//! This is the core integration point that creates receipts for every syscall and operation.

use crate::error::{DockLockError, DockLockResult};
use crate::step_receipt::{StepReceiptGenerator, StepReceipt, ResourceUsage};
use ed25519_dalek::SigningKey;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tracing::{debug, info, warn, error};

/// Container operation types that generate StepReceipts
#[derive(Debug, Clone)]
pub enum ContainerOperation {
    /// Container started
    Start { image: String, command: Vec<String> },
    /// Container stopped
    Stop { exit_code: i32, duration: Duration },
    /// Process execution
    Exec { pid: u32, command: String },
    /// File I/O operation
    FileIO { path: String, operation: String, bytes: u64 },
    /// Network I/O operation
    NetworkIO { direction: String, bytes: u64, destination: String },
    /// Memory allocation
    MemoryAlloc { size: u64, operation: String },
    /// System call
    Syscall { name: String, args: Vec<String>, result: String },
}

/// Container resource tracker for accurate usage measurement
#[derive(Debug, Clone)]
pub struct ResourceTracker {
    /// Start time of tracking
    start_time: Instant,
    /// CPU time accumulator (milliseconds)
    cpu_ms: u64,
    /// Memory usage accumulator (MB-seconds)
    memory_mb_s: u64,
    /// Storage usage accumulator (GB-days)
    storage_gb_day: f64,
    /// Network egress accumulator (MB)
    egress_mb: f64,
    /// Receipt count
    receipt_count: u64,
    /// Last measurement time
    last_measurement: Instant,
}

/// Container operation monitor
pub struct ContainerOperationMonitor {
    /// StepReceipt generator
    receipt_generator: Arc<Mutex<StepReceiptGenerator>>,
    /// Resource trackers per container
    resource_trackers: Arc<Mutex<HashMap<String, ResourceTracker>>>,
    /// Receipt sender channel
    receipt_sender: mpsc::UnboundedSender<StepReceipt>,
    /// Application ID
    app_id: String,
    /// Monitoring enabled flag
    enabled: bool,
}

impl ResourceTracker {
    /// Create new resource tracker
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            cpu_ms: 0,
            memory_mb_s: 0,
            storage_gb_day: 0.0,
            egress_mb: 0.0,
            receipt_count: 0,
            last_measurement: now,
        }
    }

    /// Update CPU usage
    pub fn add_cpu_time(&mut self, cpu_ms: u64) {
        self.cpu_ms += cpu_ms;
    }

    /// Update memory usage (called periodically with current memory usage)
    pub fn update_memory_usage(&mut self, memory_mb: u64) {
        let now = Instant::now();
        let duration_s = now.duration_since(self.last_measurement).as_secs();
        self.memory_mb_s += memory_mb * duration_s;
        self.last_measurement = now;
    }

    /// Add storage usage
    pub fn add_storage_usage(&mut self, storage_gb_day: f64) {
        self.storage_gb_day += storage_gb_day;
    }

    /// Add network egress
    pub fn add_egress(&mut self, egress_mb: f64) {
        self.egress_mb += egress_mb;
    }

    /// Increment receipt count
    pub fn increment_receipts(&mut self) {
        self.receipt_count += 1;
    }

    /// Get current resource usage
    pub fn get_usage(&self) -> ResourceUsage {
        ResourceUsage {
            cpu_ms: self.cpu_ms,
            memory_mb_s: self.memory_mb_s,
            storage_gb_day: self.storage_gb_day,
            egress_mb: self.egress_mb,
            receipts_count: self.receipt_count,
        }
    }

    /// Reset usage counters
    pub fn reset(&mut self) {
        self.cpu_ms = 0;
        self.memory_mb_s = 0;
        self.storage_gb_day = 0.0;
        self.egress_mb = 0.0;
        self.receipt_count = 0;
        self.last_measurement = Instant::now();
    }
}

impl ContainerOperationMonitor {
    /// Create new container operation monitor
    pub fn new(
        app_id: String,
        signing_key: SigningKey,
        receipt_sender: mpsc::UnboundedSender<StepReceipt>,
    ) -> Self {
        let receipt_generator = Arc::new(Mutex::new(
            StepReceiptGenerator::new(app_id.clone(), signing_key)
        ));

        Self {
            receipt_generator,
            resource_trackers: Arc::new(Mutex::new(HashMap::new())),
            receipt_sender,
            app_id,
            enabled: true,
        }
    }

    /// Enable/disable monitoring
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        info!("Container operation monitoring {}", if enabled { "enabled" } else { "disabled" });
    }

    /// Record a container operation and generate StepReceipt
    pub async fn record_operation(
        &self,
        container_id: String,
        operation: ContainerOperation,
        labels: HashMap<String, String>,
    ) -> DockLockResult<()> {
        if !self.enabled {
            return Ok(());
        }

        // Update resource tracking
        self.update_resource_tracking(&container_id, &operation).await?;

        // Get current resource usage
        let usage = {
            let trackers = self.resource_trackers.lock().unwrap();
            trackers.get(&container_id)
                .map(|tracker| tracker.get_usage())
                .unwrap_or_else(ResourceUsage::default)
        };

        // Generate operation string
        let op_string = self.operation_to_string(&operation);

        // Generate StepReceipt
        let receipt = {
            let mut generator = self.receipt_generator.lock().unwrap();
            generator.generate_receipt(
                container_id.clone(),
                op_string,
                usage,
                labels,
            )?
        };

        // Send receipt to pipeline
        if let Err(e) = self.receipt_sender.send(receipt.clone()) {
            error!("Failed to send StepReceipt to pipeline: {}", e);
            return Err(DockLockError::SystemError(format!("Receipt send failed: {}", e)));
        }

        debug!("Generated StepReceipt for container {} operation {}", 
               container_id, self.operation_to_string(&operation));

        Ok(())
    }

    /// Update resource tracking for an operation
    async fn update_resource_tracking(
        &self,
        container_id: &str,
        operation: &ContainerOperation,
    ) -> DockLockResult<()> {
        let mut trackers = self.resource_trackers.lock().unwrap();
        let tracker = trackers.entry(container_id.to_string())
            .or_insert_with(ResourceTracker::new);

        match operation {
            ContainerOperation::Start { .. } => {
                // Reset tracker for new container
                tracker.reset();
                tracker.add_cpu_time(10); // Startup overhead
            },
            ContainerOperation::Stop { duration, .. } => {
                tracker.add_cpu_time(duration.as_millis() as u64);
            },
            ContainerOperation::Exec { .. } => {
                tracker.add_cpu_time(5); // Execution overhead
            },
            ContainerOperation::FileIO { bytes, .. } => {
                let storage_gb = (*bytes as f64) / (1024.0 * 1024.0 * 1024.0);
                tracker.add_storage_usage(storage_gb / 365.0); // Convert to GB-days
            },
            ContainerOperation::NetworkIO { bytes, direction, .. } => {
                if direction == "egress" {
                    let egress_mb = (*bytes as f64) / (1024.0 * 1024.0);
                    tracker.add_egress(egress_mb);
                }
            },
            ContainerOperation::MemoryAlloc { size, .. } => {
                let memory_mb = (*size as f64) / (1024.0 * 1024.0);
                tracker.update_memory_usage(memory_mb as u64);
            },
            ContainerOperation::Syscall { .. } => {
                tracker.add_cpu_time(1); // Syscall overhead
            },
        }

        tracker.increment_receipts();
        Ok(())
    }

    /// Convert operation to string representation
    fn operation_to_string(&self, operation: &ContainerOperation) -> String {
        match operation {
            ContainerOperation::Start { .. } => "exec.start".to_string(),
            ContainerOperation::Stop { .. } => "exec.stop".to_string(),
            ContainerOperation::Exec { command, .. } => format!("exec.run:{}", command),
            ContainerOperation::FileIO { path, operation, .. } => {
                format!("io.{}:{}", operation, path)
            },
            ContainerOperation::NetworkIO { direction, destination, .. } => {
                format!("net.{}:{}", direction, destination)
            },
            ContainerOperation::MemoryAlloc { operation, .. } => {
                format!("mem.{}", operation)
            },
            ContainerOperation::Syscall { name, .. } => {
                format!("sys.{}", name)
            },
        }
    }

    /// Get resource tracker for a container
    pub fn get_container_usage(&self, container_id: &str) -> Option<ResourceUsage> {
        let trackers = self.resource_trackers.lock().unwrap();
        trackers.get(container_id).map(|tracker| tracker.get_usage())
    }

    /// Remove resource tracker for stopped container
    pub fn remove_container(&self, container_id: &str) {
        let mut trackers = self.resource_trackers.lock().unwrap();
        trackers.remove(container_id);
        debug!("Removed resource tracker for container {}", container_id);
    }

    /// Get total receipts generated
    pub fn get_total_receipts(&self) -> u64 {
        let trackers = self.resource_trackers.lock().unwrap();
        trackers.values().map(|tracker| tracker.receipt_count).sum()
    }

    /// Get active container count
    pub fn get_active_containers(&self) -> usize {
        let trackers = self.resource_trackers.lock().unwrap();
        trackers.len()
    }
}

/// Container operation hook for syscall interception
pub struct ContainerHook {
    monitor: Arc<ContainerOperationMonitor>,
}

impl ContainerHook {
    /// Create new container hook
    pub fn new(monitor: Arc<ContainerOperationMonitor>) -> Self {
        Self { monitor }
    }

    /// Hook for container start
    pub async fn on_container_start(
        &self,
        container_id: String,
        image: String,
        command: Vec<String>,
        labels: HashMap<String, String>,
    ) -> DockLockResult<()> {
        self.monitor.record_operation(
            container_id,
            ContainerOperation::Start { image, command },
            labels,
        ).await
    }

    /// Hook for container stop
    pub async fn on_container_stop(
        &self,
        container_id: String,
        exit_code: i32,
        duration: Duration,
        labels: HashMap<String, String>,
    ) -> DockLockResult<()> {
        let result = self.monitor.record_operation(
            container_id.clone(),
            ContainerOperation::Stop { exit_code, duration },
            labels,
        ).await;

        // Clean up resource tracker
        self.monitor.remove_container(&container_id);
        result
    }

    /// Hook for process execution
    pub async fn on_process_exec(
        &self,
        container_id: String,
        pid: u32,
        command: String,
        labels: HashMap<String, String>,
    ) -> DockLockResult<()> {
        self.monitor.record_operation(
            container_id,
            ContainerOperation::Exec { pid, command },
            labels,
        ).await
    }

    /// Hook for file I/O
    pub async fn on_file_io(
        &self,
        container_id: String,
        path: String,
        operation: String,
        bytes: u64,
        labels: HashMap<String, String>,
    ) -> DockLockResult<()> {
        self.monitor.record_operation(
            container_id,
            ContainerOperation::FileIO { path, operation, bytes },
            labels,
        ).await
    }

    /// Hook for network I/O
    pub async fn on_network_io(
        &self,
        container_id: String,
        direction: String,
        bytes: u64,
        destination: String,
        labels: HashMap<String, String>,
    ) -> DockLockResult<()> {
        self.monitor.record_operation(
            container_id,
            ContainerOperation::NetworkIO { direction, bytes, destination },
            labels,
        ).await
    }

    /// Hook for syscall
    pub async fn on_syscall(
        &self,
        container_id: String,
        name: String,
        args: Vec<String>,
        result: String,
        labels: HashMap<String, String>,
    ) -> DockLockResult<()> {
        self.monitor.record_operation(
            container_id,
            ContainerOperation::Syscall { name, args, result },
            labels,
        ).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_container_operation_monitoring() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let (tx, mut rx) = mpsc::unbounded_channel();

        let monitor = ContainerOperationMonitor::new(
            "TEST_APP".to_string(),
            signing_key,
            tx,
        );

        let mut labels = HashMap::new();
        labels.insert("test".to_string(), "true".to_string());

        // Test container start
        monitor.record_operation(
            "test-container".to_string(),
            ContainerOperation::Start {
                image: "test-image".to_string(),
                command: vec!["echo".to_string(), "hello".to_string()],
            },
            labels.clone(),
        ).await.unwrap();

        // Should receive a StepReceipt
        let receipt = rx.recv().await.unwrap();
        assert_eq!(receipt.app, "TEST_APP");
        assert_eq!(receipt.container, "test-container");
        assert_eq!(receipt.op, "exec.start");

        // Test resource tracking
        let usage = monitor.get_container_usage("test-container").unwrap();
        assert!(usage.cpu_ms > 0);
        assert_eq!(usage.receipts_count, 1);
    }

    #[test]
    fn test_resource_tracker() {
        let mut tracker = ResourceTracker::new();
        
        tracker.add_cpu_time(100);
        // Sleep for 1 second to ensure duration > 0 for memory calculation
        std::thread::sleep(std::time::Duration::from_secs(1));
        tracker.update_memory_usage(50);
        tracker.add_storage_usage(0.1);
        tracker.add_egress(1.5);
        tracker.increment_receipts();

        let usage = tracker.get_usage();
        assert_eq!(usage.cpu_ms, 100);
        assert!(usage.memory_mb_s > 0);
        assert_eq!(usage.storage_gb_day, 0.1);
        assert_eq!(usage.egress_mb, 1.5);
        assert_eq!(usage.receipts_count, 1);
    }
}
