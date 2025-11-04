//! # Highest Random Weight (HRW) / Rendezvous Hashing
//! 
//! Minimal-churn vPod selection with weighted load balancing.
//! Only K/N flows move when adding/removing vPods.

use std::collections::HashMap;
use blake3;
use serde::{Serialize, Deserialize};

/// vPod weight for load balancing
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VPodWeight {
    /// Base weight (1.0 = normal, 2.0 = 2x capacity)
    pub weight: f64,
    
    /// Health status (0.0 = unhealthy, 1.0 = healthy)
    pub health: f64,
}

impl VPodWeight {
    pub fn new(weight: f64, health: f64) -> Self {
        Self { weight, health }
    }
    
    /// Get effective weight (base * health)
    pub fn effective_weight(&self) -> f64 {
        self.weight * self.health
    }
}

impl Default for VPodWeight {
    fn default() -> Self {
        Self {
            weight: 1.0,
            health: 1.0,
        }
    }
}

/// Rendezvous Hasher for vPod selection
#[derive(Debug, Clone)]
pub struct RendezvousHasher {
    /// vPod weights: vpod_id → weight
    vpod_weights: HashMap<String, VPodWeight>,
}

impl RendezvousHasher {
    /// Create new rendezvous hasher
    pub fn new() -> Self {
        Self {
            vpod_weights: HashMap::new(),
        }
    }
    
    /// Add vPod to the ring
    pub fn add_vpod(&mut self, vpod_id: String, weight: VPodWeight) {
        self.vpod_weights.insert(vpod_id, weight);
    }
    
    /// Remove vPod from the ring
    pub fn remove_vpod(&mut self, vpod_id: &str) -> Option<VPodWeight> {
        self.vpod_weights.remove(vpod_id)
    }
    
    /// Update vPod weight
    pub fn update_weight(&mut self, vpod_id: &str, weight: VPodWeight) -> bool {
        if let Some(entry) = self.vpod_weights.get_mut(vpod_id) {
            *entry = weight;
            true
        } else {
            false
        }
    }
    
    /// Select vPod using HRW algorithm
    /// 
    /// # Arguments
    /// 
    /// * `holder` - Holder address
    /// * `service` - Service identifier
    /// * `epoch` - Current epoch
    /// 
    /// # Returns
    /// 
    /// Selected vPod ID, or None if no healthy vPods available
    pub fn select_vpod(&self, holder: &str, service: &str, epoch: u64) -> Option<String> {
        // Construct flow key
        let flow_key = format!("{}||{}||{}", holder, service, epoch);
        
        let mut best_vpod = None;
        let mut best_score = f64::MIN;
        
        for (vpod_id, vpod_weight) in &self.vpod_weights {
            // Skip unhealthy vPods
            if vpod_weight.health < 0.01 {
                continue;
            }
            
            // Compute hash-based score
            let hash_input = format!("{}||{}", flow_key, vpod_id);
            let hash = blake3::hash(hash_input.as_bytes());
            let hash_value = u64::from_le_bytes(
                hash.as_bytes()[0..8].try_into().unwrap()
            );
            
            // Combine with effective weight
            let score = (hash_value as f64) * vpod_weight.effective_weight();
            
            if score > best_score {
                best_score = score;
                best_vpod = Some(vpod_id.clone());
            }
        }
        
        best_vpod
    }
    
    /// Get all vPod IDs
    pub fn vpod_ids(&self) -> Vec<String> {
        self.vpod_weights.keys().cloned().collect()
    }
    
    /// Get vPod count
    pub fn vpod_count(&self) -> usize {
        self.vpod_weights.len()
    }
    
    /// Get healthy vPod count
    pub fn healthy_vpod_count(&self) -> usize {
        self.vpod_weights.values()
            .filter(|w| w.health >= 0.01)
            .count()
    }
}

impl Default for RendezvousHasher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hrw_deterministic() {
        let mut hrw = RendezvousHasher::new();
        hrw.add_vpod("vpod1".to_string(), VPodWeight::default());
        hrw.add_vpod("vpod2".to_string(), VPodWeight::default());
        hrw.add_vpod("vpod3".to_string(), VPodWeight::default());
        
        let selected1 = hrw.select_vpod("holder1", "service1", 1000);
        let selected2 = hrw.select_vpod("holder1", "service1", 1000);
        
        assert_eq!(selected1, selected2);
    }
    
    #[test]
    fn test_hrw_different_flows_different_vpods() {
        let mut hrw = RendezvousHasher::new();
        for i in 0..10 {
            hrw.add_vpod(format!("vpod{}", i), VPodWeight::default());
        }
        
        let mut selections = std::collections::HashSet::new();
        for i in 0..100 {
            let selected = hrw.select_vpod(&format!("holder{}", i), "service1", 1000);
            if let Some(vpod) = selected {
                selections.insert(vpod);
            }
        }
        
        // Should distribute across multiple vPods
        assert!(selections.len() > 1);
    }
    
    #[test]
    fn test_hrw_minimal_churn() {
        let mut hrw = RendezvousHasher::new();
        hrw.add_vpod("vpod1".to_string(), VPodWeight::default());
        hrw.add_vpod("vpod2".to_string(), VPodWeight::default());
        hrw.add_vpod("vpod3".to_string(), VPodWeight::default());
        
        // Select for 100 flows
        let mut selections_before = Vec::new();
        for i in 0..100 {
            let selected = hrw.select_vpod(&format!("holder{}", i), "service1", 1000);
            selections_before.push(selected);
        }
        
        // Add new vPod
        hrw.add_vpod("vpod4".to_string(), VPodWeight::default());
        
        // Select again
        let mut selections_after = Vec::new();
        for i in 0..100 {
            let selected = hrw.select_vpod(&format!("holder{}", i), "service1", 1000);
            selections_after.push(selected);
        }
        
        // Count how many flows moved
        let mut moved = 0;
        for i in 0..100 {
            if selections_before[i] != selections_after[i] {
                moved += 1;
            }
        }
        
        // Should be approximately 25% (1/4 of flows move to new vPod)
        assert!(moved > 10 && moved < 40);
    }
    
    #[test]
    fn test_hrw_weighted() {
        let mut hrw = RendezvousHasher::new();
        hrw.add_vpod("vpod1".to_string(), VPodWeight::new(1.0, 1.0));
        hrw.add_vpod("vpod2".to_string(), VPodWeight::new(2.0, 1.0)); // 2x weight
        
        let mut counts = HashMap::new();
        for i in 0..1000 {
            let selected = hrw.select_vpod(&format!("holder{}", i), "service1", 1000);
            if let Some(vpod) = selected {
                *counts.entry(vpod).or_insert(0) += 1;
            }
        }
        
        // vpod2 should get approximately 2x more flows
        let vpod1_count = counts.get("vpod1").unwrap_or(&0);
        let vpod2_count = counts.get("vpod2").unwrap_or(&0);
        
        assert!(*vpod2_count > *vpod1_count);
    }
    
    #[test]
    fn test_hrw_unhealthy_excluded() {
        let mut hrw = RendezvousHasher::new();
        hrw.add_vpod("vpod1".to_string(), VPodWeight::new(1.0, 1.0));
        hrw.add_vpod("vpod2".to_string(), VPodWeight::new(1.0, 0.0)); // Unhealthy
        
        let selected = hrw.select_vpod("holder1", "service1", 1000);
        
        // Should always select vpod1 (vpod2 is unhealthy)
        assert_eq!(selected, Some("vpod1".to_string()));
    }
}
