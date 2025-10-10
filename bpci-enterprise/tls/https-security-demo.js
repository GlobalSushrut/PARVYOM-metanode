// HTTPS Security Demonstration - How Browsers Show "Secure" Status
// Understanding what makes browsers display security indicators

const https = require('https');
const http = require('http');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

console.log('🔍 HTTPS Security Demonstration');
console.log('===============================');
console.log('Understanding how browsers show "Secure" vs "Not Secure"');

// Research findings about browser security indicators
function explainBrowserSecurity() {
  console.log('\n📚 Browser Security Indicators Explained:');
  console.log('==========================================');
  
  console.log('\n🟢 "Secure" (Green Lock):');
  console.log('   ✅ Valid HTTPS connection');
  console.log('   ✅ Certificate from trusted Certificate Authority');
  console.log('   ✅ Certificate matches domain name');
  console.log('   ✅ Certificate not expired');
  console.log('   ✅ Strong encryption (TLS 1.2+)');
  console.log('   ✅ No mixed content (all resources HTTPS)');
  
  console.log('\n🟡 "Not Secure" or Warning:');
  console.log('   ❌ HTTP connection (no encryption)');
  console.log('   ❌ Self-signed certificate');
  console.log('   ❌ Certificate from untrusted CA');
  console.log('   ❌ Expired certificate');
  console.log('   ❌ Domain mismatch');
  console.log('   ❌ Mixed content (HTTPS page loading HTTP)');
  
  console.log('\n🔍 Certificate Chain Validation:');
  console.log('   Browser checks: Server Cert → Intermediate CA → Root CA');
  console.log('   Root CA must be in browser\'s trust store');
}

// Show different methods to achieve "Secure" status
function showSecureMethods() {
  console.log('\n🎯 Methods to Achieve "Secure" Status:');
  console.log('======================================');
  
  console.log('\n✅ Method 1: Trusted CA Certificate (Production)');
  console.log('   • Use Let\'s Encrypt (free, automated)');
  console.log('   • Use commercial CA (DigiCert, GlobalSign, etc.)');
  console.log('   • Browser automatically trusts these');
  console.log('   • Shows green lock immediately');
  console.log('   • Best for production websites');
  
  console.log('\n✅ Method 2: Custom CA + Browser Import (Development)');
  console.log('   • Create your own Certificate Authority');
  console.log('   • Import CA certificate into browser trust store');
  console.log('   • Generate server certs signed by your CA');
  console.log('   • Browser trusts your CA and all certs it signs');
  console.log('   • Good for development/testing');
  
  console.log('\n✅ Method 3: mkcert Tool (Local Development)');
  console.log('   • Automatically creates locally-trusted certificates');
  console.log('   • Installs CA in system trust store');
  console.log('   • Perfect for localhost development');
  console.log('   • Install: npm install -g mkcert');
  
  console.log('\n❌ What Doesn\'t Work:');
  console.log('   • Self-signed certificates (always show warnings)');
  console.log('   • Ignoring certificate warnings');
  console.log('   • Using HTTP in production');
}

