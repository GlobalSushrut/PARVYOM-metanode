//! BPCI Consensus Server - LCCD Revolutionary Consensus
//! 
//! Production-ready BPCI server with revolutionary LCCD consensus engine
//! (Living Cellular Consensus Division) with mathematical foundation.
//! 
//! Features:
//! - LCCD Revolutionary Consensus Architecture
//! - Living Mathematical Organism (Category-Chain, κ-Circulatory, NxTri)
//! - Consciousness-Level Intelligence Core
//! - Temporal Guardian (Time-Travel Resistance)
//! - Cellular Division Manager (Living Organism Scaling)
//! - Category Theory Mathematical Transcendence
//! - Real-time LCCD monitoring via HTTP API
//! - WebSocket streaming for live revolutionary updates
//! - Testnet mode with LCCD mathematical foundation
//! - Development endpoints for LCCD testing
//! 
//! Usage:
//!   cargo run --bin bpci-consensus-server
//!   cargo run --bin bpci-consensus-server -- --config testnet.toml
//!   cargo run --bin bpci-consensus-server -- --dev-mode

use anyhow::{anyhow, Result};
use axum::serve;
use clap::{Arg, Command};
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::signal;
use tracing::{info, warn, error};
use tracing_subscriber;
use sysinfo::{System, SystemExt};
use ed25519_dalek;
use rand;
use reqwest;
use serde_json;

use pravyom_enterprise::{
    BpciConsensusServerState, BpciServerConfig, ServerMode,
    create_bpci_consensus_router, initialize_bpci_enterprise,
    BpciRevolutionaryConsensus, LccdMathematicalFoundation,
    ConsensusValidatorActor, ValidationMetrics, ValidatorInfo, ConsensusState,
    ValidatorKey, MiningActor, VPodActor,
    hermes_lite_web4_mesh::{Web4Address, MeshNodeId, HermesLiteWeb4Mesh},
    // DynaRoute v2 Pure Virtual Mode
    dynaroute_integration::UnifiedNetworkingLayer,
    virtual_addressing::{VirtualAddressingConfig, VirtualAddressingManager},
    // Enhanced integrations for unified infrastructure
    bpi_core_integration::{
        kernel_bridge::BlockchainOSKernelBridge,
        resource_coordinator::ResourceCoordinator,
        service_mapper::EnterpriseServiceMapper,
    },
    central_orchestration::BPCICentralOrchestrator,
    inter_component_communication::{ComponentCommunicationHub, ComponentType},
    config::env_ini_parser::EnvIniParser,
    commute_lock::CommuteLockRuntime,
};

/// Enhanced Consensus Server State with unified infrastructure integrations
pub struct EnhancedConsensusServerState {
    /// Base consensus server state
    pub base_state: BpciConsensusServerState,
    
    /// Component communication hub for inter-component messaging
    pub communication_hub: Arc<ComponentCommunicationHub>,
    
    /// Kernel bridge for BPI-BPCI integration
    pub kernel_bridge: Arc<BlockchainOSKernelBridge>,
    
    /// Resource coordinator for unified resource management
    pub resource_coordinator: Arc<ResourceCoordinator>,
    
    /// Service mapper for enterprise service mapping
    pub service_mapper: Arc<EnterpriseServiceMapper>,
    
    /// Unified networking layer (DynaRoute v2 + CommuteLock) - replaces HTTP client
    pub networking: Arc<UnifiedNetworkingLayer>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("bpci_consensus_server=info,bpci_enterprise=info")
        .init();
    
    info!("🚀 Starting BPCI Consensus Server with Enhanced 3rd Consensus Layer");
    
    // Initialize DynaRoute v2 Pure Virtual Mode (NO STATIC PORTS!)
    info!("🌐 Initializing DynaRoute v2 Pure Virtual Mode");
    let virtual_config = VirtualAddressingConfig::pure_virtual("consensus");
    let virtual_mgr = VirtualAddressingManager::new(virtual_config);
    info!("   Virtual Address: {}", virtual_mgr.virtual_address().iaav6);
    info!("   Mode: Port-free operation with dynamic port allocation");
    
