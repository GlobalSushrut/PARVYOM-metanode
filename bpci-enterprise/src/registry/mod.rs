use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

pub mod node_types;
pub mod identity;
pub mod authority;
pub mod registration;

pub use node_types::*;
pub use identity::*;
pub use authority::*;
pub use registration::*;

/// Enhanced BPCI Registry System
/// Supports comprehensive node registration, identity management, and authority systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpciRegistry {
    pub nodes: HashMap<String, NodeRegistration>,
    pub identities: HashMap<String, IdentityRecord>,
    pub authorities: HashMap<String, AuthorityRecord>,
    pub validator_set: ValidatorSet,
    pub miner_pool: MinerPool,
    pub notary_committee: NotaryCommittee,
    pub governance: GovernanceSystem,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl BpciRegistry {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            nodes: HashMap::new(),
            identities: HashMap::new(),
            authorities: HashMap::new(),
            validator_set: ValidatorSet::new(),
            miner_pool: MinerPool::new(),
            notary_committee: NotaryCommittee::new(),
            governance: GovernanceSystem::new(),
            created_at: now,
            last_updated: now,
        }
    }

    /// Register a new node in the registry
    pub async fn register_node(&mut self, registration: NodeRegistration) -> Result<String> {
        // Validate identity
        self.validate_identity(&registration.identity).await?;
        
        // Validate authority
        self.validate_authority(&registration.authority).await?;
        
        // Check node capabilities
        self.validate_capabilities(&registration.capabilities).await?;
        
        // Generate node ID
        let node_id = self.generate_node_id(&registration);
        
        // Add to appropriate pools based on node type
        match &registration.node_type {
            NodeType::BpciEnterprise { validator, miner, notary_committee, .. } => {
                if *validator {
                    self.validator_set.add_validator(&node_id, &registration).await?;
                }
                if *miner {
                    self.miner_pool.add_miner(&node_id, &registration).await?;
                }
                if *notary_committee {
                    self.notary_committee.add_member(&node_id, &registration).await?;
                }
            },
            NodeType::BpiCommunity { .. } => {
                // Community nodes participate in governance
                self.governance.add_participant(&node_id, &registration).await?;
            },
            NodeType::Hybrid { .. } => {
                // Hybrid nodes can participate in multiple systems
                self.validator_set.add_validator(&node_id, &registration).await?;
                self.governance.add_participant(&node_id, &registration).await?;
            },
        }
        
        // Store registration
        self.nodes.insert(node_id.clone(), registration);
        self.last_updated = Utc::now();
        
        Ok(node_id)
    }

    /// Lookup node by ID or other criteria
    pub async fn lookup_node(&self, query: &str, search_by: &str) -> Result<Option<NodeRegistration>> {
        match search_by {
            "id" => Ok(self.nodes.get(query).cloned()),
            "identity" => {
                for (_, node) in &self.nodes {
                    if node.identity.did == query {
                        return Ok(Some(node.clone()));
                    }
                }
                Ok(None)
            },
            "endpoint" => {
                for (_, node) in &self.nodes {
                    if node.endpoints.primary == query {
                        return Ok(Some(node.clone()));
                    }
                }
                Ok(None)
            },
            _ => Err(anyhow::anyhow!("Invalid search criteria: {}", search_by))
        }
    }

    /// List nodes with optional filtering
    pub async fn list_nodes(&self, 
        node_type: Option<&str>, 
        status: Option<&str>
    ) -> Result<Vec<(String, NodeRegistration)>> {
        let mut results = Vec::new();
        
        for (node_id, registration) in &self.nodes {
            // Filter by node type
            if let Some(filter_type) = node_type {
                let matches = match filter_type {
                    "bpi-community" => matches!(registration.node_type, NodeType::BpiCommunity { .. }),
                    "bpci-enterprise" => matches!(registration.node_type, NodeType::BpciEnterprise { .. }),
                    "hybrid" => matches!(registration.node_type, NodeType::Hybrid { .. }),
                    _ => false,
                };
                if !matches {
                    continue;
                }
            }
            
            // Filter by status
            if let Some(filter_status) = status {
                if registration.status.to_string() != filter_status {
                    continue;
                }
            }
            
            results.push((node_id.clone(), registration.clone()));
        }
        
        Ok(results)
    }

    /// Get registry statistics
    pub async fn get_stats(&self) -> Result<RegistryStats> {
        let total_nodes = self.nodes.len();
        let mut bpi_community = 0;
        let mut bpci_enterprise = 0;
        let mut hybrid = 0;
        let mut active_validators = 0;
        let mut active_miners = 0;
        let mut notary_members = 0;

        for (_, registration) in &self.nodes {
            match &registration.node_type {
                NodeType::BpiCommunity { .. } => bpi_community += 1,
                NodeType::BpciEnterprise { validator, miner, notary_committee, .. } => {
                    bpci_enterprise += 1;
                    if *validator { active_validators += 1; }
                    if *miner { active_miners += 1; }
                    if *notary_committee { notary_members += 1; }
                },
                NodeType::Hybrid { .. } => {
                    hybrid += 1;
                    active_validators += 1; // Hybrid nodes can be validators
                },
            }
        }

        Ok(RegistryStats {
            total_nodes,
            bpi_community,
            bpci_enterprise,
            hybrid,
            active_validators,
            active_miners,
            notary_members,
            governance_participants: self.governance.participants.len(),
            created_at: self.created_at,
            last_updated: self.last_updated,
        })
    }

    fn generate_node_id(&self, registration: &NodeRegistration) -> String {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(&registration.identity.did);
        hasher.update(&registration.endpoints.primary);
        hasher.update(&Utc::now().timestamp().to_string());
        
        let hash = hasher.finalize();
        format!("node_{}", hex::encode(&hash[..8]))
    }

    async fn validate_identity(&self, identity: &IdentityProof) -> Result<()> {
        // Validate D-Adhaar identity
        if identity.dadhaar.is_none() {
            return Err(anyhow::anyhow!("D-Adhaar identity required"));
        }
        
        // Additional identity validation logic
        Ok(())
    }

    async fn validate_authority(&self, authority: &AuthorityLevel) -> Result<()> {
        // Validate authority level requirements
        match authority {
            AuthorityLevel::Bank { kyc_verified, aml_compliant, .. } => {
                if !kyc_verified || !aml_compliant {
                    return Err(anyhow::anyhow!("Bank authority requires KYC and AML compliance"));
                }
            },
            _ => {} // Community and Hybrid validation
        }
        
        Ok(())
    }

    async fn validate_capabilities(&self, capabilities: &[NodeCapability]) -> Result<()> {
        // Validate that node has required capabilities for its type
        if capabilities.is_empty() {
            return Err(anyhow::anyhow!("Node must have at least one capability"));
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryStats {
    pub total_nodes: usize,
    pub bpi_community: usize,
    pub bpci_enterprise: usize,
    pub hybrid: usize,
    pub active_validators: usize,
    pub active_miners: usize,
    pub notary_members: usize,
    pub governance_participants: usize,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorSet {
    pub validators: HashMap<String, ValidatorInfo>,
    pub total_stake: u64,
}

impl ValidatorSet {
    pub fn new() -> Self {
        Self {
            validators: HashMap::new(),
            total_stake: 0,
        }
    }

    pub async fn add_validator(&mut self, node_id: &str, registration: &NodeRegistration) -> Result<()> {
        if let Some(stake) = registration.stake {
            let validator_info = ValidatorInfo {
                node_id: node_id.to_string(),
                stake_amount: stake,
                commission_rate: 0.05, // Default 5%
                status: ValidatorStatus::Active,
                joined_at: Utc::now(),
            };
            
            self.validators.insert(node_id.to_string(), validator_info);
            self.total_stake += stake;
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorInfo {
    pub node_id: String,
    pub stake_amount: u64,
    pub commission_rate: f64,
    pub status: ValidatorStatus,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidatorStatus {
    Active,
    Inactive,
    Slashed,
    Jailed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerPool {
    pub miners: HashMap<String, MinerInfo>,
    pub total_hashpower: u64,
}

impl MinerPool {
    pub fn new() -> Self {
        Self {
            miners: HashMap::new(),
            total_hashpower: 0,
        }
    }

    pub async fn add_miner(&mut self, node_id: &str, registration: &NodeRegistration) -> Result<()> {
        let miner_info = MinerInfo {
            node_id: node_id.to_string(),
            hashpower: 1000, // Default hashpower
            status: MinerStatus::Active,
            joined_at: Utc::now(),
        };
        
        self.miners.insert(node_id.to_string(), miner_info);
        self.total_hashpower += 1000;
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerInfo {
    pub node_id: String,
    pub hashpower: u64,
    pub status: MinerStatus,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MinerStatus {
    Active,
    Inactive,
    Maintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotaryCommittee {
    pub members: HashMap<String, NotaryMember>,
    pub quorum_threshold: usize,
}

impl NotaryCommittee {
    pub fn new() -> Self {
        Self {
            members: HashMap::new(),
            quorum_threshold: 3, // Minimum 3 members for quorum
        }
    }

    pub async fn add_member(&mut self, node_id: &str, registration: &NodeRegistration) -> Result<()> {
        let member = NotaryMember {
            node_id: node_id.to_string(),
            reputation: ReputationScore::new(),
            status: NotaryStatus::Active,
            joined_at: Utc::now(),
        };
        
        self.members.insert(node_id.to_string(), member);
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotaryMember {
    pub node_id: String,
    pub reputation: ReputationScore,
    pub status: NotaryStatus,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotaryStatus {
    Active,
    Inactive,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceSystem {
    pub participants: HashMap<String, GovernanceParticipant>,
    pub proposals: HashMap<String, Proposal>,
    pub treasury: TreasuryInfo,
}

impl GovernanceSystem {
    pub fn new() -> Self {
        Self {
            participants: HashMap::new(),
            proposals: HashMap::new(),
            treasury: TreasuryInfo::new(),
        }
    }

    pub async fn add_participant(&mut self, node_id: &str, registration: &NodeRegistration) -> Result<()> {
        let participant = GovernanceParticipant {
            node_id: node_id.to_string(),
            voting_power: self.calculate_voting_power(registration),
            proposals_created: 0,
            votes_cast: 0,
            joined_at: Utc::now(),
        };
        
        self.participants.insert(node_id.to_string(), participant);
        
        Ok(())
    }

    fn calculate_voting_power(&self, registration: &NodeRegistration) -> u64 {
        match &registration.node_type {
            NodeType::BpiCommunity { .. } => 1,
            NodeType::BpciEnterprise { .. } => registration.stake.unwrap_or(0) / 1000,
            NodeType::Hybrid { .. } => registration.stake.unwrap_or(0) / 500, // Hybrid gets bonus
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceParticipant {
    pub node_id: String,
    pub voting_power: u64,
    pub proposals_created: u32,
    pub votes_cast: u32,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub proposal_type: ProposalType,
    pub status: ProposalStatus,
    pub votes_for: u64,
    pub votes_against: u64,
    pub created_at: DateTime<Utc>,
    pub voting_ends_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalType {
    Treasury,
    Protocol,
    Governance,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryInfo {
    pub total_balance: u64,
    pub reserved_balance: u64,
    pub available_balance: u64,
}

impl TreasuryInfo {
    pub fn new() -> Self {
        Self {
            total_balance: 0,
            reserved_balance: 0,
            available_balance: 0,
        }
    }
}
