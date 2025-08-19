//! Zip Graph - Military-Grade Data Distribution
//! 
//! Zip graph compliant data distribution for military-grade blockchain storage

use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};

/// Zip graph node for military-grade distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZipNode {
    pub id: String,
    pub data_hash: u64,
    pub connections: HashSet<String>,
    pub layer_affinity: u8, // Preferred storage layer (1-4)
    pub access_frequency: u64,
    pub military_classification: String,
}

/// Zip graph edge for military-grade relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZipEdge {
    pub from: String,
    pub to: String,
    pub weight: f64,
    pub relationship_type: String,
    pub security_level: u8,
}

/// Military-grade zip graph distribution engine
pub struct ZipGraph {
    nodes: Arc<RwLock<HashMap<String, ZipNode>>>,
    edges: Arc<RwLock<Vec<ZipEdge>>>,
    distribution_strategy: DistributionStrategy,
}

/// Distribution strategies for military-grade data placement
#[derive(Debug, Clone)]
pub enum DistributionStrategy {
    /// Frequency-based distribution (hot data to faster layers)
    FrequencyBased,
    /// Security-based distribution (classified data to secure layers)
    SecurityBased,
    /// Size-based distribution (large data to appropriate layers)
    SizeBased,
    /// Military-grade hybrid strategy
    MilitaryHybrid,
}

