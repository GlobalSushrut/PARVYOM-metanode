// Integration Test for BPI Logbook to 6D Blockchain Bridge
// Tests the complete end-to-end conversion flow

use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::Result;

use super::*;
use crate::logbook_6d_bridge::{
    LogbookTo6DConverter, BPILogbookReader, SixDBlockchainWriter, ConversionRules,
    LogbookEntry, LogbookEntryType, OperationData, AuditTrail,
    SecurityContext, ResourceUsage, PerformanceMetrics,
    SixDTransaction, TransactionType, DimensionalCoordinates
};
use crate::logbook_6d_bridge::logbook_reader::{ExecutionContext, EncryptionInfo, LatencyPercentiles, SideEffect};

/// Integration test for the complete logbook to 6D blockchain bridge
pub async fn test_complete_bridge_integration() -> Result<()> {
    println!("🧪 Starting BPI Logbook to 6D Blockchain Bridge Integration Test...");

    // Initialize all bridge components
    let logbook_reader = BPILogbookReader::new().await?;
    let blockchain_writer = SixDBlockchainWriter::new().await?;
    let conversion_rules = ConversionRules::new().await?;
    
    // Initialize the complete bridge
    let bridge = LogbookTo6DConverter::new().await?;
    
    println!("✅ Bridge components initialized successfully");

    // Initialize all components
    logbook_reader.initialize().await?;
    blockchain_writer.initialize().await?;
    conversion_rules.initialize().await?;
    bridge.initialize().await?;
    
    println!("✅ All bridge components initialized");

    // Create test logbook entries
    let test_entries = create_test_logbook_entries().await?;
    println!("✅ Created {} test logbook entries", test_entries.len());

    // Test single entry conversion
    println!("🔄 Testing single entry conversion...");
    let single_transaction = bridge.convert_single_entry(test_entries[0].clone()).await?;
    println!("✅ Single entry converted to transaction: {}", single_transaction.transaction_id);

    // Validate the converted transaction
    validate_converted_transaction(&single_transaction, &test_entries[0]).await?;
    println!("✅ Transaction validation passed");

    // Test batch conversion
    println!("🔄 Testing batch conversion...");
    let batch_block_hash = bridge.convert_batch(test_entries.clone()).await?;
    println!("✅ Batch converted to block: {}", batch_block_hash);

    // Test blockchain writer functionality
    println!("🔄 Testing blockchain writer...");
    let writer_stats = blockchain_writer.get_stats().await?;
    println!("✅ Blockchain writer stats: {} transactions, {} blocks", 
             writer_stats.total_transactions_written, writer_stats.total_blocks_created);

    // Test conversion rules functionality
    println!("🔄 Testing conversion rules...");
    let rules_stats = conversion_rules.get_stats().await?;
    println!("✅ Conversion rules stats: {} conversions, {:.2}% success rate", 
             rules_stats.total_conversions, 
             if rules_stats.total_conversions > 0 { 
                 rules_stats.successful_conversions as f64 / rules_stats.total_conversions as f64 * 100.0 
             } else { 0.0 });

    // Test bridge metrics
    println!("🔄 Testing bridge metrics...");
    let bridge_metrics = bridge.get_conversion_metrics().await?;
    println!("✅ Bridge metrics: {} entries processed, {:.2} avg time", 
             bridge_metrics.total_logbook_entries, bridge_metrics.average_conversion_time_ms);

    // Test dimensional coordinate validation
    println!("🔄 Testing dimensional coordinate validation...");
    let test_coords = DimensionalCoordinates {
        x: 1.0, y: 2.0, z: 3.0, t: 4.0, s: 0.5, q: 0.8,
    };
    let coords_valid = blockchain_writer.validate_dimensional_coordinates(&test_coords).await?;
    assert!(coords_valid, "Dimensional coordinates should be valid");
    println!("✅ Dimensional coordinate validation passed");

    // Clean up
    bridge.stop().await?;
    blockchain_writer.stop().await?;
    
    println!("🎉 BPI Logbook to 6D Blockchain Bridge Integration Test PASSED!");
    Ok(())
}

