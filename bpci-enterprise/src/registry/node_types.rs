use serde::{Deserialize, Serialize};
use std::fmt;

/// Comprehensive node types for the BPCI Registry System
/// Supports BPI Community, BPCI Enterprise, and Hybrid nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    /// BPI Community Nodes - App hosting maintained by BPI Metanode core installer
    BpiCommunity {
        /// Supports application hosting
        app_hosting: bool,
        /// Participates in community governance
        community_governance: bool,
        /// Maximum number of apps that can be hosted
        max_apps: Option<u32>,
        /// Supported app types (docker, wasm, native)
        supported_app_types: Vec<AppType>,
    },
    
    /// BPCI Enterprise Nodes - Validators/maintainers with enhanced security
    BpciEnterprise {
        /// Acts as a validator in consensus
        validator: bool,
        /// Participates in Proof-of-Execution mining
        miner: bool,
        /// Member of the notary committee
        notary_committee: bool,
        /// Banking compliance features enabled
        banking_compliance: bool,
        /// Enhanced security features
        enhanced_security: SecurityLevel,
        /// Regulatory compliance flags
        regulatory_compliance: Vec<ComplianceType>,
    },
    
    /// Hybrid Nodes - Bank-sponsored but community-operated
    Hybrid {
        /// Sponsored by a banking institution
        bank_sponsored: bool,
        /// Operated by community members
        community_operated: bool,
        /// Has dual authority (bank + community)
        dual_authority: bool,
        /// Bank sponsor information
        bank_sponsor: Option<BankSponsor>,
        /// Community operator information
        community_operator: Option<CommunityOperator>,
    },
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeType::BpiCommunity { .. } => write!(f, "BPI Community"),
            NodeType::BpciEnterprise { .. } => write!(f, "BPCI Enterprise"),
            NodeType::Hybrid { .. } => write!(f, "Hybrid"),
        }
    }
}

/// Application types supported by nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppType {
    /// Docker containerized applications
    Docker,
    /// WebAssembly applications
    Wasm,
    /// Native binary applications
    Native,
    /// Static web applications
    Static,
    /// Serverless functions
    Serverless,
}

/// Security levels for enterprise nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// Standard security features
    Standard,
    /// Enhanced security with additional monitoring
    Enhanced,
    /// Military-grade security with quantum resistance
    MilitaryGrade,
    /// Banking-grade security with regulatory compliance
    BankingGrade,
}

/// Compliance types for regulatory requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceType {
    /// Know Your Customer
    Kyc,
    /// Anti-Money Laundering
    Aml,
    /// General Data Protection Regulation
    Gdpr,
    /// Health Insurance Portability and Accountability Act
    Hipaa,
    /// Payment Card Industry Data Security Standard
    PciDss,
    /// Sarbanes-Oxley Act
    Sox,
    /// Securities and Exchange Commission
    Sec,
    /// Commodity Futures Trading Commission
    Cftc,
}

/// Bank sponsor information for hybrid nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankSponsor {
    /// Bank name
    pub name: String,
    /// Bank identifier (routing number, SWIFT code, etc.)
    pub identifier: String,
    /// Sponsorship level (bronze, silver, gold, platinum)
    pub sponsorship_level: SponsorshipLevel,
    /// Regulatory approvals
    pub regulatory_approvals: Vec<String>,
    /// Contact information
    pub contact: ContactInfo,
}

/// Community operator information for hybrid nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityOperator {
    /// Operator name or handle
    pub name: String,
    /// Community reputation score
    pub reputation: u32,
    /// Years of experience
    pub experience_years: u32,
    /// Specializations
    pub specializations: Vec<String>,
    /// Community vouchers
    pub vouchers: Vec<CommunityVoucher>,
}

/// Sponsorship levels for bank-sponsored nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SponsorshipLevel {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

/// Community voucher for reputation building
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityVoucher {
    /// Voucher from another community member
    pub voucher_node_id: String,
    /// Voucher strength (1-10)
    pub strength: u8,
    /// Voucher reason
    pub reason: String,
    /// Timestamp of voucher
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Node capabilities that define what services a node can provide
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeCapability {
    /// Application hosting capability
    AppHosting {
        max_concurrent_apps: u32,
        supported_runtimes: Vec<Runtime>,
        resource_limits: ResourceLimits,
    },
    
    /// Validator capability for consensus participation
    Validator {
        max_stake: u64,
        commission_rate: f64,
        slashing_conditions: SlashingConditions,
    },
    
    /// Mining capability for Proof-of-Execution
    Mining {
        hashpower: u64,
        supported_algorithms: Vec<MiningAlgorithm>,
        pool_participation: bool,
    },
    
    /// Notary capability for transaction verification
    Notary {
        verification_types: Vec<VerificationType>,
        throughput_capacity: u32,
        reputation_threshold: u32,
    },
    
    /// Storage capability for distributed data
    Storage {
        capacity_gb: u64,
        redundancy_level: u8,
        encryption_level: EncryptionLevel,
    },
    
    /// Networking capability for mesh participation
    Networking {
        bandwidth_mbps: u32,
        latency_ms: u32,
        reliability_percentage: f64,
    },
    
    /// Governance capability for DAO participation
    Governance {
        voting_power: u64,
        proposal_creation: bool,
        treasury_access: bool,
    },
}

/// Runtime environments supported by nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Runtime {
    Docker,
    Wasm,
    Native,
    Kubernetes,
    Serverless,
}

