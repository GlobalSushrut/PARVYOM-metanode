// QGC-C² Core - Quantized Gradient Consensus (Category + Knot)
// Ultra-lightweight consensus for BPI ledger: ~30MB RAM, 1 vCPU + 2GB total system
// Evidence-only attestations, quantized confidence, categorical commits, knot-aware stability

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use blake3;

/// Core QGC-C² configuration (frozen defaults)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QgcConfig {
    pub committee_size: u8,           // c = 24 (cap 32)
    pub max_validators: u8,           // n ≤ 128
    pub max_parents_per_batch: u8,    // ≤3
    pub threshold_band: u8,           // Q* = 48 (of 63)
    pub rs_da_k: u8,                  // k = 10
    pub rs_da_m: u8,                  // m = 14
    pub timeout_base_ms: u64,         // 400ms
    pub checkpoint_interval: u32,     // every 256 rounds
    pub epoch_interval: u32,          // 2048 rounds
}

impl Default for QgcConfig {
    fn default() -> Self {
        Self {
            committee_size: 24,
            max_validators: 128,
            max_parents_per_batch: 3,
            threshold_band: 48,
            rs_da_k: 10,
            rs_da_m: 14,
            timeout_base_ms: 400,
            checkpoint_interval: 256,
            epoch_interval: 2048,
        }
    }
}

/// Batch - Core unit of consensus (120 bytes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Batch {
    pub id: [u8; 32],                    // 32B - batch identifier
    pub parent_ids: [[u8; 32]; 3],       // 96B - up to 3 parents (unused filled with zeros)
    pub tx_root: [u8; 32],               // 32B - transaction merkle root
    pub maker: [u8; 32],                 // 32B - batch creator
    pub strand: u16,                     // 2B - strand identifier
    pub timestamp: u64,                  // 8B - creation timestamp
    pub parent_count: u8,                // 1B - actual number of parents (≤3)
    pub _padding: [u8; 7],               // 7B - padding to align to 240B total
}

impl Batch {
    pub fn new(tx_root: [u8; 32], maker: [u8; 32], strand: u16, parents: Vec<[u8; 32]>) -> Self {
        let mut parent_ids = [[0u8; 32]; 3];
        let parent_count = std::cmp::min(parents.len(), 3) as u8;
        
        for (i, parent) in parents.iter().take(3).enumerate() {
            parent_ids[i] = *parent;
        }
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Generate batch ID from content
        let mut hasher = blake3::Hasher::new();
        hasher.update(&tx_root);
        hasher.update(&maker);
        hasher.update(&strand.to_le_bytes());
        hasher.update(&timestamp.to_le_bytes());
        for parent in &parent_ids {
            hasher.update(parent);
        }
        let id = hasher.finalize().into();
        
        Self {
            id,
            parent_ids,
            tx_root,
            maker,
            strand,
            timestamp,
            parent_count,
            _padding: [0; 7],
        }
    }
    
    pub fn get_parents(&self) -> Vec<[u8; 32]> {
        self.parent_ids[..self.parent_count as usize].to_vec()
    }
    
    pub fn size_bytes() -> usize {
        240 // Fixed size for hardware-aware allocation
    }
}

/// Confidence Attestation - Evidence-only attestation (236 bytes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceAttestation {
    pub r: u64,                          // 8B - round number
    pub cid: [u8; 32],                   // 32B - candidate batch ID
    #[serde(with = "serde_bytes")]
    pub vrf_proof: Vec<u8>,              // 80B - VRF proof for committee selection
    pub da_k: u8,                        // 1B - DA chunks seen (k)
    pub da_m: u8,                        // 1B - DA total chunks (m)
    pub parent_cc: [u8; 16],             // 16B - parent CC reference (truncated hash)
    pub qos: u16,                        // 2B - quality of service metric
    pub qstep: u8,                       // 1B - quantized step delta (0..15)
    #[serde(with = "serde_bytes")]
    pub bls_part: Vec<u8>,               // 96B - BLS partial signature
}

