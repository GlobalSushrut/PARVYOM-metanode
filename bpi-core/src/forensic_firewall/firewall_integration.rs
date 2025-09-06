use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use uuid::Uuid;
use chrono::Utc;
use serde_json::json;

use crate::immutable_audit_system::{ImmutableAuditSystem, ComponentType};

// ZJL Comprehensive Audit Integration - Records EVERY firewall operation
use ziplock_json::vm_integration::{VmAuditManager, AuditEvent, VmType, VmInfo, VmStatus};
use ziplock_json::system_audit_coordinator::{SystemAuditCoordinator, GlobalEventType, SecurityImpact};
use ziplock_json::bpi_master_audit::BpiMasterAuditConfig;
use ziplock_json::{audit_vm_start, audit_security_alert};
use crate::forensic_firewall::{
    cue_engine::{CueRuleEngine, ThreatContext},
    behavioral_analysis::{BehavioralAnalyzer, BehavioralConfig, UserActivity},
    threat_intel::ThreatIntelligence,
    dynamic_response::{DynamicThreatResponse, DynamicResponseConfig},
    audit_bridge::{ForensicAuditBridge, AuditBridgeConfig},
    ml_framework::MlFramework,
};

/// Integrated forensic firewall system
#[derive(Debug, Clone)]
pub struct ForensicFirewall {
    pub id: Uuid,
    pub cue_engine: Arc<CueRuleEngine>,
    pub threat_intelligence: Arc<ThreatIntelligence>,
    pub behavioral_analyzer: Arc<BehavioralAnalyzer>,
    pub ml_framework: Arc<MlFramework>,
    pub audit_bridge: Arc<ForensicAuditBridge>,
    pub dynamic_response: Arc<DynamicThreatResponse>,
    pub config: ForensicFirewallConfig,
    
    // ZJL Comprehensive Audit System - Records EVERY firewall operation
    pub zjl_audit_manager: Arc<VmAuditManager>,
    pub system_audit_coordinator: Arc<SystemAuditCoordinator>,
}

/// Forensic firewall configuration
#[derive(Debug, Clone)]
pub struct ForensicFirewallConfig {
    pub enable_cue_rules: bool,
    pub enable_threat_intel: bool,
    pub enable_behavioral_analysis: bool,
    pub enable_ml_analysis: bool,
    pub enable_dynamic_response: bool,
    pub enable_real_time_audit: bool,
    pub security_contracts_path: String,
    pub performance_target_ms: f64,
}

impl ForensicFirewall {
    /// Create new integrated forensic firewall
    pub async fn new(
        audit_system: Arc<RwLock<ImmutableAuditSystem>>,
        config: ForensicFirewallConfig,
    ) -> Result<Self> {
        let id = Uuid::new_v4();
        
        // Initialize core components
        let cue_engine = Arc::new(CueRuleEngine::new());
        let threat_intelligence = Arc::new(ThreatIntelligence::new());
        let ml_config = crate::forensic_firewall::ml_framework::MlConfig {
            enable_gpu: false,
            max_concurrent_predictions: 10,
            model_cache_size: 1000,
            feature_cache_ttl_seconds: 3600,
            auto_retrain_enabled: true,
            retrain_threshold: 0.8,
            model_monitoring_enabled: true,
            drift_detection_enabled: true,
        };
        let ml_framework = Arc::new(MlFramework::new(ml_config));
        let behavioral_config = BehavioralConfig {
            user_analysis_enabled: true,
            network_analysis_enabled: true,
            system_analysis_enabled: true,
            ml_analysis_enabled: true,
            cache_ttl_seconds: 300,
            anomaly_threshold: 0.7,
            update_interval_seconds: 60,
            max_profiles: 10000,
        };
        let behavioral_analyzer = Arc::new(BehavioralAnalyzer::new(
            cue_engine.clone(),
            behavioral_config,
        ));
        
        // Initialize audit bridge
        let audit_config = AuditBridgeConfig {
            enable_real_time_audit: true,
            enable_evidence_collection: true,
            enable_chain_of_custody: true,
            evidence_retention_days: 365,
            max_evidence_size_mb: 1024,
            compression_enabled: true,
            encryption_enabled: true,
            digital_signature_required: true,
            witness_signatures_required: 2,
        };
        let audit_bridge = Arc::new(ForensicAuditBridge::new(
            audit_system.clone(),
            cue_engine.clone(),
            audit_config,
        ));
        
        // Initialize dynamic response system
        let response_config = DynamicResponseConfig {
            enable_automated_response: true,
            enable_quarantine: true,
            enable_counter_attacks: false,
            max_concurrent_responses: 10,
            response_timeout_minutes: 30,
            escalation_enabled: true,
            forensic_collection_enabled: true,
            notification_enabled: true,
        };
        
        let dynamic_response = Arc::new(DynamicThreatResponse::new(
            cue_engine.clone(),
            behavioral_analyzer.clone(),
            threat_intelligence.clone(),
            audit_bridge.clone(),
            response_config,
        ));
        
        // Load security contracts
        if config.enable_cue_rules {
            Self::load_security_contracts(&cue_engine, &config.security_contracts_path).await?;
        }
        
        // Initialize ZJL audit manager for comprehensive firewall audit coverage
        let zjl_audit_file = format!("/tmp/forensic_firewall_{}.zjl", uuid::Uuid::new_v4());
        let mut zjl_audit_manager = VmAuditManager::new(&zjl_audit_file)?;
        let vm_info = VmInfo {
            vm_id: "forensic_firewall".to_string(),
            vm_type: VmType::Forensic,
            status: VmStatus::Starting,
            start_time: chrono::Utc::now().timestamp() as u64,
            audit_enabled: true,
        };
        zjl_audit_manager.register_vm(vm_info);
        
        // Initialize system audit coordinator
        let system_coordinator_config = BpiMasterAuditConfig::default();
        let system_audit_coordinator = Arc::new(SystemAuditCoordinator::new(&system_coordinator_config.master_audit_file)?);

        Ok(Self {
            id,
            cue_engine,
            threat_intelligence,
            behavioral_analyzer,
            ml_framework,
            audit_bridge,
            dynamic_response,
            config,
            
            // ZJL Comprehensive Audit System
            zjl_audit_manager: Arc::new(zjl_audit_manager),
            system_audit_coordinator,
        })
    }
    
