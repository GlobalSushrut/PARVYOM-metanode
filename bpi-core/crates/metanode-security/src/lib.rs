// Metanode Security Supercrate
// Consolidated security features and cryptographic systems for BPI Core

//! # Metanode Security
//! 
//! This supercrate consolidates all security-related functionality including:
//! - BPI encryption and decryption
//! - Split-origin auditing for tamper-proof logs
//! - Court node security and dispute resolution
//! - Court notary registry for decentralized notarization
//! - BPI shadow registry for cross-chain security

use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Error, Debug)]
pub enum SecurityError {
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Audit verification failed: {0}")]
    AuditVerificationFailed(String),
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
    #[error("Access denied: {0}")]
    AccessDenied(String),
    #[error("Security policy violation: {0}")]
    PolicyViolation(String),
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub encryption_enabled: bool,
    pub audit_enabled: bool,
    pub signature_verification: bool,
    pub access_control_enabled: bool,
    pub policy_enforcement: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        SecurityConfig {
            encryption_enabled: true,
            audit_enabled: true,
            signature_verification: true,
            access_control_enabled: true,
            policy_enforcement: true,
        }
    }
}

/// Security context for operations
#[derive(Debug, Clone)]
pub struct SecurityContext {
    pub user_id: String,
    pub permissions: Vec<String>,
    pub session_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

/// Main security manager
pub struct SecurityManager {
    config: SecurityConfig,
    audit_log: Vec<AuditEntry>,
    access_policies: HashMap<String, AccessPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub user_id: String,
    pub action: String,
    pub resource: String,
    pub result: AuditResult,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditResult {
    Success,
    Failure(String),
    Denied(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicy {
    pub resource: String,
    pub required_permissions: Vec<String>,
    pub allowed_users: Vec<String>,
    pub denied_users: Vec<String>,
}

impl SecurityManager {
    pub fn new(config: SecurityConfig) -> Self {
        SecurityManager {
            config,
            audit_log: Vec::new(),
            access_policies: HashMap::new(),
        }
    }

    pub fn encrypt_data(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, SecurityError> {
        if !self.config.encryption_enabled {
            return Ok(data.to_vec());
        }

        // Simple XOR encryption for demo - in practice would use proper encryption
        let encrypted: Vec<u8> = data.iter()
            .zip(key.iter().cycle())
            .map(|(d, k)| d ^ k)
            .collect();
        
        Ok(encrypted)
    }

    pub fn decrypt_data(&self, encrypted_data: &[u8], key: &[u8]) -> Result<Vec<u8>, SecurityError> {
        if !self.config.encryption_enabled {
            return Ok(encrypted_data.to_vec());
        }

        // Simple XOR decryption for demo
        let decrypted: Vec<u8> = encrypted_data.iter()
            .zip(key.iter().cycle())
            .map(|(d, k)| d ^ k)
            .collect();
        
        Ok(decrypted)
    }

    pub fn verify_access(&self, context: &SecurityContext, resource: &str) -> Result<(), SecurityError> {
        if !self.config.access_control_enabled {
            return Ok(());
        }

        if let Some(policy) = self.access_policies.get(resource) {
            // Check if user is explicitly denied
            if policy.denied_users.contains(&context.user_id) {
                return Err(SecurityError::AccessDenied(
                    format!("User {} is denied access to {}", context.user_id, resource)
                ));
            }

            // Check if user has required permissions
            for required_perm in &policy.required_permissions {
                if !context.permissions.contains(required_perm) {
                    return Err(SecurityError::AccessDenied(
                        format!("Missing permission: {}", required_perm)
                    ));
                }
            }
        }

        Ok(())
    }

    pub fn log_audit_entry(&mut self, entry: AuditEntry) -> Result<(), SecurityError> {
        if !self.config.audit_enabled {
            return Ok(());
        }

        // In practice, would verify signature and store in tamper-proof log
        self.audit_log.push(entry);
        Ok(())
    }

    pub fn create_audit_entry(
        &self,
        context: &SecurityContext,
        action: String,
        resource: String,
        result: AuditResult,
    ) -> AuditEntry {
        AuditEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            user_id: context.user_id.clone(),
            action,
            resource,
            result,
            signature: vec![0; 64], // Placeholder signature
        }
    }

    pub fn add_access_policy(&mut self, policy: AccessPolicy) {
        self.access_policies.insert(policy.resource.clone(), policy);
    }

    pub fn get_audit_log(&self) -> &[AuditEntry] {
        &self.audit_log
    }
}

// Re-export consolidated modules (these would be implemented from moved crates)
pub mod bpi_enc {
    //! BPI encryption and decryption
    pub use super::*;
    
    pub fn encrypt_bpi_data(data: &[u8]) -> Result<Vec<u8>, SecurityError> {
        // BPI-specific encryption logic would be here
        Ok(data.to_vec())
    }
    
    pub fn decrypt_bpi_data(encrypted_data: &[u8]) -> Result<Vec<u8>, SecurityError> {
        // BPI-specific decryption logic would be here
        Ok(encrypted_data.to_vec())
    }
}

pub mod split_origin_auditing {
    //! Split-origin auditing for tamper-proof logs
    pub use super::*;
    
    pub fn create_split_audit(client_log: &str, server_log: &str) -> Result<String, SecurityError> {
        // Split-origin audit creation logic would be here
        Ok(format!("{}:{}", client_log, server_log))
    }
    
    pub fn verify_split_audit(audit: &str) -> Result<bool, SecurityError> {
        // Split-origin audit verification logic would be here
        Ok(true)
    }
}

pub mod court_node {
    //! Court node security and dispute resolution
    pub use super::*;
    
    pub fn initialize_court_node() -> Result<(), SecurityError> {
        // Court node initialization logic would be here
        Ok(())
    }
    
    pub fn process_dispute(dispute_id: &str) -> Result<String, SecurityError> {
        // Dispute processing logic would be here
        Ok(format!("Dispute {} processed", dispute_id))
    }
}

pub mod court_notary_registry {
    //! Court notary registry for decentralized notarization
    pub use super::*;
    
    pub fn register_notary(notary_id: &str) -> Result<(), SecurityError> {
        // Notary registration logic would be here
        Ok(())
    }
    
    pub fn verify_notarization(document_hash: &str) -> Result<bool, SecurityError> {
        // Notarization verification logic would be here
        Ok(true)
    }
}

pub mod bpi_shadow_registry {
    //! BPI shadow registry for cross-chain security
    pub use super::*;
    
    pub fn register_shadow_entry(entry: &str) -> Result<(), SecurityError> {
        // Shadow registry entry logic would be here
        Ok(())
    }
    
    pub fn verify_shadow_entry(entry_id: &str) -> Result<bool, SecurityError> {
        // Shadow entry verification logic would be here
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_manager_creation() {
        let config = SecurityConfig::default();
        let manager = SecurityManager::new(config);
        assert!(manager.config.encryption_enabled);
        assert!(manager.config.audit_enabled);
    }

    #[test]
    fn test_encryption_decryption() {
        let manager = SecurityManager::new(SecurityConfig::default());
        let data = b"test data";
        let key = b"secret key";
        
        let encrypted = manager.encrypt_data(data, key).unwrap();
        let decrypted = manager.decrypt_data(&encrypted, key).unwrap();
        
        assert_eq!(data, decrypted.as_slice());
    }

    #[test]
    fn test_access_control() {
        let mut manager = SecurityManager::new(SecurityConfig::default());
        
        let policy = AccessPolicy {
            resource: "test_resource".to_string(),
            required_permissions: vec!["read".to_string()],
            allowed_users: vec!["user1".to_string()],
            denied_users: vec![],
        };
        
        manager.add_access_policy(policy);
        
        let context = SecurityContext {
            user_id: "user1".to_string(),
            permissions: vec!["read".to_string()],
            session_id: Uuid::new_v4(),
            timestamp: Utc::now(),
        };
        
        assert!(manager.verify_access(&context, "test_resource").is_ok());
    }

    #[test]
    fn test_audit_logging() {
        let mut manager = SecurityManager::new(SecurityConfig::default());
        
        let context = SecurityContext {
            user_id: "user1".to_string(),
            permissions: vec!["read".to_string()],
            session_id: Uuid::new_v4(),
            timestamp: Utc::now(),
        };
        
        let entry = manager.create_audit_entry(
            &context,
            "read".to_string(),
            "test_resource".to_string(),
            AuditResult::Success,
        );
        
        manager.log_audit_entry(entry).unwrap();
        assert_eq!(manager.get_audit_log().len(), 1);
    }
}
