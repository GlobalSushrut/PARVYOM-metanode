//! LCCD Slot Selection
//! 
//! Implements curvature-aware slot selection for resource-resident BPCI migration.
//! 
//! # Selection Algorithm
//! 
//! 1. Filter slots by Σ-majorization compliance
//! 2. Score slots by curvature and cell health
//! 3. Apply diversity requirements
//! 4. Select optimal slot using weighted criteria
//! 
//! # Scoring Factors
//! 
//! - **Curvature Score**: Prefer slots in well-connected regions
//! - **Health Score**: Prefer healthy cells
//! - **Diversity Score**: Ensure geographic/jurisdictional diversity
//! - **Performance Score**: Prefer low-latency, high-throughput slots
//! - **Cost Score**: Balance performance with economic efficiency

use super::cell::LccdCell;
use crate::bpi_chain_state::SigmaVector;
use crate::slot_marketplace::{BpiSlotOffer, BpciAllocator};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Slot selection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotSelectionConfig {
    /// Minimum curvature for slot (prefer positive)
    pub min_curvature: f64,
    
    /// Minimum health score for cell
    pub min_health: f64,
    
    /// Require diversity (different jurisdictions)
    pub require_diversity: bool,
    
    /// Maximum cost per epoch (GEN)
    pub max_cost_per_epoch: u64,
    
    /// Selection weights
    pub weights: SelectionWeights,
}

impl Default for SlotSelectionConfig {
    fn default() -> Self {
        Self {
            min_curvature: -0.5, // Allow some negative curvature
            min_health: 0.5,
            require_diversity: true,
            max_cost_per_epoch: 1000,
            weights: SelectionWeights::default(),
        }
    }
}

/// Weights for slot selection scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionWeights {
    pub curvature: f64,
    pub health: f64,
    pub diversity: f64,
    pub performance: f64,
    pub cost: f64,
}

impl Default for SelectionWeights {
    fn default() -> Self {
        Self {
            curvature: 0.25,
            health: 0.25,
            diversity: 0.20,
            performance: 0.20,
            cost: 0.10,
        }
    }
}

/// Slot selection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotSelectionResult {
    /// Selected slot offer
    pub slot_offer: BpiSlotOffer,
    
    /// Selection score (0.0 - 1.0)
    pub score: f64,
    
    /// Individual scores
    pub curvature_score: f64,
    pub health_score: f64,
    pub diversity_score: f64,
    pub performance_score: f64,
    pub cost_score: f64,
}

/// Slot selector for LCCD cells
pub struct SlotSelector {
    config: SlotSelectionConfig,
}

impl SlotSelector {
    /// Create a new slot selector
    pub fn new(config: SlotSelectionConfig) -> Self {
        Self { config }
    }
    
    /// Select optimal slot for a cell
    pub fn select_slot(
        &self,
        cell: &LccdCell,
        available_slots: &[BpiSlotOffer],
        allocator: &BpciAllocator,
        policy_sigma: &SigmaVector,
    ) -> Option<SlotSelectionResult> {
        // Filter by Σ-majorization compliance
        let compliant_slots: Vec<&BpiSlotOffer> = available_slots
            .iter()
            .filter(|slot| {
                // Check if slot's sigma satisfies policy
                slot.sigma.satisfies_policy(policy_sigma)
            })
            .collect();
        
        if compliant_slots.is_empty() {
            return None;
        }
        
        // Filter by health and curvature thresholds
        let viable_slots: Vec<&BpiSlotOffer> = compliant_slots
            .into_iter()
            .filter(|_slot| {
                // Cell must meet minimum health
                cell.health.score >= self.config.min_health
                // Curvature check (using cell's avg internal curvature)
                && cell.curvature_profile.avg_internal_curvature >= self.config.min_curvature
            })
            .collect();
        
        if viable_slots.is_empty() {
            return None;
        }
        
        // Score all viable slots
        let mut scored_slots: Vec<(SlotSelectionResult, &BpiSlotOffer)> = viable_slots
            .iter()
            .map(|slot| {
                let result = self.score_slot(cell, slot);
                (result, *slot)
            })
            .collect();
        
        // Sort by score (descending)
        scored_slots.sort_by(|a, b| b.0.score.partial_cmp(&a.0.score).unwrap());
        
        // Return best slot
        scored_slots.into_iter().next().map(|(result, _)| result)
    }
    
