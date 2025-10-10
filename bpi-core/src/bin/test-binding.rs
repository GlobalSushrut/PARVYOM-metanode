use tokio::net::TcpListener;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Testing BPI Core Server Binding Issue");
    println!("========================================");
    
    // Test binding to port 9545 (RPC)
    println!("\n1. Testing port 9545 binding:");
    let addr_9545: SocketAddr = "0.0.0.0:9545".parse().unwrap();
    match TcpListener::bind(addr_9545).await {
        Ok(listener) => {
            println!("✅ Port 9545 bound successfully!");
            println!("   Local address: {}", listener.local_addr()?);
        }
        Err(e) => {
            println!("❌ Port 9545 binding failed: {}", e);
        }
    }
    
    // Test binding to port 9546 (API)
    println!("\n2. Testing port 9546 binding:");
    let addr_9546: SocketAddr = "0.0.0.0:9546".parse().unwrap();
    match TcpListener::bind(addr_9546).await {
        Ok(listener) => {
            println!("✅ Port 9546 bound successfully!");
            println!("   Local address: {}", listener.local_addr()?);
        }
        Err(e) => {
            println!("❌ Port 9546 binding failed: {}", e);
        }
    }
    
    Ok(())
}
