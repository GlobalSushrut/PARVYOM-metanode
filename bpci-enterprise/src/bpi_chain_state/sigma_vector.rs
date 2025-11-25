//! 6-D State Vector (Σ⃗) Implementation
//! 
//! Represents the 6-dimensional state space of BPI chains:
//! - σ₁: Security epoch
//! - σ₂: Jurisdiction (geo/regulatory)
//! - σ₃: Data classification (PII/PHI/public)
//! - σ₄: QoS lane (gold/silver/bronze)
//! - σ₅: Trust tier (enterprise/verified/community)
//! - σ₆: Time-phase (epoch alignment)

use serde::{Deserialize, Serialize};
use std::fmt;

/// 6-dimensional state vector for BPI chains
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SigmaVector {
    /// σ₁: Security epoch (higher = more secure)
    pub security_epoch: u16,
    
    /// σ₂: Jurisdiction code (ISO 3166-1 numeric or custom)
    pub jurisdiction: u16,
    
    /// σ₃: Data classification level
    /// 0 = public, 1 = internal, 2 = confidential, 3 = PII, 4 = PHI, 5+ = classified
    pub data_class: u16,
    
    /// σ₄: QoS lane
    /// 0 = bronze, 1 = silver, 2 = gold, 3 = platinum
    pub qos_lane: u16,
    
    /// σ₅: Trust tier
    /// 0 = community, 1 = verified, 2 = enterprise, 3 = government
    pub trust_tier: u16,
    
    /// σ₆: Time-phase (epoch alignment)
    pub time_phase: u16,
}

impl SigmaVector {
    /// Create a new sigma vector with all dimensions
    pub fn new(
        security_epoch: u16,
        jurisdiction: u16,
        data_class: u16,
        qos_lane: u16,
        trust_tier: u16,
        time_phase: u16,
    ) -> Self {
        Self {
            security_epoch,
            jurisdiction,
            data_class,
            qos_lane,
            trust_tier,
            time_phase,
        }
    }
    
    /// Create a default sigma vector (minimal requirements)
    pub fn default_minimal() -> Self {
        Self {
            security_epoch: 0,
            jurisdiction: 0,
            data_class: 0,
            qos_lane: 0,
            trust_tier: 0,
            time_phase: 0,
        }
    }
    
    /// Create a high-security sigma vector
    pub fn high_security() -> Self {
        Self {
            security_epoch: 100,
            jurisdiction: 840, // USA
            data_class: 4,     // PHI
            qos_lane: 2,       // gold
            trust_tier: 2,     // enterprise
            time_phase: 0,
        }
    }
    
    /// Check if this sigma vector majorizes another (∀i: self[i] ≥ other[i])
    /// 
    /// Majorization means this vector dominates or equals the other in all dimensions.
    /// Used for LCCD cell validation: cells must majorize their policy requirements.
    pub fn majorizes(&self, other: &SigmaVector) -> bool {
        self.security_epoch >= other.security_epoch
            && self.jurisdiction >= other.jurisdiction
            && self.data_class >= other.data_class
            && self.qos_lane >= other.qos_lane
            && self.trust_tier >= other.trust_tier
            && self.time_phase >= other.time_phase
    }
    
    /// Check if this sigma vector is majorized by another (∀i: self[i] ≤ other[i])
    pub fn majorized_by(&self, other: &SigmaVector) -> bool {
        other.majorizes(self)
    }
    
    /// Calculate Euclidean distance between two sigma vectors
    pub fn distance(&self, other: &SigmaVector) -> f64 {
        let d1 = (self.security_epoch as i32 - other.security_epoch as i32).pow(2);
        let d2 = (self.jurisdiction as i32 - other.jurisdiction as i32).pow(2);
        let d3 = (self.data_class as i32 - other.data_class as i32).pow(2);
        let d4 = (self.qos_lane as i32 - other.qos_lane as i32).pow(2);
        let d5 = (self.trust_tier as i32 - other.trust_tier as i32).pow(2);
        let d6 = (self.time_phase as i32 - other.time_phase as i32).pow(2);
        
        ((d1 + d2 + d3 + d4 + d5 + d6) as f64).sqrt()
    }
    
    /// Calculate Manhattan distance (L1 norm)
    pub fn manhattan_distance(&self, other: &SigmaVector) -> u32 {
        let d1 = (self.security_epoch as i32 - other.security_epoch as i32).abs() as u32;
        let d2 = (self.jurisdiction as i32 - other.jurisdiction as i32).abs() as u32;
        let d3 = (self.data_class as i32 - other.data_class as i32).abs() as u32;
        let d4 = (self.qos_lane as i32 - other.qos_lane as i32).abs() as u32;
        let d5 = (self.trust_tier as i32 - other.trust_tier as i32).abs() as u32;
        let d6 = (self.time_phase as i32 - other.time_phase as i32).abs() as u32;
        
        d1 + d2 + d3 + d4 + d5 + d6
    }
    
    /// Check if two sigma vectors are compatible (within tolerance)
    /// 
    /// Compatible means they can communicate/route through each other.
    /// Uses Manhattan distance with a threshold.
    pub fn compatible(&self, other: &SigmaVector, threshold: u32) -> bool {
        self.manhattan_distance(other) <= threshold
    }
    
    /// Check if this sigma vector satisfies a policy
    /// 
    /// For routing: the slot must majorize the policy requirements
    pub fn satisfies_policy(&self, policy: &SigmaVector) -> bool {
        self.majorizes(policy)
    }
    
    /// Get the dimension value by index (0-5)
    pub fn get_dimension(&self, index: usize) -> Option<u16> {
        match index {
            0 => Some(self.security_epoch),
            1 => Some(self.jurisdiction),
            2 => Some(self.data_class),
            3 => Some(self.qos_lane),
            4 => Some(self.trust_tier),
            5 => Some(self.time_phase),
            _ => None,
        }
    }
    
