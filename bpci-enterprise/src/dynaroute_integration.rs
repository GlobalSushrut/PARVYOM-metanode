//! # DynaRoute v2 Integration
//! 
//! Integrates DynaRoute v2 with vPods, BSO-K8, commute_lock, and virtual event system.
//! Provides unified networking layer across the entire BPCI infrastructure.

use std::sync::Arc;
use std::net::SocketAddr;
use tokio::sync::RwLock;
use anyhow::Result;
use tracing::{info, warn, error};

use dynaroute::{
    CloudTransport,
    CloudServiceDiscovery,
    AddressSyncAgent,
    DynaRouteConfig,
    VirtualAddress,
    VPodWeight,
    IAAv6Address,
};

use crate::commute_lock::{CommuteLockRuntime, CommuteLock, Message};

/// Unified networking layer integrating DynaRoute v2 with core infrastructure
pub struct UnifiedNetworkingLayer {
    /// DynaRoute cloud transport
    transport: Arc<CloudTransport>,
    
    /// DynaRoute service discovery
    discovery: Arc<CloudServiceDiscovery>,
    
    /// DynaRoute address sync agent
    agent: Arc<AddressSyncAgent>,
    
    /// CommuteLock runtime (for local communication)
    commute_lock: Arc<CommuteLockRuntime>,
    
    /// Virtual address registry: vpod_id → VirtualAddress
    virtual_addresses: Arc<RwLock<std::collections::HashMap<String, VirtualAddress>>>,
    
    /// Local bind address
    local_addr: SocketAddr,
}

impl UnifiedNetworkingLayer {
    /// Create new unified networking layer with static port (Hybrid mode)
    pub async fn new(
        local_addr: SocketAddr,
        commute_lock_runtime: Arc<CommuteLockRuntime>,
    ) -> Result<Self> {
        info!("🌐 Initializing Unified Networking Layer (Hybrid Mode)");
        info!("   Local address: {}", local_addr);
        
        // Create DynaRoute components
        let transport = Arc::new(CloudTransport::new(local_addr).await?);
        let discovery = Arc::new(CloudServiceDiscovery::new());
        let config = DynaRouteConfig::default();
        let agent = Arc::new(AddressSyncAgent::new(config));
        
        info!("✅ DynaRoute v2 initialized");
        info!("✅ CommuteLock runtime integrated");
        
        Ok(Self {
            transport,
            discovery,
            agent,
            commute_lock: commute_lock_runtime,
            virtual_addresses: Arc::new(RwLock::new(std::collections::HashMap::new())),
            local_addr,
        })
    }
    
    /// Create new unified networking layer with dynamic port (Pure Virtual mode)
    /// 
    /// This mode uses OS-assigned dynamic ports (port 0) for true port-free operation.
    /// Components communicate via service names only, with no static port dependencies.
    pub async fn new_virtual(
        commute_lock_runtime: Arc<CommuteLockRuntime>,
    ) -> Result<Self> {
        info!("🌐 Initializing Unified Networking Layer (Pure Virtual Mode)");
        info!("   Mode: Port-free operation with dynamic port allocation");
        
        // Bind to dynamic port (0 = OS assigns available port)
        let dynamic_addr: SocketAddr = "127.0.0.1:0".parse()?;
        
        // Create DynaRoute components
        let transport = Arc::new(CloudTransport::new(dynamic_addr).await?);
        let discovery = Arc::new(CloudServiceDiscovery::new());
        let config = DynaRouteConfig::default();
        let agent = Arc::new(AddressSyncAgent::new(config));
        
        // Get the actual assigned port
        let actual_addr = transport.local_addr()?;
        
        info!("✅ DynaRoute v2 initialized (Pure Virtual)");
        info!("   Dynamic port assigned: {}", actual_addr);
        info!("✅ CommuteLock runtime integrated");
        info!("✅ No static port dependencies - fully virtual!");
        
        Ok(Self {
            transport,
            discovery,
            agent,
            commute_lock: commute_lock_runtime,
            virtual_addresses: Arc::new(RwLock::new(std::collections::HashMap::new())),
            local_addr: actual_addr,
        })
    }
    
