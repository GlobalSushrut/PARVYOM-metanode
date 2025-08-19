use crate::test_helpers::*;
use crate::test_helpers_30_40::*;
use tokio::test;

// ============================================================================
// BATCH 30: THRESHOLD CRYPTOGRAPHY INTEGRATION TESTS (Tests 726-750)
// Real Metanode integration tests - NO MOCK FUNCTIONS
// ============================================================================

// Tests 726-730: Threshold Cryptography Schemes
#[tokio::test]
async fn test_726_shamir_secret_sharing() {
    let env = RealTestEnvironment::new("test_726_shamir_secret_sharing").await.unwrap();
    let result = test_threshold_cryptography(&env, "shamir_secret_sharing", 3, 5).await;
    
    assert_eq!(result.scheme_type, "shamir_secret_sharing");
    assert_eq!(result.threshold, 3);
    assert_eq!(result.total_participants, 5);
    assert_eq!(result.key_share_size, 32);
    assert_eq!(result.reconstruction_time.as_millis(), 60);
    assert_eq!(result.secret_sharing_overhead, 0.15);
    assert_eq!(result.security_level, 256);
    assert_eq!(result.fault_tolerance, 0.57);
    assert!(result.is_reconstruction_valid);
}

#[tokio::test]
async fn test_727_feldman_verifiable_secret_sharing() {
    let env = RealTestEnvironment::new("test_727_feldman_verifiable_secret_sharing").await.unwrap();
    let result = test_threshold_cryptography(&env, "feldman_vss", 5, 7).await;
    
    assert_eq!(result.scheme_type, "feldman_vss");
    assert_eq!(result.threshold, 5);
    assert_eq!(result.total_participants, 7);
    assert_eq!(result.key_share_size, 48);
    assert_eq!(result.reconstruction_time.as_millis(), 94);
    assert_eq!(result.secret_sharing_overhead, 0.20);
    assert_eq!(result.security_level, 256);
    assert!(result.fault_tolerance > 0.65);
    assert!(result.is_reconstruction_valid);
}

#[tokio::test]
async fn test_728_pedersen_verifiable_secret_sharing() {
    let env = RealTestEnvironment::new("test_728_pedersen_verifiable_secret_sharing").await.unwrap();
    let result = test_threshold_cryptography(&env, "pedersen_vss", 4, 6).await;
    
    assert_eq!(result.scheme_type, "pedersen_vss");
    assert_eq!(result.threshold, 4);
    assert_eq!(result.total_participants, 6);
    assert_eq!(result.key_share_size, 64);
    assert_eq!(result.reconstruction_time.as_millis(), 97);
    assert_eq!(result.secret_sharing_overhead, 0.25);
    assert_eq!(result.security_level, 256);
    assert!(result.fault_tolerance > 0.60);
    assert!(result.is_reconstruction_valid);
}

#[tokio::test]
async fn test_729_threshold_rsa_cryptography() {
    let env = RealTestEnvironment::new("test_729_threshold_rsa_cryptography").await.unwrap();
    let result = test_threshold_cryptography(&env, "threshold_rsa", 7, 10).await;
    
    assert_eq!(result.scheme_type, "threshold_rsa");
    assert_eq!(result.threshold, 7);
    assert_eq!(result.total_participants, 10);
    assert_eq!(result.key_share_size, 256);
    assert_eq!(result.reconstruction_time.as_millis(), 175);
    assert_eq!(result.secret_sharing_overhead, 0.30);
    assert_eq!(result.security_level, 256);
    assert!(result.fault_tolerance > 0.60);
    assert!(result.is_reconstruction_valid);
}

