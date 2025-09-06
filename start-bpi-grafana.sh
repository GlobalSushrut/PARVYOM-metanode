#!/bin/bash

# BPI Grafana Monitoring Startup Script
# Simple command: "start BPI grafana"

echo "🚀 Starting BPI Grafana Monitoring..."
echo

# Check if BPI core is built
if [ ! -f "./target/release/bpi-core" ]; then
    echo "❌ BPI core not found. Building..."
    cargo build --release
    if [ $? -ne 0 ]; then
        echo "❌ Failed to build BPI core"
        exit 1
    fi
fi

# Default BPCI server URL (can be overridden)
BPCI_URL=${1:-"your-server.com:8081"}

echo "🎯 Starting BPI Grafana monitoring with BPCI server: $BPCI_URL"
echo

# Execute the BPI core grafana command
./target/release/bpi-core monitor grafana --start --bpci-url "$BPCI_URL"

if [ $? -eq 0 ]; then
    echo
    echo "✅ BPI Grafana monitoring started successfully!"
    echo "📊 Access your dashboard at: http://localhost:3000"
    echo "🔑 Login: admin / bpi-admin-2024"
    echo
    echo "🔍 Monitoring:"
    echo "   🏠 BPI Core (localhost:7777) - REQUIRES BPCI CONNECTION"
    echo "   🌐 BPCI Server ($BPCI_URL) - Economic Engine & Registry"
    echo
    echo "⚠️  CRITICAL: BPI cannot function without BPCI server connection!"
else
    echo "❌ Failed to start BPI Grafana monitoring"
    exit 1
fi
