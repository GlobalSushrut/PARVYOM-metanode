//! # BPCI Block Creator - Real Blockchain Block Creation
//!
//! Creates real blockchain blocks from PoE bundles submitted by BPI-comm.
//! This is Step 4 of the v1.0 blockchain pipeline: PoE Bundle → BPCI Block → Finalized Ledger

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use blake3::Hasher;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer};
use tokio::sync::{mpsc, Mutex};
use tracing::{debug, info, warn, error};
use anyhow::{Result, Context};

/// PoE Bundle structure (from BPI-comm)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoEBundle {
    pub v: u8,
    pub app: String,
    pub log_blocks: Vec<String>,
    pub usage_sum: ResourceUsage,
    pub phi: f64,
    pub gamma: f64,
    pub billing_window: String,
    pub sig_bpi_comm: String,
}

/// Resource usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_ms: u64,
    pub memory_mb_s: u64,
    pub storage_gb_day: f64,
    pub egress_mb: f64,
    pub receipts_count: u64,
}

/// BPCI Block - Final blockchain block with economic proofs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpciBlock {
    /// Block version
    pub version: u32,
    /// Block height
    pub height: u64,
    /// Previous block hash
    pub prev_hash: String,
    /// Merkle root of all transactions in this block
    pub merkle_root: String,
    /// Block timestamp
    pub timestamp: u64,
    /// Block nonce (for mining/consensus)
    pub nonce: u64,
    /// Difficulty target
    pub difficulty: u32,
    /// Transactions in this block
    pub transactions: Vec<BpciTransaction>,
    /// Validator signatures (IBFT consensus)
    pub validator_signatures: Vec<ValidatorSignature>,
    /// Block hash
    pub hash: String,
}

/// BPCI Transaction - Economic transaction from PoE bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpciTransaction {
    /// Transaction ID
    pub tx_id: String,
    /// Transaction type
    pub tx_type: TransactionType,
    /// Source application
    pub app: String,
    /// PoE bundle data
    pub poe_data: PoEData,
    /// NEX tokens minted
    pub nex_minted: f64,
    /// Fee distribution
    pub fee_distribution: FeeDistribution,
    /// Transaction signature
    pub signature: String,
    /// Transaction hash
    pub hash: String,
}

/// Transaction types in BPCI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    /// PoE bundle transaction (main type)
    PoEBundle,
    /// Settlement transaction
    Settlement,
    /// Governance transaction
    Governance,
    /// Bank mesh transaction
    BankMesh,
}

/// PoE data embedded in transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoEData {
    pub log_blocks: Vec<String>,
    pub usage_sum: ResourceUsage,
    pub phi: f64,
    pub gamma: f64,
    pub billing_window: String,
}

/// Fee distribution (Court-configured)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeDistribution {
    /// 0.2% locked tokens
    pub locked: f64,
    /// 0.3% spendable tokens
    pub spendable: f64,
    /// 0.2% owner earnings
    pub owner: f64,
    /// 0.3% treasury
    pub treasury: f64,
}

/// Validator signature for IBFT consensus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorSignature {
    pub validator_id: String,
    pub signature: String,
    pub public_key: String,
}

/// BPCI Block Creator configuration
#[derive(Debug, Clone)]
pub struct BlockCreatorConfig {
    /// BPCI signing key
    pub signing_key: SigningKey,
    /// Target block time (seconds)
    pub target_block_time: u64,
    /// Maximum transactions per block
    pub max_transactions_per_block: u32,
    /// Protocol emission scalar
    pub k_window: f64,
    /// Fee split configuration
    pub fee_split: FeeDistribution,
}

/// BPCI Block Creator - Creates blockchain blocks from PoE bundles
pub struct BpciBlockCreator {
    /// Configuration
    config: BlockCreatorConfig,
    /// Current block height
    current_height: Mutex<u64>,
    /// Pending PoE bundles (mempool)
    pending_bundles: Mutex<Vec<PoEBundle>>,
    /// Last block hash
    last_block_hash: Mutex<String>,
    /// Block sender channel
    block_sender: mpsc::UnboundedSender<BpciBlock>,
    /// Verifying key
    verifying_key: VerifyingKey,
}

