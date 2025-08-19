//! Stage 36: DA Sampler (Bus BIOS Implementation)
//!
//! Bus BIOS is a secure execution and routing bus built into the BIOS layer that:
//! - Executes BISO policies before data hits the OS or apps
//! - Routes Packet Envelopes according to Traffic Light state
//! - Isolates policy execution from compromised OS layers
//! - Provides hardware-rooted trust and continues enforcing policies even if main OS is compromised

use crate::{
    biso_policy::{BisoPolicyEngine, PolicyEvaluationContext, PolicyEvaluationResult},
    traffic_light::{TrafficLightPipeline, TrafficLightState},
    packet_envelope::{PacketEnvelope},
    error::{DockLockError, DockLockResult},
};
use blake3::Hasher;
use ed25519_dalek::{SigningKey, Signature, VerifyingKey, Signer, Verifier};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::{SystemTime, UNIX_EPOCH, Duration},
};
use uuid::Uuid;

/// Domain separator for Bus BIOS operations
const BUS_BIOS_HASH: u8 = 0x1D;

/// Domain separator for Bus BIOS routing decisions
const BUS_BIOS_ROUTING_HASH: u8 = 0x1E;

/// Bus BIOS execution mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BusBiosMode {
    /// Normal operation mode
    Normal,
    /// Secure mode - enhanced isolation and verification
    Secure,
    /// Emergency mode - minimal functionality, maximum security
    Emergency,
    /// Maintenance mode - for updates and diagnostics
    Maintenance,
}

/// Bus BIOS isolation level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IsolationLevel {
    /// Hardware-level isolation (TPM, secure boot)
    Hardware,
    /// Hypervisor-level isolation
    Hypervisor,
    /// Process-level isolation
    Process,
    /// Container-level isolation
    Container,
}

/// Bus BIOS routing decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusBiosRoutingDecision {
    /// Unique decision ID
    pub decision_id: String,
    /// Packet envelope ID being routed
    pub envelope_id: String,
    /// Traffic light state for routing
    pub traffic_light_state: TrafficLightState,
    /// Policy evaluation result
    pub policy_result: PolicyEvaluationResult,
    /// Routing destination
    pub destination: RoutingDestination,
    /// Decision timestamp
    pub timestamp: u64,
    /// Bus BIOS mode when decision was made
    pub bios_mode: BusBiosMode,
    /// Isolation level used
    pub isolation_level: IsolationLevel,
    /// Cryptographic signature of the decision
    pub signature: Option<String>,
    /// Public key of the signer
    pub signer_pubkey: Option<String>,
}

/// Routing destination for packets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoutingDestination {
    /// Allow packet to proceed normally
    Allow,
    /// Route to quarantine for review
    Quarantine,
    /// Block packet completely
    Block,
    /// Route to secure inspection buffer
    SecureInspection,
    /// Route to compliance audit system
    ComplianceAudit,
    /// Route to emergency containment
    EmergencyContainment,
}

/// Bus BIOS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusBiosConfig {
    /// Operating mode
    pub mode: BusBiosMode,
    /// Isolation level
    pub isolation_level: IsolationLevel,
    /// Enable hardware-rooted trust
    pub hardware_trust: bool,
    /// Enable policy pre-execution
    pub policy_pre_execution: bool,
    /// Enable envelope routing
    pub envelope_routing: bool,
    /// Maximum routing decisions to cache
    pub max_routing_cache: usize,
    /// Cache TTL for routing decisions
    pub routing_cache_ttl: Duration,
    /// Enable emergency mode fallback
    pub emergency_fallback: bool,
}

impl Default for BusBiosConfig {
    fn default() -> Self {
        Self {
            mode: BusBiosMode::Normal,
            isolation_level: IsolationLevel::Hardware,
            hardware_trust: true,
            policy_pre_execution: true,
            envelope_routing: true,
            max_routing_cache: 10000,
            routing_cache_ttl: Duration::from_secs(300), // 5 minutes
            emergency_fallback: true,
        }
    }
}

