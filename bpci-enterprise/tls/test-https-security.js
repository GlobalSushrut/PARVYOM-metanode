// HTTPS Security Test - Understanding Browser "Secure" Status
// Test different certificate configurations to see browser behavior

const https = require('https');
const http = require('http');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

console.log('🔍 HTTPS Security Test - Understanding Browser "Secure" Status');
console.log('===========================================================');

// Test 1: Check what makes browsers show "Secure"
console.log('\n📋 Browser Security Indicators Research:');
console.log('1. "Secure" (Green Lock) = Valid HTTPS + Trusted Certificate');
console.log('2. "Not Secure" = HTTP or Invalid/Self-signed Certificate');
console.log('3. Warning Triangle = Mixed Content or Certificate Issues');

// Test 2: Generate a simple self-signed certificate for testing
function generateTestCertificate() {
  console.log('\n🔧 Generating test certificate...');
  
  const certDir = path.join(__dirname, 'test-certs');
  
  // Create test certificate directory
  if (!fs.existsSync(certDir)) {
    fs.mkdirSync(certDir, { recursive: true });
  }
  
  try {
    // Generate private key
    execSync(`openssl genrsa -out ${certDir}/test-key.pem 2048`, { stdio: 'inherit' });
    
    // Generate self-signed certificate
    execSync(`openssl req -new -x509 -key ${certDir}/test-key.pem -out ${certDir}/test-cert.pem -days 365 -subj "/C=US/ST=CA/L=Test/O=BPCI Test/CN=localhost"`, { stdio: 'inherit' });
    
    console.log('✅ Test certificate generated');
    return {
      key: fs.readFileSync(path.join(certDir, 'test-key.pem')),
      cert: fs.readFileSync(path.join(certDir, 'test-cert.pem'))
    };
  } catch (error) {
    console.error('❌ Failed to generate test certificate:', error.message);
    return null;
  }
}

// Test 3: Create HTTPS server with self-signed certificate
function createTestHTTPSServer(credentials) {
  if (!credentials) {
    console.log('❌ No credentials available for HTTPS test');
    return;
  }
  
  const app = (req, res) => {
    res.writeHead(200, {
      'Content-Type': 'application/json',
      'Access-Control-Allow-Origin': '*'
    });
    
    const securityInfo = {
      message: 'HTTPS Security Test',
      protocol: 'https',
      secure: req.connection.encrypted || false,
      certificate_type: 'Self-signed (Test)',
      expected_browser_status: 'Warning or "Not Secure"',
      reason: 'Self-signed certificates are not trusted by browsers by default',
      how_to_fix: [
        '1. Use certificate from trusted CA (Let\'s Encrypt, etc.)',
        '2. Import custom CA certificate into browser trust store',
        '3. Add security exception in browser (not recommended for production)'
      ],
      test_url: 'https://localhost:8443',
      timestamp: new Date().toISOString()
    };
    
    res.end(JSON.stringify(securityInfo, null, 2));
  };
  
  const server = https.createServer(credentials, app);
  
  server.listen(8443, () => {
    console.log('\n🚀 HTTPS Test Server running on https://localhost:8443');
    console.log('📊 Expected Browser Behavior:');
    console.log('   • Chrome: "Not secure" or warning triangle');
    console.log('   • Firefox: Warning page or "Connection is not secure"');
    console.log('   • Safari: "This connection is not private"');
    console.log('\n🔍 Test this URL: https://localhost:8443');
    console.log('⚠️  You will see security warnings - this is expected!');
  });
  
  return server;
}

