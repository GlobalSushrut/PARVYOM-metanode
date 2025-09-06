//! Comprehensive Test Suite for All 20 CUE Logic Types
//! 
//! This test suite validates that all 20 CUE configuration types can be:
//! 1. Deployed successfully
//! 2. Burned to create immutable addresses
//! 3. Activated for pipeline control
//! 4. Function correctly with their specific logic

use anyhow::{Result, anyhow};
use std::path::Path;
use tokio;
use tracing::{info, error, warn};
use serde_json;

// Import our CUE deployment system
use bpi_core::cue_agreement_deployment::{
    CueAgreementDeploymentManager, 
    DeploymentConfig, 
    AgreementType
};

/// Test configuration for each CUE type
#[derive(Debug, Clone)]
struct CueTypeTest {
    name: &'static str,
    file_path: &'static str,
    agreement_type: AgreementType,
    expected_controls: &'static [&'static str],
}

/// All 20 CUE logic types to test
fn get_cue_types() -> Vec<CueTypeTest> {
    vec![
    // BPI Layer SmartContracts
    CueTypeTest {
        name: "SmartContract - Escrow",
        file_path: "/home/umesh/metanode/bpi-core/contracts/escrow_agreement.cue",
        agreement_type: AgreementType::SmartContract {
            contract_type: "escrow".to_string(),
            bpi_layer: true,
            fiat_payment_enabled: true,
        },
        expected_controls: &["docklock", "storage"],
    },
    CueTypeTest {
        name: "SmartContract - Payment",
        file_path: "/home/umesh/metanode/bpi-core/contracts/escrow_contract.cue",
        agreement_type: AgreementType::SmartContract {
            contract_type: "payment".to_string(),
            bpi_layer: true,
            fiat_payment_enabled: true,
        },
        expected_controls: &["docklock", "storage"],
    },

    // BPCI Layer SmartContract++
    CueTypeTest {
        name: "SmartContract++ - Governance",
        file_path: "/home/umesh/metanode/bpi-core/cue_configs/smartcontracts.cue",
        agreement_type: AgreementType::SmartContractPlusPlus {
            governance_type: "multi_bpi_governance".to_string(),
            multi_bpi_mesh: true,
            jurisdiction: "global".to_string(),
        },
        expected_controls: &["docklock", "enccluster", "biso", "trafficlight"],
    },

    // BPCI Layer Agreement+
    CueTypeTest {
        name: "Agreement+ - Cross-BPI Enforcement",
        file_path: "/home/umesh/metanode/bpi-core/cue_configs/agreements.cue",
        agreement_type: AgreementType::AgreementPlus {
            enforcement_type: "cross_bpi_enforcement".to_string(),
            cross_bpi: true,
            wallet_stamp_authority: true,
        },
        expected_controls: &["enforcement", "cross_bpi", "wallet_stamp"],
    },

    // Infrastructure CUE Types
    CueTypeTest {
        name: "BISO Configuration",
        file_path: "/home/umesh/metanode/bpi-core/cue_configs/biso.cue",
        agreement_type: AgreementType::SmartContractPlusPlus {
            governance_type: "compliance".to_string(),
            multi_bpi_mesh: false,
            jurisdiction: "global".to_string(),
        },
        expected_controls: &["biso"],
    },
    CueTypeTest {
        name: "TrafficLight Configuration",
        file_path: "/home/umesh/metanode/bpi-core/cue_configs/trafficlight.cue",
        agreement_type: AgreementType::SmartContractPlusPlus {
            governance_type: "security".to_string(),
            multi_bpi_mesh: false,
            jurisdiction: "global".to_string(),
        },
        expected_controls: &["trafficlight"],
    },
    CueTypeTest {
        name: "Storage Configuration",
        file_path: "/home/umesh/metanode/bpi-core/cue_configs/storage.cue",
        agreement_type: AgreementType::SmartContract {
            contract_type: "storage".to_string(),
            bpi_layer: true,
            fiat_payment_enabled: false,
        },
        expected_controls: &["storage"],
    },
    CueTypeTest {
        name: "CDNT Configuration",
        file_path: "/home/umesh/metanode/bpi-core/cue_configs/cdnt.cue",
        agreement_type: AgreementType::SmartContract {
            contract_type: "cdn".to_string(),
            bpi_layer: true,
            fiat_payment_enabled: false,
        },
        expected_controls: &["storage"],
    },
    CueTypeTest {
        name: "Security Configuration",
        file_path: "/home/umesh/metanode/bpi-core/cue_configs/security.cue",
        agreement_type: AgreementType::SmartContractPlusPlus {
            governance_type: "security".to_string(),
            multi_bpi_mesh: true,
            jurisdiction: "global".to_string(),
        },
        expected_controls: &["docklock", "enccluster", "biso", "trafficlight"],
    },
    CueTypeTest {
        name: "Firewall Configuration",
        file_path: "/home/umesh/metanode/bpi-core/cue_configs/firewall.cue",
        agreement_type: AgreementType::SmartContractPlusPlus {
            governance_type: "security".to_string(),
            multi_bpi_mesh: true,
            jurisdiction: "global".to_string(),
        },
        expected_controls: &["docklock", "enccluster", "biso", "trafficlight"],
    },
    CueTypeTest {
        name: "Court Configuration",
        file_path: "/home/umesh/metanode/bpi-core/cue_configs/court.cue",
        agreement_type: AgreementType::AgreementPlus {
            enforcement_type: "court_arbitration".to_string(),
            cross_bpi: true,
            wallet_stamp_authority: true,
        },
        expected_controls: &["enforcement"],
    },
    CueTypeTest {
        name: "M2M Configuration",
        file_path: "/home/umesh/metanode/bpi-core/cue_configs/m2m.cue",
        agreement_type: AgreementType::SmartContract {
            contract_type: "m2m".to_string(),
            bpi_layer: true,
            fiat_payment_enabled: false,
        },
        expected_controls: &["docklock"],
    },
    CueTypeTest {
        name: "Gateway Configuration",
        file_path: "/home/umesh/metanode/bpi-core/cue_configs/gateway.cue",
        agreement_type: AgreementType::SmartContract {
            contract_type: "gateway".to_string(),
            bpi_layer: true,
            fiat_payment_enabled: true,
        },
        expected_controls: &["docklock", "storage"],
    },
    CueTypeTest {
        name: "ENC Cluster Configuration",
        file_path: "/home/umesh/metanode/bpi-core/cue_configs/enc.cue",
        agreement_type: AgreementType::SmartContractPlusPlus {
            governance_type: "orchestration".to_string(),
            multi_bpi_mesh: true,
            jurisdiction: "global".to_string(),
        },
        expected_controls: &["enccluster"],
    },
    CueTypeTest {
        name: "HTTPCage Configuration",
        file_path: "/home/umesh/metanode/bpi-core/cue_configs/httpcg.cue",
        agreement_type: AgreementType::SmartContract {
            contract_type: "http_cage".to_string(),
            bpi_layer: true,
            fiat_payment_enabled: false,
        },
        expected_controls: &["docklock"],
    },
    CueTypeTest {
        name: "Nginx Configuration",
        file_path: "/home/umesh/metanode/bpi-core/cue_configs/nignix.cue",
        agreement_type: AgreementType::SmartContract {
            contract_type: "web_server".to_string(),
            bpi_layer: true,
            fiat_payment_enabled: false,
        },
        expected_controls: &["docklock"],
    },
    CueTypeTest {
        name: "VM Orchestration Configuration",
        file_path: "/home/umesh/metanode/bpi-core/cue_configs/vmorc.cue",
        agreement_type: AgreementType::SmartContractPlusPlus {
            governance_type: "vm_orchestration".to_string(),
            multi_bpi_mesh: false,
            jurisdiction: "global".to_string(),
        },
        expected_controls: &["enccluster"],
    },
    CueTypeTest {
        name: "Contracts Configuration",
        file_path: "/home/umesh/metanode/bpi-core/cue_configs/contracts.cue",
        agreement_type: AgreementType::SmartContract {
            contract_type: "general".to_string(),
            bpi_layer: true,
            fiat_payment_enabled: true,
        },
        expected_controls: &["docklock", "storage"],
    },
    CueTypeTest {
        name: "Agreements Configuration",
        file_path: "/home/umesh/metanode/bpi-core/cue_configs/agreements.cue",
        agreement_type: AgreementType::AgreementPlus {
            enforcement_type: "multi_party_agreements".to_string(),
            cross_bpi: true,
            wallet_stamp_authority: true,
        },
        expected_controls: &["enforcement", "cross_bpi"],
    },
    CueTypeTest {
        name: "Minimal Default Configuration",
        file_path: "/home/umesh/metanode/bpi-core/cue_configs/minimal.default",
        agreement_type: AgreementType::SmartContract {
            contract_type: "minimal".to_string(),
            bpi_layer: true,
            fiat_payment_enabled: false,
        },
        expected_controls: &["docklock"],
    },
    ]
}

