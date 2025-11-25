use bpi_core::config::{BpiConfig, KernelConfig, NxLaneConfig, NxNetworkConfig};
use bpi_core::errors::BpiError;
use std::path::PathBuf;
use uuid::Uuid;

#[test]
fn bpi_core_02_bpi_config_port_range_validation() {
    println!("=== Test: BPI-CORE-02: Config validation rejects invalid ports ===");

    // Use a temp data directory so we don't write into the repo
    let temp_root = std::env::temp_dir().join(format!("bpi_config_validate_{}", Uuid::new_v4()));
    let mut config = BpiConfig::default();
    config.storage.data_dir = PathBuf::from(&temp_root);

    // Valid config should pass
    println!("valid_config.data_dir: {}", config.storage.data_dir.to_string_lossy());
    let valid_res = config.validate();
    println!("valid_config.validate_result: {:?}", valid_res);
    assert!(valid_res.is_ok(), "default config with temp data_dir should validate");

    // Invalid vm_port < 1024 should fail
    let mut bad_vm = config.clone();
    bad_vm.network.vm_port = 80; // below 1024
    let bad_vm_res = bad_vm.validate();
    println!("bad_vm_port_config.validate_result: {:?}", bad_vm_res);

    match bad_vm_res {
        Err(BpiError::Config { message, field, .. }) => {
            println!("bad_vm_port_error_message: {}", message);
            println!("bad_vm_port_error_field: {:?}", field);
            assert!(message.contains("Port"));
        }
        other => panic!("expected BpiError::Config for bad vm_port, got: {:?}", other),
    }

    // Invalid db_port < 1024 should fail (upper bound >65535 is unreachable for u16)
    let mut bad_db = config.clone();
    bad_db.network.db_port = 0; // below 1024
    let bad_db_res = bad_db.validate();
    println!("bad_db_port_config.validate_result: {:?}", bad_db_res);

    match bad_db_res {
        Err(BpiError::Config { message, field, .. }) => {
            println!("bad_db_port_error_message: {}", message);
            println!("bad_db_port_error_field: {:?}", field);
            assert!(message.contains("Port"));
        }
        other => panic!("expected BpiError::Config for bad db_port, got: {:?}", other),
    }

    println!("status: OK");
}

#[test]
fn bpi_core_04_bpi_and_nx_config_status_snapshot() {
    println!("=== Test: BPI-CORE-04: BPI + NX config status snapshot ===");

    // Build kernel + BPI config in-memory
    let mut bpi = BpiConfig::default();
    let temp_root = std::env::temp_dir().join(format!("bpi_config_status_{}", Uuid::new_v4()));
    bpi.storage.data_dir = PathBuf::from(&temp_root);

    let kernel = KernelConfig {
        profile: "pilot".to_string(),
        node_id: "bpi-node-pilot-status".to_string(),
        bpi: bpi.clone(),
    };

    let nx = NxNetworkConfig {
        profile: "pilot".to_string(),
        node_id: Some(kernel.node_id.clone()),
        mesh_internal_required: false,
        lanes: NxLaneConfig {
            vm: format!("http://{}:{}", bpi.network.domain, bpi.network.vm_port),
            http_cage: "http://127.0.0.1:8081".to_string(),
            xtmp_bpci: "tcp://127.0.0.1:7778".to_string(),
            shadow_registry: "http://127.0.0.1:9090".to_string(),
        },
    };

    println!("profile: {}", kernel.profile);
    println!("node_id: {}", kernel.node_id);

    println!(
        "network: domain={}, bind_address={}, vm_port={}, bpci_port={}, db_port={}, orchestrator_port={}",
        bpi.network.domain,
        bpi.network.bind_address,
        bpi.network.vm_port,
        bpi.network.bpci_port,
        bpi.network.db_port,
        bpi.network.orchestrator_port,
    );

    println!("nx.mesh_internal_required: {}", nx.mesh_internal_required);
    println!("nx.lanes.vm: {}", nx.lanes.vm);
    println!("nx.lanes.http_cage: {}", nx.lanes.http_cage);
    println!("nx.lanes.xtmp_bpci: {}", nx.lanes.xtmp_bpci);
    println!("nx.lanes.shadow_registry: {}", nx.lanes.shadow_registry);

    let validate_net = kernel.validate_nx_network();
    let validate_bpi = bpi.validate();
    let validate_nx = nx.validate_consistency(&kernel);

    println!("kernel.validate_nx_network: {:?}", validate_net);
    println!("bpi.validate: {:?}", validate_bpi);
    println!("nx.validate_consistency: {:?}", validate_nx);

    assert!(validate_net.is_ok(), "kernel NX network invariants must hold");
    assert!(validate_bpi.is_ok(), "BPI config must validate");
    assert!(validate_nx.is_ok(), "NX config must be consistent with kernel");

    println!("status: OK");
}

#[test]
fn bpi_core_03_nx_network_config_consistency_preview() {
    println!("=== Test: BPI-CORE-03: NX network + kernel consistency ===");

    // Construct a kernel config in-memory (no file dependency)
    let kernel = KernelConfig {
        profile: "pilot".to_string(),
        node_id: "bpi-node-pilot".to_string(),
        bpi: BpiConfig::default(),
    };

    // Consistent NX config
    let nx_ok = NxNetworkConfig {
        profile: "pilot".to_string(),
        node_id: Some("bpi-node-pilot".to_string()),
        mesh_internal_required: false,
        lanes: NxLaneConfig {
            vm: "http://127.0.0.1:8080".to_string(),
            http_cage: "http://127.0.0.1:8081".to_string(),
            xtmp_bpci: "tcp://127.0.0.1:7778".to_string(),
            shadow_registry: "http://127.0.0.1:9090".to_string(),
        },
    };

    let ok_res = nx_ok.validate_consistency(&kernel);
    println!("nx_ok.validate_consistency: {:?}", ok_res);
    assert!(ok_res.is_ok(), "consistent NX config should validate");

    // Mismatched profile should fail
    let mut nx_bad_profile = nx_ok.clone();
    nx_bad_profile.profile = "devnet".to_string();
    let bad_profile_res = nx_bad_profile.validate_consistency(&kernel);
    println!("nx_bad_profile.validate_consistency: {:?}", bad_profile_res);

    // Empty lane should fail
    let mut nx_bad_lane = nx_ok.clone();
    nx_bad_lane.lanes.vm = "".to_string();
    let bad_lane_res = nx_bad_lane.validate_consistency(&kernel);
    println!("nx_bad_lane.validate_consistency: {:?}", bad_lane_res);

    assert!(bad_profile_res.is_err(), "profile mismatch should be an error");
    assert!(bad_lane_res.is_err(), "empty lane should be an error");

    println!("status: OK");
}
