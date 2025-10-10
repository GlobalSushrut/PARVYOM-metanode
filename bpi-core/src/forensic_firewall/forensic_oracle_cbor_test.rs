use std::sync::Arc;
use anyhow::Result;
use std::collections::BTreeMap;
use crate::forensic_firewall::forensic_oracle_cbor::*;
use crate::cbor_pipeline_foundation::CborSerializable;
use crate::immutable_audit_system::ImmutableAuditSystem;

#[tokio::test]
async fn test_forensic_oracle_cbor_serialization() -> Result<()> {
    println!("🔒 Testing Forensic Oracle CBOR Serialization - Government Enterprise-Grade");
    
    // Create audit system
    let audit_system = Arc::new(ImmutableAuditSystem::default());
    
    // Create forensic oracle configuration
    let config = ForensicOracleConfig {
        ai_analysis_enabled: true,
        evidence_correlation_enabled: true,
        threat_prediction_enabled: true,
        workflow_automation_enabled: true,
        intelligence_sharing_enabled: false, // Security default
        confidence_threshold: 0.85,
        analysis_depth: AnalysisDepth::Deep,
    };
    
    // Create forensic oracle with government compliance
    let mut oracle = ForensicOracle::new_with_compliance(config, audit_system)?;
    
    // Add some audit trail entries to test serialization
    let mut analysis_data = BTreeMap::new();
    analysis_data.insert("threat_type".to_string(), serde_json::Value::String("advanced_persistent_threat".to_string()));
    analysis_data.insert("confidence_score".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.92).unwrap()));
    analysis_data.insert("evidence_count".to_string(), serde_json::Value::Number(serde_json::Number::from(15)));
    analysis_data.insert("impossible_to_hide".to_string(), serde_json::Value::Bool(true));
    
    oracle.record_audit_entry("threat_analysis_completed", analysis_data)?;
    oracle.update_performance_metrics(125.5, true)?;
    
    // Test CBOR serialization
    let cbor_data = oracle.to_cbor()?;
    println!("✅ CBOR Serialization successful - Size: {} bytes", cbor_data.len());
    
    // Verify CBOR data is not empty and has reasonable size
    assert!(!cbor_data.is_empty(), "CBOR data should not be empty");
    assert!(cbor_data.len() > 100, "CBOR data should have substantial size for government compliance");
    
    // Test CBOR deserialization
    let deserialized_oracle = ForensicOracle::from_cbor(&cbor_data)?;
    println!("✅ CBOR Deserialization successful");
    
    // Verify critical data integrity
    assert_eq!(oracle.id, deserialized_oracle.id, "Oracle ID must be preserved");
    assert_eq!(oracle.config.ai_analysis_enabled, deserialized_oracle.config.ai_analysis_enabled, "AI analysis config must be preserved");
    assert_eq!(oracle.config.confidence_threshold, deserialized_oracle.config.confidence_threshold, "Confidence threshold must be preserved");
    assert_eq!(oracle.audit_trail.len(), deserialized_oracle.audit_trail.len(), "Audit trail entries must be preserved");
    assert_eq!(oracle.performance_metrics.analysis_count, deserialized_oracle.performance_metrics.analysis_count, "Performance metrics must be preserved");
    assert_eq!(oracle.compliance_metadata.retention_policy_years, deserialized_oracle.compliance_metadata.retention_policy_years, "Compliance metadata must be preserved");
    
    println!("✅ Data integrity verification passed");
    
    // Test human-readable diagnostic output
    let diagnostic = oracle.to_diagnostic()?;
    println!("✅ Human-readable diagnostic generated");
    
    // Verify diagnostic contains key information
    assert!(diagnostic.contains("FORENSIC-ORACLE-CBOR-DIAGNOSTIC"), "Diagnostic must contain header");
    assert!(diagnostic.contains(&oracle.id), "Diagnostic must contain Oracle ID");
    assert!(diagnostic.contains("Government-Compliance: VERIFIED"), "Diagnostic must show government compliance");
    assert!(diagnostic.contains("Impossible-To-Hide: ENABLED"), "Diagnostic must show impossible-to-hide feature");
    
    println!("✅ Diagnostic verification passed");
    
    Ok(())
}

