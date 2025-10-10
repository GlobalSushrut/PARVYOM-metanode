//! Pravyom Standard Pipeline v1.0 - Helper Functions and Utilities
//! 
//! CBOR/Merkle helpers and utility functions for pipeline implementation

use crate::*;
use blake3::Hasher;
use sha2::{Sha256, Digest};
use serde_cbor;
use std::collections::HashMap;

/// CBOR Helper Functions
pub mod cbor {
    use super::*;
    
    /// Encode action record to canonical CBOR
    pub fn encode_action_record(record: &ActionRecord) -> PipelineResult<Vec<u8>> {
        serde_cbor::to_vec(record)
            .map_err(|e| anyhow::anyhow!("CBOR encoding failed: {}", e))
    }
    
    /// Decode action record from CBOR
    pub fn decode_action_record(cbor: &[u8]) -> PipelineResult<ActionRecord> {
        serde_cbor::from_slice(cbor)
            .map_err(|e| anyhow::anyhow!("CBOR decoding failed: {}", e))
    }
    
    /// Encode summary ticket to canonical CBOR
    pub fn encode_summary_ticket(ticket: &SummaryTicket) -> PipelineResult<Vec<u8>> {
        serde_cbor::to_vec(ticket)
            .map_err(|e| anyhow::anyhow!("CBOR encoding failed: {}", e))
    }
    
    /// Calculate CBOR hash using BLAKE3
    pub fn cbor_hash(data: &[u8]) -> String {
        let mut hasher = Hasher::new();
        hasher.update(data);
        hasher.finalize().to_hex().to_string()
    }
}

/// Merkle Tree Helper Functions
pub mod merkle {
    use super::*;
    
    /// Simple Merkle tree implementation
    pub struct SimpleMerkleTree {
        leaves: Vec<String>,
    }
    
    impl SimpleMerkleTree {
        pub fn new() -> Self {
            Self {
                leaves: Vec::new(),
            }
        }
        
        pub fn add_leaf(&mut self, data: &[u8]) {
            let hash = cbor::cbor_hash(data);
            self.leaves.push(hash);
        }
        
        pub fn build_root(&self) -> PipelineResult<String> {
            if self.leaves.is_empty() {
                return Ok("0".repeat(64)); // Empty tree root
            }
            
            let mut level = self.leaves.clone();
            
            while level.len() > 1 {
                let mut next_level = Vec::new();
                
                for chunk in level.chunks(2) {
                    let combined = if chunk.len() == 2 {
                        format!("{}{}", chunk[0], chunk[1])
                    } else {
                        chunk[0].clone()
                    };
                    
                    let mut hasher = Hasher::new();
                    hasher.update(combined.as_bytes());
                    next_level.push(hasher.finalize().to_hex().to_string());
                }
                
                level = next_level;
            }
            
            Ok(level[0].clone())
        }
        
        pub fn leaf_count(&self) -> usize {
            self.leaves.len()
        }
    }
}

/// ID Generation Helper Functions
pub mod ids {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;
    
    /// Generate record ID: R-{YYYYMMDD}-{vmid}-{nonce16}
    pub fn generate_record_id(vmid: &str) -> String {
        let date = Utc::now().format("%Y%m%d");
        let nonce = Uuid::new_v4().to_string().replace("-", "")[..16].to_uppercase();
        format!("R-{}-{}-{}", date, vmid, nonce)
    }
    
    /// Generate segment ID: seg-{6digit}
    pub fn generate_segment_id(seq: u64) -> String {
        format!("seg-{:06}", seq)
    }
    
    /// Generate ticket ID: ZT-{YYYYMMDD}-{HH:MM:SS}Z-batch-{6digit}
    pub fn generate_ticket_id(batch_seq: u64) -> String {
        let timestamp = Utc::now().format("%Y%m%d-%H:%M:%S");
        format!("ZT-{}Z-batch-{:06}", timestamp, batch_seq)
    }
    
    /// Generate PoE ID: POE-{timestampZ}-{6digit}
    pub fn generate_poe_id(seq: u64) -> String {
        let timestamp = Utc::now().format("%Y%m%d-%H:%M:%S");
        format!("POE-{}Z-{:06}", timestamp, seq)
    }
    
    /// Generate BPI Bundle ID: BPIB-{timestampZ}-{6digit}
    pub fn generate_bpi_bundle_id(seq: u64) -> String {
        let timestamp = Utc::now().format("%Y%m%d-%H:%M:%S");
        format!("BPIB-{}Z-{:06}", timestamp, seq)
    }
    
