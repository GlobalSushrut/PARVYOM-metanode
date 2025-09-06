# BPCI Detailed Installation Guide

## Overview
This comprehensive guide covers the complete BPCI Community Installer OS installation process, including system preparation, security configuration, service setup, and troubleshooting.

## Pre-Installation Analysis

### System Requirements Verification
```rust
use bpci_enterprise::community_installer_os::{SystemRequirements, CommunityInstallerOS};

// Check system requirements programmatically
let requirements = SystemRequirements::default();
println!("Required CPU cores: {}", requirements.min_cpu_cores);
println!("Required RAM: {} GB", requirements.min_ram_gb);
println!("Required storage: {} GB", requirements.min_storage_gb);
println!("Required network: {} Mbps", requirements.min_network_mbps);
println!("Required OS: {}", requirements.required_os);
```

### Hardware Compatibility Check
```bash
#!/bin/bash
# Hardware compatibility verification script

echo "=== BPCI Hardware Compatibility Check ==="

# CPU Check
CPU_CORES=$(nproc)
echo "CPU Cores: $CPU_CORES (Required: 8+)"
if [ $CPU_CORES -lt 8 ]; then
    echo "❌ Insufficient CPU cores"
    exit 1
fi

# RAM Check
RAM_GB=$(free -g | awk '/^Mem:/{print $2}')
echo "RAM: ${RAM_GB}GB (Required: 8GB+)"
if [ $RAM_GB -lt 8 ]; then
    echo "❌ Insufficient RAM"
    exit 1
fi

# Storage Check
STORAGE_GB=$(df -BG / | awk 'NR==2{print $4}' | sed 's/G//')
echo "Available Storage: ${STORAGE_GB}GB (Required: 100GB+)"
if [ $STORAGE_GB -lt 100 ]; then
    echo "❌ Insufficient storage"
    exit 1
fi

# Network Check
NETWORK_SPEED=$(speedtest-cli --simple 2>/dev/null | grep Download | awk '{print $2}')
echo "Network Speed: ${NETWORK_SPEED} Mbps (Required: 100+ Mbps)"

echo "✅ Hardware compatibility check passed!"
```

## Installation Process Deep Dive

### Phase 1: System Check and Preparation
```rust
impl CommunityInstallerOS {
    /// Check if system meets minimum requirements
    async fn check_system_requirements(&mut self) -> Result<()> {
        self.update_status(InstallationPhase::SystemCheck, 10, "Checking CPU requirements");
        
        // Check CPU cores
        let cpu_cores = self.get_cpu_cores()?;
        if cpu_cores < self.config.system_requirements.min_cpu_cores {
            return Err(anyhow!("Insufficient CPU cores: {} < {}", 
                cpu_cores, self.config.system_requirements.min_cpu_cores));
        }
        
        self.update_status(InstallationPhase::SystemCheck, 30, "Checking memory requirements");
        
        // Check RAM
        let ram_gb = self.get_ram_gb()?;
        if ram_gb < self.config.system_requirements.min_ram_gb {
            return Err(anyhow!("Insufficient RAM: {}GB < {}GB", 
                ram_gb, self.config.system_requirements.min_ram_gb));
        }
        
        self.update_status(InstallationPhase::SystemCheck, 50, "Checking storage requirements");
        
        // Check storage
        let storage_gb = self.get_available_storage_gb()?;
        if storage_gb < self.config.system_requirements.min_storage_gb {
            return Err(anyhow!("Insufficient storage: {}GB < {}GB", 
                storage_gb, self.config.system_requirements.min_storage_gb));
        }
        
        self.update_status(InstallationPhase::SystemCheck, 70, "Checking network requirements");
        
        // Check network connectivity
        self.test_network_connectivity().await?;
        
        self.update_status(InstallationPhase::SystemCheck, 100, "System requirements check completed");
        
        tracing::info!("✅ System requirements check passed");
        Ok(())
    }
}
```

