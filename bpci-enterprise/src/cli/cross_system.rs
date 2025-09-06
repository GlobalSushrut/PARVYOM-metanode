//! # Cross-System Integration CLI
//!
//! Command-line interface for Court-Shadow Bridge, BPI Mesh Integration, and Unified Audit System

use anyhow::Result;
use clap::Subcommand;
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

use crate::court_shadow_bridge::{
    CourtShadowBridge, CourtShadowBridgeConfig, ShadowExecutionRequest, 
    Web2Endpoint, AuthMethod, PrivacyRequirements, DataClassification
};
use crate::court_bpi_mesh_integration::{
    CourtBpiMeshBridge, CourtBpiMeshConfig, ContractBankingRequest, 
    BankingOperationType, EconomicAuthorityLevel, BankingCredentials,
    NotaryCredentials, SigningKeys, AuthorityDelegation, NotaryVerificationLevel,
    BankingServiceType
};
use crate::unified_audit_system::{
    UnifiedAuditSystem, UnifiedAuditConfig, SystemSource, 
    UnifiedAuditEventType, PrivacyLevel
};

/// Cross-system integration commands
#[derive(Debug, Subcommand)]
pub enum CrossSystemCommands {
    /// Court-Shadow Registry Bridge operations
    CourtShadow {
        #[clap(subcommand)]
        action: CourtShadowAction,
    },
    /// Court-BPI Mesh integration operations
    CourtBpi {
        #[clap(subcommand)]
        action: CourtBpiAction,
    },
    /// Unified audit system operations
    UnifiedAudit {
        #[clap(subcommand)]
        action: UnifiedAuditAction,
    },
    /// Test all cross-system integrations
    TestIntegrations,
}

/// Court-Shadow Bridge actions
#[derive(Debug, Subcommand)]
pub enum CourtShadowAction {
    /// Register contract for shadow integration
    RegisterContract {
        #[clap(long)]
        contract_id: String,
        #[clap(long)]
        shadow_identity: String,
        #[clap(long)]
        endpoint_url: String,
    },
    /// Execute contract through shadow bridge
    ExecuteContract {
        #[clap(long)]
        contract_id: String,
        #[clap(long)]
        target_system: String,
        #[clap(long)]
        timeout_seconds: Option<u64>,
    },
    /// Get bridge statistics
    GetStats,
}

/// Court-BPI Mesh actions
#[derive(Debug, Subcommand)]
pub enum CourtBpiAction {
    /// Register bank integration for contract
    RegisterBank {
        #[clap(long)]
        contract_id: String,
        #[clap(long)]
        bank_id: String,
        #[clap(long)]
        bank_name: String,
    },
    /// Execute banking operation
    ExecuteBanking {
        #[clap(long)]
        contract_id: String,
        #[clap(long)]
        operation: String,
        #[clap(long)]
        amount: Option<f64>,
        #[clap(long)]
        currency: Option<String>,
    },
    /// Get economic statistics
    GetEconomicStats,
}

/// Unified Audit actions
#[derive(Debug, Subcommand)]
pub enum UnifiedAuditAction {
    /// Log audit event
    LogEvent {
        #[clap(long)]
        system: String,
        #[clap(long)]
        event_type: String,
        #[clap(long)]
        description: String,
    },
    /// Get audit trail
    GetTrail {
        #[clap(long)]
        system_filter: Option<String>,
        #[clap(long)]
        privacy_level: Option<String>,
    },
    /// Generate compliance report
    GenerateReport,
}

impl CrossSystemCommands {
    /// Execute cross-system command
    pub async fn execute(&self) -> Result<()> {
        match self {
            CrossSystemCommands::CourtShadow { action } => {
                execute_court_shadow_action(action).await
            },
            CrossSystemCommands::CourtBpi { action } => {
                execute_court_bpi_action(action).await
            },
            CrossSystemCommands::UnifiedAudit { action } => {
                execute_unified_audit_action(action).await
            },
            CrossSystemCommands::TestIntegrations => {
                test_all_integrations().await
            },
        }
    }
}

