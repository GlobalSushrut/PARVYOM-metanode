# 🚀 Instant Setup - Deploy Your First App in 30 Minutes

**Welcome to BPCI Enterprise!** This guide gets you from zero to deploying your first auditable Web3 application in just 30 minutes.

---

## 🎯 **What You'll Accomplish**

By the end of this guide, you will have:
- ✅ Installed the complete BPCI Enterprise stack
- ✅ Deployed your first app in DockLock container
- ✅ Connected to ENC Cluster for audit trails
- ✅ Integrated with BPI Ledger for blockchain records
- ✅ Connected to BPCI Enterprise for governance
- ✅ Seen your app become **auditable and transparent**

---

## ⚡ **One-Command Installation**

### **Prerequisites (2 minutes)**
```bash
# Ensure you have these installed:
curl --version    # curl for downloads
git --version     # git for repositories
docker --version  # docker for containers
```

### **Install BPCI Enterprise Stack (5 minutes)**
```bash
# Clone the complete ecosystem
git clone https://github.com/your-org/metanode.git
cd metanode

# One-command setup (installs everything)
./scripts/instant-setup.sh

# This installs:
# - BPI Core (personal blockchain)
# - BPCI Enterprise (governance server)
# - DockLock (secure containers)
# - ENC Cluster (audit system)
# - HTTP Cage (secure web hosting)
```

---

## 🏗️ **Deploy Your First App (15 minutes)**

### **Step 1: Create a Simple Web App (3 minutes)**
```bash
# Create your first app
mkdir my-first-bpci-app
cd my-first-bpci-app

# Create a simple web application
cat > app.py << 'EOF'
from flask import Flask, jsonify
import datetime

app = Flask(__name__)

@app.route('/')
def hello():
    return jsonify({
        "message": "Hello from BPCI Enterprise!",
        "timestamp": datetime.datetime.now().isoformat(),
        "status": "running_in_docklock",
        "auditable": True
    })

@app.route('/health')
def health():
    return jsonify({"status": "healthy", "auditable": True})

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8080)
EOF

# Create requirements
cat > requirements.txt << 'EOF'
Flask==2.3.3
EOF

# Create Dockerfile
cat > Dockerfile << 'EOF'
FROM python:3.11-slim
WORKDIR /app
COPY requirements.txt .
RUN pip install -r requirements.txt
COPY app.py .
EXPOSE 8080
CMD ["python", "app.py"]
EOF
```

### **Step 2: Deploy to DockLock (5 minutes)**
```bash
# Build your app for DockLock
docker build -t my-first-bpci-app .

# Deploy to DockLock (secure container with audit trails)
bpci docklock deploy \
  --image my-first-bpci-app \
  --name my-app \
  --port 8080 \
  --audit-enabled \
  --enc-cluster-connect

# Your app is now running in a secure, auditable container!
```

### **Step 3: Connect to ENC Cluster (3 minutes)**
```bash
# ENC Cluster automatically captures audit trails
bpci enc-cluster status

# View your app's audit trail
bpci enc-cluster logs --app my-app

# You'll see:
# - Container startup events
# - HTTP requests and responses
# - System calls and file access
# - All cryptographically signed
```

### **Step 4: Connect to BPI Ledger (2 minutes)**
```bash
# Your app events are automatically recorded on BPI blockchain
bpci bpi-ledger status

# View blockchain records for your app
bpci bpi-ledger query --app my-app

# You'll see:
# - Deployment transaction
# - Runtime events
# - Audit trail hashes
# - Immutable blockchain records
```

### **Step 5: Connect to BPCI Enterprise (2 minutes)**
```bash
# Connect to BPCI governance and coordination
bpci enterprise connect

# Register your app with enterprise governance
bpci enterprise register-app \
  --name my-first-bpci-app \
  --type web-service \
  --governance-level community

# Your app is now part of the BPCI ecosystem!
```

---

## 🎉 **Test Your Deployment (5 minutes)**

### **Access Your App**
```bash
# Get your app URL
bpci docklock url my-app
# Output: https://my-app.bpci.local:8080

# Test your app
curl https://my-app.bpci.local:8080/
# {
#   "message": "Hello from BPCI Enterprise!",
#   "timestamp": "2024-01-15T10:30:00",
#   "status": "running_in_docklock",
#   "auditable": true
# }
```

### **View Complete Audit Trail**
```bash
# See the complete audit trail for your request
bpci audit-trail --app my-app --request-id latest

# You'll see:
# 1. HTTP request received (HTTP Cage)
# 2. Container execution (DockLock)
# 3. Audit event created (ENC Cluster)
# 4. Blockchain record (BPI Ledger)
# 5. Governance notification (BPCI Enterprise)
```

### **Dashboard Access**
```bash
# Open the BPCI dashboard
bpci dashboard open

# You'll see:
# - Your deployed app
# - Real-time metrics
# - Audit trails
# - Blockchain records
# - Governance status
```

---

## 🌟 **What Just Happened?**

Congratulations! You've just:

### **🔒 Made Your App Auditable**
- Every HTTP request is logged and cryptographically signed
- All container operations are recorded with syscall-level detail
- Complete audit trail is immutably stored on blockchain

### **🏗️ Deployed to Secure Infrastructure**
- **DockLock**: Your app runs in a deterministic, secure container
- **ENC Cluster**: All operations are canonically encoded and notarized
- **HTTP Cage**: Web requests are handled with military-grade security

### **⛓️ Connected to Blockchain**
- **BPI Ledger**: Your personal blockchain records all app events
- **BPCI Enterprise**: Connected to governance and coordination layer
- **Immutable Records**: All operations are permanently recorded

### **🌐 Joined the Ecosystem**
- Your app is now part of the Universal Web3 Infrastructure
- Transparent, auditable, and governable
- Ready for enterprise-grade operations

---

## 🚀 **Next Steps**

Now that you have your first app running, explore:

1. **[Understanding the Architecture](02-understanding-bpi-architecture.md)** - Learn how it all works
2. **[Advanced Deployment](15-docklock-deployment.md)** - Deploy more complex applications
3. **[Smart Contracts](19-cue-sruti-introduction.md)** - Add smart contract orchestration
4. **[Community](06-community-support.md)** - Join the BPCI community

---

## 🆘 **Need Help?**

- **Quick Issues**: Check [troubleshooting section](#troubleshooting) below
- **Community Support**: [06-community-support.md](06-community-support.md)
- **Technical Docs**: [Complete documentation index](00-enhanced-index.md)

---

## 🔧 **Troubleshooting**

### **Installation Issues**
```bash
# Reset and reinstall
./scripts/cleanup.sh
./scripts/instant-setup.sh --force
```

### **Container Issues**
```bash
# Check DockLock status
bpci docklock status
bpci docklock logs my-app
```

### **Network Issues**
```bash
# Check all services
bpci system status
bpci system health-check
```

---

**🎉 Congratulations! You're now part of the Universal Web3 Infrastructure Platform!**

*Continue your journey with [Understanding BPI Architecture](02-understanding-bpi-architecture.md)*