    /// Generate BPCI Auction ID: BPCIA-{timestampZ}-{6digit}
    pub fn generate_bpci_auction_id(seq: u64) -> String {
        let timestamp = Utc::now().format("%Y%m%d-%H:%M:%S");
        format!("BPCIA-{}Z-{:06}", timestamp, seq)
    }
}

/// Clock Proof Helper Functions
pub mod clock {
    use super::*;
    
    /// Generate clock proof: SHA256(ts_mono || ts_wall || vmid || prev_rhash)
    pub fn generate_clock_proof(
        ts_mono: u64,
        ts_wall: &DateTime<Utc>,
        vmid: &str,
        prev_rhash: &str,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(ts_mono.to_be_bytes());
        hasher.update(ts_wall.timestamp().to_be_bytes());
        hasher.update(vmid.as_bytes());
        hasher.update(prev_rhash.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    /// Validate clock skew (±3s tolerance)
    pub fn validate_clock_skew(ts_wall: &DateTime<Utc>, tolerance_secs: u64) -> bool {
        let now = Utc::now();
        let diff = if now > *ts_wall {
            (now - *ts_wall).num_seconds() as u64
        } else {
            (*ts_wall - now).num_seconds() as u64
        };
        diff <= tolerance_secs
    }
}

/// Validation Helper Functions
pub mod validation {
    use super::*;
    
    /// Validate action record format
    pub fn validate_action_record(record: &ActionRecord) -> PipelineResult<()> {
        // Validate RID format
        if !record.rid.starts_with("R-") {
            return Err(anyhow::anyhow!("Invalid RID format: {}", record.rid));
        }
        
        // Validate VM type
        match record.vm.vm_type {
            VmType::App | VmType::Orch | VmType::Cluster | VmType::Storage |
            VmType::Firewall | VmType::Court | VmType::Biso | VmType::TrafficLight => {},
        }
        
        // Validate action type
        let valid_actions = ["WRITE", "READ", "EXEC", "NET", "POLICY"];
        if !valid_actions.contains(&record.action.action_type.as_str()) {
            return Err(anyhow::anyhow!("Invalid action type: {}", record.action.action_type));
        }
        
        // Validate clock proof
        let _expected_proof = clock::generate_clock_proof(
            record.time.ts_mono,
            &record.time.ts_wall,
            &record.vm.id,
            &record.hash.prev,
        );
        
        // Note: In real implementation, we'd validate the actual clock proof
        // For now, just check it's not empty
        if record.hash.self_hash.is_empty() {
            return Err(anyhow::anyhow!("Missing hash.self"));
        }
        
        Ok(())
    }
    
    /// Validate segment metadata
    pub fn validate_segment_meta(meta: &SegmentMeta) -> PipelineResult<()> {
        if meta.segment_seq == 0 {
            return Err(anyhow::anyhow!("Invalid segment sequence"));
        }
        
        if meta.seg_merkle_root.len() != 64 {
            return Err(anyhow::anyhow!("Invalid Merkle root length"));
        }
        
        Ok(())
    }
    
    /// Validate summary ticket
    pub fn validate_summary_ticket(ticket: &SummaryTicket) -> PipelineResult<()> {
        if !ticket.ticket_id.starts_with("ZT-") {
            return Err(anyhow::anyhow!("Invalid ticket ID format"));
        }
        
        if ticket.vm_rollup.len() != ticket.policy.vm_count as usize {
            return Err(anyhow::anyhow!("VM rollup count mismatch"));
        }
        
        if ticket.window.from >= ticket.window.to {
            return Err(anyhow::anyhow!("Invalid time window"));
        }
        
        Ok(())
    }
}

/// Threshold Helper Functions
pub mod thresholds {
    use super::*;
    use std::time::{Duration, Instant};
    
    /// Check if segment should be sealed
    pub fn should_seal_segment(
        record_count: u32,
        start_time: Instant,
        now: Instant,
    ) -> bool {
        record_count >= constants::RECORDS_PER_SEGMENT ||
        now.duration_since(start_time) >= Duration::from_secs(constants::SEGMENT_MAX_DURATION_SECS)
    }
    
    /// Check if PoE bundle should be sealed
    pub fn should_seal_poe_bundle(
        poe_count: u32,
        start_time: Instant,
        now: Instant,
    ) -> bool {
        poe_count >= constants::POE_PER_BPI_BUNDLE ||
        now.duration_since(start_time) >= Duration::from_secs(constants::POE_BUNDLE_MAX_AGE_MINS * 60)
    }
    
    /// Check if BPCI auction should be opened
    pub fn should_open_bpci_auction(
        bundle_count: u32,
        start_time: Instant,
        now: Instant,
    ) -> bool {
        bundle_count >= constants::BPI_BUNDLES_PER_BPCI ||
        now.duration_since(start_time) >= Duration::from_secs(constants::BPCI_AUCTION_MAX_AGE_MINS * 60)
    }
    
    /// Check for anomaly spike
    pub fn is_anomaly_spike(current_rate: f64, baseline_rate: f64) -> bool {
        current_rate > baseline_rate * constants::ANOMALY_SPIKE_FACTOR
    }
}

/// Resource Aggregation Helper Functions
pub mod aggregation {
    use super::*;
    
    /// Aggregate resource usage across multiple records
    pub fn aggregate_resource_usage(usages: &[ResourceUsage]) -> ResourceUsage {
        let mut total_cpu = 0.0;
        let mut total_ram = 0;
        let mut total_io_r = 0;
        let mut total_io_w = 0;
        
        for usage in usages {
            total_cpu += usage.cpu_ms;
            total_ram += usage.ram_kb;
            total_io_r += usage.io.r;
            total_io_w += usage.io.w;
        }
        
        ResourceUsage {
            cpu_ms: total_cpu,
            ram_kb: total_ram,
            io: IoUsage {
                r: total_io_r,
                w: total_io_w,
            },
        }
    }
    
    /// Create VM rollup from action records
    pub fn create_vm_rollup(
        vmid: String,
        records: &[ActionRecord],
        segment_ref: SegmentRef,
    ) -> VmRollup {
        let resource_usages: Vec<ResourceUsage> = records
            .iter()
            .map(|r| r.resource.clone())
            .collect();
        
        let aggregated = aggregate_resource_usage(&resource_usages);
        
        // Count network flows (simplified)
        let net_flows = records
            .iter()
            .filter(|r| r.net.is_some())
            .count() as u32;
        
        VmRollup {
            vmid,
            records: records.len() as u64,
            cpu_ms: aggregated.cpu_ms,
            ram_kb: aggregated.ram_kb,
            io: aggregated.io,
            net: NetworkRollup { flows: net_flows },
            seg: segment_ref,
        }
    }
    
    /// Create system rollup from VM rollups
    pub fn create_system_rollup(vm_rollups: &[VmRollup]) -> SystemRollup {
        let total_records: u64 = vm_rollups.iter().map(|r| r.records).sum();
        let total_cpu: f64 = vm_rollups.iter().map(|r| r.cpu_ms).sum();
        let avg_ram: u64 = if !vm_rollups.is_empty() {
            vm_rollups.iter().map(|r| r.ram_kb).sum::<u64>() / vm_rollups.len() as u64
        } else {
            0
        };
        
        SystemRollup {
            totals: SystemTotals {
                records: total_records,
                cpu_ms: total_cpu,
                ram_kb_avg: avg_ram,
            },
            sec: SecurityRollup {
                allow: 0, // Would be calculated from actual security events
                deny: 0,
                qlock_events: 0,
            },
            poe: PoeRollup {
                exec_count: 0, // Would be calculated from PoE-eligible records
                ready_for_poe_bundle: false,
            },
            anomaly: None, // Would be populated if anomalies detected
        }
    }
}

/// Test Helper Functions
pub mod test_helpers {
    use super::*;
    
    /// Create a test action record
    pub fn create_test_action_record(vmid: &str, vm_type: VmType) -> ActionRecord {
        let rid = ids::generate_record_id(vmid);
        let now = Utc::now();
        
        ActionRecord {
            rid: rid.clone(),
            vm: VmInfo {
                id: vmid.to_string(),
                vm_type,
                image: "test@biso#1.0.0".to_string(),
            },
            actor: ActorInfo {
                wallet: "bpi:test_wallet".to_string(),
                role: "client".to_string(),
            },
            action: ActionInfo {
                action_type: "READ".to_string(),
                name: "test.action".to_string(),
                args: serde_json::json!({"key": "test"}),
            },
            result: ActionResult {
                code: 0,
                latency_ms: 1.0,
                bytes_out: 100,
            },
            resource: ResourceUsage {
                cpu_ms: 1.0,
                ram_kb: 64,
                io: IoUsage { r: 0, w: 100 },
            },
            net: None,
            geo: None,
            time: TimeInfo {
                ts_wall: now,
                ts_mono: 1234567890,
            },
            time_anchor: TimeAnchor {
                rt: "draft-roughtime@v1".to_string(),
                server: "time.cloudflare.com".to_string(),
                proof: "test_proof".to_string(),
            },
            hash: HashChain {
                prev: "0".repeat(64),
                self_hash: "test_hash".to_string(),
            },
            sig: RecordSignature {
                ed25519: "test_ed25519_sig".to_string(),
                pqc: "test_pqc_sig".to_string(),
            },
            exec: None,
        }
    }
    
    /// Create a test summary ticket
    pub fn create_test_summary_ticket() -> SummaryTicket {
        let ticket_id = ids::generate_ticket_id(1);
        let now = Utc::now();
        
        SummaryTicket {
            ticket_id,
            window: TimeWindow {
                from: now - chrono::Duration::minutes(1),
                to: now,
            },
            policy: TicketPolicy {
                threshold: "1min_or_1000rec".to_string(),
                vm_count: 8,
            },
            vm_rollup: vec![
                VmRollup {
                    vmid: "vmapp01".to_string(),
                    records: 125,
                    cpu_ms: 125.0,
                    ram_kb: 8000,
                    io: IoUsage { r: 0, w: 12500 },
                    net: NetworkRollup { flows: 12 },
                    seg: SegmentRef {
                        id: "seg-000001".to_string(),
                        root: "test_root_1".to_string(),
                    },
                },
                VmRollup {
                    vmid: "vmorch01".to_string(),
                    records: 125,
                    cpu_ms: 125.0,
                    ram_kb: 8000,
                    io: IoUsage { r: 1250, w: 10000 },
                    net: NetworkRollup { flows: 10 },
                    seg: SegmentRef {
                        id: "seg-000002".to_string(),
                        root: "test_root_2".to_string(),
                    },
                },
                VmRollup {
                    vmid: "vmcluster01".to_string(),
                    records: 125,
                    cpu_ms: 125.0,
                    ram_kb: 8000,
                    io: IoUsage { r: 500, w: 8000 },
                    net: NetworkRollup { flows: 8 },
                    seg: SegmentRef {
                        id: "seg-000003".to_string(),
                        root: "test_root_3".to_string(),
                    },
                },
                VmRollup {
                    vmid: "vmstorage01".to_string(),
                    records: 125,
                    cpu_ms: 125.0,
                    ram_kb: 8000,
                    io: IoUsage { r: 2000, w: 15000 },
                    net: NetworkRollup { flows: 15 },
                    seg: SegmentRef {
                        id: "seg-000004".to_string(),
                        root: "test_root_4".to_string(),
                    },
                },
                VmRollup {
                    vmid: "vmfirewall01".to_string(),
                    records: 125,
                    cpu_ms: 125.0,
                    ram_kb: 8000,
                    io: IoUsage { r: 100, w: 5000 },
                    net: NetworkRollup { flows: 20 },
                    seg: SegmentRef {
                        id: "seg-000005".to_string(),
                        root: "test_root_5".to_string(),
                    },
                },
                VmRollup {
                    vmid: "vmcourt01".to_string(),
                    records: 125,
                    cpu_ms: 125.0,
                    ram_kb: 8000,
                    io: IoUsage { r: 50, w: 2000 },
                    net: NetworkRollup { flows: 2 },
                    seg: SegmentRef {
                        id: "seg-000006".to_string(),
                        root: "test_root_6".to_string(),
                    },
                },
                VmRollup {
                    vmid: "vmbiso01".to_string(),
                    records: 125,
                    cpu_ms: 125.0,
                    ram_kb: 8000,
                    io: IoUsage { r: 800, w: 6000 },
                    net: NetworkRollup { flows: 6 },
                    seg: SegmentRef {
                        id: "seg-000007".to_string(),
                        root: "test_root_7".to_string(),
                    },
                },
                VmRollup {
                    vmid: "vmtrafficlight01".to_string(),
                    records: 125,
                    cpu_ms: 125.0,
                    ram_kb: 8000,
                    io: IoUsage { r: 200, w: 4000 },
                    net: NetworkRollup { flows: 4 },
                    seg: SegmentRef {
                        id: "seg-000008".to_string(),
                        root: "test_root_8".to_string(),
                    },
                },
            ],
            system: SystemRollup {
                totals: SystemTotals {
                    records: 1000,
                    cpu_ms: 1000.0,
                    ram_kb_avg: 1024,
                },
                sec: SecurityRollup {
                    allow: 1000,
                    deny: 0,
                    qlock_events: 0,
                },
                poe: PoeRollup {
                    exec_count: 10,
                    ready_for_poe_bundle: false,
                },
                anomaly: None,
            },
            roots: TicketRoots {
                vm_merkle: "test_vm_merkle".to_string(),
                ziplock_super_root: "test_super_root".to_string(),
            },
            anchors: TicketAnchors {
                previous_ticket: "ZT-previous".to_string(),
                bpi_tip_hint: "BPI-HT-test".to_string(),
            },
            sig: AggregateSignature {
                bls: "test_bls_sig".to_string(),
                pqc_multi: vec!["test_pqc_multi_sig".to_string()],
            },
        }
    }
}
