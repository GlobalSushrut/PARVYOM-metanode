use std::collections::HashMap;

use bpi_core::security::deception_technology::{
    DeceptionEngine,
    DeceptionInteraction,
    DeceptionType,
};
use chrono::Utc;
use uuid::Uuid;

#[tokio::test(flavor = "current_thread")]
async fn bpi_core_18_deception_honeyfile_trigger() {
    println!("=== Test: BPI-CORE-18: Deception technology honeyfile trigger ===");

    // 1. Initialize deception engine and start systems
    let engine = DeceptionEngine::new();
    engine
        .start_deception()
        .await
        .expect("failed to start deception engine");

    // 2. Create a synthetic honeyfile at a tempting path
    let target_path = "/tmp/bpi_core_deception/honeyfiles/confidential_data.xlsx";
    let honeyfile_id = engine
        .create_honeyfile("template_finance_report", target_path)
        .await
        .expect("failed to create honeyfile");

    println!("created_honeyfile:");
    println!("  honeyfile_id: {}", honeyfile_id);
    println!("  file_path: {}", target_path);

    // 3. Simulate an attacker reading the honeyfile
    let interaction_id = Uuid::new_v4().to_string();
    let mut details = HashMap::new();
    details.insert("file_path".to_string(), target_path.to_string());
    details.insert("access_type".to_string(), "read".to_string());
    details.insert("node".to_string(), "vm-server-1".to_string());

    let interaction = DeceptionInteraction {
        interaction_id: interaction_id.clone(),
        deception_type: DeceptionType::Honeyfile,
        target_id: honeyfile_id.clone(),
        timestamp: Utc::now(),
        source_ip: "203.0.113.42".to_string(),
        user_agent: Some("curl/8.0 attacker-probe".to_string()),
        interaction_details: details,
        threat_indicators: vec![
            "suspicious_read".to_string(),
            "high_value_path".to_string(),
        ],
    };

    let alerts = engine
        .analyze_interaction(&interaction)
        .await
        .expect("failed to analyze deception interaction");

    println!("alerts:");
    for alert in &alerts {
        println!("  - alert_id: {}", alert.alert_id);
        println!("    type: {}", alert.alert_type);
        println!("    severity: {:?}", alert.severity);
        println!("    timestamp: {}", alert.timestamp);
        println!("    source_interaction: {}", alert.source_interaction);
        println!("    description: {}", alert.description);
        println!("    indicators: {:?}", alert.indicators);
        println!("    recommended_actions:");
        for action in &alert.recommended_actions {
            println!("      * {}", action);
        }
    }

    // 4. Core invariants
    // At least one alert should be generated for the honeyfile access
    assert!(
        !alerts.is_empty(),
        "expected at least one deception alert for honeyfile access",
    );

    // The alert should be tied to the interaction we created
    let mut matched_interaction = false;
    let mut non_empty_actions = 0usize;

    for alert in &alerts {
        if alert.source_interaction == interaction_id {
            matched_interaction = true;
        }
        if !alert.recommended_actions.is_empty() {
            non_empty_actions += 1;
        }
    }

    assert!(
        matched_interaction,
        "expected at least one alert to reference the simulated interaction",
    );

    assert!(
        non_empty_actions >= 1,
        "expected at least one alert with recommended actions",
    );

    println!("summary:");
    println!("  alerts_generated: {}", alerts.len());
    println!("  alerts_with_recommended_actions: {}", non_empty_actions);
    println!("status: OK");
}
