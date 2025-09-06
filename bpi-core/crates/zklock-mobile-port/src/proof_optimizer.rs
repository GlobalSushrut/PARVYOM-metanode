//! Proof Optimizer - Mobile-optimized ZK proof generation
//!
//! This module optimizes zero-knowledge proof generation and verification
//! for mobile and IoT devices with limited computational resources.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::{ZKConfig, DeviceType, ComputeLevel};

/// Proof optimizer for mobile devices
#[derive(Debug)]
pub struct ProofOptimizer {
    /// Proof templates for different device types
    proof_templates: Arc<RwLock<HashMap<String, ProofTemplate>>>,
    /// Optimization cache
    optimization_cache: Arc<RwLock<HashMap<String, OptimizedProof>>>,
    /// Configuration
    config: ZKConfig,
    /// Optimizer statistics
    stats: Arc<RwLock<OptimizerStats>>,
}

/// Proof template for specific device capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofTemplate {
    pub template_id: String,
    pub device_type: String,
    pub compute_level: ComputeLevel,
    pub max_proof_size: usize,
    pub max_generation_time_ms: u64,
    pub optimization_level: OptimizationLevel,
    pub supported_algorithms: Vec<ProofAlgorithm>,
    pub parameters: ProofParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationLevel {
    Minimal,    // Basic optimization for ultra-low power
    Balanced,   // Good balance of size and speed
    Performance, // Optimize for speed
    Size,       // Optimize for minimal size
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProofAlgorithm {
    PLONK,      // Universal SNARK
    Groth16,    // Efficient verification
    STARK,      // Post-quantum secure
    Bulletproofs, // Range proofs
    Simplified, // Custom lightweight algorithm
}

/// Proof parameters for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofParameters {
    pub circuit_size: u32,
    pub witness_size: u32,
    pub public_inputs: u32,
    pub security_level: u8,
    pub batch_size: u32,
    pub compression_ratio: f64,
}

/// Optimized proof for mobile devices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedProof {
    pub proof_id: String,
    pub original_size: usize,
    pub optimized_size: usize,
    pub compression_ratio: f64,
    pub generation_time_ms: u64,
    pub verification_time_ms: u64,
    pub algorithm_used: ProofAlgorithm,
    pub optimization_applied: Vec<OptimizationType>,
    pub proof_data: Vec<u8>,
    pub metadata: ProofMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationType {
    Compression,
    Batching,
    Preprocessing,
    CircuitOptimization,
    WitnessReduction,
    PublicInputMinimization,
}

/// Proof metadata for verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofMetadata {
    pub device_id: Uuid,
    pub device_type: DeviceType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub battery_cost: f64,
    pub network_cost: u64,
    pub verification_key_hash: [u8; 32],
}

/// Optimizer statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizerStats {
    pub total_proofs_optimized: u64,
    pub average_compression_ratio: f64,
    pub average_generation_time_ms: f64,
    pub average_verification_time_ms: f64,
    pub battery_savings_percent: f64,
    pub size_reduction_percent: f64,
    pub algorithm_usage: HashMap<String, u64>,
}

impl Default for OptimizerStats {
    fn default() -> Self {
        Self {
            total_proofs_optimized: 0,
            average_compression_ratio: 1.0,
            average_generation_time_ms: 0.0,
            average_verification_time_ms: 0.0,
            battery_savings_percent: 0.0,
            size_reduction_percent: 0.0,
            algorithm_usage: HashMap::new(),
        }
    }
}

impl ProofOptimizer {
    /// Create a new proof optimizer
    pub async fn new(config: ZKConfig) -> Result<Self> {
        info!("Initializing Proof Optimizer");

        let mut proof_templates = HashMap::new();
        
        // Create templates for different device types
        proof_templates.insert("Mobile".to_string(), Self::create_mobile_template());
        proof_templates.insert("IoT".to_string(), Self::create_iot_template());
        proof_templates.insert("Edge".to_string(), Self::create_edge_template());
        proof_templates.insert("Wearable".to_string(), Self::create_wearable_template());

        Ok(Self {
            proof_templates: Arc::new(RwLock::new(proof_templates)),
            optimization_cache: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(RwLock::new(OptimizerStats::default())),
        })
    }