    /// Score a slot for a cell
    fn score_slot(&self, cell: &LccdCell, slot: &BpiSlotOffer) -> SlotSelectionResult {
        let curvature_score = self.calculate_curvature_score(cell);
        let health_score = cell.health.score;
        let diversity_score = self.calculate_diversity_score(slot);
        let performance_score = self.calculate_performance_score(slot);
        let cost_score = self.calculate_cost_score(slot);
        
        // Weighted average
        let score = self.config.weights.curvature * curvature_score
            + self.config.weights.health * health_score
            + self.config.weights.diversity * diversity_score
            + self.config.weights.performance * performance_score
            + self.config.weights.cost * cost_score;
        
        SlotSelectionResult {
            slot_offer: slot.clone(),
            score,
            curvature_score,
            health_score,
            diversity_score,
            performance_score,
            cost_score,
        }
    }
    
    /// Calculate curvature score
    fn calculate_curvature_score(&self, cell: &LccdCell) -> f64 {
        // Positive curvature = good (well-connected)
        // Normalize to 0-1 range
        let curvature = cell.curvature_profile.avg_internal_curvature;
        ((curvature + 1.0) / 2.0).max(0.0).min(1.0)
    }
    
    /// Calculate diversity score
    fn calculate_diversity_score(&self, slot: &BpiSlotOffer) -> f64 {
        // For now, use jurisdiction as diversity metric
        // Higher jurisdiction values = more diverse
        // Normalize to 0-1 range (assuming max jurisdiction ~1000)
        (slot.sigma.jurisdiction as f64 / 1000.0).min(1.0)
    }
    
    /// Calculate performance score
    fn calculate_performance_score(&self, slot: &BpiSlotOffer) -> f64 {
        // Higher resource spec = better performance
        // Normalize based on typical values
        let cpu_score = (slot.resources.cpu_cores as f64 / 16.0).min(1.0);
        let mem_score = (slot.resources.memory_mb as f64 / 64000.0).min(1.0); // MB to GB
        let storage_score = (slot.resources.storage_mb as f64 / 1000000.0).min(1.0); // MB to GB
        
        // Weighted average
        let score = 0.4 * cpu_score + 0.4 * mem_score + 0.2 * storage_score;
        score.max(0.0).min(1.0)
    }
    
    /// Calculate cost score
    fn calculate_cost_score(&self, slot: &BpiSlotOffer) -> f64 {
        // Lower cost = better score
        // Calculate total cost per hour (simplified)
        let total_cost = slot.price.cpu_per_hour * slot.resources.cpu_cores as f64
            + slot.price.mem_gb_per_hour * (slot.resources.memory_mb as f64 / 1024.0);
        
        // Normalize based on max cost (assume 1 epoch = 1 hour for simplicity)
        if self.config.max_cost_per_epoch == 0 {
            return 1.0;
        }
        
        let cost_ratio = total_cost / self.config.max_cost_per_epoch as f64;
        let score = 1.0 - cost_ratio;
        score.max(0.0).min(1.0)
    }
    
    /// Select multiple slots for cell redundancy
    pub fn select_multiple_slots(
        &self,
        cell: &LccdCell,
        available_slots: &[BpiSlotOffer],
        allocator: &BpciAllocator,
        policy_sigma: &SigmaVector,
        count: usize,
    ) -> Vec<SlotSelectionResult> {
        // Filter and score all slots
        let compliant_slots: Vec<&BpiSlotOffer> = available_slots
            .iter()
            .filter(|slot| slot.sigma.satisfies_policy(policy_sigma))
            .collect();
        
        let mut scored_slots: Vec<SlotSelectionResult> = compliant_slots
            .iter()
            .map(|slot| self.score_slot(cell, slot))
            .collect();
        
        // Sort by score
        scored_slots.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        
        // Return top N slots
        scored_slots.into_iter().take(count).collect()
    }
}

/// Diversity analyzer for slot selection
pub struct DiversityAnalyzer;

impl DiversityAnalyzer {
    /// Check if slots provide sufficient diversity
    pub fn check_diversity(slots: &[BpiSlotOffer], min_unique_jurisdictions: usize) -> bool {
        let unique_jurisdictions: std::collections::HashSet<u16> =
            slots.iter().map(|s| s.sigma.jurisdiction).collect();
        
        unique_jurisdictions.len() >= min_unique_jurisdictions
    }
    
