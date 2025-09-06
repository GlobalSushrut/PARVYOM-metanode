# BPCI System Requirements and Setup

## Hardware Requirements

### Minimum System Specifications
```
Component          | Minimum       | Recommended   | Enterprise
-------------------|---------------|---------------|-------------
CPU                | 8 cores       | 16 cores      | 32+ cores
RAM                | 8 GB          | 16 GB         | 32+ GB
Storage            | 100 GB SSD    | 500 GB NVMe   | 2+ TB NVMe
Network            | 100 Mbps      | 1 Gbps        | 10+ Gbps
GPU (Optional)     | -             | GTX 1660      | RTX 3080+
```

### Detailed Hardware Analysis
```rust
use bpci_enterprise::community_installer_os::{SystemRequirements, SystemInfo};

impl SystemRequirements {
    /// Get requirements based on deployment type
    pub fn for_deployment_type(deployment_type: &str) -> Self {
        match deployment_type {
            "community" => Self::default(),
            "enterprise" => Self {
                min_cpu_cores: 16,
                min_ram_gb: 32,
                min_storage_gb: 500,
                min_network_mbps: 1000,
                required_os: "Ubuntu 22.04 LTS".to_string(),
                required_kernel_version: "5.15.0".to_string(),
            },
            "datacenter" => Self {
                min_cpu_cores: 32,
                min_ram_gb: 64,
                min_storage_gb: 2000,
                min_network_mbps: 10000,
                required_os: "Ubuntu 22.04 LTS".to_string(),
                required_kernel_version: "5.15.0".to_string(),
            },
            _ => Self::default(),
        }
    }
}
```

## Operating System Support

### Supported Operating Systems
| OS Distribution | Version | Support Level | Notes |
|----------------|---------|---------------|-------|
| Ubuntu LTS | 22.04, 20.04 | ✅ Full | Recommended |
| Debian | 11, 12 | ✅ Full | Stable |
| CentOS/RHEL | 8, 9 | ⚠️ Limited | Community |
| Fedora | 37, 38 | ⚠️ Limited | Testing |
| Arch Linux | Rolling | ❌ Experimental | Advanced users |

### Ubuntu 22.04 LTS Setup (Recommended)
```bash
# Update system packages
sudo apt update && sudo apt upgrade -y

# Install essential packages
sudo apt install -y \
    curl wget git vim \
    build-essential pkg-config \
    libssl-dev ca-certificates \
    software-properties-common \
    apt-transport-https \
    gnupg lsb-release

# Install additional tools
sudo apt install -y \
    htop iotop nethogs \
    ufw fail2ban \
    prometheus-node-exporter \
    docker.io docker-compose
```

### System Optimization for BPCI
```bash
# Create BPCI system configuration
cat > /etc/sysctl.d/99-bpci.conf << EOF
# Network optimizations
net.core.rmem_max = 134217728
net.core.wmem_max = 134217728
net.ipv4.tcp_rmem = 4096 87380 134217728
net.ipv4.tcp_wmem = 4096 65536 134217728
net.ipv4.tcp_congestion_control = bbr

# File system optimizations
fs.file-max = 1048576
fs.inotify.max_user_watches = 524288

# Memory management
vm.swappiness = 10
vm.dirty_ratio = 15
vm.dirty_background_ratio = 5

# Security
kernel.dmesg_restrict = 1
kernel.kptr_restrict = 2
net.ipv4.conf.all.send_redirects = 0
net.ipv4.conf.all.accept_redirects = 0
EOF

# Apply optimizations
sudo sysctl -p /etc/sysctl.d/99-bpci.conf
```

## Network Requirements

### Port Configuration
```rust
use bpci_enterprise::community_installer_os::SecurityConfig;

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            firewall_enabled: true,
            fail2ban_enabled: true,
            encrypted_storage: true,
            secure_boot: false,
            auto_updates: true,
            ssh_key_only: true,
            allowed_ports: vec![
                22,    // SSH
                80,    // HTTP
                443,   // HTTPS
                8080,  // BPCI HTTP API
                8443,  // BPCI HTTPS API
                30303, // BPCI P2P (TCP/UDP)
                9090,  // Prometheus (internal)
                3000,  // Grafana (internal)
            ],
            blocked_countries: vec![], // Configure as needed
        }
    }
}
```

