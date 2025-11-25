//! 3-Tier Audit Batch Processing System
//! Level 1: 100 ZipLock records → Summary → BPI Ledger Transaction
//! Level 2: 1000 BPI Summaries → BPI Bundle → BPCI Server
//! Level 3: Multiple BPI Bundles → BPCI Batch Bundle → Auction System

use std::sync::{Arc, Mutex};
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::PathBuf;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;
use sha2::{Sha256, Digest};

use crate::audit_http_server::ZipLockJsonAudit;
use crate::blockchain_os_kernel::BlockchainOSKernel;
use crate::blockchain_os_kernel::commute_lock::{MessageType, Priority};
use crate::config;
use crate::cbor_pipeline_foundation::serialize_canonical_bounded;

/// Level 1: ZipLock Batch Processor (100 records → summary → BPI tx)
#[derive(Debug)]
pub struct ZipLockBatchProcessor {
    /// Accumulated ZipLock records
    pending_records: Arc<Mutex<VecDeque<ZipLockJsonAudit>>>,
    /// Batch configuration
    config: ZipLockBatchConfig,
    /// Statistics
    stats: Arc<RwLock<BatchProcessorStats>>,
    quota_window: Arc<Mutex<AuditQuotaWindow>>,
}

/// Configuration for ZipLock batch processing
#[derive(Debug, Clone)]
pub struct ZipLockBatchConfig {
    pub batch_size: usize,           // 100 records per batch
    pub max_batch_age_seconds: u64,  // Force batch after timeout
    pub enable_compression: bool,     // Compress batch summaries
    pub bpi_ledger_endpoint: String, // BPI ledger submission endpoint
}

impl Default for ZipLockBatchConfig {
    fn default() -> Self {
        let default_endpoint = "http://localhost:9545/api/ledger/submit".to_string();
        let bpi_ledger_endpoint = std::env::var("BPI_LEDGER_ENDPOINT").unwrap_or(default_endpoint);
        validate_endpoint_for_mode("BPI_LEDGER_ENDPOINT", &bpi_ledger_endpoint);

        Self {
            batch_size: 100,
            max_batch_age_seconds: 300,
            enable_compression: true,
            bpi_ledger_endpoint,
        }
    }
}

fn is_bpi_ledger_enabled() -> bool {
    // If BPI_LEDGER_DISABLED is set to "1" or "true", treat the ledger as
    // inactive and skip submissions. Default is enabled.
    std::env::var("BPI_LEDGER_DISABLED")
        .map(|v| {
            let v = v.to_lowercase();
            !(v == "1" || v == "true")
        })
        .unwrap_or(true)
}

fn max_audit_records_per_hour() -> u64 {
    std::env::var("BPCI_MAX_AUDIT_RECORDS_PER_HOUR")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1_000_000)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BpiNetworkMode {
    Mock,
    Testnet,
    Mainnet,
}

fn bpi_network_mode() -> BpiNetworkMode {
    let mode = std::env::var("BPI_NETWORK_MODE")
        .unwrap_or_else(|_| "testnet".to_string())
        .to_lowercase();
    match mode.as_str() {
        "mainnet" => BpiNetworkMode::Mainnet,
        "mock" => BpiNetworkMode::Mock,
        "testnet" => BpiNetworkMode::Testnet,
        _ => BpiNetworkMode::Testnet,
    }
}

fn is_mainnet_mode() -> bool {
    matches!(bpi_network_mode(), BpiNetworkMode::Mainnet)
}

fn validate_endpoint_for_mode(name: &str, endpoint: &str) {
    if is_mainnet_mode() {
        if endpoint.starts_with("http://127.0.0.1") || endpoint.starts_with("http://localhost") {
            tracing::warn!(
                "BPI_NETWORK_MODE=mainnet but {} is using a localhost endpoint: {}",
                name,
                endpoint
            );
        }
        if endpoint.starts_with("http://") {
            tracing::warn!(
                "BPI_NETWORK_MODE=mainnet but {} is using insecure HTTP endpoint: {}",
                name,
                endpoint
            );
        }
    }
}

fn ledger_node_context() -> (String, String) {
    let node_id = std::env::var("BPI_NODE_ID").unwrap_or_else(|_| "unknown".to_string());
    let profile = std::env::var("BPI_ENV").unwrap_or_else(|_| "unknown".to_string());
    (node_id, profile)
}

fn ledger_mesh_config_hash() -> String {
    std::env::var("BPI_NX_MESH_CONFIG_HASH").unwrap_or_else(|_| "unknown".to_string())
}

fn ledger_proof_of_execution_hash() -> String {
    std::env::var("BPI_PROOF_OF_EXECUTION_HASH").unwrap_or_else(|_| "unknown".to_string())
}