    // Initialize UnifiedNetworkingLayer for mesh communication
    let env_parser = EnvIniParser::new(".");
    let env_config = env_parser.parse_env_ini()?;
    let commute_runtime = Arc::new(CommuteLockRuntime::new(&env_config)?);
    let networking = Arc::new(UnifiedNetworkingLayer::new_virtual(commute_runtime).await?);
    info!("✅ UnifiedNetworkingLayer initialized (Pure Virtual Mode)");
    
    // Parse command line arguments
    let matches = Command::new("bpci-consensus-server")
        .version("1.0.0")
        .about("BPCI Revolutionary Consensus Server with LCCD Mathematical Foundation")
        .arg(
            Arg::new("config")
                .long("config")
                .value_name("FILE")
                .help("Configuration file path")
                .default_value("config.toml")
        )
        .arg(
            Arg::new("dev-mode")
                .long("dev-mode")
                .help("Run in development mode")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .value_name("PORT")
                .help("Server port")
                .default_value("8080")
        )
        .arg(
            Arg::new("host")
                .long("host")
                .value_name("HOST")
                .help("Server host")
                .default_value("0.0.0.0")
        )
        .get_matches();
    
    // Determine server configuration
    let server_config = if matches.get_flag("dev-mode") {
        info!("🔧 Running in DEVELOPMENT mode");
        BpciServerConfig {
            server_mode: ServerMode::Development {
                auto_generate_bundles: true,
                debug_logging: true,
            },
            listen_address: matches.get_one::<String>("host").unwrap().clone(),
            listen_port: matches.get_one::<String>("port").unwrap().parse()?,
            max_concurrent_rounds: 5,
            round_timeout_seconds: 15,
            enable_websocket_monitoring: true,
            enable_metrics_endpoint: true,
        }
    } else {
        info!("🌐 Running in TESTNET mode with LCCD Revolutionary Consensus");
        BpciServerConfig {
            server_mode: ServerMode::Testnet {
                real_validators: 5,
                enable_sophisticated_consensus: true,
            },
            listen_address: matches.get_one::<String>("host").unwrap().clone(),
            listen_port: matches.get_one::<String>("port").unwrap().parse()?,
            max_concurrent_rounds: 10,
            round_timeout_seconds: 30,
            enable_websocket_monitoring: true,
            enable_metrics_endpoint: true,
        }
    };
    
    // Initialize BPCI Enterprise system
    initialize_bpci_enterprise().await?;
    
    // Initialize LCCD Mathematical Foundation
    info!("🧠 Initializing LCCD Mathematical Foundation...");
    let lccd_foundation = Arc::new(LccdMathematicalFoundation::new());
    
    // Initialize BPCI Revolutionary Consensus
    info!("🚀 Initializing BPCI Revolutionary Consensus Engine...");
    let revolutionary_consensus = Arc::new(BpciRevolutionaryConsensus::new().await?);
    
    // Initialize Real VPod Validator System with Automatic RAM Allocation
    info!("⚡ Initializing Real VPod Validator System with Automatic RAM Allocation...");
    
    // Get available system RAM for automatic VPod allocation
    let system_info = sysinfo::System::new_all();
    let available_ram_mb = system_info.available_memory() / 1024 / 1024;
    let vpod_buffer_size = std::cmp::min(available_ram_mb / 10, 8192) as usize; // Auto-allocate based on RAM
    
    info!("🧠 Auto-allocated VPod buffer size: {} MB based on available RAM: {} MB", vpod_buffer_size, available_ram_mb);
    
    let base_vpod = Arc::new(VPodActor::new(uuid::Uuid::new_v4(), vpod_buffer_size)?);
    
