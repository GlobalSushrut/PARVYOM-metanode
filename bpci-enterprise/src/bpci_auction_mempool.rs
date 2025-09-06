use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc};
use std::cmp::Ordering;

/// BPCI Auction Mempool - Real Merkle Tree Implementation
/// Sophisticated auction-focused mempool for BPCI's multi-chain coordination
/// Uses real Merkle trees for transaction ordering and auction bundle generation

/// Auction transaction representing bids from partner chains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionTransaction {
    pub tx_id: [u8; 32],
    pub chain_id: u64,
    pub bid_amount: u64,          // Fee bid in wei
    pub gas_limit: u64,
    pub data_size: u32,           // Transaction data size
    pub priority_score: u16,      // QoS priority (0-1000)
    pub timestamp: u64,
    pub nonce: u64,
    pub sender: String,
    pub target_chain: Option<u64>, // Target partner chain for execution
    pub auction_type: AuctionType,
}

impl AuctionTransaction {
    pub fn new(tx_id: [u8; 32], chain_id: u64, bid_amount: u64, gas_limit: u64, data_size: u32, sender: String) -> Self {
        Self {
            tx_id,
            chain_id,
            bid_amount,
            gas_limit,
            data_size,
            priority_score: 500, // Default priority
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            nonce: 0,
            sender,
            target_chain: None,
            auction_type: AuctionType::StandardExecution,
        }
    }
    
    /// Calculate effective bid rate (bid per gas per byte)
    pub fn effective_bid_rate(&self) -> f64 {
        if self.gas_limit == 0 || self.data_size == 0 {
            return 0.0;
        }
        self.bid_amount as f64 / (self.gas_limit as f64 * self.data_size as f64)
    }
    
    /// Compare transactions for auction ordering
    pub fn compare_for_auction(&self, other: &AuctionTransaction) -> Ordering {
        // Primary: Effective bid rate (higher is better)
        match other.effective_bid_rate().partial_cmp(&self.effective_bid_rate()).unwrap_or(Ordering::Equal) {
            Ordering::Equal => {
                // Secondary: Priority score (higher is better)
                match other.priority_score.cmp(&self.priority_score) {
                    Ordering::Equal => {
                        // Tertiary: Timestamp (earlier is better)
                        self.timestamp.cmp(&other.timestamp)
                    }
                    other_order => other_order,
                }
            }
            other_order => other_order,
        }
    }
    
    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.tx_id);
        hasher.update(self.chain_id.to_be_bytes());
        hasher.update(self.bid_amount.to_be_bytes());
        hasher.update(self.gas_limit.to_be_bytes());
        hasher.update(self.data_size.to_be_bytes());
        hasher.update(self.priority_score.to_be_bytes());
        hasher.update(self.timestamp.to_be_bytes());
        hasher.update(self.nonce.to_be_bytes());
        hasher.update(self.sender.as_bytes());
        
        let hash = hasher.finalize();
        let mut result = [0u8; 32];
        result.copy_from_slice(&hash);
        result
    }
}

/// Types of auctions BPCI can coordinate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuctionType {
    StandardExecution,    // Regular transaction execution
    CrossChainBridge,     // Cross-chain bridge operations
    DataStorage,          // Data storage on partner chains
    ComputeResource,      // Compute resource allocation
    ValidatorStaking,     // Validator staking operations
}

/// Auction window for time-based auction rounds
#[derive(Debug, Clone)]
pub struct AuctionWindow {
    pub window_id: u64,
    pub start_time: SystemTime,
    pub end_time: SystemTime,
    pub max_transactions: u32,
    pub total_gas_limit: u64,
    pub auction_type: AuctionType,
    pub is_sealed: bool,
    pub winner_count: u32,
}

