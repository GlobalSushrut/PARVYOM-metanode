// QGC DAG - Metadata-only bounded DAG for QGC-C² consensus
// Ultra-lightweight DAG with ≤3 parents per batch, fixed memory bounds

use crate::logbook_6d_bridge::qgc_core::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};
use blake3;

/// DAG Configuration (hardware-aware bounds)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagConfig {
    pub max_batches: usize,              // Maximum batches in DAG (e.g., 512)
    pub max_parents: u8,                 // Maximum parents per batch (≤3)
    pub gc_threshold: usize,             // Garbage collection threshold
    pub max_depth: u32,                  // Maximum DAG depth
}

impl Default for DagConfig {
    fn default() -> Self {
        Self {
            max_batches: 512,
            max_parents: 3,
            gc_threshold: 400,
            max_depth: 1000,
        }
    }
}

/// DAG Node Metadata (minimal, no transaction data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagNode {
    pub batch_id: [u8; 32],              // 32B - batch identifier
    pub parent_ids: Vec<[u8; 32]>,       // Variable - parent references (≤3)
    pub children_ids: Vec<[u8; 32]>,     // Variable - children references
    pub height: u32,                     // 4B - DAG height
    pub strand: u16,                     // 2B - strand identifier
    pub timestamp: u64,                  // 8B - creation timestamp
    pub status: NodeStatus,              // 1B - node status
    pub cc_round: Option<u64>,           // 8B - CC round if available
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeStatus {
    Pending,                             // Waiting for parents
    Ready,                               // Ready for consensus
    Committed,                           // Committed via two-link rule
    Finalized,                           // Finalized and can be GC'd
}

impl DagNode {
    pub fn new(batch: &Batch) -> Self {
        Self {
            batch_id: batch.id,
            parent_ids: batch.get_parents(),
            children_ids: Vec::new(),
            height: 0, // Will be computed when added to DAG
            strand: batch.strand,
            timestamp: batch.timestamp,
            status: NodeStatus::Pending,
            cc_round: None,
        }
    }
    
    pub fn size_bytes(&self) -> usize {
        64 + (self.parent_ids.len() * 32) + (self.children_ids.len() * 32)
    }
}

/// Bounded DAG for QGC-C² consensus
#[derive(Debug)]
pub struct QgcDag {
    nodes: HashMap<[u8; 32], DagNode>,   // All nodes indexed by batch_id
    tips: HashSet<[u8; 32]>,             // Current DAG tips (no children)
    committed: VecDeque<[u8; 32]>,       // Committed batch queue
    finalized: VecDeque<[u8; 32]>,       // Finalized batch queue
    config: DagConfig,
    genesis_id: Option<[u8; 32]>,        // Genesis batch ID
    max_height: u32,                     // Current maximum height
}

impl QgcDag {
    pub fn new(config: DagConfig) -> Self {
        Self {
            nodes: HashMap::new(),
            tips: HashSet::new(),
            committed: VecDeque::new(),
            finalized: VecDeque::new(),
            config,
            genesis_id: None,
            max_height: 0,
        }
    }
    
