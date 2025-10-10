//! Production-Grade Consensus Engine for BPCI Enterprise
//! 
//! This module provides real, functional consensus capabilities
//! for distributed agreement in the BPCI network.

use crate::core::types::{NodeId, BlockHeight, Timestamp};
use crate::core::network::{NetworkManager, NetworkMessage};
use crate::core::storage::{StorageManager};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::{Result, anyhow};
use sha2::{Sha256, Digest};

/// Consensus state for a single round
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConsensusState {
    /// Preparing for consensus
    Preparing,
    /// Proposing a value
    Proposing,
    /// Voting on proposals
    Voting,
    /// Committing the agreed value
    Committing,
    /// Consensus completed
    Completed,
    /// Consensus failed
    Failed(String),
}

/// A proposal in the consensus protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: String,
    pub proposer: NodeId,
    pub round: u64,
    pub block_height: BlockHeight,
    pub data: Vec<u8>,
    pub timestamp: Timestamp,
    pub hash: String,
}

impl Proposal {
    pub fn new(proposer: NodeId, round: u64, block_height: BlockHeight, data: Vec<u8>) -> Self {
        let timestamp = Timestamp::now();
        let mut hasher = Sha256::new();
        hasher.update(&data);
        hasher.update(proposer.as_str().as_bytes());
        hasher.update(&round.to_le_bytes());
        hasher.update(&block_height.value().to_le_bytes());
        let hash = format!("{:x}", hasher.finalize());
        
        Self {
            id: format!("prop_{}_{}", round, &hash[..8]),
            proposer,
            round,
            block_height,
            data,
            timestamp,
            hash,
        }
    }
    
    pub fn verify(&self) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(&self.data);
        hasher.update(self.proposer.as_str().as_bytes());
        hasher.update(&self.round.to_le_bytes());
        hasher.update(&self.block_height.value().to_le_bytes());
        let expected_hash = format!("{:x}", hasher.finalize());
        
        self.hash == expected_hash
    }
}

/// A vote for a proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub voter: NodeId,
    pub proposal_id: String,
    pub round: u64,
    pub vote_type: VoteType,
    pub timestamp: Timestamp,
}

/// Type of vote
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VoteType {
    /// Vote to accept the proposal
    Accept,
    /// Vote to reject the proposal
    Reject,
    /// Abstain from voting
    Abstain,
}

/// Consensus round information
#[derive(Debug, Clone)]
pub struct ConsensusRound {
    pub round: u64,
    pub block_height: BlockHeight,
    pub state: ConsensusState,
    pub proposals: HashMap<String, Proposal>,
    pub votes: HashMap<NodeId, Vote>,
    pub start_time: Timestamp,
    pub timeout: Option<Timestamp>,
}

impl ConsensusRound {
    pub fn new(round: u64, block_height: BlockHeight) -> Self {
        Self {
            round,
            block_height,
            state: ConsensusState::Preparing,
            proposals: HashMap::new(),
            votes: HashMap::new(),
            start_time: Timestamp::now(),
            timeout: None,
        }
    }
    
    /// Check if we have a quorum (majority) of votes
    pub fn has_quorum(&self, total_validators: usize) -> bool {
        let required = (total_validators / 2) + 1;
        self.votes.len() >= required
    }
    
    /// Get the winning proposal (if any)
    pub fn get_winning_proposal(&self, total_validators: usize) -> Option<&Proposal> {
        if !self.has_quorum(total_validators) {
            return None;
        }
        
        let mut vote_counts: HashMap<String, usize> = HashMap::new();
        
        for vote in self.votes.values() {
            if vote.vote_type == VoteType::Accept {
                *vote_counts.entry(vote.proposal_id.clone()).or_insert(0) += 1;
            }
        }
        
        // Find proposal with majority votes
        let required = (total_validators / 2) + 1;
        for (proposal_id, count) in vote_counts {
            if count >= required {
                return self.proposals.get(&proposal_id);
            }
        }
        
        None
    }
}

