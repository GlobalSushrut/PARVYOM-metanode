use axum::{
    extract::{Query, State, Path},
    http::{StatusCode, HeaderMap},
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use pravyom_enterprise::community_installer_os::{CommunityInstallerOS, InstallerConfig, InstallationPhase};
use pravyom_enterprise::testnet_config::{BpciConfig, init_config, get_config};
use pravyom_enterprise::testnet_auction_storage::{TestnetAuctionStorage, TestnetAuctionStats};
use pravyom_enterprise::bpci_auction_mempool::BpciAuctionMempool;
use pravyom_enterprise::bpi_ledger_integration::BpiLedgerClient;
use pravyom_enterprise::bpci_auth_wallet_endpoints::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::{info, error, warn};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use sha2::{Sha256, Digest};
use ed25519_dalek::{SigningKey, VerifyingKey, Signer};
use rand::rngs::OsRng;
use anyhow::{Result, anyhow};
use hex;

/// Enhanced BPCI Web Interface with User Authentication and Wallet Management
/// Provides comprehensive user dashboard, BPI wallet creation, and activation

/// Application state containing all shared resources
#[derive(Clone)]
struct AppState {
    installer: SharedInstaller,
    user_store: SharedUserStore,
    wallet_store: SharedWalletStore,
    session_store: SharedSessionStore,
    bpi_client: Arc<BpiLedgerClient>,
}

type SharedInstaller = Arc<RwLock<CommunityInstallerOS>>;
type SharedUserStore = Arc<RwLock<HashMap<String, User>>>;
type SharedWalletStore = Arc<RwLock<HashMap<String, BpiWallet>>>;
type SharedSessionStore = Arc<RwLock<HashMap<String, UserSession>>>;

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// Authentication and wallet structures are now imported from bpci_auth_wallet_endpoints

#[derive(Debug, Deserialize)]
struct InstallRequest {
    config: Option<InstallerConfig>,
}

#[derive(Debug, Serialize)]
struct StatusResponse {
    phase: String,
    progress_percent: u32,
    current_step: String,
    errors: Vec<String>,
    warnings: Vec<String>,
    system_info: SystemInfoResponse,
}

#[derive(Debug, Serialize)]
struct SystemInfoResponse {
    hostname: String,
    os_version: String,
    cpu_cores: u32,
    total_ram_gb: u32,
    available_storage_gb: u32,
    uptime_seconds: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting BPCI Community Installer OS Web Interface");
    
    // Initialize installer and user management
    let installer = Arc::new(RwLock::new(CommunityInstallerOS::new(None)));
    let user_store = Arc::new(RwLock::new(HashMap::<String, User>::new()));
    let wallet_store = Arc::new(RwLock::new(HashMap::<String, BpiWallet>::new()));
    let session_store = Arc::new(RwLock::new(HashMap::<String, UserSession>::new()));
    
    // Initialize BPI ledger client for wallet operations
    let bpi_client = Arc::new(BpiLedgerClient::new().await?);
    
    info!("🔐 User authentication and wallet management initialized");
    
    // Application state
    let app_state = AppState {
        installer,
        user_store,
        wallet_store,
        session_store,
        bpi_client,
    };
    
    // Build router with authentication and wallet management
    let app = Router::new()
        // Main interface
        .route("/", get(serve_index))
        .route("/login", get(serve_login_page))
        .route("/register", get(serve_register_page))
        .route("/dashboard", get(serve_dashboard))
        
        // Authentication API
        .route("/api/auth/register", post(register_user))
        .route("/api/auth/login", post(login_user))
        .route("/api/auth/logout", post(logout_user))
        .route("/api/auth/verify", get(verify_session))
        
        // Wallet Management API
        .route("/api/wallet/create", post(create_wallet))
        .route("/api/wallet/list", get(list_wallets))
        .route("/api/wallet/:wallet_id", get(get_wallet))
        .route("/api/wallet/:wallet_id/activate", post(activate_wallet))
        .route("/api/wallet/:wallet_id/balance", get(get_wallet_balance))
        
        // Original installer API
        .route("/api/status", get(get_status))
        .route("/api/install", post(start_installation))
        .route("/api/config", get(get_config_endpoint))
        .route("/api/config", post(update_config))
        .route("/api/logs", get(get_logs))
        .route("/static/*file", get(serve_static))
        
        .layer(CorsLayer::permissive())
        .with_state(app_state);
    
    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    info!("🌐 Web interface available at: http://localhost:8080");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

/// Extract session from headers
async fn extract_session(headers: &HeaderMap, session_store: &SharedSessionStore) -> Option<UserSession> {
    let auth_header = headers.get("Authorization")?;
    let auth_str = auth_header.to_str().ok()?;
    
    if !auth_str.starts_with("Bearer ") {
        return None;
    }
    
    let session_id = &auth_str[7..]; // Remove "Bearer " prefix
    let sessions = session_store.read().await;
    let session = sessions.get(session_id)?.clone();
    
    // Check if session is expired
    if session.expires_at < Utc::now() || !session.is_active {
        return None;
    }
    
    Some(session)
}

/// Register new user
async fn register_user(State(state): State<AppState>, Json(req): Json<RegisterRequest>) -> Result<Json<ApiResponse<String>>, StatusCode> {
    info!("🔐 User registration attempt for email: {}", req.email);
    
    // Validate input
    if req.password != req.confirm_password {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Passwords do not match".to_string()),
        }));
    }
    
    if req.password.len() < 8 {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Password must be at least 8 characters".to_string()),
        }));
    }
    
    let mut users = state.user_store.write().await;
    
    // Check if user already exists
    if users.values().any(|u| u.email == req.email) {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Email already registered".to_string()),
        }));
    }
    
    // Create new user
    let user_id = Uuid::new_v4().to_string();
    let password_hash = hash_password(&req.password);
    
    let user = User {
        user_id: user_id.clone(),
        email: req.email.clone(),
        password_hash,
        created_at: Utc::now(),
        last_login: None,
        is_active: true,
        wallet_ids: Vec::new(),
    };
    
    users.insert(user_id.clone(), user);
    
    info!("✅ User registered successfully: {}", req.email);
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(user_id),
        error: None,
    }))
}

