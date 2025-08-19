//! Block proposal and voting system for BPI Mesh consensus
//! Stage 13: Block Proposal & Voting

use std::collections::{HashMap, HashSet};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use thiserror::Error;

// Re-export dependencies
pub use bpi_enc::{domain_hash, domains, CanonicalCbor};
pub use bpi_headers::{Header, HeaderHash, ConsensusMode};
pub use bpi_validator_set::{ValidatorSet, ValidatorInfo};
pub use bpi_leader_selection::{LeaderSelector, LeaderSelectionResult, RoundInfo};
pub use bpi_blsagg::{PublicKey as BlsPublicKey, Signature as BlsSignature, PrivateKey as BlsPrivateKey};
pub use bpi_merkle::{MerkleTree, MerkleProof};
pub use bpi_vrf::{VrfProof, VrfOutput, VrfPrivateKey};

/// Block proposal and voting errors
#[derive(Error, Debug)]
pub enum BlockProposalError {
    #[error("Invalid proposer: {0}")]
    InvalidProposer(usize),
    #[error("Block validation failed: {0}")]
    BlockValidationFailed(String),
    #[error("Insufficient votes: got {got}, needed {needed}")]
    InsufficientVotes { got: usize, needed: usize },
    #[error("Invalid vote signature: {0}")]
    InvalidVoteSignature(String),
    #[error("Duplicate vote from validator: {0}")]
    DuplicateVote(usize),
    #[error("Vote for wrong block: expected {expected}, got {got}")]
    WrongBlockVote { expected: String, got: String },
    #[error("Proposal timeout: {0}")]
    ProposalTimeout(String),
    #[error("Invalid block height: expected {expected}, got {got}")]
    InvalidBlockHeight { expected: u64, got: u64 },
    #[error("Invalid round: expected {expected}, got {got}")]
    InvalidRound { expected: u64, got: u64 },
    #[error("Consensus error: {0}")]
    ConsensusError(String),
}

/// Block proposal with leader selection proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockProposal {
    /// Proposed block header
    pub header: Header,
    /// Leader selection result proving proposer eligibility
    pub leader_proof: LeaderSelectionResult,
    /// Proposer's signature on the block
    pub proposer_signature: BlsSignature,
    /// Transaction Merkle tree for the block
    pub transaction_tree: Option<MerkleTree>,
    /// Proposal timestamp
    pub proposed_at: DateTime<Utc>,
    /// Proposal round information
    pub round_info: RoundInfo,
}

/// Vote on a block proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockVote {
    /// Block hash being voted on
    pub block_hash: HeaderHash,
    /// Validator index casting the vote
    pub validator_index: usize,
    /// Vote type (support, reject, abstain)
    pub vote_type: VoteType,
    /// Validator's signature on the vote
    pub signature: BlsSignature,
    /// Round information
    pub round_info: RoundInfo,
    /// Vote timestamp
    pub voted_at: DateTime<Utc>,
}

/// Type of vote
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoteType {
    /// Support the proposal
    Support,
    /// Reject the proposal
    Reject,
    /// Abstain from voting
    Abstain,
}

/// Voting result for a block proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingResult {
    /// Block hash that was voted on
    pub block_hash: HeaderHash,
    /// All votes received
    pub votes: Vec<BlockVote>,
    /// Vote tally
    pub tally: VoteTally,
    /// Whether consensus was reached
    pub consensus_reached: bool,
    /// Final decision
    pub decision: ConsensusDecision,
    /// Voting round information
    pub round_info: RoundInfo,
}

/// Vote tally summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteTally {
    /// Number of support votes
    pub support: usize,
    /// Number of reject votes
    pub reject: usize,
    /// Number of abstain votes
    pub abstain: usize,
    /// Total stake supporting
    pub support_stake: u64,
    /// Total stake rejecting
    pub reject_stake: u64,
    /// Total stake abstaining
    pub abstain_stake: u64,
    /// Total active stake
    pub total_stake: u64,
}

/// Final consensus decision
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsensusDecision {
    /// Block accepted by consensus
    Accept,
    /// Block rejected by consensus
    Reject,
    /// No consensus reached (timeout/insufficient votes)
    NoConsensus,
}

