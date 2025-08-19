//! StepReceipt Integration for DockLock and ENC Clusters
//! 
//! This module implements the critical missing piece: automatic StepReceipt creation
//! for every DockLock container action and ENC cluster operation. This is the foundation
//! of the PoE pipeline: StepReceipt → LogBlock → PoE.

use crate::error::{DockLockError, DockLockResult};
use bpi_math::{
    receipts::{ReceiptFactory, ReceiptType, ResourceUsage, ClusterState, FinalityStatus, EconomyOperation},
    proofs::{ProofOfAction, ProofOfExecution, ProofOfHistory, ProofOfTransact, ProofOfGold},
    Hash, MathError,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, warn};
use uuid::Uuid;

/// StepReceipt generator for DockLock operations
#[derive(Debug)]
pub struct DockLockStepReceiptGenerator {
    /// Receipt storage
    receipts: Arc<RwLock<Vec<ReceiptType>>>,
    /// Operation counter for unique IDs
    operation_counter: Arc<RwLock<u64>>,
    /// Configuration
    config: StepReceiptConfig,
}

/// StepReceipt generator for ENC Cluster operations
#[derive(Debug)]
pub struct EncClusterStepReceiptGenerator {
    /// Receipt storage
    receipts: Arc<RwLock<Vec<ReceiptType>>>,
    /// Operation counter for unique IDs
    operation_counter: Arc<RwLock<u64>>,
    /// Cluster ID
    cluster_id: String,
    /// Configuration
    config: StepReceiptConfig,
}

/// Configuration for StepReceipt generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepReceiptConfig {
    /// Enable receipt generation
    pub enabled: bool,
    /// Maximum receipts to store in memory
    pub max_receipts: usize,
    /// Auto-flush interval in seconds
    pub flush_interval_secs: u64,
    /// Include detailed resource usage
    pub detailed_resources: bool,
    /// Enable cryptographic proofs
    pub enable_proofs: bool,
}

impl Default for StepReceiptConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_receipts: 10000,
            flush_interval_secs: 60,
            detailed_resources: true,
            enable_proofs: true,
        }
    }
}

/// DockLock operation types that generate StepReceipts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DockLockOperation {
    ContainerStart { container_id: String, image: String },
    ContainerStop { container_id: String },
    ContainerExec { container_id: String, command: Vec<String> },
    VolumeMount { container_id: String, volume: String, mount_point: String },
    NetworkConnect { container_id: String, network: String },
    ResourceLimit { container_id: String, cpu: Option<f64>, memory: Option<u64> },
    SecurityPolicy { container_id: String, policy: String },
    HealthCheck { container_id: String, status: String },
}

/// ENC Cluster operation types that generate StepReceipts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncClusterOperation {
    NodeAddition {
        node_id: String,
        node_type: String,
    },
    NodeJoin {
        node_id: String,
        cluster_id: String,
    },
    WorkloadScheduling {
        workload_id: String,
        target_node: String,
    },
    ResourceOptimization {
        optimization_type: String,
        affected_nodes: Vec<String>,
    },
    CryptographicVerification {
        workload_id: String,
        verification_type: String,
    },
    DeterministicExecution {
        workload_id: String,
        execution_hash: String,
    },
    BftConsensusScheduling {
        consensus_round: u64,
        participating_nodes: Vec<String>,
    },
    ZeroTrustEnforcement {
        workload_id: String,
        security_policies: Vec<String>,
    },
    DAppDeployment {
        dapp_id: String,
        microservices_count: u32,
    },
    MicroserviceDeployment {
        service_id: String,
        service_type: String,
    },
    OrchestrationRental {
        enterprise_id: String,
        rental_duration: u32,
        capacity_requested: u32,
    },
    AuditReportGeneration {
        enterprise_id: String,
        report_type: String,
    },
}

