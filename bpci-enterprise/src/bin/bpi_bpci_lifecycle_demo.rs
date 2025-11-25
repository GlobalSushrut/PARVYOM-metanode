use anyhow::Result;
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::Write;
use tracing::info;
use tracing_subscriber;

#[derive(Debug, Clone)]
struct PricingPlan {
    plan_name: String,
    monthly_cost_cad: f64,
    monthly_token_allocation: u64,
    free_allocation: u64,
    gas_fee_percentage: f64,
}

#[derive(Debug, Clone)]
struct DemoAccount {
    address: String,
    plan: PricingPlan,
    total_balance: u64,
    available_balance: u64,
    monthly_usage: u64,
}

#[derive(Debug, Clone)]
struct DemoTransaction {
    index: u64,
    tx_id: String,
    timestamp: DateTime<Utc>,
    amount: u64,
    gas_fee: u64,
    total_cost: u64,
    balance_before: u64,
    balance_after: u64,
    status: String,
    // Conceptual component lifecycle flags (offline-simulated)
    consensus_ok: bool,
    blockchain_status: String,
    auction_status: String,
    db_status: String,
    bundle_id: Option<String>,
}

#[derive(Debug, Clone)]
struct BundleSummary {
    bundle_id: String,
    tx_count: usize,
    total_principal: u64,
    total_gas: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("bpi_bpci_lifecycle_demo=info,bpci_enterprise=info")
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("🚀 Starting BPI→BPCI bundle + lifecycle mini-demo (testnet-style)");

    let report_path = "/tmp/bpi_bpci_lifecycle_demo_report.txt";
    let total_txs: u64 = 20;

    let start_time = Utc::now();

    // Testnet pricing plan aligned with bpci_bpi_bridge::initialize_pricing_plans
    let plan = PricingPlan {
        plan_name: "Testnet".to_string(),
        monthly_cost_cad: 10.0,
        monthly_token_allocation: 1000,
        free_allocation: 200,
        gas_fee_percentage: 0.5, // 0.5% gas
    };

    let mut account = DemoAccount {
        address: "bpi-testnet-user-001".to_string(),
        plan: plan.clone(),
        total_balance: plan.free_allocation,
        available_balance: plan.free_allocation,
        monthly_usage: 0,
    };

    info!(
        "✅ Created testnet account {} with free allocation {} BPI (plan: {})",
        account.address,
        account.available_balance,
        account.plan.plan_name,
    );

    let mut txs: Vec<DemoTransaction> = Vec::new();
    let mut accepted = 0u64;
    let mut rejected = 0u64;
    let mut total_gas_fees = 0u64;
    let mut total_principal = 0u64;

    for i in 0..total_txs {
        let amount: u64 = 5 + (i % 5); // 5..9 BPI
        let gas_fee = calculate_gas_fee(&account, amount);
        let total_cost = amount + gas_fee;
        let ts = Utc::now();
        let tx_id = format!("demo_tx_{}", i + 1);

        let balance_before = account.available_balance;
        let (status, balance_after, consensus_ok, blockchain_status, auction_status, db_status) =
            if balance_before >= total_cost {
            account.available_balance -= total_cost;
            account.monthly_usage += total_cost;
            accepted += 1;
            total_gas_fees += gas_fee;
            total_principal += amount;

            (
                "accepted".to_string(),
                account.available_balance,
                true,
                "queued_for_blockchain".to_string(),
                "pending_auction_bundle".to_string(),
                "pending_db_record".to_string(),
            )
        } else {
            rejected += 1;
            (
                "rejected_insufficient_balance".to_string(),
                balance_before,
                false,
                "skipped_insufficient_balance".to_string(),
                "skipped".to_string(),
                "skipped".to_string(),
            )
        };

        txs.push(DemoTransaction {
            index: i,
            tx_id,
            timestamp: ts,
            amount,
            gas_fee,
            total_cost,
            balance_before,
            balance_after,
            status,
            consensus_ok,
            blockchain_status,
            auction_status,
            db_status,
            bundle_id: None,
        });
    }

    // Conceptual rebundling + auction/DB lifecycle for accepted transactions
    let bundles = rebundle_transactions(&mut txs);

    let end_time = Utc::now();
    let duration_secs = (end_time - start_time).num_seconds().max(0) as u64;