/// Create test logbook entries for testing
async fn create_test_logbook_entries() -> Result<Vec<LogbookEntry>> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    
    let entries = vec![
        // VM Operation entry
        LogbookEntry {
            entry_id: "test_vm_op_1".to_string(),
            timestamp,
            entry_type: LogbookEntryType::VMOperation,
            vm_instance_id: "vm_test_1".to_string(),
            operation_data: OperationData {
                operation_id: "op_compute_1".to_string(),
                operation_type: "compute".to_string(),
                input_data_hash: "input_hash_1".to_string(),
                output_data_hash: "output_hash_1".to_string(),
                execution_context: ExecutionContext {
                    execution_environment: "BPI_VM".to_string(),
                    user_context: Some("user_123".to_string()),
                    session_id: Some("session_456".to_string()),
                    request_id: Some("req_789".to_string()),
                    parent_operation_id: None,
                },
                dependencies: vec!["dep_1".to_string(), "dep_2".to_string()],
                side_effects: vec![SideEffect {
                    effect_type: "data_modification".to_string(),
                    affected_resource: "data_structure".to_string(),
                    change_description: "Modified data structure".to_string(),
                    rollback_info: Some("rollback_data_1".to_string()),
                }],
            },
            audit_trail: AuditTrail {
                audit_id: "audit_vm_1".to_string(),
                compliance_tags: vec!["SOX".to_string(), "GDPR".to_string()],
                regulatory_requirements: vec!["data_protection".to_string()],
                evidence_chain: vec![],
                witness_signatures: vec![],
            },
            security_context: SecurityContext {
                security_level: "HIGH".to_string(),
                access_controls: vec!["role_admin".to_string()],
                encryption_info: EncryptionInfo {
                    algorithm: "AES-256-GCM".to_string(),
                    key_id: "key_vm_1".to_string(),
                    initialization_vector: "iv_vm_1".to_string(),
                    encryption_strength: 256,
                },
                authentication_proof: "auth_proof_vm_1".to_string(),
                authorization_proof: "authz_proof_vm_1".to_string(),
            },
            resource_usage: ResourceUsage {
                cpu_time_ms: 150,
                memory_peak_mb: 512,
                storage_bytes: 2048,
                network_bytes: 1024,
                gpu_time_ms: 0,
                quantum_operations: 0,
            },
            performance_metrics: PerformanceMetrics {
                execution_time_ms: 200,
                throughput_ops_per_sec: 50.0,
                latency_percentiles: LatencyPercentiles {
                    p50_ms: 100.0,
                    p90_ms: 180.0,
                    p95_ms: 190.0,
                    p99_ms: 195.0,
                },
                error_rate: 0.01,
                availability: 0.999,
            },
            integrity_hash: "integrity_hash_vm_1".to_string(),
        },
        
        // Security Event entry
        LogbookEntry {
            entry_id: "test_security_1".to_string(),
            timestamp: timestamp + 1,
            entry_type: LogbookEntryType::SecurityEvent,
            vm_instance_id: "vm_security_1".to_string(),
            operation_data: OperationData {
                operation_id: "op_security_1".to_string(),
                operation_type: "security_check".to_string(),
                input_data_hash: "security_input_1".to_string(),
                output_data_hash: "security_output_1".to_string(),
                execution_context: ExecutionContext {
                    execution_environment: "BPI_SECURITY".to_string(),
                    user_context: None,
                    session_id: None,
                    request_id: Some("security_req_1".to_string()),
                    parent_operation_id: None,
                },
                dependencies: vec![],
                side_effects: vec![SideEffect {
                    effect_type: "security_event".to_string(),
                    affected_resource: "security_system".to_string(),
                    change_description: "Security alert triggered".to_string(),
                    rollback_info: None,
                }],
            },
            audit_trail: AuditTrail {
                audit_id: "audit_security_1".to_string(),
                compliance_tags: vec!["SECURITY".to_string(), "AUDIT".to_string()],
                regulatory_requirements: vec!["security_monitoring".to_string()],
                evidence_chain: vec![],
                witness_signatures: vec![],
            },
            security_context: SecurityContext {
                security_level: "CRITICAL".to_string(),
                access_controls: vec!["system_only".to_string()],
                encryption_info: EncryptionInfo {
                    algorithm: "ChaCha20-Poly1305".to_string(),
                    key_id: "key_security_1".to_string(),
                    initialization_vector: "iv_security_1".to_string(),
                    encryption_strength: 256,
                },
                authentication_proof: "auth_proof_security_1".to_string(),
                authorization_proof: "authz_proof_security_1".to_string(),
            },
            resource_usage: ResourceUsage {
                cpu_time_ms: 50,
                memory_peak_mb: 128,
                storage_bytes: 512,
                network_bytes: 256,
                gpu_time_ms: 0,
                quantum_operations: 0,
            },
            performance_metrics: PerformanceMetrics {
                execution_time_ms: 75,
                throughput_ops_per_sec: 100.0,
                latency_percentiles: LatencyPercentiles {
                    p50_ms: 25.0,
                    p90_ms: 50.0,
                    p95_ms: 60.0,
                    p99_ms: 70.0,
                },
                error_rate: 0.001,
                availability: 0.9999,
            },
            integrity_hash: "integrity_hash_security_1".to_string(),
        },
        
        // System Event entry
        LogbookEntry {
            entry_id: "test_system_1".to_string(),
            timestamp: timestamp + 2,
            entry_type: LogbookEntryType::SystemEvent,
            vm_instance_id: "vm_system_1".to_string(),
            operation_data: OperationData {
                operation_id: "op_system_1".to_string(),
                operation_type: "system_maintenance".to_string(),
                input_data_hash: "system_input_1".to_string(),
                output_data_hash: "system_output_1".to_string(),
                execution_context: ExecutionContext {
                    execution_environment: "BPI_SYSTEM".to_string(),
                    user_context: None,
                    session_id: None,
                    request_id: None,
                    parent_operation_id: None,
                },
                dependencies: vec!["system_dep_1".to_string()],
                side_effects: vec![SideEffect {
                    effect_type: "system_change".to_string(),
                    affected_resource: "system_process".to_string(),
                    change_description: "System restart initiated".to_string(),
                    rollback_info: Some("restore_previous_state".to_string()),
                }],
            },
            audit_trail: AuditTrail {
                audit_id: "audit_system_1".to_string(),
                compliance_tags: vec!["SYSTEM".to_string()],
                regulatory_requirements: vec!["system_monitoring".to_string()],
                evidence_chain: vec![],
                witness_signatures: vec![],
            },
            security_context: SecurityContext {
                security_level: "MEDIUM".to_string(),
                access_controls: vec!["admin_only".to_string()],
                encryption_info: EncryptionInfo {
                    algorithm: "AES-256-CBC".to_string(),
                    key_id: "key_system_1".to_string(),
                    initialization_vector: "iv_system_1".to_string(),
                    encryption_strength: 256,
                },
                authentication_proof: "auth_proof_system_1".to_string(),
                authorization_proof: "authz_proof_system_1".to_string(),
            },
            resource_usage: ResourceUsage {
                cpu_time_ms: 300,
                memory_peak_mb: 1024,
                storage_bytes: 4096,
                network_bytes: 2048,
                gpu_time_ms: 0,
                quantum_operations: 0,
            },
            performance_metrics: PerformanceMetrics {
                execution_time_ms: 400,
                throughput_ops_per_sec: 25.0,
                latency_percentiles: LatencyPercentiles {
                    p50_ms: 200.0,
                    p90_ms: 350.0,
                    p95_ms: 380.0,
                    p99_ms: 390.0,
                },
                error_rate: 0.005,
                availability: 0.995,
            },
            integrity_hash: "integrity_hash_system_1".to_string(),
        },
    ];

    Ok(entries)
}

