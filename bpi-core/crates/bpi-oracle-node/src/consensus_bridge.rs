//! Consensus Bridge module for BPI Oracle Node
//!
//! Enables cross-node consensus coordination by bridging consensus protocols
//! between different BPI nodes, aggregating votes, and facilitating distributed
//! decision making across the BPI ecosystem.

use anyhow::Result;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::{ConsensusConfig, BpiNode, OracleMessage, MessageType, MessagePriority};

/// Consensus proposal for cross-node voting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusProposal {
    pub proposal_id: String,
    pub proposer_node: String,
    pub proposal_type: ProposalType,
    pub content: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub voting_deadline: DateTime<Utc>,
    pub minimum_votes: usize,
    pub required_threshold: f64,
    pub metadata: HashMap<String, String>,
}

/// Types of consensus proposals
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProposalType {
    /// Network parameter change
    NetworkParameter,
    /// Node admission/removal
    NodeMembership,
    /// Protocol upgrade
    ProtocolUpgrade,
    /// Economic parameter adjustment
    EconomicParameter,
    /// Emergency action
    Emergency,
    /// Custom proposal type
    Custom(String),
}

impl std::fmt::Display for ProposalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalType::NetworkParameter => write!(f, "NetworkParameter"),
            ProposalType::NodeMembership => write!(f, "NodeMembership"),
            ProposalType::ProtocolUpgrade => write!(f, "ProtocolUpgrade"),
            ProposalType::EconomicParameter => write!(f, "EconomicParameter"),
            ProposalType::Emergency => write!(f, "Emergency"),
            ProposalType::Custom(name) => write!(f, "Custom({})", name),
        }
    }
}

/// Consensus vote from a BPI node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusVote {
    pub vote_id: String,
    pub proposal_id: String,
    pub voter_node: String,
    pub vote: VoteDecision,
    pub reasoning: Option<String>,
    pub vote_weight: f64,
    pub timestamp: DateTime<Utc>,
    pub signature: Vec<u8>,
}

/// Vote decision options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VoteDecision {
    Approve,
    Reject,
    Abstain,
}

impl std::fmt::Display for VoteDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VoteDecision::Approve => write!(f, "Approve"),
            VoteDecision::Reject => write!(f, "Reject"),
            VoteDecision::Abstain => write!(f, "Abstain"),
        }
    }
}

/// Consensus round state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusRound {
    pub round_id: String,
    pub proposal: ConsensusProposal,
    pub votes: HashMap<String, ConsensusVote>,
    pub status: RoundStatus,
    pub result: Option<ConsensusResult>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub participating_nodes: Vec<String>,
}

/// Status of a consensus round
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoundStatus {
    Active,
    Completed,
    Failed,
    Expired,
}

/// Final result of a consensus round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusResult {
    pub decision: VoteDecision,
    pub total_votes: usize,
    pub approve_votes: usize,
    pub reject_votes: usize,
    pub abstain_votes: usize,
    pub total_weight: f64,
    pub approve_weight: f64,
    pub reject_weight: f64,
    pub threshold_met: bool,
    pub finalized_at: DateTime<Utc>,
}

/// Consensus bridge statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBridgeStats {
    pub total_proposals: u64,
    pub active_rounds: usize,
    pub completed_rounds: u64,
    pub failed_rounds: u64,
    pub average_participation_rate: f64,
    pub average_consensus_time_seconds: f64,
    pub total_votes_cast: u64,
}

/// Consensus bridge for cross-node coordination
#[derive(Debug)]
pub struct ConsensusBridge {
    config: ConsensusConfig,
    active_rounds: Arc<DashMap<String, ConsensusRound>>,
    completed_rounds: Arc<RwLock<Vec<ConsensusRound>>>,
    node_weights: Arc<DashMap<String, f64>>,
    stats: Arc<RwLock<ConsensusBridgeStats>>,
    vote_handlers: Arc<DashMap<String, tokio::sync::mpsc::Sender<ConsensusVote>>>,
    shutdown_tx: Arc<Mutex<Option<tokio::sync::broadcast::Sender<()>>>>,
}

impl ConsensusBridge {
    /// Create new consensus bridge
    pub async fn new(config: ConsensusConfig) -> Result<Self> {
        info!("Initializing Consensus Bridge");

        Ok(Self {
            config,
            active_rounds: Arc::new(DashMap::new()),
            completed_rounds: Arc::new(RwLock::new(Vec::new())),
            node_weights: Arc::new(DashMap::new()),
            stats: Arc::new(RwLock::new(ConsensusBridgeStats {
                total_proposals: 0,
                active_rounds: 0,
                completed_rounds: 0,
                failed_rounds: 0,
                average_participation_rate: 0.0,
                average_consensus_time_seconds: 0.0,
                total_votes_cast: 0,
            })),
            vote_handlers: Arc::new(DashMap::new()),
            shutdown_tx: Arc::new(Mutex::new(None)),
        })
    }

