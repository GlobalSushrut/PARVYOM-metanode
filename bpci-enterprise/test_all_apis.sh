#!/bin/bash

# BPCI Enterprise - Comprehensive API Test Suite
# Tests all 70+ backend APIs with advanced validation

set -e

SERVER="134.209.210.181"
RESULTS_FILE="api_test_results.json"
PASSED=0
FAILED=0
TOTAL=0

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo "=========================================="
echo "BPCI ENTERPRISE - COMPREHENSIVE API TEST SUITE"
echo "=========================================="
echo ""
echo "Server: $SERVER"
echo "Date: $(date)"
echo ""

# Initialize results
echo "{\"tests\": [], \"summary\": {}}" > $RESULTS_FILE

# Test function with advanced validation
test_api() {
    local name=$1
    local method=$2
    local endpoint=$3
    local expected_status=$4
    local validation=$5
    
    TOTAL=$((TOTAL + 1))
    echo -n "[$TOTAL] Testing $name... "
    
    # Make request
    response=$(curl -s -w "\n%{http_code}" -X $method "http://$SERVER$endpoint" 2>&1)
    http_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | sed '$d')
    
    # Check HTTP status
    if [ "$http_code" != "$expected_status" ]; then
        echo -e "${RED}❌ FAIL${NC} (HTTP $http_code, expected $expected_status)"
        FAILED=$((FAILED + 1))
        return 1
    fi
    
    # Validate response if validation provided
    if [ -n "$validation" ]; then
        if echo "$body" | grep -q "$validation"; then
            echo -e "${GREEN}✅ PASS${NC}"
            PASSED=$((PASSED + 1))
            return 0
        else
            echo -e "${RED}❌ FAIL${NC} (Validation failed)"
            FAILED=$((FAILED + 1))
            return 1
        fi
    else
        echo -e "${GREEN}✅ PASS${NC}"
        PASSED=$((PASSED + 1))
        return 0
    fi
}

echo "=========================================="
echo "CATEGORY 1: HEALTH & SYSTEM APIs (10)"
echo "=========================================="
echo ""

test_api "Web Server Health" "GET" "/health" "200" "ok"
test_api "Web Server Status" "GET" "/api/system/status" "200" ""
test_api "Blockchain Health" "GET" "/blockchain/health" "200" ""
test_api "Bridge Health" "GET" "/bridge/health" "200" "healthy"
test_api "Orchestrator Health" "GET" "/orchestrator/health" "200" ""
test_api "Cluster Ledger Health" "GET" "/api/cluster/health" "200" ""
test_api "Consensus Status" "GET" "/api/consensus/status" "200" ""
test_api "Network Status" "GET" "/api/network/status" "200" ""
test_api "Mining Status" "GET" "/api/mining/status" "200" ""
test_api "System Metrics" "GET" "/api/metrics" "200" ""

echo ""
echo "=========================================="
echo "CATEGORY 2: BLOCKCHAIN APIs (15)"
echo "=========================================="
echo ""

test_api "Blockchain Info" "GET" "/blockchain/api/v1/blockchain/info" "200" "BPCI"
test_api "Blockchain Status" "GET" "/blockchain/api/v1/blockchain/status" "200" ""
test_api "Block Height" "GET" "/blockchain/api/v1/blocks/height" "200" ""
test_api "Latest Block" "GET" "/blockchain/api/v1/blocks/latest" "200" ""
test_api "Block List" "GET" "/blockchain/api/v1/blocks" "200" ""
test_api "Transaction List" "GET" "/blockchain/api/v1/transactions" "200" ""
test_api "Mempool Status" "GET" "/blockchain/api/v1/mempool" "200" ""
test_api "Mempool Size" "GET" "/blockchain/api/v1/mempool/size" "200" ""
test_api "Network Info" "GET" "/blockchain/api/v1/network" "200" ""
test_api "Peer Count" "GET" "/blockchain/api/v1/network/peers" "200" ""
test_api "Validator List" "GET" "/blockchain/api/v1/validators" "200" ""
test_api "Consensus Info" "GET" "/blockchain/api/v1/consensus" "200" ""
test_api "Oracle Status" "GET" "/blockchain/api/v1/oracle" "200" ""
test_api "System Stats" "GET" "/blockchain/api/v1/system" "200" ""
test_api "Blockchain Stats" "GET" "/blockchain/api/v1/stats" "200" ""

echo ""
echo "=========================================="
echo "CATEGORY 3: BPI BRIDGE APIs (12)"
echo "=========================================="
echo ""

test_api "Bridge Status" "GET" "/bridge/health" "200" "Component 5"
test_api "Bridge Pricing" "GET" "/bridge/pricing" "200" ""
test_api "Address Pool Status" "GET" "/bridge/pool/status" "200" ""
test_api "Registry Tokens" "GET" "/bridge/registry/tokens" "200" ""
test_api "Account List" "GET" "/bridge/accounts" "200" ""
test_api "Transaction Stats" "GET" "/bridge/transactions/stats" "200" ""
test_api "Gas Prices" "GET" "/bridge/gas/prices" "200" ""
test_api "Rent Prices" "GET" "/bridge/rent/prices" "200" ""
test_api "Token Balance" "GET" "/bridge/tokens/balance" "200" ""
test_api "Connection Stats" "GET" "/bridge/connections/stats" "200" ""
test_api "CBOR Status" "GET" "/bridge/cbor/status" "200" ""
test_api "Bridge Metrics" "GET" "/bridge/metrics" "200" ""

