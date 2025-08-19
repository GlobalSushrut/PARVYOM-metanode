//! Main determinism cage implementation

use crate::error::{DockLockError, DockLockResult};
use crate::filter::SyscallFilter;
use crate::seeder::{RngSeeder, SystemRngInterceptor};
use crate::witness::{WitnessRecorder, WitnessOperationType};
use crate::{CageConfig, ExecutionResult};

use std::process::Stdio;
use std::sync::Arc;
use std::time::Instant;
use tokio::process::Command as AsyncCommand;
use tracing::{debug, info, warn};

/// Main determinism cage for executing processes with deterministic guarantees
#[derive(Debug)]
pub struct DeterminismCage {
    /// Configuration for the cage
    config: CageConfig,
    /// Syscall filter for blocking non-deterministic operations
    syscall_filter: Option<SyscallFilter>,
    /// RNG seeder for deterministic randomness
    rng_seeder: RngSeeder,
    /// System RNG interceptor
    rng_interceptor: SystemRngInterceptor,
    /// Witness recorder for I/O operations
    witness_recorder: WitnessRecorder,
    /// Whether the cage is active
    is_active: bool,
}

impl DeterminismCage {
    /// Create a new determinism cage with the given configuration
    pub fn new(config: CageConfig) -> DockLockResult<Self> {
        // Create syscall filter if enabled
        let syscall_filter = if config.enable_seccomp {
            Some(SyscallFilter::new(config.allowed_syscalls.clone())?)
        } else {
            None
        };

        // Create RNG seeder
        let rng_seeder = RngSeeder::new(config.rng_seed);
        let rng_interceptor = SystemRngInterceptor::new(Arc::new(rng_seeder.clone()));

        // Create witness recorder
        let witness_recorder = WitnessRecorder::new(config.max_witness_size);

        Ok(Self {
            config,
            syscall_filter,
            rng_seeder,
            rng_interceptor,
            witness_recorder,
            is_active: false,
        })
    }

    /// Create a cage with default configuration
    pub fn with_defaults() -> DockLockResult<Self> {
        Self::new(CageConfig::default())
    }

    /// Activate the determinism cage
    pub fn activate(&mut self) -> DockLockResult<()> {
        if self.is_active {
            return Ok(());
        }

        info!("Activating determinism cage");

        // Activate syscall filter
        if let Some(ref mut filter) = self.syscall_filter {
            filter.activate()?;
            debug!("Syscall filter activated");
        }

        // Activate RNG seeder
        self.rng_seeder.activate()?;
        debug!("RNG seeder activated");

        self.is_active = true;
        info!("Determinism cage activated successfully");
        Ok(())
    }

    /// Execute a command within the determinism cage
    pub async fn execute_command(
        &mut self,
        program: &str,
        args: &[&str],
        working_dir: Option<&str>,
    ) -> DockLockResult<ExecutionResult> {
        if !self.is_active {
            return Err(DockLockError::ExecutionFailed(
                "Cage not activated".to_string(),
            ));
        }

        info!("Executing command: {} {:?}", program, args);
        let start_time = Instant::now();

        // Set up the command
        let mut cmd = AsyncCommand::new(program);
        cmd.args(args);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        // Set working directory if specified
        if let Some(dir) = working_dir {
            cmd.current_dir(dir);
        }

        // Set environment variables
        for (key, value) in &self.config.env_vars {
            cmd.env(key, value);
        }

        // Set deterministic environment variables
        cmd.env("DOCKLOCK_DETERMINISTIC", "1");
        cmd.env("DOCKLOCK_SEED", hex::encode(self.config.rng_seed));

        // Execute the command
        let child = cmd.spawn().map_err(|e| {
            DockLockError::ExecutionFailed(format!("Failed to spawn process: {}", e))
        })?;

        // Record the process start
        let pid = child.id().unwrap_or(0);
        self.witness_recorder.record_syscall_result(
            pid,
            pid, // Use PID as TID for main thread
            "execve",
            &[],
            0, // Success
            0, // No errno
        )?;

        // Wait for completion
        let output = child.wait_with_output().await.map_err(|e| {
            DockLockError::ExecutionFailed(format!("Process execution failed: {}", e))
        })?;

        let duration = start_time.elapsed();
        let exit_code = output.status.code().unwrap_or(-1);

        info!(
            "Command completed with exit code {} in {:?}",
            exit_code, duration
        );

        // Finalize witness log
        let mut witness_log = self.witness_recorder.log().clone();
        witness_log.compute_merkle_root()?;

        // Check determinism (simplified check)
        let is_deterministic = self.check_determinism(&output.stdout, &output.stderr)?;

        Ok(ExecutionResult {
            exit_code,
            stdout: output.stdout,
            stderr: output.stderr,
            witness_log,
            duration_ns: duration.as_nanos() as u64,
            is_deterministic,
        })
    }