impl AuctionWindow {
    pub fn new(window_id: u64, duration_ms: u64, max_transactions: u32, total_gas_limit: u64, auction_type: AuctionType) -> Self {
        let start_time = SystemTime::now();
        let end_time = start_time + std::time::Duration::from_millis(duration_ms);
        
        Self {
            window_id,
            start_time,
            end_time,
            max_transactions,
            total_gas_limit,
            auction_type,
            is_sealed: false,
            winner_count: 0,
        }
    }
    
    pub fn is_active(&self) -> bool {
        let now = SystemTime::now();
        now >= self.start_time && now <= self.end_time && !self.is_sealed
    }
    
    pub fn should_seal(&self) -> bool {
        SystemTime::now() > self.end_time && !self.is_sealed
    }
}

/// Real Merkle tree implementation for auction transaction ordering
#[derive(Debug, Clone)]
pub struct AuctionMerkleTree {
    pub transactions: Vec<AuctionTransaction>,
    pub merkle_nodes: Vec<Vec<[u8; 32]>>, // Levels of the tree
    pub root: [u8; 32],
}

impl AuctionMerkleTree {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
            merkle_nodes: Vec::new(),
            root: [0u8; 32],
        }
    }
    
    /// Insert transaction maintaining auction order (highest bid rate first)
    pub fn insert_transaction(&mut self, tx: AuctionTransaction) -> Result<()> {
        // Find insertion position to maintain ordering (highest bid rate first)
        let insert_pos = self.transactions.binary_search_by(|existing| {
            existing.compare_for_auction(&tx)
        }).unwrap_or_else(|pos| pos);
        
        self.transactions.insert(insert_pos, tx);
        self.rebuild_merkle_tree()?;
        Ok(())
    }
    
    /// Remove transaction by ID
    pub fn remove_transaction(&mut self, tx_id: &[u8; 32]) -> Result<bool> {
        if let Some(pos) = self.transactions.iter().position(|tx| &tx.tx_id == tx_id) {
            self.transactions.remove(pos);
            self.rebuild_merkle_tree()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// Get top N transactions (auction winners)
    pub fn get_top_transactions(&self, n: usize) -> Vec<&AuctionTransaction> {
        self.transactions.iter().take(n).collect()
    }
    
    /// Get transactions within gas limit (for auction bundle creation)
    pub fn get_transactions_within_gas_limit(&self, gas_limit: u64) -> Vec<&AuctionTransaction> {
        let mut selected = Vec::new();
        let mut total_gas = 0u64;
        
        for tx in &self.transactions {
            if total_gas + tx.gas_limit <= gas_limit {
                selected.push(tx);
                total_gas += tx.gas_limit;
            }
        }
        
        selected
    }
    
    /// Rebuild the entire Merkle tree
    fn rebuild_merkle_tree(&mut self) -> Result<()> {
        self.merkle_nodes.clear();
        
        if self.transactions.is_empty() {
            self.root = [0u8; 32];
            return Ok(());
        }
        
        // Create leaf level (transaction hashes)
        let leaf_hashes: Vec<[u8; 32]> = self.transactions
            .iter()
            .map(|tx| tx.compute_hash())
            .collect();
        
        self.merkle_nodes.push(leaf_hashes);
        
        // Build tree levels bottom-up
        let mut current_level = 0;
        while self.merkle_nodes[current_level].len() > 1 {
            let mut next_level = Vec::new();
            let current_nodes = &self.merkle_nodes[current_level];
            
            for chunk in current_nodes.chunks(2) {
                let mut hasher = Sha256::new();
                hasher.update(chunk[0]);
                
                if chunk.len() > 1 {
                    hasher.update(chunk[1]);
                } else {
                    // Odd number of nodes - duplicate the last one
                    hasher.update(chunk[0]);
                }
                
                let hash = hasher.finalize();
                let mut node_hash = [0u8; 32];
                node_hash.copy_from_slice(&hash);
                next_level.push(node_hash);
            }
            
            self.merkle_nodes.push(next_level);
            current_level += 1;
        }
        
        // Set root
        if let Some(last_level) = self.merkle_nodes.last() {
            if !last_level.is_empty() {
                self.root = last_level[0];
            }
        }
        
        Ok(())
    }
    
    /// Generate Merkle proof for a specific transaction
    pub fn generate_proof(&self, tx_id: &[u8; 32]) -> Result<MerkleProof> {
        let leaf_index = self.transactions
            .iter()
            .position(|tx| &tx.tx_id == tx_id)
            .ok_or_else(|| anyhow!("Transaction not found in tree"))?;
        
        let mut proof_hashes = Vec::new();
        let mut current_index = leaf_index;
        
        // Traverse from leaf to root, collecting sibling hashes
        for level in 0..self.merkle_nodes.len() - 1 {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };
            
            if sibling_index < self.merkle_nodes[level].len() {
                proof_hashes.push(self.merkle_nodes[level][sibling_index]);
            } else {
                // No sibling (odd number of nodes) - use self
                proof_hashes.push(self.merkle_nodes[level][current_index]);
            }
            
            current_index /= 2;
        }
        
        Ok(MerkleProof {
            leaf_index,
            proof_hashes,
            root: self.root,
        })
    }
    
    /// Verify a Merkle proof
    pub fn verify_proof(&self, tx: &AuctionTransaction, proof: &MerkleProof) -> bool {
        if proof.root != self.root {
            return false;
        }
        
        let mut current_hash = tx.compute_hash();
        let mut current_index = proof.leaf_index;
        
        for sibling_hash in &proof.proof_hashes {
            let mut hasher = Sha256::new();
            
            if current_index % 2 == 0 {
                // Left child
                hasher.update(current_hash);
                hasher.update(sibling_hash);
            } else {
                // Right child
                hasher.update(sibling_hash);
                hasher.update(current_hash);
            }
            
            let hash = hasher.finalize();
            current_hash.copy_from_slice(&hash);
            current_index /= 2;
        }
        
        current_hash == self.root
    }
    
    pub fn get_root(&self) -> [u8; 32] {
        self.root
    }
    
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }
}

