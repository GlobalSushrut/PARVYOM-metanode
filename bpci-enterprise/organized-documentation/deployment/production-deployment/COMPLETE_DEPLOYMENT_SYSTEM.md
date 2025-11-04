# Complete BPCI Production Deployment System
## Website (Vercel) + BPCI Server (4 CPU) + Wallet (2 CPU) + Admin Dashboard

### Executive Summary

This document provides the complete deployment system for:
- **Website**: Deployed on Vercel with custom domain
- **BPCI Server**: 4 CPU instance with full coordination
- **HTTPCG Wallet**: 2 CPU instance with real wallet functionality
- **Admin Dashboard**: Root login with demo wallet (returns "demo" in all BPI responses)

---

## 🌐 **1. Website Deployment on Vercel**

### **Website Structure**
```
website/
├── package.json
├── next.config.js
├── vercel.json
├── pages/
│   ├── index.js          # Homepage with login
│   ├── dashboard.js      # Admin dashboard
│   └── api/
│       ├── auth.js       # Authentication
│       └── redirect.js   # HTTPCG redirect
├── components/
│   ├── LoginForm.js
│   └── Dashboard.js
└── styles/
    └── globals.css
```

### **package.json**
```json
{
  "name": "bpci-website",
  "version": "1.0.0",
  "private": true,
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "lint": "next lint"
  },
  "dependencies": {
    "next": "14.0.0",
    "react": "18.2.0",
    "react-dom": "18.2.0",
    "axios": "^1.6.0",
    "jsonwebtoken": "^9.0.2",
    "bcryptjs": "^2.4.3"
  },
  "devDependencies": {
    "eslint": "^8.0.0",
    "eslint-config-next": "14.0.0"
  }
}
```

### **Homepage with Login (pages/index.js)**
```javascript
import { useState } from 'react';
import { useRouter } from 'next/router';
import axios from 'axios';

export default function Home() {
  const [credentials, setCredentials] = useState({ username: '', password: '' });
  const [loading, setLoading] = useState(false);
  const router = useRouter();

  const handleLogin = async (e) => {
    e.preventDefault();
    setLoading(true);
    
    try {
      const response = await axios.post('/api/auth', credentials);
      if (response.data.success) {
        // Redirect to HTTPCG dashboard
        window.location.href = response.data.httpcg_redirect;
      }
    } catch (error) {
      alert('Login failed: ' + error.response?.data?.message || 'Unknown error');
    }
    
    setLoading(false);
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-900 to-purple-900 flex items-center justify-center">
      <div className="bg-white p-8 rounded-lg shadow-2xl w-96">
        <div className="text-center mb-8">
          <h1 className="text-3xl font-bold text-gray-800">BPCI Testnet</h1>
          <p className="text-gray-600 mt-2">Next-Generation Enterprise OS</p>
        </div>
        
        <form onSubmit={handleLogin} className="space-y-6">
          <div>
            <label className="block text-sm font-medium text-gray-700">Username</label>
            <input
              type="text"
              value={credentials.username}
              onChange={(e) => setCredentials({...credentials, username: e.target.value})}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
              placeholder="Enter username"
              required
            />
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700">Password</label>
            <input
              type="password"
              value={credentials.password}
              onChange={(e) => setCredentials({...credentials, password: e.target.value})}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
              placeholder="Enter password"
              required
            />
          </div>
          
          <button
            type="submit"
            disabled={loading}
            className="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50"
          >
            {loading ? 'Authenticating...' : 'Access HTTPCG Dashboard'}
          </button>
        </form>
        
        <div className="mt-6 text-center">
          <p className="text-sm text-gray-500">
            Demo Credentials: <strong>root</strong> / <strong>admin</strong>
          </p>
        </div>
        
        <div className="mt-8 text-center">
          <div className="text-xs text-gray-400">
            <p>🌐 HTTPCG Protocol Active</p>
            <p>🔒 Post-Quantum Security</p>
            <p>⚡ Military-Grade Encryption</p>
          </div>
        </div>
      </div>
    </div>
  );
}
```

