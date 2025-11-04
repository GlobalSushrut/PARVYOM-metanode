# 🌐 CLOUDFLARE + TERRAFORM PROXY INFRASTRUCTURE PLAN
## Modern Cloud-Native Proxy Architecture for BSO-K8 PRAVYOM Deployment

---

## 🎯 **COMPREHENSIVE BPCI ENTERPRISE PROXY ARCHITECTURE**

### **Complete BPCI Infrastructure Scope**

```yaml
BPCI ENTERPRISE INFRASTRUCTURE (112 vPods across 4 instances):
  Instance 1: Frontend/Backend Cluster (30 vPods)
    - BPCI Enterprise Server: 12 vPods × 8MB = 96MB
    - Vite Frontend: 8 vPods × 6MB = 48MB  
    - Management Dashboard: 6 vPods × 6MB = 36MB
    - NGINX Load Balancer: 4 vPods × 6MB = 24MB
    
  Instance 2: Database Cluster (22 vPods)
    - PostgreSQL Controller: 8 vPods × 10MB = 80MB
    - MongoDB Controller: 6 vPods × 10MB = 60MB
    - Redis Cache: 4 vPods × 8MB = 32MB
    - Database Proxy: 4 vPods × 8MB = 32MB
    
  Instance 3: BPI Ecosystem Cluster (18 vPods)
    - BPI Downloader: 6 vPods × 8MB = 48MB
    - BPI Registry: 4 vPods × 8MB = 32MB
    - BPI Installer: 4 vPods × 8MB = 32MB
    - NGINX Frontend: 4 vPods × 6MB = 24MB
    
  Instance 4: Advanced Infrastructure Cluster (42 vPods)
    - Neural Blockchain Nodes: 16 vPods × 12MB = 192MB
    - 6D Consensus Engine: 12 vPods × 15MB = 180MB
    - LCCD Validator: 8 vPods × 12MB = 96MB
    - Monitoring & Metrics: 6 vPods × 8MB = 48MB

TARGET (Modern Cloud-Native Proxy):
  - Cloudflare CDN + DDoS protection for all 112 vPods
  - Terraform Infrastructure as Code
  - Intelligent routing across all BPCI services
  - Advanced security + analytics for enterprise platform
  - BSO-K8 native vPod integration (not containers!)
```

### **Technology Stack**
- **Cloudflare**: CDN, DNS, Security, Analytics
- **Terraform**: Infrastructure as Code
- **NGINX**: Local load balancing + vPod routing
- **BSO-K8**: Native vPod orchestration
- **Let's Encrypt**: SSL certificate automation

---

## 🏗️ **TERRAFORM CLOUDFLARE INFRASTRUCTURE**

### **1. Terraform Configuration Structure**

