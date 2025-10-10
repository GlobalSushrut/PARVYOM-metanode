// Government Integration Module Tests
// Comprehensive testing of government API integration and dual-transaction system

use tokio;
use anyhow::Result;
use serde_json;
use bpi_core::government_integration::*;

#[cfg(test)]
mod government_integration_tests {
    use super::*;
    use bpi_core::government_integration::*;

    #[tokio::test]
    async fn test_government_integration_system_creation() -> Result<()> {
        println!("🏛️ Testing Government Integration System Creation...");
        
        let config = GovernmentConfig::default();
        let system = GovernmentIntegrationSystem::new(config).await?;
        
        // Test initialization
        system.initialize().await?;
        
        // Get status
        let status = system.get_integration_status().await?;
        assert_eq!(status.total_transactions_processed, 0);
        assert_eq!(status.active_government_connections, 0);
        
        // Test shutdown
        system.shutdown().await?;
        
        println!("✅ Government Integration System creation test passed");
        Ok(())
    }

    #[tokio::test]
    async fn test_government_session_management() -> Result<()> {
        println!("🔐 Testing Government Session Management...");
        
        let config = GovernmentConfig::default();
        let system = GovernmentIntegrationSystem::new(config).await?;
        system.initialize().await?;
        
        // Establish government session
        let session_id = system.establish_government_session(
            "US Treasury".to_string(),
            "US".to_string(),
            SecurityClearance::Secret,
        ).await?;
        
        assert!(!session_id.is_empty());
        
        // Check active sessions
        let sessions = system.get_active_sessions().await?;
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].government_entity, "US Treasury");
        assert_eq!(sessions[0].jurisdiction, "US");
        
        // Check integration status
        let status = system.get_integration_status().await?;
        assert_eq!(status.active_government_connections, 1);
        
        system.shutdown().await?;
        println!("✅ Government session management test passed");
        Ok(())
    }

    #[tokio::test]
    async fn test_dual_transaction_processing() -> Result<()> {
        println!("⚡ Testing Dual Transaction Processing...");
        
        let config = GovernmentConfig::default();
        let system = GovernmentIntegrationSystem::new(config).await?;
        system.initialize().await?;
        
        // Establish session
        let session_id = system.establish_government_session(
            "US SEC".to_string(),
            "US".to_string(),
            SecurityClearance::Confidential,
        ).await?;
        
        // Process dual transaction
        let transaction_data = serde_json::json!({
            "jurisdiction": "US",
            "operation_type": "securities_reporting",
            "amount": 5000.0,
            "currency": "USD",
            "transaction_id": "test_tx_001"
        });
        
        let transaction_id = system.process_dual_transaction(
            &session_id,
            transaction_data,
        ).await?;
        
        assert!(!transaction_id.is_empty());
        
        // Check status after transaction
        let status = system.get_integration_status().await?;
        assert_eq!(status.total_transactions_processed, 1);
        
        system.shutdown().await?;
        println!("✅ Dual transaction processing test passed");
        Ok(())
    }

    #[tokio::test]
    async fn test_government_api_client() -> Result<()> {
        println!("🔌 Testing Government API Client...");
        
        let config = GovernmentConfig::default();
        let client = GovernmentAPIClient::new(config).await?;
        client.initialize().await?;
        
        // Submit transaction to government API
        let transaction_data = serde_json::json!({
            "amount": 1000.0,
            "currency": "USD",
            "type": "transfer"
        });
        
        let response = client.submit_transaction(
            "US",
            "compliance_check",
            transaction_data,
        ).await?;
        
        assert!(response.success);
        assert_eq!(response.status_code, 200);
        assert!(!response.compliance_markers.is_empty());
        assert!(!response.audit_trail_id.is_empty());
        
        // Check client statistics
        let stats = client.get_client_statistics().await?;
        assert_eq!(stats.total_requests, 1);
        assert_eq!(stats.successful_requests, 1);
        
        client.shutdown().await?;
        println!("✅ Government API client test passed");
        Ok(())
    }

    #[tokio::test]
    async fn test_dual_transaction_manager() -> Result<()> {
        println!("⚡ Testing Dual Transaction Manager...");
        
        let config = GovernmentConfig::default();
        let manager = DualTransactionManager::new(config).await?;
        manager.initialize().await?;
        
        // Process dual transaction
        let transaction_data = serde_json::json!({
            "jurisdiction": "US",
            "operation_type": "transfer",
            "amount": 2500.0,
            "currency": "USD"
        });
        
        let pair_id = manager.process_dual_transaction(
            "test_session",
            transaction_data,
        ).await?;
        
        assert!(!pair_id.is_empty());
        
        // Check manager statistics
        let stats = manager.get_manager_statistics().await?;
        assert_eq!(stats.total_transactions_processed, 1);
        assert_eq!(stats.successful_transactions, 1);
        assert!(stats.average_processing_time_ms > 0.0);
        
        manager.shutdown().await?;
        println!("✅ Dual transaction manager test passed");
        Ok(())
    }

    #[tokio::test]
    async fn test_compliance_validator() -> Result<()> {
        println!("📋 Testing Compliance Validator...");
        
        let config = GovernmentConfig::default();
        let validator = ComplianceValidator::new(config).await?;
        validator.initialize().await?;
        
        // Test compliant transaction
        let compliant_transaction = serde_json::json!({
            "transaction_id": "test_compliant_001",
            "amount": 5000.0,
            "currency": "USD",
            "consent_given": true
        });
        
        let result = validator.validate_transaction(
            &compliant_transaction,
            "US",
        ).await?;
        
        assert!(result.is_compliant);
        assert_eq!(result.compliance_score, 1.0);
        assert!(result.violations.is_empty());
        
        // Test non-compliant transaction (exceeds AML limit)
        let non_compliant_transaction = serde_json::json!({
            "transaction_id": "test_non_compliant_001",
            "amount": 15000.0,  // Exceeds 10,000 AML limit
            "currency": "USD"
        });
        
        let result = validator.validate_transaction(
            &non_compliant_transaction,
            "US",
        ).await?;
        
        assert!(!result.is_compliant);
        assert_eq!(result.compliance_score, 0.5);
        assert!(!result.violations.is_empty());
        
        // Check validator statistics
        let stats = validator.get_validator_statistics().await?;
        assert_eq!(stats.total_validations, 2);
        assert_eq!(stats.compliant_validations, 1);
        assert_eq!(stats.non_compliant_validations, 1);
        
        validator.shutdown().await?;
        println!("✅ Compliance validator test passed");
        Ok(())
    }

    #[tokio::test]
    async fn test_audit_trail_manager() -> Result<()> {
        println!("📊 Testing Audit Trail Manager...");
        
        let config = GovernmentConfig::default();
        let manager = AuditTrailManager::new(config).await?;
        manager.initialize().await?;
        
        // Create test session
        let session = GovernmentSession {
            session_id: "test_audit_session".to_string(),
            government_entity: "EU GDPR Authority".to_string(),
            jurisdiction: "EU".to_string(),
            security_clearance: SecurityClearance::Restricted,
            established_at: 1234567890,
            last_activity: 1234567890,
            transaction_count: 1,
            compliance_status: ComplianceStatus::Compliant,
        };
        
        // Record government transaction
        let transaction_data = serde_json::json!({
            "amount": 750.0,
            "currency": "EUR",
            "data_protection_consent": true
        });
        
        let entry_id = manager.record_government_transaction(
            "test_audit_transaction",
            &session,
            &transaction_data,
        ).await?;
        
        assert!(!entry_id.is_empty());
        
        // Get audit trail
        let trail = manager.get_audit_trail("test_audit_session").await?;
        assert_eq!(trail.len(), 1);
        assert_eq!(trail[0].transaction_id, "test_audit_transaction");
        assert_eq!(trail[0].jurisdiction, "EU");
        
        // Generate compliance report
        let report = manager.generate_compliance_report(Some("EU".to_string())).await?;
        assert_eq!(report.total_transactions, 1);
        assert!(report.compliance_score > 0.0);
        assert!(!report.recommendations.is_empty());
        
        // Check manager statistics
        let stats = manager.get_manager_statistics().await?;
        assert!(stats.total_audit_entries > 0);
        assert!(stats.total_events > 0);
        
        manager.shutdown().await?;
        println!("✅ Audit trail manager test passed");
        Ok(())
    }

    #[tokio::test]
    async fn test_compliance_report_generation() -> Result<()> {
        println!("📋 Testing Compliance Report Generation...");
        
        let config = GovernmentConfig::default();
        let system = GovernmentIntegrationSystem::new(config).await?;
        system.initialize().await?;
        
        // Generate compliance report
        let report = system.generate_compliance_report(Some("US".to_string())).await?;
        
        assert!(!report.report_id.is_empty());
        assert_eq!(report.jurisdiction, Some("US".to_string()));
        assert!(report.compliance_score >= 0.0 && report.compliance_score <= 1.0);
        assert!(!report.recommendations.is_empty());
        
        system.shutdown().await?;
        println!("✅ Compliance report generation test passed");
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_jurisdiction_support() -> Result<()> {
        println!("🌍 Testing Multi-Jurisdiction Support...");
        
        let config = GovernmentConfig::default();
        let system = GovernmentIntegrationSystem::new(config).await?;
        system.initialize().await?;
        
        // Test US jurisdiction
        let us_session = system.establish_government_session(
            "US Treasury".to_string(),
            "US".to_string(),
            SecurityClearance::Secret,
        ).await?;
        
        // Test EU jurisdiction
        let eu_session = system.establish_government_session(
            "EU GDPR Authority".to_string(),
            "EU".to_string(),
            SecurityClearance::Restricted,
        ).await?;
        
        // Check multiple active sessions
        let sessions = system.get_active_sessions().await?;
        assert_eq!(sessions.len(), 2);
        
        let jurisdictions: Vec<String> = sessions.iter()
            .map(|s| s.jurisdiction.clone())
            .collect();
        assert!(jurisdictions.contains(&"US".to_string()));
        assert!(jurisdictions.contains(&"EU".to_string()));
        
        system.shutdown().await?;
        println!("✅ Multi-jurisdiction support test passed");
        Ok(())
    }
}

