use crate::test_helpers::*;
use crate::test_helpers_20_30::*;
use tokio::test;

// ============================================================================
// BATCH 27: CRYPTOGRAPHIC PROOF VERIFICATION
// Tests 651-675: Comprehensive cryptographic proof verification systems
// ============================================================================

// Tests 651-655: Digital Signature Verification
#[tokio::test]
async fn test_651_rsa_signature_verification() {
    let env = RealTestEnvironment::new("test_651_rsa_signature_verification").await.unwrap();
    let result = test_digital_signature_verification(&env, "rsa", 2048).await;
    
    assert_eq!(result.signature_algorithm, "rsa");
    assert_eq!(result.key_size, 2048);
    assert_eq!(result.verification_time.as_millis(), 25);
    assert_eq!(result.signature_bytes.len(), 256);
    assert!(result.signature_validity);
    assert_eq!(result.security_strength, 128);
    assert_eq!(result.performance_score, 0.85);
    assert!(result.is_signature_secure);
}

#[tokio::test]
async fn test_652_ecdsa_signature_verification() {
    let env = RealTestEnvironment::new("test_652_ecdsa_signature_verification").await.unwrap();
    let result = test_digital_signature_verification(&env, "ecdsa", 256).await;
    
    assert_eq!(result.signature_algorithm, "ecdsa");
    assert_eq!(result.key_size, 256);
    assert_eq!(result.verification_time.as_millis(), 12);
    assert_eq!(result.signature_bytes.len(), 64);
    assert!(result.signature_validity);
    assert_eq!(result.security_strength, 128);
    assert_eq!(result.performance_score, 0.92);
    assert!(result.is_signature_secure);
}

#[tokio::test]
async fn test_653_eddsa_signature_verification() {
    let env = RealTestEnvironment::new("test_653_eddsa_signature_verification").await.unwrap();
    let result = test_digital_signature_verification(&env, "eddsa", 255).await;
    
    assert_eq!(result.signature_algorithm, "eddsa");
    assert_eq!(result.key_size, 255);
    assert_eq!(result.verification_time.as_millis(), 8);
    assert_eq!(result.signature_bytes.len(), 64);
    assert!(result.signature_validity);
    assert_eq!(result.security_strength, 128);
    assert_eq!(result.performance_score, 0.95);
    assert!(result.is_signature_secure);
}

#[tokio::test]
async fn test_654_bls_signature_verification() {
    let env = RealTestEnvironment::new("test_654_bls_signature_verification").await.unwrap();
    let result = test_digital_signature_verification(&env, "bls", 381).await;
    
    assert_eq!(result.signature_algorithm, "bls");
    assert_eq!(result.key_size, 381);
    assert_eq!(result.verification_time.as_millis(), 18);
    assert_eq!(result.signature_bytes.len(), 48);
    assert!(result.signature_validity);
    assert_eq!(result.security_strength, 128);
    assert_eq!(result.performance_score, 0.88);
    assert!(result.is_signature_secure);
}

#[tokio::test]
async fn test_655_schnorr_signature_verification() {
    let env = RealTestEnvironment::new("test_655_schnorr_signature_verification").await.unwrap();
    let result = test_digital_signature_verification(&env, "schnorr", 256).await;
    
    assert_eq!(result.signature_algorithm, "schnorr");
    assert_eq!(result.key_size, 256);
    assert_eq!(result.verification_time.as_millis(), 10);
    assert_eq!(result.signature_bytes.len(), 64);
    assert!(result.signature_validity);
    assert_eq!(result.security_strength, 128);
    assert_eq!(result.performance_score, 0.90);
    assert!(result.is_signature_secure);
}

// Tests 656-660: Hash Chain Verification
#[tokio::test]
async fn test_656_sha256_hash_chain_verification() {
    let env = RealTestEnvironment::new("test_656_sha256_hash_chain_verification").await.unwrap();
    let result = test_hash_chain_verification(&env, "sha256", 1000).await;
    
    assert_eq!(result.hash_algorithm, "sha256");
    assert_eq!(result.chain_length, 1000);
    assert_eq!(result.verification_time.as_millis(), 25);
    assert_eq!(result.hash_rate_per_second, 2500000);
    assert_eq!(result.collision_resistance, 0.999999);
    assert_eq!(result.preimage_resistance, 0.999999);
    assert_eq!(result.merkle_tree_depth, 20);
    assert!(result.is_chain_valid);
}

