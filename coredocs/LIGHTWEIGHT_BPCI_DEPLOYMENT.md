# Lightweight BPCI Deployment: 4GB RAM Cloud Server Configuration
## Minimal Owner Server with Auto-Scaling Architecture

### **Executive Summary**

This document defines the lightweight deployment configuration for the BPCI owner server running on a 4GB RAM cloud server. The system starts with minimal components (1 BPCI Core, 1 BPCI Notary, 1 BPCI Validator, 1 BPCI Registry) and automatically scales as people connect and use the network.

---

## **Current Autonomous Economy Setup**

### **✅ Already Implemented Autonomous Economy Features**

Based on the codebase analysis, the autonomous economy is already implemented with:

1. **Economic Integration System** (`unified_api.rs`)
   - Economic monitoring and metrics tracking
   - Owner wallet withdrawal system
   - Infrastructure fee collection
   - Autonomous economic decision making

2. **BPCI Validator Roles** (`validator_roles.rs`)
   - BPCI Core Validator for mainnet consensus
   - ENC BPCI Validator/Communicator for bridging
   - Community validator support
   - Auto-scaling validator network

3. **Unified API Gateway** 
   - Service registration and discovery
   - Deployment management
   - Cluster registration
   - Economic status monitoring

---

## **4GB RAM Cloud Server Configuration**

### **Minimal BPCI Owner Server Components**

```rust
// Lightweight BPCI server configuration for 4GB RAM
pub struct LightweightBpciConfig {
    /// Server resource limits
    pub resource_limits: ResourceLimits,
    /// Core BPCI components
    pub core_components: CoreComponents,
    /// Auto-scaling configuration
    pub auto_scaling: AutoScalingConfig,
    /// Network discovery settings
    pub network_discovery: NetworkDiscoveryConfig,
}

#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Total RAM allocation (4GB)
    pub total_ram_mb: u32, // 4096 MB
    /// RAM allocation per component
    pub component_ram_allocation: ComponentRamAllocation,
    /// CPU limits
    pub cpu_limits: CpuLimits,
    /// Storage limits
    pub storage_limits: StorageLimits,
}

#[derive(Debug, Clone)]
pub struct ComponentRamAllocation {
    /// BPCI Core: 1.5GB RAM
    pub bpci_core_mb: u32, // 1536 MB
    /// BPCI Notary: 1GB RAM
    pub bpci_notary_mb: u32, // 1024 MB
    /// BPCI Validator: 1GB RAM
    pub bpci_validator_mb: u32, // 1024 MB
    /// BPCI Registry: 512MB RAM
    pub bpci_registry_mb: u32, // 512 MB
}

#[derive(Debug, Clone)]
pub struct CoreComponents {
    /// Single BPCI Core instance
    pub bpci_core: BpciCoreConfig,
    /// Single BPCI Notary instance
    pub bpci_notary: BpciNotaryConfig,
    /// Single BPCI Validator instance
    pub bpci_validator: BpciValidatorConfig,
    /// Single BPCI Registry instance
    pub bpci_registry: BpciRegistryConfig,
}
```

### **BPCI Core Configuration (1.5GB RAM)**

```rust
#[derive(Debug, Clone)]
pub struct BpciCoreConfig {
    /// Core blockchain engine
    pub blockchain_engine: BlockchainEngineConfig,
    /// Consensus system (IBFT)
    pub consensus: ConsensusConfig,
    /// Transaction pool (lightweight)
    pub transaction_pool: TransactionPoolConfig,
    /// State management (minimal)
    pub state_management: StateManagementConfig,
}

impl BpciCoreConfig {
    pub fn lightweight_4gb() -> Self {
        Self {
            blockchain_engine: BlockchainEngineConfig {
                max_block_size: 1024 * 1024, // 1MB blocks
                block_time_ms: 5000, // 5 second blocks
                max_transactions_per_block: 1000,
                memory_pool_size_mb: 256, // 256MB for tx pool
            },
            consensus: ConsensusConfig {
                consensus_type: ConsensusType::IBFT,
                validator_threshold: 1, // Single validator initially
                timeout_ms: 3000,
                max_validators: 100, // Auto-scale up to 100
            },
            transaction_pool: TransactionPoolConfig {
                max_pending_transactions: 10000,
                max_memory_mb: 128,
                cleanup_interval_ms: 30000,
            },
            state_management: StateManagementConfig {
                state_cache_mb: 256,
                history_retention_blocks: 1000,
                checkpoint_interval: 100,
            },
        }
    }
}
```

### **BPCI Notary Configuration (1GB RAM)**

