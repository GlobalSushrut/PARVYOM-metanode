// 6D Blockchain Writer Service
// Manages LogbookTo6DConverter and SixDBlockchainWriter with DynaRoute integration

use anyhow::Result;
use std::sync::Arc;
use std::env;
use tracing::{info, warn, error};
use crate::logbook_6d_bridge::{LogbookTo6DConverter, SixDBlockchainWriter};
use crate::dynaroute_client::DynaRouteClient;
use crate::blockchain_os_kernel::BlockchainOSKernel;
use crate::blockchain_os_kernel::commute_lock::{MessageType, Priority};
use crate::config;
use tokio::sync::RwLock;

/// 6D Blockchain Writer Service with DynaRoute integration
pub struct BlockchainWriterService {
    /// Logbook to 6D converter
    converter: Arc<LogbookTo6DConverter>,
    
    /// 6D blockchain writer
    blockchain_writer: Arc<SixDBlockchainWriter>,
    
    /// DynaRoute client for service discovery
    dynaroute_client: Arc<DynaRouteClient>,
    
    /// BPCI server address
    bpci_server: String,
    
    /// XTMP endpoint (discovered via DynaRoute)
    xtmp_endpoint: Arc<RwLock<Option<std::net::SocketAddr>>>,
    
    /// Service running flag
    running: Arc<RwLock<bool>>,

    /// Mesh vs HTTP submission metrics
    mesh_success_submissions: Arc<RwLock<u64>>,
    mesh_fallback_submissions: Arc<RwLock<u64>>,
    http_only_submissions: Arc<RwLock<u64>>,
}

impl BlockchainWriterService {
    /// Resolve BPCI server from environment with sane defaults
    /// Uses BPI_BPCI_SERVER if set, otherwise falls back to 127.0.0.1:7778
    fn resolve_bpci_server_from_env() -> String {
        env::var("BPI_BPCI_SERVER")
            .unwrap_or_else(|_| "127.0.0.1:7778".to_string())
    }

    /// Create and start the 6D blockchain writer service
    pub async fn start(bpci_server: &str) -> Result<Arc<Self>> {
        info!("⛓️ Starting 6D Blockchain Writer Service with DynaRoute integration");
        let effective_bpci_server = if bpci_server.is_empty() {
            Self::resolve_bpci_server_from_env()
        } else {
            bpci_server.to_string()
        };

        info!("   BPCI Server: {}", effective_bpci_server);
        
        // Create 6D blockchain writer
        info!("📝 Creating 6D blockchain writer...");
        let blockchain_writer = match SixDBlockchainWriter::new().await {
            Ok(writer) => {
                info!("✅ 6D blockchain writer created");
                Arc::new(writer)
            }
            Err(e) => {
                warn!("⚠️ Failed to create blockchain writer: {}", e);
                return Err(e);
            }
        };
        blockchain_writer.initialize().await?;
        info!("✅ 6D blockchain writer initialized");
        
        // Create logbook→6D converter
        info!("🔄 Creating logbook→6D converter...");
        let converter = Arc::new(LogbookTo6DConverter::new().await?);
        converter.initialize().await?;
        info!("✅ Logbook→6D converter initialized");
        
        // Start auto-conversion
        info!("🚀 Starting auto-conversion...");
        converter.start_auto_conversion().await?;
        info!("✅ Auto-conversion started");
        
        // Create DynaRoute client
        let dynaroute_client = Arc::new(DynaRouteClient::new(&effective_bpci_server));
        
        // Try to discover XTMP service
        let xtmp_endpoint = match dynaroute_client.discover_service("xtmp").await {
            Ok(endpoint) => {
                info!("✅ Discovered XTMP service at: {}", endpoint);
                Some(endpoint)
            }
            Err(e) => {
                warn!("⚠️ XTMP service not available yet: {}", e);
                warn!("   Will retry discovery in background");
                None
            }
        };
        
        let service = Arc::new(Self {
            converter,
            blockchain_writer,
            dynaroute_client,
            bpci_server: effective_bpci_server,
            xtmp_endpoint: Arc::new(RwLock::new(xtmp_endpoint)),
            running: Arc::new(RwLock::new(true)),
            mesh_success_submissions: Arc::new(RwLock::new(0)),
            mesh_fallback_submissions: Arc::new(RwLock::new(0)),
            http_only_submissions: Arc::new(RwLock::new(0)),
        });
        
        info!("✅ 6D Blockchain Writer Service started successfully");
        info!("   Note: Call start_background_tasks() to begin processing");
        
        Ok(service)
    }

    /// Convenience constructor that resolves BPCI server purely from environment
    pub async fn start_from_env() -> Result<Arc<Self>> {
        let server = Self::resolve_bpci_server_from_env();
        Self::start(&server).await
    }
    
    /// Start background tasks in a separate async context
    pub async fn start_background_tasks(self: Arc<Self>) {
        info!("🚀 Starting background tasks for 6D blockchain writer");
        if let Err(e) = self.background_tasks().await {
            warn!("⚠️ Background tasks completed with error: {}", e);
        }
    }
    
