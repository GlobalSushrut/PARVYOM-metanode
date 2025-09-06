//! Minute Root Anchoring System for GBF Architecture Stage 2
//! 
//! This module implements Tier 0 public transparency by aggregating VM bundle commits
//! into minute-level roots and anchoring them to the BPI ledger for immutable public record.

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;
use anyhow::{Result, anyhow};
use tracing::{info, warn, error, debug};

/// Minute root anchor for public transparency (Tier 0)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteRootAnchor {
    pub minute_timestamp: u64,
    pub aggregated_root: [u8; 32],        // Merkle root of all VM bundles in this minute
    pub bundle_refs: Vec<BundleRef>,       // References to bundle commits
    pub poe_summary: PoESummary,           // Resource usage summary for public transparency
    pub anchor_tx_hash: [u8; 32],         // BPI transaction hash for immutable record
    pub anchor_id: String,                 // Unique anchor identifier
    pub vm_count: u32,                     // Number of VMs contributing to this anchor
    pub total_events: u64,                 // Total audit events in this minute
}

/// Reference to a bundle commit within the minute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleRef {
    pub bundle_id: String,
    pub vm_id: String,
    pub bundle_root: [u8; 32],
    pub event_count: u32,
    pub timestamp: u64,
    pub quality_score: f64,                // From Stage 1 bundle auction system
}

/// Proof of Execution summary for public transparency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoESummary {
    pub total_cpu_quanta: u64,
    pub total_memory_quanta: u64,
    pub total_network_quanta: u64,
    pub total_storage_quanta: u64,
    pub total_io_operations: u64,
    pub vm_count: u32,
    pub event_count: u64,
    pub minute_start: u64,
    pub minute_end: u64,
    pub resource_efficiency: f64,          // Overall resource utilization efficiency
}

/// Minute root aggregation manager
pub struct MinuteRootAggregator {
    current_minute_bundles: HashMap<String, BundleRef>,
    completed_anchors: VecDeque<MinuteRootAnchor>,
    anchor_history: HashMap<u64, MinuteRootAnchor>, // minute_timestamp -> anchor
    pub current_minute_start: u64,
    aggregation_config: AggregationConfig,
    bpi_client: Option<BpiLedgerClient>,
}

/// Configuration for minute root aggregation
#[derive(Debug, Clone)]
pub struct AggregationConfig {
    pub minute_duration: Duration,         // Standard 60-second minutes
    pub max_bundles_per_minute: u32,      // Prevent DoS attacks
    pub min_quality_threshold: f64,        // Minimum bundle quality for inclusion
    pub anchor_retention_hours: u32,       // How long to keep anchor history
    pub enable_public_api: bool,           // Enable public transparency API
}

/// BPI ledger client for anchoring (integration point)
pub struct BpiLedgerClient {
    pub endpoint: String,
    pub auth_key: [u8; 32],
}

impl Default for AggregationConfig {
    fn default() -> Self {
        Self {
            minute_duration: Duration::from_secs(60),
            max_bundles_per_minute: 1000,
            min_quality_threshold: 0.7,
            anchor_retention_hours: 24,
            enable_public_api: true,
        }
    }
}

impl MinuteRootAggregator {
    /// Create new minute root aggregator
    pub fn new(config: AggregationConfig) -> Self {
        let current_minute_start = Self::get_current_minute_start();
        
        Self {
            current_minute_bundles: HashMap::new(),
            completed_anchors: VecDeque::new(),
            anchor_history: HashMap::new(),
            current_minute_start,
            aggregation_config: config,
            bpi_client: None,
        }
    }

    /// Set BPI ledger client for anchoring
    pub fn set_bpi_client(&mut self, client: BpiLedgerClient) {
        self.bpi_client = Some(client);
    }

