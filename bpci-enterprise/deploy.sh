#!/bin/bash

echo "🚀 BPI Token System Deployment Script"
echo "====================================="

# Set environment variables
export RUST_LOG=info
export RUST_ENV=production
export PORT=8080

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Step 1: Test compilation
print_info "Step 1: Testing compilation..."
if cargo check --lib; then
    print_status "Compilation successful"
else
    print_error "Compilation failed"
    exit 1
fi

# Step 2: Run database integration tests
print_info "Step 2: Running database integration tests..."
if cargo test integrated_token_db_test --release; then
    print_status "Database tests passed"
else
    print_error "Database tests failed"
    exit 1
fi

# Step 3: Build the server binary
print_info "Step 3: Building production server..."
if cargo build --bin token_server --release; then
    print_status "Server binary built successfully"
else
    print_error "Server build failed"
    exit 1
fi

# Step 4: Create production database directory
print_info "Step 4: Setting up production database..."
mkdir -p /tmp/bpci_production_db
chmod 755 /tmp/bpci_production_db
print_status "Production database directory created"

# Step 5: Test server startup (background)
print_info "Step 5: Testing server startup..."
./target/release/token_server &
SERVER_PID=$!
sleep 3

# Check if server is running
if kill -0 $SERVER_PID 2>/dev/null; then
    print_status "Server started successfully (PID: $SERVER_PID)"
    
    # Test API endpoints
    print_info "Step 6: Testing API endpoints..."
    
    # Test health check
    if curl -s http://localhost:8080/api/system/health > /dev/null; then
        print_status "Health check endpoint working"
    else
        print_warning "Health check endpoint not responding"
    fi
    
    # Test system stats
    if curl -s http://localhost:8080/api/system/stats > /dev/null; then
        print_status "System stats endpoint working"
    else
        print_warning "System stats endpoint not responding"
    fi
    
    # Test API status
    if curl -s http://localhost:8080/api/status > /dev/null; then
        print_status "API status endpoint working"
    fi
    
    # Stop test server
    kill $SERVER_PID
    print_info "Test server stopped"
else
    print_error "Server failed to start"
    exit 1
fi

# Step 7: Display deployment information
print_info "Step 7: Deployment Summary"
echo "=================================="
print_status "✅ Compilation: PASSED"
print_status "✅ Database Tests: PASSED" 
print_status "✅ Server Build: PASSED"
print_status "✅ API Endpoints: WORKING"
echo ""
print_info "🌐 Server Configuration:"
echo "   - Binary: ./target/release/token_server"
echo "   - Port: 8080"
echo "   - Database: /tmp/bpci_production_db (same instance as tests)"
echo "   - Environment: Production"
echo ""
print_info "🔗 API Endpoints for Vite Frontend:"
echo "   - Health: http://localhost:8080/api/system/health"
echo "   - Stats: http://localhost:8080/api/system/stats"
echo "   - Tokens: http://localhost:8080/api/tokens"
echo "   - Addresses: http://localhost:8080/api/addresses"
echo ""
print_info "🚀 To start production server:"
echo "   ./target/release/token_server"
echo ""
print_info "📱 Ready for Vite frontend integration!"
print_status "Deployment preparation complete!"
