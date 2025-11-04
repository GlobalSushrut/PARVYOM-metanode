# Digital Ocean BPCI Testnet Setup Guide

## 🌊 **Digital Ocean Infrastructure Requirements**

Based on the ultra-lightweight BPCI testnet architecture, here's exactly what we need on Digital Ocean:

## 📋 **Digital Ocean Resources Needed**

### 1. **Droplets (Virtual Servers)**

```yaml
# Droplet 1: BPCI Website (pravyom.com)
Name: bpci-website
Size: Regular SSD 1CPU-2GB ($6/month)
OS: Ubuntu 22.04 LTS
CPU: 1 vCPU
Memory: 2GB RAM
Storage: 25GB SSD
Network: 100GB transfer
Purpose: BPCI Enterprise website hosting

# Droplet 2: BPCI XTMP Server (bpci.pravyom.world)
Name: bpci-xtmp-server
Size: Regular SSD 2CPU-4GB ($12/month)
OS: Ubuntu 22.04 LTS
CPU: 2 vCPU
Memory: 4GB RAM
Storage: 25GB SSD
Network: 100GB transfer
Purpose: BPCI XTMP server + mock databases

# Droplet 3: BPI Downloader CDN (get.bpi.pravyom.com)
Name: bpi-downloader
Size: Regular SSD 1CPU-1GB ($4/month)
OS: Ubuntu 22.04 LTS
CPU: 1 vCPU
Memory: 1GB RAM
Storage: 25GB SSD
Network: 100GB transfer
Purpose: BPI installer files and CDN
```

### 2. **Managed Databases**

```yaml
# PostgreSQL Database for Mock Systems
Name: bpci-testnet-db
Type: Managed PostgreSQL
Size: db-s-1vcpu-1gb ($15/month)
CPU: 1 vCPU
Memory: 1GB RAM
Storage: 10GB SSD
Purpose: bpigov_db, bpicom_db, auction_mock_db
```

### 3. **Networking & DNS**

```yaml
# Domain Management
Domains needed:
- pravyom.com (BPCI Website)
- bpci.pravyom.world (XTMP Server)
- get.bpi.pravyom.com (Downloader)

# Load Balancer (Optional)
Name: bpci-load-balancer
Size: lb-small ($12/month)
Purpose: High availability for XTMP server

# Firewall Rules
Name: bpci-firewall
Rules:
- HTTP (80): Allow all
- HTTPS (443): Allow all
- XTMP (7778): Allow all
- SSH (22): Restrict to admin IPs
- PostgreSQL (5432): Internal only
```

### 4. **Storage & Backup**

```yaml
# Spaces (Object Storage) for BPI Downloader
Name: bpi-installers
Size: 5GB ($1/month)
Purpose: Store installer files (install.sh, install.py, etc.)

# Volume Backup
Automatic backups: Enabled
Retention: 7 days
Cost: ~$3/month
```

## 💰 **Total Digital Ocean Cost Breakdown (Current 2024 Pricing)**

```yaml
Monthly Costs:
- BPCI Website Droplet (Regular SSD 1CPU-2GB): $6/month
- BPCI XTMP Server Droplet (Regular SSD 2CPU-4GB): $12/month
- BPI Downloader Droplet (Regular SSD 1CPU-1GB): $4/month
- Managed PostgreSQL (db-s-1vcpu-1gb): $15/month
- Spaces Storage (5GB): $5/month
- Automatic Backups: $3/month
- Load Balancer (optional): $12/month

Total (without LB): $45/month
Total (with LB): $57/month
```

## 🚀 **Digital Ocean Setup Steps**

### Step 1: Create Digital Ocean Account & Setup

```bash
# 1. Create Digital Ocean account
# 2. Add payment method
# 3. Generate API token for automation
# 4. Install doctl (Digital Ocean CLI)

# Install doctl
curl -sL https://github.com/digitalocean/doctl/releases/download/v1.94.0/doctl-1.94.0-linux-amd64.tar.gz | tar -xzv
sudo mv doctl /usr/local/bin

# Authenticate
doctl auth init
```

### Step 2: Create SSH Keys

```bash
# Generate SSH key for server access
ssh-keygen -t ed25519 -C "bpci-testnet-admin" -f ~/.ssh/bpci_testnet_key

# Add SSH key to Digital Ocean
doctl compute ssh-key create bpci-testnet-key --public-key-file ~/.ssh/bpci_testnet_key.pub
```

### Step 3: Create Droplets