```hcl
# terraform/main.tf
terraform {
  required_providers {
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.0"
    }
  }
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}

# Domain and Zone Configuration
resource "cloudflare_zone" "pravyom_zone" {
  zone = "pravyom.com"
  plan = "free"  # or "pro" for advanced features
}

# DNS Records for Complete BPCI Enterprise Architecture
resource "cloudflare_record" "main_frontend" {
  zone_id = cloudflare_zone.pravyom_zone.id
  name    = "@"
  value   = "146.190.74.139"  # Instance 1
  type    = "A"
  ttl     = 300
  proxied = true  # Enable Cloudflare proxy
}

resource "cloudflare_record" "api_backend" {
  zone_id = cloudflare_zone.pravyom_zone.id
  name    = "api"
  value   = "146.190.74.139"  # Instance 1
  type    = "A"
  ttl     = 300
  proxied = true
}

resource "cloudflare_record" "management_dashboard" {
  zone_id = cloudflare_zone.pravyom_zone.id
  name    = "management"
  value   = "146.190.74.139"  # Instance 1
  type    = "A"
  ttl     = 300
  proxied = true
}

resource "cloudflare_record" "db_internal" {
  zone_id = cloudflare_zone.pravyom_zone.id
  name    = "db"
  value   = "157.230.238.92"  # Instance 2
  type    = "A"
  ttl     = 300
  proxied = false  # Internal only
}

resource "cloudflare_record" "postgres_internal" {
  zone_id = cloudflare_zone.pravyom_zone.id
  name    = "postgres"
  value   = "157.230.238.92"  # Instance 2
  type    = "A"
  ttl     = 300
  proxied = false  # Internal only
}

resource "cloudflare_record" "mongo_internal" {
  zone_id = cloudflare_zone.pravyom_zone.id
  name    = "mongo"
  value   = "157.230.238.92"  # Instance 2
  type    = "A"
  ttl     = 300
  proxied = false  # Internal only
}

resource "cloudflare_record" "downloader_services" {
  zone_id = cloudflare_zone.pravyom_zone.id
  name    = "downloader"
  value   = "142.93.113.141"  # Instance 3
  type    = "A"
  ttl     = 300
  proxied = true
}

resource "cloudflare_record" "registry_services" {
  zone_id = cloudflare_zone.pravyom_zone.id
  name    = "registry"
  value   = "142.93.113.141"  # Instance 3
  type    = "A"
  ttl     = 300
  proxied = true
}

resource "cloudflare_record" "installer_services" {
  zone_id = cloudflare_zone.pravyom_zone.id
  name    = "installer"
  value   = "142.93.113.141"  # Instance 3
  type    = "A"
  ttl     = 300
  proxied = true
}

resource "cloudflare_record" "advanced_bpci" {
  zone_id = cloudflare_zone.pravyom_zone.id
  name    = "advanced"
  value   = var.instance_4_ip  # Instance 4 - Advanced BPCI infrastructure
  type    = "A"
  ttl     = 300
  proxied = true
}

resource "cloudflare_record" "blockchain_nodes" {
  zone_id = cloudflare_zone.pravyom_zone.id
  name    = "blockchain"
  value   = var.instance_4_ip  # Instance 4 - Advanced BPCI infrastructure
  type    = "A"
  ttl     = 300
  proxied = true
}

resource "cloudflare_record" "consensus_engine" {
  zone_id = cloudflare_zone.pravyom_zone.id
  name    = "consensus"
  value   = var.instance_4_ip  # Instance 4 - Advanced BPCI infrastructure
  type    = "A"
  ttl     = 300
  proxied = true
}

resource "cloudflare_record" "validator" {
  zone_id = cloudflare_zone.pravyom_zone.id
  name    = "validator"
  value   = var.instance_4_ip  # Instance 4 - Advanced BPCI infrastructure
  type    = "A"
  ttl     = 300
  proxied = true
}

resource "cloudflare_record" "monitoring" {
  zone_id = cloudflare_zone.pravyom_zone.id
  name    = "monitoring"
  value   = var.instance_4_ip  # Instance 4 - Advanced BPCI infrastructure
  type    = "A"
  ttl     = 300
  proxied = true
}

# BPI OS access endpoint (external users)
resource "cloudflare_record" "bpi_os" {
  zone_id = cloudflare_zone.pravyom_zone.id
  name    = "bpi"
  value   = "146.190.74.139"  # Routes through Instance 1 for BPI OS access
  type    = "A"
  ttl     = 300
  proxied = true
}
```

### **2. Cloudflare Security Rules**

```hcl
# Security and Access Rules
resource "cloudflare_ruleset" "security_rules" {
  zone_id = cloudflare_zone.pravyom_zone.id
  name    = "PRAVYOM Security Rules"
  kind    = "zone"
  phase   = "http_request_firewall_custom"

  rules {
    action = "block"
    expression = "(http.request.uri.path contains \"/admin\" and ip.geoip.country ne \"CA\")"
    description = "Block admin access from outside Canada"
  }

  rules {
    action = "challenge"
    expression = "(http.request.method eq \"POST\" and rate(5m) > 100)"
    description = "Challenge high-rate POST requests"
  }
}

# Rate Limiting for API Endpoints
resource "cloudflare_rate_limit" "api_rate_limit" {
  zone_id = cloudflare_zone.pravyom_zone.id
  threshold = 1000
  period = 60
  match {
    request {
      url_pattern = "api.pravyom.com/*"
      schemes = ["HTTP", "HTTPS"]
      methods = ["GET", "POST", "PUT", "DELETE"]
    }
  }
  action {
    mode = "simulate"  # Change to "ban" for production
    timeout = 86400
  }
}
```

### **3. Page Rules and Caching**

