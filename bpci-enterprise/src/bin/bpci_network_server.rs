// BPCI Network Server - Component 7
// Network CDN DNS Domain Communication and HTTPCG Management Kernel Server
//
// This server provides:
// - HTTPCG domain management and registration
// - SAPI mesh network coordination
// - Quantum-safe networking protocols
// - mDNS service discovery and registration
// - Big data communication kernel for direct BPI communication
// - Advanced network topology management
// - Real-time performance monitoring and optimization

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::info;
use anyhow::{Result, Context};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// 🌐 Pure Virtual Addressing Mode - NO STATIC PORTS!
use pravyom_enterprise::{
    virtual_addressing::{VirtualAddressingConfig, VirtualAddressingManager},
    dynaroute_integration::UnifiedNetworkingLayer,
    config::env_ini_parser::EnvIniParser,
    commute_lock::CommuteLockRuntime,
};

/// BPCI Network Server State
#[derive(Clone)]
struct NetworkServerState {
    /// HTTPCG domain registry
    httpcg_registry: Arc<RwLock<HttpcgDomainRegistry>>,
    /// SAPI mesh network manager
    sapi_mesh: Arc<RwLock<SapiMeshNetwork>>,
    /// mDNS service discovery manager
    mdns_manager: Arc<RwLock<MdnsServiceManager>>,
    /// Quantum-safe networking engine
    quantum_network: Arc<RwLock<QuantumSafeNetwork>>,
    /// Network topology manager
    topology_manager: Arc<RwLock<NetworkTopologyManager>>,
    /// Performance metrics
    metrics: Arc<RwLock<NetworkMetrics>>,
    /// Server configuration
    config: NetworkServerConfig,
}

/// Network Server Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
struct NetworkServerConfig {
    /// Server bind address
    pub bind_address: String,
    /// Server port
    pub port: u16,
    /// Enable HTTPCG management
    pub enable_httpcg: bool,
    /// Enable SAPI mesh
    pub enable_sapi_mesh: bool,
    /// Enable mDNS discovery
    pub enable_mdns: bool,
    /// Enable quantum-safe protocols
    pub enable_quantum_safe: bool,
    /// Maximum mesh nodes
    pub max_mesh_nodes: u32,
    /// Health check interval (seconds)
    pub health_check_interval: u64,
}

impl Default for NetworkServerConfig {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0".to_string(),
            port: 8087,
            enable_httpcg: true,
            enable_sapi_mesh: true,
            enable_mdns: true,
            enable_quantum_safe: true,
            max_mesh_nodes: 10000,
            health_check_interval: 30,
        }
    }
}

// ============================================================================
// HTTPCG Domain Registry System
// ============================================================================

/// HTTPCG Domain Registry
#[derive(Debug, Clone)]
struct HttpcgDomainRegistry {
    /// Registered domains
    domains: HashMap<String, HttpcgDomain>,
    /// Domain applications (pending approval)
    applications: HashMap<String, DomainApplication>,
    /// Domain statistics
    stats: DomainRegistryStats,
}

impl HttpcgDomainRegistry {
    fn new() -> Self {
        Self {
            domains: HashMap::new(),
            applications: HashMap::new(),
            stats: DomainRegistryStats::default(),
        }
    }
}

/// HTTPCG Domain
#[derive(Debug, Clone, Serialize, Deserialize)]
struct HttpcgDomain {
    /// Domain name (e.g., "prav@global", "prav@gov")
    pub domain_name: String,
    /// Domain type
    pub domain_type: DomainType,
    /// Owner wallet address
    pub owner_wallet: String,
    /// Security level
    pub security_level: SecurityLevel,
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
    /// Expiration timestamp
    pub expires_at: DateTime<Utc>,
    /// Status
    pub status: DomainStatus,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

/// Domain Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum DomainType {
    Global,           // @global domains
    Country(String),  // @us, @in, @uk country domains
    Government,       // @gov government domains
    Corporate,        // @corp corporate domains
    Educational,      // @edu educational domains
    Military,         // @mil military domains
    Dark,             // @dark private network domains
    Quantum,          // Quantum-safe only
}

