use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use anyhow::{Result, anyhow};

/// CUE.Installer - Comprehensive BPI OS Installation System
/// 
/// This system explains the installation process before installing BPI OS,
/// detects Linux distributions, handles Ubuntu as a special case,
/// checks prerequisites, and provides minimum/default installation options.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CueInstaller {
    pub system_info: SystemInfo,
    pub installation_config: InstallationConfig,
    pub prerequisites: Prerequisites,
    pub distro_handler: DistroHandler,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub architecture: String,
    pub cpu_cores: u32,
    pub total_memory_gb: f64,
    pub available_disk_gb: f64,
    pub is_ubuntu: bool,
    pub distro_family: DistroFamily,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistroFamily {
    Ubuntu,
    Debian,
    RedHat,
    Fedora,
    Arch,
    SUSE,
    Alpine,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationConfig {
    pub installation_type: InstallationType,
    pub target_directory: String,
    pub enable_services: Vec<String>,
    pub security_level: SecurityLevel,
    pub network_config: NetworkConfig,
    pub storage_config: StorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstallationType {
    Minimum,        // Minimal BPI OS installation
    Default,        // Standard installation with Ubuntu pre-installed support
    Full,           // Complete enterprise installation
    Custom(Vec<String>), // Custom component selection
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Basic,
    Standard,
    Military,       // Default for BPI OS
    Government,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub enable_enc_cluster: bool,
    pub enable_bpi_relay: bool,
    pub enable_shadow_registry: bool,
    pub port_ranges: HashMap<String, (u16, u16)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub enable_distributed_storage: bool,
    pub storage_quota_gb: u64,
    pub backup_enabled: bool,
    pub encryption_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prerequisites {
    pub required_packages: Vec<String>,
    pub optional_packages: Vec<String>,
    pub system_requirements: SystemRequirements,
    pub compatibility_checks: Vec<CompatibilityCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemRequirements {
    pub min_cpu_cores: u32,
    pub min_memory_gb: f64,
    pub min_disk_gb: f64,
    pub required_kernel_version: String,
    pub supported_architectures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityCheck {
    pub name: String,
    pub description: String,
    pub check_command: String,
    pub required: bool,
    pub status: CheckStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheckStatus {
    NotChecked,
    Passed,
    Failed(String),
    Warning(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistroHandler {
    pub detected_distro: DistroFamily,
    pub package_manager: PackageManager,
    pub service_manager: ServiceManager,
    pub distro_specific_config: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageManager {
    Apt,      // Ubuntu/Debian
    Yum,      // RedHat/CentOS
    Dnf,      // Fedora
    Pacman,   // Arch
    Zypper,   // SUSE
    Apk,      // Alpine
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceManager {
    Systemd,
    OpenRC,
    SysVInit,
}

impl CueInstaller {
    /// Create new CUE installer with system detection
    pub fn new() -> Result<Self> {
        let system_info = Self::detect_system_info()?;
        let distro_handler = Self::create_distro_handler(&system_info)?;
        let prerequisites = Self::create_prerequisites(&system_info.distro_family);
        let installation_config = Self::create_default_config(&system_info);

        Ok(Self {
            system_info,
            installation_config,
            prerequisites,
            distro_handler,
        })
    }

    /// Explain installation process before starting
    pub fn explain_installation(&self) -> Result<()> {
        println!("🚀 BPI OS Installation System - CUE.Installer v1.0");
        println!("================================================================================");
        println!();
        
        // System Information
        println!("📋 DETECTED SYSTEM INFORMATION:");
        println!("   OS: {} {}", self.system_info.os_name, self.system_info.os_version);
        println!("   Kernel: {}", self.system_info.kernel_version);
        println!("   Architecture: {}", self.system_info.architecture);
        println!("   CPU Cores: {}", self.system_info.cpu_cores);
        println!("   Memory: {:.1} GB", self.system_info.total_memory_gb);
        println!("   Available Disk: {:.1} GB", self.system_info.available_disk_gb);
        println!("   Distribution Family: {:?}", self.system_info.distro_family);
        
        if self.system_info.is_ubuntu {
            println!("   ✅ Ubuntu detected - Pre-installed support enabled");
        }
        println!();

        // Installation Types
        println!("🎯 INSTALLATION TYPE OPTIONS:");
        println!("   1. MINIMUM    - Essential BPI OS components only (~100MB)");
        println!("   2. DEFAULT    - Standard installation with Ubuntu support (~500MB)");
        println!("   3. FULL       - Complete enterprise installation (~1.5GB)");
        println!("   4. CUSTOM     - Select specific components");
        println!();

        // Prerequisites
        println!("📦 PREREQUISITES CHECK:");
        self.explain_prerequisites()?;
        println!();

        // Security Information
        println!("🔒 SECURITY CONFIGURATION:");
        println!("   Default Security Level: Military-Grade");
        println!("   - Post-quantum cryptography");
        println!("   - Hardware security integration");
        println!("   - Immutable OS architecture");
        println!("   - Zero-trust networking");
        println!();

        // Network Services
        println!("🌐 NETWORK SERVICES:");
        println!("   - ENC Cluster: Encrypted network orchestration");
        println!("   - BPI Relay: High-performance blockchain relay");
        println!("   - Shadow Registry: Web2-to-Web3 bridge");
        println!("   - DockLock: Container orchestration platform");
        println!();

        // Installation Process
        println!("⚙️  INSTALLATION PROCESS:");
        println!("   1. System compatibility check");
        println!("   2. Prerequisites installation");
        println!("   3. BPI OS core installation");
        println!("   4. Service configuration");
        println!("   5. Security hardening");
        println!("   6. Network setup");
        println!("   7. Final validation");
        println!();

        Ok(())
    }

    /// Detect comprehensive system information
    fn detect_system_info() -> Result<SystemInfo> {
        let os_release = Self::read_os_release()?;
        let kernel_version = Self::get_kernel_version()?;
        let architecture = Self::get_architecture()?;
        let (cpu_cores, total_memory_gb) = Self::get_hardware_info()?;
        let available_disk_gb = Self::get_available_disk_space()?;
        
        let os_name = os_release.get("NAME").unwrap_or(&"Unknown".to_string()).clone();
        let os_version = os_release.get("VERSION").unwrap_or(&"Unknown".to_string()).clone();
        
        let is_ubuntu = os_name.to_lowercase().contains("ubuntu");
        let distro_family = Self::detect_distro_family(&os_name)?;

        Ok(SystemInfo {
            os_name,
            os_version,
            kernel_version,
            architecture,
            cpu_cores,
            total_memory_gb,
            available_disk_gb,
            is_ubuntu,
            distro_family,
        })
    }

    /// Read /etc/os-release for distribution information
    fn read_os_release() -> Result<HashMap<String, String>> {
        let mut os_info = HashMap::new();
        
        if Path::new("/etc/os-release").exists() {
            let content = fs::read_to_string("/etc/os-release")?;
            for line in content.lines() {
                if let Some((key, value)) = line.split_once('=') {
                    let clean_value = value.trim_matches('"');
                    os_info.insert(key.to_string(), clean_value.to_string());
                }
            }
        } else {
            // Fallback detection methods
            os_info.insert("NAME".to_string(), "Linux".to_string());
            os_info.insert("VERSION".to_string(), "Unknown".to_string());
        }
        
        Ok(os_info)
    }

    /// Get kernel version
    fn get_kernel_version() -> Result<String> {
        let output = Command::new("uname").arg("-r").output()?;
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    /// Get system architecture
    fn get_architecture() -> Result<String> {
        let output = Command::new("uname").arg("-m").output()?;
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    /// Get hardware information
    fn get_hardware_info() -> Result<(u32, f64)> {
        // CPU cores
        let cpu_cores = match fs::read_to_string("/proc/cpuinfo") {
            Ok(content) => content.lines()
                .filter(|line| line.starts_with("processor"))
                .count() as u32,
            Err(_) => 1, // Fallback
        };

        // Memory in GB
        let total_memory_gb = match fs::read_to_string("/proc/meminfo") {
            Ok(content) => {
                for line in content.lines() {
                    if line.starts_with("MemTotal:") {
                        if let Some(kb_str) = line.split_whitespace().nth(1) {
                            if let Ok(kb) = kb_str.parse::<u64>() {
                                return Ok((cpu_cores, kb as f64 / 1024.0 / 1024.0));
                            }
                        }
                    }
                }
                1.0 // Fallback
            },
            Err(_) => 1.0, // Fallback
        };

        Ok((cpu_cores, total_memory_gb))
    }

    /// Get available disk space
    fn get_available_disk_space() -> Result<f64> {
        let output = Command::new("df")
            .args(&["-BG", "/"])
            .output()?;
        
        let output_str = String::from_utf8(output.stdout)?;
        for line in output_str.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                if let Ok(available_gb) = parts[3].trim_end_matches('G').parse::<f64>() {
                    return Ok(available_gb);
                }
            }
        }
        
        Ok(10.0) // Fallback minimum
    }

    /// Detect distribution family
    fn detect_distro_family(os_name: &str) -> Result<DistroFamily> {
        let name_lower = os_name.to_lowercase();
        
        if name_lower.contains("ubuntu") {
            Ok(DistroFamily::Ubuntu)
        } else if name_lower.contains("debian") {
            Ok(DistroFamily::Debian)
        } else if name_lower.contains("red hat") || name_lower.contains("rhel") || name_lower.contains("centos") {
            Ok(DistroFamily::RedHat)
        } else if name_lower.contains("fedora") {
            Ok(DistroFamily::Fedora)
        } else if name_lower.contains("arch") {
            Ok(DistroFamily::Arch)
        } else if name_lower.contains("suse") || name_lower.contains("opensuse") {
            Ok(DistroFamily::SUSE)
        } else if name_lower.contains("alpine") {
            Ok(DistroFamily::Alpine)
        } else {
            Ok(DistroFamily::Unknown)
        }
    }

    /// Create distribution-specific handler
    fn create_distro_handler(system_info: &SystemInfo) -> Result<DistroHandler> {
        let package_manager = match system_info.distro_family {
            DistroFamily::Ubuntu | DistroFamily::Debian => PackageManager::Apt,
            DistroFamily::RedHat => PackageManager::Yum,
            DistroFamily::Fedora => PackageManager::Dnf,
            DistroFamily::Arch => PackageManager::Pacman,
            DistroFamily::SUSE => PackageManager::Zypper,
            DistroFamily::Alpine => PackageManager::Apk,
            DistroFamily::Unknown => PackageManager::Apt, // Default fallback
        };

        let service_manager = if Path::new("/bin/systemctl").exists() {
            ServiceManager::Systemd
        } else if Path::new("/sbin/rc-service").exists() {
            ServiceManager::OpenRC
        } else {
            ServiceManager::SysVInit
        };

        let mut distro_specific_config = HashMap::new();
        
        // Ubuntu-specific configuration
        if system_info.is_ubuntu {
            distro_specific_config.insert("pre_installed_support".to_string(), "enabled".to_string());
            distro_specific_config.insert("snap_support".to_string(), "enabled".to_string());
            distro_specific_config.insert("apt_repositories".to_string(), "universe,multiverse".to_string());
        }

        Ok(DistroHandler {
            detected_distro: system_info.distro_family.clone(),
            package_manager,
            service_manager,
            distro_specific_config,
        })
    }

    /// Create prerequisites based on distribution
    fn create_prerequisites(distro_family: &DistroFamily) -> Prerequisites {
        let mut required_packages = vec![
            "curl".to_string(),
            "wget".to_string(),
            "git".to_string(),
            "build-essential".to_string(), // Will be adjusted per distro
        ];

        let mut optional_packages = vec![
            "htop".to_string(),
            "vim".to_string(),
            "tmux".to_string(),
        ];

        // Adjust packages based on distribution
        match distro_family {
            DistroFamily::Ubuntu | DistroFamily::Debian => {
                required_packages.push("apt-transport-https".to_string());
                required_packages.push("ca-certificates".to_string());
            },
            DistroFamily::RedHat | DistroFamily::Fedora => {
                // Replace build-essential with equivalent
                if let Some(pos) = required_packages.iter().position(|x| x == "build-essential") {
                    required_packages[pos] = "gcc".to_string();
                    required_packages.push("gcc-c++".to_string());
                    required_packages.push("make".to_string());
                }
            },
            DistroFamily::Arch => {
                if let Some(pos) = required_packages.iter().position(|x| x == "build-essential") {
                    required_packages[pos] = "base-devel".to_string();
                }
            },
            _ => {}, // Keep defaults for other distros
        }

        let system_requirements = SystemRequirements {
            min_cpu_cores: 2,
            min_memory_gb: 4.0,
            min_disk_gb: 10.0,
            required_kernel_version: "4.15".to_string(),
            supported_architectures: vec![
                "x86_64".to_string(),
                "aarch64".to_string(),
            ],
        };

        let compatibility_checks = vec![
            CompatibilityCheck {
                name: "Kernel Version".to_string(),
                description: "Check minimum kernel version".to_string(),
                check_command: "uname -r".to_string(),
                required: true,
                status: CheckStatus::NotChecked,
            },
            CompatibilityCheck {
                name: "Container Support".to_string(),
                description: "Check for container runtime support".to_string(),
                check_command: "ls /proc/sys/kernel/ns_last_pid".to_string(),
                required: true,
                status: CheckStatus::NotChecked,
            },
            CompatibilityCheck {
                name: "Network Namespaces".to_string(),
                description: "Check network namespace support".to_string(),
                check_command: "ip netns list".to_string(),
                required: false,
                status: CheckStatus::NotChecked,
            },
        ];

        Prerequisites {
            required_packages,
            optional_packages,
            system_requirements,
            compatibility_checks,
        }
    }

    /// Create default installation configuration
    fn create_default_config(system_info: &SystemInfo) -> InstallationConfig {
        let installation_type = if system_info.is_ubuntu {
            InstallationType::Default // Ubuntu gets default installation
        } else {
            InstallationType::Minimum // Other distros get minimum by default
        };

        let enable_services = vec![
            "bpi-core".to_string(),
            "bpi-relay".to_string(),
            "docklock".to_string(),
        ];

        let network_config = NetworkConfig {
            enable_enc_cluster: true,
            enable_bpi_relay: true,
            enable_shadow_registry: system_info.is_ubuntu, // Only enable for Ubuntu by default
            port_ranges: {
                let mut ports = HashMap::new();
                ports.insert("bpi-core".to_string(), (7777, 7777));
                ports.insert("bpi-relay".to_string(), (8080, 8080));
                ports.insert("enc-cluster".to_string(), (9000, 9010));
                ports
            },
        };

        let storage_config = StorageConfig {
            enable_distributed_storage: true,
            storage_quota_gb: (system_info.available_disk_gb * 0.5) as u64, // Use 50% of available space
            backup_enabled: true,
            encryption_enabled: true,
        };

        InstallationConfig {
            installation_type,
            target_directory: "/opt/bpi".to_string(),
            enable_services,
            security_level: SecurityLevel::Military,
            network_config,
            storage_config,
        }
    }

    /// Explain prerequisites to user
    fn explain_prerequisites(&self) -> Result<()> {
        println!("   Required Packages:");
        for package in &self.prerequisites.required_packages {
            println!("     - {}", package);
        }
        
        println!("   System Requirements:");
        println!("     - CPU Cores: {} (detected: {})", 
                 self.prerequisites.system_requirements.min_cpu_cores,
                 self.system_info.cpu_cores);
        println!("     - Memory: {:.1} GB (detected: {:.1} GB)", 
                 self.prerequisites.system_requirements.min_memory_gb,
                 self.system_info.total_memory_gb);
        println!("     - Disk Space: {:.1} GB (available: {:.1} GB)", 
                 self.prerequisites.system_requirements.min_disk_gb,
                 self.system_info.available_disk_gb);
        
        Ok(())
    }

    /// Run comprehensive compatibility checks
    pub fn run_compatibility_checks(&mut self) -> Result<bool> {
        println!("🔍 Running Compatibility Checks...");
        let mut all_passed = true;

        for check in &mut self.prerequisites.compatibility_checks {
            print!("   Checking {}: ", check.name);
            
            let result = Command::new("sh")
                .arg("-c")
                .arg(&check.check_command)
                .output();

            match result {
                Ok(output) if output.status.success() => {
                    check.status = CheckStatus::Passed;
                    println!("✅ PASSED");
                },
                Ok(_) => {
                    let error_msg = "Command failed".to_string();
                    check.status = CheckStatus::Failed(error_msg.clone());
                    if check.required {
                        println!("❌ FAILED (Required)");
                        all_passed = false;
                    } else {
                        println!("⚠️  WARNING (Optional)");
                    }
                },
                Err(e) => {
                    let error_msg = format!("Execution error: {}", e);
                    check.status = CheckStatus::Failed(error_msg);
                    if check.required {
                        println!("❌ FAILED (Required)");
                        all_passed = false;
                    } else {
                        println!("⚠️  WARNING (Optional)");
                    }
                }
            }
        }

        println!();
        if all_passed {
            println!("✅ All required compatibility checks passed!");
        } else {
            println!("❌ Some required compatibility checks failed. Installation cannot proceed.");
        }

        Ok(all_passed)
    }

    /// Install prerequisites based on detected package manager
    pub fn install_prerequisites(&self) -> Result<()> {
        println!("📦 Installing Prerequisites...");
        
        let install_cmd = match self.distro_handler.package_manager {
            PackageManager::Apt => {
                format!("apt update && apt install -y {}", 
                       self.prerequisites.required_packages.join(" "))
            },
            PackageManager::Yum => {
                format!("yum install -y {}", 
                       self.prerequisites.required_packages.join(" "))
            },
            PackageManager::Dnf => {
                format!("dnf install -y {}", 
                       self.prerequisites.required_packages.join(" "))
            },
            PackageManager::Pacman => {
                format!("pacman -Sy --noconfirm {}", 
                       self.prerequisites.required_packages.join(" "))
            },
            PackageManager::Zypper => {
                format!("zypper install -y {}", 
                       self.prerequisites.required_packages.join(" "))
            },
            PackageManager::Apk => {
                format!("apk add {}", 
                       self.prerequisites.required_packages.join(" "))
            },
        };

        println!("   Executing: {}", install_cmd);
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(&install_cmd)
            .output()?;

        if output.status.success() {
            println!("✅ Prerequisites installed successfully!");
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Failed to install prerequisites: {}", error));
        }

        Ok(())
    }

    /// Get installation summary
    pub fn get_installation_summary(&self) -> String {
        format!(
            "BPI OS Installation Summary:\n\
             - Distribution: {} ({})\n\
             - Installation Type: {:?}\n\
             - Target Directory: {}\n\
             - Security Level: {:?}\n\
             - Services: {}\n\
             - Storage Quota: {} GB\n\
             - Ubuntu Pre-installed Support: {}",
            self.system_info.os_name,
            self.system_info.os_version,
            self.installation_config.installation_type,
            self.installation_config.target_directory,
            self.installation_config.security_level,
            self.installation_config.enable_services.join(", "),
            self.installation_config.storage_config.storage_quota_gb,
            if self.system_info.is_ubuntu { "Enabled" } else { "Disabled" }
        )
    }
}

/// Interactive installation prompt system
pub struct InstallationPrompt;

impl InstallationPrompt {
    /// Prompt user for installation type
    pub fn prompt_installation_type() -> Result<InstallationType> {
        println!("Please select installation type:");
        println!("1. Minimum    - Essential components only (~100MB)");
        println!("2. Default    - Standard installation (~500MB)");
        println!("3. Full       - Complete enterprise installation (~1.5GB)");
        println!("4. Custom     - Select specific components");
        print!("Enter choice (1-4): ");
        
        // In a real implementation, this would read from stdin
        // For now, return Default as requested by user
        Ok(InstallationType::Default)
    }

    /// Confirm installation with user
    pub fn confirm_installation(installer: &CueInstaller) -> Result<bool> {
        println!("\n📋 INSTALLATION CONFIRMATION:");
        println!("{}", installer.get_installation_summary());
        println!("\nProceed with installation? (y/N): ");
        
        // In a real implementation, this would read from stdin
        // For now, return true for automated installation
        Ok(true)
    }
}
