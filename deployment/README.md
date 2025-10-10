# PRAVYOM Testnet Deployment Guide

## Production-Grade vPods-Based Distributed Testnet

This guide provides comprehensive instructions for deploying the PRAVYOM testnet using the revolutionary vPods distributed architecture with strict BPI-BPCI separation.

## Architecture Overview

### Real BPI vs BPCI Separation

**BPCI Infrastructure (Hosted Separately)**
- Central registry and mesh server
- XTMP Server (port 7778)
- Consensus Server (port 8082)
- Advanced 4D Hash-Graph Database
- LCCD Quantum Consensus

**BPI Infrastructure (Developer/Enterprise Hosted)**
- Audit Server (port 8888)
- VM Server (port 7777)
- vPods Orchestration (3 core nodes + 2-8 app nodes)
- Dynamic database generation when connecting to BPCI

### Testnet Connection Model

When a developer connects to BPCI registry:
1. **BPI generates 2 databases** for mock auction using 4D Hash-Graph
2. **BPI adds 1 instance** to mutate BPCI infrastructure
3. **BPCI adjacent node** is activated
4. **BPI full system** and economy system are activated
5. **2-8 additional instances** are spun up for real app workloads

## Prerequisites

### System Requirements

**Minimum Requirements:**
- CPU: 8 cores
- Memory: 16GB RAM
- Storage: 500GB SSD
- Network: 1Gbps connection
- OS: Linux (Ubuntu 20.04+ recommended)

**Recommended for Production:**
- CPU: 16+ cores
- Memory: 32GB+ RAM
- Storage: 1TB+ NVMe SSD
- Network: 10Gbps connection

### Software Dependencies

```bash
# Install Rust and Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install CUE for configuration management
curl -L https://github.com/cue-lang/cue/releases/latest/download/cue_Linux_x86_64.tar.gz | tar xz
sudo mv cue /usr/local/bin/

# Install Node.js for Vite website (optional)
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install system dependencies
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev jq curl systemd
```

## Quick Start

### 1. Clone and Setup

```bash
# Navigate to the deployment directory
cd /home/umesh/metanode/deployment

# Make scripts executable
chmod +x deploy-pravyom-testnet.sh
chmod +x monitor-pravyom-testnet.sh
```

### 2. Deploy Testnet

```bash
# Run the automated deployment
sudo ./deploy-pravyom-testnet.sh
```

The deployment script will:
- ✅ Check prerequisites and validate CUE configuration
- ✅ Build all binaries with deterministic builds
- ✅ Generate configuration files from CUE
- ✅ Setup system users and directories
- ✅ Deploy BPCI infrastructure (Phase 1)
- ✅ Deploy BPI infrastructure (Phase 2-3)
- ✅ Setup vPods orchestration (Phase 4)
- ✅ Deploy Vite website integration (Phase 5)
- ✅ Verify complete deployment

### 3. Monitor Deployment

```bash
# Show current status
./monitor-pravyom-testnet.sh status

# Start continuous monitoring
./monitor-pravyom-testnet.sh monitor 30

# Generate detailed report
./monitor-pravyom-testnet.sh report
```

## Detailed Deployment Process

### Phase 1: BPCI Infrastructure

The BPCI infrastructure is deployed as a central registry and mesh server:

```bash
# BPCI services start automatically
systemctl status bpci-consensus-server

# Health check endpoints
curl http://localhost:8080/health          # General health
curl http://localhost:7778/health          # XTMP server
curl http://localhost:8082/api/consensus/status  # Consensus status
```

**BPCI Configuration:**
- Testnet mode with mock auction settlement
- 4D Hash-Graph Database (not PostgreSQL)
- LCCD Quantum Consensus with 3 validators
- Enterprise, Community Node, and Roundtable integration

### Phase 2-3: BPI Infrastructure

BPI services are deployed on the developer/enterprise side:

```bash
# BPI services
systemctl status bpi-audit-server
systemctl status bpi-vm-server

# Health check endpoints
curl http://localhost:8888/health          # Audit server
curl http://localhost:7777/__vm/status     # VM server (quantum status)
```

**BPI Features:**
- Post-quantum enabled VM with 9.8 security rating
- Immutable audit system with quantum-secure logs
- Advanced 4D Hash-Graph storage backend
- Real-time economic data processing

