pub mod types;
pub mod node;
pub mod config;
pub mod chain;
pub mod enterprise;
pub mod docklock;
pub mod stubs;

// Re-export command types
pub use types::*;

use clap::Subcommand;

// System operations
#[derive(Subcommand)]
pub enum SystemCommands {
    /// Show system information
    Info,
    /// Check system requirements
    Requirements,
    /// Show system status
    Status,
    /// Show system resources
    Resources,
    /// Show system version
    Version,
    /// Show system dependencies
    Dependencies,
    /// Show system configuration
    Configuration,
    /// Show system environment
    Environment,
}

// Configuration management
#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Show current configuration
    Show,
    /// Set configuration value
    Set { key: String, value: String },
    /// Get configuration value
    Get { key: String },
    /// Reset configuration to defaults
    Reset,
    /// Validate configuration
    Validate,
    /// Export configuration
    Export { path: String },
    /// Import configuration
    Import { path: String },
    /// Generate sample configuration
    Generate,
}

// Chain operations
#[derive(Subcommand)]
pub enum ChainCommands {
    /// Show chain information
    Info,
    /// Show chain status
    Status,
    /// Show chain statistics
    Stats,
    /// Show chain height
    Height,
    /// Show chain head
    Head,
    /// Show chain genesis
    Genesis,
    /// Sync chain
    Sync,
    /// Reset chain
    Reset,
    /// Export chain
    Export { path: String },
    /// Import chain
    Import { path: String },
}

// Block operations
#[derive(Subcommand)]
pub enum BlockCommands {
    /// Get block by hash or height
    Get { identifier: String },
    /// List recent blocks
    List { count: Option<u64> },
    /// Show block header
    Header { identifier: String },
    /// Show block transactions
    Transactions { identifier: String },
    /// Show block receipts
    Receipts { identifier: String },
    /// Validate block
    Validate { identifier: String },
    /// Search blocks
    Search { query: String },
    /// Export block
    Export { identifier: String, path: String },
}

// Transaction operations
#[derive(Subcommand)]
pub enum TransactionCommands {
    /// Get transaction by hash
    Get { hash: String },
    /// Send transaction
    Send { to: String, amount: String },
    /// List pending transactions
    Pending,
    /// Show transaction receipt
    Receipt { hash: String },
    /// Validate transaction
    Validate { hash: String },
    /// Search transactions
    Search { query: String },
    /// Export transaction
    Export { hash: String, path: String },
    /// Simulate transaction
    Simulate { data: String },
}

// Validator operations
#[derive(Subcommand)]
pub enum ValidatorCommands {
    /// List validators
    List,
    /// Show validator info
    Info { address: String },
    /// Register as validator
    Register,
    /// Deregister validator
    Deregister,
    /// Show validator performance
    Performance { address: String },
    /// Show validator rewards
    Rewards { address: String },
    /// Show validator slashing
    Slashing { address: String },
    /// Update validator info
    Update,
}

// Consensus operations
#[derive(Subcommand)]
pub enum ConsensusCommands {
    /// Show consensus status
    Status,
    /// Show consensus participants
    Participants,
    /// Show consensus rounds
    Rounds,
    /// Show consensus metrics
    Metrics,
    /// Show consensus configuration
    Config,
    /// Show consensus history
    History,
    /// Force consensus round
    Force,
    /// Reset consensus
    Reset,
}

// Proof of History operations
#[derive(Subcommand)]
pub enum PohCommands {
    /// Show PoH status
    Status,
    /// Show PoH chain
    Chain,
    /// Show PoH metrics
    Metrics,
    /// Verify PoH sequence
    Verify { start: Option<u64>, end: Option<u64> },
    /// Show PoH configuration
    Config,
    /// Reset PoH chain
    Reset,
    /// Export PoH data
    Export { path: String },
}

// Mempool operations
#[derive(Subcommand)]
pub enum MempoolCommands {
    /// Show mempool status
    Status,
    /// List mempool transactions
    List,
    /// Show mempool metrics
    Metrics,
    /// Clear mempool
    Clear,
    /// Show mempool configuration
    Config,
    /// Flush mempool
    Flush,
    /// Show mempool statistics
    Stats,
}

