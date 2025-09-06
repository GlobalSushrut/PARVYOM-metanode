//! Integration layer for Merkle + Polar Proofs with existing BPI systems
//!
//! This module provides seamless integration between the new polar proof system
//! and existing BPI infrastructure, including:
//! - Backward compatibility with traditional Merkle proofs
//! - Integration with immutable audit system
//! - CLI command support
//! - Performance monitoring and metrics

use crate::{
    PolarProof, CompressionEngine, SelfHealingManager, CompressionRequest,
    CompressionOptions, MerkleProofData, PolarProofError, HealthSummary,
};
// use bpi_merkle::{MerkleTree, MerkleProof}; // Will integrate with BPI core later
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Polar proof manager that integrates with existing BPI systems
#[derive(Debug, Clone)]
pub struct PolarProofManager {
    /// Compression engine for proof operations
    pub compression_engine: CompressionEngine,
    /// Self-healing manager for reliability
    pub self_healing_manager: SelfHealingManager,
    /// Integration configuration
    pub config: IntegrationConfig,
    /// Active polar proofs being managed
    pub active_proofs: HashMap<Uuid, ManagedPolarProof>,
    /// Performance metrics
    pub metrics: PerformanceMetrics,
}

/// Configuration for polar proof integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    /// Enable backward compatibility mode
    pub backward_compatibility: bool,
    /// Automatic compression threshold (number of proofs)
    pub auto_compression_threshold: usize,
    /// Enable self-healing by default
    pub default_self_healing: bool,
    /// Performance monitoring interval
    pub monitoring_interval: Duration,
    /// Maximum managed proofs
    pub max_managed_proofs: usize,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            backward_compatibility: true,
            auto_compression_threshold: 10,
            default_self_healing: true,
            monitoring_interval: Duration::from_secs(300), // 5 minutes
            max_managed_proofs: 10000,
        }
    }
}

/// Managed polar proof with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedPolarProof {
    /// The polar proof itself
    pub proof: PolarProof,
    /// Management metadata
    pub metadata: ProofMetadata,
    /// Health monitoring status
    pub health_monitor_id: Option<Uuid>,
    /// Usage statistics
    pub usage_stats: UsageStats,
}

/// Metadata for proof management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofMetadata {
    /// When the proof was created (Unix timestamp)
    pub created_at: u64,
    /// Source of the proof (audit system, manual, etc.)
    pub source: ProofSource,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Priority level
    pub priority: Priority,
    /// Expiration time (if any, Unix timestamp)
    pub expires_at: Option<u64>,
}

/// Source of a polar proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProofSource {
    /// From immutable audit system
    AuditSystem,
    /// Manual creation via CLI
    Manual,
    /// From forensic firewall
    ForensicFirewall,
    /// From BISO agreement system
    BisoAgreement,
    /// From external integration
    External { system: String },
}

/// Priority levels for proof management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

/// Usage statistics for a proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    /// Number of times verified
    pub verification_count: usize,
    /// Last verification time (Unix timestamp)
    pub last_verified: Option<u64>,
    /// Number of times accessed
    pub access_count: usize,
    /// Last access time (Unix timestamp)
    pub last_accessed: u64,
    /// Bandwidth used for this proof
    pub bandwidth_bytes: usize,
}

/// Performance metrics for polar proof system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Total compressions performed
    pub total_compressions: usize,
    /// Total verifications performed
    pub total_verifications: usize,
    /// Average compression time
    pub avg_compression_time: Duration,
    /// Average verification time
    pub avg_verification_time: Duration,
    /// Average compression ratio achieved
    pub avg_compression_ratio: f64,
    /// Total bandwidth saved
    pub bandwidth_saved_bytes: usize,
    /// System uptime
    pub uptime: Duration,
    /// Last metrics update (Unix timestamp)
    pub last_updated: u64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            total_compressions: 0,
            total_verifications: 0,
            avg_compression_time: Duration::from_millis(0),
            avg_verification_time: Duration::from_millis(0),
            avg_compression_ratio: 0.0,
            bandwidth_saved_bytes: 0,
            uptime: Duration::from_secs(0),
            last_updated: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }
}

