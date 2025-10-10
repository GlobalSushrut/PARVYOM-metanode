//! Email Verification Service for Web 3.5 Wallet Creation
//! 
//! Extends the existing EmailOtpService to support wallet creation flows

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Email verification service specifically for Web 3.5 wallet creation
#[derive(Debug, Clone)]
pub struct EmailVerificationService {
    /// Pending wallet creation requests
    pending_requests: Arc<RwLock<HashMap<String, WalletCreationRequest>>>,
}

/// Wallet creation request triggered by email verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletCreationRequest {
    pub request_id: String,
    pub email: String,
    pub verification_token: String,
    pub created_at: u64,
    pub expires_at: u64,
    pub verified: bool,
    pub wallet_creation_initiated: bool,
}

/// Email verification request for wallet creation
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletEmailVerificationRequest {
    pub email: String,
    pub preferred_domain: Option<String>, // @global, @corp, etc.
}

/// Email verification response
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletEmailVerificationResponse {
    pub success: bool,
    pub message: String,
    pub request_id: Option<String>,
    pub verification_link: Option<String>,
    pub expires_in_seconds: Option<u64>,
}

/// Wallet creation trigger request (from email link)
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletCreationTriggerRequest {
    pub request_id: String,
    pub verification_token: String,
}

/// Wallet creation trigger response
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletCreationTriggerResponse {
    pub success: bool,
    pub message: String,
    pub wallet_creation_url: Option<String>,
    pub request_id: Option<String>,
}

