//! ICO Participation System - Token distribution for device participation
//!
//! This module implements an ICO-like token distribution system that rewards
//! mobile and IoT devices for participating in the zklock ecosystem.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::{ICOConfig, DeviceType, ComputeLevel};

/// ICO participation system for device rewards
#[derive(Debug)]
pub struct ICOParticipation {
    /// Token pools for different device types
    token_pools: Arc<RwLock<HashMap<String, TokenPool>>>,
    /// Device participation records
    participation_records: Arc<RwLock<HashMap<Uuid, ParticipationRecord>>>,
    /// Token distribution history
    distribution_history: Arc<RwLock<Vec<TokenDistribution>>>,
    /// Configuration
    config: ICOConfig,
    /// ICO statistics
    stats: Arc<RwLock<ICOStats>>,
}

/// Token pool for specific device types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPool {
    pub device_type: String,
    pub total_tokens: u64,
    pub distributed_tokens: u64,
    pub remaining_tokens: u64,
    pub base_reward: u64,
    pub multiplier: f64,
    pub min_participation_hours: u64,
    pub bonus_conditions: Vec<BonusCondition>,
}

/// Bonus conditions for additional token rewards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BonusCondition {
    pub condition_type: BonusType,
    pub multiplier: f64,
    pub description: String,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BonusType {
    EarlyAdopter,        // First 1000 devices
    HighUptime,          // >95% uptime
    ProofGeneration,     // Generate >100 proofs/day
    NetworkStability,    // Stable network connection
    BatteryEfficiency,   // Low battery usage
    GeographicDiversity, // From underrepresented regions
    DeviceCapability,    // High-end device capabilities
}

/// Device participation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipationRecord {
    pub device_id: Uuid,
    pub device_type: DeviceType,
    pub wallet_address: String,
    pub registration_time: chrono::DateTime<chrono::Utc>,
    pub total_participation_hours: f64,
    pub total_tokens_earned: u64,
    pub total_proofs_generated: u64,
    pub uptime_percentage: f64,
    pub reputation_score: f64,
    pub bonus_multipliers: Vec<f64>,
    pub last_reward_time: chrono::DateTime<chrono::Utc>,
    pub status: ParticipationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParticipationStatus {
    Active,
    Suspended,
    Graduated,  // Moved to full BPI participation
    Banned,
}

/// Token distribution event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenDistribution {
    pub distribution_id: String,
    pub device_id: Uuid,
    pub device_type: String,
    pub tokens_awarded: u64,
    pub base_reward: u64,
    pub bonus_tokens: u64,
    pub reason: DistributionReason,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub transaction_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistributionReason {
    ProofSubmission,
    ConsensusParticipation,
    UptimeBonus,
    EarlyAdopterBonus,
    PerformanceBonus,
    MilestoneReward,
}

/// ICO statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ICOStats {
    pub total_participants: u64,
    pub active_participants: u64,
    pub total_tokens_distributed: u64,
    pub participation_rate: f64,
    pub average_tokens_per_device: f64,
    pub device_type_distribution: HashMap<String, u64>,
    pub geographic_distribution: HashMap<String, u64>,
    pub bonus_activation_rate: f64,
}

impl Default for ICOStats {
    fn default() -> Self {
        Self {
            total_participants: 0,
            active_participants: 0,
            total_tokens_distributed: 0,
            participation_rate: 0.0,
            average_tokens_per_device: 0.0,
            device_type_distribution: HashMap::new(),
            geographic_distribution: HashMap::new(),
            bonus_activation_rate: 0.0,
        }
    }
}

impl ICOParticipation {
    /// Create a new ICO participation system
    pub async fn new(config: ICOConfig) -> Result<Self> {
        info!("Initializing ICO Participation System");

        let mut token_pools = HashMap::new();
        
        // Initialize token pools for different device types
        for (device_type, multiplier) in &config.device_type_multipliers {
            let pool = TokenPool {
                device_type: device_type.clone(),
                total_tokens: 1_000_000, // 1M tokens per device type
                distributed_tokens: 0,
                remaining_tokens: 1_000_000,
                base_reward: config.base_reward_tokens,
                multiplier: *multiplier,
                min_participation_hours: config.min_participation_hours,
                bonus_conditions: Self::create_bonus_conditions(),
            };
            token_pools.insert(device_type.clone(), pool);
        }

        Ok(Self {
            token_pools: Arc::new(RwLock::new(token_pools)),
            participation_records: Arc::new(RwLock::new(HashMap::new())),
            distribution_history: Arc::new(RwLock::new(Vec::new())),
            config,
            stats: Arc::new(RwLock::new(ICOStats::default())),
        })
    }