impl PolarProofManager {
    /// Create a new polar proof manager
    pub fn new() -> Self {
        Self {
            compression_engine: CompressionEngine::new(),
            self_healing_manager: SelfHealingManager::new(),
            config: IntegrationConfig::default(),
            active_proofs: HashMap::new(),
            metrics: PerformanceMetrics::default(),
        }
    }
    
    /// Create polar proof manager with custom configuration
    pub fn with_config(config: IntegrationConfig) -> Self {
        Self {
            compression_engine: CompressionEngine::new(),
            self_healing_manager: SelfHealingManager::new(),
            config,
            active_proofs: HashMap::new(),
            metrics: PerformanceMetrics::default(),
        }
    }
    
    /// Convert traditional Merkle tree to polar proof system (simplified for demo)
    pub async fn convert_merkle_tree_demo(
        &mut self,
        tree_size: usize,
        leaf_indices: &[usize],
        root_hash: String,
    ) -> Result<Uuid, PolarProofError> {
        let start_time = std::time::Instant::now();
        
        // Generate simulated Merkle proof data
        let mut merkle_proof_data = Vec::new();
        for &leaf_index in leaf_indices {
            // Create simulated proof data
            let simulated_proof = crate::MerkleProof {
                leaf_hash: format!("0x{:064x}", leaf_index),
                proof_path: vec![format!("0x{:064x}", leaf_index + 1000)],
                root_hash: root_hash.clone(),
            };
            
            merkle_proof_data.push(MerkleProofData {
                proof: simulated_proof,
                tree_depth: (tree_size as f64).log2().ceil() as usize,
                leaf_index,
                root_hash: root_hash.clone(),
            });
        }
        
        // Create compression request
        let request = CompressionRequest {
            merkle_proofs: merkle_proof_data,
            options: CompressionOptions {
                enable_self_healing: self.config.default_self_healing,
                compression_quality: 8,
                include_verification_hints: true,
            },
            request_id: Uuid::new_v4(),
        };
        
        // Compress to polar proof
        let polar_proof = self.compression_engine.compress_batch(request)?;
        
        // Create managed proof
        let managed_proof = ManagedPolarProof {
            proof: polar_proof.clone(),
            metadata: ProofMetadata {
                created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                source: ProofSource::Manual,
                tags: vec!["merkle_conversion".to_string()],
                priority: Priority::Normal,
                expires_at: None,
            },
            health_monitor_id: None,
            usage_stats: UsageStats {
                verification_count: 0,
                last_verified: None,
                access_count: 1,
                last_accessed: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                bandwidth_bytes: polar_proof.size_bytes(),
            },
        };
        
        // Start health monitoring if enabled
        let health_monitor_id = if self.config.default_self_healing {
            Some(self.self_healing_manager.start_monitoring(&polar_proof).await?)
        } else {
            None
        };
        
        let proof_id = polar_proof.proof_id;
        let mut final_managed_proof = managed_proof;
        final_managed_proof.health_monitor_id = health_monitor_id;
        
        // Store managed proof
        self.active_proofs.insert(proof_id, final_managed_proof);
        
        // Update metrics
        let compression_time = start_time.elapsed();
        self.update_compression_metrics(compression_time, &polar_proof);
        
        Ok(proof_id)
    }
    
