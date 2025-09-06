use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

/// Validator status in the network
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidatorStatus {
    Active,
    Standby,
    Rotating,
    Suspended,
    Offline,
    Slashed,
}

/// Validator information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorInfo {
    pub validator_id: String,
    pub public_key: String,
    pub stake_amount: u64,
    pub status: ValidatorStatus,
    pub performance_score: f64,
    pub uptime_percentage: f64,
    pub last_block_produced: Option<DateTime<Utc>>,
    pub rotation_eligibility: DateTime<Utc>,
    pub geographic_location: Option<(f64, f64)>,
    pub jurisdiction: Option<String>,
    pub reputation_score: f64,
    pub slash_count: u32,
    pub joined_at: DateTime<Utc>,
    pub last_rotation: Option<DateTime<Utc>>,
}

/// Rotation strategy types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RotationStrategy {
    TimeBasedRotation,
    PerformanceBasedRotation,
    StakeWeightedRotation,
    GeographicDistributionRotation,
    RandomRotation,
    HybridRotation,
}

/// Rotation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationConfig {
    pub strategy: RotationStrategy,
    pub rotation_interval_hours: u64,
    pub min_active_validators: usize,
    pub max_active_validators: usize,
    pub min_performance_score: f64,
    pub min_uptime_percentage: f64,
    pub min_stake_amount: u64,
    pub geographic_distribution_weight: f64,
    pub performance_weight: f64,
    pub stake_weight: f64,
    pub randomness_weight: f64,
    pub cooldown_period_hours: u64,
}

impl Default for RotationConfig {
    fn default() -> Self {
        Self {
            strategy: RotationStrategy::HybridRotation,
            rotation_interval_hours: 24,
            min_active_validators: 21,
            max_active_validators: 101,
            min_performance_score: 0.8,
            min_uptime_percentage: 95.0,
            min_stake_amount: 10000,
            geographic_distribution_weight: 0.3,
            performance_weight: 0.4,
            stake_weight: 0.2,
            randomness_weight: 0.1,
            cooldown_period_hours: 72,
        }
    }
}

/// Rotation event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationEvent {
    pub event_id: String,
    pub event_type: RotationEventType,
    pub validator_id: String,
    pub timestamp: DateTime<Utc>,
    pub reason: String,
    pub previous_status: ValidatorStatus,
    pub new_status: ValidatorStatus,
    pub rotation_round: u64,
}

/// Types of rotation events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RotationEventType {
    ValidatorActivated,
    ValidatorDeactivated,
    ValidatorSuspended,
    ValidatorSlashed,
    PerformanceRotation,
    TimeBasedRotation,
    GeographicRotation,
    EmergencyRotation,
}

/// Rotation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationMetrics {
    pub total_validators: usize,
    pub active_validators: usize,
    pub standby_validators: usize,
    pub suspended_validators: usize,
    pub average_performance_score: f64,
    pub average_uptime: f64,
    pub geographic_distribution_score: f64,
    pub last_rotation_time: Option<DateTime<Utc>>,
    pub rotation_frequency: f64,
    pub validator_turnover_rate: f64,
    pub network_decentralization_score: f64,
}

/// Validator Rotation Coordinator
pub struct ValidatorRotationCoordinator {
    validators: Arc<RwLock<HashMap<String, ValidatorInfo>>>,
    active_validators: Arc<RwLock<HashSet<String>>>,
    rotation_config: RotationConfig,
    rotation_events: Arc<RwLock<VecDeque<RotationEvent>>>,
    rotation_round: Arc<RwLock<u64>>,
    last_rotation: Arc<RwLock<Option<DateTime<Utc>>>>,
    rotation_enabled: Arc<RwLock<bool>>,
    emergency_mode: Arc<RwLock<bool>>,
}

impl ValidatorRotationCoordinator {
    /// Create new validator rotation coordinator
    pub fn new(config: Option<RotationConfig>) -> Self {
        Self {
            validators: Arc::new(RwLock::new(HashMap::new())),
            active_validators: Arc::new(RwLock::new(HashSet::new())),
            rotation_config: config.unwrap_or_default(),
            rotation_events: Arc::new(RwLock::new(VecDeque::new())),
            rotation_round: Arc::new(RwLock::new(0)),
            last_rotation: Arc::new(RwLock::new(None)),
            rotation_enabled: Arc::new(RwLock::new(true)),
            emergency_mode: Arc::new(RwLock::new(false)),
        }
    }

