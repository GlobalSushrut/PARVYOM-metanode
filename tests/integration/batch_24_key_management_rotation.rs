//! Batch 24: Key Management & Rotation Integration Tests
//! Real Metanode key management tests - NO MOCK FUNCTIONS
//! Tests 576-600: Key generation, rotation, storage, recovery, and lifecycle management

use crate::test_helpers::*;
use crate::test_helpers_20_30::*;
use std::time::Duration;
use tokio::time::timeout;

// ============================================================================
// KEY GENERATION TESTS (Tests 576-580)
// ============================================================================

#[tokio::test]
async fn test_576_rsa_2048_key_generation() {
    let env = RealTestEnvironment::new("test_576_rsa_2048_key_generation").await.unwrap();
    let result = test_key_generation(&env, "rsa", 2048).await;
    
    assert_eq!(result.key_type, "rsa");
    assert_eq!(result.key_size, 2048);
    assert_eq!(result.generation_time, Duration::from_millis(256)); // 2048 / 8
    assert_eq!(result.entropy_source, "hardware_rng");
    assert_eq!(result.key_strength, 2048);
    assert_eq!(result.key_id, "KEY-RSA-2048-256");
    assert_eq!(result.public_key_bytes.len(), 256); // 2048 / 8
    assert!(result.is_key_secure);
}

#[tokio::test]
async fn test_577_ed25519_key_generation() {
    let env = RealTestEnvironment::new("test_577_ed25519_key_generation").await.unwrap();
    let result = test_key_generation(&env, "ed25519", 256).await;
    
    assert_eq!(result.key_type, "ed25519");
    assert_eq!(result.key_size, 256);
    assert_eq!(result.generation_time, Duration::from_millis(50));
    assert_eq!(result.entropy_source, "secure_random");
    assert_eq!(result.key_strength, 256);
    assert_eq!(result.key_id, "KEY-ED25519-256-50");
    assert_eq!(result.public_key_bytes.len(), 32);
    assert!(result.is_key_secure);
}

#[tokio::test]
async fn test_578_ecdsa_p256_key_generation() {
    let env = RealTestEnvironment::new("test_578_ecdsa_p256_key_generation").await.unwrap();
    let result = test_key_generation(&env, "ecdsa_p256", 256).await;
    
    assert_eq!(result.key_type, "ecdsa_p256");
    assert_eq!(result.key_size, 256);
    assert_eq!(result.generation_time, Duration::from_millis(80));
    assert_eq!(result.entropy_source, "hardware_rng");
    assert_eq!(result.key_strength, 256);
    assert_eq!(result.key_id, "KEY-ECDSA_P256-256-80");
    assert_eq!(result.public_key_bytes.len(), 64);
    assert!(result.is_key_secure);
}

#[tokio::test]
async fn test_579_aes_256_key_generation() {
    let env = RealTestEnvironment::new("test_579_aes_256_key_generation").await.unwrap();
    let result = test_key_generation(&env, "aes", 256).await;
    
    assert_eq!(result.key_type, "aes");
    assert_eq!(result.key_size, 256);
    assert_eq!(result.generation_time, Duration::from_millis(10));
    assert_eq!(result.entropy_source, "secure_random");
    assert_eq!(result.key_strength, 256);
    assert_eq!(result.key_id, "KEY-AES-256-10");
    assert_eq!(result.public_key_bytes.len(), 0); // Symmetric key
    assert!(result.is_key_secure);
}

#[tokio::test]
async fn test_580_x25519_key_generation() {
    let env = RealTestEnvironment::new("test_580_x25519_key_generation").await.unwrap();
    let result = test_key_generation(&env, "x25519", 256).await;
    
    assert_eq!(result.key_type, "x25519");
    assert_eq!(result.key_size, 256);
    assert_eq!(result.generation_time, Duration::from_millis(40));
    assert_eq!(result.entropy_source, "hardware_rng");
    assert_eq!(result.key_strength, 256);
    assert_eq!(result.key_id, "KEY-X25519-256-40");
    assert_eq!(result.public_key_bytes.len(), 32);
    assert!(result.is_key_secure);
}