### Firewall Configuration
```bash
#!/bin/bash
# BPCI Network Security Setup

# Reset and configure UFW
sudo ufw --force reset
sudo ufw default deny incoming
sudo ufw default allow outgoing

# Essential services
sudo ufw allow ssh
sudo ufw allow 80/tcp   # HTTP
sudo ufw allow 443/tcp  # HTTPS

# BPCI specific ports
sudo ufw allow 8080/tcp  # BPCI HTTP API
sudo ufw allow 8443/tcp  # BPCI HTTPS API
sudo ufw allow 30303     # BPCI P2P (both TCP and UDP)

# Monitoring (restrict to local networks)
sudo ufw allow from 10.0.0.0/8 to any port 9090    # Prometheus
sudo ufw allow from 172.16.0.0/12 to any port 9090
sudo ufw allow from 192.168.0.0/16 to any port 9090

sudo ufw allow from 10.0.0.0/8 to any port 3000    # Grafana
sudo ufw allow from 172.16.0.0/12 to any port 3000
sudo ufw allow from 192.168.0.0/16 to any port 3000

# Enable firewall
sudo ufw --force enable
sudo ufw status verbose
```

### Network Performance Testing
```bash
# Test network connectivity and performance
bpci-network-test() {
    echo "=== BPCI Network Performance Test ==="
    
    # Basic connectivity
    echo "Testing basic connectivity..."
    ping -c 4 8.8.8.8 || echo "❌ Internet connectivity failed"
    
    # DNS resolution
    echo "Testing DNS resolution..."
    nslookup bpci-network.com || echo "❌ DNS resolution failed"
    
    # Port connectivity
    echo "Testing BPCI network ports..."
    nc -zv bpci-network.com 30303 || echo "❌ P2P port unreachable"
    
    # Bandwidth test (if speedtest-cli is available)
    if command -v speedtest-cli &> /dev/null; then
        echo "Testing bandwidth..."
        speedtest-cli --simple
    fi
    
    # Latency test
    echo "Testing latency to BPCI nodes..."
    ping -c 10 node1.bpci-network.com | tail -1
    
    echo "✅ Network test completed"
}
```

## Storage Requirements

### Storage Configuration
```rust
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub data_directory: PathBuf,
    pub logs_directory: PathBuf,
    pub cache_directory: PathBuf,
    pub backup_directory: PathBuf,
    pub max_log_size_mb: u64,
    pub log_retention_days: u32,
    pub cache_size_gb: u32,
    pub backup_retention_days: u32,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            data_directory: PathBuf::from("/var/lib/bpci"),
            logs_directory: PathBuf::from("/var/log/bpci"),
            cache_directory: PathBuf::from("/var/cache/bpci"),
            backup_directory: PathBuf::from("/var/backups/bpci"),
            max_log_size_mb: 100,
            log_retention_days: 30,
            cache_size_gb: 10,
            backup_retention_days: 7,
        }
    }
}
```

### Storage Setup Script
```bash
#!/bin/bash
# BPCI Storage Configuration

# Create BPCI user and group
sudo useradd -r -s /bin/false -d /var/lib/bpci bpci

# Create directory structure
sudo mkdir -p /var/lib/bpci/{data,wallets,keys}
sudo mkdir -p /var/log/bpci
sudo mkdir -p /var/cache/bpci
sudo mkdir -p /var/backups/bpci
sudo mkdir -p /etc/bpci

# Set permissions
sudo chown -R bpci:bpci /var/lib/bpci
sudo chown -R bpci:bpci /var/log/bpci
sudo chown -R bpci:bpci /var/cache/bpci
sudo chown -R bpci:bpci /var/backups/bpci
sudo chown -R root:bpci /etc/bpci

# Set secure permissions
sudo chmod 750 /var/lib/bpci
sudo chmod 700 /var/lib/bpci/wallets
sudo chmod 700 /var/lib/bpci/keys
sudo chmod 755 /var/log/bpci
sudo chmod 755 /var/cache/bpci
sudo chmod 750 /etc/bpci

echo "✅ BPCI storage structure created"
```