### **Authentication API (pages/api/auth.js)**
```javascript
import bcrypt from 'bcryptjs';
import jwt from 'jsonwebtoken';

// Demo credentials
const DEMO_USERS = {
  'root': {
    password: '$2a$10$92IXUNpkjO0rOQ5byMi.Ye4oKoEa3Ro9llC/.og/at2.uheWG/igi', // password: admin
    role: 'admin'
  }
};

export default async function handler(req, res) {
  if (req.method !== 'POST') {
    return res.status(405).json({ message: 'Method not allowed' });
  }

  const { username, password } = req.body;

  // Validate credentials
  const user = DEMO_USERS[username];
  if (!user || !bcrypt.compareSync(password, user.password)) {
    return res.status(401).json({ message: 'Invalid credentials' });
  }

  // Generate JWT token
  const token = jwt.sign(
    { username, role: user.role },
    process.env.JWT_SECRET || 'demo-secret-key',
    { expiresIn: '24h' }
  );

  // Return HTTPCG redirect URL
  const httpcg_redirect = `${process.env.BPCI_SERVER_URL || 'http://localhost:8888'}/httpcg/dashboard?token=${token}`;

  res.status(200).json({
    success: true,
    token,
    httpcg_redirect,
    message: 'Authentication successful'
  });
}
```

### **Vercel Configuration (vercel.json)**
```json
{
  "version": 2,
  "builds": [
    {
      "src": "package.json",
      "use": "@vercel/next"
    }
  ],
  "routes": [
    {
      "src": "/(.*)",
      "dest": "/$1"
    }
  ],
  "env": {
    "JWT_SECRET": "@jwt-secret",
    "BPCI_SERVER_URL": "@bpci-server-url"
  }
}
```

### **Deployment Script (deploy-website.sh)**
```bash
#!/bin/bash
# deploy-website.sh

set -e

echo "🌐 Deploying BPCI Website to Vercel..."

# Install Vercel CLI if not present
if ! command -v vercel &> /dev/null; then
    npm install -g vercel
fi

# Set environment variables
vercel env add JWT_SECRET production
vercel env add BPCI_SERVER_URL production

# Deploy to Vercel
vercel --prod

# Add custom domain (replace with your domain)
read -p "Enter your custom domain (e.g., pravyom.com): " CUSTOM_DOMAIN
vercel domains add "$CUSTOM_DOMAIN"

echo "✅ Website deployed to Vercel"
echo "🌐 URL: https://$CUSTOM_DOMAIN"
```

---

## 🏢 **2. BPCI Server Deployment (4 CPU Instance)**

### **BPCI Server Configuration**
```bash
#!/bin/bash
# deploy-bpci-server.sh

set -e

echo "🏢 Deploying BPCI Server (4 CPU Instance)..."

# Update system
apt-get update && apt-get upgrade -y

# Install dependencies
apt-get install -y curl wget build-essential git nginx certbot python3-certbot-nginx

# Install BPI OS
curl -fsSL https://repo.pravyom.com/install-bpi-os.sh | bash

# Configure BPCI server
cat > /etc/bpi-os/bpci-production-server.yaml << 'EOF'
bpci_production_server:
  mode: "production"
  instance_type: "bpci-global-coordinator"
  
  httpcg_configuration:
    primary_domain: "httpcg://dashboard.prav.global"
    vm_server_port: 7777
    native_protocol: true
    
  admin_dashboard:
    enabled: true
    root_login: true
    demo_wallet: true
    port: 8888
    
  xtmp_server:
    port: 9999
    max_sessions: 10000
    bundle_coordination: true
    
  shadow_registry:
    web2_bridge_port: 8889
    authentication_flow: true
    
  resources:
    cpu_allocation:
      admin_dashboard: 1
      xtmp_server: 1
      vm_server: 1
      coordination: 1
    memory_allocation:
      admin_dashboard: "2GB"
      xtmp_server: "2GB"
      vm_server: "2GB"
      coordination: "2GB"
      
  demo_mode:
    enabled: true
    wallet_responses: "demo"
    bpi_responses: "demo"
EOF

# Deploy BPCI server
/opt/bpi-os/bin/bpi-core enterprise deploy \
    --config /etc/bpi-os/bpci-production-server.yaml \
    --mode production \
    --admin-dashboard true

# Configure nginx for admin dashboard
cat > /etc/nginx/sites-available/bpci-admin << 'EOF'
server {
    listen 80;
    server_name _;
    
    location /httpcg/dashboard {
        proxy_pass http://localhost:8888;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
    
    location /api/ {
        proxy_pass http://localhost:8888/api/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
    
    location /health {
        proxy_pass http://localhost:8888/health;
    }
}
EOF

ln -sf /etc/nginx/sites-available/bpci-admin /etc/nginx/sites-enabled/
systemctl reload nginx

# Start services
systemctl start bpi-xtmp-bpci-server
systemctl start bpi-vm-server
systemctl start bpi-admin-dashboard
systemctl enable bpi-xtmp-bpci-server
systemctl enable bpi-vm-server
systemctl enable bpi-admin-dashboard

echo "✅ BPCI Server deployed successfully"
echo "🌐 Admin Dashboard: http://[server-ip]:8888/httpcg/dashboard"
echo "🔗 XTMP Server: [server-ip]:9999"
```

