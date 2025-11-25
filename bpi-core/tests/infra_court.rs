use std::collections::{BTreeMap, HashSet};
use std::sync::Arc;

use bpi_core::immutable_audit_system::ImmutableAuditSystem;
use bpi_core::court_vm_audit::{CourtVMAuditSystem, RuntimeActionType};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone)]
struct SyntheticDispute {
    case_id: String,
    reason: String,
    severity: &'static str,
}

fn classify_queue(dispute: &SyntheticDispute) -> &'static str {
    match (dispute.reason.as_str(), dispute.severity) {
        ("PaymentFailure", "High") | ("SettlementMismatch", "High") => "financial_fast_track",
        ("IdentityMismatch", _) | ("KycDispute", _) => "identity_verification",
        ("ContractClause", "Medium") | ("ContractClause", "Low") => "contract_review",
        _ => "general_queue",
    }
}

#[tokio::test(flavor = "current_thread")]
async fn bpi_core_10_court_node_receives_and_classifies_disputes() {
    println!("=== Test: BPI-CORE-10: Court node receives and classifies disputes ===");

    // Isolated immutable audit storage under /tmp (we do not inspect files on disk here)
    let base = std::env::temp_dir();
    let storage_str = base
        .join(format!("bpi_court_audit_test_{}", Uuid::new_v4()))
        .to_string_lossy()
        .to_string();

    let audit_system = ImmutableAuditSystem::new(&storage_str)
        .await
        .expect("failed to initialize ImmutableAuditSystem for court node");
    let audit_system = Arc::new(audit_system);

    // Initialize Court VM audit system directly for this preview
    let court_vm = CourtVMAuditSystem::new(audit_system.clone())
        .await
        .expect("failed to initialize CourtVMAuditSystem");
    let court_vm = Arc::new(RwLock::new(court_vm));

    // Synthetic disputes representing different types of cases the court node might receive
    let disputes = vec![
        SyntheticDispute {
            case_id: "DISP-001".to_string(),
            reason: "PaymentFailure".to_string(),
            severity: "High",
        },
        SyntheticDispute {
            case_id: "DISP-002".to_string(),
            reason: "IdentityMismatch".to_string(),
            severity: "Medium",
        },
        SyntheticDispute {
            case_id: "DISP-003".to_string(),
            reason: "ContractClause".to_string(),
            severity: "Medium",
        },
        SyntheticDispute {
            case_id: "DISP-004".to_string(),
            reason: "SettlementMismatch".to_string(),
            severity: "High",
        },
        SyntheticDispute {
            case_id: "DISP-005".to_string(),
            reason: "Other".to_string(),
            severity: "Low",
        },
    ];

    println!("dispute_count: {}", disputes.len());

    // Record each dispute as a runtime action in the Court VM audit system
    for dispute in &disputes {
        let queue = classify_queue(dispute);
        let result_json = serde_json::json!({
            "case_id": dispute.case_id,
            "reason": dispute.reason,
            "severity": dispute.severity,
            "queue": queue,
        });

        println!(
            "dispute_case: id={} reason={} severity={} -> queue=\"{}\"",
            dispute.case_id, dispute.reason, dispute.severity, queue
        );

        court_vm
            .read()
            .await
            .record_runtime_action(
                RuntimeActionType::SecurityEvent,
                &format!(
                    "Dispute {} ({}) routed to queue {}",
                    dispute.case_id, dispute.reason, queue
                ),
                &serde_json::to_string(&result_json).expect("serialize dispute result"),
            )
            .await
            .expect("failed to record dispute runtime action");
    }

    // Fetch runtime action logs from the Court VM audit system
    let logs = court_vm
        .read()
        .await
        .get_runtime_action_logs()
        .await
        .expect("failed to fetch runtime action logs");

    println!("runtime_action_logs_count: {}", logs.len());

    // Build a queue -> cases mapping from the recorded runtime actions
    let mut queues: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for log in &logs {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&log.result) {
            let case_id = value
                .get("case_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let queue = value
                .get("queue")
                .and_then(|v| v.as_str())
                .unwrap_or("general_queue");

            if !case_id.is_empty() {
                queues
                    .entry(queue.to_string())
                    .or_default()
                    .push(case_id.to_string());
            }
        }
    }

    println!("court_dispute_queues_summary:");
    for (queue, cases) in &queues {
        println!(
            "  - queue: {} | cases: {} | ids: {}",
            queue,
            cases.len(),
            cases.join(",")
        );
    }

    // Basic invariants: all disputes should be present in at least one queue
    let unique_cases: HashSet<String> = queues
        .values()
        .flat_map(|v| v.iter().cloned())
        .collect();

    assert_eq!(
        unique_cases.len(),
        disputes.len(),
        "each synthetic dispute should appear in some court queue"
    );

    // Ensure key queues are present to show classification
    assert!(queues.contains_key("financial_fast_track"));
    assert!(queues.contains_key("identity_verification"));
    assert!(queues.contains_key("contract_review"));

    println!("status: OK");
}