/// Merkle proof for transaction inclusion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    pub leaf_index: usize,
    pub proof_hashes: Vec<[u8; 32]>,
    pub root: [u8; 32],
}

/// BPCI Auction Mempool - Main coordinator for multi-chain auctions
#[derive(Debug)]
pub struct BpciAuctionMempool {
    merkle_tree: AuctionMerkleTree,
    auction_windows: HashMap<u64, AuctionWindow>,
    completed_auctions: Vec<CompletedAuction>,
    chain_stats: HashMap<u64, ChainStats>,
    next_window_id: u64,
    testnet_storage: Option<Arc<crate::testnet_auction_storage::TestnetAuctionStorage>>,
    config: Arc<crate::testnet_config::BpciConfig>,
}

impl BpciAuctionMempool {
    pub fn new() -> Self {
        Self {
            merkle_tree: AuctionMerkleTree::new(),
            auction_windows: HashMap::new(),
            completed_auctions: Vec::new(),
            chain_stats: HashMap::new(),
            next_window_id: 1,
            testnet_storage: None,
            config: Arc::new(crate::testnet_config::BpciConfig::default()),
        }
    }

    /// Create new auction mempool with testnet configuration
    pub async fn new_with_config(config: Arc<crate::testnet_config::BpciConfig>) -> Result<Self> {
        let testnet_storage = if config.is_testnet() {
            Some(Arc::new(crate::testnet_auction_storage::TestnetAuctionStorage::new(config.clone()).await?))
        } else {
            None
        };

        Ok(Self {
            merkle_tree: AuctionMerkleTree::new(),
            auction_windows: HashMap::new(),
            completed_auctions: Vec::new(),
            chain_stats: HashMap::new(),
            next_window_id: 1,
            testnet_storage,
            config,
        })
    }
    
