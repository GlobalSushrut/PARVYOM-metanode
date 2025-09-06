//! # BPI Node Coordinator Commands
//! 
//! CLI commands for managing BPI node coordinator and specialized nodes

// Using simplified BPI node coordinator
use crate::bpi_node_coordinator::*;
use anyhow::{Result, anyhow};
use clap::{Args, Subcommand};
use tracing::{info, warn, error};
use serde_json;

#[derive(Debug, Clone, Args)]
pub struct NodeCoordinatorArgs {
    #[command(subcommand)]
    pub command: NodeCoordinatorCommand,
}

#[derive(Debug, Clone, Subcommand)]
pub enum NodeCoordinatorCommand {
    /// Start the BPI node coordinator
    Start {
        /// Configuration file path
        #[arg(short, long)]
        config: Option<String>,
    },
    /// Stop the BPI node coordinator
    Stop,
    /// Get status of all active nodes
    Status,
    /// Start a specific BPI node type
    StartNode {
        /// Node type to start
        #[arg(short, long)]
        node_type: String,
        /// Node endpoint
        #[arg(short, long)]
        endpoint: String,
        /// Additional configuration as JSON
        #[arg(short, long)]
        config: Option<String>,
    },
    /// Stop a specific node
    StopNode {
        /// Node ID to stop
        #[arg(short, long)]
        node_id: String,
    },
    /// List all available node types
    ListNodeTypes,
    /// Run integration tests
    Test,
    /// Start ENC cluster node
    StartEncCluster {
        /// Cluster ID
        #[arg(short, long)]
        cluster_id: String,
        /// Encryption level (standard, military, quantum)
        #[arg(short, long, default_value = "military")]
        encryption: String,
        /// Gateway endpoint
        #[arg(short, long, default_value = "http://localhost:8080")]
        gateway: String,
        /// Mempool size
        #[arg(short, long, default_value = "10000")]
        mempool_size: u32,
        /// Node endpoint
        #[arg(long, default_value = "http://localhost:9001")]
        endpoint: String,
    },
    /// Start Oracle node
    StartOracle {
        /// Oracle type (price, data, cross-chain, governance)
        #[arg(short, long, default_value = "price")]
        oracle_type: String,
        /// Supported chains (comma-separated)
        #[arg(short, long, default_value = "BPI")]
        chains: String,
        /// Update frequency in milliseconds
        #[arg(short, long, default_value = "5000")]
        frequency: u64,
        /// Reliability score (0.0-1.0)
        #[arg(short, long, default_value = "0.95")]
        reliability: f64,
        /// Node endpoint
        #[arg(long, default_value = "http://localhost:9002")]
        endpoint: String,
    },
    /// Start Shadow Registry node
    StartShadowRegistry {
        /// Registry type (web2-bridge, privacy, compliance)
        #[arg(short, long, default_value = "web2-bridge")]
        registry_type: String,
        /// Web2 endpoints (comma-separated)
        #[arg(short, long)]
        web2_endpoints: Option<String>,
        /// Web3 contract addresses (comma-separated)
        #[arg(short, long)]
        web3_contracts: Option<String>,
        /// Bridge capacity
        #[arg(short, long, default_value = "1000")]
        capacity: u32,
        /// Node endpoint
        #[arg(long, default_value = "http://localhost:9003")]
        endpoint: String,
    },
    /// Start Pipeline API node
    StartPipelineApi {
        /// Pipeline ID
        #[arg(short, long)]
        pipeline_id: String,
        /// BISO policies (comma-separated)
        #[arg(short, long)]
        biso_policies: Option<String>,
        /// Traffic light rules (comma-separated)
        #[arg(short, long)]
        traffic_rules: Option<String>,
        /// Throughput limit
        #[arg(short, long, default_value = "5000")]
        throughput: u32,
        /// Node endpoint
        #[arg(long, default_value = "http://localhost:9004")]
        endpoint: String,
    },
    /// Start Storage node
    StartStorage {
        /// Storage type (distributed, high-performance, archive)
        #[arg(short, long, default_value = "distributed")]
        storage_type: String,
        /// Capacity in GB
        #[arg(short, long, default_value = "1000")]
        capacity: u64,
        /// Replication factor
        #[arg(short, long, default_value = "3")]
        replication: u32,
        /// Enable encryption
        #[arg(short, long)]
        encryption: bool,
        /// Node endpoint
        #[arg(long, default_value = "http://localhost:9005")]
        endpoint: String,
    },
    /// Start Proof node
    StartProof {
        /// Proof type (compliance, identity, transaction)
        #[arg(short, long, default_value = "compliance")]
        proof_type: String,
        /// Compliance level (basic, enhanced, government)
        #[arg(short, long, default_value = "enhanced")]
        compliance: String,
        /// Audit retention in days
        #[arg(short, long, default_value = "2555")]
        retention: u32,
        /// Government endpoints (comma-separated)
        #[arg(short, long)]
        gov_endpoints: Option<String>,
        /// Node endpoint
        #[arg(long, default_value = "http://localhost:9006")]
        endpoint: String,
    },
    /// Start Audit node
    StartAudit {
        /// Audit scope (transaction, node, full-system)
        #[arg(short, long, default_value = "full-system")]
        scope: String,
        /// Compliance frameworks (comma-separated)
        #[arg(short, long, default_value = "SOX,GDPR")]
        frameworks: String,
        /// Audit frequency in hours
        #[arg(short, long, default_value = "24")]
        frequency: u32,
        /// Reporting endpoints (comma-separated)
        #[arg(short, long)]
        reporting_endpoints: Option<String>,
        /// Node endpoint
        #[arg(long, default_value = "http://localhost:9007")]
        endpoint: String,
    },
    /// Start Logbook node
    StartLogbook {
        /// Logbook type (auction, transaction, compliance)
        #[arg(short, long, default_value = "auction")]
        logbook_type: String,
        /// Receipt sources (comma-separated)
        #[arg(short, long, default_value = "http-cage,docklock,enc-cluster")]
        sources: String,
        /// Storage policy
        #[arg(short, long, default_value = "replicated")]
        storage_policy: String,
        /// Retention policy
        #[arg(short, long, default_value = "7years")]
        retention_policy: String,
        /// Node endpoint
        #[arg(long, default_value = "http://localhost:9008")]
        endpoint: String,
    },
}

