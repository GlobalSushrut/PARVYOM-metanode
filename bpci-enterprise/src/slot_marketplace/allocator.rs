//! BPCI Slot Allocator
//! 
//! Implements the allocator that selects BPI slots for BPCI shards using:
//! - HRW (Highest Random Weight) for deterministic selection
//! - Σ-majorization for policy compliance
//! - Diversity checking for fault tolerance
//! - QoS and resource matching

use crate::bpi_chain_state::{SigmaVector, MajorizationChecker};
use super::offer::{BpiSlotOffer, ResourceSpec, QoSSpec};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

/// BPCI Allocator for slot selection
pub struct BpciAllocator {
    /// Slot registry
    registry: Arc<RwLock<HashMap<String, BpiSlotOffer>>>,
    
    /// HRW salt for deterministic hashing
    hrw_salt: String,
    
    /// Majorization checker (with caching)
    majorization_checker: Arc<RwLock<MajorizationChecker>>,
    
    /// Configuration
    config: AllocatorConfig,
}

/// Allocator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocatorConfig {
    /// Prefer Σ-majorization over other factors
    pub prefer_sigma_majorization: bool,
    
    /// Maximum rebalancing per epoch (0.0 - 1.0)
    pub rebal_limit_per_epoch: f64,
    
    /// Minimum TEE percentage required
    pub min_tee_pct: f64,
    
    /// Minimum PoE quality required
    pub min_poe_quality: f64,
    
    /// Require diversity across fault domains
    pub require_diversity: bool,
    
    /// Minimum diversity (k-of-n)
    pub min_diversity_k: usize,
}

/// Allocation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationRequest {
    /// Request ID
    pub request_id: String,
    
    /// Policy requirements (Σ vector)
    pub policy: SigmaVector,
    
    /// Resource requirements
    pub resources: ResourceSpec,
    
    /// QoS requirements
    pub qos: QoSSpec,
    
    /// Number of slots needed
    pub slot_count: usize,
    
    /// Maximum price per hour
    pub max_price_per_hour: Option<f64>,
    
    /// Preferred jurisdictions
    pub preferred_jurisdictions: Vec<u16>,
    
    /// Required attestation types
    pub require_tee: bool,
}

/// Allocation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationResult {
    /// Request ID
    pub request_id: String,
    
    /// Selected slots
    pub slots: Vec<BpiSlotOffer>,
    
    /// Total price per hour
    pub total_price_per_hour: f64,
    
    /// Selection metadata
    pub metadata: AllocationMetadata,
}

/// Allocation metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationMetadata {
    /// Number of candidates evaluated
    pub candidates_evaluated: usize,
    
    /// Number passing Σ-majorization
    pub sigma_compliant: usize,
    
    /// Number passing resource requirements
    pub resource_compliant: usize,
    
    /// Number passing QoS requirements
    pub qos_compliant: usize,
    
    /// Diversity score (0.0 - 1.0)
    pub diversity_score: f64,
    
    /// Average PoE quality
    pub avg_poe_quality: f64,
}

impl BpciAllocator {
    /// Create a new allocator
    pub fn new(hrw_salt: String, config: AllocatorConfig) -> Self {
        Self {
            registry: Arc::new(RwLock::new(HashMap::new())),
            hrw_salt,
            majorization_checker: Arc::new(RwLock::new(MajorizationChecker::new())),
            config,
        }
    }
    
    /// Register a slot offer
    pub async fn register_slot(&self, offer: BpiSlotOffer) -> Result<()> {
        let mut registry = self.registry.write().await;
        registry.insert(offer.slot_id.clone(), offer);
        Ok(())
    }
    
    /// Unregister a slot
    pub async fn unregister_slot(&self, slot_id: &str) -> Result<()> {
        let mut registry = self.registry.write().await;
        registry.remove(slot_id);
        Ok(())
    }
    
    /// Get all available slots
    pub async fn get_available_slots(&self) -> Vec<BpiSlotOffer> {
        let registry = self.registry.read().await;
        registry
            .values()
            .filter(|offer| offer.is_valid())
            .cloned()
            .collect()
    }
    
