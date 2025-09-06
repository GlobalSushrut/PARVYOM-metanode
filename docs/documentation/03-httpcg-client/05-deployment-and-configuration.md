# HttpCG Client Deployment and Configuration Guide

## Overview

This guide provides comprehensive instructions for deploying and configuring the HttpCG protocol client in production environments. It covers installation, configuration, security hardening, monitoring setup, and operational best practices for enterprise-grade deployments.

## System Requirements

### Hardware Requirements
```yaml
Minimum Configuration:
  CPU: 4 cores (x86_64 or ARM64)
  RAM: 8GB
  Storage: 50GB SSD
  Network: 1Gbps

Recommended Configuration:
  CPU: 8+ cores (x86_64 or ARM64)
  RAM: 16GB+
  Storage: 100GB+ NVMe SSD
  Network: 10Gbps+

Enterprise Configuration:
  CPU: 16+ cores (x86_64 or ARM64)
  RAM: 32GB+
  Storage: 500GB+ NVMe SSD
  Network: 25Gbps+
```

### Software Requirements
```yaml
Operating System:
  - Ubuntu 22.04 LTS (Recommended)
  - Ubuntu 20.04 LTS
  - Debian 11+
  - CentOS 8+
  - RHEL 8+
  - Amazon Linux 2

Runtime Dependencies:
  - Rust 1.70+
  - OpenSSL 3.0+
  - libsodium 1.0.18+
  - Protocol Buffers 3.21+

Optional Dependencies:
  - Docker 24.0+ (for containerized deployment)
  - Kubernetes 1.25+ (for orchestrated deployment)
  - Prometheus (for monitoring)
  - Grafana (for visualization)
```

## Installation Methods

### Method 1: Binary Installation
```bash
#!/bin/bash
# Download and install HttpCG client binary

# 1. Download latest release
HTTPCG_VERSION="1.0.0"
wget "https://releases.pravyom.com/httpcg-client/v${HTTPCG_VERSION}/httpcg-client-linux-x86_64.tar.gz"

# 2. Verify checksum
wget "https://releases.pravyom.com/httpcg-client/v${HTTPCG_VERSION}/checksums.txt"
sha256sum -c checksums.txt --ignore-missing

# 3. Extract and install
tar -xzf httpcg-client-linux-x86_64.tar.gz
sudo cp httpcg-client /usr/local/bin/
sudo chmod +x /usr/local/bin/httpcg-client

# 4. Create configuration directory
sudo mkdir -p /etc/httpcg-client
sudo mkdir -p /var/lib/httpcg-client
sudo mkdir -p /var/log/httpcg-client

# 5. Create service user
sudo useradd --system --home /var/lib/httpcg-client --shell /bin/false httpcg

# 6. Set permissions
sudo chown -R httpcg:httpcg /var/lib/httpcg-client
sudo chown -R httpcg:httpcg /var/log/httpcg-client
```

### Method 2: Source Compilation
```bash
#!/bin/bash
# Compile HttpCG client from source

# 1. Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustup update stable

# 2. Install system dependencies
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libsodium-dev \
    protobuf-compiler \
    git

# 3. Clone repository
git clone https://github.com/pravyom/bpi-core.git
cd bpi-core

# 4. Build HttpCG client
cargo build --release --bin httpcg-client

# 5. Install binary
sudo cp target/release/httpcg-client /usr/local/bin/
sudo chmod +x /usr/local/bin/httpcg-client

# 6. Verify installation
httpcg-client --version
```

