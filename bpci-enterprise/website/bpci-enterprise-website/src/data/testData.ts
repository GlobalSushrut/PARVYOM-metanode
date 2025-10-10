// Test data for internal infrastructure testing without authentication
// This allows testing of BPI wallet system, dashboard, and other components

export interface TestUser {
  id: string;
  email: string;
  name: string;
  role: string;
  auth_token: string;
  created_at: string;
}

export interface TestWallet {
  id: string;
  wallet_type: string;
  address: { address: string };
  service_id?: { id: string };
  verification_level: string;
  public_key: Uint8Array;
  private_key_encrypted: string;
  key_type: string;
  bpci_endpoint?: string;
  bci_endpoint?: string;
  capabilities: {
    mining: boolean;
    wallet: boolean;
    registry: boolean;
    encryption_schemes: string[];
  };
  registered_at: number;
  last_activity: number;
  status: string;
  metadata: {
    node_type: string;
    version: string;
    capabilities: string;
  };
  signature?: string;
  node_id: string;
  bpi_address: string;
  activation_tx_hash?: string;
  mother_coin_balance?: number;
  baby_coin_balance?: number;
  mining_efficiency?: number;
  compliance_score?: number;
}

export interface TestSystemStatus {
  status: string;
  data: {
    bpci_nodes: number;
    active_wallets: number;
    blockchain_height: number;
    network_health: number;
    last_updated: string;
  };
}

export interface TestEconomicStatus {
  status: string;
  data: {
    mother_coins: {
      total_supply: number;
      circulating: number;
      staked: number;
    };
    baby_coins: {
      total_earned: number;
      active_miners: number;
      mining_rate: number;
    };
    bank_apis: {
      total_transactions: number;
      settlement_volume: number;
      active_connections: number;
    };
  };
}

// Test Users
export const testUsers: TestUser[] = [
  {
    id: 'test-user-001',
    email: 'founder@bpci.test',
    name: 'Test Founder',
    role: 'founder',
    auth_token: 'test_token_founder_001',
    created_at: new Date().toISOString()
  },
  {
    id: 'test-user-002',
    email: 'developer@bpci.test',
    name: 'Test Developer',
    role: 'developer',
    auth_token: 'test_token_dev_002',
    created_at: new Date().toISOString()
  },
  {
    id: 'test-user-003',
    email: 'enterprise@bpci.test',
    name: 'Test Enterprise',
    role: 'enterprise',
    auth_token: 'test_token_ent_003',
    created_at: new Date().toISOString()
  }
];

// Test BPI Wallets
export const testWallets: TestWallet[] = [
  {
    id: 'test-wallet-001',
    wallet_type: 'Personal',
    address: { address: 'bpi1qtest123abc789def456ghi789jkl012mno345pqr678stu901vwx234yz' },
    service_id: { id: 'service_test_001' },
    verification_level: 'Enhanced',
    public_key: new Uint8Array(32).fill(1),
    private_key_encrypted: 'encrypted_test_key_001',
    key_type: 'Ed25519',
    bpci_endpoint: 'http://127.0.0.1:8080',
    bci_endpoint: 'http://127.0.0.1:8081',
    capabilities: {
      mining: true,
      wallet: true,
      registry: true,
      encryption_schemes: ['Ed25519', 'AES256']
    },
    registered_at: Math.floor(Date.now() / 1000) - 86400,
    last_activity: Math.floor(Date.now() / 1000) - 3600,
    status: 'Active',
    metadata: {
      node_type: 'mining_bridge',
      version: '1.0.0',
      capabilities: 'mining,wallet,registry'
    },
    signature: 'test_signature_001',
    node_id: 'node_test_001',
    bpi_address: 'bpi1qtest123abc789def456ghi789jkl012mno345pqr678stu901vwx234yz',
    activation_tx_hash: 'tx_test_001',
    mother_coin_balance: 150,
    baby_coin_balance: 275,
    mining_efficiency: 92,
    compliance_score: 88
  },
  {
    id: 'test-wallet-002',
    wallet_type: 'Business',
    address: { address: 'bpi1qbiz456def789ghi012jkl345mno678pqr901stu234vwx567yza890bcd' },
    service_id: { id: 'service_test_002' },
    verification_level: 'Enterprise',
    public_key: new Uint8Array(32).fill(2),
    private_key_encrypted: 'encrypted_test_key_002',
    key_type: 'Ed25519',
    bpci_endpoint: 'http://127.0.0.1:8080',
    bci_endpoint: 'http://127.0.0.1:8081',
    capabilities: {
      mining: true,
      wallet: true,
      registry: true,
      encryption_schemes: ['Ed25519', 'AES256', 'RSA2048']
    },
    registered_at: Math.floor(Date.now() / 1000) - 172800,
    last_activity: Math.floor(Date.now() / 1000) - 1800,
    status: 'Active',
    metadata: {
      node_type: 'enterprise_bridge',
      version: '1.1.0',
      capabilities: 'mining,wallet,registry,enterprise'
    },
    signature: 'test_signature_002',
    node_id: 'node_test_002',
    bpi_address: 'bpi1qbiz456def789ghi012jkl345mno678pqr901stu234vwx567yza890bcd',
    activation_tx_hash: 'tx_test_002',
    mother_coin_balance: 500,
    baby_coin_balance: 1250,
    mining_efficiency: 96,
    compliance_score: 95
  },
  {
    id: 'test-wallet-003',
    wallet_type: 'Mining',
    address: { address: 'bpi1qmine789ghi012jkl345mno678pqr901stu234vwx567yza890bcd123efg' },
    service_id: { id: 'service_test_003' },
    verification_level: 'Standard',
    public_key: new Uint8Array(32).fill(3),
    private_key_encrypted: 'encrypted_test_key_003',
    key_type: 'Ed25519',
    bpci_endpoint: 'http://127.0.0.1:8080',
    bci_endpoint: 'http://127.0.0.1:8081',
    capabilities: {
      mining: true,
      wallet: false,
      registry: false,
      encryption_schemes: ['Ed25519']
    },
    registered_at: Math.floor(Date.now() / 1000) - 259200,
    last_activity: Math.floor(Date.now() / 1000) - 900,
    status: 'Active',
    metadata: {
      node_type: 'mining_node',
      version: '1.0.2',
      capabilities: 'mining'
    },
    signature: 'test_signature_003',
    node_id: 'node_test_003',
    bpi_address: 'bpi1qmine789ghi012jkl345mno678pqr901stu234vwx567yza890bcd123efg',
    activation_tx_hash: 'tx_test_003',
    mother_coin_balance: 25,
    baby_coin_balance: 850,
    mining_efficiency: 78,
    compliance_score: 65
  },
  {
    id: 'test-wallet-004',
    wallet_type: 'Validator',
    address: { address: 'bpi1qval012jkl345mno678pqr901stu234vwx567yza890bcd123efg456hij' },
    service_id: { id: 'service_test_004' },
    verification_level: 'Enhanced',
    public_key: new Uint8Array(32).fill(4),
    private_key_encrypted: 'encrypted_test_key_004',
    key_type: 'Ed25519',
    bpci_endpoint: 'http://127.0.0.1:8080',
    bci_endpoint: 'http://127.0.0.1:8081',
    capabilities: {
      mining: false,
      wallet: true,
      registry: true,
      encryption_schemes: ['Ed25519', 'AES256']
    },
    registered_at: Math.floor(Date.now() / 1000) - 345600,
    last_activity: Math.floor(Date.now() / 1000) - 600,
    status: 'Active',
    metadata: {
      node_type: 'validator_node',
      version: '1.2.0',
      capabilities: 'wallet,registry,validation'
    },
    signature: 'test_signature_004',
    node_id: 'node_test_004',
    bpi_address: 'bpi1qval012jkl345mno678pqr901stu234vwx567yza890bcd123efg456hij',
    activation_tx_hash: 'tx_test_004',
    mother_coin_balance: 300,
    baby_coin_balance: 450,
    mining_efficiency: 0,
    compliance_score: 98
  }
];

