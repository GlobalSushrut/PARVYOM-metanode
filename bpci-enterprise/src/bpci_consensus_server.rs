//! BPCI Consensus Server Integration
//! 
//! Integrates the Triple Consensus Coordinator with the BPCI server
//! for testnet deployment. Handles HTTP API endpoints, WebSocket
//! connections, and real-time consensus monitoring.

use anyhow::{anyhow, Result};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
// WebSocket functionality temporarily disabled for compilation
// use axum::extract::ws::WebSocketUpgrade;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::{info, warn, error};

use crate::triple_consensus_coordinator::{
    TripleConsensusCoordinator, ConsensusRound, ConsensusRoundStatus, 
    BundleProposal, TripleConsensusMetrics
};
use crate::auction_mode_manager::{AuctionModeManager, AuctionMode};
use crate::bpi_ledger_integration::BpiLedgerClient;

/// BPCI Consensus Server state
#[derive(Clone)]
pub struct BpciConsensusServerState {
    pub consensus_coordinator: Arc<TripleConsensusCoordinator>,
    pub auction_manager: Arc<AuctionModeManager>,
    pub bpi_ledger_client: Arc<BpiLedgerClient>,
    pub server_config: BpciServerConfig,
}

/// BPCI Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpciServerConfig {
    pub server_mode: ServerMode,
    pub listen_address: String,
    pub listen_port: u16,
    pub max_concurrent_rounds: usize,
    pub round_timeout_seconds: u64,
    pub enable_websocket_monitoring: bool,
    pub enable_metrics_endpoint: bool,
}

/// Server deployment mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMode {
    /// Testnet mode - centralized BPCI server
    Testnet {
        mock_validators: u32,
        simulate_network_delays: bool,
    },
    /// Development mode - local testing
    Development {
        auto_generate_bundles: bool,
        debug_logging: bool,
    },
}

