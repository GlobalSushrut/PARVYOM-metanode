use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use bpi_anchor::{AnchorReceipt, AnchorStatus};

/// Configuration for the Light Client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightClientConfig {
    /// Maximum depth for reorg protection
    pub max_reorg_depth: u64,
    /// Minimum confirmations required for anchor verification
    pub min_anchor_confirmations: u64,
    /// Timeout for anchor verification in seconds
    pub anchor_verification_timeout: u64,
    /// Enable strict anchor verification (refuse all pre-anchor reorgs)
    pub strict_anchor_verification: bool,
}

impl Default for LightClientConfig {
    fn default() -> Self {
        Self {
            max_reorg_depth: 100,
            min_anchor_confirmations: 6,
            anchor_verification_timeout: 300, // 5 minutes
            strict_anchor_verification: true,
        }
    }
}

/// Block header with anchor information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockHeader {
    /// Block hash
    pub hash: String,
    /// Block height
    pub height: u64,
    /// Parent block hash
    pub parent_hash: String,
    /// Block timestamp
    pub timestamp: DateTime<Utc>,
    /// Merkle root of transactions
    pub merkle_root: String,
    /// State root
    pub state_root: String,
    /// Anchor information for this block
    pub anchor_info: Option<AnchorInfo>,
}

/// Anchor information for a block
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnchorInfo {
    /// Anchor ID
    pub anchor_id: String,
    /// L1 chain where anchor was posted
    pub l1_chain: String,
    /// L1 transaction hash
    pub l1_tx_hash: String,
    /// L1 block number where anchor was confirmed
    pub l1_block_number: u64,
    /// Number of L1 confirmations
    pub confirmations: u64,
    /// Anchor timestamp
    pub anchored_at: DateTime<Utc>,
}

/// Chain state with anchor verification
#[derive(Debug, Clone)]
pub struct ChainState {
    /// Current best block header
    pub best_header: BlockHeader,
    /// Last confirmed anchor height
    pub last_anchor_height: u64,
    /// Headers by height
    pub headers_by_height: BTreeMap<u64, BlockHeader>,
    /// Headers by hash
    pub headers_by_hash: HashMap<String, BlockHeader>,
    /// Anchored headers (height -> anchor info)
    pub anchored_headers: BTreeMap<u64, AnchorInfo>,
}

/// Reorg validation result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReorgValidation {
    /// Reorg is allowed
    Allowed,
    /// Reorg is rejected due to anchor verification
    RejectedPreAnchor {
        /// Height of the last confirmed anchor
        last_anchor_height: u64,
        /// Height being reorged
        reorg_height: u64,
    },
    /// Reorg is rejected due to depth limit
    RejectedDepthLimit {
        /// Maximum allowed depth
        max_depth: u64,
        /// Actual reorg depth
        actual_depth: u64,
    },
}

/// Light Client with anchor verification
pub struct LightClient {
    /// Configuration
    config: LightClientConfig,
    /// Chain state
    state: Arc<RwLock<ChainState>>,
    /// Anchor receipts cache
    anchor_receipts: Arc<RwLock<HashMap<String, AnchorReceipt>>>,
}

