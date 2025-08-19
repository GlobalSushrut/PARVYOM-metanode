 
    try {
        const { username, email, password } = req.body;
        
        if (users.has(email)) {
            return res.status(400).json({ error: 'User already exists' });
        }

        const hashedPassword = await bcrypt.hash(password, 10);
        const userId = uuidv4();
        
        // Create Pravyom account (receipt-based, not Ethereum wallet)
        const pravyomAccountId = `pravyom_${userId.substring(0, 8)}`;
        const accountAddress = `0x${Buffer.from(pravyomAccountId).toString('hex').padStart(40, '0')}`;
        
        const user = {
            id: userId,
            username,
            email,
            password: hashedPassword,
            pravyomAccountId,
            accountAddress,
            createdAt: new Date().toISOString(),
            kycStatus: 'pending',
            accountType: 'standard',
            receiptHistory: []
        };

        users.set(email, user);
        wallets.set(userId, { accountId: pravyomAccountId, address: accountAddress });

        // Create default account
        const accountId = uuidv4();
        const account = {
            id: accountId,
            userId,
            accountNumber: `PRAVYOM-${Date.now()}`,
            balance: '0',
            currency: 'PRAVYOM',
            type: 'checking',
            status: 'active',
            createdAt: new Date().toISOString()
        };

        accounts.set(accountId, account);

        const token = jwt.sign(
            { userId, email },
            process.env.JWT_SECRET || 'pravyom-secret',
            { expiresIn: '24h' }
        );

        res.status(201).json({
            message: 'User registered successfully',
            user: {
                id: userId,
                username,
                email,
                walletAddress: wallet.address,
                accountNumber: account.accountNumber
            },
            token
        });

    } catch (error) {
        console.error('Registration error:', error);
        res.status(500).json({ error: 'Registration failed' });
    }
});

// User login
app.post('/api/auth/login', async (req, res) => {
    try {
        const { email, password } = req.body;
        const user = users.get(email);

        if (!user || !await bcrypt.compare(password, user.password)) {
            return res.status(401).json({ error: 'Invalid credentials' });
        }

        const token = jwt.sign(
            { userId: user.id, email },
            process.env.JWT_SECRET || 'pravyom-secret',
            { expiresIn: '24h' }
        );

        res.json({
            message: 'Login successful',
            user: {
                id: user.id,
                username: user.username,
                email: user.email,
                walletAddress: user.walletAddress
            },
            token
        });

    } catch (error) {
        console.error('Login error:', error);
        res.status(500).json({ error: 'Login failed' });
    }
});

// Get account balance
app.get('/api/accounts/:accountId/balance', authenticateToken, async (req, res) => {
    try {
        const { accountId } = req.params;
        const account = accounts.get(accountId);

        if (!account || account.userId !== req.user.userId) {
            return res.status(404).json({ error: 'Account not found' });
        }

        // Get Pravyom account balance using BPCI protocol
        const wallet = wallets.get(req.user.userId);
        let blockchainBalance = '0';
        let receiptCount = 0;
        
        try {
            const networkInfo = await pravyomClient.getNetworkInfo();
            if (networkInfo.success) {
                blockchainBalance = '1000.0'; // Simulated balance from BPCI
                receiptCount = Math.floor(Math.random() * 50); // Simulated receipt count
            }
        } catch (error) {
            console.log('BPCI connection error:', error.message);
        }

        res.json({
            accountId,
            accountNumber: account.accountNumber,
            balance: account.balance,
            blockchainBalance,
            receiptCount,
            pravyomAccountId: wallet.accountId,
            currency: account.currency,
            lastUpdated: new Date().toISOString()
        });

    } catch (error) {
        console.error('Balance check error:', error);
        res.status(500).json({ error: 'Failed to get balance' });
    }
});

// Create transaction
app.post('/api/transactions', authenticateToken, async (req, res) => {
    try {
        const { fromAccount, toAccount, amount, description } = req.body;
        const transactionId = uuidv4();

        const fromAcc = accounts.get(fromAccount);
        if (!fromAcc || fromAcc.userId !== req.user.userId) {
            return res.status(404).json({ error: 'Source account not found' });
        }

        const transaction = {
            id: transactionId,
            fromAccount,
            toAccount,
            amount: parseFloat(amount),
            description: description || 'Transfer',
            status: 'pending',
            createdAt: new Date().toISOString(),
            userId: req.user.userId,
            blockchainTxHash: null
        };

        // Simulate blockchain transaction
        try {
            const wallet = wallets.get(req.user.userId);
            console.log(`Simulating blockchain transaction from ${wallet.address}`);
            
            // In a real implementation, you would send the transaction to the blockchain
            transaction.blockchainTxHash = `0x${Math.random().toString(16).substr(2, 64)}`;
            transaction.status = 'completed';
            
            // Update balances
            fromAcc.balance = (parseFloat(fromAcc.balance) - parseFloat(amount)).toString();
            
        } catch (error) {
            console.log('Blockchain transaction simulation:', error.message);
            transaction.status = 'failed';
        }

        transactions.set(transactionId, transaction);

        res.status(201).json({
            message: 'Transaction created',
            transaction
        });

    } catch (error) {
        console.error('Transaction error:', error);
        res.status(500).json({ error: 'Transaction failed' });
    }
});

// Get transactions
app.get('/api/transactions', authenticateToken, (req, res) => {
    try {
        const userTransactions = Array.from(transactions.values())
            .filter(tx => tx.userId === req.user.userId)
            .sort((a, b) => new Date(b.createdAt) - new Date(a.createdAt));

        res.json({
            transactions: userTransactions,
            count: userTransactions.length
        });

    } catch (error) {
        console.error('Get transactions error:', error);
        res.status(500).json({ error: 'Failed to get transactions' });
    }
});

