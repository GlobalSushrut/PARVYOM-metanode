//! Merkle rollups and micro-receipt logic for ZIPLOCK-JSON

use blake3::{Hasher, Hash};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::{ZjlResult, ZjlError};
use crate::blocks::{SecondRoot, MinuteRoot, HourRoot, DayRoot, BlockType, Block};

/// Merkle tree node
#[derive(Debug, Clone, PartialEq)]
pub struct MerkleNode {
    pub hash: [u8; 32],
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    pub fn leaf(data: &[u8]) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(b"ZJL:LEAF:");
        hasher.update(data);
        let hash = hasher.finalize();
        
        Self {
            hash: *hash.as_bytes(),
            left: None,
            right: None,
        }
    }

    pub fn branch(left: MerkleNode, right: MerkleNode) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(b"ZJL:BRANCH:");
        hasher.update(&left.hash);
        hasher.update(&right.hash);
        let hash = hasher.finalize();

        Self {
            hash: *hash.as_bytes(),
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

/// Merkle tree builder
pub struct MerkleTreeBuilder {
    leaves: Vec<MerkleNode>,
}

impl MerkleTreeBuilder {
    pub fn new() -> Self {
        Self {
            leaves: Vec::new(),
        }
    }

    pub fn add_leaf(&mut self, data: &[u8]) {
        self.leaves.push(MerkleNode::leaf(data));
    }

    pub fn add_block(&mut self, block: &Block) {
        // Create leaf from block hash
        self.add_leaf(&block.header.hash);
    }

    pub fn build(mut self) -> ZjlResult<MerkleNode> {
        if self.leaves.is_empty() {
            return Err(ZjlError::InvalidMerkleTree("No leaves provided".to_string()));
        }

        if self.leaves.len() == 1 {
            return Ok(self.leaves.into_iter().next().unwrap());
        }

        // Build tree bottom-up
        let mut current_level = self.leaves;
        
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            
            // Process pairs
            for chunk in current_level.chunks(2) {
                if chunk.len() == 2 {
                    let left = chunk[0].clone();
                    let right = chunk[1].clone();
                    next_level.push(MerkleNode::branch(left, right));
                } else {
                    // Odd number of nodes - duplicate the last one
                    let left = chunk[0].clone();
                    let right = chunk[0].clone();
                    next_level.push(MerkleNode::branch(left, right));
                }
            }
            
            current_level = next_level;
        }

        Ok(current_level.into_iter().next().unwrap())
    }
}

/// Merkle proof for inclusion verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    /// Path from leaf to root
    pub path: Vec<ProofStep>,
    /// Root hash
    pub root: [u8; 32],
    /// Leaf index
    pub leaf_index: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofStep {
    /// Hash of sibling node
    pub sibling_hash: [u8; 32],
    /// True if sibling is on the right
    pub is_right: bool,
}

impl MerkleProof {
    /// Verify that the leaf data produces the root hash
    pub fn verify(&self, leaf_data: &[u8]) -> bool {
        let mut current_hash = MerkleNode::leaf(leaf_data).hash;
        
        for step in &self.path {
            let mut hasher = Hasher::new();
            hasher.update(b"ZJL:BRANCH:");
            
            if step.is_right {
                hasher.update(&current_hash);
                hasher.update(&step.sibling_hash);
            } else {
                hasher.update(&step.sibling_hash);
                hasher.update(&current_hash);
            }
            
            current_hash = *hasher.finalize().as_bytes();
        }
        
        current_hash == self.root
    }
}

/// Micro-receipt for individual audit events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroReceipt {
    /// Event timestamp (nanoseconds since Unix epoch)
    pub timestamp_ns: u64,
    /// Event type identifier
    pub event_type: String,
    /// VM identifier
    pub vm_id: String,
    /// Event payload hash
    pub payload_hash: [u8; 32],
    /// Sequence number within the second
    pub sequence: u32,
    /// Merkle proof of inclusion in second root
    pub merkle_proof: Option<MerkleProof>,
}

impl MicroReceipt {
    pub fn new(
        event_type: String,
        vm_id: String,
        payload: &[u8],
        sequence: u32,
    ) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(payload);
        let payload_hash = *hasher.finalize().as_bytes();

