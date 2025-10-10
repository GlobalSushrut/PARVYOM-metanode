# 🏗️ BPCI Enterprise - Deployment Architecture

## 📋 **Deployment Split Strategy**

### **1. Website (Vercel - pravyom.com)**
**Purpose**: Marketing, SEO, signup/login UI only

#### **✅ What Goes on Vercel:**
- **Marketing Pages**: Home, About, Technology, Enterprise, Community, Blog, GetStarted
- **Auth UI**: Login/signup forms (UI only, no wallet creation)
- **SEO Layer**: Next.js for server-side rendering and meta tags
- **Static Assets**: Images, CSS, marketing content
- **Demo Components**: Non-functional demos for marketing

#### **❌ What Does NOT Go on Vercel:**
- **Dashboard**: Real dashboard functionality
- **Wallet Management**: Wallet creation, transactions, balances
- **Registry**: Domain registration functionality
- **BPI Installer**: Real installation logic
- **Network Status**: Real network monitoring
- **Profile Management**: User profiles, settings, data

---

### **2. HTTPCG Protocol Instances (Separate Cloud Instances)**

#### **Admin Dashboard (admin.pravyom.com:8888)**
- **Real Dashboard**: Complete admin functionality
- **System Monitoring**: Live network status
- **User Management**: Profile creation, settings
- **Registry Management**: Domain registration, approval
- **HTTPCG Protocol**: Native HTTPCG headers and addressing

#### **Wallet Server (api.pravyom.com:7778)**
- **Wallet Creation**: Real wallet generation
- **Transaction Management**: Send, receive, history
- **Balance Tracking**: Real balance queries
- **Staking**: Staking functionality
- **HTTPCG Protocol**: Native HTTPCG wallet operations

#### **BPCI Server (pravyom.com:9443)**
- **Core Backend**: Rust BPCI server
- **HTTPCG Protocol**: Core protocol implementation
- **Network Coordination**: Node management
- **Security**: Authentication, authorization

---

## 🔧 **Integration Points**

### **Website → HTTPCG Services**
```javascript
// Website (Vercel) redirects to HTTPCG instances
const handleLogin = async (credentials) => {
  // Authenticate on website
  const auth = await fetch('/api/auth/login', { ... });
  
  if (auth.success) {
    // Redirect to HTTPCG dashboard
    window.location.href = 'https://admin.pravyom.com:8888/httpcg/dashboard';
  }
};

const handleWalletAccess = () => {
  // Direct redirect to HTTPCG wallet
  window.location.href = 'https://api.pravyom.com:7778/httpcg/wallet';
};
```

### **HTTPCG Services → Backend**
```javascript
// HTTPCG instances communicate with Rust backend
const bpciClient = {
  baseURL: 'https://pravyom.com:9443',
  headers: {
    'X-HTTPCG-Protocol': 'Enabled',
    'X-BPCI-Version': '1.0.0'
  }
};
```

---

## 🚀 **Deployment Steps**

### **Step 1: Prepare Website for Vercel**
1. **Remove Dashboard/Wallet Logic** from Vite app
2. **Keep Marketing Pages** and auth UI only
3. **Add Redirects** to HTTPCG instances
4. **Configure Next.js** for SEO and SSR
5. **Build and Deploy** to Vercel with pravyom.com domain

### **Step 2: Deploy HTTPCG Instances**
1. **Admin Dashboard**: Deploy to cloud instance with HTTPCG protocol
2. **Wallet Server**: Deploy to cloud instance with HTTPCG protocol  
3. **BPCI Server**: Deploy Rust backend to cloud instance

### **Step 3: Configure Integration**
1. **DNS Records**: Point subdomains to instances
2. **TLS Certificates**: Let's Encrypt for all instances
3. **HTTPCG Headers**: Ensure protocol compliance
4. **Cross-Origin**: Configure CORS for integration

---

## 🎯 **Expected Results**

### **Website (pravyom.com)**
- ✅ **Fast Loading**: Static site on Vercel CDN
- ✅ **SEO Optimized**: Server-side rendering with Next.js
- ✅ **Marketing Focus**: Clean, professional presentation
- ✅ **Auth Gateway**: Secure login/signup with redirect to HTTPCG

### **HTTPCG Services**
- ✅ **Full Functionality**: Complete dashboard and wallet features
- ✅ **Protocol Compliance**: Native HTTPCG addressing and headers
- ✅ **Security**: Military-grade encryption and authentication
- ✅ **Performance**: Direct backend integration without proxy overhead

### **User Experience**
1. **Visit pravyom.com**: Marketing site, learn about BPCI
2. **Sign up/Login**: Create account on website
3. **Access Dashboard**: Redirect to admin.pravyom.com:8888 (HTTPCG)
4. **Use Wallet**: Redirect to api.pravyom.com:7778 (HTTPCG)
5. **Full Features**: Complete BPCI functionality via HTTPCG protocol

---

## 🔐 **Security & Protocol**

### **Website Security**
- ✅ **HTTPS**: Vercel automatic SSL
- ✅ **CSP Headers**: Content Security Policy
- ✅ **Auth Tokens**: JWT for session management

### **HTTPCG Security**
- ✅ **TLS Certificates**: Let's Encrypt or custom CA
- ✅ **HTTPCG Protocol**: Native protocol headers
- ✅ **Quantum Security**: Post-quantum cryptography
- ✅ **Military Grade**: Advanced encryption standards

This architecture ensures clean separation of concerns, optimal performance, and proper protocol compliance while maintaining seamless user experience.