    /// Calculate diversity score for a set of slots
    pub fn calculate_diversity_score(slots: &[BpiSlotOffer]) -> f64 {
        if slots.is_empty() {
            return 0.0;
        }
        
        let unique_jurisdictions: std::collections::HashSet<u16> =
            slots.iter().map(|s| s.sigma.jurisdiction).collect();
        
        let unique_data_classes: std::collections::HashSet<u16> =
            slots.iter().map(|s| s.sigma.data_class).collect();
        
        let unique_trust_tiers: std::collections::HashSet<u16> =
            slots.iter().map(|s| s.sigma.trust_tier).collect();
        
        // Diversity = average of unique counts normalized
        let jurisdiction_diversity = unique_jurisdictions.len() as f64 / slots.len() as f64;
        let data_class_diversity = unique_data_classes.len() as f64 / slots.len() as f64;
        let trust_tier_diversity = unique_trust_tiers.len() as f64 / slots.len() as f64;
        
        (jurisdiction_diversity + data_class_diversity + trust_tier_diversity) / 3.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::slot_marketplace::{ResourceSpec, QoSSpec, PriceSpec};
    use crate::lccd::CellHealth;
    use crate::lccd::cell::{CurvatureProfile, CellState};
    
    fn create_test_cell(health_score: f64, curvature: f64) -> LccdCell {
        LccdCell {
            cell_id: 1,
            members: vec![1, 2, 3, 4, 5],
            boundary_edges: Vec::new(),
            curvature_profile: CurvatureProfile {
                avg_internal_curvature: curvature,
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
    
    fn create_test_slot(jurisdiction: u16, cpu_price: f64) -> BpiSlotOffer {
        use chrono::Utc;
        use crate::slot_marketplace::{SlotAttestation, SlotStatus};
        
        BpiSlotOffer {
            slot_id: format!("slot-{}", jurisdiction),
            chain_id: "test-chain".to_string(),
            sigma: SigmaVector::new(60, jurisdiction, 2, 2, 2, 0),
            resources: ResourceSpec {
                cpu_cores: 4,
                memory_mb: 16384,
                storage_mb: 512000,
                network_mbps: 1000,
                max_vpods: 10,
            },
            price: PriceSpec {
                cpu_per_hour: cpu_price,
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
    fn test_slot_selection_config() {
        let config = SlotSelectionConfig::default();
        assert_eq!(config.min_curvature, -0.5);
        assert_eq!(config.min_health, 0.5);
        assert!(config.require_diversity);
    }
    
    #[test]
    fn test_selection_weights() {
        let weights = SelectionWeights::default();
        assert_eq!(weights.curvature, 0.25);
        assert_eq!(weights.health, 0.25);
        assert_eq!(weights.diversity, 0.20);
    }
    
    #[test]
    fn test_slot_selector_creation() {
        let config = SlotSelectionConfig::default();
        let selector = SlotSelector::new(config);
        assert_eq!(selector.config.min_health, 0.5);
    }
    
    #[test]
    fn test_curvature_score() {
        let config = SlotSelectionConfig::default();
        let selector = SlotSelector::new(config);
        
        let cell = create_test_cell(0.8, 0.5);
        let score = selector.calculate_curvature_score(&cell);
        
        assert!(score > 0.5);
        assert!(score <= 1.0);
    }
    
    #[test]
    fn test_diversity_score() {
        let config = SlotSelectionConfig::default();
        let selector = SlotSelector::new(config);
        
        let slot = create_test_slot(840, 0.5);
        let score = selector.calculate_diversity_score(&slot);
        
        assert!(score > 0.0);
        assert!(score <= 1.0);
    }
    
    #[test]
    fn test_performance_score() {
        let config = SlotSelectionConfig::default();
        let selector = SlotSelector::new(config);
        
        let slot = create_test_slot(840, 0.5);
        let score = selector.calculate_performance_score(&slot);
        
        assert!(score > 0.0);
        assert!(score <= 1.0);
    }
    
    #[test]
    fn test_cost_score() {
        let config = SlotSelectionConfig::default();
        let selector = SlotSelector::new(config);
        
        let cheap_slot = create_test_slot(840, 0.1);
        let expensive_slot = create_test_slot(840, 0.9);
        
        let cheap_score = selector.calculate_cost_score(&cheap_slot);
        let expensive_score = selector.calculate_cost_score(&expensive_slot);
        
        assert!(cheap_score > expensive_score);
    }
    
    #[test]
    fn test_diversity_check() {
        let slots = vec![
            create_test_slot(840, 0.5), // USA
            create_test_slot(826, 0.5), // UK
            create_test_slot(276, 0.5), // Germany
        ];
        
        assert!(DiversityAnalyzer::check_diversity(&slots, 3));
        assert!(!DiversityAnalyzer::check_diversity(&slots, 4));
    }
    
    #[test]
    fn test_diversity_score_calculation() {
        let slots = vec![
            create_test_slot(840, 0.5),
            create_test_slot(826, 0.5),
            create_test_slot(276, 0.5),
        ];
        
        let score = DiversityAnalyzer::calculate_diversity_score(&slots);
        assert!(score > 0.0);
        assert!(score <= 1.0);
    }
}
