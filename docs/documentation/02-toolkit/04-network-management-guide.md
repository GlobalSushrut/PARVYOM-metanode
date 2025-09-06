# Network Management Guide - BPCI Network Operations

## Overview
BPCI Enterprise provides comprehensive network management capabilities for peer-to-peer operations, blockchain synchronization, and distributed system coordination. This guide covers network setup, monitoring, troubleshooting, and optimization.

## Network Architecture

### BPCI Network Components
```rust
// From the BPCI codebase - real network components
pub struct NetworkNode {
    pub node_id: String,
    pub node_type: NodeType,
    pub endpoint: String,
    pub status: NodeStatus,
    pub capabilities: Vec<NodeCapability>,
    pub last_seen: DateTime<Utc>,
    pub reputation_score: f64,
}

pub enum NodeType {
    Validator,        // Consensus validation nodes
    Miner,           // Proof-of-Execution mining nodes
    Oracle,          // Data oracle nodes
    Storage,         // Distributed storage nodes
    Gateway,         // Network gateway nodes
    Relay,           // Message relay nodes
}

pub enum NodeStatus {
    Active,          // Fully operational
    Syncing,         // Synchronizing with network
    Degraded,        // Partial functionality
    Offline,         // Not responding
    Banned,          // Temporarily banned
}
```

### Network Topology
- **Mesh Network**: Decentralized peer-to-peer architecture
- **Consensus Layer**: IBFT + HotStuff + Auction consensus
- **Communication Layer**: HTTP/HTTPS + WebSocket + Custom protocols
- **Discovery Layer**: DHT-based peer discovery
- **Security Layer**: TLS/TLSLS + DPoP authentication

## Network Status and Monitoring

### Basic Network Status
```bash
# Check overall network status
pravyom network status

# Detailed network information
pravyom network status --detailed

# Network status with live updates
pravyom network status --detailed --refresh 10

# JSON output for monitoring systems
pravyom network status --format json
```

### Network Status Response
```json
{
  "network_info": {
    "network_id": "bpci-testnet",
    "protocol_version": "1.0.0",
    "chain_id": "bpci-test-001",
    "current_block": 125847,
    "sync_status": "synced",
    "sync_progress": "100.0%"
  },
  "node_info": {
    "node_id": "bpci_node_a1b2c3d4e5f6",
    "node_type": "validator",
    "uptime": "5d 12h 34m",
    "version": "1.0.0-beta",
    "capabilities": ["mining", "validation", "storage"]
  },
  "connectivity": {
    "connected_peers": 23,
    "max_peers": 50,
    "inbound_connections": 12,
    "outbound_connections": 11,
    "network_latency_avg": "45ms"
  },
  "performance": {
    "transactions_per_second": 1247,
    "blocks_per_hour": 60,
    "network_hashrate": "125.7 TH/s",
    "consensus_participation": "98.2%"
  }
}
```

### Peer Management
```bash
# List connected peers
pravyom network peers

# Detailed peer information
pravyom network peers --detailed --active-only

# Show peer statistics
pravyom network peer-stats --peer-id peer-node-id

# Filter peers by type
pravyom network peers --node-type validator --detailed
```

### Peer Information Response
```json
{
  "total_peers": 23,
  "peer_breakdown": {
    "validators": 8,
    "miners": 12,
    "oracles": 2,
    "storage": 1
  },
  "peers": [
    {
      "peer_id": "bpci_peer_x1y2z3a4b5c6",
      "node_type": "validator",
      "endpoint": "https://validator-01.bpci.network:8080",
      "status": "active",
      "latency": "32ms",
      "reputation": 9.7,
      "uptime": "99.8%",
      "last_seen": "2024-09-05T08:30:45Z",
      "capabilities": ["consensus", "validation", "relay"],
      "version": "1.0.0-beta",
      "location": "US-East-1"
    }
  ]
}
```

## Network Connectivity Operations

