//! Environment Variable Configuration System for BPI Infrastructure
//! 
//! Provides comprehensive configuration management with environment variable support

use serde::{Serialize, Deserialize};
use std::env;
use std::path::PathBuf;
use crate::errors::{BpiError, BpiResult};

/// Main BPI configuration with environment variable support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpiConfig {
    pub network: NetworkConfig,
    pub security: SecurityConfig,
    pub storage: StorageConfig,
    pub logging: LoggingConfig,
    pub pilot: PilotConfig,
    pub services: ServicesConfig,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub domain: String,
    pub vm_port: u16,
    pub bpci_port: u16,
    pub db_port: u16,
    pub orchestrator_port: u16,
    pub bind_address: String,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub quantum_safe: bool,
    pub audit_enabled: bool,
    pub compliance_mode: String,
    pub tls_enabled: bool,
    pub auth_required: bool,
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub data_dir: PathBuf,
    pub backup_enabled: bool,
    pub backup_dir: Option<PathBuf>,
    pub max_storage_gb: Option<u64>,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub output: PathBuf,
    pub max_file_size_mb: u64,
    pub max_files: u32,
}

/// Pilot-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PilotConfig {
    pub enabled: bool,
    pub auto_setup: bool,
    pub health_checks: bool,
    pub sample_data: bool,
    pub demo_mode: bool,
}

/// Services configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicesConfig {
    pub vm_server_enabled: bool,
    pub bpci_bridge_enabled: bool,
    pub database_enabled: bool,
    pub orchestrator_enabled: bool,
    pub monitoring_enabled: bool,
}

impl Default for BpiConfig {
    fn default() -> Self {
        Self {
            network: NetworkConfig::default(),
            security: SecurityConfig::default(),
            storage: StorageConfig::default(),
            logging: LoggingConfig::default(),
            pilot: PilotConfig::default(),
            services: ServicesConfig::default(),
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            domain: "localhost".to_string(),
            vm_port: 8080,
            bpci_port: 8545,
            db_port: 27017,
            orchestrator_port: 9090,
            bind_address: "0.0.0.0".to_string(),
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            quantum_safe: true,
            audit_enabled: true,
            compliance_mode: "pilot".to_string(),
            tls_enabled: false, // Disabled for pilot ease
            auth_required: false, // Disabled for pilot ease
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("./data"),
            backup_enabled: true,
            backup_dir: Some(PathBuf::from("./backups")),
            max_storage_gb: Some(10), // 10GB limit for pilots
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "json".to_string(),
            output: PathBuf::from("./logs/bpi.log"),
            max_file_size_mb: 100,
            max_files: 5,
        }
    }
}

impl Default for PilotConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_setup: true,
            health_checks: true,
            sample_data: true,
            demo_mode: false,
        }
    }
}

impl Default for ServicesConfig {
    fn default() -> Self {
        Self {
            vm_server_enabled: true,
            bpci_bridge_enabled: true,
            database_enabled: true,
            orchestrator_enabled: true,
            monitoring_enabled: true,
        }
    }
}

