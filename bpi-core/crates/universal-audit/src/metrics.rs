//! Lightweight Metrics - BPI-native metrics collection for audit system
//!
//! Uses simple in-memory metrics following BPI patterns instead of heavy Prometheus

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

/// Lightweight audit system metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSystemMetrics {
    /// System uptime in seconds
    pub uptime_seconds: u64,
    /// Total audit events processed
    pub total_events: u64,
    /// Events processed per second
    pub events_per_second: f64,
    /// Events by runtime type
    pub events_by_runtime: HashMap<String, u64>,
    /// Events by audit level
    pub events_by_level: HashMap<String, u64>,
    /// Storage metrics
    pub storage_metrics: StorageMetrics,
    /// Capture metrics
    pub capture_metrics: CaptureMetrics,
    /// Export metrics
    pub export_metrics: ExportMetrics,
    /// Error metrics
    pub error_metrics: ErrorMetrics,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

/// Storage-related metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    /// Total storage size in bytes
    pub total_size_bytes: u64,
    /// Number of stored events
    pub stored_events: u64,
    /// Storage utilization percentage
    pub utilization_percent: f64,
    /// Average event size in bytes
    pub avg_event_size_bytes: f64,
    /// Cleanup operations performed
    pub cleanup_operations: u64,
}

/// Capture-related metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureMetrics {
    /// Active capture sessions
    pub active_sessions: u32,
    /// Total capture operations
    pub total_captures: u64,
    /// Average capture latency in microseconds
    pub avg_capture_latency_us: f64,
    /// Capture success rate percentage
    pub success_rate_percent: f64,
    /// Failed captures
    pub failed_captures: u64,
}

/// Export-related metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMetrics {
    /// Total exports performed
    pub total_exports: u64,
    /// Exports by format
    pub exports_by_format: HashMap<String, u64>,
    /// Average export size in bytes
    pub avg_export_size_bytes: f64,
    /// Export success rate percentage
    pub success_rate_percent: f64,
    /// Failed exports
    pub failed_exports: u64,
}

/// Error-related metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMetrics {
    /// Total errors encountered
    pub total_errors: u64,
    /// Errors by type
    pub errors_by_type: HashMap<String, u64>,
    /// Error rate per hour
    pub error_rate_per_hour: f64,
    /// Last error timestamp
    pub last_error: Option<DateTime<Utc>>,
}

/// Metrics collector for audit system
#[derive(Debug)]
pub struct MetricsCollector {
    /// Current metrics
    metrics: Arc<RwLock<AuditSystemMetrics>>,
    /// Metrics collection start time
    start_time: DateTime<Utc>,
    /// Metrics configuration
    config: MetricsConfig,
}

/// Metrics collection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Collection interval in seconds
    pub collection_interval_seconds: u64,
    /// Enable detailed metrics
    pub enable_detailed_metrics: bool,
    /// Metrics retention period in hours
    pub retention_hours: u32,
    /// Maximum metrics history size
    pub max_history_size: usize,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            collection_interval_seconds: 60, // 1 minute
            enable_detailed_metrics: true,
            retention_hours: 24, // 24 hours
            max_history_size: 1440, // 24 hours of minute-by-minute data
        }
    }
}

impl Default for AuditSystemMetrics {
    fn default() -> Self {
        Self {
            uptime_seconds: 0,
            total_events: 0,
            events_per_second: 0.0,
            events_by_runtime: HashMap::new(),
            events_by_level: HashMap::new(),
            storage_metrics: StorageMetrics::default(),
            capture_metrics: CaptureMetrics::default(),
            export_metrics: ExportMetrics::default(),
            error_metrics: ErrorMetrics::default(),
            last_updated: Utc::now(),
        }
    }
}

impl Default for StorageMetrics {
    fn default() -> Self {
        Self {
            total_size_bytes: 0,
            stored_events: 0,
            utilization_percent: 0.0,
            avg_event_size_bytes: 0.0,
            cleanup_operations: 0,
        }
    }
}

impl Default for CaptureMetrics {
    fn default() -> Self {
        Self {
            active_sessions: 0,
            total_captures: 0,
            avg_capture_latency_us: 0.0,
            success_rate_percent: 100.0,
            failed_captures: 0,
        }
    }
}

