// Metanode Consensus Supercrate
// Consolidated consensus algorithms and block production for BPI Core

//! # Metanode Consensus
//! 
//! This supercrate consolidates all consensus-related functionality including:
//! - IBFT consensus algorithm
//! - BPI consensus mechanisms
//! - Block proposal and validation
//! - Leader selection using VRF
//! - Validator set management
//! - Slashing detection and enforcement
//! - Header processing pipeline

use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Error, Debug)]
pub enum ConsensusError {
    #[error("Invalid block proposal: {0}")]
    InvalidBlockProposal(String),
    #[error("Consensus timeout: {0}")]
    ConsensusTimeout(String),
    #[error("Invalid validator: {0}")]
    InvalidValidator(String),
    #[error("Slashing violation detected: {0}")]
    SlashingViolation(String),
    #[error("Leader selection failed: {0}")]
    LeaderSelectionFailed(String),
}

/// Consensus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    pub consensus_timeout: u64,
    pub block_time: u64,
    pub validator_set_size: usize,
    pub slashing_enabled: bool,
    pub min_validators: usize,
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        ConsensusConfig {
            consensus_timeout: 10000, // 10 seconds
            block_time: 2000,         // 2 seconds
            validator_set_size: 21,
            slashing_enabled: true,
            min_validators: 4,
        }
    }
}

/// Consensus state
#[derive(Debug, Clone)]
pub struct ConsensusState {
    pub current_round: u64,
    pub current_height: u64,
    pub validators: HashMap<String, ValidatorInfo>,
    pub leader: Option<String>,
    pub config: ConsensusConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorInfo {
    pub address: String,
    pub stake: u64,
    pub is_active: bool,
    pub reputation: f64,
}

/// Main consensus engine
pub struct ConsensusEngine {
    state: ConsensusState,
}

impl ConsensusEngine {
    pub fn new(config: ConsensusConfig) -> Self {
        ConsensusEngine {
            state: ConsensusState {
                current_round: 0,
                current_height: 0,
                validators: HashMap::new(),
                leader: None,
                config,
            },
        }
    }

    pub fn add_validator(&mut self, validator: ValidatorInfo) -> Result<(), ConsensusError> {
        if self.state.validators.len() >= self.state.config.validator_set_size {
            return Err(ConsensusError::InvalidValidator(
                "Validator set is full".to_string()
            ));
        }
        
        self.state.validators.insert(validator.address.clone(), validator);
        Ok(())
    }

    pub fn select_leader(&mut self) -> Result<String, ConsensusError> {
        if self.state.validators.is_empty() {
            return Err(ConsensusError::LeaderSelectionFailed(
                "No validators available".to_string()
            ));
        }

        // Simple leader selection - in practice would use VRF
        let leader = self.state.validators.keys().next().unwrap().clone();
        self.state.leader = Some(leader.clone());
        Ok(leader)
    }

    pub fn propose_block(&self, transactions: Vec<String>) -> Result<BlockProposal, ConsensusError> {
        let leader = self.state.leader.as_ref()
            .ok_or_else(|| ConsensusError::InvalidBlockProposal("No leader selected".to_string()))?;

        Ok(BlockProposal {
            height: self.state.current_height + 1,
            round: self.state.current_round,
            proposer: leader.clone(),
            transactions,
            timestamp: Utc::now(),
        })
    }

    pub fn validate_block(&self, proposal: &BlockProposal) -> Result<(), ConsensusError> {
        // Basic validation
        if proposal.height != self.state.current_height + 1 {
            return Err(ConsensusError::InvalidBlockProposal(
                format!("Invalid height: expected {}, got {}", 
                    self.state.current_height + 1, proposal.height)
            ));
        }

        if proposal.round != self.state.current_round {
            return Err(ConsensusError::InvalidBlockProposal(
                format!("Invalid round: expected {}, got {}", 
                    self.state.current_round, proposal.round)
            ));
        }

        Ok(())
    }

