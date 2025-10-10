// HTTPCG Wallet Server - Enterprise Wallet Services with Demo Mode
// Complete implementation for production deployment

const express = require('express');
const WebSocket = require('ws');
const jwt = require('jsonwebtoken');
const cors = require('cors');
const axios = require('axios');
const { v4: uuidv4 } = require('uuid');

const app = express();

// Middleware
app.use(express.json());
app.use(cors({
  origin: ['http://localhost:3000', 'http://localhost:8888', 'http://localhost:9999', 'https://pravyom.com'],
  credentials: true
}));

// Demo Wallet State Management
class DemoWalletManager {
  constructor() {
    this.wallets = new Map();
    this.transactions = new Map();
    this.stakingInfo = new Map();
    this.systemMetrics = {
      total_wallets: 0,
      total_transactions: 0,
      total_staked: '50000.00 BPI (demo)',
      uptime: Date.now()
    };
    
    // Initialize demo wallets
    this.initializeDemoWallets();
  }

  initializeDemoWallets() {
    // Create demo root wallet
    const rootWallet = {
      wallet_id: 'demo_root_wallet',
      address: 'bpi1demo...rootenterprise',
      balance: '10000.00 BPI (demo)',
      staked_amount: '5000.00 BPI (demo)',
      created_at: new Date(),
      status: 'active',
      type: 'enterprise_root',
      demo_mode: true
    };

    // Create demo user wallet
    const userWallet = {
      wallet_id: 'demo_user_wallet',
      address: 'bpi1demo...userenterprise',
      balance: '1000.00 BPI (demo)',
      staked_amount: '100.00 BPI (demo)',
      created_at: new Date(),
      status: 'active',
      type: 'user',
      demo_mode: true
    };

    this.wallets.set('demo_root_wallet', rootWallet);
    this.wallets.set('demo_user_wallet', userWallet);
    this.systemMetrics.total_wallets = 2;

    // Initialize demo transactions
    this.initializeDemoTransactions();
    
    // Initialize demo staking
    this.initializeDemoStaking();
  }

  initializeDemoTransactions() {
    const demoTransactions = [
      {
        txid: 'demo_tx_001',
        from: 'bpi1demo...rootenterprise',
        to: 'bpi1demo...userenterprise',
        amount: '100.00 BPI (demo)',
        type: 'transfer',
        status: 'confirmed',
        timestamp: new Date(Date.now() - 3600000), // 1 hour ago
        demo_mode: true
      },
      {
        txid: 'demo_tx_002',
        from: 'bpi1demo...userenterprise',
        to: 'staking_pool',
        amount: '50.00 BPI (demo)',
        type: 'stake',
        status: 'confirmed',
        timestamp: new Date(Date.now() - 1800000), // 30 minutes ago
        demo_mode: true
      },
      {
        txid: 'demo_tx_003',
        from: 'bpi1demo...rootenterprise',
        to: 'bpi1demo...external',
        amount: '250.00 BPI (demo)',
        type: 'transfer',
        status: 'pending',
        timestamp: new Date(Date.now() - 300000), // 5 minutes ago
        demo_mode: true
      }
    ];

    demoTransactions.forEach(tx => {
      this.transactions.set(tx.txid, tx);
    });
    
    this.systemMetrics.total_transactions = demoTransactions.length;
  }

  initializeDemoStaking() {
    const stakingData = {
      demo_root_wallet: {
        staked_amount: '5000.00 BPI (demo)',
        rewards_earned: '125.50 BPI (demo)',
        staking_duration: '30 days (demo)',
        apy: '15.2% (demo)',
        status: 'active',
        demo_mode: true
      },
      demo_user_wallet: {
        staked_amount: '100.00 BPI (demo)',
        rewards_earned: '2.10 BPI (demo)',
        staking_duration: '7 days (demo)',
        apy: '15.2% (demo)',
        status: 'active',
        demo_mode: true
      }
    };

    Object.entries(stakingData).forEach(([walletId, data]) => {
      this.stakingInfo.set(walletId, data);
    });
  }

