//! Deterministic RNG seeding for reproducible randomness

use crate::error::{DockLockError, DockLockResult};
use rand::SeedableRng;
use rand_core::RngCore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::debug;

/// Deterministic RNG seeder for reproducible execution
#[derive(Debug, Clone)]
pub struct RngSeeder {
    /// Master seed for the execution
    master_seed: [u8; 32],
    /// Per-thread RNG states
    thread_rngs: Arc<Mutex<HashMap<u32, ChaCha20Rng>>>,
    /// Whether seeding is active
    is_active: bool,
}

/// ChaCha20 RNG for deterministic randomness
pub type ChaCha20Rng = rand_chacha::ChaCha20Rng;

/// Seed configuration for deterministic execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedConfig {
    /// Master seed for the execution
    pub master_seed: [u8; 32],
    /// Per-thread seed derivation salt
    pub thread_salt: [u8; 16],
    /// Whether to inject seeds into system RNG calls
    pub inject_system_calls: bool,
}

impl Default for SeedConfig {
    fn default() -> Self {
        Self {
            master_seed: [0u8; 32],
            thread_salt: [0u8; 16],
            inject_system_calls: true,
        }
    }
}

impl RngSeeder {
    /// Create a new RNG seeder with the given master seed
    pub fn new(master_seed: [u8; 32]) -> Self {
        Self {
            master_seed,
            thread_rngs: Arc::new(Mutex::new(HashMap::new())),
            is_active: false,
        }
    }

    /// Create RNG seeder from configuration
    pub fn from_config(config: &SeedConfig) -> Self {
        Self::new(config.master_seed)
    }

    /// Activate deterministic seeding
    pub fn activate(&mut self) -> DockLockResult<()> {
        if self.is_active {
            return Ok(());
        }

        // Initialize main thread RNG
        let main_tid = unsafe { libc::gettid() } as u32;
        self.get_or_create_thread_rng(main_tid)?;

        self.is_active = true;
        debug!("RNG seeder activated with master seed");
        Ok(())
    }

    /// Get or create RNG for a specific thread
    pub fn get_or_create_thread_rng(&self, tid: u32) -> DockLockResult<ChaCha20Rng> {
        let mut thread_rngs = self.thread_rngs.lock().map_err(|e| {
            DockLockError::RngError(format!("Failed to lock thread RNGs: {}", e))
        })?;

        if let Some(rng) = thread_rngs.get(&tid) {
            return Ok(rng.clone());
        }

        // Derive thread-specific seed from master seed and thread ID
        let thread_seed = self.derive_thread_seed(tid);
        let rng = ChaCha20Rng::from_seed(thread_seed);
        
        thread_rngs.insert(tid, rng.clone());
        debug!("Created deterministic RNG for thread {}", tid);
        
        Ok(rng)
    }

    /// Generate deterministic random bytes for a thread
    pub fn generate_bytes(&self, tid: u32, len: usize) -> DockLockResult<Vec<u8>> {
        let mut rng = self.get_or_create_thread_rng(tid)?;
        let mut bytes = vec![0u8; len];
        rng.fill_bytes(&mut bytes);
        
        // Update the stored RNG state
        let mut thread_rngs = self.thread_rngs.lock().map_err(|e| {
            DockLockError::RngError(format!("Failed to lock thread RNGs: {}", e))
        })?;
        thread_rngs.insert(tid, rng);
        
        Ok(bytes)
    }

    /// Generate a deterministic u64 for a thread
    pub fn generate_u64(&self, tid: u32) -> DockLockResult<u64> {
        let mut rng = self.get_or_create_thread_rng(tid)?;
        let value = rng.next_u64();
        
        // Update the stored RNG state
        let mut thread_rngs = self.thread_rngs.lock().map_err(|e| {
            DockLockError::RngError(format!("Failed to lock thread RNGs: {}", e))
        })?;
        thread_rngs.insert(tid, rng);
        
        Ok(value)
    }

    /// Derive thread-specific seed from master seed and thread ID
    fn derive_thread_seed(&self, tid: u32) -> [u8; 32] {
        use blake3::Hasher;
        
        let mut hasher = Hasher::new();
        hasher.update(&self.master_seed);
        hasher.update(&tid.to_le_bytes());
        hasher.update(b"thread_rng_seed");
        
        let hash = hasher.finalize();
        let mut seed = [0u8; 32];
        seed.copy_from_slice(hash.as_bytes());
        seed
    }

    /// Get the master seed
    pub fn master_seed(&self) -> &[u8; 32] {
        &self.master_seed
    }

    /// Check if seeding is active
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Reset all thread RNG states
    pub fn reset(&mut self) -> DockLockResult<()> {
        let mut thread_rngs = self.thread_rngs.lock().map_err(|e| {
            DockLockError::RngError(format!("Failed to lock thread RNGs: {}", e))
        })?;
        thread_rngs.clear();
        debug!("Reset all thread RNG states");
        Ok(())
    }

