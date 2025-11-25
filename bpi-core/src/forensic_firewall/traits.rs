// Forensic firewall traits to eliminate circular dependencies
// This module defines common interfaces used across forensic modules

use std::collections::HashMap;
use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::forensic_firewall::shared_types::{
    ForensicEventType, ForensicSeverity, SecurityAction, ResponseType,
    ThreatLevel, AnalysisStatus, CorrelationId, ForensicResult
};

/// Trait for behavioral analysis results
#[derive(Debug, Clone)]
pub struct BehavioralAnalysisResult {
    pub analysis_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub status: AnalysisStatus,
    pub anomaly_score: f64,
    pub detected_anomalies: Vec<DetectedAnomaly>,
    pub confidence_level: f64,
    pub correlation_id: CorrelationId,
}

/// Detected anomaly information
#[derive(Debug, Clone)]
pub struct DetectedAnomaly {
    pub anomaly_type: String,
    pub severity: ForensicSeverity,
    pub description: String,
    pub confidence: f64,
    pub affected_resources: Vec<String>,
}

/// Security decision from CUE engine
#[derive(Debug, Clone)]
pub struct SecurityDecision {
    pub decision_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub action: SecurityAction,
    pub confidence: f64,
    pub reason: String,
    pub rule_id: Option<String>,
    pub correlation_id: CorrelationId,
}

/// Threat classification from threat intelligence
#[derive(Debug, Clone)]
pub struct ThreatClassification {
    pub classification_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub threat_level: ThreatLevel,
    pub threat_type: String,
    pub indicators: Vec<String>,
    pub confidence: f64,
    pub correlation_id: CorrelationId,
}

/// Trait for behavioral analysis components
#[async_trait]
pub trait BehavioralAnalyzer: Send + Sync {
    /// Analyze user behavior for anomalies
    async fn analyze_behavior(&self, user_activity: &UserActivity) -> ForensicResult<BehavioralAnalysisResult>;
    
    /// Update behavioral baseline
    async fn update_baseline(&self, user_activity: &UserActivity) -> ForensicResult<()>;
    
    /// Get current anomaly threshold
    fn get_anomaly_threshold(&self) -> f64;
    
    /// Set anomaly threshold
    fn set_anomaly_threshold(&mut self, threshold: f64);
}

/// Trait for CUE rule engine components
#[async_trait]
pub trait CueRuleEngine: Send + Sync {
    /// Evaluate threat against security rules
    async fn evaluate_threat(&self, threat_context: &ThreatContext) -> ForensicResult<SecurityDecision>;
    
    /// Load security contract
    async fn load_security_contract(&self, contract_path: &str) -> ForensicResult<String>;
    
    /// Get active rules count
    fn get_active_rules_count(&self) -> usize;
}

/// Trait for threat intelligence components
#[async_trait]
pub trait ThreatIntelligence: Send + Sync {
    /// Classify threat based on intelligence data
    async fn classify_threat(&self, indicators: &[String]) -> ForensicResult<ThreatClassification>;
    
    /// Update threat intelligence database
    async fn update_intelligence(&self, threat_data: &ThreatData) -> ForensicResult<()>;
    
    /// Check if indicator is known threat
    async fn is_known_threat(&self, indicator: &str) -> ForensicResult<bool>;
}

/// Trait for dynamic response components
#[async_trait]
pub trait DynamicResponseEngine: Send + Sync {
    /// Execute response action
    async fn execute_response(&self, decision: &SecurityDecision) -> ForensicResult<ResponseResult>;
    
    /// Get available response types
    fn get_available_responses(&self) -> Vec<ResponseType>;
    
    /// Check if response is available
    fn is_response_available(&self, response_type: &ResponseType) -> bool;
}

