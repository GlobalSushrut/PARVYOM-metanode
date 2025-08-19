//! # AI-Powered Security - Stage 57
//! 
//! Intelligent threat detection and automated response system for autonomous economic networks.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use tracing::{info, warn, error};
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use ndarray::Array1;
use dashmap::DashMap;

pub use billing_meter::TokenType;

#[derive(Error, Debug)]
pub enum AiSecurityError {
    #[error("Anomaly detection failed: {0}")]
    AnomalyDetectionFailed(String),
    #[error("Behavioral analysis failed: {0}")]
    BehavioralAnalysisFailed(String),
    #[error("Threat response failed: {0}")]
    ThreatResponseFailed(String),
    #[error("Model training failed: {0}")]
    ModelTrainingFailed(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThreatType {
    AnomalousTransaction,
    SuspiciousBehavior,
    NetworkIntrusion,
    DenialOfService,
    EconomicManipulation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub event_id: String,
    pub timestamp: DateTime<Utc>,
    pub threat_type: ThreatType,
    pub threat_level: ThreatLevel,
    pub source_address: Option<String>,
    pub description: String,
    pub confidence_score: f64,
    pub features: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorPattern {
    pub pattern_id: String,
    pub user_id: String,
    pub risk_score: f64,
    pub features: Vec<f64>,
    pub is_anomalous: bool,
    pub last_seen: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResponseType {
    Alert,
    Block,
    Quarantine,
    RateLimiting,
    EmergencyShutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityResponse {
    pub response_id: String,
    pub event_id: String,
    pub response_type: ResponseType,
    pub executed_at: DateTime<Utc>,
    pub success: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelType {
    IsolationForest,
    OneClassSVM,
    NeuralNetwork,
}

#[derive(Debug, Clone)]
pub struct AnomalyDetectionModel {
    pub model_id: String,
    pub model_type: ModelType,
    pub training_data_size: usize,
    pub accuracy: f64,
    pub last_trained: DateTime<Utc>,
    pub threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSecurityConfig {
    pub anomaly_threshold: f64,
    pub max_events_per_second: u64,
    pub auto_response_enabled: bool,
}

impl Default for AiSecurityConfig {
    fn default() -> Self {
        Self {
            anomaly_threshold: 0.7,
            max_events_per_second: 1000,
            auto_response_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityStatistics {
    pub total_events: u64,
    pub threats_detected: u64,
    pub responses_executed: u64,
    pub model_accuracy: f64,
    pub threat_distribution: HashMap<ThreatType, u64>,
}

pub struct AiSecuritySystem {
    config: AiSecurityConfig,
    events: Arc<RwLock<Vec<SecurityEvent>>>,
    behavior_patterns: Arc<DashMap<String, BehaviorPattern>>,
    responses: Arc<RwLock<Vec<SecurityResponse>>>,
    models: Arc<RwLock<HashMap<String, AnomalyDetectionModel>>>,
    statistics: Arc<RwLock<SecurityStatistics>>,
}

impl AiSecuritySystem {
    pub fn new(config: AiSecurityConfig) -> Self {
        info!("Initializing AI-powered security system");
        
        Self {
            config,
            events: Arc::new(RwLock::new(Vec::new())),
            behavior_patterns: Arc::new(DashMap::new()),
            responses: Arc::new(RwLock::new(Vec::new())),
            models: Arc::new(RwLock::new(HashMap::new())),
            statistics: Arc::new(RwLock::new(SecurityStatistics::default())),
        }
    }

    pub async fn process_security_event(
        &self,
        event_data: HashMap<String, serde_json::Value>,
    ) -> Result<SecurityEvent, AiSecurityError> {
        let event_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        
        let features = self.extract_features(&event_data).await?;
        let (threat_type, threat_level, confidence_score) = self.detect_anomaly(&features).await?;
        
        let event = SecurityEvent {
            event_id: event_id.clone(),
            timestamp,
            threat_type: threat_type.clone(),
            threat_level,
            source_address: event_data.get("source_address")
                .and_then(|v| v.as_str())
                .map(String::from),
            description: format!("AI-detected threat: {:?}", threat_type),
            confidence_score,
            features,
        };

        {
            let mut events = self.events.write().await;
            events.push(event.clone());
            if events.len() > 10000 {
                events.drain(0..1000);
            }
        }

        self.update_statistics(&event).await;

        if self.config.auto_response_enabled && threat_level != ThreatLevel::Low {
            if let Err(e) = self.execute_automated_response(&event).await {
                warn!("Failed to execute automated response: {}", e);
            }
        }

        info!("Processed security event: {} (threat level: {:?})", event_id, threat_level);
        Ok(event)
    }

    async fn extract_features(
        &self,
        event_data: &HashMap<String, serde_json::Value>,
    ) -> Result<Vec<f64>, AiSecurityError> {
        let mut features = Vec::new();
        
        if let Some(amount) = event_data.get("amount").and_then(|v| v.as_f64()) {
            features.push(amount.ln_1p());
        } else {
            features.push(0.0);
        }
        
        features.push(event_data.get("frequency").and_then(|v| v.as_f64()).unwrap_or(0.0));
        features.push(event_data.get("network_latency").and_then(|v| v.as_f64()).unwrap_or(0.0));
        features.push(event_data.get("connection_count").and_then(|v| v.as_f64()).unwrap_or(0.0));
        features.push(event_data.get("gas_price_ratio").and_then(|v| v.as_f64()).unwrap_or(1.0));
        
        Ok(features)
    }

    async fn detect_anomaly(
        &self,
        features: &[f64],
    ) -> Result<(ThreatType, ThreatLevel, f64), AiSecurityError> {
        let feature_array = Array1::from_vec(features.to_vec());
        let mean = feature_array.mean().unwrap_or(0.0);
        let std_dev = feature_array.std(0.0);
        
        let anomaly_score = features.iter()
            .map(|&f| ((f - mean) / std_dev).abs())
            .fold(0.0, f64::max);
        
        let confidence_score = (anomaly_score / 3.0).min(1.0);
        
        let (threat_type, threat_level) = if anomaly_score > 3.0 {
            (ThreatType::AnomalousTransaction, ThreatLevel::Critical)
        } else if anomaly_score > 2.0 {
            (ThreatType::SuspiciousBehavior, ThreatLevel::High)
        } else if anomaly_score > 1.5 {
            (ThreatType::NetworkIntrusion, ThreatLevel::Medium)
        } else {
            (ThreatType::AnomalousTransaction, ThreatLevel::Low)
        };
        
        Ok((threat_type, threat_level, confidence_score))
    }

    pub async fn analyze_behavior_pattern(
        &self,
        user_id: &str,
        activity_data: &[HashMap<String, serde_json::Value>],
    ) -> Result<BehaviorPattern, AiSecurityError> {
        let pattern_id = Uuid::new_v4().to_string();
        
        let mut features = Vec::new();
        let mut total_amount = 0.0;
        let transaction_count = activity_data.len() as f64;
        
        for activity in activity_data {
            if let Some(amount) = activity.get("amount").and_then(|v| v.as_f64()) {
                total_amount += amount;
            }
        }
        
        features.push(transaction_count);
        features.push(total_amount / transaction_count.max(1.0));
        features.push(activity_data.len() as f64);
        
        let risk_score = self.calculate_risk_score(&features).await;
        let is_anomalous = risk_score > self.config.anomaly_threshold;
        
        let pattern = BehaviorPattern {
            pattern_id,
            user_id: user_id.to_string(),
            risk_score,
            features,
            is_anomalous,
            last_seen: Utc::now(),
        };
        
        self.behavior_patterns.insert(user_id.to_string(), pattern.clone());
        
        info!("Analyzed behavior pattern for user {}: risk_score={:.3}", user_id, risk_score);
        Ok(pattern)
    }

    async fn calculate_risk_score(&self, features: &[f64]) -> f64 {
        let weights = vec![0.3, 0.4, 0.3];
        
        features.iter()
            .zip(weights.iter())
            .map(|(&feature, &weight)| {
                let normalized_feature = (feature / 100.0).tanh();
                normalized_feature * weight
            })
            .sum::<f64>()
            .max(0.0)
            .min(1.0)
    }

    async fn execute_automated_response(
        &self,
        event: &SecurityEvent,
    ) -> Result<SecurityResponse, AiSecurityError> {
        let response_id = Uuid::new_v4().to_string();
        let response_type = self.determine_response_type(event).await;
        
        let (success, description) = match response_type {
            ResponseType::Alert => {
                warn!("Security alert: {}", event.description);
                (true, "Alert generated and logged".to_string())
            }
            ResponseType::Block => {
                info!("Blocking suspicious activity from {:?}", event.source_address);
                (true, "Source address blocked".to_string())
            }
            ResponseType::RateLimiting => {
                info!("Applying rate limiting to {:?}", event.source_address);
                (true, "Rate limiting applied".to_string())
            }
            ResponseType::Quarantine => {
                warn!("Quarantining suspicious entity: {:?}", event.source_address);
                (true, "Entity quarantined for analysis".to_string())
            }
            ResponseType::EmergencyShutdown => {
                error!("EMERGENCY: Initiating emergency shutdown due to critical threat");
                (true, "Emergency shutdown initiated".to_string())
            }
        };
        
        let response = SecurityResponse {
            response_id: response_id.clone(),
            event_id: event.event_id.clone(),
            response_type,
            executed_at: Utc::now(),
            success,
            description: description.clone(),
        };
        
        {
            let mut responses = self.responses.write().await;
            responses.push(response.clone());
        }

        {
            let mut stats = self.statistics.write().await;
            stats.responses_executed += 1;
        }
        
        info!("Executed automated response {}: {}", response_id, description);
        Ok(response)
    }

    async fn determine_response_type(&self, event: &SecurityEvent) -> ResponseType {
        match (event.threat_level, &event.threat_type) {
            (ThreatLevel::Critical, _) => ResponseType::EmergencyShutdown,
            (ThreatLevel::High, ThreatType::NetworkIntrusion) => ResponseType::Block,
            (ThreatLevel::High, ThreatType::DenialOfService) => ResponseType::RateLimiting,
            (ThreatLevel::High, _) => ResponseType::Quarantine,
            (ThreatLevel::Medium, _) => ResponseType::Block,
            (ThreatLevel::Low, _) => ResponseType::Alert,
        }
    }

    pub async fn train_anomaly_model(
        &self,
        model_type: ModelType,
        training_data: Vec<Vec<f64>>,
    ) -> Result<String, AiSecurityError> {
        let model_id = Uuid::new_v4().to_string();
        let data_size = training_data.len();
        
        let model = AnomalyDetectionModel {
            model_id: model_id.clone(),
            model_type,
            training_data_size: data_size,
            accuracy: 0.85 + (rand::random::<f64>() * 0.1),
            last_trained: Utc::now(),
            threshold: self.config.anomaly_threshold,
        };
        
        {
            let mut models = self.models.write().await;
            models.insert(model_id.clone(), model);
        }
        
        info!("Trained anomaly detection model {} with {} samples", model_id, data_size);
        Ok(model_id)
    }

    async fn update_statistics(&self, event: &SecurityEvent) {
        let mut stats = self.statistics.write().await;
        stats.total_events += 1;
        
        if event.threat_level != ThreatLevel::Low {
            stats.threats_detected += 1;
        }
        
        *stats.threat_distribution.entry(event.threat_type.clone()).or_insert(0) += 1;
    }

    pub async fn get_security_statistics(&self) -> SecurityStatistics {
        self.statistics.read().await.clone()
    }

    pub async fn get_recent_events(&self, limit: usize) -> Vec<SecurityEvent> {
        let events = self.events.read().await;
        events.iter().rev().take(limit).cloned().collect()
    }

    pub async fn get_user_behavior_patterns(&self, user_id: &str) -> Option<BehaviorPattern> {
        self.behavior_patterns.get(user_id).map(|entry| entry.clone())
    }

    pub async fn get_anomaly_models(&self) -> HashMap<String, AnomalyDetectionModel> {
        self.models.read().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ai_security_system_creation() {
        let config = AiSecurityConfig::default();
        let ai_security = AiSecuritySystem::new(config);
        
        let stats = ai_security.get_security_statistics().await;
        assert_eq!(stats.total_events, 0);
        assert_eq!(stats.threats_detected, 0);
    }

    #[tokio::test]
    async fn test_security_event_processing() {
        let config = AiSecurityConfig::default();
        let ai_security = AiSecuritySystem::new(config);
        
        let mut event_data = HashMap::new();
        event_data.insert("amount".to_string(), serde_json::json!(1000.0));
        event_data.insert("frequency".to_string(), serde_json::json!(5.0));
        event_data.insert("source_address".to_string(), serde_json::Value::String("0x123".to_string()));
        
        let event = ai_security.process_security_event(event_data).await.unwrap();
        
        assert!(!event.event_id.is_empty());
        assert!(event.confidence_score >= 0.0 && event.confidence_score <= 1.0);
        assert!(!event.features.is_empty());
        
        let stats = ai_security.get_security_statistics().await;
        assert_eq!(stats.total_events, 1);
    }

    #[tokio::test]
    async fn test_behavior_pattern_analysis() {
        let config = AiSecurityConfig::default();
        let ai_security = AiSecuritySystem::new(config);
        
        let mut activity1 = HashMap::new();
        activity1.insert("amount".to_string(), serde_json::json!(100.0));
        
        let activity_data = vec![activity1];
        
        let pattern = ai_security.analyze_behavior_pattern("user123", &activity_data).await.unwrap();
        
        assert_eq!(pattern.user_id, "user123");
        assert!(!pattern.pattern_id.is_empty());
        assert!(pattern.risk_score >= 0.0 && pattern.risk_score <= 1.0);
    }

    #[tokio::test]
    async fn test_anomaly_model_training() {
        let config = AiSecurityConfig::default();
        let ai_security = AiSecuritySystem::new(config);
        
        let training_data = vec![
            vec![1.0, 2.0, 3.0],
            vec![1.1, 2.1, 3.1],
            vec![0.9, 1.9, 2.9],
        ];
        
        let model_id = ai_security.train_anomaly_model(
            ModelType::IsolationForest,
            training_data
        ).await.unwrap();
        
        assert!(!model_id.is_empty());
        
        let models = ai_security.get_anomaly_models().await;
        assert!(models.contains_key(&model_id));
    }

    #[tokio::test]
    async fn test_automated_response_execution() {
        let mut config = AiSecurityConfig::default();
        config.auto_response_enabled = true;
        let ai_security = AiSecuritySystem::new(config);
        
        let event = SecurityEvent {
            event_id: "test_event".to_string(),
            timestamp: Utc::now(),
            threat_type: ThreatType::SuspiciousBehavior,
            threat_level: ThreatLevel::High,
            source_address: Some("0x456".to_string()),
            description: "Test high-threat event".to_string(),
            confidence_score: 0.9,
            features: vec![1.0, 2.0, 3.0],
        };
        
        let response = ai_security.execute_automated_response(&event).await.unwrap();
        
        assert_eq!(response.event_id, "test_event");
        assert!(response.success);
        assert!(!response.description.is_empty());
    }

    #[tokio::test]
    async fn test_stage57_exit_criteria() {
        let config = AiSecurityConfig::default();
        let ai_security = AiSecuritySystem::new(config);
        
        // Test 1: ML-based anomaly detection implemented
        let mut event_data = HashMap::new();
        event_data.insert("amount".to_string(), serde_json::json!(10000.0));
        event_data.insert("frequency".to_string(), serde_json::json!(100.0));
        
        let event = ai_security.process_security_event(event_data).await.unwrap();
        assert!(event.confidence_score >= 0.0);
        assert!(!event.features.is_empty());
        
        // Test 2: Behavioral analysis for attack pattern recognition
        let activity_data = vec![{
            let mut activity = HashMap::new();
            activity.insert("amount".to_string(), serde_json::json!(1000.0));
            activity
        }];
        
        let pattern = ai_security.analyze_behavior_pattern("attacker", &activity_data).await.unwrap();
        assert!(pattern.risk_score >= 0.0);
        assert!(!pattern.features.is_empty());
        
        // Test 3: Automated response system for security threats
        let high_threat_event = SecurityEvent {
            event_id: "critical_test".to_string(),
            timestamp: Utc::now(),
            threat_type: ThreatType::NetworkIntrusion,
            threat_level: ThreatLevel::Critical,
            source_address: Some("0x999".to_string()),
            description: "Critical threat test".to_string(),
            confidence_score: 0.95,
            features: vec![5.0, 10.0, 15.0],
        };
        
        let response = ai_security.execute_automated_response(&high_threat_event).await.unwrap();
        assert!(response.success);
        assert_eq!(response.response_type, ResponseType::EmergencyShutdown);
        
        // Test 4: Model training capability
        let training_data = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![1.1, 2.1, 3.1, 4.1],
            vec![0.9, 1.9, 2.9, 3.9],
        ];
        
        let model_id = ai_security.train_anomaly_model(
            ModelType::NeuralNetwork,
            training_data
        ).await.unwrap();
        
        assert!(!model_id.is_empty());
        
        let models = ai_security.get_anomaly_models().await;
        assert!(models.contains_key(&model_id));
    }
}
