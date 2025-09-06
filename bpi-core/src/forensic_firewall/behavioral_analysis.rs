use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;

use crate::forensic_firewall::cue_engine::{CueRuleEngine, SecurityDecision};
use crate::forensic_firewall::ml_framework::{MlModel, FeatureVector, MlPrediction};

/// Behavioral analysis framework for detecting anomalous patterns
#[derive(Debug, Clone)]
pub struct BehavioralAnalyzer {
    pub id: Uuid,
    pub cue_engine: Arc<CueRuleEngine>,
    pub user_profiles: Arc<RwLock<HashMap<String, UserProfile>>>,
    pub network_baselines: Arc<RwLock<HashMap<String, NetworkBaseline>>>,
    pub system_baselines: Arc<RwLock<HashMap<String, SystemBaseline>>>,
    pub ml_models: Arc<RwLock<HashMap<String, Box<dyn MlModel + Send + Sync>>>>,
    pub analysis_cache: Arc<RwLock<HashMap<String, CachedAnalysis>>>,
    pub config: BehavioralConfig,
}

/// User behavioral profile with ML-enhanced analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub login_patterns: LoginPatterns,
    pub access_patterns: AccessPatterns,
    pub command_patterns: CommandPatterns,
    pub risk_score: f64,
    pub anomaly_threshold: f64,
    pub ml_features: FeatureVector,
    pub behavioral_clusters: Vec<String>,
}

/// Network traffic behavioral baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkBaseline {
    pub network_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub traffic_patterns: TrafficPatterns,
    pub connection_patterns: ConnectionPatterns,
    pub protocol_distribution: HashMap<String, f64>,
    pub geographic_patterns: GeographicPatterns,
    pub ml_features: FeatureVector,
    pub anomaly_threshold: f64,
}

/// System behavioral baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemBaseline {
    pub system_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub resource_patterns: ResourcePatterns,
    pub process_patterns: ProcessPatterns,
    pub file_access_patterns: FileAccessPatterns,
    pub performance_baseline: PerformanceBaseline,
    pub ml_features: FeatureVector,
    pub anomaly_threshold: f64,
}

/// Login pattern analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginPatterns {
    pub typical_hours: Vec<u8>,
    pub typical_days: Vec<u8>,
    pub geographic_locations: Vec<String>,
    pub device_fingerprints: Vec<String>,
    pub session_durations: Vec<u64>,
    pub failure_patterns: Vec<DateTime<Utc>>,
    pub success_rate: f64,
}

/// Access pattern analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPatterns {
    pub resource_access_frequency: HashMap<String, u64>,
    pub access_time_patterns: HashMap<String, Vec<DateTime<Utc>>>,
    pub privilege_escalation_attempts: Vec<DateTime<Utc>>,
    pub unusual_resource_access: Vec<String>,
    pub access_velocity: f64,
}

/// Command execution pattern analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPatterns {
    pub command_frequency: HashMap<String, u64>,
    pub command_sequences: Vec<Vec<String>>,
    pub administrative_commands: Vec<String>,
    pub suspicious_commands: Vec<String>,
    pub execution_timing: HashMap<String, Vec<DateTime<Utc>>>,
}

/// Network traffic patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficPatterns {
    pub bandwidth_usage: Vec<f64>,
    pub packet_sizes: Vec<u64>,
    pub connection_durations: Vec<u64>,
    pub traffic_timing: Vec<DateTime<Utc>>,
    pub protocol_usage: HashMap<String, u64>,
}

/// Connection patterns analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPatterns {
    pub source_ips: HashMap<String, u64>,
    pub destination_ips: HashMap<String, u64>,
    pub port_usage: HashMap<u16, u64>,
    pub connection_states: HashMap<String, u64>,
    pub unusual_connections: Vec<String>,
}

