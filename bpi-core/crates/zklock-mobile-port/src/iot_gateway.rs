//! IoT Gateway - Ultra-lightweight protocol for embedded devices
//!
//! This module provides a minimal protocol for IoT devices with severe
//! computational and network constraints.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::{IoTConfig, ComputeLevel};

/// IoT gateway for ultra-lightweight devices
#[derive(Debug)]
pub struct IoTGateway {
    /// Connected IoT devices
    connected_devices: Arc<RwLock<HashMap<Uuid, IoTDevice>>>,
    /// Message queue for offline devices
    message_queue: Arc<RwLock<HashMap<Uuid, Vec<IoTMessage>>>>,
    /// Configuration
    config: IoTConfig,
    /// Gateway statistics
    stats: Arc<RwLock<IoTGatewayStats>>,
}

/// IoT device connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTDevice {
    pub device_id: Uuid,
    pub device_class: IoTClass,
    pub compute_level: ComputeLevel,
    pub connection_time: chrono::DateTime<chrono::Utc>,
    pub last_heartbeat: chrono::DateTime<chrono::Utc>,
    pub status: IoTDeviceStatus,
    pub protocol_version: u8,
    pub supported_features: Vec<IoTFeature>,
    pub resource_constraints: ResourceConstraints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IoTClass {
    Sensor,
    Actuator,
    Gateway,
    Controller,
    Monitor,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IoTDeviceStatus {
    Connected,
    Idle,
    Sleeping,
    Offline,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IoTFeature {
    BasicMessaging,
    CompressedData,
    OfflineQueue,
    LowPowerMode,
    BurstTransmission,
    EdgeCaching,
}

/// Resource constraints for IoT devices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConstraints {
    pub max_message_size: usize,
    pub max_queue_size: usize,
    pub battery_level: Option<f64>,
    pub memory_available: usize,
    pub processing_budget: f64, // CPU cycles per second
    pub network_budget: u64,    // bytes per minute
}

/// IoT message optimized for minimal overhead
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTMessage {
    pub message_id: u32,        // 4 bytes instead of UUID
    pub device_id: Uuid,
    pub message_type: IoTMessageType,
    pub payload: Vec<u8>,       // Minimal payload
    pub timestamp: u32,         // Unix timestamp (4 bytes)
    pub priority: u8,           // 1 byte priority
    pub ttl: u16,              // Time to live in seconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IoTMessageType {
    Heartbeat,
    SensorData,
    ProofSubmission,
    StatusUpdate,
    Command,
    Response,
}

/// IoT gateway statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTGatewayStats {
    pub total_devices: u64,
    pub connected_devices: u64,
    pub sleeping_devices: u64,
    pub offline_devices: u64,
    pub total_messages: u64,
    pub queued_messages: u64,
    pub average_message_size: f64,
    pub battery_efficiency_score: f64,
    pub network_utilization: f64,
}

impl Default for IoTGatewayStats {
    fn default() -> Self {
        Self {
            total_devices: 0,
            connected_devices: 0,
            sleeping_devices: 0,
            offline_devices: 0,
            total_messages: 0,
            queued_messages: 0,
            average_message_size: 0.0,
            battery_efficiency_score: 100.0,
            network_utilization: 0.0,
        }
    }
}

impl IoTGateway {
    /// Create a new IoT gateway
    pub async fn new(config: IoTConfig) -> Result<Self> {
        info!("Initializing IoT Gateway");

        Ok(Self {
            connected_devices: Arc::new(RwLock::new(HashMap::new())),
            message_queue: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(RwLock::new(IoTGatewayStats::default())),
        })
    }

    /// Start the IoT gateway
    pub async fn start(&self) -> Result<()> {
        info!("Starting IoT Gateway");

        // Start background tasks
        self.start_heartbeat_monitoring().await;
        self.start_message_processing().await;
        self.start_stats_update_task().await;
        self.start_power_management().await;

        Ok(())
    }

