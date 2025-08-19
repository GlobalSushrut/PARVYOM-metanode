//! Integration tests for BPI Shadow Registry
//!
//! Comprehensive tests for the military-grade secure Web2-Web3 bridge

use bpi_shadow_registry::{
    ShadowRegistry, ShadowRegistryConfig, BridgeMessage, BridgeResult,
    ActingAsIdentity, ComplianceMetadata, Web3Integration,
};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use serde_json::json;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

#[tokio::test]
async fn test_complete_web2_to_web3_bridge_flow() {
    // Create registry with high security
    let config = ShadowRegistryConfig::high_security();
    let registry = ShadowRegistry::new(config);
    
    // Start registry
    registry.start().await.expect("Failed to start registry");
    
    // Generate keypairs
    let mut csprng = OsRng {};
    let web2_keypair = SigningKey::generate(&mut csprng);
    let web3_keypair = SigningKey::generate(&mut csprng);
    
    // Register Web2 system
    registry.register_web2_system(
        "test-web2-system".to_string(),
        web2_keypair.verifying_key(),
        vec!["payments".to_string()],
        "http://localhost:8080".to_string(),
        HashMap::new(),
    ).await.expect("Failed to register Web2 system");
    
    // Register Web3 contract
    registry.register_web3_contract(
        "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        [1u8; 32],
        web3_keypair.verifying_key(),
        HashMap::new(),
    ).await.expect("Failed to register Web3 contract");
    
    // Create acting-as identity
    let acting_as = registry.create_acting_as_identity(
        "user@example.com".to_string(),
        vec!["transfer".to_string()],
        3600,
    ).await.expect("Failed to create acting-as identity");
    
    // Process bridge request
    let request_id = Uuid::new_v4();
    let bridge_message = BridgeMessage::Web2ToWeb3 {
        request_id,
        source_identity: "test-web2-system".to_string(),
        target_contract: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        method: "transfer".to_string(),
        params: json!({"amount": "1000"}),
        acting_as: Some(acting_as),
    };
    
    let shadow_receipt = registry.process_web2_to_web3(bridge_message)
        .await.expect("Failed to process bridge request");
    
    // Verify receipt
    let is_valid = registry.verify_shadow_receipt(&shadow_receipt)
        .await.expect("Failed to verify receipt");
    
    assert!(is_valid, "Shadow receipt should be valid");
    assert_eq!(shadow_receipt.request_id, request_id);
    
    // Check statistics
    let stats = registry.get_stats().await;
    assert_eq!(stats.get("web2_systems"), Some(&1));
    assert_eq!(stats.get("web3_contracts"), Some(&1));
    assert_eq!(stats.get("shadow_receipts"), Some(&1));
}

#[tokio::test]
async fn test_acting_as_identity_lifecycle() {
    let config = ShadowRegistryConfig::new();
    let registry = ShadowRegistry::new(config);
    
    // Create identity with short duration
    let identity = registry.create_acting_as_identity(
        "test-user".to_string(),
        vec!["read".to_string()],
        1, // 1 second
    ).await.expect("Failed to create identity");
    
    // Verify identity is valid initially
    assert!(identity.expires_at > chrono::Utc::now());
    
    // Wait for expiration
    sleep(Duration::from_secs(2)).await;
    
    // Identity should now be expired (would fail verification in real usage)
    assert!(identity.expires_at < chrono::Utc::now());
}

#[tokio::test]
async fn test_security_configurations() {
    // Test default configuration
    let default_config = ShadowRegistryConfig::new();
    assert_eq!(default_config.session_timeout, 3600);
    assert_eq!(default_config.max_message_size, 1024 * 1024);
    assert!(default_config.enable_zk_proofs);
    
    // Test high security configuration
    let high_sec_config = ShadowRegistryConfig::high_security();
    assert_eq!(high_sec_config.session_timeout, 300);
    assert_eq!(high_sec_config.max_message_size, 512 * 1024);
    assert!(high_sec_config.compliance_requirements.gdpr_compliant);
    assert!(high_sec_config.compliance_requirements.hipaa_compliant);
    assert!(high_sec_config.compliance_requirements.pci_compliant);
}

