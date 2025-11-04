# Manual Cloudflare DNS Setup for pravyom.com

## Current Status
✅ **API Token**: Valid and active (verified)  
❌ **Zones Access**: Still IP restricted  
🎯 **Solution**: Manual DNS setup via Cloudflare Dashboard

## Required DNS Records for BPI-BPCI Infrastructure

### Step 1: Get Your Digital Ocean Droplet IP

First, you'll need to create your Digital Ocean droplet if you haven't already:

```bash
# Create droplet (if needed)
doctl compute droplet create pravyom-production \
  --image ubuntu-22-04-x64 \
  --size s-2vcpu-4gb \
  --region nyc1 \
  --ssh-keys <YOUR_SSH_KEY_ID>

# Get the IP address
doctl compute droplet list
```

**Example IP**: `64.225.123.456` (replace with your actual IP)

### Step 2: Configure DNS Records in Cloudflare Dashboard

Go to **Cloudflare Dashboard** → **pravyom.com** → **DNS** → **Records**

#### Core Infrastructure Records

| Type | Name | Content | Proxy Status | TTL |
|------|------|---------|--------------|-----|
| A | `pravyom.com` | `<YOUR_DROPLET_IP>` | 🟠 Proxied | Auto |
| A | `www` | `<YOUR_DROPLET_IP>` | 🟠 Proxied | Auto |
| CNAME | `api` | `pravyom.com` | 🟠 Proxied | Auto |
| CNAME | `xtmp` | `pravyom.com` | 🟠 Proxied | Auto |
| CNAME | `registry` | `pravyom.com` | 🟠 Proxied | Auto |
| CNAME | `httpcg` | `pravyom.com` | 🟠 Proxied | Auto |
| CNAME | `bpci` | `pravyom.com` | 🟠 Proxied | Auto |
| CNAME | `vm` | `pravyom.com` | 🟠 Proxied | Auto |

#### Email Records (Required for @pravyom.com)

| Type | Name | Content | Proxy Status | Priority | TTL |
|------|------|---------|--------------|----------|-----|
| A | `mail` | `<YOUR_DROPLET_IP>` | ⚪ DNS only | - | Auto |
| MX | `pravyom.com` | `mail.pravyom.com` | ⚪ DNS only | 10 | Auto |
| TXT | `pravyom.com` | `v=spf1 ip4:<YOUR_DROPLET_IP> include:_spf.google.com ~all` | ⚪ DNS only | - | Auto |
| TXT | `_dmarc` | `v=DMARC1; p=quarantine; rua=mailto:dmarc@pravyom.com` | ⚪ DNS only | - | Auto |

### Step 3: Detailed Setup Instructions

#### 3.1 Add A Records

1. **Click "Add record"**
2. **Type**: A
3. **Name**: @ (for root domain) or www
4. **IPv4 address**: Your Digital Ocean droplet IP
5. **Proxy status**: 🟠 Proxied (Orange cloud)
6. **Click "Save"**

#### 3.2 Add CNAME Records

1. **Click "Add record"**
2. **Type**: CNAME
3. **Name**: api (or xtmp, registry, etc.)
4. **Target**: pravyom.com
5. **Proxy status**: 🟠 Proxied (Orange cloud)
6. **Click "Save"**

#### 3.3 Add Email Records

**MX Record:**
1. **Click "Add record"**
2. **Type**: MX
3. **Name**: @ (root domain)
4. **Mail server**: mail.pravyom.com
5. **Priority**: 10
6. **Proxy status**: ⚪ DNS only (Gray cloud)
7. **Click "Save"**

**SPF Record:**
1. **Click "Add record"**
2. **Type**: TXT
3. **Name**: @ (root domain)
4. **Content**: `v=spf1 ip4:<YOUR_DROPLET_IP> include:_spf.google.com ~all`
5. **Click "Save"**

### Step 4: SSL/TLS Configuration

1. **Go to SSL/TLS** → **Overview**
2. **Set Encryption mode**: Full (strict)
3. **Enable HSTS**: On
4. **Minimum TLS Version**: 1.2

### Step 5: Page Rules for HTTPCG Protocol

1. **Go to Rules** → **Page Rules**
2. **Add Rule 1**:
   - **URL**: `pravyom.com/*`
   - **Settings**: 
     - SSL: Full (strict)
     - Cache Level: Standard
     - Browser Cache TTL: 4 hours

3. **Add Rule 2**:
   - **URL**: `pravyom.com/httpcg/*`
   - **Settings**:
     - SSL: Full (strict)
     - Cache Level: Bypass
     - Disable Apps: On

### Step 6: Security Settings

1. **Go to Security** → **Settings**
2. **Security Level**: Medium
3. **Challenge Passage**: 30 minutes
4. **Browser Integrity Check**: On

## Expected Final Configuration

After setup, your DNS should look like this:

```
pravyom.com.        A       <YOUR_IP> (Proxied)
www.pravyom.com.    A       <YOUR_IP> (Proxied)
api.pravyom.com.    CNAME   pravyom.com (Proxied)
xtmp.pravyom.com.   CNAME   pravyom.com (Proxied)
registry.pravyom.com. CNAME pravyom.com (Proxied)
httpcg.pravyom.com. CNAME   pravyom.com (Proxied)
bpci.pravyom.com.   CNAME   pravyom.com (Proxied)
vm.pravyom.com.     CNAME   pravyom.com (Proxied)
mail.pravyom.com.   A       <YOUR_IP> (DNS only)
pravyom.com.        MX      mail.pravyom.com (Priority 10)
pravyom.com.        TXT     "v=spf1 ip4:<YOUR_IP> include:_spf.google.com ~all"
_dmarc.pravyom.com. TXT     "v=DMARC1; p=quarantine; rua=mailto:dmarc@pravyom.com"
```

## Testing Your Configuration

After setting up the DNS records, test them:

```bash
# Test main domain
dig pravyom.com
nslookup pravyom.com

# Test subdomains
dig api.pravyom.com
dig xtmp.pravyom.com
dig registry.pravyom.com

# Test email records
dig MX pravyom.com
dig TXT pravyom.com

# Test SSL
curl -I https://pravyom.com
curl -I https://api.pravyom.com
```

## Next Steps After DNS Setup

1. ✅ **DNS Records**: Configured manually in Cloudflare
2. 🚀 **Deploy BPI-BPCI**: Set up Digital Ocean droplet
3. 🔧 **Configure Nginx**: Set up web server and HTTPCG routing
4. 📱 **Deploy Vite Website**: Upload static website files
5. 🔐 **Configure SSL**: Set up origin certificates
6. 🧪 **Test Integration**: Verify HTTPCG protocol works

## Automation Script (For Later)

Once the IP restrictions are fully resolved, you can use our automated script:

```bash
# Update script with your droplet IP
sed -i 's/DIGITAL_OCEAN_IP=""/DIGITAL_OCEAN_IP="<YOUR_IP>"/' setup_cloudflare_dns.sh

# Run automated setup
./setup_cloudflare_dns.sh
```

This manual setup will give you the exact same configuration as our automated script, ensuring your BPI-BPCI infrastructure works perfectly with the Cloudflare + pravyom.com domain setup! 🚀
