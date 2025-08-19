//! BPCI Validator Roles - Distinct roles for network control and high-throughput
//! 
//! This module implements distinct validator roles:
//! - BPCI Validator: Core blockchain validation for mainnet
//! - ENC BPCI Validator/Communicator: Bridge between ENC clusters and BPCI mesh

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{Mutex, RwLock};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use bpi_validator::UltraValidatorConfig;
use crate::TransportMessage;

use crate::{BpciConfig, BpciTransport};
use bpi_validator::UltraValidator;
use bpi_validator_set::{ValidatorSet, ValidatorInfo};
use bpi_enc::domain_hash;

/// Domain separation for validator role hashing
const VALIDATOR_ROLE_HASH: u8 = 0x50;

/// BPCI Validator Role Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidatorRoleType {
    /// Core BPCI validator for mainnet consensus
    BpciCore,
    /// ENC cluster validator/communicator bridge
    EncBridge,
    /// High-throughput community validator
    Community,
    /// Emergency validator for network recovery
    Emergency,
}

/// Validator role configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorRoleConfig {
    /// Role type
    pub role_type: ValidatorRoleType,
    /// Maximum TPS for this role
    pub max_tps: u32,
    /// Network control permissions
    pub network_control: bool,
    /// ENC cluster access permissions
    pub enc_access: bool,
    /// Emergency powers
    pub emergency_powers: bool,
    /// Communication protocols enabled
    pub protocols: Vec<String>,
}

/// BPCI Core Validator - Mainnet consensus validation
#[derive(Debug)]
pub struct BpciCoreValidator {
    /// Validator configuration
    config: ValidatorRoleConfig,
    /// Ultra-high-performance validator engine
    validator: Arc<UltraValidator>,
    /// Validator set for consensus
    validator_set: Arc<RwLock<ValidatorSet>>,
    /// Transport layer for BPCI mesh
    transport: Arc<Mutex<BpciTransport>>,
    /// Performance metrics
    metrics: Arc<Mutex<CoreValidatorMetrics>>,
    /// Active consensus rounds
    active_rounds: Arc<RwLock<HashMap<u64, ConsensusRound>>>,
}

/// ENC BPCI Validator/Communicator - Bridge between ENC and BPCI
#[derive(Debug)]
pub struct EncBpciValidator {
    /// Validator configuration
    config: ValidatorRoleConfig,
    /// Ultra-high-performance validator engine
    validator: Arc<UltraValidator>,
    /// ENC cluster connections
    enc_connections: Arc<RwLock<HashMap<String, EncConnection>>>,
    /// BPCI mesh transport
    transport: Arc<Mutex<BpciTransport>>,
    /// Communication metrics
    metrics: Arc<Mutex<EncValidatorMetrics>>,
    /// Message routing table
    routing_table: Arc<RwLock<HashMap<String, RoutingEntry>>>,
}

/// Core validator performance metrics
#[derive(Debug, Clone, Default)]
pub struct CoreValidatorMetrics {
    /// Blocks validated
    pub blocks_validated: u64,
    /// Consensus rounds participated
    pub consensus_rounds: u64,
    /// Network control decisions
    pub network_decisions: u64,
    /// Average validation latency (ms)
    pub avg_validation_latency: f64,
    /// Uptime since start
    pub uptime_seconds: u64,
    /// Last metrics update
    pub last_updated: u64,
}

/// ENC validator communication metrics
#[derive(Debug, Clone, Default)]
pub struct EncValidatorMetrics {
    /// Messages bridged ENC -> BPCI
    pub messages_to_bpci: u64,
    /// Messages bridged BPCI -> ENC
    pub messages_to_enc: u64,
    /// ENC clusters connected
    pub enc_clusters_connected: u32,
    /// Average bridge latency (ms)
    pub avg_bridge_latency: f64,
    /// Failed bridge attempts
    pub failed_bridges: u64,
    /// Last metrics update
    pub last_updated: u64,
}

