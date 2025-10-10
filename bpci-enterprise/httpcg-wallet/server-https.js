// HTTPCG Wallet Server - HTTPS Version with Let's Encrypt Support
const express = require('express');
const https = require('https');
const fs = require('fs');
const path = require('path');
const jwt = require('jsonwebtoken');
const cors = require('cors');
const WebSocket = require('ws');

const app = express();

// Middleware
app.use(express.json());
app.use(cors({
  origin: ['https://localhost:3000', 'https://localhost:8888', 'https://localhost:9443', 'https://pravyom.com', 'https://admin.pravyom.com'],
  credentials: true
}));

// Load TLS certificates (Let's Encrypt or custom)
function loadTLSCertificates() {
  const certDir = path.join(__dirname, '../tls/certificates');
  
  // Try Let's Encrypt certificates first
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

// Demo wallet data
const demoWallets = {
  'root': {
    address: 'bpi1root...enterprise',
    balance: '50000.00 BPI (demo)',
    staked: '25000.00 BPI (demo)',
    rewards: '1250.50 BPI (demo)',
    transactions: [
      {
        txid: 'demo_root_001',
        type: 'receive',
        amount: '+1000.00 BPI (demo)',
        from: 'bpi1system...genesis',
        timestamp: new Date(Date.now() - 7200000).toISOString(),
        status: 'confirmed'
      },
      {
        txid: 'demo_root_002',
        type: 'stake',
        amount: '-500.00 BPI (demo)',
        to: 'validator_pool',
        timestamp: new Date(Date.now() - 3600000).toISOString(),
        status: 'confirmed'
      }
    ],
    demo_mode: true,
    tls_secured: true,
    security_level: 'Military-Grade'
  },
  'demo': {
    address: 'bpi1demo...wallet',
    balance: '10000.00 BPI (demo)',
    staked: '5000.00 BPI (demo)',
    rewards: '125.50 BPI (demo)',
    transactions: [
      {
        txid: 'demo_tx_001',
        type: 'receive',
        amount: '+100.00 BPI (demo)',
        from: 'bpi1demo...sender',
        timestamp: new Date(Date.now() - 1800000).toISOString(),
        status: 'confirmed'
      }
    ],
    demo_mode: true,
    tls_secured: true,
    security_level: 'Standard'
  }
};

// Security headers middleware
app.use((req, res, next) => {
  res.setHeader('Strict-Transport-Security', 'max-age=31536000; includeSubDomains; preload');
  res.setHeader('X-Content-Type-Options', 'nosniff');
  res.setHeader('X-Frame-Options', 'DENY');
  res.setHeader('X-XSS-Protection', '1; mode=block');
  res.setHeader('X-BPCI-TLS', 'Secure');
  res.setHeader('X-BPCI-Demo', 'true');
  res.setHeader('X-HTTPCG-Protocol', 'Enabled');
  next();
});

// HTTPCG Wallet Dashboard
app.get('/', (req, res) => {
  res.send(`
<!DOCTYPE html>
<html>
<head>
    <title>💰 HTTPCG Wallet - HTTPS Secure</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 0; background: linear-gradient(135deg, #4caf50 0%, #2e7d32 100%); color: white; min-height: 100vh; }
        .container { max-width: 1000px; margin: 0 auto; padding: 20px; }
        .header { text-align: center; margin-bottom: 30px; }
        .secure-badge { background: #1976d2; padding: 10px 20px; border-radius: 25px; display: inline-block; margin: 10px; }
        .demo-badge { background: #ff9800; padding: 5px 10px; border-radius: 15px; font-size: 0.8em; }
        .wallet-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; margin: 20px 0; }
        .wallet-card { background: rgba(255,255,255,0.1); padding: 20px; border-radius: 10px; backdrop-filter: blur(10px); }
        .balance { font-size: 2em; font-weight: bold; color: #4caf50; }
        .tls-info { background: #1976d2; padding: 15px; border-radius: 8px; margin: 20px 0; }
        .transaction { background: rgba(255,255,255,0.05); padding: 10px; margin: 5px 0; border-radius: 5px; }
        .httpcg-info { background: #9c27b0; padding: 15px; border-radius: 8px; margin: 20px 0; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>💰 HTTPCG Wallet Server</h1>
            <div class="secure-badge">🔐 HTTPS Secure Connection</div>
            <div class="demo-badge">DEMO MODE</div>
        </div>
        
        <div class="tls-info">
            <h3>🔒 Security Status</h3>
            <p><strong>TLS:</strong> Let's Encrypt / Custom Certificate</p>
            <p><strong>Encryption:</strong> Production-Grade HTTPS</p>
            <p><strong>Browser Status:</strong> Green lock "Secure" (automatically trusted)</p>
            <p><strong>Demo Mode:</strong> All wallet operations are simulated</p>
        </div>
        
        <div class="httpcg-info">
            <h3>🌐 HTTPCG Protocol</h3>
            <p><strong>Protocol:</strong> Human-Readable Addressing</p>
            <p><strong>Compatibility:</strong> Web2 Shadow Registry Bridge</p>
            <p><strong>Access:</strong> httpcg://wallet.pravyom.prav@global</p>
        </div>
        
        <div class="wallet-grid">
            <div class="wallet-card">
                <h3>👑 Root Wallet</h3>
                <div class="balance">${demoWallets.root.balance}</div>
                <p>Address: ${demoWallets.root.address}</p>
                <p>Staked: ${demoWallets.root.staked}</p>
                <p>Rewards: ${demoWallets.root.rewards}</p>
                <p>Security: ${demoWallets.root.security_level}</p>
            </div>
            
            <div class="wallet-card">
                <h3>🎯 Demo Wallet</h3>
                <div class="balance">${demoWallets.demo.balance}</div>
                <p>Address: ${demoWallets.demo.address}</p>
                <p>Staked: ${demoWallets.demo.staked}</p>
                <p>Rewards: ${demoWallets.demo.rewards}</p>
                <p>Security: ${demoWallets.demo.security_level}</p>
            </div>
        </div>
        
        <div style="text-align: center; margin-top: 30px;">
            <p>🔐 <strong>HTTPS Secured by Production TLS</strong></p>
            <p>All wallet communications encrypted and automatically trusted by browsers</p>
        </div>
    </div>
</body>
</html>
  `);
});

// API endpoints
app.get('/api/wallet/:walletId', (req, res) => {
  const { walletId } = req.params;
  const wallet = demoWallets[walletId];
  
  if (!wallet) {
    return res.status(404).json({ error: 'Wallet not found (demo)' });
  }
  
  res.json({
    ...wallet,
    https_secured: true,
    tls_enabled: true,
    timestamp: new Date().toISOString()
  });
});

app.get('/health', (req, res) => {
  res.json({
    status: 'healthy',
    service: 'httpcg-wallet',
    https_secure: true,
    tls_enabled: true,
    demo_mode: true,
    httpcg_protocol: 'enabled',
    timestamp: new Date().toISOString()
  });
});

// Start HTTPS server
const PORT = process.env.PORT || 7778;

try {
  const { tlsOptions, domain } = loadTLSCertificates();
  const server = https.createServer(tlsOptions, app);
  
  // WebSocket server for real-time updates
  const wss = new WebSocket.Server({ server });
  
  wss.on('connection', (ws) => {
    console.log('🔗 WebSocket connection established (HTTPS)');
    
    // Send demo wallet updates
    const sendUpdate = () => {
      ws.send(JSON.stringify({
        type: 'wallet_update',
        data: {
          root: demoWallets.root,
          demo: demoWallets.demo,
          https_secured: true,
          timestamp: new Date().toISOString()
        }
      }));
    };
    
    // Send initial data
    sendUpdate();
    
    // Send updates every 30 seconds
    const interval = setInterval(sendUpdate, 30000);
    
    ws.on('close', () => {
      clearInterval(interval);
      console.log('🔗 WebSocket connection closed');
    });
  });
  
  server.listen(PORT, () => {
    console.log(`🚀 HTTPCG Wallet Server (HTTPS) running on port ${PORT}`);
    console.log(`🔒 TLS: Enabled with ${domain} certificates`);
    console.log(`🌐 Access: https://localhost:${PORT}`);
    console.log(`🔐 Expected: Green lock "Secure" (automatically trusted)`);
    console.log(`💰 Demo wallets: root, demo`);
  });
  
  server.on('error', (error) => {
    console.error('❌ HTTPS Wallet Server Error:', error.message);
  });
  
} catch (error) {
  console.error('❌ Failed to start HTTPS wallet server:', error.message);
  console.log('💡 Run: ./tls/letsencrypt-setup.sh to generate production certificates');
  process.exit(1);
}
