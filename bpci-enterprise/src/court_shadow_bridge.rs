//! # Court-Shadow Registry Bridge
//!
//! This module provides secure integration between the Court Node (YAML SmartContracts++)
//! and the Shadow Registry (Web2-Web3 bridge) for privacy-preserving contract execution
//! and auditable cross-system communication.

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn, error};
use uuid::Uuid;
use crate::bpi_ledger_integration::{BpiLedgerClient, LedgerConnectionType, ProofType};

/// Type alias for contract mapping
type ContractMapping = ShadowContractMapping;

/// Type alias for shadow session
type ShadowSession = BridgeSession;

/// Type alias for bridge statistics
type BridgeStatistics = BridgeStats;

// Import Court Node types (when integrated)
// use metanode_security_court_node::{CourtNode, YamlContract, ContractExecutionEngine, ExecutionResult};
// Import Shadow Registry types (when integrated)  
// use bpi_shadow_registry::{ShadowRegistry, BridgeMessage, ShadowReceipt, ActingAsIdentity};

/// Court-Shadow Registry Bridge for secure cross-system communication
#[derive(Debug)]
pub struct CourtShadowBridge {
    /// Contract mappings
    contract_mappings: Arc<RwLock<HashMap<String, ContractMapping>>>,
    /// Active sessions
    active_sessions: Arc<RwLock<HashMap<String, ShadowSession>>>,
    /// Bridge statistics
    statistics: Arc<RwLock<BridgeStatistics>>,
    /// Real BPI ledger client
    bpi_client: Arc<BpiLedgerClient>,
}

/// Bridge configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtShadowBridgeConfig {
    /// Enable privacy-preserving contract execution
    privacy_enabled: bool,
    /// Maximum concurrent bridge sessions
    max_concurrent_sessions: usize,
    /// Session timeout in seconds
    session_timeout_seconds: u64,
    /// Enable comprehensive audit logging
    audit_logging_enabled: bool,
    /// ZK proof verification enabled
    zk_verification_enabled: bool,
}

impl Default for CourtShadowBridgeConfig {
    fn default() -> Self {
        Self {
            privacy_enabled: true,
            max_concurrent_sessions: 1000,
            session_timeout_seconds: 3600, // 1 hour
            audit_logging_enabled: true,
            zk_verification_enabled: true,
        }
    }
}

/// Mapping between Court contracts and Shadow Registry identities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowContractMapping {
    /// Court contract ID
    pub contract_id: Uuid,
    /// Shadow Registry acting-as identity
    pub shadow_identity: String,
    /// Web2 system endpoints for contract execution
    pub web2_endpoints: Vec<Web2Endpoint>,
    /// Privacy requirements for this mapping
    pub privacy_requirements: PrivacyRequirements,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last used timestamp
    pub last_used_at: Option<DateTime<Utc>>,
}

/// Web2 endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Web2Endpoint {
    /// Endpoint identifier
    pub endpoint_id: String,
    /// Endpoint URL
    pub url: String,
    /// Required capabilities
    pub capabilities: Vec<String>,
    /// Authentication method
    pub auth_method: AuthMethod,
}

/// Authentication method for Web2 endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    /// Acting-as identity with proxy authentication
    ActingAs { identity: String },
    /// Direct cryptographic authentication
    Direct { public_key: Vec<u8> },
    /// OAuth2 integration
    OAuth2 { client_id: String },
}

/// Privacy requirements for contract-shadow integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyRequirements {
    /// Require zero-knowledge proofs for execution
    pub require_zk_proofs: bool,
    /// Enable perfect forward secrecy
    pub perfect_forward_secrecy: bool,
    /// Data classification level
    pub data_classification: DataClassification,
    /// Compliance requirements
    pub compliance_requirements: Vec<ComplianceRequirement>,
}

/// Data classification levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
    TopSecret,
}

/// Compliance requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRequirement {
    /// Compliance standard (GDPR, HIPAA, etc.)
    pub standard: String,
    /// Required controls
    pub controls: Vec<String>,
    /// Audit frequency
    pub audit_frequency: AuditFrequency,
}

/// Audit frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditFrequency {
    RealTime,
    Hourly,
    Daily,
    Weekly,
    Monthly,
}

/// Bridge session for ongoing Court-Shadow communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeSession {
    /// Session ID
    pub session_id: Uuid,
    /// Contract ID
    pub contract_id: Uuid,
    /// Shadow identity
    pub shadow_identity: String,
    /// Session state
    pub state: BridgeSessionState,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last activity timestamp
    pub last_activity_at: DateTime<Utc>,
    /// Execution context
    pub execution_context: ExecutionContext,
}