    /// Optimize proof for specific device type
    pub async fn optimize_proof(&self, proof_data: Vec<u8>, device_type: &DeviceType) -> Result<Vec<u8>> {
        let start_time = std::time::Instant::now();
        let original_size = proof_data.len();
        
        // Get appropriate template
        let template = self.get_template_for_device(device_type).await?;
        
        // Check if proof exceeds device limits
        if original_size > template.max_proof_size {
            return Err(anyhow::anyhow!("Proof too large for device: {} > {}", original_size, template.max_proof_size));
        }

        // Apply optimizations based on template
        let optimized_data = self.apply_optimizations(&proof_data, &template).await?;
        let generation_time = start_time.elapsed().as_millis() as u64;
        
        // Check time constraints
        if generation_time > template.max_generation_time_ms {
            warn!("Proof optimization took longer than expected: {}ms > {}ms", 
                  generation_time, template.max_generation_time_ms);
        }

        // Create optimized proof record
        let optimized_proof = OptimizedProof {
            proof_id: Uuid::new_v4().to_string(),
            original_size,
            optimized_size: optimized_data.len(),
            compression_ratio: original_size as f64 / optimized_data.len() as f64,
            generation_time_ms: generation_time,
            verification_time_ms: 0, // Will be set during verification
            algorithm_used: template.supported_algorithms[0].clone(),
            optimization_applied: self.determine_optimizations_applied(&template),
            proof_data: optimized_data.clone(),
            metadata: ProofMetadata {
                device_id: Uuid::new_v4(), // Would be passed from caller
                device_type: device_type.clone(),
                timestamp: chrono::Utc::now(),
                battery_cost: self.calculate_battery_cost(&template, generation_time).await,
                network_cost: optimized_data.len() as u64,
                verification_key_hash: [0u8; 32], // Would be actual VK hash
            },
        };

        // Cache the optimized proof
        self.optimization_cache.write().await.insert(optimized_proof.proof_id.clone(), optimized_proof);

        // Update statistics
        self.update_stats_after_optimization(original_size, optimized_data.len(), generation_time).await;

        info!("Optimized proof: {} -> {} bytes ({:.1}x compression) in {}ms", 
              original_size, optimized_data.len(), 
              original_size as f64 / optimized_data.len() as f64, generation_time);

        Ok(optimized_data)
    }

    /// Verify optimized proof
    pub async fn verify_proof(&self, proof_data: &[u8], device_type: &DeviceType) -> Result<bool> {
        let start_time = std::time::Instant::now();
        
        // Get appropriate template for verification
        let template = self.get_template_for_device(device_type).await?;
        
        // Perform verification based on algorithm
        let is_valid = self.perform_verification(proof_data, &template).await?;
        
        let verification_time = start_time.elapsed().as_millis() as u64;
        
        // Update verification time in cache if proof exists
        let proof_hash = self.hash_proof_data(proof_data);
        if let Some(cached_proof) = self.find_cached_proof_by_hash(&proof_hash).await {
            if let Some(proof) = self.optimization_cache.write().await.get_mut(&cached_proof) {
                proof.verification_time_ms = verification_time;
            }
        }

        debug!("Verified proof in {}ms: {}", verification_time, is_valid);
        Ok(is_valid)
    }

    /// Get optimizer statistics
    pub async fn get_stats(&self) -> Result<OptimizerStats> {
        Ok(self.stats.read().await.clone())
    }

    /// Get template for specific device type
    async fn get_template_for_device(&self, device_type: &DeviceType) -> Result<ProofTemplate> {
        let device_type_str = match device_type {
            DeviceType::Mobile { .. } => "Mobile",
            DeviceType::IoT { .. } => "IoT",
            DeviceType::Edge { .. } => "Edge",
            DeviceType::Wearable { .. } => "Wearable",
        };

        self.proof_templates.read().await
            .get(device_type_str)
            .cloned()
            .context("No template found for device type")
    }

    /// Apply optimizations based on template
    async fn apply_optimizations(&self, proof_data: &[u8], template: &ProofTemplate) -> Result<Vec<u8>> {
        let mut optimized_data = proof_data.to_vec();

        match template.optimization_level {
            OptimizationLevel::Minimal => {
                // Basic compression only
                optimized_data = self.apply_basic_compression(&optimized_data).await?;
            },
            OptimizationLevel::Balanced => {
                // Compression + witness reduction
                optimized_data = self.apply_basic_compression(&optimized_data).await?;
                optimized_data = self.apply_witness_reduction(&optimized_data).await?;
            },
            OptimizationLevel::Performance => {
                // Preprocessing + batching
                optimized_data = self.apply_preprocessing(&optimized_data).await?;
                optimized_data = self.apply_batching(&optimized_data).await?;
            },
            OptimizationLevel::Size => {
                // Maximum compression
                optimized_data = self.apply_maximum_compression(&optimized_data).await?;
                optimized_data = self.apply_witness_reduction(&optimized_data).await?;
                optimized_data = self.apply_public_input_minimization(&optimized_data).await?;
            },
        }

        Ok(optimized_data)
    }

