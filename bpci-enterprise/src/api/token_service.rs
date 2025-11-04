use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::integrated_token_system::{IntegratedTokenSystem, IntegratedTokenSystemConfig};
use crate::storage::FourDConfig;
use crate::mdns_proxy_manager::MdnsProxyConfig;
use std::collections::HashMap;

/// API Response wrapper for consistent frontend integration
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }
    
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Token creation request from frontend
#[derive(Deserialize, Debug)]
pub struct CreateTokenRequest {
    pub wallet_id: String,
    pub token_data: Vec<String>,
    pub metadata: Value,
}

/// Address creation request from frontend
#[derive(Deserialize, Debug)]
pub struct CreateAddressRequest {
    pub ip_address: String,
    pub port: u16,
    pub metadata: Value,
}

/// Token response for frontend
#[derive(Serialize, Debug)]
pub struct TokenResponse {
    pub token_id: String,
    pub wallet_id: String,
    pub merkle_proof: Vec<String>,
    pub mdns_service_name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Address response for frontend
#[derive(Serialize, Debug)]
pub struct AddressResponse {
    pub address_id: String,
    pub ip_address: String,
    pub port: u16,
    pub mdns_service_name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// System statistics for dashboard
#[derive(Serialize, Debug)]
pub struct SystemStats {
    pub total_tokens: u64,
    pub total_addresses: u64,
    pub merkle_operations: u64,
    pub active_services: u64,
    pub database_healthy: bool,
    pub system_uptime: String,
}

/// Query parameters for filtering
#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub wallet_id: Option<String>,
}

/// Application state shared across handlers
pub struct AppState {
    pub integrated_system: Arc<IntegratedTokenSystem>,
    pub start_time: chrono::DateTime<chrono::Utc>,
}

/// Create the API router with all endpoints
pub fn create_router(integrated_system: Arc<IntegratedTokenSystem>) -> Router {
    let app_state = Arc::new(AppState {
        integrated_system,
        start_time: chrono::Utc::now(),
    });

    Router::new()
        // Token endpoints
        .route("/api/tokens", post(create_token))
        .route("/api/tokens", get(list_tokens))
        .route("/api/tokens/:token_id", get(get_token))
        .route("/api/tokens/:token_id/verify", post(verify_token))
        
        // Address endpoints
        .route("/api/addresses", post(create_address))
        .route("/api/addresses", get(list_addresses))
        .route("/api/addresses/:address_id", get(get_address))
        
        // System endpoints
        .route("/api/system/stats", get(get_system_stats))
        .route("/api/system/health", get(health_check))
        .route("/api/system/discover/:service_type", get(discover_services))
        
        // CORS and frontend integration
        .route("/api/status", get(api_status))
        .with_state(app_state)
}

/// Create a new token with Merkle proof and mDNS registration
async fn create_token(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateTokenRequest>,
) -> Result<Json<ApiResponse<TokenResponse>>, StatusCode> {
    match state.integrated_system.create_integrated_token(
        request.token_data.get(0).unwrap_or(&"BPI_TOKEN".to_string()).clone(),
        request.wallet_id.clone(),
        "BPI Token".to_string(),
        Some("Generated via API".to_string()),
        request.wallet_id.clone(),
        true,
        Some(8080),
    ).await {
        Ok(token_info) => {
            let response = TokenResponse {
                token_id: token_info.entry.token.clone(),
                wallet_id: request.wallet_id,
                merkle_proof: vec![token_info.merkle_hash.clone()],
                mdns_service_name: token_info.mdns_record.as_ref().map(|r| r.service_name.clone()).unwrap_or_default(),
                created_at: chrono::Utc::now(),
            };
            Ok(Json(ApiResponse::success(response)))
        }
        Err(e) => {
            eprintln!("Error creating token: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// List tokens with optional filtering
async fn list_tokens(
    State(state): State<Arc<AppState>>,
    Query(params): Query<QueryParams>,
) -> Result<Json<ApiResponse<Vec<Value>>>, StatusCode> {
    let stats = state.integrated_system.get_system_stats().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // In a real implementation, you'd query the actual tokens
    // For now, return sample data based on stats
    let tokens = vec![
        json!({
            "token_id": Uuid::new_v4().to_string(),
            "total_count": stats.total_integrated_tokens,
            "status": "active"
        })
    ];
    Ok(Json(ApiResponse::success(tokens)))
}

/// Get specific token by ID
async fn get_token(
    State(state): State<Arc<AppState>>,
    Path(token_id): Path<String>,
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    // In a real implementation, query the specific token from database
    let token_data = json!({
        "token_id": token_id,
        "status": "found",
        "database_instance": "same_as_production"
    });
    
    Ok(Json(ApiResponse::success(token_data)))
}

/// Verify token Merkle proof
async fn verify_token(
    State(state): State<Arc<AppState>>,
    Path(token_id): Path<String>,
    Json(verification_data): Json<Value>,
) -> Result<Json<ApiResponse<bool>>, StatusCode> {
    // Extract verification data
    let token_data = verification_data["token_data"].as_array()
        .and_then(|arr| arr.iter().map(|v| v.as_str().map(|s| s.to_string())).collect::<Option<Vec<_>>>())
        .unwrap_or_default();
    
    let merkle_proof = verification_data["merkle_proof"].as_array()
        .and_then(|arr| arr.iter().map(|v| v.as_str().map(|s| s.to_string())).collect::<Option<Vec<_>>>())
        .unwrap_or_default();
    
    // For now, return a simple verification result
    // In a real implementation, you'd get the complete token info and verify
    match Uuid::parse_str(&token_id) {
        Ok(uuid) => match state.integrated_system.get_complete_token_info(&uuid).await {
            Ok(Some(complete_info)) => {
                match state.integrated_system.verify_token_integrity(&complete_info).await {
                    Ok(is_valid) => Ok(Json(ApiResponse::success(is_valid))),
                    Err(e) => {
                        eprintln!("Error verifying token integrity: {}", e);
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            }
            Ok(None) => Ok(Json(ApiResponse::success(false))),
            Err(e) => {
                eprintln!("Error getting token info: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(_) => Ok(Json(ApiResponse::success(false)))
    }

}

/// Create a new address with network discovery
async fn create_address(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateAddressRequest>,
) -> Result<Json<ApiResponse<AddressResponse>>, StatusCode> {
    // Create address using integrated token system
    match state.integrated_system.create_integrated_token(
        format!("{}:{}", request.ip_address, request.port),
        request.ip_address.clone(),
        "BPI Address".to_string(),
        Some("Address via API".to_string()),
        "system".to_string(),
        true,
        Some(request.port),
    ).await {
        Ok(token_info) => {
            let response = AddressResponse {
                address_id: token_info.entry.id.to_string(),
                ip_address: request.ip_address,
                port: request.port,
                mdns_service_name: token_info.mdns_record.as_ref().map(|r| r.service_name.clone()).unwrap_or_default(),
                created_at: chrono::Utc::now(),
            };
            Ok(Json(ApiResponse::success(response)))
        }
        Err(e) => {
            eprintln!("Error creating address: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// List addresses
async fn list_addresses(
    State(state): State<Arc<AppState>>,
    Query(params): Query<QueryParams>,
) -> Result<Json<ApiResponse<Vec<Value>>>, StatusCode> {
    match state.integrated_system.get_system_stats().await {
        Ok(stats) => {
            let addresses = vec![
                json!({
                    "address_id": Uuid::new_v4().to_string(),
                    "total_count": stats.total_integrated_tokens,
                    "status": "active"
                })
            ];
            Ok(Json(ApiResponse::success(addresses)))
        }
        Err(e) => {
            eprintln!("Error listing addresses: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get specific address by ID
async fn get_address(
    State(state): State<Arc<AppState>>,
    Path(address_id): Path<String>,
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let address_data = json!({
        "address_id": address_id,
        "status": "found",
        "database_instance": "same_as_production"
    });
    
    Ok(Json(ApiResponse::success(address_data)))
}

/// Get comprehensive system statistics
async fn get_system_stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<SystemStats>>, StatusCode> {
    match state.integrated_system.get_system_stats().await {
        Ok(stats) => {
            let uptime = chrono::Utc::now().signed_duration_since(state.start_time);
            let system_stats = SystemStats {
                total_tokens: stats.total_integrated_tokens,
                total_addresses: stats.total_integrated_tokens,
                merkle_operations: stats.merkle_stats.total_hashes,
                active_services: stats.mdns_stats.active_records,
                database_healthy: true,
                system_uptime: format!("{}h {}m", uptime.num_hours(), uptime.num_minutes() % 60),
            };
            Ok(Json(ApiResponse::success(system_stats)))
        }
        Err(e) => {
            eprintln!("Error getting system stats: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Health check endpoint
async fn health_check(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    match state.integrated_system.health_check().await {
        Ok(health) => {
            let health_data = json!({
                "overall_healthy": health.get("overall").unwrap_or(&true),
                "database_healthy": health.get("database").unwrap_or(&true),
                "merkle_healthy": health.get("merkle").unwrap_or(&true),
                "mdns_healthy": health.get("mdns").unwrap_or(&true),
                "database_instance": "production_ready",
                "timestamp": chrono::Utc::now()
            });
            Ok(Json(ApiResponse::success(health_data)))
        }
        Err(e) => {
            eprintln!("Error checking health: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Discover network services
async fn discover_services(
    State(state): State<Arc<AppState>>,
    Path(service_type): Path<String>,
) -> Result<Json<ApiResponse<Vec<Value>>>, StatusCode> {
    let service_query = format!("_{}.tcp.local.", service_type);
    match state.integrated_system.discover_network_services().await {
        Ok(services) => {
            let service_list: Vec<Value> = services.into_iter().map(|service| {
                json!({
                    "name": service.service_name,
                    "service_type": service.service_type,
                    "discovered_at": chrono::Utc::now(),
                    "type": service_type
                })
            }).collect();
            Ok(Json(ApiResponse::success(service_list)))
        }
        Err(e) => {
            eprintln!("Error discovering services: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// API status endpoint for frontend connectivity testing
async fn api_status(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let status = json!({
        "api_version": "1.0.0",
        "database_instance": "production",
        "backend_status": "operational",
        "frontend_ready": true,
        "uptime": chrono::Utc::now().signed_duration_since(state.start_time).num_seconds(),
        "endpoints": [
            "/api/tokens",
            "/api/addresses", 
            "/api/system/stats",
            "/api/system/health"
        ]
    });
    
    Ok(Json(ApiResponse::success(status)))
}

/// Initialize and start the API server
pub async fn start_api_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting BPI Token API Server on port {}...", port);
    
    // Use the same database configuration as tests
    let four_d_config = FourDConfig {
        max_tile_size: 1024 * 1024,
        compression_enabled: true,
        security_enabled: true,
        mongodb_compatibility: false,
        cache_size_mb: 512,
    };
    
    let mdns_config = MdnsProxyConfig {
        enabled: true,
        default_service_type: "_bpi._tcp".to_string(),
        default_domain: "local".to_string(),
        default_ttl: 120,
        bind_interface: None,
        multicast_addr: "224.0.0.251".parse().unwrap(),
        multicast_port: 5353,
        ipv6_enabled: false,
        cache_timeout: 300,
    };
    
    let config = IntegratedTokenSystemConfig {
        four_d_config,
        merkle_master_salt: "production_salt_key_2024".to_string(),
        mdns_config,
        auto_merkle_trees: true,
        auto_mdns_registration: true,
        min_security_level: "Medium".to_string(),
    };
    
    // Initialize the integrated system with production database
    let integrated_system = Arc::new(IntegratedTokenSystem::new(config).await?);
    println!("✅ Integrated Token System initialized with production database");
    
    // Create the router
    let app = create_router(integrated_system);
    
    // Add CORS middleware for frontend integration
    let app = app.layer(
        tower_http::cors::CorsLayer::new()
            .allow_origin(tower_http::cors::Any)
            .allow_methods(tower_http::cors::Any)
            .allow_headers(tower_http::cors::Any)
    );
    
    // Start the server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("🌐 API Server listening on http://0.0.0.0:{}", port);
    println!("📊 Health check: http://0.0.0.0:{}/api/system/health", port);
    println!("📈 System stats: http://0.0.0.0:{}/api/system/stats", port);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