pub async fn handle_node_coordinator_command(args: NodeCoordinatorArgs) -> Result<()> {
    match args.command {
        NodeCoordinatorCommand::Start { config } => {
            info!("Starting BPI Node Coordinator");
            
            let coordinator = BpiNodeCoordinator::new().await?;
            info!("✅ BPI Node Coordinator started: {}", coordinator.coordinator_id);
            
            if let Some(config_path) = config {
                info!("Loading configuration from: {}", config_path);
                // TODO: Load and apply configuration
            }
            
            // Keep coordinator running
            info!("BPI Node Coordinator is running. Press Ctrl+C to stop.");
            tokio::signal::ctrl_c().await?;
            info!("Shutting down BPI Node Coordinator");
        },
        
        NodeCoordinatorCommand::Stop => {
            info!("Stopping BPI Node Coordinator");
            // TODO: Implement graceful shutdown
        },
        
        NodeCoordinatorCommand::Status => {
            info!("Getting BPI Node Coordinator status");
            let coordinator = BpiNodeCoordinator::new().await?;
            let nodes = coordinator.get_nodes_status().await?;
            
            println!("BPI Node Coordinator Status:");
            println!("Coordinator ID: {}", coordinator.coordinator_id);
            println!("Active Nodes: {}", nodes.len());
            
            for (node_id, node) in &nodes {
                println!("  Node: {} | Type: {:?} | Status: {:?} | Endpoint: {}", 
                         node_id, node.node_type, node.status, node.endpoint);
            }
        },
        
        NodeCoordinatorCommand::StartNode { node_type, endpoint, config } => {
            info!("Starting BPI node: {} at {}", node_type, endpoint);
            
            let coordinator = BpiNodeCoordinator::new().await?;
            
            // Parse node type and create appropriate BpiNodeType
            let bpi_node_type = parse_node_type(&node_type, config)?;
            let node_id = coordinator.start_node(bpi_node_type, endpoint).await?;
            
            println!("✅ BPI node started successfully: {}", node_id);
        },
        
        NodeCoordinatorCommand::StopNode { node_id } => {
            info!("Stopping BPI node: {}", node_id);
            
            let coordinator = BpiNodeCoordinator::new().await?;
            coordinator.stop_node(&node_id).await?;
            
            println!("✅ BPI node stopped: {}", node_id);
        },
        
        NodeCoordinatorCommand::ListNodeTypes => {
            println!("Available BPI Node Types:");
            println!("  • enc-cluster     - ENC Cluster with gateway and mempool integration");
            println!("  • oracle          - Oracle for price feeds and cross-chain data");
            println!("  • shadow-registry - Shadow registry for web2-web3 bridging");
            println!("  • pipeline-api    - Pipeline API with BISO integration");
            println!("  • storage         - Distributed storage with replication");
            println!("  • proof           - Proof generation for government compliance");
            println!("  • audit           - Audit trail and compliance hosting");
            println!("  • logbook         - Receipt storage from HTTP cage/docklock/ENC");
        },
        
        NodeCoordinatorCommand::Test => {
            info!("Running BPI Node Coordinator integration tests");
            println!("✅ BPI Node Coordinator test functionality would run here");
            println!("Note: Use 'bpi-core test-bpi-nodes' for actual testing");
        },
        
        NodeCoordinatorCommand::StartEncCluster { cluster_id, encryption, gateway, mempool_size, endpoint } => {
            let encryption_level = match encryption.as_str() {
                "standard" => EncryptionLevel::Standard,
                "military" => EncryptionLevel::Military,
                "quantum" => EncryptionLevel::Quantum,
                _ => return Err(anyhow!("Invalid encryption level: {}", encryption)),
            };
            
            let node_type = BpiNodeType::EncCluster {
                cluster_id,
                encryption_level,
                gateway_endpoint: gateway,
                mempool_size,
            };
            
            let coordinator = BpiNodeCoordinator::new().await?;
            let node_id = coordinator.start_node(node_type, endpoint).await?;
            
            println!("✅ ENC Cluster node started: {}", node_id);
        },
        
        NodeCoordinatorCommand::StartOracle { oracle_type, chains, frequency, reliability, endpoint } => {
            let oracle_type_enum = match oracle_type.as_str() {
                "price" => OracleType::PriceOracle,
                "data" => OracleType::DataOracle,
                "cross-chain" => OracleType::CrossChainOracle,
                "governance" => OracleType::GovernanceOracle,
                _ => return Err(anyhow!("Invalid oracle type: {}", oracle_type)),
            };
            
            let supported_chains: Vec<String> = chains.split(',').map(|s| s.trim().to_string()).collect();
            
            let node_type = BpiNodeType::Oracle {
                oracle_type: oracle_type_enum,
                supported_chains,
                update_frequency_ms: frequency,
                reliability_score: reliability,
            };
            
            let coordinator = BpiNodeCoordinator::new().await?;
            let node_id = coordinator.start_node(node_type, endpoint).await?;
            
            println!("✅ Oracle node started: {}", node_id);
        },
        
        NodeCoordinatorCommand::StartShadowRegistry { registry_type, web2_endpoints, web3_contracts, capacity, endpoint } => {
            let registry_type_enum = match registry_type.as_str() {
                "web2-bridge" => ShadowRegistryType::Web2Bridge,
                "privacy" => ShadowRegistryType::PrivacyRegistry,
                "compliance" => ShadowRegistryType::ComplianceRegistry,
                _ => return Err(anyhow!("Invalid registry type: {}", registry_type)),
            };
            
            let web2_eps = web2_endpoints.unwrap_or_default().split(',').map(|s| s.trim().to_string()).collect();
            let web3_contracts_list = web3_contracts.unwrap_or_default().split(',').map(|s| s.trim().to_string()).collect();
            
            let node_type = BpiNodeType::ShadowRegistry {
                registry_type: registry_type_enum,
                web2_endpoints: web2_eps,
                web3_contracts: web3_contracts_list,
                bridge_capacity: capacity,
            };
            
            let coordinator = BpiNodeCoordinator::new().await?;
            let node_id = coordinator.start_node(node_type, endpoint).await?;
            
            println!("✅ Shadow Registry node started: {}", node_id);
        },
        
        NodeCoordinatorCommand::StartPipelineApi { pipeline_id, biso_policies, traffic_rules, throughput, endpoint } => {
            let biso_policies_list = biso_policies.unwrap_or_default().split(',').map(|s| s.trim().to_string()).collect();
            let traffic_rules_list = traffic_rules.unwrap_or_default().split(',').map(|s| s.trim().to_string()).collect();
            
            let node_type = BpiNodeType::PipelineApi {
                pipeline_id,
                biso_policies: biso_policies_list,
                traffic_light_rules: traffic_rules_list,
                throughput_limit: throughput,
            };
            
            let coordinator = BpiNodeCoordinator::new().await?;
            let node_id = coordinator.start_node(node_type, endpoint).await?;
            
            println!("✅ Pipeline API node started: {}", node_id);
        },
        
        NodeCoordinatorCommand::StartStorage { storage_type, capacity, replication, encryption, endpoint } => {
            let storage_type_enum = match storage_type.as_str() {
                "distributed" => StorageType::Distributed,
                "high-performance" => StorageType::HighPerformance,
                "archive" => StorageType::Archive,
                _ => return Err(anyhow!("Invalid storage type: {}", storage_type)),
            };
            
            let node_type = BpiNodeType::Storage {
                storage_type: storage_type_enum,
                capacity_gb: capacity,
                replication_factor: replication,
                encryption_enabled: encryption,
            };
            
            let coordinator = BpiNodeCoordinator::new().await?;
            let node_id = coordinator.start_node(node_type, endpoint).await?;
            
            println!("✅ Storage node started: {}", node_id);
        },
        
        NodeCoordinatorCommand::StartProof { proof_type, compliance, retention, gov_endpoints, endpoint } => {
            let proof_type_enum = match proof_type.as_str() {
                "compliance" => ProofType::ComplianceProof,
                "identity" => ProofType::IdentityProof,
                "transaction" => ProofType::TransactionProof,
                _ => return Err(anyhow!("Invalid proof type: {}", proof_type)),
            };
            
            let compliance_level = match compliance.as_str() {
                "basic" => ComplianceLevel::Basic,
                "enhanced" => ComplianceLevel::Enhanced,
                "government" => ComplianceLevel::Government,
                _ => return Err(anyhow!("Invalid compliance level: {}", compliance)),
            };
            
            let gov_endpoints_list = gov_endpoints.unwrap_or_default().split(',').map(|s| s.trim().to_string()).collect();
            
            let node_type = BpiNodeType::Proof {
                proof_type: proof_type_enum,
                compliance_level,
                audit_retention_days: retention,
                government_endpoints: gov_endpoints_list,
            };
            
            let coordinator = BpiNodeCoordinator::new().await?;
            let node_id = coordinator.start_node(node_type, endpoint).await?;
            
            println!("✅ Proof node started: {}", node_id);
        },
        
        NodeCoordinatorCommand::StartAudit { scope, frameworks, frequency, reporting_endpoints, endpoint } => {
            let audit_scope = match scope.as_str() {
                "transaction" => AuditScope::Transaction,
                "node" => AuditScope::Node,
                "full-system" => AuditScope::FullSystem,
                _ => return Err(anyhow!("Invalid audit scope: {}", scope)),
            };
            
            let frameworks_list: Vec<String> = frameworks.split(',').map(|s| s.trim().to_string()).collect();
            let reporting_list = reporting_endpoints.unwrap_or_default().split(',').map(|s| s.trim().to_string()).collect();
            
            let node_type = BpiNodeType::Audit {
                audit_scope,
                compliance_frameworks: frameworks_list,
                audit_frequency_hours: frequency,
                reporting_endpoints: reporting_list,
            };
            
            let coordinator = BpiNodeCoordinator::new().await?;
            let node_id = coordinator.start_node(node_type, endpoint).await?;
            
            println!("✅ Audit node started: {}", node_id);
        },
        
        NodeCoordinatorCommand::StartLogbook { logbook_type, sources, storage_policy, retention_policy, endpoint } => {
            let logbook_type_enum = match logbook_type.as_str() {
                "auction" => LogbookType::AuctionRecords,
                "transaction" => LogbookType::TransactionRecords,
                "compliance" => LogbookType::ComplianceRecords,
                _ => return Err(anyhow!("Invalid logbook type: {}", logbook_type)),
            };
            
            let sources_list: Vec<String> = sources.split(',').map(|s| s.trim().to_string()).collect();
            
            let node_type = BpiNodeType::Logbook {
                logbook_type: logbook_type_enum,
                receipt_sources: sources_list,
                storage_policy,
                retention_policy,
            };
            
            let coordinator = BpiNodeCoordinator::new().await?;
            let node_id = coordinator.start_node(node_type, endpoint).await?;
            
            println!("✅ Logbook node started: {}", node_id);
        },
    }
    
    Ok(())
}

