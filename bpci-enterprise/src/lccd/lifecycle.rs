//! LCCD Cell Lifecycle Management
//! 
//! Implements cellular lifecycle operations:
//! - Cell growth (absorbing nearby nodes)
//! - Cell division (splitting when too large)
//! - Cell merging (combining when beneficial)
//! - Cell dissolution (removing unhealthy cells)
//! 
//! # Lifecycle States
//! 
//! ```text
//! Forming → Active → Growing → Dividing → Active
//!            ↓         ↓         ↓
//!        Unhealthy  Merging  Dissolving
//! ```

use super::cell::{LccdCell, CellId, NodeId, CellState, CellHealth, CurvatureProfile};
use crate::w4_fluid::{EdgeId, EdgeState};
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

/// Cell lifecycle configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleConfig {
    /// Health threshold for unhealthy state
    pub unhealthy_threshold: f64,
    
    /// Health threshold for dissolution
    pub dissolution_threshold: f64,
    
    /// Size threshold for division
    pub division_size_threshold: usize,
    
    /// Size threshold for merging
    pub merge_size_threshold: usize,
    
    /// Minimum epochs before division
    pub min_epochs_before_division: u64,
    
    /// Minimum epochs before merging
    pub min_epochs_before_merge: u64,
}

impl Default for LifecycleConfig {
    fn default() -> Self {
        Self {
            unhealthy_threshold: 0.5,
            dissolution_threshold: 0.3,
            division_size_threshold: 8,
            merge_size_threshold: 4,
            min_epochs_before_division: 10,
            min_epochs_before_merge: 5,
        }
    }
}

/// Cell lifecycle manager
pub struct CellLifecycleManager {
    config: LifecycleConfig,
    cell_epochs: HashMap<CellId, u64>,
}

impl CellLifecycleManager {
    /// Create a new lifecycle manager
    pub fn new(config: LifecycleConfig) -> Self {
        Self {
            config,
            cell_epochs: HashMap::new(),
        }
    }
    
    /// Update cell state based on health and lifecycle rules
    pub fn update_cell_state(&mut self, cell: &mut LccdCell) {
        // Increment epoch counter
        let epoch = self.cell_epochs.entry(cell.cell_id).or_insert(0);
        *epoch += 1;
        
        // Check health thresholds
        if cell.health.score < self.config.dissolution_threshold {
            cell.state = CellState::Dissolving;
            return;
        }
        
        if cell.health.score < self.config.unhealthy_threshold {
            cell.state = CellState::Unhealthy;
            return;
        }
        
        // Check size thresholds
        if cell.members.len() > self.config.division_size_threshold
            && *epoch >= self.config.min_epochs_before_division
        {
            cell.state = CellState::Dividing;
            return;
        }
        
        if cell.members.len() < self.config.merge_size_threshold
            && *epoch >= self.config.min_epochs_before_merge
        {
            cell.state = CellState::Merging;
            return;
        }
        
        // Default to active if healthy
        if cell.health.score >= self.config.unhealthy_threshold {
            cell.state = CellState::Active;
        }
    }
    
    /// Grow cell by absorbing nearby nodes
    pub fn grow_cell(
        &self,
        cell: &mut LccdCell,
        candidate_nodes: &[NodeId],
        edges: &HashMap<EdgeId, EdgeState>,
    ) -> Vec<NodeId> {
        let member_set: HashSet<NodeId> = cell.members.iter().copied().collect();
        let mut absorbed = Vec::new();
        
        for &candidate in candidate_nodes {
            if member_set.contains(&candidate) {
                continue;
            }
            
            // Check if candidate is connected to cell members
            let is_connected = cell.members.iter().any(|&member| {
                let edge1 = EdgeId::new(member, candidate);
                let edge2 = EdgeId::new(candidate, member);
                edges.contains_key(&edge1) || edges.contains_key(&edge2)
            });
            
            if is_connected {
                cell.members.push(candidate);
                absorbed.push(candidate);
            }
        }
        
        absorbed
    }
    
    /// Divide cell into two smaller cells
    pub fn divide_cell(
        &self,
        cell: &LccdCell,
        next_cell_id: CellId,
    ) -> (LccdCell, LccdCell) {
        // Simple division: split members in half
        let mid = cell.members.len() / 2;
        
        let members1 = cell.members[..mid].to_vec();
        let members2 = cell.members[mid..].to_vec();
        
        // Create two new cells
        let cell1 = LccdCell {
            cell_id: cell.cell_id,
            members: members1,
            boundary_edges: Vec::new(), // Will be recalculated
            curvature_profile: cell.curvature_profile.clone(),
            health: CellHealth {
                score: 0.7, // Initial health after division
                size_health: 0.7,
                connectivity_health: 0.7,
                boundary_health: 0.7,
            },
            state: CellState::Forming,
        };
        
        let cell2 = LccdCell {
            cell_id: next_cell_id,
            members: members2,
            boundary_edges: Vec::new(),
            curvature_profile: cell.curvature_profile.clone(),
            health: CellHealth {
                score: 0.7,
                size_health: 0.7,
                connectivity_health: 0.7,
                boundary_health: 0.7,
            },
            state: CellState::Forming,
        };
        
        (cell1, cell2)
    }
    
