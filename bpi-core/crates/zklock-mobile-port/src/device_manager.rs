//! Device Manager - Registration and lifecycle management for mobile/IoT devices
//!
//! This module handles device registration, authentication, lifecycle management,
//! and integration with the BPI ecosystem.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::{ZKLockConfig, DeviceType, ComputeLevel};

/// Device manager for registration and lifecycle
#[derive(Debug)]
pub struct DeviceManager {
    /// Registered devices
    devices: Arc<RwLock<HashMap<Uuid, RegisteredDevice>>>,
    /// Device authentication tokens
    auth_tokens: Arc<RwLock<HashMap<String, AuthToken>>>,
    /// Device capabilities cache
    capabilities_cache: Arc<RwLock<HashMap<Uuid, DeviceCapabilities>>>,
    /// Configuration
    config: ZKLockConfig,
    /// Manager statistics
    stats: Arc<RwLock<DeviceManagerStats>>,
}

/// Registered device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredDevice {
    pub device_id: Uuid,
    pub device_type: DeviceType,
    pub wallet_address: String,
    pub registration_time: chrono::DateTime<chrono::Utc>,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub status: DeviceStatus,
    pub capabilities: DeviceCapabilities,
    pub trust_score: f64,
    pub violation_count: u32,
    pub total_uptime_hours: f64,
    pub bpi_integration: BPIIntegration,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceStatus {
    Pending,
    Active,
    Suspended,
    Banned,
    Offline,
}

/// Device capabilities assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    pub compute_score: f64,        // 0.0 - 1.0
    pub battery_score: f64,        // 0.0 - 1.0
    pub network_score: f64,        // 0.0 - 1.0
    pub security_score: f64,       // 0.0 - 1.0
    pub reliability_score: f64,    // 0.0 - 1.0
    pub overall_score: f64,        // 0.0 - 1.0
    pub supported_features: Vec<DeviceFeature>,
    pub limitations: Vec<DeviceLimitation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceFeature {
    ZKProofGeneration,
    SecureEnclave,
    Biometrics,
    HighSpeedNetwork,
    LowPowerMode,
    OfflineCapability,
    EdgeComputing,
    SensorData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceLimitation {
    LimitedBattery,
    SlowNetwork,
    LowMemory,
    NoSecureStorage,
    InternetDependency,
    ProcessingConstraints,
}

/// BPI integration status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BPIIntegration {
    pub wallet_verified: bool,
    pub domain_registered: bool,
    pub enc_cluster_connected: bool,
    pub consensus_participating: bool,
    pub audit_compliant: bool,
    pub last_sync: chrono::DateTime<chrono::Utc>,
}

/// Authentication token for devices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: String,
    pub device_id: Uuid,
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub permissions: Vec<Permission>,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Permission {
    SubmitProofs,
    ParticipateConsensus,
    AccessRewards,
    SyncData,
    ViewStats,
    ManageProfile,
}

/// Device manager statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceManagerStats {
    pub total_devices: u64,
    pub active_devices: u64,
    pub pending_devices: u64,
    pub suspended_devices: u64,
    pub banned_devices: u64,
    pub average_trust_score: f64,
    pub device_type_distribution: HashMap<String, u64>,
    pub capability_distribution: HashMap<String, f64>,
    pub bpi_integration_rate: f64,
}

impl Default for DeviceManagerStats {
    fn default() -> Self {
        Self {
            total_devices: 0,
            active_devices: 0,
            pending_devices: 0,
            suspended_devices: 0,
            banned_devices: 0,
            average_trust_score: 0.0,
            device_type_distribution: HashMap::new(),
            capability_distribution: HashMap::new(),
            bpi_integration_rate: 0.0,
        }
    }
}

impl DeviceManager {
    /// Create a new device manager
    pub async fn new(config: ZKLockConfig) -> Result<Self> {
        info!("Initializing Device Manager");

        Ok(Self {
            devices: Arc::new(RwLock::new(HashMap::new())),
            auth_tokens: Arc::new(RwLock::new(HashMap::new())),
            capabilities_cache: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(RwLock::new(DeviceManagerStats::default())),
        })
    }