impl DockLockStepReceiptGenerator {
    /// Create new DockLock StepReceipt generator
    pub fn new(config: StepReceiptConfig) -> Self {
        Self {
            receipts: Arc::new(RwLock::new(Vec::new())),
            operation_counter: Arc::new(RwLock::new(0)),
            config,
        }
    }

    /// Generate StepReceipt for DockLock operation
    pub fn generate_step_receipt(
        &self,
        operation: DockLockOperation,
        resource_usage: ResourceUsage,
    ) -> DockLockResult<ReceiptType> {
        if !self.config.enabled {
            return Err(DockLockError::ConfigurationError("StepReceipt generation disabled".to_string()));
        }

        // Generate unique receipt ID
        let receipt_id = {
            let mut counter = self.operation_counter.write().map_err(|_| {
                DockLockError::InternalError("Failed to acquire operation counter lock".to_string())
            })?;
            *counter += 1;
            format!("docklock-{:016x}", *counter)
        };

        // Get current timestamp
        let _timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| DockLockError::InternalError(format!("Time error: {}", e)))?
            .as_millis() as u64;

        // Create proof of action
        let proof_of_action = self.create_proof_of_action(&operation, &resource_usage)?;

        // Extract container ID and operation string
        let (container_id, operation_str) = self.extract_operation_details(&operation);

        // Create DockLock receipt
        let receipt = ReceiptFactory::create_docklock_receipt(
            container_id,
            operation_str,
            proof_of_action,
            resource_usage,
        );

        let receipt_type = ReceiptType::DockLock(receipt);

        // Store receipt
        {
            let mut receipts = self.receipts.write().map_err(|_| {
                DockLockError::InternalError("Failed to acquire receipts lock".to_string())
            })?;
            receipts.push(receipt_type.clone());

            // Trim if exceeding max receipts
            if receipts.len() > self.config.max_receipts {
                let len = receipts.len(); receipts.drain(0..len - self.config.max_receipts);
            }
        }

        info!(
            receipt_id = %receipt_id,
            operation = ?operation,
            "Generated DockLock StepReceipt"
        );

        Ok(receipt_type)
    }

    /// Create proof of action for DockLock operation
    fn create_proof_of_action(
        &self,
        operation: &DockLockOperation,
        resource_usage: &ResourceUsage,
    ) -> DockLockResult<ProofOfAction> {
        if !self.config.enable_proofs {
            // Create a minimal proof when proofs are disabled
            use std::collections::HashMap;
            use bpi_math::proofs::{ProofSystem, ActionType};
            
            let container_id = self.extract_operation_details(operation).0;
            let action_type = ActionType::Start; // Default action type
            let metadata = HashMap::new();
            
            return ProofOfAction::generate_proof((container_id, action_type, metadata))
                .map_err(|e| DockLockError::InternalError(format!("Proof generation failed: {:?}", e)));
        }

        // Create deterministic proof based on operation and resources
        use std::collections::HashMap;
        use bpi_math::proofs::{ProofSystem, ActionType};
        
        let (container_id, _) = self.extract_operation_details(operation);
        let action_type = ActionType::Start; // Map operation to action type
        
        let mut metadata = HashMap::new();
        metadata.insert("cpu".to_string(), resource_usage.cpu_time.to_string());
        metadata.insert("memory".to_string(), resource_usage.memory_peak.to_string());
        metadata.insert("network".to_string(), resource_usage.network_bytes.to_string());
        metadata.insert("storage".to_string(), resource_usage.storage_bytes.to_string());
        
        ProofOfAction::generate_proof((container_id, action_type, metadata))
            .map_err(|e| DockLockError::InternalError(format!("Proof generation failed: {:?}", e)))
    }

    /// Extract container ID and operation string from operation
    fn extract_operation_details(&self, operation: &DockLockOperation) -> (String, String) {
        match operation {
            DockLockOperation::ContainerStart { container_id, image } => {
                (container_id.clone(), format!("start:{}", image))
            }
            DockLockOperation::ContainerStop { container_id } => {
                (container_id.clone(), "stop".to_string())
            }
            DockLockOperation::ContainerExec { container_id, command } => {
                (container_id.clone(), format!("exec:{}", command.join(" ")))
            }
            DockLockOperation::VolumeMount { container_id, volume, mount_point } => {
                (container_id.clone(), format!("mount:{}:{}", volume, mount_point))
            }
            DockLockOperation::NetworkConnect { container_id, network } => {
                (container_id.clone(), format!("network:{}", network))
            }
            DockLockOperation::ResourceLimit { container_id, cpu, memory } => {
                (container_id.clone(), format!("limit:cpu={:?}:mem={:?}", cpu, memory))
            }
            DockLockOperation::SecurityPolicy { container_id, policy } => {
                (container_id.clone(), format!("security:{}", policy))
            }
            DockLockOperation::HealthCheck { container_id, status } => {
                (container_id.clone(), format!("health:{}", status))
            }
        }
    }

    /// Get all generated receipts
    pub fn get_receipts(&self) -> DockLockResult<Vec<ReceiptType>> {
        let receipts = self.receipts.read().map_err(|_| {
            DockLockError::InternalError("Failed to acquire receipts lock".to_string())
        })?;
        Ok(receipts.clone())
    }

    /// Clear all receipts
    pub fn clear_receipts(&self) -> DockLockResult<()> {
        let mut receipts = self.receipts.write().map_err(|_| {
            DockLockError::InternalError("Failed to acquire receipts lock".to_string())
        })?;
        receipts.clear();
        Ok(())
    }

    /// Get receipt count
    pub fn get_receipt_count(&self) -> usize {
        self.receipts.read().map(|r| r.len()).unwrap_or(0)
    }
}