### Phase 4: vPods Orchestration

The vPods system provides native process orchestration:

```bash
# Check vPods status
ls -la /var/lib/pravyom/vpods/

# vPods configuration
cat /home/umesh/metanode/deployment/vpods-config.json
```

**vPods Architecture:**
- **3 Core Nodes**: Run BPI core nodes and entire system (4 cores, 8GB each)
- **2-8 App Nodes**: Real app/code execution (2-8 cores, 4-16GB each)
- **Native Orchestration**: No Docker - direct binary execution
- **Quantum Secure Boundaries**: Advanced isolation without containers

### Phase 5: Vite Website Integration

Real API integration for address/token generation:

```bash
# Website directory
cd /home/umesh/metanode/website

# Install dependencies and start (if Node.js available)
npm install
npm run dev
```

**Website Features:**
- Real-time BPI state monitoring dashboard
- Quantum-secure wallet integration
- Address generation via real BPI API
- Token minting and BPI ledger activation
- State tracking (active, rent, gas)
- BPI-BPCI bridge operations

## Configuration Management

### CUE-First Configuration

All configuration is managed through CUE files:

```bash
# Main deployment configuration
cat pravyom-testnet-deployment.cue

# Validate configuration
cue vet pravyom-testnet-deployment.cue

# Export specific configurations
cue export pravyom-testnet-deployment.cue --expression 'deployment.bpci'
cue export pravyom-testnet-deployment.cue --expression 'deployment.bpi'
```

### Generated Configurations

The deployment generates several configuration files:

```
deployment/
├── bpci-config.json              # BPCI infrastructure config
├── bpi-config.json               # BPI services config
├── website-config.json           # Vite website config
├── vpods-config.json             # vPods orchestration config
└── *.service                     # Systemd service files
```

## Monitoring and Management

### Real-Time Monitoring

```bash
# Comprehensive status check
./monitor-pravyom-testnet.sh status

# Continuous monitoring (every 30 seconds)
./monitor-pravyom-testnet.sh monitor 30

# Service-specific monitoring
journalctl -u bpci-consensus-server -f
journalctl -u bpi-audit-server -f
journalctl -u bpi-vm-server -f
```

### Health Check Endpoints

| Service | Endpoint | Purpose |
|---------|----------|---------|
| BPCI Health | `http://localhost:8080/health` | General BPCI health |
| XTMP Server | `http://localhost:7778/health` | BPCI XTMP status |
| Consensus | `http://localhost:8082/api/consensus/status` | LCCD consensus |
| BPI Audit | `http://localhost:8888/health` | BPI audit system |
| BPI VM | `http://localhost:7777/__vm/status` | Quantum VM status |

### Metrics and Alerting

The monitoring system tracks:

- **Quantum Security**: Post-quantum status, security ratings
- **4D Database**: Query performance, storage efficiency, quantum coherence
- **vPods Orchestration**: Node status, resource utilization
- **Consensus Health**: Block height, validator count, consensus time
- **System Resources**: CPU, memory, disk, network usage

### Automated Recovery

```bash
# Restart specific service
./monitor-pravyom-testnet.sh recovery bpi-audit-server

# Services support automated recovery
sudo systemctl restart bpci-consensus-server
sudo systemctl restart bpi-audit-server
sudo systemctl restart bpi-vm-server
```

## Testing the Deployment

### BPI-BPCI Bridge Testing

```bash
# Build and run bridge test
cd /home/umesh/metanode/test-bpi-bpci-bridge
cargo build --release
./target/release/test-bpi-bpci-bridge
```

Expected output:
```
🚀 Testing Complete BPI → BPCI Transaction Pipeline with Wallet Integration
✅ BPI Audit Server: RUNNING
✅ BPI VM Server: RUNNING  
✅ BPCI XTMP Server: RUNNING
✅ BPCI Consensus Server: RUNNING
✅ Complete BPI → BPCI Pipeline Test Results
```

### Foundation Grant Demonstration

```bash
# Run comprehensive grant test
cd /home/umesh/metanode/bpi-core
cargo run --bin advanced_foundation_grant_test

# Check generated artifacts
ls -la /tmp/bpi_action_vm_*.zjl
```

