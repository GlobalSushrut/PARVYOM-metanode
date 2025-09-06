//! # PoE Calculator - Proof of Execution Math Engine
//!
//! Implements the v1.0 blueprint PoE calculation: Φ/Γ math for NEX minting and economic proofs.
//! This is Step 3 of the blockchain pipeline: LogBlock → PoE Bundle → BPCI

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::{Result, Context};
use tracing::{debug, info, warn};

/// PoE weights configuration (must sum to 1.0)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoEWeights {
    pub cpu_ms: f64,
    pub memory_mb_s: f64,
    pub storage_gb_day: f64,
    pub egress_mb: f64,
    pub receipts_count: f64,
}

/// PoE scales configuration (normalization factors)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoEScales {
    pub cpu_ms: f64,
    pub memory_mb_s: f64,
    pub storage_gb_day: f64,
    pub egress_mb: f64,
    pub receipts_count: f64,
}

/// Resource usage aggregated from LogBlocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_ms: u64,
    pub memory_mb_s: u64,
    pub storage_gb_day: f64,
    pub egress_mb: f64,
    pub receipts_count: u64,
}

/// PoE Bundle - BPI-comm computes Φ/Γ and submits to BPCI mempool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoEBundle {
    /// Version (always 1 for v1.0)
    pub v: u8,
    /// Application identifier
    pub app: String,
    /// LogBlock hashes included in this bundle
    pub log_blocks: Vec<String>, // blake3 hashes
    /// Aggregated resource usage from all LogBlocks
    pub usage_sum: ResourceUsage,
    /// Φ (Phi) value - normalized resource usage
    pub phi: f64,
    /// Γ (Gamma) value - economic proof factor
    pub gamma: f64,
    /// Billing window (ISO 8601 interval)
    pub billing_window: String,
    /// BPI-comm signature
    pub sig_bpi_comm: String, // ed25519:...
}

/// LogBlock structure (from ENC-notary)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogBlock {
    pub v: u8,
    pub app: String,
    pub height: u64,
    pub merkle_root: String,
    pub count: u32,
    pub sig_notary: String,
    pub range: TimeRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub from_ts: String,
    pub to_ts: String,
}

/// PoE Calculator - Computes Φ/Γ from resource usage
pub struct PoECalculator {
    /// PoE weights configuration
    weights: PoEWeights,
    /// PoE scales configuration
    scales: PoEScales,
    /// Protocol emission scalar (governance parameter)
    k_window: f64,
    /// Adoption factor (network growth oracle)
    adoption_factor: f64,
}

impl Default for PoEWeights {
    fn default() -> Self {
        Self {
            cpu_ms: 0.35,
            memory_mb_s: 0.15,
            storage_gb_day: 0.15,
            egress_mb: 0.15,
            receipts_count: 0.20,
        }
    }
}

impl Default for PoEScales {
    fn default() -> Self {
        Self {
            cpu_ms: 1000.0,
            memory_mb_s: 1000.0,
            storage_gb_day: 1.0,
            egress_mb: 10.0,
            receipts_count: 100.0,
        }
    }
}

impl PoEWeights {
    /// Validate that weights sum to 1.0 (within epsilon)
    pub fn validate(&self) -> Result<()> {
        let sum = self.cpu_ms + self.memory_mb_s + self.storage_gb_day + 
                  self.egress_mb + self.receipts_count;
        
        if (sum - 1.0).abs() > 0.001 {
            return Err(anyhow::anyhow!("PoE weights must sum to 1.0, got {}", sum));
        }
        
        Ok(())
    }

    /// Create weights from Court agreement
    pub fn from_court_config(config: &HashMap<String, f64>) -> Result<Self> {
        let weights = Self {
            cpu_ms: *config.get("cpu_ms").unwrap_or(&0.35),
            memory_mb_s: *config.get("memory_mb_s").unwrap_or(&0.15),
            storage_gb_day: *config.get("storage_gb_day").unwrap_or(&0.15),
            egress_mb: *config.get("egress_mb").unwrap_or(&0.15),
            receipts_count: *config.get("receipts_count").unwrap_or(&0.20),
        };
        
        weights.validate()?;
        Ok(weights)
    }
}

