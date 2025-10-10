// BPCI Admin Dashboard - HTTPCG Protocol Implementation
const express = require('express');
const https = require('https');
const fs = require('fs');
const path = require('path');
const jwt = require('jsonwebtoken');
const cors = require('cors');

const app = express();

// HTTPCG Protocol Configuration
const HTTPCG_CONFIG = {
  protocol: 'httpcg',
  version: '1.0',
  addressing: 'human-readable',
  domain: 'admin.pravyom.prav@global',
  shadow_registry: true,
  web2_bridge: true
};

// Middleware
app.use(express.json());
app.use(cors({
  origin: ['httpcg://admin.pravyom.prav@global', 'httpcg://wallet.pravyom.prav@global', 'https://pravyom.com'],
  credentials: true
}));

// Load TLS certificates
function loadTLSCertificates() {
  const certDir = path.join(__dirname, '../tls/certificates');
  const domains = ['pravyom.com', 'localhost'];
  
  for (const domain of domains) {
    try {
      const domainCertDir = path.join(certDir, domain);
      const certPath = path.join(domainCertDir, 'certificate-chain.pem');
      const keyPath = path.join(domainCertDir, 'private-key.pem');
      
      if (fs.existsSync(certPath) && fs.existsSync(keyPath)) {
        console.log(`🔍 Loading TLS certificates for ${domain}`);
        
        const tlsOptions = {
          cert: fs.readFileSync(certPath),
          key: fs.readFileSync(keyPath),
          secureProtocol: 'TLSv1_2_method'
        };
        
        console.log(`✅ TLS certificates loaded for ${domain}`);
        return { tlsOptions, domain };
      }
    } catch (error) {
      console.log(`⚠️  Failed to load certificates for ${domain}: ${error.message}`);
    }
  }
  
  throw new Error('No valid TLS certificates found');
}

// HTTPCG Protocol Headers Middleware
app.use((req, res, next) => {
  // HTTPCG Protocol Headers
  res.setHeader('X-HTTPCG-Protocol', HTTPCG_CONFIG.version);
  res.setHeader('X-HTTPCG-Addressing', HTTPCG_CONFIG.addressing);
  res.setHeader('X-HTTPCG-Domain', HTTPCG_CONFIG.domain);
  res.setHeader('X-HTTPCG-Shadow-Registry', HTTPCG_CONFIG.shadow_registry);
  res.setHeader('X-HTTPCG-Web2-Bridge', HTTPCG_CONFIG.web2_bridge);
  
  // Security headers
  res.setHeader('Strict-Transport-Security', 'max-age=31536000; includeSubDomains; preload');
  res.setHeader('X-Content-Type-Options', 'nosniff');
  res.setHeader('X-Frame-Options', 'DENY');
  res.setHeader('X-XSS-Protection', '1; mode=block');
  
  // BPCI Enterprise headers
  res.setHeader('X-BPCI-TLS', 'Production');
  res.setHeader('X-BPCI-Demo', 'true');
  res.setHeader('X-BPCI-Service', 'httpcg-admin');
  
  next();
});

// Demo system data with HTTPCG addressing
const systemStatus = {
  bpci_server: {
    status: 'online',
    address: 'httpcg://server.bpci.pravyom.prav@global',
    port: 9443,
    protocol: 'httpcg+tls'
  },
  wallet_server: {
    status: 'online', 
    address: 'httpcg://wallet.pravyom.prav@global',
    port: 7778,
    protocol: 'httpcg+tls'
  },
  admin_dashboard: {
    status: 'online',
    address: 'httpcg://admin.pravyom.prav@global', 
    port: 8888,
    protocol: 'httpcg+tls'
  },
  httpcg_protocol: true,
  https_enabled: true,
  tls_secured: true,
  demo_mode: true,
  security_level: 'Military-Grade',
  last_updated: new Date().toISOString()
};

// Demo wallet data for admin view
const adminWalletView = {
  total_wallets: 2,
  total_balance: '60000.00 BPI (demo)',
  total_staked: '30000.00 BPI (demo)',
  total_rewards: '1376.00 BPI (demo)',
  wallets: [
    {
      id: 'root',
      address: 'httpcg://root.wallet.pravyom.prav@global',
      balance: '50000.00 BPI (demo)',
      status: 'active',
      security_level: 'Military-Grade'
    },
    {
      id: 'demo',
      address: 'httpcg://demo.wallet.pravyom.prav@global', 
      balance: '10000.00 BPI (demo)',
      status: 'active',
      security_level: 'Standard'
    }
  ],
  demo_mode: true,
  httpcg_protocol: true
};