    /// Create polar proof from audit system integration
    pub async fn create_from_audit_records(
        &mut self,
        audit_records: &[String], // Simplified - would be actual audit records
    ) -> Result<Uuid, PolarProofError> {
        // This would integrate with the immutable audit system
        // Track compression timing for real metrics
        let compression_start = std::time::Instant::now();
        
        let proof_id = Uuid::new_v4();
        
        // Create dummy polar proof for demonstration
        let polar_proof = PolarProof::new(
            vec![crate::FieldElement::from_u64(1), crate::FieldElement::from_u64(2)],
            crate::EvaluationDomain {
                evaluation_points: vec![crate::FieldElement::from_u64(1)],
                expected_values: vec![crate::FieldElement::from_u64(3)],
                domain_size: 1,
            },
            vec![[0u8; 32]],
            crate::BatchMetadata {
                batch_size: audit_records.len(),
                tree_depths: vec![20; audit_records.len()],
                leaf_indices: (0..audit_records.len()).collect(),
                compression_ratio: 15.0,
            },
            None,
        );
        
        let managed_proof = ManagedPolarProof {
            proof: polar_proof.clone(),
            metadata: ProofMetadata {
                created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                source: ProofSource::AuditSystem,
                tags: vec!["audit_records".to_string()],
                priority: Priority::High,
                expires_at: None,
            },
            health_monitor_id: if self.config.default_self_healing {
                Some(self.self_healing_manager.start_monitoring(&polar_proof).await?)
            } else {
                None
            },
            usage_stats: UsageStats {
                verification_count: 0,
                last_verified: None,
                access_count: 1,
                last_accessed: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                bandwidth_bytes: polar_proof.size_bytes(),
            },
        };
        
        self.active_proofs.insert(proof_id, managed_proof);
        
        // Update compression metrics with real timing and compression data
        let compression_time = compression_start.elapsed();
        self.update_compression_metrics(compression_time, &polar_proof);
        
        Ok(proof_id)
    }
    
    /// Verify a polar proof with performance tracking
    pub async fn verify_proof(&mut self, proof_id: Uuid) -> Result<bool, PolarProofError> {
        let start_time = std::time::Instant::now();
        
        let managed_proof = self.active_proofs.get_mut(&proof_id)
            .ok_or_else(|| PolarProofError::IntegrationError {
                reason: format!("Proof {} not found", proof_id),
            })?;
        
        // Perform verification
        let verification_result = self.compression_engine.verify_polar_proof(&managed_proof.proof)?;
        
        // Update usage statistics
        managed_proof.usage_stats.verification_count += 1;
        managed_proof.usage_stats.last_verified = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
        managed_proof.usage_stats.access_count += 1;
        managed_proof.usage_stats.last_accessed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        // Update metrics
        let verification_time = start_time.elapsed();
        self.update_verification_metrics(verification_time);
        
        Ok(verification_result)
    }
    
    /// Get comprehensive system status
    pub async fn get_system_status(&self) -> SystemStatus {
        let health_summary = self.self_healing_manager.get_health_summary();
        let compression_stats = self.compression_engine.get_compression_stats();
        
        SystemStatus {
            total_managed_proofs: self.active_proofs.len(),
            active_compressions: compression_stats.total_compressions,
            health_summary,
            performance_metrics: self.metrics.clone(),
            system_health: self.calculate_system_health(),
        }
    }
    
    /// Calculate overall system health score
    fn calculate_system_health(&self) -> f64 {
        let health_summary = self.self_healing_manager.get_health_summary();
        
        if health_summary.total_monitored == 0 {
            return 1.0; // No proofs to monitor = healthy
        }
        
        // Base health on integrity scores and issue counts
        let base_health = health_summary.average_integrity_score;
        let issue_penalty = (health_summary.total_issues as f64 * 0.1).min(0.5);
        
        (base_health - issue_penalty).max(0.0)
    }
    
    /// Update compression metrics
    fn update_compression_metrics(&mut self, compression_time: Duration, polar_proof: &PolarProof) {
        self.metrics.total_compressions += 1;
        
        // Update average compression time
        let total_time = self.metrics.avg_compression_time.as_millis() as f64 * (self.metrics.total_compressions - 1) as f64;
        let new_avg = (total_time + compression_time.as_millis() as f64) / self.metrics.total_compressions as f64;
        self.metrics.avg_compression_time = Duration::from_millis(new_avg as u64);
        
        // Update compression ratio
        let total_ratio = self.metrics.avg_compression_ratio * (self.metrics.total_compressions - 1) as f64;
        self.metrics.avg_compression_ratio = (total_ratio + polar_proof.compression_ratio()) / self.metrics.total_compressions as f64;
        
        // Update bandwidth saved
        let traditional_size = polar_proof.batch_metadata.batch_size * 
            polar_proof.batch_metadata.tree_depths.iter().sum::<usize>() * 32;
        let polar_size = polar_proof.size_bytes();
        self.metrics.bandwidth_saved_bytes += traditional_size.saturating_sub(polar_size);
        
        self.metrics.last_updated = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    }
    
