#!/bin/bash
# BPCI Server Installer - Main hosted server/bridge for BPCI ecosystem
# Version: 1.0.0
# Purpose: Central server coordinating community, BPI, registry, and all services

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# Configuration
SERVER_VERSION="1.0.0"
INSTALL_DIR="/opt/bpci-server"
BIN_DIR="/usr/local/bin"
CONFIG_DIR="/etc/bpci-server"
SERVICE_DIR="/etc/systemd/system"
USER="bpci-server"
WEB_DIR="/var/www/bpci"

echo -e "${CYAN}🌐 BPCI Server Installer v${SERVER_VERSION}${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo "Purpose: Central BPCI ecosystem server/bridge"
echo "Target: Hosted server coordinating all services"
echo ""

# Check root permissions
if [[ $EUID -ne 0 ]]; then
    echo "❌ Must run as root: sudo $0"
    exit 1
fi

# Single-deployment protection
check_existing_installation() {
    echo -e "${YELLOW}🔒 Checking for existing installation...${NC}"
    
    if [[ -f "$INSTALL_DIR/bin/bpci-server" ]] || [[ -f "$CONFIG_DIR/server.toml" ]] || systemctl is-active --quiet bpci-server 2>/dev/null; then
        echo -e "${RED}❌ BPCI Server already installed!${NC}"
        echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo "Only ONE BPCI Server can exist per ecosystem."
        echo "If you need to:"
        echo "• Manage server: Use ./server-management.sh"
        echo "• Connect nodes: Use community/enterprise installers"
        echo "• Reinstall: Contact system administrator"
        echo ""
        exit 1
    fi
    
    echo -e "${GREEN}✅ No existing installation found${NC}"
}

# Check system requirements
check_requirements() {
    echo -e "${YELLOW}🔍 Checking server requirements...${NC}"
    
    # Check OS
    if [[ "$OSTYPE" != "linux-gnu"* ]]; then
        echo -e "${RED}❌ Linux required for server deployment${NC}"
        exit 1
    fi
    
    # Check memory (minimum 4GB for server)
    MEMORY_GB=$(free -g | awk '/^Mem:/{print $2}')
    if [[ $MEMORY_GB -lt 4 ]]; then
        echo -e "${RED}❌ Minimum 4GB RAM required (found ${MEMORY_GB}GB)${NC}"
        exit 1
    fi
    
    # Check disk space (minimum 50GB)
    DISK_GB=$(df / | awk 'NR==2{print int($4/1024/1024)}')
    if [[ $DISK_GB -lt 50 ]]; then
        echo -e "${RED}❌ Minimum 50GB free space required${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ Server requirements met${NC}"
}

# Install dependencies
install_dependencies() {
    echo -e "${YELLOW}📦 Installing server dependencies...${NC}"
    
    apt-get update -qq
    apt-get install -y \
        curl \
        git \
        build-essential \
        libssl-dev \
        pkg-config \
        systemd \
        nginx \
        ufw \
        fail2ban \
        logrotate \
        htop \
        postgresql \
        redis-server
    
    # Install Rust
    if ! command -v rustc &> /dev/null; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    echo -e "${GREEN}✅ Dependencies installed${NC}"
}

# Create server user
create_server_user() {
    echo -e "${YELLOW}👤 Creating BPCI server user...${NC}"
    
    if ! id "$USER" &>/dev/null; then
        useradd --system --home "$INSTALL_DIR" --shell /bin/bash "$USER"
        usermod -aG systemd-journal "$USER"
        usermod -aG www-data "$USER"
    fi
    
    echo -e "${GREEN}✅ Server user created${NC}"
}

# Setup directories
setup_directories() {
    echo -e "${YELLOW}📁 Setting up server directories...${NC}"
    
    mkdir -p "$INSTALL_DIR"/{bin,data,logs,config,keys,backup}
    mkdir -p "$CONFIG_DIR"
    mkdir -p "$WEB_DIR"/{api,dashboard,docs}
    mkdir -p "/var/log/bpci-server"
    
    # Set permissions
    chown -R "$USER:$USER" "$INSTALL_DIR"
    chown -R "$USER:www-data" "$WEB_DIR"
    chmod 750 "$INSTALL_DIR"
    chmod 700 "$INSTALL_DIR/keys"
    chmod 755 "$WEB_DIR"
    
    echo -e "${GREEN}✅ Directories configured${NC}"
}

# Install BPCI Server
install_bpci_server() {
    echo -e "${YELLOW}⚡ Installing BPCI Server...${NC}"
    
    # Use local pravyom repository
    PRAVYOM_ROOT="$(dirname "$(dirname "$(dirname "$(realpath "$0")")")")"
    cd "$PRAVYOM_ROOT"
    
    # Build server components
    cargo build --release -p pravyom-enterprise
    
    # Install binaries
    cp target/release/pravyom-enterprise "$INSTALL_DIR/bin/pravyom-server"
    ln -sf "$INSTALL_DIR/bin/bpci-server" "$BIN_DIR/bpci-server"
    
    # Set permissions
    chown "$USER:$USER" "$INSTALL_DIR/bin/bpci-server"
    chmod 755 "$INSTALL_DIR/bin/bpci-server"
    
    echo -e "${GREEN}✅ BPCI Server installed${NC}"
}

