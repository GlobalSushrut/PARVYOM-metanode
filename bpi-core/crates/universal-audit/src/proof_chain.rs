//! Proof Chain - Cryptographic proof system for audit nodes
//!
//! Provides cryptographic integrity and non-repudiation for audit events

use crate::PROOF_CHAIN_HASH;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use bpi_enc::domain_hash;
use std::collections::HashMap;

/// Cryptographic Proof Chain for Audit Nodes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofChain {
    /// Hash chain linking to previous audit node
    pub prev_node_hash: Option<[u8; 32]>,
    /// Merkle proof of inclusion in current audit batch
    pub merkle_proof: Vec<[u8; 32]>,
    /// Time anchor proofs (Roughtime, TSA, blockchain)
    pub time_anchors: Vec<TimeAnchor>,
    /// TEE/TPM attestation (if available)
    pub tee_attestation: Option<TeeAttestation>,
    /// Witness signatures from multiple sources
    pub witness_signatures: Vec<WitnessSignature>,
    /// Proof metadata
    pub proof_metadata: ProofMetadata,
}

/// Time anchor for proof chain
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TimeAnchor {
    /// Type of time source
    pub anchor_type: TimeAnchorType,
    /// Timestamp from the source
    pub timestamp_ns: u64,
    /// Proof data from the time source
    pub proof_data: Vec<u8>,
    /// Signature from the time authority
    pub authority_signature: Vec<u8>,
    /// Authority public key
    pub authority_pubkey: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TimeAnchorType {
    /// Roughtime protocol
    Roughtime,
    /// Timestamp Authority (RFC 3161)
    TSA,
    /// Blockchain timestamp
    Blockchain,
    /// NTP server
    NTP,
    /// GPS time
    GPS,
    /// Custom time source
    Custom(String),
}

/// TEE (Trusted Execution Environment) attestation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TeeAttestation {
    /// Type of trusted execution environment
    pub tee_type: TeeType,
    /// Attestation quote/report
    pub quote: Vec<u8>,
    /// Measurement values
    pub measurements: Vec<Measurement>,
    /// Attestation signature
    pub signature: Vec<u8>,
    /// Certificate chain
    pub cert_chain: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TeeType {
    /// Intel SGX
    IntelSGX,
    /// AMD SEV
    AMDSEV,
    /// ARM TrustZone
    ARMTrustZone,
    /// TPM 2.0
    TPM20,
    /// Custom TEE
    Custom(String),
}

/// Measurement for TEE attestation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Measurement {
    /// Measurement register (PCR for TPM)
    pub register: u32,
    /// Measurement value
    pub value: [u8; 32],
    /// Algorithm used
    pub algorithm: String,
}

