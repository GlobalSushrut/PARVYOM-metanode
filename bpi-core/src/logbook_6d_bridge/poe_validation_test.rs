// PoE and Quantum Proof Validation Test
// Tests the enhanced logbook_6d_bridge with real PoE root and quantum proofs

use super::*;
use crate::logbook_6d_bridge::logbook_reader::{
    LogbookEntry, LogbookEntryType, OperationData, ExecutionContext, SideEffect, AuditTrail, 
    SecurityContext, ResourceUsage, PerformanceMetrics, EncryptionInfo, LatencyPercentiles
};
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::test]
async fn test_poe_root_generation() {
    // Create a test logbook entry
    let test_entry = create_test_logbook_entry();
    
    // Create converter
    let converter = LogbookTo6DConverter::new().await.expect("Failed to create converter");
    
    // Test PoE root calculation
    let poe_root = converter.calculate_poe_tree_root(&test_entry).await.expect("Failed to calculate PoE root");
    
    // Validate PoE root
    assert!(!poe_root.is_empty(), "PoE root should not be empty");
    assert!(poe_root.len() >= 32, "PoE root should be at least 32 characters (hex hash)");
    println!("✅ PoE root generated successfully: {}", poe_root);
}

#[tokio::test]
async fn test_quantum_proof_generation() {
    // Create a test logbook entry
    let test_entry = create_test_logbook_entry();
    
    // Create converter
    let converter = LogbookTo6DConverter::new().await.expect("Failed to create converter");
    
    // Test VM audit proof generation
    let vm_audit_proof = converter.generate_vm_audit_proof(&test_entry).await.expect("Failed to generate VM audit proof");
    
    // Validate VM audit proof
    assert!(!vm_audit_proof.is_empty(), "VM audit proof should not be empty");
    
    // Parse as JSON to validate structure
    let proof_json: serde_json::Value = serde_json::from_str(&vm_audit_proof).expect("VM audit proof should be valid JSON");
    
    // Validate required fields
    assert!(proof_json["vm_instance_id"].is_string(), "VM instance ID should be present");
    assert!(proof_json["quantum_proof"].is_string(), "Quantum proof should be present");
    assert!(proof_json["quantum_signature"].is_string(), "Quantum signature should be present");
    assert!(proof_json["truthfulness_score"].is_number(), "Truthfulness score should be present");
    
    println!("✅ VM audit proof generated successfully: {} bytes", vm_audit_proof.len());
}

#[tokio::test]
async fn test_traversal_report_generation() {
    // Create a test logbook entry
    let test_entry = create_test_logbook_entry();
    
    // Create converter
    let converter = LogbookTo6DConverter::new().await.expect("Failed to create converter");
    
    // Test traversal report generation
    let traversal_report = converter.generate_traversal_report(&test_entry).await.expect("Failed to generate traversal report");
    
    // Validate traversal report
    assert!(!traversal_report.is_empty(), "Traversal report should not be empty");
    
    // Parse as JSON to validate structure
    let report_json: serde_json::Value = serde_json::from_str(&traversal_report).expect("Traversal report should be valid JSON");
    
    // Validate required fields
    assert!(report_json["entry_id"].is_string(), "Entry ID should be present");
    assert!(report_json["quantum_signature"].is_string(), "Quantum signature should be present");
    assert!(report_json["traversal_path"].is_array(), "Traversal path should be present");
    assert!(report_json["verification_nodes"].is_array(), "Verification nodes should be present");
    
    println!("✅ Traversal report generated successfully: {} bytes", traversal_report.len());
}