### Method 3: DockLock Deployment (Recommended)
```yaml
# httpcg-client-docklock.yml - DockLock deployment specification
apiVersion: docklock.bpi.network/v1
kind: AppDeployment
metadata:
  name: httpcg-client
  namespace: bpci-infrastructure
spec:
  # Application specification
  app:
    name: httpcg-client
    version: "1.0.0"
    binary: "/usr/local/bin/httpcg-client"
    args: ["start", "--config", "/etc/httpcg-client/config.toml"]
    workingDir: "/var/lib/httpcg-client"
    
  # Security policies
  security:
    # Syscall filtering for enhanced security
    seccomp:
      defaultAction: SCMP_ACT_ERRNO
      allowedSyscalls:
        - read
        - write
        - open
        - close
        - socket
        - connect
        - bind
        - listen
        - accept
        - sendto
        - recvfrom
        - epoll_create
        - epoll_ctl
        - epoll_wait
        - futex
        - mmap
        - munmap
        - brk
        - rt_sigaction
        - rt_sigprocmask
        - clone
        - execve
        - exit_group
        - getrandom  # For cryptographic operations
        - clock_gettime  # For timestamps
        
    # Resource constraints
    resources:
      memory: "2Gi"
      cpu: "1000m"
      disk: "10Gi"
      networkBandwidth: "1Gbps"
      
    # Network policies
    network:
      allowOutbound:
        - "gateway.pravyom.com:443"
        - "ledger.bpi.network:443"
        - "shadow.bpi.network:443"
      allowInbound:
        - port: 8080
          protocol: TCP
        - port: 9090
          protocol: TCP  # Metrics
        - port: 8081
          protocol: TCP  # Health check
          
  # Deterministic execution
  deterministic:
    enabled: true
    rngSeed: "httpcg-client-deterministic-seed"
    
  # Witness recording
  witness:
    enabled: true
    maxSize: "100MB"
    recordTypes:
      - network
      - filesystem
      - cryptographic
      
  # Environment configuration
  environment:
    HTTPCG_CONFIG_FILE: "/etc/httpcg-client/config.toml"
    HTTPCG_LOG_LEVEL: "info"
    HTTPCG_ENVIRONMENT: "production"
    RUST_LOG: "httpcg_client=info"
    
  # Volume mounts
  volumes:
    - name: config
      hostPath: "/etc/httpcg-client"
      containerPath: "/etc/httpcg-client"
      readOnly: true
    - name: data
      hostPath: "/var/lib/httpcg-client"
      containerPath: "/var/lib/httpcg-client"
      readOnly: false
    - name: logs
      hostPath: "/var/log/httpcg-client"
      containerPath: "/var/log/httpcg-client"
      readOnly: false
      
  # Health checks
  health:
    livenessProbe:
      httpGet:
        path: "/health"
        port: 8081
      initialDelaySeconds: 30
      periodSeconds: 10
    readinessProbe:
      httpGet:
        path: "/health"
        port: 8081
      initialDelaySeconds: 5
      periodSeconds: 5
      
  # Audit configuration
  audit:
    enabled: true
    stepReceipts: true
    cryptographicSigning: true
    immutableLogging: true
```

```bash
# Deploy using DockLock platform
bpci docklock deploy httpcg-client-docklock.yml

# Check deployment status
bpci docklock status httpcg-client

# View deployment logs
bpci docklock logs httpcg-client --follow

# Scale deployment (if needed)
bpci docklock scale httpcg-client --replicas 3
```

## Configuration

