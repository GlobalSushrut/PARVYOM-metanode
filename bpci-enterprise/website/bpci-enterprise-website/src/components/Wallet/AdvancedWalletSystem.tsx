import React, { useState, useEffect } from 'react';
import { 
  Card, 
  Button, 
  Modal, 
  Form, 
  Select, 
  notification, 
  Tabs, 
  Statistic, 
  Progress, 
  Descriptions,
  Badge,
  Typography,
  Space
} from 'antd';
import { 
  WalletOutlined, 
  PlusOutlined, 
  SafetyOutlined,
  ThunderboltOutlined,
  CheckCircleOutlined,
  ExclamationCircleOutlined,
  InfoCircleOutlined
} from '@ant-design/icons';

const { Title, Text } = Typography;
const { TabPane } = Tabs;
const { Option } = Select;

// Real Rust Implementation Types - Exact Match
interface RegisteredWallet {
  registration_id: string; // UUID
  wallet_address: string; // BPI address
  wallet_type: WalletType;
  owner_type?: OwnerType;
  network_type: NetworkType;
  stamp_type?: StampType;
  mother_coin_allocation: number;
  baby_coin_balance: number;
  poe_stats: PoEMiningStats;
  compliance_status: ComplianceStatus;
  billing_config: BillingConfig;
  created_at: string;
  updated_at: string;
  migration_count: number;
}

// Real Rust Implementation Types - Exact Match with Rust Code
const WalletType = {
  BpciService: "BpciService",
  Personal: "Personal",
  Enterprise: "Enterprise",
  Community: "Community",
  Investor: "Investor", 
  Government: "Government",
  Bank: "Bank",
  Owner: "Owner",
  ESOP: "ESOP",
  Treasury: "Treasury",
  Company: "Company"
} as const;
type WalletType = typeof WalletType[keyof typeof WalletType];

// Types matching the real Rust BPI wallet implementation
interface WalletAddress {
  address: string;
}

interface ServiceId {
  id: string;
}

// BPI Wallet structure matching Rust implementation exactly
interface BpiWallet {
  id: string;
  wallet_type: WalletType;
  address: WalletAddress;
  service_id?: ServiceId;
  verification_level: string;
  public_key: Uint8Array;
  private_key_encrypted: string;
  key_type: KeyType;
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
  status: WalletStatus;
  metadata: {
    node_type: string;
    version: string;
    capabilities: string;
  };
  signature?: string;
  node_id: string;
  bpi_address: string;
  activation_tx_hash?: string;
  // Additional fields for UI display
  mother_coin_balance?: number;
  baby_coin_balance?: number;
  mining_efficiency?: number;
  compliance_score?: number;
}

const KeyType = {
  Ed25519: "Ed25519",
  Secp256k1: "Secp256k1"
} as const;
type KeyType = typeof KeyType[keyof typeof KeyType];

const WalletStatus = {
  Active: "Active",
  Inactive: "Inactive",
  Pending: "Pending",
  Suspended: "Suspended"
} as const;
type WalletStatus = typeof WalletStatus[keyof typeof WalletStatus];

const OwnerType = {
  Founder: 1,        // 600 mother coins
  EarlyInvestor: 2,  // 100 coins each
  CommunityLeader: 3, // variable allocation
  StrategicPartner: 4, // negotiated allocation
  PublicInvestor: 5   // market-based allocation
} as const;
type OwnerType = typeof OwnerType[keyof typeof OwnerType];

const NetworkType = {
  Testnet: "Testnet", // Free coins, no real billing, refundable
  Mainnet: "Mainnet"  // Real billing, $1/BPI default, 100% security
} as const;
type NetworkType = typeof NetworkType[keyof typeof NetworkType];

const StampType = {
  Government: "Government",
  Bank: "Bank",
  Regulatory: "Regulatory"
} as const;
type StampType = typeof StampType[keyof typeof StampType];

interface PoEMiningStats {
  total_activities: number;
  baby_coins_earned: number;
  mining_efficiency: number;
  last_activity: string;
}