// Generate test certificates to demonstrate
function generateDemoCertificates() {
  console.log('\n🔧 Generating Demo Certificates:');
  console.log('================================');
  
  const certDir = path.join(__dirname, 'demo-certs');
  
  if (!fs.existsSync(certDir)) {
    fs.mkdirSync(certDir, { recursive: true });
  }
  
  try {
    console.log('📝 Creating Certificate Authority (CA)...');
    
    // Generate CA private key
    execSync(`openssl genrsa -out ${certDir}/ca-key.pem 4096`, { stdio: 'pipe' });
    
    // Generate CA certificate
    execSync(`openssl req -new -x509 -days 3650 -key ${certDir}/ca-key.pem -out ${certDir}/ca-cert.pem -subj "/C=US/ST=CA/L=Silicon Valley/O=BPCI Enterprise/OU=Security/CN=BPCI Root CA"`, { stdio: 'pipe' });
    
    console.log('✅ CA certificate created');
    
    console.log('📝 Creating server certificate...');
    
    // Generate server private key
    execSync(`openssl genrsa -out ${certDir}/server-key.pem 4096`, { stdio: 'pipe' });
    
    // Generate server certificate signing request
    execSync(`openssl req -new -key ${certDir}/server-key.pem -out ${certDir}/server.csr -subj "/C=US/ST=CA/L=Silicon Valley/O=BPCI Enterprise/OU=Security/CN=localhost"`, { stdio: 'pipe' });
    
    // Create certificate extensions
    const extFile = `${certDir}/server.ext`;
    fs.writeFileSync(extFile, `
authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
keyUsage = digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
subjectAltName = @alt_names

[alt_names]
DNS.1 = localhost
DNS.2 = *.localhost
DNS.3 = pravyom.com
DNS.4 = *.pravyom.com
IP.1 = 127.0.0.1
IP.2 = ::1
`);
    
    // Generate server certificate signed by CA
    execSync(`openssl x509 -req -in ${certDir}/server.csr -CA ${certDir}/ca-cert.pem -CAkey ${certDir}/ca-key.pem -CAcreateserial -out ${certDir}/server-cert.pem -days 365 -extensions v3_req -extfile ${certDir}/server.ext`, { stdio: 'pipe' });
    
    console.log('✅ Server certificate created');
    
    return {
      ca: {
        cert: fs.readFileSync(`${certDir}/ca-cert.pem`),
        key: fs.readFileSync(`${certDir}/ca-key.pem`)
      },
      server: {
        cert: fs.readFileSync(`${certDir}/server-cert.pem`),
        key: fs.readFileSync(`${certDir}/server-key.pem`)
      },
      paths: {
        caCert: `${certDir}/ca-cert.pem`,
        serverCert: `${certDir}/server-cert.pem`,
        serverKey: `${certDir}/server-key.pem`
      }
    };
    
  } catch (error) {
    console.error('❌ Certificate generation failed:', error.message);
    return null;
  }
}

// Create test servers to demonstrate security behavior
function createDemoServers(certificates) {
  if (!certificates) {
    console.log('❌ No certificates available for demo');
    return;
  }
  
  console.log('\n🚀 Starting Demo Servers:');
  console.log('=========================');
  
  // HTTP Server (will show "Not Secure")
  const httpApp = (req, res) => {
    res.writeHead(200, {
      'Content-Type': 'text/html',
      'Access-Control-Allow-Origin': '*'
    });
    
    res.end(`
<!DOCTYPE html>
<html>
<head>
    <title>HTTP Demo - Not Secure</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #ffe6e6; }
        .warning { color: #d32f2f; font-weight: bold; }
        .info { background: #f5f5f5; padding: 20px; border-radius: 5px; margin: 20px 0; }
    </style>
</head>
<body>
    <h1>🌐 HTTP Demo Server</h1>
    <div class="warning">⚠️ This page will show "Not Secure" in your browser</div>
    <div class="info">
        <h3>Why "Not Secure"?</h3>
        <ul>
            <li>This is an HTTP connection (no encryption)</li>
            <li>Data can be intercepted by attackers</li>
            <li>Browsers warn users about insecure connections</li>
        </ul>
    </div>
    <p>URL: <code>http://localhost:8081</code></p>
    <p>Protocol: HTTP (Insecure)</p>
    <p>Expected Browser Status: <strong>"Not Secure"</strong></p>
</body>
</html>
    `);
  };
  
  // HTTPS Server with custom certificate (will show warning)
  const httpsApp = (req, res) => {
    res.writeHead(200, {
      'Content-Type': 'text/html',
      'Access-Control-Allow-Origin': '*'
    });
    
    res.end(`
<!DOCTYPE html>
<html>
<head>
    <title>HTTPS Demo - Custom Certificate</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #fff3e0; }
        .warning { color: #f57c00; font-weight: bold; }
        .info { background: #f5f5f5; padding: 20px; border-radius: 5px; margin: 20px 0; }
        .steps { background: #e8f5e8; padding: 20px; border-radius: 5px; margin: 20px 0; }
    </style>
</head>
<body>
    <h1>🔒 HTTPS Demo Server</h1>
    <div class="warning">⚠️ This page will show a security warning initially</div>
    <div class="info">
        <h3>Why Security Warning?</h3>
        <ul>
            <li>Uses custom/self-signed certificate</li>
            <li>Certificate not from trusted CA</li>
            <li>Browser doesn't recognize our custom CA</li>
        </ul>
    </div>
    <div class="steps">
        <h3>🎯 To Make This Show "Secure":</h3>
        <ol>
            <li>Import our CA certificate: <code>${certificates.paths.caCert}</code></li>
            <li>Add it to browser's trusted root certificates</li>
            <li>Restart browser</li>
            <li>This page will then show green lock "Secure"</li>
        </ol>
    </div>
    <p>URL: <code>https://localhost:8444</code></p>
    <p>Protocol: HTTPS (Custom Certificate)</p>
    <p>Expected Browser Status: <strong>Security Warning → "Secure" after CA import</strong></p>
</body>
</html>
    `);
  };
  
  // Start HTTP server
  const httpServer = http.createServer(httpApp);
  httpServer.listen(8081, () => {
    console.log('🌐 HTTP Server: http://localhost:8081');
    console.log('   Expected: "Not Secure" in browser');
  });
  
  // Start HTTPS server
  const httpsServer = https.createServer({
    key: certificates.server.key,
    cert: certificates.server.cert,
    ca: certificates.ca.cert
  }, httpsApp);
  
  httpsServer.listen(8444, () => {
    console.log('🔒 HTTPS Server: https://localhost:8444');
    console.log('   Expected: Security warning initially');
  });
  
  return { httpServer, httpsServer };
}

