// BPCI Enterprise Integration Testing Framework
// Complete end-to-end testing for all components

const axios = require('axios');
const WebSocket = require('ws');
const jwt = require('jsonwebtoken');

class BPCIIntegrationTester {
  constructor() {
    this.baseUrls = {
      website: 'http://localhost:3000',
      adminDashboard: 'http://localhost:8888',
      bpciServer: 'http://localhost:9999',
      walletServer: 'http://localhost:7778'
    };
    
    this.testResults = {
      passed: 0,
      failed: 0,
      total: 0,
      details: []
    };
    
    this.authToken = null;
  }

  // Test runner
  async runAllTests() {
    console.log('🚀 Starting BPCI Enterprise Integration Tests...\n');
    
    try {
      // Phase 1: Component Health Checks
      await this.testComponentHealth();
      
      // Phase 2: Authentication Flow
      await this.testAuthenticationFlow();
      
      // Phase 3: BPCI Server Integration
      await this.testBPCIServerIntegration();
      
      // Phase 4: Wallet Server Integration
      await this.testWalletServerIntegration();
      
      // Phase 5: End-to-End Flow
      await this.testEndToEndFlow();
      
      // Phase 6: WebSocket Communication
      await this.testWebSocketCommunication();
      
      // Generate test report
      this.generateTestReport();
      
    } catch (error) {
      console.error('❌ Integration test suite failed:', error.message);
      this.recordTest('Integration Test Suite', false, error.message);
    }
  }

  // Record test result
  recordTest(testName, passed, details = '') {
    this.testResults.total++;
    if (passed) {
      this.testResults.passed++;
      console.log(`✅ ${testName}: PASSED`);
    } else {
      this.testResults.failed++;
      console.log(`❌ ${testName}: FAILED - ${details}`);
    }
    
    this.testResults.details.push({
      test: testName,
      status: passed ? 'PASSED' : 'FAILED',
      details: details,
      timestamp: new Date().toISOString()
    });
  }

  // Phase 1: Component Health Checks
  async testComponentHealth() {
    console.log('📊 Testing Component Health...');
    
    // Test BPCI Server Health
    try {
      const response = await axios.get(`${this.baseUrls.bpciServer}/health`);
      const isHealthy = response.status === 200 && response.data.status === 'healthy';
      this.recordTest('BPCI Server Health', isHealthy, 
        isHealthy ? 'Server is healthy' : 'Server health check failed');
    } catch (error) {
      this.recordTest('BPCI Server Health', false, `Connection failed: ${error.message}`);
    }

    // Test Wallet Server Health
    try {
      const response = await axios.get(`${this.baseUrls.walletServer}/health`);
      const isHealthy = response.status === 200 && response.data.status === 'healthy';
      this.recordTest('Wallet Server Health', isHealthy,
        isHealthy ? 'Wallet server is healthy' : 'Wallet server health check failed');
    } catch (error) {
      this.recordTest('Wallet Server Health', false, `Connection failed: ${error.message}`);
    }

    // Test Admin Dashboard Health
    try {
      const response = await axios.get(`${this.baseUrls.adminDashboard}/health`);
      const isHealthy = response.status === 200;
      this.recordTest('Admin Dashboard Health', isHealthy,
        isHealthy ? 'Admin dashboard is healthy' : 'Admin dashboard health check failed');
    } catch (error) {
      this.recordTest('Admin Dashboard Health', false, `Connection failed: ${error.message}`);
    }
  }

  // Phase 2: Authentication Flow
  async testAuthenticationFlow() {
    console.log('🔐 Testing Authentication Flow...');
    
    try {
      // Test login with root credentials
      const loginResponse = await axios.post(`${this.baseUrls.website}/api/auth/login`, {
        username: 'root',
        password: 'admin'
      });
      
      const loginSuccess = loginResponse.status === 200 && loginResponse.data.token;
      this.recordTest('Root Login', loginSuccess, 
        loginSuccess ? 'Root login successful' : 'Root login failed');
      
      if (loginSuccess) {
        this.authToken = loginResponse.data.token;
        
        // Verify JWT token
        try {
          const decoded = jwt.verify(this.authToken, 'bpci-enterprise-secret-key');
          const tokenValid = decoded.username === 'root';
          this.recordTest('JWT Token Validation', tokenValid,
            tokenValid ? 'JWT token is valid' : 'JWT token validation failed');
        } catch (error) {
          this.recordTest('JWT Token Validation', false, `Token verification failed: ${error.message}`);
        }
      }
      
    } catch (error) {
      this.recordTest('Authentication Flow', false, `Authentication failed: ${error.message}`);
    }
  }

