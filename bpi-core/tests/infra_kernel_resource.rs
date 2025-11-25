use std::collections::BTreeMap;

use bpi_core::blockchain_os_kernel::{BlockchainResourceManager, ProcessType};
use tokio;

#[tokio::test(flavor = "current_thread")]
async fn bpi_core_12_kernel_resource_manager_tracks_cpu_memory() {
    println!("=== Test: BPI-CORE-12: Kernel resource manager tracks CPU/memory ===");

    // Initialize the blockchain resource manager
    let manager = BlockchainResourceManager::new()
        .expect("failed to create BlockchainResourceManager");
    manager
        .initialize()
        .await
        .expect("failed to initialize resource manager");

    // Helper to capture a utilization snapshot with a label
    async fn capture_snapshot(
        label: &str,
        manager: &BlockchainResourceManager,
    ) -> (String, f64, f64, f64, f64) {
        let detailed = manager
            .get_detailed_utilization()
            .await
            .expect("failed to get detailed utilization");

        println!(
            "utilization_snapshot: step={} cpu={:.3} mem={:.3} storage={:.3} overall={:.3}",
            label,
            detailed.cpu_utilization,
            detailed.memory_utilization,
            detailed.storage_utilization,
            detailed.overall_utilization,
        );

        (
            label.to_string(),
            detailed.cpu_utilization,
            detailed.memory_utilization,
            detailed.storage_utilization,
            detailed.overall_utilization,
        )
    }

    // Take an initial snapshot before any allocations
    let mut timeline: BTreeMap<String, (f64, f64, f64, f64)> = BTreeMap::new();
    let (label0, cpu0, mem0, storage0, overall0) = capture_snapshot("initial", &manager).await;
    timeline.insert(label0, (cpu0, mem0, storage0, overall0));

    // Allocate a small smart-contract process
    let proc_sc = "proc_smart_contract";
    let _alloc_sc = manager
        .allocate_resources(proc_sc, &ProcessType::SmartContract)
        .await
        .expect("failed to allocate resources for smart contract process");
    let (label1, cpu1, mem1, storage1, overall1) =
        capture_snapshot("after_smart_contract", &manager).await;
    timeline.insert(label1, (cpu1, mem1, storage1, overall1));

    // Allocate a heavier VM application process
    let proc_vm = "proc_vm_application";
    let _alloc_vm = manager
        .allocate_resources(proc_vm, &ProcessType::VMApplication)
        .await
        .expect("failed to allocate resources for VM application process");
    let (label2, cpu2, mem2, storage2, overall2) =
        capture_snapshot("after_vm_application", &manager).await;
    timeline.insert(label2, (cpu2, mem2, storage2, overall2));

    // Release the smart-contract process
    manager
        .release_resources(proc_sc)
        .await
        .expect("failed to release smart contract resources");
    let (label3, cpu3, mem3, storage3, overall3) =
        capture_snapshot("after_release_smart_contract", &manager).await;
    timeline.insert(label3, (cpu3, mem3, storage3, overall3));

    // Release the VM application process
    manager
        .release_resources(proc_vm)
        .await
        .expect("failed to release VM application resources");
    let (label4, cpu4, mem4, storage4, overall4) =
        capture_snapshot("after_release_all", &manager).await;
    timeline.insert(label4, (cpu4, mem4, storage4, overall4));

    println!("resource_manager_utilization_timeline:");
    for (label, (cpu, mem, storage, overall)) in &timeline {
        println!(
            "  - step: {} | cpu={:.3} mem={:.3} storage={:.3} overall={:.3}",
            label, cpu, mem, storage, overall
        );
    }

    // Run a health check for completeness
    let healthy = manager
        .health_check()
        .await
        .expect("resource manager health check failed");
    println!("health_check_healthy: {}", healthy);

    // Invariants
    // 1. All utilization values must be between 0.0 and 1.0
    for (label, (cpu, mem, storage, overall)) in &timeline {
        assert!(
            (0.0..=1.0).contains(cpu),
            "cpu utilization out of bounds at step {}: {}",
            label,
            cpu
        );
        assert!(
            (0.0..=1.0).contains(mem),
            "memory utilization out of bounds at step {}: {}",
            label,
            mem
        );
        assert!(
            (0.0..=1.0).contains(storage),
            "storage utilization out of bounds at step {}: {}",
            label,
            storage
        );
        assert!(
            (0.0..=1.0).contains(overall),
            "overall utilization out of bounds at step {}: {}",
            label,
            overall
        );
    }

    // 2. CPU and memory utilization should increase after allocations
    assert!(
        cpu1 > cpu0 || mem1 > mem0,
        "expected utilization to increase after smart contract allocation",
    );
    assert!(
        cpu2 >= cpu1 && mem2 >= mem1,
        "expected utilization after VM allocation to be >= after smart contract",
    );

    // 3. After releasing all processes, utilization should return close to initial levels
    let cpu_delta = (cpu4 - cpu0).abs();
    let mem_delta = (mem4 - mem0).abs();
    let storage_delta = (storage4 - storage0).abs();
    let overall_delta = (overall4 - overall0).abs();

    println!(
        "utilization_return_delta: cpu={:.6} mem={:.6} storage={:.6} overall={:.6}",
        cpu_delta, mem_delta, storage_delta, overall_delta
    );

    let epsilon = 1e-6;
    assert!(
        cpu_delta <= epsilon && mem_delta <= epsilon && storage_delta <= epsilon,
        "expected utilization to return to initial levels after releasing all processes",
    );

    // 4. Health check should report healthy under this synthetic load
    assert!(healthy, "resource manager health_check should be healthy under this load");

    manager
        .shutdown()
        .await
        .expect("failed to shutdown resource manager");

    println!("status: OK");
}
