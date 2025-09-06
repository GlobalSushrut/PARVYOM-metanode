# 🛠️ METANODE CLI ADJUNCT GUIDE
## Practical Implementation Guide & Workflows

**Version:** 1.0.0  
**Date:** 2025-08-14  
**Companion to:** METANODE_CLI_BOOK.md  
**Status:** Production Ready - Validated Infrastructure  

---

## 📋 TABLE OF CONTENTS

1. [Quick Start Guide](#quick-start-guide)
2. [Essential Workflows](#essential-workflows)
3. [Production Deployment](#production-deployment)
4. [Banking Integration Workflows](#banking-integration-workflows)
5. [Security Implementation](#security-implementation)
6. [Enterprise Setup](#enterprise-setup)
7. [Development Workflows](#development-workflows)
8. [Monitoring & Operations](#monitoring--operations)
9. [Troubleshooting Playbook](#troubleshooting-playbook)
10. [Best Practices](#best-practices)

---

## 🚀 QUICK START GUIDE

### 30-Second Setup
```bash
# Install Metanode
curl -sSL https://metanode.sh | bash

# Initialize and start
metanode init --network testnet
metanode start --daemon

# Verify installation
metanode status
metanode health
```

### 5-Minute Banking Demo
```bash
# Register a test bank
metanode bank register --name "Demo Bank" --jurisdiction "US" --dry-run
metanode bank register --name "Demo Bank" --jurisdiction "US"

# Run proof of reserves
metanode bank por run --fiat USD --gold LBMA --publish

# Issue and activate a coin
COIN_ID=$(metanode coin issue --type mother --json | jq -r '.coin_id')
metanode coin activate $COIN_ID --job $(metanode poh status --json | jq -r '.latest_tick')

# Check status
metanode coin status $COIN_ID
metanode bank info --json
```

### 10-Minute Enterprise Setup
```bash
# Start enterprise node
metanode enterprise start --config enterprise.toml

# Initialize ENC cluster
metanode enc cluster init --name "prod-cluster"

# Deploy DockLock cage
metanode docklock cage create "secure-env"
metanode docklock cage start secure-env

# Enable security features
metanode quantum keygen kyber1024
metanode ai-security start
metanode zk setup default-circuit

# Verify enterprise readiness
metanode enterprise status --detailed
```

---

## 🔄 ESSENTIAL WORKFLOWS

### Node Lifecycle Workflow
```bash
#!/bin/bash
# Complete node lifecycle management

# 1. Initialize
metanode init --network mainnet
metanode config identity generate

# 2. Configure
metanode config set consensus.timeout 5000
metanode config set network.max_peers 50
metanode config set storage.cache_size 1GB

# 3. Start with monitoring
metanode start --daemon
metanode monitor status --follow &

# 4. Health checks
while true; do
    if metanode health --json | jq -r '.status' == "healthy"; then
        echo "✅ Node healthy"
        break
    fi
    sleep 5
done

# 5. Join network
metanode config network bootstrap
metanode consensus status
```

### Validator Setup Workflow
```bash
#!/bin/bash
# Complete validator setup

# 1. Register as validator
metanode validator register \
    --moniker "MyValidator" \
    --commission 0.05 \
    --min-self-delegation 1000

# 2. Stake tokens
metanode stake delegate $(metanode config identity address) 10000

# 3. Monitor validator status
metanode validator info $(metanode config identity address)
metanode consensus peers | grep $(metanode config identity address)

# 4. Set up monitoring
metanode monitor alerts --validator-mode
metanode analytics network validators --watch
```

### Cross-Border Settlement Workflow
```bash
#!/bin/bash
# Complete cross-border settlement

# 1. Setup banks
BANK_A=$(metanode bank register --name "Bank A" --jurisdiction "US" --json | jq -r '.bank_id')
BANK_B=$(metanode bank register --name "Bank B" --jurisdiction "EU" --json | jq -r '.bank_id')

# 2. Run PoR for both banks
metanode bank por run --bank $BANK_A --fiat USD --gold LBMA
metanode bank por run --bank $BANK_B --fiat EUR --gold LBMA

# 3. Execute settlement
SETTLEMENT_ID=$(metanode settle xborder \
    --from USD --to EUR \
    --amount 100000 \
    --via gold \
    --receipt \
    --json | jq -r '.settlement_id')

# 4. Monitor settlement
while true; do
    STATUS=$(metanode settle status $SETTLEMENT_ID --json | jq -r '.status')
    echo "Settlement status: $STATUS"
    [[ "$STATUS" == "completed" ]] && break
    sleep 10
done

# 5. Generate compliance report
metanode audit generate settlement $SETTLEMENT_ID
```

---

## 🏭 PRODUCTION DEPLOYMENT

### Production Node Setup
```bash
#!/bin/bash
# Production-grade node deployment

# 1. System preparation
metanode system check --production
metanode install verify --all-components

# 2. Security hardening
metanode quantum keygen --all-algorithms
metanode ai-security start --production-mode
metanode biso policy apply production-policy

# 3. High availability setup
metanode cluster create prod-cluster \
    --nodes 5 \
    --consensus ibft \
    --replication 3

# 4. Monitoring setup
metanode monitor dashboard --production
metanode analytics enable --all-metrics
metanode audit enable --continuous

# 5. Backup configuration
metanode config backup --encrypted
metanode maintenance db backup --schedule daily

# 6. Start production services
metanode start --production --daemon
metanode enterprise start --cluster prod-cluster
```

### Load Balancer Configuration
```bash
#!/bin/bash
# Production load balancing

# 1. Create cluster
metanode cluster create lb-cluster --type load-balanced

# 2. Add nodes
for i in {1..5}; do
    metanode cluster lb add-node node-$i \
        --weight 100 \
        --health-check-interval 30
done

# 3. Configure policies
metanode cluster lb policies set \
    --algorithm round-robin \
    --sticky-sessions true \
    --failover-timeout 10

# 4. Enable health monitoring
metanode cluster lb health-checks enable
metanode monitor cluster --live
```

---

## 🏦 BANKING INTEGRATION WORKFLOWS

### Bank Onboarding Workflow
```bash
#!/bin/bash
# Complete bank onboarding process

# 1. Pre-registration validation
metanode bank verify-eligibility \
    --name "ACME Bank" \
    --jurisdiction "US" \
    --license-number "12345"

# 2. Register bank
BANK_ID=$(metanode bank register \
    --name "ACME Bank" \
    --jurisdiction "US" \
    --license-number "12345" \
    --contact-email "compliance@acmebank.com" \
    --json | jq -r '.bank_id')

# 3. Setup compliance
metanode bank compliance-setup $BANK_ID \
    --kyc-provider "Jumio" \
    --aml-provider "Chainalysis" \
    --reporting-frequency "monthly"

# 4. Configure PoR
metanode bank por configure $BANK_ID \
    --fiat-currencies "USD,EUR,GBP" \
    --gold-standard "LBMA" \
    --audit-frequency "daily"

# 5. Initial PoR run
metanode bank por run --bank $BANK_ID --all-currencies
metanode bank por publish $BANK_ID

# 6. Join bank mesh
metanode mesh join --bank $BANK_ID
metanode mesh liquidity setup $BANK_ID --initial-amount 1000000
```

### Multi-Currency Settlement
```bash
#!/bin/bash
# Multi-currency settlement workflow

# 1. Setup currency pairs
metanode settle fx configure \
    --pairs "USD/EUR,USD/GBP,EUR/GBP,USD/JPY" \
    --oracle-providers "Chainlink,Band,API3"

# 2. Create liquidity pools
for pair in "USD/EUR" "USD/GBP" "EUR/GBP"; do
    metanode settle liquidity create-pool $pair \
        --initial-liquidity 500000 \
        --fee-tier 0.3
done

# 3. Execute multi-leg settlement
SETTLEMENT_ID=$(metanode settle xborder \
    --route "USD->EUR->GBP" \
    --amount 250000 \
    --max-slippage 0.5 \
    --receipt \
    --json | jq -r '.settlement_id')

# 4. Monitor and report
metanode settle status $SETTLEMENT_ID --follow
metanode analytics settlement $SETTLEMENT_ID --detailed
```

---

## 🛡️ SECURITY IMPLEMENTATION

### Quantum Security Setup
```bash
#!/bin/bash
# Complete quantum security implementation

# 1. Generate quantum-resistant keys
metanode quantum keygen kyber1024 --name primary-key
metanode quantum keygen dilithium5 --name signing-key
metanode quantum keygen falcon1024 --name backup-key

# 2. Setup key rotation
metanode quantum keys rotate-schedule \
    --interval "24h" \
    --backup-count 5 \
    --secure-delete true

# 3. Migrate existing keys
metanode quantum migrate start \
    --from classical \
    --to quantum \
    --schedule gradual

# 4. Verify quantum readiness
metanode quantum verify-setup
metanode security posture --quantum
```

### AI Security Configuration
```bash
#!/bin/bash
# AI-powered security setup

# 1. Initialize AI security
metanode ai-security start --models all

# 2. Configure anomaly detection
metanode ai-security anomaly configure \
    --sensitivity high \
    --learning-period 7d \
    --alert-threshold 0.8

# 3. Setup behavioral analysis
metanode ai-security behavior baseline \
    --entities "validators,banks,users" \
    --metrics "transaction_patterns,network_behavior,consensus_participation"

# 4. Configure automated response
metanode ai-security response configure \
    --auto-quarantine true \
    --alert-channels "slack,email,sms" \
    --escalation-policy security-team

# 5. Enable continuous monitoring
metanode ai-security monitor start --continuous
```

### Zero-Knowledge Privacy Setup
```bash
#!/bin/bash
# ZK privacy implementation

# 1. Setup ZK circuits
metanode zk setup compliance-circuit --type snark
metanode zk setup privacy-circuit --type stark
metanode zk setup audit-circuit --type bulletproof

# 2. Generate proving keys
metanode zk keygen compliance-circuit
metanode zk keygen privacy-circuit
metanode zk keygen audit-circuit

# 3. Configure privacy policies
metanode zk privacy configure \
    --selective-disclosure true \
    --audit-compliance true \
    --data-minimization true

# 4. Test ZK proofs
metanode zk prove compliance-circuit --test-data sample.json
metanode zk verify --proof compliance.proof --public-inputs public.json
```

---

## 🏢 ENTERPRISE SETUP

### BPCI Enterprise Deployment
```bash
#!/bin/bash
# Complete BPCI Enterprise setup

# 1. Initialize enterprise cluster
metanode enterprise cluster create enterprise-prod \
    --type bpci \
    --consensus ibft \
    --validators 7

# 2. Configure external anchoring
metanode enterprise anchor chains add ethereum \
    --rpc-url "https://eth-mainnet.alchemyapi.io/v2/API_KEY" \
    --gas-limit 500000 \
    --confirmations 12

metanode enterprise anchor chains add polygon \
    --rpc-url "https://polygon-mainnet.alchemyapi.io/v2/API_KEY" \
    --gas-limit 300000 \
    --confirmations 20

# 3. Setup headers proxy
metanode enterprise headers proxy start \
    --cache-size 10GB \
    --rate-limit 10000 \
    --compression gzip

# 4. Configure billing
metanode enterprise billing configure \
    --metering-interval 1h \
    --billing-cycle monthly \
    --payment-methods "crypto,fiat"

# 5. Enable enterprise features
metanode enterprise features enable \
    --advanced-consensus \
    --enterprise-api \
    --premium-support
```

### DockLock Production Setup
```bash
#!/bin/bash
# Production DockLock deployment

# 1. Create production cages
metanode docklock cage create prod-banking \
    --isolation hardware \
    --witness-recording true \
    --policy-enforcement strict

metanode docklock cage create prod-settlement \
    --isolation hypervisor \
    --witness-recording true \
    --policy-enforcement strict

# 2. Configure BISO policies
metanode docklock policy create banking-policy.hcl
metanode docklock policy create settlement-policy.hcl
metanode docklock policy apply banking-policy prod-banking
metanode docklock policy apply settlement-policy prod-settlement

# 3. Setup witness recording
metanode docklock witness record prod-banking --continuous
metanode docklock witness record prod-settlement --continuous

# 4. Configure receipt generation
metanode docklock receipt configure \
    --auto-generate true \
    --signing-key production-key \
    --merkle-tree-depth 20

# 5. Start production workloads
metanode docklock exec prod-banking --script banking-app.sh
metanode docklock exec prod-settlement --script settlement-service.sh
```

### ENC Cluster Production
```bash
#!/bin/bash
# ENC cluster production deployment

# 1. Initialize production cluster
metanode enc cluster init --name prod-enc \
    --consensus-based-scheduling true \
    --receipt-generation true

# 2. Add cluster nodes
for i in {1..10}; do
    metanode enc node register enc-node-$i \
        --resources "cpu=16,memory=64GB,storage=1TB" \
        --labels "zone=us-east-1a,tier=production"
done

# 3. Deploy production workloads
metanode enc deploy banking-service.yaml \
    --namespace banking \
    --replicas 5 \
    --policy strict-isolation

metanode enc deploy settlement-service.yaml \
    --namespace settlement \
    --replicas 3 \
    --policy compliance-required

# 4. Configure service mesh
metanode enc mesh configure \
    --encryption required \
    --mutual-tls true \
    --policy-enforcement strict

# 5. Setup monitoring
metanode enc monitor enable --all-metrics
metanode enc alerts configure --production-rules
```

---

## 💻 DEVELOPMENT WORKFLOWS

### Development Environment Setup
```bash
#!/bin/bash
# Complete development environment

# 1. Initialize dev environment
metanode dev init --template full-stack
metanode dev testnet create --validators 4 --fast-blocks

# 2. Setup development tools
metanode dev tools install \
    --debugger \
    --profiler \
    --test-runner \
    --mock-services

# 3. Configure testing
metanode test configure \
    --parallel-execution true \
    --coverage-reporting true \
    --integration-tests true

# 4. Setup continuous testing
metanode test watch --components "consensus,mempool,banking"
metanode test coverage --threshold 90

# 5. Development shortcuts
alias mn="metanode"
alias mntest="metanode test run --fast"
alias mnlog="metanode logs --follow --level debug"
```

### Testing Workflow
```bash
#!/bin/bash
# Comprehensive testing workflow

# 1. Unit tests
metanode test run unit --parallel --coverage

# 2. Integration tests
metanode test run integration --components all

# 3. Performance tests
metanode test run performance \
    --duration 10m \
    --load-pattern ramp-up \
    --metrics-export prometheus

# 4. Security tests
metanode test run security \
    --penetration-testing \
    --vulnerability-scanning \
    --compliance-checking

# 5. End-to-end tests
metanode test run e2e \
    --scenarios "banking,settlement,governance" \
    --real-network testnet

# 6. Generate test report
metanode test report generate \
    --format html \
    --include-coverage \
    --include-performance
```

---

## 📊 MONITORING & OPERATIONS

### Production Monitoring Setup
```bash
#!/bin/bash
# Complete monitoring setup

# 1. Enable all monitoring
metanode monitor enable --all-components
metanode analytics enable --all-metrics
metanode audit enable --continuous

# 2. Configure dashboards
metanode monitor dashboard create production \
    --components "consensus,banking,settlement,security" \
    --refresh-interval 30s

# 3. Setup alerting
metanode monitor alerts configure \
    --channels "slack,pagerduty,email" \
    --severity-levels "warning,error,critical" \
    --escalation-policy ops-team

# 4. Performance monitoring
metanode monitor performance \
    --metrics "throughput,latency,resource-usage" \
    --thresholds "throughput>1000tps,latency<100ms"

# 5. Security monitoring
metanode monitor security \
    --anomaly-detection true \
    --threat-intelligence true \
    --compliance-monitoring true
```

### Operational Playbooks
```bash
#!/bin/bash
# Common operational tasks

# Daily health check
daily_health_check() {
    metanode health --detailed
    metanode consensus status
    metanode bank por-status --all-banks
    metanode security posture
    metanode analytics summary --period 24h
}

# Performance optimization
optimize_performance() {
    metanode maintenance db compact
    metanode maintenance cleanup cache
    metanode config optimize --auto
    metanode restart --graceful
}

# Security audit
security_audit() {
    metanode audit generate security --period 7d
    metanode quantum verify-setup
    metanode ai-security report --period 7d
    metanode biso violations --all-entities
}

# Backup operations
backup_operations() {
    metanode config backup --encrypted
    metanode maintenance db backup
    metanode quantum keys backup --secure
    metanode audit export --period 30d
}
```

---

## 🔧 TROUBLESHOOTING PLAYBOOK

### Common Issues & Solutions

#### Node Won't Start
```bash
# Diagnostic steps
metanode diagnose system --verbose
metanode diagnose network --connectivity
metanode logs --level error --last 100

# Common fixes
metanode config validate
metanode config reset --backup-first
metanode maintenance cleanup all
metanode recovery network bootstrap
```

#### Consensus Issues
```bash
# Diagnose consensus problems
metanode consensus status --detailed
metanode consensus peers --health-check
metanode diagnose consensus --verbose

# Fix consensus issues
metanode consensus rejoin
metanode validator unjail $(metanode config identity address)
metanode recovery network peers-reset
```

#### Banking Service Issues
```bash
# Diagnose banking problems
metanode bank list --health-check
metanode bank por-status --failures-only
metanode settle status --all-pending

# Fix banking issues
metanode bank verify --all-banks
metanode bank por run --failed-banks-only
metanode mesh liquidity rebalance
```

#### Security Alerts
```bash
# Investigate security issues
metanode ai-security alerts --severity critical
metanode biso violations --recent
metanode audit generate security --emergency

# Respond to security threats
metanode emergency safe-mode
metanode ai-security response trigger quarantine
metanode quantum keys rotate --emergency
```

### Emergency Procedures
```bash
#!/bin/bash
# Emergency response procedures

# Emergency stop
emergency_stop() {
    metanode emergency stop
    metanode ai-security response trigger lockdown
    metanode audit generate emergency --all-components
}

# Recovery procedures
emergency_recovery() {
    metanode recovery state verify
    metanode recovery network bootstrap
    metanode config restore emergency-backup.toml
    metanode start --safe-mode
}

# Incident response
incident_response() {
    metanode audit generate incident --real-time
    metanode ai-security behavior analyze --all-entities
    metanode compliance report incident --immediate
}
```

---

## 🎯 BEST PRACTICES

### Security Best Practices
```bash
# 1. Always use quantum-resistant cryptography
metanode quantum keygen --all-algorithms
metanode quantum migrate start --gradual

# 2. Enable all security features
metanode ai-security start --production-mode
metanode zk privacy enable --all-features
metanode biso policy apply production-security

# 3. Regular security audits
metanode audit generate security --weekly
metanode security posture --continuous-monitoring

# 4. Backup security keys
metanode quantum keys backup --encrypted --offsite
metanode config backup --security-keys-included
```

### Performance Best Practices
```bash
# 1. Optimize configuration
metanode config optimize --production
metanode config set consensus.timeout 3000
metanode config set mempool.max_size 10000

# 2. Regular maintenance
metanode maintenance db compact --weekly
metanode maintenance cleanup cache --daily
metanode update check --auto-install-security

# 3. Monitor performance
metanode monitor performance --continuous
metanode analytics network performance --real-time
metanode profile enable --production-safe
```

### Operational Best Practices
```bash
# 1. Comprehensive monitoring
metanode monitor enable --all-components
metanode analytics enable --all-metrics
metanode audit enable --continuous

# 2. Automated backups
metanode maintenance db backup --schedule daily
metanode config backup --schedule weekly
metanode audit export --schedule monthly

# 3. Regular testing
metanode test run --schedule nightly
metanode test capabilities --comprehensive --weekly
metanode test security --penetration --monthly
```

---

## 📚 CONCLUSION

This CLI adjunct guide provides practical implementation workflows for the complete Metanode infrastructure. Use it alongside the CLI book for:

- **Quick deployment** of production systems
- **Step-by-step workflows** for complex operations
- **Troubleshooting guidance** for common issues
- **Best practices** for security and performance
- **Operational playbooks** for day-to-day management

### Key Takeaways:
1. **Start with security** - Always implement quantum, AI, and ZK security first
2. **Monitor everything** - Enable comprehensive monitoring from day one
3. **Automate operations** - Use scripts and scheduled tasks for routine operations
4. **Test continuously** - Regular testing ensures system reliability
5. **Plan for emergencies** - Have incident response procedures ready

### Next Steps:
1. Follow the quick start guide to get your first node running
2. Implement the security workflows for production readiness
3. Set up monitoring and alerting for operational visibility
4. Practice the troubleshooting procedures in a test environment
5. Customize the workflows for your specific use case

**🚀 Ready for Production Deployment!**
