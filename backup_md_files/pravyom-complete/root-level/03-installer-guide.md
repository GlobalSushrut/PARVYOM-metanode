# PARVYOM Metanode Installer Guide

## Overview

The PARVYOM Metanode ecosystem provides four distinct installers designed for different deployment scenarios and user types. Each installer is optimized for specific use cases while maintaining seamless integration across the entire ecosystem.

## 🚀 Quick Start

Choose your installer based on your needs:

- **🏢 BPCI Host**: Internal infrastructure hosting
- **👥 Community**: Local governance and participation  
- **💻 BPI Dev/Enterprise**: DApp development and enterprise deployment
- **📱 IoT ZK**: IoT devices and edge computing

## Installation Types

### 1. 🏢 BPCI Host Installer

**Purpose**: Deploy BPCI server infrastructure for internal hosting and enterprise management.

**Target Users**: 
- Infrastructure administrators
- Enterprise IT teams
- Hosting providers

**Features**:
- Full BPCI server deployment
- PostgreSQL and Redis setup
- Nginx reverse proxy configuration
- SSL/TLS certificate management
- Monitoring and logging
- Firewall configuration
- Systemd service management

**System Requirements**:
- **OS**: Ubuntu 20.04+ / Debian 11+ / CentOS 8+
- **Memory**: 8GB RAM minimum, 16GB recommended
- **Storage**: 100GB minimum, 500GB recommended
- **Network**: Static IP address recommended
- **Permissions**: Root access required

**Installation Command**:
```bash
curl -sSL https://install.parvyom.org/bpci-host | sudo bash
```

**Manual Installation**:
```bash
wget https://github.com/metanode/metanode/raw/main/installer/bpci-host-installer.sh
chmod +x bpci-host-installer.sh
sudo ./bpci-host-installer.sh
```

**Configuration Options**:
- **Database**: PostgreSQL with automatic backup
- **Cache**: Redis for high-performance caching
- **Web Server**: Nginx with SSL termination
- **Monitoring**: Prometheus and Grafana integration
- **Ports**: 8081 (BPCI server), 8082 (API), 5432 (PostgreSQL), 6379 (Redis)

**Post-Installation**:
1. Access BPCI dashboard: `https://your-server:8081`
2. Configure enterprise settings
3. Set up user accounts and permissions
4. Enable monitoring and alerts

---

### 2. 👥 Community Installer

**Purpose**: Set up community nodes for local governance and BPCI registration.

**Target Users**:
- Community operators
- Local governance groups
- Decentralized organizations

**Features**:
- 11 specialized community nodes
- Automatic BPCI registration
- Wallet generation and management
- Governance coordination
- Mining and validation
- Community-specific policies

**System Requirements**:
- **OS**: Ubuntu 18.04+ / Debian 10+ / CentOS 7+
- **Memory**: 4GB RAM minimum, 8GB recommended
- **Storage**: 50GB minimum, 200GB recommended
- **Network**: Stable internet connection
- **Permissions**: Root access required

**Installation Command**:
```bash
curl -sSL https://install.parvyom.org/community | sudo bash
```

**Manual Installation**:
```bash
wget https://github.com/metanode/metanode/raw/main/installer/community-installer.sh
chmod +x community-installer.sh
sudo ./community-installer.sh
```

**Community Node Types**:
1. **Mining Node** (port 9001) - PoE mining with hashpower 1000
2. **BPCI Server Node 1** (port 9002) - Enterprise hosting
3. **BPCI Server Node 2** (port 9003) - Enterprise hosting backup
4. **Validator Node** (port 9004) - Consensus validation
5. **Notary Node** (port 9005) - Transaction notarization
6. **Logbook Node** (port 9006) - Auction record keeping
7. **Roundtable Node** (port 9007) - Governance coordination
8. **Box Block Node** (port 9008) - BPCI duplication per wallet
9. **Bank API Registry Node** (port 9009) - Banking integration
10. **Government API Registry Node** (port 9010) - Government compliance
11. **Roundtable API Node** (port 9011) - Governance API

**Registration Process**:
1. Installer prompts for BPCI server details
2. Generates cryptographic wallet automatically
3. Registers all 11 nodes with BPCI server
4. Configures governance participation
5. Sets up mining and validation

**Management Commands**:
```bash
# Check community status
sudo systemctl status community-node

# View community logs
sudo journalctl -u community-node -f

# Restart community services
sudo systemctl restart community-node

# Check node registration
curl http://localhost:9100/api/status
```

---

### 3. 💻 BPI Dev/Enterprise Installer

**Purpose**: Full BPI development environment with BPCI integration and ZK IoT support.

**Target Users**:
- DApp developers
- Enterprise developers
- Blockchain application builders
- Smart contract developers

**Features**:
- Complete BPI node deployment
- Development tools and hot reload
- BPCI server integration
- ZKLock Mobile Port support
- HTTP CAGE security layer
- DockLock container platform
- Oracle node coordination
- Project structure generation