```hcl
# Caching and Performance Rules
resource "cloudflare_page_rule" "api_cache_bypass" {
  zone_id = cloudflare_zone.pravyom_zone.id
  target = "api.pravyom.com/*"
  priority = 1

  actions {
    cache_level = "bypass"
    browser_cache_ttl = 0
  }
}

resource "cloudflare_page_rule" "static_assets_cache" {
  zone_id = cloudflare_zone.pravyom_zone.id
  target = "pravyom.com/assets/*"
  priority = 2

  actions {
    cache_level = "cache_everything"
    edge_cache_ttl = 86400
    browser_cache_ttl = 86400
  }
}

resource "cloudflare_page_rule" "frontend_cache" {
  zone_id = cloudflare_zone.pravyom_zone.id
  target = "pravyom.com/*"
  priority = 3

  actions {
    cache_level = "standard"
    browser_cache_ttl = 3600
  }
}
```

---

## 🔧 **NGINX CONFIGURATION FOR BSO-K8 INTEGRATION**

### **1. Instance 1 - Frontend/Backend Proxy**

```nginx
# /etc/nginx/sites-available/pravyom-bso-k8
# BPCI Enterprise Server vPod cluster (12 vPods)
upstream bpci_enterprise_vpods {
    least_conn;
    server 127.0.0.1:8081 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:8082 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:8083 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:8084 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:8085 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:8086 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:8087 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:8088 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:8089 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:8090 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:8091 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:8092 weight=1 max_fails=3 fail_timeout=30s;
}

# Management Dashboard vPod cluster (6 vPods)
upstream management_dashboard_vpods {
    least_conn;
    server 127.0.0.1:3001 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:3002 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:3003 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:3004 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:3005 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:3006 weight=1 max_fails=3 fail_timeout=30s;
}

# Vite Frontend vPod cluster (8 vPods)
upstream frontend_vpods {
    least_conn;
    server 127.0.0.1:4001 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:4002 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:4003 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:4004 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:4005 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:4006 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:4007 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:4008 weight=1 max_fails=3 fail_timeout=30s;
}

# Main BPCI Enterprise server block
server {
    listen 80;
    listen [::]:80;
    server_name pravyom.com www.pravyom.com;

    # Security headers for enterprise platform
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Referrer-Policy "strict-origin-when-cross-origin" always;
    add_header X-BPCI-Enterprise "true" always;

    # BPCI Enterprise API routing (12 vPods)
    location /api/ {
        proxy_pass http://bpci_enterprise_vpods;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # Enterprise-grade health checks
        proxy_next_upstream error timeout invalid_header http_500 http_502 http_503;
        proxy_connect_timeout 5s;
        proxy_send_timeout 30s;
        proxy_read_timeout 30s;
    }

    # BPCI Enterprise health endpoint
    location /health {
        proxy_pass http://bpci_enterprise_vpods;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        access_log off;
    }

    # Blockchain statistics endpoint
    location /stats {
        proxy_pass http://bpci_enterprise_vpods;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    # Mining endpoints
    location /mining/ {
        proxy_pass http://bpci_enterprise_vpods;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    # Registry endpoints
    location /registry/ {
        proxy_pass http://bpci_enterprise_vpods;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    # Vite Frontend routing (8 vPods)
    location / {
        proxy_pass http://frontend_vpods;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Static assets with caching
    location /assets/ {
        proxy_pass http://frontend_vpods;
        proxy_cache_valid 200 1h;
        add_header Cache-Control "public, max-age=3600";
    }
}

# Management Dashboard server block
server {
    listen 80;
    server_name management.pravyom.com;

    # Management Dashboard routing (6 vPods)
    location / {
        proxy_pass http://management_dashboard_vpods;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}

# SSL configuration (handled by Cloudflare)
server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name pravyom.com www.pravyom.com;

    # Cloudflare SSL certificates
    ssl_certificate /etc/ssl/certs/pravyom.com.pem;
    ssl_certificate_key /etc/ssl/private/pravyom.com.key;
    
    # Include the same location blocks as above
    include /etc/nginx/snippets/pravyom-locations.conf;
}
```

### **2. Instance 2 - Database Proxy Configuration**

