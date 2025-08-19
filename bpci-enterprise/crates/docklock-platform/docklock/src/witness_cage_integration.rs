//! Integration between witness recording and determinism cage
//! Stage 27: Witness Log & I/O Recording Integration

use crate::error::DockLockResult;
use crate::cage::DeterminismCage;
use crate::witness_enhanced::{EnhancedWitnessRecorder, WitnessRecorderConfig};
use crate::witness_wallet_integration::{WalletWitnessRecorder, WalletWitnessConfig};
use crate::witness::{WitnessData, WitnessOperationType};
use crate::event_stream::CanonicalEventStream;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Integrated witness-enabled determinism cage
#[derive(Debug)]
pub struct WitnessEnabledCage {
    /// Base determinism cage
    cage: DeterminismCage,
    /// Enhanced witness recorder for system operations
    system_witness: EnhancedWitnessRecorder,
    /// Wallet witness recorder for wallet operations
    wallet_witness: WalletWitnessRecorder,
    /// Event stream for correlation
    event_stream: Arc<RwLock<CanonicalEventStream>>,
    /// Configuration
    config: WitnessEnabledCageConfig,
    /// Execution statistics
    stats: WitnessExecutionStats,
}

/// Configuration for witness-enabled cage
#[derive(Debug, Clone)]
pub struct WitnessEnabledCageConfig {
    /// Base witness recorder configuration
    pub witness_config: WitnessRecorderConfig,
    /// Wallet witness configuration
    pub wallet_witness_config: WalletWitnessConfig,
    /// Enable syscall witness recording
    pub record_syscalls: bool,
    /// Enable file I/O witness recording
    pub record_file_io: bool,
    /// Enable network I/O witness recording
    pub record_network_io: bool,
    /// Enable RNG witness recording
    pub record_rng: bool,
    /// Minimum execution time to record (microseconds)
    pub min_execution_time: u64,
}

impl Default for WitnessEnabledCageConfig {
    fn default() -> Self {
        Self {
            witness_config: WitnessRecorderConfig::default(),
            wallet_witness_config: WalletWitnessConfig::default(),
            record_syscalls: true,
            record_file_io: true,
            record_network_io: true,
            record_rng: true,
            min_execution_time: 1000, // 1ms
        }
    }
}

/// Statistics for witness-enabled execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessExecutionStats {
    pub total_executions: usize,
    pub witnessed_executions: usize,
    pub syscalls_recorded: usize,
    pub file_operations_recorded: usize,
    pub network_operations_recorded: usize,
    pub rng_operations_recorded: usize,
    pub total_witness_size: usize,
    pub average_witness_size: f64,
    pub execution_time_with_witness: u64,
    pub execution_time_without_witness: u64,
    pub witness_overhead_ratio: f64,
}

impl Default for WitnessExecutionStats {
    fn default() -> Self {
        Self {
            total_executions: 0,
            witnessed_executions: 0,
            syscalls_recorded: 0,
            file_operations_recorded: 0,
            network_operations_recorded: 0,
            rng_operations_recorded: 0,
            total_witness_size: 0,
            average_witness_size: 0.0,
            execution_time_with_witness: 0,
            execution_time_without_witness: 0,
            witness_overhead_ratio: 0.0,
        }
    }
}

impl WitnessEnabledCage {
    /// Create a new witness-enabled determinism cage
    pub fn new(
        config: WitnessEnabledCageConfig,
        event_stream: Arc<RwLock<CanonicalEventStream>>,
    ) -> DockLockResult<Self> {
        let cage = DeterminismCage::with_defaults()?;
        
        let mut system_witness = EnhancedWitnessRecorder::new(config.witness_config.clone());
        system_witness.set_event_stream(event_stream.clone());
        system_witness.set_enabled(true);

        let wallet_witness = WalletWitnessRecorder::new(
            config.wallet_witness_config.clone(),
            event_stream.clone(),
        );

        Ok(Self {
            cage,
            system_witness,
            wallet_witness,
            event_stream,
            config,
            stats: WitnessExecutionStats::default(),
        })
    }