#[tokio::test]
async fn test_full_6d_transaction_enhancement() {
    // Create a test logbook entry
    let test_entry = create_test_logbook_entry();
    
    // Create converter
    let converter = LogbookTo6DConverter::new().await.expect("Failed to create converter");
    
    // Convert entry to 6D transaction (this will include PoE and quantum enhancements)
    let transaction = converter.convert_entry_to_6d_transaction(&test_entry).await.expect("Failed to convert entry to 6D transaction");
    
    // Validate enhanced transaction
    assert!(transaction.poe_tree_root.is_some(), "PoE tree root should be present");
    assert!(transaction.traversal_report.is_some(), "Traversal report should be present");
    assert!(transaction.vm_audit_proof.is_some(), "VM audit proof should be present");
    
    let poe_root = transaction.poe_tree_root.unwrap();
    let traversal_report = transaction.traversal_report.unwrap();
    let vm_audit_proof = transaction.vm_audit_proof.unwrap();
    
    assert!(!poe_root.is_empty(), "PoE root should not be empty");
    assert!(!traversal_report.is_empty(), "Traversal report should not be empty");
    assert!(!vm_audit_proof.is_empty(), "VM audit proof should not be empty");
    
    println!("✅ Full 6D transaction enhancement successful:");
    println!("   - PoE root: {} chars", poe_root.len());
    println!("   - Traversal report: {} bytes", traversal_report.len());
    println!("   - VM audit proof: {} bytes", vm_audit_proof.len());
}

fn create_test_logbook_entry() -> LogbookEntry {
    LogbookEntry {
        entry_id: "test_entry_001".to_string(),
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        entry_type: LogbookEntryType::VMOperation,
        vm_instance_id: "vm_test_001".to_string(),
        operation_data: OperationData {
            operation_id: "op_001".to_string(),
            operation_type: "quantum_computation".to_string(),
            input_data_hash: "sha256:1234567890abcdef1234567890abcdef12345678".to_string(),
            output_data_hash: "sha256:fedcba0987654321fedcba0987654321fedcba09".to_string(),
            execution_context: ExecutionContext {
                execution_environment: "test_environment".to_string(),
                user_context: Some("test_user".to_string()),
                session_id: Some("session_001".to_string()),
                request_id: Some("req_001".to_string()),
                parent_operation_id: Some("parent_op_001".to_string()),
            },
            dependencies: vec!["dep_001".to_string(), "dep_002".to_string()],
            side_effects: vec![
                SideEffect {
                    effect_type: "file_write".to_string(),
                    affected_resource: "/tmp/output.txt".to_string(),
                    change_description: "Created output file".to_string(),
                    rollback_info: Some("Delete /tmp/output.txt".to_string()),
                }
            ],
        },
        audit_trail: AuditTrail {
            audit_id: "trail_001".to_string(),
            compliance_tags: vec!["quantum_secure".to_string(), "poe_validated".to_string()],
            regulatory_requirements: vec!["GDPR".to_string(), "SOX".to_string()],
            evidence_chain: vec![],
            witness_signatures: vec![],
        },
        security_context: SecurityContext {
            security_level: "high".to_string(),
            access_controls: vec!["read".to_string(), "write".to_string()],
            encryption_info: EncryptionInfo {
                algorithm: "AES-256-GCM".to_string(),
                key_id: "key_001".to_string(),
                initialization_vector: "iv_001".to_string(),
                encryption_strength: 256,
            },
            authentication_proof: "quantum_signature_proof".to_string(),
            authorization_proof: "authorization_token_001".to_string(),
        },
        resource_usage: ResourceUsage {
            cpu_time_ms: 100,
            memory_peak_mb: 1024,
            storage_bytes: 512000,
            network_bytes: 256000,
            gpu_time_ms: 50,
            quantum_operations: 10,
        },
        performance_metrics: PerformanceMetrics {
            execution_time_ms: 100,
            throughput_ops_per_sec: 1000.0,
            latency_percentiles: LatencyPercentiles {
                p50_ms: 10.0,
                p90_ms: 25.0,
                p95_ms: 35.0,
                p99_ms: 50.0,
            },
            error_rate: 0.01,
            availability: 0.999,
        },
        integrity_hash: "sha256:abcdef1234567890abcdef1234567890abcdef12".to_string(),
    }
}
