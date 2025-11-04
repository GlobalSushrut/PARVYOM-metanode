# 🧪 Complete BPCI Test Suite - Simple to Advanced

**Date**: 2025-10-27  
**Status**: ✅ ALL 10 COMPONENTS READY FOR TESTING  
**Coverage**: End-to-End, Component-Level, Integration, Performance, Load Testing

---

## 🎯 **Test Overview**

This document provides **complete testing coverage** for the entire BPCI infrastructure:

### **Test Levels**:
1. **🟢 Simple Tests** - Basic functionality, health checks
2. **🟡 Intermediate Tests** - Component integration, transaction flow
3. **🔴 Advanced Tests** - Performance, load testing, stress testing
4. **🟣 Expert Tests** - Multi-node, cluster coordination, failure scenarios

### **Test Categories**:
- **Component Tests** - Individual component validation
- **Integration Tests** - Component-to-component communication
- **Transaction Tests** - End-to-end BPI transaction flow
- **Performance Tests** - Throughput, latency, scalability
- **Reliability Tests** - Failure recovery, error handling
- **Security Tests** - Authentication, authorization, encryption

---

## 🟢 **LEVEL 1: Simple Tests (Basic Functionality)**

### **Test 1.1: Component Health Checks**

**Objective**: Verify all 10 components are running and responsive

**Prerequisites**: All components started in Pure Virtual Mode

**Test Steps**:
```bash
# Start all components (in separate terminals)
cargo run --bin bpci-consensus-server &
cargo run --bin bpci_blockchain_server &
cargo run --bin bpci_auction_mempool_server &
cargo run --bin bpci_auction_db_maintainer &
cargo run --bin bpci_bpi_bridge &
cargo run --bin bpci_cluster_ledger_server &
cargo run --bin bpci_xtmp_server &
cargo run --bin bpci_shadow_registry_server &
cargo run --bin community_installer_web &
cargo run --bin bpci_network_server &

# Wait 30 seconds for all components to initialize

# Test 1: Check process status
ps aux | grep bpci | grep -v grep
# Expected: 10 processes running

# Test 2: Check Pure Virtual Mode (NO static ports)
netstat -tuln | grep -E "(9001|8080|9004|7002|6001|7000|8088|8087)"
# Expected: NO matches (all ports are dynamic!)

# Test 3: Check dynamic ports ARE allocated
netstat -tuln | grep LISTEN | wc -l
# Expected: At least 10 listening ports (dynamic)
```

**Expected Results**:
- ✅ All 10 components running
- ✅ NO static ports in use
- ✅ Dynamic ports allocated for each component
- ✅ Pure Virtual Mode confirmed

**Pass Criteria**: All components running, no static ports, dynamic ports allocated

---

### **Test 1.2: Service Discovery Validation**

**Objective**: Verify all components can discover each other by service name

**Test Steps**:
```bash
# Run service discovery test
cargo run --bin test_component_6

# Check logs for service discovery
tail -f /tmp/bpci_test.log | grep "Discovered"
```

**Expected Output**:
```
✅ Discovered consensus: 1 endpoints
✅ Discovered blockchain: 1 endpoints  
✅ Discovered auction: 1 endpoints
✅ Discovered db-manager: 1 endpoints
✅ Discovered bridge: 1 endpoints
✅ Discovered cluster-ledger: 1 endpoints
✅ Discovered xtmp: 1 endpoints
✅ Discovered shadow-registry: 1 endpoints
✅ Discovered web: 1 endpoints
✅ Discovered network: 1 endpoints
```

**Pass Criteria**: All 10 services discovered successfully

---

### **Test 1.3: Basic API Connectivity**

**Objective**: Test basic HTTP API endpoints for each component

**Test Steps**:
```bash
# Get dynamic ports for each component
CONSENSUS_PORT=$(netstat -tuln | grep bpci-consensus | awk '{print $4}' | cut -d: -f2)
BLOCKCHAIN_PORT=$(netstat -tuln | grep bpci_blockchain | awk '{print $4}' | cut -d: -f2)
BRIDGE_PORT=$(netstat -tuln | grep bpci_bpi_bridge | awk '{print $4}' | cut -d: -f2)

# Test health endpoints
curl -s http://localhost:$CONSENSUS_PORT/health | jq .
curl -s http://localhost:$BLOCKCHAIN_PORT/health | jq .
curl -s http://localhost:$BRIDGE_PORT/health | jq .

# Test configuration endpoints
curl -s http://localhost:$CONSENSUS_PORT/api/v1/config | jq .
curl -s http://localhost:$BLOCKCHAIN_PORT/api/v1/config | jq .
```