    /// Start the device manager
    pub async fn start(&self) -> Result<()> {
        info!("Starting Device Manager");

        // Start background tasks
        self.start_device_monitoring_task().await;
        self.start_token_cleanup_task().await;
        self.start_stats_update_task().await;
        self.start_bpi_integration_task().await;

        Ok(())
    }

    /// Register a new device
    pub async fn register_device(&self, device_type: DeviceType, wallet_address: String) -> Result<Uuid> {
        let device_id = Uuid::new_v4();
        
        // Assess device capabilities
        let capabilities = self.assess_device_capabilities(&device_type).await;
        
        // Create BPI integration record
        let bpi_integration = BPIIntegration {
            wallet_verified: false,
            domain_registered: false,
            enc_cluster_connected: false,
            consensus_participating: false,
            audit_compliant: false,
            last_sync: chrono::Utc::now(),
        };

        let device = RegisteredDevice {
            device_id,
            device_type: device_type.clone(),
            wallet_address: wallet_address.clone(),
            registration_time: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
            status: DeviceStatus::Pending,
            capabilities: capabilities.clone(),
            trust_score: 1.0, // Start with neutral trust
            violation_count: 0,
            total_uptime_hours: 0.0,
            bpi_integration,
        };

        // Store device
        self.devices.write().await.insert(device_id, device);
        self.capabilities_cache.write().await.insert(device_id, capabilities);

        // Generate authentication token
        let auth_token = self.generate_auth_token(device_id).await?;
        self.auth_tokens.write().await.insert(auth_token.token.clone(), auth_token);

        // Update statistics
        self.update_stats_after_registration(&device_type).await;

        // Initiate BPI integration
        self.initiate_bpi_integration(device_id, wallet_address).await?;

        info!("Registered device {} with ID {}", self.device_type_to_string(&device_type), device_id);
        Ok(device_id)
    }

    /// Authenticate a device using token
    pub async fn authenticate_device(&self, token: &str) -> Result<Uuid> {
        let auth_token = self.auth_tokens.read().await
            .get(token)
            .cloned()
            .context("Invalid authentication token")?;

        // Check if token is expired
        if chrono::Utc::now() > auth_token.expires_at {
            return Err(anyhow::anyhow!("Authentication token expired"));
        }

        // Update last seen
        if let Some(device) = self.devices.write().await.get_mut(&auth_token.device_id) {
            device.last_seen = chrono::Utc::now();
        }

        Ok(auth_token.device_id)
    }

    /// Update device status
    pub async fn update_device_status(&self, device_id: Uuid, status: DeviceStatus) -> Result<()> {
        let mut devices = self.devices.write().await;
        if let Some(device) = devices.get_mut(&device_id) {
            device.status = status;
            device.last_seen = chrono::Utc::now();
            info!("Updated device {} status to {:?}", device_id, device.status);
        } else {
            return Err(anyhow::anyhow!("Device not found"));
        }
        Ok(())
    }

    /// Get device information
    pub async fn get_device(&self, device_id: Uuid) -> Result<RegisteredDevice> {
        self.devices.read().await
            .get(&device_id)
            .cloned()
            .context("Device not found")
    }

    /// Get device capabilities
    pub async fn get_device_capabilities(&self, device_id: Uuid) -> Result<DeviceCapabilities> {
        self.capabilities_cache.read().await
            .get(&device_id)
            .cloned()
            .context("Device capabilities not found")
    }

    /// Update device trust score
    pub async fn update_trust_score(&self, device_id: Uuid, delta: f64) -> Result<()> {
        let mut devices = self.devices.write().await;
        if let Some(device) = devices.get_mut(&device_id) {
            device.trust_score = (device.trust_score + delta).clamp(0.0, 1.0);
            
            // Check if device should be suspended or banned
            if device.trust_score < 0.3 {
                device.status = DeviceStatus::Suspended;
                warn!("Device {} suspended due to low trust score: {}", device_id, device.trust_score);
            } else if device.trust_score < 0.1 {
                device.status = DeviceStatus::Banned;
                warn!("Device {} banned due to very low trust score: {}", device_id, device.trust_score);
            }
        }
        Ok(())
    }

    /// Get device manager statistics
    pub async fn get_stats(&self) -> Result<DeviceManagerStats> {
        Ok(self.stats.read().await.clone())
    }

