//! Export Engine - Forensic audit trail export and analysis
//!
//! Exports audit data for forensic analysis, compliance reporting, and incident investigation

use crate::{AuditTree, RuntimeAuditNode, ProofChain, ComplianceTag};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Export Engine for forensic audit trails
#[derive(Debug)]
pub struct ExportEngine {
    /// Export configuration
    config: ExportConfig,
    /// Export templates
    templates: HashMap<ExportFormat, ExportTemplate>,
    /// Storage reference for querying audit data
    storage: std::sync::Arc<crate::storage::AuditStorage>,
}

/// Export Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    /// Default export format
    pub default_format: ExportFormat,
    /// Include binary outputs in exports
    pub include_binary_outputs: bool,
    /// Include proof chains in exports
    pub include_proof_chains: bool,
    /// Maximum export size in bytes
    pub max_export_size_bytes: usize,
    /// Compression level (0-9)
    pub compression_level: u32,
    /// Export encryption settings
    pub encryption: Option<ExportEncryption>,
}

/// Export Format
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportFormat {
    /// JSON format for human readability
    Json,
    /// CBOR format for efficient binary serialization
    Cbor,
    /// MessagePack format for compact serialization
    MessagePack,
    /// CSV format for spreadsheet analysis
    Csv,
    /// XML format for enterprise systems
    Xml,
    /// PDF format for reports
    Pdf,
    /// Custom format
    Custom(String),
}

/// Export Template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTemplate {
    /// Template name
    pub name: String,
    /// Template format
    pub format: ExportFormat,
    /// Fields to include in export
    pub included_fields: Vec<String>,
    /// Custom formatting rules
    pub formatting_rules: HashMap<String, String>,
    /// Header template
    pub header_template: Option<String>,
    /// Footer template
    pub footer_template: Option<String>,
}

/// Export Encryption Settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportEncryption {
    /// Encryption algorithm
    pub algorithm: String,
    /// Key derivation function
    pub kdf: String,
    /// Salt for key derivation
    pub salt: Vec<u8>,
    /// Encryption key (should be securely managed)
    pub key: Option<Vec<u8>>,
}

/// Export Package - Complete forensic export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPackage {
    /// Package metadata
    pub metadata: ExportMetadata,
    /// Exported audit nodes
    pub audit_nodes: Vec<RuntimeAuditNode>,
    /// Proof chains
    pub proof_chains: Vec<ProofChain>,
    /// Export statistics
    pub statistics: ExportStatistics,
    /// Compliance information
    pub compliance: ComplianceInfo,
    /// Attack vector analysis
    pub attack_analysis: Option<AttackVectorAnalysis>,
}

/// Export Metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMetadata {
    /// Export ID
    pub export_id: Uuid,
    /// Export timestamp
    pub exported_at: DateTime<Utc>,
    /// Export format
    pub format: ExportFormat,
    /// Export requester
    pub requester: String,
    /// Export purpose
    pub purpose: ExportPurpose,
    /// Time range of exported data
    pub time_range: TimeRange,
    /// Export version
    pub version: String,
    /// Digital signature
    pub signature: Option<Vec<u8>>,
}

/// Export Purpose
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportPurpose {
    /// Forensic investigation
    ForensicInvestigation,
    /// Compliance audit
    ComplianceAudit,
    /// Security incident response
    IncidentResponse,
    /// Legal discovery
    LegalDiscovery,
    /// Performance analysis
    PerformanceAnalysis,
    /// Backup and archival
    BackupArchival,
    /// Custom purpose
    Custom(String),
}

/// Time Range for export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    /// Start time (inclusive)
    pub start: DateTime<Utc>,
    /// End time (inclusive)
    pub end: DateTime<Utc>,
}

/// Export Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportStatistics {
    /// Total audit nodes exported
    pub total_nodes: usize,
    /// Nodes by runtime type
    pub nodes_by_runtime: HashMap<String, usize>,
    /// Nodes by audit level
    pub nodes_by_level: HashMap<String, usize>,
    /// Total binary data size
    pub total_binary_size_bytes: usize,
    /// Export file size
    pub export_size_bytes: usize,
    /// Compression ratio
    pub compression_ratio: f64,
}