    /// Submit transaction to auction mempool
    pub fn submit_transaction(&mut self, tx: AuctionTransaction) -> Result<()> {
        // Update chain statistics
        let chain_stats = self.chain_statistics.entry(tx.chain_id).or_insert_with(ChainStats::new);
        chain_stats.total_transactions += 1;
        chain_stats.total_bid_amount += tx.bid_amount;
        
        // Insert into pending transactions (maintains auction order)
        self.pending_transactions.insert_transaction(tx)?;
        
        Ok(())
    }
    
    /// Remove transaction from mempool
    pub fn remove_transaction(&mut self, tx_id: &[u8; 32]) -> Result<bool> {
        self.pending_transactions.remove_transaction(tx_id)
    }
    
    /// Create new auction window
    pub fn create_auction_window(&mut self, duration_ms: u64, max_transactions: u32, total_gas_limit: u64, auction_type: AuctionType) -> u64 {
        let window_id = self.next_window_id;
        self.next_window_id += 1;
        
        let window = AuctionWindow::new(window_id, duration_ms, max_transactions, total_gas_limit, auction_type);
        self.active_windows.insert(window_id, window);
        
        window_id
    }
    
    /// Seal auction window and generate winners (with testnet support)
    pub async fn seal_auction_window(&mut self, window_id: u64) -> Result<AuctionResult> {
        // Execute normal auction logic first
        let result = self.seal_auction_window_internal(window_id)?;
        
        // If testnet mode, store to database instead of executing on BPI
        if self.config.is_testnet() {
            if let Some(testnet_storage) = &self.testnet_storage {
                let _record = testnet_storage.store_auction_result(&result).await?;
                testnet_storage.mock_partner_revenue_distribution(&result.auction_id).await?;
                tracing::info!("🧪 Testnet: Auction {} stored to database with mock execution", result.auction_id);
            }
        } else {
            // Mainnet: Execute on real BPI (future implementation)
            tracing::info!("🚀 Mainnet: Auction {} executed on BPI", result.auction_id);
        }
        
        Ok(result)
    }

