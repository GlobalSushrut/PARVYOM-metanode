//! # vPod Actor Implementation
//! 
//! Lightweight actors with ≤1.5KB state and ring buffer communication.
//! Each actor represents a minimal computational unit that can be scheduled
//! on dual cores with microsecond precision.

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::vpod::ring_buffer::SPSCRingBuffer;

/// Actor identifier type
pub type ActorId = String;

/// Actor message type alias
pub type ActorMessage = Message;

/// Actor command type alias  
pub type ActorCommand = ControlMessage;

/// Maximum actor state size (1.5KB as per vPod specification)
pub const MAX_ACTOR_STATE_SIZE: usize = 1536;

/// vPod Actor - Lightweight computational unit
#[derive(Debug)]
pub struct VPodActor {
    /// Unique actor identifier
    pub id: ActorId,
    
    /// Actor state (limited to 1.5KB)
    pub state: ActorState,
    
    /// Inbox ring buffer for incoming messages
    pub inbox: Arc<SPSCRingBuffer<Message>>,
    
    /// Outbox ring buffer for outgoing messages
    pub outbox: Arc<SPSCRingBuffer<Message>>,
    
    /// Actor budget for resource management
    pub budget: Arc<RwLock<ActorBudget>>,
    
    /// Actor specialization (determines behavior)
    pub specialization: Option<ActorSpecialization>,
    
    /// Performance metrics
    pub metrics: Arc<RwLock<ActorMetrics>>,
    
    /// Actor status
    pub status: Arc<RwLock<ActorStatus>>,
    
    /// Creation timestamp
    pub created_at: Instant,
}

/// Actor state container (≤1.5KB)
#[derive(Debug)]
pub struct ActorState {
    /// Raw state data (maximum 1536 bytes)
    data: Vec<u8>,
    
    /// State version for optimistic concurrency
    version: AtomicU64,
    
    /// State type identifier
    state_type: String,
}

/// Message passed between actors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message identifier
    pub id: String,
    
    /// Source actor ID
    pub from: ActorId,
    
    /// Destination actor ID
    pub to: ActorId,
    
    /// Message payload
    pub payload: MessagePayload,
    
    /// Message priority (0 = highest, 255 = lowest)
    pub priority: u8,
    
    /// Creation timestamp (microseconds since epoch)
    pub timestamp: u64,
    
    /// Message size in bytes
    pub size: u32,
}

/// Message payload types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePayload {
    /// Raw bytes
    Raw(Vec<u8>),
    
    /// JSON data
    Json(serde_json::Value),
    
    /// Text message
    Text(String),
    
    /// Control message
    Control(ControlMessage),
    
    /// Application-specific message
    Application {
        app_type: String,
        data: Vec<u8>,
    },
}

/// Control messages for actor lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlMessage {
    /// Start actor processing
    Start,
    
    /// Stop actor processing
    Stop,
    
    /// Pause actor (can be resumed)
    Pause,
    
    /// Resume paused actor
    Resume,
    
    /// Migrate actor to different core/node
    Migrate { target_core: u32 },
    
    /// Update actor budget
    UpdateBudget { new_budget: ActorBudget },
    
    /// Health check ping
    Ping,
    
    /// Health check response
    Pong,
}

/// Actor resource budget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorBudget {
    /// CPU time budget (microseconds per epoch)
    pub cpu_micros: u64,
    
    /// Memory budget (bytes)
    pub memory_bytes: u64,
    
    /// Network bandwidth budget (bytes per epoch)
    pub network_bytes: u64,
    
    /// Message count budget (messages per epoch)
    pub message_count: u32,
    
    /// Budget expiration time (as timestamp)
    #[serde(skip)]
    pub expires_at: Option<Instant>,
}

/// Actor performance metrics
#[derive(Debug, Clone, Default)]
pub struct ActorMetrics {
    /// Total messages processed
    pub messages_processed: u64,
    
    /// Total CPU time used (microseconds)
    pub cpu_time_used: u64,
    
    /// Total memory allocated (bytes)
    pub memory_used: u64,
    
    /// Average message processing latency (microseconds)
    pub avg_latency_micros: f64,
    
    /// Messages per second throughput
    pub throughput_mps: f64,
    
    /// Error count
    pub error_count: u64,
    
    /// Last update timestamp (as timestamp)
    pub last_updated: Option<u64>,
}

/// Actor status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActorStatus {
    /// Actor is initializing
    Initializing,
    
    /// Actor is ready to process messages
    Ready,
    
    /// Actor is actively processing
    Processing,
    
    /// Actor is paused
    Paused,
    
    /// Actor is stopped
    Stopped,
    
    /// Actor encountered an error
    Error { message: String },
    
    /// Actor is being migrated
    Migrating { target: String },
}

