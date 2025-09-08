use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqlitePool};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

// Database Models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
    pub role: String, // applicant, admin, super_admin
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DomainApplication {
    pub id: i64,
    pub application_id: String,
    pub user_id: i64,
    pub domain_name: String,
    pub domain_type: String,
    pub organization: String,
    pub email: String,
    pub reason: String,
    pub status: String, // pending, approved, rejected
    pub application_date: DateTime<Utc>,
    pub review_date: Option<DateTime<Utc>>,
    pub reviewer_id: Option<i64>,
    pub rejection_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DomainWaitlist {
    pub id: i64,
    pub user_id: i64,
    pub domain_name: String,
    pub joined_at: DateTime<Utc>,
    pub notified_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Web2Mapping {
    pub id: i64,
    pub mapping_id: String,
    pub httpcg_domain: String,
    pub web2_domain: String,
    pub ssl_cert_path: Option<String>,
    pub status: String, // active, inactive, pending
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuditLog {
    pub id: i64,
    pub user_id: Option<i64>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: String,
    pub details: String,
    pub timestamp: DateTime<Utc>,
}

// API Request/Response Models
#[derive(Debug, Deserialize)]
pub struct DomainApplicationRequest {
    pub domain_name: String,
    pub domain_type: String,
    pub organization: String,
    pub email: String,
    pub reason: String,
}

#[derive(Debug, Serialize)]
pub struct DomainApplicationResponse {
    pub application_id: String,
    pub status: String,
    pub message: String,
    pub estimated_review_time: String,
}

#[derive(Debug, Serialize)]
pub struct DomainAvailabilityResponse {
    pub domain: String,
    pub available: bool,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct Web2MappingRequest {
    pub httpcg_domain: String,
    pub web2_domain: String,
    pub ssl_cert_path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Web2MappingResponse {
    pub mapping_id: String,
    pub httpcg_domain: String,
    pub web2_domain: String,
    pub httpcg_endpoint: String,
    pub https_endpoint: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct ApprovalRequest {
    pub approved: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RegistryStats {
    pub total_domains: u64,
    pub active_domains: u64,
    pub pending_applications: u64,
    pub waitlist_entries: u64,
    pub web2_mappings: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub email: String,
    pub role: String,
    pub exp: usize,
}

// Application State
#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub jwt_secret: String,
    pub email_service: EmailService,
    pub httpcg_registry: HttpcgRegistryClient,
}

#[derive(Clone)]
pub struct EmailService {
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

#[derive(Clone)]
pub struct HttpcgRegistryClient {
    pub base_url: String,
}

// Domain Management API Service
pub struct DomainManagementApi {
    pub state: AppState,
}

impl DomainManagementApi {
    pub async fn new(database_url: &str, jwt_secret: String) -> Result<Self> {
        let db = SqlitePool::connect(database_url).await?;
        
        // Run database migrations
        Self::setup_database(&db).await?;
        
        let email_service = EmailService {
            smtp_server: std::env::var("SMTP_SERVER").unwrap_or_else(|_| "localhost:587".to_string()),
            smtp_username: std::env::var("SMTP_USERNAME").unwrap_or_else(|_| "noreply@httpcg.com".to_string()),
            smtp_password: std::env::var("SMTP_PASSWORD").unwrap_or_else(|_| "password".to_string()),
        };
        
        let httpcg_registry = HttpcgRegistryClient {
            base_url: std::env::var("HTTPCG_REGISTRY_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
        };
        
        let state = AppState {
            db,
            jwt_secret,
            email_service,
            httpcg_registry,
        };
        
        Ok(Self { state })
    }
    
    async fn setup_database(db: &SqlitePool) -> Result<()> {
        // Create users table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                role TEXT NOT NULL DEFAULT 'applicant',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
        "#).execute(db).await?;
        
        // Create domain_applications table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS domain_applications (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                application_id TEXT UNIQUE NOT NULL,
                user_id INTEGER NOT NULL,
                domain_name TEXT NOT NULL,
                domain_type TEXT NOT NULL,
                organization TEXT NOT NULL,
                email TEXT NOT NULL,
                reason TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending',
                application_date DATETIME DEFAULT CURRENT_TIMESTAMP,
                review_date DATETIME,
                reviewer_id INTEGER,
                rejection_reason TEXT,
                FOREIGN KEY (user_id) REFERENCES users (id),
                FOREIGN KEY (reviewer_id) REFERENCES users (id)
            )
        "#).execute(db).await?;
        
        // Create domain_waitlist table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS domain_waitlist (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                domain_name TEXT NOT NULL,
                joined_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                notified_at DATETIME,
                FOREIGN KEY (user_id) REFERENCES users (id),
                UNIQUE(user_id, domain_name)
            )
        "#).execute(db).await?;
        
        // Create web2_mappings table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS web2_mappings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                mapping_id TEXT UNIQUE NOT NULL,
                httpcg_domain TEXT NOT NULL,
                web2_domain TEXT NOT NULL,
                ssl_cert_path TEXT,
                status TEXT NOT NULL DEFAULT 'active',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(httpcg_domain, web2_domain)
            )
        "#).execute(db).await?;
        
        // Create audit_logs table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS audit_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER,
                action TEXT NOT NULL,
                resource_type TEXT NOT NULL,
                resource_id TEXT NOT NULL,
                details TEXT NOT NULL,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES users (id)
            )
        "#).execute(db).await?;
        
        // Create system_config table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS system_config (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
        "#).execute(db).await?;
        
        // Create default admin user if not exists
        let admin_exists = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM users WHERE role = 'super_admin'"
        ).fetch_one(db).await?;
        
        if admin_exists == 0 {
            let password_hash = bcrypt::hash("admin123", bcrypt::DEFAULT_COST)?;
            sqlx::query(r#"
                INSERT INTO users (email, password_hash, role)
                VALUES ('admin@httpcg.com', ?, 'super_admin')
            "#)
            .bind(password_hash)
            .execute(db)
            .await?;
        }
        
        Ok(())
    }
    
    pub fn create_router(&self) -> Router {
        Router::new()
            // Public endpoints
            .route("/api/v1/domains/check/:domain", get(check_domain_availability))
            .route("/api/v1/domains/apply", post(apply_for_domain))
            .route("/api/v1/domains/applications/:id", get(get_application_status))
            .route("/api/v1/domains/waitlist", post(join_waitlist))
            .route("/api/v1/domains/web2-mapping", post(register_web2_mapping))
            .route("/api/v1/domains", get(list_domains))
            .route("/api/v1/domains/:domain/info", get(get_domain_info))
            .route("/api/v1/stats", get(get_registry_stats))
            
            // Admin endpoints
            .route("/api/v1/admin/domains/pending", get(list_pending_applications))
            .route("/api/v1/admin/domains/:id/approve", post(approve_application))
            .route("/api/v1/admin/domains/:id/reject", post(reject_application))
            
            // Authentication endpoints
            .route("/api/v1/auth/login", post(login))
            .route("/api/v1/auth/register", post(register))
            
            // Health check
            .route("/health", get(health_check))
            
            .layer(
                ServiceBuilder::new()
                    .layer(CorsLayer::permissive())
            )
            .with_state(self.state.clone())
    }
    
    pub async fn start_server(&self, port: u16) -> Result<()> {
        let app = self.create_router();
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        
        println!("🚀 HTTPCG Domain Management API Server starting on port {}", port);
        println!("📋 API Documentation available at http://localhost:{}/docs", port);
        println!("🔍 Health check available at http://localhost:{}/health", port);
        
        axum::serve(listener, app).await?;
        Ok(())
    }
}

// API Handler Functions
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "HTTPCG Domain Management API",
        "version": "1.0.0",
        "timestamp": Utc::now()
    }))
}