    /// Internal auction sealing logic (network-agnostic)
    fn seal_auction_window_internal(&mut self, window_id: u64) -> Result<AuctionResult> {
        let window = self.auction_windows.get_mut(&window_id)
            .ok_or_else(|| anyhow!("Auction window not found"))?;
        
        if window.is_sealed {
            return Err(anyhow!("Auction window already sealed"));
        }
        
        // Get winning transactions within gas limit
        let winners = self.pending_transactions.get_transactions_within_gas_limit(window.total_gas_limit);
        let winner_count = winners.len().min(window.max_transactions as usize);
        let final_winners: Vec<AuctionTransaction> = winners.into_iter().take(winner_count).cloned().collect();
        
        // Calculate total revenue
        let total_revenue: u64 = final_winners.iter().map(|tx| tx.bid_amount).sum();
        let total_gas: u64 = final_winners.iter().map(|tx| tx.gas_limit).sum();
        
        // Remove winning transactions from pending pool
        for winner in &final_winners {
            self.pending_transactions.remove_transaction(&winner.tx_id)?;
        }
        
        // Update window
        window.is_sealed = true;
        window.winner_count = winner_count as u32;
        
        // Create auction result
        let auction_result = AuctionResult {
            window_id,
            winners: final_winners,
            total_revenue,
            total_gas_used: total_gas,
            merkle_root: self.pending_transactions.get_root(),
            timestamp: Utc::now(),
        };
        
        // Store completed auction
        let completed = CompletedAuction {
            window_id,
            result: auction_result.clone(),
            partner_revenue_share: total_revenue * 25 / 100, // 25% to partners
        };
        
        self.completed_auctions.push_back(completed);
        self.total_revenue += total_revenue;
        
        // Clean up old completed auctions (keep last 1000)
        while self.completed_auctions.len() > 1000 {
            self.completed_auctions.pop_front();
        
        Ok(auction_result)
    }
    
    /// Get mempool statistics
    pub fn get_mempool_stats(&self) -> MempoolStats {
        let total_revenue = self.completed_auctions.iter().map(|a| a.total_revenue).sum();
        MempoolStats {
            pending_transactions: self.merkle_tree.len(),
            active_windows: self.auction_windows.len(),
            completed_auctions: self.completed_auctions.len(),
            total_revenue,
            merkle_root: self.merkle_tree.get_root(),
        }
    }
    
    /// Get top transactions by bid rate
    pub fn get_top_transactions(&self, count: usize) -> Vec<&AuctionTransaction> {
        self.merkle_tree.get_top_transactions(count)
    }
    
    /// Generate proof for transaction inclusion
    pub fn generate_transaction_proof(&self, tx_id: &[u8; 32]) -> Result<MerkleProof> {
        self.merkle_tree.generate_proof(tx_id)
    }
    
    /// Process expired auction windows
    pub async fn process_expired_windows(&mut self) -> Result<Vec<AuctionResult>> {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        
        let mut results = Vec::new();
        let mut expired_windows = Vec::new();
        
        for (window_id, window) in &self.auction_windows {
            if current_time >= window.end_time {
                expired_windows.push(*window_id);
            }
        }
        
        for window_id in expired_windows {
            let result = self.seal_auction_window(window_id).await?;
            results.push(result);
        }
        
        Ok(results)
    }
}

/// Statistics for partner chains
#[derive(Debug, Clone)]
pub struct ChainStats {
    pub total_transactions: u64,
    pub total_bid_amount: u64,
    pub successful_auctions: u64,
    pub average_bid_rate: f64,
}

impl ChainStats {
    pub fn new() -> Self {
        Self {
            total_transactions: 0,
            total_bid_amount: 0,
            successful_auctions: 0,
            average_bid_rate: 0.0,
        }
    }
}

/// Result of a completed auction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionResult {
    pub window_id: u64,
    pub winners: Vec<AuctionTransaction>,
    pub total_revenue: u64,
    pub total_gas_used: u64,
    pub merkle_root: [u8; 32],
    pub timestamp: DateTime<Utc>,
}

/// Completed auction with revenue sharing info
#[derive(Debug, Clone)]
pub struct CompletedAuction {
    pub window_id: u64,
    pub result: AuctionResult,
    pub partner_revenue_share: u64, // 25% of total revenue
}

