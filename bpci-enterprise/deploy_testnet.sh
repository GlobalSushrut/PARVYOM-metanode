#!/bin/bash

# BPCI Testnet Deployment Script
# Ready for tomorrow's first stage deployment

set -e

echo "🚀 BPCI Testnet Deployment - First Stage"
echo "========================================"

# Configuration
export BPCI_NETWORK_MODE="testnet"
export RUST_LOG="info"
export BPCI_PORT="8080"
export BPCI_DATA_DIR="./testnet_data"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${BLUE}$1${NC}"
}

# Check prerequisites
check_prerequisites() {
    print_header "📋 Checking Prerequisites"
    
    # Check Rust installation
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo not found. Please install Rust first."
        exit 1
    fi
    print_status "✅ Rust/Cargo found"
    
    # Check system requirements
    CPU_CORES=$(nproc)
    MEMORY_GB=$(free -g | awk '/^Mem:/{print $2}')
    
    if [ "$CPU_CORES" -lt 4 ]; then
        print_warning "⚠️  Recommended: 8+ CPU cores (found: $CPU_CORES)"
    else
        print_status "✅ CPU cores: $CPU_CORES"
    fi
    
    if [ "$MEMORY_GB" -lt 4 ]; then
        print_warning "⚠️  Recommended: 8+ GB RAM (found: ${MEMORY_GB}GB)"
    else
        print_status "✅ Memory: ${MEMORY_GB}GB"
    fi
    
    # Create data directory
    mkdir -p "$BPCI_DATA_DIR"
    print_status "✅ Data directory created: $BPCI_DATA_DIR"
}

# Build BPCI for testnet
build_bpci() {
    print_header "🔨 Building BPCI for Testnet"
    
    print_status "Building with testnet features..."
    cargo build --release --bin community_installer_web
    
    if [ $? -eq 0 ]; then
        print_status "✅ BPCI built successfully"
    else
        print_error "❌ Build failed"
        exit 1
    fi
}

# Run tests
run_tests() {
    print_header "🧪 Running Tests"
    
    print_status "Running comprehensive test suite..."
    cargo test --lib -- --nocapture
    
    if [ $? -eq 0 ]; then
        print_status "✅ All tests passed"
    else
        print_error "❌ Tests failed"
        exit 1
    fi
}

# Start BPCI testnet server
start_testnet_server() {
    print_header "🌐 Starting BPCI Testnet Server"
    
    print_status "Starting server on port $BPCI_PORT..."
    print_status "Network mode: $BPCI_NETWORK_MODE"
    print_status "Data directory: $BPCI_DATA_DIR"
    
    # Start the server in background
    nohup cargo run --release --bin community_installer_web > testnet_server.log 2>&1 &
    SERVER_PID=$!
    
    # Wait for server to start
    sleep 5
    
    # Check if server is running
    if kill -0 $SERVER_PID 2>/dev/null; then
        print_status "✅ Server started successfully (PID: $SERVER_PID)"
        echo $SERVER_PID > testnet_server.pid
        
        # Test server connectivity
        if curl -s http://localhost:$BPCI_PORT/api/status > /dev/null; then
            print_status "✅ Server is responding to requests"
        else
            print_warning "⚠️  Server started but not responding yet"
        fi
    else
        print_error "❌ Failed to start server"
        exit 1
    fi
}

# Display deployment information
show_deployment_info() {
    print_header "📊 Testnet Deployment Information"
    
    echo ""
    echo "🌐 Web Interface:"
    echo "   URL: http://localhost:$BPCI_PORT"
    echo "   Dashboard: http://localhost:$BPCI_PORT/dashboard"
    echo ""
    echo "🔌 API Endpoints:"
    echo "   Status: http://localhost:$BPCI_PORT/api/status"
    echo "   Auction Stats: http://localhost:$BPCI_PORT/api/testnet/auction/stats"
    echo "   Partner Status: http://localhost:$BPCI_PORT/api/testnet/partners/status"
    echo ""
    echo "📁 Data Directory: $BPCI_DATA_DIR"
    echo "📝 Server Log: testnet_server.log"
    echo "🆔 Server PID: $(cat testnet_server.pid 2>/dev/null || echo 'Not found')"
    echo ""
    echo "🎯 Testnet Features:"
    echo "   ✅ Mock auction execution to database"
    echo "   ✅ Partner revenue simulation (25% share)"
    echo "   ✅ Real-time auction monitoring"
    echo "   ✅ CueDB integration for storage"
    echo "   ✅ Comprehensive audit trails"
    echo ""
    echo "🚀 Ready for Partner Onboarding!"
}

# Main deployment flow
main() {
    print_header "🚀 BPCI Testnet Deployment Starting..."
    echo ""
    
    check_prerequisites
    echo ""
    
    build_bpci
    echo ""
    
    run_tests
    echo ""
    
    start_testnet_server
    echo ""
    
    show_deployment_info
    echo ""
    
    print_status "🎉 BPCI Testnet deployment completed successfully!"
    print_status "🌐 Access the web interface at: http://localhost:$BPCI_PORT"
    echo ""
    print_status "To stop the server: ./stop_testnet.sh"
    print_status "To view logs: tail -f testnet_server.log"
}

# Handle script interruption
cleanup() {
    print_warning "Deployment interrupted"
    if [ -f testnet_server.pid ]; then
        SERVER_PID=$(cat testnet_server.pid)
        if kill -0 $SERVER_PID 2>/dev/null; then
            print_status "Stopping server..."
            kill $SERVER_PID
            rm -f testnet_server.pid
        fi
    fi
    exit 1
}

trap cleanup INT TERM

# Run main deployment
main "$@"