/// Test result for each CUE type
#[derive(Debug)]
struct TestResult {
    cue_type: String,
    deployment_success: bool,
    deployment_id: Option<String>,
    burn_success: bool,
    agreement_address: Option<String>,
    activation_success: bool,
    controls_verified: bool,
    error_message: Option<String>,
}

/// Main test function
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting comprehensive test of all 20 CUE logic types");
    
    // Initialize deployment manager
    let deployment_manager = CueAgreementDeploymentManager::new(DeploymentConfig::default());
    
    let mut test_results = Vec::new();
    let mut successful_tests = 0;
    let mut failed_tests = 0;
    
    // Test each CUE type
    let cue_types = get_cue_types();
    for (index, cue_test) in cue_types.iter().enumerate() {
        info!("📋 Testing CUE type {}/{}: {}", index + 1, cue_types.len(), cue_test.name);
        
        let result = test_cue_type(&deployment_manager, cue_test).await;
        
        if result.deployment_success && result.burn_success && result.activation_success && result.controls_verified {
            successful_tests += 1;
            info!("✅ {} - PASSED", cue_test.name);
        } else {
            failed_tests += 1;
            error!("❌ {} - FAILED: {}", cue_test.name, 
                result.error_message.as_deref().unwrap_or("Unknown error"));
        }
        
        test_results.push(result);
    }
    
    // Print comprehensive test report
    print_test_report(&test_results, successful_tests, failed_tests).await?;
    
    if failed_tests == 0 {
        info!("🎉 ALL 20 CUE LOGIC TYPES PASSED! System is ready for production.");
        Ok(())
    } else {
        error!("💥 {} tests failed. Please review and fix issues.", failed_tests);
        Err(anyhow!("{} CUE type tests failed", failed_tests))
    }
}

