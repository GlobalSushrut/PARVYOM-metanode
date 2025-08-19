use super::*;
use prometheus::Registry;

#[tokio::test]
async fn test_docklock_revenue_integration() {
    let registry = Registry::new();
    let engine = PoEMiningEngine::new(&registry).expect("Failed to create engine");
    
    // Create DockLock hosting job with comprehensive revenue streams
    let docklock_job = EconomicJob {
        job_id: "docklock_hosting_001".to_string(),
        job_type: EconomicJobType::DockLockHosting,
        miner_id: "docklock_provider".to_string(),
        gold_equivalent_value: Decimal::new(50_000, 0), // $50k base value
        proof_hash: "proof_docklock_001".to_string(),
        completion_time: Utc::now(),
        cluster_rent_revenue: Some(Decimal::new(25_000, 0)),    // $25k cluster rent
        gas_fee_revenue: Some(Decimal::new(15_000, 0)),         // $15k gas fees
        app_interaction_revenue: Some(Decimal::new(10_000, 0)), // $10k app interactions
        security_layer_revenue: Some(Decimal::new(8_000, 0)),   // $8k security services
        data_pipeline_revenue: Some(Decimal::new(12_000, 0)),   // $12k data processing
    };
    
    // Calculate DockLock revenue
    let docklock_revenue = engine.calculate_docklock_revenue(&docklock_job).await.expect("DockLock calculation failed");
    let expected_total = Decimal::new(70_000, 0); // $25k + $15k + $10k + $8k + $12k
    assert_eq!(docklock_revenue, expected_total);
    
    // Test base fee split
    let base_fee_split = engine.calculate_poe_fee_split(docklock_job.gold_equivalent_value).await.expect("Fee split failed");
    
    // Calculate owner salary with DockLock revenue
    let docklock_owner_share = docklock_revenue * Decimal::new(2, 3); // 0.2% of DockLock revenue
    let total_owner_salary = base_fee_split.owner_salary + docklock_owner_share;
    
    // Expected: $100 (0.2% of $50k) + $140 (0.2% of $70k) = $240
    let expected_owner_salary = Decimal::new(100, 0) + Decimal::new(140, 0);
    assert_eq!(total_owner_salary, expected_owner_salary);
    
    // Calculate treasury with DockLock revenue
    let docklock_treasury_share = docklock_revenue * Decimal::new(3, 3); // 0.3% of DockLock revenue
    let total_treasury = base_fee_split.treasury_net + docklock_treasury_share;
    
    // Expected: $150 (0.3% of $50k) + $210 (0.3% of $70k) = $360
    let expected_treasury = Decimal::new(150, 0) + Decimal::new(210, 0);
    assert_eq!(total_treasury, expected_treasury);
    
    println!("✅ DockLock revenue integration test passed");
    println!("   🐳 Total DockLock revenue: ${}", docklock_revenue);
    println!("   💼 Owner salary (base + DockLock): ${} (${} + ${})", 
             total_owner_salary, base_fee_split.owner_salary, docklock_owner_share);
    println!("   🏛️ Treasury (base + DockLock): ${} (${} + ${})", 
             total_treasury, base_fee_split.treasury_net, docklock_treasury_share);
}

#[tokio::test]
async fn test_owner_salary_fee_split_verification() {
    let registry = Registry::new();
    let engine = PoEMiningEngine::new(&registry).expect("Failed to create engine");
    
    // Test job with $10,000 gold-equivalent value
    let job_value = Decimal::new(10_000, 0);
    let fee_split = engine.calculate_poe_fee_split(job_value).await.expect("Fee split failed");
    
    // Verify 1% total fee split
    assert_eq!(fee_split.total_fee, Decimal::new(100, 0)); // $100 total fee
    
    // Verify miner share (0.5% total)
    assert_eq!(fee_split.miner_locked_reserve, Decimal::new(20, 0));  // 0.2% = $20
    assert_eq!(fee_split.miner_spendable, Decimal::new(30, 0));       // 0.3% = $30
    
    // Verify NEW owner salary (0.2%)
    assert_eq!(fee_split.owner_salary, Decimal::new(20, 0));          // 0.2% = $20
    
    // Verify treasury net (0.3% - reduced from 0.5%)
    assert_eq!(fee_split.treasury_net, Decimal::new(30, 0));          // 0.3% = $30
    
    // Verify total adds up to 1%
    let total_calculated = fee_split.miner_locked_reserve + fee_split.miner_spendable 
                         + fee_split.owner_salary + fee_split.treasury_net;
    assert_eq!(total_calculated, fee_split.total_fee);
    
    println!("✅ Owner salary fee split verification passed");
    println!("   💰 Miner total: ${} (${} locked + ${} spendable)", 
             fee_split.miner_locked_reserve + fee_split.miner_spendable,
             fee_split.miner_locked_reserve, fee_split.miner_spendable);
    println!("   💼 Owner salary: ${} (0.2%)", fee_split.owner_salary);
    println!("   🏛️ Treasury net: ${} (0.3%)", fee_split.treasury_net);
}

#[tokio::test]
async fn test_comprehensive_docklock_revenue_streams() {
    let registry = Registry::new();
    let engine = PoEMiningEngine::new(&registry).expect("Failed to create engine");
    
    // Test individual revenue streams
    let test_cases = vec![
        ("cluster_rent_only", Some(Decimal::new(10_000, 0)), None, None, None, None),
        ("gas_fees_only", None, Some(Decimal::new(5_000, 0)), None, None, None),
        ("app_interactions_only", None, None, Some(Decimal::new(3_000, 0)), None, None),
        ("security_only", None, None, None, Some(Decimal::new(2_000, 0)), None),
        ("pipeline_only", None, None, None, None, Some(Decimal::new(1_500, 0))),
    ];
    
    for (test_name, cluster, gas, app, security, pipeline) in test_cases {
        let job = EconomicJob {
            job_id: format!("test_{}", test_name),
            job_type: EconomicJobType::DockLockHosting,
            miner_id: "test_miner".to_string(),
            gold_equivalent_value: Decimal::new(10_000, 0),
            proof_hash: format!("proof_{}", test_name),
            completion_time: Utc::now(),
            cluster_rent_revenue: cluster,
            gas_fee_revenue: gas,
            app_interaction_revenue: app,
            security_layer_revenue: security,
            data_pipeline_revenue: pipeline,
        };
        
        let docklock_revenue = engine.calculate_docklock_revenue(&job).await.expect("DockLock calculation failed");
        let expected = cluster.unwrap_or(Decimal::ZERO) + gas.unwrap_or(Decimal::ZERO) + 
                      app.unwrap_or(Decimal::ZERO) + security.unwrap_or(Decimal::ZERO) + 
                      pipeline.unwrap_or(Decimal::ZERO);
        
        assert_eq!(docklock_revenue, expected);
        println!("✅ {} - DockLock revenue: ${}", test_name, docklock_revenue);
    }
    
    println!("✅ Comprehensive DockLock revenue streams test passed");
}