    /// Allocate slots for a request
    pub async fn allocate(&self, request: AllocationRequest) -> Result<AllocationResult> {
        let available = self.get_available_slots().await;
        
        if available.is_empty() {
            return Err(anyhow!("No available slots"));
        }
        
        // Step 1: Filter by Σ-majorization
        let sigma_compliant = self.filter_sigma_compliant(&available, &request.policy).await;
        
        // Step 2: Filter by resource requirements
        let resource_compliant = self.filter_resource_compliant(&sigma_compliant, &request.resources);
        
        // Step 3: Filter by QoS requirements
        let qos_compliant = self.filter_qos_compliant(&resource_compliant, &request.qos);
        let qos_compliant_count = qos_compliant.len();
        
        // Step 4: Filter by attestation requirements
        let mut candidates = if request.require_tee {
            self.filter_tee_compliant(&qos_compliant)
        } else {
            qos_compliant
        };
        
        // Step 5: Filter by PoE quality
        candidates = self.filter_poe_quality(&candidates, self.config.min_poe_quality);
        
        // Step 6: Filter by price if specified
        if let Some(max_price) = request.max_price_per_hour {
            candidates = self.filter_by_price(&candidates, max_price);
        }
        
        if candidates.len() < request.slot_count {
            return Err(anyhow!(
                "Insufficient slots: need {}, found {}",
                request.slot_count,
                candidates.len()
            ));
        }
        
        // Step 7: Select using HRW
        let selected = self.select_hrw(&candidates, &request.request_id, request.slot_count);
        
        // Step 8: Check diversity if required
        if self.config.require_diversity {
            let diversity_score = self.calculate_diversity(&selected);
            if diversity_score < 0.5 {
                return Err(anyhow!("Insufficient diversity: {}", diversity_score));
            }
        }
        
        // Calculate total price
        let total_price = selected
            .iter()
            .map(|s| s.calculate_price(1.0))
            .sum();
        
        // Build metadata
        let metadata = AllocationMetadata {
            candidates_evaluated: available.len(),
            sigma_compliant: sigma_compliant.len(),
            resource_compliant: resource_compliant.len(),
            qos_compliant: qos_compliant_count,
            diversity_score: self.calculate_diversity(&selected),
            avg_poe_quality: selected.iter().map(|s| s.attestation.poe_quality).sum::<f64>() / selected.len() as f64,
        };
        
        Ok(AllocationResult {
            request_id: request.request_id,
            slots: selected,
            total_price_per_hour: total_price,
            metadata,
        })
    }
    
    /// Filter slots by Σ-majorization
    async fn filter_sigma_compliant(
        &self,
        slots: &[BpiSlotOffer],
        policy: &SigmaVector,
    ) -> Vec<BpiSlotOffer> {
        let mut checker = self.majorization_checker.write().await;
        slots
            .iter()
            .filter(|slot| checker.check(&slot.sigma, policy))
            .cloned()
            .collect()
    }
    
    /// Filter slots by resource requirements
    fn filter_resource_compliant(
        &self,
        slots: &[BpiSlotOffer],
        requirements: &ResourceSpec,
    ) -> Vec<BpiSlotOffer> {
        slots
            .iter()
            .filter(|slot| slot.resources.satisfies(requirements))
            .cloned()
            .collect()
    }
    
    /// Filter slots by QoS requirements
    fn filter_qos_compliant(
        &self,
        slots: &[BpiSlotOffer],
        requirements: &QoSSpec,
    ) -> Vec<BpiSlotOffer> {
        slots
            .iter()
            .filter(|slot| slot.qos.meets(requirements))
            .cloned()
            .collect()
    }
    
    /// Filter slots by TEE requirement
    fn filter_tee_compliant(&self, slots: &[BpiSlotOffer]) -> Vec<BpiSlotOffer> {
        slots
            .iter()
            .filter(|slot| slot.has_tee())
            .cloned()
            .collect()
    }
    
    /// Filter slots by PoE quality
    fn filter_poe_quality(&self, slots: &[BpiSlotOffer], min_quality: f64) -> Vec<BpiSlotOffer> {
        slots
            .iter()
            .filter(|slot| slot.meets_poe_threshold(min_quality))
            .cloned()
            .collect()
    }
    
