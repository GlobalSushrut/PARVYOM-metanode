//! Policy Compliance and Validation
//! 
//! Defines policies for BPI chains and validates compliance.

use super::sigma_vector::{SigmaVector, DataClass, QoSLane, TrustTier};
use serde::{Deserialize, Serialize};

/// Policy requirements for LCCD cells or routing paths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigmaPolicy {
    /// Minimum required sigma vector
    pub minimum: SigmaVector,
    
    /// Policy name for identification
    pub name: String,
    
    /// Description of the policy
    pub description: String,
    
    /// Whether this policy is strict (all dimensions must match exactly)
    pub strict: bool,
}

impl SigmaPolicy {
    /// Create a new policy
    pub fn new(name: String, description: String, minimum: SigmaVector) -> Self {
        Self {
            minimum,
            name,
            description,
            strict: false,
        }
    }
    
    /// Create a strict policy (exact match required)
    pub fn new_strict(name: String, description: String, minimum: SigmaVector) -> Self {
        Self {
            minimum,
            name,
            description,
            strict: true,
        }
    }
    
    /// Check if a sigma vector complies with this policy
    pub fn complies(&self, sigma: &SigmaVector) -> bool {
        if self.strict {
            sigma == &self.minimum
        } else {
            sigma.majorizes(&self.minimum)
        }
    }
    
    /// Check if a sigma vector violates this policy
    pub fn violates(&self, sigma: &SigmaVector) -> bool {
        !self.complies(sigma)
    }
}

/// Pre-defined policy templates
pub struct PolicyTemplates;

impl PolicyTemplates {
    /// Public internet policy (minimal requirements)
    pub fn public_internet() -> SigmaPolicy {
        SigmaPolicy::new(
            "public_internet".to_string(),
            "Public internet access with minimal security".to_string(),
            SigmaVector::new(0, 0, 0, 0, 0, 0),
        )
    }
    
    /// Enterprise internal policy
    pub fn enterprise_internal() -> SigmaPolicy {
        SigmaPolicy::new(
            "enterprise_internal".to_string(),
            "Enterprise internal network with standard security".to_string(),
            SigmaVector::new(
                10,                          // security_epoch
                0,                           // jurisdiction (any)
                DataClass::Internal.as_u16(), // internal data
                QoSLane::Silver.as_u16(),    // silver QoS
                TrustTier::Enterprise.as_u16(), // enterprise trust
                0,                           // time_phase
            ),
        )
    }
    
    /// Healthcare (HIPAA) policy
    pub fn healthcare_hipaa() -> SigmaPolicy {
        SigmaPolicy::new(
            "healthcare_hipaa".to_string(),
            "HIPAA-compliant healthcare data handling".to_string(),
            SigmaVector::new(
                50,                          // high security
                840,                         // USA jurisdiction
                DataClass::PHI.as_u16(),     // PHI data
                QoSLane::Gold.as_u16(),      // gold QoS
                TrustTier::Enterprise.as_u16(), // enterprise trust
                0,
            ),
        )
    }
    
    /// Financial services policy
    pub fn financial_services() -> SigmaPolicy {
        SigmaPolicy::new(
            "financial_services".to_string(),
            "Financial services with PCI-DSS compliance".to_string(),
            SigmaVector::new(
                60,                          // very high security
                0,                           // jurisdiction varies
                DataClass::Confidential.as_u16(), // confidential
                QoSLane::Gold.as_u16(),      // gold QoS
                TrustTier::Enterprise.as_u16(), // enterprise trust
                0,
            ),
        )
    }
    
    /// Government/classified policy
    pub fn government_classified() -> SigmaPolicy {
        SigmaPolicy::new(
            "government_classified".to_string(),
            "Government classified data handling".to_string(),
            SigmaVector::new(
                100,                         // maximum security
                840,                         // USA jurisdiction
                DataClass::Classified.as_u16(), // classified
                QoSLane::Platinum.as_u16(),  // platinum QoS
                TrustTier::Government.as_u16(), // government trust
                0,
            ),
        )
    }
}

/// Category matrix for routing policy
/// 
/// Defines which sigma vectors can route through which paths.
/// Based on category theory composition.
#[derive(Debug, Clone)]
pub struct CategoryMatrix {
    policies: Vec<SigmaPolicy>,
}

impl CategoryMatrix {
    /// Create a new category matrix
    pub fn new() -> Self {
        Self {
            policies: Vec::new(),
        }
    }
    
    /// Add a policy to the matrix
    pub fn add_policy(&mut self, policy: SigmaPolicy) {
        self.policies.push(policy);
    }
    
    /// Check if a path (edge) is allowed for a given sigma vector
    pub fn allows_path(&self, sigma: &SigmaVector, path_sigma: &SigmaVector) -> bool {
        // Path must majorize the request's requirements
        path_sigma.majorizes(sigma)
    }
    