/// Compliance Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceInfo {
    /// Compliance frameworks addressed
    pub frameworks: Vec<String>,
    /// Compliance tags found
    pub tags: Vec<ComplianceTag>,
    /// Compliance violations detected
    pub violations: Vec<ComplianceViolation>,
    /// Retention requirements
    pub retention_years: u32,
}

/// Compliance Violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    /// Violation ID
    pub violation_id: Uuid,
    /// Violation type
    pub violation_type: String,
    /// Severity level
    pub severity: ViolationSeverity,
    /// Description
    pub description: String,
    /// Affected audit nodes
    pub affected_nodes: Vec<Uuid>,
    /// Remediation suggestions
    pub remediation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Attack Vector Analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackVectorAnalysis {
    /// Analysis ID
    pub analysis_id: Uuid,
    /// Detected attack patterns
    pub attack_patterns: Vec<AttackPattern>,
    /// Attack timeline
    pub timeline: Vec<AttackEvent>,
    /// Affected systems
    pub affected_systems: Vec<String>,
    /// Attack severity
    pub severity: AttackSeverity,
    /// Indicators of compromise
    pub iocs: Vec<IndicatorOfCompromise>,
}

/// Attack Pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackPattern {
    /// Pattern ID
    pub pattern_id: String,
    /// Pattern name
    pub name: String,
    /// MITRE ATT&CK technique ID
    pub mitre_technique: Option<String>,
    /// Confidence score (0-100)
    pub confidence: u32,
    /// Evidence nodes
    pub evidence_nodes: Vec<Uuid>,
}

/// Attack Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackEvent {
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event type
    pub event_type: String,
    /// Event description
    pub description: String,
    /// Related audit node
    pub audit_node_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttackSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Indicator of Compromise
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorOfCompromise {
    /// IOC type (IP, domain, hash, etc.)
    pub ioc_type: String,
    /// IOC value
    pub value: String,
    /// Confidence level
    pub confidence: u32,
    /// First seen timestamp
    pub first_seen: DateTime<Utc>,
    /// Last seen timestamp
    pub last_seen: DateTime<Utc>,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            default_format: ExportFormat::Json,
            include_binary_outputs: true,
            include_proof_chains: true,
            max_export_size_bytes: 100 * 1024 * 1024, // 100MB
            compression_level: 6,
            encryption: None,
        }
    }
}

impl ExportEngine {
    /// Create a new export engine
    pub fn new(config: ExportConfig, storage: std::sync::Arc<crate::storage::AuditStorage>) -> Self {
        let mut templates = HashMap::new();
        
        // Add default templates
        templates.insert(
            ExportFormat::Json,
            ExportTemplate {
                name: "Default JSON".to_string(),
                format: ExportFormat::Json,
                included_fields: vec![
                    "node_id".to_string(),
                    "timestamp_ns".to_string(),
                    "operation".to_string(),
                    "operation_details".to_string(),
                    "execution_context".to_string(),
                    "audit_level".to_string(),
                    "compliance_tags".to_string(),
                ],
                formatting_rules: HashMap::new(),
                header_template: None,
                footer_template: None,
            },
        );
        
        templates.insert(
            ExportFormat::Csv,
            ExportTemplate {
                name: "Default CSV".to_string(),
                format: ExportFormat::Csv,
                included_fields: vec![
                    "node_id".to_string(),
                    "timestamp_ns".to_string(),
                    "runtime_type".to_string(),
                    "operation".to_string(),
                    "operation_details".to_string(),
                    "audit_level".to_string(),
                ],
                formatting_rules: HashMap::new(),
                header_template: Some("Audit Trail Export".to_string()),
                footer_template: Some("End of Export".to_string()),
            },
        );
        
        Self { config, templates, storage }
    }
    
