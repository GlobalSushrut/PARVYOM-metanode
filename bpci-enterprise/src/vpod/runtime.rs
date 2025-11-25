//! # vPod Runtime Engine
//! 
//! Core runtime system that manages vPod actors, scheduling, and resource allocation.
//! Provides the foundation for replacing traditional container orchestration.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::interval;
use uuid::Uuid;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};

use crate::vpod::{
    VPodActor, VPodScheduler, ActorId, ActorSpecialization, Message, ActorStatus
};
use tokio::sync::mpsc;

/// vPod Runtime configuration
#[derive(Debug, Clone)]
pub struct VPodConfig {
    /// Maximum number of actors per runtime
    pub max_actors: usize,
    
    /// Epoch duration for scheduling
    pub epoch_duration: Duration,
    
    /// Ring buffer size for actor communication
    pub ring_buffer_size: usize,
    
    /// Maximum actor state size in bytes
    pub max_actor_state_bytes: usize,
    
    /// Enable dual-core scheduling
    pub dual_core_enabled: bool,
}

/// vPod Runtime - Main orchestration engine
#[derive(Debug)]
pub struct VPodRuntime {
    /// Runtime configuration
    config: VPodConfig,
    
    /// Active actors registry
    actors: Arc<RwLock<HashMap<ActorId, Arc<VPodActor>>>>,
    
    /// vPod scheduler
    scheduler: Arc<VPodScheduler>,
    
    /// Runtime metrics
    metrics: Arc<RwLock<RuntimeMetrics>>,
    
    /// Event channel for runtime events
    event_tx: mpsc::UnboundedSender<RuntimeEvent>,
    
    /// Runtime status
    status: Arc<RwLock<RuntimeStatus>>,
    
    /// Actor creation counter
    actor_counter: Arc<RwLock<u64>>,
}

/// Runtime performance metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuntimeMetrics {
    /// Total actors created
    pub actors_created: u64,
    
    /// Currently active actors
    pub active_actors: u64,
    
    /// Total messages processed
    pub messages_processed: u64,
    
    /// Average message latency (microseconds)
    pub avg_message_latency_micros: f64,
    
    /// Messages per second throughput
    pub throughput_mps: f64,
    
    /// CPU utilization percentage
    pub cpu_utilization: f64,
    
    /// Memory utilization (bytes)
    pub memory_utilization: u64,
    
    /// Epoch processing time (microseconds)
    pub epoch_processing_time_micros: f64,
    
    /// Scheduler efficiency (0.0 to 1.0)
    pub scheduler_efficiency: f64,
    
    /// Last metrics update (as timestamp)
    pub last_updated: Option<u64>,
}

/// Runtime status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuntimeStatus {
    /// Runtime is initializing
    Initializing,
    
    /// Runtime is running normally
    Running,
    
    /// Runtime is paused
    Paused,
    
    /// Runtime is shutting down
    ShuttingDown,
    
    /// Runtime has stopped
    Stopped,
    
    /// Runtime encountered an error
    Error { message: String },
}

/// Runtime events
#[derive(Debug, Clone)]
pub enum RuntimeEvent {
    /// Actor was created
    ActorCreated { actor_id: ActorId },
    
    /// Actor was destroyed
    ActorDestroyed { actor_id: ActorId },
    
    /// Actor status changed
    ActorStatusChanged { 
        actor_id: ActorId, 
        old_status: ActorStatus,
        new_status: ActorStatus 
    },
    
    /// Epoch completed
    EpochCompleted { 
        epoch_id: u64, 
        duration_micros: u64,
        messages_processed: u64 
    },
    
    /// Runtime status changed
    RuntimeStatusChanged { 
        old_status: RuntimeStatus,
        new_status: RuntimeStatus 
    },
    
    /// Performance threshold exceeded
    PerformanceAlert { 
        metric: String, 
        value: f64, 
        threshold: f64 
    },
}

impl VPodRuntime {
    /// Create a new vPod runtime
    pub async fn new(config: VPodConfig) -> Result<Self, anyhow::Error> {
        let (event_tx, _event_rx) = mpsc::unbounded_channel();
        
        let scheduler = Arc::new(VPodScheduler::new(
            config.epoch_duration,
            config.dual_core_enabled
        ).await?);
        
        let runtime = VPodRuntime {
            config,
            actors: Arc::new(RwLock::new(HashMap::new())),
            scheduler,
            metrics: Arc::new(RwLock::new(RuntimeMetrics::default())),
            event_tx,
            status: Arc::new(RwLock::new(RuntimeStatus::Initializing)),
            actor_counter: Arc::new(RwLock::new(0)),
        };
        
        // Start the runtime
        runtime.start().await?;
        
        Ok(runtime)
    }
    
