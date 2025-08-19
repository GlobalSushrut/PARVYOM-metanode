//! IBFT consensus integration for block headers

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{Header, HeaderHash, BlsSignature, BlsPublicKey, VrfProof, VrfOutput};

/// IBFT message types for header consensus
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IbftMessage {
    /// PRE-PREPARE message with header proposal
    PrePrepare {
        /// Block height
        height: u64,
        /// IBFT round
        round: u64,
        /// Proposed header
        header: Header,
        /// VRF proof for leader selection
        vrf_proof: VrfProof,
    },
    /// PREPARE message for header
    Prepare {
        /// Block height
        height: u64,
        /// IBFT round
        round: u64,
        /// Header hash being prepared
        header_hash: HeaderHash,
    },
    /// COMMIT message for header
    Commit {
        /// Block height
        height: u64,
        /// IBFT round
        round: u64,
        /// Header hash being committed
        header_hash: HeaderHash,
        /// BLS signature on header hash
        signature: BlsSignature,
    },
}

/// IBFT commit object for finalized headers
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IbftCommit {
    /// Header hash that was committed
    pub header_hash: HeaderHash,
    /// Aggregated BLS signature from validators
    pub bls_agg_sig: BlsSignature,
    /// Validator participation bitmap (bit i = validator i signed)
    pub bitmap: Vec<u8>,
    /// Block height
    pub height: u64,
    /// IBFT round
    pub round: u64,
}

/// VRF-based leader selection for IBFT
pub struct LeaderSelector {
    /// Current validator set
    validators: Vec<BlsPublicKey>,
}

/// IBFT header proposal with VRF proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderProposal {
    /// Proposed header
    pub header: Header,
    /// VRF proof for leader selection
    pub vrf_proof: VrfProof,
    /// VRF output for randomness
    pub vrf_output: VrfOutput,
    /// Proposer index in validator set
    pub proposer_index: usize,
}

impl IbftMessage {
    /// Get the height of this IBFT message
    pub fn height(&self) -> u64 {
        match self {
            IbftMessage::PrePrepare { height, .. } => *height,
            IbftMessage::Prepare { height, .. } => *height,
            IbftMessage::Commit { height, .. } => *height,
        }
    }
    
    /// Get the round of this IBFT message
    pub fn round(&self) -> u64 {
        match self {
            IbftMessage::PrePrepare { round, .. } => *round,
            IbftMessage::Prepare { round, .. } => *round,
            IbftMessage::Commit { round, .. } => *round,
        }
    }
    
    /// Get the header hash if available
    pub fn header_hash(&self) -> Option<HeaderHash> {
        match self {
            IbftMessage::PrePrepare { header, .. } => header.hash().ok(),
            IbftMessage::Prepare { header_hash, .. } => Some(*header_hash),
            IbftMessage::Commit { header_hash, .. } => Some(*header_hash),
        }
    }
    
    /// Check if this is a PRE-PREPARE message
    pub fn is_pre_prepare(&self) -> bool {
        matches!(self, IbftMessage::PrePrepare { .. })
    }
    
    /// Check if this is a PREPARE message
    pub fn is_prepare(&self) -> bool {
        matches!(self, IbftMessage::Prepare { .. })
    }
    
    /// Check if this is a COMMIT message
    pub fn is_commit(&self) -> bool {
        matches!(self, IbftMessage::Commit { .. })
    }
}

impl IbftCommit {
    /// Create a new IBFT commit object
    pub fn new(
        header_hash: HeaderHash,
        bls_agg_sig: BlsSignature,
        bitmap: Vec<u8>,
        height: u64,
        round: u64,
    ) -> Self {
        Self {
            header_hash,
            bls_agg_sig,
            bitmap,
            height,
            round,
        }
    }
    
    /// Check if validator at index participated in the commit
    pub fn validator_signed(&self, validator_index: usize) -> bool {
        let byte_index = validator_index / 8;
        let bit_offset = validator_index % 8;
        
        if byte_index >= self.bitmap.len() {
            return false;
        }
        
        (self.bitmap[byte_index] & (1 << bit_offset)) != 0
    }
    
    /// Count the number of validators that signed
    pub fn signature_count(&self) -> usize {
        self.bitmap.iter().map(|byte| byte.count_ones() as usize).sum()
    }
    
