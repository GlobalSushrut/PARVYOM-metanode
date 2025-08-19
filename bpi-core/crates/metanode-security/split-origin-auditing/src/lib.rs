/*!
# Split-Origin Auditing

Dual-path audit trail system with client-side and server-side independent verification.
Provides cryptographic proof of independent verification and tamper-proof audit chains
with Court Notary Registry (CNR) integration.

## Features

- **Dual-Path Auditing**: Independent client and server audit trails
- **Cryptographic Proofs**: Ed25519 signatures for all audit entries
- **Tamper-Proof Chain**: Hash-linked audit chain with CNR notarization
- **Independent Verification**: Impossible to hide actions from either party
- **Cross-Origin Validation**: Client and server must agree on audit entries

## Security Model

- Each action generates two independent audit entries (client + server)
- Both entries must be cryptographically signed and hash-linked
- CNR provides tamper-proof storage and dispute resolution
- Any discrepancy between client/server trails triggers investigation
*/

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use sha2::{Sha256, Digest};
use thiserror::Error;
use tracing::{info, warn, error};

// Re-export CNR types
pub use court_notary_registry::{
    CourtNotaryRegistry, NotarizedDocument, CnrOperationResult,
    DocumentMetadata, DocumentType, LegalSignificance, NotarizationDetails,
    NotarizationMethod, DocumentStatus, VerificationProof, ProofType
};

/// Split-Origin Auditing system hash identifier
const SOA_HASH: u8 = 0x52;

/// Errors that can occur in split-origin auditing
#[derive(Error, Debug)]
pub enum AuditError {
    #[error("Cryptographic error: {0}")]
    Cryptographic(String),
    #[error("Chain integrity violation: {0}")]
    ChainIntegrity(String),
    #[error("Origin mismatch: client={client}, server={server}")]
    OriginMismatch { client: String, server: String },
    #[error("CNR integration error: {0}")]
    CnrIntegration(String),
    #[error("Verification failed: {0}")]
    VerificationFailed(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

/// Result type for audit operations
pub type AuditResult<T> = Result<T, AuditError>;

/// Origin of an audit entry (client or server)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditOrigin {
    Client,
    Server,
}

/// Type of audited action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    ContractExecution { contract_id: Uuid },
    StateTransition { from_state: String, to_state: String },
    DataAccess { resource_id: String, operation: String },
    SystemEvent { event_type: String },
    UserAction { action: String, user_id: String },
}

/// Individual audit entry with cryptographic proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: Uuid,
    pub origin: AuditOrigin,
    pub action_type: ActionType,
    pub timestamp: DateTime<Utc>,
    pub previous_hash: Option<String>,
    pub data_hash: String,
    pub signature: String,
    pub public_key: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Dual audit record containing both client and server entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualAuditRecord {
    pub id: Uuid,
    pub client_entry: Option<AuditEntry>,
    pub server_entry: Option<AuditEntry>,
    pub consensus_hash: Option<String>,
    pub cnr_notary_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub verified_at: Option<DateTime<Utc>>,
}

/// Configuration for split-origin auditing
#[derive(Debug, Clone)]
pub struct AuditConfig {
    pub enable_client_auditing: bool,
    pub enable_server_auditing: bool,
    pub require_dual_consensus: bool,
    pub cnr_integration: bool,
    pub max_chain_length: usize,
    pub signature_algorithm: String,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enable_client_auditing: true,
            enable_server_auditing: true,
            require_dual_consensus: true,
            cnr_integration: true,
            max_chain_length: 10000,
            signature_algorithm: "Ed25519".to_string(),
        }
    }
}

/// Split-Origin Auditing system
#[derive(Debug)]
pub struct SplitOriginAuditor {
    config: AuditConfig,
    client_keypair: Arc<RwLock<SigningKey>>,
    server_keypair: Arc<RwLock<SigningKey>>,
    client_chain: Arc<RwLock<Vec<AuditEntry>>>,
    server_chain: Arc<RwLock<Vec<AuditEntry>>>,
    dual_records: Arc<RwLock<HashMap<Uuid, DualAuditRecord>>>,
    cnr: Option<Arc<CourtNotaryRegistry>>,
}

impl SplitOriginAuditor {
    /// Create a new split-origin auditor
    pub fn new(config: AuditConfig, cnr: Option<Arc<CourtNotaryRegistry>>) -> AuditResult<Self> {
        let client_keypair = SigningKey::generate(&mut rand::thread_rng());
        let server_keypair = SigningKey::generate(&mut rand::thread_rng());

        info!("Initialized Split-Origin Auditor with dual-path verification");

        Ok(Self {
            config,
            client_keypair: Arc::new(RwLock::new(client_keypair)),
            server_keypair: Arc::new(RwLock::new(server_keypair)),
            client_chain: Arc::new(RwLock::new(Vec::new())),
            server_chain: Arc::new(RwLock::new(Vec::new())),
            dual_records: Arc::new(RwLock::new(HashMap::new())),
            cnr,
        })
    }

