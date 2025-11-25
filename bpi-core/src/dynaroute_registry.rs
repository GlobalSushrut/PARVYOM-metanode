//! DynaRoute Registry Server
//! 
//! Central service discovery and registration system for BPI Core.
//! Provides HTTP API on port 8087 for dynamic service discovery,
//! health checking, and automatic failover.

use axum::{
    Router,
    routing::{get, post},
    extract::{Path, State},
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use tracing::{info, warn, error, debug};
use anyhow::Result;

/// DynaRoute Registry Server
pub struct DynaRouteRegistry {
    /// Registered services
    services: Arc<RwLock<HashMap<String, ServiceInfo>>>,
    /// Bind address
    bind_addr: SocketAddr,
}

/// Service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Service name
    pub service_name: String,
    /// Service address (host:port)
    pub address: String,
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
    /// Last heartbeat timestamp
    pub last_heartbeat: DateTime<Utc>,
    /// Health status
    pub health_status: HealthStatus,
    /// Service metadata
    pub metadata: HashMap<String, String>,
}

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
    Unknown,
}

/// Service registration request
#[derive(Debug, Deserialize)]
pub struct RegisterServiceRequest {
    pub service_name: String,
    pub address: String,
    pub metadata: Option<HashMap<String, String>>,
}

/// Heartbeat request
#[derive(Debug, Deserialize)]
pub struct HeartbeatRequest {
    pub service_name: String,
}

impl DynaRouteRegistry {
    /// Create new DynaRoute registry
    pub fn new(bind_addr: &str) -> Result<Self> {
        let addr: SocketAddr = bind_addr.parse()?;
        
        Ok(Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            bind_addr: addr,
        })
    }
    
    /// Start the registry server
    pub async fn start(self) -> Result<()> {
        let services = self.services.clone();
        
        // Build router
        let app = Router::new()
            .route("/services", get(list_services))
            .route("/services/:name", get(get_service))
            .route("/services", post(register_service))
            .route("/services/:name/heartbeat", post(heartbeat))
            .route("/services/:name", axum::routing::delete(deregister_service))
            .route("/health", get(health_check))
            .with_state(services);
        
        info!("🔍 DynaRoute Registry starting on {}", self.bind_addr);
        
        // Start server
        let listener = tokio::net::TcpListener::bind(self.bind_addr).await?;
        
        info!("✅ DynaRoute Registry listening on {}", self.bind_addr);
        
        axum::serve(listener, app).await?;
        
        Ok(())
    }
}

/// List all services
async fn list_services(
    State(services): State<Arc<RwLock<HashMap<String, ServiceInfo>>>>
) -> Json<Vec<ServiceInfo>> {
    let services = services.read().await;
    let service_list: Vec<ServiceInfo> = services.values().cloned().collect();
    
    debug!("📋 Listing {} services", service_list.len());
    
    Json(service_list)
}

/// Get specific service
async fn get_service(
    Path(name): Path<String>,
    State(services): State<Arc<RwLock<HashMap<String, ServiceInfo>>>>
) -> Result<Json<ServiceInfo>, StatusCode> {
    let services = services.read().await;
    
    match services.get(&name) {
        Some(service) => {
            debug!("✅ Found service: {}", name);
            Ok(Json(service.clone()))
        }
        None => {
            warn!("❌ Service not found: {}", name);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Register a service
async fn register_service(
    State(services): State<Arc<RwLock<HashMap<String, ServiceInfo>>>>,
    Json(request): Json<RegisterServiceRequest>
) -> StatusCode {
    let mut services = services.write().await;
    
    let service_info = ServiceInfo {
        service_name: request.service_name.clone(),
        address: request.address.clone(),
        registered_at: Utc::now(),
        last_heartbeat: Utc::now(),
        health_status: HealthStatus::Healthy,
        metadata: request.metadata.unwrap_or_default(),
    };
    
    services.insert(request.service_name.clone(), service_info);
    
    info!("✅ Registered service: {} at {}", request.service_name, request.address);
    
    StatusCode::CREATED
}

/// Update service heartbeat
async fn heartbeat(
    Path(name): Path<String>,
    State(services): State<Arc<RwLock<HashMap<String, ServiceInfo>>>>
) -> StatusCode {
    let mut services = services.write().await;
    
    match services.get_mut(&name) {
        Some(service) => {
            service.last_heartbeat = Utc::now();
            service.health_status = HealthStatus::Healthy;
            
            debug!("💓 Heartbeat received from: {}", name);
            
            StatusCode::OK
        }
        None => {
            warn!("❌ Heartbeat for unknown service: {}", name);
            StatusCode::NOT_FOUND
        }
    }
}

/// Deregister a service
async fn deregister_service(
    Path(name): Path<String>,
    State(services): State<Arc<RwLock<HashMap<String, ServiceInfo>>>>
) -> StatusCode {
    let mut services = services.write().await;
    
    match services.remove(&name) {
        Some(_) => {
            info!("✅ Deregistered service: {}", name);
            StatusCode::OK
        }
        None => {
            warn!("❌ Cannot deregister unknown service: {}", name);
            StatusCode::NOT_FOUND
        }
    }
}

/// Health check endpoint
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "dynaroute-registry",
        "timestamp": Utc::now().to_rfc3339()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_registry_creation() {
        let registry = DynaRouteRegistry::new("127.0.0.1:8087");
        assert!(registry.is_ok());
    }
    
    #[tokio::test]
    async fn test_service_registration() {
        let services = Arc::new(RwLock::new(HashMap::new()));
        
        let request = RegisterServiceRequest {
            service_name: "test-service".to_string(),
            address: "localhost:9000".to_string(),
            metadata: None,
        };
        
        let status = register_service(
            State(services.clone()),
            Json(request)
        ).await;
        
        assert_eq!(status, StatusCode::CREATED);
        
        let services_map = services.read().await;
        assert!(services_map.contains_key("test-service"));
    }
}
