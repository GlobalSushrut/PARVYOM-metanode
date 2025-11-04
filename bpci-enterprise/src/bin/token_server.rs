use std::env;
use tokio;
use pravyom_enterprise::api::start_api_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    println!("🚀 BPI Token & Address Management Server");
    println!("========================================");
    
    // Get port from environment or use default
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    
    println!("🔧 Configuration:");
    println!("   - Port: {}", port);
    println!("   - Database: Production instance (same as tests)");
    println!("   - Environment: {}", env::var("RUST_ENV").unwrap_or_else(|_| "production".to_string()));
    
    // Start the API server with the same database instance used in tests
    match start_api_server(port).await {
        Ok(_) => {
            println!("✅ Server started successfully!");
        }
        Err(e) => {
            eprintln!("❌ Failed to start server: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}
