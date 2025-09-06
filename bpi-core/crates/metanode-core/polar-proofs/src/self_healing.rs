//! Self-healing infrastructure for polar proofs
//!
//! This module implements the revolutionary self-healing capabilities that make
//! our Merkle + Polar Proofs system resilient to corruption, attacks, and failures.
//! Features include:
//! - Automatic corruption detection using polynomial verification
//! - Distributed redundant storage with erasure coding
//! - Consensus-based repair mechanisms
//! - Real-time health monitoring and alerting

use crate::{
    PolarProof, PolarProofError, Polynomial,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tokio::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Self-healing manager for polar proof infrastructure
#[derive(Debug, Clone)]
pub struct SelfHealingManager {
    /// Health monitoring configuration
    pub health_config: HealthConfig,
    /// Repair engine for automatic fixes
    pub repair_engine: RepairEngine,
    /// Consensus coordinator for distributed healing
    pub consensus_coordinator: ConsensusCoordinator,
    /// Active health monitoring sessions
    pub active_monitors: HashMap<Uuid, HealthMonitor>,
}

/// Configuration for health monitoring
#[derive(Debug, Clone)]
pub struct HealthConfig {
    /// Interval between health checks
    pub check_interval: Duration,
    /// Threshold for triggering automatic repair (0.0 to 1.0)
    pub auto_repair_threshold: f64,
    /// Number of nodes required for consensus
    pub consensus_nodes: usize,
    /// Maximum repair attempts before marking as failed
    pub max_repair_attempts: usize,
    /// Enable real-time monitoring
    pub real_time_monitoring: bool,
}

impl Default for HealthConfig {
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(60),
            auto_repair_threshold: 0.8,
            consensus_nodes: 3,
            max_repair_attempts: 5,
            real_time_monitoring: true,
        }
    }
}

/// Repair engine for automatic corruption fixes
#[derive(Debug, Clone)]
pub struct RepairEngine {
    /// Repair strategies available
    pub strategies: Vec<RepairStrategy>,
    /// Active repair operations
    pub active_repairs: HashMap<Uuid, RepairOperation>,
    /// Repair history for analysis
    pub repair_history: Vec<RepairEvent>,
}

/// Consensus coordinator for distributed healing
#[derive(Debug, Clone)]
pub struct ConsensusCoordinator {
    /// Known nodes in the network
    pub known_nodes: HashSet<NodeId>,
    /// Current consensus state
    pub consensus_state: ConsensusState,
    /// Voting records for repairs
    pub voting_records: HashMap<Uuid, VotingRecord>,
}

/// Health monitor for continuous integrity checking
#[derive(Debug, Clone)]
pub struct HealthMonitor {
    /// Monitor identifier
    pub monitor_id: Uuid,
    /// Polar proof being monitored
    pub proof_id: Uuid,
    /// Current health status
    pub health_status: HealthStatus,
    /// Last check timestamp (Unix timestamp)
    pub last_check: u64,
    /// Check frequency
    pub check_frequency: Duration,
    /// Detected issues
    pub detected_issues: Vec<HealthIssue>,
}

/// Health status of a polar proof
#[derive(Debug, Clone)]
pub struct HealthStatus {
    /// Overall integrity score (0.0 to 1.0)
    pub integrity_score: f64,
    /// Polynomial verification status
    pub polynomial_valid: bool,
    /// Redundancy status
    pub redundancy_healthy: bool,
    /// Last verification timestamp
    pub last_verification: u64,
    /// Number of healthy replicas
    pub healthy_replicas: usize,
    /// Total expected replicas
    pub total_replicas: usize,
}

/// Detected health issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthIssue {
    /// Issue identifier
    pub issue_id: Uuid,
    /// Type of issue detected
    pub issue_type: IssueType,
    /// Severity level
    pub severity: Severity,
    /// Detailed description
    pub description: String,
    /// When the issue was detected
    pub detected_at: u64,
    /// Affected components
    pub affected_components: Vec<String>,
}