```nginx
# /etc/nginx/sites-available/database-proxy
# PostgreSQL Controller vPod cluster (8 vPods)
upstream postgresql_vpods {
    server 127.0.0.1:5433 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:5434 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:5435 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:5436 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:5437 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:5438 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:5439 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:5440 weight=1 max_fails=3 fail_timeout=30s;
}

# MongoDB Controller vPod cluster (6 vPods)
upstream mongodb_vpods {
    server 127.0.0.1:27018 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:27019 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:27020 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:27021 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:27022 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:27023 weight=1 max_fails=3 fail_timeout=30s;
}

# Redis Cache vPod cluster (4 vPods)
upstream redis_vpods {
    server 127.0.0.1:6380 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:6381 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:6382 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:6383 weight=1 max_fails=3 fail_timeout=30s;
}

# Database Proxy vPod cluster (4 vPods)
upstream database_proxy_vpods {
    server 127.0.0.1:8190 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:8191 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:8192 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:8193 weight=1 max_fails=3 fail_timeout=30s;
}

# Database API proxy (internal only)
server {
    listen 8090;
    server_name db.pravyom.com;

    # PostgreSQL proxy endpoint
    location /postgresql/ {
        proxy_pass http://postgresql_vpods;
        proxy_set_header Host $host;
        proxy_connect_timeout 5s;
        proxy_send_timeout 30s;
        proxy_read_timeout 30s;
    }

    # MongoDB proxy endpoint
    location /mongodb/ {
        proxy_pass http://mongodb_vpods;
        proxy_set_header Host $host;
        proxy_connect_timeout 5s;
        proxy_send_timeout 30s;
        proxy_read_timeout 30s;
    }

    # Database health checks
    location /health {
        access_log off;
        return 200 "Database proxy healthy\n";
        add_header Content-Type text/plain;
    }
}
```

### **3. Instance 3 - BPCI Downloader Services Proxy**

```nginx
# /etc/nginx/sites-available/downloader-proxy
upstream bpci_downloader_vpods {
    server 127.0.0.1:9001 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:9002 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:9003 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:9004 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:9005 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:9006 weight=1 max_fails=3 fail_timeout=30s;
}

upstream bpci_registry_vpods {
    server 127.0.0.1:9101 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:9102 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:9103 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:9104 weight=1 max_fails=3 fail_timeout=30s;
}

server {
    listen 80;
    server_name downloader.pravyom.com;

    # BPCI Downloader service
    location /download/ {
        proxy_pass http://bpci_downloader_vpods;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        
        # Large file download optimization
        proxy_buffering off;
        proxy_request_buffering off;
        client_max_body_size 10G;
    }

    # BPCI Registry service
    location /registry/ {
        proxy_pass http://bpci_registry_vpods;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    # Main downloader interface
    location / {
        proxy_pass http://bpci_downloader_vpods;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

---

## 🚀 **DEPLOYMENT AUTOMATION SCRIPTS**

### **1. Terraform Deployment Script**

```bash
#!/bin/bash
# scripts/deploy-cloudflare-infrastructure.sh

set -e

echo "🌐 Deploying Cloudflare Infrastructure with Terraform"

# Check prerequisites
if ! command -v terraform &> /dev/null; then
    echo "❌ Terraform not installed"
    exit 1
fi

if [ -z "$CLOUDFLARE_API_TOKEN" ]; then
    echo "❌ CLOUDFLARE_API_TOKEN not set"
    exit 1
fi

# Initialize Terraform
cd terraform/
terraform init

# Plan deployment
echo "📋 Planning Terraform deployment..."
terraform plan -var="cloudflare_api_token=$CLOUDFLARE_API_TOKEN" \
               -var="instance_4_ip=$INSTANCE_4_IP" \
               -out=tfplan

# Apply deployment
echo "🚀 Applying Terraform deployment..."
terraform apply tfplan

# Verify DNS propagation
echo "🔍 Verifying DNS propagation..."
for domain in pravyom.com api.pravyom.com bpi.pravyom.com; do
    echo "Checking $domain..."
    dig +short $domain
done

echo "✅ Cloudflare infrastructure deployed successfully!"
```

### **2. NGINX Configuration Deployment**

```bash
#!/bin/bash
# scripts/deploy-nginx-configs.sh

set -e

echo "🔧 Deploying NGINX Configurations for BSO-K8 Integration"

