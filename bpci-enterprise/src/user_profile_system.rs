use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc, Duration};
use tracing::{info, warn, error, debug};

/// Verify password against hash
fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}

use rust_decimal::Decimal;

use crate::autonomous_economy::bpci_economic_integration::{RealBpciEconomicIntegration, RealWalletBalance};
use crate::mining::wallet_registry_bridge::{WalletRegistryMiningBridge, RegisteredWallet, WalletStatus};
use crate::registry::{BpciRegistry, NodeRegistration, NodeType, BpiWalletStamp};
use crate::email_otp_service::EmailOtpService;
use crypto_primitives::Ed25519KeyPair;

/// Scalable User Profile System for BPCI Enterprise
/// Designed to handle millions of users with efficient storage and retrieval
#[derive(Debug)]
pub struct UserProfileSystem {
    /// User profiles storage (scalable for millions of users)
    profiles: Arc<RwLock<HashMap<String, UserProfile>>>,
    /// User sessions for active users
    sessions: Arc<RwLock<HashMap<String, UserSession>>>,
    /// Email to user ID mapping for fast lookups
    email_index: Arc<RwLock<HashMap<String, String>>>,
    /// Wallet system integration
    wallet_system: Arc<RwLock<UserWalletSystem>>,
    /// Economic integration for real BPI operations
    economic_integration: Option<Arc<RealBpciEconomicIntegration>>,
    /// Registry integration for node operations
    registry: Arc<RwLock<BpciRegistry>>,
    /// Configuration
    config: UserProfileConfig,
}

/// User profile configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileConfig {
    /// Maximum number of wallets per user
    pub max_wallets_per_user: u32,
    /// Session timeout in seconds
    pub session_timeout_seconds: u64,
    /// Profile completion requirements
    pub require_profile_completion: bool,
    /// Enable demo accounts
    pub enable_demo_accounts: bool,
    /// Demo account email pattern
    pub demo_email_pattern: String,
}

impl Default for UserProfileConfig {
    fn default() -> Self {
        Self {
            max_wallets_per_user: 10,
            session_timeout_seconds: 86400, // 24 hours
            require_profile_completion: true,
            enable_demo_accounts: true,
            demo_email_pattern: "demo@bpci.local".to_string(),
        }
    }
}

/// Comprehensive user profile for BPCI Enterprise users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// Unique user identifier
    pub user_id: String,
    /// Email address (unique)
    pub email: String,
    /// Full name
    pub name: String,
    /// Password hash (secure storage)
    pub password_hash: String,
    /// Profile completion status
    pub profile_complete: bool,
    /// Account verification status
    pub email_verified: bool,
    /// Account creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last login timestamp
    pub last_login: Option<DateTime<Utc>>,
    /// Account status
    pub status: UserStatus,
    /// Associated wallet IDs
    pub wallet_ids: Vec<String>,
    /// User preferences
    pub preferences: UserPreferences,
    /// KYC/Identity verification status
    pub kyc_status: KycStatus,
    /// Demo account flag
    pub is_demo_account: bool,
    /// User tier (affects wallet limits and features)
    pub user_tier: UserTier,
    /// Profile metadata
    pub metadata: HashMap<String, String>,
}

/// User session for authentication and tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    /// Session identifier
    pub session_id: String,
    /// User identifier
    pub user_id: String,
    /// Session token for API access
    pub session_token: String,
    /// Session creation time
    pub created_at: DateTime<Utc>,
    /// Last activity time
    pub last_activity: DateTime<Utc>,
    /// Session expiration time
    pub expires_at: DateTime<Utc>,
    /// Session status
    pub status: SessionStatus,
    /// Session active flag
    pub is_active: bool,
    /// IP address
    pub ip_address: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
}

/// User account status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
    Banned,
    PendingVerification,
}

/// Session status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionStatus {
    Active,
    Expired,
    Revoked,
}

/// User preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    /// Preferred language
    pub language: String,
    /// Timezone
    pub timezone: String,
    /// Email notifications enabled
    pub email_notifications: bool,
    /// Two-factor authentication enabled
    pub two_factor_enabled: bool,
    /// Theme preference
    pub theme: String,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            timezone: "UTC".to_string(),
            email_notifications: true,
            two_factor_enabled: false,
            theme: "light".to_string(),
        }
    }
}

/// KYC/Identity verification status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KycStatus {
    NotStarted,
    InProgress,
    Completed,
    Rejected,
    Expired,
}

/// User tier affecting features and limits
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserTier {
    Basic,
    Premium,
    Enterprise,
    Demo,
}

/// Scalable User Wallet System
/// Integrates with BPI economic system and wallet registry
#[derive(Debug)]
pub struct UserWalletSystem {
    /// User wallets mapping
    user_wallets: HashMap<String, Vec<UserWallet>>,
    /// Wallet registry bridge for BPI integration
    wallet_bridge: Option<Arc<WalletRegistryMiningBridge>>,
    /// Economic integration for balance tracking
    economic_integration: Option<Arc<RealBpciEconomicIntegration>>,
}