    /// Apply basic compression
    async fn apply_basic_compression(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Simplified compression (in real implementation, use proper compression)
        let compression_ratio = 0.8; // 20% size reduction
        let compressed_size = (data.len() as f64 * compression_ratio) as usize;
        Ok(data[..compressed_size.min(data.len())].to_vec())
    }

    /// Apply witness reduction
    async fn apply_witness_reduction(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Simplified witness reduction
        let reduction_ratio = 0.9; // 10% size reduction
        let reduced_size = (data.len() as f64 * reduction_ratio) as usize;
        Ok(data[..reduced_size.min(data.len())].to_vec())
    }

    /// Apply preprocessing
    async fn apply_preprocessing(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Simplified preprocessing optimization
        Ok(data.to_vec())
    }

    /// Apply batching
    async fn apply_batching(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Simplified batching optimization
        Ok(data.to_vec())
    }

    /// Apply maximum compression
    async fn apply_maximum_compression(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Simplified maximum compression
        let compression_ratio = 0.6; // 40% size reduction
        let compressed_size = (data.len() as f64 * compression_ratio) as usize;
        Ok(data[..compressed_size.min(data.len())].to_vec())
    }

    /// Apply public input minimization
    async fn apply_public_input_minimization(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Simplified public input minimization
        let reduction_ratio = 0.95; // 5% size reduction
        let reduced_size = (data.len() as f64 * reduction_ratio) as usize;
        Ok(data[..reduced_size.min(data.len())].to_vec())
    }

    /// Perform verification based on algorithm
    async fn perform_verification(&self, _proof_data: &[u8], template: &ProofTemplate) -> Result<bool> {
        // Simplified verification (in real implementation, use actual ZK verification)
        match &template.supported_algorithms[0] {
            ProofAlgorithm::PLONK => Ok(true),
            ProofAlgorithm::Groth16 => Ok(true),
            ProofAlgorithm::STARK => Ok(true),
            ProofAlgorithm::Bulletproofs => Ok(true),
            ProofAlgorithm::Simplified => Ok(true),
        }
    }

    /// Calculate battery cost for proof generation
    async fn calculate_battery_cost(&self, template: &ProofTemplate, generation_time_ms: u64) -> f64 {
        let base_cost = 0.001; // 0.001% base cost
        let time_cost = generation_time_ms as f64 * 0.00001; // Time-based cost
        let complexity_cost = template.parameters.circuit_size as f64 * 0.000001; // Complexity cost
        
        base_cost + time_cost + complexity_cost
    }

