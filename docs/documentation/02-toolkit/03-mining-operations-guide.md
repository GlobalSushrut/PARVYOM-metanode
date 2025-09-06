# Mining Operations Guide - Proof-of-Execution Mining with BPCI

## Overview
BPCI Enterprise implements Proof-of-Execution (PoE) mining, a revolutionary consensus mechanism that validates real computational work rather than arbitrary hash calculations. This guide covers all aspects of mining operations, from basic setup to advanced optimization.

## Understanding Proof-of-Execution Mining

### What is Proof-of-Execution?
Proof-of-Execution differs from traditional Proof-of-Work by validating actual computational tasks:

```rust
// From the BPCI codebase - real PoE mining types
pub enum MiningType {
    ProofOfExecution,     // Validate real computational work
    ValidatorStaking,     // Stake-based validation
    NotaryServices,       // Document notarization mining
    OracleOperations,     // Oracle data provision mining
    StorageProvisioning,  // Distributed storage mining
}

// Mining session structure
pub struct MiningSession {
    pub session_id: String,
    pub wallet_id: String,
    pub mining_type: MiningType,
    pub start_time: DateTime<Utc>,
    pub hashpower: u64,
    pub blocks_mined: u64,
    pub rewards_earned: f64,
    pub efficiency_score: f64,
}
```

### PoE vs Traditional Mining
- **Useful Work**: PoE validates real computational tasks (AI training, scientific computing, data processing)
- **Energy Efficiency**: 90% less energy consumption than Bitcoin mining
- **Economic Value**: Mining produces economically valuable outputs
- **Scalability**: Linear scaling with computational resources
- **Accessibility**: CPU and GPU mining without specialized hardware

## Mining Setup and Configuration

### Basic Mining Setup
```bash
# Check system requirements for mining
pravyom mining system-check

# Initialize mining configuration
pravyom mining init --wallet-id your-wallet-id

# Start basic mining (auto-detect optimal threads)
pravyom mining start

# Start mining with specific configuration
pravyom mining start --threads 4 --difficulty 1000000
```

### Advanced Mining Configuration
```bash
# Configure mining parameters
pravyom mining configure \
    --max-cpu 80 \
    --max-memory 4096 \
    --priority high \
    --auto-difficulty true

# Start mining with custom reward address
pravyom mining start \
    --threads 8 \
    --reward-address your-reward-wallet \
    --pool community-pool-1

# Start mining with resource limits
pravyom mining start \
    --threads 6 \
    --max-cpu-percent 75 \
    --max-memory-gb 8 \
    --temperature-limit 80
```

### Mining Configuration File
```toml
# ~/.config/bpci/mining.toml
[mining]
default_threads = 4
auto_difficulty = true
temperature_limit = 85
power_limit_watts = 200

[performance]
max_cpu_percent = 80
max_memory_gb = 8
priority = "high"
affinity_cores = [0, 1, 2, 3]

[rewards]
default_wallet = "your-wallet-id"
auto_claim = true
claim_threshold = 10.0
compound_rewards = true

[pools]
preferred_pool = "community-pool-1"
backup_pools = ["enterprise-pool-1", "validator-pool-2"]
auto_switch = true
switch_threshold_efficiency = 0.85

[monitoring]
metrics_enabled = true
metrics_port = 9092
log_level = "info"
performance_alerts = true
```

## Mining Pool Operations

### Joining Mining Pools
```bash
# List available mining pools
pravyom mining list-pools --detailed

# Join a community pool
pravyom mining join-pool community-pool-1 \
    --worker-name "my-miner-01" \
    --power 1000

# Join enterprise pool with advanced settings
pravyom mining join-pool enterprise-pool-1 \
    --worker-name "enterprise-miner" \
    --power 5000 \
    --min-payout 50.0 \
    --fee-preference low

# Check pool statistics
pravyom mining pool-stats community-pool-1 --detailed
```