/// Login user
async fn login_user(State(state): State<AppState>, Json(req): Json<LoginRequest>) -> Result<Json<ApiResponse<String>>, StatusCode> {
    info!("🔐 Login attempt for email: {}", req.email);
    
    let mut users = state.user_store.write().await;
    let mut sessions = state.session_store.write().await;
    
    // Find user by email
    let user = users.values_mut().find(|u| u.email == req.email && u.is_active);
    
    if let Some(user) = user {
        if verify_password(&req.password, &user.password_hash) {
            // Update last login
            user.last_login = Some(Utc::now());
            
            // Create session
            let session_id = generate_session_token();
            let session = UserSession {
                session_id: session_id.clone(),
                user_id: user.user_id.clone(),
                created_at: Utc::now(),
                expires_at: Utc::now() + Duration::hours(24), // 24 hour session
                is_active: true,
            };
            
            sessions.insert(session_id.clone(), session);
            
            info!("✅ User logged in successfully: {}", req.email);
            
            return Ok(Json(ApiResponse {
                success: true,
                data: Some(session_id),
                error: None,
            }));
        }
    }
    
    warn!("❌ Login failed for email: {}", req.email);
    
    Ok(Json(ApiResponse {
        success: false,
        data: None,
        error: Some("Invalid email or password".to_string()),
    }))
}

/// Logout user
async fn logout_user(State(state): State<AppState>, headers: HeaderMap) -> Result<Json<ApiResponse<String>>, StatusCode> {
    if let Some(session) = extract_session(&headers, &state.session_store).await {
        let mut sessions = state.session_store.write().await;
        sessions.remove(&session.session_id);
        
        info!("✅ User logged out: {}", session.user_id);
        
        Ok(Json(ApiResponse {
            success: true,
            data: Some("Logged out successfully".to_string()),
            error: None,
        }))
    } else {
        Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("No active session".to_string()),
        }))
    }
}

/// Verify session
async fn verify_session(State(state): State<AppState>, headers: HeaderMap) -> Result<Json<ApiResponse<User>>, StatusCode> {
    if let Some(session) = extract_session(&headers, &state.session_store).await {
        let users = state.user_store.read().await;
        if let Some(user) = users.get(&session.user_id) {
            return Ok(Json(ApiResponse {
                success: true,
                data: Some(user.clone()),
                error: None,
            }));
        }
    }
    
    Ok(Json(ApiResponse {
        success: false,
        data: None,
        error: Some("Invalid or expired session".to_string()),
    }))
}

