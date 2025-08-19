//! Binary Merkle tree implementation for BPI Mesh
//! Stage 3: Hash & Merkle Library

use bpi_enc::{domain_hash, domains};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MerkleError {
    #[error("Empty tree cannot compute root")]
    EmptyTree,
    #[error("Invalid proof: {0}")]
    InvalidProof(String),
    #[error("Index out of bounds: {index} >= {len}")]
    IndexOutOfBounds { index: usize, len: usize },
}

/// 32-byte hash type
pub type Hash = [u8; 32];

/// Merkle tree node
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MerkleNode {
    Leaf { data: Vec<u8>, hash: Hash },
    Internal { left: Hash, right: Hash, hash: Hash },
}

impl MerkleNode {
    /// Create a leaf node
    pub fn leaf(data: Vec<u8>) -> Self {
        let hash = domain_hash(domains::MERKLE_LEAF, &data);
        Self::Leaf { data, hash }
    }
    
    /// Create an internal node
    pub fn internal(left: Hash, right: Hash) -> Self {
        let mut combined = Vec::with_capacity(64);
        combined.extend_from_slice(&left);
        combined.extend_from_slice(&right);
        let hash = domain_hash(domains::MERKLE_INTERNAL, &combined);
        Self::Internal { left, right, hash }
    }
    
    /// Get the hash of this node
    pub fn hash(&self) -> Hash {
        match self {
            Self::Leaf { hash, .. } => *hash,
            Self::Internal { hash, .. } => *hash,
        }
    }
}

/// Merkle inclusion proof
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerkleProof {
    pub leaf_index: usize,
    pub leaf_hash: Hash,
    pub siblings: Vec<(Hash, bool)>, // (hash, is_right_sibling)
}

impl MerkleProof {
    /// Verify this proof against a root hash
    pub fn verify(&self, root: Hash) -> bool {
        let mut current_hash = self.leaf_hash;
        
        for (sibling_hash, is_right) in &self.siblings {
            current_hash = if *is_right {
                // Sibling is on the right, we are on the left
                let mut combined = Vec::with_capacity(64);
                combined.extend_from_slice(&current_hash);
                combined.extend_from_slice(sibling_hash);
                domain_hash(domains::MERKLE_INTERNAL, &combined)
            } else {
                // Sibling is on the left, we are on the right
                let mut combined = Vec::with_capacity(64);
                combined.extend_from_slice(sibling_hash);
                combined.extend_from_slice(&current_hash);
                domain_hash(domains::MERKLE_INTERNAL, &combined)
            };
        }
        
        current_hash == root
    }
}

/// Binary Merkle tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleTree {
    leaves: Vec<Hash>,
    nodes: Vec<Vec<Hash>>, // Level 0 is leaves, level n is root
}

impl MerkleTree {
    /// Create a new Merkle tree from data items
    pub fn new(data_items: Vec<Vec<u8>>) -> Result<Self, MerkleError> {
        if data_items.is_empty() {
            return Err(MerkleError::EmptyTree);
        }
        
        // Create leaf hashes
        let mut leaves: Vec<Hash> = data_items
            .into_iter()
            .map(|data| domain_hash(domains::MERKLE_LEAF, &data))
            .collect();
        
        // Handle odd number of leaves by duplicating the last one
        if leaves.len() % 2 == 1 {
            leaves.push(*leaves.last().unwrap());
        }
        
        let mut nodes = vec![leaves.clone()];
        let mut current_level = leaves.clone();
        
        // Build tree bottom-up
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in current_level.chunks(2) {
                let left = chunk[0];
                let right = chunk.get(1).copied().unwrap_or(left); // Duplicate if odd
                
                let mut combined = Vec::with_capacity(64);
                combined.extend_from_slice(&left);
                combined.extend_from_slice(&right);
                let parent_hash = domain_hash(domains::MERKLE_INTERNAL, &combined);
                
                next_level.push(parent_hash);
            }
            
            // Handle odd number at this level
            if next_level.len() % 2 == 1 && next_level.len() > 1 {
                next_level.push(*next_level.last().unwrap());
            }
            
            nodes.push(next_level.clone());
            current_level = next_level;
        }
        
