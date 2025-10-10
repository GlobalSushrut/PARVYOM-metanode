//! BPCI Bundle Receiver - Dedicated service for receiving BPI bundles
//! 
//! This critical component provides a dedicated endpoint for receiving BPI PoEProofBundle
//! submissions, converts them to auction transactions, and integrates with the BPCI auction system.

use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use tracing::{info, warn, error};

use crate::bpi_bundle_converter::{BpiBundleConverter, PoEProofBundle, BundleConversionResult};
use crate::bpci_auction_mempool::BpciAuctionMempool;
use crate::bpci_bundle_ledger::{BpciBundleLedger, BundleReceipt};

/// BPCI Bundle Receiver - Main service for handling BPI bundle submissions
#[derive(Debug)]
pub struct BpciBundleReceiver {
    /// Bundle converter for PoEProofBundle → AuctionTransaction conversion
    pub bundle_converter: Arc<Mutex<BpiBundleConverter>>,
    /// Auction mempool for transaction submission
    pub auction_mempool: Arc<Mutex<BpciAuctionMempool>>,
    /// Bundle ledger for immutable receipt storage
    pub bundle_ledger: Arc<RwLock<BpciBundleLedger>>,
    /// Receiver configuration
    pub config: BundleReceiverConfig,
    /// Reception metrics and statistics
    pub metrics: Arc<RwLock<ReceptionMetrics>>,
    /// Active bundle processing queue
    pub processing_queue: Arc<RwLock<HashMap<String, ProcessingBundle>>>,
}

/// Bundle receiver configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleReceiverConfig {
    /// Maximum concurrent bundle processing
    pub max_concurrent_processing: usize,
    /// Bundle validation timeout (seconds)
    pub validation_timeout_secs: u64,
    /// Enable strict validation mode
    pub strict_validation: bool,
    /// Auction submission timeout (seconds)
    pub auction_submission_timeout_secs: u64,
    /// Enable real-time status updates
    pub enable_real_time_updates: bool,
}

/// Reception metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceptionMetrics {
    pub total_bundles_received: u64,
    pub total_bundles_processed: u64,
    pub total_bundles_rejected: u64,
    pub total_auction_transactions_created: u64,
    pub average_processing_time_ms: f64,
    pub last_bundle_received: Option<DateTime<Utc>>,
    pub processing_errors: u64,
    pub validation_failures: u64,
}

/// Bundle processing state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingBundle {
    pub bundle_id: String,
    pub received_at: DateTime<Utc>,
    pub status: ProcessingStatus,
    pub progress: f64,
    pub conversion_result: Option<BundleConversionResult>,
    pub error_message: Option<String>,
    pub auction_submission_ids: Vec<String>,
}

/// Processing status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProcessingStatus {
    Received,
    Validating,
    Converting,
    SubmittingToAuction,
    RecordingReceipt,
    Completed,
    Failed,
}

/// Bundle reception response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleReceptionResponse {
    pub success: bool,
    pub bundle_id: String,
    pub receipt_id: String,
    pub processing_id: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub auction_transaction_count: usize,
    pub estimated_processing_time_ms: u64,
}

/// Bundle status query response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleStatusResponse {
    pub bundle_id: String,
    pub status: ProcessingStatus,
    pub progress: f64,
    pub auction_transactions_created: usize,
    pub auction_submissions_completed: usize,
    pub receipt_recorded: bool,
    pub processing_time_ms: u64,
    pub error_message: Option<String>,
}

impl BpciBundleReceiver {
    /// Create new BPCI bundle receiver
    pub async fn new(
        auction_mempool: Arc<Mutex<BpciAuctionMempool>>,
        bundle_ledger: Arc<RwLock<BpciBundleLedger>>,
    ) -> Result<Self> {
        let bundle_converter = Arc::new(Mutex::new(BpiBundleConverter::new()));
        let config = BundleReceiverConfig::default();
        let metrics = Arc::new(RwLock::new(ReceptionMetrics::new()));
        let processing_queue = Arc::new(RwLock::new(HashMap::new()));

        Ok(Self {
            bundle_converter,
            auction_mempool,
            bundle_ledger,
            config,
            metrics,
            processing_queue,
        })
    }

    /// Create receiver with custom configuration
    pub async fn new_with_config(
        auction_mempool: Arc<Mutex<BpciAuctionMempool>>,
        bundle_ledger: Arc<RwLock<BpciBundleLedger>>,
        config: BundleReceiverConfig,
    ) -> Result<Self> {
        let bundle_converter = Arc::new(Mutex::new(BpiBundleConverter::new()));
        let metrics = Arc::new(RwLock::new(ReceptionMetrics::new()));
        let processing_queue = Arc::new(RwLock::new(HashMap::new()));

        Ok(Self {
            bundle_converter,
            auction_mempool,
            bundle_ledger,
            config,
            metrics,
            processing_queue,
        })
    }