/// User wallet with BPI integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWallet {
    /// Wallet ID
    pub wallet_id: String,
    /// User ID (owner)
    pub user_id: String,
    /// Wallet name/label
    pub wallet_name: String,
    /// Wallet type
    pub wallet_type: UserWalletType,
    /// BPI wallet address
    pub bpi_address: String,
    /// Public key
    pub public_key: String,
    /// Wallet status
    pub status: WalletStatus,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Activation timestamp
    pub activated_at: Option<DateTime<Utc>>,
    /// Demo wallet flag
    pub is_demo_wallet: bool,
    /// Wallet stamp for BPI operations
    pub wallet_stamp: Option<BpiWalletStamp>,
    /// Current balance (cached)
    pub cached_balance: Option<RealWalletBalance>,
    /// Last balance update
    pub balance_updated_at: Option<DateTime<Utc>>,
}

/// User wallet types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserWalletType {
    Personal,
    Business,
    Demo,
    Enterprise,
}

impl UserProfileSystem {
    /// Create new user profile system
    pub fn new(
        config: UserProfileConfig,
        economic_integration: Option<Arc<RealBpciEconomicIntegration>>,
        registry: Arc<RwLock<BpciRegistry>>,
    ) -> Self {
        Self {
            profiles: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            email_index: Arc::new(RwLock::new(HashMap::new())),
            wallet_system: Arc::new(RwLock::new(UserWalletSystem {
                user_wallets: HashMap::new(),
                wallet_bridge: None,
                economic_integration: economic_integration.clone(),
            })),
            economic_integration,
            registry,
            config,
        }
    }

    /// Create new user profile (registration)
    pub async fn create_user_profile(
        &self,
        email: String,
        password: String,
        name: String,
    ) -> Result<UserProfile> {
        // Check if email already exists
        let email_index = self.email_index.read().await;
        if email_index.contains_key(&email) {
            return Err(anyhow!("Email already registered"));
        }
        drop(email_index);

        // Check if this is a demo account
        let is_demo = email == self.config.demo_email_pattern;

        // Generate user ID
        let user_id = Uuid::new_v4().to_string();

        // Hash password (in production, use proper password hashing)
        let password_hash = format!("hashed_{}", password); // TODO: Use bcrypt or similar

        // Create user profile
        let profile = UserProfile {
            user_id: user_id.clone(),
            email: email.clone(),
            name,
            password_hash,
            profile_complete: is_demo, // Demo accounts are pre-completed
            email_verified: is_demo,   // Demo accounts are pre-verified
            created_at: Utc::now(),
            last_login: None,
            status: if is_demo { UserStatus::Active } else { UserStatus::PendingVerification },
            wallet_ids: Vec::new(),
            preferences: UserPreferences::default(),
            kyc_status: if is_demo { KycStatus::Completed } else { KycStatus::NotStarted },
            is_demo_account: is_demo,
            user_tier: if is_demo { UserTier::Demo } else { UserTier::Basic },
            metadata: HashMap::new(),
        };

        // Store profile
        let mut profiles = self.profiles.write().await;
        profiles.insert(user_id.clone(), profile.clone());
        drop(profiles);

        // Update email index
        let mut email_index = self.email_index.write().await;
        email_index.insert(email, user_id.clone());
        drop(email_index);

        // Create default wallet for the user
        if is_demo {
            self.create_demo_wallet(&user_id).await?;
        }

        info!("Created user profile: {} ({})", profile.name, profile.email);
        Ok(profile)
    }

