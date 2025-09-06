# BPI Ledger Integration - Troubleshooting Guide

## Common Issues and Solutions

### 1. Connection Issues

#### Problem: "Failed to connect to BPI ledger"
**Symptoms**:
- Connection timeouts
- HTTP 503/504 errors
- `is_connected()` returns false

**Diagnosis**:
```rust
// Check connection status
let client = BpiLedgerClient::new().await?;
if !client.is_connected() {
    // Get detailed connection health
    let health_metrics = client.get_connection_health().await?;
    for (conn_id, health_score) in health_metrics {
        println!("Connection {}: Health score {:.2}", conn_id, health_score);
    }
}
```

**Solutions**:
1. **Check network connectivity**:
   ```bash
   # Test basic connectivity
   curl -I https://bpi-ledger-endpoint.com/health
   
   # Check DNS resolution
   nslookup bpi-ledger-endpoint.com
   ```

2. **Verify endpoint configuration**:
   ```rust
   let endpoints = client.discover_bpi_endpoints().await?;
   for (ledger_id, endpoint) in endpoints {
       println!("Ledger {}: {}", ledger_id, endpoint);
   }
   ```

3. **Implement connection retry logic**:
   ```rust
   async fn robust_connection() -> Result<String> {
       let client = BpiLedgerClient::new().await?;
       let mut retries = 0;
       const MAX_RETRIES: u32 = 3;
       
       while retries < MAX_RETRIES {
           match client.connect_to_ledger("main", LedgerConnectionType::Primary).await {
               Ok(conn_id) => return Ok(conn_id),
               Err(e) => {
                   retries += 1;
                   eprintln!("Connection attempt {} failed: {}", retries, e);
                   tokio::time::sleep(Duration::from_secs(2_u64.pow(retries))).await;
               }
           }
       }
       
       Err(anyhow!("Failed to connect after {} retries", MAX_RETRIES))
   }
   ```

#### Problem: "Connection drops frequently"
**Symptoms**:
- Intermittent connection failures
- High error rates in metrics
- Frequent reconnection attempts

**Solutions**:
1. **Implement connection pooling**:
   ```rust
   // Maintain multiple connections for redundancy
   let primary_conn = client.connect_to_ledger("main", LedgerConnectionType::Primary).await?;
   let backup_conn = client.connect_to_ledger("backup", LedgerConnectionType::Secondary).await?;
   
   // Use health-based connection selection
   let health_metrics = client.get_connection_health().await?;
   let best_connection = health_metrics.iter()
       .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
       .map(|(conn_id, _)| conn_id.clone());
   ```

2. **Configure connection timeouts**:
   ```rust
   let http_client = reqwest::Client::builder()
       .timeout(Duration::from_secs(30))
       .connect_timeout(Duration::from_secs(10))
       .build()?;
   ```

### 2. ZK Proof Issues

#### Problem: "ZK proof generation failed"
**Symptoms**:
- Proof generation timeouts
- Invalid proof errors
- Memory allocation failures

**Diagnosis**:
```rust
// Test proof system initialization
let zk_system = ZkProofSystem::new().await?;

// Check verification keys
let verification_keys = zk_system.verification_keys.read().await;
println!("Loaded verification keys: {}", verification_keys.len());

// Test proof generation with small data
let test_data = b"test";
match zk_system.generate_proof(ProofType::TransactionPrivacy, test_data).await {
    Ok(proof) => println!("✅ Proof generation working: {}", proof.proof_id),
    Err(e) => println!("❌ Proof generation failed: {}", e),
}
```

**Solutions**:
1. **Increase memory allocation**:
   ```bash
   # Set environment variables for larger heap
   export RUST_MIN_STACK=8388608  # 8MB stack
   export MALLOC_ARENA_MAX=2      # Limit memory arenas
   ```

2. **Use proof caching**:
   ```rust
   // Check cache before generating new proof
   let cache_key = format!("{:?}_{}", proof_type, hex::encode(data));
   if let Some(cached_proof) = zk_system.get_cached_proof(&cache_key).await? {
       return Ok(cached_proof);
   }
   ```

3. **Implement proof batching**:
   ```rust
   async fn batch_proof_generation(
       zk_system: &ZkProofSystem,
       transactions: Vec<TransactionData>
   ) -> Result<Vec<CachedProof>> {
       let batch_size = 10;
       let mut proofs = Vec::new();
       
       for chunk in transactions.chunks(batch_size) {
           let batch_futures: Vec<_> = chunk.iter()
               .map(|tx| zk_system.generate_proof(ProofType::TransactionPrivacy, &tx.serialize()))
               .collect();
           
           let batch_proofs = futures::future::try_join_all(batch_futures).await?;
           proofs.extend(batch_proofs);
       }
       
       Ok(proofs)
   }
   ```