impl ZipGraph {
    /// Create new zip graph distribution engine
    pub async fn new() -> Result<Self> {
        info!("🔗 Initializing Zip Graph Distribution Engine");
        
        Ok(Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            edges: Arc::new(RwLock::new(Vec::new())),
            distribution_strategy: DistributionStrategy::MilitaryHybrid,
        })
    }
    
    /// Add node to zip graph with military-grade classification
    pub async fn add_node(&self, key: &str, data: &[u8], classification: &str) -> Result<()> {
        let data_hash = self.calculate_hash(data);
        
        let node = ZipNode {
            id: key.to_string(),
            data_hash,
            connections: HashSet::new(),
            layer_affinity: self.calculate_layer_affinity(key, data, classification).await,
            access_frequency: 0,
            military_classification: classification.to_string(),
        };
        
        let mut nodes = self.nodes.write().await;
        nodes.insert(key.to_string(), node);
        
        debug!("Added zip graph node: {} (layer affinity: {})", key, nodes[key].layer_affinity);
        Ok(())
    }
    
    /// Create edge between nodes for military-grade relationships
    pub async fn add_edge(&self, from: &str, to: &str, relationship: &str, security_level: u8) -> Result<()> {
        let weight = self.calculate_edge_weight(from, to, relationship).await;
        
        let edge = ZipEdge {
            from: from.to_string(),
            to: to.to_string(),
            weight,
            relationship_type: relationship.to_string(),
            security_level,
        };
        
        // Add bidirectional connections
        {
            let mut nodes = self.nodes.write().await;
            if let Some(from_node) = nodes.get_mut(from) {
                from_node.connections.insert(to.to_string());
            }
            if let Some(to_node) = nodes.get_mut(to) {
                to_node.connections.insert(from.to_string());
            }
        }
        
        let mut edges = self.edges.write().await;
        edges.push(edge);
        
        debug!("Added zip graph edge: {} -> {} ({})", from, to, relationship);
        Ok(())
    }
    
    /// Get optimal storage layer for key using zip graph analysis
    pub async fn get_optimal_layer(&self, key: &str) -> Result<u8> {
        let nodes = self.nodes.read().await;
        
        match nodes.get(key) {
            Some(node) => {
                // Military-grade layer optimization
                let base_affinity = node.layer_affinity;
                let frequency_factor = (node.access_frequency as f64).log10().max(0.0) / 10.0;
                let security_factor = match node.military_classification.as_str() {
                    "TOP_SECRET" => 0.5, // Prefer secure layers
                    "SECRET" => 0.3,
                    "CONFIDENTIAL" => 0.1,
                    _ => 0.0,
                };
                
                let optimized_layer = (base_affinity as f64 - frequency_factor + security_factor)
                    .round()
                    .max(1.0)
                    .min(4.0) as u8;
                
                Ok(optimized_layer)
            },
            None => Ok(4), // Default to Layer 4 for unknown keys
        }
    }
    
    /// Update access frequency for military-grade optimization
    pub async fn record_access(&self, key: &str) -> Result<()> {
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.get_mut(key) {
            node.access_frequency += 1;
            
            // Recalculate layer affinity based on new frequency
            node.layer_affinity = match node.access_frequency {
                0..=10 => 4,      // Cold data -> Layer 4
                11..=100 => 3,    // Warm data -> Layer 3
                101..=1000 => 2,  // Hot data -> Layer 2
                _ => 1,           // Very hot data -> Layer 1
            };
        }
        Ok(())
    }
    
    /// Get related keys using zip graph traversal
    pub async fn get_related_keys(&self, key: &str, max_depth: usize) -> Result<Vec<String>> {
        let nodes = self.nodes.read().await;
        let mut related = HashSet::new();
        let mut to_visit = vec![(key.to_string(), 0)];
        let mut visited = HashSet::new();
        
        while let Some((current_key, depth)) = to_visit.pop() {
            if depth >= max_depth || visited.contains(&current_key) {
                continue;
            }
            
            visited.insert(current_key.clone());
            
            if let Some(node) = nodes.get(&current_key) {
                for connected_key in &node.connections {
                    if !visited.contains(connected_key) {
                        related.insert(connected_key.clone());
                        to_visit.push((connected_key.clone(), depth + 1));
                    }
                }
            }
        }
        
        Ok(related.into_iter().collect())
    }
    
    /// Health check for zip graph system
    pub async fn health_check(&self) -> Result<()> {
        // Simple health check - verify we can access internal structures
        let _nodes = self.nodes.read().await;
        let _edges = self.edges.read().await;
        Ok(())
    }
    
    /// Get zip graph statistics for military-grade monitoring
    pub async fn get_stats(&self) -> Result<HashMap<String, u64>> {
        let mut stats = HashMap::new();
        
        let nodes = self.nodes.read().await;
        let edges = self.edges.read().await;
        
        stats.insert("total_nodes".to_string(), nodes.len() as u64);
        stats.insert("total_edges".to_string(), edges.len() as u64);
        
        // Layer distribution
        let mut layer_counts = [0u64; 4];
        for node in nodes.values() {
            if node.layer_affinity >= 1 && node.layer_affinity <= 4 {
                layer_counts[(node.layer_affinity - 1) as usize] += 1;
            }
        }
        
        for (i, count) in layer_counts.iter().enumerate() {
            stats.insert(format!("layer_{}_nodes", i + 1), *count);
        }
        
        // Security classification distribution
        let mut classification_counts = HashMap::new();
        for node in nodes.values() {
            *classification_counts.entry(node.military_classification.clone()).or_insert(0) += 1;
        }
        
        for (classification, count) in classification_counts {
            stats.insert(format!("classification_{}", classification.to_lowercase()), count);
        }
        
        Ok(stats)
    }
    
    // Private helper methods
    
    fn calculate_hash(&self, data: &[u8]) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish()
    }
    
    async fn calculate_layer_affinity(&self, key: &str, data: &[u8], classification: &str) -> u8 {
        match &self.distribution_strategy {
            DistributionStrategy::FrequencyBased => {
                // Default to Layer 4, will be optimized based on access patterns
                4
            },
            DistributionStrategy::SecurityBased => {
                match classification {
                    "TOP_SECRET" => 4, // Most secure layer
                    "SECRET" => 3,
                    "CONFIDENTIAL" => 2,
                    _ => 1,
                }
            },
            DistributionStrategy::SizeBased => {
                match data.len() {
                    0..=1024 => 1,           // Small data -> Fast layer
                    1025..=1048576 => 2,     // Medium data -> Layer 2
                    1048577..=104857600 => 3, // Large data -> Layer 3
                    _ => 4,                  // Very large data -> Layer 4
                }
            },
            DistributionStrategy::MilitaryHybrid => {
                // Combine all strategies for military-grade optimization
                let security_layer = match classification {
                    "TOP_SECRET" => 4,
                    "SECRET" => 3,
                    "CONFIDENTIAL" => 2,
                    _ => 1,
                };
                
                let size_layer = match data.len() {
                    0..=1024 => 1,
                    1025..=1048576 => 2,
                    1048577..=104857600 => 3,
                    _ => 4,
                };
                
                // Weighted average favoring security
                ((security_layer as f64 * 0.6) + (size_layer as f64 * 0.4)).round() as u8
            },
        }
    }
    
    async fn calculate_edge_weight(&self, from: &str, to: &str, relationship: &str) -> f64 {
        match relationship {
            "DEPENDS_ON" => 0.9,
            "REFERENCES" => 0.7,
            "SIMILAR_TO" => 0.5,
            "RELATED_TO" => 0.3,
            _ => 0.1,
        }
    }
}
