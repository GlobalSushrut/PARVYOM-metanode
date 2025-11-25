// Logbook Service Runner
// Starts and manages the BPI Logbook Service with DynaRoute integration

use anyhow::Result;
use std::sync::Arc;
use std::env;
use tokio::sync::RwLock;
use tracing::{info, warn};
use crate::dynaroute_client::DynaRouteClient;
use crate::privacy_preserving_bundle_system::BpciPrivacyInterface;
use crate::blockchain_os_kernel::BlockchainOSKernel;
use crate::blockchain_os_kernel::commute_lock::{MessageType, Priority};
use crate::config;
use serde_json::json;

/// Logbook Service Runner with DynaRoute integration
pub struct LogbookServiceRunner {
    /// DynaRoute client for service discovery
    dynaroute_client: Arc<DynaRouteClient>,
    
    /// BPCI server address
    bpci_server: String,
    
    /// Privacy-preserving BPCI interface
    bpci_privacy_interface: Arc<RwLock<BpciPrivacyInterface>>,
    
    /// Service running flag
    running: Arc<RwLock<bool>>,
    
    /// Entry counter
    entry_count: Arc<RwLock<u64>>,

    /// Mesh vs HTTP path metrics
    mesh_success_count: Arc<RwLock<u64>>,
    mesh_fallback_count: Arc<RwLock<u64>>,
    http_only_count: Arc<RwLock<u64>>,
}

impl LogbookServiceRunner {
    /// Resolve BPCI server from environment with sane defaults
    /// Uses BPI_BPCI_SERVER if set, otherwise falls back to 127.0.0.1:7778
    fn resolve_bpci_server_from_env() -> String {
        env::var("BPI_BPCI_SERVER")
            .unwrap_or_else(|_| "127.0.0.1:7778".to_string())
    }

    /// Create and start a new logbook service
    pub async fn start(bpci_server: &str) -> Result<Arc<Self>> {
        info!("📚 Starting Logbook Service with DynaRoute integration");
        let effective_bpci_server = if bpci_server.is_empty() {
            Self::resolve_bpci_server_from_env()
        } else {
            bpci_server.to_string()
        };

        info!("   BPCI Server: {}", effective_bpci_server);
        
        // Create DynaRoute client
        let dynaroute_client = Arc::new(DynaRouteClient::new(&effective_bpci_server));
        
        // Initialize privacy-preserving BPCI interface
        let bpci_endpoint = format!("http://{}", effective_bpci_server);
        let bpci_privacy_interface = Arc::new(RwLock::new(
            BpciPrivacyInterface::new(bpci_endpoint)
        ));

        let service = Arc::new(Self {
            dynaroute_client,
            bpci_server: effective_bpci_server,
            bpci_privacy_interface,
            running: Arc::new(RwLock::new(true)),
            entry_count: Arc::new(RwLock::new(0)),
            mesh_success_count: Arc::new(RwLock::new(0)),
            mesh_fallback_count: Arc::new(RwLock::new(0)),
            http_only_count: Arc::new(RwLock::new(0)),
        });
        
        // Start processing loop
        let service_clone = service.clone();
        tokio::spawn(async move {
            if let Err(e) = service_clone.processing_loop().await {
                warn!("⚠️ Logbook processing loop error: {}", e);
            }
        });
        
        info!("✅ Logbook Service started successfully");
        
        Ok(service)
    }

    /// Convenience constructor that resolves BPCI server purely from environment
    pub async fn start_from_env() -> Result<Arc<Self>> {
        let server = Self::resolve_bpci_server_from_env();
        Self::start(&server).await
    }
    