/// Test individual CUE type through full deployment/burn/activation cycle
async fn test_cue_type(
    deployment_manager: &CueAgreementDeploymentManager,
    cue_test: &CueTypeTest,
) -> TestResult {
    let mut result = TestResult {
        cue_type: cue_test.name.to_string(),
        deployment_success: false,
        deployment_id: None,
        burn_success: false,
        agreement_address: None,
        activation_success: false,
        controls_verified: false,
        error_message: None,
    };
    
    // Step 1: Check if CUE file exists
    if !Path::new(cue_test.file_path).exists() {
        result.error_message = Some(format!("CUE file not found: {}", cue_test.file_path));
        return result;
    }
    
    // Step 2: Deploy CUE agreement
    match deployment_manager.deploy_agreement(
        cue_test.file_path,
        cue_test.agreement_type.clone(),
        Some("test_wallet_123".to_string()),
    ).await {
        Ok(deployment_id) => {
            result.deployment_success = true;
            result.deployment_id = Some(deployment_id.clone());
            
            // Step 3: Burn agreement to create address
            match deployment_manager.burn_agreement(&deployment_id, Some("test_signature".to_string())).await {
                Ok(agreement_address) => {
                    result.burn_success = true;
                    result.agreement_address = Some(agreement_address.clone());
                    
                    // Step 4: Activate agreement for pipeline control
                    match deployment_manager.activate_agreement(&agreement_address).await {
                        Ok(_) => {
                            result.activation_success = true;
                            
                            // Step 5: Verify controls
                            match verify_pipeline_controls(&deployment_manager, &agreement_address, cue_test.expected_controls).await {
                                Ok(verified) => {
                                    result.controls_verified = verified;
                                    if !verified {
                                        result.error_message = Some("Pipeline controls verification failed".to_string());
                                    }
                                },
                                Err(e) => {
                                    result.error_message = Some(format!("Controls verification error: {}", e));
                                }
                            }
                        },
                        Err(e) => {
                            result.error_message = Some(format!("Activation failed: {}", e));
                        }
                    }
                },
                Err(e) => {
                    result.error_message = Some(format!("Burn failed: {}", e));
                }
            }
        },
        Err(e) => {
            result.error_message = Some(format!("Deployment failed: {}", e));
        }
    }
    
    result
}

/// Verify that burned agreement has correct pipeline controls
async fn verify_pipeline_controls(
    deployment_manager: &CueAgreementDeploymentManager,
    agreement_address: &str,
    expected_controls: &[&str],
) -> Result<bool> {
    // Get burned agreement details
    let burned_agreement = deployment_manager.get_agreement_by_address(agreement_address).await?;
    
    // Check if all expected controls are present
    info!("🔍 Verifying controls for agreement: {}", agreement_address);
    info!("📋 Expected controls: {:?}", expected_controls);
    info!("🎛️ Actual controlled_components: {:?}", burned_agreement.component_controls.controlled_components);
    
    for expected_control in expected_controls {
        if !burned_agreement.component_controls.controlled_components.contains(&expected_control.to_string()) {
            warn!("❌ Missing expected control: {} (not found in {:?})", expected_control, burned_agreement.component_controls.controlled_components);
            return Ok(false);
        }
    }
    
    // Verify pipeline permissions match agreement type
    let permissions = &burned_agreement.pipeline_permissions;
    match &burned_agreement.agreement_type {
        AgreementType::SmartContract { .. } => {
            if !permissions.can_control_docklock || !permissions.can_control_storage {
                return Ok(false);
            }
        },
        AgreementType::SmartContractPlusPlus { .. } => {
            if !permissions.can_control_docklock || !permissions.can_control_enccluster || 
               !permissions.can_control_biso || !permissions.can_control_trafficlight ||
               !permissions.can_control_security {
                return Ok(false);
            }
        },
        AgreementType::AgreementPlus { .. } => {
            // Agreement+ should have comprehensive controls
            if !permissions.can_control_security || !permissions.can_control_networking {
                return Ok(false);
            }
        }
    }
    
    Ok(true)
}