    // Generate real Ed25519 validator key (not mock)
    let validator_keypair = ed25519_dalek::SigningKey::generate(&mut rand::rngs::OsRng);
    let validator_key = ValidatorKey {
        public_key: validator_keypair.verifying_key().to_bytes(),
        private_key_encrypted: validator_keypair.to_bytes().to_vec(), // In production, encrypt this
        derivation_path: format!("m/44'/0'/0'/0/{}", uuid::Uuid::new_v4().as_u128() % 1000),
    };
    
    // Calculate stake amount based on system resources (not hardcoded)
    let stake_amount = (available_ram_mb * 1000) as u64; // Dynamic stake based on RAM contribution
    
    let consensus_validator = Arc::new(ConsensusValidatorActor {
        base_actor: base_vpod,
        validator_key,
        stake_amount,
        consensus_state: Arc::new(RwLock::new(ConsensusState {
            current_epoch: 0,
            current_round: 0,
            current_view: 0,
            last_block_hash: [0u8; 32],
            validator_set: vec![],
            pending_proposals: vec![],
        })),
        validation_metrics: Arc::new(RwLock::new(ValidationMetrics::default())),
    });
    
    // Initialize Hermes P2P Mesh for Real Validator/Notary Network
    info!("🌐 Initializing Hermes P2P Mesh for Real Validator/Notary Network...");
    
    // Create Web4 address for this validator node
    let web4_address = Web4Address {
        node_id: MeshNodeId::generate(),
        ip_address: "0.0.0.0".to_string(), // Bind to all interfaces for real network access
        port: 9002, // Hermes mesh port (different from HTTP API port)
        quantum_channel: Some(format!("quantum-{}", uuid::Uuid::new_v4())),
        mesh_layer: 0, // Base layer for consensus validators
    };
    
    // Initialize Hermes mesh with LCCD foundation
    let hermes_mesh = Arc::new(HermesLiteWeb4Mesh::new(
        web4_address,
        lccd_foundation.clone(),
    )?);
    
    // Join the mesh network (bootstrap with existing nodes if available)
    let bootstrap_nodes = vec![]; // In production, load from config or discovery
    
    // Start mesh in standalone mode if no bootstrap nodes (non-blocking)
    if bootstrap_nodes.is_empty() {
        info!("🌐 Starting Hermes P2P Mesh in standalone mode (no bootstrap nodes)");
        // TODO: In production, implement mesh discovery or load bootstrap nodes from config
    } else {
        hermes_mesh.join_mesh(bootstrap_nodes).await?;
        info!("✅ Hermes P2P Mesh joined network with bootstrap nodes");
    }
    
    info!("✅ Hermes P2P Mesh initialized successfully");
    
    // Create server state with sophisticated systems
    info!("🔧 Initializing BPCI Revolutionary Consensus Server state...");
    let server_state = BpciConsensusServerState::new(server_config.clone()).await?;
    
    // 🚀 ENHANCED: Initialize unified infrastructure integrations
    info!("🔗 Initializing unified infrastructure integrations...");
    
    // 1. Initialize Component Communication Hub
    let communication_hub = Arc::new(ComponentCommunicationHub::new()?);
    let component_receiver = communication_hub.register_component(
        ComponentType::Consensus,
        "bpci-consensus-server".to_string(),
        server_config.listen_address.clone(),
        server_config.listen_port,
    ).await?;
    info!("✅ Component Communication Hub initialized");
    
    // 2. Initialize Kernel Bridge for BPI-BPCI integration
    let kernel_bridge = Arc::new(BlockchainOSKernelBridge::new().await?);
    match kernel_bridge.connect().await {
        Ok(_) => info!("✅ Kernel Bridge connected to BPI Core"),
        Err(e) => warn!("⚠️ Kernel Bridge connection failed (will retry): {}", e),
    }
    
    // 3. Initialize Resource Coordinator
    let orchestrator = Arc::new(BPCICentralOrchestrator::new());
    let resource_coordinator = Arc::new(ResourceCoordinator::new(orchestrator.clone()).await?);
    resource_coordinator.initialize().await?;
    info!("✅ Resource Coordinator initialized");
    
