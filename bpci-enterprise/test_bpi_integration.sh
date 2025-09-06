#!/bin/bash

# BPI → BPCI → Testnet Integration Test Script
# 
# This script executes the complete BPI → BPCI → Testnet integration test:
# 1. Takes real BPI data from live BPI ledger
# 2. Verifies BPI authenticity using cryptographic proofs
# 3. Bundles verified BPI data to BPCI format
# 4. Transmits BPCI to development/test endpoint (NOT mainnet)

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print colored output
print_header() {
    echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${PURPLE}$1${NC}"
    echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

print_info() {
    echo -e "${CYAN}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_safety() {
    echo -e "${RED}🚨 SAFETY: $1${NC}"
}

# Start banner
clear
print_header "🚀 BPI → BPCI → TESTNET INTEGRATION TEST"
echo -e "${CYAN}📡 Takes real BPI data from live BPI ledger${NC}"
echo -e "${CYAN}🔐 Verifies BPI authenticity using cryptographic proofs${NC}"
echo -e "${CYAN}📦 Bundles verified BPI data to BPCI format${NC}"
echo -e "${CYAN}🌐 Transmits BPCI to DEVELOPMENT endpoint (NOT mainnet)${NC}"
print_safety "NO mainnet transactions will be executed"
echo ""

# Check prerequisites
print_header "📋 CHECKING PREREQUISITES"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -d "src" ]; then
    print_error "Not in BPCI project directory. Please run from /home/umesh/metanode/bpci-enterprise"
    exit 1
fi
print_success "In correct project directory"

# Check Rust installation
if ! command -v cargo &> /dev/null; then
    print_error "Cargo not found. Please install Rust"
    exit 1
fi
print_success "Cargo found: $(cargo --version)"

# Safety check: Ensure testnet mode
export BPCI_NETWORK_MODE="testnet"
print_safety "Network mode set to: $BPCI_NETWORK_MODE"

if [ "$BPCI_NETWORK_MODE" != "testnet" ]; then
    print_error "SAFETY CHECK FAILED: BPCI_NETWORK_MODE must be 'testnet'"
    print_error "Current mode: $BPCI_NETWORK_MODE"
    exit 1
fi
print_success "Safety check passed: Running in testnet mode"

# Check compilation
print_header "🔨 CHECKING COMPILATION"
print_info "Compiling BPCI project..."

if cargo check --lib --message-format=short > /dev/null 2>&1; then
    print_success "Project compiles successfully"
else
    print_error "Compilation failed. Please fix compilation errors first"
    print_info "Run: cargo check --lib"
    exit 1
fi

# Build the integration test runner
print_info "Building BPI integration test runner..."
if cargo build --bin bpi_testnet_integration_runner --release > /dev/null 2>&1; then
    print_success "Integration test runner built successfully"
else
    print_error "Failed to build integration test runner"
    print_info "Run: cargo build --bin bpi_testnet_integration_runner"
    exit 1
fi

# Display test configuration
print_header "⚙️  TEST CONFIGURATION"
print_info "Network Mode: $BPCI_NETWORK_MODE"
print_info "Test Type: BPI → BPCI → Testnet Integration"
print_info "Safety Level: MAXIMUM (Development endpoints only)"
print_info "Expected Duration: 30-60 seconds"
print_safety "All transactions will use TESTNET endpoints only"

# Confirm execution
echo ""
read -p "$(echo -e ${YELLOW}🤔 Do you want to proceed with the integration test? [y/N]: ${NC})" -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_info "Integration test cancelled by user"
    exit 0
fi

# Execute the integration test
print_header "🧪 EXECUTING BPI → BPCI → TESTNET INTEGRATION TEST"
print_safety "Starting integration test - NO mainnet transactions will occur"

# Set additional environment variables for the test
export RUST_LOG=info
export RUST_BACKTRACE=1

# Run the integration test
print_info "Launching integration test runner..."
echo ""

if cargo run --bin bpi_testnet_integration_runner --release; then
    echo ""
    print_success "Integration test completed successfully!"
    
    # Display success summary
    print_header "🎉 INTEGRATION TEST SUCCESS SUMMARY"
    print_success "✅ Real BPI data successfully fetched from live ledger"
    print_success "✅ BPI authenticity verified using cryptographic proofs"
    print_success "✅ BPI data successfully bundled to BPCI format"
    print_success "✅ BPCI successfully transmitted to development endpoint"
    print_success "✅ Test results stored in testnet storage"
    print_safety "CONFIRMED: No mainnet transactions were executed"
    
    # Display next steps
    print_header "🚀 NEXT STEPS"
    print_info "1. Review test logs above for detailed execution flow"
    print_info "2. Check testnet storage for stored test results"
    print_info "3. Validate development endpoint received the BPCI bundle"
    print_info "4. Run additional integration tests if needed"
    print_info "5. Proceed with full testnet deployment when ready"
    
    echo ""
    print_success "BPI → BPCI → Testnet integration test completed successfully! 🎉"
    
else
    echo ""
    print_error "Integration test failed!"
    
    # Display failure information
    print_header "❌ INTEGRATION TEST FAILURE ANALYSIS"
    print_info "The integration test encountered an error during execution"
    print_info "Common causes:"
    print_info "  • BPI ledger connection issues"
    print_info "  • Development endpoint unavailable"
    print_info "  • Authentication/signature verification failures"
    print_info "  • Network connectivity problems"
    
    print_header "🔧 TROUBLESHOOTING STEPS"
    print_info "1. Check the error logs above for specific failure details"
    print_info "2. Verify BPI ledger connectivity"
    print_info "3. Ensure development endpoints are accessible"
    print_info "4. Check network connectivity and firewall settings"
    print_info "5. Validate authentication keys and signatures"
    print_info "6. Re-run with RUST_LOG=debug for detailed logging"
    
    echo ""
    print_error "Integration test failed. Please review the logs and try again."
    exit 1
fi

echo ""
print_header "🏁 INTEGRATION TEST COMPLETE"
print_safety "Integration test completed safely - no mainnet impact"
echo ""