#[tokio::test]
async fn test_government_integration_comprehensive() -> Result<()> {
    println!("\n🏛️ === GOVERNMENT INTEGRATION COMPREHENSIVE TEST ===");
    println!("Testing complete government integration workflow...\n");
    
    let config = GovernmentConfig::default();
    let system = GovernmentIntegrationSystem::new(config).await?;
    system.initialize().await?;
    
    // 1. Establish government session
    println!("1. Establishing government session...");
    let session_id = system.establish_government_session(
        "US Treasury Department".to_string(),
        "US".to_string(),
        SecurityClearance::Secret,
    ).await?;
    println!("   ✅ Session established: {}", session_id);
    
    // 2. Process dual transaction with compliance validation
    println!("2. Processing dual transaction...");
    let transaction_data = serde_json::json!({
        "jurisdiction": "US",
        "operation_type": "securities_reporting",
        "amount": 7500.0,
        "currency": "USD",
        "transaction_id": "comprehensive_test_001",
        "compliance_required": true
    });
    
    let transaction_id = system.process_dual_transaction(
        &session_id,
        transaction_data,
    ).await?;
    println!("   ✅ Transaction processed: {}", transaction_id);
    
    // 3. Generate compliance report
    println!("3. Generating compliance report...");
    let report = system.generate_compliance_report(Some("US".to_string())).await?;
    println!("   ✅ Report generated: {} (score: {:.2})", 
        report.report_id, report.compliance_score);
    
    // 4. Check final system status
    println!("4. Checking system status...");
    let status = system.get_integration_status().await?;
    println!("   ✅ Transactions processed: {}", status.total_transactions_processed);
    println!("   ✅ Active connections: {}", status.active_government_connections);
    println!("   ✅ Compliance score: {:.2}", status.compliance_score);
    
    system.shutdown().await?;
    
    println!("\n🎉 GOVERNMENT INTEGRATION COMPREHENSIVE TEST PASSED!");
    println!("All government integration components working correctly.\n");
    
    Ok(())
}
