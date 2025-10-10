#!/bin/bash

# BPCI Enterprise Custom TLS Certificate Generation
# Generate self-signed certificates for secure HTTPS deployment

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
CERT_DIR="/home/umesh/metanode/bpci-enterprise/tls/certificates"
KEY_SIZE=4096
CERT_VALIDITY_DAYS=3650  # 10 years
COUNTRY="US"
STATE="California"
CITY="Silicon Valley"
ORGANIZATION="BPCI Enterprise"
ORGANIZATIONAL_UNIT="Security Department"

# Domain configuration
DOMAINS=(
    "pravyom.com"
    "www.pravyom.com"
    "admin.pravyom.com"
    "api.pravyom.com"
    "localhost"
)

echo -e "${BLUE}🔐 BPCI Enterprise TLS Certificate Generation${NC}"
echo -e "${BLUE}===========================================${NC}"

# Function to log messages
log() {
    echo -e "$1"
}

# Create certificate directory
create_cert_directory() {
    log "${YELLOW}📁 Creating certificate directory...${NC}"
    
    mkdir -p "$CERT_DIR"
    mkdir -p "$CERT_DIR/ca"
    mkdir -p "$CERT_DIR/server"
    mkdir -p "$CERT_DIR/client"
    
    # Set proper permissions
    chmod 700 "$CERT_DIR"
    chmod 700 "$CERT_DIR/ca"
    chmod 700 "$CERT_DIR/server"
    chmod 700 "$CERT_DIR/client"
    
    log "${GREEN}✅ Certificate directory created${NC}"
}

# Generate Certificate Authority (CA)
generate_ca() {
    log "${YELLOW}🏛️ Generating Certificate Authority...${NC}"
    
    cd "$CERT_DIR/ca"
    
    # Generate CA private key
    openssl genrsa -out ca-private-key.pem $KEY_SIZE
    chmod 600 ca-private-key.pem
    
    # Generate CA certificate
    openssl req -new -x509 -days $CERT_VALIDITY_DAYS -key ca-private-key.pem -out ca-certificate.pem -subj "/C=$COUNTRY/ST=$STATE/L=$CITY/O=$ORGANIZATION/OU=$ORGANIZATIONAL_UNIT/CN=BPCI Enterprise Root CA"
    
    # Generate CA certificate in DER format for browsers
    openssl x509 -outform DER -in ca-certificate.pem -out ca-certificate.der
    
    log "${GREEN}✅ Certificate Authority generated${NC}"
}

# Generate server certificates for each domain
generate_server_certificates() {
    log "${YELLOW}🌐 Generating server certificates...${NC}"
    
    cd "$CERT_DIR/server"
    
    for domain in "${DOMAINS[@]}"; do
        log "${BLUE}Generating certificate for: $domain${NC}"
        
        # Create domain-specific directory
        mkdir -p "$domain"
        cd "$domain"
        
        # Generate private key
        openssl genrsa -out private-key.pem $KEY_SIZE
        chmod 600 private-key.pem
        
        # Create certificate signing request (CSR)
        openssl req -new -key private-key.pem -out certificate.csr -subj "/C=$COUNTRY/ST=$STATE/L=$CITY/O=$ORGANIZATION/OU=$ORGANIZATIONAL_UNIT/CN=$domain"
        
        # Create certificate extensions file
        cat > certificate.ext << EOF
authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
keyUsage = digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
subjectAltName = @alt_names

[alt_names]
DNS.1 = $domain
DNS.2 = *.$domain
IP.1 = 127.0.0.1
IP.2 = ::1
EOF

        # Generate certificate signed by our CA
        openssl x509 -req -in certificate.csr -CA ../../ca/ca-certificate.pem -CAkey ../../ca/ca-private-key.pem -CAcreateserial -out certificate.pem -days $CERT_VALIDITY_DAYS -extensions v3_req -extfile certificate.ext
        
        # Generate certificate chain (certificate + CA)
        cat certificate.pem ../../ca/ca-certificate.pem > certificate-chain.pem
        
        # Generate PKCS#12 format for some applications
        openssl pkcs12 -export -out certificate.p12 -inkey private-key.pem -in certificate.pem -certfile ../../ca/ca-certificate.pem -password pass:bpci-enterprise
        
        log "${GREEN}✅ Certificate generated for $domain${NC}"
        
        cd ..
    done
}