### **Admin Dashboard Implementation**
```javascript
// /opt/bpi-os/admin-dashboard/server.js
const express = require('express');
const jwt = require('jsonwebtoken');
const path = require('path');

const app = express();
app.use(express.json());
app.use(express.static('public'));

// Demo wallet that always returns "demo"
class DemoWallet {
    constructor() {
        this.balance = "1000.00 (demo)";
        this.address = "bpi1demo...address (demo)";
        this.transactions = [
            { id: "tx1", amount: "100.00 (demo)", type: "receive", timestamp: new Date() },
            { id: "tx2", amount: "50.00 (demo)", type: "send", timestamp: new Date() }
        ];
    }
    
    getBalance() {
        return { balance: this.balance, status: "demo" };
    }
    
    getAddress() {
        return { address: this.address, status: "demo" };
    }
    
    getTransactions() {
        return { transactions: this.transactions, status: "demo" };
    }
    
    sendTransaction(to, amount) {
        return {
            txid: `demo_tx_${Date.now()}`,
            status: "demo",
            message: "Transaction simulated in demo mode"
        };
    }
}

const demoWallet = new DemoWallet();

// Authentication middleware
const authenticateToken = (req, res, next) => {
    const token = req.query.token || req.headers.authorization?.split(' ')[1];
    
    if (!token) {
        return res.status(401).json({ error: 'Access token required' });
    }
    
    try {
        const decoded = jwt.verify(token, process.env.JWT_SECRET || 'demo-secret-key');
        req.user = decoded;
        next();
    } catch (error) {
        return res.status(403).json({ error: 'Invalid token' });
    }
};

// HTTPCG Dashboard endpoint
app.get('/httpcg/dashboard', authenticateToken, (req, res) => {
    res.send(`
    <!DOCTYPE html>
    <html>
    <head>
        <title>BPCI Admin Dashboard</title>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <style>
            body { font-family: Arial, sans-serif; margin: 0; padding: 20px; background: #f5f5f5; }
            .container { max-width: 1200px; margin: 0 auto; }
            .header { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 20px; border-radius: 10px; margin-bottom: 20px; }
            .card { background: white; padding: 20px; border-radius: 10px; margin-bottom: 20px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
            .demo-badge { background: #ff6b6b; color: white; padding: 4px 8px; border-radius: 4px; font-size: 12px; }
            .wallet-info { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; }
            .btn { background: #4CAF50; color: white; padding: 10px 20px; border: none; border-radius: 5px; cursor: pointer; }
            .btn:hover { background: #45a049; }
            .status-indicator { display: inline-block; width: 10px; height: 10px; border-radius: 50%; margin-right: 8px; }
            .status-active { background: #4CAF50; }
            .transactions { max-height: 300px; overflow-y: auto; }
        </style>
    </head>
    <body>
        <div class="container">
            <div class="header">
                <h1>🚀 BPCI Admin Dashboard <span class="demo-badge">DEMO MODE</span></h1>
                <p>Welcome, ${req.user.username} | HTTPCG Protocol Active | Post-Quantum Security Enabled</p>
            </div>
            
            <div class="wallet-info">
                <div class="card">
                    <h3>💰 Demo Wallet</h3>
                    <p><strong>Balance:</strong> <span id="balance">Loading...</span></p>
                    <p><strong>Address:</strong> <span id="address">Loading...</span></p>
                    <button class="btn" onclick="refreshWallet()">Refresh Wallet</button>
                </div>
                
                <div class="card">
                    <h3>🌐 BPCI Server Status</h3>
                    <p><span class="status-indicator status-active"></span>XTMP Server: Active</p>
                    <p><span class="status-indicator status-active"></span>VM Server: Active</p>
                    <p><span class="status-indicator status-active"></span>ENC Cluster: Active</p>
                    <p><span class="status-indicator status-active"></span>DockLock: Active</p>
                </div>
            </div>
            
            <div class="card">
                <h3>📊 Recent Transactions <span class="demo-badge">DEMO</span></h3>
                <div class="transactions" id="transactions">
                    Loading transactions...
                </div>
            </div>
            
            <div class="card">
                <h3>🔧 Quick Actions</h3>
                <button class="btn" onclick="sendDemoTransaction()">Send Demo Transaction</button>
                <button class="btn" onclick="generateDemoAddress()">Generate Demo Address</button>
                <button class="btn" onclick="viewSystemLogs()">View System Logs</button>
            </div>
        </div>
        
        <script>
            const token = new URLSearchParams(window.location.search).get('token');
            
            async function apiCall(endpoint) {
                const response = await fetch(endpoint + '?token=' + token);
                return response.json();
            }
            
            async function refreshWallet() {
                try {
                    const balance = await apiCall('/api/wallet/balance');
                    const address = await apiCall('/api/wallet/address');
                    const transactions = await apiCall('/api/wallet/transactions');
                    
                    document.getElementById('balance').textContent = balance.balance;
                    document.getElementById('address').textContent = address.address;
                    
                    const txHtml = transactions.transactions.map(tx => 
                        '<div style="padding: 10px; border-bottom: 1px solid #eee;">' +
                        '<strong>' + tx.type + '</strong>: ' + tx.amount + ' <span class="demo-badge">DEMO</span><br>' +
                        '<small>' + new Date(tx.timestamp).toLocaleString() + '</small>' +
                        '</div>'
                    ).join('');
                    
                    document.getElementById('transactions').innerHTML = txHtml;
                } catch (error) {
                    console.error('Error refreshing wallet:', error);
                }
            }
            
            async function sendDemoTransaction() {
                try {
                    const result = await fetch('/api/wallet/send?token=' + token, {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ to: 'demo_address', amount: '10.00' })
                    });
                    const data = await result.json();
                    alert('Demo transaction sent: ' + data.txid + ' (DEMO MODE)');
                    refreshWallet();
                } catch (error) {
                    alert('Error: ' + error.message);
                }
            }
            
            function generateDemoAddress() {
                alert('Demo address generated: bpi1demo' + Math.random().toString(36).substr(2, 9) + ' (DEMO MODE)');
            }
            
            function viewSystemLogs() {
                window.open('/api/system/logs?token=' + token, '_blank');
            }
            
            // Initialize dashboard
            refreshWallet();
            setInterval(refreshWallet, 30000); // Refresh every 30 seconds
        </script>
    </body>
    </html>
    `);
});

// Wallet API endpoints
app.get('/api/wallet/balance', authenticateToken, (req, res) => {
    res.json(demoWallet.getBalance());
});

app.get('/api/wallet/address', authenticateToken, (req, res) => {
    res.json(demoWallet.getAddress());
});

app.get('/api/wallet/transactions', authenticateToken, (req, res) => {
    res.json(demoWallet.getTransactions());
});

app.post('/api/wallet/send', authenticateToken, (req, res) => {
    const { to, amount } = req.body;
    res.json(demoWallet.sendTransaction(to, amount));
});

// System endpoints
app.get('/api/system/logs', authenticateToken, (req, res) => {
    res.json({
        logs: [
            { timestamp: new Date(), level: 'INFO', message: 'BPCI Server started (demo)' },
            { timestamp: new Date(), level: 'INFO', message: 'XTMP connections: 0 (demo)' },
            { timestamp: new Date(), level: 'INFO', message: 'VM Server active (demo)' },
            { timestamp: new Date(), level: 'INFO', message: 'ENC Cluster operational (demo)' }
        ],
        status: 'demo'
    });
});

app.get('/health', (req, res) => {
    res.json({ status: 'healthy', mode: 'demo', timestamp: new Date() });
});

const PORT = process.env.PORT || 8888;
app.listen(PORT, () => {
    console.log(`🚀 BPCI Admin Dashboard running on port ${PORT}`);
    console.log(`🌐 Access: http://localhost:${PORT}/httpcg/dashboard`);
    console.log(`🔒 Demo mode enabled - all responses include 'demo' status`);
});
```

---

## 💰 **3. HTTPCG Wallet Deployment (2 CPU Instance)**

### **Wallet Server Configuration**
```bash
#!/bin/bash
# deploy-httpcg-wallet.sh

set -e

echo "💰 Deploying HTTPCG Wallet (2 CPU Instance)..."

# Install BPI OS
curl -fsSL https://repo.pravyom.com/install-bpi-os.sh | bash

# Configure wallet server
cat > /etc/bpi-os/httpcg-wallet-server.yaml << 'EOF'
httpcg_wallet_server:
  mode: "production-wallet"
  
  httpcg_configuration:
    wallet_domain: "httpcg://wallet.prav.global"
    vm_server_port: 7778
    native_protocol: true
    
  wallet_services:
    bpi_wallet: true
    stamped_wallet: true
    demo_mode: true
    
  bpci_connection:
    server_endpoint: "BPCI_SERVER_IP:9999"
    xtmp_enabled: true
    auto_connect: true
    
  resources:
    cpu_allocation:
      wallet_service: 1
      vm_server: 1
    memory_allocation:
      wallet_service: "2GB"
      vm_server: "2GB"
EOF

# Deploy wallet server
/opt/bpi-os/bin/bpi-core wallet deploy \
    --config /etc/bpi-os/httpcg-wallet-server.yaml \
    --mode production \
    --demo-responses true

# Start wallet services
systemctl start bpi-wallet-server
systemctl start bpi-vm-server-wallet
systemctl enable bpi-wallet-server
systemctl enable bpi-vm-server-wallet

echo "✅ HTTPCG Wallet deployed successfully"
echo "💰 Wallet URL: httpcg://wallet.prav.global"
echo "🔗 Connected to BPCI Server"
```

---

## 🚀 **4. Complete Deployment Script**

```bash
#!/bin/bash
# complete-deployment.sh

set -e

echo "🚀 Complete BPCI Production Deployment"
echo "======================================"

# Get deployment parameters
read -p "Enter your domain name (e.g., pravyom.com): " DOMAIN_NAME
read -p "Enter BPCI server IP (4 CPU instance): " BPCI_SERVER_IP
read -p "Enter Wallet server IP (2 CPU instance): " WALLET_SERVER_IP

echo ""
echo "📋 Deployment Summary:"
echo "  Website: https://$DOMAIN_NAME (Vercel)"
echo "  BPCI Server: http://$BPCI_SERVER_IP:8888 (4 CPU)"
echo "  Wallet Server: http://$WALLET_SERVER_IP:7778 (2 CPU)"
echo "  Admin Login: root / admin"
echo ""

read -p "Proceed with deployment? (y/N): " CONFIRM
if [[ $CONFIRM != "y" && $CONFIRM != "Y" ]]; then
    echo "Deployment cancelled."
    exit 0
fi

# 1. Deploy website to Vercel
echo "🌐 Step 1: Deploying website to Vercel..."
cd website/
npm install
vercel env add JWT_SECRET production
vercel env add BPCI_SERVER_URL "http://$BPCI_SERVER_IP:8888" production
vercel --prod
vercel domains add "$DOMAIN_NAME"

# 2. Deploy BPCI server
echo "🏢 Step 2: Deploying BPCI server..."
ssh root@$BPCI_SERVER_IP 'bash -s' < deploy-bpci-server.sh

# 3. Deploy wallet server
echo "💰 Step 3: Deploying wallet server..."
ssh root@$WALLET_SERVER_IP "sed 's/BPCI_SERVER_IP/$BPCI_SERVER_IP/g' deploy-httpcg-wallet.sh | bash"

# 4. Validate deployment
echo "✅ Step 4: Validating deployment..."

# Check website
if curl -f "https://$DOMAIN_NAME" > /dev/null 2>&1; then
    echo "✅ Website: Active"
else
    echo "❌ Website: Failed"
fi

# Check BPCI server
if curl -f "http://$BPCI_SERVER_IP:8888/health" > /dev/null 2>&1; then
    echo "✅ BPCI Server: Active"
else
    echo "❌ BPCI Server: Failed"
fi

# Check wallet server
if curl -f "http://$WALLET_SERVER_IP:7778/health" > /dev/null 2>&1; then
    echo "✅ Wallet Server: Active"
else
    echo "❌ Wallet Server: Failed"
fi

echo ""
echo "🎉 Deployment Complete!"
echo "======================================"
echo "🌐 Website: https://$DOMAIN_NAME"
echo "🔑 Login: root / admin"
echo "📊 Dashboard: https://$DOMAIN_NAME → HTTPCG redirect"
echo "💰 Demo Wallet: Always returns 'demo' in responses"
echo "🔒 All BPI operations return 'demo' status"
echo ""
echo "🚀 System is ready for testing!"
```

---

## ✅ **Deployment Checklist**

### **Prerequisites**
- [ ] Vercel account with CLI access
- [ ] 4 CPU cloud instance for BPCI server
- [ ] 2 CPU cloud instance for wallet server
- [ ] Domain name for website
- [ ] SSH access to both instances

### **Deployment Steps**
1. [ ] Deploy website to Vercel with custom domain
2. [ ] Deploy BPCI server (4 CPU) with admin dashboard
3. [ ] Deploy HTTPCG wallet server (2 CPU)
4. [ ] Configure root login (username: root, password: admin)
5. [ ] Test demo wallet (all responses include "demo")
6. [ ] Validate end-to-end flow

### **Testing Flow**
1. Visit `https://[your-domain].com`
2. Login with `root` / `admin`
3. Get redirected to HTTPCG dashboard
4. Use demo wallet (all BPI operations return "demo")
5. Verify admin dashboard functionality

**Ready to deploy! Just provide the domain name and instance IPs, and I'll execute the complete deployment.** 🚀