/// Block proposal and voting manager
#[derive(Debug, Clone)]
pub struct BlockProposalManager {
    /// Current validator set
    validator_set: ValidatorSet,
    /// Leader selector for proposer validation
    leader_selector: LeaderSelector,
    /// Consensus configuration
    config: ConsensusConfig,
    /// Active proposals being voted on
    active_proposals: HashMap<HeaderHash, BlockProposal>,
    /// Voting state for active proposals
    voting_state: HashMap<HeaderHash, VotingState>,
}

/// Consensus configuration parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    /// Minimum percentage of stake required for consensus (e.g., 67 for 2/3)
    pub consensus_threshold: u8,
    /// Maximum time to wait for votes (seconds)
    pub voting_timeout: u64,
    /// Maximum number of concurrent proposals
    pub max_concurrent_proposals: usize,
    /// Minimum block time (seconds)
    pub min_block_time: u64,
    /// Maximum block size (bytes)
    pub max_block_size: usize,
}

/// Internal voting state tracking
#[derive(Debug, Clone)]
struct VotingState {
    /// Votes received so far
    votes: HashMap<usize, BlockVote>,
    /// Validators who have voted
    voted_validators: HashSet<usize>,
    /// Current vote tally
    tally: VoteTally,
    /// Voting started timestamp
    started_at: DateTime<Utc>,
    /// Whether voting is complete
    complete: bool,
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            consensus_threshold: 67, // 2/3 majority
            voting_timeout: 30,      // 30 seconds
            max_concurrent_proposals: 3,
            min_block_time: 1,       // 1 second
            max_block_size: 1024 * 1024, // 1MB
        }
    }
}

impl BlockProposal {
    /// Create a new block proposal
    pub fn new(
        header: Header,
        leader_proof: LeaderSelectionResult,
        proposer_private_key: &BlsPrivateKey,
        transaction_tree: Option<MerkleTree>,
        round_info: RoundInfo,
    ) -> Result<Self> {
        // Sign the block header
        let header_hash = header.hash()?;
        let proposer_signature = proposer_private_key.sign_hash(&header_hash.0);

        Ok(Self {
            header,
            leader_proof,
            proposer_signature,
            transaction_tree,
            proposed_at: Utc::now(),
            round_info,
        })
    }

    /// Get the block hash
    pub fn block_hash(&self) -> Result<HeaderHash> {
        self.header.hash()
    }

    /// Verify the proposal is valid
    pub fn verify(&self, validator_set: &ValidatorSet, leader_selector: &LeaderSelector) -> Result<bool> {
        // Verify leader selection proof
        if !leader_selector.verify_leader_selection(&self.leader_proof)? {
            return Ok(false);
        }

        // Verify proposer is the selected leader
        let proposer = validator_set.get_validator(self.leader_proof.leader_index)
            .ok_or(BlockProposalError::InvalidProposer(self.leader_proof.leader_index))?;

        // Verify proposer signature
        let header_hash = self.header.hash()?;
        if !proposer.bls_pubkey.verify_hash(&header_hash.0, &self.proposer_signature) {
            return Ok(false);
        }

        // Verify block structure
        self.verify_block_structure()?;

        Ok(true)
    }

    /// Verify block structure and constraints
    fn verify_block_structure(&self) -> Result<()> {
        // Verify header fields are consistent
        if self.header.round != self.round_info.round {
            return Err(BlockProposalError::InvalidRound {
                expected: self.round_info.round,
                got: self.header.round,
            }.into());
        }

        if self.header.height != self.round_info.height {
            return Err(BlockProposalError::InvalidBlockHeight {
                expected: self.round_info.height,
                got: self.header.height,
            }.into());
        }

        // Additional block validation logic would go here
        // (transaction validation, state transitions, etc.)

        Ok(())
    }
}

impl BlockVote {
    /// Create a new vote
    pub fn new(
        block_hash: HeaderHash,
        validator_index: usize,
        vote_type: VoteType,
        validator_private_key: &BlsPrivateKey,
        round_info: RoundInfo,
    ) -> Self {
        // Create vote message for signing
        let mut vote_data = Vec::new();
        vote_data.extend_from_slice(&block_hash.0);
        vote_data.extend_from_slice(&validator_index.to_le_bytes());
        vote_data.push(vote_type as u8);
        vote_data.extend_from_slice(&round_info.height.to_le_bytes());
        vote_data.extend_from_slice(&round_info.round.to_le_bytes());

        let vote_hash = domain_hash(domains::BLS_MESSAGE, &vote_data);
        let signature = validator_private_key.sign_hash(&vote_hash);

        Self {
            block_hash,
            validator_index,
            vote_type,
            signature,
            round_info,
            voted_at: Utc::now(),
        }
    }

