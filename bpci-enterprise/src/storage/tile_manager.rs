//! Tile Manager for 4D Spatial-Temporal Organization
//! 
//! Manages 4D tiles for efficient spatial-temporal data organization

use std::collections::{HashMap, BTreeMap};
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::{Result, anyhow};
use uuid::Uuid;

use super::{FourDCoordinate, FourDBoundingBox, FourDTile, HashGraphNode, TileMetadata, SecurityLevel, FourDConfig};

/// Tile Manager for 4D spatial organization
#[derive(Debug)]
pub struct TileManager {
    tiles: Arc<RwLock<HashMap<Uuid, FourDTile>>>,
    spatial_index: Arc<RwLock<SpatialIndex>>,
    config: FourDConfig,
    stats: Arc<RwLock<TileStats>>,
}

/// Spatial index for fast tile lookup
#[derive(Debug)]
pub struct SpatialIndex {
    r_index: BTreeMap<u64, Vec<Uuid>>,
    c_index: BTreeMap<u64, Vec<Uuid>>,
    v_index: BTreeMap<OrderedFloat, Vec<Uuid>>,
    i_index: BTreeMap<u64, Vec<Uuid>>,
}

/// Wrapper for f64 to make it orderable in BTreeMap
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct OrderedFloat(f64);

impl Eq for OrderedFloat {}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(std::cmp::Ordering::Equal)
    }
}

/// Tile statistics
#[derive(Debug, Clone, Default)]
pub struct TileStats {
    pub total_tiles: usize,
    pub total_nodes: usize,
    pub hot_tiles: usize,
    pub cold_tiles: usize,
    pub average_tile_size: f64,
    pub compression_ratio: f64,
}

impl TileManager {
    /// Create new tile manager
    pub async fn new(config: FourDConfig) -> Result<Self> {
        Ok(Self {
            tiles: Arc::new(RwLock::new(HashMap::new())),
            spatial_index: Arc::new(RwLock::new(SpatialIndex::new())),
            config,
            stats: Arc::new(RwLock::new(TileStats::default())),
        })
    }
    
    /// Find or create tile for coordinate
    pub async fn find_or_create_tile_for_coordinate(&self, coord: &FourDCoordinate) -> Result<Uuid> {
        // First try to find existing tile
        if let Some(tile_id) = self.find_tile_containing_coordinate(coord).await? {
            return Ok(tile_id);
        }
        
        // Create new tile
        self.create_tile_for_coordinate(coord).await
    }
    
    /// Find tile containing coordinate
    pub async fn find_tile_containing_coordinate(&self, coord: &FourDCoordinate) -> Result<Option<Uuid>> {
        let tiles = self.tiles.read().await;
        
        for (tile_id, tile) in tiles.iter() {
            if self.coordinate_in_bounding_box(coord, &tile.bounding_box) {
                return Ok(Some(*tile_id));
            }
        }
        
        Ok(None)
    }
    
    /// Create new tile for coordinate
    pub async fn create_tile_for_coordinate(&self, coord: &FourDCoordinate) -> Result<Uuid> {
        let tile_id = Uuid::new_v4();
        
        // Calculate bounding box around coordinate
        let bounding_box = self.calculate_tile_bounding_box(coord);
        
        let tile = FourDTile {
            tile_id,
            bounding_box: bounding_box.clone(),
            nodes: Vec::new(),
            edges: Vec::new(),
            compressed_payload: Vec::new(),
            tile_metadata: TileMetadata {
                size_bytes: 0,
                node_count: 0,
                edge_count: 0,
                compression_ratio: 1.0,
                hot_data: true,
                security_level: SecurityLevel::Public,
            },
            access_count: 0,
            last_accessed: chrono::Utc::now().timestamp() as u64,
        };
        
        // Insert tile
        self.tiles.write().await.insert(tile_id, tile);
        
        // Update spatial index
        self.spatial_index.write().await.add_tile(tile_id, &bounding_box);
        
        // Update statistics
        self.stats.write().await.total_tiles += 1;
        
        Ok(tile_id)
    }
    
    /// Insert node into tile
    pub async fn insert_node(&self, tile_id: Uuid, node: HashGraphNode) -> Result<()> {
        let mut tiles = self.tiles.write().await;
        
        if let Some(tile) = tiles.get_mut(&tile_id) {
            tile.nodes.push(node);
            tile.tile_metadata.node_count += 1;
            tile.tile_metadata.size_bytes += std::mem::size_of::<HashGraphNode>();
            tile.access_count += 1;
            tile.last_accessed = chrono::Utc::now().timestamp() as u64;
            
            // Check if tile needs splitting
            if tile.tile_metadata.size_bytes > self.config.max_tile_size {
                drop(tiles); // Release lock before splitting
                self.split_tile(tile_id).await?;
            }
            
            // Update statistics
            self.stats.write().await.total_nodes += 1;
            
            Ok(())
        } else {
            Err(anyhow!("Tile not found: {}", tile_id))
        }
    }
    