    /// Connect an IoT device
    pub async fn connect_device(&self, device_id: Uuid, device_class: IoTClass, compute_level: ComputeLevel) -> Result<()> {
        // Check compute level meets minimum requirements
        if !self.meets_minimum_requirements(&compute_level) {
            return Err(anyhow::anyhow!("Device does not meet minimum compute requirements"));
        }

        let resource_constraints = self.assess_resource_constraints(&compute_level);
        let supported_features = self.determine_supported_features(&compute_level);

        let device = IoTDevice {
            device_id,
            device_class: device_class.clone(),
            compute_level,
            connection_time: chrono::Utc::now(),
            last_heartbeat: chrono::Utc::now(),
            status: IoTDeviceStatus::Connected,
            protocol_version: 1,
            supported_features,
            resource_constraints,
        };

        self.connected_devices.write().await.insert(device_id, device);
        
        // Initialize message queue for device
        self.message_queue.write().await.insert(device_id, Vec::new());

        // Update statistics
        self.update_stats_after_connection().await;

        info!("Connected IoT device {} with class {:?}", device_id, device_class);
        Ok(())
    }

    /// Disconnect an IoT device
    pub async fn disconnect_device(&self, device_id: Uuid) -> Result<()> {
        self.connected_devices.write().await.remove(&device_id);
        
        // Keep message queue for potential reconnection
        // Queue will be cleaned up after TTL expires

        info!("Disconnected IoT device {}", device_id);
        Ok(())
    }

    /// Send message to IoT device
    pub async fn send_message(&self, device_id: Uuid, message_type: IoTMessageType, payload: Vec<u8>) -> Result<()> {
        // Check if device is connected
        let device = self.connected_devices.read().await
            .get(&device_id)
            .cloned()
            .context("Device not connected")?;

        // Check message size constraints
        if payload.len() > device.resource_constraints.max_message_size {
            return Err(anyhow::anyhow!("Message too large for device constraints"));
        }

        let message = IoTMessage {
            message_id: self.generate_message_id().await,
            device_id,
            message_type,
            payload,
            timestamp: chrono::Utc::now().timestamp() as u32,
            priority: 1,
            ttl: 300, // 5 minutes default TTL
        };

        // Add to message queue
        let mut queue = self.message_queue.write().await;
        if let Some(device_queue) = queue.get_mut(&device_id) {
            if device_queue.len() < device.resource_constraints.max_queue_size {
                device_queue.push(message);
            } else {
                warn!("Message queue full for device {}", device_id);
                return Err(anyhow::anyhow!("Device message queue is full"));
            }
        }

        Ok(())
    }

    /// Receive message from IoT device
    pub async fn receive_message(&self, device_id: Uuid, message: IoTMessage) -> Result<()> {
        // Update device heartbeat
        if let Some(device) = self.connected_devices.write().await.get_mut(&device_id) {
            device.last_heartbeat = chrono::Utc::now();
            device.status = IoTDeviceStatus::Connected;
        }

        // Process message based on type
        match message.message_type {
            IoTMessageType::Heartbeat => {
                debug!("Received heartbeat from device {}", device_id);
            },
            IoTMessageType::SensorData => {
                self.process_sensor_data(device_id, &message.payload).await?;
            },
            IoTMessageType::ProofSubmission => {
                self.process_proof_submission(device_id, &message.payload).await?;
            },
            IoTMessageType::StatusUpdate => {
                self.process_status_update(device_id, &message.payload).await?;
            },
            _ => {
                debug!("Received message type {:?} from device {}", message.message_type, device_id);
            }
        }

        // Update statistics
        self.update_stats_after_message(message.payload.len()).await;

        Ok(())
    }

    /// Get IoT device status
    pub async fn get_device_status(&self, device_id: Uuid) -> Result<IoTDevice> {
        self.connected_devices.read().await
            .get(&device_id)
            .cloned()
            .context("Device not found")
    }