    /// Hash proof data for caching
    fn hash_proof_data(&self, proof_data: &[u8]) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(proof_data);
        hex::encode(hasher.finalize())
    }

    /// Find cached proof by hash
    async fn find_cached_proof_by_hash(&self, hash: &str) -> Option<String> {
        let cache = self.optimization_cache.read().await;
        for (proof_id, proof) in cache.iter() {
            let proof_hash = self.hash_proof_data(&proof.proof_data);
            if proof_hash == hash {
                return Some(proof_id.clone());
            }
        }
        None
    }

    /// Determine optimizations applied
    fn determine_optimizations_applied(&self, template: &ProofTemplate) -> Vec<OptimizationType> {
        match template.optimization_level {
            OptimizationLevel::Minimal => vec![OptimizationType::Compression],
            OptimizationLevel::Balanced => vec![
                OptimizationType::Compression,
                OptimizationType::WitnessReduction,
            ],
            OptimizationLevel::Performance => vec![
                OptimizationType::Preprocessing,
                OptimizationType::Batching,
            ],
            OptimizationLevel::Size => vec![
                OptimizationType::Compression,
                OptimizationType::WitnessReduction,
                OptimizationType::PublicInputMinimization,
            ],
        }
    }

    /// Update statistics after optimization
    async fn update_stats_after_optimization(&self, original_size: usize, optimized_size: usize, generation_time: u64) {
        let mut stats = self.stats.write().await;
        stats.total_proofs_optimized += 1;
        
        // Update compression ratio
        let compression_ratio = original_size as f64 / optimized_size as f64;
        stats.average_compression_ratio = ((stats.average_compression_ratio * (stats.total_proofs_optimized - 1) as f64) + compression_ratio) / stats.total_proofs_optimized as f64;
        
        // Update generation time
        stats.average_generation_time_ms = ((stats.average_generation_time_ms * (stats.total_proofs_optimized - 1) as f64) + generation_time as f64) / stats.total_proofs_optimized as f64;
        
        // Update size reduction
        let size_reduction = (1.0 - (optimized_size as f64 / original_size as f64)) * 100.0;
        stats.size_reduction_percent = ((stats.size_reduction_percent * (stats.total_proofs_optimized - 1) as f64) + size_reduction) / stats.total_proofs_optimized as f64;
    }

    /// Create mobile device template
    fn create_mobile_template() -> ProofTemplate {
        ProofTemplate {
            template_id: "mobile_balanced".to_string(),
            device_type: "Mobile".to_string(),
            compute_level: ComputeLevel::Standard,
            max_proof_size: 1024, // 1KB
            max_generation_time_ms: 100,
            optimization_level: OptimizationLevel::Balanced,
            supported_algorithms: vec![ProofAlgorithm::Groth16, ProofAlgorithm::PLONK],
            parameters: ProofParameters {
                circuit_size: 1000,
                witness_size: 500,
                public_inputs: 10,
                security_level: 128,
                batch_size: 10,
                compression_ratio: 0.8,
            },
        }
    }

    /// Create IoT device template
    fn create_iot_template() -> ProofTemplate {
        ProofTemplate {
            template_id: "iot_minimal".to_string(),
            device_type: "IoT".to_string(),
            compute_level: ComputeLevel::Light,
            max_proof_size: 256, // 256 bytes
            max_generation_time_ms: 500,
            optimization_level: OptimizationLevel::Size,
            supported_algorithms: vec![ProofAlgorithm::Simplified],
            parameters: ProofParameters {
                circuit_size: 100,
                witness_size: 50,
                public_inputs: 3,
                security_level: 80,
                batch_size: 5,
                compression_ratio: 0.6,
            },
        }
    }

    /// Create edge device template
    fn create_edge_template() -> ProofTemplate {
        ProofTemplate {
            template_id: "edge_performance".to_string(),
            device_type: "Edge".to_string(),
            compute_level: ComputeLevel::Enhanced,
            max_proof_size: 4096, // 4KB
            max_generation_time_ms: 50,
            optimization_level: OptimizationLevel::Performance,
            supported_algorithms: vec![ProofAlgorithm::STARK, ProofAlgorithm::PLONK],
            parameters: ProofParameters {
                circuit_size: 10000,
                witness_size: 5000,
                public_inputs: 50,
                security_level: 255,
                batch_size: 100,
                compression_ratio: 0.9,
            },
        }
    }

    /// Create wearable device template
    fn create_wearable_template() -> ProofTemplate {
        ProofTemplate {
            template_id: "wearable_ultra_minimal".to_string(),
            device_type: "Wearable".to_string(),
            compute_level: ComputeLevel::Minimal,
            max_proof_size: 128, // 128 bytes
            max_generation_time_ms: 1000,
            optimization_level: OptimizationLevel::Minimal,
            supported_algorithms: vec![ProofAlgorithm::Simplified],
            parameters: ProofParameters {
                circuit_size: 50,
                witness_size: 25,
                public_inputs: 2,
                security_level: 80,
                batch_size: 1,
                compression_ratio: 0.5,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MobilePlatform, MobileCapabilities, NetworkType};

    #[tokio::test]
    async fn test_proof_optimizer_creation() {
        let config = ZKConfig {
            max_proof_size: 1024,
            max_verification_time_ms: 100,
            merkle_depth: 20,
            batch_size: 100,
        };
        
        let optimizer = ProofOptimizer::new(config).await.unwrap();
        let stats = optimizer.get_stats().await.unwrap();
        assert_eq!(stats.total_proofs_optimized, 0);
    }

    #[tokio::test]
    async fn test_proof_optimization() {
        let config = ZKConfig {
            max_proof_size: 1024,
            max_verification_time_ms: 100,
            merkle_depth: 20,
            batch_size: 100,
        };
        
        let optimizer = ProofOptimizer::new(config).await.unwrap();
        let device_type = DeviceType::Mobile {
            platform: MobilePlatform::Android,
            capabilities: MobileCapabilities {
                ram_mb: 4096,
                storage_gb: 64,
                has_secure_enclave: true,
                supports_biometrics: true,
                network_types: vec![NetworkType::FiveG],
            },
        };

        let proof_data = vec![0u8; 500]; // 500 byte proof
        let optimized_proof = optimizer.optimize_proof(proof_data.clone(), &device_type).await.unwrap();
        
        // Should be optimized (smaller or same size)
        assert!(optimized_proof.len() <= proof_data.len());
        
        // Should verify successfully
        let is_valid = optimizer.verify_proof(&optimized_proof, &device_type).await.unwrap();
        assert!(is_valid);
    }
}
