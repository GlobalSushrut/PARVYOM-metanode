//! Gov-Index (GIDX-60) Aggregation System for GBF Architecture Stage 2
//! 
//! This module implements government signal aggregation with a 60-minute sliding window
//! for real-time compliance and security incident tracking.

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use tracing::{info, warn, error, debug};

use crate::zk3_attestation_circuits::ZK3Attestation;

/// Government Index (GIDX-60) with 60-minute sliding window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovIndex {
    pub timestamp: u64,
    pub compliance_score: f64,        // 0.0-1.0, higher is better
    pub incident_rate: f64,           // Incidents per hour
    pub exfiltration_risk: f64,       // 0.0-1.0, higher is more risk
    pub overall_security: f64,        // 0.0-1.0, higher is better
    pub jurisdiction: String,
    pub window_start: u64,
    pub window_end: u64,
    pub attestation_count: u32,
    pub vm_coverage: u32,             // Number of VMs covered
}

/// Government signal aggregator with sliding window
pub struct GovIndexAggregator {
    pub window_size: Duration,        // 60-minute sliding window
    pub attestations: VecDeque<ZK3Attestation>,
    pub current_gidx: HashMap<String, GovIndex>, // jurisdiction -> index
    pub gidx_history: VecDeque<GovIndex>,
    pub config: GovIndexConfig,
}

/// Configuration for Gov-Index aggregation
#[derive(Debug, Clone)]
pub struct GovIndexConfig {
    pub window_minutes: u64,          // Default 60 minutes
    pub update_interval_seconds: u64, // How often to recalculate
    pub min_attestations: u32,        // Minimum attestations for valid index
    pub jurisdictions: Vec<String>,   // Jurisdictions to track
    pub enable_alerts: bool,          // Enable threshold alerts
    pub compliance_threshold: f64,    // Alert if compliance drops below
    pub incident_threshold: f64,      // Alert if incidents exceed
}

/// Government signal alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovSignalAlert {
    pub alert_id: String,
    pub alert_type: AlertType,
    pub jurisdiction: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: u64,
    pub gidx_snapshot: GovIndex,
}

/// Types of government signal alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    ComplianceDropped,
    IncidentSpike,
    ExfiltrationRisk,
    SecurityDegraded,
    InsufficientData,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for GovIndexConfig {
    fn default() -> Self {
        Self {
            window_minutes: 60,
            update_interval_seconds: 300, // 5 minutes
            min_attestations: 5,
            jurisdictions: vec!["US".to_string(), "EU".to_string()],
            enable_alerts: true,
            compliance_threshold: 0.8,
            incident_threshold: 5.0,
        }
    }
}

impl GovIndexAggregator {
    /// Create new Gov-Index aggregator
    pub fn new(config: GovIndexConfig) -> Self {
        let window_size = Duration::from_secs(config.window_minutes * 60);
        
        Self {
            window_size,
            attestations: VecDeque::new(),
            current_gidx: HashMap::new(),
            gidx_history: VecDeque::new(),
            config,
        }
    }

    /// Add ZK3 attestation to sliding window
    pub async fn add_attestation(&mut self, attestation: ZK3Attestation) -> Result<Vec<GovSignalAlert>> {
        // Add to sliding window
        self.attestations.push_back(attestation);
        
        // Remove old attestations outside window
        self.cleanup_old_attestations();
        
        // Recalculate Gov-Index for all jurisdictions
        let alerts = self.recalculate_all_indices().await?;
        
        Ok(alerts)
    }

    /// Recalculate Gov-Index for all jurisdictions
    async fn recalculate_all_indices(&mut self) -> Result<Vec<GovSignalAlert>> {
        let mut alerts = Vec::new();
        
        for jurisdiction in &self.config.jurisdictions.clone() {
            let new_gidx = self.calculate_gidx_for_jurisdiction(jurisdiction).await?;
            
            // Check for alerts
            if let Some(alert) = self.check_for_alerts(&new_gidx).await? {
                alerts.push(alert);
            }
            
            // Update current index
            self.current_gidx.insert(jurisdiction.clone(), new_gidx.clone());
            
            // Add to history
            self.gidx_history.push_back(new_gidx);
        }
        
        // Cleanup old history
        self.cleanup_old_history();
        
        Ok(alerts)
    }

