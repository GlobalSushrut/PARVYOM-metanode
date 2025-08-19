//! # ENC Notary - LogBlock Aggregation
//!
//! ENC-notary aggregates StepReceipts into LogBlocks with Merkle roots.
//! This is Step 2 of the v1.0 blockchain pipeline: StepReceipt → LogBlock → PoE → BPCI

use blake3::Hasher;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use tokio::sync::{mpsc, Mutex};
use tracing::{debug, info, warn, error};
use anyhow::{Result, Context};

/// StepReceipt structure (matches DockLock implementation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepReceipt {
    pub v: u8,
    pub app: String,
    pub container: String,
    pub op: String,
    pub ts: String,
    pub usage: ResourceUsage,
    pub labels: std::collections::HashMap<String, String>,
    pub prev_hash: String,
    pub hash: String,
    pub sig: String,
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

/// LogBlock - ENC-notary aggregates StepReceipts into Merkle-rooted blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogBlock {
    /// Version (always 1 for v1.0)
    pub v: u8,
    /// Application identifier
    pub app: String,
    /// Block height (incremental)
    pub height: u64,
    /// Merkle root of all StepReceipts in this block
    pub merkle_root: String, // blake3:...
    /// Number of StepReceipts in this block
    pub count: u32,
    /// Notary signature (BLS or Ed25519)
    pub sig_notary: String, // ed25519:...
    /// Time range of receipts in this block
    pub range: TimeRange,
}

/// Time range for LogBlock
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub from_ts: String,
    pub to_ts: String,
}

/// ENC Notary configuration
#[derive(Debug, Clone)]
pub struct NotaryConfig {
    /// Maximum receipts per LogBlock
    pub max_receipts_per_block: u32,
    /// Maximum time window for LogBlock (seconds)
    pub max_block_window_s: u64,
    /// Application ID this notary serves
    pub app_id: String,
    /// Notary signing key
    pub signing_key: SigningKey,
}

/// ENC Notary - Aggregates StepReceipts into LogBlocks
pub struct EncNotary {
    /// Configuration
    config: NotaryConfig,
    /// Pending StepReceipts waiting for aggregation
    pending_receipts: Mutex<VecDeque<StepReceipt>>,
    /// Current LogBlock height
    current_height: Mutex<u64>,
    /// LogBlock sender channel (to BPI-comm and Blockbook)
    logblock_sender: mpsc::UnboundedSender<LogBlock>,
    /// Verifying key for signature verification
    verifying_key: VerifyingKey,
}

impl NotaryConfig {
    /// Create new notary configuration
    pub fn new(app_id: String, signing_key: SigningKey) -> Self {
        Self {
            max_receipts_per_block: 512, // Default: 512 receipts per LogBlock
            max_block_window_s: 60,      // Default: 60 second window
            app_id,
            signing_key,
        }
    }

    /// Set maximum receipts per LogBlock
    pub fn with_max_receipts(mut self, max_receipts: u32) -> Self {
        self.max_receipts_per_block = max_receipts;
        self
    }

    /// Set maximum block window in seconds
    pub fn with_max_window(mut self, window_s: u64) -> Self {
        self.max_block_window_s = window_s;
        self
    }
}

impl EncNotary {
    /// Create new ENC notary
    pub fn new(
        config: NotaryConfig,
        logblock_sender: mpsc::UnboundedSender<LogBlock>,
    ) -> Self {
        let verifying_key = VerifyingKey::from(&config.signing_key);
        
        Self {
            config,
            pending_receipts: Mutex::new(VecDeque::new()),
            current_height: Mutex::new(0),
            logblock_sender,
            verifying_key,
        }
    }

    /// Process incoming StepReceipt
    pub async fn process_receipt(&self, receipt: StepReceipt) -> Result<()> {
        // Verify receipt signature (optional but recommended)
        if let Err(e) = self.verify_receipt_signature(&receipt) {
            warn!("Invalid receipt signature from {}: {}", receipt.container, e);
            // Continue processing but log the issue
        }

        // Add to pending receipts
        {
            let mut pending = self.pending_receipts.lock().await;
            pending.push_back(receipt.clone());
            debug!("Added StepReceipt to pending queue: app={}, container={}, op={}", 
                   receipt.app, receipt.container, receipt.op);
        }

        // Check if we should create a LogBlock
        self.maybe_create_logblock().await?;

        Ok(())
    }

    /// Check if we should create a LogBlock and create it if needed
    async fn maybe_create_logblock(&self) -> Result<()> {
        let should_create = {
            let pending = self.pending_receipts.lock().await;
            pending.len() >= self.config.max_receipts_per_block as usize ||
            self.should_create_by_time(&pending).await
        };

        if should_create {
            self.create_logblock().await?;
        }

        Ok(())
    }