impl PoEScales {
    /// Create scales from Court agreement
    pub fn from_court_config(config: &HashMap<String, f64>) -> Self {
        Self {
            cpu_ms: *config.get("cpu_ms").unwrap_or(&1000.0),
            memory_mb_s: *config.get("memory_mb_s").unwrap_or(&1000.0),
            storage_gb_day: *config.get("storage_gb_day").unwrap_or(&1.0),
            egress_mb: *config.get("egress_mb").unwrap_or(&10.0),
            receipts_count: *config.get("receipts_count").unwrap_or(&100.0),
        }
    }
}

impl ResourceUsage {
    /// Create new empty resource usage
    pub fn new() -> Self {
        Self {
            cpu_ms: 0,
            memory_mb_s: 0,
            storage_gb_day: 0.0,
            egress_mb: 0.0,
            receipts_count: 0,
        }
    }

    /// Add another resource usage
    pub fn add(&mut self, other: &ResourceUsage) {
        self.cpu_ms += other.cpu_ms;
        self.memory_mb_s += other.memory_mb_s;
        self.storage_gb_day += other.storage_gb_day;
        self.egress_mb += other.egress_mb;
        self.receipts_count += other.receipts_count;
    }

    /// Get total usage value for debugging
    pub fn total_value(&self) -> f64 {
        self.cpu_ms as f64 + self.memory_mb_s as f64 + 
        self.storage_gb_day + self.egress_mb + self.receipts_count as f64
    }
}

impl Default for ResourceUsage {
    fn default() -> Self {
        Self::new()
    }
}

impl PoECalculator {
    /// Create new PoE calculator
    pub fn new(weights: PoEWeights, scales: PoEScales) -> Result<Self> {
        weights.validate()?;
        
        Ok(Self {
            weights,
            scales,
            k_window: 1000.0,     // Default protocol emission
            adoption_factor: 1.0,  // Default adoption factor
        })
    }

    /// Create PoE calculator with default configuration
    pub fn default() -> Result<Self> {
        Self::new(PoEWeights::default(), PoEScales::default())
    }

    /// Set protocol emission scalar (governance parameter)
    pub fn with_k_window(mut self, k_window: f64) -> Self {
        self.k_window = k_window;
        self
    }

    /// Set adoption factor (network growth oracle)
    pub fn with_adoption_factor(mut self, adoption_factor: f64) -> Self {
        self.adoption_factor = adoption_factor;
        self
    }

    /// Calculate Φ (Phi) from resource usage
    /// Φ(t) = Σᵢ wᵢ · (uᵢ/sᵢ), where 0 ≤ Σᵢ wᵢ = 1
    pub fn calculate_phi(&self, usage: &ResourceUsage) -> f64 {
        let mut phi = 0.0;
        
        phi += self.weights.cpu_ms * (usage.cpu_ms as f64 / self.scales.cpu_ms);
        phi += self.weights.memory_mb_s * (usage.memory_mb_s as f64 / self.scales.memory_mb_s);
        phi += self.weights.storage_gb_day * (usage.storage_gb_day / self.scales.storage_gb_day);
        phi += self.weights.egress_mb * (usage.egress_mb / self.scales.egress_mb);
        phi += self.weights.receipts_count * (usage.receipts_count as f64 / self.scales.receipts_count);
        
        debug!("PoE Φ calculation: cpu={:.4}, mem={:.4}, storage={:.4}, egress={:.4}, receipts={:.4} → Φ={:.6}",
               self.weights.cpu_ms * (usage.cpu_ms as f64 / self.scales.cpu_ms),
               self.weights.memory_mb_s * (usage.memory_mb_s as f64 / self.scales.memory_mb_s),
               self.weights.storage_gb_day * (usage.storage_gb_day / self.scales.storage_gb_day),
               self.weights.egress_mb * (usage.egress_mb / self.scales.egress_mb),
               self.weights.receipts_count * (usage.receipts_count as f64 / self.scales.receipts_count),
               phi);
        
        phi
    }

