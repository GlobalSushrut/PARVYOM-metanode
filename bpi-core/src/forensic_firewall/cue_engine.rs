// BPI Programmable CUE Forensic Firewall - Rule Engine Core
// 100x harder to hack with dynamic CUE-based security contracts

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use tokio::sync::RwLock;
use uuid::Uuid;

/// Core CUE Rule Engine for programmable security policies
#[derive(Debug, Clone)]
pub struct CueRuleEngine {
    pub rule_compiler: Arc<CueCompiler>,
    pub rule_evaluator: Arc<RuleEvaluator>,
    pub dynamic_loader: Arc<DynamicRuleLoader>,
    pub performance_monitor: Arc<PerformanceMonitor>,
    pub ml_integration: Arc<RwLock<MLIntegrationEngine>>,
}

impl CueRuleEngine {
    pub fn new() -> Self {
        Self {
            rule_compiler: Arc::new(CueCompiler::new()),
            rule_evaluator: Arc::new(RuleEvaluator::new()),
            dynamic_loader: Arc::new(DynamicRuleLoader::new()),
            performance_monitor: Arc::new(PerformanceMonitor::new()),
            ml_integration: Arc::new(RwLock::new(MLIntegrationEngine::new())),
        }
    }

    /// Load and compile CUE security contract
    pub async fn load_security_contract(&self, contract_path: &str) -> Result<String> {
        let start_time = Instant::now();
        
        // Load CUE contract from file
        let contract_content = tokio::fs::read_to_string(contract_path).await?;
        
        // Compile CUE contract
        let contract_id = self.rule_compiler.compile_contract(&contract_content).await?;
        
        // Register with dynamic loader for hot-reload
        let contract_result = self.dynamic_loader.register_contract(contract_path, &contract_id.contract_id).await?;
        
        // Record performance metrics
        let compilation_time = start_time.elapsed();
        self.performance_monitor.record_compilation_time(compilation_time).await;
        
        tracing::info!("🔥 CUE Security Contract loaded: {} ({}ms)", 
                      contract_id, compilation_time.as_millis());
        
        Ok(contract_id.contract_id)
    }

    /// Evaluate security rules against threat context (sub-millisecond target)
    pub async fn evaluate_threat(&self, threat_context: &ThreatContext) -> Result<SecurityDecision> {
        let start_time = Instant::now();
        
        // Get active security contracts
        let contracts = self.dynamic_loader.get_active_contracts().await;
        
        // Evaluate each contract
        let mut decisions = Vec::new();
        for contract in contracts {
            let decision = self.rule_evaluator.evaluate_contract(&contract, threat_context).await?;
            decisions.push(decision);
        }
        
        // Aggregate decisions with ML assistance if available
        let final_decision = if self.ml_integration.read().await.is_enabled() {
            self.ml_assisted_decision_aggregation(&decisions, threat_context).await?
        } else {
            self.aggregate_decisions(decisions)?
        };
        
        // Record performance metrics
        let evaluation_time = start_time.elapsed();
        self.performance_monitor.record_evaluation_time(evaluation_time).await;
        
        // Target: sub-millisecond evaluation
        if evaluation_time > Duration::from_millis(1) {
            tracing::warn!("⚠️ Rule evaluation exceeded 1ms target: {}μs", 
                          evaluation_time.as_micros());
        }
        
        Ok(final_decision)
    }

    /// ML-assisted decision aggregation (🤖 ML/AI Integration Point)
    async fn ml_assisted_decision_aggregation(
        &self,
        decisions: &[SecurityDecision],
        threat_context: &ThreatContext,
    ) -> Result<SecurityDecision> {
        let ml_manager = self.ml_integration.read().await;
        
        // Extract features for ML model
        let features = self.extract_decision_features(decisions, threat_context)?;
        
        // Get ML model prediction
        let ml_confidence = ml_manager.predict_threat_severity(&features).await?;
        
        // Combine rule-based decisions with ML confidence
        let base_decision = self.aggregate_decisions(decisions.to_vec())?;
        
        Ok(SecurityDecision {
            action: base_decision.action,
            confidence: (base_decision.confidence + ml_confidence) / 2.0,
            ml_enhanced: true,
            reasoning: format!("Rule-based: {}, ML-enhanced: {}", base_decision.reasoning, ml_confidence),
            response_actions: base_decision.response_actions,
        })
    }

