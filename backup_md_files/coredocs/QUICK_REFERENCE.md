# 🚀 DockLock + ENC Cluster - Quick Reference

## Essential Commands

### System Management
```bash
# Start system
metanode start --port 8080

# Check status
metanode status

# System health
curl http://localhost:8080/status
```

### Application Deployment
```bash
# Basic deployment
metanode deploy <app-name> --image <image> --replicas <count>

# Enterprise deployment
metanode enterprise create-cluster --name <cluster> --nodes <count>
metanode deploy <app> --cluster <cluster> --compliance SOC2,HIPAA
```

### Receipt & Audit Verification
```bash
# View receipts
metanode receipts <deployment-id>

# Check transactions
metanode ledger query <receipt-id>

# Verify proofs
metanode proofs verify <receipt-id>

# Compliance audit
metanode enterprise audit --framework SOC2
```

### Monitoring
```bash
# System status
metanode status

# Enterprise status  
metanode enterprise status

# Mining status
metanode mining status

# Ledger statistics
metanode ledger stats
```

## Key Differences from Docker + K8s

| Feature | Docker + K8s | DockLock + ENC Cluster |
|---------|-------------|------------------------|
| **Auditability** | Logs only | Cryptographic receipts |
| **Security** | Basic isolation | Military-grade + syscall filtering |
| **Compliance** | Manual | Built-in SOC2/HIPAA/PCI-DSS |
| **Blockchain** | None | Real transactions & blocks |
| **Optimization** | Manual | AI-driven resource allocation |
| **Trust Model** | Trust-based | Zero-trust + continuous verification |

## Revolutionary Features

### 🔐 **4-Tier Receipt System**
1. **Action Receipts** - Every container operation
2. **Agreement Receipts** - Policy compliance  
3. **Pipeline Receipts** - Traffic control & dual approval
4. **Economic Receipts** - Resource usage & billing

### ⛓️ **Real Blockchain Integration**
- Every operation creates real transactions
- Cryptographic proofs (POA, POE, POT, POG)
- Immutable audit trails
- Mathematical rigor (category theory + knot theory)

### 🛡️ **Enterprise Security**
- Deterministic execution environment
- Syscall filtering with seccomp
- Zero-trust networking
- Continuous security verification

### 📊 **AI-Driven Features**
- Intelligent resource allocation
- Predictive scaling
- Anomaly detection  
- Performance optimization

## Common Use Cases

### **Financial Services**
```bash
metanode enterprise create-cluster \
  --name financial \
  --security-level maximum \
  --compliance SOC2,PCI-DSS \
  --zero-trust
```

### **Healthcare**
```bash
metanode enterprise create-cluster \
  --name healthcare \
  --compliance HIPAA \
  --encryption-at-rest \
  --audit-logging
```

### **E-Commerce**
```bash
metanode deploy ecommerce-stack \
  --cluster production \
  --compliance PCI-DSS \
  --auto-scale \
  --load-balancer
```

### **AI/ML Workloads**
```bash
metanode deploy ml-training \
  --cluster ai-training \
  --gpu 2 \
  --memory 32GB \
  --checkpoint-enabled
```

## Troubleshooting Quick Fixes

### Connection Issues
```bash
# Check BPCI server
curl -s http://localhost:8080/status

# Restart if needed
pkill bpci-server
./target/release/bpci-server --port 8080
```

### Receipt Issues
```bash
# Check receipt system
metanode receipts --system-status

# Verify mathematical foundation
metanode proofs --system-check
```

### Deployment Issues
```bash
# Check logs
metanode enterprise logs <deployment-id>

# Verify resources
metanode enterprise status
```

## Best Practices

### ✅ **Do**
- Use enterprise clusters for production
- Enable compliance frameworks early
- Monitor receipt generation continuously
- Run regular security audits
- Use AI optimization for resource allocation

### ❌ **Don't**
- Deploy sensitive workloads without compliance
- Ignore receipt verification failures
- Skip security audits
- Use basic clusters for production
- Disable audit logging

---

*For detailed information, see [DOCKLOCK_ENC_CLUSTER_GUIDE.md](./DOCKLOCK_ENC_CLUSTER_GUIDE.md)*