    /// Get tile by ID
    pub async fn get_tile(&self, tile_id: Uuid) -> Result<Option<FourDTile>> {
        let mut tiles = self.tiles.write().await;
        
        if let Some(tile) = tiles.get_mut(&tile_id) {
            // Update access statistics
            tile.access_count += 1;
            tile.last_accessed = chrono::Utc::now().timestamp() as u64;
            
            Ok(Some(tile.clone()))
        } else {
            Ok(None)
        }
    }
    
    /// Find tiles intersecting with bounding box
    pub async fn find_tiles_in_region(&self, region: &FourDBoundingBox) -> Result<Vec<Uuid>> {
        let tiles = self.tiles.read().await;
        let mut result = Vec::new();
        
        for (tile_id, tile) in tiles.iter() {
            if self.bounding_boxes_intersect(&tile.bounding_box, region) {
                result.push(*tile_id);
            }
        }
        
        Ok(result)
    }
    
    /// Split tile when it becomes too large
    async fn split_tile(&self, tile_id: Uuid) -> Result<Vec<Uuid>> {
        let tile = {
            let tiles = self.tiles.read().await;
            tiles.get(&tile_id).cloned()
        };
        
        if let Some(tile) = tile {
            // Calculate split dimensions
            let split_boxes = self.calculate_split_bounding_boxes(&tile.bounding_box);
            let mut new_tile_ids = Vec::new();
            
            // Create new tiles
            for bbox in split_boxes {
                let new_tile_id = Uuid::new_v4();
                let new_tile = FourDTile {
                    tile_id: new_tile_id,
                    bounding_box: bbox.clone(),
                    nodes: Vec::new(),
                    edges: Vec::new(),
                    compressed_payload: Vec::new(),
                    tile_metadata: TileMetadata {
                        size_bytes: 0,
                        node_count: 0,
                        edge_count: 0,
                        compression_ratio: 1.0,
                        hot_data: tile.tile_metadata.hot_data,
                        security_level: tile.tile_metadata.security_level.clone(),
                    },
                    access_count: 0,
                    last_accessed: chrono::Utc::now().timestamp() as u64,
                };
                
                self.tiles.write().await.insert(new_tile_id, new_tile);
                self.spatial_index.write().await.add_tile(new_tile_id, &bbox);
                new_tile_ids.push(new_tile_id);
            }
            
            // Redistribute nodes to new tiles
            for node in &tile.nodes {
                let coord = self.extract_coordinate_from_node(node);
                for new_tile_id in &new_tile_ids {
                    let new_tile = self.tiles.read().await.get(new_tile_id).cloned();
                    if let Some(new_tile) = new_tile {
                        if self.coordinate_in_bounding_box(&coord, &new_tile.bounding_box) {
                            self.tiles.write().await.get_mut(new_tile_id).unwrap().nodes.push(node.clone());
                            break;
                        }
                    }
                }
            }
            
            // Remove original tile
            self.tiles.write().await.remove(&tile_id);
            self.spatial_index.write().await.remove_tile(tile_id);
            
            Ok(new_tile_ids)
        } else {
            Err(anyhow!("Tile not found for splitting: {}", tile_id))
        }
    }
    
    /// Health check for tile manager
    pub async fn health_check(&self) -> Result<bool> {
        let tiles = self.tiles.read().await;
        let spatial_index = self.spatial_index.read().await;
        
        // Check that all tiles in spatial index exist
        for tile_ids in spatial_index.r_index.values() {
            for tile_id in tile_ids {
                if !tiles.contains_key(tile_id) {
                    return Ok(false);
                }
            }
        }
        
        Ok(true)
    }
    
    /// Get tile statistics
    pub async fn get_stats(&self) -> TileStats {
        let tiles = self.tiles.read().await;
        let mut stats = TileStats::default();
        
        stats.total_tiles = tiles.len();
        
        let mut total_size = 0;
        let mut total_compressed_size = 0;
        
        for tile in tiles.values() {
            stats.total_nodes += tile.tile_metadata.node_count;
            total_size += tile.tile_metadata.size_bytes;
            total_compressed_size += tile.compressed_payload.len();
            
            if tile.tile_metadata.hot_data {
                stats.hot_tiles += 1;
            } else {
                stats.cold_tiles += 1;
            }
        }
        
        if stats.total_tiles > 0 {
            stats.average_tile_size = total_size as f64 / stats.total_tiles as f64;
        }
        
        if total_size > 0 {
            stats.compression_ratio = total_compressed_size as f64 / total_size as f64;
        }
        
        stats
    }
    