/// Configuration for the consensus engine
#[derive(Debug, Clone)]
pub struct ConsensusConfig {
    /// Timeout for each consensus round in seconds
    pub round_timeout_secs: u64,
    /// Maximum number of proposals per round
    pub max_proposals_per_round: usize,
    /// Minimum number of validators required
    pub min_validators: usize,
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            round_timeout_secs: 30,
            max_proposals_per_round: 10,
            min_validators: 3,
        }
    }
}

/// Production-grade consensus engine
#[derive(Debug)]
pub struct ConsensusEngine {
    /// This node's ID
    node_id: NodeId,
    /// Configuration
    config: ConsensusConfig,
    /// Current consensus round
    current_round: Arc<RwLock<Option<ConsensusRound>>>,
    /// Known validators
    validators: Arc<RwLock<HashSet<NodeId>>>,
    /// Network manager for communication
    network: Arc<NetworkManager>,
    /// Storage for persistence
    storage: Arc<StorageManager>,
    /// Consensus history
    completed_rounds: Arc<RwLock<HashMap<u64, ConsensusRound>>>,
}

impl ConsensusEngine {
    /// Create a new consensus engine
    pub async fn new(
        node_id: NodeId,
        config: ConsensusConfig,
        network: Arc<NetworkManager>,
        storage: Arc<StorageManager>,
    ) -> Result<Self> {
        let engine = Self {
            node_id,
            config,
            current_round: Arc::new(RwLock::new(None)),
            validators: Arc::new(RwLock::new(HashSet::new())),
            network,
            storage,
            completed_rounds: Arc::new(RwLock::new(HashMap::new())),
        };
        
        // Load validators from storage
        engine.load_validators().await?;
        
        Ok(engine)
    }
    
    /// Add a validator to the network
    pub async fn add_validator(&self, validator_id: NodeId) -> Result<()> {
        let mut validators = self.validators.write().await;
        validators.insert(validator_id.clone());
        
        // Persist to storage (using "config" category which exists)
        let validator_data = validator_id.as_str().as_bytes().to_vec();
        let key = format!("validator_{}", validator_id.as_str());
        self.storage.store("config", key, validator_data).await?;
        
        Ok(())
    }
    
    /// Remove a validator from the network
    pub async fn remove_validator(&self, validator_id: &NodeId) -> Result<()> {
        let mut validators = self.validators.write().await;
        validators.remove(validator_id);
        
        // Remove from storage (using "config" category which exists)
        let key = format!("validator_{}", validator_id.as_str());
        self.storage.delete("config", &key).await?;
        
        Ok(())
    }
    
    /// Get current validators
    pub async fn get_validators(&self) -> HashSet<NodeId> {
        let validators = self.validators.read().await;
        validators.clone()
    }
    
    /// Start a new consensus round
    pub async fn start_round(&self, block_height: BlockHeight) -> Result<u64> {
        let validators = self.validators.read().await;
        if validators.len() < self.config.min_validators {
            return Err(anyhow!("Not enough validators: {} < {}", validators.len(), self.config.min_validators));
        }
        
        let mut current_round = self.current_round.write().await;
        if current_round.is_some() {
            return Err(anyhow!("Consensus round already in progress"));
        }
        
        let round_number = self.get_next_round_number().await;
        let round = ConsensusRound::new(round_number, block_height);
        
        *current_round = Some(round);
        
        Ok(round_number)
    }
    
    /// Submit a proposal for the current round
    pub async fn submit_proposal(&self, data: Vec<u8>) -> Result<String> {
        let mut current_round = self.current_round.write().await;
        let round = current_round.as_mut()
            .ok_or_else(|| anyhow!("No active consensus round"))?;
        
        if round.state != ConsensusState::Preparing && round.state != ConsensusState::Proposing {
            return Err(anyhow!("Cannot submit proposal in state: {:?}", round.state));
        }
        
        if round.proposals.len() >= self.config.max_proposals_per_round {
            return Err(anyhow!("Maximum proposals per round exceeded"));
        }
        
        let proposal = Proposal::new(self.node_id.clone(), round.round, round.block_height, data);
        let proposal_id = proposal.id.clone();
        
        round.proposals.insert(proposal_id.clone(), proposal.clone());
        round.state = ConsensusState::Proposing;
        
        // Broadcast proposal to other validators
        let message = NetworkMessage::Data {
            from: self.node_id.clone(),
            to: None, // Broadcast
            payload: serde_json::to_vec(&("proposal", &proposal))?,
        };
        
        self.network.broadcast_message(message).await?;
        
        Ok(proposal_id)
    }
    
