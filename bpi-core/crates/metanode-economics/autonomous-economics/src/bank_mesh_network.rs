/*!
# Bank Mesh Network Module

Implements inter-bank communication protocols, settlement network coordination,
economic consensus mechanisms, and multi-bank liquidity sharing for the
Bank Mesh autonomous economic system.

## Features

- Inter-bank communication and coordination
- Settlement network protocols
- Economic consensus mechanisms
- Multi-bank liquidity sharing
- Distributed economic governance
- Cross-bank transaction routing
*/

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use thiserror::Error;
use tracing::{info, warn, error};
use futures::stream::{SplitSink, SplitStream};
use futures::{StreamExt, TryStreamExt, SinkExt};
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream, connect_async};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio::net::TcpStream;

use crate::{EconomicsError, TokenSupplyState};
use billing_meter::TokenType;

/// Bank mesh network errors
#[derive(Error, Debug)]
pub enum BankMeshError {
    #[error("Bank not found: {0}")]
    BankNotFound(Uuid),
    #[error("Network connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Consensus not reached: required {required}, got {actual}")]
    ConsensusNotReached { required: u32, actual: u32 },
    #[error("Invalid message format: {0}")]
    InvalidMessage(String),
    #[error("Settlement failed: {0}")]
    SettlementFailed(String),
    #[error("Liquidity sharing denied: {0}")]
    LiquiditySharingDenied(String),
    #[error("Economics error: {0}")]
    Economics(#[from] EconomicsError),
}

/// Bank node in the mesh network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankNode {
    pub id: Uuid,
    pub name: String,
    pub endpoint: String,
    pub public_key: String,
    pub stake_amount: Decimal,
    pub reputation_score: Decimal,
    pub last_seen: DateTime<Utc>,
    pub status: BankStatus,
    pub supported_tokens: Vec<TokenType>,
    pub liquidity_pools: HashMap<TokenType, Decimal>,
}

/// Bank status in the network
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BankStatus {
    Active,
    Inactive,
    Suspended,
    Slashed,
    Joining,
    Leaving,
}

/// Inter-bank message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BankMessage {
    Heartbeat {
        bank_id: Uuid,
        timestamp: DateTime<Utc>,
        status: BankStatus,
    },
    LiquidityRequest {
        request_id: Uuid,
        requesting_bank: Uuid,
        token_type: TokenType,
        amount: Decimal,
        interest_rate: Decimal,
        duration: Duration,
    },
    LiquidityOffer {
        request_id: Uuid,
        offering_bank: Uuid,
        token_type: TokenType,
        amount: Decimal,
        interest_rate: Decimal,
        conditions: Vec<String>,
    },
    SettlementProposal {
        proposal_id: Uuid,
        proposing_bank: Uuid,
        transactions: Vec<InterBankTransaction>,
        settlement_time: DateTime<Utc>,
    },
    ConsensusVote {
        proposal_id: Uuid,
        voting_bank: Uuid,
        vote: ConsensusVote,
        stake_weight: Decimal,
    },
    EconomicUpdate {
        bank_id: Uuid,
        metrics: EconomicMetrics,
        timestamp: DateTime<Utc>,
    },
}

/// Consensus vote types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConsensusVote {
    Approve,
    Reject,
    Abstain,
}

/// Inter-bank transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterBankTransaction {
    pub id: Uuid,
    pub from_bank: Uuid,
    pub to_bank: Uuid,
    pub token_type: TokenType,
    pub amount: Decimal,
    pub fee: Decimal,
    pub purpose: TransactionPurpose,
    pub created_at: DateTime<Utc>,
    pub settlement_deadline: DateTime<Utc>,
}

/// Transaction purpose types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionPurpose {
    LiquiditySharing,
    Settlement,
    Rebalancing,
    FeeDistribution,
    Governance,
}

/// Economic metrics shared between banks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicMetrics {
    pub total_liquidity: Decimal,
    pub active_users: u64,
    pub transaction_volume: Decimal,
    pub revenue: Decimal,
    pub utilization_rate: Decimal,
    pub risk_score: Decimal,
}