    /// Assess device capabilities based on device type
    async fn assess_device_capabilities(&self, device_type: &DeviceType) -> DeviceCapabilities {
        match device_type {
            DeviceType::Mobile { capabilities, .. } => {
                let compute_score = if capabilities.ram_mb >= 8192 { 0.9 }
                    else if capabilities.ram_mb >= 4096 { 0.7 }
                    else if capabilities.ram_mb >= 2048 { 0.5 }
                    else { 0.3 };

                let security_score = if capabilities.has_secure_enclave && capabilities.supports_biometrics { 0.9 }
                    else if capabilities.has_secure_enclave { 0.7 }
                    else if capabilities.supports_biometrics { 0.6 }
                    else { 0.4 };

                let network_score = if capabilities.network_types.contains(&crate::NetworkType::FiveG) { 0.9 }
                    else if capabilities.network_types.contains(&crate::NetworkType::FourG) { 0.7 }
                    else { 0.5 };

                let battery_score = 0.6; // Mobile devices have moderate battery
                let reliability_score = 0.8; // Mobile devices are generally reliable

                let overall_score = (compute_score + security_score + network_score + battery_score + reliability_score) / 5.0;

                let mut features = vec![DeviceFeature::ZKProofGeneration];
                if capabilities.has_secure_enclave { features.push(DeviceFeature::SecureEnclave); }
                if capabilities.supports_biometrics { features.push(DeviceFeature::Biometrics); }
                if capabilities.network_types.contains(&crate::NetworkType::FiveG) { 
                    features.push(DeviceFeature::HighSpeedNetwork); 
                }

                let mut limitations = Vec::new();
                if capabilities.ram_mb < 4096 { limitations.push(DeviceLimitation::LowMemory); }

                DeviceCapabilities {
                    compute_score,
                    battery_score,
                    network_score,
                    security_score,
                    reliability_score,
                    overall_score,
                    supported_features: features,
                    limitations,
                }
            },
            DeviceType::IoT { compute_level, .. } => {
                let compute_score = match compute_level {
                    ComputeLevel::Minimal => 0.2,
                    ComputeLevel::Light => 0.4,
                    ComputeLevel::Standard => 0.6,
                    ComputeLevel::Enhanced => 0.8,
                };

                let battery_score = 0.3; // IoT devices typically have limited battery
                let network_score = 0.5; // IoT networks are often slower
                let security_score = 0.4; // IoT devices often have limited security
                let reliability_score = 0.7; // IoT devices can be quite reliable

                let overall_score = (compute_score + battery_score + network_score + security_score + reliability_score) / 5.0;

                let mut features = vec![DeviceFeature::SensorData, DeviceFeature::LowPowerMode];
                if matches!(compute_level, ComputeLevel::Enhanced) {
                    features.push(DeviceFeature::EdgeComputing);
                }

                let limitations = vec![
                    DeviceLimitation::LimitedBattery,
                    DeviceLimitation::SlowNetwork,
                    DeviceLimitation::ProcessingConstraints,
                ];

                DeviceCapabilities {
                    compute_score,
                    battery_score,
                    network_score,
                    security_score,
                    reliability_score,
                    overall_score,
                    supported_features: features,
                    limitations,
                }
            },
            DeviceType::Edge { processing_power, .. } => {
                let compute_score = match processing_power {
                    crate::ProcessingPower::Low => 0.5,
                    crate::ProcessingPower::Medium => 0.7,
                    crate::ProcessingPower::High => 0.9,
                    crate::ProcessingPower::Enterprise => 1.0,
                };

                let battery_score = 0.8; // Edge devices typically have good power
                let network_score = 0.9; // Edge devices have excellent connectivity
                let security_score = 0.8; // Edge devices often have good security
                let reliability_score = 0.9; // Edge devices are very reliable

                let overall_score = (compute_score + battery_score + network_score + security_score + reliability_score) / 5.0;

                let features = vec![
                    DeviceFeature::ZKProofGeneration,
                    DeviceFeature::EdgeComputing,
                    DeviceFeature::HighSpeedNetwork,
                    DeviceFeature::OfflineCapability,
                ];

                DeviceCapabilities {
                    compute_score,
                    battery_score,
                    network_score,
                    security_score,
                    reliability_score,
                    overall_score,
                    supported_features: features,
                    limitations: Vec::new(),
                }
            },
            DeviceType::Wearable { battery_class, .. } => {
                let battery_score = match battery_class {
                    crate::BatteryClass::UltraLow => 0.1,
                    crate::BatteryClass::Low => 0.3,
                    crate::BatteryClass::Standard => 0.5,
                    crate::BatteryClass::High => 0.7,
                };

                let compute_score = 0.3; // Wearables have limited compute
                let network_score = 0.4; // Wearables often rely on phone connectivity
                let security_score = 0.5; // Wearables have moderate security
                let reliability_score = 0.6; // Wearables can be reliable but limited

                let overall_score = (compute_score + battery_score + network_score + security_score + reliability_score) / 5.0;

                let features = vec![DeviceFeature::LowPowerMode, DeviceFeature::SensorData];

                let limitations = vec![
                    DeviceLimitation::LimitedBattery,
                    DeviceLimitation::LowMemory,
                    DeviceLimitation::ProcessingConstraints,
                    DeviceLimitation::InternetDependency,
                ];

                DeviceCapabilities {
                    compute_score,
                    battery_score,
                    network_score,
                    security_score,
                    reliability_score,
                    overall_score,
                    supported_features: features,
                    limitations,
                }
            },
        }
    }

