use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::RegisteredNotary;

/// Legal Compliance Engine
#[derive(Debug)]
pub struct LegalComplianceEngine {
    rules: Vec<ComplianceRule>,
    jurisdiction_requirements: HashMap<String, Vec<String>>,
}

/// Compliance Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub jurisdictions: Vec<String>,
    pub requirements: Vec<String>,
}

impl LegalComplianceEngine {
    /// Create a new compliance engine
    pub fn new() -> Self {
        Self {
            rules: Self::default_compliance_rules(),
            jurisdiction_requirements: Self::default_jurisdiction_requirements(),
        }
    }

    /// Default compliance rules
    fn default_compliance_rules() -> Vec<ComplianceRule> {
        vec![
            ComplianceRule {
                id: "kyc_rule".to_string(),
                name: "Know Your Customer".to_string(),
                description: "Identity verification requirements".to_string(),
                jurisdictions: vec!["US".to_string(), "EU".to_string()],
                requirements: vec!["Government ID".to_string(), "Address verification".to_string()],
            },
            ComplianceRule {
                id: "aml_rule".to_string(),
                name: "Anti-Money Laundering".to_string(),
                description: "AML compliance requirements".to_string(),
                jurisdictions: vec!["US".to_string(), "EU".to_string(), "International".to_string()],
                requirements: vec!["Transaction monitoring".to_string(), "Suspicious activity reporting".to_string()],
            },
            ComplianceRule {
                id: "gdpr_rule".to_string(),
                name: "General Data Protection Regulation".to_string(),
                description: "EU data protection requirements".to_string(),
                jurisdictions: vec!["EU".to_string()],
                requirements: vec!["Data consent".to_string(), "Right to erasure".to_string()],
            },
        ]
    }

    /// Default jurisdiction requirements
    fn default_jurisdiction_requirements() -> HashMap<String, Vec<String>> {
        let mut requirements = HashMap::new();
        requirements.insert("US".to_string(), vec![
            "Notary license".to_string(), 
            "Bond".to_string(),
            "Background check".to_string(),
        ]);
        requirements.insert("EU".to_string(), vec![
            "Qualified certificate".to_string(), 
            "GDPR compliance".to_string(),
            "eIDAS regulation".to_string(),
        ]);
        requirements.insert("International".to_string(), vec![
            "Digital certificate".to_string(),
            "Identity verification".to_string(),
        ]);
        requirements
    }

    /// Check compliance for a notary in a jurisdiction
    pub fn check_compliance(&self, jurisdiction: &str, notary: &RegisteredNotary) -> bool {
        // Check if jurisdiction has requirements
        if !self.jurisdiction_requirements.contains_key(jurisdiction) {
            return false;
        }

        // Check if notary credentials are verified
        if !notary.credentials.verified {
            return false;
        }

        // Check if notary is in active status
        if notary.status != crate::NotaryStatus::Active {
            return false;
        }

        // Additional compliance checks would go here
        // For now, basic checks pass
        true
    }

    /// Get compliance requirements for jurisdiction
    pub fn get_jurisdiction_requirements(&self, jurisdiction: &str) -> Option<&Vec<String>> {
        self.jurisdiction_requirements.get(jurisdiction)
    }

    /// Get all compliance rules
    pub fn get_compliance_rules(&self) -> &Vec<ComplianceRule> {
        &self.rules
    }

    /// Add new compliance rule
    pub fn add_compliance_rule(&mut self, rule: ComplianceRule) {
        self.rules.push(rule);
    }

    /// Check if rule applies to jurisdiction
    pub fn rule_applies_to_jurisdiction(&self, rule_id: &str, jurisdiction: &str) -> bool {
        self.rules.iter()
            .find(|r| r.id == rule_id)
            .map(|r| r.jurisdictions.contains(&jurisdiction.to_string()))
            .unwrap_or(false)
    }
}