/// Consensus proposal for network decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusProposal {
    pub id: Uuid,
    pub proposer: Uuid,
    pub proposal_type: ProposalType,
    pub description: String,
    pub votes: HashMap<Uuid, (ConsensusVote, Decimal)>, // Bank ID -> (Vote, Stake Weight)
    pub created_at: DateTime<Utc>,
    pub voting_deadline: DateTime<Utc>,
    pub execution_time: Option<DateTime<Utc>>,
    pub status: ProposalStatus,
}

/// Proposal types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalType {
    ParameterChange { parameter: String, new_value: String },
    BankAdmission { candidate_bank: Uuid },
    BankSuspension { target_bank: Uuid, reason: String },
    LiquidityPoolRebalancing { pools: Vec<Uuid> },
    NetworkUpgrade { version: String, features: Vec<String> },
}

/// Proposal status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProposalStatus {
    Active,
    Approved,
    Rejected,
    Executed,
    Expired,
}

/// Liquidity sharing agreement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquiditySharingAgreement {
    pub id: Uuid,
    pub lender_bank: Uuid,
    pub borrower_bank: Uuid,
    pub token_type: TokenType,
    pub amount: Decimal,
    pub interest_rate: Decimal,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub collateral_required: Decimal,
    pub status: AgreementStatus,
}

/// Agreement status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgreementStatus {
    Pending,
    Active,
    Completed,
    Defaulted,
    Cancelled,
}

/// Bank mesh network configuration
#[derive(Debug, Clone)]
pub struct BankMeshConfig {
    pub min_stake_requirement: Decimal,
    pub consensus_threshold: Decimal, // Percentage of stake required for consensus
    pub heartbeat_interval: Duration,
    pub settlement_batch_size: u32,
    pub max_liquidity_share_ratio: Decimal,
    pub reputation_decay_rate: Decimal,
    pub slashing_penalty: Decimal,
}

impl Default for BankMeshConfig {
    fn default() -> Self {
        Self {
            min_stake_requirement: Decimal::from(100000),
            consensus_threshold: Decimal::from_str_exact("0.67").unwrap(), // 67%
            heartbeat_interval: Duration::seconds(30),
            settlement_batch_size: 100,
            max_liquidity_share_ratio: Decimal::from_str_exact("0.3").unwrap(), // 30%
            reputation_decay_rate: Decimal::from_str_exact("0.01").unwrap(), // 1% per day
            slashing_penalty: Decimal::from_str_exact("0.1").unwrap(), // 10%
        }
    }
}

/// Bank Mesh Network Engine
#[derive(Debug)]
pub struct BankMeshNetwork {
    config: BankMeshConfig,
    local_bank: BankNode,
    connected_banks: Arc<RwLock<HashMap<Uuid, BankNode>>>,
    active_proposals: Arc<RwLock<HashMap<Uuid, ConsensusProposal>>>,
    liquidity_agreements: Arc<RwLock<HashMap<Uuid, LiquiditySharingAgreement>>>,
    pending_settlements: Arc<RwLock<Vec<InterBankTransaction>>>,
    network_metrics: Arc<RwLock<HashMap<Uuid, EconomicMetrics>>>,
    message_handlers: Arc<RwLock<HashMap<Uuid, tokio::sync::mpsc::UnboundedSender<BankMessage>>>>,
}