### Main Configuration File
```toml
# /etc/httpcg-client/config.toml

[client]
# Client identification
client_id = "httpcg-client-prod-001"
version = "1.0.0"
environment = "production"

# Network configuration
bind_address = "0.0.0.0"
bind_port = 8080
max_connections = 10000
connection_timeout = "30s"
read_timeout = "60s"
write_timeout = "60s"

[wallet]
# Wallet configuration for identity
wallet_path = "/var/lib/httpcg-client/wallet"
private_key_file = "/var/lib/httpcg-client/keys/private.pem"
public_key_file = "/var/lib/httpcg-client/keys/public.pem"
did_document_path = "/var/lib/httpcg-client/did.json"

# Key generation settings
key_algorithm = "Ed25519"
key_size = 256

[shadow_registry]
# Shadow Registry endpoints
gateway_endpoints = [
    "https://gateway1.pravyom.com",
    "https://gateway2.pravyom.com",
    "https://gateway3.pravyom.com"
]

# Cache configuration
cache_size = 10000
cache_ttl = "5m"
persistent_cache = true
cache_directory = "/var/lib/httpcg-client/cache"

# Retry configuration
max_retries = 3
retry_backoff = ["100ms", "500ms", "2s"]
retry_jitter = true

[tlsls]
# TLSLS certificate configuration
certificate_lifetime = "90d"
renewal_threshold = "7d"
auto_renewal = true

# Cryptographic settings
use_hybrid_signatures = true
require_post_quantum = true
min_key_size = 256

# Certificate storage
certificate_directory = "/var/lib/httpcg-client/certificates"
ca_bundle_path = "/etc/ssl/certs/ca-certificates.crt"

[qlock]
# QLOCK session lock configuration
session_lifetime = "1m"
max_concurrent_sessions = 1000
cleanup_interval = "30s"

# Security settings
enable_distance_bounding = true
max_distance_meters = 50.0
enable_bridge_break_detection = true
strict_temporal_validation = true

# Performance settings
enable_session_caching = true
cache_size_limit = 10000
batch_key_derivation = true

[security]
# Security policies
enforce_rbac = true
require_authentication = true
allow_anonymous = false

# Rate limiting
enable_rate_limiting = true
requests_per_minute = 1000
burst_size = 100

# Input validation
max_request_size = "10MB"
max_header_size = "8KB"
max_url_length = 2048

[logging]
# Logging configuration
level = "info"
format = "json"
output = "file"
file_path = "/var/log/httpcg-client/httpcg.log"
max_file_size = "100MB"
max_files = 10
compress_rotated = true

# Audit logging
audit_enabled = true
audit_file = "/var/log/httpcg-client/audit.log"
audit_level = "all"

[monitoring]
# Metrics configuration
enable_metrics = true
metrics_port = 9090
metrics_path = "/metrics"

# Health check
health_check_port = 8081
health_check_path = "/health"

# Tracing
enable_tracing = true
tracing_endpoint = "http://jaeger:14268/api/traces"
sample_rate = 0.1

[bpi_integration]
# BPI ledger integration
ledger_endpoints = [
    "https://ledger1.bpi.network",
    "https://ledger2.bpi.network"
]

# Connection settings
connection_pool_size = 10
request_timeout = "30s"
retry_attempts = 3

# Anchoring settings
enable_anchoring = true
anchor_interval = "1h"
batch_size = 100
```

### Environment Variables
```bash
# /etc/httpcg-client/environment

# Core configuration
export HTTPCG_CONFIG_FILE="/etc/httpcg-client/config.toml"
export HTTPCG_LOG_LEVEL="info"
export HTTPCG_ENVIRONMENT="production"

# Security
export HTTPCG_PRIVATE_KEY_PATH="/var/lib/httpcg-client/keys/private.pem"
export HTTPCG_CERTIFICATE_DIR="/var/lib/httpcg-client/certificates"

# Network
export HTTPCG_BIND_ADDRESS="0.0.0.0"
export HTTPCG_BIND_PORT="8080"

# Performance
export HTTPCG_MAX_CONNECTIONS="10000"
export HTTPCG_WORKER_THREADS="8"

# Monitoring
export HTTPCG_METRICS_ENABLED="true"
export HTTPCG_METRICS_PORT="9090"

# BPI Integration
export BPI_LEDGER_ENDPOINT="https://ledger.bpi.network"
export BPI_WALLET_ADDRESS=""  # Set during deployment
```

### Systemd Service Configuration
```ini
# /etc/systemd/system/httpcg-client.service

[Unit]
Description=HttpCG Protocol Client
Documentation=https://docs.pravyom.com/httpcg-client
After=network-online.target
Wants=network-online.target

[Service]
Type=exec
User=httpcg
Group=httpcg
ExecStart=/usr/local/bin/httpcg-client start --config /etc/httpcg-client/config.toml
ExecReload=/bin/kill -HUP $MAINPID
Restart=always
RestartSec=5
TimeoutStopSec=30

# Security settings
NoNewPrivileges=yes
PrivateTmp=yes
ProtectSystem=strict
ProtectHome=yes
ReadWritePaths=/var/lib/httpcg-client /var/log/httpcg-client
CapabilityBoundingSet=CAP_NET_BIND_SERVICE

# Resource limits
LimitNOFILE=65536
LimitNPROC=4096
MemoryMax=2G
CPUQuota=400%

# Environment
EnvironmentFile=-/etc/httpcg-client/environment
WorkingDirectory=/var/lib/httpcg-client

[Install]
WantedBy=multi-user.target
```

## Security Hardening