    /// Calculate Γ (Gamma) from Φ
    /// Γ(Φ) = Φ/(1+Φ) ∈ [0,1)
    pub fn calculate_gamma(&self, phi: f64) -> f64 {
        let gamma = phi / (1.0 + phi);
        debug!("PoE Γ calculation: Φ={:.6} → Γ={:.6}", phi, gamma);
        gamma
    }

    /// Calculate NEX mint amount for billing window
    /// NEX_mint = K_window · Γ(Φ) · A
    pub fn calculate_nex_mint(&self, gamma: f64) -> f64 {
        let nex_mint = self.k_window * gamma * self.adoption_factor;
        debug!("NEX mint calculation: K={:.2}, Γ={:.6}, A={:.2} → NEX={:.2}",
               self.k_window, gamma, self.adoption_factor, nex_mint);
        nex_mint
    }

    /// Calculate PoE values from resource usage (convenience method)
    pub fn calculate_poe(&self, usage: &ResourceUsage) -> (f64, f64, f64) {
        let phi = self.calculate_phi(usage);
        let gamma = self.calculate_gamma(phi);
        let nex_mint = self.calculate_nex_mint(gamma);
        
        info!("PoE calculation complete: usage={:.2} → Φ={:.6}, Γ={:.6}, NEX={:.2}",
              usage.total_value(), phi, gamma, nex_mint);
        
        (phi, gamma, nex_mint)
    }

    /// Aggregate resource usage from multiple LogBlocks
    pub fn aggregate_logblock_usage(&self, logblocks: &[LogBlock]) -> Result<ResourceUsage> {
        let mut total_usage = ResourceUsage::new();
        
        // For now, we estimate usage from LogBlock metadata
        // In a full implementation, we would parse the actual StepReceipts
        for logblock in logblocks {
            // Estimate usage based on receipt count and time window
            let estimated_usage = self.estimate_usage_from_logblock(logblock)?;
            total_usage.add(&estimated_usage);
        }
        
        debug!("Aggregated usage from {} LogBlocks: cpu={}ms, mem={}MB·s, storage={:.2}GB·d, egress={:.2}MB, receipts={}",
               logblocks.len(), total_usage.cpu_ms, total_usage.memory_mb_s, 
               total_usage.storage_gb_day, total_usage.egress_mb, total_usage.receipts_count);
        
        Ok(total_usage)
    }

    /// Estimate resource usage from LogBlock metadata
    fn estimate_usage_from_logblock(&self, logblock: &LogBlock) -> Result<ResourceUsage> {
        // Parse time range to estimate duration
        let _duration_s = self.parse_time_range(&logblock.range)?;
        
        // Estimate usage based on receipt count and duration
        // These are rough estimates - in production, we'd parse actual StepReceipts
        let base_cpu_per_receipt = 10; // 10ms CPU per receipt
        let base_memory_per_receipt = 5; // 5MB·s memory per receipt
        let base_storage_per_receipt = 0.001; // 0.001GB·d storage per receipt
        let base_egress_per_receipt = 0.1; // 0.1MB egress per receipt
        
        let usage = ResourceUsage {
            cpu_ms: (logblock.count as u64) * base_cpu_per_receipt,
            memory_mb_s: (logblock.count as u64) * base_memory_per_receipt,
            storage_gb_day: (logblock.count as f64) * base_storage_per_receipt,
            egress_mb: (logblock.count as f64) * base_egress_per_receipt,
            receipts_count: logblock.count as u64,
        };
        
        Ok(usage)
    }

    /// Parse time range to get duration in seconds
    fn parse_time_range(&self, range: &TimeRange) -> Result<u64> {
        let from = chrono::DateTime::parse_from_rfc3339(&range.from_ts)
            .context("Failed to parse from_ts")?;
        let to = chrono::DateTime::parse_from_rfc3339(&range.to_ts)
            .context("Failed to parse to_ts")?;
        
        let duration = to.signed_duration_since(from);
        Ok(duration.num_seconds().max(0) as u64)
    }