**Expected Results**:
- ✅ All health endpoints return 200 OK
- ✅ JSON responses with component status
- ✅ Configuration endpoints accessible

**Pass Criteria**: All API endpoints responsive with valid JSON

---

## 🟡 **LEVEL 2: Intermediate Tests (Integration)**

### **Test 2.1: Component-to-Component Communication**

**Objective**: Test Pure Virtual Mode communication between components

**Test Steps**:
```bash
# Create test communication script
cat > test_component_communication.sh << 'EOF'
#!/bin/bash

echo "Testing Component Communication..."

# Test 1: Bridge → Consensus
echo "1. Testing Bridge → Consensus communication"
curl -X POST http://localhost:$BRIDGE_PORT/api/test/consensus \
  -H "Content-Type: application/json" \
  -d '{"test": "ping"}' | jq .

# Test 2: Bridge → Blockchain  
echo "2. Testing Bridge → Blockchain communication"
curl -X POST http://localhost:$BRIDGE_PORT/api/test/blockchain \
  -H "Content-Type: application/json" \
  -d '{"test": "ping"}' | jq .

# Test 3: Bridge → Auction
echo "3. Testing Bridge → Auction communication"
curl -X POST http://localhost:$BRIDGE_PORT/api/test/auction \
  -H "Content-Type: application/json" \
  -d '{"test": "ping"}' | jq .

# Test 4: Bridge → DB Manager
echo "4. Testing Bridge → DB Manager communication"
curl -X POST http://localhost:$BRIDGE_PORT/api/test/db-manager \
  -H "Content-Type: application/json" \
  -d '{"test": "ping"}' | jq .

# Test 5: Bridge → Cluster Ledger
echo "5. Testing Bridge → Cluster Ledger communication"
curl -X POST http://localhost:$BRIDGE_PORT/api/test/cluster-ledger \
  -H "Content-Type: application/json" \
  -d '{"test": "ping"}' | jq .

EOF

chmod +x test_component_communication.sh
./test_component_communication.sh
```

**Expected Results**:
- ✅ All component communications successful
- ✅ Service name resolution working
- ✅ Pure Virtual Mode messaging operational

**Pass Criteria**: All 5 component communications return success

---

### **Test 2.2: Simple BPI Transaction Flow**

**Objective**: Test complete BPI transaction processing through all components

**Test Steps**:
```bash
# Test transaction processing
curl -X POST http://localhost:$BRIDGE_PORT/api/transaction \
  -H "Content-Type: application/json" \
  -d '{
    "from_bpi": "test_wallet_alice",
    "to_bpci": "test_wallet_bob", 
    "amount": 100,
    "currency": "BPI"
  }' | jq .
```

**Expected Response**:
```json
{
  "tx_id": "tx_550e8400-e29b-41d4-a716-446655440000",
  "status": "processing",
  "amount": 100,
  "gas_fee": 5,
  "total_cost": 105,
  "components_processed": [
    "consensus",
    "blockchain", 
    "auction",
    "db-manager"
  ],
  "timestamp": "2025-10-27T21:30:00Z"
}
```

**Verification Steps**:
```bash
# Check component logs for transaction processing
grep "tx_550e8400" /tmp/bpci_*.log

# Verify transaction in each component
curl -s http://localhost:$CONSENSUS_PORT/api/v1/transactions/tx_550e8400 | jq .
curl -s http://localhost:$BLOCKCHAIN_PORT/api/v1/transactions/tx_550e8400 | jq .
curl -s http://localhost:$AUCTION_PORT/api/v1/transactions/tx_550e8400 | jq .
```

**Pass Criteria**: Transaction processed through all 4 components successfully

---

## 🔴 **LEVEL 3: Advanced Tests (Performance & Load)**

