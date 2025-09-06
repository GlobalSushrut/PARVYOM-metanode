//! Attack Vector Detection - Lightweight security analysis for audit events
//!
//! Detects attack patterns and security incidents from audit data

use crate::{RuntimeAuditNode, AuditLevel, OperationType, SecuritySeverity, ExecutionContext, RuntimeAddress};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Attack Vector Detector
#[derive(Debug)]
pub struct AttackVectorDetector {
    /// Detection rules
    rules: Vec<DetectionRule>,
    /// Attack pattern database
    patterns: HashMap<String, AttackPattern>,
    /// Detection statistics
    stats: DetectionStats,
}

/// Detection Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionRule {
    /// Rule ID
    pub rule_id: String,
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Pattern to match
    pub pattern: DetectionPattern,
    /// Severity level
    pub severity: AttackSeverity,
    /// Confidence threshold (0-100)
    pub confidence_threshold: u32,
    /// Rule enabled
    pub enabled: bool,
}

/// Detection Pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DetectionPattern {
    /// Keyword matching in operation details
    KeywordMatch { keywords: Vec<String> },
    /// Frequency-based detection
    FrequencyAnomaly { 
        operation_type: OperationType,
        threshold_per_minute: u32,
    },
    /// Error rate detection
    ErrorRate { 
        error_threshold_percent: f64,
        time_window_minutes: u32,
    },
    /// Privilege escalation detection
    PrivilegeEscalation {
        suspicious_operations: Vec<OperationType>,
    },
    /// Custom pattern
    Custom { pattern_data: HashMap<String, String> },
}

/// Attack Pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackPattern {
    /// Pattern ID
    pub pattern_id: String,
    /// Pattern name
    pub name: String,
    /// MITRE ATT&CK technique ID
    pub mitre_technique: Option<String>,
    /// Pattern description
    pub description: String,
    /// Indicators of compromise
    pub indicators: Vec<String>,
    /// Attack severity
    pub severity: AttackSeverity,
}

/// Attack Severity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum AttackSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Security Event detected by the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// Event ID
    pub event_id: Uuid,
    /// Detection timestamp
    pub detected_at: DateTime<Utc>,
    /// Related audit node
    pub audit_node_id: Uuid,
    /// Detected attack pattern
    pub attack_pattern: AttackPattern,
    /// Detection rule that triggered
    pub triggered_rule: String,
    /// Confidence score (0-100)
    pub confidence: u32,
    /// Event severity
    pub severity: AttackSeverity,
    /// Additional context
    pub context: HashMap<String, String>,
    /// Remediation suggestions
    pub remediation: Vec<String>,
}

/// Detection Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionStats {
    /// Total events analyzed
    pub total_analyzed: u64,
    /// Security events detected
    pub security_events_detected: u64,
    /// False positives
    pub false_positives: u64,
    /// Detection accuracy percentage
    pub accuracy_percent: f64,
    /// Events by severity
    pub events_by_severity: HashMap<AttackSeverity, u64>,
    /// Last analysis time
    pub last_analysis: Option<DateTime<Utc>>,
}

impl Default for DetectionStats {
    fn default() -> Self {
        Self {
            total_analyzed: 0,
            security_events_detected: 0,
            false_positives: 0,
            accuracy_percent: 100.0,
            events_by_severity: HashMap::new(),
            last_analysis: None,
        }
    }
}

impl AttackVectorDetector {
    /// Create a new attack vector detector
    pub fn new() -> Self {
        let mut detector = Self {
            rules: Vec::new(),
            patterns: HashMap::new(),
            stats: DetectionStats::default(),
        };
        
        // Load default detection rules
        detector.load_default_rules();
        detector.load_default_patterns();
        
        detector
    }
    
