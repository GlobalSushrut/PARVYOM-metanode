//! BPI Slot Offers
//! 
//! Defines the structure and validation for BPI slot resource offers.
//! Slots are resources (CPU, memory, storage, network) that BPI chains
//! offer to BPCI for running shards and validators.

use crate::bpi_chain_state::SigmaVector;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::fmt;

/// BPI slot resource offer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpiSlotOffer {
    /// Unique slot identifier
    pub slot_id: String,
    
    /// BPI chain ID that owns this slot
    pub chain_id: String,
    
    /// 6-D state vector of the slot
    pub sigma: SigmaVector,
    
    /// Available resources
    pub resources: ResourceSpec,
    
    /// Pricing information
    pub price: PriceSpec,
    
    /// QoS guarantees
    pub qos: QoSSpec,
    
    /// TEE attestation quote (optional)
    pub tee_quote: Option<Vec<u8>>,
    
    /// QEC2 finality time in milliseconds
    pub qec2_finality_ms: u32,
    
    /// Slot attestation and proof
    pub attestation: SlotAttestation,
    
    /// Offer creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Offer expiration timestamp
    pub expires_at: DateTime<Utc>,
    
    /// Current status
    pub status: SlotStatus,
}

/// Resource specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSpec {
    /// CPU cores available
    pub cpu_cores: u32,
    
    /// Memory in MB
    pub memory_mb: u32,
    
    /// Storage in MB
    pub storage_mb: u32,
    
    /// Network bandwidth in Mbps
    pub network_mbps: u32,
    
    /// Maximum concurrent vPods
    pub max_vpods: u32,
}

/// Pricing specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceSpec {
    /// Price per CPU core per hour
    pub cpu_per_hour: f64,
    
    /// Price per GB memory per hour
    pub mem_gb_per_hour: f64,
    
    /// Price per GB storage per hour
    pub storage_gb_per_hour: f64,
    
    /// Price per GB egress
    pub egress_gb: f64,
    
    /// Currency (default: "BPI" or "NEX")
    pub currency: String,
}

/// QoS specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QoSSpec {
    /// P95 latency in milliseconds
    pub latency_p95_ms: u32,
    
    /// Packet loss rate (0.0 - 1.0)
    pub loss_rate: f64,
    
    /// Uptime guarantee (0.0 - 1.0)
    pub uptime_guarantee: f64,
    
    /// Jitter in milliseconds
    pub jitter_ms: u32,
}

/// Slot attestation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotAttestation {
    /// Attestation type
    pub attestation_type: AttestationType,
    
    /// Proof of execution quality (0.0 - 1.0)
    pub poe_quality: f64,
    
    /// Signature over slot offer
    pub signature: Vec<u8>,
    
    /// Public key for verification
    pub public_key: Vec<u8>,
    
    /// Additional attestation data
    pub metadata: Option<serde_json::Value>,
}

/// Attestation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttestationType {
    /// Trusted Execution Environment
    TEE,
    
    /// Proof of Execution
    PoE,
    
    /// Combined TEE + PoE
    Hybrid,
    
    /// No attestation (community tier)
    None,
}

/// Slot status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlotStatus {
    /// Available for allocation
    Available,
    
    /// Currently allocated
    Allocated,
    
    /// Temporarily unavailable
    Unavailable,
    
    /// Expired
    Expired,
    
    /// Revoked by owner
    Revoked,
}

