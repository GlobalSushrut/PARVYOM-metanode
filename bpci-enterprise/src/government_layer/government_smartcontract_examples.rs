//! Government SmartContract++ Examples
//! 
//! YAML-based SmartContract++ examples for any jurisdiction worldwide:
//! - Generic National Template: Financial compliance, digital currency, tax automation
//! - Generic State/Province Template: Regional tax, local compliance, development
//! - Custom Template Builder: For any country, state, province, or jurisdiction
//! 
//! Examples include India, China, Karnataka as demonstrations, but the system
//! supports any jurisdiction globally with customizable templates.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Government SmartContract++ Examples
pub struct GovernmentSmartContractExamples;

impl GovernmentSmartContractExamples {
    /// Generic National Government SmartContract++ Template
    /// Can be customized for any country worldwide (US, UK, Germany, Japan, Brazil, etc.)
    pub fn generic_national_template() -> String {
        r#"
# Generic National Government SmartContract++ Template
# Customizable for any country worldwide
contract:
  name: "{{COUNTRY_NAME}}GovernmentContract"  # e.g., "USAGovernmentContract", "GermanyGovernmentContract"
  jurisdiction: "{{ISO_CODE}}"  # ISO 3166-1 alpha-2 (US, UK, DE, JP, BR, etc.)
  authority_level: "National"
  government_entity: "{{GOVERNMENT_NAME}}"  # e.g., "United States Government", "Government of Germany"
  
  state:
    digital_currency_accounts:
      type: "mapping"
      key_type: "string"
      value_type:
        account_id: "string"
        citizen_id: "string"  # SSN, National ID, etc.
        balance: "decimal"
        kyc_status: "enum[pending,verified,rejected]"
        compliance_score: "uint256"
    
    tax_registrations:
      type: "mapping"
      key_type: "string"
      value_type:
        tax_id: "string"  # TIN, VAT, etc.
        entity_name: "string"
        compliance_status: "enum[compliant,non_compliant,under_review]"
        tax_liability: "decimal"
  
  functions:
    issue_digital_currency:
      visibility: "government_only"
      authority_required: "CentralBank"  # Fed, ECB, BoJ, etc.
      params:
        account_id: "string"
        amount: "decimal"
        citizen_id: "string"
      returns: "bool"
    
    collect_taxes:
      visibility: "government_only"
      authority_required: "TaxAuthority"  # IRS, HMRC, etc.
      params:
        tax_id: "string"
        amount: "decimal"
        tax_type: "string"
      returns: "bool"
    
    verify_compliance:
      visibility: "public"
      params:
        entity_id: "string"
      returns: "bool"
    
    emergency_freeze:
      visibility: "government_only"
      authority_required: "EmergencyAuthority"
      params:
        account_id: "string"
        reason: "string"
      returns: "bool"

  regulatory_framework:
    privacy_protection: "{{PRIVACY_LAW}}"  # GDPR, CCPA, etc.
    financial_regulation: "{{FINANCIAL_LAW}}"  # SOX, Basel III, etc.
    data_sovereignty: "{{DATA_LAW}}"  # Local data protection laws
    emergency_powers: "{{EMERGENCY_LAW}}"  # National emergency legislation
    
  compliance_requirements:
    audit_frequency: "quarterly"
    reporting_standards: "{{ACCOUNTING_STANDARD}}"  # GAAP, IFRS, etc.
    transparency_level: "high"
    citizen_rights_protection: "mandatory"
"#.to_string()
    }
    
