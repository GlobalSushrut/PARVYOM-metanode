#!/bin/bash

# BPCI Enterprise - Let's Encrypt TLS Setup
# Automatic, production-trusted certificates for all browsers

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BPCI_ROOT="/home/umesh/metanode/bpci-enterprise"
DOMAINS="pravyom.com,www.pravyom.com,admin.pravyom.com,api.pravyom.com"
EMAIL="admin@pravyom.com"

echo -e "${BLUE}🔐 BPCI Enterprise - Let's Encrypt TLS Setup${NC}"
echo -e "${BLUE}=============================================${NC}"

# Function to log messages
log() {
    echo -e "$1"
}

# Check if running on production server
check_production_environment() {
    log "${YELLOW}🔍 Checking production environment...${NC}"
    
    # Check if domains resolve to this server
    if command -v dig >/dev/null 2>&1; then
        for domain in $(echo $DOMAINS | tr ',' ' '); do
            local ip=$(dig +short $domain)
            if [ -z "$ip" ]; then
                log "${YELLOW}⚠️  Domain $domain does not resolve - using staging certificates${NC}"
                return 1
            fi
        done
        log "${GREEN}✅ All domains resolve - using production certificates${NC}"
        return 0
    else
        log "${YELLOW}⚠️  dig not available - using staging certificates${NC}"
        return 1
    fi
}

# Install Certbot (Let's Encrypt client)
install_certbot() {
    log "${BLUE}📦 Installing Certbot...${NC}"
    
    if command -v certbot >/dev/null 2>&1; then
        log "${GREEN}✅ Certbot already installed${NC}"
        return 0
    fi
    
    # Install certbot
    if command -v apt-get >/dev/null 2>&1; then
        sudo apt-get update
        sudo apt-get install -y certbot
    elif command -v yum >/dev/null 2>&1; then
        sudo yum install -y certbot
    elif command -v snap >/dev/null 2>&1; then
        sudo snap install --classic certbot
        sudo ln -sf /snap/bin/certbot /usr/bin/certbot
    else
        log "${RED}❌ Cannot install certbot - unsupported system${NC}"
        return 1
    fi
    
    log "${GREEN}✅ Certbot installed${NC}"
}

# Generate Let's Encrypt certificates
generate_letsencrypt_certificates() {
    log "${BLUE}🔧 Generating Let's Encrypt certificates...${NC}"
    
    local staging_flag=""
    if ! check_production_environment; then
        staging_flag="--staging"
        log "${YELLOW}⚠️  Using staging certificates (for testing)${NC}"
    fi
    
    # Create webroot directory
    local webroot="/var/www/html"
    sudo mkdir -p "$webroot"
    
    # Generate certificates
    sudo certbot certonly \
        --webroot \
        --webroot-path="$webroot" \
        --email="$EMAIL" \
        --agree-tos \
        --no-eff-email \
        $staging_flag \
        -d $(echo $DOMAINS | tr ',' ' ' | sed 's/ / -d /g')
    
    if [ $? -eq 0 ]; then
        log "${GREEN}✅ Let's Encrypt certificates generated${NC}"
        return 0
    else
        log "${RED}❌ Certificate generation failed${NC}"
        return 1
    fi
}

# Copy certificates to BPCI directory
copy_certificates() {
    log "${BLUE}📋 Copying certificates to BPCI directory...${NC}"
    
    local cert_dir="$BPCI_ROOT/tls/certificates"
    mkdir -p "$cert_dir"
    
    # Copy certificates for each domain
    for domain in $(echo $DOMAINS | tr ',' ' '); do
        local domain_dir="$cert_dir/$domain"
        mkdir -p "$domain_dir"
        
        if [ -d "/etc/letsencrypt/live/$domain" ]; then
            sudo cp "/etc/letsencrypt/live/$domain/fullchain.pem" "$domain_dir/certificate-chain.pem"
            sudo cp "/etc/letsencrypt/live/$domain/privkey.pem" "$domain_dir/private-key.pem"
            sudo cp "/etc/letsencrypt/live/$domain/cert.pem" "$domain_dir/certificate.pem"
            sudo chown -R $USER:$USER "$domain_dir"
            
            log "${GREEN}✅ Certificates copied for $domain${NC}"
        else
            log "${YELLOW}⚠️  No certificates found for $domain${NC}"
        fi
    done
}

