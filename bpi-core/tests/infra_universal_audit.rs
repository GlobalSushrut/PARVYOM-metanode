use std::collections::{BTreeMap, HashSet};
use std::sync::Arc;

use bpi_core::ImmutableAuditSystem;
use bpi_core::universal_audit_vm::{
    UniversalAuditVM,
    AuditAggregation,
    AuditVMStatusReport,
    AggregatedEvent,
};
use tokio;
use uuid::Uuid;

#[tokio::test(flavor = "current_thread")]
async fn bpi_core_11_universal_audit_multi_component_view() {
    println!("=== Test: BPI-CORE-11: Universal audit VM multi-component view ===");

    // Isolated immutable audit storage under /tmp
    let base = std::env::temp_dir();
    let storage_str = base
        .join(format!("bpi_universal_audit_test_{}", Uuid::new_v4()))
        .to_string_lossy()
        .to_string();

    let audit_system = ImmutableAuditSystem::new(&storage_str)
        .await
        .expect("failed to initialize ImmutableAuditSystem for universal audit VM");
    let audit_system = Arc::new(audit_system);

    // Initialize and start the Universal Audit VM
    let audit_vm = UniversalAuditVM::new(audit_system.clone())
        .await
        .expect("failed to initialize UniversalAuditVM");

    audit_vm
        .start()
        .await
        .expect("failed to start UniversalAuditVM");

    // Perform a single 1-minute aggregation over synthetic monitor data
    let aggregation_id = audit_vm
        .perform_audit_aggregation()
        .await
        .expect("failed to perform audit aggregation");

    println!("aggregation_id: {}", aggregation_id);

    // Fetch VM status and print a small dashboard
    let status: AuditVMStatusReport = audit_vm
        .get_audit_vm_status()
        .await
        .expect("failed to get audit VM status");

    println!("audit_vm_status:");
    println!("  vm_id: {}", status.vm_state.vm_id);
    println!("  state: {:?}", status.vm_state.status);
    println!("  active_monitors: {}", status.vm_state.active_monitors);
    println!(
        "  audit_events_per_minute: {:.1}",
        status.vm_state.audit_events_per_minute
    );
    println!("  compliance_score: {:.1}", status.vm_state.compliance_score);
    println!("  total_aggregations: {}", status.total_aggregations);
    println!("  compliance_reports: {}", status.compliance_reports);

    // Fetch the aggregation record we just created
    let aggregation: AuditAggregation = audit_vm
        .get_audit_aggregation(&aggregation_id)
        .await
        .expect("expected aggregation to exist for given ID");

    println!("universal_audit_aggregation_summary:");
    println!(
        "  window: {:?} to {:?} ({:?})",
        aggregation.time_window.start_time,
        aggregation.time_window.end_time,
        aggregation.time_window.window_type
    );
    println!("  source_components: {:?}", aggregation.source_components);
    println!("  aggregated_events_count: {}", aggregation.aggregated_events.len());
    println!("  event_count_field: {}", aggregation.event_count);
    println!("  proof_hash: {}", aggregation.proof_hash);

    // Build a compact per-component view from aggregated events
    let mut per_component_events: BTreeMap<String, u64> = BTreeMap::new();

    for event in &aggregation.aggregated_events {
        let entry = per_component_events
            .entry(event.component.clone())
            .or_insert(0);
        *entry += event.count;
    }

    println!("universal_audit_component_table:");
    for (component, count) in &per_component_events {
        println!("  - component: {} | total_events: {}", component, count);
    }

    println!("universal_audit_events_detail:");
    for AggregatedEvent {
        event_type,
        component,
        count,
        severity_distribution,
        first_occurrence,
        last_occurrence,
    } in &aggregation.aggregated_events
    {
        let mut severities: Vec<String> = severity_distribution
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect();
        severities.sort();

        println!(
            "  - component: {} | type: {} | count: {} | severities: {} | first: {:?} | last: {:?}",
            component,
            event_type,
            count,
            severities.join(","),
            first_occurrence,
            last_occurrence
        );
    }

    // Basic invariants for this infra preview
    let expected_components: HashSet<&str> = [
        "ActionVM",
        "HttpCage",
        "ForensicVM",
        "OrchestrationVM",
        "ShadowRegistry",
    ]
    .into_iter()
    .collect();

    let present_components: HashSet<String> = aggregation
        .aggregated_events
        .iter()
        .map(|e| e.component.clone())
        .collect();

    for comp in &expected_components {
        assert!(
            present_components.contains(&comp.to_string()),
            "expected component {} to appear in aggregated view",
            comp
        );
    }

    assert_eq!(
        aggregation.aggregated_events.len(),
        expected_components.len(),
        "each monitor should contribute exactly one aggregated event in this preview",
    );

    assert!(
        aggregation.event_count as usize >= aggregation.aggregated_events.len(),
        "event_count field should be at least the number of aggregated event rows",
    );

    assert_eq!(
        status.total_aggregations, 1,
        "expected exactly one aggregation to have been recorded",
    );

    println!("status: OK");
}