/// ENC cluster connection information
#[derive(Debug, Clone)]
pub struct EncConnection {
    /// Cluster ID
    pub cluster_id: String,
    /// Connection endpoint
    pub endpoint: String,
    /// Connection status
    pub status: ConnectionStatus,
    /// Last activity timestamp
    pub last_activity: SystemTime,
    /// Authentication token
    pub auth_token: String,
    /// Supported protocols
    pub protocols: Vec<String>,
}

/// Connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Connected,
    Connecting,
    Disconnected,
    Failed,
    Authenticating,
}

/// Consensus round information
#[derive(Debug, Clone)]
pub struct ConsensusRound {
    /// Round number
    pub round: u64,
    /// Block height
    pub height: u64,
    /// Round start time
    pub started_at: SystemTime,
    /// Participating validators
    pub validators: Vec<String>,
    /// Round status
    pub status: RoundStatus,
}

/// Consensus round status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundStatus {
    Proposing,
    Voting,
    Committing,
    Completed,
    Failed,
}

/// Message routing entry
#[derive(Debug, Clone)]
pub struct RoutingEntry {
    /// Destination type
    pub destination: RoutingDestination,
    /// Route priority
    pub priority: u8,
    /// Last used timestamp
    pub last_used: SystemTime,
    /// Success rate (0.0-1.0)
    pub success_rate: f64,
}

/// Routing destination types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutingDestination {
    /// Route to BPCI core
    BpciCore,
    /// Route to ENC cluster
    EncCluster(String),
    /// Route to community validator
    Community,
    /// Broadcast to all
    Broadcast,
}

impl ValidatorRoleConfig {
    /// Create BPCI core validator configuration
    pub fn bpci_core() -> Self {
        Self {
            role_type: ValidatorRoleType::BpciCore,
            max_tps: 20000, // Ultra-high performance
            network_control: true,
            enc_access: false,
            emergency_powers: true,
            protocols: vec![
                "bpci-consensus".to_string(),
                "bpci-transport".to_string(),
                "bls-signatures".to_string(),
            ],
        }
    }

    /// Create ENC bridge validator configuration
    pub fn enc_bridge() -> Self {
        Self {
            role_type: ValidatorRoleType::EncBridge,
            max_tps: 15000, // High performance with bridge overhead
            network_control: false,
            enc_access: true,
            emergency_powers: false,
            protocols: vec![
                "bpci-transport".to_string(),
                "enc-bridge".to_string(),
                "jwt-auth".to_string(),
            ],
        }
    }

    /// Create community validator configuration
    pub fn community() -> Self {
        Self {
            role_type: ValidatorRoleType::Community,
            max_tps: 1000, // Community throughput
            network_control: false,
            enc_access: false,
            emergency_powers: false,
            protocols: vec![
                "bpci-transport".to_string(),
            ],
        }
    }
}

