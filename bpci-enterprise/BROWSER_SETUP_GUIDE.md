# 🔐 Browser Setup for Green Lock "Secure"

## 🎯 Quick Setup for Firefox

1. **Import CA Certificate:**
   - Type `about:preferences` in Firefox address bar
   - Search for "certificates"
   - Click "View Certificates..."
   - Go to "Authorities" tab
   - Click "Import..."
   - Select: `/home/umesh/metanode/bpci-enterprise/tls/certificates/ca-certificate.pem`
   - Check ✅ "Trust this CA to identify websites"
   - Click "OK"

2. **Restart Firefox**

3. **Test Results:**
   - Visit: https://localhost:8888/httpcg/dashboard
   - Should show 🟢 **Green Lock "Secure"**

## 🎯 Quick Setup for Chrome

1. **Import CA Certificate:**
   - Go to Settings → Privacy and Security → Security
   - Click "Manage certificates"
   - Go to "Authorities" tab
   - Click "Import"
   - Select: `/home/umesh/metanode/bpci-enterprise/tls/certificates/ca-certificate.pem`
   - Check ✅ "Trust this certificate for identifying websites"
   - Click "OK"

2. **Restart Chrome**

3. **Test Results:**
   - Visit: https://localhost:8888/httpcg/dashboard
   - Should show 🟢 **Green Lock "Secure"**

## ✅ Expected Results After Setup

- 🟢 Green lock "Secure" in address bar
- ✅ "Connection is secure" message
- ✅ Certificate shows "BPCI Enterprise Root CA"
- ✅ Full HTTPS functionality enabled
- ✅ No security warnings

## 🚀 Production Deployment

For production, we'll use Let's Encrypt certificates which are trusted by all browsers automatically - no manual import needed!
