use std::collections::{HashMap, VecDeque};
use serde::{Deserialize, Serialize};
use blake3::Hash;
use tracing::{debug, warn};


use bpi_merkle::MerkleTree;
use crate::error::{DockLockError, DockLockResult};

/// Event ID type - 128-bit unique identifier
pub type EventId = u128;

/// Logical sequence number - monotonic, not wall-clock time
pub type LogicalSequence = u64;

/// Event kinds for the canonical event stream
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventKind {
    /// Container lifecycle events
    ContainerStart,
    ContainerStop,
    ContainerPause,
    ContainerResume,
    
    /// Microservice events
    ServiceDeploy,
    ServiceUpdate,
    ServiceScale,
    ServiceRemove,
    
    /// Wallet and identity events
    IdentityCreate,
    IdentityUpdate,
    WalletConnect,
    WalletDisconnect,
    
    /// DAO governance events
    ProposalCreate,
    ProposalVote,
    ProposalExecute,
    
    /// Compliance and monitoring events
    ComplianceCheck,
    MonitoringAlert,
    PolicyViolation,
    
    /// System events
    SystemStart,
    SystemStop,
    ConfigUpdate,
    
    /// Custom application events
    Custom(String),
}

/// Canonical Event structure as per logic.md specification
/// Event = (eid: u128, parent: u128?, t_seq: u64, kind: ENUM, payload_commit: bytes32)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Unique event identifier
    pub eid: EventId,
    
    /// Optional parent event for causality tracking
    pub parent: Option<EventId>,
    
    /// Logical sequence number (monotonic, not wall-clock)
    pub t_seq: LogicalSequence,
    
    /// Event type/kind
    pub kind: EventKind,
    
    /// Commitment to the event payload (32-byte hash)
    pub payload_commit: [u8; 32],
    
    /// Optional metadata
    pub metadata: HashMap<String, String>,
}

impl Event {
    /// Create a new event
    pub fn new(
        eid: EventId,
        parent: Option<EventId>,
        t_seq: LogicalSequence,
        kind: EventKind,
        payload: &[u8],
    ) -> Self {
        let payload_commit = blake3::hash(payload).into();
        
        Self {
            eid,
            parent,
            t_seq,
            kind,
            payload_commit,
            metadata: HashMap::new(),
        }
    }
    
    /// Add metadata to the event
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

// Custom encoding implementation for Event
impl Event {
    /// Encode event to canonical CBOR with string representation for large numbers
    fn encode_to_cbor(&self) -> Result<Vec<u8>, String> {
        // Create a serializable version with string representation of large numbers
        let serializable = serde_json::json!({
            "eid": self.eid.to_string(),
            "parent": self.parent.map(|p| p.to_string()),
            "t_seq": self.t_seq,
            "kind": format!("{:?}", self.kind),
            "payload_commit": self.payload_commit.to_vec(),
            "metadata": self.metadata
        });
        
        serde_cbor::to_vec(&serializable)
            .map_err(|e| format!("CBOR encoding failed: {}", e))
    }

    /// Get the canonical encoding of this event
    pub fn encode(&self) -> DockLockResult<Vec<u8>> {
        self.encode_to_cbor()
            .map_err(|e| DockLockError::EncodingError(format!("Failed to encode event: {}", e)))
    }
    
    /// Get the hash of this event
    pub fn hash(&self) -> DockLockResult<Hash> {
        let encoded = self.encode()?;
        Ok(blake3::hash(&encoded))
    }
}

/// Event stream configuration
#[derive(Debug, Clone)]
pub struct EventStreamConfig {
    /// Maximum number of events to keep in memory
    pub max_events: usize,
    
    /// Enable reorder detection
    pub enable_reorder_detection: bool,
    
    /// Maximum allowed reorder window
    pub max_reorder_window: u64,
}

impl Default for EventStreamConfig {
    fn default() -> Self {
        Self {
            max_events: 10000,
            enable_reorder_detection: true,
            max_reorder_window: 100,
        }
    }
}

/// Canonical event stream for deterministic event ordering
#[derive(Debug)]
pub struct CanonicalEventStream {
    /// Configuration
    config: EventStreamConfig,
    
    /// Events ordered by t_seq
    events: VecDeque<Event>,
    
    /// Event lookup by ID
    event_lookup: HashMap<EventId, usize>,
    