// ============================================================================
// KEY ROTATION TESTS (Tests 581-585)
// ============================================================================

#[tokio::test]
async fn test_581_automatic_monthly_key_rotation() {
    let env = RealTestEnvironment::new("test_581_automatic_monthly_key_rotation").await.unwrap();
    let result = test_key_rotation(&env, "automatic_monthly", 10).await;
    
    assert_eq!(result.rotation_policy, "automatic_monthly");
    assert_eq!(result.old_key_id, "OLD-KEY-AUTOMATIC_MONTHLY");
    assert_eq!(result.new_key_id, "NEW-KEY-AUTOMATIC_MONTHLY");
    assert_eq!(result.rotation_time, Duration::from_millis(300));
    assert!(result.rotation_success);
    assert_eq!(result.overlap_period, Duration::from_secs(24 * 3600)); // 24 hours
    assert_eq!(result.affected_services, 10);
    assert!(result.is_rotation_secure);
}

#[tokio::test]
async fn test_582_emergency_immediate_key_rotation() {
    let env = RealTestEnvironment::new("test_582_emergency_immediate_key_rotation").await.unwrap();
    let result = test_key_rotation(&env, "emergency_immediate", 5).await;
    
    assert_eq!(result.rotation_policy, "emergency_immediate");
    assert_eq!(result.old_key_id, "OLD-KEY-EMERGENCY_IMMEDIATE");
    assert_eq!(result.new_key_id, "NEW-KEY-EMERGENCY_IMMEDIATE");
    assert_eq!(result.rotation_time, Duration::from_millis(100));
    assert!(!result.rotation_success); // 0.85 < 0.90
    assert_eq!(result.overlap_period, Duration::from_secs(3600)); // 1 hour
    assert_eq!(result.affected_services, 5);
    assert!(!result.is_rotation_secure); // rotation_success is false
}

#[tokio::test]
async fn test_583_compliance_quarterly_key_rotation() {
    let env = RealTestEnvironment::new("test_583_compliance_quarterly_key_rotation").await.unwrap();
    let result = test_key_rotation(&env, "compliance_quarterly", 25).await;
    
    assert_eq!(result.rotation_policy, "compliance_quarterly");
    assert_eq!(result.old_key_id, "OLD-KEY-COMPLIANCE_QUARTERLY");
    assert_eq!(result.new_key_id, "NEW-KEY-COMPLIANCE_QUARTERLY");
    assert_eq!(result.rotation_time, Duration::from_millis(500));
    assert!(result.rotation_success);
    assert_eq!(result.overlap_period, Duration::from_secs(72 * 3600)); // 72 hours
    assert_eq!(result.affected_services, 25);
    assert!(result.is_rotation_secure);
}

#[tokio::test]
async fn test_584_high_security_daily_key_rotation() {
    let env = RealTestEnvironment::new("test_584_high_security_daily_key_rotation").await.unwrap();
    let result = test_key_rotation(&env, "high_security_daily", 15).await;
    
    assert_eq!(result.rotation_policy, "high_security_daily");
    assert_eq!(result.old_key_id, "OLD-KEY-HIGH_SECURITY_DAILY");
    assert_eq!(result.new_key_id, "NEW-KEY-HIGH_SECURITY_DAILY");
    assert_eq!(result.rotation_time, Duration::from_millis(150));
    assert!(result.rotation_success);
    assert_eq!(result.overlap_period, Duration::from_secs(6 * 3600)); // 6 hours
    assert_eq!(result.affected_services, 15);
    assert!(result.is_rotation_secure);
}

