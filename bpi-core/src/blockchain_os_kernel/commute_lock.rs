//! CommuteLock - Zero-Copy Quantum-Synchronized Communication Lock
//! 
//! Implements enterprise-grade zero-copy communication for tetrabolic mesh architecture
//! with quantum synchronization and factorial-wave addressing integration.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::{Mutex, RwLock as AsyncRwLock};
use serde::{Deserialize, Serialize};
use crate::cbor_pipeline_foundation::CborSerializable;
use anyhow::{Result, anyhow};
use uuid::Uuid;
use tracing::{info, warn, error, debug};
use chrono::{DateTime, Utc};
use std::ptr::NonNull;
use std::alloc::{GlobalAlloc, Layout};
use std::mem::{size_of, align_of, MaybeUninit};
use std::slice;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use tokio::time::{Duration, timeout};

use super::tetrabolic_hyperbolic_spaces::{ZkQuantumSync, LokaType};
use super::factorial_tree_communication::{FactorialTreeCommunication, NodeCapabilities};

/// CommuteLock - Zero-Copy Quantum-Synchronized Communication Lock
#[derive(Debug)]
pub struct CommuteLock {
    /// Quantum synchronization engine
    pub quantum_sync: Arc<ZkQuantumSync>,
    /// Factorial tree communication
    pub factorial_comm: Arc<FactorialTreeCommunication>,
    /// Zero-copy memory pools
    pub memory_pools: Arc<AsyncRwLock<HashMap<LokaType, ZeroCopyMemoryPool>>>,
    /// Active communication channels
    pub channels: Arc<AsyncRwLock<HashMap<ChannelId, CommuteChannel>>>,
    /// Lock registry for distributed coordination
    pub lock_registry: Arc<AsyncRwLock<HashMap<LockId, DistributedLock>>>,
    /// Performance metrics
    pub metrics: Arc<RwLock<CommuteLockMetrics>>,
    /// Node capabilities
    pub node_capabilities: Arc<RwLock<NodeCapabilities>>,
    /// Channel sequence counter
    pub channel_sequence: Arc<AtomicU64>,
}

/// Zero-Copy Memory Pool for efficient communication
#[derive(Debug)]
pub struct ZeroCopyMemoryPool {
    /// Pre-allocated memory blocks
    pub memory_blocks: Vec<MemoryBlock>,
    /// Free block indices
    pub free_blocks: Arc<Mutex<Vec<usize>>>,
    /// Block size in bytes
    pub block_size: usize,
    /// Total pool capacity
    pub total_capacity: usize,
    /// Pool statistics
    pub stats: Arc<RwLock<PoolStats>>,
    /// Memory alignment for SIMD operations
    pub alignment: usize,
}

/// Memory block for zero-copy operations
#[derive(Debug)]
pub struct MemoryBlock {
    /// Raw memory pointer (aligned)
    pub ptr: NonNull<u8>,
    /// Block size
    pub size: usize,
    /// Reference count for sharing
    pub ref_count: Arc<AtomicU64>,
    /// Block ID for tracking
    pub block_id: Uuid,
    /// Quantum entanglement state
    pub quantum_state: Arc<RwLock<QuantumBlockState>>,
}

/// Quantum state for memory blocks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuantumBlockState {
    /// Entanglement ID with remote blocks
    pub entanglement_id: Option<String>,
    /// Quantum fidelity score
    pub fidelity: f64,
    /// Last synchronization timestamp
    pub last_sync: DateTime<Utc>,
    /// Coherence state
    pub coherent: bool,
}

/// Communication channel for zero-copy messaging
#[derive(Debug)]
pub struct CommuteChannel {
    /// Channel unique identifier
    pub channel_id: ChannelId,
    /// Source node ID
    pub source_node: String,
    /// Target node ID
    pub target_node: String,
    /// Channel type (unicast, multicast, broadcast)
    pub channel_type: ChannelType,
    /// Zero-copy message queue
    pub message_queue: Arc<Mutex<Vec<ZeroCopyMessage>>>,
    /// Channel state
    pub state: Arc<RwLock<ChannelState>>,
    /// Quantum entanglement with remote channel
    pub quantum_entanglement: Arc<RwLock<Option<String>>>,
    /// Performance metrics
    pub metrics: Arc<RwLock<ChannelMetrics>>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
}