// Light client operations
#[derive(Subcommand)]
pub enum LightClientCommands {
    /// Start light client
    Start,
    /// Stop light client
    Stop,
    /// Show light client status
    Status,
    /// Sync light client
    Sync,
    /// Show light client peers
    Peers,
    /// Show light client headers
    Headers,
    /// Verify light client data
    Verify,
    /// Reset light client
    Reset,
}

// Enterprise operations
#[derive(Subcommand)]
pub enum EnterpriseCommands {
    /// Deploy enterprise infrastructure
    Deploy,
    /// Show enterprise status
    Status,
    /// Manage enterprise users
    #[command(subcommand)]
    Users(EnterpriseUserCommands),
    /// Manage enterprise policies
    #[command(subcommand)]
    Policies(EnterprisePolicyCommands),
    /// Enterprise monitoring
    #[command(subcommand)]
    Monitor(EnterpriseMonitorCommands),
    /// Enterprise backup
    #[command(subcommand)]
    Backup(EnterpriseBackupCommands),
}

#[derive(Subcommand)]
pub enum EnterpriseUserCommands {
    List,
    Add { username: String },
    Remove { username: String },
    Update { username: String },
    Permissions { username: String },
}

#[derive(Subcommand)]
pub enum EnterprisePolicyCommands {
    List,
    Create { name: String },
    Delete { name: String },
    Apply { name: String },
    Validate { name: String },
}

#[derive(Subcommand)]
pub enum EnterpriseMonitorCommands {
    Dashboard,
    Alerts,
    Reports,
    Metrics,
}

#[derive(Subcommand)]
pub enum EnterpriseBackupCommands {
    Create,
    Restore { backup_id: String },
    List,
    Delete { backup_id: String },
}

// DockLock operations
#[derive(Subcommand)]
pub enum DocklockCommands {
    /// Deploy DockLock container
    Deploy { image: String },
    /// List DockLock containers
    List,
    /// Show container status
    Status { container_id: String },
    /// Stop container
    Stop { container_id: String },
    /// Remove container
    Remove { container_id: String },
    /// Show container logs
    Logs { container_id: String },
    /// Execute command in container
    Exec { container_id: String, command: String },
    /// Show container metrics
    Metrics { container_id: String },
    /// Show DockLock configuration
    Config,
    /// Manage DockLock policies
    #[command(subcommand)]
    Policy(DocklockPolicyCommands),
    /// Manage DockLock security
    #[command(subcommand)]
    Security(DocklockSecurityCommands),
}

#[derive(Subcommand)]
pub enum DocklockPolicyCommands {
    List,
    Create { name: String },
    Apply { name: String, container_id: String },
    Remove { name: String },
}

#[derive(Subcommand)]
pub enum DocklockSecurityCommands {
    Scan { container_id: String },
    Audit { container_id: String },
    Compliance { container_id: String },
}

// ENC cluster operations
#[derive(Subcommand)]
pub enum EncCommands {
    /// Deploy ENC cluster
    Deploy,
    /// Show cluster status
    Status,
    /// List cluster nodes
    Nodes,
    /// Add node to cluster
    AddNode { node_id: String },
    /// Remove node from cluster
    RemoveNode { node_id: String },
    /// Show cluster metrics
    Metrics,
    /// Show cluster configuration
    Config,
    /// Scale cluster
    Scale { replicas: u32 },
    /// Manage cluster workloads
    #[command(subcommand)]
    Workload(EncWorkloadCommands),
    /// Manage cluster networking
    #[command(subcommand)]
    Network(EncNetworkCommands),
}

#[derive(Subcommand)]
pub enum EncWorkloadCommands {
    List,
    Deploy { name: String, image: String },
    Scale { name: String, replicas: u32 },
    Delete { name: String },
    Logs { name: String },
}

#[derive(Subcommand)]
pub enum EncNetworkCommands {
    Status,
    Policies,
    Routes,
    Security,
}

// Continue with remaining command definitions...
// (This is getting quite long, so I'll continue in the next file)