### **Test 3.1: Transaction Throughput Test**

**Objective**: Measure maximum transaction processing capacity

**Test Setup**:
```bash
# Create load testing script
cat > load_test_transactions.py << 'EOF'
#!/usr/bin/env python3
import asyncio
import aiohttp
import time
import json
from concurrent.futures import ThreadPoolExecutor

async def send_transaction(session, tx_id):
    """Send a single transaction"""
    url = f"http://localhost:{BRIDGE_PORT}/api/transaction"
    payload = {
        "from_bpi": f"load_test_wallet_{tx_id}",
        "to_bpci": f"target_wallet_{tx_id}",
        "amount": 10,
        "currency": "BPI"
    }
    
    try:
        async with session.post(url, json=payload) as response:
            result = await response.json()
            return {"success": True, "tx_id": result.get("tx_id"), "time": time.time()}
    except Exception as e:
        return {"success": False, "error": str(e), "time": time.time()}

async def load_test(num_transactions, concurrent_requests):
    """Run load test with specified parameters"""
    print(f"Starting load test: {num_transactions} transactions, {concurrent_requests} concurrent")
    
    start_time = time.time()
    results = []
    
    async with aiohttp.ClientSession() as session:
        # Create semaphore to limit concurrent requests
        semaphore = asyncio.Semaphore(concurrent_requests)
        
        async def bounded_send(tx_id):
            async with semaphore:
                return await send_transaction(session, tx_id)
        
        # Send all transactions
        tasks = [bounded_send(i) for i in range(num_transactions)]
        results = await asyncio.gather(*tasks)
    
    end_time = time.time()
    duration = end_time - start_time
    
    # Analyze results
    successful = sum(1 for r in results if r["success"])
    failed = len(results) - successful
    tps = successful / duration if duration > 0 else 0
    
    print(f"\n=== LOAD TEST RESULTS ===")
    print(f"Duration: {duration:.2f} seconds")
    print(f"Total Transactions: {len(results)}")
    print(f"Successful: {successful}")
    print(f"Failed: {failed}")
    print(f"Success Rate: {(successful/len(results)*100):.1f}%")
    print(f"Transactions/Second: {tps:.2f}")
    print(f"Average Latency: {(duration/len(results)*1000):.2f}ms")
    
    return {
        "duration": duration,
        "total": len(results),
        "successful": successful,
        "failed": failed,
        "tps": tps,
        "success_rate": successful/len(results)*100
    }

if __name__ == "__main__":
    # Test scenarios
    scenarios = [
        {"transactions": 10, "concurrent": 2},    # Light load
        {"transactions": 50, "concurrent": 5},    # Medium load  
        {"transactions": 100, "concurrent": 10},  # Heavy load
        {"transactions": 500, "concurrent": 20},  # Stress test
    ]
    
    for scenario in scenarios:
        print(f"\n{'='*50}")
        result = asyncio.run(load_test(scenario["transactions"], scenario["concurrent"]))
        time.sleep(5)  # Cool down between tests
EOF

python3 load_test_transactions.py
```

**Expected Performance Targets**:
- **Light Load**: >95% success rate, <100ms avg latency
- **Medium Load**: >90% success rate, <200ms avg latency  
- **Heavy Load**: >85% success rate, <500ms avg latency
- **Stress Test**: >70% success rate, <1000ms avg latency

**Pass Criteria**: All scenarios meet minimum performance targets

---

### **Test 3.2: Cluster Coordination Test**

**Objective**: Test BPI node registration and coordination via Cluster Ledger

**Test Steps**:
```bash
# Test BPI node registration
curl -X POST http://localhost:$CLUSTER_LEDGER_PORT/api/v1/bpi/register \
  -H "Content-Type: application/json" \
  -d '{
    "node_id": "test_bpi_node_001",
    "node_name": "Test BPI Node 1",
    "endpoint": "192.168.1.100:8080",
    "capabilities": {
      "max_concurrent_connections": 1000,
      "supported_protocols": ["HTTP", "WebSocket", "CBOR"],
      "processing_capacity": 100.0,
      "storage_capacity": 1073741824,
      "network_bandwidth": 1000000000,
      "security_level": "High"
    },
    "resource_allocation": {
      "cpu_cores": 4,
      "memory_gb": 8,
      "storage_gb": 100
    }
  }' | jq .

# Test load distribution coordination
curl -X POST http://localhost:$CLUSTER_LEDGER_PORT/api/v1/distribution/coordinate \
  -H "Content-Type: application/json" \
  -d '{
    "target_nodes": ["test_bpi_node_001", "test_bpi_node_002"],
    "distribution_type": "cluster_ledger_coordination"
  }' | jq .

# Verify node registry
curl -s http://localhost:$CLUSTER_LEDGER_PORT/api/v1/bpi/nodes | jq .
```