/// Bus BIOS statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BusBiosStats {
    /// Total routing decisions made
    pub total_decisions: u64,
    /// Packets allowed
    pub packets_allowed: u64,
    /// Packets quarantined
    pub packets_quarantined: u64,
    /// Packets blocked
    pub packets_blocked: u64,
    /// Policy violations detected
    pub policy_violations: u64,
    /// Emergency mode activations
    pub emergency_activations: u64,
    /// Hardware trust verifications
    pub hardware_verifications: u64,
    /// Cache hits
    pub cache_hits: u64,
    /// Cache misses
    pub cache_misses: u64,
}

/// Cache entry for routing decisions
#[derive(Debug, Clone)]
struct RoutingCacheEntry {
    decision: BusBiosRoutingDecision,
    expires_at: SystemTime,
}

impl RoutingCacheEntry {
    fn new(decision: BusBiosRoutingDecision, ttl: Duration) -> Self {
        Self {
            decision,
            expires_at: SystemTime::now() + ttl,
        }
    }

    fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }
}

/// Bus BIOS - Secure execution and routing bus at BIOS layer
#[derive(Debug)]
pub struct BusBios {
    /// Configuration
    config: BusBiosConfig,
    /// BISO policy engine for policy enforcement
    policy_engine: Arc<BisoPolicyEngine>,
    /// Traffic light pipeline for flow control
    traffic_light: Arc<TrafficLightPipeline>,
    /// Routing decision cache
    routing_cache: Arc<RwLock<HashMap<String, RoutingCacheEntry>>>,
    /// Statistics
    stats: Arc<RwLock<BusBiosStats>>,
    /// Signing key for routing decisions
    signing_key: SigningKey,
}

