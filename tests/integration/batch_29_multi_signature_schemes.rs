use crate::test_helpers::*;
use crate::test_helpers_20_30::*;
use tokio::test;

// ============================================================================
// BATCH 29: MULTI-SIGNATURE SCHEMES
// Tests 701-725: Comprehensive multi-signature schemes and applications
// ============================================================================

// Tests 701-705: BLS Multi-Signature
#[tokio::test]
async fn test_701_bls_aggregation_multi_signature() {
    let env = RealTestEnvironment::new("test_701_bls_aggregation_multi_signature").await.unwrap();
    let result = test_bls_multi_signature(&env, "bls_aggregation", 100).await;
    
    assert_eq!(result.signature_scheme, "bls_aggregation");
    assert_eq!(result.signer_count, 100);
    assert_eq!(result.aggregated_signature_size, 48);
    assert_eq!(result.verification_time.as_millis(), 35);
    assert_eq!(result.key_aggregation_time.as_millis(), 20);
    assert_eq!(result.pairing_operations, 2);
    assert_eq!(result.signature_compactness, 0.95);
    assert!(result.non_interactive);
    assert!(result.is_signature_valid);
}

#[tokio::test]
async fn test_702_key_aggregation_bls_signature() {
    let env = RealTestEnvironment::new("test_702_key_aggregation_bls_signature").await.unwrap();
    let result = test_bls_multi_signature(&env, "key_aggregation", 50).await;
    
    assert_eq!(result.signature_scheme, "key_aggregation");
    assert_eq!(result.signer_count, 50);
    assert_eq!(result.aggregated_signature_size, 48);
    assert_eq!(result.verification_time.as_millis(), 25);
    assert_eq!(result.key_aggregation_time.as_millis(), 14);
    assert_eq!(result.pairing_operations, 1);
    assert_eq!(result.signature_compactness, 0.98);
    assert!(result.non_interactive);
    assert!(result.is_signature_valid);
}

#[tokio::test]
async fn test_703_pairing_based_bls_signature() {
    let env = RealTestEnvironment::new("test_703_pairing_based_bls_signature").await.unwrap();
    let result = test_bls_multi_signature(&env, "pairing_based", 75).await;
    
    assert_eq!(result.signature_scheme, "pairing_based");
    assert_eq!(result.signer_count, 75);
    assert_eq!(result.aggregated_signature_size, 48);
    assert_eq!(result.verification_time.as_millis(), 37);
    assert_eq!(result.key_aggregation_time.as_millis(), 21);
    assert_eq!(result.pairing_operations, 3);
    assert_eq!(result.signature_compactness, 0.92);
    assert!(result.non_interactive);
    assert!(result.is_signature_valid);
}

#[tokio::test]
async fn test_704_compact_signatures_bls() {
    let env = RealTestEnvironment::new("test_704_compact_signatures_bls").await.unwrap();
    let result = test_bls_multi_signature(&env, "compact_signatures", 200).await;
    
    assert_eq!(result.signature_scheme, "compact_signatures");
    assert_eq!(result.signer_count, 200);
    assert_eq!(result.aggregated_signature_size, 32);
    assert_eq!(result.verification_time.as_millis(), 42);
    assert_eq!(result.key_aggregation_time.as_millis(), 20);
    assert_eq!(result.pairing_operations, 1);
    assert_eq!(result.signature_compactness, 0.99);
    assert!(result.non_interactive);
    assert!(result.is_signature_valid);
}

#[tokio::test]
async fn test_705_non_interactive_bls_signature() {
    let env = RealTestEnvironment::new("test_705_non_interactive_bls_signature").await.unwrap();
    let result = test_bls_multi_signature(&env, "non_interactive", 150).await;
    
    assert_eq!(result.signature_scheme, "non_interactive");
    assert_eq!(result.signer_count, 150);
    assert_eq!(result.aggregated_signature_size, 48);
    assert_eq!(result.verification_time.as_millis(), 33);
    assert_eq!(result.key_aggregation_time.as_millis(), 15);
    assert_eq!(result.pairing_operations, 1);
    assert_eq!(result.signature_compactness, 0.96);
    assert!(result.non_interactive);
    assert!(result.is_signature_valid);
}

