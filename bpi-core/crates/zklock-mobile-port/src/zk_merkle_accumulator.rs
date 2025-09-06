//! ZK Merkle Accumulator - Efficient state management for lightweight devices
//!
//! This module implements a zero-knowledge Merkle accumulator that allows
//! IoT and mobile devices to maintain state without requiring full blockchain sync.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::{ZKConfig, DeviceType};

/// ZK Merkle accumulator for efficient state management
#[derive(Debug)]
pub struct ZKMerkleAccumulator {
    /// Merkle tree nodes
    tree: Arc<RwLock<MerkleTree>>,
    /// ZK proof cache for mobile optimization
    proof_cache: Arc<RwLock<HashMap<String, CachedProof>>>,
    /// Device proof history
    device_proofs: Arc<RwLock<HashMap<Uuid, Vec<ProofEntry>>>>,
    /// Configuration
    config: ZKConfig,
    /// Statistics
    stats: Arc<RwLock<AccumulatorStats>>,
}

/// Merkle tree implementation optimized for mobile devices
#[derive(Debug, Clone)]
pub struct MerkleTree {
    /// Tree depth
    depth: usize,
    /// Tree nodes (level -> index -> hash)
    nodes: HashMap<usize, HashMap<usize, [u8; 32]>>,
    /// Leaf count
    leaf_count: usize,
    /// Root hash
    root_hash: [u8; 32],
}

/// Cached proof for mobile optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedProof {
    pub proof_id: String,
    pub device_id: Uuid,
    pub proof_data: Vec<u8>,
    pub merkle_path: Vec<[u8; 32]>,
    pub leaf_index: usize,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub verification_count: u64,
    pub size_bytes: usize,
}

/// Proof entry for device history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofEntry {
    pub proof_id: String,
    pub device_id: Uuid,
    pub device_type: DeviceType,
    pub proof_hash: [u8; 32],
    pub leaf_index: usize,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub verification_status: VerificationStatus,
    pub gas_cost: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VerificationStatus {
    Pending,
    Verified,
    Failed,
    Cached,
}

/// Accumulator statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccumulatorStats {
    pub tree_size: u64,
    pub total_proofs: u64,
    pub cached_proofs: u64,
    pub verification_count: u64,
    pub average_proof_size: f64,
    pub cache_hit_rate: f64,
    pub mobile_optimizations: u64,
}

impl Default for AccumulatorStats {
    fn default() -> Self {
        Self {
            tree_size: 0,
            total_proofs: 0,
            cached_proofs: 0,
            verification_count: 0,
            average_proof_size: 0.0,
            cache_hit_rate: 0.0,
            mobile_optimizations: 0,
        }
    }
}

impl MerkleTree {
    /// Create a new Merkle tree
    pub fn new(depth: usize) -> Self {
        let mut tree = Self {
            depth,
            nodes: HashMap::new(),
            leaf_count: 0,
            root_hash: [0u8; 32],
        };

        // Initialize empty tree
        tree.initialize_empty_tree();
        tree
    }

    /// Initialize empty tree with default values
    fn initialize_empty_tree(&mut self) {
        // Initialize all levels
        for level in 0..=self.depth {
            self.nodes.insert(level, HashMap::new());
        }

        // Set empty root
        self.root_hash = Self::empty_hash();
    }

    /// Add a new leaf to the tree
    pub fn add_leaf(&mut self, data: &[u8]) -> Result<usize> {
        let leaf_index = self.leaf_count;
        let leaf_hash = Self::hash_data(data);

        // Add leaf at level 0
        self.nodes.get_mut(&0).unwrap().insert(leaf_index, leaf_hash);
        self.leaf_count += 1;

        // Update parent nodes
        self.update_parents(leaf_index)?;

        Ok(leaf_index)
    }

    /// Update parent nodes after leaf insertion
    fn update_parents(&mut self, leaf_index: usize) -> Result<()> {
        let mut current_index = leaf_index;

        for level in 1..=self.depth {
            let parent_index = current_index / 2;
            let sibling_index = if current_index % 2 == 0 { current_index + 1 } else { current_index - 1 };

            // Get current node hash
            let current_hash = self.nodes.get(&(level - 1))
                .and_then(|level_nodes| level_nodes.get(&current_index))
                .copied()
                .unwrap_or_else(Self::empty_hash);

            // Get sibling hash
            let sibling_hash = self.nodes.get(&(level - 1))
                .and_then(|level_nodes| level_nodes.get(&sibling_index))
                .copied()
                .unwrap_or_else(Self::empty_hash);

            // Compute parent hash
            let parent_hash = if current_index % 2 == 0 {
                Self::hash_pair(&current_hash, &sibling_hash)
            } else {
                Self::hash_pair(&sibling_hash, &current_hash)
            };

            // Update parent node
            self.nodes.get_mut(&level).unwrap().insert(parent_index, parent_hash);

            // Update root if we're at the top
            if level == self.depth {
                self.root_hash = parent_hash;
            }

            current_index = parent_index;
        }

        Ok(())
    }

