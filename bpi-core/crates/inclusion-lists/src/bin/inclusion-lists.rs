use anyhow::Result;
use bpi_inclusion_lists::{
    InclusionListManager, InclusionListConfig, PendingObligation, ObligationId, ObligationType
};
use bpi_mempool::TxId;
use chrono::Utc;
use clap::{Arg, Command};
use tokio::time::{sleep, Duration};
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let matches = Command::new("inclusion-lists")
        .version("0.1.0")
        .about("BPI Inclusion Lists Manager")
        .arg(
            Arg::new("max-obligations")
                .long("max-obligations")
                .value_name("COUNT")
                .help("Maximum pending obligations")
                .default_value("10000"),
        )
        .arg(
            Arg::new("timeout-blocks")
                .long("timeout-blocks")
                .value_name("BLOCKS")
                .help("Obligation timeout in blocks")
                .default_value("32"),
        )
        .arg(
            Arg::new("max-list-size")
                .long("max-list-size")
                .value_name("SIZE")
                .help("Maximum inclusion list size")
                .default_value("1000"),
        )
        .arg(
            Arg::new("enforcement-window")
                .long("enforcement-window")
                .value_name("BLOCKS")
                .help("Enforcement window in blocks")
                .default_value("8"),
        )
        .get_matches();

    let config = InclusionListConfig {
        max_pending_obligations: matches.get_one::<String>("max-obligations").unwrap().parse()?,
        obligation_timeout_blocks: matches.get_one::<String>("timeout-blocks").unwrap().parse()?,
        max_list_size: matches.get_one::<String>("max-list-size").unwrap().parse()?,
        enforcement_window_blocks: matches.get_one::<String>("enforcement-window").unwrap().parse()?,
        slashing_evidence_retention_blocks: 256,
        validator_check_interval_ms: 6000,
    };

    info!("Starting BPI Inclusion Lists Manager with config: {:?}", config);

    let manager = InclusionListManager::new(config);

    // Demo: Create and manage inclusion lists
    info!("Running inclusion lists demo...");

    // Create test obligations
    let mut obligations = Vec::new();
    for i in 0..5 {
        let obligation = PendingObligation {
            id: ObligationId::random(),
            tx_id: TxId::random(),
            proposer: format!("proposer{}", i).into_bytes(),
            created_block: 100 + i,
            deadline_block: 110 + i,
            obligation_type: ObligationType::TransactionInclusion,
            data: format!("test_data_{}", i).into_bytes(),
            timestamp: Utc::now(),
        };

        manager.add_pending_obligation(obligation.clone()).await?;
        obligations.push(obligation);
        info!("Added pending obligation {}", i);
    }

    // Create inclusion lists
    for i in 0..3 {
        let block_number = 105 + i * 5;
        let proposer = format!("proposer{}", i).into_bytes();
        let obligation_ids = vec![obligations[i as usize].id.clone()];

        let list = manager.create_inclusion_list(block_number, proposer, obligation_ids).await?;
        info!("Created inclusion list for block {} with {} obligations", 
              list.block_number, list.obligations.len());

        // Verify the list
        let is_valid = manager.verify_inclusion_list(&list).await?;
        info!("Inclusion list verification: {}", is_valid);
    }

    // Detect missing items
    let missing_items = manager.detect_missing_items(120).await?;
    info!("Detected {} missing items", missing_items.len());

    // Generate slashing evidence if there are missing items
    if !missing_items.is_empty() {
        let evidence = manager.generate_slashing_evidence(
            b"validator123".to_vec(),
            missing_items,
            (100, 120),
        ).await?;
        info!("Generated slashing evidence with severity score: {}", evidence.severity_score);
    }

    // Test proposer compliance
    for obligation in &obligations {
        let requirements = manager.get_proposer_requirements(&obligation.proposer, 115).await;
        info!("Proposer requirements for block 115: {} obligations", requirements.len());

        let is_compliant = manager.validate_proposer_compliance(
            &obligation.proposer,
            115,
            &requirements,
        ).await?;
        info!("Proposer compliance: {}", is_compliant);
    }

    // Cleanup expired obligations
    let removed_count = manager.cleanup_expired_obligations(150).await?;
    info!("Cleaned up {} expired obligations", removed_count);

    // Show inclusion list stats
    let stats = manager.get_inclusion_list_stats().await;
    info!("Inclusion list stats: {:?}", stats);

    info!("Inclusion lists demo completed successfully!");
    Ok(())
}
