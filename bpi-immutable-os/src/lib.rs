// BPI Immutable OS Library
// Exposes kernel modules and core functionality for testing and integration

pub mod atomic_updates;
pub mod blockchain_os_kernel;
pub mod bpi_integration;
pub mod content_store;
pub mod filesystem_engine;
pub mod hardware_detection;
pub mod security_hardening;
pub mod bootable_ledger;

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

// Re-export content store primitives for consumers (e.g. vPods daemon)
pub use content_store::{ContentAddress, ContentStore, HashAlgorithm};
