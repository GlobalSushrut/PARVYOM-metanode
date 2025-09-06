use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::{Result, anyhow};
use tokio::sync::RwLock;
use std::sync::Arc;

/// Comprehensive Wallet Registry System for BPI/BPCI Integration
/// Supports all stakeholder types with mandatory registration IDs and compliance
#[derive(Debug, Clone)]
pub struct ComprehensiveWalletRegistry {
    /// All registered wallets with mandatory registration IDs
    wallets: Arc<RwLock<HashMap<Uuid, RegisteredWallet>>>,
    /// Owner type allocations and limits
    owner_type_config: Arc<RwLock<OwnerTypeConfig>>,
    /// Company default wallets (treasury, ESOP, etc.)
    company_wallets: Arc<RwLock<HashMap<String, CompanyWalletSet>>>,
    /// Network migration tracking
    migration_history: Arc<RwLock<HashMap<Uuid, Vec<MigrationRecord>>>>,
    /// Compliance and regulatory framework
    compliance_engine: Arc<RwLock<ComplianceEngine>>,
}

/// Registered wallet with comprehensive metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredWallet {
    /// Mandatory unique registration ID (prevents loss/conflicts)
    pub registration_id: Uuid,
    /// BPI wallet address in production format
    pub wallet_address: String,
    /// Wallet type classification
    pub wallet_type: WalletType,
    /// Owner type (1-5 for owner wallets, None for others)
    pub owner_type: Option<OwnerType>,
    /// Network type (Testnet/Mainnet)
    pub network_type: NetworkType,
    /// Stamp type for special wallets
    pub stamp_type: Option<StampType>,
    /// Mother coin allocation for eligible wallets
    pub mother_coin_allocation: u64,
    /// Baby coin balance (earned through PoE mining)
    pub baby_coin_balance: f64,
    /// PoE mining statistics
    pub poe_stats: PoEMiningStats,
    /// Compliance status and verification
    pub compliance_status: ComplianceStatus,
    /// Billing configuration
    pub billing_config: BillingConfig,
    /// Creation and update timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Migration history
    pub migration_count: u32,
}

/// Wallet type classification for all stakeholders
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WalletType {
    /// Community investor wallets
    Community,
    /// Various investor types
    Investor,
    /// Government stamped wallets (regulatory)
    Government,
    /// Bank stamped wallets (financial)
    Bank,
    /// Company owner wallets (types 1-5)
    Owner,
    /// Employee Stock Ownership Plan wallets
    ESOP,
    /// Company treasury wallets
    Treasury,
    /// Company operational wallets
    Company,
}

/// Owner type classification (1-5) with different allocations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OwnerType {
    /// Type 1: Company founders/core team (600 mother coins for primary wallet)
    Founder = 1,
    /// Type 2: Early investors (100 coins each)
    EarlyInvestor = 2,
    /// Type 3: Community leaders (variable allocation)
    CommunityLeader = 3,
    /// Type 4: Strategic partners (negotiated allocation)
    StrategicPartner = 4,
    /// Type 5: Public investors (market-based allocation)
    PublicInvestor = 5,
}

/// Network type with different rules and billing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NetworkType {
    /// Testnet: Free coins, no real billing, refundable, relaxed security
    Testnet,
    /// Mainnet: Real billing, $1/BPI default, 100% security, compliance required
    Mainnet,
}

/// Stamp type for special wallet categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StampType {
    Government,
    Bank,
    Community,
    Enterprise,
    Emergency,
    HIPAA,
}

/// PoE (Proof of Existence) mining statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoEMiningStats {
    /// Total PoE activities recorded
    pub total_poe_activities: u64,
    /// Baby coins earned through PoE mining
    pub baby_coins_earned: f64,
    /// Node mining activities
    pub node_mining_count: u64,
    /// Internal logbook records
    pub logbook_records: u64,
    /// Last mining activity
    pub last_mining_at: Option<DateTime<Utc>>,
}

/// Compliance status and regulatory verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    /// KYC verification status
    pub kyc_verified: bool,
    /// AML screening status
    pub aml_cleared: bool,
    /// Sanctions screening status
    pub sanctions_cleared: bool,
    /// Regulatory approval status
    pub regulatory_approved: bool,
    /// Compliance verification date
    pub verified_at: Option<DateTime<Utc>>,
    /// Compliance jurisdiction
    pub jurisdiction: String,
}