# Generate client certificates for authentication
generate_client_certificates() {
    log "${YELLOW}👤 Generating client certificates...${NC}"
    
    cd "$CERT_DIR/client"
    
    # Admin client certificate
    log "${BLUE}Generating admin client certificate...${NC}"
    
    openssl genrsa -out admin-private-key.pem $KEY_SIZE
    chmod 600 admin-private-key.pem
    
    openssl req -new -key admin-private-key.pem -out admin-certificate.csr -subj "/C=$COUNTRY/ST=$STATE/L=$CITY/O=$ORGANIZATION/OU=$ORGANIZATIONAL_UNIT/CN=BPCI Admin Client"
    
    openssl x509 -req -in admin-certificate.csr -CA ../ca/ca-certificate.pem -CAkey ../ca/ca-private-key.pem -CAcreateserial -out admin-certificate.pem -days $CERT_VALIDITY_DAYS
    
    # Generate PKCS#12 for browser import
    openssl pkcs12 -export -out admin-certificate.p12 -inkey admin-private-key.pem -in admin-certificate.pem -certfile ../ca/ca-certificate.pem -password pass:bpci-admin
    
    log "${GREEN}✅ Client certificates generated${NC}"
}

# Generate Diffie-Hellman parameters for enhanced security
generate_dhparam() {
    log "${YELLOW}🔒 Generating Diffie-Hellman parameters...${NC}"
    
    cd "$CERT_DIR"
    
    # Generate strong DH parameters (this may take a while)
    openssl dhparam -out dhparam.pem 2048
    
    log "${GREEN}✅ Diffie-Hellman parameters generated${NC}"
}

# Create certificate information file
create_certificate_info() {
    log "${YELLOW}📋 Creating certificate information file...${NC}"
    
    cd "$CERT_DIR"
    
    cat > certificate-info.txt << EOF
BPCI Enterprise TLS Certificate Information
==========================================

Generated: $(date)
Validity: $CERT_VALIDITY_DAYS days ($(date -d "+$CERT_VALIDITY_DAYS days"))
Key Size: $KEY_SIZE bits
Organization: $ORGANIZATION

Certificate Authority:
- Location: ca/ca-certificate.pem
- Private Key: ca/ca-private-key.pem (KEEP SECURE)
- Browser Import: ca/ca-certificate.der

Server Certificates Generated:
EOF

    for domain in "${DOMAINS[@]}"; do
        echo "- $domain: server/$domain/certificate.pem" >> certificate-info.txt
    done
    
    cat >> certificate-info.txt << EOF

Client Certificates:
- Admin Client: client/admin-certificate.pem
- Admin PKCS#12: client/admin-certificate.p12 (Password: bpci-admin)

Security Features:
- Self-signed CA for complete control
- 4096-bit RSA keys for maximum security
- 10-year validity for long-term use
- Subject Alternative Names (SAN) for flexibility
- Certificate chains for proper validation
- Diffie-Hellman parameters for perfect forward secrecy

Browser Installation:
1. Import ca/ca-certificate.der as a trusted root certificate
2. Import client/admin-certificate.p12 for client authentication
3. Restart browser to apply changes

Server Configuration:
- Use certificate-chain.pem for server certificate
- Use private-key.pem for server private key
- Use dhparam.pem for Diffie-Hellman parameters
EOF

    log "${GREEN}✅ Certificate information file created${NC}"
}

# Display certificate summary
display_summary() {
    log "${BLUE}📊 Certificate Generation Summary${NC}"
    log "${BLUE}================================${NC}"
    
    log "${GREEN}✅ Certificate Authority: Generated${NC}"
    log "${GREEN}✅ Server Certificates: ${#DOMAINS[@]} domains${NC}"
    log "${GREEN}✅ Client Certificates: Admin client${NC}"
    log "${GREEN}✅ DH Parameters: Generated${NC}"
    log "${GREEN}✅ Certificate Info: Created${NC}"
    
    echo ""
    log "${YELLOW}📁 Certificate Directory: $CERT_DIR${NC}"
    log "${YELLOW}📋 Certificate Info: $CERT_DIR/certificate-info.txt${NC}"
    
    echo ""
    log "${BLUE}🔐 Security Status:${NC}"
    log "   • All certificates use 4096-bit RSA keys"
    log "   • 10-year validity period"
    log "   • Self-signed CA for complete control"
    log "   • Perfect Forward Secrecy enabled"
    
    echo ""
    log "${BLUE}🌐 Next Steps:${NC}"
    log "   1. Configure servers to use generated certificates"
    log "   2. Import CA certificate in browsers"
    log "   3. Update deployment configuration"
    log "   4. Test HTTPS connections"
    
    echo ""
    log "${GREEN}🎯 TLS Certificate generation completed successfully!${NC}"
}

# Main function
main() {
    create_cert_directory
    generate_ca
    generate_server_certificates
    generate_client_certificates
    generate_dhparam
    create_certificate_info
    display_summary
}

# Handle script interruption
cleanup() {
    log "${YELLOW}🛑 Certificate generation interrupted${NC}"
    exit 1
}

trap cleanup INT TERM

# Check if running as main script
if [ "${BASH_SOURCE[0]}" == "${0}" ]; then
    main "$@"
fi