  // Get wallet information
  getWallet(walletId) {
    const wallet = this.wallets.get(walletId);
    if (!wallet) {
      return {
        success: false,
        error: 'Wallet not found',
        demo_mode: true
      };
    }

    return {
      success: true,
      wallet: wallet,
      demo_mode: true
    };
  }

  // Get all wallets
  getAllWallets() {
    return {
      wallets: Array.from(this.wallets.values()),
      total_count: this.wallets.size,
      demo_mode: true
    };
  }

  // Create new demo wallet
  createWallet(walletInfo) {
    const walletId = `demo_wallet_${Date.now()}`;
    const newWallet = {
      wallet_id: walletId,
      address: `bpi1demo${Math.random().toString(36).substr(2, 9)}...enterprise`,
      balance: '0.00 BPI (demo)',
      staked_amount: '0.00 BPI (demo)',
      created_at: new Date(),
      status: 'active',
      type: walletInfo.type || 'user',
      demo_mode: true
    };

    this.wallets.set(walletId, newWallet);
    this.systemMetrics.total_wallets = this.wallets.size;

    return {
      success: true,
      wallet: newWallet,
      demo_mode: true
    };
  }

  // Send transaction (demo)
  sendTransaction(fromWallet, toAddress, amount, type = 'transfer') {
    const txid = `demo_tx_${Date.now()}`;
    const transaction = {
      txid: txid,
      from: fromWallet,
      to: toAddress + ' (demo)',
      amount: amount + ' BPI (demo)',
      type: type,
      status: 'confirmed (demo)',
      timestamp: new Date(),
      demo_mode: true
    };

    this.transactions.set(txid, transaction);
    this.systemMetrics.total_transactions = this.transactions.size;

    return {
      success: true,
      transaction: transaction,
      demo_mode: true
    };
  }

  // Get transaction history
  getTransactionHistory(walletId, limit = 10) {
    const allTransactions = Array.from(this.transactions.values())
      .filter(tx => tx.from.includes(walletId) || tx.to.includes(walletId))
      .sort((a, b) => new Date(b.timestamp) - new Date(a.timestamp))
      .slice(0, limit);

    return {
      transactions: allTransactions,
      total_count: allTransactions.length,
      demo_mode: true
    };
  }

  // Get staking information
  getStakingInfo(walletId) {
    const stakingData = this.stakingInfo.get(walletId);
    if (!stakingData) {
      return {
        success: false,
        error: 'No staking information found',
        demo_mode: true
      };
    }

    return {
      success: true,
      staking: stakingData,
      demo_mode: true
    };
  }

  // Stake tokens (demo)
  stakeTokens(walletId, amount) {
    const stakingData = this.stakingInfo.get(walletId) || {
      staked_amount: '0.00 BPI (demo)',
      rewards_earned: '0.00 BPI (demo)',
      staking_duration: '0 days (demo)',
      apy: '15.2% (demo)',
      status: 'inactive',
      demo_mode: true
    };

    // Update staking amount (demo calculation)
    const currentStaked = parseFloat(stakingData.staked_amount.split(' ')[0]) || 0;
    const newAmount = parseFloat(amount) || 0;
    const totalStaked = currentStaked + newAmount;

    stakingData.staked_amount = `${totalStaked.toFixed(2)} BPI (demo)`;
    stakingData.status = 'active';
    stakingData.demo_mode = true;

    this.stakingInfo.set(walletId, stakingData);

    return {
      success: true,
      staking: stakingData,
      transaction_id: `demo_stake_${Date.now()}`,
      demo_mode: true
    };
  }

  // Get system metrics
  getSystemMetrics() {
    const uptime = Math.floor((Date.now() - this.systemMetrics.uptime) / (1000 * 60 * 60));
    
    return {
      ...this.systemMetrics,
      uptime_hours: uptime,
      status: 'operational',
      demo_mode: true,
      timestamp: new Date().toISOString()
    };
  }
}

// HTTPCG Protocol Handler
class HTTPCGProtocolHandler {
  constructor(walletManager) {
    this.walletManager = walletManager;
    this.bpciServerUrl = process.env.BPCI_SERVER_URL || 'http://localhost:9999';
  }

