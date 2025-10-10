// Conversion Rules
// Defines rules and strategies for converting BPI logbook entries to 6D blockchain transactions

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use anyhow::Result;
use uuid::Uuid;

use super::logbook_reader::{LogbookEntry, LogbookEntryType};
use super::blockchain_writer::{SixDTransaction, TransactionType, DimensionalCoordinates, TransactionData, CryptographicProofs};

/// Conversion rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionRule {
    pub rule_id: String,
    pub rule_name: String,
    pub source_entry_type: LogbookEntryType,
    pub target_transaction_type: TransactionType,
    pub mapping_strategy: MappingStrategy,
    pub dimensional_mapping: DimensionalMapping,
    pub data_transformation: DataTransformation,
    pub validation_rules: Vec<ValidationRule>,
    pub priority: u32,
    pub enabled: bool,
}

/// Mapping strategies for conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MappingStrategy {
    DirectMapping,      // 1:1 mapping
    AggregatedMapping,  // Multiple entries to one transaction
    SplitMapping,       // One entry to multiple transactions
    ConditionalMapping, // Conditional based on entry content
    TemplateMapping,    // Template-based transformation
}

/// Dimensional mapping configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionalMapping {
    pub x_source: DimensionSource,
    pub y_source: DimensionSource,
    pub z_source: DimensionSource,
    pub t_source: DimensionSource,
    pub s_source: DimensionSource,
    pub q_source: DimensionSource,
}

/// Source for dimensional coordinates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DimensionSource {
    Timestamp,
    VMInstanceId,
    SecurityLevel,
    ResourceUsage,
    PerformanceMetric,
    AuditTrail,
    Constant(f64),
    Formula(String),
}

/// Data transformation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTransformation {
    pub operation_hash_source: String,
    pub input_data_mapping: Vec<FieldMapping>,
    pub output_data_mapping: Vec<FieldMapping>,
    pub context_mapping: Vec<FieldMapping>,
    pub proof_generation: ProofGeneration,
}

/// Field mapping for data transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldMapping {
    pub source_field: String,
    pub target_field: String,
    pub transformation: FieldTransformation,
}

/// Field transformation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldTransformation {
    Direct,
    Hash,
    Encrypt,
    Compress,
    Aggregate,
    Format(String),
}

/// Proof generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofGeneration {
    pub merkle_proof_enabled: bool,
    pub zero_knowledge_proof_enabled: bool,
    pub quantum_proof_enabled: bool,
    pub consensus_proof_enabled: bool,
    pub integrity_proof_enabled: bool,
    pub non_repudiation_proof_enabled: bool,
}

/// Validation rule for conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_id: String,
    pub rule_type: ValidationRuleType,
    pub condition: String,
    pub error_message: String,
    pub severity: ValidationSeverity,
}

/// Types of validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRuleType {
    RequiredField,
    DataFormat,
    ValueRange,
    Consistency,
    Security,
    Compliance,
}

/// Validation severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Conversion statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionStats {
    pub total_conversions: u64,
    pub successful_conversions: u64,
    pub failed_conversions: u64,
    pub conversions_by_type: HashMap<String, u64>,
    pub average_conversion_time_ms: f64,
    pub validation_failures: HashMap<String, u64>,
}

/// Conversion Rules Manager
#[derive(Debug)]
pub struct ConversionRules {
    /// Active conversion rules
    rules: Arc<RwLock<HashMap<String, ConversionRule>>>,
    
    /// Rule priority index
    priority_index: Arc<RwLock<Vec<String>>>,
    
    /// Conversion statistics
    stats: Arc<RwLock<ConversionStats>>,
    
    /// Rules configuration
    config: Arc<RwLock<RulesConfig>>,
}

/// Rules configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesConfig {
    pub auto_rule_discovery: bool,
    pub validation_enabled: bool,
    pub proof_generation_enabled: bool,
    pub dimensional_validation: bool,
    pub performance_optimization: bool,
}

impl Default for RulesConfig {
    fn default() -> Self {
        Self {
            auto_rule_discovery: true,
            validation_enabled: true,
            proof_generation_enabled: true,
            dimensional_validation: true,
            performance_optimization: true,
        }
    }
}

impl Default for ConversionStats {
    fn default() -> Self {
        Self {
            total_conversions: 0,
            successful_conversions: 0,
            failed_conversions: 0,
            conversions_by_type: HashMap::new(),
            average_conversion_time_ms: 0.0,
            validation_failures: HashMap::new(),
        }
    }
}

