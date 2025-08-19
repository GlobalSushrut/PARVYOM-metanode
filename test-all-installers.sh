#!/bin/bash

# BPCI Installer Validation Test Script
# Tests all installer types and deployment mode enforcement

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Helper functions
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
    ((TESTS_PASSED++))
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
    ((TESTS_FAILED++))
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

run_test() {
    local test_name="$1"
    local command="$2"
    local expected_exit_code="${3:-0}"
    
    ((TESTS_RUN++))
    log_info "Running test: $test_name"
    
    if eval "$command" >/dev/null 2>&1; then
        actual_exit_code=0
    else
        actual_exit_code=$?
    fi
    
    if [ "$actual_exit_code" -eq "$expected_exit_code" ]; then
        log_success "$test_name"
        return 0
    else
        log_error "$test_name (expected exit code $expected_exit_code, got $actual_exit_code)"
        return 1
    fi
}

# Test deployment mode enforcement
test_deployment_mode() {
    local mode="$1"
    local config_path="$2"
    local binary_path="$3"
    
    log_info "Testing deployment mode: $mode"
    
    # Test basic CLI functionality
    run_test "$mode: CLI status command" "$binary_path --config $config_path status"
    
    # Test network restrictions based on deployment mode
    case "$mode" in
        "Development")
            # Dev should allow localhost, reject mainnet/testnet
            run_test "$mode: Allow localhost" "$binary_path --config $config_path --network localhost status"
            run_test "$mode: Reject mainnet" "$binary_path --config $config_path --network mainnet status" 1
            run_test "$mode: Reject testnet" "$binary_path --config $config_path --network testnet status" 1
            ;;
        "Enterprise")
            # Enterprise should allow mainnet/testnet, reject localhost
            run_test "$mode: Allow mainnet" "$binary_path --config $config_path --network mainnet status"
            run_test "$mode: Allow testnet" "$binary_path --config $config_path --network testnet status"
            run_test "$mode: Reject localhost" "$binary_path --config $config_path --network localhost status" 1
            ;;
        "Community")
            # Community should allow mainnet/testnet, reject localhost
            run_test "$mode: Allow mainnet" "$binary_path --config $config_path --network mainnet status"
            run_test "$mode: Allow testnet" "$binary_path --config $config_path --network testnet status"
            run_test "$mode: Reject localhost" "$binary_path --config $config_path --network localhost status" 1
            ;;
    esac
}

# Test configuration file validation
test_config_validation() {
    local mode="$1"
    local config_path="$2"
    local binary_path="$3"
    
    log_info "Testing configuration validation for $mode"
    
    # Test that config file exists and is readable
    if [ -f "$config_path" ]; then
        log_success "$mode: Configuration file exists"
    else
        log_error "$mode: Configuration file missing at $config_path"
        return 1
    fi
    
    # Test that CLI can load and validate configuration
    if $binary_path --config "$config_path" status >/dev/null 2>&1; then
        log_success "$mode: Configuration loads successfully"
    else
        log_error "$mode: Configuration validation failed"
        return 1
    fi
}

# Main test execution
main() {
    echo -e "${BLUE}🧪 BPCI Installer Validation Test Suite${NC}"
    echo -e "${BLUE}========================================${NC}"
    echo
    
    # Check if we're running as root
    if [ "$EUID" -ne 0 ]; then
        log_error "This test script must be run as root (sudo)"
        exit 1
    fi
    
    # Test current installation (should be Enterprise from previous run)
    if [ -f "/etc/bpci/config.toml" ] && [ -f "/opt/bpci/bin/bpci-enterprise" ]; then
        log_info "Testing current Enterprise installation"
        test_config_validation "Enterprise" "/etc/bpci/config.toml" "/opt/bpci/bin/bpci-enterprise"
        test_deployment_mode "Enterprise" "/etc/bpci/config.toml" "/opt/bpci/bin/bpci-enterprise"
        echo
    else
        log_warning "No Enterprise installation found"
    fi
    
    # Test Dev installer
    log_info "Testing Dev installer..."
    echo
    
    # Clean previous installation
    rm -rf /opt/bpci /etc/bpci
    
    # Run dev installer
    if ./installer/metanode-dev-installer.sh >/dev/null 2>&1; then
        log_success "Dev installer completed"
        test_config_validation "Development" "/home/umesh/.config/bpci/config.toml" "/home/umesh/.local/bin/bpci-dev"
        test_deployment_mode "Development" "/home/umesh/.config/bpci/config.toml" "/home/umesh/.local/bin/bpci-dev"
    else
        log_error "Dev installer failed"
    fi
    echo
    
    # Test Community installer
    log_info "Testing Community installer..."
    echo
    
    # Clean previous installation
    rm -rf /opt/bpci /etc/bpci
    
    # Run community installer
    if ./installer/bpci-community-installer.sh >/dev/null 2>&1; then
        log_success "Community installer completed"
        test_config_validation "Community" "/etc/bpci/config.toml" "/opt/bpci/bin/bpci-enterprise"
        test_deployment_mode "Community" "/etc/bpci/config.toml" "/opt/bpci/bin/bpci-enterprise"
    else
        log_error "Community installer failed"
    fi
    echo
    
    # Test Enterprise installer
    log_info "Testing Enterprise installer..."
    echo
    
    # Clean previous installation
    rm -rf /opt/bpci /etc/bpci
    
    # Run enterprise installer
    if ./installer/enterprise-installer.sh >/dev/null 2>&1; then
        log_success "Enterprise installer completed"
        test_config_validation "Enterprise" "/etc/bpci/config.toml" "/opt/bpci/bin/bpci-enterprise"
        test_deployment_mode "Enterprise" "/etc/bpci/config.toml" "/opt/bpci/bin/bpci-enterprise"
    else
        log_error "Enterprise installer failed"
    fi
    echo
    
    # Test results summary
    echo -e "${BLUE}📊 Test Results Summary${NC}"
    echo -e "${BLUE}========================${NC}"
    echo -e "Tests Run: $TESTS_RUN"
    echo -e "${GREEN}Tests Passed: $TESTS_PASSED${NC}"
    echo -e "${RED}Tests Failed: $TESTS_FAILED${NC}"
    echo
    
    if [ $TESTS_FAILED -eq 0 ]; then
        echo -e "${GREEN}🎉 All tests passed! BPCI installers are working correctly.${NC}"
        exit 0
    else
        echo -e "${RED}💥 Some tests failed. Please check the output above.${NC}"
        exit 1
    fi
}

# Run main function
main "$@"
