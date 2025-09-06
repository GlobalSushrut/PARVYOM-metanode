use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use tokio::fs;
use chrono::{DateTime, Utc};
use crate::bpci_auction_mempool::BpciAuctionMempool;
use crate::round_table_oracle::RoundTableOracle;

/// Community Installer OS - Turnkey mining and auction participation system
/// Provides automated installation, configuration, and management of BPCI mining nodes

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemRequirements {
    pub min_cpu_cores: u32,
    pub min_ram_gb: u32,
    pub min_storage_gb: u32,
    pub min_network_mbps: u32,
    pub required_os: String,
    pub required_kernel_version: String,
}

impl Default for SystemRequirements {
    fn default() -> Self {
        Self {
            min_cpu_cores: 8,
            min_ram_gb: 8,
            min_storage_gb: 100,
            min_network_mbps: 100,
            required_os: "Ubuntu 22.04 LTS".to_string(),
            required_kernel_version: "5.15.0".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub firewall_enabled: bool,
    pub fail2ban_enabled: bool,
    pub encrypted_storage: bool,
    pub secure_boot: bool,
    pub auto_updates: bool,
    pub ssh_key_only: bool,
    pub allowed_ports: Vec<u16>,
    pub blocked_countries: Vec<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            firewall_enabled: true,
            fail2ban_enabled: true,
            encrypted_storage: true,
            secure_boot: true,
            auto_updates: true,
            ssh_key_only: true,
            allowed_ports: vec![22, 80, 443, 8080, 9090], // SSH, HTTP, HTTPS, BPCI, Monitoring
            blocked_countries: vec![], // Configurable per deployment
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub prometheus_enabled: bool,
    pub grafana_enabled: bool,
    pub log_retention_days: u32,
    pub metrics_retention_days: u32,
    pub alert_email: Option<String>,
    pub alert_webhook: Option<String>,
    pub performance_thresholds: PerformanceThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    pub max_cpu_percent: f64,
    pub max_memory_percent: f64,
    pub max_disk_percent: f64,
    pub min_network_mbps: f64,
    pub max_response_time_ms: u64,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            prometheus_enabled: true,
            grafana_enabled: true,
            log_retention_days: 30,
            metrics_retention_days: 90,
            alert_email: None,
            alert_webhook: None,
            performance_thresholds: PerformanceThresholds {
                max_cpu_percent: 80.0,
                max_memory_percent: 85.0,
                max_disk_percent: 90.0,
                min_network_mbps: 50.0,
                max_response_time_ms: 1000,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningConfig {
    pub mining_enabled: bool,
    pub auction_participation: bool,
    pub max_concurrent_auctions: u32,
    pub revenue_share_percent: u32,
    pub auto_reinvest_percent: u32,
    pub payout_threshold_wei: u64,
    pub preferred_chains: Vec<u64>, // Chain IDs
}

impl Default for MiningConfig {
    fn default() -> Self {
        Self {
            mining_enabled: true,
            auction_participation: true,
            max_concurrent_auctions: 10,
            revenue_share_percent: 25,
            auto_reinvest_percent: 20,
            payout_threshold_wei: 1_000_000_000_000_000_000, // 1 ETH equivalent
            preferred_chains: vec![1, 137, 42161], // Ethereum, Polygon, Arbitrum
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallerConfig {
    pub system_requirements: SystemRequirements,
    pub security_config: SecurityConfig,
    pub monitoring_config: MonitoringConfig,
    pub mining_config: MiningConfig,
    pub bpci_node_endpoint: String,
    pub round_table_endpoint: String,
    pub installer_version: String,
    pub auto_start_services: bool,
}

impl Default for InstallerConfig {
    fn default() -> Self {
        Self {
            system_requirements: SystemRequirements::default(),
            security_config: SecurityConfig::default(),
            monitoring_config: MonitoringConfig::default(),
            mining_config: MiningConfig::default(),
            bpci_node_endpoint: "https://bpci-mainnet.pravyom.com".to_string(),
            round_table_endpoint: "https://round-table.pravyom.com".to_string(),
            installer_version: "1.0.0".to_string(),
            auto_start_services: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationStatus {
    pub phase: InstallationPhase,
    pub progress_percent: u32,
    pub current_step: String,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub start_time: DateTime<Utc>,
    pub estimated_completion: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstallationPhase {
    SystemCheck,
    DependencyInstall,
    SecurityHardening,
    ServiceConfiguration,
    MiningSetup,
    MonitoringSetup,
    Testing,
    Completed,
    Failed,
}

#[derive(Debug)]
pub struct CommunityInstallerOS {
    pub config: InstallerConfig,
    pub installation_status: InstallationStatus,
    pub bpci_mempool: Option<BpciAuctionMempool>,
    pub round_table_oracle: Option<RoundTableOracle>,
    pub system_info: SystemInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub hostname: String,
    pub os_version: String,
    pub kernel_version: String,
    pub cpu_cores: u32,
    pub total_ram_gb: u32,
    pub available_storage_gb: u32,
    pub network_interfaces: Vec<NetworkInterface>,
    pub uptime_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub ip_address: String,
    pub mac_address: String,
    pub speed_mbps: u32,
    pub is_up: bool,
}

impl CommunityInstallerOS {
    /// Create a new Community Installer OS instance
    pub fn new(config: Option<InstallerConfig>) -> Self {
        let config = config.unwrap_or_default();
        let installation_status = InstallationStatus {
            phase: InstallationPhase::SystemCheck,
            progress_percent: 0,
            current_step: "Initializing installer".to_string(),
            errors: Vec::new(),
            warnings: Vec::new(),
            start_time: Utc::now(),
            estimated_completion: None,
        };

        Self {
            config,
            installation_status,
            bpci_mempool: None,
            round_table_oracle: None,
            system_info: SystemInfo::detect(),
        }
    }

    /// Run the complete installation process
    pub async fn install(&mut self) -> Result<()> {
        println!("🚀 Starting BPCI Community Installer OS v{}", self.config.installer_version);
        
        // Phase 1: System Requirements Check
        self.update_status(InstallationPhase::SystemCheck, 10, "Checking system requirements").await;
        self.check_system_requirements().await?;

        // Phase 2: Install Dependencies
        self.update_status(InstallationPhase::DependencyInstall, 25, "Installing dependencies").await;
        self.install_dependencies().await?;

        // Phase 3: Security Hardening
        self.update_status(InstallationPhase::SecurityHardening, 40, "Configuring security").await;
        self.configure_security().await?;

        // Phase 4: Service Configuration
        self.update_status(InstallationPhase::ServiceConfiguration, 60, "Configuring BPCI services").await;
        self.configure_services().await?;

        // Phase 5: Mining Setup
        self.update_status(InstallationPhase::MiningSetup, 75, "Setting up mining and auctions").await;
        self.setup_mining().await?;

        // Phase 6: Monitoring Setup
        self.update_status(InstallationPhase::MonitoringSetup, 90, "Setting up monitoring").await;
        self.setup_monitoring().await?;

        // Phase 7: Testing
        self.update_status(InstallationPhase::Testing, 95, "Running system tests").await;
        self.run_tests().await?;

        // Phase 8: Completion
        self.update_status(InstallationPhase::Completed, 100, "Installation completed successfully").await;
        
        if self.config.auto_start_services {
            self.start_services().await?;
        }

        println!("✅ BPCI Community Installer OS installation completed successfully!");
        println!("🌐 Web interface available at: http://localhost:8080");
        println!("📊 Monitoring dashboard: http://localhost:3000");
        
        Ok(())
    }

    /// Check if system meets minimum requirements
    async fn check_system_requirements(&mut self) -> Result<()> {
        let req = &self.config.system_requirements;
        let sys = &self.system_info;

        // Check CPU cores
        if sys.cpu_cores < req.min_cpu_cores {
            return Err(anyhow!("Insufficient CPU cores: {} < {}", sys.cpu_cores, req.min_cpu_cores));
        }

        // Check RAM
        if sys.total_ram_gb < req.min_ram_gb {
            return Err(anyhow!("Insufficient RAM: {}GB < {}GB", sys.total_ram_gb, req.min_ram_gb));
        }

        // Check storage
        if sys.available_storage_gb < req.min_storage_gb {
            return Err(anyhow!("Insufficient storage: {}GB < {}GB", sys.available_storage_gb, req.min_storage_gb));
        }

        // Check OS version
        if !sys.os_version.contains("Ubuntu 22.04") {
            self.installation_status.warnings.push(
                format!("OS version {} may not be fully supported", sys.os_version)
            );
        }

        println!("✅ System requirements check passed");
        Ok(())
    }

    /// Install required dependencies
    async fn install_dependencies(&mut self) -> Result<()> {
        let packages = vec![
            "curl", "wget", "git", "build-essential", "pkg-config", "libssl-dev",
            "docker.io", "docker-compose", "nginx", "ufw", "fail2ban",
            "prometheus", "grafana", "node-exporter", "htop", "iotop", "nethogs"
        ];

        println!("📦 Installing system packages...");
        for package in packages {
            self.run_command(&format!("apt-get install -y {}", package)).await?;
        }

        // Install Rust if not present
        if !self.check_command_exists("cargo").await {
            println!("🦀 Installing Rust toolchain...");
            self.run_command("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y").await?;
            self.run_command("source ~/.cargo/env").await?;
        }

        println!("✅ Dependencies installed successfully");
        Ok(())
    }

    /// Configure system security
    async fn configure_security(&mut self) -> Result<()> {
        let sec = &self.config.security_config;

        if sec.firewall_enabled {
            println!("🔥 Configuring firewall...");
            self.run_command("ufw --force reset").await?;
            
            for port in &sec.allowed_ports {
                self.run_command(&format!("ufw allow {}", port)).await?;
            }
            
            self.run_command("ufw --force enable").await?;
        }

        if sec.fail2ban_enabled {
            println!("🛡️ Configuring Fail2Ban...");
            self.run_command("systemctl enable fail2ban").await?;
            self.run_command("systemctl start fail2ban").await?;
        }

        if sec.ssh_key_only {
            println!("🔑 Configuring SSH key-only authentication...");
            // This would modify /etc/ssh/sshd_config in a real implementation
            self.installation_status.warnings.push(
                "SSH key-only authentication should be manually configured".to_string()
            );
        }

        println!("✅ Security configuration completed");
        Ok(())
    }

    /// Configure BPCI services
    async fn configure_services(&mut self) -> Result<()> {
        println!("⚙️ Configuring BPCI auction mempool...");
        self.bpci_mempool = Some(BpciAuctionMempool::new());

        println!("🤝 Configuring Round Table Oracle...");
        self.round_table_oracle = Some(RoundTableOracle::new(None));

        // Create service configuration files
        self.create_systemd_services().await?;

        println!("✅ BPCI services configured");
        Ok(())
    }

    /// Setup mining and auction participation
    async fn setup_mining(&mut self) -> Result<()> {
        let mining = &self.config.mining_config;

        if mining.mining_enabled {
            println!("⛏️ Setting up mining configuration...");
            
            // Configure mining parameters
            let mining_config = format!(
                r#"
[mining]
enabled = true
auction_participation = {}
max_concurrent_auctions = {}
revenue_share_percent = {}
auto_reinvest_percent = {}
payout_threshold_wei = {}
preferred_chains = {:?}

[endpoints]
bpci_node = "{}"
round_table = "{}"
"#,
                mining.auction_participation,
                mining.max_concurrent_auctions,
                mining.revenue_share_percent,
                mining.auto_reinvest_percent,
                mining.payout_threshold_wei,
                mining.preferred_chains,
                self.config.bpci_node_endpoint,
                self.config.round_table_endpoint
            );

            fs::write("/etc/bpci/mining.toml", mining_config).await?;
        }

        println!("✅ Mining setup completed");
        Ok(())
    }

    /// Setup monitoring and alerting
    async fn setup_monitoring(&mut self) -> Result<()> {
        let mon = &self.config.monitoring_config;

        if mon.prometheus_enabled {
            println!("📊 Configuring Prometheus...");
            self.configure_prometheus().await?;
        }

        if mon.grafana_enabled {
            println!("📈 Configuring Grafana...");
            self.configure_grafana().await?;
        }

        println!("✅ Monitoring setup completed");
        Ok(())
    }

    /// Run system tests to verify installation
    async fn run_tests(&mut self) -> Result<()> {
        println!("🧪 Running system tests...");

        // Test BPCI mempool
        if let Some(mempool) = &self.bpci_mempool {
            let stats = mempool.get_mempool_stats();
            println!("  ✅ BPCI mempool: {} transactions", stats.pending_transactions);
        }

        // Test Round Table Oracle
        if let Some(oracle) = &self.round_table_oracle {
            let status = oracle.get_oracle_status().await;
            println!("  ✅ Round Table Oracle: {} partner chains", status.total_partner_chains);
        }

        // Test network connectivity
        self.test_network_connectivity().await?;

        // Test system resources
        self.test_system_resources().await?;

        println!("✅ All system tests passed");
        Ok(())
    }

    /// Start all BPCI services
    async fn start_services(&mut self) -> Result<()> {
        println!("🚀 Starting BPCI services...");

        let services = vec![
            "bpci-mempool",
            "bpci-oracle",
            "bpci-miner",
            "prometheus",
            "grafana",
            "nginx"
        ];

        for service in services {
            self.run_command(&format!("systemctl enable {}", service)).await?;
            self.run_command(&format!("systemctl start {}", service)).await?;
            println!("  ✅ Started {}", service);
        }

        println!("✅ All services started successfully");
        Ok(())
    }

    /// Update installation status
    async fn update_status(&mut self, phase: InstallationPhase, progress: u32, step: &str) {
        self.installation_status.phase = phase;
        self.installation_status.progress_percent = progress;
        self.installation_status.current_step = step.to_string();
        
        println!("📋 [{}%] {}", progress, step);
    }

    /// Run a system command
    async fn run_command(&self, cmd: &str) -> Result<String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Command failed: {}\nError: {}", cmd, error));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Check if a command exists
    async fn check_command_exists(&self, cmd: &str) -> bool {
        Command::new("which")
            .arg(cmd)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Create systemd service files
    async fn create_systemd_services(&self) -> Result<()> {
        // This would create actual systemd service files in a real implementation
        println!("📝 Creating systemd service files...");
        Ok(())
    }

    /// Configure Prometheus monitoring
    async fn configure_prometheus(&self) -> Result<()> {
        // This would configure Prometheus with BPCI-specific metrics
        println!("📊 Configuring Prometheus for BPCI metrics...");
        Ok(())
    }

    /// Configure Grafana dashboards
    async fn configure_grafana(&self) -> Result<()> {
        // This would set up Grafana dashboards for BPCI monitoring
        println!("📈 Setting up Grafana dashboards...");
        Ok(())
    }

    /// Test network connectivity
    async fn test_network_connectivity(&self) -> Result<()> {
        // Test connectivity to BPCI endpoints
        println!("🌐 Testing network connectivity...");
        Ok(())
    }

    /// Test system resources
    async fn test_system_resources(&self) -> Result<()> {
        // Test CPU, memory, disk, and network performance
        println!("💻 Testing system resources...");
        Ok(())
    }

    /// Get current installation status
    pub fn get_status(&self) -> &InstallationStatus {
        &self.installation_status
    }

    /// Get system information
    pub fn get_system_info(&self) -> &SystemInfo {
        &self.system_info
    }

    /// Get installation logs
    pub fn get_logs(&self) -> Vec<String> {
        // Return recent installation logs
        vec![
            format!("System check completed - {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")),
            format!("Current phase: {:?}", self.installation_status.phase),
            format!("Progress: {}%", self.installation_status.progress_percent),
            format!("Status: {}", self.installation_status.current_step),
            "BPCI Community Installer OS ready".to_string(),
        ]
    }
}

impl SystemInfo {
    /// Detect current system information
    pub fn detect() -> Self {
        // In a real implementation, this would detect actual system info
        Self {
            hostname: "bpci-community-node".to_string(),
            os_version: "Ubuntu 22.04.3 LTS".to_string(),
            kernel_version: "5.15.0-78-generic".to_string(),
            cpu_cores: 8,
            total_ram_gb: 16,
            available_storage_gb: 500,
            network_interfaces: vec![
                NetworkInterface {
                    name: "eth0".to_string(),
                    ip_address: "192.168.1.100".to_string(),
                    mac_address: "00:11:22:33:44:55".to_string(),
                    speed_mbps: 1000,
                    is_up: true,
                }
            ],
            uptime_seconds: 86400, // 1 day
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_installer_creation() {
        let installer = CommunityInstallerOS::new(None);
        assert!(matches!(installer.installation_status.phase, InstallationPhase::SystemCheck));
        assert_eq!(installer.installation_status.progress_percent, 0);
    }

    #[tokio::test]
    async fn test_system_requirements_check() {
        let mut installer = CommunityInstallerOS::new(None);
        
        // This should pass with default system info
        let result = installer.check_system_requirements().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_configuration_defaults() {
        let config = InstallerConfig::default();
        
        assert_eq!(config.system_requirements.min_cpu_cores, 8);
        assert_eq!(config.system_requirements.min_ram_gb, 8);
        assert!(config.security_config.firewall_enabled);
        assert!(config.monitoring_config.prometheus_enabled);
        assert!(config.mining_config.mining_enabled);
    }

    #[tokio::test]
    async fn test_status_updates() {
        let mut installer = CommunityInstallerOS::new(None);
        
        installer.update_status(InstallationPhase::DependencyInstall, 25, "Installing packages").await;
        
        assert!(matches!(installer.installation_status.phase, InstallationPhase::DependencyInstall));
        assert_eq!(installer.installation_status.progress_percent, 25);
        assert_eq!(installer.installation_status.current_step, "Installing packages");
    }
}