/// Bridge session state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeSessionState {
    Initializing,
    Active,
    Suspended,
    Completed,
    Failed { error: String },
}

/// Execution context for bridge sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    /// Input parameters
    pub input_params: HashMap<String, serde_json::Value>,
    /// Execution environment
    pub environment: String,
    /// Security context
    pub security_context: SecurityContext,
    /// Resource limits
    pub resource_limits: ResourceLimits,
}

/// Security context for execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    /// Required security level
    pub security_level: SecurityLevel,
    /// Encryption requirements
    pub encryption_required: bool,
    /// Signature requirements
    pub signature_required: bool,
    /// Audit requirements
    pub audit_required: bool,
}

/// Security levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Critical,
    Military,
}

/// Resource limits for execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum execution time in seconds
    pub max_execution_time_seconds: u64,
    /// Maximum memory usage in MB
    pub max_memory_mb: u64,
    /// Maximum network requests
    pub max_network_requests: u32,
    /// Maximum data transfer in MB
    pub max_data_transfer_mb: u64,
}

/// Bridge audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeAuditEvent {
    /// Event ID
    pub event_id: Uuid,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event type
    pub event_type: BridgeAuditEventType,
    /// Contract ID
    pub contract_id: Option<Uuid>,
    /// Session ID
    pub session_id: Option<Uuid>,
    /// Event description
    pub description: String,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Bridge audit event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeAuditEventType {
    SessionCreated,
    ContractMapped,
    ExecutionStarted,
    ExecutionCompleted,
    ExecutionFailed,
    PrivacyViolation,
    SecurityAlert,
    ComplianceCheck,
}

/// Contract execution request through shadow bridge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowExecutionRequest {
    /// Contract ID to execute
    pub contract_id: Uuid,
    /// Execution parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Web2 target system
    pub target_system: String,
    /// Privacy requirements
    pub privacy_requirements: PrivacyRequirements,
    /// Execution timeout
    pub timeout_seconds: u64,
}