// HTTPCG Admin Dashboard
app.get('/httpcg/dashboard', (req, res) => {
  res.send(`
<!DOCTYPE html>
<html>
<head>
    <title>📊 BPCI Admin Dashboard - HTTPCG Protocol</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 0; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; min-height: 100vh; }
        .container { max-width: 1200px; margin: 0 auto; padding: 20px; }
        .header { text-align: center; margin-bottom: 30px; }
        .protocol-badge { background: #9c27b0; padding: 10px 20px; border-radius: 25px; display: inline-block; margin: 10px; }
        .secure-badge { background: #4caf50; padding: 10px 20px; border-radius: 25px; display: inline-block; margin: 10px; }
        .demo-badge { background: #ff9800; padding: 5px 10px; border-radius: 15px; font-size: 0.8em; }
        .dashboard { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; }
        .card { background: rgba(255,255,255,0.1); padding: 20px; border-radius: 10px; backdrop-filter: blur(10px); }
        .balance { font-size: 2em; font-weight: bold; color: #4caf50; }
        .httpcg-info { background: #9c27b0; padding: 15px; border-radius: 8px; margin: 20px 0; }
        .tls-info { background: #2196f3; padding: 15px; border-radius: 8px; margin: 20px 0; }
        .online { color: #4caf50; font-weight: bold; }
        .address { font-family: monospace; background: rgba(0,0,0,0.3); padding: 5px; border-radius: 3px; font-size: 0.9em; }
        .wallet-item { background: rgba(255,255,255,0.05); padding: 10px; margin: 5px 0; border-radius: 5px; }
        .system-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 15px; margin: 20px 0; }
        .system-card { background: rgba(255,255,255,0.1); padding: 15px; border-radius: 8px; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>📊 BPCI Enterprise Admin Dashboard</h1>
            <div class="protocol-badge">🌐 HTTPCG Protocol v${HTTPCG_CONFIG.version}</div>
            <div class="secure-badge">✅ TLS Secured</div>
            <div class="demo-badge">DEMO MODE</div>
        </div>
        
        <div class="httpcg-info">
            <h3>🌐 HTTPCG Protocol Status</h3>
            <p><strong>Admin Address:</strong> ${HTTPCG_CONFIG.protocol}://${HTTPCG_CONFIG.domain}</p>
            <p><strong>Addressing Type:</strong> ${HTTPCG_CONFIG.addressing}</p>
            <p><strong>Shadow Registry:</strong> ${HTTPCG_CONFIG.shadow_registry ? 'Enabled' : 'Disabled'}</p>
            <p><strong>Web2 Bridge:</strong> ${HTTPCG_CONFIG.web2_bridge ? 'Active' : 'Inactive'}</p>
            <p><strong>Demo Mode:</strong> All admin operations are simulated</p>
        </div>
        
        <div class="tls-info">
            <h3>🔒 Security Status</h3>
            <p><strong>TLS:</strong> Production-Grade HTTPS over HTTPCG</p>
            <p><strong>Encryption:</strong> Let's Encrypt / Custom Certificate</p>
            <p><strong>Browser Status:</strong> Green lock "Secure" (automatically trusted)</p>
            <p><strong>Admin Access:</strong> Root login enabled (demo)</p>
        </div>
        
        <div class="system-grid">
            <div class="system-card">
                <h3>🖥️ BPCI Server</h3>
                <p>Status: <span class="online">${systemStatus.bpci_server.status.toUpperCase()}</span></p>
                <p>Port: ${systemStatus.bpci_server.port}</p>
                <p>Protocol: ${systemStatus.bpci_server.protocol}</p>
                <div class="address">${systemStatus.bpci_server.address}</div>
            </div>
            
            <div class="system-card">
                <h3>💰 Wallet Server</h3>
                <p>Status: <span class="online">${systemStatus.wallet_server.status.toUpperCase()}</span></p>
                <p>Port: ${systemStatus.wallet_server.port}</p>
                <p>Protocol: ${systemStatus.wallet_server.protocol}</p>
                <div class="address">${systemStatus.wallet_server.address}</div>
            </div>
            
            <div class="system-card">
                <h3>📊 Admin Dashboard</h3>
                <p>Status: <span class="online">${systemStatus.admin_dashboard.status.toUpperCase()}</span></p>
                <p>Port: ${systemStatus.admin_dashboard.port}</p>
                <p>Protocol: ${systemStatus.admin_dashboard.protocol}</p>
                <div class="address">${systemStatus.admin_dashboard.address}</div>
            </div>
        </div>
        
        <div class="dashboard">
            <div class="card">
                <h3>💰 Wallet Overview</h3>
                <div class="balance">${adminWalletView.total_balance}</div>
                <p>Total Wallets: ${adminWalletView.total_wallets}</p>
                <p>Total Staked: ${adminWalletView.total_staked}</p>
                <p>Total Rewards: ${adminWalletView.total_rewards}</p>
                <p>HTTPCG Compliant: ✅</p>
            </div>
            
            <div class="card">
                <h3>👥 Active Wallets</h3>
                ${adminWalletView.wallets.map(wallet => `
                    <div class="wallet-item">
                        <strong>${wallet.id.toUpperCase()}</strong><br>
                        Balance: ${wallet.balance}<br>
                        Security: ${wallet.security_level}<br>
                        <div class="address">${wallet.address}</div>
                    </div>
                `).join('')}
            </div>
            
            <div class="card">
                <h3>📈 System Metrics</h3>
                <p>✅ All Services: Online</p>
                <p>✅ HTTPCG Protocol: Active</p>
                <p>✅ TLS Security: Enabled</p>
                <p>✅ Demo Mode: Safe Testing</p>
                <p>✅ Shadow Registry: Bridged</p>
                <p>✅ Web2 Compatibility: Active</p>
            </div>
        </div>
        
        <div style="text-align: center; margin-top: 30px;">
            <p>🌐 <strong>HTTPCG Protocol + TLS Security</strong></p>
            <p>Human-readable addressing with production-grade encryption</p>
            <p><strong>Admin Access:</strong> httpcg://admin.pravyom.prav@global</p>
            <p><strong>Root Login:</strong> root / admin (demo)</p>
        </div>
    </div>
</body>
</html>
  `);
});