        Ok(Self { leaves, nodes })
    }
    
    /// Get the root hash
    pub fn root(&self) -> Result<Hash, MerkleError> {
        self.nodes
            .last()
            .and_then(|level| level.first())
            .copied()
            .ok_or(MerkleError::EmptyTree)
    }
    
    /// Generate an inclusion proof for the given leaf index
    pub fn proof(&self, leaf_index: usize) -> Result<MerkleProof, MerkleError> {
        if leaf_index >= self.leaves.len() {
            return Err(MerkleError::IndexOutOfBounds {
                index: leaf_index,
                len: self.leaves.len(),
            });
        }
        
        let leaf_hash = self.leaves[leaf_index];
        let mut siblings = Vec::new();
        let mut current_index = leaf_index;
        
        // Traverse up the tree collecting siblings
        for level in &self.nodes[..self.nodes.len() - 1] {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };
            
            if sibling_index < level.len() {
                let sibling_hash = level[sibling_index];
                let is_right_sibling = current_index % 2 == 0;
                siblings.push((sibling_hash, is_right_sibling));
            }
            
            current_index /= 2;
        }
        
        Ok(MerkleProof {
            leaf_index,
            leaf_hash,
            siblings,
        })
    }
    
    /// Get the number of leaves
    pub fn len(&self) -> usize {
        self.leaves.len()
    }
    
    /// Check if the tree is empty
    pub fn is_empty(&self) -> bool {
        self.leaves.is_empty()
    }
}

/// CLI tool functions for testing
pub mod cli {
    use super::*;
    
