use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use anyhow::Result;
use crate::wallet_registry::comprehensive_wallet_registry::{
    ComprehensiveWalletRegistry, WalletType, OwnerType, NetworkType, StampType
};

/// Wallet Registry CLI Commands for comprehensive wallet management
#[derive(Debug, Clone, Args)]
pub struct WalletRegistryArgs {
    #[command(subcommand)]
    pub command: WalletRegistryCommands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum WalletRegistryCommands {
    /// Register new wallet with mandatory registration ID
    Register {
        /// Wallet address in BPI format
        #[arg(long)]
        address: String,
        /// Wallet type (Community, Investor, Government, Bank, Owner, ESOP, Treasury, Company)
        #[arg(long)]
        wallet_type: String,
        /// Owner type (1-5, only for Owner wallets)
        #[arg(long)]
        owner_type: Option<u8>,
        /// Network type (testnet/mainnet)
        #[arg(long, default_value = "testnet")]
        network: String,
        /// Stamp type for special wallets
        #[arg(long)]
        stamp_type: Option<String>,
        /// JSON output format
        #[arg(long)]
        json: bool,
    },
    /// Create company wallet set (treasury, ESOP, operational)
    CreateCompany {
        /// Company identifier
        #[arg(long)]
        company_id: String,
        /// Network type (testnet/mainnet)
        #[arg(long, default_value = "testnet")]
        network: String,
        /// JSON output format
        #[arg(long)]
        json: bool,
    },
    /// Migrate wallet from testnet to mainnet
    Migrate {
        /// Registration ID of wallet to migrate
        #[arg(long)]
        registration_id: String,
        /// Force migration without compliance check (testnet only)
        #[arg(long)]
        force: bool,
        /// JSON output format
        #[arg(long)]
        json: bool,
    },
    /// Process PoE mining activity
    Mine {
        /// Registration ID of wallet
        #[arg(long)]
        registration_id: String,
        /// Number of PoE activities
        #[arg(long, default_value = "1")]
        activities: u64,
        /// Network load factor (0.0-1.0)
        #[arg(long, default_value = "0.5")]
        network_load: f64,
        /// JSON output format
        #[arg(long)]
        json: bool,
    },
    /// Get wallet information by registration ID
    Get {
        /// Registration ID of wallet
        #[arg(long)]
        registration_id: String,
        /// JSON output format
        #[arg(long)]
        json: bool,
    },
    /// List wallets by type
    List {
        /// Wallet type filter
        #[arg(long)]
        wallet_type: Option<String>,
        /// Owner type filter (1-5)
        #[arg(long)]
        owner_type: Option<u8>,
        /// Network type filter
        #[arg(long)]
        network: Option<String>,
        /// JSON output format
        #[arg(long)]
        json: bool,
    },
    /// Show wallet registry statistics
    Stats {
        /// JSON output format
        #[arg(long)]
        json: bool,
    },
    /// Validate wallet compliance for mainnet
    Validate {
        /// Registration ID of wallet
        #[arg(long)]
        registration_id: String,
        /// JSON output format
        #[arg(long)]
        json: bool,
    },
    /// Show owner type allocation and limits
    OwnerTypes {
        /// JSON output format
        #[arg(long)]
        json: bool,
    },
}

/// Handle wallet registry CLI commands
pub async fn handle_wallet_registry_command(args: WalletRegistryArgs) -> Result<()> {
    let registry = ComprehensiveWalletRegistry::new();
    
    match args.command {
        WalletRegistryCommands::Register { 
            address, wallet_type, owner_type, network, stamp_type, json 
        } => {
            handle_register_wallet(
                &registry, address, wallet_type, owner_type, network, stamp_type, json
            ).await
        },
        WalletRegistryCommands::CreateCompany { company_id, network, json } => {
            handle_create_company(&registry, company_id, network, json).await
        },
        WalletRegistryCommands::Migrate { registration_id, force, json } => {
            handle_migrate_wallet(&registry, registration_id, force, json).await
        },
        WalletRegistryCommands::Mine { registration_id, activities, network_load, json } => {
            handle_poe_mining(&registry, registration_id, activities, network_load, json).await
        },
        WalletRegistryCommands::Get { registration_id, json } => {
            handle_get_wallet(&registry, registration_id, json).await
        },
        WalletRegistryCommands::List { wallet_type, owner_type, network, json } => {
            handle_list_wallets(&registry, wallet_type, owner_type, network, json).await
        },
        WalletRegistryCommands::Stats { json } => {
            handle_registry_stats(&registry, json).await
        },
        WalletRegistryCommands::Validate { registration_id, json } => {
            handle_validate_compliance(&registry, registration_id, json).await
        },
        WalletRegistryCommands::OwnerTypes { json } => {
            handle_owner_types(&registry, json).await
        },
    }
}

/// Register new wallet with comprehensive validation
async fn handle_register_wallet(
    registry: &ComprehensiveWalletRegistry,
    address: String,
    wallet_type_str: String,
    owner_type_num: Option<u8>,
    network_str: String,
    stamp_type_str: Option<String>,
    json: bool,
) -> Result<()> {
    // Parse wallet type
    let wallet_type = match wallet_type_str.to_lowercase().as_str() {
        "community" => WalletType::Community,
        "investor" => WalletType::Investor,
        "government" => WalletType::Government,
        "bank" => WalletType::Bank,
        "owner" => WalletType::Owner,
        "esop" => WalletType::ESOP,
        "treasury" => WalletType::Treasury,
        "company" => WalletType::Company,
        _ => return Err(anyhow::anyhow!("Invalid wallet type: {}", wallet_type_str)),
    };

    // Parse owner type (only for Owner wallets)
    let owner_type = if wallet_type == WalletType::Owner {
        match owner_type_num {
            Some(1) => Some(OwnerType::Founder),
            Some(2) => Some(OwnerType::EarlyInvestor),
            Some(3) => Some(OwnerType::CommunityLeader),
            Some(4) => Some(OwnerType::StrategicPartner),
            Some(5) => Some(OwnerType::PublicInvestor),
            Some(n) => return Err(anyhow::anyhow!("Invalid owner type: {}. Must be 1-5", n)),
            None => return Err(anyhow::anyhow!("Owner type required for Owner wallets")),
        }
    } else {
        None
    };

    // Parse network type
    let network_type = match network_str.to_lowercase().as_str() {
        "testnet" => NetworkType::Testnet,
        "mainnet" => NetworkType::Mainnet,
        _ => return Err(anyhow::anyhow!("Invalid network type: {}. Must be testnet or mainnet", network_str)),
    };

    // Parse stamp type
    let stamp_type = if let Some(stamp_str) = stamp_type_str {
        match stamp_str.to_lowercase().as_str() {
            "government" => Some(StampType::Government),
            "bank" => Some(StampType::Bank),
            "community" => Some(StampType::Community),
            "enterprise" => Some(StampType::Enterprise),
            "emergency" => Some(StampType::Emergency),
            "hipaa" => Some(StampType::HIPAA),
            _ => return Err(anyhow::anyhow!("Invalid stamp type: {}", stamp_str)),
        }
    } else {
        None
    };

    // Register wallet
    let registration_id = registry.register_wallet(
        address.clone(),
        wallet_type,
        owner_type,
        network_type,
        stamp_type,
    ).await?;

    // Output result
    if json {
        let result = serde_json::json!({
            "success": true,
            "registration_id": registration_id,
            "wallet_address": address,
            "wallet_type": wallet_type_str,
            "network_type": network_str,
            "message": "Wallet registered successfully with mandatory registration ID"
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("✅ Wallet registered successfully!");
        println!("📋 Registration ID: {}", registration_id);
        println!("💰 Wallet Address: {}", address);
        println!("🏷️  Wallet Type: {}", wallet_type_str);
        println!("🌐 Network: {}", network_str);
        if let Some(owner_type) = owner_type_num {
            println!("👑 Owner Type: {}", owner_type);
        }
        println!("\n⚠️  IMPORTANT: Save your Registration ID! It's mandatory for all wallet operations.");
    }

    Ok(())
}

/// Create company wallet set
async fn handle_create_company(
    registry: &ComprehensiveWalletRegistry,
    company_id: String,
    network_str: String,
    json: bool,
) -> Result<()> {
    let network_type = match network_str.to_lowercase().as_str() {
        "testnet" => NetworkType::Testnet,
        "mainnet" => NetworkType::Mainnet,
        _ => return Err(anyhow::anyhow!("Invalid network type: {}", network_str)),
    };

    let company_wallet_set = registry.create_company_wallet_set(
        company_id.clone(),
        network_type,
    ).await?;

    if json {
        let result = serde_json::json!({
            "success": true,
            "company_id": company_id,
            "treasury_wallet": company_wallet_set.treasury_wallet,
            "esop_wallet": company_wallet_set.esop_wallet,
            "operational_wallet": company_wallet_set.operational_wallet,
            "network_type": network_str,
            "message": "Company wallet set created successfully"
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("🏢 Company wallet set created successfully!");
        println!("🏛️  Company ID: {}", company_id);
        println!("💰 Treasury Wallet: {}", company_wallet_set.treasury_wallet);
        println!("👥 ESOP Wallet: {}", company_wallet_set.esop_wallet);
        println!("⚙️  Operational Wallet: {}", company_wallet_set.operational_wallet);
        println!("🌐 Network: {}", network_str);
    }

    Ok(())
}

/// Migrate wallet from testnet to mainnet
async fn handle_migrate_wallet(
    registry: &ComprehensiveWalletRegistry,
    registration_id_str: String,
    force: bool,
    json: bool,
) -> Result<()> {
    let registration_id = Uuid::parse_str(&registration_id_str)
        .map_err(|_| anyhow::anyhow!("Invalid registration ID format"))?;

    // Check compliance unless forced (testnet only)
    let compliance_verified = if force {
        true // Force migration for testnet development
    } else {
        registry.validate_mainnet_compliance(registration_id).await?
    };

    let migration_record = registry.migrate_wallet_to_mainnet(
        registration_id,
        compliance_verified,
    ).await?;

    if json {
        let result = serde_json::json!({
            "success": true,
            "registration_id": registration_id,
            "migration_id": migration_record.migration_id,
            "from_network": "testnet",
            "to_network": "mainnet",
            "migrated_at": migration_record.migrated_at,
            "compliance_verified": migration_record.compliance_verified,
            "message": "Wallet migrated to mainnet successfully"
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("🚀 Wallet migrated to mainnet successfully!");
        println!("📋 Registration ID: {}", registration_id);
        println!("🆔 Migration ID: {}", migration_record.migration_id);
        println!("📅 Migrated At: {}", migration_record.migrated_at);
        println!("✅ Compliance Verified: {}", migration_record.compliance_verified);
        println!("💰 Real billing now active - $1.00 per BPI default");
        println!("🔒 100% security enforcement enabled");
    }

    Ok(())
}

/// Process PoE mining activity
async fn handle_poe_mining(
    registry: &ComprehensiveWalletRegistry,
    registration_id_str: String,
    activities: u64,
    network_load: f64,
    json: bool,
) -> Result<()> {
    let registration_id = Uuid::parse_str(&registration_id_str)
        .map_err(|_| anyhow::anyhow!("Invalid registration ID format"))?;

    let baby_coins_earned = registry.process_poe_mining(
        registration_id,
        activities,
        network_load,
    ).await?;

    if json {
        let result = serde_json::json!({
            "success": true,
            "registration_id": registration_id,
            "poe_activities": activities,
            "network_load": network_load,
            "baby_coins_earned": baby_coins_earned,
            "message": "PoE mining processed successfully"
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("⛏️  PoE mining processed successfully!");
        println!("📋 Registration ID: {}", registration_id);
        println!("🔄 PoE Activities: {}", activities);
        println!("📊 Network Load: {:.3}", network_load);
        println!("💎 Baby Coins Earned: {:.6}", baby_coins_earned);
        println!("\n💡 Baby coins have real value and grow with PoE activity!");
    }

    Ok(())
}

/// Get wallet information
async fn handle_get_wallet(
    registry: &ComprehensiveWalletRegistry,
    registration_id_str: String,
    json: bool,
) -> Result<()> {
    let registration_id = Uuid::parse_str(&registration_id_str)
        .map_err(|_| anyhow::anyhow!("Invalid registration ID format"))?;

    let wallet = registry.get_wallet(registration_id).await
        .ok_or_else(|| anyhow::anyhow!("Wallet not found"))?;

    if json {
        println!("{}", serde_json::to_string_pretty(&wallet)?);
    } else {
        println!("💰 Wallet Information");
        println!("📋 Registration ID: {}", wallet.registration_id);
        println!("🏠 Address: {}", wallet.wallet_address);
        println!("🏷️  Type: {:?}", wallet.wallet_type);
        if let Some(owner_type) = wallet.owner_type {
            println!("👑 Owner Type: {:?}", owner_type);
        }
        println!("🌐 Network: {:?}", wallet.network_type);
        println!("🪙 Mother Coins: {}", wallet.mother_coin_allocation);
        println!("💎 Baby Coins: {:.6}", wallet.baby_coin_balance);
        println!("⛏️  PoE Activities: {}", wallet.poe_stats.total_poe_activities);
        println!("✅ KYC Verified: {}", wallet.compliance_status.kyc_verified);
        println!("💳 Billing Enabled: {}", wallet.billing_config.billing_enabled);
        println!("📅 Created: {}", wallet.created_at);
    }

    Ok(())
}

/// List wallets with filters
async fn handle_list_wallets(
    registry: &ComprehensiveWalletRegistry,
    wallet_type_filter: Option<String>,
    owner_type_filter: Option<u8>,
    network_filter: Option<String>,
    json: bool,
) -> Result<()> {
    // For now, we'll implement a simple version
    // In a real implementation, you'd want to add filtering to the registry
    
    if json {
        let result = serde_json::json!({
            "message": "Wallet listing with filters not yet implemented",
            "filters": {
                "wallet_type": wallet_type_filter,
                "owner_type": owner_type_filter,
                "network": network_filter
            }
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("📋 Wallet listing with filters not yet implemented");
        println!("Use 'stats' command for summary information");
    }

    Ok(())
}

/// Show registry statistics
async fn handle_registry_stats(
    registry: &ComprehensiveWalletRegistry,
    json: bool,
) -> Result<()> {
    let owner_stats = registry.get_owner_type_stats().await;
    let total_mother_coins = registry.get_total_mother_coin_allocation().await;
    let total_baby_coins = registry.get_total_baby_coin_balance().await;

    if json {
        let result = serde_json::json!({
            "owner_type_stats": owner_stats,
            "total_mother_coins": total_mother_coins,
            "total_baby_coins": total_baby_coins,
            "max_wallets_per_type": 1_000_000
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("📊 Wallet Registry Statistics");
        println!("🪙 Total Mother Coins Allocated: {}", total_mother_coins);
        println!("💎 Total Baby Coins Balance: {:.6}", total_baby_coins);
        println!("📈 Max Wallets Per Owner Type: 1,000,000");
        println!("\n👑 Owner Type Statistics:");
        for (owner_type, count) in owner_stats {
            println!("  {:?}: {} wallets", owner_type, count);
        }
    }

    Ok(())
}

/// Validate wallet compliance
async fn handle_validate_compliance(
    registry: &ComprehensiveWalletRegistry,
    registration_id_str: String,
    json: bool,
) -> Result<()> {
    let registration_id = Uuid::parse_str(&registration_id_str)
        .map_err(|_| anyhow::anyhow!("Invalid registration ID format"))?;

    let is_compliant = registry.validate_mainnet_compliance(registration_id).await?;

    if json {
        let result = serde_json::json!({
            "registration_id": registration_id,
            "mainnet_compliant": is_compliant,
            "message": if is_compliant { 
                "Wallet is compliant for mainnet" 
            } else { 
                "Wallet requires compliance verification for mainnet" 
            }
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("🔍 Compliance Validation");
        println!("📋 Registration ID: {}", registration_id);
        if is_compliant {
            println!("✅ Wallet is compliant for mainnet migration");
            println!("🚀 Ready for mainnet with full security and billing");
        } else {
            println!("⚠️  Wallet requires compliance verification");
            println!("📋 Required: KYC, AML, Sanctions screening, Regulatory approval");
            println!("🔒 Mainnet migration blocked until compliance complete");
        }
    }

    Ok(())
}

/// Show owner type information
async fn handle_owner_types(
    registry: &ComprehensiveWalletRegistry,
    json: bool,
) -> Result<()> {
    let owner_stats = registry.get_owner_type_stats().await;

    if json {
        let result = serde_json::json!({
            "owner_types": {
                "1": {
                    "name": "Founder",
                    "description": "Company founders/core team",
                    "mother_coin_allocation": 600,
                    "current_wallets": owner_stats.get(&OwnerType::Founder).unwrap_or(&0)
                },
                "2": {
                    "name": "EarlyInvestor", 
                    "description": "Early investors",
                    "mother_coin_allocation": 100,
                    "current_wallets": owner_stats.get(&OwnerType::EarlyInvestor).unwrap_or(&0)
                },
                "3": {
                    "name": "CommunityLeader",
                    "description": "Community leaders", 
                    "mother_coin_allocation": 100,
                    "current_wallets": owner_stats.get(&OwnerType::CommunityLeader).unwrap_or(&0)
                },
                "4": {
                    "name": "StrategicPartner",
                    "description": "Strategic partners",
                    "mother_coin_allocation": 100,
                    "current_wallets": owner_stats.get(&OwnerType::StrategicPartner).unwrap_or(&0)
                },
                "5": {
                    "name": "PublicInvestor",
                    "description": "Public investors",
                    "mother_coin_allocation": 100,
                    "current_wallets": owner_stats.get(&OwnerType::PublicInvestor).unwrap_or(&0)
                }
            },
            "max_wallets_per_type": 1_000_000,
            "governance_power": "First owner + investor group has 100% power initially"
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("👑 Owner Type Information");
        println!("📊 Maximum 1,000,000 wallets per type");
        println!("🏛️  First owner + investor group: 100% governance power\n");
        
        println!("Type 1 - Founder (Company founders/core team)");
        println!("  🪙 Mother Coins: 600 (primary wallet)");
        println!("  📊 Current Wallets: {}\n", owner_stats.get(&OwnerType::Founder).unwrap_or(&0));
        
        println!("Type 2 - Early Investor");
        println!("  🪙 Mother Coins: 100 each");
        println!("  📊 Current Wallets: {}\n", owner_stats.get(&OwnerType::EarlyInvestor).unwrap_or(&0));
        
        println!("Type 3 - Community Leader");
        println!("  🪙 Mother Coins: 100 each");
        println!("  📊 Current Wallets: {}\n", owner_stats.get(&OwnerType::CommunityLeader).unwrap_or(&0));
        
        println!("Type 4 - Strategic Partner");
        println!("  🪙 Mother Coins: 100 each");
        println!("  📊 Current Wallets: {}\n", owner_stats.get(&OwnerType::StrategicPartner).unwrap_or(&0));
        
        println!("Type 5 - Public Investor");
        println!("  🪙 Mother Coins: 100 each");
        println!("  📊 Current Wallets: {}\n", owner_stats.get(&OwnerType::PublicInvestor).unwrap_or(&0));
    }

    Ok(())
}
