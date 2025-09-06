 # Metanode Architecture Separation: Hosted vs Installable Components

## Overview

Metanode follows a **light mesh architecture** where we host minimal core infrastructure while the community builds and deploys the full economics and application layers. This separation enables rapid scaling, reduces our operational burden, and empowers developers to create their own clusters.

---

## 🏗️ **What We Host (Light Mesh)**

### **Core Infrastructure Services**
These are the minimal, essential services we maintain and host:

#### **1. Light Client Network**
- **Purpose**: Provides consensus verification and header validation
- **Components**: 
  - Light client nodes (bpi-light-client)
  - Header proxy service (Stage 49)
  - Consensus verification
- **Hosting**: Distributed across multiple regions
- **Access**: Public RPC endpoints for verification

#### **2. Directory Service**
- **Purpose**: Validator discovery and diversity policy enforcement
- **Components**:
  - Validator directory (Stage 48)
  - Diversity policy engine
  - ASN/region/client type tracking
- **Hosting**: Centralized service with redundancy
- **Access**: Public API for validator registration

#### **3. Faucet Service (Testnet)**
- **Purpose**: Token distribution for testnet development
- **Components**:
  - Testnet token dispenser
  - Rate limiting and abuse prevention
  - reCAPTCHA integration
- **Hosting**: Single service with Redis backend
- **Access**: Web UI and API endpoints

#### **4. Documentation & Registry**
- **Purpose**: Developer resources and package discovery
- **Components**:
  - API documentation
  - Package registry for community tools
  - Examples and tutorials
- **Hosting**: Static hosting (CDN)
- **Access**: Public web interface

---

## 📦 **What Users Install (Full Node Package)**

### **Metanode CLI Installer**
Users install a comprehensive CLI package that includes:

#### **1. Core Node Software**
```bash
# Full consensus node with all features
metanode-node/
├── bpi-consensus/          # IBFT consensus engine
├── bpi-headers/            # Header management
├── bpi-merkle/             # Merkle tree operations
├── bpi-vrf/                # VRF leader selection
├── bpi-poh/                # Proof of History
└── bpi-anchor/             # L1 anchoring
```

#### **2. DockLock Container Runtime**
```bash
# Revolutionary blockchain-aware container orchestration
docklock/
├── enc-cluster/            # ENC cluster management
├── receipt-system/         # Receipt generation and verification
├── policy-engine/          # WASM policy execution
├── court-container/        # Agreement hosting
└── traffic-light/          # Data flow control
```

#### **3. Autonomous Economics Engine**
```bash
# Complete economic system
autonomous-economics/
├── token-management/       # GEN, NEX, FLX, AUR tokens
├── poe-mining/             # Proof of Economics mining
├── governance/             # Community governance system
├── treasury/               # Automated fund management
└── fee-routing/            # Economic fee distribution
```

#### **4. Data Availability Layer**
```bash
# Full DA infrastructure
data-availability/
├── biso-policy/            # Policy-as-code compliance
├── packet-envelope/        # Cryptographic data wrappers
├── da-sampler/             # Data availability sampling
├── blockbook/              # Immutable audit ledger
└── traffic-dashboard/      # Real-time monitoring
```

#### **5. Networking & Security**
```bash
# P2P networking and security
networking/
├── relay-diversity/        # Relay management
├── slashing-evidence/      # Evidence export system
├── encrypted-mempool/      # Censorship resistance
└── force-inclusion/        # Inclusion guarantees
```

---

## 🎯 **Deployment Models**

### **Shadow Mainnet Clusters**
Community developers can deploy full clusters that:

1. **Connect to Light Mesh**: Use our hosted light client network for consensus verification
2. **Run Full Economics**: Deploy complete autonomous economics engine
3. **Host Applications**: Run their own DockLock container clusters
4. **Manage Governance**: Operate independent governance systems
5. **Provide Services**: Offer specialized services to their users

### **Banking Integration Clusters**
Financial institutions can deploy:

