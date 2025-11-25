use bpi_core::config::BpiConfig;
use bpi_core::diagnostics::{DiagnosticStatus, DiagnosticSystem};
use bpi_core::health::HealthChecker;
use std::path::PathBuf;
use uuid::Uuid;

#[tokio::test(flavor = "current_thread")]
async fn bpi_core_05_diagnostics_self_check_report() {
    println!("=== Test: BPI-CORE-05: Diagnostics self-check report ===");

    // Use a temp data directory so diagnostics/config validation don't touch the repo
    let temp_root = std::env::temp_dir().join(format!("bpi_diagnostics_self_check_{}", Uuid::new_v4()));
    let mut config = BpiConfig::default();
    config.storage.data_dir = PathBuf::from(&temp_root);

    let diag_system = DiagnosticSystem::new(config.clone());

    let report = diag_system
        .run_diagnostics()
        .await
        .expect("failed to run diagnostics");

    println!("timestamp: {}", report.timestamp);
    println!("overall_status: {:?}", report.overall_status);

    // System snapshot
    println!(
        "system_info: os={}, arch={}, kernel={}, cpu_cores={}, mem_total_gb={:.2}, mem_available_gb={:.2}, disk_gb={:.2}, disk_available_gb={:.2}",
        report.system_info.os,
        report.system_info.arch,
        report.system_info.kernel_version,
        report.system_info.cpu_cores,
        report.system_info.memory_total_gb,
        report.system_info.memory_available_gb,
        report.system_info.disk_space_gb,
        report.system_info.disk_available_gb,
    );

    // Network summary
    let nd = &report.network_diagnostics;
    println!(
        "network: localhost_connectivity={}, dns_resolution={}, internet_connectivity={}, firewall_status={}",
        nd.localhost_connectivity,
        nd.dns_resolution,
        nd.internet_connectivity,
        nd.firewall_status,
    );
    println!(
        "network.port_availability: {:?}",
        nd.port_availability
    );

    // Services snapshot
    let sd = &report.service_diagnostics;
    println!(
        "services.vm_server: running={}, port_listening={}",
        sd.vm_server_status.running, sd.vm_server_status.port_listening
    );
    println!(
        "services.bpci_bridge: running={}, port_listening={}",
        sd.bpci_bridge_status.running, sd.bpci_bridge_status.port_listening
    );
    println!(
        "services.database: running={}, port_listening={}",
        sd.database_status.running, sd.database_status.port_listening
    );
    println!(
        "services.orchestrator: running={}, port_listening={}",
        sd.orchestrator_status.running, sd.orchestrator_status.port_listening
    );
    println!(
        "services.running_processes_count: {}",
        sd.running_processes.len()
    );

    // Configuration snapshot
    let cd = &report.configuration_diagnostics;
    println!(
        "config: file_exists={}, config_valid={}, directories_exist={}, permissions_ok={}",
        cd.config_file_exists,
        cd.config_valid,
        cd.directories_exist,
        cd.permissions_correct,
    );
    println!("config.env_vars_set: {:?}", cd.env_vars_set);

    // Performance snapshot
    let pd = &report.performance_diagnostics;
    println!(
        "performance: cpu_usage={:.2}%, mem_usage={:.2}%, disk_usage={:.2}%, load_average={:?}",
        pd.cpu_usage_percent,
        pd.memory_usage_percent,
        pd.disk_usage_percent,
        pd.load_average,
    );
    println!("performance.response_times: {:?}", pd.response_times);

    // Recommendations summary
    println!("recommendations_count: {}", report.recommendations.len());
    for rec in &report.recommendations {
        println!(
            "- recommendation: category={}, severity={}, title={}",
            rec.category, rec.severity, rec.title
        );
    }

    // Basic invariants: system info must be non-trivial
    assert!(report.system_info.cpu_cores >= 1);
    assert!(report.system_info.memory_total_gb > 0.0);
    assert!(report.system_info.disk_space_gb > 0.0);

    // Also run the HealthChecker directly for a pilot-ready snapshot
    let health_checker = HealthChecker::new();
    match health_checker.check_health().await {
        Ok(status) => {
            println!("health.status: {}", status.status);
            println!("health.pilot_ready: {}", status.pilot_ready);
            println!("health.version: {}", status.version);
            println!("health.uptime_seconds: {}", status.uptime_seconds);
            println!("health.services: {{}}:");
            for (name, svc) in status.services.iter() {
                println!(
                    "  - {}: status={}, response_time_ms={}, error={:?}",
                    name, svc.status, svc.response_time_ms, svc.error_message
                );
            }
        }
        Err(e) => {
            println!("health_check_error: {}", e);
        }
    }

    // Overall diagnostics status must be one of the known variants (always true by type),
    // but we print it for human inspection.
    match report.overall_status {
        DiagnosticStatus::Healthy
        | DiagnosticStatus::Warning
        | DiagnosticStatus::Critical
        | DiagnosticStatus::Error => {}
    }

    println!("status: OK");
}