    /// Current logical sequence counter
    current_t_seq: LogicalSequence,
    
    /// Merkle tree of events (updated on changes)
    merkle_tree: Option<MerkleTree>,
    
    /// Last computed Merkle root
    last_root: Option<Hash>,
    
    /// Reorder detection state
    reorder_buffer: VecDeque<Event>,
}

impl CanonicalEventStream {
    /// Create a new canonical event stream
    pub fn new(config: EventStreamConfig) -> Self {
        Self {
            config,
            events: VecDeque::new(),
            event_lookup: HashMap::new(),
            current_t_seq: 0,
            merkle_tree: None,
            last_root: None,
            reorder_buffer: VecDeque::new(),
        }
    }
    
    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(EventStreamConfig::default())
    }
    
    /// Add an event to the stream
    pub fn add_event(&mut self, mut event: Event) -> DockLockResult<()> {
        // Assign logical sequence if not set
        if event.t_seq == 0 {
            self.current_t_seq += 1;
            event.t_seq = self.current_t_seq;
        } else {
            // Update current sequence to maintain monotonicity
            if event.t_seq > self.current_t_seq {
                self.current_t_seq = event.t_seq;
            }
        }
        
        debug!("Adding event {} with t_seq {}", event.eid, event.t_seq);
        
        // Check for reordering if enabled
        if self.config.enable_reorder_detection {
            if let Some(reorder) = self.detect_reorder(&event)? {
                warn!("Reorder detected: {:?}", reorder);
                return Err(DockLockError::EventReorderDetected(format!(
                    "Event {} out of order at t_seq {}",
                    event.eid, event.t_seq
                )));
            }
        }
        
        // Add to events in t_seq order
        self.insert_event_ordered(event)?;
        
        // Update Merkle tree
        self.update_merkle_root()?;
        
        // Cleanup old events if needed
        self.cleanup_old_events();
        
        Ok(())
    }
    
    /// Insert event maintaining t_seq order
    fn insert_event_ordered(&mut self, event: Event) -> DockLockResult<()> {
        let eid = event.eid;
        let t_seq = event.t_seq;
        
        // Find insertion position
        let pos = self.events
            .iter()
            .position(|e| e.t_seq > t_seq)
            .unwrap_or(self.events.len());
        
        // Insert event
        self.events.insert(pos, event);
        
        // Update lookup table (indices may have shifted)
        self.rebuild_lookup_table();
        
        debug!("Inserted event {} at position {} with t_seq {}", eid, pos, t_seq);
        Ok(())
    }
    
    /// Rebuild the event lookup table
    fn rebuild_lookup_table(&mut self) {
        self.event_lookup.clear();
        for (idx, event) in self.events.iter().enumerate() {
            self.event_lookup.insert(event.eid, idx);
        }
    }
    
    /// Detect event reordering
    fn detect_reorder(&self, event: &Event) -> DockLockResult<Option<String>> {
        if self.events.is_empty() {
            return Ok(None);
        }
        
        let last_t_seq = self.events.back().unwrap().t_seq;
        
        // Check if event is significantly out of order
        if event.t_seq < last_t_seq && (last_t_seq - event.t_seq) > self.config.max_reorder_window {
            return Ok(Some(format!(
                "Event t_seq {} is {} positions behind current t_seq {}",
                event.t_seq,
                last_t_seq - event.t_seq,
                last_t_seq
            )));
        }
        
        Ok(None)
    }
    
    /// Update the Merkle root over all events
    fn update_merkle_root(&mut self) -> DockLockResult<()> {
        if self.events.is_empty() {
            self.merkle_tree = None;
            self.last_root = None;
            return Ok(());
        }
        
        // Collect encoded events in t_seq order
        let mut event_hashes = Vec::new();
        for event in &self.events {
            let encoded = event.encode()?;
            event_hashes.push(blake3::hash(&encoded).as_bytes().to_vec());
        }
        
        // Build Merkle tree
        let tree = MerkleTree::new(event_hashes)
            .map_err(|e| DockLockError::MerkleError(format!("Failed to build Merkle tree: {}", e)))?;
        
        let root = tree.root()
            .map_err(|e| DockLockError::MerkleError(format!("Failed to get Merkle root: {}", e)))?;
        
        let root_hash = Hash::from_bytes(root.try_into().map_err(|_| {
            DockLockError::MerkleError("Invalid root hash length".to_string())
        })?);
        
        self.merkle_tree = Some(tree);
        self.last_root = Some(root_hash);
        
        debug!("Updated Merkle root: {:?}", self.last_root);
        Ok(())
    }
    