**System Requirements**:
- **Development**: 4GB RAM, 50GB storage
- **Enterprise**: 16GB RAM, 200GB storage
- **Custom**: User-defined specifications
- **OS**: Ubuntu 20.04+ / macOS 10.15+ / Windows 10+ (WSL2)
- **Tools**: Git, Docker, Node.js (for development)

**Installation Types**:

#### Development Mode
```bash
curl -sSL https://install.parvyom.org/bpi-dev | bash
```
- Hot reload enabled
- Debug mode active
- Local development optimizations
- Ports: 8000 (BPI), 8001 (API)

#### Enterprise Mode
```bash
curl -sSL https://install.parvyom.org/bpi-enterprise | bash
```
- Production-ready configuration
- Banking and audit features
- Enhanced security
- Monitoring and compliance

#### Manual Installation
```bash
wget https://github.com/metanode/metanode/raw/main/installer/bpi-dev-enterprise-installer.sh
chmod +x bpi-dev-enterprise-installer.sh
sudo ./bpi-dev-enterprise-installer.sh
```

**Service Configuration**:

| Service | Port | Purpose |
|---------|------|---------|
| BPI Core | 8000 | Main blockchain node |
| BPI API | 8001 | REST API interface |
| ZKLock Mobile | 8300 | Mobile device integration |
| ZKLock IoT | 8301 | IoT device management |
| HTTP CAGE | 8200 | Security audit layer |
| DockLock | 8400 | Container platform |
| Oracle Nodes | 9000-9010 | Cross-chain communication |

**Project Creation**:
The installer automatically creates a project structure:
```
/opt/bpi-node/projects/your-project/
├── src/           # Source code
├── config/        # Configuration files
├── docs/          # Documentation
├── tests/         # Test files
└── README.md      # Project documentation
```

**Development Commands**:
```bash
# Check BPI status
bpi status

# Restart BPI services
bpi restart

# View logs
bpi logs

# Test connectivity
bpi test

# Access project directory
cd /opt/bpi-node/projects/your-project
```

**BPCI Integration**:
- Automatic registration with BPCI server
- Project ID assignment
- API endpoint configuration
- Wallet integration
- Economic coordination

---

### 4. 📱 IoT ZK Installer

**Purpose**: Deploy ZKLock Mobile Port for IoT devices with minimal resources and BPI integration.

**Target Users**:
- IoT device manufacturers
- Edge computing operators
- Mobile app developers
- Embedded systems engineers

**Features**:
- Ultra-lightweight ZK proof system
- Battery optimization
- Device type detection
- Automatic BPI registration
- Minimal resource usage
- Edge gateway support

**Device Types**:

#### IoT Device
- **Memory**: 256MB minimum
- **Storage**: 1GB minimum
- **Features**: Ultra-lightweight, battery optimized
- **Ports**: 8300 (ZKLock), 8301 (API)

#### Edge Gateway
- **Memory**: 1GB minimum
- **Storage**: 8GB minimum
- **Features**: Multi-device support, gateway functionality
- **Ports**: 8300 (ZKLock), 8301 (API), 8302 (Gateway)

#### Mobile Device
- **Memory**: 2GB minimum
- **Storage**: 4GB minimum
- **Features**: Battery optimization, background sync
- **Ports**: 8300 (ZKLock), 8301 (API)

#### Raspberry Pi
- **Memory**: 1GB minimum
- **Storage**: 8GB minimum
- **Features**: GPIO support, sensor integration
- **Ports**: 8300 (ZKLock), 8301 (API), 8303 (GPIO)

**Installation Commands**:

#### Quick Install
```bash
curl -sSL https://install.parvyom.org/iot-zk | bash
```

#### Manual Install
```bash
wget https://github.com/metanode/metanode/raw/main/installer/iot-zk-installer.sh
chmod +x iot-zk-installer.sh
./iot-zk-installer.sh  # Can run as user for IoT devices
```

**Configuration Process**:
1. **Device Type Selection**: Choose from IoT/Edge/Mobile/Raspberry Pi
2. **Device Details**: Name, location, capabilities
3. **BPI Integration**: Connect to BPI node and BPCI server
4. **Registration**: Automatic device registration
5. **Optimization**: Battery and network optimization

**ZK Proof Types**:
- **Device Authentication**: Cryptographic device identity
- **Capability Proofs**: Device capability verification
- **Participation Proofs**: Network participation validation
- **Privacy Proofs**: Data privacy preservation

**Management Commands**:
```bash
# Device status (if installed as root)
zklock status

# Device status (user installation)
~/.zklock-iot/bin/zklock-cli status

# Test device connectivity
zklock test

# Generate test proof
zklock proof

# View device logs
zklock logs
```

**Device Integration**:
- Automatic BPI ecosystem connection
- Token economics participation
- Proof generation and verification
- Network optimization
- Security compliance

---

## 🔧 Advanced Configuration

### Port Management

The PARVYOM Metanode ecosystem uses intelligent port management to prevent conflicts:

#### Default Port Allocation
```
BPI Core Services:      8000-8001
HTTP CAGE:              8200
ZKLock Mobile Port:     8300-8303
DockLock Platform:      8400
ENC Cluster:            8500
BPI Specialized Nodes:  8600-8607
BPCI Enterprise:        8081-8082
Community Nodes:        9001-9011
Oracle Nodes:           9000-9010
Monitoring:             9300-9302
```