    /// Check if we should create LogBlock based on time window
    async fn should_create_by_time(&self, pending: &VecDeque<StepReceipt>) -> bool {
        if pending.is_empty() {
            return false;
        }

        // Get oldest receipt timestamp
        if let Some(oldest) = pending.front() {
            if let Ok(oldest_time) = chrono::DateTime::parse_from_rfc3339(&oldest.ts) {
                let now = chrono::Utc::now();
                let age = now.signed_duration_since(oldest_time);
                return age.num_seconds() >= self.config.max_block_window_s as i64;
            }
        }

        false
    }

    /// Create LogBlock from pending receipts
    async fn create_logblock(&self) -> Result<()> {
        let receipts = {
            let mut pending = self.pending_receipts.lock().await;
            if pending.is_empty() {
                return Ok(());
            }

            // Take up to max_receipts_per_block receipts
            let count = std::cmp::min(pending.len(), self.config.max_receipts_per_block as usize);
            let mut receipts = Vec::with_capacity(count);
            for _ in 0..count {
                if let Some(receipt) = pending.pop_front() {
                    receipts.push(receipt);
                }
            }
            receipts
        };

        if receipts.is_empty() {
            return Ok(());
        }

        // Get current height and increment
        let height = {
            let mut height = self.current_height.lock().await;
            *height += 1;
            *height
        };

        // Compute Merkle root
        let merkle_root = self.compute_merkle_root(&receipts)?;

        // Determine time range
        let range = self.compute_time_range(&receipts)?;

        // Create LogBlock
        let mut logblock = LogBlock {
            v: 1,
            app: self.config.app_id.clone(),
            height,
            merkle_root: format!("blake3:{}", hex::encode(merkle_root)),
            count: receipts.len() as u32,
            sig_notary: String::new(), // Will be computed
            range,
        };

        // Sign LogBlock
        let signature = self.sign_logblock(&logblock)?;
        logblock.sig_notary = format!("ed25519:{}", hex::encode(signature.to_bytes()));

        // Send to pipeline
        if let Err(e) = self.logblock_sender.send(logblock.clone()) {
            error!("Failed to send LogBlock to pipeline: {}", e);
            return Err(anyhow::anyhow!("LogBlock send failed: {}", e));
        }

        info!("Created LogBlock: app={}, height={}, count={}, merkle_root={}", 
              logblock.app, logblock.height, logblock.count, logblock.merkle_root);

        Ok(())
    }

    /// Compute Merkle root of StepReceipts
    fn compute_merkle_root(&self, receipts: &[StepReceipt]) -> Result<[u8; 32]> {
        if receipts.is_empty() {
            return Ok([0u8; 32]);
        }

        // Create leaf hashes
        let mut leaf_hashes = Vec::new();
        for receipt in receipts {
            let receipt_bytes = serde_json::to_vec(receipt)
                .context("Failed to serialize receipt")?;
            let leaf_hash = blake3::hash(&receipt_bytes);
            leaf_hashes.push(*leaf_hash.as_bytes());
        }

        // Build Merkle tree
        self.build_merkle_tree(leaf_hashes)
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
                    // Hash pair
                    let mut hasher = Hasher::new();
                    hasher.update(&chunk[0]);
                    hasher.update(&chunk[1]);
                    *hasher.finalize().as_bytes()
                } else {
                    // Odd number, hash with itself
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

    /// Compute time range from receipts
    fn compute_time_range(&self, receipts: &[StepReceipt]) -> Result<TimeRange> {
        if receipts.is_empty() {
            let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
            return Ok(TimeRange {
                from_ts: now.clone(),
                to_ts: now,
            });
        }

        let mut timestamps = Vec::new();
        for receipt in receipts {
            timestamps.push(receipt.ts.clone());
        }

        timestamps.sort();

        Ok(TimeRange {
            from_ts: timestamps.first().unwrap().clone(),
            to_ts: timestamps.last().unwrap().clone(),
        })
    }

    /// Sign LogBlock
    fn sign_logblock(&self, logblock: &LogBlock) -> Result<Signature> {
        let mut hasher = Hasher::new();
        
        // Hash all fields except signature
        hasher.update(&logblock.v.to_le_bytes());
        hasher.update(logblock.app.as_bytes());
        hasher.update(&logblock.height.to_le_bytes());
        hasher.update(logblock.merkle_root.as_bytes());
        hasher.update(&logblock.count.to_le_bytes());
        hasher.update(logblock.range.from_ts.as_bytes());
        hasher.update(logblock.range.to_ts.as_bytes());
        
        let hash = hasher.finalize();
        let signature = self.config.signing_key.sign(hash.as_bytes());
        
        Ok(signature)
    }

    /// Verify StepReceipt signature (basic validation)
    fn verify_receipt_signature(&self, receipt: &StepReceipt) -> Result<()> {
        // Basic format validation
        if !receipt.hash.starts_with("blake3:") {
            return Err(anyhow::anyhow!("Invalid hash format"));
        }
        
        if !receipt.sig.starts_with("ed25519:") {
            return Err(anyhow::anyhow!("Invalid signature format"));
        }

        // Could add full signature verification here if we had the public key
        // For now, just validate format
        Ok(())
    }

    /// Force create LogBlock from current pending receipts
    pub async fn force_create_logblock(&self) -> Result<()> {
        self.create_logblock().await
    }

    /// Get current LogBlock height
    pub async fn get_current_height(&self) -> u64 {
        *self.current_height.lock().await
    }

    /// Get pending receipt count
    pub async fn get_pending_count(&self) -> usize {
        self.pending_receipts.lock().await.len()
    }

    /// Get notary statistics
    pub async fn get_stats(&self) -> NotaryStats {
        NotaryStats {
            current_height: self.get_current_height().await,
            pending_receipts: self.get_pending_count().await,
            app_id: self.config.app_id.clone(),
            max_receipts_per_block: self.config.max_receipts_per_block,
            max_block_window_s: self.config.max_block_window_s,
        }
    }
}

/// Notary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotaryStats {
    pub current_height: u64,
    pub pending_receipts: usize,
    pub app_id: String,
    pub max_receipts_per_block: u32,
    pub max_block_window_s: u64,
}