### Connecting to Peers
```bash
# Connect to specific peer
pravyom network connect \
    --peer-id bpci_peer_x1y2z3a4b5c6 \
    --address "validator-01.bpci.network:8080"

# Connect with authentication
pravyom network connect \
    --peer-id peer-id \
    --address "peer-address:port" \
    --auth-token "auth-token"

# Bulk connect to multiple peers
pravyom network connect-bulk \
    --peers-file ./peer-list.json \
    --max-concurrent 5
```

### Disconnecting from Peers
```bash
# Disconnect from specific peer
pravyom network disconnect peer-id

# Disconnect with reason
pravyom network disconnect peer-id --reason "maintenance"

# Disconnect all peers of specific type
pravyom network disconnect-type --node-type miner

# Force disconnect (immediate)
pravyom network disconnect peer-id --force
```

### Peer Discovery
```bash
# Discover new peers
pravyom network discover --max-peers 10

# Discover peers by type
pravyom network discover --node-type validator --region "US"

# Discover peers with specific capabilities
pravyom network discover --capabilities "storage,oracle"

# Manual peer addition
pravyom network add-peer \
    --address "new-peer.bpci.network:8080" \
    --node-type validator
```

## Network Synchronization

### Blockchain Synchronization
```bash
# Check sync status
pravyom network sync-status

# Force resync from specific block
pravyom network resync --from-block 100000

# Fast sync (download state snapshots)
pravyom network fast-sync --snapshot-height 125000

# Sync with specific peers
pravyom network sync --peers "peer1,peer2,peer3"
```

### Sync Status Response
```json
{
  "sync_status": "syncing",
  "current_block": 125847,
  "target_block": 125950,
  "sync_progress": "99.2%",
  "blocks_behind": 103,
  "estimated_completion": "2024-09-05T08:45:00Z",
  "sync_speed": "12.5 blocks/sec",
  "sync_peers": [
    {
      "peer_id": "sync_peer_1",
      "blocks_provided": 15234,
      "reliability": "98.7%"
    }
  ]
}
```

### State Synchronization
```bash
# Download state snapshot
pravyom network download-snapshot \
    --height 125000 \
    --output ./state-snapshot.tar.gz

# Apply state snapshot
pravyom network apply-snapshot \
    --snapshot ./state-snapshot.tar.gz \
    --verify-integrity

# Create state snapshot
pravyom network create-snapshot \
    --height current \
    --output ./my-snapshot.tar.gz \
    --compress
```

## Network Configuration

### Network Settings
```bash
# Show current network configuration
pravyom network config

# Update network configuration
pravyom network config set \
    --max-peers 100 \
    --connection-timeout 30 \
    --heartbeat-interval 10

# Reset to default configuration
pravyom network config reset

# Load configuration from file
pravyom network config load --file ./network-config.toml
```

### Network Configuration File
```toml
# ~/.config/bpci/network.toml
[network]
network_id = "bpci-testnet"
protocol_version = "1.0.0"
max_peers = 50
connection_timeout = 30
heartbeat_interval = 10
discovery_enabled = true

[connectivity]
listen_address = "0.0.0.0:8080"
external_address = "your-node.bpci.network:8080"
enable_upnp = true
enable_nat_traversal = true

[security]
require_tls = true
require_authentication = true
allowed_peer_types = ["validator", "miner", "oracle"]
banned_peers = []

[performance]
max_concurrent_connections = 100
buffer_size = 65536
compression_enabled = true
keep_alive_timeout = 300

[discovery]
bootstrap_nodes = [
    "bootstrap-1.bpci.network:8080",
    "bootstrap-2.bpci.network:8080"
]
discovery_interval = 60
peer_cache_size = 1000

[consensus]
consensus_timeout = 30
block_time = 60
max_block_size = 1048576
transaction_pool_size = 10000
```

## Network Testing and Diagnostics

### Connectivity Testing
```bash
# Test connection to specific node
pravyom network test --target "node.bpci.network:8080"

# Comprehensive network test
pravyom network test --comprehensive --timeout 60

# Test specific protocols
pravyom network test --protocol https --target "api.bpci.network"

# Latency testing
pravyom network test-latency --peers "peer1,peer2,peer3" --count 10
```