#[tokio::test]
async fn test_forensic_oracle_audit_trail_integrity() -> Result<()> {
    println!("🔒 Testing Forensic Oracle Audit Trail Integrity - Impossible to Hide");
    
    let audit_system = Arc::new(ImmutableAuditSystem::default());
    let config = ForensicOracleConfig::default();
    let mut oracle = ForensicOracle::new_with_compliance(config, audit_system)?;
    
    // Record multiple audit entries
    for i in 0..5 {
        let mut event_data = BTreeMap::new();
        event_data.insert("event_sequence".to_string(), serde_json::Value::Number(serde_json::Number::from(i)));
        event_data.insert("event_type".to_string(), serde_json::Value::String(format!("test_event_{}", i)));
        event_data.insert("timestamp".to_string(), serde_json::Value::String(chrono::Utc::now().to_rfc3339()));
        event_data.insert("impossible_to_hide".to_string(), serde_json::Value::Bool(true));
        event_data.insert("government_compliance".to_string(), serde_json::Value::Bool(true));
        
        oracle.record_audit_entry(&format!("test_audit_event_{}", i), event_data)?;
        oracle.update_performance_metrics(50.0 + (i as f64 * 10.0), true)?;
    }
    
    // Verify all audit entries are recorded
    assert_eq!(oracle.audit_trail.len(), 11, "Should have 11 audit entries (1 creation + 5 events + 5 performance updates)");
    
    // Test CBOR round-trip preserves all audit entries
    let cbor_data = oracle.to_cbor()?;
    let deserialized_oracle = ForensicOracle::from_cbor(&cbor_data)?;
    
    assert_eq!(oracle.audit_trail.len(), deserialized_oracle.audit_trail.len(), "All audit entries must be preserved");
    
    // Verify each audit entry has required government compliance fields
    for (entry_id, entry_value) in &deserialized_oracle.audit_trail {
        let entry = entry_value.as_object().expect("Audit entry should be an object");
        
        assert!(entry.contains_key("entry_id"), "Audit entry must have entry_id");
        assert!(entry.contains_key("oracle_id"), "Audit entry must have oracle_id");
        assert!(entry.contains_key("event_type"), "Audit entry must have event_type");
        assert!(entry.contains_key("timestamp"), "Audit entry must have timestamp");
        assert!(entry.contains_key("witness_signature"), "Audit entry must have witness_signature");
        assert!(entry.contains_key("integrity_hash"), "Audit entry must have integrity_hash");
        assert!(entry.contains_key("retention_years"), "Audit entry must have retention_years");
        assert!(entry.contains_key("classification"), "Audit entry must have classification");
        assert!(entry.contains_key("impossible_to_hide"), "Audit entry must have impossible_to_hide flag");
        
        // Verify government compliance values
        assert_eq!(entry["retention_years"].as_u64().unwrap(), 7, "Retention must be 7 years");
        assert_eq!(entry["classification"].as_str().unwrap(), "GOVERNMENT-ENTERPRISE-GRADE", "Classification must be government-grade");
        assert_eq!(entry["impossible_to_hide"].as_bool().unwrap(), true, "Impossible-to-hide must be enabled");
    }
    
    println!("✅ Audit trail integrity verification passed - All {} entries preserved with government compliance", deserialized_oracle.audit_trail.len());
    
    Ok(())
}

