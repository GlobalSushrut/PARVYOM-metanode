use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use thiserror::Error;
use uuid::Uuid;

use crate::autonomous_economy::CoinType;
use crate::registry::node_types::{BpiWalletStamp, BankType, BankLicense};

/// Settlement Coin (SC4/AUR) - Bank-to-Bank Settlement Infrastructure
/// 
/// Implements practical settlement logic with NFT claim proofs and PoE auditability:
/// 1. Consumer → Bank A (Claim Initiation + NFT Receipt)
/// 2. Bank A → Bank B (SC4 Settlement Coin Transfer)
/// 3. Bank B (Clearing Process + NFT Verification)
/// 4. Final Settlement (SC4 Burn + Fiat Reconciliation)

/// NFT Claim Token representing consumer fiat payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftClaimToken {
    /// Unique NFT token ID
    pub token_id: String,
    /// Consumer who initiated the claim
    pub consumer_id: String,
    /// Originating bank that received fiat
    pub originating_bank_id: String,
    /// Destination bank for settlement
    pub destination_bank_id: String,
    /// Fiat amount claimed
    pub fiat_amount: Decimal,
    /// Currency code (USD, EUR, etc.)
    pub currency_code: String,
    /// PoE proof of the claim transaction
    pub poe_proof: PoEProof,
    /// Timestamp when claim was initiated
    pub claim_timestamp: DateTime<Utc>,
    /// Current status of the NFT claim
    pub status: NftClaimStatus,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Settlement Transaction for bank-to-bank transfers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementTransaction {
    pub transaction_id: String,
    pub from_bank_id: String,
    pub to_bank_id: String,
    pub settlement_coin_id: String,
    pub amount: Decimal,
    pub timestamp: DateTime<Utc>,
    pub status: SettlementTransactionStatus,
}

/// Bank Settlement record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankSettlement {
    pub settlement_id: String,
    pub bank_a_id: String,
    pub bank_b_id: String,
    pub total_amount: Decimal,
    pub currency_code: String,
    pub settlement_coins: Vec<String>,
    pub phase: SettlementPhase,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Settlement Phase enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SettlementPhase {
    Initiated,
    CoinTransfer,
    Clearing,
    Completed,
    Failed,
}

/// Settlement Transaction Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SettlementTransactionStatus {
    Pending,
    Confirmed,
    Failed,
    Cancelled,
}

/// Active Settlement for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveSettlement {
    pub settlement_id: String,
    pub bank_a_id: String,
    pub bank_b_id: String,
    pub total_amount: Decimal,
    pub currency_code: String,
    pub phase: SettlementPhase,
    pub progress_percentage: u8,
    pub estimated_completion: DateTime<Utc>,
}

/// Consumer Payment record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerPayment {
    pub payment_id: String,
    pub consumer_id: String,
    pub merchant_id: String,
    pub amount: Decimal,
    pub currency_code: String,
    pub payment_method: String,
    pub timestamp: DateTime<Utc>,
    pub nft_claim_id: Option<String>,
}

/// PoE (Proof of Existence) for claim verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoEProof {
    /// Hash of the original transaction
    pub transaction_hash: String,
    /// Cryptographic signature from originating bank
    pub bank_signature: String,
    /// Consumer identity verification hash
    pub consumer_identity_hash: String,
    /// Merkle proof for blockchain inclusion
    pub merkle_proof: Vec<String>,
    /// Block height where proof was recorded
    pub block_height: u64,
}

/// NFT Claim Token status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NftClaimStatus {
    /// NFT minted, awaiting SC4 transfer
    Minted,
    /// SC4 transfer initiated
    TransferInitiated,
    /// Received by destination bank, awaiting verification
    ReceivedPendingVerification,
    /// PoE proof verified successfully
    Verified,
    /// Cleared and ready for settlement
    Cleared,
    /// Final settlement completed, NFT archived
    Settled,
    /// Rejected due to verification failure
    Rejected { reason: String },
    /// Expired without settlement
    Expired,
}