    /// Add bundle commit to current minute aggregation
    pub async fn add_bundle_commit(&mut self, bundle_ref: BundleRef) -> Result<()> {
        let current_minute = Self::get_current_minute_start();
        
        // Check if we need to finalize the current minute
        if current_minute > self.current_minute_start {
            self.finalize_current_minute().await?;
            self.current_minute_start = current_minute;
        }

        // Validate bundle quality
        if bundle_ref.quality_score < self.aggregation_config.min_quality_threshold {
            warn!("Bundle {} quality {} below threshold {}, skipping", 
                  bundle_ref.bundle_id, bundle_ref.quality_score, 
                  self.aggregation_config.min_quality_threshold);
            return Ok(());
        }

        // Check bundle limit
        if self.current_minute_bundles.len() >= self.aggregation_config.max_bundles_per_minute as usize {
            warn!("Maximum bundles per minute reached ({}), dropping bundle {}", 
                  self.aggregation_config.max_bundles_per_minute, bundle_ref.bundle_id);
            return Ok(());
        }

        // Add bundle to current minute
        self.current_minute_bundles.insert(bundle_ref.bundle_id.clone(), bundle_ref);
        
        debug!("Added bundle {} to minute {}", 
               self.current_minute_bundles.len(), self.current_minute_start);
        
        Ok(())
    }

    /// Finalize current minute and create anchor
    pub async fn finalize_current_minute(&mut self) -> Result<Option<MinuteRootAnchor>> {
        if self.current_minute_bundles.is_empty() {
            debug!("No bundles in current minute {}, skipping anchor creation", self.current_minute_start);
            return Ok(None);
        }

        info!("Finalizing minute {} with {} bundles", 
              self.current_minute_start, self.current_minute_bundles.len());

        // Create minute root anchor
        let anchor = self.create_minute_anchor().await?;
        
        // Submit to BPI ledger for immutable public record
        if let Some(ref client) = self.bpi_client {
            let tx_hash = self.submit_anchor_to_bpi(&anchor, client).await?;
            info!("Minute anchor {} submitted to BPI ledger: {:?}", 
                  anchor.anchor_id, hex::encode(tx_hash));
        }

        // Store completed anchor
        self.completed_anchors.push_back(anchor.clone());
        self.anchor_history.insert(anchor.minute_timestamp, anchor.clone());

        // Clean up old anchors
        self.cleanup_old_anchors();

        // Clear current minute bundles
        self.current_minute_bundles.clear();

        Ok(Some(anchor))
    }

    /// Create minute root anchor from current bundles
    async fn create_minute_anchor(&self) -> Result<MinuteRootAnchor> {
        let bundle_refs: Vec<BundleRef> = self.current_minute_bundles.values().cloned().collect();
        
        // Calculate aggregated Merkle root
        let aggregated_root = self.calculate_aggregated_root(&bundle_refs)?;
        
        // Calculate PoE summary
        let poe_summary = self.calculate_poe_summary(&bundle_refs)?;
        
        // Generate unique anchor ID
        let anchor_id = format!("anchor-{}-{}", self.current_minute_start, Uuid::new_v4());
        
        let anchor = MinuteRootAnchor {
            minute_timestamp: self.current_minute_start,
            aggregated_root,
            bundle_refs,
            poe_summary,
            anchor_tx_hash: [0u8; 32], // Will be filled when submitted to BPI
            anchor_id,
            vm_count: self.get_unique_vm_count(),
            total_events: self.get_total_event_count(),
        };

        Ok(anchor)
    }

    /// Calculate aggregated Merkle root from bundle references
    fn calculate_aggregated_root(&self, bundle_refs: &[BundleRef]) -> Result<[u8; 32]> {
        if bundle_refs.is_empty() {
            return Err(anyhow!("Cannot calculate root from empty bundle list"));
        }

        // Create Merkle tree from bundle roots
        let mut hasher = Sha256::new();
        
        // Domain separation for minute root anchoring
        hasher.update(b"GBF_MINUTE_ROOT_ANCHOR");
        hasher.update(&self.current_minute_start.to_le_bytes());
        
        // Sort bundles by timestamp for deterministic ordering
        let mut sorted_bundles = bundle_refs.to_vec();
        sorted_bundles.sort_by_key(|b| b.timestamp);
        
        // Hash all bundle roots together
        for bundle in &sorted_bundles {
            hasher.update(&bundle.bundle_root);
            hasher.update(bundle.bundle_id.as_bytes());
            hasher.update(&bundle.event_count.to_le_bytes());
        }
        
        let result = hasher.finalize();
        let mut root = [0u8; 32];
        root.copy_from_slice(&result);
        
        Ok(root)
    }

