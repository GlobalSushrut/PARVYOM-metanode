// BPCI Enterprise Admin Dashboard Server
// Integrated within BPCI Enterprise System

const express = require('express');
const jwt = require('jsonwebtoken');
const path = require('path');
const cors = require('cors');

const app = express();

// Middleware
app.use(express.json());
app.use(express.static('public'));
app.use(cors({
  origin: ['http://localhost:3000', 'https://pravyom.com'],
  credentials: true
}));

// Demo wallet that always returns "demo" status
class BPCIDemoWallet {
    constructor() {
        this.balance = "1000.00 BPI (demo)";
        this.address = "bpi1demo...enterprise (demo)";
        this.transactions = [
            { 
                id: "tx_demo_001", 
                amount: "100.00 BPI (demo)", 
                type: "receive", 
                timestamp: new Date(),
                from: "bpi1system...coordinator (demo)",
                status: "demo"
            },
            { 
                id: "tx_demo_002", 
                amount: "50.00 BPI (demo)", 
                type: "send", 
                timestamp: new Date(),
                to: "bpi1wallet...instance (demo)",
                status: "demo"
            },
            { 
                id: "tx_demo_003", 
                amount: "25.00 BPI (demo)", 
                type: "stake", 
                timestamp: new Date(),
                validator: "bpi1validator...node (demo)",
                status: "demo"
            }
        ];
        this.stakingInfo = {
            staked_amount: "500.00 BPI (demo)",
            rewards: "12.50 BPI (demo)",
            validators: 3,
            status: "demo"
        };
    }
    
    getBalance() {
        return { 
            balance: this.balance, 
            status: "demo",
            currency: "BPI",
            demo_mode: true
        };
    }
    
    getAddress() {
        return { 
            address: this.address, 
            status: "demo",
            network: "BPCI Enterprise Testnet",
            demo_mode: true
        };
    }
    
    getTransactions() {
        return { 
            transactions: this.transactions, 
            status: "demo",
            total_count: this.transactions.length,
            demo_mode: true
        };
    }
    
    sendTransaction(to, amount) {
        const newTx = {
            txid: `demo_tx_${Date.now()}`,
            to: to + " (demo)",
            amount: amount + " BPI (demo)",
            timestamp: new Date(),
            status: "demo",
            demo_mode: true,
            message: "Transaction simulated in demo mode"
        };
        
        this.transactions.unshift(newTx);
        return newTx;
    }
    
    getStakingInfo() {
        return {
            ...this.stakingInfo,
            demo_mode: true
        };
    }
}

const bpciDemoWallet = new BPCIDemoWallet();

// BPCI System Status
class BPCISystemMonitor {
    constructor() {
        this.systemMetrics = {
            uptime: Date.now(),
            cpu_usage: "15% (demo)",
            memory_usage: "2.1GB / 8GB (demo)",
            network_connections: "127 active (demo)",
            httpcg_requests: "1,247 today (demo)",
            xtmp_sessions: "89 active (demo)"
        };
    }
    
    getSystemStatus() {
        return {
            status: "operational",
            components: {
                bpci_server: "online (demo)",
                xtmp_server: "online (demo)", 
                vm_server: "online (demo)",
                httpcg_protocol: "active (demo)",
                enc_cluster: "secured (demo)",
                docklock: "compliant (demo)"
            },
            metrics: this.systemMetrics,
            demo_mode: true
        };
    }
    
    getBPCILogs() {
        return {
            logs: [
                { 
                    timestamp: new Date(), 
                    level: 'INFO', 
                    component: 'BPCI-SERVER',
                    message: 'Enterprise system initialized (demo)' 
                },
                { 
                    timestamp: new Date(), 
                    level: 'INFO', 
                    component: 'XTMP-SERVER',
                    message: 'High-performance connections: 89 active (demo)' 
                },
                { 
                    timestamp: new Date(), 
                    level: 'INFO', 
                    component: 'VM-SERVER',
                    message: 'HTTPCG applications deployed: 5 (demo)' 
                },
                { 
                    timestamp: new Date(), 
                    level: 'INFO', 
                    component: 'ENC-CLUSTER',
                    message: 'Military-grade encryption active (demo)' 
                },
                { 
                    timestamp: new Date(), 
                    level: 'INFO', 
                    component: 'DOCKLOCK',
                    message: 'Compliance framework operational (demo)' 
                }
            ],
            status: "demo",
            demo_mode: true
        };
    }
}

