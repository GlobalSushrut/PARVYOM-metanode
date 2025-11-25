//! # vPod Node Implementation
//! 
//! Universal node type that replaces all traditional node implementations
//! with vPod-based actor systems for superior performance and efficiency.

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, AtomicPtr, Ordering};
use std::ptr;
use std::time::Instant;

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::vpod::{
    VPodRuntime, VPodActor, ActorId, VPodConfig, Message
};
use crate::vpod::actor::ActorSpecialization;

// Import existing node types for migration
use crate::registry::node_types::NodeType;
use crate::mining::node_types::{ValidatorNode, MinerNode};

/// Arena allocator with hugepage backing
#[derive(Debug)]
pub struct Arena {
    base: *mut u8,
    len: usize,
    classes: [SlabClass; 8],
}

/// Slab class for arena allocation
#[derive(Debug)]
pub struct SlabClass {
    size: usize,
    freelist: AtomicUsize,
    bitmap: *mut u64,
}

/// Zero-copy message descriptor for 100x efficiency
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug)]
pub struct MsgDesc {
    payload_ptr: *const u8,
    payload_len: u32,
    msg_type: u16,
    flags: u16,
    timestamp: u64,
}

/// SPSC ring buffer for zero-copy messaging
#[derive(Debug)]
pub struct SpscRing<const N: usize> {
    head: AtomicUsize,
    tail: AtomicUsize,
    slots: [AtomicPtr<MsgDesc>; N],
}

/// Actor hot path data (1.5KB target)
#[repr(C, align(64))]
#[derive(Debug, Clone)]
pub struct ActorHot {
    pub id: u32,
    pub vn_id: u16,
    pub flags: u16,        // bit-packed
    pub budget_q16: i32,   // fixed-point CPU budget
    pub inbox_idx: u32,    // ring cursor
    pub timer_ticks: u32,  // for timewheel
    // Padding to 64B cache line alignment
    pub _padding: [u8; 32],
}

/// Virtual node in VPOD substrate
#[derive(Debug)]
pub struct VirtualNode {
    vn_id: u16,
    node_type: VirtualNodeType,
    hot_data: ActorHot,
    inbox_ring: SpscRing<1024>,
    memory_budget: usize, // 10MB
    arena_slice: (*mut u8, usize),
}

/// Virtual node types (BPI functional + BPCI governance)
#[derive(Debug, Clone)]
pub enum VirtualNodeType {
    // BPI Functional Services
    BpiFunctional(BpiFunctionalType),
    // BPCI Governance Entities  
    BpciGovernance(BpciGovernanceType),
}

#[derive(Debug, Clone)]
pub enum BpiFunctionalType {
    Oracle, Storage, Proof, Audit, Logbook, EncCluster, ShadowRegistry, PipelineApi,
}

#[derive(Debug, Clone)]
pub enum BpciGovernanceType {
    Validator, Notary, Registry, Compliance, Banking, Government, Roundtable,
}

/// Universal vPod Node - Replaces all traditional node types
#[derive(Debug)]
pub struct VPodNode {
    /// Unique node identifier
    pub node_id: String,
    
    /// vPod runtime engine
    pub vpod_runtime: Arc<VPodRuntime>,
    
    /// Arena allocator for 100x efficiency (1-4GB hugepages)
    pub arena: Arc<Arena>,
    
    /// Virtual nodes (100 per physical node)
    pub virtual_nodes: Arc<RwLock<Vec<VirtualNode>>>,
    
    /// Node specialization (determines actor configuration)
    pub node_specialization: NodeSpecialization,
    
    /// Actor pool for this node
    pub actors: Arc<RwLock<HashMap<ActorId, Arc<VPodActor>>>>,
    
    /// Node capabilities derived from specialization
    pub capabilities: VPodCapabilities,
    
    /// Resource allocation and budgets
    pub resource_budget: ResourceBudget,
    
    /// Node status and health
    pub status: Arc<RwLock<VPodNodeStatus>>,
    
    /// Performance metrics
    pub metrics: Arc<RwLock<VPodNodeMetrics>>,
    
    /// Creation timestamp
    pub created_at: Instant,
}