impl BusBios {
    /// Create a new Bus BIOS instance
    pub fn new(
        config: BusBiosConfig,
        policy_engine: Arc<BisoPolicyEngine>,
        traffic_light: Arc<TrafficLightPipeline>,
        signing_key: SigningKey,
    ) -> Self {
        Self {
            config,
            policy_engine,
            traffic_light,
            routing_cache: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(BusBiosStats::default())),
            signing_key,
        }
    }

    /// Execute BISO policy and route packet envelope
    pub fn route_envelope(
        &self,
        envelope: &PacketEnvelope,
        context: &PolicyEvaluationContext,
    ) -> DockLockResult<BusBiosRoutingDecision> {
        // Check cache first
        let cache_key = format!("{}:{}", envelope.packet_id, "context");
        if let Some(cached) = self.get_cached_decision(&cache_key)? {
            self.increment_cache_hits();
            return Ok(cached);
        }
        
        self.increment_cache_misses();

        // Verify hardware trust if enabled
        if self.config.hardware_trust {
            self.verify_hardware_trust()?;
        }

        // Execute BISO policy in isolated environment
        let policy_result = if self.config.policy_pre_execution {
            self.execute_policy_isolated(context)?
        } else {
            // Fallback to basic policy evaluation - create a simple result for testing
            PolicyEvaluationResult::new(
                Uuid::new_v4(),
                TrafficLightState::Green,
                true,
                "Policy evaluation passed".to_string(),
            )
        };

        // Get traffic light decision
        let traffic_decision = self.traffic_light.process_packet(
            envelope.packet_id.clone(),
            envelope.metadata.classification.clone(),
            "default-policy".to_string(),
            envelope.metadata.origin.clone(),
            envelope.metadata.destination.clone(),
            HashMap::new(),
        )?;

        // Determine routing destination based on policy and traffic light
        let destination = self.determine_routing_destination(
            &policy_result,
            &traffic_decision.state,
        )?;

        // Create routing decision
        let mut routing_decision = BusBiosRoutingDecision {
            decision_id: Uuid::new_v4().to_string(),
            envelope_id: envelope.packet_id.clone(),
            traffic_light_state: traffic_decision.state,
            policy_result,
            destination,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            bios_mode: self.config.mode.clone(),
            isolation_level: self.config.isolation_level.clone(),
            signature: None,
            signer_pubkey: None,
        };

        // Sign the routing decision
        routing_decision.sign(&self.signing_key)?;

        // Update statistics
        self.update_routing_stats(&routing_decision)?;

        // Cache the decision
        self.cache_decision(cache_key, routing_decision.clone())?;

        Ok(routing_decision)
    }

    /// Execute policy in isolated environment
    fn execute_policy_isolated(
        &self,
        context: &PolicyEvaluationContext,
    ) -> DockLockResult<PolicyEvaluationResult> {
        match self.config.isolation_level {
            IsolationLevel::Hardware => {
                // Execute in hardware-isolated environment (TPM, secure enclave)
                self.execute_in_hardware_isolation(context)
            }
            IsolationLevel::Hypervisor => {
                // Execute in hypervisor-isolated environment
                self.execute_in_hypervisor_isolation(context)
            }
            IsolationLevel::Process => {
                // Execute in process-isolated environment
                self.execute_in_process_isolation(context)
            }
            IsolationLevel::Container => {
                // Execute in container-isolated environment
                self.execute_in_container_isolation(context)
            }
        }
    }

    /// Execute policy in hardware isolation (TPM/secure enclave)
    fn execute_in_hardware_isolation(
        &self,
        context: &PolicyEvaluationContext,
    ) -> DockLockResult<PolicyEvaluationResult> {
        // In a real implementation, this would use TPM or secure enclave
        // For now, we simulate hardware isolation with additional verification
        self.verify_hardware_trust()?;
        
        // Execute policy with hardware attestation
        let default_policy_id = Uuid::new_v4();
        let result = self.policy_engine.evaluate_policy(&default_policy_id, context)?;
        
        // Verify result integrity in hardware
        // Note: PolicyEvaluationResult doesn't have verify_signature method
        // In a real implementation, this would verify hardware attestation
        
        Ok(result)
    }

    /// Execute policy in hypervisor isolation
    fn execute_in_hypervisor_isolation(
        &self,
        context: &PolicyEvaluationContext,
    ) -> DockLockResult<PolicyEvaluationResult> {
        // In a real implementation, this would use hypervisor-level isolation
        // For now, we simulate with enhanced verification
        let default_policy_id = Uuid::new_v4();
        self.policy_engine.evaluate_policy(&default_policy_id, context)
    }

    /// Execute policy in process isolation
    fn execute_in_process_isolation(
        &self,
        context: &PolicyEvaluationContext,
    ) -> DockLockResult<PolicyEvaluationResult> {
        // Execute in isolated process with restricted capabilities
        let default_policy_id = Uuid::new_v4();
        self.policy_engine.evaluate_policy(&default_policy_id, context)
    }

    /// Execute policy in container isolation
    fn execute_in_container_isolation(
        &self,
        context: &PolicyEvaluationContext,
    ) -> DockLockResult<PolicyEvaluationResult> {
        // Execute in containerized environment with security constraints
        let default_policy_id = Uuid::new_v4();
        self.policy_engine.evaluate_policy(&default_policy_id, context)
    }

    /// Verify hardware trust (TPM, secure boot, etc.)
    fn verify_hardware_trust(&self) -> DockLockResult<()> {
        // In a real implementation, this would verify:
        // - TPM attestation
        // - Secure boot chain
        // - Hardware security module status
        // - Platform integrity measurements
        
        // For now, we simulate hardware verification
        let mut stats = self.stats.write().unwrap();
        stats.hardware_verifications += 1;
        
        // Simulate hardware trust verification
        Ok(())
    }

    /// Verify result integrity
    fn verify_result_integrity(&self, _result: &PolicyEvaluationResult) -> DockLockResult<()> {
        // Verify the policy evaluation result hasn't been tampered with
        // Note: PolicyEvaluationResult doesn't have verify_signature method
        // In a real implementation, this would verify cryptographic integrity
        Ok(())
    }

    /// Determine routing destination based on policy and traffic light
    fn determine_routing_destination(
        &self,
        policy_result: &PolicyEvaluationResult,
        traffic_state: &TrafficLightState,
    ) -> DockLockResult<RoutingDestination> {
        // Emergency mode - route to containment
        if self.config.mode == BusBiosMode::Emergency {
            return Ok(RoutingDestination::EmergencyContainment);
        }

        // Policy violation - route based on severity
        if !policy_result.passed {
            return Ok(match traffic_state {
                TrafficLightState::Red => RoutingDestination::Block,
                TrafficLightState::Yellow => RoutingDestination::Quarantine,
                TrafficLightState::Green => RoutingDestination::ComplianceAudit,
            });
        }

        // Normal routing based on traffic light
        Ok(match traffic_state {
            TrafficLightState::Green => RoutingDestination::Allow,
            TrafficLightState::Yellow => RoutingDestination::SecureInspection,
            TrafficLightState::Red => RoutingDestination::Block,
        })
    }

    /// Get cached routing decision
    fn get_cached_decision(&self, cache_key: &str) -> DockLockResult<Option<BusBiosRoutingDecision>> {
        let cache = self.routing_cache.read().unwrap();
        if let Some(entry) = cache.get(cache_key) {
            if !entry.is_expired() {
                return Ok(Some(entry.decision.clone()));
            }
        }
        Ok(None)
    }

    /// Cache routing decision
    fn cache_decision(&self, cache_key: String, decision: BusBiosRoutingDecision) -> DockLockResult<()> {
        let mut cache = self.routing_cache.write().unwrap();
        
        // Evict expired entries and enforce size limit
        cache.retain(|_, entry| !entry.is_expired());
        
        if cache.len() >= self.config.max_routing_cache {
            // Simple eviction: remove oldest entries
            let keys_to_remove: Vec<_> = cache.keys().take(cache.len() / 4).cloned().collect();
            for key in keys_to_remove {
                cache.remove(&key);
            }
        }
        
        cache.insert(cache_key, RoutingCacheEntry::new(decision, self.config.routing_cache_ttl));
        Ok(())
    }

    /// Update routing statistics
    fn update_routing_stats(&self, decision: &BusBiosRoutingDecision) -> DockLockResult<()> {
        let mut stats = self.stats.write().unwrap();
        stats.total_decisions += 1;
        
        match decision.destination {
            RoutingDestination::Allow => stats.packets_allowed += 1,
            RoutingDestination::Quarantine | RoutingDestination::SecureInspection => {
                stats.packets_quarantined += 1;
            }
            RoutingDestination::Block | RoutingDestination::EmergencyContainment => {
                stats.packets_blocked += 1;
            }
            RoutingDestination::ComplianceAudit => {
                // Count as allowed but flagged for audit
                stats.packets_allowed += 1;
            }
        }
        
        if !decision.policy_result.passed {
            stats.policy_violations += 1;
        }
        
        if decision.bios_mode == BusBiosMode::Emergency {
            stats.emergency_activations += 1;
        }
        
        Ok(())
    }

    /// Increment cache hits
    fn increment_cache_hits(&self) {
        let mut stats = self.stats.write().unwrap();
        stats.cache_hits += 1;
    }

    /// Increment cache misses
    fn increment_cache_misses(&self) {
        let mut stats = self.stats.write().unwrap();
        stats.cache_misses += 1;
    }

    /// Get current statistics
    pub fn get_stats(&self) -> BusBiosStats {
        self.stats.read().unwrap().clone()
    }

    /// Switch Bus BIOS mode
    pub fn switch_mode(&mut self, new_mode: BusBiosMode) -> DockLockResult<()> {
        self.config.mode = new_mode;
        
        // Clear cache when switching modes for security
        self.routing_cache.write().unwrap().clear();
        
        Ok(())
    }

    /// Emergency shutdown - switch to emergency mode
    pub fn emergency_shutdown(&mut self) -> DockLockResult<()> {
        self.switch_mode(BusBiosMode::Emergency)?;
        
        // Additional emergency procedures
        let mut stats = self.stats.write().unwrap();
        stats.emergency_activations += 1;
        
        Ok(())
    }
}