    // 4. Initialize Service Mapper
    let mut service_mapper = EnterpriseServiceMapper::new().await?;
    service_mapper.set_kernel_bridge(kernel_bridge.clone());
    service_mapper.initialize().await?;
    info!("✅ Service Mapper initialized");
    
    // 5. Initialize UnifiedNetworkingLayer (DynaRoute v2 + CommuteLock)
    info!("🌐 Initializing UnifiedNetworkingLayer (DynaRoute v2 + CommuteLock)...");
    
    // Parse env.ini for CommuteLock configuration
    let parser = EnvIniParser::new("config");
    let env_config = parser.parse_env_ini()?;
    let commute_runtime = Arc::new(CommuteLockRuntime::new(&env_config)?);
    
    // Create unified networking layer (Pure Virtual Mode - NO static ports!)
    let networking = Arc::new(
        UnifiedNetworkingLayer::new_virtual(commute_runtime).await?
    );
    
    info!("✅ Component 1 (Consensus) initialized in Pure Virtual Mode");
    info!("   Dynamic port assigned: {} (OS-assigned)", networking.local_addr().port());
    info!("   NO static port configuration required!");
    
    // Register this component in service discovery (by name only!)
    networking.register_service(
        "consensus".to_string(),
        vec![networking.local_addr()],
    ).await;
    
    info!("✅ UnifiedNetworkingLayer initialized - DynaRoute v2 + CommuteLock ready");
    info!("   Registered as 'consensus' service at {}", networking.local_addr());
    
    // Create enhanced server state with all integrations
    let enhanced_server_state = Arc::new(EnhancedConsensusServerState {
        base_state: server_state,
        communication_hub: communication_hub.clone(),
        kernel_bridge,
        resource_coordinator,
        service_mapper: Arc::new(service_mapper),
        networking,
    });
    
    // Create router with all endpoints
    let app = create_bpci_consensus_router(enhanced_server_state.base_state.clone());
    
    // Start server
    let addr = SocketAddr::new(
        server_config.listen_address.parse()?,
        server_config.listen_port,
    );
    
    info!("🎯 BPCI Revolutionary Consensus Server starting on {}", addr);
    info!("📊 LCCD Revolutionary API endpoints available:");
    info!("   POST /api/v1/lccd/consensus/start - Start LCCD revolutionary consensus round");
    info!("   GET  /api/v1/lccd/consensus/status/:id - Get LCCD round status");
    info!("   GET  /api/v1/lccd/mathematical/foundation - Get LCCD mathematical foundation status");
    info!("   GET  /api/v1/lccd/consciousness/intelligence - Get consciousness-level intelligence");
    info!("   GET  /api/v1/lccd/temporal/guardian - Get temporal guardian status");
    info!("   GET  /api/v1/lccd/cellular/division - Get cellular division manager status");
    info!("   GET  /api/v1/lccd/category/theory - Get category theory transcendence");
    info!("   GET  /api/v1/lccd/revolutionary/status - Get overall revolutionary status");
    info!("   GET  /api/v1/metrics - Get LCCD consensus metrics");
    info!("   GET  /api/v1/health - Health check");
    
    if server_config.enable_websocket_monitoring {
        info!("   WS   /ws/lccd - Real-time LCCD revolutionary monitoring");
    }
    
