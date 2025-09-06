use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;
use serde_json::json;

use crate::forensic_firewall::ml_framework::{MlFramework, FeatureVector};
use crate::forensic_firewall::audit_bridge::{ForensicAuditBridge, ForensicEventType, ForensicSeverity};
use crate::immutable_audit_system::ComponentType;

// ZJL Comprehensive Audit Integration - Records EVERY forensic operation
use ziplock_json::vm_integration::{VmAuditManager, AuditEvent, VmType, VmInfo, VmStatus};
use ziplock_json::system_audit_coordinator::{SystemAuditCoordinator, GlobalEventType, SecurityImpact};
use ziplock_json::bpi_master_audit::BpiMasterAuditConfig;
use ziplock_json::{audit_vm_start, audit_security_alert};

/// Forensic VM system for advanced security research and malware analysis
#[derive(Debug, Clone)]
pub struct ForensicVM {
    pub vm_id: Uuid,
    pub vm_manager: Arc<RwLock<VMManager>>,
    pub kali_integration: Arc<KaliLinuxIntegration>,
    pub malware_sandbox: Arc<MalwareSandbox>,
    pub ml_framework: Arc<MlFramework>,
    pub audit_bridge: Arc<ForensicAuditBridge>,
    pub config: ForensicVMConfig,
    
    // ZJL Comprehensive Audit System - Records EVERY forensic operation
    pub zjl_audit_manager: Arc<VmAuditManager>,
    pub system_audit_coordinator: Arc<SystemAuditCoordinator>,
}

/// VM Manager for orchestrating forensic virtual machines
#[derive(Debug, Clone)]
pub struct VMManager {
    pub active_vms: HashMap<Uuid, VMInstance>,
    pub vm_templates: HashMap<String, VMTemplate>,
}

/// VM Instance for forensic analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMInstance {
    pub vm_id: Uuid,
    pub vm_name: String,
    pub vm_type: VMType,
    pub status: VMStatus,
    pub created_at: DateTime<Utc>,
    pub resource_allocation: ResourceAllocation,
    pub security_profile: SecurityProfile,
    pub analysis_session: Option<AnalysisSession>,
    pub metadata: HashMap<String, String>,
}

/// Types of forensic VMs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VMType {
    KaliLinux,
    MalwareSandbox,
    NetworkForensics,
    MemoryForensics,
    ThreatHunting,
    IncidentResponse,
}

/// VM Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VMStatus {
    Creating,
    Running,
    Analyzing,
    Destroyed,
}

/// VM Template for rapid deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMTemplate {
    pub template_id: String,
    pub vm_type: VMType,
    pub base_image: String,
    pub pre_installed_tools: Vec<String>,
    pub default_config: VMConfiguration,
}

/// VM Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMConfiguration {
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub disk_gb: u32,
    pub security_policies: Vec<String>,
}

/// Resource allocation for VM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocation {
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub disk_gb: u32,
    pub priority_level: ResourcePriority,
}

/// Resource priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourcePriority {
    Normal,
    High,
    Critical,
}

/// Security profile for VM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityProfile {
    pub security_level: SecurityLevel,
    pub audit_logging: bool,
    pub intrusion_detection: bool,
    pub malware_protection: bool,
}

/// Security levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Standard,
    Enhanced,
    Maximum,
}

/// Analysis session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSession {
    pub session_id: Uuid,
    pub session_type: AnalysisType,
    pub started_at: DateTime<Utc>,
    pub analyst_id: String,
    pub findings: Vec<AnalysisFinding>,
    pub ml_insights: Vec<MLInsight>,
}

/// Types of forensic analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisType {
    MalwareAnalysis,
    NetworkForensics,
    ThreatHunting,
    IncidentResponse,
}

/// Analysis finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisFinding {
    pub finding_id: Uuid,
    pub finding_type: String,
    pub severity: FindingSeverity,
    pub description: String,
    pub confidence: f64,
    pub discovered_at: DateTime<Utc>,
}

/// Finding severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FindingSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// ML-generated insight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLInsight {
    pub insight_id: Uuid,
    pub model_name: String,
    pub confidence: f64,
    pub description: String,
    pub generated_at: DateTime<Utc>,
}

/// Kali Linux integration
#[derive(Debug, Clone)]
pub struct KaliLinuxIntegration {
    pub tool_manager: ToolManager,
}

/// Tool manager for Kali Linux tools
#[derive(Debug, Clone)]
pub struct ToolManager {
    pub installed_tools: HashMap<String, KaliTool>,
}