    /// Filter slots by price
    fn filter_by_price(&self, slots: &[BpiSlotOffer], max_price: f64) -> Vec<BpiSlotOffer> {
        slots
            .iter()
            .filter(|slot| slot.calculate_price(1.0) <= max_price)
            .cloned()
            .collect()
    }
    
    /// Select slots using HRW (Highest Random Weight)
    fn select_hrw(
        &self,
        candidates: &[BpiSlotOffer],
        key: &str,
        count: usize,
    ) -> Vec<BpiSlotOffer> {
        let mut scored: Vec<_> = candidates
            .iter()
            .map(|slot| {
                let score = self.hrw_score(key, slot);
                (score, slot.clone())
            })
            .collect();
        
        // Sort by score descending
        scored.sort_by(|a, b| b.0.cmp(&a.0));
        
        // Take top N
        scored
            .into_iter()
            .take(count)
            .map(|(_, slot)| slot)
            .collect()
    }
    
    /// Calculate HRW score for a slot
    fn hrw_score(&self, key: &str, slot: &BpiSlotOffer) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        self.hrw_salt.hash(&mut hasher);
        key.hash(&mut hasher);
        slot.slot_id.hash(&mut hasher);
        
        hasher.finish()
    }
    
    /// Calculate diversity score for selected slots
    fn calculate_diversity(&self, slots: &[BpiSlotOffer]) -> f64 {
        if slots.is_empty() {
            return 0.0;
        }
        
        // Count unique jurisdictions
        let unique_jurisdictions: HashSet<_> = slots
            .iter()
            .map(|s| s.sigma.jurisdiction)
            .collect();
        
        // Count unique chains
        let unique_chains: HashSet<_> = slots
            .iter()
            .map(|s| s.chain_id.clone())
            .collect();
        
        // Diversity score: average of jurisdiction and chain diversity
        let jurisdiction_diversity = unique_jurisdictions.len() as f64 / slots.len() as f64;
        let chain_diversity = unique_chains.len() as f64 / slots.len() as f64;
        
        (jurisdiction_diversity + chain_diversity) / 2.0
    }
    
    /// Get slot by ID
    pub async fn get_slot(&self, slot_id: &str) -> Option<BpiSlotOffer> {
        let registry = self.registry.read().await;
        registry.get(slot_id).cloned()
    }
    
    /// Mark slot as allocated
    pub async fn mark_allocated(&self, slot_id: &str) -> Result<()> {
        let mut registry = self.registry.write().await;
        if let Some(slot) = registry.get_mut(slot_id) {
            slot.allocate();
            Ok(())
        } else {
            Err(anyhow!("Slot not found: {}", slot_id))
        }
    }
    
    /// Release slot
    pub async fn release_slot(&self, slot_id: &str) -> Result<()> {
        let mut registry = self.registry.write().await;
        if let Some(slot) = registry.get_mut(slot_id) {
            slot.release();
            Ok(())
        } else {
            Err(anyhow!("Slot not found: {}", slot_id))
        }
    }
}

