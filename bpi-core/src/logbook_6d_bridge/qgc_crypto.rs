// QGC Crypto - Cryptographic operations for QGC-C² consensus
// BLS aggregation, VRF, Ed25519, and PQC-ready (Dilithium) support

use crate::logbook_6d_bridge::qgc_core::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use blake3;
use rand::{Rng, thread_rng};

/// Cryptographic configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoConfig {
    pub enable_bls: bool,                // Enable BLS aggregation
    pub enable_vrf: bool,                // Enable VRF for committee selection
    pub enable_pqc: bool,                // Enable post-quantum crypto
    pub vrf_threshold: u64,              // VRF selection threshold
    pub bls_threshold: u8,               // BLS aggregation threshold
}

impl Default for CryptoConfig {
    fn default() -> Self {
        Self {
            enable_bls: true,
            enable_vrf: true,
            enable_pqc: false,           // Off by default for now
            vrf_threshold: u64::MAX / 2, // 50% selection probability
            bls_threshold: 16,           // 2/3 of 24 committee
        }
    }
}

/// Validator identity structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorIdentity {
    pub validator_id: [u8; 32],         // Unique validator identifier
    pub bls_public_key: Vec<u8>,        // BLS public key (96 bytes)
    pub pqc_public_key: [u8; 32],       // Post-quantum public key
    pub vrf_public_key: Vec<u8>,        // VRF public key
    pub ed25519_public_key: Vec<u8>,    // Ed25519 public key
    pub stake: u64,                     // Validator stake
    pub reputation: u32,                // Reputation score
    pub is_active: bool,                 // Active status
}

impl Default for ValidatorIdentity {
    fn default() -> Self {
        Self {
            validator_id: [0; 32],
            bls_public_key: vec![0; 96],
            pqc_public_key: [0; 32],
            vrf_public_key: vec![0; 32],
            ed25519_public_key: vec![0; 32],
            stake: 0,
            reputation: 100,
            is_active: true,
        }
    }
}

impl ValidatorIdentity {
    pub fn new(validator_id: [u8; 32], stake: u64) -> Self {
        // Generate deterministic keys from validator_id (simplified)
        let mut hasher = blake3::Hasher::new();
        hasher.update(&validator_id);
        hasher.update(b"ed25519");
        let ed25519_seed = hasher.finalize();
        let ed25519_public_key = ed25519_seed.as_bytes()[..32].try_into().unwrap();
        
        hasher = blake3::Hasher::new();
        hasher.update(&validator_id);
        hasher.update(b"bls");
        let bls_seed = hasher.finalize();
        // BLS keys need 48 bytes, but Blake3 only gives 32, so we extend it
        let mut bls_key_bytes = [0u8; 48];
        bls_key_bytes[..32].copy_from_slice(bls_seed.as_bytes());
        // Fill remaining 16 bytes with a deterministic pattern
        for i in 32..48 {
            bls_key_bytes[i] = (i as u8).wrapping_mul(validator_id[0]);
        }
        let bls_public_key = bls_key_bytes;
        
        hasher = blake3::Hasher::new();
        hasher.update(&validator_id);
        hasher.update(b"vrf");
        let vrf_seed = hasher.finalize();
        let vrf_public_key = vrf_seed.as_bytes()[..32].try_into().unwrap();
        
        Self {
            validator_id,
            ed25519_public_key,
            bls_public_key: bls_public_key.to_vec(), // Convert array to Vec
            pqc_public_key: [0; 32], // Default PQC key
            vrf_public_key,
            stake,
            reputation: 100, // Default reputation
            is_active: true,
        }
    }
}

/// VRF (Verifiable Random Function) implementation
#[derive(Debug, Clone)]
pub struct VrfEngine {
    config: CryptoConfig,
}

impl VrfEngine {
    pub fn new(config: CryptoConfig) -> Self {
        Self { config }
    }
    