    /// Create PoE bundle from LogBlocks
    pub fn create_poe_bundle(
        &self,
        app_id: String,
        logblocks: Vec<LogBlock>,
        billing_window: String,
    ) -> Result<PoEBundle> {
        // Aggregate resource usage
        let usage_sum = self.aggregate_logblock_usage(&logblocks)?;
        
        // Calculate PoE values
        let (phi, gamma, _nex_mint) = self.calculate_poe(&usage_sum);
        
        // Extract LogBlock hashes
        let log_blocks: Vec<String> = logblocks.iter()
            .map(|lb| lb.merkle_root.clone())
            .collect();
        
        let bundle = PoEBundle {
            v: 1,
            app: app_id,
            log_blocks,
            usage_sum,
            phi,
            gamma,
            billing_window,
            sig_bpi_comm: String::new(), // Will be signed by BPI-comm
        };
        
        info!("Created PoE bundle: app={}, logblocks={}, Φ={:.6}, Γ={:.6}",
              bundle.app, bundle.log_blocks.len(), bundle.phi, bundle.gamma);
        
        Ok(bundle)
    }
}

/// PoE calculation golden test vectors for reproducibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoETestVector {
    pub name: String,
    pub usage: ResourceUsage,
    pub weights: PoEWeights,
    pub scales: PoEScales,
    pub expected_phi: f64,
    pub expected_gamma: f64,
    pub expected_nex: f64,
}

impl PoETestVector {
    /// Create golden test vectors
    pub fn create_golden_vectors() -> Vec<PoETestVector> {
        vec![
            PoETestVector {
                name: "basic_computation".to_string(),
                usage: ResourceUsage {
                    cpu_ms: 1000,
                    memory_mb_s: 500,
                    storage_gb_day: 1.0,
                    egress_mb: 10.0,
                    receipts_count: 100,
                },
                weights: PoEWeights::default(),
                scales: PoEScales::default(),
                expected_phi: 0.925, // Pre-calculated (corrected)
                expected_gamma: 0.4805194805194805, // Pre-calculated (corrected)
                expected_nex: 480.5194805194805, // Pre-calculated with K=1000, A=1 (corrected)
            },
            PoETestVector {
                name: "zero_usage".to_string(),
                usage: ResourceUsage::new(),
                weights: PoEWeights::default(),
                scales: PoEScales::default(),
                expected_phi: 0.0,
                expected_gamma: 0.0,
                expected_nex: 0.0,
            },
            PoETestVector {
                name: "high_cpu_usage".to_string(),
                usage: ResourceUsage {
                    cpu_ms: 10000,
                    memory_mb_s: 100,
                    storage_gb_day: 0.1,
                    egress_mb: 1.0,
                    receipts_count: 10,
                },
                weights: PoEWeights::default(),
                scales: PoEScales::default(),
                expected_phi: 3.5650000000000004, // Pre-calculated (corrected)
                expected_gamma: 0.7809419496166484, // Pre-calculated (corrected)
                expected_nex: 780.9419496166484, // Pre-calculated (corrected)
            },
        ]
    }