fn bpci_resource_budget() -> serde_json::Value {
    let cpu_percent: f64 = std::env::var("BPCI_CPU_BUDGET_PERCENT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(20.0);
    let disk_mb: u64 = std::env::var("BPCI_DISK_BUDGET_MB")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10_240); // 10 GiB default
    let net_mbps: u64 = std::env::var("BPCI_NET_BUDGET_MBPS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(100);
    let max_audits_per_hour: u64 = std::env::var("BPCI_MAX_AUDIT_RECORDS_PER_HOUR")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1_000_000);

    serde_json::json!({
        "cpu_percent": cpu_percent,
        "disk_mb": disk_mb,
        "net_mbps": net_mbps,
        "max_audits_per_hour": max_audits_per_hour,
    })
}

fn economic_metadata_for_batch(summary: &ZipLockBatchSummary) -> serde_json::Value {
    let (node_id, profile) = ledger_node_context();
    let mesh_config_hash = ledger_mesh_config_hash();
    let proof_of_execution_hash = ledger_proof_of_execution_hash();
    let resource_budget = bpci_resource_budget();

    let gas_estimate = 21_000u64 + summary.total_size_bytes * 16;
    let rent_units = summary.total_size_bytes / 1024; // Approximate KB-based rent units

    serde_json::json!({
        "node_id": node_id,
        "profile": profile,
        "mesh_config_hash": mesh_config_hash,
        "proof_of_execution_hash": proof_of_execution_hash,
        "gas_estimate": gas_estimate,
        "rent_units": rent_units,
        "pricing_version": "v1",
        "bpci_resource_budget": resource_budget,
    })
}

fn economic_metadata_for_bundle(bundle: &BpiBundle) -> serde_json::Value {
    let (node_id, profile) = ledger_node_context();
    let mesh_config_hash = ledger_mesh_config_hash();
    let proof_of_execution_hash = ledger_proof_of_execution_hash();
    let resource_budget = bpci_resource_budget();

    serde_json::json!({
        "node_id": node_id,
        "profile": profile,
        "mesh_config_hash": mesh_config_hash,
        "proof_of_execution_hash": proof_of_execution_hash,
        "gas_estimate": bundle.gas_estimate,
        "rent_units": bundle.total_audit_records,
        "economic_value": bundle.economic_value,
        "pricing_version": "v1",
        "bpci_resource_budget": resource_budget,
    })
}

fn economic_metadata_for_batch_bundle(batch_bundle: &BpciBatchBundle) -> serde_json::Value {
    let (node_id, profile) = ledger_node_context();
    let mesh_config_hash = ledger_mesh_config_hash();
    let proof_of_execution_hash = ledger_proof_of_execution_hash();
    let resource_budget = bpci_resource_budget();

    serde_json::json!({
        "node_id": node_id,
        "profile": profile,
        "mesh_config_hash": mesh_config_hash,
        "proof_of_execution_hash": proof_of_execution_hash,
        "total_economic_value": batch_bundle.total_economic_value,
        "auction_reserve_price": batch_bundle.auction_reserve_price,
        "pricing_version": "v1",
        "bpci_resource_budget": resource_budget,
    })
}

/// ZipLock batch summary for BPI ledger submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZipLockBatchSummary {
    pub batch_id: String,
    pub timestamp: DateTime<Utc>,
    pub record_count: usize,
    pub total_size_bytes: u64,
    pub merkle_root: String,
    pub vm_type_distribution: HashMap<String, u32>,
    pub security_events_count: u32,
    pub audit_integrity_hash: String,
    pub compressed_metadata: Option<Vec<u8>>,
}

/// Level 2: BPI Bundle Processor (1000 summaries → bundle → BPCI)
#[derive(Debug)]
pub struct BpiBundleProcessor {
    /// Accumulated BPI summaries
    pending_summaries: Arc<Mutex<VecDeque<ZipLockBatchSummary>>>,
    /// Bundle configuration
    config: BpiBundleConfig,
    /// Statistics
    stats: Arc<RwLock<BundleProcessorStats>>,
}

/// Configuration for BPI bundle processing
#[derive(Debug, Clone)]
pub struct BpiBundleConfig {
    pub bundle_size: usize,          // 1000 summaries per bundle
    pub max_bundle_age_seconds: u64, // Force bundle after timeout
    pub bpci_server_endpoint: String, // BPCI server submission endpoint
    pub enable_merkle_proofs: bool,   // Generate Merkle proofs
}

impl Default for BpiBundleConfig {
    fn default() -> Self {
        let default_endpoint = "http://localhost:7778/api/bundle/submit".to_string();
        let bpci_server_endpoint = std::env::var("BPI_BPCI_BUNDLE_ENDPOINT")
            .unwrap_or(default_endpoint);

        validate_endpoint_for_mode("BPI_BPCI_BUNDLE_ENDPOINT", &bpci_server_endpoint);

        Self {
            bundle_size: 1000,
            max_bundle_age_seconds: 3600, // 1 hour
            bpci_server_endpoint,
            enable_merkle_proofs: true,
        }
    }
}

fn is_bpci_server_enabled() -> bool {
    // If BPCI_SERVER_DISABLED is set to "1" or "true", treat the BPCI server
    // as inactive and skip submissions. Default is enabled.
    std::env::var("BPCI_SERVER_DISABLED")
        .map(|v| {
            let v = v.to_lowercase();
            !(v == "1" || v == "true")
        })
        .unwrap_or(true)
}

