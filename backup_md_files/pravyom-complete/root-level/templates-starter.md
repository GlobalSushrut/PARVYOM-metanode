# Metanode / BPI Mesh — Starter Templates
Three core templates for `bpi init --template <name>` with bpicompose.yml and service code.

---

## 1. Node API Template (`node-api`)

### bpicompose.yml
```yaml
version: "1.0"
name: "node-api-demo"
template: "node-api"

services:
  api:
    build: ./services/api
    ports:
      - "8080:8080"
    environment:
      - NODE_ENV=development
      - PORT=8080
    agreements:
      - basic

agreements:
  basic:
    file: ./agreements/basic.yaml
    
network:
  validators: 3
  da: false
  anchors: false
  receipts: false

devnet:
  auto_start: true
  dashboard: true
```

### services/api/package.json
```json
{
  "name": "bpi-node-api",
  "version": "1.0.0",
  "main": "index.js",
  "scripts": {
    "start": "node index.js",
    "dev": "nodemon index.js"
  },
  "dependencies": {
    "express": "^4.18.2",
    "cors": "^2.8.5",
    "helmet": "^7.0.0",
    "@bpi/client-sdk": "^0.1.0"
  },
  "devDependencies": {
    "nodemon": "^3.0.1"
  }
}
```

### services/api/index.js
```javascript
const express = require('express');
const cors = require('cors');
const helmet = require('helmet');
const { BPIClient } = require('@bpi/client-sdk');

const app = express();
const port = process.env.PORT || 8080;

// Middleware
app.use(helmet());
app.use(cors());
app.use(express.json({ limit: '1mb' }));

// BPI client setup
const bpi = new BPIClient({
  endpoint: process.env.BPI_ENDPOINT || 'mainnet://localhost:8547',
  deterministic: true
});

// Health check
app.get('/health', (req, res) => {
  res.json({ 
    status: 'healthy', 
    timestamp: new Date().toISOString(),
    service: 'node-api-demo'
  });
});

// Registry endpoint (BPI-secured)
app.post('/registry/submit', async (req, res) => {
  try {
    const { amount, metadata } = req.body;
    
    // Validate input
    if (!amount || typeof amount !== 'number') {
      return res.status(400).json({ error: 'Invalid amount' });
    }
    
    if (amount > 10000) {
      return res.status(400).json({ error: 'Amount exceeds limit' });
    }
    
    // Process request (this will be captured in receipts)
    const result = {
      id: `req_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      amount,
      processed_at: new Date().toISOString(),
      status: 'accepted',
      metadata: metadata || {}
    };
    
    // Log for DockLock witness
    console.log('Processing request:', JSON.stringify(result));
    
    res.json(result);
  } catch (error) {
    console.error('Request processing error:', error);
    res.status(500).json({ error: 'Internal server error' });
  }
});

// Query endpoint
app.get('/registry/query', (req, res) => {
  const { limit = 10 } = req.query;
  
  // Mock data for demo
  const results = Array.from({ length: Math.min(limit, 100) }, (_, i) => ({
    id: `req_${Date.now() - i * 1000}_${Math.random().toString(36).substr(2, 9)}`,
    amount: Math.floor(Math.random() * 5000),
    processed_at: new Date(Date.now() - i * 1000).toISOString(),
    status: 'accepted'
  }));
  
  res.json({ results, total: results.length });
});

app.listen(port, () => {
  console.log(`🚀 Node API server running on port ${port}`);
  console.log(`📊 Health: http://localhost:${port}/health`);
  console.log(`📝 Submit: POST http://localhost:${port}/registry/submit`);
  console.log(`🔍 Query: GET http://localhost:${port}/registry/query`);
});
```

### services/api/Dockerfile
```dockerfile
FROM node:20-alpine

WORKDIR /app

COPY package*.json ./
RUN npm ci --only=production

COPY . .

EXPOSE 8080

USER node

CMD ["npm", "start"]
```

### agreements/basic.yaml
```yaml
version: "1.0"
name: "basic-policy"
description: "Basic validation and limits"

pre_conditions:
  - name: "request_size_limit"
    check: "input.size <= 1048576"  # 1MB
    message: "Request too large"
    
  - name: "required_fields"
    check: "input.json.amount != null"
    message: "Amount field required"
    
  - name: "amount_range"
    check: "input.json.amount >= 0 && input.json.amount <= 10000"
    message: "Amount must be between 0 and 10000"

post_conditions:
  - name: "response_structure"
    check: "output.json.id != null && output.json.status != null"
    message: "Response missing required fields"
    
  - name: "no_sensitive_data"
    redact: ["input.json.ssn", "input.json.password"]
    