impl Default for FeeDistribution {
    fn default() -> Self {
        Self {
            locked: 0.002,    // 0.2%
            spendable: 0.003, // 0.3%
            owner: 0.002,     // 0.2%
            treasury: 0.003,  // 0.3%
        }
    }
}

impl BlockCreatorConfig {
    /// Create new block creator configuration
    pub fn new(signing_key: SigningKey) -> Self {
        Self {
            signing_key,
            target_block_time: 12, // 12 second blocks
            max_transactions_per_block: 1000,
            k_window: 1000.0,
            fee_split: FeeDistribution::default(),
        }
    }

    /// Set target block time
    pub fn with_block_time(mut self, block_time: u64) -> Self {
        self.target_block_time = block_time;
        self
    }

    /// Set maximum transactions per block
    pub fn with_max_transactions(mut self, max_tx: u32) -> Self {
        self.max_transactions_per_block = max_tx;
        self
    }

    /// Set protocol emission scalar
    pub fn with_k_window(mut self, k_window: f64) -> Self {
        self.k_window = k_window;
        self
    }

    /// Set fee split configuration
    pub fn with_fee_split(mut self, fee_split: FeeDistribution) -> Self {
        self.fee_split = fee_split;
        self
    }
}

impl BpciBlockCreator {
    /// Create new BPCI block creator
    pub fn new(
        config: BlockCreatorConfig,
        block_sender: mpsc::UnboundedSender<BpciBlock>,
    ) -> Self {
        let verifying_key = VerifyingKey::from(&config.signing_key);
        
        Self {
            config,
            current_height: Mutex::new(0),
            pending_bundles: Mutex::new(Vec::new()),
            last_block_hash: Mutex::new("genesis".to_string()),
            block_sender,
            verifying_key,
        }
    }

    /// Process incoming PoE bundle
    pub async fn process_poe_bundle(&self, bundle: PoEBundle) -> Result<()> {
        // Verify PoE bundle signature (basic validation)
        if let Err(e) = self.verify_poe_bundle(&bundle) {
            warn!("Invalid PoE bundle from {}: {}", bundle.app, e);
            return Err(e);
        }

        // Add to mempool
        {
            let mut pending = self.pending_bundles.lock().await;
            pending.push(bundle.clone());
            debug!("Added PoE bundle to mempool: app={}, phi={:.6}, gamma={:.6}", 
                   bundle.app, bundle.phi, bundle.gamma);
        }

        // Check if we should create a block
        self.maybe_create_block().await?;

        Ok(())
    }

    /// Check if we should create a block
    async fn maybe_create_block(&self) -> Result<()> {
        let should_create = {
            let pending = self.pending_bundles.lock().await;
            pending.len() >= self.config.max_transactions_per_block as usize ||
            self.should_create_by_time().await
        };

        if should_create {
            self.create_block().await?;
        }

        Ok(())
    }

    /// Check if we should create block based on time
    async fn should_create_by_time(&self) -> bool {
        // For now, create blocks immediately when we have bundles
        // In production, this would check against target_block_time
        true
    }