/// Settlement Coin (SC4) - Bank-to-Bank Transfer Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementCoin {
    /// Unique SC4 coin ID
    pub coin_id: String,
    /// Associated NFT claim token ID
    pub nft_claim_id: String,
    /// SC4 amount (equals fiat claim amount)
    pub amount: Decimal,
    /// Currency being settled
    pub currency_code: String,
    /// Bank that issued the SC4
    pub issuing_bank_id: String,
    /// Bank that will receive the SC4
    pub receiving_bank_id: String,
    /// Timestamp when SC4 was created
    pub created_at: DateTime<Utc>,
    /// Current status of SC4
    pub status: SettlementCoinStatus,
    /// Lock expiry (SC4 auto-expires if not settled)
    pub lock_expiry: DateTime<Utc>,
}

/// Settlement Coin status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SettlementCoinStatus {
    /// SC4 created and ready for transfer
    Created,
    /// SC4 transferred to receiving bank
    Transferred,
    /// SC4 locked in receiving bank vault
    Locked,
    /// SC4 verified and ready for settlement
    Verified,
    /// SC4 burned after final settlement
    Burned,
    /// SC4 expired and returned to issuer
    Expired,
}

/// Bank Vault Ledger for SC4 management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankVaultLedger {
    /// Bank identifier
    pub bank_id: String,
    /// Bank license and regulatory info
    pub bank_license: BankLicense,
    /// Active SC4 coins held in vault
    pub active_sc4_coins: HashMap<String, SettlementCoin>,
    /// Total SC4 value locked
    pub total_locked_value: Decimal,
    /// Pending settlements
    pub pending_settlements: Vec<String>,
    /// Settlement history
    pub settlement_history: Vec<SettlementRecord>,
    /// Last reconciliation timestamp
    pub last_reconciliation: DateTime<Utc>,
}

/// Settlement record for audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementRecord {
    /// Settlement transaction ID
    pub settlement_id: String,
    /// NFT claim that was settled
    pub nft_claim_id: String,
    /// SC4 coin that was burned
    pub sc4_coin_id: String,
    /// Settlement amount
    pub amount: Decimal,
    /// Currency settled
    pub currency_code: String,
    /// Counterparty bank
    pub counterparty_bank_id: String,
    /// Settlement timestamp
    pub settled_at: DateTime<Utc>,
    /// Final PoE proof of settlement
    pub settlement_proof: String,
}

/// Settlement system errors
#[derive(Error, Debug)]
pub enum SettlementError {
    #[error("Unauthorized bank access: {bank_id}")]
    UnauthorizedBankAccess { bank_id: String },
    #[error("Invalid NFT claim: {claim_id}")]
    InvalidNftClaim { claim_id: String },
    #[error("SC4 coin not found: {coin_id}")]
    Sc4CoinNotFound { coin_id: String },
    #[error("PoE verification failed: {reason}")]
    PoEVerificationFailed { reason: String },
    #[error("Settlement amount mismatch: expected {expected}, got {actual}")]
    SettlementAmountMismatch { expected: Decimal, actual: Decimal },
    #[error("Settlement expired: {settlement_id}")]
    SettlementExpired { settlement_id: String },
    #[error("Regulatory compliance violation: {violation}")]
    ComplianceViolation { violation: String },
}

/// Settlement Coin Engine - Core settlement infrastructure
#[derive(Debug)]
pub struct SettlementCoinEngine {
    /// Bank vault ledgers for SC4 management
    bank_vaults: Arc<RwLock<HashMap<String, BankVaultLedger>>>,
    /// NFT claim tokens registry
    nft_claims: Arc<RwLock<HashMap<String, NftClaimToken>>>,
    /// Settlement coins registry
    sc4_coins: Arc<RwLock<HashMap<String, SettlementCoin>>>,
    /// Settlement records for audit trail
    settlement_records: Arc<RwLock<Vec<SettlementRecord>>>,
    /// System configuration
    config: SettlementConfig,
    /// Settlement statistics
    stats: Arc<RwLock<SettlementStats>>,
}

/// Settlement system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementConfig {
    /// Maximum SC4 lock duration (hours)
    pub max_lock_duration_hours: u32,
    /// Settlement cycle frequency (hours)
    pub settlement_cycle_hours: u32,
    /// Minimum settlement amount
    pub minimum_settlement_amount: Decimal,
    /// Maximum settlement amount per transaction
    pub maximum_settlement_amount: Decimal,
    /// PoE verification timeout (minutes)
    pub poe_verification_timeout_minutes: u32,
    /// Regulatory compliance checks enabled
    pub compliance_checks_enabled: bool,
}