    /// Set the dimension value by index (0-5)
    pub fn set_dimension(&mut self, index: usize, value: u16) -> bool {
        match index {
            0 => { self.security_epoch = value; true }
            1 => { self.jurisdiction = value; true }
            2 => { self.data_class = value; true }
            3 => { self.qos_lane = value; true }
            4 => { self.trust_tier = value; true }
            5 => { self.time_phase = value; true }
            _ => false,
        }
    }
    
    /// Convert to array representation
    pub fn to_array(&self) -> [u16; 6] {
        [
            self.security_epoch,
            self.jurisdiction,
            self.data_class,
            self.qos_lane,
            self.trust_tier,
            self.time_phase,
        ]
    }
    
    /// Create from array representation
    pub fn from_array(arr: [u16; 6]) -> Self {
        Self {
            security_epoch: arr[0],
            jurisdiction: arr[1],
            data_class: arr[2],
            qos_lane: arr[3],
            trust_tier: arr[4],
            time_phase: arr[5],
        }
    }
}

impl Default for SigmaVector {
    fn default() -> Self {
        Self::default_minimal()
    }
}

impl fmt::Display for SigmaVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Σ⃗[{},{},{},{},{},{}]",
            self.security_epoch,
            self.jurisdiction,
            self.data_class,
            self.qos_lane,
            self.trust_tier,
            self.time_phase
        )
    }
}

/// Named QoS lanes for convenience
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QoSLane {
    Bronze = 0,
    Silver = 1,
    Gold = 2,
    Platinum = 3,
}

impl QoSLane {
    pub fn as_u16(self) -> u16 {
        self as u16
    }
}

/// Named trust tiers for convenience
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustTier {
    Community = 0,
    Verified = 1,
    Enterprise = 2,
    Government = 3,
}

impl TrustTier {
    pub fn as_u16(self) -> u16 {
        self as u16
    }
}

/// Named data classification levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataClass {
    Public = 0,
    Internal = 1,
    Confidential = 2,
    PII = 3,
    PHI = 4,
    Classified = 5,
}

impl DataClass {
    pub fn as_u16(self) -> u16 {
        self as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_majorization() {
        let high = SigmaVector::new(10, 5, 3, 2, 2, 1);
        let low = SigmaVector::new(5, 3, 2, 1, 1, 0);
        
        assert!(high.majorizes(&low));
        assert!(!low.majorizes(&high));
        assert!(low.majorized_by(&high));
    }
    
    #[test]
    fn test_majorization_equal() {
        let a = SigmaVector::new(5, 5, 5, 5, 5, 5);
        let b = SigmaVector::new(5, 5, 5, 5, 5, 5);
        
        assert!(a.majorizes(&b));
        assert!(b.majorizes(&a));
    }
    
    #[test]
    fn test_majorization_partial() {
        let a = SigmaVector::new(10, 3, 5, 2, 2, 1);
        let b = SigmaVector::new(5, 5, 3, 2, 2, 1);
        
        // Neither majorizes the other (a[0] > b[0] but a[1] < b[1])
        assert!(!a.majorizes(&b));
        assert!(!b.majorizes(&a));
    }
    
    #[test]
    fn test_distance() {
        let a = SigmaVector::new(0, 0, 0, 0, 0, 0);
        let b = SigmaVector::new(3, 4, 0, 0, 0, 0);
        
        // 3² + 4² = 9 + 16 = 25, sqrt(25) = 5
        assert_eq!(a.distance(&b), 5.0);
    }
    
    #[test]
    fn test_manhattan_distance() {
        let a = SigmaVector::new(0, 0, 0, 0, 0, 0);
        let b = SigmaVector::new(3, 4, 2, 1, 0, 0);
        
        // 3 + 4 + 2 + 1 = 10
        assert_eq!(a.manhattan_distance(&b), 10);
    }
    
    #[test]
    fn test_compatible() {
        let a = SigmaVector::new(5, 5, 5, 5, 5, 5);
        let b = SigmaVector::new(6, 6, 6, 6, 6, 6);
        
        // Manhattan distance = 6, so compatible with threshold >= 6
        assert!(a.compatible(&b, 10));
        assert!(a.compatible(&b, 6));
        assert!(!a.compatible(&b, 5));
    }
    
    #[test]
    fn test_satisfies_policy() {
        let slot = SigmaVector::new(10, 5, 3, 2, 2, 1);
        let policy = SigmaVector::new(5, 3, 2, 1, 1, 0);
        
        assert!(slot.satisfies_policy(&policy));
        assert!(!policy.satisfies_policy(&slot));
    }
    
    #[test]
    fn test_array_conversion() {
        let sigma = SigmaVector::new(1, 2, 3, 4, 5, 6);
        let arr = sigma.to_array();
        let sigma2 = SigmaVector::from_array(arr);
        
        assert_eq!(sigma, sigma2);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
    
    #[test]
    fn test_display() {
        let sigma = SigmaVector::new(1, 2, 3, 4, 5, 6);
        assert_eq!(format!("{}", sigma), "Σ⃗[1,2,3,4,5,6]");
    }
    
    #[test]
    fn test_dimension_access() {
        let mut sigma = SigmaVector::new(1, 2, 3, 4, 5, 6);
        
        assert_eq!(sigma.get_dimension(0), Some(1));
        assert_eq!(sigma.get_dimension(5), Some(6));
        assert_eq!(sigma.get_dimension(6), None);
        
        assert!(sigma.set_dimension(0, 10));
        assert_eq!(sigma.security_epoch, 10);
        assert!(!sigma.set_dimension(6, 10));
    }
}