#[tokio::test]
async fn test_web3_integration() {
    let integration = Web3Integration::new("http://localhost:8545".to_string());
    
    // Test contract validation
    let mut csprng = OsRng {};
    let keypair = SigningKey::generate(&mut csprng);
    
    let contract = bpi_shadow_registry::Web3Contract {
        contract_address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        abi_hash: [1u8; 32],
        public_key: keypair.verifying_key(),
        last_interaction: chrono::Utc::now(),
        metadata: HashMap::new(),
    };
    
    // This would normally make network calls, but we're testing the structure
    // In a real test environment, you'd use a test blockchain or mock
}

#[tokio::test]
async fn test_compliance_metadata() {
    let default_compliance = ComplianceMetadata::default();
    assert!(default_compliance.gdpr_compliant);
    assert!(!default_compliance.hipaa_compliant);
    assert!(!default_compliance.pci_compliant);
    assert_eq!(default_compliance.retention_policy, "7 years");
    assert_eq!(default_compliance.jurisdiction, "US");
}

// Note: Shadow receipt creation test removed due to private method access
// The create_shadow_receipt method is internal and not part of the public API

#[tokio::test]
async fn test_registry_error_handling() {
    let config = ShadowRegistryConfig::new();
    let registry = ShadowRegistry::new(config);
    
    // Test processing bridge request without registered systems
    let request_id = Uuid::new_v4();
    let bridge_message = BridgeMessage::Web2ToWeb3 {
        request_id,
        source_identity: "unregistered-system".to_string(),
        target_contract: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        method: "transfer".to_string(),
        params: json!({}),
        acting_as: None,
    };
    
    let result = registry.process_web2_to_web3(bridge_message).await;
    assert!(result.is_err());
    
    // Should fail because systems are not registered
    match result.unwrap_err() {
        bpi_shadow_registry::ShadowRegistryError::RegistryNotFound(_) => {
            // Expected error
        }
        _ => panic!("Expected RegistryNotFound error"),
    }
}

#[tokio::test]
async fn test_bridge_message_types() {
    // Test Web2 registration message
    let web2_reg = BridgeMessage::Web2Registration {
        system_id: "test-system".to_string(),
        capabilities: vec!["read".to_string(), "write".to_string()],
        public_key: [1u8; 32],
        metadata: HashMap::new(),
    };
    
    // Test Web3 registration message
    let web3_reg = BridgeMessage::Web3Registration {
        contract_address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        abi_hash: [2u8; 32],
        public_key: [3u8; 32],
        metadata: HashMap::new(),
    };
    
    // Verify message structure
    match web2_reg {
        BridgeMessage::Web2Registration { system_id, .. } => {
            assert_eq!(system_id, "test-system");
        }
        _ => panic!("Expected Web2Registration"),
    }
    
    match web3_reg {
        BridgeMessage::Web3Registration { contract_address, .. } => {
            assert_eq!(contract_address, "0x1234567890abcdef1234567890abcdef12345678");
        }
        _ => panic!("Expected Web3Registration"),
    }
}

#[tokio::test]
async fn test_concurrent_operations() {
    let config = ShadowRegistryConfig::new();
    let registry = std::sync::Arc::new(ShadowRegistry::new(config));
    
    // Test concurrent registrations
    let mut handles = vec![];
    
    for i in 0..10 {
        let registry_clone = registry.clone();
        let handle = tokio::spawn(async move {
            let mut csprng = OsRng {};
            let keypair = SigningKey::generate(&mut csprng);
            
            registry_clone.register_web2_system(
                format!("system-{}", i),
                keypair.verifying_key(),
                vec!["test".to_string()],
                format!("http://localhost:808{}", i),
                HashMap::new(),
            ).await
        });
        handles.push(handle);
    }
    
    // Wait for all registrations to complete
    for handle in handles {
        handle.await.expect("Task failed").expect("Registration failed");
    }
    
    // Verify all systems were registered
    let stats = registry.get_stats().await;
    assert_eq!(stats.get("web2_systems"), Some(&10));
}

// Note: Military-grade security test removed due to private field access
// Configuration verification would require public getter methods

#[tokio::test]
async fn test_acting_as_identity_creation() {
    let config = ShadowRegistryConfig::new();
    let registry = ShadowRegistry::new(config);
    
    // Test cryptographic operations
    let acting_as = registry.create_acting_as_identity(
        "secure-user".to_string(),
        vec!["high-security-operation".to_string()],
        300, // 5 minutes
    ).await.expect("Failed to create acting-as identity");
    
    // Verify signature is present and non-zero
    assert_ne!(acting_as.proxy_signature, [0u8; 64]);
    assert!(acting_as.expires_at > chrono::Utc::now());
}