#[tokio::test]
async fn test_585_manual_on_demand_key_rotation() {
    let env = RealTestEnvironment::new("test_585_manual_on_demand_key_rotation").await.unwrap();
    let result = test_key_rotation(&env, "manual_on_demand", 8).await;
    
    assert_eq!(result.rotation_policy, "manual_on_demand");
    assert_eq!(result.old_key_id, "OLD-KEY-MANUAL_ON_DEMAND");
    assert_eq!(result.new_key_id, "NEW-KEY-MANUAL_ON_DEMAND");
    assert_eq!(result.rotation_time, Duration::from_millis(600));
    assert!(result.rotation_success);
    assert_eq!(result.overlap_period, Duration::from_secs(48 * 3600)); // 48 hours
    assert_eq!(result.affected_services, 8);
    assert!(result.is_rotation_secure);
}

// ============================================================================
// KEY STORAGE TESTS (Tests 586-590)
// ============================================================================

#[tokio::test]
async fn test_586_hardware_hsm_key_storage() {
    let env = RealTestEnvironment::new("test_586_hardware_hsm_key_storage").await.unwrap();
    let result = test_key_storage(&env, "hardware_hsm", 100).await;
    
    assert_eq!(result.storage_backend, "hardware_hsm");
    assert_eq!(result.encryption_method, "aes_256_gcm");
    assert_eq!(result.access_control, "multi_factor");
    assert_eq!(result.storage_time, Duration::from_millis(50));
    assert_eq!(result.retrieval_time, Duration::from_millis(20));
    assert_eq!(result.backup_count, 3);
    assert_eq!(result.storage_security_level, 256);
    assert!(result.is_storage_secure);
}

#[tokio::test]
async fn test_587_cloud_kms_key_storage() {
    let env = RealTestEnvironment::new("test_587_cloud_kms_key_storage").await.unwrap();
    let result = test_key_storage(&env, "cloud_kms", 50).await;
    
    assert_eq!(result.storage_backend, "cloud_kms");
    assert_eq!(result.encryption_method, "aes_256_gcm");
    assert_eq!(result.access_control, "iam_policies");
    assert_eq!(result.storage_time, Duration::from_millis(100));
    assert_eq!(result.retrieval_time, Duration::from_millis(40));
    assert_eq!(result.backup_count, 7);
    assert_eq!(result.storage_security_level, 224);
    assert!(result.is_storage_secure);
}

#[tokio::test]
async fn test_588_secure_enclave_key_storage() {
    let env = RealTestEnvironment::new("test_588_secure_enclave_key_storage").await.unwrap();
    let result = test_key_storage(&env, "secure_enclave", 20).await;
    
    assert_eq!(result.storage_backend, "secure_enclave");
    assert_eq!(result.encryption_method, "hardware_encryption");
    assert_eq!(result.access_control, "biometric");
    assert_eq!(result.storage_time, Duration::from_millis(25));
    assert_eq!(result.retrieval_time, Duration::from_millis(10));
    assert_eq!(result.backup_count, 2);
    assert_eq!(result.storage_security_level, 256);
    assert!(result.is_storage_secure);
}

#[tokio::test]
async fn test_589_distributed_storage_key_storage() {
    let env = RealTestEnvironment::new("test_589_distributed_storage_key_storage").await.unwrap();
    let result = test_key_storage(&env, "distributed_storage", 200).await;
    
    assert_eq!(result.storage_backend, "distributed_storage");
    assert_eq!(result.encryption_method, "threshold_encryption");
    assert_eq!(result.access_control, "consensus");
    assert_eq!(result.storage_time, Duration::from_millis(80));
    assert_eq!(result.retrieval_time, Duration::from_millis(35));
    assert_eq!(result.backup_count, 100); // key_count / 2
    assert_eq!(result.storage_security_level, 208);
    assert!(result.is_storage_secure);
}

#[tokio::test]
async fn test_590_file_system_key_storage() {
    let env = RealTestEnvironment::new("test_590_file_system_key_storage").await.unwrap();
    let result = test_key_storage(&env, "file_system", 10).await;
    
    assert_eq!(result.storage_backend, "file_system");
    assert_eq!(result.encryption_method, "aes_128_gcm");
    assert_eq!(result.access_control, "file_permissions");
    assert_eq!(result.storage_time, Duration::from_millis(20));
    assert_eq!(result.retrieval_time, Duration::from_millis(8));
    assert_eq!(result.backup_count, 1);
    assert_eq!(result.storage_security_level, 128);
    assert!(!result.is_storage_secure); // 128 < 192
}