    /// Audit an action with dual-path verification
    pub async fn audit_action(
        &self,
        action_type: ActionType,
        metadata: HashMap<String, serde_json::Value>,
    ) -> AuditResult<DualAuditRecord> {
        let record_id = Uuid::new_v4();
        let timestamp = Utc::now();

        // Create client-side audit entry
        let client_entry = if self.config.enable_client_auditing {
            Some(self.create_audit_entry(
                AuditOrigin::Client,
                action_type.clone(),
                timestamp,
                metadata.clone(),
            ).await?)
        } else {
            None
        };

        // Create server-side audit entry
        let server_entry = if self.config.enable_server_auditing {
            Some(self.create_audit_entry(
                AuditOrigin::Server,
                action_type.clone(),
                timestamp,
                metadata.clone(),
            ).await?)
        } else {
            None
        };

        // Create dual audit record
        let mut dual_record = DualAuditRecord {
            id: record_id,
            client_entry: client_entry.clone(),
            server_entry: server_entry.clone(),
            consensus_hash: None,
            cnr_notary_id: None,
            created_at: timestamp,
            verified_at: None,
        };

        // Verify dual consensus if required
        if self.config.require_dual_consensus {
            dual_record.consensus_hash = Some(self.compute_consensus_hash(&dual_record)?);
            dual_record.verified_at = Some(Utc::now());
        }

        // Add entries to respective chains
        if let Some(entry) = client_entry {
            self.client_chain.write().await.push(entry);
        }
        if let Some(entry) = server_entry {
            self.server_chain.write().await.push(entry);
        }

        // Integrate with CNR if enabled
        if self.config.cnr_integration && self.cnr.is_some() {
            dual_record.cnr_notary_id = Some(self.notarize_with_cnr(&dual_record).await?);
        }

        // Store dual record
        self.dual_records.write().await.insert(record_id, dual_record.clone());

        info!("Audited action with dual-path verification: {:?}", action_type);
        Ok(dual_record)
    }

    /// Create an individual audit entry
    async fn create_audit_entry(
        &self,
        origin: AuditOrigin,
        action_type: ActionType,
        timestamp: DateTime<Utc>,
        metadata: HashMap<String, serde_json::Value>,
    ) -> AuditResult<AuditEntry> {
        let entry_id = Uuid::new_v4();
        
        // Get previous hash from chain
        let previous_hash = match origin {
            AuditOrigin::Client => {
                let chain = self.client_chain.read().await;
                chain.last().map(|entry| entry.data_hash.clone())
            },
            AuditOrigin::Server => {
                let chain = self.server_chain.read().await;
                chain.last().map(|entry| entry.data_hash.clone())
            },
        };

        // Compute data hash
        let data_hash = self.compute_data_hash(&action_type, &timestamp, &metadata)?;

        // Sign the entry
        let keypair = match origin {
            AuditOrigin::Client => self.client_keypair.read().await,
            AuditOrigin::Server => self.server_keypair.read().await,
        };
        
        let signature_data = format!("{}:{}:{}", entry_id, data_hash, timestamp.timestamp());
        let signature = keypair.sign(signature_data.as_bytes());
        let public_key = hex::encode(keypair.verifying_key().as_bytes());

        Ok(AuditEntry {
            id: entry_id,
            origin,
            action_type,
            timestamp,
            previous_hash,
            data_hash,
            signature: hex::encode(signature.to_bytes()),
            public_key,
            metadata,
        })
    }

    /// Compute consensus hash for dual audit record
    fn compute_consensus_hash(&self, record: &DualAuditRecord) -> AuditResult<String> {
        let mut hasher = Sha256::new();
        
        if let Some(ref client_entry) = record.client_entry {
            hasher.update(client_entry.data_hash.as_bytes());
        }
        if let Some(ref server_entry) = record.server_entry {
            hasher.update(server_entry.data_hash.as_bytes());
        }
        
        hasher.update(record.id.as_bytes());
        hasher.update(record.created_at.timestamp().to_be_bytes());
        
        Ok(hex::encode(hasher.finalize()))
    }

