//! Σ-Majorization Logic for LCCD Cell Validation
//! 
//! Implements the majorization relation for 6-D state vectors.
//! Used to validate that LCCD cells inherit BPI slot guarantees.

use super::sigma_vector::SigmaVector;
use std::collections::HashMap;

/// Majorization checker for Σ vectors
pub struct MajorizationChecker {
    /// Cached majorization results for performance
    cache: HashMap<(SigmaVector, SigmaVector), bool>,
}

impl MajorizationChecker {
    /// Create a new majorization checker
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    
    /// Check if `slot` majorizes `policy` (with caching)
    pub fn check(&mut self, slot: &SigmaVector, policy: &SigmaVector) -> bool {
        let key = (*slot, *policy);
        
        if let Some(&result) = self.cache.get(&key) {
            return result;
        }
        
        let result = slot.majorizes(policy);
        self.cache.insert(key, result);
        result
    }
    
    /// Check if all slots in a set majorize a policy
    pub fn check_all(&mut self, slots: &[SigmaVector], policy: &SigmaVector) -> bool {
        slots.iter().all(|slot| self.check(slot, policy))
    }
    
    /// Find slots that majorize a policy
    pub fn filter_majorizing(
        &mut self,
        slots: &[SigmaVector],
        policy: &SigmaVector,
    ) -> Vec<SigmaVector> {
        slots
            .iter()
            .filter(|slot| self.check(slot, policy))
            .copied()
            .collect()
    }
    
    /// Clear the cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
    
    /// Get cache size
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

impl Default for MajorizationChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a majorization check with detailed information
#[derive(Debug, Clone)]
pub struct MajorizationResult {
    pub majorizes: bool,
    pub violations: Vec<DimensionViolation>,
}

/// A dimension that violates majorization
#[derive(Debug, Clone)]
pub struct DimensionViolation {
    pub dimension: usize,
    pub dimension_name: &'static str,
    pub slot_value: u16,
    pub policy_value: u16,
    pub deficit: u16,
}

/// Detailed majorization checker with violation reporting
pub struct DetailedMajorizationChecker;

impl DetailedMajorizationChecker {
    /// Check majorization with detailed violation information
    pub fn check_detailed(slot: &SigmaVector, policy: &SigmaVector) -> MajorizationResult {
        let mut violations = Vec::new();
        
        // Check each dimension
        let dimensions = [
            (0, "security_epoch", slot.security_epoch, policy.security_epoch),
            (1, "jurisdiction", slot.jurisdiction, policy.jurisdiction),
            (2, "data_class", slot.data_class, policy.data_class),
            (3, "qos_lane", slot.qos_lane, policy.qos_lane),
            (4, "trust_tier", slot.trust_tier, policy.trust_tier),
            (5, "time_phase", slot.time_phase, policy.time_phase),
        ];
        
        for (idx, name, slot_val, policy_val) in dimensions {
            if slot_val < policy_val {
                violations.push(DimensionViolation {
                    dimension: idx,
                    dimension_name: name,
                    slot_value: slot_val,
                    policy_value: policy_val,
                    deficit: policy_val - slot_val,
                });
            }
        }
        
        MajorizationResult {
            majorizes: violations.is_empty(),
            violations,
        }
    }
    
    /// Check if a set of slots collectively majorizes a policy
    /// 
    /// This is useful for LCCD cells where multiple slots together
    /// must satisfy the policy requirements.
    pub fn check_collective(
        slots: &[SigmaVector],
        policy: &SigmaVector,
    ) -> MajorizationResult {
        if slots.is_empty() {
            return MajorizationResult {
                majorizes: false,
                violations: vec![],
            };
        }
        
        // For collective majorization, we take the minimum across all slots
        // for each dimension and check if that minimum still majorizes the policy
        let min_sigma = Self::compute_minimum(slots);
        Self::check_detailed(&min_sigma, policy)
    }
    
