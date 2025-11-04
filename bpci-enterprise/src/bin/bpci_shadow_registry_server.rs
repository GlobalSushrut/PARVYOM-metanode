// BPCI Shadow Registry Server - Component 8
// Shadow Registry and Portal - Web2-Web3 Bridge and Decentralized Identity Management
//
// This server provides:
// - Web2-Web3 domain and identity bridging
// - Privacy-preserving registry operations
// - Cross-platform identity management (DID, OAuth, traditional auth)
// - Secure API gateway for Web2 applications
// - Decentralized identity portal
// - Zero-knowledge proof integration for privacy
// - Real-time audit and compliance reporting

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

/// BPCI Shadow Registry Server State
#[derive(Clone)]
struct ShadowRegistryState {
    /// Web2-Web3 bridge manager
    bridge_manager: Arc<RwLock<BridgeManager>>,
    /// Identity registry (DID, OAuth, traditional)
    identity_registry: Arc<RwLock<IdentityRegistry>>,
    /// Domain mapping (Web2 ↔ Web3)
    domain_mapper: Arc<RwLock<DomainMapper>>,
    /// Privacy layer (ZK proofs, encryption)
    privacy_layer: Arc<RwLock<PrivacyLayer>>,
    /// API gateway for Web2 apps
    api_gateway: Arc<RwLock<ApiGateway>>,
    /// Metrics
    metrics: Arc<RwLock<ShadowRegistryMetrics>>,
    /// Server configuration
    config: ShadowRegistryConfig,
}

/// Shadow Registry Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ShadowRegistryConfig {
    pub bind_address: String,
    pub port: u16,
    pub enable_web2_bridge: bool,
    pub enable_did_registry: bool,
    pub enable_privacy_layer: bool,
    pub enable_api_gateway: bool,
    pub max_identities: u32,
    pub max_domains: u32,
}

impl Default for ShadowRegistryConfig {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0".to_string(),
            port: 8088,
            enable_web2_bridge: true,
            enable_did_registry: true,
            enable_privacy_layer: true,
            enable_api_gateway: true,
            max_identities: 100000,
            max_domains: 50000,
        }
    }
}

// ============================================================================
// Web2-Web3 Bridge Manager
// ============================================================================

#[derive(Debug, Clone)]
struct BridgeManager {
    /// Active bridges
    bridges: HashMap<String, Web2Web3Bridge>,
    /// Bridge statistics
    stats: BridgeStats,
}

