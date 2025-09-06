//! Nakamoto Coefficient Monitor for BPI Ledger Decentralization
//! 
//! Ensures Nakamoto coefficient >3.0 enforcement with:
//! - Real-time coefficient calculation
//! - Historical tracking and trend analysis
//! - Automatic alerts and corrections
//! - Stake distribution monitoring

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Duration};
use anyhow::Result;
use thiserror::Error;
use tracing::{info, warn, error};

/// Nakamoto Coefficient Monitor
#[derive(Debug)]
pub struct NakamotoMonitor {
    /// Current Nakamoto coefficient
    pub current_coefficient: f64,
    /// Target coefficient (minimum required)
    pub target_coefficient: f64,
    /// Historical coefficient data
    pub historical_data: Vec<NakamotoDataPoint>,
    /// Monitoring parameters
    pub monitoring_params: NakamotoMonitoringParams,
    /// Alert system
    pub alert_system: NakamotoAlertSystem,
}

/// Nakamoto Data Point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NakamotoDataPoint {
    /// Timestamp of measurement
    pub timestamp: DateTime<Utc>,
    /// Coefficient value
    pub coefficient: f64,
    /// Contributing factors
    pub factors: NakamotoFactors,
    /// Validator count at time of measurement
    pub validator_count: u32,
    /// Total stake at time of measurement
    pub total_stake: f64,
}

/// Factors contributing to Nakamoto coefficient
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NakamotoFactors {
    /// Stake distribution factor
    pub stake_distribution: f64,
    /// Geographic distribution factor
    pub geographic_distribution: f64,
    /// Validator diversity factor
    pub validator_diversity: f64,
    /// Network decentralization factor
    pub network_decentralization: f64,
}

/// Nakamoto Monitoring Parameters
#[derive(Debug, Clone)]
pub struct NakamotoMonitoringParams {
    /// Monitoring interval in minutes
    pub monitoring_interval_minutes: u64,
    /// Alert threshold (coefficient below this triggers alert)
    pub alert_threshold: f64,
    /// Critical threshold (coefficient below this triggers emergency response)
    pub critical_threshold: f64,
    /// Auto-correction enabled
    pub auto_correction: bool,
    /// Historical data retention period (days)
    pub retention_period_days: u32,
}

/// Nakamoto Alert System
#[derive(Debug)]
pub struct NakamotoAlertSystem {
    /// Active alerts
    pub active_alerts: Vec<NakamotoAlert>,
    /// Alert history
    pub alert_history: Vec<NakamotoAlert>,
    /// Alert thresholds
    pub thresholds: AlertThresholds,
}

/// Nakamoto Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NakamotoAlert {
    /// Alert ID
    pub alert_id: String,
    /// Alert type
    pub alert_type: AlertType,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Current coefficient value
    pub current_coefficient: f64,
    /// Threshold violated
    pub threshold_violated: f64,
    /// Alert timestamp
    pub timestamp: DateTime<Utc>,
    /// Alert message
    pub message: String,
    /// Recommended actions
    pub recommended_actions: Vec<String>,
    /// Alert status
    pub status: AlertStatus,
}

/// Alert Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    CoefficientBelowTarget,
    CoefficientCritical,
    StakeConcentration,
    ValidatorConcentration,
    TrendDeteriorating,
}

/// Alert Severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

/// Alert Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertStatus {
    Active,
    Acknowledged,
    Resolved,
    Escalated,
}

/// Alert Thresholds
#[derive(Debug, Clone)]
pub struct AlertThresholds {
    /// Warning threshold
    pub warning_threshold: f64,
    /// Critical threshold
    pub critical_threshold: f64,
    /// Emergency threshold
    pub emergency_threshold: f64,
    /// Trend deterioration threshold (percentage decline)
    pub trend_threshold: f64,
}

/// Validator Stake Information
#[derive(Debug, Clone)]
pub struct ValidatorStake {
    /// Validator ID
    pub validator_id: String,
    /// Stake amount
    pub stake_amount: f64,
    /// Stake percentage of total
    pub stake_percentage: f64,
    /// Geographic location
    pub location: String,
    /// Validator type
    pub validator_type: String,
}

/// Nakamoto Monitor Errors
#[derive(Error, Debug)]
pub enum NakamotoError {
    #[error("Nakamoto coefficient {coefficient:.2} below critical threshold {threshold:.2}")]
    CriticalThresholdViolation { coefficient: f64, threshold: f64 },
    #[error("Insufficient validator data for calculation")]
    InsufficientData,
    #[error("Stake concentration too high: {concentration:.2}")]
    StakeConcentrationHigh { concentration: f64 },
    #[error("Alert system error: {message}")]
    AlertSystemError { message: String },
}

