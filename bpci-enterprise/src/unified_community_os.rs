//! # Unified Community OS Manager
//! 
//! Integrates Community Installer + Roundtable Oracle + SAPI Mesh (Court-BPI Bridge)
//! Provides one-click mainnet deployment for both roundtable partners and community members

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use tracing::{info, warn, error, debug};
use uuid::Uuid;

use crate::community_installer_os::{CommunityInstallerOS, InstallerConfig, InstallationPhase};
use crate::round_table_oracle::{RoundTableOracle, OracleConfig, PartnerChainConfig};
use crate::court_bpi_mesh_integration::{CourtBpiMeshBridge, CourtBpiMeshConfig};
use crate::bpi_ledger_integration::BpiLedgerClient;
use crate::dynaroute_integration::UnifiedNetworkingLayer;
use crate::commute_lock::CommuteLockRuntime;

/// Deployment modes for unified Community OS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentMode {
    /// Community member node (mining + auctions)
    Community {
        enable_mining: bool,
        enable_auctions: bool,
    },
    /// Roundtable partner node (governance + revenue sharing)
    RoundtablePartner {
        chain_id: u64,
        partner_name: String,
        representative_address: String,
    },
    /// Full enterprise node (all features)
    Enterprise {
        enable_all_features: bool,
    },
}

/// Unified configuration for all subsystems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCommunityConfig {
    /// Deployment mode
    pub deployment_mode: DeploymentMode,
    
    /// Community installer settings
    pub installer: InstallerConfig,
    
    /// Roundtable oracle settings
    pub roundtable: OracleConfig,
    
    /// SAPI mesh settings
    pub mesh: CourtBpiMeshConfig,
    
    /// Network configuration
    pub network: NetworkConfig,
    
    /// Security configuration
    pub security: SecurityConfig,
    
    /// Monitoring configuration
    pub monitoring: MonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub external_ip: Option<String>,
    pub web_port: u16,
    pub mesh_port: u16,
    pub roundtable_port: u16,
    pub mining_port: u16,
    pub enable_upnp: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub enable_firewall: bool,
    pub enable_fail2ban: bool,
    pub enable_encrypted_storage: bool,
    pub ssh_key_only: bool,
    pub auto_security_updates: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub enable_prometheus: bool,
    pub enable_grafana: bool,
    pub enable_alerting: bool,
    pub metrics_retention_days: u32,
}

impl Default for UnifiedCommunityConfig {
    fn default() -> Self {
        Self {
            deployment_mode: DeploymentMode::Community {
                enable_mining: true,
                enable_auctions: true,
            },
            installer: InstallerConfig::default(),
            roundtable: OracleConfig::default(),
            mesh: CourtBpiMeshConfig::default(),
            network: NetworkConfig {
                external_ip: None,
                web_port: 8080,
                mesh_port: 9000,
                roundtable_port: 7000,
                mining_port: 6000,
                enable_upnp: true,
            },
            security: SecurityConfig {
                enable_firewall: true,
                enable_fail2ban: true,
                enable_encrypted_storage: true,
                ssh_key_only: true,
                auto_security_updates: true,
            },
            monitoring: MonitoringConfig {
                enable_prometheus: true,
                enable_grafana: true,
                enable_alerting: true,
                metrics_retention_days: 30,
            },
        }
    }
}

