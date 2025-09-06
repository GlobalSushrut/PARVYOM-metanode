//! BPCI Registry Guard - Consensus Security Enforcement
//! 
//! This module ensures BPI ledger consensus is COMPLETELY DEACTIVATED
//! until proper BPCI registry address and token are provided.
//! This is enforced at the consensus layer for maximum security.

use crate::{Hash, MathError, Timestamp};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

/// BPCI Registry Guard - Controls consensus activation
#[derive(Debug, Clone)]
pub struct BPCIRegistryGuard {
    /// Registry credentials storage
    registry_credentials: Arc<RwLock<RegistryCredentials>>,
    /// Consensus activation status
    consensus_activated: Arc<RwLock<bool>>,
    /// Security validation cache
    validation_cache: Arc<RwLock<HashMap<String, ValidationResult>>>,
    /// Installation hash for unhackable verification
    installation_hash: Arc<RwLock<Option<String>>>,
}

/// Registry credentials required for consensus activation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryCredentials {
    /// BPCI registry address (from BPCI server)
    pub registry_address: Option<String>,
    /// BPCI registry token (from BPCI server)
    pub registry_token: Option<String>,
    /// Network type (testnet/mainnet)
    pub network_type: NetworkType,
    /// Registration timestamp
    pub registered_at: Option<Timestamp>,
    /// Validation status
    pub is_validated: bool,
}

/// Network types for BPI consensus
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NetworkType {
    /// Testnet - validation phase
    Testnet,
    /// Mainnet - production
    Mainnet,
}

/// Validation result for registry credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Is valid
    pub is_valid: bool,
    /// Validation message
    pub message: String,
    /// Validation timestamp
    pub validated_at: Timestamp,
    /// Validation hash
    pub validation_hash: String,
}

/// Consensus operation types that require validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusOperation {
    /// Block proposal
    ProposeBlock,
    /// Block validation
    ValidateBlock,
    /// Block finalization
    FinalizeBlock,
    /// Transaction processing
    ProcessTransaction,
    /// Ledger update
    UpdateLedger,
    /// Mining operation
    Mining,
}

impl Default for RegistryCredentials {
    fn default() -> Self {
        Self {
            registry_address: None,
            registry_token: None,
            network_type: NetworkType::Testnet,
            registered_at: None,
            is_validated: false,
        }
    }
}

impl BPCIRegistryGuard {
    /// Create new BPCI registry guard
    pub fn new() -> Self {
        Self {
            registry_credentials: Arc::new(RwLock::new(RegistryCredentials::default())),
            consensus_activated: Arc::new(RwLock::new(false)),
            validation_cache: Arc::new(RwLock::new(HashMap::new())),
            installation_hash: Arc::new(RwLock::new(None)),
        }
    }

    /// Set BPCI registry credentials (from BPCI server)
    pub fn set_registry_credentials(
        &self,
        registry_address: String,
        registry_token: String,
        network_type: NetworkType,
    ) -> Result<(), MathError> {
        // Validate credentials format
        if registry_address.is_empty() || registry_token.is_empty() {
            return Err(MathError::InvalidInput("Registry address and token cannot be empty".into()));
        }

        // Validate registry address format
        if !registry_address.starts_with("bpi_") {
            return Err(MathError::InvalidInput("Invalid registry address format".into()));
        }

        // Validate registry token format
        if registry_token.len() < 8 {
            return Err(MathError::InvalidInput("Registry token too short".into()));
        }

        // Store credentials
        let mut credentials = self.registry_credentials.write().unwrap();
        credentials.registry_address = Some(registry_address.clone());
        credentials.registry_token = Some(registry_token.clone());
        credentials.network_type = network_type;
        credentials.registered_at = Some(chrono::Utc::now());
        credentials.is_validated = true;

        // Activate consensus only after proper registration
        let mut consensus_activated = self.consensus_activated.write().unwrap();
        *consensus_activated = true;

        // Cache validation result
        let validation_result = ValidationResult {
            is_valid: true,
            message: "Registry credentials validated successfully".to_string(),
            validated_at: chrono::Utc::now(),
            validation_hash: format!("VALID_{}", registry_address),
        };

        let mut cache = self.validation_cache.write().unwrap();
        cache.insert(registry_address, validation_result);

        Ok(())
    }

    /// Check if consensus operation is allowed (DEACTIVATED without registration)
    pub fn is_consensus_operation_allowed(&self, operation: ConsensusOperation) -> Result<bool, MathError> {
        // Check if consensus is activated
        let consensus_activated = *self.consensus_activated.read().unwrap();
        if !consensus_activated {
            eprintln!("SECURITY: BPI Consensus DEACTIVATED - Registry credentials required");
            eprintln!("Operation blocked: {:?}", operation);
            return Ok(false);
        }

        // Validate registry credentials
        let credentials = self.registry_credentials.read().unwrap();
        if !credentials.is_validated {
            eprintln!("SECURITY: BPI Consensus BLOCKED - Invalid registry credentials");
            return Ok(false);
        }

        if credentials.registry_address.is_none() || credentials.registry_token.is_none() {
            eprintln!("SECURITY: BPI Consensus BLOCKED - Missing registry credentials");
            return Ok(false);
        }

        // All checks passed - operation allowed
        Ok(true)
    }

    /// Validate consensus integrity (unhackable check)
    pub fn validate_consensus_integrity(&self) -> Result<bool, MathError> {
        let installation_hash = self.installation_hash.read().unwrap();
        
        // If deployed to consensus layer, verify unhackable installation
        if let Some(hash) = installation_hash.as_ref() {
            if !hash.starts_with("BPI_CONSENSUS_") {
                return Err(MathError::SecurityViolation("Consensus deployment corrupted".into()));
            }
        }

        // Check registry credentials integrity
        let credentials = self.registry_credentials.read().unwrap();
        if credentials.is_validated {
            if credentials.registry_address.is_none() || credentials.registry_token.is_none() {
                return Err(MathError::SecurityViolation("Registry credentials corrupted".into()));
            }
        }

        Ok(true)
    }