impl NakamotoMonitor {
    /// Create new Nakamoto monitor
    pub fn new() -> Self {
        Self {
            current_coefficient: 0.0,
            target_coefficient: 3.0, // Minimum required
            historical_data: Vec::new(),
            monitoring_params: NakamotoMonitoringParams::default(),
            alert_system: NakamotoAlertSystem::new(),
        }
    }

    /// Calculate Nakamoto coefficient from validator stakes
    pub fn calculate_coefficient(&mut self, validator_stakes: &[ValidatorStake]) -> Result<f64, NakamotoError> {
        if validator_stakes.is_empty() {
            return Err(NakamotoError::InsufficientData);
        }

        // Sort validators by stake amount (descending)
        let mut stakes: Vec<f64> = validator_stakes.iter()
            .map(|v| v.stake_amount)
            .collect();
        stakes.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let total_stake: f64 = stakes.iter().sum();
        if total_stake == 0.0 {
            return Err(NakamotoError::InsufficientData);
        }

        // Find minimum validators needed to control >33% of stake
        let control_threshold = total_stake * 0.33;
        let mut cumulative_stake = 0.0;
        let mut validators_needed = 0;

        for stake in stakes {
            cumulative_stake += stake;
            validators_needed += 1;
            if cumulative_stake > control_threshold {
                break;
            }
        }

        let coefficient = validators_needed as f64;
        self.current_coefficient = coefficient;

        // Calculate contributing factors
        let factors = self.calculate_factors(validator_stakes, total_stake);

        // Record data point
        let data_point = NakamotoDataPoint {
            timestamp: Utc::now(),
            coefficient,
            factors,
            validator_count: validator_stakes.len() as u32,
            total_stake,
        };
        self.historical_data.push(data_point);

        // Cleanup old data
        self.cleanup_historical_data();

        // Check for alerts
        self.check_alerts(coefficient)?;

        info!("Nakamoto coefficient calculated: {:.2}", coefficient);
        Ok(coefficient)
    }

    /// Calculate contributing factors
    fn calculate_factors(&self, validator_stakes: &[ValidatorStake], total_stake: f64) -> NakamotoFactors {
        // Stake distribution factor (Gini coefficient)
        let stake_distribution = self.calculate_gini_coefficient(validator_stakes);

        // Geographic distribution factor
        let geographic_distribution = self.calculate_geographic_distribution(validator_stakes);

        // Validator diversity factor
        let validator_diversity = self.calculate_validator_diversity(validator_stakes);

        // Network decentralization factor (composite)
        let network_decentralization = (stake_distribution + geographic_distribution + validator_diversity) / 3.0;

        NakamotoFactors {
            stake_distribution,
            geographic_distribution,
            validator_diversity,
            network_decentralization,
        }
    }

    /// Calculate Gini coefficient for stake distribution
    fn calculate_gini_coefficient(&self, validator_stakes: &[ValidatorStake]) -> f64 {
        if validator_stakes.len() < 2 {
            return 0.0;
        }

        let mut stakes: Vec<f64> = validator_stakes.iter()
            .map(|v| v.stake_amount)
            .collect();
        stakes.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let n = stakes.len() as f64;
        let mean = stakes.iter().sum::<f64>() / n;

        if mean == 0.0 {
            return 0.0;
        }

        let mut gini_sum = 0.0;
        for (i, stake_i) in stakes.iter().enumerate() {
            for stake_j in stakes.iter() {
                gini_sum += (stake_i - stake_j).abs();
            }
        }

        let gini = gini_sum / (2.0 * n * n * mean);
        1.0 - gini // Return inverse for better distribution score
    }

    /// Calculate geographic distribution score
    fn calculate_geographic_distribution(&self, validator_stakes: &[ValidatorStake]) -> f64 {
        let mut location_counts: HashMap<String, u32> = HashMap::new();

        for validator in validator_stakes {
            *location_counts.entry(validator.location.clone()).or_insert(0) += 1;
        }

        let total_validators = validator_stakes.len() as f64;
        if total_validators == 0.0 {
            return 0.0;
        }

        // Calculate entropy
        let mut entropy = 0.0;
        for &count in location_counts.values() {
            if count > 0 {
                let probability = count as f64 / total_validators;
                entropy -= probability * probability.log2();
            }
        }

        // Normalize entropy to 0-1 scale
        if location_counts.len() <= 1 {
            0.0
        } else {
            entropy / (location_counts.len() as f64).log2()
        }
    }

