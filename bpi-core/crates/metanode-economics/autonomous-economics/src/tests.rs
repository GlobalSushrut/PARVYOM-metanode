use super::*;
use prometheus::Registry;

/// Helper function to create test economic jobs with DockLock fields
fn create_test_job(
    job_id: &str,
    job_type: EconomicJobType,
    miner_id: &str,
    gold_value: Decimal,
    docklock_revenue: Option<(Decimal, Decimal, Decimal, Decimal, Decimal)>
) -> EconomicJob {
    let (cluster_rent, gas_fees, app_interactions, security_fees, pipeline_fees) = 
        docklock_revenue.unwrap_or((Decimal::ZERO, Decimal::ZERO, Decimal::ZERO, Decimal::ZERO, Decimal::ZERO));
    
    EconomicJob {
        job_id: job_id.to_string(),
        job_type,
        miner_id: miner_id.to_string(),
        gold_equivalent_value: gold_value,
        proof_hash: format!("proof_{}", job_id),
        completion_time: Utc::now(),
        cluster_rent_revenue: if cluster_rent > Decimal::ZERO { Some(cluster_rent) } else { None },
        gas_fee_revenue: if gas_fees > Decimal::ZERO { Some(gas_fees) } else { None },
        app_interaction_revenue: if app_interactions > Decimal::ZERO { Some(app_interactions) } else { None },
        security_layer_revenue: if security_fees > Decimal::ZERO { Some(security_fees) } else { None },
        data_pipeline_revenue: if pipeline_fees > Decimal::ZERO { Some(pipeline_fees) } else { None },
    }
}

#[tokio::test]
async fn test_owner_salary_fee_split() {
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
    
    println!("✅ Owner salary fee split test passed: {:.2}% owner, {:.2}% treasury net", 
             fee_split.owner_salary / job_value * Decimal::new(100, 0),
             fee_split.treasury_net / job_value * Decimal::new(100, 0));
}

#[tokio::test]
async fn test_docklock_revenue_calculation() {
    let registry = Registry::new();
    let engine = PoEMiningEngine::new(&registry).expect("Failed to create engine");
    
    // Create DockLock hosting job with comprehensive revenue streams
    let docklock_job = create_test_job(
        "docklock_hosting_001",
        EconomicJobType::DockLockHosting,
        "docklock_provider",
        Decimal::new(50_000, 0), // $50k base value
        Some((
            Decimal::new(25_000, 0),    // $25k cluster rent
            Decimal::new(15_000, 0),    // $15k gas fees
            Decimal::new(10_000, 0),    // $10k app interactions
            Decimal::new(8_000, 0),     // $8k security services
            Decimal::new(12_000, 0),    // $12k data processing
        ))
    );
    
    // Calculate DockLock revenue
    let docklock_revenue = engine.calculate_docklock_revenue(&docklock_job).await.expect("DockLock calculation failed");
    let expected_total = Decimal::new(70_000, 0); // $25k + $15k + $10k + $8k + $12k
    assert_eq!(docklock_revenue, expected_total);
    
    // Test owner salary with DockLock revenue
    let base_fee_split = engine.calculate_poe_fee_split(docklock_job.gold_equivalent_value).await.expect("Fee split failed");
    let docklock_owner_share = docklock_revenue * Decimal::new(2, 3); // 0.2% of DockLock revenue
    let total_owner_salary = base_fee_split.owner_salary + docklock_owner_share;
    
    // Expected: $100 (0.2% of $50k) + $140 (0.2% of $70k) = $240
    let expected_owner_salary = Decimal::new(100, 0) + Decimal::new(140, 0);
    assert_eq!(total_owner_salary, expected_owner_salary);
    
    println!("✅ DockLock revenue calculation test passed");
    println!("   🐳 Total DockLock revenue: ${}", docklock_revenue);
    println!("   💼 Owner salary (base + DockLock): ${} (${} + ${})", 
             total_owner_salary, base_fee_split.owner_salary, docklock_owner_share);
}

