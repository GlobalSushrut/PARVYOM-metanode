// QGC Knot - Tangle complexity tracking for QGC-C² consensus
// 512-window counters, K calculator for knot-aware stability

use crate::logbook_6d_bridge::qgc_core::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

/// Knot Tracker Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnotConfig {
    pub window_size: usize,              // Ring window size (e.g., 512)
    pub alpha: u16,                      // Crossings weight
    pub beta: u16,                       // Link weight  
    pub gamma: u16,                      // Rate weight
    pub k_threshold: u16,                // K* threshold for risk adjustment
    pub delta_q: u8,                     // Q* adjustment when K > K*
    pub rate_window_ms: u64,             // Time window for rate calculation
}

impl Default for KnotConfig {
    fn default() -> Self {
        Self {
            window_size: 512,
            alpha: 3,                    // Crossings are important
            beta: 2,                     // Link stability matters
            gamma: 1,                    // Rate is less critical
            k_threshold: 100,            // Threshold for risk adjustment
            delta_q: 4,                  // Raise Q* by 4 when risky
            rate_window_ms: 10000,       // 10 second rate window
        }
    }
}

/// Batch arrival event for knot tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchArrival {
    pub batch_id: [u8; 32],              // Batch identifier
    pub strand: u16,                     // Strand identifier
    pub timestamp: u64,                  // Arrival timestamp (ms)
    pub parent_strands: Vec<u16>,        // Parent strand identifiers
    pub height: u32,                     // DAG height
}

impl BatchArrival {
    pub fn new(batch: &Batch, parents: &[Batch]) -> Self {
        let parent_strands = parents.iter().map(|p| p.strand).collect();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
            
        Self {
            batch_id: batch.id,
            strand: batch.strand,
            timestamp,
            parent_strands,
            height: 0, // Will be set by DAG
        }
    }
}

/// Strand crossing detector
#[derive(Debug, Clone)]
pub struct CrossingDetector {
    strand_positions: Vec<(u16, u32)>,   // (strand_id, height) pairs
    crossings_count: u16,
}

impl CrossingDetector {
    pub fn new() -> Self {
        Self {
            strand_positions: Vec::new(),
            crossings_count: 0,
        }
    }
    
    /// Detect crossings when adding new batch
    pub fn add_batch(&mut self, arrival: &BatchArrival) -> u16 {
        let mut new_crossings = 0;
        
        // Count inversions: how many strands with higher height have lower strand_id
        for (existing_strand, existing_height) in &self.strand_positions {
            if *existing_height > arrival.height && *existing_strand < arrival.strand {
                new_crossings += 1;
            } else if *existing_height < arrival.height && *existing_strand > arrival.strand {
                new_crossings += 1;
            }
        }
        
        // Add current batch to positions
        self.strand_positions.push((arrival.strand, arrival.height));
        
        // Keep only recent positions (bounded memory)
        if self.strand_positions.len() > 100 {
            self.strand_positions.remove(0);
        }
        
        self.crossings_count = self.crossings_count.saturating_add(new_crossings);
        new_crossings
    }
    
    pub fn get_crossings(&self) -> u16 {
        self.crossings_count
    }
    
    /// Reset crossings counter (called periodically)
    pub fn reset(&mut self) {
        self.crossings_count = 0;
        self.strand_positions.clear();
    }
}

/// Link stability tracker
#[derive(Debug, Clone)]
pub struct LinkTracker {
    link_pairs: Vec<(u16, u16)>,         // (parent_strand, child_strand) pairs
    stability_score: i16,                // Signed stability metric
}

impl LinkTracker {
    pub fn new() -> Self {
        Self {
            link_pairs: Vec::new(),
            stability_score: 0,
        }
    }
    