    /// Export audit tree to forensic package
    pub async fn export_forensic_package(
        &self,
        audit_tree: &AuditTree,
        purpose: ExportPurpose,
        requester: String,
        time_range: Option<TimeRange>,
    ) -> Result<ExportPackage> {
        println!("🔍 Generating forensic export package");
        println!("   Purpose: {:?}", purpose);
        println!("   Requester: {}", requester);
        
        // Filter nodes by time range if specified
        let nodes = if let Some(range) = &time_range {
            // Get nodes from storage with time range filter
            self.storage.query_nodes(crate::storage::StorageQuery {
                time_range: Some((range.start, range.end)),
                ..Default::default()
            }).await?
        } else {
            // Get all nodes from storage
            self.storage.query_nodes(crate::storage::StorageQuery::default()).await?
        };
        
        // Extract proof chains
        let proof_chains: Vec<ProofChain> = nodes.iter()
            .map(|node| node.proof_chain.clone())
            .collect();
        
        // Generate statistics
        let statistics = self.generate_export_statistics(&nodes);
        
        // Generate compliance information
        let compliance = self.generate_compliance_info(&nodes);
        
        // Perform attack vector analysis
        let attack_analysis = self.analyze_attack_vectors(&nodes).await?;
        
        // Create export metadata
        let metadata = ExportMetadata {
            export_id: Uuid::new_v4(),
            exported_at: Utc::now(),
            format: self.config.default_format.clone(),
            requester,
            purpose,
            time_range: time_range.unwrap_or_else(|| {
                let now = Utc::now();
                TimeRange {
                    start: now - chrono::Duration::days(30),
                    end: now,
                }
            }),
            version: "1.0".to_string(),
            signature: None, // TODO: Add digital signature
        };
        
        let package = ExportPackage {
            metadata,
            audit_nodes: nodes,
            proof_chains,
            statistics,
            compliance,
            attack_analysis,
        };
        
        println!("✅ Forensic export package generated");
        println!("   Export ID: {}", package.metadata.export_id);
        println!("   Total nodes: {}", package.statistics.total_nodes);
        println!("   Binary data: {} bytes", package.statistics.total_binary_size_bytes);
        
        Ok(package)
    }
    
    /// Export to specific format
    pub async fn export_to_format(
        &self,
        package: &ExportPackage,
        format: ExportFormat,
    ) -> Result<Vec<u8>> {
        match format {
            ExportFormat::Json => {
                let json = serde_json::to_string_pretty(package)?;
                Ok(json.into_bytes())
            }
            ExportFormat::Cbor => {
                let cbor = serde_cbor::to_vec(package)?;
                Ok(cbor)
            }
            ExportFormat::MessagePack => {
                let msgpack = rmp_serde::to_vec(package)?;
                Ok(msgpack)
            }
            ExportFormat::Csv => {
                self.export_to_csv(package).await
            }
            ExportFormat::Xml => {
                self.export_to_xml(package).await
            }
            ExportFormat::Pdf => {
                self.export_to_pdf(package).await
            }
            ExportFormat::Custom(ref name) => {
                Err(anyhow!("Custom format '{}' not implemented", name))
            }
        }
    }
    
    /// Export to CSV format
    async fn export_to_csv(&self, package: &ExportPackage) -> Result<Vec<u8>> {
        let mut csv_content = String::new();
        
        // Header
        csv_content.push_str("Node ID,Timestamp,Runtime Type,Operation,Operation Details,Audit Level,Compliance Tags\n");
        
        // Data rows
        for node in &package.audit_nodes {
            let compliance_tags = node.compliance_tags.iter()
                .map(|tag| format!("{:?}", tag))
                .collect::<Vec<_>>()
                .join(";");
            
            csv_content.push_str(&format!(
                "{},{},{:?},{:?},{},{:?},{}\n",
                format!("{:?}", node.node_id),
                node.timestamp_ns,
                match &node.execution_context {
                    crate::runtime_node::ExecutionContext::DockLock { .. } => "DockLock",
                    crate::runtime_node::ExecutionContext::EncCluster { .. } => "EncCluster",
                    crate::runtime_node::ExecutionContext::HttpCage { .. } => "HttpCage",
                    crate::runtime_node::ExecutionContext::IoTGateway { .. } => "IoTGateway",
                    crate::runtime_node::ExecutionContext::MobileClient { .. } => "MobileClient",
                    crate::runtime_node::ExecutionContext::FrontendClient { .. } => "FrontendClient",
                    crate::runtime_node::ExecutionContext::SecurityMonitor { .. } => "SecurityMonitor",
                },
                format!("{:?}", node.operation_type),
                String::from_utf8_lossy(&node.operation_data.data).replace(',', ";"),
                node.audit_level,
                compliance_tags
            ));
        }
        
        Ok(csv_content.into_bytes())
    }
    