/// Geographic access patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicPatterns {
    pub countries: HashMap<String, u64>,
    pub cities: HashMap<String, u64>,
    pub asn_numbers: HashMap<u32, u64>,
    pub vpn_usage: Vec<DateTime<Utc>>,
    pub geographic_velocity: f64,
}

/// System resource usage patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePatterns {
    pub cpu_usage: Vec<f64>,
    pub memory_usage: Vec<f64>,
    pub disk_usage: Vec<f64>,
    pub network_usage: Vec<f64>,
    pub resource_spikes: Vec<DateTime<Utc>>,
}

/// Process execution patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessPatterns {
    pub process_names: HashMap<String, u64>,
    pub process_arguments: HashMap<String, Vec<String>>,
    pub parent_child_relationships: HashMap<String, Vec<String>>,
    pub unusual_processes: Vec<String>,
    pub process_timing: HashMap<String, Vec<DateTime<Utc>>>,
}

/// File access patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAccessPatterns {
    pub file_paths: HashMap<String, u64>,
    pub file_operations: HashMap<String, u64>,
    pub sensitive_file_access: Vec<String>,
    pub file_modification_patterns: HashMap<String, Vec<DateTime<Utc>>>,
    pub unusual_file_access: Vec<String>,
}

/// System performance baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    pub response_times: Vec<f64>,
    pub throughput: Vec<f64>,
    pub error_rates: Vec<f64>,
    pub availability: f64,
    pub performance_degradation: Vec<DateTime<Utc>>,
}

/// Cached behavioral analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedAnalysis {
    pub analysis_id: Uuid,
    pub entity_id: String,
    pub analysis_type: String,
    pub result: BehavioralAnalysisResult,
    pub cached_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

/// Behavioral analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralAnalysisResult {
    pub entity_id: String,
    pub analysis_type: String,
    pub anomaly_score: f64,
    pub risk_level: RiskLevel,
    pub detected_anomalies: Vec<DetectedAnomaly>,
    pub ml_predictions: Vec<MlPrediction>,
    pub recommended_actions: Vec<String>,
    pub confidence: f64,
    pub analyzed_at: DateTime<Utc>,
}

/// Risk level classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Detected behavioral anomaly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedAnomaly {
    pub anomaly_id: Uuid,
    pub anomaly_type: String,
    pub description: String,
    pub severity: f64,
    pub evidence: Vec<String>,
    pub detected_at: DateTime<Utc>,
    pub ml_confidence: Option<f64>,
}

/// Behavioral analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralConfig {
    pub user_analysis_enabled: bool,
    pub network_analysis_enabled: bool,
    pub system_analysis_enabled: bool,
    pub ml_analysis_enabled: bool,
    pub cache_ttl_seconds: u64,
    pub anomaly_threshold: f64,
    pub update_interval_seconds: u64,
    pub max_profiles: usize,
}