// HTTPCG Protocol API endpoints
app.get('/httpcg/admin/status', (req, res) => {
  res.json({
    ...systemStatus,
    httpcg_protocol: HTTPCG_CONFIG,
    admin_wallet_view: adminWalletView,
    timestamp: new Date().toISOString()
  });
});

app.get('/httpcg/admin/wallets', (req, res) => {
  res.json({
    ...adminWalletView,
    httpcg_protocol: HTTPCG_CONFIG,
    timestamp: new Date().toISOString()
  });
});

app.get('/httpcg/protocol/info', (req, res) => {
  res.json({
    protocol: HTTPCG_CONFIG,
    service: 'httpcg-admin',
    demo_mode: true,
    tls_secured: true,
    addressing_examples: [
      'httpcg://admin.pravyom.prav@global',
      'httpcg://wallet.pravyom.prav@global',
      'httpcg://server.bpci.pravyom.prav@global'
    ],
    timestamp: new Date().toISOString()
  });
});

app.get('/health', (req, res) => {
  res.json({
    status: 'healthy',
    service: 'httpcg-admin',
    protocol: 'httpcg',
    https_secure: true,
    tls_enabled: true,
    demo_mode: true,
    httpcg_compliant: true,
    timestamp: new Date().toISOString()
  });
});

// Start HTTPS server with HTTPCG protocol
const PORT = process.env.PORT || 8888;

try {
  const { tlsOptions, domain } = loadTLSCertificates();
  const server = https.createServer(tlsOptions, app);
  
  server.listen(PORT, () => {
    console.log(`🚀 HTTPCG Admin Dashboard running on port ${PORT}`);
    console.log(`🌐 Protocol: ${HTTPCG_CONFIG.protocol}://${HTTPCG_CONFIG.domain}`);
    console.log(`🔒 TLS: Enabled with ${domain} certificates`);
    console.log(`🌐 HTTPS Access: https://localhost:${PORT}/httpcg/dashboard`);
    console.log(`🌐 HTTPCG Access: httpcg://admin.pravyom.prav@global`);
    console.log(`🔐 Expected: Green lock "Secure" (automatically trusted)`);
    console.log(`👑 Root Login: root / admin (demo)`);
  });
  
  server.on('error', (error) => {
    console.error('❌ HTTPCG Admin Dashboard Error:', error.message);
  });
  
} catch (error) {
  console.error('❌ Failed to start HTTPCG admin dashboard:', error.message);
  console.log('💡 Run: ./tls/letsencrypt-setup.sh to generate production certificates');
  process.exit(1);
}
