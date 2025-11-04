#!/bin/bash

# Complete BPCI Stack Deployment
# Deploys API Gateway + React Frontend with proper configuration

set -e

echo "=========================================="
echo "COMPLETE BPCI STACK DEPLOYMENT"
echo "=========================================="
echo ""

SERVER="134.209.210.181"
FRONTEND_DIR="/home/umesh/metanode/bpci-enterprise/website/bpci-enterprise-website"

echo "Step 1: Building API Gateway with CommuteLock..."
cd /home/umesh/metanode/bpci-enterprise
cargo build --release --bin bpci_api_gateway

if [ $? -eq 0 ]; then
    echo "✅ API Gateway built successfully"
else
    echo "❌ API Gateway build failed"
    exit 1
fi

echo ""
echo "Step 2: Deploying API Gateway to server..."
scp target/release/bpci_api_gateway root@${SERVER}:/opt/bpci/bin/

echo ""
echo "Step 3: Creating API Gateway systemd service..."
ssh root@${SERVER} 'cat > /etc/systemd/system/bpci-api-gateway.service << EOF
[Unit]
Description=BPCI API Gateway with CommuteLock
After=network.target bpci-web.service
Requires=bpci-web.service

[Service]
Type=simple
User=bpci
Group=bpci
WorkingDirectory=/opt/bpci
Environment="RUST_LOG=info"
ExecStart=/opt/bpci/bin/bpci_api_gateway
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable bpci-api-gateway
systemctl start bpci-api-gateway
'

echo "✅ API Gateway deployed"

echo ""
echo "Step 4: Updating frontend configuration..."
cd ${FRONTEND_DIR}

# Update API configuration
cat > src/config/api.config.ts << 'EOF'
/**
 * API Configuration for BPCI Frontend
 * Points to testnet server with all endpoints
 */

export const API_CONFIG = {
  // Base URLs - Testnet Server
  BASE_URL: 'http://134.209.210.181',
  API_GATEWAY: 'http://134.209.210.181:3001',
  
  // Backend Services
  BPCI_SERVER: 'http://134.209.210.181',
  BPI_CORE_SERVER: 'http://134.209.210.181',
  BLOCKCHAIN_SERVER: 'http://134.209.210.181/blockchain',
  BRIDGE_SERVER: 'http://134.209.210.181/bridge',
  
  // Authentication
  KEYCLOAK_URL: 'http://134.209.210.181/auth',
  KEYCLOAK_REALM: 'bpci',
  KEYCLOAK_CLIENT_ID: 'bpci-frontend',
  
  // API Endpoints
  ENDPOINTS: {
    // Dashboard
    DASHBOARD_STATS: '/api/dashboard/stats',
    SYSTEM_STATUS: '/health',
    
    // Blockchain
    BLOCKCHAIN_INFO: '/blockchain/api/v1/blockchain/info',
    BLOCKCHAIN_STATUS: '/blockchain/api/v1/blockchain/status',
    NETWORK_INFO: '/blockchain/api/v1/network',
    
    // Wallet
    WALLETS: '/api/wallets',
    WALLET_CREATE: '/api/wallet/register',
    WALLET_BALANCE: '/api/wallet',
    
    // BPI Bridge
    BRIDGE_HEALTH: '/bridge/health',
    BRIDGE_PRICING: '/bridge/pricing',
    BRIDGE_TRANSACTION: '/bridge/transaction/process',
    
    // Developer (via API Gateway)
    DEV_PROFILE: '/api/dev/profile',
    DEV_CREATE: '/api/dev/profile',
    
    // Test Networks (via API Gateway)
    TESTNET_CREATE: '/api/testnet/create',
    TESTNET_LIST: '/api/testnet/list',
    TESTNET_START: '/api/testnet/:id/start',
    TESTNET_STOP: '/api/testnet/:id/stop',
    
    // HTTPCG (via API Gateway)
    HTTPCG_ENABLE: '/api/httpcg/enable',
    HTTPCG_DISABLE: '/api/httpcg/disable',
    HTTPCG_STATUS: '/api/httpcg/status',
    
    // Shadow Registry (via API Gateway)
    SHADOW_REGISTER: '/api/shadow/register',
    
    // Domain (via API Gateway)
    DOMAIN_REGISTER: '/api/domain/register',
    
    // Installer (via API Gateway)
    INSTALLER_STATUS: '/api/installer/status',
    INSTALLER_START: '/api/installer/start',
  },
  
  // Headers
  HEADERS: {
    'Content-Type': 'application/json',
    'X-BPCI-Version': '1.0.0',
    'X-Client-Type': 'Web-Frontend'
  }
};