    /// Receive and process BPI bundle
    pub async fn receive_bpi_bundle(&self, bundle: PoEProofBundle) -> Result<BundleReceptionResponse> {
        let start_time = std::time::Instant::now();
        let processing_id = Uuid::new_v4().to_string();
        
        info!("📦 Receiving BPI bundle: {} (processing_id: {})", bundle.bundle_id, processing_id);

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.total_bundles_received += 1;
            metrics.last_bundle_received = Some(Utc::now());
        }

        // Create processing entry
        let mut processing_bundle = ProcessingBundle {
            bundle_id: bundle.bundle_id.clone(),
            received_at: Utc::now(),
            status: ProcessingStatus::Received,
            progress: 0.0,
            conversion_result: None,
            error_message: None,
            auction_submission_ids: Vec::new(),
        };

        // Add to processing queue
        {
            let mut queue = self.processing_queue.write().await;
            queue.insert(processing_id.clone(), processing_bundle.clone());
        }

        // Process bundle asynchronously
        let result = self.process_bundle_internal(bundle, processing_id.clone()).await;

        match result {
            Ok(response) => {
                // Update processing status to completed
                {
                    let mut queue = self.processing_queue.write().await;
                    if let Some(processing) = queue.get_mut(&processing_id) {
                        processing.status = ProcessingStatus::Completed;
                        processing.progress = 1.0;
                    }
                }

                // Update metrics
                {
                    let mut metrics = self.metrics.write().await;
                    metrics.total_bundles_processed += 1;
                    let processing_time = start_time.elapsed().as_millis() as f64;
                    metrics.update_average_processing_time(processing_time);
                }

                info!("✅ Successfully processed BPI bundle: {}", response.bundle_id);
                Ok(response)
            }
            Err(e) => {
                // Update processing status to failed
                {
                    let mut queue = self.processing_queue.write().await;
                    if let Some(processing) = queue.get_mut(&processing_id) {
                        processing.status = ProcessingStatus::Failed;
                        processing.error_message = Some(e.to_string());
                    }
                }

                // Update metrics
                {
                    let mut metrics = self.metrics.write().await;
                    metrics.total_bundles_rejected += 1;
                    metrics.processing_errors += 1;
                }

                error!("❌ Failed to process BPI bundle: {}", e);
                Err(e)
            }
        }
    }

    /// Internal bundle processing logic
    async fn process_bundle_internal(&self, bundle: PoEProofBundle, processing_id: String) -> Result<BundleReceptionResponse> {
        // Step 1: Validate bundle
        self.update_processing_status(&processing_id, ProcessingStatus::Validating, 0.1).await;
        self.validate_bundle(&bundle).await?;

        // Step 2: Convert bundle to auction transactions
        self.update_processing_status(&processing_id, ProcessingStatus::Converting, 0.3).await;
        let conversion_result = {
            let mut converter = self.bundle_converter.lock().await;
            converter.convert_bundle(&bundle).await?
        };

        // Update processing with conversion result
        {
            let mut queue = self.processing_queue.write().await;
            if let Some(processing) = queue.get_mut(&processing_id) {
                processing.conversion_result = Some(conversion_result.clone());
            }
        }

        // Step 3: Submit auction transactions to mempool
        self.update_processing_status(&processing_id, ProcessingStatus::SubmittingToAuction, 0.6).await;
        let auction_submission_ids = self.submit_to_auction_mempool(&conversion_result).await?;

        // Update processing with auction submission IDs
        {
            let mut queue = self.processing_queue.write().await;
            if let Some(processing) = queue.get_mut(&processing_id) {
                processing.auction_submission_ids = auction_submission_ids.clone();
            }
        }

        // Step 4: Record immutable receipt in bundle ledger
        self.update_processing_status(&processing_id, ProcessingStatus::RecordingReceipt, 0.9).await;
        let receipt_id = self.record_bundle_receipt(&bundle, &conversion_result).await?;

        // Create response
        let response = BundleReceptionResponse {
            success: true,
            bundle_id: bundle.bundle_id.clone(),
            receipt_id,
            processing_id,
            message: format!("Bundle processed successfully: {} auction transactions created", 
                           conversion_result.auction_transactions.len()),
            timestamp: Utc::now(),
            auction_transaction_count: conversion_result.auction_transactions.len(),
            estimated_processing_time_ms: 500, // Estimated
        };

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.total_auction_transactions_created += conversion_result.auction_transactions.len() as u64;
        }

        Ok(response)
    }

    /// Validate incoming bundle
    async fn validate_bundle(&self, bundle: &PoEProofBundle) -> Result<()> {
        // Basic validation
        if bundle.bundle_id.is_empty() {
            return Err(anyhow!("Bundle ID is empty"));
        }

        if bundle.bundle_hash.is_empty() {
            return Err(anyhow!("Bundle hash is empty"));
        }

        if bundle.transaction_count == 0 {
            return Err(anyhow!("Bundle has no transactions"));
        }

        // Strict validation if enabled
        if self.config.strict_validation {
            // Validate notary signatures
            if bundle.notary_approvals.is_empty() {
                return Err(anyhow!("No notary approvals found (strict validation enabled)"));
            }

            // Validate immutable proof
            if bundle.immutable_proof.proof_hash.is_empty() {
                return Err(anyhow!("Missing immutable proof hash"));
            }

            // Validate BPI ledger metadata
            if bundle.bpi_ledger_metadata.node_id.is_empty() {
                return Err(anyhow!("Missing BPI node ID"));
            }
        }

        info!("✅ Bundle validation passed: {}", bundle.bundle_id);
        Ok(())
    }

    /// Submit auction transactions to mempool
    async fn submit_to_auction_mempool(&self, conversion_result: &BundleConversionResult) -> Result<Vec<String>> {
        let mut submission_ids = Vec::new();
        let mut mempool = self.auction_mempool.lock().await;

        for auction_tx in &conversion_result.auction_transactions {
            match mempool.submit_transaction(auction_tx.clone()) {
                Ok(()) => {
                    let submission_id = format!("auction-{}", hex::encode(&auction_tx.tx_id[..8]));
                    submission_ids.push(submission_id);
                }
                Err(e) => {
                    warn!("⚠️ Failed to submit auction transaction: {}", e);
                    // Continue with other transactions
                }
            }
        }

        if submission_ids.is_empty() {
            return Err(anyhow!("Failed to submit any auction transactions"));
        }

        info!("✅ Submitted {} auction transactions to mempool", submission_ids.len());
        Ok(submission_ids)
    }

    /// Record immutable receipt in bundle ledger
    async fn record_bundle_receipt(&self, bundle: &PoEProofBundle, conversion_result: &BundleConversionResult) -> Result<String> {
        let receipt = BundleReceipt {
            receipt_id: Uuid::new_v4().to_string(),
            bundle_id: bundle.bundle_id.clone(),
            original_bundle_hash: bundle.bundle_hash.clone(),
            received_at: Utc::now(),
            conversion_summary: conversion_result.conversion_summary.clone(),
            auction_transaction_count: conversion_result.auction_transactions.len(),
            immutable_proof: conversion_result.immutable_receipt.clone(),
            ledger_block_height: 0, // Will be set by ledger
            consensus_signatures: Vec::new(), // Will be populated by consensus
        };

        let mut ledger = self.bundle_ledger.write().await;
        let receipt_id = ledger.record_bundle_receipt(receipt).await?;

        info!("✅ Recorded immutable bundle receipt: {}", receipt_id);
        Ok(receipt_id)
    }

    /// Update processing status
    async fn update_processing_status(&self, processing_id: &str, status: ProcessingStatus, progress: f64) {
        let mut queue = self.processing_queue.write().await;
        if let Some(processing) = queue.get_mut(processing_id) {
            processing.status = status;
            processing.progress = progress;
        }
    }

    /// Get bundle processing status
    pub async fn get_bundle_status(&self, processing_id: &str) -> Result<BundleStatusResponse> {
        let queue = self.processing_queue.read().await;
        
        if let Some(processing) = queue.get(processing_id) {
            let processing_time = Utc::now().signed_duration_since(processing.received_at).num_milliseconds() as u64;
            
            Ok(BundleStatusResponse {
                bundle_id: processing.bundle_id.clone(),
                status: processing.status.clone(),
                progress: processing.progress,
                auction_transactions_created: processing.conversion_result.as_ref().map(|r| r.auction_transactions.len()).unwrap_or(0),
                auction_submissions_completed: processing.auction_submission_ids.len(),
                receipt_recorded: processing.status == ProcessingStatus::Completed,
                processing_time_ms: processing_time,
                error_message: processing.error_message.clone(),
            })
        } else {
            Err(anyhow!("Processing ID not found: {}", processing_id))
        }
    }

    /// Get reception metrics
    pub async fn get_metrics(&self) -> ReceptionMetrics {
        self.metrics.read().await.clone()
    }

    /// Clean up completed processing entries
    pub async fn cleanup_completed_processing(&self, max_age_hours: u64) {
        let cutoff_time = Utc::now() - chrono::Duration::hours(max_age_hours as i64);
        let mut queue = self.processing_queue.write().await;
        
        queue.retain(|_, processing| {
            processing.received_at > cutoff_time || 
            (processing.status != ProcessingStatus::Completed && processing.status != ProcessingStatus::Failed)
        });
    }
}

impl Default for BundleReceiverConfig {
    fn default() -> Self {
        Self {
            max_concurrent_processing: 10,
            validation_timeout_secs: 30,
            strict_validation: true,
            auction_submission_timeout_secs: 60,
            enable_real_time_updates: true,
        }
    }
}

impl ReceptionMetrics {
    fn new() -> Self {
        Self {
            total_bundles_received: 0,
            total_bundles_processed: 0,
            total_bundles_rejected: 0,
            total_auction_transactions_created: 0,
            average_processing_time_ms: 0.0,
            last_bundle_received: None,
            processing_errors: 0,
            validation_failures: 0,
        }
    }

    fn update_average_processing_time(&mut self, new_time: f64) {
        if self.total_bundles_processed == 0 {
            self.average_processing_time_ms = new_time;
        } else {
            let total_time = self.average_processing_time_ms * (self.total_bundles_processed - 1) as f64 + new_time;
            self.average_processing_time_ms = total_time / self.total_bundles_processed as f64;
        }
    }
}
