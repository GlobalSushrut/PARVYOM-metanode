# 🖥️ BPCI TESTNET SERVER INFORMATION

**Date**: 2025-10-30  
**Status**: Active and Ready for Deployment

---

## 📊 SERVER DETAILS

### **Droplet Information:**
- **ID**: 527214574
- **Name**: bpci-testnet-server
- **Status**: ✅ Active

### **Network:**
- **Public IPv4**: 134.209.210.181
- **Private IPv4**: 10.116.0.4
- **Public IPv6**: 2604:a880:400:d1:0:3:1df5:b001

### **Specifications:**
- **RAM**: 16GB (16384 MB)
- **vCPUs**: 8
- **Storage**: 320GB SSD
- **Region**: NYC1 (New York)
- **OS**: Ubuntu 22.04 (LTS) x64

### **Features:**
- ✅ Monitoring enabled
- ✅ IPv6 enabled
- ✅ Private networking enabled
- ✅ Droplet agent enabled

### **Tags:**
- bpci
- testnet
- production

### **Cost:**
- **Monthly**: $112 USD
- **Hourly**: $0.167 USD

---

## 🔐 SSH ACCESS

### **Connect to Server:**
```bash
ssh root@134.209.210.181
```

### **SSH Keys Configured:**
- desktop (ID: 50065023)
- taboomesh (ID: 49615742)
- trillon_demo_key (ID: 49363343)
- demo_key (ID: 49363209)
- Umesh_SSH (ID: 49111780)

---

## 📈 UPGRADE PATH

### **Current Size:** s-8vcpu-16gb-amd

### **Available Upgrades:**

**To 32GB RAM:**
- `s-8vcpu-32gb-amd` - 32GB RAM, 8 vCPUs, 400GB - $168/month

**To 16 vCPUs:**
- `s-16vcpu-32gb-amd` - 32GB RAM, 16 vCPUs, 640GB - $224/month

**To 64GB RAM:**
- `s-16vcpu-64gb-amd` - 64GB RAM, 16 vCPUs, 1280GB - $336/month

### **How to Upgrade:**
```bash
# Power off droplet
doctl compute droplet-action power-off 527214574 --wait

# Resize (example to 32GB)
doctl compute droplet-action resize 527214574 --size s-8vcpu-32gb-amd --wait

# Power on
doctl compute droplet-action power-on 527214574 --wait
```

---

## 🎯 NEXT STEPS

### **Phase 1: Initial Setup** (30 minutes)
1. ✅ Server created
2. ⏳ SSH access verification
3. ⏳ System update
4. ⏳ Install base dependencies

### **Phase 2: Infrastructure** (2-3 hours)
5. ⏳ Install Docker & Docker Compose
6. ⏳ Install Nginx
7. ⏳ Install PostgreSQL
8. ⏳ Install Redis
9. ⏳ Install Keycloak

### **Phase 3: BPCI Backend** (4-6 hours)
10. ⏳ Build BPCI binaries
11. ⏳ Setup directory structure
12. ⏳ Configure CommuteLock
13. ⏳ Deploy Cluster Ledger
14. ⏳ Deploy all BPCI servers

### **Phase 4: Frontend** (2-3 hours)
15. ⏳ Build React application
16. ⏳ Deploy to Nginx
17. ⏳ Configure SSL/TLS

### **Phase 5: Testing** (2-3 hours)
18. ⏳ Health checks
19. ⏳ Performance testing
20. ⏳ Security validation

---

## 📝 IMPORTANT NOTES

- Server is in NYC1 region (low latency for US East Coast)
- Private networking enabled for internal communication
- Monitoring enabled for resource tracking
- Can upgrade to 32GB RAM without data loss
- IPv6 enabled for future compatibility
- All SSH keys configured for access

---

## 🚀 READY FOR DEPLOYMENT!

**Server Status**: ✅ Active  
**SSH Access**: ✅ Ready  
**Specifications**: ✅ Meets requirements  
**Upgrade Path**: ✅ Available  

**Next Command:**
```bash
ssh root@134.209.210.181
```