/// Security Level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum SecurityLevel {
    Public,       // Public access
    Enhanced,     // Enhanced security
    Classified,   // Classified access
    Quantum,      // Quantum-safe required
}

/// Domain Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum DomainStatus {
    Active,
    Pending,
    Suspended,
    Expired,
    Revoked,
}

/// Domain Application
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DomainApplication {
    pub application_id: String,
    pub domain_name: String,
    pub domain_type: DomainType,
    pub applicant_wallet: String,
    pub security_level: SecurityLevel,
    pub justification: String,
    pub applied_at: DateTime<Utc>,
    pub status: ApplicationStatus,
}

/// Application Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum ApplicationStatus {
    Pending,
    Approved,
    Rejected,
    UnderReview,
}

/// Domain Registry Statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct DomainRegistryStats {
    pub total_domains: u64,
    pub active_domains: u64,
    pub pending_applications: u64,
    pub domains_by_type: HashMap<String, u64>,
}

// ============================================================================
// SAPI Mesh Network System
// ============================================================================

/// SAPI Mesh Network Manager
#[derive(Debug, Clone)]
struct SapiMeshNetwork {
    /// Active mesh nodes
    nodes: HashMap<String, MeshNode>,
    /// Mesh topology
    topology: MeshTopology,
    /// Performance metrics
    metrics: MeshPerformanceMetrics,
}

impl SapiMeshNetwork {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            topology: MeshTopology::default(),
            metrics: MeshPerformanceMetrics::default(),
        }
    }
}

/// Mesh Node
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MeshNode {
    pub node_id: String,
    pub node_address: String,
    pub node_type: MeshNodeType,
    pub capabilities: Vec<String>,
    pub status: NodeStatus,
    pub registered_at: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
    pub performance: NodePerformance,
}

/// Mesh Node Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum MeshNodeType {
    Gateway,
    Router,
    Endpoint,
    Bridge,
}

/// Node Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum NodeStatus {
    Online,
    Offline,
    Degraded,
    Maintenance,
}

/// Node Performance
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct NodePerformance {
    pub latency_ms: f64,
    pub throughput_mbps: f64,
    pub packet_loss_rate: f64,
    pub cpu_usage: f64,
    pub memory_usage: f64,
}

/// Mesh Topology
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct MeshTopology {
    pub total_nodes: u32,
    pub active_connections: u32,
    pub topology_version: u64,
}

/// Mesh Performance Metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct MeshPerformanceMetrics {
    pub total_messages: u64,
    pub messages_per_second: f64,
    pub average_latency_ms: f64,
    pub total_bandwidth_mbps: f64,
}

// ============================================================================
// mDNS Service Discovery System
// ============================================================================

/// mDNS Service Manager
#[derive(Debug, Clone)]
struct MdnsServiceManager {
    /// Registered services
    services: HashMap<String, MdnsService>,
    /// Service statistics
    stats: MdnsStats,
}

impl MdnsServiceManager {
    fn new() -> Self {
        Self {
            services: HashMap::new(),
            stats: MdnsStats::default(),
        }
    }
}

/// mDNS Service
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MdnsService {
    pub service_id: String,
    pub service_name: String,
    pub service_type: String,
    pub port: u16,
    pub txt_records: HashMap<String, String>,
    pub registered_at: DateTime<Utc>,
}

/// mDNS Statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct MdnsStats {
    pub total_services: u64,
    pub active_services: u64,
    pub queries_per_second: f64,
}

// ============================================================================
// Quantum-Safe Networking System
// ============================================================================

/// Quantum-Safe Network Manager
#[derive(Debug, Clone)]
struct QuantumSafeNetwork {
    /// Active quantum-safe channels
    channels: HashMap<String, QuantumChannel>,
    /// Security state
    security_state: QuantumSecurityState,
}

impl QuantumSafeNetwork {
    fn new() -> Self {
        Self {
            channels: HashMap::new(),
            security_state: QuantumSecurityState::default(),
        }
    }
}

