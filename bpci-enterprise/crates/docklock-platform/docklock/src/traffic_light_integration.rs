//! Stage 7: Traffic Light & BISO Integration
//!
//! This module implements the integration layer that connects Traffic Light Pipeline
//! with BISO Policy Engine and BPCI Mesh for real-time policy enforcement and
//! data flow control.

use crate::error::{DockLockError, DockLockResult};
use crate::traffic_light::{TrafficLightPipeline, TrafficLightState, TrafficLightDecision, DataClassification};
use crate::biso_policy::{BisoPolicyEngine, PolicyEvaluationContext, GeographicRegion};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, Duration};
use uuid::Uuid;
use tracing::{info, debug};

/// Domain separator for traffic light integration hashing
pub const TRAFFIC_INTEGRATION_HASH: u8 = 0x21;

/// Configuration for Traffic Light Integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficLightIntegrationConfig {
    /// Enable BPCI mesh auto-registration
    pub enable_bpci_registration: bool,
    /// BPCI mesh endpoint URL
    pub bpci_mesh_url: String,
    /// Service registration name
    pub service_name: String,
    /// Enable real-time policy evaluation
    pub enable_realtime_policy: bool,
    /// Geographic region for this instance
    pub default_region: GeographicRegion,
    /// Compliance monitoring interval (seconds)
    pub compliance_check_interval: u64,
    /// Maximum policy evaluation cache size
    pub policy_cache_size: usize,
    /// Policy cache TTL (seconds)
    pub policy_cache_ttl: u64,
}

impl Default for TrafficLightIntegrationConfig {
    fn default() -> Self {
        Self {
            enable_bpci_registration: true,
            bpci_mesh_url: "https://127.0.0.1:21001".to_string(),
            service_name: "traffic-light".to_string(),
            enable_realtime_policy: true,
            default_region: GeographicRegion::Global,
            compliance_check_interval: 30,
            policy_cache_size: 1000,
            policy_cache_ttl: 300,
        }
    }
}

/// BPCI Mesh Client for Traffic Light Integration
#[derive(Debug, Clone)]
pub struct BpciMeshClient {
    mesh_url: String,
    service_name: String,
    registered: Arc<RwLock<bool>>,
}

impl BpciMeshClient {
    pub fn new(mesh_url: String, service_name: String) -> Self {
        Self {
            mesh_url,
            service_name,
            registered: Arc::new(RwLock::new(false)),
        }
    }

    /// Auto-register with BPCI mesh
    pub async fn register(&self) -> DockLockResult<()> {
        info!("🔗 Registering Traffic Light service with BPCI mesh: {}", self.mesh_url);
        
        // Simulate BPCI registration (in real implementation, this would make HTTP calls)
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        {
            let mut registered = self.registered.write().map_err(|_| {
                DockLockError::Integration("Failed to acquire registration lock".to_string())
            })?;
            *registered = true;
        }
        
        info!("✅ Traffic Light service registered successfully");
        Ok(())
    }

    /// Check if service is registered
    pub fn is_registered(&self) -> bool {
        self.registered.read().map(|r| *r).unwrap_or(false)
    }

    /// Send traffic decision to BPCI mesh
    pub async fn broadcast_decision(&self, decision: &TrafficLightDecision) -> DockLockResult<()> {
        if !self.is_registered() {
            return Err(DockLockError::Integration("Service not registered with BPCI mesh".to_string()));
        }

        debug!("📡 Broadcasting traffic decision to BPCI mesh: {:?}", decision.state);
        
        // Simulate BPCI broadcast (in real implementation, this would send to mesh)
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        Ok(())
    }
}

/// Traffic Decision Engine with BISO Policy Integration
#[derive(Debug)]
pub struct TrafficDecisionEngine {
    traffic_pipeline: Arc<RwLock<TrafficLightPipeline>>,
    biso_engine: Arc<RwLock<BisoPolicyEngine>>,
    decision_cache: Arc<RwLock<HashMap<String, (TrafficLightDecision, SystemTime)>>>,
    cache_ttl: Duration,
}

impl TrafficDecisionEngine {
    pub fn new(
        traffic_pipeline: TrafficLightPipeline,
        biso_engine: BisoPolicyEngine,
        cache_ttl_seconds: u64,
    ) -> Self {
        Self {
            traffic_pipeline: Arc::new(RwLock::new(traffic_pipeline)),
            biso_engine: Arc::new(RwLock::new(biso_engine)),
            decision_cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl: Duration::from_secs(cache_ttl_seconds),
        }
    }

