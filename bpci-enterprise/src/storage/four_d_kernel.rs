//! 4D Hash-Graph Database Kernel Core Implementation
//! 
//! Core 4D mathematical operations and kernel functionality

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use uuid::Uuid;
use blake3::Hash;

use super::{FourDCoordinate, FourDBoundingBox, HashGraphNode, SecurityLevel};

/// 4D Algebraic Operations
pub struct FourDAlgebra;

impl FourDAlgebra {
    /// 4D-Select: σ_{R∩,C∩,V∩,I∩}(Tiles)
    pub fn select_4d(
        tiles: &[FourDTileRef],
        r_range: (u64, u64),
        c_range: (u64, u64),
        v_range: (f64, f64),
        i_range: (u64, u64),
    ) -> Vec<FourDTileRef> {
        tiles.iter()
            .filter(|tile| {
                tile.bounding_box.r_min <= r_range.1 && tile.bounding_box.r_max >= r_range.0 &&
                tile.bounding_box.c_min <= c_range.1 && tile.bounding_box.c_max >= c_range.0 &&
                tile.bounding_box.v_min <= v_range.1 && tile.bounding_box.v_max >= v_range.0 &&
                tile.bounding_box.i_min <= i_range.1 && tile.bounding_box.i_max >= i_range.0
            })
            .cloned()
            .collect()
    }
    
    /// 4D-Project: π_{C-slice}(Tile)
    pub fn project_4d(
        nodes: &[HashGraphNode],
        c_slice: (u64, u64),
    ) -> Vec<ProjectedNode> {
        nodes.iter()
            .map(|node| ProjectedNode {
                hash_key: node.hash_key,
                projected_data: Self::extract_c_slice(&node.content, c_slice),
                metadata: node.metadata.clone(),
            })
            .collect()
    }
    
    /// 4D-Join: Spatial join operations
    pub fn spatial_join_4d(
        left_tiles: &[FourDTileRef],
        right_tiles: &[FourDTileRef],
        join_predicate: JoinPredicate,
    ) -> Vec<JoinResult> {
        let mut results = Vec::new();
        
        for left_tile in left_tiles {
            for right_tile in right_tiles {
                if Self::tiles_intersect(&left_tile.bounding_box, &right_tile.bounding_box) {
                    if join_predicate.evaluate(left_tile, right_tile) {
                        results.push(JoinResult {
                            left_tile_id: left_tile.tile_id,
                            right_tile_id: right_tile.tile_id,
                            intersection_box: Self::compute_intersection(
                                &left_tile.bounding_box,
                                &right_tile.bounding_box,
                            ),
                        });
                    }
                }
            }
        }
        
        results
    }
    
    /// 4D-Reduce: Compressed vectorized aggregations
    pub fn reduce_4d(
        tiles: &[FourDTileRef],
        aggregation_type: AggregationType,
    ) -> AggregationResult {
        match aggregation_type {
            AggregationType::Count => AggregationResult::Count(
                tiles.iter().map(|t| t.node_count).sum()
            ),
            AggregationType::Sum => AggregationResult::Sum(
                tiles.iter().map(|t| t.total_size as f64).sum()
            ),
            AggregationType::Average => {
                let total: f64 = tiles.iter().map(|t| t.total_size as f64).sum();
                let count = tiles.len() as f64;
                AggregationResult::Average(if count > 0.0 { total / count } else { 0.0 })
            },
            AggregationType::VectorSum => {
                let mut result_vector = vec![0.0f32; 128]; // Fixed size for now
                for tile in tiles {
                    for (i, val) in tile.aggregate_vector.iter().enumerate() {
                        if i < result_vector.len() {
                            result_vector[i] += val;
                        }
                    }
                }
                AggregationResult::Vector(result_vector)
            },
        }
    }
    
    // Helper methods
    
    fn extract_c_slice(content: &[u8], c_slice: (u64, u64)) -> Vec<u8> {
        let start = c_slice.0 as usize;
        let end = (c_slice.1 as usize).min(content.len());
        if start < content.len() && start < end {
            content[start..end].to_vec()
        } else {
            Vec::new()
        }
    }
    
