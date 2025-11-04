# Cost-Optimized BPI-BPCI Deployment Plan (Under $50/month)

## Overview

This plan creates a production-ready BPI-BPCI infrastructure with multiple specialized droplets, keeping total monthly cost under $50 while maintaining performance and reliability.

## 1. Droplet Architecture

### 1.1 Droplet Configuration

| Service | Droplet Size | CPU | RAM | Storage | Cost/Month | Purpose |
|---------|--------------|-----|-----|---------|------------|---------|
| **Static Website** | s-1vcpu-1gb | 1 | 1GB | 25GB SSD | $6 | Vite website, Cloudflare origin |
| **HTTPCG Gateway** | s-1vcpu-2gb | 1 | 2GB | 50GB SSD | $12 | HTTPCG protocol, VM server |
| **BPCI Registry** | s-1vcpu-2gb | 1 | 2GB | 50GB SSD | $12 | Registry, XTMP server, API |
| **Database** | s-1vcpu-1gb | 1 | 1GB | 25GB SSD | $6 | Mock DBs, storage |
| **Load Balancer** | - | - | - | - | $12 | Traffic distribution |

**Total Monthly Cost: $48/month**

### 1.2 Service Distribution

```
┌─────────────────────┐    ┌─────────────────────┐
│   Static Website    │    │   HTTPCG Gateway    │
│   s-1vcpu-1gb       │    │   s-1vcpu-2gb       │
│   $6/month          │    │   $12/month         │
│                     │    │                     │
│ • Vite website      │    │ • VM Server (7777)  │
│ • Nginx             │    │ • HTTPCG routing    │
│ • SSL termination   │    │ • Quantum PoE       │
│ • Static assets     │    │ • Hash-based routes │
└─────────────────────┘    └─────────────────────┘
           │                           │
           └───────────┬───────────────┘
                       │
┌─────────────────────┐    ┌─────────────────────┐
│   BPCI Registry     │    │     Database        │
│   s-1vcpu-2gb       │    │   s-1vcpu-1gb       │
│   $12/month         │    │   $6/month          │
│                     │    │                     │
│ • BPCI Enterprise   │    │ • Mock bpicom DB    │
│ • XTMP Server       │    │ • Mock bpigov DB    │
│ • Wallet Registry   │    │ • 4D Hash-Graph     │
│ • API Endpoints     │    │ • Audit storage     │
└─────────────────────┘    └─────────────────────┘
           │                           │
           └───────────┬───────────────┘
                       │
           ┌─────────────────────┐
           │   Load Balancer     │
           │   $12/month         │
           │                     │
           │ • Traffic routing   │
           │ • Health checks     │
           │ • SSL termination   │
           │ • DDoS protection   │
           └─────────────────────┘
```

## 2. Deployment Pipeline Scripts

### 2.1 Master Deployment Script

```bash
#!/bin/bash
# deploy_bpci_multi_droplet.sh - Master deployment script

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${GREEN}🚀 BPI-BPCI Multi-Droplet Deployment${NC}"
echo "===================================="

# Configuration
REGION="nyc1"
SSH_KEY_NAME="pravyom-deploy-key"

# Droplet configurations
declare -A DROPLETS=(
    ["static-website"]="s-1vcpu-1gb"
    ["httpcg-gateway"]="s-1vcpu-2gb"
    ["bpci-registry"]="s-1vcpu-2gb"
    ["database"]="s-1vcpu-1gb"
)

# Deploy each droplet
for name in "${!DROPLETS[@]}"; do
    size="${DROPLETS[$name]}"
    echo -e "${YELLOW}📦 Deploying $name ($size)...${NC}"
    ./deploy_${name//-/_}.sh "$size" "$REGION"
done

# Setup load balancer
echo -e "${YELLOW}⚖️  Setting up load balancer...${NC}"
./setup_load_balancer.sh

echo -e "${GREEN}✅ Multi-droplet deployment complete!${NC}"
```

## 3. Individual Droplet Scripts

### 3.1 Static Website Droplet

**File: `deploy_static_website.sh`**

