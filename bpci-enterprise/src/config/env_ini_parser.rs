//! # Advanced Environment Configuration Parser
//! 
//! Revolutionary env.ini parser with envtoml.lock concept for BSO-K8 deployments
//! Supports vPod virtual environments, TOML configuration, and lock file versioning
//! 
//! ## Features:
//! - env.ini: Human-readable INI format for environment configuration
//! - env.toml: Structured TOML format for complex configurations
//! - envtoml.lock: Lock file for reproducible deployments (like Cargo.lock)
//! - vPod virtual environment isolation
//! - BSO-K8 orchestrator integration
//! - Cryptographic hash verification
//! - Version pinning and dependency resolution

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc};
use toml;

/// Environment configuration from env.ini
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvIniConfig {
    /// Configuration sections
    pub sections: HashMap<String, EnvSection>,
    /// Global variables
    pub globals: HashMap<String, String>,
    /// vPod virtual environment settings
    pub vpod_env: Option<VPodEnvironment>,
    /// BSO-K8 deployment settings
    pub bso_k8_config: Option<BsoK8Config>,
    /// commute.lock configuration for lock-based communication
    pub commute_lock_config: Option<CommuteLockConfig>,
}

/// Configuration section in env.ini
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvSection {
    pub name: String,
    pub variables: HashMap<String, EnvVariable>,
    pub metadata: SectionMetadata,
}

/// Environment variable with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVariable {
    pub key: String,
    pub value: String,
    pub var_type: VarType,
    pub required: bool,
    pub default: Option<String>,
    pub description: Option<String>,
    pub validation: Option<VarValidation>,
}

/// Variable type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VarType {
    String,
    Integer,
    Float,
    Boolean,
    Url,
    Path,
    Port,
    IpAddress,
    Secret,
    Json,
    Array,
}

/// Variable validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarValidation {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<String>,
    pub allowed_values: Option<Vec<String>>,
    pub min_value: Option<i64>,
    pub max_value: Option<i64>,
}

/// Section metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionMetadata {
    pub component: Option<String>,
    pub priority: u32,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// vPod virtual environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPodEnvironment {
    pub vpod_id: String,
    pub arena_size_mb: u64,
    pub max_vpods: u32,
    pub isolation_level: IsolationLevel,
    pub resource_limits: VPodResourceLimits,
    pub env_variables: HashMap<String, String>,
}

/// vPod isolation level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IsolationLevel {
    None,
    Namespace,
    Process,
    Full,
}

/// vPod resource limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPodResourceLimits {
    pub memory_mb: u64,
    pub cpu_cores: f32,
    pub disk_mb: u64,
    pub network_bandwidth_mbps: u32,
}

/// BSO-K8 deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BsoK8Config {
    pub orchestrator_id: String,
    pub deployment_strategy: DeploymentStrategy,
    pub service_type: String,
    pub replicas: u32,
    pub health_check: HealthCheckConfig,
    pub resource_allocation: ResourceAllocationConfig,
}

/// Deployment strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentStrategy {
    RollingUpdate,
    Recreate,
    BlueGreen,
    Canary,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub interval_seconds: u32,
    pub timeout_seconds: u32,
    pub retries: u32,
}

/// Resource allocation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocationConfig {
    pub memory_mb: u64,
    pub cpu_cores: f32,
    pub disk_mb: u64,
    pub vpod_count: u32,
}

/// envtoml.lock file structure (like Cargo.lock)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvTomlLock {
    /// Lock file version
    pub version: String,
    /// Locked configuration hash
    pub config_hash: String,
    /// Locked dependencies
    pub dependencies: HashMap<String, LockedDependency>,
    /// Locked environment variables
    pub locked_env: HashMap<String, LockedEnvVar>,
    /// Lock creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    /// vPod environment snapshot
    pub vpod_snapshot: Option<VPodSnapshot>,
    /// BSO-K8 deployment snapshot
    pub bso_k8_snapshot: Option<BsoK8Snapshot>,
    /// commute.lock snapshot
    pub commute_lock_snapshot: Option<CommuteLockSnapshot>,
}