impl BpciCoreValidator {
    /// Create new BPCI core validator
    pub async fn new(
        config: ValidatorRoleConfig,
        validator_info: ValidatorInfo,
        validator_set: ValidatorSet,
        bpci_config: BpciConfig,
    ) -> Result<Self> {
        // Create ultra-high-performance validator
        let validator_config = UltraValidatorConfig::default(); // Use default config for now
        let validator = Arc::new(UltraValidator::new(
            validator_config,
            validator_info,
            validator_set.clone(),
        )?);

        // Create BPCI transport
        let transport = Arc::new(Mutex::new(BpciTransport::new(bpci_config)?));

        Ok(Self {
            config,
            validator,
            validator_set: Arc::new(RwLock::new(validator_set)),
            transport,
            metrics: Arc::new(Mutex::new(CoreValidatorMetrics::default())),
            active_rounds: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Start core validator operations
    pub async fn start(&self) -> Result<()> {
        // Start validator engine
        self.validator.start().await?;

        // Start transport layer
        self.transport.lock().await.start().await?;

        // Update metrics
        self.update_metrics().await?;

        Ok(())
    }

    /// Validate block for mainnet consensus
    pub async fn validate_block(&self, block_data: &[u8]) -> Result<bool> {
        let start_time = SystemTime::now();

        // Perform validation using ultra-validator
        // Placeholder validation - in real implementation, this would validate the block
        let is_valid = true; // Placeholder

        // Update metrics
        let mut metrics = self.metrics.lock().await;
        metrics.blocks_validated += 1;
        
        if let Ok(elapsed) = start_time.elapsed() {
            let latency_ms = elapsed.as_millis() as f64;
            metrics.avg_validation_latency = 
                (metrics.avg_validation_latency * (metrics.blocks_validated - 1) as f64 + latency_ms) 
                / metrics.blocks_validated as f64;
        }

        Ok(is_valid)
    }

    /// Participate in consensus round
    pub async fn participate_consensus(&self, round: u64, height: u64) -> Result<()> {
        let consensus_round = ConsensusRound {
            round,
            height,
            started_at: SystemTime::now(),
            validators: vec![], // Would be populated with actual validator IDs
            status: RoundStatus::Proposing,
        };

        // Add to active rounds
        self.active_rounds.write().await.insert(round, consensus_round);

        // Update metrics
        let mut metrics = self.metrics.lock().await;
        metrics.consensus_rounds += 1;

        Ok(())
    }

    /// Update performance metrics
    async fn update_metrics(&self) -> Result<()> {
        let mut metrics = self.metrics.lock().await;
        
        if let Ok(uptime) = SystemTime::now().duration_since(UNIX_EPOCH) {
            metrics.uptime_seconds = uptime.as_secs();
        }
        
        metrics.last_updated = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Ok(())
    }

    /// Get validator metrics
    pub async fn get_metrics(&self) -> CoreValidatorMetrics {
        self.metrics.lock().await.clone()
    }
}

impl EncBpciValidator {
    /// Create new ENC BPCI validator/communicator
    pub async fn new(
        config: ValidatorRoleConfig,
        validator_info: ValidatorInfo,
        validator_set: ValidatorSet,
        bpci_config: BpciConfig,
    ) -> Result<Self> {
        // Create ultra-high-performance validator
        let validator_config = UltraValidatorConfig::default();
        let validator = Arc::new(UltraValidator::new(
            validator_config,
            validator_info,
            validator_set,
        )?);

        // Create BPCI transport
        let transport = Arc::new(Mutex::new(BpciTransport::new(bpci_config)?));

        Ok(Self {
            config,
            validator,
            enc_connections: Arc::new(RwLock::new(HashMap::new())),
            transport,
            metrics: Arc::new(Mutex::new(EncValidatorMetrics::default())),
            routing_table: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Start ENC bridge validator operations
    pub async fn start(&self) -> Result<()> {
        // Start validator engine
        self.validator.start().await?;

        // Start transport layer
        self.transport.lock().await.start().await?;

        // Initialize routing table
        self.initialize_routing().await?;

        Ok(())
    }

    /// Connect to ENC cluster
    pub async fn connect_enc_cluster(&self, cluster_id: String, endpoint: String, auth_token: String) -> Result<()> {
        let connection = EncConnection {
            cluster_id: cluster_id.clone(),
            endpoint,
            status: ConnectionStatus::Connecting,
            last_activity: SystemTime::now(),
            auth_token,
            protocols: vec!["enc-bridge".to_string(), "jwt-auth".to_string()],
        };

        // Add connection
        self.enc_connections.write().await.insert(cluster_id.clone(), connection);

        // Update routing table
        let routing_entry = RoutingEntry {
            destination: RoutingDestination::EncCluster(cluster_id.clone()),
            priority: 1,
            last_used: SystemTime::now(),
            success_rate: 1.0,
        };

        self.routing_table.write().await.insert(
            format!("enc-{}", cluster_id),
            routing_entry,
        );

        // Update metrics
        let mut metrics = self.metrics.lock().await;
        metrics.enc_clusters_connected += 1;

        Ok(())
    }

    /// Bridge message from ENC to BPCI
    pub async fn bridge_to_bpci(&self, enc_cluster_id: &str, message: &[u8]) -> Result<()> {
        let start_time = SystemTime::now();

        // Route message to BPCI core
        let transport_message = TransportMessage::Data { payload: message.to_vec() };
        self.transport.lock().await.broadcast(transport_message).await?;

        // Update metrics
        let mut metrics = self.metrics.lock().await;
        metrics.messages_to_bpci += 1;

        if let Ok(elapsed) = start_time.elapsed() {
            let latency_ms = elapsed.as_millis() as f64;
            metrics.avg_bridge_latency = 
                (metrics.avg_bridge_latency * (metrics.messages_to_bpci - 1) as f64 + latency_ms) 
                / metrics.messages_to_bpci as f64;
        }

        Ok(())
    }

    /// Bridge message from BPCI to ENC
    pub async fn bridge_to_enc(&self, target_cluster: &str, _message: &[u8]) -> Result<()> {
        let start_time = SystemTime::now();

        // Find ENC connection
        let connections = self.enc_connections.read().await;
        if let Some(connection) = connections.get(target_cluster) {
            if connection.status == ConnectionStatus::Connected {
                // Send message to ENC cluster (placeholder - would use actual ENC protocol)
                // In real implementation, this would use the ENC cluster API
                
                // Update metrics
                let mut metrics = self.metrics.lock().await;
                metrics.messages_to_enc += 1;

                if let Ok(elapsed) = start_time.elapsed() {
                    let latency_ms = elapsed.as_millis() as f64;
                    metrics.avg_bridge_latency = 
                        (metrics.avg_bridge_latency * (metrics.messages_to_enc - 1) as f64 + latency_ms) 
                        / metrics.messages_to_enc as f64;
                }
            }
        }

        Ok(())
    }

    /// Initialize routing table
    async fn initialize_routing(&self) -> Result<()> {
        let mut routing_table = self.routing_table.write().await;

        // Add BPCI core route
        routing_table.insert(
            "bpci-core".to_string(),
            RoutingEntry {
                destination: RoutingDestination::BpciCore,
                priority: 0, // Highest priority
                last_used: SystemTime::now(),
                success_rate: 1.0,
            },
        );

        Ok(())
    }

    /// Get validator metrics
    pub async fn get_metrics(&self) -> EncValidatorMetrics {
        self.metrics.lock().await.clone()
    }
}

/// Hash validator role configuration
pub fn hash_validator_role(config: &ValidatorRoleConfig) -> Result<[u8; 32]> {
    // Use serde_json for encoding since CanonicalCbor trait is not available
    let encoded = serde_json::to_vec(config)?;
    Ok(domain_hash(VALIDATOR_ROLE_HASH, &encoded))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use bpi_validator_set::{ValidatorMetadata, ValidatorStatus};

    fn create_test_validator_info() -> ValidatorInfo {
        use bpi_blsagg::PublicKey as BlsPublicKey;
        use bpi_vrf::VrfPublicKey;
        
        let test_bls_bytes = [1u8; 48];
        let bls_pubkey = BlsPublicKey::from_bytes(&test_bls_bytes).unwrap();
        
        let test_vrf_bytes = [2u8; 32];
        let vrf_pubkey = VrfPublicKey::from_bytes(&test_vrf_bytes).unwrap();
        
        ValidatorInfo {
            index: 0,
            bls_pubkey,
            vrf_pubkey,
            stake: 1000,
            address: "127.0.0.1:8080".to_string(),
            metadata: ValidatorMetadata {
                name: "test-validator".to_string(),
                registered_at: Utc::now(),
                last_active: Utc::now(),
                status: ValidatorStatus::Active,
            },
        }
    }

    #[test]
    fn test_validator_role_configs() {
        let bpci_config = ValidatorRoleConfig::bpci_core();
        assert_eq!(bpci_config.role_type, ValidatorRoleType::BpciCore);
        assert_eq!(bpci_config.max_tps, 20000);
        assert!(bpci_config.network_control);
        assert!(bpci_config.emergency_powers);

        let enc_config = ValidatorRoleConfig::enc_bridge();
        assert_eq!(enc_config.role_type, ValidatorRoleType::EncBridge);
        assert_eq!(enc_config.max_tps, 15000);
        assert!(!enc_config.network_control);
        assert!(enc_config.enc_access);

        let community_config = ValidatorRoleConfig::community();
        assert_eq!(community_config.role_type, ValidatorRoleType::Community);
        assert_eq!(community_config.max_tps, 1000);
        assert!(!community_config.network_control);
        assert!(!community_config.enc_access);
    }

    #[test]
    fn test_validator_role_hashing() {
        let config = ValidatorRoleConfig::bpci_core();
        let hash_result = hash_validator_role(&config);
        assert!(hash_result.is_ok());
        
        // Hash should be deterministic
        let hash1 = hash_validator_role(&config).unwrap();
        let hash2 = hash_validator_role(&config).unwrap();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_enc_connection_creation() {
        let connection = EncConnection {
            cluster_id: "test-cluster".to_string(),
            endpoint: "https://test.example.com".to_string(),
            status: ConnectionStatus::Connected,
            last_activity: SystemTime::now(),
            auth_token: "test-token".to_string(),
            protocols: vec!["enc-bridge".to_string()],
        };

        assert_eq!(connection.cluster_id, "test-cluster");
        assert_eq!(connection.status, ConnectionStatus::Connected);
        assert!(connection.protocols.contains(&"enc-bridge".to_string()));
    }

    #[test]
    fn test_routing_entry_creation() {
        let entry = RoutingEntry {
            destination: RoutingDestination::BpciCore,
            priority: 0,
            last_used: SystemTime::now(),
            success_rate: 1.0,
        };

        assert_eq!(entry.destination, RoutingDestination::BpciCore);
        assert_eq!(entry.priority, 0);
        assert_eq!(entry.success_rate, 1.0);
    }

    #[test]
    fn test_consensus_round_creation() {
        let round = ConsensusRound {
            round: 1,
            height: 100,
            started_at: SystemTime::now(),
            validators: vec!["validator1".to_string()],
            status: RoundStatus::Proposing,
        };

        assert_eq!(round.round, 1);
        assert_eq!(round.height, 100);
        assert_eq!(round.status, RoundStatus::Proposing);
    }

    #[tokio::test]
    async fn test_validator_role_exit_criteria() {
        // Test that distinct BPCI validator and ENC BPCI validator/communicator roles are implemented
        
        // BPCI Core Validator role
        let bpci_config = ValidatorRoleConfig::bpci_core();
        assert_eq!(bpci_config.role_type, ValidatorRoleType::BpciCore);
        assert!(bpci_config.network_control); // Network control for BPCI
        assert!(!bpci_config.enc_access); // No ENC access for core
        
        // ENC Bridge Validator role  
        let enc_config = ValidatorRoleConfig::enc_bridge();
        assert_eq!(enc_config.role_type, ValidatorRoleType::EncBridge);
        assert!(!enc_config.network_control); // No network control for bridge
        assert!(enc_config.enc_access); // ENC access for bridge
        
        // Roles are distinct and serve different purposes
        assert_ne!(bpci_config.role_type, enc_config.role_type);
        assert_ne!(bpci_config.max_tps, enc_config.max_tps);
        assert_ne!(bpci_config.network_control, enc_config.network_control);
        assert_ne!(bpci_config.enc_access, enc_config.enc_access);
    }
}