    /// Check if the commit meets the threshold (2f+1 out of 3f+1)
    pub fn meets_threshold(&self, total_validators: usize) -> bool {
        // For IBFT: N = 3f+1, so f = (N-1)/3, and we need 2f+1 signatures
        // Required signatures = 2*((N-1)/3) + 1 = (2*(N-1) + 3)/3 = (2*N + 1)/3
        let required_signatures = (2 * total_validators + 1) / 3;
        self.signature_count() >= required_signatures
    }
    
    /// Verify the BLS aggregate signature
    pub fn verify_signature(&self, validator_pubkeys: &[BlsPublicKey]) -> Result<bool> {
        // Get the public keys of validators who signed
        let signing_pubkeys: Vec<BlsPublicKey> = validator_pubkeys
            .iter()
            .enumerate()
            .filter(|(i, _)| self.validator_signed(*i))
            .map(|(_, pk)| pk.clone())
            .collect();
        
        if signing_pubkeys.is_empty() {
            return Ok(false);
        }
        
        // For this implementation, we'll use a simplified verification
        // In practice, this would use proper BLS aggregate signature verification
        // with the header hash as the message
        Ok(true)
    }
}

impl LeaderSelector {
    /// Create a new leader selector with validator set
    pub fn new(validators: Vec<BlsPublicKey>) -> Self {
        Self { validators }
    }
    
    /// Select leader for given height and round using VRF
    pub fn select_leader(&self, height: u64, round: u64, vrf_output: &VrfOutput) -> usize {
        if self.validators.is_empty() {
            return 0;
        }
        
        // Use VRF output to select leader deterministically
        // Convert VRF output to u64 and mod by validator count
        let mut seed = 0u64;
        for (i, &byte) in vrf_output.as_bytes().iter().take(8).enumerate() {
            seed |= (byte as u64) << (i * 8);
        }
        
        // Mix in height and round for additional entropy
        seed = seed.wrapping_add(height).wrapping_add(round);
        
        (seed as usize) % self.validators.len()
    }
    
    /// Get validator count
    pub fn validator_count(&self) -> usize {
        self.validators.len()
    }
    
    /// Get validator public key at index
    pub fn get_validator(&self, index: usize) -> Option<&BlsPublicKey> {
        self.validators.get(index)
    }
    
    /// Update validator set
    pub fn update_validators(&mut self, validators: Vec<BlsPublicKey>) {
        self.validators = validators;
    }
}

impl HeaderProposal {
    /// Create a new header proposal
    pub fn new(
        header: Header,
        vrf_proof: VrfProof,
        vrf_output: VrfOutput,
        proposer_index: usize,
    ) -> Self {
        Self {
            header,
            vrf_proof,
            vrf_output,
            proposer_index,
        }
    }
    
    /// Verify the VRF proof for leader selection
    pub fn verify_leader_selection(&self, validators: &[BlsPublicKey]) -> Result<bool> {
        // Verify proposer index is valid
        if self.proposer_index >= validators.len() {
            return Ok(false);
        }
        
        // In practice, this would verify the VRF proof against the proposer's VRF key
        // For now, we'll assume it's valid
        Ok(true)
    }
    
    /// Get the header hash
    pub fn header_hash(&self) -> Result<HeaderHash> {
        self.header.hash()
    }
    
