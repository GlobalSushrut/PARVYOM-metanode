#!/bin/bash
# BSO-K8 Controller Deployment Script
# Deploys the BSO-K8 orchestrator on current infrastructure

set -e

echo "🚀 Starting BSO-K8 Controller Deployment..."

# Create BSO-K8 configuration directory
sudo mkdir -p /etc/bso-k8
sudo mkdir -p /var/log/bso-k8
sudo mkdir -p /var/lib/bso-k8

# Create BSO-K8 controller configuration
cat > /tmp/controller.toml << EOF
[orchestrator]
id = "bso-k8-main-controller"
listen_port = 9090
listen_host = "0.0.0.0"
vpod_arena_size = 2048
max_vpods = 1000
log_level = "info"

[resources]
memory_limit_mb = 1500
cpu_limit_cores = 4.0
storage_limit_gb = 100
hugepages_enabled = false

[networking]
cluster_cidr = "10.244.0.0/16"
service_cidr = "10.96.0.0/12"
dns_domain = "cluster.local"
enable_load_balancer = true

[storage]
data_dir = "/var/lib/bso-k8"
log_dir = "/var/log/bso-k8"
backup_enabled = true

[monitoring]
metrics_enabled = true
metrics_port = 9091
health_check_port = 9092
prometheus_enabled = true
EOF

sudo mv /tmp/controller.toml /etc/bso-k8/controller.toml

# Build BSO-K8 controller
echo "📦 Building BSO-K8 Controller..."
cd /home/umesh/metanode/bpci-enterprise
cargo build --release --bin bso_k8_orchestrator

# Create systemd service
cat > /tmp/bso-k8-controller.service << EOF
[Unit]
Description=BSO-K8 Orchestrator Controller
After=network.target
Wants=network.target

[Service]
Type=simple
User=root
Group=root
ExecStart=/home/umesh/metanode/bpci-enterprise/target/release/bso_k8_orchestrator --config /etc/bso-k8/controller.toml
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=bso-k8-controller

# Resource limits
MemoryLimit=2G
CPUQuota=400%

# Security
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/bso-k8 /var/log/bso-k8

[Install]
WantedBy=multi-user.target
EOF

sudo mv /tmp/bso-k8-controller.service /etc/systemd/system/
sudo systemctl daemon-reload

# Create BSO-K8 user and set permissions
sudo useradd -r -s /bin/false bso-k8 || true
sudo chown -R bso-k8:bso-k8 /var/lib/bso-k8 /var/log/bso-k8
sudo chmod 755 /var/lib/bso-k8 /var/log/bso-k8

# Start BSO-K8 controller
echo "🎯 Starting BSO-K8 Controller..."
sudo systemctl enable bso-k8-controller
sudo systemctl start bso-k8-controller

# Wait for controller to start
sleep 10

# Verify controller is running
if curl -f http://localhost:9090/api/v1/health > /dev/null 2>&1; then
    echo "✅ BSO-K8 Controller started successfully!"
    echo "📊 Controller Status:"
    curl -s http://localhost:9090/api/v1/status | jq '.'
else
    echo "❌ BSO-K8 Controller failed to start"
    echo "📋 Service logs:"
    sudo journalctl -u bso-k8-controller --no-pager -n 20
    exit 1
fi

# Configure firewall
echo "🔥 Configuring firewall..."
sudo ufw allow 9090/tcp comment "BSO-K8 Controller API"
sudo ufw allow 9091/tcp comment "BSO-K8 Metrics"
sudo ufw allow 9092/tcp comment "BSO-K8 Health Check"

# Create monitoring script
cat > /tmp/bso-k8-monitor.sh << 'EOF'
#!/bin/bash
# BSO-K8 Monitoring Script

echo "=== BSO-K8 Controller Status ==="
systemctl status bso-k8-controller --no-pager -l

echo -e "\n=== BSO-K8 API Health ==="
curl -s http://localhost:9090/api/v1/health | jq '.'

echo -e "\n=== BSO-K8 Cluster Status ==="
curl -s http://localhost:9090/api/v1/cluster/status | jq '.'

echo -e "\n=== vPod Statistics ==="
curl -s http://localhost:9090/api/v1/vpods/stats | jq '.'

echo -e "\n=== Resource Usage ==="
curl -s http://localhost:9090/api/v1/resources/usage | jq '.'
EOF

sudo mv /tmp/bso-k8-monitor.sh /usr/local/bin/bso-k8-monitor
sudo chmod +x /usr/local/bin/bso-k8-monitor

echo "🎉 BSO-K8 Controller deployment completed!"
echo "📋 Next steps:"
echo "  1. Run 'bso-k8-monitor' to check status"
echo "  2. Deploy services using: ./scripts/deploy-stage-1-keycloak.sh"
echo "  3. Monitor logs: sudo journalctl -u bso-k8-controller -f"