impl ConfidenceAttestation {
    pub fn size_bytes() -> usize {
        236
    }
}

/// Confidence Certificate - Aggregated attestations (91 bytes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceCertificate {
    pub r: u64,                          // 8B - round number
    pub cid: [u8; 32],                   // 32B - candidate batch ID
    pub bitmap: u32,                     // 4B - validator bitmap (up to 32 validators)
    #[serde(with = "serde_bytes")]
    pub bls_agg: Vec<u8>,                // 48B - aggregated BLS signature
    pub qscore: u8,                      // 1B - quantized confidence score (0..63)
    pub da_ratio: u8,                    // 1B - DA availability ratio
    pub knot_k: u16,                     // 2B - knot complexity metric
    pub timestamp: u64,                  // 8B - certificate creation time
}

impl ConfidenceCertificate {
    pub fn size_bytes() -> usize {
        104 // Slightly larger than spec for alignment
    }
    
    pub fn get_validator_count(&self) -> u8 {
        self.bitmap.count_ones() as u8
    }
}

/// Knot Metric - Tangle complexity tracking (10 bytes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnotMetric {
    pub win: u32,                        // 4B - window size
    pub crossings: u16,                  // 2B - strand crossings/inversions
    pub link: u16,                       // 2B - signed pair linkage estimate
    pub rate: u16,                       // 2B - arrival rate (arrivals/Δt)
    pub k: u16,                          // 2B - computed K metric
}

impl KnotMetric {
    pub fn new() -> Self {
        Self {
            win: 512,
            crossings: 0,
            link: 0,
            rate: 0,
            k: 0,
        }
    }
    
    pub fn compute_k(&mut self, alpha: u16, beta: u16, gamma: u16) {
        // K = α·crossings + β·link + γ·rate
        let k_val = (alpha as u32 * self.crossings as u32 +
                     beta as u32 * self.link as u32 +
                     gamma as u32 * self.rate as u32) as u16;
        self.k = k_val;
    }
    
    pub fn size_bytes() -> usize {
        12 // Slightly larger for alignment
    }
}

/// QGC Header - Block header with consensus proof (~149 bytes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QgcHeader {
    pub height: u64,                     // 8B - block height
    pub order_root: [u8; 32],            // 32B - transaction order root
    pub ziplock_root: [u8; 32],          // 32B - VM ziplock state root
    pub poe_root: [u8; 32],              // 32B - proof of execution root
    pub cc: ConfidenceCertificate,       // 104B - confidence certificate
    pub knot: KnotMetric,                // 12B - knot complexity metric
    pub timestamp: u64,                  // 8B - header timestamp
    pub previous_hash: [u8; 32],         // 32B - previous header hash
}

impl QgcHeader {
    pub fn size_bytes() -> usize {
        292 // Larger than spec but still lightweight
    }
}

/// Quantized Confidence Scorer - LUT-driven integer scoring
#[derive(Debug, Clone)]
pub struct QuantizedScorer {
    // Committee term LUT: L_comm[valid_cas][committee_size] -> qΔ
    committee_lut: [[u8; 32]; 32],       // Max 32 CAs, 32 committee size
    // Link term constant
    link_bonus: u8,                      // e.g., 8 steps
    // DA terms
    da_small_bonus: u8,                  // per RS chunk
    da_big_bonus: u8,                    // on reaching k chunks
    // Risk adjustment
    risk_penalty: u8,                    // when K > K*
}