/// Billing configuration for testnet vs mainnet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingConfig {
    /// Billing enabled (false for testnet, true for mainnet)
    pub billing_enabled: bool,
    /// BPI coin price (default $1.00 for mainnet)
    pub bpi_price_usd: f64,
    /// Free coin allocation (1500 + 500 for testnet)
    pub free_coin_allocation: u64,
    /// Refundable status (true for testnet)
    pub refundable: bool,
    /// Real rent and gas fees
    pub rent_gas_enabled: bool,
}

/// Owner type configuration and limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerTypeConfig {
    /// Maximum wallets per owner type (1000k each)
    pub max_wallets_per_type: u64,
    /// Current wallet counts per type
    pub current_counts: HashMap<OwnerType, u64>,
    /// Mother coin allocations per type
    pub mother_coin_allocations: HashMap<OwnerType, u64>,
    /// First owner + investor group power (100%)
    pub governance_power_enabled: bool,
}

/// Company wallet set (treasury, ESOP, operational)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyWalletSet {
    /// Company identifier
    pub company_id: String,
    /// Treasury wallet for company funds
    pub treasury_wallet: Uuid,
    /// ESOP wallet for employee stock ownership
    pub esop_wallet: Uuid,
    /// Operational wallet for daily operations
    pub operational_wallet: Uuid,
    /// Reserve wallet for emergency funds
    pub reserve_wallet: Option<Uuid>,
    /// Total company allocation
    pub total_allocation: u64,
}

/// Migration record for testnet → mainnet transfers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRecord {
    /// Migration ID
    pub migration_id: Uuid,
    /// Source network
    pub from_network: NetworkType,
    /// Target network
    pub to_network: NetworkType,
    /// Migration timestamp
    pub migrated_at: DateTime<Utc>,
    /// Compliance verification required
    pub compliance_verified: bool,
    /// Security upgrade completed
    pub security_upgraded: bool,
    /// Billing setup completed
    pub billing_setup: bool,
}

/// Compliance engine for global regulations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceEngine {
    /// Global regulatory frameworks
    pub regulatory_frameworks: Vec<String>,
    /// KYC/AML providers
    pub kyc_providers: Vec<String>,
    /// Sanctions screening services
    pub sanctions_services: Vec<String>,
    /// Data sovereignty rules
    pub data_sovereignty: HashMap<String, Vec<String>>,
}

impl ComprehensiveWalletRegistry {
    /// Create new comprehensive wallet registry
    pub fn new() -> Self {
        let owner_type_config = OwnerTypeConfig {
            max_wallets_per_type: 1_000_000, // 1000k wallets per type
            current_counts: HashMap::new(),
            mother_coin_allocations: {
                let mut allocations = HashMap::new();
                allocations.insert(OwnerType::Founder, 600); // Primary wallet gets 600 coins
                allocations.insert(OwnerType::EarlyInvestor, 100); // Others get 100 each
                allocations.insert(OwnerType::CommunityLeader, 100);
                allocations.insert(OwnerType::StrategicPartner, 100);
                allocations.insert(OwnerType::PublicInvestor, 100);
                allocations
            },
            governance_power_enabled: true, // First owner + investor group has 100% power
        };

        let compliance_engine = ComplianceEngine {
            regulatory_frameworks: vec![
                "GDPR".to_string(),
                "CCPA".to_string(),
                "PCI-DSS".to_string(),
                "SOC2".to_string(),
                "ISO27001".to_string(),
                "FATF".to_string(),
            ],
            kyc_providers: vec![
                "Jumio".to_string(),
                "Onfido".to_string(),
                "Sumsub".to_string(),
            ],
            sanctions_services: vec![
                "OFAC".to_string(),
                "EU Sanctions".to_string(),
                "UN Sanctions".to_string(),
            ],
            data_sovereignty: HashMap::new(),
        };

        Self {
            wallets: Arc::new(RwLock::new(HashMap::new())),
            owner_type_config: Arc::new(RwLock::new(owner_type_config)),
            company_wallets: Arc::new(RwLock::new(HashMap::new())),
            migration_history: Arc::new(RwLock::new(HashMap::new())),
            compliance_engine: Arc::new(RwLock::new(compliance_engine)),
        }
    }