    /// Submit a vote for a proposal
    pub async fn submit_vote(&self, proposal_id: String, vote_type: VoteType) -> Result<()> {
        let mut current_round = self.current_round.write().await;
        let round = current_round.as_mut()
            .ok_or_else(|| anyhow!("No active consensus round"))?;
        
        if round.state != ConsensusState::Proposing && round.state != ConsensusState::Voting {
            return Err(anyhow!("Cannot vote in state: {:?}", round.state));
        }
        
        if !round.proposals.contains_key(&proposal_id) {
            return Err(anyhow!("Proposal not found: {}", proposal_id));
        }
        
        let vote = Vote {
            voter: self.node_id.clone(),
            proposal_id: proposal_id.clone(),
            round: round.round,
            vote_type,
            timestamp: Timestamp::now(),
        };
        
        round.votes.insert(self.node_id.clone(), vote.clone());
        round.state = ConsensusState::Voting;
        
        // Broadcast vote to other validators
        let message = NetworkMessage::Data {
            from: self.node_id.clone(),
            to: None, // Broadcast
            payload: serde_json::to_vec(&("vote", &vote))?,
        };
        
        self.network.broadcast_message(message).await?;
        
        Ok(())
    }
    
    /// Check if consensus has been reached
    pub async fn check_consensus(&self) -> Result<Option<Proposal>> {
        let mut current_round = self.current_round.write().await;
        let round = current_round.as_mut()
            .ok_or_else(|| anyhow!("No active consensus round"))?;
        
        let validators = self.validators.read().await;
        let total_validators = validators.len();
        
        // Check for winning proposal without borrowing round
        let winning_proposal = {
            if !round.has_quorum(total_validators) {
                None
            } else {
                let mut vote_counts: HashMap<String, usize> = HashMap::new();
                
                for vote in round.votes.values() {
                    if vote.vote_type == VoteType::Accept {
                        *vote_counts.entry(vote.proposal_id.clone()).or_insert(0) += 1;
                    }
                }
                
                // Find proposal with majority votes
                let required = (total_validators / 2) + 1;
                let mut winning_id = None;
                for (proposal_id, count) in vote_counts {
                    if count >= required {
                        winning_id = Some(proposal_id);
                        break;
                    }
                }
                
                winning_id.and_then(|id| round.proposals.get(&id).cloned())
            }
        };
        
        if let Some(result) = winning_proposal {
            // Consensus reached!
            round.state = ConsensusState::Committing;
            
            // Move to completed rounds
            let mut completed_round = round.clone();
            completed_round.state = ConsensusState::Completed;
            
            let mut completed_rounds = self.completed_rounds.write().await;
            completed_rounds.insert(round.round, completed_round);
            
            // Clear current round
            *current_round = None;
            
            // Persist result
            self.persist_consensus_result(&result).await?;
            
            return Ok(Some(result));
        }
        
        Ok(None)
    }
    
    /// Get consensus statistics
    pub async fn get_stats(&self) -> ConsensusStats {
        let completed_rounds = self.completed_rounds.read().await;
        let validators = self.validators.read().await;
        let current_round = self.current_round.read().await;
        
        ConsensusStats {
            total_rounds: completed_rounds.len(),
            active_validators: validators.len(),
            current_round_number: current_round.as_ref().map(|r| r.round),
            current_state: current_round.as_ref().map(|r| r.state.clone()),
        }
    }
    
    /// Load validators from storage
    async fn load_validators(&self) -> Result<()> {
        let config_keys = self.storage.list_keys("config").await?;
        let mut validators = self.validators.write().await;
        
        for key in config_keys {
            if key.starts_with("validator_") {
                let validator_id = key.strip_prefix("validator_").unwrap();
                validators.insert(NodeId::from_string(validator_id.to_string()));
            }
        }
        
        Ok(())
    }
    