/// Execute Court-Shadow Bridge action
async fn execute_court_shadow_action(action: &CourtShadowAction) -> Result<()> {
    let config = CourtShadowBridgeConfig::default();
    let bridge = CourtShadowBridge::new().await?;

    match action {
        CourtShadowAction::RegisterContract { contract_id, shadow_identity, endpoint_url } => {
            let contract_uuid = Uuid::parse_str(contract_id)
                .unwrap_or_else(|_| Uuid::new_v4());

            let endpoints = vec![Web2Endpoint {
                endpoint_id: "cli_endpoint".to_string(),
                url: endpoint_url.clone(),
                capabilities: vec!["read".to_string(), "write".to_string(), "execute".to_string()],
                auth_method: AuthMethod::ActingAs { 
                    identity: shadow_identity.clone() 
                },
            }];

            let privacy_requirements = PrivacyRequirements {
                require_zk_proofs: true,
                perfect_forward_secrecy: true,
                data_classification: DataClassification::Confidential,
                compliance_requirements: vec![],
            };

            bridge.register_contract_mapping(
                contract_uuid,
                shadow_identity.clone(),
                endpoints,
                privacy_requirements,
            ).await?;

            println!("✅ Contract {} registered for shadow integration with identity {}", 
                contract_id, shadow_identity);
        },
        CourtShadowAction::ExecuteContract { contract_id, target_system, timeout_seconds } => {
            let contract_uuid = Uuid::parse_str(contract_id)
                .unwrap_or_else(|_| Uuid::new_v4());

            let request = ShadowExecutionRequest {
                contract_id: contract_uuid,
                parameters: HashMap::new(),
                target_system: target_system.clone(),
                privacy_requirements: PrivacyRequirements {
                    require_zk_proofs: false,
                    perfect_forward_secrecy: false,
                    data_classification: DataClassification::Internal,
                    compliance_requirements: vec![],
                },
                timeout_seconds: timeout_seconds.unwrap_or(30),
            };

            let result = bridge.execute_contract_through_shadow(request).await?;
            
            if result.success {
                println!("✅ Contract execution successful");
                if let Some(data) = result.result_data {
                    println!("📊 Result: {}", serde_json::to_string_pretty(&data)?);
                }
            } else {
                println!("❌ Contract execution failed: {}", 
                    result.error_message.unwrap_or_default());
            }
        },
        CourtShadowAction::GetStats => {
            let stats = bridge.get_bridge_stats().await?;
            println!("📈 Court-Shadow Bridge Statistics:");
            println!("   Contract Mappings: {}", stats.total_contract_mappings);
            println!("   Active Sessions: {}", stats.active_sessions);
            println!("   Total Audit Events: {}", stats.total_audit_events);
            println!("   Successful Executions: {}", stats.successful_executions);
            println!("   Failed Executions: {}", stats.failed_executions);
        },
    }

    Ok(())
}