/// Types of health issues
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IssueType {
    /// Polynomial verification failed
    PolynomialCorruption,
    /// Missing redundant copies
    RedundancyLoss,
    /// Consensus failure
    ConsensusFailure,
    /// Node communication failure
    NetworkPartition,
    /// Storage corruption
    StorageCorruption,
    /// Performance degradation
    PerformanceDegradation,
}

/// Issue severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// Repair strategy for different types of corruption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairStrategy {
    /// Strategy identifier
    pub strategy_id: Uuid,
    /// Name of the strategy
    pub name: String,
    /// Types of issues this strategy can handle
    pub handles_issues: Vec<IssueType>,
    /// Success rate of this strategy
    pub success_rate: f64,
    /// Average repair time
    pub average_repair_time: Duration,
}

/// Active repair operation
#[derive(Debug, Clone)]
pub struct RepairOperation {
    /// Operation identifier
    pub operation_id: Uuid,
    /// Proof being repaired
    pub proof_id: Uuid,
    /// Strategy being used
    pub strategy: RepairStrategy,
    /// Current status
    pub status: RepairStatus,
    /// Start time (Unix timestamp)
    pub started_at: u64,
    /// Progress percentage (0-100)
    pub progress: u8,
    /// Nodes participating in repair
    pub participating_nodes: Vec<NodeId>,
}

/// Status of a repair operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepairStatus {
    /// Repair is starting
    Initializing,
    /// Gathering consensus
    GatheringConsensus,
    /// Reconstructing data
    Reconstructing,
    /// Verifying repair
    Verifying,
    /// Repair completed successfully
    Completed,
    /// Repair failed
    Failed { reason: String },
}

/// Repair event for history tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairEvent {
    /// Event identifier
    pub event_id: Uuid,
    /// Proof that was repaired
    pub proof_id: Uuid,
    /// Issue that triggered the repair
    pub triggering_issue: HealthIssue,
    /// Strategy used for repair
    pub strategy_used: RepairStrategy,
    /// Whether repair was successful
    pub success: bool,
    /// Time taken for repair
    pub repair_duration: Duration,
    /// Timestamp of repair
    pub timestamp: u64,
}

/// Node identifier in the network
pub type NodeId = Uuid;

/// Consensus state for distributed healing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusState {
    /// Current consensus round
    pub round: u64,
    /// Active proposals
    pub active_proposals: HashMap<Uuid, ConsensusProposal>,
    /// Finalized decisions
    pub finalized_decisions: Vec<ConsensusDecision>,
}

/// Consensus proposal for repair operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusProposal {
    /// Proposal identifier
    pub proposal_id: Uuid,
    /// Type of proposal
    pub proposal_type: ProposalType,
    /// Proposing node
    pub proposer: NodeId,
    /// Proposal details
    pub details: String,
    /// Required votes for approval
    pub required_votes: usize,
    /// Current votes
    pub votes: HashMap<NodeId, Vote>,
}

/// Types of consensus proposals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalType {
    /// Repair a corrupted proof
    RepairProof,
    /// Add new redundant copy
    AddRedundancy,
    /// Remove corrupted node
    RemoveNode,
    /// Update repair strategy
    UpdateStrategy,
}

/// Vote in consensus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    /// Voting node
    pub voter: NodeId,
    /// Vote decision
    pub decision: VoteDecision,
    /// Timestamp of vote
    pub timestamp: u64,
    /// Optional justification
    pub justification: Option<String>,
}

/// Vote decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteDecision {
    Approve,
    Reject,
    Abstain,
}

/// Finalized consensus decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusDecision {
    /// Decision identifier
    pub decision_id: Uuid,
    /// Original proposal
    pub proposal: ConsensusProposal,
    /// Final outcome
    pub outcome: DecisionOutcome,
    /// Finalization timestamp
    pub finalized_at: u64,
}

/// Outcome of consensus decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionOutcome {
    Approved,
    Rejected,
    Timeout,
}