const bpciSystemMonitor = new BPCISystemMonitor();

// Authentication middleware
const authenticateToken = (req, res, next) => {
    const token = req.query.token || req.headers.authorization?.split(' ')[1];
    
    if (!token) {
        return res.status(401).json({ error: 'Access token required' });
    }
    
    try {
        const decoded = jwt.verify(token, process.env.JWT_SECRET || 'bpci-enterprise-secret-key');
        req.user = decoded;
        next();
    } catch (error) {
        return res.status(403).json({ error: 'Invalid token' });
    }
};

// HTTPCG Dashboard endpoint - Main admin interface
app.get('/httpcg/dashboard', authenticateToken, (req, res) => {
    const systemStatus = bpciSystemMonitor.getSystemStatus();
    const walletInfo = bpciDemoWallet.getBalance();
    
    res.send(`
    <!DOCTYPE html>
    <html>
    <head>
        <title>BPCI Enterprise Admin Dashboard</title>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <link href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" rel="stylesheet">
        <link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0/css/all.min.css" rel="stylesheet">
        <style>
            .gradient-bg { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); }
            .card-hover:hover { transform: translateY(-2px); transition: all 0.3s ease; }
            .demo-badge { background: #ff6b6b; color: white; padding: 4px 8px; border-radius: 4px; font-size: 12px; }
            .status-indicator { display: inline-block; width: 10px; height: 10px; border-radius: 50%; margin-right: 8px; }
            .status-online { background: #10b981; }
            .status-demo { background: #f59e0b; }
            .metric-card { background: rgba(255,255,255,0.1); backdrop-filter: blur(10px); }
        </style>
    </head>
    <body class="bg-gray-900 text-white">
        <!-- Header -->
        <header class="gradient-bg shadow-lg">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center py-4">
                    <div class="flex items-center space-x-4">
                        <i class="fas fa-server text-2xl"></i>
                        <div>
                            <h1 class="text-2xl font-bold">BPCI Enterprise</h1>
                            <p class="text-sm opacity-90">Admin Dashboard <span class="demo-badge">DEMO MODE</span></p>
                        </div>
                    </div>
                    <div class="flex items-center space-x-4">
                        <div class="text-right">
                            <p class="text-sm font-semibold">${req.user.name}</p>
                            <p class="text-xs opacity-75">${req.user.role}</p>
                        </div>
                        <div class="w-10 h-10 bg-white/20 rounded-full flex items-center justify-center">
                            <i class="fas fa-user"></i>
                        </div>
                    </div>
                </div>
            </div>
        </header>

        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
            <!-- System Status Overview -->
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                <div class="metric-card rounded-lg p-6 card-hover">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-300">BPCI Server</p>
                            <p class="text-2xl font-bold text-green-400">Online</p>
                        </div>
                        <i class="fas fa-server text-3xl text-green-400"></i>
                    </div>
                    <p class="text-xs text-gray-400 mt-2">Enterprise coordination active</p>
                </div>
                
                <div class="metric-card rounded-lg p-6 card-hover">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-300">HTTPCG Protocol</p>
                            <p class="text-2xl font-bold text-blue-400">Active</p>
                        </div>
                        <i class="fas fa-globe text-3xl text-blue-400"></i>
                    </div>
                    <p class="text-xs text-gray-400 mt-2">Next-gen Web3 communication</p>
                </div>
                
                <div class="metric-card rounded-lg p-6 card-hover">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-300">Security Level</p>
                            <p class="text-2xl font-bold text-purple-400">Military</p>
                        </div>
                        <i class="fas fa-shield-alt text-3xl text-purple-400"></i>
                    </div>
                    <p class="text-xs text-gray-400 mt-2">Post-quantum encryption</p>
                </div>
                
                <div class="metric-card rounded-lg p-6 card-hover">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-300">Demo Wallet</p>
                            <p class="text-2xl font-bold text-yellow-400">${walletInfo.balance}</p>
                        </div>
                        <i class="fas fa-wallet text-3xl text-yellow-400"></i>
                    </div>
                    <p class="text-xs text-gray-400 mt-2">Enterprise test balance</p>
                </div>
            </div>

            <!-- Main Dashboard Grid -->
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                <!-- BPCI System Components -->
                <div class="bg-gray-800 rounded-lg p-6">
                    <h3 class="text-xl font-bold mb-4 flex items-center">
                        <i class="fas fa-cogs mr-2 text-blue-400"></i>
                        BPCI System Components
                    </h3>
                    <div class="space-y-4">
                        ${Object.entries(systemStatus.components).map(([component, status]) => `
                            <div class="flex items-center justify-between p-3 bg-gray-700 rounded">
                                <div class="flex items-center">
                                    <span class="status-indicator status-online"></span>
                                    <span class="capitalize">${component.replace(/_/g, ' ')}</span>
                                </div>
                                <span class="text-green-400 text-sm">${status}</span>
                            </div>
                        `).join('')}
                    </div>
                </div>

                <!-- Demo Wallet Interface -->
                <div class="bg-gray-800 rounded-lg p-6">
                    <h3 class="text-xl font-bold mb-4 flex items-center">
                        <i class="fas fa-wallet mr-2 text-yellow-400"></i>
                        Demo Wallet <span class="demo-badge ml-2">DEMO</span>
                    </h3>
                    <div class="space-y-4">
                        <div class="p-4 bg-gray-700 rounded">
                            <p class="text-sm text-gray-300">Balance</p>
                            <p class="text-2xl font-bold text-yellow-400" id="wallet-balance">${walletInfo.balance}</p>
                        </div>
                        <div class="p-4 bg-gray-700 rounded">
                            <p class="text-sm text-gray-300">Address</p>
                            <p class="text-sm font-mono text-blue-400" id="wallet-address">Loading...</p>
                        </div>
                        <div class="flex space-x-2">
                            <button onclick="sendDemoTransaction()" class="flex-1 bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded text-sm">
                                <i class="fas fa-paper-plane mr-2"></i>Send Demo TX
                            </button>
                            <button onclick="refreshWallet()" class="flex-1 bg-green-600 hover:bg-green-700 px-4 py-2 rounded text-sm">
                                <i class="fas fa-sync mr-2"></i>Refresh
                            </button>
                        </div>
                    </div>
                </div>

                <!-- Recent Transactions -->
                <div class="bg-gray-800 rounded-lg p-6">
                    <h3 class="text-xl font-bold mb-4 flex items-center">
                        <i class="fas fa-exchange-alt mr-2 text-green-400"></i>
                        Recent Transactions <span class="demo-badge ml-2">DEMO</span>
                    </h3>
                    <div class="space-y-3 max-h-64 overflow-y-auto" id="transactions-list">
                        Loading transactions...
                    </div>
                </div>

                <!-- System Metrics -->
                <div class="bg-gray-800 rounded-lg p-6">
                    <h3 class="text-xl font-bold mb-4 flex items-center">
                        <i class="fas fa-chart-line mr-2 text-purple-400"></i>
                        System Metrics <span class="demo-badge ml-2">DEMO</span>
                    </h3>
                    <div class="space-y-3">
                        ${Object.entries(systemStatus.metrics).map(([metric, value]) => `
                            <div class="flex justify-between items-center p-2 bg-gray-700 rounded">
                                <span class="text-sm capitalize">${metric.replace(/_/g, ' ')}</span>
                                <span class="text-sm text-blue-400">${value}</span>
                            </div>
                        `).join('')}
                    </div>
                </div>
            </div>

            <!-- Quick Actions -->
            <div class="mt-8 bg-gray-800 rounded-lg p-6">
                <h3 class="text-xl font-bold mb-4 flex items-center">
                    <i class="fas fa-bolt mr-2 text-yellow-400"></i>
                    Quick Actions
                </h3>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                    <button onclick="viewSystemLogs()" class="bg-blue-600 hover:bg-blue-700 p-4 rounded text-center">
                        <i class="fas fa-file-alt text-2xl mb-2"></i>
                        <p class="text-sm">System Logs</p>
                    </button>
                    <button onclick="generateDemoAddress()" class="bg-green-600 hover:bg-green-700 p-4 rounded text-center">
                        <i class="fas fa-key text-2xl mb-2"></i>
                        <p class="text-sm">Generate Address</p>
                    </button>
                    <button onclick="testHTTPCG()" class="bg-purple-600 hover:bg-purple-700 p-4 rounded text-center">
                        <i class="fas fa-globe text-2xl mb-2"></i>
                        <p class="text-sm">Test HTTPCG</p>
                    </button>
                    <button onclick="exportData()" class="bg-orange-600 hover:bg-orange-700 p-4 rounded text-center">
                        <i class="fas fa-download text-2xl mb-2"></i>
                        <p class="text-sm">Export Data</p>
                    </button>
                </div>
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
                    const [balance, address, transactions] = await Promise.all([
                        apiCall('/api/wallet/balance'),
                        apiCall('/api/wallet/address'),
                        apiCall('/api/wallet/transactions')
                    ]);
                    
                    document.getElementById('wallet-balance').textContent = balance.balance;
                    document.getElementById('wallet-address').textContent = address.address;
                    
                    const txHtml = transactions.transactions.map(tx => 
                        '<div class="p-3 bg-gray-700 rounded">' +
                        '<div class="flex justify-between items-center">' +
                        '<span class="font-semibold capitalize">' + tx.type + '</span>' +
                        '<span class="text-sm text-gray-400">' + new Date(tx.timestamp).toLocaleString() + '</span>' +
                        '</div>' +
                        '<div class="text-sm text-blue-400">' + tx.amount + '</div>' +
                        '<div class="text-xs text-gray-500">ID: ' + tx.id + '</div>' +
                        '</div>'
                    ).join('');
                    
                    document.getElementById('transactions-list').innerHTML = txHtml;
                } catch (error) {
                    console.error('Error refreshing wallet:', error);
                }
            }
            
            async function sendDemoTransaction() {
                try {
                    const result = await fetch('/api/wallet/send?token=' + token, {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ to: 'bpi1demo...recipient', amount: '10.00' })
                    });
                    const data = await result.json();
                    alert('Demo transaction sent: ' + data.txid + ' (DEMO MODE)');
                    refreshWallet();
                } catch (error) {
                    alert('Error: ' + error.message);
                }
            }
            
            function generateDemoAddress() {
                const demoAddress = 'bpi1demo' + Math.random().toString(36).substr(2, 9) + '...enterprise';
                alert('Demo address generated: ' + demoAddress + ' (DEMO MODE)');
            }
            
            function testHTTPCG() {
                alert('HTTPCG Protocol Test:\\n\\n✅ Domain Resolution: Active\\n✅ Post-Quantum Security: Enabled\\n✅ Military-Grade Encryption: Active\\n\\n(DEMO MODE)');
            }
            
            function exportData() {
                alert('Demo data export initiated (DEMO MODE)');
            }
            
            function viewSystemLogs() {
                window.open('/api/system/logs?token=' + token, '_blank');
            }
            
            // Initialize dashboard
            refreshWallet();
            setInterval(refreshWallet, 30000); // Refresh every 30 seconds
            
            // Auto-refresh system status
            setInterval(() => {
                location.reload();
            }, 300000); // Refresh page every 5 minutes
        </script>
    </body>
    </html>
    `);
});

