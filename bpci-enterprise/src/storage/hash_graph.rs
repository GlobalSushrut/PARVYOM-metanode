//! Hash-Graph Implementation for 4D Database
//! 
//! Content-addressable hash graph with immutable relations

use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use anyhow::{Result, anyhow};
use blake3::Hash;
use tokio::sync::RwLock;

use super::{HashGraphNode, HashGraphEdge};

/// Hash-Graph structure for content-addressable storage
#[derive(Debug)]
pub struct HashGraph {
    nodes: RwLock<HashMap<Hash, HashGraphNode>>,
    edges: RwLock<HashMap<Hash, HashGraphEdge>>,
    node_edges: RwLock<HashMap<Hash, HashSet<Hash>>>, // node -> set of edge hashes
    reverse_edges: RwLock<HashMap<Hash, HashSet<Hash>>>, // target -> set of edge hashes
}

impl HashGraph {
    /// Create new hash graph
    pub fn new() -> Self {
        Self {
            nodes: RwLock::new(HashMap::new()),
            edges: RwLock::new(HashMap::new()),
            node_edges: RwLock::new(HashMap::new()),
            reverse_edges: RwLock::new(HashMap::new()),
        }
    }
    
    /// Add node to hash graph
    pub async fn add_node(&self, hash_key: Hash) -> Result<()> {
        let mut nodes = self.nodes.write().await;
        
        if nodes.contains_key(&hash_key) {
            return Ok(()); // Node already exists
        }
        
        // Create minimal node entry (full node data stored in tiles)
        let node = HashGraphNode {
            hash_key,
            content: Vec::new(), // Content stored in tiles
            metadata: HashMap::new(),
            vector_shards: Vec::new(),
            labels: Vec::new(),
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        
        nodes.insert(hash_key, node);
        
        // Initialize edge sets
        self.node_edges.write().await.insert(hash_key, HashSet::new());
        self.reverse_edges.write().await.insert(hash_key, HashSet::new());
        
        Ok(())
    }
    
    /// Add edge between nodes
    pub async fn add_edge(
        &self,
        source_hash: Hash,
        target_hash: Hash,
        relation_type: String,
        intent: String,
        weight: f64,
    ) -> Result<Hash> {
        // Ensure both nodes exist
        if !self.nodes.read().await.contains_key(&source_hash) {
            return Err(anyhow!("Source node not found: {:?}", source_hash));
        }
        if !self.nodes.read().await.contains_key(&target_hash) {
            return Err(anyhow!("Target node not found: {:?}", target_hash));
        }
        
        // Create relation key
        let relation_content = format!("{:?}||{}||{:?}||{}||{}", 
            source_hash, relation_type, target_hash, intent, weight);
        let relation_key = blake3::hash(relation_content.as_bytes());
        
        // Create policy hash (simplified for now)
        let policy_hash = blake3::hash(format!("policy_{}", relation_type).as_bytes());
        
        let edge = HashGraphEdge {
            relation_key,
            source_hash,
            target_hash,
            relation_type,
            intent,
            weight,
            policy_hash,
            timestamp: chrono::Utc::now().timestamp() as u64,
        };
        
        // Add edge to graph
        self.edges.write().await.insert(relation_key, edge);
        
        // Update adjacency lists
        self.node_edges.write().await
            .entry(source_hash)
            .or_insert_with(HashSet::new)
            .insert(relation_key);
            
        self.reverse_edges.write().await
            .entry(target_hash)
            .or_insert_with(HashSet::new)
            .insert(relation_key);
        
        Ok(relation_key)
    }
    
    /// Get node by hash
    pub async fn get_node(&self, hash_key: &Hash) -> Option<HashGraphNode> {
        self.nodes.read().await.get(hash_key).cloned()
    }
    
    /// Get edge by relation key
    pub async fn get_edge(&self, relation_key: &Hash) -> Option<HashGraphEdge> {
        self.edges.read().await.get(relation_key).cloned()
    }
    
    /// Get outgoing edges from a node
    pub async fn get_outgoing_edges(&self, node_hash: &Hash) -> Vec<HashGraphEdge> {
        let node_edges = self.node_edges.read().await;
        let edges = self.edges.read().await;
        
        if let Some(edge_hashes) = node_edges.get(node_hash) {
            edge_hashes.iter()
                .filter_map(|edge_hash| edges.get(edge_hash).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get incoming edges to a node
    pub async fn get_incoming_edges(&self, node_hash: &Hash) -> Vec<HashGraphEdge> {
        let reverse_edges = self.reverse_edges.read().await;
        let edges = self.edges.read().await;
        
        if let Some(edge_hashes) = reverse_edges.get(node_hash) {
            edge_hashes.iter()
                .filter_map(|edge_hash| edges.get(edge_hash).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Find nodes by relation type
    pub async fn find_related_nodes(
        &self,
        source_hash: &Hash,
        relation_type: &str,
        max_depth: usize,
    ) -> Vec<Hash> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut queue = vec![(*source_hash, 0)];
        
        while let Some((current_hash, depth)) = queue.pop() {
            if depth >= max_depth || visited.contains(&current_hash) {
                continue;
            }
            
            visited.insert(current_hash);
            
            let outgoing_edges = self.get_outgoing_edges(&current_hash).await;
            
            for edge in outgoing_edges {
                if edge.relation_type == relation_type {
                    result.push(edge.target_hash);
                    
                    if depth + 1 < max_depth {
                        queue.push((edge.target_hash, depth + 1));
                    }
                }
            }
        }
        
        result
    }
    
    /// Verify graph integrity
    pub async fn verify_integrity(&self) -> Result<bool> {
        let nodes = self.nodes.read().await;
        let edges = self.edges.read().await;
        let node_edges = self.node_edges.read().await;
        let reverse_edges = self.reverse_edges.read().await;
        
        // Check that all edges reference existing nodes
        for edge in edges.values() {
            if !nodes.contains_key(&edge.source_hash) {
                return Ok(false);
            }
            if !nodes.contains_key(&edge.target_hash) {
                return Ok(false);
            }
        }
        
        // Check adjacency list consistency
        for (node_hash, edge_set) in node_edges.iter() {
            for edge_hash in edge_set {
                if let Some(edge) = edges.get(edge_hash) {
                    if edge.source_hash != *node_hash {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                }
            }
        }
        
        // Check reverse adjacency list consistency
        for (node_hash, edge_set) in reverse_edges.iter() {
            for edge_hash in edge_set {
                if let Some(edge) = edges.get(edge_hash) {
                    if edge.target_hash != *node_hash {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                }
            }
        }
        
        Ok(true)
    }
    
    /// Get graph statistics
    pub async fn get_stats(&self) -> GraphStats {
        let node_count = self.nodes.read().await.len();
        let edge_count = self.edges.read().await.len();
        
        // Calculate degree distribution
        let node_edges = self.node_edges.read().await;
        let mut degree_sum = 0;
        let mut max_degree = 0;
        
        for edge_set in node_edges.values() {
            let degree = edge_set.len();
            degree_sum += degree;
            max_degree = max_degree.max(degree);
        }
        
        let avg_degree = if node_count > 0 {
            degree_sum as f64 / node_count as f64
        } else {
            0.0
        };
        
        GraphStats {
            node_count,
            edge_count,
            avg_degree,
            max_degree,
        }
    }
    
    /// Perform graph traversal with custom predicate
    pub async fn traverse_with_predicate<F>(
        &self,
        start_hash: Hash,
        predicate: F,
        max_depth: usize,
    ) -> Vec<Hash>
    where
        F: Fn(&HashGraphEdge) -> bool,
    {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut queue = vec![(start_hash, 0)];
        
        while let Some((current_hash, depth)) = queue.pop() {
            if depth >= max_depth || visited.contains(&current_hash) {
                continue;
            }
            
            visited.insert(current_hash);
            result.push(current_hash);
            
            let outgoing_edges = self.get_outgoing_edges(&current_hash).await;
            
            for edge in outgoing_edges {
                if predicate(&edge) && depth + 1 < max_depth {
                    queue.push((edge.target_hash, depth + 1));
                }
            }
        }
        
        result
    }
    
    /// Find shortest path between two nodes
    pub async fn find_shortest_path(
        &self,
        source: Hash,
        target: Hash,
        max_depth: usize,
    ) -> Option<Vec<Hash>> {
        use std::collections::VecDeque;
        
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent: HashMap<Hash, Hash> = HashMap::new();
        
        queue.push_back((source, 0));
        visited.insert(source);
        
        while let Some((current_hash, depth)) = queue.pop_front() {
            if current_hash == target {
                // Reconstruct path
                let mut path = Vec::new();
                let mut current = target;
                
                while current != source {
                    path.push(current);
                    current = parent[&current];
                }
                path.push(source);
                path.reverse();
                
                return Some(path);
            }
            
            if depth >= max_depth {
                continue;
            }
            
            let outgoing_edges = self.get_outgoing_edges(&current_hash).await;
            
            for edge in outgoing_edges {
                if !visited.contains(&edge.target_hash) {
                    visited.insert(edge.target_hash);
                    parent.insert(edge.target_hash, current_hash);
                    queue.push_back((edge.target_hash, depth + 1));
                }
            }
        }
        
        None
    }
}

/// Graph statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphStats {
    pub node_count: usize,
    pub edge_count: usize,
    pub avg_degree: f64,
    pub max_degree: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_hash_graph_creation() {
        let graph = HashGraph::new();
        let stats = graph.get_stats().await;
        
        assert_eq!(stats.node_count, 0);
        assert_eq!(stats.edge_count, 0);
    }
    
    #[tokio::test]
    async fn test_add_nodes_and_edges() {
        let graph = HashGraph::new();
        
        let hash1 = blake3::hash(b"node1");
        let hash2 = blake3::hash(b"node2");
        
        graph.add_node(hash1).await.unwrap();
        graph.add_node(hash2).await.unwrap();
        
        let edge_hash = graph.add_edge(
            hash1,
            hash2,
            "connects".to_string(),
            "test".to_string(),
            1.0,
        ).await.unwrap();
        
        let stats = graph.get_stats().await;
        assert_eq!(stats.node_count, 2);
        assert_eq!(stats.edge_count, 1);
        
        let outgoing = graph.get_outgoing_edges(&hash1).await;
        assert_eq!(outgoing.len(), 1);
        assert_eq!(outgoing[0].relation_key, edge_hash);
    }
    
    #[tokio::test]
    async fn test_graph_integrity() {
        let graph = HashGraph::new();
        
        let hash1 = blake3::hash(b"node1");
        let hash2 = blake3::hash(b"node2");
        
        graph.add_node(hash1).await.unwrap();
        graph.add_node(hash2).await.unwrap();
        graph.add_edge(hash1, hash2, "test".to_string(), "intent".to_string(), 1.0).await.unwrap();
        
        let integrity = graph.verify_integrity().await.unwrap();
        assert!(integrity);
    }
    
    #[tokio::test]
    async fn test_find_related_nodes() {
        let graph = HashGraph::new();
        
        let hash1 = blake3::hash(b"node1");
        let hash2 = blake3::hash(b"node2");
        let hash3 = blake3::hash(b"node3");
        
        graph.add_node(hash1).await.unwrap();
        graph.add_node(hash2).await.unwrap();
        graph.add_node(hash3).await.unwrap();
        
        graph.add_edge(hash1, hash2, "connects".to_string(), "test".to_string(), 1.0).await.unwrap();
        graph.add_edge(hash2, hash3, "connects".to_string(), "test".to_string(), 1.0).await.unwrap();
        
        let related = graph.find_related_nodes(&hash1, "connects", 2).await;
        assert_eq!(related.len(), 2);
        assert!(related.contains(&hash2));
        assert!(related.contains(&hash3));
    }
}