impl Default for SettlementConfig {
    fn default() -> Self {
        Self {
            max_lock_duration_hours: 24,        // 24 hour max lock
            settlement_cycle_hours: 24,         // Daily settlement cycles
            minimum_settlement_amount: Decimal::new(100, 0),  // $100 minimum
            maximum_settlement_amount: Decimal::new(10_000_000, 0), // $10M maximum
            poe_verification_timeout_minutes: 30, // 30 minute verification timeout
            compliance_checks_enabled: true,
        }
    }
}

/// Settlement system statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementStats {
    pub total_nft_claims_minted: u64,
    pub total_sc4_coins_created: u64,
    pub total_sc4_coins_burned: u64,
    pub total_settlement_value: Decimal,
    pub active_settlements: u64,
    pub failed_settlements: u64,
    pub average_settlement_time_minutes: f64,
}

impl SettlementCoinEngine {
    /// Create new settlement coin engine
    pub fn new(config: SettlementConfig) -> Self {
        Self {
            bank_vaults: Arc::new(RwLock::new(HashMap::new())),
            nft_claims: Arc::new(RwLock::new(HashMap::new())),
            sc4_coins: Arc::new(RwLock::new(HashMap::new())),
            settlement_records: Arc::new(RwLock::new(Vec::new())),
            config,
            stats: Arc::new(RwLock::new(SettlementStats {
                total_nft_claims_minted: 0,
                total_sc4_coins_created: 0,
                total_sc4_coins_burned: 0,
                total_settlement_value: Decimal::ZERO,
                active_settlements: 0,
                failed_settlements: 0,
                average_settlement_time_minutes: 0.0,
            })),
        }
    }
    
    /// Register authorized bank for settlement operations
    pub async fn register_bank(&self, bank_id: String, bank_license: BankLicense) -> Result<(), SettlementError> {
        // Verify bank authorization (only real banks allowed)
        self.verify_bank_authorization(&bank_id, &bank_license)?;
        
        let vault_ledger = BankVaultLedger {
            bank_id: bank_id.clone(),
            bank_license,
            active_sc4_coins: HashMap::new(),
            total_locked_value: Decimal::ZERO,
            pending_settlements: Vec::new(),
            settlement_history: Vec::new(),
            last_reconciliation: Utc::now(),
        };
        
        let mut vaults = self.bank_vaults.write().await;
        vaults.insert(bank_id, vault_ledger);
        
        Ok(())
    }
    
    /// Step 1: Consumer → Bank A (Claim Initiation + NFT Receipt)
    pub async fn initiate_claim(
        &self,
        consumer_id: String,
        originating_bank_id: String,
        destination_bank_id: String,
        fiat_amount: Decimal,
        currency_code: String,
        consumer_identity_hash: String,
    ) -> Result<NftClaimToken, SettlementError> {
        // Verify both banks are authorized
        self.verify_bank_exists(&originating_bank_id).await?;
        self.verify_bank_exists(&destination_bank_id).await?;
        
        // Validate settlement amount
        if fiat_amount < self.config.minimum_settlement_amount {
            return Err(SettlementError::SettlementAmountMismatch {
                expected: self.config.minimum_settlement_amount,
                actual: fiat_amount,
            });
        }
        
        if fiat_amount > self.config.maximum_settlement_amount {
            return Err(SettlementError::SettlementAmountMismatch {
                expected: self.config.maximum_settlement_amount,
                actual: fiat_amount,
            });
        }
        
        // Generate PoE proof
        let poe_proof = self.generate_poe_proof(
            &consumer_id,
            &originating_bank_id,
            fiat_amount,
            &consumer_identity_hash,
        ).await?;
        
        // Mint NFT Claim Token
        let nft_claim = NftClaimToken {
            token_id: Uuid::new_v4().to_string(),
            consumer_id,
            originating_bank_id,
            destination_bank_id,
            fiat_amount,
            currency_code,
            poe_proof,
            claim_timestamp: Utc::now(),
            status: NftClaimStatus::Minted,
            metadata: HashMap::new(),
        };
        
        // Store NFT claim
        let mut claims = self.nft_claims.write().await;
        claims.insert(nft_claim.token_id.clone(), nft_claim.clone());
        
        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_nft_claims_minted += 1;
        
        Ok(nft_claim)
    }
    