// Tests 706-710: Schnorr Multi-Signature
#[tokio::test]
async fn test_706_musig_protocol_signature() {
    let env = RealTestEnvironment::new("test_706_musig_protocol_signature").await.unwrap();
    let result = test_schnorr_multi_signature(&env, "musig_protocol", 10).await;
    
    assert_eq!(result.protocol_name, "musig_protocol");
    assert_eq!(result.participants, 10);
    assert_eq!(result.signature_size_bytes, 64);
    assert_eq!(result.signing_rounds, 3);
    assert_eq!(result.verification_time.as_millis(), 22);
    assert_eq!(result.key_aggregation_security, 0.98);
    assert_eq!(result.challenge_aggregation_time.as_millis(), 16);
    assert!(result.interactive_protocol);
    assert!(result.is_signature_valid);
}

#[tokio::test]
async fn test_707_key_aggregation_schnorr_signature() {
    let env = RealTestEnvironment::new("test_707_key_aggregation_schnorr_signature").await.unwrap();
    let result = test_schnorr_multi_signature(&env, "key_aggregation", 15).await;
    
    assert_eq!(result.protocol_name, "key_aggregation");
    assert_eq!(result.participants, 15);
    assert_eq!(result.signature_size_bytes, 64);
    assert_eq!(result.signing_rounds, 2);
    assert_eq!(result.verification_time.as_millis(), 21);
    assert_eq!(result.key_aggregation_security, 0.99);
    assert_eq!(result.challenge_aggregation_time.as_millis(), 13);
    assert!(result.interactive_protocol);
    assert!(result.is_signature_valid);
}

#[tokio::test]
async fn test_708_challenge_aggregation_schnorr() {
    let env = RealTestEnvironment::new("test_708_challenge_aggregation_schnorr").await.unwrap();
    let result = test_schnorr_multi_signature(&env, "challenge_aggregation", 20).await;
    
    assert_eq!(result.protocol_name, "challenge_aggregation");
    assert_eq!(result.participants, 20);
    assert_eq!(result.signature_size_bytes, 64);
    assert_eq!(result.signing_rounds, 2);
    assert_eq!(result.verification_time.as_millis(), 26);
    assert_eq!(result.key_aggregation_security, 0.97);
    assert_eq!(result.challenge_aggregation_time.as_millis(), 20);
    assert!(result.interactive_protocol);
    assert!(result.is_signature_valid);
}

#[tokio::test]
async fn test_709_signature_aggregation_schnorr() {
    let env = RealTestEnvironment::new("test_709_signature_aggregation_schnorr").await.unwrap();
    let result = test_schnorr_multi_signature(&env, "signature_aggregation", 25).await;
    
    assert_eq!(result.protocol_name, "signature_aggregation");
    assert_eq!(result.participants, 25);
    assert_eq!(result.signature_size_bytes, 64);
    assert_eq!(result.signing_rounds, 1);
    assert_eq!(result.verification_time.as_millis(), 21);
    assert_eq!(result.key_aggregation_security, 0.96);
    assert_eq!(result.challenge_aggregation_time.as_millis(), 13);
    assert!(!result.interactive_protocol);
    assert!(result.is_signature_valid);
}

#[tokio::test]
async fn test_710_interactive_signing_schnorr() {
    let env = RealTestEnvironment::new("test_710_interactive_signing_schnorr").await.unwrap();
    let result = test_schnorr_multi_signature(&env, "interactive_signing", 8).await;
    
    assert_eq!(result.protocol_name, "interactive_signing");
    assert_eq!(result.participants, 8);
    assert_eq!(result.signature_size_bytes, 64);
    assert_eq!(result.signing_rounds, 4);
    assert_eq!(result.verification_time.as_millis(), 26);
    assert_eq!(result.key_aggregation_security, 0.99);
    assert_eq!(result.challenge_aggregation_time.as_millis(), 21);
    assert!(result.interactive_protocol);
    assert!(result.is_signature_valid);
}

