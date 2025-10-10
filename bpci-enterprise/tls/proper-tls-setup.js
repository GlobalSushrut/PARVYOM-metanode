// Proper TLS Setup for BPCI Enterprise - Browser "Secure" Status
// Understanding and implementing certificates that show "Secure"

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

console.log('🔐 BPCI Enterprise TLS Setup - Browser "Secure" Status');
console.log('======================================================');

// Explain how browsers determine "Secure" status
function explainSecureStatus() {
  console.log('\n📚 How Browsers Show "Secure" Status:');
  console.log('====================================');
  
  console.log('\n🟢 GREEN LOCK "Secure":');
  console.log('   1. Valid HTTPS connection (TLS 1.2+)');
  console.log('   2. Certificate from TRUSTED Certificate Authority');
  console.log('   3. Certificate matches the domain name');
  console.log('   4. Certificate is not expired');
  console.log('   5. Complete certificate chain validation');
  console.log('   6. No mixed content (all resources HTTPS)');
  
  console.log('\n🔴 "Not Secure" or Warning:');
  console.log('   ❌ HTTP connection (no encryption)');
  console.log('   ❌ Self-signed certificate');
  console.log('   ❌ Certificate from untrusted CA');
  console.log('   ❌ Domain name mismatch');
  console.log('   ❌ Expired certificate');
  console.log('   ❌ Broken certificate chain');
  
  console.log('\n🎯 KEY INSIGHT:');
  console.log('   The certificate must be from a CA that is in the browser\'s');
  console.log('   built-in trust store OR manually imported by the user.');
}

// Show the three methods to achieve "Secure" status
function showSecureMethods() {
  console.log('\n🛠️  Three Methods to Achieve "Secure" Status:');
  console.log('=============================================');
  
  console.log('\n✅ METHOD 1: Let\'s Encrypt (Recommended for Production)');
  console.log('   • Free, automated certificates');
  console.log('   • Trusted by all browsers automatically');
  console.log('   • 90-day validity with auto-renewal');
  console.log('   • Perfect for production websites');
  console.log('   • Command: certbot --nginx -d pravyom.com');
  
  console.log('\n✅ METHOD 2: Custom CA + Manual Import (Development)');
  console.log('   • Create your own Certificate Authority');
  console.log('   • Import CA into browser trust store');
  console.log('   • Generate server certificates signed by your CA');
  console.log('   • Good for development and testing');
  
  console.log('\n✅ METHOD 3: mkcert Tool (Local Development)');
  console.log('   • Automatically creates locally-trusted certificates');
  console.log('   • Handles CA import automatically');
  console.log('   • Perfect for localhost development');
  console.log('   • Install: npm install -g mkcert');
}

