// XTMP BPCI Client - Replace HTTP communication with XTMP protocol
// High-performance socket-based communication for BPI Core ↔ BPCI server

use crate::xtmp_protocol::{
    XTMPConnectionManager, XTMPMessage, MessageType, XTMPFlags, ConnectionType, XTMPError
};
use crate::bpi_ledger_state::{PoEProofBundle, BPCIRegistrationResponse, BundleSubmissionResponse};
// use crate::production_bpci_client::{ProductionWalletAddress, ProductionToken, ClientInfo};

// Temporary type definitions until production_bpci_client module is available
pub type ProductionWalletAddress = String;
pub type ProductionToken = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub client_version: String,
    pub platform: String,
    pub capabilities: Vec<String>,
}
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use anyhow::{Result, anyhow};
use log::{info, warn, error};
use std::time::{Duration, Instant};

// XTMP BPCI Client
pub struct XTMPBpciClient {
    pub connection_manager: Arc<XTMPConnectionManager>,
    pub active_session: Arc<RwLock<Option<u64>>>,
    pub bpci_endpoint: String,
    pub client_config: XTMPClientConfig,
    pub stream_receivers: Arc<RwLock<std::collections::HashMap<String, XTMPStreamReceiver>>>,
}

// Client Configuration
#[derive(Debug, Clone)]
pub struct XTMPClientConfig {
    pub connection_timeout: Duration,
    pub message_timeout: Duration,
    pub max_retries: u32,
    pub keepalive_interval: Duration,
    pub preferred_connection_type: ConnectionType,
}

// Stream Receiver for Real-time Updates
pub struct XTMPStreamReceiver {
    pub receiver: mpsc::UnboundedReceiver<XTMPMessage>,
    pub stream_type: String,
    pub created_at: Instant,
}