/// BPI bundle for BPCI server submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpiBundle {
    pub bundle_id: String,
    pub timestamp: DateTime<Utc>,
    pub summary_count: usize,
    pub total_audit_records: u64,
    pub bundle_merkle_root: String,
    pub economic_value: u64,         // Economic value for auction
    pub gas_estimate: u64,           // Gas estimate for processing
    pub priority_score: u16,         // Priority for auction (0-1000)
    pub compressed_summaries: Vec<u8>, // Compressed summary data
}

/// Level 3: BPCI Batch Bundle Processor (bundles → auction)
#[derive(Debug)]
pub struct BpciBatchBundleProcessor {
    /// Accumulated BPI bundles
    pending_bundles: Arc<Mutex<VecDeque<BpiBundle>>>,
    /// Batch configuration
    config: BpciBatchConfig,
    /// Statistics
    stats: Arc<RwLock<BatchBundleProcessorStats>>,
}

/// Configuration for BPCI batch bundle processing
#[derive(Debug, Clone)]
pub struct BpciBatchConfig {
    pub batch_bundle_size: usize,    // Number of bundles per batch
    pub max_batch_age_seconds: u64,  // Force batch after timeout
    pub auction_endpoint: String,    // Auction system endpoint
    pub min_economic_value: u64,     // Minimum value for auction
}

impl Default for BpciBatchConfig {
    fn default() -> Self {
        let default_auction_endpoint = "http://localhost:8080/api/auction/submit".to_string();
        let auction_endpoint = std::env::var("BPI_BPCI_AUCTION_ENDPOINT")
            .unwrap_or(default_auction_endpoint);

        validate_endpoint_for_mode("BPI_BPCI_AUCTION_ENDPOINT", &auction_endpoint);

        Self {
            batch_bundle_size: 10,       // 10 bundles per auction batch
            max_batch_age_seconds: 7200, // 2 hours
            auction_endpoint,
            min_economic_value: 1000000, // Minimum 1M units
        }
    }
}

fn is_auction_enabled() -> bool {
    // If BPCI_AUCTION_DISABLED is set to "1" or "true", treat the auction
    // endpoint as inactive and skip submissions. Default is enabled.
    std::env::var("BPCI_AUCTION_DISABLED")
        .map(|v| {
            let v = v.to_lowercase();
            !(v == "1" || v == "true")
        })
        .unwrap_or(true)
}

/// BPCI batch bundle for auction system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpciBatchBundle {
    pub batch_bundle_id: String,
    pub timestamp: DateTime<Utc>,
    pub bundle_count: usize,
    pub total_economic_value: u64,
    pub auction_reserve_price: u64,
    pub estimated_processing_time: u64,
    pub batch_merkle_root: String,
    pub revenue_sharing_info: RevenueSharing,
    pub compressed_bundles: Vec<u8>,
}

/// Revenue sharing configuration for auction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueSharing {
    pub community_share_percentage: f64,    // 20% to community
    pub roundtable_share_percentage: f64,   // 20% to roundtable
    pub validator_share_percentage: f64,    // 60% to validators
    pub treasury_allocation: u64,           // Fixed treasury allocation
}

impl Default for RevenueSharing {
    fn default() -> Self {
        Self {
            community_share_percentage: 0.20,
            roundtable_share_percentage: 0.20,
            validator_share_percentage: 0.60,
            treasury_allocation: 100000,
        }
    }
}

/// Statistics for batch processing
#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchProcessorStats {
    pub total_records_processed: u64,
    pub total_batches_created: u64,
    pub total_bpi_transactions: u64,
    pub average_batch_size: f64,
    pub processing_time_ms: u64,
    pub mesh_ledger_submissions: u64,
    pub http_ledger_submissions: u64,
    pub mesh_fallback_http_ledger_submissions: u64,
}