// Test System Status
export const testSystemStatus: TestSystemStatus = {
  status: 'success',
  data: {
    bpci_nodes: 47,
    active_wallets: 1247,
    blockchain_height: 892456,
    network_health: 98,
    last_updated: new Date().toISOString()
  }
};

// Test Economic Status
export const testEconomicStatus: TestEconomicStatus = {
  status: 'success',
  data: {
    mother_coins: {
      total_supply: 10000000,
      circulating: 2750000,
      staked: 1250000
    },
    baby_coins: {
      total_earned: 5847392,
      active_miners: 892,
      mining_rate: 1247.5
    },
    bank_apis: {
      total_transactions: 2847392,
      settlement_volume: 47392847,
      active_connections: 156
    }
  }
};

// Test API Responses
export const testApiResponses = {
  // Authentication
  '/api/auth/login': {
    success: true,
    token: 'test_auth_token_12345',
    user: testUsers[0],
    expires_in: 3600
  },
  
  // Wallet endpoints
  '/api/bpci/wallets': {
    success: true,
    wallets: testWallets
  },
  
  '/api/bpci/wallets/create': {
    success: true,
    wallet_id: 'test-wallet-new-001',
    bpi_address: 'bpi1qnew123abc789def456ghi789jkl012mno345pqr678stu901vwx234yz',
    message: 'Wallet created successfully'
  },
  
  '/api/bpci/wallets/poe-mining': {
    success: true,
    session_id: 'mining_session_001',
    estimated_earnings: 12.5,
    duration: 3600,
    message: 'PoE mining session started'
  },
  
  // System status
  '/api/system/status': testSystemStatus,
  
  // Economic data
  '/api/economic/status': testEconomicStatus
};

// Helper functions for testing
export const getTestUser = (role: string = 'founder'): TestUser => {
  return testUsers.find(user => user.role === role) || testUsers[0];
};

export const getTestWallet = (type: string = 'Personal'): TestWallet => {
  return testWallets.find(wallet => wallet.wallet_type === type) || testWallets[0];
};

export const setTestAuthToken = (token: string = 'test_auth_token_12345'): void => {
  localStorage.setItem('auth_token', token);
  localStorage.setItem('user_data', JSON.stringify(testUsers[0]));
};

export const clearTestData = (): void => {
  localStorage.removeItem('auth_token');
  localStorage.removeItem('user_data');
};

// Mock API interceptor for testing
export const enableTestMode = (): void => {
  // Set test auth token
  setTestAuthToken();
  
  // Add test mode flag
  localStorage.setItem('test_mode', 'true');
  
  console.log('🧪 Test mode enabled - using mock data for internal infrastructure testing');
  console.log('📊 Available test wallets:', testWallets.length);
  console.log('👥 Available test users:', testUsers.length);
  console.log('🔗 Mock API responses configured');
};

export const disableTestMode = (): void => {
  clearTestData();
  localStorage.removeItem('test_mode');
  console.log('🔒 Test mode disabled - using real API endpoints');
};

// Check if test mode is enabled
export const isTestMode = (): boolean => {
  return localStorage.getItem('test_mode') === 'true';
};

export default {
  testUsers,
  testWallets,
  testSystemStatus,
  testEconomicStatus,
  testApiResponses,
  getTestUser,
  getTestWallet,
  setTestAuthToken,
  clearTestData,
  enableTestMode,
  disableTestMode,
  isTestMode
};