**Expected Results**:
- ✅ BPI node registered successfully
- ✅ Load distribution coordinated
- ✅ Node appears in registry with correct capabilities

**Pass Criteria**: Node registration and coordination successful

---

## 🟣 **LEVEL 4: Expert Tests (Advanced Scenarios)**

### **Test 4.1: Failure Recovery Test**

**Objective**: Test system behavior when components fail and recover

**Test Scenario**: Consensus Server Failure
```bash
# Step 1: Kill Consensus Server
CONSENSUS_PID=$(ps aux | grep bpci-consensus-server | grep -v grep | awk '{print $2}')
kill $CONSENSUS_PID

# Step 2: Attempt transaction (should fail)
curl -X POST http://localhost:$BRIDGE_PORT/api/transaction \
  -H "Content-Type: application/json" \
  -d '{
    "from_bpi": "test_wallet_failure",
    "to_bpci": "test_wallet_target",
    "amount": 50,
    "currency": "BPI"
  }' | jq .

# Expected: {"error": "Consensus not ready for transaction processing"}

# Step 3: Restart Consensus Server
cargo run --bin bpci-consensus-server &
sleep 10

# Step 4: Retry transaction (should succeed)
curl -X POST http://localhost:$BRIDGE_PORT/api/transaction \
  -H "Content-Type: application/json" \
  -d '{
    "from_bpi": "test_wallet_recovery", 
    "to_bpci": "test_wallet_target",
    "amount": 50,
    "currency": "BPI"
  }' | jq .

# Expected: Successful transaction processing
```

**Pass Criteria**: 
- ✅ Transaction fails gracefully when consensus down
- ✅ Transaction succeeds after consensus recovery
- ✅ No data corruption or system instability

---

### **Test 4.2: Multi-Instance Scaling Test**

**Objective**: Test multiple instances of components running simultaneously

**Test Setup**:
```bash
# Start multiple instances of key components
for i in {1..3}; do
  RUST_LOG=info cargo run --bin bpci_blockchain_server &
  sleep 2
done

for i in {1..3}; do  
  RUST_LOG=info cargo run --bin bpci_auction_mempool_server &
  sleep 2
done

# Verify multiple instances running
ps aux | grep bpci_blockchain_server | grep -v grep | wc -l
# Expected: 3

ps aux | grep bpci_auction_mempool_server | grep -v grep | wc -l  
# Expected: 3

# Test load distribution across instances
for i in {1..30}; do
  curl -X POST http://localhost:$BRIDGE_PORT/api/transaction \
    -H "Content-Type: application/json" \
    -d "{
      \"from_bpi\": \"multi_test_wallet_$i\",
      \"to_bpci\": \"target_wallet_$i\",
      \"amount\": 25,
      \"currency\": \"BPI\"
    }" &
done

wait
```

**Expected Results**:
- ✅ Multiple instances running without conflicts
- ✅ Load distributed across instances
- ✅ All transactions processed successfully
- ✅ No port conflicts (Pure Virtual Mode)

**Pass Criteria**: All instances operational, transactions distributed and processed

---

### **Test 4.3: CBOR WebSocket Streaming Test**

**Objective**: Test real-time CBOR transaction streaming

**Test Steps**:
```bash
# Create WebSocket test client
cat > test_cbor_streaming.js << 'EOF'
const WebSocket = require('ws');

// Connect to CBOR WebSocket endpoint
const ws = new WebSocket(`ws://localhost:${BRIDGE_PORT}/ws/cbor`);

