use std::collections::HashMap;

use bpi_core::security::soar_engine::{
    SOAREngine,
    SecurityEvent,
    IncidentType,
    IncidentSeverity,
    ClassificationCondition,
    ClassificationRule,
    Playbook,
    PlaybookStep,
    StepType,
    ActionDefinition,
    ExecutionMethod,
    RetryPolicy,
    BackoffStrategy,
};
use chrono::{Duration, Utc};

#[tokio::test(flavor = "current_thread")]
async fn bpi_core_17_soar_engine_incident_classification() {
    println!("=== Test: BPI-CORE-17: SOAR engine incident classification ===");

    let soar = SOAREngine::new();
    soar.start_soar().await.expect("failed to start SOAR engine");

    // 1. Register a couple of playbooks for different incident types/severities
    let mut playbooks = Vec::new();

    playbooks.push(Playbook {
        playbook_id: "pb_unauth_access_low".to_string(),
        playbook_name: "Investigate low-severity unauthorized access".to_string(),
        incident_types: vec![IncidentType::UnauthorizedAccess],
        severity_levels: vec![IncidentSeverity::Low, IncidentSeverity::Medium],
        steps: vec![PlaybookStep {
            step_id: "step_collect_logs".to_string(),
            step_name: "Collect authentication logs".to_string(),
            step_type: StepType::Investigation,
            action: ActionDefinition {
                action_type: "collect_logs".to_string(),
                target_systems: vec!["auth_service".to_string()],
                parameters: HashMap::new(),
                execution_method: ExecutionMethod::API,
            },
            timeout: Duration::minutes(5),
            retry_policy: RetryPolicy {
                max_retries: 1,
                backoff_strategy: BackoffStrategy::Fixed,
            },
        }],
        estimated_duration: Duration::minutes(15),
    });

    playbooks.push(Playbook {
        playbook_id: "pb_data_breach_critical".to_string(),
        playbook_name: "Contain and investigate critical data breach".to_string(),
        incident_types: vec![IncidentType::DataBreach, IncidentType::DataExfiltration],
        severity_levels: vec![IncidentSeverity::High, IncidentSeverity::Critical, IncidentSeverity::Emergency],
        steps: vec![PlaybookStep {
            step_id: "step_isolate_systems".to_string(),
            step_name: "Isolate affected systems".to_string(),
            step_type: StepType::Containment,
            action: ActionDefinition {
                action_type: "isolate_systems".to_string(),
                target_systems: vec!["production_cluster".to_string()],
                parameters: HashMap::new(),
                execution_method: ExecutionMethod::API,
            },
            timeout: Duration::minutes(10),
            retry_policy: RetryPolicy {
                max_retries: 2,
                backoff_strategy: BackoffStrategy::Exponential,
            },
        }],
        estimated_duration: Duration::minutes(60),
    });

    for pb in playbooks {
        soar
            .register_playbook(pb)
            .await
            .expect("failed to register playbook");
    }

    // 2. Add simple classification rules to map attributes -> incidents
    let rules = vec![
        ClassificationRule {
            rule_id: "rule_unauth_admin".to_string(),
            conditions: vec![
                ClassificationCondition {
                    field: "event_type".to_string(),
                    operator: "equals".to_string(),
                    value: "auth_failure".to_string(),
                    weight: 0.6,
                },
                ClassificationCondition {
                    field: "resource".to_string(),
                    operator: "contains".to_string(),
                    value: "/admin".to_string(),
                    weight: 0.4,
                },
            ],
            incident_type: IncidentType::UnauthorizedAccess,
            severity: IncidentSeverity::Medium,
            confidence_score: 0.9,
        },
        ClassificationRule {
            rule_id: "rule_data_exfil".to_string(),
            conditions: vec![
                ClassificationCondition {
                    field: "event_type".to_string(),
                    operator: "equals".to_string(),
                    value: "large_download".to_string(),
                    weight: 0.7,
                },
                ClassificationCondition {
                    field: "bytes".to_string(),
                    operator: "contains".to_string(),
                    value: "100000000".to_string(),
                    weight: 0.3,
                },
            ],
            incident_type: IncidentType::DataExfiltration,
            severity: IncidentSeverity::High,
            confidence_score: 0.95,
        },
    ];

    for rule in rules {
        soar
            .add_classification_rule(rule)
            .await
            .expect("failed to register classification rule");
    }

    // 3. Create synthetic security events
    let mut events = Vec::new();

    let mut attrs1 = HashMap::new();
    attrs1.insert("event_type".to_string(), "auth_failure".to_string());
    attrs1.insert("resource".to_string(), "/admin/login".to_string());
    attrs1.insert("user".to_string(), "alice".to_string());

    events.push(SecurityEvent {
        event_id: "ev1".to_string(),
        event_type: "login_failure".to_string(),
        timestamp: Utc::now(),
        source: "auth_service".to_string(),
        attributes: attrs1,
    });

    let mut attrs2 = HashMap::new();
    attrs2.insert("event_type".to_string(), "large_download".to_string());
    attrs2.insert("resource".to_string(), "/data/exports/report.csv".to_string());
    attrs2.insert("bytes".to_string(), "100000000".to_string());

    events.push(SecurityEvent {
        event_id: "ev2".to_string(),
        event_type: "file_download".to_string(),
        timestamp: Utc::now(),
        source: "storage_gateway".to_string(),
        attributes: attrs2,
    });

    let mut attrs3 = HashMap::new();
    attrs3.insert("event_type".to_string(), "auth_failure".to_string());
    attrs3.insert("resource".to_string(), "/app/profile".to_string());
    attrs3.insert("user".to_string(), "bob".to_string());

    events.push(SecurityEvent {
        event_id: "ev3".to_string(),
        event_type: "login_failure".to_string(),
        timestamp: Utc::now(),
        source: "auth_service".to_string(),
        attributes: attrs3,
    });

    println!("incidents:");

    let mut classified = Vec::new();

    for ev in &events {
        let (incident_type, severity, confidence) = soar
            .classify_incident(ev)
            .await
            .expect("classification failed");

        let playbook_id = soar
            .suggest_playbook(&incident_type, &severity)
            .await;

        if let Some(ref pb_id) = playbook_id {
            // Fire the playbook execution to exercise the engine
            let _execution_id = soar
                .execute_playbook(pb_id, &ev.event_id)
                .await
                .expect("playbook execution failed");
        }

        println!(
            "  - event_id={} type={} source={} incident={:?} severity={:?} confidence={:.3} playbook={}",
            ev.event_id,
            ev.event_type,
            ev.source,
            incident_type,
            severity,
            confidence,
            playbook_id.clone().unwrap_or_else(|| "<none>".to_string()),
        );

        classified.push((incident_type, severity, confidence, playbook_id));
    }

    // 4. Basic invariants
    assert!(
        !classified.is_empty(),
        "expected at least one classified incident",
    );

    // Ensure confidence scores are within [0.0, 1.0]
    for (_, _, confidence, _) in &classified {
        assert!(
            (0.0..=1.0).contains(confidence),
            "confidence out of range: {}",
            confidence,
        );
    }

    // At least one incident should have a non-Unknown type and non-Info severity
    let mut has_meaningful_incident = false;
    let mut with_playbook = 0usize;

    for (incident_type, severity, _, playbook_id) in &classified {
        if !matches!(incident_type, IncidentType::Unknown)
            && !matches!(severity, IncidentSeverity::Info)
        {
            has_meaningful_incident = true;
        }
        if playbook_id.is_some() {
            with_playbook += 1;
        }
    }

    assert!(
        has_meaningful_incident,
        "expected at least one non-trivial incident classification",
    );

    assert!(
        with_playbook >= 1,
        "expected at least one incident to have a suggested playbook",
    );

    println!("summary:");
    println!("  total_events: {}", classified.len());
    println!("  incidents_with_playbooks: {}", with_playbook);
    println!("status: OK");
}
