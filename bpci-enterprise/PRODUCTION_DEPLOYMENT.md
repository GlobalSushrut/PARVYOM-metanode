# 🚀 BPCI ENTERPRISE PRODUCTION DEPLOYMENT
# ⚠️  CONFIDENTIAL - SECURE CREDENTIALS & INSTANCE DETAILS

## 🌐 PRODUCTION DOMAIN: pravyom.com

---

## 🔐 INSTANCE CREDENTIALS & SSH ACCESS

### 🖥️ BPCI SERVER (4 CPU Instance)
```bash
# Instance Details
INSTANCE_TYPE: 4 CPU / 8GB RAM
DOMAIN: pravyom.com
PORT: 9443 (HTTPS)
SERVICE: BPCI Coordination Server

# SSH Access
SSH_USER: [TO BE FILLED]
SSH_HOST: [TO BE FILLED - IP ADDRESS]
SSH_PORT: 22
SSH_KEY_PATH: /home/umesh/.ssh/bpci_server_key

# SSH Command
ssh -i /home/umesh/.ssh/bpci_server_key [USER]@[IP_ADDRESS]
```

### 📊 ADMIN DASHBOARD (2 CPU Instance)
```bash
# Instance Details
INSTANCE_TYPE: 2 CPU / 4GB RAM
DOMAIN: admin.pravyom.com
PORT: 8888 (HTTPS)
SERVICE: HTTPCG Admin Dashboard

# SSH Access
SSH_USER: [TO BE FILLED]
SSH_HOST: [TO BE FILLED - IP ADDRESS]
SSH_PORT: 22
SSH_KEY_PATH: /home/umesh/.ssh/admin_dashboard_key

# SSH Command
ssh -i /home/umesh/.ssh/admin_dashboard_key [USER]@[IP_ADDRESS]
```

### 💰 WALLET SERVER (2 CPU Instance)
```bash
# Instance Details
INSTANCE_TYPE: 2 CPU / 4GB RAM
DOMAIN: api.pravyom.com
PORT: 7778 (HTTPS)
SERVICE: HTTPCG Wallet Server

# SSH Access
SSH_USER: [TO BE FILLED]
SSH_HOST: [TO BE FILLED - IP ADDRESS]
SSH_PORT: 22
SSH_KEY_PATH: /home/umesh/.ssh/wallet_server_key

# SSH Command
ssh -i /home/umesh/.ssh/wallet_server_key [USER]@[IP_ADDRESS]
```

---

## 🔑 APPLICATION CREDENTIALS

### 👑 ROOT ADMIN ACCESS
```bash
# BPCI Enterprise Root Login
USERNAME: root
PASSWORD: [TO BE FILLED - SECURE PASSWORD]
ACCESS_LEVEL: Administrator
DEMO_MODE: false (production)
```

### 🎯 DEMO WALLET ACCESS
```bash
# Demo Wallet Credentials
WALLET_ID: demo
ACCESS_TYPE: Read-only demo
DEMO_MODE: true
RETURNS: "demo" responses for all operations
```

### 🔐 JWT & API SECRETS
```bash
# JWT Secret (Auto-generated)
JWT_SECRET: [TO BE FILLED - 32-byte random]

# API Keys
BPCI_API_KEY: [TO BE FILLED]
HTTPCG_API_KEY: [TO BE FILLED]
```

---

## 🌐 DNS CONFIGURATION

### 📋 DNS Records to Configure
```bash
# A Records
pravyom.com                 → [BPCI_SERVER_IP]
admin.pravyom.com          → [ADMIN_DASHBOARD_IP]
api.pravyom.com            → [WALLET_SERVER_IP]
www.pravyom.com            → [BPCI_SERVER_IP]

# CNAME Records (Optional)
httpcg.pravyom.com         → pravyom.com
wallet.pravyom.com         → api.pravyom.com
dashboard.pravyom.com      → admin.pravyom.com
```

---

## 🔐 TLS CERTIFICATES

### 📜 Let's Encrypt Configuration
```bash
# Certificate Domains
PRIMARY_DOMAIN: pravyom.com
ADDITIONAL_DOMAINS: www.pravyom.com,admin.pravyom.com,api.pravyom.com

# Let's Encrypt Email
LETSENCRYPT_EMAIL: admin@pravyom.com

# Certificate Paths (Auto-generated)
CERT_PATH: /etc/letsencrypt/live/pravyom.com/
RENEWAL_COMMAND: certbot renew --quiet
```

---

## 🚀 DEPLOYMENT COMMANDS

### 🔧 One-Click Deployment
```bash
# Deploy Complete System
cd /home/umesh/metanode/bpci-enterprise
./scripts/deploy-production-tls.sh

# Deploy Individual Services
./scripts/deploy-bpci-server.sh
./scripts/deploy-admin-dashboard.sh  
./scripts/deploy-wallet-server.sh
```

### 🔄 System Management
```bash
# Start All Services
./scripts/start-production-system.sh

# Stop All Services
./scripts/stop-system.sh

# Check System Status
./scripts/system-status.sh

# View Logs
tail -f logs/production-system.log
```

---

## 🌐 ACCESS POINTS (PRODUCTION)

### 🔗 Public URLs
```bash
# Main Website
https://pravyom.com

# Admin Dashboard (HTTPCG)
https://admin.pravyom.com:8888/httpcg/dashboard
httpcg://admin.pravyom.prav@global

# Wallet Server (HTTPCG)
https://api.pravyom.com:7778
httpcg://wallet.pravyom.prav@global

# BPCI Server API
https://pravyom.com:9443/api/status
```

---

## 🔒 SECURITY CHECKLIST

### ✅ Pre-Deployment Security
- [ ] SSH keys generated and secured
- [ ] Firewall rules configured (ports 22, 80, 443, 7778, 8888, 9443)
- [ ] Let's Encrypt certificates generated
- [ ] JWT secrets generated (32-byte random)
- [ ] Root password set (strong, unique)
- [ ] Demo mode configured correctly

### ✅ Post-Deployment Verification
- [ ] All services responding with HTTPS
- [ ] Green lock "Secure" status in browsers
- [ ] HTTPCG protocol headers present
- [ ] Demo mode returning "demo" responses
- [ ] SSH access working for all instances
- [ ] DNS records resolving correctly
- [ ] Certificate auto-renewal configured

---

## 📞 EMERGENCY CONTACTS & SUPPORT

### 🆘 Emergency Access
```bash
# If SSH access fails, use cloud provider console
CLOUD_PROVIDER: [TO BE FILLED]
CONSOLE_URL: [TO BE FILLED]
ACCOUNT_ID: [TO BE FILLED]
```

### 📋 Backup & Recovery
```bash
# Database Backup Location
BACKUP_PATH: /var/backups/bpci-enterprise/
BACKUP_FREQUENCY: Daily at 3 AM UTC
RETENTION: 30 days

# Configuration Backup
CONFIG_BACKUP: /home/umesh/metanode/bpci-enterprise-backup/
```

---

## ⚠️  IMPORTANT NOTES

1. **🔐 KEEP THIS FILE SECURE** - Contains sensitive credentials
2. **🔄 UPDATE REGULARLY** - Fill in placeholders as instances are created
3. **📋 BACKUP CREDENTIALS** - Store securely offline
4. **🔒 ROTATE KEYS** - Change SSH keys and passwords regularly
5. **📊 MONITOR SYSTEM** - Check logs and status regularly

---

**🎯 STATUS: READY FOR PRODUCTION DEPLOYMENT**
**📅 LAST UPDATED: 2025-09-08**
**👤 MAINTAINED BY: BPCI Enterprise Team**