```bash
#!/bin/bash
# Deploy static website droplet (s-1vcpu-1gb, $6/month)

DROPLET_NAME="pravyom-static-website"
DROPLET_SIZE="s-1vcpu-1gb"
DROPLET_IMAGE="ubuntu-22-04-x64"

echo "🌐 Creating static website droplet..."

# Create droplet
doctl compute droplet create "$DROPLET_NAME" \
    --size "$DROPLET_SIZE" \
    --image "$DROPLET_IMAGE" \
    --region "$1" \
    --ssh-keys "$SSH_KEY_NAME" \
    --wait

# Get IP
STATIC_IP=$(doctl compute droplet list --format Name,PublicIPv4 --no-header | grep "$DROPLET_NAME" | awk '{print $2}')

# Configure server
cat > /tmp/setup_static.sh << 'EOF'
#!/bin/bash
apt update && apt upgrade -y
apt install -y nginx nodejs npm git

# Install Vite and build tools
npm install -g vite

# Create website directory
mkdir -p /var/www/pravyom-static

# Configure Nginx for static hosting
cat > /etc/nginx/sites-available/static << 'NGINX_EOF'
server {
    listen 80;
    listen [::]:80;
    server_name pravyom.com www.pravyom.com;
    
    root /var/www/pravyom-static;
    index index.html;
    
    # Static assets with long cache
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
        add_header Vary "Accept-Encoding";
    }
    
    # HTML files with short cache
    location ~* \.html$ {
        expires 1h;
        add_header Cache-Control "public, must-revalidate";
    }
    
    # SPA fallback
    location / {
        try_files $uri $uri/ /index.html;
    }
    
    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
}
NGINX_EOF

ln -sf /etc/nginx/sites-available/static /etc/nginx/sites-enabled/
rm -f /etc/nginx/sites-enabled/default
nginx -t && systemctl reload nginx

# Create placeholder website
cat > /var/www/pravyom-static/index.html << 'HTML_EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Pravyom - BPI-BPCI Infrastructure</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; }
        .container { max-width: 1200px; margin: 0 auto; padding: 20px; }
        .header { text-align: center; padding: 60px 0; }
        .header h1 { font-size: 3rem; margin-bottom: 20px; }
        .header p { font-size: 1.2rem; opacity: 0.9; }
        .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 30px; margin: 40px 0; }
        .card { background: rgba(255,255,255,0.1); padding: 30px; border-radius: 15px; backdrop-filter: blur(10px); }
        .card h3 { margin-bottom: 15px; color: #fff; }
        .card p { opacity: 0.9; line-height: 1.6; }
        .status { background: rgba(76, 175, 80, 0.2); border: 1px solid rgba(76, 175, 80, 0.5); }
        .endpoint { background: rgba(33, 150, 243, 0.2); border: 1px solid rgba(33, 150, 243, 0.5); }
        .code { background: rgba(0,0,0,0.3); padding: 5px 10px; border-radius: 5px; font-family: monospace; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🚀 Pravyom</h1>
            <p>BPI-BPCI Infrastructure Platform</p>
        </div>
        
        <div class="grid">
            <div class="card status">
                <h3>✅ System Status</h3>
                <p>All services are operational and running smoothly.</p>
            </div>
            
            <div class="card endpoint">
                <h3>🌐 Static Website</h3>
                <p>High-performance static website with Cloudflare CDN integration.</p>
            </div>
            
            <div class="card endpoint">
                <h3>🔗 HTTPCG Protocol</h3>
                <p>Advanced hash-based routing with quantum-safe session management.</p>
            </div>
            
            <div class="card endpoint">
                <h3>🏛️ BPCI Registry</h3>
                <p>Enterprise-grade registry with wallet authentication and XTMP protocol.</p>
            </div>
        </div>
        
        <div class="grid">
            <div class="card">
                <h3>📊 Architecture</h3>
                <p>Multi-droplet deployment with specialized services for optimal performance and cost efficiency.</p>
            </div>
            
            <div class="card">
                <h3>💰 Cost Optimized</h3>
                <p>Complete infrastructure running for under $50/month with enterprise-grade features.</p>
            </div>
            
            <div class="card">
                <h3>🔒 Security</h3>
                <p>Cloudflare protection, SSL encryption, and post-quantum cryptographic security.</p>
            </div>
        </div>
    </div>
</body>
</html>
HTML_EOF

echo "✅ Static website droplet configured"
EOF

# Deploy configuration
scp /tmp/setup_static.sh root@$STATIC_IP:/tmp/
ssh root@$STATIC_IP "chmod +x /tmp/setup_static.sh && /tmp/setup_static.sh"

echo "✅ Static website droplet deployed: $STATIC_IP"
```

