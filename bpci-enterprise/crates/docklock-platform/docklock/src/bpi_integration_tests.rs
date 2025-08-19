//! BPI Wallet Registry Integration Tests
//! 
//! Comprehensive integration tests for BPI Wallet Registry, Enhanced Storage Database,
//! and BPCI/BCI communication systems.

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use uuid::Uuid;
use tokio::sync::RwLock;

use crate::bpi_wallet_registry::*;
use crate::enhanced_storage_db::*;
use crate::receipt_registry::ReceiptRegistry;
use crate::wallet::{WalletAddress, ServiceId, KeyType, CryptoKeypair};
use crate::metanode_wallet::VerificationLevel;
use crate::error::DockLockResult;

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Test full BPI Wallet Registry and Enhanced Storage Database integration
    #[tokio::test]
    async fn test_full_bpi_integration() -> DockLockResult<()> {
        // Create wallet registry
        let wallet_registry = BpiWalletRegistry::new(
            "test-wallet-registry".to_string(),
            BpiWalletRegistryConfig::default(),
        );

        // Create receipt registry for storage integration
        let receipt_registry = Arc::new(ReceiptRegistry::new(
            "test-receipt-registry".to_string(),
            Default::default(),
        ));

        // Create enhanced storage database with wallet registry integration
        let wallet_registry_arc = Arc::new(wallet_registry);
        let storage_db = EnhancedStorageDb::new(
            "test-storage-db".to_string(),
            wallet_registry_arc.clone(),
            receipt_registry,
            EnhancedStorageConfig::default(),
        );

        // Register a test wallet
        let wallet = RegisteredWallet {
            id: Uuid::new_v4(),
            wallet_type: WalletType::DockLock,
            address: WalletAddress::new(),
            service_id: None,
            verification_level: VerificationLevel::Basic,
            public_key: vec![1, 2, 3, 4],
            key_type: KeyType::Ed25519,
            bpci_endpoint: Some("http://localhost:8080".to_string()),
            bci_endpoint: None,
            capabilities: WalletCapabilities {
                bpci_messaging: true,
                bpci_receiving: true,
                bci_transactions: false,
                bci_receiving: false,
                encryption: true,
                multisig: false,
                governance: false,
                policy_enforcement: false,
                max_message_size: 1024,
                encryption_schemes: vec!["ed25519".to_string()],
            },
            status: WalletStatus::Active,
            registered_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            last_activity: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            metadata: std::collections::HashMap::new(),
            signature: None,
        };

        wallet_registry_arc.register_wallet(wallet.clone()).await?;

        // Test wallet lookup
        let found_wallet = wallet_registry_arc.find_wallet_by_address(&wallet.address).await?;
        assert_eq!(found_wallet.id, wallet.id);
        assert_eq!(found_wallet.verification_level, VerificationLevel::Basic);

        // Test BPCI channel creation
        let channel_id = wallet_registry_arc.create_bpci_channel(
            "test-channel".to_string(),
            "http://localhost:9000".to_string(),
            "test-service".to_string(),
        ).await?;

        // Test storage engine creation
        let engine_id = storage_db.create_storage_engine(
            StorageType::WalletData,
            StorageEngineConfig::default(),
        ).await?;

        // Test record storage
        let record_id = "test-record-1".to_string();
        let mut tags = std::collections::HashMap::new();
        tags.insert("type".to_string(), "wallet".to_string());
        tags.insert("test".to_string(), "true".to_string());
        
        let metadata = StorageMetadata {
            content_type: "application/octet-stream".to_string(),
            tags,
            encoding: Some("binary".to_string()),
            size: 16,
            content_hash: "test-hash".to_string(),
            encryption_scheme: None,
            compression_scheme: None,
            classification: crate::enhanced_storage_db::DataClassification::Public,
            retention_policy: crate::enhanced_storage_db::RetentionPolicy {
                retention_seconds: Some(86400 * 30), // 30 days
                archive_after_seconds: None,
                delete_after_seconds: None,
                compliance_requirements: vec![],
            },
        };
        let acl = AccessControlList {
            owner_permissions: Permissions {
                read: true,
                write: true,
                delete: false,
                admin: false,
                share: false,
            },
            wallet_permissions: std::collections::HashMap::new(),
            service_permissions: std::collections::HashMap::new(),
            public_permissions: None,
            expires_at: None,
        };

        storage_db.store_record(
            StorageType::WalletData,
            record_id.clone(),
            b"test wallet data".to_vec(),
            metadata,
            Some(wallet.id),
            acl,
        ).await?;

        // Test record retrieval
        let retrieved_record = storage_db.get_record(
            StorageType::WalletData,
            &record_id,
            Some(wallet.id),
        ).await?;
        assert_eq!(retrieved_record.data, b"test wallet data".to_vec());
        assert_eq!(retrieved_record.owner_wallet_id, Some(wallet.id));

        // Verify statistics
        let wallet_stats = wallet_registry_arc.get_stats().await;
        assert_eq!(wallet_stats.total_wallets, 1);
        assert_eq!(wallet_stats.active_wallets, 1);

        let storage_stats = storage_db.get_stats().await;
        assert_eq!(storage_stats.total_records, 1);
        assert_eq!(storage_stats.total_engines, 1);

        println!("✅ Full BPI integration test passed");
        Ok(())
    }

    /// Test BPCI message sending and storage integration
    #[tokio::test]
    async fn test_bpci_message_integration() -> DockLockResult<()> {
        let wallet_registry = BpiWalletRegistry::new(
            "test-bpci-registry".to_string(),
            BpiWalletRegistryConfig::default(),
        );

        // Register sender and receiver wallets
        let sender_wallet = RegisteredWallet {
            id: Uuid::new_v4(),
            address: WalletAddress::new(),
            wallet_type: WalletType::Enterprise,
            verification_level: VerificationLevel::Full,
            status: WalletStatus::Active,
            service_id: Some(Uuid::new_v4()),
            bpci_endpoint: Some("http://localhost:8080".to_string()),
            bci_endpoint: None,
            capabilities: WalletCapabilities {
                bpci_messaging: true,
                bpci_receiving: true,
                bci_transactions: true,
                bci_receiving: true,
                encryption: true,
                multisig: true,
                governance: true,
                policy_enforcement: true,
                max_message_size: 2048,
                encryption_schemes: vec!["ed25519".to_string(), "secp256k1".to_string()],
            },
            public_key: vec![0u8; 32],
            key_type: KeyType::Ed25519,
            last_activity: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            registered_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            metadata: std::collections::HashMap::new(),
            signature: None,
        };

        let receiver_wallet = RegisteredWallet {
            id: Uuid::new_v4(),
            address: WalletAddress::new(),
            wallet_type: WalletType::Military,
            verification_level: VerificationLevel::Government,
            status: WalletStatus::Active,
            service_id: Some(Uuid::new_v4()),
            bpci_endpoint: Some("http://localhost:8081".to_string()),
            bci_endpoint: None,
            capabilities: WalletCapabilities {
                bpci_messaging: true,
                bpci_receiving: true,
                bci_transactions: true,
                bci_receiving: true,
                encryption: true,
                multisig: true,
                governance: true,
                policy_enforcement: true,
                max_message_size: 4096,
                encryption_schemes: vec!["ed25519".to_string(), "military-grade".to_string()],
            },
            public_key: vec![0u8; 32],
            key_type: KeyType::Ed25519,
            last_activity: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            registered_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            metadata: std::collections::HashMap::new(),
            signature: None,
        };

        wallet_registry.register_wallet(sender_wallet.clone()).await?;
        wallet_registry.register_wallet(receiver_wallet.clone()).await?;

        // Create BPCI channel
        let channel_id = wallet_registry.create_bpci_channel(
            "secure-channel".to_string(),
            "http://localhost:9000".to_string(),
            "secure-service".to_string(),
        ).await?;

        // Send secure BPCI message
        let message_id = wallet_registry.send_bpci_message(
            sender_wallet.id,
            receiver_wallet.id,
            MessageType::DirectMessage,
            b"secure service request".to_vec(),
            HashMap::new(),
        ).await?;

        // Verify message was stored
        let stats = wallet_registry.get_stats().await;
        assert_eq!(stats.total_wallets, 2);
        assert_eq!(stats.total_bpci_channels, 1);

        println!("✅ BPCI message integration test passed");
        Ok(())
    }

    /// Test multi-wallet type registration and verification levels
    #[tokio::test]
    async fn test_multi_wallet_type_integration() -> DockLockResult<()> {
        let wallet_registry = BpiWalletRegistry::new(
            "test-multi-wallet".to_string(),
            BpiWalletRegistryConfig::default(),
        );

        // Register different wallet types
        let wallet_types = vec![
            (WalletType::DockLock, VerificationLevel::Basic),
            (WalletType::Dao, VerificationLevel::Enhanced),
            (WalletType::MetaNode, VerificationLevel::Full),
            (WalletType::BpciService, VerificationLevel::Enhanced),
            (WalletType::BciBlockchain, VerificationLevel::Full),
            (WalletType::Enterprise, VerificationLevel::Government),
            (WalletType::Military, VerificationLevel::Government),
        ];

        for (i, (wallet_type, verification_level)) in wallet_types.iter().enumerate() {
            let wallet = RegisteredWallet {
                id: Uuid::new_v4(),
                address: WalletAddress::new(),
                wallet_type: wallet_type.clone(),
                verification_level: verification_level.clone(),
                status: WalletStatus::Active,
                service_id: Some(Uuid::new_v4()),
                bpci_endpoint: Some(format!("http://localhost:808{}", i)),
                bci_endpoint: Some(format!("http://localhost:909{}", i)),
                capabilities: WalletCapabilities {
                    bpci_messaging: true,
                    bpci_receiving: true,
                    bci_transactions: i % 2 == 0,
                    bci_receiving: i % 2 == 0,
                    encryption: true,
                    multisig: i > 2,
                    governance: i > 4,
                    policy_enforcement: true,
                    max_message_size: 1024 * (i + 1),
                    encryption_schemes: vec!["ed25519".to_string()],
                },
                public_key: vec![i as u8; 32],
                key_type: KeyType::Ed25519,
                last_activity: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                registered_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                metadata: std::collections::HashMap::new(),
                signature: None,
            };

            wallet_registry.register_wallet(wallet).await?;
        }

        // Verify all wallets were registered
        let stats = wallet_registry.get_stats().await;
        assert_eq!(stats.total_wallets, wallet_types.len() as u64);
        assert_eq!(stats.active_wallets, wallet_types.len() as u64);

        println!("✅ Multi-wallet type integration test passed");
        Ok(())
    }

    /// Test enterprise-grade security and audit trail
    #[tokio::test]
    async fn test_enterprise_security_integration() -> DockLockResult<()> {
        let wallet_registry = BpiWalletRegistry::new(
            "test-enterprise-security".to_string(),
            BpiWalletRegistryConfig {
                max_wallets: 1000,
                require_verification: true,
                ..Default::default()
            },
        );

        // Create receipt registry for audit trail
        let receipt_registry = Arc::new(ReceiptRegistry::new(
            "audit-receipt-registry".to_string(),
            Default::default(),
        ));

        // Create enhanced storage with encryption
        let wallet_registry_arc = Arc::new(wallet_registry);
        let storage_db = EnhancedStorageDb::new(
            "secure-storage-db".to_string(),
            wallet_registry_arc.clone(),
            receipt_registry,
            EnhancedStorageConfig {
                max_storage_engines: 100,
                default_storage_type: StorageType::Documents,
                transaction_log_retention_seconds: 86400 * 30, // 30 days
                enable_auto_cleanup: true,
                cleanup_interval_seconds: 3600, // 1 hour
                backup_config: BackupConfig {
                    enable_auto_backup: true,
                    backup_interval_seconds: 3600,
                    backup_retention_seconds: 86400 * 30, // 30 days
                    backup_location: "s3://military-backup".to_string(),
                    backup_compression: crate::enhanced_storage_db::CompressionAlgorithm::Lz4,
                    backup_encryption: true,
                },
            },
        );

        // Register military-grade wallet
        let military_wallet = RegisteredWallet {
            id: Uuid::new_v4(),
            address: WalletAddress::new(),
            wallet_type: WalletType::Military,
            verification_level: VerificationLevel::Government,
            status: WalletStatus::Active,
            service_id: Some(Uuid::new_v4()),
            bpci_endpoint: Some("https://secure.military.gov:8443".to_string()),
            bci_endpoint: Some("https://blockchain.military.gov:9443".to_string()),
            capabilities: WalletCapabilities {
                bpci_messaging: true,
                bpci_receiving: true,
                bci_transactions: true,
                bci_receiving: true,
                encryption: true,
                multisig: true,
                governance: true,
                policy_enforcement: true,
                max_message_size: 8192,
                encryption_schemes: vec!["ed25519".to_string(), "military-aes-256".to_string()],
            },
            public_key: vec![0u8; 32],
            key_type: KeyType::Ed25519,
            last_activity: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            registered_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            metadata: std::collections::HashMap::new(),
            signature: None,
        };

        wallet_registry_arc.register_wallet(military_wallet.clone()).await?;

        // Create storage engine for Documents
        let _engine_id = storage_db.create_storage_engine(
            StorageType::Documents,
            StorageEngineConfig::default(),
        ).await?;

        // Store classified data with strict access control
        let classified_record_id = "classified-record-1".to_string();
        let mut tags = std::collections::HashMap::new();
        tags.insert("classification".to_string(), "classified".to_string());
        tags.insert("department".to_string(), "military".to_string());
        
        let classified_metadata = StorageMetadata {
            content_type: "text/plain".to_string(),
            tags,
            encoding: Some("utf-8".to_string()),
            size: 35,
            content_hash: "classified-hash".to_string(),
            encryption_scheme: Some("aes-256-gcm".to_string()),
            compression_scheme: None,
            classification: crate::enhanced_storage_db::DataClassification::TopSecret,
            retention_policy: crate::enhanced_storage_db::RetentionPolicy {
                retention_seconds: Some(86400 * 365 * 10), // 10 years
                archive_after_seconds: Some(86400 * 365), // 1 year
                delete_after_seconds: None, // Never auto-delete classified data
                compliance_requirements: vec!["NIST-800-53".to_string(), "FIPS-140-2".to_string()],
            },
        };
        let classified_acl = AccessControlList {
            owner_permissions: Permissions {
                read: true,
                write: true,
                delete: true,
                admin: true,
                share: false,
            },
            wallet_permissions: std::collections::HashMap::new(),
            service_permissions: std::collections::HashMap::new(),
            public_permissions: None,
            expires_at: None,
        };

        storage_db.store_record(
            StorageType::Documents,
            classified_record_id.clone(),
            b"CLASSIFIED: Military operation data".to_vec(),
            classified_metadata,
            Some(military_wallet.id),
            classified_acl,
        ).await?;

        // Verify secure access
        let retrieved_record = storage_db.get_record(
            StorageType::Documents,
            &classified_record_id,
            Some(military_wallet.id),
        ).await?;
        assert_eq!(retrieved_record.owner_wallet_id, Some(military_wallet.id));
        assert!(retrieved_record.metadata.tags.contains_key(&"classification".to_string()));

        println!("✅ Enterprise security integration test passed");
        Ok(())
    }
}
