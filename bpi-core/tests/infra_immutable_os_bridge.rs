use bpi_core::blockchain_os_kernel::immutable_os_bridge::{
    BpiImmutableOSIntegration,
    ImmutableOSServiceType,
};

#[tokio::test(flavor = "current_thread")]
async fn bpi_core_20_immutable_os_bridge_endpoint_mapping() {
    println!("=== Test: BPI-CORE-20: Immutable OS bridge endpoint mapping ===");

    // 1. Create and initialize the integration bridge
    let bridge = BpiImmutableOSIntegration::new()
        .expect("failed to create BpiImmutableOSIntegration");

    bridge
        .initialize()
        .await
        .expect("failed to initialize immutable OS bridge");

    // 2. Fetch current service mappings and integration state
    let mappings = bridge
        .get_service_mappings()
        .await
        .expect("failed to get service mappings");
    let fs_state = bridge
        .get_filesystem_state()
        .expect("failed to get filesystem state");
    let net_state = bridge
        .get_network_state()
        .expect("failed to get network state");
    let status = bridge
        .get_integration_status()
        .await
        .expect("failed to get integration status");
    let stats = bridge
        .get_integration_stats()
        .expect("failed to get integration stats");

    println!("service_mappings:");
    for (name, mapping) in &mappings {
        println!(
            "  - bpi_service={} os_service_type={:?} port={} integration_status={:?} health_status={:?}",
            name,
            mapping.service_type,
            mapping.service_port,
            mapping.integration_status,
            mapping.health_status,
        );
    }

    println!("filesystem_state:");
    println!("  namespace_mounted: {}", fs_state.namespace_mounted);
    println!("  core_paths_available: {}", fs_state.core_paths_available);
    println!("  data_layer_accessible: {}", fs_state.data_layer_accessible);
    println!("  config_management_active: {}", fs_state.config_management_active);
    println!("  runtime_state_synchronized: {}", fs_state.runtime_state_synchronized);
    println!("  immutable_overlays_count: {}", fs_state.immutable_overlays_count);

    println!("network_state:");
    println!("  vpod_network_active: {}", net_state.vpod_network_active);
    println!(
        "  service_mesh_configured: {}",
        net_state.service_mesh_configured
    );
    println!("  active_connections: {}", net_state.active_connections);

    println!("integration_status: {:?}", status);
    println!("integration_stats:");
    println!(
        "  total_services_integrated: {}",
        stats.total_services_integrated
    );
    println!("  healthy_services: {}", stats.healthy_services);
    println!("  degraded_services: {}", stats.degraded_services);
    println!("  failed_services: {}", stats.failed_services);

    // 3. Core invariants: core services must exist with expected ports
    let mut vm_ok = false;
    let mut http_ok = false;
    let mut shadow_ok = false;
    let mut zklock_ok = false;

    for (name, mapping) in &mappings {
        match (name.as_str(), &mapping.service_type, mapping.service_port) {
            ("vm_server", ImmutableOSServiceType::VMServer, 7777) => vm_ok = true,
            ("http_cage", ImmutableOSServiceType::HttpCage, 8888) => http_ok = true,
            ("shadow_registry", ImmutableOSServiceType::ShadowRegistry, 8080) => shadow_ok = true,
            ("zklock_mobile", ImmutableOSServiceType::ZKLockMobile, 8081) => zklock_ok = true,
            _ => {}
        }
    }

    assert!(
        !mappings.is_empty(),
        "expected immutable OS bridge to register at least one service mapping",
    );

    assert!(vm_ok, "expected vm_server -> port 7777 mapping to exist");
    assert!(http_ok, "expected http_cage -> port 8888 mapping to exist");
    assert!(
        shadow_ok,
        "expected shadow_registry -> port 8080 mapping to exist",
    );
    assert!(
        zklock_ok,
        "expected zklock_mobile -> port 8081 mapping to exist",
    );

    println!("summary:");
    println!("  total_services_integrated: {}", stats.total_services_integrated);
    println!("  filesystem_namespace_mounted: {}", fs_state.namespace_mounted);
    println!("  vpod_network_active: {}", net_state.vpod_network_active);
    println!("  integration_status: {:?}", status);
    println!("status: OK");
}