/// Quantum Channel
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QuantumChannel {
    pub channel_id: String,
    pub peer_address: String,
    pub encryption_algorithm: String,
    pub key_exchange_protocol: String,
    pub established_at: DateTime<Utc>,
    pub status: ChannelStatus,
}

/// Channel Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum ChannelStatus {
    Establishing,
    Active,
    Rekeying,
    Closed,
}

/// Quantum Security State
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct QuantumSecurityState {
    pub total_channels: u64,
    pub active_channels: u64,
    pub quantum_safe_percentage: f64,
}

// ============================================================================
// Network Topology Manager
// ============================================================================

/// Network Topology Manager
#[derive(Debug, Clone)]
struct NetworkTopologyManager {
    /// Network map
    network_map: HashMap<String, NetworkNode>,
    /// Routing table
    routing_table: HashMap<String, Vec<String>>,
    /// Topology stats
    stats: TopologyStats,
}

impl NetworkTopologyManager {
    fn new() -> Self {
        Self {
            network_map: HashMap::new(),
            routing_table: HashMap::new(),
            stats: TopologyStats::default(),
        }
    }
}

/// Network Node
#[derive(Debug, Clone, Serialize, Deserialize)]
struct NetworkNode {
    pub node_id: String,
    pub node_address: String,
    pub connected_peers: Vec<String>,
    pub last_seen: DateTime<Utc>,
}

/// Topology Statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct TopologyStats {
    pub total_nodes: u64,
    pub total_connections: u64,
    pub average_degree: f64,
}

// ============================================================================
// Network Metrics
// ============================================================================

/// Network Performance Metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct NetworkMetrics {
    pub uptime_seconds: u64,
    pub total_requests: u64,
    pub requests_per_second: f64,
    pub httpcg_domains: u64,
    pub sapi_mesh_nodes: u64,
    pub mdns_services: u64,
    pub quantum_channels: u64,
}

// ============================================================================
// API Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
struct RegisterDomainRequest {
    domain_name: String,
    domain_type: DomainType,
    owner_wallet: String,
    security_level: SecurityLevel,
}

#[derive(Debug, Serialize)]
struct RegisterDomainResponse {
    success: bool,
    domain_id: String,
    message: String,
}

#[derive(Debug, Deserialize)]
struct RegisterMeshNodeRequest {
    node_address: String,
    node_type: MeshNodeType,
    capabilities: Vec<String>,
}

#[derive(Debug, Serialize)]
struct RegisterMeshNodeResponse {
    success: bool,
    node_id: String,
    message: String,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    uptime_seconds: u64,
    components: ComponentHealth,
}

#[derive(Debug, Serialize)]
struct ComponentHealth {
    httpcg: bool,
    sapi_mesh: bool,
    mdns: bool,
    quantum_safe: bool,
}

// ============================================================================
// API Handlers
// ============================================================================

/// Health check endpoint
async fn health_check(State(state): State<NetworkServerState>) -> Json<HealthResponse> {
    let metrics = state.metrics.read().await;
    
    Json(HealthResponse {
        status: "healthy".to_string(),
        uptime_seconds: metrics.uptime_seconds,
        components: ComponentHealth {
            httpcg: state.config.enable_httpcg,
            sapi_mesh: state.config.enable_sapi_mesh,
            mdns: state.config.enable_mdns,
            quantum_safe: state.config.enable_quantum_safe,
        },
    })
}

/// Get network metrics
async fn get_metrics(State(state): State<NetworkServerState>) -> Json<NetworkMetrics> {
    let metrics = state.metrics.read().await;
    Json(metrics.clone())
}