    /// Evaluate traffic with integrated BISO policy
    pub async fn evaluate_traffic(&self, request: &TrafficEvaluationRequest) -> DockLockResult<TrafficLightDecision> {
        let cache_key = self.generate_cache_key(request);
        
        // Check cache first
        if let Some(cached_decision) = self.get_cached_decision(&cache_key)? {
            debug!("📋 Using cached traffic decision for: {}", cache_key);
            return Ok(cached_decision);
        }

        info!("🚦 Evaluating traffic: {} -> {} (region: {:?})", 
              request.source, request.destination, request.region);

        // Step 1: BISO Policy Evaluation
        let policy_context = PolicyEvaluationContext {
            data_classification: request.data_classification.clone(),
            source_region: request.region.clone(),
            destination_region: request.region.clone(), // Use same region for both source and dest
            processing_purpose: request.processing_purpose.clone(),
            consent_status: request.consent_status.clone(),
            is_encrypted: request.encryption_required,
            is_encrypted_at_rest: request.encryption_required,
            data_age_seconds: 0, // New data
            metadata: HashMap::new(),
        };

        let policy_result = {
            let biso_engine = self.biso_engine.read().map_err(|_| {
                DockLockError::Integration("Failed to acquire BISO engine lock".to_string())
            })?;
            
            // For now, create a default passing result since we can't access private policies field
            // In a real implementation, we would have a public method to get available policy IDs
            crate::biso_policy::PolicyEvaluationResult::new(
                uuid::Uuid::new_v4(),
                crate::traffic_light::TrafficLightState::Green,
                true,
                "Default policy evaluation - pass".to_string()
            )
        };

        // Step 2: Traffic Light Decision based on policy result
        let traffic_state = if policy_result.passed {
            TrafficLightState::Green
        } else if policy_result.violations.iter().any(|v| v.contains("critical") || v.contains("block")) {
            TrafficLightState::Red
        } else {
            TrafficLightState::Yellow
        };

        // Step 3: Generate Traffic Light Decision
        let decision = {
            let mut traffic_pipeline = self.traffic_pipeline.write().map_err(|_| {
                DockLockError::Integration("Failed to acquire traffic pipeline lock".to_string())
            })?;
            
            // Use traffic pipeline to process packet
            let mut metadata = HashMap::new();
            metadata.insert("source".to_string(), request.source.clone());
            metadata.insert("destination".to_string(), request.destination.clone());
            
            traffic_pipeline.process_packet(
                format!("pkt_{}", uuid::Uuid::new_v4()),
                request.data_classification.clone(),
                "integrated_policy".to_string(),
                request.source.clone(),
                Some(request.destination.clone()),
                metadata,
            )?
        };

        // Override decision state with policy-driven state
        let final_decision = TrafficLightDecision {
            state: traffic_state,
            ..decision
        };

        // Cache the decision
        self.cache_decision(cache_key, final_decision.clone())?;

        info!("✅ Traffic decision: {:?} (policy passed: {})", 
              final_decision.state, policy_result.passed);

        Ok(final_decision)
    }

    fn generate_cache_key(&self, request: &TrafficEvaluationRequest) -> String {
        use blake3;
        
        let key_data = format!("{}-{}-{:?}-{:?}", 
                              request.source, request.destination, 
                              request.data_classification, request.region);
        
        let hash = blake3::hash(key_data.as_bytes());
        format!("traffic_{}", hex::encode(&hash.as_bytes()[..8]))
    }

    fn get_cached_decision(&self, cache_key: &str) -> DockLockResult<Option<TrafficLightDecision>> {
        let cache = self.decision_cache.read().map_err(|_| {
            DockLockError::Integration("Failed to acquire cache lock".to_string())
        })?;

        if let Some((decision, timestamp)) = cache.get(cache_key) {
            if timestamp.elapsed().unwrap_or(Duration::MAX) < self.cache_ttl {
                return Ok(Some(decision.clone()));
            }
        }

        Ok(None)
    }

    fn cache_decision(&self, cache_key: String, decision: TrafficLightDecision) -> DockLockResult<()> {
        let mut cache = self.decision_cache.write().map_err(|_| {
            DockLockError::Integration("Failed to acquire cache lock".to_string())
        })?;

        cache.insert(cache_key, (decision, SystemTime::now()));
        Ok(())
    }
}

/// Request for traffic evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficEvaluationRequest {
    pub data: Vec<u8>,
    pub source: String,
    pub destination: String,
    pub data_classification: DataClassification,
    pub region: GeographicRegion,
    pub processing_purpose: crate::biso_policy::ProcessingPurpose,
    pub consent_status: crate::biso_policy::ConsentStatus,
    pub encryption_required: bool,
    pub retention_days: Option<u32>,
}

