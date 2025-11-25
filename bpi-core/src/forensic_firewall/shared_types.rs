// Shared types for forensic firewall modules to eliminate circular dependencies
// This module contains common types used across multiple forensic modules

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Types of forensic events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ForensicEventType {
    /// Security violation detected
    SecurityViolation,
    /// Behavioral anomaly detected
    BehavioralAnomaly,
    /// CUE rule violation
    CueRuleViolation,
    /// Threat intelligence match
    ThreatIntelligenceMatch,
    /// Network intrusion attempt
    NetworkIntrusion,
    /// Malware detection
    MalwareDetection,
    /// Data exfiltration attempt
    DataExfiltration,
    /// Privilege escalation
    PrivilegeEscalation,
    /// Authentication failure
    AuthenticationFailure,
    /// System compromise
    SystemCompromise,
    /// Policy violation
    PolicyViolation,
    /// Forensic analysis complete
    ForensicAnalysisComplete,
    /// Security threat detected
    SecurityThreatDetected,
    /// Forensic evidence collected
    ForensicEvidenceCollected,
    /// Behavioral anomaly detected
    BehavioralAnomalyDetected,
    /// Policy enforcement action
    PolicyEnforcementAction,
}

/// Forensic event severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ForensicSeverity {
    /// Info severity - informational
    Info,
    /// Low severity - informational
    Low,
    /// Medium severity - warning
    Medium,
    /// High severity - critical
    High,
    /// Critical severity - emergency
    Critical,
    /// Emergency severity - immediate action required
    Emergency,
}

/// Security decision actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecurityAction {
    /// Allow the action
    Allow,
    /// Block the action
    Block,
    /// Quarantine the source
    Quarantine,
    /// Log and monitor
    LogAndMonitor,
    /// Monitor activity
    Monitor,
    /// Rate limit
    RateLimit,
    /// Require additional authentication
    RequireAuth,
    /// Escalate to higher security level
    Escalate,
    /// Emergency shutdown
    EmergencyShutdown,
    /// Emergency block
    EmergencyBlock,
}

/// Response types for dynamic response system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResponseType {
    /// Immediate response
    Immediate,
    /// Delayed response
    Delayed,
    /// Escalated response
    Escalated,
    /// Automated response
    Automated,
    /// Manual response required
    Manual,
}

/// Threat classification levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThreatLevel {
    /// No threat detected
    None,
    /// Low threat level
    Low,
    /// Medium threat level
    Medium,
    /// High threat level
    High,
    /// Critical threat level
    Critical,
}

/// Behavioral analysis result status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnalysisStatus {
    /// Analysis in progress
    InProgress,
    /// Analysis completed successfully
    Completed,
    /// Analysis failed
    Failed,
    /// Analysis requires manual review
    ManualReview,
}

/// Common configuration for forensic modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForensicConfig {
    /// Enable real-time monitoring
    pub enable_real_time: bool,
    /// Maximum events to store in memory
    pub max_events_in_memory: usize,
    /// Event retention period in hours
    pub event_retention_hours: u64,
    /// Enable ML-assisted analysis
    pub enable_ml_analysis: bool,
    /// Severity threshold for alerts
    pub alert_threshold: ForensicSeverity,
    /// Enable automatic response
    pub enable_auto_response: bool,
}

impl Default for ForensicConfig {
    fn default() -> Self {
        Self {
            enable_real_time: true,
            max_events_in_memory: 10000,
            event_retention_hours: 24 * 7, // 1 week
            enable_ml_analysis: true,
            alert_threshold: ForensicSeverity::Medium,
            enable_auto_response: false, // Require manual approval by default
        }
    }
}

/// Common metrics for forensic modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForensicMetrics {
    /// Total events processed
    pub total_events: u64,
    /// Events by severity
    pub events_by_severity: HashMap<String, u64>,
    /// Events by type
    pub events_by_type: HashMap<String, u64>,
    /// Average processing time (microseconds)
    pub avg_processing_time_us: f64,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

impl Default for ForensicMetrics {
    fn default() -> Self {
        Self {
            total_events: 0,
            events_by_severity: HashMap::new(),
            events_by_type: HashMap::new(),
            avg_processing_time_us: 0.0,
            last_updated: Utc::now(),
        }
    }
}

impl ForensicMetrics {
    /// Update metrics with new event
    pub fn update_with_event(&mut self, event_type: &ForensicEventType, severity: &ForensicSeverity, processing_time_us: f64) {
        self.total_events += 1;
        
        // Update severity counts
        let severity_key = format!("{:?}", severity);
        *self.events_by_severity.entry(severity_key).or_insert(0) += 1;
        
        // Update type counts
        let type_key = format!("{:?}", event_type);
        *self.events_by_type.entry(type_key).or_insert(0) += 1;
        
        // Update average processing time
        self.avg_processing_time_us = (self.avg_processing_time_us * (self.total_events - 1) as f64 + processing_time_us) / self.total_events as f64;
        
        self.last_updated = Utc::now();
    }
}

/// Common error types for forensic modules
#[derive(Debug, thiserror::Error)]
pub enum ForensicError {
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Processing error: {0}")]
    Processing(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Authorization error: {0}")]
    Authorization(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

pub type ForensicResult<T> = Result<T, ForensicError>;

/// Event correlation ID for tracking related events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CorrelationId(pub Uuid);

impl CorrelationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
    
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for CorrelationId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for CorrelationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