    /// Step 2: Bank A → Bank B (SC4 Settlement Coin Creation + Transfer)
    pub async fn create_and_transfer_sc4(
        &self,
        nft_claim_id: String,
        issuing_bank_id: String,
    ) -> Result<SettlementCoin, SettlementError> {
        // Verify issuing bank authorization
        self.verify_bank_exists(&issuing_bank_id).await?;
        
        // Get NFT claim
        let mut claims = self.nft_claims.write().await;
        let nft_claim = claims.get_mut(&nft_claim_id)
            .ok_or_else(|| SettlementError::InvalidNftClaim { claim_id: nft_claim_id.clone() })?;
            
        // Verify bank is authorized to issue SC4 for this claim
        if nft_claim.originating_bank_id != issuing_bank_id {
            return Err(SettlementError::UnauthorizedBankAccess { bank_id: issuing_bank_id });
        }
        
        // Create SC4 Settlement Coin
        let sc4_coin = SettlementCoin {
            coin_id: Uuid::new_v4().to_string(),
            nft_claim_id: nft_claim_id.clone(),
            amount: nft_claim.fiat_amount,
            currency_code: nft_claim.currency_code.clone(),
            issuing_bank_id: issuing_bank_id.clone(),
            receiving_bank_id: nft_claim.destination_bank_id.clone(),
            created_at: Utc::now(),
            status: SettlementCoinStatus::Created,
            lock_expiry: Utc::now() + Duration::hours(self.config.max_lock_duration_hours as i64),
        };
        
        // Transfer SC4 to receiving bank
        let mut sc4_coins = self.sc4_coins.write().await;
        sc4_coins.insert(sc4_coin.coin_id.clone(), sc4_coin.clone());
        
        // Update NFT claim status
        nft_claim.status = NftClaimStatus::TransferInitiated;
        
        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_sc4_coins_created += 1;
        stats.active_settlements += 1;
        
        Ok(sc4_coin)
    }
    
    /// Verify bank authorization (only real financial institutions)
    fn verify_bank_authorization(&self, bank_id: &str, bank_license: &BankLicense) -> Result<(), SettlementError> {
        if bank_id.is_empty() {
            return Err(SettlementError::UnauthorizedBankAccess { bank_id: bank_id.to_string() });
        }
        
        // Simulate regulatory compliance check
        if self.config.compliance_checks_enabled {
            // Verify bank license is valid and not expired
            // This would integrate with real regulatory databases
        }
        
        Ok(())
    }
    
    /// Verify bank exists in the system
    async fn verify_bank_exists(&self, bank_id: &str) -> Result<(), SettlementError> {
        let vaults = self.bank_vaults.read().await;
        if !vaults.contains_key(bank_id) {
            return Err(SettlementError::UnauthorizedBankAccess { bank_id: bank_id.to_string() });
        }
        Ok(())
    }
    
    /// Generate PoE (Proof of Existence) for claim
    async fn generate_poe_proof(
        &self,
        consumer_id: &str,
        bank_id: &str,
        amount: Decimal,
        consumer_identity_hash: &str,
    ) -> Result<PoEProof, SettlementError> {
        let transaction_data = format!("{}:{}:{}:{}", consumer_id, bank_id, amount, Utc::now().timestamp());
        let transaction_hash = format!("0x{:x}", md5::compute(transaction_data.as_bytes()));
        
        Ok(PoEProof {
            transaction_hash,
            bank_signature: format!("BANK_SIG_{}_{}", bank_id, Utc::now().timestamp()),
            consumer_identity_hash: consumer_identity_hash.to_string(),
            merkle_proof: vec![
                "0xmerkle_root_hash".to_string(),
                "0xmerkle_branch_hash".to_string(),
            ],
            block_height: 12345, // Would be real blockchain height
        })
    }
    
    /// Get settlement statistics
    pub async fn get_settlement_stats(&self) -> SettlementStats {
        self.stats.read().await.clone()
    }
    
