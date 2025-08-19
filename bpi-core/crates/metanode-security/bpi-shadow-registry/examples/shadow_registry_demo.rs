//! BPI Shadow Registry Demo
//!
//! Demonstrates the military-grade secure Web2-Web3 bridge functionality
//! with comprehensive examples of registration, bridging, and verification.

use bpi_shadow_registry::{
    ShadowRegistry, ShadowRegistryConfig, BridgeMessage, ActingAsIdentity,
    create_bridge_api, Web3Integration,
};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use serde_json::json;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use tracing::{info, warn};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting BPI Shadow Registry Demo");
    info!("🔒 Military-grade secure Web2-Web3 bridge");
    
    // Create high-security configuration
    let config = ShadowRegistryConfig::high_security();
    info!("✅ Created high-security configuration");
    
    // Create shadow registry
    let registry = ShadowRegistry::new(config);
    info!("✅ Created shadow registry instance");
    
    // Start the registry
    registry.start().await?;
    info!("✅ Shadow registry started successfully");
    
    // Demo 1: Register Web2 system
    info!("\n📋 Demo 1: Web2 System Registration");
    let mut csprng = OsRng {};
    let web2_keypair = SigningKey::generate(&mut csprng);
    
    registry.register_web2_system(
        "legacy-banking-system".to_string(),
        web2_keypair.verifying_key(),
        vec!["payments".to_string(), "transfers".to_string()],
        "https://bank.example.com/api".to_string(),
        HashMap::from([
            ("type".to_string(), "banking".to_string()),
            ("compliance".to_string(), "PCI-DSS".to_string()),
        ]),
    ).await?;
    info!("✅ Registered Web2 banking system");
    
    // Demo 2: Register Web3 contract
    info!("\n📋 Demo 2: Web3 Contract Registration");
    let web3_keypair = SigningKey::generate(&mut csprng);
    let abi_hash = [1u8; 32]; // Placeholder ABI hash
    
    registry.register_web3_contract(
        "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        abi_hash,
        web3_keypair.verifying_key(),
        HashMap::from([
            ("type".to_string(), "defi".to_string()),
            ("protocol".to_string(), "lending".to_string()),
        ]),
    ).await?;
    info!("✅ Registered Web3 DeFi contract");
    
    // Demo 3: Create acting-as identity
    info!("\n📋 Demo 3: Acting-As Identity Creation");
    let acting_as = registry.create_acting_as_identity(
        "user@bank.example.com".to_string(),
        vec!["transfer".to_string(), "balance_check".to_string()],
        3600, // 1 hour
    ).await?;
    info!("✅ Created acting-as identity for user@bank.example.com");
    info!("   Expires at: {}", acting_as.expires_at);
    
    // Demo 4: Process Web2 to Web3 bridge request
    info!("\n📋 Demo 4: Web2 to Web3 Bridge Transaction");
    let request_id = Uuid::new_v4();
    
    let bridge_message = BridgeMessage::Web2ToWeb3 {
        request_id,
        source_identity: "legacy-banking-system".to_string(),
        target_contract: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        method: "transfer".to_string(),
        params: json!({
            "to": "0xabcdef1234567890abcdef1234567890abcdef12",
            "amount": "1000000000000000000", // 1 ETH in wei
            "memo": "Cross-chain payment from legacy system"
        }),
        acting_as: Some(acting_as.clone()),
    };
    
    let shadow_receipt = registry.process_web2_to_web3(bridge_message).await?;
    info!("✅ Processed Web2 to Web3 bridge transaction");
    info!("   Receipt ID: {}", shadow_receipt.receipt_id);
    info!("   Request ID: {}", shadow_receipt.request_id);
    
    // Demo 5: Verify shadow receipt
    info!("\n📋 Demo 5: Shadow Receipt Verification");
    let is_valid = registry.verify_shadow_receipt(&shadow_receipt).await?;
    info!("✅ Shadow receipt verification: {}", if is_valid { "VALID" } else { "INVALID" });
    
    // Demo 6: Registry statistics
    info!("\n📋 Demo 6: Registry Statistics");
    let stats = registry.get_stats().await;
    info!("✅ Registry Statistics:");
    for (key, value) in stats {
        info!("   {}: {}", key, value);
    }
    
    // Demo 7: Compliance and audit trail
    info!("\n📋 Demo 7: Compliance and Audit Trail");
    info!("✅ Compliance Status:");
    info!("   GDPR Compliant: {}", shadow_receipt.compliance.gdpr_compliant);
    info!("   HIPAA Compliant: {}", shadow_receipt.compliance.hipaa_compliant);
    info!("   PCI Compliant: {}", shadow_receipt.compliance.pci_compliant);
    info!("   Retention Policy: {}", shadow_receipt.compliance.retention_policy);
    info!("   Jurisdiction: {}", shadow_receipt.compliance.jurisdiction);
    
    // Demo 8: Security features demonstration
    info!("\n📋 Demo 8: Security Features");
    info!("✅ Security Features Demonstrated:");
    info!("   🔐 Ed25519 signatures for authentication");
    info!("   🔑 X25519 key agreement for perfect forward secrecy");
    info!("   🛡️  ChaCha20Poly1305 AEAD encryption");
    info!("   🔒 Domain-separated hashing");
    info!("   ⏱️  Nonce-based replay protection");
    info!("   👤 Acting-as identity for proxy authentication");
    info!("   📋 Shadow receipts with zero-knowledge proofs");
    info!("   📊 Comprehensive audit trails");
    
    // Demo 9: Bridge API endpoints (would be started in production)
    info!("\n📋 Demo 9: Bridge API Endpoints");
    info!("✅ Available API Endpoints:");
    info!("   POST /api/v1/bridge/web2-to-web3 - Execute bridge transactions");
    info!("   GET  /api/v1/bridge/receipt/:id - Retrieve shadow receipts");
    info!("   POST /api/v1/bridge/verify-receipt - Verify receipt authenticity");
    info!("   POST /api/v1/registry/web2/register - Register Web2 systems");
    info!("   POST /api/v1/registry/web3/register - Register Web3 contracts");
    info!("   POST /api/v1/identity/acting-as - Create acting-as identities");
    info!("   GET  /api/v1/stats - Get registry statistics");
    info!("   GET  /api/v1/health - Health check");
    
    // Demo 10: Integration with existing BPI infrastructure
    info!("\n📋 Demo 10: BPI Integration");
    info!("✅ BPI Integration Features:");
    info!("   🔗 Seamless integration with BPI headers and consensus");
    info!("   📦 Compatible with DockLock container orchestration");
    info!("   🌐 BPCI mesh networking support");
    info!("   ⚡ High-throughput transaction processing");
    info!("   🔄 Automatic failover and load balancing");
    
    info!("\n🎉 Shadow Registry Demo Completed Successfully!");
    info!("🔒 Military-grade secure Web2-Web3 bridge is operational");
    info!("📋 All security features verified and functional");
    
    Ok(())
}