    /// Generate VRF proof for committee selection
    pub fn generate_vrf_proof(&self, validator: &ValidatorIdentity, round: u64, input: &[u8]) -> VrfProof {
        // Simplified VRF implementation using Blake3
        let mut hasher = blake3::Hasher::new();
        hasher.update(&validator.vrf_public_key);
        hasher.update(&round.to_le_bytes());
        hasher.update(input);
        
        let hash = hasher.finalize();
        let value = u64::from_le_bytes(hash.as_bytes()[..8].try_into().unwrap());
        
        // Generate proof (simplified - would use actual VRF in production)
        let mut proof = [0u8; 80];
        proof[..32].copy_from_slice(&hash.as_bytes()[..32]);
        proof[32..64].copy_from_slice(&validator.vrf_public_key);
        proof[64..72].copy_from_slice(&round.to_le_bytes());
        proof[72..80].copy_from_slice(&value.to_le_bytes());
        
        VrfProof {
            proof: proof.to_vec(),
            value,
            is_selected: value < self.config.vrf_threshold,
        }
    }
    
    /// Verify VRF proof
    pub fn verify_vrf_proof(&self, proof: &VrfProof, validator: &ValidatorIdentity, round: u64, input: &[u8]) -> bool {
        // Simplified verification
        let mut hasher = blake3::Hasher::new();
        hasher.update(&validator.vrf_public_key);
        hasher.update(&round.to_le_bytes());
        hasher.update(input);
        
        let expected_hash = hasher.finalize();
        let expected_value = u64::from_le_bytes(expected_hash.as_bytes()[..8].try_into().unwrap());
        
        proof.value == expected_value && proof.proof[..32] == expected_hash.as_bytes()[..32]
    }
}

/// VRF proof structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VrfProof {
    #[serde(with = "serde_bytes")]
    pub proof: Vec<u8>,                  // VRF proof bytes (80 bytes)
    pub value: u64,                      // VRF output value
    pub is_selected: bool,               // Whether validator is selected
}

impl VrfProof {
    pub fn new(proof_bytes: [u8; 80], value: u64, is_selected: bool) -> Self {
        Self {
            proof: proof_bytes.to_vec(),
            value,
            is_selected,
        }
    }
}

/// BLS public key structure
#[derive(Debug, Clone)]
pub struct BlsPublicKey {
    pub key: Vec<u8>,  // BLS12-381 public key (96 bytes)
    pub key_id: Vec<u8>,  // Key identifier (48 bytes)
}

impl serde::Serialize for BlsPublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("BlsPublicKey", 2)?;
        state.serialize_field("key", &self.key)?;
        state.serialize_field("key_id", &self.key_id)?;
        state.end()
    }
}

impl<'de> serde::Deserialize<'de> for BlsPublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
        use std::fmt;

        struct BlsPublicKeyVisitor;

        impl<'de> Visitor<'de> for BlsPublicKeyVisitor {
            type Value = BlsPublicKey;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct BlsPublicKey")
            }

            fn visit_map<V>(self, mut map: V) -> Result<BlsPublicKey, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut key = None;
                let mut key_id = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        "key" => {
                            if key.is_some() {
                                return Err(de::Error::duplicate_field("key"));
                            }
                            key = Some(map.next_value()?);
                        }
                        "key_id" => {
                            if key_id.is_some() {
                                return Err(de::Error::duplicate_field("key_id"));
                            }
                            key_id = Some(map.next_value()?);
                        }
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }
                let key = key.ok_or_else(|| de::Error::missing_field("key"))?;
                let key_id = key_id.ok_or_else(|| de::Error::missing_field("key_id"))?;
                Ok(BlsPublicKey { key, key_id })
            }
        }

        deserializer.deserialize_struct("BlsPublicKey", &["key", "key_id"], BlsPublicKeyVisitor)
    }
}

/// BLS signature aggregation engine
#[derive(Debug, Clone)]
pub struct BlsEngine {
    config: CryptoConfig,
}

impl BlsEngine {
    pub fn new(config: CryptoConfig) -> Self {
        Self { config }
    }
    