impl BankMeshNetwork {
    /// Create new bank mesh network
    pub fn new(config: BankMeshConfig, local_bank: BankNode) -> Self {
        info!("Initialized Bank Mesh Network for bank {}", local_bank.name);
        
        Self {
            config,
            local_bank,
            connected_banks: Arc::new(RwLock::new(HashMap::new())),
            active_proposals: Arc::new(RwLock::new(HashMap::new())),
            liquidity_agreements: Arc::new(RwLock::new(HashMap::new())),
            pending_settlements: Arc::new(RwLock::new(Vec::new())),
            network_metrics: Arc::new(RwLock::new(HashMap::new())),
            message_handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Join the bank mesh network
    pub async fn join_network(&mut self, bootstrap_nodes: Vec<String>) -> Result<(), BankMeshError> {
        info!("Joining bank mesh network with {} bootstrap nodes", bootstrap_nodes.len());
        
        // Connect to bootstrap nodes
        for node_endpoint in bootstrap_nodes {
            match self.connect_to_bank(&node_endpoint).await {
                Ok(_) => info!("Connected to bootstrap node: {}", node_endpoint),
                Err(e) => warn!("Failed to connect to bootstrap node {}: {}", node_endpoint, e),
            }
        }
        
        // Start heartbeat service
        self.start_heartbeat_service().await;
        
        // Announce joining to the network
        let join_message = BankMessage::Heartbeat {
            bank_id: self.local_bank.id,
            timestamp: Utc::now(),
            status: BankStatus::Joining,
        };
        
        self.broadcast_message(join_message).await?;
        
        Ok(())
    }

    /// Connect to a specific bank
    async fn connect_to_bank(&self, endpoint: &str) -> Result<(), BankMeshError> {
        let url = format!("ws://{}/bank-mesh", endpoint);
        
        match connect_async(&url).await {
            Ok((ws_stream, _)) => {
                let (sink, stream) = ws_stream.split();
                
                // Start message handler for this connection
                let bank_id = Uuid::new_v4(); // Would get actual bank ID from handshake
                self.start_message_handler(bank_id, sink, stream).await;
                
                Ok(())
            },
            Err(e) => Err(BankMeshError::ConnectionFailed(format!("WebSocket connection failed: {}", e))),
        }
    }

    /// Start message handler for a bank connection
    async fn start_message_handler(
        &self,
        bank_id: Uuid,
        mut sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        mut stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ) {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<BankMessage>();
        self.message_handlers.write().await.insert(bank_id, tx);
        
        let connected_banks = Arc::clone(&self.connected_banks);
        let active_proposals = Arc::clone(&self.active_proposals);
        let liquidity_agreements = Arc::clone(&self.liquidity_agreements);
        let network_metrics = Arc::clone(&self.network_metrics);
        
        // Spawn sender task
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                let serialized = serde_json::to_string(&message).unwrap_or_default();
                if sink.send(Message::Text(serialized)).await.is_err() {
                    break;
                }
            }
        });
        