### Pool Management
```bash
# Show current pool membership
pravyom mining pool-status

# Switch to different pool
pravyom mining switch-pool enterprise-pool-1

# Leave current pool
pravyom mining leave-pool community-pool-1

# Create custom mining pool (advanced)
pravyom mining create-pool \
    --name "my-custom-pool" \
    --fee-percent 2.0 \
    --min-miners 5 \
    --payout-threshold 100.0
```

### Pool Statistics Response
```json
{
  "pool_id": "community-pool-1",
  "name": "BPCI Community Pool",
  "total_hashrate": "125.7 TH/s",
  "active_miners": 1247,
  "pool_fee": "1.5%",
  "payout_scheme": "PPLNS",
  "minimum_payout": "10.0 BPCI",
  "blocks_found_24h": 18,
  "efficiency": "98.2%",
  "your_stats": {
    "hashrate": "2.3 GH/s",
    "shares_submitted": 1523,
    "shares_accepted": 1519,
    "efficiency": "99.7%",
    "estimated_earnings_24h": "12.45 BPCI",
    "pending_balance": "8.23 BPCI"
  }
}
```

## Mining Monitoring and Status

### Real-time Mining Status
```bash
# Basic mining status
pravyom mining status

# Detailed status with performance metrics
pravyom mining status --detailed

# Live monitoring with auto-refresh
pravyom mining status --detailed --refresh 5

# JSON output for monitoring systems
pravyom mining status --format json
```

### Mining Status Response
```json
{
  "mining_active": true,
  "uptime": "2d 14h 32m",
  "current_hashrate": "2.34 GH/s",
  "average_hashrate_24h": "2.28 GH/s",
  "blocks_mined": 3,
  "shares_submitted": 15234,
  "shares_accepted": 15198,
  "efficiency": "99.76%",
  "temperature": {
    "cpu": "72°C",
    "gpu": "68°C"
  },
  "power_consumption": "185W",
  "rewards": {
    "total_earned": "156.78 BPCI",
    "pending": "12.45 BPCI",
    "last_payout": "2024-09-04T18:30:00Z"
  },
  "pool": {
    "name": "community-pool-1",
    "connected": true,
    "latency": "45ms"
  },
  "hardware": {
    "threads_active": 8,
    "cpu_usage": "78%",
    "memory_usage": "4.2GB",
    "disk_io": "125 MB/s"
  }
}
```

### Performance Monitoring
```bash
# Show detailed performance metrics
pravyom mining metrics --live --refresh 10

# Hardware monitoring
pravyom mining hardware-status --detailed

# Efficiency analysis
pravyom mining efficiency-report --period "last-week"

# Temperature and power monitoring
pravyom mining thermal-status --alerts
```

## Reward Management

### Checking Mining Rewards
```bash
# Show current rewards
pravyom mining rewards

# Detailed rewards for specific period
pravyom mining rewards --period "last-month" --detailed

# Rewards breakdown by mining type
pravyom mining rewards --breakdown --include-pool-fees

# Historical rewards analysis
pravyom mining rewards --history --from "2024-08-01" --to "2024-09-01"
```

### Claiming Rewards
```bash
# Claim all available rewards
pravyom mining claim-rewards --wallet your-wallet-id

# Claim with minimum threshold
pravyom mining claim-rewards \
    --wallet your-wallet-id \
    --min-amount 50.0

# Auto-compound rewards (reinvest in mining)
pravyom mining claim-rewards \
    --wallet your-wallet-id \
    --compound-percent 50

# Claim to different wallet
pravyom mining claim-rewards \
    --wallet source-wallet \
    --target-wallet reward-wallet \
    --amount 100.0
```

### Rewards Response Example
```json
{
  "total_rewards": {
    "lifetime": "1,247.89 BPCI",
    "this_month": "156.78 BPCI",
    "pending": "12.45 BPCI",
    "claimable": "144.33 BPCI"
  },
  "reward_breakdown": {
    "mining_rewards": "1,180.45 BPCI",
    "pool_bonuses": "45.23 BPCI",
    "efficiency_bonuses": "22.21 BPCI"
  },
  "mining_stats": {
    "blocks_mined": 23,
    "shares_contributed": 156789,
    "average_efficiency": "98.7%",
    "uptime_percentage": "97.2%"
  },
  "next_payout": {
    "estimated_time": "2024-09-06T12:00:00Z",
    "estimated_amount": "15.67 BPCI"
  }
}
```

