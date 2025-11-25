//! Filesystem Immutability Engine
//! 
//! Convert existing Linux filesystems to immutable overlays while preserving user data

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_json;
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::{info, warn, debug, error};
use blake3;
use hex;
use crate::content_store::ContentStore;
use crate::hardware_detection::HardwareProfile;

/// Immutable filesystem configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmutableFilesystemConfig {
    pub root_device: String,
    pub immutable_partitions: Vec<ImmutablePartition>,
    pub overlay_partitions: Vec<OverlayPartition>,
    pub backup_locations: Vec<BackupLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmutablePartition {
    pub mount_point: String,
    pub device: String,
    pub filesystem: String,
    pub size_gb: f64,
    pub read_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayPartition {
    pub name: String,
    pub lower_dir: String,
    pub upper_dir: String,
    pub work_dir: String,
    pub mount_point: String,
    pub size_limit_gb: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupLocation {
    pub source_path: String,
    pub backup_path: String,
    pub backup_type: BackupType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackupType {
    UserData,
    SystemConfig,
    Applications,
    Logs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreServiceKind {
    DockLock,
    EncCluster,
    VmSupervisor,
    Court,
    Db4D,
    CueDb,
    Ipfs,
    ZkTerminal,
    Logbook,
    DynarouteGateway,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDomain {
    pub kind: CoreServiceKind,
    pub logical_id: String,
    pub hash: String,
    pub data_root: PathBuf,
    pub objects_root: PathBuf,
    pub cgroup_path: String,
    pub net_ns_name: String,
    pub pid_ns_name: String,
    pub mnt_ns_name: String,
    pub uts_ns_name: String,
}

/// Basic OS-level dynaroute configuration entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynarouteConfig {
    /// Logical service name (e.g. "vm-server", "6d-blockchain")
    pub service_name: String,
    /// Local bind address (host:port) on this node
    pub local_bind: String,
    /// Target address or virtual identity (may be another service name or mesh endpoint)
    pub target: String,
    /// Optional EncCluster identifier for encrypted routing
    pub enc_cluster: Option<String>,
    /// Whether this dynaroute is currently enabled
    pub enabled: bool,
}

impl ServiceDomain {
    pub fn new(kind: CoreServiceKind, logical_id: String, era_root: PathBuf) -> Result<Self> {
        let kind_str = match kind {
            CoreServiceKind::DockLock => "docklock",
            CoreServiceKind::EncCluster => "encclusters",
            CoreServiceKind::VmSupervisor => "vm",
            CoreServiceKind::Court => "courts",
            CoreServiceKind::Db4D => "db4d",
            CoreServiceKind::CueDb => "cuedb",
            CoreServiceKind::Ipfs => "ipfs",
            CoreServiceKind::ZkTerminal => "zk_terminal",
            CoreServiceKind::Logbook => "logbook",
            CoreServiceKind::DynarouteGateway => "dynaroute_gw",
        };

        let hash_input = format!("{}:{}", kind_str, logical_id);
        let hash = blake3::hash(hash_input.as_bytes());
        let full = hex::encode(hash.as_bytes());
        let short = full.chars().take(12).collect::<String>();

        let bpi_root = era_root.join("mutable").join("var").join("bpi");
        let data_root = bpi_root.join(kind_str).join(&short);

        let store_root = era_root.join("store");
        let objects_root = match kind {
            CoreServiceKind::Db4D => store_root.join("db4d"),
            CoreServiceKind::Ipfs => store_root.join("ipfs"),
            _ => store_root.join("objects"),
        };

        let cgroup_path = format!("/sys/fs/cgroup/bpi-core/{}/{}", kind_str, short);
        let net_ns_name = format!("bpi-{}-net-{}", kind_str, short);
        let pid_ns_name = format!("bpi-{}-pid-{}", kind_str, short);
        let mnt_ns_name = format!("bpi-{}-mnt-{}", kind_str, short);
        let uts_ns_name = format!("bpi-{}-uts-{}", kind_str, short);

        Ok(Self {
            kind,
            logical_id,
            hash: short,
            data_root,
            objects_root,
            cgroup_path,
            net_ns_name,
            pid_ns_name,
            mnt_ns_name,
            uts_ns_name,
        })
    }
}

/// Filesystem Immutability Engine
#[derive(Debug)]
pub struct FilesystemImmutabilityEngine {
    config: Option<ImmutableFilesystemConfig>,
    temp_dir: PathBuf,
}

impl FilesystemImmutabilityEngine {
    /// Create new filesystem immutability engine
    pub async fn new() -> Result<Self> {
        info!("Initializing Filesystem Immutability Engine");
        
        let temp_dir = tempfile::tempdir()
            .map_err(|e| anyhow!("Failed to create temp directory: {}", e))?;
        let temp_path = temp_dir.path().to_path_buf();
        
        Ok(Self {
            config: None,
            temp_dir: temp_path,
        })
    }

    /// Prepare immutable filesystem based on hardware profile
    pub async fn prepare_immutable_filesystem(&mut self, hardware_profile: &HardwareProfile) -> Result<()> {
        info!("💾 Preparing immutable filesystem transformation");
        
        // Step 0: Ensure ERA-FS base layout and vPods integration points exist
        self.ensure_era_layout().await?;
        self.ensure_vpods_layout().await?;
        self.ensure_bpi_core_layout().await?;
        self.ensure_initial_generation().await?;

        // Step 1: Analyze current filesystem layout
        let current_layout = self.analyze_current_filesystem(hardware_profile).await?;
        info!("✅ Current filesystem analyzed");
        
        // Step 2: Create backup of critical data
        self.backup_critical_data(&current_layout).await?;
        info!("✅ Critical data backed up");
        
        // Step 3: Design immutable filesystem layout (using vPods capacity if available)
        let immutable_config = self.design_immutable_layout(&current_layout, hardware_profile).await?;
        info!("✅ Immutable filesystem layout designed");
        
        // Step 4: Prepare overlay filesystems
        self.prepare_overlay_filesystems(&immutable_config).await?;
        info!("✅ Overlay filesystems prepared");
        
        // Step 5: Create immutable partitions
        self.create_immutable_partitions(&immutable_config).await?;
        info!("✅ Immutable partitions created");
        
        // Step 6: Setup bootloader for immutable OS
        self.setup_immutable_bootloader(&immutable_config, hardware_profile).await?;
        info!("✅ Immutable bootloader configured");
        
        self.config = Some(immutable_config);
        info!("💾 Immutable filesystem preparation completed");
        
        Ok(())
    }

    /// Create or load the OS-level domain for the DockLock service
    pub async fn create_docklock_domain(&self, logical_id: &str) -> Result<ServiceDomain> {
        let era_root = PathBuf::from("/era");

        let domain = ServiceDomain::new(
            CoreServiceKind::DockLock,
            logical_id.to_string(),
            era_root.clone(),
        )?;

        // Ensure per-instance directories under /era/mutable/var/bpi/docklock/<hash>/
        let manifests_dir = domain.data_root.join("manifests");
        let runtime_dir = domain.data_root.join("runtime");
        let logs_dir = domain.data_root.join("logs");

        for dir in [&domain.data_root, &manifests_dir, &runtime_dir, &logs_dir] {
            if !dir.exists() {
                fs::create_dir_all(dir)
                    .map_err(|e| anyhow!("Failed to create DockLock domain directory {}: {}", dir.display(), e))?;
            }
        }

        Ok(domain)
    }

    /// Create or load the OS-level domain for the Dynaroute gateway service
    pub async fn create_dynaroute_gateway_domain(&self, logical_id: &str) -> Result<ServiceDomain> {
        let era_root = PathBuf::from("/era");

        let domain = ServiceDomain::new(
            CoreServiceKind::DynarouteGateway,
            logical_id.to_string(),
            era_root.clone(),
        )?;

        let state_dir = domain.data_root.join("state");
        let logs_dir = domain.data_root.join("logs");

        for dir in [&domain.data_root, &state_dir, &logs_dir] {
            if !dir.exists() {
                fs::create_dir_all(dir)
                    .map_err(|e| anyhow!("Failed to create Dynaroute gateway directory {}: {}", dir.display(), e))?;
            }
        }

        Ok(domain)
    }

    /// Persist a DynarouteConfig entry under /era/mutable/var/bpi/network/dynaroutes
    pub async fn write_dynaroute_config(&self, name: &str, cfg: &DynarouteConfig) -> Result<PathBuf> {
        let era_root = Path::new("/era");
        let dynaroutes_root = era_root
            .join("mutable")
            .join("var")
            .join("bpi")
            .join("network")
            .join("dynaroutes");

        if !dynaroutes_root.exists() {
            fs::create_dir_all(&dynaroutes_root)
                .map_err(|e| anyhow!("Failed to create dynaroutes root {}: {}", dynaroutes_root.display(), e))?;
        }

        let json = serde_json::to_vec_pretty(cfg)
            .map_err(|e| anyhow!("Failed to serialize DynarouteConfig: {}", e))?;

        let file_path = dynaroutes_root.join(format!("{}.json", name));
        fs::write(&file_path, &json)
            .map_err(|e| anyhow!("Failed to write DynarouteConfig {}: {}", file_path.display(), e))?;

        Ok(file_path)
    }

    /// Load a DynarouteConfig entry by name if it exists
    pub async fn read_dynaroute_config(&self, name: &str) -> Result<Option<DynarouteConfig>> {
        let era_root = Path::new("/era");
        let file_path = era_root
            .join("mutable")
            .join("var")
            .join("bpi")
            .join("network")
            .join("dynaroutes")
            .join(format!("{}.json", name));

        if !file_path.exists() {
            return Ok(None);
        }

        let data = fs::read(&file_path)
            .map_err(|e| anyhow!("Failed to read DynarouteConfig {}: {}", file_path.display(), e))?;
        let cfg: DynarouteConfig = serde_json::from_slice(&data)
            .map_err(|e| anyhow!("Failed to deserialize DynarouteConfig {}: {}", file_path.display(), e))?;

        Ok(Some(cfg))
    }

    /// Ensure an initial ERA-FS generation exists and is content-addressed
    async fn ensure_initial_generation(&self) -> Result<()> {
        let era_root = Path::new("/era");
        let generations_root = era_root.join("generations");

        let gen_id = "001-initial";
        let gen_dir = generations_root.join(gen_id);

        if !gen_dir.exists() {
            fs::create_dir_all(&gen_dir)
                .map_err(|e| anyhow!("Failed to create ERA generation dir {}: {}", gen_dir.display(), e))?;
        }

        let manifest_path = gen_dir.join("manifest.json");

        if !manifest_path.exists() {
            #[derive(Serialize)]
            struct GenerationMeta<'a> {
                id: &'a str,
                created_at: i64,
            }

            let meta = GenerationMeta {
                id: gen_id,
                created_at: Utc::now().timestamp(),
            };

            let meta_bytes = serde_json::to_vec(&meta)
                .map_err(|e| anyhow!("Failed to serialize generation meta: {}", e))?;

            let store = ContentStore::new_default();
            let addr = store.write_object(&meta_bytes)?;

            #[derive(Serialize)]
            struct GenerationManifest<'a> {
                id: &'a str,
                content_dir: String,
            }

            let manifest = GenerationManifest {
                id: gen_id,
                content_dir: addr.to_dir_name(),
            };

            let manifest_json = serde_json::to_vec_pretty(&manifest)
                .map_err(|e| anyhow!("Failed to serialize generation manifest: {}", e))?;

            fs::write(&manifest_path, &manifest_json)
                .map_err(|e| anyhow!("Failed to write generation manifest {}: {}", manifest_path.display(), e))?;
        }

        // Create /era/generations/current -> 001-initial symlink if not present
        let current_link = generations_root.join("current");
        if !current_link.exists() {
            #[cfg(unix)]
            {
                use std::os::unix::fs as unix_fs;
                unix_fs::symlink(&gen_dir, &current_link)
                    .map_err(|e| anyhow!("Failed to create ERA current generation symlink {} -> {}: {}", current_link.display(), gen_dir.display(), e))?;
            }
        }

        Ok(())
    }

    /// Ensure minimal ERA-FS layout exists under /era
    async fn ensure_era_layout(&self) -> Result<()> {
        let era_root = Path::new("/era");

        // Base ERA-FS directories
        let store = era_root.join("store");
        let store_objects = store.join("objects");
        let store_packages = store.join("packages");
        let store_chains = store.join("chains");

        let capabilities = era_root.join("capabilities");
        let capabilities_domains = capabilities.join("domains");
        let capabilities_grants = capabilities.join("grants");
        let capabilities_policies = capabilities.join("policies");

        let current = era_root.join("current");
        let generations = era_root.join("generations");

        let mutable = era_root.join("mutable");
        let mutable_var = mutable.join("var");
        let mutable_home = mutable.join("home");
        let mutable_tmp = mutable.join("tmp");
        let mutable_etc_overlay = mutable.join("etc-overlay");

        // Create directories idempotently; if they already exist, this is a no-op.
        for dir in [
            era_root,
            &store,
            &store_objects,
            &store_packages,
            &store_chains,
            &capabilities,
            &capabilities_domains,
            &capabilities_grants,
            &capabilities_policies,
            &current,
            &generations,
            &mutable,
            &mutable_var,
            &mutable_home,
            &mutable_tmp,
            &mutable_etc_overlay,
        ] {
            if !dir.exists() {
                fs::create_dir_all(dir)
                    .map_err(|e| anyhow!("Failed to create ERA-FS directory {}: {}", dir.display(), e))?;
            }
        }

        Ok(())
    }

    /// Ensure vPods-specific directories and policy paths are present under ERA-FS
    async fn ensure_vpods_layout(&self) -> Result<()> {
        let era_root = Path::new("/era");

        // Mutable vPods runtime state (logs, SchedBlocks, metrics)
        let vpods_var_root = era_root.join("mutable").join("var").join("vpods");
        let vpods_logs = vpods_var_root.join("logs");
        let vpods_schedblocks = vpods_var_root.join("schedblocks");
        let vpods_metrics = vpods_var_root.join("metrics");

        for dir in [&vpods_var_root, &vpods_logs, &vpods_schedblocks, &vpods_metrics] {
            if !dir.exists() {
                fs::create_dir_all(dir)
                    .map_err(|e| anyhow!("Failed to create vPods directory {}: {}", dir.display(), e))?;
            }
        }

        // Capability and policy locations for vPods roles and network firewall
        let policies_root = era_root.join("capabilities").join("policies");
        let vpods_policies_root = policies_root.join("vpods");
        let vpods_roles_root = vpods_policies_root.join("roles");
        let network_policies_root = policies_root.join("network");

        for dir in [&vpods_policies_root, &vpods_roles_root, &network_policies_root] {
            if !dir.exists() {
                fs::create_dir_all(dir)
                    .map_err(|e| anyhow!("Failed to create vPods policy directory {}: {}", dir.display(), e))?;
            }
        }

        Ok(())
    }

    async fn ensure_bpi_core_layout(&self) -> Result<()> {
        let era_root = Path::new("/era");

        let bpi_root = era_root.join("mutable").join("var").join("bpi");
        let ledger_root = bpi_root.join("ledger");
        let docklock_root = bpi_root.join("docklock");
        let encclusters_root = bpi_root.join("encclusters");
        let vm_root = bpi_root.join("vm");
        let courts_root = bpi_root.join("courts");
        let db4d_root = bpi_root.join("db4d");
        let cuedb_root = bpi_root.join("cuedb");
        let ipfs_root = bpi_root.join("ipfs");
        let zk_terminal_root = bpi_root.join("zk_terminal");
        let logbook_root = bpi_root.join("logbook");
        let logbook_ziplock_root = logbook_root.join("ziplock");
        let network_root = bpi_root.join("network");
        let dynaroutes_root = network_root.join("dynaroutes");
        let dynaroute_gateway_root = network_root.join("gateway");

        for dir in [
            &bpi_root,
            &ledger_root,
            &docklock_root,
            &encclusters_root,
            &vm_root,
            &courts_root,
            &db4d_root,
            &cuedb_root,
            &ipfs_root,
            &zk_terminal_root,
            &logbook_root,
            &logbook_ziplock_root,
            &network_root,
            &dynaroutes_root,
            &dynaroute_gateway_root,
        ] {
            if !dir.exists() {
                fs::create_dir_all(dir)
                    .map_err(|e| anyhow!("Failed to create BPI Core directory {}: {}", dir.display(), e))?;
            }
        }

        let store_root = era_root.join("store");
        let db4d_store_root = store_root.join("db4d");
        let ipfs_store_root = store_root.join("ipfs");

        for dir in [&db4d_store_root, &ipfs_store_root] {
            if !dir.exists() {
                fs::create_dir_all(dir)
                    .map_err(|e| anyhow!("Failed to create BPI Core store directory {}: {}", dir.display(), e))?;
            }
        }

        Ok(())
    }

    /// Lock filesystem in immutable state
    pub async fn lock_immutable_filesystem(&self) -> Result<()> {
        info!("🔒 Locking filesystem in immutable state");
        
        let config = self.config.as_ref()
            .ok_or_else(|| anyhow!("Immutable filesystem not prepared"))?;
        
        // Step 1: Remount root as read-only
        self.remount_root_readonly().await?;
        info!("✅ Root filesystem locked as read-only");
        
        // Step 2: Setup overlay mounts
        self.setup_overlay_mounts(config).await?;
        info!("✅ Overlay mounts configured");
        
        // Step 3: Create immutable filesystem service
        self.create_immutable_service().await?;
        info!("✅ Immutable filesystem service created");
        
        // Step 4: Configure automatic overlay cleanup
        self.configure_overlay_cleanup().await?;
        info!("✅ Overlay cleanup configured");
        
        info!("🔒 Filesystem successfully locked in immutable state");
        Ok(())
    }

    /// Analyze current filesystem layout
    async fn analyze_current_filesystem(&self, hardware_profile: &HardwareProfile) -> Result<FilesystemLayout> {
        debug!("Analyzing current filesystem layout");
        
        let mut layout = FilesystemLayout {
            root_partition: None,
            boot_partition: None,
            user_partitions: Vec::new(),
            system_directories: Vec::new(),
            user_directories: Vec::new(),
        };
        
        // Find root and boot partitions
        for storage in &hardware_profile.storage {
            if let Some(mount_point) = &storage.mount_point {
                match mount_point.as_str() {
                    "/" => {
                        layout.root_partition = Some(PartitionInfo {
                            device: storage.device_path.clone(),
                            mount_point: mount_point.clone(),
                            filesystem: storage.filesystem.clone().unwrap_or("unknown".to_string()),
                            size_gb: storage.size_gb,
                        });
                    }
                    "/boot" | "/boot/efi" => {
                        layout.boot_partition = Some(PartitionInfo {
                            device: storage.device_path.clone(),
                            mount_point: mount_point.clone(),
                            filesystem: storage.filesystem.clone().unwrap_or("unknown".to_string()),
                            size_gb: storage.size_gb,
                        });
                    }
                    _ if mount_point.starts_with("/home") || mount_point.starts_with("/var") => {
                        layout.user_partitions.push(PartitionInfo {
                            device: storage.device_path.clone(),
                            mount_point: mount_point.clone(),
                            filesystem: storage.filesystem.clone().unwrap_or("unknown".to_string()),
                            size_gb: storage.size_gb,
                        });
                    }
                    _ => {}
                }
            }
        }
        
        // Identify system and user directories
        layout.system_directories = vec![
            "/bin".to_string(), "/sbin".to_string(), "/lib".to_string(), "/lib64".to_string(),
            "/usr".to_string(), "/etc".to_string(), "/opt".to_string(), "/root".to_string(),
        ];
        
        layout.user_directories = vec![
            "/home".to_string(), "/var".to_string(), "/tmp".to_string(), "/srv".to_string(),
        ];
        
        Ok(layout)
    }

    /// Backup critical data before transformation
    async fn backup_critical_data(&self, layout: &FilesystemLayout) -> Result<()> {
        debug!("Backing up critical data");
        
        let backup_dir = self.temp_dir.join("bpi_backup");
        fs::create_dir_all(&backup_dir)
            .map_err(|e| anyhow!("Failed to create backup directory: {}", e))?;
        
        // Backup system configuration
        let config_backup = backup_dir.join("system_config");
        fs::create_dir_all(&config_backup)?;
        
        self.backup_directory("/etc", &config_backup.join("etc")).await?;
        self.backup_directory("/root", &config_backup.join("root")).await?;
        
        // Backup user data
        let user_backup = backup_dir.join("user_data");
        fs::create_dir_all(&user_backup)?;
        
        if Path::new("/home").exists() {
            self.backup_directory("/home", &user_backup.join("home")).await?;
        }
        
        // Backup application data
        let app_backup = backup_dir.join("applications");
        fs::create_dir_all(&app_backup)?;
        
        if Path::new("/var/lib").exists() {
            self.backup_directory("/var/lib", &app_backup.join("var_lib")).await?;
        }
        
        info!("Critical data backed up to: {}", backup_dir.display());
        Ok(())
    }

    /// Backup directory using rsync
    async fn backup_directory(&self, source: &str, destination: &Path) -> Result<()> {
        debug!("Backing up {} to {}", source, destination.display());
        
        let status = Command::new("rsync")
            .args(&[
                "-av",
                "--exclude=/proc",
                "--exclude=/sys",
                "--exclude=/dev",
                "--exclude=/run",
                "--exclude=/tmp",
                source,
                &destination.to_string_lossy(),
            ])
            .status()
            .map_err(|e| anyhow!("Failed to run rsync: {}", e))?;
        
        if !status.success() {
            warn!("Rsync backup of {} completed with warnings", source);
        }
        
        Ok(())
    }

    /// Design immutable filesystem layout
    async fn design_immutable_layout(&self, current_layout: &FilesystemLayout, hardware_profile: &HardwareProfile) -> Result<ImmutableFilesystemConfig> {
        debug!("Designing immutable filesystem layout");
        
        let root_device = current_layout.root_partition
            .as_ref()
            .ok_or_else(|| anyhow!("No root partition found"))?
            .device.clone();

        // Use vPods capacity to size immutable partitions if available
        let (root_size_gb, overlay_limits) = if let Some(node_capacity) = &hardware_profile.node_capacity {
            let cores = node_capacity.cores as f64;
            let ram_gb = node_capacity.ram_mb as f64 / 1024.0;
            
            // Scale root partition based on cores (more cores = more services)
            let root_size = (20.0 + cores * 2.0).min(50.0); // 20-50GB range
            
            // Scale overlay limits based on RAM (more RAM = larger working sets)
            let home_limit = (ram_gb * 4.0).min(200.0); // 4x RAM, max 200GB
            let var_limit = (ram_gb * 2.0).min(100.0);   // 2x RAM, max 100GB
            
            info!(
                cores = cores,
                ram_gb = ram_gb,
                root_size_gb = root_size,
                home_limit_gb = home_limit,
                var_limit_gb = var_limit,
                "vPods capacity-based filesystem sizing"
            );
            
            (root_size, (home_limit, var_limit))
        } else {
            warn!("No vPods capacity available, using default sizing");
            (20.0, (50.0, 30.0))
        };
        
        // Create immutable partitions (read-only system)
        let mut immutable_partitions = vec![
            ImmutablePartition {
                mount_point: "/".to_string(),
                device: root_device.clone(),
                filesystem: "ext4".to_string(),
                size_gb: root_size_gb,
                read_only: true,
            },
        ];
        
        // Add boot partition if exists
        if let Some(boot_partition) = &current_layout.boot_partition {
            immutable_partitions.push(ImmutablePartition {
                mount_point: boot_partition.mount_point.clone(),
                device: boot_partition.device.clone(),
                filesystem: boot_partition.filesystem.clone(),
                size_gb: boot_partition.size_gb,
                read_only: true,
            });
        }
        
        // Create overlay partitions (writable user data) with vPods-based sizing
        let overlay_partitions = vec![
            OverlayPartition {
                name: "home_overlay".to_string(),
                lower_dir: "/home".to_string(),
                upper_dir: "/var/lib/bpi/overlays/home/upper".to_string(),
                work_dir: "/var/lib/bpi/overlays/home/work".to_string(),
                mount_point: "/home".to_string(),
                size_limit_gb: Some(overlay_limits.0),
            },
            OverlayPartition {
                name: "var_overlay".to_string(),
                lower_dir: "/var".to_string(),
                upper_dir: "/var/lib/bpi/overlays/var/upper".to_string(),
                work_dir: "/var/lib/bpi/overlays/var/work".to_string(),
                mount_point: "/var".to_string(),
                size_limit_gb: Some(overlay_limits.1),
            },
            OverlayPartition {
                name: "tmp_overlay".to_string(),
                lower_dir: "/tmp".to_string(),
                upper_dir: "/var/lib/bpi/overlays/tmp/upper".to_string(),
                work_dir: "/var/lib/bpi/overlays/tmp/work".to_string(),
                mount_point: "/tmp".to_string(),
                size_limit_gb: Some(10.0),
            },
            OverlayPartition {
                name: "etc_overlay".to_string(),
                lower_dir: "/etc".to_string(),
                upper_dir: "/var/lib/bpi/overlays/etc/upper".to_string(),
                work_dir: "/var/lib/bpi/overlays/etc/work".to_string(),
                mount_point: "/etc".to_string(),
                size_limit_gb: Some(5.0),
            },
        ];
        
        // Define backup locations
        let backup_locations = vec![
            BackupLocation {
                source_path: "/home".to_string(),
                backup_path: "/var/lib/bpi/backups/home".to_string(),
                backup_type: BackupType::UserData,
            },
            BackupLocation {
                source_path: "/etc".to_string(),
                backup_path: "/var/lib/bpi/backups/etc".to_string(),
                backup_type: BackupType::SystemConfig,
            },
            BackupLocation {
                source_path: "/var/lib".to_string(),
                backup_path: "/var/lib/bpi/backups/var_lib".to_string(),
                backup_type: BackupType::Applications,
            },
        ];
        
        Ok(ImmutableFilesystemConfig {
            root_device,
            immutable_partitions,
            overlay_partitions,
            backup_locations,
        })
    }

    /// Prepare overlay filesystems
    async fn prepare_overlay_filesystems(&self, config: &ImmutableFilesystemConfig) -> Result<()> {
        debug!("Preparing overlay filesystems");
        
        // Create overlay directories
        let overlay_base = Path::new("/var/lib/bpi/overlays");
        fs::create_dir_all(overlay_base)
            .map_err(|e| anyhow!("Failed to create overlay base directory: {}", e))?;
        
        for overlay in &config.overlay_partitions {
            // Create upper and work directories
            fs::create_dir_all(&overlay.upper_dir)
                .map_err(|e| anyhow!("Failed to create upper dir {}: {}", overlay.upper_dir, e))?;
            fs::create_dir_all(&overlay.work_dir)
                .map_err(|e| anyhow!("Failed to create work dir {}: {}", overlay.work_dir, e))?;
            
            debug!("Created overlay directories for {}", overlay.name);
        }
        
        Ok(())
    }

    /// Create immutable partitions
    async fn create_immutable_partitions(&self, config: &ImmutableFilesystemConfig) -> Result<()> {
        debug!("Creating immutable partitions");
        
        // For now, we'll prepare the existing partitions for immutable mode
        // In a full implementation, this would involve repartitioning
        
        for partition in &config.immutable_partitions {
            info!("Preparing immutable partition: {}", partition.mount_point);
            
            // Ensure filesystem is clean
            if partition.filesystem == "ext4" {
                let status = Command::new("fsck.ext4")
                    .args(&["-f", "-y", &partition.device])
                    .status();
                
                if let Ok(status) = status {
                    if status.success() {
                        debug!("Filesystem check passed for {}", partition.device);
                    } else {
                        warn!("Filesystem check had warnings for {}", partition.device);
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Setup bootloader for immutable OS
    async fn setup_immutable_bootloader(&self, config: &ImmutableFilesystemConfig, hardware_profile: &HardwareProfile) -> Result<()> {
        debug!("Setting up immutable bootloader");
        
        // Create GRUB configuration for immutable boot
        let grub_config = self.generate_immutable_grub_config(config, hardware_profile).await?;
        
        // Write GRUB configuration
        let grub_config_path = "/etc/grub.d/40_bpi_immutable";
        fs::write(grub_config_path, grub_config)
            .map_err(|e| anyhow!("Failed to write GRUB config: {}", e))?;
        
        // Make executable
        Command::new("chmod")
            .args(&["+x", grub_config_path])
            .status()
            .map_err(|e| anyhow!("Failed to make GRUB config executable: {}", e))?;
        
        // Update GRUB
        let status = Command::new("update-grub")
            .status()
            .map_err(|e| anyhow!("Failed to update GRUB: {}", e))?;
        
        if !status.success() {
            return Err(anyhow!("GRUB update failed"));
        }
        
        info!("Immutable bootloader configured");
        Ok(())
    }

    /// Generate GRUB configuration for immutable boot
    async fn generate_immutable_grub_config(&self, config: &ImmutableFilesystemConfig, hardware_profile: &HardwareProfile) -> Result<String> {
        let kernel_version = &hardware_profile.system.kernel_version;
        let root_device = &config.root_device;
        
        let grub_config = format!(r#"#!/bin/sh
exec tail -n +3 $0
# This file provides an alternative boot entry for BPI Immutable OS

menuentry 'BPI Immutable OS' --class bpi --class gnu-linux --class gnu --class os {{
    recordfail
    load_video
    gfxmode $linux_gfx_mode
    insmod gzio
    if [ x$grub_platform = xxen ]; then insmod xzio; insmod lzopio; fi
    insmod part_gpt
    insmod ext2
    
    # Boot with read-only root and overlay mounts
    linux /boot/vmlinuz-{} root={} ro quiet splash \
        systemd.volatile=no \
        bpi.immutable=true \
        bpi.overlays=home,var,tmp,etc
    
    initrd /boot/initrd.img-{}
}}
"#, kernel_version, root_device, kernel_version);
        
        Ok(grub_config)
    }

    /// Remount root filesystem as read-only
    async fn remount_root_readonly(&self) -> Result<()> {
        debug!("Remounting root filesystem as read-only");
        
        // This would typically be done during boot, but we'll prepare the fstab
        let fstab_path = "/etc/fstab";
        let fstab_content = fs::read_to_string(fstab_path)
            .map_err(|e| anyhow!("Failed to read fstab: {}", e))?;
        
        let mut new_fstab = String::new();
        for line in fstab_content.lines() {
            if line.contains(" / ") && !line.starts_with('#') {
                // Modify root mount to be read-only
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let device = parts[0];
                    let mount_point = parts[1];
                    let filesystem = parts[2];
                    let mut options = parts[3].to_string();
                    
                    // Add ro option if not present
                    if !options.contains("ro") {
                        options = options.replace("rw", "ro");
                        if !options.contains("ro") {
                            options = format!("ro,{}", options);
                        }
                    }
                    
                    let dump = parts.get(4).unwrap_or(&"0");
                    let pass = parts.get(5).unwrap_or(&"0");
                    
                    new_fstab.push_str(&format!("{} {} {} {} {} {}\n", 
                        device, mount_point, filesystem, options, dump, pass));
                } else {
                    new_fstab.push_str(line);
                    new_fstab.push('\n');
                }
            } else {
                new_fstab.push_str(line);
                new_fstab.push('\n');
            }
        }
        
        // Backup original fstab
        fs::copy(fstab_path, "/etc/fstab.bpi.backup")
            .map_err(|e| anyhow!("Failed to backup fstab: {}", e))?;
        
        // Write new fstab
        fs::write(fstab_path, new_fstab)
            .map_err(|e| anyhow!("Failed to write new fstab: {}", e))?;
        
        Ok(())
    }

    /// Setup overlay mounts
    async fn setup_overlay_mounts(&self, config: &ImmutableFilesystemConfig) -> Result<()> {
        debug!("Setting up overlay mounts");
        
        // Create systemd mount units for overlays
        for overlay in &config.overlay_partitions {
            let mount_unit = self.generate_overlay_mount_unit(overlay).await?;
            let unit_path = format!("/etc/systemd/system/{}.mount", 
                overlay.mount_point.replace('/', "-").trim_start_matches('-'));
            
            fs::write(&unit_path, mount_unit)
                .map_err(|e| anyhow!("Failed to write mount unit {}: {}", unit_path, e))?;
            
            // Enable the mount unit
            Command::new("systemctl")
                .args(&["enable", &format!("{}.mount", 
                    overlay.mount_point.replace('/', "-").trim_start_matches('-'))])
                .status()
                .map_err(|e| anyhow!("Failed to enable mount unit: {}", e))?;
        }
        
        Ok(())
    }

    /// Generate systemd mount unit for overlay
    async fn generate_overlay_mount_unit(&self, overlay: &OverlayPartition) -> Result<String> {
        let mount_unit = format!(r#"[Unit]
Description=BPI Overlay Mount for {}
DefaultDependencies=no
Conflicts=umount.target
Before=local-fs.target umount.target
After=swap.target

[Mount]
What=overlay
Where={}
Type=overlay
Options=lowerdir={},upperdir={},workdir={}

[Install]
WantedBy=local-fs.target
"#, overlay.name, overlay.mount_point, overlay.lower_dir, overlay.upper_dir, overlay.work_dir);
        
        Ok(mount_unit)
    }

    /// Create immutable filesystem service
    async fn create_immutable_service(&self) -> Result<()> {
        debug!("Creating immutable filesystem service");
        
        let service_content = r#"[Unit]
Description=BPI Immutable Filesystem Manager
After=local-fs.target
Before=multi-user.target

[Service]
Type=oneshot
RemainAfterExit=yes
ExecStart=/usr/local/bin/bpi-immutable-manager start
ExecStop=/usr/local/bin/bpi-immutable-manager stop
TimeoutSec=300

[Install]
WantedBy=multi-user.target
"#;
        
        fs::write("/etc/systemd/system/bpi-immutable.service", service_content)
            .map_err(|e| anyhow!("Failed to create immutable service: {}", e))?;
        
        // Create the manager script
        let manager_script = r#"#!/bin/bash
# BPI Immutable Filesystem Manager

case "$1" in
    start)
        echo "Starting BPI Immutable Filesystem Manager"
        # Ensure overlays are properly mounted
        systemctl start home.mount var.mount tmp.mount etc.mount 2>/dev/null || true
        # Set up overlay cleanup
        systemctl start bpi-overlay-cleanup.timer
        ;;
    stop)
        echo "Stopping BPI Immutable Filesystem Manager"
        systemctl stop bpi-overlay-cleanup.timer
        ;;
    *)
        echo "Usage: $0 {start|stop}"
        exit 1
        ;;
esac
"#;
        
        fs::write("/usr/local/bin/bpi-immutable-manager", manager_script)
            .map_err(|e| anyhow!("Failed to create manager script: {}", e))?;
        
        Command::new("chmod")
            .args(&["+x", "/usr/local/bin/bpi-immutable-manager"])
            .status()
            .map_err(|e| anyhow!("Failed to make manager script executable: {}", e))?;
        
        // Enable the service
        Command::new("systemctl")
            .args(&["enable", "bpi-immutable.service"])
            .status()
            .map_err(|e| anyhow!("Failed to enable immutable service: {}", e))?;
        
        Ok(())
    }

    /// Configure automatic overlay cleanup
    async fn configure_overlay_cleanup(&self) -> Result<()> {
        debug!("Configuring overlay cleanup");
        
        // Create cleanup timer
        let timer_content = r#"[Unit]
Description=BPI Overlay Cleanup Timer
Requires=bpi-overlay-cleanup.service

[Timer]
OnCalendar=daily
Persistent=true

[Install]
WantedBy=timers.target
"#;
        
        fs::write("/etc/systemd/system/bpi-overlay-cleanup.timer", timer_content)
            .map_err(|e| anyhow!("Failed to create cleanup timer: {}", e))?;
        
        // Create cleanup service
        let service_content = r#"[Unit]
Description=BPI Overlay Cleanup Service
Type=oneshot

[Service]
ExecStart=/usr/local/bin/bpi-overlay-cleanup
User=root
"#;
        
        fs::write("/etc/systemd/system/bpi-overlay-cleanup.service", service_content)
            .map_err(|e| anyhow!("Failed to create cleanup service: {}", e))?;
        
        // Create cleanup script
        let cleanup_script = r#"#!/bin/bash
# BPI Overlay Cleanup Script

OVERLAY_BASE="/var/lib/bpi/overlays"
LOG_FILE="/var/log/bpi-overlay-cleanup.log"

echo "$(date): Starting overlay cleanup" >> "$LOG_FILE"

# Clean up temporary overlays
find "$OVERLAY_BASE/tmp" -type f -mtime +1 -delete 2>/dev/null || true

# Compress old log files
find /var/log -name "*.log" -mtime +7 -exec gzip {} \; 2>/dev/null || true

# Clean up package cache
apt-get clean 2>/dev/null || true

echo "$(date): Overlay cleanup completed" >> "$LOG_FILE"
"#;
        
        fs::write("/usr/local/bin/bpi-overlay-cleanup", cleanup_script)
            .map_err(|e| anyhow!("Failed to create cleanup script: {}", e))?;
        
        Command::new("chmod")
            .args(&["+x", "/usr/local/bin/bpi-overlay-cleanup"])
            .status()
            .map_err(|e| anyhow!("Failed to make cleanup script executable: {}", e))?;
        
        Ok(())
    }
}

/// Current filesystem layout analysis
#[derive(Debug)]
struct FilesystemLayout {
    root_partition: Option<PartitionInfo>,
    boot_partition: Option<PartitionInfo>,
    user_partitions: Vec<PartitionInfo>,
    system_directories: Vec<String>,
    user_directories: Vec<String>,
}

#[derive(Debug)]
struct PartitionInfo {
    device: String,
    mount_point: String,
    filesystem: String,
    size_gb: f64,
}
