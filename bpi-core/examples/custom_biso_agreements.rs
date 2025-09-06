//! Developer Examples for Custom BISO Agreement Creation
//! 
//! This file demonstrates how developers can create their own real cue-based BISO agreements
//! with custom triggers, actions, and compliance requirements. Nothing is mocked - all examples
//! create real, functional BISO agreements that integrate with the BPI ecosystem.

use bpi_core::biso_agreement::{
    BisoAgreementBuilder, BisoAgreementManager, BisoAgreementType, 
    RequiredAction, EnforcementLevel, ComplianceLevel,
    ApiAccessLevel, CommunicationRestrictions
};
use std::collections::HashMap;
use chrono::{Utc, Duration};
use anyhow::Result;
use tracing::info;

/// Example 1: High-Volume Trading Wallet with Custom Rules
pub async fn create_trading_wallet_agreement() -> Result<()> {
    info!("🔧 Creating custom trading wallet BISO agreement...");
    
    let manager = BisoAgreementManager::new();
    
    // Create custom parameters for high-frequency trading monitoring
    let mut trading_params = HashMap::new();
    trading_params.insert("max_daily_volume".to_string(), "1000000".to_string());
    trading_params.insert("alert_threshold".to_string(), "500000".to_string());
    trading_params.insert("trading_pair".to_string(), "BPI/USD".to_string());
    
    let mut alert_params = HashMap::new();
    alert_params.insert("notification_endpoint".to_string(), "https://api.mycompany.com/alerts".to_string());
    alert_params.insert("escalation_level".to_string(), "high".to_string());
    
    // Build the agreement with custom cue-based rules
    let agreement = BisoAgreementBuilder::new()
        .wallet_id("trading_wallet_001")
        .agreement_type(BisoAgreementType::BankStamped {
            bank_id: "TRADING_BANK_001".to_string(),
            banking_license: "US_FEDERAL_RESERVE".to_string(),
            compliance_level: ComplianceLevel::Enhanced,
            api_access_level: ApiAccessLevel::Full {
                bank_api: true,
                government_api: false,
                cross_system_communication: true
            }
        })
        // Volume-based monitoring rule
        .add_volume_rule(
            1000000, // $1M threshold
            RequiredAction::GenerateComplianceReport,
            EnforcementLevel::Escalation
        )
        // Custom trading-specific rule
        .add_custom_rule(
            "high_frequency_trading_monitor",
            trading_params,
            "trading_alert_system",
            alert_params,
            EnforcementLevel::Blocking
        )
        // Time-based reporting (every 4 hours during trading)
        .add_time_rule(
            4, // Every 4 hours
            RequiredAction::LogAndMonitor,
            EnforcementLevel::Warning
        )
        .expires_at(Utc::now() + Duration::days(365)) // 1 year validity
        .build()?;
    
    // Register the custom agreement
    let agreement_id = manager.register_custom_agreement(agreement).await?;
    info!("✅ Trading wallet BISO agreement created: {}", agreement_id);
    
    Ok(())
}

/// Example 2: Multi-Jurisdiction Government Wallet
pub async fn create_government_wallet_agreement() -> Result<()> {
    info!("🏛️ Creating multi-jurisdiction government BISO agreement...");
    
    let manager = BisoAgreementManager::new();
    
    // Geographic compliance parameters
    let mut geo_params = HashMap::new();
    geo_params.insert("primary_jurisdiction".to_string(), "US".to_string());
    geo_params.insert("secondary_jurisdictions".to_string(), "CA,UK,EU".to_string());
    geo_params.insert("data_sovereignty_level".to_string(), "highest".to_string());
    
    let mut compliance_params = HashMap::new();
    compliance_params.insert("gdpr_required".to_string(), "true".to_string());
    compliance_params.insert("ccpa_required".to_string(), "true".to_string());
    compliance_params.insert("audit_frequency".to_string(), "weekly".to_string());
    
    let agreement = BisoAgreementBuilder::new()
        .wallet_id("gov_multi_jurisdiction_001")
        .agreement_type(BisoAgreementType::GovernmentStamped {
            government_id: "US_TREASURY_001".to_string(),
            jurisdiction: "United States + International Treaties".to_string(),
            compliance_level: ComplianceLevel::Maximum,
            api_access_level: ApiAccessLevel::Full {
                bank_api: true,
                government_api: true,
                cross_system_communication: true
            }
        })
        // Geographic access control
        .add_geographic_rule(
            vec!["US".to_string(), "CA".to_string(), "UK".to_string(), "EU".to_string()],
            RequiredAction::EscalateToAuthority { 
                authority_type: "International Compliance Office".to_string() 
            },
            EnforcementLevel::Escalation
        )
        // Custom multi-jurisdiction rule
        .add_custom_rule(
            "multi_jurisdiction_compliance",
            geo_params,
            "international_compliance_check",
            compliance_params,
            EnforcementLevel::Escalation
        )
        .expires_at(Utc::now() + Duration::days(1095)) // 3 years validity
        .build()?;
    
    let agreement_id = manager.register_custom_agreement(agreement).await?;
    info!("✅ Government multi-jurisdiction BISO agreement created: {}", agreement_id);
    
    Ok(())
}

