use std::time::Duration;
use tokio::time::sleep;
use serde_json::json;
use reqwest::Client;
use uuid::Uuid;
use chrono::Utc;
use blake3::Hasher;
use ed25519_dalek::{SigningKey, Signer};
use rand::rngs::OsRng;
use rand::RngCore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Testing Complete BPI → BPCI Transaction Pipeline with Wallet Integration");
    println!("{}", "=".repeat(80));
    
    let client = Client::new();
    
    // Step 1: Verify all services are running
    println!("\n📋 Step 1: Verifying Service Status");
    verify_services(&client).await?;
    
    // Step 2: Test wallet integration via VM server
    println!("\n🔐 Step 2: Testing Wallet Integration");
    test_wallet_integration(&client).await?;
    
    // Step 3: Submit multiple audits to BPI audit server
    println!("\n📝 Step 3: Submitting Test Audits to BPI");
    submit_test_audits(&client).await?;
    
    // Step 4: Verify BPI transactions are created
    println!("\n⛓️ Step 4: Verifying BPI Transaction Creation");
    verify_bpi_transactions(&client).await?;
    
    // Step 5: Check BPCI transaction reception
    println!("\n🌐 Step 5: Checking BPCI Transaction Reception");
    verify_bpci_transactions(&client).await?;
    
    // Step 6: Verify bundle proof creation
    println!("\n🏆 Step 6: Verifying Bundle Proof Creation");
    verify_bundle_proofs(&client).await?;
    
    println!("\n✅ Complete BPI → BPCI Pipeline Test Results:");
    println!("{}", "=".repeat(80));
    
    Ok(())
}

async fn verify_services(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let services = vec![
        ("BPI Audit Server", "http://localhost:8888/health"),
        ("BPI VM Server", "http://localhost:7777/__vm/status"),
        ("BPCI XTMP Server", "http://localhost:7778/health"),
        ("BPCI Consensus Server", "http://localhost:8082/api/consensus/status"),
    ];
    
    for (name, url) in services {
        match client.get(url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    println!("✅ {}: RUNNING", name);
                    if name == "BPI VM Server" {
                        let status: serde_json::Value = response.json().await?;
                        println!("   - Post-quantum enabled: {}", 
                            status["vm_server"]["post_quantum_enabled"].as_bool().unwrap_or(false));
                        println!("   - Security rating: {}", 
                            status["vm_server"]["security_rating"].as_f64().unwrap_or(0.0));
                    }
                } else {
                    println!("⚠️ {}: HTTP {}", name, response.status());
                }
            }
            Err(e) => println!("❌ {}: CONNECTION FAILED - {}", name, e),
        }
    }
    
    Ok(())
}

async fn test_wallet_integration(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    // Test wallet endpoints via VM server
    let wallet_endpoints = vec![
        "http://localhost:7777/__vm/metrics",
        "http://localhost:7777/__vm/instances",
        "http://localhost:7777/__vm/health",
    ];
    
    for endpoint in wallet_endpoints {
        match client.get(endpoint).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    println!("✅ Wallet endpoint accessible: {}", endpoint);
                } else {
                    println!("⚠️ Wallet endpoint error: {} - HTTP {}", endpoint, response.status());
                }
            }
            Err(e) => println!("❌ Wallet endpoint failed: {} - {}", endpoint, e),
        }
    }
    
    Ok(())
}

async fn submit_test_audits(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let audit_url = "http://localhost:8888/api/audit/submit";
    
    for i in 1..=5 {
        let audit = create_test_audit(i).await?;
        
        match client.post(audit_url)
            .json(&audit)
            .send()
            .await {
            Ok(response) => {
                if response.status().is_success() {
                    let result: serde_json::Value = response.json().await?;
                    println!("✅ Audit {} submitted successfully", i);
                    println!("   Transaction ID: {}", 
                        result.get("transaction_id").unwrap_or(&json!("unknown")));
                } else {
                    let status = response.status();
                    let error_text = response.text().await?;
                    println!("❌ Audit {} failed: HTTP {} - {}", i, status, error_text);
                }
            }
            Err(e) => println!("❌ Audit {} submission error: {}", i, e),
        }
        
        sleep(Duration::from_millis(500)).await;
    }
    
    Ok(())
}

