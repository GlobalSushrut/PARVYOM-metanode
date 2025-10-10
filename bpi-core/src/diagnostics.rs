//! Diagnostic System for BPI Infrastructure
//! 
//! Provides comprehensive system diagnostics and troubleshooting capabilities

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::errors::{BpiError, BpiResult};
use crate::config::BpiConfig;
use crate::health::HealthChecker;

/// Comprehensive diagnostic report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReport {
    pub timestamp: u64,
    pub system_info: SystemInfo,
    pub network_diagnostics: NetworkDiagnostics,
    pub service_diagnostics: ServiceDiagnostics,
    pub configuration_diagnostics: ConfigurationDiagnostics,
    pub performance_diagnostics: PerformanceDiagnostics,
    pub recommendations: Vec<Recommendation>,
    pub overall_status: DiagnosticStatus,
}

/// System information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub kernel_version: String,
    pub memory_total_gb: f64,
    pub memory_available_gb: f64,
    pub cpu_cores: usize,
    pub disk_space_gb: f64,
    pub disk_available_gb: f64,
}

/// Network diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDiagnostics {
    pub localhost_connectivity: bool,
    pub port_availability: HashMap<u16, bool>,
    pub dns_resolution: bool,
    pub internet_connectivity: bool,
    pub firewall_status: String,
}

/// Service diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiagnostics {
    pub vm_server_status: ServiceStatus,
    pub bpci_bridge_status: ServiceStatus,
    pub database_status: ServiceStatus,
    pub orchestrator_status: ServiceStatus,
    pub running_processes: Vec<ProcessInfo>,
}

/// Configuration diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationDiagnostics {
    pub config_file_exists: bool,
    pub config_valid: bool,
    pub env_vars_set: HashMap<String, bool>,
    pub permissions_correct: bool,
    pub directories_exist: bool,
}

/// Performance diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDiagnostics {
    pub cpu_usage_percent: f64,
    pub memory_usage_percent: f64,
    pub disk_usage_percent: f64,
    pub load_average: Vec<f64>,
    pub response_times: HashMap<String, u64>,
}

/// Service status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub running: bool,
    pub pid: Option<u32>,
    pub port_listening: bool,
    pub response_time_ms: Option<u64>,
    pub error_message: Option<String>,
}

/// Process information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub name: String,
    pub pid: u32,
    pub cpu_percent: f64,
    pub memory_mb: f64,
    pub status: String,
}

/// Diagnostic recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub category: String,
    pub severity: String,
    pub title: String,
    pub description: String,
    pub action: String,
    pub command: Option<String>,
}

/// Overall diagnostic status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiagnosticStatus {
    Healthy,
    Warning,
    Critical,
    Error,
}

/// Main diagnostic system
pub struct DiagnosticSystem {
    config: BpiConfig,
    health_checker: HealthChecker,
}

impl DiagnosticSystem {
    pub fn new(config: BpiConfig) -> Self {
        Self {
            config,
            health_checker: HealthChecker::new(),
        }
    }

    /// Run comprehensive system diagnostics
    pub async fn run_diagnostics(&self) -> BpiResult<DiagnosticReport> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let system_info = self.collect_system_info().await?;
        let network_diagnostics = self.run_network_diagnostics().await?;
        let service_diagnostics = self.run_service_diagnostics().await?;
        let config_diagnostics = self.run_configuration_diagnostics().await?;
        let performance_diagnostics = self.run_performance_diagnostics().await?;

        let mut recommendations = Vec::new();
        let overall_status = self.analyze_and_recommend(
            &system_info,
            &network_diagnostics,
            &service_diagnostics,
            &config_diagnostics,
            &performance_diagnostics,
            &mut recommendations,
        );