// Show instructions for making certificates trusted
function showTrustInstructions(certificates) {
  if (!certificates) return;
  
  console.log('\n📋 Instructions to Make Certificates Show "Secure":');
  console.log('===================================================');
  
  console.log('\n🖥️  For Chrome/Chromium:');
  console.log('1. Go to Settings → Privacy and Security → Security');
  console.log('2. Click "Manage certificates"');
  console.log('3. Go to "Authorities" tab');
  console.log('4. Click "Import"');
  console.log(`5. Select: ${certificates.paths.caCert}`);
  console.log('6. Check "Trust this certificate for identifying websites"');
  console.log('7. Restart Chrome');
  
  console.log('\n🦊 For Firefox:');
  console.log('1. Go to Settings → Privacy & Security');
  console.log('2. Scroll to "Certificates" → Click "View Certificates"');
  console.log('3. Go to "Authorities" tab');
  console.log('4. Click "Import"');
  console.log(`5. Select: ${certificates.paths.caCert}`);
  console.log('6. Check "Trust this CA to identify websites"');
  console.log('7. Restart Firefox');
  
  console.log('\n🍎 For Safari (macOS):');
  console.log('1. Double-click the CA certificate file');
  console.log('2. Add to "System" keychain');
  console.log('3. Open Keychain Access');
  console.log('4. Find "BPCI Root CA" certificate');
  console.log('5. Double-click → Trust → "Always Trust"');
  console.log('6. Restart Safari');
  
  console.log('\n🐧 For Linux (system-wide):');
  console.log(`1. sudo cp ${certificates.paths.caCert} /usr/local/share/ca-certificates/bpci-ca.crt`);
  console.log('2. sudo update-ca-certificates');
  console.log('3. Restart browser');
}

// Main demonstration
function runSecurityDemo() {
  console.log('🎯 Starting HTTPS Security Demonstration...\n');
  
  // Explain browser security
  explainBrowserSecurity();
  showSecureMethods();
  
  // Generate demo certificates
  const certificates = generateDemoCertificates();
  
  if (certificates) {
    // Start demo servers
    const servers = createDemoServers(certificates);
    
    // Show trust instructions
    showTrustInstructions(certificates);
    
    console.log('\n🧪 Test Results:');
    console.log('================');
    console.log('1. Visit http://localhost:8081 → Will show "Not Secure"');
    console.log('2. Visit https://localhost:8444 → Will show security warning');
    console.log('3. Import CA certificate following instructions above');
    console.log('4. Visit https://localhost:8444 again → Should show "Secure"!');
    
    console.log('\n🎯 Key Takeaway:');
    console.log('For production, use Let\'s Encrypt or commercial CA');
    console.log('For development, import custom CA into browser trust store');
    
    console.log('\n⏹️  Press Ctrl+C to stop demo servers');
  }
}

// Run the demonstration
if (require.main === module) {
  runSecurityDemo();
}

module.exports = {
  explainBrowserSecurity,
  showSecureMethods,
  generateDemoCertificates,
  createDemoServers,
  showTrustInstructions
};