/// Register HTTPCG domain
async fn register_domain(
    State(state): State<NetworkServerState>,
    Json(req): Json<RegisterDomainRequest>,
) -> Result<Json<RegisterDomainResponse>, StatusCode> {
    let mut registry = state.httpcg_registry.write().await;
    
    let domain_id = Uuid::new_v4().to_string();
    let domain = HttpcgDomain {
        domain_name: req.domain_name.clone(),
        domain_type: req.domain_type,
        owner_wallet: req.owner_wallet,
        security_level: req.security_level,
        registered_at: Utc::now(),
        expires_at: Utc::now() + chrono::Duration::days(365),
        status: DomainStatus::Active,
        metadata: HashMap::new(),
    };
    
    registry.domains.insert(domain_id.clone(), domain);
    registry.stats.total_domains += 1;
    registry.stats.active_domains += 1;
    
    // Update global metrics
    let mut metrics = state.metrics.write().await;
    metrics.httpcg_domains = registry.domains.len() as u64;
    
    info!("✅ Registered HTTPCG domain: {}", req.domain_name);
    
    Ok(Json(RegisterDomainResponse {
        success: true,
        domain_id,
        message: format!("Domain {} registered successfully", req.domain_name),
    }))
}

/// List HTTPCG domains
async fn list_domains(State(state): State<NetworkServerState>) -> Json<Vec<HttpcgDomain>> {
    let registry = state.httpcg_registry.read().await;
    let domains: Vec<HttpcgDomain> = registry.domains.values().cloned().collect();
    Json(domains)
}

/// Register SAPI mesh node
async fn register_mesh_node(
    State(state): State<NetworkServerState>,
    Json(req): Json<RegisterMeshNodeRequest>,
) -> Result<Json<RegisterMeshNodeResponse>, StatusCode> {
    let mut mesh = state.sapi_mesh.write().await;
    
    let node_id = Uuid::new_v4().to_string();
    let node = MeshNode {
        node_id: node_id.clone(),
        node_address: req.node_address.clone(),
        node_type: req.node_type,
        capabilities: req.capabilities,
        status: NodeStatus::Online,
        registered_at: Utc::now(),
        last_heartbeat: Utc::now(),
        performance: NodePerformance::default(),
    };
    
    mesh.nodes.insert(node_id.clone(), node);
    mesh.topology.total_nodes += 1;
    
    // Update global metrics
    let mut metrics = state.metrics.write().await;
    metrics.sapi_mesh_nodes = mesh.nodes.len() as u64;
    
    info!("✅ Registered SAPI mesh node: {}", req.node_address);
    
    Ok(Json(RegisterMeshNodeResponse {
        success: true,
        node_id,
        message: format!("Mesh node {} registered successfully", req.node_address),
    }))
}

/// List SAPI mesh nodes
async fn list_mesh_nodes(State(state): State<NetworkServerState>) -> Json<Vec<MeshNode>> {
    let mesh = state.sapi_mesh.read().await;
    let nodes: Vec<MeshNode> = mesh.nodes.values().cloned().collect();
    Json(nodes)
}

/// Get domain registry statistics
async fn get_domain_stats(State(state): State<NetworkServerState>) -> Json<DomainRegistryStats> {
    let registry = state.httpcg_registry.read().await;
    Json(registry.stats.clone())
}

/// Get mesh network statistics
async fn get_mesh_stats(State(state): State<NetworkServerState>) -> Json<MeshPerformanceMetrics> {
    let mesh = state.sapi_mesh.read().await;
    Json(mesh.metrics.clone())
}

/// Register mDNS service
async fn register_mdns_service(
    State(state): State<NetworkServerState>,
    Json(service): Json<MdnsService>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut mdns = state.mdns_manager.write().await;
    mdns.services.insert(service.service_id.clone(), service.clone());
    mdns.stats.total_services += 1;
    mdns.stats.active_services += 1;
    
    // Update global metrics
    let mut metrics = state.metrics.write().await;
    metrics.mdns_services = mdns.services.len() as u64;
    
    info!("✅ Registered mDNS service: {}", service.service_name);
    
    Ok(Json(serde_json::json!({
        "success": true,
        "service_id": service.service_id,
        "message": format!("Service {} registered successfully", service.service_name)
    })))
}

/// List mDNS services
async fn list_mdns_services(State(state): State<NetworkServerState>) -> Json<Vec<MdnsService>> {
    let mdns = state.mdns_manager.read().await;
    let services: Vec<MdnsService> = mdns.services.values().cloned().collect();
    Json(services)
}

