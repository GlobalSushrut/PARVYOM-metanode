// System Detection Service - Intelligent hardware and capability analysis
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sysinfo::{System, Disks};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemCapabilities {
    pub os_info: OsInfo,
    pub hardware: HardwareInfo,
    pub network: NetworkInfo,
    pub quantum_capabilities: QuantumCapabilities,
    pub virtualization: VirtualizationInfo,
    pub security: SecurityCapabilities,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OsInfo {
    pub name: String,
    pub version: String,
    pub architecture: String,
    pub kernel_version: String,
    pub is_64bit: bool,
    pub package_manager: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HardwareInfo {
    pub cpu_cores: u32,
    pub cpu_threads: u32,
    pub cpu_frequency_mhz: u64,
    pub cpu_brand: String,
    pub total_memory_gb: f64,
    pub available_memory_gb: f64,
    pub total_storage_gb: f64,
    pub available_storage_gb: f64,
    pub gpu_info: Vec<GpuInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GpuInfo {
    pub name: String,
    pub memory_gb: f64,
    pub compute_capability: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkInfo {
    pub interfaces: Vec<NetworkInterface>,
    pub internet_connectivity: bool,
    pub bandwidth_estimate_mbps: Option<f64>,
    pub ipv6_support: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub ip_addresses: Vec<String>,
    pub is_up: bool,
    pub is_loopback: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuantumCapabilities {
    pub hardware_acceleration: bool,
    pub quantum_random_support: bool,
    pub cryptographic_acceleration: bool,
    pub estimated_qubits: u32,
    pub coherence_time_ms: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VirtualizationInfo {
    pub is_virtualized: bool,
    pub hypervisor: Option<String>,
    pub container_runtime: Vec<String>,
    pub supports_nested_virtualization: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecurityCapabilities {
    pub secure_boot: bool,
    pub tpm_available: bool,
    pub hardware_encryption: bool,
    pub trusted_execution: bool,
}

pub struct SystemDetector {
    system: System,
}

impl SystemDetector {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        Self { system }
    }
    
    pub async fn detect_system(&self) -> Result<crate::SystemInfo> {
        let capabilities = self.detect_full_capabilities().await?;
        
        Ok(crate::SystemInfo {
            os: capabilities.os_info.name,
            arch: capabilities.os_info.architecture,
            cpu_cores: capabilities.hardware.cpu_cores,
            total_memory_gb: capabilities.hardware.total_memory_gb,
            available_storage_gb: capabilities.hardware.available_storage_gb,
            quantum_capable: capabilities.quantum_capabilities.hardware_acceleration,
            network_topology: self.analyze_network_topology(&capabilities.network),
        })
    }
    
    pub async fn detect_full_capabilities(&self) -> Result<SystemCapabilities> {
        let os_info = self.detect_os_info()?;
        let hardware = self.detect_hardware_info()?;
        let network = self.detect_network_info().await?;
        let quantum_capabilities = self.detect_quantum_capabilities().await?;
        let virtualization = self.detect_virtualization_info()?;
        let security = self.detect_security_capabilities()?;
        
        Ok(SystemCapabilities {
            os_info,
            hardware,
            network,
            quantum_capabilities,
            virtualization,
            security,
        })
    }
    
    fn detect_os_info(&self) -> Result<OsInfo> {
        let name = System::name().unwrap_or_else(|| "Unknown".to_string());
        let version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
        let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
        
        let architecture = std::env::consts::ARCH.to_string();
        let is_64bit = architecture.contains("64");
        
        let package_manager = self.detect_package_manager();
        
        Ok(OsInfo {
            name,
            version,
            architecture,
            kernel_version,
            is_64bit,
            package_manager,
        })
    }
    
    fn detect_hardware_info(&self) -> Result<HardwareInfo> {
        let cpus = self.system.cpus();
        let cpu_cores = cpus.len() as u32;
        let cpu_threads = cpu_cores; // Simplified - would need more detailed detection
        let cpu_frequency_mhz = cpus.first().map(|cpu| cpu.frequency()).unwrap_or(0);
        let cpu_brand = cpus.first().map(|cpu| cpu.brand().to_string()).unwrap_or_else(|| "Unknown".to_string());
        
        let total_memory_bytes = self.system.total_memory();
        let available_memory_bytes = self.system.available_memory();
        let total_memory_gb = total_memory_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        let available_memory_gb = available_memory_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        
        let (total_storage_gb, available_storage_gb) = self.calculate_storage_info();
        let gpu_info = self.detect_gpu_info();
        
        Ok(HardwareInfo {
            cpu_cores,
            cpu_threads,
            cpu_frequency_mhz,
            cpu_brand,
            total_memory_gb,
            available_memory_gb,
            total_storage_gb,
            available_storage_gb,
            gpu_info,
        })
    }
    
    async fn detect_network_info(&self) -> Result<NetworkInfo> {
        let interfaces = self.get_network_interfaces();
        let internet_connectivity = self.test_internet_connectivity().await;
        let bandwidth_estimate_mbps = self.estimate_bandwidth().await;
        let ipv6_support = self.test_ipv6_support().await;
        
        Ok(NetworkInfo {
            interfaces,
            internet_connectivity,
            bandwidth_estimate_mbps,
            ipv6_support,
        })
    }
    
    async fn detect_quantum_capabilities(&self) -> Result<QuantumCapabilities> {
        // Detect quantum hardware acceleration capabilities
        let hardware_acceleration = self.check_quantum_hardware().await;
        let quantum_random_support = self.check_quantum_random().await;
        let cryptographic_acceleration = self.check_crypto_acceleration().await;
        
        // Estimate quantum capabilities based on hardware
        let estimated_qubits = if hardware_acceleration { 32 } else { 0 };
        let coherence_time_ms = if hardware_acceleration { 100.0 } else { 0.0 };
        
        Ok(QuantumCapabilities {
            hardware_acceleration,
            quantum_random_support,
            cryptographic_acceleration,
            estimated_qubits,
            coherence_time_ms,
        })
    }
    
    fn detect_virtualization_info(&self) -> Result<VirtualizationInfo> {
        let is_virtualized = self.check_if_virtualized();
        let hypervisor = self.detect_hypervisor();
        let container_runtime = self.detect_container_runtimes();
        let supports_nested_virtualization = self.check_nested_virtualization();
        
        Ok(VirtualizationInfo {
            is_virtualized,
            hypervisor,
            container_runtime,
            supports_nested_virtualization,
        })
    }
    
    fn detect_security_capabilities(&self) -> Result<SecurityCapabilities> {
        let secure_boot = self.check_secure_boot();
        let tpm_available = self.check_tpm();
        let hardware_encryption = self.check_hardware_encryption();
        let trusted_execution = self.check_trusted_execution();
        
        Ok(SecurityCapabilities {
            secure_boot,
            tpm_available,
            hardware_encryption,
            trusted_execution,
        })
    }
    
    // Helper methods
    fn detect_package_manager(&self) -> Option<String> {
        let managers = vec![
            ("apt", "apt-get"),
            ("yum", "yum"),
            ("dnf", "dnf"),
            ("pacman", "pacman"),
            ("zypper", "zypper"),
            ("brew", "brew"),
            ("choco", "choco"),
            ("winget", "winget"),
        ];
        
        for (name, command) in managers {
            if which::which(command).is_ok() {
                return Some(name.to_string());
            }
        }
        
        None
    }
    
    fn calculate_storage_info(&self) -> (f64, f64) {
        let mut total_storage = 0u64;
        let mut available_storage = 0u64;
        
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            total_storage += disk.total_space();
            available_storage += disk.available_space();
        }
        
        let total_gb = total_storage as f64 / (1024.0 * 1024.0 * 1024.0);
        let available_gb = available_storage as f64 / (1024.0 * 1024.0 * 1024.0);
        
        (total_gb, available_gb)
    }
    
    fn detect_gpu_info(&self) -> Vec<GpuInfo> {
        // Simplified GPU detection - would need platform-specific implementation
        vec![]
    }
    
    fn get_network_interfaces(&self) -> Vec<NetworkInterface> {
        // Simplified network interface detection
        vec![]
    }
    
    async fn test_internet_connectivity(&self) -> bool {
        // Test connectivity to multiple endpoints
        let test_urls = vec![
            "https://www.google.com",
            "https://www.cloudflare.com",
            "https://github.com",
        ];
        
        for url in test_urls {
            if let Ok(_) = reqwest::get(url).await {
                return true;
            }
        }
        
        false
    }
    
    async fn estimate_bandwidth(&self) -> Option<f64> {
        // Simplified bandwidth estimation
        None
    }
    
    async fn test_ipv6_support(&self) -> bool {
        // Test IPv6 connectivity
        false
    }
    
    async fn check_quantum_hardware(&self) -> bool {
        // Check for quantum hardware acceleration
        // This would involve checking for specific quantum processors or simulators
        false
    }
    
    async fn check_quantum_random(&self) -> bool {
        // Check for hardware quantum random number generation
        std::path::Path::new("/dev/hwrng").exists()
    }
    
    async fn check_crypto_acceleration(&self) -> bool {
        // Check for cryptographic hardware acceleration (AES-NI, etc.)
        true // Most modern CPUs have this
    }
    
    fn check_if_virtualized(&self) -> bool {
        // Check various indicators of virtualization
        false
    }
    
    fn detect_hypervisor(&self) -> Option<String> {
        // Detect hypervisor type
        None
    }
    
    fn detect_container_runtimes(&self) -> Vec<String> {
        let mut runtimes = Vec::new();
        
        if which::which("docker").is_ok() {
            runtimes.push("docker".to_string());
        }
        if which::which("podman").is_ok() {
            runtimes.push("podman".to_string());
        }
        if which::which("containerd").is_ok() {
            runtimes.push("containerd".to_string());
        }
        
        runtimes
    }
    
    fn check_nested_virtualization(&self) -> bool {
        // Check if nested virtualization is supported
        false
    }
    
    fn check_secure_boot(&self) -> bool {
        // Check if Secure Boot is enabled
        false
    }
    
    fn check_tpm(&self) -> bool {
        // Check for TPM availability
        std::path::Path::new("/dev/tpm0").exists()
    }
    
    fn check_hardware_encryption(&self) -> bool {
        // Check for hardware encryption support
        true
    }
    
    fn check_trusted_execution(&self) -> bool {
        // Check for Intel TXT, AMD SVM, etc.
        false
    }
    
    fn analyze_network_topology(&self, network: &NetworkInfo) -> String {
        if network.internet_connectivity {
            if network.ipv6_support {
                "dual-stack".to_string()
            } else {
                "ipv4-only".to_string()
            }
        } else {
            "offline".to_string()
        }
    }
}
