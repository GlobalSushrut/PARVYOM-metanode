//! Light Consensus Protocol - Minimal participation for mobile devices
//!
//! This module implements a lightweight consensus protocol that allows
//! mobile and IoT devices to participate in consensus with minimal
//! computational and network requirements.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::{ZKLockConfig, DeviceType, ComputeLevel};

/// Light consensus protocol for mobile devices
#[derive(Debug)]
pub struct LightConsensus {
    /// Active validators (mobile devices)
    validators: Arc<RwLock<HashMap<Uuid, LightValidator>>>,
    /// Consensus rounds
    rounds: Arc<RwLock<VecDeque<ConsensusRound>>>,
    /// Current round
    current_round: Arc<RwLock<u64>>,
    /// Configuration
    config: ZKLockConfig,
    /// Consensus statistics
    stats: Arc<RwLock<ConsensusStats>>,
}

/// Light validator for mobile devices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightValidator {
    pub device_id: Uuid,
    pub device_type: DeviceType,
    pub wallet_address: String,
    pub stake_amount: u64,
    pub reputation_score: f64,
    pub last_participation: chrono::DateTime<chrono::Utc>,
    pub participation_rate: f64,
    pub status: ValidatorStatus,
    pub compute_capability: ComputeCapability,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidatorStatus {
    Active,
    Idle,
    Slashed,
    Offline,
}