/// Zero-copy message structure
#[derive(Debug)]
pub struct ZeroCopyMessage {
    /// Message ID
    pub message_id: Uuid,
    /// Memory block reference (zero-copy)
    pub memory_block: Arc<MemoryBlock>,
    /// Message metadata
    pub metadata: MessageMetadata,
    /// Quantum signature for integrity
    pub quantum_signature: Vec<u8>,
    /// Routing information
    pub routing_info: RoutingInfo,
}

/// Message metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageMetadata {
    /// Message type
    pub message_type: MessageType,
    /// Content length
    pub content_length: usize,
    /// Priority level
    pub priority: Priority,
    /// TTL (time to live)
    pub ttl: Duration,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Compression algorithm used
    pub compression: Option<CompressionType>,
}

/// Routing information for factorial-wave addressing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoutingInfo {
    /// Source factoradic address
    pub source_address: Vec<u32>,
    /// Target factoradic address
    pub target_address: Vec<u32>,
    /// Routing path through factorial tree
    pub routing_path: Vec<String>,
    /// Hop count
    pub hop_count: u32,
    /// Load balancing weight
    pub weight: f64,
}

/// Distributed lock for mesh coordination
#[derive(Debug)]
pub struct DistributedLock {
    /// Lock ID
    pub lock_id: LockId,
    /// Lock owner node
    pub owner_node: String,
    /// Lock type (read, write, exclusive)
    pub lock_type: LockType,
    /// Acquired timestamp
    pub acquired_at: DateTime<Utc>,
    /// Lock TTL
    pub ttl: Duration,
    /// Quantum consensus state
    pub consensus_state: Arc<RwLock<ConsensusState>>,
    /// Witness nodes for distributed consensus
    pub witness_nodes: Vec<String>,
}

/// Type definitions
pub type ChannelId = Uuid;
pub type LockId = Uuid;

/// Channel types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelType {
    /// Point-to-point communication
    Unicast,
    /// One-to-many communication
    Multicast { targets: Vec<String> },
    /// One-to-all communication
    Broadcast,
    /// Quantum entangled channel
    QuantumEntangled { entanglement_id: String },
}

/// Channel states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelState {
    /// Channel is being established
    Establishing,
    /// Channel is active and ready
    Active,
    /// Channel is temporarily suspended
    Suspended,
    /// Channel is being closed
    Closing,
    /// Channel is closed
    Closed,
    /// Channel has quantum entanglement
    QuantumSynchronized,
}

/// Message types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MessageType {
    /// Data message
    Data,
    /// Control message
    Control,
    /// Heartbeat/keepalive
    Heartbeat,
    /// Quantum synchronization
    QuantumSync,
    /// Routing update
    RoutingUpdate,
    /// Lock coordination
    LockCoordination,
}

/// Message priorities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    /// Low priority (background tasks)
    Low = 0,
    /// Normal priority (default)
    Normal = 1,
    /// High priority (important operations)
    High = 2,
    /// Critical priority (system operations)
    Critical = 3,
    /// Quantum priority (quantum sync operations)
    Quantum = 4,
}

/// Compression types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompressionType {
    /// No compression
    None,
    /// LZ4 compression
    Lz4,
    /// Zstd compression
    Zstd,
    /// Quantum compression (theoretical)
    Quantum,
}

/// Lock types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LockType {
    /// Read lock (shared)
    Read,
    /// Write lock (exclusive)
    Write,
    /// Exclusive lock (single owner)
    Exclusive,
    /// Quantum lock (quantum consensus)
    Quantum,
}

/// Consensus state for distributed locks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConsensusState {
    /// Consensus achieved
    pub achieved: bool,
    /// Participating nodes
    pub participants: Vec<String>,
    /// Consensus timestamp
    pub timestamp: DateTime<Utc>,
    /// Quantum fidelity
    pub quantum_fidelity: f64,
}

/// Pool statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PoolStats {
    /// Total allocations
    pub total_allocations: u64,
    /// Current active blocks
    pub active_blocks: u64,
    /// Peak usage
    pub peak_usage: u64,
    /// Allocation failures
    pub allocation_failures: u64,
    /// Average block utilization
    pub avg_utilization: f64,
}