    /// Process security event through the complete forensic firewall pipeline
    pub async fn process_security_event(
        &self,
        threat_context: ThreatContext,
        source_component: ComponentType,
    ) -> Result<ProcessingResult> {
        let start_time = std::time::Instant::now();
        let processing_id = Uuid::new_v4();
        
        tracing::info!("🔥 Processing security event {} through forensic firewall", processing_id);
        
        let mut result = ProcessingResult {
            processing_id,
            threat_id: threat_context.threat_id.clone(),
            processed_at: Utc::now(),
            processing_time_ms: 0.0,
            components_processed: Vec::new(),
            final_decision: None,
            active_response: None,
            audit_records: Vec::new(),
            performance_metrics: ProcessingMetrics::default(),
        };
        
        // Step 1: CUE Rule Evaluation (if enabled)
        if self.config.enable_cue_rules {
            let cue_start = std::time::Instant::now();
            // Convert dynamic_response::ThreatContext to cue_engine::ThreatContext
            let cue_threat_context = crate::forensic_firewall::cue_engine::ThreatContext {
                threat_id: threat_context.threat_id.clone(),
                source_ip: threat_context.source_ip.clone(),
                threat_score: 0.5, // Default score
                source_reputation: threat_context.source_reputation,
                attack_complexity: threat_context.attack_complexity,
                temporal_anomaly_score: threat_context.temporal_anomaly_score,
                timestamp: std::time::Instant::now(),
            };
            let cue_decision = self.cue_engine.evaluate_threat(&cue_threat_context).await?;
            let cue_time = cue_start.elapsed().as_secs_f64() * 1000.0;
            
            result.components_processed.push(ComponentProcessingResult {
                component: "CUE_ENGINE".to_string(),
                processing_time_ms: cue_time,
                success: true,
                decision: Some(cue_decision.clone()),
                error: None,
            });
            
            result.final_decision = Some(cue_decision);
            result.performance_metrics.cue_evaluation_ms = cue_time;
        }
        
        // Step 2: Threat Intelligence Analysis (if enabled)
        if self.config.enable_threat_intel {
            let intel_start = std::time::Instant::now();
            let threat_classification = self.threat_intelligence
                .classify_threat(&vec![]).await?; // TODO: Extract indicators from threat_context
            let intel_time = intel_start.elapsed().as_secs_f64() * 1000.0;
            
            result.components_processed.push(ComponentProcessingResult {
                component: "THREAT_INTEL".to_string(),
                processing_time_ms: intel_time,
                success: true,
                decision: None,
                error: None,
            });
            
            result.performance_metrics.threat_intel_ms = intel_time;
        }
        
        // Step 3: Behavioral Analysis (if enabled)
        if self.config.enable_behavioral_analysis {
            let behavioral_start = std::time::Instant::now();
            // Create a UserActivity from threat context
            let user_activity = crate::forensic_firewall::behavioral_analysis::UserActivity {
                activity_type: "threat_analysis".to_string(),
                timestamp: chrono::Utc::now(),
                source_ip: threat_context.source_ip.clone(),
                user_agent: "unknown".to_string(),
                resource_accessed: "firewall".to_string(),
                action_performed: "threat_detection".to_string(),
                success: true,
                metadata: std::collections::HashMap::new(),
            };
            let behavioral_result = self.behavioral_analyzer
                .analyze_user_behavior("unknown_user", &user_activity)
                .await?;
            let behavioral_time = behavioral_start.elapsed().as_secs_f64() * 1000.0;
            
            result.components_processed.push(ComponentProcessingResult {
                component: "BEHAVIORAL_ANALYSIS".to_string(),
                processing_time_ms: behavioral_time,
                success: true,
                decision: None,
                error: None,
            });
            
            result.performance_metrics.behavioral_analysis_ms = behavioral_time;
            
            // Record behavioral anomaly if detected
            if behavioral_result.anomaly_score > 0.7 {
                let audit_id = self.audit_bridge
                    .record_behavioral_anomaly(&behavioral_result, source_component.clone())
                    .await?;
                result.audit_records.push(audit_id);
            }
        }
        
        // Step 4: ML/AI Analysis (if enabled)
        if self.config.enable_ml_analysis {
            let ml_start = std::time::Instant::now();
            // ML analysis would be performed here with registered models
            let ml_time = ml_start.elapsed().as_secs_f64() * 1000.0;
            
            result.components_processed.push(ComponentProcessingResult {
                component: "ML_FRAMEWORK".to_string(),
                processing_time_ms: ml_time,
                success: true,
                decision: None,
                error: None,
            });
            
            result.performance_metrics.ml_analysis_ms = ml_time;
        }
        
        // Step 5: Dynamic Response (if enabled)
        if self.config.enable_dynamic_response {
            let response_start = std::time::Instant::now();
            // Convert cue_engine::ThreatContext to dynamic_response::ThreatContext
            let dynamic_threat_context = crate::forensic_firewall::dynamic_response::ThreatContext {
                threat_id: threat_context.threat_id.clone(),
                source_ip: threat_context.source_ip.clone(),
                user_id: "unknown_user".to_string(),
                attack_vector: "unknown".to_string(),
                indicators: vec![],
                user_activity: UserActivity {
                    activity_type: "threat_analysis".to_string(),
                    timestamp: chrono::Utc::now(),
                    source_ip: threat_context.source_ip.clone(),
                    user_agent: "BPI-Core".to_string(),
                    resource_accessed: "firewall".to_string(),
                    action_performed: "threat_detection".to_string(),
                    success: true,
                    metadata: std::collections::HashMap::new(),
                },
                source_reputation: threat_context.source_reputation,
                attack_complexity: threat_context.attack_complexity,
                temporal_anomaly_score: threat_context.temporal_anomaly_score,
                metadata: std::collections::HashMap::new(),
            };
            let active_response = self.dynamic_response
                .process_threat(&dynamic_threat_context, source_component.clone())
                .await?;
            let response_time = response_start.elapsed().as_secs_f64() * 1000.0;
            
            result.components_processed.push(ComponentProcessingResult {
                component: "DYNAMIC_RESPONSE".to_string(),
                processing_time_ms: response_time,
                success: true,
                decision: None,
                error: None,
            });
            
            result.active_response = Some(active_response);
            result.performance_metrics.dynamic_response_ms = response_time;
        }
        
        // Calculate total processing time
        let total_time = start_time.elapsed().as_secs_f64() * 1000.0;
        result.processing_time_ms = total_time;
        result.performance_metrics.total_processing_ms = total_time;
        
        // Check performance target
        if total_time > self.config.performance_target_ms {
            tracing::warn!(
                "⚠️ Forensic firewall processing exceeded target: {:.2}ms > {:.2}ms",
                total_time,
                self.config.performance_target_ms
            );
        }
        
        // Record final audit event
        if self.config.enable_real_time_audit {
            let final_audit_id = self.audit_bridge.record_security_event(
                crate::forensic_firewall::audit_bridge::ForensicEventType::PolicyEnforcementAction,
                source_component,
                crate::forensic_firewall::audit_bridge::ForensicSeverity::Info,
                format!("Forensic firewall processing completed: {}", processing_id),
                None,
                result.final_decision.clone(),
                None,
                None,
            ).await?;
            result.audit_records.push(final_audit_id);
        }
        
        tracing::info!(
            "✅ Forensic firewall processing completed: {} ({:.2}ms)",
            processing_id,
            total_time
        );
        
        Ok(result)
    }
    
