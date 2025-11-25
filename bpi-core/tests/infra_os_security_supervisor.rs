use bpi_core::os_security_supervisor::OsSecuritySupervisor;
use uuid::Uuid;

#[tokio::test(flavor = "current_thread")]
async fn bpi_core_16_security_supervisor_firewall_hook() {
    println!("=== Test: BPI-CORE-16: Security supervisor firewall hook ===");

    // Use a unique temporary audit root for this test run
    let audit_root = format!(
        "/tmp/bpi_core_os_security_supervisor_test_{}",
        Uuid::new_v4()
    );

    let supervisor = OsSecuritySupervisor::new(&audit_root, "infra_os_security_test_profile", "test-node-1")
        .await
        .expect("failed to initialize OsSecuritySupervisor");

    // 1. Record a kernel boot event into immutable audit
    supervisor.record_kernel_boot_event().await;

    // 2. Pass a few synthetic HTTP cage-style requests through the supervisor
    supervisor
        .check_http_request("GET", "/health", "127.0.0.1")
        .await;

    supervisor
        .check_http_request_with_source(
            "http_cage",
            "POST",
            "/api/v1/admin/critical/config",
            "203.0.113.42",
        )
        .await;

    supervisor
        .check_http_request_with_source(
            "http_cage",
            "GET",
            "/api/v1/secure/resource",
            "198.51.100.7",
        )
        .await;

    // 3. Retrieve aggregated security-audit metrics from the unified engine
    let metrics = supervisor
        .get_security_audit_metrics()
        .await
        .expect("failed to get security audit metrics");

    println!("security_audit_metrics:");
    println!("  total_security_events: {}", metrics.total_security_events);
    println!("  audit_records_created: {}", metrics.audit_records_created);
    println!(
        "  forensic_evidence_collected: {}",
        metrics.forensic_evidence_collected
    );
    println!("  compliance_violations: {}", metrics.compliance_violations);
    println!(
        "  incident_response_time_avg_ms: {:.3}",
        metrics.incident_response_time_avg_ms
    );
    println!(
        "  threat_detection_accuracy: {:.3}",
        metrics.threat_detection_accuracy
    );
    println!("  false_positive_rate: {:.3}", metrics.false_positive_rate);

    println!("  events_by_component:");
    for (component, count) in &metrics.events_by_component {
        println!("    - component={} count={}", component, count);
    }

    println!("  events_by_severity:");
    for (severity, count) in &metrics.events_by_severity {
        println!("    - severity={} count={}", severity, count);
    }

    // Invariants for this infra preview
    // 1. At least one security event should have been processed
    assert!(
        metrics.total_security_events >= 1,
        "expected at least one security event after HTTP requests",
    );

    // 2. The reported audit_records_created should be consistent with total events
    assert!(
        metrics.audit_records_created >= metrics.total_security_events,
        "audit_records_created ({}) must be >= total_security_events ({})",
        metrics.audit_records_created,
        metrics.total_security_events,
    );

    // 3. Threat detection accuracy and false positive rate should be within [0.0, 1.0]
    assert!(
        (0.0..=1.0).contains(&metrics.threat_detection_accuracy),
        "threat_detection_accuracy out of expected range: {}",
        metrics.threat_detection_accuracy,
    );
    assert!(
        (0.0..=1.0).contains(&metrics.false_positive_rate),
        "false_positive_rate out of expected range: {}",
        metrics.false_positive_rate,
    );

    println!("status: OK");
}