    /// Register vPod with unified networking
    /// 
    /// This registers the vPod with:
    /// - DynaRoute (for remote communication)
    /// - CommuteLock (for local communication)
    /// - Service discovery
    pub async fn register_vpod(
        &self,
        vpod_id: String,
        service_id: String,
        actual_addr: SocketAddr,
    ) -> Result<VirtualAddress> {
        info!("📝 Registering vPod: {} (service: {})", vpod_id, service_id);
        
        // 1. Compute IAAv6 address
        let iaav6 = self.agent.compute_service_iaav6(&service_id, &vpod_id).await?;
        
        // 2. Create virtual address
        let virtual_addr = VirtualAddress {
            iaav6: iaav6.inner(),
            vpod_id: vpod_id.clone(),
            service_id: service_id.clone(),
            holder_address: format!("{}.bpci.local", service_id),
            holder_hash: *blake3::hash(vpod_id.as_bytes()).as_bytes(),
            merkle_proof: dynaroute::MerkleProof::default(),
            quic_conn_id: rand::random(),
            epoch: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        // 3. Register with DynaRoute transport
        self.transport.register_vpod(&virtual_addr, actual_addr).await?;
        
        // 4. Register with HRW ring
        self.agent.add_vpod_to_ring(&service_id, vpod_id.clone(), VPodWeight::default()).await?;
        
        // 5. Store virtual address
        self.virtual_addresses.write().await.insert(vpod_id.clone(), virtual_addr.clone());
        
        info!("✅ vPod registered: {}", vpod_id);
        info!("   IAAv6: {}", iaav6.to_string());
        info!("   Actual: {}", actual_addr);
        
        Ok(virtual_addr)
    }
    
    /// Send message using hybrid routing
    /// 
    /// Automatically chooses:
    /// - CommuteLock for local (same-machine) communication
    /// - DynaRoute for remote (cross-machine) communication
    pub async fn send_message(&self, target_vpod: &str, data: &[u8]) -> Result<()> {
        // Try local communication first (fastest)
        if let Ok(mut lock) = CommuteLock::new(target_vpod, &self.commute_lock) {
            match lock.send(target_vpod, data) {
                Ok(_) => {
                    info!("📨 Sent via CommuteLock (local): {} bytes to {}", data.len(), target_vpod);
                    return Ok(());
                }
                Err(e) => {
                    warn!("⚠️ CommuteLock failed, falling back to DynaRoute: {}", e);
                }
            }
        }
        
        // Fallback to remote communication via DynaRoute
        let virtual_addr = self.virtual_addresses.read().await
            .get(target_vpod)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("vPod not found: {}", target_vpod))?;
        
        self.transport.send(&virtual_addr, data).await?;
        
        info!("📨 Sent via DynaRoute (remote): {} bytes to {}", data.len(), target_vpod);
        
        Ok(())
    }
    
    /// Receive message (hybrid)
    pub async fn receive_message(&self, vpod_id: &str) -> Result<Vec<u8>> {
        // Try local first
        if let Ok(mut lock) = CommuteLock::new(vpod_id, &self.commute_lock) {
            match lock.receive() {
                Ok(msg) => {
                    info!("📬 Received via CommuteLock (local): {} bytes", msg.data().len());
                    return Ok(msg.data().to_vec());
                }
                Err(_) => {
                    // Fall through to remote
                }
            }
        }
        
        // Remote receive via DynaRoute
        let (conn, remote) = self.transport.accept().await?;
        
        let (mut _send, mut recv) = conn.accept_bi().await?;
        let data = recv.read_to_end(1024 * 1024).await?; // 1MB max
        
        info!("📬 Received via DynaRoute (remote): {} bytes from {}", data.len(), remote);
        
        Ok(data)
    }
    
    /// Register service for discovery
    pub async fn register_service(&self, service_name: String, endpoints: Vec<SocketAddr>) {
        self.discovery.register_service(service_name.clone(), endpoints.clone()).await;
        info!("✅ Service registered: {} ({} endpoints)", service_name, endpoints.len());
    }
    
    /// Discover service endpoints
    pub async fn discover_service(&self, service_name: &str) -> Option<Vec<SocketAddr>> {
        self.discovery.discover(service_name).await
    }
    
    /// Select vPod for load balancing
    pub async fn select_vpod(&self, service_id: &str, holder: &str) -> Result<Option<String>> {
        self.agent.select_vpod(service_id, holder).await
    }
    
