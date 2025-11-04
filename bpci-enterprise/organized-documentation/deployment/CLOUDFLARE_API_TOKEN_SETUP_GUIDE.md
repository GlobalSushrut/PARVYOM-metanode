# 🔑 Cloudflare API Token Setup Guide for Advanced BPCI Integration

## 📋 Required Token Configuration

### **Token Name**
```
BPCI-Advanced-Integration-Token-v2
```

### **🎯 Required Account Permissions**
```
✅ Workers KV Storage:Edit          (CRITICAL - DynaRoutes service discovery)
✅ Workers Scripts:Edit             (CRITICAL - Gateway workers deployment)
✅ DNS Settings:Edit                (CRITICAL - QUIC endpoint management)
✅ Account Settings:Edit            (CRITICAL - Zone configuration)
✅ Cloudflare Pages:Edit           (Domain market frontend)
✅ Load Balancing: Account Load Balancers:Edit  (BPI node load balancing)
✅ Load Balancing: Monitors And Pools:Edit      (Health monitoring)
✅ Account Firewall Access Rules:Edit           (Security policies)
✅ API Gateway:Edit                (HTTPCG protocol bridging)
✅ Transform Rules:Edit            (DynaRoutes HTTP-to-QUIC transformation)
✅ Account Rulesets:Edit          (Advanced routing rules)
✅ Logs:Read                      (Monitoring and debugging)
✅ Account Analytics:Read         (Performance monitoring)
✅ Notifications:Edit             (System alerts)
✅ Account WAF:Edit               (Web3.5 security)
✅ DDoS Protection:Read           (BPCI network protection)
✅ Account: SSL and Certificates:Edit  (QUIC/TLS management)
```

### **🌐 Required Zone Permissions**
```
✅ Zone:Edit                        (CRITICAL - Zone management)
✅ Zone Settings:Edit               (CRITICAL - Zone configuration)
✅ DNS:Edit                         (CRITICAL - QUIC endpoint management)
✅ DNS Settings:Edit                (CRITICAL - DNS infrastructure)
✅ Analytics:Read                   (Performance monitoring)
✅ Load Balancers:Edit             (BPI node load balancing)
✅ Page Rules:Edit                 (Routing rules)
✅ Firewall Services:Edit          (Security policies)
✅ SSL and Certificates:Edit       (QUIC/TLS management)
✅ Transform Rules:Edit            (DynaRoutes HTTP-to-QUIC transformation)
✅ Workers Routes:Edit             (Worker routing configuration)
✅ API Gateway:Edit                (HTTPCG protocol bridging)
✅ Cache Rules:Edit                (Performance optimization)
✅ Origin Rules:Edit               (Backend routing)
✅ Custom Pages:Edit               (Domain market pages)
✅ Bot Management:Edit             (Security protection)
✅ Zone WAF:Edit                   (Web3.5 security)
✅ Health Checks:Edit              (Service monitoring)
✅ Logs:Read                       (Debugging and monitoring)
✅ Web3 Hostnames:Edit             (Web3.5 domain system)
✅ Waiting Room:Edit               (Traffic management)
✅ Response Compression:Edit       (Performance optimization)
✅ Managed Headers:Edit            (Protocol headers)
✅ Single Redirect:Edit            (Domain redirects)
✅ Config Rules:Edit               (Advanced configuration)
✅ Custom Error Rules:Edit         (Error handling)
```

### **🎯 Zone Resources**
```
Include: pravyom.com
```

### **🏢 Account Resources**
```
Include: Umeshlamton@gmail.com's Account
```

### **🔒 IP Address Filtering**
```
134.209.210.181
68.183.25.25
99.246.124.40
```

## 🚀 Step-by-Step Token Creation

1. **Go to Cloudflare Dashboard** → My Profile → API Tokens
2. **Click "Create Token"** → Custom token
3. **Set Token Name**: `BPCI-Advanced-Integration-Token-v2`
4. **Add Account Permissions** (select from list above)
5. **Add Zone Permissions** (select from list above)
6. **Set Zone Resources**: Include pravyom.com
7. **Set Account Resources**: Include your account
8. **Add IP Restrictions**: Add all three IPs (one per field)
9. **Click "Continue to summary"**
10. **Click "Create Token"**
11. **Copy the token** (save it securely)

## 📝 After Token Creation

1. **Update `.secret` file**:
   ```bash
   echo "CLOUDFLARE_API_TOKEN=your_new_token_here" > /home/umesh/metanode/.secret
   ```

2. **Test the token**:
   ```bash
   cd /home/umesh/metanode/test_cf_api && cargo run
   ```

3. **Deploy Advanced Integration**:
   ```bash
   cd /home/umesh/metanode/cloudflare-bpci-integration && cargo run
   ```

## ✅ Expected Results

With the new token, you should see:
- ✅ All 8 KV namespaces created successfully
- ✅ DNS infrastructure configured
- ✅ DynaRoutes Gateway Worker deployed
- ✅ Domain Market Worker deployed  
- ✅ BPI Proxy Worker deployed
- ✅ All 6 integration phases completed

## 🔧 Minimum Permissions (If Full List Too Complex)

If you prefer to start minimal and add permissions later:

**Account Permissions:**
```
✅ Workers KV Storage:Edit     (MUST HAVE - DynaRoutes service discovery)
✅ Workers Scripts:Edit        (MUST HAVE - Gateway workers)
✅ DNS Settings:Edit          (MUST HAVE - DNS management)
✅ Account Settings:Edit      (MUST HAVE - Zone configuration)
```

**Zone Permissions:**
```
✅ Zone:Edit                  (MUST HAVE - Zone management)
✅ Zone Settings:Edit         (MUST HAVE - Zone configuration)
✅ DNS:Edit                   (MUST HAVE - QUIC endpoints)
✅ DNS Settings:Edit          (MUST HAVE - DNS infrastructure)
✅ Workers Routes:Edit        (MUST HAVE - Worker routing)
✅ SSL and Certificates:Edit  (MUST HAVE - QUIC/TLS)
```

This minimal set will allow basic functionality, and you can add more permissions as needed for advanced features.