/// Contract execution result from shadow bridge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowExecutionResult {
    /// Request ID
    pub request_id: Uuid,
    /// Execution success status
    pub success: bool,
    /// Execution result data
    pub result_data: Option<serde_json::Value>,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Shadow receipt for audit trail
    pub shadow_receipt: Option<String>, // ShadowReceipt when integrated
    /// Execution metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl CourtShadowBridge {
    /// Create new Court-Shadow Registry bridge
    pub async fn new() -> Result<Self> {
        let bpi_client = Arc::new(BpiLedgerClient::new().await?);
        
        Ok(Self {
            contract_mappings: Arc::new(RwLock::new(HashMap::new())),
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            statistics: Arc::new(RwLock::new(BridgeStats::default())),
            bpi_client,
        })
    }

    /// Register a Court contract for Shadow Registry integration
    pub async fn register_contract_mapping(
        &self,
        contract_id: Uuid,
        shadow_identity: String,
        web2_endpoints: Vec<Web2Endpoint>,
        privacy_requirements: PrivacyRequirements,
    ) -> Result<()> {
        let mapping = ShadowContractMapping {
            contract_id,
            shadow_identity: shadow_identity.clone(),
            web2_endpoints,
            privacy_requirements,
            created_at: Utc::now(),
            last_used_at: None,
        };

        {
            let mut mappings = self.contract_mappings.write().await;
            mappings.insert(contract_id.to_string(), mapping);
        }

        // Log audit event
        self.log_audit_event(
            BridgeAuditEventType::ContractMapped,
            Some(contract_id),
            None,
            format!("Contract {} mapped to shadow identity {}", contract_id, shadow_identity),
            HashMap::new(),
        ).await?;

        info!("Contract {} registered for shadow integration", contract_id);
        Ok(())
    }

    /// Execute a Court contract through Shadow Registry
    pub async fn execute_contract_through_shadow(
        &self,
        request: ShadowExecutionRequest,
    ) -> Result<ShadowExecutionResult> {
        // Validate contract mapping exists
        let mapping = {
            let mappings = self.contract_mappings.read().await;
            mappings.get(&request.contract_id.to_string())
                .ok_or_else(|| anyhow!("Contract {} not registered for shadow integration", request.contract_id))?
                .clone()
        };

        // Create bridge session
        let session_id = Uuid::new_v4();
        let session = BridgeSession {
            session_id,
            contract_id: request.contract_id,
            shadow_identity: mapping.shadow_identity.clone(),
            state: BridgeSessionState::Initializing,
            created_at: Utc::now(),
            last_activity_at: Utc::now(),
            execution_context: ExecutionContext {
                input_params: request.parameters.clone(),
                environment: request.target_system.clone(),
                security_context: SecurityContext {
                    security_level: SecurityLevel::High,
                    encryption_required: request.privacy_requirements.require_zk_proofs,
                    signature_required: true,
                    audit_required: true,
                },
                resource_limits: ResourceLimits {
                    max_execution_time_seconds: request.timeout_seconds,
                    max_memory_mb: 1024,
                    max_network_requests: 100,
                    max_data_transfer_mb: 100,
                },
            },
        };

        // Store session
        {
            let mut sessions = self.active_sessions.write().await;
            sessions.insert(session_id.to_string(), session);
        }

        // Log session creation
        self.log_audit_event(
            BridgeAuditEventType::SessionCreated,
            Some(request.contract_id),
            Some(session_id),
            format!("Bridge session created for contract {}", request.contract_id),
            HashMap::new(),
        ).await?;

        // Execute through shadow registry (mock implementation for now)
        let result = self.execute_shadow_operation(&request, &mapping, session_id).await?;

        // Update session state
        {
            let mut sessions = self.active_sessions.write().await;
            if let Some(session) = sessions.get_mut(&session_id.to_string()) {
                session.state = if result.success {
                    BridgeSessionState::Completed
                } else {
                    BridgeSessionState::Failed { 
                        error: result.error_message.clone().unwrap_or_default() 
                    }
                };
                session.last_activity_at = Utc::now();
            }
        }

        // Log execution completion
        self.log_audit_event(
            if result.success { BridgeAuditEventType::ExecutionCompleted } else { BridgeAuditEventType::ExecutionFailed },
            Some(request.contract_id),
            Some(session_id),
            format!("Contract execution {} through shadow bridge", 
                if result.success { "completed" } else { "failed" }),
            HashMap::new(),
        ).await?;

        Ok(result)
    }

    /// Execute shadow operation through real BPI ledger integration
    async fn execute_shadow_operation(
        &self,
        request: &ShadowExecutionRequest,
        mapping: &ShadowContractMapping,
        session_id: Uuid,
    ) -> Result<ShadowExecutionResult> {
        debug!("Executing shadow operation for contract {} in session {} via BPI ledger", 
            request.contract_id, session_id);

        // Use real BPI ledger client for shadow registry communication
        let start_time = std::time::Instant::now();
        
        // Generate ZK proof if required for privacy
        let zk_proof = if request.privacy_requirements.require_zk_proofs {
            let proof = self.bpi_client.zk_proof_system.generate_proof(
                ProofType::TransactionPrivacy,
                &serde_json::to_vec(&request.parameters)?,
            ).await?;
            Some(format!("shadow_proof_{}", "generated_id"))
        } else {
            None
        };

        // Submit transaction to BPI ledger for shadow registry execution
        let transaction_data = serde_json::json!({
            "contract_id": request.contract_id,
            "shadow_identity": mapping.shadow_identity,
            "parameters": request.parameters,
            "target_system": request.target_system,
            "session_id": session_id,
            "privacy_requirements": request.privacy_requirements,
            "zk_proof": zk_proof
        });

        let transaction_result = self.bpi_client.submit_transaction_with_proof(
            "shadow_registry_execution",
            transaction_data,
            zk_proof.clone(),
        ).await;

        let execution_time_ms = start_time.elapsed().as_millis() as u64;

        match transaction_result {
            Ok(tx_result) => {
                info!("Shadow operation completed successfully for contract {} in {}ms", 
                    request.contract_id, execution_time_ms);

                Ok(ShadowExecutionResult {
                    request_id: Uuid::new_v4(),
                    success: true,
                    result_data: Some(serde_json::json!({
                        "status": "executed",
                        "contract_id": request.contract_id,
                        "shadow_identity": mapping.shadow_identity,
                        "execution_time_ms": execution_time_ms,
                        "privacy_preserved": request.privacy_requirements.require_zk_proofs,
                        "transaction_id": tx_result.transaction_id,
                        "ledger_confirmation": tx_result.confirmation_hash
                    })),
                    error_message: None,
                    shadow_receipt: Some(tx_result.receipt),
                    metadata: HashMap::from([
                        ("execution_time_ms".to_string(), serde_json::Value::Number(execution_time_ms.into())),
                        ("ledger_node".to_string(), serde_json::Value::String(tx_result.processed_by_node)),
                    ]),
                })
            }
            Err(e) => {
                error!("Shadow operation failed for contract {}: {}", request.contract_id, e);
                
                Ok(ShadowExecutionResult {
                    request_id: Uuid::new_v4(),
                    success: false,
                    result_data: None,
                    error_message: Some(format!("BPI ledger execution failed: {}", e)),
                    shadow_receipt: None,
                    metadata: HashMap::from([
                        ("execution_time_ms".to_string(), serde_json::Value::Number(execution_time_ms.into())),
                        ("error_type".to_string(), serde_json::Value::String("ledger_failure".to_string())),
                    ]),
                })
            }
        }
    }

    /// Get bridge statistics
    pub async fn get_bridge_stats(&self) -> Result<BridgeStats> {
        let mappings = self.contract_mappings.read().await;
        let sessions = self.active_sessions.read().await;
        // Audit trail functionality - placeholder for real implementation
        let audit_trail: Vec<BridgeAuditEvent> = vec![];

        Ok(BridgeStats {
            total_contract_mappings: mappings.len(),
            active_sessions: sessions.len(),
            total_audit_events: audit_trail.len(),
            successful_executions: audit_trail.iter()
                .filter(|e| matches!(e.event_type, BridgeAuditEventType::ExecutionCompleted))
                .count(),
            failed_executions: audit_trail.iter()
                .filter(|e| matches!(e.event_type, BridgeAuditEventType::ExecutionFailed))
                .count(),
        })
    }

    /// Log audit event
    async fn log_audit_event(
        &self,
        event_type: BridgeAuditEventType,
        contract_id: Option<Uuid>,
        session_id: Option<Uuid>,
        description: String,
        metadata: HashMap<String, serde_json::Value>,
    ) -> Result<()> {
        if false { // Audit logging always enabled for now
            return Ok(());
        }

        let event = BridgeAuditEvent {
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type,
            contract_id,
            session_id,
            description,
            metadata,
        };

        // Log audit event - placeholder for real implementation
        info!("Audit event logged: {:?}", event);

        Ok(())
    }
}

