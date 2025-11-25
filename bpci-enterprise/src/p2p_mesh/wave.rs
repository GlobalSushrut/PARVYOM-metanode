//! Wave Token and Routing
//! 
//! Implements Factorial-Wave Mesh addressing and routing.
//! 
//! # Purpose
//! 
//! Compact wave addressing for deterministic P2P routing:
//! - Wave tokens: 256-bit compact representation
//! - PRF expansion: Token → permutation (deterministic)
//! - Single domain: Up to 64 nodes per domain
//! - Portalized: Multi-domain scaling with portal nodes
//! 
//! # Wave Space
//! 
//! ```text
//! Ω_n = S_n ∪ {⊥}
//! |Ω_n| = n! + 1
//! 
//! where:
//! - S_n = symmetric group (all permutations)
//! - ⊥ = base/null wave (broadcast/control)
//! ```

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

use super::factoradic::{Permutation, prf_to_permutation};

/// Wave token (compact 256-bit representation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WaveToken {
    /// PRF seed (256 bits)
    pub token: [u8; 32],
    
    /// Epoch number
    pub epoch: u64,
    
    /// Control flags
    pub flags: u8,
}

impl WaveToken {
    /// Create a new wave token
    pub fn new(token: [u8; 32], epoch: u64) -> Self {
        Self {
            token,
            epoch,
            flags: 0,
        }
    }
    
    /// Create base/null wave (broadcast/control)
    pub fn base_wave(epoch: u64) -> Self {
        Self {
            token: [0u8; 32],
            epoch,
            flags: 0x01, // Flag 0x01 = base wave
        }
    }
    
    /// Check if this is a base wave
    pub fn is_base_wave(&self) -> bool {
        self.flags & 0x01 != 0
    }
    
    /// Expand token to permutation using epoch seed
    pub fn expand(&self, epoch_seed: &[u8; 32], domain_size: usize) -> Permutation {
        if self.is_base_wave() {
            // Base wave = identity permutation
            (0..domain_size).collect()
        } else {
            // Combine epoch seed and token for PRF
            let prf_input = self.prf_input(epoch_seed);
            prf_to_permutation(&prf_input, domain_size)
        }
    }
    
    /// Generate PRF input from epoch seed and token
    fn prf_input(&self, epoch_seed: &[u8; 32]) -> [u8; 32] {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(b"WAVE_PRF_V1");
        hasher.update(epoch_seed);
        hasher.update(&self.token);
        hasher.update(self.epoch.to_le_bytes());
        hasher.finalize().into()
    }
    
    /// Generate random wave token from seed
    pub fn random(epoch: u64, seed: &[u8]) -> Self {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(b"WAVE_TOKEN_V1");
        hasher.update(seed);
        hasher.update(epoch.to_le_bytes());
        let token = hasher.finalize().into();
        
        Self::new(token, epoch)
    }
}

/// Portal wave (for multi-domain routing)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalWave {
    /// Cluster permutation (σ ∈ S_g)
    pub cluster_order: Permutation,
    
    /// Intra-cluster permutations (πᵢ ∈ S_s for each cluster)
    pub cluster_perms: Vec<Permutation>,
    
    /// Epoch
    pub epoch: u64,
}

impl PortalWave {
    /// Expand wave token to portal wave
    pub fn expand(
        token: &WaveToken,
        epoch_seed: &[u8; 32],
        cluster_count: usize,
        cluster_size: usize,
    ) -> Self {
        let prf_input = token.prf_input(epoch_seed);
        
        // Expand cluster order (σ ∈ S_g)
        let cluster_order = prf_to_permutation(&prf_input, cluster_count);
        
        // Expand intra-cluster permutations (πᵢ ∈ S_s)
        let mut cluster_perms = Vec::with_capacity(cluster_count);
        for i in 0..cluster_count {
            let mut hasher = <Sha256 as Digest>::new();
            hasher.update(&prf_input);
            hasher.update(b"CLUSTER_PERM");
            hasher.update(i.to_le_bytes());
            let cluster_prf: [u8; 32] = hasher.finalize().into();
            
            let perm = prf_to_permutation(&cluster_prf, cluster_size);
            cluster_perms.push(perm);
        }
        
        Self {
            cluster_order,
            cluster_perms,
            epoch: token.epoch,
        }
    }
    
    /// Get next hop for routing
    pub fn next_hop(
        &self,
        current_cluster: usize,
        target_cluster: usize,
        position_in_cluster: usize,
    ) -> (usize, usize) {
        if current_cluster == target_cluster {
            // Intra-cluster routing: follow πᵢ
            let perm = &self.cluster_perms[current_cluster];
            let next_pos = perm[position_in_cluster];
            (current_cluster, next_pos)
        } else {
            // Inter-cluster routing: follow σ to next cluster
            let current_idx = self.cluster_order
                .iter()
                .position(|&c| c == current_cluster)
                .unwrap_or(0);
            
            let next_cluster = self.cluster_order[(current_idx + 1) % self.cluster_order.len()];
            (next_cluster, 0) // Enter next cluster at position 0
        }
    }
}

