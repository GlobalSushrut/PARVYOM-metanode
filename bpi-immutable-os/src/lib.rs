// BPI Immutable OS Library
// Exposes kernel modules and core functionality for testing and integration

pub mod blockchain_os_kernel;
pub mod filesystem_engine;
pub mod hardware_detection;
pub mod security_hardening;
pub mod atomic_updates;
pub mod bpi_integration;

// Re-export main kernel components for easy access
pub use blockchain_os_kernel::{
    BlockchainOSKernel,
    SmartContractScheduler,
    BlockchainResourceManager,
    QuantumSecurityEnforcer,
    VMApplicationOrchestrator,
};

// Re-export key types
pub use blockchain_os_kernel::{
    KernelState,
    ProcessInfo,
    ProcessType,
    ResourceAllocation,
    SecurityContext,
    SecurityLevel,
    ProcessState,
    KernelError,
};