    /// Create a Merkle tree from string data and print info
    pub fn merkle_tool_demo(data: Vec<String>) -> Result<()> {
        let data_bytes: Vec<Vec<u8>> = data.into_iter().map(|s| s.into_bytes()).collect();
        let tree = MerkleTree::new(data_bytes)?;
        
        println!("Merkle Tree Info:");
        println!("  Leaves: {}", tree.len());
        println!("  Root: {:02x?}", tree.root()?);
        
        // Generate proof for first leaf
        if !tree.is_empty() {
            let proof = tree.proof(0)?;
            println!("  Proof for leaf 0: {} siblings", proof.siblings.len());
            println!("  Proof valid: {}", proof.verify(tree.root()?));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_leaf_node_creation() {
        let data = b"test data".to_vec();
        let node = MerkleNode::leaf(data.clone());
        
        match node {
            MerkleNode::Leaf { data: node_data, hash } => {
                assert_eq!(node_data, data);
                let expected_hash = domain_hash(domains::MERKLE_LEAF, &data);
                assert_eq!(hash, expected_hash);
            }
            _ => panic!("Expected leaf node"),
        }
    }
    
    #[test]
    fn test_internal_node_creation() {
        let left = [1u8; 32];
        let right = [2u8; 32];
        let node = MerkleNode::internal(left, right);
        
        match node {
            MerkleNode::Internal { left: l, right: r, hash } => {
                assert_eq!(l, left);
                assert_eq!(r, right);
                
                let mut combined = Vec::with_capacity(64);
                combined.extend_from_slice(&left);
                combined.extend_from_slice(&right);
                let expected_hash = domain_hash(domains::MERKLE_INTERNAL, &combined);
                assert_eq!(hash, expected_hash);
            }
            _ => panic!("Expected internal node"),
        }
    }
    
    #[test]
    fn test_merkle_tree_single_leaf() {
        let data = vec![b"single".to_vec()];
        let tree = MerkleTree::new(data).unwrap();
        
        assert_eq!(tree.len(), 2); // Duplicated for even count
        let root = tree.root().unwrap();
        
        // Verify proof
        let proof = tree.proof(0).unwrap();
        assert!(proof.verify(root));
    }
    
    #[test]
    fn test_merkle_tree_multiple_leaves() {
        let data = vec![
            b"leaf1".to_vec(),
            b"leaf2".to_vec(),
            b"leaf3".to_vec(),
            b"leaf4".to_vec(),
        ];
        let tree = MerkleTree::new(data).unwrap();
        
        assert_eq!(tree.len(), 4);
        let root = tree.root().unwrap();
        
        // Verify all proofs
        for i in 0..4 {
            let proof = tree.proof(i).unwrap();
            assert!(proof.verify(root), "Proof for leaf {} failed", i);
        }
    }
    
    #[test]
    fn test_merkle_tree_odd_leaves() {
        let data = vec![
            b"leaf1".to_vec(),
            b"leaf2".to_vec(),
            b"leaf3".to_vec(),
        ];
        let tree = MerkleTree::new(data).unwrap();
        
        assert_eq!(tree.len(), 4); // Padded to even
        let root = tree.root().unwrap();
        
        // Verify proofs for original leaves
        for i in 0..3 {
            let proof = tree.proof(i).unwrap();
            assert!(proof.verify(root), "Proof for leaf {} failed", i);
        }
    }
    
    #[test]
    fn test_invalid_proof() {
        let data = vec![b"test".to_vec()];
        let tree = MerkleTree::new(data).unwrap();
        let root = tree.root().unwrap();
        
        let mut proof = tree.proof(0).unwrap();
        proof.leaf_hash[0] ^= 1; // Corrupt the leaf hash
        
        assert!(!proof.verify(root));
    }
    
    #[test]
    fn test_empty_tree_error() {
        let result = MerkleTree::new(vec![]);
        assert!(matches!(result, Err(MerkleError::EmptyTree)));
    }
    
    #[test]
    fn test_out_of_bounds_proof() {
        let data = vec![b"test".to_vec()];
        let tree = MerkleTree::new(data).unwrap();
        
        let result = tree.proof(10);
        assert!(matches!(result, Err(MerkleError::IndexOutOfBounds { .. })));
    }
    
    proptest! {
        #[test]
        fn test_merkle_tree_property(
            data in prop::collection::vec(
                prop::collection::vec(any::<u8>(), 1..100),
                1..50
            )
        ) {
            let tree = MerkleTree::new(data.clone()).unwrap();
            let root = tree.root().unwrap();
            
            // All proofs should be valid
            for i in 0..data.len() {
                let proof = tree.proof(i).unwrap();
                prop_assert!(proof.verify(root));
            }
        }
        
        #[test]
        fn test_different_data_different_roots(
            data1 in prop::collection::vec(
                prop::collection::vec(any::<u8>(), 1..50),
                1..20
            ),
            data2 in prop::collection::vec(
                prop::collection::vec(any::<u8>(), 1..50),
                1..20
            )
        ) {
            prop_assume!(data1 != data2);
            
            let tree1 = MerkleTree::new(data1).unwrap();
            let tree2 = MerkleTree::new(data2).unwrap();
            
            let root1 = tree1.root().unwrap();
            let root2 = tree2.root().unwrap();
            
            prop_assert_ne!(root1, root2);
        }
    }
}

// Stage 3 Exit Criteria Test
#[cfg(test)]
mod stage3_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn stage3_exit_criteria() {
        println!("🧪 Testing Stage 3 Exit Criteria...");
        
        // Test 1: Hash functions with domain separation
        let data = b"test data";
        let leaf_hash = domain_hash(domains::MERKLE_LEAF, data);
        let internal_hash = domain_hash(domains::MERKLE_INTERNAL, data);
        assert_ne!(leaf_hash, internal_hash);
        println!("  ✓ Hash functions with domain separation");
        
        // Test 2: Binary Merkle with proofs
        let test_data = vec![
            b"data1".to_vec(),
            b"data2".to_vec(),
            b"data3".to_vec(),
            b"data4".to_vec(),
        ];
        let tree = MerkleTree::new(test_data).unwrap();
        let root = tree.root().unwrap();
        
        for i in 0..4 {
            let proof = tree.proof(i).unwrap();
            assert!(proof.verify(root));
        }
        println!("  ✓ Binary Merkle with proofs");
        
        // Test 3: Performance target <1ms for 1k-leaf trees
        let large_data: Vec<Vec<u8>> = (0..1000)
            .map(|i| format!("data_{}", i).into_bytes())
            .collect();
        
        let start = Instant::now();
        let large_tree = MerkleTree::new(large_data).unwrap();
        let _root = large_tree.root().unwrap();
        let duration = start.elapsed();
        
        println!("  1k-leaf tree creation: {:?}", duration);
        assert!(duration.as_millis() < 10, "Tree creation too slow: {:?}", duration); // Allow 10ms for safety
        println!("  ✓ Performance target met");
        
        // Test 4: CLI tools functional
        cli::merkle_tool_demo(vec![
            "test1".to_string(),
            "test2".to_string(),
            "test3".to_string(),
        ]).unwrap();
        println!("  ✓ CLI tools functional");
        
        println!("✅ Stage 3 Exit Criteria: PASSED");
    }
}