impl QuantizedScorer {
    pub fn new() -> Self {
        let mut scorer = Self {
            committee_lut: [[0; 32]; 32],
            link_bonus: 8,
            da_small_bonus: 1,
            da_big_bonus: 12,
            risk_penalty: 4,
        };
        
        // Initialize Chernoff-bound LUT (simplified)
        for cas in 0..32 {
            for committee_size in 1..32 {
                // Simplified Chernoff bound approximation with overflow protection
                let ratio = (cas as f64) / (committee_size as f64);
                let confidence_delta = if ratio > 0.67 {
                    let calc = 8.0 + ((ratio - 0.67) * 20.0);
                    std::cmp::min(calc as u8, 255)
                } else if ratio > 0.33 {
                    let calc = 4.0 + ((ratio - 0.33) * 12.0);
                    std::cmp::min(calc as u8, 255)
                } else {
                    let calc = ratio * 12.0;
                    std::cmp::min(calc as u8, 255)
                };
                scorer.committee_lut[cas][committee_size] = std::cmp::min(confidence_delta, 15);
            }
        }
        
        scorer
    }
    
    pub fn score_attestations(&self, cas: &[ConfidenceAttestation], committee_size: u8, has_two_link: bool, k_metric: u16, k_threshold: u16) -> u8 {
        let mut score = 0u8;
        
        // Committee term
        let valid_cas = std::cmp::min(cas.len(), 31);
        let committee_idx = std::cmp::min(committee_size as usize, 31);
        score = score.saturating_add(self.committee_lut[valid_cas][committee_idx]);
        
        // Link term (two-link bonus)
        if has_two_link {
            score = score.saturating_add(self.link_bonus);
        }
        
        // DA term
        for ca in cas {
            // Small bonus per chunk
            score = score.saturating_add(ca.da_k.saturating_mul(self.da_small_bonus));
            // Big bonus if reached k threshold
            if ca.da_k >= ca.da_m {
                score = score.saturating_add(self.da_big_bonus);
            }
        }
        
        // Risk term (penalty for high knot complexity)
        if k_metric > k_threshold {
            score = score.saturating_sub(self.risk_penalty);
        }
        
        // Cap at 63 (6-bit quantization)
        std::cmp::min(score, 63)
    }
}

/// QGC Consensus State
#[derive(Debug, Clone)]
pub struct QgcConsensusState {
    pub current_round: u64,
    pub current_height: u64,
    pub highest_cc: Option<ConfidenceCertificate>,
    pub pending_batches: HashMap<[u8; 32], Batch>,
    pub ca_accumulator: HashMap<(u64, [u8; 32]), Vec<ConfidenceAttestation>>,
    pub cc_ring: Vec<ConfidenceCertificate>,
    pub knot_window: Vec<KnotMetric>,
    pub scorer: QuantizedScorer,
    pub config: QgcConfig,
}

impl QgcConsensusState {
    pub fn new(config: QgcConfig) -> Self {
        Self {
            current_round: 0,
            current_height: 0,
            highest_cc: None,
            pending_batches: HashMap::new(),
            ca_accumulator: HashMap::new(),
            cc_ring: Vec::with_capacity(128), // Fixed ring buffer
            knot_window: Vec::with_capacity(512), // 512-batch window
            scorer: QuantizedScorer::new(),
            config,
        }
    }
    
    pub fn add_batch(&mut self, batch: Batch) -> bool {
        // Check if batch extends highest CC
        if let Some(ref highest_cc) = self.highest_cc {
            let extends = batch.get_parents().contains(&highest_cc.cid);
            if extends {
                self.pending_batches.insert(batch.id, batch);
                true
            } else {
                false
            }
        } else {
            // Genesis case
            self.pending_batches.insert(batch.id, batch);
            true
        }
    }
    
