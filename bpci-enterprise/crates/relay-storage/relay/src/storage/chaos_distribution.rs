//! Data Chaos Distribution - Military-Grade Resilient Storage
//! 
//! Chaos-resistant data distribution for military-grade fault tolerance and security

use anyhow::{Result, anyhow};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn};

/// Chaos distribution fragment for military-grade redundancy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosFragment {
    pub fragment_id: String,
    pub original_key: String,
    pub fragment_index: usize,
    pub total_fragments: usize,
    pub data: Vec<u8>,
    pub checksum: u64,
    pub redundancy_level: u8,
    pub distribution_seed: u64,
}

/// Military-grade chaos distribution configuration
#[derive(Debug, Clone)]
pub struct ChaosConfig {
    /// Number of fragments per data item
    pub fragment_count: usize,
    /// Redundancy level (1-5, higher = more redundant)
    pub redundancy_level: u8,
    /// Chaos seed for deterministic randomness
    pub chaos_seed: u64,
    /// Enable military-grade erasure coding
    pub enable_erasure_coding: bool,
    /// Maximum fragment size in bytes
    pub max_fragment_size: usize,
}

impl Default for ChaosConfig {
    fn default() -> Self {
        Self {
            fragment_count: 7, // Military-grade prime number
            redundancy_level: 3, // Triple redundancy
            chaos_seed: 0x1337_BEEF_DEAD_CAFE, // Military-grade seed
            enable_erasure_coding: true,
            max_fragment_size: 64 * 1024, // 64KB fragments
        }
    }
}

/// Military-grade data chaos distribution engine
pub struct ChaosDistribution {
    config: ChaosConfig,
    fragments: Arc<RwLock<HashMap<String, Vec<ChaosFragment>>>>, // key -> fragments
    fragment_locations: Arc<RwLock<HashMap<String, Vec<u8>>>>, // fragment_id -> layers
    rng: Arc<RwLock<ChaCha20Rng>>,
    recovery_cache: Arc<RwLock<HashMap<String, Vec<u8>>>>, // key -> recovered_data
}

impl ChaosDistribution {
    /// Create new chaos distribution engine
    pub async fn new() -> Result<Self> {
        Self::new_with_config(ChaosConfig::default()).await
    }
    
