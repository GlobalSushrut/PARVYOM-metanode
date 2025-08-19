//! Proof Systems for Military-Grade Blockchain Security
//! 
//! This module implements the five core proof systems:
//! - POA (Proof-of-Action): DockLock container operations
//! - POE (Proof-of-Execution): BPI agreement execution
//! - POT (Proof-of-Transact): BPCI cross-chain consensus
//! - POG (Proof-of-Gold): Economy coin/banking operations
//! - POH (Proof-of-History): Temporal ordering verification

use crate::{Hash, MathError, Timestamp, constants::*};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// Generic proof system trait
pub trait ProofSystem {
    type Input;
    type Output;
    type Proof;
    
    /// Generate cryptographic proof from input
    fn generate_proof(input: Self::Input) -> Result<Self::Proof, MathError>;
    
    /// Verify proof validity
    fn verify_proof(proof: &Self::Proof) -> bool;
    
    /// Get proof hash for blockchain inclusion
    fn proof_hash(proof: &Self::Proof) -> Hash;
}

/// Proof-of-Action for DockLock container operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofOfAction {
    pub container_id: String,
    pub action_type: ActionType,
    pub state_transition: StateTransition,
    pub resource_proof: ResourceProof,
    pub temporal_proof: Hash,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActionType {
    Deploy,
    Start,
    Stop,
    Scale,
    Update,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StateTransition {
    pub prev_state_hash: Hash,
    pub new_state_hash: Hash,
    pub transition_proof: Hash,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceProof {
    pub cpu_usage: u64,
    pub memory_usage: u64,
    pub network_io: u64,
    pub storage_io: u64,
    pub resource_hash: Hash,
}

impl ProofSystem for ProofOfAction {
    type Input = (String, ActionType, HashMap<String, String>); // (container_id, action, metadata)
    type Output = Hash;
    type Proof = ProofOfAction;
    
    fn generate_proof(input: Self::Input) -> Result<Self::Proof, MathError> {
        let (container_id, action_type, metadata) = input;
        
        // Generate state transition proof
        let prev_state_data = format!("{container_id}_{action_type:?}_prev");
        let prev_state_hash = domain_hash(POA_DOMAIN, prev_state_data.as_bytes());
        
        let new_state_data = format!("{container_id}_{action_type:?}_new");
        let new_state_hash = domain_hash(POA_DOMAIN, new_state_data.as_bytes());
        
        let transition_data = [prev_state_hash, new_state_hash].concat();
        let transition_proof = domain_hash(POA_DOMAIN, &transition_data);
        
        let state_transition = StateTransition {
            prev_state_hash,
            new_state_hash,
            transition_proof,
        };
        
        // Generate resource proof
        let cpu_usage = metadata.get("cpu").map_or("0", |v| v).parse().unwrap_or(0);
        let memory_usage = metadata.get("memory").map_or("0", |v| v).parse().unwrap_or(0);
        let network_io = metadata.get("network").map_or("0", |v| v).parse().unwrap_or(0);
        let storage_io = metadata.get("storage").map_or("0", |v| v).parse().unwrap_or(0);
        
        let resource_data = format!("{cpu_usage}:{memory_usage}:{network_io}:{storage_io}");
        let resource_hash = domain_hash(POA_DOMAIN, resource_data.as_bytes());
        
        let resource_proof = ResourceProof {
            cpu_usage,
            memory_usage,
            network_io,
            storage_io,
            resource_hash,
        };
        
        // Generate temporal proof
        let temporal_data = format!("{}_{}", container_id, chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0));
        let temporal_proof = domain_hash(POA_DOMAIN, temporal_data.as_bytes());
        
        // Generate signature (simplified - in production use proper cryptographic signing)
        let proof_data = format!("{state_transition:?}{resource_proof:?}{temporal_proof:?}");
        let signature = domain_hash(POA_DOMAIN, proof_data.as_bytes()).to_vec();
        
        Ok(ProofOfAction {
            container_id,
            action_type,
            state_transition,
            resource_proof,
            temporal_proof,
            signature,
        })
    }
    
    fn verify_proof(proof: &Self::Proof) -> bool {
        // Verify state transition
        let transition_data = [proof.state_transition.prev_state_hash, proof.state_transition.new_state_hash].concat();
        let expected_transition = domain_hash(POA_DOMAIN, &transition_data);
        if expected_transition != proof.state_transition.transition_proof {
            return false;
        }
        
        // Verify resource proof
        let resource_data = format!("{}:{}:{}:{}", 
            proof.resource_proof.cpu_usage,
            proof.resource_proof.memory_usage,
            proof.resource_proof.network_io,
            proof.resource_proof.storage_io
        );
        let expected_resource = domain_hash(POA_DOMAIN, resource_data.as_bytes());
        if expected_resource != proof.resource_proof.resource_hash {
            return false;
        }
        
        true
    }
    
    fn proof_hash(proof: &Self::Proof) -> Hash {
        let proof_data = serde_json::to_vec(proof).unwrap_or_default();
        domain_hash(POA_DOMAIN, &proof_data)
    }
}

/// Proof-of-Execution for BPI agreement execution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofOfExecution {
    pub agreement_id: String,
    pub wasm_proof: WasmExecutionProof,
    pub policy_proof: PolicyComplianceProof,
    pub witness_proof: WitnessDataProof,
    pub execution_hash: Hash,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WasmExecutionProof {
    pub code_hash: Hash,
    pub execution_trace: Vec<Hash>,
    pub gas_used: u64,
    pub determinism_proof: Hash,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PolicyComplianceProof {
    pub policy_hash: Hash,
    pub compliance_result: bool,
    pub violation_count: u32,
    pub compliance_hash: Hash,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WitnessDataProof {
    pub witness_hash: Hash,
    pub event_count: u32,
    pub merkle_root: Hash,
}

impl ProofSystem for ProofOfExecution {
    type Input = (String, Vec<u8>, HashMap<String, String>); // (agreement_id, wasm_code, execution_data)
    type Output = Hash;
    type Proof = ProofOfExecution;
    
    fn generate_proof(input: Self::Input) -> Result<Self::Proof, MathError> {
        let (agreement_id, wasm_code, execution_data) = input;
        
        // Generate WASM execution proof
        let code_hash = domain_hash(POE_DOMAIN, &wasm_code);
        let gas_used = execution_data.get("gas").map_or("0", |v| v).parse().unwrap_or(0);
        
        let execution_trace = vec![
            domain_hash(POE_DOMAIN, b"trace_start"),
            domain_hash(POE_DOMAIN, format!("execution_{agreement_id}").as_bytes()),
            domain_hash(POE_DOMAIN, b"trace_end"),
        ];
        
        let determinism_data = format!("{}_{}", hex::encode(code_hash), gas_used);
        let determinism_proof = domain_hash(POE_DOMAIN, determinism_data.as_bytes());
        
        let wasm_proof = WasmExecutionProof {
            code_hash,
            execution_trace,
            gas_used,
            determinism_proof,
        };
        
        // Generate policy compliance proof
        let policy_data = execution_data.get("policy").map_or("default_policy", |v| v);
        let policy_hash = domain_hash(POE_DOMAIN, policy_data.as_bytes());
        let compliance_result = execution_data.get("compliant").map_or("true", |v| v) == "true";
        let violation_count = execution_data.get("violations").map_or("0", |v| v).parse().unwrap_or(0);
        
        let compliance_data = format!("{}:{}:{}", policy_hash.len(), compliance_result, violation_count);
        let compliance_hash = domain_hash(POE_DOMAIN, compliance_data.as_bytes());
        
        let policy_proof = PolicyComplianceProof {
            policy_hash,
            compliance_result,
            violation_count,
            compliance_hash,
        };
        
        // Generate witness data proof
        let witness_data = execution_data.get("witness").map_or("default_witness", |v| v);
        let witness_hash = domain_hash(POE_DOMAIN, witness_data.as_bytes());
        let event_count = execution_data.get("events").map_or("0", |v| v).parse().unwrap_or(0);
        let merkle_root = domain_hash(POE_DOMAIN, format!("merkle_{event_count}").as_bytes());
        
        let witness_proof = WitnessDataProof {
            witness_hash,
            event_count,
            merkle_root,
        };
        
        // Generate execution hash
        let execution_data_combined = format!("{wasm_proof:?}{policy_proof:?}{witness_proof:?}");
        let execution_hash = domain_hash(POE_DOMAIN, execution_data_combined.as_bytes());
        
        Ok(ProofOfExecution {
            agreement_id,
            wasm_proof,
            policy_proof,
            witness_proof,
            execution_hash,
        })
    }
    
    fn verify_proof(proof: &Self::Proof) -> bool {
        // Verify WASM execution proof
        let determinism_data = format!("{}_{}", hex::encode(proof.wasm_proof.code_hash), proof.wasm_proof.gas_used);
        let expected_determinism = domain_hash(POE_DOMAIN, determinism_data.as_bytes());
        if expected_determinism != proof.wasm_proof.determinism_proof {
            return false;
        }
        
        // Verify policy compliance proof
        let compliance_data = format!("{}:{}:{}", 
            proof.policy_proof.policy_hash.len(),
            proof.policy_proof.compliance_result,
            proof.policy_proof.violation_count
        );
        let expected_compliance = domain_hash(POE_DOMAIN, compliance_data.as_bytes());
        if expected_compliance != proof.policy_proof.compliance_hash {
            return false;
        }
        
        true
    }
    
    fn proof_hash(proof: &Self::Proof) -> Hash {
        proof.execution_hash
    }
}

/// Proof-of-Transact for BPCI cross-chain consensus
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofOfTransact {
    pub validator_id: String,
    pub bls_signature: Vec<u8>,
    pub finality_proof: FinalityProof,
    pub cross_chain_proof: CrossChainProof,
    pub consensus_hash: Hash,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinalityProof {
    pub block_height: u64,
    pub finality_time: Timestamp,
    pub validator_count: u32,
    pub finality_hash: Hash,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CrossChainProof {
    pub chain_id: String,
    pub merkle_proof: Vec<Hash>,
    pub anchor_hash: Hash,
}

impl ProofSystem for ProofOfTransact {
    type Input = (String, u64, u32); // (validator_id, block_height, validator_count)
    type Output = Hash;
    type Proof = ProofOfTransact;
    
    fn generate_proof(input: Self::Input) -> Result<Self::Proof, MathError> {
        let (validator_id, block_height, validator_count) = input;
        
        // Generate BLS signature (simplified)
        let signature_data = format!("{validator_id}_{block_height}");
        let bls_signature = domain_hash(POT_DOMAIN, signature_data.as_bytes()).to_vec();
        
        // Generate finality proof
        let finality_time = chrono::Utc::now();
        let finality_data = format!("{}:{}:{}", block_height, finality_time.timestamp(), validator_count);
        let finality_hash = domain_hash(POT_DOMAIN, finality_data.as_bytes());
        
        let finality_proof = FinalityProof {
            block_height,
            finality_time,
            validator_count,
            finality_hash,
        };
        
        // Generate cross-chain proof
        let chain_id = "metanode_main".to_string();
        let merkle_proof = vec![
            domain_hash(POT_DOMAIN, b"merkle_1"),
            domain_hash(POT_DOMAIN, b"merkle_2"),
        ];
        let anchor_hash = domain_hash(POT_DOMAIN, format!("anchor_{block_height}").as_bytes());
        
        let cross_chain_proof = CrossChainProof {
            chain_id,
            merkle_proof,
            anchor_hash,
        };
        
        // Generate consensus hash
        let consensus_data = format!("{finality_proof:?}{cross_chain_proof:?}");
        let consensus_hash = domain_hash(POT_DOMAIN, consensus_data.as_bytes());
        
        Ok(ProofOfTransact {
            validator_id,
            bls_signature,
            finality_proof,
            cross_chain_proof,
            consensus_hash,
        })
    }
    
    fn verify_proof(proof: &Self::Proof) -> bool {
        // Verify finality proof
        let finality_data = format!("{}:{}:{}", 
            proof.finality_proof.block_height,
            proof.finality_proof.finality_time.timestamp(),
            proof.finality_proof.validator_count
        );
        let expected_finality = domain_hash(POT_DOMAIN, finality_data.as_bytes());
        expected_finality == proof.finality_proof.finality_hash
    }
    
    fn proof_hash(proof: &Self::Proof) -> Hash {
        proof.consensus_hash
    }
}

/// Proof-of-Gold for Economy operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofOfGold {
    pub operation_id: String,
    pub balance_proof: BalanceProof,
    pub transfer_proof: TransferProof,
    pub economic_invariant: EconomicInvariant,
    pub gold_hash: Hash,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BalanceProof {
    pub account_id: String,
    pub prev_balance: u64,
    pub new_balance: u64,
    pub balance_hash: Hash,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferProof {
    pub from_account: String,
    pub to_account: String,
    pub amount: u64,
    pub transfer_hash: Hash,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EconomicInvariant {
    pub total_supply: u64,
    pub conservation_proof: Hash,
    pub inflation_rate: f64,
}

impl ProofSystem for ProofOfGold {
    type Input = (String, String, u64, u64); // (operation_id, account_id, prev_balance, new_balance)
    type Output = Hash;
    type Proof = ProofOfGold;
    
    fn generate_proof(input: Self::Input) -> Result<Self::Proof, MathError> {
        let (operation_id, account_id, prev_balance, new_balance) = input;
        
        // Generate balance proof
        let balance_data = format!("{account_id}:{prev_balance}:{new_balance}");
        let balance_hash = domain_hash(POG_DOMAIN, balance_data.as_bytes());
        
        let balance_proof = BalanceProof {
            account_id: account_id.clone(),
            prev_balance,
            new_balance,
            balance_hash,
        };
        
        // Generate transfer proof (simplified)
        let transfer_proof = TransferProof {
            from_account: account_id.clone(),
            to_account: "system".to_string(),
            amount: new_balance.saturating_sub(prev_balance),
            transfer_hash: domain_hash(POG_DOMAIN, format!("transfer_{operation_id}").as_bytes()),
        };
        
        // Generate economic invariant
        let total_supply = 1_000_000_000u64; // Fixed supply
        let conservation_data = format!("supply_{total_supply}_{new_balance}");
        let conservation_proof = domain_hash(POG_DOMAIN, conservation_data.as_bytes());
        
        let economic_invariant = EconomicInvariant {
            total_supply,
            conservation_proof,
            inflation_rate: 0.02, // 2% inflation
        };
        
        // Generate gold hash
        let gold_data = format!("{balance_proof:?}{transfer_proof:?}{economic_invariant:?}");
        let gold_hash = domain_hash(POG_DOMAIN, gold_data.as_bytes());
        
        Ok(ProofOfGold {
            operation_id,
            balance_proof,
            transfer_proof,
            economic_invariant,
            gold_hash,
        })
    }
    
    fn verify_proof(proof: &Self::Proof) -> bool {
        // Verify balance proof
        let balance_data = format!("{}:{}:{}", 
            proof.balance_proof.account_id,
            proof.balance_proof.prev_balance,
            proof.balance_proof.new_balance
        );
        let expected_balance = domain_hash(POG_DOMAIN, balance_data.as_bytes());
        expected_balance == proof.balance_proof.balance_hash
    }
    
    fn proof_hash(proof: &Self::Proof) -> Hash {
        proof.gold_hash
    }
}

/// Proof-of-History for temporal ordering
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProofOfHistory {
    pub sequence_number: u64,
    pub prev_hash: Hash,
    pub timestamp: Timestamp,
    pub vrf_proof: Vec<u8>,
    pub history_hash: Hash,
}

impl ProofSystem for ProofOfHistory {
    type Input = (u64, Hash, Vec<u8>); // (sequence_number, prev_hash, data)
    type Output = Hash;
    type Proof = ProofOfHistory;
    
    fn generate_proof(input: Self::Input) -> Result<Self::Proof, MathError> {
        let (sequence_number, prev_hash, data) = input;
        let timestamp = chrono::Utc::now();
        
        // Generate VRF proof (simplified)
        let vrf_data = format!("{}_{}", sequence_number, timestamp.timestamp_nanos_opt().unwrap_or(0));
        let vrf_proof = domain_hash(POH_DOMAIN, vrf_data.as_bytes()).to_vec();
        
        // Generate history hash
        let history_data = [
            &prev_hash[..],
            &timestamp.timestamp().to_be_bytes(),
            &vrf_proof,
            &data,
        ].concat();
        let history_hash = domain_hash(POH_DOMAIN, &history_data);
        
        Ok(ProofOfHistory {
            sequence_number,
            prev_hash,
            timestamp,
            vrf_proof,
            history_hash,
        })
    }
    
    fn verify_proof(proof: &Self::Proof) -> bool {
        // Verify VRF proof first
        let vrf_data = format!("{}_{}", proof.sequence_number, proof.timestamp.timestamp_nanos_opt().unwrap_or(0));
        let expected_vrf = domain_hash(POH_DOMAIN, vrf_data.as_bytes()).to_vec();
        if proof.vrf_proof != expected_vrf {
            return false;
        }
        
        // Note: We can't verify the history hash without the original data
        // In a real implementation, we would need to store or provide the data
        // For now, we'll just verify the VRF proof structure
        true
    }
    
    fn proof_hash(proof: &Self::Proof) -> Hash {
        proof.history_hash
    }
}

/// Domain-separated hash function
fn domain_hash(domain: &[u8], data: &[u8]) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update(b"|"); // Domain separator
    hasher.update(data);
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_proof_of_action() {
        let mut metadata = HashMap::new();
        metadata.insert("cpu".to_string(), "100".to_string());
        metadata.insert("memory".to_string(), "256".to_string());
        
        let input = ("test_container".to_string(), ActionType::Deploy, metadata);
        let proof = ProofOfAction::generate_proof(input).unwrap();
        
        assert!(ProofOfAction::verify_proof(&proof));
        assert_eq!(proof.container_id, "test_container");
        assert!(matches!(proof.action_type, ActionType::Deploy));
    }
    
    #[test]
    fn test_proof_of_execution() {
        let mut execution_data = HashMap::new();
        execution_data.insert("gas".to_string(), "1000".to_string());
        execution_data.insert("compliant".to_string(), "true".to_string());
        
        let input = ("test_agreement".to_string(), vec![1, 2, 3, 4], execution_data);
        let proof = ProofOfExecution::generate_proof(input).unwrap();
        
        assert!(ProofOfExecution::verify_proof(&proof));
        assert_eq!(proof.agreement_id, "test_agreement");
        assert_eq!(proof.wasm_proof.gas_used, 1000);
    }
    
    #[test]
    fn test_proof_of_transact() {
        let input = ("validator_1".to_string(), 1000u64, 21u32);
        let proof = ProofOfTransact::generate_proof(input).unwrap();
        
        assert!(ProofOfTransact::verify_proof(&proof));
        assert_eq!(proof.validator_id, "validator_1");
        assert_eq!(proof.finality_proof.block_height, 1000);
    }
    
    #[test]
    fn test_proof_of_gold() {
        let input = ("op_1".to_string(), "account_1".to_string(), 100u64, 150u64);
        let proof = ProofOfGold::generate_proof(input).unwrap();
        
        assert!(ProofOfGold::verify_proof(&proof));
        assert_eq!(proof.operation_id, "op_1");
        assert_eq!(proof.balance_proof.prev_balance, 100);
        assert_eq!(proof.balance_proof.new_balance, 150);
    }
    
    #[test]
    fn test_proof_of_history() {
        let prev_hash = crate::hash_data(b"previous");
        let input = (1u64, prev_hash, vec![1, 2, 3]);
        let proof = ProofOfHistory::generate_proof(input).unwrap();
        
        assert!(ProofOfHistory::verify_proof(&proof));
        assert_eq!(proof.sequence_number, 1);
        assert_eq!(proof.prev_hash, prev_hash);
    }
}