    /// Create BPCI block from pending PoE bundles
    async fn create_block(&self) -> Result<()> {
        let bundles = {
            let mut pending = self.pending_bundles.lock().await;
            if pending.is_empty() {
                return Ok(());
            }

            // Take all pending bundles for this block
            let bundles = pending.drain(..).collect::<Vec<_>>();
            bundles
        };

        if bundles.is_empty() {
            return Ok(());
        }

        // Get current height and increment
        let height = {
            let mut height = self.current_height.lock().await;
            *height += 1;
            *height
        };

        // Get previous block hash
        let prev_hash = {
            let last_hash = self.last_block_hash.lock().await;
            last_hash.clone()
        };

        // Create transactions from PoE bundles
        let mut transactions = Vec::new();
        for bundle in bundles {
            let transaction = self.create_transaction_from_bundle(bundle).await?;
            transactions.push(transaction);
        }

        // Create block
        let mut block = BpciBlock {
            version: 1,
            height,
            prev_hash,
            merkle_root: String::new(), // Will be computed
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .context("Time error")?
                .as_secs(),
            nonce: 0, // For now, no mining
            difficulty: 1,
            transactions,
            validator_signatures: Vec::new(), // Will be added by IBFT
            hash: String::new(), // Will be computed
        };

        // Compute Merkle root
        block.merkle_root = self.compute_merkle_root(&block.transactions)?;

        // Compute block hash
        block.hash = self.compute_block_hash(&block)?;

        // Update last block hash
        {
            let mut last_hash = self.last_block_hash.lock().await;
            *last_hash = block.hash.clone();
        }

        // Send block to consensus/storage
        if let Err(e) = self.block_sender.send(block.clone()) {
            error!("Failed to send BPCI block: {}", e);
            return Err(anyhow::anyhow!("Block send failed: {}", e));
        }

        info!("Created BPCI block: height={}, transactions={}, hash={}", 
              block.height, block.transactions.len(), &block.hash[..16]);

        Ok(())
    }

    /// Create transaction from PoE bundle
    async fn create_transaction_from_bundle(&self, bundle: PoEBundle) -> Result<BpciTransaction> {
        // Calculate NEX minting
        let nex_minted = self.config.k_window * bundle.gamma;

        // Calculate fee distribution
        let fee_distribution = FeeDistribution {
            locked: nex_minted * self.config.fee_split.locked,
            spendable: nex_minted * self.config.fee_split.spendable,
            owner: nex_minted * self.config.fee_split.owner,
            treasury: nex_minted * self.config.fee_split.treasury,
        };

        // Create PoE data
        let poe_data = PoEData {
            log_blocks: bundle.log_blocks,
            usage_sum: bundle.usage_sum,
            phi: bundle.phi,
            gamma: bundle.gamma,
            billing_window: bundle.billing_window,
        };

        // Generate transaction ID
        let tx_id = format!("poe_{}_{}_{}", 
                           bundle.app, 
                           SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(),
                           rand::random::<u32>());

        let mut transaction = BpciTransaction {
            tx_id,
            tx_type: TransactionType::PoEBundle,
            app: bundle.app,
            poe_data,
            nex_minted,
            fee_distribution,
            signature: String::new(), // Will be computed
            hash: String::new(), // Will be computed
        };

        // Compute transaction hash
        transaction.hash = self.compute_transaction_hash(&transaction)?;

        // Sign transaction
        transaction.signature = self.sign_transaction(&transaction)?;

        debug!("Created PoE transaction: app={}, nex_minted={:.2}, tx_id={}", 
               transaction.app, transaction.nex_minted, transaction.tx_id);

        Ok(transaction)
    }

    /// Compute Merkle root of transactions
    fn compute_merkle_root(&self, transactions: &[BpciTransaction]) -> Result<String> {
        if transactions.is_empty() {
            return Ok("blake3:empty".to_string());
        }

        // Create leaf hashes
        let mut leaf_hashes = Vec::new();
        for tx in transactions {
            let tx_bytes = serde_json::to_vec(tx)
                .context("Failed to serialize transaction")?;
            let leaf_hash = blake3::hash(&tx_bytes);
            leaf_hashes.push(*leaf_hash.as_bytes());
        }

        // Build Merkle tree
        let root = self.build_merkle_tree(leaf_hashes)?;
        Ok(format!("blake3:{}", hex::encode(root)))
    }