    /// Merge two cells into one
    pub fn merge_cells(
        &self,
        cell1: &LccdCell,
        cell2: &LccdCell,
    ) -> LccdCell {
        // Combine members
        let mut members = cell1.members.clone();
        members.extend(cell2.members.iter());
        
        // Combine boundary edges
        let mut boundary_edges = cell1.boundary_edges.clone();
        boundary_edges.extend(cell2.boundary_edges.iter());
        
        // Average curvature profiles
        let curvature_profile = CurvatureProfile {
            avg_internal_curvature: (cell1.curvature_profile.avg_internal_curvature
                + cell2.curvature_profile.avg_internal_curvature)
                / 2.0,
            avg_boundary_curvature: (cell1.curvature_profile.avg_boundary_curvature
                + cell2.curvature_profile.avg_boundary_curvature)
                / 2.0,
            min_curvature: cell1
                .curvature_profile
                .min_curvature
                .min(cell2.curvature_profile.min_curvature),
            max_curvature: cell1
                .curvature_profile
                .max_curvature
                .max(cell2.curvature_profile.max_curvature),
        };
        
        // Average health
        let health = CellHealth {
            score: (cell1.health.score + cell2.health.score) / 2.0,
            size_health: (cell1.health.size_health + cell2.health.size_health) / 2.0,
            connectivity_health: (cell1.health.connectivity_health
                + cell2.health.connectivity_health)
                / 2.0,
            boundary_health: (cell1.health.boundary_health + cell2.health.boundary_health) / 2.0,
        };
        
        LccdCell {
            cell_id: cell1.cell_id, // Keep first cell's ID
            members,
            boundary_edges,
            curvature_profile,
            health,
            state: CellState::Forming,
        }
    }
    
    /// Check if cell should be dissolved
    pub fn should_dissolve(&self, cell: &LccdCell) -> bool {
        cell.state == CellState::Dissolving
            || cell.health.score < self.config.dissolution_threshold
    }
    
    /// Find candidate cells for merging
    pub fn find_merge_candidates(
        &self,
        cell: &LccdCell,
        all_cells: &[LccdCell],
        edges: &HashMap<EdgeId, EdgeState>,
    ) -> Vec<CellId> {
        let mut candidates = Vec::new();
        let member_set: HashSet<NodeId> = cell.members.iter().copied().collect();
        
        for other_cell in all_cells {
            if other_cell.cell_id == cell.cell_id {
                continue;
            }
            
            // Check if cells are adjacent (share boundary)
            let is_adjacent = other_cell.members.iter().any(|&other_member| {
                cell.members.iter().any(|&member| {
                    let edge1 = EdgeId::new(member, other_member);
                    let edge2 = EdgeId::new(other_member, member);
                    edges.contains_key(&edge1) || edges.contains_key(&edge2)
                })
            });
            
            if is_adjacent && other_cell.members.len() < self.config.merge_size_threshold {
                candidates.push(other_cell.cell_id);
            }
        }
        
        candidates
    }
    
    /// Reset epoch counter for a cell
    pub fn reset_epoch(&mut self, cell_id: CellId) {
        self.cell_epochs.insert(cell_id, 0);
    }
    