    /// Start the runtime
    async fn start(&self) -> Result<(), anyhow::Error> {
        // Update status to running
        {
            let mut status = self.status.write().await;
            *status = RuntimeStatus::Running;
        }
        
        // Start the scheduler
        self.scheduler.start().await?;
        
        // Start metrics collection
        self.start_metrics_collection().await;
        
        // Emit runtime started event
        let _ = self.event_tx.send(RuntimeEvent::RuntimeStatusChanged {
            old_status: RuntimeStatus::Initializing,
            new_status: RuntimeStatus::Running,
        });
        
        Ok(())
    }
    
    /// Create a new actor
    pub async fn create_actor(
        &self,
        specialization: Option<ActorSpecialization>
    ) -> Result<ActorId, anyhow::Error> {
        // Check if we've reached the actor limit
        let actors_count = self.actors.read().await.len();
        if actors_count >= self.config.max_actors {
            return Err(anyhow!("Maximum number of actors ({}) reached", self.config.max_actors));
        }
        
        // Generate unique actor ID
        let actor_id = Uuid::new_v4(); // Generate UUID instead of counter
        
        // Create the actor
        let mut actor = VPodActor::new(actor_id, self.config.ring_buffer_size)?;
        
        // Apply specialization if provided
        if let Some(spec) = specialization {
            actor.specialize(spec).await?;
        }
        
        let actor = Arc::new(actor);
        
        // Register actor with scheduler
        self.scheduler.register_actor(actor_id, actor.clone()).await?;
        
        // Add to actors registry
        {
            let mut actors = self.actors.write().await;
            actors.insert(actor_id, actor);
        }
        
        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.actors_created += 1;
            metrics.active_actors += 1;
        }
        
        // Emit actor created event
        let _ = self.event_tx.send(RuntimeEvent::ActorCreated { 
            actor_id 
        });
        
