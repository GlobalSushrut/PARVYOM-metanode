//! ZKLock Mobile Demo - Demonstrates the complete zklock ICO system
//!
//! This demo shows how mobile and IoT devices can participate in the BPI ecosystem
//! through the zklock ICO system with optimized zero-knowledge proofs.

use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn};
use uuid::Uuid;

use zklock_mobile_port::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("🚀 Starting ZKLock Mobile Port Demo");
    info!("This demo showcases the revolutionary zklock ICO system for IoT and mobile devices");

    // Create zklock system with default configuration
    let config = ZKLockConfig::default();
    let zklock = ZKLockMobilePort::new(config).await?;

    // Start the system
    zklock.start().await?;
    info!("✅ ZKLock Mobile Port system started successfully");

    // Demo 1: Register different types of devices
    info!("\n📱 Demo 1: Device Registration");
    
    // Register a high-end mobile device
    let mobile_device = DeviceType::Mobile {
        platform: MobilePlatform::Android,
        capabilities: MobileCapabilities {
            ram_mb: 8192,
            storage_gb: 128,
            has_secure_enclave: true,
            supports_biometrics: true,
            network_types: vec![NetworkType::FiveG, NetworkType::WiFi],
        },
    };
    let mobile_id = zklock.register_device(mobile_device, "mobile_wallet_0x123".to_string()).await?;
    info!("📱 Registered high-end mobile device: {}", mobile_id);

    // Register an IoT sensor device
    let iot_device = DeviceType::IoT {
        device_class: IoTClass::Sensor,
        compute_level: ComputeLevel::Light,
    };
    let iot_id = zklock.register_device(iot_device, "iot_wallet_0x456".to_string()).await?;
    info!("🔧 Registered IoT sensor device: {}", iot_id);

    // Register an edge computing device
    let edge_device = DeviceType::Edge {
        processing_power: ProcessingPower::High,
        connectivity: ConnectivityType::WiFi,
    };
    let edge_id = zklock.register_device(edge_device, "edge_wallet_0x789".to_string()).await?;
    info!("⚡ Registered edge computing device: {}", edge_id);

    // Register a wearable device
    let wearable_device = DeviceType::Wearable {
        form_factor: WearableType::Smartwatch,
        battery_class: BatteryClass::Standard,
    };
    let wearable_id = zklock.register_device(wearable_device, "wearable_wallet_0xabc".to_string()).await?;
    info!("⌚ Registered smartwatch device: {}", wearable_id);

    // Demo 2: Submit proofs from different devices
    info!("\n🔐 Demo 2: Zero-Knowledge Proof Submissions");

    // Mobile device submits a large proof
    let mobile_proof = generate_sample_proof(512); // 512 bytes
    let mobile_proof_id = zklock.submit_proof(mobile_id, mobile_proof).await?;
    info!("📱 Mobile device submitted proof: {}", mobile_proof_id);

    // IoT device submits a small proof
    let iot_proof = generate_sample_proof(64); // 64 bytes (IoT optimized)
    let iot_proof_id = zklock.submit_proof(iot_id, iot_proof).await?;
    info!("🔧 IoT device submitted proof: {}", iot_proof_id);

    // Edge device submits a complex proof
    let edge_proof = generate_sample_proof(2048); // 2KB (edge can handle larger)
    let edge_proof_id = zklock.submit_proof(edge_id, edge_proof).await?;
    info!("⚡ Edge device submitted proof: {}", edge_proof_id);

    // Wearable device submits a minimal proof
    let wearable_proof = generate_sample_proof(32); // 32 bytes (ultra-minimal)
    let wearable_proof_id = zklock.submit_proof(wearable_id, wearable_proof).await?;
    info!("⌚ Wearable device submitted proof: {}", wearable_proof_id);

    // Demo 3: Show device status and token earnings
    info!("\n💰 Demo 3: Device Status and Token Earnings");

    let mobile_status = zklock.get_device_status(mobile_id).await?;
    info!("📱 Mobile device status: {} tokens earned, {:.1} participation score", 
          mobile_status.tokens_earned, mobile_status.participation_score);

    let iot_status = zklock.get_device_status(iot_id).await?;
    info!("🔧 IoT device status: {} tokens earned, {:.1} participation score", 
          iot_status.tokens_earned, iot_status.participation_score);

    let edge_status = zklock.get_device_status(edge_id).await?;
    info!("⚡ Edge device status: {} tokens earned, {:.1} participation score", 
          edge_status.tokens_earned, edge_status.participation_score);

    let wearable_status = zklock.get_device_status(wearable_id).await?;
    info!("⌚ Wearable device status: {} tokens earned, {:.1} participation score", 
          wearable_status.tokens_earned, wearable_status.participation_score);

    // Demo 4: System statistics
    info!("\n📊 Demo 4: System Statistics");

    let stats = zklock.get_system_stats().await?;
    info!("📊 System Statistics:");
    info!("   • Total devices: {}", stats.total_devices);
    info!("   • Active devices: {}", stats.active_devices);
    info!("   • Total proofs submitted: {}", stats.total_proofs);
    info!("   • Total tokens distributed: {}", stats.total_tokens_distributed);
    info!("   • Merkle tree size: {}", stats.merkle_tree_size);
    info!("   • ICO participation rate: {:.1}%", stats.ico_participation_rate * 100.0);

    // Demo 5: Simulate continuous operation
    info!("\n🔄 Demo 5: Continuous Operation Simulation");
    info!("Simulating continuous device operation for 30 seconds...");

    for i in 1..=6 {
        sleep(Duration::from_secs(5)).await;
        
        // Each device submits another proof
        let mobile_proof = generate_sample_proof(256 + i * 10);
        zklock.submit_proof(mobile_id, mobile_proof).await?;
        
        let iot_proof = generate_sample_proof(32 + i * 2);
        zklock.submit_proof(iot_id, iot_proof).await?;
        
        info!("⏱️  Iteration {}/6: Devices submitted proofs", i);
    }

    // Final statistics
    info!("\n🎯 Final Results");
    let final_stats = zklock.get_system_stats().await?;
    info!("📊 Final System Statistics:");
    info!("   • Total devices: {}", final_stats.total_devices);
    info!("   • Active devices: {}", final_stats.active_devices);
    info!("   • Total proofs submitted: {}", final_stats.total_proofs);
    info!("   • Total tokens distributed: {}", final_stats.total_tokens_distributed);
    info!("   • Average tokens per device: {:.1}", 
          final_stats.total_tokens_distributed as f64 / final_stats.total_devices as f64);

    // Show individual device final status
    info!("\n🏆 Individual Device Performance:");
    
    let final_mobile = zklock.get_device_status(mobile_id).await?;
    info!("📱 Mobile: {} tokens, {} proofs", final_mobile.tokens_earned, final_mobile.proof_count);
    
    let final_iot = zklock.get_device_status(iot_id).await?;
    info!("🔧 IoT: {} tokens, {} proofs", final_iot.tokens_earned, final_iot.proof_count);
    
    let final_edge = zklock.get_device_status(edge_id).await?;
    info!("⚡ Edge: {} tokens, {} proofs", final_edge.tokens_earned, final_edge.proof_count);
    
    let final_wearable = zklock.get_device_status(wearable_id).await?;
    info!("⌚ Wearable: {} tokens, {} proofs", final_wearable.tokens_earned, final_wearable.proof_count);

    info!("\n🎉 ZKLock Mobile Port Demo completed successfully!");
    info!("This demonstrates how IoT and mobile devices can participate in the BPI ecosystem");
    info!("with optimized zero-knowledge proofs and token rewards through the zklock ICO system.");

    Ok(())
}

/// Generate a sample proof of specified size
fn generate_sample_proof(size: usize) -> Vec<u8> {
    // Generate pseudo-random proof data
    let mut proof = Vec::with_capacity(size);
    for i in 0..size {
        proof.push((i % 256) as u8);
    }
    proof
}