    /// Export to XML format (simplified)
    async fn export_to_xml(&self, package: &ExportPackage) -> Result<Vec<u8>> {
        let mut xml_content = String::new();
        xml_content.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml_content.push_str("<audit_export>\n");
        xml_content.push_str(&format!("  <metadata export_id=\"{}\" exported_at=\"{}\" />\n", 
            package.metadata.export_id, package.metadata.exported_at));
        xml_content.push_str("  <audit_nodes>\n");
        
        for node in &package.audit_nodes {
            xml_content.push_str(&format!(
                "    <node id=\"{}\" timestamp=\"{}\" operation=\"{:?}\" level=\"{:?}\" />\n",
                format!("{:?}", node.node_id), node.timestamp_ns, format!("{:?}", node.operation_type), node.audit_level
            ));
        }
        
        xml_content.push_str("  </audit_nodes>\n");
        xml_content.push_str("</audit_export>\n");
        
        Ok(xml_content.into_bytes())
    }
    
    /// Export to PDF format (placeholder)
    async fn export_to_pdf(&self, _package: &ExportPackage) -> Result<Vec<u8>> {
        // TODO: Implement PDF generation using a PDF library
        Err(anyhow!("PDF export not yet implemented"))
    }
    
    /// Generate export statistics
    fn generate_export_statistics(&self, nodes: &[RuntimeAuditNode]) -> ExportStatistics {
        let mut nodes_by_runtime = HashMap::new();
        let mut nodes_by_level = HashMap::new();
        let mut total_binary_size = 0;
        
        for node in nodes {
            // Count by runtime type
            let runtime_key = format!("{:?}", node.execution_context);
            *nodes_by_runtime.entry(runtime_key).or_insert(0) += 1;
            
            // Count by audit level
            let level_key = format!("{:?}", node.audit_level);
            *nodes_by_level.entry(level_key).or_insert(0) += 1;
            
            // Sum binary data size
            total_binary_size += node.binary_outputs.iter().map(|b| b.data.len()).sum::<usize>();
        }
        
        ExportStatistics {
            total_nodes: nodes.len(),
            nodes_by_runtime,
            nodes_by_level,
            total_binary_size_bytes: total_binary_size,
            export_size_bytes: 0, // Will be set after serialization
            compression_ratio: 1.0, // Will be calculated if compression is used
        }
    }
    
    /// Generate compliance information
    fn generate_compliance_info(&self, nodes: &[RuntimeAuditNode]) -> ComplianceInfo {
        let mut all_tags = Vec::new();
        let mut frameworks = std::collections::HashSet::new();
        
        for node in nodes {
            all_tags.extend(node.compliance_tags.clone());
            
            // Map compliance tags to frameworks
            for tag in &node.compliance_tags {
                match tag {
                    ComplianceTag::GDPR => { frameworks.insert("GDPR".to_string()); }
                    ComplianceTag::HIPAA => { frameworks.insert("HIPAA".to_string()); }
                    ComplianceTag::SOX => { frameworks.insert("SOX".to_string()); }
                    ComplianceTag::PCI_DSS => { frameworks.insert("PCI-DSS".to_string()); }
                    ComplianceTag::Custom(ref name) if name == "NIST" => { frameworks.insert("NIST".to_string()); }
                    ComplianceTag::ISO27001 => { frameworks.insert("ISO27001".to_string()); }
                    _ => {}
                }
            }
        }
        
        // Remove duplicates
        all_tags.sort();
        all_tags.dedup();
        
        ComplianceInfo {
            frameworks: frameworks.into_iter().collect(),
            tags: all_tags,
            violations: Vec::new(), // TODO: Implement violation detection
            retention_years: 7, // Default retention period
        }
    }
    