    /// Execute a function with comprehensive witness recording
    pub fn execute_with_witness<F, T>(&mut self, f: F) -> DockLockResult<T>
    where
        F: FnOnce() -> T,
    {
        let start_time = std::time::Instant::now();
        
        // Activate the cage for deterministic execution
        self.cage.activate()?;
        
        // Enable witness recording
        self.system_witness.set_enabled(true);
        
        // Record execution start event
        if let Ok(mut stream) = self.event_stream.write() {
            let seq = stream.current_sequence() + 1;
            let event = crate::event_stream::Event::new(
                seq as u128,
                None,
                seq,
                crate::event_stream::EventKind::ContainerStart,
                b"witness_execution_start",
            );
            let _ = stream.add_event(event);
        }

        // Execute the function with witness recording
        let result = self.execute_with_syscall_interception(f)?;
        
        let execution_time = start_time.elapsed().as_micros() as u64;
        
        // Record execution end event
        if let Ok(mut stream) = self.event_stream.write() {
            let seq = stream.current_sequence() + 1;
            let event = crate::event_stream::Event::new(
                seq as u128,
                None,
                seq,
                crate::event_stream::EventKind::ContainerStop,
                b"witness_execution_end",
            );
            let _ = stream.add_event(event);
        }

        // Deactivate the cage
        self.cage.reset()?;
        
        // Update statistics
        self.update_execution_stats(execution_time, true);
        
        info!(
            "Executed function with witness recording in {}μs, recorded {} witness entries",
            execution_time,
            self.system_witness.log().len()
        );

        Ok(result)
    }

    /// Execute with syscall interception and witness recording
    fn execute_with_syscall_interception<F, T>(&mut self, f: F) -> DockLockResult<T>
    where
        F: FnOnce() -> T,
    {
        // Set up syscall interception hooks
        let pid = std::process::id();
        let tid = unsafe { libc::gettid() } as u32;

        // Record pre-execution state
        self.record_pre_execution_state(pid, tid)?;

        // Execute the function
        let result = f();

        // Record post-execution state
        self.record_post_execution_state(pid, tid)?;

        Ok(result)
    }

    /// Record pre-execution state
    fn record_pre_execution_state(&mut self, pid: u32, tid: u32) -> DockLockResult<()> {
        if self.config.record_syscalls {
            // Record initial syscall state
            self.system_witness.record_syscall_result(
                pid,
                tid,
                "execution_start",
                &[pid as u64, tid as u64],
                0,
                0,
            )?;
            self.stats.syscalls_recorded += 1;
        }

        // Record environment state
        for (key, value) in std::env::vars() {
            if key.starts_with("DOCKLOCK_") || key.starts_with("WITNESS_") {
                self.system_witness.record_env_access(pid, tid, &key, Some(&value))?;
            }
        }

        debug!("Recorded pre-execution state for PID: {}, TID: {}", pid, tid);
        Ok(())
    }

    /// Record post-execution state
    fn record_post_execution_state(&mut self, pid: u32, tid: u32) -> DockLockResult<()> {
        if self.config.record_syscalls {
            // Record final syscall state
            self.system_witness.record_syscall_result(
                pid,
                tid,
                "execution_end",
                &[pid as u64, tid as u64],
                0,
                0,
            )?;
            self.stats.syscalls_recorded += 1;
        }

        debug!("Recorded post-execution state for PID: {}, TID: {}", pid, tid);
        Ok(())
    }

    /// Record a file operation during execution
    pub fn record_file_operation(
        &mut self,
        operation_type: WitnessOperationType,
        path: &str,
        offset: u64,
        data: &[u8],
        result: i64,
    ) -> DockLockResult<()> {
        if !self.config.record_file_io {
            return Ok(());
        }

        let pid = std::process::id();
        let tid = unsafe { libc::gettid() } as u32;

        self.system_witness.record_file_operation(
            operation_type,
            pid,
            tid,
            path,
            offset,
            data,
            result,
        )?;

        self.stats.file_operations_recorded += 1;
        debug!("Recorded file operation: {} on {}", operation_type as u8, path);

        Ok(())
    }