/// Print comprehensive test report
async fn print_test_report(
    test_results: &[TestResult],
    successful_tests: usize,
    failed_tests: usize,
) -> Result<()> {
    println!("\n🔍 COMPREHENSIVE CUE LOGIC TYPES TEST REPORT");
    println!("{}", "=".repeat(80));
    println!("📊 Total CUE Types Tested: {}", test_results.len());
    println!("✅ Successful Tests: {}", successful_tests);
    println!("❌ Failed Tests: {}", failed_tests);
    println!("📈 Success Rate: {:.1}%", (successful_tests as f64 / test_results.len() as f64) * 100.0);
    println!();
    
    // Detailed results
    println!("📋 DETAILED TEST RESULTS:");
    println!("{}", "-".repeat(80));
    
    for (index, result) in test_results.iter().enumerate() {
        let status = if result.deployment_success && result.burn_success && 
                        result.activation_success && result.controls_verified {
            "✅ PASS"
        } else {
            "❌ FAIL"
        };
        
        println!("{:2}. {} - {}", index + 1, result.cue_type, status);
        
        if let Some(deployment_id) = &result.deployment_id {
            println!("    📦 Deployment ID: {}", deployment_id);
        }
        
        if let Some(address) = &result.agreement_address {
            println!("    🔥 Burned Address: {}", address);
        }
        
        if let Some(error) = &result.error_message {
            println!("    💥 Error: {}", error);
        }
        
        println!("    🔄 Deploy: {} | 🔥 Burn: {} | ⚡ Activate: {} | 🎛️ Controls: {}",
            if result.deployment_success { "✅" } else { "❌" },
            if result.burn_success { "✅" } else { "❌" },
            if result.activation_success { "✅" } else { "❌" },
            if result.controls_verified { "✅" } else { "❌" }
        );
        println!();
    }
    
    // Summary by agreement type
    println!("📊 RESULTS BY AGREEMENT TYPE:");
    println!("{}", "-".repeat(80));
    
    let smartcontract_results: Vec<_> = test_results.iter()
        .filter(|r| r.cue_type.contains("SmartContract") && !r.cue_type.contains("++"))
        .collect();
    let smartcontract_plus_results: Vec<_> = test_results.iter()
        .filter(|r| r.cue_type.contains("SmartContract++") || r.cue_type.contains("BISO") || 
                    r.cue_type.contains("TrafficLight") || r.cue_type.contains("Security") ||
                    r.cue_type.contains("ENC"))
        .collect();
    let agreement_plus_results: Vec<_> = test_results.iter()
        .filter(|r| r.cue_type.contains("Agreement+") || r.cue_type.contains("Court"))
        .collect();
    
    println!("🔷 SmartContract (BPI Layer): {}/{} passed", 
        smartcontract_results.iter().filter(|r| r.deployment_success && r.burn_success && r.activation_success && r.controls_verified).count(),
        smartcontract_results.len());
    
    println!("🔶 SmartContract++ (BPCI Layer): {}/{} passed", 
        smartcontract_plus_results.iter().filter(|r| r.deployment_success && r.burn_success && r.activation_success && r.controls_verified).count(),
        smartcontract_plus_results.len());
    
    println!("🔸 Agreement+ (BPCI Layer): {}/{} passed", 
        agreement_plus_results.iter().filter(|r| r.deployment_success && r.burn_success && r.activation_success && r.controls_verified).count(),
        agreement_plus_results.len());
    
    println!("\n🎯 CONCLUSION:");
    if failed_tests == 0 {
        println!("🎉 ALL 20 CUE LOGIC TYPES ARE WORKING PERFECTLY!");
        println!("🚀 System is ready for production deployment.");
        println!("💪 Deploy → Burn → Activate → Control pipeline is fully functional.");
    } else {
        println!("⚠️  {} tests need attention before production deployment.", failed_tests);
        println!("🔧 Please review failed tests and fix issues.");
    }
    
    Ok(())
}