impl Default for ExportMetrics {
    fn default() -> Self {
        Self {
            total_exports: 0,
            exports_by_format: HashMap::new(),
            avg_export_size_bytes: 0.0,
            success_rate_percent: 100.0,
            failed_exports: 0,
        }
    }
}

impl Default for ErrorMetrics {
    fn default() -> Self {
        Self {
            total_errors: 0,
            errors_by_type: HashMap::new(),
            error_rate_per_hour: 0.0,
            last_error: None,
        }
    }
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new(config: MetricsConfig) -> Self {
        Self {
            metrics: Arc::new(RwLock::new(AuditSystemMetrics::default())),
            start_time: Utc::now(),
            config,
        }
    }
    
    /// Get current metrics
    pub async fn get_metrics(&self) -> AuditSystemMetrics {
        let mut metrics = self.metrics.write().await;
        
        // Update uptime
        metrics.uptime_seconds = (Utc::now() - self.start_time).num_seconds() as u64;
        metrics.last_updated = Utc::now();
        
        metrics.clone()
    }
    
    /// Record an audit event
    pub async fn record_event(&self, runtime_type: &str, audit_level: &str) {
        let mut metrics = self.metrics.write().await;
        
        metrics.total_events += 1;
        
        // Update runtime type counter
        *metrics.events_by_runtime.entry(runtime_type.to_string()).or_insert(0) += 1;
        
        // Update audit level counter
        *metrics.events_by_level.entry(audit_level.to_string()).or_insert(0) += 1;
        
        // Calculate events per second (simple moving average)
        let uptime = (Utc::now() - self.start_time).num_seconds() as u64;
        if uptime > 0 {
            metrics.events_per_second = metrics.total_events as f64 / uptime as f64;
        }
    }
    
    /// Update storage metrics
    pub async fn update_storage_metrics(&self, size_bytes: u64, event_count: u64, max_capacity: u64) {
        let mut metrics = self.metrics.write().await;
        
        metrics.storage_metrics.total_size_bytes = size_bytes;
        metrics.storage_metrics.stored_events = event_count;
        
        if max_capacity > 0 {
            metrics.storage_metrics.utilization_percent = 
                (event_count as f64 / max_capacity as f64) * 100.0;
        }
        
        if event_count > 0 {
            metrics.storage_metrics.avg_event_size_bytes = size_bytes as f64 / event_count as f64;
        }
    }
    
    /// Record storage cleanup
    pub async fn record_storage_cleanup(&self) {
        let mut metrics = self.metrics.write().await;
        metrics.storage_metrics.cleanup_operations += 1;
    }
    
    /// Update capture metrics
    pub async fn update_capture_metrics(&self, active_sessions: u32, total_captures: u64, 
                                       avg_latency_us: f64, failed_captures: u64) {
        let mut metrics = self.metrics.write().await;
        
        metrics.capture_metrics.active_sessions = active_sessions;
        metrics.capture_metrics.total_captures = total_captures;
        metrics.capture_metrics.avg_capture_latency_us = avg_latency_us;
        metrics.capture_metrics.failed_captures = failed_captures;
        
        // Calculate success rate
        if total_captures > 0 {
            let successful_captures = total_captures - failed_captures;
            metrics.capture_metrics.success_rate_percent = 
                (successful_captures as f64 / total_captures as f64) * 100.0;
        }
    }
    
    /// Record export operation
    pub async fn record_export(&self, format: &str, size_bytes: u64, success: bool) {
        let mut metrics = self.metrics.write().await;
        
        metrics.export_metrics.total_exports += 1;
        
        // Update format counter
        *metrics.export_metrics.exports_by_format.entry(format.to_string()).or_insert(0) += 1;
        
        // Update average size (simple moving average)
        let total = metrics.export_metrics.total_exports as f64;
        let current_avg = metrics.export_metrics.avg_export_size_bytes;
        metrics.export_metrics.avg_export_size_bytes = 
            (current_avg * (total - 1.0) + size_bytes as f64) / total;
        
        // Update success rate
        if !success {
            metrics.export_metrics.failed_exports += 1;
        }
        
        let successful_exports = metrics.export_metrics.total_exports - metrics.export_metrics.failed_exports;
        metrics.export_metrics.success_rate_percent = 
            (successful_exports as f64 / metrics.export_metrics.total_exports as f64) * 100.0;
    }
    
