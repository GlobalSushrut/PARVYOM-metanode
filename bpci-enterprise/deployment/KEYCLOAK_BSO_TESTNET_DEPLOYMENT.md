# 🔐 Keycloak BSO Testnet Deployment Plan
## BPCI Enterprise Authentication Integration ($0 Additional Cost)

---

## 📊 **Current Infrastructure Analysis**

### **Instance 1: bpci-testnet-main (146.190.74.139)**
```yaml
Resources: 2 vCPU, 4GB RAM, 78GB disk
Current Services:
  - NGINX (port 80): Web server & reverse proxy
  - Python HTTP Server (port 3000): Static file serving
  - Rust pravyom-enterprise (port 8080): BSO testnet backend
  - Available RAM: ~3.2GB free
  - Available Disk: ~72GB free
```

### **Instance 2: bpci-real-advanced-db (157.230.238.92)**
```yaml
Resources: 2 vCPU, 4GB RAM, 80GB disk
Database Services:
  - MongoDB (port 27017): Document database
  - PostgreSQL (port 5432): Relational database
  - Purpose: Database services for all instances
```

### **Instance 3: bpi-public-installer (142.93.113.141)**
```yaml
Resources: 1 vCPU, 1GB RAM, 25GB disk
Purpose: BPI installer/downloader service
```

---

## 🎯 **Keycloak Deployment Strategy**

### **Architecture Decision**
```yaml
Frontend: Docker containers (as requested)
Keycloak: Native installation (no Docker)
Database: External PostgreSQL on db instance
BSO Backend: Keep existing Rust system unchanged
```

### **Resource Allocation**
```yaml
Current Usage (bpci-testnet-main):
  - System: ~350MB RAM
  - NGINX: ~50MB RAM
  - Python Server: ~20MB RAM
  - Rust Backend: ~200MB RAM
  - Total Used: ~620MB RAM

Keycloak Addition:
  - Keycloak (Java): ~800MB RAM
  - Total After: ~1.4GB / 4GB RAM (35% utilization) ✅
```

---

## 🚀 **Implementation Plan**

### **Phase 1: Database Setup**
```bash
# Connect to database instance
ssh root@157.230.238.92

# Create Keycloak database and user
sudo -u postgres psql
CREATE DATABASE keycloak;
CREATE USER keycloak WITH ENCRYPTED PASSWORD 'secure_keycloak_password_2024';
GRANT ALL PRIVILEGES ON DATABASE keycloak TO keycloak;
\q

# Configure PostgreSQL for remote connections
echo "host keycloak keycloak 146.190.74.139/32 md5" >> /etc/postgresql/14/main/pg_hba.conf
systemctl restart postgresql
```

### **Phase 2: Keycloak Native Installation**
```bash
# Connect to main instance
ssh root@146.190.74.139

# Install Java 17 (required for Keycloak)
apt update && apt install -y openjdk-17-jdk

# Download and install Keycloak
cd /opt
wget https://github.com/keycloak/keycloak/releases/download/22.0.5/keycloak-22.0.5.tar.gz
tar -xzf keycloak-22.0.5.tar.gz
mv keycloak-22.0.5 keycloak
chown -R root:root /opt/keycloak

# Create Keycloak configuration
cat > /opt/keycloak/conf/keycloak.conf << EOF
# Database configuration
db=postgres
db-url=jdbc:postgresql://157.230.238.92:5432/keycloak
db-username=keycloak
db-password=secure_keycloak_password_2024

# HTTP configuration
http-enabled=true
http-port=8180
http-host=0.0.0.0

# Hostname configuration
hostname=auth.pravyom.com
hostname-strict=false
hostname-strict-https=false

# Admin user
admin-user=admin
admin-password=SecureAdminPassword2024!
EOF
```

### **Phase 3: Systemd Service Setup**
```bash
# Create systemd service for Keycloak
cat > /etc/systemd/system/keycloak.service << EOF
[Unit]
Description=Keycloak Authentication Server
After=network.target

[Service]
Type=exec
User=root
Group=root
ExecStart=/opt/keycloak/bin/kc.sh start
Restart=on-failure
RestartSec=5
Environment=JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64

[Install]
WantedBy=multi-user.target
EOF

# Enable and start Keycloak
systemctl daemon-reload
systemctl enable keycloak
systemctl start keycloak
```

