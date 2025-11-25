// Production Groth16 zk-SNARK Implementation
// Real cryptographic implementation using Arkworks

use ark_std::rand::RngCore;
use ark_groth16::{Groth16, ProvingKey, VerifyingKey, Proof, prepare_verifying_key};
use ark_bn254::{Bn254, Fr as BnFr};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use ark_snark::{CircuitSpecificSetupSNARK, SNARK};
use anyhow::{Result, anyhow};
use log::{info, debug};
use rand::rngs::OsRng;

/// Simple square root circuit: proves knowledge of x such that x^2 = public_input
/// This is a basic example - production circuits would be more complex
#[derive(Clone)]
pub struct SquareRootCircuit {
    pub witness: Option<BnFr>,
    pub public_input: BnFr,
}

impl ConstraintSynthesizer<BnFr> for SquareRootCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<BnFr>) -> Result<(), SynthesisError> {
        // Allocate witness variable (private)
        let witness_var = cs.new_witness_variable(|| {
            self.witness.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // Allocate public input variable
        let public_var = cs.new_input_variable(|| Ok(self.public_input))?;
        
        // Constraint: witness * witness = public_input
        // This proves knowledge of square root without revealing it
        cs.enforce_constraint(
            ark_relations::lc!() + witness_var,
            ark_relations::lc!() + witness_var,
            ark_relations::lc!() + public_var,
        )?;
        
        Ok(())
    }
}

/// Production Groth16 Prover
pub struct Groth16Prover {
    proving_key: Option<ProvingKey<Bn254>>,
    verifying_key: Option<VerifyingKey<Bn254>>,
}

impl Groth16Prover {
    pub fn new() -> Self {
        Self {
            proving_key: None,
            verifying_key: None,
        }
    }
    
    /// Perform trusted setup (circuit-specific)
    /// In production, this would be done once and keys would be cached
    pub fn setup(&mut self) -> Result<()> {
        info!("🔧 Performing Groth16 trusted setup...");
        
        let mut rng = OsRng;
        
        // Create dummy circuit for setup
        let setup_circuit = SquareRootCircuit {
            witness: None,
            public_input: BnFr::from(1u64),
        };
        
        // Generate proving and verifying keys using CircuitSpecificSetupSNARK trait
        let (pk, vk) = Groth16::<Bn254>::setup(setup_circuit, &mut rng)
            .map_err(|e| anyhow!("Groth16 setup failed: {:?}", e))?;
        
        self.proving_key = Some(pk);
        self.verifying_key = Some(vk);
        
        info!("✅ Groth16 trusted setup complete");
        Ok(())
    }
    
    /// Generate Groth16 proof
    pub fn prove(&self, witness: u64, public_input: u64) -> Result<(Vec<u8>, Vec<u8>)> {
        let pk = self.proving_key.as_ref()
            .ok_or_else(|| anyhow!("Proving key not initialized. Call setup() first."))?;
        
        let vk = self.verifying_key.as_ref()
            .ok_or_else(|| anyhow!("Verifying key not initialized. Call setup() first."))?;
        
        debug!("Generating Groth16 proof for witness={}, public_input={}", witness, public_input);
        
        let mut rng = OsRng;
        
        // Create circuit with actual witness
        let circuit = SquareRootCircuit {
            witness: Some(BnFr::from(witness)),
            public_input: BnFr::from(public_input),
        };
        
        // Generate proof using SNARK trait
        let proof = Groth16::<Bn254>::prove(pk, circuit, &mut rng)
            .map_err(|e| anyhow!("Groth16 proof generation failed: {:?}", e))?;
        
        // Serialize proof
        let mut proof_bytes = Vec::new();
        proof.serialize_compressed(&mut proof_bytes)
            .map_err(|e| anyhow!("Proof serialization failed: {:?}", e))?;
        
        // Serialize verification key
        let mut vk_bytes = Vec::new();
        vk.serialize_compressed(&mut vk_bytes)
            .map_err(|e| anyhow!("VK serialization failed: {:?}", e))?;
        
        debug!("✅ Groth16 proof generated: {} bytes", proof_bytes.len());
        Ok((proof_bytes, vk_bytes))
    }
    