interface ComplianceStatus {
  kyc_verified: boolean;
  aml_cleared: boolean;
  regulatory_approved: boolean;
  compliance_score: number;
  last_audit: string;
}

interface BillingConfig {
  network_type: NetworkType;
  billing_rate_per_bpi: number;
  monthly_cap: number;
  is_refundable: boolean;
}

interface WalletCreationSession {
  session_id: string;
  user_id: string;
  current_step: WalletCreationStep;
  wallet_type: WalletType;
  owner_type?: OwnerType;
  network_type: NetworkType;
  progress_percentage: number;
  created_at: string;
}

const WalletCreationStep = {
  ValidateRequest: "ValidateRequest",
  GenerateKeys: "GenerateKeys", 
  RegisterBlockchain: "RegisterBlockchain",
  RegisterNodes: "RegisterNodes",
  InitializeEconomic: "InitializeEconomic",
  ActivateWallet: "ActivateWallet",
  Complete: "Complete"
} as const;
type WalletCreationStep = typeof WalletCreationStep[keyof typeof WalletCreationStep];

// Crypto utilities matching Rust implementation
const generateEd25519KeyPair = (): { publicKey: Uint8Array; privateKey: Uint8Array } => {
  // In production, use a proper Ed25519 library like @noble/ed25519
  const publicKey = new Uint8Array(32);
  const privateKey = new Uint8Array(32);
  crypto.getRandomValues(publicKey);
  crypto.getRandomValues(privateKey);
  return { publicKey, privateKey };
};

const generateBpiAddress = async (publicKey: Uint8Array): Promise<string> => {
  // Generate BPI address from public key using SHA256 (matching Rust generate_bpi_address)
  const hashBuffer = await crypto.subtle.digest('SHA-256', publicKey);
  const hashArray = new Uint8Array(hashBuffer);
  const addressHex = Array.from(hashArray).map(b => b.toString(16).padStart(2, '0')).join('');
  return `bpi1q${addressHex.substring(0, 32)}`;
};

const generateNodeId = async (publicKey: Uint8Array): Promise<string> => {
  // Generate node ID from public key hash
  const hashBuffer = await crypto.subtle.digest('SHA-256', publicKey);
  const hashArray = new Uint8Array(hashBuffer);
  return Array.from(hashArray.slice(0, 16)).map(b => b.toString(16).padStart(2, '0')).join('');
};

const signWalletRegistration = (walletId: string, privateKey: Uint8Array): string => {
  // Sign wallet registration string (matching Rust sign_wallet_registration)
  const data = `wallet_registration_${walletId}`;
  // In production, use proper Ed25519 signing
  const signature = new Uint8Array(64);
  crypto.getRandomValues(signature);
  return Array.from(signature).map(b => b.toString(16).padStart(2, '0')).join('');
};

const encryptPrivateKey = (privateKey: Uint8Array, password: string): string => {
  // Encrypt private key with password (demo implementation)
  const encrypted = new Uint8Array(privateKey.length);
  for (let i = 0; i < privateKey.length; i++) {
    encrypted[i] = privateKey[i] ^ password.charCodeAt(i % password.length);
  }
  return Array.from(encrypted).map(b => b.toString(16).padStart(2, '0')).join('');
};

