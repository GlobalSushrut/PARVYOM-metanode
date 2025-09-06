//! Hardware Detection Engine
//! 
//! Automatically detect and configure hardware for immutable BPI OS installation

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use tracing::{info, warn, debug};

/// Hardware profile for the target system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareProfile {
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub storage: Vec<StorageDevice>,
    pub network: Vec<NetworkInterface>,
    pub graphics: Vec<GraphicsDevice>,
    pub system: SystemInfo,
    pub security_features: SecurityFeatures,
}

impl HardwareProfile {
    pub fn summary(&self) -> String {
        format!(
            "{} CPU, {:.2}GB RAM, {} storage devices, {} network interfaces",
            self.cpu.model_name,
            self.memory.total_gb,
            self.storage.len(),
            self.network.len()
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub model_name: String,
    pub cores: u32,
    pub threads: u32,
    pub architecture: String,
    pub features: Vec<String>,
    pub max_frequency: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub total_gb: f64,
    pub available_bytes: u64,
    pub memory_type: String,
    pub swap_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDevice {
    pub device_path: String,
    pub device_type: StorageType,
    pub size_bytes: u64,
    pub size_gb: f64,
    pub filesystem: Option<String>,
    pub mount_point: Option<String>,
    pub is_boot_device: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageType {
    SSD,
    HDD,
    NVMe,
    USB,
    SD,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub interface_type: NetworkType,
    pub mac_address: Option<String>,
    pub ip_addresses: Vec<String>,
    pub is_up: bool,
    pub speed: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkType {
    Ethernet,
    WiFi,
    Loopback,
    Bridge,
    Virtual,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsDevice {
    pub vendor: String,
    pub model: String,
    pub driver: Option<String>,
    pub memory_mb: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub boot_mode: BootMode,
    pub virtualization: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BootMode {
    UEFI,
    BIOS,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityFeatures {
    pub secure_boot: bool,
    pub tpm_available: bool,
    pub tpm_version: Option<String>,
    pub hardware_rng: bool,
    pub aes_ni: bool,
    pub virtualization_support: bool,
}

/// Hardware Detection Engine
#[derive(Debug)]
pub struct HardwareDetectionEngine {
    system: sysinfo::System,
}

impl HardwareDetectionEngine {
    /// Create new hardware detection engine
    pub async fn new() -> Result<Self> {
        info!("Initializing Hardware Detection Engine");
        
        let mut system = sysinfo::System::new();
        system.refresh_all();
        
        Ok(Self { system })
    }

    /// Detect complete hardware profile
    pub async fn detect_hardware(&self) -> Result<HardwareProfile> {
        info!("🔍 Detecting hardware configuration...");
        
        let cpu = self.detect_cpu_info().await?;
        let memory = self.detect_memory_info().await?;
        let storage = self.detect_storage_devices().await?;
        let network = self.detect_network_interfaces().await?;
        let graphics = self.detect_graphics_devices().await?;
        let system = self.detect_system_info().await?;
        let security_features = self.detect_security_features().await?;
        
        let profile = HardwareProfile {
            cpu,
            memory,
            storage,
            network,
            graphics,
            system,
            security_features,
        };
        
        info!("✅ Hardware detection completed");
        debug!("Hardware profile: {:#?}", profile);
        
        Ok(profile)
    }

    /// Detect CPU information
    async fn detect_cpu_info(&self) -> Result<CpuInfo> {
        debug!("Detecting CPU information");
        
        let cpus = self.system.cpus();
        if cpus.is_empty() {
            return Err(anyhow!("No CPU information available"));
        }
        
        let cpu = &cpus[0];
        let model_name = cpu.brand().to_string();
        let cores = self.system.physical_core_count().unwrap_or(cpus.len()) as u32;
        let threads = cpus.len() as u32;
        let architecture = std::env::consts::ARCH.to_string();
        
        // Detect CPU features from /proc/cpuinfo
        let features = self.detect_cpu_features().await.unwrap_or_default();
        
        // Try to get max frequency
        let max_frequency = cpus.iter()
            .map(|cpu| cpu.frequency())
            .max();
        
        Ok(CpuInfo {
            model_name,
            cores: cores,
            threads,
            architecture,
            features,
            max_frequency,
        })
    }

    /// Detect CPU features from /proc/cpuinfo
    async fn detect_cpu_features(&self) -> Result<Vec<String>> {
        let cpuinfo = fs::read_to_string("/proc/cpuinfo")
            .map_err(|e| anyhow!("Failed to read /proc/cpuinfo: {}", e))?;
        
        for line in cpuinfo.lines() {
            if line.starts_with("flags") || line.starts_with("Features") {
                let features: Vec<String> = line
                    .split(':')
                    .nth(1)
                    .unwrap_or("")
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                return Ok(features);
            }
        }
        
        Ok(Vec::new())
    }

    /// Detect memory information
    async fn detect_memory_info(&self) -> Result<MemoryInfo> {
        debug!("Detecting memory information");
        
        let total_bytes = self.system.total_memory();
        let total_gb = total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        let available_bytes = self.system.available_memory();
        let swap_bytes = self.system.total_swap();
        
        // Try to detect memory type from DMI
        let memory_type = self.detect_memory_type().await.unwrap_or_else(|_| "Unknown".to_string());
        
        Ok(MemoryInfo {
            total_bytes,
            total_gb,
            available_bytes,
            memory_type,
            swap_bytes,
        })
    }

    /// Detect memory type from DMI information
    async fn detect_memory_type(&self) -> Result<String> {
        // Try to read memory type from dmidecode
        let output = Command::new("dmidecode")
            .args(&["-t", "memory"])
            .output();
        
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.trim().starts_with("Type:") {
                    if let Some(memory_type) = line.split(':').nth(1) {
                        return Ok(memory_type.trim().to_string());
                    }
                }
            }
        }
        
        Ok("Unknown".to_string())
    }

    /// Detect storage devices
    async fn detect_storage_devices(&self) -> Result<Vec<StorageDevice>> {
        debug!("Detecting storage devices");
        
        let mut devices = Vec::new();
        
        // Read block devices from /proc/partitions
        let partitions = fs::read_to_string("/proc/partitions")
            .map_err(|e| anyhow!("Failed to read /proc/partitions: {}", e))?;
        
        for line in partitions.lines().skip(2) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let device_name = parts[3];
                if !device_name.chars().last().unwrap_or('0').is_ascii_digit() {
                    // This is a whole device, not a partition
                    if let Ok(device) = self.analyze_storage_device(device_name).await {
                        devices.push(device);
                    }
                }
            }
        }
        
        Ok(devices)
    }

    /// Analyze individual storage device
    async fn analyze_storage_device(&self, device_name: &str) -> Result<StorageDevice> {
        let device_path = format!("/dev/{}", device_name);
        
        // Get device size
        let size_bytes = self.get_device_size(&device_path).await.unwrap_or(0);
        let size_gb = size_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        
        // Determine device type
        let device_type = self.determine_storage_type(device_name).await;
        
        // Check if this is a boot device
        let is_boot_device = self.is_boot_device(&device_path).await;
        
        // Get filesystem and mount point information
        let (filesystem, mount_point) = self.get_filesystem_info(&device_path).await;
        
        Ok(StorageDevice {
            device_path,
            device_type,
            size_bytes,
            size_gb,
            filesystem,
            mount_point,
            is_boot_device,
        })
    }

    /// Get device size in bytes
    async fn get_device_size(&self, device_path: &str) -> Result<u64> {
        let output = Command::new("blockdev")
            .args(&["--getsize64", device_path])
            .output()
            .map_err(|e| anyhow!("Failed to get device size: {}", e))?;
        
        let size_str = String::from_utf8_lossy(&output.stdout);
        size_str.trim().parse::<u64>()
            .map_err(|e| anyhow!("Failed to parse device size: {}", e))
    }

    /// Determine storage device type
    async fn determine_storage_type(&self, device_name: &str) -> StorageType {
        // Check if it's NVMe
        if device_name.starts_with("nvme") {
            return StorageType::NVMe;
        }
        
        // Check if it's USB
        if device_name.starts_with("sd") {
            // Try to determine if it's USB by checking sysfs
            let usb_path = format!("/sys/block/{}/device/../../../../../../idVendor", device_name);
            if fs::read_to_string(&usb_path).is_ok() {
                return StorageType::USB;
            }
        }
        
        // Check if it's SSD by reading rotational flag
        let rotational_path = format!("/sys/block/{}/queue/rotational", device_name);
        if let Ok(rotational) = fs::read_to_string(&rotational_path) {
            if rotational.trim() == "0" {
                return StorageType::SSD;
            } else {
                return StorageType::HDD;
            }
        }
        
        StorageType::Unknown
    }

    /// Check if device is boot device
    async fn is_boot_device(&self, device_path: &str) -> bool {
        // Check if device contains /boot or / mount points
        let output = Command::new("lsblk")
            .args(&["-n", "-o", "MOUNTPOINT", device_path])
            .output();
        
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let mount_point = line.trim();
                if mount_point == "/" || mount_point == "/boot" {
                    return true;
                }
            }
        }
        
        false
    }

    /// Get filesystem and mount point information
    async fn get_filesystem_info(&self, device_path: &str) -> (Option<String>, Option<String>) {
        let output = Command::new("lsblk")
            .args(&["-n", "-o", "FSTYPE,MOUNTPOINT", device_path])
            .output();
        
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = stdout.lines().next() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let filesystem = if parts.is_empty() || parts[0].is_empty() {
                    None
                } else {
                    Some(parts[0].to_string())
                };
                let mount_point = if parts.len() < 2 || parts[1].is_empty() {
                    None
                } else {
                    Some(parts[1].to_string())
                };
                return (filesystem, mount_point);
            }
        }
        
        (None, None)
    }

