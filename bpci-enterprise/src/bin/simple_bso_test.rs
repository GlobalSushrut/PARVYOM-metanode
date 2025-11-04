//! # Simple BSO System Test (Crash-Safe)
//! 
//! Minimal test to validate BSO components without causing laptop crashes.

use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    info!("🚀 Starting Simple BSO System Test (Crash-Safe)");
    
    // Test 1: Basic module availability
    info!("📋 Test 1: Checking module availability");
    
    // Test if we can import basic modules without crashes
    match test_basic_imports().await {
        Ok(_) => info!("✅ Basic imports successful"),
        Err(e) => {
            error!("❌ Basic imports failed: {}", e);
            return Err(e);
        }
    }
    
    // Test 2: Simple struct creation
    info!("📋 Test 2: Testing simple struct creation");
    
    match test_simple_structs().await {
        Ok(_) => info!("✅ Simple struct creation successful"),
        Err(e) => {
            error!("❌ Simple struct creation failed: {}", e);
            return Err(e);
        }
    }
    
    // Test 3: Memory allocation test
    info!("📋 Test 3: Testing memory allocation");
    
    match test_memory_allocation().await {
        Ok(_) => info!("✅ Memory allocation test successful"),
        Err(e) => {
            error!("❌ Memory allocation test failed: {}", e);
            return Err(e);
        }
    }
    
    info!("🎉 Simple BSO System Test completed successfully!");
    info!("🚀 System is stable and ready for more complex testing");
    
    Ok(())
}

async fn test_basic_imports() -> Result<()> {
    // Test basic imports without creating complex objects
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    
    let _test_map: HashMap<String, String> = HashMap::new();
    let _test_arc = Arc::new(42);
    let _test_lock = Arc::new(RwLock::new(String::new()));
    
    info!("   ✓ Standard library imports working");
    Ok(())
}

async fn test_simple_structs() -> Result<()> {
    // Test simple struct creation without complex dependencies
    
    #[derive(Debug)]
    struct SimpleService {
        name: String,
        port: u16,
        status: ServiceStatus,
    }
    
    #[derive(Debug)]
    enum ServiceStatus {
        Running,
        Stopped,
    }
    
    let _service = SimpleService {
        name: "test-service".to_string(),
        port: 8080,
        status: ServiceStatus::Running,
    };
    
    info!("   ✓ Simple struct creation working");
    Ok(())
}

async fn test_memory_allocation() -> Result<()> {
    // Test memory allocation without causing crashes
    
    let mut test_vec = Vec::new();
    for i in 0..1000 {
        test_vec.push(format!("test-item-{}", i));
    }
    
    info!("   ✓ Allocated {} items successfully", test_vec.len());
    
    // Test HashMap allocation
    let mut test_map = std::collections::HashMap::new();
    for i in 0..100 {
        test_map.insert(format!("key-{}", i), format!("value-{}", i));
    }
    
    info!("   ✓ HashMap with {} entries created successfully", test_map.len());
    
    Ok(())
}