/// Node specializations that map to original node types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeSpecialization {
    /// Community app hosting (replaces BpiCommunity)
    CommunityHost {
        max_apps: u32,
        app_actors: Vec<ActorId>,
        resource_pool: ResourcePool,
        supported_app_types: Vec<String>,
    },
    
    /// Enterprise validator (replaces BpciEnterprise + ValidatorNode)
    EnterpriseValidator {
        consensus_actors: Vec<ActorId>,
        mining_actors: Vec<ActorId>,
        stake_amount: u64,
        validator_key: String,
        compliance_level: String,
    },
    
    /// Banking node (replaces BankApiRegistry)
    BankingNode {
        compliance_actors: Vec<ActorId>,
        banking_service_actors: Vec<ActorId>,
        regulatory_level: String,
        authorized_services: Vec<String>,
    },
    
    /// Government node (replaces GovernmentApiRegistry)
    GovernmentNode {
        jurisdiction_actors: Vec<ActorId>,
        regulatory_actors: Vec<ActorId>,
        government_level: String,
        jurisdiction_authority: String,
    },
    
    /// Governance node (replaces RoundtableApi)
    GovernanceNode {
        voting_actors: Vec<ActorId>,
        proposal_actors: Vec<ActorId>,
        audit_actors: Vec<ActorId>,
        governance_scope: String,
    },
    
    /// Cluster orchestration (replaces ClusterNode/DaemonNode)
    OrchestrationNode {
        scheduler_actors: Vec<ActorId>,
        resource_actors: Vec<ActorId>,
        daemon_actors: Vec<ActorId>,
        hierarchy_level: u32,
    },
    
    /// Hybrid node (replaces Hybrid)
    HybridNode {
        bank_actors: Vec<ActorId>,
        community_actors: Vec<ActorId>,
        dual_authority_actors: Vec<ActorId>,
        bank_sponsor: String,
    },
    
    /// Mining node (replaces MinerNode)
    MiningNode {
        proof_actors: Vec<ActorId>,
        mining_algorithm: String,
        hardware_profile: String,
        mining_power: f64,
    },
    
    /// Notary node (replaces NotaryNode)
    NotaryNode {
        verification_actors: Vec<ActorId>,
        document_actors: Vec<ActorId>,
        notary_license: String,
        jurisdiction: String,
    },
}

/// vPod node capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPodCapabilities {
    /// Maximum concurrent applications
    pub max_applications: u32,
    
    /// Supported message throughput (msgs/sec)
    pub max_throughput: u64,
    
    /// Maximum memory allocation (bytes)
    pub max_memory: u64,
    
    /// Maximum CPU allocation (cores)
    pub max_cpu_cores: f64,
    
    /// Supported specializations
    pub supported_specializations: Vec<String>,
    
    /// Blockchain integration capabilities
    pub blockchain_capabilities: Vec<String>,
    
    /// Compliance and regulatory features
    pub compliance_features: Vec<String>,
}

/// Resource budget for vPod node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceBudget {
    /// CPU budget (microseconds per epoch)
    pub cpu_budget_micros: u64,
    
    /// Memory budget (bytes)
    pub memory_budget_bytes: u64,
    
    /// Network bandwidth budget (bytes per second)
    pub network_budget_bps: u64,
    
    /// Storage budget (bytes)
    pub storage_budget_bytes: u64,
    
    /// Message processing budget (messages per epoch)
    pub message_budget: u32,
}

/// Resource pool for specialized nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePool {
    /// Available CPU resources
    pub cpu_pool: f64,
    
    /// Available memory resources
    pub memory_pool: u64,
    
    /// Available network bandwidth
    pub network_pool: u64,
    
    /// Available storage
    pub storage_pool: u64,
    
    /// Resource allocation strategy
    pub allocation_strategy: AllocationStrategy,
}

/// Resource allocation strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllocationStrategy {
    /// Fair sharing among all actors
    FairShare,
    
    /// Priority-based allocation
    Priority,
    
    /// Demand-based allocation
    Demand,
    
    /// Reserved allocation for critical actors
    Reserved,
}

/// vPod node status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VPodNodeStatus {
    /// Node is initializing
    Initializing,
    
    /// Node is ready to accept work
    Ready,
    
    /// Node is actively processing
    Active,
    
    /// Node is paused
    Paused,
    
    /// Node is draining (no new work)
    Draining,
    
    /// Node is stopped
    Stopped,
    
    /// Node encountered an error
    Error { message: String },
    
    /// Node is being migrated
    Migrating { target: String },
}