// Test 4: Create HTTP server for comparison
function createTestHTTPServer() {
  const app = (req, res) => {
    res.writeHead(200, {
      'Content-Type': 'application/json',
      'Access-Control-Allow-Origin': '*'
    });
    
    const securityInfo = {
      message: 'HTTP Security Test',
      protocol: 'http',
      secure: false,
      certificate_type: 'None (HTTP)',
      expected_browser_status: '"Not secure" in address bar',
      reason: 'HTTP connections are not encrypted',
      test_url: 'http://localhost:8080',
      timestamp: new Date().toISOString()
    };
    
    res.end(JSON.stringify(securityInfo, null, 2));
  };
  
  const server = http.createServer(app);
  
  server.listen(8080, () => {
    console.log('\n🌐 HTTP Test Server running on http://localhost:8080');
    console.log('📊 Expected Browser Behavior:');
    console.log('   • All browsers: "Not secure" in address bar');
    console.log('\n🔍 Test this URL: http://localhost:8080');
  });
  
  return server;
}

// Test 5: Research how to make browsers show "Secure"
function explainSecureBrowserStatus() {
  console.log('\n📚 How to Make Browsers Show "Secure":');
  console.log('=====================================');
  
  console.log('\n✅ Method 1: Use Trusted CA Certificate');
  console.log('   • Get certificate from Let\'s Encrypt, DigiCert, etc.');
  console.log('   • Browser automatically trusts these certificates');
  console.log('   • Shows green lock immediately');
  
  console.log('\n✅ Method 2: Import Custom CA (For Development)');
  console.log('   • Generate your own Certificate Authority');
  console.log('   • Import CA certificate into browser trust store');
  console.log('   • Generate server certificates signed by your CA');
  console.log('   • Browser will trust certificates signed by your CA');
  
  console.log('\n✅ Method 3: Use mkcert (Development Tool)');
  console.log('   • Tool that creates locally-trusted certificates');
  console.log('   • Automatically installs CA in system trust store');
  console.log('   • Perfect for local development');
  
  console.log('\n❌ What Doesn\'t Work:');
  console.log('   • Self-signed certificates (always show warnings)');
  console.log('   • Expired certificates');
  console.log('   • Wrong domain name in certificate');
  console.log('   • Mixed content (HTTPS page loading HTTP resources)');
}

// Test 6: Show certificate validation process
function showCertificateValidation() {
  console.log('\n🔍 Certificate Validation Process:');
  console.log('==================================');
  
  console.log('\n1. Browser checks certificate chain:');
  console.log('   Server Cert → Intermediate CA → Root CA');
  
  console.log('\n2. Browser validates:');
  console.log('   • Certificate not expired');
  console.log('   • Domain name matches certificate');
  console.log('   • Certificate signed by trusted CA');
  console.log('   • Certificate not revoked');
  
  console.log('\n3. If all checks pass:');
  console.log('   • Shows green lock "Secure"');
  console.log('   • Enables HTTPS features');
  
  console.log('\n4. If any check fails:');
  console.log('   • Shows warning or "Not secure"');
  console.log('   • May block connection entirely');
}

// Main test function
function runSecurityTests() {
  console.log('\n🧪 Starting HTTPS Security Tests...');
  
  // Explain how browser security works
  explainSecureBrowserStatus();
  showCertificateValidation();
  
  // Generate test certificate
  const credentials = generateTestCertificate();
  
  // Start test servers
  createTestHTTPServer();
  createTestHTTPSServer(credentials);
  
  console.log('\n📋 Test Summary:');
  console.log('================');
  console.log('1. Visit http://localhost:8080 - Will show "Not secure"');
  console.log('2. Visit https://localhost:8443 - Will show security warning');
  console.log('3. Compare the browser behavior');
  
  console.log('\n🎯 Next Steps for Production:');
  console.log('1. Choose Method 1 (Trusted CA) for production deployment');
  console.log('2. Use Method 2 (Custom CA) for development/testing');
  console.log('3. Ensure all resources load over HTTPS');
  console.log('4. Test in multiple browsers');
  
  console.log('\n⏹️  Press Ctrl+C to stop test servers');
}

// Run the tests
if (require.main === module) {
  runSecurityTests();
}

module.exports = {
  generateTestCertificate,
  createTestHTTPSServer,
  createTestHTTPServer,
  explainSecureBrowserStatus,
  showCertificateValidation
};