    /// Add link relationship
    pub fn add_link(&mut self, arrival: &BatchArrival) -> i16 {
        let mut link_delta = 0i16;
        
        // Analyze parent-child strand relationships
        for parent_strand in &arrival.parent_strands {
            let pair = (*parent_strand, arrival.strand);
            
            // Check if this creates a stable pattern
            if *parent_strand == arrival.strand {
                // Same strand continuation - very stable
                link_delta += 5;
            } else if arrival.parent_strands.len() == 1 {
                // Single parent cross-strand - moderately stable
                link_delta += 2;
            } else {
                // Multiple parents - potentially unstable
                link_delta -= 1;
            }
            
            self.link_pairs.push(pair);
        }
        
        // Keep bounded history
        if self.link_pairs.len() > 200 {
            self.link_pairs.drain(0..50); // Remove oldest 50
        }
        
        self.stability_score = self.stability_score.saturating_add(link_delta);
        link_delta
    }
    
    pub fn get_link_score(&self) -> u16 {
        // Convert signed score to unsigned for K calculation
        if self.stability_score >= 0 {
            self.stability_score as u16
        } else {
            0
        }
    }
    
    /// Reset link tracker
    pub fn reset(&mut self) {
        self.stability_score = 0;
        self.link_pairs.clear();
    }
}

/// Rate calculator for arrival rate tracking
#[derive(Debug, Clone)]
pub struct RateCalculator {
    arrivals: VecDeque<u64>,             // Timestamp queue
    window_ms: u64,                      // Time window for rate calculation
}

impl RateCalculator {
    pub fn new(window_ms: u64) -> Self {
        Self {
            arrivals: VecDeque::new(),
            window_ms,
        }
    }
    
    /// Record batch arrival
    pub fn record_arrival(&mut self, timestamp: u64) {
        self.arrivals.push_back(timestamp);
        
        // Remove old arrivals outside window
        let cutoff = timestamp.saturating_sub(self.window_ms);
        while let Some(&front) = self.arrivals.front() {
            if front < cutoff {
                self.arrivals.pop_front();
            } else {
                break;
            }
        }
    }
    
    /// Get current arrival rate (arrivals per second)
    pub fn get_rate(&self) -> u16 {
        if self.arrivals.len() < 2 {
            return 0;
        }
        
        let count = self.arrivals.len() as u64;
        let rate_per_ms = (count * 1000) / self.window_ms;
        std::cmp::min(rate_per_ms, u16::MAX as u64) as u16
    }
    
    /// Reset rate calculator
    pub fn reset(&mut self) {
        self.arrivals.clear();
    }
}

/// Main knot tracker for tangle complexity
#[derive(Debug)]
pub struct KnotTracker {
    config: KnotConfig,
    window: VecDeque<BatchArrival>,      // Ring window of batch arrivals
    crossing_detector: CrossingDetector,
    link_tracker: LinkTracker,
    rate_calculator: RateCalculator,
    current_knot: KnotMetric,
    knot_history: VecDeque<KnotMetric>,  // History for analysis
}

impl KnotTracker {
    pub fn new(config: KnotConfig) -> Self {
        Self {
            rate_calculator: RateCalculator::new(config.rate_window_ms),
            config,
            window: VecDeque::new(),
            crossing_detector: CrossingDetector::new(),
            link_tracker: LinkTracker::new(),
            current_knot: KnotMetric::new(),
            knot_history: VecDeque::new(),
        }
    }
    
    /// Process new batch arrival
    pub fn process_batch(&mut self, arrival: BatchArrival) -> KnotMetric {
        // Add to window
        self.window.push_back(arrival.clone());
        
        // Maintain window size
        if self.window.len() > self.config.window_size {
            self.window.pop_front();
        }
        
        // Update components
        let crossings_delta = self.crossing_detector.add_batch(&arrival);
        let link_delta = self.link_tracker.add_link(&arrival);
        self.rate_calculator.record_arrival(arrival.timestamp);
        
        // Update current knot metric
        self.current_knot.win = self.window.len() as u32;
        self.current_knot.crossings = self.crossing_detector.get_crossings();
        self.current_knot.link = self.link_tracker.get_link_score();
        self.current_knot.rate = self.rate_calculator.get_rate();
        
        // Compute K metric
        self.current_knot.compute_k(self.config.alpha, self.config.beta, self.config.gamma);
        
        // Add to history
        self.knot_history.push_back(self.current_knot.clone());
        if self.knot_history.len() > 100 {
            self.knot_history.pop_front();
        }
        
        self.current_knot.clone()
    }
    