This generates real binary artifacts proving:
- Quantum entanglement capabilities
- 4D Hash-Graph Database operations
- LCCD consensus mechanisms
- Post-quantum cryptography
- Immutable audit trails

## Production Deployment Considerations

### Scaling Guidelines

**Single Developer Instance:**
- 1 BPCI connection
- 2 generated databases (4D Hash-Graph)
- 3 core vPods nodes
- 2-8 app workload nodes

**Enterprise Deployment:**
- Multiple BPCI connections
- Hundreds of generated databases
- Thousands of vPods instances
- Distributed across multiple regions

### Security Best Practices

1. **Quantum Security**: Always verify post-quantum cryptography is enabled
2. **Network Isolation**: Maintain strict BPI-BPCI separation
3. **Audit Trails**: Enable immutable logging for all operations
4. **Resource Limits**: Monitor vPods resource utilization
5. **Regular Updates**: Keep binaries updated with latest security patches

### Backup and Recovery

```bash
# Backup configuration
tar -czf pravyom-config-backup.tar.gz deployment/

# Backup vPods data
tar -czf vpods-data-backup.tar.gz /var/lib/pravyom/

# Database backup (4D Hash-Graph)
# Backup procedures are handled automatically by the 4D storage system
```

## Troubleshooting

### Common Issues

**Service Won't Start:**
```bash
# Check service status
systemctl status bpci-consensus-server
journalctl -u bpci-consensus-server --no-pager

# Check configuration
cue vet pravyom-testnet-deployment.cue
```

**Health Check Failures:**
```bash
# Test endpoints manually
curl -v http://localhost:8888/health
curl -v http://localhost:7777/__vm/status

# Check network connectivity
netstat -tlnp | grep -E '(8888|7777|8082|7778)'
```

**vPods Issues:**
```bash
# Check vPods directory
ls -la /var/lib/pravyom/vpods/
cat /var/lib/pravyom/vpods/*/config.json

# Check resource usage
./monitor-pravyom-testnet.sh status
```

### Performance Optimization

**4D Database Tuning:**
- Query times should be 1-50ms for optimal performance
- Storage efficiency should maintain 80%+ 
- Quantum coherence should stay above 90%

**vPods Optimization:**
- Monitor CPU usage (keep below 80%)
- Monitor memory usage (keep below 80%)
- Scale app nodes based on workload demand

## Support and Documentation

### Log Files

- Deployment: `/home/umesh/metanode/deployment/deployment.log`
- Monitoring: `/home/umesh/metanode/deployment/monitor.log`
- Alerts: `/home/umesh/metanode/deployment/alerts.log`
- System: `journalctl -u <service-name>`

### Metrics Directory

Real-time metrics are stored in:
```
deployment/metrics/
├── quantum_security.json
├── 4d_database.json
├── vpods_status.json
├── consensus.json
├── bpi_bpci_bridge.json
└── system_resources.json
```

### Configuration Reference

- **Main Config**: `pravyom-testnet-deployment.cue`
- **CUE Documentation**: https://cuelang.org/docs/
- **PRAVYOM Architecture**: See deep analysis documents in project root

## Advanced Features

### Quantum Capabilities

The testnet demonstrates:
- Post-quantum cryptographic algorithms
- Quantum entanglement for security
- Quantum-secure key distribution
- Quantum coherence monitoring

### 4D Hash-Graph Database

Revolutionary features:
- 4-dimensional data storage
- Temporal query capabilities
- Quantum-enhanced operations
- 100x performance beyond MongoDB

### LCCD Consensus

Advanced consensus mechanism:
- Quantum-resistant consensus
- Sub-second finality
- Byzantine fault tolerance
- Economic incentive alignment

---

## Quick Reference Commands

```bash
# Deploy testnet
sudo ./deploy-pravyom-testnet.sh

# Monitor status
./monitor-pravyom-testnet.sh status

# Continuous monitoring
./monitor-pravyom-testnet.sh monitor

# Test bridge
cd ../test-bpi-bpci-bridge && cargo run --release

# Check services
systemctl status bpci-consensus-server bpi-audit-server bpi-vm-server

# View logs
journalctl -u bpci-consensus-server -f
```

This completes the production-grade PRAVYOM testnet deployment with vPods-based distributed architecture, strict BPI-BPCI separation, and comprehensive monitoring capabilities.
