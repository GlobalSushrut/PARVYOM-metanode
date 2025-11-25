use std::collections::BTreeMap;

use bpi_core::logbook_6d_bridge::blockchain_writer::{
    SixDBlockchainWriter,
    SixDTransaction,
    TransactionType,
    DimensionalCoordinates,
    TransactionData,
    CryptographicProofs,
};
use bpi_core::logbook_6d_bridge::logbook_reader::{
    BPILogbookReader,
    LogbookEntry,
};
use chrono::Utc;
use tokio;

#[tokio::test(flavor = "current_thread")]
async fn bpi_core_14_logbook_6d_write_read_cross_check() {
    println!("=== Test: BPI-CORE-14: 6D logbook write + read cross-check ===");

    // Initialize the 6D blockchain writer
    let writer = SixDBlockchainWriter::new()
        .await
        .expect("failed to create SixDBlockchainWriter");
    writer
        .initialize()
        .await
        .expect("failed to initialize 6D blockchain writer");

    // Build a small batch of synthetic 6D transactions
    let txs: Vec<SixDTransaction> = (0..3)
        .map(|i| {
            let tx_id = format!("logbook_tx_{}", i + 1);
            let timestamp = Utc::now().timestamp() as u64;

            let coords = DimensionalCoordinates {
                x: i as f64,
                y: (i + 1) as f64,
                z: (i + 2) as f64,
                t: timestamp as f64,
                s: 0.75,
                q: 0.9,
            };

            let data = TransactionData {
                operation_hash: format!("op_hash_{}", tx_id),
                input_data_hash: format!("input_{}", tx_id),
                output_data_hash: format!("output_{}", tx_id),
                execution_context: "infra_logbook_test".to_string(),
                resource_usage: format!("cpu={} mem=128", i + 1),
                performance_metrics: "latency_ms=7".to_string(),
                audit_trail: "audit_log_entry".to_string(),
                compliance_data: "compliant".to_string(),
            };

            let proofs = CryptographicProofs {
                merkle_proof: "merkle_demo".to_string(),
                zero_knowledge_proof: "zk_demo".to_string(),
                quantum_proof: "quantum_demo".to_string(),
                consensus_proof: "consensus_demo".to_string(),
                integrity_proof: format!("integrity_{}", tx_id),
                non_repudiation_proof: "non_repudiation_demo".to_string(),
            };

            SixDTransaction {
                transaction_id: tx_id,
                timestamp,
                transaction_type: TransactionType::AuditRecord,
                logbook_entry_id: format!("log_entry_{}", i + 1),
                dimensional_coordinates: coords,
                transaction_data: data,
                cryptographic_proofs: proofs,
                poe_tree_root: None,
                traversal_report: None,
                vm_audit_proof: None,
                quantum_signature: "quantum_sig_demo".to_string(),
                integrity_hash: format!("integrity_hash_{}", i + 1),
            }
        })
        .collect();

    println!("writer_transaction_batch_size: {}", txs.len());
    for tx in &txs {
        println!(
            "  - tx_id={} type={:?} logbook_entry_id={} integrity_hash={}",
            tx.transaction_id, tx.transaction_type, tx.logbook_entry_id, tx.integrity_hash
        );
    }

    // Write these transactions into a single 6D block
    let block_hash = writer
        .write_transactions_to_block(txs.clone())
        .await
        .expect("failed to write transactions to 6D block");

    let state = writer
        .get_blockchain_state()
        .await
        .expect("failed to get blockchain state");
    let stats = writer
        .get_stats()
        .await
        .expect("failed to get writer stats");

    println!("6d_blockchain_state:");
    println!("  chain_id: {}", state.chain_id);
    println!("  current_block_number: {}", state.current_block_number);
    println!("  last_block_hash: {}", state.last_block_hash);
    println!("  total_transactions: {}", state.total_transactions);
    println!("  chain_length: {}", state.chain_length);
    println!("6d_writer_stats:");
    println!("  total_blocks_created: {}", stats.total_blocks_created);
    println!("  total_transactions_written: {}", stats.total_transactions_written);
    println!("  average_block_size: {:.1}", stats.average_block_size);

    // Initialize the BPI logbook reader
    let reader = BPILogbookReader::new()
        .await
        .expect("failed to create BPILogbookReader");
    reader
        .initialize()
        .await
        .expect("failed to initialize BPILogbookReader");

    // Read a batch of new logbook entries (mocked by the reader implementation)
    let new_entries: Vec<LogbookEntry> = reader
        .read_new_entries()
        .await
        .expect("failed to read new logbook entries");

    println!("logbook_new_entries_count: {}", new_entries.len());
    for entry in &new_entries {
        println!(
            "  - entry_id={} type={:?} vm_instance_id={} integrity_hash={}",
            entry.entry_id, entry.entry_type, entry.vm_instance_id, entry.integrity_hash
        );
    }

    // Cross-check a subset of entries by ID using read_entries_by_ids
    let selected_ids: Vec<String> = new_entries
        .iter()
        .take(3)
        .map(|e| e.entry_id.clone())
        .collect();

    let entries_by_id: Vec<LogbookEntry> = reader
        .read_entries_by_ids(selected_ids.clone())
        .await
        .expect("failed to read logbook entries by IDs");

    println!("logbook_entries_by_id_count: {}", entries_by_id.len());

    let mut entry_summary: BTreeMap<String, (String, u64, u64)> = BTreeMap::new();
    for entry in &entries_by_id {
        entry_summary.insert(
            entry.entry_id.clone(),
            (
                format!("{:?}", entry.entry_type),
                entry.resource_usage.cpu_time_ms,
                entry.performance_metrics.execution_time_ms,
            ),
        );
    }

    println!("logbook_entry_summary_table:");
    for (entry_id, (etype, cpu_ms, exec_ms)) in &entry_summary {
        println!(
            "  - entry_id={} type={} cpu_time_ms={} exec_time_ms={}",
            entry_id, etype, cpu_ms, exec_ms
        );
    }

    // Invariants for this infra preview
    // 1. Blockchain state should reflect our synthetic batch
    assert_eq!(
        state.total_transactions as usize,
        txs.len(),
        "6D blockchain should report total_transactions matching our batch size",
    );
    assert_eq!(
        state.current_block_number, 1,
        "expected exactly one block to be created in this preview",
    );
    assert_eq!(
        state.chain_length, 1,
        "expected chain_length to be 1 after first block",
    );

    // 2. Writer stats should show at least one block created
    assert!(
        stats.total_blocks_created >= 1,
        "writer should report at least one block created",
    );
    assert!(
        stats.average_block_size >= 1.0,
        "average_block_size should be at least 1.0 after writing a block",
    );

    // 3. Logbook reader should return entries and be able to resolve them by ID
    assert!(
        !new_entries.is_empty(),
        "logbook reader should return some new entries in this preview",
    );
    assert_eq!(
        entries_by_id.len(),
        selected_ids.len(),
        "read_entries_by_ids should return one entry per requested ID",
    );

    // 4. Basic sanity checks on performance metrics for entries
    for entry in &entries_by_id {
        assert!(
            entry.performance_metrics.execution_time_ms > 0,
            "execution_time_ms should be positive for entry {}",
            entry.entry_id
        );
        assert!(
            entry.performance_metrics.latency_percentiles.p50_ms >= 0.0,
            "p50 latency should be non-negative for entry {}",
            entry.entry_id
        );
    }

    // Print final status line for human-readable test output
    println!("status: OK");

    // Stop components cleanly
    reader
        .stop()
        .await
        .expect("failed to stop logbook reader");
    writer
        .stop()
        .await
        .expect("failed to stop 6D blockchain writer");
}