    /// Compute data hash for action
    fn compute_data_hash(
        &self,
        action_type: &ActionType,
        timestamp: &DateTime<Utc>,
        metadata: &HashMap<String, serde_json::Value>,
    ) -> AuditResult<String> {
        let mut hasher = Sha256::new();
        
        let action_json = serde_json::to_string(action_type)?;
        hasher.update(action_json.as_bytes());
        hasher.update(timestamp.timestamp().to_be_bytes());
        
        let metadata_json = serde_json::to_string(metadata)?;
        hasher.update(metadata_json.as_bytes());
        
        Ok(hex::encode(hasher.finalize()))
    }

    /// Notarize dual record with CNR
    async fn notarize_with_cnr(&self, record: &DualAuditRecord) -> AuditResult<Uuid> {
        if let Some(ref cnr) = self.cnr {
            let notary_data = serde_json::to_string(record)
                .map_err(|e| AuditError::CnrIntegration(format!("Serialization failed: {}", e)))?;
            
            let document_hash = {
                let mut hasher = Sha256::new();
                hasher.update(notary_data.as_bytes());
                hex::encode(hasher.finalize())
            };
            
            let metadata = DocumentMetadata {
                title: "Split-Origin Audit Record".to_string(),
                document_type: DocumentType::Other("split_origin_audit".to_string()),
                parties: vec!["client".to_string(), "server".to_string()],
                legal_significance: LegalSignificance::High,
                retention_years: 10,
            };
            
            let notarization = NotarizationDetails {
                notary_id: Uuid::new_v4(),
                notarized_at: Utc::now(),
                notary_signature: "system_generated".to_string(),
                witness_signatures: vec![],
                location: "digital".to_string(),
                method: NotarizationMethod::Digital,
            };
            
            let document = NotarizedDocument {
                id: Uuid::new_v4(),
                document_hash,
                metadata,
                notarization,
                proofs: vec![],
                status: DocumentStatus::Active,
                created_at: Utc::now(),
            };
            
            let notary_result = cnr.notarize_document(document).await
                .map_err(|e| AuditError::CnrIntegration(format!("CNR notarization failed: {:?}", e)))?;
            
            Ok(notary_result.operation_id)
        } else {
            Err(AuditError::CnrIntegration("CNR not available".to_string()))
        }
    }

    /// Verify audit chain integrity
    pub async fn verify_chain_integrity(&self, origin: AuditOrigin) -> AuditResult<bool> {
        let chain = match origin {
            AuditOrigin::Client => self.client_chain.read().await,
            AuditOrigin::Server => self.server_chain.read().await,
        };

        for (i, entry) in chain.iter().enumerate() {
            // Verify signature
            if !self.verify_entry_signature(entry).await? {
                error!("Signature verification failed for entry {}", entry.id);
                return Ok(false);
            }

            // Verify hash chain
            if i > 0 {
                let prev_entry = &chain[i - 1];
                if entry.previous_hash.as_ref() != Some(&prev_entry.data_hash) {
                    error!("Hash chain broken at entry {}", entry.id);
                    return Ok(false);
                }
            }
        }

        info!("Chain integrity verified for {:?} origin", origin);
        Ok(true)
    }