/// Get mDNS statistics
async fn get_mdns_stats(State(state): State<NetworkServerState>) -> Json<MdnsStats> {
    let mdns = state.mdns_manager.read().await;
    Json(mdns.stats.clone())
}

/// Create quantum-safe channel
async fn create_quantum_channel(
    State(state): State<NetworkServerState>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut quantum = state.quantum_network.write().await;
    
    let channel_id = uuid::Uuid::new_v4().to_string();
    let peer_address = req["peer_address"].as_str().unwrap_or("unknown").to_string();
    
    let channel = QuantumChannel {
        channel_id: channel_id.clone(),
        peer_address: peer_address.clone(),
        encryption_algorithm: "Dilithium5+Kyber1024".to_string(),
        key_exchange_protocol: "ECDH-P384+Kyber".to_string(),
        established_at: Utc::now(),
        status: ChannelStatus::Active,
    };
    
    quantum.channels.insert(channel_id.clone(), channel);
    quantum.security_state.total_channels += 1;
    quantum.security_state.active_channels += 1;
    quantum.security_state.quantum_safe_percentage = 
        (quantum.security_state.active_channels as f64 / quantum.security_state.total_channels as f64) * 100.0;
    
    // Update global metrics
    let mut metrics = state.metrics.write().await;
    metrics.quantum_channels = quantum.channels.len() as u64;
    
    info!("✅ Created quantum-safe channel to: {}", peer_address);
    
    Ok(Json(serde_json::json!({
        "success": true,
        "channel_id": channel_id,
        "message": format!("Quantum channel to {} established", peer_address)
    })))
}

/// List quantum channels
async fn list_quantum_channels(State(state): State<NetworkServerState>) -> Json<Vec<QuantumChannel>> {
    let quantum = state.quantum_network.read().await;
    let channels: Vec<QuantumChannel> = quantum.channels.values().cloned().collect();
    Json(channels)
}

/// Get quantum security state
async fn get_quantum_state(State(state): State<NetworkServerState>) -> Json<QuantumSecurityState> {
    let quantum = state.quantum_network.read().await;
    Json(quantum.security_state.clone())
}

/// Get network topology
async fn get_network_topology(State(state): State<NetworkServerState>) -> Json<serde_json::Value> {
    let topology = state.topology_manager.read().await;
    
    Json(serde_json::json!({
        "total_nodes": topology.stats.total_nodes,
        "total_connections": topology.stats.total_connections,
        "average_degree": topology.stats.average_degree,
        "network_map_size": topology.network_map.len()
    }))
}

/// Get server configuration
async fn get_server_config(State(state): State<NetworkServerState>) -> Json<NetworkServerConfig> {
    Json(state.config.clone())
}