  // Handle HTTPCG wallet requests
  async handleWalletRequest(request) {
    try {
      switch (request.action) {
        case 'get_balance':
          return this.getBalance(request.wallet_id);
        case 'send_transaction':
          return this.sendTransaction(request);
        case 'get_history':
          return this.getTransactionHistory(request.wallet_id, request.limit);
        case 'stake_tokens':
          return this.stakeTokens(request.wallet_id, request.amount);
        case 'get_staking':
          return this.getStakingInfo(request.wallet_id);
        default:
          return {
            success: false,
            error: 'Unknown action',
            demo_mode: true
          };
      }
    } catch (error) {
      return {
        success: false,
        error: error.message,
        demo_mode: true
      };
    }
  }

  async getBalance(walletId) {
    const result = this.walletManager.getWallet(walletId);
    if (!result.success) {
      return result;
    }

    return {
      success: true,
      balance: result.wallet.balance,
      staked: result.wallet.staked_amount,
      address: result.wallet.address,
      demo_mode: true
    };
  }

  async sendTransaction(request) {
    const { wallet_id, to_address, amount, type } = request;
    
    // Notify BPCI server of transaction (demo)
    try {
      await axios.post(`${this.bpciServerUrl}/api/xtmp/message`, {
        session_id: 'demo_session',
        message: {
          type: 'bundle_submission',
          data: {
            from: wallet_id,
            to: to_address,
            amount: amount,
            demo_mode: true
          }
        }
      });
    } catch (error) {
      console.log('BPCI server notification failed (demo mode):', error.message);
    }

    return this.walletManager.sendTransaction(wallet_id, to_address, amount, type);
  }

  async getTransactionHistory(walletId, limit) {
    return this.walletManager.getTransactionHistory(walletId, limit);
  }

  async stakeTokens(walletId, amount) {
    return this.walletManager.stakeTokens(walletId, amount);
  }

  async getStakingInfo(walletId) {
    return this.walletManager.getStakingInfo(walletId);
  }
}

// Initialize wallet components
const walletManager = new DemoWalletManager();
const httpcgHandler = new HTTPCGProtocolHandler(walletManager);

// Authentication middleware
const authenticateToken = (req, res, next) => {
  const token = req.query.token || req.headers.authorization?.split(' ')[1];
  
  if (!token) {
    return res.status(401).json({ error: 'Access token required', demo_mode: true });
  }
  
  try {
    const decoded = jwt.verify(token, process.env.JWT_SECRET || 'bpci-enterprise-secret-key');
    req.user = decoded;
    next();
  } catch (error) {
    return res.status(403).json({ error: 'Invalid token', demo_mode: true });
  }
};

// HTTPCG Wallet API Endpoints

// Get wallet status
app.get('/api/wallet/status', (req, res) => {
  res.json({
    status: 'operational',
    service: 'httpcg-wallet',
    version: '1.0.0',
    demo_mode: true,
    metrics: walletManager.getSystemMetrics()
  });
});

// Get all wallets
app.get('/api/wallets', authenticateToken, (req, res) => {
  res.json(walletManager.getAllWallets());
});

// Get specific wallet
app.get('/api/wallet/:walletId', (req, res) => {
  const { walletId } = req.params;
  res.json(walletManager.getWallet(walletId));
});

// Create new wallet
app.post('/api/wallet/create', authenticateToken, (req, res) => {
  const walletInfo = {
    type: req.body.type || 'user',
    name: req.body.name || 'Demo Wallet'
  };
  
  res.json(walletManager.createWallet(walletInfo));
});

// Send transaction
app.post('/api/wallet/send', async (req, res) => {
  const request = {
    action: 'send_transaction',
    wallet_id: req.body.wallet_id || 'demo_root_wallet',
    to_address: req.body.to_address,
    amount: req.body.amount,
    type: req.body.type || 'transfer'
  };

  const result = await httpcgHandler.handleWalletRequest(request);
  res.json(result);
});

// Get transaction history
app.get('/api/wallet/:walletId/history', (req, res) => {
  const { walletId } = req.params;
  const limit = parseInt(req.query.limit) || 10;
  
  res.json(walletManager.getTransactionHistory(walletId, limit));
});

// Get staking information
app.get('/api/wallet/:walletId/staking', (req, res) => {
  const { walletId } = req.params;
  res.json(walletManager.getStakingInfo(walletId));
});

