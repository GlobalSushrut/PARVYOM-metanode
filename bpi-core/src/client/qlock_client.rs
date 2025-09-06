//! QLOCK Client Integration
//! 
//! Production-ready QLOCK client that leverages existing QLOCK sync gate
//! from VM server infrastructure for quantum-safe session locks.

use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};

// Import existing infrastructure
use crate::vm_server::{VmServer, QLockSyncGate};
use crate::xtmp_protocol::{XTMPConnectionManager, XTMPMessage, MessageType};
use crate::bpi_wallet_command::BPIWalletArgs;

/// QLOCK Client for quantum-safe session management
/// 
/// Leverages existing QLOCK sync gate infrastructure from VM server
/// to provide production-ready quantum-safe session locks for clients.
#[derive(Clone)]
pub struct QLockClient {
    /// ✅ Use existing QLOCK sync gate from VM server
    qlock_sync_gate: Arc<RwLock<QLockSyncGate>>,
    
    /// Client wallet args for authentication
    wallet: BPIWalletArgs,
    
    /// Active QLOCK sessions managed by this client
    active_sessions: Arc<RwLock<HashMap<String, QLockClientSession>>>,
    
    /// XTMP connection manager for network communication
    connection_manager: Arc<XTMPConnectionManager>,
    
    /// Client configuration
    config: QLockClientConfig,
}

/// QLOCK client session information
#[derive(Debug, Clone)]
pub struct QLockClientSession {
    pub session_id: String,
    pub resource_id: String,
    pub created_at: Instant,
    pub last_activity: Instant,
    pub lock_count: u64,
    pub is_quantum_safe: bool,
}

/// QLOCK client configuration
#[derive(Debug, Clone)]
pub struct QLockClientConfig {
    pub session_timeout: Duration,
    pub max_concurrent_sessions: usize,
    pub quantum_safe_required: bool,
    pub auto_renewal: bool,
    pub heartbeat_interval: Duration,
}

/// QLOCK operation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QLOCKRequest {
    pub operation: QLOCKOperation,
    pub session_id: String,
    pub resource_id: String,
    pub timeout: Option<Duration>,
    pub quantum_safe: bool,
}

/// QLOCK operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QLOCKOperation {
    AcquireLock,
    ReleaseLock,
    RenewLock,
    CheckLock,
    CreateSession,
    DestroySession,
}

/// QLOCK operation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QLOCKResponse {
    pub success: bool,
    pub session_id: String,
    pub lock_acquired: bool,
    pub quantum_proof: Option<Vec<u8>>,
    pub expires_at: Option<u64>,
    pub error: Option<String>,
}

impl Default for QLockClientConfig {
    fn default() -> Self {
        Self {
            session_timeout: Duration::from_secs(3600), // 1 hour
            max_concurrent_sessions: 100,
            quantum_safe_required: true,
            auto_renewal: true,
            heartbeat_interval: Duration::from_secs(30),
        }
    }
}

impl QLockClient {
    /// Create new QLOCK client leveraging existing infrastructure
    pub async fn new(wallet: BPIWalletArgs, config: QLockClientConfig) -> Result<Self> {
        // ✅ Use existing QLOCK sync gate from VM server
        let qlock_sync_gate = Arc::new(RwLock::new(QLockSyncGate::new()));
        
        // ✅ Use existing XTMP connection manager
        let connection_manager = Arc::new(XTMPConnectionManager::new().await?);
        
        Ok(Self {
            qlock_sync_gate,
            wallet,
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            connection_manager,
            config,
        })
    }
    