/// Actor specialization types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActorSpecialization {
    /// Generic actor (no specialization)
    Generic,
    
    /// Application hosting actor
    AppHost {
        app_id: String,
        app_type: String,
        resource_limits: ResourceLimits,
    },
    
    /// Consensus validator actor
    Validator {
        validator_key: String,
        stake_amount: u64,
    },
    
    /// Mining actor
    Miner {
        mining_algorithm: String,
        hardware_profile: String,
    },
    
    /// Banking compliance actor
    Compliance {
        regulatory_framework: String,
        compliance_level: String,
    },
    
    /// Governance voting actor
    Governance {
        voting_power: u32,
        governance_scope: String,
    },
    
    /// Resource management actor
    ResourceManager {
        managed_resources: Vec<String>,
        allocation_strategy: String,
    },
}

/// Resource limits for specialized actors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum CPU usage (percentage)
    pub max_cpu_percent: f64,
    
    /// Maximum memory usage (bytes)
    pub max_memory_bytes: u64,
    
    /// Maximum network bandwidth (bytes/sec)
    pub max_network_bps: u64,
    
    /// Maximum file descriptors
    pub max_file_descriptors: u32,
}

impl VPodActor {
    /// Create a new vPod actor
    pub fn new(id: ActorId, ring_buffer_size: usize) -> Result<Self> {
        let inbox = Arc::new(SPSCRingBuffer::new(ring_buffer_size)?);
        let outbox = Arc::new(SPSCRingBuffer::new(ring_buffer_size)?);
        
        Ok(VPodActor {
            id,
            state: ActorState::new("generic".to_string()),
            inbox,
            outbox,
            budget: Arc::new(RwLock::new(ActorBudget::default())),
            specialization: None,
            metrics: Arc::new(RwLock::new(ActorMetrics::default())),
            status: Arc::new(RwLock::new(ActorStatus::Initializing)),
            created_at: Instant::now(),
        })
    }
    
    /// Send a message to this actor
    pub async fn send_message(&self, message: Message) -> Result<()> {
        self.inbox.push(message)
            .map_err(|_| anyhow!("Actor inbox full"))?;
        Ok(())
    }
    
    /// Receive a message from this actor's outbox
    pub async fn receive_message(&self) -> Option<Message> {
        self.outbox.pop()
    }
    
    /// Process messages in the actor's inbox
    pub async fn process_messages(&self, max_messages: u32) -> Result<u32> {
        let mut processed = 0;
        let start_time = Instant::now();
        
        while processed < max_messages {
            if let Some(message) = self.inbox.pop() {
                self.process_single_message(message).await?;
                processed += 1;
            } else {
                break;
            }
        }
        
        // Update metrics
        let processing_time = start_time.elapsed();
        self.update_metrics(processed, processing_time).await;
        
        Ok(processed)
    }
    
    /// Process a single message
    async fn process_single_message(&self, message: Message) -> Result<()> {
        let start_time = Instant::now();
        
        match &message.payload {
            MessagePayload::Control(control) => {
                self.handle_control_message(control.clone()).await?;
            },
            MessagePayload::Application { app_type, data } => {
                self.handle_application_message(app_type, data).await?;
            },
            _ => {
                // Generic message processing
                self.handle_generic_message(message).await?;
            }
        }
        
        let latency = start_time.elapsed();
        self.record_message_latency(latency).await;
        
        Ok(())
    }
    
    /// Handle control messages
    async fn handle_control_message(&self, control: ControlMessage) -> Result<()> {
        let mut status = self.status.write().await;
        
        match control {
            ControlMessage::Start => {
                *status = ActorStatus::Ready;
            },
            ControlMessage::Stop => {
                *status = ActorStatus::Stopped;
            },
            ControlMessage::Pause => {
                *status = ActorStatus::Paused;
            },
            ControlMessage::Resume => {
                *status = ActorStatus::Ready;
            },
            ControlMessage::Ping => {
                // Send pong response
                let pong = Message::new_control(
                    self.id.clone(),
                    self.id.clone(), // Self-response for now
                    ControlMessage::Pong
                );
                let _ = self.outbox.push(pong);
            },
            _ => {
                // Handle other control messages
            }
        }
        
        Ok(())
    }
    
    /// Handle application-specific messages
    async fn handle_application_message(&self, _app_type: &str, _data: &[u8]) -> Result<()> {
        // Application-specific processing based on actor specialization
        if let Some(specialization) = &self.specialization {
            match specialization {
                ActorSpecialization::AppHost { .. } => {
                    // Handle app hosting messages
                },
                ActorSpecialization::Validator { .. } => {
                    // Handle consensus validation messages
                },
                ActorSpecialization::Miner { .. } => {
                    // Handle mining messages
                },
                _ => {
                    // Handle other specializations
                }
            }
        }
        
        Ok(())
    }
    
    /// Handle generic messages
    async fn handle_generic_message(&self, _message: Message) -> Result<()> {
        // Default message processing
        Ok(())
    }
    
