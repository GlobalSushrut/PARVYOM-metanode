use anyhow::Result;
use bpi_mempool::{EncryptedMempool, MempoolConfig, Transaction, TxId};
use chrono::Utc;
use clap::{Arg, Command};
use serde_json;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, error, Level};
use tracing_subscriber;
use warp::Filter;
use warp::reply::json;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let matches = Command::new("mempool")
        .version("0.1.0")
        .about("BPI Encrypted Mempool Service")
        .arg(
            Arg::new("max-pending")
                .long("max-pending")
                .value_name("COUNT")
                .help("Maximum pending transactions")
                .default_value("10000"),
        )
        .arg(
            Arg::new("reveal-timeout")
                .long("reveal-timeout")
                .value_name("MS")
                .help("Reveal timeout in milliseconds")
                .default_value("30000"),
        )
        .arg(
            Arg::new("dos-limit")
                .long("dos-limit")
                .value_name("COUNT")
                .help("DoS protection request limit per window")
                .default_value("100"),
        )
        .arg(
            Arg::new("batch-size")
                .long("batch-size")
                .value_name("SIZE")
                .help("Batch decryption size")
                .default_value("100"),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .value_name("PORT")
                .help("HTTP server port")
                .default_value("22001"),
        )
        .arg(
            Arg::new("daemon")
                .long("daemon")
                .help("Run as daemon service")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Parse CLI arguments
    let max_pending: usize = matches.get_one::<String>("max-pending").unwrap().parse()?;
    let reveal_timeout: u64 = matches.get_one::<String>("reveal-timeout").unwrap().parse()?;
    let dos_limit: u32 = matches.get_one::<String>("dos-limit").unwrap().parse()?;
    let batch_size: usize = matches.get_one::<String>("batch-size").unwrap().parse()?;
    let port: u16 = matches.get_one::<String>("port").unwrap().parse()?;
    let daemon_mode = matches.get_flag("daemon");

    let config = MempoolConfig {
        max_pending_txs: max_pending,
        reveal_timeout_ms: reveal_timeout,
        dos_max_requests_per_window: dos_limit,
        decrypt_batch_size: batch_size,
        epoch_duration_ms: 300000, // 5 minutes
        max_recovery_attempts: 3,
        stuck_tx_timeout_ms: 600000, // 10 minutes
    };

    info!("Starting BPI Encrypted Mempool Service");
    info!("Config: max_pending={}, port={}, daemon={}", max_pending, port, daemon_mode);

    let mempool = Arc::new(EncryptedMempool::new(config));

    if daemon_mode {
        // Run as HTTP daemon service
        start_http_server(mempool, port).await?;
    } else {
        // Run demo mode
        run_demo_mode(mempool).await?;
    }

    Ok(())
}