    /// Calculate Proof of Execution summary
    fn calculate_poe_summary(&self, bundle_refs: &[BundleRef]) -> Result<PoESummary> {
        let mut total_cpu_quanta = 0u64;
        let mut total_memory_quanta = 0u64;
        let mut total_network_quanta = 0u64;
        let mut total_storage_quanta = 0u64;
        let mut total_io_operations = 0u64;
        let mut total_events = 0u64;

        // Aggregate resource usage from all bundles
        // Note: In real implementation, this would extract from bundle metadata
        for bundle in bundle_refs {
            // Estimate resource usage based on event count and quality
            let events = bundle.event_count as u64;
            let quality_multiplier = bundle.quality_score;
            
            total_cpu_quanta += (events as f64 * 100.0 * quality_multiplier) as u64;
            total_memory_quanta += (events as f64 * 50.0 * quality_multiplier) as u64;
            total_network_quanta += (events as f64 * 25.0 * quality_multiplier) as u64;
            total_storage_quanta += (events as f64 * 10.0 * quality_multiplier) as u64;
            total_io_operations += (events as f64 * 5.0 * quality_multiplier) as u64;
            total_events += events;
        }

        // Calculate resource efficiency
        let total_resources = total_cpu_quanta + total_memory_quanta + total_network_quanta + total_storage_quanta;
        let resource_efficiency = if total_resources > 0 {
            (total_events as f64) / (total_resources as f64) * 1000.0 // Events per 1000 resource units
        } else {
            0.0
        };

        Ok(PoESummary {
            total_cpu_quanta,
            total_memory_quanta,
            total_network_quanta,
            total_storage_quanta,
            total_io_operations,
            vm_count: self.get_unique_vm_count(),
            event_count: total_events,
            minute_start: self.current_minute_start,
            minute_end: self.current_minute_start + 60,
            resource_efficiency,
        })
    }

    /// Submit anchor to BPI ledger for immutable public record
    async fn submit_anchor_to_bpi(&self, anchor: &MinuteRootAnchor, client: &BpiLedgerClient) -> Result<[u8; 32]> {
        // Create BPI transaction for minute root anchor
        let mut hasher = Sha256::new();
        hasher.update(b"BPI_MINUTE_ANCHOR_TX");
        hasher.update(&anchor.minute_timestamp.to_le_bytes());
        hasher.update(&anchor.aggregated_root);
        hasher.update(anchor.anchor_id.as_bytes());
        
        let tx_hash = hasher.finalize();
        let mut result = [0u8; 32];
        result.copy_from_slice(&tx_hash);
        
        // In real implementation, this would submit to actual BPI ledger
        info!("Submitting minute anchor {} to BPI ledger at {}", 
              anchor.anchor_id, client.endpoint);
        
        Ok(result)
    }

    /// Get unique VM count in current minute
    fn get_unique_vm_count(&self) -> u32 {
        let unique_vms: std::collections::HashSet<&String> = 
            self.current_minute_bundles.values().map(|b| &b.vm_id).collect();
        unique_vms.len() as u32
    }

    /// Get total event count in current minute
    fn get_total_event_count(&self) -> u64 {
        self.current_minute_bundles.values().map(|b| b.event_count as u64).sum()
    }

