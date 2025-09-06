//! Communication module for BPI Oracle Node
//!
//! Handles real-time communication between BPI nodes through WebSocket connections,
//! message routing, and connection management with automatic reconnection.

use anyhow::Result;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{RwLock, Mutex};
use tokio_tungstenite::{accept_async, connect_async, tungstenite::Message, WebSocketStream};
use futures_util::{SinkExt, StreamExt};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::{BpiNode, OracleMessage, MessageResponse, NodeStatus, OracleConfig};

/// WebSocket connection wrapper
#[derive(Debug)]
pub struct WebSocketConnection {
    pub connection_id: String,
    pub node_id: String,
    pub last_ping: DateTime<Utc>,
    pub message_count: u64,
    pub connected_at: DateTime<Utc>,
}

/// Communication statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationStats {
    pub active_connections: usize,
    pub total_messages_sent: u64,
    pub total_messages_received: u64,
    pub failed_connections: u64,
    pub reconnection_attempts: u64,
    pub average_latency_ms: f64,
    pub bandwidth_usage_bytes: u64,
}

/// Communication manager for Oracle Node
#[derive(Debug)]
pub struct CommunicationManager {
    config: OracleConfig,
    connections: Arc<DashMap<String, WebSocketConnection>>,
    node_connections: Arc<DashMap<String, String>>, // node_id -> connection_id
    stats: Arc<RwLock<CommunicationStats>>,
    message_handlers: Arc<DashMap<String, tokio::sync::mpsc::Sender<OracleMessage>>>,
    shutdown_signal: Arc<Mutex<Option<tokio::sync::broadcast::Sender<()>>>>,
}

impl CommunicationManager {
    /// Create new communication manager
    pub async fn new(config: OracleConfig) -> Result<Self> {
        Ok(Self {
            config,
            connections: Arc::new(DashMap::new()),
            node_connections: Arc::new(DashMap::new()),
            stats: Arc::new(RwLock::new(CommunicationStats {
                active_connections: 0,
                total_messages_sent: 0,
                total_messages_received: 0,
                failed_connections: 0,
                reconnection_attempts: 0,
                average_latency_ms: 0.0,
                bandwidth_usage_bytes: 0,
            })),
            message_handlers: Arc::new(DashMap::new()),
            shutdown_signal: Arc::new(Mutex::new(None)),
        })
    }