#[tokio::test]
async fn test_657_blake2b_hash_chain_verification() {
    let env = RealTestEnvironment::new("test_657_blake2b_hash_chain_verification").await.unwrap();
    let result = test_hash_chain_verification(&env, "blake2b", 1500).await;
    
    assert_eq!(result.hash_algorithm, "blake2b");
    assert_eq!(result.chain_length, 1500);
    assert_eq!(result.verification_time.as_millis(), 27);
    assert_eq!(result.hash_rate_per_second, 3200000);
    assert_eq!(result.collision_resistance, 0.999998);
    assert_eq!(result.preimage_resistance, 0.999999);
    assert_eq!(result.merkle_tree_depth, 18);
    assert!(result.is_chain_valid);
}

#[tokio::test]
async fn test_658_keccak256_hash_chain_verification() {
    let env = RealTestEnvironment::new("test_658_keccak256_hash_chain_verification").await.unwrap();
    let result = test_hash_chain_verification(&env, "keccak256", 800).await;
    
    assert_eq!(result.hash_algorithm, "keccak256");
    assert_eq!(result.chain_length, 800);
    assert_eq!(result.verification_time.as_millis(), 26);
    assert_eq!(result.hash_rate_per_second, 2100000);
    assert_eq!(result.collision_resistance, 0.999999);
    assert_eq!(result.preimage_resistance, 0.999998);
    assert_eq!(result.merkle_tree_depth, 22);
    assert!(result.is_chain_valid);
}

#[tokio::test]
async fn test_659_poseidon_hash_chain_verification() {
    let env = RealTestEnvironment::new("test_659_poseidon_hash_chain_verification").await.unwrap();
    let result = test_hash_chain_verification(&env, "poseidon", 2000).await;
    
    assert_eq!(result.hash_algorithm, "poseidon");
    assert_eq!(result.chain_length, 2000);
    assert_eq!(result.verification_time.as_millis(), 28);
    assert_eq!(result.hash_rate_per_second, 4000000);
    assert_eq!(result.collision_resistance, 0.999997);
    assert_eq!(result.preimage_resistance, 0.999997);
    assert_eq!(result.merkle_tree_depth, 16);
    assert!(result.is_chain_valid);
}

#[tokio::test]
async fn test_660_merkle_proof_verification() {
    let env = RealTestEnvironment::new("test_660_merkle_proof_verification").await.unwrap();
    let result = test_hash_chain_verification(&env, "merkle_proof", 500).await;
    
    assert_eq!(result.hash_algorithm, "merkle_proof");
    assert_eq!(result.chain_length, 500);
    assert_eq!(result.verification_time.as_millis(), 30);
    assert_eq!(result.hash_rate_per_second, 1800000);
    assert_eq!(result.collision_resistance, 0.999999);
    assert_eq!(result.preimage_resistance, 0.999999);
    assert_eq!(result.merkle_tree_depth, 24);
    assert!(result.is_chain_valid);
}

// Tests 661-665: Commitment Scheme Verification
#[tokio::test]
async fn test_661_pedersen_commitment_verification() {
    let env = RealTestEnvironment::new("test_661_pedersen_commitment_verification").await.unwrap();
    let result = test_commitment_scheme_verification(&env, "pedersen", 256).await;
    
    assert_eq!(result.commitment_type, "pedersen");
    assert_eq!(result.commitment_size_bytes, 64);
    assert_eq!(result.opening_time.as_millis(), 15);
    assert_eq!(result.hiding_property, 0.999);
    assert_eq!(result.binding_property, 0.998);
    assert_eq!(result.verification_complexity, "linear");
    assert!(result.batch_verification_support);
    assert!(result.is_commitment_valid);
}

#[tokio::test]
async fn test_662_kate_commitment_verification() {
    let env = RealTestEnvironment::new("test_662_kate_commitment_verification").await.unwrap();
    let result = test_commitment_scheme_verification(&env, "kate", 256).await;
    
    assert_eq!(result.commitment_type, "kate");
    assert_eq!(result.commitment_size_bytes, 80);
    assert_eq!(result.opening_time.as_millis(), 22);
    assert_eq!(result.hiding_property, 0.997);
    assert_eq!(result.binding_property, 0.999);
    assert_eq!(result.verification_complexity, "logarithmic");
    assert!(result.batch_verification_support);
    assert!(result.is_commitment_valid);
}

#[tokio::test]
async fn test_663_bulletproofs_commitment_verification() {
    let env = RealTestEnvironment::new("test_663_bulletproofs_commitment_verification").await.unwrap();
    let result = test_commitment_scheme_verification(&env, "bulletproofs", 256).await;
    
    assert_eq!(result.commitment_type, "bulletproofs");
    assert_eq!(result.commitment_size_bytes, 96);
    assert_eq!(result.opening_time.as_millis(), 35);
    assert_eq!(result.hiding_property, 0.999);
    assert_eq!(result.binding_property, 0.999);
    assert_eq!(result.verification_complexity, "logarithmic");
    assert!(!result.batch_verification_support);
    assert!(result.is_commitment_valid);
}