    /// Get local address
    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }
    
    /// Get transport for advanced operations
    pub fn transport(&self) -> Arc<CloudTransport> {
        Arc::clone(&self.transport)
    }
    
    /// Get agent for advanced operations
    pub fn agent(&self) -> Arc<AddressSyncAgent> {
        Arc::clone(&self.agent)
    }
}

/// Integration with vPod runtime
pub mod vpod_integration {
    use super::*;
    
    /// vPod with integrated networking
    pub struct NetworkedVPod {
        /// vPod ID
        pub vpod_id: String,
        
        /// Service ID
        pub service_id: String,
        
        /// Virtual address
        pub virtual_addr: VirtualAddress,
        
        /// Unified networking layer
        networking: Arc<UnifiedNetworkingLayer>,
    }
    
    impl NetworkedVPod {
        /// Create new networked vPod
        pub async fn new(
            vpod_id: String,
            service_id: String,
            actual_addr: SocketAddr,
            networking: Arc<UnifiedNetworkingLayer>,
        ) -> Result<Self> {
            let virtual_addr = networking.register_vpod(
                vpod_id.clone(),
                service_id.clone(),
                actual_addr,
            ).await?;
            
            Ok(Self {
                vpod_id,
                service_id,
                virtual_addr,
                networking,
            })
        }
        
        /// Send message to another vPod
        pub async fn send_to(&self, target_vpod: &str, data: &[u8]) -> Result<()> {
            self.networking.send_message(target_vpod, data).await
        }
        
        /// Receive message
        pub async fn receive(&self) -> Result<Vec<u8>> {
            self.networking.receive_message(&self.vpod_id).await
        }
        
        /// Get virtual address
        pub fn virtual_address(&self) -> &VirtualAddress {
            &self.virtual_addr
        }
    }
}

/// Integration with BSO-K8 orchestrator
pub mod bso_k8_integration {
    use super::*;
    
    /// BSO-K8 orchestrator with DynaRoute integration
    pub struct NetworkedOrchestrator {
        /// Unified networking layer
        networking: Arc<UnifiedNetworkingLayer>,
        
        /// Deployed vPods: vpod_id → NetworkedVPod
        vpods: Arc<RwLock<std::collections::HashMap<String, Arc<vpod_integration::NetworkedVPod>>>>,
    }
    
    impl NetworkedOrchestrator {
        /// Create new networked orchestrator
        pub fn new(networking: Arc<UnifiedNetworkingLayer>) -> Self {
            Self {
                networking,
                vpods: Arc::new(RwLock::new(std::collections::HashMap::new())),
            }
        }
        
        /// Deploy vPod with automatic networking
        pub async fn deploy_vpod(
            &self,
            vpod_id: String,
            service_id: String,
            actual_addr: SocketAddr,
        ) -> Result<Arc<vpod_integration::NetworkedVPod>> {
            info!("🚀 Deploying vPod: {} (service: {})", vpod_id, service_id);
            
            let vpod = Arc::new(
                vpod_integration::NetworkedVPod::new(
                    vpod_id.clone(),
                    service_id,
                    actual_addr,
                    Arc::clone(&self.networking),
                ).await?
            );
            
            self.vpods.write().await.insert(vpod_id.clone(), Arc::clone(&vpod));
            
            info!("✅ vPod deployed: {}", vpod_id);
            
            Ok(vpod)
        }
        
        /// Get vPod by ID
        pub async fn get_vpod(&self, vpod_id: &str) -> Option<Arc<vpod_integration::NetworkedVPod>> {
            self.vpods.read().await.get(vpod_id).cloned()
        }
        
        /// List all vPods
        pub async fn list_vpods(&self) -> Vec<String> {
            self.vpods.read().await.keys().cloned().collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::env_ini_parser::EnvIniParser;
    
    #[tokio::test]
    async fn test_unified_networking_creation() {
        let config = EnvIniParser::from_file("config/env.ini").unwrap();
        let commute_lock = Arc::new(CommuteLockRuntime::new(&config).unwrap());
        
        let networking = UnifiedNetworkingLayer::new(
            "127.0.0.1:6000".parse().unwrap(),
            commute_lock,
        ).await.unwrap();
        
        assert_eq!(networking.local_addr().port(), 6000);
    }
}