// Wallet API endpoints (all return demo data)
app.get('/api/wallet/balance', authenticateToken, (req, res) => {
    res.json(bpciDemoWallet.getBalance());
});

app.get('/api/wallet/address', authenticateToken, (req, res) => {
    res.json(bpciDemoWallet.getAddress());
});

app.get('/api/wallet/transactions', authenticateToken, (req, res) => {
    res.json(bpciDemoWallet.getTransactions());
});

app.post('/api/wallet/send', authenticateToken, (req, res) => {
    const { to, amount } = req.body;
    res.json(bpciDemoWallet.sendTransaction(to, amount));
});

app.get('/api/wallet/staking', authenticateToken, (req, res) => {
    res.json(bpciDemoWallet.getStakingInfo());
});

// System endpoints
app.get('/api/system/status', authenticateToken, (req, res) => {
    res.json(bpciSystemMonitor.getSystemStatus());
});

app.get('/api/system/logs', authenticateToken, (req, res) => {
    res.json(bpciSystemMonitor.getBPCILogs());
});

// Health check endpoint
app.get('/health', (req, res) => {
    res.json({ 
        status: 'healthy', 
        service: 'bpci-admin-dashboard',
        mode: 'demo', 
        timestamp: new Date().toISOString(),
        version: '1.0.0'
    });
});

// Start server
const PORT = process.env.PORT || 8888;
app.listen(PORT, () => {
    console.log(`🚀 BPCI Enterprise Admin Dashboard running on port ${PORT}`);
    console.log(`🌐 Access: http://localhost:${PORT}/httpcg/dashboard`);
    console.log(`🔒 Demo mode enabled - all responses include 'demo' status`);
    console.log(`📊 System monitoring active`);
});
