use bpi_core::quantum_entanglement::{EntanglementType, QuantumEntanglementEngine};

#[tokio::test(flavor = "current_thread")]
async fn bpi_core_26_quantum_entanglement_transaction_pair_preview() {
    println!("=== Test: BPI-CORE-26: Quantum entanglement transaction test ===");

    let engine = QuantumEntanglementEngine::new()
        .await
        .expect("failed to initialize QuantumEntanglementEngine");

    let tx_id1 = "tx-alpha-6d_consensus-demo-1";
    let tx_id2 = "tx-beta-6d_consensus-demo-2";

    let result = engine
        .create_transaction_entanglement(tx_id1, tx_id2, EntanglementType::TransactionPair)
        .await
        .expect("failed to create transaction entanglement");

    println!("entanglement_id: {}", result.entanglement_id);
    println!("coherence_factor: {:.4}", result.coherence_factor);
    println!("pattern_strength: {:.4}", result.pattern_strength);
    println!("security_level: {}", result.security_level);

    let proof_preview_len = std::cmp::min(48, result.cryptographic_proof.len());
    println!(
        "cryptographic_proof_preview: {}…",
        &result.cryptographic_proof[..proof_preview_len]
    );

    assert!(result.coherence_factor > 0.0 && result.coherence_factor <= 1.0);
    assert!(result.pattern_strength > 0.0 && result.pattern_strength <= 1.0);
    assert_eq!(result.security_level, "post_quantum_resistant");

    println!("status: OK");
}