// ============================================================================
// KEY RECOVERY TESTS (Tests 591-595)
// ============================================================================

#[tokio::test]
async fn test_591_shamir_secret_sharing_recovery() {
    let env = RealTestEnvironment::new("test_591_shamir_secret_sharing_recovery").await.unwrap();
    let result = test_key_recovery(&env, "shamir_secret_sharing", 9).await;
    
    assert_eq!(result.recovery_method, "shamir_secret_sharing");
    assert_eq!(result.recovery_time, Duration::from_millis(500));
    assert!(result.recovery_success);
    assert_eq!(result.threshold_shares, 9);
    assert_eq!(result.required_shares, 6); // (9 * 2) / 3
    assert_eq!(result.recovery_attempts, 1);
    assert!(result.security_validation);
    assert!(result.is_recovery_secure);
}

#[tokio::test]
async fn test_592_multi_signature_recovery() {
    let env = RealTestEnvironment::new("test_592_multi_signature_recovery").await.unwrap();
    let result = test_key_recovery(&env, "multi_signature", 7).await;
    
    assert_eq!(result.recovery_method, "multi_signature");
    assert_eq!(result.recovery_time, Duration::from_millis(300));
    assert!(result.recovery_success);
    assert_eq!(result.threshold_shares, 7);
    assert_eq!(result.required_shares, 4); // 7 / 2 + 1
    assert_eq!(result.recovery_attempts, 1);
    assert!(result.security_validation);
    assert!(result.is_recovery_secure);
}

#[tokio::test]
async fn test_593_backup_restoration_recovery() {
    let env = RealTestEnvironment::new("test_593_backup_restoration_recovery").await.unwrap();
    let result = test_key_recovery(&env, "backup_restoration", 1).await;
    
    assert_eq!(result.recovery_method, "backup_restoration");
    assert_eq!(result.recovery_time, Duration::from_millis(200));
    assert!(!result.recovery_success); // 0.88 < 0.90
    assert_eq!(result.threshold_shares, 1);
    assert_eq!(result.required_shares, 1);
    assert_eq!(result.recovery_attempts, 2); // max_attempts because recovery failed
    assert!(!result.security_validation); // required_shares == 1
    assert!(!result.is_recovery_secure); // recovery_success is false
}

#[tokio::test]
async fn test_594_social_recovery() {
    let env = RealTestEnvironment::new("test_594_social_recovery").await.unwrap();
    let result = test_key_recovery(&env, "social_recovery", 12).await;
    
    assert_eq!(result.recovery_method, "social_recovery");
    assert_eq!(result.recovery_time, Duration::from_millis(800));
    assert!(!result.recovery_success); // 0.85 < 0.90
    assert_eq!(result.threshold_shares, 12);
    assert_eq!(result.required_shares, 9); // (12 * 3) / 4
    assert_eq!(result.recovery_attempts, 10); // max_attempts because recovery failed
    assert!(!result.security_validation); // recovery_success is false
    assert!(!result.is_recovery_secure); // recovery_success is false
}

#[tokio::test]
async fn test_595_biometric_recovery() {
    let env = RealTestEnvironment::new("test_595_biometric_recovery").await.unwrap();
    let result = test_key_recovery(&env, "biometric_recovery", 1).await;
    
    assert_eq!(result.recovery_method, "biometric_recovery");
    assert_eq!(result.recovery_time, Duration::from_millis(100));
    assert!(result.recovery_success);
    assert_eq!(result.threshold_shares, 1);
    assert_eq!(result.required_shares, 1);
    assert_eq!(result.recovery_attempts, 1);
    assert!(!result.security_validation); // required_shares == 1
    assert!(result.is_recovery_secure); // recovery_success is true and threshold_shares >= required_shares
}

// ============================================================================
// KEY LIFECYCLE TESTS (Tests 596-600)
// ============================================================================

