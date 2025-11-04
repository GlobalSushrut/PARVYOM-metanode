#!/bin/bash
# BPCI Enterprise - Phase 2: Infrastructure Installation
# Server: 134.209.210.181
# Date: 2025-10-30
# NO DOCKER - Native installations only

set -e  # Exit on error

echo "=========================================="
echo "BPCI ENTERPRISE - PHASE 2: INFRASTRUCTURE"
echo "=========================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

export DEBIAN_FRONTEND=noninteractive

# Step 1: Install Nginx
log_info "Step 1: Installing Nginx..."
apt-get install -y nginx
systemctl enable nginx
systemctl start nginx
log_success "Nginx installed and started"

# Step 2: Install PostgreSQL
log_info "Step 2: Installing PostgreSQL..."
apt-get install -y postgresql postgresql-contrib
systemctl enable postgresql
systemctl start postgresql
log_success "PostgreSQL installed and started"

# Step 3: Configure PostgreSQL
log_info "Step 3: Configuring PostgreSQL..."

# Create databases and users
sudo -u postgres psql << EOF
-- Create keycloak database and user
CREATE DATABASE keycloak;
CREATE USER keycloak WITH ENCRYPTED PASSWORD 'keycloak_secure_password_2024';
GRANT ALL PRIVILEGES ON DATABASE keycloak TO keycloak;

-- Create BPCI databases
CREATE DATABASE bpci_users;
CREATE DATABASE bpci_blockchain;
CREATE DATABASE bpci_registry;

-- Create BPCI user
CREATE USER bpci WITH ENCRYPTED PASSWORD 'bpci_secure_password_2024';
GRANT ALL PRIVILEGES ON DATABASE bpci_users TO bpci;
GRANT ALL PRIVILEGES ON DATABASE bpci_blockchain TO bpci;
GRANT ALL PRIVILEGES ON DATABASE bpci_registry TO bpci;

-- List databases
\l
EOF

log_success "PostgreSQL databases configured"

# Step 4: Install Redis
log_info "Step 4: Installing Redis..."
apt-get install -y redis-server
systemctl enable redis-server
systemctl start redis-server

# Configure Redis
cat > /etc/redis/redis.conf.d/bpci.conf << EOF
# BPCI Redis Configuration
maxmemory 2gb
maxmemory-policy allkeys-lru
save 900 1
save 300 10
save 60 10000
EOF

systemctl restart redis-server
log_success "Redis installed and configured"

# Step 5: Install Java (required for Keycloak)
log_info "Step 5: Installing Java for Keycloak..."
apt-get install -y openjdk-17-jdk
log_success "Java $(java -version 2>&1 | head -1) installed"

# Step 6: Download and Install Keycloak
log_info "Step 6: Installing Keycloak..."

KEYCLOAK_VERSION="23.0.1"
cd /opt

if [ ! -d "keycloak-${KEYCLOAK_VERSION}" ]; then
    log_info "Downloading Keycloak ${KEYCLOAK_VERSION}..."
    wget -q https://github.com/keycloak/keycloak/releases/download/${KEYCLOAK_VERSION}/keycloak-${KEYCLOAK_VERSION}.tar.gz
    tar -xzf keycloak-${KEYCLOAK_VERSION}.tar.gz
    rm keycloak-${KEYCLOAK_VERSION}.tar.gz
    ln -sf keycloak-${KEYCLOAK_VERSION} keycloak
    log_success "Keycloak downloaded and extracted"
else
    log_success "Keycloak already exists"
fi

# Step 7: Configure Keycloak
log_info "Step 7: Configuring Keycloak..."

# Create Keycloak user
if ! id -u keycloak &> /dev/null; then
    useradd -r -s /bin/false keycloak
fi

chown -R keycloak:keycloak /opt/keycloak-${KEYCLOAK_VERSION}

# Configure Keycloak database connection
cat > /opt/keycloak/conf/keycloak.conf << EOF
# Database configuration
db=postgres
db-url=jdbc:postgresql://localhost:5432/keycloak
db-username=keycloak
db-password=keycloak_secure_password_2024

# HTTP configuration
http-enabled=true
http-port=8180
hostname=localhost

# Admin user
# Will be created on first start
EOF

log_success "Keycloak configured"