/// Validate that a converted transaction matches the original logbook entry
async fn validate_converted_transaction(transaction: &SixDTransaction, entry: &LogbookEntry) -> Result<()> {
    // Validate basic mapping
    assert_eq!(transaction.logbook_entry_id, entry.entry_id);
    assert_eq!(transaction.timestamp, entry.timestamp);
    
    // Validate transaction type mapping
    match (&transaction.transaction_type, &entry.entry_type) {
        (TransactionType::VMOperation, LogbookEntryType::VMOperation) => {},
        (TransactionType::SecurityEvent, LogbookEntryType::SecurityEvent) => {},
        (TransactionType::SystemEvent, LogbookEntryType::SystemEvent) => {},
        _ => return Err(anyhow::anyhow!("Transaction type mismatch")),
    }
    
    // Validate dimensional coordinates are within valid ranges
    let coords = &transaction.dimensional_coordinates;
    assert!(coords.x.is_finite() && coords.x >= -1000.0 && coords.x <= 1000.0);
    assert!(coords.y.is_finite() && coords.y >= -1000.0 && coords.y <= 1000.0);
    assert!(coords.z.is_finite() && coords.z >= -1000.0 && coords.z <= 1000.0);
    assert!(coords.t.is_finite() && coords.t >= 0.0);
    assert!(coords.s.is_finite() && coords.s >= 0.0 && coords.s <= 1.0);
    assert!(coords.q.is_finite() && coords.q >= 0.0 && coords.q <= 1.0);
    
    // Validate transaction data mapping
    assert_eq!(transaction.transaction_data.operation_hash, entry.operation_data.operation_id);
    assert_eq!(transaction.transaction_data.input_data_hash, entry.operation_data.input_data_hash);
    assert_eq!(transaction.transaction_data.output_data_hash, entry.operation_data.output_data_hash);
    
    // Validate cryptographic proofs are present
    assert!(!transaction.cryptographic_proofs.merkle_proof.is_empty());
    assert!(!transaction.cryptographic_proofs.zero_knowledge_proof.is_empty());
    assert!(!transaction.cryptographic_proofs.quantum_proof.is_empty());
    assert!(!transaction.cryptographic_proofs.consensus_proof.is_empty());
    assert!(!transaction.cryptographic_proofs.integrity_proof.is_empty());
    assert!(!transaction.cryptographic_proofs.non_repudiation_proof.is_empty());
    
    // Validate quantum signature and integrity hash
    assert!(!transaction.quantum_signature.is_empty());
    assert!(!transaction.integrity_hash.is_empty());
    
    println!("✅ Transaction validation passed for entry: {}", entry.entry_id);
    Ok(())
}