// Generate proper certificates using mkcert approach
function generateProperCertificates() {
  console.log('\n🔧 Generating Proper TLS Certificates:');
  console.log('======================================');
  
  const certDir = path.join(__dirname, 'certificates');
  
  if (!fs.existsSync(certDir)) {
    fs.mkdirSync(certDir, { recursive: true });
  }
  
  try {
    console.log('📝 Step 1: Creating Certificate Authority...');
    
    // Generate CA private key
    execSync(`openssl genrsa -out ${certDir}/ca-private-key.pem 4096`, { stdio: 'pipe' });
    
    // Generate CA certificate
    execSync(`openssl req -new -x509 -days 3650 -key ${certDir}/ca-private-key.pem -out ${certDir}/ca-certificate.pem -subj "/C=US/ST=California/L=Silicon Valley/O=BPCI Enterprise/OU=Security Department/CN=BPCI Enterprise Root CA"`, { stdio: 'pipe' });
    
    console.log('✅ Certificate Authority created');
    
    console.log('📝 Step 2: Creating server certificates...');
    
    const domains = ['localhost', 'pravyom.com', 'www.pravyom.com', 'admin.pravyom.com', 'api.pravyom.com'];
    
    for (const domain of domains) {
      console.log(`   Creating certificate for: ${domain}`);
      
      const domainDir = path.join(certDir, domain);
      if (!fs.existsSync(domainDir)) {
        fs.mkdirSync(domainDir, { recursive: true });
      }
      
      // Generate private key
      execSync(`openssl genrsa -out ${domainDir}/private-key.pem 4096`, { stdio: 'pipe' });
      
      // Generate certificate signing request
      execSync(`openssl req -new -key ${domainDir}/private-key.pem -out ${domainDir}/certificate.csr -subj "/C=US/ST=California/L=Silicon Valley/O=BPCI Enterprise/OU=Security Department/CN=${domain}"`, { stdio: 'pipe' });
      
      // Create extensions file
      const extContent = `
authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
keyUsage = digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
subjectAltName = @alt_names

[alt_names]
DNS.1 = ${domain}
DNS.2 = *.${domain}
IP.1 = 127.0.0.1
IP.2 = ::1
`;
      fs.writeFileSync(`${domainDir}/certificate.ext`, extContent);
      
      // Generate certificate signed by CA
      execSync(`openssl x509 -req -in ${domainDir}/certificate.csr -CA ${certDir}/ca-certificate.pem -CAkey ${certDir}/ca-private-key.pem -CAcreateserial -out ${domainDir}/certificate.pem -days 365 -extfile ${domainDir}/certificate.ext`, { stdio: 'pipe' });
      
      // Create certificate chain
      const serverCert = fs.readFileSync(`${domainDir}/certificate.pem`);
      const caCert = fs.readFileSync(`${certDir}/ca-certificate.pem`);
      fs.writeFileSync(`${domainDir}/certificate-chain.pem`, serverCert + caCert);
    }
    
    console.log('✅ Server certificates created for all domains');
    
    // Generate DH parameters
    console.log('📝 Step 3: Generating Diffie-Hellman parameters...');
    execSync(`openssl dhparam -out ${certDir}/dhparam.pem 2048`, { stdio: 'pipe' });
    console.log('✅ DH parameters generated');
    
    return {
      certDir: certDir,
      caCert: `${certDir}/ca-certificate.pem`,
      domains: domains
    };
    
  } catch (error) {
    console.error('❌ Certificate generation failed:', error.message);
    return null;
  }
}

// Show how to make browsers trust our certificates
function showBrowserTrustInstructions(certInfo) {
  if (!certInfo) return;
  
  console.log('\n📋 Making Browsers Show "Secure":');
  console.log('=================================');
  
  console.log('\n🎯 CRITICAL STEP: Import CA Certificate');
  console.log(`   CA Certificate Location: ${certInfo.caCert}`);
  
  console.log('\n🖥️  Chrome/Edge Instructions:');
  console.log('   1. Open Chrome Settings');
  console.log('   2. Go to Privacy and Security → Security');
  console.log('   3. Click "Manage certificates"');
  console.log('   4. Go to "Authorities" tab');
  console.log('   5. Click "Import"');
  console.log(`   6. Select: ${certInfo.caCert}`);
  console.log('   7. Check "Trust this certificate for identifying websites"');
  console.log('   8. Click OK and restart Chrome');
  
  console.log('\n🦊 Firefox Instructions:');
  console.log('   1. Open Firefox Settings');
  console.log('   2. Go to Privacy & Security');
  console.log('   3. Scroll to Certificates → "View Certificates"');
  console.log('   4. Go to "Authorities" tab');
  console.log('   5. Click "Import"');
  console.log(`   6. Select: ${certInfo.caCert}`);
  console.log('   7. Check "Trust this CA to identify websites"');
  console.log('   8. Click OK and restart Firefox');
  
  console.log('\n🍎 Safari (macOS) Instructions:');
  console.log('   1. Double-click the CA certificate file');
  console.log('   2. Choose "System" keychain');
  console.log('   3. Enter admin password');
  console.log('   4. Open Keychain Access app');
  console.log('   5. Find "BPCI Enterprise Root CA"');
  console.log('   6. Double-click → Expand "Trust"');
  console.log('   7. Set "When using this certificate" to "Always Trust"');
  console.log('   8. Restart Safari');
}