### **Phase 4: NGINX Configuration**
```bash
# Add Keycloak proxy to NGINX
cat >> /etc/nginx/sites-available/bpci << EOF

# Keycloak authentication server
server {
    listen 80;
    listen [::]:80;
    
    server_name auth.pravyom.com;
    
    location / {
        proxy_pass http://127.0.0.1:8180;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_buffer_size 128k;
        proxy_buffers 4 256k;
        proxy_busy_buffers_size 256k;
    }
}
EOF

# Test and reload NGINX
nginx -t && systemctl reload nginx
```

### **Phase 5: Docker Frontend Setup**
```bash
# Install Docker (when system updates complete)
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh

# Create Docker Compose for frontend
mkdir -p /var/www/docker-frontend
cat > /var/www/docker-frontend/docker-compose.yml << EOF
version: '3.8'

services:
  bpci-frontend:
    image: nginx:alpine
    container_name: bpci-frontend
    ports:
      - "3001:80"
    volumes:
      - /var/www/bpci:/usr/share/nginx/html:ro
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
    restart: unless-stopped
    
  bpci-testnet-frontend:
    image: nginx:alpine
    container_name: bpci-testnet-frontend
    ports:
      - "3002:80"
    volumes:
      - /var/www/bpci-testnet:/usr/share/nginx/html:ro
    restart: unless-stopped
EOF

# Start Docker containers
cd /var/www/docker-frontend
docker-compose up -d
```

---

## 🔧 **Port Configuration**

### **Current Ports**
```yaml
80: NGINX (HTTP reverse proxy)
3000: Python static server (to be replaced)
8080: Rust pravyom-enterprise (BSO backend)
```

### **New Port Layout**
```yaml
80: NGINX (HTTP reverse proxy)
8080: Rust pravyom-enterprise (BSO backend) - unchanged
8180: Keycloak (internal, proxied via NGINX)
3001: Docker frontend container (bpci)
3002: Docker frontend container (bpci-testnet)
```

---

## 🌐 **Domain Configuration**

### **DNS Records Needed**
```yaml
auth.pravyom.com → 146.190.74.139 (A record)
```

### **NGINX Routing**
```yaml
pravyom.com → Docker frontend (port 3001)
testnet.pravyom.com → Docker frontend (port 3002)
auth.pravyom.com → Keycloak (port 8180)
api.pravyom.com → Rust backend (port 8080)
```

---

## 🔐 **Security Configuration**

### **SSL/TLS Setup**
```bash
# Install Certbot for Let's Encrypt
apt install -y certbot python3-certbot-nginx

# Generate SSL certificates
certbot --nginx -d auth.pravyom.com

# Auto-renewal
echo "0 12 * * * /usr/bin/certbot renew --quiet" | crontab -
```

### **Firewall Rules**
```bash
# Allow required ports
ufw allow 80/tcp
ufw allow 443/tcp
ufw allow 8080/tcp
ufw allow from 157.230.238.92 to any port 5432
```

---

## 🧪 **Testing & Validation**

### **Health Checks**
```bash
# Test Keycloak
curl -I http://localhost:8180/health/ready

# Test NGINX proxy
curl -I http://auth.pravyom.com

# Test database connection
psql -h 157.230.238.92 -U keycloak -d keycloak -c "SELECT 1;"
```

### **Integration Tests**
```bash
# Test BSO backend (unchanged)
curl -I http://localhost:8080/health

# Test Docker frontend
curl -I http://localhost:3001
curl -I http://localhost:3002
```

---

## 📋 **Deployment Checklist**

- [ ] Wait for system updates to complete
- [ ] Install Java 17 on main instance
- [ ] Create Keycloak database on db instance
- [ ] Download and configure Keycloak
- [ ] Create systemd service for Keycloak
- [ ] Update NGINX configuration
- [ ] Install Docker when system is ready
- [ ] Create Docker frontend containers
- [ ] Configure SSL certificates
- [ ] Test all services and integrations
- [ ] Update DNS records for auth.pravyom.com

---

## 💰 **Cost Analysis**

```yaml
Additional Monthly Cost: $0
Resource Usage:
  - RAM: +800MB (35% total utilization)
  - Disk: +500MB (Keycloak installation)
  - CPU: Minimal additional load
  
Total Infrastructure Cost: $50/month (unchanged)
```

---

## 🎯 **Success Criteria**

1. ✅ Keycloak running natively on main instance
2. ✅ Connected to external PostgreSQL database
3. ✅ Frontend containerized with Docker
4. ✅ BSO testnet backend unchanged and functional
5. ✅ SSL/TLS enabled for auth.pravyom.com
6. ✅ All services integrated and tested
7. ✅ Zero additional infrastructure cost

**This deployment provides enterprise-grade authentication while maintaining the existing BSO testnet architecture and staying within the $50/month budget!** 🚀