    // Helper methods
    
    fn coordinate_in_bounding_box(&self, coord: &FourDCoordinate, bbox: &FourDBoundingBox) -> bool {
        coord.r >= bbox.r_min && coord.r <= bbox.r_max &&
        coord.c >= bbox.c_min && coord.c <= bbox.c_max &&
        coord.v >= bbox.v_min && coord.v <= bbox.v_max &&
        coord.i >= bbox.i_min && coord.i <= bbox.i_max
    }
    
    fn bounding_boxes_intersect(&self, bbox1: &FourDBoundingBox, bbox2: &FourDBoundingBox) -> bool {
        bbox1.r_min <= bbox2.r_max && bbox1.r_max >= bbox2.r_min &&
        bbox1.c_min <= bbox2.c_max && bbox1.c_max >= bbox2.c_min &&
        bbox1.v_min <= bbox2.v_max && bbox1.v_max >= bbox2.v_min &&
        bbox1.i_min <= bbox2.i_max && bbox1.i_max >= bbox2.i_min
    }
    
    fn calculate_tile_bounding_box(&self, coord: &FourDCoordinate) -> FourDBoundingBox {
        // Create bounding box around coordinate with fixed size
        let tile_size_r = 1000u64;
        let tile_size_c = 1000u64;
        let tile_size_v = 10.0f64;
        let tile_size_i = 1000u64;
        
        FourDBoundingBox {
            r_min: (coord.r / tile_size_r) * tile_size_r,
            r_max: ((coord.r / tile_size_r) + 1) * tile_size_r,
            c_min: (coord.c / tile_size_c) * tile_size_c,
            c_max: ((coord.c / tile_size_c) + 1) * tile_size_c,
            v_min: (coord.v / tile_size_v).floor() * tile_size_v,
            v_max: ((coord.v / tile_size_v).floor() + 1.0) * tile_size_v,
            i_min: (coord.i / tile_size_i) * tile_size_i,
            i_max: ((coord.i / tile_size_i) + 1) * tile_size_i,
        }
    }
    
    fn calculate_split_bounding_boxes(&self, bbox: &FourDBoundingBox) -> Vec<FourDBoundingBox> {
        // Split into 8 sub-tiles (2x2x2 in R,C,V dimensions, keep I same)
        let r_mid = (bbox.r_min + bbox.r_max) / 2;
        let c_mid = (bbox.c_min + bbox.c_max) / 2;
        let v_mid = (bbox.v_min + bbox.v_max) / 2.0;
        
        vec![
            FourDBoundingBox {
                r_min: bbox.r_min, r_max: r_mid,
                c_min: bbox.c_min, c_max: c_mid,
                v_min: bbox.v_min, v_max: v_mid,
                i_min: bbox.i_min, i_max: bbox.i_max,
            },
            FourDBoundingBox {
                r_min: r_mid, r_max: bbox.r_max,
                c_min: bbox.c_min, c_max: c_mid,
                v_min: bbox.v_min, v_max: v_mid,
                i_min: bbox.i_min, i_max: bbox.i_max,
            },
            FourDBoundingBox {
                r_min: bbox.r_min, r_max: r_mid,
                c_min: c_mid, c_max: bbox.c_max,
                v_min: bbox.v_min, v_max: v_mid,
                i_min: bbox.i_min, i_max: bbox.i_max,
            },
            FourDBoundingBox {
                r_min: r_mid, r_max: bbox.r_max,
                c_min: c_mid, c_max: bbox.c_max,
                v_min: bbox.v_min, v_max: v_mid,
                i_min: bbox.i_min, i_max: bbox.i_max,
            },
            FourDBoundingBox {
                r_min: bbox.r_min, r_max: r_mid,
                c_min: bbox.c_min, c_max: c_mid,
                v_min: v_mid, v_max: bbox.v_max,
                i_min: bbox.i_min, i_max: bbox.i_max,
            },
            FourDBoundingBox {
                r_min: r_mid, r_max: bbox.r_max,
                c_min: bbox.c_min, c_max: c_mid,
                v_min: v_mid, v_max: bbox.v_max,
                i_min: bbox.i_min, i_max: bbox.i_max,
            },
            FourDBoundingBox {
                r_min: bbox.r_min, r_max: r_mid,
                c_min: c_mid, c_max: bbox.c_max,
                v_min: v_mid, v_max: bbox.v_max,
                i_min: bbox.i_min, i_max: bbox.i_max,
            },
            FourDBoundingBox {
                r_min: r_mid, r_max: bbox.r_max,
                c_min: c_mid, c_max: bbox.c_max,
                v_min: v_mid, v_max: bbox.v_max,
                i_min: bbox.i_min, i_max: bbox.i_max,
            },
        ]
    }
    
