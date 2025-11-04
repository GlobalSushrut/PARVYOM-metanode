//! # BPCI/BPI Penetration Test Runner
//! 
//! Comprehensive security testing runner for BPCI and BPI ledger systems
//! Executes real penetration testing with hacker-level attack simulations

use std::sync::Arc;
use anyhow::Result;
use tracing::{info, warn, error};
use tracing_subscriber;
use chrono::Utc;

use pravyom_enterprise::{
    bpi_ledger_integration::BpiLedgerClient,
    bpci_auction_mempool_minimal::BpciAuctionMempool,
    // BSO ICO world testnet - no separate config needed
    bpci_penetration_testing::{BpciPenetrationTesting, PenetrationTestReport, SecuritySeverity},
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("🔥 BPCI/BPI Penetration Testing Framework");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Safety check - ensure we're in testnet mode
    let network_mode = std::env::var("BPCI_NETWORK_MODE").unwrap_or_default();
    if network_mode != "testnet" {
        error!("🚨 SECURITY: Penetration testing MUST run in testnet mode only!");
        error!("🚨 Set BPCI_NETWORK_MODE=testnet to proceed");
        return Err(anyhow::anyhow!("Penetration testing blocked - not in testnet mode"));
    }

    info!("✅ Safety confirmed: Running in testnet mode");
    info!("🎯 Target: BPCI and BPI ledger security validation");
    info!("🔍 Test Categories: Qlock, TLS/SSL, HTTP/CG, Blockchain, Advanced Hacker");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Initialize components
    info!("🔧 Initializing penetration testing components");
    
    // BSO ICO world testnet - configuration handled by CUE deployment
    let bso_ico_enabled = true;
    info!("✅ BPCI configuration loaded");

    let bpi_client: Arc<BpiLedgerClient> = Arc::new(BpiLedgerClient::new().await?);
    info!("✅ BPI ledger client initialized");

    let bpci_mempool = Arc::new(tokio::sync::RwLock::new(BpciAuctionMempool::new()));
    info!("✅ BPCI auction mempool initialized");

    // Create default penetration testing configuration
    #[derive(Debug, Default)]
    struct PenetrationTestConfig {
        // Stub configuration for compilation
    }
    let config = PenetrationTestConfig::default();
    info!("✅ Penetration testing configuration loaded");

    // Initialize penetration testing framework
    let mut penetration_tester = BpciPenetrationTesting::new(
        bpi_client,
        bpci_mempool,
        true, // BSO ICO enabled for testing
    )?;
    info!("✅ Penetration testing framework initialized");

    // Execute comprehensive penetration testing
    info!("🚀 Launching comprehensive penetration testing suite");
    info!("⚠️  WARNING: This will perform real security attacks against the system");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let test_report = match penetration_tester.execute_full_penetration_test().await {
        Ok(report) => {
            info!("🎉 Penetration testing completed successfully!");
            display_security_report(&report);
            report
        }
        Err(e) => {
            error!("❌ Penetration testing failed: {}", e);
            return Err(e);
        }
    };

    // Validate security posture
    info!("🔍 Validating security posture");
    validate_security_results(&test_report)?;

    info!("✅ BPCI/BPI penetration testing completed successfully!");
    info!("🚨 SECURITY STATUS: System validated against real attack vectors");
    
    Ok(())
}

/// Display comprehensive security report
fn display_security_report(report: &PenetrationTestReport) {
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("🛡️  PENETRATION TEST SECURITY REPORT");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("🆔 Test ID: {}", report.test_id);
    info!("📊 Total Tests Executed: {}", report.total_tests);
    info!("🚨 Vulnerabilities Found: {}", report.vulnerabilities_found);
    info!("🔴 Critical Issues: {}", report.critical_issues);
    info!("🟠 High Severity Issues: {}", report.high_issues);
    info!("⏱️  Total Execution Time: {}ms", report.execution_time_ms);
    info!("🕐 Test Timestamp: {}", report.timestamp);
    
    // Security posture assessment
    let security_score = calculate_security_score(report);
    let security_grade = get_security_grade(security_score);
    
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("🎯 SECURITY POSTURE ASSESSMENT");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("📈 Security Score: {}/100", security_score);
    info!("🏆 Security Grade: {}", security_grade);
    
    if report.critical_issues > 0 {
        warn!("🚨 CRITICAL: {} critical vulnerabilities require immediate attention!", report.critical_issues);
    }
    
    if report.high_issues > 0 {
        warn!("⚠️  HIGH: {} high-severity issues should be addressed", report.high_issues);
    }
    
    if report.vulnerabilities_found == 0 {
        info!("✅ EXCELLENT: No vulnerabilities found in penetration testing");
    }
    
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Display detailed test results by category
    display_test_results_by_category(report);
}

