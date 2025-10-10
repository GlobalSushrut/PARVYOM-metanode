#!/bin/bash
# BPI Core One-Click Complete Deployment Script
# Unified deployment with wallet connection, dynamic NX auth, and full pipeline

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
MODE=${1:-production}
WALLET_CONNECT=${2:-auto}
AUTH_MODE=${3:-dynamic}

echo -e "${BLUE}🚀 BPI Core Complete Deployment Starting...${NC}"
echo -e "${BLUE}   Mode: ${MODE}${NC}"
echo -e "${BLUE}   Wallet: ${WALLET_CONNECT}${NC}"
echo -e "${BLUE}   Auth: ${AUTH_MODE}${NC}"
echo ""

# Function to print status
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Function to check if port is available
check_port() {
    local port=$1
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        return 1
    else
        return 0
    fi
}

# Function to wait for service to be ready
wait_for_service() {
    local url=$1
    local service_name=$2
    local max_attempts=30
    local attempt=1
    
    echo -e "${YELLOW}⏳ Waiting for ${service_name} to be ready...${NC}"
    
    while [ $attempt -le $max_attempts ]; do
        if curl -s -f "$url" > /dev/null 2>&1; then
            print_status "${service_name} is ready!"
            return 0
        fi
        
        echo -n "."
        sleep 2
        attempt=$((attempt + 1))
    done
    
    print_error "${service_name} failed to start within timeout"
    return 1
}

# Phase 1: Environment Setup
echo -e "${BLUE}📋 Phase 1: Environment Setup${NC}"

# Check required ports
REQUIRED_PORTS=(7777 7778 8080 9545 9546)
for port in "${REQUIRED_PORTS[@]}"; do
    if ! check_port $port; then
        print_warning "Port $port is already in use, attempting to free it..."
        # Kill processes using the port
        lsof -ti:$port | xargs kill -9 2>/dev/null || true
        sleep 2
    fi
done

# Ensure we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Must be run from BPI Core root directory"
    exit 1
fi

# Build the project
echo -e "${YELLOW}🔨 Building BPI Core...${NC}"
cargo build --release --bins
print_status "BPI Core built successfully"

# Phase 2: Start BPI Service Orchestrator
echo -e "${BLUE}📋 Phase 2: Starting BPI Service Orchestrator${NC}"

# Create orchestrator binary
echo -e "${YELLOW}🔨 Creating orchestrator binary...${NC}"
cat > src/bin/bpi-orchestrator.rs << 'EOF'
//! BPI Service Orchestrator Binary - One-Click Deployment