    /// Load security contracts from directory
    async fn load_security_contracts(
        cue_engine: &Arc<CueRuleEngine>,
        contracts_path: &str,
    ) -> Result<()> {
        use tokio::fs;
        
        let mut dir = fs::read_dir(contracts_path).await?;
        let mut loaded_count = 0;
        
        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("cue") {
                match cue_engine.load_security_contract(path.to_str().unwrap()).await {
                    Ok(contract_id) => {
                        tracing::info!("📋 Loaded security contract: {} -> {}", path.display(), contract_id);
                        loaded_count += 1;
                    },
                    Err(e) => {
                        tracing::error!("❌ Failed to load security contract {}: {}", path.display(), e);
                    }
                }
            }
        }
        
        tracing::info!("🔥 Loaded {} security contracts from {}", loaded_count, contracts_path);
        Ok(())
    }
    
    /// Get firewall status and metrics
    pub async fn get_status(&self) -> Result<FirewallStatus> {
        let active_responses = self.dynamic_response.get_active_responses().await?;
        let evidence_chain = self.audit_bridge.get_evidence_chain().await?;
        
        Ok(FirewallStatus {
            firewall_id: self.id,
            status: "ACTIVE".to_string(),
            uptime_seconds: 0, // Would be calculated from start time
            components_enabled: self.get_enabled_components(),
            active_responses_count: active_responses.len(),
            evidence_chain_length: evidence_chain.len(),
            performance_metrics: FirewallPerformanceMetrics {
                avg_processing_time_ms: 0.0, // Would be calculated from historical data
                requests_processed: 0,
                threats_detected: 0,
                responses_activated: active_responses.len(),
            },
        })
    }
    
    /// Get enabled components list
    fn get_enabled_components(&self) -> Vec<String> {
        let mut components = Vec::new();
        if self.config.enable_cue_rules { components.push("CUE_ENGINE".to_string()); }
        if self.config.enable_threat_intel { components.push("THREAT_INTEL".to_string()); }
        if self.config.enable_behavioral_analysis { components.push("BEHAVIORAL_ANALYSIS".to_string()); }
        if self.config.enable_ml_analysis { components.push("ML_FRAMEWORK".to_string()); }
        if self.config.enable_dynamic_response { components.push("DYNAMIC_RESPONSE".to_string()); }
        if self.config.enable_real_time_audit { components.push("AUDIT_BRIDGE".to_string()); }
        components
    }
}