    let report = render_report(
        &account,
        &txs,
        &bundles,
        accepted,
        rejected,
        total_principal,
        total_gas_fees,
        start_time,
        end_time,
        duration_secs,
    );

    let mut file = File::create(report_path)?;
    file.write_all(report.as_bytes())?;

    info!("✅ BPI→BPCI lifecycle demo completed in ~{}s", duration_secs);
    info!("📝 Detailed report written to {}", report_path);

    println!("BPI→BPCI lifecycle demo finished. Report: {}", report_path);

    Ok(())
}

fn calculate_gas_fee(account: &DemoAccount, amount: u64) -> u64 {
    let fee_percentage = account.plan.gas_fee_percentage / 100.0;
    let calculated = (amount as f64 * fee_percentage) as u64;
    std::cmp::max(1, calculated) // mimic bridge minimum fee logic
}

fn rebundle_transactions(txs: &mut [DemoTransaction]) -> Vec<BundleSummary> {
    // Group accepted transactions into small bundles (e.g., 5 per bundle) to
    // conceptually mirror BPCI rebundling + auction selection.

    // First pass: collect index groups using only immutable access, to satisfy
    // the borrow checker.
    let mut bundle_indices: Vec<Vec<usize>> = Vec::new();
    let mut current_bundle: Vec<usize> = Vec::new();

    for (idx, tx) in txs.iter().enumerate() {
        if tx.status == "accepted" {
            current_bundle.push(idx);
            if current_bundle.len() >= 5 {
                bundle_indices.push(current_bundle.clone());
                current_bundle.clear();
            }
        }
    }

    if !current_bundle.is_empty() {
        bundle_indices.push(current_bundle);
    }

    // Second pass: mutate txs based on the precomputed index groups.
    let mut bundles = Vec::new();
    for (i, indices) in bundle_indices.iter().enumerate() {
        let bundle_id = format!("bundle_{}", i + 1);
        finalize_bundle(&bundle_id, &mut bundles, indices, txs);
    }

    bundles
}

fn finalize_bundle(
    bundle_id: &str,
    bundles: &mut Vec<BundleSummary>,
    indices: &[usize],
    txs: &mut [DemoTransaction],
) {
    let mut total_principal = 0u64;
    let mut total_gas = 0u64;

    for &idx in indices {
        let tx = &mut txs[idx];
        tx.bundle_id = Some(bundle_id.to_string());
        // Conceptually: blockchain + auction + DB all mark this tx as included
        tx.blockchain_status = "committed_in_blockchain_bundle".to_string();
        tx.auction_status = "included_in_auction_bundle".to_string();
        tx.db_status = "recorded_in_db".to_string();

        total_principal += tx.amount;
        total_gas += tx.gas_fee;
    }

    bundles.push(BundleSummary {
        bundle_id: bundle_id.to_string(),
        tx_count: indices.len(),
        total_principal,
        total_gas,
    });
}