    fn tiles_intersect(box1: &FourDBoundingBox, box2: &FourDBoundingBox) -> bool {
        box1.r_min <= box2.r_max && box1.r_max >= box2.r_min &&
        box1.c_min <= box2.c_max && box1.c_max >= box2.c_min &&
        box1.v_min <= box2.v_max && box1.v_max >= box2.v_min &&
        box1.i_min <= box2.i_max && box1.i_max >= box2.i_min
    }
    
    fn compute_intersection(box1: &FourDBoundingBox, box2: &FourDBoundingBox) -> FourDBoundingBox {
        FourDBoundingBox {
            r_min: box1.r_min.max(box2.r_min),
            r_max: box1.r_max.min(box2.r_max),
            c_min: box1.c_min.max(box2.c_min),
            c_max: box1.c_max.min(box2.c_max),
            v_min: box1.v_min.max(box2.v_min),
            v_max: box1.v_max.min(box2.v_max),
            i_min: box1.i_min.max(box2.i_min),
            i_max: box1.i_max.min(box2.i_max),
        }
    }
}

/// Reference to a 4D tile for operations
#[derive(Debug, Clone)]
pub struct FourDTileRef {
    pub tile_id: Uuid,
    pub bounding_box: FourDBoundingBox,
    pub node_count: usize,
    pub total_size: usize,
    pub aggregate_vector: Vec<f32>,
    pub security_level: SecurityLevel,
}

/// Projected node after 4D projection
#[derive(Debug, Clone)]
pub struct ProjectedNode {
    pub hash_key: Hash,
    pub projected_data: Vec<u8>,
    pub metadata: HashMap<String, String>,
}

/// Join predicate for spatial joins
#[derive(Debug, Clone)]
pub enum JoinPredicate {
    Intersects,
    Contains,
    Within,
    Overlaps,
    Custom(String), // Custom predicate logic
}

impl JoinPredicate {
    pub fn evaluate(&self, left: &FourDTileRef, right: &FourDTileRef) -> bool {
        match self {
            JoinPredicate::Intersects => {
                FourDAlgebra::tiles_intersect(&left.bounding_box, &right.bounding_box)
            },
            JoinPredicate::Contains => {
                Self::tile_contains(&left.bounding_box, &right.bounding_box)
            },
            JoinPredicate::Within => {
                Self::tile_contains(&right.bounding_box, &left.bounding_box)
            },
            JoinPredicate::Overlaps => {
                FourDAlgebra::tiles_intersect(&left.bounding_box, &right.bounding_box) &&
                !Self::tile_contains(&left.bounding_box, &right.bounding_box) &&
                !Self::tile_contains(&right.bounding_box, &left.bounding_box)
            },
            JoinPredicate::Custom(_) => {
                // Custom logic would be implemented here
                true
            },
        }
    }
    
    fn tile_contains(container: &FourDBoundingBox, contained: &FourDBoundingBox) -> bool {
        container.r_min <= contained.r_min && container.r_max >= contained.r_max &&
        container.c_min <= contained.c_min && container.c_max >= contained.c_max &&
        container.v_min <= contained.v_min && container.v_max >= contained.v_max &&
        container.i_min <= contained.i_min && container.i_max >= contained.i_max
    }
}

/// Join operation result
#[derive(Debug, Clone)]
pub struct JoinResult {
    pub left_tile_id: Uuid,
    pub right_tile_id: Uuid,
    pub intersection_box: FourDBoundingBox,
}

/// Aggregation types for 4D reduce operations
#[derive(Debug, Clone)]
pub enum AggregationType {
    Count,
    Sum,
    Average,
    VectorSum,
}

/// Aggregation result
#[derive(Debug, Clone)]
pub enum AggregationResult {
    Count(usize),
    Sum(f64),
    Average(f64),
    Vector(Vec<f32>),
}

/// 4D Distance metrics for similarity operations
pub struct FourDDistance;

impl FourDDistance {
    /// Euclidean distance in 4D space
    pub fn euclidean(coord1: &FourDCoordinate, coord2: &FourDCoordinate) -> f64 {
        let dr = (coord1.r as f64 - coord2.r as f64).powi(2);
        let dc = (coord1.c as f64 - coord2.c as f64).powi(2);
        let dv = (coord1.v - coord2.v).powi(2);
        let di = (coord1.i as f64 - coord2.i as f64).powi(2);
        
        (dr + dc + dv + di).sqrt()
    }
    