/// Start ENC notary service
pub async fn start_notary_service(
    config: NotaryConfig,
    receipt_receiver: mpsc::UnboundedReceiver<StepReceipt>,
    logblock_sender: mpsc::UnboundedSender<LogBlock>,
) -> Result<()> {
    let notary = EncNotary::new(config, logblock_sender);
    let mut receipt_receiver = receipt_receiver;

    info!("Starting ENC notary service for app: {}", notary.config.app_id);

    // Process incoming receipts
    while let Some(receipt) = receipt_receiver.recv().await {
        if let Err(e) = notary.process_receipt(receipt).await {
            error!("Failed to process receipt: {}", e);
        }
    }

    info!("ENC notary service stopped");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;
    use std::collections::HashMap;
    use tokio::sync::mpsc;

    fn create_test_receipt(app: &str, container: &str, op: &str) -> StepReceipt {
        StepReceipt {
            v: 1,
            app: app.to_string(),
            container: container.to_string(),
            op: op.to_string(),
            ts: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            usage: ResourceUsage {
                cpu_ms: 100,
                memory_mb_s: 50,
                storage_gb_day: 0.1,
                egress_mb: 1.0,
                receipts_count: 1,
            },
            labels: HashMap::new(),
            prev_hash: "blake3:genesis".to_string(),
            hash: "blake3:test".to_string(),
            sig: "ed25519:test".to_string(),
        }
    }

    #[tokio::test]
    async fn test_notary_logblock_creation() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let config = NotaryConfig::new("TEST_APP".to_string(), signing_key)
            .with_max_receipts(2); // Small batch for testing

        let (tx, mut rx) = mpsc::unbounded_channel();
        let notary = EncNotary::new(config, tx);

        // Add receipts
        let receipt1 = create_test_receipt("TEST_APP", "container1", "exec.start");
        let receipt2 = create_test_receipt("TEST_APP", "container2", "exec.stop");

        notary.process_receipt(receipt1).await.unwrap();
        notary.process_receipt(receipt2).await.unwrap();

        // Should receive a LogBlock
        let logblock = rx.recv().await.unwrap();
        assert_eq!(logblock.app, "TEST_APP");
        assert_eq!(logblock.height, 1);
        assert_eq!(logblock.count, 2);
        assert!(logblock.merkle_root.starts_with("blake3:"));
        assert!(logblock.sig_notary.starts_with("ed25519:"));
    }

    #[tokio::test]
    async fn test_merkle_root_computation() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let config = NotaryConfig::new("TEST_APP".to_string(), signing_key);
        let (tx, _rx) = mpsc::unbounded_channel();
        let notary = EncNotary::new(config, tx);

        let receipts = vec![
            create_test_receipt("TEST_APP", "container1", "op1"),
            create_test_receipt("TEST_APP", "container2", "op2"),
            create_test_receipt("TEST_APP", "container3", "op3"),
        ];

        let merkle_root = notary.compute_merkle_root(&receipts).unwrap();
        assert_ne!(merkle_root, [0u8; 32]);

        // Same receipts should produce same root
        let merkle_root2 = notary.compute_merkle_root(&receipts).unwrap();
        assert_eq!(merkle_root, merkle_root2);
    }

    #[test]
    fn test_notary_config() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        
        let config = NotaryConfig::new("TEST_APP".to_string(), signing_key)
            .with_max_receipts(1000)
            .with_max_window(120);

        assert_eq!(config.app_id, "TEST_APP");
        assert_eq!(config.max_receipts_per_block, 1000);
        assert_eq!(config.max_block_window_s, 120);
    }
}
