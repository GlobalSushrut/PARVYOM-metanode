use std::collections::HashMap;

use bpi_core::agi_digital_nation_storage::{
    AgiDigitalNationStorage,
    DataQuery,
    QueryType,
    SortOrder,
    DataClassification,
    AccessLevel,
    ArchivalStrategy,
};
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

#[tokio::test(flavor = "current_thread")]
async fn bpi_core_19_agi_digital_nation_storage_integrity() {
    println!("=== Test: BPI-CORE-19: AgiDigitalNation storage integrity ===");

    // 1. Initialize the advanced storage engine
    let storage = AgiDigitalNationStorage::new()
        .await
        .expect("failed to initialize AgiDigitalNationStorage");

    // 2. Construct a synthetic "citizen" record as real app data
    let app_id = Uuid::new_v4();
    let citizen_id = Uuid::new_v4();

    let citizen_record = json!({
        "record_type": "citizen",
        "citizen_id": citizen_id.to_string(),
        "name": "Test Citizen Alpha",
        "created_at": Utc::now().to_rfc3339(),
        "attributes": {
            "tier": "founding",
            "region": "test-region-1",
            "roles": ["validator", "governance"],
        }
    });

    let store_result = storage
        .store_app_data(app_id, citizen_record.clone())
        .await
        .expect("failed to store citizen record");

    println!("stored_citizen_record:");
    println!("  app_id: {}", app_id);
    println!("  citizen_id: {}", citizen_id);
    println!("  storage_id: {}", store_result.storage_id);
    println!("  quantum_storage_id: {}", store_result.quantum_storage_id);
    println!("  persistence_id: {}", store_result.persistence_id);
    println!("  success: {}", store_result.success);
    println!("  timestamp: {}", store_result.timestamp);

    // 3. Build a simple query to retrieve app data
    let mut filters = HashMap::new();
    filters.insert("record_type".to_string(), "citizen".to_string());

    let query = DataQuery {
        query_id: Uuid::new_v4(),
        query_type: QueryType::Simple,
        filters,
        sort_order: SortOrder::Temporal,
        limit: Some(1),
        offset: Some(0),
    };

    let retrieval = storage
        .retrieve_app_data(app_id, query)
        .await
        .expect("failed to retrieve citizen record");

    println!("retrieved_data:");
    println!("  raw: {}", retrieval.data);
    println!("  metadata:");
    println!("    data_type: {}", retrieval.metadata.data_type);
    println!("    classification: {:?}", retrieval.metadata.classification);
    println!("    access_level: {:?}", retrieval.metadata.access_level);
    println!(
        "    retention_years: {}",
        retrieval.metadata.retention_policy.retention_years
    );
    println!(
        "    archival_strategy: {:?}",
        retrieval.metadata.retention_policy.archival_strategy
    );
    println!(
        "    quantum_preservation: {}",
        retrieval.metadata.retention_policy.quantum_preservation
    );

    println!("verification:");
    println!("  verified: {}", retrieval.verification.verified);
    println!(
        "  coherence_level: {:.3}",
        retrieval.verification.coherence_level
    );
    println!(
        "  entanglement_strength: {:.3}",
        retrieval.verification.entanglement_strength
    );
    println!("  integrity_hash: {}", retrieval.verification.integrity_hash);

    // 4. Core integrity invariants
    assert!(store_result.success, "expected store_app_data to succeed");

    assert!(
        retrieval.verification.verified,
        "expected quantum integrity verification to succeed",
    );

    assert!(
        (0.0..=1.0).contains(&retrieval.verification.coherence_level),
        "coherence_level out of range: {}",
        retrieval.verification.coherence_level,
    );

    assert!(
        (0.0..=1.0).contains(&retrieval.verification.entanglement_strength),
        "entanglement_strength out of range: {}",
        retrieval.verification.entanglement_strength,
    );

    assert!(
        retrieval.metadata.retention_policy.retention_years >= 50,
        "expected retention_years to be long-term (>= 50), got {}",
        retrieval.metadata.retention_policy.retention_years,
    );

    // Storage tier sanity: the archival strategy and access level should be
    // drawn from the defined enums, and quantum_preservation should be a
    // boolean flag we can trust.
    match retrieval.metadata.access_level {
        AccessLevel::Public
        | AccessLevel::Citizen
        | AccessLevel::Government
        | AccessLevel::Agi
        | AccessLevel::QuantumSecured
        | AccessLevel::TopSecret => {}
    }

    match retrieval.metadata.retention_policy.archival_strategy {
        ArchivalStrategy::StandardArchival
        | ArchivalStrategy::QuantumArchival
        | ArchivalStrategy::DistributedArchival
        | ArchivalStrategy::ImmutableArchival
        | ArchivalStrategy::AgiPreservation => {}
    }

    println!("summary:");
    println!("  app_id: {}", app_id);
    println!("  citizen_id: {}", citizen_id);
    println!("  verified: {}", retrieval.verification.verified);
    println!(
        "  coherence_level: {:.3}",
        retrieval.verification.coherence_level
    );
    println!(
        "  entanglement_strength: {:.3}",
        retrieval.verification.entanglement_strength
    );
    println!(
        "  retention_years: {}",
        retrieval.metadata.retention_policy.retention_years
    );
    println!(
        "  archival_strategy: {:?}",
        retrieval.metadata.retention_policy.archival_strategy
    );
    println!("status: OK");
}