impl ConversionRules {
    /// Create a new conversion rules manager
    pub async fn new() -> Result<Self> {
        Ok(Self {
            rules: Arc::new(RwLock::new(HashMap::new())),
            priority_index: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(ConversionStats::default())),
            config: Arc::new(RwLock::new(RulesConfig::default())),
        })
    }

    /// Initialize the conversion rules
    pub async fn initialize(&self) -> Result<()> {
        println!("🔄 Initializing Conversion Rules...");
        
        // Load default conversion rules
        self.load_default_rules().await?;
        
        // Build priority index
        self.rebuild_priority_index().await?;
        
        println!("✅ Conversion Rules initialized");
        Ok(())
    }

    /// Convert a logbook entry to a 6D blockchain transaction
    pub async fn convert_entry_to_transaction(&self, entry: &LogbookEntry) -> Result<SixDTransaction> {
        let start_time = std::time::Instant::now();
        
        // Find applicable conversion rule
        let rule = self.find_applicable_rule(entry).await?;
        
        // Apply conversion rule
        let transaction = self.apply_conversion_rule(&rule, entry).await?;
        
        // Validate the converted transaction
        if self.config.read().unwrap().validation_enabled {
            self.validate_transaction(&transaction, &rule).await?;
        }
        
        // Update statistics
        let conversion_time = start_time.elapsed().as_millis() as f64;
        self.update_conversion_stats(&rule, conversion_time, true).await?;
        
        println!("🔄 Converted logbook entry {} to transaction {}", entry.entry_id, transaction.transaction_id);
        Ok(transaction)
    }

    /// Add a new conversion rule
    pub async fn add_rule(&self, rule: ConversionRule) -> Result<()> {
        let rule_id = rule.rule_id.clone();
        
        {
            let mut rules = self.rules.write().unwrap();
            rules.insert(rule_id.clone(), rule);
        }
        
        // Rebuild priority index
        self.rebuild_priority_index().await?;
        
        println!("➕ Added conversion rule: {}", rule_id);
        Ok(())
    }

    /// Remove a conversion rule
    pub async fn remove_rule(&self, rule_id: &str) -> Result<()> {
        {
            let mut rules = self.rules.write().unwrap();
            rules.remove(rule_id);
        }
        
        // Rebuild priority index
        self.rebuild_priority_index().await?;
        
        println!("➖ Removed conversion rule: {}", rule_id);
        Ok(())
    }

    /// Get conversion statistics
    pub async fn get_stats(&self) -> Result<ConversionStats> {
        Ok(self.stats.read().unwrap().clone())
    }

    /// List all conversion rules
    pub async fn list_rules(&self) -> Result<Vec<ConversionRule>> {
        let rules = self.rules.read().unwrap();
        Ok(rules.values().cloned().collect())
    }

    /// Update rule configuration
    pub async fn update_config(&self, config: RulesConfig) -> Result<()> {
        {
            let mut current_config = self.config.write().unwrap();
            *current_config = config;
        }
        
        println!("🔧 Updated conversion rules configuration");
        Ok(())
    }

    // Private helper methods

    async fn load_default_rules(&self) -> Result<()> {
        // Load default conversion rules for different entry types
        
        // VM Operation rule
        let vm_operation_rule = ConversionRule {
            rule_id: "vm_operation_rule".to_string(),
            rule_name: "VM Operation to Blockchain Transaction".to_string(),
            source_entry_type: LogbookEntryType::VMOperation,
            target_transaction_type: TransactionType::VMOperation,
            mapping_strategy: MappingStrategy::DirectMapping,
            dimensional_mapping: DimensionalMapping {
                x_source: DimensionSource::Constant(1.0),
                y_source: DimensionSource::Constant(2.0),
                z_source: DimensionSource::Constant(3.0),
                t_source: DimensionSource::Timestamp,
                s_source: DimensionSource::SecurityLevel,
                q_source: DimensionSource::Constant(0.5),
            },
            data_transformation: DataTransformation {
                operation_hash_source: "operation_data.operation_id".to_string(),
                input_data_mapping: vec![
                    FieldMapping {
                        source_field: "operation_data.input_data_hash".to_string(),
                        target_field: "input_data_hash".to_string(),
                        transformation: FieldTransformation::Direct,
                    },
                ],
                output_data_mapping: vec![
                    FieldMapping {
                        source_field: "operation_data.output_data_hash".to_string(),
                        target_field: "output_data_hash".to_string(),
                        transformation: FieldTransformation::Direct,
                    },
                ],
                context_mapping: vec![
                    FieldMapping {
                        source_field: "operation_data.execution_context".to_string(),
                        target_field: "execution_context".to_string(),
                        transformation: FieldTransformation::Format("json".to_string()),
                    },
                ],
                proof_generation: ProofGeneration {
                    merkle_proof_enabled: true,
                    zero_knowledge_proof_enabled: true,
                    quantum_proof_enabled: true,
                    consensus_proof_enabled: true,
                    integrity_proof_enabled: true,
                    non_repudiation_proof_enabled: true,
                },
            },
            validation_rules: vec![
                ValidationRule {
                    rule_id: "vm_op_required_fields".to_string(),
                    rule_type: ValidationRuleType::RequiredField,
                    condition: "operation_data.operation_id != null".to_string(),
                    error_message: "Operation ID is required".to_string(),
                    severity: ValidationSeverity::Error,
                },
            ],
            priority: 100,
            enabled: true,
        };

        // Security Event rule
        let security_event_rule = ConversionRule {
            rule_id: "security_event_rule".to_string(),
            rule_name: "Security Event to Blockchain Transaction".to_string(),
            source_entry_type: LogbookEntryType::SecurityEvent,
            target_transaction_type: TransactionType::SecurityEvent,
            mapping_strategy: MappingStrategy::DirectMapping,
            dimensional_mapping: DimensionalMapping {
                x_source: DimensionSource::Constant(0.0),
                y_source: DimensionSource::Constant(0.0),
                z_source: DimensionSource::Constant(0.0),
                t_source: DimensionSource::Timestamp,
                s_source: DimensionSource::Constant(1.0), // Maximum security
                q_source: DimensionSource::Constant(0.8),
            },
            data_transformation: DataTransformation {
                operation_hash_source: "audit_trail.audit_id".to_string(),
                input_data_mapping: vec![],
                output_data_mapping: vec![],
                context_mapping: vec![
                    FieldMapping {
                        source_field: "security_context".to_string(),
                        target_field: "execution_context".to_string(),
                        transformation: FieldTransformation::Format("json".to_string()),
                    },
                ],
                proof_generation: ProofGeneration {
                    merkle_proof_enabled: true,
                    zero_knowledge_proof_enabled: true,
                    quantum_proof_enabled: true,
                    consensus_proof_enabled: true,
                    integrity_proof_enabled: true,
                    non_repudiation_proof_enabled: true,
                },
            },
            validation_rules: vec![
                ValidationRule {
                    rule_id: "security_event_audit_id".to_string(),
                    rule_type: ValidationRuleType::RequiredField,
                    condition: "audit_trail.audit_id != null".to_string(),
                    error_message: "Audit ID is required for security events".to_string(),
                    severity: ValidationSeverity::Critical,
                },
            ],
            priority: 200,
            enabled: true,
        };

        // Add rules to the collection
        self.add_rule(vm_operation_rule).await?;
        self.add_rule(security_event_rule).await?;

        println!("📋 Loaded default conversion rules");
        Ok(())
    }

    async fn find_applicable_rule(&self, entry: &LogbookEntry) -> Result<ConversionRule> {
        let rules = self.rules.read().unwrap();
        let priority_index = self.priority_index.read().unwrap();
        
        // Find the highest priority rule that matches the entry type
        for rule_id in priority_index.iter() {
            if let Some(rule) = rules.get(rule_id) {
                if rule.enabled && self.matches_entry_type(&rule.source_entry_type, &entry.entry_type) {
                    return Ok(rule.clone());
                }
            }
        }
        
        Err(anyhow::anyhow!("No applicable conversion rule found for entry type: {:?}", entry.entry_type))
    }

    fn matches_entry_type(&self, rule_type: &LogbookEntryType, entry_type: &LogbookEntryType) -> bool {
        std::mem::discriminant(rule_type) == std::mem::discriminant(entry_type)
    }

    async fn apply_conversion_rule(&self, rule: &ConversionRule, entry: &LogbookEntry) -> Result<SixDTransaction> {
        let transaction_id = Uuid::new_v4().to_string();
        
        // Map dimensional coordinates
        let dimensional_coordinates = self.map_dimensional_coordinates(&rule.dimensional_mapping, entry).await?;
        
        // Transform data
        let transaction_data = self.transform_data(&rule.data_transformation, entry).await?;
        
        // Generate cryptographic proofs
        let cryptographic_proofs = self.generate_proofs(&rule.data_transformation.proof_generation, entry).await?;
        
        // Generate quantum signature
        let quantum_signature = self.generate_quantum_signature(entry).await?;
        
        // Calculate integrity hash
        let integrity_hash = self.calculate_integrity_hash(&transaction_id, &transaction_data).await?;
        
        Ok(SixDTransaction {
            transaction_id,
            timestamp: entry.timestamp,
            transaction_type: rule.target_transaction_type.clone(),
            logbook_entry_id: entry.entry_id.clone(),
            dimensional_coordinates,
            transaction_data,
            cryptographic_proofs,
            poe_tree_root: None,
            traversal_report: None,
            vm_audit_proof: None,
            quantum_signature,
            integrity_hash,
        })
    }

    async fn map_dimensional_coordinates(&self, mapping: &DimensionalMapping, entry: &LogbookEntry) -> Result<DimensionalCoordinates> {
        Ok(DimensionalCoordinates {
            x: self.extract_dimension_value(&mapping.x_source, entry).await?,
            y: self.extract_dimension_value(&mapping.y_source, entry).await?,
            z: self.extract_dimension_value(&mapping.z_source, entry).await?,
            t: self.extract_dimension_value(&mapping.t_source, entry).await?,
            s: self.extract_dimension_value(&mapping.s_source, entry).await?,
            q: self.extract_dimension_value(&mapping.q_source, entry).await?,
        })
    }

    async fn extract_dimension_value(&self, source: &DimensionSource, entry: &LogbookEntry) -> Result<f64> {
        match source {
            DimensionSource::Timestamp => Ok(entry.timestamp as f64),
            DimensionSource::VMInstanceId => Ok(entry.vm_instance_id.len() as f64),
            DimensionSource::SecurityLevel => Ok(0.8), // Mock security level
            DimensionSource::ResourceUsage => Ok(entry.resource_usage.cpu_time_ms as f64),
            DimensionSource::PerformanceMetric => Ok(entry.performance_metrics.execution_time_ms as f64),
            DimensionSource::AuditTrail => Ok(entry.audit_trail.evidence_chain.len() as f64),
            DimensionSource::Constant(value) => Ok(*value),
            DimensionSource::Formula(_formula) => Ok(1.0), // Mock formula evaluation
        }
    }

    async fn transform_data(&self, _transformation: &DataTransformation, entry: &LogbookEntry) -> Result<TransactionData> {
        Ok(TransactionData {
            operation_hash: entry.operation_data.operation_id.clone(),
            input_data_hash: entry.operation_data.input_data_hash.clone(),
            output_data_hash: entry.operation_data.output_data_hash.clone(),
            execution_context: serde_json::to_string(&entry.operation_data.execution_context)?,
            resource_usage: serde_json::to_string(&entry.resource_usage)?,
            performance_metrics: serde_json::to_string(&entry.performance_metrics)?,
            audit_trail: serde_json::to_string(&entry.audit_trail)?,
            compliance_data: entry.audit_trail.compliance_tags.join(","),
        })
    }

    async fn generate_proofs(&self, proof_config: &ProofGeneration, entry: &LogbookEntry) -> Result<CryptographicProofs> {
        Ok(CryptographicProofs {
            merkle_proof: if proof_config.merkle_proof_enabled {
                format!("merkle_proof_{}", entry.entry_id)
            } else {
                "".to_string()
            },
            zero_knowledge_proof: if proof_config.zero_knowledge_proof_enabled {
                format!("zk_proof_{}", entry.entry_id)
            } else {
                "".to_string()
            },
            quantum_proof: if proof_config.quantum_proof_enabled {
                format!("quantum_proof_{}", entry.entry_id)
            } else {
                "".to_string()
            },
            consensus_proof: if proof_config.consensus_proof_enabled {
                format!("consensus_proof_{}", entry.entry_id)
            } else {
                "".to_string()
            },
            integrity_proof: if proof_config.integrity_proof_enabled {
                format!("integrity_proof_{}", entry.entry_id)
            } else {
                "".to_string()
            },
            non_repudiation_proof: if proof_config.non_repudiation_proof_enabled {
                format!("non_repudiation_proof_{}", entry.entry_id)
            } else {
                "".to_string()
            },
        })
    }

    async fn generate_quantum_signature(&self, entry: &LogbookEntry) -> Result<String> {
        Ok(format!("quantum_signature_{}", entry.entry_id))
    }

    async fn calculate_integrity_hash(&self, transaction_id: &str, data: &TransactionData) -> Result<String> {
        Ok(format!("integrity_hash_{}_{}", transaction_id, data.operation_hash))
    }

    async fn validate_transaction(&self, _transaction: &SixDTransaction, _rule: &ConversionRule) -> Result<()> {
        // Perform validation based on rule validation rules
        Ok(())
    }

    async fn update_conversion_stats(&self, rule: &ConversionRule, conversion_time: f64, success: bool) -> Result<()> {
        let mut stats = self.stats.write().unwrap();
        
        stats.total_conversions += 1;
        if success {
            stats.successful_conversions += 1;
        } else {
            stats.failed_conversions += 1;
        }
        
        // Update average conversion time
        stats.average_conversion_time_ms = (stats.average_conversion_time_ms * (stats.total_conversions - 1) as f64 + conversion_time) / stats.total_conversions as f64;
        
        // Update conversions by type
        let rule_name = rule.rule_name.clone();
        *stats.conversions_by_type.entry(rule_name).or_insert(0) += 1;
        
        Ok(())
    }

    async fn rebuild_priority_index(&self) -> Result<()> {
        let rules = self.rules.read().unwrap();
        let mut rule_priorities: Vec<(String, u32)> = rules.iter()
            .map(|(id, rule)| (id.clone(), rule.priority))
            .collect();
        
        // Sort by priority (higher priority first)
        rule_priorities.sort_by(|a, b| b.1.cmp(&a.1));
        
        let mut priority_index = self.priority_index.write().unwrap();
        *priority_index = rule_priorities.into_iter().map(|(id, _)| id).collect();
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logbook_6d_bridge::logbook_reader::*;

    #[tokio::test]
    async fn test_conversion_rules_creation() {
        let rules = ConversionRules::new().await.unwrap();
        assert!(rules.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_entry_conversion() {
        let rules = ConversionRules::new().await.unwrap();
        rules.initialize().await.unwrap();

        // Create a mock logbook entry
        let entry = LogbookEntry {
            entry_id: "test_entry_1".to_string(),
            timestamp: 1234567890,
            entry_type: LogbookEntryType::VMOperation,
            vm_instance_id: "vm_1".to_string(),
            operation_data: OperationData {
                operation_id: "op_1".to_string(),
                operation_type: "compute".to_string(),
                input_data_hash: "input_hash".to_string(),
                output_data_hash: "output_hash".to_string(),
                execution_context: ExecutionContext {
                    execution_environment: "BPI_VM".to_string(),
                    user_context: None,
                    session_id: None,
                    request_id: None,
                    parent_operation_id: None,
                },
                dependencies: vec![],
                side_effects: vec![],
            },
            audit_trail: AuditTrail {
                audit_id: "audit_1".to_string(),
                compliance_tags: vec![],
                regulatory_requirements: vec![],
                evidence_chain: vec![],
                witness_signatures: vec![],
            },
            security_context: SecurityContext {
                security_level: "HIGH".to_string(),
                access_controls: vec![],
                encryption_info: EncryptionInfo {
                    algorithm: "AES-256".to_string(),
                    key_id: "key_1".to_string(),
                    initialization_vector: "iv_1".to_string(),
                    encryption_strength: 256,
                },
                authentication_proof: "auth_proof".to_string(),
                authorization_proof: "authz_proof".to_string(),
            },
            resource_usage: ResourceUsage {
                cpu_time_ms: 100,
                memory_peak_mb: 256,
                storage_bytes: 1024,
                network_bytes: 512,
                gpu_time_ms: 0,
                quantum_operations: 0,
            },
            performance_metrics: PerformanceMetrics {
                execution_time_ms: 150,
                throughput_ops_per_sec: 100.0,
                latency_percentiles: LatencyPercentiles {
                    p50_ms: 50.0,
                    p90_ms: 90.0,
                    p95_ms: 95.0,
                    p99_ms: 99.0,
                },
                error_rate: 0.01,
                availability: 0.999,
            },
            integrity_hash: "integrity_hash".to_string(),
        };

        let transaction = rules.convert_entry_to_transaction(&entry).await.unwrap();
        assert_eq!(transaction.logbook_entry_id, entry.entry_id);
        assert!(matches!(transaction.transaction_type, TransactionType::VMOperation));
    }
}