```bash
# Create BPCI Website droplet
doctl compute droplet create bpci-website \
  --image ubuntu-22-04-x64 \
  --size s-1vcpu-2gb \
  --region nyc1 \
  --ssh-keys $(doctl compute ssh-key list --format ID --no-header) \
  --enable-monitoring \
  --enable-ipv6

# Create BPCI XTMP Server droplet
doctl compute droplet create bpci-xtmp-server \
  --image ubuntu-22-04-x64 \
  --size s-2vcpu-4gb \
  --region nyc1 \
  --ssh-keys $(doctl compute ssh-key list --format ID --no-header) \
  --enable-monitoring \
  --enable-ipv6

# Create BPI Downloader droplet
doctl compute droplet create bpi-downloader \
  --image ubuntu-22-04-x64 \
  --size s-1vcpu-1gb \
  --region nyc1 \
  --ssh-keys $(doctl compute ssh-key list --format ID --no-header) \
  --enable-monitoring \
  --enable-ipv6
```

### Step 4: Create Managed Database

```bash
# Create PostgreSQL database for mock systems
doctl databases create bpci-testnet-db \
  --engine pg \
  --version 15 \
  --size db-s-1vcpu-1gb \
  --region nyc1 \
  --num-nodes 1
```

### Step 5: Setup Networking

```bash
# Create firewall
doctl compute firewall create bpci-firewall \
  --inbound-rules "protocol:tcp,ports:22,sources:load_balancer,sources:droplet_tag:admin" \
  --inbound-rules "protocol:tcp,ports:80,sources:0.0.0.0/0,::/0" \
  --inbound-rules "protocol:tcp,ports:443,sources:0.0.0.0/0,::/0" \
  --inbound-rules "protocol:tcp,ports:7778,sources:0.0.0.0/0,::/0" \
  --outbound-rules "protocol:tcp,ports:all,destinations:0.0.0.0/0,::/0" \
  --outbound-rules "protocol:udp,ports:all,destinations:0.0.0.0/0,::/0"

# Apply firewall to droplets
doctl compute firewall add-droplets bpci-firewall --droplet-ids $(doctl compute droplet list --format ID --no-header)
```

### Step 6: Create Spaces for BPI Downloader

```bash
# Create Spaces bucket for installer files
doctl spaces create bpi-installers --region nyc3

# Upload installer files
doctl spaces put bpi-installers install-bpi.sh --acl public-read
doctl spaces put bpi-installers install-bpi.py --acl public-read
doctl spaces put bpi-installers install-bpi.ps1 --acl public-read
doctl spaces put bpi-installers bpi-get.sh --acl public-read
```

## 🔧 **Server Configuration Scripts**

### BPCI Website Server Setup

```bash
#!/bin/bash
# bpci-website-setup.sh

# Get droplet IP
WEBSITE_IP=$(doctl compute droplet get bpci-website --format PublicIPv4 --no-header)

# SSH into server and setup
ssh -i ~/.ssh/bpci_testnet_key root@$WEBSITE_IP << 'EOF'
# Update system
apt update && apt upgrade -y

# Install dependencies
apt install -y nginx certbot python3-certbot-nginx nodejs npm git curl

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Clone BPCI codebase
git clone https://github.com/GlobalSushrut/PARVYOM-metanode.git /opt/bpci
cd /opt/bpci

# Build BPCI website
cd pravyom-website
npm install
npm run build

# Configure Nginx
cat > /etc/nginx/sites-available/bpci-website << 'NGINX_EOF'
server {
    listen 80;
    server_name pravyom.com www.pravyom.com;
    
    location / {
        root /opt/bpci/pravyom-website/dist;
        try_files $uri $uri/ /index.html;
    }
    
    location /api/ {
        proxy_pass http://localhost:8080/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
NGINX_EOF

# Enable site
ln -s /etc/nginx/sites-available/bpci-website /etc/nginx/sites-enabled/
nginx -t && systemctl reload nginx

# Setup SSL
certbot --nginx -d pravyom.com -d www.pravyom.com --non-interactive --agree-tos --email admin@pravyom.com

# Start BPCI backend
cd /opt/bpci/bpci-enterprise
cargo build --release
./target/release/bpci-consensus-server --mode testnet --port 8080 &

echo "BPCI Website setup complete!"
EOF
```

### BPCI XTMP Server Setup