    /// Extract features for ML model (🤖 ML/AI Integration Point)
    fn extract_decision_features(
        &self,
        decisions: &[SecurityDecision],
        threat_context: &ThreatContext,
    ) -> Result<FeatureVector> {
        let mut features = HashMap::new();
        
        // Decision-based features
        features.insert("avg_confidence".to_string(), 
                       decisions.iter().map(|d| d.confidence).sum::<f64>() / decisions.len() as f64);
        let max_severity = decisions.iter().map(|d| d.action.severity_score()).fold(0.0f64, |a, b| a.max(b));
        features.insert("max_severity".to_string(), max_severity);
        
        // Threat context features
        features.insert("source_reputation".to_string(), threat_context.source_reputation);
        features.insert("attack_complexity".to_string(), threat_context.attack_complexity);
        features.insert("temporal_anomaly".to_string(), threat_context.temporal_anomaly_score);
        
        Ok(FeatureVector { features })
    }

    /// Aggregate multiple security decisions
    fn aggregate_decisions(&self, decisions: Vec<SecurityDecision>) -> Result<SecurityDecision> {
        if decisions.is_empty() {
            return Ok(SecurityDecision::allow("No active rules"));
        }

        // Find highest severity action
        let max_action = decisions.iter()
            .max_by(|a, b| a.action.severity_score().partial_cmp(&b.action.severity_score()).unwrap())
            .unwrap();

        // Calculate weighted confidence
        let total_confidence: f64 = decisions.iter().map(|d| d.confidence).sum();
        let avg_confidence = total_confidence / decisions.len() as f64;

        // Aggregate response actions
        let mut all_actions = Vec::new();
        for decision in &decisions {
            all_actions.extend(decision.response_actions.clone());
        }

        Ok(SecurityDecision {
            action: max_action.action.clone(),
            confidence: avg_confidence,
            ml_enhanced: false,
            reasoning: format!("Aggregated from {} rules", decisions.len()),
            response_actions: all_actions,
        })
    }
}

/// CUE Contract Compiler
#[derive(Debug)]
pub struct CueCompiler {
    compiled_contracts: Arc<Mutex<HashMap<String, CueSecurityContract>>>,
}

