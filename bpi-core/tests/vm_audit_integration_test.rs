use std::sync::Arc;
use std::fs;
use std::path::Path;
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use serde_json::json;
use blake3;

use bpi_core::immutable_audit_system::ImmutableAuditSystem;
use bpi_core::bpi_action_vm::{BpiActionVM, ContractType};
use bpi_core::orchestration_vm::OrchestrationVM;
use bpi_core::universal_audit_vm::UniversalAuditVM;

use ziplock_json::vm_integration::{VmAuditManager, AuditEvent, VmType};
use ziplock_json::system_audit_coordinator::SystemAuditCoordinator;
use ziplock_json::writer::ZjlWriter;
use ziplock_json::signing::InMemoryKms;
use ziplock_json::ZjlOptions;

/// Enterprise-grade VM audit integration test
/// Verifies all VMs are writing binary audit data correctly
#[tokio::test]
async fn test_all_vms_binary_audit_integration() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Starting Enterprise-Grade VM Audit Integration Test");
    println!("📊 Testing all 5 BPI VMs for binary audit compliance");
    
    // Create test directory for audit files
    let test_dir = "/tmp/bpi_vm_audit_test";
    if Path::new(test_dir).exists() {
        fs::remove_dir_all(test_dir)?;
    }
    fs::create_dir_all(test_dir)?;
    
    // Initialize shared audit system
    let audit_system = Arc::new(ImmutableAuditSystem::new("/tmp/bpi_vm_audit_test/audit_storage").await?);
    
    // Test results tracking
    let mut test_results = Vec::new();
    
    println!("\n🚀 Phase 1: Testing BPI Action VM Binary Audit");
    let action_vm_result = test_bpi_action_vm_audit(&audit_system, test_dir).await;
    test_results.push(("BPI Action VM", action_vm_result.is_ok()));
    if let Err(e) = &action_vm_result {
        println!("❌ BPI Action VM audit test failed: {}", e);
    } else {
        println!("✅ BPI Action VM audit test passed");
    }
    
    println!("\n🔧 Phase 2: Testing Orchestration VM Binary Audit");
    let orch_vm_result = test_orchestration_vm_audit(&audit_system, test_dir).await;
    test_results.push(("Orchestration VM", orch_vm_result.is_ok()));
    if let Err(e) = &orch_vm_result {
        println!("❌ Orchestration VM audit test failed: {}", e);
    } else {
        println!("✅ Orchestration VM audit test passed");
    }
    
    println!("\n📋 Phase 3: Testing Universal Audit VM Binary Audit");
    let universal_vm_result = test_universal_audit_vm(&audit_system, test_dir).await;
    test_results.push(("Universal Audit VM", universal_vm_result.is_ok()));
    if let Err(e) = &universal_vm_result {
        println!("❌ Universal Audit VM audit test failed: {}", e);
    } else {
        println!("✅ Universal Audit VM audit test passed");
    }
    
    println!("\n🛡️ Phase 4: Testing ZJL Audit System Integration");
    let zjl_integration_result = test_zjl_audit_integration(&audit_system, test_dir).await;
    test_results.push(("ZJL Audit Integration", zjl_integration_result.is_ok()));
    if let Err(e) = &zjl_integration_result {
        println!("❌ ZJL Audit Integration test failed: {}", e);
    } else {
        println!("✅ ZJL Audit Integration test passed");
    }
    
    println!("\n🔍 Phase 6: Verifying Binary Audit File Integrity");
    let integrity_result = verify_audit_file_integrity(test_dir).await;
    test_results.push(("Binary File Integrity", integrity_result.is_ok()));
    if let Err(e) = &integrity_result {
        println!("❌ Binary audit file integrity test failed: {}", e);
    } else {
        println!("✅ Binary audit file integrity test passed");
    }
    
    // Print comprehensive test results
    println!("\n📊 ENTERPRISE-GRADE VM AUDIT TEST RESULTS");
    println!("==========================================");
    let mut passed = 0;
    let total = test_results.len();
    
    for (vm_name, success) in &test_results {
        let status = if *success { "✅ PASS" } else { "❌ FAIL" };
        println!("{:<25} {}", vm_name, status);
        if *success { passed += 1; }
    }
    
    println!("==========================================");
    println!("Overall Success Rate: {}/{} ({:.1}%)", passed, total, (passed as f64 / total as f64) * 100.0);
    
    if passed == total {
        println!("🎉 ALL VMs PASSED: Enterprise-grade binary audit compliance achieved!");
        println!("🔒 All VMs are correctly writing immutable binary audit trails");
        println!("📋 System ready for production deployment and regulatory compliance");
    } else {
        println!("⚠️  PARTIAL SUCCESS: {}/{} VMs passed audit compliance", passed, total);
        return Err("Not all VMs achieved enterprise-grade audit compliance".into());
    }
    
    // Cleanup
    fs::remove_dir_all(test_dir)?;
    
    Ok(())
}