    /// Verify test vector against calculator
    pub fn verify(&self, calculator: &PoECalculator) -> Result<()> {
        let (phi, gamma, nex) = calculator.calculate_poe(&self.usage);
        
        let phi_diff = (phi - self.expected_phi).abs();
        let gamma_diff = (gamma - self.expected_gamma).abs();
        let nex_diff = (nex - self.expected_nex).abs();
        
        const EPSILON: f64 = 0.001;
        
        if phi_diff > EPSILON {
            return Err(anyhow::anyhow!("Phi mismatch in {}: expected {}, got {} (diff: {})",
                                     self.name, self.expected_phi, phi, phi_diff));
        }
        
        if gamma_diff > EPSILON {
            return Err(anyhow::anyhow!("Gamma mismatch in {}: expected {}, got {} (diff: {})",
                                     self.name, self.expected_gamma, gamma, gamma_diff));
        }
        
        if nex_diff > EPSILON {
            return Err(anyhow::anyhow!("NEX mismatch in {}: expected {}, got {} (diff: {})",
                                     self.name, self.expected_nex, nex, nex_diff));
        }
        
        info!("✅ PoE test vector '{}' passed", self.name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poe_weights_validation() {
        let valid_weights = PoEWeights::default();
        assert!(valid_weights.validate().is_ok());

        let invalid_weights = PoEWeights {
            cpu_ms: 0.5,
            memory_mb_s: 0.5,
            storage_gb_day: 0.5, // Sum > 1.0
            egress_mb: 0.0,
            receipts_count: 0.0,
        };
        assert!(invalid_weights.validate().is_err());
    }

    #[test]
    fn test_phi_calculation() {
        let calculator = PoECalculator::default().unwrap();
        
        let usage = ResourceUsage {
            cpu_ms: 1000,
            memory_mb_s: 1000,
            storage_gb_day: 1.0,
            egress_mb: 10.0,
            receipts_count: 100,
        };

        let phi = calculator.calculate_phi(&usage);
        assert!(phi > 0.0);
        assert!(phi < 10.0); // Reasonable bounds
    }

    #[test]
    fn test_gamma_calculation() {
        let calculator = PoECalculator::default().unwrap();
        
        // Test gamma bounds: Γ(Φ) ∈ [0,1)
        assert_eq!(calculator.calculate_gamma(0.0), 0.0);
        assert!(calculator.calculate_gamma(1.0) < 1.0);
        assert!(calculator.calculate_gamma(100.0) < 1.0);
        
        // Test specific values
        let gamma_1 = calculator.calculate_gamma(1.0);
        assert!((gamma_1 - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_resource_usage_aggregation() {
        let mut usage1 = ResourceUsage {
            cpu_ms: 100,
            memory_mb_s: 50,
            storage_gb_day: 0.1,
            egress_mb: 1.0,
            receipts_count: 10,
        };

        let usage2 = ResourceUsage {
            cpu_ms: 200,
            memory_mb_s: 100,
            storage_gb_day: 0.2,
            egress_mb: 2.0,
            receipts_count: 20,
        };

        usage1.add(&usage2);

        assert_eq!(usage1.cpu_ms, 300);
        assert_eq!(usage1.memory_mb_s, 150);
        assert!((usage1.storage_gb_day - 0.3).abs() < 0.001);
        assert!((usage1.egress_mb - 3.0).abs() < 0.001);
        assert_eq!(usage1.receipts_count, 30);
    }

    #[test]
    fn test_golden_vectors() {
        let calculator = PoECalculator::default().unwrap();
        let vectors = PoETestVector::create_golden_vectors();
        
        for vector in vectors {
            // Create calculator with test vector config
            let test_calculator = PoECalculator::new(vector.weights.clone(), vector.scales.clone())
                .unwrap()
                .with_k_window(1000.0)
                .with_adoption_factor(1.0);
            
            vector.verify(&test_calculator).unwrap();
        }
    }

    #[test]
    fn test_poe_bundle_creation() {
        let calculator = PoECalculator::default().unwrap();
        
        let logblock = LogBlock {
            v: 1,
            app: "TEST_APP".to_string(),
            height: 1,
            merkle_root: "blake3:test".to_string(),
            count: 100,
            sig_notary: "ed25519:test".to_string(),
            range: TimeRange {
                from_ts: "2025-08-13T06:00:00Z".to_string(),
                to_ts: "2025-08-13T06:01:00Z".to_string(),
            },
        };

        let bundle = calculator.create_poe_bundle(
            "TEST_APP".to_string(),
            vec![logblock],
            "2025-08-13T06:00:00Z/2025-08-13T07:00:00Z".to_string(),
        ).unwrap();

        assert_eq!(bundle.v, 1);
        assert_eq!(bundle.app, "TEST_APP");
        assert_eq!(bundle.log_blocks.len(), 1);
        assert!(bundle.phi >= 0.0);
        assert!(bundle.gamma >= 0.0);
        assert!(bundle.gamma < 1.0);
    }
}