impl BpiConfig {
    /// Load configuration from environment variables with fallback to defaults
    pub fn from_env() -> BpiResult<Self> {
        let mut config = Self::default();
        
        // Network configuration from environment
        if let Ok(domain) = env::var("BPI_DOMAIN") {
            config.network.domain = domain;
        }
        
        if let Ok(port_str) = env::var("BPI_VM_PORT") {
            config.network.vm_port = port_str.parse()
                .map_err(|_| BpiError::config_error("Invalid BPI_VM_PORT", None, Some("BPI_VM_PORT")))?;
        }
        
        if let Ok(port_str) = env::var("BPI_BPCI_PORT") {
            config.network.bpci_port = port_str.parse()
                .map_err(|_| BpiError::config_error("Invalid BPI_BPCI_PORT", None, Some("BPI_BPCI_PORT")))?;
        }
        
        if let Ok(port_str) = env::var("BPI_DB_PORT") {
            config.network.db_port = port_str.parse()
                .map_err(|_| BpiError::config_error("Invalid BPI_DB_PORT", None, Some("BPI_DB_PORT")))?;
        }
        
        if let Ok(port_str) = env::var("BPI_ORCHESTRATOR_PORT") {
            config.network.orchestrator_port = port_str.parse()
                .map_err(|_| BpiError::config_error("Invalid BPI_ORCHESTRATOR_PORT", None, Some("BPI_ORCHESTRATOR_PORT")))?;
        }
        
        if let Ok(bind_addr) = env::var("BPI_BIND_ADDRESS") {
            config.network.bind_address = bind_addr;
        }
        
        // Security configuration from environment
        if let Ok(quantum_safe) = env::var("BPI_QUANTUM_SAFE") {
            config.security.quantum_safe = quantum_safe.parse().unwrap_or(true);
        }
        
        if let Ok(audit_enabled) = env::var("BPI_AUDIT_ENABLED") {
            config.security.audit_enabled = audit_enabled.parse().unwrap_or(true);
        }
        
        if let Ok(compliance_mode) = env::var("BPI_COMPLIANCE_MODE") {
            config.security.compliance_mode = compliance_mode;
        }
        
        if let Ok(tls_enabled) = env::var("BPI_TLS_ENABLED") {
            config.security.tls_enabled = tls_enabled.parse().unwrap_or(false);
        }
        
        // Storage configuration from environment
        if let Ok(data_dir) = env::var("BPI_DATA_DIR") {
            config.storage.data_dir = PathBuf::from(data_dir);
        }
        
        if let Ok(backup_enabled) = env::var("BPI_BACKUP_ENABLED") {
            config.storage.backup_enabled = backup_enabled.parse().unwrap_or(true);
        }
        
        if let Ok(backup_dir) = env::var("BPI_BACKUP_DIR") {
            config.storage.backup_dir = Some(PathBuf::from(backup_dir));
        }
        
        // Logging configuration from environment
        if let Ok(log_level) = env::var("BPI_LOG_LEVEL") {
            config.logging.level = log_level;
        }
        
        if let Ok(log_format) = env::var("BPI_LOG_FORMAT") {
            config.logging.format = log_format;
        }
        
        if let Ok(log_output) = env::var("BPI_LOG_OUTPUT") {
            config.logging.output = PathBuf::from(log_output);
        }
        
        // Pilot configuration from environment
        if let Ok(pilot_enabled) = env::var("BPI_PILOT_MODE") {
            config.pilot.enabled = pilot_enabled.parse().unwrap_or(true);
        }
        
        if let Ok(auto_setup) = env::var("BPI_AUTO_SETUP") {
            config.pilot.auto_setup = auto_setup.parse().unwrap_or(true);
        }
        
        if let Ok(demo_mode) = env::var("BPI_DEMO_MODE") {
            config.pilot.demo_mode = demo_mode.parse().unwrap_or(false);
        }
        
        // Services configuration from environment
        if let Ok(vm_enabled) = env::var("BPI_VM_SERVER_ENABLED") {
            config.services.vm_server_enabled = vm_enabled.parse().unwrap_or(true);
        }
        
        if let Ok(bpci_enabled) = env::var("BPI_BPCI_BRIDGE_ENABLED") {
            config.services.bpci_bridge_enabled = bpci_enabled.parse().unwrap_or(true);
        }
        
        if let Ok(db_enabled) = env::var("BPI_DATABASE_ENABLED") {
            config.services.database_enabled = db_enabled.parse().unwrap_or(true);
        }
        
        Ok(config)
    }
    
    /// Load configuration from TOML file with environment variable overrides
    pub fn from_file_with_env_override(file_path: &str) -> BpiResult<Self> {
        // First try to load from file
        let config = if std::path::Path::new(file_path).exists() {
            let content = std::fs::read_to_string(file_path)
                .map_err(|e| BpiError::config_error(&format!("Failed to read config file: {}", e), Some(file_path), None))?;
            
            toml::from_str::<Self>(&content)
                .map_err(|e| BpiError::config_error(&format!("Failed to parse TOML: {}", e), Some(file_path), None))?
        } else {
            Self::default()
        };
        
        // Then apply environment variable overrides
        Self::apply_env_overrides(config)
    }
    
    /// Apply environment variable overrides to existing config
    fn apply_env_overrides(mut config: Self) -> BpiResult<Self> {
        let env_config = Self::from_env()?;
        
        // Override with environment values if they differ from defaults
        let default_config = Self::default();
        
        if env_config.network.domain != default_config.network.domain {
            config.network.domain = env_config.network.domain;
        }
        
        if env_config.network.vm_port != default_config.network.vm_port {
            config.network.vm_port = env_config.network.vm_port;
        }
        
        // Apply other overrides as needed...
        
        Ok(config)
    }
    
