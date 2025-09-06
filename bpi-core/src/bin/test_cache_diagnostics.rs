use anyhow::Result;
use tracing::{info, warn, error};
use bpi_core::distributed_storage::{BpiDistributedStorage, DistributedStorageConfig, CloudProvider};
use bpi_core::enhanced_cdn_storage::{EnhancedCdnStorage, ContentType, GeographicLocation};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("🔍 Cache Diagnostics - Investigating Cache Issues");
    
    // Create enhanced CDN storage
    let config = DistributedStorageConfig {
        min_cloud_providers: 3,
        max_cloud_providers: 7,
        block_size_kb: 512,
        redundancy_factor: 3,
        instant_backup_threshold_ms: 2000,
        vm_audit_required: true,
    };
    
    let base_storage = BpiDistributedStorage::new(config);
    let enhanced_cdn = EnhancedCdnStorage::new(base_storage);
    
    let test_location = GeographicLocation {
        country: "USA".to_string(),
        city: "New York".to_string(),
        latitude: 40.7128,
        longitude: -74.0060,
        provider: CloudProvider::AWS,
    };
    
    // Test 1: Store content and verify storage
    info!("\n🧪 Test 1: Store Content and Verify Storage");
    let test_data = vec![1u8; 1024]; // 1KB test data
    let content_id = enhanced_cdn.store_big_data(&test_data, ContentType::Document, "test_doc.pdf").await?;
    info!("✅ Content stored with ID: {}", content_id);
    
    // Test 2: First retrieval (should be cache miss)
    info!("\n🧪 Test 2: First Retrieval (Expected Cache Miss)");
    let retrieved_data_1 = enhanced_cdn.retrieve_with_ultra_fast_cdn(&content_id, &test_location).await;
    match retrieved_data_1 {
        Ok(data) => {
            info!("✅ First retrieval successful: {} bytes", data.len());
            if data.len() != test_data.len() {
                warn!("⚠️  Data size mismatch: expected {}, got {}", test_data.len(), data.len());
            }
        }
        Err(e) => {
            error!("❌ First retrieval failed: {}", e);
            return Err(e);
        }
    }
    
    // Test 3: Second retrieval (should be cache hit)
    info!("\n🧪 Test 3: Second Retrieval (Expected Cache Hit)");
    let retrieved_data_2 = enhanced_cdn.retrieve_with_ultra_fast_cdn(&content_id, &test_location).await;
    match retrieved_data_2 {
        Ok(data) => {
            info!("✅ Second retrieval successful: {} bytes", data.len());
            if data.len() != test_data.len() {
                warn!("⚠️  Data size mismatch: expected {}, got {}", test_data.len(), data.len());
            }
        }
        Err(e) => {
            error!("❌ Second retrieval failed: {}", e);
            return Err(e);
        }
    }
    
    // Test 4: Multiple rapid retrievals to test cache consistency
    info!("\n🧪 Test 4: Multiple Rapid Retrievals (Cache Consistency Test)");
    let mut successful_retrievals = 0;
    let mut cache_hits = 0;
    let mut cache_misses = 0;
    
    for i in 0..10 {
        let start_time = std::time::Instant::now();
        match enhanced_cdn.retrieve_with_ultra_fast_cdn(&content_id, &test_location).await {
            Ok(data) => {
                successful_retrievals += 1;
                let duration = start_time.elapsed();
                if duration.as_millis() < 50 {
                    cache_hits += 1;
                    info!("🎯 Retrieval #{}: Cache HIT ({:?})", i+1, duration);
                } else {
                    cache_misses += 1;
                    info!("📡 Retrieval #{}: Cache MISS ({:?})", i+1, duration);
                }
                
                if data.len() != test_data.len() {
                    warn!("⚠️  Retrieval #{}: Data size mismatch", i+1);
                }
            }
            Err(e) => {
                error!("❌ Retrieval #{} failed: {}", i+1, e);
            }
        }
    }
    
    // Test 5: Cache statistics and metrics
    info!("\n🧪 Test 5: Cache Statistics and Metrics");
    let metrics = enhanced_cdn.get_performance_metrics().await?;
    info!("📊 Cache Performance Metrics:");
    info!("  🎯 Cache Hit Rate: {:.1}%", metrics.cache_hit_rate);
    info!("  ⚡ Average Latency: {}ms", metrics.average_latency_ms);
    info!("  🌐 Edge Nodes: {}", metrics.edge_nodes_count);
    info!("  💰 Cost Savings: {:.1}%", metrics.cost_savings_percent);
    info!("  📊 Content Served: {:.2} GB", metrics.total_content_served);
    info!("  🚀 Bandwidth Saved: {:.2} GB", metrics.bandwidth_saved_gb);
    
    // Test 6: Different content types cache behavior
    info!("\n🧪 Test 6: Different Content Types Cache Behavior");
    let content_types = vec![
        (ContentType::Image, "test_image.jpg"),
        (ContentType::Video, "test_video.mp4"),
        (ContentType::Audio, "test_audio.mp3"),
    ];
    
    for (content_type, filename) in content_types {
        let test_content = vec![2u8; 2048]; // 2KB test content
        let content_id = enhanced_cdn.store_big_data(&test_content, content_type.clone(), filename).await?;
        
        // Test retrieval
        match enhanced_cdn.retrieve_with_ultra_fast_cdn(&content_id, &test_location).await {
            Ok(data) => {
                info!("✅ {:?} content retrieval: {} bytes", content_type, data.len());
            }
            Err(e) => {
                error!("❌ {:?} content retrieval failed: {}", content_type, e);
            }
        }
    }
    
    // Summary
    info!("\n📋 Cache Diagnostics Summary:");
    info!("✅ Successful Retrievals: {}/10", successful_retrievals);
    info!("🎯 Cache Hits: {}", cache_hits);
    info!("📡 Cache Misses: {}", cache_misses);
    info!("📊 Cache Hit Rate: {:.1}%", (cache_hits as f64 / 10.0) * 100.0);
    
    if successful_retrievals == 10 {
        info!("🎉 All cache tests passed successfully!");
    } else {
        warn!("⚠️  Some cache operations failed - investigate further");
    }
    
    if cache_hits < 5 {
        warn!("⚠️  Low cache hit rate - cache may not be working optimally");
    } else {
        info!("✅ Cache hit rate is acceptable");
    }
    
    Ok(())
}