    /// Generate BLS partial signature
    pub fn sign_partial(&self, validator: &ValidatorIdentity, message: &[u8]) -> BlsPartialSignature {
        // Simplified BLS signature using Blake3 (would use actual BLS in production)
        let mut hasher = blake3::Hasher::new();
        hasher.update(&validator.bls_public_key);
        hasher.update(message);
        
        let signature_hash = hasher.finalize();
        let mut signature = [0u8; 96];
        signature[..32].copy_from_slice(signature_hash.as_bytes());
        signature[32..64].copy_from_slice(&validator.bls_public_key[..32]);
        signature[64..96].copy_from_slice(&validator.validator_id);
        
        BlsPartialSignature {
            signature: signature.to_vec(),
            validator_id: validator.validator_id,
            public_key: validator.bls_public_key.clone(),
        }
    }
    
    /// Aggregate BLS partial signatures
    pub fn aggregate_signatures(&self, partials: &[BlsPartialSignature]) -> Result<BlsAggregateSignature, String> {
        if partials.len() < self.config.bls_threshold as usize {
            return Err(format!("Insufficient signatures: {} < {}", partials.len(), self.config.bls_threshold));
        }
        
        // Simplified aggregation (would use actual BLS aggregation in production)
        let mut aggregate = [0u8; 48];
        let mut bitmap = 0u32;
        
        for (i, partial) in partials.iter().enumerate().take(32) {
            // XOR aggregation (simplified)
            for j in 0..48 {
                aggregate[j] ^= partial.signature[j];
            }
            bitmap |= 1 << i;
        }
        
        Ok(BlsAggregateSignature {
            signature: aggregate.to_vec(),
            bitmap,
            signer_count: partials.len() as u8,
        })
    }
    
    /// Verify BLS aggregate signature
    pub fn verify_aggregate(&self, aggregate: &BlsAggregateSignature, validators: &[ValidatorIdentity], message: &[u8]) -> bool {
        if aggregate.signer_count < self.config.bls_threshold {
            return false;
        }
        
        // Simplified verification (would use actual BLS verification in production)
        let mut expected_aggregate = [0u8; 48];
        let mut count = 0;
        
        for (i, validator) in validators.iter().enumerate().take(32) {
            if (aggregate.bitmap & (1 << i)) != 0 {
                let partial = self.sign_partial(validator, message);
                for j in 0..48 {
                    expected_aggregate[j] ^= partial.signature[j];
                }
                count += 1;
            }
        }
        
        count == aggregate.signer_count && expected_aggregate.to_vec() == aggregate.signature
    }
}

/// BLS partial signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlsPartialSignature {
    #[serde(with = "serde_bytes")]
    pub signature: Vec<u8>,              // BLS signature (96 bytes)
    pub validator_id: [u8; 32],          // Signer validator ID
    #[serde(with = "serde_bytes")]
    pub public_key: Vec<u8>,             // Signer BLS public key (48 bytes)
}

/// BLS aggregate signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlsAggregateSignature {
    #[serde(with = "serde_bytes")]
    pub signature: Vec<u8>,       // Aggregated BLS signature (48 bytes)
    pub bitmap: u32,                     // Signer bitmap
    pub signer_count: u8,                // Number of signers
}

/// Post-quantum cryptography engine (Dilithium)
#[derive(Debug, Clone)]
pub struct PqcEngine {
    config: CryptoConfig,
}

impl PqcEngine {
    pub fn new(config: CryptoConfig) -> Self {
        Self { config }
    }
    
    /// Generate PQC epoch signature (off hot path)
    pub fn sign_epoch(&self, validator: &ValidatorIdentity, epoch_data: &[u8]) -> PqcSignature {
        // Simplified PQC signature using Blake3 (would use Dilithium in production)
        let mut hasher = blake3::Hasher::new();
        hasher.update(&validator.validator_id);
        hasher.update(b"dilithium");
        hasher.update(epoch_data);
        
        let signature_hash = hasher.finalize();
        let mut signature = [0u8; 2420]; // Dilithium-II signature size
        signature[..32].copy_from_slice(signature_hash.as_bytes());
        
        PqcSignature {
            signature: signature.to_vec(),
            algorithm: PqcAlgorithm::Dilithium2,
            public_key: validator.validator_id, // Simplified
        }
    }
    