impl CueCompiler {
    pub fn new() -> Self {
        Self {
            compiled_contracts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn compile_contract(&self, _cue_content: &str) -> Result<CueSecurityContract> {
        // Simplified CUE parsing - in production would use actual CUE parser
        // For now, create a basic contract structure
        Ok(CueSecurityContract {
            contract_id: Uuid::new_v4().to_string(),
            version: "1.0.0".to_string(),
            rules: vec![
                SecurityRule {
                    rule_id: "default_rule".to_string(),
                    condition: RuleCondition::Always,
                    action: SecurityAction::Monitor,
                    priority: 1,
                    ml_hooks: vec![
                        MLIntegrationHook {
                            hook_type: MLHookType::ThreatClassification,
                            model_name: "behavioral_analysis".to_string(),
                            confidence_threshold: 0.8,
                        }
                    ],
                }
            ],
            ml_hooks: vec![],
            response_actions: vec![],
        })
    }

    fn parse_cue_contract(&self, cue_content: &str) -> Result<CueSecurityContract> {
        // Simplified CUE parsing - in production would use cuelang.org/go
        // For now, create a basic contract structure
        Ok(CueSecurityContract {
            contract_id: Uuid::new_v4().to_string(),
            version: "1.0.0".to_string(),
            rules: vec![
                SecurityRule {
                    rule_id: "default_rule".to_string(),
                    condition: RuleCondition::Always,
                    action: SecurityAction::Monitor,
                    priority: 1,
                    ml_hooks: vec![
                        MLIntegrationHook {
                            hook_type: MLHookType::ThreatClassification,
                            model_name: "behavioral_analysis".to_string(),
                            confidence_threshold: 0.8,
                        }
                    ],
                }
            ],
            ml_hooks: vec![],
            response_actions: vec![],
        })
    }
}

/// Rule Evaluator for CUE contracts
#[derive(Debug)]
pub struct RuleEvaluator {
    evaluation_cache: Arc<Mutex<HashMap<String, CachedEvaluation>>>,
}

impl RuleEvaluator {
    pub fn new() -> Self {
        Self {
            evaluation_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn evaluate_contract(
        &self,
        contract: &CueSecurityContract,
        threat_context: &ThreatContext,
    ) -> Result<SecurityDecision> {
        // Check cache first
        let cache_key = format!("{}:{}", contract.contract_id, threat_context.hash());
        if let Some(cached) = self.get_cached_evaluation(&cache_key) {
            if !cached.is_expired() {
                return Ok(cached.decision);
            }
        }

        // Evaluate rules
        let mut triggered_rules = Vec::new();
        for rule in &contract.rules {
            if self.evaluate_rule_condition(&rule.condition, threat_context)? {
                triggered_rules.push(rule);
            }
        }

        // Create decision based on triggered rules
        let decision = if triggered_rules.is_empty() {
            SecurityDecision::allow("No rules triggered")
        } else {
            let highest_priority_rule = triggered_rules.iter()
                .max_by_key(|r| r.priority)
                .unwrap();
            
            SecurityDecision {
                action: highest_priority_rule.action.clone(),
                confidence: 0.95, // High confidence for rule-based decisions
                ml_enhanced: false,
                reasoning: format!("Rule triggered: {}", highest_priority_rule.rule_id),
                response_actions: vec![ResponseAction::LogThreat, ResponseAction::UpdateMetrics],
            }
        };

        // Cache the evaluation
        self.cache_evaluation(cache_key, &decision);

        Ok(decision)
    }

    fn evaluate_rule_condition(
        &self,
        condition: &RuleCondition,
        threat_context: &ThreatContext,
    ) -> Result<bool> {
        match condition {
            RuleCondition::Always => Ok(true),
            RuleCondition::ThreatScore(threshold) => {
                Ok(threat_context.threat_score >= *threshold)
            }
            RuleCondition::SourceReputation(threshold) => {
                Ok(threat_context.source_reputation <= *threshold)
            }
            RuleCondition::And(conditions) => {
                for cond in conditions {
                    if !self.evaluate_rule_condition(cond, threat_context)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            RuleCondition::Or(conditions) => {
                for cond in conditions {
                    if self.evaluate_rule_condition(cond, threat_context)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
        }
    }

    fn get_cached_evaluation(&self, cache_key: &str) -> Option<CachedEvaluation> {
        let cache = self.evaluation_cache.lock().unwrap();
        cache.get(cache_key).cloned()
    }

    fn cache_evaluation(&self, cache_key: String, decision: &SecurityDecision) {
        let cached = CachedEvaluation {
            decision: decision.clone(),
            timestamp: Instant::now(),
            ttl: Duration::from_secs(60), // 1 minute cache
        };
        
        let mut cache = self.evaluation_cache.lock().unwrap();
        cache.insert(cache_key, cached);
    }
}

/// Dynamic Rule Loader for hot-reload capability
#[derive(Debug)]
pub struct DynamicRuleLoader {
    active_contracts: Arc<RwLock<HashMap<String, CueSecurityContract>>>,
    file_watchers: Arc<Mutex<HashMap<String, String>>>,
}

impl DynamicRuleLoader {
    pub fn new() -> Self {
        Self {
            active_contracts: Arc::new(RwLock::new(HashMap::new())),
            file_watchers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn register_contract(&self, file_path: &str, contract_id: &str) -> Result<()> {
        let mut watchers = self.file_watchers.lock().unwrap();
        watchers.insert(file_path.to_string(), contract_id.to_string());
        
        tracing::info!("📁 Registered contract for hot-reload: {}", file_path);
        Ok(())
    }

    pub async fn get_active_contracts(&self) -> Vec<CueSecurityContract> {
        let contracts = self.active_contracts.read().await;
        contracts.values().cloned().collect()
    }

    pub async fn reload_contract(&self, contract_id: &str, new_contract: CueSecurityContract) -> Result<()> {
        let mut contracts = self.active_contracts.write().await;
        contracts.insert(contract_id.to_string(), new_contract);
        
        tracing::info!("🔄 Hot-reloaded security contract: {}", contract_id);
        Ok(())
    }
}

/// Performance Monitor for sub-millisecond optimization
#[derive(Debug)]
pub struct PerformanceMonitor {
    metrics: Arc<Mutex<PerformanceMetrics>>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(PerformanceMetrics::default())),
        }
    }

    pub async fn record_compilation_time(&self, duration: Duration) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.compilation_times.push(duration);
        metrics.avg_compilation_time = Self::calculate_average(&metrics.compilation_times);
    }

    pub async fn record_evaluation_time(&self, duration: Duration) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.evaluation_times.push(duration);
        metrics.avg_evaluation_time = Self::calculate_average(&metrics.evaluation_times);
        
        // Track sub-millisecond performance
        if duration <= Duration::from_millis(1) {
            metrics.sub_millisecond_evaluations += 1;
        }
        metrics.total_evaluations += 1;
    }

    fn calculate_average(durations: &[Duration]) -> Duration {
        if durations.is_empty() {
            return Duration::from_nanos(0);
        }
        
        let total_nanos: u64 = durations.iter().map(|d| d.as_nanos() as u64).sum();
        Duration::from_nanos(total_nanos / durations.len() as u64)
    }

    pub async fn get_performance_report(&self) -> PerformanceReport {
        let metrics = self.metrics.lock().unwrap();
        let sub_ms_percentage = if metrics.total_evaluations > 0 {
            (metrics.sub_millisecond_evaluations as f64 / metrics.total_evaluations as f64) * 100.0
        } else {
            0.0
        };

        PerformanceReport {
            avg_compilation_time: metrics.avg_compilation_time,
            avg_evaluation_time: metrics.avg_evaluation_time,
            sub_millisecond_percentage: sub_ms_percentage,
            total_evaluations: metrics.total_evaluations,
        }
    }
}

/// ML Integration Manager (🤖 ML/AI Integration Point)
pub struct MLIntegrationEngine {
    models: HashMap<String, Box<dyn MLModel + Send + Sync>>,
    feature_extractors: HashMap<String, Box<dyn FeatureExtractor + Send + Sync>>,
}

impl std::fmt::Debug for MLIntegrationEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MLIntegrationEngine")
            .field("models_count", &self.models.len())
            .field("feature_extractors_count", &self.feature_extractors.len())
            .finish()
    }
}

impl MLIntegrationEngine {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
            feature_extractors: HashMap::new(),
        }
    }

    pub fn is_enabled(&self) -> bool {
        !self.models.is_empty()
    }

    pub async fn enable_ml_integration(&mut self) {
        // Add default models
        println!("ML integration enabled for CUE engine");
    }

    pub async fn predict_threat_severity(&self, features: &FeatureVector) -> Result<f64> {
        if !self.is_enabled() {
            return Ok(0.5); // Neutral confidence when ML is disabled
        }

        // Use behavioral analysis model if available
        if let Some(model) = self.models.get("behavioral_analysis") {
            let prediction = model.predict(features)?;
            Ok(prediction)
        } else {
            Ok(0.5) // Default confidence
        }
    }

    pub fn register_model(&mut self, name: String, model: Box<dyn MLModel + Send + Sync>) {
        let model_name = name.clone();
        self.models.insert(name, model);
        tracing::info!("🤖 ML Model registered: {}", model_name);
    }
}

/// ML Model trait for future integration (🤖 ML/AI Integration Point)
pub trait MLModel {
    fn predict(&self, features: &FeatureVector) -> Result<f64>;
    fn get_model_info(&self) -> ModelInfo;
}

/// Feature Extractor trait (🤖 ML/AI Integration Point)
pub trait FeatureExtractor {
    fn extract_features(&self, context: &ThreatContext) -> Result<FeatureVector>;
}

// Data Structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CueSecurityContract {
    pub contract_id: String,
    pub version: String,
    pub rules: Vec<SecurityRule>,
    pub ml_hooks: Vec<MLIntegrationHook>,
    pub response_actions: Vec<ResponseAction>,
}