impl BpiSlotOffer {
    /// Create a new slot offer
    pub fn new(
        slot_id: String,
        chain_id: String,
        sigma: SigmaVector,
        resources: ResourceSpec,
        price: PriceSpec,
        qos: QoSSpec,
        qec2_finality_ms: u32,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            slot_id,
            chain_id,
            sigma,
            resources,
            price,
            qos,
            tee_quote: None,
            qec2_finality_ms,
            attestation: SlotAttestation::default(),
            created_at: now,
            expires_at: now + chrono::Duration::hours(24), // Default 24h expiry
            status: SlotStatus::Available,
        }
    }
    
    /// Check if the offer is valid (not expired, available)
    pub fn is_valid(&self) -> bool {
        self.status == SlotStatus::Available && Utc::now() < self.expires_at
    }
    
    /// Check if the offer is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }
    
    /// Check if slot satisfies a policy requirement
    pub fn satisfies_policy(&self, policy: &SigmaVector) -> bool {
        self.sigma.majorizes(policy)
    }
    
    /// Calculate total price for a duration
    pub fn calculate_price(&self, duration_hours: f64) -> f64 {
        let cpu_cost = self.resources.cpu_cores as f64 * self.price.cpu_per_hour * duration_hours;
        let mem_cost = (self.resources.memory_mb as f64 / 1024.0) * self.price.mem_gb_per_hour * duration_hours;
        let storage_cost = (self.resources.storage_mb as f64 / 1024.0) * self.price.storage_gb_per_hour * duration_hours;
        
        cpu_cost + mem_cost + storage_cost
    }
    
    /// Mark slot as allocated
    pub fn allocate(&mut self) {
        self.status = SlotStatus::Allocated;
    }
    
    /// Mark slot as available
    pub fn release(&mut self) {
        self.status = SlotStatus::Available;
    }
    
    /// Revoke the slot offer
    pub fn revoke(&mut self) {
        self.status = SlotStatus::Revoked;
    }
    
    /// Update expiration time
    pub fn extend_expiration(&mut self, hours: i64) {
        self.expires_at = self.expires_at + chrono::Duration::hours(hours);
    }
    
    /// Check if slot has TEE attestation
    pub fn has_tee(&self) -> bool {
        self.attestation.attestation_type == AttestationType::TEE 
            || self.attestation.attestation_type == AttestationType::Hybrid
    }
    
    /// Check if PoE quality meets threshold
    pub fn meets_poe_threshold(&self, threshold: f64) -> bool {
        self.attestation.poe_quality >= threshold
    }
}

impl Default for SlotAttestation {
    fn default() -> Self {
        Self {
            attestation_type: AttestationType::None,
            poe_quality: 0.0,
            signature: Vec::new(),
            public_key: Vec::new(),
            metadata: None,
        }
    }
}

impl fmt::Display for BpiSlotOffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Slot[{}] Chain[{}] Σ{} CPU:{} MEM:{}MB Status:{:?}",
            self.slot_id,
            self.chain_id,
            self.sigma,
            self.resources.cpu_cores,
            self.resources.memory_mb,
            self.status
        )
    }
}

impl ResourceSpec {
    /// Create a new resource spec
    pub fn new(
        cpu_cores: u32,
        memory_mb: u32,
        storage_mb: u32,
        network_mbps: u32,
        max_vpods: u32,
    ) -> Self {
        Self {
            cpu_cores,
            memory_mb,
            storage_mb,
            network_mbps,
            max_vpods,
        }
    }
    
    /// Check if this spec can satisfy a requirement
    pub fn satisfies(&self, requirement: &ResourceSpec) -> bool {
        self.cpu_cores >= requirement.cpu_cores
            && self.memory_mb >= requirement.memory_mb
            && self.storage_mb >= requirement.storage_mb
            && self.network_mbps >= requirement.network_mbps
            && self.max_vpods >= requirement.max_vpods
    }
}

impl PriceSpec {
    /// Create a new price spec
    pub fn new(
        cpu_per_hour: f64,
        mem_gb_per_hour: f64,
        storage_gb_per_hour: f64,
        egress_gb: f64,
    ) -> Self {
        Self {
            cpu_per_hour,
            mem_gb_per_hour,
            storage_gb_per_hour,
            egress_gb,
            currency: "BPI".to_string(),
        }
    }
}

impl QoSSpec {
    /// Create a new QoS spec
    pub fn new(
        latency_p95_ms: u32,
        loss_rate: f64,
        uptime_guarantee: f64,
        jitter_ms: u32,
    ) -> Self {
        Self {
            latency_p95_ms,
            loss_rate,
            uptime_guarantee,
            jitter_ms,
        }
    }
    