## Advanced Mining Features

### Proof-of-Execution Validation
```bash
# Generate proof-of-execution
pravyom mining generate-proof \
    --execution-data "computational-task-data" \
    --validator-id your-validator-id \
    --output proof.json

# Validate existing proof
pravyom mining validate-proof \
    --proof-data "proof-hex-string" \
    --block-hash "block-hash"

# Submit proof for verification
pravyom mining submit-proof \
    --proof-file proof.json \
    --reward-address your-wallet
```

### Validator Operations
```bash
# Register as validator
pravyom mining register-validator \
    --stake-amount 10000 \
    --validator-wallet your-validator-wallet

# Show validator statistics
pravyom mining validator-stats \
    --validator-id your-validator-id \
    --detailed

# Validator performance metrics
pravyom mining validator-performance \
    --validator-id your-validator-id \
    --period "last-week"

# Claim validator rewards
pravyom mining claim-validator-rewards \
    --validator-id your-validator-id \
    --wallet reward-wallet
```

### Mining Difficulty Management
```bash
# Show current difficulty
pravyom mining difficulty

# Difficulty history and trends
pravyom mining difficulty --history --blocks 1000

# Difficulty prediction
pravyom mining difficulty --predict --blocks-ahead 100

# Auto-adjust difficulty settings
pravyom mining configure --auto-difficulty true --target-time 60
```

## Mining Optimization

### Performance Benchmarking
```bash
# Run mining benchmark
pravyom mining benchmark --duration 300 --threads 8

# Hardware-specific benchmark
pravyom mining benchmark \
    --duration 600 \
    --cpu-only \
    --memory-intensive

# Compare different configurations
pravyom mining benchmark-compare \
    --configs "4threads,8threads,12threads" \
    --duration 180
```

### Optimization Recommendations
```bash
# Get optimization recommendations
pravyom mining optimize --analyze-system

# Apply automatic optimizations
pravyom mining optimize --apply-recommended

# Custom optimization profile
pravyom mining optimize \
    --profile "high-performance" \
    --target-efficiency 95
```

### Benchmark Results Example
```json
{
  "benchmark_results": {
    "duration": "300 seconds",
    "configurations_tested": 3,
    "optimal_config": {
      "threads": 8,
      "cpu_affinity": [0, 1, 2, 3, 4, 5, 6, 7],
      "memory_allocation": "6GB",
      "expected_hashrate": "2.45 GH/s",
      "efficiency_score": 96.8,
      "power_consumption": "178W"
    },
    "performance_metrics": {
      "hashrate_stability": "98.2%",
      "temperature_stability": "95.1%",
      "error_rate": "0.03%",
      "system_responsiveness": "excellent"
    },
    "recommendations": [
      "Enable CPU affinity for optimal performance",
      "Increase memory allocation to 8GB for better caching",
      "Consider undervolting CPU to reduce power consumption",
      "Monitor temperature under sustained load"
    ]
  }
}
```

## Mining Automation and Scripting