    /// Execute a function within the determinism cage
    pub async fn execute_function<F, T>(&mut self, func: F) -> DockLockResult<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        if !self.is_active {
            return Err(DockLockError::ExecutionFailed(
                "Cage not activated".to_string(),
            ));
        }

        info!("Executing function within determinism cage");

        // Record function execution start
        let tid = unsafe { libc::gettid() } as u32;
        self.witness_recorder.record_syscall_result(
            std::process::id(),
            tid,
            "function_call",
            &[],
            0,
            0,
        )?;

        // Execute the function
        let result = tokio::task::spawn_blocking(func).await.map_err(|e| {
            DockLockError::ExecutionFailed(format!("Function execution failed: {}", e))
        })?;

        info!("Function execution completed");
        Ok(result)
    }

    /// Check if execution appears deterministic
    fn check_determinism(&self, stdout: &[u8], stderr: &[u8]) -> DockLockResult<bool> {
        // Simple heuristics for determinism checking
        let mut is_deterministic = true;

        // Check for non-deterministic patterns in output
        let output_str = String::from_utf8_lossy(stdout);
        let stderr_str = String::from_utf8_lossy(stderr);

        let non_deterministic_patterns = [
            "timestamp",
            "time:",
            "random",
            "uuid",
            "pid:",
            "thread",
        ];

        for pattern in &non_deterministic_patterns {
            if output_str.contains(pattern) || stderr_str.contains(pattern) {
                warn!("Potentially non-deterministic pattern found: {}", pattern);
                is_deterministic = false;
            }
        }

        // Check witness log for non-deterministic operations
        let witness_log = self.witness_recorder.log();
        for entry in &witness_log.entries {
            match entry.operation_type {
                WitnessOperationType::RandomGeneration => {
                    debug!("Random generation detected in witness log");
                    // This is OK if we're using deterministic seeding
                }
                WitnessOperationType::NetworkIo => {
                    warn!("Network I/O detected - potentially non-deterministic");
                    is_deterministic = false;
                }
                _ => {}
            }
        }

        Ok(is_deterministic)
    }

    /// Get execution statistics
    pub fn get_stats(&self) -> DockLockResult<CageStats> {
        let witness_log = self.witness_recorder.log();
        let thread_count = self.rng_seeder.active_thread_count()?;

        Ok(CageStats {
            is_active: self.is_active,
            syscall_filter_active: self
                .syscall_filter
                .as_ref()
                .map(|f| f.is_active())
                .unwrap_or(false),
            rng_seeder_active: self.rng_seeder.is_active(),
            witness_entries: witness_log.len(),
            witness_size_bytes: witness_log.size(),
            active_threads: thread_count,
            config: self.config.clone(),
        })
    }

    /// Reset the cage state
    pub fn reset(&mut self) -> DockLockResult<()> {
        info!("Resetting determinism cage");

        // Reset RNG seeder
        self.rng_seeder.reset()?;

        // Clear witness log
        self.witness_recorder.log_mut().clear();

        debug!("Cage state reset successfully");
        Ok(())
    }

    /// Get the configuration
    pub fn config(&self) -> &CageConfig {
        &self.config
    }

    /// Get the witness recorder
    pub fn witness_recorder(&self) -> &WitnessRecorder {
        &self.witness_recorder
    }

    /// Get mutable access to the witness recorder
    pub fn witness_recorder_mut(&mut self) -> &mut WitnessRecorder {
        &mut self.witness_recorder
    }

    /// Check if the cage is active
    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