    /// Verify PQC epoch signature
    pub fn verify_epoch(&self, signature: &PqcSignature, validator: &ValidatorIdentity, epoch_data: &[u8]) -> bool {
        // Simplified verification
        let mut hasher = blake3::Hasher::new();
        hasher.update(&validator.validator_id);
        hasher.update(b"dilithium");
        hasher.update(epoch_data);
        
        let expected_hash = hasher.finalize();
        signature.signature[..32] == expected_hash.as_bytes()[..32]
    }
}

/// Post-quantum signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqcSignature {
    #[serde(with = "serde_bytes")]
    pub signature: Vec<u8>,     // Dilithium signature (2420 bytes)
    pub algorithm: PqcAlgorithm,   // Algorithm used
    pub public_key: [u8; 32],            // Public key reference
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PqcAlgorithm {
    Dilithium2,
    Dilithium3,
    Dilithium5,
    Sphincs128s,
    Sphincs192s,
    Sphincs256s,
}

/// Main cryptographic engine for QGC-C²
#[derive(Debug)]
pub struct QgcCryptoEngine {
    config: CryptoConfig,
    vrf_engine: VrfEngine,
    bls_engine: BlsEngine,
    pqc_engine: PqcEngine,
    validators: HashMap<[u8; 32], ValidatorIdentity>,
}

impl QgcCryptoEngine {
    pub fn new(config: CryptoConfig) -> Self {
        Self {
            vrf_engine: VrfEngine::new(config.clone()),
            bls_engine: BlsEngine::new(config.clone()),
            pqc_engine: PqcEngine::new(config.clone()),
            config,
            validators: HashMap::new(),
        }
    }
    
    /// Add validator to crypto engine
    pub fn add_validator(&mut self, validator: ValidatorIdentity) {
        self.validators.insert(validator.validator_id, validator);
    }
    
    /// Select committee using VRF
    pub fn select_committee(&self, round: u64, committee_size: u8) -> Result<Vec<[u8; 32]>, String> {
        let mut selected = Vec::new();
        let input = format!("committee_selection_{}", round);
        
        for validator in self.validators.values() {
            if !validator.is_active {
                continue;
            }
            
            let vrf_proof = self.vrf_engine.generate_vrf_proof(validator, round, input.as_bytes());
            if vrf_proof.is_selected {
                selected.push(validator.validator_id);
            }
            
            if selected.len() >= committee_size as usize {
                break;
            }
        }
        
        if selected.len() < (committee_size as usize * 2 / 3) {
            return Err("Insufficient committee members selected".to_string());
        }
        
        Ok(selected)
    }
    
    /// Generate confidence attestation with crypto
    pub fn generate_ca(&self, validator_id: [u8; 32], round: u64, batch_id: [u8; 32], da_k: u8, da_m: u8) -> Result<ConfidenceAttestation, String> {
        let validator = self.validators.get(&validator_id)
            .ok_or("Validator not found")?;
        
        // Generate VRF proof for committee membership
        let vrf_input = format!("ca_{}_{}", round, hex::encode(batch_id));
        let vrf_proof = self.vrf_engine.generate_vrf_proof(validator, round, vrf_input.as_bytes());
        
        // Generate BLS partial signature
        let message = format!("ca_{}_{}_{}_{}", round, hex::encode(batch_id), da_k, da_m);
        let bls_partial = self.bls_engine.sign_partial(validator, message.as_bytes());
        
        Ok(ConfidenceAttestation {
            r: round,
            cid: batch_id,
            vrf_proof: vrf_proof.proof,
            da_k,
            da_m,
            parent_cc: [0; 16], // Would be filled with actual parent CC
            qos: 100,           // Quality of service metric
            qstep: 1,           // Quantized step
            bls_part: bls_partial.signature,
        })
    }
    