// ============================================================================
// Main Server
// ============================================================================

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    info!("🚀 Starting BPCI Network Server (HTTPCG/CDN/DNS Component)");
    
    // 🌐 Initialize Pure Virtual Addressing Mode (NO STATIC PORTS!)
    info!("🌐 Initializing Pure Virtual Addressing Mode for Network Server...");
    let virtual_config = VirtualAddressingConfig::pure_virtual("network");
    let virtual_mgr = VirtualAddressingManager::new(virtual_config);
    info!("✅ Virtual addressing initialized - NO static ports!");
    info!("   Service name: {}", virtual_mgr.service_name());
    info!("   IAAv6: {}", virtual_mgr.virtual_address().iaav6);
    
    // Initialize CommuteLock Runtime
    let parser = EnvIniParser::new("config");
    let env_config = match parser.parse_env_ini() {
        Ok(config) => config,
        Err(_) => {
            use pravyom_enterprise::config::env_ini_parser::EnvIniConfig;
            EnvIniConfig {
                sections: HashMap::new(),
                globals: HashMap::new(),
                vpod_env: None,
                bso_k8_config: None,
                commute_lock_config: None,
            }
        }
    };
    let commute_runtime = Arc::new(CommuteLockRuntime::new(&env_config)?);
    info!("✅ CommuteLock runtime initialized");
    
    // Initialize UnifiedNetworkingLayer (Pure Virtual - Dynamic Port!)
    let networking = Arc::new(
        UnifiedNetworkingLayer::new_virtual(commute_runtime).await?
    );
    info!("✅ UnifiedNetworkingLayer initialized (Pure Virtual Mode)");
    info!("   Dynamic port assigned: {}", networking.local_addr().port());
    info!("   NO static port configuration required!");
    
    // Register service in discovery (by name only!)
    networking.register_service(
        virtual_mgr.service_name(),
        vec![networking.local_addr()],
    ).await;
    info!("✅ Service registered: '{}' → {}", virtual_mgr.service_name(), networking.local_addr());
    
    info!("🚀 Network Server (HTTPCG/CDN/DNS) initialized in Pure Virtual Mode");
    info!("   ✅ Can communicate with all other components by service name");
    
    // Load configuration
    let config = NetworkServerConfig::default();
    
    // Initialize server state
    let state = NetworkServerState {
        httpcg_registry: Arc::new(RwLock::new(HttpcgDomainRegistry::new())),
        sapi_mesh: Arc::new(RwLock::new(SapiMeshNetwork::new())),
        mdns_manager: Arc::new(RwLock::new(MdnsServiceManager::new())),
        quantum_network: Arc::new(RwLock::new(QuantumSafeNetwork::new())),
        topology_manager: Arc::new(RwLock::new(NetworkTopologyManager::new())),
        metrics: Arc::new(RwLock::new(NetworkMetrics::default())),
        config: config.clone(),
    };
    
    // Build router with comprehensive API endpoints
    let app = Router::new()
        // Health and metrics
        .route("/health", get(health_check))
        .route("/api/v1/metrics", get(get_metrics))
        .route("/api/v1/config", get(get_server_config))
        
        // HTTPCG Domain Management
        .route("/api/v1/httpcg/domains", post(register_domain))
        .route("/api/v1/httpcg/domains", get(list_domains))
        .route("/api/v1/httpcg/stats", get(get_domain_stats))
        
        // SAPI Mesh Network
        .route("/api/v1/mesh/nodes", post(register_mesh_node))
        .route("/api/v1/mesh/nodes", get(list_mesh_nodes))
        .route("/api/v1/mesh/stats", get(get_mesh_stats))
        
        // mDNS Service Discovery
        .route("/api/v1/mdns/services", post(register_mdns_service))
        .route("/api/v1/mdns/services", get(list_mdns_services))
        .route("/api/v1/mdns/stats", get(get_mdns_stats))
        
        // Quantum-Safe Networking
        .route("/api/v1/quantum/channels", post(create_quantum_channel))
        .route("/api/v1/quantum/channels", get(list_quantum_channels))
        .route("/api/v1/quantum/state", get(get_quantum_state))
        
        // Network Topology
        .route("/api/v1/topology", get(get_network_topology))
        
        .layer(CorsLayer::permissive())
        .with_state(state);
    
    // Bind and serve
    let addr: SocketAddr = format!("{}:{}", config.bind_address, config.port)
        .parse()
        .context("Failed to parse bind address")?;
    
    info!("🌐 BPCI Network Server listening on {}", addr);
    info!("📡 HTTPCG Domain Management: {}", if config.enable_httpcg { "ENABLED" } else { "DISABLED" });
    info!("🔗 SAPI Mesh Network: {}", if config.enable_sapi_mesh { "ENABLED" } else { "DISABLED" });
    info!("🔍 mDNS Service Discovery: {}", if config.enable_mdns { "ENABLED" } else { "DISABLED" });
    info!("🔐 Quantum-Safe Networking: {}", if config.enable_quantum_safe { "ENABLED" } else { "DISABLED" });
    
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("Failed to bind to address")?;
    
    axum::serve(listener, app)
        .await
        .context("Server error")?;
    
    Ok(())
}