#[derive(Debug)]
struct AuditQuotaWindow {
    window_start: DateTime<Utc>,
    records_in_window: u64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BundleProcessorStats {
    pub total_summaries_processed: u64,
    pub total_bundles_created: u64,
    pub total_bpci_submissions: u64,
    pub average_bundle_size: f64,
    pub processing_time_ms: u64,
    pub mesh_bpci_submissions: u64,
    pub http_bpci_submissions: u64,
    pub mesh_fallback_http_bpci_submissions: u64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchBundleProcessorStats {
    pub total_bundles_processed: u64,
    pub total_batch_bundles_created: u64,
    pub total_auction_submissions: u64,
    pub total_economic_value: u64,
    pub average_auction_price: u64,
    pub mesh_auction_submissions: u64,
    pub http_auction_submissions: u64,
    pub mesh_fallback_http_auction_submissions: u64,
}

impl ZipLockBatchProcessor {
    /// Create new ZipLock batch processor
    pub fn new(config: ZipLockBatchConfig) -> Self {
        Self {
            pending_records: Arc::new(Mutex::new(VecDeque::new())),
            config,
            stats: Arc::new(RwLock::new(BatchProcessorStats::default())),
            quota_window: Arc::new(Mutex::new(AuditQuotaWindow {
                window_start: Utc::now(),
                records_in_window: 0,
            })),
        }
    }

    /// Add ZipLock record to batch
    pub async fn add_record(&self, record: ZipLockJsonAudit) -> Result<Option<ZipLockBatchSummary>> {
        let mut pending = self.pending_records.lock().unwrap();
        pending.push_back(record);

        // Check if batch is ready
        if pending.len() >= self.config.batch_size {
            let batch_records: Vec<ZipLockJsonAudit> = pending.drain(..).collect();
            drop(pending);
            
            return Ok(Some(self.create_batch_summary(batch_records).await?));
        }

        Ok(None)
    }

    fn consume_quota(&self, records: u64) -> bool {
        let max = max_audit_records_per_hour();
        if max == 0 {
            return true;
        }

        let mut window = self.quota_window.lock().unwrap();
        let now = Utc::now();
        let elapsed = now - window.window_start;
        if elapsed.num_seconds() >= 3600 {
            window.window_start = now;
            window.records_in_window = 0;
        }

        if window.records_in_window + records > max {
            false
        } else {
            window.records_in_window += records;
            true
        }
    }

    /// Create batch summary from 100 ZipLock records
    async fn create_batch_summary(&self, records: Vec<ZipLockJsonAudit>) -> Result<ZipLockBatchSummary> {
        let batch_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        
        // Calculate statistics
        let record_count = records.len();
        let total_size_bytes = records.iter()
            .map(|r| serde_json::to_string(r).unwrap_or_default().len() as u64)
            .sum();

        // VM type distribution
        let mut vm_type_distribution = HashMap::new();
        let mut security_events_count = 0;

        for record in &records {
            // Extract VM type from metadata
            if let Some(vm_type) = record.metadata.get("vm_type").and_then(|v| v.as_str()) {
                *vm_type_distribution.entry(vm_type.to_string()).or_insert(0) += 1;
            }

            // Count security events
            if let Some(security) = record.payload.get("security_event") {
                if security.as_bool().unwrap_or(false) {
                    security_events_count += 1;
                }
            }
        }

        // Calculate Merkle root
        let merkle_root = self.calculate_merkle_root(&records)?;
        
        // Calculate audit integrity hash
        let audit_integrity_hash = self.calculate_integrity_hash(&records)?;

        // Compress metadata if enabled
        let compressed_metadata = if self.config.enable_compression {
            Some(self.compress_metadata(&records)?)
        } else {
            None
        };

        let summary = ZipLockBatchSummary {
            batch_id,
            timestamp,
            record_count,
            total_size_bytes,
            merkle_root,
            vm_type_distribution,
            security_events_count,
            audit_integrity_hash,
            compressed_metadata,
        };

        // Best-effort CBOR archival of the batch summary using canonical
        // bounded serialization. This writes a compact, deterministic
        // representation suitable for long-term storage without impacting
        // existing JSON or ledger behaviour.
        const MAX_CBOR_BATCH_ARCHIVE_BYTES: usize = 512 * 1024; // 512 KiB
        match serialize_canonical_bounded(&summary, MAX_CBOR_BATCH_ARCHIVE_BYTES) {
            Ok(cbor_bytes) => {
                let archive_dir = std::env::var("BPI_ZIPLOCK_CBOR_ARCHIVE_DIR")
                    .unwrap_or_else(|_| "audit_cbor".to_string());
                if let Err(e) = fs::create_dir_all(&archive_dir) {
                    tracing::warn!(
                        "⚠️ Failed to create CBOR archive directory {}: {}",
                        archive_dir,
                        e
                    );
                } else {
                    let path = PathBuf::from(&archive_dir)
                        .join(format!("ziplock_batch_{}.cbor", summary.batch_id));
                    match fs::write(&path, &cbor_bytes) {
                        Ok(()) => {
                            tracing::info!(
                                "💾 CBOR archive written for ZipLock batch summary: {}",
                                path.display()
                            );
                        }
                        Err(e) => {
                            tracing::warn!(
                                "⚠️ Failed to write CBOR archive {}: {}",
                                path.display(),
                                e
                            );
                        }
                    }
                }
            }
            Err(e) => {
                tracing::warn!(
                    "⚠️ Failed to serialize ZipLock batch summary to bounded CBOR: {}",
                    e
                );
            }
        }

        if self.consume_quota(record_count as u64) {
            self.submit_to_bpi_ledger(&summary).await?;
        } else {
            tracing::warn!(
                "⚠️ Hourly audit quota exceeded (limit: {}, attempted: {}); skipping BPI ledger submission for batch {}",
                max_audit_records_per_hour(),
                record_count,
                summary.batch_id
            );
        }

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_records_processed += record_count as u64;
        stats.total_batches_created += 1;
        stats.total_bpi_transactions += 1;
        stats.average_batch_size = stats.total_records_processed as f64 / stats.total_batches_created as f64;

        tracing::info!(
            "📦 Created ZipLock batch summary: {} ({} records, {} bytes)",
            summary.batch_id,
            record_count,
            total_size_bytes
        );

        Ok(summary)
    }

    /// Calculate Merkle root for batch integrity
    fn calculate_merkle_root(&self, records: &[ZipLockJsonAudit]) -> Result<String> {
        let mut hasher = Sha256::new();
        
        for record in records {
            let record_hash = serde_json::to_string(record)?;
            hasher.update(record_hash.as_bytes());
        }
        
        Ok(format!("0x{:x}", hasher.finalize()))
    }

    /// Calculate audit integrity hash
    fn calculate_integrity_hash(&self, records: &[ZipLockJsonAudit]) -> Result<String> {
        let mut hasher = Sha256::new();
        
        // Hash all integrity fields
        for record in records {
            if let Some(integrity) = record.integrity.as_object() {
                let integrity_str = serde_json::to_string(integrity)?;
                hasher.update(integrity_str.as_bytes());
            }
        }
        
        Ok(format!("0x{:x}", hasher.finalize()))
    }

    /// Compress metadata for efficient storage
    fn compress_metadata(&self, records: &[ZipLockJsonAudit]) -> Result<Vec<u8>> {
        let metadata: Vec<_> = records.iter().map(|r| &r.metadata).collect();
        let serialized = serde_json::to_vec(&metadata)?;
        
        // Simple compression (in production, use proper compression library)
        Ok(serialized)
    }

    /// Submit batch summary to BPI ledger
    async fn submit_to_bpi_ledger(&self, summary: &ZipLockBatchSummary) -> Result<()> {
        if !is_bpi_ledger_enabled() {
            tracing::info!(
                "⏸️ BPI ledger submission disabled via BPI_LEDGER_DISABLED; skipping ZipLock batch {}",
                summary.batch_id
            );
            return Ok(());
        }
        // Try mesh-native submission first (if enabled)
        if config::is_mesh_internal_enabled() {
            let mesh_result: Result<()> = async {
                let kernel = BlockchainOSKernel::new().await?;
                let payload = serde_json::to_vec(summary)?;
                kernel
                    .send_mesh_message(
                        "bpi.ledger.submit_batch",
                        &payload,
                        MessageType::Data,
                        Priority::Normal,
                    )
                    .await
            }
            .await;

            match mesh_result {
                Ok(()) => {
                    {
                        let mut stats = self.stats.write().await;
                        stats.mesh_ledger_submissions += 1;
                    }
                    tracing::info!(
                        "✅ ZipLock batch summary submitted to BPI ledger via mesh-native path: {}",
                        summary.batch_id
                    );
                    return Ok(());
                }
                Err(e) => {
                    {
                        let mut stats = self.stats.write().await;
                        stats.mesh_fallback_http_ledger_submissions += 1;
                    }
                    tracing::warn!(
                        "⚠️ Mesh-native submission of ZipLock batch summary to BPI ledger failed, falling back to HTTP: {}",
                        e
                    );
                }
            }
        }

        let client = reqwest::Client::new();

        let response = match client
            .post(&self.config.bpi_ledger_endpoint)
            .json(&serde_json::json!({
                "transaction_type": "ziplock_batch_summary",
                "batch_id": summary.batch_id,
                "summary": summary,
                "timestamp": summary.timestamp.to_rfc3339(),
                "economic_metadata": economic_metadata_for_batch(summary),
            }))
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                tracing::warn!(
                    "⚠️ Network error while submitting batch summary to BPI ledger ({}): {}",
                    self.config.bpi_ledger_endpoint,
                    e
                );
                return Ok(());
            }
        };

        if response.status().is_success() {
            tracing::info!("✅ ZipLock batch summary submitted to BPI ledger: {}", summary.batch_id);
        } else {
            tracing::warn!(
                "⚠️ Failed to submit batch summary to BPI ledger (status {}): {}",
                response.status(),
                summary.batch_id
            );
        }

        {
            let mut stats = self.stats.write().await;
            stats.http_ledger_submissions += 1;
        }

        Ok(())
    }