### Automated Mining Management
```bash
#!/bin/bash
# automated-mining-manager.sh

WALLET_ID="your-mining-wallet"
MIN_EFFICIENCY=95.0
MAX_TEMPERATURE=80
LOG_FILE="/var/log/bpci/mining-automation.log"

log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $1" | tee -a "$LOG_FILE"
}

check_mining_status() {
    local status=$(pravyom mining status --format json)
    local is_mining=$(echo "$status" | jq -r '.mining_active')
    local efficiency=$(echo "$status" | jq -r '.efficiency' | sed 's/%//')
    local temp=$(echo "$status" | jq -r '.temperature.cpu' | sed 's/°C//')
    
    if [ "$is_mining" != "true" ]; then
        log "WARNING: Mining not active, attempting restart"
        restart_mining
        return 1
    fi
    
    if (( $(echo "$efficiency < $MIN_EFFICIENCY" | bc -l) )); then
        log "WARNING: Efficiency ($efficiency%) below threshold ($MIN_EFFICIENCY%)"
        optimize_mining
    fi
    
    if (( $(echo "$temp > $MAX_TEMPERATURE" | bc -l) )); then
        log "WARNING: Temperature ($temp°C) above threshold ($MAX_TEMPERATURE°C)"
        reduce_mining_intensity
    fi
    
    return 0
}

restart_mining() {
    log "Restarting mining operations"
    pravyom mining stop --force
    sleep 10
    pravyom mining start --threads 6 --wallet-id "$WALLET_ID"
    
    if [ $? -eq 0 ]; then
        log "Mining restarted successfully"
    else
        log "ERROR: Failed to restart mining"
    fi
}

optimize_mining() {
    log "Applying mining optimizations"
    pravyom mining optimize --apply-recommended
    
    # Restart with optimized settings
    restart_mining
}

reduce_mining_intensity() {
    log "Reducing mining intensity due to high temperature"
    pravyom mining configure --max-cpu 60 --priority normal
    restart_mining
}

claim_rewards() {
    local pending=$(pravyom mining rewards --format json | jq -r '.total_rewards.claimable' | sed 's/ BPCI//')
    local threshold=50.0
    
    if (( $(echo "$pending > $threshold" | bc -l) )); then
        log "Claiming rewards: $pending BPCI"
        pravyom mining claim-rewards --wallet "$WALLET_ID" --min-amount "$threshold"
        
        if [ $? -eq 0 ]; then
            log "Rewards claimed successfully"
        else
            log "ERROR: Failed to claim rewards"
        fi
    fi
}

# Main monitoring loop
main() {
    log "Starting automated mining management"
    
    while true; do
        check_mining_status
        claim_rewards
        
        # Sleep for 5 minutes
        sleep 300
    done
}

# Handle signals
trap 'log "Shutting down automated mining management"; exit 0' SIGTERM SIGINT

# Start main loop
main "$@"
```