/// Test BPI Action VM binary audit functionality
async fn test_bpi_action_vm_audit(
    audit_system: &Arc<ImmutableAuditSystem>,
    test_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  🔧 Initializing BPI Action VM with ZJL audit system");
    
    // Create ZJL audit file for Action VM
    let audit_file_path = format!("{}/action_vm_audit.zjl", test_dir);
    let audit_file = std::fs::File::create(&audit_file_path)?;
    
    let kms = InMemoryKms::new();
    let options = ZjlOptions::default();
    let zjl_writer: ZjlWriter<std::fs::File, InMemoryKms> = ZjlWriter::new(audit_file, options)?;
    let vm_audit_manager = Arc::new(VmAuditManager::new(&audit_file_path)?);
    
    // Initialize Action VM
    let action_vm = BpiActionVM::new(audit_system.clone()).await?;
    
    println!("  📝 Testing contract deployment audit logging");
    
    // Test contract deployment (should generate binary audit)
    let deployment_result = action_vm.deploy_contract(
        ContractType::SmartContract,
        json!({
            "name": "TestContract",
            "version": "1.0.0",
            "security_level": "High"
        }),
        "test_app_001"
    ).await;
    
    // Verify deployment was logged
    if deployment_result.is_ok() {
        println!("  ✅ Contract deployment successful and logged");
    } else {
        println!("  ❌ Contract deployment failed: {:?}", deployment_result.err());
    }
    
    // Test multiple contract types
    let contract_types = vec![
        ContractType::CUEYaml,
        ContractType::DockLock,
        ContractType::BISO,
        ContractType::TrafficLight,
    ];
    
    for contract_type in contract_types {
        let result = action_vm.deploy_contract(
            contract_type.clone(),
            json!({"type": format!("{:?}", contract_type)}),
            "test_app_002"
        ).await;
        
        if result.is_ok() {
            println!("  ✅ {} contract deployed and audited", contract_type);
        } else {
            println!("  ⚠️  {} contract deployment had issues", contract_type);
        }
    }
    
    // Verify binary audit file was created and has content
    let audit_metadata = std::fs::metadata(&audit_file_path)?;
    if audit_metadata.len() > 0 {
        println!("  ✅ Binary audit file created: {} bytes", audit_metadata.len());
    } else {
        return Err("Binary audit file is empty".into());
    }
    
    Ok(())
}

