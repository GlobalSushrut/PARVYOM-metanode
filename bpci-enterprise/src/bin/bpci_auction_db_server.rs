use std::sync::Arc;

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use warp::Filter;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AuctionRecordRequest {
    tx_id: String,
    from_bpi: String,
    to_bpci: String,
    amount: u64,
    record_type: String,
}

#[derive(Debug, Clone, Serialize)]
struct AuctionRecordResponse {
    result: String,
    status: String,
    tx_id: String,
    timestamp: DateTime<Utc>,
}

#[derive(Debug, Default, Clone)]
struct AuctionDbState {
    records: Vec<AuctionRecordRequest>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Simple, laptop-friendly Auction DB server for the constellation demo.
    // Exposes:
    // - GET  /api/v1/health
    // - POST /api/v1/auction/record

    let api_port: u16 = std::env::var("BPCI_AUCTION_DB_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(7002);

    let bind_host = std::env::var("NETWORK_BINDING").unwrap_or_else(|_| "0.0.0.0".to_string());

    let state = Arc::new(RwLock::new(AuctionDbState::default()));

    let health_state = state.clone();
    let health_route = warp::path!("api" / "v1" / "health")
        .and(warp::get())
        .and_then(move || {
            let state = health_state.clone();
            async move {
                let guard = state.read().await;
                let body = serde_json::json!({
                    "service": "bpci-auction-db",
                    "status": "healthy",
                    "records": guard.records.len(),
                    "timestamp": Utc::now(),
                });
                Result::<_, std::convert::Infallible>::Ok(warp::reply::json(&body))
            }
        });

    let db_state = state.clone();
    let record_route = warp::path!("api" / "v1" / "auction" / "record")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |req: AuctionRecordRequest| {
            let state = db_state.clone();
            async move {
                let mut guard = state.write().await;
                guard.records.push(req.clone());

                let resp = AuctionRecordResponse {
                    result: "recorded".to_string(),
                    status: "ok".to_string(),
                    tx_id: req.tx_id,
                    timestamp: Utc::now(),
                };

                Result::<_, std::convert::Infallible>::Ok(warp::reply::json(&resp))
            }
        });

    let routes = health_route.or(record_route).with(
        warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type"])
            .allow_methods(vec!["GET", "POST"]),
    );

    let bind_addr: std::net::SocketAddr = format!("{}:{}", bind_host, api_port)
        .parse()
        .expect("Invalid bind address");

    println!("🏦 BPCI Auction DB Server starting on {}", bind_addr);
    warp::serve(routes).run(bind_addr).await;

    Ok(())
}