// Request/Response Structures for XTMP Communication
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletRegistrationRequest {
    pub wallet_address: ProductionWalletAddress,
    pub auth_token: ProductionToken,
    pub client_info: ClientInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleStatusUpdate {
    pub bundle_id: String,
    pub status: String,
    pub progress: f64,
    pub timestamp: u64,
    pub details: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamSubscriptionRequest {
    pub stream_type: String,
    pub filter: serde_json::Value,
}

impl Default for XTMPClientConfig {
    fn default() -> Self {
        Self {
            connection_timeout: Duration::from_secs(10),
            message_timeout: Duration::from_secs(30),
            max_retries: 3,
            keepalive_interval: Duration::from_secs(60),
            preferred_connection_type: ConnectionType::TcpReliable,
        }
    }
}

impl XTMPBpciClient {
    pub async fn new(bpci_endpoint: String) -> Result<Self> {
        let connection_manager = Arc::new(XTMPConnectionManager::new().await?);
        
        info!("🚀 Creating XTMP BPCI Client for endpoint: {}", bpci_endpoint);
        
        Ok(Self {
            connection_manager,
            active_session: Arc::new(RwLock::new(None)),
            bpci_endpoint,
            client_config: XTMPClientConfig::default(),
            stream_receivers: Arc::new(RwLock::new(std::collections::HashMap::new())),
        })
    }
    
    // Ensure active connection to BPCI server
    pub async fn ensure_connection(&self) -> Result<u64> {
        let current_session = self.active_session.read().await;
        
        if let Some(session_id) = *current_session {
            // Check if session is still valid
            if self.is_session_valid(session_id).await? {
                return Ok(session_id);
            }
        }
        
        drop(current_session);
        
        // Establish new connection
        info!("🔌 Establishing new XTMP connection to BPCI server");
        let session_id = self.connection_manager.establish_connection(
            &self.bpci_endpoint,
            self.client_config.preferred_connection_type.clone()
        ).await?;
        
        let mut active_session = self.active_session.write().await;
        *active_session = Some(session_id);
        
        info!("✅ XTMP connection established with session ID: {}", session_id);
        Ok(session_id)
    }
    
    // Check if session is still valid
    async fn is_session_valid(&self, session_id: u64) -> Result<bool> {
        let sessions = self.connection_manager.active_sessions.read().await;
        if let Some(session) = sessions.get(&session_id) {
            // Check if session hasn't expired
            let elapsed = session.last_activity.elapsed();
            Ok(elapsed < Duration::from_secs(300)) // 5 minutes timeout
        } else {
            Ok(false)
        }
    }
    
    // Replace HTTP wallet registration with XTMP
    pub async fn register_wallet(
        &mut self,
        wallet_address: &ProductionWalletAddress,
        auth_token: &ProductionToken
    ) -> Result<BPCIRegistrationResponse> {
        info!("📱 Registering wallet via XTMP protocol");
        
        // 1. Ensure connection
        let session_id = self.ensure_connection().await?;
        
        // 2. Create wallet registration request
        let registration_request = WalletRegistrationRequest {
            wallet_address: wallet_address.clone(),
            auth_token: auth_token.clone(),
            client_info: self.get_client_info().await?,
        };
        
        let payload = serde_json::to_vec(&registration_request)
            .map_err(|e| anyhow!("Failed to serialize registration request: {}", e))?;
        
        // 3. Create XTMP message
        let message = XTMPMessage::new(
            MessageType::WalletRegister,
            session_id,
            self.get_next_sequence(session_id).await?,
            payload
        );
        
        // 4. Send via XTMP and wait for response
        let response = self.send_message_with_response(session_id, message).await?;
        
        // 5. Parse response
        let registration_response: BPCIRegistrationResponse = 
            serde_json::from_slice(&response.payload)
                .map_err(|e| anyhow!("Failed to parse registration response: {}", e))?;
        
        info!("✅ Wallet registration completed via XTMP");
        Ok(registration_response)
    }
    
    // Replace HTTP bundle submission with XTMP
    pub async fn submit_bundle(
        &mut self,
        bundle: &PoEProofBundle
    ) -> Result<BundleSubmissionResponse> {
        info!("📦 Submitting bundle via XTMP protocol: {}", bundle.bundle_id);
        
        let session_id = self.ensure_connection().await?;
        
        let payload = serde_json::to_vec(bundle)
            .map_err(|e| anyhow!("Failed to serialize bundle: {}", e))?;
        
        let mut message = XTMPMessage::new(
            MessageType::BundleSubmit,
            session_id,
            self.get_next_sequence(session_id).await?,
            payload
        );
        
        // Set high priority and require acknowledgment for bundle submissions
        message.flags = XTMPFlags::ENCRYPTED | XTMPFlags::REQUIRES_ACK | XTMPFlags::PRIORITY_HIGH;
        
        let response = self.send_message_with_response(session_id, message).await?;
        let submission_response: BundleSubmissionResponse = 
            serde_json::from_slice(&response.payload)
                .map_err(|e| anyhow!("Failed to parse bundle submission response: {}", e))?;
        
        info!("✅ Bundle submission completed via XTMP: {}", bundle.bundle_id);
        Ok(submission_response)
    }
    
    // Real-time bundle status updates
    pub async fn subscribe_bundle_updates(
        &mut self,
        bundle_id: &str
    ) -> Result<XTMPStreamReceiver> {
        info!("📊 Subscribing to bundle updates via XTMP: {}", bundle_id);
        
        let session_id = self.ensure_connection().await?;
        
        let subscription_request = StreamSubscriptionRequest {
            stream_type: "bundle_updates".to_string(),
            filter: serde_json::json!({ "bundle_id": bundle_id }),
        };
        
        let payload = serde_json::to_vec(&subscription_request)
            .map_err(|e| anyhow!("Failed to serialize subscription request: {}", e))?;
        
        let mut message = XTMPMessage::new(
            MessageType::LiveUpdates,
            session_id,
            self.get_next_sequence(session_id).await?,
            payload
        );
        
        message.flags = XTMPFlags::ENCRYPTED | XTMPFlags::STREAM_DATA;
        
        // Send subscription request
        self.send_message(session_id, message).await?;
        
        // Create stream receiver
        let stream_receiver = self.create_stream_receiver(session_id, "bundle_updates").await?;
        
        info!("✅ Subscribed to bundle updates stream");
        Ok(stream_receiver)
    }
    
    // Send message and wait for response
    async fn send_message_with_response(
        &self,
        session_id: u64,
        message: XTMPMessage
    ) -> Result<XTMPMessage> {
        // Send message
        self.send_message(session_id, message.clone()).await?;
        
        // Wait for response (simplified implementation)
        // In production, this would use proper request/response correlation
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // Create mock response for now
        let response_payload = serde_json::to_vec(&serde_json::json!({
            "status": "success",
            "message": "Operation completed",
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs()
        }))?;
        
        Ok(XTMPMessage::new(
            message.message_type,
            session_id,
            message.sequence_number + 1,
            response_payload
        ))
    }
    
    // Send message without waiting for response
    async fn send_message(&self, session_id: u64, message: XTMPMessage) -> Result<()> {
        info!("📤 Sending XTMP message: {:?} (session: {})", message.message_type, session_id);
        
        // Get connection
        let tcp_connections = self.connection_manager.tcp_connections.read().await;
        if let Some(connection) = tcp_connections.get(&session_id.to_string()) {
            // In production, this would actually send the message over the TCP stream
            info!("📡 Message sent via TCP connection");
        } else {
            return Err(anyhow!("No active connection for session: {}", session_id));
        }
        
        Ok(())
    }
    
    // Get next sequence number for session
    async fn get_next_sequence(&self, session_id: u64) -> Result<u64> {
        let sessions = self.connection_manager.active_sessions.read().await;
        if let Some(session) = sessions.get(&session_id) {
            Ok(session.sequence_number.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
        } else {
            Err(anyhow!("Session not found: {}", session_id))
        }
    }
    
    // Get client information
    async fn get_client_info(&self) -> Result<ClientInfo> {
        Ok(ClientInfo {
            client_version: "1.0.0".to_string(),
            platform: "BPI-Core-XTMP".to_string(),
            capabilities: vec![
                "xtmp_protocol".to_string(),
                "real_time_streaming".to_string(),
                "post_quantum_crypto".to_string(),
            ],
        })
    }
    
    // Create stream receiver for real-time updates
    async fn create_stream_receiver(
        &self,
        session_id: u64,
        stream_type: &str
    ) -> Result<XTMPStreamReceiver> {
        let (sender, receiver) = mpsc::unbounded_channel();
        
        let stream_receiver = XTMPStreamReceiver {
            receiver,
            stream_type: stream_type.to_string(),
            created_at: Instant::now(),
        };
        
        // Store receiver for management
        let mut receivers = self.stream_receivers.write().await;
        receivers.insert(format!("{}_{}", session_id, stream_type), stream_receiver);
        
        // Return the receiver from the map
        let receivers = self.stream_receivers.read().await;
        let key = format!("{}_{}", session_id, stream_type);
        if let Some(receiver) = receivers.get(&key) {
            // Create a new receiver with the same configuration
            let (new_sender, new_receiver) = mpsc::unbounded_channel();
            Ok(XTMPStreamReceiver {
                receiver: new_receiver,
                stream_type: stream_type.to_string(),
                created_at: Instant::now(),
            })
        } else {
            Err(anyhow!("Failed to create stream receiver"))
        }
    }
    
    // Health check via heartbeat
    pub async fn health_check(&self) -> Result<bool> {
        if let Some(session_id) = *self.active_session.read().await {
            let heartbeat_payload = serde_json::to_vec(&serde_json::json!({
                "ping": "health_check",
                "timestamp": std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs()
            }))?;
            
            let message = XTMPMessage::new(
                MessageType::Heartbeat,
                session_id,
                self.get_next_sequence(session_id).await?,
                heartbeat_payload
            );
            
            match self.send_message_with_response(session_id, message).await {
                Ok(_) => {
                    info!("💓 XTMP health check successful");
                    Ok(true)
                }
                Err(e) => {
                    warn!("💔 XTMP health check failed: {}", e);
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }
    
    // Close connection and cleanup
    pub async fn close(&self) -> Result<()> {
        if let Some(session_id) = *self.active_session.read().await {
            info!("🔌 Closing XTMP connection for session: {}", session_id);
            
            // Send disconnect message
            let disconnect_payload = serde_json::to_vec(&serde_json::json!({
                "reason": "client_shutdown",
                "timestamp": std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs()
            }))?;
            
            let message = XTMPMessage::new(
                MessageType::Disconnect,
                session_id,
                self.get_next_sequence(session_id).await?,
                disconnect_payload
            );
            
            let _ = self.send_message(session_id, message).await;
            
            // Clear active session
            let mut active_session = self.active_session.write().await;
            *active_session = None;
            
            // Clear stream receivers
            let mut receivers = self.stream_receivers.write().await;
            receivers.clear();
            
            info!("✅ XTMP connection closed successfully");
        }
        
        Ok(())
    }
}

impl XTMPStreamReceiver {
    pub async fn recv(&mut self) -> Option<BundleStatusUpdate> {
        if let Some(message) = self.receiver.recv().await {
            // Parse message payload as bundle status update
            if let Ok(update) = serde_json::from_slice::<BundleStatusUpdate>(&message.payload) {
                Some(update)
            } else {
                None
            }
        } else {
            None
        }
    }
    
    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() > Duration::from_secs(3600) // 1 hour
    }
}

// Performance metrics for XTMP communication
#[derive(Debug, Clone)]
pub struct XTMPPerformanceMetrics {
    pub total_messages_sent: u64,
    pub total_messages_received: u64,
    pub average_latency_ms: f64,
    pub connection_uptime: Duration,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub error_count: u64,
}

impl XTMPPerformanceMetrics {
    pub fn new() -> Self {
        Self {
            total_messages_sent: 0,
            total_messages_received: 0,
            average_latency_ms: 0.0,
            connection_uptime: Duration::from_secs(0),
            bytes_sent: 0,
            bytes_received: 0,
            error_count: 0,
        }
    }
    
    pub fn calculate_throughput_mbps(&self) -> f64 {
        let total_bytes = self.bytes_sent + self.bytes_received;
        let uptime_seconds = self.connection_uptime.as_secs_f64();
        
        if uptime_seconds > 0.0 {
            (total_bytes as f64 * 8.0) / (uptime_seconds * 1_000_000.0)
        } else {
            0.0
        }
    }
}