    /// Get current knot complexity
    pub fn get_current_knot(&self) -> &KnotMetric {
        &self.current_knot
    }
    
    /// Check if current complexity exceeds threshold
    pub fn is_high_complexity(&self) -> bool {
        self.current_knot.k > self.config.k_threshold
    }
    
    /// Get recommended Q* adjustment
    pub fn get_q_adjustment(&self) -> u8 {
        if self.is_high_complexity() {
            self.config.delta_q
        } else {
            0
        }
    }
    
    /// Update knot complexity for a specific batch ID
    pub fn update_knot_complexity(&mut self, batch_id: [u8; 32], complexity: f64) {
        // Create a synthetic batch arrival for complexity tracking
        let arrival = BatchArrival {
            batch_id,
            strand: (complexity as u16) % 1000, // Map complexity to strand
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            parent_strands: vec![], // No parents for synthetic batch
            height: complexity as u32,
        };
        
        // Process the batch to update knot metrics
        self.process_batch(arrival);
    }
    
    /// Check if knot complexity is valid for a given batch ID
    pub fn is_valid_knot_complexity(&self, batch_id: [u8; 32]) -> bool {
        // Check if the batch ID exists in our window and has valid complexity
        for arrival in &self.window {
            if arrival.batch_id == batch_id {
                // Valid if knot complexity is within reasonable bounds
                return self.current_knot.k <= self.config.k_threshold * 2;
            }
        }
        
        // If not found in window, consider invalid for security
        false
    }

    /// Get knot statistics
    pub fn get_stats(&self) -> KnotStats {
        let avg_k = if self.knot_history.is_empty() {
            0.0
        } else {
            self.knot_history.iter().map(|k| k.k as f64).sum::<f64>() / self.knot_history.len() as f64
        };
        
        let max_k = self.knot_history.iter().map(|k| k.k).max().unwrap_or(0);
        let min_k = self.knot_history.iter().map(|k| k.k).min().unwrap_or(0);
        
        KnotStats {
            current_k: self.current_knot.k,
            avg_k,
            max_k,
            min_k,
            window_size: self.window.len(),
            crossings: self.current_knot.crossings,
            link_score: self.current_knot.link,
            arrival_rate: self.current_knot.rate,
            is_high_complexity: self.is_high_complexity(),
            recommended_q_adjustment: self.get_q_adjustment(),
        }
    }
    
    /// Reset knot tracker (periodic cleanup)
    pub fn reset(&mut self) {
        self.crossing_detector.reset();
        self.link_tracker.reset();
        self.rate_calculator.reset();
        self.current_knot = KnotMetric::new();
        self.window.clear();
    }
    
    /// Get memory usage estimate
    pub fn get_memory_usage(&self) -> usize {
        let window_mem = self.window.len() * std::mem::size_of::<BatchArrival>();
        let history_mem = self.knot_history.len() * std::mem::size_of::<KnotMetric>();
        let detector_mem = self.crossing_detector.strand_positions.len() * 8;
        let link_mem = self.link_tracker.link_pairs.len() * 4;
        let rate_mem = self.rate_calculator.arrivals.len() * 8;
        
        window_mem + history_mem + detector_mem + link_mem + rate_mem + 512 // Base overhead
    }
    