    /// Universal Government SmartContract++ Template
    /// Can be customized for any jurisdiction worldwide
    /// Universal Template Generator - Works for ANY jurisdiction worldwide
    /// This template can be customized for any country, state, province, city, or region
    pub fn universal_template() -> String {
        r#"
# Universal Government SmartContract++ Template
# Customizable for ANY jurisdiction worldwide
# 
# SUPPORTED JURISDICTIONS:
# - Countries: All 195 UN member states + territories
# - States/Provinces: US states, Canadian provinces, German länder, etc.
# - Regions: EU regions, Chinese provinces, Indian states, etc.
# - Cities: Major metropolitan areas worldwide
# - Special Zones: Economic zones, autonomous regions, etc.
#
# USAGE EXAMPLES:
# - United States: jurisdiction: "US", authority_level: "National"
# - California: jurisdiction: "US-CA", authority_level: "State"
# - Ontario: jurisdiction: "CA-ON", authority_level: "Province"
# - Bavaria: jurisdiction: "DE-BY", authority_level: "State"
# - Tokyo: jurisdiction: "JP-13", authority_level: "Prefecture"
# - Hong Kong: jurisdiction: "HK", authority_level: "Special Administrative Region"
# - Singapore: jurisdiction: "SG", authority_level: "City-State"
# - European Union: jurisdiction: "EU", authority_level: "Supranational"

contract:
  name: "{{JURISDICTION_NAME}}GovernmentContract"
  jurisdiction: "{{ISO_CODE}}"  # ISO 3166-1 (countries) or ISO 3166-2 (subdivisions)
  authority_level: "{{AUTHORITY_LEVEL}}"  # National, State, Province, Region, City, Special Zone
  government_entity: "{{GOVERNMENT_NAME}}"
  parent_jurisdiction: "{{PARENT_CODE}}"  # For subdivisions, parent country/region code
  
  # Flexible state schema - adapts to any government structure
  state:
    citizens_and_entities:
      type: "mapping"
      key_type: "string"
      value_type:
        entity_id: "string"  # National ID, SSN, Tax ID, etc.
        entity_type: "enum[individual,business,organization,government]"
        registration_status: "enum[active,suspended,revoked]"
        compliance_score: "uint256"
        jurisdiction_specific_data: "json"  # Flexible for local requirements
    
    financial_accounts:
      type: "mapping"
      key_type: "string"
      value_type:
        account_id: "string"
        account_type: "string"  # Savings, business, government, etc.
        balance: "decimal"
        currency: "string"  # USD, EUR, JPY, CNY, etc.
        regulatory_status: "enum[compliant,under_review,frozen]"
    
    regulatory_obligations:
      type: "mapping"
      key_type: "string"
      value_type:
        obligation_id: "string"
        obligation_type: "string"  # Tax, license, permit, etc.
        amount_due: "decimal"
        due_date: "timestamp"
        status: "enum[pending,paid,overdue,waived]"
    
    government_services:
      type: "mapping"
      key_type: "string"
      value_type:
        service_id: "string"
        service_type: "string"  # Healthcare, education, infrastructure, etc.
        budget_allocated: "decimal"
        performance_metrics: "json"
        citizen_satisfaction: "uint256"
  
  # Universal government functions - applicable to any jurisdiction
  functions:
    register_entity:
      visibility: "public"
      params:
        entity_data: "json"  # Flexible registration data
        entity_type: "string"
        supporting_documents: "string[]"
      returns: "string"  # entity_id
    
    collect_obligation:
      visibility: "government_only"
      authority_required: "{{COLLECTION_AUTHORITY}}"  # IRS, HMRC, local tax office, etc.
      params:
        entity_id: "string"
        obligation_type: "string"
        amount: "decimal"
      returns: "bool"
    
    provide_service:
      visibility: "government_only"
      authority_required: "{{SERVICE_AUTHORITY}}"
      params:
        citizen_id: "string"
        service_type: "string"
        service_details: "json"
      returns: "string"  # service_record_id
    
    verify_compliance:
      visibility: "public"
      params:
        entity_id: "string"
        compliance_type: "string"
      returns:
        is_compliant: "bool"
        compliance_score: "uint256"
        last_updated: "timestamp"
    
    emergency_action:
      visibility: "government_only"
      authority_required: "{{EMERGENCY_AUTHORITY}}"
      params:
        target_entity: "string"
        action_type: "string"  # freeze, investigate, suspend, etc.
        justification: "string"
        duration: "uint256"  # in seconds
      returns: "string"  # action_id
    
    cross_jurisdiction_coordination:
      visibility: "government_only"
      authority_required: "{{COORDINATION_AUTHORITY}}"
      params:
        partner_jurisdiction: "string"
        coordination_type: "string"  # data_sharing, enforcement, mutual_aid
        request_details: "json"
      returns: "string"  # coordination_id

  # Flexible regulatory framework - adapts to local laws
  regulatory_framework:
    privacy_protection: "{{PRIVACY_LAW}}"  # GDPR, CCPA, PIPEDA, LGPD, etc.
    financial_regulation: "{{FINANCIAL_LAW}}"  # Local banking/finance laws
    data_sovereignty: "{{DATA_LAW}}"  # Local data protection requirements
    emergency_powers: "{{EMERGENCY_LAW}}"  # Emergency legislation
    business_regulation: "{{BUSINESS_LAW}}"  # Corporate/business laws
    labor_standards: "{{LABOR_LAW}}"  # Employment/labor regulations
    environmental_protection: "{{ENVIRONMENTAL_LAW}}"  # Environmental regulations
    healthcare_regulation: "{{HEALTHCARE_LAW}}"  # Health system regulations
    education_standards: "{{EDUCATION_LAW}}"  # Education system rules
    cultural_protection: "{{CULTURAL_LAW}}"  # Cultural heritage laws
    
  # Universal compliance requirements
  compliance_requirements:
    audit_frequency: "{{AUDIT_FREQUENCY}}"  # daily, weekly, monthly, quarterly, annual
    reporting_standards: "{{ACCOUNTING_STANDARD}}"  # GAAP, IFRS, local standards
    transparency_level: "{{TRANSPARENCY_LEVEL}}"  # high, medium, low
    citizen_rights_protection: "mandatory"
    data_retention_period: "{{RETENTION_PERIOD}}"  # Based on local laws
    cross_border_cooperation: "{{COOPERATION_LEVEL}}"  # full, limited, restricted
    
  # Multi-language and cultural support
  localization:
    primary_language: "{{PRIMARY_LANGUAGE}}"  # en, zh, es, fr, ar, etc.
    supported_languages: "{{LANGUAGE_LIST}}"  # Multiple language support
    cultural_considerations: "{{CULTURAL_NOTES}}"  # Local cultural requirements
    timezone: "{{TIMEZONE}}"  # Local timezone for operations
    currency: "{{LOCAL_CURRENCY}}"  # Local currency code
    
  # Integration with international standards
  international_compliance:
    un_conventions: "{{UN_TREATIES}}"  # UN treaties and conventions
    trade_agreements: "{{TRADE_AGREEMENTS}}"  # WTO, bilateral agreements, etc.
    tax_treaties: "{{TAX_TREATIES}}"  # Double taxation agreements
    mutual_legal_assistance: "{{MLA_TREATIES}}"  # Legal cooperation treaties
    extradition_treaties: "{{EXTRADITION_TREATIES}}"  # Extradition agreements
"#.to_string()
    }
    