    /// Register a new validator
    pub async fn register_validator(
        &self,
        validator_id: String,
        public_key: String,
        stake_amount: u64,
        geographic_location: Option<(f64, f64)>,
        jurisdiction: Option<String>,
    ) -> Result<()> {
        let mut validators = self.validators.write().await;
        
        if validators.contains_key(&validator_id) {
            return Err(anyhow!("Validator {} already registered", validator_id));
        }

        let validator_info = ValidatorInfo {
            validator_id: validator_id.clone(),
            public_key,
            stake_amount,
            status: ValidatorStatus::Standby,
            performance_score: 1.0,
            uptime_percentage: 100.0,
            last_block_produced: None,
            rotation_eligibility: Utc::now(),
            geographic_location,
            jurisdiction,
            reputation_score: 0.5,
            slash_count: 0,
            joined_at: Utc::now(),
            last_rotation: None,
        };

        validators.insert(validator_id.clone(), validator_info);
        info!("Registered validator: {}", validator_id);

        // Check if we need to activate this validator immediately
        self.evaluate_validator_activation().await?;

        Ok(())
    }

    /// Update validator performance metrics
    pub async fn update_validator_performance(
        &self,
        validator_id: &str,
        performance_score: f64,
        uptime_percentage: f64,
        last_block_produced: Option<DateTime<Utc>>,
    ) -> Result<()> {
        let mut validators = self.validators.write().await;
        
        if let Some(validator) = validators.get_mut(validator_id) {
            validator.performance_score = performance_score;
            validator.uptime_percentage = uptime_percentage;
            validator.last_block_produced = last_block_produced;

            // Check if performance has dropped below threshold
            if performance_score < self.rotation_config.min_performance_score ||
               uptime_percentage < self.rotation_config.min_uptime_percentage {
                warn!("Validator {} performance below threshold", validator_id);
                self.schedule_performance_rotation(validator_id).await?;
            }
        } else {
            return Err(anyhow!("Validator {} not found", validator_id));
        }

        Ok(())
    }

    /// Perform validator rotation based on strategy
    pub async fn perform_rotation(&self) -> Result<Vec<RotationEvent>> {
        let rotation_enabled = *self.rotation_enabled.read().await;
        if !rotation_enabled {
            return Ok(Vec::new());
        }

        let mut events = Vec::new();
        let mut rotation_round = self.rotation_round.write().await;
        *rotation_round += 1;
        let current_round = *rotation_round;

        info!("Starting validator rotation round {}", current_round);

        match self.rotation_config.strategy {
            RotationStrategy::PerformanceBasedRotation => {
                events.extend(self.perform_performance_based_rotation(current_round).await?);
            }
            RotationStrategy::HybridRotation => {
                events.extend(self.perform_hybrid_rotation(current_round).await?);
            }
            _ => {
                // Default to performance-based rotation for other strategies
                events.extend(self.perform_performance_based_rotation(current_round).await?);
            }
        }

        // Update last rotation time
        let mut last_rotation = self.last_rotation.write().await;
        *last_rotation = Some(Utc::now());

        // Store rotation events
        let mut rotation_events = self.rotation_events.write().await;
        rotation_events.extend(events.clone());

        // Keep only last 10000 events
        if rotation_events.len() > 10000 {
            let drain_count = rotation_events.len() - 10000;
            rotation_events.drain(0..drain_count);
        }

        info!("Completed validator rotation round {} with {} events", current_round, events.len());
        Ok(events)
    }