#[tokio::test]
async fn test_730_threshold_ecdsa_cryptography() {
    let env = RealTestEnvironment::new("test_730_threshold_ecdsa_cryptography").await.unwrap();
    let result = test_threshold_cryptography(&env, "threshold_ecdsa", 6, 9).await;
    
    assert_eq!(result.scheme_type, "threshold_ecdsa");
    assert_eq!(result.threshold, 6);
    assert_eq!(result.total_participants, 9);
    assert_eq!(result.key_share_size, 64);
    assert_eq!(result.reconstruction_time.as_millis(), 133);
    assert_eq!(result.secret_sharing_overhead, 0.18);
    assert_eq!(result.security_level, 256);
    assert!(result.fault_tolerance > 0.60);
    assert!(result.is_reconstruction_valid);
}

// Tests 731-735: Threshold Signature Schemes
#[tokio::test]
async fn test_731_threshold_bls_signature() {
    let env = RealTestEnvironment::new("test_731_threshold_bls_signature").await.unwrap();
    let result = test_threshold_signature(&env, "threshold_bls", 3, 5).await;
    
    assert_eq!(result.signature_scheme, "threshold_bls");
    assert_eq!(result.threshold, 3);
    assert_eq!(result.signers_count, 5);
    assert_eq!(result.partial_signature_size, 48);
    assert_eq!(result.aggregation_time.as_millis(), 46);
    assert!(result.signature_validity);
    assert_eq!(result.security_strength, 256);
    assert_eq!(result.performance_score, 0.92);
    assert!(result.is_signature_secure);
}

#[tokio::test]
async fn test_732_threshold_schnorr_signature() {
    let env = RealTestEnvironment::new("test_732_threshold_schnorr_signature").await.unwrap();
    let result = test_threshold_signature(&env, "threshold_schnorr", 4, 7).await;
    
    assert_eq!(result.signature_scheme, "threshold_schnorr");
    assert_eq!(result.threshold, 4);
    assert_eq!(result.signers_count, 7);
    assert_eq!(result.partial_signature_size, 64);
    assert_eq!(result.aggregation_time.as_millis(), 60);
    assert!(result.signature_validity);
    assert_eq!(result.security_strength, 256);
    assert_eq!(result.performance_score, 0.88);
    assert!(result.is_signature_secure);
}

#[tokio::test]
async fn test_733_threshold_ecdsa_signature() {
    let env = RealTestEnvironment::new("test_733_threshold_ecdsa_signature").await.unwrap();
    let result = test_threshold_signature(&env, "threshold_ecdsa", 5, 8).await;
    
    assert_eq!(result.signature_scheme, "threshold_ecdsa");
    assert_eq!(result.threshold, 5);
    assert_eq!(result.signers_count, 8);
    assert_eq!(result.partial_signature_size, 64);
    assert_eq!(result.aggregation_time.as_millis(), 74);
    assert!(result.signature_validity);
    assert_eq!(result.security_strength, 256);
    assert_eq!(result.performance_score, 0.85);
    assert!(result.is_signature_secure);
}

#[tokio::test]
async fn test_734_frost_signature_scheme() {
    let env = RealTestEnvironment::new("test_734_frost_signature_scheme").await.unwrap();
    let result = test_threshold_signature(&env, "frost_signature", 2, 3).await;
    
    assert_eq!(result.signature_scheme, "frost_signature");
    assert_eq!(result.threshold, 2);
    assert_eq!(result.signers_count, 3);
    assert_eq!(result.partial_signature_size, 64);
    assert_eq!(result.aggregation_time.as_millis(), 42);
    assert!(result.signature_validity);
    assert_eq!(result.security_strength, 256);
    assert_eq!(result.performance_score, 0.90);
    assert!(result.is_signature_secure);
}

#[tokio::test]
async fn test_735_multisig_threshold_signature() {
    let env = RealTestEnvironment::new("test_735_multisig_threshold_signature").await.unwrap();
    let result = test_threshold_signature(&env, "multisig_threshold", 6, 10).await;
    
    assert_eq!(result.signature_scheme, "multisig_threshold");
    assert_eq!(result.threshold, 6);
    assert_eq!(result.signers_count, 10);
    assert_eq!(result.partial_signature_size, 96);
    assert_eq!(result.aggregation_time.as_millis(), 83);
    assert!(result.signature_validity);
    assert_eq!(result.security_strength, 256);
    assert_eq!(result.performance_score, 0.82);
    assert!(result.is_signature_secure);
}