### Network Diagnostics
```bash
# Run network diagnostics
pravyom network diagnose

# Detailed diagnostics with recommendations
pravyom network diagnose --detailed --recommendations

# Test network performance
pravyom network performance-test --duration 300

# Bandwidth testing
pravyom network bandwidth-test --target peer-id --duration 60
```

### Diagnostic Results Example
```json
{
  "network_health": "good",
  "connectivity_score": 8.7,
  "performance_score": 9.2,
  "issues_found": [
    {
      "severity": "warning",
      "category": "connectivity",
      "description": "High latency to 2 peers",
      "recommendation": "Consider connecting to geographically closer peers"
    }
  ],
  "performance_metrics": {
    "average_latency": "45ms",
    "packet_loss": "0.1%",
    "bandwidth_utilization": "23%",
    "connection_stability": "98.7%"
  },
  "recommendations": [
    "Increase max_peers to 75 for better redundancy",
    "Enable compression to reduce bandwidth usage",
    "Consider upgrading network connection for better performance"
  ]
}
```

## Advanced Network Operations

### Network Statistics
```bash
# Show comprehensive network statistics
pravyom network stats

# Statistics for specific time period
pravyom network stats --period "last-week" --detailed

# Real-time statistics with live updates
pravyom network stats --live --refresh 5

# Export statistics to file
pravyom network stats --export csv --output ./network-stats.csv
```

### Network Metrics Response
```json
{
  "time_period": "last-24h",
  "traffic_stats": {
    "bytes_sent": "15.7 GB",
    "bytes_received": "18.2 GB",
    "messages_sent": 156789,
    "messages_received": 178234,
    "average_message_size": "1.2 KB"
  },
  "connection_stats": {
    "total_connections": 1247,
    "successful_connections": 1198,
    "failed_connections": 49,
    "connection_success_rate": "96.1%",
    "average_connection_duration": "4h 23m"
  },
  "performance_stats": {
    "average_latency": "42ms",
    "peak_latency": "156ms",
    "throughput": "1,247 TPS",
    "peak_throughput": "2,156 TPS",
    "uptime": "99.8%"
  }
}
```

### Firewall and Security
```bash
# Show firewall status
pravyom network firewall status

# Configure firewall rules
pravyom network firewall add-rule \
    --action allow \
    --source "trusted-peer-subnet" \
    --port 8080

# Block malicious peer
pravyom network firewall block-peer \
    --peer-id malicious-peer-id \
    --duration 3600

# Show security events
pravyom network security-events --limit 50 --severity high
```

### Load Balancing
```bash
# Show load balancing status
pravyom network load-balancer status

# Configure load balancing
pravyom network load-balancer configure \
    --algorithm round-robin \
    --health-check-interval 30

# Add backend peer
pravyom network load-balancer add-backend \
    --peer-id backend-peer-id \
    --weight 100

# Remove backend peer
pravyom network load-balancer remove-backend \
    --peer-id backend-peer-id
```

## Network Automation and Monitoring

