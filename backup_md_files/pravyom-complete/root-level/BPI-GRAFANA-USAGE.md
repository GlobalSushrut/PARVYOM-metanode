# 🎯 BPI Grafana Monitoring - Usage Guide

## 🚀 Quick Start - "start BPI grafana"

### Simple Command
```bash
# Start BPI Grafana monitoring (simplest way)
./start-bpi-grafana.sh

# Or with custom BPCI server URL
./start-bpi-grafana.sh your-actual-server.com:8081
```

### Advanced Commands
```bash
# Using BPI core directly
./target/release/bpi-core monitor grafana --start
./target/release/bpi-core monitor grafana --start --bpci-url your-server.com:8081
./target/release/bpi-core monitor grafana --status
./target/release/bpi-core monitor grafana --stop
```

## 📊 Access Your Dashboards

Once started, access your monitoring dashboards:

- **🎯 Grafana Dashboard**: http://localhost:3000
  - Username: `admin`
  - Password: `bpi-admin-2024`
- **📈 Prometheus Metrics**: http://localhost:9090

## 🏗️ Architecture Understanding

### 🏠 BPI Core (User Installation)
- **VM Server**: localhost:7777 - **CANNOT WORK WITHOUT BPCI**
- **HTTP Cage**: localhost:8888 - Quantum security
- **Shadow Registry**: localhost:8080 - Web2 bridge
- **BPCI Client**: Built-in client for mandatory BPCI connection

### 🌐 External BPCI Server (Your Hosted Server)
- **Economic Engine**: 4-coin economy (GEN/NEX/FLX/AUR)
- **Wallet Registry**: **MANDATORY** for all BPI operations
- **Gas/Rent Collection**: **REQUIRED** for all transactions
- **Bank/Government APIs**: Enterprise compliance

## ⚠️ CRITICAL DEPENDENCY

**BPI CORE CANNOT FUNCTION WITHOUT BPCI SERVER CONNECTION**

- All consensus operations require BpciTransport
- All wallet operations require BPCI registry
- All mining operations send POE proofs to BPCI
- All transactions require BPCI gas/rent payments

## 🔍 Monitoring Targets

### 🏠 BPI Core Metrics (User Installation)
```yaml
- localhost:7777/__vm/status           # VM server metrics
- localhost:7777/bpci/connection       # CRITICAL: BPCI connection status
- localhost:7777/wallet/bpci-status    # CRITICAL: Wallet registration with BPCI
- localhost:7777/poe/bpci-submission   # CRITICAL: POE proof submission to BPCI
- localhost:7777/consensus/bpci-transport # CRITICAL: Consensus via BPCI transport
```

### 🌐 External BPCI Server Metrics (Your Server)
```yaml
- your-server.com:8081/api/economy/status    # 4-coin system (GEN/NEX/FLX/AUR)
- your-server.com:8081/api/registry/status   # Wallet registrations (MANDATORY)
- your-server.com:8081/api/bank/status       # Bank integration
- your-server.com:8081/api/government/status # Government APIs
- your-server.com:8081/api/maintenance/status # System health
```

## 📊 Dashboard Categories

1. **🏠 BPI System Health**: Node status, consensus performance, **BPCI CONNECTION STATUS**
2. **🔗 BPI-BPCI Integration**: **CRITICAL** - Connection health, wallet registration, POE submission
3. **🛡️ BPI Security Dashboard**: ZK proofs, quantum security, audit trails
4. **🌐 BPCI Economic Overview**: 4-coin economy, wallet registrations, gas/rent collection (External Server)
5. **🌐 BPCI Enterprise Dashboard**: Registry nodes, policy compliance, economic engine (External Server)
6. **📊 BPI Developer Metrics**: BPCI connection performance, registration success rates

## 🛠️ Prerequisites

- Docker and docker-compose installed
- BPI core built (`cargo build --release`)
- Access to external BPCI server (your hosted server)

## 🔧 Configuration

### Custom BPCI Server URL
Update the BPCI server URL in the monitoring configuration:

```bash
# Method 1: Use startup script with custom URL
./start-bpi-grafana.sh your-actual-server.com:8081

# Method 2: Use BPI core command directly
./target/release/bpi-core monitor grafana --start --bpci-url your-actual-server.com:8081
```

### Manual Configuration
Edit `monitoring/prometheus/prometheus.yml` and replace `your-server.com:8081` with your actual BPCI server URL.

## 🚨 Troubleshooting

### Common Issues

1. **"Monitoring directory not found"**
   - Ensure the `monitoring/` directory exists with all configuration files

2. **"Docker command not found"**
   - Install Docker and docker-compose

3. **"BPCI connection failed"**
   - Verify your BPCI server URL is correct and accessible
   - Check if BPCI server is running on your hosted server

4. **"BPI services not responding"**
   - Start BPI core services first: `./target/release/bpi-core vm-server start`
   - Ensure BPI is connected to BPCI server

### Logs and Debugging
```bash
# Check Grafana logs
docker logs bpi-grafana

# Check Prometheus logs
docker logs bpi-prometheus

# Check BPI core status
./target/release/bpi-core monitor grafana --status
```

## 🎯 Success Indicators

When everything is working correctly, you should see:

1. ✅ Grafana accessible at http://localhost:3000
2. ✅ All BPI services showing as "Up" in dashboards
3. ✅ BPCI connection status showing as "Connected"
4. ✅ Wallet registration metrics updating
5. ✅ POE submission metrics active
6. ✅ External BPCI server metrics available

## 📞 Support

If you encounter issues:
1. Check the troubleshooting section above
2. Verify BPI-BPCI connection status in the dashboard
3. Ensure your BPCI server is accessible and running
4. Remember: **BPI cannot function without BPCI connection**

---

**🎉 Enjoy monitoring your BPI ecosystem with complete visibility into the critical BPI-BPCI dependency!**