    /// Get gateway statistics
    pub async fn get_stats(&self) -> Result<IoTGatewayStats> {
        Ok(self.stats.read().await.clone())
    }

    /// Check if device meets minimum requirements
    fn meets_minimum_requirements(&self, compute_level: &ComputeLevel) -> bool {
        match (&self.config.min_compute_level, compute_level) {
            (ComputeLevel::Minimal, _) => true,
            (ComputeLevel::Light, ComputeLevel::Minimal) => false,
            (ComputeLevel::Light, _) => true,
            (ComputeLevel::Standard, ComputeLevel::Minimal | ComputeLevel::Light) => false,
            (ComputeLevel::Standard, _) => true,
            (ComputeLevel::Enhanced, ComputeLevel::Enhanced) => true,
            (ComputeLevel::Enhanced, _) => false,
        }
    }

    /// Assess resource constraints based on compute level
    fn assess_resource_constraints(&self, compute_level: &ComputeLevel) -> ResourceConstraints {
        match compute_level {
            ComputeLevel::Minimal => ResourceConstraints {
                max_message_size: 64,   // 64 bytes max
                max_queue_size: 5,      // 5 messages max
                battery_level: Some(50.0),
                memory_available: 1024, // 1KB
                processing_budget: 1000.0, // 1K cycles/sec
                network_budget: 1024,   // 1KB/min
            },
            ComputeLevel::Light => ResourceConstraints {
                max_message_size: 256,  // 256 bytes max
                max_queue_size: 10,     // 10 messages max
                battery_level: Some(60.0),
                memory_available: 10240, // 10KB
                processing_budget: 10000.0, // 10K cycles/sec
                network_budget: 10240,  // 10KB/min
            },
            ComputeLevel::Standard => ResourceConstraints {
                max_message_size: 1024, // 1KB max
                max_queue_size: 50,     // 50 messages max
                battery_level: Some(70.0),
                memory_available: 102400, // 100KB
                processing_budget: 100000.0, // 100K cycles/sec
                network_budget: 102400, // 100KB/min
            },
            ComputeLevel::Enhanced => ResourceConstraints {
                max_message_size: 4096, // 4KB max
                max_queue_size: 100,    // 100 messages max
                battery_level: Some(80.0),
                memory_available: 1048576, // 1MB
                processing_budget: 1000000.0, // 1M cycles/sec
                network_budget: 1048576, // 1MB/min
            },
        }
    }

    /// Determine supported features based on compute level
    fn determine_supported_features(&self, compute_level: &ComputeLevel) -> Vec<IoTFeature> {
        let mut features = vec![IoTFeature::BasicMessaging];

        match compute_level {
            ComputeLevel::Minimal => {
                features.push(IoTFeature::LowPowerMode);
            },
            ComputeLevel::Light => {
                features.push(IoTFeature::LowPowerMode);
                features.push(IoTFeature::OfflineQueue);
            },
            ComputeLevel::Standard => {
                features.push(IoTFeature::LowPowerMode);
                features.push(IoTFeature::OfflineQueue);
                features.push(IoTFeature::CompressedData);
                features.push(IoTFeature::BurstTransmission);
            },
            ComputeLevel::Enhanced => {
                features.push(IoTFeature::LowPowerMode);
                features.push(IoTFeature::OfflineQueue);
                features.push(IoTFeature::CompressedData);
                features.push(IoTFeature::BurstTransmission);
                features.push(IoTFeature::EdgeCaching);
            },
        }

        features
    }

    /// Generate unique message ID
    async fn generate_message_id(&self) -> u32 {
        // Simple incrementing counter (in real implementation, use atomic counter)
        self.stats.read().await.total_messages as u32 + 1
    }

    /// Process sensor data from IoT device
    async fn process_sensor_data(&self, device_id: Uuid, payload: &[u8]) -> Result<()> {
        // In a real implementation, this would:
        // 1. Parse sensor data
        // 2. Validate data integrity
        // 3. Store in time-series database
        // 4. Trigger alerts if needed
        // 5. Forward to BPI system if relevant

        debug!("Processing sensor data from device {}: {} bytes", device_id, payload.len());
        Ok(())
    }

