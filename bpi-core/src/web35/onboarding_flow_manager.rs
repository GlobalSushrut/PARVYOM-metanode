use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::{
    WalletCreationTrigger, WalletCredentials, WalletProviderType, 
    VerificationLevel, WalletCapability, EmailVerificationService
};

/// Onboarding flow state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OnboardingState {
    EmailSubmitted,
    EmailVerified,
    WalletCreated,
    OnboardingCompleted,
    Failed(String),
}

/// Onboarding session data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingSession {
    pub session_id: String,
    pub email: String,
    pub provider_type: WalletProviderType,
    pub verification_level: VerificationLevel,
    pub state: OnboardingState,
    pub wallet_credentials: Option<WalletCredentials>,
    pub onboarding_token: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

/// Onboarding request from user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingRequest {
    pub email: String,
    pub provider_type: WalletProviderType,
    pub verification_level: VerificationLevel,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

/// Onboarding response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingResponse {
    pub success: bool,
    pub session_id: Option<String>,
    pub state: OnboardingState,
    pub message: String,
    pub next_step: Option<String>,
    pub wallet_address: Option<String>,
    pub onboarding_token: Option<String>,
}

/// Wallet setup completion data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletSetupData {
    pub wallet_name: Option<String>,
    pub recovery_email: Option<String>,
    pub backup_phrase_confirmed: bool,
    pub security_questions: Option<Vec<SecurityQuestion>>,
    pub two_factor_enabled: bool,
}

/// Security question for wallet recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityQuestion {
    pub question: String,
    pub answer_hash: String, // Hashed answer for security
}

/// Onboarding flow manager
pub struct OnboardingFlowManager {
    sessions: RwLock<HashMap<String, OnboardingSession>>,
    email_service: EmailVerificationService,
    wallet_trigger: WalletCreationTrigger,
    session_timeout_hours: u64,
}