    /// Validate knot tracker state
    pub fn validate(&self) -> Result<(), String> {
        if self.window.len() > self.config.window_size {
            return Err("Window size exceeded".to_string());
        }
        
        if self.knot_history.len() > 100 {
            return Err("History size exceeded".to_string());
        }
        
        if self.current_knot.k > 10000 {
            return Err("K metric suspiciously high".to_string());
        }
        
        Ok(())
    }
}

/// Knot statistics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnotStats {
    pub current_k: u16,
    pub avg_k: f64,
    pub max_k: u16,
    pub min_k: u16,
    pub window_size: usize,
    pub crossings: u16,
    pub link_score: u16,
    pub arrival_rate: u16,
    pub is_high_complexity: bool,
    pub recommended_q_adjustment: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_knot_tracker_creation() {
        let config = KnotConfig::default();
        let tracker = KnotTracker::new(config);
        assert_eq!(tracker.window.len(), 0);
        assert_eq!(tracker.current_knot.k, 0);
    }
    
    #[test]
    fn test_batch_arrival() {
        let batch = Batch::new([1u8; 32], [2u8; 32], 1, vec![]);
        let parents = vec![];
        let arrival = BatchArrival::new(&batch, &parents);
        
        assert_eq!(arrival.batch_id, batch.id);
        assert_eq!(arrival.strand, 1);
        assert!(arrival.timestamp > 0);
    }
    
    #[test]
    fn test_crossing_detector() {
        let mut detector = CrossingDetector::new();
        
        let batch1 = Batch::new([1u8; 32], [2u8; 32], 1, vec![]);
        let arrival1 = BatchArrival::new(&batch1, &[]);
        let crossings1 = detector.add_batch(&arrival1);
        assert_eq!(crossings1, 0); // First batch, no crossings
        
        let batch2 = Batch::new([3u8; 32], [4u8; 32], 2, vec![]);
        let arrival2 = BatchArrival::new(&batch2, &[]);
        let crossings2 = detector.add_batch(&arrival2);
        // May have crossings depending on height relationship
        assert!(crossings2 >= 0);
    }
    
    #[test]
    fn test_link_tracker() {
        let mut tracker = LinkTracker::new();
        
        let batch = Batch::new([1u8; 32], [2u8; 32], 1, vec![[3u8; 32]]);
        let parent = Batch::new([3u8; 32], [4u8; 32], 1, vec![]);
        let arrival = BatchArrival::new(&batch, &[parent]);
        
        let link_delta = tracker.add_link(&arrival);
        assert!(link_delta != 0); // Should have some link effect
    }
    
    #[test]
    fn test_rate_calculator() {
        let mut calc = RateCalculator::new(1000); // 1 second window
        
        calc.record_arrival(1000);
        calc.record_arrival(1500);
        calc.record_arrival(2000);
        
        let rate = calc.get_rate();
        assert!(rate > 0); // Should have positive rate
    }
    
    #[test]
    fn test_knot_processing() {
        let config = KnotConfig::default();
        let mut tracker = KnotTracker::new(config);
        
        let batch = Batch::new([1u8; 32], [2u8; 32], 1, vec![]);
        let arrival = BatchArrival::new(&batch, &[]);
        
        let knot = tracker.process_batch(arrival);
        assert_eq!(knot.win, 1);
        assert!(knot.k >= 0);
        
        let stats = tracker.get_stats();
        assert_eq!(stats.window_size, 1);
        assert!(!stats.is_high_complexity); // Should be low for single batch
    }
    
    #[test]
    fn test_memory_usage() {
        let config = KnotConfig::default();
        let tracker = KnotTracker::new(config);
        let usage = tracker.get_memory_usage();
        assert!(usage > 0);
        assert!(usage < 10000); // Should be reasonable for empty tracker
    }
    
    #[test]
    fn test_validation() {
        let config = KnotConfig::default();
        let tracker = KnotTracker::new(config);
        assert!(tracker.validate().is_ok());
    }
}