    /// Generate Merkle proof for a leaf
    pub fn generate_proof(&self, leaf_index: usize) -> Result<Vec<[u8; 32]>> {
        if leaf_index >= self.leaf_count {
            return Err(anyhow::anyhow!("Leaf index out of bounds"));
        }

        let mut proof = Vec::new();
        let mut current_index = leaf_index;

        for level in 0..self.depth {
            let sibling_index = if current_index % 2 == 0 { current_index + 1 } else { current_index - 1 };

            let sibling_hash = self.nodes.get(&level)
                .and_then(|level_nodes| level_nodes.get(&sibling_index))
                .copied()
                .unwrap_or_else(Self::empty_hash);

            proof.push(sibling_hash);
            current_index = current_index / 2;
        }

        Ok(proof)
    }

    /// Verify a Merkle proof
    pub fn verify_proof(&self, leaf_data: &[u8], leaf_index: usize, proof: &[[u8; 32]]) -> bool {
        if proof.len() != self.depth {
            return false;
        }

        let mut current_hash = Self::hash_data(leaf_data);
        let mut current_index = leaf_index;

        for &sibling_hash in proof {
            current_hash = if current_index % 2 == 0 {
                Self::hash_pair(&current_hash, &sibling_hash)
            } else {
                Self::hash_pair(&sibling_hash, &current_hash)
            };
            current_index = current_index / 2;
        }

        current_hash == self.root_hash
    }

    /// Get root hash
    pub fn root(&self) -> [u8; 32] {
        self.root_hash
    }

    /// Hash data using SHA256
    fn hash_data(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    /// Hash a pair of hashes
    fn hash_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(left);
        hasher.update(right);
        hasher.finalize().into()
    }

    /// Empty hash (all zeros)
    fn empty_hash() -> [u8; 32] {
        [0u8; 32]
    }
}

impl ZKMerkleAccumulator {
    /// Create a new ZK Merkle accumulator
    pub async fn new(config: ZKConfig) -> Result<Self> {
        info!("Initializing ZK Merkle Accumulator with depth {}", config.merkle_depth);

        let tree = Arc::new(RwLock::new(MerkleTree::new(config.merkle_depth)));

        Ok(Self {
            tree,
            proof_cache: Arc::new(RwLock::new(HashMap::new())),
            device_proofs: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(RwLock::new(AccumulatorStats::default())),
        })
    }

    /// Start the accumulator
    pub async fn start(&self) -> Result<()> {
        info!("Starting ZK Merkle Accumulator");
        
        // Start background tasks
        self.start_cache_cleanup_task().await;
        self.start_stats_update_task().await;

        Ok(())
    }

    /// Add a proof to the accumulator
    pub async fn add_proof(&self, device_id: Uuid, proof_data: Vec<u8>) -> Result<String> {
        let proof_id = Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now();

        // Add to Merkle tree
        let leaf_index = {
            let mut tree = self.tree.write().await;
            tree.add_leaf(&proof_data)?
        };

        // Generate Merkle proof
        let merkle_path = {
            let tree = self.tree.read().await;
            tree.generate_proof(leaf_index)?
        };

        // Create cached proof
        let cached_proof = CachedProof {
            proof_id: proof_id.clone(),
            device_id,
            proof_data: proof_data.clone(),
            merkle_path,
            leaf_index,
            timestamp,
            verification_count: 0,
            size_bytes: proof_data.len(),
        };

        // Cache the proof
        self.proof_cache.write().await.insert(proof_id.clone(), cached_proof);

        // Add to device history
        let proof_entry = ProofEntry {
            proof_id: proof_id.clone(),
            device_id,
            device_type: DeviceType::Mobile { 
                platform: crate::MobilePlatform::Android, 
                capabilities: crate::MobileCapabilities {
                    ram_mb: 0,
                    storage_gb: 0,
                    has_secure_enclave: false,
                    supports_biometrics: false,
                    network_types: vec![],
                }
            }, // TODO: Get actual device type
            proof_hash: MerkleTree::hash_data(&proof_data),
            leaf_index,
            timestamp,
            verification_status: VerificationStatus::Verified,
            gas_cost: self.calculate_gas_cost(&proof_data).await,
        };

        self.device_proofs.write().await
            .entry(device_id)
            .or_insert_with(Vec::new)
            .push(proof_entry);

        // Update statistics
        self.update_stats_after_proof_addition(proof_data.len()).await;

        info!("Added proof {} for device {} at leaf index {}", proof_id, device_id, leaf_index);
        Ok(proof_id)
    }

    /// Verify a proof
    pub async fn verify_proof(&self, proof_id: &str) -> Result<bool> {
        let cached_proof = {
            let mut cache = self.proof_cache.write().await;
            if let Some(proof) = cache.get_mut(proof_id) {
                proof.verification_count += 1;
                proof.clone()
            } else {
                return Err(anyhow::anyhow!("Proof not found in cache"));
            }
        };

        // Verify against Merkle tree
        let is_valid = {
            let tree = self.tree.read().await;
            tree.verify_proof(&cached_proof.proof_data, cached_proof.leaf_index, &cached_proof.merkle_path)
        };

        // Update statistics
        self.update_stats_after_verification().await;

        debug!("Verified proof {}: {}", proof_id, is_valid);
        Ok(is_valid)
    }