async fn start_http_server(mempool: Arc<EncryptedMempool>, port: u16) -> Result<()> {
    info!("Starting HTTP server on port {}", port);

    let mempool_filter = warp::any().map(move || mempool.clone());

    // GET /health - Health check
    let health = warp::path("health")
        .and(warp::get())
        .map(|| {
            json(&serde_json::json!({
                "status": "healthy",
                "service": "encrypted-mempool",
                "timestamp": Utc::now()
            }))
        });

    // GET /stats - Mempool statistics
    let stats = warp::path("stats")
        .and(warp::get())
        .and(mempool_filter.clone())
        .and_then(|mempool: Arc<EncryptedMempool>| async move {
            let stats = mempool.get_mempool_stats().await;
            Ok::<_, warp::Rejection>(json(&stats))
        });

    // GET /pending - Get pending transactions
    let pending = warp::path("pending")
        .and(warp::get())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and(mempool_filter.clone())
        .and_then(|query: std::collections::HashMap<String, String>, mempool: Arc<EncryptedMempool>| async move {
            let limit = query.get("limit").and_then(|s| s.parse().ok()).unwrap_or(10);
            let pending_txs = mempool.get_pending_transactions(limit).await;
            Ok::<_, warp::Rejection>(json(&pending_txs))
        });

    // POST /submit - Submit encrypted transaction
    let submit = warp::path("submit")
        .and(warp::post())
        .and(warp::body::json())
        .and(mempool_filter.clone())
        .and_then(|tx: Transaction, mempool: Arc<EncryptedMempool>| async move {
            match mempool.encrypt_transaction_for_leader(&tx).await {
                Ok(encrypted_tx) => {
                    info!("Encrypted transaction {:?} for leader", tx.id.0);
                    Ok::<_, warp::Rejection>(json(&serde_json::json!({
                        "status": "accepted",
                        "tx_id": encrypted_tx.tx_id,
                        "encrypted": true
                    })))
                }
                Err(e) => {
                    error!("Failed to encrypt transaction: {}", e);
                    Ok(json(&serde_json::json!({
                        "status": "error",
                        "message": format!("Encryption failed: {}", e)
                    })))
                }
            }
        });

    // POST /batch_decrypt - Batch decrypt transactions
    let batch_decrypt = warp::path("batch_decrypt")
        .and(warp::post())
        .and(warp::body::json())
        .and(mempool_filter.clone())
        .and_then(|req: serde_json::Value, mempool: Arc<EncryptedMempool>| async move {
            let batch_size = req.get("batch_size").and_then(|v| v.as_u64()).unwrap_or(100) as usize;
            match mempool.batch_decrypt_transactions(batch_size).await {
                Ok(decrypted_txs) => {
                    info!("Batch decrypted {} transactions", decrypted_txs.len());
                    Ok::<_, warp::Rejection>(json(&serde_json::json!({
                        "status": "success",
                        "decrypted_count": decrypted_txs.len(),
                        "transactions": decrypted_txs
                    })))
                }
                Err(e) => {
                    error!("Batch decryption failed: {}", e);
                    Ok(json(&serde_json::json!({
                        "status": "error",
                        "message": format!("Batch decryption failed: {}", e)
                    })))
                }
            }
        });

    let routes = health
        .or(stats)
        .or(pending)
        .or(submit)
        .or(batch_decrypt)
        .with(warp::cors().allow_any_origin().allow_headers(vec!["content-type"]).allow_methods(vec!["GET", "POST"]));

    info!("✅ Encrypted Mempool HTTP Server running on http://127.0.0.1:{}", port);
    info!("Available endpoints:");
    info!("  GET  /health - Health check");
    info!("  GET  /stats - Mempool statistics");
    info!("  GET  /pending?limit=N - Get pending transactions");
    info!("  POST /submit - Submit transaction");
    info!("  POST /batch_decrypt - Batch decrypt transactions");

    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;

    Ok(())
}

async fn run_demo_mode(mempool: Arc<EncryptedMempool>) -> Result<()> {
    info!("Running encrypted mempool demo...");

    // Create test transactions
    for i in 0..5 {
        let tx = Transaction {
            id: TxId::random(),
            sender: format!("sender{}", i).into_bytes(),
            recipient: format!("recipient{}", i).into_bytes(),
            amount: 1000 + i * 100,
            fee: 10 + i,
            nonce: i,
            timestamp: Utc::now(),
        };

        // Encrypt transaction for leader
        match mempool.encrypt_transaction_for_leader(&tx).await {
            Ok(encrypted_tx) => {
                info!("Encrypted transaction {} for leader", i);
                
                // Reveal transaction after short delay
                sleep(Duration::from_millis(100)).await;
                match mempool.reveal_transaction(&encrypted_tx).await {
                    Ok(_) => info!("Revealed transaction {}", i),
                    Err(e) => info!("Failed to reveal transaction {}: {}", i, e),
                }
            }
            Err(e) => info!("Failed to encrypt transaction {}: {}", i, e),
        }
    }

    // Batch decrypt remaining transactions
    let decrypted_txs = mempool.batch_decrypt_transactions(10).await?;
    info!("Batch decrypted {} transactions", decrypted_txs.len());

    // Show mempool stats
    let stats = mempool.get_mempool_stats().await;
    info!("Mempool stats: {:?}", stats);

    // Get pending transactions
    let pending = mempool.get_pending_transactions(10).await;
    info!("Retrieved {} pending transactions", pending.len());

    info!("Encrypted mempool demo completed successfully!");
    Ok(())
}