    /// Perform performance-based rotation
    async fn perform_performance_based_rotation(&self, rotation_round: u64) -> Result<Vec<RotationEvent>> {
        let mut events = Vec::new();
        let validators = self.validators.read().await;
        let mut active_validators = self.active_validators.write().await;

        // Sort active validators by performance (worst first)
        let mut active_performance: Vec<_> = validators.values()
            .filter(|v| v.status == ValidatorStatus::Active)
            .map(|v| (v.validator_id.clone(), v.performance_score, v.uptime_percentage))
            .collect();
        
        active_performance.sort_by(|a, b| {
            let score_a = a.1 * 0.7 + a.2 * 0.3;
            let score_b = b.1 * 0.7 + b.2 * 0.3;
            score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Sort standby validators by performance (best first)
        let mut standby_performance: Vec<_> = validators.values()
            .filter(|v| v.status == ValidatorStatus::Standby)
            .map(|v| (v.validator_id.clone(), v.performance_score, v.uptime_percentage))
            .collect();
        
        standby_performance.sort_by(|a, b| {
            let score_a = a.1 * 0.7 + a.2 * 0.3;
            let score_b = b.1 * 0.7 + b.2 * 0.3;
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        let now = Utc::now();
        let rotation_count = (active_performance.len() / 4).max(1);

        for i in 0..rotation_count.min(standby_performance.len()) {
            let (deactivate_id, _, _) = &active_performance[i];
            let (activate_id, _, _) = &standby_performance[i];

            // Deactivate poor performer
            active_validators.remove(deactivate_id);
            events.push(RotationEvent {
                event_id: Uuid::new_v4().to_string(),
                event_type: RotationEventType::PerformanceRotation,
                validator_id: deactivate_id.clone(),
                timestamp: now,
                reason: "Poor performance rotation".to_string(),
                previous_status: ValidatorStatus::Active,
                new_status: ValidatorStatus::Standby,
                rotation_round,
            });

            // Activate high performer
            active_validators.insert(activate_id.clone());
            events.push(RotationEvent {
                event_id: Uuid::new_v4().to_string(),
                event_type: RotationEventType::PerformanceRotation,
                validator_id: activate_id.clone(),
                timestamp: now,
                reason: "High performance promotion".to_string(),
                previous_status: ValidatorStatus::Standby,
                new_status: ValidatorStatus::Active,
                rotation_round,
            });
        }

        Ok(events)
    }

    /// Perform hybrid rotation (combines multiple strategies)
    async fn perform_hybrid_rotation(&self, rotation_round: u64) -> Result<Vec<RotationEvent>> {
        // For simplicity, use performance-based rotation as the primary strategy
        self.perform_performance_based_rotation(rotation_round).await
    }

    /// Schedule performance-based rotation for underperforming validator
    async fn schedule_performance_rotation(&self, validator_id: &str) -> Result<()> {
        let mut validators = self.validators.write().await;
        let mut active_validators = self.active_validators.write().await;

        if let Some(validator) = validators.get_mut(validator_id) {
            if validator.status == ValidatorStatus::Active {
                validator.status = ValidatorStatus::Suspended;
                active_validators.remove(validator_id);

                let mut rotation_events = self.rotation_events.write().await;
                rotation_events.push_back(RotationEvent {
                    event_id: Uuid::new_v4().to_string(),
                    event_type: RotationEventType::ValidatorSuspended,
                    validator_id: validator_id.to_string(),
                    timestamp: Utc::now(),
                    reason: "Performance below threshold".to_string(),
                    previous_status: ValidatorStatus::Active,
                    new_status: ValidatorStatus::Suspended,
                    rotation_round: *self.rotation_round.read().await,
                });

                warn!("Suspended validator {} due to poor performance", validator_id);
            }
        }

        Ok(())
    }

    /// Evaluate if new validators should be activated
    async fn evaluate_validator_activation(&self) -> Result<()> {
        let validators = self.validators.read().await;
        let mut active_validators = self.active_validators.write().await;

        let current_active = active_validators.len();
        let target_active = self.rotation_config.min_active_validators;

        if current_active < target_active {
            // Find eligible standby validators
            let mut eligible: Vec<_> = validators.values()
                .filter(|v| v.status == ValidatorStatus::Standby &&
                           v.stake_amount >= self.rotation_config.min_stake_amount &&
                           v.performance_score >= self.rotation_config.min_performance_score)
                .collect();

            // Sort by combined score (stake + performance)
            eligible.sort_by(|a, b| {
                let score_a = (a.stake_amount as f64).ln() + a.performance_score;
                let score_b = (b.stake_amount as f64).ln() + b.performance_score;
                score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
            });

            let to_activate = (target_active - current_active).min(eligible.len());
            
            for i in 0..to_activate {
                let validator = eligible[i];
                active_validators.insert(validator.validator_id.clone());
                info!("Activated validator: {}", validator.validator_id);
            }
        }

        Ok(())
    }

    /// Get current rotation metrics
    pub async fn get_rotation_metrics(&self) -> Result<RotationMetrics> {
        let validators = self.validators.read().await;
        let active_validators = self.active_validators.read().await;

        let total_validators = validators.len();
        let active_count = active_validators.len();
        let standby_count = validators.values().filter(|v| v.status == ValidatorStatus::Standby).count();
        let suspended_count = validators.values().filter(|v| v.status == ValidatorStatus::Suspended).count();

        let avg_performance = if total_validators > 0 {
            validators.values().map(|v| v.performance_score).sum::<f64>() / total_validators as f64
        } else {
            0.0
        };

        let avg_uptime = if total_validators > 0 {
            validators.values().map(|v| v.uptime_percentage).sum::<f64>() / total_validators as f64
        } else {
            0.0
        };

        // Calculate geographic distribution score
        let mut region_counts: HashMap<String, usize> = HashMap::new();
        for validator_id in active_validators.iter() {
            if let Some(validator) = validators.get(validator_id) {
                if let Some(jurisdiction) = &validator.jurisdiction {
                    *region_counts.entry(jurisdiction.clone()).or_insert(0) += 1;
                }
            }
        }

        let geographic_distribution_score = if region_counts.is_empty() {
            0.0
        } else {
            let max_count = *region_counts.values().max().unwrap_or(&0);
            let min_count = *region_counts.values().min().unwrap_or(&0);
            if max_count == 0 { 0.0 } else { min_count as f64 / max_count as f64 }
        };

        // Calculate rotation frequency and turnover rate
        let rotation_events = self.rotation_events.read().await;
        let recent_events: Vec<_> = rotation_events.iter()
            .filter(|e| {
                let now = Utc::now();
                now.signed_duration_since(e.timestamp) <= Duration::days(1)
            })
            .collect();

        let rotation_frequency = recent_events.len() as f64;
        let validator_turnover_rate = if total_validators > 0 {
            recent_events.len() as f64 / total_validators as f64
        } else {
            0.0
        };

        // Calculate network decentralization score
        let network_decentralization_score = if active_count > 0 {
            let stake_distribution = self.calculate_stake_distribution_score().await?;
            let geographic_score = geographic_distribution_score;
            (stake_distribution + geographic_score) / 2.0
        } else {
            0.0
        };

        Ok(RotationMetrics {
            total_validators,
            active_validators: active_count,
            standby_validators: standby_count,
            suspended_validators: suspended_count,
            average_performance_score: avg_performance,
            average_uptime: avg_uptime,
            geographic_distribution_score,
            last_rotation_time: *self.last_rotation.read().await,
            rotation_frequency,
            validator_turnover_rate,
            network_decentralization_score,
        })
    }

    /// Calculate stake distribution score
    async fn calculate_stake_distribution_score(&self) -> Result<f64> {
        let validators = self.validators.read().await;
        let active_validators = self.active_validators.read().await;

        let total_stake: u64 = active_validators.iter()
            .filter_map(|id| validators.get(id))
            .map(|v| v.stake_amount)
            .sum();

        if total_stake == 0 {
            return Ok(0.0);
        }

        // Calculate Gini coefficient for stake distribution
        let mut stakes: Vec<u64> = active_validators.iter()
            .filter_map(|id| validators.get(id))
            .map(|v| v.stake_amount)
            .collect();

        stakes.sort();

        let n = stakes.len() as f64;
        let mut sum_diff = 0.0;

        for i in 0..stakes.len() {
            for j in 0..stakes.len() {
                sum_diff += (stakes[i] as f64 - stakes[j] as f64).abs();
            }
        }

        let mean_stake = total_stake as f64 / n;
        let gini = sum_diff / (2.0 * n * n * mean_stake);
        
        // Convert Gini to distribution score (1.0 - gini for better distribution)
        Ok(1.0 - gini.min(1.0))
    }

    /// Enable/disable rotation
    pub async fn set_rotation_enabled(&self, enabled: bool) -> Result<()> {
        let mut rotation_enabled = self.rotation_enabled.write().await;
        *rotation_enabled = enabled;
        info!("Validator rotation {}", if enabled { "enabled" } else { "disabled" });
        Ok(())
    }

    /// Get validator information
    pub async fn get_validator(&self, validator_id: &str) -> Result<Option<ValidatorInfo>> {
        let validators = self.validators.read().await;
        Ok(validators.get(validator_id).cloned())
    }

    /// Get all active validators
    pub async fn get_active_validators(&self) -> Result<Vec<ValidatorInfo>> {
        let validators = self.validators.read().await;
        let active_validators = self.active_validators.read().await;

        let active_list = active_validators.iter()
            .filter_map(|id| validators.get(id))
            .cloned()
            .collect();

        Ok(active_list)
    }

    /// Get recent rotation events
    pub async fn get_recent_events(&self, limit: usize) -> Result<Vec<RotationEvent>> {
        let events = self.rotation_events.read().await;
        let start_idx = if events.len() > limit {
            events.len() - limit
        } else {
            0
        };
        Ok(events.range(start_idx..).cloned().collect())
    }

    /// Remove validator from the system
    pub async fn remove_validator(&self, validator_id: &str) -> Result<()> {
        let mut validators = self.validators.write().await;
        let mut active_validators = self.active_validators.write().await;

        if validators.remove(validator_id).is_some() {
            active_validators.remove(validator_id);
            info!("Removed validator: {}", validator_id);
        }

        Ok(())
    }

    /// Slash validator for malicious behavior
    pub async fn slash_validator(&self, validator_id: &str, reason: String) -> Result<()> {
        let mut validators = self.validators.write().await;
        let mut active_validators = self.active_validators.write().await;

        if let Some(validator) = validators.get_mut(validator_id) {
            validator.status = ValidatorStatus::Slashed;
            validator.slash_count += 1;
            validator.reputation_score = (validator.reputation_score * 0.5).max(0.0);
            
            active_validators.remove(validator_id);

            let mut rotation_events = self.rotation_events.write().await;
            rotation_events.push_back(RotationEvent {
                event_id: Uuid::new_v4().to_string(),
                event_type: RotationEventType::ValidatorSlashed,
                validator_id: validator_id.to_string(),
                timestamp: Utc::now(),
                reason,
                previous_status: ValidatorStatus::Active,
                new_status: ValidatorStatus::Slashed,
                rotation_round: *self.rotation_round.read().await,
            });

            warn!("Slashed validator: {}", validator_id);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_validator_registration() {
        let coordinator = ValidatorRotationCoordinator::new(None);
        
        assert!(coordinator.register_validator(
            "validator1".to_string(),
            "pubkey1".to_string(),
            50000,
            Some((40.7128, -74.0060)),
            Some("US".to_string())
        ).await.is_ok());
        
        let validator = coordinator.get_validator("validator1").await.unwrap();
        assert!(validator.is_some());
        assert_eq!(validator.unwrap().stake_amount, 50000);
    }

    #[tokio::test]
    async fn test_rotation_metrics() {
        let coordinator = ValidatorRotationCoordinator::new(None);
        
        // Register multiple validators
        for i in 0..5 {
            assert!(coordinator.register_validator(
                format!("validator{}", i),
                format!("pubkey{}", i),
                10000 + i as u64 * 1000,
                Some((40.0 + i as f64, -74.0)),
                Some("US".to_string())
            ).await.is_ok());
        }
        
        let metrics = coordinator.get_rotation_metrics().await.unwrap();
        assert_eq!(metrics.total_validators, 5);
        assert!(metrics.average_performance_score > 0.0);
    }

    #[tokio::test]
    async fn test_performance_update() {
        let coordinator = ValidatorRotationCoordinator::new(None);
        
        assert!(coordinator.register_validator(
            "validator1".to_string(),
            "pubkey1".to_string(),
            50000,
            None,
            None
        ).await.is_ok());
        
        assert!(coordinator.update_validator_performance(
            "validator1",
            0.9,
            98.5,
            Some(Utc::now())
        ).await.is_ok());
        
        let validator = coordinator.get_validator("validator1").await.unwrap().unwrap();
        assert_eq!(validator.performance_score, 0.9);
        assert_eq!(validator.uptime_percentage, 98.5);
    }
}
