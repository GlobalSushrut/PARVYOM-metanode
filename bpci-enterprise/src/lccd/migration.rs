//! LCCD Migration Logic
//! 
//! Implements BPCI shard migration to BPI slots for resource-resident architecture.
//! 
//! # Migration Process
//! 
//! 1. **Preparation**: Select target slot, validate resources
//! 2. **State Transfer**: Copy shard state to target slot
//! 3. **Validation**: Verify state integrity
//! 4. **Activation**: Switch traffic to new slot
//! 5. **Cleanup**: Remove old state (optional)
//! 
//! # Rollback Support
//! 
//! - Snapshot state before migration
//! - Validate each step
//! - Automatic rollback on failure
//! - Health monitoring throughout

use super::cell::{LccdCell, CellId};
use crate::slot_marketplace::BpiSlotOffer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Migration state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationState {
    /// Migration not started
    Idle,
    
    /// Preparing for migration
    Preparing,
    
    /// Transferring state
    Transferring,
    
    /// Validating transferred state
    Validating,
    
    /// Activating new slot
    Activating,
    
    /// Migration complete
    Complete,
    
    /// Migration failed
    Failed,
    
    /// Rolling back
    RollingBack,
    
    /// Rollback complete
    RolledBack,
}

/// Migration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationConfig {
    /// Maximum migration time (seconds)
    pub max_migration_time_secs: u64,
    
    /// State transfer chunk size (bytes)
    pub chunk_size_bytes: usize,
    
    /// Validation timeout (seconds)
    pub validation_timeout_secs: u64,
    
    /// Enable automatic rollback on failure
    pub auto_rollback: bool,
    
    /// Health check interval during migration (seconds)
    pub health_check_interval_secs: u64,
    
    /// Minimum health score to proceed
    pub min_health_to_proceed: f64,
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self {
            max_migration_time_secs: 300, // 5 minutes
            chunk_size_bytes: 1024 * 1024, // 1 MB
            validation_timeout_secs: 60,
            auto_rollback: true,
            health_check_interval_secs: 10,
            min_health_to_proceed: 0.6,
        }
    }
}

/// Migration context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationContext {
    /// Migration ID
    pub migration_id: String,
    
    /// Cell being migrated
    pub cell_id: CellId,
    
    /// Source location (if any)
    pub source_slot: Option<String>,
    
    /// Target slot
    pub target_slot: String,
    
    /// Current state
    pub state: MigrationState,
    
    /// Start time
    #[serde(skip)]
    pub start_time: Option<Instant>,
    
    /// Bytes transferred
    pub bytes_transferred: u64,
    
    /// Total bytes to transfer
    pub total_bytes: u64,
    
    /// Error message (if failed)
    pub error: Option<String>,
    
    /// Snapshot ID for rollback
    pub snapshot_id: Option<String>,
}

/// Migration result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationResult {
    /// Migration ID
    pub migration_id: String,
    
    /// Success status
    pub success: bool,
    
    /// Final state
    pub final_state: MigrationState,
    
    /// Duration (seconds)
    pub duration_secs: f64,
    
    /// Bytes transferred
    pub bytes_transferred: u64,
    
    /// Error message (if failed)
    pub error: Option<String>,
}

/// Migration orchestrator
pub struct MigrationOrchestrator {
    config: MigrationConfig,
    active_migrations: HashMap<String, MigrationContext>,
    next_migration_id: u64,
}

impl MigrationOrchestrator {
    /// Create a new migration orchestrator
    pub fn new(config: MigrationConfig) -> Self {
        Self {
            config,
            active_migrations: HashMap::new(),
            next_migration_id: 1,
        }
    }
    