    /// Start the ICO participation system
    pub async fn start(&self) -> Result<()> {
        info!("Starting ICO Participation System");

        // Start background tasks
        self.start_reward_distribution_task().await;
        self.start_bonus_evaluation_task().await;
        self.start_stats_update_task().await;

        Ok(())
    }

    /// Register a device for ICO participation
    pub async fn register_device(&self, device_id: Uuid, device_type: DeviceType, wallet_address: String) -> Result<()> {
        let device_type_str = self.device_type_to_string(&device_type);
        
        let record = ParticipationRecord {
            device_id,
            device_type,
            wallet_address,
            registration_time: chrono::Utc::now(),
            total_participation_hours: 0.0,
            total_tokens_earned: 0,
            total_proofs_generated: 0,
            uptime_percentage: 100.0,
            reputation_score: 1.0,
            bonus_multipliers: vec![1.0], // Base multiplier
            last_reward_time: chrono::Utc::now(),
            status: ParticipationStatus::Active,
        };

        self.participation_records.write().await.insert(device_id, record);

        // Update stats
        self.update_stats_after_registration(&device_type_str).await;

        info!("Registered device {} for ICO participation", device_id);
        Ok(())
    }

    /// Award tokens to a device
    pub async fn award_tokens(&self, device_id: Uuid, device_type: &DeviceType) -> Result<u64> {
        let device_type_str = self.device_type_to_string(device_type);
        
        // Get base reward and multiplier
        let (base_reward, multiplier) = {
            let pools = self.token_pools.read().await;
            let pool = pools.get(&device_type_str)
                .context("Device type not found in token pools")?;
            (pool.base_reward, pool.multiplier)
        };

        // Calculate bonus multipliers
        let bonus_multiplier = self.calculate_bonus_multiplier(device_id, device_type).await?;
        
        // Calculate total tokens
        let total_tokens = ((base_reward as f64) * multiplier * bonus_multiplier) as u64;

        // Update participation record
        {
            let mut records = self.participation_records.write().await;
            if let Some(record) = records.get_mut(&device_id) {
                record.total_tokens_earned += total_tokens;
                record.last_reward_time = chrono::Utc::now();
                record.total_proofs_generated += 1;
            }
        }

        // Update token pool
        {
            let mut pools = self.token_pools.write().await;
            if let Some(pool) = pools.get_mut(&device_type_str) {
                if pool.remaining_tokens >= total_tokens {
                    pool.distributed_tokens += total_tokens;
                    pool.remaining_tokens -= total_tokens;
                } else {
                    warn!("Insufficient tokens in pool for device type: {}", device_type_str);
                    return Ok(0);
                }
            }
        }

        // Record distribution
        let distribution = TokenDistribution {
            distribution_id: Uuid::new_v4().to_string(),
            device_id,
            device_type: device_type_str,
            tokens_awarded: total_tokens,
            base_reward,
            bonus_tokens: total_tokens.saturating_sub(base_reward),
            reason: DistributionReason::ProofSubmission,
            timestamp: chrono::Utc::now(),
            transaction_hash: None, // Would be set after blockchain transaction
        };

        self.distribution_history.write().await.push(distribution);

        // Update stats
        self.update_stats_after_distribution(total_tokens).await;

        info!("Awarded {} tokens to device {}", total_tokens, device_id);
        Ok(total_tokens)
    }

    /// Get device participation record
    pub async fn get_participation_record(&self, device_id: Uuid) -> Result<ParticipationRecord> {
        self.participation_records.read().await
            .get(&device_id)
            .cloned()
            .context("Device participation record not found")
    }