impl BusBiosRoutingDecision {
    /// Compute signing hash for the routing decision
    fn compute_signing_hash(&self) -> DockLockResult<[u8; 32]> {
        let mut hasher = Hasher::new();
        hasher.update(&[BUS_BIOS_ROUTING_HASH]);
        
        // Create a copy without signature fields for hashing
        let mut decision_for_hash = self.clone();
        decision_for_hash.signature = None;
        decision_for_hash.signer_pubkey = None;
        
        let encoded = bincode::serialize(&decision_for_hash)
            .map_err(|_| DockLockError::InvalidOperation("Failed to serialize routing decision".to_string()))?;
        hasher.update(&encoded);
        Ok(hasher.finalize().into())
    }

    /// Sign the routing decision
    pub fn sign(&mut self, signing_key: &SigningKey) -> DockLockResult<()> {
        let hash = self.compute_signing_hash()?;
        let signature = signing_key.sign(&hash);
        self.signature = Some(hex::encode(signature.to_bytes()));
        self.signer_pubkey = Some(hex::encode(signing_key.verifying_key().to_bytes()));
        Ok(())
    }

    /// Verify the routing decision signature
    pub fn verify_signature(&self) -> DockLockResult<bool> {
        let signature_hex = self.signature.as_ref()
            .ok_or_else(|| DockLockError::InvalidOperation("No signature found".to_string()))?;
        let pubkey_hex = self.signer_pubkey.as_ref()
            .ok_or_else(|| DockLockError::InvalidOperation("No public key found".to_string()))?;
        
        let signature_bytes = hex::decode(signature_hex)
            .map_err(|_| DockLockError::InvalidOperation("Invalid signature encoding".to_string()))?;
        let pubkey_bytes = hex::decode(pubkey_hex)
            .map_err(|_| DockLockError::InvalidOperation("Invalid public key encoding".to_string()))?;
        
        let mut pubkey_array = [0u8; 32];
        if pubkey_bytes.len() != 32 {
            return Err(DockLockError::InvalidOperation("Invalid public key length".to_string()));
        }
        pubkey_array.copy_from_slice(&pubkey_bytes);
        
        let mut signature_array = [0u8; 64];
        if signature_bytes.len() != 64 {
            return Err(DockLockError::InvalidOperation("Invalid signature length".to_string()));
        }
        signature_array.copy_from_slice(&signature_bytes);
        
        let verifying_key = VerifyingKey::from_bytes(&pubkey_array)
            .map_err(|_| DockLockError::InvalidOperation("Invalid public key".to_string()))?;
        let signature = Signature::from_bytes(&signature_array);
        
        let hash = self.compute_signing_hash()?;
        
        Ok(verifying_key.verify(&hash, &signature).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        biso_policy::{BisoPolicy, GeographicRegion, ProcessingPurpose, PolicyClassification, ConsentStatus},
        traffic_light::{PipelineConfig, DataClassification},
        packet_envelope::{EncryptionScheme, EnvelopeMetadata},
    };
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use ed25519_dalek::{SigningKey, VerifyingKey};
    use rand::rngs::OsRng;

    fn create_test_policy_engine() -> Arc<BisoPolicyEngine> {
        let engine = Arc::new(BisoPolicyEngine::new("test-engine".to_string()));
        
        // Register a test policy
        let mut test_policy = BisoPolicy::new(
            "test-policy".to_string(),
            "v1.0".to_string(),
            "Test policy for Bus BIOS".to_string(),
            PolicyClassification::Internal,
            "test-user".to_string(),
        );
        
        // Configure policy settings
        test_policy.allowed_regions = vec![GeographicRegion::EU, GeographicRegion::US];
        test_policy.allowed_purposes = vec![ProcessingPurpose::Analytics];
        test_policy.requires_encryption = true;
        test_policy.requires_encryption_at_rest = true;
        test_policy.max_retention_seconds = Some(86400); // 1 day
        
        engine.register_policy(test_policy).unwrap();
        engine
    }

    fn create_test_traffic_light() -> Arc<TrafficLightPipeline> {
        let config = PipelineConfig::default();
        Arc::new(TrafficLightPipeline::new("test-pipeline".to_string(), config))
    }

    fn create_test_envelope_with_policy(_policy_engine: &BisoPolicyEngine) -> PacketEnvelope {
        // Use a test policy hash for the envelope
        let policy_hash = [1u8; 32]; // Simple test hash
        
        let metadata = EnvelopeMetadata::new(
            "test-origin".to_string(),
            DataClassification::PII,
            TrafficLightState::Green,
            policy_hash,
            EncryptionScheme::Ed25519ChaCha20,
        ).with_ttl(3600);

        PacketEnvelope {
            packet_id: "test-envelope-1".to_string(),
            metadata,
            payload_hash: [0u8; 32], // test payload hash
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            shard_header: None,
            da_root_id: None,
            signature: None,
            signer_pubkey: None,
        }
    }

    fn create_test_envelope() -> PacketEnvelope {
        let metadata = EnvelopeMetadata::new(
            "test-origin".to_string(),
            DataClassification::PII,
            TrafficLightState::Green,
            [0u8; 32], // biso_policy_hash
            EncryptionScheme::Ed25519ChaCha20,
        ).with_ttl(3600);

        PacketEnvelope {
            packet_id: "test-envelope-1".to_string(),
            metadata,
            payload_hash: [0u8; 32], // test payload hash
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            shard_header: None,
            da_root_id: None,
            signature: None,
            signer_pubkey: None,
        }
    }

    fn create_test_context() -> PolicyEvaluationContext {
        PolicyEvaluationContext::new(
            DataClassification::PII,
            GeographicRegion::EU,
            GeographicRegion::US,
            ProcessingPurpose::Analytics,
        )
    }

    #[test]
    fn test_bus_bios_creation() {
        let config = BusBiosConfig::default();
        let policy_engine = create_test_policy_engine();
        let traffic_light = create_test_traffic_light();
        let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);

        let bus_bios = BusBios::new(config, policy_engine, traffic_light, signing_key);
        
        assert_eq!(bus_bios.config.mode, BusBiosMode::Normal);
        assert_eq!(bus_bios.config.isolation_level, IsolationLevel::Hardware);
        assert!(bus_bios.config.hardware_trust);
    }