// Tests 711-715: Threshold Multi-Signature
#[tokio::test]
async fn test_711_threshold_signing_scheme() {
    let env = RealTestEnvironment::new("test_711_threshold_signing_scheme").await.unwrap();
    let result = test_threshold_multi_signature(&env, "threshold_signing", 5, 10).await;
    
    assert_eq!(result.threshold_scheme, "threshold_signing");
    assert_eq!(result.threshold, 5);
    assert_eq!(result.total_signers, 10);
    assert_eq!(result.partial_signature_size, 32);
    assert_eq!(result.combination_time.as_millis(), 50);
    assert_eq!(result.secret_sharing_overhead, 0.15);
    assert_eq!(result.fault_tolerance, 0.475);
    assert!(result.distributed_key_gen);
    assert!(result.is_signature_valid);
}

#[tokio::test]
async fn test_712_secret_sharing_threshold() {
    let env = RealTestEnvironment::new("test_712_secret_sharing_threshold").await.unwrap();
    let result = test_threshold_multi_signature(&env, "secret_sharing", 3, 7).await;
    
    assert_eq!(result.threshold_scheme, "secret_sharing");
    assert_eq!(result.threshold, 3);
    assert_eq!(result.total_signers, 7);
    assert_eq!(result.partial_signature_size, 48);
    assert_eq!(result.combination_time.as_millis(), 34);
    assert_eq!(result.secret_sharing_overhead, 0.20);
    assert_eq!(result.fault_tolerance, 0.39428571428571435);
    assert!(result.distributed_key_gen);
    assert!(!result.is_signature_valid); // fault_tolerance < 0.85
}

#[tokio::test]
async fn test_713_partial_combination_threshold() {
    let env = RealTestEnvironment::new("test_713_partial_combination_threshold").await.unwrap();
    let result = test_threshold_multi_signature(&env, "partial_combination", 7, 10).await;
    
    assert_eq!(result.threshold_scheme, "partial_combination");
    assert_eq!(result.threshold, 7);
    assert_eq!(result.total_signers, 10);
    assert_eq!(result.partial_signature_size, 32);
    assert_eq!(result.combination_time.as_millis(), 49);
    assert_eq!(result.secret_sharing_overhead, 0.12);
    assert_eq!(result.fault_tolerance, 0.6859999999999999);
    assert!(!result.distributed_key_gen);
    assert!(!result.is_signature_valid); // fault_tolerance < 0.85
}

#[tokio::test]
async fn test_714_fault_tolerance_threshold() {
    let env = RealTestEnvironment::new("test_714_fault_tolerance_threshold").await.unwrap();
    let result = test_threshold_multi_signature(&env, "fault_tolerance", 8, 10).await;
    
    assert_eq!(result.threshold_scheme, "fault_tolerance");
    assert_eq!(result.threshold, 8);
    assert_eq!(result.total_signers, 10);
    assert_eq!(result.partial_signature_size, 40);
    assert_eq!(result.combination_time.as_millis(), 61);
    assert_eq!(result.secret_sharing_overhead, 0.18);
    assert_eq!(result.fault_tolerance, 0.792);
    assert!(result.distributed_key_gen);
    assert!(!result.is_signature_valid); // fault_tolerance < 0.85
}

#[tokio::test]
async fn test_715_distributed_keygen_threshold() {
    let env = RealTestEnvironment::new("test_715_distributed_keygen_threshold").await.unwrap();
    let result = test_threshold_multi_signature(&env, "distributed_keygen", 9, 10).await;
    
    assert_eq!(result.threshold_scheme, "distributed_keygen");
    assert_eq!(result.threshold, 9);
    assert_eq!(result.total_signers, 10);
    assert_eq!(result.partial_signature_size, 32);
    assert_eq!(result.combination_time.as_millis(), 68);
    assert_eq!(result.secret_sharing_overhead, 0.25);
    assert_eq!(result.fault_tolerance, 0.81);
    assert!(result.distributed_key_gen);
    assert!(!result.is_signature_valid); // fault_tolerance < 0.85
}