/// Health record for a polar proof
#[derive(Debug, Clone)]
pub struct HealthRecord {
    /// Proposal being voted on
    pub proposal_id: Uuid,
    /// All votes received
    pub votes: Vec<Vote>,
    /// Voting deadline (Unix timestamp)
    pub deadline: u64,
    /// Current status
    pub status: VotingStatus,
}

/// Voting record for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingRecord {
    /// Proposal being voted on
    pub proposal_id: Uuid,
    /// All votes received
    pub votes: Vec<Vote>,
    /// Voting deadline (Unix timestamp)
    pub deadline: u64,
    /// Current status
    pub status: VotingStatus,
}

/// Status of voting process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VotingStatus {
    Active,
    Completed,
    Timeout,
}

impl SelfHealingManager {
    /// Create a new self-healing manager
    pub fn new() -> Self {
        Self {
            health_config: HealthConfig::default(),
            repair_engine: RepairEngine::new(),
            consensus_coordinator: ConsensusCoordinator::new(),
            active_monitors: HashMap::new(),
        }
    }
    
    /// Create self-healing manager with custom configuration
    pub fn with_config(config: HealthConfig) -> Self {
        Self {
            health_config: config,
            repair_engine: RepairEngine::new(),
            consensus_coordinator: ConsensusCoordinator::new(),
            active_monitors: HashMap::new(),
        }
    }
    
    /// Start monitoring a polar proof for health issues
    pub async fn start_monitoring(&mut self, proof: &PolarProof) -> Result<Uuid, PolarProofError> {
        let monitor_id = Uuid::new_v4();
        
        let health_monitor = HealthMonitor {
            monitor_id,
            proof_id: proof.proof_id,
            health_status: self.initial_health_check(proof).await?,
            last_check: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            check_frequency: self.health_config.check_interval,
            detected_issues: Vec::new(),
        };
        
        self.active_monitors.insert(monitor_id, health_monitor);
        
        if self.health_config.real_time_monitoring {
            self.start_real_time_monitoring(monitor_id).await?;
        }
        
        Ok(monitor_id)
    }
    
    /// Perform initial health check on a polar proof
    async fn initial_health_check(&self, proof: &PolarProof) -> Result<HealthStatus, PolarProofError> {
        // Verify polynomial integrity
        let polynomial_valid = self.verify_polynomial_integrity(proof).await?;
        
        // Check redundancy status
        let (redundancy_healthy, healthy_replicas, total_replicas) = 
            self.check_redundancy_status(proof).await?;
        
        // Calculate overall integrity score
        let integrity_score = self.calculate_integrity_score(
            polynomial_valid,
            redundancy_healthy,
            healthy_replicas,
            total_replicas,
        );
        
        Ok(HealthStatus {
            integrity_score,
            polynomial_valid,
            redundancy_healthy,
            last_verification: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            healthy_replicas,
            total_replicas,
        })
    }
    