// Create a test HTTPS server to verify "Secure" status
function createTestServer(certInfo) {
  if (!certInfo) return;
  
  console.log('\n🚀 Creating Test HTTPS Server:');
  console.log('==============================');
  
  const https = require('https');
  const domain = 'localhost';
  const domainDir = path.join(certInfo.certDir, domain);
  
  try {
    const credentials = {
      key: fs.readFileSync(`${domainDir}/private-key.pem`),
      cert: fs.readFileSync(`${domainDir}/certificate-chain.pem`)
    };
    
    const app = (req, res) => {
      res.writeHead(200, {
        'Content-Type': 'text/html',
        'Strict-Transport-Security': 'max-age=31536000; includeSubDomains',
        'X-Content-Type-Options': 'nosniff',
        'X-Frame-Options': 'DENY'
      });
      
      res.end(`
<!DOCTYPE html>
<html>
<head>
    <title>BPCI Enterprise - TLS Test</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #e8f5e8; }
        .secure { color: #2e7d32; font-weight: bold; font-size: 24px; }
        .info { background: white; padding: 20px; border-radius: 8px; margin: 20px 0; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        .step { background: #f3e5f5; padding: 15px; border-radius: 5px; margin: 10px 0; }
    </style>
</head>
<body>
    <h1>🔐 BPCI Enterprise TLS Test</h1>
    <div class="secure">✅ If you see a green lock, TLS is working!</div>
    
    <div class="info">
        <h3>🎯 Current Status Check:</h3>
        <p><strong>URL:</strong> https://localhost:8443</p>
        <p><strong>Certificate:</strong> BPCI Enterprise Custom CA</p>
        <p><strong>Expected Result:</strong> Green lock "Secure" (after CA import)</p>
    </div>
    
    <div class="step">
        <h4>🔍 If you see a security warning:</h4>
        <ol>
            <li>The CA certificate is not yet imported</li>
            <li>Follow the import instructions shown in the terminal</li>
            <li>Restart your browser</li>
            <li>Reload this page</li>
        </ol>
    </div>
    
    <div class="step">
        <h4>✅ If you see a green lock:</h4>
        <p>Congratulations! Your browser now trusts our custom certificates and shows "Secure" status.</p>
    </div>
    
    <div class="info">
        <h3>🚀 For Production Deployment:</h3>
        <ul>
            <li>Use Let's Encrypt for automatic trusted certificates</li>
            <li>Or use commercial CA certificates</li>
            <li>Custom CA is perfect for development/testing</li>
        </ul>
    </div>
</body>
</html>
      `);
    };
    
    const server = https.createServer(credentials, app);
    
    server.listen(8443, () => {
      console.log('🌐 Test server running: https://localhost:8443');
      console.log('📊 Expected behavior:');
      console.log('   • Before CA import: Security warning');
      console.log('   • After CA import: Green lock "Secure"');
    });
    
    return server;
    
  } catch (error) {
    console.error('❌ Failed to create test server:', error.message);
    return null;
  }
}

// Main function to set up proper TLS
function setupProperTLS() {
  console.log('🎯 Setting up proper TLS for "Secure" browser status...\n');
  
  // Explain how it works
  explainSecureStatus();
  showSecureMethods();
  
  // Generate certificates
  const certInfo = generateProperCertificates();
  
  if (certInfo) {
    // Show trust instructions
    showBrowserTrustInstructions(certInfo);
    
    // Create test server
    const server = createTestServer(certInfo);
    
    console.log('\n🎯 Summary:');
    console.log('===========');
    console.log('1. Certificates generated successfully');
    console.log('2. Import CA certificate following instructions above');
    console.log('3. Visit https://localhost:8443 to test');
    console.log('4. Should show green lock "Secure" after CA import');
    
    console.log('\n💡 Key Takeaway:');
    console.log('   For browsers to show "Secure", the certificate must be');
    console.log('   from a trusted CA or manually imported into trust store.');
    
    console.log('\n⏹️  Press Ctrl+C to stop test server');
  }
}

// Run the setup
if (require.main === module) {
  setupProperTLS();
}

module.exports = {
  explainSecureStatus,
  showSecureMethods,
  generateProperCertificates,
  showBrowserTrustInstructions,
  createTestServer
};