        Ok(actor_id)
    }
    
    /// Destroy an actor
    pub async fn destroy_actor(&self, actor_id: &ActorId) -> Result<(), anyhow::Error> {
        // Remove from scheduler first
        self.scheduler.unregister_actor(actor_id).await?;
        
        // Remove from actors registry
        let actor = {
            let mut actors = self.actors.write().await;
            actors.remove(actor_id)
        };
        
        if actor.is_none() {
            return Err(anyhow!("Actor {} not found", actor_id));
        }
        
        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.active_actors = metrics.active_actors.saturating_sub(1);
        }
        
        // Emit actor destroyed event
        let _ = self.event_tx.send(RuntimeEvent::ActorDestroyed { 
            actor_id: actor_id.clone() 
        });
        
        Ok(())
    }
    
    /// Send a message to an actor
    pub async fn send_message(&self, message: Message) -> Result<(), anyhow::Error> {
        let actors = self.actors.read().await;
        let actor = actors.get(&message.to)
            .ok_or_else(|| anyhow!("Actor {} not found", message.to))?;
        
        actor.send_message(message).await?;
        
        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.messages_processed += 1;
        }
        
        Ok(())
    }
    
    /// Get actor by ID
    pub async fn get_actor(&self, actor_id: &ActorId) -> Option<Arc<VPodActor>> {
        let actors = self.actors.read().await;
        actors.get(actor_id).cloned()
    }
    
    /// List all active actors
    pub async fn list_actors(&self) -> Vec<ActorId> {
        let actors = self.actors.read().await;
        actors.keys().cloned().collect()
    }
    
    /// Get runtime metrics
    pub async fn get_metrics(&self) -> RuntimeMetrics {
        self.metrics.read().await.clone()
    }
    
    /// Get runtime status
    pub async fn get_status(&self) -> RuntimeStatus {
        self.status.read().await.clone()
    }
    
    /// Pause the runtime
    pub async fn pause(&self) -> Result<(), anyhow::Error> {
        let old_status = {
            let mut status = self.status.write().await;
            let old = status.clone();
            *status = RuntimeStatus::Paused;
            old
        };
        
        // Pause the scheduler
        self.scheduler.pause().await?;
        
        // Emit status change event
        let _ = self.event_tx.send(RuntimeEvent::RuntimeStatusChanged {
            old_status,
            new_status: RuntimeStatus::Paused,
        });
        
        Ok(())
    }
    
    /// Resume the runtime
    pub async fn resume(&self) -> Result<(), anyhow::Error> {
        let old_status = {
            let mut status = self.status.write().await;
            let old = status.clone();
            *status = RuntimeStatus::Running;
            old
        };
        
        // Resume the scheduler
        self.scheduler.resume().await?;
        
        // Emit status change event
        let _ = self.event_tx.send(RuntimeEvent::RuntimeStatusChanged {
            old_status,
            new_status: RuntimeStatus::Running,
        });
        
        Ok(())
    }
    
    /// Shutdown the runtime
    pub async fn shutdown(&self) -> Result<(), anyhow::Error> {
        // Update status
        {
            let mut status = self.status.write().await;
            *status = RuntimeStatus::ShuttingDown;
        }
        
        // Stop the scheduler
        self.scheduler.stop().await?;
        
        // Destroy all actors
        let actor_ids: Vec<ActorId> = {
            let actors = self.actors.read().await;
            actors.keys().cloned().collect()
        };
        
        for actor_id in actor_ids {
            let _ = self.destroy_actor(&actor_id).await;
        }
        
        // Update final status
        {
            let mut status = self.status.write().await;
            *status = RuntimeStatus::Stopped;
        }
        
        Ok(())
    }
    
    /// Start metrics collection background task
    async fn start_metrics_collection(&self) {
        let metrics = self.metrics.clone();
        let actors = self.actors.clone();
        let scheduler = self.scheduler.clone();
        let event_tx = self.event_tx.clone();
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(100)); // 100ms intervals
            
            loop {
                interval.tick().await;
                
                // Collect metrics
                let mut metrics_guard = metrics.write().await;
                
                // Update active actors count
                let actors_guard = actors.read().await;
                metrics_guard.active_actors = actors_guard.len() as u64;
                
                // Get scheduler metrics
                if let Ok(scheduler_metrics) = scheduler.get_metrics().await {
                    metrics_guard.epoch_processing_time_micros = scheduler_metrics.avg_epoch_duration_micros;
                    metrics_guard.scheduler_efficiency = scheduler_metrics.efficiency;
                }
                
                // Calculate throughput and latency from individual actors
                let mut total_throughput = 0.0;
                let mut total_latency = 0.0;
                let mut actor_count = 0;
                
                for actor in actors_guard.values() {
                    let actor_metrics = actor.get_metrics().await;
                    total_throughput += actor_metrics.throughput_mps;
                    total_latency += actor_metrics.avg_latency_micros;
                    actor_count += 1;
                }
                
                if actor_count > 0 {
                    metrics_guard.throughput_mps = total_throughput;
                    metrics_guard.avg_message_latency_micros = total_latency / actor_count as f64;
                }
                
                metrics_guard.last_updated = Some(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                );
                
                // Check for performance alerts
                if metrics_guard.avg_message_latency_micros > 1000.0 { // > 1ms
                    let _ = event_tx.send(RuntimeEvent::PerformanceAlert {
                        metric: "avg_message_latency_micros".to_string(),
                        value: metrics_guard.avg_message_latency_micros,
                        threshold: 1000.0,
                    });
                }
                
                if metrics_guard.throughput_mps < 1000.0 { // < 1K msgs/sec
                    let _ = event_tx.send(RuntimeEvent::PerformanceAlert {
                        metric: "throughput_mps".to_string(),
                        value: metrics_guard.throughput_mps,
                        threshold: 1000.0,
                    });
                }
                
                drop(metrics_guard);
                drop(actors_guard);
            }
        });
    }
    
    /// Create multiple actors with the same specialization
    pub async fn create_actor_pool(
        &self,
        count: usize,
        specialization: ActorSpecialization
    ) -> Result<Vec<ActorId>, anyhow::Error> {
        let mut actor_ids = Vec::with_capacity(count);
        
        for _ in 0..count {
            let actor_id = self.create_actor(Some(specialization.clone())).await?;
            actor_ids.push(actor_id);
        }
        
        Ok(actor_ids)
    }
    
    /// Broadcast a message to all actors
    pub async fn broadcast_message(&self, message: Message) -> Result<u64, anyhow::Error> {
        let actors = self.actors.read().await;
        let mut sent_count = 0;
        
        for (actor_id, actor) in actors.iter() {
            let mut broadcast_msg = message.clone();
            broadcast_msg.to = actor_id.clone();
            broadcast_msg.id = Uuid::new_v4().to_string(); // New ID for each copy
            
            if actor.send_message(broadcast_msg).await.is_ok() {
                sent_count += 1;
            }
        }
        
        Ok(sent_count)
    }
    
    /// Get detailed runtime statistics
    pub async fn get_detailed_stats(&self) -> RuntimeStats {
        let metrics = self.get_metrics().await;
        let status = self.get_status().await;
        let actors = self.actors.read().await;
        
        let mut actor_stats = Vec::new();
        for (actor_id, actor) in actors.iter() {
            let actor_metrics = actor.get_metrics().await;
            let actor_status = actor.get_status().await;
            
            actor_stats.push(ActorStats {
                actor_id: actor_id.clone(),
                status: actor_status,
                metrics: actor_metrics,
            });
        }
        
        RuntimeStats {
            status,
            metrics,
            actor_count: actors.len(),
            actor_stats,
        }
    }
}