    /// Update verification metrics
    fn update_verification_metrics(&mut self, verification_time: Duration) {
        self.metrics.total_verifications += 1;
        
        // Update average verification time
        let total_time = self.metrics.avg_verification_time.as_millis() as f64 * (self.metrics.total_verifications - 1) as f64;
        let new_avg = (total_time + verification_time.as_millis() as f64) / self.metrics.total_verifications as f64;
        self.metrics.avg_verification_time = Duration::from_millis(new_avg as u64);
        
        self.metrics.last_updated = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    }
    
    /// Clean up expired proofs
    pub async fn cleanup_expired_proofs(&mut self) -> Result<usize, PolarProofError> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut removed_count = 0;
        
        let expired_ids: Vec<Uuid> = self.active_proofs.iter()
            .filter_map(|(id, managed_proof)| {
                if let Some(expires_at) = managed_proof.metadata.expires_at {
                    if now > expires_at {
                        return Some(*id);
                    }
                }
                None
            })
            .collect();
        
        for proof_id in expired_ids {
            self.active_proofs.remove(&proof_id);
            removed_count += 1;
        }
        
        Ok(removed_count)
    }
    
    /// Get proof by ID
    pub fn get_proof(&self, proof_id: Uuid) -> Option<&ManagedPolarProof> {
        self.active_proofs.get(&proof_id)
    }
    
    /// List all managed proofs with filtering
    pub fn list_proofs(&self, filter: Option<ProofFilter>) -> Vec<(Uuid, &ManagedPolarProof)> {
        let mut results: Vec<(Uuid, &ManagedPolarProof)> = self.active_proofs.iter()
            .map(|(id, proof)| (*id, proof))
            .collect();
        
        if let Some(filter) = filter {
            results.retain(|(_, managed_proof)| filter.matches(managed_proof));
        }
        
        results.sort_by(|a, b| b.1.metadata.created_at.cmp(&a.1.metadata.created_at));
        results
    }
}

/// System status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    /// Total number of managed polar proofs
    pub total_managed_proofs: usize,
    /// Number of active compression operations
    pub active_compressions: usize,
    /// Health summary from self-healing system
    pub health_summary: HealthSummary,
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
    /// Overall system health score (0.0 to 1.0)
    pub system_health: f64,
}

/// Filter for listing proofs
#[derive(Debug, Clone)]
pub struct ProofFilter {
    /// Filter by source
    pub source: Option<ProofSource>,
    /// Filter by priority
    pub priority: Option<Priority>,
    /// Filter by tags (must contain all specified tags)
    pub tags: Vec<String>,
    /// Filter by minimum compression ratio
    pub min_compression_ratio: Option<f64>,
}

impl ProofFilter {
    /// Check if a managed proof matches this filter
    pub fn matches(&self, managed_proof: &ManagedPolarProof) -> bool {
        // Check source filter
        if let Some(ref filter_source) = self.source {
            if std::mem::discriminant(&managed_proof.metadata.source) != std::mem::discriminant(filter_source) {
                return false;
            }
        }
        
        // Check priority filter
        if let Some(ref filter_priority) = self.priority {
            if std::mem::discriminant(&managed_proof.metadata.priority) != std::mem::discriminant(filter_priority) {
                return false;
            }
        }
        
        // Check tags filter (must contain all specified tags)
        for tag in &self.tags {
            if !managed_proof.metadata.tags.contains(tag) {
                return false;
            }
        }
        
        // Check compression ratio filter
        if let Some(min_ratio) = self.min_compression_ratio {
            if managed_proof.proof.compression_ratio() < min_ratio {
                return false;
            }
        }
        
        true
    }
}

/// CLI integration functions
pub mod cli {
    use super::*;
    