    /// Main processing loop
    async fn processing_loop(&self) -> Result<()> {
        info!("🔄 Starting logbook processing loop");
        
        while *self.running.read().await {
            // Try to discover 6D blockchain service
            match self.dynaroute_client.discover_service("6d-chain").await {
                Ok(chain_endpoint) => {
                    info!("✅ Discovered 6D blockchain service at: {}", chain_endpoint);
                }
                Err(_) => {
                    // Service not available yet, will retry
                }
            }
            
            // Sleep before next iteration
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
        
        info!("🛑 Logbook processing loop stopped");
        Ok(())
    }
    
    /// Submit audit record to logbook (called by ImmutableAuditSystem)
    pub async fn submit_audit_record(&self, audit_data: serde_json::Value) -> Result<()> {
        let mut count = self.entry_count.write().await;
        *count += 1;
        
        info!("📝 Logbook received audit record #{}", *count);
        
        // Convert audit data to bytes for privacy-preserving processing
        let audit_bytes = serde_json::to_vec(&audit_data)?;
        
        // Send privacy-preserving proof to BPCI cluster ledger (no real data leaked)
        {
            let mut bpci_interface = self.bpci_privacy_interface.write().await;
            match bpci_interface.send_proof_to_bpci(&audit_bytes).await {
                Ok(()) => {
                    info!("✅ Privacy-preserving proof sent to BPCI cluster ledger");
                }
                Err(e) => {
                    warn!("⚠️ Failed to send privacy-preserving proof to BPCI: {}", e);
                }
            }
        }

        // Try to forward to 6D blockchain via mesh-native communication first (if enabled)
        if config::is_mesh_internal_enabled() {
            let mesh_result: Result<()> = async {
                let kernel = BlockchainOSKernel::new().await?;
                kernel
                    .send_mesh_message(
                        "bpi.6d.chain.convert_audit",
                        &audit_bytes,
                        MessageType::Data,
                        Priority::Normal,
                    )
                    .await
            }
            .await;

            match mesh_result {
                Ok(()) => {
                    {
                        let mut mesh_success = self.mesh_success_count.write().await;
                        *mesh_success += 1;
                    }
                    info!("✅ Audit record forwarded to 6D blockchain via mesh-native path");
                    return Ok(());
                }
                Err(e) => {
                    {
                        let mut mesh_fallback = self.mesh_fallback_count.write().await;
                        *mesh_fallback += 1;
                    }
                    warn!(
                        "⚠️ Mesh-native forwarding to 6D blockchain failed, falling back to HTTP: {}",
                        e
                    );
                }
            }
        }
        
        // Try to forward to 6D blockchain (internal processing)
        if !config::is_mesh_internal_enabled() {
            let mut http_only = self.http_only_count.write().await;
            *http_only += 1;
        }
        match self.dynaroute_client.discover_service("6d-chain").await {
            Ok(chain_endpoint) => {
                info!("✅ Forwarding to 6D blockchain at: {}", chain_endpoint);
                
                let client = reqwest::Client::new();
                match client
                    .post(format!("http://{}/convert-audit-to-6d", chain_endpoint))
                    .json(&audit_data)
                    .send()
                    .await
                {
                    Ok(resp) if resp.status().is_success() => {
                        info!("✅ Audit record forwarded to 6D blockchain");
                    }
                    Ok(resp) => {
                        warn!("⚠️ 6D blockchain returned error: {}", resp.status());
                    }
                    Err(e) => {
                        warn!("⚠️ Failed to forward to 6D blockchain: {}", e);
                    }
                }
            }
            Err(_) => {
                warn!("⚠️ 6D blockchain service not available");
            }
        }
        
        Ok(())
    }
    
    /// Get service statistics
    pub async fn get_stats(&self) -> Result<String> {
        let count = *self.entry_count.read().await;
        let mesh_success = *self.mesh_success_count.read().await;
        let mesh_fallback = *self.mesh_fallback_count.read().await;
        let http_only = *self.http_only_count.read().await;
        Ok(format!(
            "Logbook entries processed: {}, mesh_success: {}, mesh_fallback_http: {}, http_only: {}",
            count, mesh_success, mesh_fallback, http_only
        ))
    }
    
    /// Stop the service
    pub async fn stop(&self) -> Result<()> {
        info!("🛑 Stopping Logbook Service");
        *self.running.write().await = false;
        Ok(())
    }
}