/// Locked dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedDependency {
    pub name: String,
    pub version: String,
    pub source: String,
    pub checksum: String,
}

/// Locked environment variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedEnvVar {
    pub key: String,
    pub value_hash: String, // Hash of value for security
    pub var_type: VarType,
    pub locked_at: DateTime<Utc>,
}

/// vPod environment snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPodSnapshot {
    pub vpod_id: String,
    pub arena_size_mb: u64,
    pub active_vpods: u32,
    pub resource_usage: HashMap<String, u64>,
}

/// BSO-K8 deployment snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BsoK8Snapshot {
    pub orchestrator_id: String,
    pub deployed_services: Vec<String>,
    pub service_versions: HashMap<String, String>,
    pub deployment_hash: String,
}

/// commute.lock configuration for lock-based inter-component communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommuteLockConfig {
    /// Enable commute.lock communication
    pub enabled: bool,
    /// Communication mode (shared_memory, http, hybrid)
    pub communication_mode: CommunicationMode,
    /// Lock file directory
    pub lock_dir: PathBuf,
    /// Shared memory directory
    pub shm_dir: PathBuf,
    /// Event notification directory
    pub event_dir: PathBuf,
    /// Shared memory sizes per component (in MB)
    pub component_shm_sizes: HashMap<String, u64>,
    /// BPI address data configuration
    pub bpi_data_config: BpiDataConfig,
    /// Lock settings
    pub lock_settings: LockSettings,
    /// Event notification settings
    pub event_settings: EventSettings,
    /// Performance tuning settings
    pub performance: PerformanceSettings,
}

/// Communication mode for commute.lock
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommunicationMode {
    /// Pure shared memory (fastest, local only)
    SharedMemory,
    /// Pure HTTP (fallback, works remotely)
    Http,
    /// Hybrid: shared memory for local, HTTP for remote
    Hybrid,
}

/// BPI address data configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpiDataConfig {
    /// Directory for per-address data
    pub bpi_data_dir: PathBuf,
    /// Memory per BPI address (in MB)
    pub per_address_mb: u64,
    /// Maximum number of BPI addresses
    pub max_addresses: usize,
}

/// Lock settings for commute.lock
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockSettings {
    /// Lock timeout in milliseconds
    pub timeout_ms: u64,
    /// Number of retry attempts
    pub retry_count: u32,
    /// Enable lock monitoring
    pub enable_monitoring: bool,
}

/// Event notification settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSettings {
    /// Event buffer size
    pub buffer_size: usize,
    /// Event timeout in milliseconds
    pub timeout_ms: u64,
}

/// Performance tuning settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSettings {
    /// Enable zero-copy transfers
    pub zero_copy_enabled: bool,
    /// Use lock-free queues
    pub lock_free_queues: bool,
    /// NUMA-aware memory allocation
    pub numa_aware: bool,
}

/// commute.lock snapshot for envtoml.lock
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommuteLockSnapshot {
    pub enabled: bool,
    pub communication_mode: CommunicationMode,
    pub component_shm_sizes: HashMap<String, u64>,
    pub lock_dir: String,
    pub shm_dir: String,
    pub timestamp: DateTime<Utc>,
}

/// Advanced env.ini parser with envtoml.lock support
pub struct EnvIniParser {
    config_dir: PathBuf,
    env_ini_path: PathBuf,
    env_toml_path: PathBuf,
    lock_file_path: PathBuf,
}

impl EnvIniParser {
    /// Create new parser
    pub fn new<P: AsRef<Path>>(config_dir: P) -> Self {
        let config_dir = config_dir.as_ref().to_path_buf();
        let env_ini_path = config_dir.join("env.ini");
        let env_toml_path = config_dir.join("env.toml");
        let lock_file_path = config_dir.join("envtoml.lock");
        
        Self {
            config_dir,
            env_ini_path,
            env_toml_path,
            lock_file_path,
        }
    }
    
    /// Parse env.ini file
    pub fn parse_env_ini(&self) -> Result<EnvIniConfig> {
        let content = fs::read_to_string(&self.env_ini_path)?;
        let mut config = self.parse_ini_content(&content)?;
        
        // Parse commute.lock configuration
        self.parse_commute_lock_section(&mut config)?;
        
        Ok(config)
    }
    