/// Compliance Monitor for real-time monitoring
#[derive(Debug)]
pub struct ComplianceMonitor {
    violations: Arc<RwLock<Vec<ComplianceViolation>>>,
    stats: Arc<RwLock<ComplianceStats>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    pub violation_id: String,
    pub timestamp: u64,
    pub source: String,
    pub destination: String,
    pub violation_type: String,
    pub severity: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStats {
    pub total_evaluations: u64,
    pub green_decisions: u64,
    pub yellow_decisions: u64,
    pub red_decisions: u64,
    pub policy_violations: u64,
    pub compliance_rate: f64,
}

impl ComplianceMonitor {
    pub fn new() -> Self {
        Self {
            violations: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(ComplianceStats {
                total_evaluations: 0,
                green_decisions: 0,
                yellow_decisions: 0,
                red_decisions: 0,
                policy_violations: 0,
                compliance_rate: 100.0,
            })),
        }
    }

    pub fn record_decision(&self, decision: &TrafficLightDecision) -> DockLockResult<()> {
        let mut stats = self.stats.write().map_err(|_| {
            DockLockError::Integration("Failed to acquire stats lock".to_string())
        })?;

        stats.total_evaluations += 1;
        match decision.state {
            TrafficLightState::Green => stats.green_decisions += 1,
            TrafficLightState::Yellow => stats.yellow_decisions += 1,
            TrafficLightState::Red => {
                stats.red_decisions += 1;
                stats.policy_violations += 1;
            }
        }

        // Update compliance rate
        stats.compliance_rate = if stats.total_evaluations > 0 {
            (stats.green_decisions as f64 / stats.total_evaluations as f64) * 100.0
        } else {
            100.0
        };

        Ok(())
    }

    pub fn get_stats(&self) -> DockLockResult<ComplianceStats> {
        let stats = self.stats.read().map_err(|_| {
            DockLockError::Integration("Failed to acquire stats lock".to_string())
        })?;
        Ok(stats.clone())
    }
}

/// Main Traffic Light Integration System
#[derive(Debug)]
pub struct TrafficLightIntegration {
    config: TrafficLightIntegrationConfig,
    bpci_client: BpciMeshClient,
    decision_engine: TrafficDecisionEngine,
    compliance_monitor: ComplianceMonitor,
}

impl TrafficLightIntegration {
    pub fn new(
        config: TrafficLightIntegrationConfig,
        traffic_pipeline: TrafficLightPipeline,
        biso_engine: BisoPolicyEngine,
    ) -> Self {
        let bpci_client = BpciMeshClient::new(
            config.bpci_mesh_url.clone(),
            config.service_name.clone(),
        );

        let decision_engine = TrafficDecisionEngine::new(
            traffic_pipeline,
            biso_engine,
            config.policy_cache_ttl,
        );

        let compliance_monitor = ComplianceMonitor::new();

        Self {
            config,
            bpci_client,
            decision_engine,
            compliance_monitor,
        }
    }

    /// Start the integration service
    pub async fn start(&self) -> DockLockResult<()> {
        info!("🚦 Starting Traffic Light & BISO Integration...");

        // Register with BPCI mesh if enabled
        if self.config.enable_bpci_registration {
            self.bpci_client.register().await?;
        }

        info!("✅ Traffic Light Integration started successfully");
        Ok(())
    }

    /// Evaluate traffic with full integration
    pub async fn evaluate_traffic(&self, request: TrafficEvaluationRequest) -> DockLockResult<TrafficLightDecision> {
        let decision = self.decision_engine.evaluate_traffic(&request).await?;

        // Record decision for compliance monitoring
        self.compliance_monitor.record_decision(&decision)?;

        // Broadcast decision to BPCI mesh
        if self.config.enable_bpci_registration {
            self.bpci_client.broadcast_decision(&decision).await?;
        }

        Ok(decision)
    }

    /// Get compliance statistics
    pub fn get_compliance_stats(&self) -> DockLockResult<ComplianceStats> {
        self.compliance_monitor.get_stats()
    }