```bash
#!/bin/bash
# bpci-xtmp-setup.sh

# Get droplet IP
XTMP_IP=$(doctl compute droplet get bpci-xtmp-server --format PublicIPv4 --no-header)

# Get database connection details
DB_HOST=$(doctl databases connection bpci-testnet-db --format Host --no-header)
DB_PORT=$(doctl databases connection bpci-testnet-db --format Port --no-header)
DB_USER=$(doctl databases connection bpci-testnet-db --format User --no-header)
DB_PASS=$(doctl databases connection bpci-testnet-db --format Password --no-header)

# SSH into server and setup
ssh -i ~/.ssh/bpci_testnet_key root@$XTMP_IP << EOF
# Update system
apt update && apt upgrade -y

# Install dependencies
apt install -y git curl build-essential pkg-config libssl-dev postgresql-client

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Clone BPCI codebase
git clone https://github.com/GlobalSushrut/PARVYOM-metanode.git /opt/bpci
cd /opt/bpci

# Setup database connection
cat > /opt/bpci/testnet-config.toml << 'CONFIG_EOF'
[server]
mode = "testnet"
host = "0.0.0.0"
port = 7778

[database]
host = "$DB_HOST"
port = $DB_PORT
user = "$DB_USER"
password = "$DB_PASS"
database = "bpci_testnet"

[auction_mode]
type = "Testnet"
mock_to_bpi_db = true
simulate_community_bidding = true

[mock_databases]
bpigov_enabled = true
bpicom_enabled = true
auction_settlement_mocked = true
CONFIG_EOF

# Initialize mock databases
PGPASSWORD=$DB_PASS psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d defaultdb << 'SQL_EOF'
CREATE DATABASE bpci_testnet;
\c bpci_testnet;

-- Mock Government Database
CREATE TABLE mock_governance (
    id SERIAL PRIMARY KEY,
    proposal_id VARCHAR(255),
    decision VARCHAR(50),
    timestamp TIMESTAMP DEFAULT NOW(),
    mock_authority VARCHAR(100)
);

-- Mock Community Database  
CREATE TABLE mock_community (
    id SERIAL PRIMARY KEY,
    community_id VARCHAR(255),
    bid_amount BIGINT,
    timestamp TIMESTAMP DEFAULT NOW(),
    mock_participant VARCHAR(100)
);

-- Mock Auction Results
CREATE TABLE mock_auctions (
    id SERIAL PRIMARY KEY,
    auction_id VARCHAR(255),
    total_revenue BIGINT,
    winning_validator VARCHAR(255),
    settlement_status VARCHAR(50),
    mock_settlement BOOLEAN DEFAULT true,
    timestamp TIMESTAMP DEFAULT NOW()
);

-- Insert sample mock data
INSERT INTO mock_governance (proposal_id, decision, mock_authority) VALUES
('GOV-001', 'APPROVED', 'Mock Government Authority'),
('GOV-002', 'PENDING', 'Mock Regulatory Body');

INSERT INTO mock_community (community_id, bid_amount, mock_participant) VALUES
('COMM-001', 1000000, 'Mock Community A'),
('COMM-002', 750000, 'Mock Community B');

INSERT INTO mock_auctions (auction_id, total_revenue, winning_validator, settlement_status) VALUES
('AUC-001', 5000000, 'validator_123', 'MOCK_SETTLED'),
('AUC-002', 3500000, 'validator_456', 'MOCK_PENDING');
SQL_EOF

# Build and start BPCI XTMP server
cd /opt/bpci/bpci-enterprise
cargo build --release

# Create systemd service
cat > /etc/systemd/system/bpci-xtmp.service << 'SERVICE_EOF'
[Unit]
Description=BPCI XTMP Server (Testnet)
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/bpci/bpci-enterprise
ExecStart=/opt/bpci/bpci-enterprise/target/release/bpci-xtmp-server --config /opt/bpci/testnet-config.toml
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
SERVICE_EOF

# Start service
systemctl daemon-reload
systemctl enable bpci-xtmp
systemctl start bpci-xtmp

echo "BPCI XTMP Server setup complete!"
EOF
```

### BPI Downloader Server Setup