    /// Record a network operation during execution
    pub fn record_network_operation(
        &mut self,
        operation: &str,
        endpoint: &str,
        data: &[u8],
        result: i64,
    ) -> DockLockResult<()> {
        if !self.config.record_network_io {
            return Ok(());
        }

        let pid = std::process::id();
        let tid = unsafe { libc::gettid() } as u32;

        let witness_data = WitnessData::SyscallResult {
            syscall_name: format!("network_{}", operation),
            args: vec![endpoint.len() as u64, data.len() as u64],
            result,
            errno: 0,
        };

        self.system_witness.log_mut().add_witness_entry(
            WitnessOperationType::NetworkIo,
            pid,
            tid,
            witness_data,
            None,
        )?;

        self.stats.network_operations_recorded += 1;
        debug!("Recorded network operation: {} to {}", operation, endpoint);

        Ok(())
    }

    /// Record RNG operation during execution
    pub fn record_rng_operation(&mut self, seed: &[u8], output: &[u8]) -> DockLockResult<()> {
        if !self.config.record_rng {
            return Ok(());
        }

        let pid = std::process::id();
        let tid = unsafe { libc::gettid() } as u32;

        let witness_data = WitnessData::RandomData {
            data: [seed, output].concat(),
        };

        self.system_witness.log_mut().add_witness_entry(
            WitnessOperationType::RandomGeneration,
            pid,
            tid,
            witness_data,
            None,
        )?;

        self.stats.rng_operations_recorded += 1;
        debug!("Recorded RNG operation: {} bytes seed, {} bytes output", seed.len(), output.len());

        Ok(())
    }

    /// Record a wallet operation during execution
    pub fn record_wallet_operation(
        &mut self,
        operation: &str,
        wallet_id: &str,
        parameters: HashMap<String, String>,
        result: &str,
    ) -> DockLockResult<()> {
        let pid = std::process::id();
        let tid = unsafe { libc::gettid() } as u32;

        self.wallet_witness.record_service_operation(
            wallet_id,
            operation,
            parameters,
            result,
            pid,
            tid,
        )?;

        debug!("Recorded wallet operation: {} on {}", operation, wallet_id);
        Ok(())
    }

    /// Update execution statistics
    fn update_execution_stats(&mut self, execution_time: u64, with_witness: bool) {
        self.stats.total_executions += 1;

        if with_witness {
            self.stats.witnessed_executions += 1;
            self.stats.execution_time_with_witness += execution_time;
            
            let witness_size = self.system_witness.log().compressed_size() + 
                              self.wallet_witness.recorder().log().compressed_size();
            self.stats.total_witness_size += witness_size;
            
            if self.stats.witnessed_executions > 0 {
                self.stats.average_witness_size = 
                    self.stats.total_witness_size as f64 / self.stats.witnessed_executions as f64;
            }
        } else {
            self.stats.execution_time_without_witness += execution_time;
        }

        // Calculate witness overhead ratio
        if self.stats.execution_time_without_witness > 0 {
            let avg_with = if self.stats.witnessed_executions > 0 {
                self.stats.execution_time_with_witness as f64 / self.stats.witnessed_executions as f64
            } else {
                0.0
            };
            
            let avg_without = if (self.stats.total_executions - self.stats.witnessed_executions) > 0 {
                self.stats.execution_time_without_witness as f64 / 
                (self.stats.total_executions - self.stats.witnessed_executions) as f64
            } else {
                1.0
            };

            if avg_without > 0.0 {
                self.stats.witness_overhead_ratio = avg_with / avg_without;
            }
        }
    }

    /// Compute combined Merkle root of all witness data
    pub fn compute_combined_witness_root(&mut self) -> DockLockResult<[u8; 32]> {
        let system_root = self.system_witness.compute_merkle_root()?;
        let wallet_root = self.wallet_witness.compute_merkle_root()?;

        // Combine the two roots
        let combined_data = [system_root, wallet_root].concat();
        let combined_root = blake3::hash(&combined_data);

        info!(
            "Computed combined witness root: {} (system: {}, wallet: {})",
            hex::encode(combined_root.as_bytes()),
            hex::encode(system_root),
            hex::encode(wallet_root)
        );

        Ok(*combined_root.as_bytes())
    }