/// User activity data for behavioral analysis
#[derive(Debug, Clone)]
pub struct UserActivity {
    pub user_id: String,
    pub session_id: String,
    pub timestamp: DateTime<Utc>,
    pub activity_type: String,
    pub resource_accessed: Option<String>,
    pub source_ip: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Threat context for rule evaluation
#[derive(Debug, Clone)]
pub struct ThreatContext {
    pub source_ip: String,
    pub destination_ip: Option<String>,
    pub protocol: String,
    pub payload_size: usize,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl ThreatContext {
    /// Generate hash for caching
    pub fn hash(&self) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(self.source_ip.as_bytes());
        hasher.update(self.protocol.as_bytes());
        hasher.update(self.payload_size.to_be_bytes());
        hasher.update(self.timestamp.timestamp().to_be_bytes());
        if let Some(user_id) = &self.user_id {
            hasher.update(user_id.as_bytes());
        }
        hex::encode(hasher.finalize())
    }
}

/// Threat data for intelligence updates
#[derive(Debug, Clone)]
pub struct ThreatData {
    pub threat_id: String,
    pub threat_type: String,
    pub indicators: Vec<String>,
    pub severity: ForensicSeverity,
    pub description: String,
    pub source: String,
    pub timestamp: DateTime<Utc>,
}

/// Response execution result
#[derive(Debug, Clone)]
pub struct ResponseResult {
    pub response_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub message: String,
    pub actions_taken: Vec<String>,
}

/// Configuration for behavioral analysis
#[derive(Debug, Clone)]
pub struct BehavioralConfig {
    pub enable_real_time: bool,
    pub anomaly_threshold: f64,
    pub baseline_window_hours: u64,
    pub max_baseline_samples: usize,
}

impl Default for BehavioralConfig {
    fn default() -> Self {
        Self {
            enable_real_time: true,
            anomaly_threshold: 0.7,
            baseline_window_hours: 24,
            max_baseline_samples: 1000,
        }
    }
}

/// Configuration for dynamic response
#[derive(Debug, Clone)]
pub struct DynamicResponseConfig {
    pub enable_auto_response: bool,
    pub response_timeout_seconds: u64,
    pub max_concurrent_responses: usize,
    pub escalation_threshold: ForensicSeverity,
}

impl Default for DynamicResponseConfig {
    fn default() -> Self {
        Self {
            enable_auto_response: false, // Require manual approval by default
            response_timeout_seconds: 30,
            max_concurrent_responses: 10,
            escalation_threshold: ForensicSeverity::High,
        }
    }
}

/// Trait for audit event recording (to break circular dependency with audit_bridge)
#[async_trait]
pub trait ForensicAuditRecorder: Send + Sync {
    /// Record forensic security event
    async fn record_security_event(
        &self,
        event_type: ForensicEventType,
        severity: ForensicSeverity,
        description: String,
        correlation_id: CorrelationId,
    ) -> ForensicResult<Uuid>;
    
    /// Record behavioral anomaly
    async fn record_behavioral_anomaly(
        &self,
        analysis_result: &BehavioralAnalysisResult,
    ) -> ForensicResult<Uuid>;
    
    /// Record CUE rule violation
    async fn record_cue_violation(
        &self,
        decision: &SecurityDecision,
    ) -> ForensicResult<Uuid>;
    
    /// Record threat detection
    async fn record_threat_detection(
        &self,
        classification: &ThreatClassification,
    ) -> ForensicResult<Uuid>;
}

/// Factory trait for creating forensic components
/// Note: Using concrete types instead of trait objects due to async trait limitations
pub trait ForensicComponentFactory {
    /// The concrete behavioral analyzer type
    type BehavioralAnalyzerType: BehavioralAnalyzer;
    /// The concrete CUE engine type
    type CueEngineType: CueRuleEngine;
    /// The concrete threat intelligence type
    type ThreatIntelligenceType: ThreatIntelligence;
    /// The concrete response engine type
    type ResponseEngineType: DynamicResponseEngine;
    
    /// Create behavioral analyzer
    fn create_behavioral_analyzer(&self, config: BehavioralConfig) -> Self::BehavioralAnalyzerType;
    
    /// Create CUE rule engine
    fn create_cue_engine(&self) -> Self::CueEngineType;
    
    /// Create threat intelligence engine
    fn create_threat_intelligence(&self) -> Self::ThreatIntelligenceType;
    
    /// Create dynamic response engine
    fn create_response_engine(&self, config: DynamicResponseConfig) -> Self::ResponseEngineType;
}