### Automated Network Management
```bash
#!/bin/bash
# network-automation.sh

NETWORK="testnet"
MIN_PEERS=20
MAX_PEERS=50
LOG_FILE="/var/log/bpci/network-automation.log"

log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $1" | tee -a "$LOG_FILE"
}

check_network_health() {
    local status=$(pravyom --network "$NETWORK" network status --format json)
    local connected_peers=$(echo "$status" | jq -r '.connectivity.connected_peers')
    local sync_status=$(echo "$status" | jq -r '.network_info.sync_status')
    
    log "Network health check: $connected_peers peers, sync: $sync_status"
    
    # Check peer count
    if [ "$connected_peers" -lt "$MIN_PEERS" ]; then
        log "WARNING: Low peer count ($connected_peers), discovering new peers"
        discover_peers
    fi
    
    # Check sync status
    if [ "$sync_status" != "synced" ]; then
        log "WARNING: Node not synced ($sync_status), checking sync progress"
        check_sync_progress
    fi
    
    return 0
}

discover_peers() {
    log "Discovering new peers"
    pravyom --network "$NETWORK" network discover --max-peers 10
    
    if [ $? -eq 0 ]; then
        log "Peer discovery completed successfully"
    else
        log "ERROR: Peer discovery failed"
    fi
}

check_sync_progress() {
    local sync_status=$(pravyom --network "$NETWORK" network sync-status --format json)
    local progress=$(echo "$sync_status" | jq -r '.sync_progress' | sed 's/%//')
    local blocks_behind=$(echo "$sync_status" | jq -r '.blocks_behind')
    
    log "Sync progress: $progress%, $blocks_behind blocks behind"
    
    if [ "$blocks_behind" -gt 1000 ]; then
        log "WARNING: Significantly behind, initiating fast sync"
        pravyom --network "$NETWORK" network fast-sync
    fi
}

monitor_network_performance() {
    local stats=$(pravyom --network "$NETWORK" network stats --format json)
    local latency=$(echo "$stats" | jq -r '.performance_stats.average_latency' | sed 's/ms//')
    local throughput=$(echo "$stats" | jq -r '.performance_stats.throughput' | sed 's/ TPS//')
    
    log "Performance: ${latency}ms latency, ${throughput} TPS"
    
    # Alert on high latency
    if (( $(echo "$latency > 100" | bc -l) )); then
        log "WARNING: High network latency: ${latency}ms"
    fi
    
    # Alert on low throughput
    if (( $(echo "$throughput < 500" | bc -l) )); then
        log "WARNING: Low network throughput: ${throughput} TPS"
    fi
}

cleanup_stale_connections() {
    log "Cleaning up stale connections"
    
    # Get list of peers with high latency or low reputation
    local peers=$(pravyom --network "$NETWORK" network peers --format json | \
        jq -r '.peers[] | select(.latency > "200ms" or .reputation < 5.0) | .peer_id')
    
    for peer in $peers; do
        log "Disconnecting stale peer: $peer"
        pravyom --network "$NETWORK" network disconnect "$peer" --reason "performance"
    done
}

# Main monitoring loop
main() {
    log "Starting network automation for $NETWORK"
    
    while true; do
        check_network_health
        monitor_network_performance
        cleanup_stale_connections
        
        # Sleep for 5 minutes
        sleep 300
    done
}

# Handle signals
trap 'log "Shutting down network automation"; exit 0' SIGTERM SIGINT

# Start main loop
main "$@"
```