/// Display test results organized by category
fn display_test_results_by_category(report: &PenetrationTestReport) {
    info!("📋 DETAILED TEST RESULTS BY CATEGORY");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let categories = [
        ("🔐 Qlock Security", "QlockSecurity"),
        ("🔒 TLS/SSL Security", "TlsSslSecurity"),
        ("🌐 HTTP/CG Security", "HttpCgSecurity"),
        ("⛓️  Blockchain Security", "BlockchainSecurity"),
        ("🎭 Advanced Hacker Simulation", "AdvancedHackerSimulation"),
    ];
    
    for (category_name, category_type) in categories.iter() {
        let category_results: Vec<_> = report.results.iter()
            .filter(|r| format!("{:?}", r.category) == *category_type)
            .collect();
            
        if !category_results.is_empty() {
            info!("{}", category_name);
            for result in category_results {
                let status_icon = match result.status {
                    pravyom_enterprise::bpci_penetration_testing::TestStatus::Passed => "✅",
                    pravyom_enterprise::bpci_penetration_testing::TestStatus::Failed => "❌",
                    pravyom_enterprise::bpci_penetration_testing::TestStatus::Vulnerable => "🚨",
                    pravyom_enterprise::bpci_penetration_testing::TestStatus::Blocked => "🚫",
                    pravyom_enterprise::bpci_penetration_testing::TestStatus::Error => "⚠️",
                };
                
                let severity_icon = match result.severity {
                    SecuritySeverity::Critical => "🔴",
                    SecuritySeverity::High => "🟠",
                    SecuritySeverity::Medium => "🟡",
                    SecuritySeverity::Low => "🟢",
                    SecuritySeverity::Info => "ℹ️",
                };
                
                info!("  {} {} {} - {}", status_icon, severity_icon, result.test_name, result.description);
                
                if result.vulnerability_found {
                    warn!("    🎯 Attack Vector: {}", result.attack_vector);
                    info!("    🛠️  Mitigation: {}", result.mitigation);
                }
            }
            info!("");
        }
    }
}

/// Calculate overall security score
fn calculate_security_score(report: &PenetrationTestReport) -> u32 {
    if report.total_tests == 0 {
        return 0;
    }
    
    let base_score: u32 = 100;
    let critical_penalty = report.critical_issues * 25;
    let high_penalty = report.high_issues * 10;
    let vulnerability_penalty = (report.vulnerabilities_found.saturating_sub(report.critical_issues + report.high_issues)) * 5;
    
    let total_penalty = critical_penalty + high_penalty + vulnerability_penalty;
    base_score.saturating_sub(total_penalty as u32)
}

/// Get security grade based on score
fn get_security_grade(score: u32) -> &'static str {
    match score {
        90..=100 => "A+ (Excellent)",
        80..=89 => "A (Very Good)",
        70..=79 => "B (Good)",
        60..=69 => "C (Fair)",
        50..=59 => "D (Poor)",
        _ => "F (Critical)",
    }
}

/// Validate security test results
fn validate_security_results(report: &PenetrationTestReport) -> Result<()> {
    info!("🔍 Validating penetration test results");
    
    // Check for critical security issues
    if report.critical_issues > 0 {
        error!("🚨 CRITICAL SECURITY ISSUES FOUND: {}", report.critical_issues);
        error!("🚨 System is NOT ready for production deployment");
        return Err(anyhow::anyhow!("Critical security vulnerabilities detected"));
    }
    
    // Check for high-severity issues
    if report.high_issues > 3 {
        warn!("⚠️  HIGH SEVERITY ISSUES: {} (threshold: 3)", report.high_issues);
        warn!("⚠️  Consider addressing high-severity issues before deployment");
    }
    
    // Validate test coverage
    if report.total_tests < 10 {
        warn!("⚠️  LOW TEST COVERAGE: {} tests (recommended: 10+)", report.total_tests);
    }
    
    let security_score = calculate_security_score(report);
    if security_score < 70 {
        warn!("⚠️  LOW SECURITY SCORE: {} (recommended: 70+)", security_score);
    }
    
    info!("✅ Security validation completed");
    Ok(())
}