    /// Detect network interfaces
    async fn detect_network_interfaces(&self) -> Result<Vec<NetworkInterface>> {
        debug!("Detecting network interfaces");
        
        let mut interfaces = Vec::new();
        
        let networks = sysinfo::Networks::new_with_refreshed_list();
        for (name, data) in &networks {
            let interface_type = self.determine_network_type(name).await;
            let mac_address = data.mac_address().to_string();
            let mac_address = if mac_address.is_empty() { None } else { Some(mac_address) };
            
            // Get IP addresses
            let ip_addresses = self.get_interface_ip_addresses(name).await.unwrap_or_default();
            
            // Check if interface is up
            let is_up = self.is_interface_up(name).await;
            
            // Try to get interface speed
            let speed = self.get_interface_speed(name).await;
            
            interfaces.push(NetworkInterface {
                name: name.to_string(),
                interface_type,
                mac_address,
                ip_addresses,
                is_up,
                speed,
            });
        }
        
        Ok(interfaces)
    }

    /// Determine network interface type
    async fn determine_network_type(&self, interface_name: &str) -> NetworkType {
        if interface_name == "lo" {
            return NetworkType::Loopback;
        }
        
        if interface_name.starts_with("eth") || interface_name.starts_with("en") {
            return NetworkType::Ethernet;
        }
        
        if interface_name.starts_with("wl") || interface_name.starts_with("wlan") {
            return NetworkType::WiFi;
        }
        
        if interface_name.starts_with("br") {
            return NetworkType::Bridge;
        }
        
        if interface_name.starts_with("veth") || interface_name.starts_with("docker") {
            return NetworkType::Virtual;
        }
        
        NetworkType::Unknown
    }

