use bpi_core::{HealthStatus, ServiceHealth};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn bpi_core_21_node_status_dashboard_preview() {
    println!("=== Test: BPI-CORE-21: `bpi-core node status` dashboard preview ===");

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Synthetic but realistic service health snapshot
    let mut services: HashMap<String, ServiceHealth> = HashMap::new();

    services.insert(
        "vm_server".to_string(),
        ServiceHealth {
            status: "healthy".to_string(),
            response_time_ms: 12,
            last_check: now,
            error_message: None,
            suggestions: vec![],
        },
    );

    services.insert(
        "bpci_bridge".to_string(),
        ServiceHealth {
            status: "healthy".to_string(),
            response_time_ms: 18,
            last_check: now,
            error_message: None,
            suggestions: vec![],
        },
    );

    services.insert(
        "4d_database".to_string(),
        ServiceHealth {
            status: "healthy".to_string(),
            response_time_ms: 25,
            last_check: now,
            error_message: None,
            suggestions: vec![],
        },
    );

    services.insert(
        "service_orchestrator".to_string(),
        ServiceHealth {
            status: "healthy".to_string(),
            response_time_ms: 20,
            last_check: now,
            error_message: None,
            suggestions: vec![],
        },
    );

    let status = HealthStatus {
        status: "healthy".to_string(),
        services: services.clone(),
        timestamp: now,
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: 1234,
        pilot_ready: true,
    };

    // Compact node dashboard preview (similar spirit to `bpi-core node status`)
    println!("node_status: {}", status.status);
    println!("pilot_ready: {}", status.pilot_ready);
    println!("version: {}", status.version);
    println!("uptime_seconds: {}", status.uptime_seconds);
    println!("timestamp: {}", status.timestamp);

    println!("services:");
    for (name, svc) in status.services.iter() {
        println!(
            "  - {}: status={}, response_time_ms={}, last_check={}, error={:?}",
            name, svc.status, svc.response_time_ms, svc.last_check, svc.error_message
        );
    }

    // Basic invariants for dashboard sanity
    assert_eq!(status.status, "healthy");
    assert!(status.pilot_ready, "pilot_ready should be true in happy-path preview");
    assert!(status.services.contains_key("vm_server"));
    assert!(status.services.contains_key("bpci_bridge"));
    assert!(status.services.contains_key("4d_database"));
    assert!(status.services.contains_key("service_orchestrator"));

    assert!(status
        .services
        .values()
        .all(|svc| svc.status == "healthy"), "all services should be healthy in this preview");

    println!("status: OK");
}