    /// Verify Groth16 proof
    pub fn verify(&self, proof_bytes: &[u8], vk_bytes: &[u8], public_input: u64) -> Result<bool> {
        debug!("Verifying Groth16 proof for public_input={}", public_input);
        
        // Deserialize proof
        let proof = Proof::<Bn254>::deserialize_compressed(proof_bytes)
            .map_err(|e| anyhow!("Proof deserialization failed: {:?}", e))?;
        
        // Deserialize verification key
        let vk = VerifyingKey::<Bn254>::deserialize_compressed(vk_bytes)
            .map_err(|e| anyhow!("VK deserialization failed: {:?}", e))?;
        
        // Prepare public inputs
        let public_inputs = vec![BnFr::from(public_input)];
        
        // Prepare verification key
        let pvk = prepare_verifying_key(&vk);
        
        // Verify proof using SNARK trait
        let verified = Groth16::<Bn254>::verify_proof(&pvk, &proof, &public_inputs)
            .map_err(|e| anyhow!("Groth16 verification failed: {:?}", e))?;
        
        debug!("✅ Groth16 verification result: {}", verified);
        Ok(verified)
    }
    
    /// Get proving key bytes (for caching)
    pub fn get_proving_key_bytes(&self) -> Result<Vec<u8>> {
        let pk = self.proving_key.as_ref()
            .ok_or_else(|| anyhow!("Proving key not initialized"))?;
        
        let mut bytes = Vec::new();
        pk.serialize_compressed(&mut bytes)
            .map_err(|e| anyhow!("PK serialization failed: {:?}", e))?;
        
        Ok(bytes)
    }
    
    /// Get verifying key bytes (for caching)
    pub fn get_verifying_key_bytes(&self) -> Result<Vec<u8>> {
        let vk = self.verifying_key.as_ref()
            .ok_or_else(|| anyhow!("Verifying key not initialized"))?;
        
        let mut bytes = Vec::new();
        vk.serialize_compressed(&mut bytes)
            .map_err(|e| anyhow!("VK serialization failed: {:?}", e))?;
        
        Ok(bytes)
    }
    
    /// Load proving key from bytes
    pub fn load_proving_key(&mut self, bytes: &[u8]) -> Result<()> {
        let pk = ProvingKey::<Bn254>::deserialize_compressed(bytes)
            .map_err(|e| anyhow!("PK deserialization failed: {:?}", e))?;
        
        self.proving_key = Some(pk);
        Ok(())
    }
    
    /// Load verifying key from bytes
    pub fn load_verifying_key(&mut self, bytes: &[u8]) -> Result<()> {
        let vk = VerifyingKey::<Bn254>::deserialize_compressed(bytes)
            .map_err(|e| anyhow!("VK deserialization failed: {:?}", e))?;
        
        self.verifying_key = Some(vk);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_groth16_setup() {
        let mut prover = Groth16Prover::new();
        let result = prover.setup();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_groth16_prove_and_verify() {
        let mut prover = Groth16Prover::new();
        prover.setup().unwrap();
        
        // Prove knowledge of square root: 42^2 = 1764
        let witness = 42u64;
        let public_input = 1764u64;
        
        let (proof_bytes, vk_bytes) = prover.prove(witness, public_input).unwrap();
        
        // Verify proof
        let verified = prover.verify(&proof_bytes, &vk_bytes, public_input).unwrap();
        assert!(verified, "Proof should verify successfully");
    }
    
    #[test]
    fn test_groth16_invalid_proof() {
        let mut prover = Groth16Prover::new();
        prover.setup().unwrap();
        
        // Prove knowledge of square root: 42^2 = 1764
        let witness = 42u64;
        let public_input = 1764u64;
        
        let (proof_bytes, vk_bytes) = prover.prove(witness, public_input).unwrap();
        
        // Try to verify with wrong public input
        let wrong_public_input = 1000u64;
        let verified = prover.verify(&proof_bytes, &vk_bytes, wrong_public_input).unwrap();
        assert!(!verified, "Proof should fail verification with wrong public input");
    }
    
    #[test]
    fn test_groth16_key_serialization() {
        let mut prover = Groth16Prover::new();
        prover.setup().unwrap();
        
        // Serialize keys
        let pk_bytes = prover.get_proving_key_bytes().unwrap();
        let vk_bytes = prover.get_verifying_key_bytes().unwrap();
        
        // Create new prover and load keys
        let mut prover2 = Groth16Prover::new();
        prover2.load_proving_key(&pk_bytes).unwrap();
        prover2.load_verifying_key(&vk_bytes).unwrap();
        
        // Test proof generation with loaded keys
        let witness = 42u64;
        let public_input = 1764u64;
        
        let (proof_bytes, vk_bytes) = prover2.prove(witness, public_input).unwrap();
        let verified = prover2.verify(&proof_bytes, &vk_bytes, public_input).unwrap();
        assert!(verified, "Proof with loaded keys should verify");
    }
}
