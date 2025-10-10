// Compliance Validator
// Validates transactions against regulatory frameworks

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing;

use super::GovernmentConfig;

/// Compliance validator for regulatory frameworks
#[derive(Debug)]
pub struct ComplianceValidator {
    config: GovernmentConfig,
    compliance_rules: Arc<RwLock<HashMap<String, Vec<ComplianceRule>>>>,
    regulatory_frameworks: Arc<RwLock<HashMap<String, RegulatoryFramework>>>,
    validator_state: Arc<RwLock<ValidatorState>>,
}

/// Compliance rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRule {
    pub rule_id: String,
    pub rule_name: String,
    pub jurisdiction: String,
    pub framework: String,
    pub rule_type: RuleType,
    pub severity: RuleSeverity,
    pub enabled: bool,
}

/// Types of compliance rules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RuleType {
    DataProtection,
    FinancialReporting,
    AntiMoneyLaundering,
    KnowYourCustomer,
    TransactionLimits,
}

/// Rule severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleSeverity {
    Info,
    Warning,
    Error,
    Critical,
    Blocking,
}

/// Regulatory framework definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryFramework {
    pub framework_id: String,
    pub framework_name: String,
    pub jurisdiction: String,
    pub version: String,
    pub enforcement_level: EnforcementLevel,
}

/// Enforcement levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnforcementLevel {
    Advisory,
    Mandatory,
    Strict,
    Critical,
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub validation_id: String,
    pub transaction_id: String,
    pub is_compliant: bool,
    pub compliance_score: f64,
    pub violations: Vec<ComplianceViolation>,
    pub validated_frameworks: Vec<String>,
    pub validation_timestamp: u64,
    pub violation_details: Option<String>,
}

/// Compliance violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    pub violation_id: String,
    pub rule_id: String,
    pub rule_name: String,
    pub severity: RuleSeverity,
    pub violation_message: String,
    pub field_path: Option<String>,
}

/// Validator state tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorState {
    pub validator_id: String,
    pub total_validations: u64,
    pub compliant_validations: u64,
    pub non_compliant_validations: u64,
    pub active_frameworks: u32,
}

impl ComplianceValidator {
    /// Create a new compliance validator
    pub async fn new(config: GovernmentConfig) -> Result<Self> {
        let validator_id = Uuid::new_v4().to_string();
        let validator_state = ValidatorState {
            validator_id,
            total_validations: 0,
            compliant_validations: 0,
            non_compliant_validations: 0,
            active_frameworks: 0,
        };

        Ok(Self {
            config,
            compliance_rules: Arc::new(RwLock::new(HashMap::new())),
            regulatory_frameworks: Arc::new(RwLock::new(HashMap::new())),
            validator_state: Arc::new(RwLock::new(validator_state)),
        })
    }

    /// Initialize the compliance validator
    pub async fn initialize(&self) -> Result<()> {
        tracing::info!("📋 Initializing Compliance Validator...");
        self.load_default_rules().await?;
        tracing::info!("✅ Compliance Validator initialized successfully");
        Ok(())
    }

    /// Load default compliance rules
    async fn load_default_rules(&self) -> Result<()> {
        let mut rules = self.compliance_rules.write().unwrap();
        
        // US Rules
        rules.insert("US".to_string(), vec![
            ComplianceRule {
                rule_id: "us_aml_limit".to_string(),
                rule_name: "AML Transaction Limit".to_string(),
                jurisdiction: "US".to_string(),
                framework: "aml".to_string(),
                rule_type: RuleType::AntiMoneyLaundering,
                severity: RuleSeverity::Blocking,
                enabled: true,
            }
        ]);

        // EU Rules
        rules.insert("EU".to_string(), vec![
            ComplianceRule {
                rule_id: "eu_gdpr_consent".to_string(),
                rule_name: "GDPR Consent Required".to_string(),
                jurisdiction: "EU".to_string(),
                framework: "gdpr".to_string(),
                rule_type: RuleType::DataProtection,
                severity: RuleSeverity::Critical,
                enabled: true,
            }
        ]);

        Ok(())
    }

    /// Validate transaction against compliance rules
    pub async fn validate_transaction(
        &self,
        transaction_data: &serde_json::Value,
        jurisdiction: &str,
    ) -> Result<ValidationResult> {
        let validation_id = Uuid::new_v4().to_string();
        let transaction_id = transaction_data.get("transaction_id")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        // Get applicable rules
        let rules = self.compliance_rules.read().unwrap();
        let applicable_rules = rules.get(jurisdiction).cloned().unwrap_or_default();

        let mut violations = Vec::new();

        // Basic validation logic
        for rule in &applicable_rules {
            if let Some(amount) = transaction_data.get("amount").and_then(|v| v.as_f64()) {
                if rule.rule_type == RuleType::AntiMoneyLaundering && amount > 10000.0 {
                    violations.push(ComplianceViolation {
                        violation_id: Uuid::new_v4().to_string(),
                        rule_id: rule.rule_id.clone(),
                        rule_name: rule.rule_name.clone(),
                        severity: rule.severity.clone(),
                        violation_message: "Transaction exceeds AML limit".to_string(),
                        field_path: Some("amount".to_string()),
                    });
                }
            }
        }

        let is_compliant = violations.is_empty();
        let compliance_score = if is_compliant { 1.0 } else { 0.5 };

        // Update metrics
        {
            let mut state = self.validator_state.write().unwrap();
            state.total_validations += 1;
            if is_compliant {
                state.compliant_validations += 1;
            } else {
                state.non_compliant_validations += 1;
            }
        }

        Ok(ValidationResult {
            validation_id,
            transaction_id,
            is_compliant,
            compliance_score,
            violations: violations.clone(),
            validated_frameworks: vec![jurisdiction.to_string()],
            validation_timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            violation_details: if violations.is_empty() { 
                None 
            } else { 
                Some(violations.iter().map(|v| v.violation_message.clone()).collect::<Vec<_>>().join("; "))
            },
        })
    }

    /// Get validator statistics
    pub async fn get_validator_statistics(&self) -> Result<ValidatorState> {
        let state = self.validator_state.read().unwrap();
        Ok(state.clone())
    }

    /// Shutdown compliance validator
    pub async fn shutdown(&self) -> Result<()> {
        tracing::info!("🔄 Shutting down Compliance Validator...");
        tracing::info!("✅ Compliance Validator shutdown complete");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_compliance_validator_creation() {
        let config = GovernmentConfig::default();
        let validator = ComplianceValidator::new(config).await.unwrap();
        assert!(validator.initialize().await.is_ok());
        assert!(validator.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_transaction_validation() {
        let config = GovernmentConfig::default();
        let validator = ComplianceValidator::new(config).await.unwrap();
        validator.initialize().await.unwrap();

        let transaction_data = serde_json::json!({
            "transaction_id": "test_123",
            "amount": 5000.0,
            "currency": "USD"
        });

        let result = validator.validate_transaction(&transaction_data, "US").await.unwrap();
        assert!(result.is_compliant);
        assert_eq!(result.compliance_score, 1.0);

        validator.shutdown().await.unwrap();
    }
}