    /// Get IP addresses for interface
    async fn get_interface_ip_addresses(&self, interface_name: &str) -> Result<Vec<String>> {
        let output = Command::new("ip")
            .args(&["addr", "show", interface_name])
            .output()
            .map_err(|e| anyhow!("Failed to get IP addresses: {}", e))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut ip_addresses = Vec::new();
        
        for line in stdout.lines() {
            if line.trim().starts_with("inet ") {
                if let Some(ip) = line.split_whitespace().nth(1) {
                    ip_addresses.push(ip.to_string());
                }
            }
        }
        
        Ok(ip_addresses)
    }

    /// Check if network interface is up
    async fn is_interface_up(&self, interface_name: &str) -> bool {
        let operstate_path = format!("/sys/class/net/{}/operstate", interface_name);
        if let Ok(state) = fs::read_to_string(&operstate_path) {
            return state.trim() == "up";
        }
        false
    }

    /// Get network interface speed
    async fn get_interface_speed(&self, interface_name: &str) -> Option<u64> {
        let speed_path = format!("/sys/class/net/{}/speed", interface_name);
        if let Ok(speed_str) = fs::read_to_string(&speed_path) {
            if let Ok(speed) = speed_str.trim().parse::<u64>() {
                return Some(speed * 1_000_000); // Convert Mbps to bps
            }
        }
        None
    }