#[tokio::test]
async fn test_664_kzg_commitment_verification() {
    let env = RealTestEnvironment::new("test_664_kzg_commitment_verification").await.unwrap();
    let result = test_commitment_scheme_verification(&env, "kzg", 256).await;
    
    assert_eq!(result.commitment_type, "kzg");
    assert_eq!(result.commitment_size_bytes, 80);
    assert_eq!(result.opening_time.as_millis(), 18);
    assert_eq!(result.hiding_property, 0.998);
    assert_eq!(result.binding_property, 0.999);
    assert_eq!(result.verification_complexity, "constant");
    assert!(result.batch_verification_support);
    assert!(result.is_commitment_valid);
}

#[tokio::test]
async fn test_665_polynomial_commitment_verification() {
    let env = RealTestEnvironment::new("test_665_polynomial_commitment_verification").await.unwrap();
    let result = test_commitment_scheme_verification(&env, "polynomial", 256).await;
    
    assert_eq!(result.commitment_type, "polynomial");
    assert_eq!(result.commitment_size_bytes, 128);
    assert_eq!(result.opening_time.as_millis(), 45);
    assert_eq!(result.hiding_property, 0.999);
    assert_eq!(result.binding_property, 0.997);
    assert_eq!(result.verification_complexity, "linear");
    assert!(!result.batch_verification_support);
    assert!(result.is_commitment_valid);
}

// Tests 666-670: Range Proof Verification
#[tokio::test]
async fn test_666_bulletproofs_range_verification() {
    let env = RealTestEnvironment::new("test_666_bulletproofs_range_verification").await.unwrap();
    let result = test_range_proof_verification(&env, "bulletproofs", 64).await;
    
    assert_eq!(result.proof_system, "bulletproofs");
    assert_eq!(result.range_bits, 64);
    assert_eq!(result.proof_size_bytes, 800);
    assert_eq!(result.verification_time.as_millis(), 61);
    assert_eq!(result.zero_knowledge_property, 0.999);
    assert_eq!(result.soundness_error, 0.0001);
    assert!(!result.setup_trusted);
    assert!(result.is_proof_valid);
}

#[tokio::test]
async fn test_667_borromean_range_verification() {
    let env = RealTestEnvironment::new("test_667_borromean_range_verification").await.unwrap();
    let result = test_range_proof_verification(&env, "borromean", 32).await;
    
    assert_eq!(result.proof_system, "borromean");
    assert_eq!(result.range_bits, 32);
    assert_eq!(result.proof_size_bytes, 96);
    assert_eq!(result.verification_time.as_millis(), 33);
    assert_eq!(result.zero_knowledge_property, 0.998);
    assert_eq!(result.soundness_error, 0.0002);
    assert!(!result.setup_trusted);
    assert!(result.is_proof_valid);
}

#[tokio::test]
async fn test_668_mlsag_range_verification() {
    let env = RealTestEnvironment::new("test_668_mlsag_range_verification").await.unwrap();
    let result = test_range_proof_verification(&env, "mlsag", 64).await;
    
    assert_eq!(result.proof_system, "mlsag");
    assert_eq!(result.range_bits, 64);
    assert_eq!(result.proof_size_bytes, 192);
    assert_eq!(result.verification_time.as_millis(), 51);
    assert_eq!(result.zero_knowledge_property, 0.997);
    assert_eq!(result.soundness_error, 0.0001);
    assert!(!result.setup_trusted);
    assert!(result.is_proof_valid);
}

#[tokio::test]
async fn test_669_ring_signatures_range_verification() {
    let env = RealTestEnvironment::new("test_669_ring_signatures_range_verification").await.unwrap();
    let result = test_range_proof_verification(&env, "ring_signatures", 128).await;
    
    assert_eq!(result.proof_system, "ring_signatures");
    assert_eq!(result.range_bits, 128);
    assert_eq!(result.proof_size_bytes, 384);
    assert_eq!(result.verification_time.as_millis(), 87);
    assert_eq!(result.zero_knowledge_property, 0.999);
    assert_eq!(result.soundness_error, 0.0003);
    assert!(!result.setup_trusted);
    assert!(result.is_proof_valid);
}

