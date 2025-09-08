// Library interface for BPI Core modules
// This allows integration tests to access internal modules

pub mod immutable_audit_system;
pub mod bpi_action_vm;
pub mod orchestration_vm;
pub mod universal_audit_vm;
pub mod court_vm_audit;
pub mod court_node;
pub mod bpi_ledger_state;
pub mod shadow_registry_bridge;
pub mod forensic_firewall;
pub mod security;
pub mod vm_server;
pub mod bpi_wallet_command;
pub mod cue_orchestration;
pub mod stamped_bpi_communication;
pub mod cue_agreement_deployment;
pub mod cue_installer;
pub mod biso_agreement;
pub mod bpi_node_coordinator;
pub mod distributed_storage;
pub mod enhanced_cdn_storage;
pub mod control_fedrate_network;
pub mod xtmp_protocol;
pub mod xtmp_bpci_client;
pub mod bpci_xtmp_server;
pub mod client; // Stage 4: Advanced Transport Integration - Production Client SDK
pub mod domain_management_api; // Production-ready domain registration API service
// pub mod xtmp_integration_test; // Temporarily disabled due to compiler ICE

// Re-export commonly used types for integration tests
pub use immutable_audit_system::ImmutableAuditSystem;
pub use bpi_action_vm::{BpiActionVM, ContractType};
pub use orchestration_vm::OrchestrationVM;
pub use universal_audit_vm::UniversalAuditVM;
pub use distributed_storage::{BpiDistributedStorage, ContainerBlock, CloudProvider};
pub use enhanced_cdn_storage::{EnhancedCdnStorage, ContentType, CueStoragePolicy, CdntNetwork};
pub use control_fedrate_network::{ControlFedrateNetwork, FedrateNode, NodeSpecialization, ComponentType, MemoryStatus};
