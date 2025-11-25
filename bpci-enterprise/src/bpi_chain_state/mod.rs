//! BPI Chain State - 6-D State Vector (Σ⃗) Implementation
//! 
//! This module implements the 6-dimensional state space for BPI chains,
//! enabling Σ-majorization based routing, LCCD cell validation, and
//! policy-based path selection.
//! 
//! # Core Concepts
//! 
//! - **Σ⃗ (Sigma Vector)**: 6-dimensional state vector representing chain properties
//! - **Majorization**: Partial order relation (∀i: a[i] ≥ b[i])
//! - **Policy Compliance**: Validation that slots satisfy policy requirements
//! - **Category Matrix**: Routing policy based on category theory
//! 
//! # Usage
//! 
//! ```rust
//! use bpci_enterprise::bpi_chain_state::{SigmaVector, SigmaPolicy, PolicyTemplates};
//! 
//! // Create a sigma vector for a BPI chain
//! let chain_state = SigmaVector::new(
//!     10,  // security_epoch
//!     840, // jurisdiction (USA)
//!     3,   // data_class (PII)
//!     2,   // qos_lane (gold)
//!     2,   // trust_tier (enterprise)
//!     0,   // time_phase
//! );
//! 
//! // Check against a policy
//! let policy = PolicyTemplates::enterprise_internal();
//! assert!(policy.complies(&chain_state));
//! 
//! // Check majorization
//! let slot = SigmaVector::new(15, 840, 4, 2, 2, 0);
//! assert!(slot.majorizes(&chain_state));
//! ```

pub mod sigma_vector;
pub mod majorization;
pub mod policy;

// Re-export main types
pub use sigma_vector::{SigmaVector, QoSLane, TrustTier, DataClass};
pub use majorization::{
    MajorizationChecker, 
    DetailedMajorizationChecker,
    MajorizationResult,
    DimensionViolation,
};
pub use policy::{SigmaPolicy, PolicyTemplates, CategoryMatrix};

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_end_to_end_policy_validation() {
        // Scenario: Healthcare application needs to route through compliant paths
        
        // 1. Define the application's requirements
        let app_sigma = SigmaVector::new(
            50,                          // High security
            840,                         // USA jurisdiction
            DataClass::PHI.as_u16(),     // Healthcare data
            QoSLane::Gold.as_u16(),      // Gold QoS
            TrustTier::Enterprise.as_u16(), // Enterprise trust
            0,
        );
        
        // 2. Get the HIPAA policy
        let hipaa_policy = PolicyTemplates::healthcare_hipaa();
        
        // 3. Verify compliance
        assert!(hipaa_policy.complies(&app_sigma));
        
        // 4. Check available BPI slots
        let slots = vec![
            SigmaVector::new(60, 840, DataClass::PHI.as_u16(), QoSLane::Gold.as_u16(), TrustTier::Enterprise.as_u16(), 0),  // Compliant
            SigmaVector::new(30, 840, DataClass::Internal.as_u16(), QoSLane::Silver.as_u16(), TrustTier::Enterprise.as_u16(), 0),  // Not compliant
            SigmaVector::new(70, 840, DataClass::PHI.as_u16(), QoSLane::Platinum.as_u16(), TrustTier::Government.as_u16(), 0),  // Compliant
        ];
        
        // 5. Filter compliant slots
        let mut checker = MajorizationChecker::new();
        let compliant_slots = checker.filter_majorizing(&slots, &hipaa_policy.minimum);
        
        assert_eq!(compliant_slots.len(), 2);
    }
    
    #[test]
    fn test_lccd_cell_validation() {
        // Scenario: LCCD cell formation requires all backing slots to majorize policy
        
        let cell_policy = SigmaVector::new(
            10,
            0,
            DataClass::Internal.as_u16(),
            QoSLane::Silver.as_u16(),
            TrustTier::Enterprise.as_u16(),
            0,
        );
        
        let backing_slots = vec![
            SigmaVector::new(15, 0, DataClass::Confidential.as_u16(), QoSLane::Gold.as_u16(), TrustTier::Enterprise.as_u16(), 0),
            SigmaVector::new(12, 0, DataClass::Internal.as_u16(), QoSLane::Silver.as_u16(), TrustTier::Enterprise.as_u16(), 0),
            SigmaVector::new(20, 0, DataClass::Confidential.as_u16(), QoSLane::Gold.as_u16(), TrustTier::Enterprise.as_u16(), 0),
        ];
        
        // Check collective majorization
        let result = DetailedMajorizationChecker::check_collective(&backing_slots, &cell_policy);
        
        assert!(result.majorizes, "LCCD cell should be valid");
        assert!(result.violations.is_empty());
    }
    
    #[test]
    fn test_routing_path_selection() {
        // Scenario: Select routing paths that satisfy sigma requirements
        
        let request = SigmaVector::new(
            10,
            0,
            DataClass::Internal.as_u16(),
            QoSLane::Silver.as_u16(),
            TrustTier::Enterprise.as_u16(),
            0,
        );
        
        let available_paths = vec![
            SigmaVector::new(15, 0, DataClass::Internal.as_u16(), QoSLane::Gold.as_u16(), TrustTier::Enterprise.as_u16(), 0),     // Good
            SigmaVector::new(5, 0, DataClass::Public.as_u16(), QoSLane::Bronze.as_u16(), TrustTier::Community.as_u16(), 0),       // Bad
            SigmaVector::new(20, 0, DataClass::Confidential.as_u16(), QoSLane::Platinum.as_u16(), TrustTier::Enterprise.as_u16(), 0), // Good
            SigmaVector::new(8, 0, DataClass::Internal.as_u16(), QoSLane::Bronze.as_u16(), TrustTier::Verified.as_u16(), 0),      // Bad (low security)
        ];
        
        let matrix = CategoryMatrix::new();
        let valid_paths = matrix.filter_allowed_paths(&request, &available_paths);
        
        assert_eq!(valid_paths.len(), 2);
    }
}