    /// Legacy custom template (deprecated - use universal_template instead)
    pub fn custom_template(
        jurisdiction_name: &str,
        jurisdiction_code: &str,
        authority_level: &str,
        government_entity: &str,
        regulatory_framework: HashMap<String, String>,
    ) -> String {
        let template = format!(
            r#"
# Custom Government SmartContract++ for {}
contract:
  name: "{}GovernmentContract"
  jurisdiction: "{}"
  authority_level: "{}"
  government_entity: "{}"
  
  state:
    accounts:
      type: "mapping"
      key_type: "string"
      value_type:
        account_id: "string"
        holder_id: "string"
        balance: "decimal"
        status: "enum[active,suspended,frozen]"
    
    registrations:
      type: "mapping"
      key_type: "string"
      value_type:
        registration_id: "string"
        entity_name: "string"
        compliance_status: "enum[compliant,non_compliant,under_review]"
        obligations: "decimal"
  
  functions:
    manage_accounts:
      visibility: "government_only"
      authority_required: "FinancialAuthority"
      params:
        account_id: "string"
        action: "string"
      returns: "bool"
    
    verify_compliance:
      visibility: "public"
      params:
        entity_id: "string"
      returns: "bool"
    
    emergency_action:
      visibility: "government_only"
      authority_required: "EmergencyAuthority"
      params:
        target_id: "string"
        action: "string"
        reason: "string"
      returns: "bool"

  regulatory_framework:
{}    
  compliance_requirements:
    audit_frequency: "quarterly"
    transparency_level: "high"
    rights_protection: "mandatory"
"#,
            jurisdiction_name,
            jurisdiction_name.replace(" ", ""),
            jurisdiction_code,
            authority_level,
            government_entity,
            regulatory_framework
                .iter()
                .map(|(k, v)| format!("    {}: \"{}\"", k, v))
                .collect::<Vec<_>>()
                .join("\n")
        );
        template
    }
    