    /// Parse INI content
    fn parse_ini_content(&self, content: &str) -> Result<EnvIniConfig> {
        let mut config = EnvIniConfig {
            sections: HashMap::new(),
            globals: HashMap::new(),
            vpod_env: None,
            bso_k8_config: None,
            commute_lock_config: None,
        };
        
        let mut current_section: Option<String> = None;
        let mut current_vars: HashMap<String, EnvVariable> = HashMap::new();
        
        for line in content.lines() {
            let line = line.trim();
            
            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }
            
            // Section header
            if line.starts_with('[') && line.ends_with(']') {
                // Save previous section
                if let Some(section_name) = current_section {
                    config.sections.insert(
                        section_name.clone(),
                        EnvSection {
                            name: section_name,
                            variables: current_vars.clone(),
                            metadata: SectionMetadata {
                                component: None,
                                priority: 0,
                                enabled: true,
                                tags: vec![],
                            },
                        },
                    );
                    current_vars.clear();
                }
                
                // Start new section
                current_section = Some(line[1..line.len()-1].to_string());
                continue;
            }
            
            // Key-value pair
            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().to_string();
                let value = line[eq_pos+1..].trim().to_string();
                
                let env_var = EnvVariable {
                    key: key.clone(),
                    value,
                    var_type: VarType::String,
                    required: false,
                    default: None,
                    description: None,
                    validation: None,
                };
                
                if let Some(_) = &current_section {
                    current_vars.insert(key, env_var);
                } else {
                    config.globals.insert(key, env_var.value);
                }
            }
        }
        
        // Save last section
        if let Some(section_name) = current_section {
            config.sections.insert(
                section_name.clone(),
                EnvSection {
                    name: section_name,
                    variables: current_vars,
                    metadata: SectionMetadata {
                        component: None,
                        priority: 0,
                        enabled: true,
                        tags: vec![],
                    },
                },
            );
        }
        
        Ok(config)
    }
    
    /// Parse env.toml file
    pub fn parse_env_toml(&self) -> Result<EnvIniConfig> {
        let content = fs::read_to_string(&self.env_toml_path)?;
        let config: EnvIniConfig = toml::from_str(&content)?;
        Ok(config)
    }
    
    /// Generate envtoml.lock file
    pub fn generate_lock_file(&self, config: &EnvIniConfig) -> Result<EnvTomlLock> {
        let config_hash = self.calculate_config_hash(config)?;
        
        let mut locked_env = HashMap::new();
        for (section_name, section) in &config.sections {
            for (var_key, var) in &section.variables {
                let value_hash = self.hash_value(&var.value);
                locked_env.insert(
                    format!("{}.{}", section_name, var_key),
                    LockedEnvVar {
                        key: var_key.clone(),
                        value_hash,
                        var_type: var.var_type.clone(),
                        locked_at: Utc::now(),
                    },
                );
            }
        }
        
        let mut lock = EnvTomlLock {
            version: "1.0.0".to_string(),
            config_hash,
            dependencies: HashMap::new(),
            locked_env,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            vpod_snapshot: config.vpod_env.as_ref().map(|vpod| VPodSnapshot {
                vpod_id: vpod.vpod_id.clone(),
                arena_size_mb: vpod.arena_size_mb,
                active_vpods: vpod.max_vpods,
                resource_usage: HashMap::new(),
            }),
            bso_k8_snapshot: config.bso_k8_config.as_ref().map(|bso| BsoK8Snapshot {
                orchestrator_id: bso.orchestrator_id.clone(),
                deployed_services: vec![],
                service_versions: HashMap::new(),
                deployment_hash: String::new(),
            }),
            commute_lock_snapshot: None,
        };
        
        // Add commute.lock snapshot
        self.export_commute_lock_to_lock_file(config, &mut lock)?;
        
        Ok(lock)
    }
    
    /// Save lock file
    pub fn save_lock_file(&self, lock: &EnvTomlLock) -> Result<()> {
        let content = toml::to_string_pretty(lock)?;
        fs::write(&self.lock_file_path, content)?;
        Ok(())
    }
    
    /// Load lock file
    pub fn load_lock_file(&self) -> Result<EnvTomlLock> {
        let content = fs::read_to_string(&self.lock_file_path)?;
        let lock: EnvTomlLock = toml::from_str(&content)?;
        Ok(lock)
    }
    
    /// Verify configuration against lock file
    pub fn verify_against_lock(&self, config: &EnvIniConfig, lock: &EnvTomlLock) -> Result<bool> {
        let current_hash = self.calculate_config_hash(config)?;
        Ok(current_hash == lock.config_hash)
    }
    
    /// Calculate configuration hash
    fn calculate_config_hash(&self, config: &EnvIniConfig) -> Result<String> {
        let serialized = serde_json::to_string(config)?;
        let hash = self.hash_value(&serialized);
        Ok(hash)
    }
    
    /// Hash a value using SHA256
    fn hash_value(&self, value: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(value.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    /// Export configuration for BSO-K8 deployment
    pub fn export_for_bso_k8(&self, config: &EnvIniConfig) -> Result<HashMap<String, String>> {
        let mut env_vars = HashMap::new();
        
        // Export all sections as environment variables
        for (section_name, section) in &config.sections {
            for (key, var) in &section.variables {
                let env_key = format!("{}_{}", section_name.to_uppercase(), key.to_uppercase());
                env_vars.insert(env_key, var.value.clone());
            }
        }
        
        // Export globals
        for (key, value) in &config.globals {
            env_vars.insert(key.to_uppercase(), value.clone());
        }
        
        Ok(env_vars)
    }
    
    /// Parse commute.lock configuration from sections
    pub fn parse_commute_lock_section(&self, config: &mut EnvIniConfig) -> Result<()> {
        if let Some(section) = config.sections.get("commute_lock") {
            let vars = &section.variables;
            
            // Parse basic settings
            let enabled = vars.get("enabled")
                .map(|v| v.value == "true")
                .unwrap_or(false);
            
            let communication_mode = vars.get("communication_mode")
                .map(|v| match v.value.as_str() {
                    "shared_memory" => CommunicationMode::SharedMemory,
                    "http" => CommunicationMode::Http,
                    "hybrid" => CommunicationMode::Hybrid,
                    _ => CommunicationMode::Hybrid,
                })
                .unwrap_or(CommunicationMode::Hybrid);
            
            let lock_dir = vars.get("lock_dir")
                .map(|v| PathBuf::from(&v.value))
                .unwrap_or_else(|| PathBuf::from("/var/lock/bpci"));
            
            let shm_dir = vars.get("shm_dir")
                .map(|v| PathBuf::from(&v.value))
                .unwrap_or_else(|| PathBuf::from("/dev/shm/bpci"));
            
            let event_dir = vars.get("event_dir")
                .map(|v| PathBuf::from(&v.value))
                .unwrap_or_else(|| PathBuf::from("/var/run/bpci"));
            
            // Parse component shared memory sizes
            let mut component_shm_sizes = HashMap::new();
            component_shm_sizes.insert("consensus".to_string(), 
                vars.get("consensus_shm_mb").and_then(|v| v.value.parse().ok()).unwrap_or(10));
            component_shm_sizes.insert("blockchain".to_string(), 
                vars.get("blockchain_shm_mb").and_then(|v| v.value.parse().ok()).unwrap_or(20));
            component_shm_sizes.insert("auction".to_string(), 
                vars.get("auction_shm_mb").and_then(|v| v.value.parse().ok()).unwrap_or(15));
            component_shm_sizes.insert("bso_k8".to_string(), 
                vars.get("bso_k8_shm_mb").and_then(|v| v.value.parse().ok()).unwrap_or(5));
            component_shm_sizes.insert("bridge".to_string(), 
                vars.get("bridge_shm_mb").and_then(|v| v.value.parse().ok()).unwrap_or(10));
            component_shm_sizes.insert("cluster_ledger".to_string(), 
                vars.get("cluster_ledger_shm_mb").and_then(|v| v.value.parse().ok()).unwrap_or(100));
            component_shm_sizes.insert("xtmp".to_string(), 
                vars.get("xtmp_shm_mb").and_then(|v| v.value.parse().ok()).unwrap_or(10));
            component_shm_sizes.insert("shadow_registry".to_string(), 
                vars.get("shadow_registry_shm_mb").and_then(|v| v.value.parse().ok()).unwrap_or(10));
            component_shm_sizes.insert("web".to_string(), 
                vars.get("web_shm_mb").and_then(|v| v.value.parse().ok()).unwrap_or(5));
            
            // Parse BPI data configuration
            let bpi_data_config = BpiDataConfig {
                bpi_data_dir: vars.get("bpi_data_dir")
                    .map(|v| PathBuf::from(&v.value))
                    .unwrap_or_else(|| PathBuf::from("/dev/shm/bpci/bpi_data")),
                per_address_mb: vars.get("bpi_data_per_address_mb")
                    .and_then(|v| v.value.parse().ok())
                    .unwrap_or(1),
                max_addresses: vars.get("max_bpi_addresses")
                    .and_then(|v| v.value.parse().ok())
                    .unwrap_or(1000000),
            };
            
            // Parse lock settings
            let lock_settings = LockSettings {
                timeout_ms: vars.get("lock_timeout_ms")
                    .and_then(|v| v.value.parse().ok())
                    .unwrap_or(1000),
                retry_count: vars.get("lock_retry_count")
                    .and_then(|v| v.value.parse().ok())
                    .unwrap_or(3),
                enable_monitoring: vars.get("enable_lock_monitoring")
                    .map(|v| v.value == "true")
                    .unwrap_or(true),
            };
            
            // Parse event settings
            let event_settings = EventSettings {
                buffer_size: vars.get("event_buffer_size")
                    .and_then(|v| v.value.parse().ok())
                    .unwrap_or(1024),
                timeout_ms: vars.get("event_timeout_ms")
                    .and_then(|v| v.value.parse().ok())
                    .unwrap_or(100),
            };
            
            // Parse performance settings
            let performance = PerformanceSettings {
                zero_copy_enabled: vars.get("zero_copy_enabled")
                    .map(|v| v.value == "true")
                    .unwrap_or(true),
                lock_free_queues: vars.get("lock_free_queues")
                    .map(|v| v.value == "true")
                    .unwrap_or(true),
                numa_aware: vars.get("numa_aware")
                    .map(|v| v.value == "true")
                    .unwrap_or(true),
            };
            
            config.commute_lock_config = Some(CommuteLockConfig {
                enabled,
                communication_mode,
                lock_dir,
                shm_dir,
                event_dir,
                component_shm_sizes,
                bpi_data_config,
                lock_settings,
                event_settings,
                performance,
            });
        }
        
        Ok(())
    }
    
    /// Export commute.lock configuration to envtoml.lock
    pub fn export_commute_lock_to_lock_file(
        &self,
        config: &EnvIniConfig,
        lock: &mut EnvTomlLock
    ) -> Result<()> {
        if let Some(commute_config) = &config.commute_lock_config {
            lock.commute_lock_snapshot = Some(CommuteLockSnapshot {
                enabled: commute_config.enabled,
                communication_mode: commute_config.communication_mode.clone(),
                component_shm_sizes: commute_config.component_shm_sizes.clone(),
                lock_dir: commute_config.lock_dir.to_string_lossy().to_string(),
                shm_dir: commute_config.shm_dir.to_string_lossy().to_string(),
                timestamp: Utc::now(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple_ini() {
        let content = r#"
[database]
host=localhost
port=5432

[api]
url=http://localhost:8080
"#;
        
        let parser = EnvIniParser::new(".");
        let config = parser.parse_ini_content(content).unwrap();
        
        assert_eq!(config.sections.len(), 2);
        assert!(config.sections.contains_key("database"));
        assert!(config.sections.contains_key("api"));
    }
    
    #[test]
    fn test_hash_value() {
        let parser = EnvIniParser::new(".");
        let hash1 = parser.hash_value("test");
        let hash2 = parser.hash_value("test");
        let hash3 = parser.hash_value("different");
        
        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
}