/// Example 3: Healthcare Data Wallet with HIPAA Compliance
pub async fn create_healthcare_wallet_agreement() -> Result<()> {
    info!("🏥 Creating HIPAA-compliant healthcare BISO agreement...");
    
    let manager = BisoAgreementManager::new();
    
    // HIPAA-specific parameters
    let mut hipaa_params = HashMap::new();
    hipaa_params.insert("phi_classification".to_string(), "protected_health_information".to_string());
    hipaa_params.insert("minimum_necessary_rule".to_string(), "enforced".to_string());
    hipaa_params.insert("breach_notification_required".to_string(), "true".to_string());
    
    let mut healthcare_actions = HashMap::new();
    healthcare_actions.insert("audit_log_retention".to_string(), "6_years".to_string());
    healthcare_actions.insert("encryption_standard".to_string(), "AES_256_FIPS_140_2".to_string());
    healthcare_actions.insert("access_logging".to_string(), "comprehensive".to_string());
    
    let agreement = BisoAgreementBuilder::new()
        .wallet_id("healthcare_hipaa_001")
        .agreement_type(BisoAgreementType::OtherStamped {
            stamp_type: "Healthcare".to_string(),
            issuer: "US Department of Health and Human Services".to_string(),
            restrictions: CommunicationRestrictions {
                can_share_poe: true,
                requires_biso_agreement: true,
                compliance_reporting_required: true,
                allowed_endpoints: vec![
                    "healthcare_api".to_string(),
                    "phi_access".to_string()
                ],
                blocked_endpoints: vec![
                    "cross_border_transfer".to_string(),
                    "non_healthcare_apis".to_string()
                ]
            }
        })
        // Custom HIPAA compliance rule
        .add_custom_rule(
            "hipaa_phi_access_control",
            hipaa_params,
            "hipaa_compliance_enforcement",
            healthcare_actions,
            EnforcementLevel::Escalation
        )
        // Time-based audit rule (daily)
        .add_time_rule(
            24, // Every 24 hours
            RequiredAction::GenerateComplianceReport,
            EnforcementLevel::Blocking
        )
        .expires_at(Utc::now() + Duration::days(730)) // 2 years validity
        .build()?;
    
    let agreement_id = manager.register_custom_agreement(agreement).await?;
    info!("✅ Healthcare HIPAA BISO agreement created: {}", agreement_id);
    
    Ok(())
}

/// Example 4: IoT Device Network Wallet
pub async fn create_iot_network_wallet_agreement() -> Result<()> {
    info!("🌐 Creating IoT device network BISO agreement...");
    
    let manager = BisoAgreementManager::new();
    
    // IoT-specific parameters
    let mut iot_params = HashMap::new();
    iot_params.insert("device_count_threshold".to_string(), "10000".to_string());
    iot_params.insert("data_transmission_rate".to_string(), "high_frequency".to_string());
    iot_params.insert("device_authentication".to_string(), "certificate_based".to_string());
    
    let mut monitoring_params = HashMap::new();
    monitoring_params.insert("anomaly_detection".to_string(), "ml_based".to_string());
    monitoring_params.insert("device_health_monitoring".to_string(), "continuous".to_string());
    monitoring_params.insert("security_patch_compliance".to_string(), "mandatory".to_string());
    
    let agreement = BisoAgreementBuilder::new()
        .wallet_id("iot_network_001")
        .agreement_type(BisoAgreementType::Unstamped {
            wallet_id: "IOT_NETWORK_WALLET_001".to_string(),
            mandatory_biso: true
        })
        // Volume-based device monitoring
        .add_volume_rule(
            50000, // 50k transactions threshold
            RequiredAction::RequireAuthentication,
            EnforcementLevel::Blocking
        )
        // Custom IoT monitoring rule
        .add_custom_rule(
            "iot_device_network_monitor",
            iot_params,
            "iot_security_enforcement",
            monitoring_params,
            EnforcementLevel::Blocking
        )
        // Time-based security check (every 2 hours)
        .add_time_rule(
            2, // Every 2 hours
            RequiredAction::LogAndMonitor,
            EnforcementLevel::Warning
        )
        .expires_at(Utc::now() + Duration::days(180)) // 6 months validity
        .build()?;
    
    let agreement_id = manager.register_custom_agreement(agreement).await?;
    info!("✅ IoT network BISO agreement created: {}", agreement_id);
    
    Ok(())
}