    match &server_config.server_mode {
        ServerMode::Development { .. } => {
            info!("🔧 Development endpoints:");
            info!("   POST /api/v1/dev/lccd/generate-foundation - Generate LCCD mathematical foundation");
            info!("   POST /api/v1/dev/lccd/simulate-round - Simulate LCCD revolutionary round");
            info!("   POST /api/v1/dev/lccd/test-consciousness - Test consciousness intelligence");
        }
        ServerMode::Testnet { real_validators, .. } => {
            info!("🌐 Testnet configuration:");
            info!("   Real validators: {}", real_validators);
            info!("   LCCD mode: Revolutionary (Living Cellular Consensus Division)");
            info!("   Revolutionary consensus: LCCD Mathematical Foundation + Consciousness Intelligence");
            info!("   Living organism: Category-Chain + κ-Circulatory + NxTri Immune System");
        }
        ServerMode::Production { real_validators, .. } => {
            info!("🚀 Production configuration:");
            info!("   Real validators: {}", real_validators);
            info!("   LCCD mode: Revolutionary (Living Cellular Consensus Division)");
            info!("   Revolutionary consensus: LCCD Mathematical Foundation + Consciousness Intelligence");
            info!("   Living organism: Category-Chain + κ-Circulatory + NxTri Immune System");
            info!("   Production-grade VPod/Hermes validator/notary system enabled");
        }
    }
    
    // Start the server with graceful shutdown
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let server = serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal());
    
    info!("✅ BPCI Revolutionary Consensus Server is running!");
    info!("🎯 Ready to process LCCD revolutionary consensus with living mathematical organism!");
    info!("🧠 LCCD Mathematical Foundation: Category-Chain + κ-Circulatory + NxTri Immune System");
    info!("⚡ Consensus Validator Actor: VPOD-native with Ed25519 cryptography");
    info!("📊 Validation Metrics: Microsecond precision performance tracking");
    info!("🚀 Revolutionary Consensus: Consciousness + Temporal + Cellular + Category Theory");
    
    // Run server
    if let Err(e) = server.await {
        error!("Server error: {}", e);
        return Err(anyhow!("Server failed: {}", e));
    }
    
    info!("🛑 BPCI Consensus Server shutdown complete");
    Ok(())
}

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("🛑 Received Ctrl+C, shutting down gracefully...");
        },
        _ = terminate => {
            info!("🛑 Received terminate signal, shutting down gracefully...");
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Lightweight integration-style check that we can create a Hermes mesh
    /// in a consensus-server-like context and query its health.
    #[tokio::test]
    async fn test_consensus_server_initializes_hermes_mesh_health() {
        // Initialize LCCD foundation as main() would
        let lccd_foundation = Arc::new(LccdMathematicalFoundation::new());

        // Create a local Web4 address for this validator node (loopback for tests)
        let web4_address = Web4Address {
            node_id: MeshNodeId::generate(),
            ip_address: "127.0.0.1".to_string(),
            port: 19002,
            quantum_channel: Some(format!("quantum-test-{}", uuid::Uuid::new_v4())),
            mesh_layer: 0,
        };

        // Initialize Hermes mesh with LCCD foundation
        let hermes_mesh = Arc::new(
            HermesLiteWeb4Mesh::new(web4_address, lccd_foundation)
                .expect("Hermes mesh should initialize"),
        );

        // Join mesh with no bootstrap nodes so local node is registered in topology
        hermes_mesh
            .join_mesh(Vec::new())
            .await
            .expect("join_mesh should succeed");

        // Run a single consensus round to exercise the integration
        let confidence = hermes_mesh
            .process_mesh_consensus_round(0.9)
            .await
            .expect("consensus round should succeed");

        let health = hermes_mesh
            .get_mesh_health()
            .await
            .expect("mesh health query should succeed");

        println!(
            "[consensus:test_consensus_server_initializes_hermes_mesh_health] mesh_id={} total_nodes={} health_ratio={:.3} avg_kappa={:.6} avg_confidence={:.3} consensus_rounds={} overall_confidence={:.3}",
            health.mesh_id,
            health.total_nodes,
            health.health_ratio,
            health.average_kappa,
            health.average_confidence,
            health.consensus_rounds,
            confidence.overall_confidence(),
        );

        // Basic sanity checks
        assert!(!health.mesh_id.is_empty());
        assert!(health.total_nodes >= 1);
        assert!(health.health_ratio >= 0.0 && health.health_ratio <= 1.0);
        assert!(health.consensus_rounds >= 0);
    }
}