ws.on('open', function open() {
  console.log('✅ Connected to CBOR WebSocket stream');
  
  // Send test transaction
  const testTx = {
    action: 'submit_transaction',
    from_bpi: 'websocket_test_wallet',
    to_bpci: 'websocket_target_wallet', 
    amount: 75,
    cbor_data: Buffer.from('test cbor data').toString('base64')
  };
  
  ws.send(JSON.stringify(testTx));
});

ws.on('message', function message(data) {
  const response = JSON.parse(data);
  console.log('📨 Received CBOR response:', response);
  
  if (response.tx_id) {
    console.log('✅ Transaction ID:', response.tx_id);
    console.log('✅ Status:', response.status);
    console.log('✅ CBOR processed:', response.cbor_processed);
  }
});

ws.on('error', function error(err) {
  console.error('❌ WebSocket error:', err);
});

ws.on('close', function close() {
  console.log('🔌 WebSocket connection closed');
});

// Keep connection alive for 30 seconds
setTimeout(() => {
  ws.close();
}, 30000);
EOF

node test_cbor_streaming.js
```

**Expected Output**:
```
✅ Connected to CBOR WebSocket stream
📨 Received CBOR response: {
  "tx_id": "tx_cbor_550e8400-e29b-41d4-a716-446655440000",
  "status": "processing",
  "cbor_processed": true,
  "auction_group": "auction_1698451234",
  "components_notified": ["consensus", "blockchain", "auction", "db-manager"]
}
✅ Transaction ID: tx_cbor_550e8400-e29b-41d4-a716-446655440000
✅ Status: processing
✅ CBOR processed: true
🔌 WebSocket connection closed
```

**Pass Criteria**: WebSocket connection established, CBOR data processed and streamed

---

## 📊 **Test Execution Matrix**

### **Test Execution Order**:

| Level | Test | Duration | Prerequisites | Critical |
|-------|------|----------|---------------|----------|
| 🟢 | 1.1 Health Checks | 2 min | All components started | ✅ |
| 🟢 | 1.2 Service Discovery | 3 min | Health checks pass | ✅ |
| 🟢 | 1.3 API Connectivity | 5 min | Service discovery pass | ✅ |
| 🟡 | 2.1 Component Communication | 10 min | API connectivity pass | ✅ |
| 🟡 | 2.2 Simple Transaction | 5 min | Component communication pass | ✅ |
| 🔴 | 3.1 Throughput Test | 20 min | Simple transaction pass | ⚠️ |
| 🔴 | 3.2 Cluster Coordination | 15 min | Simple transaction pass | ⚠️ |
| 🟣 | 4.1 Failure Recovery | 25 min | All previous tests pass | 🔴 |
| 🟣 | 4.2 Multi-Instance Scaling | 30 min | All previous tests pass | 🔴 |
| 🟣 | 4.3 CBOR Streaming | 15 min | All previous tests pass | 🔴 |

**Legend**:
- ✅ **Critical**: Must pass for system to be functional
- ⚠️ **Important**: Should pass for production readiness  
- 🔴 **Advanced**: Optional but recommended for enterprise deployment

---

## 🎯 **Test Automation Script**

### **Complete Test Runner**:
```bash
#!/bin/bash
# complete_bpci_test_runner.sh

echo "🧪 BPCI Complete Test Suite Runner"
echo "=================================="

# Test configuration
TEST_LOG="/tmp/bpci_complete_test_$(date +%Y%m%d_%H%M%S).log"
RESULTS_FILE="/tmp/bpci_test_results_$(date +%Y%m%d_%H%M%S).json"

# Initialize results
echo '{"tests": [], "summary": {}}' > $RESULTS_FILE