/// Compute capability assessment for mobile devices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeCapability {
    pub compute_level: ComputeLevel,
    pub battery_level: BatteryLevel,
    pub network_quality: NetworkQuality,
    pub processing_speed: ProcessingSpeed,
    pub memory_available: u64, // MB
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BatteryLevel {
    Critical,  // <20%
    Low,       // 20-50%
    Medium,    // 50-80%
    High,      // >80%
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NetworkQuality {
    Poor,      // <1 Mbps
    Fair,      // 1-10 Mbps
    Good,      // 10-50 Mbps
    Excellent, // >50 Mbps
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProcessingSpeed {
    Slow,      // <1 GHz
    Medium,    // 1-2 GHz
    Fast,      // 2-3 GHz
    VeryFast,  // >3 GHz
}

/// Consensus round for light protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusRound {
    pub round_number: u64,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub proposer: Uuid,
    pub participants: Vec<Uuid>,
    pub proposals: Vec<LightProposal>,
    pub votes: HashMap<Uuid, Vote>,
    pub status: RoundStatus,
    pub finalized_hash: Option<[u8; 32]>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoundStatus {
    Proposing,
    Voting,
    Finalizing,
    Completed,
    Failed,
}

/// Light proposal optimized for mobile devices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightProposal {
    pub proposal_id: String,
    pub proposer: Uuid,
    pub proposal_type: ProposalType,
    pub data_hash: [u8; 32],
    pub data_size: usize,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub mobile_optimized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalType {
    StateUpdate,
    DeviceRegistration,
    TokenDistribution,
    SystemUpgrade,
    EmergencyAction,
}

/// Vote from mobile device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub voter: Uuid,
    pub proposal_id: String,
    pub vote_type: VoteType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub signature: Vec<u8>,
    pub battery_cost: f64, // Battery percentage used for voting
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VoteType {
    Approve,
    Reject,
    Abstain,
}

/// Consensus statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusStats {
    pub total_rounds: u64,
    pub successful_rounds: u64,
    pub failed_rounds: u64,
    pub average_participation_rate: f64,
    pub average_round_time_ms: f64,
    pub mobile_participation_rate: f64,
    pub iot_participation_rate: f64,
    pub battery_efficiency_score: f64,
}

impl Default for ConsensusStats {
    fn default() -> Self {
        Self {
            total_rounds: 0,
            successful_rounds: 0,
            failed_rounds: 0,
            average_participation_rate: 0.0,
            average_round_time_ms: 0.0,
            mobile_participation_rate: 0.0,
            iot_participation_rate: 0.0,
            battery_efficiency_score: 0.0,
        }
    }
}

impl LightConsensus {
    /// Create a new light consensus protocol
    pub async fn new(config: ZKLockConfig) -> Result<Self> {
        info!("Initializing Light Consensus Protocol");

        Ok(Self {
            validators: Arc::new(RwLock::new(HashMap::new())),
            rounds: Arc::new(RwLock::new(VecDeque::new())),
            current_round: Arc::new(RwLock::new(0)),
            config,
            stats: Arc::new(RwLock::new(ConsensusStats::default())),
        })
    }

    /// Start the consensus protocol
    pub async fn start(&self) -> Result<()> {
        info!("Starting Light Consensus Protocol");

        // Start consensus rounds
        self.start_consensus_rounds().await;
        
        // Start validator monitoring
        self.start_validator_monitoring().await;

        Ok(())
    }

    /// Register a device as a light validator
    pub async fn register_validator(&self, device_id: Uuid, device_type: DeviceType, wallet_address: String) -> Result<()> {
        let compute_capability = self.assess_compute_capability(&device_type).await;
        
        let validator = LightValidator {
            device_id,
            device_type,
            wallet_address,
            stake_amount: 1000, // Base stake for mobile devices
            reputation_score: 1.0,
            last_participation: chrono::Utc::now(),
            participation_rate: 0.0,
            status: ValidatorStatus::Active,
            compute_capability,
        };

        self.validators.write().await.insert(device_id, validator);
        info!("Registered light validator: {}", device_id);
        Ok(())
    }

    /// Submit a proposal (mobile-optimized)
    pub async fn submit_proposal(&self, proposer: Uuid, proposal_type: ProposalType, data: Vec<u8>) -> Result<String> {
        let proposal_id = Uuid::new_v4().to_string();
        let data_hash = self.hash_data(&data);
        
        let proposal = LightProposal {
            proposal_id: proposal_id.clone(),
            proposer,
            proposal_type,
            data_hash,
            data_size: data.len(),
            timestamp: chrono::Utc::now(),
            mobile_optimized: data.len() <= self.config.zk_config.max_proof_size,
        };

        // Add to current round
        let current_round_num = *self.current_round.read().await;
        let mut rounds = self.rounds.write().await;
        
        if let Some(current_round) = rounds.back_mut() {
            if current_round.round_number == current_round_num && current_round.status == RoundStatus::Proposing {
                current_round.proposals.push(proposal);
                info!("Added proposal {} to round {}", proposal_id, current_round_num);
                return Ok(proposal_id);
            }
        }

        Err(anyhow::anyhow!("No active round for proposals"))
    }

    /// Submit a vote (battery-optimized)
    pub async fn submit_vote(&self, voter: Uuid, proposal_id: String, vote_type: VoteType) -> Result<()> {
        // Check if voter is a registered validator
        let validator = self.validators.read().await.get(&voter).cloned()
            .context("Voter is not a registered validator")?;

        if validator.status != ValidatorStatus::Active {
            return Err(anyhow::anyhow!("Validator is not active"));
        }

        // Calculate battery cost based on device type
        let battery_cost = self.calculate_battery_cost(&validator.device_type, &vote_type).await;

        // Create vote
        let vote = Vote {
            voter,
            proposal_id: proposal_id.clone(),
            vote_type,
            timestamp: chrono::Utc::now(),
            signature: self.sign_vote(&voter, &proposal_id).await?,
            battery_cost,
        };

        // Add vote to current round
        let current_round_num = *self.current_round.read().await;
        let mut rounds = self.rounds.write().await;
        
        if let Some(current_round) = rounds.back_mut() {
            if current_round.round_number == current_round_num && current_round.status == RoundStatus::Voting {
                current_round.votes.insert(voter, vote);
                
                // Update validator participation
                self.update_validator_participation(voter).await;
                
                info!("Added vote from {} for proposal {}", voter, proposal_id);
                return Ok(());
            }
        }

        Err(anyhow::anyhow!("No active round for voting"))
    }

    /// Get consensus statistics
    pub async fn get_stats(&self) -> Result<ConsensusStats> {
        Ok(self.stats.read().await.clone())
    }

    /// Assess compute capability of a device
    async fn assess_compute_capability(&self, device_type: &DeviceType) -> ComputeCapability {
        match device_type {
            DeviceType::Mobile { capabilities, .. } => {
                let compute_level = if capabilities.ram_mb >= 8192 {
                    ComputeLevel::Enhanced
                } else if capabilities.ram_mb >= 4096 {
                    ComputeLevel::Standard
                } else if capabilities.ram_mb >= 2048 {
                    ComputeLevel::Light
                } else {
                    ComputeLevel::Minimal
                };

                ComputeCapability {
                    compute_level,
                    battery_level: BatteryLevel::Medium, // Default assumption
                    network_quality: NetworkQuality::Good, // Default assumption
                    processing_speed: ProcessingSpeed::Medium, // Default assumption
                    memory_available: capabilities.ram_mb as u64,
                }
            },
            DeviceType::IoT { compute_level, .. } => {
                ComputeCapability {
                    compute_level: compute_level.clone(),
                    battery_level: BatteryLevel::Low, // IoT devices typically have limited battery
                    network_quality: NetworkQuality::Fair, // IoT networks are often slower
                    processing_speed: ProcessingSpeed::Slow, // IoT devices are typically slower
                    memory_available: match compute_level {
                        ComputeLevel::Minimal => 1,
                        ComputeLevel::Light => 10,
                        ComputeLevel::Standard => 100,
                        ComputeLevel::Enhanced => 1000,
                    },
                }
            },
            DeviceType::Edge { processing_power, .. } => {
                let (compute_level, processing_speed, memory) = match processing_power {
                    crate::ProcessingPower::Low => (ComputeLevel::Light, ProcessingSpeed::Medium, 512),
                    crate::ProcessingPower::Medium => (ComputeLevel::Standard, ProcessingSpeed::Fast, 2048),
                    crate::ProcessingPower::High => (ComputeLevel::Enhanced, ProcessingSpeed::VeryFast, 8192),
                    crate::ProcessingPower::Enterprise => (ComputeLevel::Enhanced, ProcessingSpeed::VeryFast, 16384),
                };

                ComputeCapability {
                    compute_level,
                    battery_level: BatteryLevel::High, // Edge devices typically have good power
                    network_quality: NetworkQuality::Excellent, // Edge devices have good connectivity
                    processing_speed,
                    memory_available: memory,
                }
            },
            DeviceType::Wearable { battery_class, .. } => {
                let battery_level = match battery_class {
                    crate::BatteryClass::UltraLow => BatteryLevel::Critical,
                    crate::BatteryClass::Low => BatteryLevel::Low,
                    crate::BatteryClass::Standard => BatteryLevel::Medium,
                    crate::BatteryClass::High => BatteryLevel::High,
                };

                ComputeCapability {
                    compute_level: ComputeLevel::Minimal, // Wearables are typically limited
                    battery_level,
                    network_quality: NetworkQuality::Fair, // Wearables often rely on phone connectivity
                    processing_speed: ProcessingSpeed::Slow, // Wearables prioritize battery life
                    memory_available: 256, // Typical wearable memory
                }
            },
        }
    }

    /// Calculate battery cost for voting
    async fn calculate_battery_cost(&self, device_type: &DeviceType, _vote_type: &VoteType) -> f64 {
        match device_type {
            DeviceType::Mobile { .. } => 0.01, // 0.01% battery per vote
            DeviceType::IoT { .. } => 0.05,    // 0.05% battery per vote (more expensive for IoT)
            DeviceType::Edge { .. } => 0.001,  // 0.001% battery per vote (edge devices have more power)
            DeviceType::Wearable { .. } => 0.1, // 0.1% battery per vote (most expensive for wearables)
        }
    }

    /// Sign a vote (simplified for demo)
    async fn sign_vote(&self, voter: &Uuid, proposal_id: &str) -> Result<Vec<u8>> {
        // In a real implementation, this would use the device's private key
        let data = format!("{}:{}", voter, proposal_id);
        Ok(data.into_bytes())
    }

    /// Update validator participation
    async fn update_validator_participation(&self, validator_id: Uuid) {
        if let Some(validator) = self.validators.write().await.get_mut(&validator_id) {
            validator.last_participation = chrono::Utc::now();
            // Update participation rate (simplified calculation)
            validator.participation_rate = (validator.participation_rate * 0.9) + 0.1;
        }
    }

    /// Hash data for proposals
    fn hash_data(&self, data: &[u8]) -> [u8; 32] {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    /// Start consensus rounds background task
    async fn start_consensus_rounds(&self) {
        let validators = Arc::clone(&self.validators);
        let rounds = Arc::clone(&self.rounds);
        let current_round = Arc::clone(&self.current_round);
        let stats = Arc::clone(&self.stats);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30)); // 30-second rounds
            
            loop {
                interval.tick().await;
                
                let round_num = {
                    let mut current = current_round.write().await;
                    *current += 1;
                    *current
                };

                // Create new consensus round
                let active_validators: Vec<Uuid> = validators.read().await
                    .iter()
                    .filter(|(_, v)| v.status == ValidatorStatus::Active)
                    .map(|(id, _)| *id)
                    .collect();

                if active_validators.is_empty() {
                    continue;
                }

                let proposer = active_validators[0]; // Simple proposer selection
                
                let round = ConsensusRound {
                    round_number: round_num,
                    start_time: chrono::Utc::now(),
                    end_time: None,
                    proposer,
                    participants: active_validators,
                    proposals: Vec::new(),
                    votes: HashMap::new(),
                    status: RoundStatus::Proposing,
                    finalized_hash: None,
                };

                rounds.write().await.push_back(round);
                
                // Keep only last 100 rounds
                while rounds.read().await.len() > 100 {
                    rounds.write().await.pop_front();
                }

                // Update stats
                let mut stats = stats.write().await;
                stats.total_rounds += 1;

                debug!("Started consensus round {}", round_num);
            }
        });
    }

    /// Start validator monitoring background task
    async fn start_validator_monitoring(&self) {
        let validators = Arc::clone(&self.validators);
        let stats = Arc::clone(&self.stats);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60)); // 1-minute monitoring
            
            loop {
                interval.tick().await;
                
                let validators_read = validators.read().await;
                let total_validators = validators_read.len();
                
                if total_validators == 0 {
                    continue;
                }

                let active_validators = validators_read.values()
                    .filter(|v| v.status == ValidatorStatus::Active)
                    .count();

                let mobile_validators = validators_read.values()
                    .filter(|v| matches!(v.device_type, DeviceType::Mobile { .. }))
                    .count();

                let iot_validators = validators_read.values()
                    .filter(|v| matches!(v.device_type, DeviceType::IoT { .. }))
                    .count();

                drop(validators_read);

                // Update stats
                let mut stats = stats.write().await;
                stats.average_participation_rate = if total_validators > 0 {
                    active_validators as f64 / total_validators as f64
                } else {
                    0.0
                };

                stats.mobile_participation_rate = if total_validators > 0 {
                    mobile_validators as f64 / total_validators as f64
                } else {
                    0.0
                };

                stats.iot_participation_rate = if total_validators > 0 {
                    iot_validators as f64 / total_validators as f64
                } else {
                    0.0
                };

                debug!("Validator monitoring: {} total, {} active, {} mobile, {} IoT", 
                       total_validators, active_validators, mobile_validators, iot_validators);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MobilePlatform, MobileCapabilities, NetworkType};

    #[tokio::test]
    async fn test_light_consensus_creation() {
        let config = ZKLockConfig::default();
        let consensus = LightConsensus::new(config).await.unwrap();
        
        let stats = consensus.get_stats().await.unwrap();
        assert_eq!(stats.total_rounds, 0);
    }

    #[tokio::test]
    async fn test_validator_registration() {
        let config = ZKLockConfig::default();
        let consensus = LightConsensus::new(config).await.unwrap();
        
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

        consensus.register_validator(device_id, device_type, "wallet123".to_string()).await.unwrap();
        
        let validators = consensus.validators.read().await;
        assert!(validators.contains_key(&device_id));
        assert_eq!(validators.get(&device_id).unwrap().status, ValidatorStatus::Active);
    }
}