    /// Calculate validator diversity score
    fn calculate_validator_diversity(&self, validator_stakes: &[ValidatorStake]) -> f64 {
        let mut type_counts: HashMap<String, u32> = HashMap::new();

        for validator in validator_stakes {
            *type_counts.entry(validator.validator_type.clone()).or_insert(0) += 1;
        }

        let total_validators = validator_stakes.len() as f64;
        if total_validators == 0.0 {
            return 0.0;
        }

        // Calculate diversity index (Simpson's diversity index)
        let mut diversity_sum = 0.0;
        for &count in type_counts.values() {
            let proportion = count as f64 / total_validators;
            diversity_sum += proportion * proportion;
        }

        1.0 - diversity_sum // Simpson's diversity index
    }

    /// Check for alerts based on current coefficient
    fn check_alerts(&mut self, coefficient: f64) -> Result<(), NakamotoError> {
        let thresholds = &self.alert_system.thresholds;

        // Check critical threshold
        if coefficient < thresholds.critical_threshold {
            let alert = self.create_alert(
                AlertType::CoefficientCritical,
                AlertSeverity::Critical,
                coefficient,
                thresholds.critical_threshold,
                format!("Nakamoto coefficient {:.2} is critically low", coefficient),
                vec![
                    "Immediate validator rotation required".to_string(),
                    "Increase geographic distribution".to_string(),
                    "Recruit validators from underrepresented regions".to_string(),
                ],
            );
            self.alert_system.active_alerts.push(alert.clone());
            self.alert_system.alert_history.push(alert);

            return Err(NakamotoError::CriticalThresholdViolation {
                coefficient,
                threshold: thresholds.critical_threshold,
            });
        }

        // Check warning threshold
        if coefficient < thresholds.warning_threshold {
            let alert = self.create_alert(
                AlertType::CoefficientBelowTarget,
                AlertSeverity::Warning,
                coefficient,
                thresholds.warning_threshold,
                format!("Nakamoto coefficient {:.2} below target", coefficient),
                vec![
                    "Monitor validator distribution".to_string(),
                    "Consider validator incentives".to_string(),
                ],
            );
            self.alert_system.active_alerts.push(alert.clone());
            self.alert_system.alert_history.push(alert);
        }

        // Check trend deterioration
        if self.historical_data.len() >= 10 {
            let recent_trend = self.calculate_trend();
            if recent_trend < -thresholds.trend_threshold {
                let alert = self.create_alert(
                    AlertType::TrendDeteriorating,
                    AlertSeverity::Warning,
                    coefficient,
                    0.0,
                    format!("Nakamoto coefficient trending downward: {:.2}%", recent_trend * 100.0),
                    vec![
                        "Investigate cause of deterioration".to_string(),
                        "Implement preventive measures".to_string(),
                    ],
                );
                self.alert_system.active_alerts.push(alert.clone());
                self.alert_system.alert_history.push(alert);
            }
        }

        Ok(())
    }

    /// Create alert
    fn create_alert(
        &self,
        alert_type: AlertType,
        severity: AlertSeverity,
        current_coefficient: f64,
        threshold_violated: f64,
        message: String,
        recommended_actions: Vec<String>,
    ) -> NakamotoAlert {
        NakamotoAlert {
            alert_id: uuid::Uuid::new_v4().to_string(),
            alert_type,
            severity,
            current_coefficient,
            threshold_violated,
            timestamp: Utc::now(),
            message,
            recommended_actions,
            status: AlertStatus::Active,
        }
    }

    /// Calculate recent trend
    fn calculate_trend(&self) -> f64 {
        if self.historical_data.len() < 2 {
            return 0.0;
        }

        let recent_count = 10.min(self.historical_data.len());
        let recent_data = &self.historical_data[self.historical_data.len() - recent_count..];

        if recent_data.len() < 2 {
            return 0.0;
        }

        let first = recent_data.first().unwrap().coefficient;
        let last = recent_data.last().unwrap().coefficient;

        if first == 0.0 {
            return 0.0;
        }

        (last - first) / first
    }

    /// Cleanup old historical data
    fn cleanup_historical_data(&mut self) {
        let retention_period = Duration::days(self.monitoring_params.retention_period_days as i64);
        let cutoff_time = Utc::now() - retention_period;

        self.historical_data.retain(|data_point| data_point.timestamp > cutoff_time);
    }

    /// Get current coefficient
    pub fn get_current_coefficient(&self) -> f64 {
        self.current_coefficient
    }

    /// Get active alerts
    pub fn get_active_alerts(&self) -> &[NakamotoAlert] {
        &self.alert_system.active_alerts
    }