# Instance 1: Frontend/Backend proxy
echo "📦 Deploying Instance 1 NGINX config..."
scp configs/nginx/pravyom-bso-k8.conf root@146.190.74.139:/etc/nginx/sites-available/
ssh root@146.190.74.139 "ln -sf /etc/nginx/sites-available/pravyom-bso-k8.conf /etc/nginx/sites-enabled/"

# Instance 2: Database proxy
echo "📦 Deploying Instance 2 NGINX config..."
scp configs/nginx/database-proxy.conf root@157.230.238.92:/etc/nginx/sites-available/
ssh root@157.230.238.92 "ln -sf /etc/nginx/sites-available/database-proxy.conf /etc/nginx/sites-enabled/"

# Instance 3: BPCI Downloader proxy
echo "📦 Deploying Instance 3 NGINX config..."
scp configs/nginx/downloader-proxy.conf root@142.93.113.141:/etc/nginx/sites-available/
ssh root@142.93.113.141 "ln -sf /etc/nginx/sites-available/downloader-proxy.conf /etc/nginx/sites-enabled/"

# Test configurations
echo "🧪 Testing NGINX configurations..."
for instance in 146.190.74.139 157.230.238.92 142.93.113.141; do
    echo "Testing $instance..."
    ssh root@$instance "nginx -t && systemctl reload nginx"
done

echo "✅ NGINX configurations deployed successfully!"
```

---

## 📊 **MONITORING AND ANALYTICS**

### **1. Cloudflare Analytics Integration**

```hcl
# terraform/analytics.tf
resource "cloudflare_logpush_job" "pravyom_logs" {
  zone_id = cloudflare_zone.pravyom_zone.id
  name = "pravyom-http-logs"
  logpull_options = "fields=RayID,EdgeStartTimestamp,EdgeEndTimestamp,ClientIP,ClientRequestHost,ClientRequestMethod,ClientRequestURI,EdgeResponseStatus,EdgeResponseBytes,ClientCountry&timestamps=rfc3339"
  destination_conf = "s3://pravyom-logs/cloudflare/?region=us-east-1"
  dataset = "http_requests"
  enabled = true
}

# Web Analytics
resource "cloudflare_web_analytics_site" "pravyom_analytics" {
  zone_tag = cloudflare_zone.pravyom_zone.id
  auto_install = true
}
```

### **2. Performance Monitoring**

```bash
#!/bin/bash
# scripts/monitor-proxy-performance.sh

echo "📊 Monitoring Proxy Performance"

# Test response times
for endpoint in "https://pravyom.com" "https://api.pravyom.com/health" "https://downloader.pravyom.com" "https://bpi.pravyom.com"; do
    echo "Testing $endpoint..."
    curl -w "Response time: %{time_total}s\nHTTP status: %{http_code}\n" -o /dev/null -s "$endpoint"
    echo "---"
done

# Test load balancing distribution
echo "🔄 Testing load balancing distribution..."
for i in {1..10}; do
    curl -s https://api.pravyom.com/health | grep -o "vpod-[0-9]*"
done | sort | uniq -c

echo "✅ Performance monitoring complete"
```

---

## 🎯 **IMPLEMENTATION ROADMAP**

### **Phase 1: Terraform Infrastructure (Day 1)**
- [ ] Set up Cloudflare account and API tokens
- [ ] Configure Terraform with Cloudflare provider
- [ ] Deploy DNS records and security rules
- [ ] Verify DNS propagation

### **Phase 2: NGINX Configuration (Day 2)**
- [ ] Deploy NGINX configs to all instances
- [ ] Configure upstream vPod pools
- [ ] Test load balancing and health checks
- [ ] Validate SSL termination

### **Phase 3: BSO-K8 Integration (Day 3)**
- [ ] Configure BSO-K8 vPod port assignments
- [ ] Update NGINX upstream configurations
- [ ] Test vPod auto-discovery
- [ ] Validate failover mechanisms

### **Phase 4: Monitoring & Analytics (Day 4)**
- [ ] Set up Cloudflare analytics
- [ ] Configure log shipping
- [ ] Deploy performance monitoring
- [ ] Set up alerting rules

### **Phase 5: Optimization (Day 5)**
- [ ] Tune caching rules
- [ ] Optimize security settings
- [ ] Performance testing and tuning
- [ ] Documentation and handover

This comprehensive plan modernizes the proxy infrastructure with Cloudflare and Terraform while maintaining seamless integration with BSO-K8 native vPods.