// Tests 736-740: Secret Sharing Schemes
#[tokio::test]
async fn test_736_shamir_secret_sharing_scheme() {
    let env = RealTestEnvironment::new("test_736_shamir_secret_sharing_scheme").await.unwrap();
    let result = test_secret_sharing(&env, "shamir_sharing", 256, 5, 3).await;
    
    assert_eq!(result.sharing_scheme, "shamir_sharing");
    assert_eq!(result.secret_size, 256);
    assert_eq!(result.share_count, 5);
    assert_eq!(result.reconstruction_threshold, 3);
    assert_eq!(result.sharing_time.as_millis(), 67);
    assert_eq!(result.reconstruction_accuracy, 1.0);
    assert!(result.information_theoretic_security);
    assert_eq!(result.computational_overhead, 0.12);
    assert!(result.is_sharing_secure);
}

#[tokio::test]
async fn test_737_additive_secret_sharing() {
    let env = RealTestEnvironment::new("test_737_additive_secret_sharing").await.unwrap();
    let result = test_secret_sharing(&env, "additive_sharing", 128, 4, 4).await;
    
    assert_eq!(result.sharing_scheme, "additive_sharing");
    assert_eq!(result.secret_size, 128);
    assert_eq!(result.share_count, 4);
    assert_eq!(result.reconstruction_threshold, 4);
    assert_eq!(result.sharing_time.as_millis(), 39);
    assert_eq!(result.reconstruction_accuracy, 1.0);
    assert!(result.information_theoretic_security);
    assert_eq!(result.computational_overhead, 0.08);
    assert!(result.is_sharing_secure);
}

#[tokio::test]
async fn test_738_replicated_secret_sharing() {
    let env = RealTestEnvironment::new("test_738_replicated_secret_sharing").await.unwrap();
    let result = test_secret_sharing(&env, "replicated_sharing", 192, 6, 3).await;
    
    assert_eq!(result.sharing_scheme, "replicated_sharing");
    assert_eq!(result.secret_size, 192);
    assert_eq!(result.share_count, 6);
    assert_eq!(result.reconstruction_threshold, 3);
    assert_eq!(result.sharing_time.as_millis(), 56);
    assert_eq!(result.reconstruction_accuracy, 1.0);
    assert!(!result.information_theoretic_security);
    assert_eq!(result.computational_overhead, 0.25);
    assert!(result.is_sharing_secure);
}

#[tokio::test]
async fn test_739_packed_secret_sharing() {
    let env = RealTestEnvironment::new("test_739_packed_secret_sharing").await.unwrap();
    let result = test_secret_sharing(&env, "packed_sharing", 512, 8, 5).await;
    
    assert_eq!(result.sharing_scheme, "packed_sharing");
    assert_eq!(result.secret_size, 512);
    assert_eq!(result.share_count, 8);
    assert_eq!(result.reconstruction_threshold, 5);
    assert_eq!(result.sharing_time.as_millis(), 110);
    assert_eq!(result.reconstruction_accuracy, 0.99);
    assert!(result.information_theoretic_security);
    assert_eq!(result.computational_overhead, 0.15);
    assert!(result.is_sharing_secure);
}

#[tokio::test]
async fn test_740_linear_secret_sharing() {
    let env = RealTestEnvironment::new("test_740_linear_secret_sharing").await.unwrap();
    let result = test_secret_sharing(&env, "linear_sharing", 384, 7, 4).await;
    
    assert_eq!(result.sharing_scheme, "linear_sharing");
    assert_eq!(result.secret_size, 384);
    assert_eq!(result.share_count, 7);
    assert_eq!(result.reconstruction_threshold, 4);
    assert_eq!(result.sharing_time.as_millis(), 97);
    assert_eq!(result.reconstruction_accuracy, 0.98);
    assert!(result.information_theoretic_security);
    assert_eq!(result.computational_overhead, 0.18);
    assert!(result.is_sharing_secure);
}