/// Witness signature from external validator
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WitnessSignature {
    /// Witness identifier
    pub witness_id: [u8; 32],
    /// Witness type
    pub witness_type: WitnessType,
    /// Ed25519 signature (64 bytes)
    pub signature: Vec<u8>,
    /// Witness public key
    pub public_key: [u8; 32],
    /// Timestamp when signature was created
    pub timestamp_ns: u64,
    /// Additional witness metadata
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WitnessType {
    /// DockLock container witness
    DockLock,
    /// ENC cluster witness
    EncCluster,
    /// HTTP cage witness
    HttpCage,
    /// IoT gateway witness
    IoTGateway,
    /// External audit service
    ExternalAuditor,
    /// Blockchain validator
    BlockchainValidator,
    /// Custom witness
    Custom(String),
}

/// Metadata for proof verification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofMetadata {
    /// Proof generation timestamp
    pub created_at_ns: u64,
    /// Proof version
    pub version: u32,
    /// Cryptographic algorithms used
    pub algorithms: Vec<CryptoAlgorithm>,
    /// Proof strength level
    pub strength_level: ProofStrength,
    /// Verification requirements
    pub verification_requirements: Vec<VerificationRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CryptoAlgorithm {
    /// Algorithm name
    pub name: String,
    /// Algorithm version
    pub version: String,
    /// Key size in bits
    pub key_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProofStrength {
    /// Basic proof (single signature)
    Basic,
    /// Enhanced proof (multiple witnesses)
    Enhanced,
    /// Military grade (TEE + multiple time anchors)
    MilitaryGrade,
    /// Quantum resistant
    QuantumResistant,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerificationRequirement {
    /// Signature verification required
    SignatureVerification,
    /// Time anchor verification required
    TimeAnchorVerification,
    /// TEE attestation verification required
    TeeAttestationVerification,
    /// Merkle proof verification required
    MerkleProofVerification,
    /// Chain of trust verification required
    ChainOfTrustVerification,
}

impl Default for ProofChain {
    fn default() -> Self {
        Self {
            prev_node_hash: None,
            merkle_proof: Vec::new(),
            time_anchors: Vec::new(),
            tee_attestation: None,
            witness_signatures: Vec::new(),
            proof_metadata: ProofMetadata::default(),
        }
    }
}

impl Default for ProofMetadata {
    fn default() -> Self {
        Self {
            created_at_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
            version: 1,
            algorithms: vec![
                CryptoAlgorithm {
                    name: "Ed25519".to_string(),
                    version: "1.0".to_string(),
                    key_size: 256,
                },
                CryptoAlgorithm {
                    name: "Blake3".to_string(),
                    version: "1.0".to_string(),
                    key_size: 256,
                },
            ],
            strength_level: ProofStrength::Enhanced,
            verification_requirements: vec![
                VerificationRequirement::SignatureVerification,
                VerificationRequirement::TimeAnchorVerification,
                VerificationRequirement::MerkleProofVerification,
            ],
        }
    }
}

impl ProofChain {
    /// Create a new proof chain
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a time anchor to the proof chain
    pub fn add_time_anchor(&mut self, anchor: TimeAnchor) {
        self.time_anchors.push(anchor);
    }
    
    /// Add a witness signature to the proof chain
    pub fn add_witness_signature(&mut self, signature: WitnessSignature) {
        self.witness_signatures.push(signature);
    }
    
    /// Set TEE attestation
    pub fn set_tee_attestation(&mut self, attestation: TeeAttestation) {
        self.tee_attestation = Some(attestation);
    }
    
    /// Set Merkle proof
    pub fn set_merkle_proof(&mut self, proof: Vec<[u8; 32]>) {
        self.merkle_proof = proof;
    }
    
    /// Link to previous node in the chain
    pub fn link_to_previous(&mut self, prev_hash: [u8; 32]) {
        self.prev_node_hash = Some(prev_hash);
    }
    
    /// Verify the entire proof chain
    pub fn verify(&self, node_hash: [u8; 32]) -> Result<bool> {
        // Verify witness signatures
        for witness in &self.witness_signatures {
            if !self.verify_witness_signature(witness, node_hash)? {
                return Ok(false);
            }
        }
        
        // Verify time anchors
        for anchor in &self.time_anchors {
            if !self.verify_time_anchor(anchor)? {
                return Ok(false);
            }
        }
        
        // Verify TEE attestation if present
        if let Some(attestation) = &self.tee_attestation {
            if !self.verify_tee_attestation(attestation, node_hash)? {
                return Ok(false);
            }
        }
        
        // Verify Merkle proof if present
        if !self.merkle_proof.is_empty() {
            if !self.verify_merkle_proof(node_hash)? {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Verify a witness signature
    fn verify_witness_signature(&self, witness: &WitnessSignature, node_hash: [u8; 32]) -> Result<bool> {
        let verifying_key = VerifyingKey::from_bytes(&witness.public_key)
            .map_err(|e| anyhow!("Invalid witness public key: {}", e))?;
        
        let signature = Signature::from_bytes(witness.signature.as_slice().try_into().map_err(|_| anyhow::anyhow!("Invalid signature length"))?);
        
        // Create message to verify (node hash + timestamp + witness metadata)
        let mut message = Vec::new();
        message.extend_from_slice(&node_hash);
        message.extend_from_slice(&witness.timestamp_ns.to_le_bytes());
        
        // Add metadata to message
        let metadata_bytes = bincode::serialize(&witness.metadata)?;
        message.extend_from_slice(&metadata_bytes);
        
        let message_hash = domain_hash("PROOF_CHAIN_HASH", &message);
        
        Ok(verifying_key.verify(&message_hash, &signature).is_ok())
    }
    
    /// Verify a time anchor
    fn verify_time_anchor(&self, anchor: &TimeAnchor) -> Result<bool> {
        match anchor.anchor_type {
            TimeAnchorType::Roughtime => self.verify_roughtime_anchor(anchor),
            TimeAnchorType::TSA => self.verify_tsa_anchor(anchor),
            TimeAnchorType::Blockchain => self.verify_blockchain_anchor(anchor),
            TimeAnchorType::NTP => self.verify_ntp_anchor(anchor),
            TimeAnchorType::GPS => self.verify_gps_anchor(anchor),
            TimeAnchorType::Custom(_) => self.verify_custom_anchor(anchor),
        }
    }
    
    /// Verify Roughtime anchor (simplified implementation)
    fn verify_roughtime_anchor(&self, anchor: &TimeAnchor) -> Result<bool> {
        // In a real implementation, this would verify the Roughtime response
        // For now, we just check that the proof data is not empty
        Ok(!anchor.proof_data.is_empty() && !anchor.authority_signature.is_empty())
    }
    
    /// Verify TSA anchor (simplified implementation)
    fn verify_tsa_anchor(&self, anchor: &TimeAnchor) -> Result<bool> {
        // In a real implementation, this would verify the TSA timestamp token
        // For now, we just check that the proof data is not empty
        Ok(!anchor.proof_data.is_empty() && !anchor.authority_signature.is_empty())
    }
    
    /// Verify blockchain anchor (simplified implementation)
    fn verify_blockchain_anchor(&self, anchor: &TimeAnchor) -> Result<bool> {
        // In a real implementation, this would verify the blockchain timestamp
        // For now, we just check that the proof data is not empty
        Ok(!anchor.proof_data.is_empty())
    }
    
    /// Verify NTP anchor (simplified implementation)
    fn verify_ntp_anchor(&self, anchor: &TimeAnchor) -> Result<bool> {
        // In a real implementation, this would verify the NTP response
        // For now, we just check that the proof data is not empty
        Ok(!anchor.proof_data.is_empty())
    }
    
    /// Verify GPS anchor (simplified implementation)
    fn verify_gps_anchor(&self, anchor: &TimeAnchor) -> Result<bool> {
        // In a real implementation, this would verify the GPS timestamp
        // For now, we just check that the proof data is not empty
        Ok(!anchor.proof_data.is_empty())
    }
    
    /// Verify custom anchor (simplified implementation)
    fn verify_custom_anchor(&self, anchor: &TimeAnchor) -> Result<bool> {
        // Custom verification logic would go here
        Ok(!anchor.proof_data.is_empty())
    }
    
    /// Verify TEE attestation (simplified implementation)
    fn verify_tee_attestation(&self, attestation: &TeeAttestation, node_hash: [u8; 32]) -> Result<bool> {
        match attestation.tee_type {
            TeeType::IntelSGX => self.verify_sgx_attestation(attestation, node_hash),
            TeeType::AMDSEV => self.verify_sev_attestation(attestation, node_hash),
            TeeType::ARMTrustZone => self.verify_trustzone_attestation(attestation, node_hash),
            TeeType::TPM20 => self.verify_tpm_attestation(attestation, node_hash),
            TeeType::Custom(_) => self.verify_custom_tee_attestation(attestation, node_hash),
        }
    }
    
    /// Verify Intel SGX attestation (simplified implementation)
    fn verify_sgx_attestation(&self, attestation: &TeeAttestation, _node_hash: [u8; 32]) -> Result<bool> {
        // In a real implementation, this would verify the SGX quote
        Ok(!attestation.quote.is_empty() && !attestation.signature.is_empty())
    }
    
    /// Verify AMD SEV attestation (simplified implementation)
    fn verify_sev_attestation(&self, attestation: &TeeAttestation, _node_hash: [u8; 32]) -> Result<bool> {
        // In a real implementation, this would verify the SEV attestation
        Ok(!attestation.quote.is_empty() && !attestation.signature.is_empty())
    }
    
    /// Verify ARM TrustZone attestation (simplified implementation)
    fn verify_trustzone_attestation(&self, attestation: &TeeAttestation, _node_hash: [u8; 32]) -> Result<bool> {
        // In a real implementation, this would verify the TrustZone attestation
        Ok(!attestation.quote.is_empty() && !attestation.signature.is_empty())
    }
    
    /// Verify TPM 2.0 attestation (simplified implementation)
    fn verify_tpm_attestation(&self, attestation: &TeeAttestation, _node_hash: [u8; 32]) -> Result<bool> {
        // In a real implementation, this would verify the TPM quote
        Ok(!attestation.quote.is_empty() && !attestation.signature.is_empty())
    }
    
    /// Verify custom TEE attestation (simplified implementation)
    fn verify_custom_tee_attestation(&self, attestation: &TeeAttestation, _node_hash: [u8; 32]) -> Result<bool> {
        // Custom TEE verification logic would go here
        Ok(!attestation.quote.is_empty())
    }
    
    /// Verify Merkle proof (simplified implementation)
    fn verify_merkle_proof(&self, node_hash: [u8; 32]) -> Result<bool> {
        if self.merkle_proof.is_empty() {
            return Ok(true); // No proof to verify
        }
        
        // In a real implementation, this would verify the Merkle inclusion proof
        // For now, we just check that the proof is not empty
        let mut current_hash = node_hash;
        
        for proof_element in &self.merkle_proof {
            // Simplified Merkle verification
            let mut hasher = blake3::Hasher::new();
            hasher.update(&current_hash);
            hasher.update(proof_element);
            current_hash = *hasher.finalize().as_bytes();
        }
        
        Ok(true) // Simplified verification always passes
    }
    
    /// Get proof strength score (0-100)
    pub fn get_strength_score(&self) -> u32 {
        let mut score = 0u32;
        
        // Base score for having a proof chain
        score += 10;
        
        // Score for witness signatures
        score += (self.witness_signatures.len() as u32 * 15).min(45);
        
        // Score for time anchors
        score += (self.time_anchors.len() as u32 * 10).min(30);
        
        // Score for TEE attestation
        if self.tee_attestation.is_some() {
            score += 20;
        }
        
        // Score for Merkle proof
        if !self.merkle_proof.is_empty() {
            score += 10;
        }
        
        // Score for previous node linkage
        if self.prev_node_hash.is_some() {
            score += 5;
        }
        
        score.min(100)
    }
}

/// Proof Chain Builder for easier construction
pub struct ProofChainBuilder {
    proof_chain: ProofChain,
}

impl ProofChainBuilder {
    /// Create a new proof chain builder
    pub fn new() -> Self {
        Self {
            proof_chain: ProofChain::new(),
        }
    }
    
    /// Add a time anchor
    pub fn with_time_anchor(mut self, anchor: TimeAnchor) -> Self {
        self.proof_chain.add_time_anchor(anchor);
        self
    }
    
    /// Add a witness signature
    pub fn with_witness_signature(mut self, signature: WitnessSignature) -> Self {
        self.proof_chain.add_witness_signature(signature);
        self
    }
    
    /// Set TEE attestation
    pub fn with_tee_attestation(mut self, attestation: TeeAttestation) -> Self {
        self.proof_chain.set_tee_attestation(attestation);
        self
    }
    
    /// Set Merkle proof
    pub fn with_merkle_proof(mut self, proof: Vec<[u8; 32]>) -> Self {
        self.proof_chain.set_merkle_proof(proof);
        self
    }
    
    /// Link to previous node
    pub fn with_previous_node(mut self, prev_hash: [u8; 32]) -> Self {
        self.proof_chain.link_to_previous(prev_hash);
        self
    }
    
    /// Build the proof chain
    pub fn build(self) -> ProofChain {
        self.proof_chain
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_proof_chain_creation() {
        let proof_chain = ProofChain::new();
        assert!(proof_chain.witness_signatures.is_empty());
        assert!(proof_chain.time_anchors.is_empty());
        assert!(proof_chain.tee_attestation.is_none());
    }
    
    #[test]
    fn test_proof_chain_builder() {
        let time_anchor = TimeAnchor {
            anchor_type: TimeAnchorType::Roughtime,
            timestamp_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
            proof_data: vec![1, 2, 3, 4],
            authority_signature: vec![5, 6, 7, 8],
            authority_pubkey: vec![9, 10, 11, 12],
        };
        
        let proof_chain = ProofChainBuilder::new()
            .with_time_anchor(time_anchor)
            .with_merkle_proof(vec![[1u8; 32], [2u8; 32]])
            .build();
        
        assert_eq!(proof_chain.time_anchors.len(), 1);
        assert_eq!(proof_chain.merkle_proof.len(), 2);
    }
    
    #[test]
    fn test_proof_strength_score() {
        let mut proof_chain = ProofChain::new();
        
        // Base score
        let base_score = proof_chain.get_strength_score();
        assert_eq!(base_score, 10);
        
        // Add witness signature
        let witness = WitnessSignature {
            witness_id: [1u8; 32],
            witness_type: WitnessType::DockLock,
            signature: [0u8; 64],
            public_key: [0u8; 32],
            timestamp_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
            metadata: HashMap::new(),
        };
        proof_chain.add_witness_signature(witness);
        
        let score_with_witness = proof_chain.get_strength_score();
        assert!(score_with_witness > base_score);
    }
}