        Ok(DiagnosticReport {
            timestamp,
            system_info,
            network_diagnostics,
            service_diagnostics,
            configuration_diagnostics: config_diagnostics,
            performance_diagnostics,
            recommendations,
            overall_status,
        })
    }

    /// Collect system information
    async fn collect_system_info(&self) -> BpiResult<SystemInfo> {
        let os = std::env::consts::OS.to_string();
        let arch = std::env::consts::ARCH.to_string();
        
        let kernel_version = self.run_command("uname", &["-r"])
            .unwrap_or_else(|_| "unknown".to_string());

        // Get memory information
        let (memory_total_gb, memory_available_gb) = self.get_memory_info();
        
        // Get CPU information
        let cpu_cores = num_cpus::get();
        
        // Get disk information
        let (disk_space_gb, disk_available_gb) = self.get_disk_info();

        Ok(SystemInfo {
            os,
            arch,
            kernel_version,
            memory_total_gb,
            memory_available_gb,
            cpu_cores,
            disk_space_gb,
            disk_available_gb,
        })
    }

    /// Run network diagnostics
    async fn run_network_diagnostics(&self) -> BpiResult<NetworkDiagnostics> {
        let localhost_connectivity = self.test_localhost_connectivity().await;
        let port_availability = self.check_port_availability().await;
        let dns_resolution = self.test_dns_resolution().await;
        let internet_connectivity = self.test_internet_connectivity().await;
        let firewall_status = self.check_firewall_status().await;

        Ok(NetworkDiagnostics {
            localhost_connectivity,
            port_availability,
            dns_resolution,
            internet_connectivity,
            firewall_status,
        })
    }

    /// Run service diagnostics
    async fn run_service_diagnostics(&self) -> BpiResult<ServiceDiagnostics> {
        let vm_server_status = self.check_service_status("vm_server", self.config.network.vm_port).await;
        let bpci_bridge_status = self.check_service_status("bpci_bridge", self.config.network.bpci_port).await;
        let database_status = self.check_service_status("database", self.config.network.db_port).await;
        let orchestrator_status = self.check_service_status("orchestrator", self.config.network.orchestrator_port).await;
        let running_processes = self.get_running_processes().await;

        Ok(ServiceDiagnostics {
            vm_server_status,
            bpci_bridge_status,
            database_status,
            orchestrator_status,
            running_processes,
        })
    }

    /// Run configuration diagnostics
    async fn run_configuration_diagnostics(&self) -> BpiResult<ConfigurationDiagnostics> {
        let config_file_exists = std::path::Path::new("./config/bpi-pilot-config.toml").exists();
        let config_valid = self.config.validate().is_ok();
        let env_vars_set = self.check_environment_variables();
        let permissions_correct = self.check_permissions().await;
        let directories_exist = self.check_directories().await;

        Ok(ConfigurationDiagnostics {
            config_file_exists,
            config_valid,
            env_vars_set,
            permissions_correct,
            directories_exist,
        })
    }

    /// Run performance diagnostics
    async fn run_performance_diagnostics(&self) -> BpiResult<PerformanceDiagnostics> {
        let cpu_usage_percent = self.get_cpu_usage().await;
        let memory_usage_percent = self.get_memory_usage().await;
        let disk_usage_percent = self.get_disk_usage().await;
        let load_average = self.get_load_average().await;
        let response_times = self.measure_response_times().await;

        Ok(PerformanceDiagnostics {
            cpu_usage_percent,
            memory_usage_percent,
            disk_usage_percent,
            load_average,
            response_times,
        })
    }

    /// Analyze results and generate recommendations
    fn analyze_and_recommend(
        &self,
        system_info: &SystemInfo,
        network_diagnostics: &NetworkDiagnostics,
        service_diagnostics: &ServiceDiagnostics,
        config_diagnostics: &ConfigurationDiagnostics,
        performance_diagnostics: &PerformanceDiagnostics,
        recommendations: &mut Vec<Recommendation>,
    ) -> DiagnosticStatus {
        let mut status = DiagnosticStatus::Healthy;

        // Check system resources
        if system_info.memory_available_gb < 1.0 {
            recommendations.push(Recommendation {
                category: "system".to_string(),
                severity: "high".to_string(),
                title: "Low Memory".to_string(),
                description: "Available memory is below 1GB".to_string(),
                action: "Close unnecessary applications or add more RAM".to_string(),
                command: Some("free -h".to_string()),
            });
            status = DiagnosticStatus::Warning;
        }

        // Check network connectivity
        if !network_diagnostics.localhost_connectivity {
            recommendations.push(Recommendation {
                category: "network".to_string(),
                severity: "critical".to_string(),
                title: "Localhost Connectivity Failed".to_string(),
                description: "Cannot connect to localhost".to_string(),
                action: "Check network configuration and firewall settings".to_string(),
                command: Some("ping -c 1 127.0.0.1".to_string()),
            });
            status = DiagnosticStatus::Critical;
        }

        // Check service status
        if !service_diagnostics.vm_server_status.running {
            recommendations.push(Recommendation {
                category: "service".to_string(),
                severity: "high".to_string(),
                title: "VM Server Not Running".to_string(),
                description: "VM Server service is not running".to_string(),
                action: "Start VM Server service".to_string(),
                command: Some("bpi-core vm-server start".to_string()),
            });
            status = DiagnosticStatus::Warning;
        }

        // Check configuration
        if !config_diagnostics.config_valid {
            recommendations.push(Recommendation {
                category: "configuration".to_string(),
                severity: "high".to_string(),
                title: "Invalid Configuration".to_string(),
                description: "Configuration validation failed".to_string(),
                action: "Fix configuration errors and validate".to_string(),
                command: Some("bpi-core config validate".to_string()),
            });
            status = DiagnosticStatus::Warning;
        }

        // Check performance
        if performance_diagnostics.cpu_usage_percent > 90.0 {
            recommendations.push(Recommendation {
                category: "performance".to_string(),
                severity: "medium".to_string(),
                title: "High CPU Usage".to_string(),
                description: format!("CPU usage is {}%", performance_diagnostics.cpu_usage_percent),
                action: "Check for resource-intensive processes".to_string(),
                command: Some("top -n 1".to_string()),
            });
            if status == DiagnosticStatus::Healthy {
                status = DiagnosticStatus::Warning;
            }
        }

        status
    }

    // Helper methods for diagnostics

    async fn test_localhost_connectivity(&self) -> bool {
        tokio::net::TcpStream::connect("127.0.0.1:22").await.is_ok() ||
        tokio::net::TcpStream::connect("127.0.0.1:80").await.is_ok()
    }

    async fn check_port_availability(&self) -> HashMap<u16, bool> {
        let mut ports = HashMap::new();
        let required_ports = [
            self.config.network.vm_port,
            self.config.network.bpci_port,
            self.config.network.db_port,
            self.config.network.orchestrator_port,
        ];

        for port in required_ports {
            let available = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await.is_ok();
            ports.insert(port, available);
        }

        ports
    }

    async fn test_dns_resolution(&self) -> bool {
        tokio::net::lookup_host("localhost:80").await.is_ok()
    }

    async fn test_internet_connectivity(&self) -> bool {
        reqwest::get("https://httpbin.org/get").await.is_ok()
    }

    async fn check_firewall_status(&self) -> String {
        self.run_command("ufw", &["status"])
            .unwrap_or_else(|_| "unknown".to_string())
    }

    async fn check_service_status(&self, service_name: &str, port: u16) -> ServiceStatus {
        let running = self.is_process_running(service_name).await;
        let pid = if running { self.get_process_pid(service_name).await } else { None };
        let port_listening = self.is_port_listening(port).await;
        let response_time_ms = if port_listening {
            self.measure_service_response_time(port).await
        } else {
            None
        };

        ServiceStatus {
            running,
            pid,
            port_listening,
            response_time_ms,
            error_message: None,
        }
    }

    async fn get_running_processes(&self) -> Vec<ProcessInfo> {
        // Simplified process listing - in real implementation, use sysinfo crate
        vec![]
    }

    fn check_environment_variables(&self) -> HashMap<String, bool> {
        let mut env_vars = HashMap::new();
        let required_vars = BpiConfig::get_env_vars();

        for (var_name, _description) in required_vars {
            env_vars.insert(var_name.to_string(), std::env::var(var_name).is_ok());
        }

        env_vars
    }

    async fn check_permissions(&self) -> bool {
        // Check if we can write to data directory
        std::fs::create_dir_all(&self.config.storage.data_dir).is_ok()
    }

    async fn check_directories(&self) -> bool {
        self.config.storage.data_dir.exists() &&
        self.config.logging.output.parent().map(|p| p.exists()).unwrap_or(true)
    }

    fn get_memory_info(&self) -> (f64, f64) {
        // Simplified - in real implementation, use sysinfo crate
        (8.0, 4.0) // 8GB total, 4GB available
    }

    fn get_disk_info(&self) -> (f64, f64) {
        // Simplified - in real implementation, use sysinfo crate
        (100.0, 50.0) // 100GB total, 50GB available
    }

    async fn get_cpu_usage(&self) -> f64 {
        // Simplified - in real implementation, use sysinfo crate
        25.0
    }

    async fn get_memory_usage(&self) -> f64 {
        // Simplified - in real implementation, use sysinfo crate
        50.0
    }

    async fn get_disk_usage(&self) -> f64 {
        // Simplified - in real implementation, use sysinfo crate
        50.0
    }

    async fn get_load_average(&self) -> Vec<f64> {
        // Simplified - in real implementation, read from /proc/loadavg
        vec![1.0, 1.5, 2.0]
    }

    async fn measure_response_times(&self) -> HashMap<String, u64> {
        let mut times = HashMap::new();
        
        // Measure health check response time
        let start = std::time::Instant::now();
        let _ = self.health_checker.check_health().await;
        times.insert("health_check".to_string(), start.elapsed().as_millis() as u64);

        times
    }

    async fn is_process_running(&self, process_name: &str) -> bool {
        self.run_command("pgrep", &[process_name]).is_ok()
    }

    async fn get_process_pid(&self, process_name: &str) -> Option<u32> {
        self.run_command("pgrep", &[process_name])
            .ok()
            .and_then(|output| output.trim().parse().ok())
    }

    async fn is_port_listening(&self, port: u16) -> bool {
        tokio::net::TcpStream::connect(format!("127.0.0.1:{}", port)).await.is_ok()
    }

    async fn measure_service_response_time(&self, port: u16) -> Option<u64> {
        let start = std::time::Instant::now();
        if tokio::net::TcpStream::connect(format!("127.0.0.1:{}", port)).await.is_ok() {
            Some(start.elapsed().as_millis() as u64)
        } else {
            None
        }
    }

    fn run_command(&self, command: &str, args: &[&str]) -> Result<String, std::io::Error> {
        let output = Command::new(command).args(args).output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Quick diagnostic check for pilot readiness
    pub async fn quick_pilot_check(&self) -> BpiResult<bool> {
        let health_status = self.health_checker.check_health().await?;
        let network_ok = self.test_localhost_connectivity().await;
        let config_ok = self.config.validate().is_ok();

        Ok(health_status.pilot_ready && network_ok && config_ok)
    }

    /// Auto-fix common issues
    pub async fn auto_fix_issues(&self) -> BpiResult<Vec<String>> {
        let mut fixes_applied = Vec::new();

        // Create missing directories
        if std::fs::create_dir_all(&self.config.storage.data_dir).is_ok() {
            fixes_applied.push("Created data directory".to_string());
        }

        if let Some(parent) = self.config.logging.output.parent() {
            if std::fs::create_dir_all(parent).is_ok() {
                fixes_applied.push("Created log directory".to_string());
            }
        }

        // Generate default config if missing
        if !std::path::Path::new("./config/bpi-pilot-config.toml").exists() {
            std::fs::create_dir_all("./config").ok();
            if let Ok(config_content) = BpiConfig::generate_sample_config("pilot") {
                if std::fs::write("./config/bpi-pilot-config.toml", config_content).is_ok() {
                    fixes_applied.push("Generated default configuration".to_string());
                }
            }
        }

        Ok(fixes_applied)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_diagnostic_system() {
        let config = BpiConfig::default();
        let diagnostics = DiagnosticSystem::new(config);
        
        let report = diagnostics.run_diagnostics().await.unwrap();
        assert!(!report.system_info.os.is_empty());
    }

    #[tokio::test]
    async fn test_quick_pilot_check() {
        let config = BpiConfig::default();
        let diagnostics = DiagnosticSystem::new(config);
        
        let _result = diagnostics.quick_pilot_check().await;
        // Note: This may fail in test environment, which is expected
    }
}
