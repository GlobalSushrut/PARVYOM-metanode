//! Web 3.5 Universal Login and Wallet System
//! 
//! This module contains all components for Web 3.5 universal login,
//! wallet creation, domain management, and user onboarding.

pub mod email_verification_service;
pub mod wallet_creation_trigger;
pub mod onboarding_flow_manager;

pub use email_verification_service::{
    EmailVerificationService,
    WalletEmailVerificationRequest,
    WalletEmailVerificationResponse,
    WalletCreationTriggerRequest,
    WalletCreationTriggerResponse,
};

pub use wallet_creation_trigger::{
    WalletCreationTrigger,
    WalletCreationRequest,
    WalletProviderType,
    VerificationLevel,
    WalletCredentials,
    WalletCreationResult,
    WalletCapability,
};

pub use onboarding_flow_manager::{
    OnboardingFlowManager,
    OnboardingSession,
    OnboardingState,
    OnboardingRequest,
    OnboardingResponse,
    WalletSetupData,
    SecurityQuestion,
};