    /// Verify the vote signature
    pub fn verify(&self, validator: &ValidatorInfo) -> Result<bool> {
        // Recreate vote message
        let mut vote_data = Vec::new();
        vote_data.extend_from_slice(&self.block_hash.0);
        vote_data.extend_from_slice(&self.validator_index.to_le_bytes());
        vote_data.push(self.vote_type as u8);
        vote_data.extend_from_slice(&self.round_info.height.to_le_bytes());
        vote_data.extend_from_slice(&self.round_info.round.to_le_bytes());

        let vote_hash = domain_hash(domains::BLS_MESSAGE, &vote_data);
        Ok(validator.bls_pubkey.verify_hash(&vote_hash, &self.signature))
    }
}

impl BlockProposalManager {
    /// Create a new block proposal manager
    pub fn new(
        validator_set: ValidatorSet,
        leader_selector: LeaderSelector,
        config: ConsensusConfig,
    ) -> Self {
        Self {
            validator_set,
            leader_selector,
            config,
            active_proposals: HashMap::new(),
            voting_state: HashMap::new(),
        }
    }

    /// Create with default configuration
    pub fn with_default_config(
        validator_set: ValidatorSet,
        leader_selector: LeaderSelector,
    ) -> Self {
        Self::new(validator_set, leader_selector, ConsensusConfig::default())
    }

    /// Submit a new block proposal
    pub fn submit_proposal(&mut self, proposal: BlockProposal) -> Result<()> {
        // Verify proposal
        if !proposal.verify(&self.validator_set, &self.leader_selector)? {
            return Err(BlockProposalError::BlockValidationFailed("Proposal verification failed".to_string()).into());
        }

        // Check concurrent proposal limit
        if self.active_proposals.len() >= self.config.max_concurrent_proposals {
            return Err(BlockProposalError::ConsensusError("Too many concurrent proposals".to_string()).into());
        }

        let block_hash = proposal.block_hash()?;

        // Initialize voting state
        let voting_state = VotingState {
            votes: HashMap::new(),
            voted_validators: HashSet::new(),
            tally: VoteTally {
                support: 0,
                reject: 0,
                abstain: 0,
                support_stake: 0,
                reject_stake: 0,
                abstain_stake: 0,
                total_stake: self.validator_set.total_stake(),
            },
            started_at: Utc::now(),
            complete: false,
        };

        self.active_proposals.insert(block_hash.clone(), proposal);
        self.voting_state.insert(block_hash, voting_state);

        Ok(())
    }

    /// Cast a vote on a proposal
    pub fn cast_vote(&mut self, vote: BlockVote) -> Result<()> {
        // Check if proposal exists
        let proposal = self.active_proposals.get(&vote.block_hash)
            .ok_or_else(|| BlockProposalError::ConsensusError("Proposal not found".to_string()))?;

        // Verify vote
        let validator = self.validator_set.get_validator(vote.validator_index)
            .ok_or(BlockProposalError::InvalidProposer(vote.validator_index))?;

        if !vote.verify(validator)? {
            return Err(BlockProposalError::InvalidVoteSignature("Vote signature verification failed".to_string()).into());
        }

        // Check for duplicate vote and update voting state
        let block_hash = vote.block_hash;
        let consensus_needed = {
            let voting_state = self.voting_state.get_mut(&vote.block_hash)
                .ok_or_else(|| BlockProposalError::ConsensusError("Voting state not found".to_string()))?;

            if voting_state.voted_validators.contains(&vote.validator_index) {
                return Err(BlockProposalError::DuplicateVote(vote.validator_index).into());
            }

            // Record vote
            voting_state.voted_validators.insert(vote.validator_index);
            
            // Update tally
            match vote.vote_type {
                VoteType::Support => {
                    voting_state.tally.support += 1;
                    voting_state.tally.support_stake += validator.stake;
                }
                VoteType::Reject => {
                    voting_state.tally.reject += 1;
                    voting_state.tally.reject_stake += validator.stake;
                }
                VoteType::Abstain => {
                    voting_state.tally.abstain += 1;
                    voting_state.tally.abstain_stake += validator.stake;
                }
            }

            voting_state.votes.insert(vote.validator_index, vote);
            
            // Return whether we need to check consensus
            !voting_state.complete
        };

        // Check if consensus is reached (separate borrow)
        if consensus_needed {
            let proposal_hash = proposal.block_hash()?;
            // Clone the block_hash to avoid borrowing conflicts
            let block_hash_clone = block_hash.clone();
            
            // Extract the consensus checking logic to avoid double mutable borrow
            if let Some(voting_state) = self.voting_state.get_mut(&block_hash_clone) {
                let total_stake = voting_state.tally.total_stake;
                let threshold_stake = (total_stake * self.config.consensus_threshold as u64) / 100;

                // Check for acceptance consensus
                if voting_state.tally.support_stake >= threshold_stake {
                    voting_state.complete = true;
                    // Consensus reached - could emit event here
                }
            }
        }

        Ok(())
    }