impl EmailVerificationService {
    /// Create new email verification service
    pub fn new() -> Self {
        Self {
            pending_requests: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Send wallet creation verification email
    pub async fn send_wallet_verification_email(
        &self,
        request: WalletEmailVerificationRequest,
    ) -> Result<WalletEmailVerificationResponse> {
        // Validate email format
        if !self.is_valid_email(&request.email) {
            return Ok(WalletEmailVerificationResponse {
                success: false,
                message: "Invalid email address format".to_string(),
                request_id: None,
                verification_link: None,
                expires_in_seconds: None,
            });
        }

        // Generate unique request ID and verification token
        let request_id = Uuid::new_v4().to_string();
        let verification_token = self.generate_verification_token();
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let expires_at = now + 3600; // 1 hour expiration

        // Create wallet creation request
        let wallet_request = WalletCreationRequest {
            request_id: request_id.clone(),
            email: request.email.clone(),
            verification_token: verification_token.clone(),
            created_at: now,
            expires_at,
            verified: false,
            wallet_creation_initiated: false,
        };

        // Store the request
        {
            let mut storage = self.pending_requests.write().unwrap();
            storage.insert(request_id.clone(), wallet_request);
        }

        // Generate verification link
        let verification_link = format!(
            "https://wallet.bpi.global/create?request_id={}&token={}",
            request_id, verification_token
        );

        // Send email (for now, just log - will integrate with real email service)
        self.send_wallet_creation_email(&request.email, &verification_link, &request.preferred_domain).await?;

        Ok(WalletEmailVerificationResponse {
            success: true,
            message: "Wallet creation verification email sent successfully".to_string(),
            request_id: Some(request_id),
            verification_link: Some(verification_link),
            expires_in_seconds: Some(3600),
        })
    }

    /// Verify wallet creation token and trigger wallet creation
    pub async fn verify_and_trigger_wallet_creation(
        &self,
        request: WalletCreationTriggerRequest,
    ) -> Result<WalletCreationTriggerResponse> {
        let mut storage = self.pending_requests.write().unwrap();
        
        if let Some(wallet_request) = storage.get_mut(&request.request_id) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            // Check if expired
            if now > wallet_request.expires_at {
                return Ok(WalletCreationTriggerResponse {
                    success: false,
                    message: "Verification link has expired".to_string(),
                    wallet_creation_url: None,
                    request_id: None,
                });
            }

            // Verify token
            if wallet_request.verification_token != request.verification_token {
                return Ok(WalletCreationTriggerResponse {
                    success: false,
                    message: "Invalid verification token".to_string(),
                    wallet_creation_url: None,
                    request_id: None,
                });
            }

            // Mark as verified and wallet creation initiated
            wallet_request.verified = true;
            wallet_request.wallet_creation_initiated = true;

            // Generate wallet creation URL
            let wallet_creation_url = format!(
                "https://wallet.bpi.global/setup?request_id={}&email={}",
                request.request_id,
                urlencoding::encode(&wallet_request.email)
            );

            Ok(WalletCreationTriggerResponse {
                success: true,
                message: "Email verified successfully. Redirecting to wallet creation.".to_string(),
                wallet_creation_url: Some(wallet_creation_url),
                request_id: Some(request.request_id.clone()),
            })
        } else {
            Ok(WalletCreationTriggerResponse {
                success: false,
                message: "Invalid or expired verification request".to_string(),
                wallet_creation_url: None,
                request_id: None,
            })
        }
    }

    /// Check if email verification is completed for a request
    pub fn is_email_verified(&self, request_id: &str) -> bool {
        let storage = self.pending_requests.read().unwrap();
        if let Some(request) = storage.get(request_id) {
            request.verified
        } else {
            false
        }
    }

    /// Get wallet creation request details
    pub fn get_wallet_request(&self, request_id: &str) -> Option<WalletCreationRequest> {
        let storage = self.pending_requests.read().unwrap();
        storage.get(request_id).cloned()
    }

    /// Clean up expired requests
    pub fn cleanup_expired_requests(&self) -> Result<u32> {
        let mut storage = self.pending_requests.write().unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let initial_count = storage.len();
        storage.retain(|_, request| now <= request.expires_at);
        let final_count = storage.len();

        Ok((initial_count - final_count) as u32)
    }

    /// Generate secure verification token
    fn generate_verification_token(&self) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..32)
            .map(|_| {
                let idx = rng.gen_range(0..62);
                match idx {
                    0..=25 => (b'a' + idx) as char,
                    26..=51 => (b'A' + (idx - 26)) as char,
                    _ => (b'0' + (idx - 52)) as char,
                }
            })
            .collect()
    }

    /// Validate email format (basic validation)
    fn is_valid_email(&self, email: &str) -> bool {
        email.contains('@') && email.contains('.') && email.len() > 5
    }

    /// Send wallet creation email (demo implementation)
    async fn send_wallet_creation_email(
        &self,
        email: &str,
        verification_link: &str,
        preferred_domain: &Option<String>,
    ) -> Result<()> {
        let domain_text = match preferred_domain {
            Some(domain) => format!(" for {} domain", domain),
            None => "".to_string(),
        };

        println!("📧 WALLET CREATION EMAIL SENT TO: {}", email);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Subject: Create Your Web 3.5 Wallet{}", domain_text);
        println!();
        println!("Welcome to the future of decentralized web!");
        println!();
        println!("You requested to create a Web 3.5 wallet{}", domain_text);
        println!("Click the link below to complete your wallet setup:");
        println!();
        println!("🔗 {}", verification_link);
        println!();
        println!("This link will expire in 1 hour for security.");
        println!();
        println!("Your Web 3.5 wallet will give you:");
        println!("• Email-like wallet address ({}@domain)", email.split('@').next().unwrap_or("user"));
        println!("• Universal login across all Web 3.5 apps");
        println!("• Complete control over your data and identity");
        println!("• Access to decentralized applications and services");
        println!();
        println!("Welcome to Web 3.5! 🚀");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_email_verification_flow() {
        let service = EmailVerificationService::new();

        // Test sending verification email
        let request = WalletEmailVerificationRequest {
            email: "test@example.com".to_string(),
            preferred_domain: Some("@global".to_string()),
        };

        let response = service.send_wallet_verification_email(request).await.unwrap();
        assert!(response.success);
        assert!(response.request_id.is_some());
        assert!(response.verification_link.is_some());

        let request_id = response.request_id.unwrap();
        let verification_link = response.verification_link.unwrap();
        
        // Extract token from verification link
        let url_parts: Vec<&str> = verification_link.split('&').collect();
        let token_part = url_parts.iter().find(|part| part.starts_with("token=")).unwrap();
        let token = token_part.split('=').nth(1).unwrap();

        // Test verification and wallet creation trigger
        let trigger_request = WalletCreationTriggerRequest {
            request_id: request_id.clone(),
            verification_token: token.to_string(),
        };

        let trigger_response = service.verify_and_trigger_wallet_creation(trigger_request).await.unwrap();
        assert!(trigger_response.success);
        assert!(trigger_response.wallet_creation_url.is_some());

        // Verify the request is marked as verified
        assert!(service.is_email_verified(&request_id));
    }

    #[tokio::test]
    async fn test_invalid_email() {
        let service = EmailVerificationService::new();

        let request = WalletEmailVerificationRequest {
            email: "invalid-email".to_string(),
            preferred_domain: None,
        };

        let response = service.send_wallet_verification_email(request).await.unwrap();
        assert!(!response.success);
        assert_eq!(response.message, "Invalid email address format");
    }

    #[tokio::test]
    async fn test_invalid_token() {
        let service = EmailVerificationService::new();

        // Send verification email first
        let request = WalletEmailVerificationRequest {
            email: "test@example.com".to_string(),
            preferred_domain: None,
        };

        let response = service.send_wallet_verification_email(request).await.unwrap();
        let request_id = response.request_id.unwrap();

        // Try with invalid token
        let trigger_request = WalletCreationTriggerRequest {
            request_id,
            verification_token: "invalid_token".to_string(),
        };

        let trigger_response = service.verify_and_trigger_wallet_creation(trigger_request).await.unwrap();
        assert!(!trigger_response.success);
        assert_eq!(trigger_response.message, "Invalid verification token");
    }
}