    /// Get current statistics
    pub async fn get_stats(&self) -> BatchProcessorStats {
        self.stats.read().await.clone()
    }
}

impl BpiBundleProcessor {
    /// Create new BPI bundle processor
    pub fn new(config: BpiBundleConfig) -> Self {
        Self {
            pending_summaries: Arc::new(Mutex::new(VecDeque::new())),
            config,
            stats: Arc::new(RwLock::new(BundleProcessorStats::default())),
        }
    }

    /// Add batch summary to bundle
    pub async fn add_summary(&self, summary: ZipLockBatchSummary) -> Result<Option<BpiBundle>> {
        let mut pending = self.pending_summaries.lock().unwrap();
        pending.push_back(summary);

        // Check if bundle is ready
        if pending.len() >= self.config.bundle_size {
            let bundle_summaries: Vec<ZipLockBatchSummary> = pending.drain(..).collect();
            drop(pending);
            
            return Ok(Some(self.create_bpi_bundle(bundle_summaries).await?));
        }

        Ok(None)
    }

    /// Create BPI bundle from 1000 summaries
    async fn create_bpi_bundle(&self, summaries: Vec<ZipLockBatchSummary>) -> Result<BpiBundle> {
        let bundle_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        
        // Calculate bundle statistics
        let summary_count = summaries.len();
        let total_audit_records: u64 = summaries.iter()
            .map(|s| s.record_count as u64)
            .sum();

        // Calculate economic value based on audit volume and security events
        let economic_value = self.calculate_economic_value(&summaries);
        
        // Estimate gas for processing
        let gas_estimate = self.estimate_gas_cost(&summaries);
        
        // Calculate priority score
        let priority_score = self.calculate_priority_score(&summaries);

        // Calculate bundle Merkle root
        let bundle_merkle_root = self.calculate_bundle_merkle_root(&summaries)?;

        // Compress summaries
        let compressed_summaries = self.compress_summaries(&summaries)?;

        let bundle = BpiBundle {
            bundle_id,
            timestamp,
            summary_count,
            total_audit_records,
            bundle_merkle_root,
            economic_value,
            gas_estimate,
            priority_score,
            compressed_summaries,
        };

        // Submit to BPCI server
        self.submit_to_bpci_server(&bundle).await?;

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_summaries_processed += summary_count as u64;
        stats.total_bundles_created += 1;
        stats.total_bpci_submissions += 1;
        stats.average_bundle_size = stats.total_summaries_processed as f64 / stats.total_bundles_created as f64;

        tracing::info!(
            "📦 Created BPI bundle: {} ({} summaries, {} audit records, value: {})",
            bundle.bundle_id,
            summary_count,
            total_audit_records,
            economic_value
        );

        Ok(bundle)
    }