/// Statistics about cage execution
#[derive(Debug, Clone)]
pub struct CageStats {
    /// Whether the cage is active
    pub is_active: bool,
    /// Whether syscall filter is active
    pub syscall_filter_active: bool,
    /// Whether RNG seeder is active
    pub rng_seeder_active: bool,
    /// Number of witness entries
    pub witness_entries: usize,
    /// Size of witness log in bytes
    pub witness_size_bytes: usize,
    /// Number of active threads
    pub active_threads: usize,
    /// Cage configuration
    pub config: CageConfig,
}

/// Builder for creating determinism cages with custom configuration
#[derive(Debug, Default)]
pub struct CageBuilder {
    config: CageConfig,
}

impl CageBuilder {
    /// Create a new cage builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set whether to enable seccomp filtering
    pub fn enable_seccomp(mut self, enable: bool) -> Self {
        self.config.enable_seccomp = enable;
        self
    }

    /// Set whether to enable witness recording
    pub fn enable_witness_recording(mut self, enable: bool) -> Self {
        self.config.enable_witness_recording = enable;
        self
    }

    /// Set the maximum witness log size
    pub fn max_witness_size(mut self, size: usize) -> Self {
        self.config.max_witness_size = size;
        self
    }

    /// Set the RNG seed
    pub fn rng_seed(mut self, seed: [u8; 32]) -> Self {
        self.config.rng_seed = seed;
        self
    }

    /// Add an allowed syscall
    pub fn allow_syscall(mut self, syscall: impl Into<String>) -> Self {
        self.config.allowed_syscalls.push(syscall.into());
        self
    }

    /// Set environment variable
    pub fn env_var(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.config.env_vars.insert(key.into(), value.into());
        self
    }

    /// Build the determinism cage
    pub fn build(self) -> DockLockResult<DeterminismCage> {
        DeterminismCage::new(self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[tokio::test]
    async fn test_cage_creation() {
        let cage = DeterminismCage::with_defaults();
        assert!(cage.is_ok());

        let cage = cage.unwrap();
        assert!(!cage.is_active());
    }

    #[tokio::test]
    async fn test_cage_builder() {
        let cage = CageBuilder::new()
            .enable_seccomp(false) // Disable for testing
            .rng_seed([42u8; 32])
            .allow_syscall("test")
            .env_var("TEST_VAR", "test_value")
            .build();

        assert!(cage.is_ok());
        let cage = cage.unwrap();
        assert_eq!(cage.config().rng_seed, [42u8; 32]);
        assert!(!cage.config().enable_seccomp);
    }

    #[tokio::test]
    async fn test_cage_activation() {
        let mut cage = CageBuilder::new()
            .enable_seccomp(false) // Disable seccomp for testing
            .build()
            .unwrap();

        let result = cage.activate();
        assert!(result.is_ok());
        assert!(cage.is_active());
    }

    #[tokio::test]
    async fn test_simple_command_execution() {
        let mut cage = CageBuilder::new()
            .enable_seccomp(false) // Disable seccomp for testing
            .build()
            .unwrap();

        cage.activate().unwrap();

        // Test simple echo command
        let result = cage.execute_command("echo", &["hello", "world"], None).await;
        assert!(result.is_ok());

        let execution_result = result.unwrap();
        assert_eq!(execution_result.exit_code, 0);
        assert_eq!(execution_result.stdout, b"hello world\n");
    }

    #[tokio::test]
    async fn test_function_execution() {
        let mut cage = CageBuilder::new()
            .enable_seccomp(false) // Disable seccomp for testing
            .build()
            .unwrap();

        cage.activate().unwrap();

        let result = cage.execute_function(|| {
            42 + 8
        }).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 50);
    }

    #[test]
    fn test_cage_stats() {
        let cage = CageBuilder::new()
            .enable_seccomp(false)
            .build()
            .unwrap();

        let stats = cage.get_stats();
        assert!(stats.is_ok());

        let stats = stats.unwrap();
        assert!(!stats.is_active);
        assert!(!stats.syscall_filter_active);
        assert_eq!(stats.witness_entries, 0);
    }
}