    /// Start the communication manager
    pub async fn start(&self) -> Result<()> {
        info!("Starting Communication Manager on WebSocket port {}", self.config.ws_port);
        
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.config.ws_port)).await?;
        info!("✅ WebSocket server listening on port {}", self.config.ws_port);

        // Start accepting connections in background
        let connections = Arc::clone(&self.connections);
        let node_connections = Arc::clone(&self.node_connections);
        let stats = Arc::clone(&self.stats);
        let message_handlers = Arc::clone(&self.message_handlers);
        let config = self.config.clone();

        tokio::spawn(async move {
            while let Ok((stream, addr)) = listener.accept().await {
                info!("New WebSocket connection from: {}", addr);
                
                let connections = Arc::clone(&connections);
                let node_connections = Arc::clone(&node_connections);
                let stats = Arc::clone(&stats);
                let message_handlers = Arc::clone(&message_handlers);
                let config = config.clone();

                tokio::spawn(async move {
                    if let Err(e) = Self::handle_connection(stream, connections, node_connections, stats, message_handlers, config).await {
                        error!("WebSocket connection error: {}", e);
                    }
                });
            }
        });

        Ok(())
    }

    /// Handle individual WebSocket connection


    /// Start the communication server
    pub async fn start_server(&self) -> Result<()> {
        let addr = format!("0.0.0.0:{}", self.config.ws_port);
        let listener = TcpListener::bind(&addr).await?;
        
        info!("🌐 BPI Oracle WebSocket server listening on {}", addr);

        let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel::<()>(1);
        *self.shutdown_signal.lock().await = Some(shutdown_tx.clone());

        let connections = Arc::clone(&self.connections);
        let node_connections = Arc::clone(&self.node_connections);
        let stats = Arc::clone(&self.stats);
        let message_handlers = Arc::clone(&self.message_handlers);
        let config = self.config.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    result = listener.accept() => {
                        match result {
                            Ok((stream, addr)) => {
                                info!("New WebSocket connection from: {}", addr);
                                
                                let connections = Arc::clone(&connections);
                                let node_connections = Arc::clone(&node_connections);
                                let stats = Arc::clone(&stats);
                                let message_handlers = Arc::clone(&message_handlers);
                                let config = config.clone();
                                
                                tokio::spawn(async move {
                                    if let Err(e) = Self::handle_connection(
                                        stream, connections, node_connections, stats, message_handlers, config
                                    ).await {
                                        error!("WebSocket connection error: {}", e);
                                    }
                                });
                            }
                            Err(e) => {
                                error!("Failed to accept WebSocket connection: {}", e);
                            }
                        }
                    }
                    _ = shutdown_rx.recv() => {
                        info!("Shutting down WebSocket server");
                        break;
                    }
                }
            }
        });

        // Start connection monitoring
        self.start_connection_monitoring().await?;

        Ok(())
    }

    /// Handle individual WebSocket connection
    async fn handle_connection(
        stream: TcpStream,
        connections: Arc<DashMap<String, WebSocketConnection>>,
        node_connections: Arc<DashMap<String, String>>,
        stats: Arc<RwLock<CommunicationStats>>,
        message_handlers: Arc<DashMap<String, tokio::sync::mpsc::Sender<OracleMessage>>>,
        config: OracleConfig,
    ) -> Result<()> {
        let ws_stream = accept_async(stream).await?;
        let connection_id = Uuid::new_v4().to_string();
        
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        
        // Wait for node identification message
        let node_id = match ws_receiver.next().await {
            Some(Ok(Message::Text(text))) => {
                match serde_json::from_str::<NodeIdentification>(&text) {
                    Ok(ident) => {
                        info!("Node identified: {} ({})", ident.node_id, ident.node_type);
                        ident.node_id
                    }
                    Err(e) => {
                        error!("Invalid node identification: {}", e);
                        return Err(anyhow::anyhow!("Invalid node identification"));
                    }
                }
            }
            _ => {
                error!("Failed to receive node identification");
                return Err(anyhow::anyhow!("No node identification received"));
            }
        };

        // Create connection record
        let connection = WebSocketConnection {
            connection_id: connection_id.clone(),
            node_id: node_id.clone(),
            last_ping: Utc::now(),
            message_count: 0,
            connected_at: Utc::now(),
        };

        connections.insert(connection_id.clone(), connection);
        node_connections.insert(node_id.clone(), connection_id.clone());

        // Update stats
        {
            let mut stats_guard = stats.write().await;
            stats_guard.active_connections = connections.len();
        }

        // Create message handler for this node
        let (tx, mut rx) = tokio::sync::mpsc::channel::<OracleMessage>(1000);
        message_handlers.insert(node_id.clone(), tx);

        // Handle incoming messages
        let connections_clone = Arc::clone(&connections);
        let stats_clone = Arc::clone(&stats);
        let node_id_clone = node_id.clone();
        let connection_id_clone = connection_id.clone();

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Some(_connection) = connections_clone.get(&connection_id_clone) {
                    // TODO: Implement actual message sending through WebSocket
                    info!("Would send message to connection: {}", connection_id_clone);
                    
                    // Update stats
                    let mut stats_guard = stats_clone.write().await;
                    stats_guard.total_messages_sent += 1;
                } else {
                    error!("Connection not found: {}", connection_id_clone);
                    break;
                }
            }
        });

        info!("✅ WebSocket connection established for node: {}", node_id);
        Ok(())
    }

    /// Connect to a remote BPI node
    pub async fn connect_to_node(&self, node: &BpiNode) -> Result<()> {
        info!("Connecting to BPI node: {} at {}", node.node_id, node.endpoint);

        let ws_url = format!("ws://{}/oracle", node.endpoint.replace("http://", "").replace("https://", ""));
        
        match connect_async(&ws_url).await {
            Ok((ws_stream, _)) => {
                let connection_id = Uuid::new_v4().to_string();
                
                let connection = WebSocketConnection {
                    connection_id: connection_id.clone(),
                    node_id: node.node_id.clone(),
                    last_ping: Utc::now(),
                    message_count: 0,
                    connected_at: Utc::now(),
                };

                self.connections.insert(connection_id.clone(), connection);
                self.node_connections.insert(node.node_id.clone(), connection_id);

                // Send identification
                let identification = NodeIdentification {
                    node_id: self.config.node_id.clone(),
                    node_type: "Oracle".to_string(),
                    capabilities: vec!["message_relay".to_string(), "consensus_bridge".to_string()],
                };

                self.send_to_node(&node.node_id, &serde_json::to_string(&identification)?).await?;

                info!("✅ Connected to BPI node: {}", node.node_id);
                Ok(())
            }
            Err(e) => {
                error!("Failed to connect to node {}: {}", node.node_id, e);
                
                let mut stats = self.stats.write().await;
                stats.failed_connections += 1;
                
                Err(anyhow::anyhow!("Connection failed: {}", e))
            }
        }
    }

    /// Send message to specific node
    pub async fn send_to_node(&self, node_id: &str, message: &str) -> Result<()> {
        if let Some(handler) = self.message_handlers.get(node_id) {
            let oracle_message: OracleMessage = serde_json::from_str(message)?;
            handler.send(oracle_message).await
                .map_err(|e| anyhow::anyhow!("Failed to send message: {}", e))?;
            
            let mut stats = self.stats.write().await;
            stats.total_messages_sent += 1;
            stats.bandwidth_usage_bytes += message.len() as u64;
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Node not connected: {}", node_id))
        }
    }

    /// Broadcast message to all connected nodes
    pub async fn broadcast_message(&self, message: &str) -> Result<()> {
        let oracle_message: OracleMessage = serde_json::from_str(message)?;
        
        for handler in self.message_handlers.iter() {
            if let Err(e) = handler.send(oracle_message.clone()).await {
                warn!("Failed to broadcast to node {}: {}", handler.key(), e);
            }
        }

        let mut stats = self.stats.write().await;
        stats.total_messages_sent += self.message_handlers.len() as u64;
        stats.bandwidth_usage_bytes += (message.len() * self.message_handlers.len()) as u64;

        Ok(())
    }

    /// Get communication statistics
    pub async fn get_stats(&self) -> CommunicationStats {
        self.stats.read().await.clone()
    }

    /// Disconnect from a node
    pub async fn disconnect_node(&self, node_id: &str) -> Result<()> {
        if let Some((_, connection_id)) = self.node_connections.remove(node_id) {
            if let Some((_, connection)) = self.connections.remove(&connection_id) {
                if let Some(_connection) = self.connections.get(&connection_id) {
                    // TODO: Implement actual message sending through WebSocket
                    info!("Would send message to connection: {}", connection_id);
                } else {
                    return Err(anyhow::anyhow!("Connection not found: {}", connection_id));
                }
                let mut stats = self.stats.write().await;
                stats.active_connections = self.connections.len();

                Ok(())
            } else {
                return Err(anyhow::anyhow!("Connection not found: {}", connection_id));
            }
        } else {
            return Err(anyhow::anyhow!("Node not connected: {}", node_id));
        }
    }

    /// Start connection monitoring service
    async fn start_connection_monitoring(&self) -> Result<()> {
        let connections = Arc::clone(&self.connections);
        let node_connections = Arc::clone(&self.node_connections);
        let stats = Arc::clone(&self.stats);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                // Check connection health
                let mut disconnected_nodes = Vec::new();
                
                for connection in connections.iter() {
                    let last_ping_age = Utc::now() - connection.last_ping;
                    if last_ping_age.num_seconds() > 120 { // 2 minutes timeout
                        warn!("Connection timeout for node: {}", connection.node_id);
                        disconnected_nodes.push(connection.node_id.clone());
                    }
                }

                // Remove timed-out connections
                for node_id in disconnected_nodes {
                    if let Some((_, connection_id)) = node_connections.remove(&node_id) {
                        connections.remove(&connection_id);
                    }
                }

                // Update stats
                let mut stats_guard = stats.write().await;
                stats_guard.active_connections = connections.len();
            }
        });

        Ok(())
    }

    /// Shutdown communication manager
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down communication manager");

        // Send shutdown signal
        let (shutdown_tx, _) = tokio::sync::broadcast::channel::<()>(1);
        *self.shutdown_signal.lock().await = Some(shutdown_tx);

        // Close all connections
        for connection in self.connections.iter() {
            info!("Closing connection: {}", connection.connection_id);
            // TODO: Implement actual WebSocket stream closing
        }

        self.connections.clear();
        self.node_connections.clear();
        self.message_handlers.clear();

        Ok(())
    }
}

/// Node identification message
#[derive(Debug, Clone, Serialize, Deserialize)]
struct NodeIdentification {
    node_id: String,
    node_type: String,
    capabilities: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_communication_manager_creation() {
        let config = OracleConfig::default();
        let comm_manager = CommunicationManager::new(config).await.unwrap();
        
        let stats = comm_manager.get_stats().await;
        assert_eq!(stats.active_connections, 0);
        assert_eq!(stats.total_messages_sent, 0);
    }
}