#### Problem: "Proof verification failed"
**Symptoms**:
- Valid proofs being rejected
- Verification timeouts
- Inconsistent verification results

**Solutions**:
1. **Verify proof integrity**:
   ```rust
   async fn debug_proof_verification(proof: &CachedProof) -> Result<()> {
       println!("Proof ID: {}", proof.proof_id);
       println!("Proof type: {:?}", proof.proof_type);
       println!("Data length: {}", proof.proof_data.len());
       println!("Public inputs: {:?}", proof.public_inputs);
       
       // Check proof format
       if proof.proof_data.is_empty() {
           return Err(anyhow!("Empty proof data"));
       }
       
       // Verify public inputs format
       for input in &proof.public_inputs {
           if input.is_empty() {
               return Err(anyhow!("Empty public input"));
           }
       }
       
       Ok(())
   }
   ```

2. **Update verification keys**:
   ```rust
   // Reload verification keys if verification fails
   let mut verification_keys = zk_system.verification_keys.write().await;
   *verification_keys = ZkProofSystem::load_verification_keys().await?;
   ```

### 3. Economic Coordination Issues

#### Problem: "Cross-ledger transfer stuck"
**Symptoms**:
- Transfers remain in pending state
- Settlement timeouts
- Inconsistent balances across ledgers

**Diagnosis**:
```rust
// Check settlement status
let coordinator = EconomicCoordinator::new().await?;
let metrics = coordinator.get_metrics().await?;

println!("Pending settlements: {}", metrics.pending_settlements);
println!("Failed settlements: {}", metrics.failed_settlements);
println!("Average settlement time: {}s", metrics.average_settlement_time_seconds);

// Check specific settlement
let settlement_status = coordinator.get_settlement_status(&settlement_id).await?;
println!("Settlement status: {:?}", settlement_status);
```

**Solutions**:
1. **Implement settlement monitoring**:
   ```rust
   async fn monitor_settlement(
       coordinator: &EconomicCoordinator,
       settlement_id: &str
   ) -> Result<SettlementStatus> {
       let timeout = Duration::from_secs(300); // 5 minutes
       let start = Instant::now();
       
       loop {
           let status = coordinator.get_settlement_status(settlement_id).await?;
           
           match status {
               SettlementStatus::Completed => return Ok(status),
               SettlementStatus::Failed => return Err(anyhow!("Settlement failed")),
               _ => {
                   if start.elapsed() > timeout {
                       return Err(anyhow!("Settlement timeout"));
                   }
                   tokio::time::sleep(Duration::from_secs(5)).await;
               }
           }
       }
   }
   ```

2. **Implement settlement recovery**:
   ```rust
   async fn recover_stuck_settlement(
       coordinator: &EconomicCoordinator,
       settlement_id: &str
   ) -> Result<()> {
       // Cancel stuck settlement
       coordinator.cancel_settlement(settlement_id).await?;
       
       // Retry with new settlement
       let new_settlement_id = coordinator.initiate_settlement(
           &original_source,
           &original_target,
           original_amount,
           &original_token_type
       ).await?;
       
       println!("Recovery settlement initiated: {}", new_settlement_id);
       Ok(())
   }
   ```

### 4. Performance Issues

#### Problem: "High latency in transaction processing"
**Symptoms**:
- Slow transaction confirmations
- High average response times
- Timeout errors

**Diagnosis**:
```rust
// Monitor performance metrics
let connections = client.ledger_connections.read().await;
for (conn_id, connection) in connections.iter() {
    let metrics = &connection.performance_metrics;
    println!("Connection {}: Latency {}ms, TPS {}, Error rate {:.2}%",
        conn_id,
        metrics.average_latency_ms,
        metrics.transactions_per_second,
        metrics.error_rate * 100.0
    );
}
```

**Solutions**:
1. **Implement connection load balancing**:
   ```rust
   async fn select_best_connection(
       client: &BpiLedgerClient
   ) -> Result<String> {
       let health_metrics = client.get_connection_health().await?;
       
       let best_connection = health_metrics.iter()
           .filter(|(_, health)| **health > 0.7) // Only healthy connections
           .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
           .map(|(conn_id, _)| conn_id.clone())
           .ok_or_else(|| anyhow!("No healthy connections available"))?;
       
       Ok(best_connection)
   }
   ```