    /// Get number of active thread RNGs
    pub fn active_thread_count(&self) -> DockLockResult<usize> {
        let thread_rngs = self.thread_rngs.lock().map_err(|e| {
            DockLockError::RngError(format!("Failed to lock thread RNGs: {}", e))
        })?;
        Ok(thread_rngs.len())
    }
}

/// Interceptor for system RNG calls to inject deterministic values
#[derive(Debug)]
pub struct SystemRngInterceptor {
    /// RNG seeder for deterministic values
    seeder: Arc<RngSeeder>,
    /// Whether interception is active
    is_active: bool,
}

impl SystemRngInterceptor {
    /// Create a new system RNG interceptor
    pub fn new(seeder: Arc<RngSeeder>) -> Self {
        Self {
            seeder,
            is_active: false,
        }
    }

    /// Activate system RNG interception
    pub fn activate(&mut self) -> DockLockResult<()> {
        if self.is_active {
            return Ok(());
        }

        // Note: In a real implementation, this would involve:
        // 1. LD_PRELOAD or similar mechanism to intercept libc calls
        // 2. Hooking getrandom(), random(), rand() functions
        // 3. Replacing them with deterministic equivalents
        
        // For now, we just mark as active
        self.is_active = true;
        debug!("System RNG interception activated (placeholder)");
        Ok(())
    }

    /// Handle intercepted getrandom() call
    pub fn handle_getrandom(&self, buf_len: usize) -> DockLockResult<Vec<u8>> {
        if !self.is_active {
            return Err(DockLockError::RngError(
                "System RNG interception not active".to_string(),
            ));
        }

        let tid = unsafe { libc::gettid() } as u32;
        self.seeder.generate_bytes(tid, buf_len)
    }

    /// Handle intercepted random() call
    pub fn handle_random(&self) -> DockLockResult<u64> {
        if !self.is_active {
            return Err(DockLockError::RngError(
                "System RNG interception not active".to_string(),
            ));
        }

        let tid = unsafe { libc::gettid() } as u32;
        self.seeder.generate_u64(tid)
    }

    /// Check if interception is active
    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

/// Utility functions for RNG seeding
pub mod utils {
    use std::env;

    /// Generate a seed from environment variables and system state
    pub fn generate_seed_from_env() -> [u8; 32] {
        use blake3::Hasher;
        
        let mut hasher = Hasher::new();
        
        // Add environment variables that should be deterministic
        for (key, value) in env::vars() {
            if is_deterministic_env_var(&key) {
                hasher.update(key.as_bytes());
                hasher.update(value.as_bytes());
            }
        }
        
        // Add a default seed if no environment variables
        hasher.update(b"default_docklock_seed");
        
        let hash = hasher.finalize();
        let mut seed = [0u8; 32];
        seed.copy_from_slice(hash.as_bytes());
        seed
    }

    /// Check if an environment variable should be included in seed generation
    fn is_deterministic_env_var(key: &str) -> bool {
        // Include variables that should be deterministic across runs
        matches!(key, 
            "DOCKLOCK_SEED" | 
            "DETERMINISTIC_SEED" |
            "EXECUTION_ID" |
            "RUN_ID"
        )
    }

    /// Create a seed from a string
    pub fn seed_from_string(input: &str) -> [u8; 32] {
        use blake3::Hasher;
        
        let mut hasher = Hasher::new();
        hasher.update(input.as_bytes());
        
        let hash = hasher.finalize();
        let mut seed = [0u8; 32];
        seed.copy_from_slice(hash.as_bytes());
        seed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng_seeder_creation() {
        let seed = [42u8; 32];
        let seeder = RngSeeder::new(seed);
        assert_eq!(seeder.master_seed(), &seed);
        assert!(!seeder.is_active());
    }

    #[test]
    fn test_thread_rng_generation() {
        let seed = [42u8; 32];
        let seeder = RngSeeder::new(seed);
        
        let rng1 = seeder.get_or_create_thread_rng(1);
        assert!(rng1.is_ok());
        
        let rng2 = seeder.get_or_create_thread_rng(1);
        assert!(rng2.is_ok());
        
        // Different threads should have different RNGs
        let rng3 = seeder.get_or_create_thread_rng(2);
        assert!(rng3.is_ok());
    }

    #[test]
    fn test_deterministic_bytes_generation() {
        let seed = [42u8; 32];
        let seeder = RngSeeder::new(seed);
        
        let bytes1 = seeder.generate_bytes(1, 32);
        assert!(bytes1.is_ok());
        
        let bytes2 = seeder.generate_bytes(1, 32);
        assert!(bytes2.is_ok());
        
        // Should be different because RNG state advances
        assert_ne!(bytes1.unwrap(), bytes2.unwrap());
    }

    #[test]
    fn test_seed_from_string() {
        let seed1 = utils::seed_from_string("test");
        let seed2 = utils::seed_from_string("test");
        let seed3 = utils::seed_from_string("different");
        
        assert_eq!(seed1, seed2);
        assert_ne!(seed1, seed3);
    }
}
