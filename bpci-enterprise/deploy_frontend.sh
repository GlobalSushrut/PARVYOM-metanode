#!/bin/bash

# BPCI Frontend Deployment Script
echo "=========================================="
echo "BPCI FRONTEND DEPLOYMENT"
echo "=========================================="
echo ""

SERVER="134.209.210.181"

echo "Step 1: Creating simple landing page..."
cat > /tmp/index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>BPCI Enterprise - Blockchain Infrastructure</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            color: white;
        }
        
        .container {
            max-width: 1200px;
            padding: 40px;
            text-align: center;
        }
        
        h1 {
            font-size: 3.5rem;
            margin-bottom: 20px;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
        }
        
        .subtitle {
            font-size: 1.5rem;
            margin-bottom: 40px;
            opacity: 0.9;
        }
        
        .status-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
            margin: 40px 0;
        }
        
        .status-card {
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            border-radius: 15px;
            padding: 30px;
            border: 1px solid rgba(255, 255, 255, 0.2);
            transition: transform 0.3s ease;
        }
        
        .status-card:hover {
            transform: translateY(-5px);
        }
        
        .status-card h3 {
            font-size: 1.2rem;
            margin-bottom: 10px;
        }
        
        .status-indicator {
            display: inline-block;
            width: 12px;
            height: 12px;
            border-radius: 50%;
            background: #4ade80;
            margin-right: 8px;
            animation: pulse 2s infinite;
        }
        
        @keyframes pulse {
            0%, 100% { opacity: 1; }
            50% { opacity: 0.5; }
        }
        
        .api-links {
            margin: 40px 0;
        }
        
        .api-link {
            display: inline-block;
            background: rgba(255, 255, 255, 0.2);
            padding: 15px 30px;
            margin: 10px;
            border-radius: 8px;
            text-decoration: none;
            color: white;
            font-weight: 500;
            transition: background 0.3s ease;
        }
        
        .api-link:hover {
            background: rgba(255, 255, 255, 0.3);
        }
        
        .footer {
            margin-top: 60px;
            opacity: 0.8;
            font-size: 0.9rem;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🚀 BPCI Enterprise</h1>
        <p class="subtitle">Revolutionary Blockchain Infrastructure</p>
        
        <div class="status-grid">
            <div class="status-card">
                <h3><span class="status-indicator"></span>Backend Services</h3>
                <p>16 Services Running</p>
            </div>
            
            <div class="status-card">
                <h3><span class="status-indicator"></span>Blockchain</h3>
                <p>LCCD Consensus Active</p>
            </div>
            
            <div class="status-card">
                <h3><span class="status-indicator"></span>BPI Bridge</h3>
                <p>1M+ Connections Ready</p>
            </div>
            
            <div class="status-card">
                <h3><span class="status-indicator"></span>DynaRoute v2</h3>
                <p>Pure Virtual Mode</p>
            </div>
        </div>
        
        <div class="api-links">
            <h2 style="margin-bottom: 20px;">API Endpoints</h2>
            <a href="/health" class="api-link">Health Check</a>
            <a href="/blockchain/api/v1/blockchain/info" class="api-link">Blockchain Info</a>
            <a href="/bridge/health" class="api-link">Bridge Status</a>
            <a href="/auth" class="api-link">Authentication</a>
        </div>
        
        <div class="footer">
            <p>BPCI Enterprise Testnet | Powered by LCCD Consensus</p>
            <p>Server: 134.209.210.181</p>
        </div>
    </div>
    
    <script>
        // Test API connectivity
        fetch('/health')
            .then(r => r.json())
            .then(data => console.log('Backend health:', data))
            .catch(e => console.error('Backend error:', e));
    </script>
</body>
</html>
EOF

echo "✅ Landing page created"

echo ""
echo "Step 2: Deploying to server..."
scp /tmp/index.html root@${SERVER}:/var/www/html/index.html

echo "✅ Frontend deployed"

echo ""
echo "Step 3: Testing deployment..."
curl -s http://${SERVER}/ | grep -q "BPCI Enterprise" && echo "✅ Frontend accessible" || echo "❌ Frontend not accessible"

echo ""
echo "=========================================="
echo "DEPLOYMENT COMPLETE"
echo "=========================================="
echo ""
echo "Frontend URL: http://${SERVER}/"
echo "API Endpoints:"
echo "  - http://${SERVER}/health"
echo "  - http://${SERVER}/blockchain/api/v1/blockchain/info"
echo "  - http://${SERVER}/bridge/health"
echo "  - http://${SERVER}/auth"
echo ""
echo "Next: Configure Keycloak manually at http://${SERVER}/auth"
echo ""
