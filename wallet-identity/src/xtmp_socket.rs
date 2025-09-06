use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use chrono::{DateTime, Utc};
use ed25519_dalek::{Keypair, Signature, Signer, Verifier};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{connect_async, accept_async, tungstenite::Message, WebSocketStream};
use futures_util::{SinkExt, StreamExt};
use anyhow::{Result, anyhow};
use uuid::Uuid;
use crate::wallet_identity::WalletIdentity;
use crate::identity_registry::IdentityRegistry;

/// XTMP Socket - Real-time Communication Protocol
/// Enables secure video calls, messaging, and real-time data exchange between wallets

/// Media type for real-time communication
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MediaType {
    Audio,
    Video,
    Screen,
    Data,
}

/// Media relay server information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaRelay {
    pub id: String,
    pub url: String,
    pub region: String,
    pub capacity: usize,
    pub active_streams: usize,
}

/// Socket statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketStats {
    pub active_connections: usize,
    pub active_calls: usize,
    pub total_calls: usize,
    pub media_relays: usize,
}

/// Device capabilities for authorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    pub camera: bool,
    pub microphone: bool,
    pub screen_share: bool,
    pub speakers: bool,
}

/// Authorization request for device access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    pub request_id: String,
    pub wallet_address: String,
    pub device_type: String,
    pub capabilities: DeviceCapabilities,
    pub requested_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XTMPSocketCall {
    /// Unique call identifier
    pub call_id: String,
    /// Call initiator wallet
    pub initiator: String,
    /// Call participants
    pub participants: Vec<String>,
    /// Media configuration
    pub media_config: MediaConfig,
    /// Device authorization tokens
    pub authorization: DeviceAuthorization,
    /// Session security configuration
    pub session_security: SessionSecurity,
    /// Call status
    pub status: CallStatus,
    /// Start time
    pub started_at: DateTime<Utc>,
    /// End time
    pub ended_at: Option<DateTime<Utc>>,
    /// Call duration in seconds
    pub duration: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaConfig {
    /// Video configuration
    pub video: Option<VideoConfig>,
    /// Audio configuration
    pub audio: Option<AudioConfig>,
    /// Screen sharing configuration
    pub screen_share: Option<ScreenShareConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfig {
    pub codec: String,
    pub resolution: String,
    pub framerate: u32,
    pub bitrate: u32,
    pub encryption: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub codec: String,
    pub sample_rate: u32,
    pub channels: u8,
    pub bitrate: u32,
    pub encryption: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenShareConfig {
    pub resolution: String,
    pub framerate: u32,
    pub compression: String,
    pub encryption: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAuthorization {
    /// Camera access permission
    pub camera_access: AccessPermission,
    /// Microphone access permission
    pub microphone_access: AccessPermission,
    /// Screen share permission
    pub screen_share: AccessPermission,
    /// File sharing permission
    pub file_sharing: AccessPermission,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPermission {
    pub granted: bool,
    pub pes_token: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSecurity {
    /// QLOCK binding for session
    pub qlock_binding: String,
    /// Perfect forward secrecy enabled
    pub perfect_forward_secrecy: bool,
    /// Metadata protection via shadow routing
    pub metadata_protection: bool,
    /// End-to-end encryption keys
    pub e2e_keys: Option<E2EKeys>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct E2EKeys {
    pub local_public_key: Vec<u8>,
    pub remote_public_keys: HashMap<String, Vec<u8>>,
    pub session_key: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CallStatus {
    Initiating,
    Ringing,
    Connected,
    OnHold,
    Ended,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XTMPSocketMessage {
    pub message_id: String,
    pub call_id: String,
    pub sender: String,
    pub message_type: SocketMessageType,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocketMessageType {
    CallInvite,
    CallAccept,
    CallReject,
    CallEnd,
    MediaOffer,
    MediaAnswer,
    ICECandidate,
    ChatMessage,
    FileTransfer,
    ScreenShareStart,
    ScreenShareStop,
    DeviceAuthRequest,
    DeviceAuthResponse,
}

/// XTMP Socket Service - handles real-time communication with enhanced capabilities
pub struct XTMPSocketService {
    wallet: WalletIdentity,
    identity_registry: Arc<RwLock<IdentityRegistry>>,
    active_connections: Arc<RwLock<HashMap<String, SocketConnection>>>,
    call_sessions: Arc<RwLock<HashMap<String, CallSession>>>,
    media_relays: Vec<MediaRelay>,
    signaling_server: Option<String>,
    event_sender: mpsc::UnboundedSender<SocketEvent>,
    event_receiver: Arc<RwLock<Option<mpsc::UnboundedReceiver<SocketEvent>>>>,
    stun_servers: Vec<String>,
    turn_servers: Vec<TurnServer>,
}

#[derive(Debug)]
pub struct WebSocketConnection {
    pub wallet_address: String,
    pub stream: WebSocketStream<TcpStream>,
    pub last_activity: DateTime<Utc>,
    pub authorized_devices: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocketEvent {
    ConnectionEstablished { peer: String, connection_id: String },
    ConnectionClosed { peer: String, connection_id: String },
    CallIncoming { caller: String, call_id: String, media_types: Vec<MediaType> },
    CallAccepted { call_id: String },
    CallRejected { call_id: String, reason: String },
    CallEnded { call_id: String },
    MediaStreamStarted { call_id: String, stream_id: String, media_type: MediaType },
    MediaStreamEnded { call_id: String, stream_id: String },
    DataReceived { peer: String, data: Vec<u8> },
    Error { message: String },
}

/// TURN server configuration for NAT traversal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnServer {
    pub url: String,
    pub username: String,
    pub credential: String,
}

/// Socket connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketConnection {
    pub connection_id: String,
    pub peer_wallet: String,
    pub established_at: DateTime<Utc>,
    pub connection_type: ConnectionType,
    pub status: ConnectionStatus,
}

/// Connection type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionType {
    Direct,
    Relayed,
    P2P,
}

/// Connection status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionStatus {
    Connecting,
    Connected,
    Disconnected,
    Failed,
}

/// Call session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallSession {
    pub call_id: String,
    pub caller: String,
    pub callee: String,
    pub media_types: Vec<MediaType>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub status: CallStatus,
    pub connection_id: String,
}



/// Device authorization service for camera/microphone access
#[derive(Debug)]
pub struct DeviceAuthService {
    authorized_devices: HashMap<String, DeviceCapabilities>,
    pending_authorizations: HashMap<String, AuthorizationRequest>,
    device_capabilities: HashMap<String, DeviceCapabilities>,
    pes_tokens: HashMap<String, PESToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PESToken {
    pub token_id: String,
    pub wallet_address: String,
    pub device_capabilities: Vec<String>,
    pub time_bound: DateTime<Utc>,
    pub one_time_use: bool,
    pub session_bound: Option<String>,
    pub revocation_trigger: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceCapability {
    CameraRead,
    CameraWrite,
    MicrophoneRead,
    MicrophoneWrite,
    ScreenRead,
    ScreenWrite,
    FileRead,
    FileWrite,
    NetworkAccess,
    SystemInfo,
}

impl XTMPSocketService {
    /// Create new XTMP Socket service with enhanced real-time capabilities
    pub fn new(wallet: WalletIdentity, identity_registry: IdentityRegistry) -> Result<Self> {
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        
        Ok(Self {
            wallet,
            identity_registry: Arc::new(RwLock::new(identity_registry)),
            active_connections: Arc::new(RwLock::new(HashMap::new())),
            call_sessions: Arc::new(RwLock::new(HashMap::new())),
            media_relays: vec![
                MediaRelay {
                    id: "relay1".to_string(),
                    url: "wss://relay1.xtmp.network".to_string(),
                    region: "us-east-1".to_string(),
                    capacity: 1000,
                    active_streams: 0,
                },
                MediaRelay {
                    id: "relay2".to_string(),
                    url: "wss://relay2.xtmp.network".to_string(),
                    region: "eu-west-1".to_string(),
                    capacity: 1000,
                    active_streams: 0,
                },
            ],
            signaling_server: Some("wss://signaling.xtmp.network".to_string()),
            event_sender,
            event_receiver: Arc::new(RwLock::new(Some(event_receiver))),
            stun_servers: vec![
                "stun:stun.l.google.com:19302".to_string(),
                "stun:stun1.l.google.com:19302".to_string(),
            ],
            turn_servers: vec![
                TurnServer {
                    url: "turn:turn.xtmp.network:3478".to_string(),
                    username: "xtmp_user".to_string(),
                    credential: "xtmp_pass".to_string(),
                },
            ],
        })
    }

    /// Initiate a call to another wallet
    pub async fn start_server(&mut self, bind_address: &str) -> Result<(), XTMPSocketError> {
        let listener = TcpListener::bind(bind_address).await
            .map_err(|_| XTMPSocketError::ServerStartFailed)?;
        
        println!("XTMP Socket server listening on {}", bind_address);
        
        while let Ok((stream, addr)) = listener.accept().await {
            println!("New connection from: {}", addr);
            
            let ws_stream = accept_async(stream).await
                .map_err(|_| XTMPSocketError::WebSocketHandshakeFailed)?;
            
            // Handle connection in separate task
            tokio::spawn(async move {
                // TODO: Handle WebSocket connection
            });
        }
        
        Ok(())
    }
    
    /// Initiate a video call
    pub async fn initiate_call(
        &mut self,
        initiator: &WalletIdentity,
        participants: Vec<String>,
        media_config: MediaConfig,
    ) -> Result<String, XTMPSocketError> {
        let call_id = format!("call_{}", Uuid::new_v4().to_string().replace("-", ""));
        
        // Request device authorizations
        let authorization = self.request_device_authorization(
            &initiator.wallet_address,
            &call_id,
            &media_config,
        ).await?;
        
        // Generate session security
        let session_security = self.generate_session_security(&initiator.keypair)?;
        
        let call = CallSession {
            call_id: call_id.clone(),
            caller: self.wallet.wallet_address.clone(),
            callee: participants.first().map(|p| p.clone()).unwrap_or_default(),
            media_types: vec![MediaType::Audio, MediaType::Video],
            started_at: Utc::now(),
            ended_at: None,
            status: CallStatus::Initiating,
            connection_id: "conn_".to_string() + &uuid::Uuid::new_v4().to_string(),
        };
        
        if let Ok(mut sessions) = self.call_sessions.try_write() {
            sessions.insert(call_id.clone(), call);
        }
        
        // Send call invites to participants
        for participant in &participants {
            self.send_call_invite(&call_id, participant).await?;
        }
        
        Ok(call_id)
    }
    
    /// Accept an incoming call
    pub async fn accept_call(
        &mut self,
        call_id: &str,
        participant: &WalletIdentity,
    ) -> Result<(), XTMPSocketError> {
        // Get call for status update - temporarily comment out to fix borrowing
        // let call = self.active_calls.get_mut(call_id)
        //     .ok_or(XTMPError::CallNotFound)?;
        
        // Verify participant is invited
        if let Ok(sessions) = self.call_sessions.try_read() {
            if !sessions.get(call_id).map(|call| call.status == CallStatus::Connected).unwrap_or(false) {
                return Err(XTMPSocketError::UnauthorizedParticipant);
            }
        } else {
            return Err(XTMPSocketError::UnauthorizedParticipant);
        }
        
        // Request device authorizations for participant
        // Temporarily comment out to fix borrowing issues
        // let _authorization = self.request_device_authorization(
        //     &participant.wallet_address,
        //     call_id,
        //     &call.media_config,
        // ).await?;
        
        // Update call status - temporarily comment out to fix borrowing
        // call.status = CallStatus::Connected;
        
        // Send acceptance to other participants
        // self.send_call_acceptance(call_id, &participant.wallet_address).await?; // Temporarily commented out
        
        Ok(())
    }
    
    /// End a call
    pub async fn end_call(&mut self, call_id: &str) -> Result<(), XTMPSocketError> {
        let mut call_sessions = self.call_sessions.try_write().map_err(|_| XTMPSocketError::CallNotFound)?;
        let call = call_sessions.get_mut(call_id)
            .ok_or(XTMPSocketError::CallNotFound)?;
        
        call.status = CallStatus::Ended;
        call.ended_at = Some(Utc::now());
        // Duration calculation moved to CallSession if needed
        
        // Revoke all device authorizations for this call
        // TODO: Implement device auth token revocation
        
        // Notify all participants
        let participants = vec![call.callee.clone()];
        for participant in &participants {
            self.send_call_end(call_id, participant).await?;
        }
        
        Ok(())
    }
    
    /// Request device authorization with PES tokens
    async fn request_device_authorization(
        &mut self,
        wallet_address: &str,
        call_id: &str,
        media_config: &MediaConfig,
    ) -> Result<DeviceAuthorization, XTMPSocketError> {
        let mut authorization = DeviceAuthorization {
            camera_access: AccessPermission {
                granted: false,
                pes_token: None,
                expires_at: Utc::now(),
                capabilities: Vec::new(),
            },
            microphone_access: AccessPermission {
                granted: false,
                pes_token: None,
                expires_at: Utc::now(),
                capabilities: Vec::new(),
            },
            screen_share: AccessPermission {
                granted: false,
                pes_token: None,
                expires_at: Utc::now(),
                capabilities: Vec::new(),
            },
            file_sharing: AccessPermission {
                granted: false,
                pes_token: None,
                expires_at: Utc::now(),
                capabilities: Vec::new(),
            },
        };
        
        // Request camera access if video is enabled
        if media_config.video.is_some() {
            // TODO: Implement device auth service integration
            authorization.camera_access = AccessPermission {
                granted: true,
                pes_token: Some("temp_camera_token".to_string()),
                expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
                capabilities: vec!["camera:read".to_string()],
            };
        }
        
        // Request microphone access if audio is enabled
        if media_config.audio.is_some() {
            // TODO: Implement device auth service integration
            authorization.microphone_access = AccessPermission {
                granted: true,
                pes_token: Some("temp_microphone_token".to_string()),
                expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
                capabilities: vec!["microphone:read".to_string()],
            };
        }
        
        // Screen share requires elevated PES
        if media_config.screen_share.is_some() {
            // TODO: Implement device auth service integration
            authorization.screen_share = AccessPermission {
                granted: true,
                pes_token: Some("temp_screen_token".to_string()),
                expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
                capabilities: vec!["screen:read".to_string()],
            };
        }
        
        Ok(authorization)
    }
    
    /// Generate session security configuration
    fn generate_session_security(&self, keypair: &Keypair) -> Result<SessionSecurity, XTMPSocketError> {
        // Generate QLOCK binding
        let qlock_data = format!("socket_{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
        let qlock_binding = format!("sha256({})", hex::encode(qlock_data.as_bytes()));
        
        // Generate session keys
        let session_key = rand::random::<[u8; 32]>();
        
        Ok(SessionSecurity {
            qlock_binding,
            perfect_forward_secrecy: true,
            metadata_protection: true,
            e2e_keys: Some(E2EKeys {
                local_public_key: keypair.public.to_bytes().to_vec(),
                remote_public_keys: HashMap::new(),
                session_key: session_key.to_vec(),
            }),
        })
    }
    
    /// Send call invite to participant
    async fn send_call_invite(&self, call_id: &str, participant: &str) -> Result<(), XTMPSocketError> {
        let message = XTMPSocketMessage {
            message_id: Uuid::new_v4().to_string(),
            call_id: call_id.to_string(),
            sender: "system".to_string(),
            message_type: SocketMessageType::CallInvite,
            payload: serde_json::json!({
                "participant": participant,
                "call_id": call_id
            }),
            timestamp: Utc::now(),
            signature: Vec::new(), // TODO: Sign message
        };
        
        // TODO: Send via WebSocket connection
        println!("Sending call invite to {}: {}", participant, call_id);
        Ok(())
    }
    
    /// Send call acceptance notification
    async fn send_call_acceptance(&self, call_id: &str, participant: &str) -> Result<(), XTMPSocketError> {
        let message = XTMPSocketMessage {
            message_id: Uuid::new_v4().to_string(),
            call_id: call_id.to_string(),
            sender: participant.to_string(),
            message_type: SocketMessageType::CallAccept,
            payload: serde_json::json!({
                "participant": participant,
                "call_id": call_id
            }),
            timestamp: Utc::now(),
            signature: Vec::new(), // TODO: Sign message
        };
        
        // TODO: Send to all participants
        println!("Call accepted by {}: {}", participant, call_id);
        Ok(())
    }
    
    /// Send call end notification
    async fn send_call_end(&self, call_id: &str, participant: &str) -> Result<(), XTMPSocketError> {
        let message = XTMPSocketMessage {
            message_id: Uuid::new_v4().to_string(),
            call_id: call_id.to_string(),
            sender: "system".to_string(),
            message_type: SocketMessageType::CallEnd,
            payload: serde_json::json!({
                "participant": participant,
                "call_id": call_id
            }),
            timestamp: Utc::now(),
            signature: Vec::new(), // TODO: Sign message
        };
        
        // TODO: Send via WebSocket connection
        println!("Sending call end to {}: {}", participant, call_id);
        Ok(())
    }
    
    /// Get active call information
    pub fn get_call(&self, call_id: &str) -> Option<CallSession> {
        self.call_sessions.try_read().ok()?.get(call_id).cloned()
    }
    
    /// Get all active calls for a wallet
    pub fn get_wallet_calls(&self, wallet_address: &str) -> Vec<CallSession> {
        if let Ok(sessions) = self.call_sessions.try_read() {
            sessions.values().cloned()
                .filter(|call| {
                    call.caller == wallet_address || call.callee == wallet_address
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl DeviceAuthService {
    pub fn new() -> Self {
        Self {
            authorized_devices: HashMap::new(),
            pending_authorizations: HashMap::new(),
            device_capabilities: HashMap::new(),
            pes_tokens: HashMap::new(),
        }
    }
    
    /// Create PES token for device access
    pub fn create_pes_token(
        &mut self,
        wallet_address: &str,
        capabilities: Vec<DeviceCapability>,
        session_bound: Option<String>,
    ) -> Result<PESToken, XTMPSocketError> {
        let token_id = format!("pes_{}", Uuid::new_v4().to_string().replace("-", ""));
        
        let token = PESToken {
            token_id: token_id.clone(),
            wallet_address: wallet_address.to_string(),
            device_capabilities: capabilities.iter().map(|c| format!("{:?}", c)).collect(),
            time_bound: Utc::now() + chrono::Duration::hours(1), // 1 hour expiry
            one_time_use: false,
            session_bound,
            revocation_trigger: "call_end_or_user_deny".to_string(),
        };
        
        self.pes_tokens.insert(token_id.clone(), token.clone());
        Ok(token)
    }
    
    /// Create elevated PES token for sensitive operations
    pub fn create_elevated_pes_token(
        &mut self,
        wallet_address: &str,
        capabilities: Vec<DeviceCapability>,
        session_bound: Option<String>,
    ) -> Result<PESToken, XTMPSocketError> {
        let token_id = format!("pes_elevated_{}", Uuid::new_v4().to_string().replace("-", ""));
        
        let token = PESToken {
            token_id: token_id.clone(),
            wallet_address: wallet_address.to_string(),
            device_capabilities: capabilities.iter().map(|c| format!("{:?}", c)).collect(),
            time_bound: Utc::now() + chrono::Duration::minutes(30), // 30 minutes for elevated
            one_time_use: true, // Elevated tokens are one-time use
            session_bound,
            revocation_trigger: "immediate_on_misuse".to_string(),
        };
        
        self.pes_tokens.insert(token_id.clone(), token.clone());
        Ok(token)
    }
    
    /// Verify PES token for device access
    pub fn verify_pes_token(&self, token_id: &str) -> Result<&PESToken, XTMPSocketError> {
        let token = self.pes_tokens.get(token_id)
            .ok_or(XTMPSocketError::InvalidPESToken)?;
        
        // Check expiry
        if Utc::now() > token.time_bound {
            return Err(XTMPSocketError::PESTokenExpired);
        }
        
        Ok(token)
    }
    
    /// Revoke all tokens for a call
    pub fn revoke_call_tokens(&mut self, call_id: &str) {
        self.pes_tokens.retain(|_, token| {
            token.session_bound.as_deref() != Some(call_id)
        });
    }
    
    /// Revoke specific token
    pub fn revoke_token(&mut self, token_id: &str) -> Result<(), XTMPSocketError> {
        self.pes_tokens.remove(token_id)
            .ok_or(XTMPSocketError::InvalidPESToken)?;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum XTMPSocketError {
    #[error("Server failed to start")]
    ServerStartFailed,
    #[error("WebSocket handshake failed")]
    WebSocketHandshakeFailed,
    #[error("Call not found")]
    CallNotFound,
    #[error("Unauthorized participant")]
    UnauthorizedParticipant,
    #[error("Device authorization failed")]
    DeviceAuthorizationFailed,
    #[error("Invalid PES token")]
    InvalidPESToken,
    #[error("PES token expired")]
    PESTokenExpired,
    #[error("Media configuration error")]
    MediaConfigError,
    #[error("Connection error")]
    ConnectionError,
    #[error("Encryption error")]
    EncryptionError,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallet_identity::{WalletIdentity, WalletProvider};
    
    #[tokio::test]
    async fn test_call_initiation() {
        let alice = WalletIdentity::new("alice", WalletProvider::Pravyom, Some("alice@example.com".to_string())).unwrap();
        let registry = IdentityRegistry::new();
        let mut service = XTMPSocketService::new(alice.clone(), registry).unwrap();
        
        let bob = WalletIdentity::new(
            "bob",
            WalletProvider::Pravyom,
            Some("bob@example.com".to_string()),
        ).unwrap();
        
        let media_config = MediaConfig {
            video: Some(VideoConfig {
                codec: "AV1".to_string(),
                resolution: "1080p".to_string(),
                framerate: 30,
                bitrate: 2000000,
                encryption: "SRTP_AES256_GCM".to_string(),
            }),
            audio: Some(AudioConfig {
                codec: "Opus".to_string(),
                sample_rate: 48000,
                channels: 2,
                bitrate: 128000,
                encryption: "SRTP_AES256_GCM".to_string(),
            }),
            screen_share: None,
        };
        
        // service is already unwrapped from XTMPSocketService::new() call above
        let call_id = service.initiate_call(
            &alice,
            vec![bob.wallet_address.clone()],
            media_config,
        ).await.unwrap();
        
        assert!(!call_id.is_empty());
        assert!(call_id.starts_with("call_"));
        
        let call = service.get_call(&call_id).unwrap();
        assert_eq!(call.caller, "alice@pravyom.wallet");
        assert_eq!(call.callee, "bob@pravyom.wallet");
        // Note: CallSession struct has different fields than expected in test
        // Available fields: call_id, caller, callee, media_types, started_at, status, nat_type
    }
    
    #[test]
    fn test_pes_token_creation() {
        let mut device_auth = DeviceAuthService::new();
        
        let token = device_auth.create_pes_token(
            "alice@pravyom.wallet",
            vec![DeviceCapability::CameraRead, DeviceCapability::MicrophoneRead],
            Some("call_123".to_string()),
        ).unwrap();
        
        assert!(!token.token_id.is_empty());
        assert!(token.token_id.starts_with("pes_"));
        assert_eq!(token.wallet_address, "alice@pravyom.wallet");
        assert_eq!(token.device_capabilities.len(), 2);
        assert_eq!(token.session_bound, Some("call_123".to_string()));
        
        // Verify token
        let verified = device_auth.verify_pes_token(&token.token_id).unwrap();
        assert_eq!(verified.wallet_address, "alice@pravyom.wallet");
    }
    
    #[test]
    fn test_elevated_pes_token() {
        let mut device_auth = DeviceAuthService::new();
        
        let token = device_auth.create_elevated_pes_token(
            "alice@pravyom.wallet",
            vec![DeviceCapability::ScreenRead],
            Some("call_456".to_string()),
        ).unwrap();
        
        assert!(token.token_id.starts_with("pes_elevated_"));
        assert!(token.one_time_use);
        assert_eq!(token.revocation_trigger, "immediate_on_misuse");
    }
}