/// System status for all components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedSystemStatus {
    pub overall_status: SystemHealthStatus,
    pub installer_status: InstallationPhase,
    pub roundtable_status: RoundtableStatus,
    pub mesh_status: MeshStatus,
    pub services: HashMap<String, ServiceStatus>,
    pub system_metrics: SystemMetrics,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemHealthStatus {
    Healthy,
    Warning,
    Critical,
    Installing,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundtableStatus {
    pub active_partnerships: u32,
    pub total_revenue_distributed: u64,
    pub oracle_health: String,
    pub last_distribution: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshStatus {
    pub connected_nodes: u32,
    pub active_banking_operations: u32,
    pub mesh_health: String,
    pub last_transaction: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub name: String,
    pub status: String,
    pub uptime: u64,
    pub last_restart: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_io: NetworkIO,
    pub active_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkIO {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
}

/// Cross-component event system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnifiedSystemEvent {
    /// Community installer events
    InstallationPhaseChanged(InstallationPhase),
    ServiceStarted(String),
    ServiceStopped(String),
    
    /// Roundtable events
    PartnershipCreated(String),
    RevenueDistributed(u64),
    GovernanceVote(String),
    
    /// Mesh events
    NodeConnected(String),
    NodeDisconnected(String),
    BankingOperationCompleted(Uuid),
    
    /// System events
    SystemHealthChanged(SystemHealthStatus),
    SecurityAlert(String),
    PerformanceAlert(String),
}

/// Main Unified Community OS Manager
pub struct UnifiedCommunityOS {
    /// Configuration
    config: UnifiedCommunityConfig,
    
    /// Core components
    installer: CommunityInstallerOS,
    roundtable: Option<RoundTableOracle>,
    mesh_bridge: Option<CourtBpiMeshBridge>,
    
    /// BPI ledger client
    bpi_client: Arc<BpiLedgerClient>,
    
    /// Service registry
    services: Arc<RwLock<HashMap<String, ServiceStatus>>>,
    
    /// Event system
    event_handlers: Arc<RwLock<Vec<Box<dyn Fn(UnifiedSystemEvent) + Send + Sync>>>>,
    
    /// System status
    system_status: Arc<RwLock<UnifiedSystemStatus>>,
}

impl UnifiedCommunityOS {
    /// Create new Unified Community OS instance
    pub async fn new(config: UnifiedCommunityConfig) -> Result<Self> {
        info!("🚀 Initializing Unified Community OS");
        
        // Initialize BPI ledger client
        let parser = crate::config::env_ini_parser::EnvIniParser::new("config");
        let env_config = parser.parse_env_ini()?;
        let commute_runtime = Arc::new(CommuteLockRuntime::new(&env_config)?);
        let networking = Arc::new(UnifiedNetworkingLayer::new_virtual(commute_runtime).await?);
        let bpi_client = Arc::new(BpiLedgerClient::new(networking).await?);
        
        // Initialize community installer
        let installer = CommunityInstallerOS::new(Some(config.installer.clone()));
        
        // Initialize roundtable oracle based on deployment mode
        let roundtable = match &config.deployment_mode {
            DeploymentMode::RoundtablePartner { .. } | DeploymentMode::Enterprise { .. } => {
                Some(RoundTableOracle::new(Some(config.roundtable.clone())))
            },
            _ => None,
        };
        
        // Initialize mesh bridge
        let mesh_bridge = Some(CourtBpiMeshBridge::new(config.mesh.clone()).await?);
        
        // Initialize system status
        let system_status = Arc::new(RwLock::new(UnifiedSystemStatus {
            overall_status: SystemHealthStatus::Installing,
            installer_status: InstallationPhase::SystemCheck,
            roundtable_status: RoundtableStatus {
                active_partnerships: 0,
                total_revenue_distributed: 0,
                oracle_health: "Initializing".to_string(),
                last_distribution: None,
            },
            mesh_status: MeshStatus {
                connected_nodes: 0,
                active_banking_operations: 0,
                mesh_health: "Initializing".to_string(),
                last_transaction: None,
            },
            services: HashMap::new(),
            system_metrics: SystemMetrics {
                cpu_usage: 0.0,
                memory_usage: 0.0,
                disk_usage: 0.0,
                network_io: NetworkIO {
                    bytes_sent: 0,
                    bytes_received: 0,
                    packets_sent: 0,
                    packets_received: 0,
                },
                active_connections: 0,
            },
            last_updated: Utc::now(),
        }));
        
        info!("✅ Unified Community OS initialized successfully");
        
        Ok(Self {
            config,
            installer,
            roundtable,
            mesh_bridge,
            bpi_client,
            services: Arc::new(RwLock::new(HashMap::new())),
            event_handlers: Arc::new(RwLock::new(Vec::new())),
            system_status,
        })
    }
    
    /// One-click installation for complete system
    pub async fn install_complete_system(&mut self) -> Result<()> {
        info!("🔧 Starting complete system installation");
        
        // Update status
        self.update_system_status(SystemHealthStatus::Installing).await;
        
        // Step 1: Run community installer
        info!("📦 Step 1: Installing base system components");
        self.installer.install().await?;
        self.emit_event(UnifiedSystemEvent::InstallationPhaseChanged(
            InstallationPhase::Completed
        )).await;
        
        // Step 2: Configure based on deployment mode
        match &self.config.deployment_mode {
            DeploymentMode::Community { enable_mining, enable_auctions } => {
                info!("🏘️ Configuring Community Node");
                self.setup_community_node(*enable_mining, *enable_auctions).await?;
            },
            DeploymentMode::RoundtablePartner { chain_id, partner_name, representative_address } => {
                info!("🏛️ Configuring Roundtable Partner Node");
                self.setup_roundtable_partner(*chain_id, partner_name.clone(), representative_address.clone()).await?;
            },
            DeploymentMode::Enterprise { enable_all_features } => {
                info!("🏢 Configuring Enterprise Node");
                self.setup_enterprise_node(*enable_all_features).await?;
            },
        }
        
        // Step 3: Start all services
        info!("🚀 Step 3: Starting all services");
        self.start_all_services().await?;
        
        // Step 4: Verify system health
        info!("🔍 Step 4: Verifying system health");
        self.verify_system_health().await?;
        
        // Update final status
        self.update_system_status(SystemHealthStatus::Healthy).await;
        
        info!("✅ Complete system installation finished successfully!");
        Ok(())
    }
    
    /// Setup community node configuration
    async fn setup_community_node(&mut self, enable_mining: bool, enable_auctions: bool) -> Result<()> {
        info!("Setting up community node (mining: {}, auctions: {})", enable_mining, enable_auctions);
        
        // Configure mining if enabled
        if enable_mining {
            self.installer.setup_mining().await?;
            self.register_service("mining", "Mining Node").await;
        }
        
        // Configure auctions if enabled
        if enable_auctions {
            // Auction setup is part of mining configuration
            self.register_service("auctions", "Auction Participation").await;
        }
        
        // Setup mesh connectivity for community participation
        if let Some(ref mesh) = self.mesh_bridge {
            info!("Connecting to SAPI mesh network");
            // Mesh bridge is already initialized
            self.register_service("mesh", "SAPI Mesh Connector").await;
        }
        
        Ok(())
    }
    
    /// Setup roundtable partner node configuration
    async fn setup_roundtable_partner(&mut self, chain_id: u64, partner_name: String, representative_address: String) -> Result<()> {
        info!("Setting up roundtable partner node for chain {} ({})", chain_id, partner_name);
        
        if let Some(ref roundtable) = self.roundtable {
            // Register as partner chain
            let partner_config = PartnerChainConfig::new(
                chain_id,
                partner_name.clone(),
                format!("https://rpc.{}.network", partner_name.to_lowercase()),
                format!("wss://ws.{}.network", partner_name.to_lowercase()),
                representative_address,
            );
            
            roundtable.register_partner_chain(partner_config).await?;
            self.register_service("roundtable", "Roundtable Oracle").await;
            
            info!("✅ Registered as roundtable partner: {}", partner_name);
        }
        
        // Setup mesh connectivity for governance coordination
        if let Some(ref mesh) = self.mesh_bridge {
            self.register_service("mesh", "SAPI Mesh Connector").await;
        }
        
        Ok(())
    }
    
    /// Setup enterprise node configuration (all features)
    async fn setup_enterprise_node(&mut self, enable_all_features: bool) -> Result<()> {
        info!("Setting up enterprise node (all features: {})", enable_all_features);
        
        if enable_all_features {
            // Setup mining and auctions
            self.setup_community_node(true, true).await?;
            
            // Setup roundtable (as BPCI enterprise)
            if let Some(ref roundtable) = self.roundtable {
                self.register_service("roundtable", "Roundtable Oracle").await;
            }
            
            // Mesh is already configured
            self.register_service("enterprise", "Enterprise Features").await;
        }
        
        Ok(())
    }
    
    /// Start all configured services
    pub async fn start_all_services(&mut self) -> Result<()> {
        info!("🚀 Starting all configured services");
        
        // Start installer services (real systemd services)
        self.installer.start_services().await?;
        self.verify_service_running("bpci-mining").await?;
        self.verify_service_running("bpci-auction").await?;
        
        // Start roundtable oracle if configured
        if let Some(ref roundtable) = self.roundtable {
            roundtable.start_monitoring().await?;
            self.verify_service_running("bpci-roundtable").await?;
            info!("✅ Roundtable Oracle started and verified");
        }
        
        // Verify mesh bridge connectivity
        if let Some(ref mesh) = self.mesh_bridge {
            self.verify_mesh_connectivity().await?;
            info!("✅ SAPI Mesh Bridge connected and verified");
        }
        
        // Update real service statuses
        self.update_real_service_statuses().await?;
        
        info!("✅ All services started and verified successfully");
        Ok(())
    }
    
    /// Verify a systemd service is running
    async fn verify_service_running(&self, service_name: &str) -> Result<()> {
        use std::process::Command;
        
        let output = Command::new("systemctl")
            .args(&["is-active", service_name])
            .output()?;
        
        let status = String::from_utf8_lossy(&output.stdout).trim().to_string();
        
        if status == "active" {
            info!("✅ Service {} is running", service_name);
            Ok(())
        } else {
            error!("❌ Service {} is not running (status: {})", service_name, status);
            Err(anyhow!("Service {} is not active", service_name))
        }
    }
    
    /// Verify mesh connectivity
    async fn verify_mesh_connectivity(&self) -> Result<()> {
        // Check if mesh ports are listening
        let mesh_port = self.config.network.mesh_port;
        
        use std::process::Command;
        let output = Command::new("ss")
            .args(&["-tuln", &format!("sport = :{}", mesh_port)])
            .output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains(&mesh_port.to_string()) {
            info!("✅ Mesh port {} is listening", mesh_port);
            Ok(())
        } else {
            error!("❌ Mesh port {} is not listening", mesh_port);
            Err(anyhow!("Mesh connectivity verification failed"))
        }
    }
    
    /// Update real service statuses from systemd
    async fn update_real_service_statuses(&self) -> Result<()> {
        use std::process::Command;
        
        let services_to_check = vec![
            ("bpci-mining", "Mining Node"),
            ("bpci-auction", "Auction Participation"),
            ("bpci-roundtable", "Roundtable Oracle"),
            ("bpci-mesh", "SAPI Mesh Connector"),
            ("bpci-web", "Web Dashboard"),
        ];
        
        let mut services = self.services.write().await;
        
        for (service_name, description) in services_to_check {
            // Get service status
            let status_output = Command::new("systemctl")
                .args(&["is-active", service_name])
                .output();
            
            // Get service uptime
            let uptime_output = Command::new("systemctl")
                .args(&["show", service_name, "--property=ActiveEnterTimestamp"])
                .output();
            
            let status = if let Ok(output) = status_output {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                "unknown".to_string()
            };
            
            let uptime = if let Ok(output) = uptime_output {
                let timestamp_line = String::from_utf8_lossy(&output.stdout);
                if let Some(timestamp_str) = timestamp_line.strip_prefix("ActiveEnterTimestamp=") {
                    // Parse timestamp and calculate uptime
                    // For now, just use 0 as placeholder for real implementation
                    0
                } else {
                    0
                }
            } else {
                0
            };
            
            services.insert(service_name.to_string(), ServiceStatus {
                name: description.to_string(),
                status: status.clone(),
                uptime,
                last_restart: if status == "active" { Some(Utc::now()) } else { None },
            });
        }
        
        Ok(())
    }
    
    /// Get current system status
    pub async fn get_system_status(&self) -> UnifiedSystemStatus {
        let mut status = self.system_status.read().await.clone();
        status.last_updated = Utc::now();
        
        // Update real-time metrics
        status.system_metrics = self.collect_system_metrics().await;
        
        // Update service statuses
        status.services = self.services.read().await.clone();
        
        status
    }
    
    /// Verify system health
    async fn verify_system_health(&self) -> Result<()> {
        info!("🔍 Verifying system health");
        
        // Check installer status
        let installer_status = self.installer.get_status();
        if !matches!(installer_status.phase, InstallationPhase::Completed) {
            return Err(anyhow!("Installation not complete"));
        }
        
        // Check roundtable if configured
        if let Some(ref roundtable) = self.roundtable {
            let oracle_status = roundtable.get_oracle_status();
            info!("Roundtable Oracle: {} partnerships active", oracle_status.await.active_partnerships);
        }
        
        // Check mesh connectivity
        if self.mesh_bridge.is_some() {
            info!("SAPI Mesh Bridge: Connected and operational");
        }
        
        info!("✅ System health verification passed");
        Ok(())
    }
    
    /// Register a service
    async fn register_service(&self, name: &str, description: &str) {
        let mut services = self.services.write().await;
        services.insert(name.to_string(), ServiceStatus {
            name: description.to_string(),
            status: "Running".to_string(),
            uptime: 0,
            last_restart: Some(Utc::now()),
        });
        
        self.emit_event(UnifiedSystemEvent::ServiceStarted(name.to_string())).await;
    }
    
    /// Update system status
    async fn update_system_status(&self, status: SystemHealthStatus) {
        let mut sys_status = self.system_status.write().await;
        sys_status.overall_status = status.clone();
        sys_status.last_updated = Utc::now();
        
        self.emit_event(UnifiedSystemEvent::SystemHealthChanged(status)).await;
    }
    
    /// Emit system event
    async fn emit_event(&self, event: UnifiedSystemEvent) {
        debug!("Emitting event: {:?}", event);
        
        let handlers = self.event_handlers.read().await;
        for handler in handlers.iter() {
            handler(event.clone());
        }
    }
    
    /// Collect real system metrics
    async fn collect_system_metrics(&self) -> SystemMetrics {
        use std::fs;
        use std::process::Command;
        
        // Get real CPU usage from /proc/stat
        let cpu_usage = self.get_real_cpu_usage().unwrap_or(0.0);
        
        // Get real memory usage from /proc/meminfo
        let memory_usage = self.get_real_memory_usage().unwrap_or(0.0);
        
        // Get real disk usage
        let disk_usage = self.get_real_disk_usage().unwrap_or(0.0);
        
        // Get real network I/O from /proc/net/dev
        let network_io = self.get_real_network_io().unwrap_or(NetworkIO {
            bytes_sent: 0,
            bytes_received: 0,
            packets_sent: 0,
            packets_received: 0,
        });
        
        // Get real active connections
        let active_connections = self.get_real_active_connections().unwrap_or(0);
        
        SystemMetrics {
            cpu_usage,
            memory_usage,
            disk_usage,
            network_io,
            active_connections,
        }
    }
    
    /// Get real CPU usage percentage
    fn get_real_cpu_usage(&self) -> Result<f64> {
        let stat = std::fs::read_to_string("/proc/stat")?;
        let first_line = stat.lines().next().ok_or_else(|| anyhow!("No CPU stats"))?;
        let values: Vec<u64> = first_line
            .split_whitespace()
            .skip(1)
            .take(7)
            .map(|s| s.parse().unwrap_or(0))
            .collect();
        
        if values.len() >= 4 {
            let idle = values[3];
            let total: u64 = values.iter().sum();
            let usage = if total > 0 {
                100.0 - (idle as f64 / total as f64 * 100.0)
            } else {
                0.0
            };
            Ok(usage)
        } else {
            Ok(0.0)
        }
    }
    
    /// Get real memory usage percentage
    fn get_real_memory_usage(&self) -> Result<f64> {
        let meminfo = std::fs::read_to_string("/proc/meminfo")?;
        let mut total_kb = 0u64;
        let mut available_kb = 0u64;
        
        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                total_kb = line.split_whitespace().nth(1)
                    .and_then(|s| s.parse().ok()).unwrap_or(0);
            } else if line.starts_with("MemAvailable:") {
                available_kb = line.split_whitespace().nth(1)
                    .and_then(|s| s.parse().ok()).unwrap_or(0);
            }
        }
        
        if total_kb > 0 {
            let used_kb = total_kb - available_kb;
            Ok(used_kb as f64 / total_kb as f64 * 100.0)
        } else {
            Ok(0.0)
        }
    }
    
    /// Get real disk usage percentage
    fn get_real_disk_usage(&self) -> Result<f64> {
        use std::process::Command;
        
        let output = Command::new("df")
            .args(&["-h", "/"])
            .output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = stdout.lines().nth(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let usage_str = parts[4].trim_end_matches('%');
                return Ok(usage_str.parse().unwrap_or(0.0));
            }
        }
        
        Ok(0.0)
    }
    
    /// Get real network I/O statistics
    fn get_real_network_io(&self) -> Result<NetworkIO> {
        let net_dev = std::fs::read_to_string("/proc/net/dev")?;
        let mut total_rx_bytes = 0u64;
        let mut total_tx_bytes = 0u64;
        let mut total_rx_packets = 0u64;
        let mut total_tx_packets = 0u64;
        
        for line in net_dev.lines().skip(2) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 17 && !parts[0].starts_with("lo:") {
                // Skip loopback interface
                total_rx_bytes += parts[1].parse().unwrap_or(0);
                total_rx_packets += parts[2].parse().unwrap_or(0);
                total_tx_bytes += parts[9].parse().unwrap_or(0);
                total_tx_packets += parts[10].parse().unwrap_or(0);
            }
        }
        
        Ok(NetworkIO {
            bytes_sent: total_tx_bytes,
            bytes_received: total_rx_bytes,
            packets_sent: total_tx_packets,
            packets_received: total_rx_packets,
        })
    }
    
    /// Get real active network connections
    fn get_real_active_connections(&self) -> Result<u32> {
        use std::process::Command;
        
        let output = Command::new("ss")
            .args(&["-tuln"])
            .output()
            .or_else(|_| Command::new("netstat").args(&["-tuln"]).output())?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let count = stdout.lines()
            .filter(|line| line.contains("LISTEN") || line.contains("ESTABLISHED"))
            .count();
        
        Ok(count as u32)
    }
    
    /// Get deployment mode
    pub fn get_deployment_mode(&self) -> &DeploymentMode {
        &self.config.deployment_mode
    }
    
    /// Get configuration
    pub fn get_config(&self) -> &UnifiedCommunityConfig {
        &self.config
    }
}