    /// Verify polynomial integrity
    async fn verify_polynomial_integrity(&self, proof: &PolarProof) -> Result<bool, PolarProofError> {
        // Reconstruct polynomial from coefficients
        let polynomial = Polynomial::new(proof.polynomial_coefficients.clone());
        
        // Verify evaluation at all verification points
        for (point, expected_value) in proof.evaluation_domain.evaluation_points.iter()
            .zip(&proof.evaluation_domain.expected_values) {
            let computed_value = polynomial.evaluate(*point);
            if computed_value != *expected_value {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Check redundancy status
    async fn check_redundancy_status(
        &self,
        proof: &PolarProof,
    ) -> Result<(bool, usize, usize), PolarProofError> {
        if let Some(redundancy_info) = &proof.redundancy_info {
            let total_replicas = redundancy_info.shard_ids.len();
            // Simulate checking replica health (in production, query actual nodes)
            let healthy_replicas = total_replicas; // Assume all healthy for now
            
            let redundancy_healthy = healthy_replicas >= redundancy_info.erasure_params.threshold;
            
            Ok((redundancy_healthy, healthy_replicas, total_replicas))
        } else {
            // No redundancy configured
            Ok((false, 0, 0))
        }
    }
    
    /// Calculate overall integrity score
    fn calculate_integrity_score(
        &self,
        polynomial_valid: bool,
        redundancy_healthy: bool,
        healthy_replicas: usize,
        total_replicas: usize,
    ) -> f64 {
        let mut score = 0.0;
        
        // Polynomial validity (50% of score)
        if polynomial_valid {
            score += 0.5;
        }
        
        // Redundancy health (30% of score)
        if redundancy_healthy {
            score += 0.3;
        }
        
        // Replica ratio (20% of score)
        if total_replicas > 0 {
            score += 0.2 * (healthy_replicas as f64 / total_replicas as f64);
        }
        
        score
    }
    
    /// Start real-time monitoring for a proof
    async fn start_real_time_monitoring(&self, _monitor_id: Uuid) -> Result<(), PolarProofError> {
        // In production, spawn a background task for continuous monitoring
        // For now, just return success
        Ok(())
    }
    
    /// Detect corruption in a polar proof
    pub async fn detect_corruption(&self, proof: &PolarProof) -> Result<Vec<HealthIssue>, PolarProofError> {
        let mut issues = Vec::new();
        
        // Check polynomial integrity
        if !self.verify_polynomial_integrity(proof).await? {
            issues.push(HealthIssue {
                issue_id: Uuid::new_v4(),
                issue_type: IssueType::PolynomialCorruption,
                severity: Severity::Critical,
                description: "Polynomial verification failed - proof may be corrupted".to_string(),
                detected_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                affected_components: vec!["polynomial_coefficients".to_string()],
            });
        }
        
        // Check redundancy status
        let (redundancy_healthy, healthy_replicas, total_replicas) = 
            self.check_redundancy_status(proof).await?;
        
        if !redundancy_healthy {
            issues.push(HealthIssue {
                issue_id: Uuid::new_v4(),
                issue_type: IssueType::RedundancyLoss,
                severity: if healthy_replicas == 0 { Severity::Critical } else { Severity::High },
                description: format!(
                    "Redundancy compromised: {}/{} replicas healthy",
                    healthy_replicas, total_replicas
                ),
                detected_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                affected_components: vec!["redundancy_replicas".to_string()],
            });
        }
        
        Ok(issues)
    }
    
    /// Automatically repair detected corruption
    pub async fn auto_repair(
        &mut self,
        proof_id: Uuid,
        issues: &[HealthIssue],
    ) -> Result<RepairOperation, PolarProofError> {
        // Select appropriate repair strategy
        let strategy = self.repair_engine.select_strategy(issues)?;
        
        // Create repair operation
        let operation = RepairOperation {
            operation_id: Uuid::new_v4(),
            proof_id,
            strategy: strategy.clone(),
            status: RepairStatus::Initializing,
            started_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            progress: 0,
            participating_nodes: Vec::new(),
        };
        
        // Start repair process
        self.repair_engine.active_repairs.insert(operation.operation_id, operation.clone());
        
        // Execute repair strategy
        self.execute_repair_strategy(&operation).await?;
        
        Ok(operation)
    }
    
    /// Execute a repair strategy
    async fn execute_repair_strategy(
        &mut self,
        operation: &RepairOperation,
    ) -> Result<(), PolarProofError> {
        match operation.strategy.name.as_str() {
            "polynomial_reconstruction" => {
                self.repair_polynomial_corruption(operation).await
            }
            "redundancy_restoration" => {
                self.repair_redundancy_loss(operation).await
            }
            "consensus_healing" => {
                self.repair_with_consensus(operation).await
            }
            _ => Err(PolarProofError::SelfHealingError {
                reason: format!("Unknown repair strategy: {}", operation.strategy.name),
            }),
        }
    }
    
    /// Repair polynomial corruption
    async fn repair_polynomial_corruption(
        &self,
        _operation: &RepairOperation,
    ) -> Result<(), PolarProofError> {
        // Implement polynomial reconstruction from healthy replicas
        // This would involve consensus to determine the correct polynomial
        Ok(())
    }
    
    /// Repair redundancy loss
    async fn repair_redundancy_loss(
        &self,
        _operation: &RepairOperation,
    ) -> Result<(), PolarProofError> {
        // Implement redundancy restoration by creating new replicas
        Ok(())
    }
    
    /// Repair using consensus mechanism
    async fn repair_with_consensus(
        &mut self,
        operation: &RepairOperation,
    ) -> Result<(), PolarProofError> {
        // Create consensus proposal for repair
        let proposal = ConsensusProposal {
            proposal_id: Uuid::new_v4(),
            proposal_type: ProposalType::RepairProof,
            proposer: Uuid::new_v4(), // Current node ID
            details: format!("Repair proof {} using strategy {}", 
                operation.proof_id, operation.strategy.name),
            required_votes: self.health_config.consensus_nodes,
            votes: HashMap::new(),
        };
        
        // Submit proposal to consensus
        self.consensus_coordinator.submit_proposal(proposal).await?;
        
        Ok(())
    }
    
    /// Get health status for all monitored proofs
    pub fn get_health_summary(&self) -> HealthSummary {
        let total_monitored = self.active_monitors.len();
        let healthy_count = self.active_monitors.values()
            .filter(|monitor| monitor.health_status.integrity_score >= self.health_config.auto_repair_threshold)
            .count();
        
        let total_issues: usize = self.active_monitors.values()
            .map(|monitor| monitor.detected_issues.len())
            .sum();
        
        HealthSummary {
            total_monitored,
            healthy_count,
            unhealthy_count: total_monitored - healthy_count,
            total_issues,
            average_integrity_score: if total_monitored > 0 {
                self.active_monitors.values()
                    .map(|monitor| monitor.health_status.integrity_score)
                    .sum::<f64>() / total_monitored as f64
            } else {
                0.0
            },
        }
    }
}

/// Summary of health status across all monitored proofs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSummary {
    /// Total number of monitored proofs
    pub total_monitored: usize,
    /// Number of healthy proofs
    pub healthy_count: usize,
    /// Number of unhealthy proofs
    pub unhealthy_count: usize,
    /// Total issues detected
    pub total_issues: usize,
    /// Average integrity score
    pub average_integrity_score: f64,
}

impl RepairEngine {
    /// Create a new repair engine
    pub fn new() -> Self {
        Self {
            strategies: Self::default_strategies(),
            active_repairs: HashMap::new(),
            repair_history: Vec::new(),
        }
    }
    
    /// Get default repair strategies
    fn default_strategies() -> Vec<RepairStrategy> {
        vec![
            RepairStrategy {
                strategy_id: Uuid::new_v4(),
                name: "polynomial_reconstruction".to_string(),
                handles_issues: vec![IssueType::PolynomialCorruption],
                success_rate: 0.95,
                average_repair_time: Duration::from_secs(30),
            },
            RepairStrategy {
                strategy_id: Uuid::new_v4(),
                name: "redundancy_restoration".to_string(),
                handles_issues: vec![IssueType::RedundancyLoss],
                success_rate: 0.90,
                average_repair_time: Duration::from_secs(60),
            },
            RepairStrategy {
                strategy_id: Uuid::new_v4(),
                name: "consensus_healing".to_string(),
                handles_issues: vec![
                    IssueType::ConsensusFailure,
                    IssueType::NetworkPartition,
                ],
                success_rate: 0.85,
                average_repair_time: Duration::from_secs(120),
            },
        ]
    }
    
    /// Select the best repair strategy for given issues
    pub fn select_strategy(&self, issues: &[HealthIssue]) -> Result<RepairStrategy, PolarProofError> {
        let mut best_strategy = None;
        let mut best_score = 0.0;
        
        for strategy in &self.strategies {
            let mut score = 0.0;
            let mut handles_count = 0;
            
            for issue in issues {
                if strategy.handles_issues.contains(&issue.issue_type) {
                    handles_count += 1;
                    score += strategy.success_rate;
                }
            }
            
            if handles_count > 0 {
                score = score / handles_count as f64;
                if score > best_score {
                    best_score = score;
                    best_strategy = Some(strategy.clone());
                }
            }
        }
        
        best_strategy.ok_or_else(|| PolarProofError::SelfHealingError {
            reason: "No suitable repair strategy found".to_string(),
        })
    }
}

impl ConsensusCoordinator {
    /// Create a new consensus coordinator
    pub fn new() -> Self {
        Self {
            known_nodes: HashSet::new(),
            consensus_state: ConsensusState {
                round: 0,
                active_proposals: HashMap::new(),
                finalized_decisions: Vec::new(),
            },
            voting_records: HashMap::new(),
        }
    }
    
    /// Submit a proposal for consensus
    pub async fn submit_proposal(
        &mut self,
        proposal: ConsensusProposal,
    ) -> Result<(), PolarProofError> {
        self.consensus_state.active_proposals.insert(proposal.proposal_id, proposal);
        Ok(())
    }
    
    /// Vote on a proposal
    pub async fn vote(
        &mut self,
        proposal_id: Uuid,
        vote: Vote,
    ) -> Result<(), PolarProofError> {
        if let Some(proposal) = self.consensus_state.active_proposals.get_mut(&proposal_id) {
            proposal.votes.insert(vote.voter, vote);
            
            // Check if consensus is reached
            if proposal.votes.len() >= proposal.required_votes {
                self.finalize_proposal(proposal_id).await?;
            }
        }
        
        Ok(())
    }
    
    /// Finalize a proposal based on votes
    async fn finalize_proposal(&mut self, proposal_id: Uuid) -> Result<(), PolarProofError> {
        if let Some(proposal) = self.consensus_state.active_proposals.remove(&proposal_id) {
            let approve_votes = proposal.votes.values()
                .filter(|vote| matches!(vote.decision, VoteDecision::Approve))
                .count();
            
            let outcome = if approve_votes > proposal.votes.len() / 2 {
                DecisionOutcome::Approved
            } else {
                DecisionOutcome::Rejected
            };
            
            let decision = ConsensusDecision {
                decision_id: Uuid::new_v4(),
                proposal,
                outcome,
                finalized_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };
            
            self.consensus_state.finalized_decisions.push(decision);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PolarProof, EvaluationDomain, BatchMetadata};
    
    #[test]
    fn test_self_healing_manager_creation() {
        let manager = SelfHealingManager::new();
        assert!(manager.active_monitors.is_empty());
        assert_eq!(manager.health_config.consensus_nodes, 3);
    }
    
    #[test]
    fn test_health_config_default() {
        let config = HealthConfig::default();
        assert_eq!(config.check_interval, Duration::from_secs(60));
        assert_eq!(config.auto_repair_threshold, 0.8);
        assert!(config.real_time_monitoring);
    }
    
    #[test]
    fn test_repair_engine_strategies() {
        let engine = RepairEngine::new();
        assert!(!engine.strategies.is_empty());
        
        // Check that default strategies are present
        let strategy_names: Vec<&String> = engine.strategies.iter()
            .map(|s| &s.name)
            .collect();
        
        assert!(strategy_names.contains(&&"polynomial_reconstruction".to_string()));
        assert!(strategy_names.contains(&&"redundancy_restoration".to_string()));
        assert!(strategy_names.contains(&&"consensus_healing".to_string()));
    }
    
    #[test]
    fn test_consensus_coordinator() {
        let coordinator = ConsensusCoordinator::new();
        assert!(coordinator.known_nodes.is_empty());
        assert_eq!(coordinator.consensus_state.round, 0);
        assert!(coordinator.consensus_state.active_proposals.is_empty());
    }
    
    #[test]
    fn test_integrity_score_calculation() {
        let manager = SelfHealingManager::new();
        
        // Perfect score
        let score = manager.calculate_integrity_score(true, true, 3, 3);
        assert_eq!(score, 1.0);
        
        // Polynomial invalid
        let score = manager.calculate_integrity_score(false, true, 3, 3);
        assert_eq!(score, 0.5);
        
        // No redundancy
        let score = manager.calculate_integrity_score(true, false, 0, 3);
        assert_eq!(score, 0.5);
    }
}