    /// Example: India National Government SmartContract++ (Template for any national government)
    pub fn india_national_contract() -> String {
        r#"
# Example: India National Government SmartContract++ (Template for any national government)
contract:
  name: "IndiaGovernmentContract"
  jurisdiction: "IN"  # ISO 3166-1 alpha-2 country code (any country)
  authority_level: "National"
  government_entity: "Government of India"  # Customizable for any national government
  
  state:
    digital_rupee_accounts:
      type: "mapping"
      key_type: "string"
      value_type:
        account_id: "string"
        holder_aadhaar: "string"
        balance: "decimal"
        kyc_status: "enum[pending,verified,rejected]"
        compliance_score: "uint256"
    
    gst_registrations:
      type: "mapping"
      key_type: "string"
      value_type:
        gstin: "string"
        business_name: "string"
        compliance_status: "enum[compliant,non_compliant,under_review]"
        tax_liability: "decimal"
  
  functions:
    issue_digital_rupee:
      visibility: "government_only"
      authority_required: "RBI"
      params:
        account_id: "string"
        amount: "decimal"
        authorization_code: "string"
      returns:
        transaction_id: "string"
        compliance_check: "boolean"
      
    file_gst_return:
      visibility: "public"
      compliance_check: true
      params:
        gstin: "string"
        return_period: "string"
        tax_liability: "decimal"
      returns:
        filing_id: "string"
        compliance_score: "uint256"
  
  compliance_rules:
    digital_rupee_limits:
      individual_daily_limit: "50000"
      business_daily_limit: "500000"
    gst_thresholds:
      registration_threshold: "2000000"
      composition_scheme_limit: "15000000"
"#.to_string()
    }
    
    /// Example: China National Government SmartContract++ (Template for any national government)
    /// Note: This is just an example - the system supports ANY country worldwide
    pub fn china_national_contract() -> String {
        r#"
# Example: China National Government SmartContract++ (Template for any national government)
# Note: This is just an example - the system supports ANY country worldwide
contract:
  name: "ChinaGovernmentContract"
  jurisdiction: "CN"
  authority_level: "National"
  government_entity: "People's Republic of China"
  
  state:
    digital_yuan_accounts:
      type: "mapping"
      key_type: "string"
      value_type:
        account_id: "string"
        citizen_id: "string"
        balance: "decimal"
        social_credit_score: "uint256"
        compliance_level: "enum[excellent,good,fair,poor]"
    
    social_credit_records:
      type: "mapping"
      key_type: "string"
      value_type:
        citizen_id: "string"
        current_score: "uint256"
        positive_behaviors: "array[behavior]"
        negative_behaviors: "array[behavior]"
        restrictions: "array[restriction]"
  
  functions:
    issue_digital_yuan:
      visibility: "government_only"
      authority_required: "PBOC"
      params:
        account_id: "string"
        amount: "decimal"
        policy_alignment: "string"
      returns:
        transaction_id: "string"
        social_credit_impact: "int256"
      
    update_social_credit_score:
      visibility: "government_only"
      authority_required: "SOCIAL_CREDIT_BUREAU"
      params:
        citizen_id: "string"
        behavior_type: "string"
        score_impact: "int256"
      returns:
        new_score: "uint256"
        restrictions_applied: "array[string]"
  
  compliance_rules:
    social_credit_thresholds:
      excellent_threshold: "950"
      good_threshold: "700"
      restrictions_threshold: "300"
    digital_yuan_controls:
      daily_transaction_limit: "100000"
      social_credit_requirements: "500"
"#.to_string()
    }
    
    /// Example: Karnataka State Government SmartContract++ (Template for any state/province)
    /// Note: This is just an example - the system supports ANY state/province worldwide
    pub fn karnataka_state_contract() -> String {
        r#"
# Example: Karnataka State Government SmartContract++ (Template for any state/province)
# Note: This is just an example - the system supports ANY state/province worldwide
contract:
  name: "KarnatakaStateContract"
  jurisdiction: "IN-KA"
  authority_level: "State"
  government_entity: "Government of Karnataka"
  parent_authority: "Government of India"
  
  state:
    state_tax_registrations:
      type: "mapping"
      key_type: "string"
      value_type:
        registration_id: "string"
        entity_name: "string"
        state_tax_number: "string"
        compliance_status: "enum[compliant,defaulter,under_scrutiny]"
    
    it_companies:
      type: "mapping"
      key_type: "string"
      value_type:
        company_id: "string"
        company_name: "string"
        sez_location: "string"
        employee_count: "uint256"
        export_revenue: "decimal"
        compliance_rating: "enum[A,B,C,D]"
  
  functions:
    register_state_taxpayer:
      visibility: "government_only"
      authority_required: "COMMERCIAL_TAX_DEPARTMENT"
      params:
        entity_details: "object"
        business_type: "string"
        expected_turnover: "decimal"
      returns:
        registration_id: "string"
        state_tax_number: "string"
    
    register_it_company:
      visibility: "government_only"
      authority_required: "ITBT_DEPARTMENT"
      params:
        company_details: "object"
        investment_commitment: "decimal"
        employment_projection: "uint256"
      returns:
        company_id: "string"
        incentives_eligible: "array[string]"
  
  compliance_rules:
    state_tax_compliance:
      registration_threshold: "500000"
      filing_frequency: "monthly"
    it_sector_incentives:
      minimum_investment: "10000000"
      minimum_employment: "50"
      export_obligation: "75_percent"
"#.to_string()
    }
    