fn parse_node_type(node_type: &str, config: Option<String>) -> Result<BpiNodeType> {
    match node_type {
        "enc-cluster" => {
            // Parse config or use defaults
            Ok(BpiNodeType::EncCluster {
                cluster_id: "default-cluster".to_string(),
                encryption_level: EncryptionLevel::Military,
                gateway_endpoint: "http://localhost:8080".to_string(),
                mempool_size: 10000,
            })
        },
        "oracle" => {
            Ok(BpiNodeType::Oracle {
                oracle_type: OracleType::PriceOracle,
                supported_chains: vec!["BPI".to_string()],
                update_frequency_ms: 5000,
                reliability_score: 0.95,
            })
        },
        "shadow-registry" => {
            Ok(BpiNodeType::ShadowRegistry {
                registry_type: ShadowRegistryType::Web2Bridge,
                web2_endpoints: vec![],
                web3_contracts: vec![],
                bridge_capacity: 1000,
            })
        },
        "pipeline-api" => {
            Ok(BpiNodeType::PipelineApi {
                pipeline_id: "default-pipeline".to_string(),
                biso_policies: vec![],
                traffic_light_rules: vec![],
                throughput_limit: 5000,
            })
        },
        "storage" => {
            Ok(BpiNodeType::Storage {
                storage_type: StorageType::Distributed,
                capacity_gb: 1000,
                replication_factor: 3,
                encryption_enabled: true,
            })
        },
        "proof" => {
            Ok(BpiNodeType::Proof {
                proof_type: ProofType::ComplianceProof,
                compliance_level: ComplianceLevel::Enhanced,
                audit_retention_days: 2555,
                government_endpoints: vec![],
            })
        },
        "audit" => {
            Ok(BpiNodeType::Audit {
                audit_scope: AuditScope::FullSystem,
                compliance_frameworks: vec!["SOX".to_string(), "GDPR".to_string()],
                audit_frequency_hours: 24,
                reporting_endpoints: vec![],
            })
        },
        "logbook" => {
            Ok(BpiNodeType::Logbook {
                logbook_type: LogbookType::AuctionRecords,
                receipt_sources: vec!["http-cage".to_string(), "docklock".to_string(), "enc-cluster".to_string()],
                storage_policy: "replicated".to_string(),
                retention_policy: "7years".to_string(),
            })
        },
        _ => Err(anyhow!("Unknown node type: {}", node_type)),
    }
}