    /// Generate authentication token
    async fn generate_auth_token(&self, device_id: Uuid) -> Result<AuthToken> {
        let token = Uuid::new_v4().to_string();
        let refresh_token = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        let expires_at = now + chrono::Duration::hours(24); // 24-hour token

        let permissions = vec![
            Permission::SubmitProofs,
            Permission::ParticipateConsensus,
            Permission::AccessRewards,
            Permission::SyncData,
            Permission::ViewStats,
        ];

        Ok(AuthToken {
            token,
            device_id,
            issued_at: now,
            expires_at,
            permissions,
            refresh_token,
        })
    }

    /// Initiate BPI integration for device
    async fn initiate_bpi_integration(&self, device_id: Uuid, wallet_address: String) -> Result<()> {
        // In a real implementation, this would:
        // 1. Verify wallet with BPI wallet system
        // 2. Register domain with domain resolver
        // 3. Connect to ENC cluster
        // 4. Set up consensus participation
        // 5. Ensure audit compliance

        info!("Initiating BPI integration for device {} with wallet {}", device_id, wallet_address);
        
        // For now, we'll simulate successful integration
        if let Some(device) = self.devices.write().await.get_mut(&device_id) {
            device.bpi_integration.wallet_verified = true;
            device.status = DeviceStatus::Active;
        }

        Ok(())
    }

    /// Convert device type to string
    fn device_type_to_string(&self, device_type: &DeviceType) -> String {
        match device_type {
            DeviceType::Mobile { .. } => "Mobile".to_string(),
            DeviceType::IoT { .. } => "IoT".to_string(),
            DeviceType::Edge { .. } => "Edge".to_string(),
            DeviceType::Wearable { .. } => "Wearable".to_string(),
        }
    }

    /// Update statistics after device registration
    async fn update_stats_after_registration(&self, device_type: &DeviceType) {
        let mut stats = self.stats.write().await;
        stats.total_devices += 1;
        stats.pending_devices += 1;
        
        let device_type_str = self.device_type_to_string(device_type);
        *stats.device_type_distribution.entry(device_type_str).or_insert(0) += 1;
    }

