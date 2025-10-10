//! Communication Security Module - 100-Year Stable System
//! 
//! This module provides bulletproof, future-proof communication security with
//! government enterprise-grade CBOR integration and impossible-to-hide audit trails.
//! 
//! Components:
//! - TSLSL CBOR Integration: Post-quantum transport security with CBOR serialization
//! - QLocker CBOR Integration: Quantum sync gates with CBOR audit trails
//! - VM-Client Communication Pipeline: Complete client interaction audit
//! - BPI Core Integration: Full blockchain pipeline linkage

pub mod tslsl_cbor_integration;
pub mod qlocker_cbor_integration;
pub mod vm_client_cbor_pipeline;
pub mod bpi_core_communication_bridge;

// Re-export main components for easy access
pub use tslsl_cbor_integration::{
    TslslCborIntegration,
    CborTslslCertificate,
    CborComplianceMetadata,
    CborAuditTrail,
    TslslCborConfig,
};

pub use qlocker_cbor_integration::{
    QLockerCborIntegration,
    CborQuantumSyncGate,
    CborQuantumSession,
    CborQuantumLock,
    QLockerCborConfig,
};

pub use vm_client_cbor_pipeline::{
    VMClientCborPipeline,
    CborClientRequest,
    CborVMResponse,
    CborInteractionAudit,
};

pub use bpi_core_communication_bridge::{
    BpiCoreCommunicationBridge,
    CborCommunicationEvent,
    CborBlockchainIntegration,
};

/// Communication Security Error Types
#[derive(Debug, thiserror::Error)]
pub enum CommunicationSecurityError {
    #[error("TSLSL CBOR integration error: {0}")]
    TslslCborError(String),
    
    #[error("QLocker CBOR integration error: {0}")]
    QLockerCborError(String),
    
    #[error("VM-Client pipeline error: {0}")]
    VMClientPipelineError(String),
    
    #[error("BPI Core integration error: {0}")]
    BpiCoreIntegrationError(String),
    
    #[error("CBOR serialization error: {0}")]
    CborSerializationError(String),
    
    #[error("Audit trail error: {0}")]
    AuditTrailError(String),
    
    #[error("Government compliance error: {0}")]
    GovernmentComplianceError(String),
    
    #[error("Quantum safety validation error: {0}")]
    QuantumSafetyError(String),
    
    #[error("Cryptographic witness error: {0}")]
    CryptographicWitnessError(String),
    
    #[error("100-year stability validation error: {0}")]
    StabilityValidationError(String),
}

/// Communication Security Result Type
pub type CommunicationSecurityResult<T> = std::result::Result<T, CommunicationSecurityError>;

/// 100-Year Stability Constants
pub mod stability_constants {
    /// CBOR serialization version for 100-year compatibility
    pub const CBOR_VERSION: &str = "1.0.0-STABLE-100Y";
    
    /// Government compliance version
    pub const COMPLIANCE_VERSION: &str = "GOV-ENTERPRISE-2024";
    
    /// Quantum safety standard
    pub const QUANTUM_SAFETY_STANDARD: &str = "POST-QUANTUM-2024";
    
    /// Audit trail format version
    pub const AUDIT_TRAIL_VERSION: &str = "IMPOSSIBLE-TO-HIDE-V1";
    
    /// BPI Core integration version
    pub const BPI_CORE_INTEGRATION_VERSION: &str = "BLOCKCHAIN-PIPELINE-V1";
    
    /// Retention period in years (minimum government requirement)
    pub const RETENTION_YEARS: u32 = 7;
    
    /// Maximum supported retention period for future-proofing
    pub const MAX_RETENTION_YEARS: u32 = 100;
}
