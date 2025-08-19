//! Stage 40: DA Observability (Traffic Light Dashboard)
//! 
//! Real-time monitoring and observability dashboard for the Traffic Light Pipeline.
//! Provides comprehensive visibility into data flow decisions, policy compliance,
//! and security events across the BISO architecture.

use crate::error::{DockLockError, DockLockResult};
use crate::traffic_light::{TrafficLightPipeline, TrafficLightState, TrafficLightDecision, DataClassification};
use crate::biso_policy::{BisoPolicyEngine, GeographicRegion, PolicyEvaluationResult};
use crate::blockbook::{Blockbook, BlockbookEventType, EventSeverity};
use crate::audit_book::AuditBook;
use bpi_enc::domain_hash;
use ed25519_dalek::{SigningKey, Signature, Signer, Verifier};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, warn, debug};
use uuid::Uuid;

/// Domain separation constant for dashboard operations
pub const DASHBOARD_HASH: &str = "DASHBOARD";

/// Dashboard metrics time window
pub const METRICS_WINDOW_SECONDS: u64 = 3600; // 1 hour

/// Real-time traffic light decision event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficEvent {
    /// Event ID
    pub event_id: String,
    /// Timestamp
    pub timestamp: u64,
    /// Traffic light decision
    pub decision: TrafficLightDecision,
    /// Geographic region
    pub region: GeographicRegion,
    /// Data classification
    pub classification: DataClassification,
    /// Policy evaluation result
    pub policy_result: PolicyEvaluationResult,
    /// Processing latency in milliseconds
    pub latency_ms: u64,
    /// Event severity
    pub severity: EventSeverity,
}

/// Geographic traffic statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalStats {
    /// Region identifier
    pub region: GeographicRegion,
    /// Total decisions in time window
    pub total_decisions: u64,
    /// Green (pass) decisions
    pub green_count: u64,
    /// Yellow (quarantine) decisions
    pub yellow_count: u64,
    /// Red (block) decisions
    pub red_count: u64,
    /// Average latency
    pub avg_latency_ms: f64,
    /// Policy compliance rate
    pub compliance_rate: f64,
    /// Last updated timestamp
    pub last_updated: u64,
}

/// Dashboard alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    /// Enable SOC alerts for red events
    pub enable_soc_alerts: bool,
    /// Red event threshold per minute
    pub red_threshold_per_minute: u32,
    /// Compliance rate threshold (below triggers alert)
    pub compliance_threshold: f64,
    /// Latency threshold in milliseconds
    pub latency_threshold_ms: u64,
    /// Alert cooldown period in seconds
    pub alert_cooldown_seconds: u64,
}

/// Real-time dashboard metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardMetrics {
    /// Current timestamp
    pub timestamp: u64,
    /// Total events processed
    pub total_events: u64,
    /// Events per second
    pub events_per_second: f64,
    /// Overall compliance rate
    pub overall_compliance_rate: f64,
    /// Average processing latency
    pub avg_latency_ms: f64,
    /// Regional statistics
    pub regional_stats: HashMap<String, RegionalStats>,
    /// Recent events (last 100)
    pub recent_events: VecDeque<TrafficEvent>,
    /// Active alerts
    pub active_alerts: Vec<DashboardAlert>,
}

/// Dashboard alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardAlert {
    /// Alert ID
    pub alert_id: String,
    /// Alert type
    pub alert_type: AlertType,
    /// Alert message
    pub message: String,
    /// Severity level
    pub severity: EventSeverity,
    /// Timestamp when alert was triggered
    pub triggered_at: u64,
    /// Whether alert has been acknowledged
    pub acknowledged: bool,
    /// Related region (if applicable)
    pub region: Option<GeographicRegion>,
}

/// Types of dashboard alerts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AlertType {
    /// High red event rate
    HighRedEventRate,
    /// Low compliance rate
    LowComplianceRate,
    /// High latency
    HighLatency,
    /// Policy engine failure
    PolicyEngineFailure,
    /// Blockbook integrity issue
    BlockbookIntegrityIssue,
    /// Custom alert
    Custom(String),
}

/// Dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    /// Update interval in seconds
    pub update_interval_seconds: u64,
    /// Maximum events to keep in memory
    pub max_events_in_memory: usize,
    /// Alert configuration
    pub alert_config: AlertConfig,
    /// Enable geographic visualization
    pub enable_geographic_viz: bool,
    /// Enable real-time streaming
    pub enable_real_time_streaming: bool,
    /// Dashboard refresh rate in milliseconds
    pub refresh_rate_ms: u64,
}

