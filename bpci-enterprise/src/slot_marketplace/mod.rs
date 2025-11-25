//! Slot Marketplace
//! 
//! BPI slot marketplace for resource-resident BPCI migration.
//! 
//! # Overview
//! 
//! The slot marketplace enables BPI chains to offer compute resources (slots)
//! that BPCI can use to run shards and validators. This is the foundation for
//! the resource-resident architecture where BPCI "walks off" the central server
//! and lives inside BPI slots.
//! 
//! # Core Components
//! 
//! - **BpiSlotOffer**: Resource offers from BPI chains
//! - **BpciAllocator**: Slot selection using HRW + Σ-majorization
//! - **AllocationRequest**: Requirements for slot allocation
//! - **AllocationResult**: Selected slots and metadata
//! 
//! # Usage
//! 
//! ```rust
//! use bpci_enterprise::slot_marketplace::{
//!     BpciAllocator, AllocatorConfig, AllocationRequest,
//!     BpiSlotOffer, ResourceSpec, QoSSpec,
//! };
//! use bpci_enterprise::bpi_chain_state::SigmaVector;
//! 
//! # async fn example() -> anyhow::Result<()> {
//! // Create allocator
//! let config = AllocatorConfig::default();
//! let allocator = BpciAllocator::new("my-salt".to_string(), config);
//! 
//! // Register slot offers
//! let sigma = SigmaVector::new(10, 840, 3, 2, 2, 0);
//! let resources = ResourceSpec::new(8, 32768, 256000, 1000, 10);
//! let price = PriceSpec::new(0.10, 0.05, 0.01, 0.10);
//! let qos = QoSSpec::new(50, 0.001, 0.999, 10);
//! 
//! let offer = BpiSlotOffer::new(
//!     "slot-001".to_string(),
//!     "bpi-hc-001".to_string(),
//!     sigma,
//!     resources,
//!     price,
//!     qos,
//!     300,
//! );
//! 
//! allocator.register_slot(offer).await?;
//! 
//! // Allocate slots
//! let request = AllocationRequest {
//!     request_id: "req-001".to_string(),
//!     policy: SigmaVector::new(5, 0, 2, 1, 1, 0),
//!     resources: ResourceSpec::new(4, 16384, 128000, 500, 5),
//!     qos: QoSSpec::new(100, 0.01, 0.99, 20),
//!     slot_count: 3,
//!     max_price_per_hour: Some(20.0),
//!     preferred_jurisdictions: vec![],
//!     require_tee: true,
//! };
//! 
//! let result = allocator.allocate(request).await?;
//! println!("Allocated {} slots for ${}/hour", 
//!          result.slots.len(), 
//!          result.total_price_per_hour);
//! # Ok(())
//! # }
//! ```
//! 
//! # Allocation Algorithm
//! 
//! The allocator uses a multi-stage filtering and selection process:
//! 
//! 1. **Σ-Majorization**: Filter slots where `slot.sigma ≽ policy.sigma`
//! 2. **Resource Matching**: Filter by CPU, memory, storage, network
//! 3. **QoS Matching**: Filter by latency, loss, uptime, jitter
//! 4. **Attestation**: Filter by TEE/PoE requirements
//! 5. **Price**: Filter by maximum price (if specified)
//! 6. **HRW Selection**: Select top-N using Highest Random Weight
//! 7. **Diversity Check**: Validate fault domain diversity
//! 
//! # HRW (Highest Random Weight)
//! 
//! HRW provides deterministic, distributed slot selection:
//! - Same request ID always selects same slots (given same candidates)
//! - No coordination needed between allocators
//! - Automatic load balancing
//! - Graceful handling of slot additions/removals

pub mod offer;
pub mod allocator;

// Re-export main types
pub use offer::{
    BpiSlotOffer,
    ResourceSpec,
    PriceSpec,
    QoSSpec,
    SlotAttestation,
    AttestationType,
    SlotStatus,
};

