//! Error types for DockLock determinism cage

use thiserror::Error;

/// Errors that can occur during deterministic execution
#[derive(Error, Debug)]
pub enum DockLockError {
    /// Seccomp filter errors
    #[error("Seccomp filter error: {0}")]
    SeccompError(String),

    /// Witness recording errors
    #[error("Witness recording error: {0}")]
    WitnessError(String),

    /// RNG seeding errors
    #[error("RNG seeding error: {0}")]
    RngError(String),

    /// Encoding error
    #[error("Encoding error: {0}")]
    EncodingError(String),

    /// Already exists error
    #[error("Already exists: {0}")]
    AlreadyExists(String),

    /// Capacity exceeded error
    #[error("Capacity exceeded: {0}")]
    CapacityExceeded(String),

    /// System error
    #[error("System error: {0}")]
    SystemError(String),

    /// Internal error
    #[error("Internal error: {0}")]
    InternalError(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Invalid state error
    #[error("Invalid state: {0}")]
    InvalidState(String),

    /// Cryptographic errors
    #[error("Cryptographic error: {0}")]
    CryptoError(String),

    /// Event stream errors
    #[error("Event reorder detected: {0}")]
    EventReorderDetected(String),

    /// Merkle tree errors
    #[error("Merkle tree error: {0}")]
    MerkleError(String),

    /// Access denied errors
    #[error("Access denied: {0}")]
    AccessDenied(String),

    /// Not found errors
    #[error("Not found: {0}")]
    NotFound(String),

    /// Invalid operation errors
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    /// Execution failed
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    /// Integration errors
    #[error("Integration error: {0}")]
    Integration(String),

    /// Concurrency errors
    #[error("Concurrency error: {0}")]
    ConcurrencyError(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Policy engine errors
    #[error("Policy error: {0}")]
    PolicyError(String),

    /// Registry errors
    #[error("Registry error: {0}")]
    RegistryError(String),

    /// Authentication errors
    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    /// Non-deterministic behavior detected
    #[error("Non-deterministic behavior detected: {0}")]
    NonDeterministic(String),

    /// Witness log too large
    #[error("Witness log exceeded maximum size: {size} bytes")]
    WitnessLogTooLarge { size: usize },

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// System time error
    #[error("System time error: {0}")]
    SystemTime(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),

    /// Encoding error
    #[error("Encoding error: {0}")]
    Encoding(#[from] bpi_enc::EncodingError),

    /// Merkle tree error
    #[error("Merkle tree error: {0}")]
    MerkleTree(#[from] bpi_merkle::MerkleError),
}

/// Result type for DockLock operations
pub type DockLockResult<T> = Result<T, DockLockError>;