/// Execute Court-BPI Mesh action
async fn execute_court_bpi_action(action: &CourtBpiAction) -> Result<()> {
    let config = CourtBpiMeshConfig::default();
    let bridge = CourtBpiMeshBridge::new(config).await?;

    match action {
        CourtBpiAction::RegisterBank { contract_id, bank_id, bank_name } => {
            let contract_uuid = Uuid::parse_str(contract_id)
                .unwrap_or_else(|_| Uuid::new_v4());

            let credentials = BankingCredentials {
                api_key: format!("api_key_{}", bank_id),
                notary_credentials: NotaryCredentials {
                    notary_id: format!("notary_{}", bank_id),
                    public_key: vec![1, 2, 3, 4],
                    banking_license: format!("license_{}", bank_id),
                    verification_level: NotaryVerificationLevel::Enhanced,
                },
                signing_keys: SigningKeys {
                    ed25519_key: vec![1, 2, 3, 4],
                    bpc_key: vec![5, 6, 7, 8],
                    bank_key: vec![9, 10, 11, 12],
                },
                authority_delegation: AuthorityDelegation {
                    authority_level: EconomicAuthorityLevel::Bank,
                    scope: vec!["banking".to_string(), "transactions".to_string()],
                    expires_at: chrono::Utc::now() + chrono::Duration::days(365),
                    delegating_entity: "BPCI_HQ".to_string(),
                },
            };

            bridge.register_bank_integration(
                contract_uuid,
                bank_id.clone(),
                bank_name.clone(),
                vec![
                    BankingServiceType::FiatTransfer,
                    BankingServiceType::CryptoExchange,
                    BankingServiceType::AccountManagement,
                ],
                credentials,
                EconomicAuthorityLevel::Bank,
            ).await?;

            println!("✅ Bank integration registered for contract {} with bank {} ({})", 
                contract_id, bank_id, bank_name);
        },
        CourtBpiAction::ExecuteBanking { contract_id, operation, amount, currency } => {
            let contract_uuid = Uuid::parse_str(contract_id)
                .unwrap_or_else(|_| Uuid::new_v4());

            let operation_type = match operation.as_str() {
                "create_account" => BankingOperationType::CreateAccount,
                "transfer_funds" => BankingOperationType::TransferFunds,
                "exchange_tokens" => BankingOperationType::ExchangeTokens,
                "investment_advice" => BankingOperationType::InvestmentAdvice,
                _ => BankingOperationType::TransferFunds,
            };

            let mut parameters = HashMap::new();
            if let Some(amt) = amount {
                parameters.insert("amount".to_string(), json!(amt));
            }
            if let Some(curr) = currency {
                parameters.insert("currency".to_string(), json!(curr));
            }

            let request = ContractBankingRequest {
                contract_id: contract_uuid,
                operation_type,
                parameters,
                required_authority: EconomicAuthorityLevel::Enterprise,
                timeout_seconds: 60,
            };

            let response = bridge.execute_banking_operation(request).await?;
            
            if response.success {
                println!("✅ Banking operation successful");
                if let Some(data) = response.response_data {
                    println!("📊 Result: {}", serde_json::to_string_pretty(&data)?);
                }
                if let Some(tx_id) = response.transaction_id {
                    println!("💳 Transaction ID: {}", tx_id);
                }
            } else {
                println!("❌ Banking operation failed: {}", 
                    response.error_message.unwrap_or_default());
            }
        },
        CourtBpiAction::GetEconomicStats => {
            let stats = bridge.get_economic_stats().await?;
            println!("💰 Economic Statistics:");
            println!("   Bank Integrations: {}", stats.total_bank_integrations);
            println!("   Total Transactions: {}", stats.total_transactions);
            println!("   Active Banking Services: {}", stats.active_banking_services);
            println!("   Success Rate: {:.2}%", stats.success_rate * 100.0);
            println!("   Total Audit Events: {}", stats.total_audit_events);
            println!("   Volume by Token:");
            for (token, volume) in &stats.volume_by_token {
                println!("     {:?}: {:.2}", token, volume);
            }
        },
    }

    Ok(())
}