    /// Get the next round number
    async fn get_next_round_number(&self) -> u64 {
        let completed_rounds = self.completed_rounds.read().await;
        completed_rounds.keys().max().unwrap_or(&0) + 1
    }
    
    /// Persist consensus result to storage
    async fn persist_consensus_result(&self, proposal: &Proposal) -> Result<()> {
        let result_data = serde_json::to_vec(proposal)?;
        let key = format!("consensus_result_{}", proposal.round);
        self.storage.store("config", key, result_data).await?;
        Ok(())
    }
}

/// Consensus statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusStats {
    pub total_rounds: usize,
    pub active_validators: usize,
    pub current_round_number: Option<u64>,
    pub current_state: Option<ConsensusState>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::network::NetworkManager;
    use tempfile::TempDir;

    async fn create_test_consensus() -> (ConsensusEngine, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let storage_config = crate::core::storage::StorageConfig {
            base_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        
        let storage = Arc::new(StorageManager::new(storage_config).await.unwrap());
        let test_address = crate::core::types::NetworkAddress::localhost(8080);
        let network = Arc::new(NetworkManager::new(test_address));
        let node_id = NodeId::new();
        let config = ConsensusConfig::default();
        
        let consensus = ConsensusEngine::new(node_id, config, network, storage).await.unwrap();
        (consensus, temp_dir)
    }

    #[tokio::test]
    async fn test_consensus_creation() {
        let (consensus, _temp_dir) = create_test_consensus().await;
        let stats = consensus.get_stats().await;
        assert_eq!(stats.total_rounds, 0);
        assert_eq!(stats.active_validators, 0);
    }

    #[tokio::test]
    async fn test_validator_management() {
        let (consensus, _temp_dir) = create_test_consensus().await;
        
        let validator1 = NodeId::new();
        let validator2 = NodeId::new();
        
        // Add validators
        consensus.add_validator(validator1.clone()).await.unwrap();
        consensus.add_validator(validator2.clone()).await.unwrap();
        
        let validators = consensus.get_validators().await;
        assert_eq!(validators.len(), 2);
        assert!(validators.contains(&validator1));
        assert!(validators.contains(&validator2));
        
        // Remove validator
        consensus.remove_validator(&validator1).await.unwrap();
        let validators = consensus.get_validators().await;
        assert_eq!(validators.len(), 1);
        assert!(!validators.contains(&validator1));
        assert!(validators.contains(&validator2));
    }

    #[tokio::test]
    async fn test_proposal_creation_and_verification() {
        let node_id = NodeId::new();
        let data = b"test data".to_vec();
        let proposal = Proposal::new(node_id, 1, BlockHeight(10), data);
        
        assert!(proposal.verify());
        assert_eq!(proposal.round, 1);
        assert_eq!(proposal.block_height, BlockHeight(10));
    }

    #[tokio::test]
    async fn test_consensus_round() {
        let (consensus, _temp_dir) = create_test_consensus().await;
        
        // Add enough validators
        for _ in 0..3 {
            consensus.add_validator(NodeId::new()).await.unwrap();
        }
        
        // Start a round
        let round_num = consensus.start_round(BlockHeight(1)).await.unwrap();
        assert_eq!(round_num, 1);
        
        // Submit a proposal
        let proposal_id = consensus.submit_proposal(b"test data".to_vec()).await.unwrap();
        assert!(!proposal_id.is_empty());
        
        // Vote on the proposal
        consensus.submit_vote(proposal_id, VoteType::Accept).await.unwrap();
        
        let stats = consensus.get_stats().await;
        assert_eq!(stats.current_round_number, Some(1));
    }

    #[tokio::test]
    async fn test_insufficient_validators() {
        let (consensus, _temp_dir) = create_test_consensus().await;
        
        // Try to start round without enough validators
        let result = consensus.start_round(BlockHeight(1)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Not enough validators"));
    }
}