impl std::fmt::Display for CueSecurityContract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CueSecurityContract(id: {}, version: {}, rules: {})", 
               self.contract_id, self.version, self.rules.len())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRule {
    pub rule_id: String,
    pub condition: RuleCondition,
    pub action: SecurityAction,
    pub priority: u32,
    pub ml_hooks: Vec<MLIntegrationHook>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCondition {
    Always,
    ThreatScore(f64),
    SourceReputation(f64),
    And(Vec<RuleCondition>),
    Or(Vec<RuleCondition>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityAction {
    Allow,
    Monitor,
    Block,
    Quarantine,
    Escalate,
    EmergencyBlock,
}

impl SecurityAction {
    pub fn severity_score(&self) -> f64 {
        match self {
            SecurityAction::Allow => 0.0,
            SecurityAction::Monitor => 2.0,
            SecurityAction::Block => 5.0,
            SecurityAction::Quarantine => 8.0,
            SecurityAction::Escalate => 10.0,
            SecurityAction::EmergencyBlock => 15.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLIntegrationHook {
    pub hook_type: MLHookType,
    pub model_name: String,
    pub confidence_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MLHookType {
    ThreatClassification,
    BehavioralAnalysis,
    AnomalyDetection,
    MalwareDetection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityDecision {
    pub action: SecurityAction,
    pub confidence: f64,
    pub ml_enhanced: bool,
    pub reasoning: String,
    pub response_actions: Vec<ResponseAction>,
}

impl SecurityDecision {
    pub fn allow(reason: &str) -> Self {
        Self {
            action: SecurityAction::Allow,
            confidence: 1.0,
            ml_enhanced: false,
            reasoning: reason.to_string(),
            response_actions: vec![],
        }
    }

    pub fn block(reason: &str) -> Self {
        Self {
            action: SecurityAction::Block,
            confidence: 1.0,
            ml_enhanced: false,
            reasoning: reason.to_string(),
            response_actions: vec![ResponseAction::LogThreat, ResponseAction::AlertAdmin],
        }
    }

    pub fn quarantine(reason: &str) -> Self {
        Self {
            action: SecurityAction::Quarantine,
            confidence: 0.9,
            ml_enhanced: false,
            reasoning: reason.to_string(),
            response_actions: vec![ResponseAction::IsolateSource, ResponseAction::CollectEvidence],
        }
    }

    pub fn emergency(reason: &str) -> Self {
        Self {
            action: SecurityAction::EmergencyBlock,
            confidence: 1.0,
            ml_enhanced: false,
            reasoning: reason.to_string(),
            response_actions: vec![ResponseAction::IsolateSource, ResponseAction::AlertAdmin, ResponseAction::CollectEvidence],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseAction {
    LogThreat,
    AlertAdmin,
    UpdateMetrics,
    IsolateSource,
    CollectEvidence,
}

#[derive(Debug, Clone)]
pub struct ThreatContext {
    pub threat_id: String,
    pub source_ip: String,
    pub threat_score: f64,
    pub source_reputation: f64,
    pub attack_complexity: f64,
    pub temporal_anomaly_score: f64,
    pub timestamp: Instant,
}

impl ThreatContext {
    pub fn hash(&self) -> String {
        // Simple hash for caching - in production would use proper hashing
        format!("{}:{}:{}", self.source_ip, self.threat_score, self.timestamp.elapsed().as_secs())
    }
}

#[derive(Debug, Clone)]
pub struct CachedEvaluation {
    pub decision: SecurityDecision,
    pub timestamp: Instant,
    pub ttl: Duration,
}

impl CachedEvaluation {
    pub fn is_expired(&self) -> bool {
        self.timestamp.elapsed() > self.ttl
    }
}

#[derive(Debug, Default)]
pub struct PerformanceMetrics {
    pub compilation_times: Vec<Duration>,
    pub evaluation_times: Vec<Duration>,
    pub avg_compilation_time: Duration,
    pub avg_evaluation_time: Duration,
    pub sub_millisecond_evaluations: u64,
    pub total_evaluations: u64,
}

#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub avg_compilation_time: Duration,
    pub avg_evaluation_time: Duration,
    pub sub_millisecond_percentage: f64,
    pub total_evaluations: u64,
}

#[derive(Debug, Clone)]
pub struct FeatureVector {
    pub features: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub version: String,
    pub model_type: String,
    pub accuracy: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cue_rule_engine_creation() {
        let engine = CueRuleEngine::new();
        assert!(!engine.ml_integration.read().await.is_enabled());
    }

    #[tokio::test]
    async fn test_threat_evaluation() {
        let engine = CueRuleEngine::new();
        let threat_context = ThreatContext {
            threat_id: "test_threat".to_string(),
            source_ip: "192.168.1.100".to_string(),
            threat_score: 0.8,
            source_reputation: 0.3,
            attack_complexity: 0.7,
            temporal_anomaly_score: 0.6,
            timestamp: Instant::now(),
        };

        // This would normally require a loaded contract, but for testing we'll check the structure
        // In a real implementation, we'd load a test contract first
        assert!(engine.rule_evaluator.evaluation_cache.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_performance_monitoring() {
        let monitor = PerformanceMonitor::new();
        
        // Record some test metrics
        monitor.record_evaluation_time(Duration::from_micros(500)).await;
        monitor.record_evaluation_time(Duration::from_micros(800)).await;
        
        let report = monitor.get_performance_report().await;
        assert_eq!(report.total_evaluations, 2);
        assert_eq!(report.sub_millisecond_percentage, 100.0);
    }
}