#### Custom Port Configuration
Each installer supports custom port configuration:
```bash
# During installation, specify custom ports
./installer.sh --bpi-port 8010 --api-port 8011
```

### Firewall Configuration

All installers automatically configure firewall rules:
```bash
# Allow required ports
ufw allow 8000:8010/tcp  # BPI services
ufw allow 8300:8303/tcp  # ZKLock services
ufw allow 9000:9011/tcp  # Community/Oracle nodes
```

### SSL/TLS Configuration

Enterprise installers include automatic SSL setup:
```bash
# Automatic certificate generation
certbot --nginx -d your-domain.com
```

---

## 🔍 Troubleshooting

### Common Issues

#### Port Conflicts
```bash
# Check port usage
netstat -tlnp | grep :8000

# Kill conflicting processes
sudo kill $(lsof -t -i:8000)
```

#### Service Failures
```bash
# Check service status
systemctl status bpi-node

# View detailed logs
journalctl -u bpi-node -f --no-pager

# Restart services
systemctl restart bpi-node
```

#### Network Connectivity
```bash
# Test BPI connectivity
curl http://localhost:8000/api/status

# Test BPCI connection
curl https://bpci-server:8081/api/health
```

#### Permission Issues
```bash
# Fix file permissions
sudo chown -R bpi:bpi /opt/bpi-node
sudo chmod -R 755 /opt/bpi-node
```

### Log Locations

| Service | Log Location |
|---------|-------------|
| BPI Core | `/var/log/bpi-node/bpi.log` |
| BPCI Server | `/var/log/bpci/server.log` |
| Community Node | `/var/log/community-node/node.log` |
| ZKLock Device | `/var/log/zklock-iot/device.log` |

### Configuration Files

| Service | Config Location |
|---------|----------------|
| BPI Core | `/etc/bpi-node/bpi-node.toml` |
| BPCI Server | `/etc/bpci/server.toml` |
| Community Node | `/etc/community-node/node.toml` |
| ZKLock Device | `/etc/zklock-iot/device.toml` |

---

## 🔄 Updates and Maintenance

### Automatic Updates
```bash
# Enable automatic updates
sudo systemctl enable bpi-auto-update

# Check for updates
sudo /opt/bpi-node/bin/update-check
```

### Manual Updates
```bash
# Download latest installer
wget https://github.com/metanode/metanode/raw/main/installer/update.sh

# Run update
sudo ./update.sh
```

### Backup and Recovery
```bash
# Create backup
sudo /opt/bpi-node/bin/backup-create

# Restore from backup
sudo /opt/bpi-node/bin/backup-restore backup-file.tar.gz
```

---

## 🌐 Network Integration

### BPCI Server Connection
All installers can connect to:
- **Production**: `https://bpci.metanode.org`
- **Testnet**: `https://testnet.bpci.metanode.org`
- **Local**: Custom BPCI server URL

### Cross-System Communication
- **BPI ↔ BPCI**: Automatic registration and coordination
- **Community ↔ BPCI**: Governance and mining integration
- **IoT ↔ BPI**: Device registration and proof submission
- **Enterprise ↔ All**: Full ecosystem integration

---

## 📊 Monitoring and Analytics

### Built-in Monitoring
- **Prometheus**: Metrics collection
- **Grafana**: Visualization dashboards
- **Health Checks**: Automatic service monitoring
- **Alerts**: Email and webhook notifications

### Custom Metrics
```bash
# View system metrics
curl http://localhost:9301/metrics

# Access Grafana dashboard
http://localhost:9302
```

---

## 🔐 Security Considerations

### Cryptographic Security
- **Ed25519**: Digital signatures
- **Blake3**: Hashing algorithms
- **AES-256**: Encryption
- **TLS 1.3**: Network security

### Access Control
- **Wallet-based**: Cryptographic authentication
- **Role-based**: Permission management
- **API Keys**: Service authentication
- **Firewall**: Network protection

### Compliance
- **Banking**: Regulatory compliance features
- **Government**: Audit trail requirements
- **Enterprise**: Security standards
- **Privacy**: GDPR/CCPA compliance

---

## 📚 Additional Resources

### Documentation
- [BPI Architecture Guide](./02-understanding-bpi-architecture.md)
- [Welcome to BPI](./01-welcome-to-bpi.md)
- [API Reference](./api-reference.md)
- [Developer Guide](./developer-guide.md)

### Support
- **Community Forum**: https://forum.parvyom.org
- **Documentation**: https://docs.parvyom.org
- **GitHub Issues**: https://github.com/metanode/metanode/issues
- **Email Support**: support@parvyom.org

### Quick Links
- **Main Website**: https://parvyom.org
- **BPCI Dashboard**: https://bpci.parvyom.org
- **Developer Portal**: https://dev.parvyom.org
- **Community Hub**: https://community.parvyom.org

---

*This installer guide provides comprehensive coverage of all PARVYOM Metanode deployment scenarios. For specific technical questions or advanced configuration, please refer to the additional documentation or contact our support team.*