/// Create new BPI wallet
async fn create_wallet(State(state): State<AppState>, headers: HeaderMap, Json(req): Json<CreateWalletRequest>) -> Result<Json<ApiResponse<BpiWallet>>, StatusCode> {
    let session = match extract_session(&headers, &state.session_store).await {
        Some(s) => s,
        None => return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Authentication required".to_string()),
        }))
    };
    
    info!("💼 Creating wallet '{}' for user: {}", req.wallet_name, session.user_id);
    
    // Generate Ed25519 keypair
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    
    let private_key_hex = hex::encode(signing_key.to_bytes());
    let public_key_hex = hex::encode(verifying_key.to_bytes());
    
    // Create wallet
    let wallet_id = Uuid::new_v4().to_string();
    let bpi_address = generate_bpi_address(&public_key_hex);
    
    let wallet = BpiWallet {
        wallet_id: wallet_id.clone(),
        user_id: session.user_id.clone(),
        wallet_name: req.wallet_name,
        public_key: public_key_hex,
        private_key_encrypted: encrypt_private_key(&private_key_hex, &req.password),
        bpi_address,
        is_activated: false,
        activation_tx_hash: None,
        balance: 0,
        created_at: Utc::now(),
        activated_at: None,
    };
    
    // Store wallet
    let mut wallets = state.wallet_store.write().await;
    wallets.insert(wallet_id.clone(), wallet.clone());
    
    // Update user's wallet list
    let mut users = state.user_store.write().await;
    if let Some(user) = users.get_mut(&session.user_id) {
        user.wallet_ids.push(wallet_id.clone());
    }
    
    info!("✅ Wallet created successfully: {}", wallet_id);
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(wallet),
        error: None,
    }))
}

/// List user's wallets
async fn list_wallets(State(state): State<AppState>, headers: HeaderMap) -> Result<Json<ApiResponse<Vec<BpiWallet>>>, StatusCode> {
    let session = match extract_session(&headers, &state.session_store).await {
        Some(s) => s,
        None => return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Authentication required".to_string()),
        }))
    };
    
    let wallets = state.wallet_store.read().await;
    let user_wallets: Vec<BpiWallet> = wallets.values()
        .filter(|w| w.user_id == session.user_id)
        .cloned()
        .collect();
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(user_wallets),
        error: None,
    }))
}

/// Get specific wallet
async fn get_wallet(State(state): State<AppState>, headers: HeaderMap, Path(wallet_id): Path<String>) -> Result<Json<ApiResponse<BpiWallet>>, StatusCode> {
    let session = match extract_session(&headers, &state.session_store).await {
        Some(s) => s,
        None => return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Authentication required".to_string()),
        }))
    };
    
    let wallets = state.wallet_store.read().await;
    if let Some(wallet) = wallets.get(&wallet_id) {
        if wallet.user_id == session.user_id {
            return Ok(Json(ApiResponse {
                success: true,
                data: Some(wallet.clone()),
                error: None,
            }));
        }
    }
    
    Ok(Json(ApiResponse {
        success: false,
        data: None,
        error: Some("Wallet not found".to_string()),
    }))
}

/// Activate BPI wallet
async fn activate_wallet(State(state): State<AppState>, headers: HeaderMap, Path(wallet_id): Path<String>, Json(req): Json<ActivateWalletRequest>) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let session = match extract_session(&headers, &state.session_store).await {
        Some(s) => s,
        None => return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Authentication required".to_string()),
        }))
    };
    
    info!("🚀 Activating wallet {} for user: {}", wallet_id, session.user_id);
    
    let mut wallets = state.wallet_store.write().await;
    if let Some(wallet) = wallets.get_mut(&wallet_id) {
        if wallet.user_id == session.user_id {
            if wallet.is_activated {
                return Ok(Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Wallet already activated".to_string()),
                }));
            }
            
            // In a real implementation, this would submit a transaction to BPI
            // For testnet, we'll simulate the activation
            let activation_tx_hash = format!("tx_{}", Uuid::new_v4());
            
            wallet.is_activated = true;
            wallet.activation_tx_hash = Some(activation_tx_hash.clone());
            wallet.activated_at = Some(Utc::now());
            wallet.balance = 1000; // Give some initial testnet tokens
            
            info!("✅ Wallet activated successfully: {}", wallet_id);
            
            return Ok(Json(ApiResponse {
                success: true,
                data: Some(activation_tx_hash),
                error: None,
            }));
        }
    }
    
    Ok(Json(ApiResponse {
        success: false,
        data: None,
        error: Some("Wallet not found".to_string()),
    }))
}