    pub fn advance_round(&mut self) {
        self.state.current_round += 1;
    }

    pub fn finalize_block(&mut self, proposal: BlockProposal) -> Result<(), ConsensusError> {
        self.validate_block(&proposal)?;
        self.state.current_height = proposal.height;
        // Keep the round number for tracking purposes instead of resetting to 0
        Ok(())
    }

    pub fn get_current_round(&self) -> u64 {
        self.state.current_round
    }

    pub fn get_validator_count(&self) -> usize {
        self.state.validators.len()
    }

    pub fn get_current_height(&self) -> u64 {
        self.state.current_height
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockProposal {
    pub height: u64,
    pub round: u64,
    pub proposer: String,
    pub transactions: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

// Re-export consolidated modules (these would be implemented from moved crates)
pub mod ibft {
    //! IBFT consensus implementation
    pub use super::*;
    
    pub fn start_consensus() -> Result<(), ConsensusError> {
        // IBFT consensus logic would be here
        Ok(())
    }
}

pub mod bpi_consensus {
    //! BPI-specific consensus mechanisms
    pub use super::*;
    
    pub fn initialize_bpi_consensus() -> Result<(), ConsensusError> {
        // BPI consensus logic would be here
        Ok(())
    }
}

pub mod leader_selection {
    //! VRF-based leader selection
    pub use super::*;
    
    pub fn select_leader_vrf(validators: &[String]) -> Result<String, ConsensusError> {
        // VRF leader selection would be here
        if validators.is_empty() {
            return Err(ConsensusError::LeaderSelectionFailed("No validators".to_string()));
        }
        Ok(validators[0].clone())
    }
}

pub mod validator_set {
    //! Validator set management
    pub use super::*;
    
    pub fn update_validator_set(validators: Vec<ValidatorInfo>) -> Result<(), ConsensusError> {
        // Validator set update logic would be here
        Ok(())
    }
}

pub mod slashing {
    //! Slashing detection and enforcement
    pub use super::*;
    
    pub fn detect_slashing_violation(validator: &str) -> Result<bool, ConsensusError> {
        // Slashing detection logic would be here
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consensus_engine_creation() {
        let config = ConsensusConfig::default();
        let engine = ConsensusEngine::new(config);
        assert_eq!(engine.state.current_height, 0);
        assert_eq!(engine.state.current_round, 0);
    }

    #[test]
    fn test_validator_management() {
        let mut engine = ConsensusEngine::new(ConsensusConfig::default());
        
        let validator = ValidatorInfo {
            address: "validator1".to_string(),
            stake: 1000,
            is_active: true,
            reputation: 1.0,
        };
        
        engine.add_validator(validator).unwrap();
        assert_eq!(engine.state.validators.len(), 1);
    }

    #[test]
    fn test_leader_selection() {
        let mut engine = ConsensusEngine::new(ConsensusConfig::default());
        
        let validator = ValidatorInfo {
            address: "validator1".to_string(),
            stake: 1000,
            is_active: true,
            reputation: 1.0,
        };
        
        engine.add_validator(validator).unwrap();
        let leader = engine.select_leader().unwrap();
        assert_eq!(leader, "validator1");
    }

    #[test]
    fn test_block_proposal() {
        let mut engine = ConsensusEngine::new(ConsensusConfig::default());
        
        let validator = ValidatorInfo {
            address: "validator1".to_string(),
            stake: 1000,
            is_active: true,
            reputation: 1.0,
        };
        
        engine.add_validator(validator).unwrap();
        engine.select_leader().unwrap();
        
        let transactions = vec!["tx1".to_string(), "tx2".to_string()];
        let proposal = engine.propose_block(transactions).unwrap();
        
        assert_eq!(proposal.height, 1);
        assert_eq!(proposal.round, 0);
        assert_eq!(proposal.proposer, "validator1");
    }
}