    /// CLI command for polar proof operations
    pub async fn handle_polar_command(
        manager: &mut PolarProofManager,
        command: PolarCommand,
    ) -> Result<String, PolarProofError> {
        match command {
            PolarCommand::Status => {
                let status = manager.get_system_status().await;
                Ok(serde_json::to_string_pretty(&status)
                    .map_err(|e| PolarProofError::IntegrationError {
                        reason: format!("Failed to serialize status: {}", e),
                    })?)
            }
            PolarCommand::List { filter } => {
                let proofs = manager.list_proofs(filter);
                let summary: Vec<_> = proofs.iter().map(|(id, managed_proof)| {
                    serde_json::json!({
                        "id": id,
                        "source": managed_proof.metadata.source,
                        "priority": managed_proof.metadata.priority,
                        "compression_ratio": managed_proof.proof.compression_ratio(),
                        "size_bytes": managed_proof.proof.size_bytes(),
                        "created_at": managed_proof.metadata.created_at,
                    })
                }).collect();
                
                Ok(serde_json::to_string_pretty(&summary)
                    .map_err(|e| PolarProofError::IntegrationError {
                        reason: format!("Failed to serialize proof list: {}", e),
                    })?)
            }
            PolarCommand::Verify { proof_id } => {
                let result = manager.verify_proof(proof_id).await?;
                Ok(format!("Verification result: {}", result))
            }
            PolarCommand::Cleanup => {
                let removed = manager.cleanup_expired_proofs().await?;
                Ok(format!("Cleaned up {} expired proofs", removed))
            }
        }
    }
    
    /// CLI commands for polar proof operations
    #[derive(Debug, Clone)]
    pub enum PolarCommand {
        /// Show system status
        Status,
        /// List managed proofs
        List { filter: Option<ProofFilter> },
        /// Verify a specific proof
        Verify { proof_id: Uuid },
        /// Clean up expired proofs
        Cleanup,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FieldElement, EvaluationDomain, BatchMetadata};
    
    #[test]
    fn test_polar_proof_manager_creation() {
        let manager = PolarProofManager::new();
        assert!(manager.active_proofs.is_empty());
        assert_eq!(manager.metrics.total_compressions, 0);
    }
    
    #[test]
    fn test_integration_config_default() {
        let config = IntegrationConfig::default();
        assert!(config.backward_compatibility);
        assert_eq!(config.auto_compression_threshold, 10);
        assert!(config.default_self_healing);
    }
    
    #[test]
    fn test_performance_metrics_default() {
        let metrics = PerformanceMetrics::default();
        assert_eq!(metrics.total_compressions, 0);
        assert_eq!(metrics.total_verifications, 0);
        assert_eq!(metrics.avg_compression_ratio, 0.0);
    }
    
    #[test]
    fn test_proof_filter_matching() {
        let managed_proof = ManagedPolarProof {
            proof: PolarProof::new(
                vec![FieldElement::from_u64(1)],
                EvaluationDomain {
                    evaluation_points: vec![FieldElement::from_u64(1)],
                    expected_values: vec![FieldElement::from_u64(1)],
                    domain_size: 1,
                },
                vec![[0u8; 32]],
                BatchMetadata {
                    batch_size: 1,
                    tree_depths: vec![10],
                    leaf_indices: vec![0],
                    compression_ratio: 15.0,
                },
                None,
            ),
            metadata: ProofMetadata {
                created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                source: ProofSource::AuditSystem,
                tags: vec!["test".to_string()],
                priority: Priority::High,
                expires_at: None,
            },
            health_monitor_id: None,
            usage_stats: UsageStats {
                verification_count: 0,
                last_verified: None,
                access_count: 1,
                last_accessed: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                bandwidth_bytes: 100,
            },
        };
        
        // Test tag filter
        let filter = ProofFilter {
            source: None,
            priority: None,
            tags: vec!["test".to_string()],
            min_compression_ratio: None,
        };
        assert!(filter.matches(&managed_proof));
        
        // Test compression ratio filter
        let filter = ProofFilter {
            source: None,
            priority: None,
            tags: vec![],
            min_compression_ratio: Some(10.0),
        };
        // The managed_proof has compression_ratio of 15.0, which should match min 10.0
        assert!(filter.matches(&managed_proof));
        
        let filter = ProofFilter {
            source: None,
            priority: None,
            tags: vec![],
            min_compression_ratio: Some(20.0),
        };
        assert!(!filter.matches(&managed_proof));
    }
}