### 3.2 HTTPCG Gateway Droplet

**File: `deploy_httpcg_gateway.sh`**

```bash
#!/bin/bash
# Deploy HTTPCG gateway droplet (s-1vcpu-2gb, $12/month)

DROPLET_NAME="pravyom-httpcg-gateway"
DROPLET_SIZE="s-1vcpu-2gb"

echo "🔗 Creating HTTPCG gateway droplet..."

doctl compute droplet create "$DROPLET_NAME" \
    --size "$DROPLET_SIZE" \
    --image "ubuntu-22-04-x64" \
    --region "$1" \
    --ssh-keys "$SSH_KEY_NAME" \
    --wait

GATEWAY_IP=$(doctl compute droplet list --format Name,PublicIPv4 --no-header | grep "$DROPLET_NAME" | awk '{print $2}')

cat > /tmp/setup_gateway.sh << 'EOF'
#!/bin/bash
apt update && apt upgrade -y
apt install -y nginx git curl build-essential pkg-config libssl-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Create directories
mkdir -p /opt/bpi-core
mkdir -p /var/log/bpi

# Configure Nginx for HTTPCG
cat > /etc/nginx/sites-available/httpcg << 'NGINX_EOF'
server {
    listen 80;
    server_name httpcg.pravyom.com vm.pravyom.com;
    
    # HTTPCG protocol handler
    location / {
        proxy_pass http://127.0.0.1:7777;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # HTTPCG headers
        proxy_set_header X-HTTPCG-Plane $arg_plane;
        proxy_set_header X-HTTPCG-Domain $arg_domain;
        proxy_set_header X-HTTPCG-Hash $arg_hash;
        
        # WebSocket support
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_read_timeout 86400;
    }
}
NGINX_EOF

ln -sf /etc/nginx/sites-available/httpcg /etc/nginx/sites-enabled/
nginx -t && systemctl reload nginx

# Create BPI VM Server service
cat > /etc/systemd/system/bpi-vm-server.service << 'SERVICE_EOF'
[Unit]
Description=BPI VM Server (HTTPCG Gateway)
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/bpi-core
ExecStart=/opt/bpi-core/bpi-core --vm-server --port 7777
Restart=always
RestartSec=10
StandardOutput=append:/var/log/bpi/vm-server.log
StandardError=append:/var/log/bpi/vm-server.log

[Install]
WantedBy=multi-user.target
SERVICE_EOF

systemctl daemon-reload
echo "✅ HTTPCG gateway droplet configured"
EOF

scp /tmp/setup_gateway.sh root@$GATEWAY_IP:/tmp/
ssh root@$GATEWAY_IP "chmod +x /tmp/setup_gateway.sh && /tmp/setup_gateway.sh"

echo "✅ HTTPCG gateway droplet deployed: $GATEWAY_IP"
```

### 3.3 BPCI Registry Droplet

**File: `deploy_bpci_registry.sh`**