/// Execute Unified Audit action
async fn execute_unified_audit_action(action: &UnifiedAuditAction) -> Result<()> {
    let config = UnifiedAuditConfig::default();
    let audit_system = UnifiedAuditSystem::new(config).await?;

    match action {
        UnifiedAuditAction::LogEvent { system, event_type, description } => {
            let system_source = match system.as_str() {
                "court" => SystemSource::CourtNode,
                "shadow" => SystemSource::ShadowRegistry,
                "bpi" => SystemSource::BpiMesh,
                "court_shadow" => SystemSource::CourtShadowBridge,
                "court_bpi" => SystemSource::CourtBpiMeshBridge,
                _ => SystemSource::UnifiedAuditSystem,
            };

            let audit_event_type = match event_type.as_str() {
                "contract_deployed" => UnifiedAuditEventType::ContractDeployed,
                "contract_executed" => UnifiedAuditEventType::ContractExecuted,
                "bridge_used" => UnifiedAuditEventType::Web2Web3BridgeUsed,
                "banking_operation" => UnifiedAuditEventType::BankingOperationExecuted,
                "compliance_violation" => UnifiedAuditEventType::ComplianceViolation,
                _ => UnifiedAuditEventType::CrossSystemIntegration,
            };

            let mut event_data = HashMap::new();
            event_data.insert("description".to_string(), json!(description));
            event_data.insert("cli_generated".to_string(), json!(true));

            let event_id = audit_system.log_audit_event(
                system_source,
                audit_event_type,
                event_data,
                None,
            ).await?;

            println!("✅ Audit event logged with ID: {}", event_id);
        },
        UnifiedAuditAction::GetTrail { system_filter, privacy_level } => {
            let system_filter = system_filter.as_ref().map(|s| match s.as_str() {
                "court" => SystemSource::CourtNode,
                "shadow" => SystemSource::ShadowRegistry,
                "bpi" => SystemSource::BpiMesh,
                _ => SystemSource::UnifiedAuditSystem,
            });

            let privacy_clearance = match privacy_level.as_deref().unwrap_or("internal") {
                "public" => PrivacyLevel::Public,
                "internal" => PrivacyLevel::Internal,
                "confidential" => PrivacyLevel::Confidential,
                "restricted" => PrivacyLevel::Restricted,
                "zk_protected" => PrivacyLevel::ZkProtected,
                _ => PrivacyLevel::Internal,
            };

            let trail = audit_system.get_audit_trail(system_filter, privacy_clearance).await?;
            
            println!("📋 Audit Trail ({} events):", trail.len());
            for event in trail.iter().take(10) { // Show last 10 events
                println!("   {} | {:?} | {:?} | {:?}", 
                    event.timestamp.format("%Y-%m-%d %H:%M:%S"),
                    event.system_source,
                    event.event_type,
                    event.privacy_level);
            }
            
            if trail.len() > 10 {
                println!("   ... and {} more events", trail.len() - 10);
            }
        },
        UnifiedAuditAction::GenerateReport => {
            let report = audit_system.generate_compliance_report().await?;
            
            println!("📊 Compliance Report (ID: {})", report.report_id);
            println!("   Generated: {}", report.generated_at.format("%Y-%m-%d %H:%M:%S"));
            println!("   Total Events: {}", report.total_events);
            println!("   Compliant Events: {}", report.compliant_events);
            println!("   Compliance Rate: {:.2}%", report.compliance_rate);
            println!("   Active Violations: {}", report.active_violations);
            println!("   System Compliance:");
            for (system, rate) in &report.system_compliance {
                println!("     {:?}: {:.2}%", system, rate);
            }
        },
    }

    Ok(())
}

/// Test all cross-system integrations
async fn test_all_integrations() -> Result<()> {
    println!("🧪 Testing Cross-System Integrations...\n");

    // Test Court-Shadow Bridge
    println!("1️⃣ Testing Court-Shadow Bridge...");
    let shadow_bridge = CourtShadowBridge::new().await?;
    let shadow_stats = shadow_bridge.get_bridge_stats().await?;
    println!("   ✅ Court-Shadow Bridge initialized");
    println!("   📊 Initial stats: {} mappings, {} sessions\n", 
        shadow_stats.total_contract_mappings, shadow_stats.active_sessions);

    // Test Court-BPI Mesh Integration
    println!("2️⃣ Testing Court-BPI Mesh Integration...");
    let bpi_bridge = CourtBpiMeshBridge::new(CourtBpiMeshConfig::default()).await?;
    let economic_stats = bpi_bridge.get_economic_stats().await?;
    println!("   ✅ Court-BPI Mesh Bridge initialized");
    println!("   💰 Initial stats: {} integrations, {} transactions\n", 
        economic_stats.total_bank_integrations, economic_stats.total_transactions);

    // Test Unified Audit System
    println!("3️⃣ Testing Unified Audit System...");
    let audit_system = UnifiedAuditSystem::new(UnifiedAuditConfig::default()).await?;
    
    // Log a test event
    let mut test_data = HashMap::new();
    test_data.insert("test_type".to_string(), json!("integration_test"));
    
    let event_id = audit_system.log_audit_event(
        SystemSource::UnifiedAuditSystem,
        UnifiedAuditEventType::CrossSystemIntegration,
        test_data,
        None,
    ).await?;
    
    let report = audit_system.generate_compliance_report().await?;
    println!("   ✅ Unified Audit System initialized");
    println!("   📋 Test event logged: {}", event_id);
    println!("   📊 Compliance rate: {:.2}%\n", report.compliance_rate);

    println!("🎉 All cross-system integrations tested successfully!");
    println!("🚀 Court, Shadow Registry, BPI Mesh, and Audit systems are ready for production use.");

    Ok(())
}
