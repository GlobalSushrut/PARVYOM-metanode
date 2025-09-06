//! # BPCI Authentication and Wallet Management Endpoints
//! 
//! Comprehensive user authentication and BPI wallet management system
//! for the BPCI web interface with secure session management

use axum::{
    extract::{Query, Path},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use ed25519_dalek::{SigningKey, VerifyingKey, Signer};
use rand::rngs::OsRng;
use tracing::{info, warn, error};
use anyhow::{Result, anyhow};

use crate::bpi_ledger_integration::BpiLedgerClient;

/// User account structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub wallet_ids: Vec<String>,
}

/// User session for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub session_id: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
}

/// BPI Wallet structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BpiWallet {
    pub wallet_id: String,
    pub user_id: String,
    pub wallet_name: String,
    pub public_key: String,
    pub private_key_encrypted: String,
    pub bpi_address: String,
    pub is_activated: bool,
    pub activation_tx_hash: Option<String>,
    pub balance: u64,
    pub created_at: DateTime<Utc>,
    pub activated_at: Option<DateTime<Utc>>,
}

/// Authentication requests
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

/// Wallet management requests
#[derive(Debug, Deserialize)]
pub struct CreateWalletRequest {
    pub wallet_name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ActivateWalletRequest {
    pub wallet_id: String,
    pub password: String,
}

/// API response structure
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

/// Community voting endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityVoter {
    pub id: String,
    pub name: String,
    pub email: String,
    pub voted_at: DateTime<Utc>,
    pub ip_address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VoteRequest {
    pub name: String,
    pub email: String,
}

// In-memory storage for demo (in production, use a database)
static mut VOTER_COUNT: u32 = 0;
static mut VOTERS: Vec<CommunityVoter> = Vec::new();

pub async fn get_voter_count() -> Result<Json<serde_json::Value>, StatusCode> {
    unsafe {
        Ok(Json(serde_json::json!({
            "count": VOTER_COUNT,
            "success": true
        })))
    }
}

pub async fn register_vote(Json(req): Json<VoteRequest>) -> Result<Json<ApiResponse<String>>, StatusCode> {
    info!("Registering community vote from: {}", req.name);
    
    // Check if email already voted (simple duplicate prevention)
    unsafe {
        let already_voted = VOTERS.iter().any(|v| v.email == req.email);
        if already_voted {
            return Ok(Json(ApiResponse {
                success: false,
                data: None,
                error: Some("Email has already voted".to_string()),
            }));
        }
        
        // Add new voter
        let voter = CommunityVoter {
            id: Uuid::new_v4().to_string(),
            name: req.name.clone(),
            email: req.email,
            voted_at: Utc::now(),
            ip_address: None, // Could extract from request headers
        };
        
        VOTERS.push(voter);
        VOTER_COUNT += 1;
        
        info!("Vote registered successfully. Total votes: {}", VOTER_COUNT);
    }
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(format!("Vote registered successfully! Total votes: {}", unsafe { VOTER_COUNT })),
        error: None,
    }))
}

pub async fn get_voters() -> Result<Json<ApiResponse<Vec<CommunityVoter>>>, StatusCode> {
    unsafe {
        Ok(Json(ApiResponse {
            success: true,
            data: Some(VOTERS.clone()),
            error: None,
        }))
    }
}

/// Login page HTML
pub async fn serve_login_page() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Login - BPCI Enterprise</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; display: flex; align-items: center; justify-content: center; }
        .login-container { background: rgba(255,255,255,0.1); backdrop-filter: blur(10px); padding: 3rem; border-radius: 15px; box-shadow: 0 8px 32px rgba(0,0,0,0.1); width: 100%; max-width: 400px; }
        .logo { text-align: center; color: white; font-size: 2rem; font-weight: bold; margin-bottom: 2rem; }
        .form-group { margin-bottom: 1.5rem; }
        .form-group label { display: block; color: white; margin-bottom: 0.5rem; font-weight: 500; }
        .form-group input { width: 100%; padding: 0.75rem; border: 1px solid rgba(255,255,255,0.3); border-radius: 5px; background: rgba(255,255,255,0.1); color: white; font-size: 1rem; }
                    errorDiv.style.display = 'block';
                }
            } catch (error) {
                errorDiv.textContent = 'Network error. Please try again.';
                errorDiv.style.display = 'block';
            }
            
            loginBtn.disabled = false;
            loginBtn.textContent = 'Login';
        });
    </script>
</body>
</html>
    "#)
}