    /// Manhattan distance in 4D space
    pub fn manhattan(coord1: &FourDCoordinate, coord2: &FourDCoordinate) -> f64 {
        let dr = (coord1.r as f64 - coord2.r as f64).abs();
        let dc = (coord1.c as f64 - coord2.c as f64).abs();
        let dv = (coord1.v - coord2.v).abs();
        let di = (coord1.i as f64 - coord2.i as f64).abs();
        
        dr + dc + dv + di
    }
    
    /// Cosine similarity for vector components
    pub fn cosine_similarity(vec1: &[f32], vec2: &[f32]) -> f64 {
        if vec1.len() != vec2.len() {
            return 0.0;
        }
        
        let dot_product: f32 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();
        let norm1: f32 = vec1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm2: f32 = vec2.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        if norm1 == 0.0 || norm2 == 0.0 {
            0.0
        } else {
            (dot_product / (norm1 * norm2)) as f64
        }
    }
}

/// 4D Coordinate utilities
pub struct FourDCoordinateUtils;

impl FourDCoordinateUtils {
    /// Generate coordinate from hash
    pub fn from_hash(hash: &Hash, intent: u64) -> FourDCoordinate {
        let bytes = hash.as_bytes();
        
        FourDCoordinate {
            r: u64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], 
                                  bytes[4], bytes[5], bytes[6], bytes[7]]),
            c: u64::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11], 
                                  bytes[12], bytes[13], bytes[14], bytes[15]]),
            v: f64::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19], 
                                  bytes[20], bytes[21], bytes[22], bytes[23]]),
            i: intent,
        }
    }
    
    /// Normalize coordinate to bounding box
    pub fn normalize_to_box(coord: &FourDCoordinate, bbox: &FourDBoundingBox) -> FourDCoordinate {
        FourDCoordinate {
            r: coord.r.max(bbox.r_min).min(bbox.r_max),
            c: coord.c.max(bbox.c_min).min(bbox.c_max),
            v: coord.v.max(bbox.v_min).min(bbox.v_max),
            i: coord.i.max(bbox.i_min).min(bbox.i_max),
        }
    }
    
    /// Check if coordinate is within bounding box
    pub fn is_within_box(coord: &FourDCoordinate, bbox: &FourDBoundingBox) -> bool {
        coord.r >= bbox.r_min && coord.r <= bbox.r_max &&
        coord.c >= bbox.c_min && coord.c <= bbox.c_max &&
        coord.v >= bbox.v_min && coord.v <= bbox.v_max &&
        coord.i >= bbox.i_min && coord.i <= bbox.i_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_4d_distance_euclidean() {
        let coord1 = FourDCoordinate { r: 0, c: 0, v: 0.0, i: 0 };
        let coord2 = FourDCoordinate { r: 1, c: 1, v: 1.0, i: 1 };
        
        let distance = FourDDistance::euclidean(&coord1, &coord2);
        assert!(distance > 0.0);
    }
    
    #[test]
    fn test_4d_algebra_select() {
        let bbox = FourDBoundingBox {
            r_min: 0, r_max: 10,
            c_min: 0, c_max: 10,
            v_min: 0.0, v_max: 10.0,
            i_min: 0, i_max: 10,
        };
        
        let tile_ref = FourDTileRef {
            tile_id: Uuid::new_v4(),
            bounding_box: bbox,
            node_count: 5,
            total_size: 1024,
            aggregate_vector: vec![1.0, 2.0, 3.0],
            security_level: SecurityLevel::Public,
        };
        
        let tiles = vec![tile_ref];
        let selected = FourDAlgebra::select_4d(&tiles, (0, 5), (0, 5), (0.0, 5.0), (0, 5));
        
        assert_eq!(selected.len(), 1);
    }
    
    #[test]
    fn test_coordinate_from_hash() {
        let content = b"test content";
        let hash = blake3::hash(content);
        let coord = FourDCoordinateUtils::from_hash(&hash, 42);
        
        assert_eq!(coord.i, 42);
        assert!(coord.r > 0);
    }
}