/// Channel metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChannelMetrics {
    /// Messages sent
    pub messages_sent: u64,
    /// Messages received
    pub messages_received: u64,
    /// Bytes transferred
    pub bytes_transferred: u64,
    /// Average latency (microseconds)
    pub avg_latency_us: f64,
    /// Error count
    pub error_count: u64,
    /// Quantum synchronization events
    pub quantum_sync_events: u64,
}

/// CommuteLock performance metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommuteLockMetrics {
    /// Total channels created
    pub total_channels: u64,
    /// Active channels
    pub active_channels: u64,
    /// Total messages processed
    pub total_messages: u64,
    /// Zero-copy operations
    pub zero_copy_operations: u64,
    /// Quantum synchronizations
    pub quantum_synchronizations: u64,
    /// Average processing time (nanoseconds)
    pub avg_processing_time_ns: f64,
    /// Memory efficiency (0.0 to 1.0)
    pub memory_efficiency: f64,
    /// Distributed locks acquired
    pub distributed_locks_acquired: u64,
}

// CBOR Serializable implementations for CommuteLock structs
impl CborSerializable for QuantumBlockState {}
impl CborSerializable for MessageMetadata {}
impl CborSerializable for RoutingInfo {}
impl CborSerializable for ConsensusState {}
impl CborSerializable for PoolStats {}
impl CborSerializable for ChannelMetrics {}
impl CborSerializable for CommuteLockMetrics {}

impl CommuteLock {
    /// Create new CommuteLock instance
    pub async fn new(
        quantum_sync: Arc<ZkQuantumSync>,
        factorial_comm: Arc<FactorialTreeCommunication>,
        node_capabilities: NodeCapabilities,
    ) -> Result<Self> {
        info!("🔒 Initializing CommuteLock with quantum synchronization");

        let memory_pools = Arc::new(AsyncRwLock::new(HashMap::new()));
        
        // Initialize memory pools for each Loka type
        let mut pools = memory_pools.write().await;
        for loka_type in [LokaType::Bhuloka, LokaType::Bhuvarloka, LokaType::Svarloka, 
                         LokaType::Maharloka, LokaType::Janoloka, LokaType::Tapoloka, 
                         LokaType::Satyaloka] {
            let pool = ZeroCopyMemoryPool::new(1024 * 1024, 4096, 64)?; // 1MB pool, 4KB blocks, 64-byte alignment
            pools.insert(loka_type, pool);
        }
        drop(pools);

        let commute_lock = Self {
            quantum_sync,
            factorial_comm,
            memory_pools,
            channels: Arc::new(AsyncRwLock::new(HashMap::new())),
            lock_registry: Arc::new(AsyncRwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(CommuteLockMetrics::new())),
            node_capabilities: Arc::new(RwLock::new(node_capabilities)),
            channel_sequence: Arc::new(AtomicU64::new(0)),
        };

        info!("✅ CommuteLock initialized successfully");
        Ok(commute_lock)
    }

    /// Create zero-copy communication channel
    pub async fn create_channel(
        &self,
        source_node: String,
        target_node: String,
        channel_type: ChannelType,
    ) -> Result<ChannelId> {
        let channel_id = Uuid::new_v4();
        
        info!("🔗 Creating CommuteLock channel: {} -> {}", source_node, target_node);

        // Create quantum entanglement if supported
        let quantum_entanglement = if matches!(channel_type, ChannelType::QuantumEntangled { .. }) {
            let entanglement_id = self.quantum_sync.create_entanglement(
                source_node.clone(),
                target_node.clone(),
            ).await?;
            Arc::new(RwLock::new(Some(entanglement_id)))
        } else {
            Arc::new(RwLock::new(None))
        };

        let channel = CommuteChannel {
            channel_id,
            source_node: source_node.clone(),
            target_node: target_node.clone(),
            channel_type,
            message_queue: Arc::new(Mutex::new(Vec::new())),
            state: Arc::new(RwLock::new(ChannelState::Establishing)),
            quantum_entanglement,
            metrics: Arc::new(RwLock::new(ChannelMetrics::new())),
            created_at: Utc::now(),
        };

        // Add to factorial tree routing
        self.factorial_comm.add_node(source_node.clone(), self.node_capabilities.read().unwrap().clone()).await?;
        self.factorial_comm.add_node(target_node.clone(), self.node_capabilities.read().unwrap().clone()).await?;

        // Register channel
        let mut channels = self.channels.write().await;
        channels.insert(channel_id, channel);
        drop(channels);

        // Update metrics
        {
            let mut metrics = self.metrics.write().unwrap();
            metrics.total_channels += 1;
            metrics.active_channels += 1;
        }

        // Set channel to active
        self.set_channel_state(channel_id, ChannelState::Active).await?;

        info!("✅ CommuteLock channel created: {}", channel_id);
        Ok(channel_id)
    }