```rust
#[derive(Debug, Clone)]
pub struct BpciNotaryConfig {
    /// Notary service configuration
    pub notary_service: NotaryServiceConfig,
    /// Signature management
    pub signature_management: SignatureManagementConfig,
    /// Witness system
    pub witness_system: WitnessSystemConfig,
}

impl BpciNotaryConfig {
    pub fn lightweight_4gb() -> Self {
        Self {
            notary_service: NotaryServiceConfig {
                max_concurrent_notarizations: 100,
                notarization_timeout_ms: 10000,
                signature_cache_mb: 128,
                witness_retention_hours: 24,
            },
            signature_management: SignatureManagementConfig {
                signature_algorithm: SignatureAlgorithm::Ed25519,
                key_rotation_interval_hours: 24,
                signature_verification_threads: 4,
            },
            witness_system: WitnessSystemConfig {
                max_witnesses_per_transaction: 3,
                witness_selection_algorithm: WitnessSelection::Random,
                witness_cache_mb: 64,
            },
        }
    }
}
```

### **BPCI Validator Configuration (1GB RAM)**

```rust
#[derive(Debug, Clone)]
pub struct BpciValidatorConfig {
    /// Validator engine
    pub validator_engine: ValidatorEngineConfig,
    /// Consensus participation
    pub consensus_participation: ConsensusParticipationConfig,
    /// Network communication
    pub network_communication: NetworkCommunicationConfig,
}

impl BpciValidatorConfig {
    pub fn lightweight_4gb() -> Self {
        Self {
            validator_engine: ValidatorEngineConfig {
                validation_threads: 2,
                max_validation_queue: 1000,
                validation_timeout_ms: 5000,
                cache_size_mb: 128,
            },
            consensus_participation: ConsensusParticipationConfig {
                participation_mode: ParticipationMode::Active,
                voting_timeout_ms: 3000,
                proposal_timeout_ms: 2000,
            },
            network_communication: NetworkCommunicationConfig {
                max_connections: 50,
                connection_timeout_ms: 5000,
                message_buffer_mb: 64,
                heartbeat_interval_ms: 10000,
            },
        }
    }
}
```

### **BPCI Registry Configuration (512MB RAM)**

```rust
#[derive(Debug, Clone)]
pub struct BpciRegistryConfig {
    /// Service registry
    pub service_registry: ServiceRegistryConfig,
    /// Node discovery
    pub node_discovery: NodeDiscoveryConfig,
    /// Health monitoring
    pub health_monitoring: HealthMonitoringConfig,
}

impl BpciRegistryConfig {
    pub fn lightweight_4gb() -> Self {
        Self {
            service_registry: ServiceRegistryConfig {
                max_registered_services: 1000,
                service_ttl_seconds: 300, // 5 minutes
                registry_cache_mb: 64,
                cleanup_interval_ms: 60000, // 1 minute
            },
            node_discovery: NodeDiscoveryConfig {
                discovery_protocol: DiscoveryProtocol::mDNS,
                discovery_interval_ms: 30000, // 30 seconds
                max_discovered_nodes: 500,
                node_cache_mb: 32,
            },
            health_monitoring: HealthMonitoringConfig {
                health_check_interval_ms: 15000, // 15 seconds
                unhealthy_threshold: 3,
                recovery_check_interval_ms: 60000, // 1 minute
            },
        }
    }
}
```

---

## **Auto-Scaling Configuration**

### **Auto-Scaling System**

```rust
#[derive(Debug, Clone)]
pub struct AutoScalingConfig {
    /// Scaling triggers
    pub scaling_triggers: ScalingTriggers,
    /// Component scaling rules
    pub component_scaling: ComponentScalingRules,
    /// Resource management
    pub resource_management: ResourceManagementConfig,
}

#[derive(Debug, Clone)]
pub struct ScalingTriggers {
    /// CPU usage threshold for scaling up
    pub cpu_scale_up_threshold: f64, // 70%
    /// Memory usage threshold for scaling up
    pub memory_scale_up_threshold: f64, // 80%
    /// Connection count threshold
    pub connection_scale_up_threshold: u32, // 80% of max connections
    /// Transaction volume threshold
    pub transaction_scale_up_threshold: u32, // 80% of max tx/sec
}

impl AutoScalingConfig {
    pub fn for_4gb_server() -> Self {
        Self {
            scaling_triggers: ScalingTriggers {
                cpu_scale_up_threshold: 0.70,
                memory_scale_up_threshold: 0.80,
                connection_scale_up_threshold: 40, // 80% of 50 max connections
                transaction_scale_up_threshold: 800, // 80% of 1000 max tx/block
            },
            component_scaling: ComponentScalingRules {
                validator_scaling: ValidatorScalingRule {
                    min_validators: 1,
                    max_validators: 10, // Limited by 4GB RAM
                    scale_up_increment: 1,
                    scale_down_cooldown_ms: 300000, // 5 minutes
                },
                notary_scaling: NotaryScalingRule {
                    min_notaries: 1,
                    max_notaries: 3, // Limited by RAM
                    scale_up_threshold: 0.80,
                },
                registry_scaling: RegistryScalingRule {
                    enable_clustering: false, // Single registry for 4GB
                    max_registry_instances: 1,
                },
            },
            resource_management: ResourceManagementConfig {
                memory_pressure_threshold: 0.90, // 90% RAM usage
                emergency_scaling_enabled: true,
                resource_rebalancing_enabled: true,
            },
        }
    }
}
```

