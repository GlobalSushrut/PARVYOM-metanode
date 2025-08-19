//! IBFT consensus engine implementation

use super::*;

/// IBFT consensus engine
pub struct IbftConsensus {
    config: IbftConfig,
    node_id: Vec<u8>,
    bls_keypair: super::BlsPrivateKey,
    vrf_keypair: VrfPrivateKey,
    validators: Vec<ValidatorInfo>,
    current_round: ConsensusRound,
    state: ConsensusState,
    poh_chain: PohChain,
    
    // Message tracking
    prepare_votes: HashMap<[u8; 32], HashMap<Vec<u8>, Vec<u8>>>, // proposal_hash -> (sender -> signature)
    commit_votes: HashMap<[u8; 32], HashMap<Vec<u8>, Vec<u8>>>,
    
    // Communication channels
    message_tx: mpsc::UnboundedSender<IbftMessage>,
    message_rx: mpsc::UnboundedReceiver<IbftMessage>,
}

impl IbftConsensus {
    /// Create a new IBFT consensus engine
    pub fn new(
        config: IbftConfig,
        node_id: Vec<u8>,
        bls_keypair: super::BlsPrivateKey,
        vrf_keypair: VrfPrivateKey,
        validators: Vec<ValidatorInfo>,
    ) -> Self {
        let (message_tx, message_rx) = mpsc::unbounded_channel();
        
        // Initialize PoH chain for time ordering
        let poh_config = PohConfig {
            tick_duration_us: 1000, // 1ms ticks
            max_history_size: 10000,
            enable_vrf: true,
        };
        let mut poh_chain = PohChain::new_with_vrf(poh_config, vrf_keypair.clone());
        poh_chain.initialize().expect("PoH initialization should succeed");
        
        Self {
            config,
            node_id,
            bls_keypair,
            vrf_keypair,
            validators,
            current_round: ConsensusRound {
                height: 1,
                round: 0,
                leader: Vec::new(),
                timestamp: current_timestamp(),
            },
            state: ConsensusState::PrePrepare,
            poh_chain,
            prepare_votes: HashMap::new(),
            commit_votes: HashMap::new(),
            message_tx,
            message_rx,
        }
    }
    
    /// Select leader using VRF
    pub fn select_leader(&self) -> Result<Vec<u8>, IbftError> {
        // Use VRF to select leader based on round and height
        let round_data = format!("{}:{}", self.current_round.height, self.current_round.round);
        let (_, vrf_output) = self.vrf_keypair.prove(round_data.as_bytes());
        
        // Convert VRF output to validator index
        let total_stake: u64 = self.validators.iter().map(|v| v.stake).sum();
        let selection_value = vrf_output.to_uniform_u64(total_stake);
        
        let mut cumulative_stake = 0;
        for validator in &self.validators {
            cumulative_stake += validator.stake;
            if selection_value < cumulative_stake {
                return Ok(validator.node_id.clone());
            }
        }
        
        // Fallback to first validator
        Ok(self.validators[0].node_id.clone())
    }
    
    /// Propose a new block
    pub async fn propose_block(&mut self) -> Result<BlockProposal, IbftError> {
        // Get latest PoH proof
        let poh_proof = self.poh_chain.tick(Some(b"consensus_round".to_vec()))
            .map_err(|e| IbftError::PohError(format!("PoH tick failed: {}", e)))?;
        
        // Get previous block hash (simplified - would come from blockchain state)
        let previous_hash = self.poh_chain.latest_hash()
            .ok_or_else(|| IbftError::ConsensusError("No PoH history available".to_string()))?;
        
        // Create block proposal with empty transactions for now
        let transactions = Vec::new(); // In real implementation, would collect from mempool
        
        let proposal = BlockProposal::new(
            self.current_round.clone(),
            previous_hash,
            transactions,
            poh_proof,
            &self.bls_keypair,
        )?;
        
        Ok(proposal)
    }
    
    /// Verify PoH proof in proposal
    pub fn verify_poh_proof(&self, proposal: &BlockProposal) -> Result<(), IbftError> {
        // In a real implementation, this would verify the PoH proof chain
        // For now, we'll do a basic check that the proof exists
        if proposal.poh_proof == [0u8; 32] {
            return Err(IbftError::PohError("Invalid PoH proof".to_string()));
        }
        Ok(())
    }
    
    /// Sign a hash with BLS key
    pub fn sign_hash(&self, hash: &[u8; 32]) -> Result<Vec<u8>, IbftError> {
        let signature = self.bls_keypair.sign_hash(hash);
        Ok(signature.as_bytes().to_vec())
    }
    
    /// Calculate required votes (2f + 1 where f is max faulty nodes)
    pub fn required_votes(&self) -> usize {
        let n = self.validators.len();
        let f = (n - 1) / 3; // Max faulty nodes
        2 * f + 1
    }
    
    /// Get current consensus state
    pub fn get_state(&self) -> &ConsensusState {
        &self.state
    }
    
    /// Get current round info
    pub fn get_current_round(&self) -> &ConsensusRound {
        &self.current_round
    }
    
    /// Get validator count
    pub fn validator_count(&self) -> usize {
        self.validators.len()
    }
    
    /// Advance to next round
    pub fn advance_round(&mut self) {
        self.current_round.round += 1;
        self.current_round.timestamp = current_timestamp();
        self.state = ConsensusState::PrePrepare;
        
        // Clear vote tracking
        self.prepare_votes.clear();
        self.commit_votes.clear();
    }
    
    /// Finalize block and advance to next height
    pub async fn finalize_block(&mut self, proposal_hash: [u8; 32]) -> Result<(), IbftError> {
        println!("Block finalized! Height: {}, Hash: {:?}", 
                self.current_round.height, proposal_hash);
        
        // Move to next height
        self.current_round.height += 1;
        self.current_round.round = 0;
        self.current_round.timestamp = current_timestamp();
        self.state = ConsensusState::PrePrepare;
        
        // Clear vote tracking
        self.prepare_votes.clear();
        self.commit_votes.clear();
        
        // Advance PoH chain
        self.poh_chain.tick(Some(b"block_finalized".to_vec()))
            .map_err(|e| IbftError::PohError(format!("PoH advancement failed: {}", e)))?;
        
        Ok(())
    }
}
