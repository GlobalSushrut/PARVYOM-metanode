//! # Merkle Tree for Address Verification
//! 
//! Cryptographic verification of virtual address assignments.

use blake3;
use serde::{Serialize, Deserialize};

/// Merkle proof for address verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    /// Leaf hash
    pub leaf: [u8; 32],
    
    /// Proof path (sibling hashes)
    pub path: Vec<[u8; 32]>,
    
    /// Indices (left=0, right=1)
    pub indices: Vec<bool>,
}

impl MerkleProof {
    /// Create new merkle proof
    pub fn new(leaf: [u8; 32], path: Vec<[u8; 32]>, indices: Vec<bool>) -> Self {
        Self { leaf, path, indices }
    }
    
    /// Verify proof against merkle root
    pub fn verify(&self, root: &[u8; 32], data: &[u8]) -> bool {
        // Hash the data to get leaf
        let computed_leaf = blake3::hash(data);
        
        if computed_leaf.as_bytes() != &self.leaf {
            return false;
        }
        
        // Compute root from proof
        let mut current = self.leaf;
        for (sibling, is_right) in self.path.iter().zip(&self.indices) {
            current = if *is_right {
                Self::hash_pair(sibling, &current)
            } else {
                Self::hash_pair(&current, sibling)
            };
        }
        
        &current == root
    }
    
    /// Hash two nodes
    fn hash_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(left);
        hasher.update(right);
        *hasher.finalize().as_bytes()
    }
}

impl Default for MerkleProof {
    fn default() -> Self {
        Self {
            leaf: [0u8; 32],
            path: Vec::new(),
            indices: Vec::new(),
        }
    }
}

/// Merkle tree for address registry
#[derive(Debug, Clone)]
pub struct MerkleTree {
    /// Tree leaves (address hashes)
    leaves: Vec<[u8; 32]>,
    
    /// Cached root
    root: [u8; 32],
}

impl MerkleTree {
    /// Create new merkle tree from leaves
    pub fn new(data: Vec<Vec<u8>>) -> Self {
        let leaves: Vec<[u8; 32]> = data.iter()
            .map(|d| *blake3::hash(d).as_bytes())
            .collect();
        
        let root = Self::compute_root(&leaves);
        
        Self { leaves, root }
    }
    
    /// Get merkle root
    pub fn root(&self) -> [u8; 32] {
        self.root
    }
    
    /// Generate proof for data at index
    pub fn generate_proof(&self, index: usize) -> Option<MerkleProof> {
        if index >= self.leaves.len() {
            return None;
        }
        
        let leaf = self.leaves[index];
        let mut path = Vec::new();
        let mut indices = Vec::new();
        
        let mut current_index = index;
        let mut current_level = self.leaves.clone();
        
        while current_level.len() > 1 {
            let is_right = current_index % 2 == 1;
            let sibling_index = if is_right {
                current_index - 1
            } else {
                if current_index + 1 < current_level.len() {
                    current_index + 1
                } else {
                    current_index  // No sibling (odd number of nodes)
                }
            };
            
            if sibling_index < current_level.len() {
                path.push(current_level[sibling_index]);
                indices.push(is_right);
            }
            
            // Move to next level
            current_level = Self::compute_next_level(&current_level);
            current_index /= 2;
        }
        
        Some(MerkleProof::new(leaf, path, indices))
    }
    
    /// Compute merkle root from leaves
    fn compute_root(leaves: &[[u8; 32]]) -> [u8; 32] {
        if leaves.is_empty() {
            return [0u8; 32];
        }
        
        let mut current_level = leaves.to_vec();
        
        while current_level.len() > 1 {
            current_level = Self::compute_next_level(&current_level);
        }
        
        current_level[0]
    }
    
    /// Compute next level of tree
    fn compute_next_level(level: &[[u8; 32]]) -> Vec<[u8; 32]> {
        let mut next_level = Vec::new();
        
        for i in (0..level.len()).step_by(2) {
            let left = level[i];
            let right = if i + 1 < level.len() {
                level[i + 1]
            } else {
                left  // Duplicate if odd number
            };
            
            let mut hasher = blake3::Hasher::new();
            hasher.update(&left);
            hasher.update(&right);
            next_level.push(*hasher.finalize().as_bytes());
        }
        
        next_level
    }
    
    /// Rebuild tree with new data
    pub fn rebuild(&mut self, data: Vec<Vec<u8>>) {
        self.leaves = data.iter()
            .map(|d| *blake3::hash(d).as_bytes())
            .collect();
        self.root = Self::compute_root(&self.leaves);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_tree_creation() {
        let data = vec![
            b"address1".to_vec(),
            b"address2".to_vec(),
            b"address3".to_vec(),
            b"address4".to_vec(),
        ];
        
        let tree = MerkleTree::new(data);
        assert_ne!(tree.root(), [0u8; 32]);
    }
    
    #[test]
    fn test_merkle_proof_verification() {
        let data = vec![
            b"address1".to_vec(),
            b"address2".to_vec(),
            b"address3".to_vec(),
            b"address4".to_vec(),
        ];
        
        let tree = MerkleTree::new(data.clone());
        let proof = tree.generate_proof(0).unwrap();
        
        assert!(proof.verify(&tree.root(), &data[0]));
    }
    
    #[test]
    fn test_merkle_proof_invalid() {
        let data = vec![
            b"address1".to_vec(),
            b"address2".to_vec(),
        ];
        
        let tree = MerkleTree::new(data);
        let proof = tree.generate_proof(0).unwrap();
        
        // Wrong data should fail
        assert!(!proof.verify(&tree.root(), b"wrong_address"));
    }
}