    /// Compute the minimum sigma vector across a set
    fn compute_minimum(slots: &[SigmaVector]) -> SigmaVector {
        if slots.is_empty() {
            return SigmaVector::default();
        }
        
        let mut min = slots[0];
        
        for slot in &slots[1..] {
            min.security_epoch = min.security_epoch.min(slot.security_epoch);
            min.jurisdiction = min.jurisdiction.min(slot.jurisdiction);
            min.data_class = min.data_class.min(slot.data_class);
            min.qos_lane = min.qos_lane.min(slot.qos_lane);
            min.trust_tier = min.trust_tier.min(slot.trust_tier);
            min.time_phase = min.time_phase.min(slot.time_phase);
        }
        
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_majorization_checker() {
        let mut checker = MajorizationChecker::new();
        
        let slot = SigmaVector::new(10, 5, 3, 2, 2, 1);
        let policy = SigmaVector::new(5, 3, 2, 1, 1, 0);
        
        assert!(checker.check(&slot, &policy));
        assert_eq!(checker.cache_size(), 1);
        
        // Second call should use cache
        assert!(checker.check(&slot, &policy));
        assert_eq!(checker.cache_size(), 1);
    }
    
    #[test]
    fn test_check_all() {
        let mut checker = MajorizationChecker::new();
        
        let slots = vec![
            SigmaVector::new(10, 5, 3, 2, 2, 1),
            SigmaVector::new(8, 4, 3, 2, 2, 1),
            SigmaVector::new(12, 6, 4, 3, 3, 2),
        ];
        
        let policy = SigmaVector::new(5, 3, 2, 1, 1, 0);
        
        assert!(checker.check_all(&slots, &policy));
    }
    
    #[test]
    fn test_filter_majorizing() {
        let mut checker = MajorizationChecker::new();
        
        let slots = vec![
            SigmaVector::new(10, 5, 3, 2, 2, 1),  // Majorizes
            SigmaVector::new(3, 2, 1, 0, 0, 0),   // Does not majorize
            SigmaVector::new(8, 4, 3, 2, 2, 1),   // Majorizes
        ];
        
        let policy = SigmaVector::new(5, 3, 2, 1, 1, 0);
        
        let result = checker.filter_majorizing(&slots, &policy);
        assert_eq!(result.len(), 2);
    }
    
    #[test]
    fn test_detailed_check_success() {
        let slot = SigmaVector::new(10, 5, 3, 2, 2, 1);
        let policy = SigmaVector::new(5, 3, 2, 1, 1, 0);
        
        let result = DetailedMajorizationChecker::check_detailed(&slot, &policy);
        
        assert!(result.majorizes);
        assert!(result.violations.is_empty());
    }
    
    #[test]
    fn test_detailed_check_violations() {
        let slot = SigmaVector::new(3, 2, 1, 2, 2, 1);
        let policy = SigmaVector::new(5, 3, 2, 1, 1, 0);
        
        let result = DetailedMajorizationChecker::check_detailed(&slot, &policy);
        
        assert!(!result.majorizes);
        assert_eq!(result.violations.len(), 2); // security_epoch and jurisdiction
        
        assert_eq!(result.violations[0].dimension, 0);
        assert_eq!(result.violations[0].dimension_name, "security_epoch");
        assert_eq!(result.violations[0].deficit, 2);
        
        assert_eq!(result.violations[1].dimension, 1);
        assert_eq!(result.violations[1].dimension_name, "jurisdiction");
        assert_eq!(result.violations[1].deficit, 1);
    }
    
    #[test]
    fn test_collective_majorization() {
        let slots = vec![
            SigmaVector::new(10, 5, 3, 2, 2, 1),
            SigmaVector::new(8, 6, 4, 3, 3, 2),
            SigmaVector::new(12, 4, 5, 2, 2, 1),
        ];
        
        let policy = SigmaVector::new(5, 3, 2, 1, 1, 0);
        
        let result = DetailedMajorizationChecker::check_collective(&slots, &policy);
        
        // Minimum across slots: [8, 4, 3, 2, 2, 1]
        // This still majorizes policy [5, 3, 2, 1, 1, 0]
        assert!(result.majorizes);
    }
    
    #[test]
    fn test_collective_majorization_failure() {
        let slots = vec![
            SigmaVector::new(10, 5, 3, 2, 2, 1),
            SigmaVector::new(3, 6, 4, 3, 3, 2),  // Low security_epoch
            SigmaVector::new(12, 4, 5, 2, 2, 1),
        ];
        
        let policy = SigmaVector::new(5, 3, 2, 1, 1, 0);
        
        let result = DetailedMajorizationChecker::check_collective(&slots, &policy);
        
        // Minimum across slots: [3, 4, 3, 2, 2, 1]
        // security_epoch (3) < policy (5), so fails
        assert!(!result.majorizes);
        assert_eq!(result.violations.len(), 1);
    }
}