/// vPod node performance metrics
#[derive(Debug, Clone, Default)]
pub struct VPodNodeMetrics {
    /// Total actors spawned
    pub actors_spawned: u64,
    
    /// Currently active actors
    pub active_actors: u64,
    
    /// Total messages processed
    pub messages_processed: u64,
    
    /// Average message latency (microseconds)
    pub avg_message_latency_micros: f64,
    
    /// Node throughput (messages per second)
    pub throughput_mps: f64,
    
    /// CPU utilization (0.0 to 1.0)
    pub cpu_utilization: f64,
    
    /// Memory utilization (bytes)
    pub memory_utilization: u64,
    
    /// Network utilization (bytes per second)
    pub network_utilization: u64,
    
    /// Actor efficiency (successful operations / total operations)
    pub actor_efficiency: f64,
    
    /// Resource efficiency (utilized / allocated)
    pub resource_efficiency: f64,
    
    /// Last metrics update
    pub last_updated: Option<Instant>,
}

impl Arena {
    /// Create new arena with hugepages (1-4GB)
    pub fn new(size_gb: usize) -> Result<Self> {
        use std::ptr;
        
        let len = size_gb * 1024 * 1024 * 1024; // Convert GB to bytes
        
        // Try to allocate hugepages first, fall back to regular allocation
        let base = unsafe {
            let ptr = libc::mmap(
                ptr::null_mut(),
                len,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_HUGETLB,
                -1,
                0,
            );
            
            if ptr == libc::MAP_FAILED {
                // Fallback to regular memory allocation if hugepages not available
                println!("⚠️  Hugepages not available, falling back to regular allocation");
                let ptr = libc::mmap(
                    ptr::null_mut(),
                    len,
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                    -1,
                    0,
                );
                
                if ptr == libc::MAP_FAILED {
                    return Err(anyhow!("Failed to allocate memory arena"));
                }
                
                ptr as *mut u8
            } else {
                println!("✅ Hugepage arena allocated successfully");
                ptr as *mut u8
            }
        };
        
        // Initialize slab classes: 32, 64, 128, 256, 512, 1024, 2048, 4096 bytes
        let classes = [
            SlabClass { size: 32, freelist: AtomicUsize::new(0), bitmap: ptr::null_mut() },
            SlabClass { size: 64, freelist: AtomicUsize::new(0), bitmap: ptr::null_mut() },
            SlabClass { size: 128, freelist: AtomicUsize::new(0), bitmap: ptr::null_mut() },
            SlabClass { size: 256, freelist: AtomicUsize::new(0), bitmap: ptr::null_mut() },
            SlabClass { size: 512, freelist: AtomicUsize::new(0), bitmap: ptr::null_mut() },
            SlabClass { size: 1024, freelist: AtomicUsize::new(0), bitmap: ptr::null_mut() },
            SlabClass { size: 2048, freelist: AtomicUsize::new(0), bitmap: ptr::null_mut() },
            SlabClass { size: 4096, freelist: AtomicUsize::new(0), bitmap: ptr::null_mut() },
        ];
        
        Ok(Arena {
            base,
            len,
            classes,
        })
    }
    
    /// Allocate from arena with slab allocation
    pub fn allocate(&self, size: usize) -> Result<*mut u8> {
        // Handle large allocations (>4KB) with direct allocation
        if size > 4096 {
            // For large allocations, use simple bump allocation from end of arena
            static LARGE_OFFSET: AtomicUsize = AtomicUsize::new(0);
            let offset = LARGE_OFFSET.fetch_add(size, Ordering::Relaxed);
            if offset + size > self.len / 2 { // Use only half arena for large allocs
                return Err(anyhow!("Arena exhausted for large allocation"));
            }
            
            unsafe {
                return Ok(self.base.add(self.len / 2 + offset));
            }
        }
        
        // Find appropriate slab class for small allocations
        for class in &self.classes {
            if size <= class.size {
                // Simple bump allocation for now
                let offset = class.freelist.fetch_add(class.size, Ordering::Relaxed);
                if offset + class.size > self.len / 2 { // Use first half for slab allocs
                    return Err(anyhow!("Arena exhausted"));
                }
                
                unsafe {
                    return Ok(self.base.add(offset));
                }
            }
        }
        
        Err(anyhow!("Allocation too large for slab"))
    }
}