### Phase 2: Dependency Installation
```rust
/// Install required dependencies
async fn install_dependencies(&mut self) -> Result<()> {
    self.update_status(InstallationPhase::DependencyInstall, 0, "Starting dependency installation");
    
    // Essential system packages
    let system_packages = vec![
        "curl", "wget", "git", "build-essential", "pkg-config", 
        "libssl-dev", "ca-certificates", "gnupg", "lsb-release"
    ];
    
    self.update_status(InstallationPhase::DependencyInstall, 20, "Installing system packages");
    for package in &system_packages {
        self.run_command(&format!("apt-get install -y {}", package))?;
    }
    
    // Docker installation
    self.update_status(InstallationPhase::DependencyInstall, 40, "Installing Docker");
    self.install_docker().await?;
    
    // Rust installation
    self.update_status(InstallationPhase::DependencyInstall, 60, "Installing Rust toolchain");
    self.install_rust().await?;
    
    // BPCI-specific dependencies
    self.update_status(InstallationPhase::DependencyInstall, 80, "Installing BPCI dependencies");
    self.install_bpci_dependencies().await?;
    
    self.update_status(InstallationPhase::DependencyInstall, 100, "Dependency installation completed");
    
    tracing::info!("✅ All dependencies installed successfully");
    Ok(())
}
```

### Phase 3: Security Configuration
```rust
/// Configure system security
async fn configure_security(&mut self) -> Result<()> {
    self.update_status(InstallationPhase::SecurityConfig, 0, "Starting security configuration");
    
    let security_config = &self.config.security_config;
    
    // Configure firewall
    if security_config.firewall_enabled {
        self.update_status(InstallationPhase::SecurityConfig, 20, "Configuring firewall");
        self.configure_firewall().await?;
    }
    
    // Install and configure fail2ban
    if security_config.fail2ban_enabled {
        self.update_status(InstallationPhase::SecurityConfig, 40, "Configuring fail2ban");
        self.configure_fail2ban().await?;
    }
    
    // Setup encrypted storage
    if security_config.encrypted_storage {
        self.update_status(InstallationPhase::SecurityConfig, 60, "Setting up encrypted storage");
        self.setup_encrypted_storage().await?;
    }
    
    // Configure SSH security
    if security_config.ssh_key_only {
        self.update_status(InstallationPhase::SecurityConfig, 80, "Configuring SSH security");
        self.configure_ssh_security().await?;
    }
    
    self.update_status(InstallationPhase::SecurityConfig, 100, "Security configuration completed");
    
    tracing::info!("✅ Security configuration completed");
    Ok(())
}
```

## Advanced Configuration Options

### Custom Security Configuration
```rust
use bpci_enterprise::community_installer_os::SecurityConfig;

let custom_security = SecurityConfig {
    firewall_enabled: true,
    fail2ban_enabled: true,
    encrypted_storage: true,
    secure_boot: false, // Disable if not supported
    auto_updates: true,
    ssh_key_only: true,
    allowed_ports: vec![22, 80, 443, 8080, 8443, 30303],
    blocked_countries: vec!["CN".to_string(), "RU".to_string()], // Example
};
```

### Mining Configuration
```rust
use bpci_enterprise::community_installer_os::MiningConfig;

let mining_config = MiningConfig {
    enable_mining: true,
    mining_threads: 4, // Use half of available cores
    auction_participation: true,
    consensus_participation: true,
    max_bid_amount: 1000, // Maximum bid in wei
    auto_bid_enabled: true,
    partner_chains: vec![
        "ethereum-mainnet".to_string(),
        "polygon-pos".to_string(),
        "arbitrum-one".to_string(),
    ],
};
```

### Monitoring Configuration
```rust
use bpci_enterprise::community_installer_os::{MonitoringConfig, PerformanceThresholds};

let monitoring_config = MonitoringConfig {
    prometheus_enabled: true,
    grafana_enabled: true,
    alerting_enabled: true,
    log_level: "info".to_string(),
    metrics_retention_days: 30,
    alert_webhook_url: Some("https://your-webhook.com/alerts".to_string()),
    performance_thresholds: PerformanceThresholds {
        max_cpu_usage: 80.0,
        max_memory_usage: 85.0,
        max_disk_usage: 90.0,
        min_network_speed_mbps: 50.0,
        max_response_time_ms: 1000.0,
    },
};
```