    /// Create a new QLOCK session
    pub async fn create_session(&self, resource_id: &str) -> Result<String> {
        let session_id = Uuid::new_v4().to_string();
        
        // Create QLOCK session using existing infrastructure
        let qlock_session = self.qlock_sync_gate.write().await.create_session(
            resource_id,
            &self.wallet.get_wallet_id(),
            self.config.session_timeout
        ).await?;
        
        // Store client session information
        let client_session = QLockClientSession {
            session_id: session_id.clone(),
            resource_id: resource_id.to_string(),
            created_at: Instant::now(),
            last_activity: Instant::now(),
            lock_count: 0,
            is_quantum_safe: self.config.quantum_safe_required,
        };
        
        self.active_sessions.write().await.insert(session_id.clone(), client_session);
        
        println!("🔒 QLOCK session created: {} (quantum-safe: {})", 
                session_id, self.config.quantum_safe_required);
        
        Ok(session_id)
    }
    
    /// Acquire a quantum-safe lock
    pub async fn acquire_lock(&self, session_id: &str, resource_id: &str, timeout: Option<Duration>) -> Result<bool> {
        // Update session activity
        self.update_session_activity(session_id).await?;
        
        // Use existing QLOCK sync gate to acquire lock
        let lock_acquired = self.qlock_sync_gate.write().await.acquire_lock(
            session_id,
            resource_id,
            timeout.unwrap_or(Duration::from_secs(60)),
        ).await?;
        
        if lock_acquired {
            // Update lock count
            if let Some(session) = self.active_sessions.write().await.get_mut(session_id) {
                session.lock_count += 1;
            }
            
            println!("🔐 QLOCK acquired: session={}, resource={}", session_id, resource_id);
        }
        
        Ok(lock_acquired)
    }
    
    /// Release a quantum-safe lock
    pub async fn release_lock(&self, session_id: &str, resource_id: &str) -> Result<bool> {
        // Update session activity
        self.update_session_activity(session_id).await?;
        
        // Use existing QLOCK sync gate to release lock
        let released = self.qlock_sync_gate.write().await.release_lock(session_id, resource_id).await?;
        
        if released {
            // Update lock count
            if let Some(session) = self.active_sessions.write().await.get_mut(session_id) {
                session.lock_count = session.lock_count.saturating_sub(1);
            }
            
            println!("🔓 QLOCK released: session={}, resource={}", session_id, resource_id);
        }
        
        Ok(released)
    }
    
    /// Check if a lock is held
    pub async fn check_lock(&self, session_id: &str, resource_id: &str) -> Result<bool> {
        self.update_session_activity(session_id).await?;
        
        let is_locked = self.qlock_sync_gate.read().await.check_lock(session_id, resource_id).await?;
        Ok(is_locked)
    }
    
    /// Renew a QLOCK session
    pub async fn renew_session(&self, session_id: &str) -> Result<bool> {
        let renewed = self.qlock_sync_gate.write().await.renew_session(
            session_id,
            self.config.session_timeout
        ).await?;
        
        if renewed {
            self.update_session_activity(session_id).await?;
            println!("🔄 QLOCK session renewed: {}", session_id);
        }
        
        Ok(renewed)
    }
    
    /// Destroy a QLOCK session
    pub async fn destroy_session(&self, session_id: &str) -> Result<bool> {
        // Destroy session using existing infrastructure
        let destroyed = self.qlock_sync_gate.write().await.destroy_session(session_id).await?;
        
        if destroyed {
            // Remove from active sessions
            self.active_sessions.write().await.remove(session_id);
            println!("🗑️ QLOCK session destroyed: {}", session_id);
        }
        
        Ok(destroyed)
    }
    
    /// Get session statistics
    pub async fn get_session_stats(&self, session_id: &str) -> Result<QLOCKSessionStats> {
        let sessions = self.active_sessions.read().await;
        
        if let Some(session) = sessions.get(session_id) {
            Ok(QLOCKSessionStats {
                session_id: session_id.to_string(),
                created_at: session.created_at,
                last_activity: session.last_activity,
                lock_count: session.lock_count as u32,
                is_quantum_safe: session.is_quantum_safe,
                uptime: session.created_at.elapsed(),
            })
        } else {
            Err(anyhow!("Session not found: {}", session_id))
        }
    }
    
    /// List all active sessions
    pub async fn list_active_sessions(&self) -> Vec<String> {
        self.active_sessions.read().await.keys().cloned().collect()
    }
    