    /// Calculate Gov-Index for specific jurisdiction
    async fn calculate_gidx_for_jurisdiction(&self, jurisdiction: &str) -> Result<GovIndex> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let window_start = now - self.config.window_minutes * 60;
        
        // Filter attestations for this jurisdiction and time window
        let relevant_attestations: Vec<&ZK3Attestation> = self.attestations.iter()
            .filter(|a| a.jurisdiction == jurisdiction && a.timestamp >= window_start)
            .collect();
        
        if relevant_attestations.len() < self.config.min_attestations as usize {
            return Ok(GovIndex {
                timestamp: now,
                compliance_score: 0.0,
                incident_rate: 0.0,
                exfiltration_risk: 0.0,
                overall_security: 0.0,
                jurisdiction: jurisdiction.to_string(),
                window_start,
                window_end: now,
                attestation_count: relevant_attestations.len() as u32,
                vm_coverage: 0,
            });
        }

        // Calculate compliance score
        let compliance_score = self.calculate_compliance_score(&relevant_attestations)?;
        
        // Calculate incident rate (incidents per hour)
        let incident_rate = self.calculate_incident_rate(&relevant_attestations)?;
        
        // Calculate exfiltration risk
        let exfiltration_risk = self.calculate_exfiltration_risk(&relevant_attestations)?;
        
        // Calculate overall security score
        let overall_security = self.calculate_overall_security(
            compliance_score, 
            incident_rate, 
            exfiltration_risk
        )?;
        
        // Count unique VMs
        let unique_vms: std::collections::HashSet<String> = relevant_attestations.iter()
            .map(|a| a.attestation_id.split('-').nth(1).unwrap_or("unknown").to_string())
            .collect();
        