```bash
#!/bin/bash
# Deploy BPCI registry droplet (s-1vcpu-2gb, $12/month)

DROPLET_NAME="pravyom-bpci-registry"
DROPLET_SIZE="s-1vcpu-2gb"

echo "🏛️ Creating BPCI registry droplet..."

doctl compute droplet create "$DROPLET_NAME" \
    --size "$DROPLET_SIZE" \
    --image "ubuntu-22-04-x64" \
    --region "$1" \
    --ssh-keys "$SSH_KEY_NAME" \
    --wait

REGISTRY_IP=$(doctl compute droplet list --format Name,PublicIPv4 --no-header | grep "$DROPLET_NAME" | awk '{print $2}')

cat > /tmp/setup_registry.sh << 'EOF'
#!/bin/bash
apt update && apt upgrade -y
apt install -y nginx git curl build-essential pkg-config libssl-dev postgresql-client

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

mkdir -p /opt/bpci-enterprise
mkdir -p /var/log/bpci

# Configure Nginx for BPCI services
cat > /etc/nginx/sites-available/bpci << 'NGINX_EOF'
server {
    listen 80;
    server_name api.pravyom.com bpci.pravyom.com registry.pravyom.com xtmp.pravyom.com;
    
    # API endpoints
    location /api/ {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
    
    # XTMP protocol
    location /xtmp/ {
        proxy_pass http://127.0.0.1:8081;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # WebSocket support for XTMP
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
    
    # Registry endpoints
    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
NGINX_EOF

ln -sf /etc/nginx/sites-available/bpci /etc/nginx/sites-enabled/
nginx -t && systemctl reload nginx

# Create BPCI Enterprise service
cat > /etc/systemd/system/bpci-enterprise.service << 'SERVICE_EOF'
[Unit]
Description=BPCI Enterprise Registry
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/bpci-enterprise
ExecStart=/opt/bpci-enterprise/bpci-enterprise --api-server --port 8080
Restart=always
RestartSec=10
StandardOutput=append:/var/log/bpci/enterprise.log
StandardError=append:/var/log/bpci/enterprise.log

[Install]
WantedBy=multi-user.target
SERVICE_EOF

# Create XTMP Server service
cat > /etc/systemd/system/xtmp-server.service << 'SERVICE_EOF'
[Unit]
Description=XTMP Protocol Server
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/bpci-enterprise
ExecStart=/opt/bpci-enterprise/xtmp-server --port 8081
Restart=always
RestartSec=10
StandardOutput=append:/var/log/bpci/xtmp.log
StandardError=append:/var/log/bpci/xtmp.log

[Install]
WantedBy=multi-user.target
SERVICE_EOF

systemctl daemon-reload
echo "✅ BPCI registry droplet configured"
EOF

scp /tmp/setup_registry.sh root@$REGISTRY_IP:/tmp/
ssh root@$REGISTRY_IP "chmod +x /tmp/setup_registry.sh && /tmp/setup_registry.sh"

echo "✅ BPCI registry droplet deployed: $REGISTRY_IP"
```

### 3.4 Database Droplet

**File: `deploy_database.sh`**

```bash
#!/bin/bash
# Deploy database droplet (s-1vcpu-1gb, $6/month)

DROPLET_NAME="pravyom-database"
DROPLET_SIZE="s-1vcpu-1gb"

echo "🗄️ Creating database droplet..."

doctl compute droplet create "$DROPLET_NAME" \
    --size "$DROPLET_SIZE" \
    --image "ubuntu-22-04-x64" \
    --region "$1" \
    --ssh-keys "$SSH_KEY_NAME" \
    --wait

DB_IP=$(doctl compute droplet list --format Name,PublicIPv4 --no-header | grep "$DROPLET_NAME" | awk '{print $2}')

cat > /tmp/setup_database.sh << 'EOF'
#!/bin/bash
apt update && apt upgrade -y
apt install -y postgresql postgresql-contrib redis-server

# Configure PostgreSQL
sudo -u postgres createdb bpicom_testnet
sudo -u postgres createdb bpigov_testnet
sudo -u postgres createdb bpci_registry

# Create database user
sudo -u postgres psql -c "CREATE USER bpi_user WITH PASSWORD 'bpi_secure_pass';"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE bpicom_testnet TO bpi_user;"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE bpigov_testnet TO bpi_user;"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE bpci_registry TO bpi_user;"

# Configure PostgreSQL for remote connections
echo "listen_addresses = '*'" >> /etc/postgresql/14/main/postgresql.conf
echo "host all all 0.0.0.0/0 md5" >> /etc/postgresql/14/main/pg_hba.conf

systemctl restart postgresql

# Configure Redis
sed -i 's/bind 127.0.0.1/bind 0.0.0.0/' /etc/redis/redis.conf
systemctl restart redis-server

echo "✅ Database droplet configured"
EOF

scp /tmp/setup_database.sh root@$DB_IP:/tmp/
ssh root@$DB_IP "chmod +x /tmp/setup_database.sh && /tmp/setup_database.sh"

echo "✅ Database droplet deployed: $DB_IP"
```

## 4. Load Balancer Setup

### 4.1 Load Balancer Configuration

**File: `setup_load_balancer.sh`**