    /// Add batch to DAG with parent validation
    pub fn add_batch(&mut self, batch: &Batch) -> Result<(), String> {
        if self.nodes.contains_key(&batch.id) {
            return Err("Batch already exists in DAG".to_string());
        }
        
        // Check parent count constraint
        if batch.parent_count > self.config.max_parents {
            return Err(format!("Too many parents: {} > {}", batch.parent_count, self.config.max_parents));
        }
        
        // Validate parents exist (except for genesis)
        let parents = batch.get_parents();
        if !parents.is_empty() {
            for parent_id in &parents {
                if !self.nodes.contains_key(parent_id) {
                    return Err(format!("Parent batch not found: {:?}", parent_id));
                }
            }
        }
        
        // Create DAG node
        let mut node = DagNode::new(batch);
        
        // Compute height
        if parents.is_empty() {
            // Genesis batch
            node.height = 0;
            if self.genesis_id.is_none() {
                self.genesis_id = Some(batch.id);
            }
        } else {
            // Compute height as max(parent_heights) + 1
            let max_parent_height = parents.iter()
                .map(|pid| self.nodes.get(pid).map(|n| n.height).unwrap_or(0))
                .max()
                .unwrap_or(0);
            node.height = max_parent_height + 1;
            self.max_height = std::cmp::max(self.max_height, node.height);
        }
        
        // Update parent-child relationships
        for parent_id in &parents {
            if let Some(parent_node) = self.nodes.get_mut(parent_id) {
                parent_node.children_ids.push(batch.id);
                // Parent is no longer a tip
                self.tips.remove(parent_id);
            }
        }
        
        // Set node as ready if all parents are committed or finalized
        if parents.is_empty() || parents.iter().all(|pid| {
            self.nodes.get(pid).map(|n| 
                n.status == NodeStatus::Committed || n.status == NodeStatus::Finalized
            ).unwrap_or(false)
        }) {
            node.status = NodeStatus::Ready;
        }
        
        // Add to tips
        self.tips.insert(batch.id);
        
        // Insert node
        self.nodes.insert(batch.id, node);
        
        // Garbage collection if needed
        if self.nodes.len() > self.config.max_batches {
            self.garbage_collect()?;
        }
        
        Ok(())
    }
    
    /// Mark batch as committed via two-link rule
    pub fn commit_batch(&mut self, batch_id: [u8; 32]) -> Result<(), String> {
        if let Some(node) = self.nodes.get_mut(&batch_id) {
            if node.status == NodeStatus::Ready {
                node.status = NodeStatus::Committed;
                self.committed.push_back(batch_id);
                
                // Update children status to ready if all their parents are committed
                let children = node.children_ids.clone();
                for child_id in children {
                    // First, check if all parents are committed without holding mutable reference
                    let should_update = if let Some(child_node) = self.nodes.get(&child_id) {
                        if child_node.status == NodeStatus::Pending {
                            child_node.parent_ids.iter().all(|pid| {
                                self.nodes.get(pid).map(|n| 
                                    n.status == NodeStatus::Committed || n.status == NodeStatus::Finalized
                                ).unwrap_or(false)
                            })
                        } else {
                            false
                        }
                    } else {
                        false
                    };
                    
                    // Now update the child if needed
                    if should_update {
                        if let Some(child_node) = self.nodes.get_mut(&child_id) {
                            child_node.status = NodeStatus::Ready;
                        }
                    }
                }
                
                Ok(())
            } else {
                Err(format!("Batch not ready for commit: {:?}", node.status))
            }
        } else {
            Err("Batch not found in DAG".to_string())
        }
    }
    
    /// Finalize committed batches (can be garbage collected)
    pub fn finalize_batch(&mut self, batch_id: [u8; 32]) -> Result<(), String> {
        if let Some(node) = self.nodes.get_mut(&batch_id) {
            if node.status == NodeStatus::Committed {
                node.status = NodeStatus::Finalized;
                self.finalized.push_back(batch_id);
                Ok(())
            } else {
                Err(format!("Batch not committed: {:?}", node.status))
            }
        } else {
            Err("Batch not found in DAG".to_string())
        }
    }
    
    /// Get ready batches for consensus
    pub fn get_ready_batches(&self) -> Vec<[u8; 32]> {
        self.nodes.iter()
            .filter(|(_, node)| node.status == NodeStatus::Ready)
            .map(|(id, _)| *id)
            .collect()
    }
    
    /// Get DAG tips (batches with no children)
    pub fn get_tips(&self) -> Vec<[u8; 32]> {
        self.tips.iter().cloned().collect()
    }
    
    /// Check if batch extends given parent
    pub fn extends(&self, batch_id: [u8; 32], parent_id: [u8; 32]) -> bool {
        if let Some(node) = self.nodes.get(&batch_id) {
            node.parent_ids.contains(&parent_id)
        } else {
            false
        }
    }
    
