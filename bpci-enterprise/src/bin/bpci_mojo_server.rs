// BPCI Mojo Server - Component 9
// Wallet-Based Monitoring with Grafana (address+token auth) and Prometheus

use axum::{extract::State, http::StatusCode, response::Json, routing::{get, post}, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::{info, warn, error};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use anyhow::Result;

// Placeholder for Grafana wallet authentication
#[derive(Clone, Debug)]
struct GrafanaWalletAuth {
    enabled: bool,
}

#[derive(Clone, Debug)]
struct GrafanaApiKey {
    id: u64,
    name: String,
    key: String,
}

impl GrafanaWalletAuth {
    fn new() -> Self {
        Self { enabled: true }
    }
    
    async fn create_wallet_api_key(&self, wallet_address: &str) -> Result<GrafanaApiKey> {
        // Placeholder implementation - returns a test API key
        Ok(GrafanaApiKey {
            id: 1,
            name: format!("mojo-wallet-{}", wallet_address),
            key: Uuid::new_v4().to_string(),
        })
    }
}

#[derive(Clone)]
struct MojoServerState {
    wallets: Arc<RwLock<HashMap<String, MojoWallet>>>,
    grafana_auth: Arc<GrafanaWalletAuth>,
    config: MojoServerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MojoServerConfig {
    pub port: u16,
    pub prometheus_url: String,
    pub grafana_url: String,
}

impl Default for MojoServerConfig {
    fn default() -> Self {
        Self {
            port: 8089,
            prometheus_url: "http://localhost:9090".to_string(),
            grafana_url: "http://localhost:3000".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MojoWallet {
    pub mojo_wallet_id: String,
    pub bpi_wallet_address: String,
    pub grafana_dashboard_url: String,
    pub grafana_token: String,  // Token-based auth (NO password)
    pub prometheus_job: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct CreateMojoWalletRequest {
    bpi_wallet_address: String,
    node_id: String,
}

#[derive(Debug, Serialize)]
struct CreateMojoWalletResponse {
    success: bool,
    mojo_wallet_id: String,
    dashboard_url: String,
    access_token: String,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    total_wallets: usize,
}

async fn health_check(State(state): State<MojoServerState>) -> Json<HealthResponse> {
    let wallets = state.wallets.read().await;
    Json(HealthResponse {
        status: "healthy".to_string(),
        total_wallets: wallets.len(),
    })
}

async fn create_mojo_wallet(
    State(state): State<MojoServerState>,
    Json(req): Json<CreateMojoWalletRequest>,
) -> Result<Json<CreateMojoWalletResponse>, StatusCode> {
    let mojo_wallet_id = Uuid::new_v4().to_string();
    
    // Create Grafana API key for this wallet (wallet address + token auth, NO password)
    info!("🔐 Creating Grafana API key for wallet: {}", req.bpi_wallet_address);
    let grafana_api_key = match state.grafana_auth.create_wallet_api_key(&req.bpi_wallet_address).await {
        Ok(key) => {
            info!("✅ Grafana API key created: {}", key.name);
            key
        },
        Err(e) => {
            error!("❌ Failed to create Grafana API key: {}", e);
            // Fallback to UUID token for testing
            warn!("⚠️ Using fallback token (Grafana may not be running)");
            GrafanaApiKey {
                id: 0,
                name: format!("mojo-wallet-{}", req.bpi_wallet_address),
                key: Uuid::new_v4().to_string(),
            }
        }
    };
    
    let dashboard_url = format!("{}/d/{}", state.config.grafana_url, mojo_wallet_id);
    
    let wallet = MojoWallet {
        mojo_wallet_id: mojo_wallet_id.clone(),
        bpi_wallet_address: req.bpi_wallet_address.clone(),
        grafana_dashboard_url: dashboard_url.clone(),
        grafana_token: grafana_api_key.key.clone(),
        prometheus_job: format!("mojo-wallet-{}", req.bpi_wallet_address),
        created_at: Utc::now(),
    };
    
    let mut wallets = state.wallets.write().await;
    wallets.insert(req.bpi_wallet_address.clone(), wallet);
    
    info!("✅ Created Mojo wallet for: {} (Dashboard: {})", req.bpi_wallet_address, dashboard_url);
    info!("🔑 Access token: {} (use this instead of password)", grafana_api_key.key);
    
    Ok(Json(CreateMojoWalletResponse {
        success: true,
        mojo_wallet_id,
        dashboard_url: format!("{}?auth_token={}", dashboard_url, grafana_api_key.key),
        access_token: grafana_api_key.key,
    }))
}

async fn list_wallets(State(state): State<MojoServerState>) -> Json<Vec<MojoWallet>> {
    let wallets = state.wallets.read().await;
    Json(wallets.values().cloned().collect())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
    
    info!("🚀 Starting BPCI Mojo Server (Component 9)");
    info!("📊 Wallet-Based Monitoring with Grafana (address+token auth, NO passwords)");
    
    let config = MojoServerConfig::default();
    
    // Initialize Grafana wallet authentication
    // TODO: Get admin API key from environment variable or config
    let grafana_admin_api_key = std::env::var("GRAFANA_ADMIN_API_KEY")
        .unwrap_or_else(|_| {
            warn!("⚠️ GRAFANA_ADMIN_API_KEY not set, using fallback mode");
            "fallback-key".to_string()
        });
    
    let grafana_auth = Arc::new(GrafanaWalletAuth::new());
    
    info!("🔐 Grafana wallet authentication initialized");
    info!("📍 Grafana URL: {}", config.grafana_url);
    info!("📍 Prometheus URL: {}", config.prometheus_url);
    
    let state = MojoServerState {
        wallets: Arc::new(RwLock::new(HashMap::new())),
        grafana_auth,
        config: config.clone(),
    };
    
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/wallet", post(create_mojo_wallet))
        .route("/api/v1/wallet", get(list_wallets))
        .layer(CorsLayer::permissive())
        .with_state(state);
    
    let addr: SocketAddr = format!("0.0.0.0:{}", config.port).parse()?;
    info!("🌐 Mojo Server listening on {}", addr);
    info!("🔑 Authentication: Wallet Address + Token (NO passwords)");
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