    /// Get token pool status
    pub async fn get_token_pool_status(&self, device_type: &str) -> Result<TokenPool> {
        self.token_pools.read().await
            .get(device_type)
            .cloned()
            .context("Token pool not found")
    }

    /// Get ICO statistics
    pub async fn get_stats(&self) -> Result<ICOStats> {
        Ok(self.stats.read().await.clone())
    }

    /// Calculate bonus multiplier for a device
    async fn calculate_bonus_multiplier(&self, device_id: Uuid, device_type: &DeviceType) -> Result<f64> {
        let record = self.participation_records.read().await
            .get(&device_id)
            .cloned()
            .context("Device record not found")?;

        let mut total_multiplier = 1.0;

        // Early adopter bonus (first 1000 devices)
        if record.total_tokens_earned == 0 {
            let total_participants = self.stats.read().await.total_participants;
            if total_participants < 1000 {
                total_multiplier *= 1.5; // 50% bonus for early adopters
            }
        }

        // High uptime bonus
        if record.uptime_percentage > 95.0 {
            total_multiplier *= 1.2; // 20% bonus for high uptime
        }

        // Proof generation bonus
        if record.total_proofs_generated > 100 {
            total_multiplier *= 1.1; // 10% bonus for active proof generation
        }

        // Device capability bonus
        match device_type {
            DeviceType::Mobile { capabilities, .. } => {
                if capabilities.ram_mb >= 8192 && capabilities.has_secure_enclave {
                    total_multiplier *= 1.3; // 30% bonus for high-end mobile devices
                }
            },
            DeviceType::Edge { processing_power, .. } => {
                if matches!(processing_power, crate::ProcessingPower::High | crate::ProcessingPower::Enterprise) {
                    total_multiplier *= 1.4; // 40% bonus for high-performance edge devices
                }
            },
            DeviceType::IoT { compute_level, .. } => {
                if matches!(compute_level, ComputeLevel::Enhanced) {
                    total_multiplier *= 1.2; // 20% bonus for enhanced IoT devices
                }
            },
            DeviceType::Wearable { .. } => {
                total_multiplier *= 0.8; // Wearables get reduced rewards due to limited capability
            },
        }

        Ok(total_multiplier)
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

    /// Create bonus conditions
    fn create_bonus_conditions() -> Vec<BonusCondition> {
        vec![
            BonusCondition {
                condition_type: BonusType::EarlyAdopter,
                multiplier: 1.5,
                description: "First 1000 devices to join".to_string(),
                active: true,
            },
            BonusCondition {
                condition_type: BonusType::HighUptime,
                multiplier: 1.2,
                description: "Maintain >95% uptime".to_string(),
                active: true,
            },
            BonusCondition {
                condition_type: BonusType::ProofGeneration,
                multiplier: 1.1,
                description: "Generate >100 proofs".to_string(),
                active: true,
            },
            BonusCondition {
                condition_type: BonusType::DeviceCapability,
                multiplier: 1.3,
                description: "High-end device capabilities".to_string(),
                active: true,
            },
        ]
    }

    /// Update stats after device registration
    async fn update_stats_after_registration(&self, device_type: &str) {
        let mut stats = self.stats.write().await;
        stats.total_participants += 1;
        stats.active_participants += 1;
        
        *stats.device_type_distribution.entry(device_type.to_string()).or_insert(0) += 1;
        
        // Update participation rate
        stats.participation_rate = if stats.total_participants > 0 {
            stats.active_participants as f64 / stats.total_participants as f64
        } else {
            0.0
        };
    }

    /// Update stats after token distribution
    async fn update_stats_after_distribution(&self, tokens_awarded: u64) {
        let mut stats = self.stats.write().await;
        stats.total_tokens_distributed += tokens_awarded;
        
        // Update average tokens per device
        stats.average_tokens_per_device = if stats.total_participants > 0 {
            stats.total_tokens_distributed as f64 / stats.total_participants as f64
        } else {
            0.0
        };
    }

    /// Start reward distribution background task
    async fn start_reward_distribution_task(&self) {
        let participation_records = Arc::clone(&self.participation_records);
        let token_pools = Arc::clone(&self.token_pools);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600)); // Hourly rewards
            
