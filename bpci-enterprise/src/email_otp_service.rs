use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use tokio::time::sleep;
use rand::Rng;

/// Email OTP Service for user registration verification
/// Handles OTP generation, storage, verification, and email sending
#[derive(Debug, Clone)]
pub struct EmailOtpService {
    /// In-memory OTP storage (in production, use Redis or database)
    otp_storage: Arc<RwLock<HashMap<String, OtpEntry>>>,
    /// Email service configuration
    email_config: EmailConfig,
}

#[derive(Debug, Clone)]
struct OtpEntry {
    otp_code: String,
    email: String,
    created_at: u64,
    expires_at: u64,
    attempts: u32,
    verified: bool,
}

#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
    pub from_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtpRequest {
    pub email: String,
    pub purpose: OtpPurpose,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OtpPurpose {
    Registration,
    PasswordReset,
    EmailChange,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtpVerificationRequest {
    pub email: String,
    pub otp_code: String,
    pub purpose: OtpPurpose,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtpResponse {
    pub success: bool,
    pub message: String,
    pub expires_in_seconds: Option<u64>,
    pub attempts_remaining: Option<u32>,
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            smtp_server: "smtp.gmail.com".to_string(),
            smtp_port: 587,
            smtp_username: "noreply@bpci.local".to_string(),
            smtp_password: "demo_password".to_string(),
            from_email: "noreply@bpci.local".to_string(),
            from_name: "BPCI Enterprise".to_string(),
        }
    }
}

impl EmailOtpService {
    /// Create new email OTP service
    pub fn new(email_config: Option<EmailConfig>) -> Self {
        Self {
            otp_storage: Arc::new(RwLock::new(HashMap::new())),
            email_config: email_config.unwrap_or_default(),
        }
    }

    /// Generate and send OTP to email
    pub async fn generate_and_send_otp(&self, request: OtpRequest) -> Result<OtpResponse> {
        // Validate email format
        if !self.is_valid_email(&request.email) {
            return Ok(OtpResponse {
                success: false,
                message: "Invalid email address format".to_string(),
                expires_in_seconds: None,
                attempts_remaining: None,
            });
        }

        // Check rate limiting (max 3 OTPs per email per hour)
        if self.is_rate_limited(&request.email).await? {
            return Ok(OtpResponse {
                success: false,
                message: "Too many OTP requests. Please wait before requesting again.".to_string(),
                expires_in_seconds: None,
                attempts_remaining: None,
            });
        }

        // Generate 6-digit OTP
        let otp_code = self.generate_otp_code();
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let expires_at = now + 300; // 5 minutes expiry

        // Store OTP
        let otp_entry = OtpEntry {
            otp_code: otp_code.clone(),
            email: request.email.clone(),
            created_at: now,
            expires_at,
            attempts: 0,
            verified: false,
        };

        {
            let mut storage = self.otp_storage.write().unwrap();
            let key = format!("{}:{:?}", request.email, request.purpose);
            storage.insert(key, otp_entry);
        }

        // Send email (in demo mode, just log the OTP)
        self.send_otp_email(&request.email, &otp_code, &request.purpose).await?;

        Ok(OtpResponse {
            success: true,
            message: "OTP sent successfully to your email address".to_string(),
            expires_in_seconds: Some(300),
            attempts_remaining: Some(3),
        })
    }

    /// Verify OTP code
    pub async fn verify_otp(&self, request: OtpVerificationRequest) -> Result<OtpResponse> {
        let key = format!("{}:{:?}", request.email, request.purpose);
        
        let mut storage = self.otp_storage.write().unwrap();
        
        if let Some(otp_entry) = storage.get_mut(&key) {
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            
            // Check if OTP has expired
            if now > otp_entry.expires_at {
                storage.remove(&key);
                return Ok(OtpResponse {
                    success: false,
                    message: "OTP has expired. Please request a new one.".to_string(),
                    expires_in_seconds: None,
                    attempts_remaining: None,
                });
            }

            // Check if already verified
            if otp_entry.verified {
                return Ok(OtpResponse {
                    success: false,
                    message: "OTP has already been used.".to_string(),
                    expires_in_seconds: None,
                    attempts_remaining: None,
                });
            }

            // Check attempts limit
            if otp_entry.attempts >= 3 {
                storage.remove(&key);
                return Ok(OtpResponse {
                    success: false,
                    message: "Too many failed attempts. Please request a new OTP.".to_string(),
                    expires_in_seconds: None,
                    attempts_remaining: Some(0),
                });
            }

            // Verify OTP code
            if otp_entry.otp_code == request.otp_code {
                otp_entry.verified = true;
                return Ok(OtpResponse {
                    success: true,
                    message: "OTP verified successfully".to_string(),
                    expires_in_seconds: Some(otp_entry.expires_at - now),
                    attempts_remaining: Some(3 - otp_entry.attempts),
                });
            } else {
                otp_entry.attempts += 1;
                let remaining = 3 - otp_entry.attempts;
                return Ok(OtpResponse {
                    success: false,
                    message: format!("Invalid OTP code. {} attempts remaining.", remaining),
                    expires_in_seconds: Some(otp_entry.expires_at - now),
                    attempts_remaining: Some(remaining),
                });
            }
        }

        Ok(OtpResponse {
            success: false,
            message: "No OTP found for this email. Please request a new one.".to_string(),
            expires_in_seconds: None,
            attempts_remaining: None,
        })
    }

    /// Check if email has verified OTP for given purpose
    pub async fn is_otp_verified(&self, email: &str, purpose: &OtpPurpose) -> bool {
        let key = format!("{}:{:?}", email, purpose);
        let storage = self.otp_storage.read().unwrap();
        
        if let Some(otp_entry) = storage.get(&key) {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
            otp_entry.verified && now <= otp_entry.expires_at
        } else {
            false
        }
    }

    /// Clean up expired OTPs (should be called periodically)
    pub async fn cleanup_expired_otps(&self) -> Result<u32> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let mut storage = self.otp_storage.write().unwrap();
        
        let initial_count = storage.len();
        storage.retain(|_, otp_entry| now <= otp_entry.expires_at);
        let cleaned_count = initial_count - storage.len();
        
        Ok(cleaned_count as u32)
    }

    /// Generate 6-digit OTP code
    fn generate_otp_code(&self) -> String {
        let mut rng = rand::thread_rng();
        format!("{:06}", rng.gen_range(100000..999999))
    }

    /// Validate email format
    fn is_valid_email(&self, email: &str) -> bool {
        email.contains('@') && email.contains('.') && email.len() > 5
    }

    /// Check if email is rate limited
    async fn is_rate_limited(&self, email: &str) -> Result<bool> {
        let storage = self.otp_storage.read().unwrap();
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let one_hour_ago = now - 3600;
        
        let recent_requests = storage.values()
            .filter(|entry| entry.email == email && entry.created_at > one_hour_ago)
            .count();
        
        Ok(recent_requests >= 3)
    }

    /// Send OTP email (demo implementation - logs to console)
    async fn send_otp_email(&self, email: &str, otp_code: &str, purpose: &OtpPurpose) -> Result<()> {
        let subject = match purpose {
            OtpPurpose::Registration => "BPCI Enterprise - Email Verification",
            OtpPurpose::PasswordReset => "BPCI Enterprise - Password Reset",
            OtpPurpose::EmailChange => "BPCI Enterprise - Email Change Verification",
        };

        let body = format!(
            r#"
            <html>
            <body style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
                <div style="background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%); color: white; padding: 30px; border-radius: 10px; text-align: center;">
                    <h1 style="margin: 0; font-size: 28px;">🚀 BPCI Enterprise</h1>
                    <p style="margin: 10px 0 0 0; opacity: 0.8;">Blockchain Protocol Infrastructure</p>
                </div>
                
                <div style="padding: 30px; background: #f8fafc; border-radius: 10px; margin-top: 20px;">
                    <h2 style="color: #1e293b; margin-top: 0;">Email Verification Required</h2>
                    <p style="color: #475569; font-size: 16px; line-height: 1.6;">
                        Your verification code for {} is:
                    </p>
                    
                    <div style="background: white; border: 2px solid #e2e8f0; border-radius: 8px; padding: 20px; text-align: center; margin: 20px 0;">
                        <div style="font-size: 32px; font-weight: bold; color: #0f172a; letter-spacing: 8px; font-family: 'Courier New', monospace;">
                            {}
                        </div>
                    </div>
                    
                    <p style="color: #64748b; font-size: 14px; margin-bottom: 0;">
                        This code will expire in 5 minutes. If you didn't request this verification, please ignore this email.
                    </p>
                </div>
                
                <div style="text-align: center; margin-top: 20px; color: #94a3b8; font-size: 12px;">
                    <p>BPCI Enterprise - Military-grade blockchain infrastructure</p>
                </div>
            </body>
            </html>
            "#,
            match purpose {
                OtpPurpose::Registration => "account registration",
                OtpPurpose::PasswordReset => "password reset",
                OtpPurpose::EmailChange => "email change",
            },
            otp_code
        );

        // In demo mode, log the email content
        println!("\n📧 EMAIL SENT TO: {}", email);
        println!("📧 SUBJECT: {}", subject);
        println!("📧 OTP CODE: {}", otp_code);
        println!("📧 PURPOSE: {:?}", purpose);
        println!("📧 (In production, this would be sent via SMTP)\n");

        // In production, implement actual SMTP sending here
        // Example with lettre crate:
        // let email = Message::builder()
        //     .from(self.email_config.from_email.parse()?)
        //     .to(email.parse()?)
        //     .subject(subject)
        //     .multipart(MultiPart::alternative_plain_html(
        //         format!("Your verification code is: {}", otp_code),
        //         body
        //     ))?;
        // 
        // let creds = Credentials::new(
        //     self.email_config.smtp_username.clone(),
        //     self.email_config.smtp_password.clone()
        // );
        // 
        // let mailer = SmtpTransport::relay(&self.email_config.smtp_server)?
        //     .credentials(creds)
        //     .build();
        // 
        // mailer.send(&email)?;

        Ok(())
    }

    /// Start background cleanup task
    pub async fn start_cleanup_task(service: Arc<EmailOtpService>) {
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(300)).await; // Clean every 5 minutes
                if let Ok(cleaned) = service.cleanup_expired_otps().await {
                    if cleaned > 0 {
                        println!("🧹 Cleaned up {} expired OTP entries", cleaned);
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_otp_generation_and_verification() {
        let service = EmailOtpService::new(None);
        
        let request = OtpRequest {
            email: "test@example.com".to_string(),
            purpose: OtpPurpose::Registration,
        };

        // Generate OTP
        let response = service.generate_and_send_otp(request).await.unwrap();
        assert!(response.success);

        // Get the generated OTP from storage for testing
        let key = "test@example.com:Registration".to_string();
        let storage = service.otp_storage.read().unwrap();
        let otp_code = storage.get(&key).unwrap().otp_code.clone();
        drop(storage);

        // Verify correct OTP
        let verify_request = OtpVerificationRequest {
            email: "test@example.com".to_string(),
            otp_code,
            purpose: OtpPurpose::Registration,
        };

        let verify_response = service.verify_otp(verify_request).await.unwrap();
        assert!(verify_response.success);

        // Check if OTP is verified
        assert!(service.is_otp_verified("test@example.com", &OtpPurpose::Registration).await);
    }

    #[tokio::test]
    async fn test_invalid_otp_verification() {
        let service = EmailOtpService::new(None);
        
        let request = OtpRequest {
            email: "test@example.com".to_string(),
            purpose: OtpPurpose::Registration,
        };

        service.generate_and_send_otp(request).await.unwrap();

        // Try with wrong OTP
        let verify_request = OtpVerificationRequest {
            email: "test@example.com".to_string(),
            otp_code: "000000".to_string(),
            purpose: OtpPurpose::Registration,
        };

        let verify_response = service.verify_otp(verify_request).await.unwrap();
        assert!(!verify_response.success);
        assert!(verify_response.message.contains("Invalid OTP code"));
    }
}