    /// Register new wallet with mandatory registration ID
    pub async fn register_wallet(
        &self,
        wallet_address: String,
        wallet_type: WalletType,
        owner_type: Option<OwnerType>,
        network_type: NetworkType,
        stamp_type: Option<StampType>,
    ) -> Result<Uuid> {
        let registration_id = Uuid::new_v4();
        
        // Check owner type limits (1000k per type)
        if let Some(owner_type) = &owner_type {
            let mut config = self.owner_type_config.write().await;
            let current_count = *config.current_counts.get(owner_type).unwrap_or(&0);
            if current_count >= config.max_wallets_per_type {
                return Err(anyhow!("Owner type {:?} has reached maximum wallet limit", owner_type));
            }
            config.current_counts.insert(owner_type.clone(), current_count + 1);
        }

        // Determine mother coin allocation
        let mother_coin_allocation = if let Some(owner_type) = &owner_type {
            let config = self.owner_type_config.read().await;
            *config.mother_coin_allocations.get(owner_type).unwrap_or(&0)
        } else {
            0 // Non-owner wallets don't get mother coins
        };

        // Configure billing based on network type
        let billing_config = match network_type {
            NetworkType::Testnet => BillingConfig {
                billing_enabled: false,
                bpi_price_usd: 0.0, // Free for testnet
                free_coin_allocation: 1500 + 500, // 1500 + 500 free coins
                refundable: true,
                rent_gas_enabled: false,
            },
            NetworkType::Mainnet => BillingConfig {
                billing_enabled: true,
                bpi_price_usd: 1.0, // $1.00 per BPI default
                free_coin_allocation: 0, // No free coins on mainnet
                refundable: false,
                rent_gas_enabled: true, // Real rent and gas fees
            },
        };

        // Initialize compliance status
        let compliance_status = ComplianceStatus {
            kyc_verified: network_type == NetworkType::Testnet, // Auto-verify for testnet
            aml_cleared: network_type == NetworkType::Testnet,
            sanctions_cleared: network_type == NetworkType::Testnet,
            regulatory_approved: network_type == NetworkType::Testnet,
            verified_at: if network_type == NetworkType::Testnet {
                Some(Utc::now())
            } else {
                None // Mainnet requires manual verification
            },
            jurisdiction: "Global".to_string(),
        };

        let wallet = RegisteredWallet {
            registration_id,
            wallet_address,
            wallet_type,
            owner_type,
            network_type,
            stamp_type,
            mother_coin_allocation,
            baby_coin_balance: 0.0,
            poe_stats: PoEMiningStats {
                total_poe_activities: 0,
                baby_coins_earned: 0.0,
                node_mining_count: 0,
                logbook_records: 0,
                last_mining_at: None,
            },
            compliance_status,
            billing_config,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            migration_count: 0,
        };

        let mut wallets = self.wallets.write().await;
        wallets.insert(registration_id, wallet);

        Ok(registration_id)
    }

    /// Create default company wallet set (treasury, ESOP, operational)
    pub async fn create_company_wallet_set(
        &self,
        company_id: String,
        network_type: NetworkType,
    ) -> Result<CompanyWalletSet> {
        // Create treasury wallet
        let treasury_id = self.register_wallet(
            format!("BPI-TREASURY-{}", company_id),
            WalletType::Treasury,
            None,
            network_type.clone(),
            None,
        ).await?;

        // Create ESOP wallet
        let esop_id = self.register_wallet(
            format!("BPI-ESOP-{}", company_id),
            WalletType::ESOP,
            None,
            network_type.clone(),
            None,
        ).await?;

        // Create operational wallet
        let operational_id = self.register_wallet(
            format!("BPI-OPS-{}", company_id),
            WalletType::Company,
            None,
            network_type.clone(),
            None,
        ).await?;

        let company_wallet_set = CompanyWalletSet {
            company_id: company_id.clone(),
            treasury_wallet: treasury_id,
            esop_wallet: esop_id,
            operational_wallet: operational_id,
            reserve_wallet: None,
            total_allocation: 0, // To be set based on company size
        };

        let mut company_wallets = self.company_wallets.write().await;
        company_wallets.insert(company_id, company_wallet_set.clone());

        Ok(company_wallet_set)
    }