    /// Process proof submission from IoT device
    async fn process_proof_submission(&self, device_id: Uuid, payload: &[u8]) -> Result<()> {
        // In a real implementation, this would:
        // 1. Validate proof format
        // 2. Submit to ZK Merkle accumulator
        // 3. Award tokens through ICO system
        // 4. Update device participation record

        info!("Processing proof submission from device {}: {} bytes", device_id, payload.len());
        Ok(())
    }

    /// Process status update from IoT device
    async fn process_status_update(&self, device_id: Uuid, payload: &[u8]) -> Result<()> {
        // Parse status update (simplified)
        if payload.len() >= 4 {
            let battery_level = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]) as f64 / 100.0;
            
            // Update device battery level
            if let Some(device) = self.connected_devices.write().await.get_mut(&device_id) {
                device.resource_constraints.battery_level = Some(battery_level);
                
                // Adjust device status based on battery
                if battery_level < 10.0 {
                    device.status = IoTDeviceStatus::Sleeping;
                } else if battery_level < 20.0 {
                    device.status = IoTDeviceStatus::Idle;
                } else {
                    device.status = IoTDeviceStatus::Connected;
                }
            }
        }

        Ok(())
    }

    /// Update statistics after device connection
    async fn update_stats_after_connection(&self) {
        let mut stats = self.stats.write().await;
        stats.total_devices += 1;
        stats.connected_devices += 1;
    }

    /// Update statistics after message processing
    async fn update_stats_after_message(&self, message_size: usize) {
        let mut stats = self.stats.write().await;
        stats.total_messages += 1;
        
        // Update average message size
        let total_size = stats.average_message_size * (stats.total_messages - 1) as f64 + message_size as f64;
        stats.average_message_size = total_size / stats.total_messages as f64;
    }

    /// Start heartbeat monitoring background task
    async fn start_heartbeat_monitoring(&self) {
        let connected_devices = Arc::clone(&self.connected_devices);
        let heartbeat_interval = self.config.heartbeat_interval;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(heartbeat_interval));
            
            loop {
                interval.tick().await;
                
                // Check for devices that haven't sent heartbeat
                let timeout = chrono::Utc::now() - chrono::Duration::seconds(heartbeat_interval as i64 * 2);
                let mut devices = connected_devices.write().await;
                
                for (device_id, device) in devices.iter_mut() {
                    if device.last_heartbeat < timeout && device.status == IoTDeviceStatus::Connected {
                        device.status = IoTDeviceStatus::Offline;
                        warn!("Device {} marked as offline due to missing heartbeat", device_id);
                    }
                }
            }
        });
    }

    /// Start message processing background task
    async fn start_message_processing(&self) {
        let message_queue = Arc::clone(&self.message_queue);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5)); // Process every 5 seconds
            
            loop {
                interval.tick().await;
                
                // Process queued messages and remove expired ones
                let now = chrono::Utc::now().timestamp() as u32;
                let mut queue = message_queue.write().await;
                
                for (device_id, device_queue) in queue.iter_mut() {
                    let initial_size = device_queue.len();
                    
                    // Remove expired messages
                    device_queue.retain(|msg| (now - msg.timestamp) < msg.ttl as u32);
                    
                    let expired = initial_size - device_queue.len();
                    if expired > 0 {
                        debug!("Removed {} expired messages for device {}", expired, device_id);
                    }
                }
            }
        });
    }

    /// Start statistics update background task
    async fn start_stats_update_task(&self) {
        let connected_devices = Arc::clone(&self.connected_devices);
        let message_queue = Arc::clone(&self.message_queue);
        let stats = Arc::clone(&self.stats);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60)); // 1 minute
            
            loop {
                interval.tick().await;
                
                let devices = connected_devices.read().await;
                let queue = message_queue.read().await;
                let mut stats = stats.write().await;
                
                // Update device status counts
                stats.connected_devices = devices.values().filter(|d| d.status == IoTDeviceStatus::Connected).count() as u64;
                stats.sleeping_devices = devices.values().filter(|d| d.status == IoTDeviceStatus::Sleeping).count() as u64;
                stats.offline_devices = devices.values().filter(|d| d.status == IoTDeviceStatus::Offline).count() as u64;
                
                // Update queued messages count
                stats.queued_messages = queue.values().map(|q| q.len() as u64).sum();
                
                // Update battery efficiency score
                let battery_levels: Vec<f64> = devices.values()
                    .filter_map(|d| d.resource_constraints.battery_level)
                    .collect();
                
                if !battery_levels.is_empty() {
                    stats.battery_efficiency_score = battery_levels.iter().sum::<f64>() / battery_levels.len() as f64;
                }
            }
        });
    }

    /// Start power management background task
    async fn start_power_management(&self) {
        let connected_devices = Arc::clone(&self.connected_devices);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // 5 minutes
            
            loop {
                interval.tick().await;
                
                // Implement power management strategies
                let mut devices = connected_devices.write().await;
                for (device_id, device) in devices.iter_mut() {
                    if let Some(battery_level) = device.resource_constraints.battery_level {
                        if battery_level < 15.0 && device.status == IoTDeviceStatus::Connected {
                            // Put device in power saving mode
                            device.status = IoTDeviceStatus::Sleeping;
                            info!("Device {} entered sleep mode due to low battery: {:.1}%", 
                                  device_id, battery_level);
                        } else if battery_level > 30.0 && device.status == IoTDeviceStatus::Sleeping {
                            // Wake up device
                            device.status = IoTDeviceStatus::Connected;
                            info!("Device {} woke up, battery level: {:.1}%", device_id, battery_level);
                        }
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_iot_gateway_creation() {
        let config = IoTConfig {
            min_compute_level: ComputeLevel::Minimal,
            max_message_size: 256,
            heartbeat_interval: 60,
        };
        
        let gateway = IoTGateway::new(config).await.unwrap();
        let stats = gateway.get_stats().await.unwrap();
        assert_eq!(stats.total_devices, 0);
    }

    #[tokio::test]
    async fn test_device_connection() {
        let config = IoTConfig {
            min_compute_level: ComputeLevel::Minimal,
            max_message_size: 256,
            heartbeat_interval: 60,
        };
        
        let gateway = IoTGateway::new(config).await.unwrap();
        let device_id = Uuid::new_v4();
        
        gateway.connect_device(device_id, IoTClass::Sensor, ComputeLevel::Light).await.unwrap();
        
        let device = gateway.get_device_status(device_id).await.unwrap();
        assert_eq!(device.device_id, device_id);
        assert_eq!(device.status, IoTDeviceStatus::Connected);
    }

    #[tokio::test]
    async fn test_message_handling() {
        let config = IoTConfig {
            min_compute_level: ComputeLevel::Minimal,
            max_message_size: 256,
            heartbeat_interval: 60,
        };
        
        let gateway = IoTGateway::new(config).await.unwrap();
        let device_id = Uuid::new_v4();
        
        gateway.connect_device(device_id, IoTClass::Sensor, ComputeLevel::Standard).await.unwrap();
        
        // Send message to device
        let payload = b"test_data".to_vec();
        gateway.send_message(device_id, IoTMessageType::Command, payload).await.unwrap();
        
        // Receive message from device
        let message = IoTMessage {
            message_id: 1,
            device_id,
            message_type: IoTMessageType::SensorData,
            payload: b"sensor_reading".to_vec(),
            timestamp: chrono::Utc::now().timestamp() as u32,
            priority: 1,
            ttl: 300,
        };
        
        gateway.receive_message(device_id, message).await.unwrap();
    }
}