// Tests 716-720: Advanced Multi-Signature
#[tokio::test]
async fn test_716_multi_party_computation_signature() {
    let env = RealTestEnvironment::new("test_716_multi_party_computation_signature").await.unwrap();
    let result = test_advanced_multi_signature(&env, "multi_party_computation", "high").await;
    
    assert_eq!(result.scheme_type, "multi_party_computation");
    assert_eq!(result.complexity_level, "high");
    assert_eq!(result.signature_size_bytes, 128);
    assert_eq!(result.verification_time.as_millis(), 80);
    assert_eq!(result.accountability, 0.98);
    assert!(!result.hierarchical_support);
    assert_eq!(result.batch_verification_speedup, 4.2);
    assert_eq!(result.mpc_rounds, 5);
    assert!(result.is_signature_valid);
}

#[tokio::test]
async fn test_717_accountable_multisig_signature() {
    let env = RealTestEnvironment::new("test_717_accountable_multisig_signature").await.unwrap();
    let result = test_advanced_multi_signature(&env, "accountable_multisig", "medium").await;
    
    assert_eq!(result.scheme_type, "accountable_multisig");
    assert_eq!(result.complexity_level, "medium");
    assert_eq!(result.signature_size_bytes, 96);
    assert_eq!(result.verification_time.as_millis(), 50);
    assert_eq!(result.accountability, 0.99);
    assert!(result.hierarchical_support);
    assert_eq!(result.batch_verification_speedup, 3.5);
    assert_eq!(result.mpc_rounds, 3);
    assert!(result.is_signature_valid);
}

#[tokio::test]
async fn test_718_hierarchical_multisig_signature() {
    let env = RealTestEnvironment::new("test_718_hierarchical_multisig_signature").await.unwrap();
    let result = test_advanced_multi_signature(&env, "hierarchical_multisig", "low").await;
    
    assert_eq!(result.scheme_type, "hierarchical_multisig");
    assert_eq!(result.complexity_level, "low");
    assert_eq!(result.signature_size_bytes, 80);
    assert_eq!(result.verification_time.as_millis(), 35);
    assert_eq!(result.accountability, 0.95);
    assert!(result.hierarchical_support);
    assert_eq!(result.batch_verification_speedup, 5.1);
    assert_eq!(result.mpc_rounds, 2);
    assert!(result.is_signature_valid);
}

#[tokio::test]
async fn test_719_ring_multisig_signature() {
    let env = RealTestEnvironment::new("test_719_ring_multisig_signature").await.unwrap();
    let result = test_advanced_multi_signature(&env, "ring_multisig", "very_high").await;
    
    assert_eq!(result.scheme_type, "ring_multisig");
    assert_eq!(result.complexity_level, "very_high");
    assert_eq!(result.signature_size_bytes, 160);
    assert_eq!(result.verification_time.as_millis(), 105);
    assert_eq!(result.accountability, 0.92);
    assert!(!result.hierarchical_support);
    assert_eq!(result.batch_verification_speedup, 2.8);
    assert_eq!(result.mpc_rounds, 4);
    assert!(result.is_signature_valid);
}

#[tokio::test]
async fn test_720_batch_verification_multisig() {
    let env = RealTestEnvironment::new("test_720_batch_verification_multisig").await.unwrap();
    let result = test_advanced_multi_signature(&env, "batch_verification", "low").await;
    
    assert_eq!(result.scheme_type, "batch_verification");
    assert_eq!(result.complexity_level, "low");
    assert_eq!(result.signature_size_bytes, 64);
    assert_eq!(result.verification_time.as_millis(), 20);
    assert_eq!(result.accountability, 0.90);
    assert!(!result.hierarchical_support);
    assert_eq!(result.batch_verification_speedup, 8.7);
    assert_eq!(result.mpc_rounds, 1);
    assert!(result.is_signature_valid);
}

