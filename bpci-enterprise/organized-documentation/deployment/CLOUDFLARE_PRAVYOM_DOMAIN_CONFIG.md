# Cloudflare + pravyom.com Domain Configuration

## Overview

This document outlines the production domain configuration using **Cloudflare for SSL/DNS** with **pravyom.com** handling both the static Vite website (HTTP server entrypoint) and HTTPCG protocol routing.

## 1. Domain Architecture

### 1.1 Production Domain Setup

**Primary Domain**: `pravyom.com`
- **SSL/TLS**: Managed by Cloudflare (Full Strict)
- **DNS**: Cloudflare DNS management
- **CDN**: Global edge caching via Cloudflare
- **DDoS Protection**: Enterprise-grade Cloudflare protection

### 1.2 Dual Purpose Configuration

The `pravyom.com` domain serves two purposes:

1. **Static Vite Website** (HTTP Server Entrypoint)
   - Route: `https://pravyom.com/` → Vite website
   - Purpose: Main website, dashboard, wallet interface
   - Technology: Vite + React/Vue frontend

2. **HTTPCG Protocol Handler**
   - Route: `https://pravyom.com/httpcg/` → HTTPCG routing engine
   - Purpose: Hash-based protocol routing
   - Technology: Rust VM server backend

## 2. Cloudflare Configuration

### 2.1 DNS Records

```yaml
# Cloudflare DNS Configuration for pravyom.com
DNS Records:
  - Type: A
    Name: pravyom.com
    Content: <DIGITAL_OCEAN_IP>
    Proxy: Enabled (Orange Cloud)
    TTL: Auto
  
  - Type: A
    Name: www
    Content: <DIGITAL_OCEAN_IP>
    Proxy: Enabled (Orange Cloud)
    TTL: Auto
  
  - Type: CNAME
    Name: api
    Content: pravyom.com
    Proxy: Enabled (Orange Cloud)
    TTL: Auto
```

### 2.2 SSL/TLS Settings

```yaml
# Cloudflare SSL/TLS Configuration
SSL/TLS:
  Encryption Mode: Full (strict)
  Edge Certificates: Universal SSL enabled
  Origin Server: SSL certificate required
  TLS Version: TLS 1.3 minimum
  HSTS: Enabled (max-age=31536000)
  Certificate Transparency: Enabled
```

### 2.3 Page Rules

```yaml
# Cloudflare Page Rules for pravyom.com
Page Rules:
  # Static website caching
  - URL: pravyom.com/*
    Settings:
      - SSL: Full (strict)
      - Cache Level: Standard
      - Browser Cache TTL: 4 hours
      - Edge Cache TTL: 2 hours
  
  # HTTPCG protocol - no caching
  - URL: pravyom.com/httpcg/*
    Settings:
      - SSL: Full (strict)
      - Cache Level: Bypass
      - Disable Apps: On
      - Disable Performance: On
  
  # API endpoints - minimal caching
  - URL: pravyom.com/api/*
    Settings:
      - SSL: Full (strict)
      - Cache Level: Bypass
      - Browser Cache TTL: 30 minutes
```

### 2.4 Security Settings

```yaml
# Cloudflare Security Configuration
Security:
  Security Level: Medium
  Challenge Passage: 30 minutes
  Browser Integrity Check: Enabled
  Privacy Pass: Enabled
  
  # DDoS Protection
  DDoS Protection: Enabled
  Rate Limiting: Custom rules for /httpcg/ endpoints
  
  # Firewall Rules
  Firewall Rules:
    - Block known bad IPs
    - Rate limit /httpcg/ to 100 req/min per IP
    - Allow legitimate HTTPCG protocol traffic
```

## 3. Origin Server Configuration

### 3.1 Digital Ocean Droplet Setup

**Server Specs**:
- **CPU**: 2 cores
- **RAM**: 4GB
- **Storage**: 80GB SSD
- **Network**: 4TB transfer
- **Cost**: ~$24/month

### 3.2 Nginx Configuration