    /// Update actor performance metrics
    async fn update_metrics(&self, messages_processed: u32, processing_time: Duration) {
        let mut metrics = self.metrics.write().await;
        
        metrics.messages_processed += messages_processed as u64;
        
        let processing_micros = processing_time.as_micros() as u64;
        metrics.cpu_time_used += processing_micros;
        
        if messages_processed > 0 {
            let avg_latency = processing_micros as f64 / messages_processed as f64;
            metrics.avg_latency_micros = 
                (metrics.avg_latency_micros * 0.9) + (avg_latency * 0.1); // EMA
        }
        
        // Calculate throughput (messages per second)
        if processing_time.as_secs_f64() > 0.0 {
            let current_throughput = messages_processed as f64 / processing_time.as_secs_f64();
            metrics.throughput_mps = 
                (metrics.throughput_mps * 0.9) + (current_throughput * 0.1); // EMA
        }
        
        metrics.last_updated = Some(chrono::Utc::now().timestamp_micros() as u64);
    }
    
    /// Record message processing latency
    async fn record_message_latency(&self, latency: Duration) {
        let mut metrics = self.metrics.write().await;
        let latency_micros = latency.as_micros() as f64;
        
        // Update average latency using exponential moving average
        metrics.avg_latency_micros = 
            (metrics.avg_latency_micros * 0.95) + (latency_micros * 0.05);
    }
    
    /// Specialize this actor for a specific role
    pub async fn specialize(&mut self, specialization: ActorSpecialization) -> Result<()> {
        self.specialization = Some(specialization);
        
        // Update status to ready after specialization
        let mut status = self.status.write().await;
        *status = ActorStatus::Ready;
        
        Ok(())
    }
    
    /// Get current actor status
    pub async fn get_status(&self) -> ActorStatus {
        self.status.read().await.clone()
    }
    
    /// Get actor metrics
    pub async fn get_metrics(&self) -> ActorMetrics {
        self.metrics.read().await.clone()
    }
}

impl ActorState {
    /// Create new actor state
    pub fn new(state_type: String) -> Self {
        Self {
            data: Vec::new(),
            version: AtomicU64::new(0),
            state_type,
        }
    }
    
    /// Set state data (enforces 1.5KB limit)
    pub fn set_data(&mut self, data: Vec<u8>) -> Result<()> {
        if data.len() > MAX_ACTOR_STATE_SIZE {
            return Err(anyhow!(
                "Actor state size {} exceeds maximum {} bytes",
                data.len(),
                MAX_ACTOR_STATE_SIZE
            ));
        }
        
        self.data = data;
        self.version.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
    
    /// Get state data
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
    
    /// Get state version
    pub fn get_version(&self) -> u64 {
        self.version.load(Ordering::SeqCst)
    }
    
    /// Get state size in bytes
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl Message {
    /// Create a new message
    pub fn new(from: ActorId, to: ActorId, payload: MessagePayload) -> Self {
        let id = Uuid::new_v4().to_string();
        let size = Self::calculate_size(&payload);
        
        Self {
            id,
            from,
            to,
            payload,
            priority: 128, // Default priority
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64,
            size,
        }
    }
    
    /// Create a control message
    pub fn new_control(from: ActorId, to: ActorId, control: ControlMessage) -> Self {
        Self::new(from, to, MessagePayload::Control(control))
    }
    
    /// Calculate message size
    fn calculate_size(payload: &MessagePayload) -> u32 {
        match payload {
            MessagePayload::Raw(data) => data.len() as u32,
            MessagePayload::Json(json) => json.to_string().len() as u32,
            MessagePayload::Text(text) => text.len() as u32,
            MessagePayload::Control(_) => 64, // Estimated size
            MessagePayload::Application { data, .. } => data.len() as u32,
        }
    }
}

impl Default for ActorBudget {
    fn default() -> Self {
        Self {
            cpu_micros: 1000, // 1ms per epoch
            memory_bytes: MAX_ACTOR_STATE_SIZE as u64,
            network_bytes: 1024, // 1KB per epoch
            message_count: 10, // 10 messages per epoch
            expires_at: None,
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_cpu_percent: 10.0, // 10% CPU
            max_memory_bytes: 10 * 1024 * 1024, // 10MB
            max_network_bps: 1024 * 1024, // 1MB/s
            max_file_descriptors: 100,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_actor_creation() {
        let actor = VPodActor::new("test-actor".to_string(), 1024).unwrap();
        assert_eq!(actor.id, "test-actor");
        assert_eq!(actor.state.size(), 0);
    }

    #[tokio::test]
    async fn test_message_sending() {
        let actor = VPodActor::new("test-actor".to_string(), 1024).unwrap();
        
        let message = Message::new(
            "sender".to_string(),
            "test-actor".to_string(),
            MessagePayload::Text("Hello".to_string())
        );
        
        actor.send_message(message).await.unwrap();
        
        // Process the message
        let processed = actor.process_messages(1).await.unwrap();
        assert_eq!(processed, 1);
    }

    #[test]
    fn test_actor_state_size_limit() {
        let mut state = ActorState::new("test".to_string());
        
        // Should succeed with data under limit
        let small_data = vec![0u8; 1000];
        assert!(state.set_data(small_data).is_ok());
        
        // Should fail with data over limit
        let large_data = vec![0u8; 2000];
        assert!(state.set_data(large_data).is_err());
    }
}
