#!/bin/bash

# 🚀 BPCI Component Startup Script
echo "🚀 Starting BPCI Components in Pure Virtual Mode..."

# Kill any existing components first
echo "🧹 Cleaning up existing processes..."
pkill -f "bpci-consensus-server" 2>/dev/null || true
pkill -f "bpci_blockchain_server" 2>/dev/null || true
pkill -f "bpci_auction_mempool_server" 2>/dev/null || true
pkill -f "bpci_bpi_bridge" 2>/dev/null || true
pkill -f "bpci_cluster_ledger_server" 2>/dev/null || true

sleep 2

# Start core components in background
echo "🔄 Starting core components..."

echo "1. Starting Consensus Server..."
RUST_LOG=info cargo run --bin bpci-consensus-server > /tmp/consensus.log 2>&1 &
CONSENSUS_PID=$!
echo "   PID: $CONSENSUS_PID"
sleep 3

echo "2. Starting Blockchain Server..."
RUST_LOG=info cargo run --bin bpci_blockchain_server > /tmp/blockchain.log 2>&1 &
BLOCKCHAIN_PID=$!
echo "   PID: $BLOCKCHAIN_PID"
sleep 3

echo "3. Starting Auction Mempool..."
RUST_LOG=info cargo run --bin bpci_auction_mempool_server > /tmp/auction.log 2>&1 &
AUCTION_PID=$!
echo "   PID: $AUCTION_PID"
sleep 3

echo "4. Starting BPI-BPCI Bridge..."
RUST_LOG=info cargo run --bin bpci_bpi_bridge > /tmp/bridge.log 2>&1 &
BRIDGE_PID=$!
echo "   PID: $BRIDGE_PID"
sleep 3

echo "5. Starting Cluster Ledger..."
RUST_LOG=info cargo run --bin bpci_cluster_ledger_server > /tmp/cluster.log 2>&1 &
CLUSTER_PID=$!
echo "   PID: $CLUSTER_PID"
sleep 5

echo "⏳ Waiting for components to initialize..."
sleep 10

echo "✅ Component startup complete!"
echo ""
echo "📊 Component Status:"
ps aux | grep -E "(bpci-consensus|bpci_blockchain|bpci_auction|bpci_bpi_bridge|bpci_cluster)" | grep -v grep

echo ""
echo "🌐 Pure Virtual Mode Status:"
echo "Static ports (should be 0): $(netstat -tuln | grep -E '(9001|8080|9004|7002|6001|7000)' | wc -l)"
echo "Dynamic ports allocated: $(netstat -tuln | grep LISTEN | wc -l)"

echo ""
echo "📄 Log files:"
echo "  Consensus: /tmp/consensus.log"
echo "  Blockchain: /tmp/blockchain.log" 
echo "  Auction: /tmp/auction.log"
echo "  Bridge: /tmp/bridge.log"
echo "  Cluster: /tmp/cluster.log"

echo ""
echo "🎯 Ready for testing!"