### Python Network Monitoring
```python
#!/usr/bin/env python3
# network_monitor.py

import subprocess
import json
import time
import logging
from datetime import datetime
from typing import Dict, List

class NetworkMonitor:
    def __init__(self, network: str = "testnet"):
        self.network = network
        self.base_cmd = ["pravyom", "--network", network, "--format", "json"]
        
        # Thresholds
        self.min_peers = 20
        self.max_latency = 100  # ms
        self.min_throughput = 500  # TPS
        
        # Setup logging
        logging.basicConfig(
            level=logging.INFO,
            format='%(asctime)s - %(levelname)s - %(message)s',
            handlers=[
                logging.FileHandler('/var/log/bpci/network-monitor.log'),
                logging.StreamHandler()
            ]
        )
        self.logger = logging.getLogger(__name__)
    
    def run_command(self, cmd: List[str]) -> Dict:
        """Execute BPCI CLI command"""
        full_cmd = self.base_cmd + cmd
        try:
            result = subprocess.run(full_cmd, capture_output=True, text=True, check=True)
            return json.loads(result.stdout)
        except subprocess.CalledProcessError as e:
            self.logger.error(f"Command failed: {' '.join(full_cmd)}")
            self.logger.error(f"Error: {e.stderr}")
            raise
    
    def get_network_status(self) -> Dict:
        """Get comprehensive network status"""
        return self.run_command(["network", "status", "--detailed"])
    
    def get_peer_list(self) -> Dict:
        """Get list of connected peers"""
        return self.run_command(["network", "peers", "--detailed"])
    
    def get_network_stats(self) -> Dict:
        """Get network performance statistics"""
        return self.run_command(["network", "stats"])
    
    def check_connectivity(self) -> bool:
        """Check network connectivity health"""
        try:
            status = self.get_network_status()
            connected_peers = status["connectivity"]["connected_peers"]
            sync_status = status["network_info"]["sync_status"]
            
            self.logger.info(f"Connectivity check: {connected_peers} peers, sync: {sync_status}")
            
            # Check minimum peer count
            if connected_peers < self.min_peers:
                self.logger.warning(f"Low peer count: {connected_peers} < {self.min_peers}")
                self.discover_peers()
                return False
            
            # Check sync status
            if sync_status != "synced":
                self.logger.warning(f"Node not synced: {sync_status}")
                return False
            
            return True
            
        except Exception as e:
            self.logger.error(f"Connectivity check failed: {e}")
            return False
    
    def discover_peers(self):
        """Discover and connect to new peers"""
        try:
            self.run_command(["network", "discover", "--max-peers", "10"])
            self.logger.info("Peer discovery initiated")
        except Exception as e:
            self.logger.error(f"Peer discovery failed: {e}")
    
    def check_performance(self) -> Dict:
        """Check network performance metrics"""
        try:
            stats = self.get_network_stats()
            performance = stats["performance_stats"]
            
            latency = float(performance["average_latency"].replace("ms", ""))
            throughput = float(performance["throughput"].replace(" TPS", ""))
            
            issues = []
            
            if latency > self.max_latency:
                issues.append(f"High latency: {latency}ms > {self.max_latency}ms")
                self.logger.warning(f"High network latency: {latency}ms")
            
            if throughput < self.min_throughput:
                issues.append(f"Low throughput: {throughput} TPS < {self.min_throughput} TPS")
                self.logger.warning(f"Low network throughput: {throughput} TPS")
            
            return {
                "latency": latency,
                "throughput": throughput,
                "issues": issues,
                "healthy": len(issues) == 0
            }
            
        except Exception as e:
            self.logger.error(f"Performance check failed: {e}")
            return {"healthy": False, "error": str(e)}
    
    def cleanup_poor_peers(self):
        """Disconnect from poorly performing peers"""
        try:
            peers = self.get_peer_list()
            
            for peer in peers["peers"]:
                peer_id = peer["peer_id"]
                latency = float(peer["latency"].replace("ms", ""))
                reputation = peer["reputation"]
                
                # Disconnect peers with high latency or low reputation
                if latency > 200 or reputation < 5.0:
                    self.logger.info(f"Disconnecting poor peer {peer_id}: {latency}ms latency, {reputation} reputation")
                    self.run_command(["network", "disconnect", peer_id, "--reason", "performance"])
                    
        except Exception as e:
            self.logger.error(f"Peer cleanup failed: {e}")
    
    def generate_network_report(self) -> Dict:
        """Generate comprehensive network health report"""
        try:
            status = self.get_network_status()
            peers = self.get_peer_list()
            stats = self.get_network_stats()
            
            report = {
                "timestamp": datetime.now().isoformat(),
                "network_health": "good",  # Will be updated based on checks
                "connectivity": {
                    "connected_peers": status["connectivity"]["connected_peers"],
                    "sync_status": status["network_info"]["sync_status"],
                    "current_block": status["network_info"]["current_block"]
                },
                "performance": {
                    "average_latency": stats["performance_stats"]["average_latency"],
                    "throughput": stats["performance_stats"]["throughput"],
                    "uptime": stats["performance_stats"]["uptime"]
                },
                "peer_breakdown": peers["peer_breakdown"],
                "issues": []
            }
            
            # Determine overall health
            connectivity_ok = self.check_connectivity()
            performance_check = self.check_performance()
            
            if not connectivity_ok:
                report["issues"].append("Connectivity issues detected")
                report["network_health"] = "degraded"
            
            if not performance_check["healthy"]:
                report["issues"].extend(performance_check["issues"])
                report["network_health"] = "degraded"
            
            if not report["issues"]:
                report["network_health"] = "excellent"
            
            self.logger.info(f"Network report generated: {report['network_health']} health")
            return report
            
        except Exception as e:
            self.logger.error(f"Report generation failed: {e}")
            return {"error": str(e)}
    
    def run_monitoring(self, interval: int = 300):
        """Run continuous network monitoring"""
        self.logger.info(f"Starting network monitoring (interval: {interval}s)")
        
        while True:
            try:
                # Check connectivity and performance
                self.check_connectivity()
                self.check_performance()
                
                # Cleanup poor performing peers
                self.cleanup_poor_peers()
                
                # Generate hourly reports
                if datetime.now().minute == 0:
                    report = self.generate_network_report()
                    self.logger.info(f"Hourly report: {json.dumps(report, indent=2)}")
                
                time.sleep(interval)
                
            except KeyboardInterrupt:
                self.logger.info("Network monitoring stopped by user")
                break
            except Exception as e:
                self.logger.error(f"Monitoring error: {e}")
                time.sleep(60)  # Wait 1 minute before retrying

# Example usage
if __name__ == "__main__":
    monitor = NetworkMonitor(network="testnet")
    monitor.run_monitoring(interval=300)  # Check every 5 minutes
```