    /// Get bank vault status
    pub async fn get_bank_vault_status(&self, bank_id: &str) -> Option<BankVaultLedger> {
        let vaults = self.bank_vaults.read().await;
        vaults.get(bank_id).cloned()
    }

    /// Create settlement coin (SC4/AUR) for bank-to-bank transfer
    pub async fn create_settlement_coin(
        &self,
        bank_id: &str,
        amount: Decimal,
        currency_code: &str,
        reference_id: &str,
    ) -> Result<SettlementCoin, SettlementError> {
        // Verify bank authorization
        self.verify_bank_exists(bank_id).await?;
        
        let coin_id = format!("SC4_{}_{}_{}", bank_id, Utc::now().timestamp(), reference_id);
        
        let settlement_coin = SettlementCoin {
            coin_id: coin_id.clone(),
            nft_claim_id: reference_id.to_string(),
            amount,
            currency_code: currency_code.to_string(),
            issuing_bank_id: bank_id.to_string(),
            receiving_bank_id: String::new(), // Set during transfer
            created_at: Utc::now(),
            status: SettlementCoinStatus::Created,
            lock_expiry: Utc::now() + Duration::hours(24),
        };

        // Update bank vault
        let mut vaults = self.bank_vaults.write().await;
        if let Some(vault) = vaults.get_mut(bank_id) {
            vault.active_sc4_coins.insert(coin_id.clone(), settlement_coin.clone());
            vault.total_locked_value += amount;
            vault.pending_settlements.push(coin_id.clone());
        }

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_sc4_coins_created += 1;
        stats.active_settlements += 1;

        Ok(settlement_coin)
    }

    /// Transfer settlement coin between banks
    pub async fn transfer_settlement_coin(
        &self,
        coin_id: &str,
        from_bank_id: &str,
        to_bank_id: &str,
    ) -> Result<(), SettlementError> {
        // Verify both banks exist
        self.verify_bank_exists(from_bank_id).await?;
        self.verify_bank_exists(to_bank_id).await?;

        // Update bank vaults
        let mut vaults = self.bank_vaults.write().await;
        
        // Debit from source bank
        if let Some(from_vault) = vaults.get_mut(from_bank_id) {
            from_vault.active_sc4_coins.remove(coin_id);
        }
        
        // Credit to destination bank
        if let Some(to_vault) = vaults.get_mut(to_bank_id) {
            to_vault.pending_settlements.push(coin_id.to_string());
        }

        // Create settlement record
        let settlement_record = SettlementRecord {
            settlement_id: format!("SETTLE_{}_{}", coin_id, Utc::now().timestamp()),
            nft_claim_id: coin_id.to_string(),
            sc4_coin_id: coin_id.to_string(),
            amount: Decimal::ZERO, // Would be set from actual coin data
            currency_code: "USD".to_string(), // Would be from actual coin data
            counterparty_bank_id: to_bank_id.to_string(),
            settled_at: Utc::now(),
            settlement_proof: format!("PROOF_{}_{}", from_bank_id, to_bank_id),
        };

        let mut records = self.settlement_records.write().await;
        records.push(settlement_record);

        Ok(())
    }

    /// Burn settlement coin after successful settlement
    pub async fn burn_settlement_coin(
        &self,
        coin_id: &str,
        bank_id: &str,
    ) -> Result<(), SettlementError> {
        // Verify bank authorization
        self.verify_bank_exists(bank_id).await?;

        // Update bank vault
        let mut vaults = self.bank_vaults.write().await;
        if let Some(vault) = vaults.get_mut(bank_id) {
            vault.active_sc4_coins.remove(coin_id);
            vault.pending_settlements.retain(|s| s != coin_id);
        }

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_sc4_coins_burned += 1;
        stats.active_settlements = stats.active_settlements.saturating_sub(1);

        Ok(())
    }

    /// Validate bank settlement operation
    pub async fn validate_bank_settlement(
        &self,
        bank_id: &str,
        settlement_id: &str,
    ) -> Result<bool, SettlementError> {
        // Verify bank exists
        self.verify_bank_exists(bank_id).await?;

        // Check settlement records
        let records = self.settlement_records.read().await;
        let settlement_exists = records.iter().any(|record| {
            record.settlement_id == settlement_id &&
            record.counterparty_bank_id == bank_id
        });

        Ok(settlement_exists)
    }
}