        Self {
            timestamp_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
            event_type,
            vm_id,
            payload_hash,
            sequence,
            merkle_proof: None,
        }
    }

    /// Serialize to bytes for hashing
    pub fn to_bytes(&self) -> ZjlResult<Vec<u8>> {
        serde_json::to_vec(self)
            .map_err(|e| ZjlError::SerializationErrorString(e.to_string()))
    }

    /// Calculate hash of this receipt
    pub fn hash(&self) -> ZjlResult<[u8; 32]> {
        let bytes = self.to_bytes()?;
        let mut hasher = Hasher::new();
        hasher.update(b"ZJL:RECEIPT:");
        hasher.update(&bytes);
        Ok(*hasher.finalize().as_bytes())
    }
}

/// Rollup manager for hierarchical Merkle roots
pub struct RollupManager {
    /// Receipts for current second
    current_second_receipts: Vec<MicroReceipt>,
    /// Current second timestamp
    current_second: u64,
    /// Second roots for current minute
    current_minute_seconds: BTreeMap<u64, SecondRoot>,
    /// Current minute timestamp
    current_minute: u64,
    /// Minute roots for current hour
    current_hour_minutes: BTreeMap<u64, MinuteRoot>,
    /// Current hour timestamp
    current_hour: u64,
    /// Hour roots for current day
    current_day_hours: BTreeMap<u64, HourRoot>,
    /// Current day timestamp
    current_day: u64,
}

impl RollupManager {
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        let current_second = now.timestamp() as u64;
        let current_minute = current_second / 60;
        let current_hour = current_minute / 60;
        let current_day = current_hour / 24;