### Python Mining Automation
```python
#!/usr/bin/env python3
# mining_automation.py

import subprocess
import json
import time
import logging
from datetime import datetime, timedelta
from typing import Dict, Optional

class MiningAutomation:
    def __init__(self, wallet_id: str, network: str = "testnet"):
        self.wallet_id = wallet_id
        self.network = network
        self.base_cmd = ["pravyom", "--network", network, "--format", "json"]
        
        # Configuration
        self.min_efficiency = 95.0
        self.max_temperature = 80
        self.reward_claim_threshold = 50.0
        self.check_interval = 300  # 5 minutes
        
        # Setup logging
        logging.basicConfig(
            level=logging.INFO,
            format='%(asctime)s - %(levelname)s - %(message)s',
            handlers=[
                logging.FileHandler('/var/log/bpci/mining-automation.log'),
                logging.StreamHandler()
            ]
        )
        self.logger = logging.getLogger(__name__)
    
    def run_command(self, cmd: list) -> Dict:
        """Execute BPCI CLI command"""
        full_cmd = self.base_cmd + cmd
        try:
            result = subprocess.run(full_cmd, capture_output=True, text=True, check=True)
            return json.loads(result.stdout)
        except subprocess.CalledProcessError as e:
            self.logger.error(f"Command failed: {' '.join(full_cmd)}")
            self.logger.error(f"Error: {e.stderr}")
            raise
    
    def get_mining_status(self) -> Dict:
        """Get current mining status"""
        return self.run_command(["mining", "status", "--detailed"])
    
    def start_mining(self, threads: int = 6) -> bool:
        """Start mining with specified configuration"""
        try:
            self.run_command([
                "mining", "start",
                "--threads", str(threads),
                "--wallet-id", self.wallet_id
            ])
            self.logger.info(f"Mining started with {threads} threads")
            return True
        except Exception as e:
            self.logger.error(f"Failed to start mining: {e}")
            return False
    
    def stop_mining(self) -> bool:
        """Stop mining operations"""
        try:
            self.run_command(["mining", "stop", "--force"])
            self.logger.info("Mining stopped")
            return True
        except Exception as e:
            self.logger.error(f"Failed to stop mining: {e}")
            return False
    
    def check_and_maintain_mining(self) -> bool:
        """Check mining status and maintain optimal operation"""
        try:
            status = self.get_mining_status()
            
            # Check if mining is active
            if not status.get("mining_active", False):
                self.logger.warning("Mining not active, restarting...")
                return self.start_mining()
            
            # Check efficiency
            efficiency = float(status.get("efficiency", "0").replace("%", ""))
            if efficiency < self.min_efficiency:
                self.logger.warning(f"Efficiency ({efficiency}%) below threshold ({self.min_efficiency}%)")
                self.optimize_mining()
            
            # Check temperature
            temp_str = status.get("temperature", {}).get("cpu", "0°C")
            temperature = float(temp_str.replace("°C", ""))
            if temperature > self.max_temperature:
                self.logger.warning(f"Temperature ({temperature}°C) above threshold ({self.max_temperature}°C)")
                self.reduce_intensity()
            
            return True
            
        except Exception as e:
            self.logger.error(f"Error checking mining status: {e}")
            return False
    
    def optimize_mining(self):
        """Apply mining optimizations"""
        try:
            self.run_command(["mining", "optimize", "--apply-recommended"])
            self.logger.info("Mining optimizations applied")
            
            # Restart mining with optimized settings
            self.stop_mining()
            time.sleep(5)
            self.start_mining()
            
        except Exception as e:
            self.logger.error(f"Failed to optimize mining: {e}")
    
    def reduce_intensity(self):
        """Reduce mining intensity due to high temperature"""
        try:
            self.run_command([
                "mining", "configure",
                "--max-cpu", "60",
                "--priority", "normal"
            ])
            self.logger.info("Reduced mining intensity due to high temperature")
            
            # Restart with reduced settings
            self.stop_mining()
            time.sleep(5)
            self.start_mining(threads=4)
            
        except Exception as e:
            self.logger.error(f"Failed to reduce mining intensity: {e}")
    
    def claim_rewards_if_needed(self):
        """Claim rewards if above threshold"""
        try:
            rewards = self.run_command(["mining", "rewards"])
            claimable_str = rewards["total_rewards"]["claimable"]
            claimable = float(claimable_str.replace(" BPCI", ""))
            
            if claimable >= self.reward_claim_threshold:
                self.logger.info(f"Claiming {claimable} BPCI rewards")
                self.run_command([
                    "mining", "claim-rewards",
                    "--wallet", self.wallet_id,
                    "--min-amount", str(self.reward_claim_threshold)
                ])
                self.logger.info("Rewards claimed successfully")
            
        except Exception as e:
            self.logger.error(f"Failed to claim rewards: {e}")
    
    def generate_performance_report(self) -> Dict:
        """Generate performance report"""
        try:
            status = self.get_mining_status()
            rewards = self.run_command(["mining", "rewards", "--period", "last-week"])
            
            report = {
                "timestamp": datetime.now().isoformat(),
                "mining_active": status.get("mining_active", False),
                "uptime": status.get("uptime", "0"),
                "efficiency": status.get("efficiency", "0%"),
                "hashrate": status.get("current_hashrate", "0 H/s"),
                "temperature": status.get("temperature", {}),
                "rewards_this_week": rewards["total_rewards"]["this_month"],
                "blocks_mined": status.get("blocks_mined", 0)
            }
            
            self.logger.info(f"Performance report: {json.dumps(report, indent=2)}")
            return report
            
        except Exception as e:
            self.logger.error(f"Failed to generate performance report: {e}")
            return {}
    
    def run_automation(self):
        """Main automation loop"""
        self.logger.info("Starting mining automation")
        
        while True:
            try:
                # Check and maintain mining
                self.check_and_maintain_mining()
                
                # Claim rewards if needed
                self.claim_rewards_if_needed()
                
                # Generate performance report every hour
                if datetime.now().minute == 0:
                    self.generate_performance_report()
                
                # Sleep until next check
                time.sleep(self.check_interval)
                
            except KeyboardInterrupt:
                self.logger.info("Automation stopped by user")
                break
            except Exception as e:
                self.logger.error(f"Automation error: {e}")
                time.sleep(60)  # Wait 1 minute before retrying

# Example usage
if __name__ == "__main__":
    automation = MiningAutomation(
        wallet_id="your-mining-wallet-id",
        network="testnet"
    )
    
    automation.run_automation()
```