```nginx
# /etc/nginx/sites-available/pravyom.com
server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name pravyom.com www.pravyom.com;
    
    # SSL certificates (for Cloudflare origin)
    ssl_certificate /etc/ssl/certs/pravyom.com.pem;
    ssl_certificate_key /etc/ssl/private/pravyom.com.key;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES256-GCM-SHA512:DHE-RSA-AES256-GCM-SHA512;
    
    # Cloudflare real IP restoration
    set_real_ip_from 103.21.244.0/22;
    set_real_ip_from 103.22.200.0/22;
    set_real_ip_from 103.31.4.0/22;
    set_real_ip_from 104.16.0.0/13;
    set_real_ip_from 104.24.0.0/14;
    set_real_ip_from 108.162.192.0/18;
    set_real_ip_from 131.0.72.0/22;
    set_real_ip_from 141.101.64.0/18;
    set_real_ip_from 162.158.0.0/15;
    set_real_ip_from 172.64.0.0/13;
    set_real_ip_from 173.245.48.0/20;
    set_real_ip_from 188.114.96.0/20;
    set_real_ip_from 190.93.240.0/20;
    set_real_ip_from 197.234.240.0/22;
    set_real_ip_from 198.41.128.0/17;
    real_ip_header CF-Connecting-IP;
    
    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Referrer-Policy "strict-origin-when-cross-origin" always;
    
    # Static Vite website (HTTP server entrypoint)
    location / {
        root /var/www/pravyom-vite/dist;
        try_files $uri $uri/ /index.html;
        
        # Cache static assets
        location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot)$ {
            expires 1y;
            add_header Cache-Control "public, immutable";
            add_header Vary "Accept-Encoding";
        }
        
        # HTML files - short cache
        location ~* \.html$ {
            expires 1h;
            add_header Cache-Control "public, must-revalidate";
        }
    }
    
    # HTTPCG protocol handler - Same domain handles HTTPCG routing
    location /httpcg/ {
        proxy_pass http://127.0.0.1:7777;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # Cloudflare headers
        proxy_set_header CF-Connecting-IP $http_cf_connecting_ip;
        proxy_set_header CF-Ray $http_cf_ray;
        proxy_set_header CF-Visitor $http_cf_visitor;
        
        # HTTPCG-specific headers
        proxy_set_header X-HTTPCG-Plane $arg_plane;
        proxy_set_header X-HTTPCG-Domain $arg_domain;
        proxy_set_header X-HTTPCG-Hash $arg_hash;
        
        # Disable caching for HTTPCG requests
        add_header Cache-Control "no-cache, no-store, must-revalidate";
        add_header Pragma "no-cache";
        add_header Expires "0";
        
        # WebSocket support for XTMP protocol
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_read_timeout 86400;
    }
    
    # API endpoints
    location /api/ {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header CF-Connecting-IP $http_cf_connecting_ip;
        
        # API rate limiting
        limit_req zone=api_limit burst=20 nodelay;
    }
}

# HTTP to HTTPS redirect
server {
    listen 80;
    listen [::]:80;
    server_name pravyom.com www.pravyom.com;
    return 301 https://$server_name$request_uri;
}

# Rate limiting zones
http {
    limit_req_zone $binary_remote_addr zone=api_limit:10m rate=10r/s;
}
```

## 4. HTTPCG Protocol Integration

### 4.1 JavaScript Client (Vite Website)

```javascript
// src/utils/httpcg-client.js
class HttpcgClient {
    constructor(gateway = 'https://pravyom.com') {
        this.gateway = gateway;
        this.cloudflareEnabled = true;
    }
    
    async request(plane, domain, path, options = {}) {
        // Use pravyom.com as the domain for all HTTPCG requests
        const httpcgDomain = 'pravyom.com';
        const hash = options.hash || this.generateHash(plane, httpcgDomain, path);
        const url = `${this.gateway}/httpcg/?plane=${plane}&domain=${httpcgDomain}&path=${encodeURIComponent(path)}&hash=${hash}`;
        
        const response = await fetch(url, {
            method: options.method || 'GET',
            headers: {
                'Content-Type': 'application/json',
                'X-HTTPCG-Protocol': '1.0',
                'X-HTTPCG-Domain': httpcgDomain,
                ...options.headers
            },
            body: options.body ? JSON.stringify(options.body) : undefined
        });
        
        if (!response.ok) {
            throw new Error(`HTTPCG request failed: ${response.status}`);
        }
        
        return response.json();
    }
    
    generateHash(plane, domain, path) {
        const data = `${plane}/${domain}/${path}`;
        return btoa(data).replace(/[+/=]/g, '').substring(0, 16);
    }
    
    // Convenience methods for different planes
    async appRequest(path, options = {}) {
        return this.request('app', 'pravyom.com', path, options);
    }
    
    async secureRequest(path, options = {}) {
        return this.request('secure', 'pravyom.com', path, options);
    }
    
    async govRequest(path, options = {}) {
        return this.request('gov', 'pravyom.com', path, options);
    }
}

export default HttpcgClient;
```

### 4.2 Vite Website Integration

```javascript
// src/main.js
import { createApp } from 'vue'
import App from './App.vue'
import HttpcgClient from './utils/httpcg-client.js'

const app = createApp(App)

// Global HTTPCG client
app.config.globalProperties.$httpcg = new HttpcgClient()

// Example usage in components
app.mount('#app')
```