pub use allocator::{
    BpciAllocator,
    AllocatorConfig,
    AllocationRequest,
    AllocationResult,
    AllocationMetadata,
};

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::bpi_chain_state::{SigmaVector, DataClass, QoSLane, TrustTier, PolicyTemplates};
    
    fn create_healthcare_slot(slot_id: &str, chain_id: &str) -> BpiSlotOffer {
        let sigma = SigmaVector::new(
            60,
            840, // USA
            DataClass::PHI.as_u16(),
            QoSLane::Gold.as_u16(),
            TrustTier::Enterprise.as_u16(),
            0,
        );
        
        let resources = ResourceSpec::new(16, 65536, 512000, 2000, 20);
        let price = PriceSpec::new(0.20, 0.10, 0.02, 0.15);
        let qos = QoSSpec::new(30, 0.0005, 0.9999, 5);
        
        let mut offer = BpiSlotOffer::new(
            slot_id.to_string(),
            chain_id.to_string(),
            sigma,
            resources,
            price,
            qos,
            200,
        );
        
        offer.attestation.attestation_type = AttestationType::Hybrid;
        offer.attestation.poe_quality = 0.95;
        
        offer
    }
    
    #[tokio::test]
    async fn test_healthcare_allocation_scenario() {
        // Scenario: Healthcare application needs HIPAA-compliant slots
        
        let config = AllocatorConfig::default();
        let allocator = BpciAllocator::new("healthcare-salt".to_string(), config);
        
        // Register healthcare-compliant slots
        for i in 0..5 {
            let slot = create_healthcare_slot(
                &format!("hc-slot-{:03}", i),
                &format!("hc-chain-{:03}", i),
            );
            allocator.register_slot(slot).await.unwrap();
        }
        
        // Create HIPAA allocation request
        let hipaa_policy = PolicyTemplates::healthcare_hipaa();
        
        let request = AllocationRequest {
            request_id: "hipaa-req-001".to_string(),
            policy: hipaa_policy.minimum,
            resources: ResourceSpec::new(8, 32768, 256000, 1000, 10),
            qos: QoSSpec::new(50, 0.001, 0.999, 10),
            slot_count: 3,
            max_price_per_hour: Some(50.0),
            preferred_jurisdictions: vec![840], // USA
            require_tee: true,
        };
        
        let result = allocator.allocate(request).await.unwrap();
        
        // Verify allocation
        assert_eq!(result.slots.len(), 3);
        assert!(result.total_price_per_hour > 0.0);
        assert!(result.metadata.diversity_score > 0.0);
        assert!(result.metadata.avg_poe_quality >= 0.95);
        
        // Verify all slots are HIPAA-compliant
        for slot in &result.slots {
            assert!(slot.sigma.data_class >= DataClass::PHI.as_u16());
            assert_eq!(slot.sigma.jurisdiction, 840);
            assert!(slot.has_tee());
        }
    }
    
    #[tokio::test]
    async fn test_mixed_tier_allocation() {
        // Scenario: Allocate from mixed-tier slots
        
        let config = AllocatorConfig {
            prefer_sigma_majorization: true,
            rebal_limit_per_epoch: 0.2,
            min_tee_pct: 0.0, // Allow non-TEE
            min_poe_quality: 0.5,
            require_diversity: true,
            min_diversity_k: 2,
        };
        
        let allocator = BpciAllocator::new("mixed-salt".to_string(), config);
        
        // Register enterprise slots
        for i in 0..3 {
            let sigma = SigmaVector::new(
                20,
                0,
                DataClass::Internal.as_u16(),
                QoSLane::Gold.as_u16(),
                TrustTier::Enterprise.as_u16(),
                0,
            );
            let resources = ResourceSpec::new(8, 32768, 256000, 1000, 10);
            let price = PriceSpec::new(0.15, 0.08, 0.02, 0.12);
            let qos = QoSSpec::new(40, 0.001, 0.999, 8);
            
            let mut offer = BpiSlotOffer::new(
                format!("ent-slot-{:03}", i),
                format!("ent-chain-{:03}", i),
                sigma,
                resources,
                price,
                qos,
                250,
            );
            offer.attestation.poe_quality = 0.9;
            
            allocator.register_slot(offer).await.unwrap();
        }
        
        // Register community slots
        for i in 0..3 {
            let sigma = SigmaVector::new(
                5,
                0,
                DataClass::Public.as_u16(),
                QoSLane::Bronze.as_u16(),
                TrustTier::Community.as_u16(),
                0,
            );
            let resources = ResourceSpec::new(4, 16384, 128000, 500, 5);
            let price = PriceSpec::new(0.05, 0.03, 0.01, 0.05);
            let qos = QoSSpec::new(100, 0.01, 0.99, 20);
            
            let mut offer = BpiSlotOffer::new(
                format!("com-slot-{:03}", i),
                format!("com-chain-{:03}", i),
                sigma,
                resources,
                price,
                qos,
                400,
            );
            offer.attestation.poe_quality = 0.6;
            
            allocator.register_slot(offer).await.unwrap();
        }
        
        // Request enterprise-level slots
        let request = AllocationRequest {
            request_id: "mixed-req-001".to_string(),
            policy: SigmaVector::new(
                10,
                0,
                DataClass::Internal.as_u16(),
                QoSLane::Silver.as_u16(),
                TrustTier::Verified.as_u16(),
                0,
            ),
            resources: ResourceSpec::new(4, 16384, 128000, 500, 5),
            qos: QoSSpec::new(60, 0.005, 0.995, 15),
            slot_count: 2,
            max_price_per_hour: None,
            preferred_jurisdictions: vec![],
            require_tee: false,
        };
        
        let result = allocator.allocate(request).await.unwrap();
        
        // Should only get enterprise slots (community doesn't meet requirements)
        assert_eq!(result.slots.len(), 2);
        for slot in &result.slots {
            assert!(slot.slot_id.starts_with("ent-"));
        }
    }
    
    #[tokio::test]
    async fn test_price_filtering() {
        let config = AllocatorConfig::default();
        let allocator = BpciAllocator::new("price-salt".to_string(), config);
        
        // Register expensive slot
        let sigma = SigmaVector::new(10, 0, 2, 2, 2, 0);
        let resources = ResourceSpec::new(8, 32768, 256000, 1000, 10);
        let expensive_price = PriceSpec::new(1.0, 0.5, 0.1, 0.5); // Expensive
        let qos = QoSSpec::new(50, 0.001, 0.999, 10);
        
        let mut expensive = BpiSlotOffer::new(
            "expensive-001".to_string(),
            "chain-001".to_string(),
            sigma,
            resources.clone(),
            expensive_price,
            qos.clone(),
            300,
        );
        expensive.attestation.poe_quality = 0.9;
        
        // Register cheap slot
        let cheap_price = PriceSpec::new(0.05, 0.03, 0.01, 0.05); // Cheap
        let mut cheap = BpiSlotOffer::new(
            "cheap-001".to_string(),
            "chain-002".to_string(),
            sigma,
            resources,
            cheap_price,
            qos,
            300,
        );
        cheap.attestation.poe_quality = 0.9;
        
        allocator.register_slot(expensive).await.unwrap();
        allocator.register_slot(cheap).await.unwrap();
        
        // Request with price limit
        let request = AllocationRequest {
            request_id: "price-req-001".to_string(),
            policy: SigmaVector::new(5, 0, 1, 1, 1, 0),
            resources: ResourceSpec::new(4, 16384, 128000, 500, 5),
            qos: QoSSpec::new(100, 0.01, 0.99, 20),
            slot_count: 1,
            max_price_per_hour: Some(5.0), // Low limit
            preferred_jurisdictions: vec![],
            require_tee: false,
        };
        
        let result = allocator.allocate(request).await.unwrap();
        
        // Should only get cheap slot
        assert_eq!(result.slots.len(), 1);
        assert_eq!(result.slots[0].slot_id, "cheap-001");
        assert!(result.total_price_per_hour < 5.0);
    }
}
