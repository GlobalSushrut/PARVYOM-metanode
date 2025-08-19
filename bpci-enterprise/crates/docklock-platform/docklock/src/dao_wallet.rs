use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;


use bpi_enc::CanonicalCbor;
use crate::error::{DockLockError, DockLockResult};
use crate::event_stream::{Event, EventKind, CanonicalEventStream};
use crate::wallet::{CryptoKeypair, KeyType, WalletAddress, MicroserviceIdentity};

/// Proposal ID type
pub type ProposalId = Uuid;

/// Vote ID type
pub type VoteId = Uuid;

/// DAO member ID type
pub type MemberId = Uuid;

/// Voting power type
pub type VotingPower = u64;

/// Proposal status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProposalStatus {
    Draft,
    Active,
    Passed,
    Rejected,
    Executed,
    Cancelled,
}

/// Vote choice
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VoteChoice {
    Yes,
    No,
    Abstain,
}

/// Proposal types for DAO governance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalType {
    /// Resource allocation proposals
    ResourceAllocation {
        resource_type: String,
        amount: u64,
        recipient: WalletAddress,
        justification: String,
    },
    
    /// Security policy proposals
    SecurityPolicy {
        policy_name: String,
        policy_config: HashMap<String, String>,
        enforcement_level: String,
    },
    
    /// Container runtime proposals
    RuntimeUpgrade {
        component: String,
        version: String,
        migration_plan: String,
    },
    
    /// Governance parameter changes
    GovernanceParameter {
        parameter: String,
        old_value: String,
        new_value: String,
    },
    
    /// Member management
    MemberManagement {
        action: String, // "add", "remove", "update_power"
        member_id: MemberId,
        voting_power: Option<VotingPower>,
    },
    
    /// Custom proposals
    Custom {
        proposal_type: String,
        data: HashMap<String, String>,
    },
}

/// DAO governance proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    /// Unique proposal identifier
    pub id: ProposalId,
    
    /// Proposal title
    pub title: String,
    
    /// Detailed description
    pub description: String,
    
    /// Proposal type and data
    pub proposal_type: ProposalType,
    
    /// Proposer information
    pub proposer: MemberId,
    pub proposer_address: WalletAddress,
    
    /// Voting parameters
    pub voting_start: u64,
    pub voting_end: u64,
    pub quorum_required: VotingPower,
    pub approval_threshold: f64, // Percentage (0.0 to 1.0)
    
    /// Current status
    pub status: ProposalStatus,
    
    /// Vote tallies
    pub votes_yes: VotingPower,
    pub votes_no: VotingPower,
    pub votes_abstain: VotingPower,
    
    /// Metadata
    pub metadata: HashMap<String, String>,
    
    /// Creation and update timestamps
    pub created_at: u64,
    pub updated_at: u64,
}

impl Proposal {
    /// Create a new proposal
    pub fn new(
        title: String,
        description: String,
        proposal_type: ProposalType,
        proposer: MemberId,
        proposer_address: WalletAddress,
        voting_duration: u64,
        quorum_required: VotingPower,
        approval_threshold: f64,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            proposal_type,
            proposer,
            proposer_address,
            voting_start: now,
            voting_end: now + voting_duration,
            quorum_required,
            approval_threshold,
            status: ProposalStatus::Active,
            votes_yes: 0,
            votes_no: 0,
            votes_abstain: 0,
            metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Check if proposal is currently active for voting
    pub fn is_active(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        self.status == ProposalStatus::Active && now >= self.voting_start && now <= self.voting_end
    }
    
    /// Check if proposal has reached quorum
    pub fn has_quorum(&self) -> bool {
        let total_votes = self.votes_yes + self.votes_no + self.votes_abstain;
        total_votes >= self.quorum_required
    }
    
    /// Check if proposal has passed
    pub fn has_passed(&self) -> bool {
        if !self.has_quorum() {
            return false;
        }
        
        let total_decisive_votes = self.votes_yes + self.votes_no;
        if total_decisive_votes == 0 {
            return false;
        }
        
        let approval_rate = self.votes_yes as f64 / total_decisive_votes as f64;
        approval_rate >= self.approval_threshold
    }
    
    /// Update proposal status based on current state
    pub fn update_status(&mut self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        if self.status == ProposalStatus::Active && now > self.voting_end {
            if self.has_passed() {
                self.status = ProposalStatus::Passed;
            } else {
                self.status = ProposalStatus::Rejected;
            }
            self.updated_at = now;
        }
    }
    
    /// Get canonical encoding
    pub fn encode(&self) -> DockLockResult<Vec<u8>> {
        CanonicalCbor::encode(self)
            .map_err(|e| DockLockError::EncodingError(format!("Failed to encode proposal: {}", e)))
    }
}

/// DAO member vote record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    /// Unique vote identifier
    pub id: VoteId,
    