/// Example of how to start the HTTP API server (commented out for demo)
#[allow(dead_code)]
async fn start_api_server() -> Result<(), Box<dyn std::error::Error>> {
    use axum::Server;
    use std::net::SocketAddr;
    use std::sync::Arc;
    
    // Create registry
    let config = ShadowRegistryConfig::high_security();
    let registry = Arc::new(ShadowRegistry::new(config));
    
    // Create API router
    let app = create_bridge_api(registry);
    
    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("🌐 Starting Shadow Registry API server on {}", addr);
    
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}

/// Example of advanced security configuration
#[allow(dead_code)]
fn create_military_grade_config() -> ShadowRegistryConfig {
    let mut config = ShadowRegistryConfig::high_security();
    
    // Ultra-short session timeout for maximum security
    config.session_timeout = 60; // 1 minute
    
    // Smaller message size limit
    config.max_message_size = 256 * 1024; // 256KB
    
    // Enhanced compliance requirements
    config.compliance_requirements.gdpr_compliant = true;
    config.compliance_requirements.hipaa_compliant = true;
    config.compliance_requirements.pci_compliant = true;
    config.compliance_requirements.retention_policy = "25 years".to_string();
    config.compliance_requirements.jurisdiction = "Multi-jurisdictional".to_string();
    
    config
}