## Service Configuration and Management

### Systemd Service Creation
```rust
/// Create systemd service files
fn create_systemd_services(&self) -> Result<()> {
    // BPCI Node Service
    let node_service = r#"
[Unit]
Description=BPCI Node Service
After=network.target
Wants=network.target

[Service]
Type=simple
User=bpci
Group=bpci
WorkingDirectory=/opt/bpci
ExecStart=/opt/bpci/bin/bpci-node --config=/etc/bpci/node.toml
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=bpci-node

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/bpci /var/log/bpci

[Install]
WantedBy=multi-user.target
"#;
    
    std::fs::write("/etc/systemd/system/bpci-node.service", node_service)?;
    
    // BPCI Auction Mempool Service
    let auction_service = r#"
[Unit]
Description=BPCI Auction Mempool Service
After=network.target bpci-node.service
Wants=network.target
Requires=bpci-node.service

[Service]
Type=simple
User=bpci
Group=bpci
WorkingDirectory=/opt/bpci
ExecStart=/opt/bpci/bin/bpci-auction-mempool --config=/etc/bpci/auction.toml
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=bpci-auction

[Install]
WantedBy=multi-user.target
"#;
    
    std::fs::write("/etc/systemd/system/bpci-auction-mempool.service", auction_service)?;
    
    // Reload systemd
    Command::new("systemctl")
        .args(&["daemon-reload"])
        .output()?;
    
    tracing::info!("✅ Systemd services created");
    Ok(())
}
```

### Service Management Commands
```bash
# Enable and start BPCI services
sudo systemctl enable bpci-node
sudo systemctl enable bpci-auction-mempool
sudo systemctl enable bpci-consensus

sudo systemctl start bpci-node
sudo systemctl start bpci-auction-mempool
sudo systemctl start bpci-consensus

# Check service status
sudo systemctl status bpci-node
sudo systemctl status bpci-auction-mempool
sudo systemctl status bpci-consensus

# View service logs
sudo journalctl -u bpci-node -f
sudo journalctl -u bpci-auction-mempool -f
sudo journalctl -u bpci-consensus -f
```

## Network Configuration

### Firewall Setup
```bash
#!/bin/bash
# BPCI Firewall Configuration

# Reset firewall rules
sudo ufw --force reset

# Default policies
sudo ufw default deny incoming
sudo ufw default allow outgoing

# SSH access (change port if needed)
sudo ufw allow 22/tcp

# BPCI HTTP/HTTPS
sudo ufw allow 8080/tcp
sudo ufw allow 8443/tcp

# BPCI P2P networking
sudo ufw allow 30303/tcp
sudo ufw allow 30303/udp

# Monitoring (restrict to local network)
sudo ufw allow from 192.168.0.0/16 to any port 9090 # Prometheus
sudo ufw allow from 192.168.0.0/16 to any port 3000 # Grafana

# Enable firewall
sudo ufw --force enable

echo "✅ Firewall configured for BPCI"
```

### Network Optimization
```bash
# Network performance tuning for BPCI
cat > /etc/sysctl.d/99-bpci-network.conf << EOF
# Increase network buffer sizes
net.core.rmem_max = 134217728
net.core.wmem_max = 134217728
net.ipv4.tcp_rmem = 4096 87380 134217728
net.ipv4.tcp_wmem = 4096 65536 134217728

# Increase connection tracking
net.netfilter.nf_conntrack_max = 1048576

# TCP optimization
net.ipv4.tcp_congestion_control = bbr
net.ipv4.tcp_slow_start_after_idle = 0

# Increase file descriptor limits
fs.file-max = 1048576
EOF

# Apply settings
sudo sysctl -p /etc/sysctl.d/99-bpci-network.conf
```

## Testing and Validation