    /// Start device monitoring background task
    async fn start_device_monitoring_task(&self) {
        let devices = Arc::clone(&self.devices);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60)); // 1 minute
            
            loop {
                interval.tick().await;
                
                // Check for offline devices (not seen in 10 minutes)
                let cutoff = chrono::Utc::now() - chrono::Duration::minutes(10);
                let mut devices = devices.write().await;
                
                for (device_id, device) in devices.iter_mut() {
                    if device.last_seen < cutoff && device.status == DeviceStatus::Active {
                        device.status = DeviceStatus::Offline;
                        warn!("Device {} marked as offline", device_id);
                    }
                }
            }
        });
    }

    /// Start token cleanup background task
    async fn start_token_cleanup_task(&self) {
        let auth_tokens = Arc::clone(&self.auth_tokens);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600)); // 1 hour
            
            loop {
                interval.tick().await;
                
                // Remove expired tokens
                let now = chrono::Utc::now();
                let mut tokens = auth_tokens.write().await;
                let initial_count = tokens.len();
                
                tokens.retain(|_, token| token.expires_at > now);
                
                let cleaned = initial_count - tokens.len();
                if cleaned > 0 {
                    debug!("Cleaned up {} expired auth tokens", cleaned);
                }
            }
        });
    }

    /// Start statistics update background task
    async fn start_stats_update_task(&self) {
        let devices = Arc::clone(&self.devices);
        let stats = Arc::clone(&self.stats);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // 5 minutes
            
            loop {
                interval.tick().await;
                
                let devices = devices.read().await;
                let mut stats = stats.write().await;
                
                // Update device status counts
                stats.active_devices = devices.values().filter(|d| d.status == DeviceStatus::Active).count() as u64;
                stats.pending_devices = devices.values().filter(|d| d.status == DeviceStatus::Pending).count() as u64;
                stats.suspended_devices = devices.values().filter(|d| d.status == DeviceStatus::Suspended).count() as u64;
                stats.banned_devices = devices.values().filter(|d| d.status == DeviceStatus::Banned).count() as u64;
                
                // Update average trust score
                let total_trust: f64 = devices.values().map(|d| d.trust_score).sum();
                stats.average_trust_score = if devices.len() > 0 {
                    total_trust / devices.len() as f64
                } else {
                    0.0
                };
                
                // Update BPI integration rate
                let integrated_devices = devices.values()
                    .filter(|d| d.bpi_integration.wallet_verified)
                    .count() as u64;
                stats.bpi_integration_rate = if stats.total_devices > 0 {
                    integrated_devices as f64 / stats.total_devices as f64
                } else {
                    0.0
                };
                
                debug!("Updated device manager stats: {} total, {} active, {:.2} avg trust", 
                       stats.total_devices, stats.active_devices, stats.average_trust_score);
            }
        });
    }

    /// Start BPI integration background task
    async fn start_bpi_integration_task(&self) {
        let devices = Arc::clone(&self.devices);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1800)); // 30 minutes
            
            loop {
                interval.tick().await;
                
                // Check and update BPI integration status for all devices
                let mut devices = devices.write().await;
                for (device_id, device) in devices.iter_mut() {
                    if device.status == DeviceStatus::Active {
                        // Simulate BPI integration progress
                        if !device.bpi_integration.domain_registered {
                            device.bpi_integration.domain_registered = true;
                            debug!("Domain registered for device {}", device_id);
                        }
                        
                        if !device.bpi_integration.enc_cluster_connected {
                            device.bpi_integration.enc_cluster_connected = true;
                            debug!("ENC cluster connected for device {}", device_id);
                        }
                        
                        device.bpi_integration.last_sync = chrono::Utc::now();
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MobilePlatform, MobileCapabilities, NetworkType};

    #[tokio::test]
    async fn test_device_manager_creation() {
        let config = ZKLockConfig::default();
        let manager = DeviceManager::new(config).await.unwrap();
        
        let stats = manager.get_stats().await.unwrap();
        assert_eq!(stats.total_devices, 0);
    }

    #[tokio::test]
    async fn test_device_registration() {
        let config = ZKLockConfig::default();
        let manager = DeviceManager::new(config).await.unwrap();
        
        let device_type = DeviceType::Mobile {
            platform: MobilePlatform::Android,
            capabilities: MobileCapabilities {
                ram_mb: 4096,
                storage_gb: 64,
                has_secure_enclave: true,
                supports_biometrics: true,
                network_types: vec![NetworkType::FiveG],
            },
        };

        let device_id = manager.register_device(device_type, "wallet123".to_string()).await.unwrap();
        assert!(!device_id.is_nil());
        
        let device = manager.get_device(device_id).await.unwrap();
        assert_eq!(device.device_id, device_id);
        assert_eq!(device.wallet_address, "wallet123");
        
        let capabilities = manager.get_device_capabilities(device_id).await.unwrap();
        assert!(capabilities.overall_score > 0.0);
    }
}