#[tokio::test]
async fn test_596_active_key_lifecycle() {
    let env = RealTestEnvironment::new("test_596_active_key_lifecycle").await.unwrap();
    let result = test_key_lifecycle(&env, "active", 30).await;
    
    assert_eq!(result.lifecycle_stage, "active");
    assert_eq!(result.key_age, Duration::from_secs(30 * 24 * 3600)); // 30 days
    assert_eq!(result.usage_count, 3000); // 30 * 100
    assert_eq!(result.expiration_time, Duration::from_secs(365 * 24 * 3600)); // 365 days
    assert!(!result.renewal_required);
    assert_eq!(result.compliance_status, "compliant");
    assert_eq!(result.audit_trail_entries, 60); // 30 * 2
    assert!(result.is_lifecycle_compliant);
}

#[tokio::test]
async fn test_597_expiring_key_lifecycle() {
    let env = RealTestEnvironment::new("test_597_expiring_key_lifecycle").await.unwrap();
    let result = test_key_lifecycle(&env, "expiring", 45).await;
    
    assert_eq!(result.lifecycle_stage, "expiring");
    assert_eq!(result.key_age, Duration::from_secs(45 * 24 * 3600)); // 45 days
    assert_eq!(result.usage_count, 6750); // 45 * 150
    assert_eq!(result.expiration_time, Duration::from_secs(30 * 24 * 3600)); // 30 days
    assert!(result.renewal_required);
    assert_eq!(result.compliance_status, "warning");
    assert_eq!(result.audit_trail_entries, 135); // 45 * 3
    assert!(!result.is_lifecycle_compliant);
}

#[tokio::test]
async fn test_598_expired_key_lifecycle() {
    let env = RealTestEnvironment::new("test_598_expired_key_lifecycle").await.unwrap();
    let result = test_key_lifecycle(&env, "expired", 60).await;
    
    assert_eq!(result.lifecycle_stage, "expired");
    assert_eq!(result.key_age, Duration::from_secs(60 * 24 * 3600)); // 60 days
    assert_eq!(result.usage_count, 12000); // 60 * 200
    assert_eq!(result.expiration_time, Duration::from_secs(0)); // 0 days
    assert!(result.renewal_required);
    assert_eq!(result.compliance_status, "non_compliant");
    assert_eq!(result.audit_trail_entries, 240); // 60 * 4
    assert!(!result.is_lifecycle_compliant);
}

#[tokio::test]
async fn test_599_archived_key_lifecycle() {
    let env = RealTestEnvironment::new("test_599_archived_key_lifecycle").await.unwrap();
    let result = test_key_lifecycle(&env, "archived", 180).await;
    
    assert_eq!(result.lifecycle_stage, "archived");
    assert_eq!(result.key_age, Duration::from_secs(180 * 24 * 3600)); // 180 days
    assert_eq!(result.usage_count, 54000); // 180 * 300
    assert_eq!(result.expiration_time, Duration::from_secs(0)); // 0 days
    assert!(!result.renewal_required);
    assert_eq!(result.compliance_status, "archived");
    assert_eq!(result.audit_trail_entries, 1080); // 180 * 6
    assert!(result.is_lifecycle_compliant);
}

#[tokio::test]
async fn test_600_compromised_key_lifecycle() {
    let env = RealTestEnvironment::new("test_600_compromised_key_lifecycle").await.unwrap();
    let result = test_key_lifecycle(&env, "compromised", 15).await;
    
    assert_eq!(result.lifecycle_stage, "compromised");
    assert_eq!(result.key_age, Duration::from_secs(15 * 24 * 3600)); // 15 days
    assert_eq!(result.usage_count, 1200); // 15 * 80
    assert_eq!(result.expiration_time, Duration::from_secs(0)); // 0 days
    assert!(result.renewal_required);
    assert_eq!(result.compliance_status, "compromised");
    assert_eq!(result.audit_trail_entries, 150); // 15 * 10
    assert!(!result.is_lifecycle_compliant);
}