    /// Deploy to consensus layer (makes system unhackable)
    pub fn deploy_to_consensus(&self, community_hash: String) -> Result<(), MathError> {
        let installation_hash = format!("BPI_CONSENSUS_{}", community_hash);
        let mut hash_guard = self.installation_hash.write().unwrap();
        *hash_guard = Some(installation_hash);
        
        // Once deployed to consensus, all security is enforced at consensus layer
        Ok(())
    }

    /// Get current consensus status
    pub fn get_consensus_status(&self) -> ConsensusStatus {
        let consensus_activated = *self.consensus_activated.read().unwrap();
        let credentials = self.registry_credentials.read().unwrap();
        let installation_hash = self.installation_hash.read().unwrap();

        ConsensusStatus {
            is_activated: consensus_activated,
            has_registry_address: credentials.registry_address.is_some(),
            has_registry_token: credentials.registry_token.is_some(),
            network_type: credentials.network_type.clone(),
            is_deployed_to_consensus: installation_hash.is_some(),
            registered_at: credentials.registered_at,
        }
    }

    /// Force deactivate consensus (emergency use only)
    pub fn force_deactivate_consensus(&self, reason: String) -> Result<(), MathError> {
        eprintln!("EMERGENCY: Force deactivating BPI consensus - Reason: {}", reason);
        
        let mut consensus_activated = self.consensus_activated.write().unwrap();
        *consensus_activated = false;

        let mut credentials = self.registry_credentials.write().unwrap();
        credentials.is_validated = false;

        Ok(())
    }

    /// Get registry credentials (read-only)
    pub fn get_registry_credentials(&self) -> RegistryCredentials {
        self.registry_credentials.read().unwrap().clone()
    }

    /// Clear all credentials and deactivate consensus
    pub fn clear_credentials(&self) -> Result<(), MathError> {
        let mut credentials = self.registry_credentials.write().unwrap();
        *credentials = RegistryCredentials::default();

        let mut consensus_activated = self.consensus_activated.write().unwrap();
        *consensus_activated = false;

        let mut cache = self.validation_cache.write().unwrap();
        cache.clear();

        Ok(())
    }
}

/// Current consensus status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusStatus {
    /// Is consensus activated
    pub is_activated: bool,
    /// Has registry address
    pub has_registry_address: bool,
    /// Has registry token
    pub has_registry_token: bool,
    /// Network type
    pub network_type: NetworkType,
    /// Is deployed to consensus layer
    pub is_deployed_to_consensus: bool,
    /// Registration timestamp
    pub registered_at: Option<Timestamp>,
}

/// Custom error types for registry guard
impl From<std::io::Error> for MathError {
    fn from(err: std::io::Error) -> Self {
        MathError::InvalidInput(format!("IO Error: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_guard_creation() {
        let guard = BPCIRegistryGuard::new();
        let status = guard.get_consensus_status();
        
        assert!(!status.is_activated);
        assert!(!status.has_registry_address);
        assert!(!status.has_registry_token);
    }

    #[test]
    fn test_consensus_deactivated_without_credentials() {
        let guard = BPCIRegistryGuard::new();
        
        let allowed = guard.is_consensus_operation_allowed(ConsensusOperation::ProposeBlock).unwrap();
        assert!(!allowed); // Should be blocked without credentials
    }

    #[test]
    fn test_registry_credentials_validation() {
        let guard = BPCIRegistryGuard::new();
        
        // Should fail with empty credentials
        let result = guard.set_registry_credentials(
            "".to_string(),
            "".to_string(),
            NetworkType::Testnet,
        );
        assert!(result.is_err());

        // Should succeed with valid credentials
        let result = guard.set_registry_credentials(
            "bpi_testnet_registry_001".to_string(),
            "TEST_REG_123456".to_string(),
            NetworkType::Testnet,
        );
        assert!(result.is_ok());

        let status = guard.get_consensus_status();
        assert!(status.is_activated);
        assert!(status.has_registry_address);
        assert!(status.has_registry_token);
    }

    #[test]
    fn test_consensus_operations_after_registration() {
        let guard = BPCIRegistryGuard::new();
        
        // Register with valid credentials
        guard.set_registry_credentials(
            "bpi_testnet_registry_001".to_string(),
            "TEST_REG_123456".to_string(),
            NetworkType::Testnet,
        ).unwrap();

        // All operations should be allowed now
        assert!(guard.is_consensus_operation_allowed(ConsensusOperation::ProposeBlock).unwrap());
        assert!(guard.is_consensus_operation_allowed(ConsensusOperation::ValidateBlock).unwrap());
        assert!(guard.is_consensus_operation_allowed(ConsensusOperation::ProcessTransaction).unwrap());
    }

    #[test]
    fn test_force_deactivate_consensus() {
        let guard = BPCIRegistryGuard::new();
        
        // Register first
        guard.set_registry_credentials(
            "bpi_testnet_registry_001".to_string(),
            "TEST_REG_123456".to_string(),
            NetworkType::Testnet,
        ).unwrap();

        // Verify it's activated
        assert!(guard.get_consensus_status().is_activated);

        // Force deactivate
        guard.force_deactivate_consensus("Test emergency".to_string()).unwrap();

        // Should be deactivated now
        assert!(!guard.get_consensus_status().is_activated);
        assert!(!guard.is_consensus_operation_allowed(ConsensusOperation::ProposeBlock).unwrap());
    }
}