impl EncClusterStepReceiptGenerator {
    /// Create new ENC Cluster StepReceipt generator
    pub fn new(cluster_id: String, config: StepReceiptConfig) -> Self {
        Self {
            receipts: Arc::new(RwLock::new(Vec::new())),
            operation_counter: Arc::new(RwLock::new(0)),
            cluster_id,
            config,
        }
    }

    /// Generate StepReceipt for ENC Cluster operation
    pub fn generate_step_receipt(
        &self,
        operation: EncClusterOperation,
        node_id: String,
        cluster_state: ClusterState,
    ) -> DockLockResult<ReceiptType> {
        if !self.config.enabled {
            return Err(DockLockError::ConfigurationError("StepReceipt generation disabled".to_string()));
        }

        // Generate unique receipt ID
        let receipt_id = {
            let mut counter = self.operation_counter.write().map_err(|_| {
                DockLockError::InternalError("Failed to acquire operation counter lock".to_string())
            })?;
            *counter += 1;
            format!("enc-{}-{:016x}", self.cluster_id, *counter)
        };

        // Get current timestamp
        let _timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| DockLockError::InternalError(format!("Time error: {}", e)))?
            .as_millis() as u64;

        // Create proof of history
        let proof_of_history = self.create_proof_of_history(&operation, &cluster_state)?;

        // Extract operation string
        let operation_str = format!("{:?}", operation);

        // Create cluster receipt
        let receipt = ReceiptFactory::create_cluster_receipt(
            self.cluster_id.clone(),
            node_id,
            operation_str,
            proof_of_history,
            cluster_state,
        );

        let receipt_type = ReceiptType::Cluster(receipt);

        // Store receipt
        {
            let mut receipts = self.receipts.write().map_err(|_| {
                DockLockError::InternalError("Failed to acquire receipts lock".to_string())
            })?;
            receipts.push(receipt_type.clone());

            // Trim if exceeding max receipts
            if receipts.len() > self.config.max_receipts {
                let len = receipts.len(); receipts.drain(0..len - self.config.max_receipts);
            }
        }

        info!(
            receipt_id = %receipt_id,
            cluster_id = %self.cluster_id,
            operation = ?operation,
            "Generated ENC Cluster StepReceipt"
        );