    /// Get the current Merkle root
    pub fn get_merkle_root(&self) -> Option<Hash> {
        self.last_root
    }
    
    /// Get event by ID
    pub fn get_event(&self, eid: EventId) -> Option<&Event> {
        self.event_lookup.get(&eid)
            .and_then(|&idx| self.events.get(idx))
    }
    
    /// Get events in a range of logical sequences
    pub fn get_events_range(&self, start_t_seq: LogicalSequence, end_t_seq: LogicalSequence) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| e.t_seq >= start_t_seq && e.t_seq <= end_t_seq)
            .collect()
    }
    
    /// Get all events (ordered by t_seq)
    pub fn get_all_events(&self) -> Vec<&Event> {
        self.events.iter().collect()
    }
    
    /// Get current logical sequence
    pub fn current_sequence(&self) -> LogicalSequence {
        self.current_t_seq
    }
    
    /// Get event count
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
    
    /// Cleanup old events to maintain memory limits
    fn cleanup_old_events(&mut self) {
        while self.events.len() > self.config.max_events {
            if let Some(removed) = self.events.pop_front() {
                self.event_lookup.remove(&removed.eid);
                debug!("Removed old event {} to maintain memory limits", removed.eid);
            }
        }
        
        // Rebuild lookup table after cleanup
        if self.events.len() < self.config.max_events {
            self.rebuild_lookup_table();
        }
    }
    
    /// Clear all events
    pub fn clear(&mut self) {
        self.events.clear();
        self.current_t_seq = 0;
        self.merkle_tree = None;
        self.reorder_buffer.clear();
    }

    /// Get recent events for witness correlation
    pub fn get_recent_events(&self, count: usize) -> Vec<&Event> {
        self.events.iter().rev().take(count).collect()
    }

    /// Find events by kind for witness correlation
    pub fn find_events_by_kind(&self, kind: EventKind, limit: usize) -> Vec<&Event> {
        self.events.iter().filter(|event| event.kind == kind).rev().take(limit).collect()
    }

    /// Get event by ID for witness correlation
    pub fn get_event_by_id(&self, event_id: u128) -> Option<&Event> {
        self.events.iter().find(|event| event.eid == event_id)
    }

    /// Get stream statistics
    pub fn stats(&self) -> EventStreamStats {
        EventStreamStats {
            total_events: self.events.len(),
            current_t_seq: self.current_t_seq,
            has_merkle_root: self.last_root.is_some(),
            merkle_root: self.last_root,
        }
    }
}

/// Event stream statistics
#[derive(Debug, Clone)]
pub struct EventStreamStats {
    pub total_events: usize,
    pub current_t_seq: LogicalSequence,
    pub has_merkle_root: bool,
    pub merkle_root: Option<Hash>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_event_creation() {
        let payload = b"test payload";
        let event = Event::new(1, None, 1, EventKind::ContainerStart, payload);
        
        assert_eq!(event.eid, 1);
        assert_eq!(event.parent, None);
        assert_eq!(event.t_seq, 1);
        assert_eq!(event.kind, EventKind::ContainerStart);
        assert_eq!(event.payload_commit, *blake3::hash(payload).as_bytes());
    }
    
    #[test]
    fn test_event_stream_creation() {
        let stream = CanonicalEventStream::default();
        assert_eq!(stream.event_count(), 0);
        assert_eq!(stream.current_sequence(), 0);
        assert!(stream.get_merkle_root().is_none());
    }
    
    #[test]
    fn test_add_single_event() {
        let mut stream = CanonicalEventStream::default();
        let event = Event::new(1, None, 0, EventKind::ContainerStart, b"payload1");
        
        stream.add_event(event).unwrap();
        
        assert_eq!(stream.event_count(), 1);
        assert_eq!(stream.current_sequence(), 1);
        assert!(stream.get_merkle_root().is_some());
    }
    