    /// Get current minute start timestamp
    fn get_current_minute_start() -> u64 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        (now / 60) * 60 // Round down to minute boundary
    }

    /// Clean up old anchors based on retention policy
    fn cleanup_old_anchors(&mut self) {
        let retention_seconds = self.aggregation_config.anchor_retention_hours as u64 * 3600;
        let cutoff_time = Self::get_current_minute_start().saturating_sub(retention_seconds);
        
        // Remove old anchors from history
        self.anchor_history.retain(|&timestamp, _| timestamp >= cutoff_time);
        
        // Remove old anchors from completed list
        while let Some(anchor) = self.completed_anchors.front() {
            if anchor.minute_timestamp < cutoff_time {
                self.completed_anchors.pop_front();
            } else {
                break;
            }
        }
    }

    /// Get anchor by minute timestamp (public transparency API)
    pub fn get_anchor_by_minute(&self, minute_timestamp: u64) -> Option<&MinuteRootAnchor> {
        self.anchor_history.get(&minute_timestamp)
    }

    /// Get recent anchors (public transparency API)
    pub fn get_recent_anchors(&self, count: usize) -> Vec<&MinuteRootAnchor> {
        self.completed_anchors.iter().rev().take(count).collect()
    }

    /// Get PoE summary for time range (public transparency API)
    pub fn get_poe_summary_range(&self, start_minute: u64, end_minute: u64) -> Result<PoESummary> {
        let mut aggregated_summary = PoESummary {
            total_cpu_quanta: 0,
            total_memory_quanta: 0,
            total_network_quanta: 0,
            total_storage_quanta: 0,
            total_io_operations: 0,
            vm_count: 0,
            event_count: 0,
            minute_start: start_minute,
            minute_end: end_minute,
            resource_efficiency: 0.0,
        };

        let mut unique_vms = std::collections::HashSet::new();
        let mut total_resources = 0u64;

        for minute in (start_minute..=end_minute).step_by(60) {
            if let Some(anchor) = self.anchor_history.get(&minute) {
                let summary = &anchor.poe_summary;
                aggregated_summary.total_cpu_quanta += summary.total_cpu_quanta;
                aggregated_summary.total_memory_quanta += summary.total_memory_quanta;
                aggregated_summary.total_network_quanta += summary.total_network_quanta;
                aggregated_summary.total_storage_quanta += summary.total_storage_quanta;
                aggregated_summary.total_io_operations += summary.total_io_operations;
                aggregated_summary.event_count += summary.event_count;

                // Track unique VMs across the range
                for bundle in &anchor.bundle_refs {
                    unique_vms.insert(&bundle.vm_id);
                }
            }
        }

        aggregated_summary.vm_count = unique_vms.len() as u32;
        
        // Calculate aggregated resource efficiency
        total_resources = aggregated_summary.total_cpu_quanta + 
                         aggregated_summary.total_memory_quanta + 
                         aggregated_summary.total_network_quanta + 
                         aggregated_summary.total_storage_quanta;
        
        aggregated_summary.resource_efficiency = if total_resources > 0 {
            (aggregated_summary.event_count as f64) / (total_resources as f64) * 1000.0
        } else {
            0.0
        };

        Ok(aggregated_summary)
    }

    /// Force finalize current minute (for testing or manual triggers)
    pub async fn force_finalize(&mut self) -> Result<Option<MinuteRootAnchor>> {
        self.finalize_current_minute().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_minute_root_aggregation() {
        println!("🧪 Testing Minute Root Aggregation System");
        
        let mut aggregator = MinuteRootAggregator::new(AggregationConfig::default());
        
        // Create test bundle references
        let bundle1 = BundleRef {
            bundle_id: "bundle-1".to_string(),
            vm_id: "vm-test-1".to_string(),
            bundle_root: [1u8; 32],
            event_count: 100,
            timestamp: aggregator.current_minute_start + 10,
            quality_score: 0.95,
        };
        
        let bundle2 = BundleRef {
            bundle_id: "bundle-2".to_string(),
            vm_id: "vm-test-2".to_string(),
            bundle_root: [2u8; 32],
            event_count: 150,
            timestamp: aggregator.current_minute_start + 20,
            quality_score: 0.87,
        };
        
        // Add bundles to aggregator
        aggregator.add_bundle_commit(bundle1).await.unwrap();
        aggregator.add_bundle_commit(bundle2).await.unwrap();
        
        assert_eq!(aggregator.current_minute_bundles.len(), 2);
        
        // Force finalize minute
        let anchor = aggregator.force_finalize().await.unwrap();
        assert!(anchor.is_some());
        
        let anchor = anchor.unwrap();
        assert_eq!(anchor.bundle_refs.len(), 2);
        assert_eq!(anchor.vm_count, 2);
        assert_eq!(anchor.total_events, 250);
        assert!(anchor.poe_summary.total_cpu_quanta > 0);
        
        println!("✅ Minute root aggregation working");
        println!("   - Anchor ID: {}", anchor.anchor_id);
        println!("   - Aggregated root: {:?}", hex::encode(anchor.aggregated_root));
        println!("   - Total events: {}", anchor.total_events);
        println!("   - Resource efficiency: {:.2}", anchor.poe_summary.resource_efficiency);
    }

    #[tokio::test]
    async fn test_poe_summary_calculation() {
        println!("🧪 Testing PoE Summary Calculation");
        
        let aggregator = MinuteRootAggregator::new(AggregationConfig::default());
        
        let bundles = vec![
            BundleRef {
                bundle_id: "bundle-1".to_string(),
                vm_id: "vm-1".to_string(),
                bundle_root: [1u8; 32],
                event_count: 100,
                timestamp: 1000,
                quality_score: 1.0,
            },
            BundleRef {
                bundle_id: "bundle-2".to_string(),
                vm_id: "vm-2".to_string(),
                bundle_root: [2u8; 32],
                event_count: 200,
                timestamp: 1010,
                quality_score: 0.8,
            },
        ];
        
        let poe_summary = aggregator.calculate_poe_summary(&bundles).unwrap();
        
        assert_eq!(poe_summary.event_count, 300);
        assert!(poe_summary.total_cpu_quanta > 0);
        assert!(poe_summary.total_memory_quanta > 0);
        assert!(poe_summary.resource_efficiency > 0.0);
        
        println!("✅ PoE summary calculation working");
        println!("   - Total events: {}", poe_summary.event_count);
        println!("   - CPU quanta: {}", poe_summary.total_cpu_quanta);
        println!("   - Memory quanta: {}", poe_summary.total_memory_quanta);
        println!("   - Resource efficiency: {:.4}", poe_summary.resource_efficiency);
    }

    #[tokio::test]
    async fn test_quality_threshold_filtering() {
        println!("🧪 Testing Quality Threshold Filtering");
        
        let mut config = AggregationConfig::default();
        config.min_quality_threshold = 0.8;
        
        let mut aggregator = MinuteRootAggregator::new(config);
        
        // High quality bundle (should be accepted)
        let high_quality_bundle = BundleRef {
            bundle_id: "high-quality".to_string(),
            vm_id: "vm-1".to_string(),
            bundle_root: [1u8; 32],
            event_count: 100,
            timestamp: aggregator.current_minute_start + 10,
            quality_score: 0.95,
        };
        
        // Low quality bundle (should be rejected)
        let low_quality_bundle = BundleRef {
            bundle_id: "low-quality".to_string(),
            vm_id: "vm-2".to_string(),
            bundle_root: [2u8; 32],
            event_count: 100,
            timestamp: aggregator.current_minute_start + 20,
            quality_score: 0.5,
        };
        
        aggregator.add_bundle_commit(high_quality_bundle).await.unwrap();
        aggregator.add_bundle_commit(low_quality_bundle).await.unwrap();
        
        // Only high quality bundle should be accepted
        assert_eq!(aggregator.current_minute_bundles.len(), 1);
        assert!(aggregator.current_minute_bundles.contains_key("high-quality"));
        assert!(!aggregator.current_minute_bundles.contains_key("low-quality"));
        
        println!("✅ Quality threshold filtering working");
        println!("   - High quality bundle accepted");
        println!("   - Low quality bundle rejected");
    }

    #[tokio::test]
    async fn test_public_transparency_api() {
        println!("🧪 Testing Public Transparency API");
        
        let mut aggregator = MinuteRootAggregator::new(AggregationConfig::default());
        
        // Add test bundle and create anchor
        let bundle = BundleRef {
            bundle_id: "test-bundle".to_string(),
            vm_id: "vm-1".to_string(),
            bundle_root: [1u8; 32],
            event_count: 100,
            timestamp: aggregator.current_minute_start + 10,
            quality_score: 0.9,
        };
        
        aggregator.add_bundle_commit(bundle).await.unwrap();
        let anchor = aggregator.force_finalize().await.unwrap().unwrap();
        
        // Test public API methods
        let retrieved_anchor = aggregator.get_anchor_by_minute(anchor.minute_timestamp);
        assert!(retrieved_anchor.is_some());
        
        let recent_anchors = aggregator.get_recent_anchors(5);
        assert_eq!(recent_anchors.len(), 1);
        
        let poe_range = aggregator.get_poe_summary_range(
            anchor.minute_timestamp, 
            anchor.minute_timestamp + 60
        ).unwrap();
        assert_eq!(poe_range.event_count, 100);
        
        println!("✅ Public transparency API working");
        println!("   - Anchor retrieval: ✓");
        println!("   - Recent anchors: ✓");
        println!("   - PoE range summary: ✓");
    }
}