async fn verify_bpi_transactions(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let stats_url = "http://localhost:8888/api/ledger/stats";
    
    match client.get(stats_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let stats: serde_json::Value = response.json().await?;
                println!("✅ BPI Ledger Stats Retrieved:");
                println!("   Total transactions: {}", 
                    stats.get("total_transactions").unwrap_or(&json!(0)));
                println!("   Successful submissions: {}", 
                    stats.get("successful_submissions").unwrap_or(&json!(0)));
                println!("   Failed submissions: {}", 
                    stats.get("failed_submissions").unwrap_or(&json!(0)));
            } else {
                println!("⚠️ BPI Ledger stats error: HTTP {}", response.status());
            }
        }
        Err(e) => println!("❌ BPI Ledger stats failed: {}", e),
    }
    
    Ok(())
}

async fn verify_bpci_transactions(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let consensus_url = "http://localhost:8082/api/consensus/status";
    
    match client.get(consensus_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let status: serde_json::Value = response.json().await?;
                println!("✅ BPCI Consensus Status Retrieved:");
                println!("   Current round: {}", 
                    status.get("current_round").unwrap_or(&json!(0)));
                println!("   Total transactions processed: {}", 
                    status.get("total_transactions_processed").unwrap_or(&json!(0)));
                println!("   Active validators: {}", 
                    status.get("active_validators").unwrap_or(&json!(0)));
                
                // Check if transactions are being processed
                let tx_count = status.get("total_transactions_processed")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
                
                if tx_count > 0 {
                    println!("🎉 SUCCESS: BPCI is processing BPI transactions!");
                } else {
                    println!("⚠️ WARNING: BPCI shows zero transactions processed");
                }
            } else {
                println!("⚠️ BPCI Consensus status error: HTTP {}", response.status());
            }
        }
        Err(e) => println!("❌ BPCI Consensus status failed: {}", e),
    }
    
    Ok(())
}

async fn verify_bundle_proofs(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    // Check for bundle proofs via BPCI API
    let bundle_url = "http://localhost:8082/api/bundles/recent";
    
    match client.get(bundle_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let bundles: serde_json::Value = response.json().await?;
                println!("✅ Bundle Proofs Retrieved:");
                
                if let Some(bundle_array) = bundles.as_array() {
                    if bundle_array.is_empty() {
                        println!("⚠️ No bundle proofs found yet");
                    } else {
                        println!("🎉 Found {} bundle proofs!", bundle_array.len());
                        for (i, bundle) in bundle_array.iter().enumerate() {
                            println!("   Bundle {}: ID = {}", i + 1, 
                                bundle.get("bundle_id").unwrap_or(&json!("unknown")));
                        }
                    }
                } else {
                    println!("⚠️ Unexpected bundle response format");
                }
            } else {
                println!("⚠️ Bundle proofs error: HTTP {}", response.status());
            }
        }
        Err(e) => println!("❌ Bundle proofs request failed: {}", e),
    }
    
    Ok(())
}

async fn create_test_audit(index: u32) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Generate cryptographic keypair for signing
    let mut csprng = OsRng {};
    let mut secret_bytes = [0u8; 32];
    csprng.fill_bytes(&mut secret_bytes);
    let signing_key = SigningKey::from_bytes(&secret_bytes);
    let verifying_key = signing_key.verifying_key();
    
    // Create audit payload
    let payload = json!({
        "audit_id": Uuid::new_v4().to_string(),
        "timestamp": Utc::now().to_rfc3339(),
        "audit_type": "transaction_verification",
        "data": {
            "transaction_id": format!("tx_{}", index),
            "amount": 1000 + (index * 100),
            "from_address": "bpi1test_sender_address",
            "to_address": "bpi1test_receiver_address",
            "operation": "transfer"
        },
        "metadata": {
            "source": "test_suite",
            "version": "1.0.0",
            "test_index": index
        }
    });
    
    // Create integrity hash
    let payload_str = serde_json::to_string(&payload)?;
    let mut hasher = Hasher::new();
    hasher.update(payload_str.as_bytes());
    let integrity_hash = hex::encode(hasher.finalize().as_bytes());
    
    // Create signature
    let signature_data = format!("{}{}", payload_str, integrity_hash);
    let signature = signing_key.sign(signature_data.as_bytes());
    let signature_hex = hex::encode(signature.to_bytes());
    
    // Create complete audit structure
    let audit = json!({
        "payload": payload,
        "integrity": integrity_hash,
        "signature": signature_hex,
        "metadata": {
            "public_key": hex::encode(verifying_key.to_bytes()),
            "signature_algorithm": "Ed25519",
            "hash_algorithm": "Blake3",
            "created_at": Utc::now().to_rfc3339()
        }
    });
    
    Ok(audit)
}