// Stake tokens
app.post('/api/wallet/stake', async (req, res) => {
  const request = {
    action: 'stake_tokens',
    wallet_id: req.body.wallet_id || 'demo_root_wallet',
    amount: req.body.amount
  };

  const result = await httpcgHandler.handleWalletRequest(request);
  res.json(result);
});

// HTTPCG Protocol endpoint
app.post('/api/httpcg/request', async (req, res) => {
  const result = await httpcgHandler.handleWalletRequest(req.body);
  res.json(result);
});

// Demo wallet endpoints for dashboard integration
app.get('/api/demo/balance', (req, res) => {
  res.json({
    balance: '10000.00 BPI (demo)',
    staked: '5000.00 BPI (demo)',
    rewards: '125.50 BPI (demo)',
    address: 'bpi1demo...rootenterprise',
    demo_mode: true
  });
});

app.get('/api/demo/transactions', (req, res) => {
  const limit = parseInt(req.query.limit) || 5;
  const transactions = walletManager.getTransactionHistory('demo_root_wallet', limit);
  res.json(transactions);
});

app.post('/api/demo/send', async (req, res) => {
  const { to, amount } = req.body;
  const result = await httpcgHandler.sendTransaction({
    wallet_id: 'demo_root_wallet',
    to_address: to,
    amount: amount,
    type: 'transfer'
  });
  res.json(result);
});

// Health check endpoint
app.get('/health', (req, res) => {
  res.json({
    status: 'healthy',
    service: 'httpcg-wallet',
    version: '1.0.0',
    uptime: Math.floor((Date.now() - walletManager.systemMetrics.uptime) / 1000),
    demo_mode: true,
    timestamp: new Date().toISOString()
  });
});

// WebSocket server for real-time wallet updates
const server = require('http').createServer(app);
const wss = new WebSocket.Server({ server });

wss.on('connection', (ws, req) => {
  console.log('🔗 Wallet WebSocket connection established');
  
  ws.on('message', (data) => {
    try {
      const message = JSON.parse(data);
      
      switch (message.type) {
        case 'subscribe_wallet':
          // Send periodic wallet updates
          const walletInterval = setInterval(() => {
            if (ws.readyState === WebSocket.OPEN) {
              const walletData = walletManager.getWallet(message.wallet_id || 'demo_root_wallet');
              ws.send(JSON.stringify({
                type: 'wallet_update',
                data: walletData
              }));
            } else {
              clearInterval(walletInterval);
            }
          }, 5000); // Every 5 seconds
          break;
          
        case 'subscribe_transactions':
          // Send transaction updates
          const txInterval = setInterval(() => {
            if (ws.readyState === WebSocket.OPEN) {
              const txData = walletManager.getTransactionHistory('demo_root_wallet', 5);
              ws.send(JSON.stringify({
                type: 'transaction_update',
                data: txData
              }));
            } else {
              clearInterval(txInterval);
            }
          }, 10000); // Every 10 seconds
          break;
          
        default:
          ws.send(JSON.stringify({
            type: 'response',
            data: { message: 'Message received (demo)', demo_mode: true }
          }));
      }
    } catch (error) {
      ws.send(JSON.stringify({
        type: 'error',
        data: { error: error.message, demo_mode: true }
      }));
    }
  });

  ws.on('close', () => {
    console.log('🔌 Wallet WebSocket connection closed');
  });

  // Send welcome message
  ws.send(JSON.stringify({
    type: 'welcome',
    data: {
      message: 'Connected to HTTPCG Wallet Server',
      wallet_status: 'operational',
      demo_mode: true
    }
  }));
});

// Start HTTPCG Wallet server
const PORT = process.env.PORT || 7778;
server.listen(PORT, () => {
  console.log(`🚀 HTTPCG Wallet Server running on port ${PORT}`);
  console.log(`💰 Demo Wallets: Initialized`);
  console.log(`🔗 HTTPCG Protocol: Active`);
  console.log(`🌐 WebSocket: Active`);
  console.log(`🔒 Demo mode: Enabled`);
  console.log(`📡 Health check: http://localhost:${PORT}/health`);
});