/// API request/response types
#[derive(Debug, Serialize, Deserialize)]
pub struct StartConsensusRequest {
    pub bundle_proposals: Vec<BundleProposalRequest>,
    pub priority_mode: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BundleProposalRequest {
    pub proposer_id: String,
    pub transaction_count: u32,
    pub total_fees: u64,
    pub gas_limit: u64,
    pub bid_amount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsensusStatusResponse {
    pub round_id: String,
    pub status: ConsensusRoundStatus,
    pub current_phase: String,
    pub progress_percentage: f64,
    pub estimated_completion_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsensusMetricsResponse {
    pub metrics: TripleConsensusMetrics,
    pub active_rounds: u32,
    pub server_uptime_seconds: u64,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuctionModeResponse {
    pub current_mode: AuctionMode,
    pub mode_description: String,
    pub testnet_features_enabled: Vec<String>,
    pub mainnet_readiness: bool,
}

impl BpciConsensusServerState {
    /// Create new BPCI consensus server state
    pub async fn new(config: BpciServerConfig) -> Result<Self> {
        info!("Initializing BPCI Consensus Server in {:?} mode", config.server_mode);
        
        // Initialize BPI ledger client
        let bpi_ledger_client = Arc::new(BpiLedgerClient::new().await?);
        
        // Initialize auction manager in testnet mode
        let auction_mode = match &config.server_mode {
            ServerMode::Testnet { .. } => AuctionMode::Testnet {
                mock_to_bpi_db: true,
                simulate_community_bidding: true,
            },
            ServerMode::Development { .. } => AuctionMode::Testnet {
                mock_to_bpi_db: true,
                simulate_community_bidding: false,
            },
        };
        
        let auction_manager = Arc::new(AuctionModeManager::new(
            auction_mode,
            bpi_ledger_client.clone(),
        ));
        
        // Initialize triple consensus coordinator
        let consensus_coordinator = Arc::new(TripleConsensusCoordinator::new(
            auction_manager.clone(),
            bpi_ledger_client.clone(),
        ));
        
        Ok(Self {
            consensus_coordinator,
            auction_manager,
            bpi_ledger_client,
            server_config: config,
        })
    }
}

/// Create BPCI Consensus Server router
pub fn create_bpci_consensus_router(state: BpciConsensusServerState) -> Router {
    Router::new()
        // Core consensus endpoints
        .route("/api/v1/consensus/start", post(start_consensus_round))
        .route("/api/v1/consensus/status/:round_id", get(get_consensus_status))
        .route("/api/v1/consensus/rounds", get(list_active_rounds))
        
        // Auction management endpoints
        .route("/api/v1/auction/mode", get(get_auction_mode))
        .route("/api/v1/auction/mode", post(set_auction_mode))
        .route("/api/v1/auction/history", get(get_auction_history))
        
        // Metrics and monitoring
        .route("/api/v1/metrics", get(get_consensus_metrics))
        .route("/api/v1/health", get(health_check))
        
        // WebSocket monitoring (if enabled) - temporarily disabled
        // .route("/ws/consensus", get(websocket_consensus_monitor))
        
        // Development/testing endpoints
        .route("/api/v1/dev/generate-bundles", post(generate_test_bundles))
        .route("/api/v1/dev/simulate-round", post(simulate_consensus_round))
        
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Start a new consensus round
async fn start_consensus_round(
    State(state): State<BpciConsensusServerState>,
    Json(request): Json<StartConsensusRequest>,
) -> Result<Json<ConsensusStatusResponse>, StatusCode> {
    info!("Starting new consensus round with {} bundle proposals", request.bundle_proposals.len());
    
    // Convert request bundles to internal format
    let bundle_proposals: Vec<BundleProposal> = request.bundle_proposals
        .into_iter()
        .map(|req| BundleProposal {
            bundle_id: uuid::Uuid::new_v4().to_string(),
            proposer_id: req.proposer_id,
            transaction_count: req.transaction_count,
            total_fees: req.total_fees,
            gas_limit: req.gas_limit,
            priority_score: calculate_priority_score(req.total_fees, req.gas_limit),
            bid_amount: req.bid_amount,
            timestamp: Utc::now(),
        })
        .collect();
    
    // Start consensus round
    match state.consensus_coordinator.start_consensus_round(bundle_proposals).await {
        Ok(round_id) => {
            let status = state.consensus_coordinator.get_round_status(&round_id).await
                .unwrap_or(ConsensusRoundStatus::Failed("Unknown status".to_string()));
            
            Ok(Json(ConsensusStatusResponse {
                round_id,
                status: status.clone(),
                current_phase: format!("{:?}", status),
                progress_percentage: 0.0,
                estimated_completion_time: Some(Utc::now() + chrono::Duration::seconds(30)),
            }))
        }
        Err(e) => {
            error!("Failed to start consensus round: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get consensus round status
async fn get_consensus_status(
    State(state): State<BpciConsensusServerState>,
    Path(round_id): Path<String>,
) -> Result<Json<ConsensusStatusResponse>, StatusCode> {
    match state.consensus_coordinator.get_round_status(&round_id).await {
        Ok(status) => {
            let progress = calculate_progress_percentage(&status);
            
            Ok(Json(ConsensusStatusResponse {
                round_id,
                status: status.clone(),
                current_phase: format!("{:?}", status),
                progress_percentage: progress,
                estimated_completion_time: estimate_completion_time(&status),
            }))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// List all active consensus rounds
async fn list_active_rounds(
    State(state): State<BpciConsensusServerState>,
) -> Json<Vec<ConsensusStatusResponse>> {
    // This would return all active rounds in a real implementation
    // For now, return empty list as a placeholder
    Json(vec![])
}

/// Get current auction mode
async fn get_auction_mode(
    State(state): State<BpciConsensusServerState>,
) -> Json<AuctionModeResponse> {
    let current_mode = state.auction_manager.get_current_mode().await;
    
    let (mode_description, testnet_features, mainnet_ready) = match &current_mode {
        AuctionMode::Testnet { mock_to_bpi_db, simulate_community_bidding } => {
            let mut features = vec!["Mock auction settlement".to_string()];
            if *mock_to_bpi_db {
                features.push("BPI DB integration".to_string());
            }
            if *simulate_community_bidding {
                features.push("Community bidding simulation".to_string());
            }
            
            ("Testnet mode - Mock auctions for testing".to_string(), features, false)
        }
        AuctionMode::Mainnet { .. } => {
            ("Mainnet mode - Real community auctions".to_string(), vec![], true)
        }
    };
    
    Json(AuctionModeResponse {
        current_mode,
        mode_description,
        testnet_features_enabled: testnet_features,
        mainnet_readiness: mainnet_ready,
    })
}

/// Set auction mode
async fn set_auction_mode(
    State(state): State<BpciConsensusServerState>,
    Json(mode): Json<AuctionMode>,
) -> Result<Json<AuctionModeResponse>, StatusCode> {
    match state.auction_manager.set_auction_mode(mode).await {
        Ok(_) => {
            info!("Auction mode updated successfully");
            Ok(get_auction_mode(State(state)).await)
        }
        Err(e) => {
            error!("Failed to update auction mode: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

/// Get auction history
async fn get_auction_history(
    State(state): State<BpciConsensusServerState>,
) -> Json<Vec<serde_json::Value>> {
    let history = state.auction_manager.get_settlement_history().await;
    Json(history.into_iter().map(|s| serde_json::to_value(s).unwrap()).collect())
}

/// Get consensus metrics
async fn get_consensus_metrics(
    State(state): State<BpciConsensusServerState>,
) -> Json<ConsensusMetricsResponse> {
    let metrics = state.consensus_coordinator.get_consensus_metrics().await;
    
    Json(ConsensusMetricsResponse {
        metrics,
        active_rounds: 0, // Would be calculated from active rounds
        server_uptime_seconds: 0, // Would track actual uptime
        last_updated: Utc::now(),
    })
}

/// Health check endpoint
async fn health_check(
    State(state): State<BpciConsensusServerState>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "server_mode": state.server_config.server_mode,
        "timestamp": Utc::now(),
        "version": "1.0.0"
    }))
}

/// WebSocket consensus monitoring - temporarily disabled for compilation
async fn websocket_consensus_monitor(
    // ws: WebSocketUpgrade,
    State(state): State<BpciConsensusServerState>,
) -> Response {
    // WebSocket functionality temporarily disabled
    StatusCode::NOT_IMPLEMENTED.into_response()
    
    // ws.on_upgrade(|_socket| async move {
    //     // WebSocket implementation would go here
    //     // This would stream real-time consensus updates
    //     info!("WebSocket consensus monitor connected");
    // })
}

/// Generate test bundles for development
async fn generate_test_bundles(
    State(state): State<BpciConsensusServerState>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<BundleProposalRequest>> {
    let count = params.get("count")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(5);
    
    let mut bundles = Vec::new();
    for i in 0..count {
        bundles.push(BundleProposalRequest {
            proposer_id: format!("test_proposer_{}", i),
            transaction_count: 10 + (i as u32 * 5),
            total_fees: 1000000 + (i as u64 * 100000),
            gas_limit: 21000 * (i as u64 + 1),
            bid_amount: 500000 + (i as u64 * 50000),
        });
    }
    
    info!("Generated {} test bundle proposals", bundles.len());
    Json(bundles)
}

/// Simulate a complete consensus round for testing
async fn simulate_consensus_round(
    State(state): State<BpciConsensusServerState>,
) -> Result<Json<ConsensusStatusResponse>, StatusCode> {
    // Generate test bundles
    let test_bundles = vec![
        BundleProposal {
            bundle_id: uuid::Uuid::new_v4().to_string(),
            proposer_id: "simulator".to_string(),
            transaction_count: 25,
            total_fees: 2500000,
            gas_limit: 525000,
            priority_score: 0.9,
            bid_amount: 1000000,
            timestamp: Utc::now(),
        }
    ];
    
    // Start consensus round
    match state.consensus_coordinator.start_consensus_round(test_bundles).await {
        Ok(round_id) => {
            info!("Simulated consensus round started: {}", round_id);
            
            // Wait a moment for processing
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            
            let status = state.consensus_coordinator.get_round_status(&round_id).await
                .unwrap_or(ConsensusRoundStatus::Failed("Simulation failed".to_string()));
            
            Ok(Json(ConsensusStatusResponse {
                round_id,
                status: status.clone(),
                current_phase: format!("{:?}", status),
                progress_percentage: 100.0,
                estimated_completion_time: Some(Utc::now()),
            }))
        }
        Err(e) => {
            error!("Failed to simulate consensus round: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Helper functions
fn calculate_priority_score(total_fees: u64, gas_limit: u64) -> f64 {
    if gas_limit == 0 {
        0.0
    } else {
        (total_fees as f64) / (gas_limit as f64)
    }
}

fn calculate_progress_percentage(status: &ConsensusRoundStatus) -> f64 {
    match status {
        ConsensusRoundStatus::Initializing => 0.0,
        ConsensusRoundStatus::IbftInProgress => 25.0,
        ConsensusRoundStatus::HotStuffOptimizing => 50.0,
        ConsensusRoundStatus::AuctionInProgress => 75.0,
        ConsensusRoundStatus::Finalizing => 90.0,
        ConsensusRoundStatus::Completed => 100.0,
        ConsensusRoundStatus::Failed(_) => 0.0,
    }
}

fn estimate_completion_time(status: &ConsensusRoundStatus) -> Option<DateTime<Utc>> {
    match status {
        ConsensusRoundStatus::Completed | ConsensusRoundStatus::Failed(_) => None,
        _ => Some(Utc::now() + chrono::Duration::seconds(30)),
    }
}

impl Default for BpciServerConfig {
    fn default() -> Self {
        Self {
            server_mode: ServerMode::Testnet {
                mock_validators: 5,
                simulate_network_delays: true,
            },
            listen_address: "127.0.0.1".to_string(),
            listen_port: 8080,
            max_concurrent_rounds: 10,
            round_timeout_seconds: 30,
            enable_websocket_monitoring: true,
            enable_metrics_endpoint: true,
        }
    }
}