run_test() {
    local level=$1
    local test_name=$2
    local test_command=$3
    local critical=$4
    
    echo "🔄 Running $level.$test_name..."
    start_time=$(date +%s)
    
    if eval "$test_command" >> $TEST_LOG 2>&1; then
        end_time=$(date +%s)
        duration=$((end_time - start_time))
        echo "✅ $level.$test_name PASSED (${duration}s)"
        
        # Update results file
        jq ".tests += [{\"level\": \"$level\", \"name\": \"$test_name\", \"status\": \"PASSED\", \"duration\": $duration, \"critical\": $critical}]" $RESULTS_FILE > tmp.json && mv tmp.json $RESULTS_FILE
        return 0
    else
        end_time=$(date +%s)
        duration=$((end_time - start_time))
        echo "❌ $level.$test_name FAILED (${duration}s)"
        
        # Update results file
        jq ".tests += [{\"level\": \"$level\", \"name\": \"$test_name\", \"status\": \"FAILED\", \"duration\": $duration, \"critical\": $critical}]" $RESULTS_FILE > tmp.json && mv tmp.json $RESULTS_FILE
        
        if [ "$critical" = "true" ]; then
            echo "🚨 Critical test failed. Stopping test suite."
            exit 1
        fi
        return 1
    fi
}

# Start all components
echo "🚀 Starting all BPCI components..."
./start_all_components.sh

# Wait for initialization
echo "⏳ Waiting for component initialization..."
sleep 30

# Level 1: Simple Tests
echo -e "\n🟢 LEVEL 1: SIMPLE TESTS"
run_test "1" "Health_Checks" "test_health_checks.sh" "true"
run_test "1" "Service_Discovery" "test_service_discovery.sh" "true" 
run_test "1" "API_Connectivity" "test_api_connectivity.sh" "true"

# Level 2: Intermediate Tests  
echo -e "\n🟡 LEVEL 2: INTERMEDIATE TESTS"
run_test "2" "Component_Communication" "test_component_communication.sh" "true"
run_test "2" "Simple_Transaction" "test_simple_transaction.sh" "true"

# Level 3: Advanced Tests
echo -e "\n🔴 LEVEL 3: ADVANCED TESTS"
run_test "3" "Throughput_Test" "python3 load_test_transactions.py" "false"
run_test "3" "Cluster_Coordination" "test_cluster_coordination.sh" "false"

# Level 4: Expert Tests
echo -e "\n🟣 LEVEL 4: EXPERT TESTS"
run_test "4" "Failure_Recovery" "test_failure_recovery.sh" "false"
run_test "4" "Multi_Instance_Scaling" "test_multi_instance.sh" "false"
run_test "4" "CBOR_Streaming" "node test_cbor_streaming.js" "false"

# Generate final report
echo -e "\n📊 GENERATING TEST REPORT..."
jq '.summary = {
    "total_tests": (.tests | length),
    "passed": (.tests | map(select(.status == "PASSED")) | length),
    "failed": (.tests | map(select(.status == "FAILED")) | length),
    "critical_passed": (.tests | map(select(.critical == true and .status == "PASSED")) | length),
    "critical_failed": (.tests | map(select(.critical == true and .status == "FAILED")) | length),
    "total_duration": (.tests | map(.duration) | add)
}' $RESULTS_FILE > tmp.json && mv tmp.json $RESULTS_FILE

echo -e "\n🎉 TEST SUITE COMPLETE!"
echo "📄 Full log: $TEST_LOG"
echo "📊 Results: $RESULTS_FILE"
echo -e "\n📈 SUMMARY:"
jq -r '.summary | "Total Tests: \(.total_tests)\nPassed: \(.passed)\nFailed: \(.failed)\nCritical Passed: \(.critical_passed)\nCritical Failed: \(.critical_failed)\nTotal Duration: \(.total_duration)s"' $RESULTS_FILE
```

---

## 🎊 **Summary**

This comprehensive test suite provides:

1. ✅ **Complete Coverage** - All 10 components tested
2. ✅ **Progressive Complexity** - Simple → Advanced → Expert
3. ✅ **Real Scenarios** - Based on actual code and architecture  
4. ✅ **Performance Validation** - Throughput, latency, scalability
5. ✅ **Failure Testing** - Recovery, error handling, resilience
6. ✅ **Automation Ready** - Complete test runner script
7. ✅ **Detailed Reporting** - JSON results, logs, metrics

**Ready for crystal-clear testing of the entire BPCI infrastructure!** 🚀

Run the test suite to validate:
- Pure Virtual Mode operation
- Service name-based communication  
- End-to-end transaction processing
- Component integration
- Performance characteristics
- Failure recovery
- Multi-instance scaling
- Real-time CBOR streaming

The test suite will provide definitive validation that the BPCI infrastructure is production-ready! 🎯