```bash
#!/bin/bash
# bpi-downloader-setup.sh

# Get droplet IP
DOWNLOADER_IP=$(doctl compute droplet get bpi-downloader --format PublicIPv4 --no-header)

# SSH into server and setup
ssh -i ~/.ssh/bpci_testnet_key root@$DOWNLOADER_IP << 'EOF'
# Update system
apt update && apt upgrade -y

# Install dependencies
apt install -y nginx certbot python3-certbot-nginx git curl

# Clone installer files
git clone https://github.com/GlobalSushrut/PARVYOM-metanode.git /opt/bpi-installers
cd /opt/bpi-installers

# Setup web directory
mkdir -p /var/www/bpi-downloader
cp install-bpi.sh /var/www/bpi-downloader/
cp install-bpi.py /var/www/bpi-downloader/
cp install-bpi.ps1 /var/www/bpi-downloader/
cp bpi-get.sh /var/www/bpi-downloader/
cp INSTALLER_README.md /var/www/bpi-downloader/README.md

# Create index page
cat > /var/www/bpi-downloader/index.html << 'HTML_EOF'
<!DOCTYPE html>
<html>
<head>
    <title>BPI Downloader</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .download-btn { 
            display: inline-block; 
            padding: 10px 20px; 
            margin: 10px; 
            background: #007cba; 
            color: white; 
            text-decoration: none; 
            border-radius: 5px; 
        }
    </style>
</head>
<body>
    <h1>BPI Infrastructure Downloader</h1>
    <p>Download and install BPI infrastructure with one command:</p>
    
    <h2>Quick Install</h2>
    <pre><code>curl -fsSL https://get.bpi.pravyom.com | bash</code></pre>
    
    <h2>Manual Downloads</h2>
    <a href="/install-bpi.sh" class="download-btn">Linux/macOS Installer</a>
    <a href="/install-bpi.py" class="download-btn">Universal Python Installer</a>
    <a href="/install-bpi.ps1" class="download-btn">Windows PowerShell Installer</a>
    <a href="/bpi-get.sh" class="download-btn">BPI Package Manager</a>
    <a href="/README.md" class="download-btn">Documentation</a>
    
    <h2>Testnet Connection</h2>
    <pre><code>bpi-get connect testnet --endpoint=bpci.pravyom.world:7778</code></pre>
</body>
</html>
HTML_EOF

# Configure Nginx
cat > /etc/nginx/sites-available/bpi-downloader << 'NGINX_EOF'
server {
    listen 80;
    server_name get.bpi.pravyom.com;
    
    location / {
        root /var/www/bpi-downloader;
        autoindex on;
        add_header Access-Control-Allow-Origin *;
    }
    
    location = / {
        root /var/www/bpi-downloader;
        try_files /index.html =404;
    }
}
NGINX_EOF

# Enable site
ln -s /etc/nginx/sites-available/bpi-downloader /etc/nginx/sites-enabled/
nginx -t && systemctl reload nginx

# Setup SSL
certbot --nginx -d get.bpi.pravyom.com --non-interactive --agree-tos --email admin@pravyom.com

echo "BPI Downloader setup complete!"
EOF
```

## 🔍 **DNS Configuration**

### Domain Setup Required

```bash
# Add these DNS records to your domain registrar:

# A Records
pravyom.com                 -> [WEBSITE_DROPLET_IP]
www.pravyom.com            -> [WEBSITE_DROPLET_IP]
bpci.pravyom.world         -> [XTMP_SERVER_IP]
get.bpi.pravyom.com        -> [DOWNLOADER_IP]

# CNAME Records (alternative)
www                        -> pravyom.com
bpci.pravyom.world         -> pravyom.com
get.bpi                    -> pravyom.com
```

## 🚀 **Complete Deployment Script**

```bash
#!/bin/bash
# complete-digital-ocean-deployment.sh

echo "🌊 Starting BPCI Testnet deployment on Digital Ocean..."

# Step 1: Create all resources
echo "📦 Creating Digital Ocean resources..."
./create-droplets.sh
./create-database.sh
./setup-networking.sh
./create-spaces.sh

# Step 2: Configure servers
echo "⚙️ Configuring servers..."
./bpci-website-setup.sh
./bpci-xtmp-setup.sh
./bpi-downloader-setup.sh

# Step 3: Verify deployment
echo "✅ Verifying deployment..."
curl -f https://pravyom.com/api/health
curl -f https://bpci.pravyom.world:7778/health
curl -f https://get.bpi.pravyom.com

echo "🎉 BPCI Testnet deployment complete!"
echo "💰 Monthly cost: ~$45/month"
echo "🔗 Endpoints:"
echo "   - Website: https://pravyom.com"
echo "   - XTMP Server: bpci.pravyom.world:7778"
echo "   - Downloader: https://get.bpi.pravyom.com"
```

## 📊 **Monitoring & Health Checks**

```bash
# Health check endpoints
curl https://pravyom.com/api/health                    # Website
curl https://bpci.pravyom.world:7778/health           # XTMP server
curl https://bpci.pravyom.world:8080/testnet/status   # Testnet status
curl https://get.bpi.pravyom.com                      # Downloader

# Database health
doctl databases connection bpci-testnet-db --format Status
```

## 🎯 **Summary**

### **What We Need on Digital Ocean:**

1. **3 Droplets** ($22/month total)
   - BPCI Website: Regular SSD 1CPU-2GB ($6/month)
   - BPCI XTMP Server: Regular SSD 2CPU-4GB ($12/month)
   - BPI Downloader: Regular SSD 1CPU-1GB ($4/month)

2. **1 Managed Database** ($15/month)
   - PostgreSQL for mock systems (bpigov/bpicom/auctions)

3. **Storage & Networking** ($8/month)
   - Spaces for installer files ($5/month)
   - Backups ($3/month)

### **Total Monthly Cost: $45/month**

### **Setup Time: ~2 hours**

This gives us a complete, production-ready BPCI testnet infrastructure with mocked community/government systems, exactly as designed in the real codebase! 🚀
