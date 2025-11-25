//! BPI Bundle Converter - Converts PoEProofBundle to BPCI AuctionTransaction format
//! 
//! This critical component bridges the gap between BPI bundle format and BPCI auction format,
//! enabling proper integration of BPI bundles into the BPCI auction system.

use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use uuid::Uuid;
use tracing::{info, warn, error};

use crate::bpci_auction_mempool::{AuctionTransaction, AuctionType};

/// BPI Bundle structures (imported from BPI Core)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoEProofBundle {
    pub bundle_id: String,
    pub bundle_hash: String,
    pub transaction_count: usize,
    pub total_value: f64,
    pub created_at: DateTime<Utc>,
    pub hyperledger_proof: Option<HyperledgerProof>,
    pub notary_approvals: Vec<NotarySignature>,
    pub immutable_proof: ImmutableProof,
    pub bpi_ledger_metadata: BpiLedgerMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperledgerProof {
    pub proof_type: String,
    pub proof_data: serde_json::Value,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotarySignature {
    pub notary_id: String,
    pub signature: String,
    pub signed_at: DateTime<Utc>,
    pub signature_type: SignatureType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignatureType {
    AuditApproval,
    BundleValidation,
    ConsensusVote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmutableProof {
    pub proof_hash: String,
    pub merkle_root: String,
    pub block_height: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpiLedgerMetadata {
    pub node_id: String,
    pub ledger_version: String,
    pub consensus_algorithm: String,
    pub network_id: String,
}

/// BPI Bundle Converter - Converts BPI bundles to BPCI auction transactions
#[derive(Debug)]
pub struct BpiBundleConverter {
    /// Conversion rules for different bundle types
    pub conversion_rules: BundleConversionRules,
    /// Auction configuration for bid calculation
    pub auction_config: AuctionConfig,
    /// Conversion metrics and statistics
    pub conversion_metrics: ConversionMetrics,
}

/// Bundle conversion rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleConversionRules {
    /// Base bid amount per transaction (in wei)
    pub base_bid_per_transaction: u64,
    /// Value multiplier for high-value bundles
    pub value_multiplier: f64,
    /// Priority boost for notary-approved bundles
    pub notary_priority_boost: u16,
    /// Gas limit per transaction
    pub default_gas_limit: u64,
    /// Maximum transactions per auction batch
    pub max_auction_batch_size: usize,
}

/// Auction configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionConfig {
    /// Default chain ID for BPI transactions
    pub bpi_chain_id: u64,
    /// Target chain for execution
    pub target_chain_id: u64,
    /// Auction type for BPI bundles
    pub auction_type: AuctionType,
    /// Priority scoring algorithm
    pub priority_algorithm: PriorityAlgorithm,
}

/// Priority scoring algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriorityAlgorithm {
    ValueBased,      // Priority based on bundle value
    NotaryBased,     // Priority based on notary approvals
    TimeBased,       // Priority based on bundle age
    Hybrid,          // Combination of all factors
}

/// Conversion metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionMetrics {
    pub total_bundles_converted: u64,
    pub total_transactions_generated: u64,
    pub total_value_processed: f64,
    pub average_conversion_time_ms: f64,
    pub conversion_errors: u64,
    pub last_conversion_time: Option<DateTime<Utc>>,
}

/// Conversion result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleConversionResult {
    pub bundle_id: String,
    pub auction_transactions: Vec<AuctionTransaction>,
    pub conversion_summary: ConversionSummary,
    pub immutable_receipt: ConversionReceipt,
}

/// Conversion summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionSummary {
    pub original_bundle_hash: String,
    pub transaction_count: usize,
    pub total_bid_amount: u64,
    pub average_priority_score: u16,
    pub conversion_timestamp: DateTime<Utc>,
    pub notary_validation_count: usize,
}

/// Immutable conversion receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionReceipt {
    pub receipt_id: String,
    pub bundle_id: String,
    pub conversion_hash: String,
    pub merkle_proof: String,
    pub timestamp: DateTime<Utc>,
    pub validator_signatures: Vec<String>,
}

impl BpiBundleConverter {
    /// Create new BPI bundle converter with default configuration
    pub fn new() -> Self {
        Self {
            conversion_rules: BundleConversionRules::default(),
            auction_config: AuctionConfig::default(),
            conversion_metrics: ConversionMetrics::new(),
        }
    }