    /// Record an error
    pub async fn record_error(&self, error_type: &str) {
        let mut metrics = self.metrics.write().await;
        
        metrics.error_metrics.total_errors += 1;
        metrics.error_metrics.last_error = Some(Utc::now());
        
        // Update error type counter
        *metrics.error_metrics.errors_by_type.entry(error_type.to_string()).or_insert(0) += 1;
        
        // Calculate error rate per hour
        let uptime_hours = (Utc::now() - self.start_time).num_hours() as f64;
        if uptime_hours > 0.0 {
            metrics.error_metrics.error_rate_per_hour = 
                metrics.error_metrics.total_errors as f64 / uptime_hours;
        }
    }
    
    /// Get metrics summary as text
    pub async fn get_metrics_summary(&self) -> String {
        let metrics = self.get_metrics().await;
        
        format!(
            "Universal Audit System Metrics\n\
             ==============================\n\
             Uptime: {} seconds\n\
             Total Events: {}\n\
             Events/Second: {:.2}\n\
             Storage Utilization: {:.1}%\n\
             Active Capture Sessions: {}\n\
             Capture Success Rate: {:.1}%\n\
             Total Exports: {}\n\
             Export Success Rate: {:.1}%\n\
             Total Errors: {}\n\
             Error Rate/Hour: {:.2}\n\
             Last Updated: {}",
            metrics.uptime_seconds,
            metrics.total_events,
            metrics.events_per_second,
            metrics.storage_metrics.utilization_percent,
            metrics.capture_metrics.active_sessions,
            metrics.capture_metrics.success_rate_percent,
            metrics.export_metrics.total_exports,
            metrics.export_metrics.success_rate_percent,
            metrics.error_metrics.total_errors,
            metrics.error_metrics.error_rate_per_hour,
            metrics.last_updated.format("%Y-%m-%d %H:%M:%S UTC")
        )
    }
    
    /// Reset all metrics
    pub async fn reset_metrics(&self) {
        let mut metrics = self.metrics.write().await;
        *metrics = AuditSystemMetrics::default();
    }
    
    /// Check if metrics indicate system health issues
    pub async fn is_system_healthy(&self) -> bool {
        let metrics = self.get_metrics().await;
        
        // Simple health checks
        metrics.capture_metrics.success_rate_percent > 90.0 &&
        metrics.export_metrics.success_rate_percent > 95.0 &&
        metrics.error_metrics.error_rate_per_hour < 10.0 &&
        metrics.storage_metrics.utilization_percent < 95.0
    }
}

/// Metrics reporter for formatted output
pub struct MetricsReporter;

impl MetricsReporter {
    /// Generate JSON report
    pub fn generate_json_report(metrics: &AuditSystemMetrics) -> Result<String> {
        Ok(serde_json::to_string_pretty(metrics)?)
    }
    
    /// Generate CSV report
    pub fn generate_csv_report(metrics: &AuditSystemMetrics) -> String {
        let mut csv = String::new();
        csv.push_str("Metric,Value\n");
        csv.push_str(&format!("Uptime (seconds),{}\n", metrics.uptime_seconds));
        csv.push_str(&format!("Total Events,{}\n", metrics.total_events));
        csv.push_str(&format!("Events Per Second,{:.2}\n", metrics.events_per_second));
        csv.push_str(&format!("Storage Size (bytes),{}\n", metrics.storage_metrics.total_size_bytes));
        csv.push_str(&format!("Storage Utilization (%),{:.1}\n", metrics.storage_metrics.utilization_percent));
        csv.push_str(&format!("Active Sessions,{}\n", metrics.capture_metrics.active_sessions));
        csv.push_str(&format!("Total Exports,{}\n", metrics.export_metrics.total_exports));
        csv.push_str(&format!("Total Errors,{}\n", metrics.error_metrics.total_errors));
        csv
    }
    