impl BehavioralAnalyzer {
    /// Create new behavioral analyzer
    pub fn new(cue_engine: Arc<CueRuleEngine>, config: BehavioralConfig) -> Self {
        Self {
            id: Uuid::new_v4(),
            cue_engine,
            user_profiles: Arc::new(RwLock::new(HashMap::new())),
            network_baselines: Arc::new(RwLock::new(HashMap::new())),
            system_baselines: Arc::new(RwLock::new(HashMap::new())),
            ml_models: Arc::new(RwLock::new(HashMap::new())),
            analysis_cache: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Analyze user behavior with ML enhancement
    pub async fn analyze_user_behavior(
        &self,
        user_id: &str,
        current_activity: &UserActivity,
    ) -> Result<BehavioralAnalysisResult> {
        // Check cache first
        let cache_key = format!("user_{}_{}", user_id, current_activity.activity_type);
        if let Some(cached) = self.get_cached_analysis(&cache_key).await? {
            return Ok(cached.result);
        }

        let mut profiles = self.user_profiles.write().await;
        let profile = profiles.entry(user_id.to_string())
            .or_insert_with(|| self.create_default_user_profile(user_id));

        // Update profile with current activity
        self.update_user_profile(profile, current_activity).await?;

        // Calculate anomaly score
        let anomaly_score = self.calculate_user_anomaly_score(profile, current_activity).await?;

        // Apply ML models if available
        let ml_predictions = if self.config.ml_analysis_enabled {
            self.apply_ml_models_to_user(profile, current_activity).await?
        } else {
            Vec::new()
        };

        // Detect specific anomalies
        let detected_anomalies = self.detect_user_anomalies(profile, current_activity, anomaly_score).await?;

        // Determine risk level
        let risk_level = self.calculate_risk_level(anomaly_score, &ml_predictions);

        // Generate recommended actions
        let recommended_actions = self.generate_user_recommendations(&detected_anomalies, &risk_level).await?;

        let result = BehavioralAnalysisResult {
            entity_id: user_id.to_string(),
            analysis_type: "user_behavior".to_string(),
            anomaly_score,
            risk_level,
            detected_anomalies: detected_anomalies.clone(),
            ml_predictions,
            recommended_actions,
            confidence: self.calculate_confidence(&detected_anomalies),
            analyzed_at: Utc::now(),
        };

        // Cache result
        self.cache_analysis(&cache_key, &result).await?;

        Ok(result)
    }

    /// Analyze network behavior with ML enhancement
    pub async fn analyze_network_behavior(
        &self,
        network_id: &str,
        current_traffic: &NetworkTraffic,
    ) -> Result<BehavioralAnalysisResult> {
        let cache_key = format!("network_{}_{}", network_id, current_traffic.traffic_type);
        if let Some(cached) = self.get_cached_analysis(&cache_key).await? {
            return Ok(cached.result);
        }

        let mut baselines = self.network_baselines.write().await;
        let baseline = baselines.entry(network_id.to_string())
            .or_insert_with(|| self.create_default_network_baseline(network_id));

        // Update baseline with current traffic
        self.update_network_baseline(baseline, current_traffic).await?;

        // Calculate anomaly score
        let anomaly_score = self.calculate_network_anomaly_score(baseline, current_traffic).await?;

        // Apply ML models
        let ml_predictions = if self.config.ml_analysis_enabled {
            self.apply_ml_models_to_network(baseline, current_traffic).await?
        } else {
            Vec::new()
        };

        // Detect network anomalies
        let detected_anomalies = self.detect_network_anomalies(baseline, current_traffic, anomaly_score).await?;

        let risk_level = self.calculate_risk_level(anomaly_score, &ml_predictions);
        let recommended_actions = self.generate_network_recommendations(&detected_anomalies, &risk_level).await?;

        let result = BehavioralAnalysisResult {
            entity_id: network_id.to_string(),
            analysis_type: "network_behavior".to_string(),
            anomaly_score,
            risk_level,
            detected_anomalies: detected_anomalies.clone(),
            ml_predictions,
            recommended_actions,
            confidence: self.calculate_confidence(&detected_anomalies),
            analyzed_at: Utc::now(),
        };

        self.cache_analysis(&cache_key, &result).await?;
        Ok(result)
    }

    /// Analyze system behavior with ML enhancement
    pub async fn analyze_system_behavior(
        &self,
        system_id: &str,
        current_state: &SystemState,
    ) -> Result<BehavioralAnalysisResult> {
        let cache_key = format!("system_{}_{}", system_id, current_state.state_type);
        if let Some(cached) = self.get_cached_analysis(&cache_key).await? {
            return Ok(cached.result);
        }

        let mut baselines = self.system_baselines.write().await;
        let baseline = baselines.entry(system_id.to_string())
            .or_insert_with(|| self.create_default_system_baseline(system_id));

        // Update baseline with current state
        self.update_system_baseline(baseline, current_state).await?;

        // Calculate anomaly score
        let anomaly_score = self.calculate_system_anomaly_score(baseline, current_state).await?;

        // Apply ML models
        let ml_predictions = if self.config.ml_analysis_enabled {
            self.apply_ml_models_to_system(baseline, current_state).await?
        } else {
            Vec::new()
        };

        // Detect system anomalies
        let detected_anomalies = self.detect_system_anomalies(baseline, current_state, anomaly_score).await?;

        let risk_level = self.calculate_risk_level(anomaly_score, &ml_predictions);
        let recommended_actions = self.generate_system_recommendations(&detected_anomalies, &risk_level).await?;

        let result = BehavioralAnalysisResult {
            entity_id: system_id.to_string(),
            analysis_type: "system_behavior".to_string(),
            anomaly_score,
            risk_level,
            detected_anomalies: detected_anomalies.clone(),
            ml_predictions,
            recommended_actions,
            confidence: self.calculate_confidence(&detected_anomalies),
            analyzed_at: Utc::now(),
        };

        self.cache_analysis(&cache_key, &result).await?;
        Ok(result)
    }

    /// Register ML model for behavioral analysis
    pub async fn register_ml_model(
        &self,
        model_name: String,
        model: Box<dyn MlModel + Send + Sync>,
    ) -> Result<()> {
        let mut models = self.ml_models.write().await;
        models.insert(model_name, model);
        Ok(())
    }

    /// Get cached analysis result
    async fn get_cached_analysis(&self, cache_key: &str) -> Result<Option<CachedAnalysis>> {
        let cache = self.analysis_cache.read().await;
        if let Some(cached) = cache.get(cache_key) {
            if cached.expires_at > Utc::now() {
                return Ok(Some(cached.clone()));
            }
        }
        Ok(None)
    }

    /// Cache analysis result
    async fn cache_analysis(&self, cache_key: &str, result: &BehavioralAnalysisResult) -> Result<()> {
        let cached = CachedAnalysis {
            analysis_id: Uuid::new_v4(),
            entity_id: result.entity_id.clone(),
            analysis_type: result.analysis_type.clone(),
            result: result.clone(),
            cached_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::seconds(self.config.cache_ttl_seconds as i64),
        };

        let mut cache = self.analysis_cache.write().await;
        cache.insert(cache_key.to_string(), cached);
        Ok(())
    }

    /// Create default user profile
    fn create_default_user_profile(&self, user_id: &str) -> UserProfile {
        UserProfile {
            user_id: user_id.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            login_patterns: LoginPatterns {
                typical_hours: Vec::new(),
                typical_days: Vec::new(),
                geographic_locations: Vec::new(),
                device_fingerprints: Vec::new(),
                session_durations: Vec::new(),
                failure_patterns: Vec::new(),
                success_rate: 1.0,
            },
            access_patterns: AccessPatterns {
                resource_access_frequency: HashMap::new(),
                access_time_patterns: HashMap::new(),
                privilege_escalation_attempts: Vec::new(),
                unusual_resource_access: Vec::new(),
                access_velocity: 0.0,
            },
            command_patterns: CommandPatterns {
                command_frequency: HashMap::new(),
                command_sequences: Vec::new(),
                administrative_commands: Vec::new(),
                suspicious_commands: Vec::new(),
                execution_timing: HashMap::new(),
            },
            risk_score: 0.0,
            anomaly_threshold: self.config.anomaly_threshold,
            ml_features: FeatureVector::new(),
            behavioral_clusters: Vec::new(),
        }
    }

    /// Create default network baseline
    fn create_default_network_baseline(&self, network_id: &str) -> NetworkBaseline {
        NetworkBaseline {
            network_id: network_id.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            traffic_patterns: TrafficPatterns {
                bandwidth_usage: Vec::new(),
                packet_sizes: Vec::new(),
                connection_durations: Vec::new(),
                traffic_timing: Vec::new(),
                protocol_usage: HashMap::new(),
            },
            connection_patterns: ConnectionPatterns {
                source_ips: HashMap::new(),
                destination_ips: HashMap::new(),
                port_usage: HashMap::new(),
                connection_states: HashMap::new(),
                unusual_connections: Vec::new(),
            },
            protocol_distribution: HashMap::new(),
            geographic_patterns: GeographicPatterns {
                countries: HashMap::new(),
                cities: HashMap::new(),
                asn_numbers: HashMap::new(),
                vpn_usage: Vec::new(),
                geographic_velocity: 0.0,
            },
            ml_features: FeatureVector::new(),
            anomaly_threshold: self.config.anomaly_threshold,
        }
    }

    /// Update user profile with current activity
    async fn update_user_profile(&self, _profile: &mut UserProfile, _current_activity: &UserActivity) -> Result<()> {
        // Implementation placeholder - update profile with current activity
        Ok(())
    }

    /// Calculate user anomaly score
    async fn calculate_user_anomaly_score(&self, _profile: &UserProfile, _current_activity: &UserActivity) -> Result<f64> {
        // Implementation placeholder - calculate anomaly score
        Ok(0.5)
    }

    /// Apply ML models to user analysis
    async fn apply_ml_models_to_user(&self, _profile: &UserProfile, _current_activity: &UserActivity) -> Result<Vec<MlPrediction>> {
        // Implementation placeholder - apply ML models
        Ok(Vec::new())
    }

    /// Detect user anomalies
    async fn detect_user_anomalies(&self, _profile: &UserProfile, _current_activity: &UserActivity, _anomaly_score: f64) -> Result<Vec<DetectedAnomaly>> {
        // Implementation placeholder - detect anomalies
        Ok(Vec::new())
    }

    /// Calculate risk level
    fn calculate_risk_level(&self, _anomaly_score: f64, _ml_predictions: &[MlPrediction]) -> RiskLevel {
        // Implementation placeholder - calculate risk level
        RiskLevel::Low
    }

    /// Generate user recommendations
    async fn generate_user_recommendations(&self, _detected_anomalies: &[DetectedAnomaly], _risk_level: &RiskLevel) -> Result<Vec<String>> {
        // Implementation placeholder - generate recommendations
        Ok(Vec::new())
    }

    /// Calculate confidence
    fn calculate_confidence(&self, _detected_anomalies: &[DetectedAnomaly]) -> f64 {
        // Implementation placeholder - calculate confidence
        0.8
    }

    /// Update network baseline
    async fn update_network_baseline(&self, _baseline: &mut NetworkBaseline, _current_traffic: &NetworkTraffic) -> Result<()> {
        // Implementation placeholder - update network baseline
        Ok(())
    }

    /// Calculate network anomaly score
    async fn calculate_network_anomaly_score(&self, _baseline: &NetworkBaseline, _current_traffic: &NetworkTraffic) -> Result<f64> {
        // Implementation placeholder - calculate network anomaly score
        Ok(0.5)
    }

    /// Apply ML models to network analysis
    async fn apply_ml_models_to_network(&self, _baseline: &NetworkBaseline, _current_traffic: &NetworkTraffic) -> Result<Vec<MlPrediction>> {
        // Implementation placeholder - apply ML models to network
        Ok(Vec::new())
    }

    /// Detect network anomalies
    async fn detect_network_anomalies(&self, _baseline: &NetworkBaseline, _current_traffic: &NetworkTraffic, _anomaly_score: f64) -> Result<Vec<DetectedAnomaly>> {
        // Implementation placeholder - detect network anomalies
        Ok(Vec::new())
    }

    /// Generate network recommendations
    async fn generate_network_recommendations(&self, _detected_anomalies: &[DetectedAnomaly], _risk_level: &RiskLevel) -> Result<Vec<String>> {
        // Implementation placeholder - generate network recommendations
        Ok(Vec::new())
    }

    /// Update system baseline
    async fn update_system_baseline(&self, _baseline: &mut SystemBaseline, _current_state: &SystemState) -> Result<()> {
        // Implementation placeholder - update system baseline
        Ok(())
    }

    /// Calculate system anomaly score
    async fn calculate_system_anomaly_score(&self, _baseline: &SystemBaseline, _current_state: &SystemState) -> Result<f64> {
        // Implementation placeholder - calculate system anomaly score
        Ok(0.5)
    }

    /// Apply ML models to system analysis
    async fn apply_ml_models_to_system(&self, _baseline: &SystemBaseline, _current_state: &SystemState) -> Result<Vec<MlPrediction>> {
        // Implementation placeholder - apply ML models to system
        Ok(Vec::new())
    }

    /// Detect system anomalies
    async fn detect_system_anomalies(&self, _baseline: &SystemBaseline, _current_state: &SystemState, _anomaly_score: f64) -> Result<Vec<DetectedAnomaly>> {
        // Implementation placeholder - detect system anomalies
        Ok(Vec::new())
    }

    /// Generate system recommendations
    async fn generate_system_recommendations(&self, _detected_anomalies: &[DetectedAnomaly], _risk_level: &RiskLevel) -> Result<Vec<String>> {
        // Implementation placeholder - generate system recommendations
        Ok(Vec::new())
    }

    /// Create default system baseline
    fn create_default_system_baseline(&self, system_id: &str) -> SystemBaseline {
        SystemBaseline {
            system_id: system_id.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            resource_patterns: ResourcePatterns {
                cpu_usage: Vec::new(),
                memory_usage: Vec::new(),
                disk_usage: Vec::new(),
                network_usage: Vec::new(),
                resource_spikes: Vec::new(),
            },
            process_patterns: ProcessPatterns {
                process_names: HashMap::new(),
                process_arguments: HashMap::new(),
                parent_child_relationships: HashMap::new(),
                unusual_processes: Vec::new(),
                process_timing: HashMap::new(),
            },
            file_access_patterns: FileAccessPatterns {
                file_paths: HashMap::new(),
                file_operations: HashMap::new(),
                sensitive_file_access: Vec::new(),
                file_modification_patterns: HashMap::new(),
                unusual_file_access: Vec::new(),
            },
            performance_baseline: PerformanceBaseline {
                response_times: Vec::new(),
                throughput: Vec::new(),
                error_rates: Vec::new(),
                availability: 1.0,
                performance_degradation: Vec::new(),
            },
            ml_features: FeatureVector::new(),
            anomaly_threshold: self.config.anomaly_threshold,
        }
    }

    // Helper methods would continue here...
    // (Implementation of update methods, anomaly detection, ML integration, etc.)
}

/// User activity for behavioral analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserActivity {
    pub activity_type: String,
    pub timestamp: DateTime<Utc>,
    pub source_ip: String,
    pub user_agent: String,
    pub resource_accessed: String,
    pub action_performed: String,
    pub success: bool,
    pub metadata: HashMap<String, String>,
}

/// Network traffic for behavioral analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTraffic {
    pub traffic_type: String,
    pub timestamp: DateTime<Utc>,
    pub source_ip: String,
    pub destination_ip: String,
    pub source_port: u16,
    pub destination_port: u16,
    pub protocol: String,
    pub bytes_transferred: u64,
    pub duration: u64,
    pub metadata: HashMap<String, String>,
}

/// System state for behavioral analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    pub state_type: String,
    pub timestamp: DateTime<Utc>,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_usage: f64,
    pub active_processes: Vec<String>,
    pub recent_file_access: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl Default for BehavioralConfig {
    fn default() -> Self {
        Self {
            user_analysis_enabled: true,
            network_analysis_enabled: true,
            system_analysis_enabled: true,
            ml_analysis_enabled: true,
            cache_ttl_seconds: 300, // 5 minutes
            anomaly_threshold: 0.7,
            update_interval_seconds: 60,
            max_profiles: 10000,
        }
    }
}