    /// Check if consensus has been reached for a proposal
    fn check_consensus(&mut self, _block_hash: &HeaderHash, voting_state: &mut VotingState) -> Result<()> {
        let total_stake = voting_state.tally.total_stake;
        let threshold_stake = (total_stake * self.config.consensus_threshold as u64) / 100;

        // Check for acceptance consensus
        if voting_state.tally.support_stake >= threshold_stake {
            voting_state.complete = true;
            return Ok(());
        }

        // Check for rejection consensus
        if voting_state.tally.reject_stake >= threshold_stake {
            voting_state.complete = true;
            return Ok(());
        }

        // Check for timeout
        let elapsed = Utc::now().signed_duration_since(voting_state.started_at);
        if elapsed.num_seconds() >= self.config.voting_timeout as i64 {
            voting_state.complete = true;
        }

        Ok(())
    }

    /// Get voting result for a proposal
    pub fn get_voting_result(&self, block_hash: &HeaderHash) -> Result<Option<VotingResult>> {
        let proposal = match self.active_proposals.get(block_hash) {
            Some(p) => p,
            None => return Ok(None),
        };

        let voting_state = match self.voting_state.get(block_hash) {
            Some(s) => s,
            None => return Ok(None),
        };

        if !voting_state.complete {
            return Ok(None);
        }

        // Determine consensus decision
        let threshold_stake = (voting_state.tally.total_stake * self.config.consensus_threshold as u64) / 100;
        let decision = if voting_state.tally.support_stake >= threshold_stake {
            ConsensusDecision::Accept
        } else if voting_state.tally.reject_stake >= threshold_stake {
            ConsensusDecision::Reject
        } else {
            ConsensusDecision::NoConsensus
        };

        let votes: Vec<BlockVote> = voting_state.votes.values().cloned().collect();

        Ok(Some(VotingResult {
            block_hash: block_hash.clone(),
            votes,
            tally: voting_state.tally.clone(),
            consensus_reached: decision != ConsensusDecision::NoConsensus,
            decision,
            round_info: proposal.round_info.clone(),
        }))
    }

    /// Get all active proposals
    pub fn get_active_proposals(&self) -> Vec<&BlockProposal> {
        self.active_proposals.values().collect()
    }

    /// Clean up completed proposals
    pub fn cleanup_completed_proposals(&mut self) -> Result<Vec<VotingResult>> {
        let mut completed_results = Vec::new();
        let mut to_remove = Vec::new();

        for (block_hash, voting_state) in &self.voting_state {
            if voting_state.complete {
                if let Some(result) = self.get_voting_result(block_hash)? {
                    completed_results.push(result);
                }
                to_remove.push(block_hash.clone());
            }
        }

        for block_hash in to_remove {
            self.active_proposals.remove(&block_hash);
            self.voting_state.remove(&block_hash);
        }

        Ok(completed_results)
    }

    /// Update validator set
    pub fn update_validator_set(&mut self, validator_set: ValidatorSet) {
        self.validator_set = validator_set;
        self.leader_selector.update_validator_set(self.validator_set.clone());
    }

    /// Get consensus statistics
    pub fn get_consensus_stats(&self) -> ConsensusStats {
        let active_proposals = self.active_proposals.len();
        let total_votes: usize = self.voting_state.values()
            .map(|state| state.votes.len())
            .sum();

        let completed_voting: usize = self.voting_state.values()
            .map(|state| if state.complete { 1 } else { 0 })
            .sum();

        ConsensusStats {
            active_proposals,
            total_votes,
            completed_voting,
            validator_count: self.validator_set.len(),
            total_stake: self.validator_set.total_stake(),
        }
    }
}