/// Main Traffic Light Dashboard
#[derive(Debug)]
pub struct TrafficLightDashboard {
    /// Dashboard configuration
    config: DashboardConfig,
    /// Traffic light pipeline reference
    traffic_pipeline: Arc<TrafficLightPipeline>,
    /// BISO policy engine reference
    policy_engine: Arc<BisoPolicyEngine>,
    /// Blockbook reference for audit trail
    blockbook: Arc<RwLock<Blockbook>>,
    /// Audit book reference for compliance
    audit_book: Arc<RwLock<AuditBook>>,
    /// Real-time metrics
    metrics: Arc<RwLock<DashboardMetrics>>,
    /// Event buffer for processing
    event_buffer: Arc<RwLock<VecDeque<TrafficEvent>>>,
    /// Alert state tracking
    alert_state: Arc<RwLock<HashMap<AlertType, u64>>>, // Last triggered timestamp
    /// Dashboard statistics
    stats: Arc<RwLock<DashboardStats>>,
    /// Signing key for dashboard operations
    signing_key: SigningKey,
}

/// Dashboard operational statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    /// Total events processed
    pub total_events_processed: u64,
    /// Total alerts generated
    pub total_alerts_generated: u64,
    /// Dashboard uptime in seconds
    pub uptime_seconds: u64,
    /// Last metrics update timestamp
    pub last_update_timestamp: u64,
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
}

/// Performance metrics for dashboard operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Average event processing time in microseconds
    pub avg_event_processing_time_us: f64,
    /// Metrics computation time in milliseconds
    pub metrics_computation_time_ms: f64,
    /// Memory usage for event buffer
    pub event_buffer_memory_usage: usize,
    /// Alert processing latency
    pub alert_processing_latency_ms: f64,
}

/// Compliance report for regulatory purposes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Report ID
    pub report_id: String,
    /// Report start time
    pub start_time: u64,
    /// Report end time
    pub end_time: u64,
    /// Total events in period
    pub total_events: u64,
    /// Green (pass) events
    pub green_events: u64,
    /// Yellow (quarantine) events
    pub yellow_events: u64,
    /// Red (block) events
    pub red_events: u64,
    /// Overall compliance rate
    pub compliance_rate: f64,
    /// Average latency
    pub avg_latency_ms: f64,
    /// Report generation timestamp
    pub generated_at: u64,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            update_interval_seconds: 5,
            max_events_in_memory: 10000,
            alert_config: AlertConfig {
                enable_soc_alerts: true,
                red_threshold_per_minute: 10,
                compliance_threshold: 0.95,
                latency_threshold_ms: 1000,
                alert_cooldown_seconds: 300,
            },
            enable_geographic_viz: true,
            enable_real_time_streaming: true,
            refresh_rate_ms: 1000,
        }
    }
}

impl Default for DashboardMetrics {
    fn default() -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            total_events: 0,
            events_per_second: 0.0,
            overall_compliance_rate: 1.0,
            avg_latency_ms: 0.0,
            regional_stats: HashMap::new(),
            recent_events: VecDeque::new(),
            active_alerts: Vec::new(),
        }
    }
}

impl Default for DashboardStats {
    fn default() -> Self {
        Self {
            total_events_processed: 0,
            total_alerts_generated: 0,
            uptime_seconds: 0,
            last_update_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            performance_metrics: PerformanceMetrics {
                avg_event_processing_time_us: 0.0,
                metrics_computation_time_ms: 0.0,
                event_buffer_memory_usage: 0,
                alert_processing_latency_ms: 0.0,
            },
        }
    }
}

impl TrafficLightDashboard {
    /// Create a new Traffic Light Dashboard
    pub fn new(
        config: DashboardConfig,
        traffic_pipeline: Arc<TrafficLightPipeline>,
        policy_engine: Arc<BisoPolicyEngine>,
        blockbook: Arc<RwLock<Blockbook>>,
        audit_book: Arc<RwLock<AuditBook>>,
    ) -> DockLockResult<Self> {
        let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
        
        let dashboard = Self {
            config,
            traffic_pipeline,
            policy_engine,
            blockbook,
            audit_book,
            metrics: Arc::new(RwLock::new(DashboardMetrics::default())),
            event_buffer: Arc::new(RwLock::new(VecDeque::new())),
            alert_state: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(DashboardStats::default())),
            signing_key,
        };