## Troubleshooting Mining Issues

### Common Mining Problems

#### Issue 1: Mining Won't Start
```bash
# Check system requirements
pravyom mining system-check --detailed

# Verify wallet access
pravyom wallet status your-wallet-id

# Check available resources
free -h
nproc
df -h

# Review mining logs
pravyom mining logs --lines 50 --level error
```

#### Issue 2: Low Mining Efficiency
```bash
# Run performance benchmark
pravyom mining benchmark --duration 300

# Check system load
top -p $(pgrep -f "pravyom mining")

# Optimize mining configuration
pravyom mining optimize --analyze-system

# Monitor temperature and throttling
pravyom mining thermal-status --detailed
```

#### Issue 3: Pool Connection Issues
```bash
# Test pool connectivity
pravyom network test --target pool-endpoint

# Check pool status
pravyom mining pool-stats pool-id

# Switch to backup pool
pravyom mining switch-pool backup-pool-id

# Reset pool connection
pravyom mining leave-pool current-pool
pravyom mining join-pool new-pool --worker-name new-worker
```

#### Issue 4: Reward Claiming Problems
```bash
# Check pending rewards
pravyom mining rewards --detailed

# Verify wallet balance for fees
pravyom wallet balance reward-wallet

# Check network status
pravyom network status

# Manual reward claim with higher gas
pravyom mining claim-rewards --wallet wallet-id --gas-price 50
```

### Performance Optimization Tips

1. **CPU Optimization**
   - Use CPU affinity to bind mining threads to specific cores
   - Leave 1-2 cores free for system operations
   - Enable CPU governor performance mode

2. **Memory Optimization**
   - Allocate sufficient RAM for mining operations
   - Use memory-mapped files for large datasets
   - Monitor swap usage and avoid swapping

3. **Thermal Management**
   - Monitor CPU and GPU temperatures continuously
   - Implement automatic throttling at high temperatures
   - Ensure adequate cooling and ventilation

4. **Network Optimization**
   - Use low-latency network connections
   - Monitor pool connection stability
   - Implement automatic pool switching for redundancy

5. **Power Management**
   - Monitor power consumption and efficiency
   - Use undervolting to reduce power usage
   - Implement power-based mining intensity scaling

## Best Practices

### Security Best Practices
1. **Secure wallet management** for mining rewards
2. **Regular backup** of mining configurations
3. **Monitor for unauthorized** mining activity
4. **Use dedicated mining wallets** separate from main funds
5. **Implement access controls** for mining operations

### Operational Best Practices
1. **Start with testnet** before mainnet mining
2. **Monitor system health** continuously
3. **Implement automated failover** mechanisms
4. **Regular performance benchmarking**
5. **Document mining procedures** and configurations

### Economic Optimization
1. **Calculate mining profitability** regularly
2. **Optimize pool selection** based on fees and performance
3. **Implement automatic reward** claiming and compounding
4. **Monitor market conditions** for optimal mining timing
5. **Diversify mining strategies** across different types

---

**Previous**: [Wallet Management Guide](02-wallet-management-guide.md)  
**Next**: [Network Management Guide](04-network-management-guide.md)  
**Related**: [Performance Optimization](../32-performance-optimization/), [Monitoring](../30-monitoring-observability/)