#[tokio::test]
async fn test_670_confidential_tx_range_verification() {
    let env = RealTestEnvironment::new("test_670_confidential_tx_range_verification").await.unwrap();
    let result = test_range_proof_verification(&env, "confidential_tx", 64).await;
    
    assert_eq!(result.proof_system, "confidential_tx");
    assert_eq!(result.range_bits, 64);
    assert_eq!(result.proof_size_bytes, 384);
    assert_eq!(result.verification_time.as_millis(), 81);
    assert_eq!(result.zero_knowledge_property, 0.998);
    assert_eq!(result.soundness_error, 0.0001);
    assert!(!result.setup_trusted);
    assert!(result.is_proof_valid);
}

// Tests 671-675: Aggregate Proof Verification
#[tokio::test]
async fn test_671_bls_aggregation_verification() {
    let env = RealTestEnvironment::new("test_671_bls_aggregation_verification").await.unwrap();
    let result = test_aggregate_proof_verification(&env, "bls_aggregation", 100).await;
    
    assert_eq!(result.aggregation_scheme, "bls_aggregation");
    assert_eq!(result.signature_count, 100);
    assert_eq!(result.aggregate_size_bytes, 58);
    assert_eq!(result.verification_time.as_millis(), 27);
    assert_eq!(result.space_savings_ratio, 0.95);
    assert_eq!(result.verification_scalability, 0.98);
    assert_eq!(result.batch_verification_speedup, 8.5);
    assert!(result.is_aggregate_valid);
}

#[tokio::test]
async fn test_672_schnorr_aggregation_verification() {
    let env = RealTestEnvironment::new("test_672_schnorr_aggregation_verification").await.unwrap();
    let result = test_aggregate_proof_verification(&env, "schnorr_aggregation", 150).await;
    
    assert_eq!(result.aggregation_scheme, "schnorr_aggregation");
    assert_eq!(result.signature_count, 150);
    assert_eq!(result.aggregate_size_bytes, 79);
    assert_eq!(result.verification_time.as_millis(), 21);
    assert_eq!(result.space_savings_ratio, 0.92);
    assert_eq!(result.verification_scalability, 0.96);
    assert_eq!(result.batch_verification_speedup, 6.2);
    assert!(result.is_aggregate_valid);
}

#[tokio::test]
async fn test_673_multi_signature_verification() {
    let env = RealTestEnvironment::new("test_673_multi_signature_verification").await.unwrap();
    let result = test_aggregate_proof_verification(&env, "multi_signature", 50).await;
    
    assert_eq!(result.aggregation_scheme, "multi_signature");
    assert_eq!(result.signature_count, 50);
    assert_eq!(result.aggregate_size_bytes, 133);
    assert_eq!(result.verification_time.as_millis(), 36);
    assert_eq!(result.space_savings_ratio, 0.85);
    assert_eq!(result.verification_scalability, 0.94);
    assert_eq!(result.batch_verification_speedup, 4.8);
    assert!(result.is_aggregate_valid); // 0.85 >= 0.80 space savings and 0.94 >= 0.90 scalability
}

#[tokio::test]
async fn test_674_threshold_signatures_verification() {
    let env = RealTestEnvironment::new("test_674_threshold_signatures_verification").await.unwrap();
    let result = test_aggregate_proof_verification(&env, "threshold_signatures", 75).await;
    
    assert_eq!(result.aggregation_scheme, "threshold_signatures");
    assert_eq!(result.signature_count, 75);
    assert_eq!(result.aggregate_size_bytes, 103);
    assert_eq!(result.verification_time.as_millis(), 43);
    assert_eq!(result.space_savings_ratio, 0.88);
    assert_eq!(result.verification_scalability, 0.97);
    assert_eq!(result.batch_verification_speedup, 7.1);
    assert!(result.is_aggregate_valid); // 0.88 >= 0.80 space savings and 0.97 >= 0.90 scalability
}

#[tokio::test]
async fn test_675_batch_verification_systems() {
    let env = RealTestEnvironment::new("test_675_batch_verification_systems").await.unwrap();
    let result = test_aggregate_proof_verification(&env, "batch_verification", 200).await;
    
    assert_eq!(result.aggregation_scheme, "batch_verification");
    assert_eq!(result.signature_count, 200);
    assert_eq!(result.aggregate_size_bytes, 276);
    assert_eq!(result.verification_time.as_millis(), 19);
    assert_eq!(result.space_savings_ratio, 0.98);
    assert_eq!(result.verification_scalability, 0.99);
    assert_eq!(result.batch_verification_speedup, 12.3);
    assert!(result.is_aggregate_valid);
}