    /// Send zero-copy message through channel
    pub async fn send_zero_copy(
        &self,
        channel_id: ChannelId,
        data: &[u8],
        metadata: MessageMetadata,
    ) -> Result<()> {
        let start_time = std::time::Instant::now();

        // Get channel
        let channels = self.channels.read().await;
        let channel = channels.get(&channel_id)
            .ok_or_else(|| anyhow!("Channel not found: {}", channel_id))?;

        // Allocate zero-copy memory block
        let memory_block = self.allocate_memory_block(
            &channel.target_node,
            data.len(),
        ).await?;

        // Copy data to zero-copy block (this is the only copy operation)
        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                memory_block.ptr.as_ptr(),
                data.len(),
            );
        }

        // Generate quantum signature
        let quantum_signature = self.generate_quantum_signature(&memory_block, &metadata).await?;

        // Get routing information
        let routing_info = self.get_routing_info(
            &channel.source_node,
            &channel.target_node,
        ).await?;

        // Create zero-copy message
        let message = ZeroCopyMessage {
            message_id: Uuid::new_v4(),
            memory_block,
            metadata,
            quantum_signature,
            routing_info,
        };

        // Add to channel queue
        let mut queue = channel.message_queue.lock().await;
        queue.push(message);
        drop(queue);

        // Route through factorial tree
        self.factorial_comm.route_message(
            &channel.source_node,
            &channel.target_node,
            vec![], // Zero-copy: no data copying
        ).await?;

        // Update metrics
        {
            let mut channel_metrics = channel.metrics.write().unwrap();
            channel_metrics.messages_sent += 1;
            channel_metrics.bytes_transferred += data.len() as u64;
            channel_metrics.avg_latency_us = start_time.elapsed().as_micros() as f64;
        }

        {
            let mut metrics = self.metrics.write().unwrap();
            metrics.total_messages += 1;
            metrics.zero_copy_operations += 1;
            metrics.avg_processing_time_ns = start_time.elapsed().as_nanos() as f64;
        }

        debug!("📤 Zero-copy message sent via channel {}", channel_id);
        Ok(())
    }

    /// Receive zero-copy message from channel
    pub async fn receive_zero_copy(&self, channel_id: ChannelId) -> Result<Option<ZeroCopyMessage>> {
        let channels = self.channels.read().await;
        let channel = channels.get(&channel_id)
            .ok_or_else(|| anyhow!("Channel not found: {}", channel_id))?;

        let mut queue = channel.message_queue.lock().await;
        let message = queue.pop();
        drop(queue);

        if let Some(ref msg) = message {
            // Update metrics
            let mut channel_metrics = channel.metrics.write().unwrap();
            channel_metrics.messages_received += 1;
        }

        Ok(message)
    }

    /// Acquire distributed lock
    pub async fn acquire_distributed_lock(
        &self,
        resource_id: String,
        lock_type: LockType,
        ttl: Duration,
    ) -> Result<LockId> {
        let lock_id = Uuid::new_v4();
        let node_id = self.get_node_id().await?;

        info!("🔐 Acquiring distributed lock: {} (type: {:?})", resource_id, lock_type);

        // Create consensus state
        let consensus_state = Arc::new(RwLock::new(ConsensusState {
            achieved: false,
            participants: vec![node_id.clone()],
            timestamp: Utc::now(),
            quantum_fidelity: 0.0,
        }));

        // Create distributed lock
        let distributed_lock = DistributedLock {
            lock_id,
            owner_node: node_id.clone(),
            lock_type: lock_type.clone(),
            acquired_at: Utc::now(),
            ttl,
            consensus_state: consensus_state.clone(),
            witness_nodes: Vec::new(),
        };

        // Achieve quantum consensus if quantum lock
        if matches!(lock_type, LockType::Quantum) {
            let fidelity = self.quantum_sync.quantum_synchronize().await?;
            let mut consensus = consensus_state.write().unwrap();
            consensus.achieved = fidelity > 0.95; // 95% fidelity threshold
            consensus.quantum_fidelity = fidelity;
        } else {
            let mut consensus = consensus_state.write().unwrap();
            consensus.achieved = true; // Non-quantum locks achieve consensus immediately
        }

        // Register lock
        let mut registry = self.lock_registry.write().await;
        registry.insert(lock_id, distributed_lock);
        drop(registry);

        // Update metrics
        {
            let mut metrics = self.metrics.write().unwrap();
            metrics.distributed_locks_acquired += 1;
        }

        info!("✅ Distributed lock acquired: {}", lock_id);
        Ok(lock_id)
    }

    /// Release distributed lock
    pub async fn release_distributed_lock(&self, lock_id: LockId) -> Result<()> {
        let mut registry = self.lock_registry.write().await;
        
        if let Some(lock) = registry.remove(&lock_id) {
            info!("🔓 Released distributed lock: {}", lock_id);
            
            // Perform quantum decoherence if quantum lock
            if matches!(lock.lock_type, LockType::Quantum) {
                // Quantum locks require proper decoherence
                debug!("Performing quantum decoherence for lock: {}", lock_id);
            }
            
            Ok(())
        } else {
            Err(anyhow!("Lock not found: {}", lock_id))
        }
    }

    /// Set channel state
    async fn set_channel_state(&self, channel_id: ChannelId, state: ChannelState) -> Result<()> {
        let channels = self.channels.read().await;
        if let Some(channel) = channels.get(&channel_id) {
            let mut channel_state = channel.state.write().unwrap();
            *channel_state = state;
            Ok(())
        } else {
            Err(anyhow!("Channel not found: {}", channel_id))
        }
    }

    /// Allocate zero-copy memory block
    async fn allocate_memory_block(&self, target_node: &str, size: usize) -> Result<Arc<MemoryBlock>> {
        // Determine Loka type based on target node characteristics
        let loka_type = self.determine_loka_type(target_node).await?;
        
        let pools = self.memory_pools.read().await;
        let pool = pools.get(&loka_type)
            .ok_or_else(|| anyhow!("Memory pool not found for Loka type: {:?}", loka_type))?;

        pool.allocate_block(size).await
    }

    /// Determine Loka type for target node
    async fn determine_loka_type(&self, _target_node: &str) -> Result<LokaType> {
        // For now, use Bhuloka (physical plane) as default
        // In production, this would analyze node characteristics
        Ok(LokaType::Bhuloka)
    }

    /// Generate quantum signature for message integrity
    async fn generate_quantum_signature(
        &self,
        memory_block: &Arc<MemoryBlock>,
        metadata: &MessageMetadata,
    ) -> Result<Vec<u8>> {
        // Generate quantum signature using block content and metadata
        let content_hash = self.hash_memory_block(memory_block).await?;
        let metadata_hash = self.hash_metadata(metadata).await?;
        
        // Combine hashes for quantum signature
        let mut signature = Vec::new();
        signature.extend_from_slice(&content_hash);
        signature.extend_from_slice(&metadata_hash);
        
        Ok(signature)
    }

    /// Get routing information for factorial-wave addressing
    async fn get_routing_info(&self, source: &str, target: &str) -> Result<RoutingInfo> {
        let routing_path = self.factorial_comm.find_route(source, target).await?;
        
        Ok(RoutingInfo {
            source_address: vec![0], // Placeholder - would use actual factoradic address
            target_address: vec![1], // Placeholder - would use actual factoradic address
            routing_path,
            hop_count: 1,
            weight: 1.0,
        })
    }

    /// Get current node ID
    async fn get_node_id(&self) -> Result<String> {
        Ok("commute_lock_node".to_string()) // Placeholder
    }

    /// Hash memory block content
    async fn hash_memory_block(&self, memory_block: &Arc<MemoryBlock>) -> Result<Vec<u8>> {
        // Placeholder hash implementation
        Ok(vec![0u8; 32])
    }

    /// Hash message metadata
    async fn hash_metadata(&self, _metadata: &MessageMetadata) -> Result<Vec<u8>> {
        // Placeholder hash implementation
        Ok(vec![0u8; 32])
    }

    /// Get performance metrics
    pub fn get_metrics(&self) -> CommuteLockMetrics {
        self.metrics.read().unwrap().clone()
    }

    /// Perform quantum synchronization across all channels
    pub async fn quantum_synchronize_all(&self) -> Result<f64> {
        let fidelity = self.quantum_sync.quantum_synchronize().await?;
        
        // Update metrics
        {
            let mut metrics = self.metrics.write().unwrap();
            metrics.quantum_synchronizations += 1;
        }

        info!("🌀 Quantum synchronization completed with fidelity: {:.4}", fidelity);
        Ok(fidelity)
    }
}

