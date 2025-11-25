// DynaRoute Service Discovery Client for BPI
// Lightweight client to discover BPCI services via DynaRoute registry

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use log::{info, warn};

/// DynaRoute service discovery client
#[derive(Debug)]
pub struct DynaRouteClient {
    registry_endpoint: String,
}

/// Service endpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub service_name: String,
    pub address: SocketAddr,
    pub status: String,
}

impl DynaRouteClient {
    /// Create a new DynaRoute client with direct IP support (Cloudflare SSL issues)
    pub fn new(bpci_server: &str) -> Self {
        // Use direct IP registry (port 8087) instead of Cloudflare due to SSL certificate issues
        let registry_endpoint = if bpci_server.parse::<std::net::IpAddr>().is_ok() {
            // IP address - use direct DynaRoute registry on port 8087
            format!("http://{}:8087/services", bpci_server)
        } else {
            // Domain - try direct HTTP first (Cloudflare has SSL issues)
            format!("http://{}:8087/services", bpci_server)
        };
        info!("🔍 DynaRoute client initialized (direct registry): {}", registry_endpoint);
        
        Self {
            registry_endpoint,
        }
    }
    
    /// Get the Cloudflare proxied XTMP endpoint
    pub fn get_xtmp_endpoint() -> String {
        "https://xtmp.pravyom.com".to_string()
    }
    
    /// Get the Cloudflare proxied consensus endpoint
    pub fn get_consensus_endpoint() -> String {
        "https://consensus.pravyom.com".to_string()
    }
    
    /// Register a service with the DynaRoute registry
    pub async fn register_service(&self, service_name: &str, address: &str) -> Result<()> {
        let client = reqwest::Client::new();
        let registration_data = serde_json::json!({
            "service_name": service_name,
            "address": address,
            "metadata": {}
        });
        
        let response = client
            .post(&self.registry_endpoint)
            .json(&registration_data)
            .send()
            .await;
            
        match response {
            Ok(resp) if resp.status().is_success() => {
                info!("✅ Registered service {} at {}", service_name, address);
                Ok(())
            }
            Ok(resp) => {
                warn!("❌ Failed to register service {}: HTTP {}", service_name, resp.status());
                Err(anyhow!("Registration failed with status: {}", resp.status()))
            }
            Err(e) => {
                warn!("❌ Network error registering service {}: {}", service_name, e);
                // Don't fail startup for registration errors - services can still work
                Ok(())
            }
        }
    }
    
    /// Discover a service by name
    pub async fn discover_service(&self, service_name: &str) -> Result<SocketAddr> {
        info!("🔍 Discovering service: {}", service_name);
        
        // Try to query the DynaRoute registry
        match self.query_registry(service_name).await {
            Ok(endpoint) => {
                info!("✅ Discovered {} at {}", service_name, endpoint);
                Ok(endpoint)
            }
            Err(e) => {
                warn!("⚠️ Service discovery failed: {}", e);
                
                // Fallback: Try common ports for the service
                let fallback_endpoint = self.try_fallback_ports(service_name).await?;
                info!("✅ Using fallback endpoint for {}: {}", service_name, fallback_endpoint);
                Ok(fallback_endpoint)
            }
        }
    }
    
    /// Query the DynaRoute registry with automatic endpoint discovery
    async fn query_registry(&self, service_name: &str) -> Result<SocketAddr> {
        info!("🔍 Auto-discovering DynaRoute registry endpoint...");
        
        // Try multiple endpoint patterns automatically
        let endpoint_patterns = vec![
            format!("{}/{}", self.registry_endpoint, service_name),
            format!("{}/api/services/{}", self.registry_endpoint.trim_end_matches("/services"), service_name),
            format!("{}/v1/services/{}", self.registry_endpoint.trim_end_matches("/services"), service_name),
            format!("{}/discover/{}", self.registry_endpoint.trim_end_matches("/services"), service_name),
            format!("{}/registry/{}", self.registry_endpoint.trim_end_matches("/services"), service_name),
        ];
        
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(2))
            .build()?;
        
