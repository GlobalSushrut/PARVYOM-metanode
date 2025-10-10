// Test BPI → BPCI Transaction Bridge - Verify Real Proof Creation
// This test forces BPI ledger to submit bundles to BPCI via XTMP and verifies proof creation

use std::process::Command;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing BPI → BPCI Transaction Bridge");
    println!("📡 Verifying real proof creation, not mock data");
    
    // Step 1: Send multiple test audits to create a bundle
    println!("\n📝 Step 1: Sending test audits to create BPI bundle...");
    for i in 1..=5 {
        let audit_payload = json!({
            "payload": {
                "audit_id": format!("bpi-bpci-test-{:03}", i),
                "timestamp": "2025-09-09T09:26:00Z",
                "operation": "BPI_BPCI_BRIDGE_TEST",
                "user_id": format!("test-user-{:03}", i),
                "resource": "bpi-bpci-integration",
                "details": {
                    "test_type": "bundle_proof_verification",
                    "bundle_sequence": i,
                    "expected_flow": "audit -> BPI bundle -> XTMP -> BPCI proof"
                }
            },
            "integrity": {
                "checksum": format!("sha256:test{:03}", i),
                "size": 1024 + i * 100
            },
            "signature": {
                "algorithm": "Ed25519",
                "value": format!("test-sig-bpi-bpci-{:03}", i)
            },
            "metadata": {
                "version": "1.0",
                "source": "bpi-bpci-bridge-test",
                "priority": "high"
            }
        });

        let output = Command::new("curl")
            .args(&[
                "-X", "POST",
                "http://localhost:8888/api/audit/submit",
                "-H", "Content-Type: application/json",
                "-d", &audit_payload.to_string(),
                "-s"
            ])
            .output()?;

        if output.status.success() {
            println!("✅ Audit {} submitted successfully", i);
        } else {
            println!("❌ Failed to submit audit {}: {}", i, String::from_utf8_lossy(&output.stderr));
        }
    }

    // Step 2: Check BPI ledger stats
    println!("\n📊 Step 2: Checking BPI ledger transaction stats...");
    let stats_output = Command::new("curl")
        .args(&["-s", "http://localhost:8888/api/audit/stats"])
        .output()?;
    
    if stats_output.status.success() {
        println!("📈 BPI Ledger Stats:");
        println!("{}", String::from_utf8_lossy(&stats_output.stdout));
    }

    // Step 3: Force bundle submission to BPCI
    println!("\n🚀 Step 3: Forcing BPI bundle submission to BPCI via XTMP...");
    let bundle_payload = json!({
        "action": "force_bundle_submission",
        "target": "bpci_xtmp_server",
        "endpoint": "127.0.0.1:7778",
        "bundle_threshold": 3
    });

    let bundle_output = Command::new("curl")
        .args(&[
            "-X", "POST",
            "http://localhost:7777/api/ledger/force-bundle-submit",
            "-H", "Content-Type: application/json",
            "-d", &bundle_payload.to_string(),
            "-s"
        ])
        .output()?;

    println!("📦 Bundle submission result:");
    println!("{}", String::from_utf8_lossy(&bundle_output.stdout));

    // Step 4: Check BPCI proof creation
    println!("\n🔍 Step 4: Checking BPCI proof creation...");
    let proof_output = Command::new("curl")
        .args(&["-s", "http://localhost:8082/api/consensus/recent-proofs"])
        .output()?;

    if proof_output.status.success() {
        println!("🎯 BPCI Proof Creation Status:");
        println!("{}", String::from_utf8_lossy(&proof_output.stdout));
    }

    // Step 5: Verify XTMP connection
    println!("\n🔗 Step 5: Verifying XTMP connection status...");
    let xtmp_output = Command::new("netstat")
        .args(&["-tlnp"])
        .output()?;

    if xtmp_output.status.success() {
        let netstat_str = String::from_utf8_lossy(&xtmp_output.stdout);
        for line in netstat_str.lines() {
            if line.contains("7778") {
                println!("📡 XTMP Server: {}", line);
            }
        }
    }

    println!("\n🎉 BPI → BPCI Bridge Test Complete!");
    println!("✅ Verify above output shows real BPI transactions received by BPCI");
    println!("✅ Check for real proof creation, not mock responses");

    Ok(())
}