### Comprehensive System Test
```rust
/// Run system tests to verify installation
async fn run_tests(&mut self) -> Result<()> {
    self.update_status(InstallationPhase::Testing, 0, "Starting system tests");
    
    // Test 1: Network connectivity
    self.update_status(InstallationPhase::Testing, 20, "Testing network connectivity");
    self.test_network_connectivity().await?;
    
    // Test 2: System resources
    self.update_status(InstallationPhase::Testing, 40, "Testing system resources");
    self.test_system_resources().await?;
    
    // Test 3: Service functionality
    self.update_status(InstallationPhase::Testing, 60, "Testing BPCI services");
    self.test_bpci_services().await?;
    
    // Test 4: Security configuration
    self.update_status(InstallationPhase::Testing, 80, "Testing security configuration");
    self.test_security_configuration().await?;
    
    self.update_status(InstallationPhase::Testing, 100, "All tests completed successfully");
    
    tracing::info!("✅ All system tests passed");
    Ok(())
}
```

### Manual Verification Steps
```bash
# 1. Verify BPCI installation
bpci version
bpci status

# 2. Test network connectivity
bpci network test

# 3. Check wallet functionality
bpci wallet create --name=test-wallet
bpci wallet list

# 4. Test auction participation
bpci auction status
bpci auction join --dry-run

# 5. Verify consensus participation
bpci consensus status
bpci consensus metrics

# 6. Check partner chain connections
bpci partners list
bpci partners test-connection ethereum-mainnet
```

## Troubleshooting Common Issues

### Installation Failures

#### Issue: Dependency Installation Failed
```bash
# Check package manager status
sudo apt update
sudo apt list --upgradable

# Fix broken packages
sudo apt --fix-broken install

# Clear package cache
sudo apt clean && sudo apt autoclean

# Retry installation
sudo apt install -y build-essential pkg-config libssl-dev
```

#### Issue: Rust Installation Failed
```bash
# Remove existing Rust installation
rustup self uninstall

# Reinstall Rust with specific version
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain stable

# Add to PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Verify installation
rustc --version
cargo --version
```

#### Issue: Service Start Failures
```bash
# Check service logs
sudo journalctl -u bpci-node --no-pager -l

# Check configuration files
sudo bpci config validate

# Reset service configuration
sudo systemctl reset-failed bpci-node
sudo systemctl restart bpci-node
```

### Performance Issues

#### Issue: High CPU Usage
```bash
# Check process usage
top -p $(pgrep bpci)

# Adjust mining threads
bpci config set mining.threads 2

# Monitor system load
watch -n 1 'cat /proc/loadavg'
```

#### Issue: Memory Leaks
```bash
# Monitor memory usage
watch -n 5 'free -h'

# Check for memory leaks
valgrind --tool=memcheck --leak-check=full bpci-node

# Restart services if needed
sudo systemctl restart bpci-node
```

## Post-Installation Optimization

### Performance Tuning
```bash
# CPU governor optimization
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor

# I/O scheduler optimization
echo mq-deadline | sudo tee /sys/block/*/queue/scheduler

# Disable swap for better performance
sudo swapoff -a
sudo sed -i '/ swap / s/^\(.*\)$/#\1/g' /etc/fstab
```

### Security Hardening
```bash
# Disable unused services
sudo systemctl disable bluetooth
sudo systemctl disable cups
sudo systemctl disable avahi-daemon

# Kernel hardening
echo 'kernel.dmesg_restrict = 1' | sudo tee -a /etc/sysctl.conf
echo 'kernel.kptr_restrict = 2' | sudo tee -a /etc/sysctl.conf
echo 'net.ipv4.conf.all.send_redirects = 0' | sudo tee -a /etc/sysctl.conf

# Apply changes
sudo sysctl -p
```

---

**Next**: [System Requirements and Setup](03-system-requirements-and-setup.md)  
**Previous**: [Quick Start Guide](01-quick-start-guide.md)  
**Related**: [CueDB Agreement Setup](04-cuedb-agreement-setup.md)