### Disk Performance Optimization
```bash
# SSD optimization for BPCI data directory
echo 'ACTION=="add|change", KERNEL=="sd[a-z]", ATTR{queue/rotational}=="0", ATTR{queue/scheduler}="mq-deadline"' | sudo tee /etc/udev/rules.d/60-ssd-scheduler.rules

# Mount options for BPCI data partition
cat >> /etc/fstab << EOF
# BPCI data partition optimizations
UUID=your-data-partition-uuid /var/lib/bpci ext4 defaults,noatime,nodiratime,commit=60 0 2
EOF

# Apply I/O scheduler changes
sudo udevadm control --reload-rules
sudo udevadm trigger
```

## Security Setup

### SSL/TLS Certificate Configuration
```bash
# Generate self-signed certificates for development
sudo mkdir -p /etc/bpci/ssl
cd /etc/bpci/ssl

# Generate private key
sudo openssl genrsa -out bpci.key 4096

# Generate certificate signing request
sudo openssl req -new -key bpci.key -out bpci.csr -subj "/C=US/ST=State/L=City/O=BPCI/CN=localhost"

# Generate self-signed certificate
sudo openssl x509 -req -days 365 -in bpci.csr -signkey bpci.key -out bpci.crt

# Set secure permissions
sudo chown root:bpci /etc/bpci/ssl/*
sudo chmod 640 /etc/bpci/ssl/*

echo "✅ SSL certificates generated"
```

### Fail2Ban Configuration for BPCI
```bash
# Create BPCI-specific fail2ban configuration
cat > /etc/fail2ban/jail.d/bpci.conf << EOF
[bpci-api]
enabled = true
port = 8080,8443
filter = bpci-api
logpath = /var/log/bpci/api.log
maxretry = 5
bantime = 3600
findtime = 600

[bpci-p2p]
enabled = true
port = 30303
filter = bpci-p2p
logpath = /var/log/bpci/p2p.log
maxretry = 10
bantime = 1800
findtime = 300
EOF

# Create filter for BPCI API
cat > /etc/fail2ban/filter.d/bpci-api.conf << EOF
[Definition]
failregex = ^.*\[ERROR\].*Failed authentication from <HOST>.*$
            ^.*\[WARN\].*Suspicious activity from <HOST>.*$
            ^.*\[ERROR\].*Rate limit exceeded from <HOST>.*$
ignoreregex =
EOF

# Create filter for BPCI P2P
cat > /etc/fail2ban/filter.d/bpci-p2p.conf << EOF
[Definition]
failregex = ^.*\[ERROR\].*Invalid handshake from <HOST>.*$
            ^.*\[WARN\].*Malformed packet from <HOST>.*$
ignoreregex =
EOF

# Restart fail2ban
sudo systemctl restart fail2ban
```

## Performance Monitoring Setup

### System Monitoring Configuration
```rust
use bpci_enterprise::community_installer_os::{MonitoringConfig, PerformanceThresholds};

impl MonitoringConfig {
    pub fn production_config() -> Self {
        Self {
            prometheus_enabled: true,
            grafana_enabled: true,
            alerting_enabled: true,
            log_level: "info".to_string(),
            metrics_retention_days: 90,
            alert_webhook_url: Some("https://alerts.your-domain.com/webhook".to_string()),
            performance_thresholds: PerformanceThresholds {
                max_cpu_usage: 80.0,
                max_memory_usage: 85.0,
                max_disk_usage: 90.0,
                min_network_speed_mbps: 100.0,
                max_response_time_ms: 500.0,
            },
        }
    }
}
```