// Tests 721-725: Multi-Signature Applications
#[tokio::test]
async fn test_721_wallet_multisig_application() {
    let env = RealTestEnvironment::new("test_721_wallet_multisig_application").await.unwrap();
    let result = test_multi_signature_application(&env, "wallet_multisig", 3).await;
    
    assert_eq!(result.application_type, "wallet_multisig");
    assert_eq!(result.use_case, "digital_wallet");
    assert_eq!(result.signature_count, 3);
    assert_eq!(result.transaction_size_bytes, 448);
    assert_eq!(result.processing_time.as_millis(), 36);
    assert_eq!(result.security_level, 0.95);
    assert_eq!(result.interoperability, 0.90);
    assert!(result.enterprise_ready);
    assert!(result.is_application_secure);
}

#[tokio::test]
async fn test_722_smart_contract_multisig_application() {
    let env = RealTestEnvironment::new("test_722_smart_contract_multisig_application").await.unwrap();
    let result = test_multi_signature_application(&env, "smart_contract", 5).await;
    
    assert_eq!(result.application_type, "smart_contract");
    assert_eq!(result.use_case, "blockchain_contract");
    assert_eq!(result.signature_count, 5);
    assert_eq!(result.transaction_size_bytes, 832);
    assert_eq!(result.processing_time.as_millis(), 55);
    assert_eq!(result.security_level, 0.98);
    assert_eq!(result.interoperability, 0.95);
    assert!(result.enterprise_ready);
    assert!(result.is_application_secure);
}

#[tokio::test]
async fn test_723_consensus_multisig_application() {
    let env = RealTestEnvironment::new("test_723_consensus_multisig_application").await.unwrap();
    let result = test_multi_signature_application(&env, "consensus_multisig", 10).await;
    
    assert_eq!(result.application_type, "consensus_multisig");
    assert_eq!(result.use_case, "blockchain_consensus");
    assert_eq!(result.signature_count, 10);
    assert_eq!(result.transaction_size_bytes, 768);
    assert_eq!(result.processing_time.as_millis(), 40);
    assert_eq!(result.security_level, 0.99);
    assert_eq!(result.interoperability, 0.88);
    assert!(!result.enterprise_ready);
    assert!(result.is_application_secure);
}

#[tokio::test]
async fn test_724_cross_chain_multisig_application() {
    let env = RealTestEnvironment::new("test_724_cross_chain_multisig_application").await.unwrap();
    let result = test_multi_signature_application(&env, "cross_chain", 7).await;
    
    assert_eq!(result.application_type, "cross_chain");
    assert_eq!(result.use_case, "interoperability");
    assert_eq!(result.signature_count, 7);
    assert_eq!(result.transaction_size_bytes, 1216);
    assert_eq!(result.processing_time.as_millis(), 79);
    assert_eq!(result.security_level, 0.92);
    assert_eq!(result.interoperability, 0.99);
    assert!(result.enterprise_ready);
    assert!(result.is_application_secure);
}

#[tokio::test]
async fn test_725_enterprise_multisig_application() {
    let env = RealTestEnvironment::new("test_725_enterprise_multisig_application").await.unwrap();
    let result = test_multi_signature_application(&env, "enterprise_multisig", 12).await;
    
    assert_eq!(result.application_type, "enterprise_multisig");
    assert_eq!(result.use_case, "corporate_governance");
    assert_eq!(result.signature_count, 12);
    assert_eq!(result.transaction_size_bytes, 1152);
    assert_eq!(result.processing_time.as_millis(), 59);
    assert_eq!(result.security_level, 0.96);
    assert_eq!(result.interoperability, 0.85);
    assert!(result.enterprise_ready);
    assert!(result.is_application_secure);
}