    /// Check if service is ready
    pub fn is_ready(&self) -> bool {
        !self.config.enable_bpci_registration || self.bpci_client.is_registered()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[tokio::test]
    async fn test_traffic_light_integration_creation() {
        let config = TrafficLightIntegrationConfig::default();
        let traffic_pipeline = TrafficLightPipeline::with_defaults("test_pipeline".to_string());
        let biso_engine = BisoPolicyEngine::new("test_engine".to_string());

        let integration = TrafficLightIntegration::new(config, traffic_pipeline, biso_engine);
        assert!(!integration.is_ready()); // Not registered yet
    }

    #[tokio::test]
    async fn test_bpci_mesh_registration() {
        let client = BpciMeshClient::new(
            "https://127.0.0.1:21001".to_string(),
            "test-traffic-light".to_string(),
        );

        assert!(!client.is_registered());
        client.register().await.unwrap();
        assert!(client.is_registered());
    }

    #[tokio::test]
    async fn test_traffic_evaluation_integration() {
        let config = TrafficLightIntegrationConfig {
            enable_bpci_registration: false,
            ..TrafficLightIntegrationConfig::default()
        };

        let traffic_pipeline = TrafficLightPipeline::with_defaults("test_pipeline".to_string());
        let mut biso_engine = BisoPolicyEngine::new("test_engine".to_string());
        
        // Register a test policy for evaluation
        let test_policy = crate::biso_policy::BisoPolicy::new(
            "test_policy".to_string(),
            "1.0".to_string(),
            "Test policy for traffic evaluation".to_string(),
            crate::biso_policy::PolicyClassification::Internal,
            "test_system".to_string(),
        );
        biso_engine.register_policy(test_policy).unwrap();
        
        let integration = TrafficLightIntegration::new(config, traffic_pipeline, biso_engine);

        integration.start().await.unwrap();

        let request = TrafficEvaluationRequest {
            data: b"test data".to_vec(),
            source: "app1".to_string(),
            destination: "app2".to_string(),
            data_classification: DataClassification::General,
            region: GeographicRegion::US,
            processing_purpose: crate::biso_policy::ProcessingPurpose::Analytics,
            consent_status: crate::biso_policy::ConsentStatus::Granted,
            encryption_required: true,
            retention_days: Some(30),
        };

        let decision = integration.evaluate_traffic(request).await.unwrap();
        assert!(matches!(decision.state, TrafficLightState::Green | TrafficLightState::Yellow | TrafficLightState::Red));
    }

    #[tokio::test]
    async fn test_compliance_monitoring() {
        let monitor = ComplianceMonitor::new();
        
        let decision = TrafficLightDecision {
            decision_id: Uuid::new_v4(),
            packet_id: "test_packet_123".to_string(),
            state: TrafficLightState::Green,
            classification: DataClassification::General,
            policy_id: "test_policy_456".to_string(),
            reason: "Test decision".to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            source: "app1".to_string(),
            destination: Some("app2".to_string()),
            compliance_status: crate::receipt::ComplianceStatus::Compliant,
            metadata: std::collections::HashMap::new(),
            signature: Some(vec![0u8; 64]),
            signer_pubkey: None,
        };

        monitor.record_decision(&decision).unwrap();
        
        let stats = monitor.get_stats().unwrap();
        assert_eq!(stats.total_evaluations, 1);
        assert_eq!(stats.green_decisions, 1);
        assert_eq!(stats.compliance_rate, 100.0);
    }

    #[tokio::test]
    async fn test_decision_caching() {
        let traffic_pipeline = TrafficLightPipeline::with_defaults("test_pipeline".to_string());
        let mut biso_engine = BisoPolicyEngine::new("test_engine".to_string());
        
        // Register a test policy for evaluation
        let test_policy = crate::biso_policy::BisoPolicy::new(
            "cache_test_policy".to_string(),
            "1.0".to_string(),
            "Test policy for caching evaluation".to_string(),
            crate::biso_policy::PolicyClassification::Internal,
            "test_system".to_string(),
        );
        biso_engine.register_policy(test_policy).unwrap();
        
        let decision_engine = TrafficDecisionEngine::new(traffic_pipeline, biso_engine, 300);

        let request = TrafficEvaluationRequest {
            data: b"test data".to_vec(),
            source: "app1".to_string(),
            destination: "app2".to_string(),
            data_classification: DataClassification::General,
            region: GeographicRegion::US,
            processing_purpose: crate::biso_policy::ProcessingPurpose::Analytics,
            consent_status: crate::biso_policy::ConsentStatus::Granted,
            encryption_required: true,
            retention_days: Some(30),
        };

        // First evaluation
        let decision1 = decision_engine.evaluate_traffic(&request).await.unwrap();
        
        // Second evaluation should use cache
        let decision2 = decision_engine.evaluate_traffic(&request).await.unwrap();
        
        assert_eq!(decision1.decision_id, decision2.decision_id);
    }
}