    /// Migrate wallet from testnet to mainnet (requires compliance verification)
    pub async fn migrate_wallet_to_mainnet(
        &self,
        registration_id: Uuid,
        compliance_verified: bool,
    ) -> Result<MigrationRecord> {
        let mut wallets = self.wallets.write().await;
        let wallet = wallets.get_mut(&registration_id)
            .ok_or_else(|| anyhow!("Wallet not found"))?;

        if wallet.network_type == NetworkType::Mainnet {
            return Err(anyhow!("Wallet is already on mainnet"));
        }

        if !compliance_verified {
            return Err(anyhow!("Compliance verification required for mainnet migration"));
        }

        // Update wallet for mainnet
        wallet.network_type = NetworkType::Mainnet;
        wallet.billing_config = BillingConfig {
            billing_enabled: true,
            bpi_price_usd: 1.0,
            free_coin_allocation: 0,
            refundable: false,
            rent_gas_enabled: true,
        };
        wallet.compliance_status.regulatory_approved = true;
        wallet.compliance_status.verified_at = Some(Utc::now());
        wallet.migration_count += 1;
        wallet.updated_at = Utc::now();

        // Create migration record
        let migration_record = MigrationRecord {
            migration_id: Uuid::new_v4(),
            from_network: NetworkType::Testnet,
            to_network: NetworkType::Mainnet,
            migrated_at: Utc::now(),
            compliance_verified: true,
            security_upgraded: true,
            billing_setup: true,
        };

        // Store migration history
        let mut migration_history = self.migration_history.write().await;
        migration_history.entry(registration_id)
            .or_insert_with(Vec::new)
            .push(migration_record.clone());

        Ok(migration_record)
    }

    /// Process PoE mining activity and generate baby coins
    pub async fn process_poe_mining(
        &self,
        registration_id: Uuid,
        poe_activities: u64,
        network_load: f64,
    ) -> Result<f64> {
        let mut wallets = self.wallets.write().await;
        let wallet = wallets.get_mut(&registration_id)
            .ok_or_else(|| anyhow!("Wallet not found"))?;

        // Calculate baby coins based on PoE activities and network load
        let baby_coins_earned = (poe_activities as f64) * network_load * 0.001; // Base rate

        // Update PoE statistics
        wallet.poe_stats.total_poe_activities += poe_activities;
        wallet.poe_stats.baby_coins_earned += baby_coins_earned;
        wallet.poe_stats.node_mining_count += 1;
        wallet.poe_stats.logbook_records += poe_activities;
        wallet.poe_stats.last_mining_at = Some(Utc::now());
        wallet.baby_coin_balance += baby_coins_earned;
        wallet.updated_at = Utc::now();

        Ok(baby_coins_earned)
    }

    /// Get wallet by registration ID
    pub async fn get_wallet(&self, registration_id: Uuid) -> Option<RegisteredWallet> {
        let wallets = self.wallets.read().await;
        wallets.get(&registration_id).cloned()
    }

    /// Get all wallets by type
    pub async fn get_wallets_by_type(&self, wallet_type: WalletType) -> Vec<RegisteredWallet> {
        let wallets = self.wallets.read().await;
        wallets.values()
            .filter(|w| w.wallet_type == wallet_type)
            .cloned()
            .collect()
    }

    /// Get owner type statistics
    pub async fn get_owner_type_stats(&self) -> HashMap<OwnerType, u64> {
        let config = self.owner_type_config.read().await;
        config.current_counts.clone()
    }

    /// Get total mother coin allocation
    pub async fn get_total_mother_coin_allocation(&self) -> u64 {
        let wallets = self.wallets.read().await;
        wallets.values()
            .map(|w| w.mother_coin_allocation)
            .sum()
    }

    /// Get total baby coin balance across all wallets
    pub async fn get_total_baby_coin_balance(&self) -> f64 {
        let wallets = self.wallets.read().await;
        wallets.values()
            .map(|w| w.baby_coin_balance)
            .sum()
    }

    /// Validate wallet compliance for mainnet
    pub async fn validate_mainnet_compliance(&self, registration_id: Uuid) -> Result<bool> {
        let wallets = self.wallets.read().await;
        let wallet = wallets.get(&registration_id)
            .ok_or_else(|| anyhow!("Wallet not found"))?;

        Ok(wallet.compliance_status.kyc_verified
            && wallet.compliance_status.aml_cleared
            && wallet.compliance_status.sanctions_cleared
            && wallet.compliance_status.regulatory_approved)
    }
}

impl Default for ComprehensiveWalletRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Wallet registry statistics for monitoring
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletRegistryStats {
    pub total_wallets: u64,
    pub testnet_wallets: u64,
    pub mainnet_wallets: u64,
    pub owner_type_counts: HashMap<OwnerType, u64>,
    pub wallet_type_counts: HashMap<String, u64>,
    pub total_mother_coins: u64,
    pub total_baby_coins: f64,
    pub migration_count: u64,
    pub compliance_rate: f64,
}