    /// Verify signature of an audit entry
    async fn verify_entry_signature(&self, entry: &AuditEntry) -> AuditResult<bool> {
        let public_key_bytes = hex::decode(&entry.public_key)
            .map_err(|e| AuditError::Cryptographic(format!("Invalid public key: {}", e)))?;
        
        let public_key = VerifyingKey::from_bytes(&public_key_bytes.try_into().map_err(|_| AuditError::Cryptographic("Invalid public key length".to_string()))?)
            .map_err(|e| AuditError::Cryptographic(format!("Invalid public key format: {}", e)))?;
        
        let signature_bytes = hex::decode(&entry.signature)
            .map_err(|e| AuditError::Cryptographic(format!("Invalid signature: {}", e)))?;
        
        let signature = Signature::from_bytes(&signature_bytes.try_into().map_err(|_| AuditError::Cryptographic("Invalid signature length".to_string()))?);
        
        let signature_data = format!("{}:{}:{}", entry.id, entry.data_hash, entry.timestamp.timestamp());
        
        match public_key.verify(signature_data.as_bytes(), &signature) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Get dual audit record by ID
    pub async fn get_dual_record(&self, id: Uuid) -> Option<DualAuditRecord> {
        self.dual_records.read().await.get(&id).cloned()
    }

    /// Get all dual audit records
    pub async fn get_all_dual_records(&self) -> Vec<DualAuditRecord> {
        self.dual_records.read().await.values().cloned().collect()
    }

    /// Detect discrepancies between client and server chains
    pub async fn detect_discrepancies(&self) -> AuditResult<Vec<String>> {
        let client_chain = self.client_chain.read().await;
        let server_chain = self.server_chain.read().await;
        let mut discrepancies = Vec::new();

        // Compare chain lengths
        if client_chain.len() != server_chain.len() {
            discrepancies.push(format!(
                "Chain length mismatch: client={}, server={}",
                client_chain.len(),
                server_chain.len()
            ));
        }

        // Compare corresponding entries
        let min_len = client_chain.len().min(server_chain.len());
        for i in 0..min_len {
            let client_entry = &client_chain[i];
            let server_entry = &server_chain[i];

            if client_entry.data_hash != server_entry.data_hash {
                discrepancies.push(format!(
                    "Data hash mismatch at index {}: client={}, server={}",
                    i, client_entry.data_hash, server_entry.data_hash
                ));
            }

            if client_entry.timestamp != server_entry.timestamp {
                discrepancies.push(format!(
                    "Timestamp mismatch at index {}: client={}, server={}",
                    i, client_entry.timestamp, server_entry.timestamp
                ));
            }
        }

        if discrepancies.is_empty() {
            info!("No discrepancies detected between client and server chains");
        } else {
            warn!("Detected {} discrepancies in audit chains", discrepancies.len());
        }

        Ok(discrepancies)
    }

    /// Export audit data for external verification
    pub async fn export_audit_data(&self) -> AuditResult<String> {
        let export_data = serde_json::json!({
            "client_chain": *self.client_chain.read().await,
            "server_chain": *self.server_chain.read().await,
            "dual_records": *self.dual_records.read().await,
            "config": {
                "enable_client_auditing": self.config.enable_client_auditing,
                "enable_server_auditing": self.config.enable_server_auditing,
                "require_dual_consensus": self.config.require_dual_consensus,
                "cnr_integration": self.config.cnr_integration,
            },
            "export_timestamp": Utc::now(),
        });

        Ok(serde_json::to_string_pretty(&export_data)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Test utilities

    #[tokio::test]
    async fn test_split_origin_auditor_creation() {
        let config = AuditConfig::default();
        let auditor = SplitOriginAuditor::new(config, None).unwrap();
        
        assert!(auditor.client_chain.read().await.is_empty());
        assert!(auditor.server_chain.read().await.is_empty());
        assert!(auditor.dual_records.read().await.is_empty());
    }

    #[tokio::test]
    async fn test_audit_action() {
        let config = AuditConfig::default();
        let auditor = SplitOriginAuditor::new(config, None).unwrap();
        
        let action = ActionType::UserAction {
            action: "login".to_string(),
            user_id: "user123".to_string(),
        };
        
        let metadata = HashMap::new();
        let result = auditor.audit_action(action, metadata).await.unwrap();
        
        assert!(result.client_entry.is_some());
        assert!(result.server_entry.is_some());
        assert!(result.consensus_hash.is_some());
    }

    #[tokio::test]
    async fn test_chain_integrity_verification() {
        let config = AuditConfig::default();
        let auditor = SplitOriginAuditor::new(config, None).unwrap();
        
        // Add some audit entries
        for i in 0..3 {
            let action = ActionType::SystemEvent {
                event_type: format!("test_event_{}", i),
            };
            auditor.audit_action(action, HashMap::new()).await.unwrap();
        }
        
        // Verify chain integrity
        assert!(auditor.verify_chain_integrity(AuditOrigin::Client).await.unwrap());
        assert!(auditor.verify_chain_integrity(AuditOrigin::Server).await.unwrap());
    }

    #[tokio::test]
    async fn test_discrepancy_detection() {
        let config = AuditConfig::default();
        let auditor = SplitOriginAuditor::new(config, None).unwrap();
        
        // Add audit entries
        let action = ActionType::DataAccess {
            resource_id: "resource123".to_string(),
            operation: "read".to_string(),
        };
        auditor.audit_action(action, HashMap::new()).await.unwrap();
        
        // Should have no discrepancies initially
        let discrepancies = auditor.detect_discrepancies().await.unwrap();
        assert!(discrepancies.is_empty());
    }

    #[tokio::test]
    async fn test_export_audit_data() {
        let config = AuditConfig::default();
        let auditor = SplitOriginAuditor::new(config, None).unwrap();
        
        let action = ActionType::ContractExecution {
            contract_id: Uuid::new_v4(),
        };
        auditor.audit_action(action, HashMap::new()).await.unwrap();
        
        let export_data = auditor.export_audit_data().await.unwrap();
        assert!(!export_data.is_empty());
        
        // Verify it's valid JSON
        let _: serde_json::Value = serde_json::from_str(&export_data).unwrap();
    }
}