    /// Start the consensus bridge
    pub async fn start(&self) -> Result<()> {
        info!("Starting Consensus Bridge");

        // Start background services
        self.start_background_services().await?;

        info!("✅ Consensus Bridge started successfully");
        Ok(())
    }

    /// Submit a new consensus proposal
    pub async fn submit_proposal(&self, proposal: ConsensusProposal) -> Result<String> {
        info!("Submitting consensus proposal: {} ({})", proposal.proposal_id, proposal.proposal_type);

        // Validate proposal
        self.validate_proposal(&proposal).await?;

        // Create consensus round
        let round = ConsensusRound {
            round_id: proposal.proposal_id.clone(),
            proposal: proposal.clone(),
            votes: HashMap::new(),
            status: RoundStatus::Active,
            result: None,
            started_at: Utc::now(),
            completed_at: None,
            participating_nodes: Vec::new(),
        };

        self.active_rounds.insert(proposal.proposal_id.clone(), round);

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_proposals += 1;
        stats.active_rounds = self.active_rounds.len();

        // Broadcast proposal to all nodes
        self.broadcast_proposal(&proposal).await?;

        info!("✅ Consensus proposal submitted: {}", proposal.proposal_id);
        Ok(proposal.proposal_id)
    }

    /// Submit a vote for a consensus proposal
    pub async fn submit_vote(&self, vote: ConsensusVote) -> Result<()> {
        debug!("Submitting vote: {} -> {} ({})", vote.voter_node, vote.proposal_id, vote.vote);

        // Verify vote signature
        self.verify_vote_signature(&vote).await?;

        // Check if proposal exists and is active
        if let Some(mut round) = self.active_rounds.get_mut(&vote.proposal_id) {
            if round.status != RoundStatus::Active {
                return Err(anyhow::anyhow!("Consensus round is not active: {}", vote.proposal_id));
            }

            // Check voting deadline
            if Utc::now() > round.proposal.voting_deadline {
                round.status = RoundStatus::Expired;
                return Err(anyhow::anyhow!("Voting deadline has passed for proposal: {}", vote.proposal_id));
            }

            // Add vote to round
            round.votes.insert(vote.voter_node.clone(), vote.clone());
            
            // Add to participating nodes if not already present
            if !round.participating_nodes.contains(&vote.voter_node) {
                round.participating_nodes.push(vote.voter_node.clone());
            }

            // Update statistics
            let mut stats = self.stats.write().await;
            stats.total_votes_cast += 1;

            // Check if we have enough votes to conclude
            if self.should_conclude_round(&round).await? {
                self.conclude_consensus_round(&vote.proposal_id).await?;
            }

            debug!("✅ Vote submitted: {} for proposal {}", vote.voter_node, vote.proposal_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Proposal not found or not active: {}", vote.proposal_id))
        }
    }

    /// Get active consensus rounds
    pub async fn get_active_rounds(&self) -> Vec<ConsensusRound> {
        self.active_rounds.iter().map(|entry| entry.value().clone()).collect()
    }

    /// Get consensus round by ID
    pub async fn get_round(&self, round_id: &str) -> Option<ConsensusRound> {
        self.active_rounds.get(round_id).map(|entry| entry.value().clone())
    }

    /// Get consensus bridge statistics
    pub async fn get_stats(&self) -> ConsensusBridgeStats {
        self.stats.read().await.clone()
    }

    /// Set voting weight for a node
    pub async fn set_node_weight(&self, node_id: &str, weight: f64) -> Result<()> {
        if weight < 0.0 || weight > 10.0 {
            return Err(anyhow::anyhow!("Invalid node weight: {}", weight));
        }

        self.node_weights.insert(node_id.to_string(), weight);
        info!("Set voting weight for node {}: {}", node_id, weight);
        Ok(())
    }

    /// Get voting weight for a node
    pub async fn get_node_weight(&self, node_id: &str) -> f64 {
        self.node_weights.get(node_id).map(|entry| *entry.value()).unwrap_or(1.0)
    }

    /// Shutdown consensus bridge
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down Consensus Bridge");

        // Send shutdown signal
        if let Some(tx) = self.shutdown_tx.lock().await.take() {
            let _ = tx.send(());
        }

        // Move active rounds to completed
        for round in self.active_rounds.iter() {
            let mut completed_round = round.value().clone();
            completed_round.status = RoundStatus::Failed;
            completed_round.completed_at = Some(Utc::now());
            
            let mut completed_rounds = self.completed_rounds.write().await;
            completed_rounds.push(completed_round);
        }

        self.active_rounds.clear();

        info!("✅ Consensus Bridge shutdown complete");
        Ok(())
    }