    /// Proposal being voted on
    pub proposal_id: ProposalId,
    
    /// Voter information
    pub voter: MemberId,
    pub voter_address: WalletAddress,
    
    /// Vote choice
    pub choice: VoteChoice,
    
    /// Voting power used
    pub voting_power: VotingPower,
    
    /// Vote justification/comment
    pub comment: Option<String>,
    
    /// Vote timestamp
    pub timestamp: u64,
    
    /// Cryptographic signature
    pub signature: Vec<u8>,
}

impl Vote {
    /// Create a new vote
    pub fn new(
        proposal_id: ProposalId,
        voter: MemberId,
        voter_address: WalletAddress,
        choice: VoteChoice,
        voting_power: VotingPower,
        comment: Option<String>,
        signature: Vec<u8>,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            id: Uuid::new_v4(),
            proposal_id,
            voter,
            voter_address,
            choice,
            voting_power,
            comment,
            timestamp: now,
            signature,
        }
    }
    
    /// Get canonical encoding for signature verification
    pub fn encode_for_signature(&self) -> DockLockResult<Vec<u8>> {
        let vote_data = (
            self.proposal_id.clone(),
            self.voter.clone(),
            format!("{:?}", self.choice),
            self.choice.clone(),
            self.voting_power,
            self.comment.clone(),
            self.timestamp,
        );
        
        CanonicalCbor::encode(&vote_data)
            .map_err(|e| DockLockError::EncodingError(format!("Failed to encode vote for signature: {}", e)))
    }
}

/// DAO member information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaoMember {
    /// Unique member identifier
    pub id: MemberId,
    
    /// Member wallet address
    pub address: WalletAddress,
    
    /// Member name/identifier
    pub name: String,
    
    /// Member role/description
    pub role: String,
    
    /// Voting power assigned to this member
    pub voting_power: VotingPower,
    
    /// Member reputation score
    pub reputation: u64,
    
    /// Member metadata
    pub metadata: HashMap<String, String>,
    
    /// Join timestamp
    pub joined_at: u64,
    
    /// Last activity timestamp
    pub last_active: u64,
}

impl DaoMember {
    /// Create a new DAO member
    pub fn new(
        address: WalletAddress,
        name: String,
        role: String,
        voting_power: VotingPower,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            id: Uuid::new_v4(),
            address,
            name,
            role,
            voting_power,
            reputation: 100, // Starting reputation
            metadata: HashMap::new(),
            joined_at: now,
            last_active: now,
        }
    }
    
    /// Update last activity timestamp
    pub fn update_activity(&mut self) {
        self.last_active = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

/// DAO wallet configuration
#[derive(Debug, Clone)]
pub struct DaoWalletConfig {
    /// DAO name
    pub name: String,
    
    /// DAO description
    pub description: String,
    
    /// Default voting duration (seconds)
    pub default_voting_duration: u64,
    
    /// Default quorum requirement
    pub default_quorum: VotingPower,
    
    /// Default approval threshold
    pub default_approval_threshold: f64,
    
    /// Maximum proposals per member per day
    pub max_proposals_per_day: u32,
    
    /// Enable governance logging
    pub enable_logging: bool,
}

impl Default for DaoWalletConfig {
    fn default() -> Self {
        Self {
            name: "DockLock DAO".to_string(),
            description: "Decentralized governance for container runtime decisions".to_string(),
            default_voting_duration: 7 * 24 * 3600, // 7 days
            default_quorum: 100, // Minimum voting power required
            default_approval_threshold: 0.6, // 60% approval required
            max_proposals_per_day: 5,
            enable_logging: true,
        }
    }
}

/// DAO wallet for decentralized governance
pub struct DaoWallet {
    /// DAO configuration
    config: DaoWalletConfig,
    
    /// DAO cryptographic keypair
    keypair: CryptoKeypair,
    
    /// DAO identity
    identity: MicroserviceIdentity,
    
    /// DAO members
    members: Arc<RwLock<HashMap<MemberId, DaoMember>>>,
    
    /// Member lookup by address
    member_lookup: Arc<RwLock<HashMap<WalletAddress, MemberId>>>,
    
    /// Active proposals
    proposals: Arc<RwLock<HashMap<ProposalId, Proposal>>>,
    
    /// Vote records
    votes: Arc<RwLock<HashMap<VoteId, Vote>>>,
    
    /// Proposal votes lookup
    proposal_votes: Arc<RwLock<HashMap<ProposalId, Vec<VoteId>>>>,
    
    /// Event stream for governance events
    event_stream: Arc<RwLock<CanonicalEventStream>>,
}

/// DAO wallet statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaoWalletStats {
    pub dao_address: WalletAddress,
    pub total_members: usize,
    pub total_voting_power: VotingPower,
    pub total_proposals: usize,
    pub active_proposals: usize,
    pub total_votes: usize,
    pub total_events: usize,
}