    /// Get all government contract examples
    pub fn get_all_examples() -> HashMap<String, String> {
        let mut examples = HashMap::new();
        examples.insert("india_national".to_string(), Self::india_national_contract());
        examples.insert("china_national".to_string(), Self::china_national_contract());
        examples.insert("karnataka_state".to_string(), Self::karnataka_state_contract());
        examples
    }

    /// Generic State/Province Government SmartContract++ Template
    /// Can be customized for any state, province, or subdivision worldwide
    pub fn generic_state_template() -> String {
        r#"
# Generic State/Province Government SmartContract++ Template
# Customizable for any state, province, or subdivision worldwide
contract:
  name: "{{STATE_NAME}}StateContract"  # e.g., "CaliforniaStateContract", "OntarioStateContract"
  jurisdiction: "{{STATE_CODE}}"  # ISO 3166-2 (US-CA, CA-ON, DE-BY, etc.)
  authority_level: "State"
  government_entity: "{{STATE_GOVERNMENT_NAME}}"  # e.g., "State of California", "Province of Ontario"
  
  state:
    regional_tax_accounts:
      type: "mapping"
      key_type: "string"
      value_type:
        account_id: "string"
        business_id: "string"
        regional_tax_balance: "decimal"
        compliance_status: "enum[compliant,non_compliant,pending]"
    
    local_development_projects:
      type: "mapping"
      key_type: "string"
      value_type:
        project_id: "string"
        project_name: "string"
        funding_amount: "decimal"
        status: "enum[proposed,approved,in_progress,completed]"
  
  functions:
    register_regional_business:
      parameters:
        - name: "business_id"
          type: "string"
        - name: "business_name"
          type: "string"
        - name: "tax_category"
          type: "string"
      returns: "bool"
      
    collect_regional_tax:
      parameters:
        - name: "business_id"
          type: "string"
        - name: "tax_amount"
          type: "decimal"
      returns: "string"
      
    approve_development_project:
      parameters:
        - name: "project_id"
          type: "string"
        - name: "funding_amount"
          type: "decimal"
      returns: "bool"
"#.to_string()
    }

    /// Get worldwide government contract examples
    pub fn get_worldwide_examples() -> HashMap<String, String> {
        let mut examples = HashMap::new();
        
        // National examples
        examples.insert("usa_national".to_string(), Self::generic_national_template().replace("{{COUNTRY_NAME}}", "USA").replace("{{ISO_CODE}}", "US").replace("{{GOVERNMENT_NAME}}", "United States Government"));
        examples.insert("germany_national".to_string(), Self::generic_national_template().replace("{{COUNTRY_NAME}}", "Germany").replace("{{ISO_CODE}}", "DE").replace("{{GOVERNMENT_NAME}}", "Government of Germany"));
        examples.insert("japan_national".to_string(), Self::generic_national_template().replace("{{COUNTRY_NAME}}", "Japan").replace("{{ISO_CODE}}", "JP").replace("{{GOVERNMENT_NAME}}", "Government of Japan"));
        examples.insert("brazil_national".to_string(), Self::generic_national_template().replace("{{COUNTRY_NAME}}", "Brazil").replace("{{ISO_CODE}}", "BR").replace("{{GOVERNMENT_NAME}}", "Government of Brazil"));
        examples.insert("india_national".to_string(), Self::india_national_contract());
        examples.insert("china_national".to_string(), Self::china_national_contract());
        
        // State/Province examples
        examples.insert("california_state".to_string(), Self::generic_state_template().replace("{{STATE_NAME}}", "California").replace("{{STATE_CODE}}", "US-CA").replace("{{STATE_GOVERNMENT_NAME}}", "State of California"));
        examples.insert("ontario_province".to_string(), Self::generic_state_template().replace("{{STATE_NAME}}", "Ontario").replace("{{STATE_CODE}}", "CA-ON").replace("{{STATE_GOVERNMENT_NAME}}", "Province of Ontario"));
        examples.insert("bavaria_state".to_string(), Self::generic_state_template().replace("{{STATE_NAME}}", "Bavaria").replace("{{STATE_CODE}}", "DE-BY").replace("{{STATE_GOVERNMENT_NAME}}", "Free State of Bavaria"));
        examples.insert("karnataka_state".to_string(), Self::karnataka_state_contract());
        
        examples
    }