    /// Validate consensus proposal
    async fn validate_proposal(&self, proposal: &ConsensusProposal) -> Result<()> {
        // Check if proposal ID is unique
        if self.active_rounds.contains_key(&proposal.proposal_id) {
            return Err(anyhow::anyhow!("Proposal ID already exists: {}", proposal.proposal_id));
        }

        // Check voting deadline
        if proposal.voting_deadline <= Utc::now() {
            return Err(anyhow::anyhow!("Voting deadline must be in the future"));
        }

        // Check minimum votes requirement
        if proposal.minimum_votes < self.config.min_consensus_nodes {
            return Err(anyhow::anyhow!("Minimum votes below required threshold"));
        }

        // Check threshold validity
        if proposal.required_threshold < 0.0 || proposal.required_threshold > 1.0 {
            return Err(anyhow::anyhow!("Invalid threshold: {}", proposal.required_threshold));
        }

        Ok(())
    }

    /// Verify vote signature
    async fn verify_vote_signature(&self, vote: &ConsensusVote) -> Result<()> {
        // In a real implementation, this would verify the cryptographic signature
        // For now, we'll do basic validation
        if vote.signature.is_empty() {
            return Err(anyhow::anyhow!("Vote signature is required"));
        }

        if vote.vote_weight < 0.0 {
            return Err(anyhow::anyhow!("Invalid vote weight: {}", vote.vote_weight));
        }

        Ok(())
    }

    /// Check if consensus round should be concluded
    async fn should_conclude_round(&self, round: &ConsensusRound) -> Result<bool> {
        let total_votes = round.votes.len();
        
        // Check if minimum votes reached
        if total_votes < round.proposal.minimum_votes {
            return Ok(false);
        }

        // Check if voting deadline passed
        if Utc::now() > round.proposal.voting_deadline {
            return Ok(true);
        }

        // Check if we have unanimous decision (optimization)
        let approve_votes = round.votes.values().filter(|v| v.vote == VoteDecision::Approve).count();
        let reject_votes = round.votes.values().filter(|v| v.vote == VoteDecision::Reject).count();
        
        let total_weight: f64 = round.votes.values().map(|v| v.vote_weight).sum();
        let approve_weight: f64 = round.votes.values()
            .filter(|v| v.vote == VoteDecision::Approve)
            .map(|v| v.vote_weight)
            .sum();

        // Early conclusion if threshold clearly met or impossible to meet
        let approve_ratio = approve_weight / total_weight;
        if approve_ratio >= round.proposal.required_threshold {
            return Ok(true);
        }

        Ok(false)
    }

    /// Conclude a consensus round
    async fn conclude_consensus_round(&self, round_id: &str) -> Result<()> {
        if let Some((_, mut round)) = self.active_rounds.remove(round_id) {
            info!("Concluding consensus round: {}", round_id);

            // Calculate results
            let total_votes = round.votes.len();
            let approve_votes = round.votes.values().filter(|v| v.vote == VoteDecision::Approve).count();
            let reject_votes = round.votes.values().filter(|v| v.vote == VoteDecision::Reject).count();
            let abstain_votes = round.votes.values().filter(|v| v.vote == VoteDecision::Abstain).count();

            let total_weight: f64 = round.votes.values().map(|v| v.vote_weight).sum();
            let approve_weight: f64 = round.votes.values()
                .filter(|v| v.vote == VoteDecision::Approve)
                .map(|v| v.vote_weight)
                .sum();
            let reject_weight: f64 = round.votes.values()
                .filter(|v| v.vote == VoteDecision::Reject)
                .map(|v| v.vote_weight)
                .sum();

            let approve_ratio = if total_weight > 0.0 { approve_weight / total_weight } else { 0.0 };
            let threshold_met = approve_ratio >= round.proposal.required_threshold;

            let decision = if threshold_met {
                VoteDecision::Approve
            } else {
                VoteDecision::Reject
            };

            let result = ConsensusResult {
                decision: decision.clone(),
                total_votes,
                approve_votes,
                reject_votes,
                abstain_votes,
                total_weight,
                approve_weight,
                reject_weight,
                threshold_met,
                finalized_at: Utc::now(),
            };

            round.result = Some(result);
            round.status = RoundStatus::Completed;
            round.completed_at = Some(Utc::now());

            // Update statistics
            let mut stats = self.stats.write().await;
            stats.completed_rounds += 1;
            stats.active_rounds = self.active_rounds.len();
            
            let duration = round.completed_at.unwrap() - round.started_at;
            let duration_seconds = duration.num_seconds() as f64;
            stats.average_consensus_time_seconds = 
                (stats.average_consensus_time_seconds * (stats.completed_rounds - 1) as f64 + duration_seconds) 
                / stats.completed_rounds as f64;

            let participation_rate = round.participating_nodes.len() as f64 / total_votes as f64;
            stats.average_participation_rate = 
                (stats.average_participation_rate * (stats.completed_rounds - 1) as f64 + participation_rate) 
                / stats.completed_rounds as f64;

            // Store completed round
            let mut completed_rounds = self.completed_rounds.write().await;
            completed_rounds.push(round.clone());

            // Broadcast result
            self.broadcast_consensus_result(&round).await?;

            info!("✅ Consensus round concluded: {} -> {:?}", round_id, decision);
        }

        Ok(())
    }