impl DaoWallet {
    /// Create a new DAO wallet
    pub async fn new(config: DaoWalletConfig) -> DockLockResult<Self> {
        let keypair = CryptoKeypair::generate(KeyType::Ed25519)?;
        
        let identity = MicroserviceIdentity::new(
            config.name.clone(),
            config.description.clone(),
            "docklock/dao:latest".to_string(),
            "1.0.0".to_string(),
            &keypair,
        );
        
        info!("Created DAO wallet '{}' with address: {}", config.name, identity.wallet_address());
        
        Ok(Self {
            config,
            keypair,
            identity,
            members: Arc::new(RwLock::new(HashMap::new())),
            member_lookup: Arc::new(RwLock::new(HashMap::new())),
            proposals: Arc::new(RwLock::new(HashMap::new())),
            votes: Arc::new(RwLock::new(HashMap::new())),
            proposal_votes: Arc::new(RwLock::new(HashMap::new())),
            event_stream: Arc::new(RwLock::new(CanonicalEventStream::default())),
        })
    }
    
    /// Get DAO address
    pub fn address(&self) -> WalletAddress {
        self.identity.wallet_address()
    }
    
    /// Get DAO identity
    pub fn identity(&self) -> &MicroserviceIdentity {
        &self.identity
    }
    
    /// Add a new DAO member
    pub async fn add_member(
        &self,
        address: WalletAddress,
        name: String,
        role: String,
        voting_power: VotingPower,
    ) -> DockLockResult<MemberId> {
        let member = DaoMember::new(address.clone(), name, role, voting_power);
        let member_id = member.id;
        
        // Add to members registry
        {
            let mut members = self.members.write().await;
            let mut lookup = self.member_lookup.write().await;
            
            members.insert(member_id, member.clone());
            lookup.insert(address.clone(), member_id);
        }
        
        // Log member addition event
        if self.config.enable_logging {
            let event = Event::new(
                member_id.as_u128(),
                None,
                0,
                EventKind::IdentityCreate,
                &CanonicalCbor::encode(&member)
                    .map_err(|e| DockLockError::EncodingError(format!("Failed to encode member: {}", e)))?,
            ).with_metadata("member_name".to_string(), member.name.clone())
             .with_metadata("member_address".to_string(), address)
             .with_metadata("voting_power".to_string(), voting_power.to_string());
            
            let mut stream = self.event_stream.write().await;
            stream.add_event(event)?;
        }
        
        info!("Added DAO member {} with voting power {}", member.name, voting_power);
        Ok(member_id)
    }
    
    /// Get a DAO member by ID
    pub async fn get_member(&self, member_id: &MemberId) -> Option<DaoMember> {
        let members = self.members.read().await;
        members.get(member_id).cloned()
    }
    
    /// Get a DAO member by address
    pub async fn get_member_by_address(&self, address: &WalletAddress) -> Option<DaoMember> {
        let lookup = self.member_lookup.read().await;
        if let Some(member_id) = lookup.get(address) {
            let members = self.members.read().await;
            members.get(member_id).cloned()
        } else {
            None
        }
    }
    