    /// Get historical data
    pub fn get_historical_data(&self) -> &[NakamotoDataPoint] {
        &self.historical_data
    }

    /// Acknowledge alert
    pub fn acknowledge_alert(&mut self, alert_id: &str) -> Result<(), NakamotoError> {
        if let Some(alert) = self.alert_system.active_alerts.iter_mut()
            .find(|a| a.alert_id == alert_id) {
            alert.status = AlertStatus::Acknowledged;
            info!("Alert {} acknowledged", alert_id);
            Ok(())
        } else {
            Err(NakamotoError::AlertSystemError {
                message: format!("Alert {} not found", alert_id),
            })
        }
    }

    /// Resolve alert
    pub fn resolve_alert(&mut self, alert_id: &str) -> Result<(), NakamotoError> {
        if let Some(pos) = self.alert_system.active_alerts.iter()
            .position(|a| a.alert_id == alert_id) {
            let mut alert = self.alert_system.active_alerts.remove(pos);
            alert.status = AlertStatus::Resolved;
            info!("Alert {} resolved", alert_id);
            Ok(())
        } else {
            Err(NakamotoError::AlertSystemError {
                message: format!("Alert {} not found", alert_id),
            })
        }
    }

    /// Check if coefficient meets target
    pub fn meets_target(&self) -> bool {
        self.current_coefficient >= self.target_coefficient
    }

    /// Get coefficient health status
    pub fn get_health_status(&self) -> CoefficientHealthStatus {
        if self.current_coefficient >= self.target_coefficient {
            CoefficientHealthStatus::Healthy
        } else if self.current_coefficient >= self.alert_system.thresholds.warning_threshold {
            CoefficientHealthStatus::Warning
        } else if self.current_coefficient >= self.alert_system.thresholds.critical_threshold {
            CoefficientHealthStatus::Critical
        } else {
            CoefficientHealthStatus::Emergency
        }
    }
}

/// Coefficient Health Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoefficientHealthStatus {
    Healthy,
    Warning,
    Critical,
    Emergency,
}

impl NakamotoMonitoringParams {
    pub fn default() -> Self {
        Self {
            monitoring_interval_minutes: 5,
            alert_threshold: 3.0,
            critical_threshold: 2.0,
            auto_correction: true,
            retention_period_days: 30,
        }
    }
}

impl NakamotoAlertSystem {
    pub fn new() -> Self {
        Self {
            active_alerts: Vec::new(),
            alert_history: Vec::new(),
            thresholds: AlertThresholds {
                warning_threshold: 3.0,
                critical_threshold: 2.0,
                emergency_threshold: 1.0,
                trend_threshold: 0.1, // 10% decline
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nakamoto_coefficient_calculation() {
        let mut monitor = NakamotoMonitor::new();
        
        let validator_stakes = vec![
            ValidatorStake {
                validator_id: "v1".to_string(),
                stake_amount: 1000.0,
                stake_percentage: 50.0,
                location: "US".to_string(),
                validator_type: "full".to_string(),
            },
            ValidatorStake {
                validator_id: "v2".to_string(),
                stake_amount: 500.0,
                stake_percentage: 25.0,
                location: "EU".to_string(),
                validator_type: "full".to_string(),
            },
            ValidatorStake {
                validator_id: "v3".to_string(),
                stake_amount: 300.0,
                stake_percentage: 15.0,
                location: "ASIA".to_string(),
                validator_type: "light".to_string(),
            },
            ValidatorStake {
                validator_id: "v4".to_string(),
                stake_amount: 200.0,
                stake_percentage: 10.0,
                location: "US".to_string(),
                validator_type: "full".to_string(),
            },
        ];

        let coefficient = monitor.calculate_coefficient(&validator_stakes).unwrap();
        assert!(coefficient >= 1.0);
        assert!(coefficient <= validator_stakes.len() as f64);
    }

    #[test]
    fn test_alert_system() {
        let mut monitor = NakamotoMonitor::new();
        
        // Test with low coefficient that should trigger alert
        let validator_stakes = vec![
            ValidatorStake {
                validator_id: "v1".to_string(),
                stake_amount: 1000.0,
                stake_percentage: 90.0,
                location: "US".to_string(),
                validator_type: "full".to_string(),
            },
            ValidatorStake {
                validator_id: "v2".to_string(),
                stake_amount: 100.0,
                stake_percentage: 10.0,
                location: "US".to_string(),
                validator_type: "full".to_string(),
            },
        ];

        let result = monitor.calculate_coefficient(&validator_stakes);
        assert!(result.is_err()); // Should trigger critical alert
        assert!(!monitor.get_active_alerts().is_empty());
    }
}