// Pravyom blockchain integration endpoints

// Get Pravyom network status
app.get('/api/blockchain/status', async (req, res) => {
    try {
        const networkInfo = await pravyomClient.getNetworkInfo();
        const blockHeight = await pravyomClient.getBlockHeight();
        
        if (networkInfo.success && blockHeight.success) {
            res.json({
                connected: true,
                blockHeight: blockHeight.blockHeight,
                network: networkInfo.network,
                protocol: 'BPCI',
                bpciUrl: pravyomClient.bpciUrl,
                bpiUrl: pravyomClient.bpiUrl,
                nodeInfo: networkInfo.node
            });
        } else {
            throw new Error('Network connection failed');
        }

    } catch (error) {
        res.json({
            connected: false,
            error: error.message,
            protocol: 'BPCI',
            bpciUrl: pravyomClient.bpciUrl
        });
    }
});

// Get Pravyom account info
app.get('/api/wallet/info', authenticateToken, async (req, res) => {
    try {
        const wallet = wallets.get(req.user.userId);
        if (!wallet) {
            return res.status(404).json({ error: 'Pravyom account not found' });
        }

        let balance = '0';
        let receiptCount = 0;
        let networkConnected = false;

        try {
            const networkInfo = await pravyomClient.getNetworkInfo();
            if (networkInfo.success) {
                balance = '1000.0'; // Simulated Pravyom balance
                receiptCount = Math.floor(Math.random() * 100);
                networkConnected = true;
            }
        } catch (error) {
            console.log('BPCI query error:', error.message);
        }

        res.json({
            pravyomAccountId: wallet.accountId,
            address: wallet.address,
            balance,
            receiptCount,
            networkConnected,
            currency: 'PRAVYOM',
            protocol: 'BPCI'
        });

    } catch (error) {
        console.error('Wallet info error:', error);
        res.status(500).json({ error: 'Failed to get wallet info' });
    }
});

// Authentication middleware
function authenticateToken(req, res, next) {
    const authHeader = req.headers['authorization'];
    const token = authHeader && authHeader.split(' ')[1];

    if (!token) {
        return res.status(401).json({ error: 'Access token required' });
    }

    jwt.verify(token, process.env.JWT_SECRET || 'pravyom-secret', (err, user) => {
        if (err) {
            return res.status(403).json({ error: 'Invalid token' });
        }
        req.user = user;
        next();
    });
}

// WebSocket server for real-time updates
const wss = new WebSocket.Server({ port: 3001 });

wss.on('connection', (ws) => {
    console.log('New WebSocket connection');
    
    ws.send(JSON.stringify({
        type: 'welcome',
        message: 'Connected to Pravyom Banking SaaS',
        timestamp: new Date().toISOString()
    }));

    ws.on('message', (message) => {
        try {
            const data = JSON.parse(message);
            console.log('WebSocket message:', data);
            
            // Echo back for demo
            ws.send(JSON.stringify({
                type: 'response',
                data,
                timestamp: new Date().toISOString()
            }));
        } catch (error) {
            console.error('WebSocket message error:', error);
        }
    });
});

// Initialize Pravyom client and start server
async function startServer() {
    try {
        console.log('🔄 Connecting to Pravyom/Metanode network...');
        await pravyomClient.connect();
        
        // Set up real-time event listeners
        pravyomClient.on('newBlock', (blockData) => {
            console.log('📦 New block:', blockData.height);
            // Broadcast to WebSocket clients
            wss.clients.forEach(client => {
                if (client.readyState === WebSocket.OPEN) {
                    client.send(JSON.stringify({
                        type: 'new_block',
                        data: blockData,
                        timestamp: new Date().toISOString()
                    }));
                }
            });
        });

        pravyomClient.on('newReceipt', (receiptData) => {
            console.log('🧾 New receipt:', receiptData.tx_hash);
            // Broadcast to WebSocket clients
            wss.clients.forEach(client => {
                if (client.readyState === WebSocket.OPEN) {
                    client.send(JSON.stringify({
                        type: 'new_receipt',
                        data: receiptData,
                        timestamp: new Date().toISOString()
                    }));
                }
            });
        });

        app.listen(PORT, () => {
            console.log(`
🚀 Pravyom Banking SaaS Application Started
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🌐 Server: http://localhost:${PORT}
🔌 WebSocket: ws://localhost:3001
🔗 BPCI Server: ${pravyomClient.bpciUrl}
🏢 BPI Enterprise: ${pravyomClient.bpiUrl}
📊 Health: http://localhost:${PORT}/health
🎯 Protocol: Native Pravyom/Metanode (BPCI + Receipts)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
            `);
        });

    } catch (error) {
        console.error('❌ Failed to start Pravyom Banking SaaS:', error.message);
        console.log('🔄 Starting in offline mode...');
        
        app.listen(PORT, () => {
            console.log(`
⚠️  Pravyom Banking SaaS Started (OFFLINE MODE)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🌐 Server: http://localhost:${PORT}
🔌 WebSocket: ws://localhost:3001
❌ Blockchain: DISCONNECTED
📊 Health: http://localhost:${PORT}/health
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
            `);
        });
    }
}

startServer();

// Graceful shutdown
process.on('SIGTERM', () => {
    console.log('Shutting down Pravyom Banking SaaS...');
    process.exit(0);
});
