use bpi_core::pravyom_integration::PravyomConfig;
use bpi_core::pravyom_integration::segment_threshold_manager::SealedSegmentMeta;
use bpi_core::pravyom_integration::summary_ticket_generator::SummaryTicketGenerator;
use chrono::{Duration as ChronoDuration, Utc};
use pravyom_pipeline::{
    IoUsage, NetworkRollup, PoeRollup, SecurityRollup, SegmentRef, SummaryTicket,
    SystemRollup, SystemTotals, VmRollup,
};
use std::collections::BTreeMap;

#[tokio::test(flavor = "current_thread")]
async fn bpi_core_29_pravyom_summary_ticket_preview() {
    println!("=== Test: BPI-CORE-29: Pravyom integration summary ticket ===");

    // Build a default PravyomConfig (in-memory only)
    let config = PravyomConfig::default();

    println!("storage_path: {}", config.storage_path);
    println!("bpi_endpoint: {}", config.bpi_endpoint);
    println!("bpci_endpoint: {}", config.bpci_endpoint);

    // Create a small synthetic sealed segment meta
    let now = Utc::now();
    let later = now + ChronoDuration::seconds(60);

    let segment_id = "SEG-TEST-001".to_string();
    let merkle_root = "demo_merkle_root_123".to_string();
    let record_count = 42usize;

    // Single VM rollup for this test
    let vm_rollup = VmRollup {
        vmid: "vm-app-001".to_string(),
        records: record_count as u64,
        cpu_ms: 1234.0,
        ram_kb: 2048,
        io: IoUsage { r: 10_000, w: 5_000 },
        net: NetworkRollup { flows: 3 },
        seg: SegmentRef {
            id: segment_id.clone(),
            root: merkle_root.clone(),
        },
    };

    let mut vm_rollups = BTreeMap::new();
    vm_rollups.insert("BpiActionVm".to_string(), vm_rollup.clone());

    let system_rollup = SystemRollup {
        totals: SystemTotals {
            records: record_count as u64,
            cpu_ms: vm_rollup.cpu_ms,
            ram_kb_avg: vm_rollup.ram_kb,
        },
        sec: SecurityRollup {
            allow: record_count as u32,
            deny: 0,
            qlock_events: 0,
        },
        poe: PoeRollup {
            exec_count: 1,
            ready_for_poe_bundle: true,
        },
        anomaly: None,
    };

    let segment_meta = SealedSegmentMeta {
        created_at: now,
        first_record_id: "R-TEST-0001".to_string(),
        last_record_id: "R-TEST-0042".to_string(),
        merkle_root: merkle_root.clone(),
        record_count,
        sealed_at: later,
        segment_id: segment_id.clone(),
        system_rollup,
        vm_rollups,
    };

    println!("segment_id: {}", segment_meta.segment_id);
    println!("segment_record_count: {}", segment_meta.record_count);
    println!("segment_created_at: {}", segment_meta.created_at.to_rfc3339());
    println!("segment_sealed_at: {}", segment_meta.sealed_at.to_rfc3339());
    println!("segment_merkle_root: {}", segment_meta.merkle_root);

    let mut generator = SummaryTicketGenerator::new(&config)
        .expect("failed to create SummaryTicketGenerator");

    let ticket: SummaryTicket = generator
        .create_summary_ticket(&segment_meta)
        .await
        .expect("failed to create summary ticket");

    println!("ticket_id: {}", ticket.ticket_id);
    println!(
        "ticket_window: from={} to={}",
        ticket.window.from.to_rfc3339(),
        ticket.window.to.to_rfc3339(),
    );
    println!(
        "ticket_policy: threshold={}, vm_count={}",
        ticket.policy.threshold, ticket.policy.vm_count
    );

    println!("vm_rollup_count: {}", ticket.vm_rollup.len());
    for vr in &ticket.vm_rollup {
        println!(
            "- vm_rollup: vmid={}, records={}, cpu_ms={:.2}, ram_kb={}, io_r={}, io_w={}, flows={}, seg_id={}, seg_root={}",
            vr.vmid,
            vr.records,
            vr.cpu_ms,
            vr.ram_kb,
            vr.io.r,
            vr.io.w,
            vr.net.flows,
            vr.seg.id,
            vr.seg.root,
        );
    }

    println!(
        "system_totals: records={}, cpu_ms={:.2}, ram_kb_avg={}",
        ticket.system.totals.records,
        ticket.system.totals.cpu_ms,
        ticket.system.totals.ram_kb_avg,
    );
    println!(
        "system_security: allow={}, deny={}, qlock_events={}",
        ticket.system.sec.allow,
        ticket.system.sec.deny,
        ticket.system.sec.qlock_events,
    );
    println!(
        "system_poe: exec_count={}, ready_for_poe_bundle={}",
        ticket.system.poe.exec_count,
        ticket.system.poe.ready_for_poe_bundle,
    );

    println!(
        "roots: vm_merkle={}, ziplock_super_root={}",
        ticket.roots.vm_merkle,
        ticket.roots.ziplock_super_root,
    );
    println!(
        "anchors: previous_ticket={}, bpi_tip_hint={}",
        ticket.anchors.previous_ticket,
        ticket.anchors.bpi_tip_hint,
    );
    println!(
        "signatures: bls={}, pqc_multi_count={}",
        ticket.sig.bls,
        ticket.sig.pqc_multi.len(),
    );

    // Invariants
    assert_eq!(ticket.vm_rollup.len(), 1, "expected exactly one VM rollup in this test");
    assert_eq!(ticket.system.totals.records, record_count as u64);
    assert_eq!(ticket.roots.vm_merkle, merkle_root);

    // The ticket generator should consider this a valid ticket
    let is_valid = generator
        .validate_ticket(&ticket)
        .expect("ticket validation failed");
    println!("ticket_validate_result: {}", is_valid);
    assert!(is_valid, "summary ticket should pass internal validation");

    println!("status: OK");
}
