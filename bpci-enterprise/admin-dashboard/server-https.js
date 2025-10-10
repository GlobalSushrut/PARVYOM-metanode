// BPCI Admin Dashboard - HTTPS Version with Custom TLS
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
  origin: ['https://localhost:3000', 'https://pravyom.com', 'https://admin.pravyom.com'],
  credentials: true
}));

// Load TLS certificates
const certDir = path.join(__dirname, '../tls/certificates/localhost');
const tlsOptions = {
  cert: fs.readFileSync(path.join(certDir, 'certificate-chain.pem')),
  key: fs.readFileSync(path.join(certDir, 'private-key.pem')),
  secureProtocol: 'TLSv1_2_method'
};

// Demo wallet data with TLS security info
const demoWallet = {
  balance: '10000.00 BPI (demo)',
  staked: '5000.00 BPI (demo)',
  rewards: '125.50 BPI (demo)',
  address: 'bpi1demo...rootenterprise',
  transactions: [
    {
      txid: 'demo_tx_001',
      type: 'receive',
      amount: '+100.00 BPI (demo)',
      from: 'bpi1demo...sender',
      timestamp: new Date(Date.now() - 3600000).toISOString(),
      status: 'confirmed'
    },
    {
      txid: 'demo_tx_002', 
      type: 'stake',
      amount: '-50.00 BPI (demo)',
      to: 'staking_pool',
      timestamp: new Date(Date.now() - 1800000).toISOString(),
      status: 'confirmed'
    }
  ],
  demo_mode: true,
  tls_secured: true,
  security_level: 'Military-Grade'
};

// Security headers middleware
app.use((req, res, next) => {
  res.setHeader('Strict-Transport-Security', 'max-age=31536000; includeSubDomains; preload');
  res.setHeader('X-Content-Type-Options', 'nosniff');
  res.setHeader('X-Frame-Options', 'DENY');
  res.setHeader('X-XSS-Protection', '1; mode=block');
  res.setHeader('X-BPCI-TLS', 'Secure');
  next();
});

// HTTPCG Dashboard endpoint
app.get('/httpcg/dashboard', (req, res) => {
  res.send(`
<!DOCTYPE html>
<html>
<head>
    <title>BPCI Enterprise Dashboard - Secure</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 0; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; }
        .container { max-width: 1200px; margin: 0 auto; padding: 20px; }
        .header { text-align: center; margin-bottom: 30px; }
        .secure-badge { background: #4caf50; padding: 10px 20px; border-radius: 25px; display: inline-block; margin: 10px; }
        .dashboard { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; }
        .card { background: rgba(255,255,255,0.1); padding: 20px; border-radius: 10px; backdrop-filter: blur(10px); }
        .balance { font-size: 2em; font-weight: bold; color: #4caf50; }
        .demo-badge { background: #ff9800; padding: 5px 10px; border-radius: 15px; font-size: 0.8em; }
        .tls-info { background: #2196f3; padding: 15px; border-radius: 8px; margin: 20px 0; }
        .transaction { background: rgba(255,255,255,0.05); padding: 10px; margin: 5px 0; border-radius: 5px; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 BPCI Enterprise Dashboard</h1>
            <div class="secure-badge">✅ HTTPS Secure Connection</div>
            <div class="demo-badge">DEMO MODE</div>
        </div>
        
        <div class="tls-info">
            <h3>🔒 Security Status</h3>
            <p><strong>TLS:</strong> Custom BPCI Enterprise Certificate</p>
            <p><strong>Encryption:</strong> 4096-bit RSA + Perfect Forward Secrecy</p>
            <p><strong>Browser Status:</strong> Should show green lock "Secure"</p>
            <p><strong>Demo Mode:</strong> All operations are simulated</p>
        </div>
        
        <div class="dashboard">
            <div class="card">
                <h3>💰 Wallet Balance</h3>
                <div class="balance">${demoWallet.balance}</div>
                <p>Address: ${demoWallet.address}</p>
                <p>Staked: ${demoWallet.staked}</p>
                <p>Rewards: ${demoWallet.rewards}</p>
            </div>
            
            <div class="card">
                <h3>📊 System Status</h3>
                <p>✅ BPCI Server: Online</p>
                <p>✅ Wallet Server: Online</p>
                <p>✅ HTTPS: Secure</p>
                <p>✅ Demo Mode: Active</p>
            </div>
            
            <div class="card">
                <h3>📈 Recent Transactions</h3>
                ${demoWallet.transactions.map(tx => `
                    <div class="transaction">
                        <strong>${tx.type.toUpperCase()}</strong><br>
                        ${tx.amount}<br>
                        <small>${new Date(tx.timestamp).toLocaleString()}</small>
                    </div>
                `).join('')}
            </div>
        </div>
        
        <div style="text-align: center; margin-top: 30px;">
            <p>🔐 <strong>HTTPS Secured by BPCI Enterprise TLS</strong></p>
            <p>All communications encrypted with military-grade security</p>
        </div>
    </div>
</body>
</html>
  `);
});

// API endpoints
app.get('/api/dashboard/status', (req, res) => {
  res.json({
    status: 'operational',
    demo_mode: true,
    tls_secured: true,
    https_enabled: true,
    security_level: 'Military-Grade',
    wallet: demoWallet
  });
});

app.get('/health', (req, res) => {
  res.json({
    status: 'healthy',
    service: 'admin-dashboard',
    https_secure: true,
    tls_enabled: true,
    demo_mode: true,
    timestamp: new Date().toISOString()
  });
});

// Start HTTPS server
const PORT = process.env.PORT || 8888;
const server = https.createServer(tlsOptions, app);

server.listen(PORT, () => {
  console.log(`🚀 BPCI Admin Dashboard (HTTPS) running on port ${PORT}`);
  console.log(`🔒 TLS: Enabled with custom certificates`);
  console.log(`🌐 Access: https://localhost:${PORT}/httpcg/dashboard`);
  console.log(`🔐 Expected: Green lock "Secure" after CA import`);
});