/// Resource limits for application hosting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub storage_gb: u64,
    pub network_bandwidth_mbps: u32,
}

/// Slashing conditions for validators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlashingConditions {
    pub double_signing: SlashingPenalty,
    pub downtime: SlashingPenalty,
    pub invalid_block: SlashingPenalty,
}

/// Slashing penalty configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlashingPenalty {
    pub percentage: f64,
    pub jail_duration_hours: u32,
}

/// Mining algorithms supported
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MiningAlgorithm {
    ProofOfExecution,
    ProofOfStake,
    ProofOfWork,
    ProofOfHistory,
}

/// Verification types for notary services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationType {
    Transaction,
    Identity,
    Compliance,
    Audit,
    Signature,
}

/// Encryption levels for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionLevel {
    Standard,
    Enhanced,
    MilitaryGrade,
    QuantumResistant,
}

/// Network endpoints for node communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEndpoints {
    /// Primary endpoint for node communication
    pub primary: String,
    /// Backup endpoints for redundancy
    pub backup: Vec<String>,
    /// API endpoint for external access
    pub api: Option<String>,
    /// WebSocket endpoint for real-time communication
    pub websocket: Option<String>,
    /// P2P endpoint for mesh networking
    pub p2p: Option<String>,
}

/// Contact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub website: Option<String>,
}

/// Node status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeStatus {
    /// Node is active and operational
    Active,
    /// Node is inactive but can be activated
    Inactive,
    /// Node is under maintenance
    Maintenance,
    /// Node is suspended due to violations
    Suspended,
    /// Node is permanently banned
    Banned,
    /// Node is being onboarded
    Onboarding,
}

impl fmt::Display for NodeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeStatus::Active => write!(f, "active"),
            NodeStatus::Inactive => write!(f, "inactive"),
            NodeStatus::Maintenance => write!(f, "maintenance"),
            NodeStatus::Suspended => write!(f, "suspended"),
            NodeStatus::Banned => write!(f, "banned"),
            NodeStatus::Onboarding => write!(f, "onboarding"),
        }
    }
}

/// Reputation scoring system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationScore {
    /// Overall reputation score (0-1000)
    pub score: u32,
    /// Uptime percentage
    pub uptime: f64,
    /// Number of successful operations
    pub successful_operations: u64,
    /// Number of failed operations
    pub failed_operations: u64,
    /// Community vouchers received
    pub community_vouchers: u32,
    /// Penalties received
    pub penalties: u32,
    /// Last updated timestamp
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl ReputationScore {
    pub fn new() -> Self {
        Self {
            score: 100, // Start with base score
            uptime: 100.0,
            successful_operations: 0,
            failed_operations: 0,
            community_vouchers: 0,
            penalties: 0,
            last_updated: chrono::Utc::now(),
        }
    }

    /// Calculate reputation score based on various factors
    pub fn calculate_score(&mut self) {
        let base_score = 100.0;
        let uptime_factor = self.uptime / 100.0;
        let success_rate = if self.successful_operations + self.failed_operations > 0 {
            self.successful_operations as f64 / (self.successful_operations + self.failed_operations) as f64
        } else {
            1.0
        };
        let voucher_bonus = (self.community_vouchers as f64 * 10.0).min(200.0);
        let penalty_malus = self.penalties as f64 * 50.0;

        let calculated_score = (base_score * uptime_factor * success_rate + voucher_bonus - penalty_malus).max(0.0);
        self.score = (calculated_score as u32).min(1000);
        self.last_updated = chrono::Utc::now();
    }

    /// Add a successful operation
    pub fn add_success(&mut self) {
        self.successful_operations += 1;
        self.calculate_score();
    }

    /// Add a failed operation
    pub fn add_failure(&mut self) {
        self.failed_operations += 1;
        self.calculate_score();
    }

    /// Add a community voucher
    pub fn add_voucher(&mut self) {
        self.community_vouchers += 1;
        self.calculate_score();
    }

    /// Add a penalty
    pub fn add_penalty(&mut self) {
        self.penalties += 1;
        self.calculate_score();
    }

    /// Update uptime percentage
    pub fn update_uptime(&mut self, uptime: f64) {
        self.uptime = uptime.max(0.0).min(100.0);
        self.calculate_score();
    }
}
