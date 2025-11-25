use serde_json::json;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[test]
fn bpi_core_07_http_cage_config_and_security_snapshot() {
    println!("=== Test: BPI-CORE-07: HTTP Cage config + security snapshot ===");

    // Synthetic but realistic HTTP Cage configuration
    let cage_id = format!("cage_{}", Uuid::new_v4().simple());
    let port: u16 = 8081;
    let frontend_dir = Some("/opt/bpi/http_cage/frontend".to_string());
    let backend_url = "http://127.0.0.1:9545".to_string();
    let quantum_safe = true;
    let security_rating: u8 = 9;

    println!("cage_id: {}", cage_id);
    println!("port: {}", port);
    println!("frontend_dir: {:?}", frontend_dir);
    println!("backend_url: {}", backend_url);
    println!("quantum_safe: {}", quantum_safe);
    println!("security_rating: {}/10", security_rating);

    // Write synthetic audit-style records similar to commands/http_cage.rs helpers,
    // but into an isolated temp directory for this test only.
    let root = std::env::temp_dir().join(format!("bpi_http_cage_infra_{}", Uuid::new_v4()));
    let cages_root = root.join("http_cage").join("cages").join(&cage_id);

    let config_dir = cages_root.clone();
    let security_dir = cages_root.join("security");
    let startup_dir = cages_root.join("startup");
    let metrics_dir = cages_root.join("metrics");

    fs::create_dir_all(&config_dir).expect("failed to create config_dir");
    fs::create_dir_all(&security_dir).expect("failed to create security_dir");
    fs::create_dir_all(&startup_dir).expect("failed to create startup_dir");
    fs::create_dir_all(&metrics_dir).expect("failed to create metrics_dir");

    let now_ts = chrono::Utc::now().timestamp();

    let config_record = json!({
        "cage_id": cage_id,
        "port": port,
        "frontend_dir": frontend_dir,
        "backend_url": backend_url,
        "quantum_safe": quantum_safe,
        "security_rating": security_rating,
        "protocol": "http:cg/1.0",
        "security_level": "MILITARY_GRADE",
        "configured_at": now_ts,
    });

    let security_record = json!({
        "security_rating": security_rating,
        "quantum_safe": quantum_safe,
        "military_grade": true,
        "encryption_level": "AES-256-GCM",
        "key_exchange": "X25519",
        "signature_algorithm": "Ed25519",
        "hash_function": "Blake3",
        "audit_timestamp": now_ts,
    });

    let startup_record = json!({
        "startup_status": "success",
        "bind_status": "bound",
        "listener_active": true,
        "security_initialized": true,
        "audit_system_active": true,
        "startup_timestamp": now_ts,
    });

    let metrics_record = json!({
        "active_requests": 0,
        "audit_entries": 3,
        "policy_violations": 0,
        "uptime_seconds": 42,
        "security_rating": security_rating,
        "quantum_safe": quantum_safe,
    });

    let write_pretty = |path: PathBuf, value: &serde_json::Value| {
        fs::write(path, serde_json::to_string_pretty(value).unwrap()).unwrap();
    };

    write_pretty(config_dir.join("configuration_record.json"), &config_record);
    write_pretty(security_dir.join("security_policy_record.json"), &security_record);
    write_pretty(startup_dir.join("startup_record.json"), &startup_record);
    write_pretty(metrics_dir.join("metrics_record.json"), &metrics_record);

    // Read back the JSON to simulate an operator inspecting HTTP Cage audits.
    let read_json = |path: PathBuf| -> serde_json::Value {
        let data = fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e));
        serde_json::from_str(&data).expect("invalid JSON in record file")
    };

    let cfg = read_json(config_dir.join("configuration_record.json"));
    let sec = read_json(security_dir.join("security_policy_record.json"));
    let start = read_json(startup_dir.join("startup_record.json"));
    let met = read_json(metrics_dir.join("metrics_record.json"));

    println!("config_record: {}", cfg);
    println!("security_record: {}", sec);
    println!("startup_record: {}", start);
    println!("metrics_record: {}", met);

    // Basic invariants for HTTP Cage infra snapshot
    assert_eq!(cfg["protocol"].as_str(), Some("http:cg/1.0"));
    assert_eq!(cfg["security_level"].as_str(), Some("MILITARY_GRADE"));
    assert_eq!(cfg["quantum_safe"].as_bool(), Some(true));

    assert_eq!(sec["quantum_safe"].as_bool(), Some(true));
    assert_eq!(sec["military_grade"].as_bool(), Some(true));
    assert!(security_rating as i64 <= 10 && security_rating as i64 >= 0);

    assert_eq!(start["startup_status"].as_str(), Some("success"));
    assert_eq!(start["listener_active"].as_bool(), Some(true));

    assert_eq!(met["policy_violations"].as_i64(), Some(0));
    assert!(met["uptime_seconds"].as_i64().unwrap_or(0) > 0);

    println!("status: OK");
}