    /// Analyze attack vectors in audit data
    async fn analyze_attack_vectors(&self, nodes: &[RuntimeAuditNode]) -> Result<Option<AttackVectorAnalysis>> {
        let mut attack_patterns = Vec::new();
        let mut timeline = Vec::new();
        let mut affected_systems = std::collections::HashSet::<String>::new();
        let mut iocs = Vec::new();
        
        // Simple attack pattern detection
        for node in nodes {
            // Look for suspicious patterns
            if String::from_utf8_lossy(&node.operation_data.data).to_lowercase().contains("failed") ||
               String::from_utf8_lossy(&node.operation_data.data).to_lowercase().contains("error") ||
               String::from_utf8_lossy(&node.operation_data.data).to_lowercase().contains("unauthorized") {
                
                attack_patterns.push(AttackPattern {
                    pattern_id: "SUSPICIOUS_ACTIVITY".to_string(),
                    name: "Suspicious Activity Detected".to_string(),
                    mitre_technique: None,
                    confidence: 50,
                    evidence_nodes: vec![uuid::Uuid::from_bytes(node.node_id[..16].try_into().unwrap_or([0u8; 16]))],
                });
                
                timeline.push(AttackEvent {
                    timestamp: Utc::now(), // TODO: Fix DateTime conversion
                    event_type: "Suspicious Activity".to_string(),
                    description: String::from_utf8_lossy(&node.operation_data.data).to_string(),
                    audit_node_id: uuid::Uuid::from_bytes(node.node_id[..16].try_into().unwrap_or([0u8; 16])),
                });
                
                affected_systems.insert(format!("{:?}", node.runtime_address));
            }
        }
        
        if attack_patterns.is_empty() {
            return Ok(None);
        }
        
        // Sort timeline by timestamp
        timeline.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        let analysis = AttackVectorAnalysis {
            analysis_id: Uuid::new_v4(),
            attack_patterns,
            timeline,
            affected_systems: affected_systems.into_iter().collect(),
            severity: AttackSeverity::Medium,
            iocs,
        };
        
        Ok(Some(analysis))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RuntimeAuditNode;
    
    #[tokio::test]
    async fn test_export_engine_creation() {
        let config = ExportConfig::default();
        let engine = ExportEngine::new(config);
        
        assert!(engine.templates.contains_key(&ExportFormat::Json));
        assert!(engine.templates.contains_key(&ExportFormat::Csv));
    }
    
    #[tokio::test]
    async fn test_csv_export() {
        let config = ExportConfig::default();
        let engine = ExportEngine::new(config);
        
        let package = ExportPackage {
            metadata: ExportMetadata {
                export_id: Uuid::new_v4(),
                exported_at: Utc::now(),
                format: ExportFormat::Csv,
                requester: "test".to_string(),
                purpose: ExportPurpose::ForensicInvestigation,
                time_range: TimeRange {
                    start: Utc::now() - chrono::Duration::hours(1),
                    end: Utc::now(),
                },
                version: "1.0".to_string(),
                signature: None,
            },
            audit_nodes: vec![RuntimeAuditNode::default()],
            proof_chains: vec![],
            statistics: ExportStatistics {
                total_nodes: 1,
                nodes_by_runtime: HashMap::new(),
                nodes_by_level: HashMap::new(),
                total_binary_size_bytes: 0,
                export_size_bytes: 0,
                compression_ratio: 1.0,
            },
            compliance: ComplianceInfo {
                frameworks: vec![],
                tags: vec![],
                violations: vec![],
                retention_years: 7,
            },
            attack_analysis: None,
        };
        
        let csv_data = engine.export_to_format(&package, ExportFormat::Csv).await.unwrap();
        let csv_string = String::from_utf8(csv_data).unwrap();
        
        assert!(csv_string.contains("Node ID,Timestamp"));
        assert!(csv_string.contains("End of Export"));
    }
}