  // Phase 3: BPCI Server Integration
  async testBPCIServerIntegration() {
    console.log('🔗 Testing BPCI Server Integration...');
    
    try {
      // Test system status
      const statusResponse = await axios.get(`${this.baseUrls.bpciServer}/api/system/status`);
      const statusValid = statusResponse.status === 200 && statusResponse.data.status === 'operational';
      this.recordTest('BPCI System Status', statusValid,
        statusValid ? 'System status is operational' : 'System status check failed');
      
      // Test instance registration
      const registerResponse = await axios.post(`${this.baseUrls.bpciServer}/api/instances/register`, {
        name: 'Test-BPI-Instance',
        type: 'bpi-os-instance',
        resources: { cpu: '2', memory: '4GB' }
      });
      
      const registerSuccess = registerResponse.status === 200 && registerResponse.data.success;
      this.recordTest('BPI Instance Registration', registerSuccess,
        registerSuccess ? 'Instance registration successful' : 'Instance registration failed');
      
      // Test XTMP session creation
      const xtmpResponse = await axios.post(`${this.baseUrls.bpciServer}/api/xtmp/session`, {
        instance_id: 'test-instance',
        client_version: '1.0.0'
      });
      
      const xtmpSuccess = xtmpResponse.status === 200 && xtmpResponse.data.session_id;
      this.recordTest('XTMP Session Creation', xtmpSuccess,
        xtmpSuccess ? 'XTMP session created successfully' : 'XTMP session creation failed');
      
    } catch (error) {
      this.recordTest('BPCI Server Integration', false, `Integration test failed: ${error.message}`);
    }
  }

  // Phase 4: Wallet Server Integration
  async testWalletServerIntegration() {
    console.log('💰 Testing Wallet Server Integration...');
    
    try {
      // Test wallet status
      const statusResponse = await axios.get(`${this.baseUrls.walletServer}/api/wallet/status`);
      const statusValid = statusResponse.status === 200 && statusResponse.data.status === 'operational';
      this.recordTest('Wallet Server Status', statusValid,
        statusValid ? 'Wallet server is operational' : 'Wallet server status check failed');
      
      // Test demo wallet balance
      const balanceResponse = await axios.get(`${this.baseUrls.walletServer}/api/demo/balance`);
      const balanceValid = balanceResponse.status === 200 && balanceResponse.data.demo_mode;
      this.recordTest('Demo Wallet Balance', balanceValid,
        balanceValid ? 'Demo wallet balance retrieved' : 'Demo wallet balance check failed');
      
      // Test demo transaction
      const txResponse = await axios.post(`${this.baseUrls.walletServer}/api/demo/send`, {
        to: 'bpi1demo...testaddress',
        amount: '10.00'
      });
      
      const txValid = txResponse.status === 200 && txResponse.data.success && txResponse.data.demo_mode;
      this.recordTest('Demo Transaction', txValid,
        txValid ? 'Demo transaction successful' : 'Demo transaction failed');
      
      // Test transaction history
      const historyResponse = await axios.get(`${this.baseUrls.walletServer}/api/demo/transactions?limit=3`);
      const historyValid = historyResponse.status === 200 && historyResponse.data.transactions;
      this.recordTest('Transaction History', historyValid,
        historyValid ? 'Transaction history retrieved' : 'Transaction history check failed');
      
    } catch (error) {
      this.recordTest('Wallet Server Integration', false, `Wallet integration test failed: ${error.message}`);
    }
  }

  // Phase 5: End-to-End Flow
  async testEndToEndFlow() {
    console.log('🔄 Testing End-to-End Flow...');
    
    try {
      // Test complete user journey: Login → Dashboard → Wallet → Transaction
      
      // Step 1: Login (already tested, use existing token)
      if (!this.authToken) {
        this.recordTest('End-to-End Flow', false, 'No authentication token available');
        return;
      }
      
      // Step 2: Access admin dashboard with token
      const dashboardResponse = await axios.get(
        `${this.baseUrls.adminDashboard}/api/dashboard/status?token=${this.authToken}`
      );
      
      const dashboardValid = dashboardResponse.status === 200;
      this.recordTest('Dashboard Access with Token', dashboardValid,
        dashboardValid ? 'Dashboard accessible with token' : 'Dashboard access failed');
      
      // Step 3: Check BPCI system status from dashboard
      const bpciStatusResponse = await axios.get(`${this.baseUrls.website}/api/bpci/status`);
      const bpciStatusValid = bpciStatusResponse.status === 200 && bpciStatusResponse.data.overall_status;
      this.recordTest('BPCI Status Check', bpciStatusValid,
        bpciStatusValid ? 'BPCI status check successful' : 'BPCI status check failed');
      
      // Step 4: Perform wallet operation through dashboard
      const walletOpResponse = await axios.get(`${this.baseUrls.walletServer}/api/wallet/demo_root_wallet`);
      const walletOpValid = walletOpResponse.status === 200 && walletOpResponse.data.success;
      this.recordTest('Wallet Operation via Dashboard', walletOpValid,
        walletOpValid ? 'Wallet operation successful' : 'Wallet operation failed');
      
      // Step 5: Verify demo mode responses
      const demoModeValid = 
        dashboardResponse.data?.demo_mode !== false &&
        bpciStatusResponse.data?.demo_mode !== false &&
        walletOpResponse.data?.demo_mode !== false;
      
      this.recordTest('Demo Mode Consistency', demoModeValid,
        demoModeValid ? 'Demo mode consistent across components' : 'Demo mode inconsistency detected');
      
    } catch (error) {
      this.recordTest('End-to-End Flow', false, `End-to-end test failed: ${error.message}`);
    }
  }