/// Wave scheduler (selects active waves per epoch)
pub struct WaveScheduler {
    /// Waves per epoch
    waves_per_epoch: usize,
}

impl WaveScheduler {
    /// Create a new wave scheduler
    pub fn new(waves_per_epoch: usize) -> Self {
        Self { waves_per_epoch }
    }
    
    /// Select active waves for an epoch
    pub fn select_waves(&self, epoch: u64, epoch_seed: &[u8; 32]) -> Vec<WaveToken> {
        let mut waves = Vec::with_capacity(self.waves_per_epoch);
        
        // Always include base wave
        waves.push(WaveToken::base_wave(epoch));
        
        // Generate additional waves
        for i in 0..(self.waves_per_epoch - 1) {
            let mut hasher = <Sha256 as Digest>::new();
            hasher.update(b"WAVE_SELECTION_V1");
            hasher.update(epoch_seed);
            hasher.update(epoch.to_le_bytes());
            hasher.update(i.to_le_bytes());
            let seed = hasher.finalize();
            
            waves.push(WaveToken::random(epoch, &seed));
        }
        
        waves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wave_token_creation() {
        let token = [42u8; 32];
        let wave = WaveToken::new(token, 1);
        
        assert_eq!(wave.token, token);
        assert_eq!(wave.epoch, 1);
        assert!(!wave.is_base_wave());
    }
    
    #[test]
    fn test_base_wave() {
        let wave = WaveToken::base_wave(1);
        
        assert!(wave.is_base_wave());
        assert_eq!(wave.token, [0u8; 32]);
    }
    
    #[test]
    fn test_wave_expansion() {
        let token = [42u8; 32];
        let wave = WaveToken::new(token, 1);
        let epoch_seed = [99u8; 32];
        
        let perm = wave.expand(&epoch_seed, 8);
        
        // Check it's a valid permutation
        assert_eq!(perm.len(), 8);
        let mut sorted = perm.clone();
        sorted.sort();
        assert_eq!(sorted, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }
    
    #[test]
    fn test_base_wave_expansion() {
        let wave = WaveToken::base_wave(1);
        let epoch_seed = [99u8; 32];
        
        let perm = wave.expand(&epoch_seed, 8);
        
        // Base wave should be identity permutation
        assert_eq!(perm, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }
    
    #[test]
    fn test_deterministic_expansion() {
        let token = [42u8; 32];
        let wave = WaveToken::new(token, 1);
        let epoch_seed = [99u8; 32];
        
        let perm1 = wave.expand(&epoch_seed, 8);
        let perm2 = wave.expand(&epoch_seed, 8);
        
        // Should be deterministic
        assert_eq!(perm1, perm2);
    }
    
    #[test]
    fn test_portal_wave_expansion() {
        let token = WaveToken::new([42u8; 32], 1);
        let epoch_seed = [99u8; 32];
        
        let portal = PortalWave::expand(&token, &epoch_seed, 4, 8);
        
        assert_eq!(portal.cluster_order.len(), 4);
        assert_eq!(portal.cluster_perms.len(), 4);
        
        for perm in &portal.cluster_perms {
            assert_eq!(perm.len(), 8);
        }
    }
    
    #[test]
    fn test_portal_routing() {
        let token = WaveToken::new([42u8; 32], 1);
        let epoch_seed = [99u8; 32];
        let portal = PortalWave::expand(&token, &epoch_seed, 4, 8);
        
        // Intra-cluster routing
        let (next_cluster, next_pos) = portal.next_hop(0, 0, 0);
        assert_eq!(next_cluster, 0);
        
        // Inter-cluster routing
        let (next_cluster, _) = portal.next_hop(0, 1, 0);
        assert_ne!(next_cluster, 0);
    }
    
    #[test]
    fn test_wave_scheduler() {
        let scheduler = WaveScheduler::new(10);
        let epoch_seed = [99u8; 32];
        
        let waves = scheduler.select_waves(1, &epoch_seed);
        
        assert_eq!(waves.len(), 10);
        assert!(waves[0].is_base_wave());
    }
    
    #[test]
    fn test_wave_serialization() {
        let token = WaveToken::new([42u8; 32], 1);
        
        let json = serde_json::to_string(&token).unwrap();
        let token2: WaveToken = serde_json::from_str(&json).unwrap();
        
        assert_eq!(token, token2);
    }
}