### **Network Discovery & Auto-Configuration**

```rust
#[derive(Debug, Clone)]
pub struct NetworkDiscoveryConfig {
    /// Discovery mechanisms
    pub discovery_mechanisms: Vec<DiscoveryMechanism>,
    /// Auto-configuration settings
    pub auto_configuration: AutoConfigurationSettings,
    /// Peer management
    pub peer_management: PeerManagementConfig,
}

#[derive(Debug, Clone)]
pub enum DiscoveryMechanism {
    /// mDNS for local network discovery
    MDNS {
        service_name: String,
        port: u16,
        ttl_seconds: u32,
    },
    /// DHT for distributed discovery
    DHT {
        bootstrap_nodes: Vec<String>,
        replication_factor: u8,
    },
    /// DNS-based discovery
    DNS {
        discovery_domain: String,
        srv_record_name: String,
    },
    /// Manual peer configuration
    Manual {
        static_peers: Vec<String>,
    },
}

impl NetworkDiscoveryConfig {
    pub fn for_owner_server() -> Self {
        Self {
            discovery_mechanisms: vec![
                DiscoveryMechanism::MDNS {
                    service_name: "_bpci._tcp.local".to_string(),
                    port: 8080,
                    ttl_seconds: 300,
                },
                DiscoveryMechanism::DHT {
                    bootstrap_nodes: vec![], // Will be populated as network grows
                    replication_factor: 3,
                },
                DiscoveryMechanism::DNS {
                    discovery_domain: "bpci.metanode.network".to_string(),
                    srv_record_name: "_bpci._tcp".to_string(),
                },
            ],
            auto_configuration: AutoConfigurationSettings {
                auto_configure_firewall: true,
                auto_configure_networking: true,
                auto_generate_certificates: true,
                auto_register_services: true,
            },
            peer_management: PeerManagementConfig {
                max_inbound_connections: 25,
                max_outbound_connections: 25,
                connection_retry_interval_ms: 30000,
                peer_scoring_enabled: true,
            },
        }
    }
}
```

---

## **Deployment Configuration File**

### **config.toml for 4GB Server**

```toml
# BPCI Owner Server Configuration - 4GB RAM Cloud Server
[server]
name = "bpci-owner-server"
version = "1.0.0"
environment = "production"
log_level = "info"

[resources]
total_ram_mb = 4096
cpu_cores = 2
storage_gb = 50
network_bandwidth_mbps = 100

[bpci_core]
enabled = true
ram_allocation_mb = 1536
port = 8080
blockchain_engine = "lightweight"
consensus_type = "IBFT"
max_block_size = 1048576  # 1MB
block_time_ms = 5000      # 5 seconds
max_transactions_per_block = 1000

[bpci_notary]
enabled = true
ram_allocation_mb = 1024
port = 8081
max_concurrent_notarizations = 100
signature_algorithm = "Ed25519"
witness_retention_hours = 24

[bpci_validator]
enabled = true
ram_allocation_mb = 1024
port = 8082
validation_threads = 2
max_validation_queue = 1000
participation_mode = "active"

[bpci_registry]
enabled = true
ram_allocation_mb = 512
port = 8083
max_registered_services = 1000
service_ttl_seconds = 300
discovery_protocol = "mDNS"

[auto_scaling]
enabled = true
cpu_scale_up_threshold = 0.70
memory_scale_up_threshold = 0.80
connection_scale_up_threshold = 40
transaction_scale_up_threshold = 800

[network_discovery]
mechanisms = ["mDNS", "DHT", "DNS"]
service_name = "_bpci._tcp.local"
discovery_domain = "bpci.metanode.network"
auto_configure = true

[autonomous_economy]
enabled = true
owner_withdrawal_enabled = true
infrastructure_fee_rate = 0.001  # 0.1%
economic_monitoring_interval_ms = 60000  # 1 minute
auto_scaling_economic_triggers = true

[security]
tls_enabled = true
auto_generate_certificates = true
firewall_auto_configure = true
rate_limiting_enabled = true
max_requests_per_minute = 1000

[monitoring]
metrics_enabled = true
health_checks_enabled = true
performance_monitoring = true
economic_monitoring = true
alert_thresholds = { cpu = 0.80, memory = 0.85, disk = 0.90 }
```