    /// Validate configuration
    pub fn validate(&self) -> BpiResult<()> {
        // Validate ports are in valid range
        let ports = [
            ("vm_port", self.network.vm_port),
            ("bpci_port", self.network.bpci_port),
            ("db_port", self.network.db_port),
            ("orchestrator_port", self.network.orchestrator_port),
        ];
        
        for (name, port) in ports {
            if port < 1024 || port > 65535 {
                return Err(BpiError::config_error(
                    &format!("Port {} is out of valid range (1024-65535)", port),
                    None,
                    Some(name)
                ));
            }
        }
        
        // Validate data directory is writable
        if let Err(e) = std::fs::create_dir_all(&self.storage.data_dir) {
            return Err(BpiError::config_error(
                &format!("Cannot create data directory: {}", e),
                None,
                Some("data_dir")
            ));
        }
        
        // Validate log level
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.logging.level.as_str()) {
            return Err(BpiError::config_error(
                &format!("Invalid log level: {}", self.logging.level),
                None,
                Some("log_level")
            ));
        }
        
        Ok(())
    }
    
    /// Get environment-specific configuration
    pub fn for_environment(env: &str) -> BpiResult<Self> {
        let config_file = format!("./config/bpi-{}-config.toml", env);
        
        // Set environment variable for this session
        env::set_var("BPI_ENV", env);
        
        Self::from_file_with_env_override(&config_file)
    }
    
    /// Generate sample configuration file
    pub fn generate_sample_config(env: &str) -> BpiResult<String> {
        let config = Self::default();
        let toml_content = toml::to_string_pretty(&config)
            .map_err(|e| BpiError::config_error(&format!("Failed to serialize config: {}", e), None, None))?;
        
        let header = format!(
            "# BPI Core Configuration - {} Environment\n# Generated automatically\n\n",
            env.to_uppercase()
        );
        
        Ok(format!("{}{}", header, toml_content))
    }
    
    /// Get all environment variables that affect configuration
    pub fn get_env_vars() -> Vec<(&'static str, &'static str)> {
        vec![
            ("BPI_ENV", "Environment name (pilot/dev/staging/prod)"),
            ("BPI_DOMAIN", "Domain name for services"),
            ("BPI_VM_PORT", "VM Server port"),
            ("BPI_BPCI_PORT", "BPCI Bridge port"),
            ("BPI_DB_PORT", "4D Database port"),
            ("BPI_ORCHESTRATOR_PORT", "Service Orchestrator port"),
            ("BPI_BIND_ADDRESS", "Bind address for services"),
            ("BPI_QUANTUM_SAFE", "Enable quantum-safe cryptography"),
            ("BPI_AUDIT_ENABLED", "Enable audit logging"),
            ("BPI_COMPLIANCE_MODE", "Compliance mode"),
            ("BPI_TLS_ENABLED", "Enable TLS"),
            ("BPI_DATA_DIR", "Data directory path"),
            ("BPI_BACKUP_ENABLED", "Enable backups"),
            ("BPI_BACKUP_DIR", "Backup directory path"),
            ("BPI_LOG_LEVEL", "Logging level"),
            ("BPI_LOG_FORMAT", "Log format (json/text)"),
            ("BPI_LOG_OUTPUT", "Log output file"),
            ("BPI_PILOT_MODE", "Enable pilot mode"),
            ("BPI_AUTO_SETUP", "Enable automatic setup"),
            ("BPI_DEMO_MODE", "Enable demo mode"),
            ("BPI_VM_SERVER_ENABLED", "Enable VM Server"),
            ("BPI_BPCI_BRIDGE_ENABLED", "Enable BPCI Bridge"),
            ("BPI_DATABASE_ENABLED", "Enable 4D Database"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_config() {
        let config = BpiConfig::default();
        assert_eq!(config.network.domain, "localhost");
        assert_eq!(config.network.vm_port, 8080);
        assert!(config.pilot.enabled);
    }

    #[test]
    fn test_env_var_override() {
        env::set_var("BPI_DOMAIN", "test.example.com");
        env::set_var("BPI_VM_PORT", "9999");
        
        let config = BpiConfig::from_env().unwrap();
        assert_eq!(config.network.domain, "test.example.com");
        assert_eq!(config.network.vm_port, 9999);
        
        // Cleanup
        env::remove_var("BPI_DOMAIN");
        env::remove_var("BPI_VM_PORT");
    }

    #[test]
    fn test_config_validation() {
        let mut config = BpiConfig::default();
        
        // Valid config should pass
        assert!(config.validate().is_ok());
        
        // Invalid port should fail
        config.network.vm_port = 99999;
        assert!(config.validate().is_err());
    }
}