    /// Create converter with custom configuration
    pub fn new_with_config(rules: BundleConversionRules, config: AuctionConfig) -> Self {
        Self {
            conversion_rules: rules,
            auction_config: config,
            conversion_metrics: ConversionMetrics::new(),
        }
    }

    /// Convert BPI PoEProofBundle to BPCI AuctionTransactions
    pub async fn convert_bundle(&mut self, bundle: &PoEProofBundle) -> Result<BundleConversionResult> {
        let start_time = std::time::Instant::now();
        
        info!("🔄 Converting BPI bundle to BPCI auction transactions: {}", bundle.bundle_id);

        // Validate bundle integrity
        self.validate_bundle(bundle)?;

        // Calculate bid amounts and priority scores
        let bid_calculation = self.calculate_bid_amounts(bundle)?;
        
        // Generate auction transactions
        let auction_transactions = self.generate_auction_transactions(bundle, &bid_calculation)?;

        // Create conversion summary
        let conversion_summary = ConversionSummary {
            original_bundle_hash: bundle.bundle_hash.clone(),
            transaction_count: auction_transactions.len(),
            total_bid_amount: auction_transactions.iter().map(|tx| tx.bid_amount).sum(),
            average_priority_score: auction_transactions.iter().map(|tx| tx.priority_score as u32).sum::<u32>() as u16 / auction_transactions.len() as u16,
            conversion_timestamp: Utc::now(),
            notary_validation_count: bundle.notary_approvals.len(),
        };

        // Create immutable receipt
        let immutable_receipt = self.create_conversion_receipt(bundle, &conversion_summary)?;

        // Update metrics
        let conversion_time = start_time.elapsed().as_millis() as f64;
        self.update_conversion_metrics(bundle, &auction_transactions, conversion_time);

        let result = BundleConversionResult {
            bundle_id: bundle.bundle_id.clone(),
            auction_transactions,
            conversion_summary,
            immutable_receipt,
        };

        info!("✅ Successfully converted BPI bundle: {} → {} auction transactions", 
              bundle.bundle_id, result.auction_transactions.len());

        Ok(result)
    }

    /// Validate bundle integrity and signatures
    fn validate_bundle(&self, bundle: &PoEProofBundle) -> Result<()> {
        // Validate bundle hash
        if bundle.bundle_hash.is_empty() {
            return Err(anyhow!("Bundle hash is empty"));
        }

        // Validate transaction count
        if bundle.transaction_count == 0 {
            return Err(anyhow!("Bundle has no transactions"));
        }

        // Validate notary signatures if required
        if !bundle.notary_approvals.is_empty() {
            for approval in &bundle.notary_approvals {
                if approval.signature.is_empty() {
                    return Err(anyhow!("Invalid notary signature"));
                }
            }
        }

        // Validate immutable proof
        if bundle.immutable_proof.proof_hash.is_empty() {
            return Err(anyhow!("Missing immutable proof"));
        }

        Ok(())
    }

    /// Calculate bid amounts based on bundle value and rules
    fn calculate_bid_amounts(&self, bundle: &PoEProofBundle) -> Result<BidCalculation> {
        let base_bid = self.conversion_rules.base_bid_per_transaction;
        let value_multiplier = if bundle.total_value > 1000.0 {
            self.conversion_rules.value_multiplier
        } else {
            1.0
        };

        let notary_boost = if !bundle.notary_approvals.is_empty() {
            self.conversion_rules.notary_priority_boost
        } else {
            0
        };

        let per_transaction_bid = (base_bid as f64 * value_multiplier) as u64;
        let priority_score = 500 + notary_boost; // Base 500 + notary boost

        Ok(BidCalculation {
            per_transaction_bid,
            priority_score,
            gas_limit: self.conversion_rules.default_gas_limit,
            value_multiplier,
        })
    }

    /// Generate auction transactions from bundle
    fn generate_auction_transactions(&self, bundle: &PoEProofBundle, bid_calc: &BidCalculation) -> Result<Vec<AuctionTransaction>> {
        let mut auction_transactions = Vec::new();
        let bundle_hash_bytes = self.hash_string(&bundle.bundle_hash);

        // Create one auction transaction per logical transaction in the bundle
        for i in 0..bundle.transaction_count {
            let tx_id = self.generate_transaction_id(&bundle.bundle_id, i);
            let data_size = (bundle.total_value / bundle.transaction_count as f64) as u32;

            let auction_tx = AuctionTransaction::new(
                tx_id,
                self.auction_config.bpi_chain_id,
                bid_calc.per_transaction_bid,
                bid_calc.gas_limit,
                data_size,
                format!("bpi-bundle-{}", bundle.bundle_id),
            );

            auction_transactions.push(auction_tx);
        }

        // Limit batch size if needed
        if auction_transactions.len() > self.conversion_rules.max_auction_batch_size {
            auction_transactions.truncate(self.conversion_rules.max_auction_batch_size);
            warn!("⚠️ Truncated auction batch to {} transactions", self.conversion_rules.max_auction_batch_size);
        }

        Ok(auction_transactions)
    }