        Ok(GovIndex {
            timestamp: now,
            compliance_score,
            incident_rate,
            exfiltration_risk,
            overall_security,
            jurisdiction: jurisdiction.to_string(),
            window_start,
            window_end: now,
            attestation_count: relevant_attestations.len() as u32,
            vm_coverage: unique_vms.len() as u32,
        })
    }

    /// Calculate compliance score from attestations
    fn calculate_compliance_score(&self, attestations: &[&ZK3Attestation]) -> Result<f64> {
        if attestations.is_empty() {
            return Ok(0.0);
        }

        let compliant_count = attestations.iter()
            .filter(|a| a.compliance_ok)
            .count();
        
        let weighted_score: f64 = attestations.iter()
            .map(|a| if a.compliance_ok { a.confidence_score } else { 0.0 })
            .sum();
        
        let total_weight: f64 = attestations.iter()
            .map(|a| a.confidence_score)
            .sum();
        
        if total_weight > 0.0 {
            Ok(weighted_score / total_weight)
        } else {
            Ok(compliant_count as f64 / attestations.len() as f64)
        }
    }

    /// Calculate incident rate (incidents per hour)
    fn calculate_incident_rate(&self, attestations: &[&ZK3Attestation]) -> Result<f64> {
        if attestations.is_empty() {
            return Ok(0.0);
        }

        let incident_count = attestations.iter()
            .filter(|a| a.incident_seen)
            .count();
        
        // Convert to incidents per hour
        let window_hours = self.config.window_minutes as f64 / 60.0;
        Ok(incident_count as f64 / window_hours)
    }

    /// Calculate exfiltration risk score
    fn calculate_exfiltration_risk(&self, attestations: &[&ZK3Attestation]) -> Result<f64> {
        if attestations.is_empty() {
            return Ok(0.0);
        }

        let exfil_count = attestations.iter()
            .filter(|a| a.exfil_suspected)
            .count();
        
        let risk_ratio = exfil_count as f64 / attestations.len() as f64;
        
        // Weight by confidence scores
        let weighted_risk: f64 = attestations.iter()
            .map(|a| if a.exfil_suspected { a.confidence_score } else { 0.0 })
            .sum();
        
        let total_weight: f64 = attestations.iter()
            .map(|a| a.confidence_score)
            .sum();
        
        if total_weight > 0.0 {
            Ok(weighted_risk / total_weight)
        } else {
            Ok(risk_ratio)
        }
    }

    /// Calculate overall security score
    fn calculate_overall_security(
        &self,
        compliance_score: f64,
        incident_rate: f64,
        exfiltration_risk: f64,
    ) -> Result<f64> {
        // Normalize incident rate (assume max 10 incidents/hour is worst case)
        let normalized_incident_rate = (10.0 - incident_rate.min(10.0)) / 10.0;
        
        // Invert exfiltration risk (lower risk = higher security)
        let security_from_exfil = 1.0 - exfiltration_risk;
        
        // Weighted average (compliance is most important)
        let overall = (compliance_score * 0.5) + 
                     (normalized_incident_rate * 0.3) + 
                     (security_from_exfil * 0.2);
        
        Ok(overall.max(0.0).min(1.0))
    }

    /// Check for government signal alerts
    async fn check_for_alerts(&self, gidx: &GovIndex) -> Result<Option<GovSignalAlert>> {
        if !self.config.enable_alerts {
            return Ok(None);
        }

        // Check compliance threshold
        if gidx.compliance_score < self.config.compliance_threshold {
            return Ok(Some(GovSignalAlert {
                alert_id: format!("alert-compliance-{}", uuid::Uuid::new_v4()),
                alert_type: AlertType::ComplianceDropped,
                jurisdiction: gidx.jurisdiction.clone(),
                severity: if gidx.compliance_score < 0.5 { AlertSeverity::Critical } else { AlertSeverity::High },
                message: format!("Compliance score dropped to {:.2} in {}", gidx.compliance_score, gidx.jurisdiction),
                timestamp: gidx.timestamp,
                gidx_snapshot: gidx.clone(),
            }));
        }

        // Check incident rate threshold
        if gidx.incident_rate > self.config.incident_threshold {
            return Ok(Some(GovSignalAlert {
                alert_id: format!("alert-incidents-{}", uuid::Uuid::new_v4()),
                alert_type: AlertType::IncidentSpike,
                jurisdiction: gidx.jurisdiction.clone(),
                severity: if gidx.incident_rate > 10.0 { AlertSeverity::Critical } else { AlertSeverity::High },
                message: format!("Incident rate spiked to {:.1}/hour in {}", gidx.incident_rate, gidx.jurisdiction),
                timestamp: gidx.timestamp,
                gidx_snapshot: gidx.clone(),
            }));
        }

        // Check exfiltration risk
        if gidx.exfiltration_risk > 0.7 {
            return Ok(Some(GovSignalAlert {
                alert_id: format!("alert-exfil-{}", uuid::Uuid::new_v4()),
                alert_type: AlertType::ExfiltrationRisk,
                jurisdiction: gidx.jurisdiction.clone(),
                severity: AlertSeverity::Critical,
                message: format!("High exfiltration risk ({:.2}) detected in {}", gidx.exfiltration_risk, gidx.jurisdiction),
                timestamp: gidx.timestamp,
                gidx_snapshot: gidx.clone(),
            }));
        }

        // Check insufficient data
        if gidx.attestation_count < self.config.min_attestations {
            return Ok(Some(GovSignalAlert {
                alert_id: format!("alert-data-{}", uuid::Uuid::new_v4()),
                alert_type: AlertType::InsufficientData,
                jurisdiction: gidx.jurisdiction.clone(),
                severity: AlertSeverity::Medium,
                message: format!("Insufficient attestations ({}) for reliable index in {}", gidx.attestation_count, gidx.jurisdiction),
                timestamp: gidx.timestamp,
                gidx_snapshot: gidx.clone(),
            }));
        }

        Ok(None)
    }

    /// Remove old attestations outside sliding window
    fn cleanup_old_attestations(&mut self) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let cutoff = now - self.config.window_minutes * 60;
        
        while let Some(attestation) = self.attestations.front() {
            if attestation.timestamp < cutoff {
                self.attestations.pop_front();
            } else {
                break;
            }
        }
    }

    /// Remove old Gov-Index history
    fn cleanup_old_history(&mut self) {
        let max_history = 24 * 60 / self.config.update_interval_seconds; // 24 hours of history
        
        while self.gidx_history.len() > max_history as usize {
            self.gidx_history.pop_front();
        }
    }

    /// Get current Gov-Index for jurisdiction
    pub fn get_current_gidx(&self, jurisdiction: &str) -> Option<&GovIndex> {
        self.current_gidx.get(jurisdiction)
    }

    /// Get Gov-Index history
    pub fn get_gidx_history(&self, jurisdiction: &str, hours: u32) -> Vec<&GovIndex> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let cutoff = now - (hours as u64 * 3600);
        
        self.gidx_history.iter()
            .filter(|gidx| gidx.jurisdiction == jurisdiction && gidx.timestamp >= cutoff)
            .collect()
    }

    /// Get aggregated statistics for jurisdiction
    pub fn get_jurisdiction_stats(&self, jurisdiction: &str) -> Result<JurisdictionStats> {
        let history = self.get_gidx_history(jurisdiction, 24);
        
        if history.is_empty() {
            return Ok(JurisdictionStats::default());
        }

        let avg_compliance = history.iter().map(|g| g.compliance_score).sum::<f64>() / history.len() as f64;
        let avg_incident_rate = history.iter().map(|g| g.incident_rate).sum::<f64>() / history.len() as f64;
        let avg_exfil_risk = history.iter().map(|g| g.exfiltration_risk).sum::<f64>() / history.len() as f64;
        let avg_security = history.iter().map(|g| g.overall_security).sum::<f64>() / history.len() as f64;
        
        let max_vm_coverage = history.iter().map(|g| g.vm_coverage).max().unwrap_or(0);
        let total_attestations: u32 = history.iter().map(|g| g.attestation_count).sum();

        Ok(JurisdictionStats {
            jurisdiction: jurisdiction.to_string(),
            avg_compliance_score: avg_compliance,
            avg_incident_rate: avg_incident_rate,
            avg_exfiltration_risk: avg_exfil_risk,
            avg_overall_security: avg_security,
            max_vm_coverage,
            total_attestations,
            data_points: history.len() as u32,
        })
    }
}