/// Get wallet balance
async fn get_wallet_balance(State(state): State<AppState>, headers: HeaderMap, Path(wallet_id): Path<String>) -> Result<Json<ApiResponse<u64>>, StatusCode> {
    let session = match extract_session(&headers, &state.session_store).await {
        Some(s) => s,
        None => return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Authentication required".to_string()),
        }))
    };
    
    let wallets = state.wallet_store.read().await;
    if let Some(wallet) = wallets.get(&wallet_id) {
        if wallet.user_id == session.user_id {
            // In real implementation, query BPI ledger for current balance
            return Ok(Json(ApiResponse {
                success: true,
                data: Some(wallet.balance),
                error: None,
            }));
        }
    }
    
    Ok(Json(ApiResponse {
        success: false,
        data: None,
        error: Some("Wallet not found".to_string()),
    }))
}

/// Serve the main HTML interface with authentication
async fn serve_index() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>BPCI Enterprise - Blockchain Platform for Community Infrastructure</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; }
        .header { background: rgba(255,255,255,0.1); backdrop-filter: blur(10px); padding: 1rem 2rem; display: flex; justify-content: space-between; align-items: center; }
        .logo { color: white; font-size: 1.5rem; font-weight: bold; }
        .nav-buttons { display: flex; gap: 1rem; }
        .btn { padding: 0.5rem 1rem; border: none; border-radius: 5px; cursor: pointer; text-decoration: none; display: inline-block; transition: all 0.3s; }
        .btn-primary { background: #3498db; color: white; }
        .btn-secondary { background: rgba(255,255,255,0.2); color: white; border: 1px solid rgba(255,255,255,0.3); }
        .btn:hover { transform: translateY(-2px); box-shadow: 0 4px 8px rgba(0,0,0,0.2); }
        .hero { text-align: center; padding: 4rem 2rem; color: white; }
        .hero h1 { font-size: 3rem; margin-bottom: 1rem; }
        .hero p { font-size: 1.2rem; margin-bottom: 2rem; opacity: 0.9; }
        .features { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 2rem; padding: 2rem; max-width: 1200px; margin: 0 auto; }
        .feature-card { background: rgba(255,255,255,0.1); backdrop-filter: blur(10px); padding: 2rem; border-radius: 10px; color: white; }
        .feature-card h3 { margin-bottom: 1rem; color: #3498db; }
        .status-panel { background: rgba(255,255,255,0.1); backdrop-filter: blur(10px); margin: 2rem; padding: 2rem; border-radius: 10px; color: white; }
        .progress { width: 100%; height: 20px; background: rgba(255,255,255,0.2); border-radius: 10px; overflow: hidden; margin: 1rem 0; }
        .progress-bar { height: 100%; background: #3498db; transition: width 0.3s; }
        .logs { background: rgba(0,0,0,0.3); color: #ecf0f1; padding: 1rem; border-radius: 5px; font-family: 'Courier New', monospace; height: 200px; overflow-y: auto; margin: 1rem 0; }
    </style>
</head>
<body>
    <div class="header">
        <div class="logo">🚀 BPCI Enterprise</div>
        <div class="nav-buttons">
            <a href="/login" class="btn btn-secondary">Login</a>
            <a href="/register" class="btn btn-primary">Register</a>
        </div>
    </div>
    
    <div class="hero">
        <h1>Blockchain Platform for Community Infrastructure</h1>
        <p>Secure, scalable, and decentralized infrastructure for the next generation of blockchain applications</p>
        <div class="nav-buttons">
            <a href="/login" class="btn btn-primary">Get Started</a>
            <button onclick="checkSystemStatus()" class="btn btn-secondary">System Status</button>
        </div>
    </div>
    
    <div class="features">
        <div class="feature-card">
            <h3>🔐 Secure Authentication</h3>
            <p>Enterprise-grade user authentication with email/password login and session management</p>
        </div>
        <div class="feature-card">
            <h3>💼 BPI Wallet Management</h3>
            <p>Create, manage, and activate BPI wallets with full blockchain integration</p>
        </div>
        <div class="feature-card">
            <h3>🌐 Testnet Ready</h3>
            <p>Fully configured for testnet deployment with comprehensive monitoring and analytics</p>
        </div>
        <div class="feature-card">
            <h3>⚡ Real-time Monitoring</h3>
            <p>Live system status, transaction monitoring, and performance analytics</p>
        </div>
    </div>
    
    <div class="status-panel" id="status-panel" style="display: none;">
        <h3>System Status</h3>
        <div id="status-info">Checking system status...</div>
        <div class="progress">
            <div class="progress-bar" id="progress-bar" style="width: 0%"></div>
        </div>
        <div class="logs" id="logs">System logs will appear here...</div>
        <div style="margin-top: 1rem;">
            <button onclick="startInstallation()" class="btn btn-primary">Start Installation</button>
            <button onclick="getStatus()" class="btn btn-secondary">Refresh Status</button>
            <button onclick="getLogs()" class="btn btn-secondary">View Logs</button>
        </div>
    </div>
    
    <script>
        async function checkSystemStatus() {
            document.getElementById('status-panel').style.display = 'block';
            await getStatus();
        }
        
        async function startInstallation() {
            const response = await fetch('/api/install', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({}) });
            const result = await response.json();
            document.getElementById('logs').innerHTML += '<br>' + (result.success ? '✅ Installation started' : '❌ ' + result.error);
        }
        
        async function getStatus() {
            const response = await fetch('/api/status');
            const result = await response.json();
            if (result.success) {
                document.getElementById('status-info').innerHTML = result.data.current_step;
                document.getElementById('progress-bar').style.width = result.data.progress_percent + '%';
            }
        }
        
        async function getLogs() {
            const response = await fetch('/api/logs');
            const result = await response.json();
            if (result.success) {
                document.getElementById('logs').innerHTML = result.data.join('<br>');
            }
        }
    </script>
</body>
</html>
    "#)
}

/// Serve static files (CSS, JS, images)
async fn serve_static(
    axum::extract::Path(file): axum::extract::Path<String>,
) -> Result<String, StatusCode> {
    // Serve embedded static content
    match file.as_str() {
        "style.css" => Ok(r#"
            body { font-family: Arial, sans-serif; margin: 40px; background: #f5f5f5; }
            .container { max-width: 800px; margin: 0 auto; background: white; padding: 30px; border-radius: 10px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
            h1 { color: #2c3e50; text-align: center; }
            .status { padding: 20px; background: #ecf0f1; border-radius: 5px; margin: 20px 0; }
            .progress { width: 100%; height: 20px; background: #ddd; border-radius: 10px; overflow: hidden; }
            .progress-bar { height: 100%; background: #3498db; transition: width 0.3s; }
            button { background: #3498db; color: white; padding: 10px 20px; border: none; border-radius: 5px; cursor: pointer; margin: 10px 5px; }
            button:hover { background: #2980b9; }
            .logs { background: #2c3e50; color: #ecf0f1; padding: 15px; border-radius: 5px; font-family: monospace; height: 300px; overflow-y: auto; }
        "#.to_string()),
        "app.js" => Ok(r#"
            async function startInstallation() {
                const response = await fetch('/api/install', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({}) });
                const result = await response.json();
                document.getElementById('logs').innerHTML += '<br>' + (result.success ? '✅ Installation started' : '❌ ' + result.error);
            }
            async function getStatus() {
                const response = await fetch('/api/status');
                const result = await response.json();
                if (result.success) {
                    document.getElementById('status-info').innerHTML = result.data.current_step;
                    document.getElementById('progress-bar').style.width = result.data.progress_percent + '%';
                }
            }
            async function getLogs() {
                const response = await fetch('/api/logs');
                const result = await response.json();
                if (result.success) {
                    document.getElementById('logs').innerHTML = result.data.join('<br>');
                }
            }
            setInterval(getStatus, 2000);
        "#.to_string()),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

/// Get current installation status
async fn get_status(State(state): State<AppState>) -> Json<ApiResponse<StatusResponse>> {
    let installer = &state.installer;
    let installer = installer.read().await;
    let status = installer.get_status();
    let system_info = installer.get_system_info();
    
    let response = StatusResponse {
        phase: format!("{:?}", status.phase),
        progress_percent: status.progress_percent,
        current_step: status.current_step.clone(),
        errors: status.errors.clone(),
        warnings: status.warnings.clone(),
        system_info: SystemInfoResponse {
            hostname: system_info.hostname.clone(),
            os_version: system_info.os_version.clone(),
            cpu_cores: system_info.cpu_cores,
            total_ram_gb: system_info.total_ram_gb,
            available_storage_gb: system_info.available_storage_gb,
            uptime_seconds: system_info.uptime_seconds,
        },
    };
    
    Json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    })
}

/// Start the installation process
async fn start_installation(State(state): State<AppState>, Json(req): Json<InstallRequest>) -> Json<ApiResponse<String>> {
    let installer = &state.installer;
    info!("🚀 Starting installation via web interface");
    
    let mut installer = installer.write().await;
    
    // Update config if provided
    if let Some(config) = req.config {
        installer.config = config;
    }
    
    // Start installation in background
    match installer.install().await {
        Ok(_) => {
            info!("✅ Installation completed successfully");
            Json(ApiResponse {
                success: true,
                data: Some("Installation started successfully".to_string()),
                error: None,
            })
        }
        Err(e) => {
            error!("❌ Installation failed: {}", e);
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some(e.to_string()),
            })
        }
    }
}

/// Get current configuration
async fn get_config_endpoint(State(state): State<AppState>) -> Json<ApiResponse<InstallerConfig>> {
    let installer = &state.installer;
    let installer = installer.read().await;
    
    Json(ApiResponse {
        success: true,
        data: Some(installer.config.clone()),
        error: None,
    })
}

/// Serve user dashboard
async fn serve_dashboard() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Dashboard - BPCI Enterprise</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; }
        .header { background: rgba(255,255,255,0.1); backdrop-filter: blur(10px); padding: 1rem 2rem; display: flex; justify-content: space-between; align-items: center; }
        .logo { color: white; font-size: 1.5rem; font-weight: bold; }
        .user-info { color: white; display: flex; align-items: center; gap: 1rem; }
        .btn { padding: 0.5rem 1rem; border: none; border-radius: 5px; cursor: pointer; text-decoration: none; display: inline-block; transition: all 0.3s; }
        .btn-primary { background: #3498db; color: white; }
        .btn-secondary { background: rgba(255,255,255,0.2); color: white; border: 1px solid rgba(255,255,255,0.3); }
        .btn-danger { background: #e74c3c; color: white; }
        .btn:hover { transform: translateY(-2px); box-shadow: 0 4px 8px rgba(0,0,0,0.2); }
        .container { max-width: 1200px; margin: 2rem auto; padding: 0 2rem; }
        .dashboard-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(350px, 1fr)); gap: 2rem; }
        .card { background: rgba(255,255,255,0.1); backdrop-filter: blur(10px); padding: 2rem; border-radius: 15px; color: white; }
        .card h3 { margin-bottom: 1rem; color: #3498db; }
        .wallet-item { background: rgba(255,255,255,0.1); padding: 1rem; border-radius: 10px; margin: 1rem 0; }
        .wallet-header { display: flex; justify-content: between; align-items: center; margin-bottom: 0.5rem; }
        .wallet-name { font-weight: bold; }
        .wallet-status { padding: 0.25rem 0.5rem; border-radius: 15px; font-size: 0.8rem; }
        .status-active { background: #2ecc71; }
        .status-inactive { background: #e74c3c; }
        .wallet-details { font-size: 0.9rem; opacity: 0.8; }
        .form-group { margin-bottom: 1rem; }
        .form-group label { display: block; margin-bottom: 0.5rem; font-weight: 500; }
        .form-group input { width: 100%; padding: 0.75rem; border: 1px solid rgba(255,255,255,0.3); border-radius: 5px; background: rgba(255,255,255,0.1); color: white; }
        .form-group input::placeholder { color: rgba(255,255,255,0.7); }
        .modal { display: none; position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.5); z-index: 1000; }
        .modal-content { background: rgba(255,255,255,0.1); backdrop-filter: blur(10px); margin: 10% auto; padding: 2rem; border-radius: 15px; width: 90%; max-width: 500px; color: white; }
        .close { float: right; font-size: 1.5rem; cursor: pointer; }
        .error { background: rgba(231,76,60,0.2); border: 1px solid #e74c3c; color: white; padding: 0.75rem; border-radius: 5px; margin-bottom: 1rem; display: none; }
        .success { background: rgba(46,204,113,0.2); border: 1px solid #2ecc71; color: white; padding: 0.75rem; border-radius: 5px; margin-bottom: 1rem; display: none; }
        .loading { display: none; text-align: center; padding: 2rem; }
    </style>
</head>
<body>
    <div class="header">
        <div class="logo">🚀 BPCI Enterprise</div>
        <div class="user-info">
            <span id="user-email">Loading...</span>
            <button onclick="logout()" class="btn btn-danger">Logout</button>
        </div>
    </div>
    
    <div class="container">
        <div id="error-message" class="error"></div>
        <div id="success-message" class="success"></div>
        
        <div class="dashboard-grid">
            <!-- User Profile Card -->
            <div class="card">
                <h3>👤 User Profile</h3>
                <div id="user-profile">
                    <div class="loading">Loading profile...</div>
                </div>
            </div>
            
            <!-- BPI Wallets Card -->
            <div class="card">
                <h3>💼 BPI Wallets</h3>
                <button onclick="showCreateWalletModal()" class="btn btn-primary" style="margin-bottom: 1rem;">Create New Wallet</button>
                <div id="wallets-list">
                    <div class="loading">Loading wallets...</div>
                </div>
            </div>
            
            <!-- System Status Card -->
            <div class="card">
                <h3>⚡ System Status</h3>
                <div id="system-status">
                    <div class="loading">Loading system status...</div>
                </div>
            </div>
        </div>
    </div>
    
    <!-- Create Wallet Modal -->
    <div id="create-wallet-modal" class="modal">
        <div class="modal-content">
            <span class="close" onclick="hideCreateWalletModal()">&times;</span>
            <h3>Create New BPI Wallet</h3>
            <form id="create-wallet-form">
                <div class="form-group">
                    <label for="wallet-name">Wallet Name</label>
                    <input type="text" id="wallet-name" placeholder="Enter wallet name" required>
                </div>
                <div class="form-group">
                    <label for="wallet-password">Wallet Password</label>
                    <input type="password" id="wallet-password" placeholder="Enter password for wallet encryption" required>
                </div>
                <button type="submit" class="btn btn-primary">Create Wallet</button>
            </form>
        </div>
    </div>
    
    <!-- Activate Wallet Modal -->
    <div id="activate-wallet-modal" class="modal">
        <div class="modal-content">
            <span class="close" onclick="hideActivateWalletModal()">&times;</span>
            <h3>Activate BPI Wallet</h3>
            <form id="activate-wallet-form">
                <input type="hidden" id="activate-wallet-id">
                <div class="form-group">
                    <label for="activate-password">Wallet Password</label>
                    <input type="password" id="activate-password" placeholder="Enter wallet password" required>
                </div>
                <button type="submit" class="btn btn-primary">Activate Wallet</button>
            </form>
        </div>
    </div>
    
    <script>
        let sessionToken = localStorage.getItem('bpci_session_token');
        
        // Check authentication on page load
        window.onload = async function() {
            if (!sessionToken) {
                window.location.href = '/login';
                return;
            }
            
            await verifySession();
            await loadUserProfile();
            await loadWallets();
            await loadSystemStatus();
        };
        
        async function verifySession() {
            try {
                const response = await fetch('/api/auth/verify', {
                    headers: { 'Authorization': `Bearer ${sessionToken}` }
                });
                const result = await response.json();
                
                if (!result.success) {
                    localStorage.removeItem('bpci_session_token');
                    window.location.href = '/login';
                    return;
                }
                
                document.getElementById('user-email').textContent = result.data.email;
            } catch (error) {
                console.error('Session verification failed:', error);
                window.location.href = '/login';
            }
        }
        
        async function loadUserProfile() {
            try {
                const response = await fetch('/api/auth/verify', {
                    headers: { 'Authorization': `Bearer ${sessionToken}` }
                });
                const result = await response.json();
                
                if (result.success) {
                    const user = result.data;
                    document.getElementById('user-profile').innerHTML = `
                        <p><strong>Email:</strong> ${user.email}</p>
                        <p><strong>User ID:</strong> ${user.user_id}</p>
                        <p><strong>Created:</strong> ${new Date(user.created_at).toLocaleDateString()}</p>
                        <p><strong>Last Login:</strong> ${user.last_login ? new Date(user.last_login).toLocaleDateString() : 'Never'}</p>
                        <p><strong>Wallets:</strong> ${user.wallet_ids.length}</p>
                    `;
                }
            } catch (error) {
                console.error('Failed to load user profile:', error);
            }
        }
        
        async function loadWallets() {
            try {
                const response = await fetch('/api/wallet/list', {
                    headers: { 'Authorization': `Bearer ${sessionToken}` }
                });
                const result = await response.json();
                
                if (result.success) {
                    const wallets = result.data;
                    const walletsHtml = wallets.length > 0 ? wallets.map(wallet => `
                        <div class="wallet-item">
                            <div class="wallet-header">
                                <span class="wallet-name">${wallet.wallet_name}</span>
                                <span class="wallet-status ${wallet.is_activated ? 'status-active' : 'status-inactive'}">
                                    ${wallet.is_activated ? 'Active' : 'Inactive'}
                                </span>
                            </div>
                            <div class="wallet-details">
                                <p><strong>Address:</strong> ${wallet.bpi_address}</p>
                                <p><strong>Balance:</strong> ${wallet.balance} BPI</p>
                                <p><strong>Created:</strong> ${new Date(wallet.created_at).toLocaleDateString()}</p>
                                ${!wallet.is_activated ? `<button onclick="showActivateWalletModal('${wallet.wallet_id}')" class="btn btn-primary" style="margin-top: 0.5rem;">Activate</button>` : ''}
                            </div>
                        </div>
                    `).join('') : '<p>No wallets created yet. Create your first wallet to get started!</p>';
                    
                    document.getElementById('wallets-list').innerHTML = walletsHtml;
                }
            } catch (error) {
                console.error('Failed to load wallets:', error);
            }
        }
        
        async function loadSystemStatus() {
            try {
                const response = await fetch('/api/status');
                const result = await response.json();
                
                if (result.success) {
                    const status = result.data;
                    document.getElementById('system-status').innerHTML = `
                        <p><strong>Status:</strong> ${status.current_step}</p>
                        <p><strong>Progress:</strong> ${status.progress_percent}%</p>
                        <p><strong>System:</strong> ${status.system_info.os} ${status.system_info.arch}</p>
                        <p><strong>Memory:</strong> ${Math.round(status.system_info.memory_gb)}GB</p>
                        <p><strong>CPU Cores:</strong> ${status.system_info.cpu_cores}</p>
                    `;
                }
            } catch (error) {
                console.error('Failed to load system status:', error);
            }
        }
        
        function showCreateWalletModal() {
            document.getElementById('create-wallet-modal').style.display = 'block';
        }
        
        function hideCreateWalletModal() {
            document.getElementById('create-wallet-modal').style.display = 'none';
        }
        
        function showActivateWalletModal(walletId) {
            document.getElementById('activate-wallet-id').value = walletId;
            document.getElementById('activate-wallet-modal').style.display = 'block';
        }
        
        function hideActivateWalletModal() {
            document.getElementById('activate-wallet-modal').style.display = 'none';
        }
        
        document.getElementById('create-wallet-form').addEventListener('submit', async (e) => {
            e.preventDefault();
            
            const walletName = document.getElementById('wallet-name').value;
            const walletPassword = document.getElementById('wallet-password').value;
            
            try {
                const response = await fetch('/api/wallet/create', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                        'Authorization': `Bearer ${sessionToken}`
                    },
                    body: JSON.stringify({
                        wallet_name: walletName,
                        password: walletPassword
                    })
                });
                
                const result = await response.json();
                
                if (result.success) {
                    showSuccess('Wallet created successfully!');
                    hideCreateWalletModal();
                    await loadWallets();
                    await loadUserProfile();
                } else {
                    showError(result.error || 'Failed to create wallet');
                }
            } catch (error) {
                showError('Network error. Please try again.');
            }
        });
        
        document.getElementById('activate-wallet-form').addEventListener('submit', async (e) => {
            e.preventDefault();
            
            const walletId = document.getElementById('activate-wallet-id').value;
            const password = document.getElementById('activate-password').value;
            
            try {
                const response = await fetch(`/api/wallet/${walletId}/activate`, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                        'Authorization': `Bearer ${sessionToken}`
                    },
                    body: JSON.stringify({
                        wallet_id: walletId,
                        password: password
                    })
                });
                
                const result = await response.json();
                
                if (result.success) {
                    showSuccess('Wallet activated successfully!');
                    hideActivateWalletModal();
                    await loadWallets();
                } else {
                    showError(result.error || 'Failed to activate wallet');
                }
            } catch (error) {
                showError('Network error. Please try again.');
            }
        });
        
        async function logout() {
            try {
                await fetch('/api/auth/logout', {
                    method: 'POST',
                    headers: { 'Authorization': `Bearer ${sessionToken}` }
                });
            } catch (error) {
                console.error('Logout error:', error);
            }
            
            localStorage.removeItem('bpci_session_token');
            window.location.href = '/';
        }
        
        function showError(message) {
            const errorDiv = document.getElementById('error-message');
            errorDiv.textContent = message;
            errorDiv.style.display = 'block';
            setTimeout(() => errorDiv.style.display = 'none', 5000);
        }
        
        function showSuccess(message) {
            const successDiv = document.getElementById('success-message');
            successDiv.textContent = message;
            successDiv.style.display = 'block';
            setTimeout(() => successDiv.style.display = 'none', 5000);
        }
    </script>
</body>
</html>
    "#)
}

/// Update configuration
async fn update_config(State(state): State<AppState>, Json(config): Json<InstallerConfig>) -> Json<ApiResponse<String>> {
    let installer = &state.installer;
    let mut installer = installer.write().await;
    installer.config = config;
    
    Json(ApiResponse {
        success: true,
        data: Some("Configuration updated successfully".to_string()),
        error: None,
    })
}

/// Get installation logs
async fn get_logs(State(state): State<AppState>) -> Json<ApiResponse<Vec<String>>> {
    let installer = &state.installer;
    let installer = installer.read().await;
    let logs = installer.get_logs();
    
    Json(ApiResponse {
        success: true,
        data: Some(logs),
        error: None,
    })
}


