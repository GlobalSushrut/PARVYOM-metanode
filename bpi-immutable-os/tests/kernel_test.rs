use anyhow::Result;
use tokio;

use bpi_immutable_os::blockchain_os_kernel::{
    BlockchainOSKernel, 
    ProcessType,
    ResourceAllocation,
    SecurityContext,
    SecurityLevel,
    Permission
};

#[tokio::test]
async fn test_kernel_basic_functionality() -> Result<()> {
    println!("🚀 Testing BPI OS Kernel Basic Functionality");
    
    // Initialize kernel
    let kernel = BlockchainOSKernel::new().await?;
    println!("✅ Kernel initialized");
    
    // Start kernel
    kernel.start().await?;
    println!("✅ Kernel started");
    
    // Create a process with correct API
    let security_context = SecurityContext {
        security_level: SecurityLevel::Public,
        permissions: vec![Permission::Read, Permission::Write],
        quantum_encryption: true,
        audit_logging: true,
    };
    
    let resource_allocation = ResourceAllocation {
        cpu_percent: 10.0,
        memory_bytes: 1024 * 1024 * 512, // 512 MB
        network_bandwidth: 1024 * 1024,  // 1 MB/s
        storage_bytes: 1024 * 1024 * 1024, // 1 GB
    };
    
    let process_id = kernel.create_process(
        "test_process".to_string(),
        ProcessType::UserProcess,
        resource_allocation,
        security_context
    ).await?;
    println!("✅ Process created: {}", process_id);
    
    // Get kernel status
    let status = kernel.get_kernel_status().await?;
    println!("✅ Status - Active processes: {}", status.active_processes);
    println!("✅ Status - CPU utilization: {:.2}%", status.performance_metrics.cpu_utilization);
    
    // Shutdown kernel
    kernel.shutdown().await?;
    println!("✅ Kernel shutdown");
    
    println!("🎉 All tests passed!");
    Ok(())
}