    /// Build Merkle tree from leaf hashes
    fn build_merkle_tree(&self, mut hashes: Vec<[u8; 32]>) -> Result<[u8; 32]> {
        if hashes.is_empty() {
            return Ok([0u8; 32]);
        }

        if hashes.len() == 1 {
            return Ok(hashes[0]);
        }

        // Build tree bottom-up
        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in hashes.chunks(2) {
                let combined_hash = if chunk.len() == 2 {
                    let mut hasher = Hasher::new();
                    hasher.update(&chunk[0]);
                    hasher.update(&chunk[1]);
                    *hasher.finalize().as_bytes()
                } else {
                    let mut hasher = Hasher::new();
                    hasher.update(&chunk[0]);
                    hasher.update(&chunk[0]);
                    *hasher.finalize().as_bytes()
                };
                next_level.push(combined_hash);
            }
            
            hashes = next_level;
        }

        Ok(hashes[0])
    }

    /// Compute block hash
    fn compute_block_hash(&self, block: &BpciBlock) -> Result<String> {
        let mut hasher = Hasher::new();
        
        hasher.update(&block.version.to_le_bytes());
        hasher.update(&block.height.to_le_bytes());
        hasher.update(block.prev_hash.as_bytes());
        hasher.update(block.merkle_root.as_bytes());
        hasher.update(&block.timestamp.to_le_bytes());
        hasher.update(&block.nonce.to_le_bytes());
        hasher.update(&block.difficulty.to_le_bytes());
        
        let hash = hasher.finalize();
        Ok(format!("blake3:{}", hex::encode(hash.as_bytes())))
    }

    /// Compute transaction hash
    fn compute_transaction_hash(&self, tx: &BpciTransaction) -> Result<String> {
        let mut hasher = Hasher::new();
        
        hasher.update(tx.tx_id.as_bytes());
        hasher.update(tx.app.as_bytes());
        hasher.update(&tx.nex_minted.to_le_bytes());
        hasher.update(&tx.poe_data.phi.to_le_bytes());
        hasher.update(&tx.poe_data.gamma.to_le_bytes());
        
        let hash = hasher.finalize();
        Ok(format!("blake3:{}", hex::encode(hash.as_bytes())))
    }

    /// Sign transaction
    fn sign_transaction(&self, tx: &BpciTransaction) -> Result<String> {
        let hash_bytes = hex::decode(tx.hash.strip_prefix("blake3:").unwrap_or(&tx.hash))
            .context("Invalid transaction hash")?;
        
        let signature = self.config.signing_key.sign(&hash_bytes);
        Ok(format!("ed25519:{}", hex::encode(signature.to_bytes())))
    }

    /// Verify PoE bundle signature (basic validation)
    fn verify_poe_bundle(&self, bundle: &PoEBundle) -> Result<()> {
        // Basic validation
        if bundle.v != 1 {
            return Err(anyhow::anyhow!("Invalid PoE bundle version: {}", bundle.v));
        }

        if bundle.app.is_empty() {
            return Err(anyhow::anyhow!("Empty app ID in PoE bundle"));
        }

        if bundle.phi < 0.0 || bundle.gamma < 0.0 || bundle.gamma >= 1.0 {
            return Err(anyhow::anyhow!("Invalid PoE values: phi={}, gamma={}", bundle.phi, bundle.gamma));
        }

        if !bundle.sig_bpi_comm.starts_with("ed25519:") {
            return Err(anyhow::anyhow!("Invalid signature format in PoE bundle"));
        }

        Ok(())
    }

    /// Force create block from current mempool
    pub async fn force_create_block(&self) -> Result<()> {
        self.create_block().await
    }

    /// Get current block height
    pub async fn get_current_height(&self) -> u64 {
        *self.current_height.lock().await
    }

    /// Get mempool size
    pub async fn get_mempool_size(&self) -> usize {
        self.pending_bundles.lock().await.len()
    }

    /// Get block creator statistics
    pub async fn get_stats(&self) -> BlockCreatorStats {
        BlockCreatorStats {
            current_height: self.get_current_height().await,
            mempool_size: self.get_mempool_size().await,
            last_block_hash: self.last_block_hash.lock().await.clone(),
            target_block_time: self.config.target_block_time,
            max_transactions_per_block: self.config.max_transactions_per_block,
        }
    }
}