        // Try each endpoint pattern
        for (idx, url) in endpoint_patterns.iter().enumerate() {
            info!("  Trying pattern {}/{}: {}", idx + 1, endpoint_patterns.len(), url);
            
            match client.get(url).send().await {
                Ok(response) if response.status().is_success() => {
                    // Get response text first (consumes response)
                    if let Ok(text) = response.text().await {
                        // Try to parse as JSON first
                        if let Ok(endpoint_info) = serde_json::from_str::<ServiceEndpoint>(&text) {
                            info!("✅ Auto-discovered working endpoint (JSON): {}", url);
                            return Ok(endpoint_info.address);
                        }
                        // Try to parse as plain text (IP:port)
                        if let Ok(addr) = text.trim().parse::<SocketAddr>() {
                            info!("✅ Auto-discovered working endpoint (plain text): {}", url);
                            return Ok(addr);
                        }
                    }
                }
                Ok(response) => {
                    info!("  ⚠️ Got response but status: {}", response.status());
                }
                Err(e) => {
                    info!("  ⚠️ Request failed: {}", e);
                }
            }
        }
        
        Err(anyhow!("Auto-discovery failed: No working registry endpoint found"))
    }
    
    /// Try fallback to well-known service ports (automatic discovery)
    async fn try_fallback_ports(&self, service_name: &str) -> Result<SocketAddr> {
        info!("🔍 Trying automatic fallback to well-known service ports...");
        
        // Extract base IP from registry endpoint
        let base_ip = self.registry_endpoint
            .trim_start_matches("http://")
            .trim_start_matches("https://")
            .split(':')
            .next()
            .unwrap_or("134.209.210.181");
        
        // Well-known service ports (automatically try these)
        let well_known_ports: Vec<(String, u16)> = match service_name {
            "xtmp" => vec![
                ("XTMP primary".to_string(), 7778),
                ("XTMP websocket".to_string(), 7779),
                ("XTMP alt".to_string(), 8080),
            ],
            "logbook" | "6d-chain" => vec![
                ("Blockchain primary".to_string(), 9000),
                ("Blockchain API".to_string(), 9002),
                ("Blockchain WS".to_string(), 9003),
            ],
            "consensus" => vec![
                ("Consensus primary".to_string(), 6002),
                ("Consensus alt".to_string(), 6001),
            ],
            "auction" => vec![
                ("Auction primary".to_string(), 7002),
                ("Auction alt".to_string(), 9004),
            ],
            _ => vec![
                ("Generic service".to_string(), 8080),
                ("Generic alt".to_string(), 9000),
            ],
        };
        
        // Try each well-known port
        for (service_desc, port) in well_known_ports {
            let addr = format!("{}:{}", base_ip, port);
            info!("  Trying {}: {}", service_desc, addr);
            
            if let Ok(socket_addr) = addr.parse::<SocketAddr>() {
                // Quick connectivity check
                if self.check_endpoint(&socket_addr).await {
                    info!("✅ Auto-discovered working port: {} ({})", addr, service_desc);
                    return Ok(socket_addr);
                }
            }
        }
        
        // Last resort: Use Cloudflare proxied endpoints as fallback
        let cloudflare_endpoint = match service_name {
            "xtmp" => "xtmp.pravyom.com:443",
            "logbook" => "consensus.pravyom.com:443",
            "6d-chain" => "consensus.pravyom.com:443",
            "consensus" => "consensus.pravyom.com:443",
            "auction" => "auction.pravyom.com:443",
            _ => "consensus.pravyom.com:443",
        };
        
        info!("🌐 Using Cloudflare proxied endpoint: {}", cloudflare_endpoint);
        
        let endpoint: SocketAddr = cloudflare_endpoint
            .parse()
            .map_err(|e| anyhow!("Invalid Cloudflare endpoint: {}", e))?;
        
        // Check if Cloudflare endpoint is reachable
        if self.check_endpoint(&endpoint).await {
            Ok(endpoint)
        } else {
            Err(anyhow!("Cloudflare proxied endpoint not reachable for service: {}", service_name))
        }
    }
    
    /// Check if an endpoint is reachable
    async fn check_endpoint(&self, endpoint: &SocketAddr) -> bool {
        use tokio::net::TcpStream;
        use tokio::time::{timeout, Duration};
        
        match timeout(Duration::from_secs(2), TcpStream::connect(endpoint)).await {
            Ok(Ok(_)) => {
                info!("✅ Endpoint {} is reachable", endpoint);
                true
            }
            _ => {
                warn!("❌ Endpoint {} is not reachable", endpoint);
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_dynaroute_client_creation() {
        let client = DynaRouteClient::new("134.209.210.181");
        assert!(client.registry_endpoint.contains("134.209.210.181"));
    }
}