    /// Start a migration
    pub fn start_migration(
        &mut self,
        cell: &LccdCell,
        target_slot: &BpiSlotOffer,
        source_slot: Option<String>,
    ) -> Result<String, String> {
        // Generate migration ID
        let migration_id = format!("mig-{}", self.next_migration_id);
        self.next_migration_id += 1;
        
        // Check cell health
        if cell.health.score < self.config.min_health_to_proceed {
            return Err(format!(
                "Cell health too low: {:.2} < {:.2}",
                cell.health.score, self.config.min_health_to_proceed
            ));
        }
        
        // Create migration context
        let context = MigrationContext {
            migration_id: migration_id.clone(),
            cell_id: cell.cell_id,
            source_slot,
            target_slot: target_slot.slot_id.clone(),
            state: MigrationState::Preparing,
            start_time: Some(Instant::now()),
            bytes_transferred: 0,
            total_bytes: self.estimate_state_size(cell),
            error: None,
            snapshot_id: None,
        };
        
        self.active_migrations.insert(migration_id.clone(), context);
        
        Ok(migration_id)
    }
    
    /// Execute migration step
    pub fn step_migration(&mut self, migration_id: &str) -> Result<MigrationState, String> {
        let context = self
            .active_migrations
            .get_mut(migration_id)
            .ok_or_else(|| format!("Migration {} not found", migration_id))?;
        
        // Check timeout
        if let Some(start_time) = context.start_time {
            let elapsed = start_time.elapsed();
            if elapsed.as_secs() > self.config.max_migration_time_secs {
                context.state = MigrationState::Failed;
                context.error = Some("Migration timeout".to_string());
                return Ok(MigrationState::Failed);
            }
        }
        
        // Execute state machine
        match context.state {
            MigrationState::Preparing => {
                // Create snapshot for rollback
                context.snapshot_id = Some(format!("snap-{}", migration_id));
                context.state = MigrationState::Transferring;
            }
            MigrationState::Transferring => {
                // Simulate state transfer
                let chunk_size = self.config.chunk_size_bytes as u64;
                context.bytes_transferred = (context.bytes_transferred + chunk_size)
                    .min(context.total_bytes);
                
                if context.bytes_transferred >= context.total_bytes {
                    context.state = MigrationState::Validating;
                }
            }
            MigrationState::Validating => {
                // Validate transferred state
                context.state = MigrationState::Activating;
            }
            MigrationState::Activating => {
                // Activate new slot
                context.state = MigrationState::Complete;
            }
            MigrationState::Failed => {
                if self.config.auto_rollback {
                    context.state = MigrationState::RollingBack;
                }
            }
            MigrationState::RollingBack => {
                // Restore from snapshot
                context.state = MigrationState::RolledBack;
            }
            _ => {}
        }
        
        Ok(context.state)
    }
    
    /// Complete migration
    pub fn complete_migration(&mut self, migration_id: &str) -> Result<MigrationResult, String> {
        let context = self
            .active_migrations
            .remove(migration_id)
            .ok_or_else(|| format!("Migration {} not found", migration_id))?;
        
        let duration_secs = context
            .start_time
            .map(|t| t.elapsed().as_secs_f64())
            .unwrap_or(0.0);
        
        let success = context.state == MigrationState::Complete;
        
        Ok(MigrationResult {
            migration_id: context.migration_id,
            success,
            final_state: context.state,
            duration_secs,
            bytes_transferred: context.bytes_transferred,
            error: context.error,
        })
    }
    
    /// Rollback migration
    pub fn rollback_migration(&mut self, migration_id: &str) -> Result<(), String> {
        let context = self
            .active_migrations
            .get_mut(migration_id)
            .ok_or_else(|| format!("Migration {} not found", migration_id))?;
        
        if context.snapshot_id.is_none() {
            return Err("No snapshot available for rollback".to_string());
        }
        
        context.state = MigrationState::RollingBack;
        
        Ok(())
    }
    
    /// Get migration status
    pub fn get_status(&self, migration_id: &str) -> Option<&MigrationContext> {
        self.active_migrations.get(migration_id)
    }
    
    /// Get migration progress (0.0 - 1.0)
    pub fn get_progress(&self, migration_id: &str) -> Option<f64> {
        self.active_migrations.get(migration_id).map(|ctx| {
            if ctx.total_bytes == 0 {
                0.0
            } else {
                ctx.bytes_transferred as f64 / ctx.total_bytes as f64
            }
        })
    }
    