        Self {
            current_second_receipts: Vec::new(),
            current_second,
            current_minute_seconds: BTreeMap::new(),
            current_minute,
            current_hour_minutes: BTreeMap::new(),
            current_hour,
            current_day_hours: BTreeMap::new(),
            current_day,
        }
    }

    /// Add a micro-receipt to the current second
    pub fn add_receipt(&mut self, mut receipt: MicroReceipt) -> ZjlResult<()> {
        let receipt_second = receipt.timestamp_ns / 1_000_000_000;

        // Check if we need to roll up the current second
        if receipt_second != self.current_second {
            self.rollup_current_second()?;
            self.current_second = receipt_second;
        }

        // Set sequence number
        receipt.sequence = self.current_second_receipts.len() as u32;
        self.current_second_receipts.push(receipt);

        Ok(())
    }

    /// Roll up current second into a SecondRoot
    pub fn rollup_current_second(&mut self) -> ZjlResult<Option<SecondRoot>> {
        if self.current_second_receipts.is_empty() {
            return Ok(None);
        }

        // Build Merkle tree from receipts
        let mut builder = MerkleTreeBuilder::new();
        for receipt in &self.current_second_receipts {
            let receipt_bytes = receipt.to_bytes()?;
            builder.add_leaf(&receipt_bytes);
        }

        let tree = builder.build()?;
        let second_root = SecondRoot {
            ts_sec: self.current_second,
            count: self.current_second_receipts.len() as u32,
            root: tree.hash,
        };

        // Store second root
        self.current_minute_seconds.insert(self.current_second, second_root.clone());

        // Check if we need to roll up the minute
        let receipt_minute = self.current_second / 60;
        if receipt_minute != self.current_minute {
            self.rollup_current_minute()?;
            self.current_minute = receipt_minute;
        }

        // Clear current second receipts
        self.current_second_receipts.clear();

        Ok(Some(second_root))
    }

    /// Roll up current minute into a MinuteRoot
    pub fn rollup_current_minute(&mut self) -> ZjlResult<Option<MinuteRoot>> {
        if self.current_minute_seconds.is_empty() {
            return Ok(None);
        }

        // Build Merkle tree from second roots
        let mut builder = MerkleTreeBuilder::new();
        for second_root in self.current_minute_seconds.values() {
            builder.add_leaf(&second_root.root);
        }

        let tree = builder.build()?;
        
        // Generate BPI transaction hash (placeholder - would integrate with actual BPI)
        let mut hasher = Hasher::new();
        hasher.update(b"ZJL:BPI_TX:");
        hasher.update(&tree.hash);
        hasher.update(&self.current_minute.to_le_bytes());
        let bpi_tx = *hasher.finalize().as_bytes();

        let minute_root = MinuteRoot {
            ts_min: self.current_minute,
            root: tree.hash,
            bpi_tx,
        };

        // Store minute root
        self.current_hour_minutes.insert(self.current_minute, minute_root.clone());

        // Check if we need to roll up the hour
        let receipt_hour = self.current_minute / 60;
        if receipt_hour != self.current_hour {
            self.rollup_current_hour()?;
            self.current_hour = receipt_hour;
        }

        // Clear current minute seconds
        self.current_minute_seconds.clear();

        Ok(Some(minute_root))
    }

    /// Roll up current hour into an HourRoot
    pub fn rollup_current_hour(&mut self) -> ZjlResult<Option<HourRoot>> {
        if self.current_hour_minutes.is_empty() {
            return Ok(None);
        }

        // Build Merkle tree from minute roots
        let mut builder = MerkleTreeBuilder::new();
        for minute_root in self.current_hour_minutes.values() {
            builder.add_leaf(&minute_root.root);
        }

        let tree = builder.build()?;
        let hour_root = HourRoot {
            ts_hour: self.current_hour,
            root: tree.hash,
        };

        // Store hour root
        self.current_day_hours.insert(self.current_hour, hour_root.clone());

        // Check if we need to roll up the day
        let receipt_day = self.current_hour / 24;
        if receipt_day != self.current_day {
            self.rollup_current_day()?;
            self.current_day = receipt_day;
        }

        // Clear current hour minutes
        self.current_hour_minutes.clear();

        Ok(Some(hour_root))
    }

    /// Roll up current day into a DayRoot
    pub fn rollup_current_day(&mut self) -> ZjlResult<Option<DayRoot>> {
        if self.current_day_hours.is_empty() {
            return Ok(None);
        }

        // Build Merkle tree from hour roots
        let mut builder = MerkleTreeBuilder::new();
        for hour_root in self.current_day_hours.values() {
            builder.add_leaf(&hour_root.root);
        }

        let tree = builder.build()?;
        let day_root = DayRoot {
            ts_day: self.current_day,
            root: tree.hash,
        };

        // Clear current day hours
        self.current_day_hours.clear();

        Ok(Some(day_root))
    }

    /// Force rollup of all pending levels
    pub fn force_rollup(&mut self) -> ZjlResult<()> {
        self.rollup_current_second()?;
        self.rollup_current_minute()?;
        self.rollup_current_hour()?;
        self.rollup_current_day()?;
        Ok(())
    }

    /// Get current statistics
    pub fn stats(&self) -> RollupStats {
        RollupStats {
            pending_receipts: self.current_second_receipts.len(),
            pending_seconds: self.current_minute_seconds.len(),
            pending_minutes: self.current_hour_minutes.len(),
            pending_hours: self.current_day_hours.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RollupStats {
    pub pending_receipts: usize,
    pub pending_seconds: usize,
    pub pending_minutes: usize,
    pub pending_hours: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_tree_single_leaf() {
        let mut builder = MerkleTreeBuilder::new();
        builder.add_leaf(b"test data");
        
        let tree = builder.build().unwrap();
        assert!(tree.is_leaf());
    }

    #[test]
    fn test_merkle_tree_multiple_leaves() {
        let mut builder = MerkleTreeBuilder::new();
        builder.add_leaf(b"data1");
        builder.add_leaf(b"data2");
        builder.add_leaf(b"data3");
        
        let tree = builder.build().unwrap();
        assert!(!tree.is_leaf());
    }

    #[test]
    fn test_micro_receipt_creation() {
        let receipt = MicroReceipt::new(
            "test_event".to_string(),
            "test_vm".to_string(),
            b"test payload",
            0,
        );

        assert_eq!(receipt.event_type, "test_event");
        assert_eq!(receipt.vm_id, "test_vm");
        assert_eq!(receipt.sequence, 0);
        assert!(receipt.hash().is_ok());
    }

    #[test]
    fn test_rollup_manager() {
        let mut manager = RollupManager::new();
        
        let receipt1 = MicroReceipt::new(
            "event1".to_string(),
            "vm1".to_string(),
            b"payload1",
            0,
        );
        
        let receipt2 = MicroReceipt::new(
            "event2".to_string(),
            "vm2".to_string(),
            b"payload2",
            0,
        );

        assert!(manager.add_receipt(receipt1).is_ok());
        assert!(manager.add_receipt(receipt2).is_ok());

        let stats = manager.stats();
        assert_eq!(stats.pending_receipts, 2);
    }

    #[test]
    fn test_second_rollup() {
        let mut manager = RollupManager::new();
        
        let receipt = MicroReceipt::new(
            "test_event".to_string(),
            "test_vm".to_string(),
            b"test payload",
            0,
        );

        manager.add_receipt(receipt).unwrap();
        let second_root = manager.rollup_current_second().unwrap();
        
        assert!(second_root.is_some());
        let root = second_root.unwrap();
        let count = root.count;
        assert_eq!(count, 1);
    }
}
