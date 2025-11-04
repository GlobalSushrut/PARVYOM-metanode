//! # Cloud-Ready Transport Layer
//! 
//! Userspace-only networking that works on AWS, GCP, Azure, and any cloud provider.
//! No kernel modifications, no eBPF, no SRv6 - pure Rust userspace.

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use quinn::{Endpoint, ServerConfig, ClientConfig, Connection};
use anyhow::{Result, anyhow};
use tracing::info;

use crate::{
    hrw::RendezvousHasher,
    VirtualAddress,
};

/// Cloud-ready transport using QUIC over standard UDP
/// 
/// Works on ANY cloud provider (AWS, GCP, Azure, etc.)
/// No kernel modifications needed!
#[derive(Clone)]
pub struct CloudTransport {
    /// QUIC endpoint (binds to standard UDP port)
    endpoint: Endpoint,
    
    /// Virtual routing table: virtual_id → actual socket address
    routing_table: Arc<RwLock<std::collections::HashMap<[u8; 32], SocketAddr>>>,
    
    /// HRW hasher for vPod selection
    hrw_hasher: Arc<RwLock<RendezvousHasher>>,
    
    /// Active connections cache
    connections: Arc<RwLock<std::collections::HashMap<SocketAddr, Connection>>>,
}

impl CloudTransport {
    /// Create new cloud transport
    /// 
    /// Binds to a standard UDP port (e.g., 443 for QUIC)
    /// Works on any cloud instance!
    pub async fn new(bind_addr: SocketAddr) -> Result<Self> {
        info!("🌐 Creating cloud-ready transport on {}", bind_addr);
        
        // Create QUIC endpoint (standard UDP, works anywhere)
        let endpoint = Self::create_quic_endpoint(bind_addr).await?;
        
        info!("✅ Cloud transport ready on {}", bind_addr);
        
        Ok(Self {
            endpoint,
            routing_table: Arc::new(RwLock::new(std::collections::HashMap::new())),
            hrw_hasher: Arc::new(RwLock::new(RendezvousHasher::new())),
            connections: Arc::new(RwLock::new(std::collections::HashMap::new())),
        })
    }
    
    /// Create QUIC endpoint with cloud-friendly config
    async fn create_quic_endpoint(bind_addr: SocketAddr) -> Result<Endpoint> {
        // Generate self-signed cert (or use Let's Encrypt in production)
        let cert = rcgen::generate_simple_self_signed(vec!["bpci.local".to_string()])?;
        let cert_der = cert.serialize_der()?;
        let priv_key = cert.serialize_private_key_der();
        
        let mut server_crypto = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(
                vec![rustls::Certificate(cert_der)],
                rustls::PrivateKey(priv_key),
            )?;
        
        server_crypto.alpn_protocols = vec![b"bpci".to_vec()];
        
        let mut server_config = ServerConfig::with_crypto(Arc::new(server_crypto));
        
        // Cloud-friendly transport config
        let mut transport_config = quinn::TransportConfig::default();
        transport_config.max_concurrent_bidi_streams(1000u32.into());
        transport_config.max_concurrent_uni_streams(1000u32.into());
        transport_config.keep_alive_interval(Some(std::time::Duration::from_secs(5)));
        
        server_config.transport = Arc::new(transport_config);
        
        // Bind to UDP socket (works on any cloud!)
        let endpoint = Endpoint::server(server_config, bind_addr)?;
        
        Ok(endpoint)
    }
    
    /// Register vPod with actual socket address
    /// 
    /// Maps virtual identity to real cloud IP:port
    pub async fn register_vpod(
        &self,
        virtual_addr: &VirtualAddress,
        actual_addr: SocketAddr,
    ) -> Result<()> {
        // Use vpod_id hash as key for consistency
        let key: [u8; 32] = *blake3::hash(virtual_addr.vpod_id.as_bytes()).as_bytes();
        
        // Add to routing table
        self.routing_table.write().await.insert(
            key,
            actual_addr,
        );
        
        // Add to HRW hasher
        self.hrw_hasher.write().await.add_vpod(
            virtual_addr.vpod_id.clone(),
            crate::hrw::VPodWeight::default(),
        );
        
        info!("✅ Registered vPod {} at {}", virtual_addr.vpod_id, actual_addr);
        
        Ok(())
    }
    