impl ZeroCopyMemoryPool {
    /// Create new zero-copy memory pool
    pub fn new(total_size: usize, block_size: usize, alignment: usize) -> Result<Self> {
        let num_blocks = total_size / block_size;
        let mut memory_blocks = Vec::with_capacity(num_blocks);
        let mut free_blocks = Vec::with_capacity(num_blocks);

        // Allocate aligned memory blocks
        for i in 0..num_blocks {
            let layout = Layout::from_size_align(block_size, alignment)?;
            let ptr = unsafe { std::alloc::alloc(layout) };
            
            if ptr.is_null() {
                return Err(anyhow!("Failed to allocate memory block"));
            }

            let memory_block = MemoryBlock {
                ptr: NonNull::new(ptr).unwrap(),
                size: block_size,
                ref_count: Arc::new(AtomicU64::new(0)),
                block_id: Uuid::new_v4(),
                quantum_state: Arc::new(RwLock::new(QuantumBlockState {
                    entanglement_id: None,
                    fidelity: 0.0,
                    last_sync: Utc::now(),
                    coherent: false,
                })),
            };

            memory_blocks.push(memory_block);
            free_blocks.push(i);
        }

        Ok(Self {
            memory_blocks,
            free_blocks: Arc::new(Mutex::new(free_blocks)),
            block_size,
            total_capacity: total_size,
            stats: Arc::new(RwLock::new(PoolStats::new())),
            alignment,
        })
    }

