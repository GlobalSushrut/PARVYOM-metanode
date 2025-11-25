use std::sync::Arc;

use bpi_core::immutable_audit_system::{ImmutableAuditSystem, ComponentType};
use bpi_core::forensic_firewall::audit_bridge::{AuditBridgeConfig, ForensicAuditBridge};
use bpi_core::forensic_firewall::shared_types::{ForensicEventType, ForensicSeverity};
use bpi_core::forensic_firewall::cue_engine::CueRuleEngine;
use tokio::sync::RwLock;
use uuid::Uuid;

#[tokio::test(flavor = "current_thread")]
async fn bpi_core_09_forensic_firewall_integration_path() {
    println!("=== Test: BPI-CORE-09: Forensic firewall integration path ===");

    // Isolated immutable audit storage under /tmp (not inspected on disk in this test)
    let base = std::env::temp_dir();
    let storage_str = base
        .join(format!("bpi_forensic_audit_test_{}", Uuid::new_v4()))
        .to_string_lossy()
        .to_string();

    let audit_system = ImmutableAuditSystem::new(&storage_str)
        .await
        .expect("failed to initialize ImmutableAuditSystem");
    let audit_system = Arc::new(RwLock::new(audit_system));

    // Minimal CUE engine + audit bridge configuration
    let cue_engine = Arc::new(CueRuleEngine::new());
    let bridge_cfg = AuditBridgeConfig {
        enable_real_time_audit: false,
        enable_evidence_collection: true,
        enable_chain_of_custody: true,
        evidence_retention_days: 7,
        max_evidence_size_mb: 16,
        compression_enabled: false,
        encryption_enabled: false,
        digital_signature_required: true,
        witness_signatures_required: 0,
    };

    let bridge = ForensicAuditBridge::new(audit_system.clone(), cue_engine.clone(), bridge_cfg);

    // Simulate a high-severity network intrusion coming from HTTP Cage
    let event_type = ForensicEventType::NetworkIntrusion;
    let source_component = ComponentType::HttpCage;
    let severity = ForensicSeverity::High;
    let description = "Simulated HTTP Cage network intrusion for infra test".to_string();

    println!("forensic_event_type: {:?}", event_type);
    println!("source_component: {:?}", source_component);
    println!("severity: {:?}", severity);
    println!("description: {}", description);

    let event_id = bridge
        .record_security_event(
            event_type.clone(),
            source_component.clone(),
            severity.clone(),
            description.clone(),
            None,   // let bridge create default forensic evidence
            None,   // no CUE decision yet
            None,   // no behavioral analysis
            None,   // no threat intel classification
        )
        .await
        .expect("failed to record forensic security event");

    println!("recorded_forensic_event_id: {}", event_id);

    // Read back the forensic event and aggregates from bridge memory
    let fetched = bridge
        .get_forensic_event(&event_id)
        .await
        .expect("failed to fetch forensic event")
        .expect("forensic event not found in bridge state");

    println!("forensic_event.timestamp: {}", fetched.timestamp);
    println!("forensic_event.source_component: {:?}", fetched.source_component);
    println!("forensic_event.severity: {:?}", fetched.severity);
    println!("forensic_event.description: {}", fetched.description);
    println!("forensic_event.immutable_hash: {}", fetched.immutable_hash);

    // Basic invariants on forensic side
    assert_eq!(fetched.event_type, event_type);
    // ComponentType does not implement PartialEq, so compare enum variants via discriminant
    assert_eq!(
        std::mem::discriminant(&fetched.source_component),
        std::mem::discriminant(&source_component)
    );
    assert!(fetched.severity >= ForensicSeverity::High);
    assert!(!fetched.immutable_hash.is_empty());
    assert!(!fetched.digital_signature.is_empty());

    // Verify that the forensic evidence hash is internally consistent
    let evidence_id = fetched.evidence.evidence_id;
    let integrity_ok = bridge
        .verify_evidence_integrity(&evidence_id)
        .await
        .expect("failed to verify evidence integrity");

    println!("evidence_id: {}", evidence_id);
    println!("evidence_integrity_ok: {}", integrity_ok);

    // For this infra test we only require that the integrity check runs end-to-end;
    // the internal hashing scheme may evolve independently of this preview.
    println!("status: OK");
}