    /// Aggregate CAs into CC
    pub fn aggregate_cas(&self, cas: &[ConfidenceAttestation]) -> Result<ConfidenceCertificate, String> {
        if cas.is_empty() {
            return Err("No CAs to aggregate".to_string());
        }
        
        // Extract BLS partial signatures
        let mut partials = Vec::new();
        let mut bitmap = 0u32;
        
        for (i, ca) in cas.iter().enumerate().take(32) {
            // Find validator for this CA (simplified lookup)
            if let Some(validator) = self.validators.values().next() {
                partials.push(BlsPartialSignature {
                    signature: ca.bls_part.clone(),
                    validator_id: validator.validator_id,
                    public_key: validator.bls_public_key.clone(),
                });
                bitmap |= 1 << i;
            }
        }
        
        // Aggregate BLS signatures
        let bls_agg = self.bls_engine.aggregate_signatures(&partials)?;
        
        Ok(ConfidenceCertificate {
            r: cas[0].r,
            cid: cas[0].cid,
            bitmap,
            bls_agg: bls_agg.signature,
            qscore: 50, // Would be computed by quantized scorer
            da_ratio: 100,
            knot_k: 0,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }
    
    /// Verify confidence certificate
    pub fn verify_cc(&self, cc: &ConfidenceCertificate) -> bool {
        // Verify BLS aggregate signature
        let validators: Vec<_> = self.validators.values().cloned().collect();
        let message = format!("cc_{}_{}", cc.r, hex::encode(cc.cid));
        
        let bls_agg = BlsAggregateSignature {
            signature: cc.bls_agg.clone(),
            bitmap: cc.bitmap,
            signer_count: cc.bitmap.count_ones() as u8,
        };
        
        self.bls_engine.verify_aggregate(&bls_agg, &validators, message.as_bytes())
    }
    
    /// Generate epoch signature (PQC)
    pub fn sign_epoch(&self, validator_id: [u8; 32], epoch_data: &[u8]) -> Result<PqcSignature, String> {
        let validator = self.validators.get(&validator_id)
            .ok_or("Validator not found")?;
        
        Ok(self.pqc_engine.sign_epoch(validator, epoch_data))
    }
    
    /// Get memory usage estimate
    pub fn get_memory_usage(&self) -> usize {
        let validators_mem = self.validators.len() * std::mem::size_of::<ValidatorIdentity>();
        validators_mem + 4096 // Base crypto engine overhead
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validator_identity() {
        let validator_id = [1u8; 32];
        let identity = ValidatorIdentity::new(validator_id, 1000);
        
        assert_eq!(identity.validator_id, validator_id);
        assert_eq!(identity.stake, 1000);
        assert!(identity.is_active);
    }
    
    #[test]
    fn test_vrf_engine() {
        let config = CryptoConfig::default();
        let vrf = VrfEngine::new(config);
        let validator = ValidatorIdentity::new([1u8; 32], 1000);
        
        let proof = vrf.generate_vrf_proof(&validator, 1, b"test");
        assert!(vrf.verify_vrf_proof(&proof, &validator, 1, b"test"));
        assert!(!vrf.verify_vrf_proof(&proof, &validator, 2, b"test")); // Different round
    }
    
    #[test]
    fn test_bls_engine() {
        let config = CryptoConfig::default();
        let bls = BlsEngine::new(config);
        let validator = ValidatorIdentity::new([1u8; 32], 1000);
        
        let partial = bls.sign_partial(&validator, b"test message");
        assert_eq!(partial.validator_id, validator.validator_id);
        
        // Test aggregation with multiple partials
        let partials = vec![partial; 20]; // Above threshold
        let aggregate = bls.aggregate_signatures(&partials);
        assert!(aggregate.is_ok());
    }
    
    #[test]
    fn test_crypto_engine() {
        let config = CryptoConfig::default();
        let mut engine = QgcCryptoEngine::new(config);
        
        let validator = ValidatorIdentity::new([1u8; 32], 1000);
        engine.add_validator(validator.clone());
        
        // Test CA generation
        let ca = engine.generate_ca(validator.validator_id, 1, [2u8; 32], 10, 14);
        assert!(ca.is_ok());
        
        // Test committee selection
        let committee = engine.select_committee(1, 24);
        // May succeed or fail depending on VRF randomness
    }
    
    #[test]
    fn test_pqc_engine() {
        let config = CryptoConfig::default();
        let pqc = PqcEngine::new(config);
        let validator = ValidatorIdentity::new([1u8; 32], 1000);
        
        let signature = pqc.sign_epoch(&validator, b"epoch data");
        assert!(pqc.verify_epoch(&signature, &validator, b"epoch data"));
        assert!(!pqc.verify_epoch(&signature, &validator, b"different data"));
    }
}