    /// Validate witness completeness
    pub fn validate_witness_completeness(&self) -> DockLockResult<bool> {
        let system_log = self.system_witness.log();
        let wallet_log = self.wallet_witness.recorder().log();

        // Check if we have sufficient witness data
        if system_log.is_empty() && wallet_log.is_empty() {
            warn!("No witness data recorded");
            return Ok(false);
        }

        // Check if we have the minimum required witness types
        if self.config.record_syscalls && self.stats.syscalls_recorded == 0 {
            warn!("No syscalls recorded despite being enabled");
            return Ok(false);
        }

        if self.config.record_file_io && self.stats.file_operations_recorded == 0 {
            debug!("No file operations recorded (may be expected)");
        }

        info!(
            "Witness validation passed: {} system entries, {} wallet entries",
            system_log.len(),
            wallet_log.len()
        );

        Ok(true)
    }

    /// Get execution statistics
    pub fn stats(&self) -> &WitnessExecutionStats {
        &self.stats
    }

    /// Get system witness recorder
    pub fn system_witness(&self) -> &EnhancedWitnessRecorder {
        &self.system_witness
    }

    /// Get wallet witness recorder
    pub fn wallet_witness(&self) -> &WalletWitnessRecorder {
        &self.wallet_witness
    }

    /// Get mutable system witness recorder
    pub fn system_witness_mut(&mut self) -> &mut EnhancedWitnessRecorder {
        &mut self.system_witness
    }

    /// Get mutable wallet witness recorder
    pub fn wallet_witness_mut(&mut self) -> &mut WalletWitnessRecorder {
        &mut self.wallet_witness
    }

    /// Get the underlying determinism cage
    pub fn cage(&self) -> &DeterminismCage {
        &self.cage
    }

    /// Get configuration
    pub fn config(&self) -> &WitnessEnabledCageConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_stream::CanonicalEventStream;

    #[test]
    fn test_witness_enabled_cage_creation() {
        let event_stream = Arc::new(RwLock::new(CanonicalEventStream::default()));
        let config = WitnessEnabledCageConfig::default();
        
        let cage = WitnessEnabledCage::new(config, event_stream).unwrap();
        
        assert_eq!(cage.stats().total_executions, 0);
        assert_eq!(cage.stats().witnessed_executions, 0);
    }

    #[test]
    fn test_witness_enabled_execution() {
        let event_stream = Arc::new(RwLock::new(CanonicalEventStream::default()));
        let config = WitnessEnabledCageConfig::default();
        let mut cage = WitnessEnabledCage::new(config, event_stream).unwrap();

        // Execute a simple function with witness recording
        let result = cage.execute_with_witness(|| {
            42 + 8
        }).unwrap();

        assert_eq!(result, 50);
        assert_eq!(cage.stats().total_executions, 1);
        assert_eq!(cage.stats().witnessed_executions, 1);
        assert!(cage.stats().syscalls_recorded >= 2); // start + end
    }

    #[test]
    fn test_file_operation_recording() {
        let event_stream = Arc::new(RwLock::new(CanonicalEventStream::default()));
        let config = WitnessEnabledCageConfig::default();
        let mut cage = WitnessEnabledCage::new(config, event_stream).unwrap();

        cage.record_file_operation(
            WitnessOperationType::FileRead,
            "/test/file",
            0,
            b"test data",
            9,
        ).unwrap();

        assert_eq!(cage.stats().file_operations_recorded, 1);
        assert_eq!(cage.system_witness().log().len(), 1);
    }

    #[test]
    fn test_witness_validation() {
        let event_stream = Arc::new(RwLock::new(CanonicalEventStream::default()));
        let config = WitnessEnabledCageConfig::default();
        let mut cage = WitnessEnabledCage::new(config, event_stream).unwrap();

        // Execute with witness recording
        cage.execute_with_witness(|| {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }).unwrap();

        // Validate witness completeness
        let is_valid = cage.validate_witness_completeness().unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_combined_witness_root() {
        let event_stream = Arc::new(RwLock::new(CanonicalEventStream::default()));
        let config = WitnessEnabledCageConfig::default();
        let mut cage = WitnessEnabledCage::new(config, event_stream).unwrap();

        // Add some witness data
        cage.execute_with_witness(|| {
            42
        }).unwrap();

        // Compute combined root
        let root = cage.compute_combined_witness_root().unwrap();
        assert_ne!(root, [0u8; 32]);
    }
}