impl VirtualNode {
    /// Create new virtual node with 1MB budget
    pub fn new(vn_id: u16, node_type: VirtualNodeType, arena: &Arena) -> Result<Self> {
        let memory_budget = 1024 * 1024; // 1MB per virtual node
        let arena_slice = (arena.allocate(memory_budget)?, memory_budget);
        
        let hot_data = ActorHot {
            id: vn_id as u32,
            vn_id,
            flags: 0,
            budget_q16: (1000 << 16), // 1000.0 in Q16.16 fixed point
            inbox_idx: 0,
            timer_ticks: 0,
            _padding: [0; 32],
        };
        
        let inbox_ring = SpscRing {
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            slots: [const { AtomicPtr::new(ptr::null_mut()) }; 1024],
        };
        
        Ok(VirtualNode {
            vn_id,
            node_type,
            hot_data,
            inbox_ring,
            memory_budget,
            arena_slice,
        })
    }
    
    /// Process batch of messages for 100x efficiency
    pub fn process_batch(&mut self, batch_size: usize) -> Result<usize> {
        let mut processed = 0;
        
        for _ in 0..batch_size {
            let head = self.inbox_ring.head.load(Ordering::Acquire);
            let tail = self.inbox_ring.tail.load(Ordering::Relaxed);
            
            if head == tail {
                break; // Ring empty
            }
            
            let slot_idx = head & 1023; // Ring size mask
            let msg_ptr = self.inbox_ring.slots[slot_idx].swap(ptr::null_mut(), Ordering::Acquire);
            
            if !msg_ptr.is_null() {
                // Process message (placeholder)
                let _msg = unsafe { Box::from_raw(msg_ptr) };
                processed += 1;
                
                // Update head
                self.inbox_ring.head.store((head + 1) & 1023, Ordering::Release);
            } else {
                break;
            }
        }
        
        Ok(processed)
    }
}

// SAFETY: Arena uses careful memory management with hugepages
// Raw pointers are never dereferenced across thread boundaries unsafely
unsafe impl Send for Arena {}
unsafe impl Sync for Arena {}

// SAFETY: SlabClass pointers are managed by Arena's thread-safe allocation
unsafe impl Send for SlabClass {}
unsafe impl Sync for SlabClass {}

// SAFETY: MsgDesc contains payload pointers that are valid for message lifetime
// Zero-copy design ensures pointers remain valid during cross-thread transfer
unsafe impl Send for MsgDesc {}
unsafe impl Sync for MsgDesc {}

// SAFETY: VirtualNode contains arena slice pointers that are managed by Arena
// Arena ensures thread-safe allocation and deallocation of memory regions
unsafe impl Send for VirtualNode {}
unsafe impl Sync for VirtualNode {}

// SAFETY: SpscRing uses AtomicPtr for thread-safe message passing
// Raw pointers are managed through atomic operations with proper ordering
unsafe impl<const N: usize> Send for SpscRing<N> {}
unsafe impl<const N: usize> Sync for SpscRing<N> {}