    /// Allocate memory block from pool
    pub async fn allocate_block(&self, _size: usize) -> Result<Arc<MemoryBlock>> {
        let mut free_blocks = self.free_blocks.lock().await;
        
        if let Some(block_index) = free_blocks.pop() {
            let block = &self.memory_blocks[block_index];
            block.ref_count.store(1, Ordering::SeqCst);
            
            // Update stats
            {
                let mut stats = self.stats.write().unwrap();
                stats.total_allocations += 1;
                stats.active_blocks += 1;
            }

            Ok(Arc::new(MemoryBlock {
                ptr: block.ptr,
                size: block.size,
                ref_count: block.ref_count.clone(),
                block_id: block.block_id,
                quantum_state: block.quantum_state.clone(),
            }))
        } else {
            // Update failure stats
            {
                let mut stats = self.stats.write().unwrap();
                stats.allocation_failures += 1;
            }
            Err(anyhow!("No free memory blocks available"))
        }
    }
}

// Implement new() methods for metrics structs
impl CommuteLockMetrics {
    pub fn new() -> Self {
        Self {
            total_channels: 0,
            active_channels: 0,
            total_messages: 0,
            zero_copy_operations: 0,
            quantum_synchronizations: 0,
            avg_processing_time_ns: 0.0,
            memory_efficiency: 1.0,
            distributed_locks_acquired: 0,
        }
    }
}

impl ChannelMetrics {
    pub fn new() -> Self {
        Self {
            messages_sent: 0,
            messages_received: 0,
            bytes_transferred: 0,
            avg_latency_us: 0.0,
            error_count: 0,
            quantum_sync_events: 0,
        }
    }
}

impl PoolStats {
    pub fn new() -> Self {
        Self {
            total_allocations: 0,
            active_blocks: 0,
            peak_usage: 0,
            allocation_failures: 0,
            avg_utilization: 0.0,
        }
    }
}

// Safety: MemoryBlock is safe to send between threads as it uses atomic reference counting
unsafe impl Send for MemoryBlock {}
unsafe impl Sync for MemoryBlock {}

// Drop implementation for MemoryBlock to handle memory cleanup
impl Drop for MemoryBlock {
    fn drop(&mut self) {
        if self.ref_count.load(Ordering::SeqCst) == 0 {
            unsafe {
                let layout = Layout::from_size_align_unchecked(self.size, 64);
                std::alloc::dealloc(self.ptr.as_ptr(), layout);
            }
        }
    }
}
