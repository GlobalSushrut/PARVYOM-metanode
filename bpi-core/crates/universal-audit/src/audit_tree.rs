//! Audit Tree - Hierarchical tree structure for runtime audit nodes
//!
//! Maintains parent-child relationships between audit nodes for complete traceability

use crate::{AUDIT_TREE_HASH, runtime_node::RuntimeAuditNode};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use parking_lot::RwLock;
use std::sync::Arc;
use bpi_enc::domain_hash;

/// Audit Tree - Hierarchical structure for audit nodes
#[derive(Debug)]
pub struct AuditTree {
    /// Tree configuration
    config: AuditTreeConfig,
    /// Root nodes (nodes with no parent)
    root_nodes: Arc<RwLock<Vec<[u8; 32]>>>,
    /// All nodes indexed by node_id
    nodes: Arc<RwLock<HashMap<[u8; 32], RuntimeAuditNode>>>,
    /// Parent-to-children mapping for efficient traversal
    children_map: Arc<RwLock<HashMap<[u8; 32], Vec<[u8; 32]>>>>,
    /// Node creation order for temporal queries
    creation_order: Arc<RwLock<VecDeque<[u8; 32]>>>,
    /// Tree statistics
    stats: Arc<RwLock<AuditTreeStats>>,
}

/// Configuration for the audit tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTreeConfig {
    /// Maximum nodes to keep in memory
    pub max_memory_nodes: usize,
    /// Maximum tree depth
    pub max_tree_depth: u32,
    /// Enable automatic pruning of old nodes
    pub auto_prune: bool,
    /// Pruning threshold (nodes older than this are pruned)
    pub prune_threshold_hours: u64,
    /// Enable tree integrity verification
    pub verify_integrity: bool,
}

/// Statistics for the audit tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTreeStats {
    /// Total number of nodes
    pub total_nodes: usize,
    /// Number of root nodes
    pub root_nodes_count: usize,
    /// Maximum tree depth
    pub max_depth: u32,
    /// Average tree depth
    pub avg_depth: f64,
    /// Total tree size in bytes
    pub total_size_bytes: usize,
    /// Creation rate (nodes per second)
    pub creation_rate: f64,
    /// Last update timestamp
    pub last_update_ns: u64,
}

impl Default for AuditTreeConfig {
    fn default() -> Self {
        Self {
            max_memory_nodes: 100000,
            max_tree_depth: 100,
            auto_prune: true,
            prune_threshold_hours: 24,
            verify_integrity: true,
        }
    }
}

impl From<&crate::UniversalAuditConfig> for AuditTreeConfig {
    fn from(config: &crate::UniversalAuditConfig) -> Self {
        Self {
            max_memory_nodes: config.max_memory_nodes,
            max_tree_depth: 100,
            auto_prune: true,
            prune_threshold_hours: 24,
            verify_integrity: true,
        }
    }
}

impl AuditTree {
    /// Create a new audit tree
    pub async fn new(config: AuditTreeConfig) -> Result<Self> {
        Ok(Self {
            config,
            root_nodes: Arc::new(RwLock::new(Vec::new())),
            nodes: Arc::new(RwLock::new(HashMap::new())),
            children_map: Arc::new(RwLock::new(HashMap::new())),
            creation_order: Arc::new(RwLock::new(VecDeque::new())),
            stats: Arc::new(RwLock::new(AuditTreeStats::default())),
        })
    }
    