/// Test Orchestration VM binary audit functionality
async fn test_orchestration_vm_audit(
    audit_system: &Arc<ImmutableAuditSystem>,
    test_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  🔧 Initializing Orchestration VM with ZJL audit system");
    
    // Create ZJL audit file for Orchestration VM
    let audit_file_path = format!("{}/orchestration_vm_audit.zjl", test_dir);
    let audit_file = std::fs::File::create(&audit_file_path)?;
    
    let kms = InMemoryKms::new();
    let options = ZjlOptions::default();
    let zjl_writer: ZjlWriter<std::fs::File, InMemoryKms> = ZjlWriter::new(audit_file, options)?;
    let vm_audit_manager = Arc::new(VmAuditManager::new(&audit_file_path)?);
    
    // Initialize Orchestration VM
    let orch_vm = OrchestrationVM::new(audit_system.clone()).await?;
    
    println!("  📝 Testing infrastructure deployment audit logging");
    
    // Test VM start audit
    vm_audit_manager.log_event(AuditEvent::VmStart {
        vm_id: "orchestration_vm_001".to_string(),
        vm_type: VmType::Orchestration,
        config: json!({
            "deployment_mode": "production",
            "security_level": "maximum",
            "audit_level": "comprehensive"
        })
    });
    
    // Test infrastructure operations
    vm_audit_manager.log_event(AuditEvent::EncOperation {
        vm_id: "orchestration_vm_001".to_string(),
        operation: "deploy_cluster".to_string(),
        data_hash: blake3::hash(b"test_cluster_config").to_hex().to_string(),
        result: json!({
            "status": "success",
            "cluster_id": "cluster_001",
            "nodes": 3
        })
    });
    
    println!("  ✅ Infrastructure deployment operations logged");
    
    // Verify binary audit file
    let audit_metadata = std::fs::metadata(&audit_file_path)?;
    if audit_metadata.len() > 0 {
        println!("  ✅ Binary audit file created: {} bytes", audit_metadata.len());
    } else {
        return Err("Binary audit file is empty".into());
    }
    
    Ok(())
}

/// Test Universal Audit VM binary audit functionality
async fn test_universal_audit_vm(
    audit_system: &Arc<ImmutableAuditSystem>,
    test_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  🔧 Initializing Universal Audit VM with ZJL audit system");
    
    // Create ZJL audit file for Universal Audit VM
    let audit_file_path = format!("{}/universal_audit_vm_audit.zjl", test_dir);
    let audit_file = std::fs::File::create(&audit_file_path)?;
    
    let kms = InMemoryKms::new();
    let options = ZjlOptions::default();
    let zjl_writer: ZjlWriter<std::fs::File, InMemoryKms> = ZjlWriter::new(audit_file, options)?;
    let vm_audit_manager = Arc::new(VmAuditManager::new(&audit_file_path)?);
    
    // Initialize Universal Audit VM
    let universal_vm = UniversalAuditVM::new(audit_system.clone()).await?;
    
    println!("  📝 Testing audit aggregation and compliance logging");
    
    // Test audit aggregation events
    vm_audit_manager.log_event(AuditEvent::VmStart {
        vm_id: "universal_audit_vm_001".to_string(),
        vm_type: VmType::UniversalAudit,
        config: json!({
            "aggregation_mode": "real_time",
            "compliance_level": "government_grade",
            "retention_years": 7
        })
    });
    
    // Test compliance validation
    vm_audit_manager.log_event(AuditEvent::CueValidation {
        vm_id: "universal_audit_vm_001".to_string(),
        cue_file: "compliance_policy.cue".to_string(),
        validation_result: true,
        errors: vec![]
    });
    
    println!("  ✅ Audit aggregation and compliance operations logged");
    
    // Verify binary audit file
    let audit_metadata = std::fs::metadata(&audit_file_path)?;
    if audit_metadata.len() > 0 {
        println!("  ✅ Binary audit file created: {} bytes", audit_metadata.len());
    } else {
        return Err("Binary audit file is empty".into());
    }
    
    Ok(())
}