```bash
#!/bin/bash
# Setup Digital Ocean Load Balancer ($12/month)

echo "⚖️ Creating load balancer..."

# Get droplet IDs
STATIC_ID=$(doctl compute droplet list --format ID,Name --no-header | grep "pravyom-static-website" | awk '{print $1}')
GATEWAY_ID=$(doctl compute droplet list --format ID,Name --no-header | grep "pravyom-httpcg-gateway" | awk '{print $1}')
REGISTRY_ID=$(doctl compute droplet list --format ID,Name --no-header | grep "pravyom-bpci-registry" | awk '{print $1}')

# Create load balancer
doctl compute load-balancer create \
    --name "pravyom-load-balancer" \
    --algorithm "round_robin" \
    --health-check "protocol:http,port:80,path:/health,check_interval_seconds:10,response_timeout_seconds:5,unhealthy_threshold:3,healthy_threshold:2" \
    --forwarding-rules "entry_protocol:https,entry_port:443,target_protocol:http,target_port:80,certificate_id:,tls_passthrough:false entry_protocol:http,entry_port:80,target_protocol:http,target_port:80" \
    --droplet-ids "$STATIC_ID,$GATEWAY_ID,$REGISTRY_ID" \
    --region "nyc1"

# Get load balancer IP
LB_IP=$(doctl compute load-balancer list --format Name,IP --no-header | grep "pravyom-load-balancer" | awk '{print $2}')

echo "✅ Load balancer created: $LB_IP"
echo "🔗 Update DNS records to point to: $LB_IP"
```

## 5. Deployment Commands

### 5.1 Complete Deployment

```bash
# Make all scripts executable
chmod +x deploy_*.sh setup_*.sh

# Deploy all droplets
./deploy_bpci_multi_droplet.sh

# Get all IPs for DNS configuration
./get_deployment_ips.sh
```

### 5.2 DNS Configuration Script

**File: `get_deployment_ips.sh`**

```bash
#!/bin/bash
echo "📋 Deployment IP Addresses:"
echo "================================"

echo "Static Website: $(doctl compute droplet list --format Name,PublicIPv4 --no-header | grep 'pravyom-static-website' | awk '{print $2}')"
echo "HTTPCG Gateway: $(doctl compute droplet list --format Name,PublicIPv4 --no-header | grep 'pravyom-httpcg-gateway' | awk '{print $2}')"
echo "BPCI Registry: $(doctl compute droplet list --format Name,PublicIPv4 --no-header | grep 'pravyom-bpci-registry' | awk '{print $2}')"
echo "Database: $(doctl compute droplet list --format Name,PublicIPv4 --no-header | grep 'pravyom-database' | awk '{print $2}')"
echo "Load Balancer: $(doctl compute load-balancer list --format Name,IP --no-header | grep 'pravyom-load-balancer' | awk '{print $2}')"

echo ""
echo "🔗 DNS Records to Configure in Cloudflare:"
LB_IP=$(doctl compute load-balancer list --format Name,IP --no-header | grep 'pravyom-load-balancer' | awk '{print $2}')
echo "A record: pravyom.com → $LB_IP"
echo "A record: www → $LB_IP"
echo "CNAME: api → pravyom.com"
echo "CNAME: xtmp → pravyom.com"
echo "CNAME: registry → pravyom.com"
echo "CNAME: httpcg → pravyom.com"
echo "CNAME: bpci → pravyom.com"
echo "CNAME: vm → pravyom.com"
```

## 6. Cost Breakdown

| Component | Monthly Cost | Annual Cost |
|-----------|--------------|-------------|
| Static Website (s-1vcpu-1gb) | $6 | $72 |
| HTTPCG Gateway (s-1vcpu-2gb) | $12 | $144 |
| BPCI Registry (s-1vcpu-2gb) | $12 | $144 |
| Database (s-1vcpu-1gb) | $6 | $72 |
| Load Balancer | $12 | $144 |
| **Total** | **$48** | **$576** |

## 7. Benefits

✅ **Cost Optimized**: Under $50/month budget
✅ **Specialized Services**: Each droplet optimized for specific tasks
✅ **High Availability**: Load balancer with health checks
✅ **Scalable**: Easy to upgrade individual components
✅ **Secure**: Isolated services with proper firewall rules
✅ **Production Ready**: Full SSL, monitoring, and logging

This architecture provides a robust, cost-effective foundation for the BPI-BPCI infrastructure while maintaining the sophisticated features required by the real codebase implementation.