1. **Compliance-First Setup**: Full BISO policy enforcement
2. **AUR Token Integration**: Gold-backed settlement tokens
3. **Cross-Border Rails**: International payment infrastructure
4. **Regulatory Reporting**: Automated compliance and audit trails
5. **Private Governance**: Internal parameter management

### **Developer Sandbox Clusters**
Individual developers can run:

1. **Local Development**: Full node on laptop/desktop
2. **Testnet Integration**: Connect to our hosted testnet faucet
3. **Rapid Prototyping**: Quick setup for app development
4. **Learning Environment**: Educational and experimental use
5. **MVP Deployment**: Small-scale production testing

---

## 🔄 **Interaction Model**

### **Light Mesh Dependencies**
User installations interact with our hosted services for:

- **Consensus Verification**: Light client header validation
- **Validator Discovery**: Finding and connecting to validators
- **Testnet Tokens**: Getting tokens for development
- **Documentation**: Accessing guides and references

### **Independent Operations**
User installations operate independently for:

- **Full Consensus**: Running complete validator nodes
- **Economic Mining**: PoE mining and token generation
- **Application Hosting**: DockLock container orchestration
- **Data Storage**: Local and multi-cloud DA storage
- **Governance Decisions**: Community parameter management

---

## 📊 **Resource Requirements**

### **Our Hosting Costs (Light Mesh)**
- **Light Client Network**: ~$500/month (3 regions, 9 nodes)
- **Directory Service**: ~$200/month (single service + Redis)
- **Faucet Service**: ~$100/month (testnet only)
- **Documentation**: ~$50/month (static hosting)
- **Total**: ~$850/month for global infrastructure

### **User Installation Requirements**
- **Minimum**: 4GB RAM, 2 CPU cores, 100GB storage
- **Recommended**: 16GB RAM, 8 CPU cores, 1TB SSD
- **Production**: 32GB RAM, 16 CPU cores, 2TB NVMe
- **Network**: Stable internet, public IP preferred

---

## 🚀 **Scaling Strategy**

### **Horizontal Scaling**
As adoption grows:

1. **Community Clusters**: More shadow mainnet deployments
2. **Regional Hubs**: Geographic distribution of full nodes
3. **Specialized Services**: Domain-specific cluster deployments
4. **Banking Networks**: Financial institution adoption
5. **Developer Ecosystem**: Tool and service proliferation

### **Light Mesh Evolution**
Our hosted infrastructure evolves to:

1. **Enhanced Directory**: Better validator discovery
2. **Performance Monitoring**: Network health dashboards
3. **Security Services**: Threat detection and mitigation
4. **Integration APIs**: Easier third-party connections
5. **Global Redundancy**: Multi-region failover

---

## 🎯 **Success Metrics**

### **Adoption Indicators**
- **CLI Installations**: Monthly active installations
- **Cluster Deployments**: Number of shadow mainnet clusters
- **Transaction Volume**: Economic activity across clusters
- **Developer Activity**: Package downloads and contributions
- **Banking Partnerships**: Financial institution adoption

### **Network Health**
- **Light Client Uptime**: 99.9% availability target
- **Consensus Performance**: Sub-second finality
- **Validator Diversity**: Geographic and organizational spread
- **Economic Activity**: PoE mining participation
- **Governance Participation**: Community proposal activity

---

## 📋 **Implementation Timeline**

### **Phase 1: Light Mesh (Current)**
- ✅ Light client network operational
- ✅ Directory service deployed
- ✅ Faucet service running
- ✅ Basic documentation available

### **Phase 2: CLI Installer (Next)**
- 🔄 Complete CLI package creation
- 🔄 Docker integration and testing
- 🔄 Installation scripts and automation
- 🔄 User onboarding documentation

### **Phase 3: Community Deployment**
- 📋 First shadow mainnet clusters
- 📋 Banking pilot programs
- 📋 Developer ecosystem growth
- 📋 Performance optimization

### **Phase 4: Production Scale**
- 📋 Global cluster network
- 📋 Enterprise integrations
- 📋 Advanced governance features
- 📋 Mainstream adoption

---

This separation enables **rapid scaling** while maintaining **operational efficiency**. We focus on essential infrastructure while empowering the community to build the full economic and application ecosystem.