/// Detailed runtime statistics
#[derive(Debug, Clone)]
pub struct RuntimeStats {
    pub status: RuntimeStatus,
    pub metrics: RuntimeMetrics,
    pub actor_count: usize,
    pub actor_stats: Vec<ActorStats>,
}

/// Individual actor statistics
#[derive(Debug, Clone)]
pub struct ActorStats {
    pub actor_id: ActorId,
    pub status: ActorStatus,
    pub metrics: crate::vpod::actor::ActorMetrics,
}

impl Default for VPodConfig {
    fn default() -> Self {
        Self {
            max_actors: 1000,
            epoch_duration: Duration::from_micros(10), // 10 microseconds
            ring_buffer_size: 1024,
            max_actor_state_bytes: 1536, // 1.5KB
            dual_core_enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vpod::actor::MessagePayload;

    #[tokio::test]
    async fn test_runtime_creation() {
        let config = VPodConfig::default();
        let runtime = VPodRuntime::new(config).await.unwrap();
        
        let status = runtime.get_status().await;
        assert!(matches!(status, RuntimeStatus::Running));
    }

    #[tokio::test]
    async fn test_actor_creation() {
        let config = VPodConfig::default();
        let runtime = VPodRuntime::new(config).await.unwrap();
        
        let actor_id = runtime.create_actor(None).await.unwrap();
        assert!(!actor_id.is_nil());
        
        let actors = runtime.list_actors().await;
        assert_eq!(actors.len(), 1);
        assert_eq!(actors[0], actor_id);
    }

    #[tokio::test]
    async fn test_message_sending() {
        let config = VPodConfig::default();
        let runtime = VPodRuntime::new(config).await.unwrap();
        
        let actor_id = runtime.create_actor(None).await.unwrap();
        
        let sender_id = uuid::Uuid::new_v4();
        let message = Message::new(
            sender_id,
            actor_id,
            MessagePayload::Text("Hello".to_string())
        );
        
        runtime.send_message(message).await.unwrap();
        
        let metrics = runtime.get_metrics().await;
        assert_eq!(metrics.messages_processed, 1);
    }

    #[tokio::test]
    async fn test_actor_pool_creation() {
        let config = VPodConfig::default();
        let runtime = VPodRuntime::new(config).await.unwrap();
        
        let specialization = ActorSpecialization::Generic;
        let actor_ids = runtime.create_actor_pool(5, specialization).await.unwrap();
        
        assert_eq!(actor_ids.len(), 5);
        
        let all_actors = runtime.list_actors().await;
        assert_eq!(all_actors.len(), 5);
    }

    #[tokio::test]
    async fn test_runtime_pause_resume() {
        let config = VPodConfig::default();
        let runtime = VPodRuntime::new(config).await.unwrap();
        
        runtime.pause().await.unwrap();
        let status = runtime.get_status().await;
        assert!(matches!(status, RuntimeStatus::Paused));
        
        runtime.resume().await.unwrap();
        let status = runtime.get_status().await;
        assert!(matches!(status, RuntimeStatus::Running));
    }
}