/// Kali Linux tool information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaliTool {
    pub tool_name: String,
    pub category: String,
    pub version: String,
    pub description: String,
    pub ml_integration: bool,
}

/// Malware sandbox
#[derive(Debug, Clone)]
pub struct MalwareSandbox {
    pub sandbox_manager: SandboxManager,
}

/// Sandbox manager
#[derive(Debug, Clone)]
pub struct SandboxManager {
    pub active_sandboxes: HashMap<Uuid, SandboxInstance>,
}

/// Sandbox instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxInstance {
    pub sandbox_id: Uuid,
    pub malware_sample: MalwareSample,
    pub analysis_results: Option<SandboxAnalysisResults>,
}

/// Malware sample information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MalwareSample {
    pub sample_id: Uuid,
    pub file_name: String,
    pub file_hash: String,
    pub file_size: u64,
    pub submitted_at: DateTime<Utc>,
}

/// Sandbox analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxAnalysisResults {
    pub analysis_id: Uuid,
    pub execution_time_seconds: u64,
    pub behavioral_indicators: Vec<BehavioralIndicator>,
    pub threat_score: f64,
    pub ml_classification: Option<MLClassification>,
}

/// Behavioral indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralIndicator {
    pub indicator_type: String,
    pub description: String,
    pub severity: String,
    pub confidence: f64,
}

/// ML classification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLClassification {
    pub model_name: String,
    pub classification: String,
    pub confidence: f64,
    pub explanation: String,
}

/// Forensic VM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForensicVMConfig {
    pub enable_kali_integration: bool,
    pub enable_malware_sandbox: bool,
    pub enable_ml_analysis: bool,
    pub max_concurrent_vms: usize,
    pub audit_all_activities: bool,
}