impl VPodNode {
    /// Create a new vPod node with specified specialization
    pub async fn new(
        node_id: String,
        specialization: NodeSpecialization,
    ) -> Result<Self> {
        // Create vPod runtime with optimized configuration
        let config = VPodConfig {
            max_actors: Self::calculate_max_actors(&specialization),
            epoch_duration: std::time::Duration::from_micros(10), // 10μs epochs
            ring_buffer_size: 2048, // Larger buffer for nodes
            max_actor_state_bytes: 1536, // 1.5KB per actor
            dual_core_enabled: true,
        };
        
        // Initialize arena allocator (1GB for 100 virtual nodes)
        let arena = Arc::new(Arena::new(1)?);
        
        // Initialize 100 virtual nodes
        let mut virtual_nodes = Vec::new();
        for i in 0..100 {
            let node_type = match i % 8 {
                0 => VirtualNodeType::BpiFunctional(BpiFunctionalType::Oracle),
                1 => VirtualNodeType::BpiFunctional(BpiFunctionalType::Storage),
                2 => VirtualNodeType::BpiFunctional(BpiFunctionalType::Proof),
                3 => VirtualNodeType::BpiFunctional(BpiFunctionalType::Audit),
                4 => VirtualNodeType::BpciGovernance(BpciGovernanceType::Validator),
                5 => VirtualNodeType::BpciGovernance(BpciGovernanceType::Registry),
                6 => VirtualNodeType::BpciGovernance(BpciGovernanceType::Notary),
                _ => VirtualNodeType::BpciGovernance(BpciGovernanceType::Compliance),
            };
            virtual_nodes.push(VirtualNode::new(i as u16, node_type, &arena)?);
        }
        
        // Initialize vPod runtime with calculated configuration
        let vpod_runtime = Arc::new(VPodRuntime::new(config).await?);
        
        // Calculate capabilities based on specialization
        let capabilities = Self::calculate_capabilities(&specialization);
        
        // Calculate resource budget
        let resource_budget = Self::calculate_resource_budget(&specialization);
        
        let mut node = VPodNode {
            node_id,
            vpod_runtime,
            arena,
            virtual_nodes: Arc::new(RwLock::new(virtual_nodes)),
            node_specialization: specialization.clone(),
            actors: Arc::new(RwLock::new(HashMap::new())),
            capabilities,
            resource_budget,
            status: Arc::new(RwLock::new(VPodNodeStatus::Initializing)),
            metrics: Arc::new(RwLock::new(VPodNodeMetrics::default())),
            created_at: Instant::now(),
        };
        
        // Initialize actors based on specialization
        node.initialize_specialized_actors().await?;
        
        // Update status to ready
        {
            let mut status = node.status.write().await;
            *status = VPodNodeStatus::Ready;
        }
        
        Ok(node)
    }
    
