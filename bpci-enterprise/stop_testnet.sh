#!/bin/bash

# BPCI Testnet Stop Script

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

echo "🛑 Stopping BPCI Testnet Server"
echo "==============================="

# Check if PID file exists
if [ ! -f testnet_server.pid ]; then
    print_warning "No PID file found. Server may not be running."
    exit 0
fi

# Read PID
SERVER_PID=$(cat testnet_server.pid)

# Check if process is running
if ! kill -0 $SERVER_PID 2>/dev/null; then
    print_warning "Server process (PID: $SERVER_PID) is not running."
    rm -f testnet_server.pid
    exit 0
fi

# Stop the server
print_status "Stopping BPCI testnet server (PID: $SERVER_PID)..."
kill $SERVER_PID

# Wait for process to stop
sleep 2

# Force kill if still running
if kill -0 $SERVER_PID 2>/dev/null; then
    print_warning "Process still running, force killing..."
    kill -9 $SERVER_PID
    sleep 1
fi

# Verify process is stopped
if ! kill -0 $SERVER_PID 2>/dev/null; then
    print_status "✅ Server stopped successfully"
    rm -f testnet_server.pid
else
    print_error "❌ Failed to stop server"
    exit 1
fi

print_status "🎯 BPCI Testnet server has been stopped"