/// Registration page HTML
pub async fn serve_register_page() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Register - BPCI Enterprise</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; display: flex; align-items: center; justify-content: center; }
        .register-container { background: rgba(255,255,255,0.1); backdrop-filter: blur(10px); padding: 3rem; border-radius: 15px; box-shadow: 0 8px 32px rgba(0,0,0,0.1); width: 100%; max-width: 400px; }
        .logo { text-align: center; color: white; font-size: 2rem; font-weight: bold; margin-bottom: 2rem; }
        .form-group { margin-bottom: 1.5rem; }
        .form-group label { display: block; color: white; margin-bottom: 0.5rem; font-weight: 500; }
        .form-group input { width: 100%; padding: 0.75rem; border: 1px solid rgba(255,255,255,0.3); border-radius: 5px; background: rgba(255,255,255,0.1); color: white; font-size: 1rem; }
        .form-group input::placeholder { color: rgba(255,255,255,0.7); }
        .form-group input:focus { outline: none; border-color: #3498db; box-shadow: 0 0 0 2px rgba(52,152,219,0.3); }
        .btn { width: 100%; padding: 0.75rem; border: none; border-radius: 5px; background: #3498db; color: white; font-size: 1rem; font-weight: 500; cursor: pointer; transition: all 0.3s; }
        .btn:hover { background: #2980b9; transform: translateY(-2px); }
        .btn:disabled { background: #7f8c8d; cursor: not-allowed; transform: none; }
        .links { text-align: center; margin-top: 1.5rem; }
        .links a { color: white; text-decoration: none; opacity: 0.8; }
        .links a:hover { opacity: 1; }
        .error { background: rgba(231,76,60,0.2); border: 1px solid #e74c3c; color: white; padding: 0.75rem; border-radius: 5px; margin-bottom: 1rem; display: none; }
        .success { background: rgba(46,204,113,0.2); border: 1px solid #2ecc71; color: white; padding: 0.75rem; border-radius: 5px; margin-bottom: 1rem; display: none; }
    </style>
</head>
<body>
    <div class="register-container">
        <div class="logo">🚀 BPCI Enterprise</div>
        <div id="error-message" class="error"></div>
        <div id="success-message" class="success"></div>
        <form id="register-form">
            <div class="form-group">
                <label for="email">Email Address</label>
                <input type="email" id="email" name="email" placeholder="Enter your email" required>
            </div>
            <div class="form-group">
                <label for="password">Password</label>
                <input type="password" id="password" name="password" placeholder="Enter your password" required minlength="8">
            </div>
            <div class="form-group">
                <label for="confirm-password">Confirm Password</label>
                <input type="password" id="confirm-password" name="confirm-password" placeholder="Confirm your password" required>
            </div>
            <button type="submit" class="btn" id="register-btn">Register</button>
        </form>
        <div class="links">
            <a href="/login">Already have an account? Login</a><br>
            <a href="/">← Back to Home</a>
        </div>
    </div>
    
    <script>
        document.getElementById('register-form').addEventListener('submit', async (e) => {
            e.preventDefault();
            
            const email = document.getElementById('email').value;
            const password = document.getElementById('password').value;
            const confirmPassword = document.getElementById('confirm-password').value;
            const registerBtn = document.getElementById('register-btn');
            const errorDiv = document.getElementById('error-message');
            const successDiv = document.getElementById('success-message');
            
            if (password !== confirmPassword) {
                errorDiv.textContent = 'Passwords do not match';
                errorDiv.style.display = 'block';
                return;
            }
            
            registerBtn.disabled = true;
            registerBtn.textContent = 'Creating account...';
            errorDiv.style.display = 'none';
            successDiv.style.display = 'none';
            
            try {
                const response = await fetch('/api/auth/register', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ email, password, confirm_password: confirmPassword })
                });
                
                const result = await response.json();
                
                if (result.success) {
                    successDiv.textContent = 'Account created successfully! Redirecting to login...';
                    successDiv.style.display = 'block';
                    setTimeout(() => window.location.href = '/login', 2000);
                } else {
                    errorDiv.textContent = result.error || 'Registration failed';
                    errorDiv.style.display = 'block';
                }
            } catch (error) {
                errorDiv.textContent = 'Network error. Please try again.';
                errorDiv.style.display = 'block';
            }
            
            registerBtn.disabled = false;
            registerBtn.textContent = 'Register';
        });
    </script>
</body>
</html>
    "#)
}

/// Hash password using SHA256
pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Verify password against hash
pub fn verify_password(password: &str, hash: &str) -> bool {
    hash_password(password) == hash
}

/// Generate session token
pub fn generate_session_token() -> String {
    Uuid::new_v4().to_string()
}

/// Encrypt private key (simplified - in production use proper encryption)
pub fn encrypt_private_key(private_key: &str, password: &str) -> String {
    // Simplified encryption - in production, use proper encryption like AES
    let mut hasher = Sha256::new();
    hasher.update(private_key.as_bytes());
    hasher.update(password.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Generate BPI address from public key
pub fn generate_bpi_address(public_key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(public_key.as_bytes());
    let hash = hasher.finalize();
    format!("bpi_{}", hex::encode(&hash[..16]))
}