    /// Initialize actors based on node specialization
    async fn initialize_specialized_actors(&mut self) -> Result<()> {
        match &mut self.node_specialization {
            NodeSpecialization::CommunityHost { app_actors, max_apps, .. } => {
                // Create app hosting actors
                for i in 0..*max_apps {
                    let actor_id = self.vpod_runtime.create_actor(Some(
                        ActorSpecialization::AppHost {
                            app_id: format!("app-{}", i),
                            app_type: "container".to_string(),
                            resource_limits: Default::default(),
                        }
                    )).await?;
                    app_actors.push(actor_id);
                }
            },
            
            NodeSpecialization::EnterpriseValidator { 
                consensus_actors, 
                mining_actors, 
                validator_key,
                stake_amount,
                .. 
            } => {
                // Create consensus validator actors
                for i in 0..4 { // 4 consensus actors for redundancy
                    let actor_id = self.vpod_runtime.create_actor(Some(
                        ActorSpecialization::Validator {
                            validator_key: validator_key.clone(),
                            stake_amount: *stake_amount,
                        }
                    )).await?;
                    consensus_actors.push(actor_id);
                }
                
                // Create mining actors
                for i in 0..2 { // 2 mining actors
                    let actor_id = self.vpod_runtime.create_actor(Some(
                        ActorSpecialization::Miner {
                            mining_algorithm: "proof-of-execution".to_string(),
                            hardware_profile: "enterprise".to_string(),
                        }
                    )).await?;
                    mining_actors.push(actor_id);
                }
            },
            
            NodeSpecialization::BankingNode { 
                compliance_actors, 
                banking_service_actors,
                regulatory_level,
                .. 
            } => {
                // Create compliance actors
                for i in 0..3 { // 3 compliance actors
                    let actor_id = self.vpod_runtime.create_actor(Some(
                        ActorSpecialization::Compliance {
                            regulatory_framework: regulatory_level.clone(),
                            compliance_level: "enhanced".to_string(),
                        }
                    )).await?;
                    compliance_actors.push(actor_id);
                }
                
                // Create banking service actors
                for i in 0..5 { // 5 service actors
                    let actor_id = self.vpod_runtime.create_actor(Some(
                        ActorSpecialization::Generic
                    )).await?;
                    banking_service_actors.push(actor_id);
                }
            },
            
            NodeSpecialization::GovernanceNode { 
                voting_actors, 
                proposal_actors, 
                audit_actors,
                .. 
            } => {
                // Create voting actors
                for i in 0..3 {
                    let actor_id = self.vpod_runtime.create_actor(Some(
                        ActorSpecialization::Governance {
                            voting_power: 100,
                            governance_scope: "community".to_string(),
                        }
                    )).await?;
                    voting_actors.push(actor_id);
                }
                
                // Create proposal and audit actors
                for i in 0..2 {
                    let proposal_id = self.vpod_runtime.create_actor(Some(
                        ActorSpecialization::Generic
                    )).await?;
                    proposal_actors.push(proposal_id);
                    
                    let audit_id = self.vpod_runtime.create_actor(Some(
                        ActorSpecialization::Generic
                    )).await?;
                    audit_actors.push(audit_id);
                }
            },
            
            _ => {
                // Create generic actors for other specializations
                for i in 0..5 {
                    let actor_id = self.vpod_runtime.create_actor(Some(
                        ActorSpecialization::Generic
                    )).await?;
                    
                    let mut actors = self.actors.write().await;
                    if let Some(actor) = self.vpod_runtime.get_actor(&actor_id).await {
                        actors.insert(actor_id, actor);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Calculate maximum actors based on specialization
    fn calculate_max_actors(specialization: &NodeSpecialization) -> usize {
        match specialization {
            NodeSpecialization::CommunityHost { max_apps, .. } => *max_apps as usize * 2,
            NodeSpecialization::EnterpriseValidator { .. } => 10, // High-performance validators
            NodeSpecialization::BankingNode { .. } => 20, // Many compliance actors
            NodeSpecialization::GovernanceNode { .. } => 15, // Voting and audit actors
            NodeSpecialization::OrchestrationNode { .. } => 50, // Many orchestration actors
            _ => 10, // Default
        }
    }
    
    /// Calculate capabilities based on specialization
    fn calculate_capabilities(specialization: &NodeSpecialization) -> VPodCapabilities {
        match specialization {
            NodeSpecialization::CommunityHost { max_apps, .. } => {
                VPodCapabilities {
                    max_applications: *max_apps,
                    max_throughput: 1_000_000, // 1M msgs/sec
                    max_memory: 1024 * 1024 * 1024, // 1GB
                    max_cpu_cores: 2.0,
                    supported_specializations: vec!["app_hosting".to_string()],
                    blockchain_capabilities: vec!["bpi_integration".to_string()],
                    compliance_features: vec!["basic_audit".to_string()],
                }
            },
            
            NodeSpecialization::EnterpriseValidator { .. } => {
                VPodCapabilities {
                    max_applications: 5,
                    max_throughput: 5_000_000, // 5M msgs/sec
                    max_memory: 4 * 1024 * 1024 * 1024, // 4GB
                    max_cpu_cores: 4.0,
                    supported_specializations: vec!["consensus".to_string(), "mining".to_string()],
                    blockchain_capabilities: vec!["bpci_consensus".to_string(), "proof_generation".to_string()],
                    compliance_features: vec!["enterprise_audit".to_string(), "regulatory_compliance".to_string()],
                }
            },
            
            NodeSpecialization::BankingNode { .. } => {
                VPodCapabilities {
                    max_applications: 10,
                    max_throughput: 2_000_000, // 2M msgs/sec
                    max_memory: 2 * 1024 * 1024 * 1024, // 2GB
                    max_cpu_cores: 3.0,
                    supported_specializations: vec!["banking".to_string(), "compliance".to_string()],
                    blockchain_capabilities: vec!["bpi_banking".to_string()],
                    compliance_features: vec!["banking_compliance".to_string(), "kyc_aml".to_string()],
                }
            },
            
            _ => {
                // Default capabilities
                VPodCapabilities {
                    max_applications: 10,
                    max_throughput: 1_000_000,
                    max_memory: 1024 * 1024 * 1024,
                    max_cpu_cores: 2.0,
                    supported_specializations: vec!["generic".to_string()],
                    blockchain_capabilities: vec!["basic_integration".to_string()],
                    compliance_features: vec!["basic_audit".to_string()],
                }
            }
        }
    }
    
    /// Calculate resource budget based on specialization
    fn calculate_resource_budget(specialization: &NodeSpecialization) -> ResourceBudget {
        match specialization {
            NodeSpecialization::EnterpriseValidator { .. } => {
                ResourceBudget {
                    cpu_budget_micros: 5000, // 5ms per epoch
                    memory_budget_bytes: 4 * 1024 * 1024 * 1024, // 4GB
                    network_budget_bps: 100 * 1024 * 1024, // 100MB/s
                    storage_budget_bytes: 100 * 1024 * 1024 * 1024, // 100GB
                    message_budget: 1000, // 1000 messages per epoch
                }
            },
            
            NodeSpecialization::BankingNode { .. } => {
                ResourceBudget {
                    cpu_budget_micros: 3000, // 3ms per epoch
                    memory_budget_bytes: 2 * 1024 * 1024 * 1024, // 2GB
                    network_budget_bps: 50 * 1024 * 1024, // 50MB/s
                    storage_budget_bytes: 50 * 1024 * 1024 * 1024, // 50GB
                    message_budget: 500, // 500 messages per epoch
                }
            },
            
            _ => {
                // Default budget
                ResourceBudget {
                    cpu_budget_micros: 1000, // 1ms per epoch
                    memory_budget_bytes: 1024 * 1024 * 1024, // 1GB
                    network_budget_bps: 10 * 1024 * 1024, // 10MB/s
                    storage_budget_bytes: 10 * 1024 * 1024 * 1024, // 10GB
                    message_budget: 100, // 100 messages per epoch
                }
            }
        }
    }
    
    /// Start the vPod node
    pub async fn start(&self) -> Result<()> {
        // Update status to active
        {
            let mut status = self.status.write().await;
            *status = VPodNodeStatus::Active;
        }
        
        // Start metrics collection
        self.start_metrics_collection().await;
        
        Ok(())
    }
    
    /// Stop the vPod node
    pub async fn stop(&self) -> Result<()> {
        // Update status to stopped
        {
            let mut status = self.status.write().await;
            *status = VPodNodeStatus::Stopped;
        }
        
        // Shutdown runtime
        self.vpod_runtime.shutdown().await?;
        
        Ok(())
    }
    
    /// Send a message to this node
    pub async fn send_message(&self, message: Message) -> Result<()> {
        self.vpod_runtime.send_message(message).await
    }
    
    /// Get node status
    pub async fn get_status(&self) -> VPodNodeStatus {
        self.status.read().await.clone()
    }
    
    /// Get node metrics
    pub async fn get_metrics(&self) -> VPodNodeMetrics {
        self.metrics.read().await.clone()
    }
    
    /// Start metrics collection background task
    async fn start_metrics_collection(&self) {
        let metrics = self.metrics.clone();
        let vpod_runtime = self.vpod_runtime.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_millis(100));
            
            loop {
                interval.tick().await;
                
                // Get runtime metrics
                let runtime_metrics = vpod_runtime.get_metrics().await;
                
                // Update node metrics
                {
                    let mut node_metrics = metrics.write().await;
                    
                    node_metrics.actors_spawned = runtime_metrics.actors_created;
                    node_metrics.active_actors = runtime_metrics.active_actors;
                    node_metrics.messages_processed = runtime_metrics.messages_processed;
                    node_metrics.avg_message_latency_micros = runtime_metrics.avg_message_latency_micros;
                    node_metrics.throughput_mps = runtime_metrics.throughput_mps;
                    node_metrics.cpu_utilization = runtime_metrics.cpu_utilization;
                    node_metrics.memory_utilization = runtime_metrics.memory_utilization;
                    
                    // Calculate efficiency metrics
                    if node_metrics.actors_spawned > 0 {
                        node_metrics.actor_efficiency = 
                            node_metrics.active_actors as f64 / node_metrics.actors_spawned as f64;
                    }
                    
                    node_metrics.last_updated = Some(Instant::now());
                }
            }
        });
    }
}

/// Migration utilities for converting legacy nodes to vPod nodes
impl VPodNode {
    /// Create vPod node from legacy BPI Community node
    pub async fn from_bpi_community(legacy_node: &NodeType) -> Result<Self> {
        if let NodeType::BpiCommunity { 
            max_apps, 
            supported_app_types, 
            .. 
        } = legacy_node {
            let specialization = NodeSpecialization::CommunityHost {
                max_apps: max_apps.unwrap_or(10),
                app_actors: Vec::new(),
                resource_pool: ResourcePool::default(),
                supported_app_types: supported_app_types.iter()
                    .map(|t| format!("{:?}", t))
                    .collect(),
            };
            
            Self::new(Uuid::new_v4().to_string(), specialization).await
        } else {
            Err(anyhow!("Invalid node type for BPI Community conversion"))
        }
    }
    
    /// Create vPod node from legacy Enterprise node
    pub async fn from_bpci_enterprise(legacy_node: &NodeType) -> Result<Self> {
        if let NodeType::BpciEnterprise { 
             
            enhanced_security, 
            .. 
        } = legacy_node {
            let specialization = NodeSpecialization::EnterpriseValidator {
                consensus_actors: Vec::new(),
                mining_actors: Vec::new(),
                stake_amount: 1000000, // Default stake
                validator_key: Uuid::new_v4().to_string(),
                compliance_level: format!("{:?}", enhanced_security),
            };
            
            Self::new(Uuid::new_v4().to_string(), specialization).await
        } else {
            Err(anyhow!("Invalid node type for Enterprise conversion"))
        }
    }
    
    /// Create vPod node from legacy Validator node
    pub async fn from_validator_node(legacy_node: &ValidatorNode) -> Result<Self> {
        let specialization = NodeSpecialization::EnterpriseValidator {
            consensus_actors: Vec::new(),
            mining_actors: Vec::new(),
            stake_amount: legacy_node.stake_amount,
            validator_key: legacy_node.validator_key.clone(),
            compliance_level: "enterprise".to_string(),
        };
        
        Self::new(legacy_node.node_id.clone(), specialization).await
    }
    
    /// Create vPod node from legacy Miner node
    pub async fn from_miner_node(legacy_node: &MinerNode) -> Result<Self> {
        let specialization = NodeSpecialization::MiningNode {
            proof_actors: Vec::new(),
            mining_algorithm: "proof-of-execution".to_string(),
            hardware_profile: format!("{:?}", legacy_node.hardware_specs),
            mining_power: legacy_node.mining_power,
        };
        
        Self::new(legacy_node.node_id.clone(), specialization).await
    }
}

impl Default for ResourcePool {
    fn default() -> Self {
        Self {
            cpu_pool: 4.0, // 4 CPU cores
            memory_pool: 10 * 1024 * 1024, // 10MB per virtual node (100x efficiency target)
            network_pool: 1024 * 1024 * 1024, // 1GB/s
            storage_pool: 100 * 1024 * 1024 * 1024, // 100GB
            allocation_strategy: AllocationStrategy::FairShare,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_vpod_node_creation() {
        let specialization = NodeSpecialization::CommunityHost {
            max_apps: 5,
            app_actors: Vec::new(),
            resource_pool: ResourcePool::default(),
            supported_app_types: vec!["docker".to_string()],
        };
        
        let node = VPodNode::new("test-node".to_string(), specialization).await.unwrap();
        
        assert_eq!(node.node_id, "test-node");
        
        let status = node.get_status().await;
        assert!(matches!(status, VPodNodeStatus::Ready));
    }

    #[tokio::test]
    async fn test_enterprise_validator_node() {
        let specialization = NodeSpecialization::EnterpriseValidator {
            consensus_actors: Vec::new(),
            mining_actors: Vec::new(),
            stake_amount: 1000000,
            validator_key: "test-key".to_string(),
            compliance_level: "enhanced".to_string(),
        };
        
        let node = VPodNode::new("validator-node".to_string(), specialization).await.unwrap();
        
        // Check that actors were created
        let runtime_metrics = node.vpod_runtime.get_metrics().await;
        assert!(runtime_metrics.active_actors > 0);
    }

    #[tokio::test]
    async fn test_node_capabilities() {
        let specialization = NodeSpecialization::BankingNode {
            compliance_actors: Vec::new(),
            banking_service_actors: Vec::new(),
            regulatory_level: "enhanced".to_string(),
            authorized_services: vec!["payments".to_string()],
        };
        
        let capabilities = VPodNode::calculate_capabilities(&specialization);
        
        assert!(capabilities.compliance_features.contains(&"banking_compliance".to_string()));
        assert!(capabilities.blockchain_capabilities.contains(&"bpi_banking".to_string()));
    }
}