        Ok(receipt_type)
    }

    /// Create proof of history for ENC Cluster operation
    fn create_proof_of_history(
        &self,
        operation: &EncClusterOperation,
        cluster_state: &ClusterState,
    ) -> DockLockResult<ProofOfHistory> {
        use bpi_math::proofs::ProofSystem;
        use std::time::{SystemTime, UNIX_EPOCH};
        
        // Generate sequence number from current time
        let sequence_number = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| DockLockError::InternalError(format!("Time error: {}", e)))?
            .as_millis() as u64;
        
        // Create previous hash from cluster state
        let state_data = bincode::serialize(cluster_state)
            .map_err(|e| DockLockError::SerializationError(format!("State serialization: {}", e)))?;
        let prev_hash = blake3::hash(&state_data).into();
        
        // Create operation data
        let operation_data = bincode::serialize(operation)
            .map_err(|e| DockLockError::SerializationError(format!("Operation serialization: {}", e)))?;
        
        ProofOfHistory::generate_proof((sequence_number, prev_hash, operation_data))
            .map_err(|e| DockLockError::InternalError(format!("Proof generation failed: {:?}", e)))
    }

    /// Extract operation string from operation
    fn format_enc_cluster_operation(&self, operation: &EncClusterOperation) -> String {
        match operation {
            EncClusterOperation::NodeAddition { node_id, node_type } => {
                format!("ENC_NODE_ADDITION:{}:{}", node_id, node_type)
            }
            EncClusterOperation::WorkloadScheduling { workload_id, target_node } => {
                format!("ENC_WORKLOAD_SCHEDULING:{}:{}", workload_id, target_node)
            }
            EncClusterOperation::ResourceOptimization { optimization_type, affected_nodes } => {
                format!("ENC_RESOURCE_OPTIMIZATION:{}:{:?}", optimization_type, affected_nodes)
            }
            EncClusterOperation::CryptographicVerification { workload_id, verification_type } => {
                format!("ENC_CRYPTO_VERIFICATION:{}:{}", workload_id, verification_type)
            }
            EncClusterOperation::DeterministicExecution { workload_id, execution_hash } => {
                format!("ENC_DETERMINISTIC_EXECUTION:{}:{}", workload_id, execution_hash)
            }
            EncClusterOperation::BftConsensusScheduling { consensus_round, participating_nodes } => {
                format!("ENC_BFT_CONSENSUS_SCHEDULING:{}:{:?}", consensus_round, participating_nodes)
            }
            EncClusterOperation::ZeroTrustEnforcement { workload_id, security_policies } => {
                format!("ENC_ZERO_TRUST_ENFORCEMENT:{}:{:?}", workload_id, security_policies)
            }
            EncClusterOperation::DAppDeployment { dapp_id, microservices_count } => {
                format!("ENC_DAPP_DEPLOYMENT:{}:{}", dapp_id, microservices_count)
            }
            EncClusterOperation::MicroserviceDeployment { service_id, service_type } => {
                format!("ENC_MICROSERVICE_DEPLOYMENT:{}:{}", service_id, service_type)
            }
            EncClusterOperation::OrchestrationRental { enterprise_id, rental_duration, capacity_requested } => {
                format!("ENC_ORCHESTRATION_RENTAL:{}:{}:{}", enterprise_id, rental_duration, capacity_requested)
            }
            EncClusterOperation::NodeJoin { node_id, cluster_id } => {
                format!("ENC_NODE_JOIN:{}:{}", node_id, cluster_id)
            }
            EncClusterOperation::AuditReportGeneration { enterprise_id, report_type } => {
                format!("ENC_AUDIT_REPORT_GENERATION:{}:{}", enterprise_id, report_type)
            }
        }
    }

    /// Get the current number of stored receipts
    pub fn get_receipt_count(&self) -> DockLockResult<usize> {
        let receipts = self.receipts.read().map_err(|_| {
            DockLockError::InternalError("Failed to acquire receipts lock".to_string())
        })?;
        Ok(receipts.len())
    }
}