        // Spawn receiver task
        tokio::spawn(async move {
            while let Some(message) = stream.next().await {
                if let Ok(Message::Text(text)) = message {
                    if let Ok(bank_message) = serde_json::from_str::<BankMessage>(&text) {
                        match bank_message {
                            BankMessage::Heartbeat { bank_id: sender_id, timestamp, status } => {
                                // Update bank status
                                let mut banks = connected_banks.write().await;
                                if let Some(bank) = banks.get_mut(&sender_id) {
                                    bank.status = status;
                                    bank.last_seen = timestamp;
                                }
                            },
                            BankMessage::LiquidityRequest { request_id, requesting_bank, token_type, amount, interest_rate, duration } => {
                                // Handle liquidity request
                                info!("Received liquidity request {} from bank {}", request_id, requesting_bank);
                            },
                            BankMessage::ConsensusVote { proposal_id, voting_bank, vote, stake_weight } => {
                                // Handle consensus vote
                                let mut proposals = active_proposals.write().await;
                                if let Some(proposal) = proposals.get_mut(&proposal_id) {
                                    proposal.votes.insert(voting_bank, (vote, stake_weight));
                                }
                            },
                            BankMessage::EconomicUpdate { bank_id: sender_id, metrics, timestamp: _ } => {
                                // Update economic metrics
                                network_metrics.write().await.insert(sender_id, metrics);
                            },
                            _ => {
                                // Handle other message types
                            }
                        }
                    }
                }
            }
        });
    }

    /// Start heartbeat service
    async fn start_heartbeat_service(&self) {
        let heartbeat_interval = self.config.heartbeat_interval;
        let local_bank_id = self.local_bank.id;
        let message_handlers = Arc::clone(&self.message_handlers);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(heartbeat_interval.to_std().unwrap());
            
            loop {
                interval.tick().await;
                
                let heartbeat = BankMessage::Heartbeat {
                    bank_id: local_bank_id,
                    timestamp: Utc::now(),
                    status: BankStatus::Active,
                };
                
                let handlers = message_handlers.read().await;
                for (_, sender) in handlers.iter() {
                    let _ = sender.send(heartbeat.clone());
                }
            }
        });
    }

    /// Broadcast message to all connected banks
    async fn broadcast_message(&self, message: BankMessage) -> Result<(), BankMeshError> {
        let handlers = self.message_handlers.read().await;
        
        for (bank_id, sender) in handlers.iter() {
            if sender.send(message.clone()).is_err() {
                warn!("Failed to send message to bank {}", bank_id);
            }
        }
        
        Ok(())
    }

    /// Request liquidity from the network
    pub async fn request_liquidity(
        &self,
        token_type: TokenType,
        amount: Decimal,
        max_interest_rate: Decimal,
        duration: Duration,
    ) -> Result<Uuid, BankMeshError> {
        let request_id = Uuid::new_v4();
        
        let request = BankMessage::LiquidityRequest {
            request_id,
            requesting_bank: self.local_bank.id,
            token_type,
            amount,
            interest_rate: max_interest_rate,
            duration,
        };
        
        self.broadcast_message(request).await?;
        info!("Requesting {} {:?} tokens at max interest rate {}", amount, token_type, max_interest_rate);
        
        Ok(request_id)
    }

    /// Offer liquidity to a request
    pub async fn offer_liquidity(
        &self,
        request_id: Uuid,
        token_type: TokenType,
        amount: Decimal,
        interest_rate: Decimal,
        conditions: Vec<String>,
    ) -> Result<(), BankMeshError> {
        let offer = BankMessage::LiquidityOffer {
            request_id,
            offering_bank: self.local_bank.id,
            token_type,
            amount,
            interest_rate,
            conditions,
        };
        
        self.broadcast_message(offer).await?;
        info!("Provided {} {:?} tokens at {}% interest for request {}", amount, token_type, interest_rate, request_id);
        
        Ok(())
    }

    /// Create consensus proposal
    pub async fn create_proposal(
        &self,
        proposal_type: ProposalType,
        description: String,
        voting_duration: Duration,
    ) -> Result<Uuid, BankMeshError> {
        let proposal_id = Uuid::new_v4();
        let voting_deadline = Utc::now() + voting_duration;
        
        let proposal = ConsensusProposal {
            id: proposal_id,
            proposer: self.local_bank.id,
            proposal_type,
            description,
            votes: HashMap::new(),
            created_at: Utc::now(),
            voting_deadline,
            execution_time: None,
            status: ProposalStatus::Active,
        };
        
        self.active_proposals.write().await.insert(proposal_id, proposal);
        info!("Created consensus proposal {}", proposal_id);
        
        Ok(proposal_id)
    }

    /// Vote on a consensus proposal
    pub async fn vote_on_proposal(
        &self,
        proposal_id: Uuid,
        vote: ConsensusVote,
    ) -> Result<(), BankMeshError> {
        let vote_message = BankMessage::ConsensusVote {
            proposal_id,
            voting_bank: self.local_bank.id,
            vote: vote.clone(),
            stake_weight: self.local_bank.stake_amount,
        };
        
        self.broadcast_message(vote_message).await?;
        
        // Update local proposal
        let mut proposals = self.active_proposals.write().await;
        if let Some(proposal) = proposals.get_mut(&proposal_id) {
            proposal.votes.insert(self.local_bank.id, (vote, self.local_bank.stake_amount));
        }
        
        info!("Voted on proposal {}", proposal_id);
        Ok(())
    }

    /// Check consensus on proposals
    pub async fn check_consensus(&self) -> Result<Vec<Uuid>, BankMeshError> {
        let mut proposals = self.active_proposals.write().await;
        let mut approved_proposals = Vec::new();
        
        let now = Utc::now();
        let total_stake: Decimal = self.connected_banks.read().await.values()
            .map(|bank| bank.stake_amount)
            .sum::<Decimal>() + self.local_bank.stake_amount;
        
        for (proposal_id, proposal) in proposals.iter_mut() {
            if proposal.status != ProposalStatus::Active || now < proposal.voting_deadline {
                continue;
            }
            
            let total_votes: Decimal = proposal.votes.values().map(|(_, weight)| *weight).sum();
            let approve_votes: Decimal = proposal.votes.values()
                .filter(|(vote, _)| *vote == ConsensusVote::Approve)
                .map(|(_, weight)| *weight)
                .sum();
            
            let approval_ratio = if total_stake > Decimal::ZERO {
                approve_votes / total_stake
            } else {
                Decimal::ZERO
            };
            
            if approval_ratio >= self.config.consensus_threshold {
                proposal.status = ProposalStatus::Approved;
                approved_proposals.push(*proposal_id);
                info!("Proposal {} approved with {:.2}% consensus", proposal_id, approval_ratio * Decimal::from(100));
            } else {
                proposal.status = ProposalStatus::Rejected;
                info!("Proposal {} rejected with {:.2}% consensus", proposal_id, approval_ratio * Decimal::from(100));
            }
        }
        
        Ok(approved_proposals)
    }

    /// Process settlement batch
    pub async fn process_settlement_batch(&self) -> Result<u32, BankMeshError> {
        let mut pending = self.pending_settlements.write().await;
        let batch_size = self.config.settlement_batch_size as usize;
        let batch_size = batch_size.min(pending.len());
        let batch: Vec<InterBankTransaction> = pending.drain(..batch_size).collect();
        
        if batch.is_empty() {
            return Ok(0);
        }
        
        let proposal_id = Uuid::new_v4();
        let settlement_proposal = BankMessage::SettlementProposal {
            proposal_id,
            proposing_bank: self.local_bank.id,
            transactions: batch.clone(),
            settlement_time: Utc::now() + Duration::minutes(5),
        };
        
        self.broadcast_message(settlement_proposal).await?;
        info!("Proposed settlement batch with {} transactions", batch.len());
        
        Ok(batch.len() as u32)
    }

    /// Get network statistics
    pub async fn get_network_stats(&self) -> HashMap<String, serde_json::Value> {
        let connected_banks = self.connected_banks.read().await;
        let active_proposals = self.active_proposals.read().await;
        let liquidity_agreements = self.liquidity_agreements.read().await;
        let pending_settlements = self.pending_settlements.read().await;
        
        let mut stats = HashMap::new();
        stats.insert("connected_banks".to_string(), serde_json::Value::Number(connected_banks.len().into()));
        stats.insert("active_proposals".to_string(), serde_json::Value::Number(active_proposals.len().into()));
        stats.insert("liquidity_agreements".to_string(), serde_json::Value::Number(liquidity_agreements.len().into()));
        stats.insert("pending_settlements".to_string(), serde_json::Value::Number(pending_settlements.len().into()));
        
        let total_network_stake: Decimal = connected_banks.values()
            .map(|bank| bank.stake_amount)
            .sum::<Decimal>() + self.local_bank.stake_amount;
        
        let active_banks = connected_banks.values()
            .filter(|bank| bank.status == BankStatus::Active)
            .count();
        
        stats.insert("total_network_stake".to_string(), serde_json::Value::String(total_network_stake.to_string()));
        stats.insert("active_banks".to_string(), serde_json::Value::Number(active_banks.into()));
        
        stats
    }

    /// Get connected bank by ID
    pub async fn get_bank(&self, bank_id: Uuid) -> Option<BankNode> {
        self.connected_banks.read().await.get(&bank_id).cloned()
    }

    /// Get active proposal by ID
    pub async fn get_proposal(&self, proposal_id: Uuid) -> Option<ConsensusProposal> {
        self.active_proposals.read().await.get(&proposal_id).cloned()
    }

    /// Get liquidity agreement by ID
    pub async fn get_agreement(&self, agreement_id: Uuid) -> Option<LiquiditySharingAgreement> {
        self.liquidity_agreements.read().await.get(&agreement_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_bank() -> BankNode {
        BankNode {
            id: Uuid::new_v4(),
            name: "Test Bank".to_string(),
            endpoint: "localhost:8080".to_string(),
            public_key: "test_public_key".to_string(),
            stake_amount: Decimal::from(1000000),
            reputation_score: Decimal::from_str_exact("0.95").unwrap(),
            last_seen: Utc::now(),
            status: BankStatus::Active,
            supported_tokens: vec![TokenType::Genesis, TokenType::Nexus],
            liquidity_pools: HashMap::new(),
        }
    }

    #[tokio::test]
    async fn test_bank_mesh_network_creation() {
        let config = BankMeshConfig::default();
        let bank = create_test_bank();
        let network = BankMeshNetwork::new(config, bank);
        
        let stats = network.get_network_stats().await;
        assert_eq!(stats.get("connected_banks").unwrap(), &serde_json::Value::Number(0.into()));
    }

    #[tokio::test]
    async fn test_consensus_proposal_creation() {
        let config = BankMeshConfig::default();
        let bank = create_test_bank();
        let network = BankMeshNetwork::new(config, bank);
        
        let proposal_id = network.create_proposal(
            ProposalType::ParameterChange {
                parameter: "min_stake".to_string(),
                new_value: "200000".to_string(),
            },
            "Increase minimum stake requirement".to_string(),
            Duration::hours(24),
        ).await.unwrap();
        
        let proposal = network.get_proposal(proposal_id).await;
        assert!(proposal.is_some());
        assert_eq!(proposal.unwrap().status, ProposalStatus::Active);
    }

    #[tokio::test]
    async fn test_liquidity_request() {
        let config = BankMeshConfig::default();
        let bank = create_test_bank();
        let network = BankMeshNetwork::new(config, bank);
        
        let request_id = network.request_liquidity(
            TokenType::Genesis,
            Decimal::from(10000),
            Decimal::from_str_exact("0.05").unwrap(),
            Duration::days(30),
        ).await.unwrap();
        
        assert!(!request_id.is_nil());
    }

    #[tokio::test]
    async fn test_voting_on_proposal() {
        let config = BankMeshConfig::default();
        let bank = create_test_bank();
        let bank_id = bank.id;
        let network = BankMeshNetwork::new(config, bank);
        
        let proposal_id = network.create_proposal(
            ProposalType::BankAdmission {
                candidate_bank: Uuid::new_v4(),
            },
            "Admit new bank to network".to_string(),
            Duration::hours(24),
        ).await.unwrap();
        
        let result = network.vote_on_proposal(proposal_id, ConsensusVote::Approve).await;
        assert!(result.is_ok());
        
        let proposal = network.get_proposal(proposal_id).await.unwrap();
        assert!(proposal.votes.contains_key(&bank_id));
    }

    #[tokio::test]
    async fn test_settlement_batch_processing() {
        let config = BankMeshConfig::default();
        let bank = create_test_bank();
        let network = BankMeshNetwork::new(config, bank);
        
        // Add some pending transactions
        let transaction = InterBankTransaction {
            id: Uuid::new_v4(),
            from_bank: Uuid::new_v4(),
            to_bank: Uuid::new_v4(),
            token_type: TokenType::Genesis,
            amount: Decimal::from(1000),
            fee: Decimal::from(10),
            purpose: TransactionPurpose::Settlement,
            created_at: Utc::now(),
            settlement_deadline: Utc::now() + Duration::hours(1),
        };
        
        network.pending_settlements.write().await.push(transaction);
        
        let processed = network.process_settlement_batch().await.unwrap();
        assert_eq!(processed, 1);
    }

    #[tokio::test]
    async fn test_network_statistics() {
        let config = BankMeshConfig::default();
        let bank = create_test_bank();
        let network = BankMeshNetwork::new(config, bank);
        
        let stats = network.get_network_stats().await;
        assert!(stats.contains_key("connected_banks"));
        assert!(stats.contains_key("total_network_stake"));
        assert!(stats.contains_key("active_banks"));
    }
}