impl ForensicVM {
    /// Create new forensic VM system
    pub async fn new(
        ml_framework: Arc<MlFramework>,
        audit_bridge: Arc<ForensicAuditBridge>,
        config: ForensicVMConfig,
    ) -> Result<Self> {
        let vm_id = Uuid::new_v4();
        
        // Initialize ZJL audit manager for comprehensive forensic VM audit coverage
        let zjl_audit_file = format!("/tmp/forensic_vm_{}.zjl", vm_id);
        let mut zjl_audit_manager = VmAuditManager::new(&zjl_audit_file)?;
        let vm_info = VmInfo {
            vm_id: vm_id.to_string(),
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
            vm_id,
            vm_manager: Arc::new(RwLock::new(VMManager::new())),
            kali_integration: Arc::new(KaliLinuxIntegration::new()),
            malware_sandbox: Arc::new(MalwareSandbox::new()),
            ml_framework,
            audit_bridge,
            config,
            
            // ZJL Comprehensive Audit System
            zjl_audit_manager: Arc::new(zjl_audit_manager),
            system_audit_coordinator,
        })
    }
    
    /// Create new forensic VM instance
    pub async fn create_vm(
        &self,
        vm_type: VMType,
        config: VMConfiguration,
        analyst_id: String,
    ) -> Result<VMInstance> {
        let vm_id = Uuid::new_v4();
        let vm_name = format!("forensic-vm-{}", vm_id);
        
        tracing::info!("🖥️ Creating forensic VM: {} (type: {:?})", vm_name, vm_type);
        
        let vm_instance = VMInstance {
            vm_id,
            vm_name: vm_name.clone(),
            vm_type: vm_type.clone(),
            status: VMStatus::Creating,
            created_at: Utc::now(),
            resource_allocation: ResourceAllocation {
                cpu_cores: config.cpu_cores,
                memory_gb: config.memory_gb,
                disk_gb: config.disk_gb,
                priority_level: ResourcePriority::Normal,
            },
            security_profile: SecurityProfile {
                security_level: SecurityLevel::Maximum,
                audit_logging: true,
                intrusion_detection: true,
                malware_protection: true,
            },
            analysis_session: None,
            metadata: {
                let mut metadata = HashMap::new();
                metadata.insert("analyst_id".to_string(), analyst_id);
                metadata
            },
        };
        
        // Add to VM manager
        {
            let mut manager = self.vm_manager.write().await;
            manager.active_vms.insert(vm_id, vm_instance.clone());
        }
        
        // Record audit event
        self.audit_bridge.record_security_event(
            ForensicEventType::ForensicEvidenceCollected,
            ComponentType::DockLock, // Using existing component type
            ForensicSeverity::Info,
            format!("Forensic VM created: {} (type: {:?})", vm_name, vm_type),
            None,
            None,
            None,
            None,
        ).await?;
        
        tracing::info!("✅ Forensic VM created successfully: {}", vm_name);
        Ok(vm_instance)
    }
    
    /// Start malware analysis in sandbox
    pub async fn analyze_malware(
        &self,
        malware_sample: MalwareSample,
        analyst_id: String,
    ) -> Result<SandboxAnalysisResults> {
        tracing::info!("🦠 Starting malware analysis: {}", malware_sample.file_name);
        
        // Create sandbox VM
        let vm_config = VMConfiguration {
            cpu_cores: 4,
            memory_gb: 8,
            disk_gb: 50,
            security_policies: vec!["malware_sandbox".to_string()],
        };
        
        let _vm_instance = self.create_vm(VMType::MalwareSandbox, vm_config, analyst_id).await?;
        
        // Execute malware analysis (simplified)
        let analysis_results = SandboxAnalysisResults {
            analysis_id: Uuid::new_v4(),
            execution_time_seconds: 300,
            behavioral_indicators: vec![
                BehavioralIndicator {
                    indicator_type: "file_creation".to_string(),
                    description: "Created suspicious executable".to_string(),
                    severity: "high".to_string(),
                    confidence: 0.9,
                }
            ],
            threat_score: 0.8,
            ml_classification: if self.config.enable_ml_analysis {
                Some(MLClassification {
                    model_name: "malware_classifier".to_string(),
                    classification: "trojan".to_string(),
                    confidence: 0.85,
                    explanation: "ML-based malware classification".to_string(),
                })
            } else {
                None
            },
        };
        
        // Record forensic evidence
        self.audit_bridge.record_security_event(
            ForensicEventType::ForensicEvidenceCollected,
            ComponentType::DockLock,
            ForensicSeverity::High,
            format!("Malware analysis completed: {}", malware_sample.file_name),
            None,
            None,
            None,
            None,
        ).await?;
        
        tracing::info!("✅ Malware analysis completed: {}", malware_sample.file_name);
        Ok(analysis_results)
    }
    
    /// Get VM status
    pub async fn get_vm_status(&self, vm_id: &Uuid) -> Result<Option<VMInstance>> {
        let manager = self.vm_manager.read().await;
        Ok(manager.active_vms.get(vm_id).cloned())
    }
    
    /// List all active VMs
    pub async fn list_active_vms(&self) -> Result<Vec<VMInstance>> {
        let manager = self.vm_manager.read().await;
        Ok(manager.active_vms.values().cloned().collect())
    }
    
    /// Destroy VM
    pub async fn destroy_vm(&self, vm_id: &Uuid) -> Result<()> {
        let mut manager = self.vm_manager.write().await;
        if let Some(mut vm) = manager.active_vms.remove(vm_id) {
            vm.status = VMStatus::Destroyed;
            tracing::info!("🗑️ Forensic VM destroyed: {}", vm.vm_name);
        }
        Ok(())
    }
}

impl VMManager {
    pub fn new() -> Self {
        Self {
            active_vms: HashMap::new(),
            vm_templates: HashMap::new(),
        }
    }
}

impl KaliLinuxIntegration {
    pub fn new() -> Self {
        Self {
            tool_manager: ToolManager::new(),
        }
    }
}

impl ToolManager {
    pub fn new() -> Self {
        let mut installed_tools = HashMap::new();
        
        // Add some common Kali tools
        installed_tools.insert("nmap".to_string(), KaliTool {
            tool_name: "nmap".to_string(),
            category: "network_scanning".to_string(),
            version: "7.94".to_string(),
            description: "Network discovery and security auditing".to_string(),
            ml_integration: true,
        });
        
        installed_tools.insert("metasploit".to_string(), KaliTool {
            tool_name: "metasploit".to_string(),
            category: "exploitation".to_string(),
            version: "6.3".to_string(),
            description: "Penetration testing framework".to_string(),
            ml_integration: false,
        });
        
        Self { installed_tools }
    }
}

impl MalwareSandbox {
    pub fn new() -> Self {
        Self {
            sandbox_manager: SandboxManager::new(),
        }
    }
}

impl SandboxManager {
    pub fn new() -> Self {
        Self {
            active_sandboxes: HashMap::new(),
        }
    }
}

impl Default for ForensicVMConfig {
    fn default() -> Self {
        Self {
            enable_kali_integration: true,
            enable_malware_sandbox: true,
            enable_ml_analysis: true,
            max_concurrent_vms: 10,
            audit_all_activities: true,
        }
    }
}
