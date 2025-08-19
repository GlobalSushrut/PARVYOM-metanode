use anyhow::Result;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    CueValidator, CueSchema, CueConstraint, ValidationResult, ValidationError, ValidationWarning,
    SecuritySeverity,
};

impl CueValidator {
    /// Create a new CUE validator
    pub fn new() -> Self {
        Self {
            schemas: Self::default_cue_schemas(),
            validation_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Default CUE schemas for YAML SmartContracts++
    fn default_cue_schemas() -> HashMap<String, CueSchema> {
        let mut schemas = HashMap::new();
        
        // YAML Contract Schema
        schemas.insert("yaml_contract".to_string(), CueSchema {
            name: "yaml_contract".to_string(),
            cue_definition: r#"
                contract: {
                    name: string
                    version: string & =~"^[0-9]+\.[0-9]+\.[0-9]+$"
                    description?: string
                    author?: string
                    license?: string
                    
                    states: [string]: {
                        type: "initial" | "normal" | "final" | "error"
                        actions?: [...string]
                        conditions?: [...string]
                        transitions?: [string]: string
                    }
                    
                    variables?: [string]: {
                        type: "string" | "number" | "boolean" | "object" | "array"
                        default?: _
                        constraints?: [...string]
                    }
                    
                    events?: [string]: {
                        parameters?: [string]: string
                        description?: string
                    }
                    
                    functions?: [string]: {
                        parameters?: [string]: string
                        returns?: string
                        description?: string
                        body: string
                    }
                }
            "#.to_string(),
            constraints: vec![
                CueConstraint {
                    name: "version_format".to_string(),
                    expression: "version & =~\"^[0-9]+\\.[0-9]+\\.[0-9]+$\"".to_string(),
                    error_message: "Version must follow semantic versioning (e.g., 1.0.0)".to_string(),
                },
                CueConstraint {
                    name: "state_types".to_string(),
                    expression: "states[_].type & (\"initial\" | \"normal\" | \"final\" | \"error\")".to_string(),
                    error_message: "State type must be one of: initial, normal, final, error".to_string(),
                },
                CueConstraint {
                    name: "initial_state_required".to_string(),
                    expression: "len([for k, v in states if v.type == \"initial\" {k}]) >= 1".to_string(),
                    error_message: "Contract must have at least one initial state".to_string(),
                },
            ],
        });

        // Agreement Schema
        schemas.insert("yaml_agreement".to_string(), CueSchema {
            name: "yaml_agreement".to_string(),
            cue_definition: r#"
                agreement: {
                    name: string
                    version: string & =~"^[0-9]+\.[0-9]+\.[0-9]+$"
                    
                    parties: [...{
                        name: string
                        role: string
                        identifier?: string
                        contact?: string
                    }]
                    
                    terms: [...{
                        id: string
                        description: string
                        conditions?: [...string]
                        penalties?: [...string]
                    }]
                    
                    conditions: {
                        effective_date?: string
                        expiration_date?: string
                        jurisdiction?: string
                        governing_law?: string
                    }
                    
                    dispute_resolution?: {
                        method: "arbitration" | "mediation" | "court"
                        jurisdiction?: string
                        rules?: string
                    }
                    
                    signatures?: [...{
                        party: string
                        signature: string
                        timestamp: string
                        witness?: string
                    }]
                }
            "#.to_string(),
            constraints: vec![
                CueConstraint {
                    name: "minimum_parties".to_string(),
                    expression: "len(parties) >= 2".to_string(),
                    error_message: "Agreement must have at least two parties".to_string(),
                },
                CueConstraint {
                    name: "minimum_terms".to_string(),
                    expression: "len(terms) >= 1".to_string(),
                    error_message: "Agreement must have at least one term".to_string(),
                },
            ],
        });

        // Data Processing Agreement Schema
        schemas.insert("data_processing_agreement".to_string(), CueSchema {
            name: "data_processing_agreement".to_string(),
            cue_definition: r#"
                contract: {
                    name: "DataProcessingAgreement"
                    version: string & =~"^[0-9]+\.[0-9]+\.[0-9]+$"
                    
                    data_controller: {
                        name: string
                        contact: string
                        jurisdiction: string
                    }
                    
                    data_processor: {
                        name: string
                        contact: string
                        certifications?: [...string]
                    }
                    
                    processing_purposes: [...string]
                    
                    data_categories: [...{
                        category: string
                        sensitivity: "low" | "medium" | "high" | "critical"
                        retention_period: string
                    }]
                    
                    security_measures: [...{
                        measure: string
                        implementation: string
                        compliance_standard?: string
                    }]
                    
                    states: {
                        "data_collection": {
                            type: "initial"
                            actions: ["validate_consent", "log_collection"]
                        }
                        "data_processing": {
                            type: "normal"
                            actions: ["apply_security_measures", "monitor_access"]
                        }
                        "data_retention": {
                            type: "normal"
                            actions: ["enforce_retention_policy"]
                        }
                        "data_deletion": {
                            type: "final"
                            actions: ["secure_deletion", "generate_certificate"]
                        }
                    }
                }
            "#.to_string(),
            constraints: vec![
                CueConstraint {
                    name: "gdpr_compliance".to_string(),
                    expression: "len([for cat in data_categories if cat.sensitivity == \"high\" || cat.sensitivity == \"critical\" {cat}]) > 0 => len(security_measures) >= 3".to_string(),
                    error_message: "High/critical sensitivity data requires at least 3 security measures".to_string(),
                },
            ],
        });

        schemas
    }

    /// Validate YAML against CUE schema
    pub async fn validate(&self, yaml_content: &str) -> Result<ValidationResult> {
        // Check cache first
        let cache_key = format!("{:x}", md5::compute(yaml_content.as_bytes()));
        {
            let cache = self.validation_cache.read().await;
            if let Some(cached_result) = cache.get(&cache_key) {
                tracing::debug!("✅ Using cached CUE validation result");
                return Ok(cached_result.clone());
            }
        }

        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Parse YAML to check basic structure
        match serde_yaml::from_str::<serde_yaml::Value>(yaml_content) {
            Ok(yaml_value) => {
                // Determine which schema to use
                let schema_name = self.determine_schema(&yaml_value);
                
                if let Some(schema) = self.schemas.get(&schema_name) {
                    // Apply CUE validation using the determined schema
                    self.validate_against_schema(schema, &yaml_value, &mut errors, &mut warnings).await?;
                } else {
                    warnings.push(ValidationWarning {
                        code: "UNKNOWN_SCHEMA".to_string(),
                        message: format!("No CUE schema found for contract type: {}", schema_name),
                        location: None,
                    });
                }
            },
            Err(e) => {
                errors.push(ValidationError {
                    code: "YAML_PARSE_ERROR".to_string(),
                    message: format!("YAML parsing error: {}", e),
                    location: None,
                    severity: SecuritySeverity::High,
                });
            }
        }

        let result = ValidationResult {
            success: errors.is_empty(),
            errors,
            warnings,
            validated_at: Utc::now(),
        };

        // Cache the result
        {
            let mut cache = self.validation_cache.write().await;
            cache.insert(cache_key, result.clone());
        }

        Ok(result)
    }

    /// Determine which CUE schema to use based on YAML content
    fn determine_schema(&self, yaml_value: &serde_yaml::Value) -> String {
        if let Some(mapping) = yaml_value.as_mapping() {
            // Check for contract field
            if mapping.contains_key(&serde_yaml::Value::String("contract".to_string())) {
                // Check if it's a data processing agreement
                if let Some(contract_section) = mapping.get(&serde_yaml::Value::String("contract".to_string())) {
                    if let Some(contract_map) = contract_section.as_mapping() {
                        if let Some(name_value) = contract_map.get(&serde_yaml::Value::String("name".to_string())) {
                            if let Some(name_str) = name_value.as_str() {
                                if name_str == "DataProcessingAgreement" {
                                    return "data_processing_agreement".to_string();
                                }
                            }
                        }
                    }
                }
                return "yaml_contract".to_string();
            }
            
            // Check for agreement field
            if mapping.contains_key(&serde_yaml::Value::String("agreement".to_string())) {
                return "yaml_agreement".to_string();
            }
        }

        // Default to contract schema
        "yaml_contract".to_string()
    }

    /// Validate YAML against specific CUE schema
    async fn validate_against_schema(&self, schema: &CueSchema, yaml_value: &serde_yaml::Value, errors: &mut Vec<ValidationError>, warnings: &mut Vec<ValidationWarning>) -> Result<()> {
        tracing::debug!("🔍 Validating against CUE schema: {}", schema.name);

        // Apply each constraint
        for constraint in &schema.constraints {
            self.apply_cue_constraint(constraint, yaml_value, errors, warnings);
        }

        // Perform schema-specific validation
        match schema.name.as_str() {
            "yaml_contract" => {
                self.validate_contract_schema(yaml_value, errors, warnings);
            },
            "yaml_agreement" => {
                self.validate_agreement_schema(yaml_value, errors, warnings);
            },
            "data_processing_agreement" => {
                self.validate_data_processing_schema(yaml_value, errors, warnings);
            },
            _ => {
                warnings.push(ValidationWarning {
                    code: "UNKNOWN_SCHEMA_TYPE".to_string(),
                    message: format!("Unknown schema type: {}", schema.name),
                    location: None,
                });
            }
        }

        Ok(())
    }

    /// Apply CUE constraint
    fn apply_cue_constraint(&self, constraint: &CueConstraint, yaml_value: &serde_yaml::Value, errors: &mut Vec<ValidationError>, _warnings: &mut Vec<ValidationWarning>) {
        match constraint.name.as_str() {
            "version_format" => {
                if let Some(mapping) = yaml_value.as_mapping() {
                    if let Some(contract_section) = mapping.get(&serde_yaml::Value::String("contract".to_string())) {
                        if let Some(contract_map) = contract_section.as_mapping() {
                            if let Some(version_value) = contract_map.get(&serde_yaml::Value::String("version".to_string())) {
                                if let Some(version_str) = version_value.as_str() {
                                    if !self.is_valid_semver(version_str) {
                                        errors.push(ValidationError {
                                            code: "INVALID_VERSION_FORMAT".to_string(),
                                            message: constraint.error_message.clone(),
                                            location: None,
                                            severity: SecuritySeverity::Medium,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "initial_state_required" => {
                if let Some(mapping) = yaml_value.as_mapping() {
                    if let Some(contract_section) = mapping.get(&serde_yaml::Value::String("contract".to_string())) {
                        if let Some(contract_map) = contract_section.as_mapping() {
                            if let Some(states_section) = contract_map.get(&serde_yaml::Value::String("states".to_string())) {
                                if let Some(states_map) = states_section.as_mapping() {
                                    let has_initial_state = states_map.iter().any(|(_, state_def)| {
                                        if let Some(state_map) = state_def.as_mapping() {
                                            if let Some(type_value) = state_map.get(&serde_yaml::Value::String("type".to_string())) {
                                                if let Some(type_str) = type_value.as_str() {
                                                    return type_str == "initial";
                                                }
                                            }
                                        }
                                        false
                                    });

                                    if !has_initial_state {
                                        errors.push(ValidationError {
                                            code: "MISSING_INITIAL_STATE".to_string(),
                                            message: constraint.error_message.clone(),
                                            location: None,
                                            severity: SecuritySeverity::High,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "minimum_parties" => {
                if let Some(mapping) = yaml_value.as_mapping() {
                    if let Some(agreement_section) = mapping.get(&serde_yaml::Value::String("agreement".to_string())) {
                        if let Some(agreement_map) = agreement_section.as_mapping() {
                            if let Some(parties_section) = agreement_map.get(&serde_yaml::Value::String("parties".to_string())) {
                                if let Some(parties_array) = parties_section.as_sequence() {
                                    if parties_array.len() < 2 {
                                        errors.push(ValidationError {
                                            code: "INSUFFICIENT_PARTIES".to_string(),
                                            message: constraint.error_message.clone(),
                                            location: None,
                                            severity: SecuritySeverity::Medium,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            },
            _ => {
                // Unknown constraint, skip
            }
        }
    }

    /// Validate contract schema
    fn validate_contract_schema(&self, yaml_value: &serde_yaml::Value, errors: &mut Vec<ValidationError>, warnings: &mut Vec<ValidationWarning>) {
        if let Some(mapping) = yaml_value.as_mapping() {
            if let Some(contract_section) = mapping.get(&serde_yaml::Value::String("contract".to_string())) {
                if let Some(contract_map) = contract_section.as_mapping() {
                    // Validate required fields
                    if !contract_map.contains_key(&serde_yaml::Value::String("name".to_string())) {
                        errors.push(ValidationError {
                            code: "MISSING_CONTRACT_NAME".to_string(),
                            message: "Contract must have a name field".to_string(),
                            location: None,
                            severity: SecuritySeverity::High,
                        });
                    }

                    // Validate states section
                    if let Some(states_section) = contract_map.get(&serde_yaml::Value::String("states".to_string())) {
                        if let Some(states_map) = states_section.as_mapping() {
                            if states_map.is_empty() {
                                errors.push(ValidationError {
                                    code: "EMPTY_STATES_SECTION".to_string(),
                                    message: "Contract states section cannot be empty".to_string(),
                                    location: None,
                                    severity: SecuritySeverity::High,
                                });
                            }
                        }
                    } else {
                        errors.push(ValidationError {
                            code: "MISSING_STATES_SECTION".to_string(),
                            message: "Contract must have a states section".to_string(),
                            location: None,
                            severity: SecuritySeverity::High,
                        });
                    }
                }
            }
        }
    }

    /// Validate agreement schema
    fn validate_agreement_schema(&self, yaml_value: &serde_yaml::Value, errors: &mut Vec<ValidationError>, _warnings: &mut Vec<ValidationWarning>) {
        if let Some(mapping) = yaml_value.as_mapping() {
            if let Some(agreement_section) = mapping.get(&serde_yaml::Value::String("agreement".to_string())) {
                if let Some(agreement_map) = agreement_section.as_mapping() {
                    // Validate parties
                    if let Some(parties_section) = agreement_map.get(&serde_yaml::Value::String("parties".to_string())) {
                        if let Some(parties_array) = parties_section.as_sequence() {
                            for (index, party) in parties_array.iter().enumerate() {
                                if let Some(party_map) = party.as_mapping() {
                                    if !party_map.contains_key(&serde_yaml::Value::String("name".to_string())) {
                                        errors.push(ValidationError {
                                            code: "MISSING_PARTY_NAME".to_string(),
                                            message: format!("Party at index {} must have a name", index),
                                            location: None,
                                            severity: SecuritySeverity::Medium,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Validate data processing agreement schema
    fn validate_data_processing_schema(&self, yaml_value: &serde_yaml::Value, errors: &mut Vec<ValidationError>, warnings: &mut Vec<ValidationWarning>) {
        if let Some(mapping) = yaml_value.as_mapping() {
            if let Some(contract_section) = mapping.get(&serde_yaml::Value::String("contract".to_string())) {
                if let Some(contract_map) = contract_section.as_mapping() {
                    // Check for GDPR compliance requirements
                    if let Some(data_categories) = contract_map.get(&serde_yaml::Value::String("data_categories".to_string())) {
                        if let Some(categories_array) = data_categories.as_sequence() {
                            let has_sensitive_data = categories_array.iter().any(|cat| {
                                if let Some(cat_map) = cat.as_mapping() {
                                    if let Some(sensitivity) = cat_map.get(&serde_yaml::Value::String("sensitivity".to_string())) {
                                        if let Some(sensitivity_str) = sensitivity.as_str() {
                                            return sensitivity_str == "high" || sensitivity_str == "critical";
                                        }
                                    }
                                }
                                false
                            });

                            if has_sensitive_data {
                                // Check for adequate security measures
                                if let Some(security_measures) = contract_map.get(&serde_yaml::Value::String("security_measures".to_string())) {
                                    if let Some(measures_array) = security_measures.as_sequence() {
                                        if measures_array.len() < 3 {
                                            warnings.push(ValidationWarning {
                                                code: "INSUFFICIENT_SECURITY_MEASURES".to_string(),
                                                message: "High/critical sensitivity data should have at least 3 security measures".to_string(),
                                                location: None,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Check if version string is valid semantic version
    fn is_valid_semver(&self, version: &str) -> bool {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return false;
        }

        for part in parts {
            if part.parse::<u32>().is_err() {
                return false;
            }
        }

        true
    }

    /// Add CUE schema
    pub fn add_schema(&mut self, schema: CueSchema) {
        self.schemas.insert(schema.name.clone(), schema);
    }

    /// Get schema by name
    pub fn get_schema(&self, name: &str) -> Option<&CueSchema> {
        self.schemas.get(name)
    }

    /// Clear validation cache
    pub async fn clear_cache(&self) {
        let mut cache = self.validation_cache.write().await;
        cache.clear();
    }

    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> (usize, usize) {
        let cache = self.validation_cache.read().await;
        (cache.len(), cache.capacity())
    }
}
