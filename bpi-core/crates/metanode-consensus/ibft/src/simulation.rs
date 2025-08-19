//! IBFT consensus simulation utilities

use super::*;

/// Create a test validator set
pub fn create_test_validators(count: usize) -> Vec<ValidatorInfo> {
    let mut validators = Vec::new();
    
    for i in 0..count {
        let node_id = format!("validator_{}", i).into_bytes();
        let (bls_private_key, bls_public_key) = bls_keygen::generate_keypair(&[i as u8; 32]);
        let (_, vrf_public_key) = keygen::generate_keypair(&[i as u8; 32]);
        
        validators.push(ValidatorInfo {
            node_id,
            bls_public_key,
            vrf_public_key,
            stake: 100, // Equal stake for simplicity
        });
    }
    
    validators
}

/// Simulate a consensus round
pub async fn simulate_consensus_round(
    validator_count: usize,
) -> Result<Duration, IbftError> {
    let validators = create_test_validators(validator_count);
    let config = IbftConfig::default();
    
    // Create the first validator as our test node
    let (bls_keypair, _) = bls_keygen::generate_keypair(&[0u8; 32]);
    let (vrf_keypair, _) = keygen::generate_keypair(&[0u8; 32]);
    
    let mut consensus = crate::consensus::IbftConsensus::new(
        config,
        b"validator_0".to_vec(),
        bls_keypair,
        vrf_keypair,
        validators,
    );
    
    let start = Instant::now();
    
    // Simulate one round of consensus
    let _proposal = consensus.propose_block().await?;
    
    let duration = start.elapsed();
    Ok(duration)
}

/// Simulate multiple consensus rounds
pub async fn simulate_multiple_rounds(
    validator_count: usize,
    round_count: usize,
) -> Result<Vec<Duration>, IbftError> {
    let mut durations = Vec::new();
    
    for _ in 0..round_count {
        let duration = simulate_consensus_round(validator_count).await?;
        durations.push(duration);
    }
    
    Ok(durations)
}

/// Benchmark IBFT consensus performance
pub async fn benchmark_consensus(
    validator_count: usize,
    round_count: usize,
) -> Result<(Duration, f64), IbftError> {
    let start = Instant::now();
    let durations = simulate_multiple_rounds(validator_count, round_count).await?;
    let total_duration = start.elapsed();
    
    let avg_round_time = durations.iter().sum::<Duration>().as_millis() as f64 / round_count as f64;
    
    Ok((total_duration, avg_round_time))
}