    /// Get batch metadata
    pub fn get_batch_metadata(&self, batch_id: [u8; 32]) -> Option<&DagNode> {
        self.nodes.get(&batch_id)
    }
    
    /// Get committed batches in order
    pub fn get_committed_batches(&self) -> Vec<[u8; 32]> {
        self.committed.iter().cloned().collect()
    }
    
    /// Garbage collect old finalized batches
    pub fn garbage_collect(&mut self) -> Result<(), String> {
        let mut gc_count = 0;
        let target_gc = self.nodes.len().saturating_sub(self.config.gc_threshold);
        
        // Remove oldest finalized batches
        while gc_count < target_gc && !self.finalized.is_empty() {
            if let Some(batch_id) = self.finalized.pop_front() {
                if let Some(node) = self.nodes.remove(&batch_id) {
                    // Remove from tips if present
                    self.tips.remove(&batch_id);
                    
                    // Remove from children's parent lists
                    for child_id in &node.children_ids {
                        if let Some(child_node) = self.nodes.get_mut(child_id) {
                            child_node.parent_ids.retain(|pid| *pid != batch_id);
                        }
                    }
                    
                    gc_count += 1;
                }
            }
        }
        
        Ok(())
    }
    
    /// Get memory usage estimate
    pub fn get_memory_usage(&self) -> usize {
        let nodes_mem: usize = self.nodes.values().map(|n| n.size_bytes()).sum();
        let tips_mem = self.tips.len() * 32;
        let queues_mem = (self.committed.len() + self.finalized.len()) * 32;
        
        nodes_mem + tips_mem + queues_mem + 1024 // Base overhead
    }
    
    /// Get DAG statistics
    pub fn get_stats(&self) -> DagStats {
        let pending_count = self.nodes.values().filter(|n| n.status == NodeStatus::Pending).count();
        let ready_count = self.nodes.values().filter(|n| n.status == NodeStatus::Ready).count();
        let committed_count = self.nodes.values().filter(|n| n.status == NodeStatus::Committed).count();
        let finalized_count = self.nodes.values().filter(|n| n.status == NodeStatus::Finalized).count();
        
        DagStats {
            total_nodes: self.nodes.len(),
            pending_nodes: pending_count,
            ready_nodes: ready_count,
            committed_nodes: committed_count,
            finalized_nodes: finalized_count,
            tips_count: self.tips.len(),
            max_height: self.max_height,
            memory_usage_bytes: self.get_memory_usage(),
        }
    }
    
    /// Validate DAG integrity
    pub fn validate_integrity(&self) -> Result<(), String> {
        for (batch_id, node) in &self.nodes {
            // Check parent-child consistency
            for parent_id in &node.parent_ids {
                if let Some(parent_node) = self.nodes.get(parent_id) {
                    if !parent_node.children_ids.contains(batch_id) {
                        return Err(format!("Parent-child inconsistency: parent {:?} doesn't reference child {:?}", parent_id, batch_id));
                    }
                } else {
                    return Err(format!("Parent not found: {:?}", parent_id));
                }
            }
            
            // Check children reference back
            for child_id in &node.children_ids {
                if let Some(child_node) = self.nodes.get(child_id) {
                    if !child_node.parent_ids.contains(batch_id) {
                        return Err(format!("Child-parent inconsistency: child {:?} doesn't reference parent {:?}", child_id, batch_id));
                    }
                } else {
                    return Err(format!("Child not found: {:?}", child_id));
                }
            }
            
            // Check tips consistency
            if node.children_ids.is_empty() && !self.tips.contains(batch_id) {
                return Err(format!("Node with no children not in tips: {:?}", batch_id));
            }
            if !node.children_ids.is_empty() && self.tips.contains(batch_id) {
                return Err(format!("Node with children in tips: {:?}", batch_id));
            }
        }
        
        Ok(())
    }
}