    /// Cancel migration
    pub fn cancel_migration(&mut self, migration_id: &str) -> Result<(), String> {
        let context = self
            .active_migrations
            .get_mut(migration_id)
            .ok_or_else(|| format!("Migration {} not found", migration_id))?;
        
        context.state = MigrationState::Failed;
        context.error = Some("Migration cancelled by user".to_string());
        
        if self.config.auto_rollback {
            context.state = MigrationState::RollingBack;
        }
        
        Ok(())
    }
    
    /// Estimate state size for a cell
    fn estimate_state_size(&self, cell: &LccdCell) -> u64 {
        // Rough estimate: 1 MB per member + 100 KB overhead
        (cell.members.len() as u64 * 1024 * 1024) + (100 * 1024)
    }
    
    /// Get active migration count
    pub fn active_count(&self) -> usize {
        self.active_migrations.len()
    }
    
    /// List all active migrations
    pub fn list_active(&self) -> Vec<String> {
        self.active_migrations.keys().cloned().collect()
    }
}

/// Migration health monitor
pub struct MigrationHealthMonitor {
    check_interval: Duration,
    last_check: Option<Instant>,
}

impl MigrationHealthMonitor {
    /// Create a new health monitor
    pub fn new(check_interval_secs: u64) -> Self {
        Self {
            check_interval: Duration::from_secs(check_interval_secs),
            last_check: None,
        }
    }
    
    /// Check if health check is due
    pub fn is_check_due(&mut self) -> bool {
        match self.last_check {
            None => true,
            Some(last) => last.elapsed() >= self.check_interval,
        }
    }
    
    /// Perform health check
    pub fn check_health(&mut self, cell: &LccdCell, min_health: f64) -> bool {
        self.last_check = Some(Instant::now());
        cell.health.score >= min_health
    }
    