# Setup server configuration
setup_server_config() {
    echo -e "${YELLOW}⚙️ Setting up server configuration...${NC}"
    
    cat > "$CONFIG_DIR/server.toml" << EOF
[server]
mode = "production"
bind_address = "0.0.0.0"
rpc_port = 8545
ws_port = 8546
api_port = 3000
dashboard_port = 8080

[network]
network = "mainnet"
p2p_port = 30303
discovery_enabled = true
bootstrap_nodes = []

[database]
postgres_url = "postgresql://bpci:bpci@localhost/bpci"
redis_url = "redis://localhost:6379"

[registry]
enabled = true
authority_level = "central"
node_registration = true
identity_verification = true

[services]
wallet_service = true
mining_service = true
governance_service = true
notary_service = true
bridge_service = true

[security]
level = "production"
tls_enabled = true
rate_limiting = true
ddos_protection = true

[monitoring]
metrics_enabled = true
health_checks = true
log_level = "info"
EOF

    chown "$USER:$USER" "$CONFIG_DIR/server.toml"
    chmod 640 "$CONFIG_DIR/server.toml"
    
    echo -e "${GREEN}✅ Server configuration created${NC}"
}

# Setup systemd services
setup_services() {
    echo -e "${YELLOW}🔧 Setting up systemd services...${NC}"
    
    # Main BPCI Server service
    cat > "$SERVICE_DIR/bpci-server.service" << EOF
[Unit]
Description=BPCI Server - Main ecosystem bridge
After=network.target postgresql.service redis.service
Requires=postgresql.service redis.service

[Service]
Type=simple
User=$USER
Group=$USER
WorkingDirectory=$INSTALL_DIR
ExecStart=$INSTALL_DIR/bin/bpci-server --config $CONFIG_DIR/server.toml --mode server
ExecReload=/bin/kill -HUP \$MAINPID
Restart=always
RestartSec=10
LimitNOFILE=65536

# Security
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=$INSTALL_DIR $CONFIG_DIR /var/log/bpci-server

[Install]
WantedBy=multi-user.target
EOF

    # API Gateway service
    cat > "$SERVICE_DIR/bpci-api.service" << EOF
[Unit]
Description=BPCI API Gateway
After=bpci-server.service
Requires=bpci-server.service

[Service]
Type=simple
User=$USER
Group=$USER
WorkingDirectory=$INSTALL_DIR
ExecStart=$INSTALL_DIR/bin/bpci-server --config $CONFIG_DIR/server.toml --mode api
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

    echo -e "${GREEN}✅ Systemd services configured${NC}"
}

# Configure nginx
setup_nginx() {
    echo -e "${YELLOW}🌐 Configuring nginx reverse proxy...${NC}"
    
    cat > "$NGINX_DIR/bpci-server" << EOF
server {
    listen 80;
    server_name _;
    
    # API endpoints
    location /api/ {
        proxy_pass http://localhost:3000/;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }
    
    # RPC endpoint
    location /rpc {
        proxy_pass http://localhost:8545;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
    }
    
    # WebSocket endpoint
    location /ws {
        proxy_pass http://localhost:8546;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection "upgrade";
    }
    
    # Dashboard
    location /dashboard/ {
        proxy_pass http://localhost:8080/;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
    }
    
    # Static files
    location / {
        root $WEB_DIR;
        try_files \$uri \$uri/ =404;
    }
}
EOF

    ln -sf "$NGINX_DIR/bpci-server" "/etc/nginx/sites-enabled/"
    rm -f "/etc/nginx/sites-enabled/default"
    
    echo -e "${GREEN}✅ Nginx configured${NC}"
}

# Configure firewall
setup_firewall() {
    echo -e "${YELLOW}🔥 Configuring server firewall...${NC}"
    
    ufw --force enable
    ufw allow ssh
    ufw allow 80/tcp comment "HTTP"
    ufw allow 443/tcp comment "HTTPS"
    ufw allow 30303/tcp comment "BPCI P2P"
    
    echo -e "${GREEN}✅ Firewall configured${NC}"
}

# Setup database
setup_database() {
    echo -e "${YELLOW}🗄️ Setting up database...${NC}"
    
    # Configure PostgreSQL
    sudo -u postgres createuser bpci || true
    sudo -u postgres createdb bpci -O bpci || true
    sudo -u postgres psql -c "ALTER USER bpci PASSWORD 'bpci';" || true
    
    # Start services
    systemctl enable postgresql redis-server
    systemctl start postgresql redis-server
    
    echo -e "${GREEN}✅ Database configured${NC}"
}

# Main installation function
main() {
    echo -e "${BLUE}🚀 Starting BPCI Server Installation...${NC}"
    echo ""
    
    check_existing_installation
    check_requirements
    install_dependencies
    create_server_user
    setup_directories
    install_bpci_server
    setup_server_config
    setup_services
    setup_nginx
    setup_firewall
    setup_database
    
    # Enable and start services
    systemctl daemon-reload
    systemctl enable bpci-server.service bpci-api.service nginx
    systemctl start bpci-server.service bpci-api.service nginx
    
    echo ""
    echo -e "${GREEN}🎉 BPCI Server Installation Complete!${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo -e "${BLUE}🌐 Server Endpoints:${NC}"
    echo "• API: http://localhost/api/"
    echo "• RPC: http://localhost/rpc"
    echo "• WebSocket: ws://localhost/ws"
    echo "• Dashboard: http://localhost/dashboard/"
    echo ""
    echo -e "${BLUE}🔧 Management Commands:${NC}"
    echo "• Status: systemctl status bpci-server"
    echo "• Logs: journalctl -u bpci-server -f"
    echo "• CLI: bpci-server status"
    echo ""
    echo -e "${BLUE}📊 Services Running:${NC}"
    echo "• BPCI Server: Main ecosystem bridge"
    echo "• API Gateway: REST/GraphQL endpoints"
    echo "• Registry: Node/identity management"
    echo "• Bridge: Community/BPI coordination"
    echo ""
    echo -e "${CYAN}🚀 Server is ready to coordinate the BPCI ecosystem!${NC}"
}

# Run installation
main "$@"