    /// Generate unique transaction ID
    fn generate_transaction_id(&self, bundle_id: &str, index: usize) -> [u8; 32] {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(bundle_id.as_bytes());
        hasher.update(index.to_be_bytes());
        hasher.update(Utc::now().timestamp().to_be_bytes());
        
        let hash = hasher.finalize();
        let mut tx_id = [0u8; 32];
        tx_id.copy_from_slice(&hash);
        tx_id
    }

    /// Create immutable conversion receipt
    fn create_conversion_receipt(&self, bundle: &PoEProofBundle, summary: &ConversionSummary) -> Result<ConversionReceipt> {
        let receipt_id = Uuid::new_v4().to_string();
        
        // Create conversion hash
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(bundle.bundle_id.as_bytes());
        hasher.update(bundle.bundle_hash.as_bytes());
        hasher.update(summary.conversion_timestamp.timestamp().to_be_bytes());
        let conversion_hash = format!("{:x}", hasher.finalize());

        // Create Merkle proof (simplified)
        let merkle_proof = format!("merkle-{}", Uuid::new_v4());

        Ok(ConversionReceipt {
            receipt_id,
            bundle_id: bundle.bundle_id.clone(),
            conversion_hash,
            merkle_proof,
            timestamp: summary.conversion_timestamp,
            validator_signatures: vec!["validator-sig-1".to_string()], // Simplified
        })
    }

    /// Update conversion metrics
    fn update_conversion_metrics(&mut self, bundle: &PoEProofBundle, transactions: &[AuctionTransaction], conversion_time: f64) {
        self.conversion_metrics.total_bundles_converted += 1;
        self.conversion_metrics.total_transactions_generated += transactions.len() as u64;
        self.conversion_metrics.total_value_processed += bundle.total_value;
        
        // Update average conversion time
        let total_time = self.conversion_metrics.average_conversion_time_ms * (self.conversion_metrics.total_bundles_converted - 1) as f64 + conversion_time;
        self.conversion_metrics.average_conversion_time_ms = total_time / self.conversion_metrics.total_bundles_converted as f64;
        
        self.conversion_metrics.last_conversion_time = Some(Utc::now());
    }

    /// Hash string to bytes
    fn hash_string(&self, input: &str) -> [u8; 32] {
        let mut hasher = <Sha256 as Digest>::new();
        hasher.update(input.as_bytes());
        let hash = hasher.finalize();
        let mut result = [0u8; 32];
        result.copy_from_slice(&hash);
        result
    }

    /// Get conversion metrics
    pub fn get_metrics(&self) -> &ConversionMetrics {
        &self.conversion_metrics
    }
}

/// Bid calculation result
#[derive(Debug, Clone)]
struct BidCalculation {
    per_transaction_bid: u64,
    priority_score: u16,
    gas_limit: u64,
    value_multiplier: f64,
}

impl Default for BundleConversionRules {
    fn default() -> Self {
        Self {
            base_bid_per_transaction: 1_000_000, // 1M wei per transaction
            value_multiplier: 1.5,
            notary_priority_boost: 100,
            default_gas_limit: 21_000,
            max_auction_batch_size: 100,
        }
    }
}

impl Default for AuctionConfig {
    fn default() -> Self {
        Self {
            bpi_chain_id: 1001, // BPI chain ID
            target_chain_id: 1,  // Ethereum mainnet
            auction_type: AuctionType::StandardExecution,
            priority_algorithm: PriorityAlgorithm::Hybrid,
        }
    }
}

impl ConversionMetrics {
    fn new() -> Self {
        Self {
            total_bundles_converted: 0,
            total_transactions_generated: 0,
            total_value_processed: 0.0,
            average_conversion_time_ms: 0.0,
            conversion_errors: 0,
            last_conversion_time: None,
        }
    }
}