### SSL/TLS Configuration
```toml
# Enhanced TLS configuration
[tls]
# Protocol versions
min_version = "1.3"
max_version = "1.3"

# Cipher suites (TLS 1.3)
cipher_suites = [
    "TLS_AES_256_GCM_SHA384",
    "TLS_CHACHA20_POLY1305_SHA256",
    "TLS_AES_128_GCM_SHA256"
]

# Certificate settings
certificate_file = "/etc/httpcg-client/tls/server.crt"
private_key_file = "/etc/httpcg-client/tls/server.key"
ca_file = "/etc/httpcg-client/tls/ca.crt"

# Security options
require_client_cert = true
verify_client_cert = true
enable_sni = true
enable_ocsp_stapling = true

# HSTS settings
hsts_enabled = true
hsts_max_age = 31536000  # 1 year
hsts_include_subdomains = true
hsts_preload = true
```

### Firewall Configuration
```bash
#!/bin/bash
# UFW firewall rules for HttpCG client

# Reset firewall
sudo ufw --force reset

# Default policies
sudo ufw default deny incoming
sudo ufw default allow outgoing

# SSH access (adjust as needed)
sudo ufw allow from 10.0.0.0/8 to any port 22

# HttpCG client ports
sudo ufw allow 8080/tcp comment "HttpCG client main port"
sudo ufw allow 9090/tcp comment "HttpCG metrics port"
sudo ufw allow 8081/tcp comment "HttpCG health check"

# BPI ledger access (outgoing)
sudo ufw allow out 443/tcp comment "HTTPS outbound"
sudo ufw allow out 80/tcp comment "HTTP outbound"

# DNS
sudo ufw allow out 53/udp comment "DNS"
sudo ufw allow out 53/tcp comment "DNS over TCP"

# NTP
sudo ufw allow out 123/udp comment "NTP"

# Enable firewall
sudo ufw enable
```

### SELinux Configuration
```bash
#!/bin/bash
# SELinux configuration for HttpCG client

# Create SELinux policy
cat > httpcg-client.te << 'EOF'
module httpcg-client 1.0;

require {
    type unconfined_t;
    type http_port_t;
    type cert_t;
    class tcp_socket { bind listen };
    class file { read write };
}

# Allow binding to HTTP ports
allow unconfined_t http_port_t:tcp_socket { bind listen };

# Allow reading certificates
allow unconfined_t cert_t:file read;
EOF

# Compile and install policy
checkmodule -M -m -o httpcg-client.mod httpcg-client.te
semodule_package -o httpcg-client.pp -m httpcg-client.mod
sudo semodule -i httpcg-client.pp

# Set file contexts
sudo setsebool -P httpd_can_network_connect 1
sudo semanage fcontext -a -t bin_t "/usr/local/bin/httpcg-client"
sudo restorecon /usr/local/bin/httpcg-client
```

## Monitoring and Observability

### Prometheus Configuration
```yaml
# prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'httpcg-client'
    static_configs:
      - targets: ['localhost:9090']
    scrape_interval: 10s
    metrics_path: /metrics
    
  - job_name: 'httpcg-health'
    static_configs:
      - targets: ['localhost:8081']
    scrape_interval: 30s
    metrics_path: /health
```

### Grafana Dashboard
```json
{
  "dashboard": {
    "title": "HttpCG Client Monitoring",
    "panels": [
      {
        "title": "Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(httpcg_requests_total[5m])",
            "legendFormat": "Requests/sec"
          }
        ]
      },
      {
        "title": "Response Time",
        "type": "graph", 
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(httpcg_request_duration_seconds_bucket[5m]))",
            "legendFormat": "95th percentile"
          },
          {
            "expr": "histogram_quantile(0.50, rate(httpcg_request_duration_seconds_bucket[5m]))",
            "legendFormat": "50th percentile"
          }
        ]
      },
      {
        "title": "Error Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(httpcg_errors_total[5m])",
            "legendFormat": "Errors/sec"
          }
        ]
      },
      {
        "title": "Active Connections",
        "type": "singlestat",
        "targets": [
          {
            "expr": "httpcg_active_connections",
            "legendFormat": "Connections"
          }
        ]
      }
    ]
  }
}
```