    /// Load default detection rules
    fn load_default_rules(&mut self) {
        // Suspicious keyword detection
        self.rules.push(DetectionRule {
            rule_id: "SUSPICIOUS_KEYWORDS".to_string(),
            name: "Suspicious Keywords Detection".to_string(),
            description: "Detects suspicious keywords in operation details".to_string(),
            pattern: DetectionPattern::KeywordMatch {
                keywords: vec![
                    "failed".to_string(),
                    "error".to_string(),
                    "unauthorized".to_string(),
                    "denied".to_string(),
                    "breach".to_string(),
                    "attack".to_string(),
                    "malware".to_string(),
                    "exploit".to_string(),
                ],
            },
            severity: AttackSeverity::Medium,
            confidence_threshold: 70,
            enabled: true,
        });
        
        // High error rate detection
        self.rules.push(DetectionRule {
            rule_id: "HIGH_ERROR_RATE".to_string(),
            name: "High Error Rate Detection".to_string(),
            description: "Detects abnormally high error rates".to_string(),
            pattern: DetectionPattern::ErrorRate {
                error_threshold_percent: 50.0,
                time_window_minutes: 5,
            },
            severity: AttackSeverity::High,
            confidence_threshold: 80,
            enabled: true,
        });
        
        // Privilege escalation detection
        self.rules.push(DetectionRule {
            rule_id: "PRIVILEGE_ESCALATION".to_string(),
            name: "Privilege Escalation Detection".to_string(),
            description: "Detects potential privilege escalation attempts".to_string(),
            pattern: DetectionPattern::PrivilegeEscalation {
                suspicious_operations: vec![
                    OperationType::ProcessExit { exit_code: -1, signal: None },
                    OperationType::SecurityEvent { event_type: "violation".to_string(), severity: SecuritySeverity::High, description: "security_violation".to_string(), indicators: vec![] },
                ],
            },
            severity: AttackSeverity::Critical,
            confidence_threshold: 90,
            enabled: true,
        });
    }
    
    /// Load default attack patterns
    fn load_default_patterns(&mut self) {
        self.patterns.insert(
            "BRUTE_FORCE".to_string(),
            AttackPattern {
                pattern_id: "BRUTE_FORCE".to_string(),
                name: "Brute Force Attack".to_string(),
                mitre_technique: Some("T1110".to_string()),
                description: "Multiple failed authentication attempts".to_string(),
                indicators: vec![
                    "Multiple failed login attempts".to_string(),
                    "High frequency of authentication failures".to_string(),
                ],
                severity: AttackSeverity::High,
            },
        );
        
        self.patterns.insert(
            "PRIVILEGE_ESCALATION".to_string(),
            AttackPattern {
                pattern_id: "PRIVILEGE_ESCALATION".to_string(),
                name: "Privilege Escalation".to_string(),
                mitre_technique: Some("T1068".to_string()),
                description: "Attempt to gain higher privileges".to_string(),
                indicators: vec![
                    "Unauthorized access attempts".to_string(),
                    "System configuration changes".to_string(),
                ],
                severity: AttackSeverity::Critical,
            },
        );
        
        self.patterns.insert(
            "DATA_EXFILTRATION".to_string(),
            AttackPattern {
                pattern_id: "DATA_EXFILTRATION".to_string(),
                name: "Data Exfiltration".to_string(),
                mitre_technique: Some("T1041".to_string()),
                description: "Unauthorized data access or transfer".to_string(),
                indicators: vec![
                    "Large data transfers".to_string(),
                    "Unusual file access patterns".to_string(),
                ],
                severity: AttackSeverity::Critical,
            },
        );
    }
    
    /// Analyze audit node for security threats
    pub fn analyze_audit_node(&mut self, node: &RuntimeAuditNode) -> Result<Vec<SecurityEvent>> {
        self.stats.total_analyzed += 1;
        self.stats.last_analysis = Some(Utc::now());
        
        let mut security_events = Vec::new();
        
        // Apply each detection rule
        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }
            