```vue
<!-- src/components/Dashboard.vue -->
<template>
  <div class="dashboard">
    <h1>Pravyom Dashboard</h1>
    <button @click="testHttpcg">Test HTTPCG Protocol</button>
    <div v-if="httpcgResult">{{ httpcgResult }}</div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      httpcgResult: null
    }
  },
  methods: {
    async testHttpcg() {
      try {
        // Test HTTPCG protocol on same domain
        const result = await this.$httpcg.appRequest('/dashboard/status');
        this.httpcgResult = result;
      } catch (error) {
        console.error('HTTPCG request failed:', error);
        this.httpcgResult = { error: error.message };
      }
    }
  }
}
</script>
```

## 5. Deployment Process

### 5.1 Cloudflare Setup

```bash
# 1. Add domain to Cloudflare
# - Go to Cloudflare dashboard
# - Add site: pravyom.com
# - Update nameservers at domain registrar

# 2. Configure DNS records
# - A record: pravyom.com → <DIGITAL_OCEAN_IP>
# - A record: www → <DIGITAL_OCEAN_IP>
# - CNAME: api → pravyom.com

# 3. Enable SSL/TLS Full (strict)
# 4. Configure page rules as specified above
# 5. Set up firewall rules for HTTPCG protection
```

### 5.2 Digital Ocean Server Setup

```bash
# 1. Create droplet (Ubuntu 22.04, 2CPU/4GB)
# 2. Install dependencies
sudo apt update && sudo apt upgrade -y
sudo apt install nginx nodejs npm git certbot python3-certbot-nginx -y

# 3. Install Rust and build BPI Core
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
git clone <BPI_CORE_REPO>
cd bpi-core && cargo build --release

# 4. Deploy Vite website
git clone <VITE_WEBSITE_REPO>
cd pravyom-vite && npm install && npm run build
sudo cp -r dist/* /var/www/pravyom-vite/dist/

# 5. Configure Nginx
sudo cp nginx.conf /etc/nginx/sites-available/pravyom.com
sudo ln -s /etc/nginx/sites-available/pravyom.com /etc/nginx/sites-enabled/
sudo nginx -t && sudo systemctl reload nginx

# 6. Generate origin SSL certificate for Cloudflare
sudo certbot certonly --nginx -d pravyom.com -d www.pravyom.com

# 7. Start BPI services
./target/release/bpi-core --vm-server --port 7777 &
./target/release/bpci-enterprise --api-server --port 8080 &
```

## 6. Monitoring and Maintenance

### 6.1 Cloudflare Analytics

Monitor:
- **Traffic patterns** (HTTP vs HTTPCG requests)
- **Cache hit rates** (static assets vs dynamic content)
- **Security events** (blocked requests, rate limiting)
- **Performance metrics** (response times, bandwidth)

### 6.2 Server Monitoring

```bash
# Monitor BPI services
sudo systemctl status bpi-core
sudo systemctl status bpci-enterprise

# Check logs
tail -f /var/log/bpi-core.log
tail -f /var/log/bpci-enterprise.log
tail -f /var/log/nginx/access.log
tail -f /var/log/nginx/error.log

# Resource monitoring
htop
df -h
free -h
```

## 7. Cost Breakdown

### 7.1 Monthly Costs

```
Cloudflare:
- Free Plan: $0/month (sufficient for basic needs)
- Pro Plan: $20/month (recommended for production)

Digital Ocean:
- Droplet (2CPU/4GB): $24/month
- Managed Database: $15/month (optional)
- Spaces Storage: $5/month (optional)
- Load Balancer: $12/month (optional)

Total: $24-76/month depending on features
```

## 8. Benefits of This Configuration

### 8.1 Advantages

✅ **Single Domain**: Simplified management with pravyom.com handling both website and HTTPCG
✅ **Cloudflare Protection**: Enterprise-grade DDoS protection and SSL
✅ **Global CDN**: Fast static asset delivery worldwide
✅ **Cost Effective**: Minimal infrastructure costs
✅ **Scalable**: Can handle high traffic with Cloudflare caching
✅ **Secure**: Full SSL encryption and security headers
✅ **Real Implementation**: Matches the actual codebase architecture

### 8.2 HTTPCG Protocol Benefits

✅ **Hash-based Routing**: Quantum-safe session management
✅ **Multi-plane Architecture**: Secure separation of app/secure/gov/dark planes
✅ **Dynamic Domains**: Unlimited virtual domains via registry
✅ **Wallet Integration**: Direct BPI wallet authentication
✅ **Post-quantum Security**: Future-proof cryptographic protection

This configuration provides a production-ready deployment of the sophisticated BPI-BPCI system with proper domain management, SSL security, and HTTPCG protocol support, all managed through Cloudflare's robust infrastructure.