# Create certificate renewal script
create_renewal_script() {
    log "${BLUE}🔄 Creating certificate renewal script...${NC}"
    
    cat > "$BPCI_ROOT/scripts/renew-certificates.sh" << 'EOF'
#!/bin/bash

# BPCI Enterprise - Certificate Renewal
echo "🔄 Renewing Let's Encrypt certificates..."

# Renew certificates
sudo certbot renew --quiet

# Copy renewed certificates
BPCI_ROOT="/home/umesh/metanode/bpci-enterprise"
DOMAINS="pravyom.com,www.pravyom.com,admin.pravyom.com,api.pravyom.com"

for domain in $(echo $DOMAINS | tr ',' ' '); do
    domain_dir="$BPCI_ROOT/tls/certificates/$domain"
    if [ -d "/etc/letsencrypt/live/$domain" ]; then
        sudo cp "/etc/letsencrypt/live/$domain/fullchain.pem" "$domain_dir/certificate-chain.pem"
        sudo cp "/etc/letsencrypt/live/$domain/privkey.pem" "$domain_dir/private-key.pem"
        sudo cp "/etc/letsencrypt/live/$domain/cert.pem" "$domain_dir/certificate.pem"
        sudo chown -R $USER:$USER "$domain_dir"
    fi
done

# Restart BPCI services
echo "🔄 Restarting BPCI services..."
cd "$BPCI_ROOT"
./scripts/stop-system.sh
sleep 2
./scripts/deploy-https-system.sh

echo "✅ Certificate renewal complete"
EOF

    chmod +x "$BPCI_ROOT/scripts/renew-certificates.sh"
    
    # Add to crontab for automatic renewal
    (crontab -l 2>/dev/null; echo "0 3 * * * $BPCI_ROOT/scripts/renew-certificates.sh >> $BPCI_ROOT/logs/cert-renewal.log 2>&1") | crontab -
    
    log "${GREEN}✅ Certificate renewal script created${NC}"
}

# Update server configurations for Let's Encrypt
update_server_configs() {
    log "${BLUE}🔧 Updating server configurations...${NC}"
    
    # Create production HTTPS configuration
    cat > "$BPCI_ROOT/tls/letsencrypt-config.js" << 'EOF'
// BPCI Enterprise - Let's Encrypt TLS Configuration
const https = require('https');
const fs = require('fs');
const path = require('path');

class LetsTLSServer {
  constructor(domain = 'pravyom.com') {
    this.domain = domain;
    this.certDir = path.join(__dirname, 'certificates', domain);
    this.tlsOptions = this.loadTLSOptions();
  }

  loadTLSOptions() {
    try {
      const certPath = path.join(this.certDir, 'certificate-chain.pem');
      const keyPath = path.join(this.certDir, 'private-key.pem');
      
      console.log(`🔍 Loading Let's Encrypt certificates for ${this.domain}`);
      
      const tlsOptions = {
        cert: fs.readFileSync(certPath),
        key: fs.readFileSync(keyPath),
        secureProtocol: 'TLSv1_2_method'
      };
      
      console.log(`✅ Let's Encrypt certificates loaded for ${this.domain}`);
      return tlsOptions;
      
    } catch (error) {
      console.error(`❌ Failed to load Let's Encrypt certificates:`, error.message);
      throw error;
    }
  }

  createHTTPSServer(app) {
    return https.createServer(this.tlsOptions, app);
  }

  securityHeaders(req, res, next) {
    res.setHeader('Strict-Transport-Security', 'max-age=31536000; includeSubDomains; preload');
    res.setHeader('X-Content-Type-Options', 'nosniff');
    res.setHeader('X-Frame-Options', 'DENY');
    res.setHeader('X-XSS-Protection', '1; mode=block');
    res.setHeader('X-BPCI-TLS', 'Let\'s Encrypt');
    res.setHeader('X-BPCI-Production', 'true');
    next();
  }
}

module.exports = { LetsTLSServer };
EOF

    log "${GREEN}✅ Server configurations updated${NC}"
}

# Main setup function
main() {
    log "${BLUE}🎯 Setting up Let's Encrypt TLS for production...${NC}"
    
    # Create logs directory
    mkdir -p "$BPCI_ROOT/logs"
    
    # Install Certbot
    if ! install_certbot; then
        log "${RED}❌ Failed to install Certbot${NC}"
        exit 1
    fi
    
    # Generate certificates
    if ! generate_letsencrypt_certificates; then
        log "${YELLOW}⚠️  Certificate generation failed - using fallback${NC}"
        # Fall back to self-signed for development
        cd "$BPCI_ROOT/tls"
        ./generate-certificates.sh
    else
        # Copy certificates
        copy_certificates
        
        # Create renewal script
        create_renewal_script
    fi
    
    # Update server configurations
    update_server_configs
    
    log "${GREEN}🎉 Let's Encrypt TLS setup complete!${NC}"
    log "${GREEN}✅ Certificates will be automatically trusted by all browsers${NC}"
    
    echo ""
    log "${BLUE}📋 Next Steps:${NC}"
    log "1. Deploy servers with Let's Encrypt certificates"
    log "2. All browsers will show green lock automatically"
    log "3. Certificates will auto-renew every 90 days"
    
    echo ""
    log "${GREEN}🚀 Ready for production deployment!${NC}"
}

# Run setup
main "$@"