            loop {
                interval.tick().await;
                
                // Process hourly participation rewards
                let records = participation_records.read().await;
                for (device_id, record) in records.iter() {
                    if record.status == ParticipationStatus::Active {
                        // Award hourly participation tokens (simplified)
                        debug!("Processing hourly reward for device {}", device_id);
                    }
                }
            }
        });
    }

    /// Start bonus evaluation background task
    async fn start_bonus_evaluation_task(&self) {
        let participation_records = Arc::clone(&self.participation_records);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(86400)); // Daily bonus evaluation
            
            loop {
                interval.tick().await;
                
                // Evaluate bonus conditions for all devices
                let mut records = participation_records.write().await;
                for (device_id, record) in records.iter_mut() {
                    if record.status == ParticipationStatus::Active {
                        // Update reputation score based on performance
                        // This is a simplified calculation
                        record.reputation_score = (record.uptime_percentage / 100.0) * 
                                                 (record.total_proofs_generated as f64 / 1000.0).min(1.0);
                        
                        debug!("Updated reputation score for device {}: {}", device_id, record.reputation_score);
                    }
                }
            }
        });
    }

    /// Start statistics update background task
    async fn start_stats_update_task(&self) {
        let stats = Arc::clone(&self.stats);
        let participation_records = Arc::clone(&self.participation_records);
        let token_pools = Arc::clone(&self.token_pools);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // 5-minute stats update
            
            loop {
                interval.tick().await;
                
                let records = participation_records.read().await;
                let pools = token_pools.read().await;
                
                let mut stats = stats.write().await;
                
                // Update active participants
                stats.active_participants = records.values()
                    .filter(|r| r.status == ParticipationStatus::Active)
                    .count() as u64;
                
                // Update device type distribution
                stats.device_type_distribution.clear();
                for record in records.values() {
                    let device_type = match record.device_type {
                        DeviceType::Mobile { .. } => "Mobile",
                        DeviceType::IoT { .. } => "IoT",
                        DeviceType::Edge { .. } => "Edge",
                        DeviceType::Wearable { .. } => "Wearable",
                    };
                    *stats.device_type_distribution.entry(device_type.to_string()).or_insert(0) += 1;
                }
                
                // Update total tokens distributed
                stats.total_tokens_distributed = pools.values()
                    .map(|p| p.distributed_tokens)
                    .sum();
                
                debug!("Updated ICO stats: {} participants, {} tokens distributed", 
                       stats.total_participants, stats.total_tokens_distributed);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MobilePlatform, MobileCapabilities, NetworkType};

    #[tokio::test]
    async fn test_ico_participation_creation() {
        let mut device_multipliers = HashMap::new();
        device_multipliers.insert("Mobile".to_string(), 1.0);
        device_multipliers.insert("IoT".to_string(), 0.5);
        
        let config = ICOConfig {
            base_reward_tokens: 100,
            device_type_multipliers: device_multipliers,
            min_participation_hours: 1,
        };
        
        let ico = ICOParticipation::new(config).await.unwrap();
        let stats = ico.get_stats().await.unwrap();
        assert_eq!(stats.total_participants, 0);
    }

    #[tokio::test]
    async fn test_device_registration_and_token_award() {
        let mut device_multipliers = HashMap::new();
        device_multipliers.insert("Mobile".to_string(), 1.0);
        
        let config = ICOConfig {
            base_reward_tokens: 100,
            device_type_multipliers: device_multipliers,
            min_participation_hours: 1,
        };
        
        let ico = ICOParticipation::new(config).await.unwrap();
        let device_id = Uuid::new_v4();
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

        // Register device
        ico.register_device(device_id, device_type.clone(), "wallet123".to_string()).await.unwrap();
        
        // Award tokens
        let tokens_awarded = ico.award_tokens(device_id, &device_type).await.unwrap();
        assert!(tokens_awarded > 0);
        
        // Check participation record
        let record = ico.get_participation_record(device_id).await.unwrap();
        assert_eq!(record.total_tokens_earned, tokens_awarded);
    }
}