    /// Detect graphics devices
    async fn detect_graphics_devices(&self) -> Result<Vec<GraphicsDevice>> {
        debug!("Detecting graphics devices");
        
        let mut devices = Vec::new();
        
        // Try to use lspci to detect graphics devices
        let output = Command::new("lspci")
            .args(&["-v"])
            .output();
        
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut current_device: Option<GraphicsDevice> = None;
            
            for line in stdout.lines() {
                if line.contains("VGA compatible controller") || line.contains("3D controller") {
                    // Parse graphics device line
                    if let Some(device_info) = line.split(':').nth(2) {
                        let parts: Vec<&str> = device_info.trim().split(' ').collect();
                        let vendor = parts.first().unwrap_or(&"Unknown").to_string();
                        let model = parts[1..].join(" ");
                        
                        current_device = Some(GraphicsDevice {
                            vendor,
                            model,
                            driver: None,
                            memory_mb: None,
                        });
                    }
                } else if line.trim().starts_with("Kernel driver in use:") {
                    if let Some(ref mut device) = current_device {
                        if let Some(driver) = line.split(':').nth(1) {
                            device.driver = Some(driver.trim().to_string());
                        }
                    }
                } else if line.is_empty() && current_device.is_some() {
                    devices.push(current_device.take().unwrap());
                }
            }
            
            // Add last device if exists
            if let Some(device) = current_device {
                devices.push(device);
            }
        }
        
        Ok(devices)
    }

    /// Detect system information
    async fn detect_system_info(&self) -> Result<SystemInfo> {
        debug!("Detecting system information");
        
        let hostname = sysinfo::System::host_name().unwrap_or_else(|| "unknown".to_string());
        let os_name = sysinfo::System::name().unwrap_or_else(|| "Unknown".to_string());
        let os_version = sysinfo::System::os_version().unwrap_or_else(|| "Unknown".to_string());
        let kernel_version = sysinfo::System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
        
        // Detect boot mode
        let boot_mode = if fs::metadata("/sys/firmware/efi").is_ok() {
            BootMode::UEFI
        } else {
            BootMode::BIOS
        };
        
        // Detect virtualization
        let virtualization = self.detect_virtualization().await;
        
        Ok(SystemInfo {
            hostname,
            os_name,
            os_version,
            kernel_version,
            boot_mode,
            virtualization,
        })
    }

    /// Detect virtualization environment
    async fn detect_virtualization(&self) -> Option<String> {
        // Check for common virtualization indicators
        if let Ok(dmi_product) = fs::read_to_string("/sys/class/dmi/id/product_name") {
            let product = dmi_product.trim().to_lowercase();
            if product.contains("vmware") {
                return Some("VMware".to_string());
            } else if product.contains("virtualbox") {
                return Some("VirtualBox".to_string());
            } else if product.contains("kvm") {
                return Some("KVM".to_string());
            }
        }
        
        // Check for container environments
        if fs::metadata("/.dockerenv").is_ok() {
            return Some("Docker".to_string());
        }
        
        None
    }

    /// Detect security features
    async fn detect_security_features(&self) -> Result<SecurityFeatures> {
        debug!("Detecting security features");
        
        // Check Secure Boot
        let secure_boot = self.check_secure_boot().await;
        
        // Check TPM
        let (tpm_available, tpm_version) = self.check_tpm().await;
        
        // Check hardware RNG
        let hardware_rng = fs::metadata("/dev/hwrng").is_ok();
        
        // Check AES-NI support
        let aes_ni = self.check_aes_ni_support().await;
        
        // Check virtualization support
        let virtualization_support = self.check_virtualization_support().await;
        
        Ok(SecurityFeatures {
            secure_boot,
            tpm_available,
            tpm_version,
            hardware_rng,
            aes_ni,
            virtualization_support,
        })
    }

    /// Check if Secure Boot is enabled
    async fn check_secure_boot(&self) -> bool {
        if let Ok(secure_boot) = fs::read_to_string("/sys/firmware/efi/efivars/SecureBoot-8be4df61-93ca-11d2-aa0d-00e098032b8c") {
            // Check if the last byte is 1 (enabled)
            return secure_boot.as_bytes().last() == Some(&1);
        }
        false
    }

    /// Check TPM availability and version
    async fn check_tpm(&self) -> (bool, Option<String>) {
        // Check for TPM 2.0
        if fs::metadata("/dev/tpm0").is_ok() {
            if let Ok(version) = fs::read_to_string("/sys/class/tpm/tpm0/tpm_version_major") {
                return (true, Some(format!("2.{}", version.trim())));
            }
            return (true, Some("2.0".to_string()));
        }
        
        // Check for TPM 1.2
        if fs::metadata("/dev/tpm").is_ok() {
            return (true, Some("1.2".to_string()));
        }
        
        (false, None)
    }

    /// Check AES-NI support
    async fn check_aes_ni_support(&self) -> bool {
        if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
            return cpuinfo.contains("aes");
        }
        false
    }

    /// Check virtualization support
    async fn check_virtualization_support(&self) -> bool {
        if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
            return cpuinfo.contains("vmx") || cpuinfo.contains("svm");
        }
        false
    }
}