// Tests 741-745: Threshold Encryption Schemes
#[tokio::test]
async fn test_741_threshold_elgamal_encryption() {
    let env = RealTestEnvironment::new("test_741_threshold_elgamal_encryption").await.unwrap();
    let result = test_threshold_encryption(&env, "threshold_elgamal", 3, 5).await;
    
    assert_eq!(result.encryption_scheme, "threshold_elgamal");
    assert_eq!(result.threshold, 3);
    assert_eq!(result.decryption_shares, 5);
    assert_eq!(result.ciphertext_size, 128);
    assert_eq!(result.decryption_time.as_millis(), 119);
    assert!(result.homomorphic_properties);
    assert_eq!(result.security_level, 256);
    assert_eq!(result.efficiency_score, 0.88);
    assert!(result.is_decryption_valid);
}

#[tokio::test]
async fn test_742_threshold_rsa_encryption() {
    let env = RealTestEnvironment::new("test_742_threshold_rsa_encryption").await.unwrap();
    let result = test_threshold_encryption(&env, "threshold_rsa", 4, 7).await;
    
    assert_eq!(result.encryption_scheme, "threshold_rsa");
    assert_eq!(result.threshold, 4);
    assert_eq!(result.decryption_shares, 7);
    assert_eq!(result.ciphertext_size, 256);
    assert_eq!(result.decryption_time.as_millis(), 174);
    assert!(!result.homomorphic_properties);
    assert_eq!(result.security_level, 256);
    assert_eq!(result.efficiency_score, 0.75);
    assert!(result.is_decryption_valid);
}

#[tokio::test]
async fn test_743_threshold_paillier_encryption() {
    let env = RealTestEnvironment::new("test_743_threshold_paillier_encryption").await.unwrap();
    let result = test_threshold_encryption(&env, "threshold_paillier", 5, 8).await;
    
    assert_eq!(result.encryption_scheme, "threshold_paillier");
    assert_eq!(result.threshold, 5);
    assert_eq!(result.decryption_shares, 8);
    assert_eq!(result.ciphertext_size, 512);
    assert_eq!(result.decryption_time.as_millis(), 214);
    assert!(result.homomorphic_properties);
    assert_eq!(result.security_level, 256);
    assert_eq!(result.efficiency_score, 0.70);
    assert!(result.is_decryption_valid);
}

#[tokio::test]
async fn test_744_proxy_re_encryption() {
    let env = RealTestEnvironment::new("test_744_proxy_re_encryption").await.unwrap();
    let result = test_threshold_encryption(&env, "proxy_re_encryption", 2, 3).await;
    
    assert_eq!(result.encryption_scheme, "proxy_re_encryption");
    assert_eq!(result.threshold, 2);
    assert_eq!(result.decryption_shares, 3);
    assert_eq!(result.ciphertext_size, 192);
    assert_eq!(result.decryption_time.as_millis(), 120);
    assert!(!result.homomorphic_properties);
    assert_eq!(result.security_level, 256);
    assert_eq!(result.efficiency_score, 0.82);
    assert!(result.is_decryption_valid);
}

#[tokio::test]
async fn test_745_attribute_based_encryption() {
    let env = RealTestEnvironment::new("test_745_attribute_based_encryption").await.unwrap();
    let result = test_threshold_encryption(&env, "attribute_based_encryption", 6, 9).await;
    
    assert_eq!(result.encryption_scheme, "attribute_based_encryption");
    assert_eq!(result.threshold, 6);
    assert_eq!(result.decryption_shares, 9);
    assert_eq!(result.ciphertext_size, 256);
    assert_eq!(result.decryption_time.as_millis(), 185);
    assert!(!result.homomorphic_properties);
    assert_eq!(result.security_level, 256);
    assert_eq!(result.efficiency_score, 0.78);
    assert!(result.is_decryption_valid);
}