        info!("Traffic Light Dashboard created with real-time monitoring enabled");
        Ok(dashboard)
    }

    /// Record a traffic light event for dashboard monitoring
    pub fn record_traffic_event(
        &self,
        decision: TrafficLightDecision,
        region: GeographicRegion,
        classification: DataClassification,
        policy_result: PolicyEvaluationResult,
        latency_ms: u64,
    ) -> DockLockResult<()> {
        let event = TrafficEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| DockLockError::InvalidOperation(format!("Time error: {}", e)))?
                .as_secs(),
            decision: decision.clone(),
            region: region.clone(),
            classification,
            policy_result,
            latency_ms,
            severity: self.determine_event_severity(&decision),
        };

        // Add to event buffer
        {
            let mut buffer = self.event_buffer.write()
                .map_err(|_| DockLockError::InvalidOperation("Failed to acquire event buffer write lock".to_string()))?;
            
            buffer.push_back(event.clone());
            
            // Maintain buffer size limit
            if buffer.len() > self.config.max_events_in_memory {
                buffer.pop_front();
            }
        }

        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| DockLockError::InvalidOperation("Failed to acquire stats write lock".to_string()))?;
            stats.total_events_processed += 1;
        }

        // Check for alert conditions
        self.check_alert_conditions(&event)?;

        // Record in blockbook for audit trail
        self.record_event_in_blockbook(&event)?;

        debug!("Traffic event recorded: {} - {:?}", event.event_id, event.decision.state);
        Ok(())
    }

    /// Update dashboard metrics in real-time
    pub fn update_metrics(&self) -> DockLockResult<()> {
        let start_time = std::time::Instant::now();
        
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| DockLockError::InvalidOperation(format!("Time error: {}", e)))?
            .as_secs();

        // Calculate metrics from event buffer
        let (total_events, events_per_second, regional_stats, recent_events, compliance_rate, avg_latency) = {
            let buffer = self.event_buffer.read()
                .map_err(|_| DockLockError::InvalidOperation("Failed to acquire event buffer read lock".to_string()))?;
            
            let window_start = current_time.saturating_sub(METRICS_WINDOW_SECONDS);
            let recent_events: Vec<_> = buffer.iter()
                .filter(|event| event.timestamp >= window_start)
                .cloned()
                .collect();

            let total_events = recent_events.len() as u64;
            let events_per_second = total_events as f64 / METRICS_WINDOW_SECONDS as f64;

            // Calculate regional statistics
            let mut regional_stats = HashMap::new();
            for region in [
                GeographicRegion::US,
                GeographicRegion::EU,
                GeographicRegion::JP,
                GeographicRegion::Global,
            ] {
                let region_events: Vec<_> = recent_events.iter()
                    .filter(|event| event.region == region)
                    .collect();

                let total_decisions = region_events.len() as u64;
                let green_count = region_events.iter()
                    .filter(|event| event.decision.state == TrafficLightState::Green)
                    .count() as u64;
                let yellow_count = region_events.iter()
                    .filter(|event| event.decision.state == TrafficLightState::Yellow)
                    .count() as u64;
                let red_count = region_events.iter()
                    .filter(|event| event.decision.state == TrafficLightState::Red)
                    .count() as u64;

                let avg_latency_ms = if !region_events.is_empty() {
                    region_events.iter().map(|e| e.latency_ms as f64).sum::<f64>() / region_events.len() as f64
                } else {
                    0.0
                };

                let compliance_rate = if total_decisions > 0 {
                    (green_count + yellow_count) as f64 / total_decisions as f64
                } else {
                    1.0
                };

                regional_stats.insert(
                    format!("{:?}", region),
                    RegionalStats {
                        region: region.clone(),
                        total_decisions,
                        green_count,
                        yellow_count,
                        red_count,
                        avg_latency_ms,
                        compliance_rate,
                        last_updated: current_time,
                    }
                );
            }

            // Overall compliance rate
            let total_compliant = recent_events.iter()
                .filter(|event| matches!(event.decision.state, TrafficLightState::Green | TrafficLightState::Yellow))
                .count();
            let overall_compliance_rate = if total_events > 0 {
                total_compliant as f64 / total_events as f64
            } else {
                1.0
            };

            // Average latency
            let avg_latency = if !recent_events.is_empty() {
                recent_events.iter().map(|e| e.latency_ms as f64).sum::<f64>() / recent_events.len() as f64
            } else {
                0.0
            };

            // Keep only last 100 events for recent display
            let mut recent_display = VecDeque::new();
            for event in recent_events.iter().rev().take(100) {
                recent_display.push_front(event.clone());
            }

            (total_events, events_per_second, regional_stats, recent_display, overall_compliance_rate, avg_latency)
        };

        // Get active alerts
        let active_alerts = {
            let metrics = self.metrics.read()
                .map_err(|_| DockLockError::InvalidOperation("Failed to acquire metrics read lock".to_string()))?;
            metrics.active_alerts.clone()
        };

        // Update metrics
        {
            let mut metrics = self.metrics.write()
                .map_err(|_| DockLockError::InvalidOperation("Failed to acquire metrics write lock".to_string()))?;
            
            metrics.timestamp = current_time;
            metrics.total_events = total_events;
            metrics.events_per_second = events_per_second;
            metrics.overall_compliance_rate = compliance_rate;
            metrics.avg_latency_ms = avg_latency;
            metrics.regional_stats = regional_stats;
            metrics.recent_events = recent_events;
            metrics.active_alerts = active_alerts;
        }

        // Update performance metrics
        let computation_time = start_time.elapsed().as_millis() as f64;
        {
            let mut stats = self.stats.write()
                .map_err(|_| DockLockError::InvalidOperation("Failed to acquire stats write lock".to_string()))?;
            stats.performance_metrics.metrics_computation_time_ms = computation_time;
            stats.last_update_timestamp = current_time;
        }

        debug!("Dashboard metrics updated in {:.2}ms", computation_time);
        Ok(())
    }

    /// Get current dashboard metrics
    pub fn get_metrics(&self) -> DockLockResult<DashboardMetrics> {
        let metrics = self.metrics.read()
            .map_err(|_| DockLockError::InvalidOperation("Failed to acquire metrics read lock".to_string()))?;
        Ok(metrics.clone())
    }

    /// Get dashboard statistics
    pub fn get_stats(&self) -> DockLockResult<DashboardStats> {
        let stats = self.stats.read()
            .map_err(|_| DockLockError::InvalidOperation("Failed to acquire stats read lock".to_string()))?;
        Ok(stats.clone())
    }

    /// Acknowledge an alert
    pub fn acknowledge_alert(&self, alert_id: &str) -> DockLockResult<bool> {
        let mut metrics = self.metrics.write()
            .map_err(|_| DockLockError::InvalidOperation("Failed to acquire metrics write lock".to_string()))?;
        
        for alert in &mut metrics.active_alerts {
            if alert.alert_id == alert_id {
                alert.acknowledged = true;
                info!("Alert acknowledged: {}", alert_id);
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    /// Clear acknowledged alerts
    pub fn clear_acknowledged_alerts(&self) -> DockLockResult<usize> {
        let mut metrics = self.metrics.write()
            .map_err(|_| DockLockError::InvalidOperation("Failed to acquire metrics write lock".to_string()))?;
        
        let initial_count = metrics.active_alerts.len();
        metrics.active_alerts.retain(|alert| !alert.acknowledged);
        let cleared_count = initial_count - metrics.active_alerts.len();
        
        if cleared_count > 0 {
            info!("Cleared {} acknowledged alerts", cleared_count);
        }
        
        Ok(cleared_count)
    }

    /// Generate dashboard report for compliance
    pub fn generate_compliance_report(&self, start_time: u64, end_time: u64) -> DockLockResult<ComplianceReport> {
        let buffer = self.event_buffer.read()
            .map_err(|_| DockLockError::InvalidOperation("Failed to acquire event buffer read lock".to_string()))?;
        
        let report_events: Vec<_> = buffer.iter()
            .filter(|event| event.timestamp >= start_time && event.timestamp <= end_time)
            .cloned()
            .collect();

        let total_events = report_events.len() as u64;
        let green_events = report_events.iter()
            .filter(|event| event.decision.state == TrafficLightState::Green)
            .count() as u64;
        let yellow_events = report_events.iter()
            .filter(|event| event.decision.state == TrafficLightState::Yellow)
            .count() as u64;
        let red_events = report_events.iter()
            .filter(|event| event.decision.state == TrafficLightState::Red)
            .count() as u64;

        let compliance_rate = if total_events > 0 {
            (green_events + yellow_events) as f64 / total_events as f64
        } else {
            1.0
        };

        let avg_latency = if !report_events.is_empty() {
            report_events.iter().map(|e| e.latency_ms as f64).sum::<f64>() / report_events.len() as f64
        } else {
            0.0
        };

        let report = ComplianceReport {
            report_id: Uuid::new_v4().to_string(),
            start_time,
            end_time,
            total_events,
            green_events,
            yellow_events,
            red_events,
            compliance_rate,
            avg_latency_ms: avg_latency,
            generated_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| DockLockError::InvalidOperation(format!("Time error: {}", e)))?
                .as_secs(),
        };

        info!("Compliance report generated: {} events, {:.2}% compliance", total_events, compliance_rate * 100.0);
        Ok(report)
    }

    /// Determine event severity based on traffic light decision
    fn determine_event_severity(&self, decision: &TrafficLightDecision) -> EventSeverity {
        match decision.state {
            TrafficLightState::Green => EventSeverity::Info,
            TrafficLightState::Yellow => EventSeverity::Warning,
            TrafficLightState::Red => EventSeverity::Error,
        }
    }

    /// Check for alert conditions and generate alerts
    fn check_alert_conditions(&self, event: &TrafficEvent) -> DockLockResult<()> {
        let current_time = event.timestamp;
        
        // Check for high red event rate
        if event.decision.state == TrafficLightState::Red {
            self.check_red_event_rate_alert(current_time)?;
        }

        // Check for high latency
        if event.latency_ms > self.config.alert_config.latency_threshold_ms {
            self.generate_alert(
                AlertType::HighLatency,
                format!("High latency detected: {}ms", event.latency_ms),
                EventSeverity::Warning,
                Some(event.region.clone()),
                current_time,
            )?;
        }

        Ok(())
    }

    /// Check for high red event rate and generate alert if needed
    fn check_red_event_rate_alert(&self, current_time: u64) -> DockLockResult<()> {
        let buffer = self.event_buffer.read()
            .map_err(|_| DockLockError::InvalidOperation("Failed to acquire event buffer read lock".to_string()))?;
        
        let minute_ago = current_time.saturating_sub(60);
        let red_events_last_minute = buffer.iter()
            .filter(|event| {
                event.timestamp >= minute_ago && 
                event.decision.state == TrafficLightState::Red
            })
            .count() as u32;

        if red_events_last_minute >= self.config.alert_config.red_threshold_per_minute {
            self.generate_alert(
                AlertType::HighRedEventRate,
                format!("High red event rate: {} events in last minute", red_events_last_minute),
                EventSeverity::Critical,
                None,
                current_time,
            )?;
        }

        Ok(())
    }

    /// Generate a dashboard alert
    fn generate_alert(
        &self,
        alert_type: AlertType,
        message: String,
        severity: EventSeverity,
        region: Option<GeographicRegion>,
        timestamp: u64,
    ) -> DockLockResult<()> {
        // Check cooldown period
        {
            let alert_state = self.alert_state.read()
                .map_err(|_| DockLockError::InvalidOperation("Failed to acquire alert state read lock".to_string()))?;
            
            if let Some(&last_triggered) = alert_state.get(&alert_type) {
                if timestamp.saturating_sub(last_triggered) < self.config.alert_config.alert_cooldown_seconds {
                    return Ok(()); // Still in cooldown period
                }
            }
        }

        let alert = DashboardAlert {
            alert_id: Uuid::new_v4().to_string(),
            alert_type: alert_type.clone(),
            message: message.clone(),
            severity,
            triggered_at: timestamp,
            acknowledged: false,
            region,
        };

        // Add to active alerts
        {
            let mut metrics = self.metrics.write()
                .map_err(|_| DockLockError::InvalidOperation("Failed to acquire metrics write lock".to_string()))?;
            metrics.active_alerts.push(alert);
        }

        // Update alert state
        {
            let mut alert_state = self.alert_state.write()
                .map_err(|_| DockLockError::InvalidOperation("Failed to acquire alert state write lock".to_string()))?;
            alert_state.insert(alert_type, timestamp);
        }

        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| DockLockError::InvalidOperation("Failed to acquire stats write lock".to_string()))?;
            stats.total_alerts_generated += 1;
        }

        warn!("Dashboard alert generated: {}", message);
        Ok(())
    }

    /// Record event in blockbook for audit trail
    fn record_event_in_blockbook(&self, event: &TrafficEvent) -> DockLockResult<()> {
        let blockbook = self.blockbook.write()
            .map_err(|_| DockLockError::InvalidOperation("Failed to acquire blockbook write lock".to_string()))?;
        
        let event_data = serde_json::to_vec(event)
            .map_err(|e| DockLockError::EncodingError(format!("Failed to serialize traffic event: {}", e)))?;

        blockbook.record_event(
            BlockbookEventType::TrafficLightDecision,
            event.severity.clone(),
            format!("Dashboard event: {}", event.event_id),
            event_data,
            None,
        )?;

        Ok(())
    }

    /// Sign dashboard data for integrity verification
    pub fn sign_dashboard_data(&self, data: &[u8]) -> DockLockResult<Signature> {
        let hash = domain_hash(DASHBOARD_HASH, data);
        let signature = self.signing_key.sign(&hash);
        Ok(signature)
    }

    /// Verify dashboard data signature
    pub fn verify_dashboard_signature(&self, data: &[u8], signature: &Signature) -> DockLockResult<bool> {
        let hash = domain_hash(DASHBOARD_HASH, data);
        let verifying_key = self.signing_key.verifying_key();
        
        match verifying_key.verify(&hash, signature) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traffic_light::{TrafficLightPipeline, PipelineConfig};
    use crate::biso_policy::BisoPolicyEngine;
    use crate::blockbook::{Blockbook, BlockbookConfig};
    use crate::audit_book::{AuditBook, MultiCloudStorageConfig, ExportConfig, CloudProvider, RegulatoryFramework, Jurisdiction, ExportFormat, EncryptionScheme};
    use ed25519_dalek::SigningKey;

    fn create_test_components() -> (Arc<TrafficLightPipeline>, Arc<BisoPolicyEngine>, Arc<RwLock<Blockbook>>, Arc<RwLock<AuditBook>>) {
        let traffic_pipeline = Arc::new(TrafficLightPipeline::new(
            "test_pipeline".to_string(),
            PipelineConfig::default()
        ));
        
        let policy_engine = Arc::new(BisoPolicyEngine::new("test_engine".to_string()));
        
        let blockbook = Arc::new(RwLock::new(Blockbook::new(BlockbookConfig::default())));
        
        let storage_config = MultiCloudStorageConfig {
            primary_provider: CloudProvider::AWS,
            backup_providers: vec![CloudProvider::Azure],
            regions: vec!["us-east-1".to_string()],
            replication_factor: 2,
            encryption_scheme: EncryptionScheme::AES256GCM,
        };
        
        let export_config = ExportConfig {
            format: ExportFormat::JSON,
            include_sensitive: false,
            max_entries: 1000,
            require_signature: false,
            compression: false,
        };
        
        let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
        let audit_book = Arc::new(RwLock::new(AuditBook::new(storage_config, export_config, signing_key)));
        
        (traffic_pipeline, policy_engine, blockbook, audit_book)
    }

    fn create_test_decision(id: &str, state: TrafficLightState, classification: DataClassification) -> TrafficLightDecision {
        use crate::receipt::ComplianceStatus;
        TrafficLightDecision {
            decision_id: Uuid::new_v4(),
            packet_id: format!("packet_{}", id),
            state,
            classification,
            policy_id: "test_policy".to_string(),
            reason: format!("Test decision for {}", id),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            source: "test_source".to_string(),
            destination: Some("test_destination".to_string()),
            compliance_status: if state == TrafficLightState::Red { ComplianceStatus::NonCompliant } else { ComplianceStatus::Compliant },
            metadata: HashMap::new(),
            signature: None,
            signer_pubkey: None,
        }
    }

    fn create_test_policy_result(passed: bool) -> PolicyEvaluationResult {
        use crate::traffic_light::TrafficLightState;
        PolicyEvaluationResult {
            id: Uuid::new_v4(),
            policy_id: Uuid::new_v4(),
            decision: if passed { TrafficLightState::Green } else { TrafficLightState::Red },
            passed,
            reason: "Test policy evaluation".to_string(),
            violations: Vec::new(),
            warnings: Vec::new(),
            evaluated_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            evaluation_duration_ms: 10,
            signature: None,
            evaluator_pubkey: None,
        }
    }

    #[test]
    fn test_dashboard_creation() {
        let config = DashboardConfig::default();
        let (traffic_pipeline, policy_engine, blockbook, audit_book) = create_test_components();

        let dashboard = TrafficLightDashboard::new(
            config,
            traffic_pipeline,
            policy_engine,
            blockbook,
            audit_book,
        );

        assert!(dashboard.is_ok());
        let dashboard = dashboard.unwrap();
        
        let stats = dashboard.get_stats().unwrap();
        assert_eq!(stats.total_events_processed, 0);
        assert_eq!(stats.total_alerts_generated, 0);
    }

    #[test]
    fn test_traffic_event_recording() {
        let config = DashboardConfig::default();
        let (traffic_pipeline, policy_engine, blockbook, audit_book) = create_test_components();

        let dashboard = TrafficLightDashboard::new(
            config,
            traffic_pipeline,
            policy_engine,
            blockbook,
            audit_book,
        ).unwrap();

        let decision = create_test_decision("test_decision", TrafficLightState::Green, DataClassification::Public);
        let policy_result = create_test_policy_result(true);

        let result = dashboard.record_traffic_event(
            decision,
            GeographicRegion::US,
            DataClassification::General,
            policy_result,
            100,
        );

        assert!(result.is_ok());
        
        let stats = dashboard.get_stats().unwrap();
        assert_eq!(stats.total_events_processed, 1);
    }

    #[test]
    fn test_metrics_update() {
        let config = DashboardConfig::default();
        let (traffic_pipeline, policy_engine, blockbook, audit_book) = create_test_components();

        let dashboard = TrafficLightDashboard::new(
            config,
            traffic_pipeline,
            policy_engine,
            blockbook,
            audit_book,
        ).unwrap();

        // Record some events
        for i in 0..5 {
            let state = if i % 2 == 0 { TrafficLightState::Green } else { TrafficLightState::Yellow };
            let decision = create_test_decision(&format!("test_decision_{}", i), state, DataClassification::General);
            let policy_result = create_test_policy_result(true);

            dashboard.record_traffic_event(
                decision,
                GeographicRegion::EU,
                DataClassification::General,
                policy_result,
                100 + i * 10,
            ).unwrap();
        }

        let result = dashboard.update_metrics();
        assert!(result.is_ok());

        let metrics = dashboard.get_metrics().unwrap();
        assert_eq!(metrics.total_events, 5);
        assert_eq!(metrics.overall_compliance_rate, 1.0); // All green/yellow
        assert!(metrics.avg_latency_ms > 0.0);
    }

    #[test]
    fn test_alert_generation() {
        let mut config = DashboardConfig::default();
        config.alert_config.latency_threshold_ms = 50; // Low threshold for testing
        
        let (traffic_pipeline, policy_engine, blockbook, audit_book) = create_test_components();

        let dashboard = TrafficLightDashboard::new(
            config,
            traffic_pipeline,
            policy_engine,
            blockbook,
            audit_book,
        ).unwrap();

        let decision = create_test_decision("high_latency_test", TrafficLightState::Green, DataClassification::PII);
        let policy_result = create_test_policy_result(true);

        // Record event with high latency
        let result = dashboard.record_traffic_event(
            decision,
            GeographicRegion::JP,
            DataClassification::PII,
            policy_result,
            1000, // High latency
        );

        assert!(result.is_ok());
        
        let stats = dashboard.get_stats().unwrap();
        assert_eq!(stats.total_alerts_generated, 1);
    }

    #[test]
    fn test_red_event_alert() {
        let mut config = DashboardConfig::default();
        config.alert_config.red_threshold_per_minute = 2; // Low threshold for testing
        
        let (traffic_pipeline, policy_engine, blockbook, audit_book) = create_test_components();

        let dashboard = TrafficLightDashboard::new(
            config,
            traffic_pipeline,
            policy_engine,
            blockbook,
            audit_book,
        ).unwrap();

        // Record multiple red events
        for i in 0..5 {
            let decision = create_test_decision(&format!("blocked_event_{}", i), TrafficLightState::Red, DataClassification::PHI);
            let policy_result = create_test_policy_result(false);

            dashboard.record_traffic_event(
                decision,
                GeographicRegion::Global,
                DataClassification::PHI,
                policy_result,
                200,
            ).unwrap();
        }
        
        let stats = dashboard.get_stats().unwrap();
        assert!(stats.total_alerts_generated > 0);
    }

    #[test]
    fn test_alert_acknowledgment() {
        let config = DashboardConfig::default();
        let (traffic_pipeline, policy_engine, blockbook, audit_book) = create_test_components();

        let dashboard = TrafficLightDashboard::new(
            config,
            traffic_pipeline,
            policy_engine,
            blockbook,
            audit_book,
        ).unwrap();

        // Generate an alert by recording high latency event
        let decision = create_test_decision("test_alert", TrafficLightState::Yellow, DataClassification::Public);
        let policy_result = create_test_policy_result(true);

        dashboard.record_traffic_event(
            decision,
            GeographicRegion::US,
            DataClassification::Public,
            policy_result,
            2000, // High latency to trigger alert
        ).unwrap();

        let metrics = dashboard.get_metrics().unwrap();
        if let Some(alert) = metrics.active_alerts.first() {
            let alert_id = alert.alert_id.clone();
            let acknowledged = dashboard.acknowledge_alert(&alert_id).unwrap();
            assert!(acknowledged);
        }
    }

    #[test]
    fn test_compliance_report() {
        let config = DashboardConfig::default();
        let (traffic_pipeline, policy_engine, blockbook, audit_book) = create_test_components();

        let dashboard = TrafficLightDashboard::new(
            config,
            traffic_pipeline,
            policy_engine,
            blockbook,
            audit_book,
        ).unwrap();

        let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        // Record mixed events
        for i in 0..10 {
            let state = match i % 3 {
                0 => TrafficLightState::Green,
                1 => TrafficLightState::Yellow,
                _ => TrafficLightState::Red,
            };

            let decision = create_test_decision(&format!("report_test_event_{}", i), state, DataClassification::General);
            let policy_result = create_test_policy_result(state != TrafficLightState::Red);

            dashboard.record_traffic_event(
                decision,
                GeographicRegion::EU,
                DataClassification::General,
                policy_result,
                150,
            ).unwrap();
        }

        let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 1;
        let report = dashboard.generate_compliance_report(start_time, end_time).unwrap();

        assert_eq!(report.total_events, 10);
        assert!(report.compliance_rate > 0.0);
        assert!(report.compliance_rate <= 1.0);
    }

    #[test]
    fn test_dashboard_signature() {
        let config = DashboardConfig::default();
        let (traffic_pipeline, policy_engine, blockbook, audit_book) = create_test_components();

        let dashboard = TrafficLightDashboard::new(
            config,
            traffic_pipeline,
            policy_engine,
            blockbook,
            audit_book,
        ).unwrap();

        let test_data = b"test dashboard data";
        let signature = dashboard.sign_dashboard_data(test_data).unwrap();
        let is_valid = dashboard.verify_dashboard_signature(test_data, &signature).unwrap();
        
        assert!(is_valid);

        // Test with different data
        let different_data = b"different data";
        let is_valid_different = dashboard.verify_dashboard_signature(different_data, &signature).unwrap();
        assert!(!is_valid_different);
    }

    #[test]
    fn test_regional_statistics() {
        let config = DashboardConfig::default();
        let (traffic_pipeline, policy_engine, blockbook, audit_book) = create_test_components();

        let dashboard = TrafficLightDashboard::new(
            config,
            traffic_pipeline,
            policy_engine,
            blockbook,
            audit_book,
        ).unwrap();

        // Record events for different regions
        let regions = [GeographicRegion::US, GeographicRegion::EU, GeographicRegion::JP];
        
        for (i, region) in regions.iter().enumerate() {
            for j in 0..3 {
                let decision = create_test_decision(&format!("regional_test_{}_{}", i, j), TrafficLightState::Green, DataClassification::Public);
                let policy_result = create_test_policy_result(true);

                dashboard.record_traffic_event(
                    decision,
                    region.clone(),
                    DataClassification::Public,
                    policy_result,
                    100 + (i * 50) as u64,
                ).unwrap();
            }
        }

        dashboard.update_metrics().unwrap();
        let metrics = dashboard.get_metrics().unwrap();

        // Check that regional stats are populated
        assert!(metrics.regional_stats.contains_key("US"));
        assert!(metrics.regional_stats.contains_key("EU"));
        assert!(metrics.regional_stats.contains_key("JP"));

        // Check regional event counts
        let us_stats = &metrics.regional_stats["US"];
        assert_eq!(us_stats.total_decisions, 3);
        assert_eq!(us_stats.green_count, 3);
    }
}