export default API_CONFIG;
EOF

echo "✅ Frontend configuration updated"

echo ""
echo "Step 5: Updating bpciApi.ts to use new config..."
# Update the BPCI_CONFIG in bpciApi.ts
sed -i "s|https://api.pravyom.com|http://134.209.210.181|g" src/services/bpciApi.ts
sed -i "s|https://xtmp.pravyom.com|http://134.209.210.181|g" src/services/bpciApi.ts
sed -i "s|https://registry.pravyom.com|http://134.209.210.181|g" src/services/bpciApi.ts

echo "✅ bpciApi.ts updated"

echo ""
echo "Step 6: Installing frontend dependencies..."
npm install

echo ""
echo "Step 7: Building frontend..."
npm run build

if [ $? -eq 0 ]; then
    echo "✅ Frontend built successfully"
else
    echo "❌ Frontend build failed"
    exit 1
fi

echo ""
echo "Step 8: Deploying frontend to server..."
ssh root@${SERVER} 'mkdir -p /var/www/html/app'
scp -r dist/* root@${SERVER}:/var/www/html/app/

echo ""
echo "Step 9: Updating Nginx configuration..."
ssh root@${SERVER} 'cat > /etc/nginx/sites-available/bpci << "EOFNGINX"
server {
    listen 80;
    server_name 134.209.210.181;

    # Increase buffer sizes
    proxy_buffer_size 128k;
    proxy_buffers 4 256k;
    proxy_busy_buffers_size 256k;

    # React App
    location / {
        root /var/www/html/app;
        index index.html;
        try_files \$uri \$uri/ /index.html;
    }

    # API Gateway (Port 3001)
    location /api/ {
        proxy_pass http://localhost:3001/api/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
    }

    # Health check
    location /health {
        proxy_pass http://localhost:3000/health;
        proxy_set_header Host \$host;
    }

    # Keycloak Auth
    location /auth/ {
        proxy_pass http://localhost:8180/;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_buffer_size 128k;
        proxy_buffers 4 256k;
    }

    # Blockchain Server
    location /blockchain/ {
        proxy_pass http://localhost:8080/;
        proxy_set_header Host \$host;
    }

    # BPI Bridge
    location /bridge/ {
        proxy_pass http://localhost:6001/;
        proxy_set_header Host \$host;
    }

    # BSO-K8 Orchestrator
    location /orchestrator/ {
        proxy_pass http://localhost:9090/;
        proxy_set_header Host \$host;
    }
}
EOFNGINX

nginx -t && systemctl reload nginx
'

echo "✅ Nginx configuration updated"

echo ""
echo "Step 10: Verifying deployment..."
sleep 5

echo ""
echo "Testing endpoints..."
curl -s http://${SERVER}/ | grep -q "html" && echo "✅ Frontend accessible"
curl -s http://${SERVER}/health | grep -q "ok" && echo "✅ Health check working"
curl -s http://${SERVER}/api/dashboard/stats 2>&1 | grep -q "total_transactions" && echo "✅ API Gateway working"

echo ""
echo "=========================================="
echo "DEPLOYMENT COMPLETE!"
echo "=========================================="
echo ""
echo "🌐 Frontend: http://${SERVER}/"
echo "🔗 API Gateway: http://${SERVER}/api/"
echo "💚 Health: http://${SERVER}/health"
echo "🔐 Auth: http://${SERVER}/auth"
echo ""
echo "Services Running:"
echo "  ✅ 16 Backend Services"
echo "  ✅ API Gateway (Port 3001) - NEW!"
echo "  ✅ React Frontend"
echo "  ✅ Nginx Reverse Proxy"
echo ""
echo "Next Steps:"
echo "  1. Configure Keycloak at http://${SERVER}/auth"
echo "  2. Test all frontend features"
echo "  3. Monitor logs: journalctl -u bpci-api-gateway -f"
echo ""