obligations:
  - type: "audit_log"
    condition: "input.json.amount > 1000"
    data: ["input.json.amount", "output.json.id"]
```

---

## 2. React SPA Template (`react-spa`)

### bpicompose.yml
```yaml
version: "1.0"
name: "react-spa-demo"
template: "react-spa"

services:
  frontend:
    build: ./services/frontend
    ports:
      - "3000:3000"
    environment:
      - REACT_APP_API_URL=http://localhost:8080
    
  api:
    build: ./services/api
    ports:
      - "8080:8080"
    agreements:
      - frontend-policy

agreements:
  frontend-policy:
    file: ./agreements/frontend.yaml
    
network:
  validators: 3
  da: false
  anchors: false
  receipts: true  # Enable for SPA demo
```

### services/frontend/package.json
```json
{
  "name": "bpi-react-spa",
  "version": "1.0.0",
  "private": true,
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "react-scripts": "5.0.1",
    "@bpi/client-sdk": "^0.1.0",
    "axios": "^1.4.0"
  },
  "scripts": {
    "start": "react-scripts start",
    "build": "react-scripts build",
    "test": "react-scripts test",
    "eject": "react-scripts eject"
  },
  "browserslist": {
    "production": [
      ">0.2%",
      "not dead",
      "not op_mini all"
    ],
    "development": [
      "last 1 chrome version",
      "last 1 firefox version",
      "last 1 safari version"
    ]
  }
}
```

### services/frontend/src/App.js
```jsx
import React, { useState, useEffect } from 'react';
import './App.css';

function App() {
  const [amount, setAmount] = useState('');
  const [metadata, setMetadata] = useState('');
  const [result, setResult] = useState(null);
  const [receipts, setReceipts] = useState([]);
  const [loading, setLoading] = useState(false);

  const submitTransaction = async (e) => {
    e.preventDefault();
    setLoading(true);
    
    try {
      const response = await fetch('mainnet://registry.submit', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          amount: parseFloat(amount),
          metadata: metadata ? JSON.parse(metadata) : {}
        })
      });
      
      const data = await response.json();
      setResult(data);
      
      // Fetch receipt after a short delay
      setTimeout(async () => {
        try {
          const receiptResponse = await fetch(`/bpi/receipts/${data.id}`);
          const receipt = await receiptResponse.json();
          setReceipts(prev => [receipt, ...prev.slice(0, 4)]);
        } catch (err) {
          console.log('Receipt not yet available');
        }
      }, 2000);
      
    } catch (error) {
      setResult({ error: error.message });
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>🌐 BPI Mesh Demo</h1>
        <p>Web3 security for Web2 apps</p>
      </header>
      
      <main className="App-main">
        <div className="demo-section">
          <h2>Submit Transaction</h2>
          <form onSubmit={submitTransaction}>
            <div className="form-group">
              <label>Amount (0-10000):</label>
              <input
                type="number"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                min="0"
                max="10000"
                required
              />
            </div>
            
            <div className="form-group">
              <label>Metadata (JSON):</label>
              <textarea
                value={metadata}
                onChange={(e) => setMetadata(e.target.value)}
                placeholder='{"category": "demo"}'
                rows="3"
              />
            </div>
            
            <button type="submit" disabled={loading}>
              {loading ? '⏳ Processing...' : '🚀 Submit'}
            </button>
          </form>
        </div>
        
        {result && (
          <div className="result-section">
            <h3>Result</h3>
            <pre>{JSON.stringify(result, null, 2)}</pre>
          </div>
        )}
        
        {receipts.length > 0 && (
          <div className="receipts-section">
            <h3>Recent Receipts</h3>
            {receipts.map((receipt, i) => (
              <div key={i} className="receipt-card">
                <div className="receipt-header">
                  <span className="receipt-id">{receipt.id}</span>
                  <span className="receipt-status">✅ Verified</span>
                </div>
                <div className="receipt-details">
                  <p>Block: {receipt.block_height}</p>
                  <p>Finality: {receipt.finality_ms}ms</p>
                </div>
              </div>
            ))}
          </div>
        )}
      </main>
    </div>
  );
}

export default App;
```

### services/frontend/src/App.css
```css
.App {
  text-align: center;
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

.App-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 40px 20px;
  border-radius: 10px;
  margin-bottom: 30px;
}

.App-header h1 {
  margin: 0 0 10px 0;
  font-size: 2.5em;
}

.demo-section {
  background: #f8f9fa;
  padding: 30px;
  border-radius: 10px;
  margin-bottom: 20px;
}

.form-group {
  margin-bottom: 20px;
  text-align: left;
}

.form-group label {
  display: block;
  margin-bottom: 5px;
  font-weight: bold;
}