/// Run performance benchmark for the bridge
pub async fn benchmark_bridge_performance() -> Result<()> {
    println!("🏃 Starting BPI Logbook to 6D Blockchain Bridge Performance Benchmark...");

    let bridge = LogbookTo6DConverter::new().await?;
    bridge.initialize().await?;

    let test_entries = create_test_logbook_entries().await?;
    let entry_count = test_entries.len();
    
    let start_time = std::time::Instant::now();
    
    // Benchmark batch conversion
    for batch in test_entries.chunks(10) {
        let _block_hash = bridge.convert_batch(batch.to_vec()).await?;
    }
    
    let elapsed = start_time.elapsed();
    let entries_per_second = entry_count as f64 / elapsed.as_secs_f64();
    
    println!("📊 Performance Results:");
    println!("   - Total entries: {}", entry_count);
    println!("   - Total time: {:.2}s", elapsed.as_secs_f64());
    println!("   - Entries per second: {:.2}", entries_per_second);
    println!("   - Average time per entry: {:.2}ms", elapsed.as_millis() as f64 / entry_count as f64);
    
    bridge.stop().await?;
    
    println!("🎉 Performance benchmark completed!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bridge_integration() {
        test_complete_bridge_integration().await.unwrap();
    }

    #[tokio::test]
    async fn test_bridge_performance() {
        benchmark_bridge_performance().await.unwrap();
    }
}