    /// List all DAO members
    pub async fn list_members(&self) -> Vec<DaoMember> {
        let members = self.members.read().await;
        members.values().cloned().collect()
    }
    
    /// Create a new proposal
    pub async fn create_proposal(
        &self,
        proposer_address: WalletAddress,
        title: String,
        description: String,
        proposal_type: ProposalType,
        voting_duration: Option<u64>,
        quorum_required: Option<VotingPower>,
        approval_threshold: Option<f64>,
    ) -> DockLockResult<ProposalId> {
        // Verify proposer is a DAO member
        let proposer = self.get_member_by_address(&proposer_address).await
            .ok_or_else(|| DockLockError::AccessDenied("Proposer is not a DAO member".to_string()))?;
        
        let proposal = Proposal::new(
            title,
            description,
            proposal_type,
            proposer.id,
            proposer_address.clone(),
            voting_duration.unwrap_or(self.config.default_voting_duration),
            quorum_required.unwrap_or(self.config.default_quorum),
            approval_threshold.unwrap_or(self.config.default_approval_threshold),
        );
        
        let proposal_id = proposal.id;
        
        // Add to proposals registry
        {
            let mut proposals = self.proposals.write().await;
            let mut proposal_votes = self.proposal_votes.write().await;
            
            proposals.insert(proposal_id, proposal.clone());
            proposal_votes.insert(proposal_id, Vec::new());
        }
        
        // Log proposal creation event
        if self.config.enable_logging {
            let event = Event::new(
                proposal_id.as_u128(),
                None,
                0,
                EventKind::ProposalCreate,
                &proposal.encode()?,
            ).with_metadata("proposal_title".to_string(), proposal.title.clone())
             .with_metadata("proposer_address".to_string(), proposer_address);
            
            let mut stream = self.event_stream.write().await;
            stream.add_event(event)?;
        }
        
        info!("Created proposal '{}' with ID {}", proposal.title, proposal_id);
        Ok(proposal_id)
    }
    
    /// Cast a vote on a proposal
    pub async fn vote(
        &self,
        voter_address: WalletAddress,
        proposal_id: ProposalId,
        choice: VoteChoice,
        comment: Option<String>,
    ) -> DockLockResult<VoteId> {
        // Verify voter is a DAO member
        let mut voter = self.get_member_by_address(&voter_address).await
            .ok_or_else(|| DockLockError::AccessDenied("Voter is not a DAO member".to_string()))?;
        
        // Get proposal and verify it's active
        let mut proposal = {
            let proposals = self.proposals.read().await;
            proposals.get(&proposal_id).cloned()
                .ok_or_else(|| DockLockError::NotFound("Proposal not found".to_string()))?
        };
        
        if !proposal.is_active() {
            return Err(DockLockError::InvalidOperation("Proposal is not active for voting".to_string()));
        }
        
        // Check if member has already voted
        let existing_votes = {
            let proposal_votes = self.proposal_votes.read().await;
            let votes = self.votes.read().await;
            
            if let Some(vote_ids) = proposal_votes.get(&proposal_id) {
                vote_ids.iter()
                    .filter_map(|vote_id| votes.get(vote_id))
                    .any(|vote| vote.voter == voter.id)
            } else {
                false
            }
        };
        
        if existing_votes {
            return Err(DockLockError::InvalidOperation("Member has already voted on this proposal".to_string()));
        }
        
        // Create vote data for signing
        let vote_data = (
            proposal_id.clone(),
            voter.id.clone(),
            voter_address.clone(),
            choice.clone(),
            voter.voting_power,
            comment.clone(),
        );
        
        let vote_bytes = CanonicalCbor::encode(&vote_data)
            .map_err(|e| DockLockError::EncodingError(format!("Failed to encode vote data: {}", e)))?;
        
        let signature = self.keypair.sign(&vote_bytes)?;
        
        // Create vote record
        let vote = Vote::new(
            proposal_id,
            voter.id,
            voter_address.clone(),
            choice.clone(),
            voter.voting_power,
            comment,
            signature,
        );
        
        let vote_id = vote.id;
        
        // Update proposal vote tallies
        match choice {
            VoteChoice::Yes => proposal.votes_yes += voter.voting_power,
            VoteChoice::No => proposal.votes_no += voter.voting_power,
            VoteChoice::Abstain => proposal.votes_abstain += voter.voting_power,
        }
        
        proposal.update_status();
        
        // Update member activity
        voter.update_activity();
        
        // Store vote and update proposal
        {
            let mut votes = self.votes.write().await;
            let mut proposal_votes = self.proposal_votes.write().await;
            let mut proposals = self.proposals.write().await;
            let mut members = self.members.write().await;
            
            votes.insert(vote_id, vote.clone());
            proposal_votes.entry(proposal_id).or_insert_with(Vec::new).push(vote_id);
            proposals.insert(proposal_id, proposal.clone());
            members.insert(voter.id, voter);
        }
        
        // Log vote event
        if self.config.enable_logging {
            let event = Event::new(
                vote_id.as_u128(),
                Some(proposal_id.as_u128()),
                0,
                EventKind::ProposalVote,
                &CanonicalCbor::encode(&vote)
                    .map_err(|e| DockLockError::EncodingError(format!("Failed to encode vote: {}", e)))?,
            ).with_metadata("voter_address".to_string(), voter_address.clone())
             .with_metadata("choice".to_string(), format!("{:?}", choice))
             .with_metadata("voting_power".to_string(), vote.voting_power.to_string());
            
            let mut stream = self.event_stream.write().await;
            stream.add_event(event)?;
        }
        
        info!("Vote cast by {} on proposal {}: {:?}", voter_address, proposal_id, choice);
        Ok(vote_id)
    }
    