.form-group input,
.form-group textarea {
  width: 100%;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 5px;
  font-size: 16px;
}

button {
  background: #007bff;
  color: white;
  border: none;
  padding: 12px 24px;
  border-radius: 5px;
  font-size: 16px;
  cursor: pointer;
}

button:disabled {
  background: #6c757d;
  cursor: not-allowed;
}

.result-section,
.receipts-section {
  background: #e9ecef;
  padding: 20px;
  border-radius: 10px;
  margin-bottom: 20px;
}

.result-section pre {
  background: white;
  padding: 15px;
  border-radius: 5px;
  text-align: left;
  overflow-x: auto;
}

.receipt-card {
  background: white;
  padding: 15px;
  border-radius: 5px;
  margin-bottom: 10px;
  text-align: left;
}

.receipt-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
}

.receipt-id {
  font-family: monospace;
  font-size: 14px;
}

.receipt-status {
  color: #28a745;
  font-weight: bold;
}

.receipt-details p {
  margin: 5px 0;
  color: #666;
}
```

### agreements/frontend.yaml
```yaml
version: "1.0"
name: "frontend-policy"
description: "Policy for frontend API calls"

pre_conditions:
  - name: "json_structure"
    check: "input.json != null"
    message: "Request must be JSON"
    
  - name: "amount_validation"
    check: "input.json.amount >= 0 && input.json.amount <= 10000"
    message: "Amount must be between 0 and 10000"

post_conditions:
  - name: "success_response"
    check: "output.status == 200 || output.status == 400"
    message: "Unexpected response status"

obligations:
  - type: "receipt_required"
    condition: "true"
    metadata: ["request_id", "amount", "timestamp"]
```

---

## 3. AI Inference Template (`ai-inference`)

### bpicompose.yml
```yaml
version: "1.0"
name: "ai-inference-demo"
template: "ai-inference"

services:
  inference:
    build: ./services/inference
    ports:
      - "8080:8080"
    environment:
      - MODEL_NAME=gpt-3.5-turbo
      - MAX_TOKENS=1000
    agreements:
      - ai-safety
      
  frontend:
    build: ./services/frontend
    ports:
      - "3000:3000"
    environment:
      - REACT_APP_API_URL=http://localhost:8080

agreements:
  ai-safety:
    file: ./agreements/ai-safety.yaml
    
network:
  validators: 3
  da: true    # Enable DA for AI outputs
  anchors: false
  receipts: true
```

### services/inference/package.json
```json
{
  "name": "bpi-ai-inference",
  "version": "1.0.0",
  "main": "index.js",
  "scripts": {
    "start": "node index.js",
    "dev": "nodemon index.js"
  },
  "dependencies": {
    "express": "^4.18.2",
    "cors": "^2.8.5",
    "helmet": "^7.0.0",
    "openai": "^4.0.0",
    "@bpi/client-sdk": "^0.1.0"
  }
}
```

### services/inference/index.js
```javascript
const express = require('express');
const cors = require('cors');
const helmet = require('helmet');
const OpenAI = require('openai');

const app = express();
const port = process.env.PORT || 8080;

// Middleware
app.use(helmet());
app.use(cors());
app.use(express.json({ limit: '10mb' }));

// Mock OpenAI for demo (replace with real API key)
const openai = new OpenAI({
  apiKey: process.env.OPENAI_API_KEY || 'demo-key'
});

// Health check
app.get('/health', (req, res) => {
  res.json({ 
    status: 'healthy',
    model: process.env.MODEL_NAME || 'gpt-3.5-turbo',
    timestamp: new Date().toISOString()
  });
});

// AI inference endpoint
app.post('/inference/chat', async (req, res) => {
  try {
    const { prompt, max_tokens = 500, temperature = 0.7 } = req.body;
    
    if (!prompt || typeof prompt !== 'string') {
      return res.status(400).json({ error: 'Prompt is required' });
    }
    
    if (prompt.length > 4000) {
      return res.status(400).json({ error: 'Prompt too long' });
    }
    
    // Mock AI response for demo
    const mockResponse = {
      id: `chat_${Date.now()}`,
      model: process.env.MODEL_NAME || 'gpt-3.5-turbo',
      prompt,
      response: generateMockResponse(prompt),
      tokens_used: Math.floor(Math.random() * 200) + 50,
      created_at: new Date().toISOString(),
      safety_score: Math.random() * 0.3 + 0.7, // 0.7-1.0
      content_flags: []
    };
    
    // Add content flags for demo
    if (prompt.toLowerCase().includes('violence')) {
      mockResponse.content_flags.push('potential_violence');
      mockResponse.safety_score = 0.3;
    }
    
    // Log for DockLock witness
    console.log('AI inference request:', JSON.stringify({
      prompt_length: prompt.length,
      tokens_used: mockResponse.tokens_used,
      safety_score: mockResponse.safety_score,
      flags: mockResponse.content_flags
    }));
    
    res.json(mockResponse);
    
  } catch (error) {
    console.error('Inference error:', error);
    res.status(500).json({ error: 'Inference failed' });
  }
});

