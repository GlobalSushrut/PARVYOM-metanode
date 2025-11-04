// Inter-Component Communication Hub
// Enables sophisticated communication between all 12 BPCI components
// Provides unified messaging, coordination, and state synchronization

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::{mpsc, Mutex};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use anyhow::{Result, anyhow};
use tracing::{info, warn, debug, error};

/// Component types in the unified BPCI infrastructure
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum ComponentType {
    Consensus,
    Blockchain,
    AuctionMempool,
    Orchestrator,
    BpiBridge,
    ClusterLedger,
    NetworkSecurity,
    Monitoring,
    Administration,
    NetworkInfrastructure,
    ShadowRegistry,
    SuperAdmin,
}

/// Inter-component message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterComponentMessage {
    /// Consensus-related messages
    ConsensusRoundStarted { round_id: String, validators: Vec<String> },
    ConsensusRoundCompleted { round_id: String, result: ConsensusResult },
    
    /// Blockchain-related messages
    BlockProduced { block_hash: String, height: u64, transactions: u32 },
    TransactionProcessed { tx_hash: String, status: TransactionStatus },
    
    /// Auction-related messages
    AuctionCreated { auction_id: String, auction_type: String },
    BidPlaced { auction_id: String, bid_amount: u64, bidder: String },
    AuctionCompleted { auction_id: String, winner: String, amount: u64 },
    
    /// Resource coordination messages
    ResourceRequested { component: ComponentType, resources: ResourceRequest },
    ResourceAllocated { component: ComponentType, allocation: ResourceAllocation },
    ResourceReleased { component: ComponentType, resources: ResourceRequest },
    
    /// Health and monitoring messages
    ComponentHealthUpdate { component: ComponentType, status: HealthStatus },
    MetricsUpdate { component: ComponentType, metrics: ComponentMetrics },
    AlertTriggered { component: ComponentType, alert: AlertInfo },
    
    /// Cross-instance communication
    Instance1Request { endpoint: String, payload: Vec<u8> },
    Instance1Response { request_id: String, response: Vec<u8> },
    
    /// System-wide coordination
    SystemShutdown { reason: String },
    SystemRestart { reason: String },
    ConfigurationUpdate { component: Option<ComponentType>, config: serde_json::Value },
}

/// Consensus result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusResult {
    pub success: bool,
    pub validator_count: u32,
    pub consensus_time_ms: u64,
    pub block_hash: Option<String>,
}

/// Transaction status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Processing,
    Confirmed,
    Failed { reason: String },
}

/// Resource request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequest {
    pub cpu_cores: f64,
    pub memory_mb: u64,
    pub storage_gb: u64,
    pub network_bandwidth: u64,
    pub duration_minutes: u32,
}

/// Resource allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocation {
    pub allocation_id: String,
    pub granted_resources: ResourceRequest,
    pub expires_at: DateTime<Utc>,
}

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded { reason: String },
    Unhealthy { reason: String },
    Critical { reason: String },
}

/// Component metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMetrics {
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub requests_per_second: f64,
    pub error_rate: f64,
    pub response_time_ms: f64,
    pub custom_metrics: HashMap<String, f64>,
}

/// Alert information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertInfo {
    pub alert_id: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub triggered_at: DateTime<Utc>,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Component Communication Hub
/// Central hub for all inter-component communication in the unified BPCI infrastructure
#[derive(Debug)]
pub struct ComponentCommunicationHub {
    /// Hub identifier
    pub hub_id: String,
    
    /// Component registry
    pub components: Arc<RwLock<HashMap<ComponentType, ComponentInfo>>>,
    
    /// Message channels for each component
    pub message_channels: Arc<Mutex<HashMap<ComponentType, mpsc::UnboundedSender<InterComponentMessage>>>>,
    
    /// Message history for debugging and monitoring
    pub message_history: Arc<RwLock<Vec<MessageHistoryEntry>>>,
    
    /// Hub statistics
    pub hub_stats: Arc<RwLock<HubStatistics>>,
}

/// Component information
#[derive(Debug, Clone)]
pub struct ComponentInfo {
    pub component_type: ComponentType,
    pub component_id: String,
    pub endpoint: String,
    pub port: u16,
    pub status: HealthStatus,
    pub registered_at: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
}

/// Message history entry
#[derive(Debug, Clone)]
pub struct MessageHistoryEntry {
    pub message_id: String,
    pub from_component: ComponentType,
    pub to_component: Option<ComponentType>,
    pub message: InterComponentMessage,
    pub timestamp: DateTime<Utc>,
    pub processed: bool,
}

/// Hub statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubStatistics {
    pub total_messages: u64,
    pub messages_per_component: HashMap<ComponentType, u64>,
    pub average_processing_time_ms: f64,
    pub error_count: u64,
    pub uptime_seconds: u64,
}

impl ComponentCommunicationHub {
    /// Create new communication hub
    pub fn new() -> Result<Self> {
        let hub_id = format!("hub-{}", Uuid::new_v4());
        
        Ok(Self {
            hub_id,
            components: Arc::new(RwLock::new(HashMap::new())),
            message_channels: Arc::new(Mutex::new(HashMap::new())),
            message_history: Arc::new(RwLock::new(Vec::new())),
            hub_stats: Arc::new(RwLock::new(HubStatistics {
                total_messages: 0,
                messages_per_component: HashMap::new(),
                average_processing_time_ms: 0.0,
                error_count: 0,
                uptime_seconds: 0,
            })),
        })
    }
    