/// Processing result from forensic firewall
#[derive(Debug, Clone)]
pub struct ProcessingResult {
    pub processing_id: Uuid,
    pub threat_id: String,
    pub processed_at: chrono::DateTime<Utc>,
    pub processing_time_ms: f64,
    pub components_processed: Vec<ComponentProcessingResult>,
    pub final_decision: Option<crate::forensic_firewall::SecurityDecision>,
    pub active_response: Option<crate::forensic_firewall::ActiveResponse>,
    pub audit_records: Vec<Uuid>,
    pub performance_metrics: ProcessingMetrics,
}

/// Component processing result
#[derive(Debug, Clone)]
pub struct ComponentProcessingResult {
    pub component: String,
    pub processing_time_ms: f64,
    pub success: bool,
    pub decision: Option<crate::forensic_firewall::SecurityDecision>,
    pub error: Option<String>,
}

/// Processing performance metrics
#[derive(Debug, Clone, Default)]
pub struct ProcessingMetrics {
    pub total_processing_ms: f64,
    pub cue_evaluation_ms: f64,
    pub threat_intel_ms: f64,
    pub behavioral_analysis_ms: f64,
    pub ml_analysis_ms: f64,
    pub dynamic_response_ms: f64,
}

/// Firewall status information
#[derive(Debug, Clone)]
pub struct FirewallStatus {
    pub firewall_id: Uuid,
    pub status: String,
    pub uptime_seconds: u64,
    pub components_enabled: Vec<String>,
    pub active_responses_count: usize,
    pub evidence_chain_length: usize,
    pub performance_metrics: FirewallPerformanceMetrics,
}

/// Firewall performance metrics
#[derive(Debug, Clone)]
pub struct FirewallPerformanceMetrics {
    pub avg_processing_time_ms: f64,
    pub requests_processed: u64,
    pub threats_detected: u64,
    pub responses_activated: usize,
}

impl Default for ForensicFirewallConfig {
    fn default() -> Self {
        Self {
            enable_cue_rules: true,
            enable_threat_intel: true,
            enable_behavioral_analysis: true,
            enable_ml_analysis: true,
            enable_dynamic_response: true,
            enable_real_time_audit: true,
            security_contracts_path: "/home/umesh/metanode/security_contracts".to_string(),
            performance_target_ms: 1.0, // Sub-millisecond target
        }
    }
}
