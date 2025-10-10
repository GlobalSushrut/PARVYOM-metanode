// BPCI Enterprise HTTPS Server Configuration
// Custom TLS setup for green lock "Secure" status

const https = require('https');
const http = require('http');
const fs = require('fs');
const path = require('path');

class BPCITLSServer {
  constructor(domain = 'localhost') {
    this.domain = domain;
    this.certDir = path.join(__dirname, 'certificates');
    this.tlsOptions = this.loadTLSOptions();
  }

  // Load TLS certificates and options
  loadTLSOptions() {
    try {
      // Load server certificates
      const serverCertDir = path.join(this.certDir, this.domain);
      const certPath = path.join(serverCertDir, 'certificate-chain.pem');
      const keyPath = path.join(serverCertDir, 'private-key.pem');
      
      console.log(`🔍 Loading TLS certificates for ${this.domain}:`);
      console.log(`   Certificate: ${certPath}`);
      console.log(`   Private Key: ${keyPath}`);
      
      const tlsOptions = {
        // Server certificate and key
        cert: fs.readFileSync(certPath),
        key: fs.readFileSync(keyPath),
        
        // Certificate Authority
        ca: fs.readFileSync(path.join(this.certDir, 'ca-certificate.pem')),
        
        // Diffie-Hellman parameters for Perfect Forward Secrecy
        dhparam: fs.readFileSync(path.join(this.certDir, 'dhparam.pem')),
        
        // Security settings
        secureProtocol: 'TLSv1_2_method',
        honorCipherOrder: true,
        ciphers: [
          'ECDHE-RSA-AES256-GCM-SHA384',
          'ECDHE-RSA-AES128-GCM-SHA256',
          'ECDHE-RSA-AES256-SHA384',
          'ECDHE-RSA-AES128-SHA256',
          'ECDHE-RSA-AES256-SHA',
          'ECDHE-RSA-AES128-SHA',
          'AES256-GCM-SHA384',
          'AES128-GCM-SHA256',
          'AES256-SHA256',
          'AES128-SHA256',
          'AES256-SHA',
          'AES128-SHA'
        ].join(':'),
        
        // Additional security options
        secureOptions: require('constants').SSL_OP_NO_SSLv2 |
                       require('constants').SSL_OP_NO_SSLv3 |
                       require('constants').SSL_OP_NO_TLSv1 |
                       require('constants').SSL_OP_NO_TLSv1_1
      };
      
      console.log(`✅ TLS certificates loaded for domain: ${this.domain}`);
      return tlsOptions;
      
    } catch (error) {
      console.error(`❌ Failed to load TLS certificates for ${this.domain}:`, error.message);
      console.log(`📁 Expected certificate location: ${path.join(this.certDir, this.domain)}`);
      throw new Error(`TLS certificate loading failed: ${error.message}`);
    }
  }

  // Create HTTPS server with custom TLS
  createHTTPSServer(app) {
    const server = https.createServer(this.tlsOptions, app);
    
    // Add security event listeners
    server.on('secureConnection', (tlsSocket) => {
      console.log('🔐 Secure TLS connection established');
      console.log(`   Protocol: ${tlsSocket.getProtocol()}`);
      console.log(`   Cipher: ${tlsSocket.getCipher().name}`);
    });
    
    server.on('clientError', (err, socket) => {
      console.error('❌ TLS client error:', err.message);
      socket.end('HTTP/1.1 400 Bad Request\r\n\r\n');
    });
    
    return server;
  }

  // Security headers middleware
  securityHeaders(req, res, next) {
    // HTTPS security headers
    res.setHeader('Strict-Transport-Security', 'max-age=31536000; includeSubDomains; preload');
    res.setHeader('X-Content-Type-Options', 'nosniff');
    res.setHeader('X-Frame-Options', 'DENY');
    res.setHeader('X-XSS-Protection', '1; mode=block');
    res.setHeader('Referrer-Policy', 'strict-origin-when-cross-origin');
    res.setHeader('Content-Security-Policy', "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'");
    
    // BPCI custom headers
    res.setHeader('X-BPCI-TLS', 'Secure');
    res.setHeader('X-BPCI-Encryption', 'Military-Grade');
    res.setHeader('X-BPCI-Demo', 'true');
    
    next();
  }

  // HTTP to HTTPS redirect
  createHTTPRedirect(httpsPort) {
    const redirectServer = http.createServer((req, res) => {
      const httpsUrl = `https://${req.headers.host.split(':')[0]}:${httpsPort}${req.url}`;
      
      res.writeHead(301, {
        'Location': httpsUrl,
        'X-BPCI-Redirect': 'HTTPS-Required'
      });
      res.end(`Redirecting to HTTPS: ${httpsUrl}`);
    });
    
    return redirectServer;
  }
}

// Factory function to create BPCI TLS server
function createBPCITLSServer(domain = 'localhost') {
  return new BPCITLSServer(domain);
}

module.exports = {
  BPCITLSServer,
  createBPCITLSServer
};