### Log Aggregation
```yaml
# filebeat.yml for log shipping
filebeat.inputs:
  - type: log
    enabled: true
    paths:
      - /var/log/httpcg-client/*.log
    fields:
      service: httpcg-client
      environment: production
    json.keys_under_root: true
    json.add_error_key: true

output.elasticsearch:
  hosts: ["elasticsearch:9200"]
  index: "httpcg-client-%{+yyyy.MM.dd}"

logging.level: info
logging.to_files: true
logging.files:
  path: /var/log/filebeat
  name: filebeat
  keepfiles: 7
  permissions: 0644
```

## Backup and Recovery

### Backup Script
```bash
#!/bin/bash
# /usr/local/bin/httpcg-backup.sh

set -euo pipefail

BACKUP_DIR="/var/backups/httpcg-client"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_NAME="httpcg-client-${TIMESTAMP}"

# Create backup directory
mkdir -p "${BACKUP_DIR}/${BACKUP_NAME}"

# Stop service for consistent backup
sudo systemctl stop httpcg-client

# Backup configuration
cp -r /etc/httpcg-client "${BACKUP_DIR}/${BACKUP_NAME}/"

# Backup data
cp -r /var/lib/httpcg-client "${BACKUP_DIR}/${BACKUP_NAME}/"

# Backup logs (last 7 days)
find /var/log/httpcg-client -name "*.log" -mtime -7 -exec cp {} "${BACKUP_DIR}/${BACKUP_NAME}/" \;

# Create archive
cd "${BACKUP_DIR}"
tar -czf "${BACKUP_NAME}.tar.gz" "${BACKUP_NAME}"
rm -rf "${BACKUP_NAME}"

# Start service
sudo systemctl start httpcg-client

# Cleanup old backups (keep 30 days)
find "${BACKUP_DIR}" -name "httpcg-client-*.tar.gz" -mtime +30 -delete

echo "Backup completed: ${BACKUP_DIR}/${BACKUP_NAME}.tar.gz"
```

### Recovery Procedure
```bash
#!/bin/bash
# Recovery procedure for HttpCG client

BACKUP_FILE="$1"
if [[ -z "$BACKUP_FILE" ]]; then
    echo "Usage: $0 <backup_file.tar.gz>"
    exit 1
fi

# Stop service
sudo systemctl stop httpcg-client

# Backup current state
sudo mv /etc/httpcg-client /etc/httpcg-client.old
sudo mv /var/lib/httpcg-client /var/lib/httpcg-client.old

# Extract backup
cd /tmp
tar -xzf "$BACKUP_FILE"
BACKUP_DIR=$(basename "$BACKUP_FILE" .tar.gz)

# Restore configuration
sudo cp -r "${BACKUP_DIR}/httpcg-client" /etc/
sudo chown -R root:root /etc/httpcg-client
sudo chmod -R 644 /etc/httpcg-client
sudo chmod 755 /etc/httpcg-client

# Restore data
sudo cp -r "${BACKUP_DIR}/httpcg-client" /var/lib/
sudo chown -R httpcg:httpcg /var/lib/httpcg-client
sudo chmod -R 750 /var/lib/httpcg-client

# Start service
sudo systemctl start httpcg-client

# Verify service
sleep 5
sudo systemctl status httpcg-client

echo "Recovery completed successfully"
```

## High Availability Setup

### DockLock Load Balancer Configuration
```yaml
# httpcg-loadbalancer.yml - DockLock native load balancer
apiVersion: docklock.bpi.network/v1
kind: LoadBalancer
metadata:
  name: httpcg-client-lb
  namespace: bpci-infrastructure
spec:
  # Load balancer configuration
  algorithm: "weighted-round-robin"
  healthCheck:
    path: "/health"
    port: 8081
    interval: "10s"
    timeout: "5s"
    unhealthyThreshold: 3
    healthyThreshold: 2
    
  # Backend services
  backends:
    - name: "httpcg-client-cluster"
      weight: 100
      maxConnections: 1000
      
  # SSL/TLS configuration
  tls:
    enabled: true
    certificates:
      - secretName: "httpcg-client-tls"
        domains:
          - "httpcg.bpi.network"
          - "api.httpcg.bpi.network"
    protocols:
      - "TLSv1.3"
    cipherSuites:
      - "TLS_AES_256_GCM_SHA384"
      - "TLS_CHACHA20_POLY1305_SHA256"
      
  # Security policies
  security:
    rateLimiting:
      enabled: true
      requestsPerMinute: 10000
      burstSize: 1000
    ddosProtection:
      enabled: true
      threshold: 50000  # requests per minute
    geoBlocking:
      enabled: false  # Disabled for global access
      
  # Monitoring
  monitoring:
    metrics:
      enabled: true
      port: 9091
    logging:
      enabled: true
      level: "info"
      format: "json"
```