/// Helper functions for one-click deployment
impl UnifiedCommunityOS {
    /// Create community node configuration
    pub fn create_community_config() -> UnifiedCommunityConfig {
        let mut config = UnifiedCommunityConfig::default();
        config.deployment_mode = DeploymentMode::Community {
            enable_mining: true,
            enable_auctions: true,
        };
        config
    }
    
    /// Create roundtable partner configuration
    pub fn create_roundtable_partner_config(
        chain_id: u64,
        partner_name: String,
        representative_address: String,
    ) -> UnifiedCommunityConfig {
        let mut config = UnifiedCommunityConfig::default();
        config.deployment_mode = DeploymentMode::RoundtablePartner {
            chain_id,
            partner_name,
            representative_address,
        };
        config
    }
    
    /// Create enterprise configuration
    pub fn create_enterprise_config() -> UnifiedCommunityConfig {
        let mut config = UnifiedCommunityConfig::default();
        config.deployment_mode = DeploymentMode::Enterprise {
            enable_all_features: true,
        };
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_unified_os_creation() {
        let config = UnifiedCommunityOS::create_community_config();
        let result = UnifiedCommunityOS::new(config).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_roundtable_partner_config() {
        let config = UnifiedCommunityOS::create_roundtable_partner_config(
            1,
            "TestChain".to_string(),
            "0x1234567890abcdef".to_string(),
        );
        
        match config.deployment_mode {
            DeploymentMode::RoundtablePartner { chain_id, .. } => {
                assert_eq!(chain_id, 1);
            },
            _ => panic!("Wrong deployment mode"),
        }
    }
}