impl Default for AllocatorConfig {
    fn default() -> Self {
        Self {
            prefer_sigma_majorization: true,
            rebal_limit_per_epoch: 0.2,
            min_tee_pct: 0.4,
            min_poe_quality: 0.8,
            require_diversity: true,
            min_diversity_k: 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bpi_chain_state::{DataClass, QoSLane, TrustTier};
    use super::super::offer::{PriceSpec, AttestationType};
    
    fn create_test_slot(
        slot_id: &str,
        chain_id: &str,
        sigma: SigmaVector,
        cpu: u32,
        mem: u32,
    ) -> BpiSlotOffer {
        let resources = ResourceSpec::new(cpu, mem, 256000, 1000, 10);
        let price = PriceSpec::new(0.10, 0.05, 0.01, 0.10);
        let qos = QoSSpec::new(50, 0.001, 0.999, 10);
        
        let mut offer = BpiSlotOffer::new(
            slot_id.to_string(),
            chain_id.to_string(),
            sigma,
            resources,
            price,
            qos,
            300,
        );
        
        offer.attestation.attestation_type = AttestationType::TEE;
        offer.attestation.poe_quality = 0.9;
        
        offer
    }
    
    #[tokio::test]
    async fn test_allocator_creation() {
        let config = AllocatorConfig::default();
        let allocator = BpciAllocator::new("test-salt".to_string(), config);
        
        let available = allocator.get_available_slots().await;
        assert_eq!(available.len(), 0);
    }
    
    #[tokio::test]
    async fn test_slot_registration() {
        let config = AllocatorConfig::default();
        let allocator = BpciAllocator::new("test-salt".to_string(), config);
        
        let sigma = SigmaVector::new(10, 840, DataClass::Internal.as_u16(), QoSLane::Gold.as_u16(), TrustTier::Enterprise.as_u16(), 0);
        let slot = create_test_slot("slot-001", "chain-001", sigma, 8, 32768);
        
        allocator.register_slot(slot).await.unwrap();
        
        let available = allocator.get_available_slots().await;
        assert_eq!(available.len(), 1);
    }
    
    #[tokio::test]
    async fn test_allocation_success() {
        let config = AllocatorConfig::default();
        let allocator = BpciAllocator::new("test-salt".to_string(), config);
        
        // Register multiple slots
        for i in 0..5 {
            let sigma = SigmaVector::new(10, 840, DataClass::Internal.as_u16(), QoSLane::Gold.as_u16(), TrustTier::Enterprise.as_u16(), 0);
            let slot = create_test_slot(
                &format!("slot-{:03}", i),
                &format!("chain-{:03}", i),
                sigma,
                8,
                32768,
            );
            allocator.register_slot(slot).await.unwrap();
        }
        
        // Create allocation request
        let request = AllocationRequest {
            request_id: "req-001".to_string(),
            policy: SigmaVector::new(5, 0, DataClass::Public.as_u16(), QoSLane::Silver.as_u16(), TrustTier::Verified.as_u16(), 0),
            resources: ResourceSpec::new(4, 16384, 128000, 500, 5),
            qos: QoSSpec::new(100, 0.01, 0.99, 20),
            slot_count: 3,
            max_price_per_hour: None,
            preferred_jurisdictions: vec![],
            require_tee: true,
        };
        
        let result = allocator.allocate(request).await.unwrap();
        
        assert_eq!(result.slots.len(), 3);
        assert!(result.total_price_per_hour > 0.0);
        assert_eq!(result.metadata.candidates_evaluated, 5);
    }
    
    #[tokio::test]
    async fn test_allocation_insufficient_slots() {
        let config = AllocatorConfig::default();
        let allocator = BpciAllocator::new("test-salt".to_string(), config);
        
        // Register only 2 slots
        for i in 0..2 {
            let sigma = SigmaVector::new(10, 840, DataClass::Internal.as_u16(), QoSLane::Gold.as_u16(), TrustTier::Enterprise.as_u16(), 0);
            let slot = create_test_slot(
                &format!("slot-{:03}", i),
                &format!("chain-{:03}", i),
                sigma,
                8,
                32768,
            );
            allocator.register_slot(slot).await.unwrap();
        }
        
        // Request 5 slots
        let request = AllocationRequest {
            request_id: "req-001".to_string(),
            policy: SigmaVector::new(5, 0, DataClass::Public.as_u16(), QoSLane::Silver.as_u16(), TrustTier::Verified.as_u16(), 0),
            resources: ResourceSpec::new(4, 16384, 128000, 500, 5),
            qos: QoSSpec::new(100, 0.01, 0.99, 20),
            slot_count: 5,
            max_price_per_hour: None,
            preferred_jurisdictions: vec![],
            require_tee: false,
        };
        
        let result = allocator.allocate(request).await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_hrw_determinism() {
        let config = AllocatorConfig::default();
        let allocator = BpciAllocator::new("test-salt".to_string(), config);
        
        let sigma = SigmaVector::new(10, 840, DataClass::Internal.as_u16(), QoSLane::Gold.as_u16(), TrustTier::Enterprise.as_u16(), 0);
        let slot1 = create_test_slot("slot-001", "chain-001", sigma, 8, 32768);
        let slot2 = create_test_slot("slot-002", "chain-002", sigma, 8, 32768);
        
        let score1a = allocator.hrw_score("key1", &slot1);
        let score1b = allocator.hrw_score("key1", &slot1);
        
        assert_eq!(score1a, score1b); // Deterministic
        
        let score2 = allocator.hrw_score("key1", &slot2);
        assert_ne!(score1a, score2); // Different slots have different scores
    }
}
