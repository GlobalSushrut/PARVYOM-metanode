//! HERMES-Lite Web-4 Node Binary - Mainnet Ready
//! 
//! Simple, reliable P2P node for BPCI testnet and mainnet

use hermes_lite_web4::{HermesLiteWeb4, HermesConfig, P2PMessage, MessageType, TrafficClass};
use std::env;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("Starting HERMES-Lite Web-4 Node");
    
    // Load configuration
    let config = if let Some(config_path) = env::args().nth(1) {
        HermesConfig::from_file(config_path)?
    } else {
        // Default to testnet for now
        HermesConfig::testnet()
    };
    
    info!("Node configuration: {:?}", config);
    
    // Create and start P2P node
    let mut node = HermesLiteWeb4::new(config);
    
    match node.start().await {
        Ok(()) => {
            info!("HERMES-Lite Web-4 node started successfully");
            
            // Demo: Send some test messages
            demo_bpci_messages(&mut node).await?;
            
            // Keep running
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                let status = node.status();
                info!("Node status: {} neighbors, ready: {}", 
                      status.neighbor_count, status.is_ready);
            }
        }
        Err(e) => {
            error!("Failed to start HERMES-Lite node: {}", e);
            return Err(e);
        }
    }
}

/// Demo BPCI message types and priorities
async fn demo_bpci_messages(node: &mut HermesLiteWeb4) -> Result<(), Box<dyn std::error::Error>> {
    info!("Demonstrating BPCI message priorities");
    
    let target_node = hermes_lite_web4::NodeId::from_string("node_demo_target".to_string());
    
    // C1: Consensus message (highest priority)
    let consensus_msg = P2PMessage::consensus(
        MessageType::IbftPrepare,
        b"consensus_vote_data".to_vec()
    );
    node.send_message(target_node.clone(), consensus_msg).await?;
    info!("Sent C1 consensus message (highest priority)");
    
    // C2: Auction message (medium priority)
    let auction_msg = P2PMessage::auction(
        MessageType::AuctionBid,
        b"auction_bid_data".to_vec()
    );
    node.send_message(target_node.clone(), auction_msg).await?;
    info!("Sent C2 auction message (medium priority)");
    
    // C3: Shadow data message (background priority)
    let shadow_msg = P2PMessage::shadow_data(
        MessageType::BlockData,
        b"block_sync_data".to_vec()
    );
    node.send_message(target_node, shadow_msg).await?;
    info!("Sent C3 shadow data message (background priority)");
    
    Ok(())
}