    /// Generate human-readable report
    pub fn generate_human_report(metrics: &AuditSystemMetrics) -> String {
        let mut report = String::new();
        
        report.push_str("🔍 Universal Audit System Status Report\n");
        report.push_str("=====================================\n\n");
        
        report.push_str("📊 **System Overview**\n");
        report.push_str(&format!("   • Uptime: {} seconds\n", metrics.uptime_seconds));
        report.push_str(&format!("   • Total Events Processed: {}\n", metrics.total_events));
        report.push_str(&format!("   • Processing Rate: {:.2} events/second\n\n", metrics.events_per_second));
        
        report.push_str("💾 **Storage Metrics**\n");
        report.push_str(&format!("   • Total Size: {} bytes\n", metrics.storage_metrics.total_size_bytes));
        report.push_str(&format!("   • Stored Events: {}\n", metrics.storage_metrics.stored_events));
        report.push_str(&format!("   • Utilization: {:.1}%\n", metrics.storage_metrics.utilization_percent));
        report.push_str(&format!("   • Average Event Size: {:.0} bytes\n\n", metrics.storage_metrics.avg_event_size_bytes));
        
        report.push_str("🎯 **Capture Performance**\n");
        report.push_str(&format!("   • Active Sessions: {}\n", metrics.capture_metrics.active_sessions));
        report.push_str(&format!("   • Total Captures: {}\n", metrics.capture_metrics.total_captures));
        report.push_str(&format!("   • Success Rate: {:.1}%\n", metrics.capture_metrics.success_rate_percent));
        report.push_str(&format!("   • Average Latency: {:.0}μs\n\n", metrics.capture_metrics.avg_capture_latency_us));
        
        report.push_str("📤 **Export Statistics**\n");
        report.push_str(&format!("   • Total Exports: {}\n", metrics.export_metrics.total_exports));
        report.push_str(&format!("   • Success Rate: {:.1}%\n", metrics.export_metrics.success_rate_percent));
        report.push_str(&format!("   • Average Size: {:.0} bytes\n\n", metrics.export_metrics.avg_export_size_bytes));
        
        report.push_str("⚠️  **Error Summary**\n");
        report.push_str(&format!("   • Total Errors: {}\n", metrics.error_metrics.total_errors));
        report.push_str(&format!("   • Error Rate: {:.2}/hour\n", metrics.error_metrics.error_rate_per_hour));
        
        if let Some(last_error) = metrics.error_metrics.last_error {
            report.push_str(&format!("   • Last Error: {}\n", last_error.format("%Y-%m-%d %H:%M:%S UTC")));
        } else {
            report.push_str("   • Last Error: None\n");
        }
        
        report.push_str(&format!("\n📅 Last Updated: {}\n", 
                                metrics.last_updated.format("%Y-%m-%d %H:%M:%S UTC")));
        
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_metrics_collector_creation() {
        let config = MetricsConfig::default();
        let collector = MetricsCollector::new(config);
        
        let metrics = collector.get_metrics().await;
        assert_eq!(metrics.total_events, 0);
        assert!(collector.is_system_healthy().await);
    }
    
    #[tokio::test]
    async fn test_event_recording() {
        let config = MetricsConfig::default();
        let collector = MetricsCollector::new(config);
        
        collector.record_event("DockLock", "Standard").await;
        collector.record_event("DockLock", "Standard").await;
        collector.record_event("HttpCage", "Critical").await;
        
        let metrics = collector.get_metrics().await;
        assert_eq!(metrics.total_events, 3);
        assert_eq!(metrics.events_by_runtime.get("DockLock"), Some(&2));
        assert_eq!(metrics.events_by_runtime.get("HttpCage"), Some(&1));
        assert_eq!(metrics.events_by_level.get("Standard"), Some(&2));
        assert_eq!(metrics.events_by_level.get("Critical"), Some(&1));
    }
    
    #[tokio::test]
    async fn test_error_recording() {
        let config = MetricsConfig::default();
        let collector = MetricsCollector::new(config);
        
        collector.record_error("ConnectionError").await;
        collector.record_error("ValidationError").await;
        collector.record_error("ConnectionError").await;
        
        let metrics = collector.get_metrics().await;
        assert_eq!(metrics.error_metrics.total_errors, 3);
        assert_eq!(metrics.error_metrics.errors_by_type.get("ConnectionError"), Some(&2));
        assert_eq!(metrics.error_metrics.errors_by_type.get("ValidationError"), Some(&1));
        assert!(metrics.error_metrics.last_error.is_some());
    }
    
    #[test]
    fn test_metrics_reporting() {
        let metrics = AuditSystemMetrics::default();
        
        let json_report = MetricsReporter::generate_json_report(&metrics).unwrap();
        assert!(json_report.contains("total_events"));
        
        let csv_report = MetricsReporter::generate_csv_report(&metrics);
        assert!(csv_report.contains("Metric,Value"));
        
        let human_report = MetricsReporter::generate_human_report(&metrics);
        assert!(human_report.contains("Universal Audit System Status Report"));
    }
}