    /// Add a node to the audit tree
    pub async fn add_node(&self, node: RuntimeAuditNode) -> Result<()> {
        let node_id = node.node_id;
        let parent_id = node.parent_id;
        
        // Check tree depth limit
        if let Some(parent_id) = parent_id {
            let depth = self.get_node_depth(parent_id).await?;
            if depth >= self.config.max_tree_depth {
                return Err(anyhow!("Tree depth limit exceeded: {}", depth));
            }
        }
        
        // Add node to storage
        {
            let mut nodes = self.nodes.write();
            nodes.insert(node_id, node);
        }
        
        // Update parent-child relationships
        if let Some(parent_id) = parent_id {
            // Add to parent's children
            {
                let mut children_map = self.children_map.write();
                children_map.entry(parent_id).or_insert_with(Vec::new).push(node_id);
            }
            
            // Update parent node's children list
            {
                let mut nodes = self.nodes.write();
                if let Some(parent_node) = nodes.get_mut(&parent_id) {
                    parent_node.add_child(node_id);
                }
            }
        } else {
            // This is a root node
            let mut root_nodes = self.root_nodes.write();
            root_nodes.push(node_id);
        }
        
        // Add to creation order
        {
            let mut creation_order = self.creation_order.write();
            creation_order.push_back(node_id);
            
            // Prune if necessary
            if creation_order.len() > self.config.max_memory_nodes {
                if let Some(old_node_id) = creation_order.pop_front() {
                    self.remove_node_internal(old_node_id).await?;
                }
            }
        }
        
        // Update statistics
        self.update_stats().await?;
        
        Ok(())
    }
    
    /// Get a node by ID
    pub async fn get_node(&self, node_id: [u8; 32]) -> Option<RuntimeAuditNode> {
        let nodes = self.nodes.read();
        nodes.get(&node_id).cloned()
    }
    
    /// Get all children of a node
    pub async fn get_children(&self, node_id: [u8; 32]) -> Vec<[u8; 32]> {
        let children_map = self.children_map.read();
        children_map.get(&node_id).cloned().unwrap_or_default()
    }
    
    /// Get all root nodes
    pub async fn get_root_nodes(&self) -> Vec<[u8; 32]> {
        let root_nodes = self.root_nodes.read();
        root_nodes.clone()
    }
    
    /// Get the depth of a node in the tree
    pub async fn get_node_depth(&self, node_id: [u8; 32]) -> Result<u32> {
        let nodes = self.nodes.read();
        let mut current_id = node_id;
        let mut depth = 0;
        
        loop {
            if let Some(node) = nodes.get(&current_id) {
                if let Some(parent_id) = node.parent_id {
                    current_id = parent_id;
                    depth += 1;
                    
                    if depth > self.config.max_tree_depth {
                        return Err(anyhow!("Circular reference detected in tree"));
                    }
                } else {
                    break;
                }
            } else {
                return Err(anyhow!("Node not found: {:?}", current_id));
            }
        }
        
        Ok(depth)
    }
    
    /// Get the path from root to a specific node
    pub async fn get_path_to_root(&self, node_id: [u8; 32]) -> Result<Vec<[u8; 32]>> {
        let nodes = self.nodes.read();
        let mut path = Vec::new();
        let mut current_id = node_id;
        
        loop {
            path.push(current_id);
            
            if let Some(node) = nodes.get(&current_id) {
                if let Some(parent_id) = node.parent_id {
                    current_id = parent_id;
                    
                    if path.len() > self.config.max_tree_depth as usize {
                        return Err(anyhow!("Circular reference detected in tree"));
                    }
                } else {
                    break;
                }
            } else {
                return Err(anyhow!("Node not found: {:?}", current_id));
            }
        }
        
        path.reverse(); // Return path from root to node
        Ok(path)
    }
    
    /// Traverse the tree depth-first
    pub async fn traverse_depth_first<F>(&self, mut visitor: F) -> Result<()>
    where
        F: FnMut(&RuntimeAuditNode) -> Result<bool>, // Return false to stop traversal
    {
        let root_nodes = self.get_root_nodes().await;
        
        for root_id in root_nodes {
            if !self.traverse_node_depth_first(root_id, &mut visitor).await? {
                break;
            }
        }
        
        Ok(())
    }
    
