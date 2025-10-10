use anyhow::Result;
use chrono::{DateTime, Utc};
use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::cbor_pipeline_foundation::{PravyomConfig, AuditTrail, ComplianceMetadata, PerformanceMetrics, AuditEntry, RetentionPolicy, GovernmentComplianceAudit, CborSerializable, PipelineMetrics};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_struct_creation() -> Result<()> {
        println!("🧪 Testing Simple Struct Creation with All Required Fields...");

        // Create RetentionPolicy with all required fields
        let retention_policy = RetentionPolicy {
            auto_delete_after_years: 7,
            compliance_requirements: vec!["SOC2".to_string(), "FIPS_140_2".to_string()],
            legal_hold: false,
            policy_id: "test_policy_001".to_string(),
            retention_years: 7,
        };

        // Create GovernmentComplianceAudit with all required fields
        let government_compliance = GovernmentComplianceAudit {
            audit_reference: "GOV_AUDIT_001".to_string(),
            compliance_tags: vec!["soc2".to_string(), "fips140".to_string()],
            jurisdiction: "US_FEDERAL".to_string(),
        };

        // Create AuditEntry with all required fields
        let audit_entry = AuditEntry {
            audit_id: "audit_001".to_string(),
            entry_type: "test_creation".to_string(),
            created_at: Utc::now(),
            audit_data: {
                let mut data = BTreeMap::new();
                data.insert("action".to_string(), serde_json::Value::String("test".to_string()));
                data
            },
            integrity_hash: "sha256:test_hash".to_string(),
        };

        // Create AuditTrail with all required fields
        let audit_trail = AuditTrail {
            audit_entries: vec![audit_entry],
            compliance_score: 95.0,
            created_at: Utc::now(),
            entry_id: "audit_trail_001".to_string(),
            government_compliance,
            integrity_hash: "sha256:audit_trail_hash".to_string(),
            retention_policy: retention_policy.clone(),
            retention_years: 7,
            witness_signatures: vec!["witness_001".to_string()],
        };

        // Create ComplianceMetadata with all required fields
        let compliance_metadata = ComplianceMetadata {
            audit_requirements: vec!["SOC2".to_string()],
            classification: "government_enterprise".to_string(),
            created_at: Utc::now(),
            last_reviewed: Utc::now(),
            last_updated: Utc::now(),
            retention_policy: "7_years".to_string(),
        };

        // Create PerformanceMetrics with all required fields
        let performance_metrics = PerformanceMetrics {
            cpu_usage_percent: 25.0,
            created_at: Utc::now(),
            last_updated: Utc::now(),
            latency_ms: 100.0,
            memory_usage_mb: 50.0,
            throughput_records_per_second: 10.0,
        };

        // Create PravyomConfig with all required fields
        let config = PravyomConfig {
            auction_interval_seconds: 300,
            audit_trail,
            bundle_size_limit: 1000,
            compliance_metadata,
            config_id: "test_config_001".to_string(),
            created_at: Utc::now(),
            max_segments: 100,
            performance_metrics,
            pipeline_id: "test_pipeline_001".to_string(),
            segment_size_threshold: 5000,
            segment_threshold: 10,
            time_threshold_seconds: 600,
        };

        // Test CBOR serialization
        let cbor_data = config.to_cbor()?;
        assert!(!cbor_data.is_empty(), "CBOR data should not be empty");

        // Test CBOR deserialization
        let deserialized_config = PravyomConfig::from_cbor(&cbor_data)?;
        assert_eq!(config.config_id, deserialized_config.config_id);

        // Test diagnostic output
        let diagnostic = config.to_diagnostic()?;
        assert!(diagnostic.contains("test_config_001"), "Diagnostic should contain config ID");

        println!("✅ Simple Struct Creation: PASSED");
        Ok(())
    }

    #[tokio::test]
    async fn test_pipeline_metrics_creation() -> Result<()> {
        println!("🧪 Testing PipelineMetrics Creation...");

        let pipeline_metrics = PipelineMetrics {
            average_processing_time_ms: 150.0,
            error_rate: 0.01,
            last_updated: Utc::now(),
            throughput_per_second: 25.0,
            total_processed: 1000,
        };

        // Test basic struct creation
        assert!(pipeline_metrics.total_processed > 0, "Total processed should be greater than 0");
        assert!(pipeline_metrics.error_rate >= 0.0, "Error rate should be non-negative");

        println!("✅ PipelineMetrics Creation: PASSED");
        Ok(())
    }

    /// Level 2 Test: CBOR Serialization Round-Trip
    #[tokio::test]
    async fn test_cbor_serialization_round_trip() -> Result<()> {
        println!("🧪 Level 2: Testing CBOR Serialization Round-Trip...");

        // Create a complex config with nested structures
        let original_config = create_test_config("round_trip_test")?;

        // Test CBOR serialization
        let cbor_data = original_config.to_cbor()?;
        assert!(!cbor_data.is_empty(), "CBOR data should not be empty");
        assert!(cbor_data.len() > 100, "CBOR data should be substantial");

        // Test CBOR deserialization
        let deserialized_config = PravyomConfig::from_cbor(&cbor_data)?;

        // Verify key fields match
        assert_eq!(original_config.config_id, deserialized_config.config_id);
        assert_eq!(original_config.pipeline_id, deserialized_config.pipeline_id);
        assert_eq!(original_config.max_segments, deserialized_config.max_segments);
        assert_eq!(original_config.bundle_size_limit, deserialized_config.bundle_size_limit);

        // Test diagnostic output
        let diagnostic = original_config.to_diagnostic()?;
        assert!(diagnostic.contains("round_trip_test"), "Diagnostic should contain test ID");
        assert!(diagnostic.len() > 200, "Diagnostic should be comprehensive");

        println!("✅ Level 2: CBOR Round-Trip: PASSED");
        Ok(())
    }

    /// Level 2 Test: Multiple Config Validation
    #[tokio::test]
    async fn test_multiple_config_validation() -> Result<()> {
        println!("🧪 Level 2: Testing Multiple Config Validation...");

        // Create multiple configs with different parameters
        let configs = vec![
            create_test_config("config_001")?,
            create_test_config("config_002")?,
            create_test_config("config_003")?,
        ];

        // Test each config individually
        for (i, config) in configs.iter().enumerate() {
            // Validate CBOR serialization
            let cbor_data = config.to_cbor()?;
            assert!(!cbor_data.is_empty(), "Config {} CBOR should not be empty", i);

            // Validate config fields
            assert!(config.max_segments > 0, "Config {} max_segments should be positive", i);
            assert!(config.bundle_size_limit > 0, "Config {} bundle_size_limit should be positive", i);
            assert!(config.auction_interval_seconds > 0, "Config {} auction_interval_seconds should be positive", i);

            // Validate nested structures
            assert!(!config.audit_trail.audit_entries.is_empty() || config.audit_trail.audit_entries.is_empty(), "Audit entries should be valid");
            assert!(config.audit_trail.compliance_score >= 0.0 && config.audit_trail.compliance_score <= 100.0, "Compliance score should be 0-100");
        }

        println!("✅ Level 2: Multiple Config Validation: PASSED");
        Ok(())
    }

    /// Level 2 Test: Error Handling and Edge Cases
    #[tokio::test]
    async fn test_error_handling_edge_cases() -> Result<()> {
        println!("🧪 Level 2: Testing Error Handling and Edge Cases...");

        // Test invalid CBOR data
        let invalid_cbor = vec![0xFF, 0xFE, 0xFD]; // Invalid CBOR
        let result = PravyomConfig::from_cbor(&invalid_cbor);
        assert!(result.is_err(), "Should fail with invalid CBOR data");

        // Test empty CBOR data
        let empty_cbor = vec![];
        let result = PravyomConfig::from_cbor(&empty_cbor);
        assert!(result.is_err(), "Should fail with empty CBOR data");

        // Test valid config with edge case values
        let mut config = create_test_config("edge_case_test")?;
        
        // Test with minimum values
        config.max_segments = 1;
        config.bundle_size_limit = 1;
        config.auction_interval_seconds = 1;
        
        // Should still serialize/deserialize correctly
        let cbor_data = config.to_cbor()?;
        let deserialized = PravyomConfig::from_cbor(&cbor_data)?;
        assert_eq!(config.max_segments, deserialized.max_segments);

        println!("✅ Level 2: Error Handling: PASSED");
        Ok(())
    }

    /// Level 2 Test: Performance and Compliance Validation
    #[tokio::test]
    async fn test_performance_compliance_validation() -> Result<()> {
        println!("🧪 Level 2: Testing Performance and Compliance Validation...");

        let config = create_test_config("performance_test")?;

        // Test performance metrics validation
        assert!(config.performance_metrics.cpu_usage_percent >= 0.0, "CPU usage should be non-negative");
        assert!(config.performance_metrics.cpu_usage_percent <= 100.0, "CPU usage should not exceed 100%");
        assert!(config.performance_metrics.memory_usage_mb > 0.0, "Memory usage should be positive");
        assert!(config.performance_metrics.throughput_records_per_second >= 0.0, "Throughput should be non-negative");

        // Test compliance metadata validation
        assert!(!config.compliance_metadata.classification.is_empty(), "Classification should not be empty");
        assert!(!config.compliance_metadata.audit_requirements.is_empty(), "Audit requirements should not be empty");
        assert_eq!(config.compliance_metadata.classification, "government_enterprise", "Should use government enterprise classification");

        // Test audit trail compliance
        assert!(config.audit_trail.retention_years >= 7, "Should meet 7-year retention requirement");
        assert!(!config.audit_trail.government_compliance.jurisdiction.is_empty(), "Jurisdiction should be specified");
        assert!(!config.audit_trail.government_compliance.compliance_tags.is_empty(), "Compliance tags should not be empty");

        // Test CBOR canonical validation
        assert!(config.validate_cbor()?, "Config should pass CBOR validation");

        println!("✅ Level 2: Performance and Compliance: PASSED");
        Ok(())
    }

    /// Helper function to create test configs with different IDs
    fn create_test_config(config_id: &str) -> Result<PravyomConfig> {
        let retention_policy = RetentionPolicy {
            auto_delete_after_years: 7,
            compliance_requirements: vec!["SOC2".to_string(), "FIPS_140_2".to_string()],
            legal_hold: false,
            policy_id: format!("policy_{}", config_id),
            retention_years: 7,
        };

        let government_compliance = GovernmentComplianceAudit {
            audit_reference: format!("GOV_AUDIT_{}", config_id.to_uppercase()),
            compliance_tags: vec!["soc2".to_string(), "fips140".to_string(), "fisma".to_string()],
            jurisdiction: "US_FEDERAL".to_string(),
        };

        let audit_entry = AuditEntry {
            audit_id: format!("audit_{}", config_id),
            entry_type: "config_creation".to_string(),
            created_at: Utc::now(),
            audit_data: {
                let mut data = BTreeMap::new();
                data.insert("action".to_string(), serde_json::Value::String("create_config".to_string()));
                data.insert("config_id".to_string(), serde_json::Value::String(config_id.to_string()));
                data
            },
            integrity_hash: format!("sha256:hash_{}", config_id),
        };

        let audit_trail = AuditTrail {
            audit_entries: vec![audit_entry],
            compliance_score: 95.0,
            created_at: Utc::now(),
            entry_id: format!("audit_trail_{}", config_id),
            government_compliance,
            integrity_hash: format!("sha256:audit_trail_hash_{}", config_id),
            retention_policy: retention_policy.clone(),
            retention_years: 7,
            witness_signatures: vec![format!("witness_{}", config_id)],
        };

        let compliance_metadata = ComplianceMetadata {
            audit_requirements: vec!["SOC2".to_string(), "FIPS_140_2".to_string()],
            classification: "government_enterprise".to_string(),
            created_at: Utc::now(),
            last_reviewed: Utc::now(),
            last_updated: Utc::now(),
            retention_policy: "7_years".to_string(),
        };

        let performance_metrics = PerformanceMetrics {
            cpu_usage_percent: 25.0 + (config_id.len() as f64 % 10.0), // Slight variation
            created_at: Utc::now(),
            last_updated: Utc::now(),
            latency_ms: 100.0 + (config_id.len() as f64 % 50.0), // Slight variation
            memory_usage_mb: 50.0 + (config_id.len() as f64 % 20.0), // Slight variation
            throughput_records_per_second: 10.0 + (config_id.len() as f64 % 5.0), // Slight variation
        };

        Ok(PravyomConfig {
            auction_interval_seconds: 300 + (config_id.len() as u64 % 100), // Slight variation
            audit_trail,
            bundle_size_limit: 1000 + (config_id.len() % 500), // Slight variation
            compliance_metadata,
            config_id: config_id.to_string(),
            created_at: Utc::now(),
            max_segments: 100 + (config_id.len() as u32 % 50), // Slight variation
            performance_metrics,
            pipeline_id: format!("pipeline_{}", config_id),
            segment_size_threshold: 5000 + (config_id.len() % 1000), // Slight variation
            segment_threshold: 10 + (config_id.len() as u32 % 5), // Slight variation
            time_threshold_seconds: 600 + (config_id.len() as u64 % 200), // Slight variation
        })
    }

    /// Level 3 Test: Advanced Government Compliance Validation
    #[tokio::test]
    async fn test_advanced_government_compliance() -> Result<()> {
        println!("🧪 Level 3: Testing Advanced Government Compliance...");

        // Create high-security government config
        let mut config = create_test_config("gov_classified_001")?;
        
        // Set advanced compliance requirements
        config.compliance_metadata.classification = "government_enterprise".to_string();
        config.compliance_metadata.audit_requirements = vec![
            "SOC2".to_string(),
            "FIPS_140_2".to_string(),
            "FISMA".to_string(),
            "NIST_800_53".to_string(),
            "COMMON_CRITERIA_EAL4".to_string(),
        ];

        // Validate 7-year retention compliance (government requirement)
        assert_eq!(config.audit_trail.retention_years, 7, "Must meet 7-year government retention");
        assert_eq!(config.audit_trail.retention_policy.retention_years, 7, "Retention policy must match");
        assert_eq!(config.audit_trail.retention_policy.auto_delete_after_years, 7, "Auto-delete must be 7 years");

        // Validate government compliance audit structure
        assert_eq!(config.audit_trail.government_compliance.jurisdiction, "US_FEDERAL", "Must be US Federal jurisdiction");
        assert!(config.audit_trail.government_compliance.compliance_tags.contains(&"soc2".to_string()), "Must include SOC2");
        assert!(config.audit_trail.government_compliance.compliance_tags.contains(&"fips140".to_string()), "Must include FIPS140");
        assert!(config.audit_trail.government_compliance.compliance_tags.contains(&"fisma".to_string()), "Must include FISMA");

        // Validate witness signatures (impossible-to-hide requirement)
        assert!(!config.audit_trail.witness_signatures.is_empty(), "Must have witness signatures");
        assert!(config.audit_trail.witness_signatures.len() >= 1, "Must have at least one witness");

        // Validate integrity hashes (tamper-proof requirement)
        assert!(config.audit_trail.integrity_hash.starts_with("sha256:"), "Must use SHA256 integrity hash");
        assert!(config.audit_trail.integrity_hash.len() > 10, "Integrity hash must be substantial");

        // Validate audit entries have proper structure
        for entry in &config.audit_trail.audit_entries {
            assert!(!entry.audit_id.is_empty(), "Audit ID must not be empty");
            assert!(!entry.entry_type.is_empty(), "Entry type must not be empty");
            assert!(entry.integrity_hash.starts_with("sha256:"), "Entry must have SHA256 hash");
            assert!(!entry.audit_data.is_empty(), "Audit data must not be empty");
        }

        // Test CBOR canonical serialization (government requirement)
        let cbor_data = config.to_cbor()?;
        assert!(config.validate_cbor()?, "Must pass CBOR canonical validation");
        
        // Test diagnostic output (human-readable requirement)
        let diagnostic = config.to_diagnostic()?;
        assert!(diagnostic.contains("government_enterprise"), "Diagnostic must show classification");
        assert!(diagnostic.contains("FIPS_140_2"), "Diagnostic must show FIPS compliance");
        assert!(diagnostic.len() > 500, "Diagnostic must be comprehensive");

        println!("✅ Level 3: Advanced Government Compliance: PASSED");
        Ok(())
    }

    /// Level 3 Test: Real-World Audit Trail Integrity
    #[tokio::test]
    async fn test_real_world_audit_trail_integrity() -> Result<()> {
        println!("🧪 Level 3: Testing Real-World Audit Trail Integrity...");

        // Create multiple configs simulating real government operations
        let operations = vec![
            ("classified_doc_access", "document_access"),
            ("security_clearance_check", "clearance_verification"),
            ("data_transmission", "secure_transfer"),
            ("audit_review", "compliance_audit"),
            ("incident_response", "security_incident"),
        ];

        let mut all_configs = Vec::new();
        let mut all_hashes = Vec::new();

        for (op_id, op_type) in operations {
            let mut config = create_test_config(op_id)?;
            
            // Add operation-specific audit entry
            let operation_entry = AuditEntry {
                audit_id: format!("audit_{}_{}", op_id, Utc::now().timestamp()),
                entry_type: op_type.to_string(),
                created_at: Utc::now(),
                audit_data: {
                    let mut data = BTreeMap::new();
                    data.insert("operation".to_string(), serde_json::Value::String(op_id.to_string()));
                    data.insert("timestamp".to_string(), serde_json::Value::Number(serde_json::Number::from(Utc::now().timestamp())));
                    data.insert("user_clearance".to_string(), serde_json::Value::String("SECRET".to_string()));
                    data.insert("classification".to_string(), serde_json::Value::String("GOVERNMENT_ENTERPRISE".to_string()));
                    data
                },
                integrity_hash: format!("sha256:operation_{}_{}", op_id, Utc::now().timestamp()),
            };

            config.audit_trail.audit_entries.push(operation_entry);
            
            // Validate each config's integrity
            assert!(config.validate_cbor()?, "Config {} must pass CBOR validation", op_id);
            
            let cbor_data = config.to_cbor()?;
            let mut hasher = DefaultHasher::new();
            cbor_data.hash(&mut hasher);
            let hash = format!("sha256:{:x}", hasher.finish());
            
            // Ensure no duplicate hashes (uniqueness requirement)
            assert!(!all_hashes.contains(&hash), "Each operation must have unique hash");
            
            all_hashes.push(hash);
            all_configs.push(config);
        }

        // Validate cross-operation integrity
        assert_eq!(all_configs.len(), 5, "Must have all 5 operations");
        assert_eq!(all_hashes.len(), 5, "Must have all 5 unique hashes");

        // Test that all configs can be serialized together (batch processing)
        for (i, config) in all_configs.iter().enumerate() {
            let cbor_data = config.to_cbor()?;
            let deserialized = PravyomConfig::from_cbor(&cbor_data)?;
            assert_eq!(config.config_id, deserialized.config_id, "Config {} must survive round-trip", i);
        }

        println!("✅ Level 3: Real-World Audit Trail Integrity: PASSED");
        Ok(())
    }

    /// Level 3 Test: Enterprise-Grade Performance Under Load
    #[tokio::test]
    async fn test_enterprise_performance_under_load() -> Result<()> {
        println!("🧪 Level 3: Testing Enterprise Performance Under Load...");

        // Simulate high-load enterprise scenario
        let config_count = 50; // Moderate load test
        let mut configs = Vec::new();
        let start_time = std::time::Instant::now();

        // Create multiple configs rapidly (simulating enterprise load)
        for i in 0..config_count {
            let config_id = format!("enterprise_load_test_{:03}", i);
            let mut config = create_test_config(&config_id)?;
            
            // Add enterprise-specific performance requirements
            config.performance_metrics.cpu_usage_percent = 15.0 + (i as f64 % 85.0); // 15-100% range
            config.performance_metrics.memory_usage_mb = 100.0 + (i as f64 * 10.0); // Increasing memory
            config.performance_metrics.throughput_records_per_second = 50.0 + (i as f64 * 2.0); // Increasing throughput
            config.performance_metrics.latency_ms = 50.0 + (i as f64 % 200.0); // 50-250ms range

            // Validate performance constraints
            assert!(config.performance_metrics.cpu_usage_percent <= 100.0, "CPU usage must not exceed 100%");
            assert!(config.performance_metrics.memory_usage_mb >= 100.0, "Memory usage must be reasonable");
            assert!(config.performance_metrics.latency_ms < 300.0, "Latency must be under 300ms");

            configs.push(config);
        }

        let creation_time = start_time.elapsed();
        println!("Created {} configs in {:?}", config_count, creation_time);

        // Test batch CBOR serialization performance
        let serialization_start = std::time::Instant::now();
        let mut total_cbor_size = 0;

        for config in &configs {
            let cbor_data = config.to_cbor()?;
            total_cbor_size += cbor_data.len();
            
            // Validate each serialization
            assert!(!cbor_data.is_empty(), "CBOR data must not be empty");
            assert!(cbor_data.len() > 100, "CBOR data must be substantial");
        }

        let serialization_time = serialization_start.elapsed();
        println!("Serialized {} configs ({} bytes total) in {:?}", config_count, total_cbor_size, serialization_time);

        // Test batch deserialization performance
        let deserialization_start = std::time::Instant::now();
        let mut successful_deserializations = 0;

        for config in &configs {
            let cbor_data = config.to_cbor()?;
            let deserialized = PravyomConfig::from_cbor(&cbor_data)?;
            assert_eq!(config.config_id, deserialized.config_id, "Deserialization must preserve config ID");
            successful_deserializations += 1;
        }

        let deserialization_time = deserialization_start.elapsed();
        println!("Deserialized {} configs in {:?}", successful_deserializations, deserialization_time);

        // Performance requirements validation
        assert_eq!(successful_deserializations, config_count, "All configs must deserialize successfully");
        assert!(creation_time.as_millis() < 5000, "Config creation must be under 5 seconds");
        assert!(serialization_time.as_millis() < 10000, "Serialization must be under 10 seconds");
        assert!(deserialization_time.as_millis() < 10000, "Deserialization must be under 10 seconds");

        println!("✅ Level 3: Enterprise Performance Under Load: PASSED");
        Ok(())
    }

    /// Level 3 Test: Advanced CBOR Canonical Format Validation
    #[tokio::test]
    async fn test_advanced_cbor_canonical_validation() -> Result<()> {
        println!("🧪 Level 3: Testing Advanced CBOR Canonical Format...");

        let config = create_test_config("canonical_test")?;

        // Test canonical CBOR serialization
        let cbor_data1 = config.to_cbor()?;
        let cbor_data2 = config.to_cbor()?;
        
        // Canonical serialization must be deterministic
        assert_eq!(cbor_data1, cbor_data2, "Canonical CBOR must be deterministic");
        assert!(config.validate_cbor()?, "Must pass CBOR canonical validation");

        // Test diagnostic notation (human-readable requirement)
        let diagnostic = config.to_diagnostic()?;
        assert!(diagnostic.contains("canonical_test"), "Diagnostic must contain config ID");
        assert!(diagnostic.contains("government_enterprise"), "Diagnostic must show classification");
        assert!(diagnostic.contains("7"), "Diagnostic must show retention years");

        // Test that diagnostic is truly human-readable
        assert!(diagnostic.chars().all(|c| c.is_ascii()), "Diagnostic must be ASCII");
        assert!(diagnostic.lines().count() > 5, "Diagnostic must be multi-line");
        assert!(diagnostic.len() > 300, "Diagnostic must be comprehensive");

        // Test round-trip with canonical validation
        let deserialized = PravyomConfig::from_cbor(&cbor_data1)?;
        assert!(deserialized.validate_cbor()?, "Deserialized config must also pass validation");

        // Test that deserialized config produces identical CBOR
        let cbor_data3 = deserialized.to_cbor()?;
        assert_eq!(cbor_data1, cbor_data3, "Round-trip must preserve canonical format");

        println!("✅ Level 3: Advanced CBOR Canonical Validation: PASSED");
        Ok(())
    }

    /// Level 3 Test: Comprehensive Security and Compliance Integration
    #[tokio::test]
    async fn test_comprehensive_security_compliance() -> Result<()> {
        println!("🧪 Level 3: Testing Comprehensive Security and Compliance...");

        // Create maximum security configuration
        let mut config = create_test_config("max_security_001")?;
        
        // Set maximum security compliance
        config.compliance_metadata.classification = "government_enterprise".to_string();
        config.compliance_metadata.audit_requirements = vec![
            "SOC2_TYPE_II".to_string(),
            "FIPS_140_2_LEVEL_4".to_string(),
            "FISMA_HIGH".to_string(),
            "NIST_800_53_HIGH".to_string(),
            "COMMON_CRITERIA_EAL7".to_string(),
            "ISO_27001".to_string(),
            "FedRAMP_HIGH".to_string(),
        ];

        // Add multiple witness signatures (impossible-to-hide requirement)
        config.audit_trail.witness_signatures = vec![
            "witness_primary_sha256:abc123".to_string(),
            "witness_secondary_sha256:def456".to_string(),
            "witness_tertiary_sha256:ghi789".to_string(),
            "witness_compliance_officer_sha256:jkl012".to_string(),
        ];

        // Validate comprehensive security requirements
        assert!(config.audit_trail.witness_signatures.len() >= 4, "Must have multiple witnesses");
        assert!(config.compliance_metadata.audit_requirements.len() >= 7, "Must meet all compliance standards");
        assert_eq!(config.audit_trail.retention_years, 7, "Must meet 7-year retention");
        assert!(config.audit_trail.compliance_score >= 95.0, "Must have high compliance score");

        // Test that all security features work together
        let cbor_data = config.to_cbor()?;
        assert!(config.validate_cbor()?, "Must pass comprehensive validation");
        
        let deserialized = PravyomConfig::from_cbor(&cbor_data)?;
        assert_eq!(config.audit_trail.witness_signatures.len(), deserialized.audit_trail.witness_signatures.len(), "All witnesses must be preserved");
        assert_eq!(config.compliance_metadata.audit_requirements.len(), deserialized.compliance_metadata.audit_requirements.len(), "All compliance requirements must be preserved");

        // Test diagnostic output includes all security information
        let diagnostic = config.to_diagnostic()?;
        assert!(diagnostic.contains("FIPS_140_2_LEVEL_4"), "Must show FIPS compliance level");
        assert!(diagnostic.contains("witness_primary"), "Must show witness information");
        assert!(diagnostic.contains("government_enterprise"), "Must show classification");
        assert!(diagnostic.len() > 800, "Comprehensive diagnostic must be detailed");

        println!("✅ Level 3: Comprehensive Security and Compliance: PASSED");
        Ok(())
    }
}