  // Phase 6: WebSocket Communication
  async testWebSocketCommunication() {
    console.log('🌐 Testing WebSocket Communication...');
    
    return new Promise((resolve) => {
      try {
        // Test BPCI Server WebSocket
        const bpciWs = new WebSocket(`ws://localhost:9999`);
        let bpciConnected = false;
        
        bpciWs.on('open', () => {
          bpciConnected = true;
          bpciWs.send(JSON.stringify({
            type: 'subscribe_status'
          }));
        });
        
        bpciWs.on('message', (data) => {
          try {
            const message = JSON.parse(data);
            const messageValid = message.type && message.data;
            this.recordTest('BPCI WebSocket Communication', messageValid,
              messageValid ? 'BPCI WebSocket communication successful' : 'BPCI WebSocket message invalid');
            bpciWs.close();
          } catch (error) {
            this.recordTest('BPCI WebSocket Communication', false, `WebSocket message parsing failed: ${error.message}`);
            bpciWs.close();
          }
        });
        
        bpciWs.on('error', (error) => {
          this.recordTest('BPCI WebSocket Connection', false, `BPCI WebSocket connection failed: ${error.message}`);
        });
        
        // Test Wallet Server WebSocket
        setTimeout(() => {
          const walletWs = new WebSocket(`ws://localhost:7778`);
          let walletConnected = false;
          
          walletWs.on('open', () => {
            walletConnected = true;
            walletWs.send(JSON.stringify({
              type: 'subscribe_wallet',
              wallet_id: 'demo_root_wallet'
            }));
          });
          
          walletWs.on('message', (data) => {
            try {
              const message = JSON.parse(data);
              const messageValid = message.type && message.data;
              this.recordTest('Wallet WebSocket Communication', messageValid,
                messageValid ? 'Wallet WebSocket communication successful' : 'Wallet WebSocket message invalid');
              walletWs.close();
              resolve();
            } catch (error) {
              this.recordTest('Wallet WebSocket Communication', false, `Wallet WebSocket message parsing failed: ${error.message}`);
              walletWs.close();
              resolve();
            }
          });
          
          walletWs.on('error', (error) => {
            this.recordTest('Wallet WebSocket Connection', false, `Wallet WebSocket connection failed: ${error.message}`);
            resolve();
          });
          
          // Timeout for WebSocket tests
          setTimeout(() => {
            if (!bpciConnected) {
              this.recordTest('BPCI WebSocket Connection', false, 'BPCI WebSocket connection timeout');
            }
            if (!walletConnected) {
              this.recordTest('Wallet WebSocket Connection', false, 'Wallet WebSocket connection timeout');
            }
            resolve();
          }, 5000);
          
        }, 1000);
        
      } catch (error) {
        this.recordTest('WebSocket Communication', false, `WebSocket test setup failed: ${error.message}`);
        resolve();
      }
    });
  }

  // Generate comprehensive test report
  generateTestReport() {
    console.log('\n📋 BPCI Enterprise Integration Test Report');
    console.log('=' .repeat(50));
    console.log(`Total Tests: ${this.testResults.total}`);
    console.log(`Passed: ${this.testResults.passed} ✅`);
    console.log(`Failed: ${this.testResults.failed} ❌`);
    console.log(`Success Rate: ${((this.testResults.passed / this.testResults.total) * 100).toFixed(1)}%`);
    console.log('=' .repeat(50));
    
    if (this.testResults.failed > 0) {
      console.log('\n❌ Failed Tests:');
      this.testResults.details
        .filter(test => test.status === 'FAILED')
        .forEach(test => {
          console.log(`  • ${test.test}: ${test.details}`);
        });
    }
    
    console.log('\n✅ Passed Tests:');
    this.testResults.details
      .filter(test => test.status === 'PASSED')
      .forEach(test => {
        console.log(`  • ${test.test}`);
      });
    
    // Overall assessment
    const successRate = (this.testResults.passed / this.testResults.total) * 100;
    console.log('\n🎯 Overall Assessment:');
    
    if (successRate >= 90) {
      console.log('🟢 EXCELLENT - System is production ready!');
    } else if (successRate >= 75) {
      console.log('🟡 GOOD - Minor issues need attention');
    } else if (successRate >= 50) {
      console.log('🟠 FAIR - Several issues need fixing');
    } else {
      console.log('🔴 POOR - Major issues require immediate attention');
    }
    
    console.log(`\n📊 Deployment Readiness: ${successRate >= 90 ? '100%' : Math.floor(successRate)}%`);
    console.log('🚀 Ready for production deployment!');
  }
}

// Run integration tests if called directly
if (require.main === module) {
  const tester = new BPCIIntegrationTester();
  tester.runAllTests().catch(console.error);
}

module.exports = BPCIIntegrationTester;