#[tokio::test]
async fn test_forensic_oracle_performance_metrics_tracking() -> Result<()> {
    println!("🔒 Testing Forensic Oracle Performance Metrics Tracking");
    
    let audit_system = Arc::new(ImmutableAuditSystem::default());
    let config = ForensicOracleConfig::default();
    let mut oracle = ForensicOracle::new_with_compliance(config, audit_system)?;
    
    // Simulate various operations with different success rates and timings
    let operations = vec![
        (100.0, true),   // Fast successful operation
        (250.0, true),   // Slower successful operation
        (75.0, false),   // Fast failed operation
        (300.0, true),   // Slow successful operation
        (150.0, false),  // Medium failed operation
    ];
    
    for (time_ms, success) in operations {
        oracle.update_performance_metrics(time_ms, success)?;
    }
    
    // Verify performance metrics are calculated correctly
    assert_eq!(oracle.performance_metrics.analysis_count, 6, "Should have 6 analyses (1 initial + 5 operations)");
    assert!(oracle.performance_metrics.avg_analysis_time_ms > 0.0, "Average analysis time should be positive");
    assert!(oracle.performance_metrics.threat_detection_rate > 0.0, "Threat detection rate should be positive");
    assert!(oracle.performance_metrics.threat_detection_rate <= 1.0, "Threat detection rate should not exceed 1.0");
    
    // Test CBOR serialization preserves performance metrics
    let cbor_data = oracle.to_cbor()?;
    let deserialized_oracle = ForensicOracle::from_cbor(&cbor_data)?;
    
    assert_eq!(oracle.performance_metrics.analysis_count, deserialized_oracle.performance_metrics.analysis_count, "Analysis count must be preserved");
    assert_eq!(oracle.performance_metrics.avg_analysis_time_ms, deserialized_oracle.performance_metrics.avg_analysis_time_ms, "Average analysis time must be preserved");
    assert_eq!(oracle.performance_metrics.threat_detection_rate, deserialized_oracle.performance_metrics.threat_detection_rate, "Threat detection rate must be preserved");
    assert_eq!(oracle.performance_metrics.evidence_correlation_rate, deserialized_oracle.performance_metrics.evidence_correlation_rate, "Evidence correlation rate must be preserved");
    assert_eq!(oracle.performance_metrics.workflow_success_rate, deserialized_oracle.performance_metrics.workflow_success_rate, "Workflow success rate must be preserved");
    
    println!("✅ Performance metrics tracking verification passed");
    println!("   Analysis Count: {}", oracle.performance_metrics.analysis_count);
    println!("   Avg Analysis Time: {:.2}ms", oracle.performance_metrics.avg_analysis_time_ms);
    println!("   Threat Detection Rate: {:.3}", oracle.performance_metrics.threat_detection_rate);
    
    Ok(())
}

#[tokio::test]
async fn test_forensic_oracle_government_compliance_metadata() -> Result<()> {
    println!("🔒 Testing Forensic Oracle Government Compliance Metadata");
    
    let audit_system = Arc::new(ImmutableAuditSystem::default());
    let config = ForensicOracleConfig::default();
    let oracle = ForensicOracle::new_with_compliance(config, audit_system)?;
    
    // Verify government compliance metadata
    assert_eq!(oracle.compliance_metadata.retention_policy_years, 7, "Retention policy must be 7 years");
    assert_eq!(oracle.compliance_metadata.classification, "GOVERNMENT-ENTERPRISE-GRADE", "Classification must be government-grade");
    assert_eq!(oracle.compliance_metadata.encryption_standard, "AES-256-GCM", "Encryption standard must be AES-256-GCM");
    
    // Verify required audit standards
    let required_standards = vec!["SOC2", "FIPS_140_2", "FISMA", "COMMON_CRITERIA"];
    for standard in required_standards {
        assert!(oracle.compliance_metadata.audit_requirements.contains(&standard.to_string()), 
                "Must include {} audit requirement", standard);
    }
    
    // Verify required access controls
    let required_controls = vec!["RBAC", "MFA", "ZERO_TRUST"];
    for control in required_controls {
        assert!(oracle.compliance_metadata.access_controls.contains(&control.to_string()), 
                "Must include {} access control", control);
    }
    
    // Test CBOR serialization preserves compliance metadata
    let cbor_data = oracle.to_cbor()?;
    let deserialized_oracle = ForensicOracle::from_cbor(&cbor_data)?;
    
    assert_eq!(oracle.compliance_metadata.retention_policy_years, deserialized_oracle.compliance_metadata.retention_policy_years, "Retention policy must be preserved");
    assert_eq!(oracle.compliance_metadata.classification, deserialized_oracle.compliance_metadata.classification, "Classification must be preserved");
    assert_eq!(oracle.compliance_metadata.encryption_standard, deserialized_oracle.compliance_metadata.encryption_standard, "Encryption standard must be preserved");
    assert_eq!(oracle.compliance_metadata.audit_requirements, deserialized_oracle.compliance_metadata.audit_requirements, "Audit requirements must be preserved");
    assert_eq!(oracle.compliance_metadata.access_controls, deserialized_oracle.compliance_metadata.access_controls, "Access controls must be preserved");
    
    println!("✅ Government compliance metadata verification passed");
    println!("   Retention Policy: {} years", oracle.compliance_metadata.retention_policy_years);
    println!("   Classification: {}", oracle.compliance_metadata.classification);
    println!("   Encryption Standard: {}", oracle.compliance_metadata.encryption_standard);
    println!("   Audit Requirements: {:?}", oracle.compliance_metadata.audit_requirements);
    println!("   Access Controls: {:?}", oracle.compliance_metadata.access_controls);
    
    Ok(())
}