impl OnboardingFlowManager {
    /// Create new onboarding flow manager
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            email_service: EmailVerificationService::new(),
            wallet_trigger: WalletCreationTrigger::new(),
            session_timeout_hours: 24, // 24 hour session timeout
        }
    }
    
    /// Start onboarding flow with email verification
    pub async fn start_onboarding(&self, request: OnboardingRequest) -> Result<OnboardingResponse> {
        // Validate email format
        if !self.validate_email(&request.email) {
            return Ok(OnboardingResponse {
                success: false,
                session_id: None,
                state: OnboardingState::Failed("Invalid email format".to_string()),
                message: "Please provide a valid email address".to_string(),
                next_step: None,
                wallet_address: None,
                onboarding_token: None,
            });
        }
        
        // Check if email already has an active session
        if let Some(existing_session) = self.find_active_session_by_email(&request.email).await {
            return Ok(OnboardingResponse {
                success: true,
                session_id: Some(existing_session.session_id.clone()),
                state: existing_session.state.clone(),
                message: "Existing onboarding session found".to_string(),
                next_step: Some(self.get_next_step(&existing_session.state)),
                wallet_address: existing_session.wallet_credentials.as_ref().map(|w| w.wallet_address.clone()),
                onboarding_token: existing_session.onboarding_token.clone(),
            });
        }
        
        // Create new onboarding session
        let session_id = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + chrono::Duration::hours(self.session_timeout_hours as i64);
        
        let session = OnboardingSession {
            session_id: session_id.clone(),
            email: request.email.clone(),
            provider_type: request.provider_type.clone(),
            verification_level: request.verification_level.clone(),
            state: OnboardingState::EmailSubmitted,
            wallet_credentials: None,
            onboarding_token: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            expires_at,
        };
        
        // Store session
        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), session);
        drop(sessions);
        
        // Send verification email
        let verification_request = super::WalletEmailVerificationRequest {
            email: request.email.clone(),
            preferred_domain: None, // Use default domain
        };
        
        match self.email_service.send_wallet_verification_email(verification_request).await {
            Ok(_) => {
                println!("✅ Started onboarding for {} with session {}", request.email, session_id);
                
                Ok(OnboardingResponse {
                    success: true,
                    session_id: Some(session_id),
                    state: OnboardingState::EmailSubmitted,
                    message: "Verification email sent successfully".to_string(),
                    next_step: Some("Check your email and click the verification link".to_string()),
                    wallet_address: None,
                    onboarding_token: None,
                })
            }
            Err(e) => {
                // Update session state to failed
                self.update_session_state(&session_id, OnboardingState::Failed(e.to_string())).await?;
                
                Ok(OnboardingResponse {
                    success: false,
                    session_id: Some(session_id),
                    state: OnboardingState::Failed(e.to_string()),
                    message: format!("Failed to send verification email: {}", e),
                    next_step: Some("Please try again or contact support".to_string()),
                    wallet_address: None,
                    onboarding_token: None,
                })
            }
        }
    }
    
    /// Process email verification completion
    pub async fn process_email_verification(&self, session_id: &str, verification_token: &str) -> Result<OnboardingResponse> {
        // Get session
        let session = self.get_session(session_id).await?;
        
        // Verify the email verification token using wallet creation trigger
        let trigger_request = super::WalletCreationTriggerRequest {
            request_id: session_id.to_string(), // Use session_id as request_id for now
            verification_token: verification_token.to_string(),
        };
        
        let verification_response = self.email_service.verify_and_trigger_wallet_creation(trigger_request).await?;
        
        if !verification_response.success {
            self.update_session_state(session_id, OnboardingState::Failed("Email verification failed".to_string())).await?;
            return Ok(OnboardingResponse {
                success: false,
                session_id: Some(session_id.to_string()),
                state: OnboardingState::Failed("Email verification failed".to_string()),
                message: "Email verification token is invalid or expired".to_string(),
                next_step: Some("Please request a new verification email".to_string()),
                wallet_address: None,
                onboarding_token: None,
            });
        }
        
        // Update session state to verified
        self.update_session_state(session_id, OnboardingState::EmailVerified).await?;
        
        // Trigger wallet creation
        let wallet_request = super::WalletCreationRequest {
            email: session.email.clone(),
            verification_token: verification_token.to_string(),
            provider_type: session.provider_type.clone(),
            verification_level: session.verification_level.clone(),
            timestamp: Utc::now(),
        };
        
        let request_id = self.wallet_trigger.queue_wallet_creation(wallet_request).await?;
        let wallet_result = self.wallet_trigger.process_wallet_creation(&request_id).await?;
        
        if wallet_result.success {
            let credentials = wallet_result.wallet_credentials.unwrap();
            let onboarding_token = wallet_result.onboarding_token.unwrap();
            
            // Update session with wallet credentials
            self.update_session_with_wallet(session_id, credentials.clone(), onboarding_token.clone()).await?;
            
            println!("✅ Wallet created for session {}: {}", session_id, credentials.wallet_address);
            
            Ok(OnboardingResponse {
                success: true,
                session_id: Some(session_id.to_string()),
                state: OnboardingState::WalletCreated,
                message: "Wallet created successfully".to_string(),
                next_step: Some("Complete wallet setup and backup your recovery phrase".to_string()),
                wallet_address: Some(credentials.wallet_address),
                onboarding_token: Some(onboarding_token),
            })
        } else {
            let error_msg = wallet_result.error_message.unwrap_or("Unknown wallet creation error".to_string());
            self.update_session_state(session_id, OnboardingState::Failed(error_msg.clone())).await?;
            
            Ok(OnboardingResponse {
                success: false,
                session_id: Some(session_id.to_string()),
                state: OnboardingState::Failed(error_msg.clone()),
                message: format!("Wallet creation failed: {}", error_msg),
                next_step: Some("Please try again or contact support".to_string()),
                wallet_address: None,
                onboarding_token: None,
            })
        }
    }
    
    /// Complete onboarding with wallet setup
    pub async fn complete_onboarding(&self, session_id: &str, setup_data: WalletSetupData) -> Result<OnboardingResponse> {
        let session = self.get_session(session_id).await?;
        
        // Validate session state
        if !matches!(session.state, OnboardingState::WalletCreated) {
            return Ok(OnboardingResponse {
                success: false,
                session_id: Some(session_id.to_string()),
                state: session.state.clone(),
                message: "Invalid session state for completion".to_string(),
                next_step: Some(self.get_next_step(&session.state)),
                wallet_address: session.wallet_credentials.as_ref().map(|w| w.wallet_address.clone()),
                onboarding_token: session.onboarding_token.clone(),
            });
        }
        
        // Validate setup data
        if !setup_data.backup_phrase_confirmed {
            return Ok(OnboardingResponse {
                success: false,
                session_id: Some(session_id.to_string()),
                state: OnboardingState::WalletCreated,
                message: "Please confirm you have backed up your recovery phrase".to_string(),
                next_step: Some("Backup your recovery phrase and confirm".to_string()),
                wallet_address: session.wallet_credentials.as_ref().map(|w| w.wallet_address.clone()),
                onboarding_token: session.onboarding_token.clone(),
            });
        }
        
        // Complete onboarding
        self.update_session_state(session_id, OnboardingState::OnboardingCompleted).await?;
        
        println!("✅ Onboarding completed for session {}", session_id);
        
        Ok(OnboardingResponse {
            success: true,
            session_id: Some(session_id.to_string()),
            state: OnboardingState::OnboardingCompleted,
            message: "Onboarding completed successfully! Welcome to Web 3.5".to_string(),
            next_step: Some("You can now use your wallet to access Web 3.5 services".to_string()),
            wallet_address: session.wallet_credentials.as_ref().map(|w| w.wallet_address.clone()),
            onboarding_token: session.onboarding_token.clone(),
        })
    }
    
    /// Get onboarding session status
    pub async fn get_session_status(&self, session_id: &str) -> Result<OnboardingResponse> {
        let session = self.get_session(session_id).await?;
        
        Ok(OnboardingResponse {
            success: true,
            session_id: Some(session_id.to_string()),
            state: session.state.clone(),
            message: format!("Session status: {:?}", session.state),
            next_step: Some(self.get_next_step(&session.state)),
            wallet_address: session.wallet_credentials.as_ref().map(|w| w.wallet_address.clone()),
            onboarding_token: session.onboarding_token.clone(),
        })
    }
    
    /// Get wallet capabilities for provider
    pub fn get_wallet_capabilities(&self, provider: &WalletProviderType) -> Option<&Vec<WalletCapability>> {
        self.wallet_trigger.get_provider_capabilities(provider)
    }
    
    /// Validate email format
    fn validate_email(&self, email: &str) -> bool {
        // Basic email validation
        email.contains('@') && email.contains('.') && email.len() > 5 && email.len() < 255
    }
    
    /// Find active session by email
    async fn find_active_session_by_email(&self, email: &str) -> Option<OnboardingSession> {
        let sessions = self.sessions.read().await;
        for session in sessions.values() {
            if session.email == email && session.expires_at > Utc::now() {
                return Some(session.clone());
            }
        }
        None
    }
    
    /// Get session by ID
    async fn get_session(&self, session_id: &str) -> Result<OnboardingSession> {
        let sessions = self.sessions.read().await;
        let session = sessions.get(session_id)
            .ok_or_else(|| anyhow!("Onboarding session not found: {}", session_id))?;
        
        // Check if session is expired
        if session.expires_at <= Utc::now() {
            return Err(anyhow!("Onboarding session expired: {}", session_id));
        }
        
        Ok(session.clone())
    }
    
    /// Update session state
    async fn update_session_state(&self, session_id: &str, new_state: OnboardingState) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.state = new_state;
            session.updated_at = Utc::now();
        } else {
            return Err(anyhow!("Session not found for state update: {}", session_id));
        }
        Ok(())
    }
    
    /// Update session with wallet credentials
    async fn update_session_with_wallet(&self, session_id: &str, credentials: WalletCredentials, onboarding_token: String) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.wallet_credentials = Some(credentials);
            session.onboarding_token = Some(onboarding_token);
            session.state = OnboardingState::WalletCreated;
            session.updated_at = Utc::now();
        } else {
            return Err(anyhow!("Session not found for wallet update: {}", session_id));
        }
        Ok(())
    }
    
    /// Get next step message for current state
    fn get_next_step(&self, state: &OnboardingState) -> String {
        match state {
            OnboardingState::EmailSubmitted => "Check your email and click the verification link".to_string(),
            OnboardingState::EmailVerified => "Wallet creation in progress...".to_string(),
            OnboardingState::WalletCreated => "Complete wallet setup and backup your recovery phrase".to_string(),
            OnboardingState::OnboardingCompleted => "You can now use your wallet to access Web 3.5 services".to_string(),
            OnboardingState::Failed(error) => format!("Error occurred: {}. Please try again or contact support", error),
        }
    }
    
    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&self) -> Result<usize> {
        let mut sessions = self.sessions.write().await;
        let now = Utc::now();
        let initial_count = sessions.len();
        
        sessions.retain(|_, session| session.expires_at > now);
        
        let removed_count = initial_count - sessions.len();
        if removed_count > 0 {
            println!("🧹 Cleaned up {} expired onboarding sessions", removed_count);
        }
        
        Ok(removed_count)
    }
    
    /// Get active sessions count
    pub async fn get_active_sessions_count(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions.len()
    }
    
    /// Get sessions by state
    pub async fn get_sessions_by_state(&self, state: &OnboardingState) -> Vec<OnboardingSession> {
        let sessions = self.sessions.read().await;
        sessions.values()
            .filter(|session| std::mem::discriminant(&session.state) == std::mem::discriminant(state))
            .cloned()
            .collect()
    }
}

