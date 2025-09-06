#!/bin/bash

# BPCI/BPI Penetration Testing Script
# Comprehensive security testing with hacker-level attack simulations

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${RED}🔥 BPCI/BPI PENETRATION TESTING FRAMEWORK${NC}"
echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${YELLOW}⚠️  WARNING: This will perform REAL security attacks against the system${NC}"
echo -e "${YELLOW}⚠️  Only run in testnet/development environment${NC}"
echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

# Safety confirmation
echo -e "${CYAN}🔍 Security Testing Categories:${NC}"
echo -e "  ${PURPLE}🔐 Qlock Security${NC} - Quantum-resistant cryptographic testing"
echo -e "  ${PURPLE}🔒 TLS/SSL Security${NC} - Protocol vulnerability testing"
echo -e "  ${PURPLE}🌐 HTTP/CG Security${NC} - Web application security testing"
echo -e "  ${PURPLE}⛓️  Blockchain Security${NC} - Consensus attack simulation"
echo -e "  ${PURPLE}🎭 Advanced Hacker Simulation${NC} - APT and exploit chain testing"
echo ""

read -p "$(echo -e ${YELLOW}🚨 Are you sure you want to run penetration testing? [y/N]: ${NC})" -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${RED}❌ Penetration testing cancelled by user${NC}"
    exit 1
fi

# Environment setup
echo -e "${BLUE}🔧 Setting up penetration testing environment${NC}"
export BPCI_NETWORK_MODE=testnet
export RUST_LOG=info
export RUST_BACKTRACE=1

# Verify testnet mode
if [ "$BPCI_NETWORK_MODE" != "testnet" ]; then
    echo -e "${RED}🚨 SECURITY ERROR: Must run in testnet mode only!${NC}"
    echo -e "${RED}🚨 Set BPCI_NETWORK_MODE=testnet${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Safety confirmed: Running in testnet mode${NC}"

# Build the penetration testing framework
echo -e "${BLUE}🔨 Building penetration testing framework${NC}"
if ! cargo build --bin bpci_penetration_test_runner --release; then
    echo -e "${RED}❌ Failed to build penetration testing framework${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Penetration testing framework built successfully${NC}"

# Execute comprehensive penetration testing
echo -e "${RED}🚀 Launching comprehensive penetration testing suite${NC}"
echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

# Run the penetration tests
if cargo run --bin bpci_penetration_test_runner --release; then
    echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}🎉 Penetration testing completed successfully!${NC}"
    echo -e "${GREEN}✅ BPCI/BPI system validated against real attack vectors${NC}"
    echo -e "${CYAN}📊 Check the detailed security report above for findings${NC}"
    echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
else
    echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${RED}❌ Penetration testing failed or found critical vulnerabilities${NC}"
    echo -e "${YELLOW}⚠️  Review the security report above for details${NC}"
    echo -e "${RED}🚨 System may NOT be ready for production deployment${NC}"
    echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    exit 1
fi

echo -e "${PURPLE}🛡️  BPCI/BPI Penetration Testing Complete${NC}"