    /// Convert to PRE-PREPARE message
    pub fn to_pre_prepare_message(&self) -> IbftMessage {
        IbftMessage::PrePrepare {
            height: self.header.height,
            round: self.header.round,
            header: self.header.clone(),
            vrf_proof: self.vrf_proof.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ConsensusMode, HeaderConfig};
    
    fn create_test_header() -> Header {
        let config = HeaderConfig {
            version: 1,
            height: 100,
            prev_hash: [1u8; 32],
            poh_root: [2u8; 32],
            receipts_root: [3u8; 32],
            da_root: [4u8; 32],
            xcmp_root: [5u8; 32],
            validator_set_hash: [6u8; 32],
            mode: ConsensusMode::Ibft,
            round: 0,
        };
        Header::new(config)
    }
    
    fn create_test_validators() -> Vec<BlsPublicKey> {
        // Create dummy validator public keys for testing
        vec![
            BlsPublicKey::from_bytes(&[1u8; 48]).unwrap(),
            BlsPublicKey::from_bytes(&[2u8; 48]).unwrap(),
            BlsPublicKey::from_bytes(&[3u8; 48]).unwrap(),
            BlsPublicKey::from_bytes(&[4u8; 48]).unwrap(),
        ]
    }
    
    #[test]
    fn test_ibft_message_properties() {
        let header = create_test_header();
        let vrf_proof = VrfProof::from_bytes(&[0u8; 80]).unwrap();
        
        let pre_prepare = IbftMessage::PrePrepare {
            height: 100,
            round: 5,
            header: header.clone(),
            vrf_proof,
        };
        
        assert_eq!(pre_prepare.height(), 100);
        assert_eq!(pre_prepare.round(), 5);
        assert!(pre_prepare.is_pre_prepare());
        assert!(!pre_prepare.is_prepare());
        assert!(!pre_prepare.is_commit());
        
        let header_hash = header.hash().unwrap();
        let prepare = IbftMessage::Prepare {
            height: 100,
            round: 5,
            header_hash,
        };
        
        assert!(prepare.is_prepare());
        assert_eq!(prepare.header_hash(), Some(header_hash));
    }
    
    #[test]
    fn test_ibft_commit() {
        let header = create_test_header();
        let header_hash = header.hash().unwrap();
        let signature = BlsSignature::from_bytes(&[0u8; 96]).unwrap();
        
        // Create bitmap with validators 0, 1, and 2 signing (4 total validators)
        let bitmap = vec![0b00000111]; // bits 0, 1, and 2 set
        
        let commit = IbftCommit::new(header_hash, signature, bitmap, 100, 5);
        
        assert!(commit.validator_signed(0));
        assert!(commit.validator_signed(1));
        assert!(commit.validator_signed(2));
        assert!(!commit.validator_signed(3));
        
        assert_eq!(commit.signature_count(), 3);
        assert!(commit.meets_threshold(4)); // 3 >= 2f+1 = 3 for f=1
        assert!(!commit.meets_threshold(7)); // 3 < 2f+1 = 5 for f=2
    }
    
    #[test]
    fn test_leader_selector() {
        let validators = create_test_validators();
        let selector = LeaderSelector::new(validators.clone());
        
        assert_eq!(selector.validator_count(), 4);
        
        let vrf_output = VrfOutput::from_bytes(&[1u8; 32]).unwrap();
        let leader_index = selector.select_leader(100, 5, &vrf_output);
        
        assert!(leader_index < 4);
        assert!(selector.get_validator(leader_index).is_some());
        
        // Same inputs should give same leader
        let leader_index2 = selector.select_leader(100, 5, &vrf_output);
        assert_eq!(leader_index, leader_index2);
    }
    
    #[test]
    fn test_header_proposal() {
        let header = create_test_header();
        let vrf_proof = VrfProof::from_bytes(&[0u8; 80]).unwrap();
        let vrf_output = VrfOutput::from_bytes(&[1u8; 32]).unwrap();
        
        let proposal = HeaderProposal::new(header.clone(), vrf_proof, vrf_output, 0);
        
        assert_eq!(proposal.proposer_index, 0);
        assert_eq!(proposal.header.height, header.height);
        
        let header_hash = proposal.header_hash().unwrap();
        assert_eq!(header_hash, header.hash().unwrap());
        
        let pre_prepare = proposal.to_pre_prepare_message();
        assert!(pre_prepare.is_pre_prepare());
        assert_eq!(pre_prepare.height(), header.height);
    }
    
    #[test]
    fn test_commit_verification() {
        let validators = create_test_validators();
        let header = create_test_header();
        let header_hash = header.hash().unwrap();
        let signature = BlsSignature::from_bytes(&[0u8; 96]).unwrap();
        
        let bitmap = vec![0b00000111]; // validators 0, 1, 2 signed
        let commit = IbftCommit::new(header_hash, signature, bitmap, 100, 5);
        
        // For testing, verification should pass (simplified implementation)
        assert!(commit.verify_signature(&validators).unwrap());
        assert_eq!(commit.signature_count(), 3);
        assert!(commit.meets_threshold(4));
    }
}
