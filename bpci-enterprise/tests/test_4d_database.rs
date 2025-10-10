//! Simple 4D Hash-Graph Database Test
//! 
//! Verifies the 4D database functionality works correctly

use tokio;
use serde_json::json;

// Import the 4D database - using correct crate name
use pravyom_enterprise::storage::{
    FourDHashGraphKernel, FourDConfig, FourDCoordinate, FourDDistance
};

#[tokio::test]
async fn test_revolutionary_4d_database_works() {
    println!("🚀 Testing 4D Hash-Graph Database Basic Functionality");
    
    // Create 4D database
    let config = FourDConfig::default();
    let db = FourDHashGraphKernel::new(config).await.expect("Failed to create 4D database");
    
    // Test health check
    let health = db.health_check().await.expect("Health check failed");
    assert!(health, "Database should be healthy");
    println!("✅ Database health check passed");
    
    // Test MongoDB-compatible insert
    let document = json!({
        "name": "Test User",
        "age": 30,
        "city": "Test City"
    });
    
    let doc_id = db.insert_document("test_collection", document).await
        .expect("Failed to insert document");
    assert!(!doc_id.is_empty(), "Document ID should not be empty");
    println!("✅ Document inserted with ID: {}", doc_id);
    
    // Test MongoDB-compatible find
    let query = json!({ "name": "Test User" });
    let result = db.find_documents("test_collection", query, Some(10)).await
        .expect("Failed to find documents");
    
    assert!(!result.documents.is_empty(), "Should find at least one document");
    assert!(result.query_time_ms < 1000, "Query should be fast");
    println!("✅ Found {} documents in {}ms", result.documents.len(), result.query_time_ms);
    
    // Test 4D coordinate operations
    let coord1 = FourDCoordinate { r: 100, c: 200, v: 1.5, i: 300 };
    let coord2 = FourDCoordinate { r: 150, c: 250, v: 2.0, i: 350 };
    
    let distance = FourDDistance::euclidean(&coord1, &coord2);
    assert!(distance > 0.0, "Distance should be positive");
    println!("✅ 4D distance calculation: {:.2}", distance);
    
    // Test database statistics
    let stats = db.get_stats().await;
    assert!(stats.total_nodes > 0, "Should have nodes after insert");
    println!("✅ Database stats: {} tiles, {} nodes, {} queries", 
             stats.total_tiles, stats.total_nodes, stats.queries_executed);
    
    println!("🎉 All 4D Hash-Graph Database tests passed!");
}

#[tokio::test]
async fn test_4d_database_performance() {
    println!("⚡ Testing 4D Database Performance");
    
    let config = FourDConfig::default();
    let db = FourDHashGraphKernel::new(config).await.expect("Failed to create 4D database");
    
    // Performance test: Insert multiple documents
    let start_time = std::time::Instant::now();
    
    for i in 0..10 {
        let doc = json!({
            "id": i,
            "data": format!("performance_test_{}", i),
            "timestamp": chrono::Utc::now().timestamp()
        });
        
        db.insert_document("performance", doc).await
            .expect("Failed to insert performance test document");
    }
    
    let insert_time = start_time.elapsed();
    let docs_per_sec = 10.0 / insert_time.as_secs_f64();
    
    println!("✅ Inserted 10 documents in {:?} ({:.2} docs/sec)", insert_time, docs_per_sec);
    assert!(docs_per_sec > 1.0, "Should insert at least 1 doc per second");
    
    // Performance test: Query documents
    let query_start = std::time::Instant::now();
    let query = json!({});
    let result = db.find_documents("performance", query, Some(20)).await
        .expect("Failed to query performance test documents");
    let query_time = query_start.elapsed();
    
    println!("✅ Queried {} documents in {:?}", result.documents.len(), query_time);
    assert!(query_time.as_millis() < 100, "Query should be under 100ms");
    
    println!("🎉 Performance tests passed!");
}

#[test]
fn test_4d_algebra_operations() {
    println!("🧮 Testing 4D Algebraic Operations");
    
    use pravyom_enterprise::storage::{FourDAlgebra, FourDTileRef, FourDBoundingBox, SecurityLevel};
    use uuid::Uuid;
    
    // Create test tiles
    let tile1 = FourDTileRef {
        tile_id: Uuid::new_v4(),
        bounding_box: FourDBoundingBox {
            r_min: 0, r_max: 100,
            c_min: 0, c_max: 100,
            v_min: 0.0, v_max: 10.0,
            i_min: 0, i_max: 100,
        },
        node_count: 5,
        total_size: 1024,
        aggregate_vector: vec![1.0, 2.0, 3.0],
        security_level: SecurityLevel::Public,
    };
    
    let tiles = vec![tile1];
    
    // Test 4D-Select operation
    let selected = FourDAlgebra::select_4d(&tiles, (0, 50), (0, 50), (0.0, 5.0), (0, 50));
    assert_eq!(selected.len(), 1, "Should select one tile");
    println!("✅ 4D-Select operation works");
    
    // Test 4D distance calculations
    let coord1 = FourDCoordinate { r: 0, c: 0, v: 0.0, i: 0 };
    let coord2 = FourDCoordinate { r: 1, c: 1, v: 1.0, i: 1 };
    
    let euclidean = FourDDistance::euclidean(&coord1, &coord2);
    let manhattan = FourDDistance::manhattan(&coord1, &coord2);
    
    assert!(euclidean > 0.0, "Euclidean distance should be positive");
    assert!(manhattan > 0.0, "Manhattan distance should be positive");
    println!("✅ 4D distance calculations work: euclidean={:.2}, manhattan={:.2}", euclidean, manhattan);
    
    println!("🎉 4D algebraic operations tests passed!");
}