fn render_report(
    account: &DemoAccount,
    txs: &[DemoTransaction],
    bundles: &[BundleSummary],
    accepted: u64,
    rejected: u64,
    total_principal: u64,
    total_gas_fees: u64,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    duration_secs: u64,
) -> String {
    use std::fmt::Write as FmtWrite;

    let mut out = String::new();

    writeln!(out, "BPI→BPCI Testnet Lifecycle Mini-Demo Report").ok();
    writeln!(out, "==========================================").ok();
    writeln!(out, "").ok();

    writeln!(out, "1. Overview").ok();
    writeln!(out, "-----------").ok();
    writeln!(out, "Account address    : {}", account.address).ok();
    writeln!(out, "Plan name          : {}", account.plan.plan_name).ok();
    writeln!(out, "Plan monthly cost  : {:.2} CAD", account.plan.monthly_cost_cad).ok();
    writeln!(out, "Monthly allocation : {} BPI", account.plan.monthly_token_allocation).ok();
    writeln!(out, "Free allocation    : {} BPI", account.plan.free_allocation).ok();
    writeln!(out, "Gas fee percentage : {:.3}%", account.plan.gas_fee_percentage).ok();
    writeln!(out, "Start time (UTC)   : {}", start_time.to_rfc3339()).ok();
    writeln!(out, "End time (UTC)     : {}", end_time.to_rfc3339()).ok();
    writeln!(out, "Duration (seconds) : {}", duration_secs).ok();
    writeln!(out, "Total tx attempted : {}", txs.len()).ok();
    writeln!(out, "Tx accepted        : {}", accepted).ok();
    writeln!(out, "Tx rejected        : {}", rejected).ok();
    writeln!(out, "").ok();

    let initial_balance = account.plan.free_allocation;
    let final_balance = account.available_balance;

    writeln!(out, "2. Account Lifecycle Summary").ok();
    writeln!(out, "----------------------------").ok();
    writeln!(out, "Initial free balance : {} BPI", initial_balance).ok();
    writeln!(out, "Final balance        : {} BPI", final_balance).ok();
    writeln!(out, "Total principal sent : {} BPI", total_principal).ok();
    writeln!(out, "Total gas fees       : {} BPI", total_gas_fees).ok();
    writeln!(out, "Total usage          : {} BPI", total_principal + total_gas_fees).ok();
    writeln!(out, "").ok();

    writeln!(out, "3. BPI Bundle Flow Timeline (offline-simulated)").ok();
    writeln!(out, "-----------------------------------------------").ok();
    writeln!(out, "idx | timestamp                  | amount | fee | total | bal_before | bal_after | status                         | bundle_id").ok();
    writeln!(out, "----+----------------------------+--------+-----+-------+-----------+-----------+------------------------------+----------").ok();

    for tx in txs {
        writeln!(
            out,
            "{:3} | {} | {:6} | {:3} | {:5} | {:9} | {:9} | {:28} | {}",
            tx.index,
            tx.timestamp.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            tx.amount,
            tx.gas_fee,
            tx.total_cost,
            tx.balance_before,
            tx.balance_after,
            tx.status,
            tx.bundle_id.clone().unwrap_or_else(|| "<none>".to_string()),
        )
        .ok();
    }

    writeln!(out, "").ok();

    writeln!(out, "4. Bundle & Auction Summary (Conceptual)").ok();
    writeln!(out, "----------------------------------------").ok();
    if bundles.is_empty() {
        writeln!(out, "No bundles were formed (insufficient accepted transactions).").ok();
    } else {
        writeln!(out, "Total bundles formed : {}", bundles.len()).ok();
        for b in bundles {
            writeln!(
                out,
                "- {} → tx_count={} principal={} gas_fees={}",
                b.bundle_id,
                b.tx_count,
                b.total_principal,
                b.total_gas,
            )
            .ok();
        }
    }
    writeln!(out, "").ok();

    writeln!(out, "5. Component Lifecycle (Conceptual)").ok();
    writeln!(out, "-----------------------------------").ok();
    writeln!(out, "- Component 1 (Consensus): In this offline demo, consensus readiness is assumed. In production,").ok();
    writeln!(out, "  the bridge would call the consensus health endpoint before processing a bundle.").ok();
    writeln!(out, "- Component 2 (Blockchain): Transaction submissions and fee accounting would be recorded via HTTP.").ok();
    writeln!(out, "  Here we simulate only the accounting side (principal + gas fee debits).").ok();
    writeln!(out, "- Component 3 (Auction Mempool): In production, each tx becomes a candidate in the auction mempool.").ok();
    writeln!(out, "  This demo models the BPI-side bundle economics without a real mempool.").ok();
    writeln!(out, "- Component 4 (Auction DB): Persistent records would be written via the DB maintainer. Here we").ok();
    writeln!(out, "  capture a time-series table instead, suitable for testnet analysis.").ok();
    writeln!(out, "").ok();

    writeln!(out, "5. Observations").ok();
    writeln!(out, "----------------").ok();
    writeln!(out, "- This demo is fully in-memory and offline; it does not contact any external servers.").ok();
    writeln!(out, "- Gas fee calculation matches the bridge testnet plan: {:.3}% with a minimum 1 BPI per tx.", account.plan.gas_fee_percentage).ok();
    writeln!(out, "- The report shows the full BPI token lifecycle for a single testnet account: from free allocation,").ok();
    writeln!(out, "  through multiple bundle-like sends, to final remaining balance and total usage.").ok();
    writeln!(out, "- It is designed to be laptop-safe and quick to run, ideal for reviewers and testnet tuning.").ok();

    out
}