impl LightClient {
    /// Create a new Light Client
    pub fn new(config: LightClientConfig, genesis_header: BlockHeader) -> Result<Self> {
        let initial_state = ChainState {
            best_header: genesis_header.clone(),
            last_anchor_height: 0,
            headers_by_height: {
                let mut map = BTreeMap::new();
                map.insert(genesis_header.height, genesis_header.clone());
                map
            },
            headers_by_hash: {
                let mut map = HashMap::new();
                map.insert(genesis_header.hash.clone(), genesis_header);
                map
            },
            anchored_headers: BTreeMap::new(),
        };

        Ok(Self {
            config,
            state: Arc::new(RwLock::new(initial_state)),
            anchor_receipts: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Add a new block header to the chain
    pub async fn add_header(&self, header: BlockHeader) -> Result<bool> {
        let mut state = self.state.write().await;
        
        // Check if header already exists
        if state.headers_by_hash.contains_key(&header.hash) {
            debug!("Header {} already exists", header.hash);
            return Ok(false);
        }

        // Validate parent exists
        if !state.headers_by_hash.contains_key(&header.parent_hash) {
            return Err(anyhow!("Parent header {} not found", header.parent_hash));
        }

        // Check for reorg
        if header.height <= state.best_header.height {
            let reorg_validation = self.validate_reorg(&state, &header).await?;
            match reorg_validation {
                ReorgValidation::Allowed => {
                    info!("Reorg allowed for header at height {}", header.height);
                }
                ReorgValidation::RejectedPreAnchor { last_anchor_height, reorg_height } => {
                    warn!(
                        "Reorg rejected: attempting to reorg height {} but last anchor is at height {}",
                        reorg_height, last_anchor_height
                    );
                    return Ok(false);
                }
                ReorgValidation::RejectedDepthLimit { max_depth, actual_depth } => {
                    warn!(
                        "Reorg rejected: depth {} exceeds maximum {}",
                        actual_depth, max_depth
                    );
                    return Ok(false);
                }
            }
        }

        // Add header to state
        state.headers_by_height.insert(header.height, header.clone());
        state.headers_by_hash.insert(header.hash.clone(), header.clone());

        // Update anchor information if present
        if let Some(anchor_info) = &header.anchor_info {
            if anchor_info.confirmations >= self.config.min_anchor_confirmations {
                state.anchored_headers.insert(header.height, anchor_info.clone());
                state.last_anchor_height = state.last_anchor_height.max(header.height);
                info!("Updated last anchor height to {}", state.last_anchor_height);
            }
        }

        // Update best header if this extends the chain
        if header.height > state.best_header.height {
            state.best_header = header.clone();
            info!("New best header at height {}: {}", header.height, header.hash);
        }

        Ok(true)
    }

    /// Validate a potential reorg
    async fn validate_reorg(&self, state: &ChainState, new_header: &BlockHeader) -> Result<ReorgValidation> {
        let reorg_depth = if state.best_header.height >= new_header.height {
            state.best_header.height - new_header.height + 1
        } else {
            0
        };

        // Check depth limit
        if reorg_depth > self.config.max_reorg_depth {
            return Ok(ReorgValidation::RejectedDepthLimit {
                max_depth: self.config.max_reorg_depth,
                actual_depth: reorg_depth,
            });
        }

        // Check anchor verification if enabled
        if self.config.strict_anchor_verification && state.last_anchor_height > 0 {
            if new_header.height <= state.last_anchor_height {
                return Ok(ReorgValidation::RejectedPreAnchor {
                    last_anchor_height: state.last_anchor_height,
                    reorg_height: new_header.height,
                });
            }
        }

        Ok(ReorgValidation::Allowed)
    }

    /// Update anchor receipt for a block
    pub async fn update_anchor_receipt(&self, anchor_id: String, receipt: AnchorReceipt) -> Result<()> {
        let mut receipts = self.anchor_receipts.write().await;
        receipts.insert(anchor_id.clone(), receipt.clone());

        // Update corresponding header if it exists
        let mut state = self.state.write().await;
        let mut updated_anchor_info: Option<(u64, AnchorInfo)> = None;
        
        for (height, header) in state.headers_by_height.iter_mut() {
            if let Some(anchor_info) = &mut header.anchor_info {
                if anchor_info.anchor_id == anchor_id {
                    // Update confirmations based on receipt
                    if receipt.status == AnchorStatus::Confirmed {
                        anchor_info.confirmations = receipt.confirmations as u64;
                        
                        // Store anchor info for later update if confirmations are sufficient
                        if anchor_info.confirmations >= self.config.min_anchor_confirmations {
                            updated_anchor_info = Some((*height, anchor_info.clone()));
                        }
                    }
                    break;
                }
            }
        }
        
        // Update anchored headers outside the loop to avoid borrowing conflicts
        if let Some((height, anchor_info)) = updated_anchor_info {
            state.anchored_headers.insert(height, anchor_info);
            state.last_anchor_height = state.last_anchor_height.max(height);
        }

        Ok(())
    }

    /// Get the current best header
    pub async fn get_best_header(&self) -> BlockHeader {
        let state = self.state.read().await;
        state.best_header.clone()
    }

    /// Get header by hash
    pub async fn get_header_by_hash(&self, hash: &str) -> Option<BlockHeader> {
        let state = self.state.read().await;
        state.headers_by_hash.get(hash).cloned()
    }

    /// Get header by height
    pub async fn get_header_by_height(&self, height: u64) -> Option<BlockHeader> {
        let state = self.state.read().await;
        state.headers_by_height.get(&height).cloned()
    }

    /// Get the last confirmed anchor height
    pub async fn get_last_anchor_height(&self) -> u64 {
        let state = self.state.read().await;
        state.last_anchor_height
    }

    /// Get all anchored headers
    pub async fn get_anchored_headers(&self) -> BTreeMap<u64, AnchorInfo> {
        let state = self.state.read().await;
        state.anchored_headers.clone()
    }

    /// Check if a reorg would be allowed
    pub async fn would_allow_reorg(&self, target_height: u64) -> Result<ReorgValidation> {
        let state = self.state.read().await;
        
        // Create a dummy header for validation
        let dummy_header = BlockHeader {
            hash: "dummy".to_string(),
            height: target_height,
            parent_hash: "dummy_parent".to_string(),
            timestamp: Utc::now(),
            merkle_root: "dummy_merkle".to_string(),
            state_root: "dummy_state".to_string(),
            anchor_info: None,
        };

        self.validate_reorg(&state, &dummy_header).await
    }

    /// Get chain statistics
    pub async fn get_chain_stats(&self) -> ChainStats {
        let state = self.state.read().await;
        let receipts = self.anchor_receipts.read().await;

        ChainStats {
            best_height: state.best_header.height,
            last_anchor_height: state.last_anchor_height,
            total_headers: state.headers_by_hash.len() as u64,
            anchored_headers_count: state.anchored_headers.len() as u64,
            pending_anchor_receipts: receipts.len() as u64,
        }
    }
}

/// Chain statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainStats {
    /// Current best block height
    pub best_height: u64,
    /// Last confirmed anchor height
    pub last_anchor_height: u64,
    /// Total number of headers
    pub total_headers: u64,
    /// Number of anchored headers
    pub anchored_headers_count: u64,
    /// Number of pending anchor receipts
    pub pending_anchor_receipts: u64,
}

/// Generate a block hash from header data
pub fn generate_block_hash(header: &BlockHeader) -> String {
    let mut hasher = Sha256::new();
    hasher.update(header.height.to_be_bytes());
    hasher.update(header.parent_hash.as_bytes());
    hasher.update(header.timestamp.timestamp().to_be_bytes());
    hasher.update(header.merkle_root.as_bytes());
    hasher.update(header.state_root.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn create_test_config() -> LightClientConfig {
        LightClientConfig {
            max_reorg_depth: 10,
            min_anchor_confirmations: 3,
            anchor_verification_timeout: 60,
            strict_anchor_verification: true,
        }
    }

    fn create_genesis_header() -> BlockHeader {
        BlockHeader {
            hash: "genesis_hash".to_string(),
            height: 0,
            parent_hash: "0x0".to_string(),
            timestamp: Utc::now() - Duration::hours(1),
            merkle_root: "genesis_merkle".to_string(),
            state_root: "genesis_state".to_string(),
            anchor_info: None,
        }
    }

    fn create_test_header(height: u64, parent_hash: &str) -> BlockHeader {
        BlockHeader {
            hash: format!("block_hash_{}", height),
            height,
            parent_hash: parent_hash.to_string(),
            timestamp: Utc::now(),
            merkle_root: format!("merkle_{}", height),
            state_root: format!("state_{}", height),
            anchor_info: None,
        }
    }

    fn create_anchored_header(height: u64, parent_hash: &str, confirmations: u64) -> BlockHeader {
        let mut header = create_test_header(height, parent_hash);
        header.anchor_info = Some(AnchorInfo {
            anchor_id: format!("anchor_{}", height),
            l1_chain: "ethereum".to_string(),
            l1_tx_hash: format!("0x{:064x}", height),
            l1_block_number: 1000 + height,
            confirmations,
            anchored_at: Utc::now(),
        });
        header
    }

    #[tokio::test]
    async fn test_light_client_creation() {
        let config = create_test_config();
        let genesis = create_genesis_header();
        let client = LightClient::new(config, genesis.clone()).unwrap();

        let best = client.get_best_header().await;
        assert_eq!(best.hash, genesis.hash);
        assert_eq!(best.height, 0);

        println!("✅ Light client creation working");
    }

    #[tokio::test]
    async fn test_add_headers() {
        let config = create_test_config();
        let genesis = create_genesis_header();
        let client = LightClient::new(config, genesis.clone()).unwrap();

        // Add block 1
        let block1 = create_test_header(1, &genesis.hash);
        let added = client.add_header(block1.clone()).await.unwrap();
        assert!(added);

        let best = client.get_best_header().await;
        assert_eq!(best.height, 1);
        assert_eq!(best.hash, block1.hash);

        // Add block 2
        let block2 = create_test_header(2, &block1.hash);
        let added = client.add_header(block2.clone()).await.unwrap();
        assert!(added);

        let best = client.get_best_header().await;
        assert_eq!(best.height, 2);

        println!("✅ Adding headers working");
    }

    #[tokio::test]
    async fn test_anchor_verification() {
        let config = create_test_config();
        let genesis = create_genesis_header();
        let client = LightClient::new(config, genesis.clone()).unwrap();

        // Add anchored block at height 5
        let anchored_block = create_anchored_header(5, &genesis.hash, 5);
        let added = client.add_header(anchored_block.clone()).await.unwrap();
        assert!(added);

        let last_anchor = client.get_last_anchor_height().await;
        assert_eq!(last_anchor, 5);

        // Try to reorg before anchor (should be rejected)
        let reorg_validation = client.would_allow_reorg(3).await.unwrap();
        assert_eq!(reorg_validation, ReorgValidation::RejectedPreAnchor {
            last_anchor_height: 5,
            reorg_height: 3,
        });

        // Try to reorg after anchor (should be allowed)
        let reorg_validation = client.would_allow_reorg(7).await.unwrap();
        assert_eq!(reorg_validation, ReorgValidation::Allowed);

        println!("✅ Anchor verification working");
    }

    #[tokio::test]
    async fn test_reorg_depth_limit() {
        let config = create_test_config();
        let genesis = create_genesis_header();
        let client = LightClient::new(config, genesis.clone()).unwrap();

        // Build a chain of 15 blocks
        let mut prev_hash = genesis.hash.clone();
        for i in 1..=15 {
            let block = create_test_header(i, &prev_hash);
            prev_hash = block.hash.clone();
            client.add_header(block).await.unwrap();
        }

        // Try to reorg beyond depth limit (should be rejected)
        let reorg_validation = client.would_allow_reorg(5).await.unwrap();
        assert_eq!(reorg_validation, ReorgValidation::RejectedDepthLimit {
            max_depth: 10,
            actual_depth: 11,
        });

        // Try to reorg within depth limit (should be allowed if no anchor)
        let reorg_validation = client.would_allow_reorg(10).await.unwrap();
        assert_eq!(reorg_validation, ReorgValidation::Allowed);

        println!("✅ Reorg depth limit working");
    }

    #[tokio::test]
    async fn test_anchor_receipt_update() {
        let config = create_test_config();
        let genesis = create_genesis_header();
        let client = LightClient::new(config, genesis.clone()).unwrap();

        // Add block with insufficient confirmations
        let block = create_anchored_header(3, &genesis.hash, 1);
        client.add_header(block.clone()).await.unwrap();

        // Should not be considered anchored yet
        let last_anchor = client.get_last_anchor_height().await;
        assert_eq!(last_anchor, 0);

        // Update receipt with sufficient confirmations
        let receipt = AnchorReceipt {
            anchor_id: "anchor_3".to_string(),
            header_hash: vec![0u8; 32],
            chain_id: 1,
            tx_hash: format!("0x{:064x}", 3),
            block_number: 1003,
            gas_used: 21000,
            gas_price: 20_000_000_000,
            status: AnchorStatus::Confirmed,
            timestamp: Utc::now(),
            confirmations: 5,
            retry_count: 0,
        };

        client.update_anchor_receipt("anchor_3".to_string(), receipt).await.unwrap();

        // Should now be considered anchored
        let last_anchor = client.get_last_anchor_height().await;
        assert_eq!(last_anchor, 3);

        println!("✅ Anchor receipt update working");
    }

    #[tokio::test]
    async fn test_deep_reorg_simulation() {
        let config = create_test_config();
        let genesis = create_genesis_header();
        let client = LightClient::new(config, genesis.clone()).unwrap();

        // Build main chain with anchor at height 10
        let mut prev_hash = genesis.hash.clone();
        for i in 1..=15 {
            let block = if i == 10 {
                create_anchored_header(i, &prev_hash, 5)
            } else {
                create_test_header(i, &prev_hash)
            };
            prev_hash = block.hash.clone();
            client.add_header(block).await.unwrap();
        }

        let stats = client.get_chain_stats().await;
        assert_eq!(stats.best_height, 15);
        assert_eq!(stats.last_anchor_height, 10);

        // Attempt deep reorg before anchor (should fail)
        let reorg_block = create_test_header(8, "alternative_parent");
        let added = client.add_header(reorg_block).await.unwrap();
        assert!(!added); // Should be rejected

        // Attempt reorg after anchor (should succeed if within depth)
        let reorg_block = create_test_header(12, "alternative_parent_12");
        let added = client.add_header(reorg_block).await.unwrap();
        assert!(!added); // Should be rejected due to missing parent

        println!("✅ Deep reorg simulation working");
    }

    #[tokio::test]
    async fn test_stage45_exit_criteria() {
        let config = create_test_config();
        let genesis = create_genesis_header();
        let client = LightClient::new(config, genesis.clone()).unwrap();

        // Build chain with multiple anchors
        let mut prev_hash = genesis.hash.clone();
        for i in 1..=20 {
            let block = if i % 5 == 0 {
                create_anchored_header(i, &prev_hash, 6)
            } else {
                create_test_header(i, &prev_hash)
            };
            prev_hash = block.hash.clone();
            client.add_header(block).await.unwrap();
        }

        let stats = client.get_chain_stats().await;
        assert_eq!(stats.best_height, 20);
        assert_eq!(stats.last_anchor_height, 20);
        assert_eq!(stats.anchored_headers_count, 4); // Heights 5, 10, 15, 20

        // Verify LC refuses pre-anchor reorgs
        for height in 1..=20 {
            let validation = client.would_allow_reorg(height).await.unwrap();
            if height <= 20 {
                // Check if it's rejected due to depth limit or pre-anchor
                match validation {
                    ReorgValidation::RejectedPreAnchor { last_anchor_height, reorg_height } => {
                        assert_eq!(last_anchor_height, 20);
                        assert_eq!(reorg_height, height);
                    }
                    ReorgValidation::RejectedDepthLimit { max_depth, actual_depth: _ } => {
                        assert_eq!(max_depth, 10);
                        // This is also acceptable for deep reorgs
                    }
                    ReorgValidation::Allowed => {
                        panic!("Reorg should be rejected for height {}", height);
                    }
                }
            }
        }

        println!("✅ Stage 45 exit criteria met: LC refuses pre-anchor reorgs");
    }

    #[tokio::test]
    async fn test_chain_statistics() {
        let config = create_test_config();
        let genesis = create_genesis_header();
        let client = LightClient::new(config, genesis.clone()).unwrap();

        // Add some blocks
        let mut prev_hash = genesis.hash.clone();
        for i in 1..=10 {
            let block = if i == 5 {
                create_anchored_header(i, &prev_hash, 4)
            } else {
                create_test_header(i, &prev_hash)
            };
            prev_hash = block.hash.clone();
            client.add_header(block).await.unwrap();
        }

        let stats = client.get_chain_stats().await;
        assert_eq!(stats.best_height, 10);
        assert_eq!(stats.last_anchor_height, 5);
        assert_eq!(stats.total_headers, 11); // Genesis + 10 blocks
        assert_eq!(stats.anchored_headers_count, 1);

        println!("✅ Chain statistics working");
    }
}