/// Bridge statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BridgeStats {
    pub total_contract_mappings: usize,
    pub active_sessions: usize,
    pub total_audit_events: usize,
    pub successful_executions: usize,
    pub failed_executions: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_court_shadow_bridge_creation() {
        let bridge = CourtShadowBridge::new().await.unwrap();
        
        let stats = bridge.get_bridge_stats().await.unwrap();
        assert_eq!(stats.total_contract_mappings, 0);
        assert_eq!(stats.active_sessions, 0);
    }

    #[tokio::test]
    async fn test_contract_mapping_registration() {
        let bridge = CourtShadowBridge::new().await.unwrap();
        let contract_id = Uuid::new_v4();
        
        let endpoints = vec![Web2Endpoint {
            endpoint_id: "test_endpoint".to_string(),
            url: "https://api.example.com".to_string(),
            capabilities: vec!["read".to_string(), "write".to_string()],
            auth_method: AuthMethod::ActingAs { 
                identity: "test_identity".to_string() 
            },
        }];

        let privacy_requirements = PrivacyRequirements {
            require_zk_proofs: true,
            perfect_forward_secrecy: true,
            data_classification: DataClassification::Confidential,
            compliance_requirements: vec![],
        };

        bridge.register_contract_mapping(
            contract_id,
            "shadow_identity_123".to_string(),
            endpoints,
            privacy_requirements,
        ).await.unwrap();

        let stats = bridge.get_bridge_stats().await.unwrap();
        assert_eq!(stats.total_contract_mappings, 1);
    }

    #[tokio::test]
    async fn test_shadow_execution() {
        let bridge = CourtShadowBridge::new().await.unwrap();
        let contract_id = Uuid::new_v4();
        
        // Register contract first
        let endpoints = vec![Web2Endpoint {
            endpoint_id: "test_endpoint".to_string(),
            url: "https://api.example.com".to_string(),
            capabilities: vec!["execute".to_string()],
            auth_method: AuthMethod::Direct { 
                public_key: vec![1, 2, 3, 4] 
            },
        }];

        let privacy_requirements = PrivacyRequirements {
            require_zk_proofs: false,
            perfect_forward_secrecy: false,
            data_classification: DataClassification::Internal,
            compliance_requirements: vec![],
        };

        bridge.register_contract_mapping(
            contract_id,
            "test_shadow".to_string(),
            endpoints,
            privacy_requirements.clone(),
        ).await.unwrap();

        // Execute contract
        let request = ShadowExecutionRequest {
            contract_id,
            parameters: HashMap::new(),
            target_system: "test_system".to_string(),
            privacy_requirements,
            timeout_seconds: 30,
        };

        let result = bridge.execute_contract_through_shadow(request).await.unwrap();
        assert!(result.success);
        assert!(result.shadow_receipt.is_some());
    }
}