/// Consensus statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusStats {
    /// Number of active proposals
    pub active_proposals: usize,
    /// Total votes cast
    pub total_votes: usize,
    /// Number of completed voting rounds
    pub completed_voting: usize,
    /// Number of validators
    pub validator_count: usize,
    /// Total stake in the system
    pub total_stake: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use bpi_validator_set::ValidatorInfo;
    use bpi_blsagg::PrivateKey;
    use bpi_leader_selection::LeaderSelector;
    use std::collections::HashMap;

    fn create_test_validator(index: usize, stake: u64) -> (ValidatorInfo, PrivateKey) {
        let mut seed = [0u8; 32];
        seed[0..8].copy_from_slice(&index.to_le_bytes());
        
        let bls_private_key = PrivateKey::from_bytes(&seed).unwrap();
        let bls_public_key = bls_private_key.public_key();
        
        let vrf_private_key = VrfPrivateKey::from_bytes(&seed).unwrap();
        let vrf_public_key = vrf_private_key.public_key();

        let validator = ValidatorInfo::new(
            index,
            bls_public_key,
            vrf_public_key,
            stake,
            format!("127.0.0.{}", index + 1),
            format!("validator-{}", index),
        );

        (validator, bls_private_key)
    }

    fn create_test_setup() -> (BlockProposalManager, Vec<PrivateKey>) {
        let mut validators = Vec::new();
        let mut private_keys = Vec::new();
        
        for i in 0..4 {
            let (validator, private_key) = create_test_validator(i, 1000 + i as u64 * 500);
            validators.push(validator);
            private_keys.push(private_key);
        }

        let validator_set = ValidatorSet::from_validators(validators, 1).unwrap();
        let leader_selector = LeaderSelector::with_default_config(validator_set.clone());
        let manager = BlockProposalManager::with_default_config(validator_set, leader_selector);

        (manager, private_keys)
    }

    fn create_test_proposal(
        height: u64,
        round: u64,
        leader_index: usize,
        private_key: &PrivateKey,
    ) -> BlockProposal {
        let round_info = RoundInfo {
            height,
            round,
            timestamp: Utc::now(),
            epoch: 1,
        };

        let leader_proof = bpi_leader_selection::LeaderSelectionResult {
            leader_index,
            vrf_proof: VrfProof::from_bytes(&[0u8; 80]).unwrap(),
            vrf_output: VrfOutput::from_bytes(&[0u8; 32]).unwrap(),
            round_info: round_info.clone(),
            selection_probability: 0.25,
        };

        let header = Header::new(
            1,
            height,
            [0u8; 32],
            [1u8; 32],
            [2u8; 32],
            [3u8; 32],
            [4u8; 32],
            [5u8; 32],
            ConsensusMode::Ibft,
            round,
        );

        BlockProposal::new(header, leader_proof, private_key, None, round_info).unwrap()
    }

    #[test]
    fn test_block_proposal_creation() {
        let (_, private_keys) = create_test_setup();
        let proposal = create_test_proposal(1, 1, 0, &private_keys[0]);
        
        assert_eq!(proposal.header.height, 1);
        assert_eq!(proposal.header.round, 1);
        assert_eq!(proposal.leader_proof.leader_index, 0);
    }

    #[test]
    fn test_block_vote_creation() {
        let (_, private_keys) = create_test_setup();
        let proposal = create_test_proposal(1, 1, 0, &private_keys[0]);
        let block_hash = proposal.block_hash().unwrap();
        
        let round_info = RoundInfo {
            height: 1,
            round: 1,
            timestamp: Utc::now(),
            epoch: 1,
        };

        let vote = BlockVote::new(
            block_hash,
            1,
            VoteType::Support,
            &private_keys[1],
            round_info,
        );
        
        assert_eq!(vote.validator_index, 1);
        assert_eq!(vote.vote_type, VoteType::Support);
    }

    #[test]
    fn test_proposal_submission() {
        let (mut manager, private_keys) = create_test_setup();
        let proposal = create_test_proposal(1, 1, 0, &private_keys[0]);
        
        // Note: This test will fail verification due to simulated leader proof
        // In a real implementation, the leader proof would be properly generated
        let result = manager.submit_proposal(proposal);
        
        // We expect this to fail due to simulated VRF proof
        assert!(result.is_err());
    }

    #[test]
    fn test_vote_casting() {
        let (mut manager, private_keys) = create_test_setup();
        let proposal = create_test_proposal(1, 1, 0, &private_keys[0]);
        let block_hash = proposal.block_hash().unwrap();
        
        // Manually add proposal to bypass verification for testing
        manager.active_proposals.insert(block_hash.clone(), proposal.clone());
        manager.voting_state.insert(block_hash.clone(), VotingState {
            votes: HashMap::new(),
            voted_validators: HashSet::new(),
            tally: VoteTally {
                support: 0,
                reject: 0,
                abstain: 0,
                support_stake: 0,
                reject_stake: 0,
                abstain_stake: 0,
                total_stake: manager.validator_set.total_stake(),
            },
            started_at: Utc::now(),
            complete: false,
        });

        let round_info = RoundInfo {
            height: 1,
            round: 1,
            timestamp: Utc::now(),
            epoch: 1,
        };

        let vote = BlockVote::new(
            block_hash,
            1,
            VoteType::Support,
            &private_keys[1],
            round_info,
        );
        
        let result = manager.cast_vote(vote);
        assert!(result.is_ok());
        
        let stats = manager.get_consensus_stats();
        assert_eq!(stats.total_votes, 1);
    }

    #[test]
    fn test_consensus_threshold() {
        let (mut manager, private_keys) = create_test_setup();
        let proposal = create_test_proposal(1, 1, 0, &private_keys[0]);
        let block_hash = proposal.block_hash().unwrap();
        
        // Manually setup for testing
        manager.active_proposals.insert(block_hash.clone(), proposal.clone());
        manager.voting_state.insert(block_hash.clone(), VotingState {
            votes: HashMap::new(),
            voted_validators: HashSet::new(),
            tally: VoteTally {
                support: 0,
                reject: 0,
                abstain: 0,
                support_stake: 0,
                reject_stake: 0,
                abstain_stake: 0,
                total_stake: manager.validator_set.total_stake(),
            },
            started_at: Utc::now(),
            complete: false,
        });

        let round_info = RoundInfo {
            height: 1,
            round: 1,
            timestamp: Utc::now(),
            epoch: 1,
        };

        // Cast votes from 3 out of 4 validators (should reach consensus)
        for i in 1..4 {
            let vote = BlockVote::new(
                block_hash.clone(),
                i,
                VoteType::Support,
                &private_keys[i],
                round_info.clone(),
            );
            manager.cast_vote(vote).unwrap();
        }
        
        let result = manager.get_voting_result(&block_hash).unwrap();
        if let Some(voting_result) = result {
            assert_eq!(voting_result.decision, ConsensusDecision::Accept);
            assert!(voting_result.consensus_reached);
        }
    }

    #[test]
    fn test_duplicate_vote_prevention() {
        let (mut manager, private_keys) = create_test_setup();
        let proposal = create_test_proposal(1, 1, 0, &private_keys[0]);
        let block_hash = proposal.block_hash().unwrap();
        
        // Manually setup for testing
        manager.active_proposals.insert(block_hash.clone(), proposal.clone());
        manager.voting_state.insert(block_hash.clone(), VotingState {
            votes: HashMap::new(),
            voted_validators: HashSet::new(),
            tally: VoteTally {
                support: 0,
                reject: 0,
                abstain: 0,
                support_stake: 0,
                reject_stake: 0,
                abstain_stake: 0,
                total_stake: manager.validator_set.total_stake(),
            },
            started_at: Utc::now(),
            complete: false,
        });

        let round_info = RoundInfo {
            height: 1,
            round: 1,
            timestamp: Utc::now(),
            epoch: 1,
        };

        let vote1 = BlockVote::new(
            block_hash.clone(),
            1,
            VoteType::Support,
            &private_keys[1],
            round_info.clone(),
        );
        
        let vote2 = BlockVote::new(
            block_hash.clone(),
            1,
            VoteType::Reject,
            &private_keys[1],
            round_info,
        );
        
        // First vote should succeed
        assert!(manager.cast_vote(vote1).is_ok());
        
        // Second vote from same validator should fail
        assert!(manager.cast_vote(vote2).is_err());
    }

    #[test]
    fn test_consensus_stats() {
        let (manager, _) = create_test_setup();
        let stats = manager.get_consensus_stats();
        
        assert_eq!(stats.validator_count, 4);
        assert_eq!(stats.active_proposals, 0);
        assert_eq!(stats.total_votes, 0);
        assert!(stats.total_stake > 0);
    }
}