## Troubleshooting Network Issues

### Common Network Problems

#### Issue 1: Cannot Connect to Network
```bash
# Check network configuration
pravyom network config

# Test basic connectivity
ping bootstrap-1.bpci.network

# Check firewall settings
sudo ufw status
sudo iptables -L

# Verify DNS resolution
nslookup bootstrap-1.bpci.network

# Check port availability
netstat -tlnp | grep 8080
```

#### Issue 2: Slow Synchronization
```bash
# Check sync status
pravyom network sync-status --detailed

# Try fast sync
pravyom network fast-sync --snapshot-height latest

# Check peer quality
pravyom network peers --detailed | grep -E "(latency|reputation)"

# Switch to better peers
pravyom network discover --node-type validator --region closest
```

#### Issue 3: High Network Latency
```bash
# Test latency to specific peers
pravyom network test-latency --peers "peer1,peer2" --count 10

# Check network path
traceroute peer-address

# Optimize network settings
pravyom network config set --connection-timeout 15 --heartbeat-interval 5

# Enable compression
pravyom network config set --compression-enabled true
```

#### Issue 4: Peer Connection Failures
```bash
# Check peer status
pravyom network peer-stats --peer-id failing-peer

# Test direct connection
pravyom network test --target "peer-address:port"

# Check authentication
pravyom network connect --peer-id peer-id --address peer-address --debug

# Review connection logs
pravyom network logs --component connectivity --level error
```

## Best Practices

### Network Security
1. **Use TLS encryption** for all peer connections
2. **Implement peer authentication** and reputation systems
3. **Regular security audits** of network configurations
4. **Monitor for malicious peers** and implement automatic blocking
5. **Keep network software updated** to latest versions

### Performance Optimization
1. **Maintain optimal peer count** (20-50 peers typically)
2. **Connect to geographically diverse peers** for redundancy
3. **Monitor and optimize network latency**
4. **Use compression** for bandwidth-limited connections
5. **Implement connection pooling** for high-traffic scenarios

### Operational Excellence
1. **Automate network monitoring** and health checks
2. **Implement alerting** for network issues
3. **Regular backup** of network configurations
4. **Document network topology** and peer relationships
5. **Plan for network upgrades** and maintenance windows

---

**Previous**: [Mining Operations Guide](03-mining-operations-guide.md)  
**Next**: [Advanced CLI Features](05-advanced-cli-features.md)  
**Related**: [Security Configuration](../07-firewall-and-security/), [Performance Monitoring](../30-monitoring-observability/)
