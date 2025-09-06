// Node types and capabilities for the mining registry bridge
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeCapability {
    Validator {
        max_stake: u64,
        commission_rate: f64,
        slashing_conditions: SlashingConditions,
    },
    AppHosting {
        max_apps: u32,
        supported_types: Vec<String>,
        max_containers: u32,
        resource_limits: ResourceLimits,
    },
    NotaryServices {
        max_documents: u32,
        verification_types: Vec<VerificationType>,
    },
    Notary {
        verification_types: Vec<VerificationType>,
        throughput_capacity: u32,
        reputation_threshold: u32,
    },
    /// Auditing capability for logbook nodes
    Auditing {
        audit_types: Vec<String>,
        retention_period_days: u32,
        compliance_standards: Vec<String>,
    },
    /// Governance capability for roundtable nodes
    Governance {
        voting_power: u32,
        proposal_threshold: u32,
        quorum_requirement: u32,
        governance_types: Vec<String>,
    },
    /// Wallet scaling capability for box block nodes
    WalletScaling {
        max_wallet_instances: u32,
        auto_scaling_enabled: bool,
        load_balancing: bool,
        replication_factor: u32,
    },
}

/// Validator Node for consensus and validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorNode {
    pub node_id: String,
    pub stake_amount: u64,
    pub commission_rate: f64,
    pub uptime_percentage: f64,
    pub slashing_history: Vec<SlashingEvent>,
    pub validator_key: String,
    pub status: ValidatorStatus,
}

/// Miner Node for proof-of-execution mining
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerNode {
    pub node_id: String,
    pub mining_power: f64,
    pub blocks_mined: u64,
    pub mining_rewards: u64,
    pub hardware_specs: HardwareSpecs,
    pub mining_pool: Option<String>,
    pub status: MinerStatus,
}

/// Notary Node for document verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotaryNode {
    pub node_id: String,
    pub documents_verified: u64,
    pub verification_accuracy: f64,
    pub notary_license: String,
    pub jurisdiction: String,
    pub specializations: Vec<NotarySpecialization>,
    pub status: NotaryStatus,
}

/// Validator status enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidatorStatus {
    Active,
    Inactive,
    Slashed,
    Jailed,
}

/// Miner status enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MinerStatus {
    Mining,
    Idle,
    Maintenance,
    Offline,
}

/// Notary status enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotaryStatus {
    Available,
    Busy,
    Offline,
    Suspended,
}

/// Slashing event record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlashingEvent {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub reason: String,
    pub penalty_amount: u64,
    pub evidence_hash: String,
}

/// Hardware specifications for miners
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareSpecs {
    pub cpu_cores: u32,
    pub ram_gb: u32,
    pub storage_gb: u32,
    pub gpu_count: u32,
    pub network_bandwidth_mbps: u32,
}

/// Notary specialization areas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotarySpecialization {
    RealEstate,
    Financial,
    Legal,
    Medical,
    Educational,
    Corporate,
    International,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlashingConditions {
    pub double_signing: SlashingPenalty,
    pub downtime: SlashingPenalty,
    pub downtime_threshold: u64,
    pub double_sign_penalty: SlashingPenalty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlashingPenalty {
    pub percentage: f64,
    pub jail_duration_hours: u64,
    pub duration_days: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub storage_gb: u32,
    pub network_mbps: u32,
    pub bandwidth_mbps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceType {
    Basic,
    Enhanced,
    Enterprise,
    KYC,
    AML,
    Kyc,
    Aml,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationType {
    Document,
    Identity,
    Financial,
    Legal,
    Transaction,
}

// Authority module types
pub mod authority {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum CommunityRole {
        Member,
        Validator,
        Moderator,
        Notary,
        Developer,
        Operator,
        Auditor,
        Governance,
    }
}