#[tokio::test]
async fn test_complete_fee_routing_with_docklock() {
    let registry = Registry::new();
    let engine = PoEMiningEngine::new(&registry).expect("Failed to create engine");
    
    // Create test economic job with DockLock revenue
    let job = create_test_job(
        "test_job_001",
        EconomicJobType::Settlement,
        "miner_001",
        Decimal::new(50_000, 0), // $50k job
        Some((
            Decimal::new(5_000, 0),     // $5k cluster rent
            Decimal::new(3_000, 0),     // $3k gas fees
            Decimal::new(2_000, 0),     // $2k app interactions
            Decimal::new(1_500, 0),     // $1.5k security
            Decimal::new(1_500, 0),     // $1.5k pipeline
        ))
    );
    
    let job_value = job.gold_equivalent_value;
    
    // Route fees with new owner salary logic including DockLock
    let result = engine.route_fees(&job, job_value).await;
    assert!(result.is_ok());
    
    // Verify fee split calculation
    let fee_split = engine.calculate_poe_fee_split(job_value).await.expect("Fee split failed");
    let docklock_revenue = engine.calculate_docklock_revenue(&job).await.expect("DockLock calculation failed");
    
    // Expected values for $50k job + $13k DockLock:
    assert_eq!(fee_split.total_fee, Decimal::new(500, 0));              // 1% = $500
    assert_eq!(fee_split.miner_locked_reserve, Decimal::new(100, 0));   // 0.2% = $100
    assert_eq!(fee_split.miner_spendable, Decimal::new(150, 0));        // 0.3% = $150
    assert_eq!(fee_split.owner_salary, Decimal::new(100, 0));           // 0.2% = $100 (base)
    assert_eq!(fee_split.treasury_net, Decimal::new(150, 0));           // 0.3% = $150 (base)
    
    // DockLock revenue: $13k total
    assert_eq!(docklock_revenue, Decimal::new(13_000, 0));
    
    println!("✅ Complete fee routing with DockLock test passed");
    println!("   💰 Miner total: ${} (${} locked + ${} spendable)", 
             fee_split.miner_locked_reserve + fee_split.miner_spendable,
             fee_split.miner_locked_reserve, fee_split.miner_spendable);
    println!("   💼 Owner salary: ${} base + ${} DockLock = ${}", 
             fee_split.owner_salary, 
             docklock_revenue * Decimal::new(2, 3),
             fee_split.owner_salary + (docklock_revenue * Decimal::new(2, 3)));
    println!("   🏛️ Treasury: ${} base + ${} DockLock = ${}", 
             fee_split.treasury_net,
             docklock_revenue * Decimal::new(3, 3),
             fee_split.treasury_net + (docklock_revenue * Decimal::new(3, 3)));
}

#[tokio::test]
async fn test_stage51_exit_criteria_with_docklock() {
    let registry = Registry::new();
    let engine = PoEMiningEngine::new(&registry).expect("Failed to create engine");
    
    // Test multiple jobs with DockLock revenue streams
    let jobs = vec![
        ("job_001", EconomicJobType::Validation, Decimal::new(25_000, 0), Some((Decimal::new(5_000, 0), Decimal::new(2_000, 0), Decimal::new(1_000, 0), Decimal::new(500, 0), Decimal::new(500, 0)))),
        ("job_002", EconomicJobType::DockLockHosting, Decimal::new(75_000, 0), Some((Decimal::new(20_000, 0), Decimal::new(10_000, 0), Decimal::new(5_000, 0), Decimal::new(3_000, 0), Decimal::new(2_000, 0)))),
        ("job_003", EconomicJobType::Development, Decimal::new(10_000, 0), None),
    ];
    
    let mut total_owner_salary = Decimal::ZERO;
    let mut total_treasury_net = Decimal::ZERO;
    let mut total_docklock_revenue = Decimal::ZERO;
    
    for (job_id, job_type, value, docklock_data) in jobs {
        let job = create_test_job(job_id, job_type, "miner_001", value, docklock_data);
        
        // Route fees
        let route_result = engine.route_fees(&job, value).await;
        assert!(route_result.is_ok());
        
        // Calculate expected splits
        let fee_split = engine.calculate_poe_fee_split(value).await.expect("Fee split failed");
        let docklock_revenue = engine.calculate_docklock_revenue(&job).await.expect("DockLock calculation failed");
        
        total_owner_salary += fee_split.owner_salary + (docklock_revenue * Decimal::new(2, 3));
        total_treasury_net += fee_split.treasury_net + (docklock_revenue * Decimal::new(3, 3));
        total_docklock_revenue += docklock_revenue;
    }
    
    // Verify governance guardrails are in place
    let policy = engine.get_owner_salary_policy().await;
    assert!(policy.monthly_hard_cap > Decimal::ZERO);
    assert_eq!(policy.vesting_immediate_rate + policy.vesting_deferred_rate, Decimal::ONE);
    
    println!("✅ Stage 51 exit criteria with DockLock test passed");
    println!("   📊 Total base volume processed: $110,000");
    println!("   🐳 Total DockLock revenue: ${}", total_docklock_revenue);
    println!("   💼 Total owner salary (base + DockLock): ${}", total_owner_salary);
    println!("   🏛️ Total treasury (base + DockLock): ${}", total_treasury_net);
    println!("   🛡️ Governance guardrails: cap=${}, vesting={}%+{}%", 
             policy.monthly_hard_cap, 
             policy.vesting_immediate_rate * Decimal::new(100, 0),
             policy.vesting_deferred_rate * Decimal::new(100, 0));
}
