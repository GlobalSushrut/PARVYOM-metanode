use anyhow::Result;
use axum::{
    extract::{State, WebSocketUpgrade},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use clap::Parser;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing::{info, Level};

mod api;
mod core;

use crate::core::{WalletCore, WalletConfig};

#[derive(Parser)]
#[command(name = "pravyom-wallet")]
#[command(about = "Modern, light and powerful wallet for BPI Core infrastructure management")]
struct Cli {
    /// Port to run the wallet server on
    #[arg(short, long, default_value = "3000")]
    port: u16,
    
    /// Configuration file path
    #[arg(short, long, default_value = "wallet-config.yaml")]
    config: String,
    
    /// Enable development mode
    #[arg(long)]
    dev: bool,
}

#[derive(Clone)]
pub struct AppState {
    pub wallet_core: Arc<WalletCore>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let cli = Cli::parse();
    
    info!("🚀 Starting Pravyom Wallet...");

    // Load configuration
    let config = WalletConfig::load_or_default().await?;
    
    // Initialize wallet core
    let wallet_core = Arc::new(WalletCore::new(config).await?);
    
    // Create application state
    let app_state = AppState { wallet_core };

    // Build API router
    let api_router = Router::new()
        .route("/status", get(api::status::get_status))
        .route("/wallet/info", get(api::wallet::get_wallet_info))
        .route("/wallet/balance", get(api::wallet::get_balance))
        .route("/wallet/transactions", get(api::wallet::get_transactions))
        .route("/wallet/send", post(api::wallet::send_transaction))
        .route("/bpi/components", get(api::bpi::get_components_status))
        .route("/bpi/components/:name", get(api::bpi::get_component_details))
        .route("/bpi/components/:name/start", post(api::bpi::start_component))
        .route("/bpi/components/:name/stop", post(api::bpi::stop_component))
        .route("/bpi/components/:name/logs", get(api::bpi::get_component_logs))
        .route("/bpi/metrics", get(api::bpi::get_metrics));

    // Build main router
    let app = Router::new()
        .route("/", get(serve_wallet_ui))
        .route("/ws", get(websocket_handler))
        .nest("/api", api_router)
        .nest_service("/assets", ServeDir::new("ui/assets"))
        .nest_service("/static", ServeDir::new("ui/dist"))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        )
        .with_state(app_state);

    // Start the server
    let addr = format!("127.0.0.1:{}", cli.port);
    let listener = TcpListener::bind(&addr).await?;
    
    info!("🌟 Pravyom Wallet running at http://{}", addr);
    info!("📊 Dashboard: http://{}", addr);
    info!("🔧 API: http://{}/api/status", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn serve_wallet_ui() -> impl IntoResponse {
    Html(include_str!("../ui/index.html"))
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| api::websocket::handle_websocket(socket, state))
}