# Step 8: Create Keycloak systemd service
log_info "Step 8: Creating Keycloak systemd service..."

cat > /etc/systemd/system/keycloak.service << EOF
[Unit]
Description=Keycloak Application Server
After=network.target postgresql.service

[Service]
Type=idle
User=keycloak
Group=keycloak
ExecStart=/opt/keycloak/bin/kc.sh start --optimized
Restart=on-failure
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
log_success "Keycloak systemd service created"

# Step 9: Build Keycloak (optimize for production)
log_info "Step 9: Building Keycloak for production..."
cd /opt/keycloak
sudo -u keycloak bin/kc.sh build
log_success "Keycloak built successfully"

# Step 10: Start Keycloak
log_info "Step 10: Starting Keycloak..."
systemctl enable keycloak
systemctl start keycloak

log_info "Waiting for Keycloak to start (this may take 60-90 seconds)..."
sleep 30

# Check if Keycloak is running
if systemctl is-active --quiet keycloak; then
    log_success "Keycloak is running"
else
    log_warning "Keycloak may still be starting, check with: systemctl status keycloak"
fi

# Step 11: Configure Nginx as reverse proxy
log_info "Step 11: Configuring Nginx..."

cat > /etc/nginx/sites-available/bpci << 'EOF'
# BPCI Enterprise Nginx Configuration

# Upstream definitions
upstream keycloak {
    server localhost:8180;
}

upstream bpci_blockchain {
    server localhost:8080;
}

upstream bpci_xtmp {
    server localhost:8889;
}

# Main server block
server {
    listen 80;
    server_name _;

    # Increase buffer sizes for large headers
    client_max_body_size 100M;
    proxy_buffer_size 128k;
    proxy_buffers 4 256k;
    proxy_busy_buffers_size 256k;

    # Frontend (will be added in Phase 4)
    location / {
        root /var/www/bpci-frontend;
        try_files $uri $uri/ /index.html;
    }

    # Keycloak authentication
    location /auth/ {
        proxy_pass http://keycloak/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # BPCI API
    location /api/ {
        proxy_pass http://bpci_blockchain/api/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }

    # WebSocket for real-time updates
    location /ws/ {
        proxy_pass http://bpci_xtmp/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    # Health check endpoint
    location /health {
        access_log off;
        return 200 "healthy\n";
        add_header Content-Type text/plain;
    }
}
EOF

# Enable the site
ln -sf /etc/nginx/sites-available/bpci /etc/nginx/sites-enabled/
rm -f /etc/nginx/sites-enabled/default

# Test nginx configuration
nginx -t
systemctl reload nginx

log_success "Nginx configured as reverse proxy"

# Step 12: System Information
log_info "Step 12: Gathering service status..."
echo ""
echo "=========================================="
echo "SERVICE STATUS"
echo "=========================================="
echo ""

echo "Nginx:"
systemctl status nginx --no-pager | head -3
echo ""

echo "PostgreSQL:"
systemctl status postgresql --no-pager | head -3
echo ""

echo "Redis:"
systemctl status redis-server --no-pager | head -3
echo ""

echo "Keycloak:"
systemctl status keycloak --no-pager | head -3
echo ""

echo "=========================================="
echo "DATABASE STATUS"
echo "=========================================="
sudo -u postgres psql -c "\l" | grep -E "Name|bpci|keycloak"
echo ""

echo "=========================================="
echo "LISTENING PORTS"
echo "=========================================="
ss -tlnp | grep -E "80|5432|6379|8180" || echo "Checking ports..."
echo ""

echo ""
echo "=========================================="
echo "PHASE 2 COMPLETE!"
echo "=========================================="
echo ""
log_success "Infrastructure installation completed successfully"
echo ""
echo "Installed Services:"
echo "  ✅ Nginx (Port 80) - Web server & reverse proxy"
echo "  ✅ PostgreSQL (Port 5432) - Database"
echo "  ✅ Redis (Port 6379) - Cache"
echo "  ✅ Keycloak (Port 8180) - Authentication"
echo ""
echo "Next steps:"
echo "  1. Run deploy_phase3_bpci_backend.sh"
echo "  2. Build and deploy all 11 BPCI servers"
echo "  3. Configure CommuteLock communication"
echo ""