/// DAG Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagStats {
    pub total_nodes: usize,
    pub pending_nodes: usize,
    pub ready_nodes: usize,
    pub committed_nodes: usize,
    pub finalized_nodes: usize,
    pub tips_count: usize,
    pub max_height: u32,
    pub memory_usage_bytes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dag_creation() {
        let config = DagConfig::default();
        let dag = QgcDag::new(config);
        assert_eq!(dag.nodes.len(), 0);
        assert_eq!(dag.tips.len(), 0);
    }
    
    #[test]
    fn test_genesis_batch() {
        let mut dag = QgcDag::new(DagConfig::default());
        let genesis_batch = Batch::new([1u8; 32], [2u8; 32], 1, vec![]);
        
        assert!(dag.add_batch(&genesis_batch).is_ok());
        assert_eq!(dag.nodes.len(), 1);
        assert_eq!(dag.tips.len(), 1);
        assert_eq!(dag.genesis_id, Some(genesis_batch.id));
        
        let node = dag.get_batch_metadata(genesis_batch.id).unwrap();
        assert_eq!(node.height, 0);
        assert_eq!(node.status, NodeStatus::Ready);
    }
    
    #[test]
    fn test_parent_child_relationship() {
        let mut dag = QgcDag::new(DagConfig::default());
        
        // Add genesis
        let genesis = Batch::new([1u8; 32], [2u8; 32], 1, vec![]);
        let genesis_id = genesis.id;
        assert!(dag.add_batch(&genesis).is_ok());
        
        // Add child
        let child = Batch::new([3u8; 32], [4u8; 32], 1, vec![genesis_id]);
        let child_id = child.id;
        assert!(dag.add_batch(&child).is_ok());
        
        // Check relationships
        let genesis_node = dag.get_batch_metadata(genesis_id).unwrap();
        let child_node = dag.get_batch_metadata(child_id).unwrap();
        
        assert!(genesis_node.children_ids.contains(&child_id));
        assert!(child_node.parent_ids.contains(&genesis_id));
        assert_eq!(child_node.height, 1);
        
        // Genesis should no longer be a tip
        assert!(!dag.tips.contains(&genesis_id));
        assert!(dag.tips.contains(&child_id));
    }
    
    #[test]
    fn test_commit_finalize() {
        let mut dag = QgcDag::new(DagConfig::default());
        let batch = Batch::new([1u8; 32], [2u8; 32], 1, vec![]);
        let batch_id = batch.id;
        
        assert!(dag.add_batch(&batch).is_ok());
        assert!(dag.commit_batch(batch_id).is_ok());
        assert!(dag.finalize_batch(batch_id).is_ok());
        
        let node = dag.get_batch_metadata(batch_id).unwrap();
        assert_eq!(node.status, NodeStatus::Finalized);
        assert_eq!(dag.committed.len(), 1);
        assert_eq!(dag.finalized.len(), 1);
    }
    
    #[test]
    fn test_dag_integrity() {
        let mut dag = QgcDag::new(DagConfig::default());
        let genesis = Batch::new([1u8; 32], [2u8; 32], 1, vec![]);
        let genesis_id = genesis.id;
        
        assert!(dag.add_batch(&genesis).is_ok());
        assert!(dag.validate_integrity().is_ok());
        
        let child = Batch::new([3u8; 32], [4u8; 32], 1, vec![genesis_id]);
        assert!(dag.add_batch(&child).is_ok());
        assert!(dag.validate_integrity().is_ok());
    }
    
    #[test]
    fn test_memory_usage() {
        let mut dag = QgcDag::new(DagConfig::default());
        let initial_usage = dag.get_memory_usage();
        
        let batch = Batch::new([1u8; 32], [2u8; 32], 1, vec![]);
        assert!(dag.add_batch(&batch).is_ok());
        
        let after_usage = dag.get_memory_usage();
        assert!(after_usage > initial_usage);
    }
}