echo ""
echo "=========================================="
echo "CATEGORY 4: WALLET & REGISTRY APIs (10)"
echo "=========================================="
echo ""

test_api "Wallet Registry" "GET" "/api/wallet/registry" "200" ""
test_api "Wallet List" "GET" "/api/wallets" "200" ""
test_api "Wallet Stats" "GET" "/api/wallet/stats" "200" ""
test_api "Registry Status" "GET" "/api/registry/status" "200" ""
test_api "Registry Nodes" "GET" "/api/registry/nodes" "200" ""
test_api "Identity Proofs" "GET" "/api/identity/proofs" "200" ""
test_api "Authority Levels" "GET" "/api/authority/levels" "200" ""
test_api "Stamped Wallets" "GET" "/api/wallets/stamped" "200" ""
test_api "Wallet Types" "GET" "/api/wallet/types" "200" ""
test_api "Verification Status" "GET" "/api/verification/status" "200" ""

echo ""
echo "=========================================="
echo "CATEGORY 5: MINING & ECONOMY APIs (10)"
echo "=========================================="
echo ""

test_api "Mining Status" "GET" "/api/mining/status" "200" ""
test_api "Mining Sessions" "GET" "/api/mining/sessions" "200" ""
test_api "Mining Stats" "GET" "/api/mining/stats" "200" ""
test_api "Economy Status" "GET" "/api/economy/status" "200" ""
test_api "Economy Services" "GET" "/api/economy/services" "200" ""
test_api "Coin Distribution" "GET" "/api/economy/coins" "200" ""
test_api "Treasury Status" "GET" "/api/economy/treasury" "200" ""
test_api "Reward System" "GET" "/api/economy/rewards" "200" ""
test_api "Gas Economy" "GET" "/api/economy/gas" "200" ""
test_api "Rent Economy" "GET" "/api/economy/rent" "200" ""

echo ""
echo "=========================================="
echo "CATEGORY 6: BANK & GOVERNMENT APIs (8)"
echo "=========================================="
echo ""

test_api "Bank Status" "GET" "/api/bank/status" "200" ""
test_api "Bank Services" "GET" "/api/bank/services" "200" ""
test_api "Bank Settlements" "GET" "/api/bank/settlements" "200" ""
test_api "Bank Mesh" "GET" "/api/bank/mesh" "200" ""
test_api "Government Status" "GET" "/api/government/status" "200" ""
test_api "Government Services" "GET" "/api/government/services" "200" ""
test_api "Jurisdiction Status" "GET" "/api/jurisdiction/status" "200" ""
test_api "Regulatory Compliance" "GET" "/api/regulatory/compliance" "200" ""

echo ""
echo "=========================================="
echo "CATEGORY 7: ORCHESTRATOR & vPOD APIs (8)"
echo "=========================================="
echo ""

test_api "Orchestrator Status" "GET" "/orchestrator/health" "200" ""
test_api "vPod List" "GET" "/orchestrator/vpods" "200" ""
test_api "vPod Stats" "GET" "/orchestrator/vpods/stats" "200" ""
test_api "Service List" "GET" "/orchestrator/services" "200" ""
test_api "Service Stats" "GET" "/orchestrator/services/stats" "200" ""
test_api "Cellular Division" "GET" "/orchestrator/cellular" "200" ""
test_api "Resource Allocation" "GET" "/orchestrator/resources" "200" ""
test_api "Orchestrator Metrics" "GET" "/orchestrator/metrics" "200" ""

echo ""
echo "=========================================="
echo "CATEGORY 8: ADVANCED FEATURES (7)"
echo "=========================================="
echo ""

test_api "Auction Status" "GET" "/api/auction/status" "200" ""
test_api "Auction Mempool" "GET" "/api/auction/mempool" "200" ""
test_api "Shadow Registry" "GET" "/api/shadow/registry" "200" ""
test_api "XTMP Status" "GET" "/api/xtmp/status" "200" ""
test_api "CommuteLock Status" "GET" "/api/commutelock/status" "200" ""
test_api "DynaRoute Status" "GET" "/api/dynaroute/status" "200" ""
test_api "Pure Virtual Mode" "GET" "/api/virtual/mode" "200" ""

echo ""
echo "=========================================="
echo "TEST SUMMARY"
echo "=========================================="
echo ""
echo "Total Tests: $TOTAL"
echo -e "Passed: ${GREEN}$PASSED${NC}"
echo -e "Failed: ${RED}$FAILED${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✅ ALL TESTS PASSED!${NC}"
    echo ""
    echo "API Conformity: 100%"
    echo "Backend Status: FULLY OPERATIONAL"
    echo "Ready for Frontend: YES"
    exit 0
else
    PASS_RATE=$((PASSED * 100 / TOTAL))
    echo -e "${YELLOW}⚠️  SOME TESTS FAILED${NC}"
    echo ""
    echo "Pass Rate: $PASS_RATE%"
    echo "Backend Status: PARTIALLY OPERATIONAL"
    echo "Action Required: Review failed endpoints"
    exit 1
fi