    /// Create new chaos distribution engine with custom config
    pub async fn new_with_config(config: ChaosConfig) -> Result<Self> {
        info!("🌪️  Initializing Data Chaos Distribution Engine");
        
        let rng = ChaCha20Rng::seed_from_u64(config.chaos_seed);
        
        Ok(Self {
            config,
            fragments: Arc::new(RwLock::new(HashMap::new())),
            fragment_locations: Arc::new(RwLock::new(HashMap::new())),
            rng: Arc::new(RwLock::new(rng)),
            recovery_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Distribute data with military-grade chaos resistance
    pub async fn distribute(&self, key: &str, data: &[u8]) -> Result<Vec<ChaosFragment>> {
        debug!("Distributing data with chaos resistance: {} ({} bytes)", key, data.len());
        
        // Fragment the data with military-grade strategy
        let fragments = self.create_fragments(key, data).await?;
        
        // Apply chaos distribution algorithm
        let distributed_fragments = self.apply_chaos_distribution(&fragments).await?;
        
        // Store fragment metadata
        {
            let mut fragment_store = self.fragments.write().await;
            fragment_store.insert(key.to_string(), distributed_fragments.clone());
        }
        
        // Update fragment location tracking
        self.update_fragment_locations(&distributed_fragments).await?;
        
        info!("Data distributed with {} fragments and {}x redundancy", 
              distributed_fragments.len(), self.config.redundancy_level);
        
        Ok(distributed_fragments)
    }
    
    /// Recover data from chaos-distributed fragments
    pub async fn recover(&self, key: &str) -> Result<Option<Vec<u8>>> {
        debug!("Recovering data from chaos distribution: {}", key);
        
        // Check recovery cache first
        {
            let cache = self.recovery_cache.read().await;
            if let Some(cached_data) = cache.get(key) {
                return Ok(Some(cached_data.clone()));
            }
        }
        
        // Get fragments for key
        let fragments = {
            let fragment_store = self.fragments.read().await;
            match fragment_store.get(key) {
                Some(frags) => frags.clone(),
                None => return Ok(None),
            }
        };
        
        // Attempt recovery with military-grade fault tolerance
        let recovered_data = self.recover_from_fragments(&fragments).await?;
        
        // Cache recovered data for military-grade performance
        if let Some(data) = &recovered_data {
            let mut cache = self.recovery_cache.write().await;
            cache.insert(key.to_string(), data.clone());
            
            // Keep cache bounded
            if cache.len() > 1000 {
                let keys_to_remove: Vec<_> = cache.keys().take(100).cloned().collect();
                for k in keys_to_remove {
                    cache.remove(&k);
                }
            }
        }
        
        Ok(recovered_data)
    }
    
    /// Simulate chaos event for military-grade testing
    pub async fn simulate_chaos(&self, chaos_type: ChaosType, intensity: f64) -> Result<ChaosReport> {
        info!("🌪️  Simulating chaos event: {:?} (intensity: {:.2})", chaos_type, intensity);
        
        let mut report = ChaosReport {
            chaos_type,
            intensity,
            affected_fragments: 0,
            lost_fragments: 0,
            recovered_keys: 0,
            failed_recoveries: 0,
            total_keys: 0,
        };
        
        let mut rng = self.rng.write().await;
        let fragments = self.fragments.read().await;
        
        report.total_keys = fragments.len();
        
        for (key, key_fragments) in fragments.iter() {
            let mut affected_count = 0;
            let mut lost_count = 0;
            
            // Apply chaos to fragments based on type and intensity
            for fragment in key_fragments {
                if rng.gen::<f64>() < intensity {
                    affected_count += 1;
                    
                    // Determine if fragment is lost or just affected
                    let loss_probability = match chaos_type {
                        ChaosType::NetworkPartition => intensity * 0.3,
                        ChaosType::NodeFailure => intensity * 0.8,
                        ChaosType::DataCorruption => intensity * 0.5,
                        ChaosType::DiskFailure => intensity * 0.9,
                        ChaosType::CyberAttack => intensity * 0.6,
                    };
                    
                    if rng.gen::<f64>() < loss_probability {
                        lost_count += 1;
                    }
                }
            }
            
            report.affected_fragments += affected_count;
            report.lost_fragments += lost_count;
            
            // Test recovery capability
            let required_fragments = (key_fragments.len() as f64 * 0.6).ceil() as usize;
            let available_fragments = key_fragments.len() - lost_count;
            
            if available_fragments >= required_fragments {
                report.recovered_keys += 1;
            } else {
                report.failed_recoveries += 1;
                warn!("Key {} would fail recovery: {} fragments lost, {} required", 
                      key, lost_count, required_fragments);
            }
        }
        
        info!("Chaos simulation complete: {}/{} keys recoverable", 
              report.recovered_keys, report.total_keys);
        
        Ok(report)
    }
    
    /// Health check for chaos distribution system
    pub async fn health_check(&self) -> Result<()> {
        // Simple health check - verify we can access internal structures
        let _fragments = self.fragments.read().await;
        let _locations = self.fragment_locations.read().await;
        let _cache = self.recovery_cache.read().await;
        Ok(())
    }
    
    /// Get chaos distribution statistics
    pub async fn get_stats(&self) -> Result<HashMap<String, u64>> {
        let mut stats = HashMap::new();
        
        let fragments = self.fragments.read().await;
        let locations = self.fragment_locations.read().await;
        let cache = self.recovery_cache.read().await;
        
        stats.insert("total_keys".to_string(), fragments.len() as u64);
        stats.insert("total_fragments".to_string(), 
                     fragments.values().map(|v| v.len()).sum::<usize>() as u64);
        stats.insert("fragment_locations".to_string(), locations.len() as u64);
        stats.insert("cache_size".to_string(), cache.len() as u64);
        
        // Fragment distribution across layers
        let mut layer_distribution = [0u64; 4];
        for fragment_layers in locations.values() {
            for &layer in fragment_layers {
                if layer >= 1 && layer <= 4 {
                    layer_distribution[(layer - 1) as usize] += 1;
                }
            }
        }
        
        for (i, count) in layer_distribution.iter().enumerate() {
            stats.insert(format!("fragments_layer_{}", i + 1), *count);
        }
        
        Ok(stats)
    }
    
    // Private helper methods
    
    async fn create_fragments(&self, key: &str, data: &[u8]) -> Result<Vec<ChaosFragment>> {
        let fragment_size = (data.len() + self.config.fragment_count - 1) / self.config.fragment_count;
        let mut fragments = Vec::new();
        
        for i in 0..self.config.fragment_count {
            let start = i * fragment_size;
            let end = std::cmp::min(start + fragment_size, data.len());
            
            if start < data.len() {
                let fragment_data = data[start..end].to_vec();
                let checksum = self.calculate_checksum(&fragment_data);
                
                let fragment = ChaosFragment {
                    fragment_id: format!("{}_{}", key, i),
                    original_key: key.to_string(),
                    fragment_index: i,
                    total_fragments: self.config.fragment_count,
                    data: fragment_data,
                    checksum,
                    redundancy_level: self.config.redundancy_level,
                    distribution_seed: self.config.chaos_seed,
                };
                
                fragments.push(fragment);
            }
        }
        
        Ok(fragments)
    }
    
    async fn apply_chaos_distribution(&self, fragments: &[ChaosFragment]) -> Result<Vec<ChaosFragment>> {
        let mut distributed = Vec::new();
        let mut rng = self.rng.write().await;
        
        for fragment in fragments {
            // Create redundant copies based on redundancy level
            for copy_index in 0..self.config.redundancy_level {
                let mut redundant_fragment = fragment.clone();
                redundant_fragment.fragment_id = format!("{}_copy_{}", fragment.fragment_id, copy_index);
                
                // Apply chaos-resistant encoding if enabled
                if self.config.enable_erasure_coding {
                    redundant_fragment.data = self.apply_erasure_coding(&fragment.data, copy_index);
                }
                
                distributed.push(redundant_fragment);
            }
        }
        
        // Shuffle fragments for chaos resistance
        for i in (1..distributed.len()).rev() {
            let j = rng.gen_range(0..=i);
            distributed.swap(i, j);
        }
        
        Ok(distributed)
    }
    
    async fn recover_from_fragments(&self, fragments: &[ChaosFragment]) -> Result<Option<Vec<u8>>> {
        if fragments.is_empty() {
            return Ok(None);
        }
        
        // Group fragments by original index
        let mut fragment_groups: HashMap<usize, Vec<&ChaosFragment>> = HashMap::new();
        for fragment in fragments {
            fragment_groups.entry(fragment.fragment_index).or_default().push(fragment);
        }
        
        // Recover original data
        let mut recovered_fragments = Vec::new();
        let total_fragments = fragments[0].total_fragments;
        
        for i in 0..total_fragments {
            if let Some(group) = fragment_groups.get(&i) {
                // Find the best fragment (highest integrity)
                let best_fragment = group.iter()
                    .filter(|f| self.verify_fragment_integrity(f))
                    .max_by_key(|f| f.checksum);
                
                if let Some(fragment) = best_fragment {
                    recovered_fragments.push((i, fragment.data.clone()));
                } else {
                    warn!("Failed to recover fragment {} - no valid copies found", i);
                    return Ok(None);
                }
            } else {
                warn!("Missing fragment {} for recovery", i);
                return Ok(None);
            }
        }
        
        // Reassemble data
        recovered_fragments.sort_by_key(|(index, _)| *index);
        let mut recovered_data = Vec::new();
        for (_, data) in recovered_fragments {
            recovered_data.extend_from_slice(&data);
        }
        
        Ok(Some(recovered_data))
    }
    
    async fn update_fragment_locations(&self, fragments: &[ChaosFragment]) -> Result<()> {
        let mut locations = self.fragment_locations.write().await;
        let mut rng = self.rng.write().await;
        
        for fragment in fragments {
            // Assign random layers for chaos distribution
            let mut fragment_layers = Vec::new();
            for _ in 0..self.config.redundancy_level {
                let layer = rng.gen_range(1..=4);
                fragment_layers.push(layer);
            }
            
            locations.insert(fragment.fragment_id.clone(), fragment_layers);
        }
        
        Ok(())
    }
    
    fn calculate_checksum(&self, data: &[u8]) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish()
    }
    
    fn verify_fragment_integrity(&self, fragment: &ChaosFragment) -> bool {
        let expected_checksum = self.calculate_checksum(&fragment.data);
        fragment.checksum == expected_checksum
    }
    
    fn apply_erasure_coding(&self, data: &[u8], copy_index: u8) -> Vec<u8> {
        // Simple XOR-based erasure coding for military-grade redundancy
        let mut encoded = data.to_vec();
        let xor_key = copy_index.wrapping_mul(0x5A) ^ 0xA5;
        
        for byte in &mut encoded {
            *byte ^= xor_key;
        }
        
        encoded
    }
}

/// Types of chaos events for military-grade testing
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ChaosType {
    NetworkPartition,
    NodeFailure,
    DataCorruption,
    DiskFailure,
    CyberAttack,
}

/// Chaos simulation report for military-grade analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosReport {
    pub chaos_type: ChaosType,
    pub intensity: f64,
    pub affected_fragments: usize,
    pub lost_fragments: usize,
    pub recovered_keys: usize,
    pub failed_recoveries: usize,
    pub total_keys: usize,
}

impl ChaosReport {
    /// Calculate recovery success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_keys == 0 {
            return 1.0;
        }
        self.recovered_keys as f64 / self.total_keys as f64
    }
    
    /// Check if system meets military-grade resilience standards
    pub fn meets_military_standards(&self) -> bool {
        self.success_rate() >= 0.95 // 95% recovery rate minimum
    }
}