    /// Register a component with the hub
    pub async fn register_component(
        &self,
        component_type: ComponentType,
        component_id: String,
        endpoint: String,
        port: u16,
    ) -> Result<mpsc::UnboundedReceiver<InterComponentMessage>> {
        let (tx, rx) = mpsc::unbounded_channel();
        
        // Register component info
        {
            let mut components = self.components.write().unwrap();
            components.insert(component_type.clone(), ComponentInfo {
                component_type: component_type.clone(),
                component_id,
                endpoint,
                port,
                status: HealthStatus::Healthy,
                registered_at: Utc::now(),
                last_heartbeat: Utc::now(),
            });
        }
        
        // Register message channel
        {
            let mut channels = self.message_channels.lock().await;
            channels.insert(component_type.clone(), tx);
        }
        
        info!("✅ Component {:?} registered with communication hub", component_type);
        Ok(rx)
    }
    
    /// Send message to specific component
    pub async fn send_to_component(
        &self,
        target: ComponentType,
        message: InterComponentMessage,
        from: ComponentType,
    ) -> Result<()> {
        let message_id = Uuid::new_v4().to_string();
        
        // Record message in history
        {
            let mut history = self.message_history.write().unwrap();
            history.push(MessageHistoryEntry {
                message_id: message_id.clone(),
                from_component: from.clone(),
                to_component: Some(target.clone()),
                message: message.clone(),
                timestamp: Utc::now(),
                processed: false,
            });
        }
        
        // Send message
        {
            let channels = self.message_channels.lock().await;
            if let Some(tx) = channels.get(&target) {
                tx.send(message)?;
                debug!("📤 Message {} sent from {:?} to {:?}", message_id, from, target);
            } else {
                warn!("⚠️ Component {:?} not found for message {}", target, message_id);
                return Err(anyhow!("Component {:?} not registered", target));
            }
        }
        
        // Update statistics
        {
            let mut stats = self.hub_stats.write().unwrap();
            stats.total_messages += 1;
            *stats.messages_per_component.entry(from).or_insert(0) += 1;
        }
        
        Ok(())
    }
    
    /// Broadcast message to all components
    pub async fn broadcast_message(
        &self,
        message: InterComponentMessage,
        from: ComponentType,
    ) -> Result<()> {
        let message_id = Uuid::new_v4().to_string();
        
        // Record message in history
        {
            let mut history = self.message_history.write().unwrap();
            history.push(MessageHistoryEntry {
                message_id: message_id.clone(),
                from_component: from.clone(),
                to_component: None,
                message: message.clone(),
                timestamp: Utc::now(),
                processed: false,
            });
        }
        
        // Broadcast to all components
        {
            let channels = self.message_channels.lock().await;
            for (component_type, tx) in channels.iter() {
                if *component_type != from {
                    if let Err(e) = tx.send(message.clone()) {
                        warn!("⚠️ Failed to send broadcast message to {:?}: {}", component_type, e);
                    }
                }
            }
        }
        
        debug!("📡 Broadcast message {} sent from {:?}", message_id, from);
        Ok(())
    }
    
    /// Update component health status
    pub async fn update_component_health(
        &self,
        component_type: ComponentType,
        status: HealthStatus,
    ) -> Result<()> {
        {
            let mut components = self.components.write().unwrap();
            if let Some(component) = components.get_mut(&component_type) {
                component.status = status.clone();
                component.last_heartbeat = Utc::now();
            }
        }
        
        // Broadcast health update
        self.broadcast_message(
            InterComponentMessage::ComponentHealthUpdate {
                component: component_type.clone(),
                status,
            },
            component_type,
        ).await?;
        
        Ok(())
    }
    
    /// Get hub statistics
    pub async fn get_hub_statistics(&self) -> HubStatistics {
        self.hub_stats.read().unwrap().clone()
    }
    
    /// Get all registered components
    pub async fn get_registered_components(&self) -> HashMap<ComponentType, ComponentInfo> {
        self.components.read().unwrap().clone()
    }
    
    /// Shutdown communication hub
    pub async fn shutdown(&self) -> Result<()> {
        info!("🛑 Shutting down component communication hub {}", self.hub_id);
        
        // Send shutdown message to all components
        self.broadcast_message(
            InterComponentMessage::SystemShutdown {
                reason: "Hub shutdown requested".to_string(),
            },
            ComponentType::SuperAdmin,
        ).await?;
        
        info!("✅ Component communication hub shutdown complete");
        Ok(())
    }
}

impl Default for ComponentCommunicationHub {
    fn default() -> Self {
        Self::new().expect("Failed to create default ComponentCommunicationHub")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_component_registration() {
        let hub = ComponentCommunicationHub::new().unwrap();
        
        let _rx = hub.register_component(
            ComponentType::Consensus,
            "consensus-1".to_string(),
            "localhost".to_string(),
            9001,
        ).await.unwrap();
        
        let components = hub.get_registered_components().await;
        assert!(components.contains_key(&ComponentType::Consensus));
    }
    
    #[tokio::test]
    async fn test_message_sending() {
        let hub = ComponentCommunicationHub::new().unwrap();
        
        let mut rx = hub.register_component(
            ComponentType::Blockchain,
            "blockchain-1".to_string(),
            "localhost".to_string(),
            8080,
        ).await.unwrap();
        
        let message = InterComponentMessage::ConsensusRoundStarted {
            round_id: "round-1".to_string(),
            validators: vec!["validator-1".to_string()],
        };
        
        hub.send_to_component(
            ComponentType::Blockchain,
            message.clone(),
            ComponentType::Consensus,
        ).await.unwrap();
        
        let received = rx.recv().await.unwrap();
        match received {
            InterComponentMessage::ConsensusRoundStarted { round_id, .. } => {
                assert_eq!(round_id, "round-1");
            }
            _ => panic!("Unexpected message type"),
        }
    }
}