    /// Calculate economic value for bundle
    fn calculate_economic_value(&self, summaries: &[ZipLockBatchSummary]) -> u64 {
        let base_value = summaries.len() as u64 * 1000; // Base value per summary
        
        // Add bonus for security events
        let security_bonus: u64 = summaries.iter()
            .map(|s| s.security_events_count as u64 * 500)
            .sum();
        
        // Add bonus for data volume
        let volume_bonus: u64 = summaries.iter()
            .map(|s| s.total_size_bytes / 1000) // Bonus per KB
            .sum();

        base_value + security_bonus + volume_bonus
    }

    /// Estimate gas cost for bundle processing
    fn estimate_gas_cost(&self, summaries: &[ZipLockBatchSummary]) -> u64 {
        let base_gas = 21000; // Base transaction cost
        let data_gas = summaries.iter()
            .map(|s| s.total_size_bytes * 16) // 16 gas per byte
            .sum::<u64>();
        
        base_gas + data_gas
    }

    /// Calculate priority score for auction
    fn calculate_priority_score(&self, summaries: &[ZipLockBatchSummary]) -> u16 {
        let security_weight = summaries.iter()
            .map(|s| s.security_events_count as u16)
            .sum::<u16>();
        
        let volume_weight = (summaries.len() / 10) as u16; // Volume bonus
        
        std::cmp::min(1000, 500 + security_weight + volume_weight)
    }

    /// Calculate bundle Merkle root
    fn calculate_bundle_merkle_root(&self, summaries: &[ZipLockBatchSummary]) -> Result<String> {
        let mut hasher = Sha256::new();
        
        for summary in summaries {
            hasher.update(summary.merkle_root.as_bytes());
        }
        
        Ok(format!("0x{:x}", hasher.finalize()))
    }

    /// Compress summaries for efficient transmission
    fn compress_summaries(&self, summaries: &[ZipLockBatchSummary]) -> Result<Vec<u8>> {
        let serialized = serde_json::to_vec(summaries)?;
        // Simple compression (in production, use proper compression library)
        Ok(serialized)
    }

    /// Submit bundle to BPCI server
    async fn submit_to_bpci_server(&self, bundle: &BpiBundle) -> Result<()> {
        if !is_bpci_server_enabled() {
            tracing::info!(
                "⏸️ BPCI server submission disabled via BPCI_SERVER_DISABLED; skipping BPI bundle {}",
                bundle.bundle_id
            );
            return Ok(());
        }
        // Try mesh-native submission first (if enabled)
        if config::is_mesh_internal_enabled() {
            let mesh_result: Result<()> = async {
                let kernel = BlockchainOSKernel::new().await?;
                let payload = serde_json::to_vec(bundle)?;
                kernel
                    .send_mesh_message(
                        "bpi.bpci.submit_bundle",
                        &payload,
                        MessageType::Data,
                        Priority::Normal,
                    )
                    .await
            }
            .await;

            match mesh_result {
                Ok(()) => {
                    {
                        let mut stats = self.stats.write().await;
                        stats.mesh_bpci_submissions += 1;
                    }
                    tracing::info!(
                        "✅ BPI bundle submitted to BPCI server via mesh-native path: {}",
                        bundle.bundle_id
                    );
                    return Ok(());
                }
                Err(e) => {
                    {
                        let mut stats = self.stats.write().await;
                        stats.mesh_fallback_http_bpci_submissions += 1;
                    }
                    tracing::warn!(
                        "⚠️ Mesh-native submission of BPI bundle to BPCI server failed, falling back to HTTP: {}",
                        e
                    );
                }
            }
        }

        let client = reqwest::Client::new();

        let response = match client
            .post(&self.config.bpci_server_endpoint)
            .json(&serde_json::json!({
                "type": "bundle_submission",
                "bundle_id": bundle.bundle_id,
                "bundle": bundle,
                "timestamp": bundle.timestamp.to_rfc3339(),
                "economic_metadata": economic_metadata_for_bundle(bundle),
            }))
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                tracing::warn!(
                    "⚠️ Network error while submitting BPI bundle to BPCI server ({}): {}",
                    self.config.bpci_server_endpoint,
                    e
                );
                return Ok(());
            }
        };

        if response.status().is_success() {
            tracing::info!("✅ BPI bundle submitted to BPCI server: {}", bundle.bundle_id);
        } else {
            tracing::warn!(
                "⚠️ Failed to submit bundle to BPCI server (status {}): {}",
                response.status(),
                bundle.bundle_id
            );
        }

        {
            let mut stats = self.stats.write().await;
            stats.http_bpci_submissions += 1;
        }

        Ok(())
    }

    /// Get current statistics
    pub async fn get_stats(&self) -> BundleProcessorStats {
        self.stats.read().await.clone()
    }
}