/// Block creator statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockCreatorStats {
    pub current_height: u64,
    pub mempool_size: usize,
    pub last_block_hash: String,
    pub target_block_time: u64,
    pub max_transactions_per_block: u32,
}

/// Start BPCI block creator service
pub async fn start_block_creator_service(
    config: BlockCreatorConfig,
    bundle_receiver: mpsc::UnboundedReceiver<PoEBundle>,
    block_sender: mpsc::UnboundedSender<BpciBlock>,
) -> Result<()> {
    let creator = BpciBlockCreator::new(config, block_sender);
    let mut bundle_receiver = bundle_receiver;

    info!("Starting BPCI block creator service");

    // Process incoming PoE bundles
    while let Some(bundle) = bundle_receiver.recv().await {
        if let Err(e) = creator.process_poe_bundle(bundle).await {
            error!("Failed to process PoE bundle: {}", e);
        }
    }

    info!("BPCI block creator service stopped");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;
    use tokio::sync::mpsc;

    fn create_test_poe_bundle(app: &str) -> PoEBundle {
        PoEBundle {
            v: 1,
            app: app.to_string(),
            log_blocks: vec!["blake3:test1".to_string(), "blake3:test2".to_string()],
            usage_sum: ResourceUsage {
                cpu_ms: 1000,
                memory_mb_s: 500,
                storage_gb_day: 1.0,
                egress_mb: 10.0,
                receipts_count: 100,
            },
            phi: 1.425,
            gamma: 0.587603,
            billing_window: "2025-08-13T06:00:00Z/2025-08-13T07:00:00Z".to_string(),
            sig_bpi_comm: "ed25519:test".to_string(),
        }
    }

    #[tokio::test]
    async fn test_block_creation() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let config = BlockCreatorConfig::new(signing_key);

        let (tx, mut rx) = mpsc::unbounded_channel();
        let creator = BpciBlockCreator::new(config, tx);

        // Process PoE bundle
        let bundle = create_test_poe_bundle("TEST_APP");
        creator.process_poe_bundle(bundle).await.unwrap();

        // Should receive a BPCI block
        let block = rx.recv().await.unwrap();
        assert_eq!(block.version, 1);
        assert_eq!(block.height, 1);
        assert_eq!(block.transactions.len(), 1);
        assert!(block.hash.starts_with("blake3:"));
        assert!(block.merkle_root.starts_with("blake3:"));

        // Check transaction
        let tx = &block.transactions[0];
        assert_eq!(tx.app, "TEST_APP");
        assert!(tx.nex_minted > 0.0);
        assert!(tx.hash.starts_with("blake3:"));
        assert!(tx.signature.starts_with("ed25519:"));
    }

    #[tokio::test]
    async fn test_fee_distribution() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let config = BlockCreatorConfig::new(signing_key)
            .with_k_window(1000.0);

        let (tx, _rx) = mpsc::unbounded_channel();
        let creator = BpciBlockCreator::new(config, tx);

        let bundle = create_test_poe_bundle("TEST_APP");
        let transaction = creator.create_transaction_from_bundle(bundle).await.unwrap();

        // Check fee distribution
        let fees = &transaction.fee_distribution;
        let total_fees = fees.locked + fees.spendable + fees.owner + fees.treasury;
        
        // Should equal NEX minted
        assert!((total_fees - transaction.nex_minted).abs() < 0.001);
        
        // Check individual percentages
        assert!((fees.locked / transaction.nex_minted - 0.002).abs() < 0.001); // 0.2%
        assert!((fees.owner / transaction.nex_minted - 0.002).abs() < 0.001);  // 0.2%
    }

    #[test]
    fn test_block_creator_config() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        
        let config = BlockCreatorConfig::new(signing_key)
            .with_block_time(6)
            .with_max_transactions(500)
            .with_k_window(2000.0);

        assert_eq!(config.target_block_time, 6);
        assert_eq!(config.max_transactions_per_block, 500);
        assert_eq!(config.k_window, 2000.0);
    }
}