    /// Authenticate user and create session
    pub async fn authenticate_user(
        &self,
        email: String,
        password: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<UserSession> {
        // Find user by email
        let email_index = self.email_index.read().await;
        let user_id = email_index.get(&email)
            .ok_or_else(|| anyhow!("Invalid email or password"))?
            .clone();
        drop(email_index);

        // Get user profile
        let mut profiles = self.profiles.write().await;
        let profile = profiles.get_mut(&user_id)
            .ok_or_else(|| anyhow!("User not found"))?;

        // Verify password (in production, use proper password verification)
        let expected_hash = format!("hashed_{}", password);
        if profile.password_hash != expected_hash {
            return Err(anyhow!("Invalid email or password"));
        }

        // Check user status
        if profile.status != UserStatus::Active && !profile.is_demo_account {
            return Err(anyhow!("Account not active"));
        }

        // Update last login
        let (email, session_id) = {
            let mut profiles = self.profiles.write().await;
            let profile = profiles.get_mut(&user_id)
                .ok_or_else(|| anyhow!("User profile not found"))?;

            // Verify password
            if !verify_password(&password, &profile.password_hash) {
                return Err(anyhow!("Invalid password"));
            }

            // Update last login
            profile.last_login = Some(Utc::now());
            let email = profile.email.clone();

            // Create session
            let session_id = format!("session_{}", Utc::now().timestamp());
            let session = UserSession {
                session_id: session_id.clone(),
                user_id: user_id.clone(),
                session_token: session_id.clone(),
                created_at: Utc::now(),
                last_activity: Utc::now(),
                expires_at: Utc::now() + Duration::hours(24),
                status: SessionStatus::Active,
                is_active: true,
                ip_address: None,
                user_agent: None,
            };

            // Store session
            let mut sessions = self.sessions.write().await;
            sessions.insert(session_id.clone(), session);
            
            (email, session_id)
        };

        info!("User authenticated: {} ({})", email, user_id);
        Ok(UserSession {
            session_id: session_id.clone(),
            user_id,
            session_token: session_id,
            created_at: Utc::now(),
            last_activity: Utc::now(),
            expires_at: Utc::now() + Duration::hours(24),
            status: SessionStatus::Active,
            is_active: true,
            ip_address,
            user_agent,
        })
    }

    /// Create demo wallet for demo users
    async fn create_demo_wallet(&self, user_id: &str) -> Result<UserWallet> {
        // Generate demo wallet
        let wallet_id = format!("demo_wallet_{}", Uuid::new_v4());
        let keypair = Ed25519KeyPair::generate();
        
        let demo_wallet = UserWallet {
            wallet_id: wallet_id.clone(),
            user_id: user_id.to_string(),
            wallet_name: "Demo Wallet".to_string(),
            wallet_type: UserWalletType::Demo,
            bpi_address: hex::encode(keypair.public_key_bytes()),
            public_key: hex::encode(keypair.public_key_bytes()),
            status: WalletStatus::Active,
            created_at: Utc::now(),
            activated_at: Some(Utc::now()),
            is_demo_wallet: true,
            wallet_stamp: Some(BpiWalletStamp::Demo {
                test_mode: true,
                limited_features: true,
            }),
            cached_balance: None,
            balance_updated_at: None,
        };

        // Store wallet
        let mut wallet_system = self.wallet_system.write().await;
        wallet_system.user_wallets
            .entry(user_id.to_string())
            .or_insert_with(Vec::new)
            .push(demo_wallet.clone());
        drop(wallet_system);

        // Update user profile with wallet ID
        let mut profiles = self.profiles.write().await;
        if let Some(profile) = profiles.get_mut(user_id) {
            profile.wallet_ids.push(wallet_id.clone());
        }
        drop(profiles);

        info!("Created demo wallet: {} for user: {}", wallet_id, user_id);
        Ok(demo_wallet)
    }

    /// Get user profile by ID
    pub async fn get_user_profile(&self, user_id: &str) -> Result<Option<UserProfile>> {
        let profiles = self.profiles.read().await;
        Ok(profiles.get(user_id).cloned())
    }

    /// Get user wallets
    pub async fn get_user_wallets(&self, user_id: &str) -> Result<Vec<UserWallet>> {
        let wallet_system = self.wallet_system.read().await;
        Ok(wallet_system.user_wallets.get(user_id).cloned().unwrap_or_default())
    }

    /// Validate session token
    pub async fn validate_session(&self, session_token: &str) -> Result<Option<UserSession>> {
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.get_mut(session_token) {
            // Check if session is expired
            if session.expires_at < Utc::now() {
                session.status = SessionStatus::Expired;
                return Ok(None);
            }

            // Update last activity
            session.last_activity = Utc::now();
            Ok(Some(session.clone()))
        } else {
            Ok(None)
        }
    }

    /// Get user statistics (for admin dashboard)
    pub async fn get_user_statistics(&self) -> Result<UserStatistics> {
        let profiles = self.profiles.read().await;
        let sessions = self.sessions.read().await;
        let wallet_system = self.wallet_system.read().await;

        let total_users = profiles.len();
        let active_users = profiles.values()
            .filter(|p| p.status == UserStatus::Active)
            .count();
        let demo_users = profiles.values()
            .filter(|p| p.is_demo_account)
            .count();
        let verified_users = profiles.values()
            .filter(|p| p.email_verified)
            .count();
        let active_sessions = sessions.values()
            .filter(|s| s.status == SessionStatus::Active)
            .count();
        let total_wallets = wallet_system.user_wallets.values()
            .map(|wallets| wallets.len())
            .sum();

        Ok(UserStatistics {
            total_users,
            active_users,
            demo_users,
            verified_users,
            active_sessions,
            total_wallets,
        })
    }
}

/// User statistics for admin dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatistics {
    pub total_users: usize,
    pub active_users: usize,
    pub demo_users: usize,
    pub verified_users: usize,
    pub active_sessions: usize,
    pub total_wallets: usize,
}

/// Global user profile system instance
static GLOBAL_USER_PROFILE_SYSTEM: std::sync::OnceLock<Arc<UserProfileSystem>> = std::sync::OnceLock::new();

/// Initialize global user profile system
pub fn initialize_global_user_profile_system(
    config: UserProfileConfig,
    economic_integration: Option<Arc<RealBpciEconomicIntegration>>,
    registry: Arc<RwLock<BpciRegistry>>,
) -> Arc<UserProfileSystem> {
    GLOBAL_USER_PROFILE_SYSTEM.get_or_init(|| {
        Arc::new(UserProfileSystem::new(config, economic_integration, registry))
    }).clone()
}

/// Get global user profile system
pub fn get_global_user_profile_system() -> Option<Arc<UserProfileSystem>> {
    GLOBAL_USER_PROFILE_SYSTEM.get().cloned()
}
