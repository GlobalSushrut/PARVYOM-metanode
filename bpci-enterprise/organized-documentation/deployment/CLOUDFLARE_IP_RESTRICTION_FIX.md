# Cloudflare API Token IP Restriction Fix

## Issue Identified

Your Cloudflare API token is working correctly, but it has **IP restrictions** enabled that are blocking access from your current location:

```
Error: "Cannot use the access token from location: 2607:fea8:4c81:cb00:b950:b3de:1d28:2f50"
```

## Solution Options

### Option 1: Remove IP Restrictions (Recommended)

1. **Go to Cloudflare Dashboard**
2. **Navigate to**: My Profile → API Tokens
3. **Find your token**: `Nv8upAIz4t5idjnNcLW9BPG_2q030dFrq47T6nfk`
4. **Click "Edit"**
5. **Under "IP Address Filtering"**: Remove all IP restrictions or add your current IP
6. **Save the token**

### Option 2: Add Your Current IP to Allowed List

Your current IPv6 address: `2607:fea8:4c81:cb00:b950:b3de:1d28:2f50`

1. **Go to Cloudflare Dashboard**
2. **Navigate to**: My Profile → API Tokens
3. **Edit your token**
4. **Add IP**: `2607:fea8:4c81:cb00:b950:b3de:1d28:2f50/128`
5. **Save the token**

### Option 3: Create New Token Without IP Restrictions

1. **Go to Cloudflare Dashboard**
2. **Navigate to**: My Profile → API Tokens
3. **Click "Create Token"**
4. **Use "Custom token" template**
5. **Permissions**:
   - Zone:Zone:Read
   - Zone:DNS:Edit
   - Zone:Zone Settings:Read
6. **Zone Resources**: Include → Specific zone → pravyom.com
7. **IP Address Filtering**: Leave empty (no restrictions)
8. **Create token**

## Testing After Fix

Once you've removed the IP restrictions or added your IP, test the token:

```bash
# Test the updated token
curl -s -X GET "https://api.cloudflare.com/client/v4/zones" \
  -H "Authorization: Bearer Nv8upAIz4t5idjnNcLW9BPG_2q030dFrq47T6nfk" \
  -H "Content-Type: application/json" | jq .
```

## Required Token Permissions

For our DNS setup script to work, the token needs these permissions:

- **Zone:Zone:Read** - To list and find the pravyom.com zone
- **Zone:DNS:Edit** - To create, update, and delete DNS records
- **Zone:Zone Settings:Read** - To read zone configuration

## Next Steps After Fixing IP Restrictions

1. **Test the token** with the curl command above
2. **Run our DNS setup script**: `./setup_cloudflare_dns.sh`
3. **Create all required DNS records** for BPI-BPCI infrastructure
4. **Deploy to Digital Ocean** with proper domain configuration

## Alternative: Manual DNS Setup

If you prefer to set up DNS records manually through the Cloudflare dashboard, here are the required records:

### Core Infrastructure Records
```
Type: A
Name: pravyom.com
Content: <YOUR_DIGITAL_OCEAN_IP>
Proxy: Enabled (Orange Cloud)

Type: A  
Name: www
Content: <YOUR_DIGITAL_OCEAN_IP>
Proxy: Enabled (Orange Cloud)

Type: CNAME
Name: api
Content: pravyom.com
Proxy: Enabled (Orange Cloud)

Type: CNAME
Name: xtmp
Content: pravyom.com
Proxy: Enabled (Orange Cloud)

Type: CNAME
Name: registry
Content: pravyom.com
Proxy: Enabled (Orange Cloud)
```

### Email Records
```
Type: A
Name: mail
Content: <YOUR_DIGITAL_OCEAN_IP>
Proxy: Disabled (Gray Cloud)

Type: MX
Name: pravyom.com
Content: mail.pravyom.com
Priority: 10
Proxy: Disabled (Gray Cloud)

Type: TXT
Name: pravyom.com
Content: "v=spf1 ip4:<YOUR_DIGITAL_OCEAN_IP> include:_spf.google.com ~all"

Type: TXT
Name: _dmarc
Content: "v=DMARC1; p=quarantine; rua=mailto:dmarc@pravyom.com"
```

The IP restriction issue is easily fixable through the Cloudflare dashboard. Once resolved, our automated DNS setup script will work perfectly! 🚀
