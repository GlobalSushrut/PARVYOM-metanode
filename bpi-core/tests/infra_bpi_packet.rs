use bpi_core::bpi_packet::{
    AuditEntry,
    AuditTrail,
    BpiPacket,
    ComplianceStatus,
    ConsensusProof,
    GovernmentComplianceAudit,
    MerkleProof,
    PoeProof,
    ResourceMetrics,
    StateChange,
    ExecutionStep,
    ValidatorSignature,
    VmAuditProof,
};
use bpi_core::cbor_pipeline_foundation::CborSerializable;
use bpi_core::logbook_6d_bridge::blockchain_writer::{
    CryptographicProofs,
    DimensionalCoordinates,
    SixDTransaction,
    TransactionData,
    TransactionType,
};
use bpi_core::blockchain_os_kernel::zk_kernel::{ZkProof, ZkProofType, SixDIntegration};
use chrono::Utc;

#[test]
fn bpi_core_27_bpi_packet_cbor_roundtrip_and_signature() {
    println!("=== Test: BPI-CORE-27: BpiPacket encode/decode + signature ===");

    // --- Build a sample SixDTransaction ---
    let coords = DimensionalCoordinates {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        t: Utc::now().timestamp() as f64,
        s: 0.85,
        q: 0.42,
    };

    let tx_data = TransactionData {
        operation_hash: "op-hash-demo".to_string(),
        input_data_hash: "input-hash-demo".to_string(),
        output_data_hash: "output-hash-demo".to_string(),
        execution_context: "vm://demo-context".to_string(),
        resource_usage: "cpu=10ms,mem=4MB".to_string(),
        performance_metrics: "latency_ms=5,tps=120".to_string(),
        audit_trail: "audit-demo".to_string(),
        compliance_data: "gdpr:ok,sox:ok".to_string(),
    };

    let cryptographic_proofs = CryptographicProofs {
        merkle_proof: "merkle-demo".to_string(),
        zero_knowledge_proof: "zk-demo".to_string(),
        quantum_proof: "quantum-demo".to_string(),
        consensus_proof: "consensus-demo".to_string(),
        integrity_proof: "integrity-demo".to_string(),
        non_repudiation_proof: "nr-demo".to_string(),
    };

    let integrity_hash = "tx-integrity-demo".to_string();

    let transaction = SixDTransaction {
        transaction_id: "tx-demo-1".to_string(),
        timestamp: Utc::now().timestamp() as u64,
        transaction_type: TransactionType::VMOperation,
        logbook_entry_id: "log-1".to_string(),
        dimensional_coordinates: coords,
        transaction_data: tx_data,
        cryptographic_proofs,
        poe_tree_root: Some("poe-root-demo".to_string()),
        traversal_report: Some("traversal-ok".to_string()),
        vm_audit_proof: Some("vm-audit-demo".to_string()),
        quantum_signature: "quantum-sig-demo".to_string(),
        integrity_hash: integrity_hash.clone(),
    };

    // --- Build Merkle/ZK/Consensus/PoE/VM-audit proofs ---
    let merkle_proof = MerkleProof {
        root_hash: integrity_hash.clone(),
        proof_path: vec!["leaf-0".to_string(), "leaf-1".to_string()],
        leaf_index: 0,
        leaf_data: vec![1, 2, 3, 4],
    };

    let zk_proof = ZkProof {
        proof_id: "zk-proof-demo".to_string(),
        proof_type: ZkProofType::Groth16,
        proof_data: vec![0, 1, 2, 3],
        public_inputs: vec![b"public-input-1".to_vec()],
        verification_key: b"vk-demo".to_vec(),
        generated_at: Utc::now(),
        generator_id: "kernel-demo-1".to_string(),
        battery_cost_mw: 10.5,
        verified: true,
        six_d_integration: Some(SixDIntegration {
            tx_hash: integrity_hash.clone(),
            block_height: 42,
            quantum_signature: b"q-sig".to_vec(),
            poe_tree_root: b"poe-root".to_vec(),
        }),
    };

    let consensus_proof = ConsensusProof {
        consensus_type: "QGC-C²".to_string(),
        validator_signatures: vec![ValidatorSignature {
            validator_id: hex::encode([7u8; 32]),
            signature: vec![8u8; 64],
            timestamp: Utc::now(),
            stake_weight: 100,
        }],
        consensus_timestamp: Utc::now(),
        finality_proof: "finality-demo".to_string(),
        quantum_entanglement_proof: "qe-demo".to_string(),
    };

    let poe_proof = PoeProof {
        tree_root: "poe-tree-root-demo".to_string(),
        execution_trace: vec![ExecutionStep {
            step_number: 1,
            operation: "EXEC_VM_OP".to_string(),
            input_state: "state-in-0".to_string(),
            output_state: "state-out-1".to_string(),
            gas_consumed: 123,
        }],
        resource_usage: ResourceMetrics {
            cpu_usage_ms: 10,
            memory_usage_bytes: 4 * 1024 * 1024,
            storage_io_ops: 8,
            network_bandwidth_bytes: 1024,
        },
        state_transitions: vec![StateChange {
            key: "storage:key1".to_string(),
            previous_value: "prev-hash".to_string(),
            new_value: "new-hash".to_string(),
            timestamp: Utc::now(),
        }],
    };

    let vm_audit_proof = VmAuditProof {
        vm_state_hash: "vm-state-demo".to_string(),
        execution_trace: vec!["trace-line-1".to_string(), "trace-line-2".to_string()],
        truthfulness_score: 0.99,
        witness_signatures: vec!["witness-1".to_string()],
        audit_timestamp: Utc::now(),
    };

    let audit_trail = AuditTrail {
        audit_entries: vec![AuditEntry {
            entry_id: "audit-1".to_string(),
            event_type: "vm_op".to_string(),
            description: "VM operation committed to 6D logbook".to_string(),
            timestamp: Utc::now(),
            auditor_signature: "auditor-demo".to_string(),
        }],
        compliance_score: 0.97,
        government_compliance: GovernmentComplianceAudit {
            framework: "SOX".to_string(),
            status: ComplianceStatus::Compliant,
            score: 0.99,
            last_audit_date: Utc::now(),
            officer_signature: "officer-demo".to_string(),
        },
        retention_years: 7,
        witness_signatures: vec!["witness-1".to_string()],
    };

    // --- Create packet, encode to CBOR, decode back ---
    let packet = BpiPacket::new(
        transaction,
        merkle_proof,
        zk_proof,
        consensus_proof,
        poe_proof,
        vm_audit_proof,
        audit_trail,
    )
    .expect("failed to construct BpiPacket");

    let encoded = packet.to_cbor().expect("CBOR encode failed");
    let decoded = BpiPacket::from_cbor(&encoded).expect("CBOR decode failed");

    println!("packet_hash: {}", packet.metadata.packet_hash);
    println!("packet_size_bytes: {}", packet.metadata.packet_size_bytes);
    println!("encoded_len: {}", encoded.len());
    println!("decoded_equals_original: {}", packet == decoded);

    assert_eq!(packet, decoded);

    // --- Sign and verify ---
    let mut signed_packet = decoded.clone();
    let private_key = [7u8; 32];
    signed_packet
        .sign(&private_key)
        .expect("failed to sign packet");

    let verified = signed_packet
        .verify_signature()
        .expect("signature verification errored");

    println!(
        "signature_len: {}",
        signed_packet.metadata.signature.len()
    );
    println!(
        "signer_public_key_len: {}",
        signed_packet.metadata.signer_public_key.len()
    );
    println!("verification_result: {}", verified);

    assert!(verified, "signature must verify successfully");

    println!("status: OK");
}