/// Aggregated statistics for a jurisdiction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JurisdictionStats {
    pub jurisdiction: String,
    pub avg_compliance_score: f64,
    pub avg_incident_rate: f64,
    pub avg_exfiltration_risk: f64,
    pub avg_overall_security: f64,
    pub max_vm_coverage: u32,
    pub total_attestations: u32,
    pub data_points: u32,
}

impl Default for JurisdictionStats {
    fn default() -> Self {
        Self {
            jurisdiction: "unknown".to_string(),
            avg_compliance_score: 0.0,
            avg_incident_rate: 0.0,
            avg_exfiltration_risk: 0.0,
            avg_overall_security: 0.0,
            max_vm_coverage: 0,
            total_attestations: 0,
            data_points: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zk3_attestation_circuits::ZK3Attestation;
    use tokio;

    #[tokio::test]
    async fn test_gov_index_calculation() {
        println!("🧪 Testing Gov-Index Calculation");
        
        let mut aggregator = GovIndexAggregator::new(GovIndexConfig::default());
        
        // Create test attestations
        let attestations = vec![
            ZK3Attestation {
                compliance_ok: true,
                incident_seen: false,
                exfil_suspected: false,
                zk_proof: vec![1u8; 32],
                vm_commitment: [1u8; 32],
                attestation_id: "zk3-vm1-test".to_string(),
                timestamp: 1000,
                jurisdiction: "US".to_string(),
                confidence_score: 0.95,
            },
            ZK3Attestation {
                compliance_ok: true,
                incident_seen: true,
                exfil_suspected: false,
                zk_proof: vec![2u8; 32],
                vm_commitment: [2u8; 32],
                attestation_id: "zk3-vm2-test".to_string(),
                timestamp: 1010,
                jurisdiction: "US".to_string(),
                confidence_score: 0.87,
            },
        ];
        
        // Add attestations
        for attestation in attestations {
            aggregator.add_attestation(attestation).await.unwrap();
        }
        
        // Get current Gov-Index
        let gidx = aggregator.get_current_gidx("US").unwrap();
        
        assert!(gidx.compliance_score > 0.0);
        assert!(gidx.incident_rate >= 0.0);
        assert_eq!(gidx.jurisdiction, "US");
        assert_eq!(gidx.attestation_count, 2);
        
        println!("✅ Gov-Index calculation working");
        println!("   - Compliance score: {:.2}", gidx.compliance_score);
        println!("   - Incident rate: {:.2}/hour", gidx.incident_rate);
        println!("   - Exfiltration risk: {:.2}", gidx.exfiltration_risk);
        println!("   - Overall security: {:.2}", gidx.overall_security);
    }

    #[tokio::test]
    async fn test_sliding_window() {
        println!("🧪 Testing Sliding Window");
        
        let mut config = GovIndexConfig::default();
        config.window_minutes = 1; // 1 minute window for testing
        
        let mut aggregator = GovIndexAggregator::new(config);
        
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        // Add old attestation (outside window)
        let old_attestation = ZK3Attestation {
            compliance_ok: true,
            incident_seen: false,
            exfil_suspected: false,
            zk_proof: vec![1u8; 32],
            vm_commitment: [1u8; 32],
            attestation_id: "old-attestation".to_string(),
            timestamp: now - 120, // 2 minutes ago
            jurisdiction: "US".to_string(),
            confidence_score: 0.9,
        };
        
        // Add recent attestation (inside window)
        let recent_attestation = ZK3Attestation {
            compliance_ok: true,
            incident_seen: false,
            exfil_suspected: false,
            zk_proof: vec![2u8; 32],
            vm_commitment: [2u8; 32],
            attestation_id: "recent-attestation".to_string(),
            timestamp: now - 30, // 30 seconds ago
            jurisdiction: "US".to_string(),
            confidence_score: 0.95,
        };
        
        aggregator.add_attestation(old_attestation).await.unwrap();
        aggregator.add_attestation(recent_attestation).await.unwrap();
        
        // Only recent attestation should be in window
        assert_eq!(aggregator.attestations.len(), 1);
        assert_eq!(aggregator.attestations[0].attestation_id, "recent-attestation");
        
        println!("✅ Sliding window working");
        println!("   - Old attestations removed: ✓");
        println!("   - Recent attestations kept: ✓");
    }

    #[tokio::test]
    async fn test_alert_generation() {
        println!("🧪 Testing Alert Generation");
        
        let mut config = GovIndexConfig::default();
        config.compliance_threshold = 0.9; // High threshold for testing
        config.incident_threshold = 1.0;   // Low threshold for testing
        
        let mut aggregator = GovIndexAggregator::new(config);
        
        // Create attestation that should trigger alerts
        let problem_attestation = ZK3Attestation {
            compliance_ok: false, // Compliance issue
            incident_seen: true,  // Incident detected
            exfil_suspected: false,
            zk_proof: vec![1u8; 32],
            vm_commitment: [1u8; 32],
            attestation_id: "problem-attestation".to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            jurisdiction: "US".to_string(),
            confidence_score: 0.8,
        };
        
        let alerts = aggregator.add_attestation(problem_attestation).await.unwrap();
        
        // Should generate alerts for compliance and incidents
        assert!(!alerts.is_empty());
        
        println!("✅ Alert generation working");
        println!("   - Generated {} alerts", alerts.len());
        for alert in &alerts {
            println!("   - Alert: {:?} - {}", alert.alert_type, alert.message);
        }
    }

    #[tokio::test]
    async fn test_jurisdiction_stats() {
        println!("🧪 Testing Jurisdiction Statistics");
        
        let mut aggregator = GovIndexAggregator::new(GovIndexConfig::default());
        
        // Add multiple attestations to build history
        for i in 0..10 {
            let attestation = ZK3Attestation {
                compliance_ok: i % 3 != 0, // Mix of compliant/non-compliant
                incident_seen: i % 5 == 0, // Occasional incidents
                exfil_suspected: false,
                zk_proof: vec![i as u8; 32],
                vm_commitment: [i as u8; 32],
                attestation_id: format!("attestation-{}", i),
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + i,
                jurisdiction: "US".to_string(),
                confidence_score: 0.8 + (i as f64 * 0.02),
            };
            
            aggregator.add_attestation(attestation).await.unwrap();
        }
        
        let stats = aggregator.get_jurisdiction_stats("US").unwrap();
        
        assert!(stats.avg_compliance_score > 0.0);
        assert!(stats.total_attestations > 0);
        assert_eq!(stats.jurisdiction, "US");
        
        println!("✅ Jurisdiction statistics working");
        println!("   - Average compliance: {:.2}", stats.avg_compliance_score);
        println!("   - Average incident rate: {:.2}/hour", stats.avg_incident_rate);
        println!("   - Total attestations: {}", stats.total_attestations);
        println!("   - Data points: {}", stats.data_points);
    }
}