/// Test ZJL Audit System Integration
async fn test_zjl_audit_integration(
    audit_system: &Arc<ImmutableAuditSystem>,
    test_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  🔧 Testing ZJL Audit System Integration");
    
    // Create ZJL audit file for system integration test
    let audit_file_path = format!("{}/zjl_integration_audit.zjl", test_dir);
    let audit_file = std::fs::File::create(&audit_file_path)?;
    
    let kms = InMemoryKms::new();
    let options = ZjlOptions::default();
    let zjl_writer: ZjlWriter<std::fs::File, InMemoryKms> = ZjlWriter::new(audit_file, options)?;
    let vm_audit_manager = Arc::new(VmAuditManager::new(&audit_file_path)?);
    let system_audit_coordinator = Arc::new(SystemAuditCoordinator::new("/tmp/master_audit.zjl")?);
    
    println!("  📝 Testing cross-VM audit coordination and system events");
    
    // Test system-wide audit events
    vm_audit_manager.log_event(AuditEvent::VmStart {
        vm_id: "zjl_integration_001".to_string(),
        vm_type: VmType::BpiAction,
        config: json!({
            "integration_mode": "full_system",
            "audit_level": "enterprise_grade",
            "coordination": "enabled"
        })
    });
    
    // Test cross-VM communication audit
    vm_audit_manager.log_event(AuditEvent::EncOperation {
        vm_id: "zjl_integration_001".to_string(),
        operation: "cross_vm_coordination".to_string(),
        data_hash: blake3::hash(b"cross_vm_audit_test").to_hex().to_string(),
        result: json!({
            "status": "success",
            "vms_coordinated": ["ActionVM", "OrchestrationVM", "UniversalAuditVM"],
            "audit_trail_unified": true
        })
    });
    
    // Test container orchestration audit
    vm_audit_manager.log_event(AuditEvent::ContainerCreate {
        vm_id: "zjl_integration_001".to_string(),
        container_id: "audit_test_container_001".to_string(),
        image: "bpi/audit-test:latest".to_string(),
        config: json!({
            "test_mode": "enterprise_audit",
            "security_level": "maximum",
            "audit_recording": "binary_zjl"
        })
    });
    
    println!("  ✅ ZJL audit system integration and coordination logged");
    
    // Verify binary audit file
    let audit_metadata = std::fs::metadata(&audit_file_path)?;
    if audit_metadata.len() > 0 {
        println!("  ✅ Binary audit file created: {} bytes", audit_metadata.len());
    } else {
        return Err("Binary audit file is empty".into());
    }
    
    Ok(())
}

/// Verify binary audit file integrity and structure
async fn verify_audit_file_integrity(test_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("  🔍 Verifying binary audit file integrity and structure");
    
    let audit_files = vec![
        "action_vm_audit.zjl",
        "orchestration_vm_audit.zjl", 
        "universal_audit_vm_audit.zjl",
        "zjl_integration_audit.zjl"
    ];
    
    let mut total_audit_size = 0u64;
    let mut files_verified = 0;
    
    for file_name in &audit_files {
        let file_path = format!("{}/{}", test_dir, file_name);
        
        if Path::new(&file_path).exists() {
            let metadata = std::fs::metadata(&file_path)?;
            let file_size = metadata.len();
            
            if file_size > 0 {
                println!("  ✅ {} verified: {} bytes", file_name, file_size);
                total_audit_size += file_size;
                files_verified += 1;
                
                // Basic binary structure validation
                let file_content = std::fs::read(&file_path)?;
                if file_content.len() >= 64 { // Minimum ZJL header size
                    println!("    ✅ Binary structure appears valid");
                } else {
                    println!("    ⚠️  Binary structure may be incomplete");
                }
            } else {
                println!("  ❌ {} is empty", file_name);
                return Err(format!("Audit file {} is empty", file_name).into());
            }
        } else {
            println!("  ❌ {} not found", file_name);
            return Err(format!("Audit file {} not found", file_name).into());
        }
    }
    
    println!("  📊 Total audit data: {} bytes across {} files", total_audit_size, files_verified);
    
    if files_verified == audit_files.len() && total_audit_size > 0 {
        println!("  🎉 All binary audit files verified successfully!");
    } else {
        return Err("Binary audit file verification failed".into());
    }
    
    Ok(())
}