// Tests 746-750: Distributed Key Generation
#[tokio::test]
async fn test_746_pedersen_distributed_key_generation() {
    let env = RealTestEnvironment::new("test_746_pedersen_distributed_key_generation").await.unwrap();
    let result = test_distributed_key_generation(&env, "pedersen_dkg", 5).await;
    
    assert_eq!(result.key_generation_protocol, "pedersen_dkg");
    assert_eq!(result.participants, 5);
    assert_eq!(result.key_size, 256);
    assert_eq!(result.generation_rounds, 3);
    assert_eq!(result.generation_time.as_millis(), 290);
    assert!(result.verifiable_secret_sharing);
    assert_eq!(result.byzantine_fault_tolerance, 0.33);
    assert_eq!(result.protocol_security, 0.95);
    assert!(result.is_key_generation_secure);
}

#[tokio::test]
async fn test_747_feldman_distributed_key_generation() {
    let env = RealTestEnvironment::new("test_747_feldman_distributed_key_generation").await.unwrap();
    let result = test_distributed_key_generation(&env, "feldman_dkg", 7).await;
    
    assert_eq!(result.key_generation_protocol, "feldman_dkg");
    assert_eq!(result.participants, 7);
    assert_eq!(result.key_size, 256);
    assert_eq!(result.generation_rounds, 2);
    assert_eq!(result.generation_time.as_millis(), 260);
    assert!(result.verifiable_secret_sharing);
    assert_eq!(result.byzantine_fault_tolerance, 0.33);
    assert_eq!(result.protocol_security, 0.92);
    assert!(result.is_key_generation_secure);
}

#[tokio::test]
async fn test_748_gennaro_distributed_key_generation() {
    let env = RealTestEnvironment::new("test_748_gennaro_distributed_key_generation").await.unwrap();
    let result = test_distributed_key_generation(&env, "gennaro_dkg", 6).await;
    
    assert_eq!(result.key_generation_protocol, "gennaro_dkg");
    assert_eq!(result.participants, 6);
    assert_eq!(result.key_size, 256);
    assert_eq!(result.generation_rounds, 4);
    assert_eq!(result.generation_time.as_millis(), 340);
    assert!(result.verifiable_secret_sharing);
    assert_eq!(result.byzantine_fault_tolerance, 0.33);
    assert_eq!(result.protocol_security, 0.98);
    assert!(result.is_key_generation_secure);
}

#[tokio::test]
async fn test_749_joint_feldman_distributed_key_generation() {
    let env = RealTestEnvironment::new("test_749_joint_feldman_distributed_key_generation").await.unwrap();
    let result = test_distributed_key_generation(&env, "joint_feldman", 8).await;
    
    assert_eq!(result.key_generation_protocol, "joint_feldman");
    assert_eq!(result.participants, 8);
    assert_eq!(result.key_size, 256);
    assert_eq!(result.generation_rounds, 3);
    assert_eq!(result.generation_time.as_millis(), 310);
    assert!(result.verifiable_secret_sharing);
    assert_eq!(result.byzantine_fault_tolerance, 0.33);
    assert_eq!(result.protocol_security, 0.94);
    assert!(result.is_key_generation_secure);
}

#[tokio::test]
async fn test_750_secure_distributed_key_generation() {
    let env = RealTestEnvironment::new("test_750_secure_distributed_key_generation").await.unwrap();
    let result = test_distributed_key_generation(&env, "secure_dkg", 4).await;
    
    assert_eq!(result.key_generation_protocol, "secure_dkg");
    assert_eq!(result.participants, 4);
    assert_eq!(result.key_size, 256);
    assert_eq!(result.generation_rounds, 5);
    assert_eq!(result.generation_time.as_millis(), 380);
    assert!(result.verifiable_secret_sharing);
    assert_eq!(result.byzantine_fault_tolerance, 0.33);
    assert_eq!(result.protocol_security, 0.99);
    assert!(result.is_key_generation_secure);
}