function generateMockResponse(prompt) {
  const responses = [
    "This is a mock AI response for demonstration purposes. In a real implementation, this would connect to an actual AI model.",
    "I understand your request. This demo shows how AI inference can be made verifiable and auditable using BPI Mesh receipts.",
    "Your prompt has been processed. This response demonstrates deterministic AI inference with policy enforcement.",
    "Thank you for your query. This mock response illustrates how AI outputs can be cryptographically verified."
  ];
  
  return responses[Math.floor(Math.random() * responses.length)] + 
         ` (Responding to: "${prompt.substring(0, 50)}...")`;
}

app.listen(port, () => {
  console.log(`🤖 AI Inference server running on port ${port}`);
  console.log(`📊 Health: http://localhost:${port}/health`);
  console.log(`🧠 Chat: POST http://localhost:${port}/inference/chat`);
});
```

### agreements/ai-safety.yaml
```yaml
version: "1.0"
name: "ai-safety-policy"
description: "Safety and compliance for AI inference"

pre_conditions:
  - name: "prompt_length"
    check: "input.json.prompt.length <= 4000"
    message: "Prompt too long"
    
  - name: "token_limit"
    check: "input.json.max_tokens == null || input.json.max_tokens <= 1000"
    message: "Token limit exceeded"
    
  - name: "rate_limit"
    check: "true"  # Implement rate limiting logic
    message: "Rate limit exceeded"

post_conditions:
  - name: "safety_threshold"
    check: "output.json.safety_score >= 0.5"
    message: "Content safety threshold not met"
    action: "deny"
    
  - name: "content_flags"
    check: "output.json.content_flags.length == 0"
    message: "Content flagged for review"
    action: "flag"
    
  - name: "response_structure"
    check: "output.json.response != null && output.json.tokens_used != null"
    message: "Invalid response structure"

obligations:
  - type: "audit_log"
    condition: "output.json.safety_score < 0.8"
    data: ["input.json.prompt", "output.json.safety_score", "output.json.content_flags"]
    
  - type: "content_review"
    condition: "output.json.content_flags.length > 0"
    data: ["request_id", "flags", "prompt_hash"]

redactions:
  - field: "input.json.prompt"
    condition: "output.json.content_flags.includes('personal_info')"
    method: "hash"
```

---

## Template Generation Script

### scripts/generate-template.sh
```bash
#!/bin/bash

TEMPLATE_NAME=$1
PROJECT_NAME=$2

if [ -z "$TEMPLATE_NAME" ]; then
    echo "Usage: $0 <template-name> [project-name]"
    exit 1
fi

if [ -z "$PROJECT_NAME" ]; then
    PROJECT_NAME="my-bpi-app"
fi

case $TEMPLATE_NAME in
    "node-api")
        echo "🚀 Generating Node.js API template..."
        # Copy node-api template files
        ;;
    "react-spa")
        echo "⚛️ Generating React SPA template..."
        # Copy react-spa template files
        ;;
    "ai-inference")
        echo "🤖 Generating AI Inference template..."
        # Copy ai-inference template files
        ;;
    *)
        echo "❌ Unknown template: $TEMPLATE_NAME"
        echo "Available templates: node-api, react-spa, ai-inference"
        exit 1
        ;;
esac

echo "✅ Template '$TEMPLATE_NAME' generated as '$PROJECT_NAME'"
echo "📝 Next steps:"
echo "   cd $PROJECT_NAME"
echo "   bpi up"
echo "   bpi verify"
```

---

## Template Registry (for `bpi templates list`)

### templates.json
```json
{
  "templates": [
    {
      "name": "node-api",
      "description": "Node.js Express API with BPI integration",
      "language": "JavaScript",
      "framework": "Express",
      "features": ["REST API", "Basic agreements", "Health checks"],
      "complexity": "beginner"
    },
    {
      "name": "react-spa",
      "description": "React single-page app with BPI backend",
      "language": "JavaScript",
      "framework": "React",
      "features": ["SPA", "Receipt display", "Real-time updates"],
      "complexity": "intermediate"
    },
    {
      "name": "ai-inference",
      "description": "AI inference service with safety policies",
      "language": "JavaScript",
      "framework": "Express + React",
      "features": ["AI/ML", "Content safety", "Advanced agreements"],
      "complexity": "advanced"
    }
  ]
}
```
