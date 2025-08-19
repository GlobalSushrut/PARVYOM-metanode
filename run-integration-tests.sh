#!/bin/bash
# Integration Test Runner for Metanode
# This script runs all integration tests in the /tests directory

set -e

echo "🚀 Metanode Integration Test Runner"
echo "=================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Function to run a single test file
run_test() {
    local test_file=$1
    local test_name=$(basename "$test_file" .rs)
    
    echo -e "\n${YELLOW}Running test: $test_name${NC}"
    echo "----------------------------------------"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    # Try to compile and run the test
    if cargo test --manifest-path Cargo.toml --test "$test_name" 2>/dev/null; then
        echo -e "${GREEN}✅ PASSED: $test_name${NC}"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo -e "${RED}❌ FAILED: $test_name${NC}"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        
        # Try alternative approaches
        echo "   Attempting alternative test execution..."
        
        # Try running as a standalone Rust file
        if rustc --test "tests/$test_file" -o "/tmp/test_$test_name" 2>/dev/null && "/tmp/test_$test_name" 2>/dev/null; then
            echo -e "${GREEN}✅ PASSED (standalone): $test_name${NC}"
            PASSED_TESTS=$((PASSED_TESTS + 1))
            FAILED_TESTS=$((FAILED_TESTS - 1))
        else
            echo -e "${RED}   Still failing - test needs integration work${NC}"
        fi
        
        # Cleanup
        rm -f "/tmp/test_$test_name"
    fi
}

# Main execution
echo "Scanning for test files in /tests directory..."

cd /home/umesh/metanode

# Check if tests directory exists
if [ ! -d "tests" ]; then
    echo -e "${RED}Error: tests directory not found!${NC}"
    exit 1
fi

# Get list of test files
test_files=$(find tests -name "*.rs" -type f | sort)

if [ -z "$test_files" ]; then
    echo -e "${RED}No test files found in tests directory!${NC}"
    exit 1
fi

echo "Found $(echo "$test_files" | wc -l) test files"
echo ""

# Run each test file
for test_file in $test_files; do
    run_test "$(basename "$test_file")"
done

# Summary
echo ""
echo "========================================"
echo "🎯 Test Execution Summary"
echo "========================================"
echo -e "Total Tests:  $TOTAL_TESTS"
echo -e "${GREEN}Passed Tests: $PASSED_TESTS${NC}"
echo -e "${RED}Failed Tests: $FAILED_TESTS${NC}"

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "\n${GREEN}🎉 All tests passed!${NC}"
    exit 0
else
    echo -e "\n${YELLOW}⚠️  Some tests need integration work${NC}"
    echo "This is expected for a workspace project with complex dependencies."
    echo "Consider creating individual test crates or fixing dependency issues."
    exit 1
fi