    /// Get epoch count for a cell
    pub fn get_epoch(&self, cell_id: CellId) -> u64 {
        self.cell_epochs.get(&cell_id).copied().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::w4_fluid::EdgeTelemetry;
    
    fn create_test_cell(cell_id: CellId, members: Vec<NodeId>, health_score: f64) -> LccdCell {
        LccdCell {
            cell_id,
            members,
            boundary_edges: Vec::new(),
            curvature_profile: CurvatureProfile {
                avg_internal_curvature: 0.5,
                avg_boundary_curvature: -0.3,
                min_curvature: -0.5,
                max_curvature: 0.8,
            },
            health: CellHealth {
                score: health_score,
                size_health: health_score,
                connectivity_health: health_score,
                boundary_health: health_score,
            },
            state: CellState::Active,
        }
    }
    
    fn create_test_edge(source: u64, dest: u64) -> EdgeState {
        let id = EdgeId::new(source, dest);
        EdgeState {
            id,
            weight: 1.0,
            viscosity: 0.03,
            temperature: 0.0,
            curvature: 0.0,
            density: 0.0,
            load: 0.0,
            telemetry: EdgeTelemetry::new(1000.0),
            last_update: None,
        }
    }
    
    #[test]
    fn test_lifecycle_config() {
        let config = LifecycleConfig::default();
        assert_eq!(config.unhealthy_threshold, 0.5);
        assert_eq!(config.dissolution_threshold, 0.3);
    }
    
    #[test]
    fn test_lifecycle_manager_creation() {
        let config = LifecycleConfig::default();
        let manager = CellLifecycleManager::new(config);
        assert_eq!(manager.cell_epochs.len(), 0);
    }
    
    #[test]
    fn test_update_cell_state_healthy() {
        let config = LifecycleConfig::default();
        let mut manager = CellLifecycleManager::new(config);
        
        let mut cell = create_test_cell(1, vec![1, 2, 3, 4, 5], 0.8);
        manager.update_cell_state(&mut cell);
        
        assert_eq!(cell.state, CellState::Active);
    }
    
    #[test]
    fn test_update_cell_state_unhealthy() {
        let config = LifecycleConfig::default();
        let mut manager = CellLifecycleManager::new(config);
        
        let mut cell = create_test_cell(1, vec![1, 2, 3], 0.4);
        manager.update_cell_state(&mut cell);
        
        assert_eq!(cell.state, CellState::Unhealthy);
    }
    
    #[test]
    fn test_update_cell_state_dissolving() {
        let config = LifecycleConfig::default();
        let mut manager = CellLifecycleManager::new(config);
        
        let mut cell = create_test_cell(1, vec![1, 2], 0.2);
        manager.update_cell_state(&mut cell);
        
        assert_eq!(cell.state, CellState::Dissolving);
    }
    
    #[test]
    fn test_grow_cell() {
        let config = LifecycleConfig::default();
        let manager = CellLifecycleManager::new(config);
        
        let mut cell = create_test_cell(1, vec![1, 2, 3], 0.8);
        
        let mut edges = HashMap::new();
        edges.insert(EdgeId::new(3, 4), create_test_edge(3, 4));
        
        let candidates = vec![4, 5];
        let absorbed = manager.grow_cell(&mut cell, &candidates, &edges);
        
        assert_eq!(absorbed.len(), 1);
        assert_eq!(absorbed[0], 4);
        assert!(cell.members.contains(&4));
    }
    
    #[test]
    fn test_divide_cell() {
        let config = LifecycleConfig::default();
        let manager = CellLifecycleManager::new(config);
        
        let cell = create_test_cell(1, vec![1, 2, 3, 4, 5, 6], 0.8);
        let (cell1, cell2) = manager.divide_cell(&cell, 2);
        
        assert_eq!(cell1.cell_id, 1);
        assert_eq!(cell2.cell_id, 2);
        assert_eq!(cell1.members.len(), 3);
        assert_eq!(cell2.members.len(), 3);
        assert_eq!(cell1.state, CellState::Forming);
        assert_eq!(cell2.state, CellState::Forming);
    }
    
    #[test]
    fn test_merge_cells() {
        let config = LifecycleConfig::default();
        let manager = CellLifecycleManager::new(config);
        
        let cell1 = create_test_cell(1, vec![1, 2], 0.6);
        let cell2 = create_test_cell(2, vec![3, 4], 0.7);
        
        let merged = manager.merge_cells(&cell1, &cell2);
        
        assert_eq!(merged.cell_id, 1);
        assert_eq!(merged.members.len(), 4);
        assert!(merged.members.contains(&1));
        assert!(merged.members.contains(&4));
    }
    
    #[test]
    fn test_should_dissolve() {
        let config = LifecycleConfig::default();
        let manager = CellLifecycleManager::new(config);
        
        let mut cell = create_test_cell(1, vec![1, 2], 0.2);
        cell.state = CellState::Dissolving;
        
        assert!(manager.should_dissolve(&cell));
    }
    
    #[test]
    fn test_epoch_tracking() {
        let config = LifecycleConfig::default();
        let mut manager = CellLifecycleManager::new(config);
        
        let mut cell = create_test_cell(1, vec![1, 2, 3], 0.8);
        
        assert_eq!(manager.get_epoch(1), 0);
        
        manager.update_cell_state(&mut cell);
        assert_eq!(manager.get_epoch(1), 1);
        
        manager.update_cell_state(&mut cell);
        assert_eq!(manager.get_epoch(1), 2);
        
        manager.reset_epoch(1);
        assert_eq!(manager.get_epoch(1), 0);
    }
}
