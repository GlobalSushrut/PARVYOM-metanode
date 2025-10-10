//! Pravyom Standard Pipeline v1.0 Integration Module
//! 
//! This module provides integration between existing ziplock-json and VM audit manager
//! systems with the canonical Pravyom Standard Pipeline v1.0 specification.

pub mod action_record_adapter;
pub mod segment_threshold_manager;
pub mod summary_ticket_generator;
pub mod poe_bundle_coordinator;
pub mod bpci_auction_manager;
pub mod pipeline_coordinator;
pub mod bundle_v2_emitter;

pub use action_record_adapter::*;
pub use segment_threshold_manager::*;
pub use summary_ticket_generator::*;
pub use poe_bundle_coordinator::*;
pub use bpci_auction_manager::*;
pub use pipeline_coordinator::*;
pub use bundle_v2_emitter::*;

use anyhow::Result;
// use pravyom_pipeline::*; // Temporarily commented out
use crate::immutable_audit_system::{ImmutableAuditSystem, AuditRecord};
use ziplock_json::*;
use serde::{Serialize, Deserialize};

/// VM Type enumeration (CBOR-compatible)
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum VmType {
    App,
    Court,
    Firewall,
    Orch,
    Cluster,
}

/// Threshold Configuration (CBOR-compatible)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdConfig {
    pub records_per_segment: u64,
    pub segment_max_duration_secs: u64,
    pub poe_per_bpi_bundle: u64,
    pub bpi_bundles_per_bpci: u64,
    pub poe_bundle_max_age_mins: u64,
    pub bpci_auction_max_age_mins: u64,
    pub anomaly_spike_factor: f64,
}

/// Signing Configuration (CBOR-compatible)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigningConfig {
    pub ed25519_enabled: bool,
    pub pqc_algorithm: String,
    pub bls_enabled: bool,
    pub pqc_multi_enabled: bool,
}

/// Main integration facade for Pravyom pipeline compliance
pub struct PravyomIntegration {
    pub action_adapter: ActionRecordAdapter,
    pub threshold_manager: SegmentThresholdManager,
    pub ticket_generator: SummaryTicketGenerator,
    pub poe_coordinator: PoeBundleCoordinator,
    pub auction_manager: BpciAuctionManager,
    pub pipeline_coordinator: crate::pravyom_integration::pipeline_coordinator::PipelineCoordinator,
}

impl PravyomIntegration {
    /// Create new Pravyom integration instance
    pub fn new(config: &PravyomConfig) -> Result<Self> {
        Ok(Self {
            action_adapter: ActionRecordAdapter::new(config)?,
            threshold_manager: SegmentThresholdManager::new(config)?,
            ticket_generator: SummaryTicketGenerator::new(config)?,
            poe_coordinator: PoeBundleCoordinator::new(config.clone()),
            auction_manager: BpciAuctionManager::new(config.clone()),
            pipeline_coordinator: crate::pravyom_integration::pipeline_coordinator::PipelineCoordinator::new(config)?,
        })
    }

    /// Start the integrated pipeline
    pub async fn start(&mut self) -> Result<()> {
        self.pipeline_coordinator.start().await
    }

    /// Process audit record through the pipeline
    pub async fn process_audit_record(&mut self, audit_record: &AuditRecord) -> Result<String> {
        // Convert to canonical action record
        let action_record = self.action_adapter.convert_audit_record(audit_record)?;
        
        // Process through threshold manager
        let segment_result = self.threshold_manager.process_record(&action_record).await?;
        
        // If segment was sealed, generate ticket
        if let Some(segment_meta) = segment_result {
            let ticket = self.ticket_generator.create_summary_ticket(&segment_meta).await?;
            
            // Process ticket for PoE bundling
            let poe_result = self.poe_coordinator.process_ticket(&ticket).await?;
            
            // If PoE bundle was created, process for auction
            if let Some(poe_bundle) = poe_result {
                self.auction_manager.process_poe_bundle(&poe_bundle).await?;
            }
        }
        
        Ok(action_record.rid)
    }
}

/// Configuration for Pravyom integration (CBOR-compatible)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PravyomConfig {
    pub storage_path: String,
    pub vm_type_mapping: std::collections::HashMap<String, VmType>,
    pub thresholds: ThresholdConfig,
    pub signing_config: SigningConfig,
    pub bpi_endpoint: String,
    pub bpci_endpoint: String,
    
    // PoE Bundle Coordinator configuration
    pub min_poe_units_per_bundle: u64,
    pub max_ticket_age_hours: u32,
    pub validator_share_percentage: f64,
    pub treasury_share_percentage: f64,
    pub bundle_signing_key: String,
    
    // BPCI Auction Manager configuration
    pub min_poe_for_auction: u64,
    pub auction_duration_hours: u32,
    pub min_auction_poe_value: u64,
    pub reserve_price_multiplier: f64,
    pub minimum_bid_multiplier: f64,
    pub platform_fee_percentage: f64,
}

impl Default for PravyomConfig {
    fn default() -> Self {
        let mut vm_type_mapping = std::collections::HashMap::new();
        vm_type_mapping.insert("BpiActionVm".to_string(), VmType::App);
        vm_type_mapping.insert("ForensicVm".to_string(), VmType::Court);
        vm_type_mapping.insert("HttpCageVm".to_string(), VmType::Firewall);
        vm_type_mapping.insert("OrchestrationVm".to_string(), VmType::Orch);
        vm_type_mapping.insert("UniversalAuditVm".to_string(), VmType::Cluster);
        
        Self {
            storage_path: "/ziplock".to_string(),
            vm_type_mapping,
            thresholds: ThresholdConfig {
                records_per_segment: 1000,
                segment_max_duration_secs: 60,
                poe_per_bpi_bundle: 100,
                bpi_bundles_per_bpci: 100,
                poe_bundle_max_age_mins: 10,
                bpci_auction_max_age_mins: 60,
                anomaly_spike_factor: 10.0,
            },
            signing_config: SigningConfig {
                ed25519_enabled: true,
                pqc_algorithm: "dilithium2".to_string(),
                bls_enabled: true,
                pqc_multi_enabled: true,
            },
            bpi_endpoint: "http://localhost:9545".to_string(),
            bpci_endpoint: "http://localhost:8080".to_string(),
            
            // PoE Bundle Coordinator defaults
            min_poe_units_per_bundle: 50,
            max_ticket_age_hours: 2,
            validator_share_percentage: 30.0,
            treasury_share_percentage: 20.0,
            bundle_signing_key: "default_bundle_signing_key".to_string(),
            
            // BPCI Auction Manager defaults
            min_poe_for_auction: 100,
            auction_duration_hours: 24,
            min_auction_poe_value: 100,
            reserve_price_multiplier: 1.2,
            minimum_bid_multiplier: 0.8,
            platform_fee_percentage: 2.5,
        }
    }
}