export const AdvancedWalletSystem: React.FC = () => {
  const [wallets, setWallets] = useState<BpiWallet[]>([]);
  const [loading, setLoading] = useState(true);
  const [createModalVisible, setCreateModalVisible] = useState(false);
  const [form] = Form.useForm();
  const [error, setError] = useState<string | null>(null);

  // Load wallet data from real BPCI backend
  const fetchWalletData = async () => {
    setLoading(true);
    try {
      // Try real backend first - matching Rust API endpoints
      const response = await fetch('http://127.0.0.1:8080/api/registry/wallets', {
        headers: { 'Authorization': `Bearer ${localStorage.getItem('auth_token')}` }
      });
      
      if (response.ok) {
        const data = await response.json();
        setWallets(data.wallets || []);
      } else {
        throw new Error('Backend not available');
      }
    } catch (err) {
      setError('Using demo data - BPCI backend not connected');
      // Demo data matching real Rust BPI wallet structure
      const demoWallets: BpiWallet[] = [
        {
          id: 'demo-wallet-001',
          wallet_type: WalletType.Personal,
          address: { address: 'bpi1qxyz123abc789def456ghi789jkl012mno345pqr678stu901vwx234yz' },
          service_id: { id: 'service_demo_001' },
          verification_level: 'Enhanced',
          public_key: new Uint8Array(32),
          private_key_encrypted: 'encrypted_demo_key_001',
          key_type: KeyType.Ed25519,
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
          status: WalletStatus.Active,
          metadata: {
            node_type: 'mining_bridge',
            version: '1.0.0',
            capabilities: 'mining,wallet,registry'
          },
          signature: 'demo_signature_001',
          node_id: 'node_demo_001',
          bpi_address: 'bpi1qxyz123abc789def456ghi789jkl012mno345pqr678stu901vwx234yz',
          activation_tx_hash: 'tx_demo_001',
          mother_coin_balance: 50,
          baby_coin_balance: 125,
          mining_efficiency: 85,
          compliance_score: 75
        }
      ]);
    }
    setLoading(false);
  };

  useEffect(() => {
    fetchWalletData();
  }, []);

  // Create new BPI wallet with real Rust implementation flow
  const createWallet = async (values: any) => {
    try {
      // Generate Ed25519 key pair (matching Rust implementation)
      const keyPair = generateEd25519KeyPair();
      const walletId = `wallet_${Date.now()}`;
      
      // Generate BPI address from public key (matching Rust generate_bpi_address)
      const bpiAddress = await generateBpiAddress(keyPair.publicKey);
      
      // Generate node ID from BPC key (matching Rust implementation)
      const nodeId = await generateNodeId(keyPair.publicKey);
      
      // Create wallet registration signature (matching Rust sign_wallet_registration)
      const signature = signWalletRegistration(walletId, keyPair.privateKey);
      
      // Encrypt private key with user password
      const encryptedPrivateKey = encryptPrivateKey(keyPair.privateKey, 'user_password');

      const walletData = {
        wallet_type: values.walletType,
        password: 'user_password', // In production, get from secure input
        verification_level: 'Enhanced'
      };

      const response = await fetch('http://127.0.0.1:8080/api/bpci/wallets/create', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${localStorage.getItem('auth_token')}`
        },
        body: JSON.stringify(walletData)
      });

      if (response.ok) {
        const data = await response.json();
        notification.success({
          message: 'BPI Wallet Created Successfully',
          description: `Wallet ID: ${data.wallet_id}. BPI address: ${data.bpi_address}`
        });
        setCreateModalVisible(false);
        form.resetFields();
        fetchWalletData();
      } else {
        throw new Error('Failed to create wallet');
      }
    } catch (err) {
      notification.error({
        message: 'Creation Failed',
        description: 'Failed to create wallet. Creating demo wallet with real BPI structure.'
      });
      
      // Create demo wallet matching exact Rust BpiWallet structure
      const keyPair = generateEd25519KeyPair();
      const walletId = `demo_wallet_${Date.now()}`;
      const bpiAddress = await generateBpiAddress(keyPair.publicKey);
      const nodeId = await generateNodeId(keyPair.publicKey);
      const signature = signWalletRegistration(walletId, keyPair.privateKey);
      
      const newWallet: BpiWallet = {
        id: walletId,
        wallet_type: values.walletType || WalletType.Personal,
        address: { address: bpiAddress },
        service_id: { id: `service_${nodeId}` },
        verification_level: 'Enhanced',
        public_key: keyPair.publicKey,
        private_key_encrypted: encryptPrivateKey(keyPair.privateKey, 'demo_password'),
        key_type: KeyType.Ed25519,
        bpci_endpoint: 'http://127.0.0.1:8080',
        bci_endpoint: 'http://127.0.0.1:8081',
        capabilities: {
          mining: true,
          wallet: true,
          registry: true,
          encryption_schemes: ['Ed25519', 'AES256']
        },
        registered_at: Math.floor(Date.now() / 1000),
        last_activity: Math.floor(Date.now() / 1000),
        status: WalletStatus.Active,
        metadata: {
          node_type: 'mining_bridge',
          version: '1.0.0',
          capabilities: 'mining,wallet,registry'
        },
        signature: signature,
        node_id: nodeId,
        bpi_address: bpiAddress,
        activation_tx_hash: `tx_${Math.random().toString(36).substring(2, 15)}`
      };
      
      setWallets(prev => [...prev, newWallet]);
      setCreateModalVisible(false);
      form.resetFields();
    }
  };

  // Process PoE mining activity
  const processPoEMining = async (registrationId: string) => {
    try {
      const response = await fetch('http://127.0.0.1:8080/api/registry/wallets/poe-mining', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${localStorage.getItem('auth_token')}`
        },
        body: JSON.stringify({ 
          registration_id: registrationId,
          poe_activities: 10,
          network_load: 0.75
        })
      });

      if (response.ok) {
        const data = await response.json();
        notification.success({
          message: 'PoE Mining Processed',
          description: `Earned ${data.baby_coins_earned} baby coins!`
        });
        fetchWalletData();
      } else {
        throw new Error('Failed to process PoE mining');
      }
    } catch (err) {
      notification.error({
        message: 'Mining Failed',
  }
};

// Process PoE mining activity
const processPoEMining = async (registrationId: string) => {
  try {
    const response = await fetch('http://127.0.0.1:8080/api/registry/wallets/poe-mining', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${localStorage.getItem('auth_token')}`
      },
      body: JSON.stringify({ 
        registration_id: registrationId,
        poe_activities: 10,
        network_load: 0.75
      })
    });

    if (response.ok) {
      const data = await response.json();
      notification.success({
        message: 'PoE Mining Processed',
        description: `Earned ${data.baby_coins_earned} baby coins!`
      });
      fetchWalletData();
    } else {
      throw new Error('Failed to process PoE mining');
    }
  } catch (err) {
    notification.error({
      message: 'Mining Failed',
      description: 'Failed to process PoE mining. Check backend connection.'
    });
  }
};

// Helper functions for wallet type icons and colors
const getWalletTypeIcon = (walletType: WalletType) => {
  switch (walletType) {
    case WalletType.Owner: return <CrownOutlined />;
    case WalletType.Bank: return <BankOutlined />;
    case WalletType.Government: return <GlobalOutlined />;
    case WalletType.Community: return <TeamOutlined />;
    case WalletType.ESOP: return <SafetyOutlined />;
    default: return <WalletOutlined />;
  }
};

const getWalletTypeColor = (walletType: WalletType) => {
  switch (walletType) {
    case WalletType.Owner: return 'gold';
    case WalletType.Bank: return 'blue';
    case WalletType.Government: return 'purple';
    case WalletType.Community: return 'green';
    case WalletType.ESOP: return 'orange';
    default: return 'default';
  }
};

const getComplianceColor = (score: number) => {
  if (score >= 80) return 'success';
  if (score >= 60) return 'warning';
  return 'exception';
};

if (loading) {
  return (
    <div className="flex justify-center items-center min-h-[400px]">
      <Space direction="vertical" size="large" className="text-center">
        <Spin size="large" />
        <div>Loading comprehensive wallet registry...</div>
      </Space>
    </div>
  );
}

return (
  <div className="space-y-6">
    <div className="flex justify-between items-center">
      <div>
        <Title level={3} className="mb-2">
          <WalletOutlined className="mr-2" />
          Comprehensive Wallet Registry
        </Title>
        <Paragraph className="text-gray-600">
          Real BPI-BPCI wallet system with mother/baby coins, PoE mining, and compliance tracking
        </Paragraph>
      </div>
      <Button 
        type="primary" 
        icon={<PlusOutlined />}
        onClick={() => setCreateModalVisible(true)}
      >
        Register New Wallet
      </Button>
    </div>

    {error && (
      <Alert
        message="Backend Connection"
        description={error}
        type="warning"
        showIcon
        className="mb-4"
      />
    )}

    <Tabs defaultActiveKey="1">
      <TabPane tab="Registered Wallets" key="1">
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {wallets.map((wallet) => (
            <Card
              key={wallet.registration_id}
              className="shadow-lg hover:shadow-xl transition-all duration-300"
              title={
                <div className="flex items-center justify-between">
                  <div className="flex items-center">
                    {getWalletTypeIcon(wallet.wallet_type)}
                    <span className="ml-2">{wallet.wallet_type} Wallet</span>
                  </div>
                  <Tag color={getWalletTypeColor(wallet.wallet_type)}>
                    {wallet.network_type}
                  </Tag>
                </div>
              }
              actions={[
                <Button 
                  key="mining"
                  type="link" 
                  icon={<ThunderboltOutlined />}
                  onClick={() => processPoEMining(wallet.registration_id)}
                >
                  Process PoE Mining
                </Button>,
                <Button key="details" type="link" icon={<ApiOutlined />}>
                  View Details
                </Button>
              ]}
            >
              <Descriptions size="small" column={1}>
                <Descriptions.Item label="Registration ID">
                  <Text code>{wallet.registration_id}</Text>
                </Descriptions.Item>
                <Descriptions.Item label="BPI Address">
                  <Text code className="text-xs">{wallet.wallet_address}</Text>
                </Descriptions.Item>
                {wallet.owner_type && (
                  <Descriptions.Item label="Owner Type">
                    <Tag color="gold">Type {wallet.owner_type}</Tag>
                  </Descriptions.Item>
                )}
                <Descriptions.Item label="Mother Coins">
                  <Statistic 
                    value={wallet.mother_coin_allocation} 
                    valueStyle={{ fontSize: '16px', color: '#1890ff' }}
                  />
                </Descriptions.Item>
                <Descriptions.Item label="Baby Coins">
                  <Statistic 
                    value={wallet.baby_coin_balance} 
                    precision={2}
                    valueStyle={{ fontSize: '16px', color: '#52c41a' }}
                  />
                </Descriptions.Item>
                <Descriptions.Item label="PoE Mining">
                  <div className="space-y-1">
                    <div>Activities: {wallet.poe_stats.total_activities}</div>
                    <Progress 
                      percent={Math.round(wallet.poe_stats.mining_efficiency * 100)} 
                      size="small"
                      status={getComplianceColor(wallet.poe_stats.mining_efficiency * 100)}
                    />
                  </div>
                </Descriptions.Item>
                <Descriptions.Item label="Compliance">
                  <div className="space-y-1">
                    <Progress 
                      percent={wallet.compliance_status.compliance_score} 
                      size="small"
                      status={getComplianceColor(wallet.compliance_status.compliance_score)}
                    />
                    <div className="flex space-x-2">
                      <Badge 
                        status={wallet.compliance_status.kyc_verified ? "success" : "error"} 
                        text="KYC" 
                      />
                      <Badge 
                        status={wallet.compliance_status.aml_cleared ? "success" : "error"} 
                        text="AML" 
                      />
                    </div>
                  </div>
                </Descriptions.Item>
                <Descriptions.Item label="Billing">
                  <div>
                    <div>Rate: ${wallet.billing_config.billing_rate_per_bpi}/BPI</div>
                    <div>Cap: ${wallet.billing_config.monthly_cap}/month</div>
                    {wallet.billing_config.is_refundable && (
                      <Tag color="green" size="small">Refundable</Tag>
                    )}
                  </div>
                </Descriptions.Item>
              </Descriptions>
            </Card>
          ))}
        </div>
      </TabPane>

      <TabPane tab="Registry Statistics" key="2">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          <Card className="text-center">
            <Statistic
              title="Total Wallets"
              value={wallets.length}
              prefix={<WalletOutlined />}
            />
          </Card>
          <Card className="text-center">
            <Statistic
              title="Total Mother Coins"
              value={wallets.reduce((sum, w) => sum + w.mother_coin_allocation, 0)}
              prefix={<CrownOutlined />}
            />
          </Card>
          <Card className="text-center">
            <Statistic
              title="Total Baby Coins"
              value={wallets.reduce((sum, w) => sum + w.baby_coin_balance, 0)}
              precision={2}
              prefix={<ThunderboltOutlined />}
            />
          </Card>
          <Card className="text-center">
            <Statistic
              title="Avg Compliance"
              value={wallets.reduce((sum, w) => sum + w.compliance_status.compliance_score, 0) / wallets.length}
              precision={1}
              suffix="%"
              prefix={<SafetyOutlined />}
            />
          </Card>
        </div>
      </TabPane>
    </Tabs>

    {/* Create Wallet Modal - Matching Real Rust Implementation */}
    <Modal
      title="Register New Wallet"
      open={createModalVisible}
      onCancel={() => setCreateModalVisible(false)}
      footer={null}
      width={600}
    >
      <Form
        form={form}
        layout="vertical"
        onFinish={createWallet}
      >
        <Form.Item
          name="walletType"
          label="Wallet Type"
          rules={[{ required: true, message: 'Please select wallet type' }]}
        >
          <Select placeholder="Select wallet type">
            <Option value={WalletType.Community}>Community</Option>
            <Option value={WalletType.Investor}>Investor</Option>
            <Option value={WalletType.Government}>Government</Option>
            <Option value={WalletType.Bank}>Bank</Option>
            <Option value={WalletType.Owner}>Owner</Option>
            <Option value={WalletType.ESOP}>ESOP</Option>
            <Option value={WalletType.Treasury}>Treasury</Option>
            <Option value={WalletType.Company}>Company</Option>
          </Select>
        </Form.Item>

        <Form.Item
          name="ownerType"
          label="Owner Type (for Owner wallets)"
          tooltip="Only required for Owner wallet type"
        >
          <Select placeholder="Select owner type" allowClear>
            <Option value={OwnerType.Founder}>Founder (600 mother coins)</Option>
            <Option value={OwnerType.EarlyInvestor}>Early Investor (100 coins)</Option>
            <Option value={OwnerType.CommunityLeader}>Community Leader</Option>
            <Option value={OwnerType.StrategicPartner}>Strategic Partner</Option>
            <Option value={OwnerType.PublicInvestor}>Public Investor</Option>
          </Select>
        </Form.Item>

        <Form.Item
          name="networkType"
          label="Network Type"
          initialValue={NetworkType.Testnet}
          rules={[{ required: true, message: 'Please select network type' }]}
        >
          <Select>
            <Option value={NetworkType.Testnet}>Testnet (Free, Refundable)</Option>
            <Option value={NetworkType.Mainnet}>Mainnet ($1/BPI, Production)</Option>
          </Select>
        </Form.Item>

        <Form.Item
          name="stampType"
          label="Stamp Type (for special wallets)"
          tooltip="Only for Government, Bank, or Regulatory wallets"
        >
          <Select placeholder="Select stamp type" allowClear>
            <Option value={StampType.Government}>Government</Option>
            <Option value={StampType.Bank}>Bank</Option>
            <Option value={StampType.Regulatory}>Regulatory</Option>
          </Select>
        </Form.Item>

        <div className="flex justify-end space-x-2">
          <Button onClick={() => setCreateModalVisible(false)}>
            Cancel
          </Button>
          <Button type="primary" htmlType="submit">
            Register Wallet
          </Button>
        </div>
      </Form>
    </Modal>
  </div>
);
};

export default AdvancedWalletSystem;