### Prometheus Configuration
```yaml
# /etc/prometheus/prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

rule_files:
  - "bpci_rules.yml"

scrape_configs:
  - job_name: 'bpci-node'
    static_configs:
      - targets: ['localhost:9100']  # Node exporter
      - targets: ['localhost:8080']  # BPCI metrics endpoint
    scrape_interval: 5s
    metrics_path: /metrics

  - job_name: 'bpci-auction'
    static_configs:
      - targets: ['localhost:8081']  # Auction mempool metrics
    scrape_interval: 10s

  - job_name: 'bpci-consensus'
    static_configs:
      - targets: ['localhost:8082']  # Consensus metrics
    scrape_interval: 5s

alerting:
  alertmanagers:
    - static_configs:
        - targets:
          - alertmanager:9093
```

### Grafana Dashboard Setup
```bash
# Install Grafana
wget -q -O - https://packages.grafana.com/gpg.key | sudo apt-key add -
echo "deb https://packages.grafana.com/oss/deb stable main" | sudo tee /etc/apt/sources.list.d/grafana.list
sudo apt update && sudo apt install grafana

# Configure Grafana for BPCI
cat > /etc/grafana/provisioning/datasources/prometheus.yml << EOF
apiVersion: 1
datasources:
  - name: Prometheus
    type: prometheus
    access: proxy
    url: http://localhost:9090
    isDefault: true
EOF

# Start and enable Grafana
sudo systemctl enable grafana-server
sudo systemctl start grafana-server
```

## Validation and Testing

### System Validation Script
```bash
#!/bin/bash
# BPCI System Validation

validate_system() {
    echo "=== BPCI System Validation ==="
    
    # Check hardware requirements
    echo "Validating hardware requirements..."
    
    CPU_CORES=$(nproc)
    RAM_GB=$(free -g | awk '/^Mem:/{print $2}')
    STORAGE_GB=$(df -BG / | awk 'NR==2{print $4}' | sed 's/G//')
    
    echo "CPU Cores: $CPU_CORES (Required: 8+)"
    echo "RAM: ${RAM_GB}GB (Required: 8GB+)"
    echo "Storage: ${STORAGE_GB}GB (Required: 100GB+)"
    
    # Check software dependencies
    echo "Validating software dependencies..."
    
    command -v docker >/dev/null 2>&1 || echo "❌ Docker not installed"
    command -v rustc >/dev/null 2>&1 || echo "❌ Rust not installed"
    command -v git >/dev/null 2>&1 || echo "❌ Git not installed"
    
    # Check network connectivity
    echo "Validating network connectivity..."
    
    ping -c 1 8.8.8.8 >/dev/null 2>&1 || echo "❌ Internet connectivity failed"
    
    # Check firewall configuration
    echo "Validating firewall configuration..."
    
    sudo ufw status | grep -q "Status: active" || echo "❌ Firewall not active"
    
    # Check directory structure
    echo "Validating directory structure..."
    
    [ -d "/var/lib/bpci" ] || echo "❌ BPCI data directory missing"
    [ -d "/var/log/bpci" ] || echo "❌ BPCI log directory missing"
    [ -d "/etc/bpci" ] || echo "❌ BPCI config directory missing"
    
    echo "✅ System validation completed"
}

validate_system
```

### Performance Benchmark
```bash
# CPU benchmark
sysbench cpu --cpu-max-prime=20000 --threads=$(nproc) run

# Memory benchmark
sysbench memory --memory-total-size=10G --threads=$(nproc) run

# Disk I/O benchmark
sysbench fileio --file-total-size=10G --file-test-mode=rndrw prepare
sysbench fileio --file-total-size=10G --file-test-mode=rndrw --threads=16 run
sysbench fileio --file-total-size=10G --file-test-mode=rndrw cleanup

# Network benchmark (if iperf3 is available)
iperf3 -c speedtest.net -t 30
```

---

**Next**: [CueDB Agreement Setup](04-cuedb-agreement-setup.md)  
**Previous**: [Detailed Installation Guide](02-detailed-installation-guide.md)  
**Related**: [Security Configuration](../07-firewall-and-security/), [Performance Optimization](../32-performance-optimization/)