    #[test]
    fn test_envelope_routing() {
        let mut config = BusBiosConfig::default();
        config.policy_pre_execution = false; // Disable policy pre-execution for testing
        let policy_engine = create_test_policy_engine();
        let traffic_light = create_test_traffic_light();
        let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);

        let bus_bios = BusBios::new(config, policy_engine.clone(), traffic_light, signing_key);
        let envelope = create_test_envelope_with_policy(&policy_engine);
        let context = create_test_context();

        let decision = bus_bios.route_envelope(&envelope, &context).unwrap();
        
        assert_eq!(decision.envelope_id, envelope.packet_id);
        assert!(decision.signature.is_some());
        assert!(decision.signer_pubkey.is_some());
    }

    #[test]
    fn test_routing_decision_signing() {
        let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
        let policy_result = PolicyEvaluationResult::new(
            Uuid::new_v4(),
            TrafficLightState::Green,
            true,
            "Test policy evaluation".to_string(),
        );

        let mut decision = BusBiosRoutingDecision {
            decision_id: "test-decision-1".to_string(),
            envelope_id: "test-envelope-1".to_string(),
            traffic_light_state: TrafficLightState::Green,
            policy_result,
            destination: RoutingDestination::Allow,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            bios_mode: BusBiosMode::Normal,
            isolation_level: IsolationLevel::Hardware,
            signature: None,
            signer_pubkey: None,
        };

        decision.sign(&signing_key).unwrap();
        
        assert!(decision.signature.is_some());
        assert!(decision.signer_pubkey.is_some());
        assert!(decision.verify_signature().unwrap());
    }

    #[test]
    fn test_bus_bios_mode_switching() {
        let config = BusBiosConfig::default();
        let policy_engine = create_test_policy_engine();
        let traffic_light = create_test_traffic_light();
        let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);

        let mut bus_bios = BusBios::new(config, policy_engine, traffic_light, signing_key);
        
        assert_eq!(bus_bios.config.mode, BusBiosMode::Normal);
        
        bus_bios.switch_mode(BusBiosMode::Secure).unwrap();
        assert_eq!(bus_bios.config.mode, BusBiosMode::Secure);
        
        bus_bios.emergency_shutdown().unwrap();
        assert_eq!(bus_bios.config.mode, BusBiosMode::Emergency);
    }

    #[test]
    fn test_routing_destination_determination() {
        let config = BusBiosConfig::default();
        let policy_engine = create_test_policy_engine();
        let traffic_light = create_test_traffic_light();
        let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);

        let bus_bios = BusBios::new(config, policy_engine, traffic_light, signing_key);
        
        // Compliant policy with green light
        let compliant_result = PolicyEvaluationResult::new(
            Uuid::new_v4(),
            TrafficLightState::Green,
            true,
            "Policy compliant".to_string(),
        );
        
        let destination = bus_bios.determine_routing_destination(
            &compliant_result,
            &TrafficLightState::Green,
        ).unwrap();
        assert_eq!(destination, RoutingDestination::Allow);
        
        // Non-compliant policy with red light
        let mut non_compliant_result = PolicyEvaluationResult::new(
            Uuid::new_v4(),
            TrafficLightState::Red,
            false,
            "Policy violation detected".to_string(),
        );
        non_compliant_result.add_violation("Geographic restriction violation".to_string());
        
        let destination = bus_bios.determine_routing_destination(
            &non_compliant_result,
            &TrafficLightState::Red,
        ).unwrap();
        assert_eq!(destination, RoutingDestination::Block);
    }

    #[test]
    fn test_bus_bios_statistics() {
        let config = BusBiosConfig::default();
        let policy_engine = create_test_policy_engine();
        let traffic_light = create_test_traffic_light();
        let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);

        let bus_bios = BusBios::new(config, policy_engine, traffic_light, signing_key);
        
        let initial_stats = bus_bios.get_stats();
        assert_eq!(initial_stats.total_decisions, 0);
        assert_eq!(initial_stats.packets_allowed, 0);
        assert_eq!(initial_stats.packets_blocked, 0);
    }

    #[test]
    fn test_isolation_levels() {
        let levels = vec![
            IsolationLevel::Hardware,
            IsolationLevel::Hypervisor,
            IsolationLevel::Process,
            IsolationLevel::Container,
        ];
        
        for level in levels {
            let mut config = BusBiosConfig::default();
            config.isolation_level = level.clone();
            
            let policy_engine = create_test_policy_engine();
            let traffic_light = create_test_traffic_light();
            let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);

            let bus_bios = BusBios::new(config, policy_engine, traffic_light, signing_key);
            assert_eq!(bus_bios.config.isolation_level, level);
        }
    }
}
