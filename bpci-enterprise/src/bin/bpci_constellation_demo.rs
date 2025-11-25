use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::fs::File;
use std::io::Write as IoWrite;
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

#[derive(Debug, Clone)]
struct DemoAccount {
    address: String,
    available_balance: u64,
    gas_fee_percentage: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ConstellationTxRecord {
    index: u64,
    tx_id: String,
    timestamp: DateTime<Utc>,
    amount: u64,
    gas_fee: u64,
    total_cost: u64,
    balance_before: u64,
    balance_after: u64,
    status: String,
    /// Basic consensus health endpoint status
    consensus_status: String,
    /// Whether an LCCD round was successfully started for this tx
    lccd_round_started: bool,
    /// LCCD round identifier (if returned by the server)
    lccd_round_id: Option<String>,
    /// High-level LCCD round status/result
    lccd_round_status: String,
    blockchain_status: String,
    auction_status: String,
    db_status: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("bpci_constellation_demo=info,bpci_enterprise=info")
        .init();

    info!("Starting BPCI constellation 1-minute demo");

    let start_time = Utc::now();

    let consensus_base = std::env::var("BPCI_CONSENSUS_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
    let blockchain_base = std::env::var("BPCI_BLOCKCHAIN_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
    let auction_base = std::env::var("BPCI_AUCTION_MEMPOOL_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:9004".to_string());
    let auction_db_base = std::env::var("BPCI_AUCTION_DB_URL")
        .unwrap_or_else(|_| "http://159.203.101.136:7002".to_string());

    let mut account = DemoAccount {
        address: "bpi-constellation-demo-001".to_string(),
        available_balance: 200,
        gas_fee_percentage: 0.5,
    };

    let mut txs: Vec<ConstellationTxRecord> = Vec::new();

    // Keep the demo comfortably under 1 minute even if components are slow
    let total_txs: u64 = 8;
    let sleep_per_tx = Duration::from_secs(2);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(1))
        .build()?;

    for i in 0..total_txs {
        let amount: u64 = 5 + (i % 5);
        let gas_fee = calculate_gas_fee(&account, amount);
        let total_cost = amount + gas_fee;
        let balance_before = account.available_balance;
        let ts = Utc::now();
        let tx_id = format!("bpci_constellation_tx_{}", i + 1);

        let mut status = String::from("accepted");
        let mut balance_after = balance_before;
        let mut consensus_status = String::from("n/a");
        let mut lccd_round_started = false;
        let mut lccd_round_id: Option<String> = None;
        let mut lccd_round_status = String::from("n/a");
        let mut blockchain_status = String::from("n/a");
        let mut auction_status = String::from("n/a");
        let mut db_status = String::from("n/a");

        if balance_before < total_cost {
            status = "rejected_insufficient_balance".to_string();
        } else {
            account.available_balance -= total_cost;
            balance_after = account.available_balance;

            // 1. Consensus health (LCCD server)
            consensus_status = match client
                .get(format!("{}/api/v1/health", consensus_base))
                .send()
                .await
            {
                Ok(resp) if resp.status().is_success() => "healthy".to_string(),
                Ok(resp) => format!("status_{}", resp.status().as_u16()),
                Err(e) => format!("error_{}", e),
            };

            // 1b. Start an LCCD consensus round associated with this tx
            let lccd_payload = serde_json::json!({
                "tx_id": &tx_id,
                "amount": amount,
                "demo": true,
                // Minimal valid shape expected by the server
                "bundle_proposals": [],
            });

            match client
                .post(format!("{}/api/v1/lccd/consensus/start", consensus_base))
                .json(&lccd_payload)
                .send()
                .await
            {
                Ok(resp) => {
                    let code = resp.status();
                    if code.is_success() {
                        match resp.json::<serde_json::Value>().await {
                            Ok(v) => {
                                let id = v
                                    .get("round_id")
                                    .or_else(|| v.get("id"))
                                    .and_then(|x| x.as_str())
                                    .unwrap_or("unknown_round")
                                    .to_string();
                                lccd_round_started = true;
                                lccd_round_id = Some(id);
                                lccd_round_status = "started".to_string();
                            }
                            Err(e) => {
                                lccd_round_status = format!("parse_err_{}", e);
                            }
                        }
                    } else {
                        lccd_round_status = format!("status_{}", code.as_u16());
                    }
                }
                Err(e) => {
                    lccd_round_status = format!("error_{}", e);
                }
            }

            // 2. Blockchain submit
            let bc_payload = serde_json::json!({
                "amount": amount,
                "gas_fee": gas_fee,
                "tx_id": &tx_id,
            });

            blockchain_status = match client
                .post(format!("{}/api/v1/transactions", blockchain_base))
                .json(&bc_payload)
                .send()
                .await
            {
                Ok(resp) if resp.status().is_success() => "submitted".to_string(),
                Ok(resp) => format!("status_{}", resp.status().as_u16()),
                Err(e) => format!("error_{}", e),
            };

            // 3. Auction mempool
            let auction_payload = serde_json::json!({
                "tx_id": &tx_id,
                "amount": amount,
            });

            auction_status = match client
                .post(format!("{}/auction/submit", auction_base))
                .json(&auction_payload)
                .send()
                .await
            {
                Ok(resp) if resp.status().is_success() => "submitted".to_string(),
                Ok(resp) => format!("status_{}", resp.status().as_u16()),
                Err(e) => format!("error_{}", e),
            };

            // 4. Auction DB
            let db_payload = serde_json::json!({
                "tx_id": &tx_id,
                "from_bpi": &account.address,
                "to_bpci": "bpci-constellation-service",
                "amount": amount,
                "record_type": "bpi_bridge_transaction_demo",
            });

            db_status = match client
                .post(format!("{}/api/v1/auction/record", auction_db_base))
                .json(&db_payload)
                .send()
                .await
            {
                Ok(resp) if resp.status().is_success() => "recorded".to_string(),
                Ok(resp) => format!("status_{}", resp.status().as_u16()),
                Err(e) => format!("error_{}", e),
            };
        }

        txs.push(ConstellationTxRecord {
            index: i,
            tx_id,
            timestamp: ts,
            amount,
            gas_fee,
            total_cost,
            balance_before,
            balance_after,
            status,
            consensus_status,
            lccd_round_started,
            lccd_round_id,
            lccd_round_status,
            blockchain_status,
            auction_status,
            db_status,
        });

        sleep(sleep_per_tx).await;
    }

    let end_time = Utc::now();
    let duration_secs = (end_time - start_time).num_seconds().max(0) as u64;

    let report = render_report(&account, &txs, start_time, end_time, duration_secs);
    let report_path = "/tmp/bpci_constellation_demo_report.txt";
    let mut file = File::create(report_path)?;
    file.write_all(report.as_bytes())?;

    info!("BPCI constellation demo finished. Report: {}", report_path);
    println!("BPCI constellation demo finished. Report: {}", report_path);

    Ok(())
}

fn calculate_gas_fee(account: &DemoAccount, amount: u64) -> u64 {
    let fee_percentage = account.gas_fee_percentage / 100.0;
    let calculated = (amount as f64 * fee_percentage) as u64;
    std::cmp::max(1, calculated)
}

fn render_report(
    account: &DemoAccount,
    txs: &[ConstellationTxRecord],
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    duration_secs: u64,
) -> String {
    use std::fmt::Write as FmtWrite;

    let mut out = String::new();

    writeln!(out, "BPCI Constellation 1-Minute Demo Report").ok();
    writeln!(out, "=======================================").ok();
    writeln!(out, "").ok();

    writeln!(out, "Account: {}", account.address).ok();
    writeln!(out, "Initial balance: 200 BPI").ok();
    writeln!(out, "Final balance  : {} BPI", account.available_balance).ok();
    writeln!(out, "Duration (s)   : {}", duration_secs).ok();
    writeln!(out, "Total tx       : {}", txs.len()).ok();
    writeln!(out, "").ok();

    writeln!(out, "idx | amount | fee | total | status | cons_hlth | lccd_started | lccd_status | lccd_round | blockchain | auction | db").ok();
    writeln!(out, "----+--------+-----+-------+--------+-----------+-------------+-------------+------------+-----------+--------+--------").ok();

    for tx in txs {
        let lccd_started_flag = if tx.lccd_round_started { "yes" } else { "no" };
        let lccd_round_short = tx
            .lccd_round_id
            .as_ref()
            .map(|s| truncate(s, 10))
            .unwrap_or_else(|| "-".to_string());

        writeln!(
            out,
            "{:3} | {:6} | {:3} | {:5} | {:8} | {:11} | {:11} | {:11} | {:10} | {:9} | {:7} | {}",
            tx.index,
            tx.amount,
            tx.gas_fee,
            tx.total_cost,
            tx.status,
            truncate(&tx.consensus_status, 11),
            lccd_started_flag,
            truncate(&tx.lccd_round_status, 11),
            lccd_round_short,
            truncate(&tx.blockchain_status, 9),
            truncate(&tx.auction_status, 7),
            truncate(&tx.db_status, 9),
        )
        .ok();
    }

    out
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        s.chars().take(max_len.saturating_sub(1)).collect::<String>() + "…"
    }
}