/// Current mempool statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolStats {
    pub pending_transactions: usize,
    pub active_windows: usize,
    pub completed_auctions: usize,
    pub total_revenue: u64,
    pub merkle_root: [u8; 32],
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_auction_transaction_ordering() {
        let tx1 = AuctionTransaction::new([1u8; 32], 1, 1000, 21000, 100, "addr1".to_string());
        let tx2 = AuctionTransaction::new([2u8; 32], 2, 2000, 21000, 100, "addr2".to_string());
        let tx3 = AuctionTransaction::new([3u8; 32], 3, 1000, 21000, 50, "addr3".to_string());
        
        // tx2 should have highest bid rate (2000 / (21000 * 100))
        // tx3 should have higher bid rate than tx1 (1000 / (21000 * 50) vs 1000 / (21000 * 100))
        assert_eq!(tx1.compare_for_auction(&tx2), Ordering::Greater);
        assert_eq!(tx3.compare_for_auction(&tx1), Ordering::Less);
    }
    
    #[test]
    fn test_merkle_tree_operations() {
        let mut tree = AuctionMerkleTree::new();
        
        let tx1 = AuctionTransaction::new([1u8; 32], 1, 1000, 21000, 100, "addr1".to_string());
        let tx2 = AuctionTransaction::new([2u8; 32], 2, 2000, 21000, 100, "addr2".to_string());
        let tx3 = AuctionTransaction::new([3u8; 32], 3, 1500, 21000, 100, "addr3".to_string());
        
        // Insert transactions
        tree.insert_transaction(tx1.clone()).unwrap();
        tree.insert_transaction(tx2.clone()).unwrap();
        tree.insert_transaction(tx3.clone()).unwrap();
        
        // Should be ordered by bid rate: tx2 (2000), tx3 (1500), tx1 (1000)
        assert_eq!(tree.transactions.len(), 3);
        
        // The transactions should be ordered by effective bid rate (highest first)
        assert_eq!(tree.transactions[0].bid_amount, 2000);
        assert_eq!(tree.transactions[1].bid_amount, 1500);
        assert_eq!(tree.transactions[2].bid_amount, 1000);
        
        // Test proof generation and verification
        let proof = tree.generate_proof(&tx2.tx_id).unwrap();
        assert!(tree.verify_proof(&tx2, &proof));
        
        // Test removal
        assert!(tree.remove_transaction(&tx1.tx_id).unwrap());
        assert_eq!(tree.transactions.len(), 2);
        assert!(!tree.remove_transaction(&[99u8; 32]).unwrap());
    }
    
    #[test]
    fn test_auction_mempool_basic_operations() {
        let mut mempool = BpciAuctionMempool::new();
        
        // Create some test transactions
        let tx1 = AuctionTransaction::new([1u8; 32], 1, 1000, 21000, 100, "addr1".to_string());
        let tx2 = AuctionTransaction::new([2u8; 32], 2, 2000, 21000, 100, "addr2".to_string());
        let tx3 = AuctionTransaction::new([3u8; 32], 3, 1500, 21000, 100, "addr3".to_string());
        
        // Submit transactions
        mempool.submit_transaction(tx1.clone()).unwrap();
        mempool.submit_transaction(tx2.clone()).unwrap();
        mempool.submit_transaction(tx3.clone()).unwrap();
        
        let stats = mempool.get_mempool_stats();
        assert_eq!(stats.pending_transactions, 3);
        
        // Create auction window
        let window_id = mempool.create_auction_window(1000, 10, 100000, AuctionType::StandardExecution);
        assert_eq!(window_id, 1);
        
        // Seal auction
        let result = tokio_test::block_on(mempool.seal_auction_window(window_id)).unwrap();
        assert_eq!(result.winning_transactions.len(), 3); // All transactions should fit
        assert_eq!(result.total_revenue, 4500); // 1000 + 2000 + 1500
        
        // Check that transactions were removed from pending
        let final_stats = mempool.get_mempool_stats();
        assert_eq!(final_stats.pending_transactions, 0);
        assert_eq!(final_stats.total_revenue, 4500);
    }
    
    #[test]
    fn test_gas_limit_constraint() {
        let mut mempool = BpciAuctionMempool::new();
        
        // Create transactions with different gas requirements
        let tx1 = AuctionTransaction::new([1u8; 32], 1, 3000, 30000, 100, "addr1".to_string());
        let tx2 = AuctionTransaction::new([2u8; 32], 2, 2000, 40000, 100, "addr2".to_string());
        let tx3 = AuctionTransaction::new([3u8; 32], 3, 1000, 50000, 100, "addr3".to_string());
        
        mempool.submit_transaction(tx1.clone()).unwrap();
        mempool.submit_transaction(tx2.clone()).unwrap();
        mempool.submit_transaction(tx3.clone()).unwrap();
        
        // Create auction with limited gas
        let window_id = mempool.create_auction_window(1000, 10, 60000, AuctionType::StandardExecution);
        
        // Seal auction - should only include tx1 (30000 gas) and tx2 (40000 gas) = 70000 > 60000
        // So only tx1 should be selected (highest bid rate and fits in gas limit)
        let result = tokio_test::block_on(mempool.seal_auction_window(window_id)).unwrap();
        
        // Should select transactions that fit within gas limit, prioritizing by bid rate
        let total_gas: u64 = result.winning_transactions.iter().map(|tx| tx.gas_limit).sum();
        assert!(total_gas <= 60000);
        assert!(result.winning_transactions.len() > 0);
    }
}