/// Helper function to create resource usage from system metrics
pub fn create_resource_usage_from_metrics(
    cpu_time_ms: u64,
    memory_bytes: u64,
    network_bytes: u64,
    storage_bytes: u64,
) -> ResourceUsage {
    ResourceUsage {
        cpu_time: cpu_time_ms,
        memory_peak: memory_bytes,
        network_bytes,
        storage_bytes,
    }
}

/// Helper function to create cluster state
pub fn create_cluster_state(
    active_nodes: u32,
    total_capacity: u64,
    used_capacity: u64,
    health_score: f64,
) -> ClusterState {
    ClusterState {
        active_nodes,
        total_capacity,
        used_capacity,
        health_score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docklock_step_receipt_generator() {
        let config = StepReceiptConfig::default();
        let generator = DockLockStepReceiptGenerator::new(config);

        let operation = DockLockOperation::ContainerStart {
            container_id: "test-container".to_string(),
            image: "nginx:latest".to_string(),
        };

        let resource_usage = create_resource_usage_from_metrics(1000, 1024 * 1024, 500, 2048);

        let receipt = generator.generate_step_receipt(operation, resource_usage).unwrap();
        
        match receipt {
            ReceiptType::DockLock(docklock_receipt) => {
                assert_eq!(docklock_receipt.container_id, "test-container");
                assert!(docklock_receipt.operation.contains("start:nginx:latest"));
            }
            _ => panic!("Expected DockLock receipt"),
        }

        assert_eq!(generator.get_receipt_count(), 1);
    }

    #[test]
    fn test_enc_cluster_step_receipt_generator() {
        let config = StepReceiptConfig::default();
        let generator = EncClusterStepReceiptGenerator::new("test-cluster".to_string(), config);

        let operation = EncClusterOperation::NodeJoin {
            node_id: "node-1".to_string(),
            cluster_id: "test-cluster".to_string(),
        };

        let cluster_state = create_cluster_state(3, 1000, 500, 0.95);

        let receipt = generator.generate_step_receipt(
            operation,
            "node-1".to_string(),
            cluster_state,
        ).unwrap();
        
        match receipt {
            ReceiptType::Cluster(cluster_receipt) => {
                assert_eq!(cluster_receipt.cluster_id, "test-cluster");
                assert_eq!(cluster_receipt.node_id, "node-1");
                assert!(cluster_receipt.operation.contains("NodeJoin"));
            }
            _ => panic!("Expected Cluster receipt"),
        }

        assert_eq!(generator.get_receipt_count().unwrap(), 1);
    }

    #[test]
    fn test_receipt_generation_disabled() {
        let mut config = StepReceiptConfig::default();
        config.enabled = false;
        
        let generator = DockLockStepReceiptGenerator::new(config);

        let operation = DockLockOperation::ContainerStop {
            container_id: "test-container".to_string(),
        };

        let resource_usage = create_resource_usage_from_metrics(500, 512 * 1024, 100, 1024);

        let result = generator.generate_step_receipt(operation, resource_usage);
        assert!(result.is_err());
    }

    #[test]
    fn test_receipt_count_limit() {
        let mut config = StepReceiptConfig::default();
        config.max_receipts = 2;
        
        let generator = DockLockStepReceiptGenerator::new(config);

        // Generate 3 receipts
        for i in 0..3 {
            let operation = DockLockOperation::ContainerStart {
                container_id: format!("container-{}", i),
                image: "test:latest".to_string(),
            };
            let resource_usage = create_resource_usage_from_metrics(100, 1024, 50, 512);
            generator.generate_step_receipt(operation, resource_usage).unwrap();
        }

        // Should only keep the last 2 receipts
        assert_eq!(generator.get_receipt_count(), 2);
    }
}
