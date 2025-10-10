// BPCI Enterprise HTTPS Server - Simple Working Version
const express = require('express');
const https = require('https');
const fs = require('fs');
const path = require('path');
const jwt = require('jsonwebtoken');
const cors = require('cors');

const app = express();

// Middleware
app.use(express.json());
app.use(cors({
  origin: ['https://localhost:3000', 'https://localhost:8888', 'https://localhost:7778'],
  credentials: true
}));

// Load TLS certificates
const certDir = path.join(__dirname, '../tls/certificates/localhost');
const tlsOptions = {
  cert: fs.readFileSync(path.join(certDir, 'certificate-chain.pem')),
  key: fs.readFileSync(path.join(certDir, 'private-key.pem')),
  ca: fs.readFileSync(path.join(__dirname, '../tls/certificates/ca-certificate.pem'))
};

// Security headers middleware
app.use((req, res, next) => {
  res.setHeader('Strict-Transport-Security', 'max-age=31536000; includeSubDomains; preload');
  res.setHeader('X-Content-Type-Options', 'nosniff');
  res.setHeader('X-Frame-Options', 'DENY');
  res.setHeader('X-XSS-Protection', '1; mode=block');
  res.setHeader('X-BPCI-TLS', 'Secure');
  res.setHeader('X-BPCI-Demo', 'true');
  next();
});

// Demo system status
const systemStatus = {
  bpci_server: 'online',
  wallet_server: 'online',
  admin_dashboard: 'online',
  https_enabled: true,
  tls_secured: true,
  demo_mode: true,
  security_level: 'Military-Grade',
  last_updated: new Date().toISOString()
};

// Main BPCI dashboard
app.get('/', (req, res) => {
  res.send(`
<!DOCTYPE html>
<html>
<head>
    <title>🔐 BPCI Enterprise Server - HTTPS Secure</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 0; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; min-height: 100vh; }
        .container { max-width: 1000px; margin: 0 auto; padding: 20px; }
        .header { text-align: center; margin-bottom: 30px; }
        .secure-badge { background: #4caf50; padding: 10px 20px; border-radius: 25px; display: inline-block; margin: 10px; }
        .demo-badge { background: #ff9800; padding: 5px 10px; border-radius: 15px; font-size: 0.8em; }
        .status-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px; margin: 20px 0; }
        .status-card { background: rgba(255,255,255,0.1); padding: 20px; border-radius: 10px; backdrop-filter: blur(10px); }
        .online { color: #4caf50; font-weight: bold; }
        .tls-info { background: #2196f3; padding: 15px; border-radius: 8px; margin: 20px 0; }
        .access-links { background: rgba(255,255,255,0.1); padding: 20px; border-radius: 10px; margin: 20px 0; }
        .link { display: block; color: #4caf50; text-decoration: none; margin: 5px 0; font-weight: bold; }
        .link:hover { color: #81c784; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 BPCI Enterprise Server</h1>
            <div class="secure-badge">✅ HTTPS Secure Connection</div>
            <div class="demo-badge">DEMO MODE</div>
        </div>
        
        <div class="tls-info">
            <h3>🔒 Security Status</h3>
            <p><strong>TLS:</strong> Custom BPCI Enterprise Certificate</p>
            <p><strong>Encryption:</strong> 4096-bit RSA + Perfect Forward Secrecy</p>
            <p><strong>Browser Status:</strong> Should show green lock "Secure" after CA import</p>
            <p><strong>Demo Mode:</strong> All operations are simulated</p>
        </div>
        
        <div class="status-grid">
            <div class="status-card">
                <h3>🖥️ BPCI Server</h3>
                <p>Status: <span class="online">ONLINE</span></p>
                <p>Port: 9443 (HTTPS)</p>
                <p>TLS: Enabled</p>
            </div>
            
            <div class="status-card">
                <h3>💰 Wallet Server</h3>
                <p>Status: <span class="online">ONLINE</span></p>
                <p>Port: 7778 (HTTPS)</p>
                <p>Demo Wallets: Active</p>
            </div>
            
            <div class="status-card">
                <h3>📊 Admin Dashboard</h3>
                <p>Status: <span class="online">ONLINE</span></p>
                <p>Port: 8888 (HTTPS)</p>
                <p>Root Access: Enabled</p>
            </div>
        </div>
        
        <div class="access-links">
            <h3>🌐 Access Points</h3>
            <a href="https://localhost:8888/httpcg/dashboard" class="link">📊 Admin Dashboard</a>
            <a href="https://localhost:7778" class="link">💰 Wallet Server</a>
            <a href="/api/status" class="link">📈 System Status API</a>
            <a href="/health" class="link">🔍 Health Check</a>
        </div>
        
        <div style="text-align: center; margin-top: 30px;">
            <p>🔐 <strong>HTTPS Secured by BPCI Enterprise TLS</strong></p>
            <p>All communications encrypted with military-grade security</p>
            <p><strong>To see green lock:</strong> Import CA certificate from /home/umesh/metanode/bpci-enterprise/tls/certificates/ca-certificate.pem</p>
        </div>
    </div>
</body>
</html>
  `);
});

// API endpoints
app.get('/api/status', (req, res) => {
  res.json({
    ...systemStatus,
    timestamp: new Date().toISOString(),
    https_secure: true,
    certificate_info: {
      issuer: 'BPCI Enterprise Root CA',
      subject: 'localhost',
      valid_from: '2024-09-08',
      valid_to: '2034-09-08'
    }
  });
});

app.get('/health', (req, res) => {
  res.json({
    status: 'healthy',
    service: 'bpci-server',
    https_secure: true,
    tls_enabled: true,
    demo_mode: true,
    timestamp: new Date().toISOString()
  });
});

// Start HTTPS server
const PORT = process.env.HTTPS_PORT || 9443;
const server = https.createServer(tlsOptions, app);

server.listen(PORT, () => {
  console.log(`🚀 BPCI Enterprise Server (HTTPS) running on port ${PORT}`);
  console.log(`🔒 TLS: Enabled with custom certificates`);
  console.log(`🌐 Access: https://localhost:${PORT}`);
  console.log(`🔐 Expected: Green lock "Secure" after CA import`);
  console.log(`📁 CA Certificate: /home/umesh/metanode/bpci-enterprise/tls/certificates/ca-certificate.pem`);
});

server.on('error', (error) => {
  console.error('❌ HTTPS Server Error:', error.message);
});