impl Default for OnboardingFlowManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_onboarding_flow_start() {
        let manager = OnboardingFlowManager::new();
        
        let request = OnboardingRequest {
            email: "test@example.com".to_string(),
            provider_type: WalletProviderType::Pravyom,
            verification_level: VerificationLevel::Basic,
            user_agent: Some("Test Browser".to_string()),
            ip_address: Some("127.0.0.1".to_string()),
        };
        
        let response = manager.start_onboarding(request).await.unwrap();
        
        assert!(response.success);
        assert!(response.session_id.is_some());
        assert!(matches!(response.state, OnboardingState::EmailSubmitted));
        assert_eq!(response.message, "Verification email sent successfully");
        
        // Verify session was created
        assert_eq!(manager.get_active_sessions_count().await, 1);
    }
    
    #[tokio::test]
    async fn test_invalid_email() {
        let manager = OnboardingFlowManager::new();
        
        let request = OnboardingRequest {
            email: "invalid-email".to_string(),
            provider_type: WalletProviderType::Pravyom,
            verification_level: VerificationLevel::Basic,
            user_agent: None,
            ip_address: None,
        };
        
        let response = manager.start_onboarding(request).await.unwrap();
        
        assert!(!response.success);
        assert!(matches!(response.state, OnboardingState::Failed(_)));
        assert_eq!(response.message, "Please provide a valid email address");
    }
    
    #[tokio::test]
    async fn test_session_cleanup() {
        let mut manager = OnboardingFlowManager::new();
        manager.session_timeout_hours = 0; // Immediate expiry for testing
        
        let request = OnboardingRequest {
            email: "test@example.com".to_string(),
            provider_type: WalletProviderType::Pravyom,
            verification_level: VerificationLevel::Basic,
            user_agent: None,
            ip_address: None,
        };
        
        let _response = manager.start_onboarding(request).await.unwrap();
        assert_eq!(manager.get_active_sessions_count().await, 1);
        
        // Wait a moment for expiry
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        let removed = manager.cleanup_expired_sessions().await.unwrap();
        assert_eq!(removed, 1);
        assert_eq!(manager.get_active_sessions_count().await, 0);
    }
    
    #[tokio::test]
    async fn test_wallet_capabilities() {
        let manager = OnboardingFlowManager::new();
        
        let pravyom_caps = manager.get_wallet_capabilities(&WalletProviderType::Pravyom);
        assert!(pravyom_caps.is_some());
        assert!(pravyom_caps.unwrap().contains(&WalletCapability::EncryptedMessaging));
        
        let bank_caps = manager.get_wallet_capabilities(&WalletProviderType::Bank);
        assert!(bank_caps.is_some());
        assert!(bank_caps.unwrap().contains(&WalletCapability::BankingServices));
    }
}