    /// Build custom template for any jurisdiction
    pub fn build_custom_template(
        name: &str,
        code: &str,
        level: &str,
        entity: &str,
        regulatory_framework: &str,
    ) -> String {
        let template = match level {
            "National" => Self::generic_national_template(),
            "State" | "Province" => Self::generic_state_template(),
            _ => Self::universal_template(),
        };
        
        template
            .replace("{{COUNTRY_NAME}}", name)
            .replace("{{STATE_NAME}}", name)
            .replace("{{ISO_CODE}}", code)
            .replace("{{STATE_CODE}}", code)
            .replace("{{GOVERNMENT_NAME}}", entity)
            .replace("{{STATE_GOVERNMENT_NAME}}", entity)
            .replace("# Regulatory Framework", &format!("# Regulatory Framework: {}", regulatory_framework))
    }

    /// Get template by jurisdiction code
    pub fn get_template_by_jurisdiction(jurisdiction_code: &str) -> Option<String> {
        match jurisdiction_code {
            "US" => Some(Self::generic_national_template().replace("{{COUNTRY_NAME}}", "USA").replace("{{ISO_CODE}}", "US").replace("{{GOVERNMENT_NAME}}", "United States Government")),
            "DE" => Some(Self::generic_national_template().replace("{{COUNTRY_NAME}}", "Germany").replace("{{ISO_CODE}}", "DE").replace("{{GOVERNMENT_NAME}}", "Government of Germany")),
            "JP" => Some(Self::generic_national_template().replace("{{COUNTRY_NAME}}", "Japan").replace("{{ISO_CODE}}", "JP").replace("{{GOVERNMENT_NAME}}", "Government of Japan")),
            "BR" => Some(Self::generic_national_template().replace("{{COUNTRY_NAME}}", "Brazil").replace("{{ISO_CODE}}", "BR").replace("{{GOVERNMENT_NAME}}", "Government of Brazil")),
            "IN" => Some(Self::india_national_contract()),
            "CN" => Some(Self::china_national_contract()),
            "US-CA" => Some(Self::generic_state_template().replace("{{STATE_NAME}}", "California").replace("{{STATE_CODE}}", "US-CA").replace("{{STATE_GOVERNMENT_NAME}}", "State of California")),
            "CA-ON" => Some(Self::generic_state_template().replace("{{STATE_NAME}}", "Ontario").replace("{{STATE_CODE}}", "CA-ON").replace("{{STATE_GOVERNMENT_NAME}}", "Province of Ontario")),
            "IN-KA" => Some(Self::karnataka_state_contract()),
            _ => Some(Self::universal_template()),
        }
    }

    /// List all supported jurisdictions
    pub fn list_supported_jurisdictions() -> Vec<String> {
        vec![
            // National jurisdictions
            "US".to_string(),
            "DE".to_string(), 
            "JP".to_string(),
            "BR".to_string(),
            "IN".to_string(),
            "CN".to_string(),
            "UK".to_string(),
            "FR".to_string(),
            "AU".to_string(),
            "CA".to_string(),
            // State/Province jurisdictions
            "US-CA".to_string(),  // California
            "US-NY".to_string(),  // New York
            "US-TX".to_string(),  // Texas
            "CA-ON".to_string(),  // Ontario
            "CA-BC".to_string(),  // British Columbia
            "DE-BY".to_string(),  // Bavaria
            "DE-NW".to_string(),  // North Rhine-Westphalia
            "IN-KA".to_string(),  // Karnataka
            "IN-MH".to_string(),  // Maharashtra
            "AU-NSW".to_string(), // New South Wales
            "AU-VIC".to_string(), // Victoria
        ]
    }

    /// Get all templates (alias for get_all_examples for backward compatibility)
    pub fn get_all_templates() -> HashMap<String, String> {
        Self::get_all_examples()
    }
}