---

## **Auto-Scaling Behavior**

### **How Components Scale as People Connect**

**Initial State (4GB Server):**
```
BPCI Core (1.5GB) + BPCI Notary (1GB) + BPCI Validator (1GB) + BPCI Registry (512MB) = 4GB
```

**As Network Grows:**

1. **1-10 Connected Nodes:**
   - Single instance of each component
   - Automatic service discovery via mDNS
   - Basic economic monitoring active

2. **11-50 Connected Nodes:**
   - BPCI Validator scales to 2 instances (if RAM allows)
   - BPCI Notary increases concurrent capacity
   - DHT discovery mechanism activates

3. **51-100 Connected Nodes:**
   - Additional validator instances spawn on connecting nodes
   - Registry begins clustering preparation
   - Economic incentives increase to attract more resources

4. **100+ Connected Nodes:**
   - Full distributed network with multiple owner servers
   - Autonomous economic scaling kicks in
   - Community validators join the network
   - Bank Mesh components activate

### **Economic Auto-Scaling Triggers**

```rust
pub struct EconomicScalingTriggers {
    /// Revenue threshold to spawn new components
    pub revenue_scale_up_threshold: f64, // $100/day
    /// Transaction volume economic trigger
    pub transaction_volume_economic_trigger: u64, // 10,000 tx/day
    /// Network value threshold for expansion
    pub network_value_expansion_threshold: f64, // $10,000 total value locked
}

impl EconomicScalingTriggers {
    pub fn trigger_scaling(&self, current_metrics: &EconomicMetrics) -> ScalingDecision {
        if current_metrics.daily_revenue > self.revenue_scale_up_threshold {
            ScalingDecision::ScaleUp {
                reason: "Revenue threshold exceeded".to_string(),
                recommended_action: ScalingAction::SpawnAdditionalServer,
            }
        } else if current_metrics.daily_transactions > self.transaction_volume_economic_trigger {
            ScalingDecision::ScaleUp {
                reason: "Transaction volume threshold exceeded".to_string(),
                recommended_action: ScalingAction::IncreaseCapacity,
            }
        } else {
            ScalingDecision::Maintain
        }
    }
}
```

---

## **Deployment Commands**

### **Single Command Deployment**

```bash
# Deploy BPCI owner server on 4GB cloud server
./bpci-server --config config.toml --mode owner-server --auto-scale

# Alternative with environment variables
export BPCI_RAM_LIMIT=4096
export BPCI_AUTO_SCALE=true
export BPCI_OWNER_MODE=true
./bpci-server start
```

### **Docker Deployment (Optional)**

```dockerfile
# Lightweight BPCI Docker image for 4GB server
FROM rust:1.70-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin bpci-server

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/bpci-server /usr/local/bin/
COPY config.toml /etc/bpci/
EXPOSE 8080-8083
CMD ["bpci-server", "--config", "/etc/bpci/config.toml"]
```

---

## **Success Metrics**

### **4GB Server Performance Targets**

- **Memory Usage:** < 90% of 4GB (3.6GB max usage)
- **CPU Usage:** < 70% average, < 90% peak
- **Transaction Throughput:** > 100 TPS initially, auto-scale to 1000+ TPS
- **Network Connections:** Support 50+ concurrent connections
- **Uptime:** > 99.9% availability
- **Auto-Scaling Response:** < 30 seconds to detect and respond to scaling triggers
- **Economic Efficiency:** > $1/day revenue to cover server costs

### **Network Growth Targets**

- **Week 1:** 1-5 connected nodes (testing phase)
- **Month 1:** 10-25 connected nodes (early adoption)
- **Month 3:** 50-100 connected nodes (network effect)
- **Month 6:** 100+ connected nodes (autonomous scaling)

This lightweight deployment configuration ensures the BPCI owner server can start small on a 4GB RAM cloud server and automatically scale as the network grows, providing a cost-effective foundation for the autonomous economy while maintaining enterprise-grade reliability and security.
