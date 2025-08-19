//! Minimal Working Integration Test
//! 
//! This demonstrates basic functionality using only existing working modules

use crate::{
    proofs::*,
    receipts::*,
    mining::*,
    MathError,
};

/// Test that basic modules work correctly
pub fn test_basic_functionality() -> Result<(), MathError> {
    println!("🧪 Testing basic functionality...");
    
    // Test 1: Create a simple proof using correct method
    let poa_proof = ProofOfAction::generate_proof(("test_container".to_string(), crate::proofs::ActionType::Deploy, std::collections::HashMap::new()))?;
    println!("✅ ProofOfAction created");
    
    // Test 2: Create a receipt using existing factory with correct field names
    let resource_usage = ResourceUsage {
        cpu_time: 1000,
        memory_peak: 1024 * 1024,
        network_bytes: 512 * 1024,
        storage_bytes: 256 * 1024,
    };
    
    let docklock_receipt = ReceiptFactory::create_docklock_receipt(
        "test_container".to_string(),
        "test_operation".to_string(),
        poa_proof,
        resource_usage,
    );
    println!("✅ DockLock receipt created");
    
    // Test 3: Create receipt aggregator with correct config
    let config = ReceiptAggregationConfig::default();
    let mut aggregator = ReceiptAggregator::new(config);
    
    // Test 4: Add receipt to aggregator
    aggregator.add_receipt(ReceiptType::DockLock(docklock_receipt))?;
    println!("✅ Receipt added to aggregator");
    
    // Test 5: Create mining engine with correct config
    let mining_difficulty = MiningDifficulty::default();
    let mining_rewards = MiningRewards::default();
    let economic_governance = EconomicGovernance::default();
    
    let _mining_engine = MiningEngine::new(
        "test_miner".to_string(),
        mining_difficulty,
        mining_rewards,
        economic_governance,
    );
    println!("✅ Mining engine created");
    
    // Test 6: Get aggregator stats
    let pending_count = aggregator.get_total_pending();
    println!("✅ Pending receipts: {pending_count}");
    
    println!("🎉 All basic functionality tests passed!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_minimal_integration() {
        assert!(test_basic_functionality().is_ok());
    }
}
