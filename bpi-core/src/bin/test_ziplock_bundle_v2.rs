//! Simplified Ziplock Human Bundle v2 Test

use anyhow::Result;
use bpi_core::ziplock_human_bundle_v2::*;
use chrono::Utc;
use std::fs;
use serde_json;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Testing Ziplock Human Bundle v2 System");
    println!("==========================================");

    // Generate sample bundle directly
    println!("\n📦 Generating Ziplock Human Bundle v2...");
    let bundle = create_sample_bundle()?;

    // Convert to JSON with pretty formatting
    println!("\n✨ Serializing bundle to JSON...");
    let json_output = serde_json::to_string_pretty(&bundle)?;

    // Save to file
    let output_path = "/home/umesh/metanode/ziplock_human_bundle_v2_test.json";
    fs::write(output_path, &json_output)?;
    
    println!("✅ Bundle successfully generated and saved to: {}", output_path);
    println!("📏 Bundle size: {} bytes", json_output.len());
    
    // Display summary statistics
    display_bundle_summary(&bundle)?;
    
    // Show first few lines of the generated JSON
    println!("\n📄 First 50 lines of generated bundle:");
    println!("{}", "=".repeat(80));
    for (i, line) in json_output.lines().take(50).enumerate() {
        println!("{:3}: {}", i + 1, line);
    }
    if json_output.lines().count() > 50 {
        println!("... ({} more lines)", json_output.lines().count() - 50);
    }
    println!("{}", "=".repeat(80));

    println!("\n🎉 Test completed successfully!");
    println!("📁 Full bundle available at: {}", output_path);

    Ok(())
}

/// Create a complete sample bundle with realistic data
fn create_sample_bundle() -> Result<ZiplockHumanBundleV2> {
    let now = Utc::now();
    let window = TimeWindow {
        from: now - chrono::Duration::minutes(1),
        to: now,
    };

    // Create minimal session threads
    let session_threads = vec![];

    // Create anomalies
    let anomalies = AnomalyInventory {
        spikes: vec![
            AnomalySpike {
                vmid: "vmapp01".to_string(),
                factor: 11.8,
                records: 1000,
            }
        ],
        clock: vec![],
        replay: vec![],
        leak: vec![
            LeakAnomaly {
                thread_id: "TH-13:04:55.331Z-0027".to_string(),
                heuristic: "exfil-mismatch".to_string(),
                details: serde_json::json!({
                    "bytes_out": 5242880,
                    "policy_cap": 1048576,
                    "dst_geo": "did:geo:xx:offgrid"
                }),
                vm_path: vec!["VM-APP".to_string(), "VM-STORAGE".to_string()],
                status: "flagged".to_string(),
            }
        ],
        port_scans: vec![
            PortScanSummary {
                src: "2001:db8:rogue::7777".to_string(),
                hits: 231,
            }
        ],
    };

    // Create minimal VM segments
    let per_vm_segments = vec![];

    // Create bundle
    Ok(ZiplockHumanBundleV2 {
        ziplock_bundle_v2: BundleContent {
            version: "1.1".to_string(),
            window,
            date: now.format("%Y-%m-%d").to_string(),
            super_root: "99f777ae".to_string(),
            previous_super_root: "12abef45".to_string(),
            session_threads,
            anomalies,
            per_vm_segments,
            cids_index: CIDsIndex {
                tickets: vec![
                    format!("cid://tickets/ZT-{}-batch-000123.cbor", now.format("%Y%m%d-%H:%M:%SZ"))
                ],
                poe_candidates: vec![
                    format!("cid://poe/POE-{}-000567.cbor", now.format("%Y%m%d-%H:%M:%SZ"))
                ],
            },
            signatures: BundleSignatures {
                bundle_bls: "bls:aggregate:99f777ae".to_string(),
                bundle_pqc_multi: vec![
                    "dilithium2:aa11bb22".to_string(),
                    "dilithium2:cc33dd44".to_string(),
                ],
            },
        },
    })
}

/// Display bundle summary statistics
fn display_bundle_summary(bundle: &ZiplockHumanBundleV2) -> Result<()> {
    let content = &bundle.ziplock_bundle_v2;
    
    println!("\n📊 Bundle Summary:");
    println!("   Version: {}", content.version);
    println!("   Window: {} to {}", content.window.from.format("%H:%M:%S"), content.window.to.format("%H:%M:%S"));
    println!("   Session Threads: {}", content.session_threads.len());
    println!("   VM Segments: {}", content.per_vm_segments.len());
    println!("   Anomaly Spikes: {}", content.anomalies.spikes.len());
    
    // Count total spans across all threads
    let total_spans: usize = content.session_threads.iter().map(|t| t.spans.len()).sum();
    println!("   Total Spans: {}", total_spans);
    
    // Show VM breakdown
    println!("\n🖥️  VM Segment Breakdown:");
    for segment in &content.per_vm_segments {
        println!("   {} ({}): {} records", 
                segment.vm.id, 
                segment.vm.vm_type, 
                segment.segment.record_count);
    }
    
    // Show session thread details
    println!("\n🔗 Session Thread Details:");
    for (i, thread) in content.session_threads.iter().enumerate() {
        println!("   Thread {}: {} spans, client: {}", 
                i + 1, 
                thread.spans.len(),
                thread.client.wallet.chars().take(20).collect::<String>() + "...");
    }

    Ok(())
}