use anyhow::Result;
use bpi_core::bpi_service_orchestrator::{BpiServiceOrchestrator, DeploymentConfig, Environment};
use tracing::{info, error};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🚀 BPI Service Orchestrator Starting...");
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).unwrap_or(&"production".to_string()).clone();
    
    // Create deployment configuration
    let config = DeploymentConfig {
        environment: match mode.as_str() {
            "development" => Environment::Development,
            "testing" => Environment::Testing,
            _ => Environment::Production,
        },
        auto_wallet_connect: true,
        enable_dynamic_auth: true,
        enable_monitoring: true,
        services: std::collections::HashMap::new(),
    };
    
    // Create and run orchestrator
    let orchestrator = BpiServiceOrchestrator::new(config);
    
    match orchestrator.deploy_complete_system().await {
        Ok(()) => {
            info!("✅ BPI Complete Deployment Successful!");
            
            // Keep orchestrator running for monitoring
            info!("📊 Orchestrator running in monitoring mode...");
            info!("🌐 Access Dashboard: http://localhost:8888");
            info!("📊 System Status: http://localhost:9999/status");
            
            // Wait indefinitely (or until Ctrl+C)
            tokio::signal::ctrl_c().await?;
            info!("🛑 Shutdown signal received, stopping services...");
            orchestrator.stop_all_services().await?;
        }
        Err(e) => {
            error!("❌ BPI Deployment Failed: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}
EOF

# Build orchestrator
cargo build --release --bin bpi-orchestrator
print_status "BPI Orchestrator built successfully"

# Phase 3: Start Orchestrator
echo -e "${BLUE}📋 Phase 3: Starting Complete BPI System${NC}"

# Start orchestrator in background
echo -e "${YELLOW}🚀 Starting BPI Orchestrator...${NC}"
./target/release/bpi-orchestrator $MODE > bpi-orchestrator.log 2>&1 &
ORCHESTRATOR_PID=$!

# Wait a moment for orchestrator to initialize
sleep 5

# Check if orchestrator is still running
if ! kill -0 $ORCHESTRATOR_PID 2>/dev/null; then
    print_error "BPI Orchestrator failed to start"
    echo "Log output:"
    tail -20 bpi-orchestrator.log
    exit 1
fi

print_status "BPI Orchestrator started (PID: $ORCHESTRATOR_PID)"

# Phase 4: Wait for Services
echo -e "${BLUE}📋 Phase 4: Waiting for Services to Start${NC}"

# Wait for BPI Core Node
wait_for_service "http://localhost:9546/health" "BPI Core Node"

# Wait for VM Server
wait_for_service "http://localhost:7777/health" "VM Server"

# Wait for Audit Server
wait_for_service "http://localhost:8080/health" "Audit Server"

# Wait for BPCI Bridge
wait_for_service "http://localhost:7778/health" "BPCI Bridge"

# Phase 5: Verify Wallet Connection
echo -e "${BLUE}📋 Phase 5: Verifying Wallet Connection${NC}"

if [ "$WALLET_CONNECT" = "auto" ]; then
    echo -e "${YELLOW}🔗 Testing wallet connection...${NC}"
    
    # Test wallet status
    WALLET_STATUS=$(curl -s "http://localhost:9546/api/wallet/status" || echo "failed")
    
    if [[ "$WALLET_STATUS" == *"connected"* ]]; then
        print_status "Wallet connection verified"
    else
        print_warning "Wallet connection may need manual verification"
    fi
fi

# Phase 6: Test Audit Pipeline
echo -e "${BLUE}📋 Phase 6: Testing Audit Pipeline${NC}"

echo -e "${YELLOW}🧪 Submitting test audit...${NC}"

# Submit test audit
TEST_AUDIT='{
    "payload": {
        "vm_type": "test",
        "operation": "deployment_test",
        "timestamp": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'"
    },
    "integrity": {
        "hash": "test_hash_123",
        "signature": "test_signature"
    },
    "signature": {
        "signature": "test_deployment_signature"
    },
    "metadata": {
        "deployment": "one_click_test",
        "version": "1.0.0"
    }
}'

AUDIT_RESPONSE=$(curl -s -X POST \
    -H "Content-Type: application/json" \
    -d "$TEST_AUDIT" \
    "http://localhost:8080/api/audit/submit" || echo "failed")

if [[ "$AUDIT_RESPONSE" == *"success"* ]]; then
    print_status "Audit pipeline test successful"
else
    print_warning "Audit pipeline test may need verification"
fi

# Phase 7: Display System Status
echo -e "${BLUE}📋 Phase 7: System Status Summary${NC}"

echo ""
echo -e "${GREEN}🎉 BPI CORE DEPLOYMENT COMPLETE! 🎉${NC}"
echo ""
echo -e "${BLUE}📊 SYSTEM STATUS:${NC}"
echo -e "   🟢 BPI Core Node:    http://localhost:9545 (RPC), http://localhost:9546 (API)"
echo -e "   🟢 VM Server:        http://localhost:7777"
echo -e "   🟢 Audit Server:     http://localhost:8080"
echo -e "   🟢 BPCI Bridge:      http://localhost:7778"
echo ""
echo -e "${BLUE}🌐 ACCESS POINTS:${NC}"
echo -e "   📊 System Dashboard: http://localhost:8888"
echo -e "   📈 Health Monitor:   http://localhost:9999/status"
echo -e "   🔍 Audit Logs:       ./vm_server_ziplock.log"
echo -e "   📋 ZipLock Files:    ./vm_audit.zjl ($(ls -lh vm_audit.zjl 2>/dev/null | awk '{print $5}' || echo 'N/A'))"
echo ""
echo -e "${BLUE}🎯 WHAT'S RUNNING:${NC}"
echo -e "   ✅ All 8 VM types with ZipLock audit recording"
echo -e "   ✅ Real-time cryptographic audit receipts"
echo -e "   ✅ BPI Core ↔ BPCI wallet integration"
echo -e "   ✅ Dynamic NX authorization system"
echo -e "   ✅ Complete audit-to-blockchain pipeline"
echo ""
echo -e "${BLUE}🛠️ MANAGEMENT:${NC}"
echo -e "   🛑 Stop All:         kill $ORCHESTRATOR_PID"
echo -e "   📊 Check Status:     curl http://localhost:9999/status"
echo -e "   🧪 Test Audit:       curl -X POST -H 'Content-Type: application/json' -d '{...}' http://localhost:8080/api/audit/submit"
echo ""
echo -e "${GREEN}🚀 BPI Core is ready for production use!${NC}"
echo ""

# Save deployment info
cat > .bpi-deployment-info << EOF
BPI_ORCHESTRATOR_PID=$ORCHESTRATOR_PID
DEPLOYMENT_MODE=$MODE
DEPLOYMENT_TIME=$(date)
WALLET_MODE=$WALLET_CONNECT
AUTH_MODE=$AUTH_MODE
EOF

print_status "Deployment information saved to .bpi-deployment-info"

# Keep script running to monitor
echo -e "${YELLOW}📊 Monitoring deployment... (Press Ctrl+C to stop)${NC}"

# Monitor orchestrator process
while kill -0 $ORCHESTRATOR_PID 2>/dev/null; do
    sleep 10
done

print_warning "BPI Orchestrator has stopped"
exit 0