    /// Connect to virtual address (resolves to actual cloud IP)
    pub async fn connect(&self, virtual_addr: &VirtualAddress) -> Result<Connection> {
        // Use vpod_id hash as key for consistency
        let key: [u8; 32] = *blake3::hash(virtual_addr.vpod_id.as_bytes()).as_bytes();
        
        // Lookup actual address from routing table
        let actual_addr = self.routing_table.read().await
            .get(&key)
            .copied()
            .ok_or_else(|| anyhow!("Virtual address not found in routing table"))?;
        
        // Check connection cache
        {
            let connections = self.connections.read().await;
            if let Some(conn) = connections.get(&actual_addr) {
                if !conn.close_reason().is_some() {
                    return Ok(conn.clone());
                }
            }
        }
        
        // Create new connection
        info!("🔌 Connecting to {} (virtual: {})", actual_addr, virtual_addr.vpod_id);
        
        let mut client_crypto = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
            .with_no_client_auth();
        
        // Set ALPN protocol to match server
        client_crypto.alpn_protocols = vec![b"bpci".to_vec()];
        
        let mut client_config = ClientConfig::new(Arc::new(client_crypto));
        client_config.transport_config(Arc::new(Self::create_transport_config()));
        
        let conn = self.endpoint
            .connect_with(client_config, actual_addr, "bpci")?
            .await?;
        
        // Cache connection
        self.connections.write().await.insert(actual_addr, conn.clone());
        
        info!("✅ Connected to {}", actual_addr);
        
        Ok(conn)
    }
    
    /// Send data to virtual address
    pub async fn send(&self, virtual_addr: &VirtualAddress, data: &[u8]) -> Result<()> {
        let conn = self.connect(virtual_addr).await?;
        
        // Open bidirectional stream
        let (mut send, _recv) = conn.open_bi().await?;
        
        // Send data
        send.write_all(data).await?;
        send.finish().await?;
        
        Ok(())
    }
    
    /// Accept incoming connections
    pub async fn accept(&self) -> Result<(Connection, SocketAddr)> {
        let conn = self.endpoint.accept().await
            .ok_or_else(|| anyhow!("Endpoint closed"))?
            .await?;
        
        let remote_addr = conn.remote_address();
        
        Ok((conn, remote_addr))
    }
    
    /// Create transport config
    fn create_transport_config() -> quinn::TransportConfig {
        let mut config = quinn::TransportConfig::default();
        config.max_concurrent_bidi_streams(1000u32.into());
        config.max_concurrent_uni_streams(1000u32.into());
        config.keep_alive_interval(Some(std::time::Duration::from_secs(5)));
        config
    }
    
    /// Get local address
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.endpoint.local_addr()
            .map_err(|e| anyhow!("Failed to get local address: {}", e))
    }
}

/// Skip server certificate verification (for self-signed certs)
/// In production, use proper CA-signed certificates!
struct SkipServerVerification;

impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}

/// DNS-based service discovery for cloud deployments
/// 
/// Maps service names to actual cloud IPs
pub struct CloudServiceDiscovery {
    /// Service registry: service_name → Vec<SocketAddr>
    registry: Arc<RwLock<std::collections::HashMap<String, Vec<SocketAddr>>>>,
}

impl CloudServiceDiscovery {
    /// Create new service discovery
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }
    
    /// Register service with cloud IP addresses
    pub async fn register_service(&self, service_name: String, addrs: Vec<SocketAddr>) {
        self.registry.write().await.insert(service_name.clone(), addrs.clone());
        info!("✅ Registered service {} with {} endpoints", service_name, addrs.len());
    }
    
    /// Discover service endpoints
    pub async fn discover(&self, service_name: &str) -> Option<Vec<SocketAddr>> {
        self.registry.read().await.get(service_name).cloned()
    }
    
    /// Resolve service to single endpoint (round-robin)
    pub async fn resolve(&self, service_name: &str) -> Option<SocketAddr> {
        let addrs = self.discover(service_name).await?;
        if addrs.is_empty() {
            return None;
        }
        
        // Simple round-robin (can be replaced with HRW)
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let index = (now as usize) % addrs.len();
        
        Some(addrs[index])
    }
}

impl Default for CloudServiceDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cloud_transport_creation() {
        let addr = "127.0.0.1:0".parse().unwrap();
        let transport = CloudTransport::new(addr).await.unwrap();
        assert!(transport.local_addr().is_ok());
    }
    
    #[tokio::test]
    async fn test_service_discovery() {
        let discovery = CloudServiceDiscovery::new();
        
        let addrs = vec![
            "10.0.1.1:443".parse().unwrap(),
            "10.0.1.2:443".parse().unwrap(),
        ];
        
        discovery.register_service("test-service".to_string(), addrs.clone()).await;
        
        let discovered = discovery.discover("test-service").await;
        assert_eq!(discovered, Some(addrs));
    }
}
