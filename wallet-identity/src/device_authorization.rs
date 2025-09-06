use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use ed25519_dalek::{Keypair, Signature, Signer, Verifier};
use uuid::Uuid;

use crate::wallet_identity::{WalletIdentity, WalletError};

/// Device Authorization System
/// Manages graduated access levels for computer and system resources

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAuthorizationManager {
    /// Active authorization sessions
    active_sessions: HashMap<String, AuthorizationSession>,
    /// Access level policies
    access_policies: HashMap<AccessLevel, AccessPolicy>,
    /// Device capability registry
    device_registry: HashMap<String, DeviceInfo>,
    /// Audit trail for all access requests
    audit_trail: Vec<AccessAuditEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationSession {
    pub session_id: String,
    pub wallet_address: String,
    pub access_level: AccessLevel,
    pub granted_capabilities: Vec<SystemCapability>,
    pub pes_tokens: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub device_fingerprint: String,
    pub ip_address: String,
    pub user_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AccessLevel {
    Basic,
    Elevated,
    Administrative,
    Root,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicy {
    pub level: AccessLevel,
    pub capabilities: Vec<SystemCapability>,
    pub duration_limit: chrono::Duration,
    pub requires_fresh_auth: bool,
    pub requires_webauthn_uv: bool,
    pub requires_government_stamp: bool,
    pub requires_hsm_signature: bool,
    pub audit_level: AuditLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SystemCapability {
    // File System Access
    FileRead(FileScope),
    FileWrite(FileScope),
    FileDelete(FileScope),
    FileExecute(FileScope),
    
    // Network Access
    NetworkOutboundHTTPS,
    NetworkOutboundAny,
    NetworkInboundRestricted,
    NetworkInboundAny,
    
    // System Information
    SystemInfoRead,
    SystemInfoWrite,
    HardwareInfoRead,
    ProcessListRead,
    
    // Process Management
    ProcessStart,
    ProcessKill,
    ProcessManageUser,
    ProcessManageSystem,
    
    // Device Access
    CameraAccess,
    MicrophoneAccess,
    ScreenCapture,
    USBDeviceAccess,
    BluetoothAccess,
    
    // System Configuration
    SystemConfigRead,
    SystemConfigWrite,
    RegistryRead,
    RegistryWrite,
    ServiceManagement,
    
    // Administrative
    UserAccountManagement,
    SystemAdministration,
    SecurityPolicyManagement,
    
    // Root Level
    KernelAccess,
    DriverInstallation,
    SystemRecovery,
    FullSystemControl,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FileScope {
    UserDocumentsOnly,
    UserHomeDirectory,
    UserDocumentsAndTemp,
    SystemTemp,
    ProgramFiles,
    SystemFiles,
    Unrestricted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditLevel {
    None,
    Basic,
    Detailed,
    Comprehensive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_id: String,
    pub device_type: DeviceType,
    pub os_info: OSInfo,
    pub hardware_info: HardwareInfo,
    pub security_features: SecurityFeatures,
    pub trust_level: TrustLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    Desktop,
    Laptop,
    Mobile,
    Tablet,
    Server,
    IoT,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSInfo {
    pub name: String,
    pub version: String,
    pub architecture: String,
    pub security_patches: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub cpu: String,
    pub memory_gb: u32,
    pub storage_gb: u32,
    pub has_tpm: bool,
    pub has_secure_enclave: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityFeatures {
    pub has_biometric: bool,
    pub has_hardware_key: bool,
    pub has_secure_boot: bool,
    pub encryption_enabled: bool,
    pub has_tpm: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum TrustLevel {
    Untrusted,
    Basic,
    Verified,
    Trusted,
    HighlyTrusted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessAuditEntry {
    pub entry_id: String,
    pub timestamp: DateTime<Utc>,
    pub wallet_address: String,
    pub session_id: String,
    pub action: AuditAction,
    pub capability: SystemCapability,
    pub result: AccessResult,
    pub details: serde_json::Value,
    pub bpi_anchor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditAction {
    AccessRequested,
    AccessGranted,
    AccessDenied,
    AccessRevoked,
    CapabilityUsed,
    PolicyViolation,
    SuspiciousActivity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessResult {
    Success,
    Denied,
    Error,
    Timeout,
}

impl DeviceAuthorizationManager {
    pub fn new() -> Self {
        let mut manager = Self {
            active_sessions: HashMap::new(),
            access_policies: HashMap::new(),
            device_registry: HashMap::new(),
            audit_trail: Vec::new(),
        };
        
        manager.initialize_access_policies();
        manager
    }
    
    /// Request device authorization for a wallet
    pub fn request_authorization(
        &mut self,
        wallet: &WalletIdentity,
        requested_level: AccessLevel,
        requested_capabilities: Vec<SystemCapability>,
        device_info: DeviceInfo,
    ) -> Result<String, DeviceAuthError> {
        // Check if wallet is authorized for this access level
        self.verify_wallet_authorization(wallet, &requested_level)?;
        
        // Verify device trust level
        self.verify_device_trust(&device_info, &requested_level)?;
        
        // Check policy requirements
        let policy = self.access_policies.get(&requested_level)
            .ok_or(DeviceAuthError::InvalidAccessLevel)?;
        
        // Verify all requested capabilities are allowed
        for capability in &requested_capabilities {
            if !policy.capabilities.contains(capability) {
                return Err(DeviceAuthError::CapabilityNotAllowed);
            }
        }
        
        // Create authorization session
        let session_id = format!("auth_{}", Uuid::new_v4().to_string().replace("-", ""));
        let device_fingerprint = self.generate_device_fingerprint(&device_info);
        
        let session = AuthorizationSession {
            session_id: session_id.clone(),
            wallet_address: wallet.wallet_address.clone(),
            access_level: requested_level.clone(),
            granted_capabilities: requested_capabilities.clone(),
            pes_tokens: Vec::new(),
            created_at: Utc::now(),
            expires_at: Utc::now() + policy.duration_limit,
            last_activity: Utc::now(),
            device_fingerprint: device_fingerprint.clone(),
            ip_address: "127.0.0.1".to_string(), // TODO: Get real IP
            user_agent: "XTMP-Client/1.0".to_string(), // TODO: Get real user agent
        };
        
        // Store session
        self.active_sessions.insert(session_id.clone(), session);
        
        // Register device if new
        if !self.device_registry.contains_key(&device_fingerprint) {
            self.device_registry.insert(device_fingerprint, device_info);
        }
        
        // Create audit entry
        self.create_audit_entry(
            &wallet.wallet_address,
            &session_id,
            AuditAction::AccessGranted,
            SystemCapability::SystemInfoRead, // Generic capability for session creation
            AccessResult::Success,
            serde_json::json!({
                "access_level": requested_level,
                "capabilities_count": requested_capabilities.len()
            }),
        );
        
        Ok(session_id)
    }
    
    /// Use a capability within an authorized session
    pub fn use_capability(
        &mut self,
        session_id: &str,
        capability: SystemCapability,
        operation_details: serde_json::Value,
    ) -> Result<(), DeviceAuthError> {
        let session = self.active_sessions.get_mut(session_id)
            .ok_or(DeviceAuthError::SessionNotFound)?;
        
        // Check session expiry
        if Utc::now() > session.expires_at {
            return Err(DeviceAuthError::SessionExpired);
        }
        
        // Check if capability is granted
        if !session.granted_capabilities.contains(&capability) {
            let wallet_address = session.wallet_address.clone();
            self.create_audit_entry(
                &wallet_address,
                session_id,
                AuditAction::AccessDenied,
                capability,
                AccessResult::Denied,
                operation_details,
            );
            return Err(DeviceAuthError::CapabilityNotGranted);
        }
        
        // Update last activity
        session.last_activity = Utc::now();
        
        // Create audit entry for capability use
        let wallet_address = session.wallet_address.clone();
        self.create_audit_entry(
            &wallet_address,
            session_id,
            AuditAction::CapabilityUsed,
            capability,
            AccessResult::Success,
            operation_details,
        );
        
        Ok(())
    }
    
    /// Revoke authorization session
    pub fn revoke_session(&mut self, session_id: &str) -> Result<(), DeviceAuthError> {
        let session = self.active_sessions.remove(session_id)
            .ok_or(DeviceAuthError::SessionNotFound)?;
        
        // Create audit entry
        self.create_audit_entry(
            &session.wallet_address,
            session_id,
            AuditAction::AccessRevoked,
            SystemCapability::SystemInfoRead, // Generic capability
            AccessResult::Success,
            serde_json::json!({
                "reason": "manual_revocation"
            }),
        );
        
        Ok(())
    }
    
    /// Get active session information
    pub fn get_session(&self, session_id: &str) -> Option<&AuthorizationSession> {
        self.active_sessions.get(session_id)
    }
    
    /// Get all sessions for a wallet
    pub fn get_wallet_sessions(&self, wallet_address: &str) -> Vec<&AuthorizationSession> {
        self.active_sessions.values()
            .filter(|session| session.wallet_address == wallet_address)
            .collect()
    }
    
    /// Clean up expired sessions
    pub fn cleanup_expired_sessions(&mut self) {
        let now = Utc::now();
        let expired_sessions: Vec<String> = self.active_sessions.iter()
            .filter(|(_, session)| now > session.expires_at)
            .map(|(id, _)| id.clone())
            .collect();
        
        for session_id in expired_sessions {
            if let Some(session) = self.active_sessions.remove(&session_id) {
                self.create_audit_entry(
                    &session.wallet_address,
                    &session_id,
                    AuditAction::AccessRevoked,
                    SystemCapability::SystemInfoRead,
                    AccessResult::Success,
                    serde_json::json!({
                        "reason": "session_expired"
                    }),
                );
            }
        }
    }
    
    /// Verify wallet authorization for access level
    fn verify_wallet_authorization(
        &self,
        wallet: &WalletIdentity,
        access_level: &AccessLevel,
    ) -> Result<(), DeviceAuthError> {
        match access_level {
            AccessLevel::Basic => Ok(()),
            AccessLevel::Elevated => {
                // Require trusted provider
                if wallet.is_trusted_provider() {
                    Ok(())
                } else {
                    Err(DeviceAuthError::InsufficientWalletTrust)
                }
            },
            AccessLevel::Administrative => {
                // Require government or bank wallet
                match &wallet.provider {
                    crate::wallet_identity::WalletProvider::Government(_) |
                    crate::wallet_identity::WalletProvider::Bank(_) => Ok(()),
                    _ => Err(DeviceAuthError::InsufficientWalletTrust),
                }
            },
            AccessLevel::Root => {
                // Require government wallet with specific authority
                match &wallet.provider {
                    crate::wallet_identity::WalletProvider::Government(_) => Ok(()),
                    _ => Err(DeviceAuthError::InsufficientWalletTrust),
                }
            },
        }
    }
    
    /// Verify device trust level for access level
    fn verify_device_trust(
        &self,
        device_info: &DeviceInfo,
        access_level: &AccessLevel,
    ) -> Result<(), DeviceAuthError> {
        let required_trust = match access_level {
            AccessLevel::Basic => TrustLevel::Basic,
            AccessLevel::Elevated => TrustLevel::Verified,
            AccessLevel::Administrative => TrustLevel::Trusted,
            AccessLevel::Root => TrustLevel::HighlyTrusted,
        };
        
        if (device_info.trust_level as u8) >= (required_trust as u8) {
            Ok(())
        } else {
            Err(DeviceAuthError::InsufficientDeviceTrust)
        }
    }
    
    /// Generate device fingerprint
    fn generate_device_fingerprint(&self, device_info: &DeviceInfo) -> String {
        use sha2::{Sha256, Digest};
        
        let fingerprint_data = format!(
            "{}:{}:{}:{}:{}",
            device_info.os_info.name,
            device_info.os_info.version,
            device_info.hardware_info.cpu,
            device_info.hardware_info.memory_gb,
            device_info.security_features.has_tpm
        );
        
        let mut hasher = Sha256::new();
        hasher.update(fingerprint_data.as_bytes());
        hex::encode(hasher.finalize())
    }
    
    /// Create audit entry
    fn create_audit_entry(
        &mut self,
        wallet_address: &str,
        session_id: &str,
        action: AuditAction,
        capability: SystemCapability,
        result: AccessResult,
        details: serde_json::Value,
    ) {
        let entry = AccessAuditEntry {
            entry_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            wallet_address: wallet_address.to_string(),
            session_id: session_id.to_string(),
            action,
            capability,
            result,
            details,
            bpi_anchor: None, // TODO: Anchor to BPI ledger
        };
        
        self.audit_trail.push(entry);
    }
    
    /// Initialize access level policies
    fn initialize_access_policies(&mut self) {
        // Basic access level
        self.access_policies.insert(AccessLevel::Basic, AccessPolicy {
            level: AccessLevel::Basic,
            capabilities: vec![
                SystemCapability::FileRead(FileScope::UserDocumentsOnly),
                SystemCapability::NetworkOutboundHTTPS,
                SystemCapability::SystemInfoRead,
            ],
            duration_limit: chrono::Duration::hours(8),
            requires_fresh_auth: false,
            requires_webauthn_uv: false,
            requires_government_stamp: false,
            requires_hsm_signature: false,
            audit_level: AuditLevel::Basic,
        });
        
        // Elevated access level
        self.access_policies.insert(AccessLevel::Elevated, AccessPolicy {
            level: AccessLevel::Elevated,
            capabilities: vec![
                SystemCapability::FileRead(FileScope::UserHomeDirectory),
                SystemCapability::FileWrite(FileScope::UserDocumentsAndTemp),
                SystemCapability::NetworkOutboundAny,
                SystemCapability::SystemInfoRead,
                SystemCapability::HardwareInfoRead,
                SystemCapability::ProcessListRead,
                SystemCapability::CameraAccess,
                SystemCapability::MicrophoneAccess,
            ],
            duration_limit: chrono::Duration::hours(4),
            requires_fresh_auth: true,
            requires_webauthn_uv: false,
            requires_government_stamp: false,
            requires_hsm_signature: false,
            audit_level: AuditLevel::Detailed,
        });
        
        // Administrative access level
        self.access_policies.insert(AccessLevel::Administrative, AccessPolicy {
            level: AccessLevel::Administrative,
            capabilities: vec![
                SystemCapability::FileRead(FileScope::SystemFiles),
                SystemCapability::FileWrite(FileScope::SystemTemp),
                SystemCapability::SystemConfigRead,
                SystemCapability::SystemConfigWrite,
                SystemCapability::ProcessManageUser,
                SystemCapability::ServiceManagement,
                SystemCapability::RegistryRead,
                SystemCapability::RegistryWrite,
            ],
            duration_limit: chrono::Duration::hours(2),
            requires_fresh_auth: true,
            requires_webauthn_uv: true,
            requires_government_stamp: false,
            requires_hsm_signature: false,
            audit_level: AuditLevel::Comprehensive,
        });
        
        // Root access level
        self.access_policies.insert(AccessLevel::Root, AccessPolicy {
            level: AccessLevel::Root,
            capabilities: vec![
                SystemCapability::FullSystemControl,
                SystemCapability::KernelAccess,
                SystemCapability::DriverInstallation,
                SystemCapability::SystemRecovery,
                SystemCapability::UserAccountManagement,
                SystemCapability::SystemAdministration,
                SystemCapability::SecurityPolicyManagement,
            ],
            duration_limit: chrono::Duration::hours(1),
            requires_fresh_auth: true,
            requires_webauthn_uv: true,
            requires_government_stamp: true,
            requires_hsm_signature: true,
            audit_level: AuditLevel::Comprehensive,
        });
    }
    
    /// Get audit trail for wallet
    pub fn get_audit_trail(&self, wallet_address: &str) -> Vec<&AccessAuditEntry> {
        self.audit_trail.iter()
            .filter(|entry| entry.wallet_address == wallet_address)
            .collect()
    }
    
    /// Get system-wide audit trail (requires administrative access)
    pub fn get_system_audit_trail(&self) -> &Vec<AccessAuditEntry> {
        &self.audit_trail
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DeviceAuthError {
    #[error("Session not found")]
    SessionNotFound,
    #[error("Session expired")]
    SessionExpired,
    #[error("Invalid access level")]
    InvalidAccessLevel,
    #[error("Capability not allowed for this access level")]
    CapabilityNotAllowed,
    #[error("Capability not granted in this session")]
    CapabilityNotGranted,
    #[error("Insufficient wallet trust level")]
    InsufficientWalletTrust,
    #[error("Insufficient device trust level")]
    InsufficientDeviceTrust,
    #[error("Fresh authentication required")]
    FreshAuthRequired,
    #[error("WebAuthn UV step-up required")]
    WebAuthnUVRequired,
    #[error("Government stamp required")]
    GovernmentStampRequired,
    #[error("HSM signature required")]
    HSMSignatureRequired,
    #[error("Device not registered")]
    DeviceNotRegistered,
    #[error("Suspicious activity detected")]
    SuspiciousActivity,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallet_identity::{WalletIdentity, WalletProvider};
    
    #[test]
    fn test_basic_authorization() {
        let mut manager = DeviceAuthorizationManager::new();
        
        let wallet = WalletIdentity::new(
            "alice",
            WalletProvider::Pravyom,
            Some("alice@gmail.com".to_string()),
        ).unwrap();
        
        let device_info = DeviceInfo {
            device_id: "device_123".to_string(),
            device_type: DeviceType::Laptop,
            os_info: OSInfo {
                name: "Linux".to_string(),
                version: "5.15.0".to_string(),
                architecture: "x86_64".to_string(),
                security_patches: vec!["CVE-2023-1234".to_string()],
            },
            hardware_info: HardwareInfo {
                cpu: "Intel i7".to_string(),
                memory_gb: 16,
                storage_gb: 512,
                has_tpm: true,
                has_secure_enclave: false,
            },
            security_features: SecurityFeatures {
                has_biometric: true,
                has_hardware_key: false,
                has_secure_boot: true,
                has_tpm: true,
                encryption_enabled: true,
            },
            trust_level: TrustLevel::Verified,
        };
        
        let session_id = manager.request_authorization(
            &wallet,
            AccessLevel::Basic,
            vec![
                SystemCapability::FileRead(FileScope::UserDocumentsOnly),
                SystemCapability::NetworkOutboundHTTPS,
            ],
            device_info,
        ).unwrap();
        
        assert!(!session_id.is_empty());
        assert!(session_id.starts_with("auth_"));
        
        let session = manager.get_session(&session_id).unwrap();
        assert_eq!(session.wallet_address, "alice@pravyom.wallet");
        assert_eq!(session.access_level, AccessLevel::Basic);
        assert_eq!(session.granted_capabilities.len(), 2);
    }
    
    #[test]
    fn test_capability_usage() {
        let mut manager = DeviceAuthorizationManager::new();
        
        let wallet = WalletIdentity::new(
            "bob",
            WalletProvider::Bank("chase".to_string()),
            None,
        ).unwrap();
        
        let device_info = DeviceInfo {
            device_id: "device_456".to_string(),
            device_type: DeviceType::Desktop,
            os_info: OSInfo {
                name: "Windows".to_string(),
                version: "11".to_string(),
                architecture: "x86_64".to_string(),
                security_patches: Vec::new(),
            },
            hardware_info: HardwareInfo {
                cpu: "AMD Ryzen".to_string(),
                memory_gb: 32,
                storage_gb: 1024,
                has_tpm: true,
                has_secure_enclave: true,
            },
            security_features: SecurityFeatures {
                has_biometric: true,
                has_hardware_key: true,
                has_secure_boot: true,
                has_tpm: true,
                encryption_enabled: true,
            },
            trust_level: TrustLevel::Trusted,
        };
        
        let session_id = manager.request_authorization(
            &wallet,
            AccessLevel::Elevated,
            vec![SystemCapability::CameraAccess],
            device_info,
        ).unwrap();
        
        // Use the capability
        manager.use_capability(
            &session_id,
            SystemCapability::CameraAccess,
            serde_json::json!({
                "operation": "start_video_call",
                "duration": "30_minutes"
            }),
        ).unwrap();
        
        // Try to use a capability not granted
        let result = manager.use_capability(
            &session_id,
            SystemCapability::SystemConfigWrite,
            serde_json::json!({}),
        );
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DeviceAuthError::CapabilityNotGranted));
    }
    
    #[test]
    fn test_government_wallet_root_access() {
        let mut manager = DeviceAuthorizationManager::new();
        
        let gov_wallet = WalletIdentity::new(
            "admin",
            WalletProvider::Government("uscitizen".to_string()),
            Some("admin@irs.gov".to_string()),
        ).unwrap();
        
        let trusted_device = DeviceInfo {
            device_id: "gov_device_001".to_string(),
            device_type: DeviceType::Server,
            os_info: OSInfo {
                name: "RHEL".to_string(),
                version: "9.0".to_string(),
                architecture: "x86_64".to_string(),
                security_patches: vec!["All current".to_string()],
            },
            hardware_info: HardwareInfo {
                cpu: "Intel Xeon".to_string(),
                memory_gb: 128,
                storage_gb: 2048,
                has_tpm: true,
                has_secure_enclave: true,
            },
            security_features: SecurityFeatures {
                has_biometric: true,
                has_hardware_key: true,
                has_secure_boot: true,
                has_tpm: true,
                encryption_enabled: true,
            },
            trust_level: TrustLevel::HighlyTrusted,
        };
        
        let session_id = manager.request_authorization(
            &gov_wallet,
            AccessLevel::Root,
            vec![SystemCapability::FullSystemControl],
            trusted_device,
        ).unwrap();
        
        let session = manager.get_session(&session_id).unwrap();
        assert_eq!(session.access_level, AccessLevel::Root);
        assert!(session.granted_capabilities.contains(&SystemCapability::FullSystemControl));
    }
}