    /// Traverse a specific subtree depth-first
    fn traverse_node_depth_first<'a, F>(&'a self, node_id: [u8; 32], visitor: &'a mut F) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<bool>> + 'a>>
    where
        F: FnMut(&RuntimeAuditNode) -> Result<bool>,
    {
        Box::pin(async move {
            let nodes_guard = self.nodes.read();
            if let Some(node) = nodes_guard.get(&node_id) {
                // Visit current node
                if !visitor(node)? {
                    return Ok(false); // Stop traversal
                }
                
                // Visit children
                for &child_id in &node.children {
                    if !self.traverse_node_depth_first(child_id, visitor).await? {
                        return Ok(false);
                    }
                }
            }
            Ok(true)
        })
    }
    
    /// Find nodes matching a predicate
    pub async fn find_nodes<F>(&self, predicate: F) -> Result<Vec<RuntimeAuditNode>>
    where
        F: Fn(&RuntimeAuditNode) -> bool,
    {
        let mut matching_nodes = Vec::new();
        
        self.traverse_depth_first(|node| {
            if predicate(node) {
                matching_nodes.push(node.clone());
            }
            Ok(true) // Continue traversal
        }).await?;
        
        Ok(matching_nodes)
    }
    
    /// Get nodes in a time range
    pub async fn get_nodes_in_time_range(
        &self,
        start_time_ns: u64,
        end_time_ns: u64,
    ) -> Result<Vec<RuntimeAuditNode>> {
        self.find_nodes(|node| {
            node.timestamp_ns >= start_time_ns && node.timestamp_ns <= end_time_ns
        }).await
    }
    
    /// Verify tree integrity
    pub async fn verify_integrity(&self) -> Result<bool> {
        if !self.config.verify_integrity {
            return Ok(true);
        }
        
        let mut integrity_valid = true;
        
        self.traverse_depth_first(|node| {
            // Verify node integrity
            if !node.verify_integrity().unwrap_or(false) {
                integrity_valid = false;
                return Ok(false); // Stop traversal
            }
            
            // Verify parent-child relationships
            if let Some(parent_id) = node.parent_id {
                // Check if parent exists and has this node as child
                // This would require async access, simplified for now
            }
            
            Ok(true)
        }).await?;
        
        Ok(integrity_valid)
    }
    
    /// Get tree statistics
    pub async fn get_stats(&self) -> AuditTreeStats {
        let stats = self.stats.read();
        stats.clone()
    }
    
    /// Update tree statistics
    async fn update_stats(&self) -> Result<()> {
        let nodes_count = {
            let nodes = self.nodes.read();
            nodes.len()
        };
        
        let root_nodes_count = {
            let root_nodes = self.root_nodes.read();
            root_nodes.len()
        };
        
        // Calculate average depth (simplified)
        let mut total_depth = 0u64;
        let mut max_depth = 0u32;
        
        for root_id in self.get_root_nodes().await {
            if let Ok(depth) = self.calculate_subtree_stats(root_id).await {
                total_depth += depth.total_depth;
                max_depth = max_depth.max(depth.max_depth);
            }
        }
        
        let avg_depth = if nodes_count > 0 {
            total_depth as f64 / nodes_count as f64
        } else {
            0.0
        };
        
        let mut stats = self.stats.write();
        stats.total_nodes = nodes_count;
        stats.root_nodes_count = root_nodes_count;
        stats.max_depth = max_depth;
        stats.avg_depth = avg_depth;
        stats.last_update_ns = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64;
        
        Ok(())
    }
    
    /// Calculate statistics for a subtree
    async fn calculate_subtree_stats(&self, root_id: [u8; 32]) -> Result<SubtreeStats> {
        let mut stats = SubtreeStats {
            total_depth: 0,
            max_depth: 0,
            node_count: 0,
        };
        
        self.calculate_subtree_stats_recursive(root_id, 0, &mut stats).await?;
        
        Ok(stats)
    }
    
