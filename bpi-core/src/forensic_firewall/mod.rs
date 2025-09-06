// Forensic Firewall Module - Programmable CUE-based security with ML/AI integration
pub mod cue_engine;
pub mod threat_intel;
pub mod audit_bridge;
pub mod behavioral_analysis;
pub mod ml_framework;
pub mod dynamic_response;
pub mod firewall_integration;
pub mod forensic_vm;
pub mod enhanced_dynamic_firewall;
pub mod forensic_oracle;
pub mod kali_forensic_bridge;

// Re-export main components
pub use cue_engine::{CueRuleEngine, SecurityDecision, SecurityAction};
pub use threat_intel::{ThreatIntelligence, ThreatClassification, ThreatLevel};
pub use audit_bridge::{ForensicAuditBridge, ForensicEvent, ForensicEvidence};
pub use behavioral_analysis::{BehavioralAnalyzer, BehavioralAnalysisResult, DetectedAnomaly};
pub use ml_framework::{MlFramework, MlModel, FeatureVector, MlPrediction};
pub use dynamic_response::{DynamicThreatResponse, ActiveResponse, ThreatContext};
pub use firewall_integration::{ForensicFirewall, ForensicFirewallConfig, ProcessingResult};
pub use forensic_vm::{ForensicVM, VMInstance, MalwareSample, SandboxAnalysisResults};