#[tokio::test]
async fn test_forensic_oracle_cbor_compression_efficiency() -> Result<()> {
    println!("🔒 Testing Forensic Oracle CBOR Compression Efficiency");
    
    let audit_system = Arc::new(ImmutableAuditSystem::default());
    let config = ForensicOracleConfig::default();
    let mut oracle = ForensicOracle::new_with_compliance(config, audit_system)?;
    
    // Add substantial audit trail data to test compression
    for i in 0..20 {
        let mut large_event_data = BTreeMap::new();
        large_event_data.insert("event_id".to_string(), serde_json::Value::String(format!("event_{:04}", i)));
        large_event_data.insert("event_description".to_string(), serde_json::Value::String(format!("Comprehensive forensic analysis event {} with detailed metadata and government compliance tracking", i)));
        large_event_data.insert("threat_indicators".to_string(), serde_json::Value::Array(vec![
            serde_json::Value::String("suspicious_network_activity".to_string()),
            serde_json::Value::String("anomalous_file_access".to_string()),
            serde_json::Value::String("privilege_escalation_attempt".to_string()),
        ]));
        large_event_data.insert("evidence_artifacts".to_string(), serde_json::Value::Array(vec![
            serde_json::Value::String("network_packet_capture.pcap".to_string()),
            serde_json::Value::String("memory_dump.raw".to_string()),
            serde_json::Value::String("filesystem_timeline.csv".to_string()),
        ]));
        large_event_data.insert("confidence_score".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.85 + (i as f64 * 0.005)).unwrap()));
        large_event_data.insert("impossible_to_hide".to_string(), serde_json::Value::Bool(true));
        large_event_data.insert("government_compliance".to_string(), serde_json::Value::Bool(true));
        
        oracle.record_audit_entry(&format!("comprehensive_analysis_{}", i), large_event_data)?;
        oracle.update_performance_metrics(100.0 + (i as f64 * 5.0), i % 4 != 0)?; // 75% success rate
    }
    
    // Test CBOR serialization efficiency
    let cbor_data = oracle.to_cbor()?;
    let json_data = serde_json::to_vec(&oracle)?;
    
    println!("✅ CBOR Compression Analysis:");
    println!("   CBOR Size: {} bytes", cbor_data.len());
    println!("   JSON Size: {} bytes", json_data.len());
    println!("   Compression Ratio: {:.2}%", (cbor_data.len() as f64 / json_data.len() as f64) * 100.0);
    
    // CBOR should be more efficient than JSON for structured data
    assert!(cbor_data.len() <= json_data.len(), "CBOR should be at least as efficient as JSON");
    
    // Test round-trip integrity with large dataset
    let deserialized_oracle = ForensicOracle::from_cbor(&cbor_data)?;
    assert_eq!(oracle.audit_trail.len(), deserialized_oracle.audit_trail.len(), "All audit entries must be preserved");
    assert_eq!(oracle.performance_metrics.analysis_count, deserialized_oracle.performance_metrics.analysis_count, "Performance metrics must be preserved");
    
    println!("✅ CBOR compression efficiency verification passed");
    
    Ok(())
}