            if let Some(event) = self.apply_detection_rule(rule, node)? {
                // Update statistics before moving event
                *self.stats.events_by_severity.entry(event.severity.clone()).or_insert(0) += 1;
                security_events.push(event);
            }
        }
        
        Ok(security_events)
    }
    
    /// Apply a detection rule to an audit node
    fn apply_detection_rule(&self, rule: &DetectionRule, node: &RuntimeAuditNode) -> Result<Option<SecurityEvent>> {
        let confidence = match &rule.pattern {
            DetectionPattern::KeywordMatch { keywords } => {
                // Check operation data for keyword matches
                let operation_str = format!("{:?}", node.operation_type);
                self.check_keyword_match(keywords, &operation_str)
            }
            DetectionPattern::FrequencyAnomaly { .. } => {
                // Simplified frequency check (would need historical data in real implementation)
                if matches!(node.audit_level, AuditLevel::Critical) { 80 } else { 0 }
            }
            DetectionPattern::ErrorRate { .. } => {
                // Simplified error rate check
                // Check for error patterns in operation type and data
                let operation_str = format!("{:?}", node.operation_type).to_lowercase();
                if operation_str.contains("error") || operation_str.contains("exit") { 75 } else { 0 }
            }
            DetectionPattern::PrivilegeEscalation { suspicious_operations } => {
                if suspicious_operations.contains(&node.operation_type) { 85 } else { 0 }
            }
            DetectionPattern::Custom { .. } => {
                // Custom pattern matching would be implemented here
                0
            }
        };
        
        if confidence >= rule.confidence_threshold {
            // Find matching attack pattern
            let attack_pattern = self.find_matching_pattern(rule, node)
                .unwrap_or_else(|| self.create_generic_pattern(rule));
            
            let security_event = SecurityEvent {
                event_id: Uuid::new_v4(),
                detected_at: Utc::now(),
                audit_node_id: {
                    let mut uuid_bytes = [0u8; 16];
                    uuid_bytes.copy_from_slice(&node.node_id[..16]);
                    Uuid::from_bytes(uuid_bytes)
                },
                attack_pattern,
                triggered_rule: rule.rule_id.clone(),
                confidence,
                severity: rule.severity.clone(),
                context: self.build_event_context(node),
                remediation: self.generate_remediation_suggestions(&rule.severity),
            };
            
            Ok(Some(security_event))
        } else {
            Ok(None)
        }
    }
    
    /// Check for keyword matches
    fn check_keyword_match(&self, keywords: &[String], text: &str) -> u32 {
        let text_lower = text.to_lowercase();
        let matches = keywords.iter()
            .filter(|keyword| text_lower.contains(&keyword.to_lowercase()))
            .count();
        
        if matches > 0 {
            // Calculate confidence based on number of matches
            std::cmp::min(50 + (matches * 20) as u32, 100)
        } else {
            0
        }
    }
    
    /// Find matching attack pattern
    fn find_matching_pattern(&self, rule: &DetectionRule, _node: &RuntimeAuditNode) -> Option<AttackPattern> {
        // Simplified pattern matching based on rule ID
        match rule.rule_id.as_str() {
            "SUSPICIOUS_KEYWORDS" => self.patterns.get("BRUTE_FORCE").cloned(),
            "HIGH_ERROR_RATE" => self.patterns.get("DATA_EXFILTRATION").cloned(),
            "PRIVILEGE_ESCALATION" => self.patterns.get("PRIVILEGE_ESCALATION").cloned(),
            _ => None,
        }
    }
    
    /// Create generic attack pattern
    fn create_generic_pattern(&self, rule: &DetectionRule) -> AttackPattern {
        AttackPattern {
            pattern_id: format!("GENERIC_{}", rule.rule_id),
            name: format!("Generic {}", rule.name),
            mitre_technique: None,
            description: rule.description.clone(),
            indicators: vec!["Suspicious activity detected".to_string()],
            severity: rule.severity.clone(),
        }
    }
    
    /// Build event context
    fn build_event_context(&self, node: &RuntimeAuditNode) -> HashMap<String, String> {
        let mut context = HashMap::new();
        
        context.insert("runtime_type".to_string(), format!("{:?}", node.execution_context));
        context.insert("runtime_address".to_string(), format!("{:?}", node.runtime_address));
        context.insert("operation_type".to_string(), format!("{:?}", node.operation_type));
        context.insert("audit_level".to_string(), format!("{:?}", node.audit_level));
        context.insert("timestamp".to_string(), node.timestamp_ns.to_string());
        
        // Container ID is now part of ExecutionContext enum variants
        match &node.execution_context {
            ExecutionContext::DockLock { container_id, .. } => {
                context.insert("container_id".to_string(), format!("{:?}", container_id));
            }
            _ => {}
        }
        
        context
    }
    
    /// Generate remediation suggestions
    fn generate_remediation_suggestions(&self, severity: &AttackSeverity) -> Vec<String> {
        match severity {
            AttackSeverity::Low => vec![
                "Monitor for additional suspicious activity".to_string(),
                "Review system logs for related events".to_string(),
            ],
            AttackSeverity::Medium => vec![
                "Investigate the source of the suspicious activity".to_string(),
                "Consider increasing monitoring on affected systems".to_string(),
                "Review access controls and permissions".to_string(),
            ],
            AttackSeverity::High => vec![
                "Immediately investigate the security incident".to_string(),
                "Consider isolating affected systems".to_string(),
                "Review and strengthen security controls".to_string(),
                "Notify security team and stakeholders".to_string(),
            ],
            AttackSeverity::Critical => vec![
                "Activate incident response procedures immediately".to_string(),
                "Isolate affected systems from the network".to_string(),
                "Preserve forensic evidence".to_string(),
                "Notify executive leadership and legal team".to_string(),
                "Consider engaging external security experts".to_string(),
            ],
        }
    }
    
    /// Get detection statistics
    pub fn get_stats(&self) -> &DetectionStats {
        &self.stats
    }
    
    /// Add custom detection rule
    pub fn add_rule(&mut self, rule: DetectionRule) {
        self.rules.push(rule);
    }
    
    /// Add custom attack pattern
    pub fn add_pattern(&mut self, pattern: AttackPattern) {
        self.patterns.insert(pattern.pattern_id.clone(), pattern);
    }
    
    /// Enable/disable detection rule
    pub fn set_rule_enabled(&mut self, rule_id: &str, enabled: bool) -> Result<()> {
        if let Some(rule) = self.rules.iter_mut().find(|r| r.rule_id == rule_id) {
            rule.enabled = enabled;
            Ok(())
        } else {
            Err(anyhow!("Rule not found: {}", rule_id))
        }
    }
    
    /// Start monitoring for attack vectors
    pub async fn start_monitoring(&mut self) -> Result<()> {
        // Initialize monitoring state
        self.stats.last_analysis = Some(Utc::now());
        println!("Attack vector detector monitoring started");
        Ok(())
    }
    
    /// Analyze a runtime event for security threats
    pub async fn analyze_event(&mut self, event: &crate::capture_engine::CaptureEvent) -> Result<Vec<SecurityEvent>> {
        // Convert RuntimeEvent to RuntimeAuditNode for analysis
        // This is a simplified conversion - in a real implementation, 
        // you'd have proper event-to-node conversion logic
        let audit_node = RuntimeAuditNode {
            node_id: event.event_id.as_bytes().to_vec().try_into().unwrap_or([0u8; 32]),
            parent_id: None,
            children: Vec::new(),
            timestamp_ns: event.captured_at.timestamp_nanos_opt().unwrap_or(0) as u64,
            duration_ns: None,
            sequence_number: 0,
            execution_context: event.audit_node.execution_context.clone(),
            runtime_address: RuntimeAddress::default(), // TODO: Convert from String
            operation_type: event.audit_node.operation_type.clone(),
            operation_data: event.audit_node.operation_data.clone(),
            binary_outputs: Vec::new(),
            proof_chain: event.audit_node.proof_chain.clone(),
            integrity_hash: [0u8; 32],
            signature: Vec::new(),
            audit_level: event.audit_node.audit_level.clone(),
            compliance_tags: Vec::new(),
            export_metadata: crate::runtime_node::ExportMetadata::default(),
        };
        
        self.analyze_audit_node(&audit_node)
    }
    
    /// Get all security events for a time range
    pub fn get_events_in_range(&self, _start: DateTime<Utc>, _end: DateTime<Utc>) -> Vec<SecurityEvent> {
        // In a real implementation, this would query stored security events
        // For now, return empty vector
        Vec::new()
    }
}