    /// Broadcast proposal to all participating nodes
    async fn broadcast_proposal(&self, proposal: &ConsensusProposal) -> Result<()> {
        // In a real implementation, this would send the proposal to all connected nodes
        // For now, we'll log the broadcast
        debug!("Broadcasting consensus proposal: {}", proposal.proposal_id);
        Ok(())
    }

    /// Broadcast consensus result to all participating nodes
    async fn broadcast_consensus_result(&self, round: &ConsensusRound) -> Result<()> {
        // In a real implementation, this would send the result to all participating nodes
        debug!("Broadcasting consensus result for round: {}", round.round_id);
        Ok(())
    }

    /// Start background services
    async fn start_background_services(&self) -> Result<()> {
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel(1);
        *self.shutdown_tx.lock().await = Some(shutdown_tx);

        // Round expiration service
        let active_rounds = Arc::clone(&self.active_rounds);
        let stats = Arc::clone(&self.stats);
        let mut shutdown_rx_expiry = shutdown_rx.resubscribe();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
            
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        let now = Utc::now();
                        let mut expired_rounds = Vec::new();

                        // Check for expired rounds
                        for round in active_rounds.iter() {
                            if now > round.proposal.voting_deadline && round.status == RoundStatus::Active {
                                expired_rounds.push(round.round_id.clone());
                            }
                        }

                        // Mark expired rounds
                        for round_id in expired_rounds {
                            if let Some(mut round) = active_rounds.get_mut(&round_id) {
                                round.status = RoundStatus::Expired;
                                round.completed_at = Some(now);
                                
                                let mut stats_guard = stats.write().await;
                                stats_guard.failed_rounds += 1;
                                stats_guard.active_rounds = active_rounds.len();
                                
                                warn!("Consensus round expired: {}", round_id);
                            }
                        }
                    }
                    _ = shutdown_rx_expiry.recv() => break,
                }
            }
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_consensus_bridge_creation() {
        let config = ConsensusConfig {
            enable_consensus_bridge: true,
            min_consensus_nodes: 3,
            consensus_timeout_secs: 60,
            vote_threshold: 0.67,
        };
        
        let bridge = ConsensusBridge::new(config).await.unwrap();
        let stats = bridge.get_stats().await;
        
        assert_eq!(stats.total_proposals, 0);
        assert_eq!(stats.active_rounds, 0);
    }

    #[tokio::test]
    async fn test_proposal_submission() {
        let config = ConsensusConfig {
            enable_consensus_bridge: true,
            min_consensus_nodes: 3,
            consensus_timeout_secs: 60,
            vote_threshold: 0.67,
        };
        
        let bridge = ConsensusBridge::new(config).await.unwrap();
        
        let proposal = ConsensusProposal {
            proposal_id: "test-proposal-1".to_string(),
            proposer_node: "node-1".to_string(),
            proposal_type: ProposalType::NetworkParameter,
            content: serde_json::json!({"parameter": "block_size", "value": 2048}),
            created_at: Utc::now(),
            voting_deadline: Utc::now() + chrono::Duration::hours(1),
            minimum_votes: 3,
            required_threshold: 0.67,
            metadata: HashMap::new(),
        };

        let result = bridge.submit_proposal(proposal).await.unwrap();
        assert_eq!(result, "test-proposal-1");
        
        let active_rounds = bridge.get_active_rounds().await;
        assert_eq!(active_rounds.len(), 1);
    }
}
