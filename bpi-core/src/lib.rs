// Library interface for BPI Core modules
// This allows integration tests to access internal modules

pub mod immutable_audit_system;
pub mod audit_http_server;
pub mod health;
pub mod errors;
pub mod config;
pub mod diagnostics;
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
pub mod vpod_bpi_coordinator;
pub mod four_d_database_bridge;
pub mod four_d_bridge_integration_tests;
pub mod four_d_bridge_standalone_test;
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
pub mod ziplock_human_bundle_v2; // Ziplock Human Bundle v2 for forensic recording
pub mod cbor_pipeline_foundation; // CBOR Pipeline Foundation for government compliance
pub mod pravyom_integration; // Pravyom integration modules
pub mod client; // Stage 4: Advanced Transport Integration - Production Client SDK
pub mod domain_management_api; // Production-ready domain registration API service
// pub mod xtmp_integration_test; // Temporarily disabled due to compiler ICE

// Advanced foundation grant test modules
pub mod quantum_entanglement;
pub mod logbook_6d_bridge; // 6D Blockchain Bridge for logbook entries
pub mod consensus;
pub mod interoperability;

// Re-export commonly used types for integration tests
pub use immutable_audit_system::ImmutableAuditSystem;
pub use health::{HealthChecker, HealthStatus, ServiceHealth};
pub use errors::{BpiError, BpiResult, ErrorContext, ErrorSeverity};
pub use config::{BpiConfig, NetworkConfig, SecurityConfig, StorageConfig, LoggingConfig, PilotConfig};
pub use bpi_action_vm::{BpiActionVM, ContractType};
pub use orchestration_vm::OrchestrationVM;
pub use universal_audit_vm::UniversalAuditVM;
pub use distributed_storage::{BpiDistributedStorage, ContainerBlock, CloudProvider};
pub use enhanced_cdn_storage::{EnhancedCdnStorage, ContentType, CueStoragePolicy, CdntNetwork};
pub use control_fedrate_network::{ControlFedrateNetwork, FedrateNode, NodeSpecialization, ComponentType, MemoryStatus};