    /// Background tasks (XTMP discovery, block creation, etc.)
    async fn background_tasks(&self) -> Result<()> {
        info!("🔄 Starting background tasks");
        
        while *self.running.read().await {
            // Rediscover XTMP if not available
            if self.xtmp_endpoint.read().await.is_none() {
                match self.dynaroute_client.discover_service("xtmp").await {
                    Ok(endpoint) => {
                        info!("✅ Discovered XTMP service at: {}", endpoint);
                        *self.xtmp_endpoint.write().await = Some(endpoint);
                    }
                    Err(_) => {
                        // Silent fail, will retry next iteration
                    }
                }
            }
            
            // Create blocks from pending transactions
            if let Err(e) = self.create_blocks_from_pending().await {
                warn!("⚠️ Error creating blocks: {}", e);
            }
            
            // Submit blocks to BPCI via XTMP
            if let Err(e) = self.submit_blocks_to_bpci().await {
                warn!("⚠️ Error submitting blocks: {}", e);
            }
            
            // Sleep before next iteration
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
        
        info!("🛑 Background tasks stopped");
        Ok(())
    }
    
    /// Create blocks from pending transactions
    async fn create_blocks_from_pending(&self) -> Result<()> {
        let pending_count = self.blockchain_writer.get_pending_transaction_count().await?;
        
        if pending_count > 0 {
            info!("📦 Creating block from {} pending transactions", pending_count);
            
            let block_id = self.blockchain_writer.create_block_from_pending().await?;
            info!("✅ Created block: {}", block_id);
        }
        
        Ok(())
    }
    
    /// Submit blocks to BPCI via XTMP
    async fn submit_blocks_to_bpci(&self) -> Result<()> {
        // Check if XTMP endpoint is available
        let xtmp_endpoint = match *self.xtmp_endpoint.read().await {
            Some(endpoint) => endpoint,
            None => {
                // XTMP not available, skip
                return Ok(());
            }
        };
        
        // Get blockchain state
        let state = self.blockchain_writer.get_blockchain_state().await?;
        
        if state.current_block_number > 0 {
            info!("📤 Submitting blockchain state to BPCI via XTMP");
            info!("   XTMP Endpoint: {}", xtmp_endpoint);
            info!("   Current Block: {}", state.current_block_number);

            // Try to submit via mesh-native communication first (if enabled)
            if config::is_mesh_internal_enabled() {
                let mesh_result: Result<()> = async {
                    let kernel = BlockchainOSKernel::new().await?;
                    let payload = serde_json::to_vec(&state)?;
                    kernel
                        .send_mesh_message(
                            "bpi.6d.chain.submit_state",
                            &payload,
                            MessageType::Data,
                            Priority::High,
                        )
                        .await
                }
                .await;

                match mesh_result {
                    Ok(()) => {
                        {
                            let mut mesh_success = self.mesh_success_submissions.write().await;
                            *mesh_success += 1;
                        }
                        info!(
                            "✅ Blockchain state submitted to BPCI via mesh-native path (6D writer → BPCI)"
                        );
                        return Ok(());
                    }
                    Err(e) => {
                        {
                            let mut mesh_fallback = self.mesh_fallback_submissions.write().await;
                            *mesh_fallback += 1;
                        }
                        warn!(
                            "⚠️ Mesh-native submission of blockchain state failed, falling back to HTTP XTMP path: {}",
                            e
                        );
                    }
                }
            }
            if !config::is_mesh_internal_enabled() {
                let mut http_only = self.http_only_submissions.write().await;
                *http_only += 1;
            }
            
            // Submit via XTMP (simplified - real implementation would use XTMP protocol)
            let client = reqwest::Client::new();
            match client
                .post(format!("http://{}/submit-blockchain-state", xtmp_endpoint))
                .json(&state)
                .send()
                .await
            {
                Ok(response) if response.status().is_success() => {
                    info!("✅ Blockchain state submitted to BPCI successfully");
                }
                Ok(response) => {
                    warn!("⚠️ BPCI returned error: {}", response.status());
                }
                Err(e) => {
                    warn!("⚠️ Failed to submit to BPCI: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    /// Get converter metrics
    pub async fn get_metrics(&self) -> Result<String> {
        let state = self.converter.get_converter_status().await?;
        let mesh_success = *self.mesh_success_submissions.read().await;
        let mesh_fallback = *self.mesh_fallback_submissions.read().await;
        let http_only = *self.http_only_submissions.read().await;
        Ok(format!(
            "Converter Status: {:?}, Processed: {}, mesh_success: {}, mesh_fallback_http: {}, http_only: {}",
            state.status,
            state.total_entries_processed,
            mesh_success,
            mesh_fallback,
            http_only
        ))
    }
    
    /// Stop the service
    pub async fn stop(&self) -> Result<()> {
        info!("🛑 Stopping 6D Blockchain Writer Service");
        *self.running.write().await = false;
        self.converter.stop().await?;
        self.blockchain_writer.stop().await?;
        Ok(())
    }
}
