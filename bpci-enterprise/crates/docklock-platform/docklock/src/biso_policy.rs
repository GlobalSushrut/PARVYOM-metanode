//! Stage 35: DA Pinner Service (BISO Policy Engine)
//!
//! Implements the BISO (Block ISO) Policy Engine - a policy-as-code compliance system

use crate::error::{DockLockError, DockLockResult};
use crate::traffic_light::{DataClassification, TrafficLightState};
use blake3;
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

const BISO_POLICY_HASH: u8 = 0x1B;
const POLICY_EVALUATION_HASH: u8 = 0x1C;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeographicRegion {
    EU, US, UK, CA, AU, JP, SG, CH, BR, IN, CN, Global, Custom(String),
}

impl GeographicRegion {
    pub fn description(&self) -> &'static str {
        match self {
            GeographicRegion::EU => "European Union (GDPR compliance)",
            GeographicRegion::US => "United States (HIPAA, CCPA compliance)",
            GeographicRegion::UK => "United Kingdom (UK GDPR, DPA compliance)",
            GeographicRegion::Global => "Global (no regional restrictions)",
            _ => "Regional compliance requirements",
        }
    }

    pub fn requires_explicit_consent(&self) -> bool {
        matches!(self, GeographicRegion::EU | GeographicRegion::UK | GeographicRegion::BR | GeographicRegion::CN)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProcessingPurpose {
    Analytics, Marketing, CustomerService, Security, Compliance, Research, Financial, Healthcare, Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PolicyClassification {
    Public, Internal, Confidential, Restricted, TopSecret,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentStatus {
    Granted, Denied, Pending, Withdrawn, NotApplicable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BisoPolicy {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub description: String,
    pub classification: PolicyClassification,
    pub allowed_regions: Vec<GeographicRegion>,
    pub blocked_regions: Vec<GeographicRegion>,
    pub allowed_purposes: Vec<ProcessingPurpose>,
    pub applicable_data_types: Vec<DataClassification>,
    pub requires_consent: bool,
    pub requires_encryption: bool,
    pub requires_encryption_at_rest: bool,
    pub max_retention_seconds: Option<u64>,
    pub custom_rules: HashMap<String, String>,
    pub created_at: u64,
    pub updated_at: u64,
    pub created_by: String,
    pub signature: Option<String>,
    pub signer_pubkey: Option<String>,
}

impl BisoPolicy {
    pub fn new(name: String, version: String, description: String, 
               classification: PolicyClassification, created_by: String) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Self {
            id: Uuid::new_v4(), name, version, description, classification,
            allowed_regions: vec![GeographicRegion::Global], blocked_regions: Vec::new(),
            allowed_purposes: Vec::new(), applicable_data_types: Vec::new(),
            requires_consent: false, requires_encryption: true, requires_encryption_at_rest: false,
            max_retention_seconds: None, custom_rules: HashMap::new(),
            created_at: now, updated_at: now, created_by,
            signature: None, signer_pubkey: None,
        }
    }

    pub fn compute_signing_hash(&self) -> DockLockResult<[u8; 32]> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[BISO_POLICY_HASH]);
        let mut policy_for_hash = self.clone();
        policy_for_hash.signature = None;
        policy_for_hash.signer_pubkey = None;
        let encoded = bincode::serialize(&policy_for_hash)
            .map_err(|_| DockLockError::InvalidOperation("Failed to serialize policy".to_string()))?;
        hasher.update(&encoded);
        Ok(hasher.finalize().into())
    }

    pub fn sign(&mut self, signing_key: &SigningKey) -> DockLockResult<()> {
        let hash = self.compute_signing_hash()?;
        let signature = signing_key.sign(&hash);
        self.signature = Some(hex::encode(signature.to_bytes()));
        self.signer_pubkey = Some(hex::encode(signing_key.verifying_key().to_bytes()));
        Ok(())
    }

    pub fn verify_signature(&self) -> DockLockResult<bool> {
        let signature_b64 = self.signature.as_ref()
            .ok_or_else(|| DockLockError::InvalidOperation("No signature found".to_string()))?;
        let pubkey_b64 = self.signer_pubkey.as_ref()
            .ok_or_else(|| DockLockError::InvalidOperation("No public key found".to_string()))?;
        
        let signature_bytes = hex::decode(signature_b64)
            .map_err(|_| DockLockError::InvalidOperation("Invalid signature encoding".to_string()))?;
        let pubkey_bytes = hex::decode(pubkey_b64)
            .map_err(|_| DockLockError::InvalidOperation("Invalid public key encoding".to_string()))?;
        
        let mut pubkey_array = [0u8; 32];
        if pubkey_bytes.len() != 32 {
            return Err(DockLockError::CryptoError("Invalid public key length".to_string()));
        }
        pubkey_array.copy_from_slice(&pubkey_bytes);
        
        let mut signature_array = [0u8; 64];
        if signature_bytes.len() != 64 {
            return Err(DockLockError::CryptoError("Invalid signature length".to_string()));
        }
        signature_array.copy_from_slice(&signature_bytes);
        
        let verifying_key = VerifyingKey::from_bytes(&pubkey_array)
            .map_err(|e| DockLockError::CryptoError(format!("Invalid public key: {}", e)))?;
        let signature = ed25519_dalek::Signature::from_bytes(&signature_array);
        let hash = self.compute_signing_hash()?;
        
        match verifying_key.verify(&hash, &signature) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn is_region_allowed(&self, region: &GeographicRegion) -> bool {
        if self.blocked_regions.contains(region) { return false; }
        self.allowed_regions.contains(region) || self.allowed_regions.contains(&GeographicRegion::Global)
    }

    pub fn is_purpose_allowed(&self, purpose: &ProcessingPurpose) -> bool {
        self.allowed_purposes.is_empty() || self.allowed_purposes.contains(purpose)
    }

    pub fn is_data_type_applicable(&self, data_type: &DataClassification) -> bool {
        self.applicable_data_types.is_empty() || self.applicable_data_types.contains(data_type)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEvaluationContext {
    pub data_classification: DataClassification,
    pub source_region: GeographicRegion,
    pub destination_region: GeographicRegion,
    pub processing_purpose: ProcessingPurpose,
    pub consent_status: ConsentStatus,
    pub is_encrypted: bool,
    pub is_encrypted_at_rest: bool,
    pub data_age_seconds: u64,
    pub metadata: HashMap<String, String>,
}

impl PolicyEvaluationContext {
    pub fn new(data_classification: DataClassification, source_region: GeographicRegion,
               destination_region: GeographicRegion, processing_purpose: ProcessingPurpose) -> Self {
        Self {
            data_classification, source_region, destination_region, processing_purpose,
            consent_status: ConsentStatus::NotApplicable, is_encrypted: false,
            is_encrypted_at_rest: false, data_age_seconds: 0, metadata: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEvaluationResult {
    pub id: Uuid,
    pub policy_id: Uuid,
    pub decision: TrafficLightState,
    pub passed: bool,
    pub reason: String,
    pub violations: Vec<String>,
    pub warnings: Vec<String>,
    pub evaluated_at: u64,
    pub evaluation_duration_ms: u64,
    pub signature: Option<String>,
    pub evaluator_pubkey: Option<String>,
}

impl PolicyEvaluationResult {
    pub fn new(policy_id: Uuid, decision: TrafficLightState, passed: bool, reason: String) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Self {
            id: Uuid::new_v4(), policy_id, decision, passed, reason,
            violations: Vec::new(), warnings: Vec::new(), evaluated_at: now,
            evaluation_duration_ms: 0, signature: None, evaluator_pubkey: None,
        }
    }

    pub fn add_violation(&mut self, violation: String) {
        self.violations.push(violation);
        self.passed = false;
        if self.decision == TrafficLightState::Green {
            self.decision = TrafficLightState::Red;
        }
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
        if self.decision == TrafficLightState::Green && !self.warnings.is_empty() {
            self.decision = TrafficLightState::Yellow;
        }
    }

    pub fn sign(&mut self, signing_key: &SigningKey) -> DockLockResult<()> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[POLICY_EVALUATION_HASH]);
        let mut result_for_hash = self.clone();
        result_for_hash.signature = None;
        result_for_hash.evaluator_pubkey = None;
        let encoded = bincode::serialize(&result_for_hash)
            .map_err(|_| DockLockError::InvalidOperation("Failed to serialize evaluation result".to_string()))?;
        hasher.update(&encoded);
        let hash = hasher.finalize();
        
        let signature = signing_key.sign(hash.as_bytes());
        self.signature = Some(hex::encode(signature.to_bytes()));
        self.evaluator_pubkey = Some(hex::encode(signing_key.verifying_key().to_bytes()));
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PolicyEngineStats {
    pub total_evaluations: u64,
    pub passed_evaluations: u64,
    pub failed_evaluations: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl PolicyEngineStats {
    pub fn update_evaluation(&mut self, result: &PolicyEvaluationResult, cache_hit: bool) {
        self.total_evaluations += 1;
        if result.passed { self.passed_evaluations += 1; } else { self.failed_evaluations += 1; }
        if cache_hit { self.cache_hits += 1; } else { self.cache_misses += 1; }
    }
}

#[derive(Debug, Clone)]
struct CacheEntry {
    result: PolicyEvaluationResult,
    expires_at: SystemTime,
}

impl CacheEntry {
    fn new(result: PolicyEvaluationResult, ttl: Duration) -> Self {
        Self { result, expires_at: SystemTime::now() + ttl }
    }

    fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }
}

#[derive(Debug)]
pub struct BisoPolicyEngine {
    pub id: Uuid,
    pub name: String,
    policies: Arc<RwLock<HashMap<Uuid, BisoPolicy>>>,
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    stats: Arc<RwLock<PolicyEngineStats>>,
    cache_ttl: Duration,
    max_cache_size: usize,
    signing_key: SigningKey,
}

impl BisoPolicyEngine {
    pub fn new(name: String) -> Self {
        let signing_key = SigningKey::generate(&mut rand::thread_rng());
        Self {
            id: Uuid::new_v4(), name,
            policies: Arc::new(RwLock::new(HashMap::new())),
            cache: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(PolicyEngineStats::default())),
            cache_ttl: Duration::from_secs(300), max_cache_size: 10000, signing_key,
        }
    }

    pub fn register_policy(&self, policy: BisoPolicy) -> DockLockResult<()> {
        if policy.signature.is_some() && !policy.verify_signature()? {
            return Err(DockLockError::CryptoError("Invalid policy signature".to_string()));
        }
        let mut policies = self.policies.write()
            .map_err(|e| DockLockError::InvalidOperation(format!("Failed to acquire write lock: {}", e)))?;
        policies.insert(policy.id, policy);
        Ok(())
    }

    pub fn get_policy(&self, policy_id: &Uuid) -> DockLockResult<Option<BisoPolicy>> {
        let policies = self.policies.read()
            .map_err(|e| DockLockError::InvalidOperation(format!("Failed to acquire read lock: {}", e)))?;
        Ok(policies.get(policy_id).cloned())
    }

    pub fn evaluate_policy(&self, policy_id: &Uuid, context: &PolicyEvaluationContext) -> DockLockResult<PolicyEvaluationResult> {
        let start_time = SystemTime::now();
        
        let policy = self.get_policy(policy_id)?
            .ok_or_else(|| DockLockError::NotFound(format!("Policy not found: {}", policy_id)))?;

        let mut result = PolicyEvaluationResult::new(*policy_id, TrafficLightState::Green, true, "Policy evaluation passed".to_string());

        // Check region restrictions
        if !policy.is_region_allowed(&context.source_region) {
            result.add_violation(format!("Source region {:?} is not allowed", context.source_region));
        }
        if !policy.is_region_allowed(&context.destination_region) {
            result.add_violation(format!("Destination region {:?} is not allowed", context.destination_region));
        }

        // Check purpose binding
        if !policy.is_purpose_allowed(&context.processing_purpose) {
            result.add_violation(format!("Processing purpose {:?} is not allowed", context.processing_purpose));
        }

        // Check consent requirements
        if policy.requires_consent {
            match context.consent_status {
                ConsentStatus::Granted => {},
                ConsentStatus::Denied => result.add_violation("Consent denied for data processing".to_string()),
                ConsentStatus::Withdrawn => result.add_violation("Consent withdrawn for data processing".to_string()),
                ConsentStatus::Pending => result.add_warning("Consent pending for data processing".to_string()),
                ConsentStatus::NotApplicable => {
                    if context.source_region.requires_explicit_consent() {
                        result.add_violation("Explicit consent required for this region".to_string());
                    }
                }
            }
        }

        // Check encryption requirements
        if policy.requires_encryption && !context.is_encrypted {
            result.add_violation("Data must be encrypted in transit".to_string());
        }
        if policy.requires_encryption_at_rest && !context.is_encrypted_at_rest {
            result.add_warning("Data should be encrypted at rest".to_string());
        }

        // Set evaluation duration
        let duration = start_time.elapsed().unwrap_or(Duration::ZERO);
        result.evaluation_duration_ms = duration.as_millis() as u64;

        // Sign the result
        result.sign(&self.signing_key)?;

        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            stats.update_evaluation(&result, false);
        }

        Ok(result)
    }

    pub fn get_stats(&self) -> DockLockResult<PolicyEngineStats> {
        let stats = self.stats.read()
            .map_err(|e| DockLockError::InvalidOperation(format!("Failed to acquire stats lock: {}", e)))?;
        Ok(stats.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biso_policy_creation() {
        let policy = BisoPolicy::new(
            "Test Policy".to_string(),
            "1.0".to_string(),
            "Test policy for unit testing".to_string(),
            PolicyClassification::Internal,
            "test_user".to_string(),
        );

        assert_eq!(policy.name, "Test Policy");
        assert_eq!(policy.version, "1.0");
        assert_eq!(policy.classification, PolicyClassification::Internal);
        assert!(policy.allowed_regions.contains(&GeographicRegion::Global));
    }

    #[test]
    fn test_policy_signing_and_verification() {
        let signing_key = SigningKey::generate(&mut rand::thread_rng());
        let mut policy = BisoPolicy::new(
            "Signed Policy".to_string(),
            "1.0".to_string(),
            "Policy with signature".to_string(),
            PolicyClassification::Confidential,
            "signer".to_string(),
        );

        assert!(policy.sign(&signing_key).is_ok());
        assert!(policy.signature.is_some());
        assert!(policy.verify_signature().unwrap());
    }

    #[test]
    fn test_geographic_region_compliance() {
        assert!(GeographicRegion::EU.requires_explicit_consent());
        assert!(GeographicRegion::UK.requires_explicit_consent());
        assert!(!GeographicRegion::US.requires_explicit_consent());
        assert_eq!(GeographicRegion::EU.description(), "European Union (GDPR compliance)");
    }

    #[test]
    fn test_policy_evaluation_context() {
        let context = PolicyEvaluationContext::new(
            DataClassification::PII,
            GeographicRegion::US,
            GeographicRegion::EU,
            ProcessingPurpose::Analytics,
        );

        assert_eq!(context.data_classification, DataClassification::PII);
        assert_eq!(context.source_region, GeographicRegion::US);
        assert_eq!(context.destination_region, GeographicRegion::EU);
        assert_eq!(context.processing_purpose, ProcessingPurpose::Analytics);
    }

    #[test]
    fn test_biso_policy_engine_creation() {
        let engine = BisoPolicyEngine::new("Test Engine".to_string());
        assert_eq!(engine.name, "Test Engine");
        assert!(engine.get_stats().is_ok());
    }

    #[test]
    fn test_policy_registration_and_retrieval() {
        let engine = BisoPolicyEngine::new("Test Engine".to_string());
        let policy = BisoPolicy::new(
            "Test Policy".to_string(),
            "1.0".to_string(),
            "Test policy".to_string(),
            PolicyClassification::Internal,
            "test_user".to_string(),
        );
        let policy_id = policy.id;

        assert!(engine.register_policy(policy).is_ok());
        let retrieved = engine.get_policy(&policy_id).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test Policy");
    }

    #[test]
    fn test_policy_evaluation_pass() {
        let engine = BisoPolicyEngine::new("Test Engine".to_string());
        let mut policy = BisoPolicy::new(
            "Permissive Policy".to_string(),
            "1.0".to_string(),
            "Allows all operations".to_string(),
            PolicyClassification::Public,
            "test_user".to_string(),
        );
        policy.allowed_regions = vec![GeographicRegion::Global];
        policy.requires_consent = false;
        policy.requires_encryption = false;

        let policy_id = policy.id;
        engine.register_policy(policy).unwrap();

        let context = PolicyEvaluationContext::new(
            DataClassification::Public,
            GeographicRegion::US,
            GeographicRegion::EU,
            ProcessingPurpose::Analytics,
        );

        let result = engine.evaluate_policy(&policy_id, &context).unwrap();
        assert!(result.passed);
        assert_eq!(result.decision, TrafficLightState::Green);
        assert!(result.violations.is_empty());
    }

    #[test]
    fn test_policy_evaluation_violation() {
        let engine = BisoPolicyEngine::new("Test Engine".to_string());
        let mut policy = BisoPolicy::new(
            "Restrictive Policy".to_string(),
            "1.0".to_string(),
            "Blocks EU destinations".to_string(),
            PolicyClassification::Restricted,
            "test_user".to_string(),
        );
        policy.blocked_regions = vec![GeographicRegion::EU];
        policy.requires_consent = true;
        policy.requires_encryption = true;

        let policy_id = policy.id;
        engine.register_policy(policy).unwrap();

        let context = PolicyEvaluationContext::new(
            DataClassification::PII,
            GeographicRegion::US,
            GeographicRegion::EU,
            ProcessingPurpose::Marketing,
        );

        let result = engine.evaluate_policy(&policy_id, &context).unwrap();
        assert!(!result.passed);
        assert_eq!(result.decision, TrafficLightState::Red);
        assert!(!result.violations.is_empty());
    }

    #[test]
    fn test_policy_evaluation_result_signing() {
        let signing_key = SigningKey::generate(&mut rand::thread_rng());
        let mut result = PolicyEvaluationResult::new(
            Uuid::new_v4(),
            TrafficLightState::Green,
            true,
            "Test evaluation".to_string(),
        );

        assert!(result.sign(&signing_key).is_ok());
        assert!(result.signature.is_some());
    }
}
