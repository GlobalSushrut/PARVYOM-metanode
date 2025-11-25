use bpi_core::blockchain_os_kernel::commute_lock::{
    CompressionType,
    MessageMetadata,
    MessageType,
    Priority,
    RoutingInfo,
};
use chrono::Utc;
use std::time::Duration;

#[test]
fn bpi_core_28_mesh_native_communication_envelope_preview() {
    println!("=== Test: BPI-CORE-28: Mesh-native communication envelope ===");

    // Simulated payload and content length
    let payload = b"demo-mesh-payload-zero-copy-preview".to_vec();
    let content_length = payload.len();

    // Build message metadata (header fields)
    let metadata = MessageMetadata {
        message_type: MessageType::Data,
        content_length,
        priority: Priority::High,
        ttl: Duration::from_secs(30),
        created_at: Utc::now(),
        compression: Some(CompressionType::Lz4),
    };

    // Build routing info as factorial-wave style path
    let routing = RoutingInfo {
        source_address: vec![1, 0, 3],
        target_address: vec![2, 1, 4],
        routing_path: vec![
            "node://mesh/root".to_string(),
            "node://mesh/edge-a".to_string(),
            "node://mesh/edge-b".to_string(),
            "node://mesh/target".to_string(),
        ],
        hop_count: 4,
        weight: 0.73,
    };

    println!("message_type: {:?}", metadata.message_type);
    println!("content_length: {} bytes", metadata.content_length);
    println!("priority: {:?}", metadata.priority);
    println!("ttl_secs: {}", metadata.ttl.as_secs());
    println!("created_at: {}", metadata.created_at.to_rfc3339());
    println!("compression: {:?}", metadata.compression);

    println!("source_address(factoradic): {:?}", routing.source_address);
    println!("target_address(factoradic): {:?}", routing.target_address);
    println!("routing_path: {:?}", routing.routing_path);
    println!("hop_count: {}", routing.hop_count);
    println!("weight: {:.3}", routing.weight);

    // Invariants for the mesh-native envelope
    assert_eq!(metadata.content_length, payload.len());
    assert!(metadata.ttl.as_secs() > 0, "TTL should be positive for live messages");

    // Priority ordering sanity: High must be >= Normal
    assert!(Priority::High > Priority::Normal);

    // Hop count should reflect the routing path length
    assert_eq!(routing.hop_count as usize, routing.routing_path.len());

    // Factoradic addresses should be non-empty for both ends
    assert!(!routing.source_address.is_empty());
    assert!(!routing.target_address.is_empty());

    println!("status: OK");
}