```bash
# Deploy load balancer
bpci docklock deploy httpcg-loadbalancer.yml

# Check load balancer status
bpci docklock status httpcg-client-lb

# View load balancer metrics
bpci docklock metrics httpcg-client-lb
```

### DockLock Cluster Deployment
```yaml
# httpcg-client-cluster.yml - Multi-node DockLock deployment
apiVersion: docklock.bpi.network/v1
kind: ClusterDeployment
metadata:
  name: httpcg-client-cluster
  namespace: bpci-infrastructure
spec:
  # Cluster configuration
  cluster:
    replicas: 3
    distribution: "zone-aware"  # Distribute across availability zones
    loadBalancing:
      algorithm: "least-connections"
      healthCheck:
        path: "/health"
        interval: "10s"
        
  # Application template
  template:
    metadata:
      labels:
        app: httpcg-client
        tier: infrastructure
    spec:
      # Inherit from base deployment
      extends: "httpcg-client-docklock.yml"
      
      # Cluster-specific overrides
      security:
        # Enhanced security for cluster deployment
        isolation:
          level: "strict"
          networkNamespace: true
          pidNamespace: true
          ipcNamespace: true
          
        # Inter-node communication
        mesh:
          enabled: true
          encryption: "tls-1.3"
          authentication: "mutual-tls"
          
      # Resource allocation per node
      resources:
        memory: "4Gi"  # Increased for cluster node
        cpu: "2000m"
        disk: "20Gi"
        
      # Cluster networking
      network:
        clusterIP: "auto"
        externalTrafficPolicy: "Cluster"
        sessionAffinity: "ClientIP"
        
  # Service mesh configuration
  serviceMesh:
    enabled: true
    provider: "bpi-mesh"
    encryption: true
    observability:
      tracing: true
      metrics: true
      logging: true
      
  # Auto-scaling
  autoscaling:
    enabled: true
    minReplicas: 3
    maxReplicas: 10
    metrics:
      - type: "cpu"
        targetAverageUtilization: 70
      - type: "memory"
        targetAverageUtilization: 80
      - type: "custom"
        name: "httpcg_requests_per_second"
        targetValue: 1000
```

```bash
# Deploy cluster using DockLock
bpci docklock cluster deploy httpcg-client-cluster.yml

# Monitor cluster status
bpci docklock cluster status httpcg-client-cluster

# Scale cluster
bpci docklock cluster scale httpcg-client-cluster --replicas 5

# Rolling update
bpci docklock cluster update httpcg-client-cluster --image httpcg-client:1.1.0
```

## Performance Tuning

### System Optimization
```bash
#!/bin/bash
# System performance tuning for HttpCG client

# Kernel parameters
cat >> /etc/sysctl.conf << 'EOF'
# Network performance
net.core.rmem_max = 134217728
net.core.wmem_max = 134217728
net.ipv4.tcp_rmem = 4096 65536 134217728
net.ipv4.tcp_wmem = 4096 65536 134217728
net.core.netdev_max_backlog = 5000
net.ipv4.tcp_congestion_control = bbr

# File descriptors
fs.file-max = 1000000
fs.nr_open = 1000000

# Memory
vm.swappiness = 1
vm.dirty_ratio = 15
vm.dirty_background_ratio = 5
EOF

# Apply settings
sysctl -p

# Increase file descriptor limits
cat >> /etc/security/limits.conf << 'EOF'
httpcg soft nofile 65536
httpcg hard nofile 65536
httpcg soft nproc 32768
httpcg hard nproc 32768
EOF
```