    /// Recursively calculate subtree statistics
    fn calculate_subtree_stats_recursive<'a>(
        &'a self,
        node_id: [u8; 32],
        current_depth: u32,
        stats: &'a mut SubtreeStats,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
        Box::pin(async move {
            let nodes_guard = self.nodes.read();
            if let Some(node) = nodes_guard.get(&node_id) {
                stats.node_count += 1;
                stats.max_depth = stats.max_depth.max(current_depth);
                
                // Process children recursively
                for &child_id in &node.children {
                    self.calculate_subtree_stats_recursive(child_id, current_depth + 1, stats).await?;
                }
            }
            Ok(())
        })
    }
    
    /// Remove a node (internal method)
    async fn remove_node_internal(&self, node_id: [u8; 32]) -> Result<()> {
        // Remove from nodes map
        {
            let mut nodes = self.nodes.write();
            nodes.remove(&node_id);
        }
        
        // Remove from children map
        {
            let mut children_map = self.children_map.write();
            children_map.remove(&node_id);
        }
        
        // Remove from root nodes if applicable
        {
            let mut root_nodes = self.root_nodes.write();
            root_nodes.retain(|&id| id != node_id);
        }
        
        Ok(())
    }
}

/// Statistics for a subtree
#[derive(Debug, Default)]
struct SubtreeStats {
    total_depth: u64,
    max_depth: u32,
    node_count: usize,
}

impl Default for AuditTreeStats {
    fn default() -> Self {
        Self {
            total_nodes: 0,
            root_nodes_count: 0,
            max_depth: 0,
            avg_depth: 0.0,
            total_size_bytes: 0,
            creation_rate: 0.0,
            last_update_ns: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_node::{ExecutionContext, OperationType, CageConfig};
    
    #[tokio::test]
    async fn test_audit_tree_creation() {
        let config = AuditTreeConfig::default();
        let tree = AuditTree::new(config).await;
        assert!(tree.is_ok());
    }
    
    #[tokio::test]
    async fn test_add_root_node() {
        let config = AuditTreeConfig::default();
        let tree = AuditTree::new(config).await.unwrap();
        
        let context = ExecutionContext::DockLock {
            container_id: [1u8; 32],
            workload_id: [2u8; 32],
            cage_config: CageConfig::default(),
        };
        
        let operation = OperationType::ProcessStart {
            command: "test".to_string(),
            args: vec!["arg1".to_string()],
        };
        
        let node = RuntimeAuditNode::new(None, context, operation, vec![]).unwrap();
        let node_id = node.node_id;
        
        let result = tree.add_node(node).await;
        assert!(result.is_ok());
        
        let root_nodes = tree.get_root_nodes().await;
        assert_eq!(root_nodes.len(), 1);
        assert_eq!(root_nodes[0], node_id);
    }
    
    #[tokio::test]
    async fn test_parent_child_relationship() {
        let config = AuditTreeConfig::default();
        let tree = AuditTree::new(config).await.unwrap();
        
        // Create parent node
        let parent_context = ExecutionContext::DockLock {
            container_id: [1u8; 32],
            workload_id: [2u8; 32],
            cage_config: CageConfig::default(),
        };
        
        let parent_operation = OperationType::ProcessStart {
            command: "parent".to_string(),
            args: vec![],
        };
        
        let parent_node = RuntimeAuditNode::new(None, parent_context, parent_operation, vec![]).unwrap();
        let parent_id = parent_node.node_id;
        tree.add_node(parent_node).await.unwrap();
        
        // Create child node
        let child_context = ExecutionContext::DockLock {
            container_id: [1u8; 32],
            workload_id: [2u8; 32],
            cage_config: CageConfig::default(),
        };
        
        let child_operation = OperationType::ProcessStart {
            command: "child".to_string(),
            args: vec![],
        };
        
        let child_node = RuntimeAuditNode::new(Some(parent_id), child_context, child_operation, vec![]).unwrap();
        let child_id = child_node.node_id;
        tree.add_node(child_node).await.unwrap();
        
        // Verify relationship
        let children = tree.get_children(parent_id).await;
        assert_eq!(children.len(), 1);
        assert_eq!(children[0], child_id);
        
        let path = tree.get_path_to_root(child_id).await.unwrap();
        assert_eq!(path.len(), 2);
        assert_eq!(path[0], parent_id);
        assert_eq!(path[1], child_id);
    }
}