impl BpciBatchBundleProcessor {
    /// Create new BPCI batch bundle processor
    pub fn new(config: BpciBatchConfig) -> Self {
        Self {
            pending_bundles: Arc::new(Mutex::new(VecDeque::new())),
            config,
            stats: Arc::new(RwLock::new(BatchBundleProcessorStats::default())),
        }
    }

    /// Add BPI bundle to batch
    pub async fn add_bundle(&self, bundle: BpiBundle) -> Result<Option<BpciBatchBundle>> {
        let mut pending = self.pending_bundles.lock().unwrap();
        pending.push_back(bundle);

        // Check if batch bundle is ready
        if pending.len() >= self.config.batch_bundle_size {
            let batch_bundles: Vec<BpiBundle> = pending.drain(..).collect();
            drop(pending);
            
            return Ok(Some(self.create_batch_bundle(batch_bundles).await?));
        }

        Ok(None)
    }

    /// Create BPCI batch bundle for auction
    async fn create_batch_bundle(&self, bundles: Vec<BpiBundle>) -> Result<BpciBatchBundle> {
        let batch_bundle_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        
        // Calculate batch statistics
        let bundle_count = bundles.len();
        let total_economic_value: u64 = bundles.iter()
            .map(|b| b.economic_value)
            .sum();

        // Check minimum economic value
        if total_economic_value < self.config.min_economic_value {
            return Err(anyhow::anyhow!(
                "Batch economic value {} below minimum {}",
                total_economic_value,
                self.config.min_economic_value
            ));
        }

        // Calculate auction reserve price (80% of economic value)
        let auction_reserve_price = (total_economic_value as f64 * 0.8) as u64;
        
        // Estimate processing time
        let estimated_processing_time = bundles.iter()
            .map(|b| b.gas_estimate / 1000) // Rough time estimate
            .sum();

        // Calculate batch Merkle root
        let batch_merkle_root = self.calculate_batch_merkle_root(&bundles)?;

        // Revenue sharing configuration
        let revenue_sharing_info = RevenueSharing::default();

        // Compress bundles
        let compressed_bundles = self.compress_bundles(&bundles)?;

        let batch_bundle = BpciBatchBundle {
            batch_bundle_id,
            timestamp,
            bundle_count,
            total_economic_value,
            auction_reserve_price,
            estimated_processing_time,
            batch_merkle_root,
            revenue_sharing_info,
            compressed_bundles,
        };

        // Submit to auction system
        self.submit_to_auction(&batch_bundle).await?;

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_bundles_processed += bundle_count as u64;
        stats.total_batch_bundles_created += 1;
        stats.total_auction_submissions += 1;
        stats.total_economic_value += total_economic_value;
        stats.average_auction_price = stats.total_economic_value / stats.total_auction_submissions;

        tracing::info!(
            "🏆 Created BPCI batch bundle for auction: {} ({} bundles, value: {}, reserve: {})",
            batch_bundle.batch_bundle_id,
            bundle_count,
            total_economic_value,
            auction_reserve_price
        );

        Ok(batch_bundle)
    }

    /// Calculate batch Merkle root
    fn calculate_batch_merkle_root(&self, bundles: &[BpiBundle]) -> Result<String> {
        let mut hasher = Sha256::new();
        
        for bundle in bundles {
            hasher.update(bundle.bundle_merkle_root.as_bytes());
        }
        
        Ok(format!("0x{:x}", hasher.finalize()))
    }

    /// Compress bundles for auction
    fn compress_bundles(&self, bundles: &[BpiBundle]) -> Result<Vec<u8>> {
        let serialized = serde_json::to_vec(bundles)?;
        // Simple compression (in production, use proper compression library)
        Ok(serialized)
    }