    /// Get a proposal by ID
    pub async fn get_proposal(&self, proposal_id: &ProposalId) -> Option<Proposal> {
        let proposals = self.proposals.read().await;
        proposals.get(proposal_id).cloned()
    }
    
    /// List all proposals
    pub async fn list_proposals(&self) -> Vec<Proposal> {
        let proposals = self.proposals.read().await;
        proposals.values().cloned().collect()
    }
    
    /// Get votes for a proposal
    pub async fn get_proposal_votes(&self, proposal_id: &ProposalId) -> Vec<Vote> {
        let proposal_votes = self.proposal_votes.read().await;
        let votes = self.votes.read().await;
        
        if let Some(vote_ids) = proposal_votes.get(proposal_id) {
            vote_ids.iter()
                .filter_map(|vote_id| votes.get(vote_id))
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Execute a passed proposal
    pub async fn execute_proposal(&self, proposal_id: ProposalId) -> DockLockResult<String> {
        let mut proposal = {
            let proposals = self.proposals.read().await;
            proposals.get(&proposal_id).cloned()
                .ok_or_else(|| DockLockError::NotFound("Proposal not found".to_string()))?
        };
        
        if proposal.status != ProposalStatus::Passed {
            return Err(DockLockError::InvalidOperation("Proposal has not passed".to_string()));
        }
        
        // Mark as executed
        proposal.status = ProposalStatus::Executed;
        proposal.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Update proposal
        {
            let mut proposals = self.proposals.write().await;
            proposals.insert(proposal_id, proposal.clone());
        }
        
        // Log execution event
        if self.config.enable_logging {
            let event = Event::new(
                rand::random::<u128>(),
                Some(proposal_id.as_u128()),
                0,
                EventKind::ProposalExecute,
                &proposal.encode()?,
            ).with_metadata("proposal_title".to_string(), proposal.title.clone());
            
            let mut stream = self.event_stream.write().await;
            stream.add_event(event)?;
        }
        
        // In a real implementation, this would execute the actual proposal actions
        let execution_id = format!("exec_{}", Uuid::new_v4());
        info!("Executed proposal '{}' with execution ID: {}", proposal.title, execution_id);
        
        Ok(execution_id)
    }
    
    /// Get DAO statistics
    pub async fn get_dao_stats(&self) -> DaoWalletStats {
        let members = self.members.read().await;
        let proposals = self.proposals.read().await;
        let votes = self.votes.read().await;
        let stream = self.event_stream.read().await;
        
        let total_voting_power: VotingPower = members.values().map(|m| m.voting_power).sum();
        
        let active_proposals = proposals.values()
            .filter(|p| p.is_active())
            .count();
        
        DaoWalletStats {
            dao_address: self.address(),
            total_members: members.len(),
            total_voting_power,
            total_proposals: proposals.len(),
            active_proposals,
            total_votes: votes.len(),
            total_events: stream.event_count(),
        }
    }
}
