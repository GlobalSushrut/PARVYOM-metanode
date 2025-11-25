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
pub mod legal_compliance_engine;
pub mod regulatory_compliance_engine;
pub mod bpi_ledger_state;
pub mod shadow_registry_bridge;
pub mod forensic_firewall;
pub mod security;
pub mod os_security_supervisor;
pub mod proof_service;
pub mod vm_server;
pub mod vpod_bpi_coordinator;
pub mod virtual_addressing_system;
pub mod mesh_native_communication;
pub mod privacy_preserving_bundle_system;
pub mod four_d_database_bridge;
pub mod agi_digital_nation_storage;
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
pub mod dynaroute_client;
pub mod dynaroute_registry;
pub mod dynamic_port_config;
pub mod mesh_migration_adapter; // DynaRoute service discovery for BPCI integration
pub mod ziplock_human_bundle_v2; // Ziplock Human Bundle v2 for forensic recording
pub mod cbor_pipeline_foundation; // CBOR Pipeline Foundation for government compliance
pub mod pravyom_integration; // Pravyom integration modules
pub mod client; // Stage 4: Advanced Transport Integration - Production Client SDK
pub mod cli; // Pravyom CLI modules for advanced forensic and infrastructure management
pub mod domain_management_api; // Production-ready domain registration API service
pub mod six_d_blockchain;
pub mod bpi_packet;
pub mod qgc_consensus; // 6D Blockchain integration for immutable transaction recording
pub mod cuedb_enterprise_engine; // Enterprise-grade CueDB with ACID, vPods, and proof-backed storage
pub mod bpi_service_orchestrator; // One-click deployment orchestrator for BPI services
pub mod wallet_address_orchestrator;
pub mod vpods_daemon;
pub mod vpods_control_handler;
pub mod vpods_unix_transport;
pub mod vpods_docklock_integration; // BPI Service Orchestrator for one-click deployment
pub mod cuedb_query_engine; // Production-grade query engine for CueDB
pub mod ipfs_plus_plus_engine; // IPFS++ revolutionary storage engine
pub mod cue_court_data_orchestrator; // CUE Court data orchestration layer
pub mod cli_utilities; // Safe Rust-based CLI utility functions
// pub mod xtmp_integration_test; // Temporarily disabled due to compiler ICE

// CUE Court Database Ecosystem - Revolutionary Data Architecture

// Advanced foundation grant test modules
pub mod quantum_entanglement;
pub mod proof_systems; // Comprehensive 7-proof system architecture (POA, POE, POT, POG, POH + Merkle, ZK, Quantum)
pub mod logbook_6d_bridge; // 6D Blockchain Bridge for logbook entries
pub mod consensus;
pub mod interoperability;
pub mod services; // Service runners for logbook and 6D blockchain with DynaRoute
pub mod blockchain_os_kernel; // Blockchain OS Kernel with tetrabolic mesh architecture

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