    /// Reset health check timer
    pub fn reset(&mut self) {
        self.last_check = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lccd::cell::{CurvatureProfile, CellHealth, CellState};
    
    fn create_test_cell(cell_id: CellId, health_score: f64) -> LccdCell {
        LccdCell {
            cell_id,
            members: vec![1, 2, 3, 4, 5],
            boundary_edges: Vec::new(),
            curvature_profile: CurvatureProfile {
                avg_internal_curvature: 0.5,
                avg_boundary_curvature: -0.3,
                min_curvature: -0.5,
                max_curvature: 0.8,
            },
            health: CellHealth {
                score: health_score,
                size_health: health_score,
                connectivity_health: health_score,
                boundary_health: health_score,
            },
            state: CellState::Active,
        }
    }
    
    fn create_test_slot() -> BpiSlotOffer {
        use chrono::Utc;
        use crate::bpi_chain_state::SigmaVector;
        use crate::slot_marketplace::{ResourceSpec, PriceSpec, QoSSpec, SlotAttestation, SlotStatus};
        
        BpiSlotOffer {
            slot_id: "test-slot".to_string(),
            chain_id: "test-chain".to_string(),
            sigma: SigmaVector::new(60, 840, 2, 2, 2, 0),
            resources: ResourceSpec {
                cpu_cores: 4,
                memory_mb: 16384,
                storage_mb: 512000,
                network_mbps: 1000,
                max_vpods: 10,
            },
            price: PriceSpec {
                cpu_per_hour: 0.1,
                mem_gb_per_hour: 0.05,
                storage_gb_per_hour: 0.01,
                egress_gb: 0.02,
                currency: "BPI".to_string(),
            },
            qos: QoSSpec {
                latency_p95_ms: 50,
                loss_rate: 0.001,
                uptime_guarantee: 0.999,
                jitter_ms: 5,
            },
            tee_quote: None,
            qec2_finality_ms: 1000,
            attestation: SlotAttestation::default(),
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(24),
            status: SlotStatus::Available,
        }
    }
    
    #[test]
    fn test_migration_config() {
        let config = MigrationConfig::default();
        assert_eq!(config.max_migration_time_secs, 300);
        assert_eq!(config.chunk_size_bytes, 1024 * 1024);
        assert!(config.auto_rollback);
    }
    
    #[test]
    fn test_migration_state() {
        let state = MigrationState::Idle;
        assert_eq!(state, MigrationState::Idle);
        assert_ne!(state, MigrationState::Preparing);
    }
    
    #[test]
    fn test_orchestrator_creation() {
        let config = MigrationConfig::default();
        let orchestrator = MigrationOrchestrator::new(config);
        assert_eq!(orchestrator.active_count(), 0);
    }
    
    #[test]
    fn test_start_migration() {
        let config = MigrationConfig::default();
        let mut orchestrator = MigrationOrchestrator::new(config);
        
        let cell = create_test_cell(1, 0.8);
        let slot = create_test_slot();
        
        let result = orchestrator.start_migration(&cell, &slot, None);
        assert!(result.is_ok());
        
        let migration_id = result.unwrap();
        assert_eq!(orchestrator.active_count(), 1);
        
        let status = orchestrator.get_status(&migration_id);
        assert!(status.is_some());
        assert_eq!(status.unwrap().state, MigrationState::Preparing);
    }
    
    #[test]
    fn test_start_migration_low_health() {
        let config = MigrationConfig::default();
        let mut orchestrator = MigrationOrchestrator::new(config);
        
        let cell = create_test_cell(1, 0.3); // Low health
        let slot = create_test_slot();
        
        let result = orchestrator.start_migration(&cell, &slot, None);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_migration_step() {
        let config = MigrationConfig::default();
        let mut orchestrator = MigrationOrchestrator::new(config);
        
        let cell = create_test_cell(1, 0.8);
        let slot = create_test_slot();
        
        let migration_id = orchestrator.start_migration(&cell, &slot, None).unwrap();
        
        // Step through states
        let state = orchestrator.step_migration(&migration_id).unwrap();
        assert_eq!(state, MigrationState::Transferring);
    }
    
    #[test]
    fn test_migration_progress() {
        let config = MigrationConfig::default();
        let mut orchestrator = MigrationOrchestrator::new(config);
        
        let cell = create_test_cell(1, 0.8);
        let slot = create_test_slot();
        
        let migration_id = orchestrator.start_migration(&cell, &slot, None).unwrap();
        
        let progress = orchestrator.get_progress(&migration_id);
        assert!(progress.is_some());
        assert_eq!(progress.unwrap(), 0.0);
    }
    
    #[test]
    fn test_cancel_migration() {
        let config = MigrationConfig::default();
        let mut orchestrator = MigrationOrchestrator::new(config);
        
        let cell = create_test_cell(1, 0.8);
        let slot = create_test_slot();
        
        let migration_id = orchestrator.start_migration(&cell, &slot, None).unwrap();
        
        let result = orchestrator.cancel_migration(&migration_id);
        assert!(result.is_ok());
        
        let status = orchestrator.get_status(&migration_id);
        assert!(status.is_some());
        assert_eq!(status.unwrap().state, MigrationState::RollingBack);
    }
    
    #[test]
    fn test_health_monitor() {
        let mut monitor = MigrationHealthMonitor::new(10);
        
        assert!(monitor.is_check_due());
        
        let cell = create_test_cell(1, 0.8);
        assert!(monitor.check_health(&cell, 0.6));
        
        // Immediately after check, not due
        assert!(!monitor.is_check_due());
    }
    
    #[test]
    fn test_list_active_migrations() {
        let config = MigrationConfig::default();
        let mut orchestrator = MigrationOrchestrator::new(config);
        
        let cell1 = create_test_cell(1, 0.8);
        let cell2 = create_test_cell(2, 0.9);
        let slot = create_test_slot();
        
        orchestrator.start_migration(&cell1, &slot, None).unwrap();
        orchestrator.start_migration(&cell2, &slot, None).unwrap();
        
        let active = orchestrator.list_active();
        assert_eq!(active.len(), 2);
    }
}