    /// Get proof by ID
    pub async fn get_proof(&self, proof_id: &str) -> Result<CachedProof> {
        self.proof_cache.read().await
            .get(proof_id)
            .cloned()
            .context("Proof not found")
    }

    /// Get device proof history
    pub async fn get_device_proofs(&self, device_id: Uuid) -> Result<Vec<ProofEntry>> {
        Ok(self.device_proofs.read().await
            .get(&device_id)
            .cloned()
            .unwrap_or_default())
    }

    /// Get accumulator statistics
    pub async fn get_stats(&self) -> Result<AccumulatorStats> {
        Ok(self.stats.read().await.clone())
    }

    /// Calculate gas cost for proof (mobile-optimized)
    async fn calculate_gas_cost(&self, proof_data: &[u8]) -> u64 {
        // Base cost + size-based cost (optimized for mobile)
        let base_cost = 1000u64;
        let size_cost = (proof_data.len() as u64) / 10; // 1 gas per 10 bytes
        base_cost + size_cost
    }

    /// Update statistics after proof addition
    async fn update_stats_after_proof_addition(&self, proof_size: usize) {
        let mut stats = self.stats.write().await;
        stats.total_proofs += 1;
        stats.tree_size = {
            let tree = self.tree.try_read();
            if let Ok(tree) = tree {
                tree.leaf_count as u64
            } else {
                stats.tree_size
            }
        };
        
        // Update average proof size
        let total_size = stats.average_proof_size * (stats.total_proofs - 1) as f64 + proof_size as f64;
        stats.average_proof_size = total_size / stats.total_proofs as f64;
    }

    /// Update statistics after verification
    async fn update_stats_after_verification(&self) {
        let mut stats = self.stats.write().await;
        stats.verification_count += 1;
        
        // Update cache hit rate
        let cache_size = self.proof_cache.try_read().map(|c| c.len()).unwrap_or(0);
        stats.cached_proofs = cache_size as u64;
        stats.cache_hit_rate = if stats.verification_count > 0 {
            stats.cached_proofs as f64 / stats.verification_count as f64
        } else {
            0.0
        };
    }

    /// Start cache cleanup background task
    async fn start_cache_cleanup_task(&self) {
        let proof_cache = Arc::clone(&self.proof_cache);
        let stats = Arc::clone(&self.stats);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // 5 minutes
            
            loop {
                interval.tick().await;
                
                // Clean up old cached proofs (older than 1 hour)
                let cutoff = chrono::Utc::now() - chrono::Duration::hours(1);
                let mut cache = proof_cache.write().await;
                let initial_size = cache.len();
                
                cache.retain(|_, proof| proof.timestamp > cutoff);
                
                let cleaned = initial_size - cache.len();
                if cleaned > 0 {
                    debug!("Cleaned up {} old cached proofs", cleaned);
                    
                    // Update stats
                    let mut stats = stats.write().await;
                    stats.cached_proofs = cache.len() as u64;
                }
            }
        });
    }

    /// Start statistics update background task
    async fn start_stats_update_task(&self) {
        let stats = Arc::clone(&self.stats);
        let tree = Arc::clone(&self.tree);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60)); // 1 minute
            
            loop {
                interval.tick().await;
                
                // Update tree size
                if let Ok(tree) = tree.try_read() {
                    let mut stats = stats.write().await;
                    stats.tree_size = tree.leaf_count as u64;
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_merkle_tree_operations() {
        let mut tree = MerkleTree::new(3);
        
        // Add some leaves
        let leaf1 = tree.add_leaf(b"data1").unwrap();
        let leaf2 = tree.add_leaf(b"data2").unwrap();
        
        assert_eq!(leaf1, 0);
        assert_eq!(leaf2, 1);
        
        // Generate and verify proof
        let proof = tree.generate_proof(leaf1).unwrap();
        assert!(tree.verify_proof(b"data1", leaf1, &proof));
        assert!(!tree.verify_proof(b"wrong_data", leaf1, &proof));
    }

    #[tokio::test]
    async fn test_zk_merkle_accumulator() {
        let config = ZKConfig {
            max_proof_size: 1024,
            max_verification_time_ms: 100,
            merkle_depth: 10,
            batch_size: 100,
        };
        
        let accumulator = ZKMerkleAccumulator::new(config).await.unwrap();
        let device_id = Uuid::new_v4();
        
        // Add a proof
        let proof_data = b"test_proof_data".to_vec();
        let proof_id = accumulator.add_proof(device_id, proof_data).await.unwrap();
        
        // Verify the proof
        let is_valid = accumulator.verify_proof(&proof_id).await.unwrap();
        assert!(is_valid);
        
        // Get proof
        let cached_proof = accumulator.get_proof(&proof_id).await.unwrap();
        assert_eq!(cached_proof.device_id, device_id);
    }
}
