use bpi_core::immutable_audit_system::ImmutableAuditSystem;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[tokio::test(flavor = "current_thread")]
async fn bpi_core_06_immutable_audit_append_and_readback() {
    println!("=== Test: BPI-CORE-06: Immutable audit append + readback ===");

    let base = std::env::temp_dir();
    let storage_dir: PathBuf = base.join(format!("bpi_immutable_audit_test_{}", Uuid::new_v4()));
    let storage_str = storage_dir.to_string_lossy().to_string();

    let mut audit_system = ImmutableAuditSystem::new(&storage_str)
        .await
        .expect("failed to initialize ImmutableAuditSystem");

    let id1 = audit_system
        .record_code_execution_event(
            "test_action_1",
            "/usr/bin/demo1",
            vec!["--flag-a".to_string()],
            "test_context_1",
        )
        .await
        .expect("failed to record first code execution event");

    let id2 = audit_system
        .record_code_execution_event(
            "test_action_2",
            "/usr/bin/demo2",
            vec!["--flag-b".to_string()],
            "test_context_2",
        )
        .await
        .expect("failed to record second code execution event");

    println!("recorded_event_ids: {}, {}", id1, id2);

    let events_dir = storage_dir.join("events");
    let mut files: Vec<PathBuf> = Vec::new();

    if events_dir.exists() {
        for entry in fs::read_dir(&events_dir).expect("failed to read events directory") {
            let entry = entry.expect("invalid dir entry");
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                files.push(path);
            }
        }
    }

    println!("events_directory: {}", events_dir.to_string_lossy());
    println!("event_files_count: {}", files.len());

    for path in &files {
        match fs::read_to_string(path) {
            Ok(contents) => {
                println!(
                    "event_file: {} ({} bytes)",
                    path.file_name().and_then(|n| n.to_str()).unwrap_or("<invalid>"),
                    contents.len()
                );
            }
            Err(e) => {
                println!(
                    "event_file_error: {} -> {}",
                    path.to_string_lossy(),
                    e
                );
            }
        }
    }

    assert!(
        files.len() >= 2,
        "expected at least 2 event JSON files in events directory"
    );

    println!("status: OK");
}

#[tokio::test(flavor = "current_thread")]
async fn bpi_core_06b_immutable_audit_invariant_check() {
    println!("=== Test: BPI-CORE-06b: Immutable audit invariants in JSON record ===");

    let base = std::env::temp_dir();
    let storage_dir: PathBuf = base.join(format!("bpi_immutable_audit_test_invariants_{}", Uuid::new_v4()));
    let storage_str = storage_dir.to_string_lossy().to_string();

    let mut audit_system = ImmutableAuditSystem::new(&storage_str)
        .await
        .expect("failed to initialize ImmutableAuditSystem");

    let event_id = audit_system
        .record_code_execution_event(
            "test_action_invariant",
            "/usr/bin/demo_invariant",
            vec!["--flag-invariant".to_string()],
            "test_context_invariant",
        )
        .await
        .expect("failed to record code execution event");

    println!("recorded_event_id: {}", event_id);

    let events_dir = storage_dir.join("events");
    let mut files: Vec<PathBuf> = Vec::new();

    if events_dir.exists() {
        for entry in fs::read_dir(&events_dir).expect("failed to read events directory") {
            let entry = entry.expect("invalid dir entry");
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                files.push(path);
            }
        }
    }

    assert!(
        !files.is_empty(),
        "expected at least one event JSON file for invariant check",
    );

    let path = &files[0];
    let contents = fs::read_to_string(path).expect("failed to read event JSON file");
    println!(
        "event_file_for_invariants: {} ({} bytes)",
        path.file_name().and_then(|n| n.to_str()).unwrap_or("<invalid>"),
        contents.len()
    );

    let value: Value = serde_json::from_str(&contents).expect("failed to parse event JSON");

    let state_hash = value["system_state"]["state_hash"]
        .as_str()
        .expect("missing system_state.state_hash");
    let state_ts = value["system_state"]["timestamp"]
        .as_i64()
        .expect("missing system_state.timestamp");
    let record_ts = value["timestamp"]
        .as_i64()
        .expect("missing top-level timestamp");
    let proof_type = value["immutable_proof"]["proof_type"]
        .as_str()
        .expect("missing immutable_proof.proof_type");
    let crypt_hash = value["immutable_proof"]["cryptographic_hash"]
        .as_str()
        .expect("missing immutable_proof.cryptographic_hash");

    println!("system_state.timestamp: {}", state_ts);
    println!("record.timestamp: {}", record_ts);
    println!("state_hash: {}", state_hash);
    println!("immutable_proof.proof_type: {}", proof_type);
    println!("immutable_proof.cryptographic_hash: {}", crypt_hash);

    // Invariants based on ImmutableAuditSystem implementation
    assert_eq!(
        state_ts, record_ts,
        "system_state.timestamp should match top-level timestamp",
    );
    assert_eq!(
        proof_type,
        "code_execution_audit",
        "proof_type should be code_execution_audit for code execution events",
    );
    assert_eq!(
        state_hash, crypt_hash,
        "state_hash and cryptographic_hash should be identical for code execution audits",
    );

    assert!(state_hash.starts_with("0x"));
    assert_eq!(
        state_hash.len(),
        66,
        "state_hash should be 0x followed by 64 hex chars",
    );

    println!("status: OK");
}