    /// Check if this QoS meets a requirement
    pub fn meets(&self, requirement: &QoSSpec) -> bool {
        self.latency_p95_ms <= requirement.latency_p95_ms
            && self.loss_rate <= requirement.loss_rate
            && self.uptime_guarantee >= requirement.uptime_guarantee
            && self.jitter_ms <= requirement.jitter_ms
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bpi_chain_state::{DataClass, QoSLane, TrustTier};
    
    fn create_test_offer() -> BpiSlotOffer {
        let sigma = SigmaVector::new(10, 840, DataClass::Internal.as_u16(), QoSLane::Gold.as_u16(), TrustTier::Enterprise.as_u16(), 0);
        let resources = ResourceSpec::new(8, 32768, 256000, 1000, 10);
        let price = PriceSpec::new(0.10, 0.05, 0.01, 0.10);
        let qos = QoSSpec::new(50, 0.001, 0.999, 10);
        
        BpiSlotOffer::new(
            "slot-001".to_string(),
            "bpi-hc-001".to_string(),
            sigma,
            resources,
            price,
            qos,
            300,
        )
    }
    
    #[test]
    fn test_slot_creation() {
        let offer = create_test_offer();
        
        assert_eq!(offer.slot_id, "slot-001");
        assert_eq!(offer.chain_id, "bpi-hc-001");
        assert_eq!(offer.status, SlotStatus::Available);
        assert!(offer.is_valid());
    }
    
    #[test]
    fn test_policy_satisfaction() {
        let offer = create_test_offer();
        
        let policy = SigmaVector::new(5, 0, DataClass::Public.as_u16(), QoSLane::Silver.as_u16(), TrustTier::Verified.as_u16(), 0);
        assert!(offer.satisfies_policy(&policy));
        
        let strict_policy = SigmaVector::new(20, 0, DataClass::PHI.as_u16(), QoSLane::Platinum.as_u16(), TrustTier::Government.as_u16(), 0);
        assert!(!offer.satisfies_policy(&strict_policy));
    }
    
    #[test]
    fn test_price_calculation() {
        let offer = create_test_offer();
        
        // 1 hour: 8 cores * 0.10 + 32GB * 0.05 + 250GB * 0.01
        // = 0.80 + 1.60 + 2.50 = 4.90
        let price_1h = offer.calculate_price(1.0);
        assert!((price_1h - 4.90).abs() < 0.01);
        
        let price_24h = offer.calculate_price(24.0);
        assert!((price_24h - 117.60).abs() < 0.01);
    }
    
    #[test]
    fn test_slot_lifecycle() {
        let mut offer = create_test_offer();
        
        assert_eq!(offer.status, SlotStatus::Available);
        assert!(offer.is_valid());
        
        offer.allocate();
        assert_eq!(offer.status, SlotStatus::Allocated);
        assert!(!offer.is_valid());
        
        offer.release();
        assert_eq!(offer.status, SlotStatus::Available);
        assert!(offer.is_valid());
        
        offer.revoke();
        assert_eq!(offer.status, SlotStatus::Revoked);
        assert!(!offer.is_valid());
    }
    
    #[test]
    fn test_resource_satisfaction() {
        let resources = ResourceSpec::new(8, 32768, 256000, 1000, 10);
        
        let small_req = ResourceSpec::new(4, 16384, 128000, 500, 5);
        assert!(resources.satisfies(&small_req));
        
        let large_req = ResourceSpec::new(16, 65536, 512000, 2000, 20);
        assert!(!resources.satisfies(&large_req));
    }
    
    #[test]
    fn test_qos_requirements() {
        let qos = QoSSpec::new(50, 0.001, 0.999, 10);
        
        let lenient_req = QoSSpec::new(100, 0.01, 0.99, 20);
        assert!(qos.meets(&lenient_req));
        
        let strict_req = QoSSpec::new(20, 0.0001, 0.9999, 5);
        assert!(!qos.meets(&strict_req));
    }
    
    #[test]
    fn test_expiration() {
        let mut offer = create_test_offer();
        
        assert!(!offer.is_expired());
        
        offer.extend_expiration(48);
        assert!(!offer.is_expired());
    }
    
    #[test]
    fn test_attestation() {
        let mut offer = create_test_offer();
        
        assert!(!offer.has_tee());
        assert!(!offer.meets_poe_threshold(0.8));
        
        offer.attestation.attestation_type = AttestationType::TEE;
        offer.attestation.poe_quality = 0.9;
        
        assert!(offer.has_tee());
        assert!(offer.meets_poe_threshold(0.8));
    }
}
