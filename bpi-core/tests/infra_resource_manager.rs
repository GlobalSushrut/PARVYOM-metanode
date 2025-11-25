use bpi_core::blockchain_os_kernel::resource_manager::BlockchainResourceManager;
use bpi_core::blockchain_os_kernel::{OrchestrationMode, ProcessType};

#[tokio::test(flavor = "current_thread")]
async fn bpi_core_12_resource_manager_utilization_preview() {
    println!("=== Test: BPI-CORE-12: Resource manager tracks CPU/memory ===");

    let manager = BlockchainResourceManager::new().expect("failed to create resource manager");

    // Initial state
    let initial_overall = manager
        .get_detailed_utilization()
        .await
        .expect("failed to get initial utilization");

    println!(
        "initial_utilization: overall={:.3}, cpu={:.3}, mem={:.3}, storage={:.3}",
        initial_overall.overall_utilization,
        initial_overall.cpu_utilization,
        initial_overall.memory_utilization,
        initial_overall.storage_utilization,
    );

    manager
        .initialize()
        .await
        .expect("failed to initialize resource manager");

    // Allocate resources for a smart contract process
    let process_id = "test-process-smart-contract";
    let allocation = manager
        .allocate_resources(process_id, &ProcessType::SmartContract)
        .await
        .expect("failed to allocate resources");

    println!(
        "allocation: id={}, cpu_cores={}, memory_mb={}, storage_gb={}",
        allocation.allocation_id,
        allocation.cpu_cores,
        allocation.memory_mb,
        allocation.storage_gb,
    );

    let after_allocation = manager
        .get_detailed_utilization()
        .await
        .expect("failed to get utilization after allocation");

    println!(
        "after_allocation: overall={:.3}, cpu={:.3}, mem={:.3}, storage={:.3}",
        after_allocation.overall_utilization,
        after_allocation.cpu_utilization,
        after_allocation.memory_utilization,
        after_allocation.storage_utilization,
    );

    // Expect utilization to be >= initial (cannot go down after allocating)
    assert!(
        after_allocation.overall_utilization >= initial_overall.overall_utilization,
        "overall utilization should not decrease after allocation",
    );

    // Health check should still be OK for this small allocation
    let healthy = manager
        .health_check()
        .await
        .expect("health_check errored");
    println!("health_check_result: {}", healthy);
    assert!(healthy, "resource manager should be healthy after small allocation");

    // Release resources and check utilization again
    manager
        .release_resources(process_id)
        .await
        .expect("failed to release resources");

    let final_utilization = manager
        .get_detailed_utilization()
        .await
        .expect("failed to get final utilization");

    println!(
        "final_utilization: overall={:.3}, cpu={:.3}, mem={:.3}, storage={:.3}",
        final_utilization.overall_utilization,
        final_utilization.cpu_utilization,
        final_utilization.memory_utilization,
        final_utilization.storage_utilization,
    );

    manager
        .shutdown()
        .await
        .expect("failed to shutdown resource manager");

    println!("status: OK");
}

#[tokio::test(flavor = "current_thread")]
async fn bpi_core_30_resource_manager_orchestration_mode_preview() {
    println!("=== Test: BPI-CORE-30: Resource orchestrator progress steps via modes ===");

    let manager = BlockchainResourceManager::new().expect("failed to create resource manager");

    let modes = [
        OrchestrationMode::Autonomous,
        OrchestrationMode::Supervised,
        OrchestrationMode::Manual,
        OrchestrationMode::Emergency,
    ];

    for mode in modes.iter() {
        manager
            .update_orchestration_mode(mode)
            .await
            .expect("failed to update orchestration mode");
        println!("- mode_updated: {:?}", mode);
    }

    // A final health check to ensure manager is still operational
    let healthy = manager
        .health_check()
        .await
        .expect("health_check errored");
    println!("health_check_result_after_mode_changes: {}", healthy);

    manager
        .shutdown()
        .await
        .expect("failed to shutdown resource manager");

    assert!(healthy, "resource manager should remain healthy after mode changes");

    println!("status: OK");
}