2. **Use parallel processing**:
   ```rust
   async fn parallel_transaction_processing(
       client: &BpiLedgerClient,
       transactions: Vec<serde_json::Value>
   ) -> Result<Vec<TransactionResult>> {
       let semaphore = Arc::new(Semaphore::new(10)); // Limit concurrent requests
       let futures: Vec<_> = transactions.into_iter()
           .map(|tx| {
               let client = client.clone();
               let semaphore = semaphore.clone();
               async move {
                   let _permit = semaphore.acquire().await.unwrap();
                   client.submit_transaction_with_proof(&conn_id, tx, None).await
               }
           })
           .collect();
       
       let results = futures::future::try_join_all(futures).await?;
       Ok(results)
   }
   ```

## Debugging Tools

### 1. Connection Health Monitor
```rust
async fn connection_health_monitor(client: Arc<BpiLedgerClient>) {
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    
    loop {
        interval.tick().await;
        
        match client.get_connection_health().await {
            Ok(health_metrics) => {
                for (conn_id, health_score) in health_metrics {
                    if health_score < 0.5 {
                        eprintln!("⚠️ Connection {} unhealthy: {:.2}", conn_id, health_score);
                    } else {
                        println!("✅ Connection {} healthy: {:.2}", conn_id, health_score);
                    }
                }
            }
            Err(e) => eprintln!("Failed to get health metrics: {}", e),
        }
    }
}
```

### 2. ZK Proof Validator
```rust
async fn validate_proof_system() -> Result<()> {
    let zk_system = ZkProofSystem::new().await?;
    
    // Test all proof types
    let test_cases = vec![
        (ProofType::TransactionPrivacy, b"test_transaction".as_slice()),
        (ProofType::BalanceVerification, b"test_balance".as_slice()),
        (ProofType::IdentityProof, b"test_identity".as_slice()),
        (ProofType::MembershipProof, b"test_membership".as_slice()),
    ];
    
    for (proof_type, test_data) in test_cases {
        println!("Testing proof type: {:?}", proof_type);
        
        let start = Instant::now();
        let proof = zk_system.generate_proof(proof_type, test_data).await?;
        let generation_time = start.elapsed();
        
        let start = Instant::now();
        let is_valid = zk_system.verify_proof(&proof).await?;
        let verification_time = start.elapsed();
        
        println!("  Generation: {:?}, Verification: {:?}, Valid: {}",
            generation_time, verification_time, is_valid);
        
        if !is_valid {
            return Err(anyhow!("Proof validation failed for {:?}", proof_type));
        }
    }
    
    println!("✅ All proof types validated successfully");
    Ok(())
}
```

### 3. Economic Metrics Dashboard
```rust
async fn economic_metrics_dashboard(coordinator: Arc<EconomicCoordinator>) {
    let mut interval = tokio::time::interval(Duration::from_secs(60));
    
    loop {
        interval.tick().await;
        
        match coordinator.get_metrics().await {
            Ok(metrics) => {
                println!("\n=== Economic Metrics ===");
                println!("Total Value Locked: ${:.2}", metrics.total_value_locked);
                println!("Transactions per Second: {}", metrics.transactions_per_second);
                println!("Pending Settlements: {}", metrics.pending_settlements);
                println!("Completed Today: {}", metrics.settlements_completed_today);
                println!("Average Settlement Time: {:.2}s", metrics.average_settlement_time_seconds);
                println!("Success Rate: {:.1}%", metrics.settlement_success_rate * 100.0);
            }
            Err(e) => eprintln!("Failed to get economic metrics: {}", e),
        }
    }
}
```

## Emergency Procedures

### 1. System Recovery
```bash
#!/bin/bash
# Emergency recovery script

echo "🚨 Starting BPI Ledger emergency recovery..."

# 1. Check system resources
echo "Checking system resources..."
free -h
df -h

# 2. Restart services
echo "Restarting BPI services..."
systemctl restart bpci-ledger-service

# 3. Clear proof cache if corrupted
echo "Clearing proof cache..."
rm -rf /tmp/zk_proof_cache/*

# 4. Test basic connectivity
echo "Testing connectivity..."
curl -f http://localhost:8080/health || echo "❌ Health check failed"

echo "✅ Recovery complete"
```

### 2. Data Backup
```rust
async fn emergency_backup() -> Result<()> {
    let client = BpiLedgerClient::new().await?;
    
    // Backup connection states
    let connections = client.ledger_connections.read().await;
    let backup_data = serde_json::to_string_pretty(&*connections)?;
    
    let backup_file = format!("ledger_backup_{}.json", Utc::now().timestamp());
    tokio::fs::write(&backup_file, backup_data).await?;
    
    println!("✅ Backup saved to: {}", backup_file);
    Ok(())
}
```

---

**Previous**: [Real Code Implementation](03-real-code-implementation.md)  
**Related**: [Performance Optimization](../32-performance-optimization/), [Security Auditing](../31-security-auditing/)