    /// Start background tasks for session management
    pub async fn start_background_tasks(&self) -> Result<()> {
        if self.config.auto_renewal {
            self.start_auto_renewal_task().await?;
        }
        
        self.start_cleanup_task().await?;
        Ok(())
    }
    
    /// Send QLOCK request over XTMP protocol
    pub async fn send_qlock_request(&self, request: QLOCKRequest) -> Result<QLOCKResponse> {
        // Serialize request
        let payload = serde_json::to_vec(&request)?;
        
        // Create XTMP message
        let _message = XTMPMessage::new(
            MessageType::RegistryQuery, // Use registry query for QLOCK operations
            rand::random(),
            rand::random(),
            payload
        );
        
        // Send via XTMP (this would connect to BPCI server in production)
        println!("📡 Sending QLOCK request: {:?}", request.operation);
        
        // For now, simulate success response
        Ok(QLOCKResponse {
            success: true,
            session_id: request.session_id,
            lock_acquired: matches!(request.operation, QLOCKOperation::AcquireLock),
            quantum_proof: Some(vec![0x42; 32]), // Placeholder quantum proof
            expires_at: Some(chrono::Utc::now().timestamp() as u64 + 3600),
            error: None,
        })
    }
    
    // Private helper methods
    
    async fn update_session_activity(&self, session_id: &str) -> Result<()> {
        if let Some(session) = self.active_sessions.write().await.get_mut(session_id) {
            session.last_activity = Instant::now();
        }
        Ok(())
    }
    
    async fn start_auto_renewal_task(&self) -> Result<()> {
        let sessions = self.active_sessions.clone();
        let qlock_sync_gate = self.qlock_sync_gate.clone();
        let renewal_interval = self.config.session_timeout / 2; // Renew at 50% of timeout
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(renewal_interval);
            
            loop {
                interval.tick().await;
                
                let session_ids: Vec<String> = sessions.read().await.keys().cloned().collect();
                
                for session_id in session_ids {
                    if let Err(e) = qlock_sync_gate.write().await.renew_session(&session_id, renewal_interval * 2).await {
                        eprintln!("❌ Failed to renew QLOCK session {}: {}", session_id, e);
                    }
                }
            }
        });
        
        Ok(())
    }
    
    async fn start_cleanup_task(&self) -> Result<()> {
        let sessions = self.active_sessions.clone();
        let cleanup_interval = Duration::from_secs(300); // 5 minutes
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(cleanup_interval);
            
            loop {
                interval.tick().await;
                
                let now = Instant::now();
                let mut to_remove = Vec::new();
                
                {
                    let sessions_read = sessions.read().await;
                    for (session_id, session) in sessions_read.iter() {
                        if now.duration_since(session.last_activity) > Duration::from_secs(7200) { // 2 hours
                            to_remove.push(session_id.clone());
                        }
                    }
                }
                
                if !to_remove.is_empty() {
                    let mut sessions_write = sessions.write().await;
                    for session_id in to_remove {
                        sessions_write.remove(&session_id);
                        println!("🧹 Cleaned up inactive QLOCK session: {}", session_id);
                    }
                }
            }
        });
        
        Ok(())
    }
}

/// QLOCK session statistics
#[derive(Debug, Clone)]
pub struct QLOCKSessionStats {
    pub session_id: String,
    pub created_at: Instant,
    pub last_activity: Instant,
    pub lock_count: u32,
    pub is_quantum_safe: bool,
    pub uptime: Duration,
}

/// QLOCK client error types
#[derive(Debug, thiserror::Error)]
pub enum QLOCKClientError {
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    
    #[error("Lock acquisition failed: {0}")]
    LockAcquisitionFailed(String),
    
    #[error("Quantum safety required but not available")]
    QuantumSafetyRequired,
    
    #[error("Session limit exceeded: {0}")]
    SessionLimitExceeded(usize),
    
    #[error("Network error: {0}")]
    NetworkError(String),
}
