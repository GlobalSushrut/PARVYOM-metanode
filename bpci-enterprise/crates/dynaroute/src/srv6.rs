//! # Segment Routing v6 (SRv6)
//! 
//! Programmable paths encoded in packet headers.
//! Policy + vPod + egress routing without stateful middleboxes.

use std::net::Ipv6Addr;
use blake3;
use serde::{Serialize, Deserialize};

/// SRv6 Segment types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SRv6SegmentType {
    /// ENC/HTTP-Cage policy enforcement
    PolicyEnforcement,
    
    /// vPod rendezvous routing
    VPodRendezvous,
    
    /// Mesh return path
    MeshReturn,
    
    /// Custom segment
    Custom,
}

/// SRv6 Segment - single segment in routing path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRv6Segment {
    /// Segment type
    pub segment_type: SRv6SegmentType,
    
    /// IPv6 address for this segment
    pub address: Ipv6Addr,
    
    /// Optional metadata
    pub metadata: Option<String>,
}

impl SRv6Segment {
    /// Create new segment
    pub fn new(segment_type: SRv6SegmentType, address: Ipv6Addr) -> Self {
        Self {
            segment_type,
            address,
            metadata: None,
        }
    }
    
    /// Create policy enforcement segment
    pub fn policy(policy_id: &str) -> Self {
        let base = Ipv6Addr::new(0x2001, 0x0db8, 0x0001, 0, 0, 0, 0, 0);
        let address = Self::encode_with_hash(base, policy_id);
        
        Self {
            segment_type: SRv6SegmentType::PolicyEnforcement,
            address,
            metadata: Some(policy_id.to_string()),
        }
    }
    
    /// Create vPod rendezvous segment
    pub fn vpod(vpod_id: &str) -> Self {
        let base = Ipv6Addr::new(0x2001, 0x0db8, 0x0002, 0, 0, 0, 0, 0);
        let address = Self::encode_with_hash(base, vpod_id);
        
        Self {
            segment_type: SRv6SegmentType::VPodRendezvous,
            address,
            metadata: Some(vpod_id.to_string()),
        }
    }
    
    /// Create mesh return segment
    pub fn mesh_return(egress_id: &str) -> Self {
        let base = Ipv6Addr::new(0x2001, 0x0db8, 0x0003, 0, 0, 0, 0, 0);
        let address = Self::encode_with_hash(base, egress_id);
        
        Self {
            segment_type: SRv6SegmentType::MeshReturn,
            address,
            metadata: Some(egress_id.to_string()),
        }
    }
    
    /// Encode base address with hash
    fn encode_with_hash(base: Ipv6Addr, data: &str) -> Ipv6Addr {
        let hash = blake3::hash(data.as_bytes());
        let hash_bytes = hash.as_bytes();
        
        let mut addr_bytes = base.octets();
        // XOR lower 64 bits with hash
        for i in 8..16 {
            addr_bytes[i] ^= hash_bytes[i - 8];
        }
        
        Ipv6Addr::from(addr_bytes)
    }
}

/// SRv6 Segment List - complete routing path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRv6SegmentList {
    /// List of segments in order
    pub segments: Vec<SRv6Segment>,
    
    /// Segment list ID (for caching)
    pub seglist_id: u32,
}

impl SRv6SegmentList {
    /// Create new segment list
    pub fn new(segments: Vec<SRv6Segment>) -> Self {
        // Compute seglist_id from segments
        let mut hasher = blake3::Hasher::new();
        for seg in &segments {
            hasher.update(&seg.address.octets());
        }
        let hash = hasher.finalize();
        let seglist_id = u32::from_le_bytes(hash.as_bytes()[0..4].try_into().unwrap());
        
        Self {
            segments,
            seglist_id,
        }
    }
    
    /// Create segment list for vPod routing
    /// 
    /// # Arguments
    /// 
    /// * `policy_id` - Policy enforcement ID
    /// * `vpod_id` - Target vPod ID
    /// * `egress_id` - Optional egress/return path ID
    pub fn for_vpod(policy_id: &str, vpod_id: &str, egress_id: Option<&str>) -> Self {
        let mut segments = vec![
            SRv6Segment::policy(policy_id),
            SRv6Segment::vpod(vpod_id),
        ];
        
        if let Some(egress) = egress_id {
            segments.push(SRv6Segment::mesh_return(egress));
        }
        
        Self::new(segments)
    }
    
    /// Get segment addresses as Vec
    pub fn addresses(&self) -> Vec<Ipv6Addr> {
        self.segments.iter().map(|s| s.address).collect()
    }
    
    /// Get segment count
    pub fn len(&self) -> usize {
        self.segments.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }
}

/// SRv6 Policy Manager
/// 
/// Manages SRv6 policies and segment lists
#[derive(Debug)]
pub struct SRv6PolicyManager {
    /// Cached segment lists: seglist_id → SRv6SegmentList
    segment_lists: parking_lot::RwLock<std::collections::HashMap<u32, SRv6SegmentList>>,
}

impl SRv6PolicyManager {
    /// Create new policy manager
    pub fn new() -> Self {
        Self {
            segment_lists: parking_lot::RwLock::new(std::collections::HashMap::new()),
        }
    }
    
    /// Register segment list
    pub fn register_seglist(&self, seglist: SRv6SegmentList) -> u32 {
        let seglist_id = seglist.seglist_id;
        self.segment_lists.write().insert(seglist_id, seglist);
        seglist_id
    }
    
    /// Get segment list by ID
    pub fn get_seglist(&self, seglist_id: u32) -> Option<SRv6SegmentList> {
        self.segment_lists.read().get(&seglist_id).cloned()
    }
    
    /// Update policies (atomic swap)
    pub async fn update_policies(&self, new_seglists: Vec<SRv6SegmentList>) -> anyhow::Result<()> {
        let mut map = std::collections::HashMap::new();
        for seglist in new_seglists {
            map.insert(seglist.seglist_id, seglist);
        }
        
        // Atomic swap
        *self.segment_lists.write() = map;
        
        Ok(())
    }
}

impl Default for SRv6PolicyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_creation() {
        let seg = SRv6Segment::policy("enc-enterprise");
        assert_eq!(seg.segment_type, SRv6SegmentType::PolicyEnforcement);
        assert_eq!(seg.metadata, Some("enc-enterprise".to_string()));
    }
    
    #[test]
    fn test_seglist_creation() {
        let seglist = SRv6SegmentList::for_vpod("policy1", "vpod123", Some("egress1"));
        assert_eq!(seglist.len(), 3);
        assert_eq!(seglist.segments[0].segment_type, SRv6SegmentType::PolicyEnforcement);
        assert_eq!(seglist.segments[1].segment_type, SRv6SegmentType::VPodRendezvous);
        assert_eq!(seglist.segments[2].segment_type, SRv6SegmentType::MeshReturn);
    }
    
    #[test]
    fn test_seglist_deterministic() {
        let seglist1 = SRv6SegmentList::for_vpod("policy1", "vpod123", None);
        let seglist2 = SRv6SegmentList::for_vpod("policy1", "vpod123", None);
        
        assert_eq!(seglist1.seglist_id, seglist2.seglist_id);
    }
    
    #[test]
    fn test_policy_manager() {
        let manager = SRv6PolicyManager::new();
        let seglist = SRv6SegmentList::for_vpod("policy1", "vpod123", None);
        let id = manager.register_seglist(seglist.clone());
        
        let retrieved = manager.get_seglist(id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().seglist_id, seglist.seglist_id);
    }
}