async fn check_domain_availability(
    Path(domain): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<DomainAvailabilityResponse>, StatusCode> {
    // Check if domain is already registered in our system
    let existing = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM domain_applications WHERE domain_name = ? AND status = 'approved'"
    )
    .bind(&domain)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let available = existing == 0;
    
    // Log the check
    let _ = sqlx::query(
        "INSERT INTO audit_logs (action, resource_type, resource_id, details) VALUES (?, ?, ?, ?)"
    )
    .bind("check_availability")
    .bind("domain")
    .bind(&domain)
    .bind(format!("Domain availability check: {}", available))
    .execute(&state.db)
    .await;
    
    Ok(Json(DomainAvailabilityResponse {
        domain: domain.clone(),
        available,
        status: if available { "available".to_string() } else { "unavailable".to_string() },
        message: if available {
            "Domain is available for registration".to_string()
        } else {
            "Domain is already registered or pending approval".to_string()
        },
    }))
}

async fn apply_for_domain(
    State(state): State<AppState>,
    Json(request): Json<DomainApplicationRequest>,
) -> Result<Json<DomainApplicationResponse>, StatusCode> {
    let application_id = format!("app_{}", Uuid::new_v4().to_string()[..8].to_string());
    
    // Create application record
    let result = sqlx::query(r#"
        INSERT INTO domain_applications 
        (application_id, user_id, domain_name, domain_type, organization, email, reason)
        VALUES (?, 1, ?, ?, ?, ?, ?)
    "#)
    .bind(&application_id)
    .bind(&request.domain_name)
    .bind(&request.domain_type)
    .bind(&request.organization)
    .bind(&request.email)
    .bind(&request.reason)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Log the application
    let _ = sqlx::query(
        "INSERT INTO audit_logs (action, resource_type, resource_id, details) VALUES (?, ?, ?, ?)"
    )
    .bind("domain_application")
    .bind("domain")
    .bind(&request.domain_name)
    .bind(format!("Domain application submitted: {}", application_id))
    .execute(&state.db)
    .await;
    
    // Send notification email (placeholder)
    let _ = state.email_service.send_application_notification(&request.email, &application_id).await;
    
    Ok(Json(DomainApplicationResponse {
        application_id,
        status: "submitted".to_string(),
        message: "Application submitted for review. You will be notified via email when processed.".to_string(),
        estimated_review_time: "3-5 business days".to_string(),
    }))
}

async fn get_application_status(
    Path(application_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let application = sqlx::query_as::<_, DomainApplication>(
        "SELECT * FROM domain_applications WHERE application_id = ?"
    )
    .bind(&application_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match application {
        Some(app) => Ok(Json(serde_json::json!({
            "application_id": app.application_id,
            "domain": app.domain_name,
            "status": app.status,
            "application_date": app.application_date,
            "review_date": app.review_date,
            "rejection_reason": app.rejection_reason
        }))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn register_web2_mapping(
    State(state): State<AppState>,
    Json(request): Json<Web2MappingRequest>,
) -> Result<Json<Web2MappingResponse>, StatusCode> {
    let mapping_id = format!("map_{}", Uuid::new_v4().to_string()[..8].to_string());
    
    // Create Web2 mapping record
    let result = sqlx::query(r#"
        INSERT INTO web2_mappings 
        (mapping_id, httpcg_domain, web2_domain, ssl_cert_path)
        VALUES (?, ?, ?, ?)
    "#)
    .bind(&mapping_id)
    .bind(&request.httpcg_domain)
    .bind(&request.web2_domain)
    .bind(&request.ssl_cert_path)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Log the mapping
    let _ = sqlx::query(
        "INSERT INTO audit_logs (action, resource_type, resource_id, details) VALUES (?, ?, ?, ?)"
    )
    .bind("web2_mapping")
    .bind("mapping")
    .bind(&mapping_id)
    .bind(format!("Web2 mapping created: {} -> {}", request.httpcg_domain, request.web2_domain))
    .execute(&state.db)
    .await;
    
    Ok(Json(Web2MappingResponse {
        mapping_id,
        httpcg_domain: request.httpcg_domain.clone(),
        web2_domain: request.web2_domain.clone(),
        httpcg_endpoint: format!("httpcg://{}", request.httpcg_domain),
        https_endpoint: format!("https://{}", request.web2_domain),
        status: "registered".to_string(),
    }))
}

async fn get_registry_stats(
    State(state): State<AppState>,
) -> Result<Json<RegistryStats>, StatusCode> {
    let total_domains = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM domain_applications WHERE status = 'approved'"
    ).fetch_one(&state.db).await.unwrap_or(0);
    
    let pending_applications = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM domain_applications WHERE status = 'pending'"
    ).fetch_one(&state.db).await.unwrap_or(0);
    
    let waitlist_entries = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM domain_waitlist"
    ).fetch_one(&state.db).await.unwrap_or(0);
    
    let web2_mappings = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM web2_mappings WHERE status = 'active'"
    ).fetch_one(&state.db).await.unwrap_or(0);
    
    Ok(Json(RegistryStats {
        total_domains: total_domains as u64,
        active_domains: total_domains as u64,
        pending_applications: pending_applications as u64,
        waitlist_entries: waitlist_entries as u64,
        web2_mappings: web2_mappings as u64,
    }))
}

// Placeholder implementations for remaining handlers
async fn join_waitlist() -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Waitlist functionality coming soon"}))
}

async fn list_domains() -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Domain listing functionality coming soon"}))
}

async fn get_domain_info() -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Domain info functionality coming soon"}))
}

async fn list_pending_applications() -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Admin functionality coming soon"}))
}

async fn approve_application() -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Admin functionality coming soon"}))
}

async fn reject_application() -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Admin functionality coming soon"}))
}

async fn login() -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Authentication functionality coming soon"}))
}

async fn register() -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Authentication functionality coming soon"}))
}

// Email Service Implementation
impl EmailService {
    pub async fn send_application_notification(&self, email: &str, application_id: &str) -> Result<()> {
        // Placeholder for email notification
        println!("📧 Sending application notification to {} for application {}", email, application_id);
        Ok(())
    }
}

// HTTPCG Registry Client Implementation
impl HttpcgRegistryClient {
    pub async fn check_domain_availability(&self, domain: &str) -> Result<bool> {
        // Placeholder for HTTPCG registry integration
        println!("🔍 Checking domain availability in HTTPCG registry: {}", domain);
        Ok(true)
    }
    
    pub async fn register_domain(&self, domain: &str) -> Result<()> {
        // Placeholder for domain registration in HTTPCG registry
        println!("📝 Registering domain in HTTPCG registry: {}", domain);
        Ok(())
    }
}
