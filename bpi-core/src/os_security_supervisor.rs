//! OS Security Supervisor
//!
//! OS-level wrapper that unifies ImmutableAuditSystem, ForensicFirewall, and
//! BPISecurityEngine for the universal kernel. At this stage it focuses on
//! honest initialization and kernel-boot auditing without changing existing
//! network/runtime behaviour.

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use uuid::Uuid;

use crate::immutable_audit_system::ImmutableAuditSystem;
use crate::forensic_firewall::{ForensicFirewall, ForensicFirewallConfig};
use crate::security::{BPISecurityEngine, UnifiedSecurityEvent, ThreatSeverity};

/// OS-level security supervisor: owns audit, firewall, and macro security
/// engine for a single kernel instance.
#[derive(Clone, Debug)]
pub struct OsSecuritySupervisor {
    profile: String,
    node_id: String,
    audit: Arc<RwLock<ImmutableAuditSystem>>,
    firewall: Arc<ForensicFirewall>,
    security_engine: Arc<BPISecurityEngine>,
}

impl OsSecuritySupervisor {
    /// Initialize the OS Security Supervisor for a given kernel profile/node.
    pub async fn new(audit_root: &str, profile: &str, node_id: &str) -> Result<Self> {
        // Immutable audit system
        let audit_system = ImmutableAuditSystem::new(audit_root).await?;
        let audit = Arc::new(RwLock::new(audit_system));

        {
            let mut audit_guard = audit.write().await;
            audit_guard.start_continuous_runtime_auditing().await?;
        }

        // Forensic firewall configuration – fully enabled for kernel pilot use.
        let firewall_config = ForensicFirewallConfig {
            enable_cue_rules: true,
            enable_threat_intel: true,
            enable_behavioral_analysis: true,
            enable_ml_analysis: true,
            enable_dynamic_response: true,
            enable_real_time_audit: true,
            security_contracts_path: "./security/contracts".to_string(),
            performance_target_ms: 10.0,
        };

        let firewall = Arc::new(ForensicFirewall::new(audit.clone(), firewall_config).await?);

        // Macro security engine with its own audit integration.
        let security_engine = Arc::new(BPISecurityEngine::new(audit_root).await?);
        security_engine.start_security_engine().await?;

        info!(
            profile = profile,
            node_id = node_id,
            "OS Security Supervisor initialized with audit, firewall, and security engine",
        );

        Ok(Self {
            profile: profile.to_string(),
            node_id: node_id.to_string(),
            audit,
            firewall,
            security_engine,
        })
    }

    /// Record a kernel boot execution event in the immutable audit system.
    /// Errors are logged but do not abort the kernel; this mirrors previous
    /// behaviour in `start_kernel`.
    pub async fn record_kernel_boot_event(&self) {
        let mut guard = self.audit.write().await;

        if let Err(e) = guard
            .record_code_execution_event(
                "kernel_boot",
                "bpi-core",
                vec![format!("profile={}", self.profile)],
                "start_kernel",
            )
            .await
        {
            warn!(
                profile = %self.profile,
                node_id = %self.node_id,
                error = %e,
                "Failed to record kernel boot audit event",
            );
        }
    }

    pub async fn check_http_request(&self, method: &str, path: &str, remote_addr: &str) {
        self
            .check_http_request_with_source("vm_server_http", method, path, remote_addr)
            .await;
    }

    pub async fn check_http_request_with_source(
        &self,
        source_component: &str,
        method: &str,
        path: &str,
        remote_addr: &str,
    ) {
        let mut attrs = HashMap::new();
        attrs.insert("method".to_string(), method.to_string());
        attrs.insert("path".to_string(), path.to_string());
        attrs.insert("remote_addr".to_string(), remote_addr.to_string());
        attrs.insert("profile".to_string(), self.profile.clone());
        attrs.insert("node_id".to_string(), self.node_id.clone());

        let event = UnifiedSecurityEvent {
            event_id: Uuid::new_v4().to_string(),
            event_type: "http_request".to_string(),
            source_component: source_component.to_string(),
            timestamp: Utc::now(),
            severity: ThreatSeverity::Info,
            attributes: attrs,
            raw_data: String::new(),
        };

        if let Err(e) = self.security_engine.process_security_event(&event).await {
            warn!(
                profile = %self.profile,
                node_id = %self.node_id,
                error = %e,
                "Security engine failed to process HTTP request event",
            );
        }
    }

    /// Record a storage operation (e.g. CDN/bootstrap write) as a unified
    /// security event. This does not change storage behaviour; failures are
    /// logged and ignored.
    pub async fn check_storage_operation(
        &self,
        operation: &str,
        scope: &str,
        size_bytes: u64,
    ) {
        let mut attrs = HashMap::new();
        attrs.insert("operation".to_string(), operation.to_string());
        attrs.insert("scope".to_string(), scope.to_string());
        attrs.insert("size_bytes".to_string(), size_bytes.to_string());
        attrs.insert("profile".to_string(), self.profile.clone());
        attrs.insert("node_id".to_string(), self.node_id.clone());

        let event = UnifiedSecurityEvent {
            event_id: Uuid::new_v4().to_string(),
            event_type: "storage_operation".to_string(),
            source_component: "storage_fabric".to_string(),
            timestamp: Utc::now(),
            severity: ThreatSeverity::Info,
            attributes: attrs,
            raw_data: String::new(),
        };

        if let Err(e) = self.security_engine.process_security_event(&event).await {
            warn!(
                profile = %self.profile,
                node_id = %self.node_id,
                error = %e,
                "Security engine failed to process storage operation event",
            );
        }
    }

    /// Get a high-level snapshot of security-audit metrics flowing through the
    /// unified security engine. Intended for infra tests and observability.
    pub async fn get_security_audit_metrics(&self) -> Result<crate::security::security_audit_integration::SecurityAuditMetrics> {
        self.security_engine.get_audit_metrics().await
    }
}