### Application Tuning
```toml
# Performance-optimized configuration
[performance]
# Worker threads (set to number of CPU cores)
worker_threads = 8

# Connection pool settings
connection_pool_size = 1000
connection_pool_timeout = "30s"
connection_keepalive = "60s"

# Buffer sizes
read_buffer_size = "64KB"
write_buffer_size = "64KB"
max_frame_size = "1MB"

# Caching
enable_response_caching = true
response_cache_size = "100MB"
response_cache_ttl = "5m"

# Compression
enable_compression = true
compression_algorithm = "brotli"
compression_level = 6
min_compress_size = "1KB"
```

## Troubleshooting

### Common Issues and Solutions

#### Issue 1: Connection Timeouts
```bash
# Check network connectivity
curl -v https://gateway.pravyom.com/health

# Check DNS resolution
nslookup gateway.pravyom.com

# Check firewall rules
sudo ufw status verbose

# Increase timeout values in config
[client]
connection_timeout = "60s"
read_timeout = "120s"
```

#### Issue 2: Certificate Errors
```bash
# Check certificate validity
openssl x509 -in /var/lib/httpcg-client/certificates/client.crt -text -noout

# Regenerate certificates
httpcg-client cert generate --force

# Check certificate permissions
ls -la /var/lib/httpcg-client/certificates/
```

#### Issue 3: High Memory Usage
```bash
# Check memory usage
ps aux | grep httpcg-client
cat /proc/$(pgrep httpcg-client)/status

# Adjust cache sizes
[shadow_registry]
cache_size = 5000  # Reduce from 10000

[qlock]
cache_size_limit = 5000  # Reduce from 10000
```

### Diagnostic Commands
```bash
# Service status
sudo systemctl status httpcg-client

# View logs
sudo journalctl -u httpcg-client -f

# Check configuration
httpcg-client config validate

# Test connectivity
httpcg-client test connection --endpoint https://gateway.pravyom.com

# Performance metrics
curl http://localhost:9090/metrics

# Health check
curl http://localhost:8081/health
```

## Maintenance Procedures

### Regular Maintenance Tasks
```bash
#!/bin/bash
# /usr/local/bin/httpcg-maintenance.sh

# Log rotation
sudo logrotate /etc/logrotate.d/httpcg-client

# Certificate renewal check
httpcg-client cert check --warn-days 30

# Cache cleanup
httpcg-client cache cleanup --max-age 7d

# Performance metrics collection
httpcg-client metrics export --format prometheus > /tmp/httpcg-metrics.txt

# Health check
httpcg-client health-check --detailed

# Update check
httpcg-client version check --notify
```

### Upgrade Procedure
```bash
#!/bin/bash
# Upgrade HttpCG client

NEW_VERSION="$1"
if [[ -z "$NEW_VERSION" ]]; then
    echo "Usage: $0 <new_version>"
    exit 1
fi

# Backup current installation
/usr/local/bin/httpcg-backup.sh

# Download new version
wget "https://releases.pravyom.com/httpcg-client/v${NEW_VERSION}/httpcg-client-linux-x86_64.tar.gz"

# Verify checksum
wget "https://releases.pravyom.com/httpcg-client/v${NEW_VERSION}/checksums.txt"
sha256sum -c checksums.txt --ignore-missing

# Stop service
sudo systemctl stop httpcg-client

# Install new binary
tar -xzf httpcg-client-linux-x86_64.tar.gz
sudo cp httpcg-client /usr/local/bin/
sudo chmod +x /usr/local/bin/httpcg-client

# Start service
sudo systemctl start httpcg-client

# Verify upgrade
sleep 10
httpcg-client --version
sudo systemctl status httpcg-client

echo "Upgrade to version ${NEW_VERSION} completed successfully"
```

## Next Steps

1. **[Troubleshooting Guide](./06-troubleshooting-and-monitoring.md)** - Advanced troubleshooting and monitoring
2. **[API Reference](./07-api-reference.md)** - Complete API documentation

## References

- **Configuration Examples**: `/home/umesh/metanode/bpi-core/cue_configs/httpcg.cue`
- **Service Integration**: `/home/umesh/metanode/bpi-core/src/bpi_vm_server.rs`
- **Security Implementation**: `/home/umesh/metanode/wallet-identity/src/client/transport/httpcg_client.rs`
- **Deployment Scripts**: Production deployment automation examples