    /// Filter paths that are allowed for a sigma vector
    pub fn filter_allowed_paths(
        &self,
        sigma: &SigmaVector,
        paths: &[SigmaVector],
    ) -> Vec<SigmaVector> {
        paths
            .iter()
            .filter(|path| self.allows_path(sigma, path))
            .copied()
            .collect()
    }
    
    /// Check if two sigma vectors are compatible for routing
    pub fn compatible_for_routing(&self, a: &SigmaVector, b: &SigmaVector) -> bool {
        // For routing, we use a more lenient compatibility check
        // Healthcare data cannot traverse public paths, etc.
        
        // If either has classified data, they must match exactly
        if a.data_class >= DataClass::Classified.as_u16() 
            || b.data_class >= DataClass::Classified.as_u16() {
            return a.data_class == b.data_class;
        }
        
        // Otherwise, use distance-based compatibility
        a.compatible(b, 10)
    }
}

impl Default for CategoryMatrix {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_policy_compliance() {
        let policy = PolicyTemplates::enterprise_internal();
        
        let compliant = SigmaVector::new(
            15,                          // Higher security
            0,
            DataClass::Internal.as_u16(),
            QoSLane::Gold.as_u16(),      // Better QoS
            TrustTier::Enterprise.as_u16(),
            0,
        );
        
        let non_compliant = SigmaVector::new(
            5,                           // Lower security
            0,
            DataClass::Public.as_u16(),  // Lower data class
            QoSLane::Bronze.as_u16(),
            TrustTier::Community.as_u16(),
            0,
        );
        
        assert!(policy.complies(&compliant));
        assert!(!policy.complies(&non_compliant));
    }
    
    #[test]
    fn test_strict_policy() {
        let sigma = SigmaVector::new(10, 5, 3, 2, 2, 1);
        let policy = SigmaPolicy::new_strict(
            "test".to_string(),
            "test".to_string(),
            sigma,
        );
        
        assert!(policy.complies(&sigma));
        
        let different = SigmaVector::new(11, 5, 3, 2, 2, 1);
        assert!(!policy.complies(&different));
    }
    
    #[test]
    fn test_healthcare_policy() {
        let policy = PolicyTemplates::healthcare_hipaa();
        
        let compliant = SigmaVector::new(
            60,
            840,
            DataClass::PHI.as_u16(),
            QoSLane::Gold.as_u16(),
            TrustTier::Enterprise.as_u16(),
            0,
        );
        
        assert!(policy.complies(&compliant));
    }
    
    #[test]
    fn test_category_matrix_path_filtering() {
        let matrix = CategoryMatrix::new();
        
        let request = SigmaVector::new(
            10,
            0,
            DataClass::Internal.as_u16(),
            QoSLane::Silver.as_u16(),
            TrustTier::Enterprise.as_u16(),
            0,
        );
        
        let paths = vec![
            SigmaVector::new(15, 0, DataClass::Internal.as_u16(), QoSLane::Gold.as_u16(), TrustTier::Enterprise.as_u16(), 0),  // Good
            SigmaVector::new(5, 0, DataClass::Public.as_u16(), QoSLane::Bronze.as_u16(), TrustTier::Community.as_u16(), 0),    // Bad
            SigmaVector::new(20, 0, DataClass::Confidential.as_u16(), QoSLane::Gold.as_u16(), TrustTier::Enterprise.as_u16(), 0),  // Good
        ];
        
        let allowed = matrix.filter_allowed_paths(&request, &paths);
        assert_eq!(allowed.len(), 2);
    }
    
    #[test]
    fn test_routing_compatibility() {
        let matrix = CategoryMatrix::new();
        
        let a = SigmaVector::new(10, 0, DataClass::Internal.as_u16(), QoSLane::Silver.as_u16(), TrustTier::Enterprise.as_u16(), 0);
        let b = SigmaVector::new(12, 0, DataClass::Internal.as_u16(), QoSLane::Gold.as_u16(), TrustTier::Enterprise.as_u16(), 0);
        
        assert!(matrix.compatible_for_routing(&a, &b));
    }
    
    #[test]
    fn test_classified_routing_restriction() {
        let matrix = CategoryMatrix::new();
        
        let classified = SigmaVector::new(100, 840, DataClass::Classified.as_u16(), QoSLane::Platinum.as_u16(), TrustTier::Government.as_u16(), 0);
        let public = SigmaVector::new(0, 0, DataClass::Public.as_u16(), QoSLane::Bronze.as_u16(), TrustTier::Community.as_u16(), 0);
        
        assert!(!matrix.compatible_for_routing(&classified, &public));
    }
}
