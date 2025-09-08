use anyhow::Result;
use std::env;
use tokio;

// Import the domain management API module
use bpi_core::domain_management_api::DomainManagementApi;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    println!("🚀 Starting HTTPCG Domain Management API Server");
    println!("🔧 Initializing database and services...");
    
    // Get configuration from environment variables
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./domain_management.db".to_string());
    
    let jwt_secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());
    
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);
    
    // Create and start the API service
    let api_service = DomainManagementApi::new(&database_url, jwt_secret).await?;
    
    println!("✅ Database initialized successfully");
    println!("🌐 Starting web server on port {}", port);
    println!("📚 API Endpoints:");
    println!("   GET  /health - Health check");
    println!("   GET  /api/v1/domains/check/:domain - Check domain availability");
    println!("   POST /api/v1/domains/apply - Submit domain application");
    println!("   GET  /api/v1/domains/applications/:id - Get application status");
    println!("   POST /api/v1/domains/waitlist - Join domain waitlist");
    println!("   POST /api/v1/domains/web2-mapping - Register Web2 mapping");
    println!("   GET  /api/v1/domains - List registered domains");
    println!("   GET  /api/v1/domains/:domain/info - Get domain information");
    println!("   GET  /api/v1/stats - Get registry statistics");
    println!("   GET  /api/v1/admin/domains/pending - List pending applications (admin)");
    println!("   POST /api/v1/admin/domains/:id/approve - Approve application (admin)");
    println!("   POST /api/v1/admin/domains/:id/reject - Reject application (admin)");
    println!("   POST /api/v1/auth/login - User authentication");
    println!("   POST /api/v1/auth/register - User registration");
    println!();
    println!("🔐 Default admin credentials:");
    println!("   Email: admin@httpcg.com");
    println!("   Password: admin123");
    println!("   ⚠️  Change these credentials in production!");
    println!();
    
    // Start the server
    api_service.start_server(port).await?;
    
    Ok(())
}