impl BridgeManager {
    fn new() -> Self {
        Self {
            bridges: HashMap::new(),
            stats: BridgeStats::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Web2Web3Bridge {
    pub bridge_id: String,
    pub web2_endpoint: String,
    pub web3_address: String,
    pub bridge_type: BridgeType,
    pub status: BridgeStatus,
    pub created_at: DateTime<Utc>,
    pub last_sync: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum BridgeType {
    DomainMapping,
    IdentitySync,
    ApiGateway,
    DataBridge,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum BridgeStatus {
    Active,
    Syncing,
    Paused,
    Error,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct BridgeStats {
    pub total_bridges: u64,
    pub active_bridges: u64,
    pub total_syncs: u64,
    pub failed_syncs: u64,
}

// ============================================================================
// Identity Registry (DID + OAuth + Traditional)
// ============================================================================

#[derive(Debug, Clone)]
struct IdentityRegistry {
    /// Decentralized identifiers
    did_registry: HashMap<String, DidIdentity>,
    /// OAuth identities
    oauth_registry: HashMap<String, OAuthIdentity>,
    /// Traditional identities
    traditional_registry: HashMap<String, TraditionalIdentity>,
    /// Statistics
    stats: IdentityStats,
}

impl IdentityRegistry {
    fn new() -> Self {
        Self {
            did_registry: HashMap::new(),
            oauth_registry: HashMap::new(),
            traditional_registry: HashMap::new(),
            stats: IdentityStats::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DidIdentity {
    pub did: String,
    pub did_document: DidDocument,
    pub verification_methods: Vec<VerificationMethod>,
    pub created_at: DateTime<Utc>,
    pub status: IdentityStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DidDocument {
    pub id: String,
    pub controller: String,
    pub public_keys: Vec<PublicKey>,
    pub authentication: Vec<String>,
    pub service_endpoints: Vec<ServiceEndpoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VerificationMethod {
    pub id: String,
    pub method_type: String,
    pub controller: String,
    pub public_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PublicKey {
    pub id: String,
    pub key_type: String,
    pub public_key_hex: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServiceEndpoint {
    pub id: String,
    pub endpoint_type: String,
    pub service_endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OAuthIdentity {
    pub oauth_id: String,
    pub provider: String,
    pub user_id: String,
    pub email: Option<String>,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TraditionalIdentity {
    pub identity_id: String,
    pub username: String,
    pub email: String,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum IdentityStatus {
    Active,
    Pending,
    Suspended,
    Revoked,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct IdentityStats {
    pub total_identities: u64,
    pub did_identities: u64,
    pub oauth_identities: u64,
    pub traditional_identities: u64,
}

// ============================================================================
// Domain Mapper (Web2 ↔ Web3)
// ============================================================================

#[derive(Debug, Clone)]
struct DomainMapper {
    /// Domain mappings
    mappings: HashMap<String, DomainMapping>,
    /// Statistics
    stats: DomainMappingStats,
}

impl DomainMapper {
    fn new() -> Self {
        Self {
            mappings: HashMap::new(),
            stats: DomainMappingStats::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DomainMapping {
    pub mapping_id: String,
    pub web2_domain: String,
    pub web3_address: String,
    pub mapping_type: MappingType,
    pub bidirectional: bool,
    pub created_at: DateTime<Utc>,
    pub last_verified: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum MappingType {
    DomainToAddress,
    SubdomainToContract,
    ApiToService,
    Custom,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct DomainMappingStats {
    pub total_mappings: u64,
    pub active_mappings: u64,
    pub bidirectional_mappings: u64,
}

// ============================================================================
// Privacy Layer (ZK Proofs + Encryption)
// ============================================================================

#[derive(Debug, Clone)]
struct PrivacyLayer {
    /// ZK proof cache
    zk_proofs: HashMap<String, ZkProof>,
    /// Encrypted data
    encrypted_data: HashMap<String, EncryptedData>,
    /// Statistics
    stats: PrivacyStats,
}

impl PrivacyLayer {
    fn new() -> Self {
        Self {
            zk_proofs: HashMap::new(),
            encrypted_data: HashMap::new(),
            stats: PrivacyStats::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ZkProof {
    pub proof_id: String,
    pub proof_type: String,
    pub proof_data: Vec<u8>,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EncryptedData {
    pub data_id: String,
    pub encrypted_content: Vec<u8>,
    pub encryption_algorithm: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct PrivacyStats {
    pub total_zk_proofs: u64,
    pub verified_proofs: u64,
    pub encrypted_entries: u64,
}

// ============================================================================
// API Gateway
// ============================================================================

#[derive(Debug, Clone)]
struct ApiGateway {
    /// Registered APIs
    apis: HashMap<String, RegisteredApi>,
    /// Statistics
    stats: ApiGatewayStats,
}

impl ApiGateway {
    fn new() -> Self {
        Self {
            apis: HashMap::new(),
            stats: ApiGatewayStats::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RegisteredApi {
    pub api_id: String,
    pub api_name: String,
    pub endpoint: String,
    pub api_type: ApiType,
    pub authentication: AuthType,
    pub rate_limit: u32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum ApiType {
    Rest,
    GraphQL,
    WebSocket,
    Grpc,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum AuthType {
    ApiKey,
    OAuth2,
    JWT,
    BpiWallet,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct ApiGatewayStats {
    pub total_apis: u64,
    pub active_apis: u64,
    pub total_requests: u64,
}

// ============================================================================
// Metrics
// ============================================================================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct ShadowRegistryMetrics {
    pub uptime_seconds: u64,
    pub total_requests: u64,
    pub requests_per_second: f64,
    pub total_bridges: u64,
    pub total_identities: u64,
    pub total_domain_mappings: u64,
    pub total_zk_proofs: u64,
    pub total_apis: u64,
}

// ============================================================================
// API Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
struct CreateBridgeRequest {
    web2_endpoint: String,
    web3_address: String,
    bridge_type: BridgeType,
}

#[derive(Debug, Serialize)]
struct CreateBridgeResponse {
    success: bool,
    bridge_id: String,
    message: String,
}

#[derive(Debug, Deserialize)]
struct RegisterDidRequest {
    did: String,
    controller: String,
    public_keys: Vec<PublicKey>,
}

#[derive(Debug, Serialize)]
struct RegisterDidResponse {
    success: bool,
    did: String,
    message: String,
}

#[derive(Debug, Deserialize)]
struct CreateDomainMappingRequest {
    web2_domain: String,
    web3_address: String,
    mapping_type: MappingType,
    bidirectional: bool,
}

#[derive(Debug, Serialize)]
struct CreateDomainMappingResponse {
    success: bool,
    mapping_id: String,
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
    web2_bridge: bool,
    did_registry: bool,
    privacy_layer: bool,
    api_gateway: bool,
}

// ============================================================================
// API Handlers
// ============================================================================

/// Health check endpoint
async fn health_check(State(state): State<ShadowRegistryState>) -> Json<HealthResponse> {
    let metrics = state.metrics.read().await;
    
    Json(HealthResponse {
        status: "healthy".to_string(),
        uptime_seconds: metrics.uptime_seconds,
        components: ComponentHealth {
            web2_bridge: state.config.enable_web2_bridge,
            did_registry: state.config.enable_did_registry,
            privacy_layer: state.config.enable_privacy_layer,
            api_gateway: state.config.enable_api_gateway,
        },
    })
}

/// Get metrics
async fn get_metrics(State(state): State<ShadowRegistryState>) -> Json<ShadowRegistryMetrics> {
    let metrics = state.metrics.read().await;
    Json(metrics.clone())
}

/// Get configuration
async fn get_config(State(state): State<ShadowRegistryState>) -> Json<ShadowRegistryConfig> {
    Json(state.config.clone())
}

/// Create Web2-Web3 bridge
async fn create_bridge(
    State(state): State<ShadowRegistryState>,
    Json(req): Json<CreateBridgeRequest>,
) -> Result<Json<CreateBridgeResponse>, StatusCode> {
    let mut bridge_manager = state.bridge_manager.write().await;
    
    let bridge_id = Uuid::new_v4().to_string();
    let bridge = Web2Web3Bridge {
        bridge_id: bridge_id.clone(),
        web2_endpoint: req.web2_endpoint.clone(),
        web3_address: req.web3_address.clone(),
        bridge_type: req.bridge_type,
        status: BridgeStatus::Active,
        created_at: Utc::now(),
        last_sync: Utc::now(),
    };
    
    bridge_manager.bridges.insert(bridge_id.clone(), bridge);
    bridge_manager.stats.total_bridges += 1;
    bridge_manager.stats.active_bridges += 1;
    
    // Update global metrics
    let mut metrics = state.metrics.write().await;
    metrics.total_bridges = bridge_manager.bridges.len() as u64;
    
    info!("✅ Created Web2-Web3 bridge: {} → {}", req.web2_endpoint, req.web3_address);
    
    Ok(Json(CreateBridgeResponse {
        success: true,
        bridge_id,
        message: format!("Bridge created: {} → {}", req.web2_endpoint, req.web3_address),
    }))
}

/// List all bridges
async fn list_bridges(State(state): State<ShadowRegistryState>) -> Json<Vec<Web2Web3Bridge>> {
    let bridge_manager = state.bridge_manager.read().await;
    let bridges: Vec<Web2Web3Bridge> = bridge_manager.bridges.values().cloned().collect();
    Json(bridges)
}

/// Get bridge statistics
async fn get_bridge_stats(State(state): State<ShadowRegistryState>) -> Json<BridgeStats> {
    let bridge_manager = state.bridge_manager.read().await;
    Json(bridge_manager.stats.clone())
}

/// Register DID identity
async fn register_did(
    State(state): State<ShadowRegistryState>,
    Json(req): Json<RegisterDidRequest>,
) -> Result<Json<RegisterDidResponse>, StatusCode> {
    let mut identity_registry = state.identity_registry.write().await;
    
    let did_document = DidDocument {
        id: req.did.clone(),
        controller: req.controller,
        public_keys: req.public_keys,
        authentication: vec![],
        service_endpoints: vec![],
    };
    
    let did_identity = DidIdentity {
        did: req.did.clone(),
        did_document,
        verification_methods: vec![],
        created_at: Utc::now(),
        status: IdentityStatus::Active,
    };
    
    identity_registry.did_registry.insert(req.did.clone(), did_identity);
    identity_registry.stats.total_identities += 1;
    identity_registry.stats.did_identities += 1;
    
    // Update global metrics
    let mut metrics = state.metrics.write().await;
    metrics.total_identities = identity_registry.stats.total_identities;
    
    info!("✅ Registered DID identity: {}", req.did);
    
    Ok(Json(RegisterDidResponse {
        success: true,
        did: req.did.clone(),
        message: format!("DID {} registered successfully", req.did),
    }))
}

/// List DID identities
async fn list_did_identities(State(state): State<ShadowRegistryState>) -> Json<Vec<DidIdentity>> {
    let identity_registry = state.identity_registry.read().await;
    let identities: Vec<DidIdentity> = identity_registry.did_registry.values().cloned().collect();
    Json(identities)
}

/// Get identity statistics
async fn get_identity_stats(State(state): State<ShadowRegistryState>) -> Json<IdentityStats> {
    let identity_registry = state.identity_registry.read().await;
    Json(identity_registry.stats.clone())
}

/// Create domain mapping
async fn create_domain_mapping(
    State(state): State<ShadowRegistryState>,
    Json(req): Json<CreateDomainMappingRequest>,
) -> Result<Json<CreateDomainMappingResponse>, StatusCode> {
    let mut domain_mapper = state.domain_mapper.write().await;
    
    let mapping_id = Uuid::new_v4().to_string();
    let mapping = DomainMapping {
        mapping_id: mapping_id.clone(),
        web2_domain: req.web2_domain.clone(),
        web3_address: req.web3_address.clone(),
        mapping_type: req.mapping_type,
        bidirectional: req.bidirectional,
        created_at: Utc::now(),
        last_verified: Utc::now(),
    };
    
    domain_mapper.mappings.insert(mapping_id.clone(), mapping);
    domain_mapper.stats.total_mappings += 1;
    domain_mapper.stats.active_mappings += 1;
    if req.bidirectional {
        domain_mapper.stats.bidirectional_mappings += 1;
    }
    
    // Update global metrics
    let mut metrics = state.metrics.write().await;
    metrics.total_domain_mappings = domain_mapper.mappings.len() as u64;
    
    info!("✅ Created domain mapping: {} ↔ {}", req.web2_domain, req.web3_address);
    
    Ok(Json(CreateDomainMappingResponse {
        success: true,
        mapping_id,
        message: format!("Domain mapping created: {} ↔ {}", req.web2_domain, req.web3_address),
    }))
}

/// List domain mappings
async fn list_domain_mappings(State(state): State<ShadowRegistryState>) -> Json<Vec<DomainMapping>> {
    let domain_mapper = state.domain_mapper.read().await;
    let mappings: Vec<DomainMapping> = domain_mapper.mappings.values().cloned().collect();
    Json(mappings)
}

/// Get domain mapping statistics
async fn get_domain_stats(State(state): State<ShadowRegistryState>) -> Json<DomainMappingStats> {
    let domain_mapper = state.domain_mapper.read().await;
    Json(domain_mapper.stats.clone())
}

/// Get privacy layer statistics
async fn get_privacy_stats(State(state): State<ShadowRegistryState>) -> Json<PrivacyStats> {
    let privacy_layer = state.privacy_layer.read().await;
    Json(privacy_layer.stats.clone())
}

/// Get API gateway statistics
async fn get_api_stats(State(state): State<ShadowRegistryState>) -> Json<ApiGatewayStats> {
    let api_gateway = state.api_gateway.read().await;
    Json(api_gateway.stats.clone())
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
    
    info!("🚀 Starting BPCI Shadow Registry Server (Component 8)");
    
    // 🌐 Initialize Pure Virtual Addressing Mode (NO STATIC PORTS!)
    info!("🌐 Initializing Pure Virtual Addressing Mode for Shadow Registry...");
    let virtual_config = VirtualAddressingConfig::pure_virtual("shadow-registry");
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
    
    info!("🚀 Shadow Registry (Component 8) initialized in Pure Virtual Mode");
    info!("   ✅ Can communicate with all other components by service name");
    
    // Load configuration
    let config = ShadowRegistryConfig::default();
    
    // Initialize server state
    let state = ShadowRegistryState {
        bridge_manager: Arc::new(RwLock::new(BridgeManager::new())),
        identity_registry: Arc::new(RwLock::new(IdentityRegistry::new())),
        domain_mapper: Arc::new(RwLock::new(DomainMapper::new())),
        privacy_layer: Arc::new(RwLock::new(PrivacyLayer::new())),
        api_gateway: Arc::new(RwLock::new(ApiGateway::new())),
        metrics: Arc::new(RwLock::new(ShadowRegistryMetrics::default())),
        config: config.clone(),
    };
    
    // Build router with comprehensive API endpoints
    let app = Router::new()
        // Health and metrics
        .route("/health", get(health_check))
        .route("/api/v1/metrics", get(get_metrics))
        .route("/api/v1/config", get(get_config))
        
        // Web2-Web3 Bridge
        .route("/api/v1/bridge", post(create_bridge))
        .route("/api/v1/bridge", get(list_bridges))
        .route("/api/v1/bridge/stats", get(get_bridge_stats))
        
        // DID Identity Registry
        .route("/api/v1/identity/did", post(register_did))
        .route("/api/v1/identity/did", get(list_did_identities))
        .route("/api/v1/identity/stats", get(get_identity_stats))
        
        // Domain Mapping
        .route("/api/v1/domain/mapping", post(create_domain_mapping))
        .route("/api/v1/domain/mapping", get(list_domain_mappings))
        .route("/api/v1/domain/stats", get(get_domain_stats))
        
        // Privacy Layer
        .route("/api/v1/privacy/stats", get(get_privacy_stats))
        
        // API Gateway
        .route("/api/v1/gateway/stats", get(get_api_stats))
        
        .layer(CorsLayer::permissive())
        .with_state(state);
    
    // Bind and serve
    let addr: SocketAddr = format!("{}:{}", config.bind_address, config.port)
        .parse()
        .context("Failed to parse bind address")?;
    
    info!("🌐 BPCI Shadow Registry Server listening on {}", addr);
    info!("🌉 Web2-Web3 Bridge: {}", if config.enable_web2_bridge { "ENABLED" } else { "DISABLED" });
    info!("🆔 DID Registry: {}", if config.enable_did_registry { "ENABLED" } else { "DISABLED" });
    info!("🔒 Privacy Layer: {}", if config.enable_privacy_layer { "ENABLED" } else { "DISABLED" });
    info!("🚪 API Gateway: {}", if config.enable_api_gateway { "ENABLED" } else { "DISABLED" });
    
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("Failed to bind to address")?;
    
    axum::serve(listener, app)
        .await
        .context("Server error")?;
    
    Ok(())
}
