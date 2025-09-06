//! # ZKLock Mobile Port - Revolutionary IoT & Mobile DApp Infrastructure
//!
//! This crate provides a zero-knowledge Merkle accumulator system optimized for
//! IoT devices, mobile phones, and other light-compute environments. It enables
//! these devices to participate in the BPI ecosystem with minimal resource usage.
//!
//! ## Features
//!
//! - **ZK Merkle Accumulator**: Efficient state management without full blockchain sync
//! - **Light Consensus Protocol**: Minimal participation requirements for mobile devices
//! - **ICO Token Distribution**: Device participation rewards and staking
//! - **Mobile SDK**: Battery-optimized APIs for mobile applications
//! - **IoT Gateway**: Ultra-lightweight protocol for embedded devices
//! - **BPI Integration**: Seamless connection to existing BPI infrastructure

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

pub mod zk_merkle_accumulator;
pub mod light_consensus;
pub mod ico_participation;
pub mod mobile_api;
pub mod iot_gateway;
pub mod device_manager;
pub mod proof_optimizer;

pub use zk_merkle_accumulator::*;
pub use light_consensus::*;
pub use ico_participation::*;
pub use mobile_api::*;
pub use iot_gateway::*;
pub use device_manager::*;
pub use proof_optimizer::*;

/// Device types supported by zklock system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DeviceType {
    /// Mobile phones and tablets
    Mobile {
        platform: MobilePlatform,
        capabilities: MobileCapabilities,
    },
    /// IoT sensors and embedded devices
    IoT {
        device_class: IoTClass,
        compute_level: ComputeLevel,
    },
    /// Edge computing devices
    Edge {
        processing_power: ProcessingPower,
        connectivity: ConnectivityType,
    },
    /// Wearable devices
    Wearable {
        form_factor: WearableType,
        battery_class: BatteryClass,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MobilePlatform {
    Android,
    iOS,
    HarmonyOS,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MobileCapabilities {
    pub ram_mb: u32,
    pub storage_gb: u32,
    pub has_secure_enclave: bool,
    pub supports_biometrics: bool,
    pub network_types: Vec<NetworkType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum IoTClass {
    Sensor,
    Actuator,
    Gateway,
    Controller,
    Monitor,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComputeLevel {
    Minimal,    // <1MB RAM, <10MHz
    Light,      // 1-10MB RAM, 10-100MHz
    Standard,   // 10-100MB RAM, 100MHz-1GHz
    Enhanced,   // >100MB RAM, >1GHz
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProcessingPower {
    Low,
    Medium,
    High,
    Enterprise,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConnectivityType {
    WiFi,
    Cellular,
    Bluetooth,
    LoRa,
    Zigbee,
    Ethernet,
    Satellite,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WearableType {
    Smartwatch,
    FitnessTracker,
    SmartGlasses,
    HealthMonitor,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BatteryClass {
    UltraLow,   // <100mAh
    Low,        // 100-500mAh
    Standard,   // 500-2000mAh
    High,       // >2000mAh
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NetworkType {
    FiveG,
    FourG,
    ThreeG,
    WiFi,
    Bluetooth,
    NFC,
}

/// ZKLock mobile port system - main orchestrator
#[derive(Debug)]
pub struct ZKLockMobilePort {
    /// Device manager for registration and lifecycle
    pub device_manager: Arc<DeviceManager>,
    /// ZK Merkle accumulator for state management
    pub merkle_accumulator: Arc<ZKMerkleAccumulator>,
    /// Light consensus protocol
    pub consensus: Arc<LightConsensus>,
    /// ICO participation system
    pub ico_system: Arc<ICOParticipation>,
    /// Mobile API server
    pub mobile_api: Arc<MobileAPI>,
    /// IoT gateway for ultra-lightweight devices
    pub iot_gateway: Arc<IoTGateway>,
    /// Proof optimizer for mobile devices
    pub proof_optimizer: Arc<ProofOptimizer>,
    /// System configuration
    pub config: ZKLockConfig,
    /// Active device sessions
    pub device_sessions: Arc<RwLock<HashMap<Uuid, DeviceSession>>>,
}

/// Configuration for ZKLock mobile port system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZKLockConfig {
    /// Maximum number of concurrent device sessions
    pub max_device_sessions: usize,
    /// ZK proof parameters
    pub zk_config: ZKConfig,
    /// Mobile optimization settings
    pub mobile_config: MobileConfig,
    /// IoT gateway settings
    pub iot_config: IoTConfig,
    /// ICO participation settings
    pub ico_config: ICOConfig,
    /// BPI integration settings
    pub bpi_integration: BPIIntegrationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZKConfig {
    /// Maximum proof size in bytes (optimized for mobile)
    pub max_proof_size: usize,
    /// Maximum verification time in milliseconds
    pub max_verification_time_ms: u64,
    /// Merkle tree depth
    pub merkle_depth: usize,
    /// Batch size for proof generation
    pub batch_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileConfig {
    /// Battery optimization level
    pub battery_optimization: BatteryOptimization,
    /// Network usage limits
    pub network_limits: NetworkLimits,
    /// Cache settings
    pub cache_config: CacheConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BatteryOptimization {
    Aggressive,
    Balanced,
    Performance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkLimits {
    /// Maximum bytes per minute
    pub max_bytes_per_minute: u64,
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Prefer WiFi over cellular
    pub prefer_wifi: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Maximum cache size in MB
    pub max_cache_size_mb: usize,
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
    /// Enable persistent cache
    pub persistent_cache: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTConfig {
    /// Minimum compute level required
    pub min_compute_level: ComputeLevel,
    /// Maximum message size for IoT devices
    pub max_message_size: usize,
    /// Heartbeat interval in seconds
    pub heartbeat_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ICOConfig {
    /// Token reward per device participation
    pub base_reward_tokens: u64,
    /// Bonus multiplier for different device types
    pub device_type_multipliers: HashMap<String, f64>,
    /// Minimum participation time for rewards
    pub min_participation_hours: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BPIIntegrationConfig {
    /// BPI gateway endpoint
    pub gateway_endpoint: String,
    /// Wallet integration endpoint
    pub wallet_endpoint: String,
    /// Domain resolver endpoint
    pub domain_resolver_endpoint: String,
    /// ENC cluster manager endpoint
    pub enc_cluster_endpoint: String,
}

/// Active device session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSession {
    pub device_id: Uuid,
    pub device_type: DeviceType,
    pub wallet_address: String,
    pub session_start: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
    pub participation_score: f64,
    pub tokens_earned: u64,
    pub proof_count: u64,
    pub status: SessionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionStatus {
    Active,
    Idle,
    Suspended,
    Terminated,
}

impl Default for ZKLockConfig {
    fn default() -> Self {
        let mut device_multipliers = HashMap::new();
        device_multipliers.insert("Mobile".to_string(), 1.0);
        device_multipliers.insert("IoT".to_string(), 0.5);
        device_multipliers.insert("Edge".to_string(), 2.0);
        device_multipliers.insert("Wearable".to_string(), 0.3);

        Self {
            max_device_sessions: 10000,
            zk_config: ZKConfig {
                max_proof_size: 1024,      // 1KB max proof size
                max_verification_time_ms: 100,  // 100ms max verification
                merkle_depth: 20,
                batch_size: 100,
            },
            mobile_config: MobileConfig {
                battery_optimization: BatteryOptimization::Balanced,
                network_limits: NetworkLimits {
                    max_bytes_per_minute: 1024 * 1024, // 1MB per minute
                    max_connections: 5,
                    prefer_wifi: true,
                },
                cache_config: CacheConfig {
                    max_cache_size_mb: 10,
                    cache_ttl_seconds: 3600,
                    persistent_cache: true,
                },
            },
            iot_config: IoTConfig {
                min_compute_level: ComputeLevel::Minimal,
                max_message_size: 256,     // 256 bytes max for IoT
                heartbeat_interval: 60,    // 1 minute heartbeat
            },
            ico_config: ICOConfig {
                base_reward_tokens: 100,
                device_type_multipliers: device_multipliers,
                min_participation_hours: 1,
            },
            bpi_integration: BPIIntegrationConfig {
                gateway_endpoint: "http://localhost:8080".to_string(),
                wallet_endpoint: "http://localhost:8081".to_string(),
                domain_resolver_endpoint: "http://localhost:8082".to_string(),
                enc_cluster_endpoint: "http://localhost:8083".to_string(),
            },
        }
    }
}

impl ZKLockMobilePort {
    /// Create a new ZKLock mobile port system
    pub async fn new(config: ZKLockConfig) -> Result<Self> {
        info!("Initializing ZKLock Mobile Port system");

        let device_manager = Arc::new(DeviceManager::new(config.clone()).await?);
        let merkle_accumulator = Arc::new(ZKMerkleAccumulator::new(config.zk_config.clone()).await?);
        let consensus = Arc::new(LightConsensus::new(config.clone()).await?);
        let ico_system = Arc::new(ICOParticipation::new(config.ico_config.clone()).await?);
        let mobile_api = Arc::new(MobileAPI::new(config.mobile_config.clone()).await?);
        let iot_gateway = Arc::new(IoTGateway::new(config.iot_config.clone()).await?);
        let proof_optimizer = Arc::new(ProofOptimizer::new(config.zk_config.clone()).await?);

        Ok(Self {
            device_manager,
            merkle_accumulator,
            consensus,
            ico_system,
            mobile_api,
            iot_gateway,
            proof_optimizer,
            config,
            device_sessions: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Start the ZKLock mobile port system
    pub async fn start(&self) -> Result<()> {
        info!("Starting ZKLock Mobile Port system");

        // Start all subsystems
        self.device_manager.start().await?;
        self.merkle_accumulator.start().await?;
        self.consensus.start().await?;
        self.ico_system.start().await?;
        self.mobile_api.start().await?;
        self.iot_gateway.start().await?;

        info!("ZKLock Mobile Port system started successfully");
        Ok(())
    }

    /// Register a new device
    pub async fn register_device(&self, device_type: DeviceType, wallet_address: String) -> Result<Uuid> {
        let device_id = self.device_manager.register_device(device_type.clone(), wallet_address.clone()).await?;

        // Register device with ICO participation system
        self.ico_system.register_device(device_id, device_type.clone(), wallet_address.clone()).await?;

        // Create device session
        let session = DeviceSession {
            device_id,
            device_type,
            wallet_address,
            session_start: chrono::Utc::now(),
            last_activity: chrono::Utc::now(),
            participation_score: 0.0,
            tokens_earned: 0,
            proof_count: 0,
            status: SessionStatus::Active,
        };

        self.device_sessions.write().await.insert(device_id, session);

        info!("Device registered successfully: {}", device_id);
        Ok(device_id)
    }

    /// Submit a proof from a device
    pub async fn submit_proof(&self, device_id: Uuid, proof_data: Vec<u8>) -> Result<String> {
        // Verify device is registered and active
        let mut sessions = self.device_sessions.write().await;
        let session = sessions.get_mut(&device_id)
            .context("Device not found or not active")?;

        if session.status != SessionStatus::Active {
            return Err(anyhow::anyhow!("Device session is not active"));
        }

        // Optimize proof for mobile device
        let optimized_proof = self.proof_optimizer.optimize_proof(proof_data, &session.device_type).await?;

        // Submit to Merkle accumulator
        let proof_id = self.merkle_accumulator.add_proof(device_id, optimized_proof).await?;

        // Update session
        session.last_activity = chrono::Utc::now();
        session.proof_count += 1;
        session.participation_score += 1.0;

        // Award tokens through ICO system
        let tokens_awarded = self.ico_system.award_tokens(device_id, &session.device_type).await?;
        session.tokens_earned += tokens_awarded;

        info!("Proof submitted successfully: {} from device {}", proof_id, device_id);
        Ok(proof_id)
    }

    /// Get device session status
    pub async fn get_device_status(&self, device_id: Uuid) -> Result<DeviceSession> {
        let sessions = self.device_sessions.read().await;
        sessions.get(&device_id)
            .cloned()
            .context("Device session not found")
    }

    /// Get system statistics
    pub async fn get_system_stats(&self) -> Result<SystemStats> {
        let sessions = self.device_sessions.read().await;
        let total_devices = sessions.len();
        let active_devices = sessions.values().filter(|s| s.status == SessionStatus::Active).count();
        let total_proofs = sessions.values().map(|s| s.proof_count).sum();
        let total_tokens_distributed = sessions.values().map(|s| s.tokens_earned).sum();

        let merkle_stats = self.merkle_accumulator.get_stats().await?;
        let ico_stats = self.ico_system.get_stats().await?;

        Ok(SystemStats {
            total_devices,
            active_devices,
            total_proofs,
            total_tokens_distributed,
            merkle_tree_size: merkle_stats.tree_size,
            ico_participation_rate: ico_stats.participation_rate,
        })
    }
}

/// System statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub total_devices: usize,
    pub active_devices: usize,
    pub total_proofs: u64,
    pub total_tokens_distributed: u64,
    pub merkle_tree_size: u64,
    pub ico_participation_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_zklock_mobile_port_creation() {
        let config = ZKLockConfig::default();
        let zklock = ZKLockMobilePort::new(config).await.unwrap();
        assert_eq!(zklock.config.max_device_sessions, 10000);
    }

    #[tokio::test]
    async fn test_device_registration() {
        let config = ZKLockConfig::default();
        let zklock = ZKLockMobilePort::new(config).await.unwrap();

        let device_type = DeviceType::Mobile {
            platform: MobilePlatform::Android,
            capabilities: MobileCapabilities {
                ram_mb: 4096,
                storage_gb: 64,
                has_secure_enclave: true,
                supports_biometrics: true,
                network_types: vec![NetworkType::FiveG, NetworkType::WiFi],
            },
        };

        let device_id = zklock.register_device(device_type, "wallet123".to_string()).await.unwrap();
        assert!(!device_id.is_nil());

        let status = zklock.get_device_status(device_id).await.unwrap();
        assert_eq!(status.status, SessionStatus::Active);
    }
}