    /// Submit batch bundle to auction system
    async fn submit_to_auction(&self, batch_bundle: &BpciBatchBundle) -> Result<()> {
        if !is_auction_enabled() {
            tracing::info!(
                "⏸️ Auction submission disabled via BPCI_AUCTION_DISABLED; skipping batch bundle {}",
                batch_bundle.batch_bundle_id
            );
            return Ok(());
        }
        // Try mesh-native submission first (if enabled)
        if config::is_mesh_internal_enabled() {
            let mesh_result: Result<()> = async {
                let kernel = BlockchainOSKernel::new().await?;
                let payload = serde_json::to_vec(batch_bundle)?;
                kernel
                    .send_mesh_message(
                        "bpi.auction.submit_batch_bundle",
                        &payload,
                        MessageType::Data,
                        Priority::Normal,
                    )
                    .await
            }
            .await;

            match mesh_result {
                Ok(()) => {
                    {
                        let mut stats = self.stats.write().await;
                        stats.mesh_auction_submissions += 1;
                    }
                    tracing::info!(
                        "🏆 BPCI batch bundle submitted to auction via mesh-native path: {}",
                        batch_bundle.batch_bundle_id
                    );
                    return Ok(());
                }
                Err(e) => {
                    {
                        let mut stats = self.stats.write().await;
                        stats.mesh_fallback_http_auction_submissions += 1;
                    }
                    tracing::warn!(
                        "⚠️ Mesh-native submission of BPCI batch bundle to auction failed, falling back to HTTP: {}",
                        e
                    );
                }
            }
        }

        let client = reqwest::Client::new();

        let response = match client
            .post(&self.config.auction_endpoint)
            .json(&serde_json::json!({
                "auction_type": "bpci_batch_bundle",
                "batch_bundle_id": batch_bundle.batch_bundle_id,
                "batch_bundle": batch_bundle,
                "timestamp": batch_bundle.timestamp.to_rfc3339(),
                "economic_metadata": economic_metadata_for_batch_bundle(batch_bundle),
            }))
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                tracing::warn!(
                    "⚠️ Network error while submitting BPCI batch bundle to auction ({}): {}",
                    self.config.auction_endpoint,
                    e
                );
                return Ok(());
            }
        };

        if response.status().is_success() {
            tracing::info!(
                "🏆 BPCI batch bundle submitted to auction: {}",
                batch_bundle.batch_bundle_id
            );
        } else {
            tracing::warn!(
                "⚠️ Failed to submit batch bundle to auction (status {}): {}",
                response.status(),
                batch_bundle.batch_bundle_id
            );
        }

        {
            let mut stats = self.stats.write().await;
            stats.http_auction_submissions += 1;
        }

        Ok(())
    }

    /// Get current statistics
    pub async fn get_stats(&self) -> BatchBundleProcessorStats {
        self.stats.read().await.clone()
    }
}

/// Master 3-Tier Audit Batch Coordinator
#[derive(Debug)]
pub struct AuditBatchCoordinator {
    pub ziplock_processor: ZipLockBatchProcessor,
    pub bundle_processor: BpiBundleProcessor,
    pub batch_bundle_processor: BpciBatchBundleProcessor,
}

impl AuditBatchCoordinator {
    /// Create new audit batch coordinator with default configuration
    pub fn new() -> Self {
        Self {
            ziplock_processor: ZipLockBatchProcessor::new(ZipLockBatchConfig::default()),
            bundle_processor: BpiBundleProcessor::new(BpiBundleConfig::default()),
            batch_bundle_processor: BpciBatchBundleProcessor::new(BpciBatchConfig::default()),
        }
    }

    /// Process ZipLock audit through complete 3-tier pipeline
    pub async fn process_ziplock_audit(&self, audit: ZipLockJsonAudit) -> Result<()> {
        // Level 1: Add to ZipLock batch
        if let Some(summary) = self.ziplock_processor.add_record(audit).await? {
            tracing::info!("📦 Level 1: ZipLock batch summary created: {}", summary.batch_id);
            
            // Level 2: Add summary to BPI bundle
            if let Some(bundle) = self.bundle_processor.add_summary(summary).await? {
                tracing::info!("📦 Level 2: BPI bundle created: {}", bundle.bundle_id);
                
                // Level 3: Add bundle to BPCI batch for auction
                if let Some(batch_bundle) = self.batch_bundle_processor.add_bundle(bundle).await? {
                    tracing::info!("🏆 Level 3: BPCI batch bundle created for auction: {}", batch_bundle.batch_bundle_id);
                }
            }
        }

        Ok(())
    }

    /// Get comprehensive statistics across all levels
    pub async fn get_comprehensive_stats(&self) -> serde_json::Value {
        serde_json::json!({
            "level_1_ziplock_batching": self.ziplock_processor.get_stats().await,
            "level_2_bpi_bundling": self.bundle_processor.get_stats().await,
            "level_3_bpci_auction": self.batch_bundle_processor.get_stats().await,
            "timestamp": Utc::now().to_rfc3339()
        })
    }
}

impl Default for AuditBatchCoordinator {
    fn default() -> Self {
        Self::new()
    }
}