impl Default for AttackVectorDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        runtime_node::{RuntimeAuditNode, ExecutionContext, RuntimeAddress},
        capture_engine::{CaptureEvent, RuntimeType},
        SecuritySeverity,
    };
    
    #[test]
    fn test_attack_detector_creation() {
        let detector = AttackVectorDetector::new();
        assert!(!detector.patterns.is_empty());
    }
    
    #[test]
    fn test_keyword_detection() {
        let mut detector = AttackVectorDetector::new();
        
        let node = RuntimeAuditNode {
            node_id: Uuid::new_v4(),
            parent_id: None,
            execution_context: ExecutionContext {
                runtime_type: RuntimeType::DockLock,
                runtime_address: "test://address".to_string(),
                container_id: Some("test-container".to_string()),
                process_id: None,
                thread_id: None,
                user_id: None,
                session_id: None,
            },
            operation: OperationType::ContainerExecution,
            operation_data: "Authentication failed for user admin".to_string(),
            binary_outputs: vec![],
            timestamp_ns: Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
            audit_level: AuditLevel::Critical,
            compliance_tags: vec![],
            proof_chain: Default::default(),
            export_metadata: HashMap::new(),
        };
        
        let events = detector.analyze_audit_node(&node).unwrap();
        assert!(!events.is_empty());
        
        let event = &events[0];
        assert_eq!(event.audit_node_id, node.node_id);
        assert!(event.confidence >= 70);
    }
    
    #[test]
    fn test_privilege_escalation_detection() {
        let mut detector = AttackVectorDetector::new();
        
        let node = RuntimeAuditNode {
            node_id: Uuid::new_v4(),
            parent_id: None,
            execution_context: ExecutionContext {
                runtime_type: RuntimeType::HttpCage,
                runtime_address: "test://address".to_string(),
                container_id: None,
                process_id: None,
                thread_id: None,
                user_id: None,
                session_id: None,
            },
            operation: OperationType::SecurityViolation,
            operation_details: "Unauthorized access attempt detected".to_string(),
            binary_output: b"security violation".to_vec(),
            timestamp_ns: Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
            audit_level: AuditLevel::Critical,
            compliance_tags: vec![],
            proof_chain: Default::default(),
            export_metadata: HashMap::new(),
        };
        
        let events = detector.analyze_audit_node(&node).unwrap();
        assert!(!events.is_empty());
        
        let event = &events[0];
        assert_eq!(event.severity, AttackSeverity::Critical);
        assert!(event.confidence >= 85);
    }
    
    #[test]
    fn test_detection_stats() {
        let mut detector = AttackVectorDetector::new();
        
        let initial_stats = detector.get_stats();
        assert_eq!(initial_stats.total_analyzed, 0);
        assert_eq!(initial_stats.security_events_detected, 0);
        
        // Create a suspicious audit node
        let node = RuntimeAuditNode {
            node_id: Uuid::new_v4(),
            parent_id: None,
            execution_context: ExecutionContext {
                runtime_type: RuntimeType::DockLock,
                runtime_address: "test://address".to_string(),
                container_id: Some("test-container".to_string()),
                process_id: None,
                thread_id: None,
                user_id: None,
                session_id: None,
            },
            operation: OperationType::ContainerExecution,
            operation_details: "System error occurred during execution".to_string(),
            binary_output: b"error".to_vec(),
            timestamp_ns: Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
            audit_level: AuditLevel::Critical,
            compliance_tags: vec![],
            proof_chain: Default::default(),
            export_metadata: HashMap::new(),
        };
        
        detector.analyze_audit_node(&node).unwrap();
        
        let updated_stats = detector.get_stats();
        assert_eq!(updated_stats.total_analyzed, 1);
        assert!(updated_stats.last_analysis.is_some());
    }
}