    fn extract_coordinate_from_node(&self, node: &HashGraphNode) -> FourDCoordinate {
        // Extract coordinate from node hash (simplified)
        let hash_bytes = node.hash_key.as_bytes();
        
        FourDCoordinate {
            r: u64::from_be_bytes([hash_bytes[0], hash_bytes[1], hash_bytes[2], hash_bytes[3], 
                                  hash_bytes[4], hash_bytes[5], hash_bytes[6], hash_bytes[7]]),
            c: u64::from_be_bytes([hash_bytes[8], hash_bytes[9], hash_bytes[10], hash_bytes[11], 
                                  hash_bytes[12], hash_bytes[13], hash_bytes[14], hash_bytes[15]]),
            v: f64::from_be_bytes([hash_bytes[16], hash_bytes[17], hash_bytes[18], hash_bytes[19], 
                                  hash_bytes[20], hash_bytes[21], hash_bytes[22], hash_bytes[23]]),
            i: node.created_at,
        }
    }
}

impl SpatialIndex {
    fn new() -> Self {
        Self {
            r_index: BTreeMap::new(),
            c_index: BTreeMap::new(),
            v_index: BTreeMap::new(),
            i_index: BTreeMap::new(),
        }
    }
    
    fn add_tile(&mut self, tile_id: Uuid, bbox: &FourDBoundingBox) {
        // Add to R index
        self.r_index.entry(bbox.r_min).or_insert_with(Vec::new).push(tile_id);
        
        // Add to C index
        self.c_index.entry(bbox.c_min).or_insert_with(Vec::new).push(tile_id);
        
        // Add to V index
        self.v_index.entry(OrderedFloat(bbox.v_min)).or_insert_with(Vec::new).push(tile_id);
        
        // Add to I index
        self.i_index.entry(bbox.i_min).or_insert_with(Vec::new).push(tile_id);
    }
    
    fn remove_tile(&mut self, tile_id: Uuid) {
        // Remove from all indexes (simplified - would need bbox info for efficient removal)
        for tile_list in self.r_index.values_mut() {
            tile_list.retain(|&id| id != tile_id);
        }
        for tile_list in self.c_index.values_mut() {
            tile_list.retain(|&id| id != tile_id);
        }
        for tile_list in self.v_index.values_mut() {
            tile_list.retain(|&id| id != tile_id);
        }
        for tile_list in self.i_index.values_mut() {
            tile_list.retain(|&id| id != tile_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_tile_manager_creation() {
        let config = FourDConfig::default();
        let manager = TileManager::new(config).await.unwrap();
        
        let stats = manager.get_stats().await;
        assert_eq!(stats.total_tiles, 0);
    }
    
    #[tokio::test]
    async fn test_create_tile_for_coordinate() {
        let config = FourDConfig::default();
        let manager = TileManager::new(config).await.unwrap();
        
        let coord = FourDCoordinate { r: 100, c: 200, v: 1.5, i: 300 };
        let tile_id = manager.find_or_create_tile_for_coordinate(&coord).await.unwrap();
        
        let tile = manager.get_tile(tile_id).await.unwrap().unwrap();
        assert!(manager.coordinate_in_bounding_box(&coord, &tile.bounding_box));
    }
    
    #[tokio::test]
    async fn test_insert_node_into_tile() {
        let config = FourDConfig::default();
        let manager = TileManager::new(config).await.unwrap();
        
        let coord = FourDCoordinate { r: 100, c: 200, v: 1.5, i: 300 };
        let tile_id = manager.find_or_create_tile_for_coordinate(&coord).await.unwrap();
        
        let node = HashGraphNode {
            hash_key: blake3::hash(b"test node"),
            content: b"test content".to_vec(),
            metadata: HashMap::new(),
            vector_shards: vec![1.0, 2.0, 3.0],
            labels: vec!["test".to_string()],
            created_at: chrono::Utc::now().timestamp() as u64,
        };
        
        manager.insert_node(tile_id, node).await.unwrap();
        
        let tile = manager.get_tile(tile_id).await.unwrap().unwrap();
        assert_eq!(tile.nodes.len(), 1);
        assert_eq!(tile.tile_metadata.node_count, 1);
    }
}
