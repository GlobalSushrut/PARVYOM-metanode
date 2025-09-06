//! Simple UDP Transport for Web-4 Testnet

use crate::{NodeId, P2PMessage};
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tracing::{info, warn, error};

/// Simple UDP transport (no QUIC complexity for testnet)
pub struct UdpTransport {
    port: u16,
    socket: Option<UdpSocket>,
}

impl UdpTransport {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            socket: None,
        }
    }
    
    /// Start UDP transport
    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let addr = format!("0.0.0.0:{}", self.port);
        let socket = UdpSocket::bind(&addr).await?;
        info!("UDP transport bound to {}", addr);
        
        self.socket = Some(socket);
        Ok(())
    }
    
    /// Send message directly (for consensus - highest priority)
    pub async fn send_direct(&self, target: &NodeId, message: &P2PMessage) -> Result<(), Box<dyn Error>> {
        if let Some(socket) = &self.socket {
            let serialized = serde_json::to_vec(message)?;
            
            // For testnet, use simple port mapping from node ID
            let target_port = self.node_id_to_port(&target.0);
            let target_addr: SocketAddr = format!("127.0.0.1:{}", target_port).parse()?;
            
            socket.send_to(&serialized, target_addr).await?;
            info!("Sent direct message to {} ({})", target, target_addr);
        }
        Ok(())
    }
    
    /// Send with retry (for auction - medium priority)
    pub async fn send_with_retry(&self, target: &NodeId, message: &P2PMessage, retries: u8) -> Result<(), Box<dyn Error>> {
        for attempt in 0..=retries {
            match self.send_direct(target, message).await {
                Ok(()) => return Ok(()),
                Err(e) => {
                    if attempt == retries {
                        error!("Failed to send after {} retries: {}", retries, e);
                        return Err(e);
                    }
                    warn!("Send attempt {} failed, retrying: {}", attempt + 1, e);
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
            }
        }
        Ok(())
    }
    
    /// Send best effort (for shadow data - background priority)
    pub async fn send_best_effort(&self, target: &NodeId, message: &P2PMessage) -> Result<(), Box<dyn Error>> {
        // Just try once, don't worry if it fails
        if let Err(e) = self.send_direct(target, message).await {
            warn!("Best effort send failed (ignoring): {}", e);
        }
        Ok(())
    }
    
    /// Simple port mapping for testnet (node_id -> port)
    fn node_id_to_port(&self, node_id: &str) -> u16 {
        // Extract hex from node_id and map to port range 9000-9999
        let hex_part = node_id.replace("node_", "");
        if let Ok(id) = u64::from_str_radix(&hex_part[..4], 16) {
            9000 + (id % 1000) as u16
        } else {
            9000 // Default port
        }
    }
}