    #[test]
    fn test_add_multiple_events() {
        let mut stream = CanonicalEventStream::default();
        
        let event1 = Event::new(1, None, 0, EventKind::ContainerStart, b"payload1");
        let event2 = Event::new(2, Some(1), 0, EventKind::ServiceDeploy, b"payload2");
        let event3 = Event::new(3, Some(2), 0, EventKind::ContainerStop, b"payload3");
        
        stream.add_event(event1).unwrap();
        stream.add_event(event2).unwrap();
        stream.add_event(event3).unwrap();
        
        assert_eq!(stream.event_count(), 3);
        assert_eq!(stream.current_sequence(), 3);
        
        let events = stream.get_all_events();
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].t_seq, 1);
        assert_eq!(events[1].t_seq, 2);
        assert_eq!(events[2].t_seq, 3);
    }
    
    #[test]
    fn test_event_ordering() {
        let mut stream = CanonicalEventStream::default();
        
        // Add events out of order
        let event3 = Event::new(3, None, 3, EventKind::ContainerStop, b"payload3");
        let event1 = Event::new(1, None, 1, EventKind::ContainerStart, b"payload1");
        let event2 = Event::new(2, None, 2, EventKind::ServiceDeploy, b"payload2");
        
        stream.add_event(event3).unwrap();
        stream.add_event(event1).unwrap();
        stream.add_event(event2).unwrap();
        
        let events = stream.get_all_events();
        assert_eq!(events[0].t_seq, 1);
        assert_eq!(events[1].t_seq, 2);
        assert_eq!(events[2].t_seq, 3);
    }
    
    #[test]
    fn test_reorder_detection() {
        let mut config = EventStreamConfig::default();
        config.max_reorder_window = 2;
        let mut stream = CanonicalEventStream::new(config);
        
        // Add events in order
        let event1 = Event::new(1, None, 1, EventKind::ContainerStart, b"payload1");
        let event2 = Event::new(2, None, 2, EventKind::ServiceDeploy, b"payload2");
        let event5 = Event::new(5, None, 5, EventKind::ContainerStop, b"payload5");
        
        stream.add_event(event1).unwrap();
        stream.add_event(event2).unwrap();
        stream.add_event(event5).unwrap();
        
        // Try to add an event that's too far out of order
        let event_old = Event::new(10, None, 1, EventKind::Custom("old".to_string()), b"old_payload");
        let result = stream.add_event(event_old);
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DockLockError::EventReorderDetected(_)));
    }
    
    #[test]
    fn test_merkle_root_changes() {
        let mut stream = CanonicalEventStream::default();
        
        let event1 = Event::new(1, None, 0, EventKind::ContainerStart, b"payload1");
        stream.add_event(event1).unwrap();
        let root1 = stream.get_merkle_root();
        
        let event2 = Event::new(2, None, 0, EventKind::ServiceDeploy, b"payload2");
        stream.add_event(event2).unwrap();
        let root2 = stream.get_merkle_root();
        
        assert!(root1.is_some());
        assert!(root2.is_some());
        assert_ne!(root1, root2);
    }
    
    #[test]
    fn test_event_lookup() {
        let mut stream = CanonicalEventStream::default();
        
        let event1 = Event::new(1, None, 0, EventKind::ContainerStart, b"payload1");
        let event2 = Event::new(2, None, 0, EventKind::ServiceDeploy, b"payload2");
        
        stream.add_event(event1).unwrap();
        stream.add_event(event2).unwrap();
        
        assert!(stream.get_event(1).is_some());
        assert!(stream.get_event(2).is_some());
        assert!(stream.get_event(3).is_none());
        
        assert_eq!(stream.get_event(1).unwrap().kind, EventKind::ContainerStart);
        assert_eq!(stream.get_event(2).unwrap().kind, EventKind::ServiceDeploy);
    }
    
    #[test]
    fn test_event_range_query() {
        let mut stream = CanonicalEventStream::default();
        
        for i in 1..=5 {
            let event = Event::new(i, None, i as u64, EventKind::ContainerStart, b"payload");
            stream.add_event(event).unwrap();
        }
        
        let range_events = stream.get_events_range(2, 4);
        assert_eq!(range_events.len(), 3);
        assert_eq!(range_events[0].t_seq, 2);
        assert_eq!(range_events[1].t_seq, 3);
        assert_eq!(range_events[2].t_seq, 4);
    }
}