    pub fn add_ca(&mut self, ca: ConfidenceAttestation) -> Option<ConfidenceCertificate> {
        // Validate CA extends highest CC
        if let Some(ref highest_cc) = self.highest_cc {
            if !self.pending_batches.contains_key(&ca.cid) {
                return None;
            }
        }
        
        let key = (ca.r, ca.cid);
        self.ca_accumulator.entry(key).or_insert_with(Vec::new).push(ca);
        
        // Try to form CC
        if let Some(cas) = self.ca_accumulator.get(&key) {
            let qscore = self.scorer.score_attestations(
                cas,
                self.config.committee_size,
                true, // Simplified: assume two-link
                0,    // Simplified: no knot penalty
                100   // Simplified: high threshold
            );
            
            if qscore >= self.config.threshold_band && cas.len() >= (self.config.committee_size as usize * 2 / 3) {
                // Form CC
                let mut bitmap = 0u32;
                for (i, _) in cas.iter().enumerate().take(32) {
                    bitmap |= 1 << i;
                }
                
                let cc = ConfidenceCertificate {
                    r: key.0,
                    cid: key.1,
                    bitmap,
                    bls_agg: vec![0; 48], // Simplified: would aggregate BLS signatures
                    qscore,
                    da_ratio: 100, // Simplified
                    knot_k: 0,     // Simplified
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                };
                
                // Add to ring buffer
                if self.cc_ring.len() >= 128 {
                    self.cc_ring.remove(0);
                }
                self.cc_ring.push(cc.clone());
                
                return Some(cc);
            }
        }
        
        None
    }
    
    pub fn check_commit(&mut self, cc: ConfidenceCertificate) -> Option<[u8; 32]> {
        // Two-link commit rule
        if let Some(ref highest_cc) = self.highest_cc {
            if cc.r == highest_cc.r + 1 && 
               cc.qscore >= self.config.threshold_band &&
               highest_cc.qscore >= self.config.threshold_band {
                // Check if cc.cid has highest_cc.cid as parent
                if let Some(batch) = self.pending_batches.get(&cc.cid) {
                    if batch.get_parents().contains(&highest_cc.cid) {
                        // Commit highest_cc.cid
                        let commit_id = highest_cc.cid;
                        let new_round = cc.r;
                        self.highest_cc = Some(cc);
                        self.current_round = new_round;
                        return Some(commit_id);
                    }
                }
            }
        }
        
        // Update highest CC if newer
        if self.highest_cc.as_ref().map_or(true, |hcc| cc.r > hcc.r) {
            let new_round = cc.r;
            self.highest_cc = Some(cc);
            self.current_round = new_round;
        }
        
        None
    }
    
    pub fn get_memory_usage(&self) -> usize {
        // Estimate memory usage in bytes
        let batches_mem = self.pending_batches.len() * Batch::size_bytes();
        let ca_mem = self.ca_accumulator.values()
            .map(|v| v.len() * ConfidenceAttestation::size_bytes())
            .sum::<usize>();
        let cc_mem = self.cc_ring.len() * ConfidenceCertificate::size_bytes();
        let knot_mem = self.knot_window.len() * KnotMetric::size_bytes();
        
        batches_mem + ca_mem + cc_mem + knot_mem + 8192 // Base overhead
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_batch_creation() {
        let tx_root = [1u8; 32];
        let maker = [2u8; 32];
        let parents = vec![[3u8; 32], [4u8; 32]];
        
        let batch = Batch::new(tx_root, maker, 1, parents);
        assert_eq!(batch.tx_root, tx_root);
        assert_eq!(batch.maker, maker);
        assert_eq!(batch.strand, 1);
        assert_eq!(batch.parent_count, 2);
        assert_eq!(batch.get_parents().len(), 2);
    }
    
    #[test]
    fn test_quantized_scorer() {
        let scorer = QuantizedScorer::new();
        let cas = vec![]; // Empty for simplicity
        let score = scorer.score_attestations(&cas, 24, true, 0, 100);
        assert!(score <= 63); // Should be capped at 63
    }
    
    #[test]
    fn test_consensus_state() {
        let config = QgcConfig::default();
        let mut state = QgcConsensusState::new(config);
        
        let batch = Batch::new([1u8; 32], [2u8; 32], 1, vec![]);
        assert!(state.add_batch(batch));
        
        let memory_usage = state.get_memory_usage();
        assert!(memory_usage > 0);
        assert!(memory_usage < 1024 * 1024); // Should be under 1MB for empty state
    }
}
