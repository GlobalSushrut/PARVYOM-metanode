# 🌐 PHASE 4: WEB BACKEND & KEYCLOAK DEPLOYMENT

**Date**: 2025-10-30  
**Server**: bpci-testnet-server (134.209.210.181)  
**Status**: READY TO DEPLOY

---

## 📋 PHASE 4 OVERVIEW

Deploy the web backend layer that connects the frontend to the BPCI backend services, with Keycloak authentication.

---

## 🎯 COMPONENTS TO DEPLOY

### **1. BPCI Web Server (web.rs)**
- **Port**: 3000 (HTTP API)
- **Features**:
  - Blockchain statistics and monitoring
  - Wallet registry integration
  - Mining bridge API
  - Bank API integration
  - Government API integration
  - Stamped wallet API access
  - Real-time blockchain stats
  - User management
  - Session management
  - API key management

### **2. Keycloak Authentication**
- **Port**: 8180 (already running)
- **Status**: ✅ Installed and running
- **Configuration Needed**:
  - Create BPCI realm
  - Configure clients
  - Set up roles
  - Configure user federation

### **3. Nginx Reverse Proxy**
- **Port**: 80/443
- **Status**: ✅ Installed and running
- **Configuration Needed**:
  - Proxy /api → Web Server (3000)
  - Proxy /auth → Keycloak (8180)
  - Proxy /blockchain → Blockchain Server (8080)
  - Proxy /bridge → BPI Bridge (6001)

---

## 🔧 DEPLOYMENT STEPS

### **Step 1: Configure Keycloak**

```bash
# Access Keycloak admin console
http://134.209.210.181:8180/admin

# Create BPCI realm
# Create clients: bpci-web, bpci-frontend
# Configure roles: admin, user, miner, validator
# Set up user federation with PostgreSQL
```

### **Step 2: Build Web Server Binary**

The web server is part of the main `pravyom-enterprise` binary with CLI commands.

```bash
# Already built in Phase 3
/opt/bpci/bin/pravyom-enterprise web start --port 3000
```

### **Step 3: Create Web Server Systemd Service**

```bash
cat > /etc/systemd/system/bpci-web.service << EOF
[Unit]
Description=BPCI Web Backend Server
After=network.target postgresql.service redis-server.service keycloak.service
Requires=postgresql.service redis-server.service

[Service]
Type=simple
User=bpci
Group=bpci
WorkingDirectory=/opt/bpci
Environment="RUST_LOG=info"
Environment="DATABASE_URL=postgresql://bpci:bpci_secure_password_2024@localhost:5432/bpci_blockchain"
Environment="REDIS_URL=redis://localhost:6379"
Environment="KEYCLOAK_URL=http://localhost:8180"
ExecStart=/opt/bpci/bin/pravyom-enterprise web start --port 3000 --host 0.0.0.0
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF
```

### **Step 4: Configure Nginx Reverse Proxy**

```nginx
server {
    listen 80;
    server_name 134.209.210.181;

    # Web API
    location /api/ {
        proxy_pass http://localhost:3000/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }

    # Keycloak Auth
    location /auth/ {
        proxy_pass http://localhost:8180/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Blockchain Server
    location /blockchain/ {
        proxy_pass http://localhost:8080/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    # BPI Bridge
    location /bridge/ {
        proxy_pass http://localhost:6001/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    # BSO-K8 Orchestrator
    location /orchestrator/ {
        proxy_pass http://localhost:9090/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

### **Step 5: Start Web Server**

```bash
systemctl daemon-reload
systemctl enable bpci-web
systemctl start bpci-web
systemctl status bpci-web
```

---

## 🔐 KEYCLOAK CONFIGURATION

### **Realm Configuration**

```json
{
  "realm": "bpci",
  "enabled": true,
  "sslRequired": "none",
  "registrationAllowed": true,
  "loginWithEmailAllowed": true,
  "duplicateEmailsAllowed": false,
  "resetPasswordAllowed": true,
  "editUsernameAllowed": false,
  "bruteForceProtected": true
}
```

### **Client Configuration**

**bpci-web** (Backend):
- Client ID: bpci-web
- Access Type: confidential
- Valid Redirect URIs: http://134.209.210.181/*
- Web Origins: *

**bpci-frontend** (React):
- Client ID: bpci-frontend
- Access Type: public
- Valid Redirect URIs: http://134.209.210.181/*
- Web Origins: *

### **Roles**

- **admin**: Full system access
- **user**: Basic user access
- **miner**: Mining operations
- **validator**: Consensus validation
- **bank**: Bank API access
- **government**: Government API access

---

## 📊 API ENDPOINTS

### **Web Server Endpoints (Port 3000)**

```
GET  /health                    - Health check
GET  /stats                     - Blockchain statistics
GET  /blockchain/info           - Blockchain information
GET  /blockchain/height         - Current block height
GET  /blockchain/peers          - Connected peers
POST /wallet/register           - Register wallet
GET  /wallet/:address           - Get wallet info
POST /mining/start              - Start mining
GET  /mining/status             - Mining status
POST /bank/register             - Register bank
POST /bank/settlement           - Initiate settlement
GET  /government/regulatory     - Regulatory info
```

---

## ✅ VERIFICATION CHECKLIST

- [ ] Keycloak accessible at http://134.209.210.181:8180
- [ ] BPCI realm created in Keycloak
- [ ] Clients configured (bpci-web, bpci-frontend)
- [ ] Roles created and assigned
- [ ] Web server binary available
- [ ] Web server systemd service created
- [ ] Web server running on port 3000
- [ ] Nginx reverse proxy configured
- [ ] Nginx reloaded with new config
- [ ] API endpoints accessible via Nginx
- [ ] Authentication flow working
- [ ] Database connections working
- [ ] Redis connections working
- [ ] Backend services integration working

---

## 🚀 EXPECTED RESULT

After Phase 4 completion:

- ✅ Web backend running on port 3000
- ✅ Keycloak authentication configured
- ✅ Nginx reverse proxy routing all requests
- ✅ All API endpoints accessible
- ✅ Frontend can connect to backend
- ✅ Authentication flow complete
- ✅ Full stack operational

---

## 📝 NEXT STEPS (Phase 5)

1. Deploy React frontend
2. Configure frontend to use backend APIs
3. Test end-to-end authentication
4. Test all API endpoints
5. Performance testing
6. Security hardening

---

**Status**: Ready to execute Phase 4 deployment
