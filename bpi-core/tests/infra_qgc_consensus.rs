use std::collections::HashMap;

use bpi_core::logbook_6d_bridge::blockchain_writer::{
    SixDTransaction,
    TransactionType,
    DimensionalCoordinates,
    TransactionData,
    CryptographicProofs,
};
use bpi_core::qgc_consensus::{
    QgcConsensusEngine,
    Validator,
    ValidatorStatus,
};
use chrono::Utc;
use ed25519_dalek::SigningKey;
use rand::{rngs::StdRng, SeedableRng};
use tokio;

#[tokio::test(flavor = "current_thread")]
async fn bpi_core_13_qgc_consensus_happy_path_round() {
    println!("=== Test: BPI-CORE-13: QGC consensus happy-path round ===");

    // Create a small validator set with real Ed25519 keys
    let mut rng = StdRng::from_seed([42u8; 32]);
    let mut validators: Vec<Validator> = Vec::new();
    let mut private_keys: HashMap<String, [u8; 32]> = HashMap::new();

    for i in 0..4 {
        let signing_key: SigningKey = SigningKey::generate(&mut rng);
        let public_key = signing_key.verifying_key().to_bytes().to_vec();
        let private_bytes: [u8; 32] = signing_key.to_bytes();

        let validator_id = format!("validator-{}", i + 1);
        let validator = Validator {
            validator_id: validator_id.clone(),
            public_key,
            stake: 1_000,
            reputation: 1.0,
            last_activity: Utc::now(),
            status: ValidatorStatus::Active,
        };

        println!(
            "validator: id={} stake={} status={:?}",
            validator.validator_id, validator.stake, validator.status
        );

        private_keys.insert(validator_id, private_bytes);
        validators.push(validator);
    }

    // Initialize QGC-C² consensus engine
    let engine = QgcConsensusEngine::new()
        .await
        .expect("failed to create QgcConsensusEngine");
    engine
        .initialize(validators.clone())
        .await
        .expect("failed to initialize validator set");

    // Build a small batch of 6D transactions
    let txs: Vec<SixDTransaction> = (0..2)
        .map(|i| {
            let tx_id = format!("tx-{}", i + 1);
            let timestamp = Utc::now().timestamp() as u64;

            let coords = DimensionalCoordinates {
                x: i as f64,
                y: (i + 1) as f64,
                z: 0.0,
                t: timestamp as f64,
                s: 0.99,
                q: 1.0,
            };

            let data = TransactionData {
                operation_hash: format!("op_hash_{}", tx_id),
                input_data_hash: format!("input_hash_{}", tx_id),
                output_data_hash: format!("output_hash_{}", tx_id),
                execution_context: "qgc_consensus_test".to_string(),
                resource_usage: "cpu=1,mem=64".to_string(),
                performance_metrics: "latency_ms=5".to_string(),
                audit_trail: "audit_entry".to_string(),
                compliance_data: "compliant".to_string(),
            };

            let proofs = CryptographicProofs {
                merkle_proof: "merkle_proof_demo".to_string(),
                zero_knowledge_proof: "zk_proof_demo".to_string(),
                quantum_proof: "quantum_proof_demo".to_string(),
                consensus_proof: "consensus_proof_demo".to_string(),
                integrity_proof: "integrity_proof_demo".to_string(),
                non_repudiation_proof: "non_repudiation_proof_demo".to_string(),
            };

            SixDTransaction {
                transaction_id: tx_id,
                timestamp,
                transaction_type: TransactionType::VMOperation,
                logbook_entry_id: format!("logbook-{}", i + 1),
                dimensional_coordinates: coords,
                transaction_data: data,
                cryptographic_proofs: proofs,
                poe_tree_root: None,
                traversal_report: None,
                vm_audit_proof: None,
                quantum_signature: "quantum_sig_demo".to_string(),
                integrity_hash: "integrity_hash_demo".to_string(),
            }
        })
        .collect();

    println!("transaction_batch_size: {}", txs.len());

    // Start a consensus round for these transactions
    let round_id = engine
        .start_consensus_round(txs.clone())
        .await
        .expect("failed to start consensus round");

    println!("round_id: {}", round_id);

    // For this infra test we use a synthetic block hash label for votes;
    // the engine uses stake-based thresholds and does not re-derive it from the vote.
    let vote_block_hash = "synthetic_block_hash_for_demo";

    // Submit prevotes from all validators
    for v in &validators {
        let priv_bytes = private_keys
            .get(&v.validator_id)
            .expect("missing private key for validator");

        engine
            .submit_prevote(&round_id, &v.validator_id, vote_block_hash, priv_bytes)
            .await
            .expect("failed to submit prevote");

        println!(
            "prevote_submitted: validator={} block_hash={}",
            v.validator_id, vote_block_hash
        );
    }

    // Submit precommits from a 2/3+ quorum (3 out of 4 validators)
    for v in validators.iter().take(3) {
        let priv_bytes = private_keys
            .get(&v.validator_id)
            .expect("missing private key for validator");

        engine
            .submit_precommit(&round_id, &v.validator_id, vote_block_hash, priv_bytes)
            .await
            .expect("failed to submit precommit");

        println!(
            "precommit_submitted: validator={} block_hash={}",
            v.validator_id, vote_block_hash
        );
    }

    // At this point the precommit threshold should be reached and the block finalized at height 1
    let proof_opt = engine
        .get_consensus_proof(1)
        .await
        .expect("failed to query consensus proof for height 1");

    println!("consensus_proof_present: {}", proof_opt.is_some());

    let proof = proof_opt.expect("expected consensus proof at height 1");

    println!("qgc_consensus_result:");
    println!("  consensus_type: {}", proof.consensus_type);
    println!("  finality_proof: {}", proof.finality_proof);
    println!(
        "  validator_signatures_count: {}",
        proof.validator_signatures.len()
    );

    for sig in &proof.validator_signatures {
        println!(
            "  - validator_signature: id={} weight={} timestamp={}",
            sig.validator_id, sig.stake_weight, sig.timestamp
        );
    }

    // Invariants
    // 1. A 2/3+ quorum of validators should have contributed signatures
    assert_eq!(
        proof.validator_signatures.len(),
        3,
        "expected signatures from the 2/3+ quorum in happy-path round",
    );

    // 2. Consensus type should be QGC-C² as produced by the engine
    assert_eq!(
        proof.consensus_type,
        "QGC-C²".to_string(),
        "unexpected consensus_type in consensus proof",
    );

    // 3. Participation weights should all be positive
    for sig in &proof.validator_signatures {
        assert!(
            sig.stake_weight > 0,
            "validator {} has non-positive stake_weight in consensus proof",
            sig.validator_id
        );
    }

    println!("status: OK");
}