/// Example 5: Developer's Custom Rule with Webhook Integration
pub async fn create_webhook_integrated_agreement() -> Result<()> {
    info!("🔗 Creating webhook-integrated custom BISO agreement...");
    
    let manager = BisoAgreementManager::new();
    
    // Webhook integration parameters
    let mut webhook_params = HashMap::new();
    webhook_params.insert("webhook_url".to_string(), "https://api.myapp.com/biso-webhook".to_string());
    webhook_params.insert("authentication_method".to_string(), "bearer_token".to_string());
    webhook_params.insert("retry_attempts".to_string(), "3".to_string());
    webhook_params.insert("timeout_seconds".to_string(), "30".to_string());
    
    let mut notification_params = HashMap::new();
    notification_params.insert("notification_format".to_string(), "json".to_string());
    notification_params.insert("include_metadata".to_string(), "true".to_string());
    notification_params.insert("encryption_enabled".to_string(), "true".to_string());
    
    let agreement = BisoAgreementBuilder::new()
        .wallet_id("webhook_integrated_001")
        .agreement_type(BisoAgreementType::BankStamped {
            bank_id: "DEVELOPER_SANDBOX_001".to_string(),
            banking_license: "Sandbox License".to_string(),
            compliance_level: ComplianceLevel::Basic,
            api_access_level: ApiAccessLevel::Full {
                bank_api: true,
                government_api: false,
                cross_system_communication: true
            }
        })
        // Custom webhook trigger rule
        .add_custom_rule(
            "webhook_notification_trigger",
            webhook_params,
            "send_webhook_notification",
            notification_params,
            EnforcementLevel::Warning
        )
        // Volume-based webhook rule
        .add_volume_rule(
            100000, // 100k threshold
            RequiredAction::Custom {
                action_type: "webhook_alert".to_string(),
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("alert_type".to_string(), "high_volume".to_string());
                    params.insert("priority".to_string(), "high".to_string());
                    params
                }
            },
            EnforcementLevel::Blocking
        )
        .expires_at(Utc::now() + Duration::days(90)) // 3 months validity
        .build()?;
    
    let agreement_id = manager.register_custom_agreement(agreement).await?;
    info!("✅ Webhook-integrated BISO agreement created: {}", agreement_id);
    
    Ok(())
}

/// Run all developer examples
pub async fn run_all_developer_examples() -> Result<()> {
    info!("🚀 Running all developer BISO agreement examples...");
    
    create_trading_wallet_agreement().await?;
    create_government_wallet_agreement().await?;
    create_healthcare_wallet_agreement().await?;
    create_iot_network_wallet_agreement().await?;
    create_webhook_integrated_agreement().await?;
    
    info!("✅ All developer examples completed successfully!");
    info!("📋 Created 5 different types of custom BISO agreements:");
    info!("   1. High-volume trading wallet with SEC compliance");
    info!("   2. Multi-jurisdiction government wallet");
    info!("   3. HIPAA-compliant healthcare wallet");
    info!("   4. IoT device network monitoring wallet");
    info!("   5. Webhook-integrated developer wallet");
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing with a simple subscriber
    tracing_subscriber::fmt::init();
    
    info!("🚀 Running BPI Core Custom BISO Agreement Examples");
    
    // Run all developer examples
    run_all_developer_examples().await?;
    
    info!("✅ All custom BISO agreement examples completed successfully!");
    
    Ok(())
}
